# Heat Trophy: rbk-11-mvp-parallel-cleanups

**Firemark:** ₣BA
**Created:** 260426
**Retired:** 260514
**Status:** retired

## Paddock

## Context

Sibling heat to ₣A_ (rbk-mvp-3-resource-prefix-and-depot-regen). Holds the post-AAK paces whose work is independent of AAE depot regeneration, so they can race in parallel to ₣A_'s live-infra spine. Intended execution pattern: a second officium working in `../rbm_beta_recipemuster` while the primary officium continues ₣A_ in the main tree.

All design decisions are baked into individual dockets — agents executing here should expect mechanical-apply work with explicit file lists and verbatim verification gates.

## Cross-officium discipline

This heat shares the repo with ₣A_; the parallel-tree split is geometric, not semantic. File-level boundaries enforce coordination — a pace's docket may carry an explicit blacklist of files belonging to ₣A_'s active paces, and those constraints persist across directories. When in doubt, additive only: no destructive git operations, no fixing "wrong" repo state without asking, commits land only the file lists each docket specifies.

## Test-suite reservation

During the parallel period, this heat runs fast suite only. Crucible, service, and complete suites are reserved for ₣A_'s live-infra paces — they share regime state and container/network namespaces, and concurrent runs will fail. Once ₣A_'s burn-in clears (or this heat's tail crosses with ₣A_'s wrap), the restriction can lift.

## Live-GCP coordination

Most paces in this heat are pure local refactor or spec churn — no GCP cost. Where a docket does call for live infrastructure, it carries explicit cost notes. Coordinate any billable runs against ₣A_'s active spend; ₣A_ owns the heavier live-infra workload during the parallel period.

## References

- Parent heat ₣A_ paddock — design history for resource prefixing (`RBRR_CLOUD_PREFIX`, `RBRR_RUNTIME_PREFIX`), GAR categorical layout migration (hallmarks/reliquaries/enshrines), payor subdir migration. Vocabulary established there is assumed background.
- BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) — referenced for cross-module shellcheck discipline study.

## Paces

### imageops-broaden (₢BAAAD) [complete]

**[260426-0731] complete**

## Character

Mid-pace pickup. The wide mechanical bash/Rust refactor is **done and verified green** (notched at e40a2b59, fast suite 75/75, handbook 15/15). What's left is **doc churn**: spec doc updates, one new spec mint, and a small verb-table edit in CLAUDE.consumer.md. Posture: read existing spec docs to learn the house style, then write the broadened versions and the new RBSIM. The hard architectural decisions are already baked into the code below — the docs need to *describe* what the code does, not re-argue the design.

## Docket

Broaden the `rbw-i` (image) tabtarget family from hallmark-only to three-domain symmetric (hallmarks, reliquaries, enshrinements). Adds `muster` as the catalog-listing verb.

### Status: implementation complete (notched e40a2b59)

**Code shipped, verified green:**
- 11 tabtargets enrolled (3 renamed, 8 new) — all executable in `tt/`
- Foundry-side functions: `rbfl_rekon_hallmark`, `rbfl_rekon_reliquary`, `rbfl_wrest`, `rbfl_jettison`, `rbfl_muster_hallmarks`, `rbfl_muster_reliquaries`, `rbfl_muster_enshrinements`
- Helpers generalized: `zrbfc_list_packages_capture(token, subtree_root)` + new `zrbfc_list_anchors_capture(token, subtree_root)` (1-deep for enshrinements). Old `zrbfc_list_hallmarks_capture` and `ZRBFC_HALLMARK_LIST_FILE` retired
- Manifest constants in `rbtdrm_manifest.rs` renamed to `_HALLMARK` suffix where domain-applicable; 8 new constants added; 5 callsites in `rbtdrc_crucible.rs` updated
- Tabtarget context doc Image table rewritten
- Handbook prose: 2 sites (`rbhocd_credential_director.sh`, `rbhodf_director_first_build.sh`) updated `RBZ_REKON_IMAGE` → `RBZ_REKON_HALLMARK`

### Locked design decisions (do not re-litigate)

1. **`rbfr_wrest` was relocated to `rbfl_wrest`** in `rbfl_FoundryLedger.sh`. It was the only Director-tier function inside the Retriever module — verified by inspecting all `rbfr_*` functions and their credential usage. `rbfr_summon` correctly stays in `rbfr_FoundryRetriever.sh` (uses Retriever creds). Module header updated.

2. **Helper generalization (option 2a chosen)**: `zrbfc_list_hallmarks_capture` was generalized rather than duplicated. The hallmark-specific `ZRBFC_HALLMARK_LIST_FILE` constant was renamed to `ZRBFC_PACKAGE_LIST_FILE`. Three callers updated: `rbfl_tally`, `rbfl_rekon_hallmark`, and `rbfv_FoundryVerify.sh:789`.

3. **Folio channel = `""` (empty)**, NOT `param1`, for all 11 enrollments. The docket text said "param1" but `param1` channel *consumes* the first arg into `BUZ_FOLIO` and strips it from the function's positional args (see `Tools/buk/buz_zipper.sh:263-268`). Existing rekon/jettison/wrest functions read `${1:-}`, so `""` is the correct channel — args flow through positional. The docket author was using "param1" loosely.

4. **Function naming asymmetry is load-bearing**: `rbfl_jettison` and `rbfl_wrest` stay undotted because they are locator-based and domain-blind (work on any package-path:tag). `rbfl_rekon_hallmark` vs `rbfl_rekon_reliquary` are domain-specific because they enumerate different subtrees with different schemas.

### Remaining work — three discrete tasks

#### 1. Spec docs (4 files)

Read `Tools/rbk/vov_veiled/RBSIR-image_rekon.adoc`, `RBSIJ-image_jettison.adoc`, `RBSIW-image_wrest.adoc` first to learn house style (linked-term patterns, attribute references, anchor definitions per MCM).

- **`RBSIR-image_rekon.adoc`** — broaden from hallmark-only to two-domain (hallmark + reliquary). Add reliquary section. Note that rekon-on-enshrinement is intentionally omitted (degenerate: enshrinements are 1-deep, single image; muster + wrest cover the workflow).

- **`RBSIJ-image_jettison.adoc`** — broaden to three-domain. Implementation note: same `rbfl_jettison` function backs all three tabtargets (locator-generic). Flag interior-hole risk for reliquary jettison (operator accepts piecemeal-by-image semantics; recovery is nuclear via re-inscribe).

- **`RBSIW-image_wrest.adoc`** — broaden to three-domain. Note: `rbfl_wrest` is locator-generic and Director-tier (was relocated from `rbfr_FoundryRetriever.sh` — see decision 1 above).

- **Mint `RBSIM-image_muster.adoc`** — new spec for muster verb. Covers all three domains' catalog listing. Distinguish from tally (`rbw-ft`): tally is retriever-tier with health-state computation; muster is director-tier, just a flat list of identifiers. Three muster functions: `rbfl_muster_hallmarks`, `rbfl_muster_reliquaries`, `rbfl_muster_enshrinements`.

- **Register `RBSIM` in CLAUDE.md File Acronym Mappings** under `### Tools Directory (`Tools/`) → #### RBK Subdirectory` — alphabetical placement between RBSIJ and RBSIR (or wherever IM sorts).

#### 2. Consumer-facing verb doc (1 file)

`Tools/rbk/vov_veiled/CLAUDE.consumer.md` — verb tables around lines 54-66 currently mention `wrest` and `jettison` but not `muster`. Two edits:

- Add `muster` verb to a verb table (likely under "How do I verify and inspect images?" alongside tally — or mint a new section "How do I list registry contents?" with muster + tally side-by-side, distinguishing roles).
- Broaden the `wrest` and `jettison` rows to acknowledge they now apply across hallmark/reliquary/enshrinement domains via locator format. Wording shouldn't get long — current style is one-line verb descriptions.

#### 3. Final verification

- `tt/rbtd-s.TestSuite.fast.sh` — should still be green; the spec doc changes don't touch fixtures
- `tt/rbw-tf.QualifyFast.sh` — confirms tabtarget/colophon/nameplate health green after the rename
- Behavioral spot-check (optional, requires GCP creds): run `tt/rbw-irh.DirectorRekonsHallmark.sh <existing-hallmark>` and confirm output matches what `rbw-ir` produced pre-rename

### Out of scope (unchanged from original docket)

- Integrity-check verb (folded into Pace 2's ordain-internal precheck — no user-invocable form)
- Theurge fixtures for the new ops (Pace 3)
- Auth tier review for tally (no change — tally stays retriever)
- Catalog-listing for hallmarks via tally (different role; `rbw-imh` is director-side parallel, deliberately added for symmetry)
- Backwards-compatibility shims for renamed colophons (hard cutover per heat discipline)

### Verification gate before wrap

All four below must be green:
- `tt/rbtd-s.TestSuite.fast.sh` (75 cases)
- `tt/rbtd-r.Run.handbook-render.sh` (15 cases — included in fast suite, but worth eyeballing separately if any handbook prose changes during this work)
- `tt/rbw-tf.QualifyFast.sh`
- `tt/rbtd-b.Build.sh` (theurge crate compiles)

### Affiliated context

Structural foundation for Paces 2 and 3 in the imageops sequence.

**[260426-0713] rough**

## Character

Mid-pace pickup. The wide mechanical bash/Rust refactor is **done and verified green** (notched at e40a2b59, fast suite 75/75, handbook 15/15). What's left is **doc churn**: spec doc updates, one new spec mint, and a small verb-table edit in CLAUDE.consumer.md. Posture: read existing spec docs to learn the house style, then write the broadened versions and the new RBSIM. The hard architectural decisions are already baked into the code below — the docs need to *describe* what the code does, not re-argue the design.

## Docket

Broaden the `rbw-i` (image) tabtarget family from hallmark-only to three-domain symmetric (hallmarks, reliquaries, enshrinements). Adds `muster` as the catalog-listing verb.

### Status: implementation complete (notched e40a2b59)

**Code shipped, verified green:**
- 11 tabtargets enrolled (3 renamed, 8 new) — all executable in `tt/`
- Foundry-side functions: `rbfl_rekon_hallmark`, `rbfl_rekon_reliquary`, `rbfl_wrest`, `rbfl_jettison`, `rbfl_muster_hallmarks`, `rbfl_muster_reliquaries`, `rbfl_muster_enshrinements`
- Helpers generalized: `zrbfc_list_packages_capture(token, subtree_root)` + new `zrbfc_list_anchors_capture(token, subtree_root)` (1-deep for enshrinements). Old `zrbfc_list_hallmarks_capture` and `ZRBFC_HALLMARK_LIST_FILE` retired
- Manifest constants in `rbtdrm_manifest.rs` renamed to `_HALLMARK` suffix where domain-applicable; 8 new constants added; 5 callsites in `rbtdrc_crucible.rs` updated
- Tabtarget context doc Image table rewritten
- Handbook prose: 2 sites (`rbhocd_credential_director.sh`, `rbhodf_director_first_build.sh`) updated `RBZ_REKON_IMAGE` → `RBZ_REKON_HALLMARK`

### Locked design decisions (do not re-litigate)

1. **`rbfr_wrest` was relocated to `rbfl_wrest`** in `rbfl_FoundryLedger.sh`. It was the only Director-tier function inside the Retriever module — verified by inspecting all `rbfr_*` functions and their credential usage. `rbfr_summon` correctly stays in `rbfr_FoundryRetriever.sh` (uses Retriever creds). Module header updated.

2. **Helper generalization (option 2a chosen)**: `zrbfc_list_hallmarks_capture` was generalized rather than duplicated. The hallmark-specific `ZRBFC_HALLMARK_LIST_FILE` constant was renamed to `ZRBFC_PACKAGE_LIST_FILE`. Three callers updated: `rbfl_tally`, `rbfl_rekon_hallmark`, and `rbfv_FoundryVerify.sh:789`.

3. **Folio channel = `""` (empty)**, NOT `param1`, for all 11 enrollments. The docket text said "param1" but `param1` channel *consumes* the first arg into `BUZ_FOLIO` and strips it from the function's positional args (see `Tools/buk/buz_zipper.sh:263-268`). Existing rekon/jettison/wrest functions read `${1:-}`, so `""` is the correct channel — args flow through positional. The docket author was using "param1" loosely.

4. **Function naming asymmetry is load-bearing**: `rbfl_jettison` and `rbfl_wrest` stay undotted because they are locator-based and domain-blind (work on any package-path:tag). `rbfl_rekon_hallmark` vs `rbfl_rekon_reliquary` are domain-specific because they enumerate different subtrees with different schemas.

### Remaining work — three discrete tasks

#### 1. Spec docs (4 files)

Read `Tools/rbk/vov_veiled/RBSIR-image_rekon.adoc`, `RBSIJ-image_jettison.adoc`, `RBSIW-image_wrest.adoc` first to learn house style (linked-term patterns, attribute references, anchor definitions per MCM).

- **`RBSIR-image_rekon.adoc`** — broaden from hallmark-only to two-domain (hallmark + reliquary). Add reliquary section. Note that rekon-on-enshrinement is intentionally omitted (degenerate: enshrinements are 1-deep, single image; muster + wrest cover the workflow).

- **`RBSIJ-image_jettison.adoc`** — broaden to three-domain. Implementation note: same `rbfl_jettison` function backs all three tabtargets (locator-generic). Flag interior-hole risk for reliquary jettison (operator accepts piecemeal-by-image semantics; recovery is nuclear via re-inscribe).

- **`RBSIW-image_wrest.adoc`** — broaden to three-domain. Note: `rbfl_wrest` is locator-generic and Director-tier (was relocated from `rbfr_FoundryRetriever.sh` — see decision 1 above).

- **Mint `RBSIM-image_muster.adoc`** — new spec for muster verb. Covers all three domains' catalog listing. Distinguish from tally (`rbw-ft`): tally is retriever-tier with health-state computation; muster is director-tier, just a flat list of identifiers. Three muster functions: `rbfl_muster_hallmarks`, `rbfl_muster_reliquaries`, `rbfl_muster_enshrinements`.

- **Register `RBSIM` in CLAUDE.md File Acronym Mappings** under `### Tools Directory (`Tools/`) → #### RBK Subdirectory` — alphabetical placement between RBSIJ and RBSIR (or wherever IM sorts).

#### 2. Consumer-facing verb doc (1 file)

`Tools/rbk/vov_veiled/CLAUDE.consumer.md` — verb tables around lines 54-66 currently mention `wrest` and `jettison` but not `muster`. Two edits:

- Add `muster` verb to a verb table (likely under "How do I verify and inspect images?" alongside tally — or mint a new section "How do I list registry contents?" with muster + tally side-by-side, distinguishing roles).
- Broaden the `wrest` and `jettison` rows to acknowledge they now apply across hallmark/reliquary/enshrinement domains via locator format. Wording shouldn't get long — current style is one-line verb descriptions.

#### 3. Final verification

- `tt/rbtd-s.TestSuite.fast.sh` — should still be green; the spec doc changes don't touch fixtures
- `tt/rbw-tf.QualifyFast.sh` — confirms tabtarget/colophon/nameplate health green after the rename
- Behavioral spot-check (optional, requires GCP creds): run `tt/rbw-irh.DirectorRekonsHallmark.sh <existing-hallmark>` and confirm output matches what `rbw-ir` produced pre-rename

### Out of scope (unchanged from original docket)

- Integrity-check verb (folded into Pace 2's ordain-internal precheck — no user-invocable form)
- Theurge fixtures for the new ops (Pace 3)
- Auth tier review for tally (no change — tally stays retriever)
- Catalog-listing for hallmarks via tally (different role; `rbw-imh` is director-side parallel, deliberately added for symmetry)
- Backwards-compatibility shims for renamed colophons (hard cutover per heat discipline)

### Verification gate before wrap

All four below must be green:
- `tt/rbtd-s.TestSuite.fast.sh` (75 cases)
- `tt/rbtd-r.Run.handbook-render.sh` (15 cases — included in fast suite, but worth eyeballing separately if any handbook prose changes during this work)
- `tt/rbw-tf.QualifyFast.sh`
- `tt/rbtd-b.Build.sh` (theurge crate compiles)

### Affiliated context

Structural foundation for Paces 2 and 3 in the imageops sequence.

**[260426-0622] rough**

Drafted from ₢A_AAV in ₣A_.

## Character

Mechanical-but-wide. The verb cosmology change is small (no new verbs minted; muster reused from JJ's heat-listing register applied to GAR categories) but the touch surface is broad: 11 tabtargets (3 renamed, 8 new), 4 spec docs (3 updated + 1 new), implementation across `rbfl_FoundryLedger.sh` and the depot family modules that today own inscribe/enshrine, plus prose churn across handbook tracks, Memos, Marshal zero template, four-mode test cases, manifest, and CLAUDE.md tabtarget context.

Posture: enumerate-and-translate. Most edits are sed-grade; the judgment work is in the file-by-file pass to confirm each occurrence still reads sensibly with the new colophon name.

## Docket

Broaden the `rbw-i` (image) tabtarget family from hallmark-only to three-domain symmetric: hallmarks, reliquaries, enshrinements. Adds `muster` as the catalog-listing verb (cult-coherent reuse of JJ's heat-listing register).

### Convention

Colophon shape: `rbw-i{op}{domain}` where:
- Operation letter: `r` rekon (lowercase, member-list), `m` muster (lowercase, catalog-list), `J` jettison (capital, mutating delete), `w` wrest (lowercase, registry-read pull)
- Domain letter: `h` hallmark, `r` reliquary, `e` enshrinement (always lowercase — domain is not the operation)

Director-tier across the entire family. Tally (`rbw-ft`) remains retriever-tier as the registry-state-survey command — different role, no overlap with director-side image ops.

### Tabtarget matrix

**Renames (3):**
- `rbw-ir` → `rbw-irh` (DirectorRekonsHallmark)
- `rbw-iJ` → `rbw-iJh` (DirectorJettisonsHallmarkImage)
- `rbw-iw` → `rbw-iwh` (DirectorWrestsHallmarkImage)

**New (8):**
- `rbw-irr` (DirectorRekonsReliquary, param1) — list tool images at one reliquary stamp
- `rbw-imh` (DirectorMustersHallmarks, no folio) — list all hallmarks
- `rbw-imr` (DirectorMustersReliquaries, no folio) — list all reliquary stamps
- `rbw-ime` (DirectorMustersEnshrinements, no folio) — list all enshrined bases
- `rbw-iwr` (DirectorWrestsReliquaryImage, param1) — pull one tool image from reliquary
- `rbw-iwe` (DirectorWrestsEnshrinedImage, param1) — pull one enshrined base
- `rbw-iJr` (DirectorJettisonsReliquaryImage, param1) — delete one tool image from reliquary
- `rbw-iJe` (DirectorJettisonsEnshrinement, param1) — delete one enshrinement

Total: 11 tabtargets. Rekon-on-enshrinement intentionally omitted (degenerate — enshrinement is one image; muster + wrest cover the workflow).

### Implementation surface (verify on touch — do not assume single site)

Three concerns split across multiple modules:

1. **Hallmark-domain ops** (`*h` suffix): already exist in `rbfl_FoundryLedger.sh` as the rekon/jettison/wrest functions backing `rbw-ir`/`rbw-iJ`/`rbw-iw`. Touch is light — internal renames where the function name encodes the operation.
2. **Reliquary-domain ops** (`*r` suffix): inscribe lives in the depot family (`rbw-dI` → `rbgji01-inscribe-mirror.sh` and adjacent). Reliquary rekon/muster/wrest/jettison need new functions; placement decision at pace start — prefer extending `rbfl_FoundryLedger.sh` if the helper functions there generalize cleanly (HEAD/DELETE machinery already exists), otherwise mint a sibling module.
3. **Enshrinement-domain ops** (`*e` suffix): enshrine lives in the depot family (`rbw-dE` → `rbgje01-enshrine-copy.sh` and adjacent). Same placement decision as reliquary.

GAR path constants in `rbgl_GarLayout.sh`: confirm `RBGC_RELIQUARIES_ROOT` and `RBGC_ENSHRINES_ROOT` (or equivalent) exist post-AAK; mint at pace start if absent.

### Per-tabtarget mechanical scope

For each new tabtarget:
- Create `tt/rbw-i*.<silks>.sh` shell script with standard launcher invocation
- Register colophon in `rbz_zipper.sh` (kit-side enrollment)
- Add BUC/BUW channel mapping (folio: imprint/param1/empty per matrix above)
- Add manifest constant in `rbtdrm_manifest.rs` (`RBTDRM_COLOPHON_*`)

For each renamed tabtarget: rename file under `tt/`, update zipper enrollment, update BUC/BUW mappings, update manifest constants.

### Spec doc fanout

- Update `RBSIR-image_rekon.adoc` — three-domain reach for rekon (member listing for hallmark + reliquary)
- Update `RBSIJ-image_jettison.adoc` — three-domain jettison; flag interior-hole risk for reliquary jettison (operator accepts piecemeal-by-image semantics; recovery is nuclear via re-inscribe)
- Update `RBSIW-image_wrest.adoc` — three-domain wrest
- Mint `RBSIM-image_muster.adoc` — new spec for muster verb; covers all three domains' catalog listing

Register `RBSIM` in CLAUDE.md File Acronym Mappings under RBK Subdirectory.

### Prose churn (full inventory before mechanical sed)

Grep-sweep with filename-anchored patterns (portable, no false-positive matches in prose): `rbw-ir\.`, `rbw-iJ\.`, `rbw-iw\.`. Expected sites:

- Handbook tracks (`Tools/rbk/rbh0/`)
- Onboarding tabtargets that reference image ops
- Memos that mention image-family commands
- `Tools/rbk/rbk-claude-tabtarget-context.md` (Image section gets full rewrite + Muster section added)
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` (verify on touch)
- Marshal zero template (verify; likely no change since marshal handles regimes not tabtargets)
- `rbtdrc_crucible.rs` four-mode test cases — `rekon` and `jettison` cases use the existing colophons; rename to `*h` suffix
- `rbtdrm_manifest.rs` — colophon constants for renamed tabtargets, plus new constants for the 8 added tabtargets

The grep sweep IS the pace's first work item — turns the touch surface from "wide" to "concrete."

### User-facing breakage note

Hard rename — operators invoking `rbw-ir`/`rbw-iJ`/`rbw-iw` after this pace lands get shell "command not found." Consonant with hard-cutover discipline elsewhere in the heat. Handbook tracks updated in same pace so onboarding flows surface the new names cleanly.

### Verification

- `tt/rbtd-s.TestSuite.fast.sh` — green (no regression on enrollment/manifest after constant additions)
- `tt/rbtd-s.TestSuite.crucible.sh` — green (four-mode rename catches any missed callsites)
- Behavioral spot-check: capture `rbw-ir <existing-hallmark>` output on green branch pre-rename (or `git stash`-based), compare against `rbw-irh <existing-hallmark>` post-rename — outputs should match
- Spot-check: `rbw-imr` runs cleanly against current depot
- `tt/rbw-tf.QualifyFast.sh` — tabtarget/colophon/nameplate health green after rename

### Out of scope

- Integrity-check verb (folded into Pace 2's ordain-internal precheck — no user-invocable form)
- Theurge fixtures for the new ops (Pace 3)
- Auth tier review for tally (no change — tally stays retriever)
- Catalog-listing for hallmarks via tally (different role, different command — `rbw-imh` is director-side parallel, deliberately added for symmetry so directors don't role-switch for inventory)
- Backwards-compatibility shims for the renamed colophons (hard cutover per heat discipline)

### Affiliated context

Structural foundation for Paces 2 and 3 in the imageops sequence.

**[260425-0930] rough**

## Character

Mechanical-but-wide. The verb cosmology change is small (no new verbs minted; muster reused from JJ's heat-listing register applied to GAR categories) but the touch surface is broad: 11 tabtargets (3 renamed, 8 new), 4 spec docs (3 updated + 1 new), implementation across `rbfl_FoundryLedger.sh` and the depot family modules that today own inscribe/enshrine, plus prose churn across handbook tracks, Memos, Marshal zero template, four-mode test cases, manifest, and CLAUDE.md tabtarget context.

Posture: enumerate-and-translate. Most edits are sed-grade; the judgment work is in the file-by-file pass to confirm each occurrence still reads sensibly with the new colophon name.

## Docket

Broaden the `rbw-i` (image) tabtarget family from hallmark-only to three-domain symmetric: hallmarks, reliquaries, enshrinements. Adds `muster` as the catalog-listing verb (cult-coherent reuse of JJ's heat-listing register).

### Convention

Colophon shape: `rbw-i{op}{domain}` where:
- Operation letter: `r` rekon (lowercase, member-list), `m` muster (lowercase, catalog-list), `J` jettison (capital, mutating delete), `w` wrest (lowercase, registry-read pull)
- Domain letter: `h` hallmark, `r` reliquary, `e` enshrinement (always lowercase — domain is not the operation)

Director-tier across the entire family. Tally (`rbw-ft`) remains retriever-tier as the registry-state-survey command — different role, no overlap with director-side image ops.

### Tabtarget matrix

**Renames (3):**
- `rbw-ir` → `rbw-irh` (DirectorRekonsHallmark)
- `rbw-iJ` → `rbw-iJh` (DirectorJettisonsHallmarkImage)
- `rbw-iw` → `rbw-iwh` (DirectorWrestsHallmarkImage)

**New (8):**
- `rbw-irr` (DirectorRekonsReliquary, param1) — list tool images at one reliquary stamp
- `rbw-imh` (DirectorMustersHallmarks, no folio) — list all hallmarks
- `rbw-imr` (DirectorMustersReliquaries, no folio) — list all reliquary stamps
- `rbw-ime` (DirectorMustersEnshrinements, no folio) — list all enshrined bases
- `rbw-iwr` (DirectorWrestsReliquaryImage, param1) — pull one tool image from reliquary
- `rbw-iwe` (DirectorWrestsEnshrinedImage, param1) — pull one enshrined base
- `rbw-iJr` (DirectorJettisonsReliquaryImage, param1) — delete one tool image from reliquary
- `rbw-iJe` (DirectorJettisonsEnshrinement, param1) — delete one enshrinement

Total: 11 tabtargets. Rekon-on-enshrinement intentionally omitted (degenerate — enshrinement is one image; muster + wrest cover the workflow).

### Implementation surface (verify on touch — do not assume single site)

Three concerns split across multiple modules:

1. **Hallmark-domain ops** (`*h` suffix): already exist in `rbfl_FoundryLedger.sh` as the rekon/jettison/wrest functions backing `rbw-ir`/`rbw-iJ`/`rbw-iw`. Touch is light — internal renames where the function name encodes the operation.
2. **Reliquary-domain ops** (`*r` suffix): inscribe lives in the depot family (`rbw-dI` → `rbgji01-inscribe-mirror.sh` and adjacent). Reliquary rekon/muster/wrest/jettison need new functions; placement decision at pace start — prefer extending `rbfl_FoundryLedger.sh` if the helper functions there generalize cleanly (HEAD/DELETE machinery already exists), otherwise mint a sibling module.
3. **Enshrinement-domain ops** (`*e` suffix): enshrine lives in the depot family (`rbw-dE` → `rbgje01-enshrine-copy.sh` and adjacent). Same placement decision as reliquary.

GAR path constants in `rbgl_GarLayout.sh`: confirm `RBGC_RELIQUARIES_ROOT` and `RBGC_ENSHRINES_ROOT` (or equivalent) exist post-AAK; mint at pace start if absent.

### Per-tabtarget mechanical scope

For each new tabtarget:
- Create `tt/rbw-i*.<silks>.sh` shell script with standard launcher invocation
- Register colophon in `rbz_zipper.sh` (kit-side enrollment)
- Add BUC/BUW channel mapping (folio: imprint/param1/empty per matrix above)
- Add manifest constant in `rbtdrm_manifest.rs` (`RBTDRM_COLOPHON_*`)

For each renamed tabtarget: rename file under `tt/`, update zipper enrollment, update BUC/BUW mappings, update manifest constants.

### Spec doc fanout

- Update `RBSIR-image_rekon.adoc` — three-domain reach for rekon (member listing for hallmark + reliquary)
- Update `RBSIJ-image_jettison.adoc` — three-domain jettison; flag interior-hole risk for reliquary jettison (operator accepts piecemeal-by-image semantics; recovery is nuclear via re-inscribe)
- Update `RBSIW-image_wrest.adoc` — three-domain wrest
- Mint `RBSIM-image_muster.adoc` — new spec for muster verb; covers all three domains' catalog listing

Register `RBSIM` in CLAUDE.md File Acronym Mappings under RBK Subdirectory.

### Prose churn (full inventory before mechanical sed)

Grep-sweep with filename-anchored patterns (portable, no false-positive matches in prose): `rbw-ir\.`, `rbw-iJ\.`, `rbw-iw\.`. Expected sites:

- Handbook tracks (`Tools/rbk/rbh0/`)
- Onboarding tabtargets that reference image ops
- Memos that mention image-family commands
- `Tools/rbk/rbk-claude-tabtarget-context.md` (Image section gets full rewrite + Muster section added)
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` (verify on touch)
- Marshal zero template (verify; likely no change since marshal handles regimes not tabtargets)
- `rbtdrc_crucible.rs` four-mode test cases — `rekon` and `jettison` cases use the existing colophons; rename to `*h` suffix
- `rbtdrm_manifest.rs` — colophon constants for renamed tabtargets, plus new constants for the 8 added tabtargets

The grep sweep IS the pace's first work item — turns the touch surface from "wide" to "concrete."

### User-facing breakage note

Hard rename — operators invoking `rbw-ir`/`rbw-iJ`/`rbw-iw` after this pace lands get shell "command not found." Consonant with hard-cutover discipline elsewhere in the heat. Handbook tracks updated in same pace so onboarding flows surface the new names cleanly.

### Verification

- `tt/rbtd-s.TestSuite.fast.sh` — green (no regression on enrollment/manifest after constant additions)
- `tt/rbtd-s.TestSuite.crucible.sh` — green (four-mode rename catches any missed callsites)
- Behavioral spot-check: capture `rbw-ir <existing-hallmark>` output on green branch pre-rename (or `git stash`-based), compare against `rbw-irh <existing-hallmark>` post-rename — outputs should match
- Spot-check: `rbw-imr` runs cleanly against current depot
- `tt/rbw-tf.QualifyFast.sh` — tabtarget/colophon/nameplate health green after rename

### Out of scope

- Integrity-check verb (folded into Pace 2's ordain-internal precheck — no user-invocable form)
- Theurge fixtures for the new ops (Pace 3)
- Auth tier review for tally (no change — tally stays retriever)
- Catalog-listing for hallmarks via tally (different role, different command — `rbw-imh` is director-side parallel, deliberately added for symmetry so directors don't role-switch for inventory)
- Backwards-compatibility shims for the renamed colophons (hard cutover per heat discipline)

### Affiliated context

Structural foundation for Paces 2 and 3 in the imageops sequence.

**[260425-0920] rough**

## Character

Mechanical-but-wide. The verb cosmology change is small (no new verbs minted; muster reused from JJ's heat-listing register applied to GAR categories) but the touch surface is broad: 11 tabtargets (3 renamed, 8 new), 4 spec docs (3 updated + 1 new), implementation in `rbfl_FoundryLedger.sh`, plus prose churn across handbook tracks, Memos, Marshal zero template, four-mode test cases, manifest, and CLAUDE.md tabtarget context.

Posture: enumerate-and-translate. Most edits are sed-grade; the judgment work is in the file-by-file pass to confirm each occurrence still reads sensibly with the new colophon name.

## Docket

Broaden the `rbw-i` (image) tabtarget family from hallmark-only to three-domain symmetric: hallmarks, reliquaries, enshrinements. Adds `muster` as the catalog-listing verb (cult-coherent reuse of JJ's heat-listing register).

### Convention

Colophon shape: `rbw-i{op}{domain}` where:
- Operation letter: `r` rekon (lowercase, member-list), `m` muster (lowercase, catalog-list), `J` jettison (capital, mutating delete), `w` wrest (lowercase, registry-read pull)
- Domain letter: `h` hallmark, `r` reliquary, `e` enshrinement (always lowercase — domain is not the operation)

Director-tier across the entire family. Tally (`rbw-ft`) remains retriever-tier as the registry-state-survey command — different role, no overlap with director-side image ops.

### Tabtarget matrix

**Renames (3):**
- `rbw-ir` → `rbw-irh` (DirectorRekonsHallmark)
- `rbw-iJ` → `rbw-iJh` (DirectorJettisonsHallmarkImage)
- `rbw-iw` → `rbw-iwh` (DirectorWrestsHallmarkImage)

**New (8):**
- `rbw-irr` (DirectorRekonsReliquary, param1) — list tool images at one reliquary stamp
- `rbw-imh` (DirectorMustersHallmarks, no folio) — list all hallmarks
- `rbw-imr` (DirectorMustersReliquaries, no folio) — list all reliquary stamps
- `rbw-ime` (DirectorMustersEnshrinements, no folio) — list all enshrined bases
- `rbw-iwr` (DirectorWrestsReliquaryImage, param1) — pull one tool image from reliquary
- `rbw-iwe` (DirectorWrestsEnshrinedImage, param1) — pull one enshrined base
- `rbw-iJr` (DirectorJettisonsReliquaryImage, param1) — delete one tool image from reliquary
- `rbw-iJe` (DirectorJettisonsEnshrinement, param1) — delete one enshrinement

Total: 11 tabtargets. Rekon-on-enshrinement intentionally omitted (degenerate — enshrinement is one image; muster + wrest cover the workflow).

### Implementation site

Extend `rbfl_FoundryLedger.sh` — already owns the GAR enumeration/abjure machinery for hallmarks. Muster + the reliquary/enshrinement variants of rekon/jettison/wrest slot in alongside.

Verify path constants in `rbgl_GarLayout.sh` cover reliquary-root and enshrines-root constructions; mint `RBGC_*` additions if any are missing.

### Spec doc fanout

- Update `RBSIR-image_rekon.adoc` — three-domain reach for rekon (member listing for hallmark + reliquary)
- Update `RBSIJ-image_jettison.adoc` — three-domain jettison; flag interior-hole risk for reliquary jettison (operator accepts piecemeal-by-image semantics; recovery is nuclear)
- Update `RBSIW-image_wrest.adoc` — three-domain wrest
- Mint `RBSIM-image_muster.adoc` — new spec for muster verb; covers all three domains' catalog listing

Register `RBSIM` in CLAUDE.md File Acronym Mappings under RBK Subdirectory.

### Prose churn (full inventory before mechanical sed)

Grep-sweep `rbw-ir\b`, `rbw-iJ\b`, `rbw-iw\b` across the tree. Expected sites:
- Handbook tracks (`Tools/rbk/rbh0/`)
- Onboarding tabtargets that reference image ops
- Memos that mention image-family commands
- `Tools/rbk/rbk-claude-tabtarget-context.md` (Image section gets full rewrite + Muster section added)
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` (verify)
- Marshal zero template (verify)
- `rbtdrc_crucible.rs` four-mode test cases — `rekon` and `jettison` cases use the existing colophons; rename to `*h` suffix
- `rbtdrm_manifest.rs` — colophon constants for renamed tabtargets, plus new constants for the 8 added tabtargets

The grep sweep IS the pace's first work item — turns the touch surface from "wide" to "concrete."

### Verification

- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (no regression on enrollment/manifest after constant additions)
- `tt/rbtd-s.TestSuite.crucible.sh` — green (four-mode rename catches any missed callsites)
- Spot-check: `rbw-irh <existing-hallmark>` produces same output as old `rbw-ir <existing-hallmark>`
- Spot-check: `rbw-imr` runs cleanly against current depot
- `tt/rbw-tf.QualifyFast.sh` — tabtarget/colophon/nameplate health green after rename

### Out of scope

- Integrity-check verb (folded into Pace 2's ordain-internal precheck — no user-invocable form)
- Theurge fixtures for the new ops (Pace 3)
- Auth tier review for tally (no change — tally stays retriever)
- Catalog-listing for hallmarks via tally (different role, different command — `rbw-imh` is director-side parallel, deliberately added for symmetry so directors don't role-switch for inventory)

### Affiliated context

Structural foundation for Paces 2 and 3 in the imageops sequence.

### imageops-precheck (₢BAAAE) [complete]

**[260426-0841] complete**

Drafted from ₢A_AAW in ₣A_.

## Character

Single concentrated edit at the ordain submission boundary. Small diff, real safety upside — catches missing-ref class of bugs before consuming Cloud Build minutes. No new tabtarget, no new verb. The integrity check exists ONLY as an ordain pre-step; the standalone diagnostic use case is covered by the structured error message itself.

Front-loaded design decision: the source of truth for "which reliquary tool images does this vessel need" is unspecified by the existing layout — pick at pace start (see Open design question below).

## Docket

Add a precheck inside the ordain pipeline that resolves vessel ref dependencies against GAR before submitting Cloud Build. Fail loudly with nuclear-rebuild advice on any missing ref.

### Open design question — resolve at pace start

`RBRV_RELIQUARY=<stamp>` tells the precheck WHICH reliquary, but not WHICH tools from it the vessel consumes. Three options, pick one:

1. **Vessel-regime extension**: add `RBRV_RELIQUARY_TOOLS` (or similar) declaring the tool list. Most explicit; requires regime schema work and per-vessel docket updates.
2. **Full-subtree presence check**: precheck enumerates every image under `<prefix>reliquaries/<stamp>/` and verifies all are present. Catches piecemeal jettison on the whole stamp; doesn't distinguish tools the vessel actually needs from tools it doesn't. Conservative — flags integrity damage even on tools the vessel ignores.
3. **Cloudbuild-substitution parse**: derive tool list from the rendered cloudbuild.json's `_RBG?_*` tool-image substitutions before submission. Brittle (parser tracks producer-side grammar) but precise.

Recommend option 2 for v1 simplicity (any tool missing means the stamp's integrity is questionable; conservative failure is acceptable). Reconsider if false-positive rate is high in practice.

### Operation

Before Cloud Build submission in conjure and bind ordain modes:

1. Resolve the vessel's `RBRV_IMAGE_ORIGIN` against `<prefix>enshrines/<base>` — confirm the enshrined base image exists at the expected GAR path.
2. If the vessel is reliquary-yoked (`RBRV_RELIQUARY` set), apply the chosen tool-list resolution (above) and confirm each expected image is present.
3. On any miss, exit non-zero before any Cloud Build call. Error message format (nuclear advice only):

```
PRECHECK: GAR image not found at <prefix>enshrines/<base>
  Required by <vessel>'s RBRV_IMAGE_ORIGIN.
  Recover by re-enshrining: tt/rbw-dE.DirectorEnshrinesVessel.sh
```

```
PRECHECK: GAR image not found at <prefix>reliquaries/<stamp>/<tool>
  Required by <vessel>'s RBRV_RELIQUARY=<stamp>.
  Recover by re-inscribing: tt/rbw-dI.DirectorInscribesReliquary.sh
```

No surgical recovery hints. If a ref is broken, the operator re-creates the whole reliquary or re-enshrines the base — the cosmology accepts that piecemeal jettison is allowed but unrecoverable surgically.

### Implementation site

`rbfd_FoundryDirectorBuild.sh` (or its caller in `rbfd_cli.sh`). Add a function `zrbfd_precheck_refs <vessel-path> <mode>` invoked at the start of conjure and bind paths. Caller passes the mode; function does not redetect mode from regime. Function reads vessel's RBRV regime, performs HEAD requests against the resolved GAR paths, accumulates misses, prints the structured error message(s), exits non-zero on any miss.

Reuses existing GAR auth path (whatever conjure already uses for its REST calls — director-tier credentials).

### Mode coverage

- **Conjure** — precheck: enshrined base required; reliquary refs if yoked.
- **Bind** — precheck: enshrined base required; reliquary refs if yoked.
- **Graft** — skip: graft is local-image-push, no GCB-side ref consumption.
- **Enshrine** (`rbw-dE`) — skip: input is upstream image (Docker Hub / similar), not GAR.
- **Inscribe** (`rbw-dI`) — skip: input is upstream tool images, not GAR.
- **Yoke** (`rbw-dY`) — out of scope for this pace. Yoke-time stamp existence check is a sensible follow-up; surface as paddock note if the gap matters in practice.

### Cost / latency note

Per-call HEAD requests scale with reliquary size. For a vessel yoked to a stamp with 20 tool images, precheck adds ~21 HEAD requests (1 enshrine + 20 reliquary) before any Cloud Build call. Expected sub-second total against GAR. Acceptable overhead given the failure-mode-cost asymmetry (failed Cloud Build wastes minutes-to-hours of build time).

### Verification

- Negative: jettison the enshrined base of a test vessel, attempt conjure → precheck fails with the structured message; no Cloud Build submission occurs (verify by absence of build in Cloud Build console).
- Negative: jettison one tool image from a reliquary, attempt bind on a vessel yoked to that reliquary → precheck fails with structured message naming the missing tool.
- Positive: clean conjure → precheck passes silently, build submits as before (no observable change in normal-path latency beyond the small HEAD-request overhead).
- Verify graft path is unaffected (no precheck call in graft codepath; no overhead).

### Out of scope

- User-invocable integrity command (deliberately not minted; precheck output covers the diagnostic use case)
- Surgical recovery hints (nuclear advice only)
- Caching the precheck result across multiple ordain calls (per-call freshness — refs can drift between calls)
- Precheck for graft / enshrine / inscribe modes (as enumerated above)
- Yoke-time stamp existence check (follow-up consideration, not blocking)

### Affiliated context

Builds on Pace 1's tabtarget vocabulary (the precheck error message references `rbw-dE` and `rbw-dI` recovery commands; those names are stable, predate this sequence). The precheck logic itself uses GAR HEAD requests — independent of the rekon/muster/jettison machinery extended in Pace 1, so this pace can land independent of Pace 1's status if priority shifts.

**[260426-0622] rough**

Drafted from ₢A_AAW in ₣A_.

## Character

Single concentrated edit at the ordain submission boundary. Small diff, real safety upside — catches missing-ref class of bugs before consuming Cloud Build minutes. No new tabtarget, no new verb. The integrity check exists ONLY as an ordain pre-step; the standalone diagnostic use case is covered by the structured error message itself.

Front-loaded design decision: the source of truth for "which reliquary tool images does this vessel need" is unspecified by the existing layout — pick at pace start (see Open design question below).

## Docket

Add a precheck inside the ordain pipeline that resolves vessel ref dependencies against GAR before submitting Cloud Build. Fail loudly with nuclear-rebuild advice on any missing ref.

### Open design question — resolve at pace start

`RBRV_RELIQUARY=<stamp>` tells the precheck WHICH reliquary, but not WHICH tools from it the vessel consumes. Three options, pick one:

1. **Vessel-regime extension**: add `RBRV_RELIQUARY_TOOLS` (or similar) declaring the tool list. Most explicit; requires regime schema work and per-vessel docket updates.
2. **Full-subtree presence check**: precheck enumerates every image under `<prefix>reliquaries/<stamp>/` and verifies all are present. Catches piecemeal jettison on the whole stamp; doesn't distinguish tools the vessel actually needs from tools it doesn't. Conservative — flags integrity damage even on tools the vessel ignores.
3. **Cloudbuild-substitution parse**: derive tool list from the rendered cloudbuild.json's `_RBG?_*` tool-image substitutions before submission. Brittle (parser tracks producer-side grammar) but precise.

Recommend option 2 for v1 simplicity (any tool missing means the stamp's integrity is questionable; conservative failure is acceptable). Reconsider if false-positive rate is high in practice.

### Operation

Before Cloud Build submission in conjure and bind ordain modes:

1. Resolve the vessel's `RBRV_IMAGE_ORIGIN` against `<prefix>enshrines/<base>` — confirm the enshrined base image exists at the expected GAR path.
2. If the vessel is reliquary-yoked (`RBRV_RELIQUARY` set), apply the chosen tool-list resolution (above) and confirm each expected image is present.
3. On any miss, exit non-zero before any Cloud Build call. Error message format (nuclear advice only):

```
PRECHECK: GAR image not found at <prefix>enshrines/<base>
  Required by <vessel>'s RBRV_IMAGE_ORIGIN.
  Recover by re-enshrining: tt/rbw-dE.DirectorEnshrinesVessel.sh
```

```
PRECHECK: GAR image not found at <prefix>reliquaries/<stamp>/<tool>
  Required by <vessel>'s RBRV_RELIQUARY=<stamp>.
  Recover by re-inscribing: tt/rbw-dI.DirectorInscribesReliquary.sh
```

No surgical recovery hints. If a ref is broken, the operator re-creates the whole reliquary or re-enshrines the base — the cosmology accepts that piecemeal jettison is allowed but unrecoverable surgically.

### Implementation site

`rbfd_FoundryDirectorBuild.sh` (or its caller in `rbfd_cli.sh`). Add a function `zrbfd_precheck_refs <vessel-path> <mode>` invoked at the start of conjure and bind paths. Caller passes the mode; function does not redetect mode from regime. Function reads vessel's RBRV regime, performs HEAD requests against the resolved GAR paths, accumulates misses, prints the structured error message(s), exits non-zero on any miss.

Reuses existing GAR auth path (whatever conjure already uses for its REST calls — director-tier credentials).

### Mode coverage

- **Conjure** — precheck: enshrined base required; reliquary refs if yoked.
- **Bind** — precheck: enshrined base required; reliquary refs if yoked.
- **Graft** — skip: graft is local-image-push, no GCB-side ref consumption.
- **Enshrine** (`rbw-dE`) — skip: input is upstream image (Docker Hub / similar), not GAR.
- **Inscribe** (`rbw-dI`) — skip: input is upstream tool images, not GAR.
- **Yoke** (`rbw-dY`) — out of scope for this pace. Yoke-time stamp existence check is a sensible follow-up; surface as paddock note if the gap matters in practice.

### Cost / latency note

Per-call HEAD requests scale with reliquary size. For a vessel yoked to a stamp with 20 tool images, precheck adds ~21 HEAD requests (1 enshrine + 20 reliquary) before any Cloud Build call. Expected sub-second total against GAR. Acceptable overhead given the failure-mode-cost asymmetry (failed Cloud Build wastes minutes-to-hours of build time).

### Verification

- Negative: jettison the enshrined base of a test vessel, attempt conjure → precheck fails with the structured message; no Cloud Build submission occurs (verify by absence of build in Cloud Build console).
- Negative: jettison one tool image from a reliquary, attempt bind on a vessel yoked to that reliquary → precheck fails with structured message naming the missing tool.
- Positive: clean conjure → precheck passes silently, build submits as before (no observable change in normal-path latency beyond the small HEAD-request overhead).
- Verify graft path is unaffected (no precheck call in graft codepath; no overhead).

### Out of scope

- User-invocable integrity command (deliberately not minted; precheck output covers the diagnostic use case)
- Surgical recovery hints (nuclear advice only)
- Caching the precheck result across multiple ordain calls (per-call freshness — refs can drift between calls)
- Precheck for graft / enshrine / inscribe modes (as enumerated above)
- Yoke-time stamp existence check (follow-up consideration, not blocking)

### Affiliated context

Builds on Pace 1's tabtarget vocabulary (the precheck error message references `rbw-dE` and `rbw-dI` recovery commands; those names are stable, predate this sequence). The precheck logic itself uses GAR HEAD requests — independent of the rekon/muster/jettison machinery extended in Pace 1, so this pace can land independent of Pace 1's status if priority shifts.

**[260425-0930] rough**

## Character

Single concentrated edit at the ordain submission boundary. Small diff, real safety upside — catches missing-ref class of bugs before consuming Cloud Build minutes. No new tabtarget, no new verb. The integrity check exists ONLY as an ordain pre-step; the standalone diagnostic use case is covered by the structured error message itself.

Front-loaded design decision: the source of truth for "which reliquary tool images does this vessel need" is unspecified by the existing layout — pick at pace start (see Open design question below).

## Docket

Add a precheck inside the ordain pipeline that resolves vessel ref dependencies against GAR before submitting Cloud Build. Fail loudly with nuclear-rebuild advice on any missing ref.

### Open design question — resolve at pace start

`RBRV_RELIQUARY=<stamp>` tells the precheck WHICH reliquary, but not WHICH tools from it the vessel consumes. Three options, pick one:

1. **Vessel-regime extension**: add `RBRV_RELIQUARY_TOOLS` (or similar) declaring the tool list. Most explicit; requires regime schema work and per-vessel docket updates.
2. **Full-subtree presence check**: precheck enumerates every image under `<prefix>reliquaries/<stamp>/` and verifies all are present. Catches piecemeal jettison on the whole stamp; doesn't distinguish tools the vessel actually needs from tools it doesn't. Conservative — flags integrity damage even on tools the vessel ignores.
3. **Cloudbuild-substitution parse**: derive tool list from the rendered cloudbuild.json's `_RBG?_*` tool-image substitutions before submission. Brittle (parser tracks producer-side grammar) but precise.

Recommend option 2 for v1 simplicity (any tool missing means the stamp's integrity is questionable; conservative failure is acceptable). Reconsider if false-positive rate is high in practice.

### Operation

Before Cloud Build submission in conjure and bind ordain modes:

1. Resolve the vessel's `RBRV_IMAGE_ORIGIN` against `<prefix>enshrines/<base>` — confirm the enshrined base image exists at the expected GAR path.
2. If the vessel is reliquary-yoked (`RBRV_RELIQUARY` set), apply the chosen tool-list resolution (above) and confirm each expected image is present.
3. On any miss, exit non-zero before any Cloud Build call. Error message format (nuclear advice only):

```
PRECHECK: GAR image not found at <prefix>enshrines/<base>
  Required by <vessel>'s RBRV_IMAGE_ORIGIN.
  Recover by re-enshrining: tt/rbw-dE.DirectorEnshrinesVessel.sh
```

```
PRECHECK: GAR image not found at <prefix>reliquaries/<stamp>/<tool>
  Required by <vessel>'s RBRV_RELIQUARY=<stamp>.
  Recover by re-inscribing: tt/rbw-dI.DirectorInscribesReliquary.sh
```

No surgical recovery hints. If a ref is broken, the operator re-creates the whole reliquary or re-enshrines the base — the cosmology accepts that piecemeal jettison is allowed but unrecoverable surgically.

### Implementation site

`rbfd_FoundryDirectorBuild.sh` (or its caller in `rbfd_cli.sh`). Add a function `zrbfd_precheck_refs <vessel-path> <mode>` invoked at the start of conjure and bind paths. Caller passes the mode; function does not redetect mode from regime. Function reads vessel's RBRV regime, performs HEAD requests against the resolved GAR paths, accumulates misses, prints the structured error message(s), exits non-zero on any miss.

Reuses existing GAR auth path (whatever conjure already uses for its REST calls — director-tier credentials).

### Mode coverage

- **Conjure** — precheck: enshrined base required; reliquary refs if yoked.
- **Bind** — precheck: enshrined base required; reliquary refs if yoked.
- **Graft** — skip: graft is local-image-push, no GCB-side ref consumption.
- **Enshrine** (`rbw-dE`) — skip: input is upstream image (Docker Hub / similar), not GAR.
- **Inscribe** (`rbw-dI`) — skip: input is upstream tool images, not GAR.
- **Yoke** (`rbw-dY`) — out of scope for this pace. Yoke-time stamp existence check is a sensible follow-up; surface as paddock note if the gap matters in practice.

### Cost / latency note

Per-call HEAD requests scale with reliquary size. For a vessel yoked to a stamp with 20 tool images, precheck adds ~21 HEAD requests (1 enshrine + 20 reliquary) before any Cloud Build call. Expected sub-second total against GAR. Acceptable overhead given the failure-mode-cost asymmetry (failed Cloud Build wastes minutes-to-hours of build time).

### Verification

- Negative: jettison the enshrined base of a test vessel, attempt conjure → precheck fails with the structured message; no Cloud Build submission occurs (verify by absence of build in Cloud Build console).
- Negative: jettison one tool image from a reliquary, attempt bind on a vessel yoked to that reliquary → precheck fails with structured message naming the missing tool.
- Positive: clean conjure → precheck passes silently, build submits as before (no observable change in normal-path latency beyond the small HEAD-request overhead).
- Verify graft path is unaffected (no precheck call in graft codepath; no overhead).

### Out of scope

- User-invocable integrity command (deliberately not minted; precheck output covers the diagnostic use case)
- Surgical recovery hints (nuclear advice only)
- Caching the precheck result across multiple ordain calls (per-call freshness — refs can drift between calls)
- Precheck for graft / enshrine / inscribe modes (as enumerated above)
- Yoke-time stamp existence check (follow-up consideration, not blocking)

### Affiliated context

Builds on Pace 1's tabtarget vocabulary (the precheck error message references `rbw-dE` and `rbw-dI` recovery commands; those names are stable, predate this sequence). The precheck logic itself uses GAR HEAD requests — independent of the rekon/muster/jettison machinery extended in Pace 1, so this pace can land independent of Pace 1's status if priority shifts.

**[260425-0920] rough**

## Character

Single concentrated edit at the ordain submission boundary. Small diff, real safety upside — catches missing-ref class of bugs before consuming Cloud Build minutes. No new tabtarget, no new verb. The integrity check exists ONLY as an ordain pre-step; the standalone diagnostic use case is covered by the structured error message itself.

## Docket

Add a precheck inside the ordain pipeline that resolves vessel ref dependencies against GAR before submitting Cloud Build. Fail loudly with nuclear-rebuild advice on any missing ref.

### Operation

Before Cloud Build submission in conjure and bind ordain modes (graft skipped — consumes neither reliquary nor enshrine refs):

1. Resolve the vessel's `RBRV_IMAGE_ORIGIN` against `<prefix>enshrines/<base>` — confirm the enshrined base image exists at the expected GAR path.
2. If the vessel is reliquary-yoked (`RBRV_RELIQUARY` set), resolve every tool image referenced in the reliquary's manifest against `<prefix>reliquaries/<stamp>/<tool>` — confirm each is present.
3. On any miss, exit non-zero before any Cloud Build call. Error message format (nuclear advice only):

```
PRECHECK: GAR image not found at <prefix>enshrines/<base>
  Required by <vessel>'s RBRV_IMAGE_ORIGIN.
  Recover by re-enshrining: tt/rbw-dE.DirectorEnshrinesVessel.sh
```

```
PRECHECK: GAR image not found at <prefix>reliquaries/<stamp>/<tool>
  Required by <vessel>'s RBRV_RELIQUARY=<stamp>.
  Recover by re-inscribing: tt/rbw-dI.DirectorInscribesReliquary.sh
```

No surgical recovery hints. If a ref is broken, the operator re-creates the whole reliquary or re-enshrines the base — the cosmology accepts that piecemeal jettison is allowed but unrecoverable surgically.

### Implementation site

`rbfd_FoundryDirectorBuild.sh` (or its caller in `rbfd_cli.sh`). Add a function `zrbfd_precheck_refs <vessel-path>` invoked at the start of conjure and bind paths. Function reads vessel's RBRV regime, performs HEAD requests against the resolved GAR paths, accumulates misses, prints the structured error message(s), exits non-zero on any miss.

Reuses existing GAR auth path (whatever conjure already uses for its REST calls — director-tier credentials).

### Mode coverage

- **Conjure** — always: enshrined base required.
- **Bind** — always: enshrined base required + reliquary refs if yoked.
- **Graft** — skip: graft is local-image-push, no GCB-side ref consumption.

### Verification

- Negative: jettison the enshrined base of a test vessel, attempt conjure → precheck fails with the structured message; no Cloud Build submission occurs (verify by absence of build in cloud build console / billing).
- Negative: jettison one tool image from a reliquary, attempt bind on a vessel yoked to that reliquary → precheck fails with structured message naming the missing tool.
- Positive: clean conjure → precheck passes silently, build submits as before (no observable change in normal-path latency beyond the small HEAD-request overhead).
- Verify graft path is unaffected (no precheck call in graft codepath; no overhead).

### Out of scope

- User-invocable integrity command (deliberately not minted; precheck output covers the diagnostic use case)
- Surgical recovery hints (nuclear advice only)
- Caching the precheck result across multiple ordain calls (per-call freshness — refs can drift between calls)
- Precheck for graft mode (graft has no GCB-side dependencies)

### Affiliated context

Builds on Pace 1's tabtarget vocabulary (the precheck error message references `rbw-dE` and `rbw-dI` recovery commands; those names are stable, predate this sequence). The precheck logic itself uses GAR HEAD requests — independent of the rekon/muster/jettison machinery extended in Pace 1, so this pace can land independent of Pace 1's status if priority shifts.

### jettison-multi-platform-cascade-fix (₢BAAAG) [complete]

**[260513-1217] complete**

## Character

Display-side correctness fix replacing the original V2-cascade design.
User concern reframed during the jettison-clarification session: "walking
dead hallmarks in tally/rekon" is the real problem, not GAR storage
reclamation latency. GAR autonomously reaps orphan children of deleted
parent indexes on its cleanup-policy schedule (~24h once configured);
this pace makes the user-facing display accurate immediately by filtering
tagless packages and underwrites GAR's reclamation by configuring the
cleanup policy at depot levy.

V2-cascade (original docket) and package-delete-via-abjure-path
(alternative considered) both explicitly dropped — both correct but
overcomplex once the acceptance criterion is "display honest" not
"delete transactional."

## Docket

### Three-part change

1. **Cleanup policy at depot levy** — extend depot creation to set a GAR
   cleanup policy targeting untagged manifests. Operator-chosen retention
   window at slate-time; suggestion 1-3 days.
2. **Display filter in package enumeration** — modify the package
   enumerator to skip packages with zero live tags. In our push patterns
   every live package has ≥1 tag at push time (URI shape uniformly
   `<discriminator>/<basename>:<discriminator>`), so tagless =
   walking-dead, unambiguously.
3. **`rbfl_jettison` untouched** — V2 DELETE-by-tag stays as is; orphan
   children become GAR's responsibility via the policy from #1.

### Surface

- `rbgp_Payor.sh` (`rbgp_depot_levy`) — add cleanup-policy clause to
  depot creation.
- `rbfc_FoundryCore.sh` (`zrbfc_list_packages_capture`) — per-package
  tag-list check, filter tagless from output.
- `RBSIJ-image_jettison.adoc` — replace cascade plan with NOTE describing
  the display-filter + cleanup-policy contract.
- `RBSDE-depot_levy.adoc` — document the new cleanup-policy clause.

No tabtarget changes. No new helpers in rbfc. No new prefixes.

### Out of scope

- V2 cascade in `rbfl_jettison` (original design, dropped).
- Package-delete via abjure path (alternative, dropped).
- Workstation-vs-Cloud-Build relocation — now moot (no new cascade
  machinery to relocate). Revisit separately if audit/symmetry argument
  earns its own pace.
- Retroactive cleanup-policy application to existing depots — separate
  follow-up pace if operator wants.

### Verification

- `batch_vouch` fixture: assertion target shifts from "package absent
  from GAR ListPackages" to "tally output excludes hallmark" —
  functionally equivalent under the new contract.
- `bind_lifecycle`: unchanged, single-platform path untouched.
- Fast suite GREEN; bash -n / shellcheck clean.
- Manual: ordain → jettison vouch → `rbw-ft` shows pending (not
  vouched). GAR REST still lists package until cleanup policy runs;
  tally does not.
- `gcloud artifacts repositories describe <depot-repo>` shows the
  configured policy on a fresh depot.

### Hard dependency notes

- ₢BAAAF (imageops-fixtures) — `muster_absent` cases need the same
  assertion-target shift (GAR-level absence → display-level absence).
- Test-suite reservation: ₣BA paddock blocks crucible/service during
  ₣A_'s live-infra runs.

### References

- jettison-clarification session (search jjx_log on ₣BA / ₢BAAAG for
  notch on this redocket): full derivation of why option 3 over
  V2-cascade and abjure-delegation alternatives, verified GAR API
  contract findings (no on-demand cleanup trigger; package URI shape
  uniformity in our codebase; one-tag-per-package invariant for
  multi-platform content).
- GAR cleanup policy:
  https://docs.cloud.google.com/artifact-registry/docs/repositories/cleanup-policy
- `gcloud artifacts repositories set-cleanup-policies` reference.
- `Tools/rbk/rbfc_FoundryCore.sh` — `zrbfc_list_packages_capture` is
  the enumerator to extend.
- `Tools/rbk/rbgp_Payor.sh` — `rbgp_depot_levy` is the depot-creation
  site.
- `Tools/rbk/rbfl_FoundryLedger.sh` — `rbfl_jettison` stays as is.
- `Tools/rbk/vov_veiled/RBSIJ-image_jettison.adoc` — spec to amend.
- `Tools/rbk/vov_veiled/RBSDE-depot_levy.adoc` — spec to amend.

**[260513-1142] rough**

## Character

Display-side correctness fix replacing the original V2-cascade design.
User concern reframed during the jettison-clarification session: "walking
dead hallmarks in tally/rekon" is the real problem, not GAR storage
reclamation latency. GAR autonomously reaps orphan children of deleted
parent indexes on its cleanup-policy schedule (~24h once configured);
this pace makes the user-facing display accurate immediately by filtering
tagless packages and underwrites GAR's reclamation by configuring the
cleanup policy at depot levy.

V2-cascade (original docket) and package-delete-via-abjure-path
(alternative considered) both explicitly dropped — both correct but
overcomplex once the acceptance criterion is "display honest" not
"delete transactional."

## Docket

### Three-part change

1. **Cleanup policy at depot levy** — extend depot creation to set a GAR
   cleanup policy targeting untagged manifests. Operator-chosen retention
   window at slate-time; suggestion 1-3 days.
2. **Display filter in package enumeration** — modify the package
   enumerator to skip packages with zero live tags. In our push patterns
   every live package has ≥1 tag at push time (URI shape uniformly
   `<discriminator>/<basename>:<discriminator>`), so tagless =
   walking-dead, unambiguously.
3. **`rbfl_jettison` untouched** — V2 DELETE-by-tag stays as is; orphan
   children become GAR's responsibility via the policy from #1.

### Surface

- `rbgp_Payor.sh` (`rbgp_depot_levy`) — add cleanup-policy clause to
  depot creation.
- `rbfc_FoundryCore.sh` (`zrbfc_list_packages_capture`) — per-package
  tag-list check, filter tagless from output.
- `RBSIJ-image_jettison.adoc` — replace cascade plan with NOTE describing
  the display-filter + cleanup-policy contract.
- `RBSDE-depot_levy.adoc` — document the new cleanup-policy clause.

No tabtarget changes. No new helpers in rbfc. No new prefixes.

### Out of scope

- V2 cascade in `rbfl_jettison` (original design, dropped).
- Package-delete via abjure path (alternative, dropped).
- Workstation-vs-Cloud-Build relocation — now moot (no new cascade
  machinery to relocate). Revisit separately if audit/symmetry argument
  earns its own pace.
- Retroactive cleanup-policy application to existing depots — separate
  follow-up pace if operator wants.

### Verification

- `batch_vouch` fixture: assertion target shifts from "package absent
  from GAR ListPackages" to "tally output excludes hallmark" —
  functionally equivalent under the new contract.
- `bind_lifecycle`: unchanged, single-platform path untouched.
- Fast suite GREEN; bash -n / shellcheck clean.
- Manual: ordain → jettison vouch → `rbw-ft` shows pending (not
  vouched). GAR REST still lists package until cleanup policy runs;
  tally does not.
- `gcloud artifacts repositories describe <depot-repo>` shows the
  configured policy on a fresh depot.

### Hard dependency notes

- ₢BAAAF (imageops-fixtures) — `muster_absent` cases need the same
  assertion-target shift (GAR-level absence → display-level absence).
- Test-suite reservation: ₣BA paddock blocks crucible/service during
  ₣A_'s live-infra runs.

### References

- jettison-clarification session (search jjx_log on ₣BA / ₢BAAAG for
  notch on this redocket): full derivation of why option 3 over
  V2-cascade and abjure-delegation alternatives, verified GAR API
  contract findings (no on-demand cleanup trigger; package URI shape
  uniformity in our codebase; one-tag-per-package invariant for
  multi-platform content).
- GAR cleanup policy:
  https://docs.cloud.google.com/artifact-registry/docs/repositories/cleanup-policy
- `gcloud artifacts repositories set-cleanup-policies` reference.
- `Tools/rbk/rbfc_FoundryCore.sh` — `zrbfc_list_packages_capture` is
  the enumerator to extend.
- `Tools/rbk/rbgp_Payor.sh` — `rbgp_depot_levy` is the depot-creation
  site.
- `Tools/rbk/rbfl_FoundryLedger.sh` — `rbfl_jettison` stays as is.
- `Tools/rbk/vov_veiled/RBSIJ-image_jettison.adoc` — spec to amend.
- `Tools/rbk/vov_veiled/RBSDE-depot_levy.adoc` — spec to amend.

**[260426-1029] rough**

## Character

Targeted machinery fix on a defect surfaced empirically by ₣A_'s AAS pace.
Small-diff, high-confidence change once the cascade traversal is written —
the OCI Distribution Spec already nails down the index-to-children
relationship, and `zrbfc_gar_extract_artifact` already implements the
read-side of that traversal. This pace mirrors that traversal on the
delete side.

The risk surface is narrow: existing single-platform jettisons (pouch on
bind, anything else not pushed via buildx) must remain unchanged. The new
code path activates only when the manifest at the locator is an OCI image
index / Docker manifest list.

Verification is concrete: the batch_vouch fixture (already implemented under
AAS) is the regression test. It currently FAILS at step 03 because vouch
jettison leaves the package container in GAR's ListPackages; after this pace
lands, it must pass step 03 (tally pending) through step 06 (abjure) cleanly.

## Docket

### Read-only investigation completed (carryover from prior chat)

A previous mounting did cadence-step-2 (the read pass) and pre-validated the
docket against the actual code. The premise holds: all three reference points
(`zrbfc_gar_extract_artifact`'s mediaType branch, `rbfl_jettison`'s
single-DELETE shape, `rbgjv03-assemble-push-vouch.sh`'s buildx push) match
the docket's description structurally. No design pivots were surfaced.

Three small decisions were ratified by the working chat:

1. **Helper extraction is in.** Add `zrbfc_jettison_index_children` to
   `rbfc_FoundryCore.sh` next to `zrbfc_gar_extract_artifact`. Args:
   `(token, package, tag)`. Side effects: per-digest DELETEs only — the
   index-by-tag DELETE stays in `rbfl_jettison` so the existing single-DELETE
   path is the unconditional finalizer. Mirrors the read-side helper's
   curl+jq pattern.

2. **Retrofit existing DELETE's stderr capture in the same notch.** The
   current `rbfl_jettison` curl doesn't redirect stderr; BCG's "Stderr
   Capture — Never Suppress" applies. Single-line addition (`2>"${z_stderr_file}"`
   plus a die-message reference). No behavior change.

3. **RBSIJ amendment rides the same notch.** The spec is small and ships
   with the behavior. Two new operation steps between current "Confirm
   Jettison" and "Jettison Tag":
   - **Probe Manifest Type** — GET manifest, branch 404 (early-exit
     idempotent success) vs 200 (continue), extract mediaType.
   - **Cascade Children if Index** — when mediaType matches
     `*manifest.list*` or `*image.index*`, enumerate `.manifests[].digest`
     and DELETE each child by digest before the existing tag DELETE.
   Plus a NOTE explaining the producer-side asymmetry (single-platform
   `docker push` vs multi-platform `docker buildx --push`).

### Empirical motivation (recorded for the next mounting)

`rbfl_jettison` calls Docker Registry V2 `DELETE /v2/<pkg>/manifests/<tag>`.
For single-platform pushes (e.g. pouch, which is `docker push` from
`rbfd_FoundryDirectorBuild.sh`), this removes the only manifest, the package
becomes empty, and GAR drops it from `ListPackages`. For multi-platform
buildx pushes (vouch in `rbgjv03-assemble-push-vouch.sh`, all reliquary tool
images, all enshrinements), the tag points to an OCI image index referencing
N per-platform manifests; DELETE-by-tag removes only the index, leaving the
N children as untagged content. The package container persists in
`ListPackages` because it still has manifests.

`zrbfc_list_hallmarks_capture` and consequently `rbfl_tally` and `rbfl_rekon`
enumerate via `ListPackages`, so they continue to report the basename present
after a jettison-by-tag against multi-platform content. AAS's batch_vouch
fixture exposed this as a tally-still-vouched failure post-jettison; the
morning's stale-hallmark abjure (which uses the GAR REST package-delete path,
not the V2 manifest path) cleanly removed all six packages including vouch
— confirming the package container persistence is real and is exactly what
abjure already overcomes.

The asymmetry was bind_lifecycle's pouch jettison passing while batch_vouch's
vouch jettison failed; same locator grammar, different push mechanism on the
producer side.

### Approach: cascade through the index

In `rbfl_jettison`, after parsing the locator and authenticating:

1. **Probe** — GET the manifest at `<pkg>/manifests/<tag>` with the
   manifest Accept header (use `ZRBFC_ACCEPT_MANIFEST_MTYPES`). Capture
   status, response, stderr to per-call temp files under
   `${ZRBFL_DELETE_PREFIX}probe_*`.
2. **Branch on probe status:**
   - `404` → preserve existing idempotent semantics: emit
     `"Jettisoned or nonexistent: <locator>"` and return success.
   - `200` → fall through.
   - other → die with status code.
3. **Read mediaType** with `jq -r '.mediaType // empty' ... 2>/dev/null || true`
   (matching `zrbfc_gar_extract_artifact`'s pattern). Empty mediaType is
   safe — falls through to single-DELETE behavior.
4. **Conditional cascade** — `case "${z_media_type}" in
   *manifest.list*|*image.index*) zrbfc_jettison_index_children ... ;; esac`.
5. **DELETE-by-tag** (existing code, unchanged behavior) — remains as the
   unconditional final step. Accepts 202|204.

The cascade helper itself: GET the index manifest (manifest Accept header),
extract `.manifests[].digest` to a temp file, load-then-iterate (BCG
stdin-consumption discipline), per-child DELETE with 202|204 acceptance.

### Surface

- **`Tools/rbk/rbfl_FoundryLedger.sh`** — extend `rbfl_jettison` with
  probe + branch + cascade-call before the existing DELETE. Retrofit
  stderr capture on the existing DELETE.
- **`Tools/rbk/rbfc_FoundryCore.sh`** — new helper
  `zrbfc_jettison_index_children` near `zrbfc_gar_extract_artifact`.
  Reuses existing constants: `ZRBFC_REGISTRY_API_BASE`,
  `ZRBFC_ACCEPT_MANIFEST_MTYPES`, `RBCC_CURL_*`.
- **`Tools/rbk/vov_veiled/RBSIJ-image_jettison.adoc`** — two new operation
  steps + a NOTE about the producer-side asymmetry.

No tabtarget surface change. No new prefixes. No new modules. No new
constants in regimes.

### BCG patterns to preserve

Working chat noted these explicitly so they survive the next mounting:

- All new curl calls capture stderr to per-call temp files and reference
  them in `buc_die` messages.
- `local -r` for one-shot locals; plain `local` only for per-iteration loop
  variables (BCG Exception 2).
- Load-then-iterate for the child-digest loop (read digests into array,
  iterate over `${!arr[@]}` with index discriminator) — protects against
  stdin consumption hazards.
- `case "${z_media_type}" in *manifest.list*|*image.index*) ... ;; esac`
  for pattern matching, not `[[ ]]`.
- HTTP code branches via `case`, accepting 202|204 for DELETEs and 200
  (with 404 idempotent) for the probe GET.

Important `rbgu_http_json` caveat: it accepts only 200/201/204 in
`rbgu_http_require_ok`. The V2 manifest DELETE path returns 202, so the
existing single-DELETE pattern (raw curl with explicit `-w "%{http_code}"`
plus a case-statement check) is the right model — do not refactor those
calls onto `rbgu_http_json`.

### Verification

1. **Regression — batch_vouch fixture must pass end-to-end:**
   `tt/rbtd-s.SingleCase.batch-vouch.sh rbtdrc_batch_vouch_lifecycle`
   (currently fails at step 03 tally pending; after this pace must pass
   through step 07 passed).
2. **Bind/pouch unaffected:** `tt/rbtd-s.SingleCase.four-mode.sh
   rbtdrc_fourmode_bind_lifecycle` — single-platform jettison path must
   still pass.
3. **All four single-mode SingleCases GREEN** — sanity sweep:
   conjure_lifecycle, bind_lifecycle, graft_lifecycle, kludge_lifecycle.
4. **Fast suite GREEN** — no regression on non-crucible tests
   (`tt/rbtd-s.TestSuite.fast.sh`).
5. **Manual one-shot:** ordain a fresh hallmark, jettison vouch via
   `rbw-iJh.DirectorJettisonsHallmarkImage.sh`, run `rbw-ft` —
   the hallmark should reflect "pending" (image+about, no vouch) rather
   than "vouched" with all six basenames.
6. **Cost discipline:** the regression test is one ordain (≈3-5min Cloud
   Build) per fixture run. Don't loop unnecessarily.

### Hard dependency notes

- `₢BAAAF` (imageops-fixtures) **depends on this pace landing**. BAAAF's
  `muster_absent` cases assert concrete absence, which requires the cascade
  fix to make packages truly empty.
- This pace is independent of `₢BAAAA` (oauth hygiene), `₢BAAAC` (OCI URL
  keyword extraction), and `₢BAAAB` (BCG shellcheck sweep) — disjoint
  surfaces.
- **Test-suite reservation:** ₣BA's paddock blocks crucible/service suites
  while ₣A_ is racing on live infra. The notch criteria (bind_lifecycle +
  batch_vouch GREEN) require coordination — either wait for ₣A_'s wrap, or
  the user authorizes a partial-verification notch (bash -n + shellcheck +
  fast suite) with full crucible verification deferred. Mounting agent
  should surface this before running fixtures.

### Out of scope

- **Changing `rbfl_abjure`.** Abjure already uses GAR REST package-delete
  and works correctly for multi-platform content; don't refactor it.
- **Changing `zrbfc_list_hallmarks_capture` semantics.** Package-level
  enumeration is correct given the post-cascade behavior. Don't switch to
  tag-level enumeration.
- **Adding new tabtargets.** The existing `rbw-iJh` /
  `DirectorJettisonsHallmarkImage` (and its sibling `rbw-iJr` / `rbw-iJe`
  introduced by BAAAD) suffice; the cascade is a behavior change inside
  `rbfl_jettison`, not a new operation.
- **Changing the producer side** (rbgjv03's buildx push, rbgjb*'s buildx
  pushes, enshrine's skopeo copy). Multi-platform pushes are a feature, not
  a defect; the consumer side (jettison) just needs to handle them.
- **Cascading abjure semantics into jettison** (i.e., letting jettison
  delete a whole package, not a tag). The spec for jettison is "by locator"
  not "by package"; broadening would conflate operations.
- **Backward-compat shims** for the silent-no-op behavior. Hard cutover:
  after this pace, jettison-by-tag against a multi-platform locator does
  the right thing; nothing should depend on the broken behavior.
- **Refactoring the existing single DELETE onto `rbgu_http_json`.** That
  helper rejects 202; the V2 manifest DELETE path returns 202. Keep the
  raw curl + explicit code check.

### Cadence

1. Mount BA, parade this pace, read this docket.
2. Read in order before any edit:
   - `rbfl_FoundryLedger.sh` — `rbfl_jettison` and the existing DELETE
     pattern
   - `rbfc_FoundryCore.sh` — `zrbfc_gar_extract_artifact` for the mediaType
     branch shape and the curl+jq pattern
   - `rbgu_Utility.sh` — `rbgu_http_json` and acceptance helpers (note
     200/201/204-only acceptance — see caveat above)
   - `rbgjv03-assemble-push-vouch.sh` — confirm buildx push shape
3. Implement the cascade. Single notch when:
   - bash -n / shellcheck clean on touched files
   - bind_lifecycle GREEN (no regression)
   - batch_vouch GREEN (the regression test passes)
4. Wrap.

### References

- ₣A_'s AAS docket and wrap summary — full empirical investigation and
  decision history.
- `Tools/rbk/rbfc_FoundryCore.sh` — `zrbfc_gar_extract_artifact` is the
  read-side mediaType branch, the structural template for the delete-side
  cascade. Insert `zrbfc_jettison_index_children` adjacent.
- `Tools/rbk/rbfl_FoundryLedger.sh` — `rbfl_jettison` is the function to
  extend.
- `Tools/rbk/rbgu_Utility.sh` — `rbgu_http_json` /
  `rbgu_http_require_ok` reference for HTTP helper acceptance behavior.
- `Tools/rbk/rbgjv/rbgjv03-assemble-push-vouch.sh` — confirms vouch is a
  multi-platform buildx push (canonical example).
- `Tools/rbk/vov_veiled/RBSIJ-image_jettison.adoc` — spec to amend.
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — BCG patterns: stderr
  capture, load-then-iterate, local -r discipline.

**[260426-0907] rough**

## Character

Targeted machinery fix on a defect surfaced empirically by ₣A_'s AAS pace.
Small-diff, high-confidence change once the cascade traversal is written —
the OCI Distribution Spec already nails down the index-to-children
relationship, and `zrbfc_gar_extract_artifact` (rbfc_FoundryCore.sh:575)
already implements the read-side of that traversal. This pace mirrors that
traversal on the delete side.

The risk surface is narrow: existing single-platform jettisons (pouch on
bind, anything else not pushed via buildx) must remain unchanged. The new
code path activates only when the manifest at the locator is an OCI image
index / Docker manifest list.

Verification is concrete: the batch_vouch fixture (already implemented under
AAS) is the regression test. It currently FAILS at step 03 because vouch
jettison leaves the package container in GAR's ListPackages; after this pace
lands, it must pass step 03 (tally pending) through step 06 (abjure) cleanly.

## Docket

### Empirical motivation (recorded for the next mounting)

`rbfl_jettison` (rbfl_FoundryLedger.sh:328+) calls Docker Registry V2
`DELETE /v2/<pkg>/manifests/<tag>`. For single-platform pushes (e.g. pouch,
which is `docker push` from rbfd_FoundryDirectorBuild.sh:734), this removes
the only manifest, the package becomes empty, and GAR drops it from
`ListPackages`. For multi-platform buildx pushes (vouch in
rbgjv03-assemble-push-vouch.sh:48-52, all reliquary tool images, all
enshrinements), the tag points to an OCI image index referencing N
per-platform manifests; DELETE-by-tag removes only the index, leaving the N
children as untagged content. The package container persists in
`ListPackages` because it still has manifests.

`zrbfc_list_hallmarks_capture` (rbfc_FoundryCore.sh:531) and consequently
`rbfl_tally` and `rbfl_rekon` enumerate via `ListPackages`, so they continue
to report the basename present after a jettison-by-tag against multi-platform
content. AAS's batch_vouch fixture exposed this as a tally-still-vouched
failure post-jettison; the morning's stale-hallmark abjure (which uses the
GAR REST package-delete path, not the V2 manifest path) cleanly removed all
six packages including vouch — confirming the package container persistence
is real and is exactly what abjure already overcomes.

The asymmetry was bind_lifecycle's pouch jettison passing while batch_vouch's
vouch jettison failed; same locator grammar, different push mechanism on the
producer side.

### Approach: cascade through the index

In `rbfl_jettison`, after parsing the locator and authenticating, fetch the
manifest at `<pkg>/manifests/<tag>`. Branch on mediaType:

1. **Single manifest** (`application/vnd.docker.distribution.manifest.v2+json`,
   `application/vnd.oci.image.manifest.v1+json`): existing behavior —
   DELETE-by-tag, done.
2. **Image index / manifest list** (`application/vnd.oci.image.index.v1+json`,
   `application/vnd.docker.distribution.manifest.list.v2+json`): enumerate
   `.manifests[].digest`. For each child digest, DELETE
   `<pkg>/manifests/<digest>`. After all children are removed, DELETE
   `<pkg>/manifests/<tag>` (the index itself) to clear the tag reference.

Mirror `zrbfc_gar_extract_artifact`'s mediaType branch (rbfc_FoundryCore.sh:622)
for the recognition pattern; that function already handles the same
branching shape on the read side.

### Surface

- **`Tools/rbk/rbfl_FoundryLedger.sh`** — extend `rbfl_jettison` to do the
  cascade. Most lines added are inside the existing function; helpers
  factored into `rbfc_FoundryCore.sh` if they generalize to other future
  callers (likely just one helper: enumerate-and-delete-children-by-digest).
- **`Tools/rbk/rbfc_FoundryCore.sh`** — possibly add a small helper
  `zrbfc_jettison_index_children` if extraction is clean. If the cascade
  fits cleanly inline, skip the extraction.

No tabtarget surface change. No spec change.

### Spec touch

- **`Tools/rbk/vov_veiled/RBSIJ-image_jettison.adoc`** — current spec
  describes tag-by-tag DELETE without acknowledging the multi-platform case.
  Add a note (post-cascade): "Multi-platform manifests (OCI image indices /
  manifest lists) are jettisoned by enumerating per-platform child manifests
  via the index, deleting each by digest, then deleting the index by tag.
  This produces a fully-empty package, which GAR then drops from
  `ListPackages`."

### Verification

1. **Regression — batch_vouch fixture must pass end-to-end:**
   `tt/rbtd-s.SingleCase.batch-vouch.sh rbtdrc_batch_vouch_lifecycle`
   (currently fails at step 03 tally pending; after this pace must pass
   through step 07 passed).
2. **Bind/pouch unaffected:** `tt/rbtd-s.SingleCase.four-mode.sh
   rbtdrc_fourmode_bind_lifecycle` — single-platform jettison path must
   still pass.
3. **All four single-mode SingleCases GREEN** — sanity sweep:
   conjure_lifecycle, bind_lifecycle, graft_lifecycle, kludge_lifecycle.
4. **Fast suite GREEN** — no regression on non-crucible tests
   (`tt/rbtd-s.TestSuite.fast.sh`).
5. **Manual one-shot:** ordain a fresh hallmark, jettison vouch via
   `rbw-iJh.DirectorJettisonsHallmarkImage.sh`, run `rbw-ft` —
   the hallmark should reflect "pending" (image+about, no vouch) rather than
   "vouched" with all six basenames.
6. **Cost discipline:** the regression test is one ordain (≈3-5min Cloud
   Build) per fixture run. Don't loop unnecessarily.

### Hard dependency notes

- `₢BAAAF` (imageops-fixtures) **depends on this pace landing**. BAAAF's
  `muster_absent` cases assert concrete absence, which requires the cascade
  fix to make packages truly empty.
- This pace is independent of `₢BAAAA` (oauth hygiene), `₢BAAAC` (OCI URL
  keyword extraction), and `₢BAAAB` (BCG shellcheck sweep) — disjoint
  surfaces.

### Out of scope

- **Changing `rbfl_abjure`.** Abjure already uses GAR REST package-delete
  and works correctly for multi-platform content; don't refactor it.
- **Changing `zrbfc_list_hallmarks_capture` semantics.** Package-level
  enumeration is correct given the post-cascade behavior. Don't switch to
  tag-level enumeration.
- **Adding new tabtargets.** The existing `rbw-iJh` /
  `DirectorJettisonsHallmarkImage` (and its sibling `rbw-iJr` / `rbw-iJe`
  introduced by BAAAD) suffice; the cascade is a behavior change inside
  `rbfl_jettison`, not a new operation.
- **Changing the producer side** (rbgjv03's buildx push, rbgjb*'s buildx
  pushes, enshrine's skopeo copy). Multi-platform pushes are a feature, not
  a defect; the consumer side (jettison) just needs to handle them.
- **Cascading abjure semantics into jettison** (i.e., letting jettison
  delete a whole package, not a tag). The spec for jettison is "by locator"
  not "by package"; broadening would conflate operations.
- **Backward-compat shims** for the silent-no-op behavior. Hard cutover:
  after this pace, jettison-by-tag against a multi-platform locator does
  the right thing; nothing should depend on the broken behavior.

### Cadence

1. Mount BA, parade this pace, read this docket.
2. Read `rbfl_FoundryLedger.sh` (jettison function), `rbfc_FoundryCore.sh`
   (zrbfc_gar_extract_artifact for the mediaType branch shape), and
   `rbgjv03-assemble-push-vouch.sh` (confirm buildx push shape) before
   any edit.
3. Implement the cascade. Single notch when:
   - bash -n / shellcheck clean on touched files
   - bind_lifecycle GREEN (no regression)
   - batch_vouch GREEN (the regression test passes)
4. RBSIJ note update can ride the same notch or a follow-up — choice at
   the working chat's discretion.
5. Wrap.

### References

- ₣A_'s AAS docket and wrap summary (commit `3a9199ba` + chain) — full
  empirical investigation and decision history.
- `Tools/rbk/rbfc_FoundryCore.sh:575` (`zrbfc_gar_extract_artifact`) —
  read-side mediaType branch, the structural template for the delete-side
  cascade.
- `Tools/rbk/rbfl_FoundryLedger.sh:328` (`rbfl_jettison`) — the function
  to extend.
- `Tools/rbk/rbgjv/rbgjv03-assemble-push-vouch.sh:48-52` — confirms vouch
  is a multi-platform buildx push (canonical example).
- `Tools/rbk/vov_veiled/RBSIJ-image_jettison.adoc` — spec to amend.

### oauth-trust-hygiene-sweep (₢BAAAA) [complete]

**[260513-0800] complete**

Drafted from ₢A_AAN in ₣A_.

## Character

Mechanical apply. All design decisions are baked into this docket; the agent's job is to execute the listed edits, run the listed tests, and notch with the listed file list. No options to evaluate. No "decide between..." sections. If anything looks ambiguous, the docket is the source of truth — re-read it rather than improvise.

This docket was built and reviewed in a prior chat; its line numbers and diffs were verified against the working tree at slate time. Spot-check before applying — line numbers may drift if other officia touch the target files.

## Pre-flight (re-verify before edits)

Run these to establish a known baseline:

```
git status --short                           # expect clean of AAN target files (see file list at end)
tt/rbtd-s.TestSuite.fast.sh                  # expect 93/93 green (47+23+8+15)
shellcheck Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbgp_Payor.sh Tools/buk/buh_handbook.sh \
           Tools/rbk/rbgu_Utility.sh Tools/rbk/rbgg_Governor.sh \
           Tools/rbk/rbgc_Constants.sh Tools/rbk/rbrp_regime.sh 2>&1 | grep -c '^In '
# expect ~119 warnings baseline (post-AAN target: ~110, drops 9: 1 SC2034 from F1, 7 SC2034 from F6a, 1-2 SC2153 from F6b directives)
```

If baseline differs significantly, investigate before editing. Another officium may have touched these files.

## Cross-officium constraints (HARD)

A parallel officium may be working on ₢A_AAS (theurge Rust + tabtargets). **Never** touch:

- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs`
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs`
- `Tools/rbk/rbtd/src/rbtdrf_fast.rs`
- `Tools/rbk/rbtd/tests/rbtdtm_manifest.rs`
- `tt/rbtd-r.Run.batch-vouch.sh`
- `tt/rbtd-s.SingleCase.batch-vouch.sh`
- `Tools/rbk/rbtd/scripts/rbte_engine.sh`

Never run `tt/rbtd-s.TestSuite.crucible.sh` or `tt/rbtd-s.TestSuite.complete.sh`. Fast suite is read-only and safe.

If `git status --short` shows uncommitted changes to AAS-affiliated files at any point, leave them alone — that's the other officium's in-flight work.

## Apply order (one notch at the end, not per-finding)

### Step 1 — Pure deletions (F1, F6a) and shellcheck directives (F6b)

**F1** — Delete dead `ZRBGO_PRIVATE_KEY_FILE`. File: `Tools/rbk/rbgo_OAuth.sh`, line 50:

```bash
  readonly ZRBGO_PRIVATE_KEY_FILE="${BURD_TEMP_DIR}/rbgo_private_key.pem"
```

Delete the entire line.

**F6a** — Delete 7 dead `ZRBGP_INFIX_*` constants in `Tools/rbk/rbgp_Payor.sh`. Verified zero consumers tree-wide. Lines 46, 47, 48, 53, 54, 61, 63:

```bash
  readonly ZRBGP_INFIX_PROJECT_DELETE="project_delete"
  readonly ZRBGP_INFIX_PROJECT_RESTORE="project_restore"
  readonly ZRBGP_INFIX_PROJECT_STATE="project_state"
  readonly ZRBGP_INFIX_CREATE_REPO="create_repo"
  readonly ZRBGP_INFIX_VERIFY_REPO="verify_repo"
  readonly ZRBGP_INFIX_GOV_VERIFY_SA="gov_verify_sa"
  readonly ZRBGP_INFIX_GOV_IAM="gov_iam"
```

Delete those exact lines; leave the other 11 INFIX constants intact.

**F6b** — Add file-scoped shellcheck directive to silence SC2153 false positive on cross-module `ZRBGU_PREFIX`. Apply to BOTH files (the extension to `rbgg_Governor.sh` is a confirmed freebie since both have the same false-positive shape and the file is touched by F13 anyway).

In `Tools/rbk/rbgp_Payor.sh`, insert after line 25 (after `ZRBGP_SOURCED=1`):

```bash
# shellcheck disable=SC2153
# ZRBGU_PREFIX and ZRBGU_POSTFIX_* are defined in rbgu_Utility.sh and shared
# across modules via the kindle/sentinel chain (zrbgu_sentinel asserted in
# zrbgp_kindle). Shellcheck cannot follow the runtime sourcing graph.
```

In `Tools/rbk/rbgg_Governor.sh`, insert the same block after the file's own `ZRBGG_SOURCED=1` line (find via grep — the multiple-inclusion guard).

Note: the BCG-wide answer to this class of false positive is being studied under ₢A_AAY (`bcg-shellcheck-cross-module-discipline`). When AAY lands, these per-file directives become redundant and should be removed by AAY's apply pass.

### Step 2 — F3 OAuth URL constants

`RBGC_OAUTH_TOKEN_URL` already exists at `Tools/rbk/rbgc_Constants.sh:69`. Mint two more.

In `rbgc_Constants.sh`, after line 69 (`readonly RBGC_OAUTH_TOKEN_URL=...`), insert:

```bash
  readonly RBGC_OAUTH_AUTHORIZE_URL="https://accounts.google.com/o/oauth2/v2/auth"
  readonly RBGC_OAUTH_USERINFO_URL="https://www.googleapis.com/oauth2/v3/userinfo"
```

Then four substitutions in `Tools/rbk/rbgp_Payor.sh`:

- Line ~102: `"https://oauth2.googleapis.com/token"` → `"${RBGC_OAUTH_TOKEN_URL}"`
- Line ~443: replace ONLY the bare URL portion, NOT the query string. The full line is:
  ```
  local -r z_auth_url="https://accounts.google.com/o/oauth2/v2/auth?client_id=${z_client_id}&redirect_uri=urn:ietf:wg:oauth:2.0:oob&scope=openid%20email%20https://www.googleapis.com/auth/cloud-platform%20https://www.googleapis.com/auth/cloud-billing&response_type=code&access_type=offline"
  ```
  Replace `https://accounts.google.com/o/oauth2/v2/auth` (the prefix) with `${RBGC_OAUTH_AUTHORIZE_URL}` — keep the entire `?client_id=...` query string inline (it's parameterized, not extractable).
- Line ~485: `"https://oauth2.googleapis.com/token"` → `"${RBGC_OAUTH_TOKEN_URL}"`
- Line ~538: `"https://www.googleapis.com/oauth2/v3/userinfo"` → `"${RBGC_OAUTH_USERINFO_URL}"`

### Step 3 — Behavior-adjacent (F2, F5)

**F2** — Close TOCTOU on RBRO write. `Tools/rbk/rbgp_Payor.sh:504-509`. Current:

```bash
  buc_step 'Store OAuth credentials'
  {
    echo "RBRO_CLIENT_SECRET=${z_client_secret}"
    echo "RBRO_REFRESH_TOKEN=${z_refresh_token}"
  } > "${z_rbro_file}" || buc_die "Failed to write RBRO credentials file"
  chmod 600 "${z_rbro_file}" || buc_die "Failed to set RBRO file permissions"
```

Replace with:

```bash
  buc_step 'Store OAuth credentials'
  (
    umask 077
    {
      echo "RBRO_CLIENT_SECRET=${z_client_secret}"
      echo "RBRO_REFRESH_TOKEN=${z_refresh_token}"
    } > "${z_rbro_file}"
  ) || buc_die "Failed to write RBRO credentials file"
  chmod 600 "${z_rbro_file}" || buc_die "Failed to set RBRO file permissions"
```

The subshell isolates the umask change so the file is created at 0600 from the start. The chmod 600 stays as belt-and-suspenders.

**F5** — Mint `buh_prompt_secret` and swap callsite.

In `Tools/buk/buh_handbook.sh`, after the `buh_prompt` function (currently ending around line 237), insert:

```bash
# buh_prompt_secret "prompt text"
# Like buh_prompt but suppresses terminal echo of typed/pasted input.
# Emits a trailing newline to stderr so subsequent output starts on a fresh line.
buh_prompt_secret() {
  zbuh_sentinel
  printf '%s' "${1:-}" >&2
  local z_input
  read -rs z_input
  printf '\n' >&2
  printf '%s' "${z_input}"
}
```

Place this BEFORE the existing `buh_prompt_required` function so the prompts cluster together.

In `Tools/rbk/rbgp_Payor.sh:461`, change:

```bash
  z_auth_code=$(buh_prompt "Copy the authorization code and paste here: ")
```

to:

```bash
  z_auth_code=$(buh_prompt_secret "Copy the authorization code and paste here: ")
```

### Step 4 — Comments only (F8, F9, F10, F11, F12)

**F8** — Document `rbgu_rbro_load`. Decision: **document, do not inline** (the auto-source of `rbro_regime.sh` is genuine load-bearing value; AAD just landed the payor/ subdirectory migration so callers shouldn't grow path knowledge).

In `Tools/rbk/rbgu_Utility.sh`, replace the existing comment block above `rbgu_rbro_load` (lines 786-787 currently say "RBTOE: RBRO Load Pattern / Loads and validates RBRO credentials") with:

```bash
# RBTOE: RBRO Load Pattern
# Thin wrapper: defensively sources rbro_regime.sh (callers don't need to know
# its path, which moved under AAD's payor/ subdirectory migration), then
# delegates to rbro_load. Parallels rbgu_rbra_load at the call-signature level
# even though that function carries its own validation; the uniform rbgu_*
# load-through-utility convention is the load-bearing reason this wrapper exists.
```

**F9** — Comment `RBRP_OAUTH_CLIENT_ID` deliberate `min=0`. In `Tools/rbk/rbrp_regime.sh:48-49`, insert before the enrollment line:

```bash
  buv_group_enroll "OAuth Configuration"
  # min=0 deliberate — not every operator has a Payor identity (Retriever-only
  # operators authenticate via JWT SA, never via Payor OAuth). Required-at-use
  # is enforced by test -n in rbgp_Payor.sh consumers. Do not tighten to min=1.
  buv_string_enroll  RBRP_OAUTH_CLIENT_ID  0  256  "OAuth 2.0 client identifier"
```

**F10** — Comment field-name-basis scrubber. In `Tools/rbk/rbgo_OAuth.sh:153`, insert before the `buc_log_args "Debug: Show the actual response..."` line:

```bash
  # Scrubber filters by field NAME, not value. Deliberate best-effort log hygiene:
  # explicitly deletes access_token/refresh_token, then drops any field whose key
  # matches token|secret|key|password (case-insensitive) — catches id_token,
  # client_secret, etc. If the OAuth provider ever returns a new secret-carrying
  # field whose name doesn't match this regex, the scrub would miss it; update
  # the regex here when that happens.
```

**F11** — Scope-tagging header comment on `rbgp_Payor.sh`. **CORRECTION FROM ORIGINAL DOCKET:** the original docket said "OAuth ~76-566 / Depot 568-end" — that is wrong. `rbgp_payor_oauth_refresh` is OAuth and lives at line ~1123. Use the corrected map below.

In `Tools/rbk/rbgp_Payor.sh:19` (the existing single-line `# Recipe Bottle GCP Payor - ...` comment), replace with:

```bash
# Recipe Bottle GCP Payor - Billing and Destructive Lifecycle Operations
#
# Scope: this file mixes two concerns; entry points are interleaved.
#   OAuth credential flow:
#     zrbgp_refresh_capture         (~76)   refresh-token exchange
#     zrbgp_authenticate_capture    (~122)  load + exchange
#     rbgp_payor_install            (~400)  full install ceremony
#     rbgp_payor_oauth_refresh      (~1123) display refresh procedure
#   Depot lifecycle operations:
#     zrbgp_billing_attach/detach, liens, bucket helpers (~196-395)
#     rbgp_depot_levy / unmake / list (~568-1121)
#     rbgp_governor_mantle          (~1162) Governor SA reset (writes RBRA)
```

**F12** — Comment no-cache intent on `zrbgp_authenticate_capture`. In `Tools/rbk/rbgp_Payor.sh:120-121`, replace the existing two-line comment with:

```bash
# RBTOE: Payor OAuth Authentication Pattern
# Establishes Payor OAuth context by loading RBRO credentials and obtaining access token.
# Tokens are deliberately not cached — each call refreshes. Rationale: simplicity
# and freshness; refresh tokens are long-lived so the extra roundtrips are cheap
# relative to the depot operations they authorize, and uncached tokens can't grow
# stale between distinct Payor ceremonies.
```

### Step 5 — F13 producer-reality docs + BCG brace freebie + consumer-comment fix

**F13 — CORRECTION FROM ORIGINAL DOCKET:** the original docket asserted "RBRA_PRIVATE_KEY must hold '\n' escape sequences (literal backslash-n), NOT real newlines." This premise is **inverted** with respect to the actual code. The reality:

- All three writer sites use `z_private_key=$(jq -r '.private_key' ...)`. `jq -r` unescapes JSON `\n` → real newlines.
- All three writer sites then use `printf '%s' "$z_private_key"`, preserving real newlines verbatim.
- The RBRA file therefore contains a multi-line `RBRA_PRIVATE_KEY="-----BEGIN PRIVATE KEY-----<real newline>...-----END-----<real newline>"`.
- Consumer at `rbgo_OAuth.sh:125` does `printf '%b' "${RBRA_PRIVATE_KEY}\n"`. `%b` on real newlines is a no-op; PEM has no backslashes for `%b` to process. The `%b` is harmless redundancy.
- The consumer comment at `rbgo_OAuth.sh:82-83` ("contains \n sequences that must become real newlines") is **misleading or stale**.

Decision: **document reality, not the docket's misframing**. Three writer sites confirmed (rgbs_ServiceAccounts.sh checked, does NOT touch RBRA — only three writers exist).

Also confirmed during prep: **BCG brace inconsistency** at the writer sites. `rbgu:602` and `rbgg:264` use unbraced `"$z_private_key"`; `rbgp:1332` uses braced `"${z_private_key}"`. BCG mandates braces. Brace the two unbraced sites in the same pass — confirmed freebie.

**Producer comment + brace fix at all three sites:**

`Tools/rbk/rbgu_Utility.sh:602` — current:

```bash
  buc_log_args 'Write RBRA file'
  {
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "$z_client_email"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "$z_private_key"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "$z_project_id"
    printf 'RBRA_TOKEN_LIFETIME_SEC=%s\n'  "$z_lifetime_sec"
  } > "${z_rbra_path}" || buc_die "Failed to write RBRA file ${z_rbra_path}"
```

Replace with:

```bash
  buc_log_args 'Write RBRA file'
  # CAUTION: jq -r unescapes JSON \n to real newlines, so z_private_key holds a
  # multi-line PEM string. printf '%s' below preserves real newlines into the
  # RBRA file. Consumer (rbgo_OAuth.sh:zrbgo_build_jwt_capture) tolerates either
  # real-newline or '\n'-escape form via printf '%b'; do not "normalize" to one
  # form without auditing the consumer.
  {
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "${z_client_email}"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "${z_private_key}"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "${z_project_id}"
    printf 'RBRA_TOKEN_LIFETIME_SEC=%s\n'  "${z_lifetime_sec}"
  } > "${z_rbra_path}" || buc_die "Failed to write RBRA file ${z_rbra_path}"
```

(Note: also brace the other three printf args in this block while we're here — full BCG sweep on the 4-line block.)

`Tools/rbk/rbgg_Governor.sh:264` — current block (lines ~260-267):

```bash
  buc_step 'Write RBRA file' "${z_rbra_file}"
  {
    printf 'RBRA_ROLE=%s\n'                  "${z_role}"
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "$z_client_email"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "$z_private_key"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "$z_project_id"
    printf 'RBRA_TOKEN_LIFETIME_SEC=1800\n'
  } > "${z_rbra_file}" || buc_die "Failed to write RBRA file ${z_rbra_file}"
```

Replace with:

```bash
  buc_step 'Write RBRA file' "${z_rbra_file}"
  # CAUTION: jq -r unescapes JSON \n to real newlines, so z_private_key holds a
  # multi-line PEM string. printf '%s' below preserves real newlines into the
  # RBRA file. Consumer (rbgo_OAuth.sh:zrbgo_build_jwt_capture) tolerates either
  # real-newline or '\n'-escape form via printf '%b'; do not "normalize" to one
  # form without auditing the consumer.
  {
    printf 'RBRA_ROLE=%s\n'                  "${z_role}"
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "${z_client_email}"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "${z_private_key}"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "${z_project_id}"
    printf 'RBRA_TOKEN_LIFETIME_SEC=1800\n'
  } > "${z_rbra_file}" || buc_die "Failed to write RBRA file ${z_rbra_file}"
```

`Tools/rbk/rbgp_Payor.sh:1332` (already braced — only add the comment):

Insert the same CAUTION comment block immediately above the `buc_step 'Write RBRA file'` line (or above the `{` if `buc_step` isn't immediately before it — find the heredoc-style block at line 1328-1335). Block already uses `${z_private_key}` etc., so no brace edits needed here.

**Consumer-comment fix** at `Tools/rbk/rbgo_OAuth.sh:82-83`. Current:

```bash
  # RBRA_* expected: CLIENT_EMAIL, PRIVATE_KEY, TOKEN_LIFETIME_SEC
  # RBRA_PRIVATE_KEY contains \n sequences that must become real newlines for openssl
```

Replace with:

```bash
  # RBRA_* expected: CLIENT_EMAIL, PRIVATE_KEY, TOKEN_LIFETIME_SEC
  # RBRA_PRIVATE_KEY is sourced as a multi-line string (real newlines from PEM).
  # printf '%b' below is defensive and tolerates either real-newline or '\n'-escape
  # form; PEM keys contain no backslashes so %b is otherwise a no-op.
```

## Test plan

### Tier 1 — mechanical correctness

```
bash -n Tools/rbk/rbgo_OAuth.sh
bash -n Tools/rbk/rbgp_Payor.sh
bash -n Tools/rbk/rbgu_Utility.sh
bash -n Tools/rbk/rbgg_Governor.sh
bash -n Tools/rbk/rbgc_Constants.sh
bash -n Tools/rbk/rbrp_regime.sh
bash -n Tools/buk/buh_handbook.sh

tt/rbtd-s.TestSuite.fast.sh                  # expect 93/93

shellcheck Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbgp_Payor.sh Tools/buk/buh_handbook.sh \
           Tools/rbk/rbgu_Utility.sh Tools/rbk/rbgg_Governor.sh \
           Tools/rbk/rbgc_Constants.sh Tools/rbk/rbrp_regime.sh
# expect ~110 warnings (down 9-10 from 119 baseline). NO new findings.

tt/rbw-rov.ValidateOauthRegime.sh
tt/rbw-rpv.ValidatePayorRegime.sh
```

### Tier 2 — behavior on edited surfaces

Interactive Payor install — exercises F2, F3, F5 in one ceremony:

```
ls -la ~/.rbw/payor/rbro.env 2>/dev/null    # note pre-state

tt/rbw-gPI.PayorInstall.sh <path-to-oauth-client.json>

# F5: visually confirm auth code does NOT echo when typed/pasted
# F2: immediately after ceremony, stat the file
stat -f %Lp ~/.rbw/payor/rbro.env           # macOS — expect 600
# (or stat -c %a on Linux/WSL)
# F3: ceremony completes without "Failed to execute..." OAuth URL errors
```

Subsequent Payor op — exercises F12 + F3 refresh path:

```
tt/rbw-dl.PayorListsDepots.sh
```

### Tier 3 — full credential lifecycle

Order matters; Governor reset writes the RBRA the others depend on.

```
# 3.1 Governor reset (writes via rbgp_Payor.sh:1332 — F13 site #3)
tt/rbw-aM.PayorMantlesGovernor.sh

# Inspect the resulting RBRA format (F13 reality check):
ls -la output/governor-*.rbra | tail -1
cat output/governor-*.rbra | tail -10
# Expected: multi-line PEM with real newlines inside RBRA_PRIVATE_KEY="..."

# Install per the buc_bare hint emitted by mantle:
cp output/governor-*.rbra <RBDC_GOVERNOR_RBRA_FILE>      # actual path printed at end of mantle

# 3.2 Charter Retriever (writes via rbgg_Governor.sh:264 — F13 site #2)
tt/rbw-aC.GovernorChartersRetriever.sh

# 3.3 Knight Director (writes via rbgg_Governor.sh:264 — F13 site #2 again)
tt/rbw-aK.GovernorKnightsDirector.sh

# 3.4 Verify all three SAs exist
tt/rbw-aL.GovernorListsServiceAccounts.sh

# 3.5 Consumer round-trip — proves F13 reality claim end-to-end
tt/rbw-ft.RetrieverTalliesHallmarks.sh
# Clean tally output proves: producer wrote real newlines → consumer's %b accepted them.
```

## Notch

ONE `jjx_record` at the end. Explicit file list:

- `Tools/rbk/rbgo_OAuth.sh`
- `Tools/rbk/rbgp_Payor.sh`
- `Tools/rbk/rbgc_Constants.sh`
- `Tools/rbk/rbrp_regime.sh`
- `Tools/rbk/rbgu_Utility.sh`
- `Tools/rbk/rbgg_Governor.sh`
- `Tools/buk/buh_handbook.sh`

**Never** include AAS files in the record (see Cross-officium constraints above).

Intent line for the commit: synthesize from what was actually applied. Suggested base: "OAuth-surface trust hygiene sweep (F1+F6a deletes, F6b shellcheck directives in rbgp+rbgg, F3 OAuth URL constants, F2 TOCTOU subshell, F5 buh_prompt_secret, F8-F12 comments, F13 producer-reality docs + BCG brace freebie at rbgu/rbgg writer sites + consumer-comment fix; AAY slated for BCG-wide SC2153 study)."

## Out of scope (do not expand)

- F4 (ID-token scrub generalization) — folded into F7.
- F7 (factor token-exchange paths) — separate pace, pre-AAE.
- BCG-wide SC2153 study — slated as ₢A_AAY.
- Splitting `rbgp_Payor.sh` into multiple files — F11 is a comment-only map; an actual split is a larger heat.
- Behavior changes to OAuth flow, token caching, alternate auth.
- Touching `rbgv_AccessProbe.sh` — review concluded clean.

## Corrections from the docket's prior version

This docket's prior version asserted:

1. **F11**: "OAuth ~76-566 / Depot 568-end" — wrong. `rbgp_payor_oauth_refresh` is OAuth and lives at 1123. Corrected map in F11 above.
2. **F13**: "RBRA_PRIVATE_KEY must hold '\n' escape sequences" — inverted. Reality is real newlines today; `%b` is defensive. Corrected approach in F13 above.
3. **F13 candidate writer files**: original mentioned `rbgg_Governor.sh, rbgi_IAM.sh, rgbs_ServiceAccounts.sh` as candidates. Verified: the three writers are `rbgu_Utility.sh:602`, `rbgg_Governor.sh:264`, `rbgp_Payor.sh:1332`. `rbgi_IAM.sh` and `rgbs_ServiceAccounts.sh` do NOT write RBRA.
4. **F6b extension**: original scoped to `rbgp_Payor.sh` only; the same SC2153 false positive fires in `rbgg_Governor.sh:195`. Extended as a confirmed freebie since rbgg is touched by F13 anyway.
5. **BCG brace freebie**: original did not address it; rbgu:602 and rbgg:264 use unbraced expansion in the F13 block. Brace fix folded in.

## References

- Original AAN docket (this file, prior version) — for context on review provenance.
- ₢A_AAY (`bcg-shellcheck-cross-module-discipline`) — slated to study whether F6b's per-file directives should be replaced by a BCG-mandated one-liner.
- BCG (Bash Console Guide): `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`.
- `Tools/rbk/rbgc_Constants.sh:69` — existing `RBGC_OAUTH_TOKEN_URL`, sibling to the two new mints.
- Cross-officium constraints: enforce AAS file boundary throughout.

## Verification gates before notching

- [ ] Tier 1 green: bash -n clean, fast suite 93/93, shellcheck delta ~9 fewer warnings, regime validators pass.
- [ ] Tier 2 observed: F5 no-echo confirmed, F2 perms 600 confirmed, F3 URLs round-trip.
- [ ] Tier 3 observed: Governor mantled, RBRA format inspected, retriever+director chartered, list shows all three, retriever tally rounds-trips successfully.
- [ ] Git status shows only AAN target files dirty before notch (no AAS file pollution).

If any gate fails, STOP and report. Do not adjust scope to make a failing gate pass.

**[260426-0622] rough**

Drafted from ₢A_AAN in ₣A_.

## Character

Mechanical apply. All design decisions are baked into this docket; the agent's job is to execute the listed edits, run the listed tests, and notch with the listed file list. No options to evaluate. No "decide between..." sections. If anything looks ambiguous, the docket is the source of truth — re-read it rather than improvise.

This docket was built and reviewed in a prior chat; its line numbers and diffs were verified against the working tree at slate time. Spot-check before applying — line numbers may drift if other officia touch the target files.

## Pre-flight (re-verify before edits)

Run these to establish a known baseline:

```
git status --short                           # expect clean of AAN target files (see file list at end)
tt/rbtd-s.TestSuite.fast.sh                  # expect 93/93 green (47+23+8+15)
shellcheck Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbgp_Payor.sh Tools/buk/buh_handbook.sh \
           Tools/rbk/rbgu_Utility.sh Tools/rbk/rbgg_Governor.sh \
           Tools/rbk/rbgc_Constants.sh Tools/rbk/rbrp_regime.sh 2>&1 | grep -c '^In '
# expect ~119 warnings baseline (post-AAN target: ~110, drops 9: 1 SC2034 from F1, 7 SC2034 from F6a, 1-2 SC2153 from F6b directives)
```

If baseline differs significantly, investigate before editing. Another officium may have touched these files.

## Cross-officium constraints (HARD)

A parallel officium may be working on ₢A_AAS (theurge Rust + tabtargets). **Never** touch:

- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs`
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs`
- `Tools/rbk/rbtd/src/rbtdrf_fast.rs`
- `Tools/rbk/rbtd/tests/rbtdtm_manifest.rs`
- `tt/rbtd-r.Run.batch-vouch.sh`
- `tt/rbtd-s.SingleCase.batch-vouch.sh`
- `Tools/rbk/rbtd/scripts/rbte_engine.sh`

Never run `tt/rbtd-s.TestSuite.crucible.sh` or `tt/rbtd-s.TestSuite.complete.sh`. Fast suite is read-only and safe.

If `git status --short` shows uncommitted changes to AAS-affiliated files at any point, leave them alone — that's the other officium's in-flight work.

## Apply order (one notch at the end, not per-finding)

### Step 1 — Pure deletions (F1, F6a) and shellcheck directives (F6b)

**F1** — Delete dead `ZRBGO_PRIVATE_KEY_FILE`. File: `Tools/rbk/rbgo_OAuth.sh`, line 50:

```bash
  readonly ZRBGO_PRIVATE_KEY_FILE="${BURD_TEMP_DIR}/rbgo_private_key.pem"
```

Delete the entire line.

**F6a** — Delete 7 dead `ZRBGP_INFIX_*` constants in `Tools/rbk/rbgp_Payor.sh`. Verified zero consumers tree-wide. Lines 46, 47, 48, 53, 54, 61, 63:

```bash
  readonly ZRBGP_INFIX_PROJECT_DELETE="project_delete"
  readonly ZRBGP_INFIX_PROJECT_RESTORE="project_restore"
  readonly ZRBGP_INFIX_PROJECT_STATE="project_state"
  readonly ZRBGP_INFIX_CREATE_REPO="create_repo"
  readonly ZRBGP_INFIX_VERIFY_REPO="verify_repo"
  readonly ZRBGP_INFIX_GOV_VERIFY_SA="gov_verify_sa"
  readonly ZRBGP_INFIX_GOV_IAM="gov_iam"
```

Delete those exact lines; leave the other 11 INFIX constants intact.

**F6b** — Add file-scoped shellcheck directive to silence SC2153 false positive on cross-module `ZRBGU_PREFIX`. Apply to BOTH files (the extension to `rbgg_Governor.sh` is a confirmed freebie since both have the same false-positive shape and the file is touched by F13 anyway).

In `Tools/rbk/rbgp_Payor.sh`, insert after line 25 (after `ZRBGP_SOURCED=1`):

```bash
# shellcheck disable=SC2153
# ZRBGU_PREFIX and ZRBGU_POSTFIX_* are defined in rbgu_Utility.sh and shared
# across modules via the kindle/sentinel chain (zrbgu_sentinel asserted in
# zrbgp_kindle). Shellcheck cannot follow the runtime sourcing graph.
```

In `Tools/rbk/rbgg_Governor.sh`, insert the same block after the file's own `ZRBGG_SOURCED=1` line (find via grep — the multiple-inclusion guard).

Note: the BCG-wide answer to this class of false positive is being studied under ₢A_AAY (`bcg-shellcheck-cross-module-discipline`). When AAY lands, these per-file directives become redundant and should be removed by AAY's apply pass.

### Step 2 — F3 OAuth URL constants

`RBGC_OAUTH_TOKEN_URL` already exists at `Tools/rbk/rbgc_Constants.sh:69`. Mint two more.

In `rbgc_Constants.sh`, after line 69 (`readonly RBGC_OAUTH_TOKEN_URL=...`), insert:

```bash
  readonly RBGC_OAUTH_AUTHORIZE_URL="https://accounts.google.com/o/oauth2/v2/auth"
  readonly RBGC_OAUTH_USERINFO_URL="https://www.googleapis.com/oauth2/v3/userinfo"
```

Then four substitutions in `Tools/rbk/rbgp_Payor.sh`:

- Line ~102: `"https://oauth2.googleapis.com/token"` → `"${RBGC_OAUTH_TOKEN_URL}"`
- Line ~443: replace ONLY the bare URL portion, NOT the query string. The full line is:
  ```
  local -r z_auth_url="https://accounts.google.com/o/oauth2/v2/auth?client_id=${z_client_id}&redirect_uri=urn:ietf:wg:oauth:2.0:oob&scope=openid%20email%20https://www.googleapis.com/auth/cloud-platform%20https://www.googleapis.com/auth/cloud-billing&response_type=code&access_type=offline"
  ```
  Replace `https://accounts.google.com/o/oauth2/v2/auth` (the prefix) with `${RBGC_OAUTH_AUTHORIZE_URL}` — keep the entire `?client_id=...` query string inline (it's parameterized, not extractable).
- Line ~485: `"https://oauth2.googleapis.com/token"` → `"${RBGC_OAUTH_TOKEN_URL}"`
- Line ~538: `"https://www.googleapis.com/oauth2/v3/userinfo"` → `"${RBGC_OAUTH_USERINFO_URL}"`

### Step 3 — Behavior-adjacent (F2, F5)

**F2** — Close TOCTOU on RBRO write. `Tools/rbk/rbgp_Payor.sh:504-509`. Current:

```bash
  buc_step 'Store OAuth credentials'
  {
    echo "RBRO_CLIENT_SECRET=${z_client_secret}"
    echo "RBRO_REFRESH_TOKEN=${z_refresh_token}"
  } > "${z_rbro_file}" || buc_die "Failed to write RBRO credentials file"
  chmod 600 "${z_rbro_file}" || buc_die "Failed to set RBRO file permissions"
```

Replace with:

```bash
  buc_step 'Store OAuth credentials'
  (
    umask 077
    {
      echo "RBRO_CLIENT_SECRET=${z_client_secret}"
      echo "RBRO_REFRESH_TOKEN=${z_refresh_token}"
    } > "${z_rbro_file}"
  ) || buc_die "Failed to write RBRO credentials file"
  chmod 600 "${z_rbro_file}" || buc_die "Failed to set RBRO file permissions"
```

The subshell isolates the umask change so the file is created at 0600 from the start. The chmod 600 stays as belt-and-suspenders.

**F5** — Mint `buh_prompt_secret` and swap callsite.

In `Tools/buk/buh_handbook.sh`, after the `buh_prompt` function (currently ending around line 237), insert:

```bash
# buh_prompt_secret "prompt text"
# Like buh_prompt but suppresses terminal echo of typed/pasted input.
# Emits a trailing newline to stderr so subsequent output starts on a fresh line.
buh_prompt_secret() {
  zbuh_sentinel
  printf '%s' "${1:-}" >&2
  local z_input
  read -rs z_input
  printf '\n' >&2
  printf '%s' "${z_input}"
}
```

Place this BEFORE the existing `buh_prompt_required` function so the prompts cluster together.

In `Tools/rbk/rbgp_Payor.sh:461`, change:

```bash
  z_auth_code=$(buh_prompt "Copy the authorization code and paste here: ")
```

to:

```bash
  z_auth_code=$(buh_prompt_secret "Copy the authorization code and paste here: ")
```

### Step 4 — Comments only (F8, F9, F10, F11, F12)

**F8** — Document `rbgu_rbro_load`. Decision: **document, do not inline** (the auto-source of `rbro_regime.sh` is genuine load-bearing value; AAD just landed the payor/ subdirectory migration so callers shouldn't grow path knowledge).

In `Tools/rbk/rbgu_Utility.sh`, replace the existing comment block above `rbgu_rbro_load` (lines 786-787 currently say "RBTOE: RBRO Load Pattern / Loads and validates RBRO credentials") with:

```bash
# RBTOE: RBRO Load Pattern
# Thin wrapper: defensively sources rbro_regime.sh (callers don't need to know
# its path, which moved under AAD's payor/ subdirectory migration), then
# delegates to rbro_load. Parallels rbgu_rbra_load at the call-signature level
# even though that function carries its own validation; the uniform rbgu_*
# load-through-utility convention is the load-bearing reason this wrapper exists.
```

**F9** — Comment `RBRP_OAUTH_CLIENT_ID` deliberate `min=0`. In `Tools/rbk/rbrp_regime.sh:48-49`, insert before the enrollment line:

```bash
  buv_group_enroll "OAuth Configuration"
  # min=0 deliberate — not every operator has a Payor identity (Retriever-only
  # operators authenticate via JWT SA, never via Payor OAuth). Required-at-use
  # is enforced by test -n in rbgp_Payor.sh consumers. Do not tighten to min=1.
  buv_string_enroll  RBRP_OAUTH_CLIENT_ID  0  256  "OAuth 2.0 client identifier"
```

**F10** — Comment field-name-basis scrubber. In `Tools/rbk/rbgo_OAuth.sh:153`, insert before the `buc_log_args "Debug: Show the actual response..."` line:

```bash
  # Scrubber filters by field NAME, not value. Deliberate best-effort log hygiene:
  # explicitly deletes access_token/refresh_token, then drops any field whose key
  # matches token|secret|key|password (case-insensitive) — catches id_token,
  # client_secret, etc. If the OAuth provider ever returns a new secret-carrying
  # field whose name doesn't match this regex, the scrub would miss it; update
  # the regex here when that happens.
```

**F11** — Scope-tagging header comment on `rbgp_Payor.sh`. **CORRECTION FROM ORIGINAL DOCKET:** the original docket said "OAuth ~76-566 / Depot 568-end" — that is wrong. `rbgp_payor_oauth_refresh` is OAuth and lives at line ~1123. Use the corrected map below.

In `Tools/rbk/rbgp_Payor.sh:19` (the existing single-line `# Recipe Bottle GCP Payor - ...` comment), replace with:

```bash
# Recipe Bottle GCP Payor - Billing and Destructive Lifecycle Operations
#
# Scope: this file mixes two concerns; entry points are interleaved.
#   OAuth credential flow:
#     zrbgp_refresh_capture         (~76)   refresh-token exchange
#     zrbgp_authenticate_capture    (~122)  load + exchange
#     rbgp_payor_install            (~400)  full install ceremony
#     rbgp_payor_oauth_refresh      (~1123) display refresh procedure
#   Depot lifecycle operations:
#     zrbgp_billing_attach/detach, liens, bucket helpers (~196-395)
#     rbgp_depot_levy / unmake / list (~568-1121)
#     rbgp_governor_mantle          (~1162) Governor SA reset (writes RBRA)
```

**F12** — Comment no-cache intent on `zrbgp_authenticate_capture`. In `Tools/rbk/rbgp_Payor.sh:120-121`, replace the existing two-line comment with:

```bash
# RBTOE: Payor OAuth Authentication Pattern
# Establishes Payor OAuth context by loading RBRO credentials and obtaining access token.
# Tokens are deliberately not cached — each call refreshes. Rationale: simplicity
# and freshness; refresh tokens are long-lived so the extra roundtrips are cheap
# relative to the depot operations they authorize, and uncached tokens can't grow
# stale between distinct Payor ceremonies.
```

### Step 5 — F13 producer-reality docs + BCG brace freebie + consumer-comment fix

**F13 — CORRECTION FROM ORIGINAL DOCKET:** the original docket asserted "RBRA_PRIVATE_KEY must hold '\n' escape sequences (literal backslash-n), NOT real newlines." This premise is **inverted** with respect to the actual code. The reality:

- All three writer sites use `z_private_key=$(jq -r '.private_key' ...)`. `jq -r` unescapes JSON `\n` → real newlines.
- All three writer sites then use `printf '%s' "$z_private_key"`, preserving real newlines verbatim.
- The RBRA file therefore contains a multi-line `RBRA_PRIVATE_KEY="-----BEGIN PRIVATE KEY-----<real newline>...-----END-----<real newline>"`.
- Consumer at `rbgo_OAuth.sh:125` does `printf '%b' "${RBRA_PRIVATE_KEY}\n"`. `%b` on real newlines is a no-op; PEM has no backslashes for `%b` to process. The `%b` is harmless redundancy.
- The consumer comment at `rbgo_OAuth.sh:82-83` ("contains \n sequences that must become real newlines") is **misleading or stale**.

Decision: **document reality, not the docket's misframing**. Three writer sites confirmed (rgbs_ServiceAccounts.sh checked, does NOT touch RBRA — only three writers exist).

Also confirmed during prep: **BCG brace inconsistency** at the writer sites. `rbgu:602` and `rbgg:264` use unbraced `"$z_private_key"`; `rbgp:1332` uses braced `"${z_private_key}"`. BCG mandates braces. Brace the two unbraced sites in the same pass — confirmed freebie.

**Producer comment + brace fix at all three sites:**

`Tools/rbk/rbgu_Utility.sh:602` — current:

```bash
  buc_log_args 'Write RBRA file'
  {
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "$z_client_email"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "$z_private_key"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "$z_project_id"
    printf 'RBRA_TOKEN_LIFETIME_SEC=%s\n'  "$z_lifetime_sec"
  } > "${z_rbra_path}" || buc_die "Failed to write RBRA file ${z_rbra_path}"
```

Replace with:

```bash
  buc_log_args 'Write RBRA file'
  # CAUTION: jq -r unescapes JSON \n to real newlines, so z_private_key holds a
  # multi-line PEM string. printf '%s' below preserves real newlines into the
  # RBRA file. Consumer (rbgo_OAuth.sh:zrbgo_build_jwt_capture) tolerates either
  # real-newline or '\n'-escape form via printf '%b'; do not "normalize" to one
  # form without auditing the consumer.
  {
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "${z_client_email}"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "${z_private_key}"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "${z_project_id}"
    printf 'RBRA_TOKEN_LIFETIME_SEC=%s\n'  "${z_lifetime_sec}"
  } > "${z_rbra_path}" || buc_die "Failed to write RBRA file ${z_rbra_path}"
```

(Note: also brace the other three printf args in this block while we're here — full BCG sweep on the 4-line block.)

`Tools/rbk/rbgg_Governor.sh:264` — current block (lines ~260-267):

```bash
  buc_step 'Write RBRA file' "${z_rbra_file}"
  {
    printf 'RBRA_ROLE=%s\n'                  "${z_role}"
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "$z_client_email"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "$z_private_key"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "$z_project_id"
    printf 'RBRA_TOKEN_LIFETIME_SEC=1800\n'
  } > "${z_rbra_file}" || buc_die "Failed to write RBRA file ${z_rbra_file}"
```

Replace with:

```bash
  buc_step 'Write RBRA file' "${z_rbra_file}"
  # CAUTION: jq -r unescapes JSON \n to real newlines, so z_private_key holds a
  # multi-line PEM string. printf '%s' below preserves real newlines into the
  # RBRA file. Consumer (rbgo_OAuth.sh:zrbgo_build_jwt_capture) tolerates either
  # real-newline or '\n'-escape form via printf '%b'; do not "normalize" to one
  # form without auditing the consumer.
  {
    printf 'RBRA_ROLE=%s\n'                  "${z_role}"
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "${z_client_email}"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "${z_private_key}"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "${z_project_id}"
    printf 'RBRA_TOKEN_LIFETIME_SEC=1800\n'
  } > "${z_rbra_file}" || buc_die "Failed to write RBRA file ${z_rbra_file}"
```

`Tools/rbk/rbgp_Payor.sh:1332` (already braced — only add the comment):

Insert the same CAUTION comment block immediately above the `buc_step 'Write RBRA file'` line (or above the `{` if `buc_step` isn't immediately before it — find the heredoc-style block at line 1328-1335). Block already uses `${z_private_key}` etc., so no brace edits needed here.

**Consumer-comment fix** at `Tools/rbk/rbgo_OAuth.sh:82-83`. Current:

```bash
  # RBRA_* expected: CLIENT_EMAIL, PRIVATE_KEY, TOKEN_LIFETIME_SEC
  # RBRA_PRIVATE_KEY contains \n sequences that must become real newlines for openssl
```

Replace with:

```bash
  # RBRA_* expected: CLIENT_EMAIL, PRIVATE_KEY, TOKEN_LIFETIME_SEC
  # RBRA_PRIVATE_KEY is sourced as a multi-line string (real newlines from PEM).
  # printf '%b' below is defensive and tolerates either real-newline or '\n'-escape
  # form; PEM keys contain no backslashes so %b is otherwise a no-op.
```

## Test plan

### Tier 1 — mechanical correctness

```
bash -n Tools/rbk/rbgo_OAuth.sh
bash -n Tools/rbk/rbgp_Payor.sh
bash -n Tools/rbk/rbgu_Utility.sh
bash -n Tools/rbk/rbgg_Governor.sh
bash -n Tools/rbk/rbgc_Constants.sh
bash -n Tools/rbk/rbrp_regime.sh
bash -n Tools/buk/buh_handbook.sh

tt/rbtd-s.TestSuite.fast.sh                  # expect 93/93

shellcheck Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbgp_Payor.sh Tools/buk/buh_handbook.sh \
           Tools/rbk/rbgu_Utility.sh Tools/rbk/rbgg_Governor.sh \
           Tools/rbk/rbgc_Constants.sh Tools/rbk/rbrp_regime.sh
# expect ~110 warnings (down 9-10 from 119 baseline). NO new findings.

tt/rbw-rov.ValidateOauthRegime.sh
tt/rbw-rpv.ValidatePayorRegime.sh
```

### Tier 2 — behavior on edited surfaces

Interactive Payor install — exercises F2, F3, F5 in one ceremony:

```
ls -la ~/.rbw/payor/rbro.env 2>/dev/null    # note pre-state

tt/rbw-gPI.PayorInstall.sh <path-to-oauth-client.json>

# F5: visually confirm auth code does NOT echo when typed/pasted
# F2: immediately after ceremony, stat the file
stat -f %Lp ~/.rbw/payor/rbro.env           # macOS — expect 600
# (or stat -c %a on Linux/WSL)
# F3: ceremony completes without "Failed to execute..." OAuth URL errors
```

Subsequent Payor op — exercises F12 + F3 refresh path:

```
tt/rbw-dl.PayorListsDepots.sh
```

### Tier 3 — full credential lifecycle

Order matters; Governor reset writes the RBRA the others depend on.

```
# 3.1 Governor reset (writes via rbgp_Payor.sh:1332 — F13 site #3)
tt/rbw-aM.PayorMantlesGovernor.sh

# Inspect the resulting RBRA format (F13 reality check):
ls -la output/governor-*.rbra | tail -1
cat output/governor-*.rbra | tail -10
# Expected: multi-line PEM with real newlines inside RBRA_PRIVATE_KEY="..."

# Install per the buc_bare hint emitted by mantle:
cp output/governor-*.rbra <RBDC_GOVERNOR_RBRA_FILE>      # actual path printed at end of mantle

# 3.2 Charter Retriever (writes via rbgg_Governor.sh:264 — F13 site #2)
tt/rbw-aC.GovernorChartersRetriever.sh

# 3.3 Knight Director (writes via rbgg_Governor.sh:264 — F13 site #2 again)
tt/rbw-aK.GovernorKnightsDirector.sh

# 3.4 Verify all three SAs exist
tt/rbw-aL.GovernorListsServiceAccounts.sh

# 3.5 Consumer round-trip — proves F13 reality claim end-to-end
tt/rbw-ft.RetrieverTalliesHallmarks.sh
# Clean tally output proves: producer wrote real newlines → consumer's %b accepted them.
```

## Notch

ONE `jjx_record` at the end. Explicit file list:

- `Tools/rbk/rbgo_OAuth.sh`
- `Tools/rbk/rbgp_Payor.sh`
- `Tools/rbk/rbgc_Constants.sh`
- `Tools/rbk/rbrp_regime.sh`
- `Tools/rbk/rbgu_Utility.sh`
- `Tools/rbk/rbgg_Governor.sh`
- `Tools/buk/buh_handbook.sh`

**Never** include AAS files in the record (see Cross-officium constraints above).

Intent line for the commit: synthesize from what was actually applied. Suggested base: "OAuth-surface trust hygiene sweep (F1+F6a deletes, F6b shellcheck directives in rbgp+rbgg, F3 OAuth URL constants, F2 TOCTOU subshell, F5 buh_prompt_secret, F8-F12 comments, F13 producer-reality docs + BCG brace freebie at rbgu/rbgg writer sites + consumer-comment fix; AAY slated for BCG-wide SC2153 study)."

## Out of scope (do not expand)

- F4 (ID-token scrub generalization) — folded into F7.
- F7 (factor token-exchange paths) — separate pace, pre-AAE.
- BCG-wide SC2153 study — slated as ₢A_AAY.
- Splitting `rbgp_Payor.sh` into multiple files — F11 is a comment-only map; an actual split is a larger heat.
- Behavior changes to OAuth flow, token caching, alternate auth.
- Touching `rbgv_AccessProbe.sh` — review concluded clean.

## Corrections from the docket's prior version

This docket's prior version asserted:

1. **F11**: "OAuth ~76-566 / Depot 568-end" — wrong. `rbgp_payor_oauth_refresh` is OAuth and lives at 1123. Corrected map in F11 above.
2. **F13**: "RBRA_PRIVATE_KEY must hold '\n' escape sequences" — inverted. Reality is real newlines today; `%b` is defensive. Corrected approach in F13 above.
3. **F13 candidate writer files**: original mentioned `rbgg_Governor.sh, rbgi_IAM.sh, rgbs_ServiceAccounts.sh` as candidates. Verified: the three writers are `rbgu_Utility.sh:602`, `rbgg_Governor.sh:264`, `rbgp_Payor.sh:1332`. `rbgi_IAM.sh` and `rgbs_ServiceAccounts.sh` do NOT write RBRA.
4. **F6b extension**: original scoped to `rbgp_Payor.sh` only; the same SC2153 false positive fires in `rbgg_Governor.sh:195`. Extended as a confirmed freebie since rbgg is touched by F13 anyway.
5. **BCG brace freebie**: original did not address it; rbgu:602 and rbgg:264 use unbraced expansion in the F13 block. Brace fix folded in.

## References

- Original AAN docket (this file, prior version) — for context on review provenance.
- ₢A_AAY (`bcg-shellcheck-cross-module-discipline`) — slated to study whether F6b's per-file directives should be replaced by a BCG-mandated one-liner.
- BCG (Bash Console Guide): `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`.
- `Tools/rbk/rbgc_Constants.sh:69` — existing `RBGC_OAUTH_TOKEN_URL`, sibling to the two new mints.
- Cross-officium constraints: enforce AAS file boundary throughout.

## Verification gates before notching

- [ ] Tier 1 green: bash -n clean, fast suite 93/93, shellcheck delta ~9 fewer warnings, regime validators pass.
- [ ] Tier 2 observed: F5 no-echo confirmed, F2 perms 600 confirmed, F3 URLs round-trip.
- [ ] Tier 3 observed: Governor mantled, RBRA format inspected, retriever+director chartered, list shows all three, retriever tally rounds-trips successfully.
- [ ] Git status shows only AAN target files dirty before notch (no AAS file pollution).

If any gate fails, STOP and report. Do not adjust scope to make a failing gate pass.

**[260425-0945] rough**

## Character

Mechanical apply. All design decisions are baked into this docket; the agent's job is to execute the listed edits, run the listed tests, and notch with the listed file list. No options to evaluate. No "decide between..." sections. If anything looks ambiguous, the docket is the source of truth — re-read it rather than improvise.

This docket was built and reviewed in a prior chat; its line numbers and diffs were verified against the working tree at slate time. Spot-check before applying — line numbers may drift if other officia touch the target files.

## Pre-flight (re-verify before edits)

Run these to establish a known baseline:

```
git status --short                           # expect clean of AAN target files (see file list at end)
tt/rbtd-s.TestSuite.fast.sh                  # expect 93/93 green (47+23+8+15)
shellcheck Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbgp_Payor.sh Tools/buk/buh_handbook.sh \
           Tools/rbk/rbgu_Utility.sh Tools/rbk/rbgg_Governor.sh \
           Tools/rbk/rbgc_Constants.sh Tools/rbk/rbrp_regime.sh 2>&1 | grep -c '^In '
# expect ~119 warnings baseline (post-AAN target: ~110, drops 9: 1 SC2034 from F1, 7 SC2034 from F6a, 1-2 SC2153 from F6b directives)
```

If baseline differs significantly, investigate before editing. Another officium may have touched these files.

## Cross-officium constraints (HARD)

A parallel officium may be working on ₢A_AAS (theurge Rust + tabtargets). **Never** touch:

- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs`
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs`
- `Tools/rbk/rbtd/src/rbtdrf_fast.rs`
- `Tools/rbk/rbtd/tests/rbtdtm_manifest.rs`
- `tt/rbtd-r.Run.batch-vouch.sh`
- `tt/rbtd-s.SingleCase.batch-vouch.sh`
- `Tools/rbk/rbtd/scripts/rbte_engine.sh`

Never run `tt/rbtd-s.TestSuite.crucible.sh` or `tt/rbtd-s.TestSuite.complete.sh`. Fast suite is read-only and safe.

If `git status --short` shows uncommitted changes to AAS-affiliated files at any point, leave them alone — that's the other officium's in-flight work.

## Apply order (one notch at the end, not per-finding)

### Step 1 — Pure deletions (F1, F6a) and shellcheck directives (F6b)

**F1** — Delete dead `ZRBGO_PRIVATE_KEY_FILE`. File: `Tools/rbk/rbgo_OAuth.sh`, line 50:

```bash
  readonly ZRBGO_PRIVATE_KEY_FILE="${BURD_TEMP_DIR}/rbgo_private_key.pem"
```

Delete the entire line.

**F6a** — Delete 7 dead `ZRBGP_INFIX_*` constants in `Tools/rbk/rbgp_Payor.sh`. Verified zero consumers tree-wide. Lines 46, 47, 48, 53, 54, 61, 63:

```bash
  readonly ZRBGP_INFIX_PROJECT_DELETE="project_delete"
  readonly ZRBGP_INFIX_PROJECT_RESTORE="project_restore"
  readonly ZRBGP_INFIX_PROJECT_STATE="project_state"
  readonly ZRBGP_INFIX_CREATE_REPO="create_repo"
  readonly ZRBGP_INFIX_VERIFY_REPO="verify_repo"
  readonly ZRBGP_INFIX_GOV_VERIFY_SA="gov_verify_sa"
  readonly ZRBGP_INFIX_GOV_IAM="gov_iam"
```

Delete those exact lines; leave the other 11 INFIX constants intact.

**F6b** — Add file-scoped shellcheck directive to silence SC2153 false positive on cross-module `ZRBGU_PREFIX`. Apply to BOTH files (the extension to `rbgg_Governor.sh` is a confirmed freebie since both have the same false-positive shape and the file is touched by F13 anyway).

In `Tools/rbk/rbgp_Payor.sh`, insert after line 25 (after `ZRBGP_SOURCED=1`):

```bash
# shellcheck disable=SC2153
# ZRBGU_PREFIX and ZRBGU_POSTFIX_* are defined in rbgu_Utility.sh and shared
# across modules via the kindle/sentinel chain (zrbgu_sentinel asserted in
# zrbgp_kindle). Shellcheck cannot follow the runtime sourcing graph.
```

In `Tools/rbk/rbgg_Governor.sh`, insert the same block after the file's own `ZRBGG_SOURCED=1` line (find via grep — the multiple-inclusion guard).

Note: the BCG-wide answer to this class of false positive is being studied under ₢A_AAY (`bcg-shellcheck-cross-module-discipline`). When AAY lands, these per-file directives become redundant and should be removed by AAY's apply pass.

### Step 2 — F3 OAuth URL constants

`RBGC_OAUTH_TOKEN_URL` already exists at `Tools/rbk/rbgc_Constants.sh:69`. Mint two more.

In `rbgc_Constants.sh`, after line 69 (`readonly RBGC_OAUTH_TOKEN_URL=...`), insert:

```bash
  readonly RBGC_OAUTH_AUTHORIZE_URL="https://accounts.google.com/o/oauth2/v2/auth"
  readonly RBGC_OAUTH_USERINFO_URL="https://www.googleapis.com/oauth2/v3/userinfo"
```

Then four substitutions in `Tools/rbk/rbgp_Payor.sh`:

- Line ~102: `"https://oauth2.googleapis.com/token"` → `"${RBGC_OAUTH_TOKEN_URL}"`
- Line ~443: replace ONLY the bare URL portion, NOT the query string. The full line is:
  ```
  local -r z_auth_url="https://accounts.google.com/o/oauth2/v2/auth?client_id=${z_client_id}&redirect_uri=urn:ietf:wg:oauth:2.0:oob&scope=openid%20email%20https://www.googleapis.com/auth/cloud-platform%20https://www.googleapis.com/auth/cloud-billing&response_type=code&access_type=offline"
  ```
  Replace `https://accounts.google.com/o/oauth2/v2/auth` (the prefix) with `${RBGC_OAUTH_AUTHORIZE_URL}` — keep the entire `?client_id=...` query string inline (it's parameterized, not extractable).
- Line ~485: `"https://oauth2.googleapis.com/token"` → `"${RBGC_OAUTH_TOKEN_URL}"`
- Line ~538: `"https://www.googleapis.com/oauth2/v3/userinfo"` → `"${RBGC_OAUTH_USERINFO_URL}"`

### Step 3 — Behavior-adjacent (F2, F5)

**F2** — Close TOCTOU on RBRO write. `Tools/rbk/rbgp_Payor.sh:504-509`. Current:

```bash
  buc_step 'Store OAuth credentials'
  {
    echo "RBRO_CLIENT_SECRET=${z_client_secret}"
    echo "RBRO_REFRESH_TOKEN=${z_refresh_token}"
  } > "${z_rbro_file}" || buc_die "Failed to write RBRO credentials file"
  chmod 600 "${z_rbro_file}" || buc_die "Failed to set RBRO file permissions"
```

Replace with:

```bash
  buc_step 'Store OAuth credentials'
  (
    umask 077
    {
      echo "RBRO_CLIENT_SECRET=${z_client_secret}"
      echo "RBRO_REFRESH_TOKEN=${z_refresh_token}"
    } > "${z_rbro_file}"
  ) || buc_die "Failed to write RBRO credentials file"
  chmod 600 "${z_rbro_file}" || buc_die "Failed to set RBRO file permissions"
```

The subshell isolates the umask change so the file is created at 0600 from the start. The chmod 600 stays as belt-and-suspenders.

**F5** — Mint `buh_prompt_secret` and swap callsite.

In `Tools/buk/buh_handbook.sh`, after the `buh_prompt` function (currently ending around line 237), insert:

```bash
# buh_prompt_secret "prompt text"
# Like buh_prompt but suppresses terminal echo of typed/pasted input.
# Emits a trailing newline to stderr so subsequent output starts on a fresh line.
buh_prompt_secret() {
  zbuh_sentinel
  printf '%s' "${1:-}" >&2
  local z_input
  read -rs z_input
  printf '\n' >&2
  printf '%s' "${z_input}"
}
```

Place this BEFORE the existing `buh_prompt_required` function so the prompts cluster together.

In `Tools/rbk/rbgp_Payor.sh:461`, change:

```bash
  z_auth_code=$(buh_prompt "Copy the authorization code and paste here: ")
```

to:

```bash
  z_auth_code=$(buh_prompt_secret "Copy the authorization code and paste here: ")
```

### Step 4 — Comments only (F8, F9, F10, F11, F12)

**F8** — Document `rbgu_rbro_load`. Decision: **document, do not inline** (the auto-source of `rbro_regime.sh` is genuine load-bearing value; AAD just landed the payor/ subdirectory migration so callers shouldn't grow path knowledge).

In `Tools/rbk/rbgu_Utility.sh`, replace the existing comment block above `rbgu_rbro_load` (lines 786-787 currently say "RBTOE: RBRO Load Pattern / Loads and validates RBRO credentials") with:

```bash
# RBTOE: RBRO Load Pattern
# Thin wrapper: defensively sources rbro_regime.sh (callers don't need to know
# its path, which moved under AAD's payor/ subdirectory migration), then
# delegates to rbro_load. Parallels rbgu_rbra_load at the call-signature level
# even though that function carries its own validation; the uniform rbgu_*
# load-through-utility convention is the load-bearing reason this wrapper exists.
```

**F9** — Comment `RBRP_OAUTH_CLIENT_ID` deliberate `min=0`. In `Tools/rbk/rbrp_regime.sh:48-49`, insert before the enrollment line:

```bash
  buv_group_enroll "OAuth Configuration"
  # min=0 deliberate — not every operator has a Payor identity (Retriever-only
  # operators authenticate via JWT SA, never via Payor OAuth). Required-at-use
  # is enforced by test -n in rbgp_Payor.sh consumers. Do not tighten to min=1.
  buv_string_enroll  RBRP_OAUTH_CLIENT_ID  0  256  "OAuth 2.0 client identifier"
```

**F10** — Comment field-name-basis scrubber. In `Tools/rbk/rbgo_OAuth.sh:153`, insert before the `buc_log_args "Debug: Show the actual response..."` line:

```bash
  # Scrubber filters by field NAME, not value. Deliberate best-effort log hygiene:
  # explicitly deletes access_token/refresh_token, then drops any field whose key
  # matches token|secret|key|password (case-insensitive) — catches id_token,
  # client_secret, etc. If the OAuth provider ever returns a new secret-carrying
  # field whose name doesn't match this regex, the scrub would miss it; update
  # the regex here when that happens.
```

**F11** — Scope-tagging header comment on `rbgp_Payor.sh`. **CORRECTION FROM ORIGINAL DOCKET:** the original docket said "OAuth ~76-566 / Depot 568-end" — that is wrong. `rbgp_payor_oauth_refresh` is OAuth and lives at line ~1123. Use the corrected map below.

In `Tools/rbk/rbgp_Payor.sh:19` (the existing single-line `# Recipe Bottle GCP Payor - ...` comment), replace with:

```bash
# Recipe Bottle GCP Payor - Billing and Destructive Lifecycle Operations
#
# Scope: this file mixes two concerns; entry points are interleaved.
#   OAuth credential flow:
#     zrbgp_refresh_capture         (~76)   refresh-token exchange
#     zrbgp_authenticate_capture    (~122)  load + exchange
#     rbgp_payor_install            (~400)  full install ceremony
#     rbgp_payor_oauth_refresh      (~1123) display refresh procedure
#   Depot lifecycle operations:
#     zrbgp_billing_attach/detach, liens, bucket helpers (~196-395)
#     rbgp_depot_levy / unmake / list (~568-1121)
#     rbgp_governor_mantle          (~1162) Governor SA reset (writes RBRA)
```

**F12** — Comment no-cache intent on `zrbgp_authenticate_capture`. In `Tools/rbk/rbgp_Payor.sh:120-121`, replace the existing two-line comment with:

```bash
# RBTOE: Payor OAuth Authentication Pattern
# Establishes Payor OAuth context by loading RBRO credentials and obtaining access token.
# Tokens are deliberately not cached — each call refreshes. Rationale: simplicity
# and freshness; refresh tokens are long-lived so the extra roundtrips are cheap
# relative to the depot operations they authorize, and uncached tokens can't grow
# stale between distinct Payor ceremonies.
```

### Step 5 — F13 producer-reality docs + BCG brace freebie + consumer-comment fix

**F13 — CORRECTION FROM ORIGINAL DOCKET:** the original docket asserted "RBRA_PRIVATE_KEY must hold '\n' escape sequences (literal backslash-n), NOT real newlines." This premise is **inverted** with respect to the actual code. The reality:

- All three writer sites use `z_private_key=$(jq -r '.private_key' ...)`. `jq -r` unescapes JSON `\n` → real newlines.
- All three writer sites then use `printf '%s' "$z_private_key"`, preserving real newlines verbatim.
- The RBRA file therefore contains a multi-line `RBRA_PRIVATE_KEY="-----BEGIN PRIVATE KEY-----<real newline>...-----END-----<real newline>"`.
- Consumer at `rbgo_OAuth.sh:125` does `printf '%b' "${RBRA_PRIVATE_KEY}\n"`. `%b` on real newlines is a no-op; PEM has no backslashes for `%b` to process. The `%b` is harmless redundancy.
- The consumer comment at `rbgo_OAuth.sh:82-83` ("contains \n sequences that must become real newlines") is **misleading or stale**.

Decision: **document reality, not the docket's misframing**. Three writer sites confirmed (rgbs_ServiceAccounts.sh checked, does NOT touch RBRA — only three writers exist).

Also confirmed during prep: **BCG brace inconsistency** at the writer sites. `rbgu:602` and `rbgg:264` use unbraced `"$z_private_key"`; `rbgp:1332` uses braced `"${z_private_key}"`. BCG mandates braces. Brace the two unbraced sites in the same pass — confirmed freebie.

**Producer comment + brace fix at all three sites:**

`Tools/rbk/rbgu_Utility.sh:602` — current:

```bash
  buc_log_args 'Write RBRA file'
  {
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "$z_client_email"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "$z_private_key"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "$z_project_id"
    printf 'RBRA_TOKEN_LIFETIME_SEC=%s\n'  "$z_lifetime_sec"
  } > "${z_rbra_path}" || buc_die "Failed to write RBRA file ${z_rbra_path}"
```

Replace with:

```bash
  buc_log_args 'Write RBRA file'
  # CAUTION: jq -r unescapes JSON \n to real newlines, so z_private_key holds a
  # multi-line PEM string. printf '%s' below preserves real newlines into the
  # RBRA file. Consumer (rbgo_OAuth.sh:zrbgo_build_jwt_capture) tolerates either
  # real-newline or '\n'-escape form via printf '%b'; do not "normalize" to one
  # form without auditing the consumer.
  {
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "${z_client_email}"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "${z_private_key}"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "${z_project_id}"
    printf 'RBRA_TOKEN_LIFETIME_SEC=%s\n'  "${z_lifetime_sec}"
  } > "${z_rbra_path}" || buc_die "Failed to write RBRA file ${z_rbra_path}"
```

(Note: also brace the other three printf args in this block while we're here — full BCG sweep on the 4-line block.)

`Tools/rbk/rbgg_Governor.sh:264` — current block (lines ~260-267):

```bash
  buc_step 'Write RBRA file' "${z_rbra_file}"
  {
    printf 'RBRA_ROLE=%s\n'                  "${z_role}"
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "$z_client_email"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "$z_private_key"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "$z_project_id"
    printf 'RBRA_TOKEN_LIFETIME_SEC=1800\n'
  } > "${z_rbra_file}" || buc_die "Failed to write RBRA file ${z_rbra_file}"
```

Replace with:

```bash
  buc_step 'Write RBRA file' "${z_rbra_file}"
  # CAUTION: jq -r unescapes JSON \n to real newlines, so z_private_key holds a
  # multi-line PEM string. printf '%s' below preserves real newlines into the
  # RBRA file. Consumer (rbgo_OAuth.sh:zrbgo_build_jwt_capture) tolerates either
  # real-newline or '\n'-escape form via printf '%b'; do not "normalize" to one
  # form without auditing the consumer.
  {
    printf 'RBRA_ROLE=%s\n'                  "${z_role}"
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "${z_client_email}"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "${z_private_key}"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "${z_project_id}"
    printf 'RBRA_TOKEN_LIFETIME_SEC=1800\n'
  } > "${z_rbra_file}" || buc_die "Failed to write RBRA file ${z_rbra_file}"
```

`Tools/rbk/rbgp_Payor.sh:1332` (already braced — only add the comment):

Insert the same CAUTION comment block immediately above the `buc_step 'Write RBRA file'` line (or above the `{` if `buc_step` isn't immediately before it — find the heredoc-style block at line 1328-1335). Block already uses `${z_private_key}` etc., so no brace edits needed here.

**Consumer-comment fix** at `Tools/rbk/rbgo_OAuth.sh:82-83`. Current:

```bash
  # RBRA_* expected: CLIENT_EMAIL, PRIVATE_KEY, TOKEN_LIFETIME_SEC
  # RBRA_PRIVATE_KEY contains \n sequences that must become real newlines for openssl
```

Replace with:

```bash
  # RBRA_* expected: CLIENT_EMAIL, PRIVATE_KEY, TOKEN_LIFETIME_SEC
  # RBRA_PRIVATE_KEY is sourced as a multi-line string (real newlines from PEM).
  # printf '%b' below is defensive and tolerates either real-newline or '\n'-escape
  # form; PEM keys contain no backslashes so %b is otherwise a no-op.
```

## Test plan

### Tier 1 — mechanical correctness

```
bash -n Tools/rbk/rbgo_OAuth.sh
bash -n Tools/rbk/rbgp_Payor.sh
bash -n Tools/rbk/rbgu_Utility.sh
bash -n Tools/rbk/rbgg_Governor.sh
bash -n Tools/rbk/rbgc_Constants.sh
bash -n Tools/rbk/rbrp_regime.sh
bash -n Tools/buk/buh_handbook.sh

tt/rbtd-s.TestSuite.fast.sh                  # expect 93/93

shellcheck Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbgp_Payor.sh Tools/buk/buh_handbook.sh \
           Tools/rbk/rbgu_Utility.sh Tools/rbk/rbgg_Governor.sh \
           Tools/rbk/rbgc_Constants.sh Tools/rbk/rbrp_regime.sh
# expect ~110 warnings (down 9-10 from 119 baseline). NO new findings.

tt/rbw-rov.ValidateOauthRegime.sh
tt/rbw-rpv.ValidatePayorRegime.sh
```

### Tier 2 — behavior on edited surfaces

Interactive Payor install — exercises F2, F3, F5 in one ceremony:

```
ls -la ~/.rbw/payor/rbro.env 2>/dev/null    # note pre-state

tt/rbw-gPI.PayorInstall.sh <path-to-oauth-client.json>

# F5: visually confirm auth code does NOT echo when typed/pasted
# F2: immediately after ceremony, stat the file
stat -f %Lp ~/.rbw/payor/rbro.env           # macOS — expect 600
# (or stat -c %a on Linux/WSL)
# F3: ceremony completes without "Failed to execute..." OAuth URL errors
```

Subsequent Payor op — exercises F12 + F3 refresh path:

```
tt/rbw-dl.PayorListsDepots.sh
```

### Tier 3 — full credential lifecycle

Order matters; Governor reset writes the RBRA the others depend on.

```
# 3.1 Governor reset (writes via rbgp_Payor.sh:1332 — F13 site #3)
tt/rbw-aM.PayorMantlesGovernor.sh

# Inspect the resulting RBRA format (F13 reality check):
ls -la output/governor-*.rbra | tail -1
cat output/governor-*.rbra | tail -10
# Expected: multi-line PEM with real newlines inside RBRA_PRIVATE_KEY="..."

# Install per the buc_bare hint emitted by mantle:
cp output/governor-*.rbra <RBDC_GOVERNOR_RBRA_FILE>      # actual path printed at end of mantle

# 3.2 Charter Retriever (writes via rbgg_Governor.sh:264 — F13 site #2)
tt/rbw-aC.GovernorChartersRetriever.sh

# 3.3 Knight Director (writes via rbgg_Governor.sh:264 — F13 site #2 again)
tt/rbw-aK.GovernorKnightsDirector.sh

# 3.4 Verify all three SAs exist
tt/rbw-aL.GovernorListsServiceAccounts.sh

# 3.5 Consumer round-trip — proves F13 reality claim end-to-end
tt/rbw-ft.RetrieverTalliesHallmarks.sh
# Clean tally output proves: producer wrote real newlines → consumer's %b accepted them.
```

## Notch

ONE `jjx_record` at the end. Explicit file list:

- `Tools/rbk/rbgo_OAuth.sh`
- `Tools/rbk/rbgp_Payor.sh`
- `Tools/rbk/rbgc_Constants.sh`
- `Tools/rbk/rbrp_regime.sh`
- `Tools/rbk/rbgu_Utility.sh`
- `Tools/rbk/rbgg_Governor.sh`
- `Tools/buk/buh_handbook.sh`

**Never** include AAS files in the record (see Cross-officium constraints above).

Intent line for the commit: synthesize from what was actually applied. Suggested base: "OAuth-surface trust hygiene sweep (F1+F6a deletes, F6b shellcheck directives in rbgp+rbgg, F3 OAuth URL constants, F2 TOCTOU subshell, F5 buh_prompt_secret, F8-F12 comments, F13 producer-reality docs + BCG brace freebie at rbgu/rbgg writer sites + consumer-comment fix; AAY slated for BCG-wide SC2153 study)."

## Out of scope (do not expand)

- F4 (ID-token scrub generalization) — folded into F7.
- F7 (factor token-exchange paths) — separate pace, pre-AAE.
- BCG-wide SC2153 study — slated as ₢A_AAY.
- Splitting `rbgp_Payor.sh` into multiple files — F11 is a comment-only map; an actual split is a larger heat.
- Behavior changes to OAuth flow, token caching, alternate auth.
- Touching `rbgv_AccessProbe.sh` — review concluded clean.

## Corrections from the docket's prior version

This docket's prior version asserted:

1. **F11**: "OAuth ~76-566 / Depot 568-end" — wrong. `rbgp_payor_oauth_refresh` is OAuth and lives at 1123. Corrected map in F11 above.
2. **F13**: "RBRA_PRIVATE_KEY must hold '\n' escape sequences" — inverted. Reality is real newlines today; `%b` is defensive. Corrected approach in F13 above.
3. **F13 candidate writer files**: original mentioned `rbgg_Governor.sh, rbgi_IAM.sh, rgbs_ServiceAccounts.sh` as candidates. Verified: the three writers are `rbgu_Utility.sh:602`, `rbgg_Governor.sh:264`, `rbgp_Payor.sh:1332`. `rbgi_IAM.sh` and `rgbs_ServiceAccounts.sh` do NOT write RBRA.
4. **F6b extension**: original scoped to `rbgp_Payor.sh` only; the same SC2153 false positive fires in `rbgg_Governor.sh:195`. Extended as a confirmed freebie since rbgg is touched by F13 anyway.
5. **BCG brace freebie**: original did not address it; rbgu:602 and rbgg:264 use unbraced expansion in the F13 block. Brace fix folded in.

## References

- Original AAN docket (this file, prior version) — for context on review provenance.
- ₢A_AAY (`bcg-shellcheck-cross-module-discipline`) — slated to study whether F6b's per-file directives should be replaced by a BCG-mandated one-liner.
- BCG (Bash Console Guide): `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`.
- `Tools/rbk/rbgc_Constants.sh:69` — existing `RBGC_OAUTH_TOKEN_URL`, sibling to the two new mints.
- Cross-officium constraints: enforce AAS file boundary throughout.

## Verification gates before notching

- [ ] Tier 1 green: bash -n clean, fast suite 93/93, shellcheck delta ~9 fewer warnings, regime validators pass.
- [ ] Tier 2 observed: F5 no-echo confirmed, F2 perms 600 confirmed, F3 URLs round-trip.
- [ ] Tier 3 observed: Governor mantled, RBRA format inspected, retriever+director chartered, list shows all three, retriever tally rounds-trips successfully.
- [ ] Git status shows only AAN target files dirty before notch (no AAS file pollution).

If any gate fails, STOP and report. Do not adjust scope to make a failing gate pass.

**[260423-1107] rough**

## Character

Mechanical and small-diff — each item is a visible-to-first-read trust signal with a straightforward fix. No behavior change intended. Positioned before ₢A_AAE (depot regen) so the OAuth surface a fresh depot exercises reads as deliberate. F7 (factor the three near-duplicate token-exchange code paths) is OUT of scope here — separate pace, also pre-AAE.

## Docket

Surface-hygiene sweep across the OAuth-touching bash, originating from a trust-evaluation review of `rbgo_OAuth.sh`, `rbro_regime.sh`, `rbgp_Payor.sh`, `rbgv_AccessProbe.sh`, and the `buh_prompt` path. Findings numbered per the review (F1/F2/F3/F5/F6a/F6b/F8/F9/F10/F11/F12/F13). F4 folds into F7.

### F1 — Remove dead `ZRBGO_PRIVATE_KEY_FILE` constant

`Tools/rbk/rbgo_OAuth.sh:50` declares `readonly ZRBGO_PRIVATE_KEY_FILE="${BURD_TEMP_DIR}/rbgo_private_key.pem"`; grep-confirmed unreferenced anywhere in the tree. The live signing path (lines 124-127) uses process substitution and does NOT write the private key to disk. The dead constant invites "did they used to write the key to disk?" misread. Delete the line.

### F2 — Close TOCTOU on RBRO credential file write

`Tools/rbk/rbgp_Payor.sh:504-509` writes the RBRO file under default umask, then `chmod 600`. Between the `>` redirect and the chmod, the file exists with permissive perms (typically 644). Add `umask 077` in local scope before the heredoc write block (shell umask restores on function exit automatically — actually no, bash umask is not local to functions; use a subshell `( umask 077; { echo ...; echo ...; } > "${z_rbro_file}" )` OR preserve-and-restore pattern). Either is fine; the subshell approach is cleanest. The explicit `chmod 600` can stay as belt-and-suspenders.

### F3 — Extract OAuth endpoint URLs to RBGC constants

Three inlined URLs in `Tools/rbk/rbgp_Payor.sh` duplicate an existing pattern:

- `:102` and `:485`: `"https://oauth2.googleapis.com/token"` — already exists as `RBGC_OAUTH_TOKEN_URL` (used by `rbgo_OAuth.sh:146`). Replace both with the constant.
- `:443`: `"https://accounts.google.com/o/oauth2/v2/auth"` — mint `RBGC_OAUTH_AUTHORIZE_URL` in `rbgc_Constants.sh`.
- `:538`: `"https://www.googleapis.com/oauth2/v3/userinfo"` — mint `RBGC_OAUTH_USERINFO_URL`.

Per RCG Constant Discipline: 2+ occurrences extractable. Current inconsistency (rbgo uses the constant, rbgp inlines the same string) reads as "nobody's in charge of this."

### F5 — Suppress terminal echo on auth-code prompt

OAuth authorization codes appear on-screen and in terminal scrollback today because `buh_prompt` (`Tools/buk/buh_handbook.sh:231-237`) uses `read -r`. Mint `buh_prompt_secret` in `buh_handbook.sh` using `read -rs` followed by a trailing newline emitted to stderr (so the user sees the prompt progress but not the value). Callsite: `Tools/rbk/rbgp_Payor.sh:461`. Auth codes are short-lived (single-use, minutes) but are secrets redeemable for a refresh token until consumed; auditors flag terminal echo verbatim.

### F6a — Remove 7 dead `ZRBGP_INFIX_*` constants

`Tools/rbk/rbgp_Payor.sh:46-63`. Grep-confirmed unreferenced anywhere in the tree:

- Line 46: `ZRBGP_INFIX_PROJECT_DELETE`
- Line 47: `ZRBGP_INFIX_PROJECT_RESTORE`
- Line 48: `ZRBGP_INFIX_PROJECT_STATE`
- Line 53: `ZRBGP_INFIX_CREATE_REPO`
- Line 54: `ZRBGP_INFIX_VERIFY_REPO`
- Line 61: `ZRBGP_INFIX_GOV_VERIFY_SA`
- Line 63: `ZRBGP_INFIX_GOV_IAM`

Names imply planned-but-unimplemented or removed-but-not-cleaned code paths. Delete them.

### F6b — Resolve SC2153 false-positive on `ZRBGU_PREFIX`

`Tools/rbk/rbgp_Payor.sh:278` references `ZRBGU_PREFIX` (defined once in `rbgu_Utility.sh:41`, legitimately cross-module shared — 30+ references across 5 files). Shellcheck can't follow the sourcing chain, flags SC2153. Add `# shellcheck source=rbgu_Utility.sh` directive at the rbgu source line in the furnish/kindle path, OR a targeted `# shellcheck disable=SC2153` at the use site with a pointer comment. Goal: shellcheck runs clean on rbgp_Payor.sh (useful trust signal for anyone running shellcheck across the tree).

### F8 — Inline or document `rbgu_rbro_load`

`Tools/rbk/rbgu_Utility.sh:788-802` is a thin wrapper around `rbro_load` that adds only log calls. Decide the intent:

- **Inline option:** delete the wrapper, callers call `rbro_load` directly. Simplest.
- **Document option:** add a one-line comment explaining the indirection (e.g., "centralizes RBRO load through the utility module so consumer modules don't need to source `rbro_regime.sh` directly — RBTOE pattern parallels `rbgu_rbra_load`").

The existing `rbgu_rbra_load` function at line 780 appears to follow the same pattern, which suggests the wrapper is deliberate (uniform load-through-utility convention). If so, document. If not, inline.

### F9 — Comment `RBRP_OAUTH_CLIENT_ID` deliberate `min=0`

`Tools/rbk/rbrp_regime.sh:49` enrolls `RBRP_OAUTH_CLIENT_ID` with `min=0` (empty allowed). Consumer-side `test -n` checks in `rbgp_Payor.sh` are the load-bearing requirement. **Rationale surfaced during review: not every operator has a Payor identity** — some operators run Retriever-only against a depot provisioned by someone else, never needing Payor OAuth. Add a comment above the enrollment:

```
# min=0 deliberate — not every operator has a Payor identity (Retriever-only
# operators authenticate via JWT SA, never via Payor OAuth). Required-at-use
# is enforced by test -n in rbgp_Payor.sh consumers. Do not tighten to min=1.
```

Prevents a future reviewer from "tightening" and breaking non-Payor operators.

### F10 — Comment field-name-basis scrubber

`Tools/rbk/rbgo_OAuth.sh:154-156` filters OAuth response fields by key-name regex (`token|secret|key|password`) rather than value content. This is deliberate defense-in-depth, but a reader can't tell. Add a comment:

```
# Scrubber filters by field NAME, not value. Deliberate best-effort log hygiene:
# catches known secret-carrying keys (access_token, refresh_token, id_token via
# the regex, client_secret). If the OAuth provider ever returns a new
# secret-carrying field whose name doesn't match this regex, the scrub would
# miss it — update the regex here when that happens.
```

Frames the scrubber as best-effort-by-design, not comprehensive.

### F11 — Scope-tagging header comment on `rbgp_Payor.sh`

Top-of-file comment currently says "Recipe Bottle GCP Payor - Billing and Destructive Lifecycle Operations" (line 19). File is 1350 lines mixing two concerns:

- OAuth credential flow: ~lines 76-566 (install, refresh, authenticate-capture)
- Depot lifecycle: ~lines 568-end (levy, unmake, buckets, repos, worker pools, SA provisioning, IAM)

Add a brief map to the header after line 19:

```
# Scope: two concerns in this file
#   OAuth credential flow ........... ~lines 76-566 (install, refresh, authenticate-capture)
#   Depot lifecycle operations ...... ~lines 568-end (levy, unmake, bucket/repo/pool/SA provisioning)
```

Reduces first-read review surface; signals to an auditor where to focus. Does not split the file (larger scope).

### F12 — Comment no-cache intent on `zrbgp_authenticate_capture`

`Tools/rbk/rbgp_Payor.sh:122-141`. Each call re-exchanges the refresh token. For operations with many API calls (e.g., `rbgp_depot_levy` — 60+ auth sites), this means dozens of refresh exchanges per ceremony. Add a one-line comment at the top of the function:

```
# Tokens are deliberately not cached — each call refreshes. Rationale: simplicity
# and freshness; refresh tokens are long-lived so the extra roundtrips are cheap
# relative to the depot operations they authorize, and uncached tokens can't
# grow stale between distinct Payor ceremonies.
```

Sets reader expectation; answers the "is this calling Google every time?" question without requiring code reading.

### F13 — Document RBRA `\n`-escaped private-key format at the producer side

`Tools/rbk/rbgo_OAuth.sh:82-83` documents at the CONSUMER side that `RBRA_PRIVATE_KEY` holds `\n` escape sequences converted via `printf '%b'`. The producer (wherever RBRA files are written by Governor/IAM code) should carry parallel documentation so a new producer doesn't write real newlines and silently break signing. Locate the RBRA-writing site (candidates: `rbgg_Governor.sh`, `rbgi_IAM.sh`, `rgbs_ServiceAccounts.sh` — grep for `RBRA_PRIVATE_KEY=` write sites). Add a comment there referencing the format requirement, e.g.:

```
# CAUTION: RBRA_PRIVATE_KEY must hold '\n' escape sequences (literal backslash-n),
# NOT real newlines. rbgo_OAuth.sh:82-83 expects this format and converts via
# printf '%b' for openssl sign. Writing real newlines here breaks signing silently.
```

If the producer extracts the key from a Google SA JSON response (which has real newlines), the write path must escape them. Note in the comment what transform is needed.

## Not in this pace

- **F4** (ID-token scrub; scrubber shared across token-exchange paths): folds into the F7 factoring pace. Currently only `zrbgo_exchange_jwt_capture` logs the response body; other OAuth paths don't log, so no live leak today. F7 will introduce a shared helper that uses the scrubber; doing F4 twice is wasted work.
- **F7** (factor the three near-duplicate token-exchange code paths across rbgo and rbgp): separate pace, to land before ₢A_AAE.

## Verification

- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (no behavior change; regime validation should still pass)
- `shellcheck Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbgp_Payor.sh Tools/rbk/rbro_regime.sh Tools/rbk/rbro_cli.sh Tools/rbk/rbgv_AccessProbe.sh` — clean exit on rbgo and rbgp (false-positive directives documented, dead constants removed)
- `tt/rbw-rov.ValidateOauthRegime.sh` — still passes (RBRO regime untouched)
- Credentialed-station smoke (user-side if not reachable here): `tt/rbw-rpv.ValidatePayorRegime.sh` + any single Payor op (`tt/rbw-dl.PayorListsDepots.sh`) — confirms OAuth authenticate round-trip survives the URL-constant extraction
- **F5 regression check**: run `rbgp_payor_install <json>` interactively — confirm auth-code prompt does not echo the typed/pasted value
- **F2 regression check**: delete `~/.rbw/payor/rbro.env`, re-run install, immediately after the heredoc block verify `stat -f %Lp ~/.rbw/payor/rbro.env` (BSD) or `stat -c %a ~/.rbw/payor/rbro.env` (GNU/Linux/WSL) reports `600` at the moment of creation (not only post-chmod)

## Out of scope

- Factoring token-exchange paths (F7, separate pace)
- Rearchitecting the OAuth flow; caching tokens; alternate auth mechanisms
- Splitting `rbgp_Payor.sh` into separate files (F11 adds a map comment; a file split is larger scope — if it happens, separate heat)
- Changes to `rbgv_AccessProbe.sh` — review concluded it's clean; no findings there
- Anything that changes regime enrollment semantics beyond the F9 comment (F9 only documents existing behavior; does not change validation)

## Notes on ordering

Slated after ₢A_AAD (payor rbro.env subdirectory migration). AAD moves the `rbro.env` path and touches `rbhpr_refresh.sh`; this pace touches adjacent OAuth bash. Minimal file overlap between the two — mostly rbdc/rbcc constants for AAD vs rbgo/rbgp/buh for this pace — so the ordering is for logical grouping rather than dependency. F7 will slot adjacent, also before ₢A_AAE.

### bcg-shellcheck-cross-module-discipline (₢BAAAB) [complete]

**[260513-1344] complete**

Drafted from ₢A_AAY in ₣A_.

## Character

Simple-pattern discipline. BCG exists because nuanced bash craftsmanship gets forgotten across chat boundaries and corrupts. The shape of the answer here should be a uniform, easily-verifiable rule baked into BCG — not a per-script judgment call. Bias toward the simplest mechanical pattern. Only consider a more-nuanced shape if there's a concrete reason against the default.

## Docket

Cross-module readonly state (Z*_PREFIX, RB*_* constants) is shared across BCG-compliant modules via the kindle/sentinel chain. Shellcheck cannot follow runtime sourcing and fires SC2153 ("possible misspelling") on every cross-module reference. ₢A_AAN's F6b addresses this per-file in two scripts (`rbgp_Payor.sh`, `rbgg_Governor.sh`). This pace decides whether to promote the fix to a BCG-level pattern.

### Default answer

A single mandated line in BCG's prologue spec for kindled modules:

    # shellcheck disable=SC2153  # cross-module Z*_/RB*_ state via kindle chain

Placed below the copyright header, above `set -euo pipefail`. Uniform across all BCG modules. Verifiable by `grep -L 'shellcheck disable=SC2153' Tools/**/*.sh` (find files missing the directive).

### Decision criterion

Adopt the default unless one of these holds:

- A meaningful fraction of BCG modules do NOT participate in cross-module state sharing (the directive becomes dead weight in leaf modules — silent typo-detection loss for no benefit).
- A survey finds existing typo bugs that SC2153 *would* have caught (suggests the false-positive rate is acceptable and the warning carries real value).

If neither holds, apply the default. If either holds, report the finding and slate a follow-up — do not invent a middle ground.

### Work items

1. **Survey** — count BCG-compliant modules in the tree; spot-check ~10 for whether they reference cross-module readonly state. Estimate participation rate.
2. **Decide** using the criterion above. Record the outcome in one line.
3. **If applying:** amend BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`), sweep the directive across all BCG modules, remove pre-existing per-file SC2153 disables (including those landed by AAN F6b). Single notch.
4. **If not applying:** report why; slate a follow-up if a different shape is warranted.
5. **Verify** — shellcheck delta across the tree matches expectation; a deliberate scratch-script typo confirms `set -u` still catches what SC2153 used to catch.

## Why simple

The architectural alternatives (`.shellcheckrc` repo config, `# shellcheck source=...` per-reference directives) preserve more typo detection but cost more cognitive load and survive chat boundaries less well. BCG's track record favors verifiable uniformity over engineering nuance. Stay there unless evidence forces otherwise.

## Out of scope

- SC2154 disable (different shellcheck class, different tradeoff).
- `.shellcheckrc` repo-level config.
- `# shellcheck source=...` directives.
- Bringing non-BCG scripts into BCG compliance.

## References

- ₢A_AAN F6b — becomes redundant if the default is applied.
- BCG: `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`.
- CLAUDE.md "Load-Bearing Complexity" — uniformity is what's load-bearing here, not technical sophistication.

**[260426-0622] rough**

Drafted from ₢A_AAY in ₣A_.

## Character

Simple-pattern discipline. BCG exists because nuanced bash craftsmanship gets forgotten across chat boundaries and corrupts. The shape of the answer here should be a uniform, easily-verifiable rule baked into BCG — not a per-script judgment call. Bias toward the simplest mechanical pattern. Only consider a more-nuanced shape if there's a concrete reason against the default.

## Docket

Cross-module readonly state (Z*_PREFIX, RB*_* constants) is shared across BCG-compliant modules via the kindle/sentinel chain. Shellcheck cannot follow runtime sourcing and fires SC2153 ("possible misspelling") on every cross-module reference. ₢A_AAN's F6b addresses this per-file in two scripts (`rbgp_Payor.sh`, `rbgg_Governor.sh`). This pace decides whether to promote the fix to a BCG-level pattern.

### Default answer

A single mandated line in BCG's prologue spec for kindled modules:

    # shellcheck disable=SC2153  # cross-module Z*_/RB*_ state via kindle chain

Placed below the copyright header, above `set -euo pipefail`. Uniform across all BCG modules. Verifiable by `grep -L 'shellcheck disable=SC2153' Tools/**/*.sh` (find files missing the directive).

### Decision criterion

Adopt the default unless one of these holds:

- A meaningful fraction of BCG modules do NOT participate in cross-module state sharing (the directive becomes dead weight in leaf modules — silent typo-detection loss for no benefit).
- A survey finds existing typo bugs that SC2153 *would* have caught (suggests the false-positive rate is acceptable and the warning carries real value).

If neither holds, apply the default. If either holds, report the finding and slate a follow-up — do not invent a middle ground.

### Work items

1. **Survey** — count BCG-compliant modules in the tree; spot-check ~10 for whether they reference cross-module readonly state. Estimate participation rate.
2. **Decide** using the criterion above. Record the outcome in one line.
3. **If applying:** amend BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`), sweep the directive across all BCG modules, remove pre-existing per-file SC2153 disables (including those landed by AAN F6b). Single notch.
4. **If not applying:** report why; slate a follow-up if a different shape is warranted.
5. **Verify** — shellcheck delta across the tree matches expectation; a deliberate scratch-script typo confirms `set -u` still catches what SC2153 used to catch.

## Why simple

The architectural alternatives (`.shellcheckrc` repo config, `# shellcheck source=...` per-reference directives) preserve more typo detection but cost more cognitive load and survive chat boundaries less well. BCG's track record favors verifiable uniformity over engineering nuance. Stay there unless evidence forces otherwise.

## Out of scope

- SC2154 disable (different shellcheck class, different tradeoff).
- `.shellcheckrc` repo-level config.
- `# shellcheck source=...` directives.
- Bringing non-BCG scripts into BCG compliance.

## References

- ₢A_AAN F6b — becomes redundant if the default is applied.
- BCG: `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`.
- CLAUDE.md "Load-Bearing Complexity" — uniformity is what's load-bearing here, not technical sophistication.

**[260425-0940] rough**

## Character

Simple-pattern discipline. BCG exists because nuanced bash craftsmanship gets forgotten across chat boundaries and corrupts. The shape of the answer here should be a uniform, easily-verifiable rule baked into BCG — not a per-script judgment call. Bias toward the simplest mechanical pattern. Only consider a more-nuanced shape if there's a concrete reason against the default.

## Docket

Cross-module readonly state (Z*_PREFIX, RB*_* constants) is shared across BCG-compliant modules via the kindle/sentinel chain. Shellcheck cannot follow runtime sourcing and fires SC2153 ("possible misspelling") on every cross-module reference. ₢A_AAN's F6b addresses this per-file in two scripts (`rbgp_Payor.sh`, `rbgg_Governor.sh`). This pace decides whether to promote the fix to a BCG-level pattern.

### Default answer

A single mandated line in BCG's prologue spec for kindled modules:

    # shellcheck disable=SC2153  # cross-module Z*_/RB*_ state via kindle chain

Placed below the copyright header, above `set -euo pipefail`. Uniform across all BCG modules. Verifiable by `grep -L 'shellcheck disable=SC2153' Tools/**/*.sh` (find files missing the directive).

### Decision criterion

Adopt the default unless one of these holds:

- A meaningful fraction of BCG modules do NOT participate in cross-module state sharing (the directive becomes dead weight in leaf modules — silent typo-detection loss for no benefit).
- A survey finds existing typo bugs that SC2153 *would* have caught (suggests the false-positive rate is acceptable and the warning carries real value).

If neither holds, apply the default. If either holds, report the finding and slate a follow-up — do not invent a middle ground.

### Work items

1. **Survey** — count BCG-compliant modules in the tree; spot-check ~10 for whether they reference cross-module readonly state. Estimate participation rate.
2. **Decide** using the criterion above. Record the outcome in one line.
3. **If applying:** amend BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`), sweep the directive across all BCG modules, remove pre-existing per-file SC2153 disables (including those landed by AAN F6b). Single notch.
4. **If not applying:** report why; slate a follow-up if a different shape is warranted.
5. **Verify** — shellcheck delta across the tree matches expectation; a deliberate scratch-script typo confirms `set -u` still catches what SC2153 used to catch.

## Why simple

The architectural alternatives (`.shellcheckrc` repo config, `# shellcheck source=...` per-reference directives) preserve more typo detection but cost more cognitive load and survive chat boundaries less well. BCG's track record favors verifiable uniformity over engineering nuance. Stay there unless evidence forces otherwise.

## Out of scope

- SC2154 disable (different shellcheck class, different tradeoff).
- `.shellcheckrc` repo-level config.
- `# shellcheck source=...` directives.
- Bringing non-BCG scripts into BCG compliance.

## References

- ₢A_AAN F6b — becomes redundant if the default is applied.
- BCG: `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`.
- CLAUDE.md "Load-Bearing Complexity" — uniformity is what's load-bearing here, not technical sophistication.

### oci-magic-string-extraction (₢BAAAC) [complete]

**[260513-1359] complete**

Drafted from ₢A_AAM in ₣A_.

## Character

Mechanical — small-scope extraction of OCI Distribution Spec protocol keywords (`manifests`, `blobs`, `tags/list`, `v2`) that currently appear as bare literals in URL construction across rbfr and rbfv. Low per-site judgment once the naming scheme is chosen; the design thought lies in choosing between named-constant replacement vs a URL-builder helper.

## Docket

The OCI Registry HTTP API v2 URL shape `/v2/<name>/manifests/<reference>` (and peers `blobs/`, `tags/list`) appears ~10× across `rbfr_FoundryRetriever.sh` and `rbfv_FoundryVerify.sh` with the endpoint keywords inlined as bare string literals. These are protocol-defined keywords from the OCI Distribution Spec — frozen, not arbitrary — but repeated inlining violates BCG's Load-Bearing Complexity principle. See BCG:609 `Z«PREFIX»_API_VERSION="v2"` as the canonical pattern for capturing protocol keywords.

### Two options (pick one at pace execution)

**Option A — named protocol constants as RBFC tinder**. Add `ZRBFC_ENDPOINT_MANIFESTS="manifests"`, `ZRBFC_ENDPOINT_BLOBS="blobs"`, `ZRBFC_ENDPOINT_TAGS_LIST="tags/list"`, `ZRBFC_API_VERSION="v2"` at RBFC tinder-constant position (post-`SOURCED`, pre-kindle). Each site replaces the bare literal with the named constant. Minimal structural change, most direct expression of BCG's tinder-constant pattern.

**Option B — URL-builder helper**. Add `zrbfc_manifest_url_capture <vessel_path> <reference>` returning `${ZRBFC_REGISTRY_API_BASE}/<vessel_path>/manifests/<reference>`. Callers collapse to one line. Higher upfront effort, higher payoff if future OCI endpoints get added. Tilts toward "one place owns the URL shape."

### Sites (verify-on-touch; line numbers will drift)

- `rbfr_FoundryRetriever.sh` — `/tags/list` (~142), `/manifests/` (~188, 213, 238)
- `rbfv_FoundryVerify.sh` — `/manifests/` (~89, 142, 166, 224, 617, 641), `/tags/list` (~822)
- Any other hits from `grep -rnE '/manifests/|/tags/list|/blobs/|/v2/' Tools/rbk/`

### Verification

- Fast suite green (no new test surface for this refactor)
- Crucible suite green (exercises actual registry fetches end-to-end)
- Manual smoke: `tt/rbw-cC.Charge.tadmor.sh` succeeds unchanged

### Scope guardrails

- Pure extraction — no URL semantics change, no new endpoint types
- Independent of AAK: OCI protocol keywords persist regardless of whether path grammar reshapes into hallmark-centered layout
- Does not block or get blocked by depot regen (AAE)

**[260426-0622] rough**

Drafted from ₢A_AAM in ₣A_.

## Character

Mechanical — small-scope extraction of OCI Distribution Spec protocol keywords (`manifests`, `blobs`, `tags/list`, `v2`) that currently appear as bare literals in URL construction across rbfr and rbfv. Low per-site judgment once the naming scheme is chosen; the design thought lies in choosing between named-constant replacement vs a URL-builder helper.

## Docket

The OCI Registry HTTP API v2 URL shape `/v2/<name>/manifests/<reference>` (and peers `blobs/`, `tags/list`) appears ~10× across `rbfr_FoundryRetriever.sh` and `rbfv_FoundryVerify.sh` with the endpoint keywords inlined as bare string literals. These are protocol-defined keywords from the OCI Distribution Spec — frozen, not arbitrary — but repeated inlining violates BCG's Load-Bearing Complexity principle. See BCG:609 `Z«PREFIX»_API_VERSION="v2"` as the canonical pattern for capturing protocol keywords.

### Two options (pick one at pace execution)

**Option A — named protocol constants as RBFC tinder**. Add `ZRBFC_ENDPOINT_MANIFESTS="manifests"`, `ZRBFC_ENDPOINT_BLOBS="blobs"`, `ZRBFC_ENDPOINT_TAGS_LIST="tags/list"`, `ZRBFC_API_VERSION="v2"` at RBFC tinder-constant position (post-`SOURCED`, pre-kindle). Each site replaces the bare literal with the named constant. Minimal structural change, most direct expression of BCG's tinder-constant pattern.

**Option B — URL-builder helper**. Add `zrbfc_manifest_url_capture <vessel_path> <reference>` returning `${ZRBFC_REGISTRY_API_BASE}/<vessel_path>/manifests/<reference>`. Callers collapse to one line. Higher upfront effort, higher payoff if future OCI endpoints get added. Tilts toward "one place owns the URL shape."

### Sites (verify-on-touch; line numbers will drift)

- `rbfr_FoundryRetriever.sh` — `/tags/list` (~142), `/manifests/` (~188, 213, 238)
- `rbfv_FoundryVerify.sh` — `/manifests/` (~89, 142, 166, 224, 617, 641), `/tags/list` (~822)
- Any other hits from `grep -rnE '/manifests/|/tags/list|/blobs/|/v2/' Tools/rbk/`

### Verification

- Fast suite green (no new test surface for this refactor)
- Crucible suite green (exercises actual registry fetches end-to-end)
- Manual smoke: `tt/rbw-cC.Charge.tadmor.sh` succeeds unchanged

### Scope guardrails

- Pure extraction — no URL semantics change, no new endpoint types
- Independent of AAK: OCI protocol keywords persist regardless of whether path grammar reshapes into hallmark-centered layout
- Does not block or get blocked by depot regen (AAE)

**[260423-0947] rough**

## Character

Mechanical — small-scope extraction of OCI Distribution Spec protocol keywords (`manifests`, `blobs`, `tags/list`, `v2`) that currently appear as bare literals in URL construction across rbfr and rbfv. Low per-site judgment once the naming scheme is chosen; the design thought lies in choosing between named-constant replacement vs a URL-builder helper.

## Docket

The OCI Registry HTTP API v2 URL shape `/v2/<name>/manifests/<reference>` (and peers `blobs/`, `tags/list`) appears ~10× across `rbfr_FoundryRetriever.sh` and `rbfv_FoundryVerify.sh` with the endpoint keywords inlined as bare string literals. These are protocol-defined keywords from the OCI Distribution Spec — frozen, not arbitrary — but repeated inlining violates BCG's Load-Bearing Complexity principle. See BCG:609 `Z«PREFIX»_API_VERSION="v2"` as the canonical pattern for capturing protocol keywords.

### Two options (pick one at pace execution)

**Option A — named protocol constants as RBFC tinder**. Add `ZRBFC_ENDPOINT_MANIFESTS="manifests"`, `ZRBFC_ENDPOINT_BLOBS="blobs"`, `ZRBFC_ENDPOINT_TAGS_LIST="tags/list"`, `ZRBFC_API_VERSION="v2"` at RBFC tinder-constant position (post-`SOURCED`, pre-kindle). Each site replaces the bare literal with the named constant. Minimal structural change, most direct expression of BCG's tinder-constant pattern.

**Option B — URL-builder helper**. Add `zrbfc_manifest_url_capture <vessel_path> <reference>` returning `${ZRBFC_REGISTRY_API_BASE}/<vessel_path>/manifests/<reference>`. Callers collapse to one line. Higher upfront effort, higher payoff if future OCI endpoints get added. Tilts toward "one place owns the URL shape."

### Sites (verify-on-touch; line numbers will drift)

- `rbfr_FoundryRetriever.sh` — `/tags/list` (~142), `/manifests/` (~188, 213, 238)
- `rbfv_FoundryVerify.sh` — `/manifests/` (~89, 142, 166, 224, 617, 641), `/tags/list` (~822)
- Any other hits from `grep -rnE '/manifests/|/tags/list|/blobs/|/v2/' Tools/rbk/`

### Verification

- Fast suite green (no new test surface for this refactor)
- Crucible suite green (exercises actual registry fetches end-to-end)
- Manual smoke: `tt/rbw-cC.Charge.tadmor.sh` succeeds unchanged

### Scope guardrails

- Pure extraction — no URL semantics change, no new endpoint types
- Independent of AAK: OCI protocol keywords persist regardless of whether path grammar reshapes into hallmark-centered layout
- Does not block or get blocked by depot regen (AAE)

### kludge-aware-charge-prereq (₢BAAAH) [complete]

**[260512-1758] complete**

## Character

Mint a small constant family, swap four bare literals to use it, and add one
discriminator helper. Mechanical with one micro-design choice (the helper's
name and signature). The judgment work is recognizing that the BCG magic-string
issue here is broader than just `k` — there is a four-letter undeclared
provenance vocabulary (`c`/`k`/`b`/`g`) that wants symbolic treatment as a
group.

## Docket

The hallmark prefix letter (`c`=conjure, `k`=kludge, `b`=bind, `g`=graft)
encodes artifact provenance and lives as a bare literal at four production
sites. Charge-time prerequisite checks in `rbob_bottle.sh` cannot discriminate
local-only kludge hallmarks from depot-resident ones, so the diagnostic when
state is missing always suggests `tt/rbw-fs.RetrieverSummonsHallmark.sh` —
which is guaranteed to fail for `k`-prefixed hallmarks because they never go
to GAR.

Surfaced during ₢A_AAU verification: a stale tadmor sentry hallmark
(`k260416190047-...`) in `.rbk/tadmor/rbrn.env` produced "vouch artifact
missing locally" with the unhelpful "run summon" suggestion. The actual fix
was `tt/rbw-cKS.KludgeSentry.sh tadmor`. The diagnostic should have said so.

### Pattern

Symbolic constants for the four mode prefixes; pattern-match against
constants at the discrimination site.

### Sites

**Mint** — `Tools/rbk/rbgc_Constants.sh`, parallel to the existing
`RBGC_ARK_BASENAME_*` block:

```
readonly RBGC_HALLMARK_PREFIX_CONJURE="c"
readonly RBGC_HALLMARK_PREFIX_KLUDGE="k"
readonly RBGC_HALLMARK_PREFIX_BIND="b"
readonly RBGC_HALLMARK_PREFIX_GRAFT="g"
```

**Production sites in `Tools/rbk/rbfd_FoundryDirectorBuild.sh`** —
replace bare literals with constant references:

- `:1137` — `"c${BURD_NOW_STAMP...}"` → `"${RBGC_HALLMARK_PREFIX_CONJURE}${BURD_NOW_STAMP...}"`
- `:1293` — `"k${BURD_NOW_STAMP...}"` → `"${RBGC_HALLMARK_PREFIX_KLUDGE}${BURD_NOW_STAMP...}"`
- `:1391` — `"b${BURD_NOW_STAMP...}"` → `"${RBGC_HALLMARK_PREFIX_BIND}${BURD_NOW_STAMP...}"`
- `:1648` — `"g${z_cdate...}"` → `"${RBGC_HALLMARK_PREFIX_GRAFT}${z_cdate...}"`

The existing inline comment at `:1290` ("k prefix distinguishes from
conjure c, bind b") should drop — the constant names carry the meaning.

**Discriminator + diagnostic in `Tools/rbk/rbob_bottle.sh`** —
add helper and apply at the prerequisite checks:

```
zrbob_hallmark_is_kludge() {
  case "${1}" in
    "${RBGC_HALLMARK_PREFIX_KLUDGE}"*) return 0 ;;
    *) return 1 ;;
  esac
}
```

Update the missing-vouch and missing-image paths around `:307` and `:320`
(both for sentry and for bottle — four sites total) so the diagnostic
branches:

- Kludge hallmark missing locally → `"Kludge hallmark not built locally — run: tt/rbw-cK{S,B}.Kludge{Sentry,Bottle}.sh ${RBRN_MONIKER}"`. Do NOT call `zrbob_vouch_gate_and_summon` (guaranteed to fail).
- Non-kludge hallmark missing locally → existing summon path unchanged.

### Verification

- `tt/rbtd-s.TestSuite.fast.sh` — green (no behavior change to fast paths)
- Manual: edit `.rbk/tadmor/rbrn.env` to a fake `k000000000000-deadbeef`
  hallmark, run `tt/rbw-cC.Charge.tadmor.sh`. Should produce a clear
  "kludge required" diagnostic naming the right tabtarget. Restore stamp
  after.
- Manual: with a real-but-not-locally-summoned non-kludge hallmark (any
  conjure stamp from depot), confirm the summon path still triggers
  unchanged.
- `tt/rbtd-s.TestSuite.crucible.sh` — green for tadmor (the fixture this
  defect was originally surfaced under).

### Out of scope

- **Fixture-level bootstrap** ("ensure local state before tests run") —
  the larger architectural conversation about test-rig vs. system-under-test
  separation. Lives in its own pace or grooming session, not here.
- **Theurge runner reporting** — making "Charge failed (exit 1)" itself
  more diagnostic at the runner level. Separate concern; this pace fixes
  the upstream warning that prints before the failure.
- **Cross-checking depot for non-kludge hallmark presence** before
  attempting summon — tempting but already covered by the existing
  `zrbob_vouch_gate_and_summon` path which surfaces a sensible error if
  the artifact isn't in GAR.
- **Hallmark stamp coherence between `.rbk/<nameplate>/rbrn.env` and
  reachable state** — the deeper "should committed stamps be reproducibility
  anchors or convenience defaults" question. Defer.
- **Other consumers of the prefix letter** beyond the four production
  sites in `rbfd_FoundryDirectorBuild.sh` — none found in initial search;
  if any surface during the swap, fold into this pace, otherwise leave for
  follow-up.

### Cadence

Mint constants, refactor the four production sites, add helper, update the
four diagnostic sites. Run fast suite. Notch once when the manual verification
above passes against tadmor.

### Discovery context

Surfaced during ₣A_ ₢A_AAU verification (260426). Original blocker: stale
sentry hallmark `k260416190047-e29568f4` in `.rbk/tadmor/rbrn.env`, fixed by
hand-kludging. Conversation noted that the existing diagnostic actively
misled — suggesting summon for a hallmark class that never goes to depot.
Slotted into ₣BA (`rbk-mvp-3-cleanups-and-imageops`) as squarely image-lifecycle
work; natural neighbor is `imageops-fixtures` ₢BAAAF.

**[260426-0956] rough**

## Character

Mint a small constant family, swap four bare literals to use it, and add one
discriminator helper. Mechanical with one micro-design choice (the helper's
name and signature). The judgment work is recognizing that the BCG magic-string
issue here is broader than just `k` — there is a four-letter undeclared
provenance vocabulary (`c`/`k`/`b`/`g`) that wants symbolic treatment as a
group.

## Docket

The hallmark prefix letter (`c`=conjure, `k`=kludge, `b`=bind, `g`=graft)
encodes artifact provenance and lives as a bare literal at four production
sites. Charge-time prerequisite checks in `rbob_bottle.sh` cannot discriminate
local-only kludge hallmarks from depot-resident ones, so the diagnostic when
state is missing always suggests `tt/rbw-fs.RetrieverSummonsHallmark.sh` —
which is guaranteed to fail for `k`-prefixed hallmarks because they never go
to GAR.

Surfaced during ₢A_AAU verification: a stale tadmor sentry hallmark
(`k260416190047-...`) in `.rbk/tadmor/rbrn.env` produced "vouch artifact
missing locally" with the unhelpful "run summon" suggestion. The actual fix
was `tt/rbw-cKS.KludgeSentry.sh tadmor`. The diagnostic should have said so.

### Pattern

Symbolic constants for the four mode prefixes; pattern-match against
constants at the discrimination site.

### Sites

**Mint** — `Tools/rbk/rbgc_Constants.sh`, parallel to the existing
`RBGC_ARK_BASENAME_*` block:

```
readonly RBGC_HALLMARK_PREFIX_CONJURE="c"
readonly RBGC_HALLMARK_PREFIX_KLUDGE="k"
readonly RBGC_HALLMARK_PREFIX_BIND="b"
readonly RBGC_HALLMARK_PREFIX_GRAFT="g"
```

**Production sites in `Tools/rbk/rbfd_FoundryDirectorBuild.sh`** —
replace bare literals with constant references:

- `:1137` — `"c${BURD_NOW_STAMP...}"` → `"${RBGC_HALLMARK_PREFIX_CONJURE}${BURD_NOW_STAMP...}"`
- `:1293` — `"k${BURD_NOW_STAMP...}"` → `"${RBGC_HALLMARK_PREFIX_KLUDGE}${BURD_NOW_STAMP...}"`
- `:1391` — `"b${BURD_NOW_STAMP...}"` → `"${RBGC_HALLMARK_PREFIX_BIND}${BURD_NOW_STAMP...}"`
- `:1648` — `"g${z_cdate...}"` → `"${RBGC_HALLMARK_PREFIX_GRAFT}${z_cdate...}"`

The existing inline comment at `:1290` ("k prefix distinguishes from
conjure c, bind b") should drop — the constant names carry the meaning.

**Discriminator + diagnostic in `Tools/rbk/rbob_bottle.sh`** —
add helper and apply at the prerequisite checks:

```
zrbob_hallmark_is_kludge() {
  case "${1}" in
    "${RBGC_HALLMARK_PREFIX_KLUDGE}"*) return 0 ;;
    *) return 1 ;;
  esac
}
```

Update the missing-vouch and missing-image paths around `:307` and `:320`
(both for sentry and for bottle — four sites total) so the diagnostic
branches:

- Kludge hallmark missing locally → `"Kludge hallmark not built locally — run: tt/rbw-cK{S,B}.Kludge{Sentry,Bottle}.sh ${RBRN_MONIKER}"`. Do NOT call `zrbob_vouch_gate_and_summon` (guaranteed to fail).
- Non-kludge hallmark missing locally → existing summon path unchanged.

### Verification

- `tt/rbtd-s.TestSuite.fast.sh` — green (no behavior change to fast paths)
- Manual: edit `.rbk/tadmor/rbrn.env` to a fake `k000000000000-deadbeef`
  hallmark, run `tt/rbw-cC.Charge.tadmor.sh`. Should produce a clear
  "kludge required" diagnostic naming the right tabtarget. Restore stamp
  after.
- Manual: with a real-but-not-locally-summoned non-kludge hallmark (any
  conjure stamp from depot), confirm the summon path still triggers
  unchanged.
- `tt/rbtd-s.TestSuite.crucible.sh` — green for tadmor (the fixture this
  defect was originally surfaced under).

### Out of scope

- **Fixture-level bootstrap** ("ensure local state before tests run") —
  the larger architectural conversation about test-rig vs. system-under-test
  separation. Lives in its own pace or grooming session, not here.
- **Theurge runner reporting** — making "Charge failed (exit 1)" itself
  more diagnostic at the runner level. Separate concern; this pace fixes
  the upstream warning that prints before the failure.
- **Cross-checking depot for non-kludge hallmark presence** before
  attempting summon — tempting but already covered by the existing
  `zrbob_vouch_gate_and_summon` path which surfaces a sensible error if
  the artifact isn't in GAR.
- **Hallmark stamp coherence between `.rbk/<nameplate>/rbrn.env` and
  reachable state** — the deeper "should committed stamps be reproducibility
  anchors or convenience defaults" question. Defer.
- **Other consumers of the prefix letter** beyond the four production
  sites in `rbfd_FoundryDirectorBuild.sh` — none found in initial search;
  if any surface during the swap, fold into this pace, otherwise leave for
  follow-up.

### Cadence

Mint constants, refactor the four production sites, add helper, update the
four diagnostic sites. Run fast suite. Notch once when the manual verification
above passes against tadmor.

### Discovery context

Surfaced during ₣A_ ₢A_AAU verification (260426). Original blocker: stale
sentry hallmark `k260416190047-e29568f4` in `.rbk/tadmor/rbrn.env`, fixed by
hand-kludging. Conversation noted that the existing diagnostic actively
misled — suggesting summon for a hallmark class that never goes to depot.
Slotted into ₣BA (`rbk-mvp-3-cleanups-and-imageops`) as squarely image-lifecycle
work; natural neighbor is `imageops-fixtures` ₢BAAAF.

### kludge-honors-enshrine-anchors (₢BAAAI) [complete]

**[260513-1413] complete**

## Character

Targeted feature addition on the consumer side of enshrine. Closes a silent
semantic divergence between kludge and conjure: today an enshrined vessel
produces different base layers under the two paths. Small-diff, high-confidence
— the resolution code already exists in conjure (rbfd_FoundryDirectorBuild.sh:381+392)
and just needs DRY extraction plus a local-presence guard. The credential
boundary stays exactly where it is: kludge remains uncredentialed; wrest
remains the credentialed step.

## Problem

`rbfd_kludge` (rbfd_FoundryDirectorBuild.sh:1268-1278) reads only
`RBRV_IMAGE_n_ORIGIN` and ignores `RBRV_IMAGE_n_ANCHOR`. Conjure (line 381+392)
consumes both. Result: an enshrined vessel built via kludge uses upstream base
layers; the same vessel built via conjure uses GAR-anchored base layers. A
developer iterating with kludge believes they are testing the same base image
as conjure — they are not.

The clean path closes the gap without making kludge credentialed. Kludge
resolves the anchor to a full GAR ref, then performs a local-only presence
check (`docker image inspect`) and fails with a remediation pointing at
`rbw-iwe.DirectorWrestsEnshrinedImage.sh`. The credentialed pull happens in
wrest, which already exists. Same pattern applies to origin slots: missing
local cache → `docker pull «origin»` remediation. The `--pull=never` flag on
docker build enforces the no-network contract uniformly.

## Docket

### Approach: kindle-time constant + uniform local-presence check

Three coordinated changes:

1. **Kindle one composite constant** in `rbfc_FoundryCore.sh` after the rbgl
   dependency is satisfied:
   ```
   readonly ZRBFC_ENSHRINES_BASE="${ZRBFC_REGISTRY_HOST}/${ZRBFC_REGISTRY_PATH}/${RBGL_ENSHRINES_ROOT}"
   ```
   Verify the kindle order — conjure already references `RBGL_ENSHRINES_ROOT`
   from rbfc-level code (rbfd line 392), so the dependency exists in practice.
   If not formalized in `zrbfc_kindle`, add `zrbgl_kindle` call.

2. **DRY conjure** — in `zrbfd_stitch_build_json`, replace the recomputed
   `z_gar_repo_base` (line 381, currently
   `${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}`)
   with `ZRBFC_ENSHRINES_BASE` substitution at line 392. Slot resolution
   becomes `"${ZRBFC_ENSHRINES_BASE}/${z_anchor}:${z_anchor}"`. Behavior must
   be byte-equivalent — this is a refactor, not a change.

3. **Rewrite kludge slot loop** (rbfd_FoundryDirectorBuild.sh:1268-1278) to
   mirror conjure's anchor-aware resolution and add a local-presence guard:
   - For each slot, read both ORIGIN and ANCHOR
   - ANCHOR set with empty ORIGIN → `buc_die` (malformed regime — defensive)
   - ANCHOR set → resolve via `${ZRBFC_ENSHRINES_BASE}/${anchor}:${anchor}`
   - ANCHOR empty → pass ORIGIN through verbatim
   - Run `docker image inspect REF >/dev/null 2>&1` on every resolved ref
     (local daemon query — no network, no credentials)
   - Accumulate misses with type-specific remediation:
     - Anchored miss → emit tabtarget pointer to
       `rbw-iwe.DirectorWrestsEnshrinedImage.sh enshrines/«anchor»:«anchor»`
     - Origin miss → emit `docker pull «origin»` instruction
   - On any miss: print full remediation block (use existing
     `buc_warn`/`buc_tabtarget`/`buc_die` pattern from
     `zrbfd_registry_preflight` lines 336-345) and abort
   - Add `--pull=never` to the `docker build` invocation (line 1295)
     unconditionally

### Spec touch

- **`Tools/rbk/vov_veiled/RBSAK-ark_kludge.adoc`** — replace step 2 with
  resolve+verify language. Add new step "Verify Local Image Presence" between
  resolve and assign-hallmark. Amend build step to mention `--pull=never`. Add
  Limitations bullet:
  ```
  * All declared base images must be locally cached before kludge runs:
    anchored slots via {rbtgo_image_wrest_enshrined}, origin slots via plain
    `docker pull`. Kludge never reaches a registry.
  ```
  Drafted wording is in the working chat that produced this pace; mounting
  agent should refine for AsciiDoc/MCM conformance.

- **`Tools/rbk/vov_veiled/RBSAE-ark_enshrine.adoc`** — optional one-line
  consumer note that {rbrv_image_anchor} is read by both {rbtgo_ark_conjure}
  and {rbtgo_ark_kludge}. Defer if it makes the producer spec noisy.

### Surface

- `Tools/rbk/rbfc_FoundryCore.sh` — one kindled constant added; verify rbgl
  kindle precedes (add the call if not present)
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh` — `zrbfd_stitch_build_json`
  (line 381+392, DRY refactor) and `rbfd_kludge` (line 1268-1295, slot loop
  rewrite + presence check + `--pull=never`)
- `Tools/rbk/vov_veiled/RBSAK-ark_kludge.adoc` — step 2 rewrite, new step,
  build step amendment, Limitations bullet
- `Tools/rbk/vov_veiled/RBSAE-ark_enshrine.adoc` — optional consumer note

No tabtarget surface change. No new prefixes. No new modules.

### Verification

1. **Kludge unchanged for origin-only, pre-cached vessels** — pick a vessel
   with no anchors, base images already in `docker images`, kludge succeeds
   exactly as today.
2. **Kludge fails clearly when origin not cached** — same vessel, fresh
   docker cache, kludge fails with `docker pull «origin»` remediation; run
   the pull; kludge succeeds.
3. **Kludge fails clearly when anchor not cached** — vessel with anchors,
   fresh cache, kludge fails with `rbw-iwe enshrines/«anchor»:«anchor»`
   remediation; run wrest; kludge succeeds.
4. **Anchored kludge produces same base layer as conjure** — for the same
   anchored vessel, diff `docker inspect --format='{{.RootFS.Layers}}'` of
   the kludge build and the conjure build's pulled image. Base-layer digests
   identical confirms the gap is closed.
5. **Malformed regime caught** — fabricate a vessel rbrv.env with ANCHOR
   set and ORIGIN empty; kludge dies with explicit "malformed regime"
   message before docker is invoked.
6. **`--pull=never` enforcement** — confirm `docker build --pull=never`
   surfaces hardcoded FROM stages not parameterized through `RBF_IMAGE_n`.
   This is a vessel hygiene side benefit, not a regression — but if any
   existing vessel in the repo has such a FROM, document it as a
   spook-or-itch for follow-up rather than blocking this pace.
7. **kludge_lifecycle SingleCase GREEN** —
   `tt/rbtd-s.SingleCase.kludge-lifecycle.sh` (or whatever the canonical
   single-case name is in the fundus registry).
8. **crucible suite GREEN** — `tt/rbtd-s.TestSuite.crucible.sh`. Watch for
   onboarding-flow scripts (`rbw-Ofc`, `rbw-Ots`, `rbw-Oda`) that may need
   to pre-pull/pre-wrest before the kludge step. If a flow breaks because
   it assumed auto-pull, the fix is one new step in the flow, not a
   weakening of this pace's contract.
9. **fast suite GREEN** — `tt/rbtd-s.TestSuite.fast.sh`. The kindled
   constant addition should be invisible to non-foundry tests.

### Out of scope

- **Conjure behavioral change.** The DRY refactor must be byte-equivalent.
  Conjure resolve-loop output identical pre/post.
- **Wrest API change.** `rbfl_wrest`'s general locator signature
  (`category/path:tag` with cloud prefix prepended internally) stays. Three
  categories (hallmarks/reliquaries/enshrines) under one signature is the
  right abstraction for wrest.
- **Auto-wrest from kludge.** Tempting but wrong — that re-introduces
  credentials into kludge. Hard line: kludge never authenticates.
- **Auto-pull from kludge for origin slots.** Same principle — `--pull=never`
  is unconditional. Operator pre-pulls.
- **Conditional `--pull=never` based on anchor presence.** Considered and
  rejected in working chat — uniform contract is simpler and the
  origin-pre-pull cost is acceptable given the clear remediation message.
- **Onboarding script refactor.** Existing flows may need a one-line note
  ("first-run kludge requires pre-pulled origin images"); leave to a
  follow-up if the cold-cache pass surfaces real friction.

### Hard dependency notes

- Independent of BA's other paces. Disjoint files from BAAAG (jettison
  cascade): BAAAG is `rbfl_jettison`/`rbfc_FoundryCore.sh:575`; this pace
  is `rbfd_kludge`/`rbfc_FoundryCore.sh` kindle section.
- Builds on BAAAD's `rbw-iwe.DirectorWrestsEnshrinedImage.sh` mint (already
  landed). The remediation message references that tabtarget by exact name.

### Cadence

1. Mount BA, parade this pace, read this docket.
2. Read in order before any edit:
   - `rbfc_FoundryCore.sh` kindle section — find right insertion point,
     verify or add rbgl kindle precedence
   - `rbfd_FoundryDirectorBuild.sh:381+392` — conjure DRY target
   - `rbfd_FoundryDirectorBuild.sh:1268-1295` — kludge target
   - `rbfd_FoundryDirectorBuild.sh:336-345` — existing
     `buc_warn`/`buc_tabtarget`/`buc_die` remediation pattern to mirror
   - `rbfl_wrest` (rbfl_FoundryLedger.sh:733-779) — locator format reminder
3. Apply changes in this order:
   - Kindled constant → conjure DRY (verify behavior unchanged with a
     manual diff or quick conjure run) → kludge slot loop rewrite →
     presence-check block → `--pull=never` → spec rewrite
4. Single notch when:
   - bash -n / shellcheck clean on touched files
   - kludge_lifecycle SingleCase GREEN
   - manual cold-cache test of an anchored vessel passes (remediation →
     wrest → build)
   - manual cold-cache test of an origin-only vessel passes (remediation →
     docker pull → build)
   - crucible suite GREEN
   - fast suite GREEN
5. Wrap.

### References

- `Tools/rbk/rbfc_FoundryCore.sh:51-53` (`ZRBFC_REGISTRY_HOST` /
  `ZRBFC_REGISTRY_PATH` — composition pattern)
- `Tools/rbk/rbgl_GarLayout.sh:50` (`RBGL_ENSHRINES_ROOT` — already includes
  `RBRR_CLOUD_PREFIX`)
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh:381+392` (conjure's anchor
  resolution — DRY target)
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh:1234-1320` (`rbfd_kludge` — target)
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh:336-345` (preflight remediation
  pattern to mirror)
- `Tools/rbk/rbfl_FoundryLedger.sh:733-779` (`rbfl_wrest` — locator format
  `enshrines/«anchor»:«anchor»` for remediation message)
- `Tools/rbk/vov_veiled/RBSAK-ark_kludge.adoc` (spec to rewrite)
- `Tools/rbk/vov_veiled/RBSAE-ark_enshrine.adoc` (producer-side spec;
  optional consumer note)

**[260426-1025] rough**

## Character

Targeted feature addition on the consumer side of enshrine. Closes a silent
semantic divergence between kludge and conjure: today an enshrined vessel
produces different base layers under the two paths. Small-diff, high-confidence
— the resolution code already exists in conjure (rbfd_FoundryDirectorBuild.sh:381+392)
and just needs DRY extraction plus a local-presence guard. The credential
boundary stays exactly where it is: kludge remains uncredentialed; wrest
remains the credentialed step.

## Problem

`rbfd_kludge` (rbfd_FoundryDirectorBuild.sh:1268-1278) reads only
`RBRV_IMAGE_n_ORIGIN` and ignores `RBRV_IMAGE_n_ANCHOR`. Conjure (line 381+392)
consumes both. Result: an enshrined vessel built via kludge uses upstream base
layers; the same vessel built via conjure uses GAR-anchored base layers. A
developer iterating with kludge believes they are testing the same base image
as conjure — they are not.

The clean path closes the gap without making kludge credentialed. Kludge
resolves the anchor to a full GAR ref, then performs a local-only presence
check (`docker image inspect`) and fails with a remediation pointing at
`rbw-iwe.DirectorWrestsEnshrinedImage.sh`. The credentialed pull happens in
wrest, which already exists. Same pattern applies to origin slots: missing
local cache → `docker pull «origin»` remediation. The `--pull=never` flag on
docker build enforces the no-network contract uniformly.

## Docket

### Approach: kindle-time constant + uniform local-presence check

Three coordinated changes:

1. **Kindle one composite constant** in `rbfc_FoundryCore.sh` after the rbgl
   dependency is satisfied:
   ```
   readonly ZRBFC_ENSHRINES_BASE="${ZRBFC_REGISTRY_HOST}/${ZRBFC_REGISTRY_PATH}/${RBGL_ENSHRINES_ROOT}"
   ```
   Verify the kindle order — conjure already references `RBGL_ENSHRINES_ROOT`
   from rbfc-level code (rbfd line 392), so the dependency exists in practice.
   If not formalized in `zrbfc_kindle`, add `zrbgl_kindle` call.

2. **DRY conjure** — in `zrbfd_stitch_build_json`, replace the recomputed
   `z_gar_repo_base` (line 381, currently
   `${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}`)
   with `ZRBFC_ENSHRINES_BASE` substitution at line 392. Slot resolution
   becomes `"${ZRBFC_ENSHRINES_BASE}/${z_anchor}:${z_anchor}"`. Behavior must
   be byte-equivalent — this is a refactor, not a change.

3. **Rewrite kludge slot loop** (rbfd_FoundryDirectorBuild.sh:1268-1278) to
   mirror conjure's anchor-aware resolution and add a local-presence guard:
   - For each slot, read both ORIGIN and ANCHOR
   - ANCHOR set with empty ORIGIN → `buc_die` (malformed regime — defensive)
   - ANCHOR set → resolve via `${ZRBFC_ENSHRINES_BASE}/${anchor}:${anchor}`
   - ANCHOR empty → pass ORIGIN through verbatim
   - Run `docker image inspect REF >/dev/null 2>&1` on every resolved ref
     (local daemon query — no network, no credentials)
   - Accumulate misses with type-specific remediation:
     - Anchored miss → emit tabtarget pointer to
       `rbw-iwe.DirectorWrestsEnshrinedImage.sh enshrines/«anchor»:«anchor»`
     - Origin miss → emit `docker pull «origin»` instruction
   - On any miss: print full remediation block (use existing
     `buc_warn`/`buc_tabtarget`/`buc_die` pattern from
     `zrbfd_registry_preflight` lines 336-345) and abort
   - Add `--pull=never` to the `docker build` invocation (line 1295)
     unconditionally

### Spec touch

- **`Tools/rbk/vov_veiled/RBSAK-ark_kludge.adoc`** — replace step 2 with
  resolve+verify language. Add new step "Verify Local Image Presence" between
  resolve and assign-hallmark. Amend build step to mention `--pull=never`. Add
  Limitations bullet:
  ```
  * All declared base images must be locally cached before kludge runs:
    anchored slots via {rbtgo_image_wrest_enshrined}, origin slots via plain
    `docker pull`. Kludge never reaches a registry.
  ```
  Drafted wording is in the working chat that produced this pace; mounting
  agent should refine for AsciiDoc/MCM conformance.

- **`Tools/rbk/vov_veiled/RBSAE-ark_enshrine.adoc`** — optional one-line
  consumer note that {rbrv_image_anchor} is read by both {rbtgo_ark_conjure}
  and {rbtgo_ark_kludge}. Defer if it makes the producer spec noisy.

### Surface

- `Tools/rbk/rbfc_FoundryCore.sh` — one kindled constant added; verify rbgl
  kindle precedes (add the call if not present)
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh` — `zrbfd_stitch_build_json`
  (line 381+392, DRY refactor) and `rbfd_kludge` (line 1268-1295, slot loop
  rewrite + presence check + `--pull=never`)
- `Tools/rbk/vov_veiled/RBSAK-ark_kludge.adoc` — step 2 rewrite, new step,
  build step amendment, Limitations bullet
- `Tools/rbk/vov_veiled/RBSAE-ark_enshrine.adoc` — optional consumer note

No tabtarget surface change. No new prefixes. No new modules.

### Verification

1. **Kludge unchanged for origin-only, pre-cached vessels** — pick a vessel
   with no anchors, base images already in `docker images`, kludge succeeds
   exactly as today.
2. **Kludge fails clearly when origin not cached** — same vessel, fresh
   docker cache, kludge fails with `docker pull «origin»` remediation; run
   the pull; kludge succeeds.
3. **Kludge fails clearly when anchor not cached** — vessel with anchors,
   fresh cache, kludge fails with `rbw-iwe enshrines/«anchor»:«anchor»`
   remediation; run wrest; kludge succeeds.
4. **Anchored kludge produces same base layer as conjure** — for the same
   anchored vessel, diff `docker inspect --format='{{.RootFS.Layers}}'` of
   the kludge build and the conjure build's pulled image. Base-layer digests
   identical confirms the gap is closed.
5. **Malformed regime caught** — fabricate a vessel rbrv.env with ANCHOR
   set and ORIGIN empty; kludge dies with explicit "malformed regime"
   message before docker is invoked.
6. **`--pull=never` enforcement** — confirm `docker build --pull=never`
   surfaces hardcoded FROM stages not parameterized through `RBF_IMAGE_n`.
   This is a vessel hygiene side benefit, not a regression — but if any
   existing vessel in the repo has such a FROM, document it as a
   spook-or-itch for follow-up rather than blocking this pace.
7. **kludge_lifecycle SingleCase GREEN** —
   `tt/rbtd-s.SingleCase.kludge-lifecycle.sh` (or whatever the canonical
   single-case name is in the fundus registry).
8. **crucible suite GREEN** — `tt/rbtd-s.TestSuite.crucible.sh`. Watch for
   onboarding-flow scripts (`rbw-Ofc`, `rbw-Ots`, `rbw-Oda`) that may need
   to pre-pull/pre-wrest before the kludge step. If a flow breaks because
   it assumed auto-pull, the fix is one new step in the flow, not a
   weakening of this pace's contract.
9. **fast suite GREEN** — `tt/rbtd-s.TestSuite.fast.sh`. The kindled
   constant addition should be invisible to non-foundry tests.

### Out of scope

- **Conjure behavioral change.** The DRY refactor must be byte-equivalent.
  Conjure resolve-loop output identical pre/post.
- **Wrest API change.** `rbfl_wrest`'s general locator signature
  (`category/path:tag` with cloud prefix prepended internally) stays. Three
  categories (hallmarks/reliquaries/enshrines) under one signature is the
  right abstraction for wrest.
- **Auto-wrest from kludge.** Tempting but wrong — that re-introduces
  credentials into kludge. Hard line: kludge never authenticates.
- **Auto-pull from kludge for origin slots.** Same principle — `--pull=never`
  is unconditional. Operator pre-pulls.
- **Conditional `--pull=never` based on anchor presence.** Considered and
  rejected in working chat — uniform contract is simpler and the
  origin-pre-pull cost is acceptable given the clear remediation message.
- **Onboarding script refactor.** Existing flows may need a one-line note
  ("first-run kludge requires pre-pulled origin images"); leave to a
  follow-up if the cold-cache pass surfaces real friction.

### Hard dependency notes

- Independent of BA's other paces. Disjoint files from BAAAG (jettison
  cascade): BAAAG is `rbfl_jettison`/`rbfc_FoundryCore.sh:575`; this pace
  is `rbfd_kludge`/`rbfc_FoundryCore.sh` kindle section.
- Builds on BAAAD's `rbw-iwe.DirectorWrestsEnshrinedImage.sh` mint (already
  landed). The remediation message references that tabtarget by exact name.

### Cadence

1. Mount BA, parade this pace, read this docket.
2. Read in order before any edit:
   - `rbfc_FoundryCore.sh` kindle section — find right insertion point,
     verify or add rbgl kindle precedence
   - `rbfd_FoundryDirectorBuild.sh:381+392` — conjure DRY target
   - `rbfd_FoundryDirectorBuild.sh:1268-1295` — kludge target
   - `rbfd_FoundryDirectorBuild.sh:336-345` — existing
     `buc_warn`/`buc_tabtarget`/`buc_die` remediation pattern to mirror
   - `rbfl_wrest` (rbfl_FoundryLedger.sh:733-779) — locator format reminder
3. Apply changes in this order:
   - Kindled constant → conjure DRY (verify behavior unchanged with a
     manual diff or quick conjure run) → kludge slot loop rewrite →
     presence-check block → `--pull=never` → spec rewrite
4. Single notch when:
   - bash -n / shellcheck clean on touched files
   - kludge_lifecycle SingleCase GREEN
   - manual cold-cache test of an anchored vessel passes (remediation →
     wrest → build)
   - manual cold-cache test of an origin-only vessel passes (remediation →
     docker pull → build)
   - crucible suite GREEN
   - fast suite GREEN
5. Wrap.

### References

- `Tools/rbk/rbfc_FoundryCore.sh:51-53` (`ZRBFC_REGISTRY_HOST` /
  `ZRBFC_REGISTRY_PATH` — composition pattern)
- `Tools/rbk/rbgl_GarLayout.sh:50` (`RBGL_ENSHRINES_ROOT` — already includes
  `RBRR_CLOUD_PREFIX`)
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh:381+392` (conjure's anchor
  resolution — DRY target)
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh:1234-1320` (`rbfd_kludge` — target)
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh:336-345` (preflight remediation
  pattern to mirror)
- `Tools/rbk/rbfl_FoundryLedger.sh:733-779` (`rbfl_wrest` — locator format
  `enshrines/«anchor»:«anchor»` for remediation message)
- `Tools/rbk/vov_veiled/RBSAK-ark_kludge.adoc` (spec to rewrite)
- `Tools/rbk/vov_veiled/RBSAE-ark_enshrine.adoc` (producer-side spec;
  optional consumer note)

### imageops-fixtures (₢BAAAF) [abandoned]

**[260514-1120] abandoned**

## Character

Additive test infrastructure. Two new service-tier fixtures exercising the reliquary and enshrinement lifecycles end-to-end against live GAR. Catches the regression class where Pace 1's piecemeal jettison either silently fails OR rekon fails to reflect the resulting interior-hole state.

Sequential/mechanical work item per fixture: scaffold the fixture file, add cases, register in manifest, run.

## Docket

Two new theurge fixtures, both service-tier (require GCP credentials, no container runtime). Together they form the safety net for piecemeal-jettison correctness and post-jettison enumeration accuracy.

### Inputs (specify before scaffolding)

- **Reliquary fixture**: inscribe a fresh reliquary using a stable test spec. Suggest a 3-tool reliquary (small enough for fast cycle, large enough to verify decrement behavior). Test spec lives at a path under the kit's existing reliquary spec convention — mint or reuse on touch.
- **Enshrine fixture**: use `rbev-busybox` (or smallest available kit-shipped vessel with `RBRV_IMAGE_ORIGIN` set). Confirms the small-base enshrine path with minimal Cloud Build cost per fixture run.

### Idempotency / re-run discipline

Each fixture begins with a prereq-sweep case that probes for residue from prior runs and jettisons it before the inscribe/enshrine case. Avoids "already exists" failures on retry. Alternative considered: unique-per-run stamps (e.g., timestamped). Rejected — inflates GAR clutter over time and complicates manual cleanup.

### GAR empty-package behavior — answered (per ₢A_AAS investigation, 260426)

GAR's `ListPackages` retains a package entry as long as ANY manifest content remains under it. After a single-platform `DELETE manifest-by-tag` (e.g. pouch from a `docker push`), the package becomes empty and GAR drops it from `ListPackages`. After a multi-platform `DELETE manifest-by-tag` (e.g. vouch, all reliquary tool images, all enshrinements — all pushed via buildx), only the index manifest is removed; per-platform child manifests remain as untagged content and the package persists in `ListPackages`. Empirically confirmed via the morning of 260426's stale-hallmark abjure cleanup, which used GAR REST package-delete and removed all six packages including vouch — the package container had persisted post-jettison.

The new ₣BA pace **`₢BAAAG jettison-multi-platform-cascade-fix`** (slated ahead of this fixture) extends `rbfl_jettison` to cascade through OCI image indices: enumerate per-platform child manifest digests, DELETE each by digest, then DELETE the index by tag. After BAAAG lands, package containers become truly empty and GAR drops them.

Therefore both fixtures' `muster_absent` cases assert **concrete absence**: the stamp/base does NOT appear in catalog after the post-cascade jettison sequence completes. No branching tolerance, no probe-and-codify-at-runtime — pick the asserted shape and write it concretely.

If `₢BAAAG` has not yet landed when this fixture is mounted, this fixture is **blocked**. Hard dependency. Do not implement against a probe-then-decide strategy; do not implement against pre-cascade behavior expecting a follow-up to flip assertions.

### Fixture 1: `reliquary-lifecycle`

Implementation: new `rbtdrc_reliquary_lifecycle.rs` section. Assumes 3-tool spec (parameterize if changed).

Cases (sequential within fixture):

1. `prereq_sweep` — muster reliquaries; jettison-and-abjure any prior fixture stamp residue.
2. `inscribe` — invoke `rbw-dI.DirectorInscribesReliquary.sh` against the fixture spec; verify creates a stamp under `<prefix>reliquaries/<stamp>/` with all 3 tools.
3. `muster_present` — invoke `rbw-imr`; verify the new stamp appears in catalog.
4. `rekon_full` — invoke `rbw-irr <stamp>`; verify all 3 tools listed.
5. `wrest_one` — invoke `rbw-iwr <stamp>/<tool-1>`; verify image pulls cleanly to local daemon.
6. `jettison_one` — invoke `rbw-iJr <stamp>/<tool-1>`; verify surgical delete succeeds.
7. `rekon_decremented` — invoke `rbw-irr <stamp>`; verify exactly 2 tools listed (regression case: catches jettison silently failing OR rekon failing to reflect post-delete state). Post-BAAAG, this MUST pass concretely.
8. `jettison_two` — invoke `rbw-iJr <stamp>/<tool-2>`; verify delete succeeds.
9. `jettison_three` — invoke `rbw-iJr <stamp>/<tool-3>`; verify delete succeeds.
10. `muster_absent` — invoke `rbw-imr`; verify stamp absent from catalog. Concrete absence (post-BAAAG cascade).

### Fixture 2: `enshrine-lifecycle`

Implementation: new `rbtdrc_enshrine_lifecycle.rs` section.

Cases (sequential within fixture):

1. `prereq_sweep` — muster enshrinements; jettison-and-abjure any prior `rbev-busybox`-base residue.
2. `enshrine` — invoke `rbw-dE.DirectorEnshrinesVessel.sh` against the test vessel; verify creates `<prefix>enshrines/<base>`. **Cost note**: invokes Cloud Build (billable per fixture run).
3. `muster_present` — invoke `rbw-ime`; verify the new enshrinement appears in catalog.
4. `wrest` — invoke `rbw-iwe <base>`; verify image pulls cleanly.
5. `jettison` — invoke `rbw-iJe <base>`; verify delete succeeds.
6. `muster_absent` — invoke `rbw-ime`; verify enshrinement absent from catalog. Concrete absence (post-BAAAG cascade).

### Manifest integration

`rbtdrm_manifest.rs` — add fixture sections for both new fixtures. The colophon constants for the new tabtargets land in BAAAD (already wrapped); this pace registers the fixture cases referencing those constants.

### Tier classification

Service-tier — both fixtures need GCP credentials at `RBDC_DIRECTOR_RBRA_FILE` (director ops: inscribe, enshrine, jettison, wrest, muster — all director-tier per BAAAD).

Add to `tt/rbtd-s.TestSuite.service.sh`. Service suite grows by **+16 cases** (10 reliquary-lifecycle + 6 enshrine-lifecycle). Absolute count depends on AAS's contributions already merged; express as relative delta in the docket and verify post-merge.

### Cost / discipline note

- `rbw-dE` (enshrine) invokes Cloud Build per fixture run — billable. Expect a few cents per run.
- `rbw-dI` (inscribe) is pull-and-push, no Cloud Build — negligible cost.
- Run frequency discipline: don't loop these in CI without budget consideration. Sequential-only per existing test discipline; never parallelize.

### Test discipline

Sequential only — both fixtures interact with shared GAR state. Never parallelize with each other or with other service-tier fixtures (per existing test discipline).

The prereq-sweep case at the head and the muster-absent case at the tail bookend cleanup. Mid-fixture failure may leave partial state; manual recovery via `rbw-imr` / `rbw-ime` enumeration + `rbw-iJr`/`rbw-iJe` cleanup is the documented path.

### Verification

- Both fixtures pass cleanly against post-BAAAG depot.
- Each case individually runnable via `tt/rbtd-s.SingleCase.reliquary-lifecycle.sh <case>` and `tt/rbtd-s.SingleCase.enshrine-lifecycle.sh <case>`.
- Fixture cleanup leaves no stamp/base residue (post-fixture muster confirms expected empty state — concrete, post-cascade).
- `tt/rbtd-r.Run.reliquary-lifecycle.sh` and `tt/rbtd-r.Run.enshrine-lifecycle.sh` runnable as standalone fixtures.
- Informal cross-pace check: post-jettison ordain attempts on a yoked vessel fail at BAAAE's precheck — confirms producer + safety net working in concert.

### Dependencies

- **₢BAAAD (imageops-broaden) — DONE.** Provides the `rbw-irr`, `rbw-imr`, `rbw-iwr`, `rbw-iJr`, `rbw-ime`, `rbw-iwe`, `rbw-iJe` tabtargets and underlying ops. Wrapped 260426; available on main.
- **₢BAAAE (imageops-precheck) — DONE.** Provides ordain-time ref existence check; not a hard dependency for this fixture but the informal cross-pace check above relies on it. Wrapped 260426; available on main.
- **₢BAAAG (jettison-multi-platform-cascade-fix) — HARD DEPENDENCY.** The cascade fix is what makes `muster_absent` cases assert concrete absence rather than branching tolerance. Mount this fixture only after BAAAG wraps.

### Out of scope

- Crucible-tier integration (these don't need container runtime)
- Negative-path coverage for the precheck path itself (BAAAE handles its own coverage; not blocking here)
- Tally fixture coverage (tally is retriever-tier; orthogonal to image-family coverage)
- Fixture for hallmark muster (`rbw-imh`) — coverage already provided by AAS's four-mode fixture's tally cases; symmetric director muster of hallmarks doesn't need its own lifecycle fixture
- Stress/scale testing of jettison loops (cost-prohibitive; covered conceptually by 3-tool reliquary)

### Affiliated context

The interior-hole bug class becomes real once piecemeal reliquary jettison is exposed by BAAAD's tabtarget surface. These fixtures are the safety net. Closes the imageops sequence in ₣BA after BAAAG fixes the underlying jettison defect.

**[260426-0908] rough**

## Character

Additive test infrastructure. Two new service-tier fixtures exercising the reliquary and enshrinement lifecycles end-to-end against live GAR. Catches the regression class where Pace 1's piecemeal jettison either silently fails OR rekon fails to reflect the resulting interior-hole state.

Sequential/mechanical work item per fixture: scaffold the fixture file, add cases, register in manifest, run.

## Docket

Two new theurge fixtures, both service-tier (require GCP credentials, no container runtime). Together they form the safety net for piecemeal-jettison correctness and post-jettison enumeration accuracy.

### Inputs (specify before scaffolding)

- **Reliquary fixture**: inscribe a fresh reliquary using a stable test spec. Suggest a 3-tool reliquary (small enough for fast cycle, large enough to verify decrement behavior). Test spec lives at a path under the kit's existing reliquary spec convention — mint or reuse on touch.
- **Enshrine fixture**: use `rbev-busybox` (or smallest available kit-shipped vessel with `RBRV_IMAGE_ORIGIN` set). Confirms the small-base enshrine path with minimal Cloud Build cost per fixture run.

### Idempotency / re-run discipline

Each fixture begins with a prereq-sweep case that probes for residue from prior runs and jettisons it before the inscribe/enshrine case. Avoids "already exists" failures on retry. Alternative considered: unique-per-run stamps (e.g., timestamped). Rejected — inflates GAR clutter over time and complicates manual cleanup.

### GAR empty-package behavior — answered (per ₢A_AAS investigation, 260426)

GAR's `ListPackages` retains a package entry as long as ANY manifest content remains under it. After a single-platform `DELETE manifest-by-tag` (e.g. pouch from a `docker push`), the package becomes empty and GAR drops it from `ListPackages`. After a multi-platform `DELETE manifest-by-tag` (e.g. vouch, all reliquary tool images, all enshrinements — all pushed via buildx), only the index manifest is removed; per-platform child manifests remain as untagged content and the package persists in `ListPackages`. Empirically confirmed via the morning of 260426's stale-hallmark abjure cleanup, which used GAR REST package-delete and removed all six packages including vouch — the package container had persisted post-jettison.

The new ₣BA pace **`₢BAAAG jettison-multi-platform-cascade-fix`** (slated ahead of this fixture) extends `rbfl_jettison` to cascade through OCI image indices: enumerate per-platform child manifest digests, DELETE each by digest, then DELETE the index by tag. After BAAAG lands, package containers become truly empty and GAR drops them.

Therefore both fixtures' `muster_absent` cases assert **concrete absence**: the stamp/base does NOT appear in catalog after the post-cascade jettison sequence completes. No branching tolerance, no probe-and-codify-at-runtime — pick the asserted shape and write it concretely.

If `₢BAAAG` has not yet landed when this fixture is mounted, this fixture is **blocked**. Hard dependency. Do not implement against a probe-then-decide strategy; do not implement against pre-cascade behavior expecting a follow-up to flip assertions.

### Fixture 1: `reliquary-lifecycle`

Implementation: new `rbtdrc_reliquary_lifecycle.rs` section. Assumes 3-tool spec (parameterize if changed).

Cases (sequential within fixture):

1. `prereq_sweep` — muster reliquaries; jettison-and-abjure any prior fixture stamp residue.
2. `inscribe` — invoke `rbw-dI.DirectorInscribesReliquary.sh` against the fixture spec; verify creates a stamp under `<prefix>reliquaries/<stamp>/` with all 3 tools.
3. `muster_present` — invoke `rbw-imr`; verify the new stamp appears in catalog.
4. `rekon_full` — invoke `rbw-irr <stamp>`; verify all 3 tools listed.
5. `wrest_one` — invoke `rbw-iwr <stamp>/<tool-1>`; verify image pulls cleanly to local daemon.
6. `jettison_one` — invoke `rbw-iJr <stamp>/<tool-1>`; verify surgical delete succeeds.
7. `rekon_decremented` — invoke `rbw-irr <stamp>`; verify exactly 2 tools listed (regression case: catches jettison silently failing OR rekon failing to reflect post-delete state). Post-BAAAG, this MUST pass concretely.
8. `jettison_two` — invoke `rbw-iJr <stamp>/<tool-2>`; verify delete succeeds.
9. `jettison_three` — invoke `rbw-iJr <stamp>/<tool-3>`; verify delete succeeds.
10. `muster_absent` — invoke `rbw-imr`; verify stamp absent from catalog. Concrete absence (post-BAAAG cascade).

### Fixture 2: `enshrine-lifecycle`

Implementation: new `rbtdrc_enshrine_lifecycle.rs` section.

Cases (sequential within fixture):

1. `prereq_sweep` — muster enshrinements; jettison-and-abjure any prior `rbev-busybox`-base residue.
2. `enshrine` — invoke `rbw-dE.DirectorEnshrinesVessel.sh` against the test vessel; verify creates `<prefix>enshrines/<base>`. **Cost note**: invokes Cloud Build (billable per fixture run).
3. `muster_present` — invoke `rbw-ime`; verify the new enshrinement appears in catalog.
4. `wrest` — invoke `rbw-iwe <base>`; verify image pulls cleanly.
5. `jettison` — invoke `rbw-iJe <base>`; verify delete succeeds.
6. `muster_absent` — invoke `rbw-ime`; verify enshrinement absent from catalog. Concrete absence (post-BAAAG cascade).

### Manifest integration

`rbtdrm_manifest.rs` — add fixture sections for both new fixtures. The colophon constants for the new tabtargets land in BAAAD (already wrapped); this pace registers the fixture cases referencing those constants.

### Tier classification

Service-tier — both fixtures need GCP credentials at `RBDC_DIRECTOR_RBRA_FILE` (director ops: inscribe, enshrine, jettison, wrest, muster — all director-tier per BAAAD).

Add to `tt/rbtd-s.TestSuite.service.sh`. Service suite grows by **+16 cases** (10 reliquary-lifecycle + 6 enshrine-lifecycle). Absolute count depends on AAS's contributions already merged; express as relative delta in the docket and verify post-merge.

### Cost / discipline note

- `rbw-dE` (enshrine) invokes Cloud Build per fixture run — billable. Expect a few cents per run.
- `rbw-dI` (inscribe) is pull-and-push, no Cloud Build — negligible cost.
- Run frequency discipline: don't loop these in CI without budget consideration. Sequential-only per existing test discipline; never parallelize.

### Test discipline

Sequential only — both fixtures interact with shared GAR state. Never parallelize with each other or with other service-tier fixtures (per existing test discipline).

The prereq-sweep case at the head and the muster-absent case at the tail bookend cleanup. Mid-fixture failure may leave partial state; manual recovery via `rbw-imr` / `rbw-ime` enumeration + `rbw-iJr`/`rbw-iJe` cleanup is the documented path.

### Verification

- Both fixtures pass cleanly against post-BAAAG depot.
- Each case individually runnable via `tt/rbtd-s.SingleCase.reliquary-lifecycle.sh <case>` and `tt/rbtd-s.SingleCase.enshrine-lifecycle.sh <case>`.
- Fixture cleanup leaves no stamp/base residue (post-fixture muster confirms expected empty state — concrete, post-cascade).
- `tt/rbtd-r.Run.reliquary-lifecycle.sh` and `tt/rbtd-r.Run.enshrine-lifecycle.sh` runnable as standalone fixtures.
- Informal cross-pace check: post-jettison ordain attempts on a yoked vessel fail at BAAAE's precheck — confirms producer + safety net working in concert.

### Dependencies

- **₢BAAAD (imageops-broaden) — DONE.** Provides the `rbw-irr`, `rbw-imr`, `rbw-iwr`, `rbw-iJr`, `rbw-ime`, `rbw-iwe`, `rbw-iJe` tabtargets and underlying ops. Wrapped 260426; available on main.
- **₢BAAAE (imageops-precheck) — DONE.** Provides ordain-time ref existence check; not a hard dependency for this fixture but the informal cross-pace check above relies on it. Wrapped 260426; available on main.
- **₢BAAAG (jettison-multi-platform-cascade-fix) — HARD DEPENDENCY.** The cascade fix is what makes `muster_absent` cases assert concrete absence rather than branching tolerance. Mount this fixture only after BAAAG wraps.

### Out of scope

- Crucible-tier integration (these don't need container runtime)
- Negative-path coverage for the precheck path itself (BAAAE handles its own coverage; not blocking here)
- Tally fixture coverage (tally is retriever-tier; orthogonal to image-family coverage)
- Fixture for hallmark muster (`rbw-imh`) — coverage already provided by AAS's four-mode fixture's tally cases; symmetric director muster of hallmarks doesn't need its own lifecycle fixture
- Stress/scale testing of jettison loops (cost-prohibitive; covered conceptually by 3-tool reliquary)

### Affiliated context

The interior-hole bug class becomes real once piecemeal reliquary jettison is exposed by BAAAD's tabtarget surface. These fixtures are the safety net. Closes the imageops sequence in ₣BA after BAAAG fixes the underlying jettison defect.

**[260426-0622] rough**

Drafted from ₢A_AAX in ₣A_.

## Character

Additive test infrastructure. Two new service-tier fixtures exercising the reliquary and enshrinement lifecycles end-to-end against live GAR. Catches the regression class where Pace 1's piecemeal jettison either silently fails OR rekon fails to reflect the resulting interior-hole state.

Sequential/mechanical work item per fixture: scaffold the fixture file, add cases, register in manifest, run.

## Docket

Two new theurge fixtures, both service-tier (require GCP credentials, no container runtime). Together they form the safety net for piecemeal-jettison correctness and post-jettison enumeration accuracy.

### Inputs (specify before scaffolding)

- **Reliquary fixture**: inscribe a fresh reliquary using a stable test spec. Suggest a 3-tool reliquary (small enough for fast cycle, large enough to verify decrement behavior). Test spec lives at a path under the kit's existing reliquary spec convention — mint or reuse on touch.
- **Enshrine fixture**: use `rbev-busybox` (or smallest available kit-shipped vessel with `RBRV_IMAGE_ORIGIN` set). Confirms the small-base enshrine path with minimal Cloud Build cost per fixture run.

### Idempotency / re-run discipline

Each fixture begins with a prereq-sweep case that probes for residue from prior runs and jettisons it before the inscribe/enshrine case. Avoids "already exists" failures on retry. Alternative considered: unique-per-run stamps (e.g., timestamped). Rejected — inflates GAR clutter over time and complicates manual cleanup.

### GAR empty-package behavior — pre-research before writing assertions

Before writing the `muster_absent` cases: invoke jettison on an isolated test stamp/base, then immediately query GAR catalog. Observe whether GAR retains an empty package entry or garbage-collects. Codify the observed behavior as the deterministic assertion in both fixtures' final cases. Do NOT write the case with branching tolerance — pick the asserted shape and write it concretely.

### Fixture 1: `reliquary-lifecycle`

Implementation: new `rbtdrc_reliquary_lifecycle.rs` section. Assumes 3-tool spec (parameterize if changed).

Cases (sequential within fixture):

1. `prereq_sweep` — muster reliquaries; jettison-and-abjure any prior fixture stamp residue.
2. `inscribe` — invoke `rbw-dI.DirectorInscribesReliquary.sh` against the fixture spec; verify creates a stamp under `<prefix>reliquaries/<stamp>/` with all 3 tools.
3. `muster_present` — invoke `rbw-imr`; verify the new stamp appears in catalog.
4. `rekon_full` — invoke `rbw-irr <stamp>`; verify all 3 tools listed.
5. `wrest_one` — invoke `rbw-iwr <stamp>/<tool-1>`; verify image pulls cleanly to local daemon.
6. `jettison_one` — invoke `rbw-iJr <stamp>/<tool-1>`; verify surgical delete succeeds.
7. `rekon_decremented` — invoke `rbw-irr <stamp>`; verify exactly 2 tools listed (regression case: catches jettison silently failing OR rekon failing to reflect post-delete state).
8. `jettison_two` — invoke `rbw-iJr <stamp>/<tool-2>`; verify delete succeeds.
9. `jettison_three` — invoke `rbw-iJr <stamp>/<tool-3>`; verify delete succeeds.
10. `muster_absent` — invoke `rbw-imr`; verify stamp absent from catalog (per pre-researched GAR empty-package behavior).

### Fixture 2: `enshrine-lifecycle`

Implementation: new `rbtdrc_enshrine_lifecycle.rs` section.

Cases (sequential within fixture):

1. `prereq_sweep` — muster enshrinements; jettison-and-abjure any prior `rbev-busybox`-base residue.
2. `enshrine` — invoke `rbw-dE.DirectorEnshrinesVessel.sh` against the test vessel; verify creates `<prefix>enshrines/<base>`. **Cost note**: invokes Cloud Build (billable per fixture run).
3. `muster_present` — invoke `rbw-ime`; verify the new enshrinement appears in catalog.
4. `wrest` — invoke `rbw-iwe <base>`; verify image pulls cleanly.
5. `jettison` — invoke `rbw-iJe <base>`; verify delete succeeds.
6. `muster_absent` — invoke `rbw-ime`; verify enshrinement absent from catalog (per pre-researched GAR empty-package behavior).

### Manifest integration

`rbtdrm_manifest.rs` — add fixture sections for both new fixtures. The colophon constants for the new tabtargets land in Pace 1; this pace registers the fixture cases referencing those constants.

### Tier classification

Service-tier — both fixtures need GCP credentials at `RBDC_DIRECTOR_RBRA_FILE` (director ops: inscribe, enshrine, jettison, wrest, muster — all director-tier per Pace 1).

Add to `tt/rbtd-s.TestSuite.service.sh`. Service suite grows by **+16 cases** (10 reliquary-lifecycle + 6 enshrine-lifecycle). Absolute count depends on AAS landing first (AAS expands four-mode); express as relative delta in the docket and verify post-merge.

### Cost / discipline note

- `rbw-dE` (enshrine) invokes Cloud Build per fixture run — billable. Expect a few cents per run.
- `rbw-dI` (inscribe) is pull-and-push, no Cloud Build — negligible cost.
- Run frequency discipline: don't loop these in CI without budget consideration. Sequential-only per existing test discipline; never parallelize.

### Test discipline

Sequential only — both fixtures interact with shared GAR state. Never parallelize with each other or with other service-tier fixtures (per existing test discipline).

The prereq-sweep case at the head and the muster-absent case at the tail bookend cleanup. Mid-fixture failure may leave partial state; manual recovery via `rbw-imr` / `rbw-ime` enumeration + `rbw-iJr`/`rbw-iJe` cleanup is the documented path.

### Verification

- Both fixtures pass cleanly against post-AAE depot.
- Each case individually runnable via `tt/rbtd-s.SingleCase.reliquary-lifecycle.sh <case>` and `tt/rbtd-s.SingleCase.enshrine-lifecycle.sh <case>`.
- Fixture cleanup leaves no stamp/base residue (post-fixture muster confirms expected empty state per pre-researched GAR behavior).
- `tt/rbtd-r.Run.reliquary-lifecycle.sh` and `tt/rbtd-r.Run.enshrine-lifecycle.sh` runnable as standalone fixtures.
- Informal cross-pace check: post-jettison ordain attempts on a yoked vessel fail at Pace 2's precheck — confirms producer + safety net working in concert.

### Dependencies

- **Pace 1 (₢A_AAV) must land first.** Cases reference `rbw-irr`, `rbw-imr`, `rbw-iwr`, `rbw-iJr`, `rbw-ime`, `rbw-iwe`, `rbw-iJe` — none exist before Pace 1.
- Pace 2 (₢A_AAW) is independent — fixtures don't directly assert against precheck behavior, though precheck's correctness is implicitly validated by the informal cross-pace check above.

### Out of scope

- Crucible-tier integration (these don't need container runtime)
- Negative-path coverage for the precheck path itself (could be a follow-up pace; not blocking)
- Tally fixture coverage (tally is retriever-tier; orthogonal to image-family coverage)
- Fixture for hallmark muster (`rbw-imh`) — coverage already provided by existing four-mode fixture's tally cases; symmetric director muster of hallmarks doesn't need its own lifecycle fixture
- Stress/scale testing of jettison loops (cost-prohibitive; covered conceptually by 3-tool reliquary)

### Affiliated context

The interior-hole bug class becomes real once Pace 1 exposes piecemeal reliquary jettison. These fixtures are the safety net. Closes the imageops sequence.

**[260425-0930] rough**

## Character

Additive test infrastructure. Two new service-tier fixtures exercising the reliquary and enshrinement lifecycles end-to-end against live GAR. Catches the regression class where Pace 1's piecemeal jettison either silently fails OR rekon fails to reflect the resulting interior-hole state.

Sequential/mechanical work item per fixture: scaffold the fixture file, add cases, register in manifest, run.

## Docket

Two new theurge fixtures, both service-tier (require GCP credentials, no container runtime). Together they form the safety net for piecemeal-jettison correctness and post-jettison enumeration accuracy.

### Inputs (specify before scaffolding)

- **Reliquary fixture**: inscribe a fresh reliquary using a stable test spec. Suggest a 3-tool reliquary (small enough for fast cycle, large enough to verify decrement behavior). Test spec lives at a path under the kit's existing reliquary spec convention — mint or reuse on touch.
- **Enshrine fixture**: use `rbev-busybox` (or smallest available kit-shipped vessel with `RBRV_IMAGE_ORIGIN` set). Confirms the small-base enshrine path with minimal Cloud Build cost per fixture run.

### Idempotency / re-run discipline

Each fixture begins with a prereq-sweep case that probes for residue from prior runs and jettisons it before the inscribe/enshrine case. Avoids "already exists" failures on retry. Alternative considered: unique-per-run stamps (e.g., timestamped). Rejected — inflates GAR clutter over time and complicates manual cleanup.

### GAR empty-package behavior — pre-research before writing assertions

Before writing the `muster_absent` cases: invoke jettison on an isolated test stamp/base, then immediately query GAR catalog. Observe whether GAR retains an empty package entry or garbage-collects. Codify the observed behavior as the deterministic assertion in both fixtures' final cases. Do NOT write the case with branching tolerance — pick the asserted shape and write it concretely.

### Fixture 1: `reliquary-lifecycle`

Implementation: new `rbtdrc_reliquary_lifecycle.rs` section. Assumes 3-tool spec (parameterize if changed).

Cases (sequential within fixture):

1. `prereq_sweep` — muster reliquaries; jettison-and-abjure any prior fixture stamp residue.
2. `inscribe` — invoke `rbw-dI.DirectorInscribesReliquary.sh` against the fixture spec; verify creates a stamp under `<prefix>reliquaries/<stamp>/` with all 3 tools.
3. `muster_present` — invoke `rbw-imr`; verify the new stamp appears in catalog.
4. `rekon_full` — invoke `rbw-irr <stamp>`; verify all 3 tools listed.
5. `wrest_one` — invoke `rbw-iwr <stamp>/<tool-1>`; verify image pulls cleanly to local daemon.
6. `jettison_one` — invoke `rbw-iJr <stamp>/<tool-1>`; verify surgical delete succeeds.
7. `rekon_decremented` — invoke `rbw-irr <stamp>`; verify exactly 2 tools listed (regression case: catches jettison silently failing OR rekon failing to reflect post-delete state).
8. `jettison_two` — invoke `rbw-iJr <stamp>/<tool-2>`; verify delete succeeds.
9. `jettison_three` — invoke `rbw-iJr <stamp>/<tool-3>`; verify delete succeeds.
10. `muster_absent` — invoke `rbw-imr`; verify stamp absent from catalog (per pre-researched GAR empty-package behavior).

### Fixture 2: `enshrine-lifecycle`

Implementation: new `rbtdrc_enshrine_lifecycle.rs` section.

Cases (sequential within fixture):

1. `prereq_sweep` — muster enshrinements; jettison-and-abjure any prior `rbev-busybox`-base residue.
2. `enshrine` — invoke `rbw-dE.DirectorEnshrinesVessel.sh` against the test vessel; verify creates `<prefix>enshrines/<base>`. **Cost note**: invokes Cloud Build (billable per fixture run).
3. `muster_present` — invoke `rbw-ime`; verify the new enshrinement appears in catalog.
4. `wrest` — invoke `rbw-iwe <base>`; verify image pulls cleanly.
5. `jettison` — invoke `rbw-iJe <base>`; verify delete succeeds.
6. `muster_absent` — invoke `rbw-ime`; verify enshrinement absent from catalog (per pre-researched GAR empty-package behavior).

### Manifest integration

`rbtdrm_manifest.rs` — add fixture sections for both new fixtures. The colophon constants for the new tabtargets land in Pace 1; this pace registers the fixture cases referencing those constants.

### Tier classification

Service-tier — both fixtures need GCP credentials at `RBDC_DIRECTOR_RBRA_FILE` (director ops: inscribe, enshrine, jettison, wrest, muster — all director-tier per Pace 1).

Add to `tt/rbtd-s.TestSuite.service.sh`. Service suite grows by **+16 cases** (10 reliquary-lifecycle + 6 enshrine-lifecycle). Absolute count depends on AAS landing first (AAS expands four-mode); express as relative delta in the docket and verify post-merge.

### Cost / discipline note

- `rbw-dE` (enshrine) invokes Cloud Build per fixture run — billable. Expect a few cents per run.
- `rbw-dI` (inscribe) is pull-and-push, no Cloud Build — negligible cost.
- Run frequency discipline: don't loop these in CI without budget consideration. Sequential-only per existing test discipline; never parallelize.

### Test discipline

Sequential only — both fixtures interact with shared GAR state. Never parallelize with each other or with other service-tier fixtures (per existing test discipline).

The prereq-sweep case at the head and the muster-absent case at the tail bookend cleanup. Mid-fixture failure may leave partial state; manual recovery via `rbw-imr` / `rbw-ime` enumeration + `rbw-iJr`/`rbw-iJe` cleanup is the documented path.

### Verification

- Both fixtures pass cleanly against post-AAE depot.
- Each case individually runnable via `tt/rbtd-s.SingleCase.reliquary-lifecycle.sh <case>` and `tt/rbtd-s.SingleCase.enshrine-lifecycle.sh <case>`.
- Fixture cleanup leaves no stamp/base residue (post-fixture muster confirms expected empty state per pre-researched GAR behavior).
- `tt/rbtd-r.Run.reliquary-lifecycle.sh` and `tt/rbtd-r.Run.enshrine-lifecycle.sh` runnable as standalone fixtures.
- Informal cross-pace check: post-jettison ordain attempts on a yoked vessel fail at Pace 2's precheck — confirms producer + safety net working in concert.

### Dependencies

- **Pace 1 (₢A_AAV) must land first.** Cases reference `rbw-irr`, `rbw-imr`, `rbw-iwr`, `rbw-iJr`, `rbw-ime`, `rbw-iwe`, `rbw-iJe` — none exist before Pace 1.
- Pace 2 (₢A_AAW) is independent — fixtures don't directly assert against precheck behavior, though precheck's correctness is implicitly validated by the informal cross-pace check above.

### Out of scope

- Crucible-tier integration (these don't need container runtime)
- Negative-path coverage for the precheck path itself (could be a follow-up pace; not blocking)
- Tally fixture coverage (tally is retriever-tier; orthogonal to image-family coverage)
- Fixture for hallmark muster (`rbw-imh`) — coverage already provided by existing four-mode fixture's tally cases; symmetric director muster of hallmarks doesn't need its own lifecycle fixture
- Stress/scale testing of jettison loops (cost-prohibitive; covered conceptually by 3-tool reliquary)

### Affiliated context

The interior-hole bug class becomes real once Pace 1 exposes piecemeal reliquary jettison. These fixtures are the safety net. Closes the imageops sequence.

**[260425-0921] rough**

## Character

Additive test infrastructure. Two new service-tier fixtures exercising the reliquary and enshrinement lifecycles end-to-end against live GAR. Catches the interior-hole class of bug that Pace 1's piecemeal jettison enables.

Sequential/mechanical work item per fixture: scaffold the fixture file, add cases, register in manifest, run.

## Docket

Two new theurge fixtures, both service-tier (require GCP credentials, no container runtime). Together they form the safety net for piecemeal jettison + interior-hole regressions.

### Fixture 1: `reliquary-lifecycle`

Implementation: new `rbtdrc_reliquary_lifecycle.rs` section (parallel structure to existing `rbtdrc_*` fixture sections).

Cases (sequential within fixture):

1. `inscribe` — invoke `rbw-dI.DirectorInscribesReliquary.sh` against the test reliquary spec; verify creates a stamp under `<prefix>reliquaries/<stamp>/`.
2. `muster_present` — invoke `rbw-imr`; verify the new stamp appears in catalog.
3. `rekon_full` — invoke `rbw-irr <stamp>`; verify all N tools listed.
4. `wrest_one` — invoke `rbw-iwr <stamp>/<tool>`; verify image pulls cleanly to local daemon.
5. `jettison_one` — invoke `rbw-iJr <stamp>/<tool>` on the wrested tool; verify surgical delete succeeds.
6. `rekon_decremented` — invoke `rbw-irr <stamp>`; verify N-1 tools listed (interior-hole state explicitly surfaced — this is the regression case the fixture exists to catch).
7. `jettison_remaining` — loop: jettison each remaining tool one-by-one.
8. `muster_absent` — invoke `rbw-imr`; verify the now-empty stamp is gone (or surfaces as empty — confirm GAR behavior on first run and codify the expectation in the case assertion).

### Fixture 2: `enshrine-lifecycle`

Implementation: new `rbtdrc_enshrine_lifecycle.rs` section.

Cases (sequential within fixture):

1. `enshrine` — invoke `rbw-dE.DirectorEnshrinesVessel.sh` against a test vessel with a small base image; verify creates `<prefix>enshrines/<base>`.
2. `muster_present` — invoke `rbw-ime`; verify the new enshrinement appears in catalog.
3. `wrest` — invoke `rbw-iwe <base>`; verify image pulls cleanly.
4. `jettison` — invoke `rbw-iJe <base>`; verify delete succeeds.
5. `muster_absent` — invoke `rbw-ime`; verify the enshrinement is gone from catalog.

### Manifest integration

`rbtdrm_manifest.rs` — add fixture sections for both new fixtures. The colophon constants for the new tabtargets land in Pace 1; this pace registers the fixture cases referencing those constants.

Test-side colophon constants if not already added by Pace 1:
- `RBTDRM_COLOPHON_INSCRIBE_RELIQUARY`, `RBTDRM_COLOPHON_ENSHRINE_VESSEL` — verify present
- `RBTDRM_COLOPHON_REKON_RELIQUARY`, `RBTDRM_COLOPHON_MUSTER_RELIQUARIES`, `RBTDRM_COLOPHON_MUSTER_ENSHRINEMENTS`, `RBTDRM_COLOPHON_WREST_RELIQUARY`, `RBTDRM_COLOPHON_WREST_ENSHRINED`, `RBTDRM_COLOPHON_JETTISON_RELIQUARY`, `RBTDRM_COLOPHON_JETTISON_ENSHRINEMENT` — verify added by Pace 1

### Tier classification

Service-tier — both fixtures need GCP credentials at `RBDC_DIRECTOR_RBRA_FILE` (director ops: inscribe, enshrine, jettison, wrest, muster — all director-tier per Pace 1).

Add to `tt/rbtd-s.TestSuite.service.sh` — service-suite case count grows from 80 to ~80+13 = 93 cases (8 reliquary-lifecycle + 5 enshrine-lifecycle).

### Test discipline

Sequential only — both fixtures interact with shared GAR state. Never parallelize with each other or with other service-tier fixtures (per existing test discipline).

Each fixture must clean up after itself (final muster-absent cases double as cleanup verification). Failed fixtures may leave residue; rerun with manual cleanup if needed.

### Verification

- Both fixtures pass cleanly against post-AAE depot.
- Each case individually runnable via `tt/rbtd-s.SingleCase.reliquary-lifecycle.sh <case>` and `tt/rbtd-s.SingleCase.enshrine-lifecycle.sh <case>`.
- Fixture cleanup leaves no stamp/base residue (post-fixture muster confirms empty state).
- `tt/rbtd-r.Run.reliquary-lifecycle.sh` and `tt/rbtd-r.Run.enshrine-lifecycle.sh` runnable as standalone fixtures.
- Informal cross-pace check: post-jettison ordain attempts on a yoked vessel fail at Pace 2's precheck — confirms producer + safety net working in concert.

### Dependencies

- **Pace 1 (₢A_AAV) must land first.** Cases reference `rbw-irr`, `rbw-imr`, `rbw-iwr`, `rbw-iJr`, `rbw-ime`, `rbw-iwe`, `rbw-iJe` — none exist before Pace 1.
- Pace 2 (₢A_AAW) is independent — fixtures don't directly assert against precheck behavior, though precheck's correctness is implicitly validated by the informal cross-pace check above.

### Out of scope

- Crucible-tier integration (these don't need container runtime)
- Negative-path coverage for the precheck path itself (could be a follow-up pace; not blocking)
- Tally fixture coverage (tally is retriever-tier; orthogonal to image-family coverage)
- Fixture for hallmark muster (`rbw-imh`) — coverage already provided by existing four-mode fixture's tally cases; symmetric director muster of hallmarks doesn't need its own lifecycle fixture

### Affiliated context

The interior-hole bug class is real now that Pace 1 exposes piecemeal reliquary jettison. These fixtures are the safety net. Closes the imageops sequence.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 D imageops-broaden
  2 E imageops-precheck
  3 G jettison-multi-platform-cascade-fix
  4 A oauth-trust-hygiene-sweep
  5 B bcg-shellcheck-cross-module-discipline
  6 C oci-magic-string-extraction
  7 H kludge-aware-charge-prereq
  8 I kludge-honors-enshrine-anchors

DEGABCHI
··xx··x· rbgc_Constants.sh
··xxx··· rbgp_Payor.sh
·x····xx rbfd_FoundryDirectorBuild.sh
···xx··· bud_dispatch.sh, rbgg_Governor.sh
x······x RBS0-SpecTop.adoc
x····x·· rbfv_FoundryVerify.sh
x··x···· rbk-claude-tabtarget-context.md, rbz_zipper.sh
x·x····· RBSIJ-image_jettison.adoc, RBSIR-image_rekon.adoc, rbfc_FoundryCore.sh
·······x RBSAK-ark_kludge.adoc, RBSRV-RegimeVessel.adoc
······x· rbob_bottle.sh
····x··· BCG-BashConsoleGuide.md, bul_launcher.sh, bul_nolog_launcher.sh, but_test.sh, rbgb_Buckets.sh, rbgd_DepotConstants.sh
···x···· buh_handbook.sh, rbgo_OAuth.sh, rbgu_Utility.sh, rbrp_regime.sh
··x····· RBSCL-consecration_tally.adoc, RBSDE-depot_levy.adoc
x······· CLAUDE.consumer.md, CLAUDE.md, RBSIM-image_muster.adoc, RBSIW-image_wrest.adoc, rbfl_FoundryLedger.sh, rbfr_FoundryRetriever.sh, rbhocd_credential_director.sh, rbhodf_director_first_build.sh, rbtdrc_crucible.rs, rbtdrm_manifest.rs, rbw-iJ.DirectorJettisonsImage.sh, rbw-iJe.DirectorJettisonsEnshrinement.sh, rbw-iJh.DirectorJettisonsHallmarkImage.sh, rbw-iJr.DirectorJettisonsReliquaryImage.sh, rbw-ime.DirectorMustersEnshrinements.sh, rbw-imh.DirectorMustersHallmarks.sh, rbw-imr.DirectorMustersReliquaries.sh, rbw-ir.DirectorRekonsImages.sh, rbw-irh.DirectorRekonsHallmark.sh, rbw-irr.DirectorRekonsReliquary.sh, rbw-iw.DirectorWrestsImage.sh, rbw-iwe.DirectorWrestsEnshrinedImage.sh, rbw-iwh.DirectorWrestsHallmarkImage.sh, rbw-iwr.DirectorWrestsReliquaryImage.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 41 commits)

  1 D imageops-broaden
  2 E imageops-precheck
  3 H kludge-aware-charge-prereq
  4 A oauth-trust-hygiene-sweep
  5 G jettison-multi-platform-cascade-fix
  6 B bcg-shellcheck-cross-module-discipline
  7 C oci-magic-string-extraction
  8 I kludge-honors-enshrine-anchors

123456789abcdefghijklmnopqrstuvwxyz
xx·xxx·····························  D  5c
··x···xx···························  E  3c
·················xx················  H  2c
···················xxxxx···········  A  5c
························xxx········  G  3c
···························xx······  B  2c
·····························xx····  C  2c
·······························xxx·  I  3c
```

## Steeplechase

### 2026-05-14 11:20 - Heat - T

imageops-fixtures

### 2026-05-13 14:13 - ₢BAAAI - W

Closed the silent semantic divergence between kludge and conjure: rbfd_kludge now reads both RBRV_IMAGE_n_ORIGIN and RBRV_IMAGE_n_ANCHOR, resolves anchored slots to GAR refs identically to conjure (mirror of rbfd_stitch_build_json:413-448 with malformed-regime defense for anchor-without-origin), runs a docker-image-inspect local-presence guard on every resolved ref, accumulates per-slot misses, and emits a type-specific remediation block before aborting — anchored misses point operators at rbw-iwe.DirectorWrestsEnshrinedImage.sh, origin misses at plain docker pull. The credentialed fetch stays out-of-band; kludge never authenticates. docker build gains --pull=never as a backstop against hardcoded FROM stages that bypass the RBF_IMAGE_n parameterization. Two notches landed under the pace. First notch (a2171543) carried the rbfd_kludge rewrite and RBSAK spec rewrite (intro NOTE softened, Resolve Base Images step rewritten anchor-aware, new Verify Local Image Presence step inserted, Build Image Locally step amended for --pull=never, Limitations bullet for the local-cache contract, See Also gains {rbtgo_image_retrieve}). Departed from the docket's proposed ZRBFC_ENSHRINES_BASE kindle constant: its literal form would have broken byte-equivalence in conjure (the stored anchor carries the rbi_es/ prefix verbatim, so the substitution would have double-prefixed), and the byte-equivalent rename to ZRBFC_GAR_REPO_BASE would have served only 2 of ~9 candidate open-coded sites — asymmetric half-finished cleanup that ran against AAC's freshly-landed Load-Bearing precedent. Skipped the kindle work entirely; kludge mirrors conjure's local z_gar_repo_base pattern verbatim. The full DRY lift remains a candidate for a separate focused pace. Second notch (3d9a5c41) carried spec-drift remediation that AAI's first-class kludge work surfaced. Pre-existing finding: {rbtgo_ark_kludge} was orphaned from RBS0 — every other ark operation (abjure/ordain/conjure/enshrine/graft/about/vouch/summon/inspect) had four touchpoints (mapping attribute, anchor, heading, include directive); kludge had none, despite the hallmark-prefix table at RBS0:1779-1782 already treating it as a peer of c/b/g. Added all four touchpoints with the new === {rbtgo_ark_kludge} section positioned after graft (peer placement). AAI-introduced finding: RBSRV-RegimeVessel.adoc:140-143 enumerated only conjure as the {rbrv_image_anchor} consumer; rewrote the two prose sentences to enumerate both {rbtgo_ark_conjure} and {rbtgo_ark_kludge}, with note about kludge's additional local-cache requirement. BCG bash audit on AAI's code: clean (variable expansion form, local-with-init discipline, ||-form control flow, loop body error handling, array-iteration guards, comment WHY-only justification, no cd/eval/$()-in-locals, external-command probe pattern consistent with rbfc:291 / rbfl:433/486 / etc.). One pre-existing unbraced ${#z_build_args[@]} test and one BCG interpretive ambiguity around multi-decl-with-init (matched by adjacent conjure code) flagged for transparency, not addressed. Operator-side gates from the docket (kludge_lifecycle SingleCase, crucible suite, cold-cache manual flows) deferred per ₣BA paddock's fast-suite reservation against ₣A_'s parallel-period burn-in. Verification landed: bash -n clean, shellcheck delta zero, fast suite 98/98, handbook-render 15/15. Docket-stale notes for the wrap: the docket's kludge_lifecycle SingleCase fixture name was speculative — does not exist in the registry; the docket's kindle-constant prescription was not byte-equivalent as written and was declined per AAC's Load-Bearing precedent; the docket's optional RBSAE consumer note was skipped per Dependency Inversion (producer specs shouldn't enumerate consumers).

### 2026-05-13 14:09 - ₢BAAAI - n

Spec drift remediation around AAI's first-class kludge work. Two findings, both follow-on consequences of taking kludge seriously as an ark operation. Finding (A) — pre-existing but newly surfaced: {rbtgo_ark_kludge} was orphaned from RBS0. Every other ark operation (abjure, ordain, conjure, enshrine, graft, about, vouch, summon, inscribe, inspect) has four touchpoints in RBS0 — attribute reference in the mapping section, [[rbtgo_ark_*]] anchor, === {rbtgo_ark_*} heading, include::RBSA*-*.adoc[] directive. Kludge had none, yet RBSAK:2 referenced {rbtgo_ark_kludge} as its own heading-linked term and the hallmark-prefix table at RBS0:1779-1782 already listed kludge as a peer of c/b/g. Result: the linked term rendered as raw text, RBSAK was orphaned from RBS0's table-of-contents, the spec model treated kludge inconsistently. Repaired all four touchpoints. Mapping section gains :rbtgo_ark_kludge: <<rbtgo_ark_kludge,rbfd_kludge>> after :rbtgo_ark_graft: (peer placement — both are local-build operations, divergent in destination). New section after include::RBSAG-ark_graft.adoc[] and before [[rbtgo_ark_about]]: [[rbtgo_ark_kludge]] anchor + //axvo_method axd_transient axd_grouped voicing triple (matches sibling ark operations) + === {rbtgo_ark_kludge} heading + concise intro paragraph (no registry push, no Cloud Build, no credentials; honors {rbrv_image_anchor} on the same terms as conjure but requires local cache; credentialed fetch out-of-band via {rbtgo_image_retrieve}) + include::RBSAK-ark_kludge.adoc[]. Finding (B) — drift introduced by AAI's anchor-consumer expansion: RBSRV-RegimeVessel.adoc:140-143 enumerated only conjure as the {rbrv_image_anchor} consumer ('When present, conjure substitutes the full GAR reference. When absent, conjure passes...'). After AAI, kludge is an equal consumer of the anchor field with conjure-equivalent semantics. Rewrote the two prose sentences to enumerate both {rbtgo_ark_conjure} and {rbtgo_ark_kludge}, noting kludge's additional local-cache requirement and {rbtgo_image_retrieve} as the credentialed pre-cache path. Non-issues confirmed orthogonal during survey: RBSAC's Resolve Base Images step is now structurally symmetric with the new RBSAK; RBSAE producer spec deliberately does not enumerate consumers per Dependency Inversion; RBSIR/RBSIA anchor mentions concern the enshrine catalog rather than consumer behavior; RBSIP/RBSGS/RBSCB have no kludge-specific surface to align. Verification: handbook-render fixture 15/15 (renders all 15 onboarding/handbook documents via asciidoctor — exercises the include chain and would surface a broken linked term or asciidoc structural error); spec edits are pure prose/structure additions with no executable surface.

### 2026-05-13 14:04 - ₢BAAAI - n

Close the silent semantic divergence between kludge and conjure where rbfd_kludge read only RBRV_IMAGE_n_ORIGIN and ignored RBRV_IMAGE_n_ANCHOR. After AAC's just-landed Load-Bearing precedent (₢BAAAC declined kindle-constant introduction for the OCI magic-string sweep on single-context grounds), departed from this pace's docket on the proposed ZRBFC_ENSHRINES_BASE kindle constant: its literal form would have broken byte-equivalence in conjure (stored RBRV_IMAGE_n_ANCHOR carries the rbi_es/ prefix verbatim via rbgje01-enshrine-copy.sh:65 + zrbfd_enshrine_extract_anchors:1079, so substituting a host/project/repo/rbi_es base would double-prefix), and the byte-equivalent rename to ZRBFC_GAR_REPO_BASE would have served only 2 of ~9 candidate sites (rbfd:413/834/1306/1398/1399/1422/1516/1794/1827 all open-code the same ${REGISTRY_HOST}/${REGISTRY_PATH} root), producing asymmetric half-finished cleanup. Skipped the kindle-constant work entirely — kludge mirrors conjure's local z_gar_repo_base pattern (rbfd:413) verbatim instead. The full DRY lift remains a future itch candidate as its own focused pace. rbfd_kludge slot loop (1381-1391) rewrite: read both ORIGIN and ANCHOR per slot, malformed-regime defense (anchor set + origin empty → buc_die), anchor-aware resolution mirroring conjure (case ':' validation, %:* / ##:* split, pkg_path/tag emptiness checks, GAR ref composition), pass-through for unanchored slots — preserving the prior origin-as-build-arg behavior for vessels without anchors. New presence guard: docker image inspect on every resolved ref, accumulate per-slot misses distinguishing anchored vs pass-through, emit remediation block with buc_warn/buc_bare/buc_tabtarget mirroring rbfd:336-345 pattern (RBZ_WREST_ENSHRINED_IMAGE for anchored misses, literal docker pull for origin misses), buc_die on any miss before docker is invoked — kludge stays uncredentialed by deferring the credentialed fetch out-of-band to wrest/pull. docker build gains --pull=never as a backstop for hardcoded FROM stages that bypass the RBF_IMAGE_n parameterization (per docket's hygiene side benefit). RBSAK spec: intro NOTE softens 'bypassing GAR entirely' to 'without authenticating to or fetching from' (anchored kludge still resolves a GAR ref, just doesn't reach the registry) and adds the explicit conjure-equivalence claim with {rbtgo_image_retrieve} as the wrest pointer. Resolve Base Images step rewritten with anchor-aware branching and the malformed-regime fatal. New Verify Local Image Presence step between resolve and assign-hallmark capturing the docker image inspect contract and the type-specific remediation, with explicit note that the credentialed fetch is out-of-band. Build Image Locally step amended for --pull=never with the backstop rationale. Limitations gains the local-cache-required bullet pointing operators at wrest/docker pull. See Also gains {rbtgo_image_retrieve} for the remediation reference. Operator-side gates from the docket (cold-cache anchored vessel kludge, crucible suite GREEN) deferred per ₣BA paddock's fast-only test-suite reservation — surfaced for operator action when ₣A_'s parallel-period burn-in clears. Verification: bash -n clean; shellcheck baseline-only (no new findings in edit range; all 9 reported findings pre-existing at lines 28/31/504/507/895/953/1207/1538/1833 — SC1090/1091 sourcing, SC2154 cross-module kindle state — none introduced); fast suite 98/98 passed (enrollment-validation 47, regime-validation 27, regime-smoke 9, handbook-render 15).

### 2026-05-13 13:59 - ₢BAAAC - W

Declined the OCI magic-string extraction after BCG:623-grounded survey — protocol-frozen keywords ('manifests', 'blobs') in a single URL-template context don't earn the kindle-constant overhead; 15 inline literals stay. BCG:623's precedent ('v2' as Z«PREFIX»_API_VERSION) extracts a string with cross-context major-version meaning; 'manifests' has neither cross-context use nor drift risk. Landed the one real bug found during survey (A2): rbfv_vouch_gate (rbfv_FoundryVerify.sh:72-73) reconstructed the registry API base locally (`local -r z_registry_host` + `local -r z_registry_api_base`), value-identical to ZRBFC_REGISTRY_API_BASE but a silent drift hazard if RBGC_GAR_HOST_SUFFIX, the /v2/ pin, or RBDC_GAR_REPOSITORY ever change shape. Deleted both locals; call site at line 90 uses the module constant directly. Verification: bash -n clean; fast suite 98/98; crucible 157/157 through tadmor (full lifecycle charge→59 security cases→quench); srjcl/pluml/moriah crucible fixtures unreachable on this branch (nameplates reference pre-migration hallmarks; only tadmor was rekludged after the r260513125123 yoke per ₢A_AAF) — orthogonal to A2's surface, would resolve with a conjure cycle. rbfv_vouch_gate didn't execute end-to-end in any reached fixture (tadmor uses kludge vessels which bypass vouch via ₢BAAAH's kludge-aware preflight); A2 is correct-by-construction (value-identical at the URL-string level), not correct-by-execution. First attempt added a zrbfc_oci_manifest_url_capture function and swept 9 sites before operator stopped on BCG-compliance grounds (BCG:623 calls for kindle constants, not capture functions); reverted in full via Edit before reconsidering. Spook flagged at wrap-time: 'docket-options-as-menu-not-starting-point' — the docket presented two named options (A: constants, B: builder) and I synthesized a third (_capture) without earning that move; worth slating if the pattern recurs.

### 2026-05-13 13:56 - ₢BAAAC - n

Survey-outcome verdict on ₢BAAAC's OCI magic-string extraction docket: declined the per-site extraction; landed the one real bug surfaced by the survey. Investigation across all foundry (rbfc, rbfd, rbfl, rbfr, rbfv) found 15 manifest URL sites and 1 blobs site (not the ~10 + tags/list the docket predicted — tags/list literals are absent; rbfc:642's tags lookup is the GAR REST API, not OCI). Initial attempt added a zrbfc_oci_manifest_url_capture function and swept 9 sites; operator stopped on BCG-compliance grounds — BCG:623 calls for kindle-position readonly constants, not capture functions. Reverted in full. Reconsidered against the BCG:623 precedent (Z«PREFIX»_API_VERSION='v2' as literal-valued kindle constant): both verb-only constants (ZRBFC_OCI_VERB_MANIFESTS='manifests') and slash-bracketed infix constants (ZRBFC_OCI_INFIX_MANIFESTS='/manifests/') would grow each URL-construction line ~20 chars at 15 sites, hide load-bearing slash-separator grammar inside the constant (infix shape), and protect a frozen protocol-defined keyword that doesn't drift and reads cleanly inline. Per Load-Bearing Complexity the constant doesn't earn its existence: BCG:623's 'v2' carries cross-context meaning (major-version pinning across URL types), but 'manifests' is a single fixed string in a single context (one URL template). Operator concurred. A2 landed alone: rbfv_vouch_gate (rbfv_FoundryVerify.sh:72-73) reconstructed the registry API base locally via `local -r z_registry_host` + `local -r z_registry_api_base="https://${z_registry_host}/v2/${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"` — value-identical to ZRBFC_REGISTRY_API_BASE but a silent drift hazard if the URL shape ever changes (RBGC_GAR_HOST_SUFFIX swap, v3 OCI bump, RBDC_GAR_REPOSITORY rename). Deleted both locals; line 90's call site uses ZRBFC_REGISTRY_API_BASE directly. Verification: bash -n clean; fast suite 98/98 (enrollment-validation 47, regime-validation 27, regime-smoke 9, handbook-render 15); crucible suite green through tadmor (157 cases: 47+27+9+15+59 — tadmor full lifecycle charge→59 security cases→quench); srjcl/pluml/moriah crucible fixtures unreachable on this branch (nameplates reference pre-migration hallmarks; only tadmor was rekludged after the r260513125123 yoke per ₢A_AAF) — orthogonal to this pace's surface, would resolve with a conjure cycle. rbfv_vouch_gate didn't run end-to-end in any reached fixture (tadmor uses kludge-vessels which bypass vouch via ₢BAAAH's kludge-aware preflight), but the swap is provably value-identical at the URL-string level. Five other foundry files I touched on the first attempt (rbfc, rbfd, rbfl, rbfr, +rbfv's intended sweep sites) reverted to baseline via Edit before this notch — git diff confirmed clean. Magic-string extraction itself: investigated, declined, captured here for the record.

### 2026-05-13 13:44 - ₢BAAAB - W

Surveyed BCG modules for SC2153 cross-module false positives; survey triggered docket criterion (a) — only 8 of 59 modules fire SC2153, uniform sweep rejected. Discovered AAN F6b's two existing disables in rbgp/rbgg were silently inert (placed after the SOURCED guard, took next-statement scope not file-scope). Corroborated with shellcheck wiki: file-scope requires placement immediately after shebang. Applied line-2 directive with normative coda `# shellcheck disable=SC2153  # kindle chain - per BCG` across 8 files (4 BCG modules: rbgp, rbgg, rbgd, rbgb; 4 BCG-adjacent consumers: but_test, bud_dispatch, bul_launcher, bul_nolog_launcher). Coda is cryptic BCG citation breadcrumb — normative without leaking proprietary doctrine. Removed F6b's verbose rationale comment blocks. BCG amended: Template 1 gains the line-2 directive; 2-paragraph normative section pins required-for-consumers scope, fixed coda wording, load-bearing placement rule, and names `set -u` as the runtime backstop for the typo class SC2153 catches statically. Verification: aggregate SC2153 firings 5 → 1 (the surviving firing in rbev-vessels/.../entrypoint.sh is container-runtime env-var pattern, not kindle chain — out of scope, flagged for separate consideration). bash -n clean × 8, scratch-typo confirms set -u backstop, fast suite 98/98 passed.

### 2026-05-13 13:44 - ₢BAAAB - n

Land the SC2153 cross-module-disable repair across the kindle-chain consumer set in ₢BAAAB. Two findings drove the shape: (1) survey showed only 4 of 59 active BCG modules empirically fire SC2153 — docket criterion (a) triggered ('meaningful fraction do NOT participate in cross-module state sharing'), uniform sweep would be dead weight in 93% of modules. (2) AAN F6b's two existing disables in rbgp/rbgg were silently inert: placed after `ZRBGG_SOURCED=1` / `ZRBGP_SOURCED=1` they took next-statement scope, not file-scope; SC2153 firing count was identical with and without F6b's directives. Corroborated by canonical shellcheck wiki: file-scope requires directive immediately after the shebang, before any executable statement. The repair: line-2 placement with normative coda `# shellcheck disable=SC2153  # kindle chain - per BCG` across 8 files that actually fire SC2153 — 4 BCG modules (rbgp, rbgg, rbgd, rbgb) and 4 BCG-adjacent consumers (but_test sources bute_engine, bud_dispatch references BURC_*, bul_launcher/bul_nolog_launcher are bootstrap launchers that load BCG modules and reference BURC_TOOLS_DIR before kindle). Coda is a cryptic breadcrumb: 'per BCG' is normative citation form, 'kindle chain' names the BCG-internal mechanism — both signal BCG doctrine across the codebase without leaking proprietary wisdom inline (project doesn't ship BCG). Verbose F6b rationale comment blocks removed (they leaked the kindle/sentinel mechanism BCG owns; the line-2 coda intentionally replaces them with a single cryptic citation). BCG amended: Template 1 (Shebang and Copyright) gains the line-2 directive in its example block; a 2-paragraph normative section follows pinning required-for-consumers scope, fixed coda wording, the load-bearing placement rule (post-set/post-SOURCED reduces to next-statement scope and goes silently inert — F6b's failure mode), and names `set -u` as the runtime backstop for the same typo class SC2153 catches statically (misspelled cross-module refs die at first reference rather than expanding to empty). Verification: aggregate SC2153 firings across all .sh files excluding tt/ dropped 5 → 1; the surviving firing in rbev-vessels/rbev-bottle-ccyolo/build-context/entrypoint.sh is a container runtime script (RBOB_HOST_UID/RBOB_HOST_GID arrive via compose env, not kindle chain) and the BCG coda doesn't fit — flagged out of scope, may be a real bug to look at separately (shellcheck suggests _GID could be a typo of _UID; container source defends them as a pair via the `[ -n ${RBOB_HOST_UID:-} ]` guard immediately above). bash -n clean across all 8 .sh files. Scratch-typo backstop verified: `set -u` dies on unbound `$ZRBGU_PREFOX` with exit code 1, confirming runtime backstop intact. Fast suite 98/98 passed (enrollment-validation 47, regime-validation 27, regime-smoke 9, handbook-render 15).

### 2026-05-13 12:17 - ₢BAAAG - W

Bash implementation for the redocketed display-filter + cleanup-policy contract. Three edits: rbgc mints RBGC_GAR_CLEANUP_POLICY_ID and RBGC_GAR_CLEANUP_OLDER_THAN_SEC near existing GAR constants; rbgp_depot_levy extends the create-repo jq body with cleanupPolicyDryRun: false and the map-keyed cleanupPolicies entry matching the verified REST schema in RBSDE step 124; zrbfc_list_packages_capture restructures the package enumerator into a sort-then-iterate loop that calls tags.list per package (pageSize=1) and retains only packages with non-empty .tags, filtering out post-jettison walking-dead packages before they reach ZRBFC_PACKAGE_LIST_FILE. zrbfc_list_anchors_capture (1-deep enshrinements) intentionally untouched per spec scope. rbfl_jettison untouched per docket; V2 DELETE-by-tag stays as is, orphan-child reclamation becomes GAR's responsibility via the new cleanup policy. bash -n clean on all three files; shellcheck delta zero; fast suite 98 passed / 0 failed / 0 skipped. Verification gate 1 live-verified against a freshly-levied depot (cancbhl-d-canest2bhm100003): gcloud artifacts repositories describe returned cleanupPolicies.rb-delete-untagged with action DELETE, condition.tagState UNTAGGED, condition.olderThan '86400s'; cleanupPolicyDryRun absent from response (expected — GCP REST strips proto3 false defaults from read-back, request body had it set explicitly). Map-keyed REST shape confirmed end-to-end. Verification gate 2 (ordain → jettison → tally cycle on multi-platform content showing tally hides walking-dead while GAR REST still lists package container) requires heavier live-infra and is deferred to when ₣A_'s reservation lifts. Hard-dependency follow-up: ₢BAAAF needs the assertion-target shift from GAR-level absence to display-level absence in batch_vouch / muster_absent cases.

### 2026-05-13 12:06 - ₢BAAAG - n

Land bash implementation for ₢BAAAG's display-filter + cleanup-policy contract. rbgc mints RBGC_GAR_CLEANUP_POLICY_ID (rb-delete-untagged) and RBGC_GAR_CLEANUP_OLDER_THAN_SEC (86400s) near the existing GAR constants with comment cross-referencing RBSDE Create Container Repository and RBSIJ. rbgp_depot_levy extends the create-repo jq body with cleanupPolicyDryRun: false and the map-keyed cleanupPolicies entry — standalone jq dry-run confirms the output matches the verified REST schema in RBSDE step 124 (map keyed by policy ID, string-enum action DELETE, condition.tagState UNTAGGED, condition.olderThan Duration 86400s). zrbfc_list_packages_capture restructured: initial packages.list jq extraction unchanged; new sort-then-iterate loop calls tags.list per package (pageSize=1) and retains only packages with (.tags // []) | length > 0 — walking-dead packages with zero live tags (post-V2-DELETE orphan-children carriers) are filtered out before being written to ZRBFC_PACKAGE_LIST_FILE, making tally/rekon display honest immediately and decoupling the user-facing view from GAR's daily cleanup-job cadence. zrbfc_list_anchors_capture (1-deep enshrines) intentionally untouched — spec calls for filter only at the 2-deep hallmark/reliquary sites per RBSCL and RBSIR. rbfl_jettison untouched per docket; V2 DELETE-by-tag stays as is, orphan-child reclamation becomes GAR's responsibility via the new policy. bash -n clean on all three; shellcheck delta zero (no new findings); fast suite 98 passed / 0 failed / 0 skipped. Live-infra verification gates (gcloud describe on fresh depot, ordain→jettison→tally cycle, batch_vouch assertion-target shift in ₢BAAAF) deferred to operator under ₣BA's test-suite reservation against ₣A_.

### 2026-05-13 11:59 - ₢BAAAG - n

Land spec contract for ₢BAAAG's redocketed display-filter + cleanup-policy design. RBSDE Create Container Repository step gains the rb-delete-untagged cleanup policy (UNTAGGED tagState, 86400s olderThan) with cleanupPolicyDryRun: false, plus NOTEs documenting the producer-side asymmetric V2 DELETE contract this underwrites and the verified REST schema with three canonical source URLs embedded for future re-verification (Repository resource reference, v1 discovery doc, cleanup-policy concept guide) — distinguishing the map-keyed-by-policy-ID REST shape from the differently-shaped gcloud CLI policy-file format used with set-cleanup-policies --policy=FILE that does not apply here. RBSIJ adds two NOTEs after Jettison Tag covering single-vs-multi-platform DELETE semantics (single-platform docker push leaves empty package which GAR drops from ListPackages; multi-platform buildx --push / skopeo copy --all leaves orphan children under a persisting package container) and the autonomous-reap-plus-display-filter contract, cross-referenced to RBSDE. RBSCL and RBSIR add a per-package tags.list call with the rule that zero-tag packages are skipped as walking dead, cross-referenced back to RBSDE for the underwriting policy. Pure spec churn, no code changes. Establishes the design contract before the bash implementation lands.

### 2026-05-13 08:00 - ₢BAAAA - W

Mechanical hygiene sweep across 7 OAuth-touching files (rbgo/rbgp/rbgc/rbrp/rbgu/rbgg/buh) applying 12 findings from the upstream trust review: F1+F6a unused-constant deletes (8 total), F6b cross-module SC2153 directives in rbgp+rbgg, F3 OAuth URL constants minted in rbgc with four call-sites swapped in rbgp, F2 TOCTOU close on RBRO write (subshell + umask 077), F5 buh_prompt_secret mint and auth-code callsite swap, F8-F12 rationale comments at five sites, F13 producer-reality correction at the three RBRA writers (rbgu/rbgg/rbgp) and the consumer (rbgo) — multi-line PEM with real newlines, printf %b is defensive — plus BCG brace freebie at rbgu/rbgg writer sites. Three pre-existing bugs surfaced by Tier 2 verification and fixed in-scope (operator authorized): bud_dispatch.sh NO_LOG+INTERACTIVE branch unbound BURD_LOG_HIST (tightened condition so NO_LOG+INTERACTIVE falls through to the no-log elif); rbz_zipper.sh rbw-gPI channel enrollment '' to 'param1' (with regenerated rbk-claude-tabtarget-context.md so rbq_qualify_fast diff stays clean); rbgp_payor_install $1 to BUZ_FOLIO (matched canonical rbfd_enshrine convention — param1 channel exports the folio via env, strips it from positional args before exec). Mid-flow handbook amendment: documented new 'Sign in to Recipe Bottle Payor' Google screen between Choose-account and permissions consent (renumbered screen list to 1-5, hoisted z_yelp_continue declaration above first use). All F-findings verified empirically. Tier 1: bash -n clean on all 7 originally-planned files, fast suite 98 passed / 0 failed / 0 skipped, shellcheck delta 136 to 130 (drop of 6, short of docket-predicted 9-10 but no new findings introduced), regime validators rbw-rov and rbw-rpv PASS. Tier 2: rbw-gPI ceremony completed cleanly — F5 no-echo confirmed empirically (auth-code paste showed no echoed characters), F2 perms 600 confirmed via stat (rbro.env created at 0600 from first byte), F3 URLs round-tripped live to Google's OAuth endpoints, F12 fresh auth via zrbgp_authenticate_capture exercised by follow-on rbw-dl which authenticated and listed 38 depots. Tier 3: rbw-dL levied new depot cancbhl-d-canest2bhm100002 (after initial mantle 403 surfaced missing-depot precondition; operator authorized); rbw-aM minted Governor SA governor-202605130754 with 32-line multi-line PEM RBRA confirming F13 producer reality; rbw-arI invested Retriever t3r, rbw-adI invested Director t3d, both rostered confirming SA presence; rbw-ft Retriever tally signed JWT from multi-line PEM and queried GAR successfully — F13 consumer reality confirmed end-to-end (printf %b correctly handled real-newline PEM, openssl signed without error). Out-of-scope follow-ups surfaced for future paces: (a) RBRA writers (rbgu/rbgg/rbgp) produce files at mode 644 (world-readable) — F2's TOCTOU fix was scoped to RBRO only, RBRA writers don't apply umask 077; the parallel hygiene improvement is worth a sibling pace; (b) AAY remains slated for BCG-wide SC2153 false-positive study to retire the per-file directives added in F6b; (c) docket-baked Tier 3 tabtarget vocabulary (rbw-aC charter / rbw-aK knight / rbw-aL list) is stale relative to current tree (rbw-arI invest retriever / rbw-adI invest director / rbw-arr roster retrievers / rbw-adr roster directors / rbw-aL never existed). Live GCP resources created (operator-authorized): depot cancbhl-d-canest2bhm100002 with Governor, Retriever t3r, and Director t3d SAs. Four notches in this session: bcc29f58 (F1-F13 sweep across 7 files), 44d0f08f (dispatcher + zipper + tabtarget context), 79d8b115 (BUZ_FOLIO consumption in rbgp_payor_install), 50bc4756 (handbook prose for new Google screen).

### 2026-05-13 07:58 - ₢BAAAA - n

Amend rbgp_payor_install handbook prose to document the new 'Sign in to Recipe Bottle Payor' screen Google now interposes between Choose-account and the permissions consent (operator-observed during Tier 2 verification of this pace). Adds the screen as step 2 (yelps the title 'Sign in to Recipe Bottle Payor', explains it confirms account selection and previews the email-address scope, instructs Continue). Renumbers subsequent screens 3-5 (was 2-4): conditional unverified-app, permissions consent, authorization code. Hoists z_yelp_continue declaration above first use so both step 2 and step 4 reference the same button word. Header line updated from 'three or four screens' to 'four or five screens'. Pure handbook prose; no behavior change.

### 2026-05-13 07:37 - ₢BAAAA - n

Third pre-existing bug surfaced by Tier 2 of ₢BAAAA (operator authorized in-scope). rbgp_payor_install was reading its folio from positional ${1:-}, but the BUK workbench dispatch convention for param1-channel enrollments (see buz_zipper.sh:280, buz_exec_lookup) is to export the folio as BUZ_FOLIO and strip it from the positional args before exec'ing the command. The 'param1' channel name describes how the OPERATOR passes the folio (as the first command-line arg to the tabtarget), not how the COMMAND function receives it. Canonical consumers like rbfd_enshrine (rbfd_FoundryDirectorBuild.sh:870) use `local -r z_X="${BUZ_FOLIO:-}"`; rbgp_payor_install was the lone exception still reading $1. After this pace's earlier zipper-channel fix (enrollment '' → 'param1'), the dispatch began consuming the JSON path into BUZ_FOLIO and the function — still reading $1 — saw an empty positional arg and died at 'OAuth JSON file path required as first argument'. Fix: change `local -r z_oauth_json_file="${1:-}"` to `local -r z_oauth_json_file="${BUZ_FOLIO:-}"`. One-line change, single file. bash -n clean. Tier 2 retry pending.

### 2026-05-13 07:36 - ₢BAAAA - n

Fix two pre-existing bugs surfaced by Tier 2 verification of ₢BAAAA's F2/F5/F3 edits (operator authorized in-scope). Bug 1 (bud_dispatch.sh:358): the dispatcher's interactive branch unconditionally referenced BURD_LOG_HIST at the tee pipeline, but BURD_LOG_HIST is only initialized when BURD_NO_LOG is empty (line 179). The rbw-gPI tabtarget intentionally sets BOTH BURD_NO_LOG=1 (credentials must not be logged) and BURD_INTERACTIVE=1 (preserve line-buffered prompt display so the auth-code paste prompt actually appears), and was the only tabtarget hitting this combination. Result was 'BURD_LOG_HIST: unbound variable' from set -u before rbgp_payor_install could even start. Fix tightens the interactive-branch condition from `test -n BURD_INTERACTIVE` to `test -n BURD_INTERACTIVE && test -z BURD_NO_LOG`, so the NO_LOG+INTERACTIVE combination falls through to the existing NO_LOG elif branch — which already preserves line buffering (no while-read curating loop) and skips log writes. The two flags are now properly composable as originally intended. Bug 2 (rbz_zipper.sh:79): the rbw-gPI colophon was enrolled with channel '' (no folio), but rbgp_payor_install takes a JSON path as $1 — the workbench therefore stripped the argument and emitted 'Colophon takes no folio; ignoring unexpected argument(s)' before rbgp_payor_install died on missing arg. Fix changes the enrollment channel from '' to 'param1', matching the pattern used by neighbor enrollments like rbw-dE (param1, takes vessel moniker) and rbw-dY (param1, takes stamp). Also regenerated rbk-claude-tabtarget-context.md via rbw-MG.MarshalGenerate to reflect the channel change (single-line diff in the Guide table: rbw-gPI Folio column now shows 'param1' instead of '—'); this keeps rbq_qualify_fast's live-vs-committed diff clean. Tier 2 retry pending operator action; will run fast-suite regression check after Tier 2 succeeds to backstop the dispatcher fix.

### 2026-05-13 07:22 - ₢BAAAA - n

OAuth-surface trust hygiene sweep across the 7 files: F1 deletes ZRBGO_PRIVATE_KEY_FILE (rbgo, dead). F6a deletes 7 unused ZRBGP_INFIX_* constants (rbgp). F6b adds file-scoped SC2153 shellcheck directives to rbgp and rbgg (false positive on cross-module ZRBGU_PREFIX). F3 mints RBGC_OAUTH_AUTHORIZE_URL + RBGC_OAUTH_USERINFO_URL in rbgc; swaps four bare-URL sites in rbgp (refresh-token POST, install authorize URL prefix, install token POST, install userinfo GET) to the new constants and to existing RBGC_OAUTH_TOKEN_URL. F2 closes TOCTOU on the RBRO credentials write — wraps the write in a subshell with umask 077 so the file is created at 0600 from the first byte; the explicit chmod 600 remains as belt-and-suspenders. F5 mints buh_prompt_secret in buh_handbook (read -rs, trailing newline to stderr), swaps the rbgp_payor_install auth-code-paste callsite from buh_prompt to buh_prompt_secret so the OAuth authorization code no longer echoes to terminal. F8 documents rbgu_rbro_load's wrapper rationale (defensive auto-source after AAD's payor/ subdirectory migration + uniform rbgu_* load-through-utility convention). F9 adds a min=0-deliberate comment to RBRP_OAUTH_CLIENT_ID's enrollment (Retriever-only operators have no Payor identity). F10 documents the field-name-basis OAuth log scrubber in rbgo (catches id_token/client_secret/etc by regex; needs update if a new secret-carrying field arrives). F11 replaces the single-line rbgp header comment with a scope-tag map showing the interleaved OAuth + Depot entry points with corrected line ranges (zrbgp_refresh_capture ~76, zrbgp_authenticate_capture ~122, rbgp_payor_install ~400, rbgp_payor_oauth_refresh ~1229, billing/lien/bucket helpers ~196-395, rbgp_depot_levy/unmake/list ~568-1163, rbgp_governor_mantle ~1268). F12 documents the deliberate no-token-caching choice on zrbgp_authenticate_capture (simplicity + freshness, refresh tokens are long-lived). F13 corrects the inverted RBRA private-key documentation across the three writer sites (rbgu, rbgg, rbgp) and the consumer (rbgo) — reality is jq -r unescapes JSON \n to real newlines, file holds multi-line PEM, consumer's printf %b is defensive against either form. BCG brace freebie folded in at rbgu and rbgg writer sites (rbgp already braced). Tier 1 verification green: bash -n clean on all 7, fast suite 98 passed / 0 failed / 0 skipped, shellcheck delta 136 → 130 (drop of 6 — short of docket's predicted 9-10, no new findings introduced; likely fewer SC2153 false-positive lines collapsed than expected). Tier 2 (interactive Payor install) and Tier 3 (live GCP credential lifecycle) deferred at operator pause; will resume on operator signal. Regime validators (rbw-rov, rbw-rpv) not yet run — were about to launch when paused.

### 2026-05-12 17:58 - ₢BAAAH - W

Minted RBGC_HALLMARK_PREFIX_{CONJURE,KLUDGE,BIND,GRAFT} in rbgc_Constants.sh and swapped the four bare-letter prefix sites in rbfd_FoundryDirectorBuild.sh (conjure inscribe_ts, kludge hallmark, bind mirror_ts, graft graft_ts). Dropped the inline k/c/b distinction comment; constant names carry the meaning. Added zrbob_hallmark_is_kludge in rbob_bottle.sh and branched the four charge-preflight diagnostic sites (sentry/bottle × vouch-missing/image-missing) so kludge-prefixed hallmarks die naming the empty stamp rather than falling through to zrbob_summon_full_hallmark or zrbob_vouch_gate_and_summon — both guaranteed to fail for kludge content that never reaches GAR. Die messages name only the failure, no tabtarget references baked in (per operator feedback). bash -n and shellcheck clean on the three touched files. Fast suite green post-merge (98/98 cases across enrollment-validation, regime-validation, regime-smoke, handbook-render). Discriminator behavior verified directly across all four prefix types using stamps from the live merged regime: kludge (tadmor sentry k260512131447-b914bcc66) → KLUDGE; conjure (moriah/pluml/srjcl sentry c260512131458-...) → not-kludge; bind (pluml bottle b260512140343-...) → not-kludge; synthetic graft g260512100000-... → not-kludge; empty → not-kludge; docket's fake k000000000000-deadbeef → KLUDGE. Four call-site wiring inspected: helper at :230, conditionals at :442/:450/:458/:466 with correct role-pairing. End-to-end charge integration test from the docket was deferred in favor of direct helper validation — the rbob_charge dirty-tree gate would refuse a working-tree fake-stamp edit, and post-merge the real kludge images are present in the shared macOS docker daemon (from beta's pristine run on the same host), so a natural charge would skip the missing-image branch entirely. Pristine qualification passed on both macOS (1h 39m, recorded in merge a30823a8) and Linux/cerebro (95m 58s, recorded in commit 8afa7ff6).

### 2026-05-12 14:15 - ₢BAAAH - n

Mint RBGC_HALLMARK_PREFIX_{CONJURE,KLUDGE,BIND,GRAFT} constants and swap the four bare-letter prefix sites in rbfd_FoundryDirectorBuild.sh (conjure inscribe_ts at the inscribe path, kludge hallmark at the kludge path, bind mirror_ts at the bind path, graft graft_ts at the graft path). Drop the inline comment that explained the k/c/b distinction — constant names carry the meaning. Add zrbob_hallmark_is_kludge helper in rbob_bottle.sh and branch the four charge-preflight diagnostic sites (sentry/bottle × vouch-missing/image-missing) so a kludge-prefixed hallmark dies naming the empty stamp instead of falling through to zrbob_summon_full_hallmark or zrbob_vouch_gate_and_summon — both guaranteed to fail since kludge hallmarks never reach GAR. Die messages name only the failure (no tabtarget suggestions baked in). bash -n and shellcheck clean on the three touched files. Fast-suite verification deferred: the one regime-validation failure (rbtdrf_rv_rbrn_all_nameplates) traces to commit 0ffee867 Marshal Zero zeroing hallmark fields across moriah/pluml/srjcl — orthogonal to this pace's surface, resolves on the upcoming re-stamp merge. Manual tadmor diagnostic test deferred until that merge lands.

### 2026-05-11 15:40 - Heat - f

silks=rbk-11-mvp-parallel-cleanups

### 2026-04-30 10:39 - Heat - n

Regenerate tabtarget context to flush the rbw-Oda description edit from ae039b87. That commit updated rbz_zipper.sh's RBZ_ONBOARD_DIR_AIRGAP enrollment from 'conjure forge' to 'conjure base' (matching the buc_doc_brief sweep) but did not run tt/rbw-MG.MarshalGenerate.sh, leaving the generated rbk-claude-tabtarget-context.md one line stale. Surfaced by ₣BB BBAAs onboarding-sequence empirical re-run: rbq_qualify_fast (the fail-fast precondition gate that runs before every ordain) caught the drift via its diff between live buz_emit_context output and the committed file. Pure prose regeneration; no behavior change.

### 2026-04-30 10:19 - Heat - n

Remove prose 'forge' references from handbook bash and spec docs. The word was acting as a pseudo-quoin without an anchor - the system already had the right vocabulary. Replaced with existing RBYC_VESSEL / RBYC_HALLMARK quoins, the z_lk_forge sigil link (which renders 'rbev-bottle-ifrit-forge' linked to the Vessel concept page), or restructured with anaphora where antecedent was already established. Identifier uses preserved: the rbev-bottle-ifrit-forge vessel sigil (in code samples and ORIGIN config), z_forge_vessel and z_lk_forge local vars, the FORGE_HALLMARK learner-facing env var, and the common-ifrit-forge-context/ source directory. Files: rbhoda_director_airgap.sh (21 edits including header comment, buc_doc_brief, and ~18 buh_line/buh_step1 prose sites), rbho0_start_here.sh (2 edits in airgap track summary), rbz_zipper.sh (1 edit, enrollment description for rbw-Oda matching the buc_doc_brief), RBS0-SpecTop.adoc (1 edit, illustrative example in RBRV_IMAGE_n_ANCHOR linked-term definition - 'just-ordained forge image' to 'another vessel's just-ordained hallmark', generic), RBSAE-ark_enshrine.adoc (1 edit, parallel construction), RBSIP-ifrit_pentester.adoc (2 edits, vessel roster gloss and Dockerfile FROM reference). All bash files pass bash -n. No behavior change - prose only.

### 2026-04-26 10:25 - Heat - r

moved BAAAF to last

### 2026-04-26 10:25 - Heat - S

kludge-honors-enshrine-anchors

### 2026-04-26 10:02 - Heat - r

moved BAAAF to last

### 2026-04-26 09:56 - Heat - S

kludge-aware-charge-prereq

### 2026-04-26 09:55 - Heat - S

kludge-honors-enshrine-anchors

### 2026-04-26 09:07 - Heat - S

jettison-multi-platform-cascade-fix

### 2026-04-26 08:41 - ₢BAAAE - W

Live verification complete — all three docket scenarios PASS against live GCP. Positive: rbev-busybox conjure produced hallmark c260426082922-r260426152928 (conjure 3m12s + vouch 1m32s, both SUCCESS). Negative 1 (enshrined-base miss, conjure path): jettison rbxc-enshrines/busybox-latest-1487d0af5f, attempt ordain → preflight fired with structured error block citing vessel sigil + recovery tabtargets, exited before Cloud Build submission. Negative 2 (reliquary-tool miss, bind path — the new call site closed at 58717214): jettison rbxc-reliquaries/r260425082412/skopeo, attempt ordain rbev-bottle-plantuml (bind vessel) → preflight fired naming skopeo missing (1/6 tools), exited before CB submission. Graft codepath confirmed unmodified by inspection (no preflight call site in rbfd_graft). Side benefit confirmed: enshrine path's pre-existing zrbfd_preflight_reliquary call exercised the upgraded subtree check during restore. Cloud Build cost: 3 runs (conjure + vouch + restore-enshrine). Residual state (deferred per user): reliquary r260425082412 left integrity-broken (skopeo tag jettisoned, not restored) — 10 yoked vessels non-ordainable until re-inscribed via rbw-dI.

### 2026-04-26 07:56 - ₢BAAAE - n

imageops-precheck — bind-path registry preflight + reliquary subtree check. zrbfd_registry_preflight now invoked from rbfd_mirror (closing the bind-path gap); zrbfd_preflight_reliquary upgraded from docker-canary to full canonical-tool subtree (gcloud, docker, alpine, syft, binfmt, skopeo) with miss accumulation and structured per-tool error block citing vessel sigil + RBRV_RELIQUARY stamp + nuclear recovery tabtargets. Graft path remains untouched. Side benefit: enshrine-path's pre-existing zrbfd_preflight_reliquary call inherits the upgrade. Local gates green: build, QualifyFast, fast suite (regime-smoke 8/8, handbook-render 15/15, plus enrollment-validation, regime-validation).

### 2026-04-26 07:31 - ₢BAAAD - W

Documented the imageops three-domain broadening that shipped at e40a2b59. Broadened RBSIR (rekon) to hallmark+reliquary procedures, RBSIJ (jettison) and RBSIW (wrest) to three-domain locator-generic. Minted RBSIM (muster) covering hallmarks/reliquaries/enshrinements with explicit director vs retriever role contrast against tally. RBS0 surgical updates: added rbtgo_image_muster operation, rbst_reliquary_stamp linked term, broadened rbst_locator format from moniker:tag to package-path:tag, refreshed image-op summaries. CLAUDE.md acronym registration, CLAUDE.consumer.md verb tables (muster added; wrest/jettison broadened). All four verification gates green: theurge build, QualifyFast (after MarshalGenerate refresh), handbook-render 15/15, fast suite 93/93.

### 2026-04-26 07:30 - ₢BAAAD - n

imageops doc churn — broadened RBSIR (rekon) to hallmark+reliquary, RBSIJ (jettison) and RBSIW (wrest) to three-domain locator-generic; minted RBSIM (muster) for three-domain catalog listing with director vs retriever role contrast against tally; RBS0 mapping and operations section updated with rbtgo_image_muster, rbst_reliquary_stamp, broadened rbst_locator format, and refreshed image-op summaries; RBSIM registered in CLAUDE.md; CLAUDE.consumer.md verb tables broadened (muster added; wrest/jettison rows acknowledge three-domain locator). Tabtarget context regenerated via rbw-MG. All four gates green: build, QualifyFast, handbook-render 15/15, fast suite 93/93.

### 2026-04-26 07:12 - ₢BAAAD - n

imageops broaden — Foundry refactor, 3-domain tabtarget family, and manifest+handbook plumbing

### 2026-04-26 07:56 - ₢BAAAE - n

imageops-precheck — bind-path registry preflight + reliquary subtree check. zrbfd_registry_preflight now invoked from rbfd_mirror (closing the bind-path gap); zrbfd_preflight_reliquary upgraded from docker-canary to full canonical-tool subtree (gcloud, docker, alpine, syft, binfmt, skopeo) with miss accumulation and structured per-tool error block citing vessel sigil + RBRV_RELIQUARY stamp + nuclear recovery tabtargets. Graft path remains untouched. Side benefit: enshrine-path's pre-existing zrbfd_preflight_reliquary call inherits the upgrade. Local gates green: build, QualifyFast, fast suite (regime-smoke 8/8, handbook-render 15/15, plus enrollment-validation, regime-validation).

### 2026-04-26 07:31 - ₢BAAAD - W

Documented the imageops three-domain broadening that shipped at e40a2b59. Broadened RBSIR (rekon) to hallmark+reliquary procedures, RBSIJ (jettison) and RBSIW (wrest) to three-domain locator-generic. Minted RBSIM (muster) covering hallmarks/reliquaries/enshrinements with explicit director vs retriever role contrast against tally. RBS0 surgical updates: added rbtgo_image_muster operation, rbst_reliquary_stamp linked term, broadened rbst_locator format from moniker:tag to package-path:tag, refreshed image-op summaries. CLAUDE.md acronym registration, CLAUDE.consumer.md verb tables (muster added; wrest/jettison broadened). All four verification gates green: theurge build, QualifyFast (after MarshalGenerate refresh), handbook-render 15/15, fast suite 93/93.

### 2026-04-26 07:30 - ₢BAAAD - n

imageops doc churn — broadened RBSIR (rekon) to hallmark+reliquary, RBSIJ (jettison) and RBSIW (wrest) to three-domain locator-generic; minted RBSIM (muster) for three-domain catalog listing with director vs retriever role contrast against tally; RBS0 mapping and operations section updated with rbtgo_image_muster, rbst_reliquary_stamp, broadened rbst_locator format, and refreshed image-op summaries; RBSIM registered in CLAUDE.md; CLAUDE.consumer.md verb tables broadened (muster added; wrest/jettison rows acknowledge three-domain locator). Tabtarget context regenerated via rbw-MG. All four gates green: build, QualifyFast, handbook-render 15/15, fast suite 93/93.

### 2026-04-26 07:12 - ₢BAAAD - n

imageops broaden — Foundry refactor, 3-domain tabtarget family, and manifest+handbook plumbing

### 2026-04-26 06:34 - Heat - r

moved BAAAF after BAAAE

### 2026-04-26 06:33 - Heat - r

moved BAAAE after BAAAD

### 2026-04-26 06:33 - Heat - r

moved BAAAD to first

### 2026-04-26 06:33 - Heat - f

racing

### 2026-04-26 06:22 - Heat - N

rbk-mvp-3-cleanups-and-imageops

