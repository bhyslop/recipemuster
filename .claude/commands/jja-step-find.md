# /jja-step-find: Show next incomplete step

You are helping the user see the next incomplete step from the current Job Jockey effort.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Steps:
1. Check `.claude/jji/current/` for effort files
   - If none found: Report "No active efforts found" and stop
   - If multiple found: Ask which effort to check, or check the most recently modified
   - If one found: Proceed to step 2

2. Read the current effort file and find the "### Pending" section

3. Extract the first unchecked step (starts with `- [ ]`)
   - Show the step title in bold
   - Show the full description if present
   - If no pending steps: Report "All steps complete!"

4. Report the next step clearly

Error handling: If files missing or paths wrong, announce issue and stop.
