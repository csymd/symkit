---
name: write-gherkin
description: >
  Author clear Gherkin (.feature) acceptance scenarios as living specifications.
  Use with instructor/ta/materials-author for lab success criteria, and with
  researcher for study/analysis pipelines. Triggers: Gherkin, BDD, .feature
  files, acceptance criteria, scenarios, Given/When/Then, write-gherkin,
  /write-gherkin.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Write Gherkin

Readable, testable Gherkin (usually `.feature` files). Compose with the
active agent:

| Agent | Typical use |
|:------|:------------|
| `instructor` / `materials-author` | Lab/assignment success criteria (not an answer key) |
| `ta` | Draft criteria or check that a submission meets existing scenarios |
| `researcher` | Analysis or study pipelines (inputs → process → observables) |

You write **specifications**. Step definitions and production code are out of
scope unless the user also asks.

## What you produce

- Valid Gherkin: `Feature`, `Scenario` / `Scenario Outline`, `Given` / `When` /
  `Then` / `And` / `But`
- One primary behavior per scenario; happy path + a meaningful failure
- **Observable** outcomes (files, status, messages, metrics, reports)

## Mode (pick one; ask once if unclear)

| Mode | Focus |
|:-----|:------|
| **Learning / assignment** | What “done” means for a lab or homework |
| **Research pipeline** | Data in → processing → figures/tables/metrics |
| **Product / system** | User or API behavior of software under test |

## Do

- Language a human can read without knowing the codebase.
- Scenarios are contracts: if they cannot be checked, rewrite them.
- Prefer a few high-value scenarios over exhaustive combinatorial noise.
- Match existing layout (`features/`, `specs/`, course or analysis paths).
- If no runner exists yet, still write clean Gherkin and note a recommended
  path (`pytest-bdd`, `behave`, cucumber-rs, or docs-only until wired).

## Don't

- Invent product behavior, endpoints, rubric points, or analysis claims.
- Write “Then the student understands X” or other untestable wishes.
- Leak a full solution when writing assignment criteria.
- Over-claim causality in research scenarios; keep measurement vs model
  vs interpretation distinct.
- Dump 20 scenarios when 3 cover the risk.

## Output

1. **Mode** used
2. **Path** for the `.feature` file(s)
3. **Full Gherkin** (fenced `gherkin` block or ready-to-write files)
4. **Scenario map** — one line per scenario: intent + what “pass” means
5. **Gaps** — missing product facts, data rights, or untestable lines avoided
