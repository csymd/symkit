# Install

Prebuilt binaries ship on [GitHub Releases](https://github.com/csymd/symkit/releases)
(Linux, macOS, Windows). No Rust toolchain required.

```bash
# GitHub Release — pick the archive for your OS/arch, then:
tar -xzf symkit-*-unknown-linux-musl.tar.gz        # Linux
tar -xzf symkit-*-apple-darwin.tar.gz               # macOS
tar -xzf symkit-*-pc-windows-msvc.tar.gz            # Windows 10+ (tar.exe)

./symkit --help          # includes the primary flow
./symkit guide           # GitHub vs cargo vs clone, adapters, what to commit
```

Assets look like `symkit-<version>-<target>.tar.gz`:

| Target | Who |
|:-------|:----|
| `x86_64-unknown-linux-musl` | Linux x86_64 (static; no glibc pin) |
| `aarch64-unknown-linux-musl` | Linux ARM64 |
| `aarch64-apple-darwin` | macOS Apple Silicon |
| `x86_64-apple-darwin` | macOS Intel |
| `x86_64-pc-windows-msvc` | Windows x86_64 (`symkit.exe` in the archive) |

Each archive is the binary plus `LICENSE`. `SHA256SUMS` is attached to the
release. Optional provenance check (GitHub CLI):

```bash
gh attestation verify symkit-<version>-<target>.tar.gz --repo csymd/symkit
sha256sum -c SHA256SUMS
```

macOS Gatekeeper and Windows SmartScreen may warn on an unsigned download
(there is no Apple Developer ID / Authenticode cert on these builds). Open
via Finder/Explorer once, or `xattr -d com.apple.quarantine symkit` on macOS.

The binary embeds `catalog.yaml`, `core/`, and `harnesses/`. The first run
that is **not** inside a checkout writes them under
`$XDG_DATA_HOME/symkit/<version>/` (or `~/.local/share/symkit/<version>/`;
`%LOCALAPPDATA%\symkit\<version>` on Windows).

## From crates.io (needs Rust)

```bash
cargo install --locked symkit
```

Same embed and extract path as the GitHub binary.

## From a clone

```bash
git clone https://github.com/csymd/symkit.git
cd symkit
./cli/symkit --help    # builds target/debug/symkit if needed
```

Running `symkit` from inside a checkout uses that tree, not the embed.
Override the checkout with `SYMKIT_ROOT`; override the cache parent
with `SYMKIT_DATA`.

## Trust

A GitHub Release binary is a compiled blob. Treat it like any other
CLI download: prefer the asset from [this repo's Releases
page](https://github.com/csymd/symkit/releases), check `SHA256SUMS`,
and use `gh attestation verify` if you want the build tied back to the
tag's Actions run.

That is the usual model for tools such as `gh` or `ripgrep`, not a
shortcut around crates.io. Compiling with `cargo install --locked
symkit` (or `./cli/symkit` from a clone) is the "build it yourself"
path. A compromised GitHub token or Actions workflow is the same
*class* of risk as a compromised crates.io publish token; attestations
and checksums make substitution harder to hide. We do not ship a
`curl | sh` installer.

## New workspace

```bash
symkit init /path/to/new-course \
  --harness teaching --role instructor --scaffold --yes
```

Creates the directory if needed, copies the harness workspace template,
merges agent packs, writes the grok adapter, and updates `.gitignore`.

## Existing repo

```bash
symkit install /path/to/existing --harness research --role researcher --yes
```

Do not pass `--scaffold` onto a live tree unless you want folder stubs
(existing files are left alone unless `--force`).

## Adapters

Harness `AGENTS.md` is written as `AGENTS-SYMKIT.md` (last pack wins). A
pointer is appended to `AGENTS.md`; existing `AGENTS.md` is never replaced.
Canonical trees also include `.agents/`. Adapters are mirrors:

```bash
symkit install DIR --harness teaching --role instructor --adapters all
symkit adapt DIR --adapters none   # does not delete existing vendor trees
```

Default: `grok` only.

## What not to do

- Do not run init/install against the symkit repo itself.
- Do not commit `.agents/`, `.grok/`, `.claude/`, `.codex/`, or `.symkit/`.
- Do not push staff/instructor/ta packs to a student-visible branch.
