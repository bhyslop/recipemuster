You are finding an itch by keyword in the Job Jockey system.

Configuration:
- JJ files path: .claude/jji/
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Ask for search term (or use from context)

2. Search both files:
   - .claude/jji/jjf-future.md
   - .claude/jji/jjs-shelved.md

3. Use fuzzy/keyword matching to find relevant itches:
   - Match in titles
   - Match in descriptions
   - Show context around matches

4. Display results:
   ```
   Found in Future:
   - [itch title and context]

   Found in Shelved:
   - [itch title and context]
   ```

5. If no matches: "No itches found matching '[term]'"

6. Suggest next actions:
   - Use /jja-itch-move to relocate an itch
   - Refine search if too many/few results

Error handling: If paths wrong or files missing, announce issue and stop.
