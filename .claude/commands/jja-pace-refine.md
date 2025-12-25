You are helping refine a pace's specification in the current Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md

Steps:

1. Check for current heat in .claude/jjm/current/
   - If no heat: announce "No active heat" and stop
   - If multiple: ask which one

2. Identify which pace to refine (infer from context or ask)

3. Read the pace and assess delegatability:
   - Mechanical vs judgment: Are steps explicit or require decisions?
   - Scope clarity: Are boundaries well-defined?
   - Verifiability: Can success be checked objectively?
   - Risk: What happens if it goes wrong?

4. Make a recommendation with brief rationale:
   ```
   Refining pace: **[title]**

   Current state: [summary of what's defined]

   **Recommendation: [delegated (model-hint) | manual]**
   - [1-3 bullet points explaining why]

   Accept recommendation, or override?
   ```

5. If user accepts delegated, ensure spec covers:
   - Objective: What specifically to achieve?
   - Scope: What files/systems to touch or avoid?
   - Success: How do we know it's done?
   - Failure: What to do if stuck? (stop/report/retry)
   - Model hint: haiku-ok / needs-sonnet / needs-opus

   Draft spec from available context. Ask only for missing elements.

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
