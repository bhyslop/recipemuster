# /jja-itch-move: Move or promote an itch

You are helping move an itch between future, shelved, or promote it to a new effort.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Steps:

1. Ask the user for the itch to move:
   - "Which itch would you like to move?" (or specify location)
   - Suggest using `/jja-itch-locate` first if they need to find it

2. Ask where to move it:
   - "Where should this go?"
   - Options:
     - **Future**: `jjf-future.md` (worthy of doing someday)
     - **Shelved**: `jjs-shelved.md` (setting aside for now)
     - **New Effort**: Create new `jje-description.md` file (promote to active work)

3. If promoting to new effort:
   - Ask for effort name/description
   - Create new effort file: `.claude/jji/current/jje-description.md`
   - Include the itch as first step(s)
   - Ask if user wants to start working on it now

4. Update the source and destination files:
   - Remove itch from source location (Future or Shelved)
   - Add itch to destination location

5. Commit the change:
   ```
   git add .claude/jji/current/ .claude/jji/jjf-future.md .claude/jji/jjs-shelved.md
   git commit -m "JJA: itch-move - Moved '[itch]' from [source] to [destination]"
   ```

6. Report what was moved

Error handling: If files are missing or paths are wrong, announce the issue and stop.
