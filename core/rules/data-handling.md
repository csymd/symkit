<!--
Copyright (c) 2026, cSYMd
Licensed under Apache 2.0
-->

# Data handling

Domain-agnostic defaults. A harness may add stricter rules.

## Allowed in-repo

- Tiny public or synthetic samples under `data/public/` (or the harness equivalent).
- Documentation of *how* to obtain restricted data — not the data itself.

## Never commit

- Credentialed, DUA-restricted, or clinical extracts
- Real patient, student, or participant identifiers
- Large private dumps (`.duckdb`, restricted `.parquet`, raw device stores)
- API keys, `.env` secrets, tokens

## Local extracts

- Prefer paths **outside** the git root.
- If a local path under the repo is used temporarily, ensure `.gitignore` covers it (`data/private/`, etc.).
- Scripts should document expected external paths; do not hardcode secrets.

## Sharing

- Do not upload restricted files to unapproved cloud tools or public gists.
- Do not ask agents to “store” or “check in” restricted data for convenience.
