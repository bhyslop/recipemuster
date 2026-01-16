---
argument-hint: [message]
description: JJ-aware git commit
---

Create a git commit with Job Jockey heat/pace context prefix.

Arguments: $ARGUMENTS (optional commit message; if omitted, Claude generates from diff)

## Prerequisites

- Must have active heat context (run `/jjc-heat-mount` first)
- Must have staged or modified files to commit

## Step 1: Get context

Retrieve from current session:
- FIREMARK (current heat)
- PACE_SILKS (current pace)

If no context available:
- Error: "No heat context. Run /jjc-heat-mount first."

## Step 2: Check for changes

Run:
```bash
git status --porcelain
```

If no changes:
- Error: "Nothing to commit. Working tree clean."

## Step 3: Execute notch

**If $ARGUMENTS provided (user gave message):**
```bash
./tt/vvw-r.RunVVX.sh jjx_notch <FIREMARK> --pace <PACE_SILKS> --message "<$ARGUMENTS>"
```

**If $ARGUMENTS empty (generate message):**
```bash
./tt/vvw-r.RunVVX.sh jjx_notch <FIREMARK> --pace <PACE_SILKS>
```

The `./tt/vvw-r.RunVVX.sh jjx_notch` command will:
1. Acquire lock
2. Stage modified files (`git add -u`)
3. Run size guard
4. Generate commit message from diff (if not provided)
5. Format as: `[jj:BRAND][₣XX/pace-silks] message`
6. Commit
7. Release lock

## Step 4: Report result

On success, report:
- Commit hash
- Files changed summary
- "Push when ready: `git push`"

On failure, report the error from vvx.

## Commit message format

The commit will be formatted as:
```
[jj:BRAND][₣AA/pace-silks] <message>

Co-Authored-By: Claude <noreply@anthropic.com>
```

Where:
- BRAND = repository identifier
- ₣AA = Firemark (example)
- pace-silks = current pace silks

## Available Operations

- `/jjc-heat-mount` — Begin work on next pace
- `/jjc-heat-muster` — List all heats
- `/jjc-heat-groom` — Review and refine heat
- `/jjc-heat-nominate` — Create new heat
- `/jjc-pace-slate` — Add a new pace
- `/jjc-pace-notch` — JJ-aware git commit
- `/jjc-parade-overview` — Heat summary
