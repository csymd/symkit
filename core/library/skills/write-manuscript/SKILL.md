---
name: write-manuscript
description: >
  Draft or revise scientific manuscript prose (IMRaD and related sections).
  Use with the researcher agent for papers, theses, methods reports, or figure
  legends. Triggers: write manuscript, draft paper, revise Introduction,
  Methods, Results, Discussion, write-manuscript, /write-manuscript.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Write manuscript

Scientific stance lives in the **`researcher`** agent and `aims-as-truth`
(measurement vs model vs interpretation; no invented endpoints, n, or
results). Apply that here; do not invent a second epistemology.

## Scope

**In:** outlines, full or partial drafts, section rewrites, figure/table
legends, transitions, hedging language, structure cleanup.

**Out:** peer-review report (`review-manuscript`), citation-only audit
(`check-citations`), abstract-only (`write-abstract`), R&R letter
(`response-to-reviewers`), literature tables (`lit-matrix`).

## Do

1. Confirm target section(s) and audience (journal, committee, preprint).
2. Prefer IMRaD (or the user’s template) unless the project uses another structure.
3. Separate what was measured, what was modeled, and what is interpreted.
4. Use consistent terminology for signals, cohorts, and metrics.
5. Keep figures/tables referenced honestly; note missing panels as TODOs.
6. Hedge when causal language is unwarranted.
7. After a large draft, suggest `check-citations` and optionally `write-abstract`.

## Don't

- Fabricate results or “typical” numbers to fill gaps.
- Invent papers, statistics, or quotes.
- Inflate contribution or bury limitations.
- Duplicate software README style (`write-docs` is for code docs, when present).

## Output

1. **Scope** — sections touched, venue if known
2. **Manuscript text** — ready to paste or write to a path
3. **Open gaps** — missing data, undecided analysis, uncited claims
4. **Next** — e.g. check-citations, write-abstract, review-manuscript
