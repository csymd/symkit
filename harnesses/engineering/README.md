# Engineering harness

Software engineering for application and library repos. Separate from
`product` (specs) and `creative` (brand).

```bash
./cli/symkit init /path/to/new-app \
  --harness engineering --role engineer --scaffold
```

Roles: `engineer` (`swe`), `materials`.

Existing code, tests, and lockfiles are the source of truth. Do not invent
a second stack.

One harness per target. To attach product or creative packs, pass `--also`
on purpose.
