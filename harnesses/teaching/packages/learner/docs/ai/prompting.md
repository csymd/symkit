<!--
Copyright (c) 2026, cSYMd
Licensed under Apache 2.0
-->

# Prompting for learning (not outsourcing)

Good prompts get **help you can verify**. Weak prompts get polished text you cannot defend.

## Pattern that works

```text
Context: course / assignment / what you already tried
Goal: what you need (explain / debug / critique / quiz)
Constraints: no restricted data; don’t write the full submission
Ask: one clear question
```

### Example (debug)

> I’m in this lab. I loaded `sample-observation.json` with Python’s `json` module.  
> I tried to print `obs["code"]["coding"][0]["code"]` and got KeyError.  
> Here’s the structure I see for keys under `code`: …  
> What should I check next? **Don’t write the full lab answers.**

### Example (concept)

> Explain TEFCA in two paragraphs for a graduate student who already knows what an EHR is.  
> Then give me three self-check questions **without answers**.

### Example (rubric self-check)

> Here is my Path B write-up (draft).  
> Rubric lenses: accuracy, vocabulary, limitations, clarity.  
> List gaps only—**do not rewrite the whole piece.**

## Do / don’t

| Do | Don’t |
|:--|:--|
| Paste **errors** and **small** code slices | Paste entire solutions from classmates |
| Name the **assignment goal** | “Just give me the A version” |
| Ask for **critiques** after you draft | Ask for a first draft you never attempt |
| Request **questions** to test yourself | Request fabricated citations or fake results |
| Use **public** sample data | Paste MIMIC / PHI / credentials |

## After the model answers

1. **Predict** what the fix should do.  
2. **Apply** it yourself in your environment.  
3. **Explain** the fix in one sentence in your notes or submission.  
4. If you can’t explain it, **don’t ship it**.  

## Prompts that raise integrity risk

- “Write my full lab / project / paper.”  
- “Make this sound more academic” on work you didn’t do.  
- “Invent references that support X.”  
- “Hide that I used AI” when disclosure is required.  

See [when-not-to.md](when-not-to.md) and [disclosure.md](disclosure.md).

---

*Part of the teaching learner pack.*
