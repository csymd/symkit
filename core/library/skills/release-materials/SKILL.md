---
name: release-materials
description: >
  Prepare a student-facing course materials release. Use when cutting a
  release, tagging materials, or checking a branch before students pull main.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Release materials

Faculty checklist for shipping course materials. Adapt to the target repo’s
CONTRIBUTING.md.

## 1. Intent

- Confirm which changes on `develop` (or the release branch) are meant for
  students this drop.
- Note anything deferred to a later release.

## 2. Safety scan

- [ ] No credentialed or restricted data under `data/` or elsewhere
- [ ] No secrets, tokens, or local env files
- [ ] No private answer keys in student-facing assignment paths (unless
      intentional and documented)
- [ ] Public samples only; docs still point to external paths for restricted data

## 3. Consistency

- [ ] Syllabus / calendar / module cards agree on dates and deliverables if
      this release touches them
- [ ] Links and paths in README still resolve
- [ ] Container / `uv` instructions still match `pyproject.toml` /
      `Containerfile` if present

## 4. Ship

Typical flow (confirm in repo docs):

1. Land work on `develop`.
2. Merge to `main` as documented.
3. Tag a release when the course process requires it.

## 5. Communicate

- Short student-facing release notes: what changed, what to pull, any action
  required.
- Point graded work back to the **LMS** when relevant.

## Output

Produce a concise checklist result (pass/fail items) and a draft release blurb
faculty can edit.
