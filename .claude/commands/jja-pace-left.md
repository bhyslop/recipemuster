You are listing all remaining paces in the current Job Jockey heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check for heat files in .claude/jji/current/
   - If 0 heats: announce "No active heat" and stop
   - If multiple heats: ask which one

2. Read the heat file

3. Find all incomplete paces (`- [ ]` items)

4. Display terse list with mode:
   ```
   Remaining paces (3):
   1. [manual] Audit BUK portability
   2. [manual] Create test harness
   3. [delegated] Document migration guide
   ```

5. If no incomplete paces: "All paces complete! Ready to retire this heat?"

Error handling: If .claude/jji/current/ doesn't exist or no heat found, announce issue and stop.
