---
name: repro-check
description: >
  Check that a research repo can be rerun: lockfile, how-to-run, seeds,
  artifact paths, and missing assumptions. Triggers: reproducibility,
  how do I run this, repro-check.
---

<!--
Copyright (c) 2026, cSYMd
Licensed under Apache 2.0
-->

# Repro check

## Checklist

- [ ] Environment documented (`uv.lock` / `pyproject.toml` / container)
- [ ] One documented command to run the primary analysis
- [ ] Inputs and outputs listed per workstream README
- [ ] Seeds / config files committed if the run is stochastic
- [ ] Restricted data paths are external and documented, not committed
- [ ] Generated large binaries are gitignored or asked about
- [ ] Assumptions that are not in the aims are labeled provisional

## Output

A short pass/fail list plus the single most important gap to fix first.
