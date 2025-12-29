You are flying an armed pace - executing it autonomously.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md

Steps:

1. Check for current heat in .claude/jjm/current/
   - If no heat: announce "No active heat" and stop
   - Note the heat's silks (kebab-case description from filename)

2. Identify the pace to fly (from context or ask)

3. Check the pace is armed:
   - Look for `[armed]` marker in the pace title
   - If not armed: refuse with "This pace is not armed - use /jja-pace-arm first"

4. Present the spec and begin execution:
   ```
   Flying pace: **[title]**
   Objective: [objective]
   Scope: [scope]
   Success: [criteria]
   On failure: [behavior]
   ```

5. Execute the pace based solely on the spec
   - If target repo != `.`, work in target repo directory: .
   - Work from the spec, not from conversation context
   - Stay within defined scope
   - Stop when success criteria met OR failure condition hit

6. Report outcome:
   - Success: what was accomplished, evidence of success criteria
   - Failure: what was attempted, why stopped, what's needed
   - Modified files: list absolute paths

7. Append FLY entry to steeplechase (.claude/jjm/current/jjc_*.md):
   ```markdown
   ---
   ### YYYY-MM-DD HH:MM - [pace-silks] - FLY
   **Spec**: [brief summary]
   **Execution trace**: [key actions taken]
   **Result**: success | failure | partial
   **Modified files**: [list]
   ---
   ```

8. Do NOT auto-complete the pace. User decides via /jja-pace-wrap
   Work is NOT auto-committed. User can review and use /jja-notch.

Error handling: If paths wrong or files missing, announce issue and stop.
