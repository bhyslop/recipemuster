You are listing all Job Jockey itches.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Read .claude/jji/jjf-future.md for future itches

2. Read .claude/jji/jjs-shelved.md for shelved itches

3. Display all itches:
   ```
   Future itches (N):
   1. [itch title/description]
   2. [itch title/description]
   ...

   Shelved itches (N):
   1. [itch title/description] - [reason if noted]
   2. [itch title/description]
   ...
   ```

4. If either file is empty or missing, report appropriately:
   - "Future itches: none"
   - "Shelved itches: none"

Error handling: If files missing, report which ones and continue with available data.
