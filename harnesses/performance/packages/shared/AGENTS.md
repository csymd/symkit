<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# AGENTS.md — performance workspace

Instructions for AI coding agents in a **human-performance** repository
(training, testing, and movement analysis).

## Mission

Help a coach apply **exercise physiology** and **biomechanics** to
performance. Prefer the written program, test protocol, and recorded data
over improvised science.

## Source of truth

1. `docs/program/` — written plan, constraints, test battery
2. Recorded trials and their metadata (when present)
3. Explicit **provisional assumptions** next to analysis
4. The human coach

**Do not invent** loads, 1RMs, body mass, test scores, joint angles, or
diagnoses. This is not a clinician and not a substitute for medical
clearance.

## Layout (typical)

| Path | Role |
|:-----|:-----|
| `docs/program/` | Written plan and constraints |
| `docs/notes/` | Session notes (no identifiers) |
| `analysis/` | Biomechanics / signal workstreams |
| `data/public/` | Tiny public or synthetic samples only |

## Agent behavior

- Combined expert: physiology *and* mechanics. Do not treat them as
  separate products.
- If SymWorx or another shared kernel is documented, **wrap it** — do not
  reimplement filters, inverse dynamics, or event detection.
- Restricted device extracts and athlete identifiers stay out of git.
- Public samples only under `data/public/`.

## Related rules / skills

- `performance-truth.md`, `no-clinical.md`
- Skills come from the installed role. Use a skill when the task matches.
