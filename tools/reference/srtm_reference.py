"""Reference SRTM/HGT calculations for development validation.

Production implementation: Rust. Real SRTM datasets are not stored here.
"""

import math
import struct
from pathlib import Path

GRID_SIZE = 1201
VOID = -32768


def tile_name(lat: float, lon: float) -> str:
    south = math.floor(lat)
    west = math.floor(lon)
    return f"{'N' if south >= 0 else 'S'}{abs(south):02d}{'E' if west >= 0 else 'W'}{abs(west):03d}.hgt"


def sample_offset(row: int, col: int) -> int:
    return (row * GRID_SIZE + col) * 2


def decode_sample(data: bytes, row: int, col: int) -> int:
    return struct.unpack_from(">h", data, sample_offset(row, col))[0]


def bilinear_elevation(data: bytes, lat: float, lon: float) -> float:
    south = math.floor(lat)
    west = math.floor(lon)
    x = max(0.0, min(1200.0, (lon - west) * 1200.0))
    y = max(0.0, min(1200.0, (south + 1 - lat) * 1200.0))
    x0 = math.floor(x)
    y0 = math.floor(y)
    x1 = min(1200, x0 + 1)
    y1 = min(1200, y0 + 1)
    xf = x - x0
    yf = y - y0

    values = (
        decode_sample(data, y0, x0),
        decode_sample(data, y0, x1),
        decode_sample(data, y1, x0),
        decode_sample(data, y1, x1),
    )
    if any(value == VOID for value in values):
        raise ValueError("SRTM NoData in interpolation neighborhood")

    top = values[0] + (values[1] - values[0]) * xf
    bottom = values[2] + (values[3] - values[2]) * xf
    return top + (bottom - top) * yf


def read_tile(path: str | Path) -> bytes:
    return Path(path).read_bytes()
