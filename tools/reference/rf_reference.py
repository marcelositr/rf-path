"""Reference RF calculations for development validation.

Production implementation: Rust. Keep formulas explicit and readable.
"""

import math

SPEED_OF_LIGHT_M_S = 299_792_458.0
EARTH_RADIUS_M = 6_371_000.0
DEFAULT_K_FACTOR = 4.0 / 3.0
DEFAULT_FRESNEL_CLEARANCE_RATIO = 0.60


def wavelength_m(frequency_hz: float) -> float:
    if frequency_hz <= 0 or not math.isfinite(frequency_hz):
        raise ValueError("frequency must be positive")
    return SPEED_OF_LIGHT_M_S / frequency_hz


def fresnel_radius_m(frequency_hz: float, d1_m: float, d2_m: float) -> float:
    if d1_m < 0 or d2_m < 0 or d1_m + d2_m <= 0:
        raise ValueError("invalid path distances")
    return math.sqrt(wavelength_m(frequency_hz) * d1_m * d2_m / (d1_m + d2_m))


def free_space_path_loss_db(distance_m: float, frequency_hz: float) -> float:
    if distance_m <= 0 or frequency_hz <= 0:
        raise ValueError("distance and frequency must be positive")
    return 20 * math.log10(4 * math.pi * distance_m * frequency_hz / SPEED_OF_LIGHT_M_S)


def effective_earth_radius_m(k_factor: float) -> float:
    if k_factor <= 0 or not math.isfinite(k_factor):
        raise ValueError("k-factor must be positive")
    return k_factor * EARTH_RADIUS_M


def earth_bulge_m(d1_m: float, d2_m: float, k_factor: float) -> float:
    if d1_m < 0 or d2_m < 0 or not math.isfinite(d1_m) or not math.isfinite(d2_m):
        raise ValueError("invalid path distances")
    total = d1_m + d2_m
    if total <= 0:
        raise ValueError("path distance must be positive")
    return d1_m * d2_m / (2 * effective_earth_radius_m(k_factor))


def reference_path_elevation_m(
    tx_altitude_m: float,
    rx_altitude_m: float,
    d1_m: float,
    d2_m: float,
) -> float:
    if not math.isfinite(tx_altitude_m) or not math.isfinite(rx_altitude_m):
        raise ValueError("endpoint altitudes must be finite")
    if d1_m < 0 or d2_m < 0 or not math.isfinite(d1_m) or not math.isfinite(d2_m):
        raise ValueError("invalid path distances")
    total = d1_m + d2_m
    if total <= 0:
        raise ValueError("path distance must be positive")
    return tx_altitude_m + (rx_altitude_m - tx_altitude_m) * d1_m / total


def effective_reference_path_elevation_m(
    tx_altitude_m: float,
    rx_altitude_m: float,
    d1_m: float,
    d2_m: float,
    k_factor: float,
) -> float:
    return reference_path_elevation_m(tx_altitude_m, rx_altitude_m, d1_m, d2_m) - earth_bulge_m(
        d1_m, d2_m, k_factor
    )


def terrain_clearance_m(
    terrain_m: float,
    tx_altitude_m: float,
    rx_altitude_m: float,
    d1_m: float,
    d2_m: float,
    k_factor: float,
) -> float:
    if not math.isfinite(terrain_m):
        raise ValueError("terrain altitude must be finite")
    return effective_reference_path_elevation_m(
        tx_altitude_m, rx_altitude_m, d1_m, d2_m, k_factor
    ) - terrain_m


def classify_clearance(
    clearance_m: float,
    fresnel_radius_m: float,
    required_fraction: float = DEFAULT_FRESNEL_CLEARANCE_RATIO,
) -> str:
    if not math.isfinite(clearance_m) or not math.isfinite(fresnel_radius_m):
        raise ValueError("clearance values must be finite")
    if fresnel_radius_m < 0:
        raise ValueError("Fresnel radius cannot be negative")
    if not math.isfinite(required_fraction) or not 0 <= required_fraction <= 1:
        raise ValueError("Fresnel clearance fraction must be between 0 and 1")
    if clearance_m < 0:
        return "LineOfSightBlocked"
    if fresnel_radius_m > 0 and clearance_m < required_fraction * fresnel_radius_m:
        return "FresnelPartial"
    return "Clear"
