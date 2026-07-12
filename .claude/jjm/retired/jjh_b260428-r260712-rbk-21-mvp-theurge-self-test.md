# Heat Trophy: rbk-21-mvp-theurge-self-test

**Firemark:** ₣BC
**Created:** 260428
**Retired:** 260712
**Status:** retired

## Paddock

## Context

Framework self-certification for the theurge: the rbtd binary that runs every RB test fixture has unit-tested internals but no test of its own operator surface — exit codes, stderr diagnostics, fail-fast behavior.
Split from the retired mvp-3 release-qualification heat (₣BB) so framework polish never blocks the release ladder; the calibrant fixture family that heat landed (`rbtdrl_calibrant.rs`) is this heat's test subject.
Regroomed 260711 against the post-split landscape; the decisions below are cinched.

## Two pillars

**Surface certification.**
An in-crate surface fixture — a green, credless reveille member — spawns child rbtd runs through the real tabtarget chain against the deliberately-failing calibrant fixtures and asserts the child's exit code, stderr shape, and sentinel/trace files.
The watcher passes; the watched stay roster-only.
This replaces the original bash-testbench architecture: no new testbench module, no new colophon, no runbook entry; RBSTC's ratification note about re-minting the testbench family at this heat's mount is moot.
Self-hosting is accepted: the child is observed from outside (exit codes, files), and a child run traverses the full sandwich — tabtarget, launcher, workbench, rbte_engine, binary — wider coverage than a bash driver calling the binary directly.

**Census enforcement.**
Each fixture's `rbtdrm_required_colophons` list is actively maintained but consumed by nothing: the runtime existence check (`rbtdrm_verify`) was deliberately retired in favor of compile-time const projection, and usage alignment cannot be compile-checked — you only learn what a fixture invokes by running it.
Enforce both directions at the invocation chokepoint and report per-colophon usage.

## Cinched (260711 groom)

- Enforce the census, both directions; a declared-but-never-invoked colophon FAILS the fixture on a fully-green full-fixture run.
  This deliberately overrides the tariff precedent (`count_drift` warns, never affects verdict): census drift is a defect, not a curiosity.
- The surface fixture is a reveille member; the calibrant fixtures themselves stay out of every dependency-tier and release suite (deliberate failers).
- One small calibrant suite is registered in `RBTDRA_SUITES` solely as the suite-abort test subject, driven only by the surface fixture.
  The prior lock against it ("bash `set -e` provides suite fail-fast, no Rust suite needed") dissolved when suite composition moved into the binary — `rbte_suite` is now a passthrough.
- Keep-going is plumbed through the tabtarget-to-binary chain so the `rbtdre_resolve_fail_fast` policy — including the StateProgressing refusal — is reachable from outside; today no CLI flag exists and the gate is unit-test-only.
- Anchor word `calibrant`, module `rbtdrl_calibrant.rs` — consumed from ₣BB, unchanged.
- BURV chain (BUS0 §540-552) is contract, not hook: child runs nest BURV through real invocations and incidentally regression-test it.
- No per-fixture tabtargets for the calibrant family.

## Surface-fixture case catalog

Sections name behavior, not case counts (counts are mount-time):

- verdict-propagation — pass / fail / skip child exit codes + trace file presence
- fixture-fail-fast — intra-section and inter-section halt
- disposition-policy — Independent + keep-going runs all cases; StateProgressing + keep-going refused with policy stderr; StateProgressing default runs fail-fast
- probe-diagnostics — unmet-probe stderr carries the precondition and remediation lines
- suite-abort — a failing fixture halts the registered calibrant suite; the later fixture's sentinel absent
- cli-surface — unknown fixture errors clearly; missing arg yields usage
- coverage — aligned passes; undeclared fails naming the colophon; unused fails naming the colophon; single-case exempt from the negative check

## Win-series role

Theurge is the substrate-sensitive piece of the stack; the Cygwin failures bit exactly at the tabtarget-invocation boundary (`Memos/memo-20260517-windows-substrate-landscape-for-theurge.md`).
A reveille run on a Windows substrate therefore certifies that boundary on every pass — the surface fixture is the parity instrument.

## Out of scope

- Trace-file format invariants beyond a single smoke check; color / terminal-width contracts.
- Imprint coverage (which nameplate a per-imprint colophon was called with), argument patterns, compile-time/static analysis of usage.
- Fixture-level declaration edits in existing fixtures — a real stale entry surfaced by the negative check is a separate find.
- Deleting or reshaping the tariff mechanism.

## References

- `Tools/rbk/rbtd/src/rbtdrl_calibrant.rs` — calibrant family (test subjects)
- `Tools/rbk/rbtd/src/rbtdri_invocation.rs` — invoke primitives (the chokepoint)
- `Tools/rbk/rbtd/src/rbtdre_engine.rs` — `rbtdre_resolve_fail_fast`, tariff machinery
- `Tools/rbk/rbtd/src/rbtdra_almanac.rs` — `RBTDRA_SUITES` registry, roster-only doctrine
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — `rbtdrm_required_colophons`
- `Tools/rbk/vov_veiled/RBSTC-theurge_cosmology.adoc` — cosmology; ratification note on the now-moot testbench re-mint

## Paces

### calibrant-surface-fixture (₢BCAAA) [complete]

**[260711-1659] complete**

## Character

In-crate surface fixture: theurge certifying its own operator surface.
A green, credless reveille member whose cases spawn child rbtd runs through the real tabtarget chain (rbtdri invocation of the theurge `rbw-t*` tabtargets) against the deliberately-failing calibrant fixtures,
asserting the child's exit code, its diagnostic shape on the operator-visible stream, and sentinel/trace files.
The watcher passes; the watched stay roster-only.
Self-hosting accepted per paddock: the child is observed from outside, and a child run traverses tabtarget, launcher, workbench, rbte_engine, and binary.

## Cinched

- Assertion surface: under the logged dispatch chain, `bud_dispatch` folds the coordinator's stderr into stdout before the logging loop,
  so a child's rbtd diagnostics arrive on the parent's captured stdout — `rbtdri_InvokeResult.stderr` carries only pre-coordinator chain failures.
  Assert diagnostic shape on captured stdout (BURE_COLOR resolves 0 through a pipe, so it is deterministic and colorless).
  Where a case certifies stream placement itself (diagnostics land on stderr, not stdout), use the established `BURD_NO_LOG` extra_env pattern (see writ in `rbtdrc_crucible.rs`), accepting that mode's station-load skip for that case only.
- Child log isolation rides `BURV_LOG_DIR` (landed in the launcher spine by ₢BrAAL, cross-heat): every child invoke passes it via extra_env, pointed into the case's dir and POSIX-rendered like the sibling BURV roots,
  so deliberately-failing child runs never truncate the station's shared `last.txt` mid-suite nor pollute the logs-buk hist census `rbw-td` dowses.
  Narrow by choice: this fixture only — `rbtdri_invoke_impl` unchanged; uniform application is banked for the spine's per-billet log-dir arc (see ₢BrAAL's wrap).
  The child's self-logs landing in the override dir double as the regression guard that wrap noted was missing.

## Scope

- New surface fixture in the rbtd crate, registered as a reveille member in `RBTDRA_SUITES`; its manifest entry declares the colophons it drives (a census subject like any other fixture).
- Case sections per the paddock catalog: verdict-propagation, fixture-fail-fast, disposition-policy, probe-diagnostics, suite-abort, cli-surface.
  (The coverage section lands in its own pace.)
- Plumb keep-going: CLI surface on rbtd plus pass-through in the `rbte_engine.sh` fixture and suite runners, honoring `rbtdre_resolve_fail_fast` so the StateProgressing refusal is reachable from outside.
- Register the calibrant suite (a failing calibrant fixture ordered before calibrant-sentinel) in `RBTDRA_SUITES`; amend the roster-only comments where they name calibrant.
- Fixture name, module placement, and flag spelling minted at mount per the minting workflow.
- Self-hosting guard: a child run re-enters `zrbte_build_binary` (write-on-change codegen + cargo) while the parent binary executes; on a clean freshly-built tree that is a no-op — verify it stays one, since a relink over the running binary fails on Windows.

## Out of scope

- No bash testbench, no new colophon, no tabtarget stub, no runbook entry — superseded architecture, see paddock.
- Uniform `BURV_LOG_DIR` on every rbtdri invoke, and any change to what the dowse census reads.
- Census-enforcement assertions (sibling paces).
- Trace-file format beyond one smoke check; color/terminal-width contracts.

## Done when

Reveille runs green with the surface fixture as a member; each catalog behavior is asserted against a child run;
keep-going is drivable from a tabtarget and refused for StateProgressing; suite-abort is proven via the registered calibrant suite;
child runs leave the station's logs-buk untouched, proven by asserting the child's self-logs appear in the override dir.

### theurge-manifest-coverage-check (₢BCAAB) [complete]

**[260712-0653] complete**

## Character

Tighten the per-fixture required-colophons census from maintained-but-unenforced to enforced-both-directions at the invocation chokepoint, with per-colophon usage reported.
Positive: an invoked colophon absent from the fixture's declared list refuses the invocation.
Negative: on a fully-green full-fixture run, a declared colophon never invoked FAILS the fixture (operator ruling, 260711 groom).
This deliberately overrides the tariff precedent (`count_drift` warns, never affects verdict) — census drift is a defect.
Not superseded by the compile-time const projection that retired `rbtdrm_verify`: existence is compile-checked, usage alignment cannot be.

## Scope

- Track declared and used colophon sets in `rbtdri_Context`; enforce in the shared invoke implementation so every primitive (`rbtdri_invoke`, `_env`, `_global`, `_imprint`) is covered.
- Negative check plus per-colophon usage report at fixture completion (`rbtdre_run_fixture`), gated on zero failed cases — failure paths suppress the check.
- Single-case runs (`rbtdre_run_single_case`) enforce positive only — one case cannot satisfy exhaustiveness by construction.
- Operator-facing diagnostics name the fixture and the offending colophon.
- Unit tests covering both directions and the single-case exemption.
- Turn-on audit gate: existing TestSuite runs.
  A real stale declaration surfaced by the negative check is reported as a separate find, never silently edited here.

## Out of scope

- Imprint coverage, argument patterns, compile-time/static analysis.
- Deleting or reshaping the tariff mechanism.

## Done when

Enforcement live on every invoke path; unit tests prove refuse / fail / exempt; existing suites pass clean or surface real declaration drift as named finds.

### calibrant-coverage-foundation (₢BCAAC) [complete]

**[260712-0721] complete**

## Character

Rust extension to the calibrant family proving the census enforcement lands its diagnostics: three fixtures with deliberately aligned or mis-declared manifests, plus one synthetic noop tabtarget they invoke without cloud or filesystem side effects.
Roster-only deliberate failers — driven by the surface fixture's coverage cases, never by any suite.
Self-contained; the surface-side assertions land in the sibling pace.

## Scope

- Synthetic noop tabtarget, theurge-internal; colophon minted at mount within the current tabtarget universe (the old `rbtd-*` family is retired — the theurge tabtargets are `rbw-t*` now).
- Three fixtures in the calibrant family (Independent disposition):
  coverage-aligned — manifest declares noop, case invokes noop, Pass;
  coverage-undeclared — manifest declares nothing, case invokes noop, fixture-level FAIL via the positive check;
  coverage-unused — manifest declares noop, case invokes nothing, fixture-level FAIL via the negative check.
- Manifest entries and disposition tags; Rust unit tests pin registration shape only.

## Depends on

The census-enforcement pace — without the runtime checks landed, the failing fixtures pass instead of fail.

## Out of scope

- Exit-code or stderr assertions (sibling surface pace).
- Suite membership for these fixtures.

### coverage-surface-cases (₢BCAAD) [complete]

**[260712-0732] complete**

## Character

Coverage cases added to the surface fixture: black-box assertions that the census enforcement's failures look right from outside — child exit codes and stderr naming the offending colophon — plus the single-case exemption.
Rust cases in the surface fixture, not bash (superseded architecture, see paddock).

## Scope

- The coverage section of the surface fixture, four cases:
  aligned child run exits 0;
  undeclared child run exits non-zero with stderr naming the colophon (positive check);
  unused child run exits non-zero with stderr naming the colophon (negative check);
  single-case child invocation of the unused fixture's case exits 0 (exemption proof).
- Census declarations for whatever child tabtargets these cases drive.

## Depends on

The surface fixture (case home), the coverage fixtures (test subjects), and transitively the census enforcement.

## Out of scope

- New harness infrastructure.
- Trace-file contracts beyond exit code plus stderr substring.

### census-permitted-tier (₢BCAAE) [complete]

**[260712-0752] complete**

## Character

Split the colophon census declaration into required and permitted tiers so conditional-by-design invocations stop failing healthy runs.
Required keeps both directions exactly as landed; permitted is positive-only — admitted at the invoke funnel, never demanded by the negative check.
Fixes the census turn-on's one known defect: foedus-reuse (affiance fires only on a descry deficit) and freehold-establish (reuse-or-levy) census-fail their healthy paths today.

## Cinched (260712 attribution sweep)

- Two tiers in the manifest: required (unchanged semantics) plus a permitted set defaulting to empty.
- Positive check admits required ∪ permitted; negative check demands required only.
- Census report prints permitted usage as advisory lines, never verdict-affecting.
- Membership moves: foedus-reuse AFFIANCE_MANOR and freehold-establish LEVY_DEPOT move from required to permitted; nothing else moves.
- "Permitted" is hearting-level naming (interior Rust), not a minted ashlar.

## Scope

- Manifest two-tier declaration; the census arm state carries both sets; the rbtdrc context arm passes both.
- Unit tests: permitted-invoked does not refuse; permitted-unused does not fail; required semantics unchanged.
- Gate: unit tests + reveille.
  The two moved fixtures need live credentials — their verification rides the operator's next standalone run, never this pace.

## Done when

Both tiers enforced as cinched; unit tests prove the permitted semantics; reveille green.

### census-declare-unmanifested (₢BCAAF) [complete]

**[260712-0826] complete**

## Character

Extend the colophon census to the four fixtures with no manifest entry — regime-poison, clipboard, chaining-fact-band, chaining-fact-livery — so coverage is total across the roster.
Depends on the permitted tier from the preceding pace: conditional launches land there.

## Cinched

- clipboard declares empty — its cases launch no tabtargets.
- chaining-fact-livery's banish launches are banish-if-present cleanup: permitted; its ensconce/divine/feoff arc: required.
- Tier rule everywhere: launched on every green path → required; conditional → permitted.

## Scope

- Attribution per fixture: read the case bodies (rbtdrh_chain.rs; rbtdrs_poison.rs; livery in rbtdrv_patrol.rs; clipboard in rbtdrf_fast.rs) for tabtarget launches, funnel and direct-Command alike.
- Converge the runnable ones by running them and reading the census usage report: chaining-fact-band standalone or via reveille; regime-poison via tt/rbw-tf.FixtureRun.sh regime-poison (its operator-local cases may self-skip — skips suppress the negative check, so a partial-skip green is a valid gate).
- chaining-fact-livery cannot run without live GAR: attribute statically from the case source; its next service-tier run is the live proof.
- The enforcement machinery is landed — this pace writes declarations, not mechanism.

## Done when

All four fixtures carry manifest entries; reveille green; regime-poison standalone green with its census report showing the declared set used, skips permitting.

## Commit Activity

```

File touches (file: the paces whose commits touched it):

  A  ₢BCAAA  calibrant-surface-fixture
  B  ₢BCAAB  theurge-manifest-coverage-check
  C  ₢BCAAC  calibrant-coverage-foundation
  D  ₢BCAAD  coverage-surface-cases
  E  ₢BCAAE  census-permitted-tier
  F  ₢BCAAF  census-declare-unmanifested

  rbtdrm_manifest.rs               A B C E F
  rbtdre_engine.rs                 A B E
  rbtdri_invocation.rs             A B E
  rbtdte_engine.rs                 A B E
  main.rs                          A B
  rbtdra_almanac.rs                A C
  rbtdrc_crucible.rs               B E
  rbtdrj_touchstone.rs             A D
  rbtdrl_calibrant.rs              A C
  rbtdti_invocation.rs             B E
  rbte_engine.sh                   A C
  claude-rbk-tabtarget-context.md  C
  lib.rs                           A
  rbtdgc_consts.rs                 C
  rbtdtc_crucible.rs               A
  rbtdtl_calibrant.rs              C
  rbtdtm_manifest.rs               E
  rbw-tn.Nihil.sh                  C
  rbw-tn.Noop.sh                   C
  rbw-ts.TestSuite.calibrant.sh    A
  rbz_zipper.sh                    C

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 45 commits)

  1 A calibrant-surface-fixture
  2 B theurge-manifest-coverage-check
  3 C calibrant-coverage-foundation
  4 D coverage-surface-cases
  5 E census-permitted-tier
  6 F census-declare-unmanifested

123456789abcdefghijklmnopqrstuvwxyz
········xxxxx······················  A  5c
·············xxxx··················  B  4c
·····················xxxx··········  C  4c
·························xxxx······  D  4c
·····························xxx···  E  3c
································xxx  F  3c
```

## Steeplechase

### 2026-07-12 08:26 - ₢BCAAF - W

Colophon census now covers the whole fixture roster: the four unmanifested fixtures carry declarations. clipboard declares empty (both cases shell to bash/arboard directly, never through rbtdri_tabtarget_command) — proven by running it: 2 passed, invocations=0, census armed and silent. chaining-fact-band declares 8 required, every one empirically used on the reveille run. chaining-fact-livery declares divine/ensconce/feoff required and banish permitted (banish-if-present, per the docket cinch); attributed statically since it needs live GAR, but the declaration is complete — no fifth colophon hides in the body, so the positive check cannot refuse on its next service-tier run. regime-poison declares all 8 validate colophons required, zero permitted.

### 2026-07-12 08:24 - ₢BCAAF - n

Review repair: promote regime-poison's two operator-local colophons (rbw-rov, buw-rsv) from permitted to required. The permitted tier means a colophon may legitimately go UNUSED on a healthy run, and neither of these ever can: the census records at the launch (rbtdri_tabtarget_command), and rbtdrs_poison_optional launches the un-poisoned baseline probe unconditionally, before the skip decision — the self-skip elides only the POISONED invocation, never the launch the census counts. Permitted therefore exempted both from the negative check while buying nothing, so a deleted rbro/burs case would have drifted silently. Required cannot false-fail on an unconfigured station either: the case skips there, and a skipped case suppresses the negative check outright. regime-poison now declares eight required, zero permitted.

### 2026-07-12 08:16 - ₢BCAAF - n

Declare colophon-census manifest entries for the four fixtures with no prior entry: clipboard (empty, no colophons launched), chaining-fact-band (8 required colophons via the per-verb and readside funnels), chaining-fact-livery (divine/ensconce/feoff required, banish permitted as banish-if-present cleanup), and regime-poison (six in-tree-backed regimes required, oauth/station-tincture permitted as operator-local self-skips).

### 2026-07-12 07:52 - ₢BCAAE - W

Split the colophon census into required and permitted tiers. Permitted is admitted at the invoke chokepoint (rbtdri_invoke_impl now tests declared ∪ permitted) but never demanded by the negative post-fixture check (rbtdre_check_census still loops the required set alone, printing permitted usage as advisory-only lines) — fixing the census turn-on's one known defect, where foedus-reuse and freehold-establish census-failed their healthy paths. New rbtdrm_permitted_colophons carries the tier; a second thread-local rides rbtdri_census_arm; rbtdrc_set_context arms both from the manifest. AFFIANCE_MANOR moved to permitted for foedus-reuse and LEVY_DEPOT for freehold-establish; nothing else moved.

### 2026-07-12 07:48 - ₢BCAAE - n

Review repair: pin the invariant that a permitted declaration requires a required entry. rbtdrm_required_colophons returning None disables the census outright, and the positive check reads the permitted set only from inside that Some arm — so a fixture declaring permitted colophons with no required entry would silently get no census at all, the permitted list reading as coverage while enforcing nothing. Assert across the whole RBTDRA_FIXTURES roster so that pairing cannot land unnoticed.

### 2026-07-12 07:41 - ₢BCAAE - n

Split the colophon census declaration into required and permitted tiers. Permitted admits at the invoke chokepoint but is never demanded by the negative post-fixture check, fixing foedus-reuse (affiance fires only on a descry deficit) and freehold-establish (levy fires only on reuse-miss) census-failing their healthy paths. Moved RBTDGC_AFFIANCE_MANOR and RBTDGC_LEVY_DEPOT from required to permitted for those two fixtures; added rbtdrm_permitted_colophons alongside rbtdrm_required_colophons; threaded a second thread-local through rbtdri_census_arm/rbtdri_invoke_impl; rbtdre_check_census now prints permitted usage as advisory-only lines. Updated every existing census_arm call site to the two-arg form and added new unit tests proving permitted-invoked is not refused and permitted-unused does not fail, at both the invocation-chokepoint and fixture-run levels.

### 2026-07-12 07:32 - ₢BCAAD - W

Landed the coverage section of the touchstone surface fixture — four black-box cases certifying the colophon census enforcement from outside a child rbtd run: aligned exits 0 with the per-colophon usage line naming rbw-tn used; undeclared exits nonzero carrying the positive-check refusal from rbtdri_invoke_impl naming the colophon; unused passes its case then fails at fixture level with the negative-check census line naming the colophon; and the unused fixture's case driven standalone through rbw-tc exits 0, proving rbtdre_run_single_case's exemption. Case array 16 -> 20, tariff invocations declaration and const assert moved with it (one child spawn per case). Reviewed the work independently against the emitters rather than the report: both census diagnostic strings match the needles byte-for-byte (the positive check's message spans a backslash-continuation that strips the newline and indent, so the flat needle is a genuine substring); all four cases were confirmed non-vacuous by inversion (disabling either census direction, or the single-case exemption, flips the exact case that claims to cover it); and the exemption case proves real behavior, since rbtdrc_set_context arms the census on the single-case path too (main.rs:418) with a non-empty declaration and an empty used-set, so exit 0 is purchased by run_single_case genuinely skipping the check. One review repair applied: the exemption case asserted PASSED without asserting WHICH case passed, so a mis-resolved case would have satisfied it — now asserts the case name alongside, matching every sibling's convention. Verified on the committed tree: 187 theurge unit tests pass, touchstone 20/20 green with tariff exact at 20 invocations, shellcheck clean across 228 files, reveille 13/13 fixtures green at 145 passed / 0 failed / 0 skipped.

### 2026-07-12 07:31 - ₢BCAAD - n

Review repair on the coverage cases: the single-case exemption case asserted PASSED without asserting WHICH case passed, so a mis-resolved case would still have satisfied it. Assert the case name alongside PASSED, matching the convention every sibling case in the fixture already follows.

### 2026-07-12 07:27 - ₢BCAAD - L

claude-sonnet-5 landed

### 2026-07-12 07:25 - ₢BCAAD - n

Add the coverage section to the touchstone surface fixture: four black-box cases proving the colophon census enforcement's diagnostics from outside a child rbtd run — aligned exits 0 naming the colophon used, undeclared fails naming it via the positive chokepoint check, unused fails naming it via the negative post-fixture check, and the unused fixture's case run standalone through the single-case runner is exempt and exits 0.

### 2026-07-12 07:21 - ₢BCAAC - W

Landed the three calibrant-coverage-* fixtures (aligned/undeclared/unused) and the synthetic rbw-tn colophon proving the colophon census enforcement lands its diagnostics in both directions, then reviewed and repaired the work. Repairs: re-minted the colophon's word from the trodden 'Noop' to 'Nihil' across every namespace it touches (tabtarget, zipper enrollment, rbte_nihil verb, generated RBTDGC_THEURGE_NIHIL const, Rust helper, test names) with the colophon rbw-tn itself unchanged; split calibrant-coverage-undeclared into its own manifest match arm with a do-not-repair warning, since its empty declaration is the thing under test rather than a vacuous one and a maintainer 'fixing' the failing fixture would have silently deleted the test; renamed rbtdtl_required_colophons_all_empty, whose name asserted a family-wide invariant that two calibrants now break; dropped a heat-silks reference that would have rotted in a code comment; refreshed stale calibrant-family counts in the almanac and test-module headers. Verified independently on the committed tree: 187 unit tests pass, all three fixtures emit exactly their intended diagnostics (aligned exits 0 with the used-colophon census line; undeclared exits 1 via the rbtdri_invoke_impl positive-check refusal; unused passes its case then exits 1 via the engine's negative census check), shellcheck clean across 228 files, reveille green at 13 fixtures / 141 passed / 0 failed with no regression from the rename. Sibling touchstone pace should assert on the census diagnostic lines rather than on the case tally, since a fixture-level census failure folds into the case-failure counter and reports '1 passed, 1 failed (2 total)' for a single-case fixture.

### 2026-07-12 07:18 - ₢BCAAC - n

Review repairs on the calibrant coverage work: re-mint the synthetic colophon's word from the trodden 'Noop' to 'Nihil' (colophon rbw-tn unchanged; tabtarget, zipper enrollment, rbte_nihil verb, and the generated RBTDGC_THEURGE_NIHIL const all follow); give calibrant-coverage-undeclared its own manifest match arm with a do-not-repair warning, since its empty declaration is the thing under test rather than a vacuous one; rename the misleading rbtdtl_required_colophons_all_empty test to rbtdtl_verdict_family_declares_no_colophons, as two calibrants now declare a colophon; drop the rotting heat-silks reference from the zipper comment; refresh the stale calibrant-family counts in the almanac and test-module headers

### 2026-07-12 07:07 - ₢BCAAC - n

Verification run on the committed tree: all 187 theurge unit tests pass (183 prior + 4 new registration-shape tests for the calibrant-coverage-* fixtures); reveille suite runs 13/13 green, 141 passed/0 failed/0 skipped; manual FixtureRun spot-checks confirm calibrant-coverage-aligned passes clean, calibrant-coverage-undeclared fails via the positive census check (rbtdri_invoke_impl refusal), and calibrant-coverage-unused passes its one case then fails at fixture level via the negative census check — exactly the three diagnostics the sibling touchstone pace will assert against; shellcheck clean across 228 files.

### 2026-07-12 07:05 - ₢BCAAC - n

Mint the synthetic rbw-tn noop colophon and add three calibrant-coverage-* fixtures (aligned/undeclared/unused) proving the colophon census enforcement, plus registration-shape unit tests

### 2026-07-12 06:56 - Heat - T

bridled ₢BCAAF at sonnet

### 2026-07-12 06:56 - Heat - T

bridled ₢BCAAE at sonnet

### 2026-07-12 06:56 - Heat - S

census-declare-unmanifested

### 2026-07-12 06:55 - Heat - S

census-permitted-tier

### 2026-07-12 06:53 - ₢BCAAB - W

Colophon census enforced both directions. Positive: rbtdri_invoke_impl refuses an undeclared colophon at the funnel, the only launch path that can refuse. Negative: rbtdre_run_fixture fails a fully-run green fixture carrying a declared-but-never-launched colophon, gated on zero failed AND zero skipped cases so self-skipping suite-passengers stay protected; single-case runs exempt. Used-set recording rides rbtdri_tabtarget_command — the universal launch chokepoint the tariff already rides, colophon derived from the script filename's leading dot-segment — so direct-Command bypass launches count. Arm/disarm rides rbtdrc_set_context/take_context beside the credless guard, derived from the manifest. Turn-on audit: full static attribution sweep over every funnel fixture surfaced and repaired six drifted declarations (podvm-resolve presage-not-immure; hallmark-lifecycle +summon +plumb-full; lode/wsl +augur; reliquary +augur +list-images +jettison-image; onboarding-sequence +feoff +anoint +drive); all other funnel fixtures verified aligned. 183 unit tests green; reveille 13/13 green with per-colophon usage reports live. Known residue: foedus-reuse and freehold-establish carry conditional-by-design colophons whose healthy paths census-fail — follow-up two-tier ruling.

### 2026-07-12 06:50 - ₢BCAAB - n

Verification run on the committed tree: all 183 theurge unit tests pass (181 prior + the new bypass-recording and skip-suppression census tests); the full reveille suite runs 13/13 green — 141 passed, 0 failed, 0 skipped — with census enforcement live on every fixture, the per-colophon usage report printing for every non-empty declaration (podvm-resolve rbw-lp, handbook-render 7/7, dockerfile-hygiene 2/2, touchstone 3/3), and the three formerly-red fixtures green via the universal-chokepoint used-set recording plus the repaired podvm-resolve declaration. Static attribution sweep completed over every funnel fixture: crucible x4, touchstone, podvm-lifecycle, foedus-lifecycle, batch-vouch, access-probe, credential-readiness, polity-denial, parley, depot-lifecycle, freehold-churn, kludge-tadmor, and dogfight verified aligned; the six drifted declarations repaired; foedus-reuse and freehold-establish flagged as conditional-by-design (their healthy-path runs will census-fail until the required-vs-permitted ruling lands).

### 2026-07-12 06:49 - ₢BCAAB - n

Census hardening after independent review of the first landing, three corrections plus the operator-blessed drift repairs. (1) Used-set recording moves to rbtdri_tabtarget_command — the universal launch chokepoint the tariff already rides — with the colophon derived from the script filename's leading dot-segment, so direct-Command bypass launches (rbtdrf_fast, rbtdrf_handbook helpers) satisfy the negative direction; positive refusal stays at the rbtdri_invoke_impl funnel, the only path that can refuse. (2) The negative check is now gated on zero skipped as well as zero failed cases — a skipped run is not exhaustive, preserving suite-passenger protection for self-skipping fixtures (polity-denial, parley). (3) Census arm/disarm rides rbtdrc_set_context/rbtdrc_take_context beside the credless guard, derived from the fixture's manifest entry, replacing the three forgettable main.rs arm sites; the single-case charge probe now runs pre-context and thus deliberately outside the fixture's census. Manifest drift repairs from the full static attribution sweep (all funnel fixtures audited): podvm-resolve declares presage (rbw-lp) not immure (rbw-lI); hallmark-lifecycle +summon +plumb-full (step-6b vacant-band checks); lode/wsl-lifecycle +augur; reliquary-lifecycle +augur +list-images +jettison-image (member-jettison block); onboarding-sequence +feoff +anoint +drive. Conditional-by-design colophons left untouched and flagged for a follow-up ruling: foedus-reuse affiance (fires only on descry deficit) and freehold-establish levy (reuse-or-levy) census-fail a healthy standalone run. New tests: bypass launches record into the used-set; skip suppresses the negative check; existing census tests reordered to arm after rbtdrc_set_context (which now derives from the manifest).

### 2026-07-12 06:22 - ₢BCAAB - n

Enforce the per-fixture required-colophons census both directions: rbtdri_invoke_impl refuses an invoke of an undeclared colophon (positive), and rbtdre_run_fixture fails a fully-green run carrying a declared-but-never-invoked colophon (negative), gated on zero failed cases. Census state is thread-local in rbtdri_invocation (paralleling the credless guard and tariff tally), armed explicitly via rbtdri_census_arm(Option<declared>) right after each Context::new() in main.rs; None (no manifest entry) disables tracking entirely so ad hoc invocation-mechanics tests are unaffected. Single-case runs stay positive-only since rbtdre_run_single_case never calls the new rbtdre_check_census. Unit tests cover both directions and the single-case exemption.

### 2026-07-11 16:59 - ₢BCAAA - W

Touchstone surface fixture landed: rbtdrj_touchstone.rs, a 16-case credless reveille member spawning child rbtd runs through the real tabtarget chain against the calibrant fixtures, asserting exit codes, diagnostic shape on the folded operator stream, sentinel/trace files, stream placement under BURD_NO_LOG, and BURV_LOG_DIR log isolation (regression-guarding the BrAAL launcher-spine override). Keep-going plumbed end to end (--keep-going through rbte_run/rbte_suite into rbtdre_resolve_fail_fast, refusal before setup); calibrant suite registered with its rbw-ts tabtarget as the suite-abort subject; touchstone joined RBTDRA_REVEILLE_BASE and all six base-bearing suites per the set-equality guard. RCG compliance pass extracted RBTDRE_WORD_* verdict-word consts and z-prefixed internals. Verified: 175/175 unit tests, touchstone 16/16 standalone (~9s, tariff-exact), reveille green 141/0/0 with coherent parent last.txt, shellcheck clean 228 files.

### 2026-07-11 16:58 - ₢BCAAA - n

Verification run on the committed tree: all 175 theurge unit tests pass (including the four new rbtdre_parse_keep_going tests and the wrapper-model oracle now classifying calibrant); the touchstone fixture passes 16/16 standalone via tt/rbw-tf.FixtureRun.sh with invocations exactly on its declared tariff (16, ~9s); the full reveille suite runs green with touchstone as its thirteenth member — 13 fixtures, 141 passed, 0 failed, 0 skipped — and the parent suite's last.txt stayed coherent because every deliberately-failing child logged into its BURV_LOG_DIR override dir; shellcheck qualification clean across 228 files including the edited rbte_engine.sh and the new rbw-ts.TestSuite.calibrant.sh tabtarget.

### 2026-07-11 16:56 - ₢BCAAA - n

Classify the calibrant suite in the wrapper(inner) oracle: Base wrapper (substrate-independent, like reveille — its deliberately-failing inner bodies touch no substrate), spelled via the lib's RBTDRA_SUITE_NAME_CALIBRANT const rather than a test-side duplicate. The oracle's exhaustiveness test caught the unclassified suite exactly as designed.

### 2026-07-11 16:55 - ₢BCAAA - n

RCG compliance pass over the touchstone work: internal identifiers take the z/Z prefix (zrbtdrj_Child, ZRBTDRJ_CHILD_TRACE_SUBDIR); touchstone and the edited main.rs use-block reformatted to one-import-per-line sorted; the PASSED:/FAILED:/SKIPPED: verdict words extracted to single RBTDRE_WORD_* consts in the engine, rewiring the engine's own duplicated emission sites (run_cases, run_single_case, write_trace, tariff too-fast) and the touchstone assertions to the one definition; main.rs usage strings derive --keep-going from RBTDRE_FLAG_KEEP_GOING instead of repeating the literal; per-case label literals bound once per case fn; one process-language doc comment rephrased to permanent form. Build clean under deny(warnings).

### 2026-07-11 16:48 - ₢BCAAA - n

Touchstone surface fixture landed: new rbtdrj_touchstone.rs reveille member spawning child rbtd runs through the real tabtarget chain against the calibrant fixtures (16 cases across verdict-propagation, fixture-fail-fast, disposition-policy, probe-diagnostics, suite-abort, cli-surface, stream-placement, log-isolation), each logged child carrying a BURV_LOG_DIR override into its case dir plus TERM=dumb for colorless determinism. Keep-going plumbed end to end: --keep-going parsed by the binary (rbtdre_parse_keep_going, unit-tested), rbtdre_run_fixture resolves the disposition policy BEFORE setup so the StateProgressing refusal precedes any work, and rbte_run/rbte_suite pass extra CLI args through. Calibrant suite registered in RBTDRA_SUITES (fail-fast ordered before sentinel) with its rbw-ts.TestSuite.calibrant.sh tabtarget; touchstone joins RBTDRA_REVEILLE_BASE and all six base-bearing suites per the set-equality guard; manifest declares rbw-tf/rbw-ts/rbw-tc; rbtdri gains invoke_imprint_env; stale bash-blackbox-testbench comments amended to name touchstone. Build clean; tests not yet run (notch-before-test). Size limit raised to 60000 by operator direction — bulk is the 31KB hand-written touchstone module.

### 2026-07-11 16:40 - Heat - f

silks=rbk-21-mvp-theurge-self-test

### 2026-07-11 16:27 - Heat - d

batch: 1 reslate

### 2026-07-11 16:12 - Heat - f

racing

### 2026-07-11 16:11 - Heat - T

bridled ₢BCAAD at sonnet

### 2026-07-11 16:11 - Heat - T

bridled ₢BCAAC at sonnet

### 2026-07-11 16:11 - Heat - T

bridled ₢BCAAB at sonnet high

### 2026-07-11 16:10 - Heat - T

bridled ₢BCAAA at opus high

### 2026-07-11 16:08 - Heat - f

silks=rbk-21-win-theurge-tabtarget-auditing

### 2026-07-11 16:07 - Heat - T

coverage-surface-cases

### 2026-07-11 16:07 - Heat - T

calibrant-surface-fixture

### 2026-07-11 16:07 - Heat - d

batch: paddock, 4 reslate

### 2026-07-01 13:28 - Heat - d

batch: 2 reslate

### 2026-05-11 15:40 - Heat - f

silks=rbk-21-win-theurge-tabarget-auditing

### 2026-05-11 15:08 - Heat - f

silks=rbk-20-mvp-theurge-tabarget-auditing

### 2026-04-28 08:38 - Heat - d

paddock curried: amend BB section reference after BB scrub

### 2026-04-28 08:33 - Heat - d

paddock curried: initial paddock — split from ₣BB

### 2026-04-28 08:32 - Heat - D

restring 4 paces from ₣BB

### 2026-04-28 08:32 - Heat - N

rbk-mvp-3-theurge-self-test

