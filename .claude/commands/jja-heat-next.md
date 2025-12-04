You are showing the current Job Jockey heat and its next pace(s).

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for heat files in .claude/jji/current/
   - Look for files matching pattern `jjh-b*.md`

2. Branch based on heat count:

   **If 0 heats:**
   - Check .claude/jji/pending/ for pending heats
   - Announce: "No active heat found in .claude/jji/current/"
   - If pending heats exist: "You have N pending heat(s): [list names]"
   - Ask: "Would you like to start a new heat, activate a pending heat, or promote an itch from jjf-future.md?"
   - Stop and wait for user direction

   **If 1 heat:**
   - Read the heat file
   - Display:
     - Heat name (from filename, e.g., "buk-portability" from jjh-b251108-buk-portability.md)
     - Brief summary from Context section (first sentence or goal)
     - Next incomplete pace (first `- [ ]` item) with its description
   - If multiple unchecked paces exist and priority is unclear:
     - Ask: "Which pace should we focus on next?"
   - Otherwise: Ask "Ready to start?" or similar

   **If 2+ heats:**
   - List all heats by name with brief summary
   - Ask: "Which heat would you like to work on?"
   - Wait for selection, then display that heat as in "1 heat" case

3. Example output format:
   ```
   Current heat: **BUK Utility Rename**
   Goal: Rename BCU/BDU/BTU/BVU utilities to use consistent buc/bdu/btu/bvu prefixes

   Next pace: **Update bcu_command.sh internal functions**
     Rename zbcu_* functions to zbuc_*
     (11 internal functions total)

   Ready to start?
   ```

Error handling: If .claude/jji/current/ doesn't exist, announce issue and stop.
