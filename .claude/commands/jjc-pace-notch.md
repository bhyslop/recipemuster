---
argument-hint: [message]
description: JJ-aware git commit
---

Create a git commit with Job Jockey heat/pace context prefix.

Arguments: $ARGUMENTS (optional commit message; if omitted, Claude generates from diff)

## Execution

Use FIREMARK and PACE_SILKS from current session context. If no context, error: "No heat context. Run /jjc-heat-mount first."

**With message:**
```bash
./tt/vvw-r.RunVVX.sh jjx_notch <FIREMARK> --pace <PACE_SILKS> --message "<$ARGUMENTS>"
```

**Without message (Claude generates):**
```bash
./tt/vvw-r.RunVVX.sh jjx_notch <FIREMARK> --pace <PACE_SILKS>
```

The Rust command handles everything: lock, staging, guard, message generation, commit, release.

Report the commit hash on success, or the error on failure.
