# RF-Path — Project Status

## Current phase

**Phase 7 — Hardening**

## Current state

The Rust foundation, geometry, SRTM terrain layer, RF propagation/clearance primitives, integrated `LinkAnalysis` workflow, CLI execution path, Rust/Python differential regression, and presentation/export layers are implemented and validated. The CLI supports the detailed terminal profile, PNG/SVG profile charts, and GeoJSON export, all downstream of the same `LinkAnalysis` result.

Phase 6 presentation/export work is closed. PNG/SVG rendering is deliberately font-independent so graphical generation does not require an installed system font, while the labelled terminal profile remains the human-readable presentation.

Phase 7 has progressed through CLI hardening, terrain failure coverage, numerical edge-case hardening, reproducible validation examples, and a reproducible SRTM access benchmark. Analysis controls are validated before terrain access, SRTM directory paths are checked early, output/export failures include their destination path, malformed CLI controls have dedicated tests, end-to-end missing-tile/NoData scenarios verify explicit terrain failures, and the geometry/RF/link-analysis layers reject ambiguous or non-finite inputs without inventing values.

## Completed

- [x] Persistent repository documentation and AI-agent continuity rules.
- [x] Specification, architecture, engineering decisions, roadmap, testing strategy, and developer workflow.
- [x] Rust 2021 crate foundation and module tree.
- [x] Baseline GitHub Actions Rust CI.
- [x] Frequency parsing and geographic point validation.
- [x] Great-circle distance, interpolation, and sampling.
- [x] Wavelength, first Fresnel radius, and FSPL.
- [x] Independent Python geometry/RF/SRTM reference tooling.
- [x] SRTM3 HGT naming with floor semantics.
- [x] mmap-backed HGT access and big-endian signed sample decoding.
- [x] Correct HGT north-to-south row orientation and bilinear interpolation.
- [x] Explicit `NoData` handling.
- [x] Multi-tile provider lookup and tile caching.
- [x] Deterministic synthetic SRTM tests, boundary/corner tests, malformed-fixture tests, and Rust/Python SRTM reference vectors.
- [x] Configurable effective Earth radius with default `k=4/3` and parabolic bulge model.
- [x] Linear/reference LOS path, effective path, terrain clearance, and Clear/FresnelPartial/LineOfSightBlocked classification.
- [x] `ProfileSample`, `Obstacle`, and `LinkAnalysis` result models.
- [x] End-to-end clear/obstructed analysis and Rust/Python differential regression.
- [x] CLI wiring, terminal summary, and `--profile` detailed output.
- [x] PNG/SVG rendering and GeoJSON export from the same `LinkAnalysis` result.
- [x] Font-independent graphical rendering.
- [x] CLI hardening for analysis controls, SRTM directory, and destination-aware output/export failures.
- [x] Missing SRTM tile and endpoint `NoData` integration coverage.
- [x] Numerical edge-case hardening for non-finite Fresnel inputs, overflow-safe Fresnel arithmetic, effective Earth radius/bulge validation, antipodal interpolation, direct coordinate validation, and early frequency validation.
- [x] Reproducible CLI validation examples in `doc/VALIDATION.md`.
- [x] Reproducible SRTM access benchmark covering one cached tile and alternating access across two cached tiles.
- [x] Record local benchmark baseline: 96.2 ns/query for one cached tile and 101.9 ns/query for two cached tiles, each over 100,000 queries.

## Current task

Evaluate which existing Rust/Python differential cases provide enough regression value to automate further without introducing generated-data or floating-point-sensitive CI fragility.

## Known constraints

- Production implementation is Rust.
- Python reference tooling is development-only and must not become a runtime dependency.
- SRTM data is supplied locally and is not committed to the repository.
- No real SRTM fixtures are present yet.
- The CLI requires all SRTM3 tiles touched by the sampled path.
- The effective-Earth model uses the agreed parabolic approximation; a more exact propagation model is future work.
- Rendering/export consumes `LinkAnalysis` and does not recalculate domain math.
- `--output-image` currently accepts `.png` and `.svg`; other extensions are rejected explicitly.
- PNG/SVG rendering intentionally does not require a system-installed font.
- CLI failures return process exit code `2` and write the diagnostic to stderr.
- Benchmark timings are machine-dependent and are not CI pass/fail thresholds.

## Validation

- CI #124 validated presentation/export with formatting, tests, and Clippy green.
- CI #130 validated CLI hardening.
- CI #137 validated missing-tile and `NoData` integration scenarios.
- CI #155 validated numerical edge-case hardening with `cargo fmt --check`, `cargo test`, and Clippy green.
- CI #163 validated the subsequent benchmark/validation documentation changes with formatting, tests, and Clippy green.
- Local validation on 2026-09-10 passed `cargo fmt --check`, `cargo test` (30 unit tests plus integration/geo/RF/SRTM/differential suites), `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo bench --bench srtm_access`.
- Local benchmark result on 2026-09-10: 96.2 ns/query single cached tile; 101.9 ns/query alternating two cached tiles; 100,000 queries per workload.

## Next recommended task

Review `tests/differential.rs` and the existing `tools/reference/` workflow, then automate only stable, high-value reference cases. Keep performance benchmarking manual and machine-local.

## Continuity note

At the end of every implementation session, update this file so another AI agent can determine exactly where work stopped without relying on chat history.
