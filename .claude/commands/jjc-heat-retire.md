---
argument-hint: <firemark>
description: Retire a completed heat
---

Retire a heat: extract trophy, archive to retired/, remove from gallops.

Arguments: $ARGUMENTS (required firemark, e.g., "AB" or "₣AB")

## Step 1: Extract trophy data

```bash
./tt/vvw-r.RunVVX.sh jjx_retire $ARGUMENTS
```

Capture the JSON output - this is the complete heat archive including paddock and all pace history.

## Step 2: Create trophy file

Parse the JSON and create trophy file at:
```
.claude/jjm/retired/jjh_<created>-r<today>-<silks>.md
```

Where:
- `<created>` = heat creation date from JSON (YYMMDD format)
- `<today>` = today's date (YYMMDD format)
- `<silks>` = heat silks from JSON

Trophy file format:
```markdown
# Heat Trophy: <silks>

**Firemark:** <firemark>
**Created:** <created>
**Retired:** <today>
**Status:** retired

## Paddock

<paddock_content from JSON>

## Paces

<For each pace in paces array:>
### <silks> (<coronet>) [<final state>]

<Format tacks as history - newest first>
```

## Step 3: Remove from gallops

Edit `.claude/jjm/jjg_gallops.json`:
1. Remove the heat entry from the `heats` object
2. Do NOT change `next_heat_seed`

## Step 4: Delete paddock file

Delete the paddock file path from the JSON (e.g., `.claude/jjm/jjp_AB.md`)

## Step 5: Commit

```bash
./tt/vvw-r.RunVVX.sh vvx_commit --message "Retire: ₣<firemark> <silks>"
```

## Step 6: Report

- Trophy file location
- Commit hash
- "Heat ₣<firemark> retired successfully"

## Available Operations

- `/jjc-heat-muster` — List all heats
- `/jjc-heat-retire` — Retire completed heat (this command)
- `/jjc-parade-full` — View heat details before retiring
