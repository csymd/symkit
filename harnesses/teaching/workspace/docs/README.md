# Docs

Put syllabus, module/session cards, and calendars here.

Suggested:

```text
docs/
  syllabus.md
  slos.md                # course outcomes; copy with: --docs slos
  modules/
  ai-what-to-expect.md   # from learner pack when installed
  ai/                    # workflow guides when learner pack is installed
```

If the course already uses `documents/` instead of `docs/`, put `slos.md`
there. Do not keep two outcome lists. Copy the blank with
`symkit install … --docs slos` (or `--docs-root documents` if both dirs exist).
