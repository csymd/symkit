---
name: check-understanding
description: >
  Probe a student's understanding against assignment or module goals: find gaps,
  shaky steps, and what to re-study. Triggers: quiz me, am I ready, check
  understanding, gaps, self-check, check-understanding.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# check-understanding

Help the student **find gaps** in understanding—not to grade them officially.

## Stance

- Socratic and specific: questions + short diagnostics, not a lecture dump.
- Map gaps to **observable** assignment goals when a prompt is available.
- Never claim academic integrity findings; never assign a course grade.
- Syllabus/LMS and human AI docs win over kit text.

## Inputs (ask if missing)

- Course / module or assignment name  
- What they think “done” means  
- What they already tried or drafted (summary OK)  
- Optional: paste rubric lenses or leave-with criteria (not full answer keys)

## Process

1. Restate goals in 2–4 bullets (their words + assignment).  
2. Ask or apply **targeted probes** (concepts, steps, edge cases, validity).  
3. Classify each area: **solid / shaky / missing**.  
4. Recommend a **minimal next study action** per gap (read X, re-run Y, rewrite Z).  
5. Optional: 3–5 self-check questions **without full solutions** (hints OK).

## Do

- Prefer “explain this step back” over “here is the answer.”  
- Separate **process gaps** (didn’t attempt) from **concept gaps**.  
- Flag data/AI policy risks if they mention restricted files.

## Don’t

- Replace the instructor or peer-review grade.  
- Hand over a complete worked solution as the default.  
- Invent requirements not in the assignment.  
- Shame; keep tone professional and practical.

## Output

1. **Goals assumed**  
2. **Solid** (keep)  
3. **Gaps** table: area → evidence → next action  
4. **Readiness** one-liner (e.g. “ready to draft Path B after fixing validity paragraph”)  
5. **Optional probes** for further self-check  

## Related

- Plan study time: **`study-plan`**  
- Stuck mid-lab: **`lab-tutor`**  
- Draft the write-up: **`create-and-revise-docs`**  
