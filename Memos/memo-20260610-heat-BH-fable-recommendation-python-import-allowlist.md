# Fable recommendation — python cloud steps need an import allowlist (and a subprocess policy)

Date: 2026-06-10
Status: recommendation, grounded in code (second Fable review pass); not yet acted on. Operator-prompted:
"should we create an equivalent of the bash command allowlist for python, anchored on imports?"

## Verdict: yes — and the gap is already carrying live traffic

The cupel's supply-chain conformance walks `*.sh` only (`zrbtdru_walk_sh`,
`ZRBTDRU_UNIVERSE_FILE_EXT` in rbtdru_cupel.rs). Four python cloud steps exist today —
`rbgja01-discover-platforms.py`, `rbgja03-build-info-per-platform.py`,
`rbgjl06-package-delete.py`, `rbgjv02-verify-provenance.py` — and sit entirely outside the
conformance surface. A `.py` step sidesteps the bash-command discipline by construction.

The live specimen proving the hole is not hypothetical: **rbgjv02 `subprocess.run`s `gcloud`** —
a command absent from `ZRBTDRU_GCB_ALLOWED` — invisibly to the cupel. The invocation itself may be
legitimate (the SLSA provenance `describe`), but it is *unreviewed by the mechanism*, which is the
whole point of having one. CBG already names the rule this would mechanize (CBi_104 "code to the
builder image's pinned runtime") and already has the `CBp_` python family with a known-gap marker
(CBp_101) — the import allowlist is the missing citer.

## Recommended shape

Extend the cupel with a `*.py` walk over the step directories and two checks:

1. **Import allowlist, anchored on the module root** of every `import X` / `from X import Y`.
   Current empirical floor (the union of all four steps): `base64, datetime, io, json, os, sys,
   tarfile, time, urllib`. Everything else flagged. `importlib`, `__import__`, `exec`, `eval`
   banned outright — dynamic import defeats static conformance. Third-party imports (`requests`
   et al.) are the real quarry: they bind a step to the floating builder's unpinned pip set, the
   exact drift class the bash allowlist exists to stop. Stdlib-only is the right floor.

2. **Subprocess policy — the bridge between the two universes.** `subprocess` lets a python step
   shell out past both disciplines. Either ban it in steps (python steps exist for in-process
   JSON/HTTP work; shelling out belongs in bash steps), or — since rbgjv02 uses it today — scan
   `subprocess.run([...])` argv[0] literals against the same `ZRBTDRU_GCB_ALLOWED` list: one tool
   floor, two languages. Under that option, `gcloud` needs explicit adjudication (add it to the
   allowlist, or convert rbgjv02's describe call to REST urllib like every other step).

## Interaction with the standing builder-pin memo

memo-20260610-heat-BH-fable-recommendation-pin-delete-builder flags the floating builder under a
privileged identity; this memo is its step-body complement. Floating builder × unscanned imports
compound: the builder decides what `import requests` resolves to. Pinning bounds the supply;
the allowlist bounds the demand.

## Addenda from the 2026-06-10 CBG exchange (operator review of these recommendations)

- **Where the authoritative tool floor lives (surveyed):** the doc comment on
  `ZRBTDRU_GCB_ALLOWED` in `rbtdru_cupel.rs` is the single clearest statement — it carries the
  membership criterion (empirical per-container presence in the controlled builders, NOT kit
  portability), the jq-exclusion rationale, and the conformance purpose. No spec or guide copy
  exists, and none should be created: CBG should carry a **pointer** to that constant as the floor's
  home ("reference the home, don't recreate" — ACG), never a restated list. CBG's CBi_105/CBp_102
  currently reference the concept; the pointer-line addition awaits operator placement choice.
- **Why the unsanctioned `gcloud` never failed an audit (operator question):** vacuous pass — the
  cupel never opens `.py` files, so rbgjv02 was never scanned at all. The audit did not tolerate the
  violation; it never saw it. No additional mechanism is implicated beyond the gap this memo records.
- **Probe before building the python walk:** check whether `zrbfc_expand_includes`
  (rbfcb_BuildHost.sh) is already language-blind — the `#@rbgjs_include` marker is a `#` comment,
  valid python syntax, so a shared python preamble snippet (`die`/`require_env`/`metadata_token`/
  `gar_fetch`/`gar_json`) might splice with zero expander work, resolving CBG CBp_101 (the
  preamble-duplication gap) nearly for free. Ten-minute probe; do it when planning the walk pace —
  the shebang-strip and any bash-shaped assumptions in the expander are the things to check.
- **buildStepOutputs cap is API contract, not folklore:** the Build API's `results.buildStepOutputs`
  stores "only the first 4KB" of `$BUILDER_OUTPUT/output`, ordered by build step index
  (https://cloud.google.com/build/docs/build-config-file-schema; the cloudbuild v1 API model docs
  state the 4KB retention on BuildStep outputs). Now cited in CBG CBh_103. Relevant ceiling for the
  podvm native-family 8-leaf envelope.
