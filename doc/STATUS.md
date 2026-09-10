# RF-Path — Project Status

## Current phase

**Phase 1 — Rust and validation foundation**

## Current state

The repository now has the planned initial project tree: Rust crate structure, integration-test structure, development-only Python reference tooling, test-fixture documentation, and baseline GitHub CI.

The production application remains Rust. Python is a development-only laboratory/reference implementation used to independently validate numerical and geospatial behavior.

## Completed

- [x] Define repository documentation as persistent project context.
- [x] Define AI-agent onboarding and continuity rules.
- [x] Define the initial technical specification.
- [x] Define module boundaries and data flow.
- [x] Record the initial engineering/math decisions.
- [x] Define Rust unit/integration testing strategy.
- [x] Decide to use Python as a development-only reference implementation.
- [x] Define Rust/Python differential validation with explicit tolerances.
- [x] Document Python's role and boundaries in architecture, decisions, testing, and developer workflow.
- [x] Create `Cargo.toml`.
- [x] Create the initial Rust source/module tree.
- [x] Create the Rust integration-test tree.
- [x] Create `tools/reference/` Python tooling skeleton.
- [x] Add deterministic test-fixture documentation.
- [x] Add baseline GitHub Actions Rust CI.
- [ ] Implement CLI parsing.
- [ ] Implement units.
- [ ] Implement geographic primitives and great-circle sampling.
- [ ] Implement SRTM HGT reader.
- [ ] Implement RF calculations.
- [ ] Implement link analysis orchestration.
- [ ] Implement terminal output.
- [ ] Implement PNG/SVG rendering.
- [ ] Implement GeoJSON export.
- [ ] Add complete test suite.
- [ ] Implement automated Rust/Python differential test harness.

## Current task

Start the first real implementation increment: units and geographic primitives, with deterministic Rust tests and matching simple Python reference calculations.

## Known constraints

- Production implementation is Rust.
- Python reference tooling is development-only and must not become a runtime dependency.
- SRTM data will be supplied locally and must not be committed to the repository.
- No real SRTM fixtures are present yet.
- The initial source modules are scaffolds; domain behavior has not been implemented yet.

## Next recommended task

Implement `src/units.rs` and `src/geo.rs`. Add focused Rust tests, then implement the corresponding Python reference calculations and compare representative values.

## Validation

The repository now contains a Cargo manifest and CI configuration. Full Rust validation should be run locally/through CI after the scaffold is synchronized. Numerical differential validation begins with the first implemented domain functions.

## Continuity note

At the end of every implementation session, update this file so another AI agent can determine exactly where work stopped without relying on chat history.
