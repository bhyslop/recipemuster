You are helping mark a pace complete in the current Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md

Steps:

1. Check for current heat in .claude/jjm/current/
   - If no heat: announce "No active heat" and stop
   - If multiple: ask which one

2. Ask which pace to mark done (or infer from context)

3. Summarize the pace completion based on chat context
   - Focus on WHAT changed, not WHERE (no line numbers - they go stale)
   - Include: file names, function/section names, nature of change
   - Keep brief: one sentence, under 100 chars if possible

4. Show proposed summary and ask for approval

5. Update the heat file in .claude/jjm/current/:
   - Number the pace and move to ## Done section
   - Replace description with brief summary
   - Move next pace from ## Remaining to ## Current (if any)

6. Commit JJ state (this repo only, no push):
   ```bash
   git add .claude/jjm/current/jjh-*.md
   git commit -m "JJA: pace-wrap - [brief description]"
   ```

7. Report what was done

Error handling: If files missing or paths wrong, announce issue and stop.
