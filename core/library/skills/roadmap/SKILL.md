---
name: roadmap
description: >
  Build or refresh a Now / Next / Later list from the written PRD and the
  actual repo. Dates only if already written. Triggers: roadmap, now next
  later, prioritize, what ships next, roadmap, /roadmap.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Roadmap

Now / next / later. Source: `docs/prd/`, `docs/roadmap.md`, and git — not
hope.

## Steps

1. Read `docs/roadmap.md` and current PRDs.
2. Scan the repo for what already exists (do not list vapor).
3. Put each item in **Now**, **Next**, or **Later** with a one-line why.
4. Keep dates only when a doc already has them.
5. Flag conflicts (PRD says X, tree does Y).

## Do not

- Invent quarters, headcount, or “we will hit 10k users”.
- Promote Later → Now to make the list look ambitious.
- Treat this file as a commit calendar.

## Output

- Updated Now / Next / Later table
- Path: `docs/roadmap.md` unless the repo already uses another
- Unresolved priority questions
