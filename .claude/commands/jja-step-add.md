# /jja-step-add: Add a new step to current effort

You are helping add a new step to the current Jaunt Jockey effort.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Steps:
1. Check `.claude/jji/current/` for effort files
   - If none found: Report "No active efforts found" and stop
   - If multiple found: Ask which effort to add to
   - If one found: Proceed to step 2

2. Analyze the effort context and existing steps

3. Propose a new step with:
   - Step title (bold format)
   - Optional description if relevant
   - Suggested position in the list
   - Brief explanation of why you're placing it there

4. Wait for user approval or amendment
   - "Yes" → proceed to step 5
   - Request changes → adjust proposal and re-ask
   - "No" → cancel and report

5. Update `.claude/jji/current/[effort-file]`:
   - Add step to Pending section at proposed position
   - Format: `- [ ] **Step title**` with optional description

6. Commit with:
   ```bash
   git add .claude/jji/current/jje-*.md
   git commit -m "JJA: step-add - [brief description of new step]"
   ```

7. Report what was added

Error handling: If files missing or paths wrong, announce issue and stop.
