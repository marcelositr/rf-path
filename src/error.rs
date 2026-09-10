//! Application and domain error types.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid SRTM tile: {0}")]
    InvalidTile(String),
    #[error("SRTM no-data at {lat}, {lon}")]
    NoData { lat: f64, lon: f64 },
}

pub type Result<T> = std::result::Result<T, Error>;
