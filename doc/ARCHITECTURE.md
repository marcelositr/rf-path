# RF-Path — Architecture

## 1. Architectural goals

The architecture separates domain mathematics from I/O, reference tooling, and presentation so that the RF model can be tested independently and audited.

The preferred production dependency direction is:

`CLI → analysis → {geo, srtm, rf}`

with rendering/export consuming analysis results rather than recalculating them.

A separate Python reference layer may validate the Rust implementation, but Python is **development/test tooling only** and is never a runtime dependency of `rf-path`.

## 2. Planned repository structure

```text
rf-path/
├── README.md
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── error.rs
│   ├── units.rs
│   ├── geo.rs
│   ├── srtm.rs
│   ├── rf.rs
│   ├── analysis.rs
│   ├── render.rs
│   └── geojson_export.rs
├── tests/
│   ├── srtm.rs
│   ├── geo.rs
│   ├── rf.rs
│   └── integration.rs
├── tools/
│   └── reference/
│       ├── README.md
│       ├── geo_reference.py
│       ├── srtm_reference.py
│       ├── rf_reference.py
│       └── compare.py
└── doc/
```

The exact Python file split may evolve, but reference tooling must remain clearly separated from production Rust code.

## 3. Planned Rust modules

### `src/main.rs`

Application entry point. Responsible for wiring dependencies, invoking the analysis workflow, and selecting outputs.

It should remain small.

### `src/cli.rs`

Command-line parsing with `clap`.

Responsibilities:

- parse TX/RX input;
- parse frequency and other user-facing units;
- validate CLI-level constraints;
- expose defaults.

It must not contain RF equations.

### `src/units.rs`

Centralizes conversion from CLI units to internal SI units.

### `src/geo.rs`

Geographic primitives and path geometry: points, angular conversion, great-circle distance, and robust great-circle interpolation.

It should not know how SRTM files are stored.

### `src/srtm.rs`

SRTM HGT access layer: tile naming, memory mapping, endian decoding, row orientation, bilinear interpolation, NoData handling, and tile caching.

The analysis layer should depend on a small terrain abstraction so tests can use synthetic terrain.

### `src/rf.rs`

Pure RF and propagation-model functions: wavelength, Fresnel radius, FSPL, effective Earth radius, curvature, LOS/reference-path calculations, and clearance classification.

Functions should be deterministic and independent of filesystem/CLI concerns.

### `src/analysis.rs`

Orchestrates path sampling, terrain queries, endpoint altitude calculation, RF geometry, per-sample classification, and summary metrics.

This module owns the structured `LinkAnalysis` result.

### `src/render.rs`

Presentation only: terminal/ASCII, PNG, and SVG. It consumes `LinkAnalysis` and never independently recalculates terrain or RF mathematics.

### `src/geojson_export.rs`

Converts analysis results into GeoJSON. It does not own RF calculations.

### `src/error.rs`

Shared typed errors and application `Result` conventions.

## 4. Python reference layer

### Purpose

Python is an intentional **reference and laboratory implementation** for validating the Rust implementation.

The reference layer exists because the project contains numerical/geospatial logic where a bug can still produce plausible-looking output. A second, simple implementation makes discrepancies visible.

Python should favor clarity over performance. It may use straightforward formulas and data structures even when Rust uses optimized implementations.

### Responsibilities

The Python reference implementation may reproduce, independently and transparently:

- unit conversions;
- great-circle distance and sampling;
- SRTM tile naming/indexing rules;
- HGT decoding and interpolation using small fixtures;
- wavelength and Fresnel calculations;
- effective-Earth/curvature calculations;
- LOS and clearance calculations;
- FSPL;
- selected end-to-end profile calculations.

### Non-responsibilities

Python must not:

- become a runtime dependency;
- be required for normal `rf-path` execution;
- replace Rust production tests;
- silently define behavior that is absent from the project specification;
- become a second production implementation that has to be maintained feature-for-feature.

The specification remains authoritative. Python is an independent check against the specification, not the source of truth.

## 5. Differential validation

Where practical, the project should compare Rust and Python outputs for the same controlled inputs.

A comparison should define explicit tolerances rather than requiring bit-for-bit floating-point equality.

Example categories:

```text
input case
    ↓
Python reference ──────┐
                       ├── compare with tolerances
Rust implementation ───┘
                       ↓
                 pass / discrepancy
```

The comparison tooling should report the first meaningful discrepancy and enough context to reproduce it.

For numerical results, tolerances must be documented according to the quantity being compared. Distances, elevations, angles, and dB values do not necessarily require identical tolerances.

## 6. Core domain model

The target model is approximately:

```text
GeoPoint
  lat_deg
  lon_deg

AntennaPoint
  position: GeoPoint
  antenna_height_m

ProfileSample
  distance_m
  lat_deg
  lon_deg
  terrain_m
  los_m
  earth_bulge_m
  fresnel_radius_m
  clearance_m
  clearance_ratio
  status

Obstacle
  distance_m
  position
  terrain_m
  los_m
  fresnel_radius_m
  clearance_m
  clearance_ratio

LinkAnalysis
  distance_m
  frequency_hz
  tx
  rx
  tx_ground_m
  rx_ground_m
  fspl_db
  min_clearance_m
  min_clearance_ratio
  los_blocked
  fresnel_60_blocked
  worst_point
  samples
```

Names may evolve, but separation of concerns should remain.

## 7. Terrain abstraction

Analysis should ideally depend on a small terrain interface such as:

```text
TerrainProvider::elevation_at(point) -> Result<meters>
```

The production implementation is backed by SRTM HGT files. Tests can provide deterministic in-memory terrain.

This keeps mathematical and integration tests independent of a user's SRTM collection.

## 8. Data flow

```text
CLI arguments
    ↓
validated input
    ↓
great-circle path generation
    ↓
terrain provider ──────┐
    ↓                  │
RF geometry/model ─────┤
    ↓                  │
ProfileSample[] ←──────┘
    ↓
LinkAnalysis
    ├── terminal renderer
    ├── image renderer
    └── GeoJSON exporter

Development-only validation path:

Specification
    ├──→ Python reference
    └──→ Rust implementation
              │
              └──→ differential comparison
```

## 9. Error handling

Errors should propagate through typed `Result` values. Do not use `unwrap()`/`expect()` for user-controlled data or filesystem operations.

## 10. Performance strategy

Production Rust is responsible for performance. Initial strategy:

- memory-map HGT files;
- cache opened tiles;
- avoid loading all DEM tiles into RAM;
- calculate profiles in one main pass where possible;
- keep rendering separate from analysis.

Python reference code is explicitly not performance-critical.

Premature parallelism is not required for v1. Benchmark before introducing complexity.

## 11. Testability

The project uses three complementary validation layers:

1. **Rust unit tests** — fast, authoritative regression tests for production code.
2. **Rust integration tests** — complete workflows using controlled/synthetic terrain.
3. **Python reference/differential tests** — independent validation that important numerical behavior agrees with the specification.

Python comparisons should complement, not replace, the Rust test suite.

## 12. Dependency policy

Production dependencies should be added only when they have a clear role.

Initial intended dependencies:

- `clap` — CLI;
- `memmap2` — HGT memory mapping;
- `byteorder` or equivalent explicit endian decoding;
- `plotters` — PNG/SVG charts;
- `serde` / `serde_json` — serialization;
- `geojson` — GeoJSON representation.

Python tooling may have its own development-only dependencies, but the production Rust binary must remain independent of Python.
