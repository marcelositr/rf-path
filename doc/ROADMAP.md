# RF-Path — Roadmap

The roadmap is intentionally incremental. Each phase should leave the repository buildable and testable. `STATUS.md` records the current implementation state; this file records the planned progression and remaining work.

## Phase 0 — Documentation foundation

- [x] Define README vs `doc/` responsibilities.
- [x] Define AI-agent continuity contract.
- [x] Define technical specification.
- [x] Define architecture.
- [x] Record initial decisions.
- [x] Establish project status tracking.
- [x] Define Python reference/validation role.
- [x] Define differential-testing strategy.

## Phase 1 — Rust and validation foundation

- [x] Create `Cargo.toml`.
- [x] Establish Rust 2021 crate.
- [x] Add error handling conventions.
- [x] Add module skeleton.
- [x] Establish CI-quality formatting/lint/test baseline.
- [x] Create `tools/reference/` Python skeleton.
- [x] Add instructions for running reference calculations.

## Phase 2 — Geometry and units

- [x] Parse geographic endpoints.
- [x] Parse frequency units.
- [x] Implement SI unit conversions.
- [x] Implement great-circle distance.
- [x] Implement robust great-circle sampling.
- [x] Add deterministic Rust geometry tests.
- [x] Add corresponding Python reference calculations.
- [x] Compare representative Rust/Python results with documented tolerances.

## Phase 3 — SRTM terrain

- [x] Implement HGT tile naming.
- [x] Implement mmap-backed tile access.
- [x] Decode big-endian samples.
- [x] Correct north-to-south row orientation.
- [x] Implement bilinear interpolation.
- [x] Handle `NoData` explicitly.
- [x] Support multiple tiles along a path.
- [x] Add synthetic HGT fixtures/tests.
- [x] Add Python reference checks for tile/index/interpolation behavior.
- [x] Add explicit Rust/Python differential vectors for controlled SRTM cases.
- [x] Strengthen boundary and malformed-fixture coverage.
- [x] Validate the complete SRTM test suite through green CI.

Phase 3 was closed by CI run #52.

## Phase 4 — RF model

- [x] Implement wavelength.
- [x] Implement first Fresnel radius.
- [x] Implement effective Earth radius model.
- [x] Implement LOS/reference-path calculations.
- [x] Implement clearance and classification.
- [x] Implement FSPL.
- [x] Add known-value Rust tests for implemented RF primitives.
- [x] Add independent Python reference calculations for implemented RF primitives.
- [x] Differential-test the effective-Earth and clearance model with explicit tolerances.

Phase 4 was validated by green CI.

## Phase 5 — Link analysis

- [x] Define `ProfileSample`, `Obstacle`, and `LinkAnalysis` result models.
- [x] Combine great-circle geometry with `TerrainProvider` sampling.
- [x] Calculate endpoint ground elevation and absolute antenna altitude.
- [x] Produce auditable per-sample terrain/LOS/curvature/Fresnel/clearance data.
- [x] Identify the minimum-clearance worst point.
- [x] Produce summary blocking metrics.
- [x] Add end-to-end clear/obstructed synthetic-terrain tests.
- [x] Validate the complete `LinkAnalysis` workflow through green CI (#73).
- [x] Wire CLI arguments to `analyze_link`.
- [x] Add first user-visible terminal summary.
- [x] Add independent Python-generated end-to-end reference profile.
- [x] Add Rust differential regression for the complete 21-sample profile.
- [x] Validate the Rust/Python differential regression through green CI (#94).

Phase 5 is closed.

## Phase 6 — Presentation and export

- [x] Implement terminal profile/details.
- [x] Add `--profile` CLI flag.
- [x] Implement PNG profile rendering.
- [x] Implement SVG profile rendering.
- [x] Implement GeoJSON export.
- [x] Ensure terminal, image, and GeoJSON outputs consume `LinkAnalysis` data only.
- [x] Make PNG/SVG rendering independent of installed system fonts.
- [x] Validate the complete presentation/export path through green CI on `ed2de2c`.

Phase 6 is closed.

## Phase 7 — Hardening

- [x] Improve CLI diagnostics.
- [x] Add malformed-input tests.
- [x] Add missing-tile and NoData scenarios.
- [x] Review numerical edge cases.
- [x] Document reproducible validation examples.
- [x] Add reproducible SRTM access benchmark harness.
- [x] Record a local SRTM benchmark baseline.
- [ ] Automate selected Rust/Python differential cases.

The first Phase 7 increment validated analysis controls before terrain access, checked the SRTM directory early, added destination context to output/export failures, and covered malformed analysis-control inputs. CI #130 passed formatting, tests, and Clippy.

The second increment added end-to-end missing-tile and endpoint `NoData` coverage. CI #137 passed formatting, tests, and Clippy after correcting the synthetic HGT fixture to target the actual south-edge sample cell.

The numerical increment hardened non-finite Fresnel inputs, overflow-safe Fresnel arithmetic, effective Earth-radius/bulge validation, antipodal interpolation, direct-coordinate validation, and early frequency validation. CI #155 passed formatting, all tests, and Clippy.

The reproducible-validation increment added `doc/VALIDATION.md` with Rust quality gates, deterministic CLI failure cases, a complete local-SRTM CLI example, and the exit-code convention.

The benchmark increment added `benches/srtm_access.rs` and documented methodology. A local run on 2026-09-10 measured 96.2 ns/query for 100,000 repeated queries in one cached tile and 101.9 ns/query for 100,000 queries alternating between two cached tiles. These timings are a machine-local baseline, not a CI threshold.

## Future / explicitly out of v1

Potential later work may include:

- complete link-budget calculations;
- antenna gains and radiation patterns;
- diffraction models;
- clutter/vegetation/building data;
- additional DEM formats/resolutions;
- atmospheric and weather attenuation;
- richer GIS output;
- performance parallelization after profiling.

These features should not be treated as implicitly supported by v1.
