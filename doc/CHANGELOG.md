# Changelog

All notable project changes are recorded here.

## Unreleased

### Documentation

- Established the persistent developer/AI documentation structure.
- Added the AI-agent development and continuity contract.
- Added the initial technical specification.
- Added the architecture and module boundaries.
- Recorded initial engineering and RF-model decisions.
- Added project status, roadmap, developer workflow, and testing strategy.
- Defined Python as a development-only reference/laboratory implementation.
- Defined Rust/Python differential validation with quantity-specific numerical tolerances.
- Added Python validation workflow and responsibilities to the architecture, testing, developer guide, decisions, and roadmap.
- Updated README with the current CLI usage and implementation status.
- Documented the deterministic end-to-end Python reference vector and regeneration command.
- Closed Phase 5 and moved the roadmap focus to Phase 6 presentation/export.
- Documented `--profile`, image outputs, and GeoJSON export in the README.
- Documented the presentation/export font-independence constraint.
- Closed Phase 6 and moved the roadmap focus to Phase 7 hardening.
- Documented the first Phase 7 CLI-hardening increment and its CI validation.
- Recorded the second Phase 7 terrain-error coverage increment and CI validation.
- Recorded the numerical edge-case hardening increment and its CI validation.
- Added `doc/VALIDATION.md` with reproducible Rust quality gates and CLI validation examples.
- Documented the methodology for the synthetic SRTM access benchmark and recorded its first local baseline.

### Implementation

- Implemented Rust geometry, units, RF primitives, and SRTM3 terrain access.
- Added deterministic synthetic SRTM integration tests.
- Implemented effective-Earth curvature, LOS/reference path, terrain clearance, Fresnel classification, and `LinkAnalysis`.
- Added end-to-end clear/obstructed analysis tests.
- Wired the CLI into validated antenna/frequency input, `SrtmProvider`, and `analyze_link`.
- Added the first user-visible terminal link-analysis summary.
- Added an independent Python-generated 21-sample end-to-end reference profile.
- Isolated the Rust/Python differential regression in `tests/differential.rs` and compare all end-to-end summary and profile quantities against the Python vector.
- Added `render_terminal_profile` to render the complete sampled profile as an auditable terminal table.
- Added the opt-in `--profile` CLI flag for detailed terminal output.
- Corrected the differential regression import layout to match current rustfmt behavior.
- Corrected the terminal-profile test line-count expectation and removed unused test imports.
- Implemented PNG and SVG profile rendering from `LinkAnalysis` samples using Plotters.
- Added deterministic PNG/SVG renderer tests that verify non-empty output files.
- Implemented GeoJSON `FeatureCollection` export with a path `LineString` and one profile `Point` per sample.
- Added GeoJSON parsing/export coverage.
- Wired `--output-image` into the CLI with explicit `.png`/`.svg` extension validation.
- Wired `--export-geojson` into the CLI using the same `LinkAnalysis` instance as terminal and image outputs.
- Removed the graphical renderer's dependency on host-installed fonts so PNG/SVG generation remains portable across headless environments.
- Added early validation for CLI analysis controls (`--samples`, `--k-factor`, and `--fresnel-threshold`).
- Added early `--srtm-dir` directory validation.
- Added destination context to profile-image and GeoJSON write/export failures.
- Added dedicated CLI tests for malformed analysis-control inputs.
- Added end-to-end coverage for missing SRTM tiles.
- Added end-to-end coverage for SRTM `NoData` at an endpoint using the correct HGT south-edge sample index.
- Hardened Fresnel distance validation against `NaN`/infinite inputs and made the geometric calculation resistant to distance-sum overflow.
- Hardened effective-Earth radius and curvature validation against non-finite results.
- Rejected antipodal great-circle interpolation where a unique shortest path is undefined.
- Validated `analyze_link` coordinates before terrain access and frequency before wavelength derivation.
- Added deterministic unit and integration coverage for the numerical edge cases.
- Added a standalone `srtm_access` benchmark harness for warm single-tile and two-tile cached access.

### Validation

- CI run #52 closed SRTM validation with formatting, tests, and Clippy green.
- CI run #73 validated the integrated `LinkAnalysis` workflow with formatting, tests, and Clippy green.
- CI runs #75–#79 were diagnosed as repeated formatter-only failures in `src/main.rs`; the issue was corrected and CI #81 returned green.
- CI runs #90–#93 exposed formatter-only issues introduced while integrating the differential regression; those issues were corrected without removing the regression coverage.
- CI run #94 passed formatting, all tests, and Clippy, validating the complete current Rust/Python end-to-end differential regression.
- CI runs #100–#105 exposed repeated formatter-only failures during terminal-profile/differential-test integration; the root cause was the differential-test import layout under current rustfmt 1.98.1.
- CI run #106 reached `cargo test` after formatting was fixed and exposed a stale terminal-profile test line-count assertion; the test was corrected.
- CI run #107 validated the corrected terminal-profile path with formatting, tests, and Clippy green.
- CI runs #116 and #118 exposed presentation/export `cargo fmt --check` regressions; those formatting issues were corrected for rustfmt 1.98.1.
- Local validation on Rust 1.97.1 exposed a Plotters host-font failure in the PNG/SVG test; the renderer was changed to avoid text/font rendering entirely, preserving the profile curves while removing the environment dependency.
- CI run #124 on `ed2de2c42c50a1d3177e8ac0b9a826fed0fc67bc` passed `cargo fmt --check`, `cargo test`, and Clippy, completing validation of the presentation/export path.
- Local validation on `ed2de2c` independently passed `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings`.
- CI run #129 exposed only a rustfmt 1.98.1 layout mismatch in the new CLI validation function; no test or lint stage was reached.
- CI run #130 on `7037afcc127fe324ab41e73787407d34cfef855f` passed `cargo fmt --check`, `cargo test`, and Clippy after the formatter correction.
- CI run #134 exposed an incorrect synthetic HGT `NoData` byte offset in the first terrain-error test attempt; the fixture was corrected without production-code changes.
- CI run #135 still showed the fixture targeting the wrong edge of the HGT tile; it was corrected to use the actual tile selected by the endpoint.
- CI run #136 exposed the HGT north-to-south row-orientation detail; the fixture was changed to target row `SRTM3_SAMPLES - 1` and the endpoint column directly.
- CI run #137 on `b43bbd5ac5e43c550a85582ac01a6873f776dfd1` passed formatting, tests, and Clippy, completing missing-tile and `NoData` coverage.
- CI runs #148 and #149 exposed rustfmt layout regressions while adding numerical edge-case coverage; both stopped before tests and linting.
- CI run #155 on `066252fe6779bf0690493491a7fa5ecab9048d82` passed `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings`, validating the numerical edge-case hardening.
- CI run #163 passed formatting, all tests, and Clippy after the benchmark/validation documentation changes.
- Local validation on 2026-09-10 passed `cargo fmt --check`, `cargo test` (30 unit tests plus integration, geo, RF, SRTM, and differential suites), `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo bench --bench srtm_access`.
- Local SRTM benchmark baseline on 2026-09-10: 96.2 ns/query for 100,000 repeated queries in one cached tile and 101.9 ns/query for 100,000 queries alternating between two cached tiles. These are machine-local measurements, not universal performance guarantees.
