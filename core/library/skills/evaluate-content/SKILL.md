---
name: evaluate-content
description: >
  Structured evaluation of student submissions or course assignment materials.
  Use when the instructor asks to evaluate, grade formatively, review a
  lab/project submission, assess prompting quality, or QA an assignment
  handout/rubric. Instructor-only; do not use for student-facing auto-grading.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Evaluate content, understanding & materials

You assist the **instructor** (or a TA drafting for the instructor) with
structured evaluation. The **instructor owns all grades** and any message
sent to students.

**Faculty pack only.** Do not commit this skill, generated feedback, or
student artifacts into student-facing course trees.

## Modes

Pick one per request. If unclear, ask once before deep review.

| Mode | Evaluate |
|:-----|:---------|
| **Student submission** | Code, notebook, report/write-up, optional AI prompt log |
| **Materials QA** | Assignment handout, rubric, starter code in the materials repo |

## Prerequisites

Before writing scores or `FEEDBACK.md`:

- [ ] Assignment objectives / prompt (or materials under review)
- [ ] Rubric or success criteria — if none: **formative feedback only, no points/letter grades**
- [ ] Artifacts to review (paths or pasted content the instructor provided)
- [ ] Whether an AI **prompt log** is in scope

**Do not invent** missing rubrics, students, chat histories, results, or quotes.

## Privacy & integrity

- Prefer non-identifying labels (`Student A`, team id) unless the instructor
  gives a preferred label for a local file.
- Do not put real student identifiers into paths or files that might be committed.
- Possible integrity issues = **hypotheses to verify**, not conclusions.
- Align tone with faculty engagement norms: teach, don’t mock; support first,
  accountability second.
- Never encourage pasting restricted data into tools; flag if submission
  evidence suggests that risk.

## Coding and programming (if applicable)

- Clear file / notebook structure; sensible naming
- Comments: needed vs noise; missing “why” where non-obvious
- Documentation: clarity; consistent inclusion/exclusion across the project
- Reproducibility signals when relevant (seeds, environment notes, how to run)
- **N/A** if there is no code

## Prompting (if a log is provided)

- Concision and clarity; suggest cleaner wording where useful
- Conflicting or ambiguous instructions in the prompt(s)
- Thoroughness vs thrash
- Alignment with course AI expectations when `docs/ai-what-to-expect.md`
  (or syllabus AI section) is available
- **N/A** if no prompt log — do not invent one

## Content / understanding

- Method fit for the **stated** learning objective
- Interpretation aligned with evidence
- Limitations, validation story, or data-ethics notes when the assignment warrants them

## Materials QA extras (mode 2 only)

- Are objectives testable and matched to the rubric?
- Is AI-use guidance clear and consistent with course docs?
- Public/sample data only in handouts; restricted-data paths documented externally?
- Starter code/docs consistent with what students are asked to produce?

For a full pre-ship assignment pass, prefer `assignment-review`.

## Out of scope

- Final course grades or letter grades without a rubric mapping
- Integrity **verdicts** or misconduct case write-ups
- Full rewrite of the submission as an answer key unless the instructor asks
- FERPA-sensitive detail in anything that might land in public git

## Providing feedback

Generate a **local** feedback file (default `FEEDBACK.md` in the working tree,
or a path the instructor names). Suggest a non-identifying name if multiple
reviews (`FEEDBACK-student-a.md`).

### Required structure

```markdown
# Feedback

## Context
- Mode:
- Assignment / materials:
- Artifacts reviewed:
- Rubric available: yes/no

## Coding and programming
…

## Prompting
… (or N/A)

## Content / understanding
…

## Summary
- …

## Suggested next steps
1. …

## Open questions for the instructor
- …
```

For each applicable section: **strengths**, **issues** with evidence, **actionable fix**.
Keep each section roughly one screen.

## Output checklist

- [ ] Mode stated
- [ ] N/A used where a dimension does not apply
- [ ] No invented rubric scores
- [ ] Feedback file path is local / instructor-chosen
- [ ] Integrity only as hypotheses, if at all
