---
name: write-abstract
description: >
  Draft or revise a scientific abstract (structured or unstructured) aligned
  with the manuscript’s actual claims and limits. Use with the researcher
  agent. Triggers: write abstract, structured abstract, revise abstract,
  summary for submission, write-abstract, /write-abstract.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Write abstract

A tight, honest summary of work that exists — not a marketing blurb.

Prefer composing after `write-manuscript` has a stable Results/Discussion
story.

## Scope

**In:** unstructured or structured abstracts; conference short abstracts;
lay summary **only if asked** (label it separately).

**Out:** full paper draft, citation audit, peer review.

## Stance

- Must **not claim more** than the body (or available results).
- No invented metrics, p-values, or “first ever” claims without support.
- Respect word limits if the user gives a venue limit.

## Structure

**Default unstructured:** Background → objective → methods (brief) → key
results → conclusion/limits.

**Structured** (when asked or the venue requires): Background / Objective /
Methods / Results / Conclusions. Use venue field names if provided.

## Do

1. Confirm word/character limit and structured vs unstructured.
2. Pull only claims supported by the provided manuscript or data summary.
3. Prefer one primary result + one limit or implication.
4. Avoid undefined acronyms on first use.
5. Offer 2 variants if helpful (conservative vs slightly punchier) — never hype.

## Don't

- Promise clinical action without evidence.
- Cite references inside the abstract unless the venue requires it.
- Stuff methods detail that belongs only in the paper.

## Output

1. **Constraints** — limit, type, audience
2. **Abstract text** (and structured fields if used)
3. **Word count**
4. **Claims to verify** against full text before submit
5. **Optional alt version** if useful
