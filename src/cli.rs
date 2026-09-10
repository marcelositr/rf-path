//! Command-line interface definition.

use clap::Parser;

use crate::error::{Error, Result};
use crate::geo::{AntennaPoint, GeoPoint};

/// RF-Path command-line interface.
#[derive(Debug, Parser)]
#[command(
    name = "rf-path",
    version,
    about = "Offline RF link path analysis using SRTM terrain data"
)]
pub struct Cli {
    /// Transmitter position: lat,lon,height_m.
    #[arg(long)]
    pub tx: Option<String>,

    /// Receiver position: lat,lon,height_m.
    #[arg(long)]
    pub rx: Option<String>,

    /// Operating frequency, for example 2.4GHz or 2400MHz.
    #[arg(long)]
    pub freq: Option<String>,

    /// Directory containing SRTM HGT tiles.
    #[arg(long)]
    pub srtm_dir: Option<String>,

    /// Number of path samples.
    #[arg(long, default_value_t = 500)]
    pub samples: usize,

    /// Effective-Earth k-factor.
    #[arg(long, default_value_t = 4.0 / 3.0)]
    pub k_factor: f64,

    /// First-Fresnel clearance threshold.
    #[arg(long, default_value_t = 0.60)]
    pub fresnel_threshold: f64,

    /// Output profile image path.
    #[arg(long)]
    pub output_image: Option<String>,

    /// GeoJSON output path.
    #[arg(long)]
    pub export_geojson: Option<String>,
}

/// Parses a CLI antenna specification in `lat,lon,height_m` form.
pub fn parse_antenna_point(input: &str, label: &str) -> Result<AntennaPoint> {
    let mut parts = input.split(',');
    let lat = parts
        .next()
        .ok_or_else(|| Error::InvalidInput(format!("{label} must be lat,lon,height_m")))?
        .trim()
        .parse::<f64>()
        .map_err(|_| Error::InvalidInput(format!("invalid {label} latitude: {input}")))?;
    let lon = parts
        .next()
        .ok_or_else(|| Error::InvalidInput(format!("{label} must be lat,lon,height_m")))?
        .trim()
        .parse::<f64>()
        .map_err(|_| Error::InvalidInput(format!("invalid {label} longitude: {input}")))?;
    let height = parts
        .next()
        .ok_or_else(|| Error::InvalidInput(format!("{label} must be lat,lon,height_m")))?
        .trim()
        .parse::<f64>()
        .map_err(|_| Error::InvalidInput(format!("invalid {label} antenna height: {input}")))?;
    if parts.next().is_some() {
        return Err(Error::InvalidInput(format!(
            "{label} must be lat,lon,height_m"
        )));
    }

    Ok(AntennaPoint {
        position: GeoPoint::new(lat, lon)?,
        antenna_height_m: height,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_antenna_point() {
        let point = parse_antenna_point("-20.2831,-47.7812,12", "TX").unwrap();
        assert_eq!(point.position.lat_deg, -20.2831);
        assert_eq!(point.position.lon_deg, -47.7812);
        assert_eq!(point.antenna_height_m, 12.0);
    }

    #[test]
    fn rejects_extra_fields() {
        assert!(parse_antenna_point("0,0,10,20", "TX").is_err());
    }

    #[test]
    fn rejects_invalid_coordinates() {
        assert!(parse_antenna_point("91,0,10", "TX").is_err());
    }
}
