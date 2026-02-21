# Paddock: vvk-file-reservation

## Problem

Multiple Claude Code sessions (officia) work concurrently in the same repo.
Today, "don't touch other officia's files" is advisory (CLAUDE.md instructions).
There is no mechanical enforcement. Concurrent edits to the same file, or
building while another session is mid-edit, can cause silent corruption.

## Design: Git-ref-based file reservations

Use git refs as the reservation database — no JSON, no extra files. Same pattern
as the existing VVX commit lock (`refs/vvg/locks/*`).

### Primitives (Rust, in VVX binary)

```
vvx_reserve file1 file2 file3   -> rv_a3f8  (reservation ID)
vvx_release rv_a3f8             -> released
vvx_status                      -> list all active reservations
```

### Git storage

- Each reservation is a ref: `refs/vvg/reserve/{id}`
- Ref points to a blob containing newline-separated file paths
- `git update-ref` for atomic create/delete
- `git cat-file blob` to read file lists
- `git for-each-ref refs/vvg/reserve/` to enumerate
- All via `git2` crate in Rust — no shell

### Protocol

1. **Reserve**: Agent declares full file set upfront as a single batch.
   All-or-nothing — if any file is held by another reservation, the whole
   request fails. Returns an opaque reservation ID held in conversation context.

2. **Edit**: Agent edits files normally. Hook on Edit/Write checks that the
   target file is reserved by the current reservation (or unreserved).
   Edits to files reserved by others are blocked.

3. **Commit**: `vvx_commit` (or `jjx_record`) auto-releases committed files
   from the reservation.

4. **Release**: `vvx_release {id}` releases remaining files. Session end
   should release, but crash-orphaned reservations need break-lock.

5. **Break**: `/vvc-BREAK-LOCK` extended to clear all reservations.

### Batch-first discipline (avoids deadlock)

CLAUDE.md guidance: "Before editing files, run `vvx_reserve file1 file2...`
with your complete file set. If you discover you need more files mid-work,
release and re-reserve the expanded set."

All-or-nothing acquisition means no partial lock holding, no deadlock.

### Build interlock

Build tabtargets check for reservations held by OTHER reservation IDs.
Warn: "Files under active edit by another session — build may be stale."
Same-reservation builds are fine (you're building your own changes).

### Identity

No officium/session IDs needed. The reservation ID IS the identity. Agent
holds it in conversation context. Break-lock clears all reservations
regardless of holder.

### Hook enforcement

Claude Code hook on Edit/Write tool calls:
- Calls `vvx_check_reserve {filepath}` — single binary invocation
- Returns pass (file is ours or unreserved) or fail (held by another)
- Auto-acquire possible: first edit to unreserved file triggers implicit reserve

## Rejected alternative: Git worktrees

Explored using `git worktree` to give each session a physically separate
checkout — full file isolation, independent builds, no locking needed.

**Why it almost works:** Each worktree gets its own `tt/`, `Tools/`, `.buk/`,
etc. BUK's relative path resolution means tabtargets, launchers, and all shell
infrastructure work correctly within each worktree without modification.

**Why it fails:** Shared mutable state — particularly `gallops.json` — cannot
survive concurrent modification across worktrees. JSON is not line-mergeable;
concurrent JJK operations (pace transitions, commits) from different worktrees
would produce catastrophic merge conflicts. Same problem for `.claude/`
settings and any cross-session coordination state.

Worktrees solve file isolation but create a worse problem for shared state.
The reservation approach embraces git's single-working-tree model instead of
fighting it.

## Implementation notes

- All reservation logic in Rust (VVX binary, `git2` crate)
- Hook is a thin shell script calling VVX
- Reservation refs are local-only (never pushed) — `.git/refs/vvg/reserve/`
- Need to update CLAUDE.md VVK section with reservation commands and discipline
- Need to update `_core.md` insert sections (vocjjmc_core.md or equivalent)
  to teach agents the reserve-before-edit ceremony
- Stabled until we decide to commit to this path

## Open question: jjx_close git-add-A hazard and pace-reservation convergence

Discovered during ₣AT wrap (2026-02-16): `jjx_close` uses `git add -A` to
stage all changes before committing. In multi-officium context, this can
capture another officium's uncommitted work in the wrap commit. This is the
exact class of problem the reservation system aims to solve.

The deeper question: **should reservations be a standalone VVK primitive, or
integrated into JJK pace lifecycle?**

### Convergence argument

Paces already declare file read/write sets (docket, warrant). If a pace
declares its files, then:

- **Pace in_progress → files reserved.** No separate `vvx_reserve` call.
- **`jjx_close` stages only declared write-files** instead of `git add -A`.
  The pace knows what it's allowed to touch.
- **`jjx_record` already takes explicit files** — it's `jjx_close` that's
  the hazard. Aligning close with record's file-explicit model fixes this.

### Separation argument

- Not all file edits happen inside paces (ad-hoc fixes, heat-level commits).
- VVK serves non-JJK repos — reservation primitives should be reusable.
- Separation of concerns: reservation is a git primitive, pace is a workflow.

### Possible hybrid

VVK owns reservation primitives (`vvx_reserve`, `vvx_release`). JJK
automatically creates/releases reservations as paces transition:
- `jjx_enroll` or mount → `vvx_reserve` with declared write-files
- `jjx_close` → commit only reserved files, then `vvx_release`
- `jjx_record` → already file-explicit, release committed files

Rough paces without declarations either require declaration at mount time
or fall back to `git add -A` with a warning.

### Decision needed when heat unstables

1. Where does the reservation layer live (VVK, JJK, or hybrid)?
2. Should pace file declarations be required or optional?
3. What happens to rough paces that don't declare files?
4. Does `jjx_close` change to file-explicit staging, or does a hook guard
   the `git add -A` against other officia's reservations?

## References

- VVX commit lock: existing `refs/vvg/locks/*` pattern
- CLAUDE.md namespace table: `refs/{prefix}/...`
- Claude Code hooks: shell commands on tool call events
- Discussion origin: ₣AT ₢ATAAH session (burn-4-items, 2026-02-12)
- jjx_close hazard discovery: ₣AT wrap session (2026-02-16)