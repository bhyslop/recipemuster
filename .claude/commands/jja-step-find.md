You are helping find the next incomplete step in the current Job Jockey effort.

Configuration:
- JJ files path: `.claude/jji/`
- Separate repo: `no`
- Kit path: `Tools/jjk/job-jockey-kit.md`

Steps:
1. Read the current effort file from `.claude/jji/current/`
2. Find the first unchecked step (- [ ]) in the Pending section
3. Display the step title and full description
4. If no pending steps found, report that all steps are complete

Error handling: If paths are misconfigured or files missing, announce issue and stop.
