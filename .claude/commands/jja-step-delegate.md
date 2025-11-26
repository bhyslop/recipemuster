You are executing a delegated step from the current Job Jockey effort.

Configuration:
- JJ files path: .claude/jji/
- Separate repo: no
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for current effort in .claude/jji/current/
   - If no effort: announce "No active effort" and stop
   - If multiple: ask which one

2. Identify the step to delegate (from context or ask)

3. Validate the step:
   - Is mode `delegated`?
     - If `manual`: refuse with "This step is manual - work on it conversationally"
     - If unset: refuse with "Run /jja-step-refine first to set mode"
   - Is spec healthy? Check for:
     - Objective defined
     - Scope bounded
     - Success criteria clear
     - Failure behavior specified
   - If unhealthy: refuse with "This step needs refinement - [specific gap]"

4. If valid, present the step spec clearly:
   ```
   Executing delegated step: **[title]**

   Objective: [objective]
   Scope: [scope]
   Success: [criteria]
   On failure: [behavior]
   ```

5. Execute the step based solely on the spec
   - Work from the spec, not from refinement conversation context
   - Stay within defined scope
   - Stop when success criteria met OR failure condition hit

6. Report outcome:
   - Success: what was accomplished, evidence of success criteria
   - Failure: what was attempted, why stopped, what's needed

7. Do NOT auto-complete the step. User decides via /jja-step-wrap

Error handling: If paths wrong or files missing, announce issue and stop.
