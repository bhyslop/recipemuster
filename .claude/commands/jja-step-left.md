# /jja-step-left: List all remaining steps

You are helping show a terse list of all remaining (incomplete) steps in the current Job Jockey effort.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Steps:

1. Check if there is a current effort in `.claude/jji/current/`
   - If no effort file found: Announce "No active effort" and suggest using `/jja-next`
   - If multiple effort files: Ask which one to examine
   - If one effort file: Proceed to step 2

2. Read the effort file and extract all incomplete steps
   - Find all checklist items with `- [ ]` (not checked)
   - Extract step titles only (skip descriptions)

3. Display to the user in format:
   ```
   Remaining steps (N):
   1. First step title
   2. Second step title
   3. Third step title
   ```

4. If no incomplete steps: Report "All steps complete! Consider retiring this effort with `/jja-effort-retire`"

5. Report what was found

Error handling: If files are missing or paths are wrong, announce the issue and stop.
