# /jja-step-done: Mark step complete

You are helping mark a step complete in the current Job Jockey effort.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Steps:
1. Check `.claude/jji/current/` for effort files
   - If none found: Report "No active efforts found" and stop
   - If multiple found: Ask which effort to update
   - If one found: Proceed to step 2

2. Ask which step to mark done (or infer from context)
   - Show list of pending steps for selection
   - Or accept step name from user

3. Summarize the step completion based on chat context
   - Brief, factual description of what was accomplished
   - One line is ideal (e.g., "Found 12 issues: 8 in BCU, 3 in BDU, 1 in BTU. Documented in portability-notes.md")

4. Show proposed summary and ask for approval:
   - Show the step title and summary
   - "Does this look right?" or "Make changes?"

5. Update `.claude/jji/current/[effort-file]`:
   - Move step from Pending to Completed section
   - Format: `- [x] **Step title** - Completed summary`
   - Keep the summary brief

6. Commit with:
   ```bash
   git add .claude/jji/current/jje-*.md
   git commit -m "JJA: step-done - [brief description of completed step]"
   ```

7. Report what was done

Error handling: If files missing or paths wrong, announce issue and stop.
