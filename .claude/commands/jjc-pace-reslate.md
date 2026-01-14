---
argument-hint: <pace> [new text]
description: Refine a pace's plan
---

Refine a pace's plan by adding a new Tack with updated text.

Arguments: $ARGUMENTS (format: `<pace> [optional new text]`)

The pace can be identified by:
- Coronet (e.g., `₢AAAAA` or `AAAAA`)
- Silks (e.g., `kit-asset-registry`) - resolved within current heat

## Step 1: Parse arguments

Extract from $ARGUMENTS:
- First word = pace identifier (coronet or silks)
- Remaining text = new tack text (optional)

**If $ARGUMENTS is empty:**
- Error: "Usage: /jjc-pace-reslate <pace> [new text]"
- Example: `/jjc-pace-reslate kit-asset-registry Refined approach: use macro for asset declaration`

**If new text is missing (only pace identifier provided):**
- Synthesize refined text from recent conversation context
- Draw on what was discussed about refining this pace
- Do NOT ask for confirmation - proceed directly

## Step 2: Resolve pace

**If identifier looks like a Coronet (5 base64 chars, optionally with ₢ prefix):**
- Use directly

**If identifier looks like silks (kebab-case):**
- Need heat context to resolve
- If FIREMARK available from session: use it
- Otherwise run `vvx jjx_muster --status current`:
  - If 1 heat: use it
  - If 0 heats: Error "No active heats"
  - If 2+ heats: ask user to select
- Run `vvx jjx_parade <FIREMARK>` and find pace by silks match
- Error if silks not found in heat

## Step 3: Apply reslate

Run:
```bash
echo "<NEW_TEXT>" | vvx jjx_tally <CORONET>
```

The new tack text is passed via stdin. State is inherited (stays rough, stays primed, etc.).

To change state, use:
- `/jjc-pace-prime` → primed (arm for autonomous execution)
- `/jjc-pace-wrap` → complete

## Step 4: Report and assess

On success, report:
- "Refined pace: **<SILKS>** (<CORONET>)"
- "State: <current state> (unchanged)"
- "New tack text: <first 100 chars>..."

Then **assess the pace's health**:

1. **Clarity**: Is the refined description clearer and more actionable?
2. **Scope**: Has scope crept or tightened appropriately?
3. **Readiness**: Is it now ready for priming?

## Step 5: Propose priming (if appropriate)

If the pace is now a good candidate for autonomous execution:

**Prime candidate?** Evaluate:
- Clear, bounded scope
- No blocking dependencies
- Can be accomplished by agents without human decision points

**If primeable, suggest direction:**
- Which agent type(s): Explore, Bash, Plan, general-purpose?
- Parallelism: single agent or multiple in parallel?
- Key files or patterns to target

**Remind user:**
> To arm for autonomous execution: `/jjc-pace-prime`

## Step 6: Next steps

**Next:** `/jjc-pace-reslate` (refine more) | `/jjc-pace-prime` (arm) | `/jjc-heat-parade` (view all)

## Error handling

On failure, report the error from vvx.

Common errors:
- "Pace not found" — invalid coronet or silks not in heat
- "Heat not found" — invalid Firemark context
- "text must not be empty" — synthesis failed
