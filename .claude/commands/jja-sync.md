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

Error handling: If paths wrong or repos inaccessible, announce issue and stop.
