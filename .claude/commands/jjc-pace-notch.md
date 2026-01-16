---
argument-hint:
description: JJ-aware git commit with Claude-generated message
---

Create a git commit with Job Jockey pace context prefix. Claude always generates the commit message from the diff.

## Execution

Use PACE_CORONET from current session context. If no context, error: "No heat context. Run /jjc-heat-mount first."

```bash
./tt/vvw-r.RunVVX.sh jjx_notch <PACE_CORONET>
```

The coronet embeds the parent heat, so no separate firemark argument is needed.

The Rust command handles everything: lock, staging, guard, message generation, commit, release.

Report the commit hash on success, or the error on failure.
