# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Install writes harness `AGENTS.md` to `AGENTS-SYMKIT.md` and appends a
  pointer on `AGENTS.md`. Existing `AGENTS.md` is never replaced.
- `--docs <id>` copies catalogued faculty-owned blanks into `docs/` or
  `documents/` (copy-if-missing unless `--force`). Teaching: `slos`.
  Research: `aims`, `protocol`. `symkit show` lists ids.

### Changed


## [0.1.2] - 2026-08-22

### Added

- GitHub Release assets: prebuilt `symkit` binaries for Linux (musl x86_64
  and aarch64), macOS (x86_64 and Apple Silicon), and Windows (x86_64),
  plus `SHA256SUMS` and GitHub artifact attestations.

### Changed

- Install docs and `symkit guide` lead with GitHub Release binaries; crates.io
  remains the compile-from-source path (`cargo install --locked symkit`).

## [0.1.1] - 2026-08-17

### Added

- `symkit --help` prints a short FLOW / EXAMPLES / WRITES block.
- `symkit guide` covers cargo install vs clone, adapters, and what to commit.
- Flag help on `init`, `install`, and `adapt`.
- `write-docs` library skill; assigned to engineering `engineer` / `swe`.

### Changed

- Research packs treat an existing tree as the layout source (`study-layout`,
  `repro-check`, research `AGENTS.md`).
- Teaching `course-materials` rule states what to commit; overlay example
  ships a stub rule.
- Engineering `match-repo` forbids a parallel `src/` / test tree.

---

## [0.1.0] - 2026-08-15

### Added

- Initial release.
- Provides an initial pass at key agents for education as well as engineering, 
  creative, research, and product management. 
- A long list of common skills; some unique to one agent harness and some shared 
  across multiple agent harnesses.
  
### Notes

- Pushing a an early release ... we expect bugs, but hope to keep iterating on this
  to help provide an easy framework for newcomers (and experienced users) to set up 
      new harnesses...

---

## Version Links

[0.1.2]: https://github.com/csymd/symkit/releases/tag/v0.1.2
[0.1.1]: https://github.com/csymd/symkit/releases/tag/v0.1.1
[0.1.0]: https://github.com/csymd/symkit/releases/tag/v0.1.0
