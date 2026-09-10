"""Reference SRTM/HGT calculations for development validation.

Production implementation: Rust. Real SRTM datasets are not stored here.
"""

import math

GRID_SIZE = 1201
VOID = -32768


def tile_name(lat: float, lon: float) -> str:
    south = math.floor(lat)
    west = math.floor(lon)
    return f"{'N' if south >= 0 else 'S'}{abs(south):02d}{'E' if west >= 0 else 'W'}{abs(west):03d}.hgt"


def sample_offset(row: int, col: int) -> int:
    return (row * GRID_SIZE + col) * 2
