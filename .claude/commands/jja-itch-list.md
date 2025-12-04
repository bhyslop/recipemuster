You are listing all Job Jockey itches from both future and shelved files.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Read .claude/jji/jjf-future.md

2. Read .claude/jji/jjs-shelved.md

3. Display all itches:
   ```
   Future itches (3):
   1. Add dark mode support
   2. Refactor authentication module
   3. Performance optimization for large datasets

   Shelved itches (2):
   1. Legacy API migration (blocked on vendor)
   2. Mobile app prototype (deferred to Q2)
   ```

4. If either file is empty, note it:
   ```
   Future itches: (none)
   ```

Error handling: If files missing, announce which ones and stop.
