# /jja-itch-locate: Find an itch by keyword

You are helping find an itch (future idea or shelved item) by keyword or fuzzy match.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Steps:

1. Ask the user for a search term or keyword
   - "What would you like to search for?" (e.g., "documentation", "refactoring", "performance")

2. Search both itch files:
   - Read `.claude/jji/jjf-future.md` (future itches)
   - Read `.claude/jji/jjs-shelved.md` (shelved itches)

3. Find matches:
   - Use fuzzy keyword matching (partial matches acceptable)
   - Extract matching itch lines/entries
   - Note which file each came from (Future or Shelved)

4. Display results:
   - Show each match with source file
   - Format: "Found in [Future/Shelved]: [itch title/description]"
   - If no matches: "No itches found matching '[search term]'"

5. If matches found, offer next steps:
   - "Would you like to move or promote any of these?"
   - Suggest using `/jja-itch-move` if user wants to act on one

Error handling: If files are missing or paths are wrong, announce the issue and stop.
