# /jja-itch-move: Move or promote an itch

You are helping move an itch between Future, Shelved, or promote to a new effort.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Steps:
1. Locate the itch (use /jja-itch-locate first if needed)
   - Get the exact text of the itch from user or from earlier search

2. Ask where to move it:
   - To Future (worthy of doing)
   - To Shelved (set aside for now)
   - Promote to new effort (create jje-YYMMDD-*.md file)

3. If promoting to effort:
   - Ask for effort title/description
   - Create new `jje-YYMMDD-[description].md` in `.claude/jji/current/`
   - Use today's date (YYMMDD format)
   - Include Context and Pending sections with initial steps
   - The promoted itch becomes part of the effort context or a first step

4. Update the source file:
   - Remove itch from `.claude/jji/jjf-future.md` or `.claude/jji/jjs-shelved.md`
   - OR move it to the new location (if moving between Future/Shelved)

5. Commit with:
   ```bash
   git add .claude/jji/jjf-future.md .claude/jji/jjs-shelved.md .claude/jji/current/jje-*.md
   git commit -m "JJA: itch-move - [brief description of itch movement]"
   ```

6. Report what was done

Error handling: If files missing or paths wrong, announce issue and stop.
