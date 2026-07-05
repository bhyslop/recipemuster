# Heat Trophy: rbk-02-mvp-review-theurge-refactor

**Firemark:** ₣Bl
**Created:** 260622
**Retired:** 260705
**Status:** retired

## Paddock

## Character — the theurge-refactor consolidation stream

This heat is the theurge-crate refactor plus the zipper-colophon trio,
the foedus descry/instate REUSE-path code,
and the test-orchestration cosmology spec.
It is consolidation work — decided and dependency-sequenced — not a catalog of undecided ideas;
every band below names work whose shape is settled and whose order is a real dependency, not a loose priority.

The heat owns the crucible-module hotspot.
The substrate-lifecycle the suites stand on is already built — the freehold/depot substrate family plus the gauntlet ladder —
so the novel work is not building that machinery but lifting the implicit (inner-body × wrapper) suite encoding into a first-class combinator over the engine's existing setup/teardown hooks,
then collapsing the crucible god-module into a true crucible-lifecycle module plus a relocated registry home,
folding the duplicated test-module boilerplate,
and homing the test-orchestration cosmology as spec.

It also owns the zipper trio — the zipper source plus its two build-regenerated artifacts (the theurge colophon constants and the tabtarget command reference) — which travel as one three-file unit, never hand-edited apart.

Two bands of work.
The production-crate refactor: the engine combinator lift, the god-module decomposition, the registry extract, dead-code removal, the shared-invocation hoist, config-console routing, and the helper folds.
The spec-and-REUSE band: the cosmology subdoc, the two REUSE-path foedus toothings (descry and instate) as atomic production verbs, the REUSE-wrapper fixture, and the zipper/colophon relocation.

The single dominant tension is that the crucible module is the heat's hotspot:
nearly every pace writes it,
so paces here are not independent the way a junk-drawer's are —
the crucible-module decomposition is the spine the helper-folds and the registry-extract ride,
and the cosmology-spec work is downstream of the combinator (the combinator settles the abstraction the spec homes).

## Crucible-module hotspot — the shape the consolidation cinched

The crucible module is the heat's single most-written file,
and the partition pulled every writer into this one clone on purpose,
so the decomposition and the folds that ride it sequence in-tree rather than across heats —
including the read-side class-2 named-band negative case the partition transferred in from the MVP-loose-ends heat (₣Bi) so its crucible write is in-clone with the rest.

The load-bearing ordering is the god-module decomposition first.
Extracting the two framework-global registries — the fixture roster and the suite composition, kept co-located as a single hand-written source of suite/fixture truth — into a registry home,
along with the bare no-charge GCP-service fixtures,
leaves behind a true crucible-lifecycle module;
this extract is the structural predecessor of the cluster,
because every other crate pace edits the symbols it moves,
so the registry-relocation pace must bracket the cluster — run first or last, never concurrent with the writers that touch what it relocates.

Once the registries are relocated, two assertions pin them against drift:
the build-time suite-subset-of-roster check,
and — restored to the gauntlet and skirmish release ladders by operator ruling (260623) —
the credless feoff/yoke band-matrix fixture,
guarded by a reveille-base set-equality assertion so its membership cannot silently drift again as it did when it was quietly dropped from both ladders.

The helper hoists and the dead ifrit-invoke removal are independent cleanups,
but they share the file,
so they fold into the same in-clone sequence rather than racing it.

## Cosmology spec — supersedes this heat's native subdoc pace

This heat's original lone pace authored a skimpy RBS0 theurge-test-cosmology subdoc —
base quoins only, included into RBS0, with later inflation deferred.
The cosmology-finalization work draining in from the federation heat supersedes it,
homing the durable concepts the audit lifted —
the inner-body × wrapper combinator, the REUSE and LIFECYCLE wrappers, the three strata, and the freehold composite —
as a finished subdoc rather than a skimpy seat.
The native pace is wrapped or dropped as superseded;
its membrane list is the surviving pace's source, so the supersession loses no rigor.

Five membranes the subdoc honors, carried forward as the surviving pace's contract:
reference the BUS0 fixture/suite vocabulary, the dependency-tier axis, and the tweak mechanism rather than re-minting them;
point at the suite-composition registry for suite membership, never transcribing member lists;
cite the crucible-security specs rather than restating them;
split the identity-layers model so the permanent-versus-pool citizen fact stays production-side and only freehold-subject pinning is the subdoc's;
and thin the CLAUDE.md suites table toward a pointer.

Cinched on the subdoc itself:
it is a SUBDOC of RBS0, never a 0-suffixed sibling top-spec;
its quoin stem `rbtt_` (sub-letters read Theurge-Test) and the word "theurge" are Fable-ratified (260701) — no longer provisional;
the colliding docket-only testbench claim on the same stem in ₣BC (tabtarget-auditing, stabled) was reconciled at ratification: that testbench family re-mints at its own mount.
The canonical territory list names a second spec home alongside the cosmology subdoc;
the single-subdoc lean holds — the second file is provisional, split only if a conviction emerges, not asserted settled.

One characterization the subdoc must close by decision, not omission:
the federation authentic-verb suite/fixture word — the live reservation that names the unbuilt federation real-admission-verb fixture, owned by the federation heat (₣Bf) — must be either characterized in the cosmology among the strata or explicitly scoped out by a one-line note.
The gap closes either way; it does not close by silence.

## REUSE-path tabtargets — descry and instate

The premise these two verbs serve: the regime selects ONE active foedus at a time (RBRR_ACTIVE_FOEDUS), and a test points the selector at the standing foedus it runs against.
Under the 260630 one-pool Model (₣Bf paddock) the foedera CO-RESIDE — each is a PROVIDER under one manor pool, not its own pool.
The SPECS are re-cut to provider-grain (affiance adds a provider, jilt removes one);
the CODE re-cut is federation-stream-pending — that stream owns the manor-level pool regime relocation and the verb re-cuts (affiance, jilt, and descry's verdict vocabulary),
and this heat takes delivery through its one-pool barrier pace before the canvass-test weave and the parley fixture mount.
The foedus-lifecycle fixture's own provider-grain re-cut stays THIS heat's (the canvass-weave pace owns it — theurge-crate writes stay in this clone); the federation stream touches the crate only for the one-line poison-const safety re-point riding its regime relocation.
The muniment schema was a SECOND provider-grain delivery beyond the verb list above, and it is TAKEN (260702 merge):
the provider-qualified muniment (four-segment key, provider content field) merged through and converged with this heat's depot-attribution work into the four-column roll — (depot, mantle, provider, subject), depot from the key, record fields from content (RBSPO depot-attributed emission).
The parley fixture asserts the fully-attributed exact line and ran live green end-to-end on the merged tree
(the depot lens is what made the schema skew legible in the first place: attributed to the current depot, the aliasing roll line resolved to schema grain, not the cross-depot orphan first hypothesized).
descry/instate stay the per-foedus select-and-check toothings, operating provider-grain once that delivery lands.
The broader governor-selects feature stays premise-gated and deferred.

instate is the active-foedus switch toothing,
homed in the rbw-j family.
Its register is office: install the active foedus, one at a time, the prior stepping down —
re-point the regime selector and force a sitting reset (quash the live sitting, re-sign-in against the new foedus).

descry is the foedus-health check toothing (provider presence under the manor pool, per the 260630 Model),
rbw-jd, lowercase and read-only,
rhyming with Google's describe-pool;
its exact signature is settled upstream in the federation stream's foedus-reuse design pass — the build pace builds against the operation subdocs that pass authors, it does not re-choose the signature.

The one open hinge — casing — is settled upstream in that same design pass, not carried to the build pace:
it keys off whether the selector is committed config (uppercase, gated on a clean tree) or local runtime state (lowercase, gate-free).
The selection mechanism itself is settled there too; the build pace consumes the settled operation subdocs rather than re-litigating it.

Colophon homing for these verbs, settled this pass:
the foedus-cardinality verbs take rbw-j;
the foedus noun itself stays a folio with no colophon family — only its cardinality verbs take a colophon.
The casing convention across the family: an uppercase second letter marks an op that mutates cloud or critical manor state, lowercase marks read-only or non-mutating.
These lines govern the colophon-relocation pace, which rides the zipper baton.

The REUSE-wrapper fixture composes these two verbs with founding plus the credential heal —
the standing-freehold readiness step the release ladders lost when the keyfile-era credential-heal preamble was demolished.
The boundary is firm:
founding and dissolving a foedus stay the federation heat's verbs (Stream-FED);
this heat builds only the switch-and-check toothing on a standing foedus.

The read-all sibling canvass (rbw-jc) rounds out the foedus-cardinality family:
it lists every foedus the manor holds by interrogating the Manor's one pool (providers.list, per the 260630 Model), emitting depot_list-shape fact files for fixture chaining, and stays a standalone tabtarget — the Lode divine/augur split, never conjoined with descry.
Its operation sheaf is authored with descry's and instate's at the foedus-reuse design pass, so the cardinality set's contracts seat together and its build pace only consumes the sheaf.

## Provenance

These paces restrung in from two heats.
From ₣Bf: the theurge-crate refactor cluster, the colophon-relocation pace, and the cosmology pace —
along with the consolidation shape above (the combinator lift, the crucible hotspot, the chaining-fact-band restore with its reveille-base guard, and the descry/instate REUSE-path framing), rewritten here from the federation heat's audit-disposition record.
From ₣Bi: the one crucible writer escapee (the class-2 named-band negative case) pulled in so the crucible hotspot is single-clone.
The federation-lifecycle shape — the founding verbs, the foedus-reuse selection-mechanism design pass, the affiance-centric reuse-leg reframe — stayed in ₣Bf.

## Paces

### rehearse-depot-attribution (₢BlAAf) [complete]

**[260702-0314] complete**

## Character
Design conversation requiring judgment — federation read-surface architecture; likely touches federation-owned rehearse and/or terrier hygiene.

## The question
The polity admission verbs (brevet/unseat, zrbgp_*_core in rbgp_payor.sh) write depot-scoped — muniment key `<depot>/<mantle>/<subject>`, depot = RBDC_DEPOT_PROJECT_ID.
The sole exposed read, rehearse (rbw-pr -> rbgft_peruse_manor), is manor-wide and deliberately drops the depot, emitting `<mantle>\t<subject>`.
So no exposed read can witness a depot-scoped admission churn: a manor-wide roll cannot say which depot a (mantle, subject) line belongs to, and it collides across depots.
Decide the read surface for witnessing a depot-scoped admission change.

## Evidence (parley live run)
Against depot canest3bhm100002: the baseline roll showed retriever for the freehold subject; unseat retriever returned "Muniment already absent" (HTTP 404 — the current depot has none); the roll after unseat was unchanged.
The roll's retriever line is under a depot other than the current one — near-certainly an orphan from a retired freehold levy (freehold-churn unmakes the depot project, but the payor-level terrier bucket keeps its muniments, and churn runs no terrier sweep).
Once two depots hold the same (mantle, subject), the manor roll cannot distinguish the two identical lines.

## Options (from the discussion — not pre-decided)
Emit the depot in rehearse's roll (three-column) so a consumer can attribute and filter — small, but changes federation-owned rehearse output.
A depot-scoped read colophon (rbgft_peruse per-polity exists as an internal fn, no colophon) — a new verb.
Orphan-sweep as terrier hygiene on churn — then manor-wide rehearse equals the standing slice.
Reframe the positive-admission assertion off the roll (verb disposition) — abandons "rehearse's roll," parley's stated novel content.

## Done when
The read-surface question is ruled and recorded where it binds (RBSTC and/or the federation paddock), with the bounded fix landed or a clear follow-on slated — enough that parley can assert its unseat->restore churn through a lens that survives cross-depot collisions.

## Cinched
rehearse is the federation-owned read surface; terrier hygiene / orphan-sweep is federation-heat work, not parley's.

### thg-syncs-federation-rbs0-foedus (₢BlAAQ) [abandoned]

**[260624-2332] abandoned**

Cross-clone coordination gate for the theurge-refactor stream (its own repo clone).
No code change of its own — the standing sync contract for this stream's cross-clone waits, and the zipper baton it holds first.
Pace order within each heat is fixed and encodes every in-clone wait; this gate names only the CROSS-clone waits.

## Spec of needed change
This stream owns rbtdrc_crucible.rs and the zipper trio, and HOLDS the zipper baton first — ₢BlAAN and ₢BlAAE push the trio before ₣Bf takes it.
Its cross-clone WAITS are on ₣Bf, which owns RBS0-SpecTop and authors the foedus operation subdocs:
- Foedus spec-first: ₢BlAAE builds the descry/instate REUSE-path code against the two foedus operation subdocs and their RBS0 include lines that ₣Bf's ₢BfAAT authors contract-first. Build ₢BlAAE only AFTER ₢BfAAT has landed those subdocs; sync ₣Bf first.
- RBS0 cosmology region: ₢BlAAO adds an RBS0-SpecTop include and a cosmology region; rebase onto ₣Bf's RBS0 federation-block push (led by ₢BfAAO) before landing it — a disjoint region behind a single forward sync, never a concurrent edit. (₢BlAAO also depends in-clone on ₢BlAAB's combinator and supersedes the native ₢BlAAA, which is wrapped/dropped.)

## Done when
₢BlAAE built after ₢BfAAT's subdocs existed, and ₢BlAAO's cosmology region landed behind ₣Bf's RBS0 push with no federation-block conflict.

## Cinched
Coordination-only; this stream holds the zipper baton first, then hands to ₣Bf.

## Character
Cross-clone coordination gate; mechanical.

### create-skimpy-theurge-subdoc (₢BlAAA) [abandoned]

**[260625-0601] abandoned**

## Create a skimpy proper RBS0 SUBDOC for the theurge test-cosmology — base quoins only, include'd into RBS0, Fable inflates later.

The test-orchestration cosmology (theurge / fixture / case / suite / freehold / leasehold / cupel) has no spec home — it lives in code comments + CLAUDE.md tables + memos. Operator-settled: a proper RBS0 SUBDOC (include'd into RBS0, for free production-quoin reference), NOT a sibling top-spec — so it is named per the subdoc convention (RBS + tail, like RBSTR / RBSMA), NEVER a "0"-suffixed name (the "0" suffix is the top-spec convention).

Full scope + the five membranes + the placement reasoning: Memos/memo-20260622-fable-review-queue.md (the rbst0-theurge-subdoc item) and the theurge-spec-gap analysis it cites.

## Done when
A skimpy RBS0 subdoc (RBS+tail acronym, NOT "0"-suffixed) is include'd into RBS0, seating base quoins for the core test-cosmology nouns under a FRESH quoin stem (rbst_ is TAKEN — 14 production quoins at RBS0:492-506 — so a provisional, operator-sanctioned, eviction-sweepable stem; Fable later ratifies/sweeps the stem, the "theurge" word, and the asterism call). The five membranes hold: reference BUS0's busr_fixture/busr_suite + dependency-tier axis + tweak mechanism (do not re-mint); point at RBTDRC_SUITES for suite membership (never transcribe member lists); cite the crucible-security specs; split the identity-layers model (the permanent-vs-pool citizen fact stays production-side in RBS0/RBSRF, only the freehold-subject pinning is the subdoc's); thin the CLAUDE.md suites table toward a pointer.

## Cinched
A SUBDOC of RBS0 (not a sibling top-spec); skimpy (base quoins, Fable inflates later); a fresh quoin stem (not rbst_). One subdoc probably enough — split only if conviction emerges.

## Character
Spec authoring over operator-settled structure; the fresh-stem + theurge-word + asterism are provisional-now / Fable-ratified-later (carried in the Fable queue).

### freehold-wrapper-combinator-lift (₢BlAAB) [complete]

**[260626-0709] complete**

Drafted from ₢BfAAf in ₣Bf.

Lift the implicit (inner-body × wrapper) encoding into a first-class combinator over the engine's setup/teardown hooks, factoring the shared stand-up/teardown spines into rbtdrk helpers parameterized by REUSE versus LIFECYCLE — building on the already-built substrate machinery, not rebuilding it. The audit core.

## Done when
The nine hand-spelled suite fixture lists are expressible as wrapper(inner) compositions verified against the gauntlet membership matrix; both REUSE and LIFECYCLE semantics survive; gauntlet green.

## Cinched
The teardown post-state assertion is STRUCTURALLY INVERTED between tear_down (fail-closed allowlist) and churn (fail-open denylist) — if folded at all, the polarity is an explicit predicate, never a boolean relaxation flag; the lower-risk path leaves the two assertions un-merged.
Do NOT rebuild the substrate (rbtdrk_freehold/depot/lifecycle + gauntlet) — LIFT it.
This is the unblocked head of ₣Bl's order: it lifts already-committed substrate and consumes no cross-clone product, so it has no upstream barrier.
(The integrity audit struck three stale slate-carryover claims that previously sat in this Cinched block: the foedus fixture-to-verb seam is the LATER reuse-build pace's dependency on the design pace, not this combinator's; the payor-gate fold is LATER in this heat's order, not a predecessor of this pace; the terrier-scaffold retirement is the federation stream's work, not this pace's surface.)
Detail: the audit memo's pace-slate.

## Character
Design conversation requiring operator judgment; opus; mount with the operator present.

### theurge-crucible-module-split (₢BlAAC) [complete]

**[260626-1051] complete**

Drafted from ₢BfAAQ in ₣Bf.

Decompose the rbtdrc_crucible god-module:
extract the two framework-global registries (RBTDRC_FIXTURES, RBTDRC_SUITES + their lookups)
and the bare no-charge/quench GCP-service fixtures,
leaving a true crucible-lifecycle module behind.

## Refresh (post-rebase)
The rebase grew the file:
also relocate the new chaining-fact-livery bare fixture with its cluster,
and preserve the rbtdrh_chain cross-module registry references now in RBTDRC_FIXTURES and four suites.

## Settled at mount (260626) — design decisions, do not re-litigate
Symbols are renamed to their new module's prefix (RCG-clean), not name-preserved.
- rbtdra_almanac: the registry home — fixtures roll + suite composition + both lookups (RBTDRA_FIXTURES, RBTDRA_SUITES, rbtdra_lookup_fixture, rbtdra_lookup_suite).
- rbtdrv_patrol: the eleven bare cloud fixtures + their shared support cluster (ark basenames, BURV fact keys, GAR category, vessel dirs, the four docker_*/rekon helpers); rbtdrd_dogfight and rbtdro_onboarding repoint their imports to patrol.
- Crucible remainder keeps rbtdrc_: charge/quench, the context machinery (set/take/with_ctx stays here), the ifrit/sentry/dns/curl/srjcl/pluml helpers + cases, the four charge/quench fixture statics, darken_svg + tests.
- "patrol" accepted provisionally. Names were grep-gated: roster/rota/muster/census/cohort and assay/vigil/sortie/foray all failed (taken or ungreppable); almanac/patrol clean.
- Authorized addition beyond pure relocation: a compile-time duplicate-name guard — a const _ assertion over RBTDRA_FIXTURES and RBTDRA_SUITES that fails the build on a duplicate name field; fall back to an equivalent unit test if this toolchain's const &str comparison balks.

## Done when
rbtdrc_crucible holds only crucible-lifecycle concerns;
almanac + patrol hold the registries and the bare fixtures + shared cluster;
the duplicate-name guard is in;
rbw-tb build and rbw-tt tests are green;
suite composition and fixture behavior are byte-unchanged.

## Cinched
Relocation only beyond the named guard — do NOT collapse the suite membership lists (const-concat-is-cleverness stands).
Notch before testing.

## Character
Mechanical module relocation + one compile-time guard, fully compile-checked.

### reveille-base-set-equality-assertion (₢BlAAD) [complete]

**[260626-1159] complete**

Drafted from ₢BfAAV in ₣Bf.

Add a REVEILLE_BASE set-equality assertion pinning the canonical reveille set as a regression oracle across the dependency-tiered suites, and RESTORE chaining-fact-band to the gauntlet and skirmish ladders (operator ruling 260623 — it was dropped from both).

## Done when
The assertion fails the build on any reveille-base drift; chaining-fact-band rides gauntlet and skirmish; the defended verbatim reveille lists stay literal.

## Cinched
Do NOT collapse the reveille lists into a concat — the assertion guards the set, the lists stay literal. Detail: the audit memo's pace-slate.

## Character
Mechanical guard; the restore ruling is settled.

### await-fed-foedus-subdocs (₢BlAAR) [complete]

**[260627-1315] complete**

BARRIER — cross-clone wait. No code work of its own.
This pace cannot complete until ₢BfAAT (heat ₣Bf — the foedus-reuse design that authors the two foedus operation subdocs and their RBS0 include lines) is complete, pushed to origin, and origin merged back into this clone.
Only then may the next pace — the descry/instate REUSE-path build — proceed, because it builds against those subdocs (contract before code).

## Done when
₢BfAAT is wrapped and pushed to origin, and this clone has pulled origin so the foedus operation subdocs and their RBS0 includes are present here.

## Character
Cross-clone barrier; hold here until the named pace is merged through.

### foedus-reuse-leg (₢BlAAE) [complete]

**[260627-1452] complete**

Drafted from ₢BfAAg in ₣Bf.

Build the day-to-day REUSE path end to end on the INTERACTIVE foedus: enroll the active-foedus regime selector, then the foedus check (descry) and switch (instate) as first-class atomic production tabtargets per the design pace, then the REUSE fixture that composes them — check the foedus, switch to it if valid, heal its credentials (avow + don the mantles), and affiance only when the check fails.

## Spec of needed change
FIRST, enroll the regime field the switch reads and writes: RBSRR authored the RBRR_ACTIVE_FOEDUS selector and a Federation Selection group, but rbrr_regime.sh carries neither (spec/code gap). Add the field and the Federation Selection section to rbrr_regime.sh — a regime-validation change, so run reveille + regime-poison — before building the verbs that depend on it.
Then build the two new foedus toothings as clean standalone tabtargets, each atomic per the design pace — the pool-integrity check (descry) reads pool validity, the switch (instate) re-points the regime selector and resets the sitting; neither bundles create. Then build the REUSE-wrapper fixture that composes them with the existing affiance and the credential heal. This is the standing-reuse credential leg that skirmish/dogfight/blockade assume but no fixture establishes today.

## Done when
RBRR_ACTIVE_FOEDUS + the Federation Selection group are enrolled in rbrr_regime.sh (spec'd in RBSRR, absent in code today); the check and switch exist as first-class atomic tabtargets; the REUSE wrapper reuses a valid standing freehold (the interactive Entra foedus) with no pool churn (cap-flat), heals its credentials via the existing avow + don, and rebuilds only when the check fails; skirmish/dogfight/blockade prove their mantle credentials via a fixture, not prose.

## Cinched
The verbs are ATOMIC and first-class — standalone production tabtargets, never a fat check-then-create, never buried only inside the fixture; "reuse if valid, else create" lives in the fixture. The credential heal is quota-neutral (idempotent IAM); affiance fires only on check-failure.
SCOPE — interactive only (verified against the code 260623): the standing freehold runs on the interactive Entra foedus, so the heal uses the existing interactive avow + don and is human-present (one avow at suite head, then reuse cache-hits the live sitting). The switch verb builds but is DEGENERATE until a second foedus stands (only Entra exists today). Fully-unattended / cross-mechanism (Entra<->Keycloak) reuse is OUT of scope here — a later pace gated on the programmatic spine (programmatic affiance arm, Keycloak orchestrator, programmatic accessor). Do NOT chase the unattended path here; it would hit unbuilt programmatic code.
The field-enrollment depends only on RBSRR (already landed), not on this pace's barrier — but it is folded here because the switch/check verbs cannot build without it (operator decision 260627: fold, not a separate micro-pace).
descry and instate are built against their own operation subdocs (authored contract-first at the design pace) — no exemption (operator 260624).
Detail: the audit memo's pace-10.

## Character
Bash build — the regime-field enrollment plus two production tabtargets plus the composing fixture; depends on the design pace and the combinator. No dependency on the programmatic spine (interactive machinery only).

### rbtdth-shared-test-helpers (₢BlAAF) [complete]

**[260630-0907] complete**

Drafted from ₢BfAAd in ₣Bf.

Enrich rbtdth_helpers with two hoisted helpers: a registration-triplet assert pair (disposition + cases) folding the lookup/projection idiom across rbtdtk/rbtdtp/rbtdto/rbtdtl, and a scratch-dir maker folding the triplicated make_temp across rbtdte/rbtdti/rbtdtl.

## Done when
The ~150 lines of test boilerplate collapse to call sites; the rbtdt* unit tests green.

## Cinched
disposition is a PARAMETER, never a constant — calibrant's three Independent fixtures must survive; the scratch maker uses the strongest pid+nanos+preclean recipe; leave rbtdtk's deliberately-nonexistent path and rbtdti's tt-staging alone. Detail: the audit memo's gap-closure addendum.

## Character
Mechanical test-mirror hoist, order-independent.

### registration-mirror-to-const-guard (₢BlAAZ) [complete]

**[260630-0925] complete**

The per-fixture registration unit-tests — the `*_disposition_*` and `*_cases_registered` mirrors in rbtdtk/rbtdtp/rbtdto/rbtdtl —
re-enter the registry by name-string lookup and restate the roster as hand-typed literals.
That is a weaker second paradigm than the almanac's compile-time const-guards (`const _: () = assert!(…)` over the single declarative source in rbtdra_almanac.rs).
Collapse the first into the second: assert each pinned fact at the fixture declaration site against the fixture static by symbol, and delete the runtime mirror.

The case-NAME literals go entirely —
the compile-checked `case!(ident)` list already proves each case is a real function, so only the count is non-redundant (`assert!(FIXTURE.cases.len() == N)`).
Disposition, where it earns pinning, becomes `assert!(matches!(FIXTURE.disposition, …))` — `matches!`, never `==` (derived PartialEq isn't const).

## Done when
The per-fixture disposition/cases mirror tests are gone, their surviving signal expressed as const-asserts at the fixture declarations;
genuinely-semantic runtime tests (the onboarding conclave-ordering test) stay;
theurge build + unit tests green.

## Cinched
No fn-pointer comparison anywhere — it is optimization-dependent and lint-flagged (`unpredictable_function_pointer_comparisons`); websearch-settled in the slating chat.
Per-fixture disposition correctness is already tested generically in rbtdte_engine, so those asserts are restatement — delete unless a declaration-site const-assert earns its keep.

## Open
Before deleting the name lists, confirm whether the bash blackbox testbench consumes these case names as an external contract (the rbtdtl header hints at it);
if so, guard the Rust registry against the bash side's real list, not a third hand-typed copy.

## Character
Architectural paradigm-collapse, judgment per fixture — not a mechanical sweep.

### ifrit-invoke-dead-code-removal (₢BlAAG) [complete]

**[260630-0944] complete**

Drafted from ₢BfAAc in ₣Bf.

Remove the dead rbtdri_invoke_ifrit and its duplicate ifrit-binary const, leaving the live rbtdrc_invoke_ifrit pair as the single ifrit-invoke home.

## Done when
One ifrit-invoke home and one rbid const remain; the crate builds; bivouac suite green.

## Cinched
Verify zero external callers before removing; rbtdri_parse_ifrit_verdict and the rest of rbtdri_invocation stay live. Detail: the audit memo's pace-slate.

## Character
Surgical dead-code removal.

### ordain-capture-helpers-to-shared-home (₢BlAAH) [complete]

**[260630-1056] complete**

Drafted from ₢BfAAW in ₣Bf.

Hoist rbtdro_ordain_capture (single + three-fact), the invoke-or-fail wrapper, and the GAR-locator builders out of rbtdro_onboarding into a shared home (rbtdri_invocation is the natural fit), then route the open-coded copies in dogfight and the crucible bodies through them.

## Done when
No crucible/dogfight body open-codes the ordain+fact-read or invoke-error shape; picket and dogfight suites green.

## Cinched
The TWO GAR-locator shapes (category-rooted wrest_locator vs fact-rooted image_ref) must NOT merge — keep two builders or none. Detail: the audit memo's pace-slate.

## Character
Coverage-neutral lift; the helpers are already proven in onboarding.

### lode-roundtrip-block-helpers (₢BlAAI) [complete]

**[260630-1823] complete**

Drafted from ₢BfAAX in ₣Bf.

Extract the byte-near-identical invariant blocks shared by the four Lode round-trip case bodies (touchmark read-back, divine-contains, banish-and-verify-gone) and the two-site member-jettison block into within-module helpers.

## Done when
The four Lode bodies call the shared blocks; the six load-bearing per-kind differences stay inline; the four Lode fixtures green standalone.

## Cinched
Do NOT widen any helper to absorb the six exclusions; do NOT touch lode-collision or chaining-livery. Detail: the audit memo's pace-slate.

## Character
Within-file mechanical extraction, independent of the wrapper lift.

### registry-drift-guard-and-heal (₢BlAAJ) [complete]

**[260630-1321] complete**

Drafted from ₢BfAAU in ₣Bf.

Close the discovery-vs-suite registry drift: register foundry-path in RBTDRC_FIXTURES (it rides six suites but is unresolvable by name), and add a build-time assertion that every suite-referenced fixture is roster-resolvable.

## Done when
FixtureRun resolves foundry-path; the invariant fails the build if any suite gains a roster-less fixture; reveille suite green.

## Cinched
Assert suite ⊆ roster ONLY, never the reverse — foedus-lifecycle / freehold-churn / calibrant-* are intentional roster-only members of no suite.
(The integrity audit struck a false "rbtdgc_consts.rs + tabtarget-context.md regenerate, yours to commit" claim that previously sat here: those build-generated files derive ONLY from zipper/colophon edits, and this pace registers a Rust fixture + adds a test — it touches no colophon, so nothing regenerates.)
Detail: the audit memo's pace-slate (Memos/memo-20260623-theurge-test-consolidation-audit.md).

## Character
Mechanical heal plus one guard.

### drive-hallmark-through-config-console (₢BlAAK) [complete]

**[260630-1222] complete**

Drafted from ₢BfAAa in ₣Bf.

Route rbtdro_drive_hallmark through the engine config-console (rbtdre_config_set_field) for rbrn.env, as its rbrv.env sibling already does, retiring the hand-rolled find-replace-rename.

## Done when
drive_hallmark delegates to the config-console; atomic-rename semantics unchanged; onboarding-sequence green.

## Cinched
The find-or-err schema-drift catch is preserved by routing through config_set_field, not lost. Detail: the audit memo's pace-slate.

## Character
The cleanest route-through-the-shared-home item.

### onboarding-reliquary-probe-helper (₢BlAAL) [complete]

**[260630-1339] complete**

Drafted from ₢BfAAY in ₣Bf.

Collapse the seven byte-identical reliquary-touchmark precondition-probe preambles in the onboarding cases into one assert helper.

## Done when
The seven preambles become one helper; onboarding-sequence green; the standalone kludge-tadmor path stays probe-free.

## Cinched
At the graft-demo case replace only the reliquary preamble; leave the standalone path's deliberate probe-omission alone. Detail: the audit memo's pace-slate.

## Character
Trivial within-file fold, coverage-neutral.

### payor-gate-helper-richer-signature (₢BlAAM) [complete]

**[260630-1051] complete**

Drafted from ₢BfAAZ in ₣Bf.

Extract the triplicated payor-credential probe-and-gate preamble (terrier-scaffold, terrier-atomicity, foedus-lifecycle) into one helper whose signature reproduces all three verdicts verbatim.

## Done when
All three gates route through the helper; the foedus Fail still dumps stdout/stderr verbatim; the two terrier Skips still skip; picket suite green.

## Cinched
The naive (fixture_name + Skip|Fail) signature is INSUFFICIENT — it drops the foedus stdout/stderr dump; carry per-policy templates or return the probe Result. Detail: the audit memo's pace-slate.

## Character
Real dedup gated on a richer-than-naive signature.

### qualify-colophon-check-revive (₢BlAAT) [complete]

**[260630-1059] complete**

Revive rbq_qualify's dead colophon-freshness check — it currently validates ZERO tabtargets, so an unregistered or stale colophon passes qualify silently.

## Why this pace exists
The heat-integrity audit observed that rbq_qualify's tabtarget/colophon-freshness check is a no-op: live qualify logs "Checked 0 RBW tabtargets." That check is meant to catch a tabtarget whose colophon (parsed from its filename) is unregistered in the zipper or stale against the regenerated context — exactly the failure mode the colophon-relocation pace's renames could introduce. With the count at zero, that safety net is silently down, which is why the colophon-relocation docket currently has to say "verify by dispatch, not by qualify." This pace restores the net.

## Spec of needed change
Re-derive the cause from the live rbq_qualify.sh scan — the audit's suspected cause is a launcher-path-match bug (the scan's tt/ discovery or its launcher-path predicate matching nothing), but confirm it against the code rather than trusting the diagnosis.
Repair the scan so it enumerates the real tt/ tabtarget population and checks each one's colophon against the zipper registry (the rbz-derived rbtdgc consts / tabtarget-context), failing loud on an unregistered or stale colophon.

## Done when
rbq_qualify's colophon-freshness check enumerates the real tt/ tabtarget population (not zero); a deliberately-mis-registered tabtarget is CAUGHT (prove the net actually fires, do not just assert a nonzero count); rbw-tl / rbw-tr qualify green on the clean tree.

## Cinched
Standalone qualify-infrastructure repair surfaced by the integrity audit — NOT part of the theurge-crate refactor or the zipper relocation, but homed here because the colophon check ties qualify to the zipper trio this heat owns.
Sits before the colophon-relocation pace so that relocation's tabtarget renames regain a working qualify net — the relocation does not HARD-depend on it (it can verify by dispatch), but a live check is the right safety net for it.

## Character
Investigation-then-mechanical: confirm the zero-count cause from the live scan, repair the match, prove the net fires. Independent of the crucible / zipper refactor work.

### colophon-relocation-260624 (₢BlAAN) [complete]

**[260630-1410] complete**

Drafted from ₢BfAAj in ₣Bf.

Apply the settled 260624 colophon relocations the vocabulary sweep left undone — the verb-level rename (compear→avow, assize→sitting) swept tree-wide, but the colophon half is orphaned and the tree still contradicts the decision.

## Spec of needed change
Two settled moves (paddock Foedus-accessor-vocabulary section, 260624):
Collapse the access family rbw-ac → rbw-a — rbw-aa avow, rbw-ap payor-credential check.
Move gird rbw-pE → rbw-mG, so rbw-m is the payor founding trio (affiance / jilt / gird) and rbw-p stays purely governor.
Edit rbz_zipper.sh (the enrollment home), then build to regenerate rbtdgc_consts.rs + claude-rbk-tabtarget-context.md (both build-generated — yours to commit), and repoint the affected citations (claude-rbk-acronyms.md, and gird's spec RBSPG if it cites the colophon).

## Full edit set (the integrity audit found the original list omitted the runtime-load-bearing parts)
- RENAME the tabtarget FILES in tt/ (rbw-acp / rbw-acf / rbw-acm / rbw-pE → their new colophons), because a tabtarget's colophon is parsed from its FILENAME at dispatch — an unrenamed file routes the wrong launcher.
- CAUTION: rbq_qualify does NOT catch a missed rename — live qualify logs "Checked 0 RBW tabtargets" (its colophon-freshness check is a dead no-op, a launcher-path-match bug). The renames are RUNTIME-load-bearing, not qualify-enforced; do not trust "qualify green" to prove them. (That qualify dead-check is a separate latent defect, flagged for its own disposition.)
- Clean the stale-by-name colophon references the rename leaves behind, which qualify also will not catch: comments / strings naming rbw-acf / rbw-acm in rbgp_payor.sh, and doc-comments / Fail() strings naming rbw-pE / rbw-acf / rbw-acm in rbtdrk_depot.rs (grep them).

## rbw-acm (mantle-access probe) — destination resolved
The 260624 note said "relocate the mantle-access probe to the test bucket," but the test bucket rbw-q does NOT exist yet (it is FED-minted downstream, and this pace explicitly disclaims minting rbw-q) — so the original instruction self-contradicted.
Resolution: rename rbw-acm → rbw-am into the collapsed rbw-a access family NOW (mechanical, in scope); DEFER the move into the test bucket to whenever rbw-q is minted, rather than strand rbw-acm under the retired rbw-ac prefix. (Operator may override the interim home.)

## Done when
The tree matches the settled homing — rbw-a (incl. the interim rbw-am) / rbw-mG enrolled, rbw-ac / rbw-pE gone, the tt/ files renamed to match; the generated files re-derive clean; acronym + spec citations repointed; the stale-by-name refs in rbgp_payor.sh / rbtdrk_depot.rs corrected. Verify by dispatch, not by rbq_qualify, until the qualify colophon-check is repaired.

## Cinched
Settled decision (operator 260624) — naming is not reopened; this is mechanical execution.
The casing convention holds (uppercase second letter marks an op that mutates cloud/critical state).
Only the already-shipped rbw-ac family and the gird move are in scope; the new-colophon foedus / test moves stay with their build paces; the rbw-acm test-bucket move defers with rbw-q.

## Character
Mechanical colophon relocation via the zipper; verify by dispatch (rbq_qualify's colophon check is currently a no-op).

### await-fed-rbs0-block (₢BlAAS) [complete]

**[260701-1246] complete**

BARRIER — cross-clone wait. No code work of its own.
This pace cannot complete until ₢BfAAv (heat ₣Bf — the 260630 one-pool spec re-cut that re-cuts the RBS0-SpecTop federation block to foedus≡provider; retargeted from the stale ₢BfAAO, which seated the OLD block ₢BfAAv now re-cuts) is complete, pushed to origin, and origin merged back into this clone.
Only then may the next pace — the theurge-cosmology subdoc finalize — proceed, appending its cosmology region as a disjoint region behind FED's RE-CUT federation block rather than racing the re-cut.

## Done when
₢BfAAv is wrapped and pushed to origin, and this clone has pulled origin so the re-cut RBS0 federation block is present here.

## Character
Cross-clone barrier; retargeted 260630 from ₢BfAAO (the old block) to ₢BfAAv (the re-cut), so the cosmology appends behind the current federation block.

### theurge-cosmology-subdoc-finalize (₢BlAAO) [complete]

**[260701-1311] complete**

Drafted from ₢BfAAe in ₣Bf.

Finalize the RBS0 theurge-test-cosmology subdoc, homing the durable concepts this audit lifted — the inner-body × wrapper combinator, the REUSE/LIFECYCLE wrappers, the three strata, the freehold composite — as spec rather than memo, superseding this heat's former native cosmology-spec pace (₢BlAAA, dropped as superseded in the split-study restitch — its docket survives as this pace's rich source).

## Parley characterization (surfaced by the heat-integrity audit)
Either name the parley authentic-verb federation suite/fixture in the cosmology — its sub-register among the strata — or explicitly scope it out with a one-line note, so the characterization gap closes by decision rather than omission.

## Done when
The cosmology is spec-homed in a new RBS-tail subdoc include'd into RBS0 (a fresh quoin stem, not rbst_); the wrapper/strata/composite vocabulary is citable from code; the parley suite/fixture is either characterized or explicitly scoped out; the vow + adoc build is green.
(The former native cosmology-spec pace ₢BlAAA was already dropped as superseded in the restitch — there is no separate heat to retire; this heat IS the active theurge-refactor stream.)

## Cinched
A SUBDOC of RBS0, not a sibling top-spec; honor the five membranes (reference BUS0 busr_fixture/busr_suite, point at RBTDRC_SUITES, cite the crucible-security specs, split the identity-layers model, thin the CLAUDE.md table).
Depends in-clone on the combinator having settled the abstraction — the combinator pace precedes this one in this heat's order, so the dependency is in-clone ordering, not a cross-clone wait.
Detail: the audit memo plus the dropped native pace's (₢BlAAA's) membrane list, its rich source.

## Character
Spec authoring; depends in-clone on the combinator.

### read-existence-failure-bands (₢BlAAP) [complete]

**[260701-1440] complete**

Drafted from ₢BiAAb in ₣Bi.

## Character
Mechanical extension of the read-side named-band discipline to a second failure class, woven into existing cloud fixtures.

The read verbs (summon/plumb/augur) reject a second user-controlled cause beyond the local chaining resolve: the user names an artifact that is absent or in the wrong state in the registry — knowable only after a round-trip.
These die generic (buc_die) today.

Band mechanics (from the ₣Be Fable review, `Memos/memo-20260701-fbl-be-negatives-review.md`):
mint the new code in the bubc sole-mint block at the next free slot as read at mount — the block is the registry;
₢BoAAB is slated to take 109, so expect 110 —
and extend the hand-kept band emit list in `rbcc_emit_consts` (rbcc_constants.sh) so the `RBTDGC_` const projects,
then rebuild via `tt/rbw-tb.Build.sh` (a new band code does NOT auto-flow to Rust; the miss is a loud compile error).

## Discovery
grep buc_die across rbfr_retriever.sh, rbfcp_plumb.sh, rbldl_lifecycle.sh.
The Class-2 sites are the absent / wrong-state ones — hallmark absent (summon; plumb vessel-resolve), Lode absent and Lode-unvouched (augur) — distinct from the local chaining resolve and the infra dies (auth / network / integrity), which stay buc_die.

## Coverage — deft insertion, no new fixtures
The cloud lifecycle fixtures already stand up the artifacts; add negatives at their natural absent moments — summon/plumb a hallmark before ordain or after abjure (hallmark-lifecycle / dogfight), augur a touchmark after banish (lode-lifecycle).

## Done when
- Each Class-2 user-controlled failure rejects with a named band.
- A negative case asserting that band by exit code rides an existing cloud fixture — no new fixture stood up.
- The new band projects via the emit list (`RBTDGC_` const consumed by the negative cases, never a re-literaled code).
- The infra dies are left as buc_die.

## Mount-time decisions
- Whether augur's unvouched-Lode case earns a band or stays buc_die (it blends user cause with capture integrity; lean, not cinched: unvouched reads as capture integrity → buc_die).

## Cinched
Tests assert the named exit code, never output text. Coding errors stay flushed-out statically, never named runtime bands.
A NEW absent-artifact band, never a reuse of the chaining band: plumb's spawn path crosses the vessel-resolve chaining gate, and the allocation rule forbids sharing a code along a pipeline.

### await-fed-onepool-recut (₢BlAAa) [complete]

**[260701-1509] complete**

BARRIER — cross-clone wait. No code work of its own.
This pace cannot complete until ₢BfAAv (heat ₣Bf — the 260630 one-pool spec re-cut: RBSRF / RBSMA / RBSFD / RBSPB + the canvass operation sheaf + the RBS0 federation civics) is complete, pushed to origin, and origin merged back into this clone.
The foedus-cardinality build paces below — canvass build/test, and any descry/instate code that consumes the re-cut sheaves — would otherwise build against the pre-260630 (per-foedus-pool) contract.

## Done when
₢BfAAv is wrapped and pushed to origin, and this clone has pulled origin so the re-cut federation subdocs + the providers.list canvass sheaf are present here.

## Character
Cross-clone barrier (₣Bf → ₣Bl); hold here until the named pace is merged through.

### foedus-canvass-build (₢BlAAU) [complete]

**[260701-1613] complete**

Build canvass — the read-only foedus enumeration verb, the read-all sibling of descry's read-one: list every foedus the manor holds by interrogating the Manor's one workforce pool (providers.list). RE-CUT 260630: under the one-pool Model (₣Bf paddock) foedera are PROVIDERS under one manor pool, so this enumerates providers, not pools (was workforcePools.list).

## Spec of needed change
canvass follows the depot_list shape (RBSDL), not a bare human dump: it enumerates the manor's foedera and emits machine-readable facts for programmatic consumers — the foedus fixtures chain on them.
Payor-credentialed: listing the org pool's providers is the affiance/jilt org-level authority; depot mantles cannot reach it.
Call providers.list against the one manor pool's provider collection (the URL affiance constructs when it adds a provider) — verify list parent-scoping against the live contract.
Emit one fact file per foedus keyed on a new RBCC_fact_ext_foedus extension into BURD_OUTPUT_DIR (provider id, provider state, regime-selected flag), then a typed-output summary; an empty manor reports zero, non-fatal.
The foedus↔provider correlation (mark the regime-selected one): match provider ids/displayNames against the rbef_ library's RBRF_PROVIDER_ID values, or have affiance stamp the rbef_ name into the provider — settle at build (the Model's open canvass→rbef_ mapping detail).
Home it with the workforce-pool REST siblings, dispatch via the payor CLI, add the rbw-jc colophon to the zipper.

## Done when
canvass is a first-class atomic tabtarget (rbw-jc), payor-credentialed and read-only, emitting per-foedus fact files (RBCC_fact_ext_foedus) plus a typed-output summary that marks the regime-selected foedus, non-fatal on an empty manor — built against the re-cut canvass operation sheaf (providers.list).

## Cinched
Naming settled (₣Bf census): rbw-jc / rbtf_canvass / foedus-cardinality; lowercase = read-only.
Output follows depot_list (RBSDL): fact files + typed-output summary + non-fatal empty.
260630 Model: enumerate PROVIDERS under the one manor pool (providers.list), not pools; consumes its re-cut operation sheaf (the ₣Bf spec re-cut), does not re-choose the signature.

## Character
Bash build — one read-only payor verb plus its fact-ext constant and zipper colophon; a one-method extension of affiance's provider-collection-URL machinery against the re-cut sheaf.

### colophon-completeness-check (₢BlAAb) [complete]

**[260701-1731] complete**

Invent a strict colophon-completeness check — a source-repo-only tabtarget that FAILS if any enrolled colophon lacks its tabtarget — so the source repo's test sequence proves completeness while a delivered subset repo simply never runs it. Replaces the BURC strict/lax config approach (operator 260630).

## Spec of needed change
The delivery split (RB ships code but WITHHOLDS internal accelerator tabtargets — the force-delete, the populate-citizens, this check itself) needs the source repo to catch a genuinely-missing tabtarget while the consumer/proof subset repo does not fail.
Rather than the stateful BURC_BUW_COLOPHON_CHECK = bvc_strict / bvc_lax config (which marshal-zero would have to flip), do it with a dedicated tabtarget:
- A check tabtarget that asserts every enrolled colophon has its tabtarget file present; fails loudly otherwise. Runs in the source repo's release-qualify / echelon sequence.
- It is itself withheld from delivery (source-only), so a subset consumer never runs it and never trips on its own withheld accelerators.
- This is a WORKING completeness check — the current rbq_qualify colophon-freshness check is a known no-op (the launcher-path-match bug ₢BlAAN flags, "Checked 0 RBW tabtargets"). Evaluate retiring BURC_BUW_COLOPHON_CHECK once this lands (grep its consumers first).
- No marshal-zero lax step — there is no config to flip.

## Done when
A source-only tabtarget fails on any enrolled-colophon-without-tabtarget and rides the source release-qualify sequence; the consumer/proof subset repo does not run it and passes; BURC_BUW_COLOPHON_CHECK retirement is evaluated (retired if no other consumer).

## Cinched
A tabtarget, not a config toggle (operator 260630) — no per-context stateful flip.
Source-repo-only (withheld from delivery), like the force-delete + populate-citizens accelerators.

## Character
Bash check tabtarget over the colophon registry; the BURC_BUW_COLOPHON_CHECK retirement is a small regime cleanup gated on a grep. Self-contained ₣Bl (no new cross-heat barrier).

### rekon-chaining-band-case (₢BlAAW) [complete]

**[260701-1745] complete**

## Character
Small mechanical test addition — extend an existing matrix, no new apparatus.

## Scope
Add rekon (`rbfl_rekon_hallmark`, colophon `rbw-irh`) to the read-consumer
chaining-fact-band matrix in `rbtdrh_chain.rs`, inline with the existing
summon / plumb / augur cases:
assert a broken express-or-chain resolve rejects with `BUBC_band_chain`,
with the same fact-intact / no-durable-write checks the sibling read cases make.

## Dependency
Asserts a conversion shipped in ₣Bi:
rekon's broken-resolve moved from `buc_die` to `buc_reject BUBC_band_chain` in `rbfln_inventory.sh`.
Mount after that lands — rekon must already band-reject for the case to pass.
May fold into an existing ₣Bl band pace if one fits.

## Done when
Rekon carries a passing band case in the read-consumer matrix asserting
`BUBC_band_chain` on a broken resolve, peer to summon / plumb / augur.

### await-fed-onepool-verbs (₢BlAAe) [complete]

**[260702-0107] complete**

BARRIER — cross-clone wait. No code work of its own.
This pace cannot complete until ₢BfAA1 (heat ₣Bf — the affiance-jilt provider re-cut) is complete, pushed to origin, and origin merged back into this clone.
By ₣Bf heat order, ₢BfAA1 rides behind the RBRW regime buildout (₢BfAA0), the membership-composer re-cut (₢BfAAw), and the manor finisher (₢BfAAF), so its merge transitively delivers the whole one-pool code substrate.
The next paces — the canvass-test weave (which re-cuts the foedus-lifecycle fixture to provider grain) and the parley authentic-verb fixture (which needs the finisher-provisioned terrier and the re-cut principal shape) — require that substrate standing here before they mount.

## Done when
₢BfAA1 is wrapped and pushed to origin, and this clone has pulled origin so the one-pool verb code (and its ₣Bf predecessors by heat order) is present before the canvass-test and parley fixtures mount.

## Character
Cross-clone barrier (₣Bf → ₣Bl hand-back of the one-pool code re-cut); hold here until ₢BfAA1 is merged through.

### foedus-canvass-test (₢BlAAV) [complete]

**[260702-0122] complete**

Re-cut the foedus-lifecycle round-trip to provider grain (consuming the landed ₣Bf one-pool verb code), then weave canvass into the live window and assert its fact file names the standing throwaway provider — no new fixture.

## Spec of needed change
Two moves in one patrol edit (rbtdrv_patrol.rs), behind the await-fed-onepool-verbs barrier:
1. Fixture re-cut: the round-trip poisons the pool id and asserts pool-grain banners (create/dissolve/no-op naming the POOL); after the ₣Bf provider re-cut, affiance creates a provider under the standing manor pool and jilt deletes that provider. Re-point the poison at the provider field (expected RBRF_PROVIDER_ID — confirm the poisonable field and terminal banners against the landed ₢BfAA1 code at mount), re-target the banner assertions at the provider terminals, and keep the cleanup safety net (best-effort provider jilt on failure).
2. Canvass weave: after the affianced terminal, before jilt, invoke canvass (RBTDGC_CANVASS_FOEDUS) and assert via the multi-fact reader on RBCC_fact_ext_foedus that a fact file for the throwaway provider is present and its provider= line names it (expected stem: the bare provider id, since the throwaway is not in the rbef_ library — confirm at mount). Read the facts, not the human table.
canvass is payor-credentialed and the fixture proves that credential at its head — no new credential, charge, or quench.

## Done when
The round-trip exercises provider-grain affiance/jilt against the standing manor pool, and between them canvass's fact file enumerates the live throwaway provider — proving canvass reads a real, live foedus from the Manor; no new fixture stood up.

## Cinched
Behind the await-fed-onepool-verbs barrier — the ₣Bf one-pool verb code (₢BfAA0 through ₢BfAA1) must be merged here first.
This pace owns the fixture's provider-grain re-cut — theurge-crate writes stay in this clone; no ₣Bf pace touches the crate beyond the poison-const safety re-point riding ₢BfAA0.
Existing foedus-lifecycle fixture; assert on the fact file, not stdout.
Depends on the canvass build (₢BlAAU, landed).

## Character
Theurge patrol edit — fixture re-cut plus one fact-file assertion; mechanical once the barrier clears.

### parley-authentic-verb-fixture (₢BlAAX) [complete]

**[260702-0318] complete**

The parley fixture — the POSITIVE federation admission round-trip:
drive the real polity verbs (brevet, unseat, rehearse) on the real freehold subject against the standing terrier,
and graduate the RBSTC-reserved word parley.
Positive mirror of polity-denial (the negative sibling in the patrol module, and the pattern source):
parley's novel content is rehearse's positive manor-roll assertions and the real verbs succeeding through the governor-wielded folder-scoped IAM path (the verbs don internally; their exit 0 is that proof).
polity-denial owns all denial-band assertion — no second denial poll here.

DEPENDS ON the preceding read-surface pace:
a first cut is notched (de1501dd7) that asserts via manor-wide rehearse,
but the live run proved a manor-wide roll cannot witness a depot-scoped admission churn (unseat is depot-scoped; rehearse is manor-wide and drops the depot; a cross-depot orphan keeps the vanish from ever holding).
Re-cut parley's assertion to whatever read surface that pace settles.

## Done when
A fixture named parley — almanac roster + picket/echelon membership, plus a base-free parley probe suite in RBTDRA_SUITES with the usual ride-alongs (rbw-ts tabtarget, CLAUDE.md probe-suite row) —
drives unseat then restore-brevet on the freehold subject's retriever mantle,
asserting through the settled read surface that the (subject, mantle) muniment stands at baseline, vanishes after unseat, and stands again after the restore;
ends on the don-green poll so the freehold leaves exactly as found.
The scaffolding from the notched first cut (fixture, suite, manifest, oracle model, tabtarget, RBSTC characterization) is reused; only the assertion lens is re-cut.
Proven live against a standing depot seated for the subject.

## Cinched
Unseat-first: the subject's retriever muniment already stands, so brevet-first would ride the 412-idempotent engross — unseat first makes the restore a genuinely fresh write.
Churn retriever only (governor stays the pinned wielding mantle); payor-gate self-skip, the suite-passenger posture.
No test-rig tabtarget in the call path — the standing terrier is presumed; retiring the scaffold pair / terrier-atomicity stays federation-heat work.
The read lens is inherited from the preceding read-surface pace, not re-decided here.

## Character
Theurge bare-cloud fixture; the scaffolding is mechanical (mostly notched); the live novelty is the settled positive-read assertion.

### await-fed-terrier-band (₢BlAAc) [complete]

**[260702-0333] complete**

BARRIER — cross-clone wait. No code work of its own.
This pace cannot complete until ₢BfAAt (heat ₣Bf — terrier-band-discrimination: the bubc terrier bands plus the rbuh fault-injection seam) is complete, pushed to origin, and origin merged back into this clone.
The next pace — the terrier-poison fixture (₢BlAAY) — forces each terrier error through the rbuh seam and asserts the EXACT bubc terrier band, so the bands and seam ₢BfAAt creates must be present here first.

## Done when
₢BfAAt is wrapped and pushed to origin, and this clone has pulled origin so the bubc terrier bands + the rbuh seam are present before ₢BlAAY asserts them.

## Character
Cross-clone barrier (the terrier zigzag ₢BfAAt → ₢BlAAY → ₢BfAAz, the ₣Bl→₣Bf hand-back leg); hold here until ₢BfAAt is merged through.

### terrier-poison-fixture (₢BlAAY) [complete]

**[260702-0106] complete**

A theurge poison fixture (the regime-poison analogue) for the terrier — drive the REAL polity verbs with the rbuh fault-injection seam forcing each terrier error condition, and assert the EXACT named bubc terrier band from the tabtarget exit. Rust-validated negative testing, no rbgft test-rig tabtarget.

## Done when
A theurge fixture (regime-poison shape, the rbtdrs_poison precedent) forces each terrier error through the real verbs via the rbuh seam and asserts the exact bubc terrier band; this fixture stands registered as the replacement for the terrier-atomicity proof.

## Cinched
Depends on ₢BfAAt (terrier-band-discrimination — the bubc band + buc_reject + rbuh seam), delivered via the await-fed-terrier-band barrier ahead of this pace.
This fixture REPLACES the terrier-atomicity proof, but the retirement itself is ₢BfAAz's (₣Bf), coordinated through that heat's await-thg-registry-relocation barrier — do not retire the old fixture here, and do not duplicate the retirement.

## Character
Theurge negative fixture on the regime-poison precedent.

### cosmology-stem-ratification (₢BlAAd) [complete]

**[260701-1330] complete**

Perform the terminal Fable review the RBSTC header reserves:
ratify or re-mint the provisional `rbtt_` quoin stem and the word "theurge" (asterism fit, Lapidary pass, sub-letter legend).

## Model gate
Fable 5 only — the provisional marker names the model; decline under any other.

## Spec of needed change
Resolve the `rbtt` namespace collision discovered at review:
the stabled heat rbk-21-win-theurge-tabarget-auditing holds docket-only claims on `Tools/rbk/rbtt_testbench.sh` — one side must move (terminal exclusivity / monosemy).
Sweep the provisional markers: RBSTC header block, the `rbtt_theurge` definition parenthetical, and any echo in the RBS0 mapping block and the RBSTC acronyms entry.
Record the ratification (date, model) at the sweep sites and reconcile the ₣Bl paddock's provisional-stem cinch line.
Reslate the claiming dockets in the stabled heat if the testbench side moves.

## Done when
No await-review / provisional-naming marker survives;
the stem and word are final and recorded;
the colliding docket claim is reconciled with the outcome.

## Character
Fable judgment on the mint; mechanical sweep once decided.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 f rehearse-depot-attribution
  2 A create-skimpy-theurge-subdoc
  3 B freehold-wrapper-combinator-lift
  4 C theurge-crucible-module-split
  5 D reveille-base-set-equality-assertion
  6 R await-fed-foedus-subdocs
  7 E foedus-reuse-leg
  8 F rbtdth-shared-test-helpers
  9 Z registration-mirror-to-const-guard
  10 G ifrit-invoke-dead-code-removal
  11 H ordain-capture-helpers-to-shared-home
  12 I lode-roundtrip-block-helpers
  13 J registry-drift-guard-and-heal
  14 K drive-hallmark-through-config-console
  15 L onboarding-reliquary-probe-helper
  16 M payor-gate-helper-richer-signature
  17 T qualify-colophon-check-revive
  18 N colophon-relocation-260624
  19 S await-fed-rbs0-block
  20 O theurge-cosmology-subdoc-finalize
  21 P read-existence-failure-bands
  22 a await-fed-onepool-recut
  23 U foedus-canvass-build
  24 b colophon-completeness-check
  25 W rekon-chaining-band-case
  26 e await-fed-onepool-verbs
  27 V foedus-canvass-test
  28 X parley-authentic-verb-fixture
  29 c await-fed-terrier-band
  30 Y terrier-poison-fixture
  31 d cosmology-stem-ratification

fABCDREFZGHIJKLMTNSOPaUbWeVXcYd
x··x··x···xx···x·x··x·····xx·x· rbtdrv_patrol.rs
···xx·x·····x··············x·x· rbtdra_almanac.rs
······x··········x····xx···x··· rbz_zipper.sh
···x····x·x··xx················ rbtdro_onboarding.rs
······x···················xx·x· rbtdrm_manifest.rs
······x··········x····x····x··· claude-rbk-tabtarget-context.md
······x··········x··x·x········ rbtdgc_consts.rs
······x··········x·x··········x claude-rbk-acronyms.md
x··················x·······x··x RBSTC-theurge_cosmology.adoc
······x·············x·x········ rbcc_constants.sh
···x···xx······················ rbtdtk_freehold.rs, rbtdtl_calibrant.rs, rbtdto_onboarding.rs, rbtdtp_lifecycle.rs
··x·····x········x············· rbtdrk_depot.rs
··xx·······················x··· rbtdtc_crucible.rs
··xx····x······················ lib.rs
···················x··········x RBS0-SpecTop.adoc
···················x·······x··· CLAUDE.md
················x······x······· rbq_qualify.sh
·········xx···················· rbtdri_invocation.rs
·······xx······················ rbtdth_helpers.rs
······x···············x········ rbof_cli.sh, rbof_foedus.sh
······x·············x·········· bubc_constants.sh
···x······x···················· rbtdrd_dogfight.rs
··x·····x······················ rbtdrp_lifecycle.rs
x················x············· rbgp_payor.sh
···························x··· rbw-ts.TestSuite.parley.sh
························x······ rbtdrh_chain.rs
·······················x······· burc.env, burc_regime.sh, buw_workbench.sh, buwz_zipper.sh, buz_zipper.sh, rblm_cli.sh, rbw_workbench.sh
······················x········ rbw-jc.FoedusCanvass.sh
····················x·········· rbfcg_gar.sh, rbfr_retriever.sh, rbldl_lifecycle.sh
·················x············· rbw-aa.CheckFederatedAccess.sh, rbw-acf.CheckFederatedAccess.sh, rbw-acm.CheckMantleAccess.sh, rbw-acp.CheckPayorCredential.sh, rbw-am.CheckMantleAccess.sh, rbw-ap.CheckPayorCredential.sh, rbw-mG.PayorGirdsGovernor.sh, rbw-pE.PayorGirdsGovernor.sh
············x·················· rboo_observe.sh, rbrn.env
········x······················ rbtdrl_calibrant.rs
·······x······················· rbtdte_engine.rs, rbtdti_invocation.rs
······x························ rbrf.env, rbrr.env, rbrr_regime.sh, rbw-jI.FoedusInstate.sh, rbw-jd.FoedusDescry.sh
···x··························· main.rs, rbtdrc_crucible.rs, rbtdrs_poison.rs
··x···························· rbtdrk_freehold.rs
·x····························· memo-20260622-fable-review-queue.md
x······························ RBSPO-terrier_rehearse.adoc, rbgft_terrier.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 133 commits)

  1 d cosmology-stem-ratification
  2 P read-existence-failure-bands
  3 a await-fed-onepool-recut
  4 U foedus-canvass-build
  5 b colophon-completeness-check
  6 W rekon-chaining-band-case
  7 Y terrier-poison-fixture
  8 e await-fed-onepool-verbs
  9 V foedus-canvass-test
  10 X parley-authentic-verb-fixture
  11 f rehearse-depot-attribution
  12 c await-fed-terrier-band

123456789abcdefghijklmnopqrstuvwxyz
··xx·······························  d  2c
····xxxx···························  P  4c
········x··························  a  1c
·········xx························  U  2c
···············xx··················  b  2c
·················xxx···············  W  3c
····················xx·············  Y  2c
······················x············  e  1c
·······················xx··········  V  2c
··························x······x·  X  2c
····························xx··x··  f  3c
··································x  c  1c
```

## Steeplechase

### 2026-07-02 03:33 - ₢BlAAc - W

Cross-clone barrier lifted and verified — ₢BfAAt (bubc terrier bands BUBC_band_engross/expunge/peruse=111/112/113 + the rbuh fault seam zrbuh_fault_apply under buorb_http_fault) is wrapped and in this clone's HEAD ancestry (alongside the later ₢BlAAe:W barrier-lift wrap), and the bands, seam, and projected RBTDGC_BAND_*/RBTDGC_TWEAK_HTTP_FAULT consts are all present in the tree. No code work of its own. This was the heat's sole remaining pace, so wrapping it completes ₣Bl (rbk-02-mvp-review-theurge-refactor).

### 2026-07-02 03:18 - ₢BlAAX - W

Parley fixture complete and proven live. First cut (de1501dd7) built the full scaffolding: fixture in the patrol module, almanac roster + picket/echelon membership, base-free parley probe suite in RBTDRA_SUITES, manifest required-colophon arm, oracle-model classification, rbw-ts tabtarget, CLAUDE.md probe-suite row, RBSTC characterization. The assertion-lens re-cut this docket gated on arrived via the read-surface pace's convergence work: the depot-attributed emission plus the 260702 provider-grain merge yielded the four-column roll (depot, mantle, provider, subject), and parley now composes depot + active provider from the kindled regimes and asserts the fully-attributed exact line at baseline/vanish/restore, ending on the don-green poll. Live green end-to-end on the merged tree (build, 156/156 unit tests, shellcheck 224 clean) per the read-surface pace wrap; no further code motion under this coronet.

### 2026-07-02 03:14 - ₢BlAAf - W

Ruled the terrier read surface depot-attributed: the shared lister emits a depot column read from the object key's first segment (placement is the index's to tell; content stays the authoritative record), recorded where it binds (RBSPO depot-attributed emission NOTE superseding depot-blind; RBSTC parley characterization). The live parley run then falsified the docket's cross-depot-orphan hypothesis — attributed, the aliasing line resolved to a SAME-depot muniment-schema skew against the federation stream's provider-grain re-cut, which the depot column made legible. Merged that re-cut through (merge 5e7cb82c2), converging both attributions into the four-column roll (depot, mantle, provider, subject); parley composes depot + active provider from the kindled regimes and asserts the fully-attributed exact line. Verified: build, 156/156 unit tests, shellcheck 224, terrier proof live green, parley live green end-to-end for the first time. Follow-on terrier-hygiene-sweep slated as ₢BfAA5; ₣Bl paddock updated to delivery-taken.

### 2026-07-02 03:13 - Heat - d

paddock curried: provider-grain delivery taken via 260702 merge; four-column roll converged; parley live green

### 2026-07-02 02:35 - Heat - d

paddock curried: parley live run: aliasing roll line resolved to same-depot muniment-schema skew (provider-grain standing terrier vs provider-blind verbs here), recorded as a second pending federation-stream delivery

### 2026-07-02 02:34 - ₢BlAAf - n

Correct the collision provenance in the parley header comment and RBSTC's parley characterization after the depot-attributed live run: the aliasing line was NOT a cross-depot orphan (the docket's hypothesis) — attributed, it resolved to a SAME-depot muniment-schema skew. The standing terrier muniments are provider-grain (four-segment key <depot>/<mantle>/<provider>/<subject> with provider spike-entra — the live RBRF_PROVIDER_ID — plus an rbgft_provider content field), engrossed by the federation stream's edition; rbgft_provider has never existed in this clone's history, so this tree's provider-blind unseat cannot expunge them and parley's live green waits on that verb re-cut's delivery. The depot column keeps its cross-depot rationale (a depot-blind roll cannot attribute at all, and churn runs no terrier sweep) and additionally proved its diagnostic worth by making the skew legible. Live parley run: baseline passed depot-scoped, unseat genuinely expunged the provider-blind stray (204), the after-unseat assertion correctly failed on the standing provider-grain line, and the on-breach restore left the freehold as found. Unit tests 156/156, shellcheck 224 clean.

### 2026-07-02 02:26 - ₢BlAAf - n

Rule and land depot-attributed emission for the terrier read surface. The manor-wide roll could not witness a depot-scoped admission churn: identical (mantle, subject) pairs co-reside across polity slices (orphans of unmade depots persist in the payor-grain terrier), and the parley live run hit exactly that alias — an orphan retriever line read an unseat as a no-op. Ruling: the shared lister zrbgft_list_fetch_emit now emits <depot>\t<mantle>\t<subject>, the depot column read from the object key's first segment (placement is the index's alone to tell; content stays the authoritative record for mantle/subject), uniform across per-polity peruse and manor-wide peruse_manor. rehearse passes it through unchanged. Terrier proof's pair assertion grows the depot column (strictly stronger). Parley now composes the freehold's live depot project id from the kindled regime (new zrbtdrv_freehold_depot_capture over rbtdrk_read_env_value + rbtdrk_compose_project_id, composed once before any verb runs) and every roll assertion is the exact depot-scoped three-column line, so cross-depot orphans cannot alias baseline/vanish/restore. Ruling recorded where it binds: RBSPO's Depot-blind emission NOTE superseded by Depot-attributed emission (with the located-consumer rationale), its step/typed-output/completion语 updated to (depot, mantle, subject); RBSTC's parley characterization now names the depot-scoped exact-line lens. Builds clean; unit tests + live parley run next.

### 2026-07-02 02:09 - Heat - d

batch: 1 reslate, 1 slate

### 2026-07-02 01:56 - ₢BlAAX - n

Build the parley fixture — the POSITIVE federation-admission round-trip, positive mirror of polity-denial. Drives the real polity verbs against the real freehold subject: baseline rehearse asserts the (retriever, subject) muniment stands, unseat withdraws it, rehearse asserts it vanished, restore-brevet re-engrosses (genuinely fresh, unseat-first per Cinched), rehearse asserts it stands again, then a final don-green poll over the eventually-consistent IAM binding leaves the freehold as found. Novel content is rehearse's positive manor-roll assertions (asserted nowhere else); rehearse/brevet/unseat don governor internally so their exit 0 is the governor-wielded folder-scoped IAM-path proof. No don-denial poll — polity-denial owns all denial-band assertion. Roll assertion is exact-line '<mantle>\t<subject>' membership via zrbtdrv_roll_holds_retriever + zrbtdrv_rehearse_roll helpers; reuses zrbtdrv_payor_gate (Skip, suite-passenger) and zrbtdrv_mantle_denial_poll_until. Fixture in rbtdrv_patrol.rs; RBTDRM_FIXTURE_PARLEY + required-colophon arm (check-payor/check-mantle/unseat/brevet/rehearse) in the manifest; roster + picket + echelon membership plus a new base-free 'parley' Reuse probe suite in the almanac; ZRBTDTC_PARLEY + Reuse classification in the rbtdtc oracle model. Ride-alongs: rbw-ts.TestSuite.parley.sh tabtarget, CLAUDE.md probe-suite row. Trimmed the rbw-ts zipper purpose string to 'Run a named test suite' (dropped the drift-prone suite enumeration — the imprint tabtargets + RBTDRA_SUITES are the discoverable set; matches rbw-tf's cadence), regenerating tabtarget-context.md. RBSTC's scoped-out parley note migrated to a built '== The parley admission probe' characterization. Builds clean (const guards pass); unit tests + live run pending.

### 2026-07-02 01:40 - Heat - d

batch: 1 reslate

### 2026-07-02 01:22 - ₢BlAAV - W

Re-cut the foedus-lifecycle round-trip to provider grain and wove canvass into its live window (no new fixture). The throwaway id now rides the regime-poison seam on RBRF_PROVIDER_ID (kindled under scope RBRF by affiance/jilt via rbgp_cli) instead of RBRW_WORKFORCE_POOL_ID: affiance seats a fresh throwaway provider under the manor's standing pool, jilt removes it, the pool is never created/destroyed here. Banners re-targeted to the landed one-pool verbs (RBSMA/RBSMJ): 'Provider <id> created under pool', 'Manor affianced: provider=<id>', 'Foedus jilted: provider <id> dissolved', re-jilt no-op naming the provider. New step 2 invokes canvass (rbw-jc, unpoisoned) and asserts via rbtdri_read_burv_facts_multi on RBTDGC_FACT_EXT_FOEDUS that the throwaway provider's fact file (stem = bare provider id, library-unknown) is present and its provider= line names it. Manifest foedus-lifecycle required-colophon list gained RBTDGC_CANVASS_FOEDUS. Output files renumbered (canvass=03, jilt=04, rejilt=05, passed=06); id shape provider-<millis>; header + cleanup-net comments updated to provider grain. Notched c7d413de1 (builds clean). PROVEN LIVE against the real org: 1 passed / 0 failed / 0 skipped (~20s) — canvass enumerated the throwaway provider-1782980393134 (unmatched) alongside the real rbef_entrada/spike-entra which stayed ACTIVE+SELECTED; jilt soft-deleted the throwaway so nothing leaked; real foedus untouched.

### 2026-07-02 01:19 - ₢BlAAV - n

Re-cut the foedus-lifecycle round-trip to provider grain and weave canvass into its live window. The throwaway id now rides the regime-poison seam on RBRF_PROVIDER_ID (RBRF_ enroll-scope, kindled by affiance/jilt via rbgp_cli), not RBRW_WORKFORCE_POOL_ID: affiance seats a fresh throwaway provider under the manor's standing pool and jilt removes it (the pool is manor-level and never created/destroyed here). Banners re-targeted to the landed one-pool verb terminals (RBSMA/RBSMJ): create 'Provider <id> created under pool', terminal 'Manor affianced: provider=<id>', dissolved 'Foedus jilted: provider <id> dissolved', re-jilt no-op naming the provider. New step 2 invokes canvass (rbw-jc, unpoisoned — reads the pool live from the Manor) and asserts via rbtdri_read_burv_facts_multi on RBTDGC_FACT_EXT_FOEDUS that the throwaway provider's fact file (stem = bare provider id, library-unknown) is present and its provider= line names it. Output files renumbered (canvass=03, jilt=04, rejilt=05, passed=06); throwaway id shape provider-<millis>; header comment + cleanup-net rationale updated to provider grain. Manifest: added RBTDGC_CANVASS_FOEDUS to the foedus-lifecycle required-colophon list. Imported RBTDGC_CANVASS_FOEDUS + RBTDGC_FACT_EXT_FOEDUS. Builds clean; live run pending.

### 2026-07-02 01:07 - ₢BlAAe - W

Cross-clone barrier lifted — verified, no code work of its own. ₢BfAA1 (the one-pool provider re-cut of affiance/jilt/descry) is wrapped (commit 2b57a0e4e :W, picket green post-clone-sync 157/0), present on origin/main (local HEAD ahead only by this session's officium commit, so BfAA1's wrap sits below origin's tip), and in this clone's main ancestry along with its ₣Bf predecessors (BfAA0/BfAAw/BfAAF by heat order) and siblings (BfAAx raze :W, BfAAt terrier band :W). The one-pool code substrate now stands in this clone; ₢BfAA1's wrap deliberately left the foedus-lifecycle fixture pool-grain for ₣Bl to re-cut, teeing up the canvass-test weave and parley fixtures to mount.

### 2026-07-02 01:06 - ₢BlAAY - W

Folded the terrier-band poison coverage into the mantle-denial fixture and renamed it polity-denial: one picket fixture, two arcs on one credentialed setup. The preserved admission arc asserts BAND_ADMISSION (109) via the IAM-denial dance; the new terrier-band arc drives the REAL polity verbs (brevet/unseat/rehearse) under the rbuh buorb_http_fault seam on a SYNTHETIC subject, asserting engross 111 / expunge 112 / peruse 113 exactly (the drive helper asserts exit==band, never bare-nonzero). Chose augmentation over a standalone sibling for suite-time economy — reuses the one avow/don + the 420s IAM poll, and steady-state leaves 2 terrier picket fixtures not 3 once terrier-atomicity retires under BfAAz. No explicit charge: the admission arc's restore brevet already engrosses a real muniment. Pre-clean + final sweep leave no synthetic muniment. VALIDATED LIVE: real credentialed picket run, 1 passed / 0 failed / 0 skipped (~90s), all three terrier bands + the admission arc asserted, freehold subject restored. Committed+pushed as 8ef0ecec9. Cross-heat note for later: BfAAz should key terrier-atomicity retirement off polity-denial's terrier arc, not a fixture named terrier-poison.

### 2026-07-01 21:21 - ₢BlAAY - n

Fold the terrier-band poison coverage into the mantle-denial fixture and rename it polity-denial (economy over a sibling: reuse the one credentialed setup + the 420s IAM poll; steady-state 2 terrier picket fixtures, not 3, once terrier-atomicity retires under BfAAz). The fixture now proves the polity verbs reject with the EXACT bubc precision band across two arcs on one setup: the existing admission arc (IAM-denial dance -> BAND_ADMISSION 109) and a new terrier-band arc driving the same real verbs under the rbuh buorb_http_fault seam on a SYNTHETIC subject -> brevet/engross 111, unseat/expunge 112, rehearse/peruse 113. The seam overwrites the captured code only after the real transport succeeds, so the fault fires regardless of the real code; a synthetic subject (never the freehold subject) plus pre-clean + final sweep leave no muniment, and no explicit charge is needed since the admission arc's restore brevet already engrossed a real muniment. Added zrbtdrv_terrier_poison_sweep/_drive helpers and the synthetic-subject + fault-spec consts; imported the three terrier band consts + RBTDGC_REHEARSE_POLITY + RBTDGC_TWEAK_HTTP_FAULT. Rename swept the fixture static, case fn, the abjure cross-ref comment, the manifest const/name (mantle-denial -> polity-denial) with rehearse added to its colophon list, and the almanac roster + picket + echelon refs. Held: not yet built or tested (parallel picket run is using the shared theurge binary).

### 2026-07-01 17:45 - ₢BlAAW - W

Added rekon (rbw-irh) to the read-consumer chaining-fact-band matrix in rbtdrh_chain.rs. New rbtdrh_rekon_no_folio case drives rekon folio-less against an empty BURV root and asserts a broken express-or-chain resolve rejects with BUBC_band_chain, peer to summon/plumb/augur (credless drive; fact-intact/no-durable-write come free from the empty-BURV path). Registered after the augur cases; updated the four read-side-consumer enumerating comments (module header, read-side block, driver doc, case-block) to name rekon. Dependency confirmed landed (Bi: rbfln_inventory.sh:156 rekon broken-resolve buc_die -> buc_reject BUBC_band_chain). Fixture chaining-fact-band: 15 passed, 0 failed.

### 2026-07-01 17:45 - ₢BlAAW - n

Import RBTDGC_REKON_HALLMARK into rbtdrh_chain.rs scope (add to the crate::rbtdgc_consts use list, alpha-ordered between PLUMB_FULL and SUMMON_HALLMARK) — the new rekon case referenced it but the module's explicit-const import omitted it. Build fix for the prior notch.

### 2026-07-01 17:44 - ₢BlAAW - n

Add rekon (rbw-irh) to the read-consumer chaining-fact-band matrix in rbtdrh_chain.rs: new rbtdrh_rekon_no_folio case drives rekon folio-less against an empty BURV root and asserts the broken express-or-chain resolve rejects with BUBC_band_chain, peer to summon/plumb/augur (same credless drive, fact-intact/no-durable-write checks come free from the empty-BURV path). Registered in RBTDRH_CASES_CHAINING_FACT_BAND after the augur cases. Updated the four enumerating comments (module header, read-side-consumer block, driver doc, case-block comment) to name rekon in the read-side consumer set. Dependency landed in Bi (rbfln_inventory.sh:156 rekon broken-resolve buc_die -> buc_reject BUBC_band_chain); RBTDGC_REKON_HALLMARK already present.

### 2026-07-01 17:31 - ₢BlAAb - W

Relocated the colophon-completeness proof off the per-dispatch path onto the rbw-MZ (marshal-zero) precondition — the source-only tabtarget already withheld from delivery. Added rbq_qualify_completeness sweeping both RBW+BUW registries via buz_healthcheck (improved to list all missing colophons, not first-only); wired it as an rblm_zero gate beside shellcheck. Removed the per-dispatch buz_healthcheck sweep from rbw_route and buw_route (and the dead zrbz_healthcheck/zbuwz_healthcheck), letting a stripped consumer pass at dispatch — also fixes a latent prep-release Step 10 breakage where the strict sweep would die on already-withheld rbw-MZ/rbw-MP colophons. Retired BURC_BUW_COLOPHON_CHECK (enum + value; grep confirmed no other consumer). Verified: shellcheck 221 clean, harness proves both-registry positive (144 colophons, exit 0) + negative (dies listing all missing), buw + rbw-tq dispatch green. Left the reverse-direction no-op rbq_qualify_colophons untouched (₢BlAAN territory). One code decision made and confirmed with operator mid-pace: home the check as rbw-MZ's precondition rather than mint a new dedicated colophon.

### 2026-07-01 17:26 - ₢BlAAb - n

Relocate colophon-completeness proof off the per-dispatch path to an rbw-MZ (marshal-zero) precondition. Add rbq_qualify_completeness sweeping both RBW+BUW registries via buz_healthcheck (improved to list all missing colophons); wire it as an rblm_zero gate beside shellcheck. Remove the per-dispatch buz_healthcheck sweep from rbw_route and buw_route (and the dead zrbz_healthcheck/zbuwz_healthcheck functions) so a stripped consumer passes at dispatch. Retire BURC_BUW_COLOPHON_CHECK (enum + value) now that its per-dispatch job is gone.

### 2026-07-01 16:50 - Heat - d

batch: paddock, 3 reslate, 1 slate

### 2026-07-01 16:49 - Heat - r

moved ₢BlAAW after ₢BlAAb

### 2026-07-01 16:49 - Heat - r

moved ₢BlAAb to first

### 2026-07-01 16:31 - Heat - d

batch: 1 reslate

### 2026-07-01 16:13 - ₢BlAAU - W

Built canvass (rbw-jc), the read-only payor-credentialed foedus enumeration verb — rbof_canvass paginates providers.list under the one manor pool per the re-cut sheaf, correlates provider ids against the rbef_ library's RBRF_PROVIDER_ID values (settle-at-build: matching, not affiance-stamping), emits one fact file per foedus on the new RBCC_fact_ext_foedus extension (stem: matched rbef_ name or bare provider id; provider=/state=/selected= lines marking RBRR_ACTIVE_FOEDUS) plus a depot_list-shape summary; empty manor and pool-absent report zero non-fatally, broken reads die generic (no new band, per the depot_list shape). Homed with descry in rbof_foedus.sh sharing its furnish branch in rbof_cli.sh; fact-ext projected to RBTDGC; zipper-enrolled with regenerated artifacts. Verified: shellcheck 222 clean, reveille 119/0, fast qualify passed, and a live canvass of the real Manor correlated spike-entra to rbef_entrada as selected with the fact file emitted to spec.

### 2026-07-01 16:08 - ₢BlAAU - n

Build canvass (rbw-jc) — the read-only payor-credentialed foedus enumeration verb, read-all sibling of descry: rbof_canvass lists every provider under the one manor workforce pool (providers.list against the re-cut one-pool sheaf), correlates each provider id against the rbef_ library's RBRF_PROVIDER_ID values (the settle-at-build choice: matching, not affiance-stamping — read-only and works against standing providers), emits one fact file per foedus on the new RBCC_fact_ext_foedus extension (stem: matched rbef_ name or bare provider id; provider=/state=/selected= lines, selected marking RBRR_ACTIVE_FOEDUS), then a depot_list-shape summary table; empty manor and pool-absent both report zero non-fatally. Pool coordinates resolve from the active foedus's rbrf.env (one-pool Model; RBSRW is spec-only). Broken reads die generic per the depot_list shape — no new precision band. Homed with descry in rbof_foedus.sh sharing its payor-OAuth/IAM-REST furnish branch in rbof_cli.sh; RBCC_fact_ext_foedus projected to RBTDGC beside foedus-health; rbw-jc enrolled in the zipper (channel empty — no folio) with the two build-regenerated artifacts riding along; tabtarget byte-identical to siblings.

### 2026-07-01 15:09 - ₢BlAAa - W

Cross-clone barrier satisfied and closed. Verified ₢BfAAv (260630 one-pool federation re-cut) is wrapped (26459cf0e, :W: chalk), present on origin/main, and an ancestor of this clone's HEAD. The re-cut content is confirmed in-tree: RBSRF recut to provider-under-one-pool, the mid-pace-minted RBSRW-RegimeWorkforce.adoc, and the providers.list canvass references in RBSRF + RBS0. No code work of its own; the commit-based Done-when is fully met. Clears the foedus-cardinality build paces below (canvass build/test, descry/instate code consuming the re-cut sheaves) to build against the one-pool contract.

### 2026-07-01 14:40 - ₢BlAAP - W

Extended the read-side named-band discipline to a second failure class. Minted BUBC_band_vacant=110 (a hallmark/Lode the user names is absent from the registry, knowable only after a round-trip) and made the three read verbs reject it instead of dying generic: summon (rbfr:164 neither-ark), plumb (rbfcg:379 sole-caller vessel-resolve no-vouch-ark, band propagated through the outer buc_die membrane), augur (new 404->vacant branch after the tags GET, plus the empty-tags rbldl:174 alternative). Integrity/infra dies left as buc_die; augur-unvouched and plumb-orphaned-about held as buc_die per the mount-time calls. Projected RBTDGC_BAND_VACANT via the rbcc emit list. Proved with negatives riding two existing cloud fixtures (no new fixture stood up): hallmark-lifecycle gained post-abjure summon+plumb, lode-lifecycle a post-banish augur, each asserting the exact 110 via a shared zrbtdrv_expect_vacant helper. Both cloud fixtures green; reveille's regime/band machinery green (the sole reveille-red is an orthogonal Bi kroki-vs-bind-digest-gate matter, pre-existing since 6009125de). Three commits: 09db02992 band+retags, 6e87be411 fixture cases, b99d2d564 augur 404 fix.

### 2026-07-01 14:26 - ₢BlAAP - n

Augur: map a 404 on the tags fetch to the vacant band, the real absent-Lode signal. A banished or never-captured Lode answers 404 (GAR NOT_FOUND), which rbuh_require_ok previously swallowed into a generic infra die before the empty-tags check at line 174 could fire — so the retag alone never covered the realistic absent case. Add an explicit 404->buc_reject BUBC_band_vacant branch after the tags GET (mirroring summon's explicit 404 handling and plumb's vouch-ark-extract failure), leaving every other non-OK status infra-generic through rbuh_require_ok. The empty-tags line-174 guard stays as the secondary 200-with-no-tags alternative. Surfaced by the lode-lifecycle fixture failing (post-banish augur exited 1, not 110).

### 2026-07-01 14:05 - ₢BlAAP - n

Add read-side vacant-band negative cases riding the two existing cloud lifecycle fixtures — no new fixture. hallmark-lifecycle gains post-abjure summon + plumb of the now-absent hallmark (Step 6b/6c); lode-lifecycle gains a post-banish augur of the now-absent touchmark (Step 5). Each asserts the exact RBTDGC_BAND_VACANT exit code via a shared zrbtdrv_expect_vacant bookend helper (no propagation poll — abjure/banish are synchronous, unlike mantle-denial's IAM revocation). Reuses each fixture's already-established precondition; read-only, so the restored-baseline invariants hold. Imports RBTDGC_BAND_VACANT/SUMMON_HALLMARK/PLUMB_FULL.

### 2026-07-01 13:56 - ₢BlAAP - n

Mint BUBC_band_vacant=110 for read-side absent-artifact rejection (named hallmark/Lode not present in registry, knowable only after a round-trip); retag the three Class-2 absent sites to buc_reject the vacant band — summon (rbfr:164 neither ark), augur (rbldl:174 no member tags), and plumb via the sole-caller vessel-resolve inner line (rbfcg:379 no vouch ark), whose band the existing outer buc_die propagates unchanged through the band membrane. Integrity/infra dies (vessel-resolve integrity checks, orphaned-about, augur unvouched, auth/network) left as buc_die per the absent-vs-integrity split. Project RBTDGC_BAND_VACANT via the rbcc emit list and rebuild. Negative test cases riding cloud fixtures deferred to the fixture phase.

### 2026-07-01 13:30 - ₢BlAAd - W

Fable terminal ratification of the RBSTC naming, executed same-session as slated: the quoin stem rbtt_ (sub-letters Theurge-Test) and the word theurge are final — Lapidary held on every clause and the word anchors the ritual asterism. The rbtt namespace collision with the stabled tabtarget-auditing heat's docket-only testbench claims (module, rbts/ dir, rbtc case prefix) was resolved in the cosmology's favor on eviction asymmetry and semantic fit; ₢BCAAA and ₢BCAAD reslated to mint their testbench family letters at mount under the recorded constraint. Provisional markers swept from RBSTC, RBS0 (mapping comment now carries the sub-letter legend), and the acronyms entry; ₣Bl paddock cinch line records the ratification. Sweep verified by grep — zero surviving markers. Notched 54eb252ea. Out-of-scope observation left on the record: wrapper/case/lifecycle word-parts are trodden under strict Lapidary, settled at the subdoc pace, not reopened.

### 2026-07-01 13:29 - ₢BlAAd - n

Fable terminal ratification of the RBSTC naming: the quoin stem rbtt_ (sub-letters read Theurge-Test) and the word theurge are final. Lapidary held on every clause — repo-wide semantic uniqueness, no trodden-word taint, rare-but-real with the word doing semantic work (a theurge conducts rites invoking higher agencies, as the orchestrator does to crucibles), and the word anchors the ritual asterism (ifrit, pentacle, crucible, charge/quench, conjure, summon, augur). The rbtt namespace collision surfaced at review — the stabled tabtarget-auditing heat held docket-only claims on rbtt_testbench.sh plus the rbts/ dir and rbtc case prefix — resolved in the cosmology's favor on eviction asymmetry (landed three-surface spec vs docket prose, ~10:1) and semantic fit; the claiming dockets were reslated to mint their testbench family letters at mount, and the heat paddock's provisional cinch line now records the ratification. Provisional markers swept from the RBSTC header and rbtt_theurge definition, the RBS0 mapping-block comment (which now carries the sub-letter legend), and the acronyms entry.

### 2026-07-01 13:28 - Heat - d

paddock curried: rbtt_ stem + theurge word ratified at ₢BlAAd; provisional cinch line updated

### 2026-07-01 13:25 - Heat - S

cosmology-stem-ratification

### 2026-07-01 13:18 - Heat - d

batch: 1 reslate

### 2026-07-01 13:11 - ₢BlAAO - W

Homed the theurge test-orchestration cosmology as a new RBS0 subdoc (RBSTC-theurge_cosmology.adoc), superseding the dropped skimpy-subdoc pace ₢BlAAA. Homes the four-wrapper(inner) combinator (Base/Reuse/Lifecycle/Local + the structural laws: REUSE completeness = service ∪ crucible, ladder blockade ⊆ skirmish ⊆ gauntlet), the three dependency strata (reveille/picket/bivouac, echelon = union), and the freehold/leasehold substrate composite — all citable from code under a PROVISIONAL rbtt_ quoin stem (Fable ratifies the stem + the 'theurge' word). Wired into RBS0 (rbtt_ mapping block + a new == Theurge Test Cosmology include, leveloffset=+1), added the RBSTC acronyms entry, and thinned the CLAUDE.md suites table to a registry+subdoc pointer. Honored all five membranes; parley scoped out by decision as unbuilt federation-owned. Corrected the stale RBTDRC_SUITES→RBTDRA_SUITES reference and dropped the already-drifted CLAUDE.md fixture counts/lists. vow build green; adoc verified structurally (linked-term/anchor integrity clean, headings well-formed) since no local asciidoctor/GAD runner exists.

### 2026-07-01 13:06 - ₢BlAAO - n

Home the theurge test-orchestration cosmology as an RBS0 subdoc (RBSTC), superseding the dropped skimpy-subdoc pace. Homes the four-wrapper(inner) combinator (Base/Reuse/Lifecycle/Local + structural laws), the three dependency strata (reveille/picket/bivouac, echelon=union), and the freehold/leasehold substrate composite as citable spec under a provisional rbtt_ quoin stem (Fable ratifies the stem + 'theurge' word). Honors the five membranes: references BUS0 busr_fixture/busr_suite as literals, points at the RBTDRA_SUITES registry (correcting the stale RBTDRC_SUITES reference), cites the crucible-security specs, keeps the permanent-vs-pool citizen fact production-side (pins only the one freehold subject), and thins the CLAUDE.md suites table to a registry+subdoc pointer (dropping already-drifted fixture counts/lists). parley scoped out by decision as unbuilt federation-owned. Registers the rbtt_ mapping block + == Theurge Test Cosmology include in RBS0 and an RBSTC acronyms entry.

### 2026-07-01 12:46 - ₢BlAAS - W

Cross-clone barrier satisfied and closed. Verified ₢BfAAv (the 260630 one-pool RBS0-SpecTop federation re-cut, foedus≡provider) is wrapped (commit 26459cf0e, :W: chalk), pushed to origin/main, and an ancestor of this clone's HEAD — so the re-cut federation block is present locally. No code work of its own; the barrier's Done-when is commit-based and fully met. This clears the following pace (theurge-cosmology subdoc finalize) to append its cosmology region behind the now-present re-cut federation block.

### 2026-06-30 15:39 - Heat - r

moved ₢BlAAb before ₢BlAAW

### 2026-06-30 15:36 - Heat - d

batch: 1 reslate, 1 slate

### 2026-06-30 15:20 - Heat - S

colophon-completeness-check

### 2026-06-30 14:51 - Heat - S

await-fed-onepool-recut

### 2026-06-30 14:37 - Heat - d

paddock curried: 260630 one-pool Model touch: re-point REUSE-path premise (founding now provider-grain), descry (provider presence), canvass (providers.list)

### 2026-06-30 14:35 - Heat - d

batch: 2 reslate

### 2026-06-30 14:10 - ₢BlAAN - W

Applied the settled 260624 colophon relocations: collapsed the access family rbw-ac -> rbw-a (rbw-ap payor-credential check, rbw-aa avow, rbw-am interim mantle-access probe) and moved gird rbw-pE -> rbw-mG into the Manor founding trio (affiance/jilt/gird), leaving rbw-p purely governor. Edited the zipper enrollment home and rebuilt to regenerate rbtdgc_consts.rs + tabtarget-context.md, git-mv'd the four tt/ files (colophon is parsed from filename at dispatch), repointed the two acronym citations. Kept the RBZ_GIRD_POLITY constant name (it names the verb's demesne, not its colophon group -- proven by the access CHECK_* constants which need no rename) with an inline note so no future minter renames it; its five by-name consumers needed no edit and pick up rbw-mG on rebuild. Then folded in the operator-approved reference-hygiene pass over the same sites: removed the colophon from 6 explanatory comments (naming the stable verb rbgv_check_avowal where the tabtarget was the subject), and converted 3 operator-facing remediation strings to self-healing references -- the bash buc_warn to a true yawp, the two Rust Fail() verdicts to format!-interpolation of the already-imported RBTDGC_CHECK_AVOWAL (theurge has no yawp primitive). Build green, shellcheck 220 files clean, deliverable in 528005a4a. Verified by dispatch (the docket's required proof, qualify's colophon check being a dead no-op): rbw-ap routed to rbgv_check_payor and ran the OAuth probe to completion (exit 0); rbw-mG routed to rbgp_gird and arg-rejected before any mutation. rbw-aa/rbw-am routing follows by construction from rbw-ap (identical trampoline+enrollment mechanism) and were deliberately not dispatched to avoid triggering device-flow/cloud ops.

### 2026-06-30 14:07 - ₢BlAAN - n

Apply the settled 260624 colophon relocations, then fold in the operator-approved reference-hygiene pass over the same sites.

### 2026-06-30 13:39 - ₢BlAAL - W

Folded the seven byte-identical reliquary-touchmark probe preambles in the onboarding cases into one rbtdro_assert_reliquary_touchmark helper, homed beside its rbtdro_probe_reliquary_touchmark check fn and returning Result<(), rbtdre_Verdict>. Each of the seven reliquary-consuming cases (kludge_tadmor, kludge_ccyolo, ordain_conjure_sentry/jupyter, ordain_airgap_chain, ordain_bind_plantuml, ordain_graft_demo) now calls the helper via the identical if-let-Err-return shape; coverage-neutral by construction. Carve-outs honored per the docket: graft-demo folded only its reliquary preamble (the second graft-demo-anointed probe stays inline); the standalone rbtdro_kludge_tadmor_standalone path stays deliberately probe-free. Deliverable committed at e4931dc2c. Verified: theurge build green under deny(warnings), 156 unit tests pass (incl. the assert-fail-branch coverage rbtdtb_assert_returns_fail_verdict_when_precondition_unmet and rbtdtl_progressing_probe_err_fails_with_diagnostic, plus rbtdto_conclave_precedes_reliquary_consumers), and the altered happy-path exercised live by kludge_tadmor + kludge_ccyolo (passed) and conjure_sentry (entered) before the live onboarding-sequence run was stopped as redundant-for-this-edit. Six fixture-generated test-rig commits (conclave yoke, kludge hallmark pins, graft anoint) sit above the deliverable from the partial run, left for operator disposition.

### 2026-06-30 13:26 - ₢BlAAL - n

Fold the seven byte-identical reliquary-touchmark probe preambles in the onboarding cases into one rbtdro_assert_reliquary_touchmark helper. Each reliquary-consuming case (cases 3-7 plus the kludge-tadmor and graft-demo ordain entries) now calls the helper via the same if-let-Err-return shape; the probe construction (name/check/remediation) is homed once beside its rbtdro_probe_reliquary_touchmark check fn, returning Result<(), rbtdre_Verdict>. Coverage-neutral: identical probe, verdict text, and control flow at every site. Carve-outs honored per the docket: at the graft-demo case only the reliquary preamble was folded -- the second graft-demo-anointed probe stays inline; and the standalone kludge-tadmor path (rbtdro_kludge_tadmor_standalone) stays deliberately probe-free. No colophon or zipper touched, so the build-generated artifacts are untouched. Theurge build green under deny(warnings).

### 2026-06-30 13:21 - ₢BlAAJ - W

Closed the theurge discovery-vs-suite registry drift and guarded it: registered foundry-path in RBTDRA_FIXTURES (its static was already wired into reveille-base and six suites; only the lookup-by-name path was broken) and added the compile-time zrbtdra_assert_suites_subset_fixtures guard (suite ⊆ roster only, never the reverse — the intentional roster-only fixtures foedus-lifecycle/freehold-churn/calibrant-* are named in the comment). Operator then directed folding in a same-file awk-eviction (BiAAG's, holding reveille red) — which on reading BCG turned into a full BCG-compliance rewrite of rboo_observe.sh scry interface-discovery: temp-file capture + stderr-to-file + read-from-file replacing command-substitution/2>/dev/null/multi-line-here-string, a new ZRBOO_SCRY_PREFIX kindle constant, one-decl-per-line, plus line 169's network-inspect command-substitution; live tcpdump display pipelines and the builtin kill-trap deliberately left with rationale. Validated three ways: reveille fully green (11 fixtures, 119 passed, 0 failed, kit-bash gate restored), shellcheck 220 clean, and a live tadmor charge→bounded-scry→quench confirming runtime interface discovery (enclave=eth0 uplink=eth1). Five commits; includes one labeled test-rig rbrn.env kludge pin (57d43a01b) the operator may reset. Docket Done-when all met; guard reasoned-correct but not deliberately proof-fired.

### 2026-06-30 13:15 - ₢BlAAJ - n

Test-rig kludge pin (NOT pace deliverable): bump tadmor RBRN_BOTTLE_HALLMARK to the freshly-kludged k260630131316-f4f6348a3 so the live scry validation of this pace's rboo_observe BCG rewrite can charge a crucible. Single-line hallmark pin, same shape as the already-committed morning kludges; committed only to clear the kludge ceremony's clean-tree gate (bug_require_clean_tree) ahead of charge. The bottle re-kludge was incidental churn -- the sentry was already kludged this morning (RBRN_SENTRY_HALLMARK unchanged) and only a charge was strictly needed. Operator may reset this pin afterward if they do not want the transient local build ID in history.

### 2026-06-30 13:11 - ₢BlAAJ - n

Bring the podman-branch bridge-interface discovery into BCG compliance, the same-class straggler flagged in the prior notch. Line 169 captured 'network inspect --format {{.NetworkInterface}}' via unguarded command substitution with an unquoted runtime -- replaced with the temp-file pattern: redirect the inspect to a ZRBOO_SCRY_PREFIX bridge_if temp file (stderr to a sibling, buc_die naming it), then 'read -r z_rboo_bridge_interface < file' with a test-n guard for an absent bridge interface. Quoted ${ZRBOB_RUNTIME} at both podman sites (line 169 inspect and the line-175 machine-ssh streaming leg). Behaviorally equivalent in the success path (the --format output is a single line, so read equals the old command-sub) with added fail-fast guards; podman branch only, not runtime-exercised here (tadmor is docker). Deliberately left in place: the live tcpdump display pipelines (cmd 2>&1 | zrboo_prefix &) are background streaming whose output never enters a variable and whose exit code is irrelevant by design -- outside the Temp-Files-Instead-of-Command-Substitution rule (BCG: the pipeline concern is captured pipelines that hide failures, not live display); and the SIGINT-trap 'kill 0 2>/dev/null', since kill is a builtin and Stderr-Capture-Never-Suppress governs external commands. Shellcheck green (220 files clean).

### 2026-06-30 13:09 - ₢BlAAJ - n

Rewrite the scry sentry-interface discovery to genuine BCG compliance (operator caught the prior de-awk pass still violating BCG). The earlier fix swapped awk->read but left/introduced four violations the actual guide forbids: external command output captured via command substitution (BCG Temp-Files-Instead-of-Command-Substitution), 2>/dev/null stderr suppression (Stderr-Capture-Never-Suppress), a multi-line here-string (checklist: here-strings single-line only), and two declarations on one line (NEVER declare multiple variables per line). Replaced both 'ip -o addr' captures with the house temp-file pattern: a new kindle constant ZRBOO_SCRY_PREFIX="${BURD_TEMP_DIR}/rboo_scry_" (mirroring rbfr/rbgb/rbld0), each exec redirected to a temp file with stderr to a sibling temp file and a buc_die naming the stderr path, then read from the file -- 'read -r ... < file' for the enclave leg (first line, the evicted-table head->read replacement), and a 'while read -r ... || test -n' loop over the uplink file for the first global interface whose name differs from the enclave leg. One declaration per line; die messages and the is-the-crucible-charged hint preserved. Shellcheck green (220 files clean). Pre-existing same-class violations elsewhere in the file (line 169 network-inspect command-substitution) handled in a follow-up notch; the live tcpdump display pipelines and the builtin kill-trap are out of the temp-file rule's scope.

### 2026-06-30 12:57 - ₢BlAAJ - n

Clear the BCG awk-eviction that was holding reveille red, folded into this pace (operator-directed). The kit-bash discipline fixture (rbtdru_kit_bash, cupel, in reveille) flagged two awk uses in rboo_observe.sh:105,112 -- awk is an evicted command (use read with IFS + parameter expansion). Both were introduced by pace BiAAG (heat Bi, scry sentry-interface work, commit c87fb2ec4 2026-06-27), pre-existing and unrelated to this pace's foundry-path registry work, but blocking the pace's 'reveille suite green' Done-when; operator chose to fix in place rather than slate into Bi. Replaced 'awk {print $2; exit}' (first interface holding the enclave IP) with a capture-then-read: collect 'ip -o addr show to <IP>' into z_enclave_addr, then 'read -r z_if_idx z_sentry_enclave_if z_if_rest' takes field 2 of the first line (default-whitespace IFS, the rbfh_hygiene.sh:89 house idiom). Replaced 'awk -v enc=... {$2 != enc; print $2; exit}' (first global -4 interface whose name differs from the enclave leg) with a capture-then-while-read loop over z_uplink_addrs that skips empty ifnames and breaks on the first name != enclave. Semantics, die messages, and the || true exec-failure tolerance preserved byte-for-byte; both throwaway field vars are clean under the repo's disabled SC2034. Shellcheck green (220 files clean).

### 2026-06-30 12:50 - ₢BlAAJ - n

Close the discovery-vs-suite registry drift and guard against its recurrence. Heal: register foundry-path in the fixture roster RBTDRA_FIXTURES (one line, &crate::rbtdrf_fast::RBTDRF_FIXTURE_FOUNDRY_PATH, slotted between dockerfile-hygiene and recipe-validation to match suite ordering) -- the static was already fully wired (its cases, RBTDRA_REVEILLE_BASE, and all six major suites referenced it), so the only break was the lookup-by-name path (rbtdra_lookup_fixture) which the roster gap left unresolvable; FixtureRun foundry-path could not resolve despite the fixture riding reveille. Guard: add the compile-time suite-subset-of-roster assertion zrbtdra_assert_suites_subset_fixtures, a const fn + const _: () invocation mirroring the adjacent zrbtdra_assert_reveille_base idiom and reusing the existing zrbtdra_fixtures_contain helper -- it walks every fixture named in every RBTDRA_SUITES entry and panics at const-eval if any is absent from RBTDRA_FIXTURES, so a suite gaining a roster-less fixture now fails the build before any test runs. Direction is strictly suite subset-of roster, NEVER the reverse (cinched): the comment names the intentional roster-only fixtures (foedus-lifecycle, freehold-churn, calibrant-*) that belong to no suite by design, so a reverse check would wrongly fail them. Live constant names are RBTDRA_*/rbtdra_almanac.rs (the docket's RBTDRC_*/rbtdrc_crucible.rs names were stale -- the constants were extracted and renamed in the earlier crucible-module split). No colophon touched, so the build-generated rbtdgc_consts.rs / tabtarget-context.md are untouched. Theurge build green under deny(warnings).

### 2026-06-30 12:22 - ₢BlAAK - W

Routed rbtdro_drive_hallmark through the engine config-console seam (rbtdre_config_set_field), retiring the hand-rolled find-replace-rename so it mirrors its rbrv.env sibling rbtdro_write_vessel_env; the find-or-err schema-drift catch is preserved by construction (config_set_field carries it), atomic-rename semantics unchanged, and the orphaned Write import was trimmed (notch 83b2b66ea). Folded in a pre-existing harness fix surfaced while validating the gate: the onboarding-sequence kludge_ccyolo case was missing ctx.chain_next_invoke() between the kludge and the anoint, so under per-invoke BURV isolation the bottle kludge's rbf_fact_hallmark never reached anoint's previous/ and anoint rejected with band 105; added the one-line chain-arm matching the airgap case (notch 1e3027734). Latent since BHAAS (4d89aa018) introduced chain_next_invoke for the airgap case only and closed 'live onboarding fixture not yet run'.

### 2026-06-30 10:56 - ₢BlAAK - n

Arm the kludge->anoint build-fact chain in the onboarding-sequence kludge_ccyolo case (folded into this pace; pre-existing bug surfaced while validating the pace's onboarding-sequence gate). Root cause: theurge isolates each tabtarget invoke in its own BURV_OUTPUT_ROOT_DIR, so a build's hallmark fact in current/ never reaches the next invoke's previous/ unless the case calls ctx.chain_next_invoke(). The ccyolo case kludges (bottle build writes rbf_fact_hallmark) then immediately anoints, but never armed the chain -- anoint got a fresh root with an empty previous/, the chain read found nothing, and anoint rejected with band 105 (BUBC_band_chain). Fix: add ctx.chain_next_invoke() between rbtdro_kludge_nameplate and the anoint invoke, so anoint reuses the bottle kludge's root and bud promotes the hallmark into anoint's previous/ -- exactly as the airgap-chain case already does at its ensconce->feoff pair. The intervening rbtdre_commit_nameplates is a git commit, not a dispatch, so the depth-1 invoke chain holds. Provenance: chain_next_invoke was introduced by BHAAS (4d89aa018), which applied it to the airgap case only and signed off 'live onboarding fixture not yet run'; the identical ccyolo kludge->anoint chain was overlooked and stayed latent until the fixture was finally run. Comment added mirroring the airgap pair's explanation. Theurge build green under deny(warnings).

### 2026-06-30 10:40 - ₢BlAAK - n

Route rbtdro_drive_hallmark through the engine config-console seam (rbtdre_config_set_field), retiring the hand-rolled find-replace-rename. The function now mirrors its rbrv.env sibling rbtdro_write_vessel_env: resolve the rbrn.env path under the nameplate's moorings dir, then delegate to the validated config-field seam. The ~50-line open-coded body (File::open -> BufReader lines -> prefix-match rewrite -> found-guard -> tmp create/writeln -> atomic rename) collapses to a single delegated call. The find-or-err schema-drift catch is preserved by construction, not lost -- config_set_field carries it (a renamed/removed field stops the run, never a silent skip); doc comment updated to mirror the sibling's 'delegates to the validated config-field seam' phrasing and to name the preserved catch. Collapsing the body orphaned the Write trait (line 426 writeln! was the file's only Write use), so trimmed Write from the std::io import; BufRead/BufReader stay live for rbtdro_read_vessel_env. Atomic-rename semantics unchanged (the seam's own .write_tmp + rename). Theurge build green under deny(warnings).

### 2026-06-30 18:23 - ₢BlAAI - W

Extracted the four Lode round-trip shared blocks into within-module zrbtdrv_ helpers and routed the four lifecycle bodies through them. Three four-site invariant blocks: touchmark read-back (zrbtdrv_read_touchmark), divine-contains (zrbtdrv_divine_contains -- returns the divine stdout so podvm layers its cohort-count assertion inline rather than the helper absorbing it), and the banish-and-verify-gone bookend (zrbtdrv_banish_and_verify_gone). Plus the two-site member-jettison block (zrbtdrv_member_jettison_proof, reliquary+podvm, victim/survivor tags as params). The six load-bearing per-kind differences stay inline verbatim per the audit-memo guardrails: capture verb+args, augur member-tag sets, trust grade, lode's literal-HEAD-commit envelope assertion, podvm's refresh+cohort-count sub-sequence and trust-posture prose, and the jettison step's reliquary+podvm-only presence. lode-collision and chaining-livery untouched per cinch. Fail-messages reproduced exactly (divine via verb_label param) so coverage is neutral. Theurge build green under deny(warnings); all four Lode fixtures green standalone (lode-lifecycle 2/2 incl untouched lode-collision, reliquary/wsl/podvm 1/1 each).

### 2026-06-30 17:52 - ₢BlAAI - n

Extract the four Lode round-trip shared blocks into within-module zrbtdrv_ helpers and route the four lifecycle bodies through them. Lifted the three four-site invariant blocks -- touchmark read-back (zrbtdrv_read_touchmark), divine-contains (zrbtdrv_divine_contains, which returns the divine stdout so podvm layers its cohort-count assertion inline), and the banish-and-verify-gone bookend (zrbtdrv_banish_and_verify_gone) -- plus the two-site member-jettison block (zrbtdrv_member_jettison_proof, reliquary+podvm, victim/survivor tags as params). The six load-bearing per-kind differences stay inline verbatim: the capture verb+args, the augur member-tag sets, the trust grade, lode's literal-HEAD-commit envelope assertion, podvm's refresh+cohort-count sub-sequence and trust-posture prose, and the jettison step's reliquary+podvm-only presence. lode-collision and chaining-livery left untouched per cinch. Fail-messages reproduced exactly (divine via a verb_label param), so coverage is neutral. Theurge build green under deny(warnings).

### 2026-06-30 10:59 - ₢BlAAT - W

Revived rbq_qualify's dead colophon-freshness net. Root cause confirmed against the live scan (not just the audit's diagnosis): ZRBQ_RBW_LAUNCHER was constructed as a moorings/launchers FULL PATH, but every RBW tabtarget's line-2 carries the bare BURD_LAUNCHER basename launcher.rbw_workbench.sh since the path-indirection migration, so the `case ${z_lines[1]} in *${full_path}*` substring test matched none of 137 tabtargets -- z_checked stayed 0 and the net silently lifted. Fix: predicate -> bare basename (the literal already lived inside the old full-path string; only the erroneous dir prefix was dropped), plus a comment tying it to BCG Tabtarget Path Indirection so a future reader does not re-add the path. Independently verified the line-2 index assumption (all 137 RBW tabtargets carry the launcher on line 2, uniformly). Results: rbw-tq colophon count 0 -> 137, All 137 registered, exit 0; net PROVEN to fire (dropped a structurally-valid rbw-zzqqproof tabtarget with an unregistered colophon -> WARNING 'not registered' + ERROR '1 of 138' + exit 1, caught at the colophon step past the structural check; artifact removed, tree clean); rbw-tl shellcheck 220 files clean, exit 0. Committed 0feaa5649. rbw-tr's heavy echelon suite deliberately NOT run -- orthogonal to the qualify-net repair, needs live GCP+container infra, depot noted as concurrently contended; operator agreed to skip.

### 2026-06-30 10:57 - ₢BlAAT - n

Repair rbq_qualify's dead colophon-freshness predicate. Confirmed against the live scan: ZRBQ_RBW_LAUNCHER was built as a moorings/launchers FULL PATH, but every RBW tabtarget's line-2 BURD_LAUNCHER carries only the bare basename launcher.rbw_workbench.sh since the path-indirection migration, so the `case ... in *${full_path}*` substring test matched none of 137 RBW tabtargets -- z_checked stayed 0 and the net silently lifted. Fixed the predicate to the bare basename; the line-2 index assumption was verified correct (all 137 RBW tabtargets carry the launcher on line 2, uniformly). Proof-of-fire (net catches a mis-registered colophon) and green qualify run still pending.

### 2026-06-30 10:51 - ₢BlAAM - W

Folded the payor-credential probe-and-gate preamble in rbtdrv_patrol into one zrbtdrv_payor_gate helper with a zrbtdrv_PayorGatePolicy (Skip|Fail) enum carrying the whole per-policy verdict template — the naive (fixture+Skip|Fail) signature was insufficient because the Fail side dumps the probe's stdout/stderr verbatim with no Skip analogue. Helper returns Option<Verdict> (None=green/proceed, Some=gate tripped); callers collapse to `if let Some(v)=...{return v;}`. Fixture name interpolates from the RBTDRM_FIXTURE_* constants so every verdict reproduces byte-for-byte. Routed all FOUR sites, one beyond the docket's named three: Skip-policy terrier pair (terrier-scaffold, terrier-atomicity) and Fail-policy foedus pair (foedus-lifecycle + foedus-reuse — the in-heat fourth copy, operator confirmed including it). Verified: theurge build green under deny(warnings); terrier-scaffold + terrier-atomicity PASS live through the helper's green path. Full picket deliberately NOT run — depot in concurrent use by other chats; the Skip/Fail branches fire only on a non-green probe (unreachable with live creds) and are verbatim-by-construction.

### 2026-06-30 10:43 - ₢BlAAM - n

Fold the quadruplicated payor-credential probe-and-gate preamble in rbtdrv_patrol into one zrbtdrv_payor_gate helper carrying a per-policy verdict template. The naive (fixture + Skip|Fail) signature was insufficient — the Fail side dumps the probe's stdout/stderr verbatim with no Skip analogue — so the zrbtdrv_PayorGatePolicy enum (Skip|Fail) carries the whole per-policy template, fixture name interpolated from the RBTDRM_FIXTURE_* constants so every verdict reproduces byte-for-byte. Helper returns Option<Verdict> (None=green/proceed, Some=gate tripped); callers collapse to `if let Some(v) = ... { return v; }`. Routed all four sites: the Skip-policy terrier pair (terrier-scaffold, terrier-atomicity, auto-suite passenger-protection) and the Fail-policy foedus pair (foedus-lifecycle, foedus-reuse, operator-invoked). Scope expanded one beyond the docket's named three: foedus-reuse landed in this heat (pace foedus-reuse-leg) after the docket was drafted in Bf, an identical fourth Fail-policy copy — operator confirmed including it. Theurge build green under deny(warnings).

### 2026-06-30 10:56 - ₢BlAAH - W

Hoisted the ordain-capture helpers into the shared rbtdri_invocation home and routed every open-coded copy through them. Moved invoke_or_fail (+ its private invoke_logged spine), ordain_capture (single-fact) and ordain_capture_full (three-fact) out of rbtdro_onboarding into rbtdri_invocation, carrying the three ordain fact-name constants (RBTDRV_FACT_* -> RBTDRI_FACT_*, their correct home beside rbtdri_read_burv_fact; after routing they had no other consumer, which also dissolved the would-be rbtdri<->rbtdrv cycle). Added the two deliberately-separate GAR-locator builders per the audit-memo cinch -- rbtdri_gar_ref_categorical (Family A: {category}/{hallmark}/{basename}:{hallmark}) and rbtdri_gar_ref_fact (Family B: {gar_root}/{ark_stem}/{basename}:{hallmark}) -- param-driven so rbtdri imports no rbtdrv constant; never merged. Routed all four sites: dogfight, patrol's hallmark-lifecycle + batch-vouch (the crucible bodies, relocated into rbtdrv_patrol by the earlier module split), and onboarding-internal; the distinct middles (audit/rekon/tally) stay open-coded per the bookend-spine-only guardrail. Coverage-neutral: bespoke Fail prefixes became the helper's structured prefixes; changed scratch-file names are unasserted debug artifacts. Proven: theurge build clean under deny(warnings), 156/156 unit tests, dogfight green (ordain_capture_full + Family B + invoke_or_fail), and the two routed picket members hallmark-lifecycle + batch-vouch green (ordain_capture single + Family A + invoke_or_fail) against live GCP/cloud-build. Commits 220fe911 (lift) + 0608cd58 (rbrd.env depot-drift resync, heat-affiliated). Full picket suite not run to green: blocked by an unrelated pluml blank-hallmark, a separate heat loose-end outside this pace's surface.

### 2026-06-30 10:19 - Heat - n

Resync the frozen depot regime rbrd.env to its inscribed depot copy, clearing the RBRD drift tripwire that blocked the dogfight gate. The drift was comment-only: pace BiAAH's build-bucket-residue cleanup (86c6b849c) edited the live rbrd.env derivation comment ('pool stem, and bucket at kindle.' -> 'and pool stem at kindle.') without re-inscribing, but rbrd.env is depot-time-immutable (frozen after levy per its own header). Restored the comment to match the standing depot's inscribed snapshot so the frozen file is back in sync with its depot; the residue cleanup correctly persists in rbrd_regime.sh's meaning strings, so the next levy renders rbrd.env clean. Functional config values were never in drift. Depot env-state, not the theurge refactor.

### 2026-06-30 10:10 - ₢BlAAH - n

Hoist the ordain-capture helpers into the shared rbtdri_invocation home and route the open-coded copies through them. Moved invoke_or_fail + its private invoke_logged spine + ordain_capture (single) + ordain_capture_full (three-fact) out of rbtdro_onboarding into rbtdri_invocation (rbtdro_->rbtdri_), carrying the three ordain fact-name constants with them (RBTDRV_FACT_HALLMARK/GAR_ROOT/ARK_STEM -> RBTDRI_FACT_*, their correct home beside rbtdri_read_burv_fact; after routing they had no other consumer, which also dissolves the rbtdri<->rbtdrv cycle). Added the two deliberately-separate GAR-locator builders per the audit-memo cinch: rbtdri_gar_ref_categorical (Family A, {category}/{hallmark}/{basename}:{hallmark}) and rbtdri_gar_ref_fact (Family B, {gar_root}/{ark_stem}/{basename}:{hallmark}) -- param-driven so rbtdri imports no rbtdrv constants; never merged. Routed all four open-coded sites: dogfight (ordain+3fact -> capture_full, image_ref -> Family B, summon + abjure -> invoke_or_fail), patrol's hallmark-lifecycle + batch-vouch crucible bodies (ordain opener -> capture, abjure closer -> invoke_or_fail, batch-vouch vouch locator -> Family A; middles left open-coded per the bookend-spine-only guardrail), and onboarding-internal (every locator format! through the two builders, helper calls repointed). Coverage-neutral: bespoke Fail prefixes become the helper's structured prefixes; changed scratch-file names are unasserted debug artifacts. Theurge build green under deny(warnings).

### 2026-06-30 09:44 - ₢BlAAG - W

Removed the dead rbtdri_invoke_ifrit (zero callers — every live invoke goes through the rbtdrc pair) and its orphaned scaffolding: the duplicate RBTDRI_IFRIT_BINARY const mirroring the live RBTDRC_IFRIT_BINARY, and the RBTDGC_CRUCIBLE_BARK import whose sole use was inside the removed function. One ifrit-invoke home (rbtdrc_invoke_ifrit pair) and one rbid const (RBTDRC_IFRIT_BINARY) remain; rbtdri_parse_ifrit_verdict and the rest of rbtdri_invocation stay live (rbtdre_Verdict import retained for it). Theurge build green under deny(warnings); 156/156 unit tests; tadmor fixture 61/61, exercising the live rbtdrc_invoke_ifrit path end-to-end. Full bivouac literal-green not run on this darwin station — blocked by pre-existing blank sentry/bottle hallmarks on 5 nameplates (ccyolo/fdkyclk/moriah/pluml/srjcl, two of them bivouac members), unrelated to this removal; tadmor stands as the substantive ifrit-path coverage.

### 2026-06-30 09:30 - ₢BlAAG - n

Remove the dead rbtdri_invoke_ifrit and its now-orphaned scaffolding, leaving rbtdrc_invoke_ifrit as the single ifrit-invoke home. The function had zero callers (every live invoke goes through the rbtdrc pair in rbtdrc_crucible.rs); its sole dependencies died with it: the duplicate RBTDRI_IFRIT_BINARY const ("rbid", mirroring the live RBTDRC_IFRIT_BINARY) and the RBTDGC_CRUCIBLE_BARK import (only use was inside the removed function). rbtdri_parse_ifrit_verdict and the rest of rbtdri_invocation stay live; rbtdre_Verdict import retained for it. One ifrit-invoke home and one rbid const remain.

### 2026-06-30 09:25 - ₢BlAAZ - W

Collapsed the per-fixture registration-mirror unit tests into compile-time const-guards at the fixture declarations. Replaced 12 runtime mirror tests (the *_cases_registered count-mirrors and *_disposition_* tag-mirrors across rbtdtk/rbtdtp/rbtdto/rbtdtl) with 8 const _: () = assert!(FIXTURE.cases.len() == N) co-located with each fixture static (freehold-establish 6 / churn 1, depot-lifecycle 4, onboarding-sequence 8, calibrant 4/3/2/1); their passing compilation is the count proof. Case-name literals deleted entirely (case!(ident) derives names from compile-checked paths). Disposition mirrors deleted outright rather than converted — a declaration-site assert restating the adjacent literal is tautology, and disposition x keep-going behavior is covered generically in rbtdte_engine. Removed the now-dead rbtdth_assert_cases/rbtdth_assert_disposition helpers (superseding the prior pace's fold) and trimmed imports; rbtdtp_lifecycle.rs held only its two mirror tests so the file + its lib.rs mod decl were removed. Kept genuinely-semantic tests: onboarding conclave-ordering, calibrant per-case verdict/sentinel/probe + required-colophons, freehold composition/rejection. Open resolved: no bash blackbox testbench exists in-repo and it asserts engine-output (exit codes, stderr, sentinel globbing), not the case-name roster, so the mirrors had no external reader. Theurge build green under deny(warnings); 156/156 unit tests pass (was 168, minus the 12 deleted mirrors).

### 2026-06-30 09:23 - ₢BlAAZ - n

Collapse the per-fixture registration-mirror unit tests into compile-time const-guards at the fixture declarations. Replace the runtime *_cases_registered count-mirrors and *_disposition_* tag-mirrors (rbtdtk/rbtdtp/rbtdto/rbtdtl) with `const _: () = assert!(FIXTURE.cases.len() == N)` co-located with each fixture static: freehold-establish 6 / churn 1 (rbtdrk_depot), depot-lifecycle 4 (rbtdrp_lifecycle), onboarding-sequence 8 (rbtdro_onboarding), calibrant verdicts/fail-fast/progressing/sentinel 4/3/2/1 (rbtdrl_calibrant). Case-NAME literals deleted entirely — case!(ident) already derives names from compile-checked paths, so only the count was non-redundant. Disposition mirrors deleted outright rather than converted: a declaration-site assert restating the adjacent literal is tautology, and disposition x keep-going behavior is already covered generically in rbtdte_engine. Removed the now-dead rbtdth_assert_cases/rbtdth_assert_disposition helpers (every call site gone, prior pace's fold superseded) and trimmed unused imports across the four test modules + helpers. rbtdtp_lifecycle.rs held only its two mirror tests -> file removed with its lib.rs mod decl. Kept the genuinely-semantic runtime tests: onboarding conclave-ordering, calibrant per-case verdict/sentinel/probe paths + required-colophons, freehold composition/rejection. Open resolved: no bash blackbox testbench exists in-repo, and by its own contract it asserts engine-output (exit codes, stderr, sentinel globbing), not the registered case-name roster -- the mirrors had no external reader. Theurge build green under deny(warnings).

### 2026-06-30 09:07 - ₢BlAAF - W

Hoisted duplicated theurge test boilerplate into rbtdth_helpers: registration-triplet assert pair (rbtdth_assert_disposition + rbtdth_assert_cases) folding the lookup/projection idiom across rbtdtk/rbtdtp/rbtdto/rbtdtl (disposition stays a parameter, calibrant's Independent fixtures preserved, rbtdtl count-only via empty needle slice), plus rbtdth_make_scratch (pid+nanos+preclean) folding the triplicated make_temp across rbtdte/rbtdti/rbtdtl. rbtdtk's nonexistent-path and rbtdti's tt-staging left alone per cinch. RCG import discipline applied. 168/168 unit tests green under deny(warnings).

### 2026-06-30 09:04 - ₢BlAAF - n

Hoist duplicated theurge test boilerplate into rbtdth_helpers. Two registration-triplet assert helpers (rbtdth_assert_disposition + rbtdth_assert_cases) fold the lookup/projection idiom across rbtdtk/rbtdtp/rbtdto/rbtdtl; disposition stays a PARAMETER so calibrant's three Independent fixtures survive, and the rbtdtl cases asserts pass an empty needle slice (count-only, unchanged). One scratch-dir maker (rbtdth_make_scratch, strongest pid+nanos+preclean recipe) folds the triplicated make_temp across rbtdte/rbtdti/rbtdtl; call sites collapsed, the three local make_temp impls deleted. Left untouched per cinch: rbtdtk's deliberately-nonexistent rbrr path and rbtdti's tt-staging helpers. RCG import discipline (one-per-line, alphabetical) applied to the new rbtdth_helpers imports.

### 2026-06-30 08:58 - Heat - S

registration-mirror-to-const-guard

### 2026-06-27 14:52 - ₢BlAAE - W

Built the day-to-day REUSE credential path on the interactive Entra foedus end to end. Two first-class atomic production verbs in the new rbof module (rbw-j family): descry (rbw-jd, read-only Manor pool-health probe, band 106, reporting healthy/pool-absent/pool-deleted/provider-absent via a foedus-health fact) and instate (rbw-jI, atomic RBRR_ACTIVE_FOEDUS rewrite via the feoff idiom, band 107, no clean-tree gate, operator commits) — both built against their RBSFD/RBSFI subdocs and live-validated on-station (descry read the real pool HEALTHY; instate idempotent; bands + foedera-listing correct). Under them a THIN foedera-library substrate (rbmm_moorings/rbmf_foedera/rbef_entrada/rbrf.env; RBCC_rbrf_file resolves into it at the sole foedus as a degenerate single-foedus literal; RBRR_ACTIVE_FOEDUS enrolled) — the heavy family-of-named-instances rework stays deferred per the 06-23 ruling, so no consumer rewire. The foedus-reuse theurge fixture composes descry -> reuse-if-healthy-else-affiance (branch at the fixture, verbs stay atomic, cap-flat reuse, affiance only on check-failure) -> instate -> avow + don governor/director/retriever, the standing-freehold credential leg skirmish/dogfight/blockade assumed but no fixture established. New mints: rbmf_foedera dir, rbof module, bands 106/107. Green on every axis the pace owns: shellcheck 219 clean, theurge build + 168/168 unit tests, RBRR/RBRF regime validation, regime-poison 30/30, live verb validation.

### 2026-06-27 14:48 - Heat - n

Re-kludge tadmor's bottle hallmark — drove RBRN_BOTTLE_HALLMARK=k260627144807-e8fdff423 from a fresh local bottle kludge (rbev-bottle-ifrit-tether). Completes the tadmor nameplate re-population (sentry + bottle) so the nameplate validation in reveille/regime-poison clears its non-empty hallmark gate. Heat test-infrastructure state, distinct from the foedus pace.

### 2026-06-27 14:48 - Heat - n

Re-kludge tadmor's sentry hallmark to lift the blank-hallmark interim state that masked the nameplate validation in reveille/regime-poison. Drove RBRN_SENTRY_HALLMARK=k260627144704-9b46b1a4c from a fresh local sentry kludge (rbev-sentry-deb-tether). Heat test-infrastructure state, not the foedus pace's work — committed separately so the clean-tree gate lets the bottle kludge follow.

### 2026-06-27 14:42 - ₢BlAAE - n

Register the pace's two new persistent names in the RBK acronym map: RBOF (the rbof_foedus.sh foedus-cardinality module — descry/instate, the rbw-j colophon family, bands 106/107, contracts RBSFD/RBSFI) and the rbmf_ moorings-family branch (the foedera library rbmf_foedera/ holding one rbef_ subdirectory per standing foedus, RBCC_rbrf_file resolving into it at the active foedus).

### 2026-06-27 14:40 - ₢BlAAE - n

Add the foedus-reuse theurge fixture — the standing-freehold REUSE credential leg skirmish/dogfight/blockade assume but no fixture established. A single operator-invoked case composes the two new atoms with the credential heal: payor precondition (Fail not Skip — never a passenger), read the committed RBRR_ACTIVE_FOEDUS selector, descry the standing foedus (probe exit 0 + read the foedus-health fact), then the reuse-or-establish BRANCH at the fixture call site (the verbs stay atomic) — reuse cap-flat when healthy with no affiance/pool churn, affiance only on a descry deficit — then instate (idempotent on the active one), then heal credentials: avow the sitting once, don governor/director/retriever and reach AR (rbw-acm). Unlike foedus-lifecycle it touches the REAL standing foedus (no regime-poison, no throwaway pool) so it is quota-neutral on the reuse path; still a member of no suite (heals against a STANDING freehold, does not provision one). Registered in the manifest (name + required-colophons), patrolled next to foedus-lifecycle, and discovery-listed in the almanac. Theurge build green.

### 2026-06-27 14:31 - ₢BlAAE - n

Build descry and instate as first-class atomic production tabtargets in the new rbof foedus-cardinality module (colophon family rbw-j). instate (rbw-jI) re-points the RBRR_ACTIVE_FOEDUS selector via the feoff/yoke/anoint atomic single-field rewrite — temp+rename, no clean-tree gate, no commit, dies if the enrolled selector line is absent (RBSFI). descry (rbw-jd) parses a named foedus's own rbrf.env (never sourcing it — the active foedus is kindled readonly), authenticates as payor (reusing zrbgp_authenticate_capture, the same credential affiance/jilt use for the org-level pool), GETs the workforce pool + provider, and reports health (healthy / pool-absent / pool-deleted / provider-absent) via a foedus-health fact file for the reuse-or-establish fixture to branch on; a deficit is a reported verdict (exit 0), only an unresolvable name or broken read rejects in descry's own band (RBSFD). Each verb rejects in its own precision band (descry 106, instate 107 — minted in bubc, distinct because they co-occur in the fixture spawn path) and fails a bad/missing identity by listing the discovered foedera. rbof_cli per-command furnish keeps instate light (no payor creds) and pulls the IAM-REST stack only for descry. Validated end-to-end on the station: instate bands+listing+idempotent rewrite, descry bands+listing+live Manor read reporting HEALTHY for rbef_entrada. Shellcheck 219 clean, theurge build green, generated consts/context regenerated from the zipper.

### 2026-06-27 14:14 - ₢BlAAE - n

Establish the thin foedera-library substrate the descry/instate verbs read and write. Mint the moorings foedera library rbmm_moorings/rbmf_foedera/ and relocate the single flat federation regime into it as rbef_entrada/rbrf.env (the contract's stored-once, no-copied-active-file shape); repoint RBCC_rbrf_file into the library at the sole standing foedus as a DEGENERATE single-foedus literal (selector-derived resolution defers with the federation family-of-named-instances rework, so the ~11 singleton accessors read the constant unchanged and no per-consumer rewire is needed); enroll the RBRR_ACTIVE_FOEDUS selector (xname) in rbrr_regime.sh with an rbef_ sprue enforce check, and seed rbrr.env to rbef_entrada. Both affected regimes validate green (rbw-rrv, rbw-rfv) at the new path.

### 2026-06-27 13:15 - ₢BlAAR - W

Cross-clone barrier cleared. ₢BfAAT (heat ₣Bf) authored the foedus test-bed switching contract — operation subdocs RBSFD (descry) and RBSFI (instate) plus their RBS0 includes and operation quoins (rbtgo_foedus_descry / rbtgo_foedus_instate) — wrapped at 9f9c9f09e, and a git pull --rebase merged origin into this clone. Verified the 'Done when' on all three counts: BfAAT's :W: wrap commit is in-tree, both .adoc subdocs are present in Tools/rbk/vov_veiled/, and RBS0 carries the include::RBSFD/RBSFI lines (1243/1259) with both quoins wired in the mapping section. No code work of its own; the descry/instate REUSE-path build pace may now proceed against the settled subdocs (contract before code).

### 2026-06-27 14:52 - ₢BlAAE - W

Built the day-to-day REUSE credential path on the interactive Entra foedus end to end. Two first-class atomic production verbs in the new rbof module (rbw-j family): descry (rbw-jd, read-only Manor pool-health probe, band 106, reporting healthy/pool-absent/pool-deleted/provider-absent via a foedus-health fact) and instate (rbw-jI, atomic RBRR_ACTIVE_FOEDUS rewrite via the feoff idiom, band 107, no clean-tree gate, operator commits) — both built against their RBSFD/RBSFI subdocs and live-validated on-station (descry read the real pool HEALTHY; instate idempotent; bands + foedera-listing correct). Under them a THIN foedera-library substrate (rbmm_moorings/rbmf_foedera/rbef_entrada/rbrf.env; RBCC_rbrf_file resolves into it at the sole foedus as a degenerate single-foedus literal; RBRR_ACTIVE_FOEDUS enrolled) — the heavy family-of-named-instances rework stays deferred per the 06-23 ruling, so no consumer rewire. The foedus-reuse theurge fixture composes descry -> reuse-if-healthy-else-affiance (branch at the fixture, verbs stay atomic, cap-flat reuse, affiance only on check-failure) -> instate -> avow + don governor/director/retriever, the standing-freehold credential leg skirmish/dogfight/blockade assumed but no fixture established. New mints: rbmf_foedera dir, rbof module, bands 106/107. Green on every axis the pace owns: shellcheck 219 clean, theurge build + 168/168 unit tests, RBRR/RBRF regime validation, regime-poison 30/30, live verb validation.

### 2026-06-27 14:48 - Heat - n

Re-kludge tadmor's bottle hallmark — drove RBRN_BOTTLE_HALLMARK=k260627144807-e8fdff423 from a fresh local bottle kludge (rbev-bottle-ifrit-tether). Completes the tadmor nameplate re-population (sentry + bottle) so the nameplate validation in reveille/regime-poison clears its non-empty hallmark gate. Heat test-infrastructure state, distinct from the foedus pace.

### 2026-06-27 14:48 - Heat - n

Re-kludge tadmor's sentry hallmark to lift the blank-hallmark interim state that masked the nameplate validation in reveille/regime-poison. Drove RBRN_SENTRY_HALLMARK=k260627144704-9b46b1a4c from a fresh local sentry kludge (rbev-sentry-deb-tether). Heat test-infrastructure state, not the foedus pace's work — committed separately so the clean-tree gate lets the bottle kludge follow.

### 2026-06-27 14:42 - ₢BlAAE - n

Register the pace's two new persistent names in the RBK acronym map: RBOF (the rbof_foedus.sh foedus-cardinality module — descry/instate, the rbw-j colophon family, bands 106/107, contracts RBSFD/RBSFI) and the rbmf_ moorings-family branch (the foedera library rbmf_foedera/ holding one rbef_ subdirectory per standing foedus, RBCC_rbrf_file resolving into it at the active foedus).

### 2026-06-27 14:40 - ₢BlAAE - n

Add the foedus-reuse theurge fixture — the standing-freehold REUSE credential leg skirmish/dogfight/blockade assume but no fixture established. A single operator-invoked case composes the two new atoms with the credential heal: payor precondition (Fail not Skip — never a passenger), read the committed RBRR_ACTIVE_FOEDUS selector, descry the standing foedus (probe exit 0 + read the foedus-health fact), then the reuse-or-establish BRANCH at the fixture call site (the verbs stay atomic) — reuse cap-flat when healthy with no affiance/pool churn, affiance only on a descry deficit — then instate (idempotent on the active one), then heal credentials: avow the sitting once, don governor/director/retriever and reach AR (rbw-acm). Unlike foedus-lifecycle it touches the REAL standing foedus (no regime-poison, no throwaway pool) so it is quota-neutral on the reuse path; still a member of no suite (heals against a STANDING freehold, does not provision one). Registered in the manifest (name + required-colophons), patrolled next to foedus-lifecycle, and discovery-listed in the almanac. Theurge build green.

### 2026-06-27 14:31 - ₢BlAAE - n

Build descry and instate as first-class atomic production tabtargets in the new rbof foedus-cardinality module (colophon family rbw-j). instate (rbw-jI) re-points the RBRR_ACTIVE_FOEDUS selector via the feoff/yoke/anoint atomic single-field rewrite — temp+rename, no clean-tree gate, no commit, dies if the enrolled selector line is absent (RBSFI). descry (rbw-jd) parses a named foedus's own rbrf.env (never sourcing it — the active foedus is kindled readonly), authenticates as payor (reusing zrbgp_authenticate_capture, the same credential affiance/jilt use for the org-level pool), GETs the workforce pool + provider, and reports health (healthy / pool-absent / pool-deleted / provider-absent) via a foedus-health fact file for the reuse-or-establish fixture to branch on; a deficit is a reported verdict (exit 0), only an unresolvable name or broken read rejects in descry's own band (RBSFD). Each verb rejects in its own precision band (descry 106, instate 107 — minted in bubc, distinct because they co-occur in the fixture spawn path) and fails a bad/missing identity by listing the discovered foedera. rbof_cli per-command furnish keeps instate light (no payor creds) and pulls the IAM-REST stack only for descry. Validated end-to-end on the station: instate bands+listing+idempotent rewrite, descry bands+listing+live Manor read reporting HEALTHY for rbef_entrada. Shellcheck 219 clean, theurge build green, generated consts/context regenerated from the zipper.

### 2026-06-27 14:09 - Heat - d

batch: 1 reslate

### 2026-06-27 13:08 - Heat - S

terrier-poison-fixture

### 2026-06-27 13:07 - Heat - S

parley-authentic-verb-fixture

### 2026-06-27 14:14 - ₢BlAAE - n

Establish the thin foedera-library substrate the descry/instate verbs read and write. Mint the moorings foedera library rbmm_moorings/rbmf_foedera/ and relocate the single flat federation regime into it as rbef_entrada/rbrf.env (the contract's stored-once, no-copied-active-file shape); repoint RBCC_rbrf_file into the library at the sole standing foedus as a DEGENERATE single-foedus literal (selector-derived resolution defers with the federation family-of-named-instances rework, so the ~11 singleton accessors read the constant unchanged and no per-consumer rewire is needed); enroll the RBRR_ACTIVE_FOEDUS selector (xname) in rbrr_regime.sh with an rbef_ sprue enforce check, and seed rbrr.env to rbef_entrada. Both affected regimes validate green (rbw-rrv, rbw-rfv) at the new path.

### 2026-06-27 13:15 - ₢BlAAR - W

Cross-clone barrier cleared. ₢BfAAT (heat ₣Bf) authored the foedus test-bed switching contract — operation subdocs RBSFD (descry) and RBSFI (instate) plus their RBS0 includes and operation quoins (rbtgo_foedus_descry / rbtgo_foedus_instate) — wrapped at 9f9c9f09e, and a git pull --rebase merged origin into this clone. Verified the 'Done when' on all three counts: BfAAT's :W: wrap commit is in-tree, both .adoc subdocs are present in Tools/rbk/vov_veiled/, and RBS0 carries the include::RBSFD/RBSFI lines (1243/1259) with both quoins wired in the mapping section. No code work of its own; the descry/instate REUSE-path build pace may now proceed against the settled subdocs (contract before code).

### 2026-06-27 07:48 - Heat - S

rekon-chaining-band-case

### 2026-06-26 10:55 - Heat - f

silks=rbk-02-mvp-review-theurge-refactor

### 2026-06-26 11:59 - ₢BlAAD - W

Added the RBTDRA_REVEILLE_BASE compile-time guard in rbtdra_almanac.rs: an independent literal of the canonical 11-fixture reveille set, against which reveille is held to set-equality and picket/bivouac/echelon/gauntlet/skirmish to superset (dogfight/siege/blockade deliberately base-free). Restored chaining-fact-band (credless feoff/yoke band-matrix fixture) to gauntlet and skirmish after conformance, closing the audit X-d conformance hole per operator ruling 260623. Lists stay literal (no concat). Negative-tested the oracle has teeth: re-dropping the member breaks the build with a named const-eval panic. Theurge build green, all 168 unit tests pass (ladder_containment + model_classifies_every_suite corroborate). Notched 745ff368.

### 2026-06-26 11:57 - ₢BlAAD - n

Add a compile-time RBTDRA_REVEILLE_BASE guard pinning the canonical 11-fixture reveille set as a regression oracle: an independent literal (never spliced into a suite) against which reveille is held to set-equality and picket/bivouac/echelon/gauntlet/skirmish to superset; dogfight/siege/blockade are deliberately base-free bearers named nowhere. Restore chaining-fact-band (the credless feoff/yoke band-matrix fixture) to the gauntlet and skirmish ladders, placed after conformance with the reveille block (audit 260623 finding X-d; operator ruling 260623 = restore). Negative-tested: dropping the member breaks the build with a named const-eval panic. Theurge build green.

### 2026-06-26 10:56 - Heat - f

racing

### 2026-06-26 10:51 - ₢BlAAC - W

Split the rbtdrc_crucible god-module (4680 lines) into three: rbtdra_almanac holds the fixture/suite registries + both lookups + a new compile-time const guard rejecting duplicate fixture/suite names (RBTDRA_*); rbtdrv_patrol holds the 11 bare no-charge cloud fixtures + the shared ark/GAR/docker-helper cluster (RBTDRV_*); rbtdrc_crucible is left as the charge/quench lifecycle remainder (context machinery, ifrit/sentry/dns/curl/srjcl/pluml helpers+cases, the 4 crucible fixture statics, darken_svg+tests). Symbols renamed to their new module prefixes (grep-gated: almanac/patrol clean; roster/rota/muster/census/cohort/assay/vigil/sortie/foray rejected as taken or ungreppable). dogfight/onboarding/main/4 test-modules repointed; registry refs qualified. Build + 168 unit tests green, suite-oracle confirms byte-unchanged composition; re-verified green after a 12-commit rebase. Committed 1decc48, pushed as 68ca7e99d.

### 2026-06-26 10:29 - ₢BlAAC - n

Split the rbtdrc_crucible god-module: extract the fixture/suite registries + lookups into rbtdra_almanac (RBTDRA_*) and the 11 bare cloud fixtures + shared ark/GAR/docker cluster into rbtdrv_patrol (RBTDRV_*), leaving crucible as the charge/quench lifecycle remainder. Symbols renamed to their new module prefixes; dogfight/onboarding/main/test-modules repointed. Added a compile-time const guard rejecting duplicate fixture/suite names. Theurge build green.

### 2026-06-26 09:26 - Heat - d

batch: 1 reslate

### 2026-06-26 07:53 - Heat - n

Re-blank the 6 nameplate hallmarks (RBRN_SENTRY_HALLMARK / RBRN_BOTTLE_HALLMARK) that the prior 15-file restore wrongly un-blanked. The restored hallmarks were 06-11 builds absent from the current standing depot 100002 (GAR tally shows only 06-21/06-24 hallmarks); pluml charge proved them unresolvable. Blank is the honest interim state — a later onboarding-sequence re-ordains fresh hallmarks against 100002. Vessel reliquary/anchor restores stay (dogfight-proven valid).

### 2026-06-26 07:39 - Heat - n

Restore the remaining 9 vessel rbrv.env + 6 nameplate rbrn.env files to pre-marshal-zero state (git-recovered from 7eafd12f7), undoing marshal-zero's blanking of RBRV_RELIQUARY/RBRV_IMAGE_*_ANCHOR and RBRN_*_HALLMARK. With the standing depot pointer (canest3bhm100002) already restored, the working environment is now whole again. Each file was touched only by the marshal-zero commit since 7eafd12f7, so a wholesale revert of these 15 undoes exactly the blanking.

### 2026-06-26 07:23 - Heat - n

Restore busybox vessel RBRV_RELIQUARY=r260621114527 (git-recovered, blanked by marshal-zero) so the dogfight conjure can resolve its build-tooling Lode against the standing depot's GAR. Anchor stays blank (busybox is tether-mode, conjures from public busybox:latest origin).

### 2026-06-26 07:18 - Heat - n

Restore standing depot pointer (RBRD_DEPOT_MONIKER=canest3bhm100002) after the depot-lifecycle validation run left it at the tear-down placeholder. Verified canest3bhm100002 COMPLETE in the live depot list; the throwaway leasehold canest3bhm100003 the run minted+tore down is DELETE_REQUESTED.

### 2026-06-26 07:09 - ₢BlAAB - W

Lifted the depot wrapper machinery: factored the shared stand-up cross-check tail (rbtdrk_crosscheck_project_id), depot fact-path builders (rbtdrk_depot_fact_dir/_path), and unmake preamble (rbtdrk_unmake_preamble) into rbtdrk_freehold, parameterized REUSE-vs-LIFECYCLE. Kept the structurally-inverted tear_down (fail-closed allowlist) and churn (fail-open denylist) post-unmake assertions un-merged at their call sites per cinch. Added rbtdtc_crucible oracle: first-class wrapper(inner) model verified against the literal RBTDRC_SUITES membership matrix (REUSE product completeness echelon==picket∪bivouac, ladder containment blockade⊆skirmish⊆gauntlet); suite lists stay literal. RCG-compliant (one-per-line imports, pub(crate) fields, no memo/audit refs in comments).

### 2026-06-26 10:10 - Heat - n

JJK core: add gazette-discipline guard — gazette_in.md is the argument to the next jjx call, never decouple a gazette write from its call (deleted-on-entry means read-on-entry), with a strong asymmetric-stakes warning. Membrane against the mis-moded-setter spook hit while refining the canvass dockets.

### 2026-06-26 09:25 - Heat - d

paddock curried: add canvass read-all member to the foedus-cardinality shape note

### 2026-06-26 09:17 - Heat - d

batch: 2 reslate

### 2026-06-26 08:33 - Heat - d

batch: 2 reslate

### 2026-06-26 08:29 - Heat - d

paddock curried: restore paddock content erroneously emptied by an empty-body setter call earlier this officium

### 2026-06-26 08:25 - Heat - d

paddock curried

### 2026-06-26 08:06 - Heat - S

foedus-canvass-test

### 2026-06-26 08:05 - Heat - S

foedus-canvass-build

### 2026-06-26 07:20 - Heat - f

stabled, silks=rbk-42-fbl-review-theurge-refactor

### 2026-06-26 07:05 - Heat - f

silks=rbk-04-mvp-theurge-refactor

### 2026-06-25 15:43 - ₢BlAAB - n

Lift depot wrapper machinery: factor shared stand-up cross-check tail, fact-path builders, and unmake preamble into rbtdrk_freehold helpers parameterized REUSE-vs-LIFECYCLE; keep the structurally-inverted tear-down (fail-closed allowlist) and churn (fail-open denylist) post-unmake assertions un-merged at call sites. Add rbtdtc_crucible oracle: first-class wrapper(inner) model verified against the suite membership matrix (REUSE product completeness, ladder containment), lists stay literal.

### 2026-06-25 16:50 - Heat - S

qualify-colophon-check-revive

### 2026-06-25 16:30 - Heat - d

paddock curried: integrity-audit fix: descry/instate signature+casing+selection-mechanism attributed to the federation stream's design pass (resolving the L100/102/104 vs Provenance self-contradiction); the build pace consumes settled subdocs

### 2026-06-25 16:26 - Heat - d

batch: 3 reslate

### 2026-06-25 06:01 - Heat - d

batch: 1 reslate

### 2026-06-25 06:01 - Heat - T

create-skimpy-theurge-subdoc

### 2026-06-24 23:33 - Heat - S

await-fed-rbs0-block

### 2026-06-24 23:33 - Heat - S

await-fed-foedus-subdocs

### 2026-06-24 23:32 - Heat - T

thg-syncs-federation-rbs0-foedus

### 2026-06-24 23:18 - Heat - f

silks=rbk-14-mvp-theurge-refactor

### 2026-06-24 23:12 - Heat - S

thg-syncs-federation-rbs0-foedus

### 2026-06-24 22:59 - Heat - d

paddock curried: fresh charter for the relabelled+un-stabled theurge-refactor stream (split study): crucible-module hotspot shape, cosmology supersession with parley-characterization requirement, descry/instate REUSE-path toothings, theurge cluster restrung in from Bf + crucible escapee from Bi

### 2026-06-24 22:44 - Heat - f

racing, silks=rbk-15-mvp-theurge-refactor

### 2026-06-24 22:44 - Heat - D

restring 1 paces from ₣Bi

### 2026-06-22 07:57 - ₢BlAAA - n

Correct theurge spec naming (RBST0 was a leftover sibling-top-spec name; it is a proper RBS0 SUBDOC, RBS+tail, include'd) per operator catch; graduate the rbst0 itch to slated pace BlAAA (remove the itch); repoint the memo's theurge-subdoc item at the pace; add the pre-wrap-audit RBSMA acronym-gap note to the jilt-mint row

### 2026-06-22 07:55 - Heat - S

create-skimpy-theurge-subdoc

### 2026-06-22 07:54 - Heat - N

rbk-15-theurge-cosmology-spec

