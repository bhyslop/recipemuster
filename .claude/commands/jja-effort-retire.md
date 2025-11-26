You are retiring a completed effort in the Job Jockey system.

Configuration:
- JJ files path: .claude/jji/
- Separate repo: no
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for current effort in .claude/jji/current/
   - If no effort: announce "No active effort to retire" and stop
   - If multiple: ask which one to retire

2. Read the effort file and verify all steps are complete
   - Check that all steps in Pending section are marked [x] or explicitly noted as discarded
   - If incomplete steps found: warn and ask for confirmation

3. Calculate retire date (today's date in YYMMDD format)

4. Show proposed rename:
   - From: jje-bYYMMDD-description.md
   - To: jje-bYYMMDD-rYYMMDD-description.md
   - Ask for approval

5. Once approved:
   - Move file from .claude/jji/current/ to .claude/jji/retired/
   - Rename with retire date

6. Commit:
   ```bash
   git add .claude/jji/current/ .claude/jji/retired/
   git commit -m "JJA: effort-retire - [effort description]"
   ```

7. Report what was done

Error handling: If paths wrong or files missing, announce issue and stop.
