---
name: write-prd
description: >
  Draft or revise a short product one-pager (problem, who, non-goals,
  constraints, open questions). Use for PRDs and specs. Do not invent
  metrics, dates, or “users said”. Triggers: write PRD, product spec,
  one-pager, problem statement, write-prd, /write-prd.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Write PRD

A one-pager another human can argue with. Product truth is `docs/prd/` plus
`product-truth.md`.

## Steps

1. Read existing `docs/prd/`, `docs/roadmap.md`, and the repo README.
2. Name the problem in one sentence. Who hurts, and what they do today.
3. List **non-goals**. If everything is in scope, the doc is not done.
4. Constraints that already exist (stack, data rights, one-harness, license).
5. Open questions — not fake answers.
6. Point `write-gherkin` at behavior once the problem is stable.

## Do not

- Invent metrics, conversion, quotes, or ship dates.
- Specify UI chrome or brand voice (that is `brand-voice` / `asset-brief`).
- Write a 12-page “vision”.

## Output

1. Suggested path (`docs/prd/<slug>.md`)
2. Full one-pager
3. Gaps the human must fill before it is truth
