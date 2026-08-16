<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# AGENTS.md — research workspace

Instructions for AI coding agents in a **research / study** repository.

## Mission

Help design, implement, and document analyses so another researcher can
reproduce them. Prefer the study’s written aims, protocol, or SAP over
improvised science.

## Source of scientific truth

1. Documents in `docs/` (aims, protocol, SAP) when present
2. Explicit assumption blocks next to analysis code when the aims leave
   parameters open
3. The human running the study

**Do not invent** effect sizes, primary endpoints, sample sizes, or design
changes without labeling them as **provisional assumptions**. Quote or cite
the aims/SAP when restating design facts.

## Layout

If this repo already has a tree, **map workstreams onto it**. Do not add a
parallel `analysis/` next to live `R/`, `src/`, or `notebooks/`.

Typical names when you are starting empty:

| Path | Role |
|:-----|:-----|
| `analysis/` | Code, workstream READMEs |
| `docs/` | Narrative science (aims, protocol, SAP) |
| `results/` | Generated artifacts (no huge binaries without asking) |
| `data/` | Pointers and tiny public samples — not restricted extracts |

## Agent behavior

- Prefer one directory per workstream with a short README (purpose, inputs,
  outputs, assumptions) — under `analysis/` only when that is the existing
  convention.
- Do not dump analysis into a root `main.py`.
- Do not create empty publication trees unless asked.
- Keep pre-commit hooks fast.
- If a shared methods library is documented (e.g. SymWorx), wrap it — do not
  reimplement algorithms that already exist there.

## Related rules / skills

- `aims-as-truth.md`
- `study-layout` — propose or check folder conventions
- `repro-check` — env lock, how-to-run, artifact paths
- `write-gherkin` — pipeline specs as observables
- `check-citations` — claim ↔ source integrity; never invent papers or DOIs
- manuscript suite when assigned: `lit-matrix`, `write-manuscript`,
  `write-abstract`, `review-manuscript`, `response-to-reviewers`
