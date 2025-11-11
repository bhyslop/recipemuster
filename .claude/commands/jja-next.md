# /jja-next: Show current effort and next step(s)

You are helping the user see their current Job Jockey effort and what needs to be done next.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Steps:

1. Check `.claude/jji/current/` for active effort files
   - If no files found: Announce "No active efforts" and ask if user wants to start one or promote an itch from jjf-future.md
   - If one file found: Proceed to step 2
   - If multiple files found: Show list and ask user which effort to work on, then proceed to step 2 with selected effort

2. For the active effort file:
   - Read the file to extract: effort name/summary, and list of pending steps
   - Identify the first unchecked step (first `- [ ]` item)

3. Display to the user:
   - Effort name/summary (first line or context)
   - Next step title and description
   - If there are multiple pending steps or the priority is unclear: Ask clarifying question ("Which step would you like to focus on next?")

4. Example output format:
   ```
   Current effort: **Effort Name**
   Next step: Step title
   Step description here...

   Ready to start?
   ```

5. Report what's displayed

Error handling: If files are missing or paths are wrong, announce the issue and stop - do not guess or auto-fix.
