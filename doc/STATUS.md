# RF-Path — Project Status

## Current phase

**Phase 2 — Geometry and units**

## Current state

The Rust crate foundation is in place and the first production numerical primitives have been implemented. Frequency parsing, SI constants, spherical great-circle distance, robust great-circle interpolation/sampling, wavelength, first Fresnel radius, and FSPL are now represented in Rust with deterministic unit tests.

The development-only Python reference layer contains corresponding readable geometry and RF calculations. It remains independent from the Rust runtime.

## Completed

- [x] Establish persistent repository documentation and AI-agent continuity rules.
- [x] Define specification, architecture, decisions, roadmap, testing, and developer workflow.
- [x] Create `Cargo.toml` and Rust 2021 crate foundation.
- [x] Create Rust source/module tree.
- [x] Create Rust integration-test tree and fixture documentation.
- [x] Create `tools/reference/` Python tooling skeleton.
- [x] Add baseline GitHub Actions Rust CI.
- [x] Define CLI structure and defaults.
- [x] Implement frequency parsing into Hz.
- [x] Implement SI speed-of-light constant.
- [x] Implement geographic point validation.
- [x] Implement great-circle distance.
- [x] Implement robust great-circle interpolation and sampling.
- [x] Add deterministic Rust tests for units and geometry.
- [x] Implement Python reference geometry calculations.
- [x] Implement Python reference RF calculations.
- [x] Implement Python SRTM tile-name/index skeleton.
- [x] Implement Python differential-comparison helper.
- [x] Implement Rust wavelength, first Fresnel radius, and FSPL.
- [x] Add deterministic Rust tests for the initial RF formulas.

## Current task

Continue Phase 2 by strengthening the geometry/units test vectors and performing explicit Rust/Python differential checks before moving to SRTM.

## Known constraints

- Production implementation is Rust.
- Python reference tooling is development-only and must not become a runtime dependency.
- SRTM data will be supplied locally and must not be committed to the repository.
- No real SRTM fixtures are present yet.
- CLI parsing is currently a structural foundation; full command execution remains pending.

## Next recommended task

Add shared known-value/reference vectors for geometry and units, run the Rust and Python calculations against them, document quantity-specific tolerances, then begin the SRTM HGT implementation according to `doc/SPECIFICATION.md`.

## Validation

Rust unit tests now exist for the first numerical modules. CI is configured for the repository baseline. Full validation should include `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings` as implementation progresses. Python reference calculations are available for independent numerical cross-checking.

## Continuity note

At the end of every implementation session, update this file so another AI agent can determine exactly where work stopped without relying on chat history.
