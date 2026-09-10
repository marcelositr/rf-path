# rf-path

Offline RF link-path analysis using local SRTM terrain data.

RF-Path is a Rust CLI focused on terrain-aware radio-link geometry. It samples the great-circle path between two antennas, reads local SRTM/DEM elevation, models effective Earth curvature, evaluates line-of-sight and first Fresnel-zone clearance, and calculates free-space path loss.

## Goals

- Work offline with local terrain data.
- Produce deterministic, auditable path analysis.
- Keep geographic, terrain, RF mathematics, and presentation concerns separated.
- Provide useful terminal, image, and GeoJSON outputs.

## Current capabilities

- SRTM3 `.hgt` terrain support.
- Great-circle path sampling.
- Effective Earth radius with configurable `k` factor (default `4/3`).
- First Fresnel-zone radius and 60% clearance classification.
- Free-space path loss (FSPL).
- Integrated `LinkAnalysis` profile with worst-point and blocking metrics.
- CLI execution with an auditable terminal summary.

PNG/SVG rendering and GeoJSON export are planned but are not connected to the CLI yet.

## Usage

```text
rf-path \
  --tx="-20.2831,-47.7812,12" \
  --rx="-20.3541,-47.8523,15" \
  --freq=2.4GHz \
  --srtm-dir=./srtm \
  --samples=500
```

The endpoint format is `latitude,longitude,antenna_height_m`, where antenna height is measured above local terrain.

Defaults:

- samples: `500`;
- k-factor: `4/3`;
- Fresnel clearance threshold: `0.60`.

The supplied SRTM directory must contain the required `.hgt` tiles for all sampled path positions.

## Documentation

The `doc/` directory is the persistent source of truth for development and AI-agent continuity:

- [`doc/AI.md`](doc/AI.md) — mandatory onboarding and operating rules for AI agents.
- [`doc/SPECIFICATION.md`](doc/SPECIFICATION.md) — technical behavior and mathematical model.
- [`doc/ARCHITECTURE.md`](doc/ARCHITECTURE.md) — modules, data flow, and design boundaries.
- [`doc/DECISIONS.md`](doc/DECISIONS.md) — important engineering decisions and rationale.
- [`doc/STATUS.md`](doc/STATUS.md) — current implementation state and next task.
- [`doc/ROADMAP.md`](doc/ROADMAP.md) — implementation phases and future work.
- [`doc/DEVELOPER.md`](doc/DEVELOPER.md) — development workflow and conventions.
- [`doc/TESTING.md`](doc/TESTING.md) — testing strategy and validation vectors.
- [`doc/CHANGELOG.md`](doc/CHANGELOG.md) — project history.

## Important limitations

SRTM is terrain elevation data, not a complete surface model. Buildings, trees, vegetation, and other local obstructions are not represented by the initial model.

FSPL is free-space spreading loss, not a complete RF link budget. The v1 scope does not claim to model antenna patterns, multipath, interference, detailed diffraction, weather attenuation, or receiver sensitivity.

The 60% Fresnel criterion is an engineering clearance indicator, not a guarantee of real-world link performance.

## License

MIT — see [`LICENSE`](LICENSE).
