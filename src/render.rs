//! Terminal and graphical profile rendering.

use crate::analysis::{ClearanceStatus, LinkAnalysis};

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

    #[test]
    fn renders_profile_rows_and_status_labels() {
        let analysis = LinkAnalysis {
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
        };

        let rendered = render_terminal_profile(&analysis);
        assert!(rendered.contains("Profile details"));
        assert!(rendered.contains("CLEAR"));
        assert!(rendered.contains("LOS_BLOCKED"));
        assert_eq!(rendered.lines().count(), 7);
    }
}
