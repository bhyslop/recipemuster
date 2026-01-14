---
argument-hint: <coronet> [coronet...]
description: Reorder paces within a heat
---

Reorder paces within a Job Jockey heat.

Arguments: $ARGUMENTS (space-separated Coronets in desired order)

## Step 1: Parse arguments

Extract Coronets from $ARGUMENTS (space-separated list like `₢AAAAD ₢AAAAB ₢AAAAC`).

**If $ARGUMENTS is empty or has fewer than 2 Coronets:**
- Error: "Usage: /jjc-heat-rail <coronet> <coronet> [coronet...]"
- Example: `/jjc-heat-rail ₢AAAAD ₢AAAAB ₢AAAAC ₢AAAAA`

**Syntax note:** Coronets are positional arguments, NOT a `--order` flag.

## Step 2: Get heat context

The Firemark is embedded in the Coronets (first two characters after ₢).

**If FIREMARK is available from current session context:**
- Verify it matches the Coronets' embedded Firemark

**Otherwise:**
- Extract Firemark from first Coronet (e.g., `₢AAAAD` → `AA`)

## Step 3: Reorder paces

Run:
```bash
vvx jjx_rail <FIREMARK> <CORONET1> <CORONET2> [CORONET3...]
```

Example:
```bash
vvx jjx_rail AA ₢AAAAD ₢AAAAB ₢AAAAC ₢AAAAA
```

**Important:** All existing Coronets must be included exactly once. The command validates:
- Count matches current pace count
- No duplicates
- All Coronets exist in the heat
- All Coronets belong to the specified heat

## Step 4: Confirm new order

On success, run parade to show the new order:
```bash
vvx jjx_parade <FIREMARK>
```

Display the reordered pace list with silks and states.

## Error handling

On failure, report the error from vvx.

Common errors:
- "Heat not found" — invalid Firemark
- "Coronet count mismatch" — must include all paces
- "Duplicate coronet" — each pace listed once
- "Coronet not found" — typo or wrong heat
