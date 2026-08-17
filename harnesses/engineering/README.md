# Engineering harness

Software engineering for application and library repos. Separate from
`product` (specs) and `creative` (brand).

```bash
# New tree
./cli/symkit init /path/to/new-app \
  --harness engineering --role engineer --scaffold

# Existing app or library — do not pass --scaffold
./cli/symkit install /path/to/existing \
  --harness engineering --role engineer
```

Roles: `engineer` (`swe`), `materials`.

Existing code, tests, and lockfiles are the source of truth. Do not invent
a second stack. `match-repo` and `write-docs` follow the tree that is here.

One harness per target. To attach product or creative packs, pass `--also`
on purpose.
