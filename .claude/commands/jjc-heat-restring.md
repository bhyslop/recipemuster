---
argument-hint: <source-firemark> <dest-firemark> <coronet> [coronet...]
description: Draft paces between heats with context merge ceremony
---

Move one or more paces from a source heat to a destination heat. This is the ceremony that wraps the `jjx_draft` primitive with context merge and steeplechase tracking.

Arguments: $ARGUMENTS

## Prerequisites

- Gallops JSON must exist
- Source and destination heats must exist
- All specified Coronets must exist in source heat

## Step 1: Parse arguments

Extract from $ARGUMENTS:
- First positional: source Firemark
- Second positional: destination Firemark
- Remaining positionals: one or more Coronets to draft

**Validation:**
- At least 3 arguments required (source, dest, and at least one coronet)
- Source and destination must be different

## Step 2: Validate heats exist

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_muster
```

Verify both source and destination Firemarks appear in the output.

## Step 3: Draft and commit each pace

For each Coronet:

1. Run draft:
```bash
./tt/vvw-r.RunVVX.sh jjx_draft <CORONET> --to <DEST_FIREMARK>
```

2. Capture the new coronet from stdout.

3. Commit using the new coronet:
```bash
./tt/vvw-r.RunVVX.sh jjx_notch <NEW_CORONET>
```

Collect the mapping: old coronet -> new coronet for each pace.

**On any failure:** Report error and stop. No partial drafts - if one fails, abort before any more.

## Step 4: Display draft summary

Show:
- Number of paces drafted
- Source heat Firemark and silks
- Destination heat Firemark and silks
- Mapping table: old coronet -> new coronet with pace silks

## Step 5: Check source heat status

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_get_coronets <SOURCE_FIREMARK>
```

**If output is empty (no coronets):**
- Warn that source heat is now empty and suggest considering retirement.

## Step 6: Context merge analysis

This is the critical step for preserving pace context across heats.

### Step 6a: Gather materials

Read three sources:
1. Source paddock (`.claude/jjm/jjp_{source}.md`)
2. Destination paddock (`.claude/jjm/jjp_{dest}.md`)
3. Specs of the paces being drafted (from gallops JSON or parade output)

### Step 6b: Analyze context needs

For each drafted pace, analyze what context it requires to be worked on effectively. Compare against what the destination paddock already contains.

Present a structured analysis to the user identifying:
- Context the pace needs that already exists in destination (no action needed)
- Context the pace needs that should be imported from source
- Context that conflicts between source and destination (requires user decision)
- Context in source that is irrelevant to the drafted paces (can be omitted)

For each item, cite the source location (which paddock, which section).

### Step 6c: User approves merge

Ask the user to confirm:
- Which items to import
- How to resolve any conflicts
- Whether any additional context should be captured

Do not proceed until user confirms the merge plan.

### Step 6d: Store approved context in tack

For any context the user approves for import, add a tack entry to the pace that preserves the existing spec text and prepends the imported context. The tack should record:
- That context was imported via restring
- The source heat it came from
- The substantive context itself (not just a reference)

Use `jjx_tally` with the combined text via stdin:
```bash
cat <<'PACESPEC' | ./tt/vvw-r.RunVVX.sh jjx_tally <CORONET>
[Imported context from ₣{source}]
{original spec text}
PACESPEC
```

### Step 6e: Update destination paddock (if needed)

If the user identified context that belongs in the destination paddock (rather than just in tack), edit `.claude/jjm/jjp_{dest}.md` to incorporate it.

### Step 6f: Clean source paddock references

Remove or annotate references to the drafted paces in the source paddock, since those paces no longer live there.

## Step 7: Create steeplechase marker

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_chalk <DEST_FIREMARK> --marker d --description "Restring: {N} paces from ₣{source}"
```

## Error handling

Common errors:
- "Source heat not found" — invalid source Firemark
- "Heat not found" — invalid destination Firemark
- "Cannot draft pace to same heat" — source equals destination
- "Pace not found in heat" — Coronet doesn't exist in source

## Notes

- Draft preserves pace state (rough stays rough, complete stays complete)
- Draft preserves all tack history with a note recording the transfer
- New Coronets are assigned using destination heat's seed
- Old Coronets become invalid after draft
- Context imported via tack travels with the pace if it is drafted again
