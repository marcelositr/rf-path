# RF-Path — Project Status

## Current phase

**Phase 5 — Link analysis**

## Current state

The Rust foundation, geometry, SRTM terrain layer, and RF propagation/clearance primitives are implemented and validated. The project now has the first integrated `LinkAnalysis` workflow: great-circle path sampling, endpoint terrain/absolute antenna altitude calculation, per-sample effective LOS/curvature/Fresnel/clearance values, obstruction classification, worst-point selection, and summary metrics.

Deterministic synthetic terrain is used by integration tests so the analysis workflow remains offline and independent of real SRTM datasets. Rendering/export layers are still separate and do not participate in RF calculations.

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
- [x] Correct formatting for the new RF analysis code and integration tests.
- [x] Validate the RF primitive implementation through green CI.
- [x] Create `ProfileSample`, `Obstacle`, and `LinkAnalysis` result models.
- [x] Integrate great-circle sampling with `TerrainProvider`.
- [x] Calculate endpoint ground elevations and absolute antenna altitudes.
- [x] Calculate auditable per-sample terrain, effective LOS, Earth bulge, Fresnel radius, clearance, ratio, and status.
- [x] Identify the minimum-clearance worst point and summary blocking flags.
- [x] Add deterministic end-to-end integration tests for clear and obstructed synthetic terrain.

## Current task

Validate the first complete `LinkAnalysis` workflow through green CI. After validation, connect the CLI to this analysis layer and implement the first user-visible summary output.

## Known constraints

- Production implementation is Rust.
- Python reference tooling is development-only and must not become a runtime dependency.
- SRTM data will be supplied locally and must not be committed to the repository.
- No real SRTM fixtures are present yet.
- CLI parsing is currently a structural foundation; full command execution remains pending.
- The effective-Earth model uses the agreed parabolic approximation; a more exact propagation model is future work.
- Rendering/export must continue consuming `LinkAnalysis` rather than recalculating domain math.

## Next recommended task

Verify the new end-to-end analysis tests through CI. Once green, wire CLI argument parsing into validated `GeoPoint`/`AntennaPoint` inputs, instantiate `SrtmProvider`, run `analyze_link`, and print a concise auditable summary before implementing PNG/SVG/GeoJSON output.

## Validation

CI run #52 validated the SRTM phase completely. The RF primitives subsequently passed validation after the formatting correction. The current end-to-end `LinkAnalysis` integration commit adds clear and obstructed synthetic-terrain cases; its CI result is pending and is the gate for closing the initial Phase 5 increment.

## Continuity note

At the end of every implementation session, update this file so another AI agent can determine exactly where work stopped without relying on chat history.
