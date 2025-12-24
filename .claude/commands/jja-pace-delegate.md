You are executing a delegated pace from the current Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for current heat in .claude/jji/current/
   - If no heat: announce "No active heat" and stop

2. Identify the pace to delegate (from context or ask)

3. Validate the pace:
   - Is mode `delegated`?
     - If `manual`: refuse with "This pace is manual - work on it conversationally"
     - If unset: refuse with "Run /jja-pace-refine first to set mode"
   - Is spec healthy? Check for:
     - Objective defined
     - Scope bounded
     - Success criteria clear
     - Failure behavior specified
   - If unhealthy: refuse with "This pace needs refinement - [specific gap]"

4. If valid, present the pace spec clearly:
   ```
   Executing delegated pace: **[title]**

   Objective: [objective]
   Scope: [scope]
   Success: [criteria]
   On failure: [behavior]
   ```

5. Execute the pace based solely on the spec
   - If target repo != `.`, work in target repo directory: .
   - Work from the spec, not from refinement conversation context
   - Stay within defined scope
   - Stop when success criteria met OR failure condition hit

6. Report outcome:
   - Success: what was accomplished, evidence of success criteria
   - Failure: what was attempted, why stopped, what's needed

7. Do NOT auto-complete the pace. User decides via /jja-pace-wrap
   Work in target repo is NOT auto-committed. User can review and use /jja-sync.

Error handling: If paths wrong or files missing, announce issue and stop.
