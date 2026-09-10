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
- [ ] Validate the complete SRTM test suite through green CI.

## Phase 4 — RF model

- [x] Implement wavelength.
- [x] Implement first Fresnel radius.
- [ ] Implement effective Earth radius model.
- [ ] Implement LOS/reference-path calculations.
- [ ] Implement clearance and classification.
- [x] Implement FSPL.
- [x] Add known-value Rust tests for implemented RF primitives.
- [x] Add independent Python reference calculations for implemented RF primitives.
- [ ] Differential-test the complete RF model with explicit tolerances.

## Phase 5 — Link analysis

- [ ] Combine geometry, terrain, and RF models.
- [ ] Produce auditable profile samples.
- [ ] Identify worst obstruction.
- [ ] Produce summary metrics.
- [ ] Add end-to-end tests with synthetic terrain.
- [ ] Cross-check representative end-to-end profiles against Python.

## Phase 6 — Presentation and export

- [ ] Implement terminal summary/profile.
- [ ] Implement PNG profile.
- [ ] Implement SVG profile.
- [ ] Implement GeoJSON export.
- [ ] Ensure outputs consume analysis data only.

## Phase 7 — Hardening

- [ ] Improve CLI diagnostics.
- [ ] Add malformed-input tests.
- [ ] Add missing-tile and NoData scenarios.
- [ ] Benchmark terrain access.
- [ ] Review numerical edge cases.
- [ ] Automate selected Rust/Python differential cases.
- [ ] Document reproducible validation examples.

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
