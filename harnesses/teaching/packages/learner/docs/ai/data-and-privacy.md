<!--
Copyright (c) 2026, cSYMd
Licensed under Apache 2.0
-->

# Data, privacy, and AI tools

## Rule of thumb

**If you would not email the file to a stranger, do not paste it into a consumer AI tool.**

## Usually OK for external AI tools

- Tiny **public** samples from the course (`data/public/`, synthetic FHIR, toy CSVs)  
- Error messages and **de-identified** snippets you created  
- Your own prose drafts that contain **no** private data  
- Public documentation (FHIR, HL7, ONC pages)  

## Not OK (defaults)

- **MIMIC-IV** or other **credentialed** PhysioNet extracts (even “just a few rows”)  
- Any **PHI** or real patient identifiers  
- Student records, classmates’ full submissions  
- API keys, passwords, `.env` files  
- Non-public institutional extracts under DUA or contract  

## Credentialed project tracks (e.g. MIMIC)

1. Complete required training / credentialing as the course directs.  
2. Keep extracts **outside** git and outside consumer AI tools.  
3. Use AI on **concepts**, **public samples**, or **your code structure**—not on restricted tables.  
4. If the instructor names an **approved** institutional AI path, use only that path for restricted content.  

## Local coding agents (Copilot, Claude Code, Cursor, etc.)

- Same data rules: don’t open restricted folders in a tool that uploads context to the cloud unless approved.  
- Prefer pointing tools at **public** sample paths when exploring.  
- See [tools-overview.md](tools-overview.md).  

## If unsure

Ask the instructor **before** pasting. “I didn’t know” is harder to fix after a paste into a third-party system.

---

*Part of the teaching learner pack.*
