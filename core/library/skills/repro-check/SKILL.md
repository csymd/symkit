---
name: repro-check
description: >
  Check that a repo can be rerun using the lockfile and README that are
  already here. Triggers: reproducibility, how do I run this, repro-check.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Repro check

Use the **existing** lockfile, container, and README. Do not recommend a
second environment manager.

## Detect first

- Env: `uv.lock` / `pyproject.toml`, `renv.lock`, `Cargo.lock`, `conda` /
  `environment.yml`, `flake.lock`, a documented container or toolbox
- Run: the first command the README or CONTRIBUTING already names
- Workstreams: whatever `study-layout` mapped — not a required
  `analysis/<name>/` path

## Checklist

- [ ] Environment documented from what is already in the tree
- [ ] One documented command to run the primary analysis
- [ ] Inputs and outputs listed per workstream README
- [ ] Seeds / config files committed if the run is stochastic
- [ ] Restricted data paths are external and documented, not committed
- [ ] Generated large binaries are gitignored or asked about
- [ ] Assumptions that are not in the aims are labeled provisional

## Output

A short pass/fail list plus the single most important gap to fix first.
Do not invent a lockfile or runner that is not here.
