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

Phase 3 was closed by CI run #52, which passed formatting, all tests, and Clippy.

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

Phase 4 was validated by green CI after the formatting correction. The model uses configurable `k` with default `4/3`, a parabolic curvature approximation, linear endpoint reference altitude, and the 60% first-Fresnel classification.

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
- [x] Diagnose and fix the formatter-only CI regression from #75–#79.
- [x] Add independent Python-generated end-to-end reference profile.
- [x] Add Rust differential regression test for the complete 21-sample profile.
- [x] Validate the Rust/Python differential regression through green CI (#94).

Phase 5 is closed. The production analysis workflow, CLI wiring, and representative end-to-end Rust/Python differential validation are all implemented and validated.

## Phase 6 — Presentation and export

- [x] Implement terminal profile/details beyond the summary.
- [x] Diagnose and correct CI #102–#105 formatter-only regressions in the differential-test import layout.
- [x] Diagnose and correct CI #106 terminal-profile test failure caused by a stale line-count assertion.
- [x] Validate the corrected terminal profile path through green CI (#107).
- [x] Document `--profile` usage in the README.
- [x] Implement PNG profile rendering.
- [x] Implement SVG profile rendering.
- [x] Implement GeoJSON export.
- [x] Ensure terminal, image, and GeoJSON outputs consume `LinkAnalysis` data only.
- [x] Diagnose and correct presentation/export `cargo fmt` regressions in CI #116/#118.
- [x] Make PNG/SVG rendering independent of installed system fonts.
- [x] Validate the complete presentation/export path through green CI on `ed2de2c`.

Phase 6 is closed. Presentation and export are implemented, consume the established `LinkAnalysis` result, and pass local and GitHub Actions formatting, test, and Clippy validation.

## Phase 7 — Hardening

- [x] Improve CLI diagnostics.
- [x] Add malformed-input tests.
- [x] Add missing-tile and NoData scenarios.
- [ ] Benchmark terrain access.
- [x] Review numerical edge cases.
- [ ] Automate selected Rust/Python differential cases.
- [x] Document reproducible validation examples.

The first Phase 7 increment validates analysis controls before terrain access, checks the SRTM directory before analysis, adds destination context to output/export failures, and covers malformed analysis-control inputs. CI run #130 passed formatting, tests, and Clippy after the rustfmt-only correction from #129.

The second Phase 7 increment adds end-to-end coverage for missing SRTM tiles and endpoint `NoData`. The tests verify that terrain access failures remain explicit and that the analysis does not invent replacement elevation data. CI run #137 passed formatting, tests, and Clippy after correcting the synthetic HGT fixture to target the actual south-edge sample cell.

The numerical edge-case increment hardens non-finite RF distances, overflow-safe Fresnel arithmetic, effective Earth-radius/bulge validation, antipodal great-circle interpolation, direct-coordinate validation in `analyze_link`, and early frequency validation. Deterministic unit/integration tests cover these cases, and CI run #155 passed formatting, all tests, and Clippy on the resulting head.

The reproducible-validation increment adds `doc/VALIDATION.md` with automated Rust quality gates, deterministic CLI failure cases, a complete local-SRTM CLI example, and the project exit-code convention.

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
