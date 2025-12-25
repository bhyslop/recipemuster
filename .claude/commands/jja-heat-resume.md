You are resuming the current Job Jockey heat.

Use this command for:
- Starting a new Claude Code session (cold start)
- Explicitly reviewing full heat status
- Switching between multiple active heats

Note: After /jja-pace-wrap or /jja-sync, you do NOT need this command -
those commands automatically analyze and propose the next pace.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md

Steps:

1. Check for heat files in .claude/jjm/current/
   - Look for files matching pattern `jjh_b*.md`

2. Branch based on heat count:

   **If 0 heats:**
   - Announce: "No active heat found"
   - Ask: "Would you like to start a new heat or promote an itch?"
   - Stop and wait for user direction

   **If 1 heat:**
   - Read the heat file
   - Note the heat's silks (kebab-case description from filename)
   - Display heat name and brief context summary
   - Display current pace (from ## Current section) with mode
   - Read the pace description and any files/context it references
   - Analyze what the work entails
   - Propose a concrete approach (2-4 bullets)
   - Append APPROACH entry to steeplechase (.claude/jjm/current/jjc_*.md):
     - Create steeplechase file if it doesn't exist (jjc_bYYMMDD-[silks].md)
     - Entry format:
       ```markdown
       ---
       ### YYYY-MM-DD HH:MM - [pace-silks] - APPROACH
       **Mode**: manual | delegated
       **Proposed approach**:
       - [bullet 1]
       - [bullet 2]
       ---
       ```
   - Ask: "Ready to proceed with this approach?"
   - On approval: begin work directly

   **If 2+ heats:**
   - List all heats by name
   - Ask: "Which heat would you like to work on?"
   - Wait for selection, then proceed as above

Error handling: If .claude/jjm/current/ doesn't exist, announce issue and stop.
