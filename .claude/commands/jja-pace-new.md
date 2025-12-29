You are adding a new pace to the current Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md

Steps:

1. Check for heat files in .claude/jjm/current/
   - If 0 heats: announce "No active heat" and stop
   - If 2+ heats: ask which one

2. Read the heat file to understand context and existing paces

3. Analyze the heat context and existing paces

4. Propose a new pace:
   - Title (bold format)
   - Optional description
   - Position in the list (explain reasoning)

5. Present proposal:
   ```
   I propose adding pace '**[title]**' after '[existing pace]'
   because [reasoning].
   Should I add it there?
   ```

6. Wait for user approval or amendment

7. If approved, update the heat file:
   - Add pace to ## Remaining section at proposed position

8. Do NOT commit (preparatory work, accumulates until /jja-pace-wrap or /jja-sync)

9. Report what was added

Error handling: If paths wrong or files missing, announce issue and stop.
