You are resuming the current Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for heat files in .claude/jji/current/
   - Look for files matching pattern `jjh-b*.md`

2. Branch based on heat count:

   **If 0 heats:**
   - Check .claude/jji/pending/ for pending heats
   - Announce: "No active heat found"
   - If pending heats exist: "You have N pending heat(s): [list names]"
   - Ask: "Would you like to start a new heat, activate a pending heat, or promote an itch?"
   - Stop and wait for user direction

   **If 1 heat:**
   - Read the heat file
   - Display:
     - Heat name (from filename)
     - Brief summary from Context section
     - Current pace (from ## Current section)
   - Ask "Ready to continue?" or similar

   **If 2+ heats:**
   - List all heats by name
   - Ask: "Which heat would you like to work on?"
   - Wait for selection, then display as above

3. Example output format:
   ```
   Resuming heat: **BUK Utility Rename**

   Current pace: **Update internal functions**

   Ready to continue?
   ```

Error handling: If .claude/jji/current/ doesn't exist, announce issue and stop.
