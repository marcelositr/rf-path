"""Small differential-comparison helpers for Rust/Python validation."""


def assert_close(name: str, rust_value: float, python_value: float, tolerance: float) -> None:
    difference = abs(rust_value - python_value)
    if difference > tolerance:
        raise AssertionError(
            f"{name}: Rust={rust_value!r}, Python={python_value!r}, "
            f"difference={difference!r}, tolerance={tolerance!r}"
        )
