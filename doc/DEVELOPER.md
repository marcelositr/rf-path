# RF-Path — Developer Guide

## Prerequisites

Install a current stable Rust toolchain with Cargo.

## Standard workflow

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

## Coding conventions

- Prefer small, focused modules.
- Keep domain calculations pure where practical.
- Use explicit names for units (`*_m`, `*_hz`, `*_deg`) when ambiguity is possible.
- Keep external/user units at the CLI boundary.
- Return typed errors for expected failures.
- Avoid hidden global state.
- Avoid unnecessary allocations in terrain sampling paths.
- Do not duplicate RF equations across modules.

## Tests

Tests should be deterministic and runnable without network access.

Use synthetic terrain fixtures for analysis tests. Do not require a user's private SRTM collection for ordinary unit tests.

When adding a formula, add at least one known-value test and boundary tests where relevant.

## Documentation maintenance

Update documentation when behavior changes:

- `SPECIFICATION.md` — observable requirements/model;
- `ARCHITECTURE.md` — module/data-flow changes;
- `DECISIONS.md` — important choices and rationale;
- `STATUS.md` — current implementation state;
- `ROADMAP.md` — milestone changes;
- `CHANGELOG.md` — meaningful project history.

## Commit discipline

Prefer small, coherent commits with messages describing the change, for example:

```text
feat: implement SRTM HGT interpolation
fix: handle negative longitude tile selection
 test: add Fresnel radius vectors
```

Avoid mixing unrelated refactors with functional changes.

## SRTM fixtures

Real SRTM data should not be committed to the repository. Small synthetic HGT fixtures are appropriate for tests when their binary layout is useful for validating the reader.

## Pull requests

A meaningful change should include:

- implementation;
- tests;
- documentation updates when required;
- confirmation of `cargo fmt`, `cargo test`, and `cargo clippy`.

Reviewers should pay particular attention to unit consistency, coordinate edge cases, HGT row orientation, NoData handling, and mathematical changes.
