You are validating the Job Jockey installation.

Configuration:
- Target repo dir: .
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check kit file:
   - Verify Tools/jjk/job-jockey-kit.md exists and is readable
   - Report: ✓ Kit file exists / ✗ Kit file not found at [path]

2. Check JJ directory structure:
   - Verify .claude/jji/ exists
   - Verify .claude/jji/current/ exists
   - Verify .claude/jji/pending/ exists
   - Verify .claude/jji/retired/ exists
   - Report status of each

3. Check JJ content files:
   - Verify .claude/jji/jjf-future.md exists
   - Verify .claude/jji/jjs-shelved.md exists
   - Report: ✓ exists / ✗ missing for each

4. Check git tracking:
   - Run: git check-ignore .claude/jji/
   - If ignored: ⚠ WARNING: JJ state is gitignored - /jja-sync will not track changes
   - If not ignored: ✓ JJ state is tracked by git

5. Check command files:
   - Verify these files exist in .claude/commands/:
     - jja-heat-next.md
     - jja-heat-retire.md
     - jja-pace-find.md
     - jja-pace-left.md
     - jja-pace-add.md
     - jja-pace-refine.md
     - jja-pace-delegate.md
     - jja-pace-wrap.md
     - jja-sync.md
     - jja-itch-list.md
     - jja-itch-find.md
     - jja-itch-move.md
     - jja-doctor.md
   - Report: ✓ All 13 commands present / ✗ Missing: [list]

6. Check heats:
   - List any files in .claude/jji/current/
   - List any files in .claude/jji/pending/
   - Report counts and names

7. Summary:
   ```
   Job Jockey Health Check
   =======================
   Kit:        ✓ Found at Tools/jjk/job-jockey-kit.md
   Structure:  ✓ All directories present
   Files:      ✓ jjf-future.md, jjs-shelved.md present
   Git:        ✓ JJ state tracked
   Commands:   ✓ All 13 commands installed

   Active heats: 1
   - jjh-b251108-buk-portability.md

   Pending heats: 0

   Status: HEALTHY
   ```

   Or if issues:
   ```
   Status: NEEDS ATTENTION
   - Missing command: jja-pace-wrap.md
   - Directory missing: .claude/jji/pending/
   ```

Error handling: Report all issues found, don't stop at first error.
