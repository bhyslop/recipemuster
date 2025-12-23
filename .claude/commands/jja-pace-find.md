You are showing the current pace from the active Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for heat files in .claude/jji/current/
   - If 0 heats: announce "No active heat" and stop
   - If 2+ heats: ask which one

2. Read the heat file

3. Find the current pace:
   - Look for the ## Current section
   - Extract the pace title (bold text) and any working notes

4. Determine the mode:
   - Look for `mode: manual` or `mode: delegated` in the pace spec
   - Default to `manual` if not specified

5. Display:
   ```
   Current pace: **[title]** [mode]

   [working notes if any]
   ```

Error handling: If no current pace found, announce "No current pace - check ## Remaining for next pace"
