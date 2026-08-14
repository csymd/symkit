---
name: response-to-reviewers
description: >
  Draft a response-to-reviewers / revision letter for scientific manuscripts:
  map each comment to a reply, manuscript change, or reasoned disagreement.
  Use with the researcher agent. Triggers: response to reviewers, R&R letter,
  revision response, rebuttal letter, response-to-reviewers,
  /response-to-reviewers.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Response to reviewers

A diplomatic, precise R&R letter — not a second manuscript draft.

Pair with `write-manuscript` for body edits and `check-citations` when refs
are contested.

## Inputs needed

- Reviewer comments (paste or file)
- Manuscript version context (what changed or will change)
- Any constraints (page limits, due date, coauthor decisions)

If comments are incomplete, structure what you can and list missing pieces.

## Stance

- Thank, address **every** point, stay factual and calm.
- Prefer **change + location** (“we revised §2.1, p.4”) over vague agreement.
- Disagreement is OK when reasoned and evidenced — not dismissive.
- Do not invent new analyses or citations to “win”; mark proposed work as
  planned or done only if the user confirms.

## Letter structure

1. **Cover note** — thanks, summary of major changes (short).
2. **Point-by-point** — for each reviewer/editor item:
   - Restate comment (brief)
   - Response
   - Manuscript change (quote or paraphrase + section) **or** justification if no change
3. Optional **additional changes** not requested but made.

## Don't

- Attack reviewers.
- Claim changes that were not made.
- Ignore hard comments.
- Paste the entire revised paper into the letter.

## Output

```markdown
## Cover note
…

## Response to Editor
…

## Response to Reviewer 1
### Comment 1.1
**Comment:** …
**Response:** …
**Changes:** … (section / “none — rationale”)

## Open items for authors
- Decisions still needed
- Analyses not yet run
```
