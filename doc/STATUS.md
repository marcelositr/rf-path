# RF-Path — Project Status

## Current phase

**Phase 6 — Presentation and export**

## Current state

The Rust foundation, geometry, SRTM terrain layer, RF propagation/clearance primitives, integrated `LinkAnalysis` workflow, CLI execution path, Rust/Python differential regression, and presentation/export layers are implemented. The CLI supports the detailed terminal profile, PNG/SVG profile charts, and GeoJSON export, all downstream of the same `LinkAnalysis` result.

The remaining Phase 6 work is CI validation of the complete presentation/export path. After that, development moves to Phase 7 hardening.

## Completed

- [x] Establish persistent repository documentation and AI-agent continuity rules.
- [x] Define specification, architecture, decisions, roadmap, testing, and developer workflow.
- [x] Create `Cargo.toml` and Rust 2021 crate foundation.
- [x] Create Rust source/module tree.
- [x] Create Rust integration-test tree and fixture documentation.
- [x] Add baseline GitHub Actions Rust CI.
- [x] Define CLI structure and defaults.
- [x] Implement frequency parsing into Hz.
- [x] Implement geographic point validation.
- [x] Implement great-circle distance, interpolation, and sampling.
- [x] Implement wavelength, first Fresnel radius, and FSPL.
- [x] Add deterministic Rust tests for units, geometry, and RF formulas.
- [x] Implement independent Python reference calculations for geometry and RF.
- [x] Implement Python SRTM tile naming, decoding, and bilinear-reference calculations.
- [x] Implement SRTM3 HGT tile naming with floor semantics.
- [x] Implement mmap-backed HGT access.
- [x] Implement signed 16-bit big-endian sample decoding.
- [x] Implement HGT north-to-south row orientation.
- [x] Implement bilinear interpolation.
- [x] Implement explicit NoData handling.
- [x] Implement multi-tile provider lookup and tile caching.
- [x] Add deterministic synthetic SRTM integration tests.
- [x] Add exact integer tile-boundary selection tests.
- [x] Add exact tile-corner interpolation coverage across adjacent tiles.
- [x] Add malformed HGT file-size rejection coverage.
- [x] Add controlled Rust/Python SRTM interpolation reference vectors.
- [x] Validate the complete SRTM test suite through green CI.
- [x] Implement configurable effective Earth radius with default `k=4/3`.
- [x] Implement parabolic effective-Earth bulge calculation.
- [x] Implement linear LOS/reference-path elevation.
- [x] Implement effective reference-path elevation with curvature.
- [x] Implement terrain clearance.
- [x] Implement Clear/FresnelPartial/LineOfSightBlocked classification.
- [x] Add Rust/Python reference vectors for effective Earth and clearance calculations.
- [x] Record the effective-Earth curvature convention in `DECISIONS.md`.
- [x] Validate the RF primitive implementation through green CI.
- [x] Create `ProfileSample`, `Obstacle`, and `LinkAnalysis` result models.
- [x] Integrate great-circle sampling with `TerrainProvider`.
- [x] Calculate endpoint ground elevations and absolute antenna altitudes.
- [x] Calculate auditable per-sample terrain, effective LOS, Earth bulge, Fresnel radius, clearance, ratio, and status.
- [x] Identify the minimum-clearance worst point and summary blocking flags.
- [x] Add deterministic end-to-end integration tests for clear and obstructed synthetic terrain.
- [x] Validate the complete `LinkAnalysis` workflow through green CI (#73).
- [x] Parse CLI `lat,lon,height_m` antenna arguments with validation tests.
- [x] Wire CLI execution into `analyze_link` and `SrtmProvider`.
- [x] Print first user-visible terminal analysis summary.
- [x] Update README with the current CLI usage and capability status.
- [x] Diagnose CI #75–#79 as the same `cargo fmt --check` failure in `src/main.rs`.
- [x] Correct the formatter-only failure in `src/main.rs`.
- [x] Add an independent Python end-to-end `LinkAnalysis` reference generator.
- [x] Add the generated 21-sample Rust/Python differential regression vector.
- [x] Isolate the Rust/Python differential regression in `tests/differential.rs`.
- [x] Diagnose CI #90–#93 as formatter-only failures during differential-test integration.
- [x] Correct the differential-test formatting without removing its coverage.
- [x] Validate the corrected differential regression through green CI (#94).
- [x] Implement `render_terminal_profile` from `LinkAnalysis`.
- [x] Add the `--profile` CLI flag.
- [x] Add deterministic terminal-profile rendering tests and explicit status labels.
- [x] Diagnose CI #102–#105 as repeated formatter-only failures caused by the differential-test import layout under current rustfmt 1.98.1.
- [x] Correct `tests/differential.rs` to the formatter-required import layout.
- [x] Diagnose CI #106 as the first non-formatting failure in the terminal-profile test: the expected line count was stale after the renderer added its table separator/header structure.
- [x] Correct the terminal-profile test expectation and remove now-unused test imports.
- [x] Validate the corrected terminal-profile path through green CI (#107).
- [x] Document `--profile` usage in the README.
- [x] Implement PNG profile rendering from `LinkAnalysis` samples.
- [x] Implement SVG profile rendering from `LinkAnalysis` samples.
- [x] Add PNG/SVG rendering tests that verify non-empty output files.
- [x] Implement GeoJSON `FeatureCollection` export with path and per-sample features.
- [x] Add GeoJSON parsing/export coverage.
- [x] Wire `--output-image` with `.png`/`.svg` extension validation.
- [x] Wire `--export-geojson` into the same analysed result.
- [x] Keep terminal, image, and GeoJSON presentation layers downstream of `LinkAnalysis`.

## Current task

Run and validate the complete presentation/export path through CI. The next Phase 7 increment should focus on CLI diagnostics, malformed inputs, missing tiles/NoData scenarios, numerical edge cases, and reproducible validation examples.

## Known constraints

- Production implementation is Rust.
- Python reference tooling is development-only and must not become a runtime dependency.
- SRTM data will be supplied locally and must not be committed to the repository.
- No real SRTM fixtures are present yet.
- The CLI requires a local SRTM directory containing every tile needed by the sampled path.
- The effective-Earth model uses the agreed parabolic approximation; a more exact propagation model is future work.
- Rendering/export consumes `LinkAnalysis` and does not recalculate domain math.
- `--output-image` currently accepts `.png` and `.svg`; other extensions are rejected explicitly.

## Next recommended task

Validate the new image/GeoJSON code locally and through GitHub Actions. If green, begin Phase 7 with CLI diagnostics and malformed-input coverage.

## Validation

The repository's last explicitly verified green CI state before the presentation/export implementation was CI #107, which passed formatting, tests, and Clippy after the terminal-profile test correction. The current image/export changes need a new CI validation cycle.

## Continuity note

At the end of every implementation session, update this file so another AI agent can determine exactly where work stopped without relying on chat history.
