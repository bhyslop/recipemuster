You are finding a Job Jockey itch by keyword.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Ask for search term (or use term from context)

2. Read .claude/jji/jjf-future.md

3. Read .claude/jji/jjs-shelved.md

4. Search both files for matches (case-insensitive)

5. Report matches with context:
   ```
   Found 2 matches for "auth":

   In jjf-future.md:
   - Refactor authentication module
     "Consolidate auth logic into single service"

   In jjs-shelved.md:
   - OAuth provider migration
     "Switch from provider X to Y (blocked on contract)"
   ```

6. If no matches: "No itches found matching '[term]'"

Error handling: If files missing, announce which ones and stop.
