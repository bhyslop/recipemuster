You are retiring a completed Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md

Steps:

1. Check for heat files in .claude/jjm/current/
   - If 0 heats: announce "No active heat to retire" and stop
   - If 2+ heats: ask which one to retire

2. Read the heat file and verify completion:
   - Check for any incomplete paces (`- [ ]` items)
   - If incomplete paces exist:
     - List them
     - Ask: "These paces are incomplete. Mark them as discarded, or continue working?"
     - If user wants to discard: mark them with `- [~]` prefix and note "(discarded)"
     - If user wants to continue: stop retirement process

3. Determine filenames:
   - Current filename pattern: `jjh_bYYMMDD-description.md`
   - Extract the begin date (bYYMMDD) and description
   - Generate retire date: today's date as rYYMMDD
   - New filename: `jjh_bYYMMDD-rYYMMDD-description.md`

   Example:
   - Before: `jjh_b251108-buk-rename.md`
   - After: `jjh_b251108-r251127-buk-rename.md`

4. Move the file:
   ```bash
   git mv .claude/jjm/current/jjh_b251108-buk-rename.md .claude/jjm/retired/jjh_b251108-r251127-buk-rename.md
   ```

5. Commit the retirement (JJ state repo only, no push):
   ```bash
   git commit -m "JJA: heat-retire - [heat description]"
   ```

6. Report completion:
   ```
   Retired heat: **BUK Rename**
   - Began: 2025-11-08
   - Retired: 2025-11-27
   - File: .claude/jjm/retired/jjh_b251108-r251127-buk-rename.md
   ```

7. Offer next steps:
   - "Would you like to start a new heat or promote an itch from jji_itch.md?"

Error handling: If paths wrong or files missing, announce issue and stop.
