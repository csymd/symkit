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
    .agents/skills/*/SKILL.md
    .agents/agents/*.md
    docs/                     # merged into target docs/
  workspace/                  # copied only by `symkit init --scaffold`
```

## Register

Add the harness under `harnesses:` in `catalog.yaml`:

- `status: active` (or `later` to list but refuse install)
- `packages` with `path` and `student_safe`
- `roles` mapping role → package list
- `prune` for leftover skills/agents/rules when switching roles
- `workspace` path if you have a scaffold

## Check

```bash
./cli/symkit show <name>
./cli/symkit init /tmp/symkit-try --harness <name> --role <role> --scaffold --yes
```

See `core/templates/new-harness.md` and `examples/teaching-overlay/`.
The installer is Rust (`src/`); you do not register packs in code.

## Overlays vs new harnesses

- **Overlay:** extra package on an existing harness (`--pack` or `--also`).
  Use for one course or lab’s rules.
- **New harness:** different domain (layout, roles, skills). Use when teaching
  conventions would be the wrong default.
