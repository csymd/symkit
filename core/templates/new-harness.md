# New harness

1. Copy `examples/teaching-overlay/` or an existing `harnesses/<name>/` tree.
2. Add packages under `harnesses/<name>/packages/<pkg>/` with any of:
   - `AGENTS.md` (installed as `AGENTS-SYMKIT.md`; a pointer is appended to the target `AGENTS.md`)
   - `.agents/rules/`, `.agents/agents/`
   - `docs/`
3. Add or reuse skill bodies under `core/library/skills/`.
4. Optional workspace scaffold: `harnesses/<name>/workspace/`.
5. Optional `--docs` blanks: `harnesses/<name>/templates/` plus a
   `templates:` map in `catalog.yaml`.
6. Register the harness, packages, role `packages:` / `skills:`, and prune
   lists (agents/rules) in `catalog.yaml`.
7. Check with:

```bash
./cli/symkit show <name>
./cli/symkit init /tmp/symkit-try --harness <name> --role <role> --scaffold --yes
```

Keep `AGENTS.md` short. Long procedures belong in skills.
