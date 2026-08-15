---
name: release-notes
description: >
  Draft human-facing release notes from the actual diff, tags, or changelog.
  Use before a ship announcement. Do not market work that did not land.
  Triggers: release notes, changelog blurb, what shipped, release-notes,
  /release-notes.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Release notes

Notes a human can send. Evidence is the diff, tag, or existing changelog.

## Steps

1. Collect what actually changed (git log/diff, `CHANGELOG`, merged PRs).
2. Group by user-visible change. Drop chore-only noise unless asked.
3. One line per item: what a user can do now, or what broke and was fixed.
4. Call out required action (migrate, re-install, new flag).
5. Match voice in `docs/brand/` when present; otherwise plain English.

## Do not

- Announce features that are not in the tree.
- Inflate impact (“delights users”, “10x”).
- Invent metrics or quotes.

## Output

- Audience + channel (README, GitHub release, email)
- Draft notes
- Omissions (known unshipped work you refused to include)
