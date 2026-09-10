"""Independent end-to-end reference profile for RF-Path validation.

Production implementation: Rust. This script intentionally uses plain Python
math and a deterministic flat-terrain case so the Rust integration test can
compare an independently generated numerical vector.
"""

from __future__ import annotations

import json
import math
from pathlib import Path

EARTH_RADIUS_M = 6_371_000.0
SPEED_OF_LIGHT_M_S = 299_792_458.0
K_FACTOR = 4.0 / 3.0
FREQUENCY_HZ = 2.4e9
SAMPLE_COUNT = 21
TX = (0.0, 0.0)
RX = (0.0, 0.01)
ANTENNA_HEIGHT_M = 30.0


def great_circle_distance_m(a: tuple[float, float], b: tuple[float, float]) -> float:
    lat1, lon1 = map(math.radians, a)
    lat2, lon2 = map(math.radians, b)
    dlat = lat2 - lat1
    dlon = lon2 - lon1
    h = math.sin(dlat / 2.0) ** 2 + math.cos(lat1) * math.cos(lat2) * math.sin(dlon / 2.0) ** 2
    return 2.0 * EARTH_RADIUS_M * math.asin(math.sqrt(min(1.0, h)))


def great_circle_interpolate(
    a: tuple[float, float], b: tuple[float, float], fraction: float
) -> tuple[float, float]:
    if fraction == 0.0:
        return a
    if fraction == 1.0:
        return b
    lat1, lon1 = map(math.radians, a)
    lat2, lon2 = map(math.radians, b)
    v1 = (math.cos(lat1) * math.cos(lon1), math.cos(lat1) * math.sin(lon1), math.sin(lat1))
    v2 = (math.cos(lat2) * math.cos(lon2), math.cos(lat2) * math.sin(lon2), math.sin(lat2))
    dot = max(-1.0, min(1.0, sum(x * y for x, y in zip(v1, v2))))
    angle = math.acos(dot)
    sin_angle = math.sin(angle)
    w1 = math.sin((1.0 - fraction) * angle) / sin_angle
    w2 = math.sin(fraction * angle) / sin_angle
    x = w1 * v1[0] + w2 * v2[0]
    y = w1 * v1[1] + w2 * v2[1]
    z = w1 * v1[2] + w2 * v2[2]
    return math.degrees(math.atan2(z, math.hypot(x, y))), math.degrees(math.atan2(y, x))


def build_reference() -> dict[str, object]:
    distance_m = great_circle_distance_m(TX, RX)
    wavelength_m = SPEED_OF_LIGHT_M_S / FREQUENCY_HZ
    fspl_db = 20.0 * math.log10(
        4.0 * math.pi * distance_m * FREQUENCY_HZ / SPEED_OF_LIGHT_M_S
    )
    samples: list[dict[str, object]] = []
    for index in range(SAMPLE_COUNT):
        fraction = index / (SAMPLE_COUNT - 1)
        d1_m = distance_m * fraction
        d2_m = distance_m - d1_m
        lat_deg, lon_deg = great_circle_interpolate(TX, RX, fraction)
        earth_bulge_m = d1_m * d2_m / (2.0 * K_FACTOR * EARTH_RADIUS_M)
        los_m = ANTENNA_HEIGHT_M - earth_bulge_m
        fresnel_m = math.sqrt(wavelength_m * d1_m * d2_m / distance_m)
        clearance_m = los_m
        ratio = 1.0 if fresnel_m == 0.0 else clearance_m / fresnel_m
        samples.append(
            {
                "index": index,
                "distance_m": d1_m,
                "lat_deg": lat_deg,
                "lon_deg": lon_deg,
                "terrain_m": 0.0,
                "los_m": los_m,
                "earth_bulge_m": earth_bulge_m,
                "fresnel_radius_m": fresnel_m,
                "clearance_m": clearance_m,
                "clearance_ratio": ratio,
                "status": "Clear",
            }
        )
    return {
        "distance_m": distance_m,
        "frequency_hz": FREQUENCY_HZ,
        "wavelength_m": wavelength_m,
        "fspl_db": fspl_db,
        "tx_ground_m": 0.0,
        "rx_ground_m": 0.0,
        "tx_altitude_m": ANTENNA_HEIGHT_M,
        "rx_altitude_m": ANTENNA_HEIGHT_M,
        "min_clearance_m": min(sample["clearance_m"] for sample in samples),
        "min_clearance_ratio": min(sample["clearance_ratio"] for sample in samples),
        "los_blocked": False,
        "fresnel_60_blocked": False,
        "samples": samples,
    }


def main() -> None:
    output = Path(__file__).with_name("link_analysis_clear_reference.json")
    output.write_text(json.dumps(build_reference(), separators=(",", ":"), allow_nan=False) + "\n")
    print(output)


if __name__ == "__main__":
    main()
