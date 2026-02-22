---
argument-hint: --move <coronet> --before|--after|--first|--last
description: Reorder paces within a heat
---

Relocate a single pace within a Job Jockey heat.

```bash
/jjc-heat-rail --move ₢AAAAE --first
/jjc-heat-rail --move ₢AAAAE --last
/jjc-heat-rail --move ₢AAAAE --before ₢AAAAC
/jjc-heat-rail --move ₢AAAAE --after ₢AAAAD
```

Arguments: $ARGUMENTS

## Step 1: Parse arguments

Extract from $ARGUMENTS:
- `--move <CORONET>` — the pace to relocate
- One positioning flag: `--before <CORONET>`, `--after <CORONET>`, `--first`, or `--last`

**Validation:**
- `--move` requires exactly one positioning flag

## Step 2: Get heat context

Extract Firemark from the move Coronet.

## Step 3: Relocate

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_reorder <FIREMARK> --move <CORONET> --first
./tt/vvw-r.RunVVX.sh jjx_reorder <FIREMARK> --move <CORONET> --last
./tt/vvw-r.RunVVX.sh jjx_reorder <FIREMARK> --move <CORONET> --before <TARGET>
./tt/vvw-r.RunVVX.sh jjx_reorder <FIREMARK> --move <CORONET> --after <TARGET>
```

**Validation:**
- Move Coronet must exist in heat
- Target Coronet (for --before/--after) must exist and differ from move Coronet

## Output

On success, the command outputs the new order (one Coronet per line).

Run parade to display the reordered pace list with silks and states:
```bash
./tt/vvw-r.RunVVX.sh jjx_show <FIREMARK>
```

## Auto-commit changes

Run guarded commit:
```bash
./tt/vvw-r.RunVVX.sh vvx_commit --message "Rail: reorder ₣<FIREMARK>"
```

On failure (e.g., lock held), report error but don't fail the operation — gallops changes are already saved.

## Error handling

On failure, report the error from vvx.

Common errors:
- "Heat not found" — invalid Firemark
- "Pace not found" — Coronet doesn't exist in heat
- "Move mode requires exactly one positioning flag" — missing position
- "Cannot position pace relative to itself" — self-reference

## Available Operations

- `/jjc-pace-slate` — Add a new pace
- `/jjc-pace-reslate` — Refine pace docket
- `jjx_close` — Mark pace complete
- `/jjc-pace-bridle` — Arm pace for autonomous execution
- `/jjc-heat-mount` — Begin work on next pace
- `/jjc-heat-rail` — Reorder paces
- `/jjc-heat-chalk` — Add steeplechase marker
- `/jjc-parade-overview` — Heat summary
