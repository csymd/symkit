# Install

Clone this repo and run the CLI from the clone. No package registry.

```bash
git clone https://github.com/csymd/symkit.git
cd symkit
./cli/symrig --help
```

Requires a Rust toolchain (`cargo` / `rustc`). `./cli/symrig` builds
`target/debug/symrig` if it is missing.

## New workspace

```bash
./cli/symrig init ~/worx/my-course \
  --harness teaching --role instructor --scaffold --yes
```

Creates the directory if needed, copies the harness workspace template,
merges agent packs, writes the grok adapter, and updates `.gitignore`.

## Existing repo

```bash
./cli/symrig install ~/worx/existing --harness research --role researcher --yes
```

## Adapters

Canonical content is always `AGENTS.md` + `.agents/`. Adapters are mirrors:

```bash
./cli/symrig install DIR --harness teaching --role instructor --adapters all
./cli/symrig adapt DIR --adapters none   # does not delete existing vendor trees
```

Default: `grok` only.

## What not to do

- Do not run init/install against the symrig repo itself.
- Do not commit `.agents/`, `.grok/`, `.claude/`, `.codex/`, or `.symrig/`.
- Do not push staff/instructor/ta packs to a student-visible branch.
