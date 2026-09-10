"""Independent, readable geographic reference calculations for development."""

import math

EARTH_RADIUS_M = 6_371_000.0


def distance_m(a: tuple[float, float], b: tuple[float, float]) -> float:
    lat1, lon1 = map(math.radians, a)
    lat2, lon2 = map(math.radians, b)
    dlat = lat2 - lat1
    dlon = lon2 - lon1
    h = math.sin(dlat / 2) ** 2 + math.cos(lat1) * math.cos(lat2) * math.sin(dlon / 2) ** 2
    return 2 * EARTH_RADIUS_M * math.asin(math.sqrt(min(1.0, h)))


def interpolate(a: tuple[float, float], b: tuple[float, float], fraction: float) -> tuple[float, float]:
    if not 0.0 <= fraction <= 1.0 or not math.isfinite(fraction):
        raise ValueError("interpolation fraction must be in [0,1]")
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
    if angle < 1e-12:
        x, y, z = v1
    else:
        sin_angle = math.sin(angle)
        w1 = math.sin((1.0 - fraction) * angle) / sin_angle
        w2 = math.sin(fraction * angle) / sin_angle
        x = w1 * v1[0] + w2 * v2[0]
        y = w1 * v1[1] + w2 * v2[1]
        z = w1 * v1[2] + w2 * v2[2]
    return math.degrees(math.atan2(z, math.hypot(x, y))), math.degrees(math.atan2(y, x))


def sample(a: tuple[float, float], b: tuple[float, float], count: int) -> list[tuple[float, float]]:
    if count < 2:
        raise ValueError("samples must be at least 2")
    return [interpolate(a, b, i / (count - 1)) for i in range(count)]
