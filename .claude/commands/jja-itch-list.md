You are listing all Job Jockey itches and scars.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/README.md

Steps:

1. Read .claude/jjm/jji_itch.md for itches (future work)

2. Read .claude/jjm/jjs_scar.md for scars (closed with lessons)

3. Display all entries:
   ```
   Itches (N):
   1. [section header] - [brief description]
   2. [section header] - [brief description]
   ...

   Scars (N):
   1. [section header] - [closed reason]
   2. [section header] - [closed reason]
   ...
   ```

4. If either file is empty or missing, report appropriately:
   - "Itches: none"
   - "Scars: none"

Error handling: If files missing, report which ones and continue with available data.
