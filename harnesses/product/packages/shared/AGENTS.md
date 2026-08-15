<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# AGENTS.md — product workspace

Instructions for AI coding agents in a **product** repository (specs and
shipping notes). Brand and voice live in the **creative** harness.

## Mission

Help frame problems and keep a now/next/later list honest. Prefer documents
in this repo over improvised strategy.

## Source of product truth

1. `docs/prd/` — problem, who, non-goals
2. `docs/roadmap.md` — now / next / later
3. The human who owns the product

**Do not invent** metrics, conversion, “users said”, ship dates, or
headcount. Label guesses as **provisional assumptions**.

## Layout (typical)

| Path | Role |
|:-----|:-----|
| `docs/prd/` | One-pagers and specs |
| `docs/roadmap.md` | Now / next / later |
| `features/` | Product behavior as Gherkin |

## Agent behavior

- Read PRD + roadmap before proposing scope.
- Prefer `write-gherkin` for checkable behavior over slide-ware requirements.
- Voice and naming belong to the `creative` harness. Do not invent a brand
  system here.
- If a shared kernel is documented (e.g. SymWorx), wrap it — do not fork it.

## Related rules / skills

- `product-truth.md`
- Skills come from the installed role (`catalog.yaml`).
