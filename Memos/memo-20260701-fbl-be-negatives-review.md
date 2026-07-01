# Fable review of ₣Be (rbk-40-fbl-better-negatives) as landed

Model-gated full-heat review (₢BeAAL), performed by Fable 5 on 2026-07-01.
Read-review of all eleven landed paces against the paddock's Done-when;
live suite runs deferred (tree carried another officium's uncommitted
theurge-cosmology work at review time). Line numbers below are as of this
date — dated provenance, expect drift.

## Verdict

The work holds. The negatives system is coherent, doctrine-conformant, and
honestly built. One small Done-when overclaim, a handful of stale-prose
nits, nothing structural. Corrections fold into ₢BoAAB (the admission-band
pace), which touches the same files.

## What holds (verified by reading, not presumed)

- **Band mechanism correct.** `buc_die` (buc_command.sh:103) captures `$?`
  as its first act, propagates in-band only when the bubc tinder is
  present, flattens all else to 1. `buc_reject` (buc_command.sh:120)
  bounds-checks and dies imprecisely on programmer error. Sole-mint rule
  holds: every band code lives in the bubc block (100–108 allocated,
  selftest pin 115), and post-heat minters (clean_tree=108,
  descry/instate) used the block as intended.
- **Survival proof real** (BUK self-test, buts/butcbd_band.sh): six cases —
  in-band re-exit, plain→1, out-of-band launders→1, reject origin,
  reject-refuses-out-of-band, and end-to-end survival of code 115 through
  tabtarget/launcher/dispatch *beneath a command substitution*
  (bux_band_chain, buw-xb). Full-width pin, full laundering gauntlet.
- **Credless genuinely by construction.** The guard env rides
  `rbtdri_tabtarget_command` (rbtdri_invocation.rs:325) — the one
  constructor every tabtarget launch crosses — armed structurally by
  `rbtdrc_set_context` from the fixture's `credless` field, disarmed by
  `take_context`. All 11 reveille members declare `credless: true`
  (count matches the fixture census exactly). Both live mint entries gate
  with `buc_reject` on the credless band: federated avowal
  (rba_auth.sh:408) and Payor OAuth (rbgp_payor.sh:159). The proof case
  (rbtdrf_rs_credless_guard_mint_refusal) is defense-in-depth: junk
  namespace + confirm-after-mint means even a broken guard cannot delete.
- **Poison conversion doctrine-pure.** One membrane
  (zbuv_poison_apply, buv_validation.sh): guarded presence check on the
  optional slot, unguarded tinder reference (typo dies under set -u),
  prefix-scoped so the poison lands exactly once, set/unset semantics,
  credless guard rides inert through it. Zero `probate` survivors
  repo-wide (code, tt, specs).
- **Census conforms to BUS0**: stamp (`buorb_ensconce_stamp`, read at
  rbldb_bole.sh), poison (`buost_regime_poison`), guard
  (`buorb_credless_guard`), plus the BUS0-sanctioned self-test stub
  `buost_example`. Both retired tweaks left no residue; their
  replacements (rbw-lp presage colophon; the feoff/yoke/anoint chain +
  chaining-fact-band fixture) are live.
- **Suite composition compile-checked** (rbtdra_almanac.rs): poison rides
  picket/bivouac/echelon, never reveille; the RBTDRA_REVEILLE_BASE
  set-equality guard pins the base against per-ladder drift.
- **BCG/BUS0 doctrine entries complete** — band semantics, allocation
  rule, enrollment rule, and the rejected stderr-sentinel alternative
  (BCG "Precision Exit-Code Band"); tweak doctrine incl. the standing
  guard reservation (BUS0 "Tweak Mechanism").

## Findings (ranked)

1. **Done-when overclaim, small.** "Every fast-tier negative case asserts
   a specific band code" — the rbw-dU empty-arg refusal case
   (rbtdrf_rs_unmake_empty_arg_refusal, rbtdrf_fast.rs) asserts bare
   nonzero plus output text (the rbw-dl pointer). The refusal originates
   in a `buc_die` usage branch, not a banded gate; banding usage refusals
   would grow the band toward the general taxonomy BCG forbids.
   **Repair: document the carve-out** — one line in BCG's band section:
   usage refusals stay imprecise death by design; their negative cases
   compensate with an operator-discovery pointer assertion.
2. **Stale manifest comment.** rbtdrm_manifest.rs (podvm-resolve entry)
   still describes the pre-eviction shape ("invokes immure colophon …
   expects non-zero exit"); the landed fixture drives rbw-lp presage
   expecting exit 0 with a mapping-line assertion.
3. **Ghost comment.** rba_auth.sh (credless guard at rba_avow): "Mirrors
   the keyfile mint's guard" — the keyfile mint no longer exists; the
   living sibling is the Payor OAuth gate in rbgp_payor.sh.
4. **Pre-rename suite names.** rbtdrs_poison.rs header: "cannot ride
   fast … enrolls in service/crucible/complete" — current names are
   reveille and picket/bivouac/echelon.
5. **CLAUDE.md suite-table drift.** Reveille row lists 10 fixtures,
   missing chaining-fact-band (added by this heat's own pace); the
   composition-owner pointer names `RBTDRC_SUITES`
   (rbtdrc_crucible.rs) but composition lives in `RBTDRA_SUITES`
   (rbtdra_almanac.rs). Picket count also stale (predates the terrier
   fixtures — not this heat's drift).
6. **Nit.** Coronet "BBAA9" cited in a durable code comment
   (rbtdrf_fast.rs, empty-arg case) — same staleness failure mode JJK
   bars from paddock prose; reword purpose-based.
7. **Forward note.** The Rust band-const emit is a hand-kept list
   (rbcc_emit_consts, rbcc_constants.sh) — a new band code does not
   auto-flow; drift fails safe (compile error), but every band mint must
   extend the list by hand.

## Settled design for ₢BoAAB (admission band), from this review

Recorded here so the reslated docket can point rather than restate.

- **Word settled: `admission`** — `BUBC_band_admission=109` (next free
  slot; 100–108 + 115 taken). Polity/civic asterism, consistent with the
  admission verbs (brevet/unseat/attaint); grep-clean as a band name.
- **Capture change is one arm.** rba_don_capture's Leg-3 HTTP `403` case
  arm (the structural admission-deficit Palisade signature, deliberately
  never retried) changes its `return 1` to
  `return "${BUBC_band_admission}"`. The other five failure modes —
  lapsed sitting, jq body compose, curl transport, other HTTP codes,
  empty accessToken — keep `return 1`. The function stays a pure capture
  (return signal, no die/reject, stdout carries only the token).
- **Membrane does the fan-out.** Because `buc_die` propagates in-band
  `$?`, every existing `rba_don_capture … || buc_die` consumer (the four
  governor dons in rbgp_payor.sh, rba_token_capture's delegation)
  propagates admission-denial to the boundary with NO edit. Intended
  consequence: a polity verb wielded without the governor mantle now
  exits 109 instead of 1.
- **Explicit reject only at the probe.** rbgv_check_mantle (rbgv_cli.sh)
  branches on the capture's return: admission band → `buc_reject` with
  an operator-facing deficit message (brevet instruction); any other
  nonzero → the existing `buc_die` unchanged.
- **Fixture shape** (picket tier, rbtdrv_patrol.rs home,
  terrier-atomicity as enrollment template): don retriever (positive) →
  unseat retriever → poll don until it exits the admission band
  (bounded — IAM revocation propagates eventually; assert the exact band
  at terminal state, never weaken) → brevet retriever back → poll don
  until positive (restore proof). Target mantle is retriever, never
  governor (unseating governor saws off the wielding branch). Self-skip
  on payor-unreachable like sibling picket fixtures. Keep the
  don/unseat scaffolding a discrete helper so the parley authentic-verb
  fixture (₣Bl heat) can fold onto it later.
- **Emit list**: extend the band block in rbcc_emit_consts with
  `BUBC_band_admission` so `RBTDGC_BAND_ADMISSION` projects.
