---
argument-hint: [firemark]
description: Evaluate remaining paces and bridle the first primeable one
---

Evaluate all remaining rough paces in a heat, identify the first primeable one, and bridle it for autonomous execution.

Arguments: $ARGUMENTS (optional Firemark; uses current heat if omitted)

## Prerequisites

- Gallops JSON must exist
- Heat must have remaining rough paces

## Step 1: Identify target heat

**If $ARGUMENTS contains a Firemark (e.g., `AA` or `₣AA`):**
- Use that Firemark directly

**If $ARGUMENTS is empty:**
- Use FIREMARK from current context
- If no context, error: "No heat context. Run /jjc-heat-mount first."

## Step 2: Get remaining paces

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_parade <FIREMARK> --format overview --remaining
```

Parse output to get list of rough paces (skip primed — already ready).

**If no rough paces remain:**
- Report "No rough paces to quarter. All remaining paces are already primed or none remain."
- Suggest `/jjc-pace-slate` to add work, or `/jjc-heat-mount` to execute primed paces
- Stop

## Step 3: Evaluate all rough paces (parallel)

Evaluate all rough paces in parallel:

1. **Fetch tack text** for all rough paces via `jjx_parade --format detail --pace <coronet>`

2. **Apply primeability criteria** to each (all four must hold):
   - **Mechanical**: Clear transformation, not design work
   - **Pattern exists**: Following established pattern, not creating new one
   - **No forks**: Single obvious approach, not "we could do X or Y"
   - **Bounded**: Touches known files, not "find where this should go"

3. **Build summary table** with verdict and reason for each pace

Identify the first primeable pace in parade order as the bridle target.

## Step 4: Report findings

Display summary table:
```
Quartering heat <SILKS> (₣<FIREMARK>):

₢AAAAC fix-auth-bug        — NOT primeable: needs investigation first
₢AAAAD add-logging         — NOT primeable: multiple approaches (structured vs unstructured)
₢AAAAE rename-config-keys  — PRIMEABLE ✓
```

**If no primeable pace found:**
- Report "No primeable paces found. All remaining work requires human judgment."
- List the blockers for each pace
- Suggest `/jjc-pace-reslate` to refine specs, or `/jjc-heat-mount` to work collaboratively
- Stop

## Step 5: Bridle the primeable pace

Invoke `/jjc-pace-prime` for the identified pace.

This delegates to the prime command which will:
- Recommend execution strategy (agent tier, parallelization)
- Get user approval
- Write direction and transition to primed state

## Step 6: Offer next action

After bridling completes:
- "Ready to `/jjc-heat-mount` and execute?"
- Or: "Continue quartering remaining paces?"

## Available Operations

- `/jjc-heat-mount` — Begin work on next pace
- `/jjc-heat-quarter` — Evaluate and bridle next primeable pace
- `/jjc-pace-prime` — Manually prime a specific pace
- `/jjc-pace-reslate` — Refine pace specification
- `/jjc-parade-overview` — View all paces
