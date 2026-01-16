---
argument-hint: [firemark]
description: Mount up and execute next pace
---

Mount a heat: identify the next actionable pace and begin execution.

Use this command when you're ready to work - it finds the next rough or primed pace and drives toward completing it.

Arguments: $ARGUMENTS (optional Firemark or silks to select specific heat)

## Prerequisites

## Step 1: Identify target heat

**If $ARGUMENTS contains a Firemark (e.g., `AA` or `₣AA`):**
- Use that Firemark directly
- Skip to Step 2

**If $ARGUMENTS is empty or contains silks:**
- Run: `./tt/vvw-r.RunVVX.sh jjx_muster --status current`
- Parse TSV output: `FIREMARK<TAB>SILKS<TAB>STATUS<TAB>PACE_COUNT`

**If 0 heats:** Report "No active heats. Create one with `./tt/vvw-r.RunVVX.sh jjx_nominate`." and stop.

**If 1 heat:** Use that heat's Firemark.

**If 2+ heats:**
- If $ARGUMENTS matches a silks value, use that heat
- Otherwise list heats and ask user to select

## Step 2: Get current pace context

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_saddle <FIREMARK>
```

Parse JSON output:
```json
{
  "heat_silks": "...",
  "paddock_file": ".claude/jjm/jjp_XX.md",
  "paddock_content": "...",
  "pace_coronet": "₢AAAAC",
  "pace_silks": "...",
  "pace_state": "rough|primed",
  "tack_text": "...",
  "tack_direction": "..."
}
```

Fields `pace_coronet` through `tack_direction` are absent if no actionable pace.

## Step 3: Display context

Show:
- Heat silks and Firemark
- Brief paddock summary (from paddock_content)
- Current pace silks and state (if present)
- Tack text (the pace specification)

## Step 4: Branch on state

**If no actionable pace:**
- Report "All paces complete or abandoned"
- Suggest `/jjc-pace-slate` to add a new pace
- Suggest `/jjc-heat-groom` to review the heat
- Stop

**If pace_state is "rough":**
- Analyze the tack_text to understand the work
- If tack_text mentions dependencies, blockers, or sequencing concerns:
  - Surface these to the user as questions before proceeding
  - Do NOT investigate gallops data to validate the system's pace selection
  - If sequencing appears wrong, suggest `/jjc-heat-groom` to reorder or refine
- Read any files referenced in the pace spec
- Propose a concrete approach (2-4 bullets)
- Create chalk APPROACH marker: `./tt/vvw-r.RunVVX.sh jjx_chalk <PACE_CORONET> --marker A --description "<approach summary>"`
- Ask: "Ready to proceed with this approach?"
- On approval: Begin work directly

**If pace_state is "primed":**
- The pace has explicit direction in tack_direction
- Create chalk FLY marker: `./tt/vvw-r.RunVVX.sh jjx_chalk <PACE_CORONET> --marker F --description "Executing primed pace"`
- Execute per the direction autonomously (no confirmation needed)
- When complete, run `/jjc-pace-wrap` to mark done

## Context preservation

Store for use by other commands:
- Current FIREMARK (for heat-level operations)
- Current PACE_CORONET (primary identifier for pace-level operations)
- Current PACE_SILKS (for display)

## Available Operations

- `/jjc-heat-mount` — Begin work on next pace
- `/jjc-heat-muster` — List all heats
- `/jjc-heat-groom` — Review and refine heat
- `/jjc-heat-nominate` — Create new heat
- `/jjc-pace-slate` — Add a new pace
- `/jjc-pace-notch` — JJ-aware git commit
- `/jjc-parade-overview` — Heat summary
