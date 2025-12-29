You are dispatching a JJ-aware git commit (notch).

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md
- Brand: 603

Steps:

1. Identify active heat from conversation context
   - The heat should be unambiguous from prior conversation (e.g., heat-saddle, pace-wrap)
   - If heat context is clear: proceed with that heat
   - If no heat context or ambiguous: fail with "No heat context - run /jja-heat-saddle first"
   - Do NOT fall back to filesystem checks or offer choices

2. Extract heat silks from context
   - Heat filename pattern: `jjh_bYYMMDD-SILKS.md`
   - Example: `jjh_b251227-cloud-first-light.md` → silks = `cloud-first-light`

3. Extract current pace from context (optional)
   - If a specific pace is being worked: extract silks from title (`**Fix quota bug**` → `fix-quota-bug`)
   - If no specific pace (general heat work, editing heat file): proceed without pace

4. Check for new (untracked) files:
   ```bash
   git ls-files --others --exclude-standard
   ```
   - If any output: announce "New files detected - stage manually first" and stop
   - New files require explicit user staging; notch only handles modified/deleted

5. Dispatch notcher agent in background:
   - Use Task tool with subagent_type='general-purpose', model='haiku', run_in_background=true
   - Prompt template (substitute HEAT and optionally PACE at runtime):
     ```
     Git commit agent. Heat=HEAT, Pace=PACE (or none), Brand=603
     Format:
       With pace: [jj:603][HEAT/PACE] Summary
       Without pace: [jj:603][HEAT] Summary
       Line 1 under 72 chars, imperative tense
       Line 2: blank
       Lines 3+: - Detail bullet per logical change
     Process: git add -u, git diff --cached, write message, git commit -m "...", git push, report hash.
     ```

6. Announce "Notch dispatched" and return immediately
   - Do NOT wait for agent completion
   - Do NOT re-engage with pace (user continues naturally)
   - User can check background task status if needed

Error handling: If heat context missing or ambiguous, fail immediately - do not attempt recovery.
