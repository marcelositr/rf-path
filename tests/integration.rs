//! End-to-end application integration tests.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use approx::assert_relative_eq;

use rf_path::analysis::{
    analyze_link, ClearanceStatus, DEFAULT_FRESNEL_CLEARANCE_RATIO, DEFAULT_K_FACTOR,
};
use rf_path::error::Error;
use rf_path::geo::{AntennaPoint, GeoPoint};
use rf_path::srtm::{SrtmProvider, TerrainProvider, TileKey, SRTM3_SAMPLES};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

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

fn temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "rf-path-integration-{}-{nonce}-{counter}",
        std::process::id()
    ));
    fs::create_dir_all(&path).unwrap();
    path
}

fn write_uniform_tile(directory: &Path, key: TileKey, value: i16) {
    let path = directory.join(key.filename());
    let mut file = File::create(path).unwrap();
    let bytes = value.to_be_bytes();
    for _ in 0..SRTM3_SAMPLES * SRTM3_SAMPLES {
        file.write_all(&bytes).unwrap();
    }
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

#[test]
fn link_analysis_reports_missing_srtm_tile() {
    let directory = temp_dir();
    let (tx, rx) = endpoints();
    let mut terrain = SrtmProvider::new(&directory);

    let result = analyze_link(
        &mut terrain,
        tx,
        rx,
        2.4e9,
        2,
        DEFAULT_K_FACTOR,
        DEFAULT_FRESNEL_CLEARANCE_RATIO,
    );

    assert!(matches!(result, Err(Error::Io(_))));
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn link_analysis_reports_srtm_nodata_at_endpoint() {
    let directory = temp_dir();
    let key = TileKey {
        south_lat: 0,
        west_lon: 0,
    };
    write_uniform_tile(&directory, key, 0);

    let path = directory.join(key.filename());
    let mut bytes = fs::read(&path).unwrap();
    bytes[0..2].copy_from_slice(&(-32768i16).to_be_bytes());
    fs::write(&path, bytes).unwrap();

    let tx = AntennaPoint {
        position: GeoPoint::new(1.0, 0.0).unwrap(),
        antenna_height_m: 30.0,
    };
    let rx = AntennaPoint {
        position: GeoPoint::new(1.0, 0.01).unwrap(),
        antenna_height_m: 30.0,
    };
    let mut terrain = SrtmProvider::new(&directory);

    let result = analyze_link(
        &mut terrain,
        tx,
        rx,
        2.4e9,
        2,
        DEFAULT_K_FACTOR,
        DEFAULT_FRESNEL_CLEARANCE_RATIO,
    );

    assert!(matches!(result, Err(Error::NoData { lat, lon }) if lat == 1.0 && lon == 0.0));
    fs::remove_dir_all(directory).unwrap();
}
