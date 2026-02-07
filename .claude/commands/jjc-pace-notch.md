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

## Large Commits

The default size guard rejects commits over 50KB. For legitimate large commits:

```bash
./tt/vvw-r.RunVVX.sh jjx_record <IDENTITY> --size-limit 200000 <file1> [file2...]
```

**Requirement**: The pace docket must justify why the large commit is necessary.
