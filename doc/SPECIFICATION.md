# RF-Path — Technical Specification

## 1. Purpose

RF-Path is an offline command-line tool for analysing an RF link path over terrain represented by local SRTM/DEM data.

The primary output is an auditable terrain profile showing the relationship between terrain, the nominal line of sight, effective Earth curvature, and the first Fresnel zone.

## 2. Scope of version 1

Version 1 should provide:

- two geographic endpoints with antenna heights above local terrain;
- configurable frequency;
- local SRTM3 HGT terrain lookup;
- great-circle path sampling;
- effective-Earth-radius modelling with configurable `k`, default `4/3`;
- first Fresnel radius calculation;
- 60% Fresnel clearance analysis;
- free-space path loss (FSPL);
- terminal/ASCII profile output;
- optional PNG/SVG rendering;
- optional GeoJSON export.

RF-Path is not a complete RF link-budget or propagation simulator.

## 3. Inputs

### 3.1 Geographic points

Coordinates are latitude/longitude in decimal degrees.

An antenna height is measured vertically above the local terrain elevation returned by the DEM.

Absolute endpoint height is therefore:

`antenna_altitude = terrain_elevation + antenna_height_above_ground`

The implementation must not confuse antenna height above ground with absolute altitude.

### 3.2 Frequency

The CLI may accept human-friendly units such as MHz or GHz, but calculations use hertz internally.

Frequency must be positive and finite.

### 3.3 SRTM directory

The user supplies a local directory containing `.hgt` tiles.

For an integer-degree tile, the filename is derived from its southwest corner. For example, a point around latitude `-20.28`, longitude `-47.78` is inside tile `S21W048.hgt`.

Negative coordinates require floor semantics, not truncation toward zero.

## 4. Terrain model

### 4.1 HGT format

The initial target is SRTM3:

- 1201 × 1201 samples;
- signed 16-bit integers;
- big-endian byte order;
- approximately 3 arc-second spacing.

The implementation should use memory mapping (`memmap2`) for efficient random access without eagerly loading every tile.

### 4.2 Row orientation

HGT sample row zero is the northern edge of the tile. Rows progress southward.

Latitude-to-row conversion must therefore invert the usual Cartesian/image Y direction correctly.

### 4.3 NoData

The standard SRTM void value `-32768` is not a terrain elevation.

Interpolation must explicitly handle void samples. RF analysis must not silently turn missing terrain into invented terrain.

The preferred initial behavior is to return a typed `NoData`/terrain lookup error when a required interpolation neighborhood cannot be evaluated reliably.

### 4.4 Multiple tiles

A path may cross tile boundaries. Terrain lookup must select the appropriate tile independently for each sample.

An internal cache of opened/memory-mapped tiles is recommended so a long path does not repeatedly open the same files.

### 4.5 Interpolation

Bilinear interpolation is the initial interpolation method.

The four neighboring samples must all be valid unless an explicitly documented alternative void policy is introduced.

## 5. Geographic path

The path between endpoints must follow the great-circle route rather than linear interpolation of latitude and longitude.

Sampling should use a numerically robust spherical interpolation method such as SLERP between endpoint unit vectors.

The first and last samples correspond to the two endpoints.

## 6. Earth curvature

The initial model uses an effective Earth radius:

`R_eff = k * R_e`

where the default `k = 4/3` represents a standard-atmosphere engineering approximation.

For the sampled path, the vertical Earth-curvature effect is represented consistently relative to the chosen effective-radius model. A parabolic approximation may be used for the initial profile implementation, provided the sign and reference system are consistent.

The model must not apply Earth curvature twice or mix incompatible geometric references.

The physical Earth radius should be centralized in one documented constant or model implementation.

## 7. Line of sight

Let:

- `D` = total path distance in metres;
- `x` = distance from transmitter to the sample;
- `h_tx` = absolute transmitter altitude;
- `h_rx` = absolute receiver altitude.

The nominal straight line between endpoints is:

`h_los(x) = h_tx + (h_rx - h_tx) * x / D`

The effective-Earth correction is then applied consistently with the selected Earth model.

Clearance is the vertical separation between the terrain and the modelled line-of-sight reference:

`clearance = h_los_model - h_terrain`

Positive clearance means terrain is below the reference path; negative clearance means terrain intrudes into it.

## 8. First Fresnel zone

Wavelength is:

`lambda = c / f`

For a sample at distance `x` on a path of total length `D`, the first Fresnel radius is:

`F1(x) = sqrt(lambda * x * (D - x) / D)`

All distances are in metres internally.

The maximum first Fresnel radius occurs near the midpoint.

## 9. Fresnel clearance classification

The default engineering criterion is 60% of the first Fresnel radius.

For each sample:

- `Clear`: `clearance >= 0.6 * F1`;
- `FresnelPartial`: `0 <= clearance < 0.6 * F1`;
- `LineOfSightBlocked`: `clearance < 0`.

This classification is an engineering geometry indicator, not a guarantee that a real RF link will work.

Endpoint samples naturally have a Fresnel radius of zero.

## 10. Free-space path loss

FSPL is calculated from the physical SI relationship:

`FSPL_dB = 20 * log10(4 * pi * d * f / c)`

where `d` is metres and `f` is hertz.

Equivalent MHz/km or GHz/km formulas may be used only at interfaces or presentation layers.

FSPL describes free-space spreading loss only. It is not a complete link budget.

## 11. Analysis result

The analysis layer should preserve enough information to audit the result.

At minimum, the result should contain:

- total path distance;
- input frequency;
- TX/RX coordinates and antenna heights;
- terrain elevation at both endpoints;
- calculated FSPL;
- profile samples;
- minimum terrain clearance;
- minimum clearance ratio relative to the first Fresnel radius where defined;
- worst obstruction/sample location;
- LOS-blocked state;
- 60%-Fresnel state.

A profile sample should retain at least:

- distance from TX;
- latitude/longitude;
- terrain elevation;
- modelled LOS altitude/reference;
- Earth-curvature contribution if separately represented;
- Fresnel radius;
- clearance;
- clearance ratio where defined;
- classification.

## 12. Outputs

### 12.1 Terminal

The CLI should provide a concise summary and a useful terrain/profile representation suitable for offline operation.

### 12.2 Image

PNG and SVG are optional presentation outputs. Rendering must consume analysis data rather than reimplement RF mathematics.

### 12.3 GeoJSON

GeoJSON should use WGS84 longitude/latitude coordinate order and may include:

- the link line;
- TX and RX points;
- the worst obstruction;
- optionally the sampled profile.

Export must not alter the analysis model.

## 13. CLI direction

The intended initial interface is approximately:

```text
rf-path \
  --tx="-20.2831,-47.7812,12" \
  --rx="-20.3541,-47.8523,15" \
  --freq=2.4GHz \
  --srtm-dir=./srtm \
  --samples=500 \
  --output-image=profile.png \
  --export-geojson=link.geojson
```

Defaults:

- samples: `500`;
- k-factor: `4/3`;
- Fresnel clearance threshold: `0.60`.

## 14. Explicit limitations

SRTM represents terrain elevation and does not provide a complete surface model for buildings, trees, towers, or other local obstructions.

Version 1 does not claim to model:

- antenna radiation patterns;
- polarization;
- TX power or receiver sensitivity;
- cable/connector losses;
- fade margin;
- multipath;
- interference;
- detailed diffraction loss;
- rain or atmospheric attenuation;
- clutter/vegetation;
- building geometry;
- a complete link budget.

These are future capabilities and must be added explicitly rather than implied by the current result.

## 15. Quality requirements

Mathematical functions must be independently testable.

Tests should cover at least:

- SRTM tile naming, especially negative coordinates;
- HGT big-endian sample decoding;
- HGT row orientation;
- bilinear interpolation;
- NoData handling;
- great-circle endpoint and midpoint behavior;
- Fresnel radius known values;
- FSPL known values;
- effective-Earth model behavior;
- 60% classification boundaries;
- multi-tile path lookup;
- end-to-end analysis with controlled test terrain.
