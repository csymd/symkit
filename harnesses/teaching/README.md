# Teaching harness

Course materials, staff agents, and optional learner literacy.

## Roles

| Role | Packs | Notes |
|:-----|:------|:------|
| `instructor` / `faculty` | shared + staff + instructor | Full design + evaluation toolkit |
| `ta` | shared + staff + ta | Evaluation + engagement; prunes instructor-only skills |
| `learner` | shared + learner | Student docs + study skills |
| `materials` | shared | Student-safe materials defaults only |

Install:

```bash
./cli/symkit install /path/to/course --harness teaching --role instructor
./cli/symkit init /path/to/new-course --harness teaching --role instructor --scaffold
```

Do not commit `staff`, `instructor`, or `ta` packs into student-visible trees.
Learner docs under `docs/` are safe to commit; `.agents/` usually is not.

Custom course overlays: see `examples/teaching-overlay/`.
