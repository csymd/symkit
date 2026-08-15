---
name: accessibility-review
description: >
  Accessibility QA of student- or user-facing artifacts (documents, slides,
  multimedia) before they ship. Headings, contrast, links, images, captions,
  transcripts, math, tables. Use for handouts, decks, labs, product docs, or
  public copy. Not a legal ADA/504/WCAG determination. Not grading
  (evaluate-content), week-plan, or a full assignment rewrite
  (assignment-review). Triggers: accessibility review, captions, alt text,
  slide reading order, WCAG pass, accessibility-review, /accessibility-review.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Accessibility review

Structured **artifact QA** for materials people will read, watch, or submit
against. Reviews the *file*, not a person.

This is **not** a legal ADA, Section 504, or WCAG determination, and **not**
a substitute for official disability-services accommodations.

## Mode (pick one; ask once if unclear)

Apply only that mode’s extra checks. **Mixed** = one dimension table, plus
the extra lists that match each file.

| Mode | Typical files | Extra checks |
|:-----|:--------------|:-------------|
| **Document** | Markdown, Word, Google Doc, tagged PDF, notebook | Styles, inline images, data tables, built-in checker |
| **Presentation** | Slides | Unique titles, built-in layouts, ≥18 pt, reading order |
| **Multimedia** | Pre-recorded video / audio | Captions, transcript, player keyboard, flashing, AD |
| **Mixed** | A module with notes + deck + clip | All that apply, named per path |

## When to use

- “Can someone actually use this handout / deck / clip / product page?”
- Headings skipped or faked with bold
- Color is the only status signal
- Meaning lives only in an image, screenshot, or unspoken visual
- Equations are pictures; tables are layout grids
- Video has no captions or transcript

## When not to use

| Need | Use instead |
|:-----|:------------|
| Full assignment QA (rubric, AI, data, feasibility) | `assignment-review` |
| Score a student’s submission | `evaluate-content` |
| Plan the next class | `week-plan` |
| Design a new unit | `course-prep` |
| Copy voice / naming only | `critique-copy` / `brand-voice` |
| Decide an individual accommodation | published syllabus + the office the *course* names |

## Review dimensions

Score each: **Pass / Needs work / Blocker / N/A**, with evidence (path +
quote or line). Skip rows that the mode does not use.

1. **Headings & structure** — real heading styles or `#` / `##` (not
   bold-as-heading); no skipped levels; one title. **Presentation:** every
   slide has a **unique** title; built-in slide layouts; logical reading
   order. Do not fake layout with empty paragraphs, tabs, or space-runs.
2. **Emphasis, color & type** — never color alone; contrast still works in
   grayscale. **Presentation:** body text at least **18 pt**. Prefer a
   readable sans or the file’s documented accessible font — do not invent a
   campus type list.
3. **Links, lists, tables** — link text describes the target (not “click
   here”). Lists for sequences. **Tables are data only:** header row (and
   row headers when needed); no merged cells; no blank rows used for
   spacing. Spreadsheets: a real header row.
4. **Images & complex visuals** — equivalent alt (purpose + message);
   decorative marked empty; no essential step only in a screenshot. **Docs:**
   images in line with text, not floating. Infographics need a long
   description or adjacent caption that carries the same facts.
5. **Multimedia** — **Captions** (Level A): present, synced, throughout,
   speaker identified, readable contrast, enough time on screen. **Transcript**
   (Level A): speech + relevant non-speech + on-screen text not spoken;
   speakers named; accessible format (text/Doc); if on another page, a way
   back. **Player:** keyboard pause/seek/volume; no surprise autoplay (or
   stoppable at start); no flashing. **Audio description** (Level AA) when
   visuals are not spoken — or a written plan if the course treats AD as
   an accommodation to prepare. Point at `docs/` if the target already
   names a captioning tool.
6. **Language & cognitive load** — short sentences; acronyms defined once;
   time box and “what to turn in” in text.
7. **Code, math, data** — copy-pasteable text, not images of code.
   **Equations** as real math (LaTeX, Equation Editor, MathML) — not a
   screenshot of a formula. Large tables also offered as a file when needed.
8. **File & format** — prefer markdown/HTML/tagged Word over a scanned PDF.
   If a PDF or deck ships, it has a heading/outline and selectable text.
   **Run the file’s own accessibility checker** (Word, Google, PowerPoint,
   Acrobat, Canvas RCE, etc.) when one exists. Report what it flagged vs
   what this pass found. “Checker not available” is valid.

## Workflow

1. Identify artifacts, audience (student-facing / public / staff-only), and
   **mode**. Staff-only files are out of scope unless they will be pasted
   into a student-facing channel.
2. If the **target repo** already names a WCAG level or campus deadline,
   quote that path. Do not invent a university or a date.
3. Skim the outline (headings or slide titles) first.
4. Fill the dimension table. Apply only the extra checks for the mode.
5. **Blockers** first (cannot complete the task; meaning is color- or
   image-only; required captions/transcript missing).
6. **High-value edits** as concrete rewrites, not “make it accessible.”
7. Optional: patch heading/alt/link/caption snippets.
8. **Ship recommendation**: Ship / Ship with nits / Do not ship.

## Honesty

- Do **not** write “WCAG 2.1 AA”, “Title II ready”, or “ADA compliant.”
- Do **not** approve or deny an accommodation letter.
- If you cannot open the media (no captions file, no audio), mark those
  rows **unknown** and say what is missing — do not invent a pass.

## Output template

```markdown
# Accessibility review — <title or path>

## Context
- Mode: document | presentation | multimedia | mixed
- Paths:
- Audience (student-facing / public?):
- Formats:
- Checker: ran <tool> | not available | n/a
- Target-stated bar (quote path, or “none”):

## Dimension scores
| Dimension | Rating | Notes |
|:----------|:-------|:------|
| Headings & structure | | |
| Emphasis, color & type | | |
| Links, lists, tables | | |
| Images & complex visuals | | |
| Multimedia | | |
| Language & cognitive load | | |
| Code, math, data | | |
| File & format | | |

## Blockers
1. …

## High-value edits
1. …

## Ship recommendation
- Ship | Ship with nits | Do not ship

## Open questions
- …
```

## Out of scope

- Approving or denying an individual accommodation request
- Claiming WCAG, ADA, or Title II conformance
- Grading people
- Auto-publishing to an LMS or running a named campus scanner
- Rewriting the whole assignment (`assignment-review` / `course-prep`)
- Campus-specific offices, scanners, or deadlines — those live in the
  **target course docs** or a teaching overlay, not this skill
