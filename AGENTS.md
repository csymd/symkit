# AGENTS.md — symkit development guidelines

This file contains instructions for agentic development tools working **in
this repository** (the kit itself). It is not a harness package and is not
installed into target repos.

All changes must be owned by the human contributor, who is responsible for
reviewing, explaining, and maintaining the work.

Harness-specific agent files (`harnesses/*/packages/*/AGENTS.md`) are content
that gets copied into *other* projects. Edit those when changing what a
target receives. Edit **this** file when changing how we develop the kit.

## Project overview

symkit is a **content + installer** kit, not an application server.

- **Canonical content:** `harnesses/<name>/packages/` (`AGENTS.md`,
  `.agents/{rules,skills,agents}`, `docs/`)
- **Engine:** Rust crate (`src/`) + `catalog.yaml`
- **Front door:** `cli/symkit` shim → `target/debug/symkit`
  (`list`, `show`, `init`, `install`, `adapt`)
- **Adapters:** optional mirrors of `.agents/` into `.grok/`, `.claude/`,
  `.codex/` (default: grok)

## Development focus

Keep the installer catalog-driven and the teaching/research/ai harnesses
honest about what they install.

### Do not break

- `catalog.yaml` is the source of truth. No new hardcoded harness, role, or
  course IDs in `src/`.
- Preview before write. Never commit or push from the installer.
- Refuse `init` / `install` when the target *is* this repo.
- Privilege is **pack-based**: TA installs prune instructor-only skills;
  learner installs must not drop staff trees onto disk.
- `AGENTS.md` in a target is last-pack-wins; `.agents/` and `docs/` merge.
- Default adapters: **grok only**. `--adapters none` writes no vendor trees.
- Scaffold does not overwrite existing files unless `--force`.
- Gitignore in the target is **additive** (marker block for `.agents/`,
  `.grok/`, `.claude/`, `.codex/`, `.symkit/`).

### Working style

- Prefer incremental, working changes. Get `./tests/smoke.sh` green.
- Avoid over-engineering: no plugin marketplace, no npm/pip publish, no
  web UI in v1.
- Keep package `AGENTS.md` short. Long procedures belong in `SKILL.md`.
- One harness per target (warn if a second is installed). Do not invent
  multi-harness merge semantics.
- Copyright headers: engine (`src/`, CLI) and pack-owned installed content
  (`AGENTS.md`, `.agents/`, pack `docs/`, `core/rules/`,
  `core/library/skills/`). Not on workspace stubs or kit-internal
  README/docs. See DEVELOPMENT.md.

## Development commands

```bash
cargo +nightly fmt
cargo test
cargo clippy -- -D warnings
./cli/symkit list
./cli/symkit show teaching
./tests/smoke.sh
```

Requires rustup with **stable** (build/clippy/test) and **nightly**
(`rustfmt`). See [DEVELOPMENT.md](DEVELOPMENT.md).

## When to ask vs. when to just do it

**Ask first when:**

- Changing CLI flag semantics or default adapters
- Changing prune policy or `student_safe` meaning
- Adding a new harness or renaming an existing one
- Introducing a language, test runner, or package manager
- Retrofitting existing lab repos (sleep-study, llm-revision-dynamics, etc.)

**Just implement when:**

- Bug fixes that match documented intent
- Docs that describe the system as it exists
- New skills/rules inside an existing package
- Smoke-test coverage for a behavior you already changed

## File locations of note

| Path | Role |
|:-----|:-----|
| [`catalog.yaml`](catalog.yaml) | Harnesses, packs, roles, prune, adapters |
| [`src/catalog.rs`](src/catalog.rs) | Load + resolve |
| [`src/install.rs`](src/install.rs) | Scaffold, merge, prune, state |
| [`src/adapters.rs`](src/adapters.rs) | Vendor mirrors |
| [`src/main.rs`](src/main.rs) | clap CLI |
| [`cli/symkit`](cli/symkit) | Shim that builds/execs the debug binary |
| [`core/rules/`](core/rules/) | Always installed (`data-handling`, `secrets`) |
| [`core/library/skills/`](core/library/skills/) | Skill bodies; `catalog.yaml` assigns them to roles |
| [`tests/smoke.sh`](tests/smoke.sh) | Integration gate |
| [`docs/authoring-a-harness.md`](docs/authoring-a-harness.md) | How to add a harness |

## Content rules (harness packages)

- Teaching faculty packs (`staff`, `instructor`, `ta`) are **not**
  student-safe. Do not weaken the warnings.
- Learner literacy is a separate pack. Do not stack full faculty + full
  learner trees unless the human passed `--also` / `--pack`.
- Research: written aims / protocol / SAP are the source of scientific
  truth. Do not invent endpoints, n, or effect sizes in skills or rules.
- AI experiments: config is the source of truth. Do not invent metrics or
  “we ran this.”
- Performance: written program and recorded trials are the source of truth.
  Do not invent loads, kinematics, or clinical advice.
- Engineering: existing code, tests, and lockfiles are the source of truth.
  Do not invent a second stack or claim tests passed if they did not run.
- If a target repo documents SymWorx (or another shared kernel), **wrap
  it** — do not reimplement those algorithms inside a harness.

## Data and secrets

Follow `core/rules/data-handling.md` and `core/rules/secrets.md`. Never
commit credentials, PHI, DUA-restricted extracts, or `.env` files. Public
samples only under paths like `data/public/`.

## Related docs

- [CONTRIBUTING.md](CONTRIBUTING.md) — how humans send changes
- [DEVELOPMENT.md](DEVELOPMENT.md) — commands, layout, branch model
- [README.md](README.md) — user-facing overview

---

When you start a new session, read this file and respect the catalog-driven
install rules above.
