---
name: study-layout
description: >
  Propose or check a research repository layout (analysis workstreams, docs,
  results, data pointers). Use when starting a study repo or when the tree
  has drifted.
---

<!--
Copyright (c) 2026, cSYMd
Licensed under Apache 2.0
-->

# Study layout

## Default tree

```text
analysis/<workstream>/   # code + README (purpose, inputs, outputs, assumptions)
docs/                    # aims, protocol, SAP
results/<workstream>/    # generated tables/figures
data/                    # README + public samples only
```

## Steps

1. Read existing README / CONTRIBUTING / `docs/`.
2. Map current folders to the default tree; do not invent parallel hierarchies.
3. For each workstream, require a short README.
4. Flag analysis dumped at repo root.
5. Propose a minimal move list — do not mass-move without asking.

## Output

- Current vs proposed layout
- Missing READMEs
- Suggested first workstream if the repo is empty
