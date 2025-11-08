# /jja-doctor: Validate Jaunt Jockey setup

You are validating that Jaunt Jockey is correctly installed and all files are present.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Validation checklist:

1. Kit file exists: `Tools/jjk/Jaunt-Jockey-Kit.md`
   - Report: ✓ or ✗ with path

2. JJ files directory exists: `.claude/jji/`
   - Report: ✓ or ✗

3. Required files present:
   - `.claude/jji/jjf-future.md` (✓ or ✗)
   - `.claude/jji/jjs-shelved.md` (✓ or ✗)
   - `.claude/jji/current/` directory (✓ or ✗)
   - `.claude/jji/retired/` directory (✓ or ✗)

4. Commands exist:
   - `.claude/commands/jja-*.md` (list which ones are present)

5. Current efforts:
   - List any `jje-*.md` files in `.claude/jji/current/`

6. Summary:
   - "Jaunt Jockey is properly installed" or "Issues found:" with details

Error handling: Report any missing files or paths. Do not attempt to auto-fix.
