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

## Planned layout

```text
tools/reference/
├── README.md
├── geo_reference.py
├── srtm_reference.py
├── rf_reference.py
└── compare.py
```

The scripts will be introduced incrementally as the corresponding Rust modules are implemented.
