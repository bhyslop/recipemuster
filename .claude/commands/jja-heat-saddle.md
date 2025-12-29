---
argument-hint: [silks]
description: Select active heat
---

Select the active Job Jockey heat and show recommended approach.

Use cases:
- Session start (cold start)
- Switching between multiple active heats
- Explicit heat status review

Note: After /jja-pace-wrap or /jja-notch, you do NOT need this command -
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
   - Display current pace (first bolded item in ## Remaining)
   - Read the pace description and any files/context it references
   - Analyze what the work entails
   - Propose a concrete approach (2-4 bullets)
   - Append APPROACH entry to steeplechase (.claude/jjm/current/jjc_*.md):
     - Create steeplechase file if it doesn't exist (jjc_bYYMMDD-[silks].md)
     - Entry format:
       ```markdown
       ---
       ### YYYY-MM-DD HH:MM - [pace-silks] - APPROACH
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
