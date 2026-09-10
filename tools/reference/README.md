# Python Reference Tools

This directory contains development-only Python implementations used as an independent reference for RF-Path numerical and geospatial behavior.

Python is not a runtime dependency of `rf-path`.

## Purpose

Use these tools to:

- explore and validate formulas;
- generate known-value cases;
- investigate discrepancies;
- compare selected Rust results against an independent implementation;
- validate controlled SRTM/HGT fixtures.

The project specification and accepted decisions remain authoritative. Python does not define production behavior by itself.

## End-to-end reference vector

`end_to_end_reference.py` generates `link_analysis_clear_reference.json`, a deterministic flat-terrain 21-sample profile using the same public inputs as the Rust integration case. The Rust test consumes the checked-in JSON vector and compares all summary and per-sample numerical fields with quantity-specific tolerances.

Regenerate it with:

```bash
python3 tools/reference/end_to_end_reference.py
```

The generated JSON is intentionally checked into the repository so CI remains fully offline and the Rust test has a stable differential regression vector.

## Layout

```text
tools/reference/
├── README.md
├── geo_reference.py
├── srtm_reference.py
├── rf_reference.py
├── end_to_end_reference.py
├── link_analysis_clear_reference.json
└── compare.py
```
