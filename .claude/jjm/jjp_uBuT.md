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