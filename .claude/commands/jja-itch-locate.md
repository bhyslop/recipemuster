# jja-itch-locate: Find Itch

You are helping find an itch by keyword or fuzzy match.

**Configuration:**
- JJ files path: `.claude/jj/`
- Separate repo: no
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

**Steps:**

1. Ask user what to search for (if not already provided): "Search term for itch?"

2. Search both `.claude/jj/jjf-future.md` and `.claude/jj/jjs-shelved.md`:
   - Look for keyword matches (title or description)
   - Perform fuzzy matching if exact match not found
   - Report all matches with context (location and brief description)

3. Display results:
   ```
   Found 2 itches:

   1. [From jjf-future.md]
      Itch: [Title]
      [Description or context]

   2. [From jjs-shelved.md]
      Itch: [Title]
      [Description or context]
   ```

4. If nothing found, report: "No itches found matching '[search term]'"

**Error handling:** If files missing, report "Itch files not found at .claude/jj/". Stop.
