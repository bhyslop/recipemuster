# /jja-effort-retire: Move completed effort to retired with datestamp

You are helping retire a completed Job Jockey effort by moving it to the retired directory with a datestamp added to the filename.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`
- Separate repo: no

Steps:

1. Check `.claude/jji/current/` for active effort files
   - If none found: Report "No active efforts to retire"
   - If one found: Proceed to step 2
   - If multiple found: Ask user which effort to retire, then proceed to step 2

2. Verify the effort is complete:
   - Read the effort file
   - Check that all steps are marked complete `- [x]` or explicitly discarded
   - If incomplete steps remain: Ask user "This effort still has pending steps. Are you sure you want to retire it?"
   - Wait for user confirmation before proceeding

3. Prepare to rename and move:
   - Extract today's date in YYMMDD format (e.g., 251110 for 2025-11-10)
   - Current filename: `jje-description.md`
   - New filename: `jje-YYMMDD-description.md` (add datestamp before .md)
   - Example: `jje-buk-rename.md` → `jje-251110-buk-rename.md`

4. Perform the retirement:
   - Move file from `.claude/jji/current/` to `.claude/jji/retired/`
   - Rename file to include datestamp
   - Execute git commands:
     ```
     git add .claude/jji/current/ .claude/jji/retired/
     git commit -m "JJA: effort-retire - Archived [effort-name] as jje-YYMMDD-[effort-name]"
     ```

5. Report the retirement:
   - Show old and new file paths
   - Confirm successful archival

Error handling: If files are missing or paths are wrong, announce the issue and stop.
