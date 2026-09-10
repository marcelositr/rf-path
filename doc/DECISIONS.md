# RF-Path — Architecture & Engineering Decisions

This file records decisions that should not be casually reversed by future contributors or AI agents.

## Decision status

- **Accepted** — current project direction.
- **Provisional** — selected for v1 but intentionally open to later revision.
- **Superseded** — retained for history only.

## D001 — Repository documentation is persistent project context

**Status:** Accepted

`README.md` is the public entry point. `doc/` is the developer and AI-agent source of truth.

**Reason:** The project must remain understandable across different chats, agents, models, and contributors without relying on conversation history.

## D002 — Separate specification, architecture, state, and rationale

**Status:** Accepted

Use:

- `SPECIFICATION.md` for what the system must do;
- `ARCHITECTURE.md` for how it is organized;
- `DECISIONS.md` for why important choices were made;
- `STATUS.md` for where implementation currently stands;
- `ROADMAP.md` for planned future work.

**Reason:** Separating these concerns reduces context drift and makes repository onboarding deterministic.

## D003 — SRTM3 HGT is the initial terrain format

**Status:** Accepted

Version 1 targets 1201 × 1201 signed 16-bit big-endian SRTM3 `.hgt` tiles.

**Reason:** It is sufficient for the initial offline terrain-path use case and has a simple, documented binary format.

## D004 — HGT tiles are memory-mapped

**Status:** Accepted

Use `memmap2` rather than eagerly loading all tiles.

**Reason:** RF paths can cross multiple tiles and users may have large local DEM collections. Memory mapping keeps random access efficient without requiring the entire dataset in memory.

## D005 — Negative coordinates use floor semantics for tile naming

**Status:** Accepted

Tile identity is determined by the southwest integer-degree corner. Negative latitude/longitude must use mathematical floor.

**Reason:** Truncation toward zero assigns points near negative coordinate boundaries to the wrong tile.

## D006 — HGT row order is explicitly handled

**Status:** Accepted

HGT row zero is the northern edge; row indices increase southward.

**Reason:** Treating HGT as a conventional bottom-to-top Cartesian raster silently flips terrain vertically.

## D007 — SRTM void is not an elevation

**Status:** Accepted

`-32768` is treated as `NoData` and is not interpolated as a valid height.

**Reason:** Fabricating terrain at missing samples could produce falsely optimistic or pessimistic RF conclusions.

## D008 — Great-circle sampling is the path model

**Status:** Accepted

Use robust spherical great-circle interpolation rather than linear latitude/longitude interpolation.

**Reason:** RF paths may span enough distance that coordinate-linear interpolation is geometrically inaccurate, while great-circle sampling is explicit and testable.

## D009 — Effective Earth radius defaults to k = 4/3

**Status:** Accepted / Provisional

The default effective-Earth factor is `4/3`, with the model designed so the factor can be configured.

**Reason:** `k = 4/3` is a standard engineering approximation for a typical atmosphere. The implementation must not hard-code the assumption so tightly that alternate k values are impossible.

## D010 — Fresnel radius uses the physical SI equation

**Status:** Accepted

Internally use:

`F1 = sqrt(lambda * d1 * d2 / (d1 + d2))`

with metres and hertz, and `lambda = c/f`.

**Reason:** The physical equation avoids unit-dependent constants and makes dimensional correctness obvious.

## D011 — 60% Fresnel is a classification criterion, not a link guarantee

**Status:** Accepted

Samples are classified as Clear, FresnelPartial, or LineOfSightBlocked.

**Reason:** The 60% criterion is an engineering clearance guideline. It should not be presented as proof of real-world link performance.

## D012 — FSPL is not a complete link budget

**Status:** Accepted

Version 1 calculates free-space path loss but does not claim to calculate a complete link budget.

**Reason:** A complete budget requires additional inputs and propagation effects such as antenna gains, cable losses, receiver sensitivity, fade margin, diffraction, and atmospheric attenuation.

## D013 — Analysis owns the computed result; renderers do not recalculate

**Status:** Accepted

All renderers consume `LinkAnalysis` produced by the analysis layer.

**Reason:** Duplicating equations in presentation code creates inconsistent results and makes auditing difficult.

## D014 — Documentation state must be updated at the end of work

**Status:** Accepted

Every meaningful development session must leave `STATUS.md` accurate enough for the next agent to continue.

**Reason:** This is the mechanism that replaces lost conversation context.

## D015 — Python is a development-only reference implementation

**Status:** Accepted

Python will be used as an independent reference/laboratory implementation for validating important Rust calculations and controlled workflows.

Python is not a runtime dependency of `rf-path` and is not part of the production architecture.

**Reason:** Numerical/geospatial bugs can produce plausible output. A second, deliberately simple implementation provides an independent way to detect discrepancies between the specification and Rust implementation.

## D016 — Rust remains the production implementation and authoritative test target

**Status:** Accepted

The production application and its normal regression suite remain Rust-based. Python reference code complements Rust tests; it does not replace them.

**Reason:** The shipped product is Rust. Production behavior must be testable without Python, and users must not need Python installed to run `rf-path`.

## D017 — Differential testing uses explicit numerical tolerances

**Status:** Accepted / Provisional

When comparing Python reference results with Rust results, comparisons use quantity-specific tolerances rather than bit-for-bit floating-point equality.

**Reason:** Independent implementations can legitimately differ in floating-point rounding while representing the same physical result. Tolerances should reflect the numerical sensitivity of each quantity.

## D018 — Python reference code prioritizes clarity over performance

**Status:** Accepted

Reference implementations should use straightforward, readable algorithms and small controlled inputs where possible. They are not performance targets.

**Reason:** The purpose of the reference layer is to expose mathematical intent and discrepancies, not to duplicate Rust optimization work.

## D019 — The specification remains above the Python reference

**Status:** Accepted

Neither Python nor Rust may silently redefine the project's behavior. The specification and accepted engineering decisions remain authoritative.

**Reason:** Otherwise the two implementations could agree while both implement the wrong model.

## D020 — Effective Earth curvature uses a configurable k-factor and parabolic bulge

**Status:** Accepted / Provisional

The effective radius is `R_eff = k * R_e`, with default `k = 4/3`. For a sampled point with endpoint distances `d1` and `d2` and total distance `D = d1 + d2`, the effective-Earth bulge is approximated as:

`bulge = d1 * d2 / (2 * R_eff)`

The reference path is the linear interpolation of the absolute endpoint antenna altitudes minus this bulge. Terrain clearance is then `effective_reference_path - terrain`.

**Reason:** This gives one explicit, auditable curvature convention for v1 and avoids applying Earth curvature twice in different layers. The approximation is intentionally isolated so a more exact propagation model can replace it later without changing the surrounding analysis contract.
