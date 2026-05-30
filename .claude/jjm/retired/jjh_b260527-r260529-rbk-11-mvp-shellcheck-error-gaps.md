# Heat Trophy: rbk-11-mvp-shellcheck-error-gaps

**Firemark:** ₣BT
**Created:** 260527
**Retired:** 260529
**Status:** retired

## Paddock

## Shape: close the gate, then pay the debt

The defect is a process gap, not a list of warnings. shellcheck qualification
(`buq_shellcheck` — scans every `*.sh` under `Tools/` with `-S style` against
`busc_shellcheckrc`) runs only under release qualify (`rbw-tr`) or the standalone
`buw-qsc` colophon. Nothing on the routine fast path (`rbw-tq`) gates it, so style
debt accumulates invisibly between releases. A baseline sweep surfaced a backlog
that nobody had been shown.

Two halves, in this order: **(1)** fold shellcheck into `rbw-tq` so every fast
qualify gates it and retire the standalone `buw-qsc`; **(2)** drive the surfaced
baseline to zero. Halve-one first is deliberate — once the gate is routine, the
backlog can't silently regrow while we work it.

## The backlog has three textures

Run the shellcheck colophon for the authoritative live list (it drifts as fixes
land — do not trust any enumeration written here). The findings sort into:

- **Mechanical, BCG-canonical** — unquoted numeric `test` operands, unquoted inner
  expansions in `#`/`%` operators, the `("${dir}"/${var}.*)` glob-array quoting
  pattern (same shape as the `buyy_tt_yawp` repair already landed under ₣BQ). Blind-safe.
- **Mechanical and a safety win** — a `rm -rf "/home/${var}"` wanting a `:?` guard.
  Worth doing for its own sake, independent of the warning.
- **Judgment, possibly masking real bugs** — these do NOT get blind-quoted:
  - `rbfl_FoundryLedger.sh` carries an array-used-then-assigned-scalar cluster.
    That class can be a genuine data-flow bug, not a style nit — read the whole
    function before touching.
  - `bujb_jurisdiction.sh` stores an `ssh-keygen ... -P '' -f` command in a string
    and word-splits it at several call sites. The warning itself questions whether
    the empty-passphrase `''` survives — a behavioral question to verify, and the
    array fix changes a tinder constant's type across all its sites.
  - The `rbgjb*`/`rbgjr*` Cloud Build job scripts raise an interpreter question
    (pipefail flagged as undefined → read as POSIX `sh`) entangled with intended
    word-splitting. Settle the shebang/interpreter before quoting.
  - A remote-side tilde (`"~/..."` handed to a double-hop ssh pipe) and a
    literal-backslash `case` pattern are likely intentional — candidates for a
    documented `busc_shellcheckrc` entry rather than a code change.

## Locked decisions

- **One routine gate.** shellcheck folds into `rbw-tq`; the standalone `buw-qsc`
  is retired. The BUK `buq_shellcheck` engine stays — only the invocation path
  consolidates. `buq_shellcheck` is BUK-level (reusable), so the engine must not
  gain rbk-specific assumptions.
- **Fixes are opportunistic-correct, not blanket-suppressed.** A finding is silenced
  only when it is a true false-positive, only in `busc_shellcheckrc`, never inline,
  and always with a rationale. Genuine findings get fixed.
- **Judgment findings get investigated.** The `rbfl` and `bujb` clusters are treated
  as bug-or-not questions, resolved explicitly — not quoted away on reflex.

## Leaning, not yet locked

- **Absent-shellcheck behavior on the fast path.** `rbw-tq` is the dependency-light,
  fast qualify; shellcheck is an external tool and scanning the tree costs seconds —
  the likely original reason it lived in release qualify. Lean: **fail when shellcheck
  is present, loud-skip when absent**, so dependency-light machines still pass fast
  qualify while anyone with the tool is gated. Settle this at slate time; it shapes
  whether folding into `rbw-tq` is unconditional.

## Done looks like

- `rbw-tq` runs shellcheck and is green; the standalone `buw-qsc` is gone — a single
  routine gate, no release-only blind spot.
- Every baseline finding is either fixed or a documented `busc_shellcheckrc` survivor
  with rationale.
- The two suspect clusters (`rbfl` array/scalar, `bujb` ssh-keygen empty-passphrase)
  are explicitly resolved as bug or non-bug, with the verdict recorded.

## Paces

### close-the-gate-fold-shellcheck-into-rbw-tq (₢BTAAA) [complete]

**[260527-1040] complete**

## Character
Mechanical plumbing with one locked policy call. Routing change, not logic.

## Goal
Fold shellcheck onto the routine fast-qualify path so style debt can no longer
accumulate invisibly between releases, and retire the now-redundant standalone
invocation.

## Locked decisions
- `rbw-tq` (QualifyFast) gains a shellcheck gate that runs `buq_shellcheck`.
- **Hard-require shellcheck**: absent shellcheck → `rbw-tq` fails. (Operator
  decision at slate time — strongest gate; dependency-light machines must
  install shellcheck to pass fast qualify.)
- The standalone `buw-qsc` colophon is retired (tabtarget + its zipper/launcher
  enrollment).
- `buq_shellcheck` stays BUK-level and reusable — the gate must add no
  rbk-specific assumptions to the engine; only the invocation path consolidates.

## Done looks like
- `rbw-tq` invokes shellcheck and reports its findings; `buw-qsc` is gone.
- The gate runs RED against the remaining ~12 baseline findings — expected and
  intentional; later paces drive it green.

## Entry points
- `rbw-tq` lives in the qualify orchestrator (`Tools/rbk/rbq_Qualify.sh`,
  `rbw-tq`/`rbw-tr` colophons).
- Find the standalone path: `grep -rn 'buw-qsc\|buq_shellcheck' tt/ Tools/`.

**[260527-1032] rough**

## Character
Mechanical plumbing with one locked policy call. Routing change, not logic.

## Goal
Fold shellcheck onto the routine fast-qualify path so style debt can no longer
accumulate invisibly between releases, and retire the now-redundant standalone
invocation.

## Locked decisions
- `rbw-tq` (QualifyFast) gains a shellcheck gate that runs `buq_shellcheck`.
- **Hard-require shellcheck**: absent shellcheck → `rbw-tq` fails. (Operator
  decision at slate time — strongest gate; dependency-light machines must
  install shellcheck to pass fast qualify.)
- The standalone `buw-qsc` colophon is retired (tabtarget + its zipper/launcher
  enrollment).
- `buq_shellcheck` stays BUK-level and reusable — the gate must add no
  rbk-specific assumptions to the engine; only the invocation path consolidates.

## Done looks like
- `rbw-tq` invokes shellcheck and reports its findings; `buw-qsc` is gone.
- The gate runs RED against the remaining ~12 baseline findings — expected and
  intentional; later paces drive it green.

## Entry points
- `rbw-tq` lives in the qualify orchestrator (`Tools/rbk/rbq_Qualify.sh`,
  `rbw-tq`/`rbw-tr` colophons).
- Find the standalone path: `grep -rn 'buw-qsc\|buq_shellcheck' tt/ Tools/`.

### mechanical-blind-safe-quoting-sweep (₢BTAAB) [complete]

**[260527-1135] complete**

## Character
Mechanical, blind-safe. BCG-canonical quoting plus one safety guard.

## Goal
Clear the mechanical baseline findings — the ones that are correct-by-rote under
BCG quoting discipline, plus the one `rm -rf` guard worth doing for its own sake.

## Scope
Run the live list (`buw-qsc`, or `rbw-tq` once the gate lands) — it drifts as
fixes land, so trust the run, not any enumeration. The mechanical findings:
- BCG-canonical quoting: SC2086 (double-quote), SC2295 (quote expansions inside
  `${..}`), SC2231 (quote for-loop glob), SC2001. Same shape as the
  `buyy_tt_yawp` repair landed under ₣BQ — blind-safe.
- Safety win: SC2115 on `jjfp_fundus.sh` — `rm -rf` wants a `${var:?}` guard so
  it can never expand to `/home`. Worth doing independent of the warning.

## Exclusions
- Do NOT touch the two shellcheckrc-survivor candidates (the `jjfp_fundus.sh`
  remote tilde SC2088 and the `rbfh_FoundryHygiene.sh` literal-backslash SC1003)
  — those are the next pace.
- `rbv_PodmanVM.sh` sits under `vov_veiled/FUTURE/`; confirm it is in scan scope
  intentionally before editing — if FUTURE/ should be excluded, that is a
  shellcheckrc/scan-path question, not a code fix.

## Done looks like
- Every mechanical finding is fixed; only the two survivor candidates remain.

**[260527-1032] rough**

## Character
Mechanical, blind-safe. BCG-canonical quoting plus one safety guard.

## Goal
Clear the mechanical baseline findings — the ones that are correct-by-rote under
BCG quoting discipline, plus the one `rm -rf` guard worth doing for its own sake.

## Scope
Run the live list (`buw-qsc`, or `rbw-tq` once the gate lands) — it drifts as
fixes land, so trust the run, not any enumeration. The mechanical findings:
- BCG-canonical quoting: SC2086 (double-quote), SC2295 (quote expansions inside
  `${..}`), SC2231 (quote for-loop glob), SC2001. Same shape as the
  `buyy_tt_yawp` repair landed under ₣BQ — blind-safe.
- Safety win: SC2115 on `jjfp_fundus.sh` — `rm -rf` wants a `${var:?}` guard so
  it can never expand to `/home`. Worth doing independent of the warning.

## Exclusions
- Do NOT touch the two shellcheckrc-survivor candidates (the `jjfp_fundus.sh`
  remote tilde SC2088 and the `rbfh_FoundryHygiene.sh` literal-backslash SC1003)
  — those are the next pace.
- `rbv_PodmanVM.sh` sits under `vov_veiled/FUTURE/`; confirm it is in scan scope
  intentionally before editing — if FUTURE/ should be excluded, that is a
  shellcheckrc/scan-path question, not a code fix.

## Done looks like
- Every mechanical finding is fixed; only the two survivor candidates remain.

### document-shellcheckrc-survivors-go-green (₢BTAAC) [complete]

**[260527-1141] complete**

## Character
Judgment-light but verify-before-silence. Two suspected false-positives, then green.

## Goal
Resolve the last two findings as documented `busc_shellcheckrc` survivors — the
ones that are likely intentional and want a recorded rationale rather than a code
change — and bring `rbw-tq` green.

## The two candidates
- `jjfp_fundus.sh:482` SC2088 — a `"~/..."` tilde handed to a double-hop ssh
  pipe. Tilde is meant to expand on the REMOTE side, not locally; quoting is
  correct. Verify the remote-expansion intent before silencing.
- `rbfh_FoundryHygiene.sh:85` SC1003 — a literal-backslash `case` pattern.
  Confirm the backslash is genuinely literal/intentional.

## Locked decisions
- Suppression only in `busc_shellcheckrc`, never inline, and only for a TRUE
  false-positive, always with a rationale comment naming why.
- If verification shows either is actually a bug, fix the code instead of
  silencing.

## Done looks like
- Both findings are either fixed or documented `busc_shellcheckrc` survivors with
  rationale.
- `rbw-tq` runs shellcheck and is GREEN — the single routine gate, no
  release-only blind spot. (Closes the heat's done-looks-like.)

**[260527-1032] rough**

## Character
Judgment-light but verify-before-silence. Two suspected false-positives, then green.

## Goal
Resolve the last two findings as documented `busc_shellcheckrc` survivors — the
ones that are likely intentional and want a recorded rationale rather than a code
change — and bring `rbw-tq` green.

## The two candidates
- `jjfp_fundus.sh:482` SC2088 — a `"~/..."` tilde handed to a double-hop ssh
  pipe. Tilde is meant to expand on the REMOTE side, not locally; quoting is
  correct. Verify the remote-expansion intent before silencing.
- `rbfh_FoundryHygiene.sh:85` SC1003 — a literal-backslash `case` pattern.
  Confirm the backslash is genuinely literal/intentional.

## Locked decisions
- Suppression only in `busc_shellcheckrc`, never inline, and only for a TRUE
  false-positive, always with a rationale comment naming why.
- If verification shows either is actually a bug, fix the code instead of
  silencing.

## Done looks like
- Both findings are either fixed or documented `busc_shellcheckrc` survivors with
  rationale.
- `rbw-tq` runs shellcheck and is GREEN — the single routine gate, no
  release-only blind spot. (Closes the heat's done-looks-like.)

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 A close-the-gate-fold-shellcheck-into-rbw-tq
  2 B mechanical-blind-safe-quoting-sweep
  3 C document-shellcheckrc-survivors-go-green

ABC
··x busc_shellcheckrc
·x· jjfp_fundus.sh, rbfc_FoundryCore.sh, rbgp_Payor.sh, rbv_PodmanVM.sh
x·· BCG-BashConsoleGuide.md, buq_cli.sh, buw-qsc.QualifyShellCheck.sh, buwz_zipper.sh, rbq_Qualify.sh

Commit swim lanes (x = commit affiliated with pace):

  1 A close-the-gate-fold-shellcheck-into-rbw-tq
  2 B mechanical-blind-safe-quoting-sweep
  3 C document-shellcheckrc-survivors-go-green

123456789abcd
·······xx····  A  2c
·········xx··  B  2c
···········xx  C  2c
```

## Steeplechase

### 2026-05-27 11:41 - ₢BTAAC - W

Documented the two survivor shellcheck findings as verified site-specific busc_shellcheckrc disables (no code change). SC2088 in jjfp_fundus.sh: the quoted "~/.ssh/authorized_keys" must stay literal through the double-hop ssh pipe and expand on the final remote shell — local expansion would be the curia operator's home. SC1003 in rbfh_FoundryHygiene.sh: the *'\') case glob is a genuinely literal backslash matching FROM line-continuation. Added under a new 'Site-specific false positives (verified, not BCG-structural)' section to keep the rcfile taxonomy honest. shellcheck now clean (181 files, exit 0) and rbw-tq fast-qualify gate is GREEN — closes heat BT's done-looks-like.

### 2026-05-27 11:41 - ₢BTAAC - n

kludge-BTAAC: shellcheckrc site-specific false positives

### 2026-05-27 11:35 - ₢BTAAB - W

Cleared all mechanical shellcheck findings (12 to 2): BCG-canonical quoting (SC2086 in rbfc_FoundryCore x3, SC2295 x4/SC2231 in rbgp_Payor), rm -rf ${var:?} safety guard (SC2115 in jjfp_fundus), and parameter-expansion rewrite of an echo|sed capture-group (SC2001 in FUTURE/rbv_PodmanVM, per operator decision to rewrite). Only the two survivor candidates remain for BTAAC. Noted but left out-of-scope: symmetric /Users rm -rf at jjfp_fundus.sh:129 (unflagged by SC2115).

### 2026-05-27 11:34 - ₢BTAAB - n

Clear mechanical shellcheck findings: BCG-canonical quoting (SC2086 in rbfc_FoundryCore, SC2295/SC2231 in rbgp_Payor), rm -rf :? guard (SC2115 in jjfp_fundus), and parameter-expansion rewrite of echo|sed capture-group (SC2001 in FUTURE/rbv_PodmanVM). Leaves only the two survivor candidates for the next pace.

### 2026-05-27 10:40 - ₢BTAAA - W

Folded shellcheck onto the routine fast-qualify path and retired the standalone buw-qsc.

### 2026-05-27 10:40 - ₢BTAAA - n

Fold BUK shellcheck into rbw-tq fast-qualify gate

### 2026-05-27 10:33 - Heat - f

racing

### 2026-05-27 10:32 - Heat - S

document-shellcheckrc-survivors-go-green

### 2026-05-27 10:32 - Heat - S

mechanical-blind-safe-quoting-sweep

### 2026-05-27 10:32 - Heat - S

close-the-gate-fold-shellcheck-into-rbw-tq

### 2026-05-27 10:31 - Heat - n

Settle the three judgment shellcheck clusters before slating — all real fixes, no suppressions.

### 2026-05-27 08:15 - Heat - d

paddock curried: initial shape at nomination — gate-then-debt, paces deferred

### 2026-05-27 08:14 - Heat - N

rbk-11-mvp-shellcheck-error-gaps

