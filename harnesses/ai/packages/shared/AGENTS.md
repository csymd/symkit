<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# AGENTS.md — model experiment workspace

Instructions for AI coding agents in a **model-run / evaluation** repository.

## Mission

Keep experiment configuration, runs, and metrics reproducible and comparable.
Config is the source of truth for what was run.

## Hard rules

- Do not invent metrics, seeds, or “we ran this” claims.
- Log provenance with every run (model id, config, seed, command, timestamp).
- Prefer sentence- or step-granular artifacts when diffs matter.
- Do not silently change evaluation formulas to match a hoped-for result.

## Layout (typical)

| Path | Role |
|:-----|:-----|
| `config.yaml` | Models, phases, seeds, arms |
| `scripts/` | Run and metric entrypoints |
| `experiments/` or `runs/` | Per-run outputs |
| `metrics/` | Aggregated tables |

## Agent behavior

- Read `config.yaml` before adding a model or arm.
- After dependency changes, update the lockfile the repo already uses.
- If a shared dynamics/metrics library is documented, call it — do not fork
  the algorithm in this repo.

## Related

- `config-as-source.md`, `provenance.md`
- Skills: `config-check`, `eval-run`, `repro-check`
