---
name: write-tests
description: >
  Write tests that match this repo’s style and runner. Use when adding
  coverage, reproducing a bug, or the user asks for tests. Triggers:
  write tests, add coverage, unit test, write-tests, /write-tests.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Write tests

Tests are first-class code. Match what is already here.

## Steps

1. Detect the runner (`cargo test`, `pytest`, `npm test`, `./tests/smoke.sh`,
   etc.). Do not add a second one.
2. Copy naming and layout from neighboring tests.
3. Cover the invariant and one real failure path. Prefer table-driven
   cases if the project uses them.
4. If `.feature` files exist, map them to the repo’s BDD harness or say
   there isn’t one and write the closest unit/integration test.
5. Run the tests you can. If you cannot run them, say so.

## Do not

- Snapshot noise or assert private trivia.
- Invent fixtures that hide the behavior under test.
- Add a new test framework.

## Output

- Runner used
- Cases covered (and skipped)
- The test code
- Result of the run, or why it was not run
