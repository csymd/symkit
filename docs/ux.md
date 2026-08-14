# UX (v1)

CLI-first. No web app.

## Flow: create a workspace

1. Where — target directory
2. Which harness — teaching / research / ai
3. Which role — from that harness
4. Scaffold? — workspace files if the tree is new
5. Adapters — default grok
6. Preview — always printed
7. Write — never commit
8. Next steps — `cd` + `git status` + private-pack reminder

Interactive `init` asks for missing pieces when stdin is a TTY.
Scripts should pass `--yes` and all flags.

## Principles

- Preview before write
- Last pack wins for `AGENTS.md`; trees merge
- Private packs are loud
- Target `.gitignore` is updated, not replaced
- One harness per target (warn if a second is installed)
- No network required
