# Instructor package

Adds the **instructor** agent and instructor-only *rules*. Skills
(`course-prep`, `week-plan`, `assignment-review`, `accessibility-review`,
and shared `write-gherkin` / `evaluate-content`) live in
`core/library/skills/` and are assigned in `catalog.yaml`.

```bash
./cli/symkit install /path/to/course --harness teaching --role instructor
```

| Contents | Path |
|:---------|:-----|
| Instructor agent | `.agents/agents/instructor.md` |
| Instructor-only rules | `.agents/rules/` |

Do not install on TA-only machines if you want pack-level separation from
course design tools.
