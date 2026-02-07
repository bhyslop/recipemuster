---
argument-hint: <source-firemark> <dest-firemark> <coronet> [coronet...]
description: Draft paces between heats with context merge ceremony
---

Move one or more paces from a source heat to a destination heat. This is the ceremony that wraps the `jjx_transfer` primitive with context merge and steeplechase tracking.

Arguments: $ARGUMENTS

## Prerequisites

- Gallops JSON must exist
- Source and destination heats must exist
- All specified Coronets must exist in source heat

## Step 1-5: Draft paces using jjx_transfer

Extract from $ARGUMENTS:
- First positional: source Firemark
- Second positional: destination Firemark
- Remaining positionals: one or more Coronets to draft

Build a JSON array of coronets and pipe to jjx_transfer:

```bash
# Parse arguments
SOURCE_FIREMARK="${1}"
DEST_FIREMARK="${2}"
shift 2
CORONETS=("$@")

# Build JSON coronet array
CORONET_JSON="["
for coronet in "${CORONETS[@]}"; do
  CORONET_JSON="${CORONET_JSON}\"${coronet}\","
done
CORONET_JSON="${CORONET_JSON%,}]"

# Run jjx_transfer and capture JSON output
RESTRING_OUTPUT=$(cat <<SPEC | ./tt/vvw-r.RunVVX.sh jjx_transfer "${SOURCE_FIREMARK}" --to "${DEST_FIREMARK}"
${CORONET_JSON}
SPEC
)

# Parse JSON output
SOURCE_PADDOCK=$(echo "${RESTRING_OUTPUT}" | jq -r '.source_paddock')
DEST_PADDOCK=$(echo "${RESTRING_OUTPUT}" | jq -r '.dest_paddock')
DRAFTED_ARRAY=$(echo "${RESTRING_OUTPUT}" | jq -r '.drafted[]')
SOURCE_SILKS=$(echo "${RESTRING_OUTPUT}" | jq -r '.source_silks')
DEST_SILKS=$(echo "${RESTRING_OUTPUT}" | jq -r '.dest_silks')
SOURCE_IS_EMPTY=$(echo "${RESTRING_OUTPUT}" | jq -r '.source_is_empty')
```

**On any failure:** Report error and stop. jjx_transfer is atomic - no partial drafts.

Display draft summary:
- Number of paces drafted
- Source heat: ₣{SOURCE_FIREMARK} ({SOURCE_SILKS})
- Destination heat: ₣{DEST_FIREMARK} ({DEST_SILKS})
- Mapping table from DRAFTED_ARRAY output

**If source is now empty** (SOURCE_IS_EMPTY is true):
- Warn that source heat is now empty and suggest considering retirement.

## Step 6: Context merge analysis

This is the critical step for preserving pace context across heats. Use the parsed paddock paths from jjx_transfer output.

### Step 6a: Gather materials

Read three sources:
1. Source paddock (`.claude/jjm/${SOURCE_PADDOCK}.md`)
2. Destination paddock (`.claude/jjm/${DEST_PADDOCK}.md`)
3. Specs of the paces being drafted (from DRAFTED_ARRAY output)

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

Use `jjx_revise_docket` with the combined text via stdin:
```bash
cat <<'DOCKET' | ./tt/vvw-r.RunVVX.sh jjx_revise_docket <CORONET>
[Imported context from ₣{SOURCE_FIREMARK}]
{original docket text}
DOCKET
```

### Step 6e: Update destination paddock (if needed)

If the user identified context that belongs in the destination paddock (rather than just in tack), edit `.claude/jjm/${DEST_PADDOCK}.md` to incorporate it.

### Step 6f: Clean source paddock references

Remove or annotate references to the drafted paces in the source paddock, since those paces no longer live there.

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
