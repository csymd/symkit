# Course workspace

Scaffolded by [symkit](https://github.com/csymd/symkit) (`teaching` harness).

## Layout

| Path | Purpose |
|:-----|:--------|
| `docs/` | Syllabus, module cards, calendars |
| `lectures/` | Teachable / study notes |
| `assignments/` | Graded prompts (submit on the LMS) |
| `data/public/` | Tiny public or synthetic samples |
| `AGENTS.md` | Installed agent defaults |

## Getting started

```bash
# If this repo uses uv:
#   uv sync
# Review agent install (local, gitignored):
ls -a .agents .grok 2>/dev/null || true
```

Graded work is submitted on the **LMS**, not by pushing to this repository,
unless the instructor says otherwise.
