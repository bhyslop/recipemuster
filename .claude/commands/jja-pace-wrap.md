You are helping mark a pace complete in the current Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Ask which pace to mark done (or infer from context)

2. Summarize the pace completion based on chat context

3. Show proposed summary and ask for approval

4. Update the heat file in .claude/jji/current/
   - Move pace from Pending to Completed section
   - Change `- [ ]` to `- [x]`
   - Replace description with brief summary

5. Commit JJ state (this repo only, no push):
   ```bash
   git add .claude/jji/current/jjh-*.md
   git commit -m "JJA: pace-wrap - [brief description]"
   ```

6. Report what was done:
   ```
   Updated pace 'Audit BUK portability' →
   'Found 12 issues: 8 in BCU, 3 in BDU, 1 in BTU. Documented in portability-notes.md'
   Committed to JJ state.
   ```

Error handling: If files missing or paths wrong, announce issue and stop.
