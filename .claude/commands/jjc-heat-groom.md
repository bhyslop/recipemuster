---
argument-hint: [firemark]
description: Review and refine heat plan
---

Groom a heat: review the full plan and prepare for refinement work.

Use this command when you want to work on the heat's overall structure - adding paces, reordering, refining specifications - rather than executing the next pace.

Arguments: $ARGUMENTS (optional Firemark or silks to select specific heat)

## Prerequisites

## Step 1: Identify target heat

**If $ARGUMENTS contains a Firemark (e.g., `AA` or `₣AA`):**
- Use that Firemark directly
- Skip to Step 2

**If $ARGUMENTS is empty or contains silks:**
- Run: `./tt/vvw-r.RunVVX.sh jjx_muster`
- Parse TSV output: `FIREMARK<TAB>SILKS<TAB>STATUS<TAB>PACE_COUNT`

**If 0 heats:** Report "No heats found. Create one with `./tt/vvw-r.RunVVX.sh jjx_nominate`." and stop.

**If 1 heat:** Use that heat's Firemark.

**If 2+ heats:**
- If $ARGUMENTS matches a silks value, use that heat
- Otherwise list heats and ask user to select

## Step 2: Get parade data

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_parade <FIREMARK> --remaining
```

This returns:
- Progress stats comment line: `# Progress: X complete, Y abandoned, Z remaining (A rough, B bridled)`
- Remaining paces list in execution order

## Step 3: Display heat overview

**Always display the remaining paces list.** This shows actionable paces in execution order — the essential context for deciding what to work on.

If heat status is "stabled", note: "⚠ This heat is stabled (paused). Use `/jjc-heat-furlough <firemark> --racing` to resume execution."

Summarize progress:
- Complete: X | Abandoned: Y | Remaining: Z (rough: A, bridled: B)

If user needs full specs or paddock content, they can request it or use `/jjc-parade full`.

## Step 4: Enter planning mode

Identify the **next pace** — first incomplete pace by order (skip complete/abandoned). This is what `/jjc-heat-mount` will execute.

**Order determines execution priority; state determines execution mode:**
- rough → interactive (human collaboration)
- bridled → autonomous (agent execution)

A bridled pace later in the queue is NOT higher priority than a rough pace earlier in the queue.

Prompt the user:

"Heat **{silks}** has {N} paces. **Next up:** {next_pace_silks} (₢{coronet}) [{state}]"

"What would you like to work on?"

Suggest actions anchored to the next pace:
- If next pace is rough: "Execute interactively with `/jjc-heat-mount`" or "Bridle for autonomous execution with `/jjc-pace-bridle {coronet}`"
- If next pace is bridled: "Execute autonomously with `/jjc-heat-mount`"

Then offer structural operations:
- "Add new paces with `/jjc-pace-slate`"
- "Reorder paces with `/jjc-heat-rail`"
- "Refine a pace specification with `/jjc-pace-reslate`"
- "Review paddock context" (offer to edit paddock file)

If all paces complete:
- "All paces complete. `/jjc-heat-retire` to archive."

Remain in planning discussion mode - do not automatically proceed to execution.

## Context preservation

Store for use by other commands:
- Current FIREMARK
- Current heat SILKS

## Available Operations

- `/jjc-heat-mount` — Begin work on next pace
- `/jjc-heat-muster` — List all heats
- `/jjc-heat-groom` — Review and refine heat
- `/jjc-heat-nominate` — Create new heat
- `/jjc-pace-slate` — Add a new pace
- `/jjc-pace-notch` — JJ-aware git commit
- `/jjc-parade` — Heat or pace details
