---
name: eval-run
description: >
  Log a model evaluation run with provenance: command, config, seed, outputs.
  Triggers: run eval, log a run, eval-run, how should I record this experiment.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Eval run

## Record

```text
model:
config:          # path + relevant keys
seed:
command:
started:
outputs:         # metrics path, run dir
notes:
```

## Steps

1. Confirm config entry exists (`config-check` if not).
2. Name the output directory from model + arm + seed (match repo convention).
3. Write metrics next to that convention (`metrics/` or per-run files).
4. Do not overwrite a previous run without asking.
5. Do not invent metric values.

## Output

A ready-to-paste run log and the exact command the human should execute.
