You are moving a Job Jockey itch between locations or promoting it to a heat.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Identify the itch to move (from context or ask)
   - May need to use /jja-itch-find first

2. Ask destination:
   - `future` → move to jjf-future.md
   - `shelved` → move to jjs-shelved.md
   - `heat` → promote to new heat file

3. If moving between future/shelved:
   - Remove from source file
   - Add to destination file
   - Do NOT commit (accumulates until /jja-sync)

4. If promoting to heat:
   - Ask for heat description (kebab-case)
   - Create new file: .claude/jji/current/jjh-bYYMMDD-description.md
   - Use today's date for bYYMMDD
   - Initialize with:
     - Context section (from itch description)
     - Pending section with initial pace from itch
   - Remove itch from source file
   - Do NOT commit (accumulates until /jja-sync)

5. Report what was done:
   ```
   Moved 'Refactor authentication module' from future → shelved
   ```
   Or:
   ```
   Promoted 'Refactor authentication module' to new heat
   Created: .claude/jji/current/jjh-b251127-auth-refactor.md
   ```

Error handling: If files missing or itch not found, announce issue and stop.
