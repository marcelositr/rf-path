# RF-Path — Project Status

## Current phase

**Phase 7 — Hardening**

## Current state

The Rust foundation, geometry, SRTM terrain layer, RF propagation/clearance primitives, integrated `LinkAnalysis` workflow, CLI execution path, Rust/Python differential regression, and presentation/export layers are implemented and validated. The CLI supports the detailed terminal profile, PNG/SVG profile charts, and GeoJSON export, all downstream of the same `LinkAnalysis` result.

Phase 6 presentation/export work is closed. PNG/SVG rendering is deliberately font-independent so graphical generation does not require an installed system font, while the labelled terminal profile remains the human-readable presentation.

Phase 7 has progressed through CLI hardening, terrain failure coverage, numerical edge-case hardening, and reproducible validation examples. Analysis controls are validated before terrain access, SRTM directory paths are checked early, output/export failures include their destination path, malformed CLI controls have dedicated tests, end-to-end missing-tile/NoData scenarios verify explicit terrain failures, and the geometry/RF/link-analysis layers reject ambiguous or non-finite inputs without inventing values.

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
- [x] Diagnose CI #116/#118 as `cargo fmt --check` regressions in the newly added presentation/export files.
- [x] Correct presentation/export formatting for the current rustfmt 1.98.1.
- [x] Diagnose the local PNG/SVG test failure caused by unavailable host fonts in Plotters.
- [x] Make PNG/SVG rendering independent of installed system fonts while preserving the same analysis curves.
- [x] Validate the complete presentation/export path through green CI on `ed2de2c`.
- [x] Improve CLI diagnostics for invalid analysis controls, SRTM directory validation, and output/export failures.
- [x] Add dedicated malformed-input tests for analysis controls while preserving existing antenna/frequency validation coverage.
- [x] Validate the CLI-hardening increment through green CI (#130).
- [x] Add end-to-end coverage for missing SRTM tiles.
- [x] Add end-to-end coverage for SRTM `NoData` at an endpoint.
- [x] Preserve explicit `NoData` failure rather than inventing terrain values.
- [x] Validate the missing-tile/NoData increment through green CI (#137).
- [x] Reject non-finite Fresnel distances and make Fresnel geometric arithmetic overflow-safe.
- [x] Reject non-finite effective Earth radius and curvature bulge results.
- [x] Reject antipodal great-circle interpolation where the shortest path is not unique.
- [x] Validate `analyze_link` coordinates before terrain access and validate frequency before deriving wavelength.
- [x] Add deterministic integration/unit coverage for these numerical edge cases.
- [x] Add reproducible CLI validation examples and document the quality-gate commands in `doc/VALIDATION.md`.

## Current task

Benchmark SRTM terrain access and record a reproducible performance baseline. After profiling, automate selected Rust/Python differential cases if they provide useful regression value.

## Known constraints

- Production implementation is Rust.
- Python reference tooling is development-only and must not become a runtime dependency.
- SRTM data will be supplied locally and must not be committed to the repository.
- No real SRTM fixtures are present yet.
- The CLI requires a local SRTM directory containing every tile needed by the sampled path.
- The effective-Earth model uses the agreed parabolic approximation; a more exact propagation model is future work.
- Rendering/export consumes `LinkAnalysis` and does not recalculate domain math.
- `--output-image` currently accepts `.png` and `.svg`; other extensions are rejected explicitly.
- PNG/SVG rendering intentionally does not require a system-installed font; the labelled terminal profile remains the human-readable presentation.
- CLI failures return process exit code `2` and write the diagnostic to stderr.

## Next recommended task

Benchmark SRTM terrain access across representative repeated and cross-tile sampling workloads, using deterministic synthetic HGT tiles and recording methodology plus results.

## Validation

CI run #124 validated the complete presentation/export path with formatting, all tests, and Clippy green. CI run #130 validated the first Phase 7 CLI-hardening increment. CI run #137 on `b43bbd5` passed the missing-tile and `NoData` integration scenarios. CI run #155 on `066252fe6779bf0690493491a7fa5ecab9048d82` passed `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings`, validating the numerical edge-case hardening.

The reproducible CLI validation examples are documented in `doc/VALIDATION.md` and do not require committed SRTM data for the negative-path checks.

## Continuity note

At the end of every implementation session, update this file so another AI agent can determine exactly where work stopped without relying on chat history.
