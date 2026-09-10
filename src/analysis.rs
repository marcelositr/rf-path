//! Link-path analysis orchestration and structured results.

use crate::error::{Error, Result};
use crate::geo::{
    great_circle_distance_m, sample_great_circle, AntennaPoint, GeoPoint, EARTH_RADIUS_M,
};
use crate::rf::{free_space_path_loss_db, fresnel_radius_m};
use crate::srtm::TerrainProvider;
use crate::units::SPEED_OF_LIGHT_M_S;

/// Default effective-Earth k-factor used by RF-Path.
pub const DEFAULT_K_FACTOR: f64 = 4.0 / 3.0;
/// Default fraction of the first Fresnel zone that should remain clear.
pub const DEFAULT_FRESNEL_CLEARANCE_RATIO: f64 = 0.60;

/// Classification of a point relative to the LOS and first Fresnel zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearanceStatus {
    Clear,
    FresnelPartial,
    LineOfSightBlocked,
}

/// One auditable point in the sampled RF path profile.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProfileSample {
    pub distance_m: f64,
    pub position: GeoPoint,
    pub terrain_m: f64,
    pub los_m: f64,
    pub earth_bulge_m: f64,
    pub fresnel_radius_m: f64,
    pub clearance_m: f64,
    pub clearance_ratio: f64,
    pub status: ClearanceStatus,
}

/// Highest-impact obstruction point in the sampled profile.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Obstacle {
    pub distance_m: f64,
    pub position: GeoPoint,
    pub terrain_m: f64,
    pub los_m: f64,
    pub fresnel_radius_m: f64,
    pub clearance_m: f64,
    pub clearance_ratio: f64,
}

/// Complete calculated link analysis. Rendering/export layers consume this model.
#[derive(Debug, Clone, PartialEq)]
pub struct LinkAnalysis {
    pub distance_m: f64,
    pub frequency_hz: f64,
    pub wavelength_m: f64,
    pub tx: AntennaPoint,
    pub rx: AntennaPoint,
    pub tx_ground_m: f64,
    pub rx_ground_m: f64,
    pub tx_altitude_m: f64,
    pub rx_altitude_m: f64,
    pub fspl_db: f64,
    pub min_clearance_m: f64,
    pub min_clearance_ratio: f64,
    pub los_blocked: bool,
    pub fresnel_60_blocked: bool,
    pub worst_point: Option<Obstacle>,
    pub samples: Vec<ProfileSample>,
}

/// Returns the effective Earth radius for the supplied k-factor.
pub fn effective_earth_radius_m(k_factor: f64) -> Result<f64> {
    if !k_factor.is_finite() || k_factor <= 0.0 {
        return Err(Error::InvalidInput("k-factor must be positive".into()));
    }
    let radius = k_factor * EARTH_RADIUS_M;
    if !radius.is_finite() || radius <= 0.0 {
        return Err(Error::InvalidInput(
            "effective Earth radius is not finite".into(),
        ));
    }
    Ok(radius)
}

/// Returns the conventional parabolic Earth-curvature bulge between two points.
pub fn earth_bulge_m(d1_m: f64, d2_m: f64, k_factor: f64) -> Result<f64> {
    if !d1_m.is_finite() || !d2_m.is_finite() || d1_m < 0.0 || d2_m < 0.0 {
        return Err(Error::InvalidInput("invalid path distances".into()));
    }
    let total = d1_m + d2_m;
    if total <= 0.0 {
        return Err(Error::InvalidInput("path distance must be positive".into()));
    }
    let radius = effective_earth_radius_m(k_factor)?;
    let bulge = d1_m * d2_m / (2.0 * radius);
    if !bulge.is_finite() {
        return Err(Error::InvalidInput(
            "Earth-curvature bulge is not finite".into(),
        ));
    }
    Ok(bulge)
}

/// Linearly interpolates the absolute endpoint altitudes along the path.
pub fn reference_path_elevation_m(
    tx_altitude_m: f64,
    rx_altitude_m: f64,
    d1_m: f64,
    d2_m: f64,
) -> Result<f64> {
    if !tx_altitude_m.is_finite() || !rx_altitude_m.is_finite() {
        return Err(Error::InvalidInput(
            "endpoint altitudes must be finite".into(),
        ));
    }
    if !d1_m.is_finite() || !d2_m.is_finite() || d1_m < 0.0 || d2_m < 0.0 {
        return Err(Error::InvalidInput("invalid path distances".into()));
    }
    let total = d1_m + d2_m;
    if total <= 0.0 {
        return Err(Error::InvalidInput("path distance must be positive".into()));
    }
    Ok(tx_altitude_m + (rx_altitude_m - tx_altitude_m) * (d1_m / total))
}

/// Returns the effective reference-path altitude after applying Earth bulge.
pub fn effective_reference_path_elevation_m(
    tx_altitude_m: f64,
    rx_altitude_m: f64,
    d1_m: f64,
    d2_m: f64,
    k_factor: f64,
) -> Result<f64> {
    Ok(
        reference_path_elevation_m(tx_altitude_m, rx_altitude_m, d1_m, d2_m)?
            - earth_bulge_m(d1_m, d2_m, k_factor)?,
    )
}

/// Returns terrain clearance relative to the effective reference path.
pub fn terrain_clearance_m(
    terrain_m: f64,
    tx_altitude_m: f64,
    rx_altitude_m: f64,
    d1_m: f64,
    d2_m: f64,
    k_factor: f64,
) -> Result<f64> {
    if !terrain_m.is_finite() {
        return Err(Error::InvalidInput(
            "terrain altitude must be finite".into(),
        ));
    }
    Ok(
        effective_reference_path_elevation_m(tx_altitude_m, rx_altitude_m, d1_m, d2_m, k_factor)?
            - terrain_m,
    )
}

/// Classifies clearance using the requested fraction of the first Fresnel zone.
pub fn classify_clearance(
    clearance_m: f64,
    fresnel_radius_m: f64,
    required_fraction: f64,
) -> Result<ClearanceStatus> {
    if !clearance_m.is_finite() || !fresnel_radius_m.is_finite() {
        return Err(Error::InvalidInput(
            "clearance values must be finite".into(),
        ));
    }
    if fresnel_radius_m < 0.0 {
        return Err(Error::InvalidInput(
            "Fresnel radius cannot be negative".into(),
        ));
    }
    if !required_fraction.is_finite() || !(0.0..=1.0).contains(&required_fraction) {
        return Err(Error::InvalidInput(
            "Fresnel clearance fraction must be between 0 and 1".into(),
        ));
    }
    if clearance_m < 0.0 {
        return Ok(ClearanceStatus::LineOfSightBlocked);
    }
    if fresnel_radius_m > 0.0 && clearance_m < required_fraction * fresnel_radius_m {
        return Ok(ClearanceStatus::FresnelPartial);
    }
    Ok(ClearanceStatus::Clear)
}

fn clearance_ratio(clearance_m: f64, fresnel_radius_m: f64) -> f64 {
    if fresnel_radius_m > 0.0 {
        clearance_m / fresnel_radius_m
    } else if clearance_m >= 0.0 {
        1.0
    } else {
        f64::NEG_INFINITY
    }
}

/// Computes a complete auditable link analysis from a terrain provider.
pub fn analyze_link<T: TerrainProvider>(
    terrain: &mut T,
    tx: AntennaPoint,
    rx: AntennaPoint,
    frequency_hz: f64,
    samples: usize,
    k_factor: f64,
    fresnel_threshold: f64,
) -> Result<LinkAnalysis> {
    if !tx.antenna_height_m.is_finite() || tx.antenna_height_m < 0.0 {
        return Err(Error::InvalidInput(
            "TX antenna height must be finite and non-negative".into(),
        ));
    }
    if !rx.antenna_height_m.is_finite() || rx.antenna_height_m < 0.0 {
        return Err(Error::InvalidInput(
            "RX antenna height must be finite and non-negative".into(),
        ));
    }
    GeoPoint::new(tx.position.lat_deg, tx.position.lon_deg)?;
    GeoPoint::new(rx.position.lat_deg, rx.position.lon_deg)?;
    if samples < 2 {
        return Err(Error::InvalidInput("samples must be at least 2".into()));
    }
    effective_earth_radius_m(k_factor)?;
    if !fresnel_threshold.is_finite() || !(0.0..=1.0).contains(&fresnel_threshold) {
        return Err(Error::InvalidInput(
            "Fresnel clearance fraction must be between 0 and 1".into(),
        ));
    }
    if !frequency_hz.is_finite() || frequency_hz <= 0.0 {
        return Err(Error::InvalidInput("frequency must be positive".into()));
    }

    let distance_m = great_circle_distance_m(tx.position, rx.position);
    if distance_m <= 0.0 || !distance_m.is_finite() {
        return Err(Error::InvalidInput(
            "TX and RX must be different finite points".into(),
        ));
    }
    let wavelength_m = SPEED_OF_LIGHT_M_S / frequency_hz;

    let tx_ground_m = terrain.elevation_at(tx.position)?;
    let rx_ground_m = terrain.elevation_at(rx.position)?;
    let tx_altitude_m = tx_ground_m + tx.antenna_height_m;
    let rx_altitude_m = rx_ground_m + rx.antenna_height_m;
    let fspl_db = free_space_path_loss_db(distance_m, frequency_hz)?;

    let path = sample_great_circle(tx.position, rx.position, samples)?;
    let mut profile = Vec::with_capacity(samples);
    for (index, position) in path.into_iter().enumerate() {
        let fraction = index as f64 / (samples - 1) as f64;
        let d1_m = distance_m * fraction;
        let d2_m = distance_m - d1_m;
        let terrain_m = terrain.elevation_at(position)?;
        let los_m = reference_path_elevation_m(tx_altitude_m, rx_altitude_m, d1_m, d2_m)?;
        let earth_bulge_m = earth_bulge_m(d1_m, d2_m, k_factor)?;
        let effective_los_m = los_m - earth_bulge_m;
        let fresnel_radius_m = fresnel_radius_m(frequency_hz, d1_m, d2_m)?;
        let clearance_m = effective_los_m - terrain_m;
        let status = classify_clearance(clearance_m, fresnel_radius_m, fresnel_threshold)?;
        profile.push(ProfileSample {
            distance_m: d1_m,
            position,
            terrain_m,
            los_m: effective_los_m,
            earth_bulge_m,
            fresnel_radius_m,
            clearance_m,
            clearance_ratio: clearance_ratio(clearance_m, fresnel_radius_m),
            status,
        });
    }

    let worst = profile
        .iter()
        .min_by(|a, b| a.clearance_m.total_cmp(&b.clearance_m))
        .copied()
        .map(|sample| Obstacle {
            distance_m: sample.distance_m,
            position: sample.position,
            terrain_m: sample.terrain_m,
            los_m: sample.los_m,
            fresnel_radius_m: sample.fresnel_radius_m,
            clearance_m: sample.clearance_m,
            clearance_ratio: sample.clearance_ratio,
        });
    let min_clearance_m = worst.map(|p| p.clearance_m).unwrap_or(f64::INFINITY);
    let min_clearance_ratio = profile
        .iter()
        .map(|p| p.clearance_ratio)
        .fold(f64::INFINITY, f64::min);
    let los_blocked = profile
        .iter()
        .any(|p| p.status == ClearanceStatus::LineOfSightBlocked);
    let fresnel_60_blocked = profile
        .iter()
        .any(|p| p.clearance_m < fresnel_threshold * p.fresnel_radius_m);

    Ok(LinkAnalysis {
        distance_m,
        frequency_hz,
        wavelength_m,
        tx,
        rx,
        tx_ground_m,
        rx_ground_m,
        tx_altitude_m,
        rx_altitude_m,
        fspl_db,
        min_clearance_m,
        min_clearance_ratio,
        los_blocked,
        fresnel_60_blocked,
        worst_point: worst,
        samples: profile,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_k_factor_scales_earth_radius() {
        let radius = effective_earth_radius_m(DEFAULT_K_FACTOR).unwrap();
        assert!((radius - EARTH_RADIUS_M * 4.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn extreme_k_factor_is_rejected_before_overflow() {
        assert!(effective_earth_radius_m(f64::MAX).is_err());
    }

    #[test]
    fn earth_bulge_is_zero_at_endpoints_and_maximum_at_midpoint() {
        let total = 10_000.0;
        let midpoint = earth_bulge_m(5_000.0, 5_000.0, DEFAULT_K_FACTOR).unwrap();
        assert!(midpoint > 0.0);
        assert_eq!(earth_bulge_m(0.0, total, DEFAULT_K_FACTOR).unwrap(), 0.0);
        assert_eq!(earth_bulge_m(total, 0.0, DEFAULT_K_FACTOR).unwrap(), 0.0);
    }

    #[test]
    fn reference_path_preserves_endpoints() {
        assert_eq!(
            reference_path_elevation_m(100.0, 200.0, 0.0, 1_000.0).unwrap(),
            100.0
        );
        assert_eq!(
            reference_path_elevation_m(100.0, 200.0, 1_000.0, 0.0).unwrap(),
            200.0
        );
    }

    #[test]
    fn midpoint_reference_path_is_average_of_endpoints() {
        let elevation = reference_path_elevation_m(100.0, 300.0, 500.0, 500.0).unwrap();
        assert_eq!(elevation, 200.0);
    }

    #[test]
    fn clearance_classification_matches_sixty_percent_rule() {
        assert_eq!(
            classify_clearance(-0.1, 10.0, DEFAULT_FRESNEL_CLEARANCE_RATIO).unwrap(),
            ClearanceStatus::LineOfSightBlocked
        );
        assert_eq!(
            classify_clearance(5.9, 10.0, DEFAULT_FRESNEL_CLEARANCE_RATIO).unwrap(),
            ClearanceStatus::FresnelPartial
        );
        assert_eq!(
            classify_clearance(6.0, 10.0, DEFAULT_FRESNEL_CLEARANCE_RATIO).unwrap(),
            ClearanceStatus::Clear
        );
        assert_eq!(
            classify_clearance(0.0, 0.0, DEFAULT_FRESNEL_CLEARANCE_RATIO).unwrap(),
            ClearanceStatus::Clear
        );
    }
}
