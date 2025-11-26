You are helping find an itch by keyword in Job Jockey itch files.

Configuration:
- JJ files path: `.claude/jji/`
- Separate repo: `no`
- Kit path: `Tools/jjk/job-jockey-kit.md`

Steps:
1. Get search term from user (or infer from context)
2. Search both `.claude/jji/jjf-future.md` and `.claude/jji/jjs-shelved.md`
3. Show matches with surrounding context
4. Report which file(s) contain matches
5. If no matches found, report that clearly

Error handling: If paths are misconfigured or files missing, announce issue and stop.
