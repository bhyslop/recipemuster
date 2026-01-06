You are helping mark a pace complete in the current Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md

Steps:

1. Check for current heat in .claude/jjm/current/
   - If no heat: announce "No active heat" and stop
   - If multiple: ask which one
   - Note the heat's silks (kebab-case description from filename)

2. Ask which pace to mark done (or infer from context)

3. Summarize the pace completion based on chat context
   - Focus on WHAT changed, not WHERE (no line numbers - they go stale)
   - Include: file names, function/section names, nature of change
   - Keep brief: one sentence, under 100 chars if possible

4. Update the heat file in .claude/jjm/current/:
   - Move the pace to ## Done section
   - Replace description with brief summary
   - Bold the next pace in ## Remaining to mark it current (if any)

5. Append WRAP entry to steeplechase (.claude/jjm/current/jjc_*.md):
   - Create steeplechase file if it doesn't exist (jjc_bYYMMDD-[silks].md matching heat)
   - Append entry in this format:
   ```markdown
   ---
   ### YYYY-MM-DD HH:MM - [pace-silks] - WRAP
   **Outcome**: [summary from step 3]
   ---
   ```

6. Commit JJ state (this repo only, no push):
   ```bash
   git add .claude/jjm/current/
   git commit -m "JJA: pace-wrap - [brief description]"
   ```

7. Report what was done

8. If there is a next pace (now first in ## Remaining):
   - Read the pace description and any files/context it references
   - Analyze what the work entails
   - Propose a concrete approach (2-4 bullets)
   - Append APPROACH entry to steeplechase:
     ```markdown
     ---
     ### YYYY-MM-DD HH:MM - [pace-silks] - APPROACH
     **Proposed approach**:
     - [bullet 1]
     - [bullet 2]
     ---
     ```
   - Ask: "Ready to proceed with this approach?"
   - On approval: begin work directly (no /jja-heat-saddle needed)

   If no next pace: announce "All paces complete - ready to retire heat?"

Error handling: If files missing or paths wrong, announce issue and stop.
