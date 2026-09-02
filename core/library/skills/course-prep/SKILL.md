---
name: course-prep
description: >
  Instructor-only course preparation: session design, interactive engagement,
  assignment/lab drafting, private prep notes, and pre-release safety checks.
  Use when the instructor is planning or building upcoming class content.
  Not for TAs. Not for grading student submissions (use evaluate-content).
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Course preparation (instructor only)

You help the **instructor of record** prepare course experiences that are
clear, engaging, and safe to ship.

**Instructor package only.** Do not run this skill in a TA role. Do not commit
private prep or answer keys to student-facing trees.

## When to use

- Designing a class session (goals, timing blocks, activities)
- Drafting or revising labs, checks, discussion prompts
- Planning engagement (gallery walks, critique cards, pair work)
- Instructor-only notes (pacing, board plan, common stuck points)
- Pre-flight before students see materials (with `release-materials` when shipping)

## When not to use

| Need | Use instead |
|:-----|:------------|
| Grade / feedback on a submission | `evaluate-content` |
| Discuss a specific student’s engagement | engagement norms; keep non-identifying |
| Student AI handout | edit `docs/ai-what-to-expect.md` (learner pack) |
| TA workflow | stop — TAs do not run course-prep |

## Inputs to gather (if missing)

- Course id and session/module
- Time box
- Learning objectives (`docs/slos.md` or `documents/slos.md` when present; do not invent SLOs)
- What already exists in the repo (`docs/modules/`, `lectures/`, `assignments/`)
- Constraints (no new tools, public data only, LMS deliverable, etc.)

## Workflow

### 1. Align with the course tree

- Prefer existing templates (module cards, day plans, assignment layout).
- Link LMS vs git boundaries: materials in repo; graded work on the LMS unless
  documented otherwise.
- Call out data rules: public samples only in-repo; restricted data external.

### 2. Session design (default skeleton)

1. Objectives (3 or fewer primary)
2. Timing table (block, minutes, activity)
3. Interactive beat (what students *do*, not only hear)
4. Checks for understanding
5. Deliverable / next step
6. Faculty notes (private): pitfalls, board plan, stretch goals

### 3. Materials drafting

- Short, accurate drafts over long generated lectures.
- Explicit prerequisites and success criteria.
- AI-use: point students at `docs/ai-what-to-expect.md` (or syllabus); do not
  invent policy.

### 4. Safety before ship

- [ ] No secrets, credentials, or restricted data in student paths
- [ ] No answer keys in student-facing assignment dirs unless intentional and gated
- [ ] Links and paths resolve
- [ ] Domain/policy claims are checkable—no fabricated citations
- [ ] If releasing: follow `release-materials`

## Output formats

1. **Session card** — markdown matching the course’s module/day style
2. **Assignment stub** — title, objectives, tasks, rubric sketch, submit path
3. **Private prep note** — clearly labeled `INSTRUCTOR ONLY` for local use

## Out of scope

- Finalizing grades or bulk evaluation of submissions
- Changing institutional policy
- Committing staff-only packs into `main`
- TA delegation of course design without instructor direction
