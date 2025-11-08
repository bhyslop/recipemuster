# jja-step-left: List Remaining Steps

You are helping the user see all remaining incomplete steps in the current Jaunt Jockey effort.

**Configuration:**
- JJ files path: `.claude/jj/`
- Separate repo: no
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

**Steps:**

1. Look in `.claude/jj/current/` for the current effort file (single `jje-*.md` file)
   - If 0 files: Report "No active effort found"
   - If 1+ files: Open the file

2. Find the "### Pending" section

3. Extract all unchecked steps (`- [ ]` items) and display in terse format:
   ```
   Remaining steps (N):
   1. Step title one
   2. Step title two
   3. Step title three
   ```

4. If no pending steps, report: "No remaining steps. Ready to archive."

**Error handling:** If files missing or paths wrong, announce the issue and stop.
