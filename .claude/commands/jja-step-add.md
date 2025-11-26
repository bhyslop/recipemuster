You are helping add a new step to the current Job Jockey effort.

Configuration:
- JJ files path: `.claude/jji/`
- Separate repo: `no`
- Kit path: `Tools/jjk/job-jockey-kit.md`

Steps:
1. Read the current effort file from `.claude/jji/current/`
2. Understand the context and existing steps
3. Based on user request or conversation context, propose:
   - Step title (in bold: **Title**)
   - Optional description (if needed for clarity)
   - Position in the step list (explain reasoning)
4. Show proposal and ask for approval
5. After approval, update the effort file:
   - Add step to Pending section at proposed position
   - Maintain markdown checklist format: `- [ ] **Title**`
6. Commit with: `git add .claude/jji/current/*.md && git commit -m "JJA: step-add - [brief step description]"`
7. Report what was added

Error handling: If paths are misconfigured or files missing, announce issue and stop.
