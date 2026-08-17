<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Match the repo

The existing project is the source of truth for how to write code here.

## Do

- Use the language, package manager, and test runner already in the tree.
- Copy naming, error, and comment conventions from adjacent files.
- Keep diffs scoped to the request.
- Follow the directories that are already here (`src/`, `lib/`, `pkg/`, …).

## Do not

- Add a new language, framework, or CI system to “improve” the repo.
- Create a parallel `src/` or test tree beside the one that already exists.
- Reformat unrelated files.
- Claim tests pass if you did not run them (or could not run them — say so).
- Reimplement a documented shared kernel (wrap it).
