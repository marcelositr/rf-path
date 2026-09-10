//! Command-line interface definition.

use clap::Parser;

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
