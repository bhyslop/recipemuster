# Heat Trophy: rbk-40-fbl-better-negatives

**Firemark:** ₣Be
**Created:** 260610
**Retired:** 260705
**Status:** retired

## Paddock

## Gate

Cleared (2026-06-12): the bash-filename case-consolidation landed and its heat retired;
this heat's docket file references have been refreshed to the lowercase successors.
This heat runs comfortably in a parallel repo alongside ₣Bb.

## Shape

Two strands, one gestalt: doctrine-pure negative testing.

Strand one — named refusals.
Theurge negative cases assert bare nonzero exit,
so a case expecting deliberate rejection also passes on any harness breakage
(unbound variable, missing file, refactor typo) —
the fast-tier fixtures carry this wrong-reason hole across their negative cases.
The fix: a small precision exit-code band, allocated per rejection gate
(roughly half a dozen codes; the census at the 2026-06-11 groom counted 6–8 gates),
homed as tinder in bubc,
projected into the theurge Rust consts by the existing zipper codegen,
and asserted specifically by the negative helpers.

Strand two — the tweak channel re-grounded (added at the 2026-06-11 groom).
The BUS0 Tweak Mechanism doctrine (landed at heat start) states what a tweak is for:
force one hard-to-produce condition for a test to observe handled correctly;
one tweak at a time per test/fixture/suite, by design;
a suite may reserve the slot for a standing guard.
This heat brings the live census into conformance:
the two non-conforming tweaks retire
(the immure convenience short-circuit becomes a real read-only dry-run colophon;
the graft parameter injection becomes the hallmark-installer election chain,
riding the existing hallmark fact + previous-dir chaining machinery),
the fast tier reserves the slot for the credless guard
(closing the recorded near-miss class: a passing fast suite that spends money and mutates the depot,
gated at the token-mint chokepoint with a band code),
and the regime-validation negatives convert from fabricated files driven through
test-only `*_probate` side doors to in-universe poisoning —
real validate verbs against real regimes, one BUK regime-load seam with set/unset semantics —
housed in a new fixture that runs in every suite above fast
(fast holds the guard; poison cases need the slot; the two never share a run).

Work order: census cleanup first (band-independent),
then band membrane + numeric codegen,
then the guard (its rejection code needs the band),
then the conversions, prose last.

## Cinched

- Exit-code band over stderr sentinel:
  a sentinel is string interpretation minimized, not avoided,
  and is swallowed by the same wrappers that launder codes.
- Per-gate codes, never per-rule —
  the hole being closed is wrong-layer failure, not wrong-rule failure.
- Allocation rule: gates may share a code only if they never co-occur
  in one test case's spawn path — share across alternatives, never along a pipeline.
- `buc_die` propagates in-band `$?` values instead of remapping to 1,
  so existing `|| buc_die` call sites need no audit or change.
- No code minted outside the bubc tinder block.
- One tweak at a time is doctrine, not limitation;
  a dedicated constraint variable mints only when a genuine dual-tweak need survives scrutiny —
  never by widening the single slot.
- Fast carries the credless guard in the tweak slot;
  fast cases carry no tweaks of their own.
  A case that needs a seam has self-identified as not belonging in fast.
- In-universe over fabrication: negatives run real verbs against real regimes wherever the
  injection point is a value; the probate side doors delete with no parallel survival.
- The guard gates token mint (actual credential use), never zipper/dispatch — zippers untouched.
- The hallmark installer is graft-slot election only;
  any general election verb belongs to the made-side retrofit heat.

## Out of scope

- tadmor-security's containment cases:
  exit codes there cross docker-exec boundaries with their own laundering rules —
  a different problem, a different heat if it ever itches.
- enrollment-validation needs no in-universe conversion:
  it already stages values inline against the validator with no file fabrication and no tweak;
  it takes only the band-code assertion migration.
- recipe-validation and dockerfile-hygiene fabrication is honest:
  the artifact under test IS a file; they take only the band-code migration.

## Done when

Every fast-tier negative case asserts a specific band code,
the survival proof lives in the BUK self-test,
the tweak census conforms to the BUS0 doctrine (stamp, poison seam, guard — nothing else),
no regime `*_probate` side door survives,
fast is credless by construction,
and the BCG admission entry documents band semantics and the allocation rule.

## Paces

### evict-immure-resolve-only-tweak (₢BeAAG) [complete]

**[260612-1222] complete**

## Character
Tweak-census cleanup, first of three — small and file-local plus one fixture repoint.
Tier: sonnet-delegable.

## Goal
Retire the `buorb_immure_resolve_only` tweak —
a doctrine violation per the BUS0 Tweak Mechanism section (convenience short-circuit, not condition-forcing).
Give the immure resolve/selection path its own read-only lowercase colophon:
a dry-run/plan verb with operator value in its own right (show what immure would capture),
minted at mount-time per Word Selection against the live colophon tree.
Repoint the fast podvm-resolve case to invoke the new colophon directly,
dropping the tweak export and the seam-marker output assertion it existed to carry.

## Cinched
- The tweak read in the immure body and both theurge constants go; no compatibility shim.
- New colophon is lowercase (touches no GAR/cost state), in the Lode family.
- Full immure behavior unchanged.

## Done when
`grep -rn "buorb_immure_resolve_only" Tools/` returns zero;
fast suite green with the podvm-resolve case exercising the new colophon.

### graft-chaining-installer-conversion (₢BeAAH) [complete]

**[260612-1258] complete**

## Character
The one pace in this heat that mints made-side operator surface (a new election verb + colophon):
word-selection and colophon ceremony, plus test rewiring whose verification rides the next gauntlet/skirmish run.
Tier: opus-driven.

## Goal
Retire the `buorb_graft_image` tweak —
a doctrine violation per the BUS0 Tweak Mechanism section (parameter injection for a positive path;
its original dynamic-chaining rationale lapsed when the conjure-to-graft chain test retired) —
by minting the hallmark installer:
a lowercase consumer-family election verb that reads the chained hallmark fact
(`rbf_fact_hallmark`, already emitted by every ordain and kludge, via the BURD previous-dir chaining machinery)
and writes a vessel's graft-image slot.
Yoke-precedent properties: operator-committed, never self-committing, no dirty-tree self-gate.
Rewire the onboarding graft case to chain through the installer instead of tweak-injecting,
and replace the fossil graft-image value in the committed graft-demo vessel regime.

## Open at mount
Whether the rewired case chains a dynamic hallmark from a prior conjure
(tracked-regime churn per run — the yoke precedent)
or proves the installer against a staged vessel copy with a static-primed source.

## Cinched
- Installer scope is narrow: graft-slot election only.
  General hallmark election into nameplates stays bundled in the cycle tabtargets
  and any broader verb belongs to the made-side retrofit heat.
- The tweak read in the foundry director build and the theurge mirror constant both go; no shim.

## Done when
`grep -rn "buorb_graft_image" Tools/` returns zero;
the onboarding graft case passes via the installer chain on its next gauntlet or skirmish run;
fast green.

### band-tinder-and-die-membrane (₢BeAAA) [complete]

**[260612-1324] complete**

## Character
The heat's keystone membrane — small surface, subtle exit-status semantics;
every later negative assertion rides through it.
Tier: opus-driven.

## Goal
Establish the precision exit-code band and make `buc_die` transparent to it.

Define band tinder in `Tools/buk/bubc_constants.sh`:
band base/width plus per-gate rejection codes —
per rejection gate, never per validation rule.
Pick the band clear of shell-reserved codes (2, 126, 127, 128+n).

Teach `buc_die` (`Tools/buk/buc_command.sh`) to capture `$?` on entry
and re-exit with it when the value lies in the band;
add an origin helper for deliberate rejection sites.
Existing `cmd || buc_die` call sites stay untouched —
the membrane makes them propagate band codes for free.

Prove end-to-end survival:
a BUK self-test case where a band code raised under a `cmd || buc_die` chain
reaches the caller's captured exit status through the tabtarget/launcher exec path.

## Cinched
Exit-code band over stderr sentinel;
per-gate codes, not per-rule.

## Done when
Band tinder and membrane land;
`tt/buw-st.BukSelfTest.sh` green including the new survival case.

### numeric-const-codegen (₢BeAAB) [complete]

**[260612-1330] complete**

## Character
Mechanical codegen extension riding an existing emit pattern.
Tier: sonnet-delegable.

## Goal
Project the band constants into Rust as numeric consts.

Extend the rbcc emission (`Tools/rbk/rbcc_constants.sh`, riding
`rbz_emit_consts` in `Tools/rbk/rbz_zipper.sh` → `rbtdgc_consts.rs`)
with a numeric emit shape — current emission is `&str` only,
theurge compares exit codes as integers.
bubc is sourced at emission time,
so projecting BUK-homed band values needs no cross-module tinder trick.
Write-on-change and the qualify freshness gate apply unchanged.

## Done when
Generated `rbtdgc_consts.rs` carries the band as numeric consts;
`tt/rbw-tb.Build.sh` regenerates it and `tt/rbw-tq.QualifyFast.sh` passes freshness.

### fast-credless-guard (₢BeAAI) [complete]

**[260612-1352] complete**

## Character
Hazard-closure — closes the recorded near-miss class where a PASSING fast suite spends money and mutates the depot.
Small surface: one gate at the token-mint chokepoint, one suite-runner export, one proof case, prose.
Tier: sonnet-delegable with care.

## Goal
Make fast-tier credlessness structural.
The fast suite runner sets the credless-guard tweak on every case invocation,
and the OAuth/JWT token-mint chokepoint (the rbgo/rba path) dies instantly under it
with its own band rejection code naming the violation.
This establishes the suite invariant from the BUS0 tweak doctrine's slot-reservation rule:
fast cases carry no tweaks of their own — in the fast tier the slot belongs to the guard.
Add the doctrine-pure proof case:
a deliberate cloud-verb invocation under the guard asserting the guard's specific band code.
State the convention in the theurge/test authoring context where an author of a new fast case will read it.

## Open at mount
Guard scope when fast fixtures run inside larger suites:
the service/complete suites include the fast fixtures alongside credentialed and poison-tweak fixtures,
so decide whether the guard rides only fast-suite runs
or every invocation of a fast-tier fixture's cases regardless of hosting suite —
the invariant (fast cases carry no tweaks of their own) holds either way.

## Cinched
- The guard rides the single tweak slot; no new mechanism.
  The demolition condition is the doctrine's own: a genuine dual-tweak need mints the dedicated variable then.
- The gate lands at token mint, gating actual credential use — never at zipper/dispatch; zippers untouched.
- The guard's rejection code is a band code from the bubc tinder block.

## Sources
Memos/memo-20260610-heat-BH-fast-tier-credless-by-convention.md
(the near-miss record; the spot-fix seam it describes retires earlier in this heat).

## Done when
A fast case reaching token mint dies with the guard's band code by construction;
the proof case asserts that code;
fast suite green;
the convention documented in the test authoring context.

### migrate-regime-validation-negatives (₢BeAAC) [complete]

**[260615-0927] complete**

## Character
The heat's centerpiece conversion — replaces fabricated-regime probate testing with in-universe poisoning.
Opens with a short census; the conversion itself is mechanical but each case needs real reading.
Tier: opus-driven with sonnet-delegable bodies.

## Goal
Convert the regime-validation negative cases from Rust-fabricated regime files
driven through the test-only `*_probate` side-door entries
to in-universe tests:
the real validate verbs run against real regime files,
with a regime-poison tweak forcing the invalid condition —
one BUK seam in the regime-load path, post-source pre-validate,
supporting set and unset of one named variable —
and each case asserting its rejection gate's specific band code.
House the converted negatives in a NEW fixture outside the fast suite:
fast reserves the tweak slot for the credless guard, and poison cases need the slot,
so this fixture enrolls in the service, crucible, and complete suites instead.
The regime-validation positives stay in fast,
converted to drive the real validate verbs directly (no poison, no tweak) —
the probate doors they currently ride delete with the negatives,
so the positive cases must come off them too.
Delete each `*_probate` side-door entry as its consumers convert.
Update the CLAUDE.md test-table guidance:
regime/validation changes run fast plus the new fixture.

## Open at mount
- Baseline availability census: which regimes have a valid in-tree baseline (nameplate/vessel/repo)
  versus operator-local (station/payor/oauth/auth/depot) needing a staged valid-baseline fixture file —
  the census covers positives and negatives alike.
- Per-gate band-code allocation against the allocation rule
  (share across alternatives, never along a pipeline).

## Cinched
- The poison seam is one membrane in BUK regime-load with set+unset semantics, carried on the tweak channel.
- The probate doors delete — no parallel survival.
- The enrollment-validation, recipe-validation, and dockerfile-hygiene fixtures are out of scope here:
  their staging shapes are already honest (inline values against the validator; the artifact under test is a file).

## Done when
The regime-validation negatives run in-universe in the new fixture asserting specific band codes;
the positives run in-universe in fast;
no regime `*_probate` entry survives in Tools/;
fast green;
the new fixture green standalone via the single-fixture tabtarget.

### migrate-enrollment-validation-negatives (₢BeAAD) [complete]

**[260615-0950] complete**

## Character
Mechanical assertion flip;
the only judgment is reuse-or-build on the expect-code plumbing.
Tier: sonnet-delegable.

## Goal
Close the wrong-reason hole for the enrollment-validation fixture's negative cases.

Teach the enrollment validation gate to reject with its band code,
and flip the fixture's negative cases to assert that specific code.
This fixture needs no in-universe conversion —
it already stages values inline against the validator, file-fabrication-free and tweak-free —
and it stays in fast.
Expect-code assertion plumbing for the fast-tier negative helpers lands here
if the regime conversion pace has not already provided a shareable form;
check before building.

## Done when
`tt/rbw-ts.TestSuite.fast.sh` green
with every enrollment-validation negative case asserting the gate's code.

### migrate-recipe-and-hygiene-negatives (₢BeAAE) [complete]

**[260615-1049] complete**

## Character
Mechanical assertion flip across two fixtures;
one one-look judgment on whether the two gates may share a code.
Tier: sonnet-delegable.

## Goal
Close the wrong-reason hole for the recipe-validation
and dockerfile-hygiene fixtures' negative cases.

Teach the recipe validation gate and the rbfh FROM-line hygiene gate
(`Tools/rbk/rbfh_hygiene.sh`) to reject with band codes.
These two gates are alternatives — never in one spawn path —
so they may share a code per the allocation rule;
confirm that at migration time rather than assuming.
Flip both fixtures' negative cases to assert the expected code.

## Done when
`tt/rbw-ts.TestSuite.fast.sh` green
with both fixtures' negative cases asserting specific codes.

### bcg-precision-band-admission (₢BeAAF) [complete]

**[260615-1106] complete**

## Character
Prose craft against settled design — judgment in wording, not in shape.
Tier: opus-driven.

## Goal
Write the BCG admission entry for the precision exit-code band.
State the design position:
differentiated exit codes are bad design, interpreting error strings is worse,
so a small per-gate band exists for exactly the tabtargets the test orchestrator asserts negatively — nothing else.
Include: band semantics (in-band means deliberate rejection, code 1 stays "imprecise death"),
the allocation rule (share across alternatives, never along a pipeline),
the enrollment requirement (no code minted outside the bubc tinder block),
and the rejected stderr-sentinel alternative with why it lost.
Cross-reference from the bubc tinder block comment.
Then sweep the tweak-doctrine touchpoints into agreement with the BUS0 Tweak Mechanism section
(landed at heat start):
the CLAUDE.md BUK include's tweak paragraph,
the fast-tier slot reservation (the credless guard),
and the in-universe poison seam as the doctrine's worked examples —
confirming the live tweak census conforms (stamp, poison, guard) with no stale exceptions.

## Done when
BCG carries the band entry; the bubc tinder block points at it;
the CLAUDE.md BUK include and BUS0 agree;
the doctrine's worked examples name the live census.

### cupel-python-scan-split (₢BeAAJ) [complete]

**[260615-1158] complete**

## Character
Mechanical Rust refactor — move code between modules, no behavior change.
Sonnet-delegable body with driver verification.

## Goal
`Tools/rbk/rbtd/src/rbtdru_cupel.rs` stands ~1330 lines,
past the RCG 800-line stop-and-ask threshold;
the operator elected a split at the ₣BH terminal triage (2026-06-12).
Carve the python-scan machinery (the import-allowlist scanning surface)
into its own module under the rbtd prefix discipline,
leaving the shell-side cupel checks in place;
module name and exact seam are mount-time choices.

## Cinched
- Pure relocation — no scanner behavior change, no new checks ride along.
- No tidemark: the moved code carries no "moved from" narration; the commit tells the story.

## Done when
`rbtdru_cupel.rs` is back under the 800-line threshold,
the new module obeys rbtd naming discipline and the acronym map knows it,
and the `fast` suite (cupel fixture included) is green.

### bubc-moorings-tinder-relocate (₢BeAAK) [complete]

**[260615-1848] complete**

## Character
Mechanical relocation plus a multi-consumer rename.
The judgment — namespace, naming, home — is settled in the memo.

## Goal
`BUBC_moorings_dir` reads a runtime variable (`BURD_CONFIG_DIR`) in tinder position —
a BCG tinder-purity violation, since `bubc` declares itself pure source-time literals.
Relocate the derivation into `bul_launcher` bootstrap as `BURD_MOORINGS_DIR`,
beside the existing `BURD_REGIME_FILE` derivation;
enroll it in `burd_regime`, allowlist it in `bud_dispatch`, rename the consumers;
`bubc` is left holding only pure literals.
Analysis, topology, and rejected alternatives:
`Memos/memo-20260615-moorings-basename-tinder-relocation.md`.

## Cinched
- Relocate, not delete — the basename is a load-bearing operator-display idiom (repo-root-relative, actionable from cwd); delete-and-absolutize and export-from-z-launcher were both rejected (memo).
- `SCREAMING` name per BCG's kindle/runtime-derived rule, not `lower_snake`.
- Home is bootstrap (`bul_launcher`), not `zburd_kindle` — two consumers run before the BURD kindle.
- A `BURD_` var rides two scope guards: `bud_dispatch`'s `z_known` allowlist and `zburd_kindle`'s `buv_scope_sentinel` — register with both.
- Only the runtime-derived basename moves; the pure-literal subdir constants stay in `bubc`.

## Done when
`bubc` holds only pure literals;
`BURD_MOORINGS_DIR` is derived once in bootstrap, enrolled, and allowlisted;
all consumers reference it;
shellcheck clean; fast green.

### fable-full-heat-review (₢BeAAL) [complete]

**[260701-1314] complete**

## Character
Model-gated full-heat review requiring judgment.
This pace exists to be done by Fable 5, and only by Fable 5.

## Model gate
Before any review work, the mounting agent MUST confirm its own model ID is `claude-fable-5`.
If the active model is opus, sonnet, haiku, or anything other than Fable 5, decline immediately and stop — do not attempt, partially attempt, or best-effort the review under the wrong model.
Report the model mismatch to the operator and wait for a Fable session.
An opus attempt is the explicit non-goal: the operator wants Fable's eyes on this heat, nothing else.

## Goal
Review the ₣Be heat as landed, across all its paces.
Read the heat paddock first — it carries the shape, the cinched decisions, the Done-when, and the doctrine the work answers to (BUS0 tweak doctrine, the BCG precision exit-code band, in-universe-over-fabrication).
Assess correctness, doctrine conformance, and whether the landed work actually meets the paddock's Done-when or overclaims it.

## Done when
Fable has surfaced a review verdict to the operator — what holds, what is weak, and any gap between the Done-when and the work as landed.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 G evict-immure-resolve-only-tweak
  2 H graft-chaining-installer-conversion
  3 A band-tinder-and-die-membrane
  4 B numeric-const-codegen
  5 I fast-credless-guard
  6 C migrate-regime-validation-negatives
  7 D migrate-enrollment-validation-negatives
  8 E migrate-recipe-and-hygiene-negatives
  9 F bcg-precision-band-admission
  10 J cupel-python-scan-split
  11 K bubc-moorings-tinder-relocate
  12 L fable-full-heat-review

GHABICDEFJKL
x···xxxx···· rbtdrf_fast.rs
xx·xxx······ rbtdgc_consts.rs
··x··x··x·x· bubc_constants.sh
·x·xxx······ rbcc_constants.sh
·x···x··x··· CLAUDE.md
·····x····x· burn_regime.sh, burp_regime.sh, burs_regime.sh
·····x···x·· lib.rs
····x····x·· rbtdru_cupel.rs
····xx······ rbtdrc_crucible.rs, rbtdri_invocation.rs
·x··x······· rbtdro_onboarding.rs
xx·········· claude-rbk-tabtarget-context.md, rbz_zipper.sh
···········x memo-20260701-fbl-be-negatives-review.md
··········x· RBS0-SpecTop.adoc, bud_dispatch.sh, bul_launcher.sh, buq_qualify.sh, burd_regime.sh, burn_cli.sh, burp_cli.sh, memo-20260615-moorings-basename-tinder-relocation.md
·········x·· rbtdru_bash.rs, rbtdru_python.rs, rbtdtu_cupel.rs
········x··· BCG-BashConsoleGuide.md, BUS0-BashUtilitiesSpec.adoc
·······x···· rbfh_hygiene.sh, rblds_spine.sh
······x····· RBSTR-Terrier.adoc, claude-rbk-acronyms.md
·····x······ burc_regime.sh, buv_validation.sh, rbra_regime.sh, rbrd_regime.sh, rbrn_regime.sh, rbro_regime.sh, rbrp_regime.sh, rbrr_regime.sh, rbrs_regime.sh, rbrv_regime.sh, rbtdrm_manifest.rs, rbtdrs_poison.rs
····x······· claude-rbk-theurge-ifrit-context.md, rba_auth.sh, rbgo_oauth.sh, rbgp_payor.sh, rbtdrd_dogfight.rs, rbtdre_engine.rs, rbtdrf_handbook.rs, rbtdrk_canonical.rs, rbtdrl_calibrant.rs, rbtdrn_conformance.rs, rbtdrp_pristine.rs, rbtdti_invocation.rs
···x········ buz_zipper.sh
··x········· buc_command.sh, butcbd_band.sh, buto_operations.sh, butt_testbench.sh, buw-xb.BandChain.sh, buwz_zipper.sh, bux_cli.sh
·x·········· rbfd_director.sh, rbfl0_ledger.sh, rbfla_anoint.sh, rbrv.env, rbw-rva.DirectorAnointsGraftVessel.sh
x··········· rbldv_immure.sh, rbw-lp.DirectorPresagesImmure.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 57 commits)

  1 H graft-chaining-installer-conversion
  2 A band-tinder-and-die-membrane
  3 B numeric-const-codegen
  4 I fast-credless-guard
  5 C migrate-regime-validation-negatives
  6 D migrate-enrollment-validation-negatives
  7 E migrate-recipe-and-hygiene-negatives
  8 F bcg-precision-band-admission
  9 J cupel-python-scan-split
  10 K bubc-moorings-tinder-relocate
  11 L fable-full-heat-review

123456789abcdefghijklmnopqrstuvwxyz
x··································  H  1c
·xx································  A  2c
···xx······························  B  2c
·····xx····························  I  2c
·······xxxxxxxx·x··················  C  9c
·················xxx···············  D  3c
····················xx·············  E  2c
······················xx···········  F  2c
························xx·········  J  2c
··························xxx······  K  3c
·································xx  L  2c
```

## Steeplechase

### 2026-07-01 13:14 - ₢BeAAL - W

Fable full-heat review of ₣Be delivered: verdict is the work holds — band membrane, BUK self-test survival proof, structural credless arming, poison seam, suite composition, and tweak census all verified doctrine-conformant against the paddock Done-when. One small Done-when overclaim found (rbw-dU empty-arg refusal case asserts bare nonzero + output text; repair is a BCG usage-refusal carve-out line, not a new band code) plus stale-prose nits and a forward note on the hand-kept band emit list. Findings and the settled ₢BoAAB admission-band design recorded in Memos/memo-20260701-fbl-be-negatives-review.md (notched 27aef834c); the follow-ups fold into the reslated ₢BoAAB. Wrap sweep of the in-flight theurge-cosmology tree files was operator-accepted. Live suite runs deferred to ₢BoAAB per the same tree state.

### 2026-07-01 13:11 - ₢BeAAL - n

Fable full-heat review of ₣Be as landed: verdict is the work holds — band membrane, survival proof, structural credless arming, poison seam, and census all verified doctrine-conformant; one small Done-when overclaim (the rbw-dU empty-arg refusal case asserts bare nonzero + output text; repair is a documented usage-refusal carve-out in BCG, not a new band code) plus stale-prose nits (podvm-resolve manifest comment, keyfile-ghost guard comment, pre-rename suite names in the poison header, CLAUDE.md suite-table drift, a coronet in a durable code comment) and a forward note that the hand-kept band emit list must be extended at every mint. Memo also records the settled ₢BoAAB design so the reslated docket points rather than restates: BUBC_band_admission=109, the 403-arm return-signal change, membrane fan-out to existing consumers, explicit reject at rbgv_check_mantle only, and the don/unseat/restore fixture shape.

### 2026-06-26 10:56 - Heat - f

silks=rbk-40-fbl-better-negatives

### 2026-06-18 13:08 - Heat - f

silks=rbk-09-mvp-FABLE-better-negatives

### 2026-06-17 19:32 - Heat - f

silks=rbk-09-mvp-better-negatives

### 2026-06-15 18:52 - Heat - S

fable-full-heat-review

### 2026-06-15 18:48 - ₢BeAAK - W

Relocated the moorings-basename out of bubc tinder, closing the last BCG tinder-purity violation in the heat. BUBC_moorings_dir read runtime BURD_CONFIG_DIR in tinder position (BCG line 447); relocated to BURD_MOORINGS_DIR derived once in bul_launcher bootstrap (SCREAMING, beside BURD_REGIME_FILE; bootstrap not zburd_kindle because two consumers run before the BURD kindle), enrolled in burd_regime, allowlisted in bud_dispatch z_known. bubc keeps only pure literals. Dropped the now-redundant source bubc in zburd_kindle (buv_validation provisions bubc universally before any kindle). Renamed ~9 display consumers + RBS0 cross-ref. Shellcheck 212 clean; fast 109/0/0 green. Code at a5678d8e6.

### 2026-06-15 18:46 - ₢BeAAK - n

Relocate the moorings-basename out of bubc tinder, closing the BCG tinder-purity violation (BUBC_moorings_dir read runtime BURD_CONFIG_DIR in tinder position — BCG line 447, runtime expansion in tinder; lower_snake name was the tell). Relocate to BURD_MOORINGS_DIR: derived once in bul_launcher bootstrap beside BURD_REGIME_FILE (SCREAMING per BCG runtime-derived rule; bootstrap not zburd_kindle because two consumers — the SETUP NEEDED block and burs_regime's enroll description — run before the BURD kindle), enrolled in burd_regime as sibling to BURD_CONFIG_DIR, and added to bud_dispatch's z_known allowlist (the two scope guards a BURD_ var rides). bubc keeps only its pure-literal subdir constants, matching its 'source-time literal' self-declaration. Dropped the now-redundant source bubc in zburd_kindle: investigation confirmed buv_validation.sh sources bubc at module top and every zburd_kindle caller sources buv first, so bubc is universally provisioned before any kindle — the memo's 'no other caller affected' conclusion held, though its stated reasoning (no BURD_ desc references BUBC) was incidental; the real provider is buv. Renamed all ~9 display-message consumers (burs/burp/burn regimes+CLIs, buq_qualify, bul_launcher SETUP block) and the RBS0 cross-ref comment. Analysis in Memos/memo-20260615-moorings-basename-tinder-relocation.md. Shellcheck 212 clean; fast verification follows this notch (clears the theurge clean-tree gate).

### 2026-06-15 12:46 - ₢BeAAK - n

Memo capturing the moorings-basename tinder-relocation analysis (pace bubc-moorings-tinder-relocate): the BCG tinder/kindle violation, the single-source topology (no per-launcher copies, no manual sync), the consumer-as-display-idiom finding, the rejected DRY-from-z-launcher and delete-and-absolutize alternatives, the chosen BURD_MOORINGS_DIR relocation, and the SCREAMING-naming + two-scope-guard + bootstrap-timing gotchas. Execution deferred for operator review.

### 2026-06-15 11:58 - ₢BeAAJ - W

Split rbtdru_cupel.rs (1318 lines, past the RCG 800-line threshold) into three by-language modules under the shared rbtdru letter: cupel frame (380 lines — the BCG allowlists as source of truth, the Finding/ScanResult/Domain types, the corpus walk, reporting, run-drivers, fixture wiring), rbtdru_bash (720 — lexer + collect_functions + classify + is_external + scan_domain), rbtdru_python (315 — PyToken/py_tokens/py_import_roots/py_scan/scan_python). Pure relocation: every zrbtdru_ function kept its name (the rbtdrf two-file precedent licenses multiple files under one letter), so the diff is a body-move, not a rename. The docket's named seam (python only) was infeasible — the bash side plus the frame is ~1000 lines on its own, so under-800 forced the shell-side scanning out of cupel.rs too; the by-language split (cupel = allowlists+frame, one module per language) was chosen in conversation over keeping shell-side in place. Verified: theurge build clean under deny(warnings), 161 unit tests green, fast suite 109/109 (cupel's three cases pass — split scanners give identical verdicts). Code committed in b8e46430 (size-guard limit raised to 100000 for the relocation bulk, operator-approved). RBTD acronym map covers the new files at directory grain; no module-grain catalogue exists to extend (cupel itself was never listed there).

### 2026-06-15 11:56 - ₢BeAAJ - n

Split rbtdru_cupel.rs (1318 lines, past the RCG 800-line threshold) into three by-language modules under the shared rbtdru letter, so every zrbtdru_ function keeps its name — pure relocation, no scanner behavior change. rbtdru_cupel.rs (380 lines) keeps the frame: the BCG allowlists that are the single source of truth, the Finding/ScanResult/Domain types, the corpus walk, the trace-file reporting, the run-drivers, and the case/fixture wiring. New rbtdru_bash.rs (720 lines) holds the bash pipeline: the command-position lexer (read_word/keyword_kind/is_assignment/skip_balanced_parens/command_words) plus collect_functions, classify, is_external, and scan_domain. New rbtdru_python.rs (315 lines) holds the python cloud-step pipeline: PyToken/py_tokens/py_import_roots/py_scan/scan_python. The docket's named seam (python only) could not reach under 800 — the bash side plus the frame is ~1000 lines on its own — so the by-language split moves the shell-side scanning out of cupel.rs too, keeping only the allowlists-and-frame there; this revisits the docket's 'leave the shell-side checks in place' line, which was infeasible alongside the under-800 goal. ScanResult, walk_ext, and is_gcb bumped to pub(crate) for cross-module use; is_external stays private in the bash module. The rbtdtu_cupel test imports now split across the three modules. All three files obey rbtd naming; the RBTD acronym-map entry covers them at directory grain (individual rbtdr* modules are not catalogued at module grain, cupel included). Theurge build clean under deny(warnings); all 161 unit tests green. Fast-suite verification follows this notch, which clears the theurge clean-tree gate.

### 2026-06-15 11:06 - ₢BeAAF - W

Documented the precision exit-code band in BCG (design position, band semantics via buc_reject/buc_die membrane, the per-gate allocation rule with share-across-alternatives-never-along-a-pipeline, the bubc-only enrollment requirement, and the rejected stderr-sentinel alternative), cross-referenced it from the bubc tinder block, and swept the tweak doctrine into agreement: BUS0's worked examples now name the live behavioral census (stamp/poison/credless guard), and CLAUDE.md's tweak paragraph points to BUS0 as doctrine home, names the census, and corrects the stale buost_ description that predated the regime_poison seam. Shellcheck 212 clean.

### 2026-06-15 11:03 - ₢BeAAF - n

Document the precision exit-code band and bring the tweak doctrine into agreement across its homes. BCG gains a 'Precision Exit-Code Band' section stating the design position (differentiated exit codes are bad design, interpreting error strings is worse, so a small per-gate band exists for exactly the tabtargets the test orchestrator asserts negatively — nothing else), the band semantics (buc_reject as the in-band rejection origin, buc_die as the membrane that re-exits in-band $? unchanged and collapses everything else to imprecise 1), the allocation rule (one code per gate not per rule; share across alternatives, never along a pipeline), the enrollment requirement (no code minted outside the bubc tinder block), and the rejected stderr-sentinel alternative with why it lost (string interpretation minimized not avoided, and swallowed by the same wrappers that launder codes). buc_reject is threaded into the Message Hierarchy and the Quick Reference Decision Matrix. The bubc band comment now cross-references BCG and declares itself the sole mint. BUS0's Tweak Mechanism doctrine now names the live behavioral census against its worked examples — stamp (buorb_ensconce_stamp), poison (buost_regime_poison), credless guard (buorb_credless_guard) — with the fast tier named as the live slot-reservation instance, and accounts for buost_example as BUK's self-test channel stub. CLAUDE.md's tweak paragraph now points to BUS0 as the doctrine home, names the census, references the BCG band, and corrects the now-stale buost_ description (it homes the real regime_poison seam, not merely test-stub placeholders). Shellcheck 212 clean.

### 2026-06-15 10:49 - ₢BeAAE - W

Closed the wrong-reason hole for the recipe-validation and dockerfile-hygiene fixtures' negatives. The band-membrane pace had already minted both gate codes distinct (BUBC_band_recipe=102, BUBC_band_hygiene=103), so the docket's share-a-code question was moot — distinct is correct for two gates that never co-occur. Recipe gate (zrbld_spine_validate, a predicate consumed by `|| buc_die`): uncovered-register rejection returns BUBC_band_recipe so dispatch propagates 102 in-band and keeps the step-identity message while the direct-driving fixture sees 102; file-not-found preconditions stay return 1. Hygiene gate (rbfh_dockerfile_check): the three FROM-line rule violations buc_reject BUBC_band_hygiene; the two path preconditions stay buc_die. Both Rust harnesses moved from expect_ok:bool to expect_code:i32 with expected+actual+stderr in the mismatch message. bubc was already in scope on both paths via buv_validation — no new sourcing. Implemented by a delegated sonnet agent, reviewed line-by-line on the diff. Verified: fast green 109/0/0, with dockerfile-hygiene 9/9 (rejects assert 103 through the full rbw-fhc dispatch chain) and recipe-validation 10/10 (rejects assert 102). Shellcheck 212 clean; theurge build green. Code landed at commit 1b13c7c1c.

### 2026-06-15 10:17 - ₢BeAAE - n

Migrate the recipe-validation and dockerfile-hygiene fixtures' negatives from bare-nonzero to specific band-code assertion, closing the wrong-reason hole. The codes were already minted distinct by the band-membrane pace (BUBC_band_recipe=102, BUBC_band_hygiene=103), so the share-a-code question the docket flagged is moot — distinct is correct for two gates (recipe-substitution validation vs Dockerfile FROM-line hygiene) that never co-occur in one spawn path. Recipe gate (zrbld_spine_validate, a predicate consumed in production by `|| buc_die`): its uncovered-register rejection now `return`s BUBC_band_recipe instead of bare 1, so dispatch's `|| buc_die` propagates 102 in-band and keeps the step-identity message while the direct-driving fixture observes 102; the two file-not-found preconditions stay `return 1` (usage, not rejection). Hygiene gate (rbfh_dockerfile_check): the three FROM-line rule violations (tab, trailing backslash, unapproved image token) buc_reject BUBC_band_hygiene; the two path preconditions stay buc_die. Both Rust harnesses (rbtdrf_rc_run; rbtdrf_dh_run_synthetic + rbtdrf_run_tt_neg) take expect_code:i32 — negatives assert the exact band const, positives assert 0, and the mismatch message now carries expected+actual+stderr so an off-band harness breakage fails loud. bubc was already in scope on both gate paths transitively via buv_validation. Shellcheck 212 clean; theurge build green. Implemented by a delegated sonnet agent, reviewed line-by-line on the diff. Fast-suite verification pending — blocked by the theurge clean-tree gate, which this notch is intended to clear.

### 2026-06-15 09:50 - ₢BeAAD - W

Flipped the enrollment-validation fixture's negatives from bare-nonzero to specific-band assertion, closing the wrong-reason hole. Gate was already taught by the regime conversion (buv_vet/buv_scope_sentinel buc_reject BUBC_band_enroll=101); the regime pace's shareable form is the tweak-driven rbtdrs_poison harness, which does not fit this fixture's inline RbtdrfSub mechanism, so the expect-code plumbing landed here. RbtdrfSub.expect_ok:bool -> expect_code:i32 (ok/ok_cmd=0, fatal=RBTDGC_BAND_ENROLL, fatal_cmd takes explicit code); the one buv_report negative asserts RBTDRF_REPORT_NONZERO=1 (a report path returning bare 1, not a buc_reject gate); the inline multiscope BETA negative asserts band_enroll. rbtdrf_run_ev collapsed to one exact-code equality with stderr in the mismatch message. Build green under deny(warnings); fast suite 109/0/0 with EV fixture 47/47. Rust-only change, no shellcheck owed.

### 2026-06-15 09:50 - ₢BeAAD - n

Mint RBSTR-Terrier.adoc as a contract-first stub, homing the Terrier's settled write semantics as spec authority ahead of the M4 terrier pace. The Terrier is the federation's one durable, cloud-resident record of which citizen holds which mantle — manor-homed GCS bucket, one muniment (principal subject, mantle held) per object, read by rehearse and the reconciliation diff, written by the admission verbs. The settled decision the stub captures: writes are atomic via Cloud Storage itself, no Cloud Build and no external lock. Create carries ifGenerationMatch=0 (write only if absent; concurrent creators race cleanly to one winner, 412 losers treat an identical present muniment as idempotent success); update carries ifGenerationMatch=<generation> read beforehand (optimistic concurrency, 412 directs read-and-retry). Distinct muniments are independent objects so distinct governors never contend; the precondition covers the rare same-entry case. Each write is one conditioned REST call — within bash's good case under BCG, glue carrying no lock logic. Left explicitly open for the terrier pace: entry/object format and muniment JSON shape under a fresh terrier sprue, physical bucket name and constant home, and managed-folder grain plus per-polity write IAM (today's rbgb_ carries bucket-level IAM only). MCM integration — quoin registration in the RBS0 mapping section and the include:: into RBS0 — is deferred to that pace; the civic words (terrier, muniment, mantle, rehearse, admission verbs) stand as plain prose until the verb movement registers them. RBSTR registered in the RBK acronym map.

### 2026-06-15 09:44 - ₢BeAAD - n

Flip the enrollment-validation fixture's negatives from bare-nonzero to specific-band assertion, closing the wrong-reason hole. The gate was already taught by the regime conversion pace (buv_vet and buv_scope_sentinel buc_reject BUBC_band_enroll=101); the regime conversion's shareable form is the tweak-driven rbtdrs_poison harness, which does not fit this fixture's inline RbtdrfSub mechanism, so the expect-code plumbing lands here. RbtdrfSub.expect_ok:bool becomes expect_code:i32 — ok/ok_cmd expect 0, fatal expects RBTDGC_BAND_ENROLL (buv_vet's gate code), and fatal_cmd takes an explicit code. The single fatal_cmd negative drives buv_report, which is a report path (returns bare 1 by its documented contract), not a buc_reject gate, so it asserts RBTDRF_REPORT_NONZERO=1; the inline multiscope BETA negative now asserts band_enroll directly. rbtdrf_run_ev collapses to a single exact-code equality with stderr in the mismatch message. Build green under deny(warnings); live fast-suite run was blocked by the clean-tree gate — this notch unblocks it.

### 2026-06-15 09:27 - ₢BeAAC - W

Converted the regime-validation negatives from fabricated-file probate testing to in-universe poisoning. New regime-poison fixture (rbtdrs_poison.rs — its own per-fixture-cluster file, classifier 's', enrolled in service/crucible/complete, never fast) houses 32 negatives that drive the REAL validate verbs against REAL regimes with one field corrupted via the regime-poison tweak (BURE_TWEAK_NAME/VALUE as extra env on the validate Command), each asserting its gate's specific band code — RBTDGC_BAND_REGIME (100, module enforce) or RBTDGC_BAND_ENROLL (101, buv pipeline). Bands came from reading each module's enroll-vs-enforce split, not the census hypothesis (which mis-guessed timeout and cloud-prefix-too-long); every band was correct on first authorship. The six operator-local regimes (station/oauth/auth/node/privilege) use a baseline-probe helper that self-skips when the regime is unconfigured and never touches the operator's real secret files (the poison corrupts an in-memory var post-source). Tore out the entire probate path: the harness, all 32 negative + 13 baseline-valid fns, the baseline/prelude consts, RBTDRF_BUK_ROOT (528 lines from rbtdrf_fast.rs), and 24 RBTDRM_MODULE_/PROBATE_ manifest consts — the fast regime-validation fixture now holds only the three real-verb positives, every regime's green baseline being proven by regime-smoke and the poison fixture's own baseline probe. The full fast run then surfaced exactly the wrong-layer class this heat targets: a latent set-u crash in zbuv_poison_apply under the credless guard; fixed per BCG by having buv source bubc (formalizing the dependency buc_reject already needs) so the poison-name reference stays unguarded and set-u's typo backstop survives. Verdicts: fast green 109/0/0, regime-poison 32/32 standalone, no 'probate' survives in Tools/, shellcheck 212 clean, CLAUDE.md test-table guidance updated. The seam fix widened bubc's source-time footprint, exposing a pre-existing tinder-purity violation (BUBC_moorings_dir expands a runtime var); the clean restructure is slated as a follow-on pace in this heat.

### 2026-06-15 09:26 - Heat - S

bubc-moorings-tinder-relocate

### 2026-06-15 09:08 - ₢BeAAC - n

Test-table guidance for the regime-poison fixture: bumped the service/crucible/complete fixture counts (18/14/21) to include regime-poison, added a paragraph describing it as the in-universe negative-validation fixture (real verbs against real regimes, one field poisoned, specific band asserted) that rides above fast because fast reserves the tweak slot for the credless guard, noting operator-local cases self-skip when unconfigured; and changed the after-changes guidance so regime/validation changes run fast plus the regime-poison fixture.

### 2026-06-15 09:04 - ₢BeAAC - n

Fix a latent set-u crash in the regime-poison seam, surfaced by the full fast run once regime-validation was green again. The handbook-render fixture is credless, so it dispatches rbw-o with BURE_TWEAK_NAME set to the credless guard; zbuv_poison_apply's line-134 -n check passed, then line 135 dereferenced ${BUBC_tweak_regime_poison} unguarded — unbound on rbw-o's path, which does not source bubc — and crashed under set -u. The original 'unguarded on purpose, die loud' intent was wrong: it conflated any tweak on the slot with a poison, so the credless guard tripped it. Fixed per BCG rather than by guarding the reference (which would forfeit set-u's typo backstop, BCG line 215): buv_validation now sources bubc_constants.sh at module top, formalizing the dependency buc_reject already needs at runtime, so the poison-name reference stays unguarded (a typo still dies under set -u) and is always defined wherever buv is sourced. Verified safe: bubc's source-time ${BURD_CONFIG_DIR##*/} does not trip set-u when unset (yields empty, harmless where moorings is unused), so the buv-direct enrollment harness and other non-dispatch contexts still source cleanly. handbook-render and enrollment-validation pass via the single-case runner; shellcheck 212 clean. Full fast run next.

### 2026-06-15 08:50 - ₢BeAAC - n

Probate teardown — the Rust side of the side-door deletion. Removed the dead probate harness (rbtdrf_run_probate / rbtdrf_run_probate_in), all 32 negative case fns + the 13 baseline-valid positive fns + their per-regime _neg wrappers, every RBTDRF_*_BASELINE and the rbgc/bubc prelude consts, and the now-unused RBTDRF_BUK_ROOT (528 lines from rbtdrf_fast.rs). The fast regime-validation fixture now holds only the three real-verb positives (validate repo, all vessels, all nameplates) — the baseline-valid anchors retire because every regime's green baseline is now proven elsewhere in fast (regime-smoke renders+validates the real regimes; rbrr_nonempty_prefix sources+enforces rbrd) and the regime-poison optional helper baseline-probes the operator-local ones, so re-adding them would be non-load-bearing duplication. Stripped the 12 RBTDRM_MODULE_* + 12 RBTDRM_PROBATE_* manifest consts (no consumers remained) and the dead imports. Grep confirms no 'probate' survives anywhere in Tools/. Build green under deny(warnings); the three surviving positives pass via the single-case runner. Full fast + regime-poison runs next.

### 2026-06-15 08:33 - ₢BeAAC - n

Regime-poison negatives, operator-local batch (6 cases) — completes the 32-negative conversion. Resolved the operator-local fork to in-place poison + self-skip (not staged synthetic baselines): an empirical probe found station/oauth/auth/node/privilege all validate green on a configured workstation, which is exactly where the fixture's home tiers (service/crucible/complete) run, and the poison corrupts an in-memory variable in the validate subshell after the file is sourced so it never touches the operator's real secret files. New rbtdrs_poison_optional helper runs the baseline verb un-poisoned first: non-green exit means the regime is absent here and the case self-skips (the regime-smoke station precedent), while a green baseline also proves the poison is the only variable before the poisoned run asserts the band. Cases with bands grounded in each module's enroll-vs-enforce split: rbrs missing-platform / rbro missing-refresh-token / burp missing-workload-key reject at the buv presence gate (enroll 101); burn bad-platform at the buv enum gate (101); rbra bad-private-key (non-PEM clears the secret-length enroll, fails the zrbra_enforce BEGIN check) and burs bad-tincture (uppercase clears length, fails the zburs_enforce regex) reject in-module (regime 100). Folios: rbra a credential role, burn/burp the committed node. Builds green; live run next.

### 2026-06-15 08:26 - ₢BeAAC - n

Regime-poison negatives, folio-bearing in-tree batch (12 cases): rbrn against a real entry-enabled nameplate (8) and rbrv against real vessels (4). rbrn: seven reject in the buv pipeline (enum runtime/entry-mode/dns-mode/access-mode, ipv4 base-ip, presence moniker, sentinel bogus → enroll 101); port-conflict is the single module-enforce case — poisoning one field (workstation port to 10001, a valid port number that clears the buv port enroll but sits at/above the uplink minimum) trips the zrbrn_enforce cross-port check (regime 100), the single-var-poison reframing of the old four-field override since the tweak sets exactly one variable. rbrv: BIND_IMAGE and CONJURE_PLATFORMS are buv gated enrolls, so unsetting them on a mode-matching vessel rejects at the presence gate (enroll 101); conjure cases use the busybox vessel, the bind case the plantuml vessel, so the poisoned gated field is active. Added folio-moniker consts for the nameplate and conjure vessel referenced by 2+ cases. Builds green; live oracle run next.

### 2026-06-15 08:21 - ₢BeAAC - n

Regime-poison negatives, clean no-folio in-tree batch (12 cases): rbrr completion (bad-vessel-dir, bad-secrets-dir, runtime-prefix uppercase/no-trailing-hyphen/too-long), rbrd (missing-moniker, bad-moniker, cloud-prefix uppercase/no-trailing-hyphen/too-long), rbrp (bad-payor-project), burc (missing-station-file). Bands grounded by reading each module's enroll-vs-enforce split rather than trusting the census hypothesis: prefix-format regexes live in z*_enforce (band regime 100) while a prefix exceeding the buv_string_enroll max length rejects in the buv pipeline (band enroll 101) before the module's own checks run — so rbrd cloud-prefix-too-long is 101, not the joint-length 100 the census guessed, mirroring rbrr runtime-prefix-too-long. Unset-required cases (rbrd moniker, burc station-file) reject at the buv presence gate (101); existence/format/payor-regex checks reject in-module (100). Added a small const block for the three var names referenced by 2+ specs (RBRR_RUNTIME_PREFIX, RBRD_CLOUD_PREFIX, RBRD_DEPOT_MONIKER) and the buw-rcv BUK colophon (not projected into RBTDGC_*). Builds green under deny(warnings); live oracle run next.

### 2026-06-15 08:07 - ₢BeAAC - n

Rust half, vertical slice: scaffold the regime-poison fixture in a new file rbtdrs_poison.rs (its own per-fixture-cluster home, classifier 's' from poiSon since 'p' is pristine's — cupel-precedent for a non-first letter; keeps the non-fast fixture off the 'fast' classifier and off the already-2523-line rbtdrf_fast.rs per RCG File Size Discipline). The poison harness rbtdrs_poison drives a real validate verb against a real regime with one field corrupted via the regime-poison tweak (BURE_TWEAK_NAME/VALUE as extra env on the one tabtarget-launch constructor — slot free since the fixture is not credless) and asserts the SPECIFIC band code of the gate that fires. Two-case slice proving both bands through the real rbw-rrv against the tracked rbrr.env: bad-timeout (RBRR_GCB_TIMEOUT=1200 fails the zrbrr_enforce NNNs regex → RBTDGC_BAND_REGIME 100) and unexpected-var (RBRR_BOGUS trips buv_scope_sentinel → RBTDGC_BAND_ENROLL 101). Reading rbrr_regime.sh corrected the band-census hypothesis on the spot: timeout enrolls as a plain string, so its format check is module-enforce (100), not enroll (101). Minted RBTDRI_BURE_TWEAK_VALUE_KEY (symmetric with the name key) and RBTDRM_FIXTURE_REGIME_POISON; module wired in lib.rs; fixture registered in RBTDRC_FIXTURES and the service/crucible/complete suites (not fast, not the release ladders). Builds green under deny(warnings). Live band-oracle run was blocked by the clean-tree gate — this notch unblocks it; full negative set (33 cases across 12 regimes, incl. staged-baseline operator-local handling) and the fast-positive conversion + probate removal still owed.

### 2026-06-12 14:27 - ₢BeAAC - n

Bash half of the regime-validation in-universe conversion. Minted the regime-poison tweak (BUBC_tweak_regime_poison = buost_regime_poison, bubc tinder) and its seam: zbuv_poison_apply in buv_regime_enroll — the one BUK membrane every regime kindle crosses post-source pre-validate — with set ('VAR=value') and unset (bare 'VAR') semantics on the tweak channel, applied only when the variable carries the enrolling scope's prefix so a poison rides inert through a dispatch's host regimes and lands exactly once. Taught the two pipeline gates their band codes per the allocation rule (they co-occur in one verb spawn path, so they must differ): buv_vet and buv_scope_sentinel reject with BUBC_band_enroll; the regime modules' custom enforce rules (rbrr timeout/prefix/dir-existence — converted from buv_dir_exists to explicit in-module checks — rbrd moniker/prefix/joint-length, rbrn cross-port + ip-in-subnet, rbrp payor/billing/oauth formats, rbra role/email/PEM, burs tincture) reject with BUBC_band_regime. Deleted all twelve *_probate side doors (burc/burs/burn/burp + rbra/rbrd/rbrn/rbro/rbrp/rbrr/rbrs/rbrv). Projected the tweak name to theurge via a third rbcc_emit_consts section (RBTDGC_TWEAK_REGIME_POISON). Seam proven live: set/unset/inert behavior and vet exit 101 verified end-to-end in bash. Build green, unit 161/161, shellcheck 212 clean. Mid-pace state: the Rust conversion is next — the regime-validation fixture's probate-harness cases now reference deleted bash functions, so fast is knowingly red on that fixture until the new regime-poison fixture and the fast-positive rewiring land.

### 2026-06-12 13:52 - ₢BeAAI - W

Fast-tier credlessness is now structural. Both token-mint membranes (rbgo_get_token_capture, zrbgp_authenticate_capture) reject under the credless-guard tweak with buc_reject BUBC_band_credless as their first action — before any credential-file touch, so the verdict is identical on credentialed and bare machines; rba mint wrappers return $? so the band code survives laundering. Guard name single-homed as RBCC_tweak_credless_guard (buorb_credless_guard), codegen'd to RBTDGC_TWEAK_CREDLESS_GUARD. Guard scope settled per-fixture: rbtdre_Fixture.credless (true on exactly the ten fast-suite fixtures) arms a thread-local in rbtdrc_set_context and the env lands in rbtdri_tabtarget_command — the single Command constructor every launch uses, covering the direct-Command case helpers that bypass rbtdri_invoke — so fast fixtures stay guarded inside hosting suites. Loud invoke-level conflict gate enforces the BUS0 slot reservation (a guarded case may not set its own BURE_TWEAK_NAME). Proof case rbtdrf_rs_credless_guard_mint_refusal (regime-smoke) drives the real rbw-iJ registry-delete under the guard and asserts exit 104 end-to-end through launcher/dispatch, verified live on a credentialed workstation. Convention documented in the theurge authoring context. Unit 161/161, fast suite 154/154, shellcheck 212 clean, QualifyFast green.

### 2026-06-12 13:51 - ₢BeAAI - n

Made fast-tier credlessness structural. Minted RBCC_tweak_credless_guard (buorb_credless_guard, codegen'd to RBTDGC_TWEAK_CREDLESS_GUARD) and gated both token-mint membranes — rbgo_get_token_capture and zrbgp_authenticate_capture — with buc_reject BUBC_band_credless as their first action, before any credential touch, so a guarded run rejects identically on credentialed and bare machines; rba mint wrappers now return $? so the band code survives to the buc_die membrane. Theurge side: rbtdre_Fixture gains credless (true on exactly the ten fast-suite fixtures), armed thread-locally by rbtdrc_set_context and applied in rbtdri_tabtarget_command — the one Command constructor every tabtarget launch uses, covering the direct-Command case helpers — with a loud invoke-level conflict gate when a guarded case supplies its own BURE_TWEAK_NAME (the fast slot belongs to the guard, per BUS0 slot reservation). Guard scope is per-fixture, not per-suite: fast fixtures stay guarded inside service/complete. Proof case rbtdrf_rs_credless_guard_mint_refusal (regime-smoke) drives the real rbw-iJ registry-delete tabtarget under the guard and asserts exit RBTDGC_BAND_CREDLESS end-to-end through launcher/dispatch — verified live on a credentialed workstation. Three rbtdti unit tests cover arm/conflict/disarm; convention documented in the theurge authoring context. Unit 161/161, shellcheck 212 clean.

### 2026-06-12 13:30 - ₢BeAAB - W

Projected the bubc exit-code band into theurge as numeric consts: minted buz_emit_const_i32 (numeric sibling of buz_emit_const, all-digit guard) and extended rbcc_emit_consts with a second section emitting the eight BUBC band constants (base, width, five gate codes, selftest probe) via the same strip-uppercase transform — bubc arrives via the launcher so no cross-module tinder trick. Regenerated rbtdgc_consts.rs carries RBTDGC_BAND_* as i32 matching theurge's exit_code type. Build regenerates, QualifyFast freshness gate passes, shellcheck 212 clean, theurge unit tests 158/158.

### 2026-06-12 13:30 - ₢BeAAB - n

Projected the BUBC precision exit-code band into theurge as numeric consts. Minted buz_emit_const_i32 in the BUK zipper — the i32 sibling of buz_emit_const for values consumers compare as integers (process status lands as i32 on the Rust side), all-digit-gated since the band lives in 0-255 exit space. rbcc_emit_consts grows a second section applying the same mechanical strip-prefix/upcase transform to the eight BUBC_band_* values (base/width + five gate codes + selftest probe); bubc is already sourced by the launcher on every dispatch, so the values are present at emission time with no cross-module tinder trick. Regenerated rbtdgc_consts.rs carries RBTDGC_BAND_* (100/16 band, gates 100-104, selftest 115), giving theurge integer band assertions sourced from the single-homed shell constants instead of hand-copied literals.

### 2026-06-12 13:24 - ₢BeAAA - W

Established the precision exit-code band and its buc_die membrane. bubc band tinder: base 100 width 16 — placed above curl's exit range (1-92) and clear of sysexits, timeout/container codes, and signals — with per-gate codes (regime/enroll/recipe/hygiene/credless) and a band-top selftest probe (115). buc_die captures $? on entry and re-exits in-band values unchanged so existing cmd || buc_die sites propagate for free; buc_reject minted as the deliberate-rejection origin, loud on out-of-band codes. Survival proven end-to-end: buw-xb BandChain fixture raises the probe code beneath a command-substitution die chain and the new band-survival self-test fixture (6 cases) asserts it through the full tabtarget/launcher/dispatch exec path via new buto_unit_expect_code / buto_tt_expect_code assertions. Bonus catch: zbuto_invoke ran the argv inside its capture subshell, masking every function exit to 127 (the heat's wrong-reason hole inside the harness itself) — fixed with a nested subshell layer. buw-st 47/47, shellcheck 212 clean, fast 153/153.

### 2026-06-12 13:21 - ₢BeAAA - n

Established the precision exit-code band: bubc band tinder (base 100, width 16 — clear of shell-reserved, sysexits, curl, and timeout/container codes) with per-gate rejection codes (regime/enroll/recipe/hygiene/credless) and a band-top selftest probe. buc_die now captures $? on entry and re-exits in-band values unchanged (the band membrane — existing `cmd || buc_die` sites propagate for free); minted buc_reject as the deliberate-rejection origin helper, refusing out-of-band codes loud. Added buto_unit_expect_code / buto_tt_expect_code precision assertions and repaired zbuto_invoke's capture subshell so a function exit reports its real code instead of masking to 127. New buw-xb BandChain fixture command (bux_cli, zipper-enrolled, tabtarget) raises the selftest code beneath a command-substitution die chain; new band-survival self-test fixture (6 cases) proves membrane semantics and end-to-end survival through the tabtarget/launcher/dispatch exec path. buw-st 47/47 green; shellcheck 212 files clean

### 2026-06-12 12:58 - ₢BeAAH - W

Retired the buorb_graft_image tweak (doctrine violation: parameter injection on a positive path). Minted rbfl_anoint / rbw-rva / DirectorAnointsGraftVessel — reads the previous build's chained facts (hallmark/gar_root/ark_stem) and rewrites RBRV_GRAFT_IMAGE in a graft vessel's rbrv.env; yoke-precedent operator-committed, credless, warns on non-local image. Rewired the onboarding ccyolo kludge case to anoint graft-demo off its bottle kludge (real depth-1 chain, zero added builds) and the graft case to consume the committed slot with an anointed-witness probe; busybox pull and tweak injection deleted. Fossil graft-image value replaced with anoint-pending placeholder; CLAUDE.md tweak example repointed at buorb_ensconce_stamp. grep for the tweak returns zero; fast 153/153 green; shellcheck clean; graft-case chain verification rides the next gauntlet/skirmish run

### 2026-06-12 12:56 - ₢BeAAH - n

Set the executable bit on the anoint tabtarget — smoke-tested the dispatch end-to-end (workbench routing, rbfl kindle, clean no-arg refusal)

### 2026-06-12 12:54 - ₢BeAAH - n

Retired the buorb_graft_image tweak: minted rbfl_anoint (rbw-rva, DirectorAnointsGraftVessel) — reads the previous build's chained facts (hallmark/gar_root/ark_stem) and rewrites RBRV_GRAFT_IMAGE in a graft vessel's rbrv.env, yoke-precedent operator-committed. Removed the tweak read from rbfd_graft, rewired the onboarding ccyolo kludge case to anoint graft-demo off its bottle kludge and the graft case to consume the committed slot (busybox pull and tweak injection gone), replaced the fossil graft-image value with the anoint-pending placeholder, repointed the CLAUDE.md tweak-name example at the surviving buorb_ensconce_stamp

### 2026-06-12 12:22 - ₢BeAAG - W

Retired the buorb_immure_resolve_only tweak (doctrine violation: convenience short-circuit). Minted rbld_presage / rbw-lp / DirectorPresagesImmure — read-only Lode dry-run showing what immure would capture (family resolution + leaf selection, optional version renders full origins). Removed the seam from rbld_immure, repointed the fast podvm-resolve case at the presage colophon with an exit-status check replacing the seam-marker assertion. grep for the tweak returns zero; fast suite 153/153 green.

### 2026-06-12 12:19 - ₢BeAAG - n

Retired the buorb_immure_resolve_only tweak: minted rbld_presage (rbw-lp, read-only Lode dry-run showing what immure would capture — family resolution + leaf selection, optional version renders full origins), removed the short-circuit seam from rbld_immure, repointed the fast podvm-resolve case at the presage colophon with an exit-status check replacing the seam-marker assertion

### 2026-06-12 11:51 - Heat - d

paddock curried: groom: gate cleared, stale filenames refreshed, tier annotations completed across all paces

### 2026-06-12 09:22 - Heat - S

cupel-python-scan-split

### 2026-06-11 13:40 - Heat - d

paddock curried: Gate repointed: the rename pace restrung from Bb to BH tail; verify against BH now

### 2026-06-11 13:37 - Heat - f

racing

### 2026-06-11 13:19 - Heat - d

paddock curried: gestalt broadened at the 2026-06-11 groom: tweak doctrine + census cleanup + in-universe regime negatives join the band strand

### 2026-06-11 13:18 - Heat - S

fast-credless-guard

### 2026-06-11 13:17 - Heat - S

graft-chaining-installer-conversion

### 2026-06-11 13:17 - Heat - S

evict-immure-resolve-only-tweak

### 2026-06-11 13:16 - Heat - n

BUS0 Tweak Mechanism section now carries the tweak doctrine: a tweak forces one hard-to-produce condition for a test to observe handled correctly (condition-forcing only — parameter injection and convenience short-circuits are not tweaks); one tweak at a time per test/fixture/suite as deliberate design (sufficiency, not limitation — a dual need mints a second mechanism, never widens this one); a suite may reserve the slot for a standing guard as part of its tier definition. Spec example swapped from the doctrine-violating buorb_graft_image to the buorb_ensconce_stamp archetype. bure_regime.sh shape-enforce block points at the doctrine.

### 2026-06-10 12:02 - Heat - d

paddock curried: gate on Bb filename consolidation landing first

### 2026-06-10 11:55 - Heat - d

paddock curried: initial shape at nomination — cinched band design from negatives survey

### 2026-06-10 11:55 - Heat - S

bcg-precision-band-admission

### 2026-06-10 11:55 - Heat - S

migrate-recipe-and-hygiene-negatives

### 2026-06-10 11:55 - Heat - S

migrate-enrollment-validation-negatives

### 2026-06-10 11:54 - Heat - S

migrate-regime-validation-negatives

### 2026-06-10 11:54 - Heat - S

numeric-const-codegen

### 2026-06-10 11:54 - Heat - S

band-tinder-and-die-membrane

### 2026-06-10 11:53 - Heat - N

rbk-09-better-negatives

