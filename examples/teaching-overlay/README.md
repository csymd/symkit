# Example teaching overlay

A course-specific package you can copy. It is **not** registered in
`catalog.yaml` by default.

Stub rule: `.agents/rules/course-overlay.md`. Rename it when you copy.

## Add your own

1. Copy this directory to `harnesses/teaching/packages/<your-id>/`.
2. Edit `.agents/rules/course-overlay.md` (or rename it to `<your-id>.md`).
3. Register under `harnesses.teaching.packages` in `catalog.yaml`.
4. Optionally add the pack to a role list, or install it on demand:

```bash
./cli/symkit install /path/to/course \
  --harness teaching --role instructor --also <your-id>
```

Keep overlays short. Org-wide policy stays in `shared` / `learner`.
`--also` only accepts packs already in that harness’s catalog.
