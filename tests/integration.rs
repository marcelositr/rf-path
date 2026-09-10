//! End-to-end application integration tests.

use approx::assert_relative_eq;

use rf_path::analysis::{analyze_link, ClearanceStatus, DEFAULT_FRESNEL_CLEARANCE_RATIO, DEFAULT_K_FACTOR};
use rf_path::geo::{AntennaPoint, GeoPoint};
use rf_path::srtm::TerrainProvider;

struct SyntheticTerrain {
    obstructed: bool,
}

impl TerrainProvider for SyntheticTerrain {
    fn elevation_at(&mut self, point: GeoPoint) -> rf_path::error::Result<f64> {
        if self.obstructed && point.lon_deg > 0.004 && point.lon_deg < 0.006 {
            Ok(100.0)
        } else {
            Ok(0.0)
        }
    }
}

fn endpoints() -> (AntennaPoint, AntennaPoint) {
    (
        AntennaPoint {
            position: GeoPoint::new(0.0, 0.0).unwrap(),
            antenna_height_m: 30.0,
        },
        AntennaPoint {
            position: GeoPoint::new(0.0, 0.01).unwrap(),
            antenna_height_m: 30.0,
        },
    )
}

#[test]
fn link_analysis_produces_auditable_clear_profile() {
    let (tx, rx) = endpoints();
    let mut terrain = SyntheticTerrain { obstructed: false };

    let result = analyze_link(
        &mut terrain,
        tx,
        rx,
        2.4e9,
        21,
        DEFAULT_K_FACTOR,
        DEFAULT_FRESNEL_CLEARANCE_RATIO,
    )
    .unwrap();

    assert_eq!(result.samples.len(), 21);
    assert_relative_eq!(result.tx_ground_m, 0.0, max_relative = 1e-12);
    assert_relative_eq!(result.rx_ground_m, 0.0, max_relative = 1e-12);
    assert_relative_eq!(result.tx_altitude_m, 30.0, max_relative = 1e-12);
    assert_relative_eq!(result.rx_altitude_m, 30.0, max_relative = 1e-12);
    assert!(!result.los_blocked);
    assert!(!result.fresnel_60_blocked);
    assert!(result.min_clearance_m > 0.0);
    assert!(result.min_clearance_ratio > DEFAULT_FRESNEL_CLEARANCE_RATIO);
    assert_eq!(result.samples.first().unwrap().position, tx.position);
    assert_eq!(result.samples.last().unwrap().position, rx.position);
    assert!(result
        .samples
        .iter()
        .all(|sample| sample.status == ClearanceStatus::Clear));
    assert!(result.worst_point.is_some());
}

#[test]
fn link_analysis_identifies_los_and_fresnel_obstruction() {
    let (tx, rx) = endpoints();
    let mut terrain = SyntheticTerrain { obstructed: true };

    let result = analyze_link(
        &mut terrain,
        tx,
        rx,
        2.4e9,
        21,
        DEFAULT_K_FACTOR,
        DEFAULT_FRESNEL_CLEARANCE_RATIO,
    )
    .unwrap();

    assert!(result.los_blocked);
    assert!(result.fresnel_60_blocked);
    let worst = result.worst_point.unwrap();
    assert!(worst.clearance_m < 0.0);
    assert!(worst.distance_m > 0.0);
    assert!(worst.distance_m < result.distance_m);
    assert!(result
        .samples
        .iter()
        .any(|sample| sample.status == ClearanceStatus::LineOfSightBlocked));
}
