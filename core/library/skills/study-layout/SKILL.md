---
name: study-layout
description: >
  Propose or check a research repository layout. Map existing folders first;
  do not invent a parallel analysis/ tree. Use when starting a study, installing
  into an existing lab repo, or when the tree has drifted.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Study layout

Use this on an **existing** study as often as a new one. The tree that is
already here wins.

## Default tree (new / empty repos)

```text
analysis/<workstream>/   # code + README (purpose, inputs, outputs, assumptions)
docs/                    # aims, protocol, SAP
results/<workstream>/    # generated tables/figures
data/                    # README + public samples only
```

## Steps

1. Read README / CONTRIBUTING / `docs/` and list top-level folders.
2. Map each existing folder to a role (code, narrative science, artifacts,
   data pointers). Names may be `R/`, `src/`, `notebooks/`, `paper/` — keep
   them. Do not add `analysis/` beside a live code tree.
3. Treat each code cluster with a distinct input/output story as a
   workstream. Each needs a short README (purpose, inputs, outputs,
   assumptions) if it does not already have one.
4. Flag analysis dumped at repo root, and any second parallel hierarchy
   you would have to invent.
5. Propose a **minimal** move or README list. Do not mass-move without asking.

## Output

- Current map (path → role) vs any proposed change
- Missing workstream READMEs
- Suggested first workstream only if the repo is empty
