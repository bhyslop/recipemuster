---
argument-hint: [firemark]
description: Review and refine heat plan
---

Groom a heat: review the full plan and prepare for refinement work.

Use this command when you want to work on the heat's overall structure - adding paces, reordering, refining dockets - rather than executing the next pace.

Arguments: $ARGUMENTS (optional Firemark to select specific heat)

## Step 1: Get parade data

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_show $ARGUMENTS --remaining
```

If $ARGUMENTS is empty, show defaults to the newest racing heat.

This returns:
- Progress stats comment line: `# Progress: X complete, Y abandoned, Z remaining (A rough, B bridled)`
- Remaining paces list in execution order

## Step 2: Display heat overview

**Echo the vvx parade output directly** (no code blocks, no markdown tables, no box-drawing). The vvx output is already column-aligned plain text.

If heat status is "stabled", note: "⚠ This heat is stabled (paused). Use `/jjc-heat-furlough <firemark> --racing` to resume execution."

Summarize progress:
- Complete: X | Abandoned: Y | Remaining: Z (rough: A, bridled: B)

If user needs full dockets or paddock content, they can request it or use `/jjc-parade detail`.

## Step 3: Enter planning mode

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
- "Refine a pace docket with `/jjc-pace-reslate`"
- "Review paddock context" (offer to edit paddock file)

If all paces complete:
- "All paces complete. `/jjc-heat-retire` to archive."

Remain in planning discussion mode - do not automatically proceed to execution.

## Context preservation

Store for use by other commands:
- Current FIREMARK
- Current heat SILKS

## Available Operations

**Commits:** Always use `/jjc-pace-notch` — never vvx_commit directly.

- `/jjc-heat-mount` — Begin work on next pace
- `/jjc-heat-muster` — List all heats
- `/jjc-heat-groom` — Review and refine heat
- `/jjc-heat-nominate` — Create new heat
- `/jjc-pace-slate` — Add a new pace
- `/jjc-pace-notch` — JJ-aware git commit
- `/jjc-parade` — Heat or pace details
