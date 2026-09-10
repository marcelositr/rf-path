# RF-Path — Project Status

## Current phase

**Phase 6 — Presentation and export**

## Current state

The Rust foundation, geometry, SRTM terrain layer, RF propagation/clearance primitives, integrated `LinkAnalysis` workflow, CLI execution path, and representative Rust/Python end-to-end differential regression are implemented and validated. The CLI now also supports an opt-in detailed terminal profile table rendered directly from `LinkAnalysis`.

PNG/SVG rendering and GeoJSON export remain separate presentation/export work. Their existing output flags are recognized but currently rejected with an explicit message rather than being silently ignored.

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
- [x] Explicitly reject not-yet-wired image/GeoJSON output flags.
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
- [x] Correct `tests/differential.rs` to the formatter-required one-line import layout.
- [x] Diagnose CI #106 as the first non-formatting failure in the terminal-profile test: the expected line count was stale after the renderer added its table separator/header structure.
- [x] Correct the terminal-profile test expectation and remove now-unused test imports.

## Current task

Validate the corrected terminal-profile path through green CI. Then document `--profile` usage in the README and continue Phase 6 with PNG/SVG rendering and GeoJSON export, keeping all presentation layers downstream of `LinkAnalysis`.

## Known constraints

- Production implementation is Rust.
- Python reference tooling is development-only and must not become a runtime dependency.
- SRTM data will be supplied locally and must not be committed to the repository.
- No real SRTM fixtures are present yet.
- The CLI requires a local SRTM directory containing every tile needed by the sampled path.
- PNG/SVG and GeoJSON output flags are present but intentionally not implemented yet.
- The effective-Earth model uses the agreed parabolic approximation; a more exact propagation model is future work.
- Rendering/export must continue consuming `LinkAnalysis` rather than recalculating domain math.

## Next recommended task

Wait for the current CI validation of the corrected terminal-profile test to complete. Once green, document the `--profile` usage and begin PNG/SVG rendering from the same `LinkAnalysis` sample data.

## Validation

CI #94 validated the Rust/Python end-to-end differential regression with formatting, tests, and Clippy green. CI #100–#105 exposed repeated formatter-only failures during terminal-profile/differential-test integration; the differential import is now in the exact layout expected by rustfmt 1.98.1. CI #106 reached `cargo test` and exposed a stale renderer test line-count assertion; that test is now corrected. The current CI run #107 validates the corrected state.

## Continuity note

At the end of every implementation session, update this file so another AI agent can determine exactly where work stopped without relying on chat history.
