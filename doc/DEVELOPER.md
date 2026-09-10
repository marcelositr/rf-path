# RF-Path — Developer Guide

## Prerequisites

Install a current stable Rust toolchain with Cargo.

Python 3 is recommended for development and validation tooling. Python is not required to build or run the production `rf-path` binary.

## Standard Rust workflow

Before editing:

```bash
cargo test
```

During development:

```bash
cargo fmt
cargo test
cargo clippy
```

For a release-quality check, run all three again after the final changes.

## Python reference workflow

Python tooling lives under `tools/reference/` and should be runnable independently from the Rust crate.

The reference implementation should be easy to execute against small, deterministic cases. Keep its setup lightweight and document any development-only dependencies in `tools/reference/README.md` when they are introduced.

Use Python particularly when:

- deriving or checking a new formula;
- investigating a numerical discrepancy;
- validating a known-value vector;
- comparing profile samples produced by Rust;
- testing controlled HGT fixtures;
- exploring edge cases before encoding them as Rust regression tests.

A Python experiment is not a substitute for adding the final regression test to Rust when the behavior belongs to the production application.

## Coding conventions

- Prefer small, focused modules.
- Keep domain calculations pure where practical.
- Use explicit names for units (`*_m`, `*_hz`, `*_deg`) when ambiguity is possible.
- Keep external/user units at the CLI boundary.
- Return typed errors for expected failures.
- Avoid hidden global state.
- Avoid unnecessary allocations in terrain sampling paths.
- Do not duplicate RF equations across production modules.

## Numerical development workflow

When implementing or changing numerical logic:

1. Start from `doc/SPECIFICATION.md`.
2. Check related decisions in `doc/DECISIONS.md`.
3. Build a small Python reference calculation when useful.
4. Create known-value and boundary cases.
5. Implement the Rust version.
6. Compare Rust against Python using explicit tolerances.
7. Turn important cases into permanent Rust tests.
8. Update documentation if the model or behavior changed.

This workflow is intended to catch plausible-but-wrong results early.

## Tests

Tests should be deterministic and runnable without network access.

Use synthetic terrain fixtures for analysis tests. Do not require a user's private SRTM collection for ordinary unit tests.

When adding a formula, add at least one known-value test and boundary tests where relevant.

Python differential checks should complement the Rust suite rather than become a runtime requirement.

## Documentation maintenance

Update documentation when behavior changes:

- `SPECIFICATION.md` — observable requirements/model;
- `ARCHITECTURE.md` — module/data-flow changes;
- `DECISIONS.md` — important choices and rationale;
- `STATUS.md` — current implementation state;
- `ROADMAP.md` — milestone changes;
- `CHANGELOG.md` — meaningful project history.

Changes to the Python validation architecture should also be reflected in `ARCHITECTURE.md` and `TESTING.md`.

## Commit discipline

Prefer small, coherent commits with messages describing the change, for example:

```text
feat: implement SRTM HGT interpolation
fix: handle negative longitude tile selection
test: add Fresnel radius vectors
test: add Python differential validation
```

Avoid mixing unrelated refactors with functional changes.

## SRTM fixtures

Real SRTM data should not be committed to the repository. Small synthetic HGT fixtures are appropriate for tests when their binary layout is useful for validating the reader.

## Pull requests

A meaningful change should include:

- implementation;
- tests;
- documentation updates when required;
- confirmation of `cargo fmt`, `cargo test`, and `cargo clippy`;
- Python differential validation for numerical changes when applicable.

Reviewers should pay particular attention to unit consistency, coordinate edge cases, HGT row orientation, NoData handling, numerical tolerances, and mathematical changes.
