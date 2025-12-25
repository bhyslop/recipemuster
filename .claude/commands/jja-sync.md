You are synchronizing JJ state and target repo work.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md

Steps:

1. Check if .claude/jjm/ is gitignored
   - If yes: warn "JJ state is gitignored - cannot sync" and stop

2. Commit and push JJ state (this repo):
   git add -A .claude/jjm/
   git commit -m "JJA: sync" --allow-empty
   git push
   Report: "JJ state: committed and pushed"

3. Target repo = `.`:
   - JJ state and work are same repo, already handled
   - Report: "Target repo: same as JJ state (direct mode)"

4. If any git operation fails, report the specific failure

5. Check for active heat with current pace:
   - Read .claude/jjm/current/ for active heat
   - Note the heat's silks (kebab-case description from filename)
   - If heat exists with a ## Current pace:
     - Read the pace description and any files/context it references
     - Analyze what the work entails
     - Propose a concrete approach (2-4 bullets)
     - Append APPROACH entry to steeplechase (.claude/jjm/current/jjc_*.md):
       - Create steeplechase file if it doesn't exist (jjc_bYYMMDD-[silks].md)
       - Entry format:
         ```markdown
         ---
         ### YYYY-MM-DD HH:MM - [pace-silks] - APPROACH
         **Mode**: manual | delegated
         **Proposed approach**:
         - [bullet 1]
         - [bullet 2]
         ---
         ```
     - Ask: "Ready to proceed with this approach?"
     - On approval: begin work directly (no /jja-heat-resume needed)
   - If heat exists but no current pace:
     - Announce "All paces complete - ready to retire heat?"
   - If no heat: just report sync complete

Error handling: If paths wrong or repos inaccessible, announce issue and stop.
