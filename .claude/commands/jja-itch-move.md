You are helping move an itch between future, shelved, or promote to a new effort.

Configuration:
- JJ files path: `.claude/jji/`
- Separate repo: `no`
- Kit path: `Tools/jjk/job-jockey-kit.md`

Steps:
1. Identify which itch to move (user should specify or use /jja-itch-locate first)
2. Ask where to move it:
   - `.claude/jji/jjf-future.md` (worthy of doing)
   - `.claude/jji/jjs-shelved.md` (setting aside for now)
   - New effort file in `.claude/jji/current/jje-description.md` (promote to active effort)
3. Show the move and ask for approval
4. After approval, update the files:
   - Remove from source file
   - Add to destination file
   - If promoting to effort, create proper effort structure with Context and Steps sections
5. Commit with: `git add .claude/jji/*.md .claude/jji/current/*.md && git commit -m "JJA: itch-move - [brief description]"`
6. Report what was done

Error handling: If paths are misconfigured or files missing, announce issue and stop.
