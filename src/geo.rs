//! Geographic primitives, distances, and great-circle sampling.

use std::f64::consts::PI;

use crate::error::{Error, Result};

pub const EARTH_RADIUS_M: f64 = 6_371_000.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoPoint {
    pub lat_deg: f64,
    pub lon_deg: f64,
}

impl GeoPoint {
    pub fn new(lat_deg: f64, lon_deg: f64) -> Result<Self> {
        if !lat_deg.is_finite() || !lon_deg.is_finite() || !(-90.0..=90.0).contains(&lat_deg) {
            return Err(Error::InvalidInput(format!(
                "invalid coordinate: {lat_deg},{lon_deg}"
            )));
        }
        Ok(Self { lat_deg, lon_deg })
    }

    fn radians(self) -> (f64, f64) {
        (self.lat_deg.to_radians(), self.lon_deg.to_radians())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AntennaPoint {
    pub position: GeoPoint,
    pub antenna_height_m: f64,
}

pub fn great_circle_distance_m(a: GeoPoint, b: GeoPoint) -> f64 {
    let (lat1, lon1) = a.radians();
    let (lat2, lon2) = b.radians();
    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;
    let h = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    2.0 * EARTH_RADIUS_M * h.min(1.0).sqrt().asin()
}

pub fn great_circle_interpolate(a: GeoPoint, b: GeoPoint, fraction: f64) -> Result<GeoPoint> {
    if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
        return Err(Error::InvalidInput(
            "interpolation fraction must be in [0,1]".into(),
        ));
    }
    let (lat1, lon1) = a.radians();
    let (lat2, lon2) = b.radians();
    let v1 = [lat1.cos() * lon1.cos(), lat1.cos() * lon1.sin(), lat1.sin()];
    let v2 = [lat2.cos() * lon2.cos(), lat2.cos() * lon2.sin(), lat2.sin()];
    let dot = (v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]).clamp(-1.0, 1.0);
    let angle = dot.acos();
    let (x, y, z) = if angle < 1e-12 {
        (v1[0], v1[1], v1[2])
    } else {
        let sin_angle = angle.sin();
        let w1 = ((1.0 - fraction) * angle).sin() / sin_angle;
        let w2 = (fraction * angle).sin() / sin_angle;
        (
            w1 * v1[0] + w2 * v2[0],
            w1 * v1[1] + w2 * v2[1],
            w1 * v1[2] + w2 * v2[2],
        )
    };
    let lat = z.atan2((x * x + y * y).sqrt());
    let lon = y.atan2(x);
    GeoPoint::new(lat * 180.0 / PI, lon * 180.0 / PI)
}

pub fn sample_great_circle(a: GeoPoint, b: GeoPoint, samples: usize) -> Result<Vec<GeoPoint>> {
    if samples < 2 {
        return Err(Error::InvalidInput("samples must be at least 2".into()));
    }
    (0..samples)
        .map(|i| great_circle_interpolate(a, b, i as f64 / (samples - 1) as f64))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_point_has_zero_distance() {
        let p = GeoPoint::new(0.0, 0.0).unwrap();
        assert_eq!(great_circle_distance_m(p, p), 0.0);
    }

    #[test]
    fn equatorial_degree_distance_is_reasonable() {
        let a = GeoPoint::new(0.0, 0.0).unwrap();
        let b = GeoPoint::new(0.0, 1.0).unwrap();
        let d = great_circle_distance_m(a, b);
        assert!((d - 111_194.926).abs() < 1.0);
    }

    #[test]
    fn interpolation_preserves_endpoints() {
        let a = GeoPoint::new(-20.0, -47.0).unwrap();
        let b = GeoPoint::new(-21.0, -48.0).unwrap();
        assert_eq!(great_circle_interpolate(a, b, 0.0).unwrap(), a);
        assert_eq!(great_circle_interpolate(a, b, 1.0).unwrap(), b);
    }
}
