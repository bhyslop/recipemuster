# Matricula consumer drain — rulings and mapping sets (260814)

**Date:** 2026-08-14
**Heat:** ₣CM (rb-matricula-normalization)
**Status:** Authority. The apply-paces execute against the mapping sets below; this file retires when they land.
Its companion `memo-20260814-matricula-consumer-census.md` is the frozen census these rulings were made against.

Every one of the 118 native barring findings carries a disposition here — a ruling with a mapping set, or a deferral with its reason and its home.

| disposition | findings | kind |
|---|---|---|
| M1 segment-drop | 7 | ruled — 33 renames |
| M2 private-marker | 14 | ruled — 14 renames |
| M4 crate-head | 8 | ruled — 8 renames |
| M5 file-head | 2 | ruled — 2 renames |
| M6 rbthd de-duplication | 7 | ruled — de-duplication, no rename |

M1 and M2 overlap on the eight `rbtdro` inners, which both reach; composed into single-step form they are renamed once, so the mapping sets below carry **49 distinct renames**, not 57.
No mapping set contains a chain or a swap — every target was checked against every source — because the rename act refuses one by construction.
| D1 BUK jurisdiction | 15 | deferred — routed to the BUK home |
| D2 ifrit local-helper convention | 8 | deferred |
| D3 rbtdrk family homing | 5 | deferred |
| D4 variant-suffix and elaboration class | 52 | deferred — routed to ₣Bj |
| **total** | **118** | |

## The serialized-surface sweep

Run ahead of every ruling below, per the docket's cinch.
The question it answers is whether any repair would move a token that reaches a durable store.

**Verdict: none would.** No finding is deferred on serialized-surface grounds.

The sweep took the 218 names any repair could move — every barring signet plus every child named in a breach — and asked two questions.

**Does the name stand in a durable-store file?** No.
Zero hits across every tracked `.env`, `.yml`, `.yaml`, `.json`, `.txt`, and Dockerfile in the tree.

**Does the name reach a store as a string literal in code?** No.
109 literal sites across 40 names were read individually, and every one falls into a class that reaches no store: a diagnostic message naming its own function (`buc_die "rbfv_vouch_gate: hallmark required"`), an in-tree file path (`source ".../rbfb_beckon.sh"`), a temp-file fragment under `BURD_TEMP_DIR`, or a test-case name in the registry.

The families that genuinely touch a durable store do so through constant **values**, and in every case the value is a kebab-case string distinct from the identifier a rename would move:

| constant | value written to the store |
|---|---|
| `RBCC_fact_ext_depot` / `_depot_project` | `depot` / `depot-project` (fact-file extensions) |
| `RBCC_fact_ext_foedus` / `_foedus_health` | `foedus` / `foedus-health` |
| `RBTDGC_FACT_EXT_FOEDUS` / `_HEALTH` | `foedus` / `foedus-health` |
| `RBTDRK_FACT_EXT_DEPOT` / `_PROJECT` | `depot` / `depot-project` |
| `RBIDA_SEL_NET_SRCIP_SPOOF` / `_EXTERNAL` | `net-srcip-spoof` / `net-srcip-spoof-external` |
| `RBIDA_SEL_DNS_ALLOWED_EXAMPLE` / `_ORG` | `dns-allowed-example` / `dns-allowed-example-org` |
| `BUWGC_TT_*` and siblings | `buw-tt-ll`, `buw-rcv`, … (colophons, which name real tabtarget files) |

Worth recording because it is the sweep's most interesting result: the **values** carry the same parent/child prefix shape as the identifiers — `depot` is a strict prefix of `depot-project`, and consumers walk fact files keyed on extension.
The serialized layer therefore has the same latent collision the census reports at the identifier layer.
It is out of this drain's reach and must stay so; renaming a constant leaves its value untouched, which is exactly why these repairs are available.

Two couplings bind every apply-pace, and they are obligations rather than deferrals:

- **Diagnostic strings must move with the name.** A rename that leaves `buc_die "rbfv_vouch_gate: …"` naming a function that no longer exists is a silent wrongness no compiler catches. The class proof's zero-old-names grep must therefore read string literals, not just declarations.
- **Test-case name strings must move in the same commit.** `rbtdtl_calibrant.rs`, `rbtdto_onboarding.rs`, and `rbtdtb_probe.rs` reference case functions by string, and those strings are also the operator-facing case identifiers for `tt/rbw-tc.FixtureCase.sh`.

## The gate

Every target below was run against both directions of the terminal-exclusivity law, over 2800 declared signets harvested from the tree.

One failure was found and repaired at the ruling.
Segment-dropping `rbtdro_onboarding_kludge_tadmor` to `rbtdro_kludge_tadmor` would have seated it over a living child, `rbtdro_kludge_tadmor_standalone` — a breach the rename would have minted.
The file already carries both conventions, so the pair is ruled `rbtdro_kludge_tadmor_onboarding` / `rbtdro_kludge_tadmor_standalone`, which spends no new word and leaves neither a prefix of the other.

After that repair the gate is clean: no target is occupied, none seats over living children, and none extends a living seated name.

## Rulings

### M1 — segment-drop (7 findings, 33 renames)

A filename seats a signet, and declarations inside repeat the file's own subject.
The repair drops the redundant segment, leaving the file seat childless and terminal.

Ruled for seven families only, and the fork is deliberate: segment-drop is clean where the file-stem tail is **redundant** with a sole file, and unsound where it **distinguishes** sibling files sharing one head.
`rbfb`, `rbtdrh`, `rbtdro`, `rbtdrs`, and `rbtdtk` each have exactly one file, so the tail carries nothing.
`rbgp` and `rbq` have two, but the second is the thin `_cli.sh` partner rather than a peer subject, so the tail is redundant there too.
`rbtdrk` has two genuine peers (`depot` and `freehold`) and is deferred at D3 for that reason.

### M2 — private-marker (14 findings, 14 renames)

A public wrapper `X` calls a private inner `X_impl`, which extends it.
The repair prepends the estate's private marker rather than electing a word: `X_impl` becomes `zX_impl`.

This is conformance, not invention. Every one of these inners is a bare `fn` (private), and the same crate already carries the canonical form — `fn zrbtdrc_charge_impl` and `fn zrbtdrc_quench_impl` in `rbtdrc_crucible.rs` wear the marker and raise no finding.
The crate is simply inconsistent with itself, and the repair picks its own existing specimen.

Ruled only where `_impl` is the **sole** child; where the parent has other children besides, renaming the inner clears nothing and the finding rides its family's ruling instead.

### M4 — crate-head (8 findings, 8 renames)

`Tools/rbk/rbtd/src/main.rs` declares `rbtdb_*`.
A toolchain-fixed filename holds no prefix of its own, so declarations there answer to the **crate directory** — here `rbtd`, not `rbtdb`.
The repair spells them `rbtd_*`.

The gate confirms `rbtd_*` is free in both directions, and `rbtdb` is an undocumented family the RBK acronym map never claimed, so folding it also retires an unmapped prefix.

### M5 — file-head (2 findings, 2 renames)

`Tools/rbk/rbndb_base.sh` declares two public functions under a banner reading "External Functions (rbrd_\*)", wearing the depot-tripwire regime's prefix rather than the file's.
The file's own private helpers are already `zrbndb_*`.
The repair spells the two public verbs `rbndb_*`.

The regime's serialized surface is untouched: `rbrd.env`'s variables are `RBRD_*` in upper case, and these are lower-case function names.

### M6 — rbthd de-duplication (7 findings, no renames)

Six exact collisions and one exclusivity breach in the veiled `rbthd` crate are one defect, and it is **not a naming defect**.
`RBTHDR_COL_SUITE` (`"rbw-ts."`) and `RBTHDR_TT_SUBDIR` (`"tt"`) are each declared twice with identical values; `ZRBTHDT_CLEAN` and `ZRBTHDT_PLANTED` twice apiece; `zrbthdr_find_tt` and `zrbthdr_git` twice apiece with identical signatures.
`RBTHDR_TT_SUBDIR_REL` is a third copy of `"tt"` under a name that extends the second.

The repair is to single-home each and have the other sites use it.
Renaming either copy would entrench the duplication rather than repair it, which is why this family takes no mapping set.
`zrbthdr_git` / `zrbthdr_git_c` is **not** part of it — those are genuinely different functions — and stays with D4.

## Deferrals

### D1 — the `BUWGC_` family (15 findings): out of jurisdiction

Fifteen `BUWGC_*` colophon constants stand in `Tools/rbk/rbtd/src/rbtdgc_consts.rs`, whose own head is `rbtdgc`.
The site is native, which is why the census bars it here — but every repair reaches into the parcel drop-zone:

- the prefix is declared by `Tools/buk/buwz_zipper.sh` (`buz_tome "buwz" "BUWGC_" "BUWZ_"`);
- the emit is `buz_emit_colophon_consts` in `Tools/buk/buz_zipper.sh`, which writes both the `RBTDGC_` and `BUWGC_` blocks into one stream, so splitting them into separate files is equally a BUK-side change.

An edit beneath `Tools/buk/` is deleted whole by the next parcel install, so a repair made here is destroyed work.

This is structural rather than accidental: BUK colophon constants projected into any consumer's generated file will stand away from home in **every** consumer tree, forever.
The durable answer is therefore a home-side one — rename the family, split the emit, or declare `BUWGC_` incardinated at BUK's spec so the residency is sanctioned estate-wide — and none of the three is recipemuster's to make.

### D2 — the ifrit local-helper convention (8 findings): whole-file, not eight names

Eight `build_*` helpers in `rbmm_moorings/rbmv_vessels/common-ifrit-context/src/rbida_sorties.rs` wear no prefix at all.
So do their neighbours — `env_require`, `fail`, `pass`, `random_hex`, `dig_resolve`, `tcp_probe`, `ip_checksum`, and more.
The file is written throughout in an unprefixed local style.

The eight bar and the rest are advisory for one reason only: the barring test is `signet.contains('_')`, so `build_icmp_echo` bars and `fail` does not.
That is a property of the test, not a difference in kind between the names.

Renaming only the eight would leave the file half-converted on a line no reader could reconstruct.
The honest unit is the file's whole convention, and adopting `zrbida_*` across it is a real change needing a crucible-tier proof, since the ifrit builds inside the container and no `reveille` run touches it.
Deferred as its own decision rather than smuggled in as eight mechanical renames.

### D3 — the `rbtdrk_` family (5 findings): needs a homing decision

`rbtdrk_depot.rs` and `rbtdrk_freehold.rs` share the head `rbtdrk`, and both filenames seat signets their contents extend.
The family is already tangled: `rbtdrk_freehold_ensure` and `rbtdrk_freehold_ensure_impl` are declared in `rbtdrk_depot.rs`, not in the freehold file.

Segment-drop is unavailable — it would flatten `rbtdrk_depot_*` and `rbtdrk_freehold_*` into one namespace and erase the distinction the two files exist to draw.
The repair needs a decision about how the two subjects are homed, which a mapping set cannot carry.

### D4 — the variant-suffix and elaboration class (52 findings): routed to ₣Bj

The largest deferral, and the one that most needs its reason on the record rather than a shrug.

Every member has one shape: a seated name and a more specific sibling that extends it.
But the extending tails are almost all singletons — `_capture`, `_predicate`, `_apply`, `_gate`, `_top`, `_table`, `_global`, `_single`, `_for`, `_c`, `_with_args`, `_manor`, `_vessel`, `_external`, `_neg` — so there is no formula, only fifty-two individual naming decisions.

Three things make this a class rather than a residue:

- **The conventions generate the breaches.** `_capture` (return by capture), `_predicate` (return a boolean), `_fatal` (the dying variant) are estate-wide conventions. A base verb can never coexist with its own conventional variant under this law, so the breach is produced by the convention as a matter of course.
- **It is the same question ₣Bj already recorded.** That paddock's "Test-name exclusivity — a structural collision class" says descriptive test naming produces elaborated siblings structurally, and that the large-family exclusivity-ruling pace weighs whether the answer is a carve-out, a convention that forecloses elaboration, or the full law with rename as the standing remedy. Members like `rbtdte_fail` / `rbtdte_fail_fast_stops_after_first_failure` and `zrbtdrc_darken_svg` / `zrbtdrc_darken_svg_maps_the_surveyed_palette` are exactly that class.
- **Four members raise a tool-law question, not a naming one.** `rbob_charge` / `rbob_charged`, `rbtdre_Tariff` / `rbtdre_TariffReport`, `rbtdri_read_burv_fact` / `rbtdri_read_burv_facts_multi`, and `rbthdr_Captured` / `rbthdr_CapturedBytes` breach with **no segment boundary between them**, because the validator's test is a raw `starts_with`. Whether the law should be segment-bounded is ₣Bj's to settle; renaming these in a consumer would bank a repair against a rule that may not survive.

One measurement is banked for whoever takes it up: of the 69 parent/child pairs in this class, 15 have a child that is both private and unmarked, where prepending the private marker would clear the breach mechanically. The remaining 54 need judgment. That 15 is an upper bound and not a recommendation — several of them, notably the paired `RBIDA_SEL_*` selectors, are peers rather than inner implementations, and marking one private would be wrong.

A consumer tree is the wrong place to settle an estate-wide convention, and this heat's own cinch — that findings converge at their home — points the same way.

## Mapping sets

The two couplings above bind every set: diagnostic strings and test-case name strings move with the name.
**No apply-pace may rewrite either memo of 260814** — the census names every signet in its pre-rename spelling, and a mechanical sweep would destroy the frozen record.

The six sets are clustered by occurrence overlap and are **pairwise disjoint**, computed over occurrence sites rather than declaration sites, per ₣Bj's standing rule.
Every set may therefore run in parallel with every other, in any order.
The two memos are excluded from the overlap computation for the reason just given; they are the only files all six would otherwise share.

| set | files touched |
|---|---|
| P1 | `rbtdro_onboarding.rs`, `rbtdto_onboarding.rs` |
| P2 | `claude-rbk-core.md`, `rbcc_constants.sh`, `rbfb_beckon.sh`, `rbfd_director.sh`, `rbfk_kludge.sh`, `rbfv_verify.sh`, `rbgp_payor.sh`, `rblds_spine.sh`, `rblm_cli.sh`, `rbndb_base.sh`, `rbq_qualify.sh`, `rbro_regime.sh`, `rbw_workbench.sh`, `rbz_zipper.sh` |
| P3 | `rbtdrh_chain.rs`, `rbtdrs_poison.rs`, `rbtdtk_freehold.rs` |
| P4 | `rbtdrd_dogfight.rs`, `rbtdrp_lifecycle.rs` |
| P5 | `main.rs`, `rbtdrj_touchstone.rs` |
| P6 | `rbthd/src/` — the veiled crate alone |

P2 is the one set that reaches a generator: `rbq_qualify_*` are enrolled in `rbz_zipper.sh` and invoked as CLI verb strings, so that set rebuilds and its generated files re-derive.

### P1 — rbtdro onboarding family (M1 and M2 composed; 16 renames)

| from | to |
|---|---|
| `rbtdro_onboarding_conclave_reliquary` | `rbtdro_conclave_reliquary` |
| `rbtdro_onboarding_conclave_reliquary_impl` | `zrbtdro_conclave_reliquary_impl` |
| `rbtdro_onboarding_kludge_ccyolo` | `rbtdro_kludge_ccyolo` |
| `rbtdro_onboarding_kludge_ccyolo_impl` | `zrbtdro_kludge_ccyolo_impl` |
| `rbtdro_onboarding_kludge_tadmor` | `rbtdro_kludge_tadmor_onboarding` |
| `rbtdro_onboarding_kludge_tadmor_impl` | `zrbtdro_kludge_tadmor_onboarding_impl` |
| `rbtdro_onboarding_ordain_airgap_chain` | `rbtdro_ordain_airgap_chain` |
| `rbtdro_onboarding_ordain_airgap_chain_impl` | `zrbtdro_ordain_airgap_chain_impl` |
| `rbtdro_onboarding_ordain_bind_plantuml` | `rbtdro_ordain_bind_plantuml` |
| `rbtdro_onboarding_ordain_bind_plantuml_impl` | `zrbtdro_ordain_bind_plantuml_impl` |
| `rbtdro_onboarding_ordain_conjure_jupyter` | `rbtdro_ordain_conjure_jupyter` |
| `rbtdro_onboarding_ordain_conjure_jupyter_impl` | `zrbtdro_ordain_conjure_jupyter_impl` |
| `rbtdro_onboarding_ordain_conjure_sentry` | `rbtdro_ordain_conjure_sentry` |
| `rbtdro_onboarding_ordain_conjure_sentry_impl` | `zrbtdro_ordain_conjure_sentry_impl` |
| `rbtdro_onboarding_ordain_graft_demo` | `rbtdro_ordain_graft_demo` |
| `rbtdro_onboarding_ordain_graft_demo_impl` | `zrbtdro_ordain_graft_demo_impl` |

### P2 — rbk shell families (M1 rbfb/rbgp/rbq + M5; 11 renames)

| from | to |
|---|---|
| `rbfb_beckon_hallmark` | `rbfb_hallmark` |
| `rbgp_payor_install` | `rbgp_install` |
| `rbq_qualify_colophons` | `rbq_colophons` |
| `rbq_qualify_completeness` | `rbq_completeness` |
| `rbq_qualify_context` | `rbq_context` |
| `rbq_qualify_fast` | `rbq_fast` |
| `rbq_qualify_release` | `rbq_release` |
| `rbq_qualify_rust_consts` | `rbq_rust_consts` |
| `rbq_qualify_shellcheck` | `rbq_shellcheck` |
| `rbrd_check` | `rbndb_check` |
| `rbrd_inscribe` | `rbndb_inscribe` |

### P3 — rbtd rust file-eponyms (M1 rbtdrh/rbtdrs/rbtdtk; 8 renames)

| from | to |
|---|---|
| `rbtdrh_chain_dies_at_non_chain_dispatch` | `rbtdrh_dies_at_non_chain_dispatch` |
| `rbtdrh_chain_multi_consumer` | `rbtdrh_multi_consumer` |
| `rbtdrh_chain_retry_after_failure` | `rbtdrh_retry_after_failure` |
| `rbtdrs_poison_optional` | `rbtdrs_optional` |
| `rbtdtk_freehold_base_shape` | `rbtdtk_base_shape` |
| `rbtdtk_freehold_disjoint_per_tincture` | `rbtdtk_disjoint_per_tincture` |
| `rbtdtk_freehold_dual_station_disjoint` | `rbtdtk_dual_station_disjoint` |
| `rbtdtk_freehold_prefix_compose` | `rbtdtk_prefix_compose` |

### P4 — rbtd lifecycle and dogfight impls (M2; 6 renames)

| from | to |
|---|---|
| `rbtdrd_build_run_lifecycle_impl` | `zrbtdrd_build_run_lifecycle_impl` |
| `rbtdrp_depot_live_disqualify_impl` | `zrbtdrp_depot_live_disqualify_impl` |
| `rbtdrp_depot_stand_up_impl` | `zrbtdrp_depot_stand_up_impl` |
| `rbtdrp_depot_tear_down_impl` | `zrbtdrp_depot_tear_down_impl` |
| `rbtdrp_tripwire_confirm_impl` | `zrbtdrp_tripwire_confirm_impl` |
| `rbtdrp_tripwire_recover_impl` | `zrbtdrp_tripwire_recover_impl` |

### P5 — rbtd crate-head (M4; 8 renames)

| from | to |
|---|---|
| `rbtdb_Roots` | `rbtd_Roots` |
| `rbtdb_allocate_roots` | `rbtd_allocate_roots` |
| `rbtdb_list_fixtures` | `rbtd_list_fixtures` |
| `rbtdb_list_suites` | `rbtd_list_suites` |
| `rbtdb_run_dowse` | `rbtd_run_dowse` |
| `rbtdb_run_fixture` | `rbtd_run_fixture` |
| `rbtdb_run_single` | `rbtd_run_single` |
| `rbtdb_run_suite` | `rbtd_run_suite` |

### P6 — rbthd de-duplication (M6; no renames)

| duplicated signet | sites | value / shape |
|---|---|---|
| `RBTHDR_COL_SUITE` | `rbthdr_docimasy.rs:50`, `rbthdr_essai.rs:43` | `"rbw-ts."` — identical |
| `RBTHDR_TT_SUBDIR` | `rbthdr_docimasy.rs:54`, `rbthdr_essai.rs:70` | `"tt"` — identical |
| `RBTHDR_TT_SUBDIR_REL` | `rbthdr_expede.rs:133` | `"tt"` — a third copy under an extending name |
| `ZRBTHDT_CLEAN` | `rbthdt_loupe.rs:80`, `rbthdt_perambulation.rs:138` | identical slice |
| `ZRBTHDT_PLANTED` | `rbthdt_loupe.rs:72`, `rbthdt_perambulation.rs:131` | identical slice |
| `zrbthdr_find_tt` | `rbthdr_docimasy.rs:203`, `rbthdr_essai.rs:353` | identical signature |
| `zrbthdr_git` | `rbthdr_essai.rs:390`, `rbthdr_expede.rs:593` | identical signature |
