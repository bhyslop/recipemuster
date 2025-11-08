# jja-step-find: Find Next Step

You are helping the user find the next incomplete step in the current Jaunt Jockey effort.

**Configuration:**
- JJ files path: `.claude/jj/`
- Separate repo: no
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

**Steps:**

1. Look in `.claude/jj/current/` for the current effort file (single `jje-*.md` file)
   - If 0 files: Report "No active effort found. Use /jja-step-add to start one, or promote an itch from /jja-itch-move"
   - If 1+ files: Open the file

2. Find the "### Pending" section

3. Display the first unchecked step (first `- [ ]` item):
   - Show its title in bold
   - Show its description (if any)
   - Stop here

4. If no pending steps found, report: "All steps complete! Archive this effort with: `git mv .claude/jj/current/jje-*.md .claude/jj/retired/`"

**Error handling:** If files missing or paths wrong, announce the issue and stop.
