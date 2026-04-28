## Context

Release qualification gap surfaced by the ₣A_ depot-regen heat. Every existing test tier (`rbw-tf` fast, `rbw-tr` release) tolerates accumulated depot state, so silent first-build assumptions slip through. Recent spooks — kludge-aware-charge-prereq (k-prefixed hallmarks at GAR), ZRBOB_PROJECT (compose project name without runtime prefix), rbob_charged_predicate prefix omission, reliquary integrity-broken on negative test — all share a property: they only manifest on first-build paths, and none were caught until live verification surfaced them.

This heat constructs `rbw-tP` QualifyPristine — a third tier that **refuses to run** unless marshal-zero state was just committed. The refusal is enforced by the test itself, not by ceremony or operator discipline.

## Single operator prerequisite chain

Operator has exactly one prerequisite chain: confirm Payor health → marshal-zero → commit → run `rbw-tP`. Payor OAuth is the only credential the operator must have ready; everything else (governor, retriever, director SAs and their RBRA files) is **minted by the qualification itself**, not restored from backup.

## Entry contract — load-bearing

`rbw-tP` fails-fast unless ALL of the following hold:

- Working tree clean
- HEAD commit is a marshal-zero commit (detectable signature baked into `rblm_zero` per BBAAB)
- RBRR fields are blank (prefixes, depot project ID, etc.)
- No RBRA credential files present (governor, retriever, director — all deleted by marshal-zero, recreated by the qualification)
- No hallmark fields populated in nameplates
- No depot-scoped fields populated in vessel rbrv.env files

Operator cannot skip the prerequisite. This is the property that makes the tier catch the silent-first-build bug class by construction.

## Failure mode contract

Mid-qualification failure means start-over-from-zero, not patch-and-continue. Documented explicitly in runbook to prevent the very accumulated-state bug class this tier exists to catch.

## Release-branch execution contract

Three properties bind every fixture and phase in this tier.

**Run on a release branch.** The release machinery is engineered for branch execution, not main. The branch will accumulate many commits across a successful pristine run. After a failed run, the branch holds bygone commits that `rbw-MZ` does not touch — they remain as history; the next run starts from a fresh marshal-zero commit on top.

**Commits during the run are first-class.** Each step that mutates regime state (rbrr.env, rbra files, vessel rbrv.env, etc.) commits the change. Container-recipe work has a documented cognitive failure mode: humans lose track of where they are in long sequences. The growing commit trail is the operator's mental anchor against that.

**Stop on very first failure, cleanly and clearly.** A step fails → fixture stops → operator stops → debugging happens immediately on the failed branch. Stderr names the failed step; fixture exits non-zero. Recovery is `rbw-MZ` + retry on a fresh marshal-zero. No graceful degradation, no recovery branches.

Engineering scaffolding for any other shape — partial-run cleanup, soft-delete tolerance, multi-mode failure handling, recovery-time orphan inspection, diagnostic-redundancy — is non-load-bearing and does not belong in pristine-tier code. The single mechanical path is the entire surface.

## Tier layering

Three tiers, escalating cost:

| Colophon | Frontispiece | Cost | Entry contract |
|----------|--------------|------|----------------|
| `rbw-tf` | QualifyFast | seconds | none |
| `rbw-tr` | QualifyRelease | minutes | none (accumulated state OK) |
| `rbw-tP` | QualifyPristine *(new)* | ~1 hour + cloud $ | marshal-zero just committed |

`rbw-tP` is THE release gate. `rbw-tr` becomes the pre-pristine smoke test — cheap-and-frequent for development confidence; pristine for actual release.

## Gauntlet test suite — design

`rbw-tP` exec's the **gauntlet suite** — a TestSuite (not a custom orchestrator script) composed of fixtures sequenced from marshal-zero state through canonical-credentialed state to crucible verification. The TestSuite mechanism is existing infrastructure (`tt/rbtd-s.TestSuite.{fast,service,crucible,complete}.sh`); the gauntlet suite extends it.

**Suite-of-fixtures, not orchestrator-script.** The original construction plan (custom bash orchestrator with inline tabtarget calls) was abandoned in favor of suite composition. Each section of the release-qualification sequence becomes a fixture; the suite composes them in order.

**Naming hygiene.** "Gauntlet" is the suite name; "pristine" is reserved for the §1 pristine-lifecycle fixture and informally for tier vocabulary. The suite name does not appear at fixture or case level — distinct stratification across naming layers (suite ≠ fixture ≠ case).

**State ladder, fixtures along it.** The suite walks a state ladder no single fixture covers:

1. enrollment-validation (preflight, state-indifferent)
2. pristine-lifecycle (§1: marshal-zero gate + throwaway depot/SA lifecycle)
3. canonical-establish (§2: canonical depot levy + governor mantle + retriever invest + director invest + IAM propagation wait)
4. canonical-onboarding-sequence (§3: reliquary inscribe + per-vessel hallmark ordain — handbook-driven build walks per BBAAf)
5. regime-validation, regime-smoke (post-§3, regimes now populated)
6. four-mode (cloud-build mode coverage)
7. tadmor, moriah, srjcl, pluml (§4: per-crucible test fixtures, vessel-exists preconditions per BBAAs)

No automatic teardown — see "No-teardown decision" below.

## Dual purpose: handbook-regression coverage

The gauntlet's primary purpose is release qualification (catch silent first-build assumptions). Its **secondary purpose** is automated handbook-regression coverage: ensure operator-facing onboarding handbook tracks (`rbh*`) stay in sync with working code so future operators following the prescribed steps aren't surprised by drift.

Build-side handbook coverage lives in BBAAf (canonical-onboarding-sequence): each ordain case walks the corresponding director-mode handbook track (`rbw-Odf` conjure / `rbw-Oda` airgap / `rbw-Odb` bind / `rbw-Odg` graft) rather than calling inscribe/ordain machinery directly.

**Build/run split — load-bearing.** Handbook walks cover the BUILD step (vessel construction) and stop when the hallmark lands in GAR. Run-side fixtures (per-crucible: tadmor/moriah/srjcl/pluml) stay as direct test fixtures with vessel-exists preconditions, NOT handbook walks. The split keeps run-side reusable against any vessel source — coupling them to handbook prose breaks reusability.

Drift between fixtures and handbooks is accepted future maintenance: whoever changes one is responsible for updating the other; the gauntlet's job is to surface drift, not prevent it by construction.

Run-side handbook coverage (`rbw-Ots`, the rack portion of `rbw-Ofc`) is explicitly out of scope for this heat.

## Fixture disposition — Independent vs StateProgressing

Each fixture declares a disposition flag in Rust, controlling engine behavior:

- **Independent** — cases are self-contained; suite-order is informational. Engine permits keep-going mode for surveying.
- **StateProgressing** — case N's success establishes preconditions for case N+1. Engine refuses keep-going mode (incoherent: failed case leaves broken precondition). Fail-fast required.

**Default is Independent.** Existing fixtures (`enrollment-validation`, `regime-validation`, `regime-smoke`, `four-mode`, `tadmor`, `moriah`, `srjcl`, `pluml`) are Independent and require no tagging change. Only fixtures whose case-N success establishes case-N+1 preconditions get StateProgressing — currently `pristine-lifecycle`, `canonical-establish`, and `canonical-onboarding-sequence`.

Suite-level disposition derives from membership: any StateProgressing fixture in the suite makes the suite state-progressing → suite-level keep-going refused.

**Per-case precondition probes.** Each case in a StateProgressing fixture probes its precondition at start. If state matches expectation, run. If not, refuse with operator-actionable diagnostic naming the expected state and the closest fixture that produces it. This enables safe a-la-carte single-case rerun — the engine doesn't need to know the case's history; the case's probe enforces.

## Suite-level fail-fast

Across all suites — not just gauntlet — any fixture failure stops subsequent fixtures from running automatically. State-progressing suites require this for correctness; independent suites adopt it for the simpler mental model. Operators who want full surveying run individual fixtures separately.

## No-teardown decision

The gauntlet suite omits an automatic teardown phase. After green tally, the canonical depot, SAs, hallmarks, and RBRA files persist for operator inspection. Deliberate trade-off:

- **Inspect-after-success** is genuine operator value — browse canonical depot in GCP console, inspect GAR contents, run ad-hoc verification a test sequence cannot anticipate.
- **Cleanup is operator-driven** — manual sequence is `rbw-fA` per vessel + `rbw-arD` + `rbw-adD` + `rbw-dU` (governor cleanup folds into unmake per BBAAN). All existing tabtargets; no new infrastructure.
- **Failed runs don't accumulate cleanup debt** — under fail-fast, teardown wouldn't run anyway; the no-teardown decision only changes behavior on full success.
- **Cost visibility** — pending-delete soft-deleted projects accumulate per run (RELEASE.md accepts this); not changed by teardown decision.

Documentation of the post-success cleanup ceremony is BBAAI work (RELEASE.md).

## Operator scope

Single operator (project lead). Not designed for multi-operator workflow. The README.md release section remains the runbook for the gauntlet itself; the dual-purpose handbook-regression coverage protects the broader operator surface (onboarding tracks under `rbh*`) from drift.

## Coupling to ₣A_

Cutover work from ₣A_ informing `rbw-tP`'s sequence has landed (BBAAM depot-identity-collapse). ₣A_'s remaining paces run independently of BB.

## Locked design decisions — pointers

- **Depot identity collapse** (BBAAM, landed): RBRR collapses to `RBRR_DEPOT_MONIKER`; `RBRR_CLOUD_PREFIX` flows through depot-affiliated resources; per-moniker depot fact-files; pristine-fixture moniker autodetect lives in Rust (`rbtdrp_pristine.rs`), not payor.
- **Cult-verb naming** (BBAAN/BBAAQ/BBAAR, landed): SA domain muster→**roster**; image domain muster→**audit**. Domain-exclusive split; lowercase tail letters preserved for no-cloud-change observation colophons.
- **Build/run split** (BBAAc, landed via paddock pivot): handbook walks cover build only (BBAAf); per-crucible fixtures stay reusable as direct tests (BBAAs).

## Calibrant fixture family — BBAAh

Synthetic Rust fixtures with deterministic verdicts, registered in the manifest by BBAAh. The family exists to give ₣BC `rbk-mvp-3-theurge-self-test` deterministic targets for bash-blackbox driving and manifest-coverage validation; here in ₣BB only the foundation lands.

**Anchor word: `calibrant`.** Module `rbtdrl_calibrant.rs` (classifier `l` for caLibrant since `c` is owned by crucible). Replaces the dormant `rbtdrd_dummy.rs` placeholder. Test counterpart `rbtdtl_calibrant.rs`. Internal-only vocabulary; not end-user-facing.

**Fixture catalog** (Rust, manifest-registered):

- `calibrant-verdicts` (Independent) — pass / fail / skip / pass_with_output. Verdict-path coverage. The case-written file (`output.txt`) is distinct from the engine-auto `trace.txt`.
- `calibrant-fail-fast` (Independent, multi-section) — fixture-internal fail-fast across cases and across sections; "not_reached" cases write sentinel files used as absence-of-side-effect assertions.
- `calibrant-progressing` (StateProgressing) — exercises `rbtdrb_Probe` diagnostic format and `rbtdre_resolve_fail_fast` policy gate.
- `calibrant-sentinel` (Independent) — single case writing a known file when it runs. Used for suite-level fail-fast verification.

Suite: `calibrant` = [verdicts, fail-fast, sentinel]. Sentinel-after-fail verifies cross-fixture fail-fast contract.

Downstream consumption — bash testbench, manifest-coverage runtime checks, coverage-validation fixtures and driver — lives in ₣BC.

## References

- ₣A_ rbk-mvp-3-resource-prefix-and-depot-regen — surfaced the gap
- ₣BC rbk-mvp-3-theurge-self-test — calibrant downstream consumers
- `tt/rbw-tr.QualifyRelease.sh` — current release qualify; layered alongside, not replaced
- `Tools/rbk/rblm_cli.sh` — `rbw-MZ` zeroes local regime; marshal-zero signature baked here per BBAAB
- `.claude/commands/rbk-prep-release.md` — upstream contribution ceremony; pristine-pass becomes a precondition (BBAAH)