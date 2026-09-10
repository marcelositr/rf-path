//! RF-Path application entry point.

use std::process::ExitCode;

use clap::Parser;
use rf_path::analysis::analyze_link;
use rf_path::cli::{parse_antenna_point, Cli};
use rf_path::error::{Error, Result};
use rf_path::srtm::SrtmProvider;
use rf_path::units::parse_frequency_hz;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    if cli.output_image.is_some() || cli.export_geojson.is_some() {
        return Err(Error::InvalidInput(
            "image and GeoJSON outputs are not wired into the CLI yet".into(),
        ));
    }

    let tx = parse_antenna_point(
        cli.tx
            .as_deref()
            .ok_or_else(|| Error::InvalidInput("--tx is required".into()))?,
        "TX",
    )?;
    let rx = parse_antenna_point(
        cli.rx
            .as_deref()
            .ok_or_else(|| Error::InvalidInput("--rx is required".into()))?,
        "RX",
    )?;
    let frequency_hz = parse_frequency_hz(
        cli.freq
            .as_deref()
            .ok_or_else(|| Error::InvalidInput("--freq is required".into()))?,
    )?;
    let srtm_dir = cli
        .srtm_dir
        .as_deref()
        .ok_or_else(|| Error::InvalidInput("--srtm-dir is required".into()))?;

    let mut terrain = SrtmProvider::new(srtm_dir);
    let analysis = analyze_link(
        &mut terrain,
        tx,
        rx,
        frequency_hz,
        cli.samples,
        cli.k_factor,
        cli.fresnel_threshold,
    )?;

    println!("RF-Path link analysis");
    println!("  distance: {:.3} km", analysis.distance_m / 1_000.0);
    println!("  frequency: {:.6} GHz", analysis.frequency_hz / 1.0e9);
    println!("  wavelength: {:.6} m", analysis.wavelength_m);
    println!("  FSPL: {:.3} dB", analysis.fspl_db);
    println!("  samples: {}", analysis.samples.len());
    println!("  minimum clearance: {:.3} m", analysis.min_clearance_m);
    println!(
        "  minimum clearance ratio: {:.3}",
        analysis.min_clearance_ratio
    );
    println!("  LOS blocked: {}", analysis.los_blocked);
    println!("  60% Fresnel blocked: {}", analysis.fresnel_60_blocked);

    if let Some(worst) = analysis.worst_point {
        println!(
            "  worst point: {:.3} km at ({:.6},{:.6}), clearance {:.3} m",
            worst.distance_m / 1_000.0,
            worst.position.lat_deg,
            worst.position.lon_deg,
            worst.clearance_m,
        );
    }

    Ok(())
}
