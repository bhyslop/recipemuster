You are showing the next incomplete pace from the current Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for heat files in .claude/jji/current/
   - If 0 heats: announce "No active heat" and stop
   - If multiple heats: ask which one

2. Read the heat file

3. Find the first incomplete pace (`- [ ]` item)

4. Display the pace:
   - Title (bold text after `- [ ]`)
   - Mode (manual/delegated) if specified
   - Description (indented lines following the title)

5. Example output:
   ```
   Next pace: **Audit BUK portability** [manual]
     Review all four BUK utilities for hardcoded paths
     Document findings in portability-notes.md
   ```

Error handling: If .claude/jji/current/ doesn't exist or no heat found, announce issue and stop.
