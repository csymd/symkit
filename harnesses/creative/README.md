# Creative harness

Voice, naming, copy critique, and asset briefs. Separate from `product`
(specs and roadmap).

```bash
./cli/symkit init /path/to/new-brand \
  --harness creative --role creative-director --scaffold
```

Roles: `creative-director` (`creative`), `materials`.

`docs/brand/` is the source of truth. Do not invent a visual system.

One harness per target. To use both creative and product in one repo, pass
`--also` on purpose — v1 will warn.
