//! End-to-end application integration tests.

use approx::assert_relative_eq;
use serde::Deserialize;

use rf_path::analysis::{
    analyze_link, ClearanceStatus, DEFAULT_FRESNEL_CLEARANCE_RATIO, DEFAULT_K_FACTOR,
};
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

#[derive(Debug, Deserialize)]
struct ReferenceVector {
    distance_m: f64,
    frequency_hz: f64,
    wavelength_m: f64,
    fspl_db: f64,
    tx_ground_m: f64,
    rx_ground_m: f64,
    tx_altitude_m: f64,
    rx_altitude_m: f64,
    min_clearance_m: f64,
    min_clearance_ratio: f64,
    los_blocked: bool,
    fresnel_60_blocked: bool,
    samples: Vec<ReferenceSample>,
}

#[derive(Debug, Deserialize)]
struct ReferenceSample {
    index: usize,
    distance_m: f64,
    lat_deg: f64,
    lon_deg: f64,
    terrain_m: f64,
    los_m: f64,
    earth_bulge_m: f64,
    fresnel_radius_m: f64,
    clearance_m: f64,
    clearance_ratio: f64,
    status: String,
}

#[test]
fn link_analysis_matches_independent_python_reference_vector() {
    let reference: ReferenceVector = serde_json::from_str(include_str!(
        "../tools/reference/link_analysis_clear_reference.json"
    ))
    .unwrap();
    let (tx, rx) = endpoints();
    let mut terrain = SyntheticTerrain { obstructed: false };

    let result = analyze_link(
        &mut terrain,
        tx,
        rx,
        reference.frequency_hz,
        reference.samples.len(),
        DEFAULT_K_FACTOR,
        DEFAULT_FRESNEL_CLEARANCE_RATIO,
    )
    .unwrap();

    assert_relative_eq!(result.distance_m, reference.distance_m, epsilon = 1e-9);
    assert_relative_eq!(result.frequency_hz, reference.frequency_hz, epsilon = 1e-6);
    assert_relative_eq!(result.wavelength_m, reference.wavelength_m, epsilon = 1e-12);
    assert_relative_eq!(result.fspl_db, reference.fspl_db, epsilon = 1e-9);
    assert_relative_eq!(result.tx_ground_m, reference.tx_ground_m, epsilon = 1e-12);
    assert_relative_eq!(result.rx_ground_m, reference.rx_ground_m, epsilon = 1e-12);
    assert_relative_eq!(
        result.tx_altitude_m,
        reference.tx_altitude_m,
        epsilon = 1e-12
    );
    assert_relative_eq!(
        result.rx_altitude_m,
        reference.rx_altitude_m,
        epsilon = 1e-12
    );
    assert_relative_eq!(result.min_clearance_m, reference.min_clearance_m, epsilon = 1e-9);
    assert_eq!(result.min_clearance_ratio, reference.min_clearance_ratio);
    assert_eq!(result.los_blocked, reference.los_blocked);
    assert_eq!(result.fresnel_60_blocked, reference.fresnel_60_blocked);
    assert_eq!(result.samples.len(), reference.samples.len());

    for (actual_index, (actual, expected)) in result
        .samples
        .iter()
        .zip(reference.samples.iter())
        .enumerate()
    {
        assert_eq!(expected.index, actual_index);
        assert_relative_eq!(actual.distance_m, expected.distance_m, epsilon = 1e-9);
        assert_relative_eq!(actual.position.lat_deg, expected.lat_deg, epsilon = 1e-12);
        assert_relative_eq!(actual.position.lon_deg, expected.lon_deg, epsilon = 1e-12);
        assert_relative_eq!(actual.terrain_m, expected.terrain_m, epsilon = 1e-12);
        assert_relative_eq!(actual.los_m, expected.los_m, epsilon = 1e-9);
        assert_relative_eq!(
            actual.earth_bulge_m,
            expected.earth_bulge_m,
            epsilon = 1e-12
        );
        assert_relative_eq!(
            actual.fresnel_radius_m,
            expected.fresnel_radius_m,
            epsilon = 1e-9
        );
        assert_relative_eq!(actual.clearance_m, expected.clearance_m, epsilon = 1e-9);
        assert_relative_eq!(
            actual.clearance_ratio,
            expected.clearance_ratio,
            epsilon = 1e-9
        );
        assert_eq!(format!("{:?}", actual.status), expected.status);
    }
}
