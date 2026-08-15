---
name: check-citations
description: >
  Audit citation and reference integrity in a manuscript or course materials:
  claims needing sources, unsupported cites, in-text vs bibliography mismatch,
  and style consistency. Triggers: check citations, citation audit, missing
  references, cite check, check-citations, /check-citations.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Check citations

Claim ↔ source integrity. Not prose style.

Compose with the active agent:

| Agent | Typical input |
|:------|:--------------|
| `researcher` | Manuscript, protocol, related-work notes |
| `instructor` / `ta` / `materials-author` | Handouts, lectures, assignment prompts |
| `learner` | The student's own draft only |

## Hard rules

- **Never invent** papers, DOIs, URLs, quotes, or page numbers.
- If you cannot verify a source from available files or context, mark
  **UNVERIFIED** or **NEEDS SOURCE**.
- Prefer honest gaps over a polished fake reference list.

## What to check

1. **Unsupported claims** — non-obvious factual or literature claims without a cite
2. **Weak coupling** — citation present but does not support the sentence
3. **Orphans** — in-text cite missing from the reference list (or `.bib`)
4. **Unused refs** — bibliography entries never cited (flag; may be intentional)
5. **Consistency** — years, author spelling, keys, numbering vs author-year
6. **Style** — if the user names APA / Vancouver / BibTeX / a journal style, check shape
7. **Balance** (only if asked) — recency, self-cite density, missing classics

## Inputs (use what is available)

- Manuscript or course text (path or paste)
- `.bib` / reference-manager export
- PDF library or notes **only if provided**

State what you could not access.

## Don't

- Pad the bibliography with “standard” papers you invent.
- Treat citation count as quality.
- Rewrite the whole document.
- Deliver a full peer review or assignment grade.

## Output

```markdown
## Scope
Files/sections audited; styles assumed; what could not be verified.

## Blockers
1. Claim / location — issue — suggested fix (add cite / rephrase / verify)

## Should-fix
1. …

## Nits
1. … (style only)

## Unverified
Items that need a human library check.

## Summary table
| Location | Claim (short) | Status | Note |
|----------|---------------|--------|------|
| … | … | OK / MISSING / WEAK / ORPHAN / UNVERIFIED | … |

## Verdict
**PASS** | **PASS WITH GAPS** | **FAIL** — one sentence.
```
