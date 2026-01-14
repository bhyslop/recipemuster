---
argument-hint: <silks> <text>
description: Add a new pace to a heat
---

Add a new pace (discrete action) to a Job Jockey heat.

Arguments: $ARGUMENTS (format: `<silks> <pace description text>`)

## Prerequisites

- Must have active heat context (run `/jjc-heat-saddle` first, or provide Firemark)
- Gallops JSON must exist at `.claude/jjm/jjg_gallops.json`

## Step 1: Parse arguments

Extract from $ARGUMENTS:
- First word = silks (kebab-case pace name)
- Remaining text = pace description (the tack text)

**If $ARGUMENTS is empty or missing description:**
- Error: "Usage: /jjc-pace-slate <silks> <description>"
- Example: `/jjc-pace-slate add-validation Add input validation to the form submission handler`

**Validate silks format:**
- Must be kebab-case: `[a-z0-9]+(-[a-z0-9]+)*`
- Error if invalid: "Silks must be kebab-case (e.g., 'add-tests', 'fix-bug')"

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

## Step 4: Report result

On success:
- "Created pace: **<SILKS>** (<CORONET>)"
- "Heat: <HEAT_SILKS> (₣XX)"
- "State: rough (needs approach before execution)"
- "Text: <first 100 chars of description>..."

On failure, report the error from vvx.

## Step 5: Offer next steps

Ask: "Would you like to:"
1. Add another pace with `/jjc-pace-slate`
2. Saddle up and start working with `/jjc-heat-saddle`
3. View all paces with `/jjc-heat-parade`
