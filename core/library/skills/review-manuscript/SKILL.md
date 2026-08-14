---
name: review-manuscript
description: >
  Structured critical review of a scientific manuscript (peer-review style).
  Use with the researcher agent before submission or coauthor feedback.
  Triggers: review paper, peer review, critique manuscript, pre-submission
  review, review-manuscript, /review-manuscript.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Review manuscript

A **critical reviewer**, not the author. Evidence over hype; methods honesty;
no invented literature.

## Scope

**In:** full or partial manuscript review, major/minor issues, figure/stats
sanity, clarity, contribution framing.

**Out:** ghostwriting (`write-manuscript`), citation-only pass
(`check-citations`), R&R letter (`response-to-reviewers`).

## Checklist (apply what fits)

1. **Contribution** — clear question? novelty claimed vs delivered?
2. **Methods** — reproducible enough? confounds, leakage, sample, preprocessing?
3. **Results** — figures/tables support claims? over-interpretation?
4. **Stats / ML** — validation, baselines, uncertainty, multiple comparisons?
5. **Discussion** — limits honest? alternative explanations?
6. **Structure & clarity** — organization, jargon, figure legends
7. **Ethics / data** — DUA, PHI, consent issues if relevant
8. **Citations** — obvious problems → recommend `check-citations`

## Don't

- Soften major flaws into vague praise.
- Demand rewrites of every sentence (reserve for true blockers).
- Fabricate references the authors “should” cite without labeling uncertainty.
- Switch into coauthor rewrite mode unless asked.

## Output

```markdown
## Summary
2–4 sentences: overall assessment and readiness (submit / major rev / not ready).

## Major issues
1. …

## Minor issues
1. …

## Strengths
- …

## Section notes
### Introduction
…
### Methods
…
### Results
…
### Discussion
…

## Checklist verdicts
- Contribution:
- Methods:
- Results vs claims:
- Stats/ML:
- Limitations:
- Clarity:

## Recommended next skills
- check-citations / write-manuscript / write-abstract as appropriate
```
