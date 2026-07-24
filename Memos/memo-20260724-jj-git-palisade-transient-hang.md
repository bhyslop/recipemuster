# Incident Report: JJ plain-git remote-op hang/panic under a github-SSH transient

**Date:** 2026-07-24
**Author:** Claude Opus 4.8 (Claude Code, groom-billet session for heat ₣B9)
**Severity:** Medium — no data loss, no lock corruption; blocked one non-critical write across repeated attempts.
**Status:** Diagnosed. Durable fix captured as a docket handed to a fable session for slate into ₣B9. This memo is provenance only.

## Summary

While bridling paces during a groom of ₣B9, `jjx_apostille` (a studbook write) repeatedly hung for
the full 120 s harness cutoff, alternating unpredictably with fast, clean `LockHeld` refusals — and,
in an earlier operator console session, a hard Rust panic. The root cause is an intermittent
`git@github.com` SSH transient under concurrent connection load, striking JJ's plain-git driver
(`Tools/jjk/vov_veiled/src/jjrfg_plaingit.rs`), which performs its remote git operations with **no
timeout and no retry**. Reads were unaffected; only the remote-coupled write path stalled.

## What I was doing

Grooming ₣B9 (footprint-delivery) and bridling its two rough paces to opus:

- `₢B9·CAABT` → opus: **landed instantly**, clean.
- `₢B9·CAAA5` → opus: **never landed** — every attempt either hung or refused on a held lock.

## Timeline (observed)

1. `jjx_apostille CAABT opus` → landed immediately.
2. `jjx_apostille CAAA5 opus` → hung 120 s → backgrounded → killed.
3. (operator changed networking)
4. `jjx_apostille CAAA5 opus` → **fast, clean** `Another commit in progress - lock held`.
5. `jjx_apostille CAAA5 opus` → hung 120 s → killed.
6. Bounded SSH probe of the studbook remote (`git ls-remote`, `ConnectTimeout=6`) → **succeeded
   instantly**; remote `refs/jjv/*` empty (no lock held).
7. `jjx_apostille CAAA5 opus` (one more, window verified open) → **hung again** → killed.

The same command, within the same minute, produced instant success (step 1), a fast clean refusal
(step 4), and an indefinite hang (steps 2/5/7). That oscillation is the signature of an intermittent
foreign-service transient, not a deterministic bug.

Earlier context, from the operator's console in a prior session: `jjy_saddle B0AAK` **panicked** with
`plain-git sight hit an unclassified git failure ... git@github.com: Permission denied (publickey)` —
the same transient in a different manifestation. (`B0AAK` itself was already a complete pace, so the
saddle was misdirected regardless; the panic is the finding, not the refusal.)

## Root cause

Two layers.

**Environmental — the Palisade, not JJ's to fix.** `git@github.com` intermittently denies or stalls
the SSH handshake. Most plausibly ssh-agent / connection-multiplexing contention: the working setup
fans many concurrent SSH connections at github (the alpha hippodrome, several billet clones, the
studbook clone, and multiple concurrent Claude sessions), and JJ's own dispatch remote-coupling
(saddle / lunge / apostille as locked studbook writes, per the ₣B9 catchword-serial ruling) adds to
that burst. The failure verdict does not match reality: a retried operation succeeds, proving access
is intact.

**JJ defect — ours, and the real finding.** `jjrfg_plaingit.rs` models a git failure as exactly two
things: a *known farrier rejection kind* (returned as `jjrfr_Rejection`) or *unclassified* (panic, via
`zjjrfg_unexpected`). There is **no third category** for a retryable foreign-service transient, and a
repo-wide scan of the driver finds **no retry, no backoff, and no connection timeout** anywhere.
Consequences:

- When the transient surfaces as `Permission denied (publickey)`, the signature is unclassified →
  **panic** — a raw backtrace, and unretryable by construction, since a panic is not a `Result` that
  any caller could catch and retry.
- When it surfaces as a connection stall, the unbounded `git` spawn **hangs indefinitely** until the
  harness cutoff.

Both are the same missing membrane. The unbounded hang is the worse of the two: it has no bound at all.

## Why reads survived but writes hung (inference)

Reads (`jjx_coronets`, `jjx_show`) returned instantly throughout; only writes hung. Inference: the
write path gleans the studbook remote to establish currency *before* it acquires the blotter lock, so
a write is remote-coupled where a read is served from the local studbook clone. Supporting evidence:
during every hang, `refs/jjv/*` was empty both locally and on the remote — the hung apostille never
reached lock-acquire, so it was stalled on the pre-lock glean.

## Safety verification (no collateral damage)

Killing a hung background task risks orphaning the blotter lock (a dead process leaving
`refs/jjv/guidon` held). Before each kill I verified, with local and bounded-remote reads, that no
lock was held locally (`git -C <studbook> for-each-ref refs/jjv/`) or remotely
(`git ls-remote ... 'refs/jjv/*'`). Both were empty every time. No lock was orphaned, no zombie
`git`/`ssh` child processes were left behind, and `jjx_coronets` confirmed `₢B9·CAAA5` stayed `rough`
with no half-write. The system remained consistent throughout.

## Recommended fix (durable)

Post JJ's vedette at its git Palisade — the pattern already proven on the RB side, cited here as
precedent rather than restated:

- Doctrine: `rbsk_vedette` / the no-completion-contract premise (RBS0), RBSCIP's call-context rule,
  the `RBGC_PROPAGATION_*` budget.
- Machinery precedent: `rbgo_curl_status_is_transient_predicate`.

Applied to `jjrfg_plaingit`:

1. A **connection / operation timeout** on every remote-touching git op — an unbounded spawn is the
   deeper defect than the panic.
2. **Transient classification** of the remote signatures (`Permission denied (publickey)`,
   `Could not read from remote repository`, connection reset) as a *distinct third category* — never a
   widening of the closed rejection-kind taxonomy the module doc rightly guards.
3. **Bounded retry with backoff** on that category, and only on exhaustion a **guided `buc_die`**
   naming the remedy — never a raw panic or an unbounded hang.
4. Spec-home the new category in the farrier sheaf (JJSVF) first: the taxonomy is closed by the sheaf,
   so this is a spec change before a code change.

The load-bearing artifact is the docket handed off for slate into ₣B9; on landing, the durable
knowledge lives in JJSVF. Per project doctrine, this memo is provenance, never authority.

## Environment mitigations (operator side, independent of the code fix)

- SSH `ControlMaster` / `ControlPersist` multiplexing to github collapses the concurrent-connection
  burst into one persistent channel — directly targets the contention cause.
- Confirm ssh-agent holds the key stably (`AddKeysToAgent`), and re-examine the networking change made
  mid-incident: it appears to have shifted the failure from a fast publickey-denial to an indefinite
  stall.

## Disposition

- `₢B9·CAAA5` left `rough`, deliberately. It is the terminal boundary pace of the ₣B9 phase ladder and
  runs dead-last; its bridle designation blocks nothing. It will be bridled once the SSH path is stable
  or the vedette lands.
- The other ₣B9 paces are bridled or landed; the heat continued advancing in parallel during this
  incident.
- The vedette-fix docket has been handed to a fable session for review and slate.
