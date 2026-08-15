---
name: program-check
description: >
  Check a written training or testing program for internal consistency
  (volume, recovery, test timing). Use before a block starts. Does not
  write a new periodization model. Triggers: program check, periodization
  QA, is this plan coherent, program-check, /program-check.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Program check

Consistency of what is **already written**. Not a new mesocycle.

## Steps

1. Read `docs/program/` (and tests/calendar if present).
2. Map days: high neural / high metabolic / stated rest.
3. Flag collisions (two max-intent days with no written recovery, test on
   a stated high-fatigue day).
4. Flag missing pieces the program refers to but does not define.
5. Do not fill holes with a borrowed template (5/3/1, block, undulating)
   unless the human asks to *compare* to a named model.

## Do not

- Invent weekly volume or “they need a deload in week 4”.
- Diagnose overtraining.

## Output

| Item | Status | Evidence (path / quote) |
|:-----|:-------|:------------------------|
| | ok / conflict / missing | |

Most important gap to fix first.
