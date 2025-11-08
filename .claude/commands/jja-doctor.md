# jja-doctor: Validate Jaunt Jockey Setup

You are validating that Jaunt Jockey is correctly installed and all files are present.

**Configuration:**
- JJ files path: `.claude/jj/`
- Separate repo: no
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

**Validation checklist:**

1. Kit file exists: `Tools/jjk/Jaunt-Jockey-Kit.md`
   - Report: ✓ or ✗ with path

2. JJ files directory exists: `.claude/jj/`
   - Report: ✓ or ✗

3. Required files present:
   - `.claude/jj/jjf-future.md` (✓ or ✗)
   - `.claude/jj/jjs-shelved.md` (✓ or ✗)
   - `.claude/jj/current/` directory (✓ or ✗)
   - `.claude/jj/retired/` directory (✓ or ✗)

4. Commands exist:
   - `.claude/commands/jja-*.md` (list which ones are present)

5. Current efforts:
   - List any `jje-*.md` files in `.claude/jj/current/`

6. Summary:
   - "Jaunt Jockey is properly installed" or "Issues found:" with details

**Error handling:** Report any missing files or paths. Do not attempt to auto-fix.
