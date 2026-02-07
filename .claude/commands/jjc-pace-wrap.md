---
argument-hint: [coronet]
description: Mark a pace complete
---

Mark a pace as complete, commit all changes, and record the wrap in steeplechase history.

Arguments: $ARGUMENTS (optional Coronet; uses current pace if omitted)

## Prerequisites

- **User must explicitly invoke this command** — never auto-wrap
- Gallops JSON must exist
- Pace should be in "rough" or "bridled" state
- Work on the pace should be done (verification is heat-specific; check paddock notes)

## Step 1: Identify target pace

**If $ARGUMENTS contains a Coronet (e.g., `AAAAC` or `₢AAAAC`):**
- Use that Coronet directly

**If $ARGUMENTS is empty:**
- Use PACE_CORONET from current session context (set by `/jjc-heat-mount`)
- If no context: Stop with "No pace context. Run /jjc-heat-mount first."

## Step 2: Execute wrap

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_close <CORONET>
```

**Interpret the result:**
- Exit 0 with commit hash → Success. Report the hash.
- Exit 2 → Size guard exceeded. Report: "Commit too large. Use `--size-limit N` flag if this is intentional, e.g.: `./tt/vvw-r.RunVVX.sh jjx_close <CORONET> --size-limit 200000`"
- Exit 1 → General error. Report the error message.

## Step 3: Report result

Display:
- Success/failure status and commit hash
- Any "Recommended:" lines from tool output (pass through verbatim)
