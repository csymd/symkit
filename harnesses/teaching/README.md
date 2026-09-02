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
./cli/symkit install /path/to/course --harness teaching --role instructor --docs slos
```

`--docs slos` copies a faculty-owned SLO blank into `docs/` or `documents/`
(detected; pass `--docs-root` if both exist). It does not overwrite unless
`--force`. `symkit show teaching` lists template ids.

Do not commit `staff`, `instructor`, or `ta` packs into student-visible trees.
Learner docs under `docs/` are safe to commit; `.agents/` usually is not.
See the installed `course-materials.md` rule for the commit list.

Course-specific extras: copy `examples/teaching-overlay/` into
`harnesses/teaching/packages/<id>/`, register it in `catalog.yaml`, then
`--also <id>`. Keep overlays short. Overlay packs are not first-class local
paths yet — they must live in the catalog.
