You are adding a new step to the current Job Jockey effort.

Configuration:
- JJ files path: .claude/jji/
- Separate repo: no
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for current effort in .claude/jji/current/
   - If no effort: announce "No active effort" and stop
   - If multiple: ask which one

2. Read the effort file to understand context and existing steps

3. Based on conversation context or ask user:
   - What is the step title?
   - Where should it go in the sequence?
   - Does it need a description? (optional for simple steps)

4. Propose the new step:
   - Title in bold
   - Position in sequence
   - Optional description
   - Mode: manual (default for new steps)
   - Explain reasoning for the placement

5. Wait for approval or amendment

6. Once approved, update the effort file:
   - Add step in Pending section at proposed position
   - Format: `- [ ] **Title**` with optional description below
   - Include `mode: manual` in description

7. Commit:
   ```bash
   git add .claude/jji/current/jje-*.md
   git commit -m "JJA: step-add - [step title]"
   ```

8. Report what was added

Error handling: If paths wrong or files missing, announce issue and stop.
