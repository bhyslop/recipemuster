You are helping show all remaining steps in the current Job Jockey effort.

Configuration:
- JJ files path: `.claude/jji/`
- Separate repo: `no`
- Kit path: `Tools/jjk/job-jockey-kit.md`

Steps:
1. Read the current effort file from `.claude/jji/current/`
2. List all unchecked steps (- [ ]) from the Pending section
3. Show count and brief titles only (no descriptions)
4. Format as: "Remaining steps (N):" followed by numbered list

Example output:
```
Remaining steps (3):
1. Audit BUK portability
2. Create test harness
3. Document migration guide
```

Error handling: If paths are misconfigured or files missing, announce issue and stop.
