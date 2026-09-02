// Copyright (c) 2026, PalEm Dynamics LLC
// Licensed under the Apache License, Version 2.0.

//! Top-level help copy. Keep this aligned with README.md and docs/install.md.

/// Shown after the command list on `symkit`, `-h`, and `--help`.
pub const AFTER_HELP: &str = "\
FLOW
  1. list / show     pick a harness and role
  2. init DIR        new workspace  (add --scaffold for folder stubs)
     install DIR     existing repo  (pass --scaffold only if you want stubs)
                     --docs ID      copy a catalogued blank (slos, aims, …)
  3. Read the preview. Nothing is committed.
  4. In the target: git status. Commit public / student-safe paths only.

EXAMPLES
  cargo install --locked symkit # or GitHub Release / ./cli/symkit
  symkit list
  symkit show teaching
  symkit init ~/course --harness teaching --role instructor --scaffold --docs slos
  symkit install ~/study --harness research --role researcher --docs aims --docs protocol

WRITES   AGENTS-SYMKIT.md (harness, last pack wins), AGENTS.md pointer
         (never replaces existing), .agents/, pack docs/, vendor adapters
         (default: grok), additive .gitignore, .symkit/state.yaml
SKIPS    git commit, network, installing into this kit repo

  Full flow:  symkit guide
  Install:    docs/install.md
";

/// Slightly fuller picture for `symkit guide`.
pub fn guide() -> String {
    format!(
        "\
symkit {} — install agent harnesses into another repo

WHAT THIS IS
  A catalog-driven installer. You pick a harness (domain) and a role
  (who you are). It copies harness AGENTS.md to AGENTS-SYMKIT.md,
  appends a pointer on AGENTS.md (never replaces it), and copies rules,
  skills, and optional workspace stubs into a target directory. It does
  not commit, push, or talk to the network.

HOW TO GET THE BINARY
  GitHub Release           # https://github.com/csymd/symkit/releases
                           # Linux / macOS / Windows archives; no Rust
                           # toolchain. SHA256SUMS on the release; optional:
                           #   gh attestation verify FILE --repo csymd/symkit
  cargo install --locked symkit
                           # compile from crates.io (needs Rust)
  ./cli/symkit …           # from a clone; uses that checkout

  Release binaries and cargo install both embed catalog + harnesses;
  first run extracts them to $XDG_DATA_HOME/symkit/<version>/ (or
  %LOCALAPPDATA%\\symkit\\<version> on Windows). Running from inside a
  checkout uses the files on disk, not the embed. Set SYMKIT_ROOT to
  force a checkout. Set SYMKIT_DATA to change the extract parent.

PRIMARY FLOW
  symkit list                         # harnesses and default roles
  symkit show <harness>               # packs, roles, skills
  symkit init <dir> --harness … --role … [--scaffold]
  symkit install <dir> --harness … --role …

  init     creates the directory if needed; asks on a TTY for missing
           target / harness / role / scaffold.
  install  requires an existing directory and --harness.

  Preview always prints. Confirm, or pass --yes. --dry-run exits after
  the preview. --force overwrites existing scaffold files and --docs copies.

  --docs <id> copies a catalogued faculty-owned blank into docs/ or
  documents/ (detected; --docs-root if both exist). Copy-if-missing unless
  --force. symkit show <harness> lists ids.

ADAPTERS
  Canonical trees are AGENTS-SYMKIT.md + .agents/ (+ docs/). AGENTS.md
  is the repo's file: a pointer is appended, never replaced.
  Default adapter: grok (.grok/). --adapters all|none|grok,claude,codex
  adapt DIR rewrites vendor mirrors only; --adapters none does not
  delete trees already on disk.

WHAT TO COMMIT IN THE TARGET
  Usually yes:  AGENTS.md (repo rules + pointer), AGENTS-SYMKIT.md
                (harness defaults), docs/ literacy or aims, --docs blanks
                (slos.md, aims.md, …), workspace stubs (assignments/, …)
  Usually no:   .agents/, .grok/, .claude/, .codex/, .symkit/
  Teaching:     staff / instructor / ta packs stay off student-facing git.

ONE HARNESS PER TARGET
  Mixing prints a warning. Extra in-catalog packs: --also <pack>.
  --pack replaces the role's pack list and does not apply role skills.

MORE
  docs/install.md    user how-to
  docs/ux.md         preview / merge / privilege
  catalog.yaml       harnesses, roles, adapters
",
        env!("CARGO_PKG_VERSION")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn after_help_points_at_the_flow() {
        for needle in [
            "FLOW",
            "symkit list",
            "symkit init",
            "symkit guide",
            "Nothing is committed",
        ] {
            assert!(AFTER_HELP.contains(needle), "after_help missing {needle:?}");
        }
    }

    #[test]
    fn guide_covers_cargo_install_and_checkout() {
        let g = guide();
        for needle in [
            "cargo install",
            "GitHub Release",
            "SYMKIT_ROOT",
            "init",
            "install",
            "staff / instructor / ta",
            "--also",
            "--docs",
        ] {
            assert!(g.contains(needle), "guide missing {needle:?}");
        }
    }
}
