# Performance harness

Combined **exercise physiology** and **biomechanics** for human-performance
repos (training, testing, movement analysis).

```bash
./cli/symkit init /path/to/new-program \
  --harness performance --role coach --scaffold
```

Roles: `coach` (`performance-coach`), `materials`.

The harness is the domain (`performance`). The persona is `coach` — not a
life coach, coding coach, or clinician.

Written program and recorded trials are the source of truth. If the repo
documents [SymWorx](https://github.com/symworx/symworx) or another kernel,
wrap it — do not reimplement filters or inverse dynamics.
