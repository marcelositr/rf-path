use std::fs::{self, File};
use std::hint::black_box;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

use rf_path::geo::GeoPoint;
use rf_path::srtm::{SrtmProvider, TerrainProvider, TileKey, SRTM3_SAMPLES};

const QUERIES: usize = 100_000;

fn write_uniform_tile(directory: &Path, key: TileKey, value: i16) {
    let path = directory.join(key.filename());
    let bytes = value.to_be_bytes();
    let data = vec![bytes[0], bytes[1]]
        .into_iter()
        .cycle()
        .take(SRTM3_SAMPLES * SRTM3_SAMPLES * 2)
        .collect::<Vec<_>>();
    let mut file = File::create(path).unwrap();
    file.write_all(&data).unwrap();
}

fn benchmark_cached_tile(directory: &Path) -> (u128, usize) {
    let mut provider = SrtmProvider::new(directory);
    let first = GeoPoint::new(0.5, 0.5).unwrap();
    black_box(provider.elevation_at(first).unwrap());

    let start = Instant::now();
    for index in 0..QUERIES {
        let fraction = (index % 1000) as f64 / 1000.0;
        let point = GeoPoint::new(0.25 + fraction * 0.5, 0.25 + fraction * 0.5).unwrap();
        black_box(provider.elevation_at(point).unwrap());
    }
    (start.elapsed().as_nanos(), provider.cached_tile_count())
}

fn benchmark_two_cached_tiles(directory: &Path) -> (u128, usize) {
    let mut provider = SrtmProvider::new(directory);

    let start = Instant::now();
    for index in 0..QUERIES {
        let fraction = (index % 1000) as f64 / 1000.0;
        let longitude = if index % 2 == 0 {
            0.25 + fraction * 0.5
        } else {
            1.25 + fraction * 0.5
        };
        let point = GeoPoint::new(0.25 + fraction * 0.5, longitude).unwrap();
        black_box(provider.elevation_at(point).unwrap());
    }
    (start.elapsed().as_nanos(), provider.cached_tile_count())
}

fn main() {
    let directory = std::env::temp_dir().join(format!("rf-path-bench-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    write_uniform_tile(
        &directory,
        TileKey {
            south_lat: 0,
            west_lon: 0,
        },
        100,
    );
    write_uniform_tile(
        &directory,
        TileKey {
            south_lat: 0,
            west_lon: 1,
        },
        200,
    );

    let (single_ns, single_tiles) = benchmark_cached_tile(&directory);
    let (two_ns, two_tiles) = benchmark_two_cached_tiles(&directory);

    println!("SRTM access benchmark ({} queries)", QUERIES);
    println!(
        "single cached tile: total={} ms, {:.1} ns/query, cached_tiles={}",
        single_ns as f64 / 1_000_000.0,
        single_ns as f64 / QUERIES as f64,
        single_tiles
    );
    println!(
        "two cached tiles:    total={} ms, {:.1} ns/query, cached_tiles={}",
        two_ns as f64 / 1_000_000.0,
        two_ns as f64 / QUERIES as f64,
        two_tiles
    );

    fs::remove_dir_all(directory).unwrap();
}
