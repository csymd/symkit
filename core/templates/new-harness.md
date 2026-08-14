# New harness

1. Copy `examples/teaching-overlay/` or an existing `harnesses/<name>/` tree.
2. Add packages under `harnesses/<name>/packages/<pkg>/` with any of:
   - `AGENTS.md`
   - `.agents/rules/`, `.agents/skills/`, `.agents/agents/`
   - `docs/`
3. Optional workspace scaffold: `harnesses/<name>/workspace/`.
4. Register the harness, packages, roles, and prune lists in `catalog.yaml`.
5. Check with:

```bash
./cli/symkit show <name>
./cli/symkit init /tmp/symkit-try --harness <name> --role <role> --scaffold --yes
```

Keep `AGENTS.md` short. Long procedures belong in skills.
