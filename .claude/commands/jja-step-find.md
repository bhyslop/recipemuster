You are finding the next incomplete step in the current Job Jockey effort.

Configuration:
- JJ files path: .claude/jji/
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for current effort in .claude/jji/current/
   - If no effort: announce "No active effort" and stop
   - If multiple: ask which one

2. Read the effort file and find the first unchecked step in Pending section

3. Display the step:
   - Title (in bold)
   - Mode (manual or delegated)
   - Full description

4. If no pending steps found:
   - Announce: "All steps complete! Consider using /jja-effort-retire"

Error handling: If paths wrong or files missing, announce issue and stop.
