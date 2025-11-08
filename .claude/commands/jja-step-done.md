# jja-step-done: Mark Step Complete

You are helping mark a step complete in the current Jaunt Jockey effort.

**Configuration:**
- JJ files path: `.claude/jj/`
- Separate repo: no
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

**Steps:**

1. Look in `.claude/jj/current/` for the current effort file (single `jje-*.md` file)
   - If 0 files: Report "No active effort found"
   - If 1+ files: Open the file

2. Identify which step to mark done:
   - If obvious from recent chat context (user just finished something), propose that step
   - Otherwise ask: "Which step should I mark done?"

3. Summarize the step completion based on chat context:
   - Be concise: capture what was accomplished, not how
   - Example: "Audited 12 files, found 3 issues, documented in notes.md"
   - Should be 1-2 sentences max

4. Show proposed summary:
   ```
   Mark '[Step Title]' complete with summary:
   "[Summary text]"

   Approve?
   ```

5. Wait for approval (yes/no/amendment)

6. If approved, update the effort file:
   - Find the step under "### Pending" section (first `- [ ] **Title**`)
   - Move it to "### Completed" section (or create if missing)
   - Format: `- [x] **Title** - Summary text`
   - Delete the description lines (summary replaces it)

7. Commit with:
   ```bash
   git add .claude/jj/current/jje-*.md
   git commit -m "JJA: step-done - [Summary of what was accomplished]"
   ```

8. Report: "Updated '[Step Title]' → '[Summary]'"

**Error handling:** If files missing or paths wrong, announce the issue and stop.
