You are showing the current effort and its next step(s) in the Job Jockey system.

Configuration:
- JJ files path: .claude/jji/
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for active efforts in .claude/jji/current/
   - Count the number of jje-*.md files

2. Based on count:

   **0 efforts:**
   - Announce: "No active efforts found"
   - Ask: "Would you like to start a new effort or promote an itch from Future?"
   - Stop

   **1 effort:**
   - Read the effort file
   - Display:
     - Effort name (from filename description)
     - Brief summary/gesture from Context section
     - Next incomplete step(s) with title and description
   - If multiple pending steps or priority unclear:
     - Ask: "Which step should we focus on next?"
   - Otherwise ask: "Ready to start?"

   **2+ efforts:**
   - List all efforts by name
   - Ask: "Which effort would you like to work on?"
   - Once selected, display that effort with next step(s) as above

3. Do not automatically start work - wait for user direction

Error handling: If paths wrong or files missing, announce issue and stop.
