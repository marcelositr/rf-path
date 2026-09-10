//! Unit types and conversions. Internal calculations use SI units.

use crate::error::{Error, Result};

pub const SPEED_OF_LIGHT_M_S: f64 = 299_792_458.0;

pub fn parse_frequency_hz(input: &str) -> Result<f64> {
    let value = input.trim().to_ascii_lowercase();
    let (number, multiplier) = if let Some(v) = value.strip_suffix("ghz") {
        (v, 1.0e9)
    } else if let Some(v) = value.strip_suffix("mhz") {
        (v, 1.0e6)
    } else if let Some(v) = value.strip_suffix("khz") {
        (v, 1.0e3)
    } else if let Some(v) = value.strip_suffix("hz") {
        (v, 1.0)
    } else {
        return Err(Error::InvalidInput(format!("frequency must include Hz/kHz/MHz/GHz: {input}")));
    };

    let hz = number
        .trim()
        .parse::<f64>()
        .map_err(|_| Error::InvalidInput(format!("invalid frequency: {input}")))?
        * multiplier;
    if !hz.is_finite() || hz <= 0.0 {
        return Err(Error::InvalidInput(format!("frequency must be positive: {input}")));
    }
    Ok(hz)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_common_frequency_units() {
        assert_eq!(parse_frequency_hz("2.4GHz").unwrap(), 2.4e9);
        assert_eq!(parse_frequency_hz("2400MHz").unwrap(), 2.4e9);
        assert_eq!(parse_frequency_hz("2400000000Hz").unwrap(), 2.4e9);
    }

    #[test]
    fn rejects_unitless_frequency() {
        assert!(parse_frequency_hz("2.4").is_err());
    }
}
