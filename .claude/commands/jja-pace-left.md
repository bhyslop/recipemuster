You are listing all remaining paces in the current Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for heat files in .claude/jji/current/
   - If 0 heats: announce "No active heat" and stop
   - If 2+ heats: ask which one

2. Read the heat file

3. Collect remaining paces:
   - Current pace from ## Current section
   - Future paces from ## Remaining section

4. For each pace, determine mode:
   - Look for `mode: manual` or `mode: delegated`
   - Default to `manual` if not specified

5. Display terse list:
   ```
   Remaining paces (N):
   1. [mode] Current pace title
   2. [mode] Next pace title
   3. [mode] Another pace title
   ...
   ```

Error handling: If no remaining paces, announce "All paces complete - ready to retire heat?"
