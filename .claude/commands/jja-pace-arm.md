You are arming a pace for autonomous execution.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md

Steps:

1. Check for current heat in .claude/jjm/current/
   - If no heat: announce "No active heat" and stop

2. Identify the pace to arm (from context or ask)

3. Validate the pace spec is sufficient for autonomous execution:
   - Objective: Is it clearly defined?
   - Scope: Is it bounded (what files/areas to touch, what to avoid)?
   - Success criteria: How do we know it worked?
   - Failure behavior: What to do if stuck or blocked?

4. If spec is insufficient:
   - List specific gaps
   - Help user refine the spec
   - Do NOT arm until spec is healthy

5. If spec is sufficient:
   - Recommend model tier (haiku for mechanical, sonnet for judgment, opus for complex)
   - Add `[armed]` marker to the pace title in the heat file
   - Report: "Pace armed and ready to fly"

6. Do NOT execute the pace - arming is validation only

Error handling: If paths wrong or files missing, announce issue and stop.
