# RF-Path — Project Status

## Current phase

**Phase 0 — Documentation and technical foundation**

## Current state

The repository began as a minimal project placeholder. The documentation foundation is now established and explicitly includes a Python reference/validation layer for development.

The production application will be Rust. Python will be used as a development-only laboratory and independent reference implementation to validate numerical/geospatial behavior and detect discrepancies.

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
- [ ] Create `Cargo.toml` and initial Rust crate.
- [ ] Create `tools/reference/` Python tooling skeleton.
- [ ] Implement CLI parsing.
- [ ] Implement geographic primitives and great-circle sampling.
- [ ] Implement SRTM HGT reader.
- [ ] Implement RF calculations.
- [ ] Implement link analysis orchestration.
- [ ] Implement terminal output.
- [ ] Implement PNG/SVG rendering.
- [ ] Implement GeoJSON export.
- [ ] Add complete test suite.
- [ ] Add automated Rust/Python differential test harness.

## Current task

Finish the documentation foundation and then create the initial Rust project together with the development-only Python reference skeleton.

The first numerical implementation should start with units and geographic primitives. Python should be used to validate the mathematical behavior before important reference cases are encoded as permanent Rust tests.

## Known constraints

- No production source code exists yet.
- No Cargo manifest exists yet.
- No Python reference tooling exists yet.
- No test fixtures exist yet.
- SRTM data will be supplied by the user locally; it should not be committed to the repository.
- Python is not a production/runtime dependency.

## Next recommended task

Create the initial Cargo project and `tools/reference/` skeleton. Implement units and geographic primitives with deterministic Rust tests, plus corresponding simple Python reference calculations for cross-checking.

## Validation

Documentation-only changes have been committed. Rust build/test validation will begin once `Cargo.toml` and source code exist. Python differential validation will begin when the reference tooling and first Rust numerical modules exist.

## Continuity note

At the end of every implementation session, update this file so another AI agent can determine exactly where work stopped without relying on chat history.
