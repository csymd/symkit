# Instructor package

Adds the **instructor** agent and instructor-only skills. Install with `staff`
(and usually `shared`).

```bash
./cli/symkit install /path/to/course --harness teaching --role instructor
```

| Contents | Path |
|:---------|:-----|
| Instructor agent | `.agents/agents/instructor.md` |
| `course-prep` | `.agents/skills/course-prep/` |
| `week-plan` | `.agents/skills/week-plan/` |
| `assignment-review` | `.agents/skills/assignment-review/` |

Do not install on TA-only machines if you want pack-level separation from
course design tools.
