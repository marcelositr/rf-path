//! SRTM HGT terrain access, interpolation, and tile caching.

use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use memmap2::Mmap;

use crate::error::{Error, Result};
use crate::geo::GeoPoint;

pub const SRTM3_SAMPLES: usize = 1201;
const SAMPLE_BYTES: usize = 2;
const EXPECTED_FILE_BYTES: usize = SRTM3_SAMPLES * SRTM3_SAMPLES * SAMPLE_BYTES;
const NODATA: i16 = -32768;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileKey {
    pub south_lat: i32,
    pub west_lon: i32,
}

impl TileKey {
    pub fn from_point(point: GeoPoint) -> Self {
        Self {
            south_lat: point.lat_deg.floor() as i32,
            west_lon: point.lon_deg.floor() as i32,
        }
    }

    pub fn filename(self) -> String {
        let lat_prefix = if self.south_lat >= 0 { 'N' } else { 'S' };
        let lon_prefix = if self.west_lon >= 0 { 'E' } else { 'W' };
        format!(
            "{}{:02}{}{:03}.hgt",
            lat_prefix,
            self.south_lat.unsigned_abs(),
            lon_prefix,
            self.west_lon.unsigned_abs()
        )
    }
}

pub struct SrtmTile {
    key: TileKey,
    path: PathBuf,
    mmap: Mmap,
}

impl std::fmt::Debug for SrtmTile {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SrtmTile")
            .field("key", &self.key)
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}

impl SrtmTile {
    pub fn open(directory: impl AsRef<Path>, key: TileKey) -> Result<Self> {
        let path = directory.as_ref().join(key.filename());
        let file = File::open(&path)?;
        let metadata = file.metadata()?;
        if metadata.len() as usize != EXPECTED_FILE_BYTES {
            return Err(Error::InvalidTile(format!(
                "{} has {} bytes; expected {}",
                path.display(),
                metadata.len(),
                EXPECTED_FILE_BYTES
            )));
        }
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self { key, path, mmap })
    }

    fn sample(&self, row: usize, column: usize) -> Result<i16> {
        if row >= SRTM3_SAMPLES || column >= SRTM3_SAMPLES {
            return Err(Error::InvalidTile(format!(
                "sample index out of bounds: row={row}, column={column}"
            )));
        }
        let offset = (row * SRTM3_SAMPLES + column) * SAMPLE_BYTES;
        Ok(i16::from_be_bytes([
            self.mmap[offset],
            self.mmap[offset + 1],
        ]))
    }

    fn elevation_bilinear(&self, point: GeoPoint) -> Result<f64> {
        let x = ((point.lon_deg - self.key.west_lon as f64) * 1200.0).clamp(0.0, 1200.0);
        let y = (((self.key.south_lat + 1) as f64 - point.lat_deg) * 1200.0).clamp(0.0, 1200.0);

        let x0 = x.floor() as usize;
        let y0 = y.floor() as usize;
        let x1 = (x0 + 1).min(1200);
        let y1 = (y0 + 1).min(1200);

        let x_frac = x - x0 as f64;
        let y_frac = y - y0 as f64;

        let samples = [
            self.sample(y0, x0)?,
            self.sample(y0, x1)?,
            self.sample(y1, x0)?,
            self.sample(y1, x1)?,
        ];
        if samples.iter().any(|&value| value == NODATA) {
            return Err(Error::NoData {
                lat: point.lat_deg,
                lon: point.lon_deg,
            });
        }

        let q11 = samples[0] as f64;
        let q21 = samples[1] as f64;
        let q12 = samples[2] as f64;
        let q22 = samples[3] as f64;
        let top = q11 + (q21 - q11) * x_frac;
        let bottom = q12 + (q22 - q12) * x_frac;
        Ok(top + (bottom - top) * y_frac)
    }
}

pub trait TerrainProvider {
    fn elevation_at(&mut self, point: GeoPoint) -> Result<f64>;
}

pub struct SrtmProvider {
    directory: PathBuf,
    cache: HashMap<TileKey, SrtmTile>,
}

impl std::fmt::Debug for SrtmProvider {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SrtmProvider")
            .field("directory", &self.directory)
            .field("cached_tiles", &self.cache.len())
            .finish()
    }
}

impl SrtmProvider {
    pub fn new(directory: impl Into<PathBuf>) -> Self {
        Self {
            directory: directory.into(),
            cache: HashMap::new(),
        }
    }

    pub fn cached_tile_count(&self) -> usize {
        self.cache.len()
    }

    fn tile(&mut self, key: TileKey) -> Result<&SrtmTile> {
        if !self.cache.contains_key(&key) {
            let tile = SrtmTile::open(&self.directory, key)?;
            self.cache.insert(key, tile);
        }
        self.cache
            .get(&key)
            .ok_or_else(|| Error::InvalidTile(format!("tile cache insertion failed: {}", key.filename())))
    }
}

impl TerrainProvider for SrtmProvider {
    fn elevation_at(&mut self, point: GeoPoint) -> Result<f64> {
        let key = TileKey::from_point(point);
        self.tile(key)?.elevation_bilinear(point)
    }
}
