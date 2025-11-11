# /jja-step-find: Show next incomplete step

You are helping show the next incomplete step from the current Job Jockey effort.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Steps:

1. Check if there is a current effort in `.claude/jji/current/`
   - If no effort file found: Announce "No active effort" and suggest using `/jja-next`
   - If multiple effort files: Ask which one to examine, or suggest using `/jja-next`
   - If one effort file: Proceed to step 2

2. Read the effort file and find the first incomplete step
   - Look for the first checklist item with `- [ ]` (not checked)
   - Extract the step title (in bold, e.g., `**Step name**`)
   - Extract the step description (lines following the checklist item)

3. Display to the user:
   - Step title
   - Step description (if any)
   - Suggest next action ("Ready to work on this?")

4. Report what was found

Error handling: If no pending steps remain, report "All steps complete!" and ask if the effort should be retired using `/jja-effort-retire`.
