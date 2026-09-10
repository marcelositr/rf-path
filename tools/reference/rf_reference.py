"""Reference RF calculations for development validation.

Production implementation: Rust. Keep formulas explicit and readable.
"""

import math

SPEED_OF_LIGHT_M_S = 299_792_458.0


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
