You are adding a new pace to the current Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for heat files in .claude/jji/current/
   - If 0 heats: announce "No active heat" and stop
   - If multiple heats: ask which one

2. Read the heat file to understand context and existing paces

3. Ask user what pace to add (or infer from conversation context)

4. Analyze the heat and propose:
   - Pace title (bold format)
   - Optional description
   - Suggested position (before/after which existing pace)
   - Default mode: manual

5. Explain reasoning for the placement

6. Example proposal:
   ```
   I propose adding pace '**Test BCU fixes**' after 'Audit BUK portability'
   because we'll need to validate each fix before moving to BDU.
   Mode: manual (default)

   Should I add it there?
   ```

7. Wait for user approval or amendment

8. Update the heat file with the new pace

9. Do NOT commit (preparatory work, accumulates until /jja-pace-wrap or /jja-sync)

10. Report what was added

Error handling: If paths wrong or files missing, announce issue and stop.
