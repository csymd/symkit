# Install

From a clone (uses the files in that checkout):

```bash
git clone https://github.com/csymd/symkit.git
cd symkit
./cli/symkit --help
```

Requires a Rust toolchain (`cargo` / `rustc`). `./cli/symkit` builds
`target/debug/symkit` if it is missing.

From crates.io the binary embeds `catalog.yaml`, `core/`, and `harnesses/`.
The first run that is not inside a checkout writes them under
`$XDG_DATA_HOME/symkit/<version>/` (or `~/.local/share/symkit/<version>/`).
Override the checkout with `SYMKIT_ROOT`; override the cache parent with
`SYMKIT_DATA`.

## New workspace

```bash
./cli/symkit init /path/to/new-course \
  --harness teaching --role instructor --scaffold --yes
```

Creates the directory if needed, copies the harness workspace template,
merges agent packs, writes the grok adapter, and updates `.gitignore`.

## Existing repo

```bash
./cli/symkit install /path/to/existing --harness research --role researcher --yes
```

## Adapters

Canonical content is always `AGENTS.md` + `.agents/`. Adapters are mirrors:

```bash
./cli/symkit install DIR --harness teaching --role instructor --adapters all
./cli/symkit adapt DIR --adapters none   # does not delete existing vendor trees
```

Default: `grok` only.

## What not to do

- Do not run init/install against the symkit repo itself.
- Do not commit `.agents/`, `.grok/`, `.claude/`, `.codex/`, or `.symkit/`.
- Do not push staff/instructor/ta packs to a student-visible branch.
