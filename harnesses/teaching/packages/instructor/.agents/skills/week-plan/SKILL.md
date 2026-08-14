---
name: week-plan
description: >
  Instructor-only weekly prep plan from calendar, module/session cards, and
  existing materials. Use when planning the next class meeting or week,
  building a pre-class checklist, or aligning LMS items with the session.
  Not for full course design (use course-prep) or grading (use evaluate-content).
---

<!--
Copyright (c) 2026, cSYMd
Licensed under Apache 2.0
-->

# Week plan (instructor only)

Produce a **one-week / one-meeting prep pack** the instructor can follow
before class.

**Instructor package only.** Do not run as TA. Do not invent syllabus dates or
student lists.

## When to use

- “What do I need for this class meeting?”
- Align module card + calendar + assignments for one session
- Pre-flight checklist (materials, demo, LMS, data)

## When not to use

| Need | Use instead |
|:-----|:------------|
| Design a new module or lab from scratch | `course-prep` |
| Full assignment rewrite / deep handout QA | `assignment-review` |
| Grade student work | `evaluate-content` |
| Tag a student release | `release-materials` |

## Inputs (ask if missing)

- Course id
- Date or week / module or day id
- Meeting length
- Paths: calendar, module/session card, related assignments
- Constraints (no live demo, guest, exam week, etc.)

Prefer reading files in the course repo (`docs/course-calendar.md`,
`docs/modules/`, `assignments/`, `lectures/`) when available.

## Workflow

### 1. Anchor

- Session identity (module/day title)
- Objectives (from materials; mark gaps if missing)
- Student-facing prep they should have done
- Deliverables due soon (LMS vs in-class)

### 2. Timing skeleton

If the module card already has a table, **refine it**—do not replace without reason.

Default long-block sketch (adjust to actual length):

| Block | Minutes | Purpose |
|:------|--------:|:--------|
| Open / recap | 10–15 | Hook + link to last time |
| Core teach / demo | 40–50 | One main idea |
| Interactive | 40–50 | Students produce something visible |
| Apply / lab | 45–60 | Guided work |
| Close | 10–15 | Takeaways + next steps |

### 3. Prep checklist

- [ ] Slides / notes path or “use board only”
- [ ] Demo / data: public sample paths only; external restricted data noted
- [ ] Handouts / assignment links live as intended
- [ ] AI expectations linked if AI is in play
- [ ] Backup if demo or network fails
- [ ] Time buffer for 2–3 likely stuck points

### 4. Engagement beat

One concrete interactive move. State what students do, what “good” looks like,
and how you’ll sample understanding.

### 5. LMS / logistics

- Optional announcement draft (2–4 sentences)
- What is due this week vs next
- TA asks (if any)

### 6. Private instructor notes

Label clearly `INSTRUCTOR ONLY`: board plan, stretch if early, cut list if late.

## Output template

```markdown
# Week plan — <course> — <date or module>

## Anchor
- …
## Objectives
- …
## Timing
| Block | Min | Activity |
## Instructor prep checklist
- [ ] …
## Engagement beat
- …
## Student prep / deliverables
- …
## LMS / logistics
- …
## Likely stuck points
- …
## INSTRUCTOR ONLY notes
- …
## Open questions
- …
```

## Quality bar

- Feasible in the stated time box
- No restricted data in student paths
- Matches existing materials; flags contradictions with calendar/syllabus
- No fabricated guest names, readings, or due dates
