# Authoring a harness

A harness is a named set of packages plus an optional workspace scaffold,
registered in `catalog.yaml`.

## Package layout

```text
harnesses/<name>/
  README.md
  packages/<pkg>/
    AGENTS.md                 # last pack wins
    .agents/rules/*.md
    .agents/agents/*.md
    docs/                     # merged into target docs/
  workspace/                  # copied only by `symrig init --scaffold`
```

## Register

Add the harness under `harnesses:` in `catalog.yaml`:

- `status: active` (or `later` to list but refuse install)
- `packages` with `path` and `student_safe`
- `roles` mapping role → `packages:` plus `skills:` (library names)
- `prune` for leftover agents/rules when switching roles (skills are derived)
- `workspace` path if you have a scaffold

## Check

```bash
./cli/symrig show <name>
./cli/symrig init /tmp/symrig-try --harness <name> --role <role> --scaffold --yes
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
