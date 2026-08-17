# Development

This document describes how to build, test, and extend the symkit repository.

For guidelines when working with AI/agentic development tools, see
[AGENTS.md](AGENTS.md). All contributors are expected to take full ownership
of submitted code. Human how-to for *using* the installer is in
[docs/install.md](docs/install.md); this file is for people changing the kit.

## Prerequisites

Use **[rustup](https://rustup.rs/)** so `stable` (build, clippy, tests) and
`nightly` (canonical `rustfmt`) stay in sync with CI.

```bash
# Install rustup if needed, then:
rustup toolchain install stable
rustup toolchain install nightly
rustup component add rustfmt clippy --toolchain stable
rustup component add rustfmt --toolchain nightly
rustup default stable

rustc --version          # should be rustup's stable
cargo +nightly fmt --version
```

- `./cli/symkit` builds `target/debug/symkit` if it is missing
- No Python, no rsync, no network required for install
- End users of the installer only need **stable** `rustc` / `cargo` (see
  [docs/install.md](docs/install.md))
- The binary embeds `catalog.yaml`, `core/`, and `harnesses/` at compile
  time (`include_str!` / `include_dir!`). A checkout still wins over the
  embedded cache so local edits are what `./cli/symkit` installs.

## Common commands

From the repository root:

```bash
# Build
cargo build
cargo test
cargo +nightly fmt
cargo clippy -- -D warnings

# Discover (shim builds the debug binary if needed)
./cli/symkit --help
./cli/symkit guide
./cli/symkit list
./cli/symkit show teaching

# Or call the binary directly
cargo run -- list
./target/debug/symkit show teaching

# Smoke tests (creates a temp dir, then removes it)
./tests/smoke.sh

# Dry-run a real target (no writes)
./cli/symkit install /path/to/existing --harness research --role researcher --dry-run
```

Do **not** run `init` / `install` against this repository. The CLI refuses
that (`catalog.yaml` + `harnesses/` present).

## Repository structure

```text
Cargo.toml            Rust installer crate
src/                  catalog, install, adapters, gitignore
catalog.yaml          harnesses, packages, roles, adapters (source of truth)
cli/symkit            thin shim → target/debug/symkit
core/rules/           installed into every target (.agents/rules/)
core/library/skills/  skill bodies; catalog assigns which roles get them
core/templates/       new-agent / new-rule / new-skill / new-harness
harnesses/<name>/
  packages/<pkg>/     AGENTS.md, .agents/{rules,skills,agents}, docs/
  workspace/          copied only by `symkit init --scaffold`
examples/             overlay pattern (not registered by default)
docs/                 install, UX, authoring
tests/smoke.sh        installer behavior
```

`catalog.yaml` drives the CLI. Do not add hardcoded harness or course IDs in
`src/`. See [docs/authoring-a-harness.md](docs/authoring-a-harness.md).

## How an install works

1. `cli/symkit` execs the Rust binary; clap parses flags; `catalog.rs` resolves
   harness + role + packs.
2. Optional workspace scaffold (`init --scaffold`) copies files without
   overwriting unless `--force`.
3. Catalog `prune` lists remove leftover role paths (skills / agents / rules
   under `.agents/` and vendor mirrors).
4. `core/rules/` merges into the target `.agents/rules/`.
5. Library skills listed on the role (plus `core.always_skills`) copy into
   `.agents/skills/<name>/`. Other library skills are pruned.
6. Each resolved package merges `AGENTS.md` (last pack wins), `.agents/`
   (rules/agents; leftover package skills if any), and `docs/`.
7. Selected adapters mirror `.agents/` into `.grok/`, `.claude/`, and/or
   `.codex/` (default: grok).
8. Target `.gitignore` is updated additively (`.agents/`, vendor trees,
   `.symkit/`).
9. `.symkit/state.yaml` records harness, role, packs, adapters.
10. The installer **never commits**.

Private packs (`student_safe: false`) print a reminder. That is social, not
enforced access control.

## Adding or changing a harness

1. Add or edit packages under `harnesses/<name>/packages/`.
2. Add or reuse skill bodies under `core/library/skills/`.
3. Optional workspace under `harnesses/<name>/workspace/`.
4. Register `status`, `packages`, role `packages:` / `skills:`, `prune`,
   and `workspace` in `catalog.yaml`.
5. Check:

```bash
./cli/symkit show <name>
./cli/symkit init /tmp/symkit-try --harness <name> --role <role> --scaffold --yes
./tests/smoke.sh
```

`status: later` harnesses appear in `list` / `show` and refuse `install`.

Overlays vs new harnesses: an overlay is an extra package (`--pack` / `--also`)
on an existing harness. A new harness is for a different default layout and
role matrix. Copy `examples/teaching-overlay/` for the overlay pattern.

## Code style

- Keep the CLI thin. Content lives in harness packages, not in `src/`.
- Prefer catalog fields over new flags when the data is already structured.
- Run `cargo +nightly fmt` and `cargo clippy -- -D warnings` before opening a PR.
- Keep `AGENTS.md` in packages short; long procedures belong in `SKILL.md`.
- Do not commit secrets, credentials, or restricted data.

### Copyright headers

Apache 2.0 covers the repo via [`LICENSE`](LICENSE), [`NOTICE.md`](NOTICE.md),
the README license section, and `Cargo.toml`. Copyright is PalEm Dynamics LLC.
Do not stamp every file.

Use this two-line header (comment syntax for the file type):

```
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
```

Rust / C-style: `//`. Python, shell, YAML, TOML: `#`. Markdown: an HTML comment.

**Keep** the short header on:

- Engine: `src/`, `cli/symkit`, `tests/smoke.sh`, `Cargo.toml`, `catalog.yaml`
- Pack-owned content the installer copies: package `AGENTS.md`,
  `.agents/{rules,skills,agents}`, package `docs/`, `core/rules/`,
  `core/library/skills/`
- Authoring templates in `core/templates/` (they seed installed content)

**Do not** put headers on:

- Workspace scaffold stubs (`harnesses/*/workspace/`) — those become the
  target author's files after `init --scaffold`
- Kit-internal docs (`README.md`, `CONTRIBUTING.md`, `DEVELOPMENT.md`, repo
  `AGENTS.md`, `docs/`, harness and package READMEs)
- Tooling config (`rustfmt.toml`, `clippy.toml`, `.gitignore`)

## Tests

`cargo test` covers catalog resolve, adapter parsing, and gitignore.

[`tests/smoke.sh`](tests/smoke.sh) is the integration gate. It checks:

- `list` / `show`
- materials → instructor → TA prune
- learner isolation (no staff skills)
- `--adapters none` and `--adapters all`
- `init --scaffold` and no-clobber without `--force`
- research, ai, product, creative, performance, and engineering scaffolds
- install into this repo refused
- gitignore is additive

If you change prune, adapters, scaffold, or catalog resolve, add a unit test
and extend the smoke script when the behavior is user-visible.

## Branch model

| Branch | Role |
|:-------|:-----|
| `develop` | Day-to-day integration (open PRs here) |
| `main` | Stable, review-ready |

Suggested names: `feat/…`, `fix/…`, `docs/…`, `harness/…`.

Do not force-push `main` or `develop`. 

## CI

[`.github/workflows/ci.yml`](.github/workflows/ci.yml) runs on **push and
PRs to `develop` only** (the quality gate). `stage` and `main` skip
day-to-day CI (promotions of a SHA that already passed).
[`.github/workflows/release.yml`](.github/workflows/release.yml) re-runs
that gate on PRs into `main`, pushes to `release/**`, and `v*` tags.
Push to `main` is not a Release trigger (the PR already ran it).
`workflow_dispatch` re-runs `check` only. GitHub Release and crates.io
publish are gated on `refs/tags/v*`.

## Releasing

Users install with `cargo install symkit` (embedded catalog + harnesses)
or by cloning this repo and running `./cli/symkit`.

When a slice is ready:

1. Merge to `develop` with CI green.
2. On `release/vX.Y.Z` (or develop): `./scripts/bump-version.sh patch --changelog`
3. Open a PR from `release/vX.Y.Z` (or `develop`) into `main`. Merge when
   Release `check` is green.
4. Tag `vX.Y.Z` on that commit (tag must match `[package] version` in
   `Cargo.toml`). Push the tag. The release workflow runs `check` again,
   opens the GitHub Release, and publishes `symkit` to crates.io.

crates.io publish uses GitHub Environment `crates-io` and secret
`CARGO_REGISTRY_TOKEN`. Create that environment and token before the
first tag you want on the registry. A failed publish does not block the
GitHub Release. Yank a bad crate version on crates.io if you must; do
not reuse a burned version.

## Other notes

- Target repos receive **copies**. Editing a harness here does not update
  already-installed trees until someone re-runs `install`.
- One harness per target is the v1 assumption; mixing prints a warning.
- Internal agent guidelines live in [AGENTS.md](AGENTS.md).
- Feel free to open issues for anything unclear in this document.

---

**Quick links**

- [AGENTS.md](AGENTS.md) — guidelines for agentic development
- [CONTRIBUTING.md](CONTRIBUTING.md)
- [docs/install.md](docs/install.md) — user install
- [docs/authoring-a-harness.md](docs/authoring-a-harness.md)
- [docs/ux.md](docs/ux.md)
- [`catalog.yaml`](catalog.yaml)
