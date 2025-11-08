# /jja-step-left: List all remaining steps

You are helping the user see all remaining steps in the current Jaunt Jockey effort.

Configuration:
- JJ files path: `.claude/jji/`
- Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`

Steps:
1. Check `.claude/jji/current/` for effort files
   - If none found: Report "No active efforts found" and stop
   - If multiple found: Ask which effort to check
   - If one found: Proceed to step 2

2. Read the current effort file and find the "### Pending" section

3. Count and list all pending steps (marked with `- [ ]`)
   - Format: Terse list with numbers, just the step titles
   - Include count: "Remaining steps (N):"

4. Report the list

Example output:
```
Remaining steps (3):
1. Audit BUK portability
2. Create test harness
3. Document migration guide
```

Error handling: If files missing or paths wrong, announce issue and stop.
