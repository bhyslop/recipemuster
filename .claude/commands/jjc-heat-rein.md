---
argument-hint: <firemark> [--limit N]
description: Display steeplechase history for a heat
---

Show recent steeplechase entries (git commit history) for a heat.

Arguments: $ARGUMENTS (required Firemark, optional --limit N, default 20)

## Prerequisites

## Step 1: Parse arguments

**Extract Firemark** (required):
- First argument should be Firemark (e.g., `AE` or `₣AE`)
- If missing: Error "Firemark required. Usage: /jjc-heat-rein <firemark> [--limit N]"

**Extract limit** (optional):
- Look for `--limit N` in arguments
- Default to 20 if not specified

## Step 2: Get steeplechase entries

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_log <FIREMARK> --limit <N>
```

Output is already formatted as human-readable text with columns:
- `timestamp`: "YYYY-MM-DD HH:MM" (16 chars)
- `commit`: abbreviated git SHA (8 chars)
- `action`: single letter code in brackets (3 chars, e.g., `[W]`)
- `coronet`: "₢XXXXX" for pace entries (7 chars, blank if heat-level)
- `subject`: description text

## Step 3: Display output

Output format:
```
YYYY-MM-DD HH:MM  abc123ef  [W]  ₢XXXXX  Wrap description
YYYY-MM-DD HH:MM  def456ab  [T]          Tally: pace-silks
YYYY-MM-DD HH:MM  1234abcd  [F]  ₢XXXXX  Fly: agent execution
YYYY-MM-DD HH:MM  5678efgh  [n]  ₢XXXXX  Commit message
YYYY-MM-DD HH:MM  9abc0123  [S]          Slate: new-pace-silks
```

Action code meanings:
- `W` = Wrap (pace complete)
- `T` = Tally (state transition)
- `F` = Fly (bridled execution)
- `A` = Approach (plan marker)
- `S` = Slate (new pace)
- `n` = Notch (standard commit)
- `r` = Rail (reorder)
- `D` = Discussion

## Available Operations

- `/jjc-heat-rein` — Display steeplechase history
- `/jjc-heat-mount` — Begin work on next pace
- `/jjc-heat-muster` — List all heats
- `/jjc-heat-groom` — Review and refine heat
- `/jjc-parade-overview` — Heat summary
