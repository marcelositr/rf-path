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
- Explicitly reject PNG/GeoJSON output flags until presentation/export implementations are connected.
- Corrected the differential regression import layout to match current rustfmt behavior.
- Corrected the terminal-profile test line-count expectation and removed unused test imports.

### Validation

- CI run #52 closed SRTM validation with formatting, tests, and Clippy green.
- CI run #73 validated the integrated `LinkAnalysis` workflow with formatting, tests, and Clippy green.
- CI runs #75–#79 were diagnosed as repeated formatter-only failures in `src/main.rs`; the issue was corrected and CI #81 returned green.
- CI runs #90–#93 exposed formatter-only issues introduced while integrating the differential regression; those issues were corrected without removing the regression coverage.
- CI run #94 passed formatting, all tests, and Clippy, validating the complete current Rust/Python end-to-end differential regression.
- CI runs #100–#105 exposed repeated formatter-only failures during terminal-profile/differential-test integration; the root cause was the differential-test import layout under current rustfmt 1.98.1.
- CI run #106 reached `cargo test` after formatting was fixed and exposed a stale terminal-profile test line-count assertion; the test was corrected.
- CI run #107 validates the corrected terminal-profile test path; its final result is still pending at the time of this documentation update.
