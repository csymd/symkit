# Research harness

Experiment tracking, reproducibility, and paper-oriented layout for study
repositories (grants, analyses, methods papers).

```bash
# New study (folder stubs)
./cli/symkit init /path/to/new-study --harness research --role researcher --scaffold

# Existing lab repo — packs only; do not pass --scaffold unless you want stubs
./cli/symkit install /path/to/existing-study --harness research --role researcher
```

`study-layout` maps the tree that is already here. It does not invent a
second `analysis/` next to live code. Written aims / protocol / SAP stay
the source of scientific truth.
