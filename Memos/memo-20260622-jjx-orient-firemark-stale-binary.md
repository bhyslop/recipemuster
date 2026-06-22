# jjx_orient "missing field firemark" is a stale-binary symptom

**Date:** 2026-06-22
**Surfaced during:** mount of ₣Bi (rbk-14-mvp-loose-ends)

## Symptom

`jjx_orient {}` — the documented gazette-halter path (target supplied solely
via a `# jjezs_halter` notice in `gazette_in.md`, no `firemark` param) — fails
with:

```
jjx jjx_orient: invalid params: missing field `firemark`
```

and only succeeds when called the *old* way, `{"firemark": "Bi"}`. This is the
exact inverse of what CLAUDE.md documents (which says the `firemark` param is
"rejected, not a fallback").

## Root cause — NOT a doc or code bug

Source and docs are **forward and aligned**. The gazette-halter migration for
`jjx_orient` / `jjx_show` landed in commit `e78aa537c` (₢BDAAT, heat ₣BD): it
drops the `firemark`/`targets` params and makes a param-supplied target
*rejected* (`zjjrm_rejected_target_param`). `jjrm_mcp.rs` on `main` rejects the
param; CLAUDE.md matches.

The **deployed MCP `vvx` binary was stale** — it predated `e78aa537c` and still
demanded the param. The running server contradicting the source it was built
from is the tell: stale binary.

## Resolution

1. Rebuild: `tt/vow-b.Build.sh` (builds `vvr`, installs to VVK bin).
2. Restart Claude Code — the live MCP server holds the old binary open
   ("Text file busy" on install; see spook `079a3f2d`), so a restart is required
   to release it and serve the new one. The build alone is not enough.

Confirmed: after rebuild + restart, `jjx_orient {}` + a `# jjezs_halter`
notice resolves as documented.

## Prior sightings

- `b3381559` — first log of this exact divergence (different mount, ₣BZ).
- `079a3f2d`, `aac921e6` — same family (orient/show ergonomics + the held-open
  binary).

This memo records the second confirmed sighting and its root cause, so a future
session that hits "missing field firemark" can resolve it in one step (rebuild +
restart) instead of re-diagnosing.

## Residual class (not fixed here)

An MCP-surface migration can land in source while the running server keeps
serving old behavior until a rebuild **and** a restart, with no drift signal —
which is why this bit twice across sessions. A live-server-vs-source drift check
is a candidate ₣BD item or itch; deferred.
