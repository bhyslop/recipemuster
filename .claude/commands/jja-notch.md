You are dispatching a JJ-aware git commit (notch).

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md

Steps:

1. Verify active heat exists in .claude/jjm/current/
   - Look for files matching `jjh_b*.md`
   - If no heat: announce "No active heat - use regular git commit" and stop
   - If multiple heats: ask which one

2. Extract heat silks from filename
   - Pattern: `jjh_bYYMMDD-SILKS.md`
   - Example: `jjh_b251227-cloud-first-light.md` → silks = `cloud-first-light`

3. Find current pace (first bold item in ## Remaining)
   - Extract pace silks from the bolded title
   - Example: `**Fix quota bug**` → silks = `fix-quota-bug`
   - If no current pace: announce "No active pace" and stop

4. Get brand from Job Jockey Configuration in CLAUDE.md

5. Check for staged changes
   - Run `git diff --cached --stat`
   - If nothing staged: ask "Stage all changes?" or stop

6. Dispatch to jjsa-notcher agent:
   - Use Task tool with subagent_type='jjsa-notcher'
   - Use model='haiku' for speed
   - Prompt: "Heat: [silks], Pace: [silks], Brand: [brand]. Commit staged changes."
   - Can run in background if other work continues

7. Report result from agent

Error handling: If heat/pace not found, explain and stop.
