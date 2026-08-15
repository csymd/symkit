<!--
Copyright (c) 2026, PalEm Dynamics LLC
Licensed under the Apache License, Version 2.0.
-->

# Default student AI workflow

Use this loop on most assignments, labs, and project milestones.  
Adjust only if your syllabus or the LMS says otherwise.

## The loop (short)

```text
1. Understand the ask
2. Attempt first (without AI, or with minimal lookup)
3. Assist (targeted AI help)
4. Verify (you check; don’t trust the model)
5. Own (rewrite, explain, disclose if required)
6. Submit
```

## Step by step

### 1. Understand the ask

- Read the assignment end-to-end (objectives, deliverables, data rules).  
- Note **what “done” looks like** (files, sections, lab outputs).  
- Flag anything about **data**, **AI disclosure**, or **collaboration**.  

### 2. Attempt first

Spend real effort before heavy AI use:

- Sketch your approach (bullets, diagram, pseudocode).  
- Run the starter code / open the sample data.  
- Write a first answer even if incomplete.  

**Why:** The goal is *your* skill. AI is a tutor and editor, not the author of record.

### 3. Assist (targeted, not “do the whole lab”)

Good asks:

- “Explain this error message in plain language.”  
- “I think my FHIR subject link is wrong—what should I check?”  
- “Quiz me on TEFCA vs HIE models.”  
- “Compare my outline to this rubric; what’s missing?” *(after you drafted)*  

Weaker asks:

- “Write the entire lab submission.”  
- “Generate a perfect analysis notebook for MIMIC.”  
- “Summarize this PDF so I don’t have to read it” *(when reading is the point)*  

See [prompting.md](prompting.md).

### 4. Verify

Before you keep AI output:

| Check | Question |
|:--|:--|
| **Runs** | Does the code run on *your* machine with *course* data? |
| **True** | Are claims checkable (standards, definitions, citations)? |
| **Yours** | Can you explain each step without the chat open? |
| **Allowed** | Did you avoid restricted data and follow disclosure rules? |

If you cannot re-create the result, treat the AI output as **scratch**, not submission.

### 5. Own

- Rewrite in your voice.  
- Delete scaffolding you don’t understand.  
- Add disclosure if required ([disclosure.md](disclosure.md)).  
- For projects: keep a short note of tools used (tool + purpose is enough).  

### 6. Submit

- On the **LMS** (unless the course says otherwise).  
- Course materials repos are for **materials**, not graded dropboxes.  

## Suggested time split (graduate assignment)

| Phase | Rough share |
|:--|:--|
| Read + attempt | 50–70% |
| AI assist + debug | 15–30% |
| Verify + polish + disclose | 15–20% |

If assist is most of your time, you are probably outsourcing the learning.

## Course materials paths (typical)

| You need… | Look in… |
|:--|:--|
| What to turn in | `assignments/`, the LMS |
| Class design / outcomes | `docs/modules/` |
| Deeper notes | `lectures/` |
| Narrative study hub (if any) | course `site/` or published Pages URL |
| AI policy for *this* course | `docs/ai-what-to-expect.md` + syllabus |

## When you’re stuck

1. Re-read the assignment and any “success criteria.”  
2. Try a smaller experiment (one function, one table, one paragraph).  
3. Ask AI a **narrow** question with context (error + what you tried).  
4. Office hours / discussion with a **specific** question—not “do it for me.”  

---

*Part of the teaching learner pack. Syllabus and LMS override this guide.*
