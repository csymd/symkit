---
name: write-docs
description: >
  Write or revise repo documentation (README, CONTRIBUTING, how-to-run)
  to match the system as it exists. Triggers: write docs, update README,
  runbook, write-docs, /write-docs.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Write docs

Document this repository as it is. Match neighboring tone and layout.

## Steps

1. Read the existing README / CONTRIBUTING / comments next to the change.
2. Name the real command, path, or flag. Do not invent a second interface.
3. Prefer a short happy-path example and one common failure.
4. If you cannot run the command, say so.

## Do not

- Describe a future design as if it already shipped.
- Duplicate long code; link the path and show a minimal snippet.
- Rewrite unrelated pages.

## Output

- What changed and where
- The runnable example
- Any command you could not verify
