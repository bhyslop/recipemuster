You are helping show the current Job Jockey effort and its next step(s).

Configuration:
- JJ files path: `.claude/jji/`
- Separate repo: `no`
- Kit path: `Tools/jjk/job-jockey-kit.md`

Steps:
1. Check `.claude/jji/current/` for active efforts
2. Handle based on count:
   - **0 efforts**: Announce no active work, ask if user wants to start an effort or promote an itch
   - **1 effort**: Display effort name, brief summary, and next incomplete step(s) with description. If multiple next steps or unclear priority, ask for clarification ("Which step should we focus on next?")
   - **2+ efforts**: Ask user which effort to work on, then display that effort with next step(s)
3. Keep output concise and focused on what's next

Error handling: If paths are misconfigured or files missing, announce issue and stop.
