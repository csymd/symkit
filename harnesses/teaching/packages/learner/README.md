# Learner package

Student-safe AI literacy plus optional **learner** agent.

| Path | Purpose |
|:--|:--|
| `docs/ai-what-to-expect.md` | Human policy handout |
| `docs/ai/*` | Workflow, prompting, disclosure, data |
| `AGENTS.md` | Optional student agent defaults |
| `.agents/agents/learner.md` | Learner agent |
| `.agents/skills/` | create-and-revise-docs, check-understanding, study-plan, lab-tutor |

Install:

```bash
./cli/symkit install /path/to/course --harness teaching --role learner
```

Commit the docs. Usually do not commit `.agents/`.
