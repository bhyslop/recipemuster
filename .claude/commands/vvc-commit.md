You are dispatching a guarded git commit via VVK.

Arguments: $ARGUMENTS (commit description from user)

## Step 1: Run guard

Execute the pre-commit guard to acquire lock, stage files, and validate size:

```bash
Tools/vvk/vvg_cli.sh vvg_guard_begin
```

**If guard fails:** Stop and report the error. Do not proceed.

**If guard succeeds:** Lock is held, files are staged. Proceed to step 2.

## Step 2: Dispatch commit agent

Spawn a background agent to commit the pre-staged files:

- Use Task tool with subagent_type='general-purpose', model='haiku', run_in_background=true
- Prompt template:

```
Guarded commit agent. Description: $ARGUMENTS

You are committing pre-staged files. The lock is held and files are already staged.

1. Run: git diff --cached --stat
   Review what will be committed.

2. Construct commit message:
   - Line 1: $ARGUMENTS (under 72 chars, imperative tense)
   - Line 2: blank
   - Lines 3+: bullet points summarizing changes (from the diff)
   - Final line: Co-Authored-By: Claude <noreply@anthropic.com>

3. Run: git commit -m "<your message>"
   Use the message you constructed. Use HEREDOC format for multi-line.

4. Run: Tools/vvk/vvg_cli.sh vvg_guard_end
   This releases the commit lock.

5. Report the commit hash and files changed.
   Remind user to push when ready.

Git safety rules:
- NEVER run git add (files are already staged)
- NEVER force push or skip hooks
- NEVER commit files matching secret patterns (.env, credentials.*, etc.)
- Do NOT push automatically
```

## Step 3: Announce and return

Report "Commit dispatched" and return immediately.
Do NOT wait for agent completion.
