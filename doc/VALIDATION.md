# RF-Path — Reproducible Validation Examples

This document records validation commands that can be reproduced without relying on chat history.

## Static and automated Rust validation

Run the same quality gates used by GitHub Actions:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

The repository's differential regression is included in `cargo test` and compares the production Rust analysis against the independent Python reference vector.

## CLI validation without SRTM data access

The CLI validates analysis controls and the SRTM directory before attempting terrain analysis. These cases are useful for verifying deterministic diagnostics without requiring a downloaded DEM.

Too few samples:

```bash
cargo run -- \
  --tx="0,0,10" \
  --rx="0,0.01,10" \
  --freq=2.4GHz \
  --srtm-dir=./missing-srtm \
  --samples=1
```

The expected result is a non-zero exit with an `--samples` diagnostic before terrain access is attempted.

Invalid k-factor:

```bash
cargo run -- \
  --tx="0,0,10" \
  --rx="0,0.01,10" \
  --freq=2.4GHz \
  --srtm-dir=./missing-srtm \
  --k-factor=0
```

Invalid Fresnel threshold:

```bash
cargo run -- \
  --tx="0,0,10" \
  --rx="0,0.01,10" \
  --freq=2.4GHz \
  --srtm-dir=./missing-srtm \
  --fresnel-threshold=1.1
```

Missing SRTM directory:

```bash
cargo run -- \
  --tx="0,0,10" \
  --rx="0,0.01,10" \
  --freq=2.4GHz \
  --srtm-dir=./missing-srtm
```

The expected result is a non-zero exit with an explicit SRTM-directory diagnostic.

## Complete CLI validation with local SRTM data

A successful run requires a local directory containing every SRTM3 `.hgt` tile touched by the sampled path. SRTM data is intentionally not stored in the repository.

Example:

```bash
cargo run -- \
  --tx="-20.2831,-47.7812,12" \
  --rx="-20.3541,-47.8523,15" \
  --freq=2.4GHz \
  --srtm-dir=./srtm \
  --samples=500 \
  --profile \
  --output-image=profile.png \
  --export-geojson=profile.geojson
```

The terminal profile, graphical profile, and GeoJSON export are all derived from the same `LinkAnalysis` result.

## Exit-code convention

CLI validation failures return process exit code `2` and write the diagnostic to stderr. Successful analysis returns `0`.

## Reproducibility notes

- Do not commit SRTM terrain files.
- Keep the exact CLI arguments used for field validation alongside any generated output.
- For numerical regressions, prefer deterministic synthetic fixtures or the checked-in Rust/Python differential vector.
- The Python reference implementation is a development oracle only; Rust remains the production authority.
