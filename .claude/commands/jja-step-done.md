# /jja-step-done: Mark step complete with summary

You are helping mark a step as complete in the current Job Jockey effort with automatic summarization.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Steps:

1. Check if there is a current effort in `.claude/jji/current/`
   - If no effort file found: Announce "No active effort"
   - If multiple effort files: Ask which one
   - If one effort file: Proceed to step 2

2. Ask which step to mark complete (or infer from context):
   - "Which step did you just complete?" (show list of pending steps)
   - Or infer from chat history if there's a clear recent step

3. Summarize the step completion based on chat context:
   - Review what was accomplished in the current chat session
   - Write a brief, factual 1-line summary
   - Include specific outcomes, artifacts, file references if relevant
   - Example: "Found 12 issues: 8 in BCU, 3 in BDU, 1 in BTU. Documented in portability-notes.md"

4. Propose the update:
   - Show the step title
   - Show the proposed summary
   - Ask: "Does this look right?" or "Should I change anything?"

5. Update based on user feedback:
   - If yes: Proceed to step 6
   - If amendments: Revise the summary, re-propose

6. Update the effort file:
   - Find the step in the Pending section
   - Move it to the Completed section
   - Replace the description with the brief summary
   - Format: `- [x] **Step title** - Summary text`

7. Commit the change:
   ```
   git add .claude/jji/current/jje-*.md
   git commit -m "JJA: step-done - [brief description of what was completed]"
   ```

8. Report what was done

Error handling: If files are missing or paths are wrong, announce the issue and stop.
