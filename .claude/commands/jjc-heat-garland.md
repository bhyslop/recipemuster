---
argument-hint: [firemark]
description: Transfer remaining paces to continuation heat
---

Transfer remaining paces from a completed heat to a fresh continuation heat, preserving the original for retrospective.

Use this when a heat has grown beyond its original scope and you want to:
- Preserve completed work in a garlanded archive
- Continue remaining work in a fresh heat with clean context

Arguments: $ARGUMENTS (optional: Firemark to garland; if omitted, uses first racing heat)

## Step 1: Parse arguments

Extract Firemark from $ARGUMENTS if provided.

**If $ARGUMENTS contains a Firemark (e.g., `AA` or `₣AA`):**
- Use that Firemark directly
- Skip to Step 2

**If $ARGUMENTS is empty:**
- Run: `./tt/vvw-r.RunVVX.sh jjx_list`
- Parse TSV output: `FIREMARK<TAB>SILKS<TAB>STATUS<TAB>PACE_COUNT`
- Filter for lines where STATUS column is "racing"

**If 0 racing heats:**
- Error: "No racing heats found. Use `jjx_list` to see all heats."
- Stop

**If 1 racing heat:**
- Use that heat's Firemark automatically

**If 2+ racing heats:**
- Use the first racing heat (most recently activated)
- Report which heat was selected

## Step 2: Get current state

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_show <FIREMARK> --remaining
```

Parse output to extract:
- Heat silks and status
- Total pace count
- Actionable pace count (remaining work to transfer)

**If heat is not racing:**
- Error: "Heat ₣<FIREMARK> is not racing. Only racing heats can be garlanded."
- Stop

**If no actionable paces:**
- Error: "Heat ₣<FIREMARK> has no actionable paces to transfer. Use `/jjc-heat-retire` instead."
- Stop

## Step 3: Confirmation prompt

Display:
```
Garland ₣<FIREMARK> (<silks>)?

This will:
- Transfer <N> actionable paces to a new continuation heat
- Rename current heat to garlanded-<silks> and stable it

⚠ Context-heavy operation — consider /clear first if session is long.

Proceed? [y/n]
```

**On n:** Stop without changes.

**On y:** Continue to Step 4.

## Step 4: Execute garland

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_continue <FIREMARK>
```

Capture output which includes:
- Old heat's new silks (garlanded-*)
- Old heat's retained pace count
- New heat's Firemark and silks
- New heat's transferred pace count

## Step 5: Report summary

On success:
```
Garlanded ₣<OLD_FIREMARK> → <old_silks> (stabled, <retained> paces retained)
New heat: ₣<NEW_FIREMARK> <new_silks> (racing, <transferred> paces)

Next: /jjc-heat-groom <NEW_FIREMARK> to review paddock
```

On failure, report the error from vvx.

## Step 6: Auto-commit

Run:
```bash
./tt/vvw-r.RunVVX.sh vvx_commit --message "Garland: ₣<OLD_FIREMARK> → ₣<NEW_FIREMARK>"
```

On commit failure (e.g., lock held), report error but don't fail the operation — gallops changes are already saved.

## Error handling

Common errors:
- "Heat not found" — invalid Firemark
- "Heat not racing" — can only garland racing heats
- "No actionable paces" — nothing to transfer, use retire instead

## Available Operations

**Commits:** Always use `/jjc-pace-notch` — never vvx_commit directly.

- `/jjc-heat-garland` — Transfer paces to continuation (this command)
- `/jjc-heat-retire` — Archive completed heat (no remaining work)
- `/jjc-heat-mount` — Begin work on next pace
- `jjx_list` — List all heats
- `/jjc-heat-groom` — Review and refine heat
- `/jjc-parade-overview` — Heat summary
