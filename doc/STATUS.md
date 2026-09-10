# RF-Path — Project Status

## Current phase

**Phase 4 — RF model**

## Current state

The Rust foundation, geometry, SRTM terrain layer, and core RF propagation/clearance primitives are implemented. The effective-Earth model uses configurable `k`, defaulting to `4/3`, with a single parabolic curvature convention. LOS/reference-path elevation, terrain clearance, and the 60% first-Fresnel classification are now implemented and covered by deterministic Rust tests plus independent Python reference formulas.

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

## Current task

Validate the new RF primitives through green CI, then move into Phase 5: combine geometry, SRTM terrain, and RF calculations into the `LinkAnalysis` result model and auditable sampled profiles.

## Known constraints

- Production implementation is Rust.
- Python reference tooling is development-only and must not become a runtime dependency.
- SRTM data will be supplied locally and must not be committed to the repository.
- No real SRTM fixtures are present yet.
- CLI parsing is currently a structural foundation; full command execution remains pending.
- The effective-Earth model currently uses the agreed parabolic approximation; a more exact propagation model is future work.

## Next recommended task

Build the `LinkAnalysis` layer around great-circle sampling and `TerrainProvider`: obtain endpoint ground elevations, convert antenna heights to absolute altitudes, compute per-sample distance/terrain/LOS/curvature/Fresnel/clearance values, identify the worst obstruction, and expose auditable summary metrics. Keep rendering/export separate from these calculations.

## Validation

CI run #52 for the SRTM validation commit passed the complete required suite. The effective-Earth and clearance implementation is in `src/analysis.rs`, with Python reference formulas in `tools/reference/rf_reference.py` and integration vectors in `tests/rf.rs`. The latest CI run for this RF increment is pending; green CI is required before closing Phase 4.

## Continuity note

At the end of every implementation session, update this file so another AI agent can determine exactly where work stopped without relying on chat history.
