# Authoring a harness

A harness is a named set of packages plus an optional workspace scaffold,
registered in `catalog.yaml`.

## Package layout

```text
harnesses/<name>/
  README.md
  packages/<pkg>/
    AGENTS.md                 # copied to target AGENTS-SYMKIT.md (last pack wins)
    .agents/rules/*.md
    .agents/agents/*.md
    docs/                     # merged into target docs/
  templates/                  # faculty-owned blanks; copied by --docs <id>
  workspace/                  # copied only by `symkit init --scaffold`
```

## Register

Add the harness under `harnesses:` in `catalog.yaml`:

- `status: active` (or `later` to list but refuse install)
- `packages` with `path` and `student_safe`
- `roles` mapping role → `packages:` plus `skills:` (library names)
- `prune` for leftover agents/rules when switching roles (skills are derived)
- `workspace` path if you have a scaffold
- optional `templates:` map (`id: { path, dest }`) for `--docs <id>` blanks.
  Teaching ships `slos`; research ships `aims` and `protocol`. Product,
  creative, and performance already scaffold their truth dirs
  (`docs/prd/`, `docs/brand/`, `docs/program/`). AI uses `config.yaml`.
  Engineering follows existing code — do not add a parallel spec template.

## Check

```bash
./cli/symkit show <name>
./cli/symkit init /tmp/symkit-try --harness <name> --role <role> --scaffold --yes
```

See `core/templates/new-harness.md` and `examples/teaching-overlay/`.
The installer is Rust (`src/`); you do not register packs in code.

Skill **bodies** live in `core/library/skills/<name>/SKILL.md`. Who receives
them is `catalog.yaml`: `core.always_skills` (every install) plus each
role’s `skills:` list. Do not copy a skill into more than one package.

## Overlays vs new harnesses

- **Overlay:** extra package on an existing harness (`--pack` or `--also`).
  Use for one course or lab’s rules.
- **New harness:** different domain (layout, roles, skills). Use when teaching
  conventions would be the wrong default.
