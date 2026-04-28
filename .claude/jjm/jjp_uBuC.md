## Context

Split from ₣BB rbk-mvp-3-release-qualification to decouple theurge framework self-test infrastructure from the gauntlet release ladder. ₣BB stays focused on canonical-establish → onboarding-sequence → suite-assembly → first-run; this heat owns framework-polish work that doesn't qualify mvp-3 but tightens the framework that runs it. Both heats can race independently once the calibrant fixture foundation lands in ₣BB.

## Cross-heat coupling

Mounts after ₣BB's BBAAh (calibrant fixture foundation) commits. The four fixtures BBAAh registers — `calibrant-verdicts`, `calibrant-fail-fast`, `calibrant-progressing`, `calibrant-sentinel` — are this heat's blackbox subjects. ₣BB's paddock § "Bash blackbox calibrant — design" and the BBAAh commit carry the fixture catalog, anchor-word rationale, and module-rename decisions; mount-time agents read those for fixture-contract detail rather than restating here.

## Origin gaps — two design pillars

**Operator-surface validation gap (BCAAA).** Rust unit tests verify engine functions in isolation; nothing verifies the operator-facing surface of the `rbtd` binary — CLI exit codes, stderr diagnostic format, fixture and suite fail-fast, the disposition × keep-going policy gate landed in BBAAd. The calibrant family provides synthetic fixtures with deterministic verdicts; this heat drives them through the binary as a black box.

**Manifest-coverage drift gap (BCAAB → BCAAC → BCAAD).** rbtdrm's per-fixture `required_colophons` list is asserted-superset — declarations may carry stale entries (declared but never invoked) or missing entries (invoked but not declared) without surfacing. Two runtime checks at the `rbtdri_invoke_*` chokepoint tighten the contract: positive (invoked must be declared) refuses unknown invocations; negative (declared must be invoked) fails on dead declarations after a successful full-fixture run. Coverage-validation fixtures (BCAAC) and a bash driver (BCAAD) pin diagnostic shape.

## Bash case catalog — operator-surface validation (BCAAA)

`Tools/rbk/rbtt_testbench.sh`, 14 cases across 6 sections:

- `verdict-propagation` (4) — pass / fail / skip exit codes + trace file
- `fixture-fail-fast` (2) — intra-section, inter-section
- `disposition-policy` (3) — Independent + keep-going runs all; StateProgressing + keep-going refused with policy stderr; StateProgressing default runs fail-fast
- `probe-diagnostics` (1) — unmet-probe stderr contains `precondition '%s' not met:` + `remediation:`
- `suite-fail-fast` (2) — suite aborts on failing fixture; subsequent fixture's sentinel absent
- `cli-surface` (2) — unknown fixture errors clearly; missing arg → usage

## Coverage-validation case catalog (BCAAC + BCAAD)

Three coverage-fixtures (Independent disposition) registered alongside the BBAAh family, exercising both check directions of the manifest-coverage gate:

- `calibrant-coverage-aligned` — manifest declares noop, case invokes noop → Pass
- `calibrant-coverage-undeclared` — manifest declares nothing, case invokes noop → fixture-level FAIL via positive check
- `calibrant-coverage-unused` — manifest declares noop, case invokes nothing → fixture-level FAIL via negative check

Bash driver (BCAAD) — section `manifest-coverage`, 4 cases: `coverage-aligned-passes`, `coverage-undeclared-fails`, `coverage-unused-fails`, `coverage-single-case-skips-negative` (proves the `run_single` exemption from the negative check).

## Locked design decisions

- **Anchor word `calibrant`**, module `rbtdrl_calibrant.rs` (classifier `l` per RCG terminal exclusivity — `c` is owned by crucible). Set in BBAAh; this heat consumes it.
- **With-the-grain composition** — tabtargets are bare exec stubs; orchestration lives in zippers / workbenches / testbenches / suites. BUK's self-test stays untouched. No "BUK + calibrant" composite tabtarget — operator sequences `buw-st` then the new colophon.
- **No per-fixture tabtargets for calibrant family** — framework-test plumbing, not operator workflows. The bash testbench invokes `rbtd` directly; deviates from BBAAe's per-fixture pattern, justified because pristine / canonical are operator-facing and calibrant isn't.
- **BURV chain is contract, not hook** — `BURV_TEMP_ROOT_DIR` / `BURV_OUTPUT_ROOT_DIR` (BUS0 §540-552) is load-bearing infrastructure. Calibrant cases nest BURV through `rbtd` invocations and incidentally regression-test the chain.
- **Manifest-coverage check enforcement (BCAAB):**
  - Positive at all three `rbtdri_invoke_*` primitives — refuses undeclared invocations
  - Negative in `run_suite` after `run_sections` returns success — gated on `result.failed == 0` (failure paths suppress)
  - `run_single` enforces positive only — cannot satisfy exhaustiveness by construction

## Out of scope

- Cross-kit testbench composition (BUK fixtures registered in rbk testbench)
- Orchestrating tabtargets containing exec-of-children logic
- Trace-file format invariants beyond a single `pass_with_output` smoke check
- Color rendering / terminal width contracts
- Imprint coverage (which nameplate a per-imprint colophon was called with), argument patterns, compile-time / static analysis
- Fixture-level declaration edits in existing fixtures (canonical-establish, etc.) — surface as separate finds if BCAAB's negative check turns up stale entries during ₣BB's gauntlet run

## References

- ₣BB paddock § "Bash blackbox calibrant — design" — fixture-family origin, anchor-word rationale, sub-pace catalog
- ₣BB BBAAh — calibrant fixture foundation (prerequisite)
- BBAAd — engine concepts (`rbtdre_Disposition`, `rbtdre_resolve_fail_fast`, `rbtdrb_Probe`)
- `rbtdri_invoke`, `rbtdri_invoke_global`, `rbtdri_invoke_imprint` — chokepoint primitives BCAAB modifies
- `Tools/rbk/rbts/` — case-file convention (BCAAA establishes; BCAAD extends)
- BUS0 §540-552 — BURV chain contract