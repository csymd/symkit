# symkit

Vendor-neutral **agent harnesses** you install into another repository.

Edit content once under `harnesses/`. Install into a teaching, research, experiment, product, creative, or performance repo so Grok,
Claude Code, Codex, and similar tools share the same
policies via thin path adapters.

cSYMd lab kit. The UNCG-facing predecessor lives separately as
[`not-uncg-msia/ai-kit`](https://github.com/not-uncg-msia/ai-kit).

## Quick start

```bash
# From a clone of this repo
./cli/symkit list
./cli/symkit show teaching

# New workspace (scaffold + instructor packs + grok adapter)
./cli/symkit init /path/to/new-course \
  --harness teaching --role instructor --scaffold

# Existing repo
./cli/symkit install /path/to/existing-study \
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
  teaching/           course templates, staff/learner agents
  research/           experiment tracking, reproducibility, paper layout
  ai/                 model-run and evaluation scaffolding
  product/            PRDs, roadmap, shipping notes
  creative/           voice, naming, copy, asset briefs
  performance/        exercise physiology + biomechanics; role: coach
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

Adapters default to **grok**. Canonical content always lands in `AGENTS.md` +
`.agents/` (+ `docs/`). Use `--adapters all` or `--adapters none`.

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

**Usually commit:** `AGENTS.md` (if you want shared defaults), `docs/` literacy
guides, workspace scaffold (`assignments/`, `analysis/`, …).

**Usually do not commit:** `.agents/`, `.grok/`, `.claude/`, `.codex/`, `.symkit/`.
The installer adds those patterns to the target `.gitignore`.

Staff, instructor, and TA packs must not land on student-visible branches.

## Tests

```bash
cargo test
./tests/smoke.sh
```

Requires a Rust toolchain. `./cli/symkit` builds the debug binary if needed.

## Contributing

- [CONTRIBUTING.md](CONTRIBUTING.md) — how to send changes
- [DEVELOPMENT.md](DEVELOPMENT.md) — how to extend the kit
- [AGENTS.md](AGENTS.md) — guidelines for agentic tools working *in this repo*

## License

Apache License 2.0. See [LICENSE](LICENSE) and [NOTICE.md](NOTICE.md).

Copyright (c) 2026, PalEm Dynamics LLC.
