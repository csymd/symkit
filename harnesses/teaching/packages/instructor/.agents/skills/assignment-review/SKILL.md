---
name: assignment-review
description: >
  Instructor-only review of an assignment or lab handout before students see it
  (clarity, rubric, AI/data safety, feasibility, alignment with objectives).
  Use when QA-ing assignment markdown, tightening a rubric, or pre-ship review.
  Not for grading student submissions (use evaluate-content) or weekly class
  prep (use week-plan).
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Assignment review (instructor only)

Structured **materials QA** for a single assignment, lab, or checkpoint
**before** it goes to students.

**Instructor package only.** This reviews the *handout*, not student work.

## When to use

- “Is this lab ready to publish?”
- Rubric too vague / too long
- AI or data rules missing or conflicting
- Alignment check: objectives ↔ tasks ↔ evidence of learning

## When not to use

| Need | Use instead |
|:-----|:------------|
| Score a student’s submission | `evaluate-content` |
| Plan the next class | `week-plan` |
| Dedicated accessibility pass | `accessibility-review` |
| Invent a whole new unit | `course-prep` then this skill on the draft |
| Ship/tag the course release | `release-materials` |

## Review dimensions

Score each: **Pass / Needs work / Blocker**, with evidence.

1. **Clarity & structure** — title, overview, “what you will turn in”
2. **Objectives ↔ tasks ↔ evidence**
3. **Rubric / success criteria** — observable, not “good effort” alone
4. **Feasibility** — time box, environment, dataset size
5. **Data & privacy** — public/synthetic in-repo only
6. **AI-use alignment** with `docs/ai-what-to-expect.md` and syllabus
7. **Submit path & logistics** — LMS (or documented channel) explicit
8. **Inclusion & tone**

## Workflow

1. Read the assignment and linked rubric/objectives.
2. Fill the dimension table.
3. List **Blockers** first.
4. List **High-value edits** (concrete rewrites).
5. Optional: patch draft snippets.
6. End with **Ship recommendation**: Ship / Ship with nits / Do not ship.

## Output template

```markdown
# Assignment review — <title or path>

## Context
- Paths:
- Objectives source:
- Intended student time:
- AI stance (from handout):

## Dimension scores
| Dimension | Rating | Notes |
|:----------|:-------|:------|
| Clarity & structure | | |
| Objectives ↔ tasks ↔ evidence | | |
| Rubric / success criteria | | |
| Feasibility | | |
| Data & privacy | | |
| AI-use alignment | | |
| Submit path & logistics | | |
| Inclusion & tone | | |

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

- Grading individuals
- Integrity investigations
- Auto-publishing to an LMS
- Embedding answer keys in student-facing paths
