---
name: signal-quality
description: >
  Check wearable, force-plate, or IMU data quality and provenance
  (units, sampling, missingness, who/when). Use before analysis. Do not
  invent a cleaned series. Triggers: signal quality, IMU check, force
  plate QC, bad trace, signal-quality, /signal-quality.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Signal quality

Provenance and obvious quality issues. Wrap the repo’s kernel for
processing.

## Steps

1. What file, device, units, and sampling rate are *documented*.
2. Who / when / which trial — if identifiers would be PHI, keep them out
   of git and out of the report.
3. Missing samples, clipped ranges, dropped packets, clock issues.
4. If SymWorx (or another kernel) already filters or detects events, call
   it. Do not paste a new filter “real quick”.
5. Do not write a “cleaned” series you did not compute from the files.

## Do not

- Invent sampling rates or calibration.
- Commit restricted device dumps.

## Output

- Provenance table (path, units, fs, gaps)
- Pass / fail / unknown per check
- Next command or kernel entrypoint, if the repo has one
