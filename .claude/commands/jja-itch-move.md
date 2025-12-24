You are moving a Job Jockey itch between locations.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Identify the itch to move (from context or ask)

2. Determine current location:
   - .claude/jji/jjf-future.md (Future)
   - .claude/jji/jjs-shelved.md (Shelved)

3. Ask for destination:
   - Future (jjf-future.md) - worthy of doing
   - Shelved (jjs-shelved.md) - setting aside
   - Promote to Heat - create new jjh-*.md file

4. Execute the move:

   **If moving between Future/Shelved:**
   - Remove from source file
   - Add to destination file
   - Preserve any context/notes

   **If promoting to Heat:**
   - Remove from source file
   - Create new heat file: .claude/jji/current/jjh-bYYMMDD-[description].md
   - Use itch as initial context
   - Create initial pace from itch objective
   - Ask user to confirm heat structure

5. Do NOT commit (accumulates until /jja-sync)

6. Report what was moved

Error handling: If itch not found or files missing, announce issue and stop.
