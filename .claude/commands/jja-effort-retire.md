You are helping move a completed Job Jockey effort to the retired directory.

Configuration:
- JJ files path: `.claude/jji/`
- Separate repo: `no`
- Kit path: `Tools/jjk/job-jockey-kit.md`

Steps:
1. Verify current effort exists in `.claude/jji/current/`
2. Check that all steps are marked complete (or explicitly discarded)
3. Confirm with user which effort to retire (if multiple)
4. Rename file from `jje-description.md` → `jje-YYMMDD-description.md` (add today's date in YYMMDD format)
5. Move file to `.claude/jji/retired/`
6. Commit with: `git add .claude/jji/current/ .claude/jji/retired/ && git commit -m "JJA: effort-retire - [effort description]"`
7. Report what was done

Today's date for YYMMDD format: use the current date from environment.

Error handling: If paths are misconfigured or files missing, announce issue and stop.
