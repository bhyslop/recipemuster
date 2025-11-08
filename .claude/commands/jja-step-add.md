# jja-step-add: Add New Step

You are helping add a new step to the current Jaunt Jockey effort.

**Configuration:**
- JJ files path: `.claude/jj/`
- Separate repo: no
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

**Steps:**

1. Look in `.claude/jj/current/` for the current effort file (single `jje-*.md` file)
   - If 0 files: Report "No active effort found"
   - If 1+ files: Open the file

2. Read the effort Context section and existing Pending steps to understand direction

3. Ask user (if not obvious from context) what the new step should be

4. Analyze where this step logically fits:
   - Is it a blocker for other steps? Should go early.
   - Is it dependent on other steps? Should go after them.
   - Is it orthogonal? Can go anywhere reasonable.

5. Propose the new step:
   ```
   I propose adding:
   - [ ] **Step title** [optional description]

   After/before: [step X] because [reasoning]

   Should I add it there?
   ```

6. Wait for user approval (yes/no/amendment)

7. If approved, update the effort file:
   - Find the "### Pending" section
   - Insert step at the proposed position (maintaining checklist format)
   - Preserve step format: `- [ ] **Title** Optional description on new lines`

8. Commit with:
   ```bash
   git add .claude/jj/current/jje-*.md
   git commit -m "JJA: step-add - Added step: [title]"
   ```

9. Report: "Added step '[title]' to current effort"

**Error handling:** If files missing or paths wrong, announce the issue and stop.
