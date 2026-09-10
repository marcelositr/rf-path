//! RF propagation and clearance calculations.

use crate::error::{Error, Result};
use crate::units::SPEED_OF_LIGHT_M_S;

pub fn wavelength_m(frequency_hz: f64) -> Result<f64> {
    if !frequency_hz.is_finite() || frequency_hz <= 0.0 {
        return Err(Error::InvalidInput("frequency must be positive".into()));
    }
    Ok(SPEED_OF_LIGHT_M_S / frequency_hz)
}

pub fn fresnel_radius_m(frequency_hz: f64, d1_m: f64, d2_m: f64) -> Result<f64> {
    if !d1_m.is_finite() || !d2_m.is_finite() || d1_m < 0.0 || d2_m < 0.0 {
        return Err(Error::InvalidInput("invalid Fresnel distances".into()));
    }
    let smaller = d1_m.min(d2_m);
    let larger = d1_m.max(d2_m);
    if larger <= 0.0 {
        return Err(Error::InvalidInput("invalid Fresnel distances".into()));
    }
    let wavelength_m = wavelength_m(frequency_hz)?;
    let geometric_term = smaller / (1.0 + smaller / larger);
    let value = wavelength_m * geometric_term;
    if !value.is_finite() || value < 0.0 {
        return Err(Error::InvalidInput("Fresnel radius is not finite".into()));
    }
    Ok(value.sqrt())
}

pub fn free_space_path_loss_db(distance_m: f64, frequency_hz: f64) -> Result<f64> {
    if !distance_m.is_finite() || distance_m <= 0.0 {
        return Err(Error::InvalidInput("distance must be positive".into()));
    }
    if !frequency_hz.is_finite() || frequency_hz <= 0.0 {
        return Err(Error::InvalidInput("frequency must be positive".into()));
    }
    Ok(
        20.0 * (4.0 * std::f64::consts::PI * distance_m * frequency_hz / SPEED_OF_LIGHT_M_S)
            .log10(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wavelength_at_2_4_ghz() {
        let lambda = wavelength_m(2.4e9).unwrap();
        assert!((lambda - 0.124913524).abs() < 1e-9);
    }

    #[test]
    fn fresnel_radius_at_midpoint() {
        let f = fresnel_radius_m(2.4e9, 500.0, 500.0).unwrap();
        assert!((f - 5.588_235_951).abs() < 1e-9);
    }

    #[test]
    fn fresnel_rejects_non_finite_distances() {
        assert!(fresnel_radius_m(2.4e9, f64::NAN, 500.0).is_err());
        assert!(fresnel_radius_m(2.4e9, 500.0, f64::INFINITY).is_err());
    }

    #[test]
    fn fresnel_handles_extreme_finite_distances_without_overflowing() {
        let radius = fresnel_radius_m(2.4e9, f64::MAX / 2.0, f64::MAX / 2.0).unwrap();
        assert!(radius.is_finite());
    }

    #[test]
    fn fspl_is_reasonable() {
        let loss = free_space_path_loss_db(1_000.0, 2.4e9).unwrap();
        assert!((loss - 100.052_008).abs() < 1e-6);
    }
}
