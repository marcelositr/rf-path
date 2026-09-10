# RF-Path — Testing Strategy

## Principles

Tests must be deterministic, offline, and independent of the user's external SRTM collection.

The test suite is divided into:

1. pure mathematical/unit tests;
2. SRTM format and interpolation tests;
3. integration tests for the complete analysis workflow;
4. CLI/output tests as those interfaces are implemented.

## Geometry vectors

Verify:

- identical endpoints have zero distance;
- interpolation returns the exact endpoints;
- midpoint sampling lies on the expected great-circle route;
- antimeridian and high-latitude behavior is numerically stable where applicable.

## SRTM vectors

Fixtures should explicitly verify:

- positive and negative tile naming;
- exact integer-degree boundaries;
- `S21W048.hgt` selection for a point around `(-20.28, -47.78)`;
- big-endian signed 16-bit decoding;
- north-to-south row order;
- bilinear interpolation over known four-corner values;
- `-32768` NoData behavior;
- tile transitions on a path.

## RF vectors

Use known physical values to verify:

- wavelength;
- first Fresnel radius;
- FSPL;
- effective Earth-radius calculations;
- LOS/reference profile;
- clearance classification at exactly 0, exactly 60% Fresnel, and just below/above those boundaries.

## Analysis tests

Synthetic terrain providers should allow tests such as:

- completely flat terrain;
- a terrain peak that does not reach LOS;
- a peak that blocks LOS;
- a peak that intrudes into the 60% Fresnel threshold but remains below LOS;
- paths crossing two or more synthetic tiles.

The expected result should be asserted from the structured `LinkAnalysis`, not only from rendered text.

## Numerical edge cases

Test or explicitly validate:

- zero/near-zero path distance;
- invalid/zero/negative frequency;
- extremely small or large but finite distances;
- endpoint Fresnel radius of zero;
- finite-value propagation through the model;
- missing terrain tiles;
- NoData terrain samples.

## Validation command

The normal local validation sequence is:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

The exact command may be adjusted as the project's feature set evolves.
