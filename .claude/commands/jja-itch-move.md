You are moving an itch between Future, Shelved, or promoting to an effort.

Configuration:
- JJ files path: .claude/jji/
- Separate repo: no
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Identify the itch to move:
   - From context
   - Or ask user to specify
   - Use /jja-itch-locate if needed

2. Read current location (Future or Shelved)

3. Ask destination:
   - Future (jjf-future.md) - worthy of doing
   - Shelved (jjs-shelved.md) - setting aside
   - Promote to effort - create new jje-bYYMMDD-description.md

4. If promoting to effort:
   - Ask for effort description
   - Create new effort file in .claude/jji/current/
   - Use today's date for bYYMMDD
   - Include Context section
   - Include Steps section with initial step from itch
   - Remove itch from source file

5. If moving between Future/Shelved:
   - Remove from source file
   - Add to destination file
   - Preserve any context/notes

6. Show proposed change and ask for approval

7. Once approved, update the files

8. Commit:
   ```bash
   git add .claude/jji/jjf-future.md .claude/jji/jjs-shelved.md .claude/jji/current/
   git commit -m "JJA: itch-move - [brief description]"
   ```

9. Report what was done

Error handling: If paths wrong or files missing, announce issue and stop.
