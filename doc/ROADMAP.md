# RF-Path — Roadmap

The roadmap is intentionally incremental. Each phase should leave the repository buildable and testable.

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

- [ ] Create `Cargo.toml`.
- [ ] Establish Rust 2021 crate.
- [ ] Add error handling conventions.
- [ ] Add module skeleton.
- [ ] Establish CI-quality formatting/lint/test baseline.
- [ ] Create `tools/reference/` Python skeleton.
- [ ] Add instructions for running reference calculations.

## Phase 2 — Geometry and units

- [ ] Parse geographic endpoints.
- [ ] Parse frequency units.
- [ ] Implement SI unit conversions.
- [ ] Implement great-circle distance.
- [ ] Implement robust great-circle sampling.
- [ ] Add deterministic Rust geometry tests.
- [ ] Add corresponding Python reference calculations.
- [ ] Compare representative Rust/Python results with documented tolerances.

## Phase 3 — SRTM terrain

- [ ] Implement HGT tile naming.
- [ ] Implement mmap-backed tile access.
- [ ] Decode big-endian samples.
- [ ] Correct north-to-south row orientation.
- [ ] Implement bilinear interpolation.
- [ ] Handle `NoData` explicitly.
- [ ] Support multiple tiles along a path.
- [ ] Add synthetic HGT fixtures/tests.
- [ ] Add Python reference checks for tile/index/interpolation behavior.
- [ ] Differential-test controlled SRTM cases.

## Phase 4 — RF model

- [ ] Implement wavelength.
- [ ] Implement first Fresnel radius.
- [ ] Implement effective Earth radius model.
- [ ] Implement LOS/reference-path calculations.
- [ ] Implement clearance and classification.
- [ ] Implement FSPL.
- [ ] Add known-value Rust tests.
- [ ] Add independent Python reference calculations.
- [ ] Differential-test numerical vectors with explicit tolerances.

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
