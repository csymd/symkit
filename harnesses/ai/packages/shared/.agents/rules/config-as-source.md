<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Config as source of truth

- What can run is whatever `config.yaml` (or the repo’s named config) lists.
- Adding a model, arm, or phase means editing config first, then scripts.
- Do not hardcode model names or seeds in scripts when they belong in config.
- Filter runs by documented phase tags; do not invent extra arms in chat.
