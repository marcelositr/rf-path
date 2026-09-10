# RF-Path — Architecture

## 1. Architectural goals

The architecture separates domain mathematics from I/O and presentation so that the RF model can be tested without real SRTM files or terminal rendering.

The preferred dependency direction is:

`CLI → analysis → {geo, srtm, rf}`

with rendering/export consuming analysis results rather than recalculating them.

## 2. Planned modules

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

Examples:

- GHz/MHz → Hz;
- km → m where applicable.

### `src/geo.rs`

Geographic primitives and path geometry.

Responsibilities:

- `GeoPoint`;
- antenna endpoint representation;
- angular/radian conversion;
- great-circle distance;
- robust great-circle interpolation.

It should not know how SRTM files are stored.

### `src/srtm.rs`

SRTM HGT access layer.

Responsibilities:

- identify HGT tile from latitude/longitude;
- construct canonical tile filenames;
- open and memory-map HGT files;
- decode signed big-endian 16-bit samples;
- handle north-to-south row ordering;
- bilinear interpolation;
- detect `NoData`;
- cache open tiles.

The rest of the application should depend on a terrain lookup abstraction where practical, allowing tests to use synthetic terrain.

### `src/rf.rs`

Pure RF and propagation-model functions.

Responsibilities:

- wavelength;
- first Fresnel radius;
- FSPL;
- effective Earth-radius model;
- curvature/reference-path calculations.

Functions should be deterministic and independent of filesystem/CLI concerns.

### `src/analysis.rs`

Orchestrates the complete link analysis.

Responsibilities:

- generate great-circle samples;
- query terrain;
- calculate endpoint altitudes;
- calculate LOS/reference path;
- calculate Fresnel radius and clearance;
- classify each sample;
- calculate summary metrics and worst obstruction;
- produce a structured `LinkAnalysis` result.

This module is the main domain workflow.

### `src/render.rs`

Presentation only.

Responsibilities:

- terminal/ASCII profile;
- PNG rendering;
- SVG rendering.

It must consume `LinkAnalysis` and never independently recalculate terrain, Fresnel, FSPL, or curvature.

### `src/geojson_export.rs`

Converts analysis results into GeoJSON.

It should not own RF calculations.

### `src/error.rs`

Shared typed errors and application `Result` conventions.

Expected failures include missing tiles, malformed HGT files, invalid coordinates, invalid frequency, and terrain `NoData`.

## 3. Core domain model

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

Names may evolve during implementation, but the separation of concerns should remain.

## 4. Terrain abstraction

Analysis should ideally depend on a small terrain interface such as:

```text
TerrainProvider::elevation_at(point) -> Result<meters>
```

The production implementation is backed by SRTM HGT files. Tests can provide a deterministic in-memory implementation.

This avoids making mathematical tests dependent on external DEM files.

## 5. Data flow

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
```

## 6. Error handling

Errors should propagate through typed `Result` values.

Do not use `unwrap()`/`expect()` for user-controlled data or filesystem operations.

Panics may be acceptable only for genuine programmer invariants that cannot arise from external input, and should be rare.

## 7. Performance strategy

The main expected performance risks are terrain I/O and large profile sampling.

Initial strategy:

- memory-map HGT files;
- cache opened tiles;
- avoid loading all DEM tiles into RAM;
- calculate the profile in one main pass where possible;
- keep rendering separate from analysis.

Premature parallelism is not required for version 1. Benchmark before introducing complexity.

## 8. Testability

Pure functions should be tested at unit level.

Integration tests should exercise the complete analysis with a synthetic terrain provider or controlled HGT fixtures.

Filesystem-specific HGT behavior belongs in SRTM tests; RF formulas should not require HGT files.

## 9. Dependency policy

Dependencies should be added only when they have a clear role.

Initial intended dependencies:

- `clap` — CLI;
- `memmap2` — HGT memory mapping;
- `byteorder` or equivalent explicit endian decoding;
- `plotters` — PNG/SVG charts;
- `serde` / `serde_json` — serialization;
- `geojson` — GeoJSON representation.

Avoid introducing a large GIS framework when the required v1 geometry can remain explicit and auditable.
