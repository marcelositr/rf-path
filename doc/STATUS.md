# RF-Path — Project Status

## Current phase

**Phase 0 — Documentation and technical foundation**

## Current state

The repository started as a minimal Rust-project placeholder containing the project name/description, license, and gitignore. The documentation foundation is now being established before implementation.

## Completed

- [x] Define repository documentation as persistent project context.
- [x] Define AI-agent onboarding and continuity rules.
- [x] Define the initial technical specification.
- [x] Define module boundaries and data flow.
- [x] Record the initial engineering/math decisions.
- [ ] Create `Cargo.toml` and initial Rust crate.
- [ ] Implement CLI parsing.
- [ ] Implement geographic primitives and great-circle sampling.
- [ ] Implement SRTM HGT reader.
- [ ] Implement RF calculations.
- [ ] Implement link analysis orchestration.
- [ ] Implement terminal output.
- [ ] Implement PNG/SVG rendering.
- [ ] Implement GeoJSON export.
- [ ] Add complete test suite.

## Current task

Finish the documentation foundation, then create the minimal Rust project structure and begin implementation from the documented architecture.

## Known constraints

- No production source code exists yet.
- No Cargo manifest exists yet.
- No test fixtures exist yet.
- SRTM data will be supplied by the user locally; it should not be committed to the repository.

## Next recommended task

Create the initial Cargo project and module skeleton, then implement the units and geographic primitives with tests before moving to SRTM I/O.

## Validation

Documentation-only changes have been committed. Rust build/test validation will begin once `Cargo.toml` and source code exist.

## Continuity note

At the end of every implementation session, update this file so another AI agent can determine exactly where work stopped without relying on chat history.
