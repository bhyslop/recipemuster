---
argument-hint:
description: JJ-aware git commit with Claude-generated message
---

Create a git commit with Job Jockey heat/pace context prefix. Claude always generates the commit message from the diff.

## Execution

Use FIREMARK and PACE_SILKS from current session context. If no context, error: "No heat context. Run /jjc-heat-mount first."

```bash
./tt/vvw-r.RunVVX.sh jjx_notch <FIREMARK> --pace <PACE_SILKS>
```

The Rust command handles everything: lock, staging, guard, message generation, commit, release.

Report the commit hash on success, or the error on failure.
