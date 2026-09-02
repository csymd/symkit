# symkit

**agentic harnesses** designed for educational purposes but available for all...

Edit content once under `harnesses/`. Install into a teaching, research, experiment, product, creative, performance, or engineering repo so Grok,
Claude Code, Codex, and similar tools share the same
policies via thin path adapters.

Provided via the cSYMd lab. 

## Quick start

```bash
# GitHub Release (no Rust toolchain): pick your OS/arch from
# https://github.com/csymd/symkit/releases
# crates.io (needs Rust; same embed)
cargo install --locked symkit
symkit --help
symkit guide

# From a clone of this repo
./cli/symkit list
./cli/symkit show teaching
 
# New workspace (scaffold + instructor packs + grok adapter)
symkit init /path/to/new-course \
  --harness teaching --role instructor --scaffold

# Existing repo
symkit install /path/to/existing-study \
  --harness research --role researcher
```

The installer **does not commit**. Review the target, then commit only
student-safe / public paths.

## Layout

```text
src/                  Rust installer (catalog, merge, adapters)
core/rules/           always-on rules copied into every target
core/library/skills/  skill bodies; catalog.yaml assigns them to roles
harnesses/
  ai/                 model-run and evaluation scaffolding
  creative/           voice, naming, copy, asset briefs
  engineering/        software implementation and tests; role: engineer
  performance/        exercise physiology + biomechanics; role: coach
  product/            PRDs, roadmap, shipping notes
  research/           experiment tracking, reproducibility, paper layout
  teaching/           course templates, staff/learner agents
cli/symkit            shim → target/debug/symkit
examples/             how to add a custom overlay
```

`catalog.yaml` is the source of truth for harnesses, packages, roles, and adapters.

## Commands

| Command | Purpose |
|:--------|:--------|
| `symkit list` | Harnesses, roles, pack summaries |
| `symkit show <harness>` | Role matrix and on-disk paths |
| `symkit init [dir]` | Create or activate a workspace |
| `symkit install <dir>` | Install packs into an existing repo |
| `symkit adapt <dir>` | Rewrite vendor adapters only |
| `symkit guide` | Big-picture flow, cargo vs clone, what to commit |

Adapters default to **grok**. Harness `AGENTS.md` lands in `AGENTS-SYMKIT.md`
(last pack wins). A pointer is appended to `AGENTS.md`; an existing file is
never replaced. Canonical trees also include `.agents/` (+ `docs/`). Use
`--adapters all` or `--adapters none`.

## Teaching roles

| Role | Installs | Student-facing git? |
|:-----|:---------|:--------------------|
| `instructor` / `faculty` | shared + staff + instructor | No (staff packs are local) |
| `ta` | shared + staff + ta (prunes instructor-only skills) | No |
| `learner` | shared + learner literacy/skills | Docs yes; `.agents/` usually no |
| `materials` | shared only | Optional yes |

Do not stack a full faculty tree and a full learner tree unless you pass both
explicitly (`--pack`).

## What to commit in a target repo

**Usually commit:** `AGENTS.md` (repo rules plus the harness pointer),
`AGENTS-SYMKIT.md` (harness defaults), `docs/` literacy guides, workspace
scaffold (`assignments/`, `analysis/`, …).

**Usually do not commit:** `.agents/`, `.grok/`, `.claude/`, `.codex/`, `.symkit/`.
The installer adds those patterns to the target `.gitignore`.

Staff, instructor, and TA packs must not land on student-visible branches.

## Tests

```bash
cargo test
./tests/smoke.sh
```

CI tests require a Rust toolchain. `./cli/symkit` builds the debug binary if
needed. End users can install a GitHub Release binary instead (see
[docs/install.md](docs/install.md)).

GitHub Release binaries and `cargo install --locked symkit` both embed the
catalog and harness trees. Outside a checkout, the CLI extracts them to
`$XDG_DATA_HOME/symkit/<version>/`. Set `SYMKIT_ROOT` to force a checkout.

## Contributing

- [CONTRIBUTING.md](CONTRIBUTING.md) — how to send changes
- [DEVELOPMENT.md](DEVELOPMENT.md) — how to extend the kit
- [AGENTS.md](AGENTS.md) — guidelines for agentic tools working *in this repo*

## License

Apache License 2.0. See [LICENSE](LICENSE) and [NOTICE.md](NOTICE.md).

Copyright (c) 2026, PalEm Dynamics LLC.
