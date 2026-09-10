use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use approx::assert_relative_eq;
use rf_path::geo::GeoPoint;
use rf_path::srtm::{SrtmProvider, TerrainProvider, TileKey, SRTM3_SAMPLES};

fn temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("rf-path-srtm-{}-{nonce}", std::process::id()));
    fs::create_dir_all(&path).unwrap();
    path
}

fn write_tile<F>(directory: &Path, key: TileKey, mut value: F)
where
    F: FnMut(usize, usize) -> i16,
{
    let path = directory.join(key.filename());
    let mut file = File::create(path).unwrap();
    for row in 0..SRTM3_SAMPLES {
        for column in 0..SRTM3_SAMPLES {
            file.write_all(&value(row, column).to_be_bytes()).unwrap();
        }
    }
}

#[test]
fn negative_coordinates_use_floor_tile_semantics() {
    let point = GeoPoint::new(-20.2831, -47.7812).unwrap();
    let key = TileKey::from_point(point);
    assert_eq!(key.south_lat, -21);
    assert_eq!(key.west_lon, -48);
    assert_eq!(key.filename(), "S21W048.hgt");
}

#[test]
fn exact_integer_boundaries_select_the_northeast_tile() {
    let cases = [
        (
            GeoPoint::new(1.0, 0.5).unwrap(),
            TileKey {
                south_lat: 1,
                west_lon: 0,
            },
        ),
        (
            GeoPoint::new(0.5, 1.0).unwrap(),
            TileKey {
                south_lat: 0,
                west_lon: 1,
            },
        ),
        (
            GeoPoint::new(-20.0, -48.0).unwrap(),
            TileKey {
                south_lat: -20,
                west_lon: -48,
            },
        ),
    ];

    for (point, expected) in cases {
        assert_eq!(TileKey::from_point(point), expected);
    }
}

#[test]
fn bilinear_interpolation_respects_hgt_row_orientation() {
    let directory = temp_dir();
    let key = TileKey {
        south_lat: 0,
        west_lon: 0,
    };
    write_tile(&directory, key, |row, column| {
        1000 + row as i16 * 10 + column as i16 * 4
    });

    let lat = 1.0 - 0.5 / 1200.0;
    let lon = 0.25 / 1200.0;
    let point = GeoPoint::new(lat, lon).unwrap();
    let mut provider = SrtmProvider::new(&directory);
    let elevation = provider.elevation_at(point).unwrap();

    let expected = 1000.0 + 0.5 * 10.0 + 0.25 * 4.0;
    assert_relative_eq!(elevation, expected, max_relative = 1e-12);

    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn interpolation_matches_controlled_python_reference_vectors() {
    let directory = temp_dir();
    let key = TileKey {
        south_lat: 0,
        west_lon: 0,
    };
    write_tile(&directory, key, |row, column| {
        1000 + row as i16 * 10 + column as i16 * 4
    });

    // Values independently evaluated with tools/reference/srtm_reference.py.
    let vectors = [
        (1.0, 0.0, 1000.0),
        (
            1.0 - 600.5 / 1200.0,
            400.25 / 1200.0,
            1000.0 + 600.5 * 10.0 + 400.25 * 4.0,
        ),
        (0.0, 1.0, 1000.0 + 1200.0 * 10.0 + 1200.0 * 4.0),
    ];

    let mut provider = SrtmProvider::new(&directory);
    for (lat, lon, expected) in vectors {
        let point = GeoPoint::new(lat, lon).unwrap();
        let elevation = provider.elevation_at(point).unwrap();
        assert_relative_eq!(elevation, expected, max_relative = 1e-12);
    }

    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn interpolation_handles_exact_tile_corners() {
    let directory = temp_dir();
    let keys = [
        TileKey {
            south_lat: 0,
            west_lon: 0,
        },
        TileKey {
            south_lat: 0,
            west_lon: 1,
        },
        TileKey {
            south_lat: 1,
            west_lon: 0,
        },
        TileKey {
            south_lat: 1,
            west_lon: 1,
        },
    ];
    for key in keys {
        write_tile(&directory, key, |row, column| {
            row as i16 * 10 + column as i16
        });
    }

    let points = [
        (GeoPoint::new(1.0, 0.0).unwrap(), 0.0),
        (GeoPoint::new(1.0, 1.0).unwrap(), 0.0),
        (GeoPoint::new(0.0, 0.0).unwrap(), 12000.0),
        (GeoPoint::new(0.0, 1.0).unwrap(), 12000.0),
    ];

    let mut provider = SrtmProvider::new(&directory);
    for (point, expected) in points {
        let elevation = provider.elevation_at(point).unwrap();
        assert_relative_eq!(elevation, expected, max_relative = 1e-12);
    }

    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn malformed_tile_size_is_rejected() {
    let directory = temp_dir();
    let key = TileKey {
        south_lat: 0,
        west_lon: 0,
    };
    let path = directory.join(key.filename());
    let mut file = File::create(path).unwrap();
    file.write_all(&[0, 1]).unwrap();

    let result = rf_path::srtm::SrtmTile::open(&directory, key);
    assert!(matches!(
        result,
        Err(rf_path::error::Error::InvalidTile(_))
    ));

    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn nodata_is_reported_instead_of_interpolated() {
    let directory = temp_dir();
    let key = TileKey {
        south_lat: 0,
        west_lon: 0,
    };
    write_tile(&directory, key, |row, column| {
        if row == 600 && column == 600 {
            -32768
        } else {
            1000
        }
    });

    let point = GeoPoint::new(1.0 - 600.25 / 1200.0, 600.25 / 1200.0).unwrap();
    let mut provider = SrtmProvider::new(&directory);
    assert!(matches!(
        provider.elevation_at(point),
        Err(rf_path::error::Error::NoData { .. })
    ));

    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn provider_caches_tiles_and_switches_at_boundaries() {
    let directory = temp_dir();
    write_tile(
        &directory,
        TileKey {
            south_lat: 0,
            west_lon: 0,
        },
        |_, _| 100,
    );
    write_tile(
        &directory,
        TileKey {
            south_lat: 0,
            west_lon: 1,
        },
        |_, _| 200,
    );

    let mut provider = SrtmProvider::new(&directory);
    let west = GeoPoint::new(0.5, 0.5).unwrap();
    let east = GeoPoint::new(0.5, 1.5).unwrap();

    assert_relative_eq!(provider.elevation_at(west).unwrap(), 100.0);
    assert_eq!(provider.cached_tile_count(), 1);
    assert_relative_eq!(provider.elevation_at(east).unwrap(), 200.0);
    assert_eq!(provider.cached_tile_count(), 2);
    assert_relative_eq!(provider.elevation_at(west).unwrap(), 100.0);
    assert_eq!(provider.cached_tile_count(), 2);

    fs::remove_dir_all(directory).unwrap();
}
