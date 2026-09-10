//! Link-path geometry used to evaluate effective Earth curvature and clearance.

use crate::error::{Error, Result};
use crate::geo::EARTH_RADIUS_M;

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

/// Returns the effective Earth radius for the supplied k-factor.
pub fn effective_earth_radius_m(k_factor: f64) -> Result<f64> {
    if !k_factor.is_finite() || k_factor <= 0.0 {
        return Err(Error::InvalidInput("k-factor must be positive".into()));
    }
    Ok(k_factor * EARTH_RADIUS_M)
}

/// Returns the conventional parabolic Earth-curvature bulge between two points.
///
/// The value is the apparent terrain/reference-path separation introduced by
/// effective Earth curvature. `d1_m + d2_m` is the total path distance.
pub fn earth_bulge_m(d1_m: f64, d2_m: f64, k_factor: f64) -> Result<f64> {
    if !d1_m.is_finite() || !d2_m.is_finite() || d1_m < 0.0 || d2_m < 0.0 {
        return Err(Error::InvalidInput("invalid path distances".into()));
    }
    let total = d1_m + d2_m;
    if total <= 0.0 {
        return Err(Error::InvalidInput("path distance must be positive".into()));
    }
    let radius = effective_earth_radius_m(k_factor)?;
    Ok(d1_m * d2_m / (2.0 * radius))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_k_factor_scales_earth_radius() {
        let radius = effective_earth_radius_m(DEFAULT_K_FACTOR).unwrap();
        assert!((radius - EARTH_RADIUS_M * 4.0 / 3.0).abs() < 1e-9);
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
