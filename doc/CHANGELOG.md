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

### Implementation

- Implemented Rust geometry, units, RF primitives, and SRTM3 terrain access.
- Added deterministic synthetic SRTM integration tests.
- Implemented effective-Earth curvature, LOS/reference path, terrain clearance, Fresnel classification, and `LinkAnalysis`.
- Added end-to-end clear/obstructed analysis tests.
- Wired the CLI into validated antenna/frequency input, `SrtmProvider`, and `analyze_link`.
- Added the first user-visible terminal link-analysis summary.
- Added an independent Python-generated 21-sample end-to-end reference profile.
- Added a Rust integration test comparing all end-to-end summary and profile quantities against the Python vector.
- Explicitly reject PNG/GeoJSON output flags until presentation/export implementations are connected.

### Validation

- CI run #52 closed SRTM validation with formatting, tests, and Clippy green.
- CI run #73 validated the integrated `LinkAnalysis` workflow with formatting, tests, and Clippy green.
- CI runs #75–#79 were diagnosed as repeated formatter-only failures in `src/main.rs`; the issue was corrected and CI #81 returned green.
