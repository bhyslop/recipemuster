---
argument-hint: <marker> <description>
description: Add steeplechase marker
---

Create an empty commit marking a steeplechase event.

Arguments: $ARGUMENTS (format: `<MARKER> <description text>`)

Markers:
- `APPROACH` — Proposed approach for a pace (requires pace context)
- `WRAP` — Pace completion summary (requires pace context)
- `FLY` — Autonomous execution began (requires pace context)
- `DISCUSSION` — Significant decision or design discussion (pace optional)

## Prerequisites

- Must have active heat context (run `/jjc-heat-saddle` first)
- For APPROACH/WRAP/FLY markers, must have pace context

## Step 1: Parse arguments

Extract from $ARGUMENTS:
- First word = MARKER (APPROACH, WRAP, FLY, or DISCUSSION)
- Remaining text = description

If no marker or invalid marker:
- Error: "Usage: /jjc-heat-chalk <APPROACH|WRAP|FLY|DISCUSSION> <description>"

## Step 2: Get context

Retrieve from current session:
- FIREMARK (current heat) — required
- PACE_SILKS (current pace) — required for APPROACH/WRAP/FLY

If no FIREMARK:
- Error: "No heat context. Run /jjc-heat-saddle first."

If no PACE_SILKS and marker requires it:
- Error: "<MARKER> marker requires pace context."

## Step 3: Execute chalk

**For APPROACH/WRAP/FLY (pace required):**
```bash
vvx jjx_chalk <FIREMARK> --pace <PACE_SILKS> --marker <MARKER> --description "<description>"
```

**For DISCUSSION (pace optional):**
```bash
vvx jjx_chalk <FIREMARK> [--pace <PACE_SILKS>] --marker DISCUSSION --description "<description>"
```

Include `--pace` if pace context is available.

## Step 4: Report result

On success:
- "Chalk marker created: [jj:BRAND][₣XX/pace] <MARKER>: <description>"
- Report commit hash

On failure, report the error from vvx.

## Commit format

The empty commit message will be:
```
[jj:BRAND][₣XX/pace-silks] MARKER: description

Co-Authored-By: Claude <noreply@anthropic.com>
```

Or for DISCUSSION without pace:
```
[jj:BRAND][₣XX] DISCUSSION: description

Co-Authored-By: Claude <noreply@anthropic.com>
```
