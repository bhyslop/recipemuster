---
argument-hint: [<coronet>...] | --move <coronet> --before|--after|--first|--last
description: Reorder paces within a heat
---

Reorder paces within a Job Jockey heat. Supports two modes:

**Order mode** — replace entire sequence:
```bash
/jjc-heat-rail ₢AAAAD ₢AAAAB ₢AAAAC
```

**Move mode** — relocate a single pace:
```bash
/jjc-heat-rail --move ₢AAAAD --first
/jjc-heat-rail --move ₢AAAAD --last
/jjc-heat-rail --move ₢AAAAD --before ₢AAAAB
/jjc-heat-rail --move ₢AAAAD --after ₢AAAAC
```

Arguments: $ARGUMENTS

## Mode Detection

**If $ARGUMENTS contains `--move`:** Use move mode
**Otherwise:** Use order mode

## Order Mode

### Step 1: Parse Coronets

Extract Coronets from $ARGUMENTS (space-separated list like `₢AAAAD ₢AAAAB ₢AAAAC`).

**If fewer than 2 Coronets:**
- Error: "Usage: /jjc-heat-rail <coronet> <coronet> [coronet...]"

### Step 2: Get heat context

Extract Firemark from first Coronet (e.g., `₢AAAAD` → `AA`).

### Step 3: Reorder

Run:
```bash
vvx jjx_rail <FIREMARK> <CORONET1> <CORONET2> [CORONET3...]
```

**Validation:**
- Count must match current pace count
- No duplicates
- All Coronets must exist in the heat
- All Coronets must belong to the specified heat

## Move Mode

### Step 1: Parse arguments

Extract from $ARGUMENTS:
- `--move <CORONET>` — the pace to relocate
- One positioning flag: `--before <CORONET>`, `--after <CORONET>`, `--first`, or `--last`

**Validation:**
- `--move` requires exactly one positioning flag
- Cannot combine with positional Coronets

### Step 2: Get heat context

Extract Firemark from the move Coronet.

### Step 3: Relocate

Run:
```bash
vvx jjx_rail <FIREMARK> --move <CORONET> --first
vvx jjx_rail <FIREMARK> --move <CORONET> --last
vvx jjx_rail <FIREMARK> --move <CORONET> --before <TARGET>
vvx jjx_rail <FIREMARK> --move <CORONET> --after <TARGET>
```

**Validation:**
- Move Coronet must exist in heat
- Target Coronet (for --before/--after) must exist and differ from move Coronet

## Output

On success, the command outputs the new order (one Coronet per line).

Run parade to display the reordered pace list with silks and states:
```bash
vvx jjx_parade <FIREMARK> --format order
```

## Error handling

On failure, report the error from vvx.

Common errors:
- "Heat not found" — invalid Firemark
- "Pace not found" — Coronet doesn't exist in heat
- "Cannot combine --move with positional coronets" — mode conflict
- "Move mode requires exactly one positioning flag" — missing position
- "Cannot position pace relative to itself" — self-reference
- "Order count mismatch" — order mode must include all paces
- "Order contains duplicate Coronets" — each pace listed once
