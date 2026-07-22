# Memo: Wrap-convergence redesign + kit-split intent (hand-merge era plan)

**Date:** 2026-07-22
**Author:** Claude Fable 5 with operator (design session "alpha-blocked"), sequel to
`memo-20260722-billet-divergence-studbook-repair-blocker.md`
**Status:** Intent capture destined for the footprint-delivery heat's seed.
Provenance, never authority. Retires when its rulings are transcribed into that
heat and the spec homes below take them.

## Why this memo

The billet-divergence blocker (sibling memo above) exposed the missing
convergence beat in the billet lifecycle: wrapped work stays on billet branches,
nothing carries it to trunk, and later billets mount stale. The operator ruled
the repair shape in the 260722 design session this memo records. Until the
machinery lands, pace convergence is a HAND process (final section).

## Operator rulings (260722 design session)

1. **Wrap staleness gate — INTERDICTUM, no automatics.** At wrap entry, if
   trunk has advanced beyond what has been enfolded into the billet, wrap
   refuses (interdictum grammar: bars the act, names refit as the remedy) and
   stops. No retries, no auto-refit. After the operator-directed refit, wrap
   re-attempts; the gate guarantees the subsequent converge is trivial by
   construction.
2. **Squash-converge to the sire.** Post-gate, wrap composes one commit whose
   tree is the billet tree with the sire's trunk tip as sole parent (the
   `commit-tree` gesture — the blotter proffer's proven in-house pattern) and
   pushes it to the sire's trunk. No checkout anywhere; the hippodrome working
   tree is never touched. The billet branch keeps its interior history
   (branches are durable); the studbook journal keeps the step log. This
   preserves JJSVD's "the branch's interior history dies at the operator's
   trunk merge" verbatim.
3. **Notch consigns the billet branch every time.** The chapbook Act II beat
   (commit the list, take the counterfoil, push the branch) is designed but
   unbuilt in the notch path (`jjrnc_notch.rs` commits locally only;
   `jjrfr_consign` is called only by refit and muck salvage). Wire it.
   Rationale: backup posture — no work stranded in a billet — and rung 2's
   REMOTE-AWARE SEAT seam depends on branches being pushed.
4. **Hippodrome = operator freehold, lazy follower.** Machinery treats trunk
   as a ref, never a tree. Billets are born from the sire's fetched tip (glean
   at saddle; the no-exfiltration posture), so the operator's local pull
   cadence is free — correctness never waits on it.
5. **Concurrency cost accepted.** Under squash, a sibling billet never carries
   the new trunk commit in its ancestry, so each concurrently-open billet trips
   the wrap gate once per sibling wrap and pays one attended refit. Named here
   so it is read as design, not defect. The dominant one-pace-at-a-time case
   never pays it.
6. **Ordering: wrap machinery first, then the kit split.** Getting wrap solid
   ends the hand-merge era fastest. The extraction of Job Jockey from
   recipemuster into its own repo (`jjqa_app`, already the cosmology's named
   infield peer) follows — it is the final de-insertion under the
   engagement-inversion cinch, and it unties the self-hosting knot the blocker
   memo demonstrated (engine source riding the very billet branches the engine
   manages). Both belong to the footprint-delivery heat cut by ₣B3's terminal
   carry-forward pace; this memo enters that heat's seed as a live finding.

## Spec homes owed (when the machinery is built)

- **JJSRWP-wrap.adoc rewrite** (it still speaks pre-studbook dialect: chalk
  markers, `refs/vvg/locks/vvx`) — the gate + converge behavioral sequence.
- **JJSVD-dispatch.adoc** — revise "trunk belongs to the operator alone" to
  "trunk advances only through the wrap ceremony's converge, under the
  operator's wrap confirmation"; reconcile refit prose with the wrap-entry
  gate.
- **JJSAC chapbook Act III** — the converge beat; the deferred stale-trunk
  resync sibling tale is largely obviated by the gate.
- **Muck reap condition** — tighten with "converged to trunk". Note: ancestry
  checks cannot see a squash, so the wrap's studbook journal entry must record
  the trunk commit it produced (a second counterfoil).

## Hand-merge era (until the wrap machinery lands)

Each pace wrap is followed by an operator (or operator-directed) plain merge of
the billet branch into main in the hippodrome, then push to origin. Merge,
never rebase (the ₣B3 paddock's collision specimen is the standing proof).

Executed this session: `CAAAU` (paddock-tenancy repair + normative hardening +
studbook-droppings sweep) and `CAAAy` (blocker memo) merged to main and pushed;
vvx rebuilt and installed from converged main. Remaining in ₣B3 — ₢B3·CAAAy
(drain-jjsas-cutover-section) and ₢B3·B3AAA (cut-the-footprint-heat) — each
wraps by hand the same way.
