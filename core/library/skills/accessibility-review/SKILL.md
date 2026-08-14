---
name: accessibility-review
description: >
  Instructor-only accessibility review of course documents before students
  see them (headings, contrast, emphasis, links, images, language). Use when
  QA-ing a handout, slide deck, rubric, lab, or syllabus excerpt for
  readability and access. Not for grading (use evaluate-content), weekly
  prep (use week-plan), or a full assignment rewrite (use assignment-review).
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Accessibility review (instructor only)

Structured **document accessibility QA** for materials students will read or
submit against.

**Instructor package only.** This reviews the *artifact*, not a student, and
is **not** a legal ADA/Section 504 determination or a substitute for official
disability-services accommodations.

## When to use

- “Can students actually use this handout / deck / rubric?”
- Headings are missing, skipped, or faked with bold
- Color is the only way to see importance or status
- Images, screenshots, or plots carry meaning with no text equivalent
- Language is dense, undefined, or hard to scan

## When not to use

| Need | Use instead |
|:-----|:------------|
| Full assignment QA (rubric, AI, data, feasibility) | `assignment-review` |
| Score a student’s submission | `evaluate-content` |
| Plan the next class | `week-plan` |
| Design a new unit | `course-prep` |
| Decide an individual accommodation | published syllabus + campus disability services |

Run this **after** or **beside** `assignment-review`. Inclusion/tone there is
not a substitute for this pass.

## Review dimensions

Score each: **Pass / Needs work / Blocker**, with evidence (path + quote or
line).

1. **Headings & structure** — one title (`#`); real headings (`##` / `###`),
   not bold-as-heading; no skipped levels; sections a screen reader or
   outline can walk
2. **Emphasis & color** — importance via wording + **bold** (or a heading),
   never color alone; if color is used, contrast must stay readable on a
   typical display and in grayscale
3. **Links, lists, tables** — link text describes the target (not “click
   here”); lists for sequences; tables have a header row and are not used
   for page layout
4. **Images & media** — meaningful figures have alt text or an adjacent
   caption; no essential step lives only in a screenshot; AV has a caption
   or transcript pointer
5. **Language & cognitive load** — short sentences; acronyms defined once;
   consistent terms; time box and “what to turn in” easy to find
6. **Code, math, data** — copy-pasteable text, not only images of code;
   commands in fenced blocks; large tables offered as a file when needed
7. **File & format** — prefer markdown/HTML source over scanned PDF; if a
   PDF/slide deck ships, it still has a heading outline and selectable text

## Workflow

1. Identify the artifact(s) and who reads them (all students vs a staff
   note — staff-only files are out of scope unless they will be pasted into
   student-facing channels).
2. Skim the outline (heading list) before the prose.
3. Fill the dimension table.
4. List **Blockers** first (cannot complete the task, or meaning is
   color/image-only).
5. List **High-value edits** as concrete rewrites, not “make it accessible.”
6. Optional: patch heading/alt/link snippets in the draft.
7. End with **Ship recommendation**: Ship / Ship with nits / Do not ship.

## Checks (apply what exists)

- [ ] Title is a heading, not a bold paragraph
- [ ] Heading levels go `#` → `##` → `###` without jumps
- [ ] Color is never the only signal (status, required vs optional, key)
- [ ] Colored headings/callouts still read if color is removed
- [ ] Links make sense out of context
- [ ] Images that teach have alt or a caption that carries the same fact
- [ ] No “see the red box / the figure” without a named referent
- [ ] Rubric or submit path is in text, not only a screenshot of the LMS

## Output template

```markdown
# Accessibility review — <title or path>

## Context
- Paths:
- Audience (student-facing?):
- Formats (md / slides / pdf / notebook):

## Dimension scores
| Dimension | Rating | Notes |
|:----------|:-------|:------|
| Headings & structure | | |
| Emphasis & color | | |
| Links, lists, tables | | |
| Images & media | | |
| Language & cognitive load | | |
| Code, math, data | | |
| File & format | | |

## Blockers
1. …

## High-value edits
1. …

## Ship recommendation
- Ship | Ship with nits | Do not ship

## Open questions for instructor
- …
```

## Out of scope

- Approving or denying an individual accommodation request
- Claiming WCAG conformance or “ADA compliant”
- Grading people
- Auto-publishing to an LMS
- Rewriting the whole assignment (hand that to `assignment-review` / `course-prep`)
