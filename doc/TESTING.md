# RF-Path — Testing Strategy

## Principles

Tests must be deterministic and offline. They must not depend on the user's external SRTM collection or network access.

The project uses complementary validation layers:

1. Rust unit tests;
2. Rust integration tests;
3. Python reference calculations;
4. Rust/Python differential comparisons;
5. CLI/output tests as those interfaces are implemented.

The specification is authoritative. Python is an independent validation implementation, while Rust tests are the production regression suite.

## 1. Rust unit tests

Pure domain functions should have direct tests. These are the fastest and most important regression tests for production code.

Cover at least:

- unit conversions;
- geographic distance;
- great-circle interpolation;
- wavelength;
- first Fresnel radius;
- FSPL;
- effective-Earth calculations;
- clearance classification;
- boundary conditions and invalid inputs.

## 2. SRTM tests

Fixtures should explicitly verify:

- positive and negative tile naming;
- exact integer-degree boundaries;
- `S21W048.hgt` selection for a point around `(-20.28, -47.78)`;
- big-endian signed 16-bit decoding;
- north-to-south row order;
- bilinear interpolation over known four-corner values;
- `-32768` NoData behavior;
- tile transitions on a path.

Small synthetic HGT fixtures are preferred. Real SRTM datasets must not be committed to the repository.

## 3. Analysis integration tests

Synthetic terrain providers should allow tests such as:

- completely flat terrain;
- a terrain peak that does not reach LOS;
- a peak that blocks LOS;
- a peak that intrudes into the 60% Fresnel threshold but remains below LOS;
- paths crossing two or more synthetic tiles;
- missing tiles and NoData conditions.

Assert against structured `LinkAnalysis`, not only rendered text.

## 4. Python reference tests

The Python reference layer should independently reproduce selected calculations from the specification.

Initial reference targets:

- great-circle distance/interpolation;
- SRTM tile naming and indexing;
- HGT sample decoding/interpolation on controlled fixtures;
- wavelength;
- Fresnel radius;
- effective-Earth/curvature model;
- LOS and clearance;
- FSPL;
- selected end-to-end profiles.

The Python implementation should favor explicit formulas and readability over optimization.

## 5. Differential Rust/Python testing

For selected deterministic cases, run the same inputs through both implementations and compare the resulting values.

Conceptually:

```text
                 same input
                     │
              ┌──────┴──────┐
              ▼             ▼
        Python reference   Rust
              │             │
              └──────┬──────┘
                     ▼
                 comparator
                     │
              pass / mismatch
```

Comparisons must use explicit, documented tolerances. Do not require exact binary equality for floating-point results.

Tolerances should be specific to the quantity. For example, a geographic distance in metres, an elevation in metres, and FSPL in dB may reasonably have different tolerances.

A mismatch should report enough information to reproduce the case, including the input, quantity, Rust result, Python result, absolute difference, relative difference where meaningful, and allowed tolerance.

Differential tests are especially valuable after changes to:

- coordinate math;
- SRTM indexing/interpolation;
- Earth curvature;
- Fresnel calculations;
- clearance calculations;
- FSPL;
- profile sampling.

## 6. Numerical edge cases

Test or explicitly validate:

- zero/near-zero path distance;
- invalid/zero/negative frequency;
- extremely small or large but finite distances;
- endpoint Fresnel radius of zero;
- finite-value propagation through the model;
- antimeridian and high-latitude cases where applicable;
- missing terrain tiles;
- NoData terrain samples.

## 7. Test ownership

A useful rule is:

| Layer | Purpose | Authority |
| --- | --- | --- |
| Rust unit tests | Production regression | Required |
| Rust integration tests | Production workflow | Required |
| Python reference | Independent model check | Development tooling |
| Differential comparison | Detect implementation divergence | Development tooling |
| CLI/output tests | User-facing behavior | Required as implemented |

Python failures do not automatically mean Rust is wrong; first determine whether the two implementations and the specification agree on the intended model.

Likewise, Rust and Python agreeing does not prove the model is correct if both were derived from the same mistaken assumption. Important model choices belong in `SPECIFICATION.md` and `DECISIONS.md`.

## 8. Validation commands

The normal Rust validation sequence is:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

When the reference tooling exists, an additional development validation command will be documented under `tools/reference/`.

The exact command may evolve as the project's test harness is implemented.
