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

**If $ARGUMENTS contains a Coronet (e.g., `AAAAC` or `₢AAAAC`):**
- Extract Firemark from first 2 characters
- Use that Coronet directly

**If $ARGUMENTS is empty:**
- Use PACE_CORONET from current context
- If no context, error: "No pace context. Run /jjc-heat-mount first."

## Step 2: Get current state

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_saddle <FIREMARK>
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
echo "<outcome summary>" | ./tt/vvw-r.RunVVX.sh jjx_tally <CORONET> --state complete
```

## Step 5: Create wrap marker

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_chalk <FIREMARK> --pace <PACE_SILKS> --marker WRAP --description "<outcome summary>"
```

## Step 6: Advance to next pace

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_saddle <FIREMARK>
```

**If another actionable pace exists:**
- Display the next pace's silks and tack_text
- If rough: Propose approach (as in /jjc-heat-mount)
- If primed: Ask if ready to execute

**If no more actionable paces:**
- Report "All paces complete for heat <SILKS>"
- Suggest `/jjc-pace-slate` to add more work, or `/jjc-heat-retire` if done

## Step 7: Auto-commit changes

Run guarded commit:
```bash
./tt/vvw-r.RunVVX.sh vvx_commit --message "Wrap: <SILKS>"
```

On failure (e.g., lock held), report error but don't fail the operation — gallops changes are already saved.

## Available Operations

- `/jjc-pace-slate` — Add a new pace
- `/jjc-pace-reslate` — Refine pace specification
- `/jjc-pace-wrap` — Mark pace complete
- `/jjc-pace-prime` — Arm pace for autonomous execution
- `/jjc-heat-mount` — Begin work on next pace
- `/jjc-heat-rail` — Reorder paces
- `/jjc-heat-chalk` — Add steeplechase marker
- `/jjc-parade-overview` — Heat summary
