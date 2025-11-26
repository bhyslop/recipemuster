You are listing all remaining steps in the current Job Jockey effort.

Configuration:
- JJ files path: .claude/jji/
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for current effort in .claude/jji/current/
   - If no effort: announce "No active effort" and stop
   - If multiple: ask which one

2. Read the effort file and collect all unchecked steps from Pending section

3. Display terse list:
   ```
   Remaining steps (N):
   1. [mode] Step title
   2. [mode] Step title
   ...
   ```

4. If no pending steps:
   - Announce: "No steps remaining! Consider using /jja-effort-retire"

Error handling: If paths wrong or files missing, announce issue and stop.
