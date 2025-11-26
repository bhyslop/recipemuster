You are validating the Job Jockey installation.

Configuration:
- JJ files path: .claude/jji/
- Separate repo: no
- Kit path: Tools/jjk/job-jockey-kit.md

Steps:

1. Check Kit file exists:
   - Look for Tools/jjk/job-jockey-kit.md
   - Report: ✓ or ✗

2. Check JJ files directory exists:
   - Look for .claude/jji/
   - Report: ✓ or ✗

3. Check expected files present:
   - .claude/jji/jjf-future.md - Report: ✓ or ✗
   - .claude/jji/jjs-shelved.md - Report: ✓ or ✗
   - .claude/jji/current/ directory - Report: ✓ or ✗
   - .claude/jji/retired/ directory - Report: ✓ or ✗

4. Check command files exist:
   - .claude/commands/jja-effort-next.md
   - .claude/commands/jja-effort-retire.md
   - .claude/commands/jja-step-find.md
   - .claude/commands/jja-step-left.md
   - .claude/commands/jja-step-add.md
   - .claude/commands/jja-step-refine.md
   - .claude/commands/jja-step-delegate.md
   - .claude/commands/jja-step-wrap.md
   - .claude/commands/jja-itch-locate.md
   - .claude/commands/jja-itch-move.md
   - .claude/commands/jja-doctor.md
   - Report count: ✓ all 11 or ✗ N/11

5. Verify paths in command files match configuration:
   - Spot check 2-3 command files
   - Verify they reference .claude/jji/ not «JJC_FILESYSTEM_RELATIVE_PATH»
   - Report: ✓ or ✗

6. List any active efforts found in .claude/jji/current/

7. Summary:
   - Overall health: ✓ Healthy / ⚠ Issues found / ✗ Not functional
   - If issues: list specific problems and remediation steps

Error handling: Report all findings even if some checks fail.
