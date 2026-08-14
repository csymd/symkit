---
name: lit-matrix
description: >
  Build a structured literature matrix (comparison table) of papers: design,
  sample, methods, findings, limits, and relevance to a research question.
  Use with the researcher agent for related-work synthesis. Triggers: lit
  matrix, literature table, related work table, paper comparison matrix,
  lit-matrix, /lit-matrix.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Literature matrix

A **structured evidence table**, not a narrative review essay (use
`write-manuscript` for Related Work prose).

## Hard rules

- **Never invent** studies, sample sizes, or findings.
- If metadata is missing, use **?** or **not reported** — do not guess.
- If the user lists papers without PDFs, extract only from provided text/bib
  and flag thin rows.

## Default columns

| Column | Content |
|:-------|:--------|
| Cite key / short cite | Author year or bib key |
| Year | |
| Domain / population | |
| Design | e.g. observational, RCT, sim, secondary data |
| N / sample | as reported |
| Signals / data | |
| Methods / model | |
| Primary findings | 1–2 lines |
| Limits (authors’ or yours) | |
| Relevance to RQ | high / med / low + why |
| Notes | open questions |

Add/drop columns for ML metrics, sensors, or clinical outcomes as needed.

## Don't

- Write a multi-page narrative instead of a table.
- Homogenize conflicting findings into false consensus.
- Cite papers you cannot support from available sources.

## Output

1. **RQ / inclusion notes**
2. **Matrix** (markdown table; CSV block if many rows and the user wants it)
3. **Themes** — 3–6 bullets synthesizing patterns (no new fake cites)
4. **Gaps** — what the matrix does not cover
5. **Next** — papers to retrieve, or sections for write-manuscript
