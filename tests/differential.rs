//! Rust/Python end-to-end differential regression tests.

use approx::assert_relative_eq;
use serde::Deserialize;

use rf_path::analysis::{
    analyze_link, DEFAULT_FRESNEL_CLEARANCE_RATIO, DEFAULT_K_FACTOR,
};
use rf_path::geo::{AntennaPoint, GeoPoint};
use rf_path::srtm::TerrainProvider;

struct FlatTerrain;

impl TerrainProvider for FlatTerrain {
    fn elevation_at(&mut self, _point: GeoPoint) -> rf_path::error::Result<f64> {
        Ok(0.0)
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
    let mut terrain = FlatTerrain;

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
    assert_relative_eq!(
        result.min_clearance_m,
        reference.min_clearance_m,
        epsilon = 1e-9
    );
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
