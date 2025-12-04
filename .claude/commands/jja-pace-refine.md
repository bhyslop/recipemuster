You are helping refine a pace's specification in the current Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for current heat in .claude/jji/current/
   - If no heat: announce "No active heat" and stop
   - If multiple: ask which one

2. Ask which pace to refine (or infer from context)

3. Read the current pace spec and assess its state:
   - Is mode defined? (manual/delegated/unset)
   - Is spec sparse or detailed?

4. Conduct adaptive interview:

   If spec is sparse/new:
   - "Is this a manual pace (you drive) or should we prepare it for delegation (model drives)?"
   - If manual: confirm and done
   - If delegated: continue to step 5

   If spec exists:
   - Show current spec summary
   - "What needs to change?"
   - Focus on the delta

5. For delegated mode, ensure spec covers:
   - Objective: What specifically to achieve?
   - Scope: What files/systems to touch or avoid?
   - Success: How do we know it's done?
   - Failure: What to do if stuck? (stop/report/retry)
   - Model hint: haiku-ok / needs-sonnet / needs-opus

   Ask only for missing elements.

6. Final clarity check (delegated only):
   Read the spec as if you have no prior context. Assess:
   - Objective: clear or ambiguous?
   - Scope: bounded or unclear?
   - Success: measurable or vague?
   - Stuck: know when to stop or might spin?

   If any check fails, explain why and ask clarifying question.
   Loop until all checks pass.

7. Update the pace in the heat file with refined spec

8. Do NOT commit (preparatory work, accumulates until /jja-pace-wrap or /jja-sync)

9. Report what was updated

Error handling: If paths wrong or files missing, announce issue and stop.
