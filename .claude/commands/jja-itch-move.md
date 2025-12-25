You are moving a Job Jockey itch between locations.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md

Steps:

1. Identify the itch to move (from context or ask)

2. Determine current location:
   - .claude/jjm/jji_itch.md (Itches)
   - .claude/jjm/jjs_scar.md (Scars)

3. Ask for destination:
   - Itch (jji_itch.md) - future work
   - Scar (jjs_scar.md) - closing with lessons learned
   - Promote to Heat - create new heat file

4. Execute the move:

   **If moving to Scar:**
   - Remove section from jji_itch.md
   - Add section to jjs_scar.md
   - Append **Closed**: [reason] and Learned: [lesson] lines

   **If restoring from Scar to Itch:**
   - Remove section from jjs_scar.md
   - Add section to jji_itch.md
   - Remove the Closed/Learned lines

   **If promoting to Heat:**
   - Remove section from source file
   - Create new heat file: .claude/jjm/current/jjh_bYYMMDD-[description].md
   - Use itch as initial context
   - Create initial pace from itch objective
   - Ask user to confirm heat structure

5. Do NOT commit (accumulates until /jja-sync)

6. Report what was moved

Error handling: If entry not found or files missing, announce issue and stop.
