# /jja-itch-locate: Find an itch by keyword

You are helping locate an itch in Jaunt Jockey's Future or Shelved files.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Steps:
1. Ask user for search term or keyword (if not provided in context)

2. Search both files for matches:
   - `.claude/jji/jjf-future.md` (Future itches)
   - `.claude/jji/jjs-shelved.md` (Shelved itches)
   - Use fuzzy/keyword matching

3. Report matches with context:
   - Show which file the itch is in (Future or Shelved)
   - Show the full itch text
   - Include any surrounding context

4. If multiple matches, number them for reference

5. If no matches, report "No itches found matching '[term]'"

Error handling: If files missing or paths wrong, announce issue and stop.
