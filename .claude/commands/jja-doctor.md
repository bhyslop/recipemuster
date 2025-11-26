You are validating the Job Jockey installation.

Configuration:
- JJ files path: `.claude/jji/`
- Separate repo: `no`
- Kit path: `Tools/jjk/job-jockey-kit.md`

Validation checks:
1. Kit file exists at `Tools/jjk/job-jockey-kit.md`
2. JJ files directory exists at `.claude/jji/`
3. Required files present:
   - `.claude/jji/jjf-future.md`
   - `.claude/jji/jjs-shelved.md`
   - `.claude/jji/current/` directory
   - `.claude/jji/retired/` directory
4. Command files exist in `.claude/commands/`:
   - jja-next.md
   - jja-effort-retire.md
   - jja-step-find.md
   - jja-step-left.md
   - jja-step-add.md
   - jja-step-done.md
   - jja-itch-locate.md
   - jja-itch-move.md
   - jja-doctor.md
5. CLAUDE.md contains Job Jockey Configuration section
6. Since separate repo is `no`, verify we're in a git repository

Report:
- ✓ for each passing check
- ✗ for each failing check with details
- Overall status: "Job Jockey installation is healthy" or list of issues found

Do not attempt to fix issues - just report them.
