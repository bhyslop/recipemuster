# /jja-step-add: Add a new step to current effort

You are helping add a new step to the current Job Jockey effort with intelligent positioning.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Steps:

1. Check if there is a current effort in `.claude/jji/current/`
   - If no effort file found: Announce "No active effort" and suggest using `/jja-next`
   - If multiple effort files: Ask which one to add a step to
   - If one effort file: Proceed to step 2

2. Ask the user for the new step:
   - Step title (required)
   - Step description (optional)
   - Suggested position (optional, Claude can suggest)

3. Analyze the effort context and existing steps:
   - Read the effort file
   - Understand the context and goals
   - Examine existing pending steps to understand sequence

4. Propose the new step:
   - Suggest a title (if user didn't provide one)
   - Suggest a position (after which existing step, or at end)
   - Provide brief description (if user didn't provide one)
   - Explain the reasoning for the placement
   - Example: "I propose adding '**Test fixes**' after 'Audit portability' because we'll need to validate each fix before moving to the next phase. Should I add it there?"

5. Wait for user approval or amendment:
   - "Yes" / "sounds good" → Proceed to step 6
   - "Change it to..." → Incorporate amendments, re-propose
   - "No" → Cancel and explain why

6. Update the effort file:
   - Add the new step to the Pending section
   - Format: `- [ ] **Step title**` with optional description
   - Place it in the agreed position

7. Commit the change:
   ```
   git add .claude/jji/current/jje-*.md
   git commit -m "JJA: step-add - Added step '[step title]' to [effort-name]"
   ```

8. Report what was added

Error handling: If files are missing or paths are wrong, announce the issue and stop.
