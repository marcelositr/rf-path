# RF-Path — AI Development Guide

## Purpose

This document is the operating contract for AI agents working on RF-Path.

RF-Path is an offline Rust CLI for terrain-aware RF path analysis using local SRTM/DEM data. The repository, especially `doc/`, is the persistent project context. Conversation history is not a source of truth.

## Mandatory startup procedure

Before changing code:

1. Read `README.md`.
2. Read this file.
3. Read `doc/STATUS.md`.
4. Read `doc/SPECIFICATION.md`.
5. Read `doc/ARCHITECTURE.md`.
6. Read `doc/DECISIONS.md`.
7. Read `doc/ROADMAP.md`.
8. Inspect the relevant source and tests.
9. Run the existing test suite before making substantial changes.

If the repository contains newer or more specific documentation, follow it and update the documentation when necessary.

## Source of truth

The repository is the source of truth for project requirements, architecture, mathematical models, decisions, current state, tests, and roadmap.

Do not silently replace documented decisions with assumptions from memory, another project, or a convenient implementation.

If code and documentation disagree:

- identify the disagreement;
- determine whether the code or documentation represents the intended current state;
- preserve working behavior unless there is a justified change;
- document important corrections.

## Core engineering rules

- Keep the application offline-first.
- Use Rust 2021 and idiomatic Rust.
- Prefer `Result`/`Option` over panics for expected failures.
- Keep RF mathematics deterministic and independently testable.
- Use SI units internally: metres, seconds, hertz, radians where appropriate.
- Keep parsing, geography, terrain access, RF calculations, analysis orchestration, and rendering separated.
- Do not hide unit conversions inside mathematical functions.
- Do not silently fabricate terrain data when SRTM contains `NoData`.
- Treat SRTM as terrain elevation, not as a complete surface model containing buildings or vegetation.
- Do not represent Fresnel clearance as a binary guarantee of link performance.

## Mathematical integrity

Do not change RF or geodetic equations merely to make an implementation easier.

A change to a mathematical model must:

1. be technically justified;
2. be recorded in `doc/DECISIONS.md` when it is architectural or model-defining;
3. have or update tests;
4. update `doc/SPECIFICATION.md` when observable behavior changes.

## SRTM rules

- SRTM HGT tiles are selected from the tile's southwest integer-degree corner.
- HGT samples are stored north-to-south and must not be interpreted as a conventional image with the same vertical orientation.
- Standard SRTM3 tiles use 1201 × 1201 signed 16-bit big-endian samples.
- `-32768` is treated as `NoData`.
- A link may cross multiple tiles.
- Tile access should be cacheable and should not require loading the whole dataset into memory.

## RF model rules

The initial model is a terrain/geometry analyzer, not a complete propagation simulator.

The initial scope includes:

- great-circle sampling;
- SRTM terrain elevation;
- effective Earth radius using a configurable `k` factor, default 4/3;
- line-of-sight geometry;
- first Fresnel-zone radius;
- 60% Fresnel clearance classification;
- free-space path loss.

It does not claim to model buildings, vegetation, antenna radiation patterns, multipath, interference, detailed diffraction, weather attenuation, or a complete link budget unless those capabilities are explicitly added and documented.

## Implementation workflow

For each task:

1. Understand the requested change.
2. Check `STATUS.md` and `ROADMAP.md`.
3. Locate existing implementation and tests.
4. Make the smallest coherent change.
5. Add/update tests.
6. Run `cargo fmt`.
7. Run `cargo test`.
8. Run `cargo clippy` when available.
9. Review the resulting behavior and interfaces.
10. Update `STATUS.md`.
11. Update `DECISIONS.md` if a significant decision was made.
12. Update `CHANGELOG.md` for user-visible or meaningful project changes.

## Continuity between AI sessions

At the end of a session, the repository must contain enough information for another agent to continue without the previous conversation.

`doc/STATUS.md` must identify:

- current phase;
- completed work;
- current task;
- known problems;
- next recommended task;
- validation status.

Do not leave critical context only in chat.

## When requirements are ambiguous

Do not invent domain behavior silently.

Use the existing specification and decisions first. If a choice materially affects correctness, document the alternatives and selected behavior before implementing it.

## Definition of done

A task is not complete merely because the code compiles. It is complete when implementation, tests, documentation, and project state are consistent.
