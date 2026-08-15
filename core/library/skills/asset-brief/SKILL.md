---
name: asset-brief
description: >
  Write a brief for image, deck, or video generation — subject, constraints,
  and do-nots. Use before Imagine, pptx, or similar. Does not produce the
  asset. Triggers: asset brief, image brief, slide brief, art direction,
  asset-brief, /asset-brief.
---

<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Asset brief

Direction for a vendor tool. Not the PNG, PDF, or MP4.

## Steps

1. What is needed (still, deck, clip) and where it will live.
2. Subject, audience, aspect, and must-include facts from product truth.
3. Voice/visual constraints from `docs/brand/` if present.
4. Explicit **do not** (invent logos, fake UI metrics, real people unasked).
5. If the human then wants the asset and a vendor skill exists (`imagine`,
   `pptx`, `design`), point at it. Do not reimplement those pipelines.

## Do not

- Generate the asset in this skill.
- Invent brand marks or screenshots of features that do not exist.

## Output

```markdown
# Brief — <name>
- Format / size:
- Use (web, talk, social):
- Subject:
- Must include:
- Do not:
- Brand refs (paths):
```
