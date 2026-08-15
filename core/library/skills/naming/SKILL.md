---
name: naming
description: >
  Product or feature naming: shortlist, occupancy, and daily ergonomics
  (say once, cargo install, search). Use before publishing a crate, CLI, or
  brand name. Triggers: name this, rename, naming occupancy, crates.io name,
  naming, /naming.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Naming

Pick a name people can say and type. Occupancy is a filter, not the goal.

## Steps

1. One sentence: what the thing *is* (kit, app, CLI, feature).
2. Constraints already written (`docs/brand/`, family prefix, crate vs binary).
3. Shortlist 3–7 names. Kill phonetic twins of huge companies and drugs.
4. Check occupancy: crates.io / npm / PyPI as relevant, GitHub, `buy <name>`,
   spoken collision.
5. Score **ergonomics** (guessable install, one-beat explanation) vs
   **uniqueness** (search). Say which you are optimizing.
6. Recommend **one** publish name. Do not ship two live names.

## Do not

- Park empty registry packages (name squatting).
- Rename the GitHub repo in the same breath unless asked.
- Treat a free crates.io slug as proof the name is good.

## Output

- Constraint recap
- Table: name / occupancy / ergonomics / notes
- One recommendation and why
- What you did not check
