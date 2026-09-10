//! RF-Path application entry point.

use std::path::Path;
use std::process::ExitCode;

use clap::Parser;
use rf_path::analysis::analyze_link;
use rf_path::cli::{parse_antenna_point, Cli};
use rf_path::error::{Error, Result};
use rf_path::geojson_export::export_geojson;
use rf_path::render::{render_profile_png, render_profile_svg, render_terminal_profile};
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

    if cli.profile {
        print!("{}", render_terminal_profile(&analysis));
    }

    if let Some(path) = cli.output_image.as_deref() {
        render_profile_image(&analysis, path)?;
        println!("  profile image: {path}");
    }

    if let Some(path) = cli.export_geojson.as_deref() {
        export_geojson(&analysis, path)?;
        println!("  GeoJSON: {path}");
    }

    Ok(())
}

fn render_profile_image(analysis: &rf_path::analysis::LinkAnalysis, path: &str) -> Result<()> {
    let extension = Path::new(path)
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
        .ok_or_else(|| {
            Error::InvalidInput("--output-image must use a .png or .svg extension".into())
        })?;

    match extension.as_str() {
        "png" => render_profile_png(analysis, path),
        "svg" => render_profile_svg(analysis, path),
        _ => Err(Error::InvalidInput(
            "--output-image must use a .png or .svg extension".into(),
        )),
    }
}
