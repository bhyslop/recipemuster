---
argument-hint: [description-or-silks]
---

You are adding a new itch to the Job Jockey backlog.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md

Steps:

1. Understand the itch from conversation context
   - What future work is being captured?
   - What's the core idea or problem?

2. Read existing itches from .claude/jjm/jji_itch.md
   - Note existing silks to avoid duplicates

3. Generate unique silks (kebab-case identifier):
   - 3-5 words, short enough to say aloud
   - Must not duplicate existing itch silks
   - Should be memorable and descriptive

4. Append new itch section to .claude/jjm/jji_itch.md:
   ```markdown
   ## [silks]
   [Brief description of the itch - what and why]
   ```

5. Report what was added (no confirmation step - user can request adjustment)

6. Do NOT commit (accumulates until /jja-notch)

Error handling: If jji_itch.md missing, announce issue and stop.
