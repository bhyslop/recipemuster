---
argument-hint: [coronet]
description: Mark a pace complete
---

Mark a pace as complete and record the wrap in steeplechase history.

Arguments: $ARGUMENTS (optional Coronet; uses current pace if omitted)

## Prerequisites

- Gallops JSON must exist
- Pace should be in "rough" or "primed" state
- Work on the pace should be done

## Step 1: Identify target pace

**If $ARGUMENTS contains a Coronet (e.g., `ABCDE` or `₢ABCDE`):**
- Extract Firemark from first 2 characters
- Use that Coronet directly

**If $ARGUMENTS is empty:**
- Use PACE_CORONET from current saddle context
- If no context, error: "No pace context. Run /jjc-heat-saddle first."

## Step 2: Get current state

Run:
```bash
vvx jjx_saddle <FIREMARK>
```

Verify pace state:
- If "complete": "Pace already complete."
- If "abandoned": "Pace was abandoned, cannot wrap."

## Step 3: Summarize outcome

Review the work completed:
- What was accomplished
- Key changes made
- Any notable decisions or deviations from plan

Construct a brief outcome summary (1-3 sentences).

## Step 4: Transition to complete

Run:
```bash
echo "<outcome summary>" | vvx jjx_tally <CORONET> --state complete
```

## Step 5: Create wrap marker

Run:
```bash
vvx jjx_chalk <FIREMARK> --pace <PACE_SILKS> --marker WRAP --description "<outcome summary>"
```

## Step 6: Advance to next pace

Run:
```bash
vvx jjx_saddle <FIREMARK>
```

**If another actionable pace exists:**
- Display the next pace's silks and tack_text
- If rough: Propose approach (as in /jjc-heat-saddle)
- If primed: Ask if ready to execute

**If no more actionable paces:**
- Report "All paces complete for heat <SILKS>"
- Suggest `/jjc-pace-slate` to add more work, or `/jjc-heat-retire` if done

## Step 7: Prompt for commit

If there are uncommitted changes:
- Ask: "Commit these changes with /jjc-pace-notch?"
- If yes, invoke `/jjc-pace-notch`
