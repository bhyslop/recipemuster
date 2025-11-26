You are marking a step complete in the current Job Jockey effort.

Configuration:
- JJ files path: .claude/jji/
- Separate repo: no
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for current effort in .claude/jji/current/
   - If no effort: announce "No active effort" and stop
   - If multiple: ask which one

2. Ask which step to mark done (or infer from conversation context)

3. Based on recent chat context, draft a concise summary:
   - One sentence, factual
   - Focus on outcome, not process
   - Examples: "Found 12 issues, documented in notes.md" or "Refactored 8 functions to use new pattern"

4. Show proposed summary and ask for approval or amendment

5. Once approved, update the effort file:
   - Mark step as [x] in Pending section
   - Move to Completed section
   - Replace detailed description with brief summary
   - Format: `- [x] **Title** - Summary`

6. Commit:
   ```bash
   git add .claude/jji/current/jje-*.md
   git commit -m "JJA: step-wrap - [brief summary]"
   ```

7. Report what was done

Error handling: If paths wrong or files missing, announce issue and stop.
