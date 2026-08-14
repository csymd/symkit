---
name: config-check
description: >
  Check experiment config against scripts and documented phases. Use when
  adding a model, arm, or run_phase, or when a run failed to pick up config.
---

<!--
Copyright (c) 2026, cSYMd
Licensed under Apache 2.0
-->

# Config check

## Steps

1. Read the repo’s config file (usually `config.yaml`).
2. List models, arms, seeds, and phase tags.
3. Confirm scripts read those fields (no leftover hardcoded names).
4. Flag missing seeds, duplicate ids, or undocumented phases.
5. Do not add models that are not requested.

## Output

- Summary table of config entries
- Gaps (script vs config)
- Suggested config snippet only if the user asked to add something
