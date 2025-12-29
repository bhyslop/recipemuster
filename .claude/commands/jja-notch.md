You are dispatching a JJ-aware git commit (notch).

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md
- Brand: 602

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

4. Check for new (untracked) files:
   ```bash
   git ls-files --others --exclude-standard
   ```
   - If any output: announce "New files detected - stage manually first" and stop
   - New files require explicit user staging; notch only handles modified/deleted

5. Dispatch to notcher agent:
   - Use Task tool with subagent_type='general-purpose' and model='haiku'
   - Prompt template (substitute HEAT and PACE at runtime):
     ```
     Git commit agent. Heat=HEAT, Pace=PACE, Brand=602
     Format:
       Line 1: [jj:602][HEAT/PACE] Summary (imperative, under 72 chars)
       Line 2: blank
       Lines 3+: - Detail bullet per logical change
     Process: git add -u, git diff --cached, write message, git commit -m "...", report hash.
     ```

6. Report result from agent (commit hash or failure)

7. Push to remote:
   ```bash
   git push
   ```

8. Re-engage with current pace:
   - If there is a current pace (first in ## Remaining):
     - Read the pace description and any files/context it references
     - Analyze what the work entails
     - Propose a concrete approach (2-4 bullets)
     - Ask: "Ready to proceed with this approach?"
     - On approval: begin work directly
   - If no current pace:
     - Announce "All paces complete - ready to retire heat?"

Error handling: If heat/pace not found, explain and stop.
