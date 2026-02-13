---
argument-hint: <file1> [file2...]
description: JJ-aware git commit with Claude-generated message
---

Create a git commit with Job Jockey context prefix. Requires explicit file list.

Arguments: $ARGUMENTS (one or more file paths to commit)

## Prerequisites

- Must have pace context from `/jjc-heat-mount`, OR
- Provide heat-only context with Firemark

## Execution

**Step 1: Synthesize commit intent from conversation**

Generate a one-line commit message based on what was just accomplished in this session. Draw on:
- The work discussed and completed
- Files being committed and why
- Heat/pace context

Use this as `--intent "<message>"` in the command. Do not prompt the user — synthesize from your awareness of the conversation.

**Step 2: Execute commit**

**Pace-affiliated commit (default):**
Use PACE_CORONET from current session context:
```bash
./tt/vvw-r.RunVVX.sh jjx_record <PACE_CORONET> [--intent "<message>"] <file1> [file2...]
```

**Heat-only commit (no pace affiliation):**
Use FIREMARK for commits that affect the heat but not a specific pace:
```bash
./tt/vvw-r.RunVVX.sh jjx_record <FIREMARK> [--intent "<message>"] <file1> [file2...]
```

The Rust command handles: lock, staging specified files only, guard, message generation (or using intent), commit, release.

## File list requirement

- At least one file must be specified
- All files must exist
- Only specified files are staged and committed
- Warning printed for uncommitted changes outside the file list

## If the size guard rejects the commit

Do NOT pre-check commit size — just attempt the commit and react if it fails. The guard's measurement is smarter than anything you can estimate.

If `jjx_record` fails with a size-limit error, **ask the user** whether to proceed. Report the guard's measured size and let them decide. Do NOT autonomously retry with `--size-limit`.

On user approval, retry with an explicit limit:

```bash
./tt/vvw-r.RunVVX.sh jjx_record <IDENTITY> --size-limit 200000 <file1> [file2...]
```
