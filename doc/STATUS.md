# RF-Path — Project Status

## Current phase

**Phase 5 — Link analysis**

## Current state

The Rust foundation, geometry, SRTM terrain layer, RF propagation/clearance primitives, integrated `LinkAnalysis` workflow, and CLI execution path are implemented. The project now also contains an independent Python-generated end-to-end reference vector covering a deterministic 21-sample flat-terrain profile; the Rust integration suite compares the complete summary and every profile sample against that vector with quantity-specific tolerances.

PNG/SVG rendering and GeoJSON export remain separate presentation/export work. Their CLI flags are recognized but currently rejected with an explicit message rather than being silently ignored.

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
- [x] Add Rust integration assertions for all reference summary and profile quantities.
- [x] Diagnose CI #90 as a formatter-only failure in `tests/integration.rs`.
- [x] Correct the formatter-only failure in the differential integration test.

## Current task

Validate the corrected Rust/Python end-to-end differential test through green CI. Then close Phase 5 and move to Phase 6 presentation/export, starting with a terminal profile/details view before image and GeoJSON outputs.

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

Verify the corrected differential test through the current CI run. Once green, mark the representative Rust/Python end-to-end comparison complete, close Phase 5, and begin the terminal profile/details renderer in `src/render.rs`.

## Validation

CI #81 validated the CLI wiring with formatting, tests, and Clippy green. CI #90 failed only at `cargo fmt --check` on the new differential test; the formatter corrections are now committed and the following CI run is pending.

## Continuity note

At the end of every implementation session, update this file so another AI agent can determine exactly where work stopped without relying on chat history.
