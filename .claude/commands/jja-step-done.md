You are helping mark a step complete in the current Job Jockey effort.

Configuration:
- JJ files path: `.claude/jji/`
- Separate repo: `no`
- Kit path: `Tools/jjk/job-jockey-kit.md`

Steps:
1. Identify which step to mark done (from conversation context or ask user)
2. Summarize the step completion based on chat context
3. Show proposed summary and ask for approval
4. After approval, update the effort file in `.claude/jji/current/`:
   - Move step from Pending section to Completed section
   - Change `- [ ]` to `- [x]`
   - Replace detailed description with brief factual summary
   - Format: `- [x] **Step title** - Brief summary of what was accomplished`
5. Commit with: `git add .claude/jji/current/*.md && git commit -m "JJA: step-done - [brief description]"`
6. Report what was done

Error handling: If paths are misconfigured or files missing, announce issue and stop.
