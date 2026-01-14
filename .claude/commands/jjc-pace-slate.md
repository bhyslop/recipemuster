---
argument-hint: <silks> [description]
description: Add a new pace to a heat
---

Add a new pace (discrete action) to a Job Jockey heat.

Arguments: $ARGUMENTS (format: `<silks> [optional description]`)

## Step 1: Parse arguments

Extract from $ARGUMENTS:
- First word = silks (kebab-case pace name)
- Remaining text = pace description (optional)

**If $ARGUMENTS is empty:**
- Error: "Usage: /jjc-pace-slate <silks> [description]"
- Example: `/jjc-pace-slate add-validation Add input validation to the form handler`

**Validate silks format:**
- Must be kebab-case: `[a-z0-9]+(-[a-z0-9]+)*`
- Error if invalid: "Silks must be kebab-case (e.g., 'add-tests', 'fix-bug')"

**If description is missing (only silks provided):**
- Synthesize a description from recent conversation context
- Draw on what was discussed about this pace concept
- Do NOT ask for confirmation - proceed directly

## Step 2: Get heat context

**If FIREMARK is available from current session context:**
- Use that Firemark

**Otherwise:**
- Run: `vvx jjx_muster --status current`
- Parse TSV output

**If 0 heats:** Error: "No active heats. Create one with `/jjc-heat-nominate` first."

**If 1 heat:** Use that heat's Firemark.

**If 2+ heats:** List heats and ask user to select.

## Step 3: Create pace

Run:
```bash
echo "<PACE_TEXT>" | vvx jjx_slate <FIREMARK> --silks "<SILKS>"
```

The pace description text is passed via stdin.

Capture the new Coronet from stdout.

## Step 4: Report and assess

On success, report:
- "Created pace: **<SILKS>** (<CORONET>)"
- "Heat: <HEAT_SILKS> (₣XX)"
- "State: rough"

Then **assess the pace's health**:

1. **Clarity**: Is the description clear and actionable?
2. **Scope**: Is it well-bounded or too broad?
3. **Dependencies**: Does it depend on other paces completing first?

## Step 5: Assess primeability

Apply **Primeability Assessment** criteria from CLAUDE.md.

If primeable, suggest direction (agent type, parallelism, key files).
If not primeable, state why: "Needs human judgment — [reason]"

**Next:** `/jjc-pace-slate` (add another) | `/jjc-pace-reslate` (refine) | `/jjc-pace-prime` (arm)

## Error handling

On failure, report the error from vvx.

Common errors:
- "Heat not found" — invalid Firemark
- "text must not be empty" — description synthesis failed
