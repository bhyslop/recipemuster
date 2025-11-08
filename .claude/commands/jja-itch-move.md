# jja-itch-move: Move or Promote Itch

You are helping move an itch between future, shelved, or promote to a new effort.

**Configuration:**
- JJ files path: `.claude/jj/`
- Separate repo: no
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

**Steps:**

1. Ask user what to do (if not clear from context):
   - "Move to jjf-future.md (worthy), jjs-shelved.md (setting aside), or promote to new effort?"

2. **If moving to future or shelved:**
   - Ask which itch (by title or search)
   - Remove from current location
   - Add to target file with existing itches
   - Commit: `git commit -m "JJA: itch-move - [Itch title] → [destination]"`
   - Report: "Moved '[Itch]' to [destination]"

3. **If promoting to new effort:**
   - Ask which itch to promote
   - Create new `jje-YYMMDD-description.md` in `.claude/jj/current/` with:
     - Context section (extracted from itch description + any user input)
     - Initial step(s) derived from the itch description
   - Remove itch from source file
   - Commit: `git commit -m "JJA: itch-move - Promoted '[Itch title]' to new effort"`
   - Report: "Promoted '[Itch]' to new effort: `jje-YYMMDD-*.md`"

**Error handling:** If files missing, report issue and stop.
