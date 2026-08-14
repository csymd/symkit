# Contributing to symrig

Thank you for your interest in contributing to **symrig**.
We welcome contributions of all kinds—whether it's fixing a typo, improving
documentation, reporting bugs, or adding a harness, package, or installer
behavior.

## Our philosophy

symrig is a **cSYMd** lab kit: vendor-neutral agent harnesses you install into
*other* repositories. It emphasizes **clarity** (catalog-driven, one source of
truth), **privilege separation** (role packs on disk, not runtime ACL), and
**safe defaults** (preview before write, never commit, never push).

We encourage contributions that keep the installer small, the catalog honest,
and harness content customizable elsewhere.

The UNCG-facing predecessor lives separately as
[`not-uncg-msia/ai-kit`](https://github.com/not-uncg-msia/ai-kit). Do not
re-introduce course- or institution-specific identity as the default.

## AI-assisted contributions

Feel free to use AI tools (such as Grok, Claude, Copilot, etc.) to assist
your work. We do not regulate how you use AI, but all contributors must:

1. Be able to clearly explain the changes and reasoning behind them.
2. Ensure the contribution meets project standards for quality, safety, and
   style.
3. Take full ownership of the submitted code and content.

AI should be treated as a helpful collaborator — **you** remain responsible
for the final result.

See [AGENTS.md](AGENTS.md) for guidelines when working with agentic tools in
this repository.

## Ways to contribute

- **Submit an Issue** — Report bugs, request harnesses, or suggest installer
  improvements.
- **Submit a Pull Request (PR)** — From small documentation fixes to a new
  package or harness.
- **Improve documentation** — Keep [README.md](README.md),
  [DEVELOPMENT.md](DEVELOPMENT.md), and `docs/` aligned with the code.
- **Write tests** — Add `cargo test` coverage and extend
  [`tests/smoke.sh`](tests/smoke.sh) when you change install, prune, adapter,
  or scaffold behavior.
- **Review Pull Requests** — Provide constructive feedback.

If you see something that needs fixing, feel free to open a PR directly—no
need to wait for an issue to be assigned.

## Getting started

1. **Fork** the repository (or use a branch if you have write access) and clone.
2. Set up the development environment (see [DEVELOPMENT.md](DEVELOPMENT.md)).
3. Create a focused branch from **`develop`**
   (`git checkout -b feat/your-feature-name`).
4. Make your changes, ensuring they follow the catalog/harness conventions,
   pass `cargo test`, `cargo clippy -- -D warnings`, and `./tests/smoke.sh`.
5. Commit with clear, descriptive messages.
6. Push your branch and open a Pull Request against **`develop`**.

## Submitting pull requests

- **Keep PRs focused** — one logical change per PR is strongly preferred.
- **Include tests** when adding or modifying installer behavior.
- **Update documentation** as needed (`README.md`, `docs/`, harness READMEs).
- Register new harnesses and packages in [`catalog.yaml`](catalog.yaml) — do
  not hardcode names in `src/`.
- **Follow the Code of Conduct** in all interactions.
- Be prepared to address review feedback and iterate until the PR is ready
  to merge.

We ask that you stay engaged with your PR—respond to comments and keep the
conversation moving so we can merge high-quality contributions quickly.

## What belongs where

| Change | Put it in |
|:-------|:----------|
| Installer, adapters, catalog parser | `src/`, `cli/symrig` (shim) |
| Domain-agnostic always-on rules | `core/rules/` |
| Skill bodies (placement in catalog) | `core/library/skills/` |
| Authoring templates | `core/templates/` |
| Teaching / research / AI content | `harnesses/<name>/packages/` |
| Workspace files written by `init --scaffold` | `harnesses/<name>/workspace/` |
| Course- or lab-specific extras | overlay package or `examples/` |
| Human how-to | `docs/` |

Keep `AGENTS.md` files in packages short. Long procedures belong in skills.

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).
By participating, you agree to uphold this code in all project spaces.

## Questions or need help?

- Open an **issue** or **discussion** in the repository.
- Lab context: [cSYMd](https://csymd.com) / [github.com/csymd](https://github.com/csymd).
- Copyright is PalEm Dynamics LLC (Apache-2.0). See [LICENSE](LICENSE) and [NOTICE.md](NOTICE.md).

---

**Thank you for helping make symrig better.**
Your contributions support reusable, inspectable agent assistance for
teaching, research, and experiment work.
