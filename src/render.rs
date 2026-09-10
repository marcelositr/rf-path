//! Terminal and graphical profile rendering.

use std::path::Path;

use plotters::coord::Shift;
use plotters::prelude::*;

use crate::analysis::{ClearanceStatus, LinkAnalysis};
use crate::error::{Error, Result};

/// Renders the complete sampled profile as an auditable terminal table.
pub fn render_terminal_profile(analysis: &LinkAnalysis) -> String {
    let mut output = String::new();
    output.push_str("\nProfile details\n");
    output.push_str(
        "  idx   distance_km      latitude      longitude   terrain_m       LOS_m  Fresnel_m  clear_m  ratio  status\n",
    );
    output.push_str(
        "  ---  ------------  ------------  ------------  -----------  -----------  ---------  -------  -----  -------------------\n",
    );

    for (index, sample) in analysis.samples.iter().enumerate() {
        output.push_str(&format!(
            "  {index:>3}  {:>12.3}  {:>12.6}  {:>12.6}  {:>11.3}  {:>11.3}  {:>9.3}  {:>7.3}  {:>5.2}  {}\n",
            sample.distance_m / 1_000.0,
            sample.position.lat_deg,
            sample.position.lon_deg,
            sample.terrain_m,
            sample.los_m,
            sample.fresnel_radius_m,
            sample.clearance_m,
            sample.clearance_ratio,
            status_label(sample.status),
        ));
    }

    output
}

/// Writes the sampled profile as a PNG chart.
pub fn render_profile_png(analysis: &LinkAnalysis, output_path: impl AsRef<Path>) -> Result<()> {
    if analysis.samples.is_empty() {
        return Err(Error::InvalidInput(
            "cannot render an empty link profile".into(),
        ));
    }
    let root = BitMapBackend::new(output_path.as_ref(), (1200, 700)).into_drawing_area();
    render_profile_chart(root, analysis)
}

/// Writes the sampled profile as an SVG chart.
pub fn render_profile_svg(analysis: &LinkAnalysis, output_path: impl AsRef<Path>) -> Result<()> {
    if analysis.samples.is_empty() {
        return Err(Error::InvalidInput(
            "cannot render an empty link profile".into(),
        ));
    }
    let root = SVGBackend::new(output_path.as_ref(), (1200, 700)).into_drawing_area();
    render_profile_chart(root, analysis)
}

fn render_profile_chart<DB>(root: DrawingArea<DB, Shift>, analysis: &LinkAnalysis) -> Result<()>
where
    DB: DrawingBackend,
    DB::ErrorType: std::fmt::Debug,
{
    root.fill(&WHITE)
        .map_err(|error| Error::InvalidInput(format!("failed to initialize chart: {error:?}")))?;

    let max_distance_km = analysis
        .samples
        .last()
        .map(|s| s.distance_m / 1_000.0)
        .unwrap_or(0.0);
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;
    for sample in &analysis.samples {
        for value in [
            sample.terrain_m,
            sample.los_m,
            sample.los_m + sample.fresnel_radius_m,
            sample.los_m - sample.fresnel_radius_m,
        ] {
            y_min = y_min.min(value);
            y_max = y_max.max(value);
        }
    }
    if !y_min.is_finite() || !y_max.is_finite() {
        return Err(Error::InvalidInput(
            "profile contains non-finite elevation values".into(),
        ));
    }
    let span = y_max - y_min;
    let padding = if span > 0.0 { span * 0.08 } else { 1.0 };
    y_min -= padding;
    y_max += padding;
    let x_max = if max_distance_km > 0.0 {
        max_distance_km
    } else {
        1.0
    };

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!(
                "RF-Path profile — {:.3} km @ {:.3} GHz",
                analysis.distance_m / 1_000.0,
                analysis.frequency_hz / 1.0e9
            ),
            ("sans-serif", 28),
        )
        .margin(20)
        .x_label_area_size(45)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..x_max, y_min..y_max)
        .map_err(|error| Error::InvalidInput(format!("failed to build chart: {error:?}")))?;

    chart
        .configure_mesh()
        .x_desc("Distance (km)")
        .y_desc("Elevation / path height (m)")
        .draw()
        .map_err(|error| Error::InvalidInput(format!("failed to draw chart mesh: {error:?}")))?;

    let terrain = analysis
        .samples
        .iter()
        .map(|s| (s.distance_m / 1_000.0, s.terrain_m));
    let los = analysis
        .samples
        .iter()
        .map(|s| (s.distance_m / 1_000.0, s.los_m));
    let fresnel_upper = analysis
        .samples
        .iter()
        .map(|s| (s.distance_m / 1_000.0, s.los_m + s.fresnel_radius_m));
    let fresnel_lower = analysis
        .samples
        .iter()
        .map(|s| (s.distance_m / 1_000.0, s.los_m - s.fresnel_radius_m));

    chart
        .draw_series(LineSeries::new(terrain, &BLACK))
        .map_err(|error| Error::InvalidInput(format!("failed to draw terrain: {error:?}")))?
        .label("Terrain")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));

    chart
        .draw_series(LineSeries::new(los, &BLUE))
        .map_err(|error| Error::InvalidInput(format!("failed to draw LOS: {error:?}")))?
        .label("Effective LOS")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .draw_series(LineSeries::new(fresnel_upper, &RED))
        .map_err(|error| Error::InvalidInput(format!("failed to draw Fresnel upper: {error:?}")))?
        .label("Fresnel +")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(LineSeries::new(fresnel_lower, &RED))
        .map_err(|error| Error::InvalidInput(format!("failed to draw Fresnel lower: {error:?}")))?
        .label("Fresnel -")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.8))
        .draw()
        .map_err(|error| Error::InvalidInput(format!("failed to draw chart legend: {error:?}")))?;

    root.present()
        .map_err(|error| Error::InvalidInput(format!("failed to finalize chart: {error:?}")))?;
    Ok(())
}

fn status_label(status: ClearanceStatus) -> &'static str {
    match status {
        ClearanceStatus::Clear => "CLEAR",
        ClearanceStatus::FresnelPartial => "FRESNEL_PARTIAL",
        ClearanceStatus::LineOfSightBlocked => "LOS_BLOCKED",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::ProfileSample;
    use crate::geo::{AntennaPoint, GeoPoint};

    fn sample_analysis() -> LinkAnalysis {
        LinkAnalysis {
            distance_m: 100.0,
            frequency_hz: 2.4e9,
            wavelength_m: 0.1,
            tx: AntennaPoint {
                position: GeoPoint::new(0.0, 0.0).unwrap(),
                antenna_height_m: 10.0,
            },
            rx: AntennaPoint {
                position: GeoPoint::new(0.0, 0.001).unwrap(),
                antenna_height_m: 10.0,
            },
            tx_ground_m: 0.0,
            rx_ground_m: 0.0,
            tx_altitude_m: 10.0,
            rx_altitude_m: 10.0,
            fspl_db: 80.0,
            min_clearance_m: -1.0,
            min_clearance_ratio: -0.5,
            los_blocked: true,
            fresnel_60_blocked: true,
            worst_point: None,
            samples: vec![
                ProfileSample {
                    distance_m: 0.0,
                    position: GeoPoint::new(0.0, 0.0).unwrap(),
                    terrain_m: 0.0,
                    los_m: 10.0,
                    earth_bulge_m: 0.0,
                    fresnel_radius_m: 0.0,
                    clearance_m: 10.0,
                    clearance_ratio: 1.0,
                    status: ClearanceStatus::Clear,
                },
                ProfileSample {
                    distance_m: 50.0,
                    position: GeoPoint::new(0.0, 0.0005).unwrap(),
                    terrain_m: 12.0,
                    los_m: 10.0,
                    earth_bulge_m: 0.0,
                    fresnel_radius_m: 2.0,
                    clearance_m: -2.0,
                    clearance_ratio: -1.0,
                    status: ClearanceStatus::LineOfSightBlocked,
                },
                ProfileSample {
                    distance_m: 100.0,
                    position: GeoPoint::new(0.0, 0.001).unwrap(),
                    terrain_m: 9.0,
                    los_m: 10.0,
                    earth_bulge_m: 0.0,
                    fresnel_radius_m: 0.0,
                    clearance_m: 1.0,
                    clearance_ratio: 1.0,
                    status: ClearanceStatus::Clear,
                },
            ],
        }
    }

    #[test]
    fn renders_profile_rows_and_status_labels() {
        let rendered = render_terminal_profile(&sample_analysis());
        assert!(rendered.contains("Profile details"));
        assert!(rendered.contains("CLEAR"));
        assert!(rendered.contains("LOS_BLOCKED"));
        assert_eq!(rendered.lines().count(), 7);
    }

    #[test]
    fn renders_png_and_svg_files() {
        let analysis = sample_analysis();
        let temp_dir = std::env::temp_dir();
        let stem = format!("rf_path_render_test_{}", std::process::id());
        let png_path = temp_dir.join(format!("{stem}.png"));
        let svg_path = temp_dir.join(format!("{stem}.svg"));
        render_profile_png(&analysis, &png_path).unwrap();
        render_profile_svg(&analysis, &svg_path).unwrap();
        assert!(std::fs::metadata(&png_path).unwrap().len() > 0);
        assert!(std::fs::metadata(&svg_path).unwrap().len() > 0);
        std::fs::remove_file(png_path).unwrap();
        std::fs::remove_file(svg_path).unwrap();
    }
}
