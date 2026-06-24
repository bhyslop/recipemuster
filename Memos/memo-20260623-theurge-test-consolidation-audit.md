# Theurge Test-Consolidation Audit

*Memo — 2026-06-23 — RBK theurge test-orchestration crate (`Tools/rbk/rbtd/`)*

---

## Charge

This memo is the final assembly of a read-only audit of the RBK theurge
test-orchestration crate (`Tools/rbk/rbtd/`). It maps duplication across the
`rbtdr*` / `rbtdt*` modules, classifies every fixture by dependency stratum,
identifies the already-built substrate-lifecycle machinery the proposed
inner×wrapper abstraction must *lift* (not rebuild), and slates the
dependency-ordered repair paces.

**Posture: audit, not mutate.** Every item below is a *proposal*. Nothing in the
crate was edited during this audit. The governing rule throughout: **a wrong
merge that collapses a load-bearing difference is strictly worse than a missed
dedup.** Every confirmed fold either preserves the differing assertions as
call-site code or carries them as an explicit parameter — never drops them.
Items the verification phase refuted are reported as refutations, not folds —
because a refutation is itself a load-bearing finding (it names coverage that a
naive merge would silently lose).

**Scope boundary.** IN SCOPE: the `rbtdr*` (production) and `rbtdt*` (unit-test)
modules of the theurge crate, plus the BUK self-test bodies (`butcfc_facts.sh`)
that exercise the shared fact-chaining discipline. OUT OF SCOPE (not entered):
the feoff verb, `rbldk_kind.sh`, the rbfd/rbfl/rbld product refactors, the
`rbgft_`/RBGFT terrier verbs, the JJK MCP viewer, and the local-inference memo.

**This memo is provenance, not authority.** Durable facts — the cosmology
vocabulary, the strata, the inner×wrapper product, the freehold composite — do
not live here as their permanent home. They graduate to the RBS0 theurge-cosmology
subdoc drafted in this memo (the `RBSTC-TheurgeCosmology.adoc` skimpy draft below,
target home `Tools/rbk/vov_veiled/RBSTC-TheurgeCosmology.adoc`). A memo retires;
a spec is cited. The temptation to home this knowledge in the memo is itself the
signal that formal specification is due — which is exactly why the final pace in
the slate is the subdoc finalization.

All file references are absolute under
`/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtd/src/` unless noted.
Line numbers are mount-time-fragile and given only as entry points.

---

## The target model

The audit yardstick is a single durable **freehold** = (depot / GCP project) +
(foedus / workforce pool) + (terrier / muniment store) — one substrate, reused
day-to-day and create→destroyed only at release. Over that substrate sit two
**wrappers** and three **strata**.

**The three strata** (a fixture's dependency class — its *substrate need*,
distinct from which suite lists it):

- **Stratum 1 — Substrate-Independent.** The always-runs credless/fast base.
  Needs no depot, pool, container, or credential. NOT part of the inner×wrapper
  product. Carries `disposition: Independent, credless: true`.
- **Stratum 2 — Freehold-Dependent Inner Body.** The left term of the product.
  A fixture that exercises the one standing freehold (GCP-service fixtures, the
  depot-using crucible) and runs identically under EITHER wrapper. It tests the
  substrate; it never creates or destroys it.
- **Stratum 3 — Wrapper Machinery.** The create / churn / teardown OF the
  substrate — not a test OF it. depot-lifecycle, freehold-establish/churn, the
  foedus lifecycle, the terrier scaffold, and the credential heal all live here.

**The two wrappers** (the two ways an inner body runs against the substrate):

- **REUSE wrapper** = run against a standing freehold, no create/destroy. The
  day-to-day posture. Setup is freehold-establish's reuse branch (reuse an ACTIVE
  depot, idempotently re-heal the mantles, spend no workforce-pool quota);
  teardown is none.
- **LIFECYCLE wrapper** = create the substrate, run, destroy it — the
  create→destroy proof. The release posture. Setup mints a fresh *leasehold*
  (always above the standing freehold, so teardown never reaches it); teardown is
  the strict fail-closed unmake of the used leasehold.

**The release lifecycle** is the throwaway-leasehold-then-durable-freehold
ordering carried by the gauntlet ladder: mint a leasehold and tear it down
(proving teardown of a dirtied install), THEN establish a durable freehold and
leave it standing for the REUSE suites. A runnable **suite** is, in this model,
one inner body × one wrapper — a *product* that is implicit today (spelled by
hand in `RBTDRC_SUITES` plus branch logic inside the depot fixtures) and whose
lift to a first-class combinator is the audit's core work.

---

## Key finding — the substrate lifecycle already exists

The substrate-lifecycle half is built and works; the novel work is the
**wrapper abstraction**, not the machinery.

What is already shipped, to be **lifted not rebuilt**:

- `rbtdrk_freehold.rs` — the single scheme home: identities, tincture
  composition, `pick_next_moniker` (the safety boundary that always mints above
  the standing freehold), `install_freehold_prefixes` / `install_depot_moniker`
  committing through the `rbtdre_engine` config-console.
- `rbtdrk_depot.rs` — `freehold-establish` (with its reuse-or-create branch),
  `freehold-churn`.
- `rbtdrp_lifecycle.rs` — `depot-lifecycle` (the throwaway leasehold
  create→destroy proof).
- The **gauntlet ladder** that sequences them: marshal-zero attest →
  depot-lifecycle (throwaway) → freehold-establish (durable, left standing) →
  onboarding → fast → crucibles. The two-depot throwaway-then-durable release
  shape *already lives* in that fixture ordering.

**The engine already has the channel.** `rbtdre_Fixture` carries
`setup: Option<fn() -> Result<(), String>>` and `teardown: Option<fn()>`,
dispatched by `rbtdre_run_fixture` in a finally-shape (setup failure
short-circuits cases but teardown still runs; teardown errors surface as
warnings, never panic). The crucible fixtures already use it:
`setup: Some(rbtdrc_charge_crucible)`, `teardown: Some(rbtdrc_quench_crucible)` —
one charge verb, fixture-name-parameterized via `ctx.fixture()`. **This is the
wrapper combinator's substrate, already proven.** The mapping is exact: REUSE =
`setup: Some(freehold_ensure)`, `teardown: None`; LIFECYCLE =
`setup: Some(freehold_stand_up_fresh)`, `teardown: Some(freehold_tear_down)`.

Today these two postures live as branch logic cloned across two near-identical
fixtures: `freehold_ensure_impl`'s reuse-vs-create branch *is* the REUSE wrapper,
`stand_up_impl` + `tear_down_impl` *is* the LIFECYCLE wrapper. The lift does not
invent a mechanism; it generalizes the hook channel crucibles already prove works.

---

## Inventory summary

Fixtures grouped by stratum (straddlers flagged; a straddler carries a primary
stratum plus an explicit secondary role).

### Stratum 1 — Substrate-Independent (the always-runs base)

`enrollment-validation`, `regime-validation`, `regime-smoke`,
`dockerfile-hygiene`, `foundry-path`, `recipe-validation`, `podvm-resolve`,
`handbook-render`, `cupel`, `conformance`, `regime-poison`, `chaining-fact-band`.
Plus framework-self-test bodies: `calibrant-*` (4), `kick-tires`, `band-survival`,
`bure-tweak`, `burx-exchange`, `buh-link`, `buym-yelp`, BUK `fact-chaining`.

`regime-poison` is stratum-1 but BARRED from `fast` purely by tweak-slot
contention (the credless guard owns fast's one slot) — it rides
service/crucible/complete instead. This is the one place a fixture's suite
membership misrepresents its stratum, documented not defective.

### Stratum 2 — Freehold-Dependent Inner Bodies

`hallmark-lifecycle`, `lode-lifecycle`, `reliquary-lifecycle`, `wsl-lifecycle`,
`podvm-lifecycle`, `batch-vouch`, `chaining-fact-livery`, `moriah` (via the
crucible GAR-summon charge branch), `dogfight`.

### Stratum 3 — Wrapper Machinery

`depot-lifecycle`, `freehold-establish`, `freehold-churn`, `foedus-lifecycle`,
`access-probe`, `terrier-scaffold`, `terrier-atomicity`.

### Straddlers (resolved by classification, not papered over)

- **tadmor / srjcl / pluml** — local-kludge charge → stratum-1; **moriah** —
  GAR-summon charge needing a live Retriever mantle → stratum-2. Same security
  cases (`RBTDRC_CASES_SECURITY`), different charge substrate. The `.name` field
  is load-bearing: it routes the charge to a different nameplate.
- **onboarding-sequence** — spans wrapper-machinery (it stands up
  reliquary/yoke state other cases depend on) and inner-body (it tests
  ordain/conjure/graft against the substrate). Homes as wrapper-machinery for
  stratum purposes; inner-body cases ride inside it.
- **marshal-zero attest** (case 1 of depot-lifecycle) — stratum-1-flavored
  (pure filesystem + git-porcelain) but lives inside the LIFECYCLE wrapper as its
  precondition gate, not the always-runs base.

### Suites (all home as `rbtt_product` instances)

`fast` (stratum-1 base alone), `service`/`crucible`/`complete` (inner-bodies ×
REUSE at three dependency facets), `gauntlet` (inner-bodies × LIFECYCLE, carries
the release shape), `skirmish`/`dogfight`/`blockade` (inner-bodies × REUSE
against a standing freehold), `siege` (local crucible). Verified containment
lattice (the regression oracle): `blockade ⊆ skirmish ⊆ gauntlet`;
`service ⊆ complete`, `crucible ⊆ complete`, `complete == service ∪ crucible`
exactly.

---

## Duplication map

Both confirmed consolidations AND refuted ones matter: a refutation names the
load-bearing coverage a naive merge would lose.

### CONFIRMED consolidations

**S1-a. BUK fact-chaining seed prelude** (`Tools/buk/buts/butcfc_facts.sh`). The
`mkdir -p "${BURD_PREVIOUS_DIR}" || buto_fatal …` + `zbutcfc_seed …` idiom recurs
at 5 sites. Fold via a dedicated `zbutcfc_seed_previous(name, value)` wrapper that
hardcodes `BURD_PREVIOUS_DIR` and inlines the mkdir. **Guardrails:** leave the
OUTPUT_DIR seed on the bare helper; leave the 2 unseeded cases untouched; keep the
mkdir inside the wrapper (the header's readonly-path constraint forbids
redirecting `BURD_PREVIOUS_DIR`). Pure framework-self-test boilerplate; lowest
priority — outside the inner×wrapper product entirely.

**S1-b. regime-poison flat case table** (`rbtdrs_poison.rs`). The 31 cases are
one-liners over `rbtdrs_poison(...)`; a `&[{colophon, folio, poison, band, label}]`
slice + generated cases could replace ~150 lines coverage-preservingly.
**Guardrail:** case names must survive (FixtureCase selection keys on them).
Optional, coverage-neutral, confined to one file.

**S2-a. Four Lode round-trip case bodies** (`rbtdrc_crucible.rs`:
lode/reliquary/wsl/podvm-lifecycle). All four share a byte-near-identical
`capture-verb → read touchmark → divine-contains → augur member-tags →
[optional jettison] → banish → final-divine-NOT-contains` skeleton. Extract three
within-file invariant-block helpers, all returning `Option<rbtdre_Verdict>`
(Some = short-circuit Fail, None = continue):

- `zrbtdrc_read_touchmark(result, dir) → Result<String, Verdict>` — confirmed
  exact at 6 sites (all six write `02-touchmark.txt`; the digest's "ordinal
  differences" claim was verified false). **Excludes:** the two
  `RBTDRC_FACT_HALLMARK` reads (different fact, `02-hallmark.txt`); the
  chaining-livery site's read is followed by a load-bearing `'b'`+12-digit
  shape-validation block that **must remain inline**.
- `zrbtdrc_divine_contains(ctx, dir, touchmark, verb_label) → Option<Verdict>` —
  **takes a verb-label arg** to preserve the per-kind Fail-message diagnostic.
- `zrbtdrc_banish_and_verify_gone(ctx, dir, …) → Option<Verdict>` — the
  banish-with-confirm-skip + final-divine-NOT-contains bookend.

**HARD coverage guardrails** — six load-bearing differences stay verbatim at call
sites and must **never** be absorbed: (a) the capture verb+args
(`ENSCONCE_BOLE`+vessel_dir / `CONCLAVE_RELIQUARY` / `UNDERPIN_WSL`+release+point /
`IMMURE_PODVM`+family+version); (b) the augur member-tag sets; (c) the trust grade
(verified for lode/reliquary/wsl vs recorded-at-acquisition for podvm); (d)
lode-lifecycle's literal-HEAD-commit envelope assertion; (e) podvm's
refresh+cohort-count sub-sequence and trust-posture prose; (f) the member-grain
jettison step (reliquary+podvm only). Do not touch lode-collision or
chaining-livery in the fold. The full `(capture-verb, tags, grade, has-jettison)`
combinator stays operator-territory — folding the optional HEAD-commit/refresh
steps needs optional-step machinery whose risk is exactly the coverage-loss class
this audit exists to prevent.

**S2-b. Member-grain jettison block** (reliquary + podvm). The highest-confidence
Lode-family dedup: a pure mechanical clone (~54 lines, identical match-arm error
strings) differing only in the tag-constant pair. Extract
`zrbtdrc_member_jettison_proof(ctx, dir, touchmark, victim_tag, survivor_tag) →
Option<Verdict>`, emitting the `04b/04c/04d` scratch files. **Drop the
`all_member_tags` param** the proposal floated — `{victim, survivor}` suffices.
**Guardrail:** preserve the pre-list presence assertion for *both* tags inside the
helper.

**S2-c. Three made-side busybox ordain→…→abjure bodies** (hallmark-lifecycle +
batch-vouch + dogfight). All three share the `ordain RBTDGC_ORDAIN_HALLMARK
&[vessel_dir]` opener + `read RBTDRC_FACT_HALLMARK` and (hallmark-lifecycle +
dogfight) the abjure closer over `RBTDRC_BUSYBOX_VESSEL_DIR`. **Extract the
bookend spine ONLY** — keep all three middles at the call site (audit/rekon vs
jettison/tally/batch-vouch vs summon/bare-run are three distinct coverages).
**Do NOT merge the fixtures.** Route through the existing
`rbtdro_ordain_capture`/`_full` pair, not a third competing helper.

**S2-d. dogfight ↔ onboarding helper-locality.** The single clearest
zero-coverage-risk dedup in the crate. dogfight hand-inlines the exact `ordain +
three-fact read` that `rbtdro_ordain_capture_full` packages, plus the image_ref
builder, plus the `Ok exit==0 / Ok exit!=0 → Fail / Err → Fail` shape that
`rbtdro_invoke_or_fail` centralizes — written 3× by hand in dogfight. The sole
blocker is module-locality: `rbtdro_invoke_or_fail`, `rbtdro_ordain_capture`, and
`rbtdro_ordain_capture_full` are all module-private `fn` in
`rbtdro_onboarding.rs`, and **`rbtdri_invocation.rs` is the correct shared home**
— it already owns `rbtdri_read_burv_fact` and the `rbtdri_invoke_*` wrappers both
helpers call, and is already imported by both modules.

**Action:** hoist the three helpers (and a separate image_ref builder) into
`rbtdri_invocation.rs`; route **all four** open-coded sites through them — dogfight,
the conformance lifecycle and batch_vouch lifecycle in `rbtdrc_crucible.rs` (both
read only the single `RBTDRC_FACT_HALLMARK`, so they map to `ordain_capture`, not
`_full`), and onboarding-internal. Routing only `_full` leaves the cluster
half-deduped. **Coverage-neutral by construction:** the inline copies' bespoke
Fail prefixes become the helper's structured `"{operation} {target}"` prefixes —
strictly *more* diagnostic; the differing scratch-file names are unasserted debug
artifacts.

**S2-d CORRECTION — two GAR-locator families, never one union.** The proposal's
"one `zrbtdrc_ark_ref` builder folds ~9 sites" is **rejected.** **Family A**
(category-rooted): `{RBTDRC_GAR_CATEGORY_HALLMARKS}/{hallmark}/{basename}:{hallmark}`
— first segment is the const `rbi_hm`, `hallmark` appears twice; the wrest_locator
at onboarding's three wrest sites + batch-vouch. **Family B** (fact-rooted):
`{gar_root}/{ark_stem}/{basename}:{hallmark}` — first two segments are runtime
ordain-captured facts, `hallmark` appears once; dogfight's image_ref + onboarding's
two conjure/graft tails. The graft function uses **both** on the same hallmark in
the same scope — `rbi_hm/<hallmark>/…` is the canonical hallmark home,
`<gar_root>/<ark_stem>/…` is the per-vessel build namespace. Keep them as two
separate builders, or extract neither — never a union signature.

**S2-e. Seven verbatim reliquary-touchmark probe preambles**
(`rbtdro_onboarding.rs`). 7 byte-identical `rbtdrb_Probe{name:"reliquary touchmark
captured", check:rbtdro_probe_reliquary_touchmark, remediation:…}` + `rbtdrb_assert`
blocks — identical name, check fn, *and* remediation string. Collapse to a single
`assert_reliquary_witnessed() → Option<Verdict>` (zero-arg). **Guardrails:** at the
graft-demo case, replace only the *reliquary* preamble — leave the second
"graft-demo anointed" probe intact; do **NOT** touch
`rbtdro_kludge_tadmor_standalone` — its omission is deliberate and documented. A
parallel 6-site fold exists for `rbtdrk_depot.rs`'s
`"freehold depot moniker installed"` preamble — there the remediation string
varies, so pass it as the one parameter; keep `rbtdrp_probe_depot_levied`'s
stricter stem-prefix predicate distinct.

**S2-f. Discovery-walk and resolve-preamble** (`rbtdrf_fast.rs`). (b)
resolve-vessel-dir preamble (3 sites) folds safely into a shared helper (the
helper owns `.trim()`; keep dh_all_vessels_pass's explicit empty-string guard at
its callsite). (a) the discovery walk is NOT safe as a single 5-site signature —
narrow it to the four `.exists()/found:bool` sites and keep dh_all_vessels_pass
bespoke (its `.is_file()` + count + resolution-diagnostic are deliberate).

**S3-a. Stand-up create-spine** (`rbtdrp_lifecycle.rs` ↔ `rbtdrk_depot.rs`). Both
run the same spine; the load-bearing difference is the REUSE-vs-LIFECYCLE axis as
a branch. **CORRECTION:** the spine to extract is **NOT "levy + cross-check."**
`ensure_impl`'s reuse path performs *neither a levy nor a re-list* — on reuse it
sets `fact_list = list_pre`. The only byte-identical shared block is the
**project-id cross-check TAIL alone**: `read-fact-from-a-supplied-list →
compose_project_id → assert-equal`. Correct extract:
`zrbtdrk_crosscheck_project_id(root, fact_list, prefix_dir, moniker) →
Option<Verdict>` (parameterized by the `InvokeResult` that supplies the fact dir),
plus the shared fact-path builder (S3-c). The reuse-vs-create branch *stays at the
call site* — it *is* the wrapper's instance selection.

**S3-b. Teardown unmake-spine** (`rbtdrp_lifecycle.rs` ↔ `rbtdrk_depot.rs`).
**The audit's single most important coverage correction.** The proposal modeled
the difference as a placeholder literal + an `accept_states: &[&str]` set.
**REFUTED** — the post-unmake acceptance logic is **structurally inverted, not a
different set:**

| state after unmake | tear-down | churn |
|---|---|---|
| fact absent | **Pass** | **Pass** |
| `DELETE_REQUESTED` | **Pass** | **Pass** (falls through) |
| `COMPLETE` | **Fail** | **Fail** |
| read-error on existing fact | **Fail** | **Pass** (silent `if let Ok`) |
| any other / `DELETING` / empty | **Fail** | **Pass** |

Tear-down is a strict **allowlist, fail-closed**. Churn is a **denylist,
fail-open**. A finite `accept_states` allowlist **cannot express** churn's
"anything but COMPLETE" (the complement of `{COMPLETE}`, not enumerable).
Tear-down's strict allowlist *is* the gauntlet's create→destroy proof.
**Correct extract:** the shared *preamble* spine into one `rbtdrk` helper; the
post-unmake assertion injected as an **explicit polarity parameter** — a closure
or `enum {StrictAllowlist, PermissiveDenylist}`, *never* a boolean "strict/lax"
flag, and both call sites keep their current predicate verbatim. Given only two
call sites with opposite polarities, **leaving the two assertions un-merged and
lifting only the preamble is the lower-risk repair.** The placeholder literals
(`'torndown'`/`'churned'`) are low-stakes forensic labels — defer the literal-count
merge; the real debt is the ~6-line duplicated guard-doctrine comment.

**S3-c. Depot-fact-path construction** (`rbtdrk_freehold.rs` + 5 cluster sites).
`list_result.burv_output.join(RBTDRI_BURV_OUTPUT_SUBDIR).join(&prefix_dir).join(format!("{}.{}", moniker, ext))`
is verbatim at 5 file-path sites differing only in moniker + ext. **Two-builder
shape required:** the 6th site, `pick_next_moniker`, builds only the *parent
directory* (3-join) then `read_dir()`s it — author a parent-builder + a
file-builder calling it. Home is `rbtdrk_freehold`. **Note:** the existing
`rbtdri_read_burv_fact` joins with *no* prefix_dir segment — depot facts live one
dir deeper, so this prefixed builder is a genuine analogue, not a re-duplication.

**S3-d. Payor credential gate** (terrier-scaffold + terrier-atomicity Skip;
`zrbtdrc_foedus_roundtrip` Fail). The Skip-vs-Fail policy divergence is
load-bearing (terrier fixtures are suite passengers → Skip; foedus is
operator-invoked-only → an absent credential is a *run failure*).
**CORRECTION:** the "name-only `payor_gate(fixture_name, Skip|Fail)`" signature is
**rejected** — the foedus Fail interpolates `r.exit_code` **AND `r.stdout` AND
`r.stderr`** with distinct prose; the terrier Skips interpolate only `r.exit_code`.
A name-only signature would silently regress the foedus diagnostic at the one site
where the message does the most work. **Two safe shapes:** (i) the two terrier Skip
sites alone fold today with no caveat; (ii) foedus joins only via a richer
signature carrying a per-policy message template *plus* stdout/stderr — or a helper
that returns the probe `Result` and lets each call-site format its own verdict.
The helper must take `&mut ctx` directly (foedus is outside `rbtdrc_with_ctx`).
Defer the terrier-atomicity-re-invokes-scaffold consolidation to the wrapper-lift.

**S3-e. Test-module (`rbtdt*`) duplications.** The tempdir idiom
`rbtdth_scratch_root().join(format!("rbtd-test-{}-{}", std::process::id(), label))`
is open-coded at `rbtdte_engine.rs`, `rbtdti_invocation.rs`, and wrapped in
`rbtdtl_make_tempdir` — a clean `rbtdth_scratch_dir(label)` candidate. The
verdict-assert idiom is factored into `rbtdtl_assert_pass/_fail_with/_skip` in
`rbtdtl_calibrant.rs` ONLY; `rbtdti_invocation.rs` open-codes the same shape
repeatedly. 48 call-sites across the test modules — a duplication surface
comparable to the production-side `rbtdrf` cluster.

**X-b. `rbtdro_drive_hallmark` bypasses the engine config-console.** It hand-rolls
the find-replace-rename that `rbtdre_config_set_field` already centralizes — while
its sibling `rbtdro_write_vessel_env` *already* routes through that seam for
rbrv.env. Route `drive_hallmark` through `rbtdre_config_set_field`. Keep the
production-mirror doc-note (the `zrbob_drive_hallmark` Palisade obligation is out
of scope).

**X-c. Discovery-vs-suite roster drift — `foundry-path`.**
`RBTDRF_FIXTURE_FOUNDRY_PATH` is a member of 6 suites but **absent from
`RBTDRC_FIXTURES`** — so `tt/rbw-tf.FixtureRun.sh foundry-path` cannot resolve it.
One-line additive fix: add it to the roster among the other `rbtdrf_fast` imports.
**Durable guard:** a build-time const-check `suite-fixtures ⊆ RBTDRC_FIXTURES`.
The asymmetry is load-bearing: check `suite ⊆ roster` ONLY — `roster ⊆ suite`
would wrongly reject the intentional operator-invoked-only members.

**X-d. Fast-base set drift — `chaining-fact-band`.** The digest's framing was
backwards. Two digest claims are FALSE: service does **not** omit
chaining-fact-band (service ⊇ fast); and chaining-fact-band is **not** listed
twice (the extra hits are the `RBTDRC_FIXTURES` registry). The **real** defect:
fast/service/crucible/complete carry all 11 fast members, but **gauntlet and
skirmish carry only 10 — both omit chaining-fact-band.** Consequence: the
feoff/yoke band-matrix conformance fixture silently does not run in the release
ladder (gauntlet) or mini-gauntlet (skirmish) — a conformance hole in exactly the
suites meant to gate a release. **Proposal:** restore the member to both ladders,
or record-as-intent. Optionally add a `FAST_BASE` const used *purely* for a `⊆`
assertion (never spliced into the suites).

### REFUTED consolidations (coverage a naive merge would lose)

- **chaining-fact family is NOT a fold** (`helper-invocation-9`). `rbtdrh_chain.rs`
  (band, synthetic-seed, credless, stratum-1), `chaining-fact-livery` (live
  producer→consumer, credentialed, service-tier), and `butcfc_facts.sh` (BUK
  primitive unit) test the *same feoff/yoke discipline at three different strata*.
  The band-vs-livery split is load-bearing in three dimensions: strata; dispatch
  (raw `rbtdri_tabtarget_command` subprocess so the `RBTDGC_BAND_CHAIN` precision
  band survives, vs the in-process `ctx.chain_next_invoke()` chain); BURV-root
  staging. The livery body actively cross-validates the live touchmark against
  band's synthetic `'b'`+12-digit shape. Keep all three distinct.

- **tadmor/moriah static merge is NOT a fold** (`fixture-setup-1`,
  `suite-composition-3`). Byte-identical save `.name` — but `.name` is
  load-bearing: it routes the charge to a different nameplate with a different
  substrate dependency (tadmor = local kludge, stratum-1; moriah = conjure
  GAR-summon needing a live Retriever mantle, stratum-2). The
  "provenance not behavior" comment refers only to the in-bottle *security cases*.
  Merging the statics erases the network-posture/substrate axis. **Resolution
  (Part 3 below):** express as `(security-inner) × (charge-posture)`, not a clone.

- **payor-gate name-only signature is NOT a fold** (`case-body-3`). See S3-d above
  — the foedus Fail interpolates stdout/stderr the terrier Skips do not.

- **teardown placeholder-only parameterization is NOT a fold** (`fixture-setup-0`).
  See S3-b above — the assertion polarity is structurally inverted.

- **stand-up levy-bundling spine is NOT the extract** (`helper-invocation-3`). See
  S3-a above — the reuse path performs no levy and no re-list; only the cross-check
  tail is shared.

- **fast-list verbatim re-listing is NOT a fold** (`suite-composition-0`). The
  11-member fast set re-listed across service/crucible/complete is the DEFENDED
  compile-checked case (`rbtdrc_crucible.rs` doc-comment): const-slice concat is
  non-load-bearing cleverness, and REGIME_POISON is interleaved at position 3
  (absent from fast itself), so a naive `fast` prepend would not reproduce the
  membership.

- **the suite containment lattice must NOT be collapsed** (`suite-composition-4`).
  `skirmish ⊆ gauntlet` (delta = the LIFECYCLE-wrapper machinery skirmish drops);
  the near-subset is the wrapper/tier semantics, not redundancy. It is the
  regression oracle the combinator must reproduce.

- **the single all-swallowing ifrit/ark builder is NOT a fold**
  (`helper-invocation-0`, `helper-invocation-8`). Inverted-exit assertions (a
  non-zero exit is the PASS in hallmark-lifecycle Step 6), the depot
  `.burv_output` boundary, and the two distinct GAR-locator families all defeat a
  single-helper merge.

### Cross-stratum cleanup

**X-a. Dead ifrit-invoke wrapper** (`rbtdri_invocation.rs`). `rbtdri_invoke_ifrit`
(and its `RBTDRI_IFRIT_BINARY="rbid"` const) has **zero callers** crate-wide;
every live ifrit invocation goes through `rbtdrc_invoke_ifrit`/`_with_args`, which
adds per-case-dir capture the dead copy lacks. **Operator-gated:** delete the dead
fn + its orphaned const, leaving `RBTDRC_IFRIT_BINARY` as the single home. Low
priority; flag, do not remove during this read-only audit.

---

## Quota / reorientation debt

Recipe Bottle's test substrate is one durable freehold = (depot) + (foedus) +
(terrier). The federation reorientation began collapsing the old per-suite
credential ceremony onto that single freehold but **stopped partway**, under a
hard external constraint: **soft-deleted workforce pools hold the 100-per-org cap
for ~30 days** (`rbtdrm_manifest.rs`). That single quota fact is the thread
through every debt item — it is why the *create/destroy* halves shipped
asymmetrically, why three suites depend on hand-maintained state, and why interim
scaffolds are squatting.

**DEBT-1 — Credential-incomplete suites (skirmish / dogfight / blockade).**
`gauntlet` heals federation credentials inline by running `freehold-establish`
(compear → gird-governor → brevet+don-director → brevet+don-retriever).
skirmish/dogfight/blockade **drop** it to avoid its levy and replace the heal with
a **prose-only operator precondition**. `access-probe` checks **only** the payor
OAuth credential (`rbtdrm_credential_check_colophon` returns `Some` for payor
alone), never proving the director/retriever mantles donnable that every
`*-lifecycle` body, batch-vouch, and the terrier fixtures need. The heal itself is
**quota-neutral** (brevet/don are idempotent IAM re-grants; the quota lives in
affiance/jilt). **Remediation:** add freehold-establish's reuse-safe credential
heal (or a lighter compear+don-only readiness probe) as the credential-heal HEAD
of skirmish, and a readiness probe to dogfight/blockade — converting prose
precondition into a run fixture. This is the REUSE wrapper's credential preamble:
stratum-3 wrapper machinery, must NOT collapse into the inner bodies.

**DEBT-2 — The churny foedus posture shipped; the safe standing-reuse posture
never did.** `foedus-lifecycle` affiances a **throwaway** pool
(`foedus-<millis>`) → jilts → re-jilts, every run burning a unique pool against
the 30-day cap. The safe half — a standing-reuse foedus *ensure* (affiance-if-absent,
never jilt) — was never built. The freehold "composite" the model names is in code
**only the depot**: `rbtdrk_freehold` / `rbtdrk_depot` carry zero foedus
references. **Remediation:** add a foedus-ensure reuse leg mirroring the depot
reuse gate; keep `foedus-lifecycle` as the LIFECYCLE-wrapper churn proof. Both
lean on the already-factored `zrbtdrc_foedus_roundtrip`. **Operator-gated:** whether
the workforce-pool create gesture belongs *inside* the durable REUSE wrapper at all
is a judgment call — it spends the 30-day-cap pool.

**DEBT-3 — Squatting interim terrier scaffolds.** `terrier-scaffold` and
`terrier-atomicity` are self-labeled **interim**; they ride the `service` suite as
if inner bodies but their true stratum is **wrapper machinery** (the muniment
store, the third composite-freehold leg). `terrier-atomicity` **re-invokes the
whole scaffold in-body** (duplicating the provision work scaffold *is*). Both are
payor-credentialed, not mantle-scoped. **Remediation (two items, ordered):**
*immediate, coverage-neutral* — extract a shared `terrier_charge()` helper both
call, preserving each fixture's distinct failure-attribution string (do NOT
collapse the two fixtures); *deferred, operator-gated* — lift `terrier-scaffold`
into the wrapper's muniment-leg setup, migrate atomicity's manifest dependency,
and *then* remove the inline charge. **Sequencing is load-bearing:** remove the
inline scaffold first and standalone `rbw-tf terrier-atomicity` loses its terrier
and breaks. Gate on the terrier noun formalizing at RBS0 M4.

**DEBT-4 — Half-migrated credential model.** The old keyfile JWT-probe map was
demolished (the governor/director/retriever JWT probes "retired with the RBRA
estate"); the intended replacement (mantle readiness via compear + don inside
`freehold-establish`) exists but is reachable only through suites that *run*
freehold-establish (gauntlet). For the REUSE suites the readiness coverage was
demolished on the keyfile side and never re-established on the federation side —
the root cause of DEBT-1. **Discharged by DEBT-1's remediation.** Record-as-intent:
`rbtdrm_credential_check_colophon`'s single-role shape is **correct and
intentional** — do not "restore" the retired JWT probes.

**A latent sequencing dependency** (completeness GAP 5): DEBT-1's heal repairs
*mantle donnability* (brevet/don) but a donnable mantle presupposes a *standing
workforce pool*. If the pool has aged out of the 30-day window, freehold-establish's
compear leg fails at a layer DEBT-1's fix does not itself establish. **DEBT-1's fix
is latently dependent on DEBT-2's foedus-ensure landing first** — make this
explicit in the slate, or DEBT-1 will appear to "heal credentials" while still
failing on an absent pool.

---

## Suite naming slate (operator elects)

ONE coherent conflict/military asterism over the whole post-consolidation suite
set, with an audible escalation arc: readiness → standing posture → engagement →
full order of battle, plus a diplomatic sub-register for federation. All proposed
words gated to ZERO whole-word hits across `Tools/` and `tt/`. **These are
PROPOSALS; the operator elects.**

| Role | Current | Candidate | Runners-up | Grep | Rationale |
|---|---|---|---|---|---|
| Substrate-independent base | `fast` | **reveille** | drill, standfast | 0 files | The morning bugle putting the camp at readiness before any operation — the always-runs-first, no-substrate base. Pulls `fast` off the bare speed adjective into the asterism. |
| REUSE × GCP-credentialed inner bodies | `service` | **picket** | bivouac, ~~garrison~~ | 0 files (garrison TAKEN, 16 files) | A standing guard post held against a standing line — signals REUSE (held in place, not minted-and-torn). `service` is a trodden ambient-software word. |
| REUSE × local crucibles (**MANDATORY rename**) | `crucible` | **bivouac** | redoubt, rampart | 0 files | `crucible` the suite shadows `crucible` the production runtime noun — a terminal-exclusivity violation the Lapidary forbids. A bivouac is a field camp you fight from on your own ground with no supply line — LOCAL + self-contained. |
| REUSE × everything (service ∪ crucible) | `complete` | **echelon** | cordon, vanguard | 0 files (muster TAKEN: JJK verb) | The full layered formation, every tier in place — names the union semantics without the bare adjective. `muster` would be perfect but is a live JJK verb. |
| LIFECYCLE release ladder | `gauntlet` | **gauntlet** (keep) | — | incumbent | Running the gauntlet is the punishing end-to-end trial — exactly the LIFECYCLE release ladder. |
| REUSE mini-ladder | `skirmish` | **skirmish** (keep) | — | incumbent | The small/preliminary engagement — the mini-gauntlet against a standing position. |
| REUSE viability probe | `dogfight` | **dogfight** (keep) | — | incumbent | Close, fast, single-pass aerial engagement — the quick build-and-run probe. |
| Fully-local (tether) | `siege` | **siege** (keep) | — | incumbent | Sustained local investment of a position. Pairs with blockade on the network-posture axis. |
| Airgap moriah (airgap) | `blockade` | **blockade** (keep) | — | incumbent | A blockade cuts off supply lines — exactly the airgap posture. Forms the siege(tether)/blockade(airgap) pair. |
| Federation authentic-verb (elected) | (new) | **parley** | — | 0 files | The negotiated handshake with the other side — apt for the federation authentication/admission. Establishes the diplomatic sub-register. |

**Taken-words flagged:** `sortie` (ifrit attack-run colophon, 19 files);
`muster`/`parade`/`foray` (JJK verbs); `crucible` (production runtime noun — the
self-collision the slate resolves); `garrison` (BUK ceremony, 16 files — ideal for
the `service` role, forced to picket); `sentinel` (49 files); `probe` (72 files);
`campaign` (MCM quoin); `outpost` (RBK roadmap concept); `sally`/`investment`
(dictionary-only hits in APCK data files — not identifier collisions, but excluded
per the strict Lapidary gate).

**Two painful constraints shaped the picks:** `muster` (the literal word for the
union role) is a taken JJK verb, forcing echelon; `garrison` (ideal for the
standing-presence role) is a taken BUK ceremony, forcing picket. The single
mandatory rename is **crucible→bivouac** — every other dependency-tier rename is
upgrade-not-obligation.

**Rename blast radius** (completeness GAP 2, un-costed in the slate as shown):
suite names are not one-const strings — they appear as **tabtarget filenames**
(`tt/rbw-ts.TestSuite.{name}.sh` — a file rename), in **CLAUDE.md's two suite
tables**, and in **≥4 `.adoc` specs** (RBSCIP, RBSCB, RBSMF, RBSHR carry
`gauntlet`/`skirmish`/`blockade`). A suite rename is a tabtarget-rename + multi-doc
sweep + the `rbtdgc_consts.rs`/tabtarget-context.md regen. The grep scope as
documented (`Tools/ tt/`) is narrower than CLAUDE.md's mandated mint universe
("any persistent name anywhere") — the result happened to hold (clean in `Memos/`,
lenses, specs), but flag the *method*, not the result.

---

## Theurge-cosmology spec grounding

The audit established a concept vocabulary that currently lives only in this memo
and in code shape. Memos are provenance, not authority — these facts must outlive
the memo, so they need a spec home. Below: the quoin inventory and the full draft
subdoc.

### Quoin inventory

**Attribute stem: `rbtt_`** ("RB Term Theurge"). Grep-checked clean repo-wide.
Rejected alternatives all collide: `rbtc_` (RB Term Colophon), `rbst_` (RB
Subspecialized Types), `rbte_` (already the live theurge-engine CLI colophon —
minting an `rbte_` attribute stem would breach terminal-exclusivity).

| Quoin | Role | Meaning |
|---|---|---|
| `rbtt_theurge` | entity | The test-orchestration crate (`rbtd/`, entry `rbte`); the harness, not a fixture. |
| `rbtt_stratum` | entity | The dependency class a fixture belongs to — exactly one of three; distinct from suite membership. |
| `rbtt_substrate_independent` | term | Stratum 1: credless/fast base; not a term in the product. |
| `rbtt_inner_body` | term | Stratum 2: freehold-dependent fixture; left term of the product; runs under either wrapper. |
| `rbtt_wrapper_machinery` | term | Stratum 3: create/churn/teardown OF the substrate. |
| `rbtt_product` | entity | A runnable suite as (one inner-body × one wrapper); implicit today, the lift target. |
| `rbtt_wrapper` | entity | One of two ways to run the inner body: REUSE or LIFECYCLE. |
| `rbtt_reuse` | term | Run against a standing freehold, no create/destroy. |
| `rbtt_lifecycle` | term | Create → run → destroy. |
| `rbtt_freehold` | entity | The single durable substrate = (depot)+(foedus)+(terrier muniment). |
| `rbtt_leasehold` | term | The ephemeral instance the LIFECYCLE wrapper mints+tears-down; minted above the freehold. |
| `rbtt_release_shape` | term | Throwaway-leasehold-then-durable-freehold ordering (the gauntlet ladder). |
| `rbtt_freehold_subject` | term | The one PERMANENT federated identity (Entra `oid`); only this layer is seated here. |
| `rbtt_crucible` | term | The container-runtime vessel set; cites RBSCC/RBSIP; stratum varies by charge provenance. |
| `rbtt_suite_table` | term | `RBTDRC_SUITES` — the sole membership authority; pointed at, never transcribed. |

`busr_fixture` / `busr_suite` are **referenced from BUS0, not re-minted**. `cupel`
has no concept quoin (it is a fixture name only) — fixtures need no per-name quoin.

### Draft subdoc — `RBSTC-TheurgeCosmology.adoc`

Target home when adopted: `Tools/rbk/vov_veiled/RBSTC-TheurgeCosmology.adoc`,
`include::`'d into `RBS0-SpecTop.adoc` under the test-infrastructure movement, with
the `rbtt_` mapping block added to the RBS0 mapping section.

```asciidoc
// RBSTC — Theurge cosmology (test-orchestration concept subdoc)
//
// SKIMPY draft, contract-first. The concept home for the theurge
// test-orchestration vocabulary: the strata, the inner-body × wrapper product,
// the freehold/leasehold substrate, and the release lifecycle shape. Base
// quoins only — this seats the cosmology so the rbtd crate, the suite table, and
// the eventual inner×wrapper combinator have one place to cite, and CLAUDE.md's
// suites table thins to a pointer.
//
// FIVE MEMBRANES honored here, each a do-not-recreate boundary:
//   1. busr_fixture / busr_suite are BUS0's (the test-registry nouns). This
//      subdoc REFERENCES them and never re-mints — the theurge fixture/suite ARE
//      BUS0 fixtures/suites; what is new here is their stratification and the
//      wrapper product, not the registry primitives.
//   2. Suite MEMBERSHIP lives in RBTDRC_SUITES (rbtdrc_crucible.rs), the sole
//      composition owner. This subdoc points at it and transcribes NO member
//      lists — a member list in prose drifts the day a suite is edited.
//   3. Crucible-security behavior lives in RBSCC (charge) and RBSIP (ifrit
//      pentester). The crucible quoin here cites them; it does not restate the
//      security model.
//   4. The identity-layers model is split: only the PERMANENT freehold-SUBJECT
//      pinning is seated here. The EVOLVING foedus/depot instance layers stay in
//      their config homes (rbrf.env / rbrd.env) and rbpc_constants.sh, cited not
//      copied.
//   5. The stratum/wrapper concepts are the s of THIS subdoc; the durable
//      substrate MACHINERY (rbtdrk_freehold / rbtdrk_depot / rbtdrp_lifecycle)
//      is already built — these quoins name it for the lift, they do not respec it.
//
// Wiring (forthcoming): the rbtt_ quoins below register in the RBS0 mapping
// section under a new rbtt_ category line, and this file is `include::`'d into
// RBS0 under the test-infrastructure movement, proximal to wherever the crucible
// civic quoin is seated. Settled in this draft: the cosmology identity, the three
// strata, the two wrappers, the freehold/leasehold pair, and the release
// lifecycle shape. NOT settled: the first-class combinator's surface (operator
// territory) and whether the foedus leg joins the durable REUSE wrapper.

== Theurge cosmology

The
{rbtt_theurge}
is Recipe Bottle's test-orchestration crate (`rbtd/`, entry `rbte`). It runs
{busr_fixture_s}
and
{busr_suite_s}
— BUS0's registry nouns — but layers two concepts over them that BUS0 does not
carry: every fixture sits in exactly one
{rbtt_stratum},
and a runnable suite is, in the target model, a
{rbtt_product}
of one
{rbtt_inner_body}
crossed with one
{rbtt_wrapper}.
Today that product is encoded implicitly — by which suite lists which fixtures in
{rbtt_suite_table}
plus branch logic inside the depot fixtures — and lifting it to a first-class
combinator is the cosmology's open work.

[[rbtt_theurge]]
//axo_entity
{rbtt_theurge}::
The crucible/test-orchestration crate under `rbtd/`. Owns fixture discovery,
suite composition, the engine config-console, and the crucible charge/quench
hooks. Not a
{busr_fixture}
itself — the harness that runs them.

[[rbtt_stratum]]
//axo_entity
{rbtt_stratum}::
The dependency class a
{busr_fixture}
belongs to — exactly one of three. The stratum is a property of the fixture's
substrate need, distinct from the
{busr_suite_s}
it is listed in (a fixture's suite membership may, today, misrepresent its
stratum — see
{rbtt_substrate_independent}
on the regime-poison slot).

[[rbtt_substrate_independent]]
{rbtt_substrate_independent}::
Stratum 1. Needs no depot, pool, container, or credential — credless and fast
(enrollment, regime-*, cupel, conformance, dockerfile-hygiene, foundry-path,
podvm-resolve, handbook-render, chaining-fact-band). The base that always runs;
NOT a term in the
{rbtt_product}.
A fixture here carries `disposition: Independent, credless: true`.

[[rbtt_inner_body]]
{rbtt_inner_body}::
Stratum 2, and the left term of the
{rbtt_product}.
A
{busr_fixture}
that exercises the one standing
{rbtt_freehold}
— GCP-service fixtures and the depot-using crucible — and runs identically under
EITHER
{rbtt_wrapper}.
The inner body tests the substrate; it never creates or destroys it.

[[rbtt_wrapper_machinery]]
{rbtt_wrapper_machinery}::
Stratum 3. A
{busr_fixture}
that establishes, churns, or tears down the composite
{rbtt_freehold}
— the create/destroy OF the substrate, not a test OF it. The two
{rbtt_wrapper_s}
are realized by this machinery: depot-lifecycle, freehold-establish/churn, and
the foedus lifecycle live here.

[[rbtt_product]]
//axo_entity
{rbtt_product}::
The target-model identity of a runnable
{busr_suite}:
one
{rbtt_inner_body}
crossed with one
{rbtt_wrapper}.
NOT yet a first-class abstraction — currently spelled by hand in
{rbtt_suite_table}.
The consolidation goal is to lift this cross-product into a combinator so the
inner body is written once and each suite names `wrapper(inner)` instead of
re-listing fixtures.

[[rbtt_wrapper]]
//axo_entity
{rbtt_wrapper}::
One of two ways a
{rbtt_inner_body}
is run against the substrate. The
{rbtt_reuse}
wrapper runs against a standing
{rbtt_freehold}
with no create/destroy; the
{rbtt_lifecycle}
wrapper creates the substrate, runs, then destroys it. The distinction maps onto
the engine's existing setup/teardown hook channel: REUSE = no hooks, LIFECYCLE =
setup(ensure-fresh) + teardown(churn).

[[rbtt_reuse]]
{rbtt_reuse}::
The wrapper that runs an
{rbtt_inner_body}
against an already-standing
{rbtt_freehold}
— no levy, no unmake. The day-to-day posture (service / crucible / complete and
the standing-depot ladders). Its credential preamble is freehold-establish's
reuse branch, which reuses an ACTIVE depot and re-heals (idempotently) the
mantles, spending no workforce-pool quota.

[[rbtt_lifecycle]]
{rbtt_lifecycle}::
The wrapper that CREATES the substrate, runs the
{rbtt_inner_body},
then DESTROYS it — proving teardown of a used install. The release posture
(gauntlet). Realized by
{rbtt_wrapper_machinery}.
See
{rbtt_release_shape}
for the throwaway-then-durable ordering.

[[rbtt_freehold]]
//axo_entity
{rbtt_freehold}::
The single durable test substrate — a composite of (depot / GCP project) +
(foedus / workforce pool) + (terrier muniment bindings). Reused day-to-day by the
{rbtt_reuse}
wrapper; destroyed and recreated only at release. In current code the composite
is, in practice, only the depot leg (rbtdrk_freehold / rbtdrk_depot); the foedus
and terrier legs are not yet folded into the durable substrate — a known gap the
lift must close, not a property of the concept.

[[rbtt_leasehold]]
{rbtt_leasehold}::
The EPHEMERAL substrate instance the
{rbtt_lifecycle}
wrapper mints and tears down — the throwaway depot the gauntlet creates to prove
create→destroy on a USED install, then destroys. Always minted ABOVE the standing
{rbtt_freehold}
(pick-next-moniker = max + 1), so a lifecycle teardown never reaches the freehold.
The leasehold/freehold split IS the safety boundary between the two
{rbtt_wrapper_s}.

[[rbtt_release_shape]]
{rbtt_release_shape}::
The release-qualification ordering carried by the gauntlet ladder: mint a
{rbtt_leasehold}
and tear it down (proving teardown of a dirtied install), THEN establish a durable
{rbtt_freehold}
and leave it standing for the
{rbtt_reuse}
suites. Throwaway-then-durable. The ordering lives in
{rbtt_suite_table};
this quoin names the shape, not the member list.

[[rbtt_freehold_subject]]
{rbtt_freehold_subject}::
The single PERMANENT federated identity the test rig exercises — the operator's
standing Entra `oid` (`RBPC_freehold_subject`, rbpc_constants.sh). The
citizen-definition layer of the identity-layers model: pool-independent, it
survives foedus re-mints and depot churn, and a re-mint re-brevets THIS same
subject into the fresh pool. The EVOLVING instance layers (foedus pool-id +
provider in rbrf.env; depot moniker + prefix in rbrd.env) are NOT seated here —
they are config homes this quoin points at. Only the subject pinning is the
cosmology's.

[[rbtt_crucible]]
{rbtt_crucible}::
The container-runtime test vessel set (sentry + pentacle + bottle) a crucible
{busr_fixture}
charges and quenches. Its security behavior — charge sequence, in-bottle
adversarial probing — is contracted by RBSCC (crucible charge) and RBSIP (ifrit
pentester); this quoin cites them and does not restate the model. A crucible
fixture's
{rbtt_stratum}
depends on its charge provenance: a LOCAL kludge charge is
{rbtt_substrate_independent},
a charge that auto-summons hallmarks from the depot GAR is an
{rbtt_inner_body}
(the tadmor/moriah straddle — same security cases, different charge substrate,
resolved on the network-posture sub-axis: tether/local vs airgap/GAR-summon).

[[rbtt_suite_table]]
{rbtt_suite_table}::
`RBTDRC_SUITES` (rbtdrc_crucible.rs) — the sole owner of
{busr_suite}
→
{busr_fixture}
composition for the
{rbtt_theurge}.
Where the
{rbtt_product}
is encoded today (which suite lists which fixtures). This subdoc points at it as
the membership authority and transcribes no member lists; the per-suite census
belongs there and in the CLAUDE.md pointer, never in this prose.

=== Stratum is not suite tier (settled)

A fixture's
{rbtt_stratum}
is its substrate NEED; its
{busr_suite}
membership is where it is LISTED. These usually agree but need not: regime-poison
is
{rbtt_substrate_independent}
yet rides service/crucible/complete (not `fast`), because `fast` reserves its one
tweak slot for the credless guard. The misfile is documented, not a defect — but
it is exactly why the two concepts are kept distinct here: a future multi-tweak
`fast` channel would re-home regime-poison by stratum without touching any suite
definition.

=== The product is implicit today (the lift)

The
{rbtt_product}
is not yet a combinator. The
{rbtt_lifecycle}
vs
{rbtt_reuse}
distinction lives as branch logic inside two near-clone depot fixtures
(stand-up always levies; ensure reuses-or-levies), and the same
{rbtt_inner_body}
under two wrappers is spelled by listing it in two suites. The durable substrate
machinery (rbtdrk_freehold / rbtdrk_depot / rbtdrp_lifecycle + the gauntlet
ladder) is ALREADY BUILT; the open work is LIFTING it — factoring the shared
levy-and-cross-check and unmake-and-assert spines into wrapper-parameterized
helpers, and elevating
{rbtt_product}
to a first-class `wrapper(inner)` form whose regression oracle is the current
{rbtt_suite_table}
membership lattice. This subdoc seats the vocabulary that lift will cite; it does
not perform the lift.

// eof
```

**Mapping-section block** to add to RBS0 (the new `rbtt_` category):

```asciidoc
// rbtt_ prefix: RB Term Theurge (test-orchestration cosmology)
:rbtt_theurge:                  <<rbtt_theurge,Theurge>>
:rbtt_stratum:                  <<rbtt_stratum,Stratum>>
:rbtt_substrate_independent:    <<rbtt_substrate_independent,Substrate-Independent>>
:rbtt_inner_body:               <<rbtt_inner_body,Inner Body>>
:rbtt_wrapper_machinery:        <<rbtt_wrapper_machinery,Wrapper Machinery>>
:rbtt_product:                  <<rbtt_product,Product>>
:rbtt_wrapper:                  <<rbtt_wrapper,Wrapper>>
:rbtt_wrapper_s:                <<rbtt_wrapper,Wrappers>>
:rbtt_reuse:                    <<rbtt_reuse,Reuse Wrapper>>
:rbtt_lifecycle:                <<rbtt_lifecycle,Lifecycle Wrapper>>
:rbtt_freehold:                 <<rbtt_freehold,Freehold>>
:rbtt_leasehold:                <<rbtt_leasehold,Leasehold>>
:rbtt_release_shape:            <<rbtt_release_shape,Release Shape>>
:rbtt_freehold_subject:         <<rbtt_freehold_subject,Freehold Subject>>
:rbtt_crucible:                 <<rbtt_crucible,Crucible>>
:rbtt_suite_table:              <<rbtt_suite_table,Suite Table>>
```

### Resolving the crucible network-posture straddle (Part 3)

The duplication is a *symptom* that fixture identity today conflates (case-body +
charge-flavor) in a cloned static. The fix is the same combinator as the wrapper
lift, on a second axis — a **charge-posture** field. The inner body is
`RBTDRC_CASES_SECURITY` (one set, unchanged). The posture carries the
network-posture sub-axis (tether vs airgap) *and* the provenance/substrate
(local-kludge vs GAR-summon-needing-Retriever); it selects the nameplate the
shared `rbtdrc_charge_crucible` imprints. `tadmor = (security-inner) × (local-tether
posture)`; `moriah = (security-inner) × (airgap-GAR posture)`. siege/blockade then
read as `posture(tether)` / `posture(airgap)`, with blockade additionally riding
the REUSE wrapper (Retriever mantle donnable). **Do NOT collapse the statics by
hand-merge** — the lift expresses the distinction as `(inner) × (posture)`.

---

## Recommended pace-slate

13-pace dependency-ordered slate for heat Bf, derived from the audit with the five
scope-changing claims re-verified against current source.

1. **rbtdrc-module-split-refresh** — *intricate but mechanical.* Carry the
   already-slated module split, refreshing its inventory to relocate
   chaining-fact-livery and preserve the new rbtdrh_chain cross-module registry
   references. The two registries (RBTDRC_FIXTURES, RBTDRC_SUITES) must remain in
   one module after the split.

2. **registry-drift-guard-and-heal** — *small, high-leverage.* Register
   foundry-path in RBTDRC_FIXTURES; add a build-time assertion that every
   suite-referenced fixture is roster-resolvable (assert `suite ⊆ roster` ONLY).
   Depends on 1.

3. **fast-base-set-equality-assertion** — *mechanical guard + one operator
   decision.* Add a FAST_BASE membership assertion as a regression oracle; surface
   the gauntlet/skirmish chaining-fact-band omission for restore-or-record.
   Depends on 2.

4. **ordain-capture-helpers-to-shared-home** — *clearest zero-coverage-risk dedup.*
   Hoist rbtdro_ordain_capture/_full, rbtdro_invoke_or_fail, and the two
   GAR-locator builders into rbtdri_invocation; route dogfight + the crucible
   lifecycle bodies through them. Depends on 1.

5. **lode-roundtrip-block-helpers** — *within-file extraction, six named
   exclusions.* Extract the touchmark read-back, divine-contains, and
   banish-and-verify-gone blocks plus the member-grain jettison block; the six
   load-bearing differences stay inline.

6. **onboarding-reliquary-probe-helper** — *trivial within-file fold, one
   exclusion.* Collapse the seven byte-identical reliquary-touchmark probe
   preambles; leave the standalone kludge-tadmor path probe-free.

7. **payor-gate-helper-richer-signature** — *real dedup gated on a richer
   signature.* Extract the triplicated payor-credential gate; the signature must
   reproduce the foedus stdout/stderr dump the naive form would drop.

8. **drive-hallmark-through-config-console** — *route through the shared seam.*
   Route rbtdro_drive_hallmark through rbtdre_config_set_field for rbrn.env, as
   its rbrv.env sibling already does.

9. **freehold-wrapper-combinator-lift** — *the audit core; operator judgment.*
   Lift the implicit (inner-body × wrapper) encoding into a first-class combinator
   over the engine setup/teardown channel; factor the stand-up/teardown spines
   parameterized by REUSE-vs-LIFECYCLE; the teardown polarity is an explicit
   predicate, never a boolean. Do NOT rebuild the substrate machinery — LIFT it.
   Depends on 7.

10. **freehold-credential-reuse-leg** — *design work against the quota crisis.*
    Build the standing-reuse credential-heal leg skirmish/dogfight/blockade
    assume; add a foedus-ensure reuse leg. Converts prose preconditions into
    fixtures. Depends on 9 (and latently precedes DEBT-1's heal — the foedus-ensure
    must land for the heal to be complete).

11. **butcfc-seed-previous-wrapper** — *trivial BUK-self-test fold.* Fold the
    mkdir+seed-previous idiom; leave the OUTPUT_DIR seed and unseeded cases
    untouched.

12. **ifrit-invoke-dead-code-removal** — *surgical dead-code removal,
    operator-gated.* Remove the dead rbtdri_invoke_ifrit + duplicate const,
    leaving the live rbtdrc pair as the single home.

13. **theurge-cosmology-subdoc-finalize** — *spec authoring.* Finalize the RBS0
    theurge-cosmology subdoc, homing the lifted concepts as spec rather than memo;
    supersedes the prior cosmology-spec pace. Depends on 9.

**Tiering:** paces 1-8, 11, 12 are mechanical/low-risk (standard tiers); 9, 10, 13
are design conversations requiring operator judgment (opus). Two coverage-gap
decisions need an explicit operator ruling: pace 3 (restore-vs-record
chaining-fact-band) and pace 10 (foedus placement in the durable wrapper).

---

## Open questions / completeness gaps

Ranked by severity (from the completeness critique, calibrated against findings
checked-and-sound: suite line numbers, moriah membership = exactly three suites,
onboarding-sequence placement, foundry-path drift, the tadmor/moriah straddle, and
all four refutation clusters held).

- **GAP 1 (SEVERE) — the `rbtdt*` unit-test half was never run through the
  duplication lens.** Eleven `rbtdt*` modules (~2,460 lines; `rbtdti_invocation.rs`
  641, `rbtdtu_cupel.rs` 454, `rbtdte_engine.rs` 420) are explicitly in-scope, yet
  only `rbtdtl_calibrant.rs`/`rbtdth_helpers.rs`/`rbtdtm` were touched. Confirmed
  concrete dups (the `rbtd-test-{pid}-{label}` tempdir idiom, the verdict-assert
  family factored only in calibrant, 48 unmapped call-sites) — a
  coverage-comparable code half with no lens (captured as pace S3-e, but the full
  sweep is undone).

- **GAP 2 (MODERATE) — suite-rename blast radius un-costed; five of nine slots
  absent.** See the naming slate's rename-blast-radius note. The slate excerpt is
  truncated mid-echelon and the siege/blockade/gauntlet/skirmish/dogfight slots
  are absent from the JSON shown — incomplete on its face (filled in the table
  above from the rationale).

- **GAP 3 (MODERATE) — cupel cluster stratum mis-statement.**
  `rbtdru_bash.rs`/`rbtdru_python.rs` are NOT fixture homes — `RBTDRU_FIXTURE_CUPEL`
  lives only in `rbtdru_cupel.rs`; the other two are pure scan-domain libraries.
  The "three-file cupel cluster" framing overstates where the fixture surface is.
  Precision defect, no coverage risk.

- **GAP 4 (MODERATE) — `rbtdrn_conformance.rs` lens declared but not executed.**
  `conformance` was inventoried by name only — no helper list, no dup-hint, no
  within-module pass. Either confirm dup-free explicitly or run the lens.

- **GAP 5 (MINOR but real) — DEBT-1's heal is latently dependent on DEBT-2's
  foedus-ensure.** A donnable mantle presupposes a standing pool; if the pool aged
  out, freehold-establish's compear fails at a layer DEBT-1 does not establish.
  Make the sequencing explicit (reflected in pace 10's dependency note) or DEBT-1
  ships a fix that looks complete and fails on an aged-out pool.

- **GAP 6 (MINOR) — the combinator's interaction with the `credless` arming seam
  is unspecified.** `rbtdrc_set_context` arms the credless guard from
  `fixture.credless`. When the combinator composes an inner body under a wrapper,
  which `credless` value wins? A small membrane the spec draft must name.

- **G2/G3 (spec-ahead anchors, deliberate) — the foedus and terrier legs of
  `rbtt_freehold` are concept-only, not yet code; `rbtt_release_shape` assumes the
  gauntlet is the only release shape.** Both are deliberate aspirational anchors
  the lift builds toward, flagged in the quoin definitions, not mis-carves.

---

## Addendum — gap-closure sweep (2026-06-23, follow-up)

After the main audit, the two unswept-module gaps (GAP 1, GAP 4) were closed with
focused follow-up scouts. Both are now discharged; coverage of the in-scope corpus
is complete (no module left unlensed). The slate grows by one pace (14).

### GAP 1 (`rbtdt*` unit-test half) — DISCHARGED, mostly clean

The eleven `rbtdt*` modules (~2,460 lines) were swept across three lenses
(framework-infra mirrors, fixture-behavior mirrors, cross-cutting). Headline: the
test-of-the-test half shares *grammar* (test vocabulary) but not *logic* — most
apparent duplication is deliberately-distinct coverage and was refused. Exactly one
genuine net-new consolidation surfaced (Pace 14); everything else was checked and
kept distinct.

Preserved (refused merges — coverage findings):
- The `rbtdte`/`rbtdti` verdict-count, ifrit-parse, and `rbtdtx` conversion tests —
  each pins a distinct oracle. Keep.
- `rbtdtl_calibrant` disposition tests — three assert `Independent`, the rest
  `StateProgressing`; a naive "all StateProgressing" fold would silently destroy the
  `Independent` coverage (the single most dangerous merge in the half). Pace 14's
  helper parameterizes disposition to preserve it.
- `rbtdtk_freehold` disjointness/compose tests — the regression oracle the
  wrapper-lift (Pace 9) must keep green; keep the cheap pure-prefix check
  (`disjoint_per_tincture`) SEPARATE from the full derived-identifier check
  (`dual_station_disjoint`, which alone carries the ≤30-char GCP-naming bound +
  SA-email composition) so a Pace-9 failure localizes. → watch-note on Pace 9.
- `rbtdtu_cupel` table-driven classify tests — six distinct supply-chain conformance
  rules wearing a shared loop. Keep.
- `rbtde_engine.rs:403` git-fixture — must stay an independent oracle (it proves the
  production commit verb is scoped by `git status`-ing an unrelated edit; routing it
  through the verb-under-test would be circular). → watch-note on Pace 8 (mechanical
  update if commit signatures change).

### GAP 4 (`rbtdrn_conformance.rs`) — DISCHARGED, dup-free

Lens run. The module is the unique vocabulary-eviction static-analysis scanner the
audit suspected — its eviction table ships empty by design (rows added per-cluster),
so there are no per-rule case bodies to dedup; its four self-tests assert distinct
discriminations (identifier-boundary, PathPrefix exemption, basename/line-0 sprue,
no-false-positive) and must stay distinct. The only cross-module touch is a 1-line
`{path}:{line}: {token} — {text}` report-format convention shared with
`rbtdru_cupel.rs:288` — below any pace threshold. Note: its eviction vocabulary
(renamed-concept stems) must NOT be conflated with cupel's (evicted shell commands)
— a coverage trap.

### GAP 3 (cupel stratum precision) — corrected

The "three-file cupel cluster" framing in the Inventory overstates the fixture
surface: `RBTDRU_FIXTURE_CUPEL` lives only in `rbtdru_cupel.rs`; `rbtdru_bash.rs` /
`rbtdru_python.rs` are pure scan-domain libraries (the bash/python lexers), not
fixture homes. Precision defect only, no coverage risk.

### Pace 14 (net-new, from GAP 1) — mechanical, order-independent in the mechanical band

**`rbtdth-shared-test-helpers`** — enrich `rbtdth_helpers` (today it exports only
`rbtdth_scratch_root`) with two hoisted helpers, collapsing ~150 lines of test
boilerplate:
- `rbtdth_assert_disposition(fixture, want)` + `rbtdth_assert_cases(fixture, count,
  must_contain)` — fold the registration-triplet idiom (lookup + `.expect` +
  disposition + arity + name-`contains`) repeated across
  `rbtdtk`/`rbtdtp`/`rbtdto`/`rbtdtl`. **Cinch:** disposition is a parameter, never a
  constant — calibrant's three `Independent` fixtures must survive.
- `rbtdth_make_scratch(label)` — fold the scratch-dir maker triplicated across
  `rbtdte`/`rbtdti`/`rbtdtl`, adopting the strongest recipe (pid + nanos + pre-clean)
  as the superset. **Cinch:** leave `rbtdtk_freehold`'s deliberately-nonexistent path
  (missing-root test) and `rbtdti`'s `make_tt_dir`/`write_script` alone; before
  unifying the dir-name prefix, check the `rbtde` isolation test that reads the dir
  name back.

### Watch-notes folded onto existing paces

- **Pace 8** (`drive-hallmark-through-config-console`): if it changes the
  `rbtdre_commit_*` signatures, the `rbtde_engine.rs:403` git-fixture test needs a
  mechanical update — downstream watch, not a dep.
- **Pace 9** (`freehold-wrapper-combinator-lift`): the `rbtdtk_freehold`
  disjointness/compose tests are the lift's regression oracle — keep them un-merged
  and localizing. Also GAP 6: name which `credless` value wins when the combinator
  composes an inner body under a wrapper.

### Still-open (operator-facing, not coverage gaps)

- **GAP 2 (rename blast radius un-costed):** the suite-naming slate proposes
  reveille/picket/bivouac/echelon but does not cost the rename churn (tabtargets,
  docs, the CLAUDE.md tables, the `RBTDRC_SUITES`/CLI surfaces). Bears on the naming
  election — cost it before electing the non-mandatory three.
