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
    if d1_m < 0.0 || d2_m < 0.0 || d1_m + d2_m <= 0.0 {
        return Err(Error::InvalidInput("invalid Fresnel distances".into()));
    }
    Ok((wavelength_m(frequency_hz)? * d1_m * d2_m / (d1_m + d2_m)).sqrt())
}

pub fn free_space_path_loss_db(distance_m: f64, frequency_hz: f64) -> Result<f64> {
    if !distance_m.is_finite() || distance_m <= 0.0 {
        return Err(Error::InvalidInput("distance must be positive".into()));
    }
    if !frequency_hz.is_finite() || frequency_hz <= 0.0 {
        return Err(Error::InvalidInput("frequency must be positive".into()));
    }
    Ok(20.0 * (4.0 * std::f64::consts::PI * distance_m * frequency_hz / SPEED_OF_LIGHT_M_S).log10())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wavelength_at_2_4_ghz() {
        let lambda = wavelength_m(2.4e9).unwrap();
        assert!((lambda - 0.1249135).abs() < 1e-6);
    }

    #[test]
    fn fresnel_radius_at_midpoint() {
        let f = fresnel_radius_m(2.4e9, 500.0, 500.0).unwrap();
        assert!((f - 3.951_858).abs() < 1e-5);
    }

    #[test]
    fn fspl_is_reasonable() {
        let loss = free_space_path_loss_db(1_000.0, 2.4e9).unwrap();
        assert!((loss - 100.045_536).abs() < 1e-4);
    }
}
