<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Course materials policy

Always-on rules for course materials repositories.

## Audience and surfaces

- **`main`** (and release tags) are student-facing. Treat anything that lands
  there as public to the class.
- Authoring may use `develop` and short-lived branches; do not assume private
  branches stay private forever.

## Content norms

- Prefer the course’s existing structure (`docs/modules/`, `lectures/`,
  `assignments/` when those exist).
- Link to shared setup docs rather than duplicating long install guides.
- Keep faculty logistics that students should not see out of student-facing
  trees (or gate them clearly).

## What to commit

- **Usually commit:** `AGENTS.md` (if the course wants shared defaults),
  `docs/` literacy guides, course SLOs (`docs/slos.md` or `documents/slos.md`),
  workspace stubs (`assignments/`, `lectures/`, …).
- **Usually do not commit:** `.agents/`, `.grok/`, `.claude/`, `.codex/`,
  `.symkit/`. The installer adds those patterns to `.gitignore`.
- **Never** put `staff`, `instructor`, or `ta` packs on a student-facing
  branch. `student_safe: false` is a reminder, not access control — review
  `git status` before you push.

## Release hygiene

- Before a student release: no restricted data, no secrets, no accidental
  answer keys.
- Release naming follows the course’s documented scheme.
- Changelog or release notes should describe what students get, not internal
  debate.

## AI-assisted authoring

- Agents assist faculty; faculty own accuracy of domain, legal, and policy claims.
- Do not cite fabricated papers, vendors, or standards. Prefer primary sources
  the course already uses.
