---
argument-hint: [message]
description: Guarded git commit
---

Create a guarded git commit via VVK infrastructure.

Arguments: $ARGUMENTS (optional commit message; if omitted, Claude generates from diff)

Use this command for non-JJ repositories or when you don't need heat/pace context.
For JJ-aware commits, use `/jjc-pace-notch` instead.

## Step 1: Execute commit (in background)

Run the commit command in the background using `run_in_background: true`.

**If $ARGUMENTS provided (user gave message):**
```bash
vvx commit --message "<$ARGUMENTS>"
```

**If $ARGUMENTS empty (generate message):**
```bash
vvx commit
```

The `vvx commit` command will:
1. Acquire lock (`refs/vvg/locks/vvx`)
2. Stage all files including untracked (`git add -A`)
3. Run size guard (reject if staged changes too large)
4. Generate commit message from diff using `claude --print` (if not provided)
5. Commit with Co-Authored-By trailer
6. Release lock

## Step 2: Report result

Use `TaskOutput` to retrieve the background task result, then report:

On success:
- Commit hash (first line of stdout)
- "Push when ready: `git push`"

On failure, report the error from vvx.

## Commit message format

The commit will be formatted as:
```
<message>

Co-Authored-By: Claude <noreply@anthropic.com>
```

## Size guard

If staged changes exceed the configured limit, the commit will be rejected.
This prevents accidentally committing large binary files or generated content.

## Error handling

Common errors:
- "Lock held" — another vvx operation is in progress; wait and retry
- "Size guard failed" — staged changes too large; unstage large files
- "claude CLI not found" — install claude CLI or provide --message
