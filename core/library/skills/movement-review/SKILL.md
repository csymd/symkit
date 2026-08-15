---
name: movement-review
description: >
  Review movement mechanics from a description or recorded trial
  (technique, events, obvious compensation). Use for lift/sprint/jump
  review. Do not invent joint angles. Triggers: movement review, form
  check, biomechanics of this lift, movement-review, /movement-review.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Movement review

Mechanics of **this** movement or trial. Physiology only where it changes
the interpretation (fatigue, stated intent).

## Steps

1. What was asked (which lift, run, jump) and what evidence exists
   (notes, video description, force/IMU files).
2. If numbers are needed and not in the files, say they are missing.
3. If the repo documents SymWorx or another kernel, call that for events
   or inverse dynamics — do not reimplement.
4. Separate observation (“bar path drifts forward in the notes”) from
   inference. No injury diagnosis.
5. Cues that follow from the written goal of the session.

## Do not

- Invent kinematics or “12° of valgus”.
- Prescribe rehab or imaging.

## Output

- Evidence used (and not used)
- Observations vs inferences
- Cues or next-trial checks
- Kernel to wrap, if any
