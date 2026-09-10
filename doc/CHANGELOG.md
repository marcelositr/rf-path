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
- Local validation on `ed2de2c` independently passed `cargo fmt --check`, `cargo test` (21 unit tests plus 10 integration/differential tests), and `cargo clippy --all-targets --all-features -- -D warnings`.
- CI run #129 exposed only a rustfmt 1.98.1 layout mismatch in the new CLI validation function; no test or lint stage was reached.
- CI run #130 on `7037afcc127fe324ab41e73787407d34cfef855f` passed `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings` after the formatter correction.
- CI run #134 exposed an incorrect synthetic HGT `NoData` byte offset in the first end-to-end terrain-error test attempt; the failure led to a fixture correction rather than a production-code change.
- CI run #135 still showed the fixture targeting the wrong edge of the HGT tile; the test was corrected to use the actual tile selected by the endpoint.
- CI run #136 still exposed the HGT north-to-south row-orientation detail because latitude `1.0` is the south edge of the `south_lat=1` tile; the fixture was changed to target row `SRTM3_SAMPLES - 1` and the endpoint column directly.
- CI run #137 on `b43bbd5ac5e43c550a85582ac01a6873f776dfd1` passed `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings`, completing validation of missing-tile and `NoData` integration coverage.
