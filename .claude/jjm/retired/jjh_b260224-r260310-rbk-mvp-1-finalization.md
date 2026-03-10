# Heat Trophy: rbk-mvp-1-finalization

**Firemark:** ₣Ak
**Created:** 260224
**Retired:** 260310
**Status:** retired

## Paddock

# Paddock: rbk-mvp-verification

## Context

(Describe the initiative's background and goals)

## References

(List relevant files, docs, or prior work)

## Paces

### harden-test-sweep-reliability (₢AkAA1) [complete]

**[260310-1206] complete**

Fix test sweep reliability issues discovered during full run.

Prior discussion: session **AkAA1-test-suite-issue-resolution** has accumulated context on all three issues including design trade-offs, test output analysis, and code exploration. Resume that session for continuity.

## 1. ark-lifecycle fact file gap (design discussion needed)

`rbf_check_consecrations` (`rbf_Foundry.sh:1934`) is display-only — iterates all vessels, writes no fact files. The test `rbtcal_ArkLifecycle.sh` expects `rbf_fact_consecrations.txt` per vessel, which doesn't exist.

**Option A — Vessel filter + count fact file.** Add optional vessel arg to `rbf_check_consecrations`. When provided, check only that vessel and write `rbf_fact_consecration_count.txt` (single integer) to `BURD_OUTPUT_DIR`. No-arg mode unchanged. ~10 lines.

**Option B — Clean-slate lifecycle rewrite.** Test unconditionally abjures all busybox consecrations first, asserts zero, conjures one, asserts exactly one, retrieves, abjures, asserts zero. Owns the full lifecycle from empty state.

**Options are not mutually exclusive.** Option A provides the machine-readable query any test needs. Option B improves test isolation. Both may be warranted. User wants fresh-eyes discussion before implementing.

## 2. Bottle service fixtures need stop-before-start

nsproto-security, srjcl-jupyter, pluml-diagram all failed with HTTP 000 / container not running. Root cause may be "never started" or "crashed" — investigate before assuming.

Fix: each fixture setup function should (a) stop any existing bottle service of that type (ok if none running), then (b) start a fresh instance. Identify the specific setup functions in `rbtb_testbench.sh` and the per-service test case files.

## 3. Test sweep needs sequential execution guard

During this session, the agent launched `complete` and `service` suites in parallel bash sessions. This was an agent judgment error — tests exercise shared cloud infrastructure (GCP registry, build triggers, OAuth tokens, container runtime).

Fix: add a guard to the sweep tabtarget or testbench that rejects concurrent invocations (lockfile or PID check). Scope the constraint precisely — pure-local suites like `fast` (enrollment-validation) may be safe to parallelize with each other, but any suite touching GCP or container runtime must be exclusive.

## Splitting consideration

Issues 1, 2, and 3 are independent. If this pace feels too broad during mount, split into sub-paces. Issue 3 (guard) is likely smallest; issue 1 needs design discussion; issue 2 is straightforward once setup functions are identified.

**[260309-2050] rough**

Fix test sweep reliability issues discovered during full run.

Prior discussion: session **AkAA1-test-suite-issue-resolution** has accumulated context on all three issues including design trade-offs, test output analysis, and code exploration. Resume that session for continuity.

## 1. ark-lifecycle fact file gap (design discussion needed)

`rbf_check_consecrations` (`rbf_Foundry.sh:1934`) is display-only — iterates all vessels, writes no fact files. The test `rbtcal_ArkLifecycle.sh` expects `rbf_fact_consecrations.txt` per vessel, which doesn't exist.

**Option A — Vessel filter + count fact file.** Add optional vessel arg to `rbf_check_consecrations`. When provided, check only that vessel and write `rbf_fact_consecration_count.txt` (single integer) to `BURD_OUTPUT_DIR`. No-arg mode unchanged. ~10 lines.

**Option B — Clean-slate lifecycle rewrite.** Test unconditionally abjures all busybox consecrations first, asserts zero, conjures one, asserts exactly one, retrieves, abjures, asserts zero. Owns the full lifecycle from empty state.

**Options are not mutually exclusive.** Option A provides the machine-readable query any test needs. Option B improves test isolation. Both may be warranted. User wants fresh-eyes discussion before implementing.

## 2. Bottle service fixtures need stop-before-start

nsproto-security, srjcl-jupyter, pluml-diagram all failed with HTTP 000 / container not running. Root cause may be "never started" or "crashed" — investigate before assuming.

Fix: each fixture setup function should (a) stop any existing bottle service of that type (ok if none running), then (b) start a fresh instance. Identify the specific setup functions in `rbtb_testbench.sh` and the per-service test case files.

## 3. Test sweep needs sequential execution guard

During this session, the agent launched `complete` and `service` suites in parallel bash sessions. This was an agent judgment error — tests exercise shared cloud infrastructure (GCP registry, build triggers, OAuth tokens, container runtime).

Fix: add a guard to the sweep tabtarget or testbench that rejects concurrent invocations (lockfile or PID check). Scope the constraint precisely — pure-local suites like `fast` (enrollment-validation) may be safe to parallelize with each other, but any suite touching GCP or container runtime must be exclusive.

## Splitting consideration

Issues 1, 2, and 3 are independent. If this pace feels too broad during mount, split into sub-paces. Issue 3 (guard) is likely smallest; issue 1 needs design discussion; issue 2 is straightforward once setup functions are identified.

**[260309-2048] rough**

Fix test sweep reliability issues discovered during full run. Three concerns:

## 1. ark-lifecycle fact file gap (design discussion needed)

`rbf_check_consecrations` is display-only — iterates all vessels, writes no fact files. The test `rbtcal_lifecycle_tcase` expects `rbf_fact_consecrations.txt` per vessel, which doesn't exist.

Design options discussed but NOT resolved:
- Add optional vessel arg to `rbf_check_consecrations` that writes `rbf_fact_consecration_count.txt` (single integer)
- Rewrite test to unconditionally abjure all busybox consecrations first, assert zero, conjure, assert one, retrieve, abjure, assert zero — owning the full lifecycle from clean slate
- Concern: per-vessel consecration *list* fact files introduce too many edge cases; a simple count may suffice
- The agent proposed both approaches; user wants a fresh-eyes discussion on the right trade-off before implementing

## 2. Bottle service fixtures need stop-before-start

nsproto-security, srjcl-jupyter, pluml-diagram all failed with HTTP 000 / container not running. Fixture setup functions should each: (a) stop any existing bottle service of that type (ok if none running), then (b) start a fresh instance. This makes tests self-contained rather than depending on pre-existing container state.

## 3. Test suites must NOT run in parallel

During this session, `complete` and `service` sweep suites were launched in parallel bash sessions. This is unsafe — tests exercise shared cloud infrastructure (GCP registry, build triggers, OAuth tokens, container runtime). Suite execution must be sequential. Document this constraint and ensure the test runner enforces it or at minimum the sweep tabtarget rejects concurrent invocations.

### delete-retriever-list-images (₢AkAAt) [complete]

**[260308-1425] complete**

## Goal

Remove `rbw-Rl` (RetrieverListsImages) and `rbf_list`. The flat tag dump is superseded by `rbw-Dc` (DirectorChecksConsecrations) which provides structured consecration health. The Retriever fallback-to-Director credential pattern violates credential discipline.

## Changes

1. Delete `tt/rbw-Rl.RetrieverListsImages.sh`
2. Remove `rbf_list()` from `Tools/rbw/rbf_Foundry.sh`
3. Remove zipper enrollment `RBZ_LIST_IMAGES` from `Tools/rbw/rbz_zipper.sh`
4. Grep for any references to `rbw-Rl` or `rbf_list` and clean up

**[260308-1348] rough**

## Goal

Remove `rbw-Rl` (RetrieverListsImages) and `rbf_list`. The flat tag dump is superseded by `rbw-Dc` (DirectorChecksConsecrations) which provides structured consecration health. The Retriever fallback-to-Director credential pattern violates credential discipline.

## Changes

1. Delete `tt/rbw-Rl.RetrieverListsImages.sh`
2. Remove `rbf_list()` from `Tools/rbw/rbf_Foundry.sh`
3. Remove zipper enrollment `RBZ_LIST_IMAGES` from `Tools/rbw/rbz_zipper.sh`
4. Grep for any references to `rbw-Rl` or `rbf_list` and clean up

### fix-images-field-tag-base-mismatch (₢AkAAm) [complete]

**[260308-0928] complete**

## Problem

₢AkAAj introduced TAG_BASE = `i<inscribe_ts>-b<build_ts>` so each build gets a unique tag even when the same inscription is used multiple times. But the CB `images:` field (which triggers SLSA provenance) must be declared at inscribe time in `cloudbuild.json`, and can only reference CB substitution variables known before the build runs. `_RBGY_INSCRIBE_TIMESTAMP` is known; the build timestamp is not.

Before ₢AkAAj, TAG_BASE was just the inscribe timestamp, so `images:` matched the per-platform tags. ₢AkAAj changed TAG_BASE but couldn't update `images:` (the build timestamp isn't knowable at inscribe time), so it removed `images:` entirely — breaking SLSA provenance. This was a latent regression: the committed `cloudbuild.json` files still had `images:` from a prior inscribe, so provenance kept working until ₢AkAAZ re-inscribed.

## Regression chain

- `8878f887` (₢AlAAS): `images:` present in stitch, provenance working
- `c7c91636` (₢AkAAj): `images:` removed from stitch (TAG_BASE mismatch), latent regression
- `2127c3ed` (₢AkAAZ): first re-inscribe, regression exposed — builds succeed but no provenance
- `ecc06341` (₢AkAAZ): `images:` restored in stitch, but tag mismatch causes `PUSH_IMAGE_NOT_FOUND` build failure

## User's desired design direction

One inscription should support multiple distinct builds, each producing images with unique tags. The build timestamp should be captured early in the CB step sequence, then used to rename/retag the per-platform images. Concurrent builds of the same vessel are not a concern.

Key idea: the pullback step (rbgjb04) currently tags images as `TAG_BASE-image-<platform>`. It should ALSO tag them with the `images:`-declared tag (`_RBGY_INSCRIBE_TIMESTAMP + ARK_SUFFIX_IMAGE + platform_suffix`) so CB's native `images:` push can find them. The CB push creates the provenance-attested copies; the TAG_BASE-tagged copies in the registry (from step 05 push) carry the unique build identity.

Alternatively: step 04 could tag with the `images:`-compatible name, step 05 pushes those (CB `images:` also pushes them with provenance), and a later step retags in the registry with the full TAG_BASE for consumer-facing uniqueness.

## Current state

The `images:` field is currently restored in the stitch function but causes build failures. Before executing this pace, either:
- Revert the `images:` restoration so builds work (without provenance), or
- Leave it and accept that conjure will fail until this pace is completed

## Files

- `Tools/rbw/rbf_Foundry.sh` — stitch function `images:` generation
- `Tools/rbw/rbgjb/rbgjb04-per-platform-pullback.sh` — docker tag commands
- `Tools/rbw/rbgjb/rbgjb05-push-per-platform.sh` — docker push commands
- Possibly `Tools/rbw/rbgjb/rbgjb01-derive-tag-base.sh` — TAG_BASE construction

## Acceptance criteria

- `images:` field present in `cloudbuild.json` with inscribe-time-predictable tags
- Per-platform images in local daemon tagged to match `images:` entries
- Builds succeed with `VERIFIED` and produce SLSA Level 3 provenance
- Consumer-facing tags in the registry include the build timestamp for uniqueness
- Multiple conjures from the same inscription produce distinct, independently-provenance-attested images

**[260307-2101] rough**

## Problem

₢AkAAj introduced TAG_BASE = `i<inscribe_ts>-b<build_ts>` so each build gets a unique tag even when the same inscription is used multiple times. But the CB `images:` field (which triggers SLSA provenance) must be declared at inscribe time in `cloudbuild.json`, and can only reference CB substitution variables known before the build runs. `_RBGY_INSCRIBE_TIMESTAMP` is known; the build timestamp is not.

Before ₢AkAAj, TAG_BASE was just the inscribe timestamp, so `images:` matched the per-platform tags. ₢AkAAj changed TAG_BASE but couldn't update `images:` (the build timestamp isn't knowable at inscribe time), so it removed `images:` entirely — breaking SLSA provenance. This was a latent regression: the committed `cloudbuild.json` files still had `images:` from a prior inscribe, so provenance kept working until ₢AkAAZ re-inscribed.

## Regression chain

- `8878f887` (₢AlAAS): `images:` present in stitch, provenance working
- `c7c91636` (₢AkAAj): `images:` removed from stitch (TAG_BASE mismatch), latent regression
- `2127c3ed` (₢AkAAZ): first re-inscribe, regression exposed — builds succeed but no provenance
- `ecc06341` (₢AkAAZ): `images:` restored in stitch, but tag mismatch causes `PUSH_IMAGE_NOT_FOUND` build failure

## User's desired design direction

One inscription should support multiple distinct builds, each producing images with unique tags. The build timestamp should be captured early in the CB step sequence, then used to rename/retag the per-platform images. Concurrent builds of the same vessel are not a concern.

Key idea: the pullback step (rbgjb04) currently tags images as `TAG_BASE-image-<platform>`. It should ALSO tag them with the `images:`-declared tag (`_RBGY_INSCRIBE_TIMESTAMP + ARK_SUFFIX_IMAGE + platform_suffix`) so CB's native `images:` push can find them. The CB push creates the provenance-attested copies; the TAG_BASE-tagged copies in the registry (from step 05 push) carry the unique build identity.

Alternatively: step 04 could tag with the `images:`-compatible name, step 05 pushes those (CB `images:` also pushes them with provenance), and a later step retags in the registry with the full TAG_BASE for consumer-facing uniqueness.

## Current state

The `images:` field is currently restored in the stitch function but causes build failures. Before executing this pace, either:
- Revert the `images:` restoration so builds work (without provenance), or
- Leave it and accept that conjure will fail until this pace is completed

## Files

- `Tools/rbw/rbf_Foundry.sh` — stitch function `images:` generation
- `Tools/rbw/rbgjb/rbgjb04-per-platform-pullback.sh` — docker tag commands
- `Tools/rbw/rbgjb/rbgjb05-push-per-platform.sh` — docker push commands
- Possibly `Tools/rbw/rbgjb/rbgjb01-derive-tag-base.sh` — TAG_BASE construction

## Acceptance criteria

- `images:` field present in `cloudbuild.json` with inscribe-time-predictable tags
- Per-platform images in local daemon tagged to match `images:` entries
- Builds succeed with `VERIFIED` and produce SLSA Level 3 provenance
- Consumer-facing tags in the registry include the build timestamp for uniqueness
- Multiple conjures from the same inscription produce distinct, independently-provenance-attested images

### fix-consumer-image-tag-uses-full-consecration (₢AkAAj) [complete]

**[260307-1819] complete**

Fix consumer image tag to use full consecration (TAG_BASE) instead of inscribe timestamp only.

## Problem

The cloudbuild.json step 09 (imagetools-create) constructs the consumer-facing image tag as:
```
CONSUMER_TAG="${_RBGY_INSCRIBE_TIMESTAMP}${_RBGY_ARK_SUFFIX_IMAGE}"
```

This produces tags like `i20260306_125145-image`, dropping the build timestamp (`-b20260307_HHMMSS`). But `rbob_bottle.sh` constructs image references as `${RBRN_*_CONSECRATION}${RBGC_ARK_SUFFIX_IMAGE}`, expecting the full consecration in the tag.

Result: nameplate consecrations don't match actual image tags. Two builds from the same inscribed JSON produce identical consumer tags, making them indistinguishable.

## Fix

### Pipeline side (cloudbuild.json, all vessels)

Step 09 (imagetools-create): Change CONSUMER_TAG to use TAG_BASE (the full consecration) instead of _RBGY_INSCRIBE_TIMESTAMP.

Step 04 (per-platform-pullback) and Step 05 (push-per-platform): Same fix — per-platform tags should also use TAG_BASE so imagetools-create can reference them.

TAG_BASE is already derived in step 01 and written to `.tag_base`. Steps that need it can read from that file (steps 07 and 08 already do this).

### Stitch function (rbf_Foundry.sh)

The inscribe/stitch function generates the cloudbuild.json substitutions. Verify that the stitch templates pass through correctly — the fix is in the step scripts themselves, not substitutions, since TAG_BASE is derived at build time.

### Validation

- Rebuild one vessel (rbev-busybox is fastest) and verify consumer tag is `{consecration}-image`
- Summon and verify rbob_start can find the image
- Update nameplate consecrations to match

## Scope

All vessel cloudbuild.json files (7 files in rbev-vessels/).
The stitch function that generates them (rbf_Foundry.sh).
Possibly rbf_summon if it also assumes inscribe-only tags.

**[260307-1344] rough**

Fix consumer image tag to use full consecration (TAG_BASE) instead of inscribe timestamp only.

## Problem

The cloudbuild.json step 09 (imagetools-create) constructs the consumer-facing image tag as:
```
CONSUMER_TAG="${_RBGY_INSCRIBE_TIMESTAMP}${_RBGY_ARK_SUFFIX_IMAGE}"
```

This produces tags like `i20260306_125145-image`, dropping the build timestamp (`-b20260307_HHMMSS`). But `rbob_bottle.sh` constructs image references as `${RBRN_*_CONSECRATION}${RBGC_ARK_SUFFIX_IMAGE}`, expecting the full consecration in the tag.

Result: nameplate consecrations don't match actual image tags. Two builds from the same inscribed JSON produce identical consumer tags, making them indistinguishable.

## Fix

### Pipeline side (cloudbuild.json, all vessels)

Step 09 (imagetools-create): Change CONSUMER_TAG to use TAG_BASE (the full consecration) instead of _RBGY_INSCRIBE_TIMESTAMP.

Step 04 (per-platform-pullback) and Step 05 (push-per-platform): Same fix — per-platform tags should also use TAG_BASE so imagetools-create can reference them.

TAG_BASE is already derived in step 01 and written to `.tag_base`. Steps that need it can read from that file (steps 07 and 08 already do this).

### Stitch function (rbf_Foundry.sh)

The inscribe/stitch function generates the cloudbuild.json substitutions. Verify that the stitch templates pass through correctly — the fix is in the step scripts themselves, not substitutions, since TAG_BASE is derived at build time.

### Validation

- Rebuild one vessel (rbev-busybox is fastest) and verify consumer tag is `{consecration}-image`
- Summon and verify rbob_start can find the image
- Update nameplate consecrations to match

## Scope

All vessel cloudbuild.json files (7 files in rbev-vessels/).
The stitch function that generates them (rbf_Foundry.sh).
Possibly rbf_summon if it also assumes inscribe-only tags.

### extract-rbdc-derived-constants (₢AkAAh) [complete]

**[260307-1251] complete**

Create RBDC (Derived Constants) module to hold credential file path constants currently in RBRR's lock function.

## Problem

RBRR's zrbrr_lock() derives 4 credential file paths from RBRR_SECRETS_DIR:
- RBRR_GOVERNOR_RBRA_FILE
- RBRR_RETRIEVER_RBRA_FILE
- RBRR_DIRECTOR_RBRA_FILE
- RBRR_PAYOR_RBRO_FILE

These are convention constants (hardcoded filenames + runtime path) that serve as a directory service for other regimes' credential files. They don't belong in RBRR — RBRR is a config regime, not a path resolver.

## Deliverables

1. Create `Tools/rbw/rbdc_DerivedConstants.sh` with zrbdc_kindle() that:
   - Requires RBRR kindled (needs RBRR_SECRETS_DIR)
   - Declares readonly RBDC_GOVERNOR_RBRA_FILE, RBDC_RETRIEVER_RBRA_FILE, RBDC_DIRECTOR_RBRA_FILE, RBDC_PAYOR_RBRO_FILE
   - Sentinel function zrbdc_sentinel()

2. Remove the 4 file path derivations from zrbrr_lock() (ZRBRR_DOCKER_ENV stays — it's genuinely RBRR-derived)

3. Update all ~40 consumer sites to use RBDC_ prefix instead of RBRR_
   - rbgg_Governor.sh, rbf_Foundry.sh, rbro_regime.sh, rbro_cli.sh, rbgu_Utility.sh, rbi_Image.sh, rbra_cli.sh, rbgp_Payor.sh, rbap_AccessProbe.sh, butcrg_RegimeCredentials.sh

4. Update all cli furnish functions to source rbdc and call zrbdc_kindle() after zrbrr_kindle/enforce

5. Update CLAUDE.md acronym mappings

## Prefix Mint

- RBDC: rbd (new non-terminal) + c (constants by domain convention)
- Variables: RBDC_*
- Functions: zrbdc_kindle(), zrbdc_sentinel()
- No .env file — pure kindle-time derivation

## Acceptance

- No RBRR_*_RBRA_FILE or RBRR_PAYOR_RBRO_FILE references remain
- All tests pass (rbw-ta)
- CLAUDE.md updated with RBDC mapping

**[260307-1233] rough**

Create RBDC (Derived Constants) module to hold credential file path constants currently in RBRR's lock function.

## Problem

RBRR's zrbrr_lock() derives 4 credential file paths from RBRR_SECRETS_DIR:
- RBRR_GOVERNOR_RBRA_FILE
- RBRR_RETRIEVER_RBRA_FILE
- RBRR_DIRECTOR_RBRA_FILE
- RBRR_PAYOR_RBRO_FILE

These are convention constants (hardcoded filenames + runtime path) that serve as a directory service for other regimes' credential files. They don't belong in RBRR — RBRR is a config regime, not a path resolver.

## Deliverables

1. Create `Tools/rbw/rbdc_DerivedConstants.sh` with zrbdc_kindle() that:
   - Requires RBRR kindled (needs RBRR_SECRETS_DIR)
   - Declares readonly RBDC_GOVERNOR_RBRA_FILE, RBDC_RETRIEVER_RBRA_FILE, RBDC_DIRECTOR_RBRA_FILE, RBDC_PAYOR_RBRO_FILE
   - Sentinel function zrbdc_sentinel()

2. Remove the 4 file path derivations from zrbrr_lock() (ZRBRR_DOCKER_ENV stays — it's genuinely RBRR-derived)

3. Update all ~40 consumer sites to use RBDC_ prefix instead of RBRR_
   - rbgg_Governor.sh, rbf_Foundry.sh, rbro_regime.sh, rbro_cli.sh, rbgu_Utility.sh, rbi_Image.sh, rbra_cli.sh, rbgp_Payor.sh, rbap_AccessProbe.sh, butcrg_RegimeCredentials.sh

4. Update all cli furnish functions to source rbdc and call zrbdc_kindle() after zrbrr_kindle/enforce

5. Update CLAUDE.md acronym mappings

## Prefix Mint

- RBDC: rbd (new non-terminal) + c (constants by domain convention)
- Variables: RBDC_*
- Functions: zrbdc_kindle(), zrbdc_sentinel()
- No .env file — pure kindle-time derivation

## Acceptance

- No RBRR_*_RBRA_FILE or RBRR_PAYOR_RBRO_FILE references remain
- All tests pass (rbw-ta)
- CLAUDE.md updated with RBDC mapping

### rebuild-vessels-fix-nameplate-tests (₢AkAAg) [complete]

**[260307-1351] complete**

Rebuild container vessels and fix the 3 failing nameplate test fixtures.

## Problem

nsproto-security, srjcl-jupyter, and pluml-diagram fixtures fail with "container is not running" errors. The container images need rebuilding/re-summoning.

## Steps

1. Rebuild/re-summon vessel images for nsproto, srjcl, pluml
2. Verify all three nameplate fixtures pass in rbw-ta

## Acceptance

- nsproto-security passes
- srjcl-jupyter passes
- pluml-diagram passes

**[260307-1219] rough**

Rebuild container vessels and fix the 3 failing nameplate test fixtures.

## Problem

nsproto-security, srjcl-jupyter, and pluml-diagram fixtures fail with "container is not running" errors. The container images need rebuilding/re-summoning.

## Steps

1. Rebuild/re-summon vessel images for nsproto, srjcl, pluml
2. Verify all three nameplate fixtures pass in rbw-ta

## Acceptance

- nsproto-security passes
- srjcl-jupyter passes
- pluml-diagram passes

### regime-lock-in-kindle-antipattern (₢AkAAf) [complete]

**[260307-1945] complete**

Move buv_lock from separate zXXX_lock() functions into zXXX_kindle() for all regimes, and strip readonly from .env files.

## Done

- RBRN: buv_lock moved into kindle, buv_docker_env moved into enforce, zrbrn_lock deleted, 3 caller sites cleaned. Tested green (regime-smoke, access-probe, enrollment-validation all pass).

## Remaining

### 1. Strip readonly from rbrr.env

Remove `readonly` prefix from every variable in `.rbk/rbrr.env`. buv_lock is the single locking mechanism.

### 2. Fix rbrr_cli.sh grep/sed patterns

- Line 139: grep pattern matches `readonly RBRR_CRANE_TAR_GZ=` — update to match without readonly
- Line 148: sed replacement emits `readonly RBRR_CRANE_TAR_GZ=` — emit without readonly
- Line 248: sed replacement emits `readonly RBRR_GCB_PINS_REFRESHED_AT=` — emit without readonly

### 3. Fix rbgp_Payor.sh copy/paste help

Lines 1184-1188: buc_bare emits `readonly RBRR_*=...` for user copy/paste. Remove readonly prefix from output.

### 4. Transform remaining 10 regimes

For each: move buv_lock into kindle, delete zXXX_lock function, remove all caller sites.

- Simple (just buv_lock): RBRO, RBRA, RBRP, RBRS, RBRV, BURD, BURC, BURS, BURE (9 regimes)
- With derived state: RBRR (file paths + docker env move to enforce)

Approx 37 caller sites across cli/test/launcher files.

### 5. Fix RBRO render readonly conflict

rbro_cli.sh rbro_render() mutates RBRO_CLIENT_SECRET after lock. Pre-existing bug, now exposed. Needs redesign of masking approach (copy to local, render from locals, or pass mask flag to buv_render).

## Acceptance Criteria

- No regime has a separate zXXX_lock() function
- buv_lock called inside zXXX_kindle() for every regime
- No .env file uses readonly prefix
- All grep/sed/copy-paste helpers emit without readonly
- rbw-ta passes (all fixtures including regime-credentials)
- RBRO render works with locked variables

**[260307-1218] rough**

Move buv_lock from separate zXXX_lock() functions into zXXX_kindle() for all regimes, and strip readonly from .env files.

## Done

- RBRN: buv_lock moved into kindle, buv_docker_env moved into enforce, zrbrn_lock deleted, 3 caller sites cleaned. Tested green (regime-smoke, access-probe, enrollment-validation all pass).

## Remaining

### 1. Strip readonly from rbrr.env

Remove `readonly` prefix from every variable in `.rbk/rbrr.env`. buv_lock is the single locking mechanism.

### 2. Fix rbrr_cli.sh grep/sed patterns

- Line 139: grep pattern matches `readonly RBRR_CRANE_TAR_GZ=` — update to match without readonly
- Line 148: sed replacement emits `readonly RBRR_CRANE_TAR_GZ=` — emit without readonly
- Line 248: sed replacement emits `readonly RBRR_GCB_PINS_REFRESHED_AT=` — emit without readonly

### 3. Fix rbgp_Payor.sh copy/paste help

Lines 1184-1188: buc_bare emits `readonly RBRR_*=...` for user copy/paste. Remove readonly prefix from output.

### 4. Transform remaining 10 regimes

For each: move buv_lock into kindle, delete zXXX_lock function, remove all caller sites.

- Simple (just buv_lock): RBRO, RBRA, RBRP, RBRS, RBRV, BURD, BURC, BURS, BURE (9 regimes)
- With derived state: RBRR (file paths + docker env move to enforce)

Approx 37 caller sites across cli/test/launcher files.

### 5. Fix RBRO render readonly conflict

rbro_cli.sh rbro_render() mutates RBRO_CLIENT_SECRET after lock. Pre-existing bug, now exposed. Needs redesign of masking approach (copy to local, render from locals, or pass mask flag to buv_render).

## Acceptance Criteria

- No regime has a separate zXXX_lock() function
- buv_lock called inside zXXX_kindle() for every regime
- No .env file uses readonly prefix
- All grep/sed/copy-paste helpers emit without readonly
- rbw-ta passes (all fixtures including regime-credentials)
- RBRO render works with locked variables

**[260306-1353] rough**

Move buv_lock from separate zXXX_lock() functions into zXXX_kindle() for all regimes.

The current pattern has a separate lock step (kindle → enforce → lock) but there is no ordering reason for lock to be after enforce: if values are invalid, enforce dies and locked invalid values are never consumed. Derived state computations (e.g., buv_docker_env in RBRN) only read variables, so they work fine after lock.

## Approach

1. Start with RBRN (rbrn_regime.sh): move buv_lock into zrbrn_kindle(), move buv_docker_env into zrbrn_enforce() or after it, remove zrbrn_lock(). Update all callers.
2. Run full test suite — RBRN has the most coverage.
3. If green, apply same pattern to all remaining regimes: RBRR, RBRA, BURD, BURS, and any others with the separate lock step.
4. RBRO already fixed (lock in kindle, no separate lock step).

## Acceptance Criteria

- No regime has a separate zXXX_lock() function
- buv_lock called inside zXXX_kindle() for every regime
- All existing tests pass
- Callers that called zXXX_lock() updated (remove the call)

### regime-lock-in-kindle-antipattern (₢AkAAe) [abandoned]

**[260307-1145] abandoned**

Move buv_lock from separate zXXX_lock() functions into zXXX_kindle() for all regimes.

The current pattern has a separate lock step (kindle → enforce → lock) but there is no ordering reason for lock to be after enforce: if values are invalid, enforce dies and locked invalid values are never consumed. Derived state computations (e.g., buv_docker_env in RBRN) only read variables, so they work fine after lock.

## Approach

1. Start with RBRN (rbrn_regime.sh): move buv_lock into zrbrn_kindle(), move buv_docker_env into zrbrn_enforce() or after it, remove zrbrn_lock(). Update all callers.
2. Run full test suite — RBRN has the most coverage.
3. If green, apply same pattern to all remaining regimes: RBRR, RBRA, BURD, BURS, and any others with the separate lock step.
4. RBRO already fixed (lock in kindle, no separate lock step).

## Acceptance Criteria

- No regime has a separate zXXX_lock() function
- buv_lock called inside zXXX_kindle() for every regime
- All existing tests pass
- Callers that called zXXX_lock() updated (remove the call)

**[260306-1347] rough**

Move buv_lock from separate zXXX_lock() functions into zXXX_kindle() for all regimes.

The current pattern has a separate lock step (kindle → enforce → lock) but there is no ordering reason for lock to be after enforce: if values are invalid, enforce dies and locked invalid values are never consumed. Derived state computations (e.g., buv_docker_env in RBRN) only read variables, so they work fine after lock.

## Approach

1. Start with RBRN (rbrn_regime.sh): move buv_lock into zrbrn_kindle(), move buv_docker_env into zrbrn_enforce() or after it, remove zrbrn_lock(). Update all callers.
2. Run full test suite — RBRN has the most coverage.
3. If green, apply same pattern to all remaining regimes: RBRR, RBRA, BURD, BURS, and any others with the separate lock step.
4. RBRO already fixed (lock in kindle, no separate lock step).

## Acceptance Criteria

- No regime has a separate zXXX_lock() function
- buv_lock called inside zXXX_kindle() for every regime
- All existing tests pass
- Callers that called zXXX_lock() updated (remove the call)

### vouch-implementation-from-spec (₢AkAAr) [complete]

**[260309-0859] complete**

## Goal

Implement `rbf_vouch` function per the revised RBSAV-ark_vouch.adoc specification.

## Context

RBSAV has been revised through three commits in this pace: style alignment with sibling subdocs, step decomposition, digest discovery folded into build step 2, substitution variable inventory added, source URI format documented (`git+` prefix). All spec questions resolved with user (predicate version auto-detection, FROM scratch not oras, default pool, no gcloud).

Mechanical wiring already exists from the prior implementation attempt (₢AkAAo): `RBGC_ARK_SUFFIX_VOUCH` constant, zipper enrollment, vouch handling in `rbf_abjure`, vouch column in `rbf_check_consecrations`. The tabtarget rename (`rbw-Rv` → `rbw-DV`) is scoped to ₢AkAAs.

## Remaining Work

Implement `rbf_vouch()` in `rbf_Foundry.sh` following the spec:

1. Validate vessel dir + consecration params
2. Authenticate as Director, get OAuth token
3. Gate: HEAD for `-about` (require 200), HEAD for `-vouch` (warn if re-vouch)
4. Construct Build resource JSON: 3 steps, Mason SA, `_RBGV_*` substitutions, default pool
5. POST `builds.create`, extract build ID
6. Poll for completion (reuse `zrbf_wait_build_completion` pattern)
7. Display results

The three Cloud Build steps (spec-defined):
- Step 1 (Alpine): download slsa-verifier, verify SHA256
- Step 2 (Alpine): auth to GAR via metadata server, discover per-platform digests from manifest list, run `slsa-verifier verify-image` per platform
- Step 3 (Docker): assemble vouch_summary.json + per-platform results, build FROM scratch container, push as `-vouch`

## Files

- `Tools/rbw/rbf_Foundry.sh` — implement `rbf_vouch()`
- `lenses/RBSAV-ark_vouch.adoc` — minor fixes if discovered during implementation

**[260308-1349] rough**

## Goal

Implement `rbf_vouch` function per the revised RBSAV-ark_vouch.adoc specification.

## Context

RBSAV has been revised through three commits in this pace: style alignment with sibling subdocs, step decomposition, digest discovery folded into build step 2, substitution variable inventory added, source URI format documented (`git+` prefix). All spec questions resolved with user (predicate version auto-detection, FROM scratch not oras, default pool, no gcloud).

Mechanical wiring already exists from the prior implementation attempt (₢AkAAo): `RBGC_ARK_SUFFIX_VOUCH` constant, zipper enrollment, vouch handling in `rbf_abjure`, vouch column in `rbf_check_consecrations`. The tabtarget rename (`rbw-Rv` → `rbw-DV`) is scoped to ₢AkAAs.

## Remaining Work

Implement `rbf_vouch()` in `rbf_Foundry.sh` following the spec:

1. Validate vessel dir + consecration params
2. Authenticate as Director, get OAuth token
3. Gate: HEAD for `-about` (require 200), HEAD for `-vouch` (warn if re-vouch)
4. Construct Build resource JSON: 3 steps, Mason SA, `_RBGV_*` substitutions, default pool
5. POST `builds.create`, extract build ID
6. Poll for completion (reuse `zrbf_wait_build_completion` pattern)
7. Display results

The three Cloud Build steps (spec-defined):
- Step 1 (Alpine): download slsa-verifier, verify SHA256
- Step 2 (Alpine): auth to GAR via metadata server, discover per-platform digests from manifest list, run `slsa-verifier verify-image` per platform
- Step 3 (Docker): assemble vouch_summary.json + per-platform results, build FROM scratch container, push as `-vouch`

## Files

- `Tools/rbw/rbf_Foundry.sh` — implement `rbf_vouch()`
- `lenses/RBSAV-ark_vouch.adoc` — minor fixes if discovered during implementation

**[260308-1235] rough**

## Goal

Implement rbf_vouch and supporting infrastructure per RBSAV-ark_vouch.adoc specification.

## Context

₢AkAAo produced the vouch spec subdoc after a failed implementation attempt that discovered critical constraints (default pool, metadata server auth, no gcloud in bash, GCB dollar escaping). The spec captures all lessons learned. This pace implements from the reviewed spec rather than ad-hoc coding.

## Approach

1. Review the last two committed versions of RBSAV-ark_vouch.adoc (the original and the revision from ₢AkAAo wrap). Identify any gaps or ambiguities that need resolution before coding.
2. Critically review RBSAV against the improvement suggestions from the ₢AkAAo session (GCB escaping, builder-id, Alpine capabilities, step auth isolation, source URI format, re-vouch comparison, Mason permissions, prerequisites). For each complication, ask: is there a simpler way?
3. Discuss findings with user before writing any code.
4. Implement: RBGC_ARK_SUFFIX_VOUCH constant, zipper enrollment, vouch in abjure, vouch column in check_consecrations, rbf_vouch function, rbw-Rv tabtarget.

## Files (expected)

- lenses/RBSAV-ark_vouch.adoc (review, possible updates)
- lenses/RBS0-SpecTop.adoc (review)
- Tools/rbw/rbgc_Constants.sh
- Tools/rbw/rbz_zipper.sh
- Tools/rbw/rbf_Foundry.sh
- tt/rbw-Rv.RetrieverVouchesArk.sh

**[260308-1224] rough**

## Goal

Implement rbf_vouch and supporting infrastructure per RBSAV-ark_vouch.adoc specification.

## Context

₢AkAAo produced the vouch spec subdoc after a failed implementation attempt that discovered critical constraints (default pool, metadata server auth, no gcloud in bash, GCB dollar escaping). The spec captures all lessons learned. This pace implements from the reviewed spec rather than ad-hoc coding.

## Approach

Before writing any code, critically review RBSAV-ark_vouch.adoc and the vouch sections of RBS0-SpecTop.adoc. For each complication, ask: is there a simpler way? Discuss with user before proceeding.

Then implement: RBGC_ARK_SUFFIX_VOUCH constant, zipper enrollment, vouch in abjure, vouch column in check_consecrations, rbf_vouch function, rbw-Rv tabtarget.

## Files (expected)

- Tools/rbw/rbgc_Constants.sh
- Tools/rbw/rbz_zipper.sh
- Tools/rbw/rbf_Foundry.sh
- tt/rbw-Rv.RetrieverVouchesArk.sh

### director-batch-vouch-and-health-display (₢AkAAs) [complete]

**[260309-1843] complete**

## Goal

Add `rbw-DV` zero-arg batch vouch operation and enhance `rbw-Dc` (check consecrations) with vouch health recommendations.

## Specification

Create subdocument for the batch vouch operation. Proposed name: mint during pace (needs terminal exclusivity check). The spec should cover:

- **Batch vouch operation** (`rbw-DV`, zero-arg): Enumerate all vessels, find all consecrations with `-about` but no `-vouch`, submit `rbf_vouch` for each. No vessel parameter. Director auth.
- **Consecration health states** as displayed by `rbw-Dc`:
  - *vouched*: has `-about` + `-vouch` — healthy
  - *vouchable*: has `-about`, no `-vouch`, provenance exists — recommend `rbw-DV`
  - *unvouchable*: has `-about`, no `-vouch`, no SLSA provenance — skip with warning (older builds may lack provenance; batch vouch must not abort on these)
  - *incomplete*: has `-image` but no `-about` — conjure didn't finish, recommend `rbw-DA`
- **Recommendation display**: Use `buc_tabtarget` at bottom of `rbw-Dc` output when vouchable or incomplete consecrations exist

## No-provenance resilience

Batch vouch MUST handle consecrations lacking SLSA provenance gracefully:
- Skip with a clear warning message (not the opaque `invalid DSSE envelope payload`)
- Continue processing remaining consecrations
- Summary at end: N vouched, M skipped (no provenance)
- Consider pre-checking provenance availability before submitting Cloud Build (avoids wasting a build on a consecration that will fail)

## Implementation

1. Rename `rbw-Rv.RetrieverVouchesArk.sh` → `rbw-DV.DirectorVouchesArk.sh`
2. Update zipper enrollment from `rbw-Rv` to `rbw-DV`
3. Implement batch orchestration in new function (or modify `rbf_vouch` to support zero-arg batch mode)
4. Enhance `rbf_check_consecrations` with health categorization and `buc_tabtarget` recommendations

## Files (expected)

- `lenses/RBS??-*.adoc` — new subdocument (name TBD)
- `lenses/RBS0-SpecTop.adoc` — include directive, update vouch display description
- `Tools/rbw/rbf_Foundry.sh` — batch vouch + check_consecrations enhancement
- `Tools/rbw/rbz_zipper.sh` — update enrollment colophon
- `tt/rbw-DV.DirectorVouchesArk.sh` — new tabtarget (rename from rbw-Rv)
- `tt/rbw-Rv.RetrieverVouchesArk.sh` — delete

**[260309-0859] rough**

## Goal

Add `rbw-DV` zero-arg batch vouch operation and enhance `rbw-Dc` (check consecrations) with vouch health recommendations.

## Specification

Create subdocument for the batch vouch operation. Proposed name: mint during pace (needs terminal exclusivity check). The spec should cover:

- **Batch vouch operation** (`rbw-DV`, zero-arg): Enumerate all vessels, find all consecrations with `-about` but no `-vouch`, submit `rbf_vouch` for each. No vessel parameter. Director auth.
- **Consecration health states** as displayed by `rbw-Dc`:
  - *vouched*: has `-about` + `-vouch` — healthy
  - *vouchable*: has `-about`, no `-vouch`, provenance exists — recommend `rbw-DV`
  - *unvouchable*: has `-about`, no `-vouch`, no SLSA provenance — skip with warning (older builds may lack provenance; batch vouch must not abort on these)
  - *incomplete*: has `-image` but no `-about` — conjure didn't finish, recommend `rbw-DA`
- **Recommendation display**: Use `buc_tabtarget` at bottom of `rbw-Dc` output when vouchable or incomplete consecrations exist

## No-provenance resilience

Batch vouch MUST handle consecrations lacking SLSA provenance gracefully:
- Skip with a clear warning message (not the opaque `invalid DSSE envelope payload`)
- Continue processing remaining consecrations
- Summary at end: N vouched, M skipped (no provenance)
- Consider pre-checking provenance availability before submitting Cloud Build (avoids wasting a build on a consecration that will fail)

## Implementation

1. Rename `rbw-Rv.RetrieverVouchesArk.sh` → `rbw-DV.DirectorVouchesArk.sh`
2. Update zipper enrollment from `rbw-Rv` to `rbw-DV`
3. Implement batch orchestration in new function (or modify `rbf_vouch` to support zero-arg batch mode)
4. Enhance `rbf_check_consecrations` with health categorization and `buc_tabtarget` recommendations

## Files (expected)

- `lenses/RBS??-*.adoc` — new subdocument (name TBD)
- `lenses/RBS0-SpecTop.adoc` — include directive, update vouch display description
- `Tools/rbw/rbf_Foundry.sh` — batch vouch + check_consecrations enhancement
- `Tools/rbw/rbz_zipper.sh` — update enrollment colophon
- `tt/rbw-DV.DirectorVouchesArk.sh` — new tabtarget (rename from rbw-Rv)
- `tt/rbw-Rv.RetrieverVouchesArk.sh` — delete

**[260308-1344] rough**

## Goal

Add `rbw-DV` zero-arg batch vouch operation and enhance `rbw-Dc` (check consecrations) with vouch health recommendations.

## Specification

Create subdocument for the batch vouch operation. Proposed name: mint during pace (needs terminal exclusivity check). The spec should cover:

- **Batch vouch operation** (`rbw-DV`, zero-arg): Enumerate all vessels, find all consecrations with `-about` but no `-vouch`, submit `rbf_vouch` for each. No vessel parameter. Director auth.
- **Consecration health states** as displayed by `rbw-Dc`:
  - *vouched*: has `-about` + `-vouch` — healthy
  - *vouchable*: has `-about`, no `-vouch` — recommend `rbw-DV`
  - *incomplete*: has `-image` but no `-about` — conjure didn't finish, recommend `rbw-DA`
- **Recommendation display**: Use `buc_tabtarget` at bottom of `rbw-Dc` output when vouchable or incomplete consecrations exist

## Implementation

1. Rename `rbw-Rv.RetrieverVouchesArk.sh` → `rbw-DV.DirectorVouchesArk.sh`
2. Update zipper enrollment from `rbw-Rv` to `rbw-DV`
3. Implement batch orchestration in new function (or modify `rbf_vouch` to support zero-arg batch mode)
4. Enhance `rbf_check_consecrations` with health categorization and `buc_tabtarget` recommendations

## Files (expected)

- `lenses/RBS??-*.adoc` — new subdocument (name TBD)
- `lenses/RBS0-SpecTop.adoc` — include directive, update vouch display description
- `Tools/rbw/rbf_Foundry.sh` — batch vouch + check_consecrations enhancement
- `Tools/rbw/rbz_zipper.sh` — update enrollment colophon
- `tt/rbw-DV.DirectorVouchesArk.sh` — new tabtarget (rename from rbw-Rv)
- `tt/rbw-Rv.RetrieverVouchesArk.sh` — delete

### gcb-pin-slsa-verifier (₢AkAAp) [abandoned]

**[260308-1352] abandoned**

## Goal

Add `slsa-verifier` container image as a managed GCB pin in the regime configuration.

## Context

The upcoming vouch-via-slsa-verifier architecture (₢AkAAo) requires `slsa-verifier` to run inside a Cloud Build step. Like all CB step images (`gcr.io/cloud-builders/docker`, etc.), it must be digest-pinned for reproducibility and refreshable via `rbw-DP`.

## Source

- Image: `ghcr.io/slsa-framework/slsa-verifier` (official SLSA framework)
- Note: hosted on GitHub Container Registry, not Google's GCR — pin refresh logic may need to handle `ghcr.io` as a source registry

## Changes

- Add `slsa-verifier` pin entry to regime GCB pin configuration
- Verify `rbw-DP` (refresh GCB pins) can resolve and pin from `ghcr.io`
- If `ghcr.io` is not supported by pin refresh, extend it
- Test: refresh pins, verify slsa-verifier digest is resolved and recorded

## Files (expected)

- Regime pin configuration (rbrg.env or equivalent)
- `Tools/rbw/rbrr_cli.sh` or pin refresh implementation — if ghcr.io support needed
- `lenses/RBSRG-RegimeGcbPins.adoc` — document new pin

**[260308-1032] rough**

## Goal

Add `slsa-verifier` container image as a managed GCB pin in the regime configuration.

## Context

The upcoming vouch-via-slsa-verifier architecture (₢AkAAo) requires `slsa-verifier` to run inside a Cloud Build step. Like all CB step images (`gcr.io/cloud-builders/docker`, etc.), it must be digest-pinned for reproducibility and refreshable via `rbw-DP`.

## Source

- Image: `ghcr.io/slsa-framework/slsa-verifier` (official SLSA framework)
- Note: hosted on GitHub Container Registry, not Google's GCR — pin refresh logic may need to handle `ghcr.io` as a source registry

## Changes

- Add `slsa-verifier` pin entry to regime GCB pin configuration
- Verify `rbw-DP` (refresh GCB pins) can resolve and pin from `ghcr.io`
- If `ghcr.io` is not supported by pin refresh, extend it
- Test: refresh pins, verify slsa-verifier digest is resolved and recorded

## Files (expected)

- Regime pin configuration (rbrg.env or equivalent)
- `Tools/rbw/rbrr_cli.sh` or pin refresh implementation — if ghcr.io support needed
- `lenses/RBSRG-RegimeGcbPins.adoc` — document new pin

### vouch-slsa-verifier-architecture (₢AkAAo) [complete]

**[260308-1236] complete**

## Goal

Reintroduce `-vouch` as an authoritative SLSA verification artifact. Run `slsa-verifier verify-image` inside a Cloud Build job to cryptographically verify provenance, then push a `-vouch` container that includes `-about` content plus verification results.

## Current state

Vouch infrastructure removed (constant, zipper, tabtarget, function, spec references). `rbf_check_consecrations` is a lightweight tag-only listing showing full consecrations, platforms, and about status. No provenance queries at list time.

## Prerequisites

- GCB pin for `slsa-verifier` container image (₢AkAAp)

## Architecture

### Vouch as separate Cloud Build job
- Own tabtarget, new command (manual operator invocation, not automated)
- Uses RunBuild API with substitution variables (not inscribe — consecration unknown at inscribe time)
- Runs as Director service account
- Takes vessel + consecration as parameters
- Pulls existing `-about` artifact content (vouch depends on about existing)
- Runs `slsa-verifier verify-image` against each per-platform digest
- Assembles `-vouch` artifact: about content + cryptographic verification results
- Pushes `{CONSECRATION}-vouch` tag
- Does NOT regenerate SBOMs — reuses about's SBOMs

### Idempotent re-vouch
- On re-run: re-runs slsa-verifier, generates vouch content, pulls existing `-vouch`, compares for precise byte-level equality
- Match = stable (idempotent success)
- Mismatch = LOUD alarm (provenance changed or tampering detected)
- Vouch content presumed deterministic (same digests + same provenance + same keys = identical output)
- If determinism assumption fails in practice, narrow comparison to verification verdict + digests + builder identity

### Offline list with local vouches
- New tabtarget to pull all `-vouch` artifacts for a vessel locally (docker pull into local daemon)
- `rbf_check_consecrations` inspects local `-vouch` container images to display provenance facts per consecration
- When no local vouches exist: warn and display pull tabtarget colophon via `buc_tabtarget`
- Vouch-less consecrations display without provenance columns

### Bottle gate
- Bottle service already fetches by consecration — require `-vouch` to exist before serving
- Operator flow: conjure → vouch → deploy. No vouch, no deploy.
- Enforcement at consumer boundary, not producer boundary

### -about relationship
- `-about` stays — generated during conjure with SBOMs from local daemon
- `-vouch` is a superset: includes about content + slsa-verifier verification output
- Both artifacts persist independently

### Abjure
- Restore `-vouch` artifact check and deletion in `rbf_abjure`

## Testing

- Interactive exercise against real builds (no automated test for vouch CB job initially)
- `rbtcsl_SlsaProvenance.sh` SLSA assertions restored once vouch is operational

## Files (expected)

- `Tools/rbw/rbf_Foundry.sh` — new vouch function (RunBuild API), updated list to read local vouches, restore vouch in abjure
- `Tools/rbw/rbgc_Constants.sh` — restore `RBGC_ARK_SUFFIX_VOUCH`
- `Tools/rbw/rbz_zipper.sh` — restore vouch enrollment
- `Tools/rbw/rbob_bottle.sh` — require `-vouch` before serving
- `Tools/rbw/rbts/rbtcsl_SlsaProvenance.sh` — restore SLSA assertions using vouch
- `lenses/RBS0-SpecTop.adoc` — vouch definition with slsa-verifier semantics
- `lenses/RBSCB-CloudBuildRoadmap.adoc` — vouch architecture documentation
- New tabtarget for vouch command
- New tabtarget for pulling vouch artifacts locally
- Vouch cloudbuild.json submitted via RunBuild API

**[260308-1039] rough**

## Goal

Reintroduce `-vouch` as an authoritative SLSA verification artifact. Run `slsa-verifier verify-image` inside a Cloud Build job to cryptographically verify provenance, then push a `-vouch` container that includes `-about` content plus verification results.

## Current state

Vouch infrastructure removed (constant, zipper, tabtarget, function, spec references). `rbf_check_consecrations` is a lightweight tag-only listing showing full consecrations, platforms, and about status. No provenance queries at list time.

## Prerequisites

- GCB pin for `slsa-verifier` container image (₢AkAAp)

## Architecture

### Vouch as separate Cloud Build job
- Own tabtarget, new command (manual operator invocation, not automated)
- Uses RunBuild API with substitution variables (not inscribe — consecration unknown at inscribe time)
- Runs as Director service account
- Takes vessel + consecration as parameters
- Pulls existing `-about` artifact content (vouch depends on about existing)
- Runs `slsa-verifier verify-image` against each per-platform digest
- Assembles `-vouch` artifact: about content + cryptographic verification results
- Pushes `{CONSECRATION}-vouch` tag
- Does NOT regenerate SBOMs — reuses about's SBOMs

### Idempotent re-vouch
- On re-run: re-runs slsa-verifier, generates vouch content, pulls existing `-vouch`, compares for precise byte-level equality
- Match = stable (idempotent success)
- Mismatch = LOUD alarm (provenance changed or tampering detected)
- Vouch content presumed deterministic (same digests + same provenance + same keys = identical output)
- If determinism assumption fails in practice, narrow comparison to verification verdict + digests + builder identity

### Offline list with local vouches
- New tabtarget to pull all `-vouch` artifacts for a vessel locally (docker pull into local daemon)
- `rbf_check_consecrations` inspects local `-vouch` container images to display provenance facts per consecration
- When no local vouches exist: warn and display pull tabtarget colophon via `buc_tabtarget`
- Vouch-less consecrations display without provenance columns

### Bottle gate
- Bottle service already fetches by consecration — require `-vouch` to exist before serving
- Operator flow: conjure → vouch → deploy. No vouch, no deploy.
- Enforcement at consumer boundary, not producer boundary

### -about relationship
- `-about` stays — generated during conjure with SBOMs from local daemon
- `-vouch` is a superset: includes about content + slsa-verifier verification output
- Both artifacts persist independently

### Abjure
- Restore `-vouch` artifact check and deletion in `rbf_abjure`

## Testing

- Interactive exercise against real builds (no automated test for vouch CB job initially)
- `rbtcsl_SlsaProvenance.sh` SLSA assertions restored once vouch is operational

## Files (expected)

- `Tools/rbw/rbf_Foundry.sh` — new vouch function (RunBuild API), updated list to read local vouches, restore vouch in abjure
- `Tools/rbw/rbgc_Constants.sh` — restore `RBGC_ARK_SUFFIX_VOUCH`
- `Tools/rbw/rbz_zipper.sh` — restore vouch enrollment
- `Tools/rbw/rbob_bottle.sh` — require `-vouch` before serving
- `Tools/rbw/rbts/rbtcsl_SlsaProvenance.sh` — restore SLSA assertions using vouch
- `lenses/RBS0-SpecTop.adoc` — vouch definition with slsa-verifier semantics
- `lenses/RBSCB-CloudBuildRoadmap.adoc` — vouch architecture documentation
- New tabtarget for vouch command
- New tabtarget for pulling vouch artifacts locally
- Vouch cloudbuild.json submitted via RunBuild API

**[260308-1032] rough**

## Goal

Reintroduce `-vouch` as an authoritative SLSA verification artifact. Run `slsa-verifier verify-image` inside a Cloud Build job to cryptographically verify provenance, then push a `-vouch` container that includes `-about` content plus verification results.

## Current state

Vouch infrastructure removed (constant, zipper, tabtarget, function, spec references). `rbf_check_consecrations` is a lightweight tag-only listing showing full consecrations, platforms, and about status. No provenance queries at list time.

## Prerequisites

- GCB pin for `slsa-verifier` container image (separate prior pace)

## Architecture

### Vouch as separate Cloud Build job
- Own tabtarget, new command (manual operator invocation, not automated)
- Uses RunBuild API with substitution variables (not inscribe — consecration unknown at inscribe time)
- Takes vessel + consecration as parameters
- Pulls existing `-about` artifact content (vouch depends on about existing)
- Runs `slsa-verifier verify-image` against each per-platform digest
- Assembles `-vouch` artifact: about content + cryptographic verification results
- Pushes `{CONSECRATION}-vouch` tag
- Does NOT regenerate SBOMs — reuses about's SBOMs

### Idempotent re-vouch
- On re-run: re-runs slsa-verifier, generates vouch content, pulls existing `-vouch`, compares for precise byte-level equality
- Match = stable (idempotent success)
- Mismatch = LOUD alarm (provenance changed or tampering detected)
- Vouch content presumed deterministic (same digests + same provenance + same keys = identical output)
- If determinism assumption fails in practice, narrow comparison to verification verdict + digests + builder identity

### Offline list with local vouches
- New tabtarget to pull all `-vouch` artifacts for a vessel locally
- `rbf_check_consecrations` reads local `-vouch` content to display provenance facts per consecration
- When no local vouches exist: warn and display pull tabtarget colophon via `buc_tabtarget`
- Vouch-less consecrations display without provenance columns

### Bottle gate
- Bottle service already fetches by consecration — require `-vouch` to exist before serving
- Operator flow: conjure → vouch → deploy. No vouch, no deploy.
- Enforcement at consumer boundary, not producer boundary

### -about relationship
- `-about` stays — generated during conjure with SBOMs from local daemon
- `-vouch` is a superset: includes about content + slsa-verifier verification output
- Both artifacts persist independently

## Testing

- Interactive exercise against real builds (no automated test for vouch CB job initially)
- `rbtcsl_SlsaProvenance.sh` SLSA assertions restored once vouch is operational

## Files (expected)

- `Tools/rbw/rbf_Foundry.sh` — new vouch function (RunBuild API), updated list to read local vouches
- `Tools/rbw/rbgc_Constants.sh` — restore `RBGC_ARK_SUFFIX_VOUCH`
- `Tools/rbw/rbz_zipper.sh` — restore vouch enrollment
- `Tools/rbw/rbts/rbtcsl_SlsaProvenance.sh` — restore SLSA assertions using vouch
- `lenses/RBS0-SpecTop.adoc` — vouch definition with slsa-verifier semantics
- `lenses/RBSCB-CloudBuildRoadmap.adoc` — vouch architecture documentation
- New tabtarget for vouch command
- New tabtarget for pulling vouch artifacts locally
- Vouch cloudbuild.json submitted via RunBuild API

**[260308-1031] rough**

## Goal

Reintroduce `-vouch` as an authoritative SLSA verification artifact. Run `slsa-verifier verify-image` inside a Cloud Build job to cryptographically verify provenance, then push a `-vouch` container that includes `-about` content plus verification results.

## Current state

Vouch infrastructure removed (constant, zipper, tabtarget, function, spec references). `rbf_check_consecrations` is a lightweight tag-only listing showing full consecrations, platforms, and about status. No provenance queries at list time.

## Prerequisites

- GCB pin for `slsa-verifier` container image (separate prior pace)

## Architecture

### Vouch as separate Cloud Build job
- Own tabtarget, new command (manual operator invocation, not automated)
- Uses RunBuild API with substitution variables (not inscribe — consecration unknown at inscribe time)
- Takes vessel + consecration as parameters
- Pulls existing `-about` artifact content (vouch depends on about existing)
- Runs `slsa-verifier verify-image` against each per-platform digest
- Assembles `-vouch` artifact: about content + cryptographic verification results
- Pushes `{CONSECRATION}-vouch` tag
- Does NOT regenerate SBOMs — reuses about's SBOMs

### Idempotent re-vouch
- On re-run: re-runs slsa-verifier, generates vouch content, pulls existing `-vouch`, compares for precise byte-level equality
- Match = stable (idempotent success)
- Mismatch = LOUD alarm (provenance changed or tampering detected)
- Vouch content presumed deterministic (same digests + same provenance + same keys = identical output)
- If determinism assumption fails in practice, narrow comparison to verification verdict + digests + builder identity

### Offline list with local vouches
- New tabtarget to pull all `-vouch` artifacts for a vessel locally
- `rbf_check_consecrations` reads local `-vouch` content to display provenance facts per consecration
- When no local vouches exist: warn and display pull tabtarget colophon via `buc_tabtarget`
- Vouch-less consecrations display without provenance columns

### Bottle gate
- Bottle service already fetches by consecration — require `-vouch` to exist before serving
- Operator flow: conjure → vouch → deploy. No vouch, no deploy.
- Enforcement at consumer boundary, not producer boundary

### -about relationship
- `-about` stays — generated during conjure with SBOMs from local daemon
- `-vouch` is a superset: includes about content + slsa-verifier verification output
- Both artifacts persist independently

## Testing

- Interactive exercise against real builds (no automated test for vouch CB job initially)
- `rbtcsl_SlsaProvenance.sh` SLSA assertions restored once vouch is operational

## Files (expected)

- `Tools/rbw/rbf_Foundry.sh` — new vouch function (RunBuild API), updated list to read local vouches
- `Tools/rbw/rbgc_Constants.sh` — restore `RBGC_ARK_SUFFIX_VOUCH`
- `Tools/rbw/rbz_zipper.sh` — restore vouch enrollment
- `Tools/rbw/rbts/rbtcsl_SlsaProvenance.sh` — restore SLSA assertions using vouch
- `lenses/RBS0-SpecTop.adoc` — vouch definition with slsa-verifier semantics
- `lenses/RBSCB-CloudBuildRoadmap.adoc` — vouch architecture documentation
- New tabtarget for vouch command
- New tabtarget for pulling vouch artifacts locally
- Vouch cloudbuild.json submitted via RunBuild API

**[260308-1019] rough**

## Goal

Reintroduce `-vouch` as an authoritative SLSA verification artifact. Run `slsa-verifier verify-image` inside a Cloud Build job to cryptographically verify provenance, then push a `-vouch` container that is a superset of `-about` (SBOM + build_info + verification results).

## Current state

Vouch infrastructure has been removed (constant, zipper, tabtarget, function, spec references). `rbf_check_consecrations` is a lightweight tag-only listing showing full consecrations, platforms, and about status. No provenance queries happen at list time.

## Architecture

### Vouch as separate Cloud Build job
- Own tabtarget and trigger (not part of conjure build — can't self-verify since provenance doesn't exist until after CB finishes)
- Takes vessel + consecration as parameters
- Runs `slsa-verifier verify-image` against each per-platform digest
- Assembles `-vouch` artifact: about content + cryptographic verification results
- Pushes `{CONSECRATION}-vouch` tag

### Idempotent re-vouch
- On re-run: re-runs slsa-verifier, generates same vouch content, pulls existing `-vouch`, compares for precise equality
- Match = stable (idempotent success)
- Mismatch = LOUD alarm (provenance changed or tampering detected)
- Vouch content must be deterministic (no timestamps in artifact — same digests + same provenance + same keys = identical output)

### Offline list with local vouches
- New tabtarget to pull all `-vouch` artifacts for a vessel locally
- `rbf_check_consecrations` reads local `-vouch` content to display provenance facts per consecration (no API calls beyond tag listing)
- Vouch-less consecrations display without provenance columns

### -about relationship
- `-about` stays — generated during conjure with SBOMs from local daemon
- `-vouch` is a superset: includes about content + slsa-verifier verification output
- Both artifacts persist independently

## Files (expected)

- `Tools/rbw/rbf_Foundry.sh` — new vouch function, updated list to read local vouches
- `Tools/rbw/rbgc_Constants.sh` — restore `RBGC_ARK_SUFFIX_VOUCH`
- `Tools/rbw/rbz_zipper.sh` — restore vouch enrollment
- `Tools/rbw/rbts/rbtcsl_SlsaProvenance.sh` — restore SLSA assertions using vouch
- `lenses/RBS0-SpecTop.adoc` — vouch definition with slsa-verifier semantics
- `lenses/RBSCB-CloudBuildRoadmap.adoc` — vouch architecture documentation
- New tabtarget for vouch, new tabtarget for pulling vouches
- Vouch cloudbuild.json (static or inscribe-generated)

**[260308-0934] rough**

## Goal

Eliminate `rbf_vouch` as a separate workflow step. Make `rbf_check_consecrations` (list) query Container Analysis for SLSA provenance on demand and display it inline per consecration.

## Current state

Three-step flow: conjure → vouch (human step) → list shows vouch presence.
`rbf_vouch` queries Container Analysis, displays results, then builds and pushes a `-vouch` container to the registry. List (`rbf_check_consecrations`) groups tags by inscription timestamp but doesn't show individual full consecrations or provenance status.

## Changes

### List overhaul (`rbf_check_consecrations`)
- Parse tags to show **full consecrations** (inscription + build timestamp), not just inscription-level grouping
- For each consecration: resolve per-platform image tags to digests, query Container Analysis for SLSA attestations
- Display inline: consecration, platforms, SLSA level, build ID
- Single-origin check (all platforms share one build invocation ID)

### Remove vouch
- Remove `rbf_vouch` function from `rbf_Foundry.sh`
- Remove `-vouch` tag from tag scheme (`RBGC_ARK_SUFFIX_VOUCH`)
- Remove vouch tabtarget (`rbw-Rv`)
- Remove vouch from zipper registration
- Update `rbf_abjure` to stop looking for `-vouch` tags

### Spec updates (RBS0)
- Remove `-vouch` from tag scheme documentation
- Update consecration definition: remove vouch references
- Update pipeline description: conjure produces images + about, list verifies provenance
- Update RBSCB-CloudBuildRoadmap.adoc tag table

## Design rationale

Provenance is already in Container Analysis, keyed by digest. The `-vouch` container is a redundant snapshot of information that's queryable on demand. Eliminating it removes a human step, a failure mode (stale/forgotten vouch), and a registry artifact.

## Files

- `Tools/rbw/rbf_Foundry.sh` — overhaul list, remove vouch
- `Tools/rbw/rbz_zipper.sh` — remove vouch registration
- `Tools/rbw/rbgc_Constants.sh` — remove `RBGC_ARK_SUFFIX_VOUCH`
- `lenses/RBS0-SpecTop.adoc` — update tag scheme, remove vouch references
- `lenses/RBSCB-CloudBuildRoadmap.adoc` — update tag table
- `tt/rbw-Rv.RetrieverVouchesArk.sh` — delete

### bottle-start-auto-summon-missing (₢AkAAX) [complete]

**[260309-1137] complete**

## Goal

Ensure bottle start auto-pulls missing images instead of dying with a tabtarget hint.

Currently `rbob_start` (Tools/rbw/rbob_bottle.sh lines 310-322) checks if sentry/bottle images are local and dies with `buc_tabtarget` hints if missing. The consecration is already known from the nameplate (`RBRN_*_CONSECRATION`), and `rbf_summon` works with Retriever credentials.

## Change

When an image is missing, auto-invoke `rbf_summon` with the vessel+consecration from the nameplate instead of dying. Log what's happening so the user sees the pull activity. Die only if the summon itself fails.

**Vouch gate (hard fail):** Before summoning, HEAD request for `«CONSECRATION»-vouch` tag. If the vouch artifact does not exist, `buc_die` with a clear message: the Director has not vouched this consecration. Do not pull or run unvouched images. This is the Retriever-side enforcement of the vouch discipline — no warn-and-continue, no flag to bypass.

## Validation

Update nsproto nameplate consecrations to the new builds from ₢AkAAm:
- `RBRN_SENTRY_CONSECRATION` → sentry consecration from `i20260308_082033` inscription
- `RBRN_BOTTLE_CONSECRATION` → `i20260308_082033-b20260308_152727`

Then start nsproto — the new images won't be local, so auto-summon must fire. Success = nsproto starts without manual summon steps. Also test with an unvouched consecration to verify the hard fail.

## Files

- `Tools/rbw/rbob_bottle.sh` — replace die-with-hint blocks with vouch-gate + auto-summon
- `Tools/rbw/rbrn_nsproto.env` — update consecrations to new builds

## Design note

The Retriever role doesn't need consecration discovery — the nameplate IS the selection mechanism. Directors browse with `rbf_beseech`; Retrievers trust the committed nameplate and pull what it says. The vouch gate ensures the Director has verified provenance before any Retriever consumption.

**[260308-1344] rough**

## Goal

Ensure bottle start auto-pulls missing images instead of dying with a tabtarget hint.

Currently `rbob_start` (Tools/rbw/rbob_bottle.sh lines 310-322) checks if sentry/bottle images are local and dies with `buc_tabtarget` hints if missing. The consecration is already known from the nameplate (`RBRN_*_CONSECRATION`), and `rbf_summon` works with Retriever credentials.

## Change

When an image is missing, auto-invoke `rbf_summon` with the vessel+consecration from the nameplate instead of dying. Log what's happening so the user sees the pull activity. Die only if the summon itself fails.

**Vouch gate (hard fail):** Before summoning, HEAD request for `«CONSECRATION»-vouch` tag. If the vouch artifact does not exist, `buc_die` with a clear message: the Director has not vouched this consecration. Do not pull or run unvouched images. This is the Retriever-side enforcement of the vouch discipline — no warn-and-continue, no flag to bypass.

## Validation

Update nsproto nameplate consecrations to the new builds from ₢AkAAm:
- `RBRN_SENTRY_CONSECRATION` → sentry consecration from `i20260308_082033` inscription
- `RBRN_BOTTLE_CONSECRATION` → `i20260308_082033-b20260308_152727`

Then start nsproto — the new images won't be local, so auto-summon must fire. Success = nsproto starts without manual summon steps. Also test with an unvouched consecration to verify the hard fail.

## Files

- `Tools/rbw/rbob_bottle.sh` — replace die-with-hint blocks with vouch-gate + auto-summon
- `Tools/rbw/rbrn_nsproto.env` — update consecrations to new builds

## Design note

The Retriever role doesn't need consecration discovery — the nameplate IS the selection mechanism. Directors browse with `rbf_beseech`; Retrievers trust the committed nameplate and pull what it says. The vouch gate ensures the Director has verified provenance before any Retriever consumption.

**[260308-0911] rough**

## Goal

Ensure bottle start auto-pulls missing images instead of dying with a tabtarget hint.

Currently `rbob_start` (Tools/rbw/rbob_bottle.sh lines 310-322) checks if sentry/bottle images are local and dies with `buc_tabtarget` hints if missing. The consecration is already known from the nameplate (`RBRN_*_CONSECRATION`), and `rbf_summon` works with Retriever credentials.

## Change

When an image is missing, auto-invoke `rbf_summon` with the vessel+consecration from the nameplate instead of dying. Log what's happening so the user sees the pull activity. Die only if the summon itself fails.

## Validation

Update nsproto nameplate consecrations to the new builds from ₢AkAAm:
- `RBRN_SENTRY_CONSECRATION` → sentry consecration from `i20260308_082033` inscription
- `RBRN_BOTTLE_CONSECRATION` → `i20260308_082033-b20260308_152727`

Then start nsproto — the new images won't be local, so auto-summon must fire. Success = nsproto starts without manual summon steps.

## Files

- `Tools/rbw/rbob_bottle.sh` — replace die-with-hint blocks with auto-summon calls
- `Tools/rbw/rbrn_nsproto.env` — update consecrations to new builds

## Design note

The Retriever role doesn't need consecration discovery — the nameplate IS the selection mechanism. Directors browse with `rbf_beseech`; Retrievers trust the committed nameplate and pull what it says.

**[260304-1959] rough**

Ensure bottle start auto-pulls missing images instead of dying with a tabtarget hint.

Currently `rbob_start` (Tools/rbw/rbob_bottle.sh lines 310-322) checks if sentry/bottle images are local and dies with `buc_tabtarget` hints if missing. The consecration is already known from the nameplate (`RBRN_*_CONSECRATION`), and `rbf_summon` works with Retriever credentials.

Change: when an image is missing, auto-invoke `rbf_summon` with the vessel+consecration from the nameplate instead of dying. Log what's happening so the user sees the pull activity. Die only if the summon itself fails.

Files:
- `Tools/rbw/rbob_bottle.sh` — replace die-with-hint blocks with auto-summon calls (sentry ~line 310, bottle ~line 317)

Design note: The Retriever role doesn't need consecration discovery — the nameplate IS the selection mechanism. Directors browse with `rbf_beseech`; Retrievers trust the committed nameplate and pull what it says.

**[260304-1959] rough**

Ensure bottle start auto-pulls missing images instead of dying with a tabtarget hint.

Currently `rbob_start` (Tools/rbw/rbob_bottle.sh lines 310-322) checks if sentry/bottle images are local and dies with `buc_tabtarget` hints if missing. The consecration is already known from the nameplate (`RBRN_*_CONSECRATION`), and `rbf_summon` works with Retriever credentials.

Change: when an image is missing, auto-invoke `rbf_summon` with the vessel+consecration from the nameplate instead of dying. Log what's happening so the user sees the pull activity. Die only if the summon itself fails.

Files:
- `Tools/rbw/rbob_bottle.sh` — replace die-with-hint blocks with auto-summon calls (sentry ~line 310, bottle ~line 317)

Design note: The Retriever role doesn't need consecration discovery — the nameplate IS the selection mechanism. Directors browse with `rbf_beseech`; Retrievers trust the committed nameplate and pull what it says.

**[260304-1941] rough**

Investigate whether the Retriever should support listing consecrations (not just images). Currently `rbf_list` lists images in a depot, but there's no operation to enumerate the consecrations (tagged artifact groups) themselves. Consider:

- Is there a user need to see "what consecrations exist in this depot" vs "what images exist"?
- Would this be a distinct tabtarget or a flag on the existing list operation?
- How does this relate to the ark/vessel/consecration vocabulary in RBSRV?

This is a design musing — capture findings and recommendation, not implementation.

### rename-buc-next-and-fix-payor-refresh (₢AkAAW) [complete]

**[260304-1909] complete**

Two changes in one pace:

1. Rename `buc_next` to `buc_tabtarget` across all live code:
   - `Tools/buk/buc_command.sh` — definition + error message
   - `Tools/rbw/rbf_Foundry.sh` — 1 call site
   - `Tools/rbw/rbgp_Payor.sh` — 4 call sites
   - `Tools/rbw/rbgm_ManualProcedures.sh` — 1 call site
   - `Tools/rbw/rbob_bottle.sh` — 2 call sites

2. In `rbgm_ManualProcedures.sh`, payor_refresh procedure step 3 "Verify Installation" (~line 320): replace the raw `rbgp_depot_list` function call with guidance to use the tabtarget `tt/rbw-Pl.PayorListsDepots.sh` (discoverable via `buc_tabtarget`), consistent with BUK tabtarget-driven workflow.

Do NOT touch retired heat docs or gallops JSON — those are historical records.

**[260304-1853] rough**

Two changes in one pace:

1. Rename `buc_next` to `buc_tabtarget` across all live code:
   - `Tools/buk/buc_command.sh` — definition + error message
   - `Tools/rbw/rbf_Foundry.sh` — 1 call site
   - `Tools/rbw/rbgp_Payor.sh` — 4 call sites
   - `Tools/rbw/rbgm_ManualProcedures.sh` — 1 call site
   - `Tools/rbw/rbob_bottle.sh` — 2 call sites

2. In `rbgm_ManualProcedures.sh`, payor_refresh procedure step 3 "Verify Installation" (~line 320): replace the raw `rbgp_depot_list` function call with guidance to use the tabtarget `tt/rbw-Pl.PayorListsDepots.sh` (discoverable via `buc_tabtarget`), consistent with BUK tabtarget-driven workflow.

Do NOT touch retired heat docs or gallops JSON — those are historical records.

**[260304-1852] rough**

Two changes in one pace:

1. Rename `buc_next` to `buc_tabtarget` across all live code:
   - `Tools/buk/buc_command.sh` — definition + error message
   - `Tools/rbw/rbf_Foundry.sh` — 1 call site
   - `Tools/rbw/rbgp_Payor.sh` — 4 call sites
   - `Tools/rbw/rbgm_ManualProcedures.sh` — 1 call site
   - `Tools/rbw/rbob_bottle.sh` — 2 call sites

2. In `rbgm_ManualProcedures.sh`, payor_refresh procedure step 3 "Verify Installation" (~line 320): replace the raw `rbgp_depot_list` function call with guidance to use the tabtarget `tt/rbw-Pl.PayorListsDepots.sh` (discoverable via `buc_tabtarget`), consistent with BUK tabtarget-driven workflow.

Do NOT touch retired heat docs or gallops JSON — those are historical records.

**[260304-1850] rough**

In rbgm_ManualProcedures.sh, the payor_refresh procedure step 3 "Verify Installation" directs users to run `rbgp_depot_list` as a raw bash function call. This should instead guide users to use the existing tabtarget `tt/rbw-Pl.PayorListsDepots.sh` (discoverable via `buc_next`), consistent with the BUK tabtarget-driven workflow.

Fix the procedure text at line ~320 to reference the tabtarget instead of the raw function.

### readonly-bcg-trial (₢AkAAD) [complete]

**[260224-1127] complete**

Add `readonly` pattern to BCG and trial on selected modules/regime.

## Scope

1. **BCG update**: Add a new section to BCG covering `readonly` patterns:
   - `readonly VAR` (lock after assignment)
   - `readonly VAR=value` (assign-and-lock for script-local constants)
   - The re-source trap (why `readonly` cannot go in sourceable .env files)
   - The "lock after kindle" pattern for validated configuration

2. **Remove kindle defaults**: Eliminate the `RBRR_FOO="${RBRR_FOO:-}"` default-setting pattern from `zrbrr_kindle()`. These destroy information (unset vs empty) and are unnecessary because buv already uses `${!z_varname:-}` for safe indirect expansion under `set -u`.

3. **buv enhancement**: In `zbuv_check_capture`, add an unset-detection check using `${!z_varname+x}` before the type-switch. Unset required variables should produce a distinct error: "VARNAME is not set (missing from .env?)" rather than "must not be empty".

4. **Post-enforce readonly**: After `zrbrr_enforce()` succeeds, lock all enrolled RBRR_ variables with `readonly`. Move `ZRBRR_DOCKER_ENV` construction to after enforcement (don't build from unvalidated values).

5. **Trial targets**: Apply to:
   - One regime: RBRR (`rbrr_regime.sh` + `rbrr.env`)
   - One or two modules: candidates TBD (look for modules with literal constants suitable for `readonly VAR=value`)

6. **Literal constants**: Identify and convert appropriate literal constants in trial modules to `readonly`.

## Acceptance

- BCG documents `readonly` patterns with rationale and anti-patterns
- `zrbrr_kindle()` no longer sets defaults; buv detects unset vs empty
- RBRR variables are readonly after enforcement
- Trial module literal constants are readonly
- All existing tests pass (regime-smoke, xname-validation, etc.)
- Attempting to reassign a locked RBRR_ variable after kindle produces a clear bash error

**[260224-0843] rough**

Add `readonly` pattern to BCG and trial on selected modules/regime.

## Scope

1. **BCG update**: Add a new section to BCG covering `readonly` patterns:
   - `readonly VAR` (lock after assignment)
   - `readonly VAR=value` (assign-and-lock for script-local constants)
   - The re-source trap (why `readonly` cannot go in sourceable .env files)
   - The "lock after kindle" pattern for validated configuration

2. **Remove kindle defaults**: Eliminate the `RBRR_FOO="${RBRR_FOO:-}"` default-setting pattern from `zrbrr_kindle()`. These destroy information (unset vs empty) and are unnecessary because buv already uses `${!z_varname:-}` for safe indirect expansion under `set -u`.

3. **buv enhancement**: In `zbuv_check_capture`, add an unset-detection check using `${!z_varname+x}` before the type-switch. Unset required variables should produce a distinct error: "VARNAME is not set (missing from .env?)" rather than "must not be empty".

4. **Post-enforce readonly**: After `zrbrr_enforce()` succeeds, lock all enrolled RBRR_ variables with `readonly`. Move `ZRBRR_DOCKER_ENV` construction to after enforcement (don't build from unvalidated values).

5. **Trial targets**: Apply to:
   - One regime: RBRR (`rbrr_regime.sh` + `rbrr.env`)
   - One or two modules: candidates TBD (look for modules with literal constants suitable for `readonly VAR=value`)

6. **Literal constants**: Identify and convert appropriate literal constants in trial modules to `readonly`.

## Acceptance

- BCG documents `readonly` patterns with rationale and anti-patterns
- `zrbrr_kindle()` no longer sets defaults; buv detects unset vs empty
- RBRR variables are readonly after enforcement
- Trial module literal constants are readonly
- All existing tests pass (regime-smoke, xname-validation, etc.)
- Attempting to reassign a locked RBRR_ variable after kindle produces a clear bash error

### readonly-widespread (₢AkAAE) [complete]

**[260224-1152] complete**

Apply `readonly` patterns across all regimes and modules, following patterns proven in ₢AkAAD trial.

## Scope

1. **All regime kindle functions**: Remove defaults, add post-enforce `readonly` locking, move docker-env construction to post-enforce. Apply to:
   - RBRN (`rbrn_regime.sh`) — all three nameplates (nsproto, srjcl, pluml)
   - RBRA if applicable
   - Any other regime modules

2. **All module literal constants**: Convert appropriate literal constants to `readonly` across all modules in `Tools/rbw/`, `Tools/buk/`, and other kit directories.

3. **Script-local constants**: Apply `readonly VAR=value` pattern to script-local constants in CLI entry points and install/uninstall scripts (extending the pattern already used in VVK's `vvi_install.sh` and `vvu_uninstall.sh`).

## Acceptance

- All regime variables are readonly after enforcement
- All identified literal constants are readonly
- Full test suite passes
- No regressions in any tabtarget

**[260224-0843] rough**

Apply `readonly` patterns across all regimes and modules, following patterns proven in ₢AkAAD trial.

## Scope

1. **All regime kindle functions**: Remove defaults, add post-enforce `readonly` locking, move docker-env construction to post-enforce. Apply to:
   - RBRN (`rbrn_regime.sh`) — all three nameplates (nsproto, srjcl, pluml)
   - RBRA if applicable
   - Any other regime modules

2. **All module literal constants**: Convert appropriate literal constants to `readonly` across all modules in `Tools/rbw/`, `Tools/buk/`, and other kit directories.

3. **Script-local constants**: Apply `readonly VAR=value` pattern to script-local constants in CLI entry points and install/uninstall scripts (extending the pattern already used in VVK's `vvi_install.sh` and `vvu_uninstall.sh`).

## Acceptance

- All regime variables are readonly after enforcement
- All identified literal constants are readonly
- Full test suite passes
- No regressions in any tabtarget

### bootstrap-constants-secrets-consolidation (₢AkAAG) [complete]

**[260224-1416] complete**

Add `BURD_CONFIG_DIR` to the launcher, create RBBC bootstrap constants with `.rbk/` file locations, move regime files to `.rbk/`, and migrate consumers from RBCC to RBBC file constants.

## Launcher Change

1. **`bul_launcher.sh`** — Add `export BURD_CONFIG_DIR="${ZBUL_PROJECT_ROOT}/.buk"` early. Derive `BURD_REGIME_FILE` from it: `export BURD_REGIME_FILE="${BURD_CONFIG_DIR}/burc.env"`.

## New Files

2. **`.buk/rbbc_constants.sh`** (RBBC — RBK Bootstrap Constants)
   - Multiple-inclusion guard pattern
   - Source-time literal constants:
     - `RBBC_dot_dir=".rbk"`
     - `RBBC_rbrr_file="${RBBC_dot_dir}/rbrr.env"`
     - `RBBC_rbrp_file="${RBBC_dot_dir}/rbrp.env"`

3. **`.rbk/`** directory — new home for RBK regime files
   - `git mv rbrr.env .rbk/rbrr.env`
   - `git mv rbrp.env .rbk/rbrp.env`

## RBCC Changes

4. **`rbcc_Constants.sh`** — Source RBBC at source time (top of file, after inclusion guard):
   - Add `source "${BURD_CONFIG_DIR}/rbbc_constants.sh"`
   - Add `BURD_CONFIG_DIR` sentinel
   - Remove `RBCC_rbrr_file="rbrr.env"` and `RBCC_rbrp_file="rbrp.env"` (replaced by RBBC equivalents)
   - `RBCC_rbro_file` left unchanged (handled by ₢AkAAH)
   - Remaining constants stay: `RBCC_rbrs_file`, `RBCC_rbrn_prefix`, `RBCC_rbrn_ext`, `RBCC_KIT_DIR`

## Consumer Renames

5. **All `RBCC_rbrr_file` consumers** — Rename to `RBBC_rbrr_file`. Files: rbf_cli.sh, rbgg_cli.sh, rbgm_cli.sh, rbgp_cli.sh, rbrr_cli.sh, rbrn_cli.sh, rbrv_cli.sh, rbra_cli.sh, rbob_cli.sh, rbtb_testbench.sh, rbtcap_AccessProbe.sh, butcrg_RegimeSmoke.sh, butcrg_RegimeCredentials.sh, rbgm_ManualProcedures.sh.

6. **All `RBCC_rbrp_file` consumers** — Rename to `RBBC_rbrp_file`. Files: rbrp_cli.sh, rbgp_cli.sh, rbgm_cli.sh, rbtcap_AccessProbe.sh, rbgm_ManualProcedures.sh.

7. **`BCG-BashConsoleGuide.md`** — Update any references to `RBCC_rbrr_file` / `RBCC_rbrp_file`.

## Design Rationale

- `BURD_CONFIG_DIR` is the canonical locator for `.buk/`, exported from launcher, survives exec boundary
- RBBC owns all `.rbk/` file knowledge — source-time literals, no kindle dependency
- `rbcc_Constants.sh` sources RBBC at source time, so RBBC literals are available before kindle (consumers use them before `zrbcc_kindle`)
- `.rbk/` parallels `.buk/` as the RBK installation directory for regime files
- Consumer renames are mechanical find/replace (`RBCC_rbrr_file` → `RBBC_rbrr_file`)
- `RBCC_rbro_file` left unchanged in this pace (handled by follow-on ₢AkAAH)

## Acceptance

- `BURD_CONFIG_DIR` exported from `bul_launcher.sh`, points to `.buk/`
- `.buk/rbbc_constants.sh` exists with `RBBC_dot_dir`, `RBBC_rbrr_file`, `RBBC_rbrp_file`
- `.rbk/` directory exists with `rbrr.env` and `rbrp.env`
- No remaining `RBCC_rbrr_file` or `RBCC_rbrp_file` references
- All existing operations work unchanged
- Smoke tests pass

**[260224-1403] rough**

Add `BURD_CONFIG_DIR` to the launcher, create RBBC bootstrap constants with `.rbk/` file locations, move regime files to `.rbk/`, and migrate consumers from RBCC to RBBC file constants.

## Launcher Change

1. **`bul_launcher.sh`** — Add `export BURD_CONFIG_DIR="${ZBUL_PROJECT_ROOT}/.buk"` early. Derive `BURD_REGIME_FILE` from it: `export BURD_REGIME_FILE="${BURD_CONFIG_DIR}/burc.env"`.

## New Files

2. **`.buk/rbbc_constants.sh`** (RBBC — RBK Bootstrap Constants)
   - Multiple-inclusion guard pattern
   - Source-time literal constants:
     - `RBBC_dot_dir=".rbk"`
     - `RBBC_rbrr_file="${RBBC_dot_dir}/rbrr.env"`
     - `RBBC_rbrp_file="${RBBC_dot_dir}/rbrp.env"`

3. **`.rbk/`** directory — new home for RBK regime files
   - `git mv rbrr.env .rbk/rbrr.env`
   - `git mv rbrp.env .rbk/rbrp.env`

## RBCC Changes

4. **`rbcc_Constants.sh`** — Source RBBC at source time (top of file, after inclusion guard):
   - Add `source "${BURD_CONFIG_DIR}/rbbc_constants.sh"`
   - Add `BURD_CONFIG_DIR` sentinel
   - Remove `RBCC_rbrr_file="rbrr.env"` and `RBCC_rbrp_file="rbrp.env"` (replaced by RBBC equivalents)
   - `RBCC_rbro_file` left unchanged (handled by ₢AkAAH)
   - Remaining constants stay: `RBCC_rbrs_file`, `RBCC_rbrn_prefix`, `RBCC_rbrn_ext`, `RBCC_KIT_DIR`

## Consumer Renames

5. **All `RBCC_rbrr_file` consumers** — Rename to `RBBC_rbrr_file`. Files: rbf_cli.sh, rbgg_cli.sh, rbgm_cli.sh, rbgp_cli.sh, rbrr_cli.sh, rbrn_cli.sh, rbrv_cli.sh, rbra_cli.sh, rbob_cli.sh, rbtb_testbench.sh, rbtcap_AccessProbe.sh, butcrg_RegimeSmoke.sh, butcrg_RegimeCredentials.sh, rbgm_ManualProcedures.sh.

6. **All `RBCC_rbrp_file` consumers** — Rename to `RBBC_rbrp_file`. Files: rbrp_cli.sh, rbgp_cli.sh, rbgm_cli.sh, rbtcap_AccessProbe.sh, rbgm_ManualProcedures.sh.

7. **`BCG-BashConsoleGuide.md`** — Update any references to `RBCC_rbrr_file` / `RBCC_rbrp_file`.

## Design Rationale

- `BURD_CONFIG_DIR` is the canonical locator for `.buk/`, exported from launcher, survives exec boundary
- RBBC owns all `.rbk/` file knowledge — source-time literals, no kindle dependency
- `rbcc_Constants.sh` sources RBBC at source time, so RBBC literals are available before kindle (consumers use them before `zrbcc_kindle`)
- `.rbk/` parallels `.buk/` as the RBK installation directory for regime files
- Consumer renames are mechanical find/replace (`RBCC_rbrr_file` → `RBBC_rbrr_file`)
- `RBCC_rbro_file` left unchanged in this pace (handled by follow-on ₢AkAAH)

## Acceptance

- `BURD_CONFIG_DIR` exported from `bul_launcher.sh`, points to `.buk/`
- `.buk/rbbc_constants.sh` exists with `RBBC_dot_dir`, `RBBC_rbrr_file`, `RBBC_rbrp_file`
- `.rbk/` directory exists with `rbrr.env` and `rbrp.env`
- No remaining `RBCC_rbrr_file` or `RBCC_rbrp_file` references
- All existing operations work unchanged
- Smoke tests pass

**[260224-1352] rough**

Add `BURD_CONFIG_DIR` to the launcher, create RBBC bootstrap constants, move regime files to `.rbk/`, and rename RBCC file constants to uppercase.

## Launcher Change

1. **`bul_launcher.sh`** — Add `export BURD_CONFIG_DIR="${ZBUL_PROJECT_ROOT}/.buk"` early. Derive `BURD_REGIME_FILE` from it: `export BURD_REGIME_FILE="${BURD_CONFIG_DIR}/burc.env"`.

## New Files

2. **`.buk/rbbc_constants.sh`** (RBBC — RBK Bootstrap Constants)
   - Literal constant: `RBBC_DOT_DIR=".rbk"`
   - Multiple-inclusion guard pattern

3. **`.rbk/`** directory — new home for RBK regime files
   - `git mv rbrr.env .rbk/rbrr.env`
   - `git mv rbrp.env .rbk/rbrp.env`

## RBCC Changes

4. **`rbcc_Constants.sh`** — Remove literal `RBCC_rbrr_file="rbrr.env"` and `RBCC_rbrp_file="rbrp.env"` from top-level. Move to `zrbcc_kindle()` as derived kindle constants:
   - `readonly RBCC_RBRR_FILE="${RBBC_DOT_DIR}/rbrr.env"`
   - `readonly RBCC_RBRP_FILE="${RBBC_DOT_DIR}/rbrp.env"`
   - `zrbcc_kindle()` sources RBBC internally via `source "${BURD_CONFIG_DIR}/rbbc_constants.sh"`
   - Add `test -n "${BURD_CONFIG_DIR:-}" || buc_die ...` sentinel

## Consumer Renames

5. **All `RBCC_rbrr_file` consumers** — Rename to `RBCC_RBRR_FILE`. Files: rbf_cli.sh, rbgg_cli.sh, rbgm_cli.sh, rbgp_cli.sh, rbrr_cli.sh, rbrn_cli.sh, rbrv_cli.sh, rbra_cli.sh, rbob_cli.sh, rbtb_testbench.sh, rbtcap_AccessProbe.sh, butcrg_RegimeSmoke.sh, butcrg_RegimeCredentials.sh, rbgm_ManualProcedures.sh.

6. **All `RBCC_rbrp_file` consumers** — Rename to `RBCC_RBRP_FILE`. Files: rbrp_cli.sh, rbgp_cli.sh, rbgm_cli.sh, rbtcap_AccessProbe.sh, rbgm_ManualProcedures.sh.

## Design Rationale

- `BURD_CONFIG_DIR` is the canonical locator for `.buk/`, exported from launcher, survives exec boundary
- RBBC defines `.rbk/` location as a single literal; RBCC derives file paths at kindle time
- `.rbk/` parallels `.buk/` as the RBK installation directory for regime files
- Consumer code changes are mechanical rename only — no sourcing chain changes
- `zrbcc_kindle` internally sources RBBC, so CLI furnish functions need no changes to sourcing order
- RBCC_rbro_file left unchanged in this pace (handled by follow-on pace ₢AkAAB)

## Acceptance

- `BURD_CONFIG_DIR` exported from `bul_launcher.sh`, points to `.buk/`
- `.buk/rbbc_constants.sh` exists with `RBBC_DOT_DIR=".rbk"`
- `.rbk/` directory exists with `rbrr.env` and `rbrp.env`
- `RBCC_RBRR_FILE` and `RBCC_RBRP_FILE` are kindle constants derived from `RBBC_DOT_DIR`
- No remaining `RBCC_rbrr_file` or `RBCC_rbrp_file` (lowercase) references
- All existing operations work unchanged
- Smoke tests pass

**[260224-1348] rough**

Add `BURD_CONFIG_DIR` to the launcher, create RBBC bootstrap constants and `.rbk/` directory, then consolidate credential file paths under `RBRR_SECRETS_DIR`.

## Launcher Change

1. **`bul_launcher.sh`** — Add `export BURD_CONFIG_DIR="${ZBUL_PROJECT_ROOT}/.buk"` early. Derive `BURD_REGIME_FILE` from it: `export BURD_REGIME_FILE="${BURD_CONFIG_DIR}/burc.env"`.

## New Files

2. **`.buk/rbbc_constants.sh`** (RBBC — RBK Bootstrap Constants)
   - Sourced by CLI furnish functions via `source "${BURD_CONFIG_DIR}/rbbc_constants.sh"`
   - Literal constants for RBK regime file locations:
     - `RBBC_RBRR_FILE=".rbk/rbrr.env"` (repo regime file, relative to project root)
     - `RBBC_RBRP_FILE=".rbk/rbrp.env"` (payor regime file, relative to project root)

3. **`.rbk/`** directory — new home for RBK regime files
   - `git mv rbrr.env .rbk/rbrr.env`
   - `git mv rbrp.env .rbk/rbrp.env`

## RBRR Regime Changes

4. **`rbrr_regime.sh`** — Remove three `buv_string_enroll` for `RBRR_GOVERNOR_RBRA_FILE`, `RBRR_RETRIEVER_RBRA_FILE`, `RBRR_DIRECTOR_RBRA_FILE`. Add `buv_string_enroll RBRR_SECRETS_DIR` (validated via `buv_dir_exists` in enforce step). Add four readonly kindle constants in the lock step, derived from `RBRR_SECRETS_DIR`:
   - `RBRR_GOVERNOR_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-governor.env"`
   - `RBRR_RETRIEVER_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-retriever.env"`
   - `RBRR_DIRECTOR_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-director.env"`
   - `RBRR_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/rbro-payor.env"`

5. **`rbrr.env`** (now at `.rbk/rbrr.env`) — Remove three `RBRR_*_RBRA_FILE=` lines. Add `RBRR_SECRETS_DIR=../station-files/secrets`.

## RBRO/Payor Changes

6. **`rbro_regime.sh`** — Rewrite `rbro_load()` to use `RBRR_PAYOR_RBRO_FILE` instead of hardcoded `${HOME}/.rbw`. Remove directory check. RBRR must be kindled before rbro_load is called.

7. **`rbro_cli.sh`** — Replace `RBCC_rbro_file` with `RBRR_PAYOR_RBRO_FILE`. Source RBBC and RBRR before RBRO. Remove RBCC sourcing if no longer needed here.

8. **`rbgp_Payor.sh`** — Write to `RBRR_PAYOR_RBRO_FILE` instead of `${HOME}/.rbw/rbro.env`. Create parent dir via `dirname`.

## RBCC Cleanup

9. **`rbcc_Constants.sh`** — Remove `RBCC_rbrr_file`, `RBCC_rbrp_file`, `RBCC_rbro_file`. Remaining constants stay: `RBCC_rbrs_file`, `RBCC_rbrn_prefix`, `RBCC_rbrn_ext`, `RBCC_KIT_DIR`.

## Consumer Updates

10. **All `RBCC_rbrr_file` consumers** — Add `source "${BURD_CONFIG_DIR}/rbbc_constants.sh"` early in furnish, replace `source "${RBCC_rbrr_file}"` with `source "${RBBC_RBRR_FILE}"`. Files: rbf_cli.sh, rbgg_cli.sh, rbgm_cli.sh, rbgp_cli.sh, rbrr_cli.sh, rbrn_cli.sh, rbrv_cli.sh, rbra_cli.sh, rbob_cli.sh, rbtb_testbench.sh, rbtcap_AccessProbe.sh, butcrg_RegimeSmoke.sh, butcrg_RegimeCredentials.sh, rbgm_ManualProcedures.sh.

11. **All `RBCC_rbrp_file` consumers** — Replace `source "${RBCC_rbrp_file}"` with `source "${RBBC_RBRP_FILE}"` (RBBC already sourced from item 10). Files: rbrp_cli.sh, rbgp_cli.sh, rbgm_cli.sh, rbtcap_AccessProbe.sh, rbgm_ManualProcedures.sh.

12. **`butcrg_RegimeCredentials.sh`** — Replace hardcoded `${HOME}/.rbw/rbro.env` with `RBRR_PAYOR_RBRO_FILE`.

13. **`rbgm_ManualProcedures.sh`** — Update user-facing text referencing `~/.rbw/rbro.env`.

## Manual Step

14. **Copy credentials** — `cp ~/.rbw/rbro.env ../station-files/secrets/rbro-payor.env`

## Design Rationale

- `BURD_CONFIG_DIR` is the canonical locator for `.buk/`, exported from the launcher, survives exec boundary
- RBBC lives in `.buk/` and is sourced via `BURD_CONFIG_DIR` — clean, no path surgery
- `.rbk/` parallels `.buk/` as the RBK installation directory for regime files
- BUBC dropped — `BURD_CONFIG_DIR` already provides `.buk/` location
- Credential file paths are kindle constants in `rbrr_regime.sh`, derived from user-configurable `RBRR_SECRETS_DIR`
- RBCC retains station file, nameplate prefix/ext, and KIT_DIR — only file-path constants migrate to RBBC
- `RBRR_*_RBRA_FILE` names preserved — consumers unchanged, just the source of truth moves from enrollment to kindle

## Acceptance

- `BURD_CONFIG_DIR` exported from `bul_launcher.sh`, points to `.buk/`
- RBBC file exists at `.buk/rbbc_constants.sh` with `RBBC_RBRR_FILE` and `RBBC_RBRP_FILE`
- `.rbk/` directory exists with `rbrr.env` and `rbrp.env`
- `RBRR_SECRETS_DIR` is the single enrolled variable; four credential paths derived as kindle constants
- No remaining `RBCC_rbrr_file`, `RBCC_rbrp_file`, or `RBCC_rbro_file` references
- No remaining hardcoded `~/.rbw` references in Tools/
- All existing operations work unchanged
- Smoke tests pass

**[260224-1330] rough**

Introduce BUBC and RBBC bootstrap constant files in `.buk/`, create `.rbk/` directory for RBK regime files, then consolidate credential file paths to use RBBC-derived constants.

## New Files

1. **`.buk/bubc_constants.sh`** (BUBC — BUK Bootstrap Constants)
   - Sourced very early by `bul_launcher.sh`, before any BUK infrastructure
   - Literal constants for directory names that are currently hardcoded:
     - `BUBC_DOT_DIR=".buk"` (the `.buk` directory name)
     - Other BUK directory name constants as needed
   - Tabtargets still hardcode `.buk/` (unavoidable — they're the bootstrap entry point)

2. **`.buk/rbbc_constants.sh`** (RBBC — RBK Bootstrap Constants)
   - Sourced very early by `bul_launcher.sh`, after BUBC
   - Literal constants for RBK paths:
     - `RBBC_DOT_DIR=".rbk"` (the `.rbk` directory name)
     - `RBBC_RBRR_FILE=".rbk/rbrr.env"` (repo regime file)
     - `RBBC_RBRP_FILE=".rbk/rbrp.env"` (payor regime file)
     - `RBBC_SECRETS_DIR="../station-files/secrets"` (credential file directory)
     - `RBBC_GOVERNOR_RBRA_FILE` — derived from RBBC_SECRETS_DIR
     - `RBBC_RETRIEVER_RBRA_FILE` — derived from RBBC_SECRETS_DIR
     - `RBBC_DIRECTOR_RBRA_FILE` — derived from RBBC_SECRETS_DIR
     - `RBBC_PAYOR_RBRO_FILE` — derived from RBBC_SECRETS_DIR

3. **`.rbk/`** directory — new home for RBK regime files
   - `mv rbrr.env .rbk/rbrr.env`
   - `mv rbrp.env .rbk/rbrp.env`

## Launcher Change

4. **`bul_launcher.sh`** — Source `.buk/bubc_constants.sh` and `.buk/rbbc_constants.sh` early (after `burc.env`, before BUK modules load)

## Consolidation Changes

5. **`rbrr_regime.sh`** — Remove three `buv_string_enroll` for `RBRR_GOVERNOR_RBRA_FILE`, `RBRR_RETRIEVER_RBRA_FILE`, `RBRR_DIRECTOR_RBRA_FILE`. Add `buv_dir_enroll RBRR_SECRETS_DIR` which validates the directory exists. RBRR remains a configuration interface for secrets directory location.

6. **`rbrr.env`** — Remove three `RBRR_*_RBRA_FILE=` lines. Add `RBRR_SECRETS_DIR=../station-files/secrets`.

7. **`rbro_regime.sh`** — Rewrite `rbro_load()` to use `RBBC_PAYOR_RBRO_FILE` instead of hardcoded `${HOME}/.rbw`.

8. **`rbro_cli.sh`** — Replace `RBCC_rbro_file` with `RBBC_PAYOR_RBRO_FILE`.

9. **`rbgp_Payor.sh`** — Write to `RBBC_PAYOR_RBRO_FILE` instead of `${HOME}/.rbw/rbro.env`.

10. **`rbcc_Constants.sh`** — Remove dead `RBCC_rbro_file` and `RBCC_rbrr_file` and `RBCC_rbrp_file` constants. These are replaced by RBBC equivalents.

11. **All `RBCC_rbrr_file` consumers** (rbf_cli.sh, rbgg_cli.sh, rbgm_cli.sh, rbgp_cli.sh, rbrr_cli.sh, rbrn_cli.sh, rbrv_cli.sh, rbra_cli.sh, rbob_cli.sh, rbtb_testbench.sh, rbtcap_AccessProbe.sh, butcrg_RegimeSmoke.sh, butcrg_RegimeCredentials.sh, rbgm_ManualProcedures.sh) — Replace with `RBBC_RBRR_FILE`.

12. **All `RBCC_rbrp_file` consumers** (rbrp_cli.sh, rbgp_cli.sh, rbgm_cli.sh, rbtcap_AccessProbe.sh, rbgm_ManualProcedures.sh) — Replace with `RBBC_RBRP_FILE`.

13. **All `RBRR_*_RBRA_FILE` consumers** (rbf_Foundry.sh, rbi_Image.sh, rbgg_Governor.sh, rbgu_Utility.sh, rbap_AccessProbe.sh, rbra_cli.sh, rbrn_cli.sh, rbgm_ManualProcedures.sh, butcrg_RegimeCredentials.sh) — Rename to `RBBC_*` equivalents.

14. **Copy credentials** — `cp ~/.rbw/rbro.env ../station-files/secrets/rbro-payor.env`

## Design Rationale

- Both BUBC and RBBC live in `.buk/` — the bootstrap root already found by hardcoded relative paths in tabtargets and launchers
- RBBC is project-installation-scoped (per-installation, not per-kit) — same reason `burc.env` lives in `.buk/`
- `.rbk/` parallels `.buk/` as the RBK installation directory for regime files
- Tabtargets unavoidably hardcode `.buk/`; everything downstream uses BUBC/RBBC constants for documentation correctness
- RBRR remains a pure configuration regime (secrets directory path is user-configurable)
- Credential file NAMES are intimate constants in RBBC, not regime variables
- RBBC is available to all CLIs without requiring RBRR kindle first
- `rbrr.env` and `rbrp.env` move to `.rbk/` because they are RBK installation config, not project source
- `rbrs.env` stays in `station-files` and `rbrn_*.env` stay in `Tools/rbw/` — they already have intimate location logic

## Acceptance

- BUBC file exists at `.buk/bubc_constants.sh` and is sourced by `bul_launcher.sh`
- RBBC file exists at `.buk/rbbc_constants.sh` with all path constants
- `.rbk/` directory exists with `rbrr.env` and `rbrp.env`
- RBRR_SECRETS_DIR is the single enrolled regime variable for secrets location
- No remaining `RBCC_rbrr_file`, `RBCC_rbrp_file`, or `RBCC_rbro_file` references
- No remaining hardcoded `~/.rbw` references in Tools/
- All existing operations work unchanged
- Smoke tests pass

**[260224-1326] rough**

Introduce BUBC and RBBC bootstrap constant files in `.buk/`, then consolidate credential file paths to use `RBRR_SECRETS_DIR` with RBBC-derived kindle constants.

## New Files

1. **`.buk/bubc_constants.sh`** (BUBC — BUK Bootstrap Constants)
   - Sourced very early by `bul_launcher.sh`, before any BUK infrastructure
   - Literal constants for directory names that are currently hardcoded:
     - `BUBC_DOT_DIR=".buk"` (the `.buk` directory name)
     - Other BUK directory name constants as needed
   - Tabtargets still hardcode `.buk/` (unavoidable — they're the bootstrap entry point)

2. **`.buk/rbbc_constants.sh`** (RBBC — RBK Bootstrap Constants)
   - Sourced very early by `bul_launcher.sh`, after BUBC
   - Literal constants for RBK credential file paths:
     - `RBBC_SECRETS_DIR="../station-files/secrets"` (credential file directory)
     - `RBBC_GOVERNOR_RBRA_FILE` — derived from RBBC_SECRETS_DIR
     - `RBBC_RETRIEVER_RBRA_FILE` — derived from RBBC_SECRETS_DIR
     - `RBBC_DIRECTOR_RBRA_FILE` — derived from RBBC_SECRETS_DIR
     - `RBBC_PAYOR_RBRO_FILE` — derived from RBBC_SECRETS_DIR

## Launcher Change

3. **`bul_launcher.sh`** — Source `.buk/bubc_constants.sh` and `.buk/rbbc_constants.sh` early (after `burc.env`, before BUK modules load)

## Consolidation Changes

4. **`rbrr_regime.sh`** — Remove three `buv_string_enroll` for `RBRR_GOVERNOR_RBRA_FILE`, `RBRR_RETRIEVER_RBRA_FILE`, `RBRR_DIRECTOR_RBRA_FILE`. Add `buv_dir_enroll RBRR_SECRETS_DIR` which validates the directory exists. RBRR remains a configuration interface for secrets directory location.

5. **`rbrr.env`** — Remove three `RBRR_*_RBRA_FILE=` lines. Add `RBRR_SECRETS_DIR=../station-files/secrets`.

6. **`rbro_regime.sh`** — Rewrite `rbro_load()` to use `RBBC_PAYOR_RBRO_FILE` instead of hardcoded `${HOME}/.rbw`.

7. **`rbro_cli.sh`** — Replace `RBCC_rbro_file` with `RBBC_PAYOR_RBRO_FILE`.

8. **`rbgp_Payor.sh`** — Write to `RBBC_PAYOR_RBRO_FILE` instead of `${HOME}/.rbw/rbro.env`.

9. **`rbcc_Constants.sh`** — Remove dead `RBCC_rbro_file` constant.

10. **All RBRR_*_RBRA_FILE consumers** (rbf_Foundry.sh, rbi_Image.sh, rbgg_Governor.sh, rbgu_Utility.sh, rbap_AccessProbe.sh, rbra_cli.sh, rbrn_cli.sh, rbgm_ManualProcedures.sh, butcrg_RegimeCredentials.sh) — Rename to `RBBC_*` equivalents.

11. **Copy credentials** — `cp ~/.rbw/rbro.env ../station-files/secrets/rbro-payor.env`

## Design Rationale

- Both BUBC and RBBC live in `.buk/` — the bootstrap root already found by hardcoded relative paths in tabtargets and launchers
- RBBC is project-installation-scoped (per-installation, not per-kit) — same reason `burc.env` lives in `.buk/`
- Tabtargets unavoidably hardcode `.buk/`; everything downstream uses BUBC/RBBC constants for documentation correctness
- RBRR remains a pure configuration regime (secrets directory path is user-configurable)
- Credential file NAMES are intimate constants in RBBC, not regime variables
- RBBC is available to all CLIs without requiring RBRR kindle first

## Acceptance

- BUBC file exists at `.buk/bubc_constants.sh` and is sourced by `bul_launcher.sh`
- RBBC file exists at `.buk/rbbc_constants.sh` with all credential path constants
- RBRR_SECRETS_DIR is the single enrolled regime variable for secrets location
- No remaining hardcoded `~/.rbw` references in Tools/
- All existing operations work unchanged
- Smoke tests pass

**[260224-1318] rough**

Introduce BUBC and RBBC bootstrap constant files, then consolidate credential file paths to use `RBRR_SECRETS_DIR` with RBBC-derived kindle constants.

## New Files

1. **`.buk/bubc_constants.sh`** (BUBC — BUK Bootstrap Constants)
   - Sourced very early, before any BUK infrastructure
   - Literal constants for directory names that are currently hardcoded:
     - `BUBC_DOT_DIR=".buk"` (the `.buk` directory name)
     - Other BUK directory name constants as needed

2. **`Tools/rbw/rbbc_constants.sh`** (RBBC — RBK Bootstrap Constants)
   - Sourced very early, after BUBC
   - Literal constants for RBK directory paths:
     - `RBBC_SECRETS_DIR="../station-files/secrets"` (credential file directory)
     - `RBBC_GOVERNOR_RBRA_FILE` — derived from RBBC_SECRETS_DIR
     - `RBBC_RETRIEVER_RBRA_FILE` — derived from RBBC_SECRETS_DIR
     - `RBBC_DIRECTOR_RBRA_FILE` — derived from RBBC_SECRETS_DIR
     - `RBBC_PAYOR_RBRO_FILE` — derived from RBBC_SECRETS_DIR

## Consolidation Changes

3. **`rbrr_regime.sh`** — Remove three `buv_string_enroll` for `RBRR_GOVERNOR_RBRA_FILE`, `RBRR_RETRIEVER_RBRA_FILE`, `RBRR_DIRECTOR_RBRA_FILE`. Add `buv_string_enroll RBRR_SECRETS_DIR` which validates the directory exists. RBRR remains a configuration interface for secrets directory location.

4. **`rbrr.env`** — Remove three `RBRR_*_RBRA_FILE=` lines. Add `RBRR_SECRETS_DIR=../station-files/secrets`.

5. **`rbro_regime.sh`** — Rewrite `rbro_load()` to use `RBBC_PAYOR_RBRO_FILE` instead of hardcoded `${HOME}/.rbw`.

6. **`rbro_cli.sh`** — Replace `RBCC_rbro_file` with `RBBC_PAYOR_RBRO_FILE`. Add RBRR source/kindle/enforce/lock.

7. **`rbgp_Payor.sh`** — Write to `RBBC_PAYOR_RBRO_FILE` instead of `${HOME}/.rbw/rbro.env`.

8. **`rbcc_Constants.sh`** — Remove dead `RBCC_rbro_file` constant.

9. **All RBRR_*_RBRA_FILE consumers** (rbf_Foundry.sh, rbi_Image.sh, rbgg_Governor.sh, rbgu_Utility.sh, rbap_AccessProbe.sh, rbra_cli.sh, rbrn_cli.sh, rbgm_ManualProcedures.sh, butcrg_RegimeCredentials.sh) — Rename to `RBBC_*` equivalents.

10. **Copy credentials** — `cp ~/.rbw/rbro.env ../station-files/secrets/rbro-payor.env`

## Design Rationale

- BUBC/RBBC are bootstrap files sourced before any regime kindle — no lifecycle dependency issues
- RBRR remains a pure configuration regime (secrets directory path is user-configurable)
- Credential file NAMES are intimate constants in RBBC, not regime variables
- RBBC is available to all CLIs without requiring RBRR kindle first
- `.buk` hardcoding cleanup is enabled by BUBC (separate pace)

## Acceptance

- BUBC file exists and is sourced by BUK launcher infrastructure
- RBBC file exists with all credential path constants
- RBRR_SECRETS_DIR is the single enrolled regime variable for secrets location
- No remaining hardcoded `~/.rbw` references in Tools/
- All existing operations work unchanged
- Smoke tests pass

**[260224-1318] rough**

Introduce BUBC and RBBC bootstrap constant files, then consolidate credential file paths to use `RBRR_SECRETS_DIR` with RBBC-derived kindle constants.

## New Files

1. **`.buk/bubc_constants.sh`** (BUBC — BUK Bootstrap Constants)
   - Sourced very early, before any BUK infrastructure
   - Literal constants for directory names that are currently hardcoded:
     - `BUBC_DOT_DIR=".buk"` (the `.buk` directory name)
     - Other BUK directory name constants as needed

2. **`Tools/rbw/rbbc_constants.sh`** (RBBC — RBK Bootstrap Constants)
   - Sourced very early, after BUBC
   - Literal constants for RBK directory paths:
     - `RBBC_SECRETS_DIR="../station-files/secrets"` (credential file directory)
     - `RBBC_GOVERNOR_RBRA_FILE` — derived from RBBC_SECRETS_DIR
     - `RBBC_RETRIEVER_RBRA_FILE` — derived from RBBC_SECRETS_DIR
     - `RBBC_DIRECTOR_RBRA_FILE` — derived from RBBC_SECRETS_DIR
     - `RBBC_PAYOR_RBRO_FILE` — derived from RBBC_SECRETS_DIR

## Consolidation Changes

3. **`rbrr_regime.sh`** — Remove three `buv_string_enroll` for `RBRR_GOVERNOR_RBRA_FILE`, `RBRR_RETRIEVER_RBRA_FILE`, `RBRR_DIRECTOR_RBRA_FILE`. Add `buv_string_enroll RBRR_SECRETS_DIR` which validates the directory exists. RBRR remains a configuration interface for secrets directory location.

4. **`rbrr.env`** — Remove three `RBRR_*_RBRA_FILE=` lines. Add `RBRR_SECRETS_DIR=../station-files/secrets`.

5. **`rbro_regime.sh`** — Rewrite `rbro_load()` to use `RBBC_PAYOR_RBRO_FILE` instead of hardcoded `${HOME}/.rbw`.

6. **`rbro_cli.sh`** — Replace `RBCC_rbro_file` with `RBBC_PAYOR_RBRO_FILE`. Add RBRR source/kindle/enforce/lock.

7. **`rbgp_Payor.sh`** — Write to `RBBC_PAYOR_RBRO_FILE` instead of `${HOME}/.rbw/rbro.env`.

8. **`rbcc_Constants.sh`** — Remove dead `RBCC_rbro_file` constant.

9. **All RBRR_*_RBRA_FILE consumers** (rbf_Foundry.sh, rbi_Image.sh, rbgg_Governor.sh, rbgu_Utility.sh, rbap_AccessProbe.sh, rbra_cli.sh, rbrn_cli.sh, rbgm_ManualProcedures.sh, butcrg_RegimeCredentials.sh) — Rename to `RBBC_*` equivalents.

10. **Copy credentials** — `cp ~/.rbw/rbro.env ../station-files/secrets/rbro-payor.env`

## Design Rationale

- BUBC/RBBC are bootstrap files sourced before any regime kindle — no lifecycle dependency issues
- RBRR remains a pure configuration regime (secrets directory path is user-configurable)
- Credential file NAMES are intimate constants in RBBC, not regime variables
- RBBC is available to all CLIs without requiring RBRR kindle first
- `.buk` hardcoding cleanup is enabled by BUBC (separate pace)

## Acceptance

- BUBC file exists and is sourced by BUK launcher infrastructure
- RBBC file exists with all credential path constants
- RBRR_SECRETS_DIR is the single enrolled regime variable for secrets location
- No remaining hardcoded `~/.rbw` references in Tools/
- All existing operations work unchanged
- Smoke tests pass

**[260224-1247] rough**

Consolidate all credential file paths to a single `RBRR_SECRETS_DIR` enrolled variable with kindle constants for individual files.

## Changes

1. **`rbrr_regime.sh`** — Remove three `buv_string_enroll` for `RBRR_GOVERNOR_RBRA_FILE`, `RBRR_RETRIEVER_RBRA_FILE`, `RBRR_DIRECTOR_RBRA_FILE`. Add `buv_dir_enroll RBRR_SECRETS_DIR`. Add four `readonly` kindle constants computed from `RBRR_SECRETS_DIR`:
   - `RBRR_GOVERNOR_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-governor.env"`
   - `RBRR_RETRIEVER_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-retriever.env"`
   - `RBRR_DIRECTOR_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-director.env"`
   - `RBRR_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/rbro-payor.env"`

2. **`rbrr.env`** — Remove three `RBRR_*_RBRA_FILE=` lines. Add `RBRR_SECRETS_DIR=../station-files/secrets`.

3. **`rbro_regime.sh`** — Rewrite `rbro_load()` to use `RBRR_PAYOR_RBRO_FILE` instead of hardcoded `${HOME}/.rbw`. Remove directory check. Update comments and error messages.

4. **`rbro_cli.sh`** — Replace `RBCC_rbro_file` reference with `RBRR_PAYOR_RBRO_FILE`.

5. **`rbgp_Payor.sh`** — Write to `RBRR_PAYOR_RBRO_FILE` instead of `${HOME}/.rbw/rbro.env`. Create parent dir via `dirname`.

6. **`rbcc_Constants.sh`** — Remove dead `RBCC_rbro_file` constant.

7. **`butcrg_RegimeCredentials.sh`** — Replace hardcoded `${HOME}/.rbw/rbro.env` with `RBRR_PAYOR_RBRO_FILE`.

8. **`rbgm_ManualProcedures.sh`** — Update user-facing text referencing `~/.rbw/rbro.env`.

9. **Copy credentials** — `cp ~/.rbw/rbro.env ../station-files/secrets/rbro-payor.env`

## Acceptance

- `RBRR_SECRETS_DIR` is the single enrolled path for all credential files
- All four credential file paths are kindle constants derived from it
- No remaining hardcoded `~/.rbw` references in Tools/
- Existing operations (payor install, OAuth flow, credential validation) work unchanged

### secrets-dir-credential-consolidation (₢AkAAH) [complete]

**[260224-1529] complete**

Consolidate credential file paths under `RBRR_SECRETS_DIR`, eliminate all hardcoded `~/.rbw` references, and update RBS0 spec.

Depends on ₢AkAAG (BURD_CONFIG_DIR, RBBC, .rbk/ infrastructure).

## RBRR Regime Changes

1. **`rbrr_regime.sh`** — Remove three `buv_string_enroll` for `RBRR_GOVERNOR_RBRA_FILE`, `RBRR_RETRIEVER_RBRA_FILE`, `RBRR_DIRECTOR_RBRA_FILE`. Add `buv_string_enroll RBRR_SECRETS_DIR` (validated via `buv_dir_exists` in enforce step). Add four readonly kindle constants in `zrbrr_lock()` (MUST be lock step — `buv_scope_sentinel RBRR RBRR_` in kindle would flag them), derived from `RBRR_SECRETS_DIR`:
   - `readonly RBRR_GOVERNOR_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-governor.env"`
   - `readonly RBRR_RETRIEVER_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-retriever.env"`
   - `readonly RBRR_DIRECTOR_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-director.env"`
   - `readonly RBRR_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/rbro-payor.env"`

2. **`.rbk/rbrr.env`** — Remove three `RBRR_*_RBRA_FILE=` lines and associated comments. Add `RBRR_SECRETS_DIR=../station-files/secrets`.

## RBRO/Payor Changes

3. **`rbro_regime.sh`** — Rewrite `rbro_load()` to use `RBRR_PAYOR_RBRO_FILE` instead of hardcoded `${HOME}/.rbw`. Remove directory check. RBRR must be kindled+locked before `rbro_load` is called (RBRR_PAYOR_RBRO_FILE is a lock-step kindle constant).

4. **`rbro_cli.sh`** — Significant furnish rewrite: must source+kindle+enforce+lock RBRR chain before RBRO load, since `RBRR_PAYOR_RBRO_FILE` is a kindle constant. Replace `RBCC_rbro_file` with `RBRR_PAYOR_RBRO_FILE`. RBCC still needed for other constants.

5. **`rbgp_Payor.sh`** — Write to `RBRR_PAYOR_RBRO_FILE` instead of `${HOME}/.rbw/rbro.env`. Create parent dir via `dirname`.

## RBCC Cleanup

6. **`rbcc_Constants.sh`** — Remove `RBCC_rbro_file="${HOME}/.rbw/rbro.env"`.

## Consumer Updates

7. **`butcrg_RegimeCredentials.sh`** — Replace hardcoded `${HOME}/.rbw/rbro.env` with `RBRR_PAYOR_RBRO_FILE`.

8. **`rbgm_ManualProcedures.sh`** — Update user-facing text referencing `~/.rbw/rbro.env`.

## Spec Updates

9. **`RBS0-SpecTop.adoc`** — Update regime variable definitions:
   - Add new linked term: `rbrr_secrets_dir` (enrolled regime variable, path, validated)
   - Add new linked term: `rbrr_payor_rbro_file` (kindle constant, path)
   - Change annotations on `rbrr_governor_rbra_file`, `rbrr_director_rbra_file`, `rbrr_retriever_rbra_file` from enrolled regime variables (`axrg_variable axvr_kindle axvr_validate axvr_render axtu_path`) to kindle constants
   - Update Service Accounts Group definition to reflect RBRR_SECRETS_DIR derivation
   - Add mapping section entries for new linked terms

10. **`RBSRR-RegimeRepo.adoc`** — Update regime documentation to reflect secrets directory consolidation and kindle constant derivation.

11. **`RBSRA-CredentialFormat.adoc`** — Update description (line 8 references three RBRR variables as regime variables; they're now kindle constants).

## Manual Step

12. **Copy credentials** — `cp ~/.rbw/rbro.env ../station-files/secrets/rbro-payor.env`

## Acceptance

- `RBRR_SECRETS_DIR` is the single enrolled variable; four credential paths derived as kindle constants in lock step
- No remaining `RBCC_rbro_file` references
- No remaining hardcoded `~/.rbw` references in Tools/
- RBS0 linked terms updated for new/changed regime variables
- All existing operations (payor install, OAuth flow, credential validation) work unchanged
- Smoke tests pass

**[260224-1403] rough**

Consolidate credential file paths under `RBRR_SECRETS_DIR`, eliminate all hardcoded `~/.rbw` references, and update RBS0 spec.

Depends on ₢AkAAG (BURD_CONFIG_DIR, RBBC, .rbk/ infrastructure).

## RBRR Regime Changes

1. **`rbrr_regime.sh`** — Remove three `buv_string_enroll` for `RBRR_GOVERNOR_RBRA_FILE`, `RBRR_RETRIEVER_RBRA_FILE`, `RBRR_DIRECTOR_RBRA_FILE`. Add `buv_string_enroll RBRR_SECRETS_DIR` (validated via `buv_dir_exists` in enforce step). Add four readonly kindle constants in `zrbrr_lock()` (MUST be lock step — `buv_scope_sentinel RBRR RBRR_` in kindle would flag them), derived from `RBRR_SECRETS_DIR`:
   - `readonly RBRR_GOVERNOR_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-governor.env"`
   - `readonly RBRR_RETRIEVER_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-retriever.env"`
   - `readonly RBRR_DIRECTOR_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-director.env"`
   - `readonly RBRR_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/rbro-payor.env"`

2. **`.rbk/rbrr.env`** — Remove three `RBRR_*_RBRA_FILE=` lines and associated comments. Add `RBRR_SECRETS_DIR=../station-files/secrets`.

## RBRO/Payor Changes

3. **`rbro_regime.sh`** — Rewrite `rbro_load()` to use `RBRR_PAYOR_RBRO_FILE` instead of hardcoded `${HOME}/.rbw`. Remove directory check. RBRR must be kindled+locked before `rbro_load` is called (RBRR_PAYOR_RBRO_FILE is a lock-step kindle constant).

4. **`rbro_cli.sh`** — Significant furnish rewrite: must source+kindle+enforce+lock RBRR chain before RBRO load, since `RBRR_PAYOR_RBRO_FILE` is a kindle constant. Replace `RBCC_rbro_file` with `RBRR_PAYOR_RBRO_FILE`. RBCC still needed for other constants.

5. **`rbgp_Payor.sh`** — Write to `RBRR_PAYOR_RBRO_FILE` instead of `${HOME}/.rbw/rbro.env`. Create parent dir via `dirname`.

## RBCC Cleanup

6. **`rbcc_Constants.sh`** — Remove `RBCC_rbro_file="${HOME}/.rbw/rbro.env"`.

## Consumer Updates

7. **`butcrg_RegimeCredentials.sh`** — Replace hardcoded `${HOME}/.rbw/rbro.env` with `RBRR_PAYOR_RBRO_FILE`.

8. **`rbgm_ManualProcedures.sh`** — Update user-facing text referencing `~/.rbw/rbro.env`.

## Spec Updates

9. **`RBS0-SpecTop.adoc`** — Update regime variable definitions:
   - Add new linked term: `rbrr_secrets_dir` (enrolled regime variable, path, validated)
   - Add new linked term: `rbrr_payor_rbro_file` (kindle constant, path)
   - Change annotations on `rbrr_governor_rbra_file`, `rbrr_director_rbra_file`, `rbrr_retriever_rbra_file` from enrolled regime variables (`axrg_variable axvr_kindle axvr_validate axvr_render axtu_path`) to kindle constants
   - Update Service Accounts Group definition to reflect RBRR_SECRETS_DIR derivation
   - Add mapping section entries for new linked terms

10. **`RBSRR-RegimeRepo.adoc`** — Update regime documentation to reflect secrets directory consolidation and kindle constant derivation.

11. **`RBSRA-CredentialFormat.adoc`** — Update description (line 8 references three RBRR variables as regime variables; they're now kindle constants).

## Manual Step

12. **Copy credentials** — `cp ~/.rbw/rbro.env ../station-files/secrets/rbro-payor.env`

## Acceptance

- `RBRR_SECRETS_DIR` is the single enrolled variable; four credential paths derived as kindle constants in lock step
- No remaining `RBCC_rbro_file` references
- No remaining hardcoded `~/.rbw` references in Tools/
- RBS0 linked terms updated for new/changed regime variables
- All existing operations (payor install, OAuth flow, credential validation) work unchanged
- Smoke tests pass

**[260224-1352] rough**

Consolidate credential file paths under `RBRR_SECRETS_DIR` and eliminate all hardcoded `~/.rbw` references.

Depends on ₢AkAAG (BURD_CONFIG_DIR, RBBC, .rbk/ infrastructure).

## RBRR Regime Changes

1. **`rbrr_regime.sh`** — Remove three `buv_string_enroll` for `RBRR_GOVERNOR_RBRA_FILE`, `RBRR_RETRIEVER_RBRA_FILE`, `RBRR_DIRECTOR_RBRA_FILE`. Add `buv_string_enroll RBRR_SECRETS_DIR` (validated via `buv_dir_exists` in enforce step). Add four readonly kindle constants in the lock step, derived from `RBRR_SECRETS_DIR`:
   - `RBRR_GOVERNOR_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-governor.env"`
   - `RBRR_RETRIEVER_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-retriever.env"`
   - `RBRR_DIRECTOR_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-director.env"`
   - `RBRR_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/rbro-payor.env"`

2. **`.rbk/rbrr.env`** — Remove three `RBRR_*_RBRA_FILE=` lines. Add `RBRR_SECRETS_DIR=../station-files/secrets`.

## RBRO/Payor Changes

3. **`rbro_regime.sh`** — Rewrite `rbro_load()` to use `RBRR_PAYOR_RBRO_FILE` instead of hardcoded `${HOME}/.rbw`. Remove directory check. RBRR must be kindled before rbro_load is called.

4. **`rbro_cli.sh`** — Replace `RBCC_rbro_file` with `RBRR_PAYOR_RBRO_FILE`. Ensure RBRR is sourced and kindled before RBRO load.

5. **`rbgp_Payor.sh`** — Write to `RBRR_PAYOR_RBRO_FILE` instead of `${HOME}/.rbw/rbro.env`. Create parent dir via `dirname`.

## RBCC Cleanup

6. **`rbcc_Constants.sh`** — Remove `RBCC_rbro_file="${HOME}/.rbw/rbro.env"`.

## Consumer Updates

7. **`butcrg_RegimeCredentials.sh`** — Replace hardcoded `${HOME}/.rbw/rbro.env` with `RBRR_PAYOR_RBRO_FILE`.

8. **`rbgm_ManualProcedures.sh`** — Update user-facing text referencing `~/.rbw/rbro.env`.

## Manual Step

9. **Copy credentials** — `cp ~/.rbw/rbro.env ../station-files/secrets/rbro-payor.env`

## Acceptance

- `RBRR_SECRETS_DIR` is the single enrolled variable; four credential paths derived as kindle constants
- No remaining `RBCC_rbro_file` references
- No remaining hardcoded `~/.rbw` references in Tools/
- All existing operations (payor install, OAuth flow, credential validation) work unchanged
- Smoke tests pass

### rbrr-load-helper (₢AkAAI) [abandoned]

**[260224-1530] abandoned**

Add `rbrr_load()` public function to `rbrr_regime.sh` that wraps the 4-line RBRR chain (source RBBC_rbrr_file + kindle + enforce + lock) into a single call. Then replace all furnish functions that manually perform this chain with a call to `rbrr_load()`.

## Changes

1. **`rbrr_regime.sh`** — Add public function:
   ```
   rbrr_load() {
     source "${RBBC_rbrr_file}" || buc_die "Failed to source RBRR: ${RBBC_rbrr_file}"
     zrbrr_kindle
     zrbrr_enforce
     zrbrr_lock
   }
   ```

2. **All CLI furnish functions** that currently do the manual chain — replace with `rbrr_load()`:
   - `rbgm_cli.sh`
   - `rbgp_cli.sh`
   - `rbro_cli.sh`
   - `butcrg_RegimeCredentials.sh` (rbra and rbro test cases)
   - Any others found via grep for `zrbrr_kindle`

## Acceptance

- `rbrr_load()` exists in `rbrr_regime.sh`
- No remaining manual `source RBBC_rbrr_file` + `zrbrr_kindle` + `zrbrr_enforce` + `zrbrr_lock` sequences outside `rbrr_load()`
- Smoke tests pass

**[260224-1530] rough**

Add `rbrr_load()` public function to `rbrr_regime.sh` that wraps the 4-line RBRR chain (source RBBC_rbrr_file + kindle + enforce + lock) into a single call. Then replace all furnish functions that manually perform this chain with a call to `rbrr_load()`.

## Changes

1. **`rbrr_regime.sh`** — Add public function:
   ```
   rbrr_load() {
     source "${RBBC_rbrr_file}" || buc_die "Failed to source RBRR: ${RBBC_rbrr_file}"
     zrbrr_kindle
     zrbrr_enforce
     zrbrr_lock
   }
   ```

2. **All CLI furnish functions** that currently do the manual chain — replace with `rbrr_load()`:
   - `rbgm_cli.sh`
   - `rbgp_cli.sh`
   - `rbro_cli.sh`
   - `butcrg_RegimeCredentials.sh` (rbra and rbro test cases)
   - Any others found via grep for `zrbrr_kindle`

## Acceptance

- `rbrr_load()` exists in `rbrr_regime.sh`
- No remaining manual `source RBBC_rbrr_file` + `zrbrr_kindle` + `zrbrr_enforce` + `zrbrr_lock` sequences outside `rbrr_load()`
- Smoke tests pass

### add-test-sweep-suites (₢AkAAC) [complete]

**[260225-1835] complete**

Rework BUK test registry to support cross-cutting test suites (N:M case-to-suite).

## Design decisions (settled)

- **Suite** = cross-cutting selection group (fast, service, complete). A case can be in multiple suites.
- **Fixture** = init/setup execution context (renamed from old "suite"). Each case has exactly one fixture.
- `butr_suite_enroll BUTR_SUITE_FAST BUTR_SUITE_COMPLETE` — replacement (not additive), sets current suite context for subsequent case enrollments. At least one suite required.
- `butr_case_enroll "fixture-name" case_fn` — assigns case to fixture + inherits current suite set.
- Suite constants are kindle `readonly` strings, defined in testbench. `set -u` catches typos.
- Dispatch by suite: collect cases, group by owning fixture, run each fixture's init/setup with only the selected cases.

## Open design question

- Rename existing `butr_suite_enroll` to `butr_fixture_enroll` (or other name for the init/setup grouping). "Fixture" is standard testing vocabulary for setup/teardown context.

## Files to change

1. `Tools/buk/butr_registry.sh` — Rename suite→fixture internally, add suite enrollment (sets current suite context), add `z_butr_case_suites_roll[]` parallel array, add `butr_suite_cases_recite`, `butr_suites_recite` query functions.
2. `Tools/buk/butd_dispatch.sh` — Refactor `butd_run_suite` to accept optional case list (default=all). Add `butd_run_sweep` entry point that queries by suite, groups by fixture, runs filtered.
3. `Tools/buk/bute_engine.sh` — No changes expected (runs individual cases).
4. `Tools/rbw/rbtb_testbench.sh` — Define suite constants (BUTR_SUITE_FAST, BUTR_SUITE_SERVICE, BUTR_SUITE_COMPLETE). Rename suite_enroll→fixture_enroll. Add suite_enroll calls before case groups. Categorize all 98 cases.
5. Tabtargets — New `rbw-tw.TestSweep.sh` for suite-based runs. Existing `rbw-ts.TestSuite.sh` renamed to fixture-based or kept as alias.
6. `Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc` — Document suite/fixture model.

## Acceptance

- `butr_case_enroll` fatals if no `butr_suite_enroll` has been called
- All 98 cases assigned to at least one suite
- `butd_run_sweep "fast"` runs only fast-tagged cases, grouped by fixture
- `butd_run_all` unchanged (runs all fixtures)
- Existing tabtargets (TestAll, TestSuite, TestOne) still work
- New TestSweep tabtarget works

**[260224-0843] rough**

Rework BUK test registry to support cross-cutting test suites (N:M case-to-suite).

## Design decisions (settled)

- **Suite** = cross-cutting selection group (fast, service, complete). A case can be in multiple suites.
- **Fixture** = init/setup execution context (renamed from old "suite"). Each case has exactly one fixture.
- `butr_suite_enroll BUTR_SUITE_FAST BUTR_SUITE_COMPLETE` — replacement (not additive), sets current suite context for subsequent case enrollments. At least one suite required.
- `butr_case_enroll "fixture-name" case_fn` — assigns case to fixture + inherits current suite set.
- Suite constants are kindle `readonly` strings, defined in testbench. `set -u` catches typos.
- Dispatch by suite: collect cases, group by owning fixture, run each fixture's init/setup with only the selected cases.

## Open design question

- Rename existing `butr_suite_enroll` to `butr_fixture_enroll` (or other name for the init/setup grouping). "Fixture" is standard testing vocabulary for setup/teardown context.

## Files to change

1. `Tools/buk/butr_registry.sh` — Rename suite→fixture internally, add suite enrollment (sets current suite context), add `z_butr_case_suites_roll[]` parallel array, add `butr_suite_cases_recite`, `butr_suites_recite` query functions.
2. `Tools/buk/butd_dispatch.sh` — Refactor `butd_run_suite` to accept optional case list (default=all). Add `butd_run_sweep` entry point that queries by suite, groups by fixture, runs filtered.
3. `Tools/buk/bute_engine.sh` — No changes expected (runs individual cases).
4. `Tools/rbw/rbtb_testbench.sh` — Define suite constants (BUTR_SUITE_FAST, BUTR_SUITE_SERVICE, BUTR_SUITE_COMPLETE). Rename suite_enroll→fixture_enroll. Add suite_enroll calls before case groups. Categorize all 98 cases.
5. Tabtargets — New `rbw-tw.TestSweep.sh` for suite-based runs. Existing `rbw-ts.TestSuite.sh` renamed to fixture-based or kept as alias.
6. `Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc` — Document suite/fixture model.

## Acceptance

- `butr_case_enroll` fatals if no `butr_suite_enroll` has been called
- All 98 cases assigned to at least one suite
- `butd_run_sweep "fast"` runs only fast-tagged cases, grouped by fixture
- `butd_run_all` unchanged (runs all fixtures)
- Existing tabtargets (TestAll, TestSuite, TestOne) still work
- New TestSweep tabtarget works

### kindle-mutable-state-naming (₢AkAAF) [complete]

**[260225-1908] complete**

Audit and fix kindle functions that initialize mutable state (counters, accumulators) alongside constants.

## Problem

`ZRBI_FILE_INDEX=0` in `zrbi_kindle()` is module-level mutable state initialized in kindle among readonly constants. The uppercase `Z`-prefixed name follows kindle constant convention but the variable is mutated post-kindle. This pattern likely exists in other modules.

## Scope

1. **Audit**: Scan all kindle functions for variables that are initialized in kindle but mutated after kindle returns. Identify the pattern across modules.
2. **Naming convention**: Establish a BCG convention distinguishing kindle mutable state from kindle constants — likely a lowercase `z` prefix or similar marker so the name signals mutability.
3. **Apply**: Rename identified variables to the new convention.
4. **BCG update**: Document the convention in the Readonly Patterns section (mutable kindle state as an exception to the "every kindle constant" rule, with naming discipline).

## Acceptance

- All kindle-initialized mutable state uses the new naming convention
- BCG documents the pattern and naming rule
- No functional changes — rename only

**[260225-1908] complete**

Audit and fix kindle functions that initialize mutable state (counters, accumulators) alongside constants.

## Problem

`ZRBI_FILE_INDEX=0` in `zrbi_kindle()` is module-level mutable state initialized in kindle among readonly constants. The uppercase `Z`-prefixed name follows kindle constant convention but the variable is mutated post-kindle. This pattern likely exists in other modules.

## Scope

1. **Audit**: Scan all kindle functions for variables that are initialized in kindle but mutated after kindle returns. Identify the pattern across modules.
2. **Naming convention**: Establish a BCG convention distinguishing kindle mutable state from kindle constants — likely a lowercase `z` prefix or similar marker so the name signals mutability.
3. **Apply**: Rename identified variables to the new convention.
4. **BCG update**: Document the convention in the Readonly Patterns section (mutable kindle state as an exception to the "every kindle constant" rule, with naming discipline).

## Acceptance

- All kindle-initialized mutable state uses the new naming convention
- BCG documents the pattern and naming rule
- No functional changes — rename only

**[260224-1124] rough**

Audit and fix kindle functions that initialize mutable state (counters, accumulators) alongside constants.

## Problem

`ZRBI_FILE_INDEX=0` in `zrbi_kindle()` is module-level mutable state initialized in kindle among readonly constants. The uppercase `Z`-prefixed name follows kindle constant convention but the variable is mutated post-kindle. This pattern likely exists in other modules.

## Scope

1. **Audit**: Scan all kindle functions for variables that are initialized in kindle but mutated after kindle returns. Identify the pattern across modules.
2. **Naming convention**: Establish a BCG convention distinguishing kindle mutable state from kindle constants — likely a lowercase `z` prefix or similar marker so the name signals mutability.
3. **Apply**: Rename identified variables to the new convention.
4. **BCG update**: Document the convention in the Readonly Patterns section (mutable kindle state as an exception to the "every kindle constant" rule, with naming discipline).

## Acceptance

- All kindle-initialized mutable state uses the new naming convention
- BCG documents the pattern and naming rule
- No functional changes — rename only

### suite-infrastructure-preconditions (₢AkAAA) [complete]

**[260307-1925] complete**

Introduce litmus predicates and baste functions to the test framework, replacing the unused init/tsuite_setup vocabulary.

## What changes

### New test vocabulary

| Suffix | Role | Contract |
|--------|------|----------|
| `_litmus_predicate` | Can this fixture run? | 0/1, never dies, no output, composable |
| `_baste` | Prepare fixture environment | Kindle, source, configure; runs in fixture subshell |
| `_tcase` | Test case (unchanged) | Assertions, verification; runs in case subshell |

Eliminates `_tsuite_init` (never used) and `_tsuite_setup` (misnamed — it's fixture-level, not suite-level).

### File changes

**`Tools/buk/butr_registry.sh`**
- Rename `z_butr_init_roll` → `z_butr_litmus_roll`
- Rename `z_butr_setup_roll` → `z_butr_baste_roll`
- Rename `butr_init_recite` → `butr_litmus_recite`
- Rename `butr_setup_recite` → `butr_baste_recite`
- Update param names in `butr_fixture_enroll` (init_fn → litmus_fn, setup_fn → baste_fn)

**`Tools/buk/butd_dispatch.sh`**
- Update `butr_init_recite` → `butr_litmus_recite`
- Update `butr_setup_recite` → `butr_baste_recite`
- Update skip warning to log litmus function name: "Fixture 'X' skipped (litmus: zrbtb_container_runtime_litmus_predicate)"

**`Tools/rbw/rbtb_testbench.sh`**
- Define 3 litmus predicates:
  - `zrbtb_container_runtime_litmus_predicate` — `timeout 5 docker version`
  - `zrbtb_clean_git_litmus_predicate` — `git diff-index --quiet HEAD`
  - `zrbtb_container_clean_git_litmus_predicate` — calls both
- Rename all 12 `_tsuite_setup` functions → `_baste`
- Wire litmus into fixture enrollment:
  - ark-lifecycle, slsa-provenance → `zrbtb_container_clean_git_litmus_predicate`
  - nsproto-security, srjcl-jupyter, pluml-diagram → `zrbtb_container_runtime_litmus_predicate`
  - All others → `""` (no litmus)
- Remove `git diff-index` checks from ark and slsa baste functions (litmus handles it)

**`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`**
- Replace `_tsuite_init` row with `_litmus_predicate` in special function patterns table
- Replace `_tsuite_setup` with `_baste` in special function patterns table
- Update naming convention table
- Update "When to Use Special Functions" section

## Acceptance

- `rbw-ta` completes all fast/service suites even when container runtime is unavailable
- Container-dependent fixtures report skipped (not failed) with litmus function name in warning
- No `_tsuite_init` or `_tsuite_setup` references remain in BCG or test code
- All existing tests still pass when container runtime IS available

**[260307-1841] rough**

Introduce litmus predicates and baste functions to the test framework, replacing the unused init/tsuite_setup vocabulary.

## What changes

### New test vocabulary

| Suffix | Role | Contract |
|--------|------|----------|
| `_litmus_predicate` | Can this fixture run? | 0/1, never dies, no output, composable |
| `_baste` | Prepare fixture environment | Kindle, source, configure; runs in fixture subshell |
| `_tcase` | Test case (unchanged) | Assertions, verification; runs in case subshell |

Eliminates `_tsuite_init` (never used) and `_tsuite_setup` (misnamed — it's fixture-level, not suite-level).

### File changes

**`Tools/buk/butr_registry.sh`**
- Rename `z_butr_init_roll` → `z_butr_litmus_roll`
- Rename `z_butr_setup_roll` → `z_butr_baste_roll`
- Rename `butr_init_recite` → `butr_litmus_recite`
- Rename `butr_setup_recite` → `butr_baste_recite`
- Update param names in `butr_fixture_enroll` (init_fn → litmus_fn, setup_fn → baste_fn)

**`Tools/buk/butd_dispatch.sh`**
- Update `butr_init_recite` → `butr_litmus_recite`
- Update `butr_setup_recite` → `butr_baste_recite`
- Update skip warning to log litmus function name: "Fixture 'X' skipped (litmus: zrbtb_container_runtime_litmus_predicate)"

**`Tools/rbw/rbtb_testbench.sh`**
- Define 3 litmus predicates:
  - `zrbtb_container_runtime_litmus_predicate` — `timeout 5 docker version`
  - `zrbtb_clean_git_litmus_predicate` — `git diff-index --quiet HEAD`
  - `zrbtb_container_clean_git_litmus_predicate` — calls both
- Rename all 12 `_tsuite_setup` functions → `_baste`
- Wire litmus into fixture enrollment:
  - ark-lifecycle, slsa-provenance → `zrbtb_container_clean_git_litmus_predicate`
  - nsproto-security, srjcl-jupyter, pluml-diagram → `zrbtb_container_runtime_litmus_predicate`
  - All others → `""` (no litmus)
- Remove `git diff-index` checks from ark and slsa baste functions (litmus handles it)

**`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`**
- Replace `_tsuite_init` row with `_litmus_predicate` in special function patterns table
- Replace `_tsuite_setup` with `_baste` in special function patterns table
- Update naming convention table
- Update "When to Use Special Functions" section

## Acceptance

- `rbw-ta` completes all fast/service suites even when container runtime is unavailable
- Container-dependent fixtures report skipped (not failed) with litmus function name in warning
- No `_tsuite_init` or `_tsuite_setup` references remain in BCG or test code
- All existing tests still pass when container runtime IS available

**[260224-0722] rough**

Drafted from ₢APAAm in ₣AP.

Prevent test-all from failing fast when infrastructure prerequisites are missing.

## Problem

`rbw-ta` (TestAll) stops at the first suite failure. Suites like ark-lifecycle fail immediately when the Podman VM isn't running, blocking all downstream suites (including CI-safe ones like regime-smoke and xname-validation) from executing.

Additionally, some test cases hang indefinitely when they issue a docker/podman command and the daemon isn't available (first-time startup or VM not running).

A third failure mode discovered during ₣AP ₢APAAo: on macOS, `docker manifest inspect` hangs indefinitely when the TCC (Transparency, Consent, and Control) system presents an invisible modal dialog — "iTerm would like to access data from other apps." This blocks the Docker CLI socket connection with no timeout and no error. The dialog may not be visible if the terminal is not in focus. Once "Allow" is clicked the permission is cached in TCC.db permanently, but first-time users or fresh macOS installs will hit this silently.

## Solution

Add infrastructure precondition checks to suites that require container runtime:

1. **Suite-level precondition**: In setup functions for infrastructure-dependent suites (e.g., `zrbtb_setup_ark`), verify the container runtime is available before running cases. If unavailable, mark the suite as **inconclusive** (not failed) using the existing BUT inconclusive mechanism from the ₣Ac test overhaul.

2. **Timeout protection**: Ensure docker/podman commands in test cases have timeouts so they cannot hang indefinitely when the daemon is unresponsive. A lightweight probe like `timeout 5 docker version` catches both "VM not running" and "TCC dialog blocking socket access" — both present as unresponsive Docker CLI.

3. **TCC-specific diagnostic**: If the timeout probe fails on macOS (detected via `uname -s`), include an actionable message: "Docker CLI not responding. If macOS prompted 'iTerm would like to access data from other apps', click Allow and retry."

4. **Scope**: ark-lifecycle, nameplate suites (nsproto, srjcl, pluml), and any other suite that dispatches container operations.

## Acceptance

- `rbw-ta` completes all CI-safe suites even when Podman VM is not running
- Infrastructure-dependent suites report inconclusive (not failed) when preconditions unmet
- No test case can hang indefinitely on a missing container runtime or TCC dialog
- macOS TCC failure produces a clear, actionable diagnostic within 10 seconds

**[260216-1016] rough**

Prevent test-all from failing fast when infrastructure prerequisites are missing.

## Problem

`rbw-ta` (TestAll) stops at the first suite failure. Suites like ark-lifecycle fail immediately when the Podman VM isn't running, blocking all downstream suites (including CI-safe ones like regime-smoke and xname-validation) from executing.

Additionally, some test cases hang indefinitely when they issue a docker/podman command and the daemon isn't available (first-time startup or VM not running).

A third failure mode discovered during ₣AP ₢APAAo: on macOS, `docker manifest inspect` hangs indefinitely when the TCC (Transparency, Consent, and Control) system presents an invisible modal dialog — "iTerm would like to access data from other apps." This blocks the Docker CLI socket connection with no timeout and no error. The dialog may not be visible if the terminal is not in focus. Once "Allow" is clicked the permission is cached in TCC.db permanently, but first-time users or fresh macOS installs will hit this silently.

## Solution

Add infrastructure precondition checks to suites that require container runtime:

1. **Suite-level precondition**: In setup functions for infrastructure-dependent suites (e.g., `zrbtb_setup_ark`), verify the container runtime is available before running cases. If unavailable, mark the suite as **inconclusive** (not failed) using the existing BUT inconclusive mechanism from the ₣Ac test overhaul.

2. **Timeout protection**: Ensure docker/podman commands in test cases have timeouts so they cannot hang indefinitely when the daemon is unresponsive. A lightweight probe like `timeout 5 docker version` catches both "VM not running" and "TCC dialog blocking socket access" — both present as unresponsive Docker CLI.

3. **TCC-specific diagnostic**: If the timeout probe fails on macOS (detected via `uname -s`), include an actionable message: "Docker CLI not responding. If macOS prompted 'iTerm would like to access data from other apps', click Allow and retry."

4. **Scope**: ark-lifecycle, nameplate suites (nsproto, srjcl, pluml), and any other suite that dispatches container operations.

## Acceptance

- `rbw-ta` completes all CI-safe suites even when Podman VM is not running
- Infrastructure-dependent suites report inconclusive (not failed) when preconditions unmet
- No test case can hang indefinitely on a missing container runtime or TCC dialog
- macOS TCC failure produces a clear, actionable diagnostic within 10 seconds

**[260216-0612] rough**

Prevent test-all from failing fast when infrastructure prerequisites are missing.

## Problem

`rbw-ta` (TestAll) stops at the first suite failure. Suites like ark-lifecycle fail immediately when the Podman VM isn't running, blocking all downstream suites (including CI-safe ones like regime-smoke and xname-validation) from executing.

Additionally, some test cases hang indefinitely when they issue a docker/podman command and the daemon isn't available (first-time startup or VM not running).

## Solution

Add infrastructure precondition checks to suites that require container runtime:

1. **Suite-level precondition**: In setup functions for infrastructure-dependent suites (e.g., `zrbtb_setup_ark`), verify the container runtime is available before running cases. If unavailable, mark the suite as **inconclusive** (not failed) using the existing BUT inconclusive mechanism from the ₣Ac test overhaul.

2. **Timeout protection**: Ensure docker/podman commands in test cases have timeouts so they cannot hang indefinitely when the daemon is unresponsive.

3. **Scope**: ark-lifecycle, nameplate suites (nsproto, srjcl, pluml), and any other suite that dispatches container operations.

## Acceptance

- `rbw-ta` completes all CI-safe suites even when Podman VM is not running
- Infrastructure-dependent suites report inconclusive (not failed) when preconditions unmet
- No test case can hang indefinitely on a missing container runtime

### rename-rubric-inscribe-tabtarget (₢AkAAL) [abandoned]

**[260308-1341] abandoned**

Drafted from ₢AiAAm in ₣Ai.

Rename tabtarget tt/rbw-RI.RubricInscribe.sh. New name TBD — requires minting decision when pace is worked. Current colophon rbw-RI with frontispiece RubricInscribe.

**[260302-0713] rough**

Drafted from ₢AiAAm in ₣Ai.

Rename tabtarget tt/rbw-RI.RubricInscribe.sh. New name TBD — requires minting decision when pace is worked. Current colophon rbw-RI with frontispiece RubricInscribe.

**[260227-1533] rough**

Rename tabtarget tt/rbw-RI.RubricInscribe.sh. New name TBD — requires minting decision when pace is worked. Current colophon rbw-RI with frontispiece RubricInscribe.

### director-colophon-reorg (₢AkAAM) [complete]

**[260302-0713] complete**

Drafted from ₢AiAAs in ₣Ai.

Reorganize Director-affiliated tabtargets under rbw-D* colophon prefix.

## Context

Governor operations use rbw-G* prefix consistently. Director operations are
scattered across rbw-i* (image), rbw-a* (ark), rbw-RI (inscribe), and
rbw-rrg (pins). This pace consolidates Director-affiliated tabtargets under
rbw-D* for consistency.

## Inventory

Current Director-affiliated tabtargets:

Ark operations (high-level, vessel-scoped):
- rbw-aC ConjureArk → rbf_build (build image from vessel)
- rbw-aA AbjureArk → rbf_abjure (delete ark, vessel-scoped)
- rbw-as SummonArk → rbf_summon (pull ark)
- rbw-ab BeseechArk → rbf_beseech (list arks with correlation)

Image operations (low-level, locator-scoped):
- rbw-iB BuildImageRemotely → rbf_build (DUPLICATE of rbw-aC)
- rbw-iD DeleteImage → rbf_delete (delete by moniker:tag)
- rbw-il ImageList → rbf_list (raw list of locators)
- rbw-ir RetrieveImage → rbf_retrieve (pull by moniker:tag)

Rubric/build infrastructure:
- rbw-RI RubricInscribe → rbf_inscribe
- rbw-rrg RefreshGcbPins → rbrr_refresh_gcb_pins

## Proposed Mapping (requires human review)

Candidate rbw-D* colophons — present to user for approval before executing:

| Proposed   | Description                  | Replaces       |
|------------|------------------------------|----------------|
| rbw-DC     | DirectorConjuresArk          | rbw-aC, rbw-iB |
| rbw-DA     | DirectorAbjuresArk           | rbw-aA         |
| rbw-DS     | DirectorSummonsArk           | rbw-as         |
| rbw-DB     | DirectorBeseechesArk         | rbw-ab         |
| rbw-DI     | DirectorInscribesRubric      | rbw-RI         |
| rbw-DP     | DirectorRefreshesPins        | rbw-rrg        |
| rbw-Dd     | DirectorDeletesImage         | rbw-iD         |
| rbw-Dl     | DirectorListsImages          | rbw-il         |
| rbw-Dr     | DirectorRetrievesImage       | rbw-ir         |

## CRITICAL: Human review gate

Do NOT proceed with renames without presenting the full mapping to the user
and getting explicit approval. The colophon choices affect tab-completion
ergonomics, terminal exclusivity rules, and muscle memory. This is a design
decision, not a mechanical transformation.

## Execution (after approval)

1. Rename tabtarget files in tt/
2. Update rbz_zipper.sh colophon strings
3. Update any buc_next references in code
4. Retire rbw-iB (duplicate of conjure)
5. Verify no stale colophon references remain

**[260301-1540] complete**

Reorganize Director-affiliated tabtargets under rbw-D* colophon prefix.

## Context

Governor operations use rbw-G* prefix consistently. Director operations are
scattered across rbw-i* (image), rbw-a* (ark), rbw-RI (inscribe), and
rbw-rrg (pins). This pace consolidates Director-affiliated tabtargets under
rbw-D* for consistency.

## Inventory

Current Director-affiliated tabtargets:

Ark operations (high-level, vessel-scoped):
- rbw-aC ConjureArk → rbf_build (build image from vessel)
- rbw-aA AbjureArk → rbf_abjure (delete ark, vessel-scoped)
- rbw-as SummonArk → rbf_summon (pull ark)
- rbw-ab BeseechArk → rbf_beseech (list arks with correlation)

Image operations (low-level, locator-scoped):
- rbw-iB BuildImageRemotely → rbf_build (DUPLICATE of rbw-aC)
- rbw-iD DeleteImage → rbf_delete (delete by moniker:tag)
- rbw-il ImageList → rbf_list (raw list of locators)
- rbw-ir RetrieveImage → rbf_retrieve (pull by moniker:tag)

Rubric/build infrastructure:
- rbw-RI RubricInscribe → rbf_inscribe
- rbw-rrg RefreshGcbPins → rbrr_refresh_gcb_pins

## Proposed Mapping (requires human review)

Candidate rbw-D* colophons — present to user for approval before executing:

| Proposed   | Description                  | Replaces       |
|------------|------------------------------|----------------|
| rbw-DC     | DirectorConjuresArk          | rbw-aC, rbw-iB |
| rbw-DA     | DirectorAbjuresArk           | rbw-aA         |
| rbw-DS     | DirectorSummonsArk           | rbw-as         |
| rbw-DB     | DirectorBeseechesArk         | rbw-ab         |
| rbw-DI     | DirectorInscribesRubric      | rbw-RI         |
| rbw-DP     | DirectorRefreshesPins        | rbw-rrg        |
| rbw-Dd     | DirectorDeletesImage         | rbw-iD         |
| rbw-Dl     | DirectorListsImages          | rbw-il         |
| rbw-Dr     | DirectorRetrievesImage       | rbw-ir         |

## CRITICAL: Human review gate

Do NOT proceed with renames without presenting the full mapping to the user
and getting explicit approval. The colophon choices affect tab-completion
ergonomics, terminal exclusivity rules, and muscle memory. This is a design
decision, not a mechanical transformation.

## Execution (after approval)

1. Rename tabtarget files in tt/
2. Update rbz_zipper.sh colophon strings
3. Update any buc_next references in code
4. Retire rbw-iB (duplicate of conjure)
5. Verify no stale colophon references remain

**[260301-1145] rough**

Reorganize Director-affiliated tabtargets under rbw-D* colophon prefix.

## Context

Governor operations use rbw-G* prefix consistently. Director operations are
scattered across rbw-i* (image), rbw-a* (ark), rbw-RI (inscribe), and
rbw-rrg (pins). This pace consolidates Director-affiliated tabtargets under
rbw-D* for consistency.

## Inventory

Current Director-affiliated tabtargets:

Ark operations (high-level, vessel-scoped):
- rbw-aC ConjureArk → rbf_build (build image from vessel)
- rbw-aA AbjureArk → rbf_abjure (delete ark, vessel-scoped)
- rbw-as SummonArk → rbf_summon (pull ark)
- rbw-ab BeseechArk → rbf_beseech (list arks with correlation)

Image operations (low-level, locator-scoped):
- rbw-iB BuildImageRemotely → rbf_build (DUPLICATE of rbw-aC)
- rbw-iD DeleteImage → rbf_delete (delete by moniker:tag)
- rbw-il ImageList → rbf_list (raw list of locators)
- rbw-ir RetrieveImage → rbf_retrieve (pull by moniker:tag)

Rubric/build infrastructure:
- rbw-RI RubricInscribe → rbf_inscribe
- rbw-rrg RefreshGcbPins → rbrr_refresh_gcb_pins

## Proposed Mapping (requires human review)

Candidate rbw-D* colophons — present to user for approval before executing:

| Proposed   | Description                  | Replaces       |
|------------|------------------------------|----------------|
| rbw-DC     | DirectorConjuresArk          | rbw-aC, rbw-iB |
| rbw-DA     | DirectorAbjuresArk           | rbw-aA         |
| rbw-DS     | DirectorSummonsArk           | rbw-as         |
| rbw-DB     | DirectorBeseechesArk         | rbw-ab         |
| rbw-DI     | DirectorInscribesRubric      | rbw-RI         |
| rbw-DP     | DirectorRefreshesPins        | rbw-rrg        |
| rbw-Dd     | DirectorDeletesImage         | rbw-iD         |
| rbw-Dl     | DirectorListsImages          | rbw-il         |
| rbw-Dr     | DirectorRetrievesImage       | rbw-ir         |

## CRITICAL: Human review gate

Do NOT proceed with renames without presenting the full mapping to the user
and getting explicit approval. The colophon choices affect tab-completion
ergonomics, terminal exclusivity rules, and muscle memory. This is a design
decision, not a mechanical transformation.

## Execution (after approval)

1. Rename tabtarget files in tt/
2. Update rbz_zipper.sh colophon strings
3. Update any buc_next references in code
4. Retire rbw-iB (duplicate of conjure)
5. Verify no stale colophon references remain

### shellcheck-bcg-exemption-catalogue (₢AkAAQ) [complete]

**[260309-2045] complete**

Evaluate shellcheck as a bash module quality gate, with BCG-aware exemptions.

REQUIRES HUMAN CONVERSATION — this pace should teach the human about shellcheck
capabilities and limitations before making integration decisions.

## Context

BCG intentionally deviates from several shellcheck recommendations:
- || buc_die pattern
- Two-line capture pattern
- Bash 3.2 compat constraints (no associative arrays, no mapfile, etc.)
- Possibly others discovered during the survey

Running shellcheck naively produces noise that trains people and LLMs to ignore output.
A curated disable list catches genuine errors without fighting BCG.

## Open question: scope

Does shellcheck cover Tools/rbw/ only, or also Tools/buk/? BUK *defines* BCG
patterns — checking the pattern source itself may surface different issues than
checking consumers of those patterns. Flag for human decision during pace.

## Discovery step (FIRST)

Run shellcheck on two representative files and classify every warning as
BCG-intentional or genuine catch:
- rbgu_Utility.sh (library module)
- One _cli.sh file (entrypoint module — survey existing _cli files during pace to pick)

The classification catalogue is the primary output of discovery. It documents WHY
each suppression exists, which is valuable if BCG ever evolves.

## Deliverables

1. Shellcheck discovery run — full warning catalogue with BCG classification
2. Human-reviewed decision on integration approach (config file format, per-directory
   vs global, inline directives vs config — present options during pace, don't pre-decide)
3. If integration approved: tabtarget or qualification check that runs shellcheck
4. Add the check to RBS0 Release Compliance section

## Sequencing

Depends on ₢AkAAO (Release Compliance section must exist first).

## Not in scope
- Fixing all shellcheck findings (those become their own paces)
- Changing BCG patterns to satisfy shellcheck
- Pre-deciding shellcheck configuration format (human learns during pace)

**[260304-1502] rough**

Evaluate shellcheck as a bash module quality gate, with BCG-aware exemptions.

REQUIRES HUMAN CONVERSATION — this pace should teach the human about shellcheck
capabilities and limitations before making integration decisions.

## Context

BCG intentionally deviates from several shellcheck recommendations:
- || buc_die pattern
- Two-line capture pattern
- Bash 3.2 compat constraints (no associative arrays, no mapfile, etc.)
- Possibly others discovered during the survey

Running shellcheck naively produces noise that trains people and LLMs to ignore output.
A curated disable list catches genuine errors without fighting BCG.

## Open question: scope

Does shellcheck cover Tools/rbw/ only, or also Tools/buk/? BUK *defines* BCG
patterns — checking the pattern source itself may surface different issues than
checking consumers of those patterns. Flag for human decision during pace.

## Discovery step (FIRST)

Run shellcheck on two representative files and classify every warning as
BCG-intentional or genuine catch:
- rbgu_Utility.sh (library module)
- One _cli.sh file (entrypoint module — survey existing _cli files during pace to pick)

The classification catalogue is the primary output of discovery. It documents WHY
each suppression exists, which is valuable if BCG ever evolves.

## Deliverables

1. Shellcheck discovery run — full warning catalogue with BCG classification
2. Human-reviewed decision on integration approach (config file format, per-directory
   vs global, inline directives vs config — present options during pace, don't pre-decide)
3. If integration approved: tabtarget or qualification check that runs shellcheck
4. Add the check to RBS0 Release Compliance section

## Sequencing

Depends on ₢AkAAO (Release Compliance section must exist first).

## Not in scope
- Fixing all shellcheck findings (those become their own paces)
- Changing BCG patterns to satisfy shellcheck
- Pre-deciding shellcheck configuration format (human learns during pace)

**[260304-1445] rough**

Evaluate shellcheck as a bash module quality gate, with BCG-aware exemptions.

REQUIRES HUMAN CONVERSATION — this pace should teach the human about shellcheck
capabilities and limitations before making integration decisions.

## Context

BCG intentionally deviates from several shellcheck recommendations:
- || buc_die pattern
- Two-line capture pattern
- Bash 3.2 compat constraints (no associative arrays, no mapfile, etc.)
- Possibly others discovered during the survey

Running shellcheck naively produces noise that trains people and LLMs to ignore output.
A curated disable list catches genuine errors without fighting BCG.

## Discovery step (FIRST)

Run shellcheck on two representative files and classify every warning as
BCG-intentional or genuine catch:
- rbgu_Utility.sh (library module)
- One _cli.sh file (entrypoint module — survey existing _cli files during pace to pick)

The classification catalogue is the primary output of discovery. It documents WHY
each suppression exists, which is valuable if BCG ever evolves.

## Deliverables

1. Shellcheck discovery run — full warning catalogue with BCG classification
2. Human-reviewed decision on integration approach (config file format, per-directory
   vs global, inline directives vs config — present options during pace, don't pre-decide)
3. If integration approved: tabtarget or qualification check that runs shellcheck
4. Add the check to RBS0 Release Compliance section

## Sequencing

Depends on ₢AkAAO (Release Compliance section must exist first).

## Not in scope
- Fixing all shellcheck findings (those become their own paces)
- Changing BCG patterns to satisfy shellcheck
- Pre-deciding shellcheck configuration format (human learns during pace)

**[260304-1429] rough**

Integrate shellcheck with a curated BCG exemption list for bash module checking.

REQUIRES HUMAN CONVERSATION — BCG deviations from shellcheck need cataloguing.

## Context

BCG intentionally deviates from several shellcheck recommendations:
- || buc_die pattern
- Two-line capture pattern
- Bash 3.2 compat constraints (no associative arrays, no mapfile, etc.)
- Possibly others discovered during the survey

Running shellcheck naively produces noise that trains people and LLMs to ignore output.
A curated disable list catches genuine errors without fighting BCG.

## Discovery step (FIRST)

Run shellcheck on one representative file (rbgu_Utility.sh recommended) and classify
every warning as:
- BCG-intentional: document why, add to disable list
- Genuine catch: these are the value — errors shellcheck finds that we want

The classification catalogue is the primary output of discovery. It documents WHY
each suppression exists, which is valuable if BCG ever evolves.

## Deliverables

1. Shellcheck discovery run on rbgu_Utility.sh — full warning catalogue with BCG classification
2. .shellcheckrc or equivalent config with curated disable list
3. Tabtarget or qualification check that runs shellcheck across rbw modules
4. Add the check to RBS0 Release Compliance section

## Not in scope
- Fixing all shellcheck findings (those become their own paces)
- Changing BCG patterns to satisfy shellcheck

### extract-rbrg-rbrm-from-rbrr (₢AkAAY) [complete]

**[260307-2011] complete**

Extract two variable groups from rbrr.env into dedicated config regimes.

## Group 1: GCB Image Pins -> RBRG regime

Prefix: RBRG_ (Google Cloud Build items). File: .rbk/rbrg.env.
Spec doc: lenses/RBSRG-RegimeGcbPins.adoc.

GCB image pins (RBRR_GCB_*_IMAGE_REF, RBRR_GCB_PINS_REFRESHED_AT) live inside
rbrr.env alongside regime identity variables. The pin refresh command
(rbrr_refresh_gcb_pins) edits rbrr.env in-place via sed, which is
manifestly BCG incompliant.

Variables moving (with rename):
- RBRR_GCB_ORAS_IMAGE_REF -> RBRG_ORAS_IMAGE_REF
- RBRR_GCB_GCLOUD_IMAGE_REF -> RBRG_GCLOUD_IMAGE_REF
- RBRR_GCB_DOCKER_IMAGE_REF -> RBRG_DOCKER_IMAGE_REF
- RBRR_GCB_ALPINE_IMAGE_REF -> RBRG_ALPINE_IMAGE_REF
- RBRR_GCB_SYFT_IMAGE_REF -> RBRG_SYFT_IMAGE_REF
- RBRR_GCB_BINFMT_IMAGE_REF -> RBRG_BINFMT_IMAGE_REF
- RBRR_GCB_SKOPEO_IMAGE_REF -> RBRG_SKOPEO_IMAGE_REF
- RBRR_GCB_PINS_REFRESHED_AT -> RBRG_PINS_REFRESHED_AT

Work:
- Create .rbk/rbrg.env with all pin variables
- Create lenses/RBSRG-RegimeGcbPins.adoc regime spec
- Create RBRG regime kindle/validate/render in appropriate module
- Rewrite pin refresh to generate entire rbrg.env from scratch
  (discover -> resolve -> write complete file, BCG compliant)
- Source rbrg.env from rbrr regime or stitch function
- Remove pin variables from rbrr.env
- Update all consumers of renamed variables

## Group 2: Podman VM Supply Chain -> RBRM regime

Prefix: RBRM_ (Machine/VM supply chain). File: .rbk/rbrm.env.
Spec doc: lenses/RBSRM-RegimeMachine.adoc.

Podman VM supply chain variables share a collection moment (operator
chooses a Podman release) and Podman support is deferred.

Variables moving (with rename):
- RBRR_MANIFEST_PLATFORMS -> RBRM_MANIFEST_PLATFORMS
- RBRR_CHOSEN_PODMAN_VERSION -> RBRM_CHOSEN_PODMAN_VERSION
- RBRR_CHOSEN_VMIMAGE_ORIGIN -> RBRM_CHOSEN_VMIMAGE_ORIGIN
- RBRR_CHOSEN_IDENTITY -> RBRM_CHOSEN_IDENTITY
- RBRR_CRANE_TAR_GZ -> RBRM_CRANE_TAR_GZ

Work:
- Create .rbk/rbrm.env with all VM supply chain variables
- Create lenses/RBSRM-RegimeMachine.adoc regime spec
- Create RBRM regime kindle/validate/render in appropriate module
- Remove VM supply chain variables from rbrr.env
- Update all consumers of renamed variables
  (primarily rbv_PodmanVM.sh, rbrr_regime.sh)

## Variables explicitly staying in RBRR

- RBRR_GCB_MACHINE_TYPE, RBRR_GCB_TIMEOUT, RBRR_GCB_MIN_CONCURRENT_BUILDS
  (operator-chosen build parameters, not auto-refreshed pins)
- RBRR_GCB_WORKER_POOL (depot infrastructure)
- Depot identity, CBv2/rubric, secrets dir, DNS, machine names, vessel dir

## Spec document updates

- Create lenses/RBSRG-RegimeGcbPins.adoc
- Create lenses/RBSRM-RegimeMachine.adoc
- Update lenses/RBSRR-RegimeRepo.adoc (remove extracted groups)
- Update lenses/RBS0-SpecTop.adoc (add RBRG/RBRM regime definitions)

## References

- Tools/rbw/rbrr_cli.sh -- current pin refresh (rbrr_refresh_gcb_pins)
- .rbk/rbrr.env -- current storage for both groups
- Tools/rbw/rbrr_regime.sh -- regime validation
- Tools/rbw/rbf_Foundry.sh -- stitch function (pin consumer)
- Tools/rbw/rbv_PodmanVM.sh -- crane and CHOSEN_*/MANIFEST_PLATFORMS consumer
- Tools/buk/vov_veiled/BCG-BashConsoleGuide.md -- BCG patterns
- lenses/RBSRR-RegimeRepo.adoc -- regime spec (groups to update)
- lenses/RBS0-SpecTop.adoc -- top spec (regime definitions)

**[260307-1920] rough**

Extract two variable groups from rbrr.env into dedicated config regimes.

## Group 1: GCB Image Pins -> RBRG regime

Prefix: RBRG_ (Google Cloud Build items). File: .rbk/rbrg.env.
Spec doc: lenses/RBSRG-RegimeGcbPins.adoc.

GCB image pins (RBRR_GCB_*_IMAGE_REF, RBRR_GCB_PINS_REFRESHED_AT) live inside
rbrr.env alongside regime identity variables. The pin refresh command
(rbrr_refresh_gcb_pins) edits rbrr.env in-place via sed, which is
manifestly BCG incompliant.

Variables moving (with rename):
- RBRR_GCB_ORAS_IMAGE_REF -> RBRG_ORAS_IMAGE_REF
- RBRR_GCB_GCLOUD_IMAGE_REF -> RBRG_GCLOUD_IMAGE_REF
- RBRR_GCB_DOCKER_IMAGE_REF -> RBRG_DOCKER_IMAGE_REF
- RBRR_GCB_ALPINE_IMAGE_REF -> RBRG_ALPINE_IMAGE_REF
- RBRR_GCB_SYFT_IMAGE_REF -> RBRG_SYFT_IMAGE_REF
- RBRR_GCB_BINFMT_IMAGE_REF -> RBRG_BINFMT_IMAGE_REF
- RBRR_GCB_SKOPEO_IMAGE_REF -> RBRG_SKOPEO_IMAGE_REF
- RBRR_GCB_PINS_REFRESHED_AT -> RBRG_PINS_REFRESHED_AT

Work:
- Create .rbk/rbrg.env with all pin variables
- Create lenses/RBSRG-RegimeGcbPins.adoc regime spec
- Create RBRG regime kindle/validate/render in appropriate module
- Rewrite pin refresh to generate entire rbrg.env from scratch
  (discover -> resolve -> write complete file, BCG compliant)
- Source rbrg.env from rbrr regime or stitch function
- Remove pin variables from rbrr.env
- Update all consumers of renamed variables

## Group 2: Podman VM Supply Chain -> RBRM regime

Prefix: RBRM_ (Machine/VM supply chain). File: .rbk/rbrm.env.
Spec doc: lenses/RBSRM-RegimeMachine.adoc.

Podman VM supply chain variables share a collection moment (operator
chooses a Podman release) and Podman support is deferred.

Variables moving (with rename):
- RBRR_MANIFEST_PLATFORMS -> RBRM_MANIFEST_PLATFORMS
- RBRR_CHOSEN_PODMAN_VERSION -> RBRM_CHOSEN_PODMAN_VERSION
- RBRR_CHOSEN_VMIMAGE_ORIGIN -> RBRM_CHOSEN_VMIMAGE_ORIGIN
- RBRR_CHOSEN_IDENTITY -> RBRM_CHOSEN_IDENTITY
- RBRR_CRANE_TAR_GZ -> RBRM_CRANE_TAR_GZ

Work:
- Create .rbk/rbrm.env with all VM supply chain variables
- Create lenses/RBSRM-RegimeMachine.adoc regime spec
- Create RBRM regime kindle/validate/render in appropriate module
- Remove VM supply chain variables from rbrr.env
- Update all consumers of renamed variables
  (primarily rbv_PodmanVM.sh, rbrr_regime.sh)

## Variables explicitly staying in RBRR

- RBRR_GCB_MACHINE_TYPE, RBRR_GCB_TIMEOUT, RBRR_GCB_MIN_CONCURRENT_BUILDS
  (operator-chosen build parameters, not auto-refreshed pins)
- RBRR_GCB_WORKER_POOL (depot infrastructure)
- Depot identity, CBv2/rubric, secrets dir, DNS, machine names, vessel dir

## Spec document updates

- Create lenses/RBSRG-RegimeGcbPins.adoc
- Create lenses/RBSRM-RegimeMachine.adoc
- Update lenses/RBSRR-RegimeRepo.adoc (remove extracted groups)
- Update lenses/RBS0-SpecTop.adoc (add RBRG/RBRM regime definitions)

## References

- Tools/rbw/rbrr_cli.sh -- current pin refresh (rbrr_refresh_gcb_pins)
- .rbk/rbrr.env -- current storage for both groups
- Tools/rbw/rbrr_regime.sh -- regime validation
- Tools/rbw/rbf_Foundry.sh -- stitch function (pin consumer)
- Tools/rbw/rbv_PodmanVM.sh -- crane and CHOSEN_*/MANIFEST_PLATFORMS consumer
- Tools/buk/vov_veiled/BCG-BashConsoleGuide.md -- BCG patterns
- lenses/RBSRR-RegimeRepo.adoc -- regime spec (groups to update)
- lenses/RBS0-SpecTop.adoc -- top spec (regime definitions)

**[260307-1919] rough**

Extract two variable groups from rbrr.env into dedicated config regimes.

## Group 1: GCB Image Pins -> RBRG regime

Prefix: RBRG_ (Google Cloud Build items). File: .rbk/rbrg.env.
Spec doc: lenses/RBSRG-RegimeGcbPins.adoc.

GCB image pins (RBRR_GCB_*_IMAGE_REF, RBRR_GCB_PINS_REFRESHED_AT) live inside
rbrr.env alongside regime identity variables. The pin refresh command
(rbrr_refresh_gcb_pins) edits rbrr.env in-place via sed, which is
manifestly BCG incompliant.

Variables moving (with rename):
- RBRR_GCB_ORAS_IMAGE_REF -> RBRG_ORAS_IMAGE_REF
- RBRR_GCB_GCLOUD_IMAGE_REF -> RBRG_GCLOUD_IMAGE_REF
- RBRR_GCB_DOCKER_IMAGE_REF -> RBRG_DOCKER_IMAGE_REF
- RBRR_GCB_ALPINE_IMAGE_REF -> RBRG_ALPINE_IMAGE_REF
- RBRR_GCB_SYFT_IMAGE_REF -> RBRG_SYFT_IMAGE_REF
- RBRR_GCB_BINFMT_IMAGE_REF -> RBRG_BINFMT_IMAGE_REF
- RBRR_GCB_SKOPEO_IMAGE_REF -> RBRG_SKOPEO_IMAGE_REF
- RBRR_GCB_PINS_REFRESHED_AT -> RBRG_PINS_REFRESHED_AT

Work:
- Create .rbk/rbrg.env with all pin variables
- Create lenses/RBSRG-RegimeGcbPins.adoc regime spec
- Create RBRG regime kindle/validate/render in appropriate module
- Rewrite pin refresh to generate entire rbrg.env from scratch
  (discover -> resolve -> write complete file, BCG compliant)
- Source rbrg.env from rbrr regime or stitch function
- Remove pin variables from rbrr.env
- Update all consumers of renamed variables

## Group 2: Podman VM Supply Chain -> RBRM regime

Prefix: RBRM_ (Machine/VM supply chain). File: .rbk/rbrm.env.
Spec doc: lenses/RBSRM-RegimeMachine.adoc.

Podman VM supply chain variables share a collection moment (operator
chooses a Podman release) and Podman support is deferred.

Variables moving (with rename):
- RBRR_MANIFEST_PLATFORMS -> RBRM_MANIFEST_PLATFORMS
- RBRR_CHOSEN_PODMAN_VERSION -> RBRM_CHOSEN_PODMAN_VERSION
- RBRR_CHOSEN_VMIMAGE_ORIGIN -> RBRM_CHOSEN_VMIMAGE_ORIGIN
- RBRR_CHOSEN_IDENTITY -> RBRM_CHOSEN_IDENTITY
- RBRR_CRANE_TAR_GZ -> RBRM_CRANE_TAR_GZ

Work:
- Create .rbk/rbrm.env with all VM supply chain variables
- Create lenses/RBSRM-RegimeMachine.adoc regime spec
- Create RBRM regime kindle/validate/render in appropriate module
- Remove VM supply chain variables from rbrr.env
- Update all consumers of renamed variables
  (primarily rbv_PodmanVM.sh, rbrr_regime.sh)

## Variables explicitly staying in RBRR

- RBRR_GCB_MACHINE_TYPE, RBRR_GCB_TIMEOUT, RBRR_GCB_MIN_CONCURRENT_BUILDS
  (operator-chosen build parameters, not auto-refreshed pins)
- RBRR_GCB_WORKER_POOL (depot infrastructure)
- Depot identity, CBv2/rubric, secrets dir, DNS, machine names, vessel dir

## Spec document updates

- Create lenses/RBSRG-RegimeGcbPins.adoc
- Create lenses/RBSRM-RegimeMachine.adoc
- Update lenses/RBSRR-RegimeRepo.adoc (remove extracted groups)
- Update lenses/RBS0-SpecTop.adoc (add RBRG/RBRM regime definitions)

## References

- Tools/rbw/rbrr_cli.sh -- current pin refresh (rbrr_refresh_gcb_pins)
- .rbk/rbrr.env -- current storage for both groups
- Tools/rbw/rbrr_regime.sh -- regime validation
- Tools/rbw/rbf_Foundry.sh -- stitch function (pin consumer)
- Tools/rbw/rbv_PodmanVM.sh -- crane and CHOSEN_*/MANIFEST_PLATFORMS consumer
- Tools/buk/vov_veiled/BCG-BashConsoleGuide.md -- BCG patterns
- lenses/RBSRR-RegimeRepo.adoc -- regime spec (groups to update)
- lenses/RBS0-SpecTop.adoc -- top spec (regime definitions)

**[260307-1907] rough**

Extract two variable groups from rbrr.env into dedicated config regimes.

## Group 1: GCB Image Pins → new regime

GCB image pins (`RBRR_GCB_*_IMAGE_REF`, `RBRR_GCB_PINS_REFRESHED_AT`) live inside
`rbrr.env` alongside regime identity variables. The pin refresh command
(`rbrr_refresh_gcb_pins`) edits `rbrr.env` in-place via sed, which is
manifestly BCG incompliant — BCG requires config regimes to be fully
rewritable, not surgically edited.

- Create a dedicated GCB pins config regime file
- Rewrite pin refresh to generate the entire pins file from scratch
  (discover → resolve → write complete file) rather than grep/sed editing
- Source the pins file from the stitch function or rbrr.env,
  maintaining existing variable names for downstream consumers
- Remove pin variables from rbrr.env
- Decide crane disposition — crane is no longer used in the pipeline
  (OCI Layout Bridge removed). Still used in `rbv_PodmanVM.sh`.
  Either remove from pins or keep if PodmanVM still needs it.

## Group 2: Podman VM Supply Chain → RBRM regime

Podman VM supply chain variables share a collection moment (operator
chooses a Podman release) and Podman support is deferred, making this
a natural extraction candidate.

- `RBRR_CHOSEN_PODMAN_VERSION` → `RBRM_CHOSEN_PODMAN_VERSION`
- `RBRR_CHOSEN_VMIMAGE_ORIGIN` → `RBRM_CHOSEN_VMIMAGE_ORIGIN`
- `RBRR_CHOSEN_IDENTITY` → `RBRM_CHOSEN_IDENTITY`
- `RBRR_CRANE_TAR_GZ` → candidate for RBRM (local tool dependency)
- Create RBRM regime file, kindle, validate, render
- Update all consumers of renamed variables

## BCG Compliance Target (both regimes)

Pin/config refresh should follow the BCG "config regime rewrite" pattern:
- Discover all current values
- Write complete new file (not edit existing)
- Validate new file
- Replace atomically

## References

- `Tools/rbw/rbrr_cli.sh` — current pin refresh (`rbrr_refresh_gcb_pins`)
- `.rbk/rbrr.env` — current storage for both groups
- `Tools/rbw/rbrr_regime.sh` — regime validation
- `Tools/rbw/rbf_Foundry.sh` — stitch function (pin consumer)
- `Tools/rbw/rbv_PodmanVM.sh` — crane and CHOSEN_* consumer
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — BCG patterns
- `lenses/RBSRR-RegimeRepo.adoc` — regime spec (groups to update)
- `lenses/RBS0-SpecTop.adoc` — top spec (regime definitions)

**[260305-1333] rough**

Extract GCB tool pins from rbrr.env into a dedicated config regime.

## Problem

GCB image pins (`RBRR_GCB_*_IMAGE_REF`, `RBRR_CRANE_TAR_GZ`) live inside
`rbrr.env` alongside regime identity variables. The pin refresh command
(`rbrr_refresh_gcb_pins`) edits `rbrr.env` in-place via sed, which is
manifestly BCG incompliant — BCG requires config regimes to be fully
rewritable, not surgically edited.

The grep/sed patterns are already fragile (e.g., `readonly` prefix mismatch
on crane grep — fixed in ₢AlAAO). More pins will make this worse.

## Scope

1. **Create a dedicated GCB pins config regime** (e.g., `rbgp.env` or similar)
   that holds all `RBRR_GCB_*_IMAGE_REF` pins and `RBRR_CRANE_TAR_GZ`
2. **Rewrite pin refresh** to generate the entire pins file from scratch
   (discover → resolve → write complete file) rather than grep/sed editing
3. **Source the pins file from rbrr.env** or from the stitch function,
   maintaining the existing variable names for downstream consumers
4. **Remove pin variables from rbrr.env** — they move to the new file
5. **Decide crane disposition** — crane is no longer used in the pipeline
   (₢AlAAJ removed OCI Layout Bridge). Still used in `rbv_PodmanVM.sh`.
   Either remove from pins entirely or keep if PodmanVM still needs it.

## BCG Compliance Target

Pin refresh should follow the BCG "config regime rewrite" pattern:
- Discover all current values
- Write complete new file (not edit existing)
- Validate new file
- Replace atomically

## References

- `Tools/rbw/rbrr_cli.sh` — current pin refresh (`rbrr_refresh_gcb_pins`)
- `.rbk/rbrr.env` — current pin storage
- `Tools/rbw/rbrr_regime.sh` — regime validation
- `Tools/rbw/rbf_Foundry.sh` — stitch function (pin consumer)
- `Tools/rbw/rbv_PodmanVM.sh` — crane consumer
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — BCG patterns

### rationalize-build-step-naming (₢AkAAZ) [complete]

**[260307-2010] complete**

Delete single-arch build pipeline and rationalize remaining step script naming.

## Deletions

1. Delete single-arch-only step scripts: `rbgjb03-build-and-load.sh`, `rbgjb04-sbom-and-summary.sh`, `rbgjb05-assemble-metadata.sh`, `rbgjb06-build-and-push-metadata.sh`
2. Delete `trbim-macos` vessel directory (only consumer of single-arch pipeline)
3. Delete the single-arch branch in `zrbf_stitch_build_json()` — collapse to multi-platform pipeline only

## Renames

Drop the `m` infix from multi-platform scripts (no longer needed since there's only one pipeline):

| Current | New |
|---|---|
| `rbgjbm03-buildx-push-multi.sh` | `rbgjb03-buildx-push-multi.sh` |
| `rbgjbm04-per-platform-pullback.sh` | `rbgjb04-per-platform-pullback.sh` |
| `rbgjbm05-push-per-platform.sh` | `rbgjb05-push-per-platform.sh` |
| `rbgjbm06-syft-per-platform.sh` | `rbgjb06-syft-per-platform.sh` |
| `rbgjbm07-build-info-per-platform.sh` | `rbgjb07-build-info-per-platform.sh` |
| `rbgjbm08-buildx-push-about.sh` | `rbgjb08-buildx-push-about.sh` |
| `rbgjbm09-imagetools-create.sh` | `rbgjb09-imagetools-create.sh` |

## Stitch function update

Update `z_step_defs` array in `rbf_Foundry.sh` to use new filenames. Remove `z_is_multi_platform` branching — always use the (formerly multi-platform) pipeline.

## Files

- `Tools/rbw/rbgjb/` — delete 4 scripts, rename 7 scripts
- `Tools/rbw/rbf_Foundry.sh` — collapse stitch pipeline branching
- `rbev-vessels/trbim-macos/` — delete directory
- Lens docs referencing old filenames — update where current (skip obsolete refs in historical memos)

**[260307-2001] rough**

Delete single-arch build pipeline and rationalize remaining step script naming.

## Deletions

1. Delete single-arch-only step scripts: `rbgjb03-build-and-load.sh`, `rbgjb04-sbom-and-summary.sh`, `rbgjb05-assemble-metadata.sh`, `rbgjb06-build-and-push-metadata.sh`
2. Delete `trbim-macos` vessel directory (only consumer of single-arch pipeline)
3. Delete the single-arch branch in `zrbf_stitch_build_json()` — collapse to multi-platform pipeline only

## Renames

Drop the `m` infix from multi-platform scripts (no longer needed since there's only one pipeline):

| Current | New |
|---|---|
| `rbgjbm03-buildx-push-multi.sh` | `rbgjb03-buildx-push-multi.sh` |
| `rbgjbm04-per-platform-pullback.sh` | `rbgjb04-per-platform-pullback.sh` |
| `rbgjbm05-push-per-platform.sh` | `rbgjb05-push-per-platform.sh` |
| `rbgjbm06-syft-per-platform.sh` | `rbgjb06-syft-per-platform.sh` |
| `rbgjbm07-build-info-per-platform.sh` | `rbgjb07-build-info-per-platform.sh` |
| `rbgjbm08-buildx-push-about.sh` | `rbgjb08-buildx-push-about.sh` |
| `rbgjbm09-imagetools-create.sh` | `rbgjb09-imagetools-create.sh` |

## Stitch function update

Update `z_step_defs` array in `rbf_Foundry.sh` to use new filenames. Remove `z_is_multi_platform` branching — always use the (formerly multi-platform) pipeline.

## Files

- `Tools/rbw/rbgjb/` — delete 4 scripts, rename 7 scripts
- `Tools/rbw/rbf_Foundry.sh` — collapse stitch pipeline branching
- `rbev-vessels/trbim-macos/` — delete directory
- Lens docs referencing old filenames — update where current (skip obsolete refs in historical memos)

**[260305-1540] rough**

Assess and rationalize the naming and numbering of build step scripts in
Tools/rbw/rbgjb/ (or Tools/rbk/rbgjb/ post-₢AkAAK rename).

## Problem

The step script inventory has evolved through multiple paces across ₣Al:
- ₢AlAAJ renumbered the single-arch pipeline from 10 steps to 6 (rbgjb01-06)
- Multi-platform pipeline scripts (rbgjbm03-09) were added in parallel
- Steps 01-02 are shared between pipelines; steps 03+ diverge
- Overlapping step numbers (rbgjb03 vs rbgjbm03) mean different things
- The `m` infix is ad-hoc — not in the kit infrastructure suffix table

These inconsistencies accumulated through iterative patches, not bad design.
The scripts work correctly; the naming doesn't reflect the final structure.

## Approach

DO NOT prescribe the naming convention in advance. At execution time:

1. **Survey the step inventory** — enumerate all scripts, which pipeline
   uses each, what stitch function references, and what the step IDs are
2. **Identify the constraints** — mint discipline (terminal exclusivity,
   prefix rules), directory sort order, log readability, stitch function
   mechanics, any downstream references (docs, memos, paddock)
3. **Design the convention** — propose naming/numbering scheme that resolves
   overlaps and the `m` infix question. Present options to user.
4. **Execute the rename** — rename files, update stitch step_defs arrays,
   update any references

## Prerequisites

- ₢AlAAQ result known — determines whether multi-platform pipeline survives
- Multi-platform stitch path stable — no more step additions/deletions expected
- Ideally after ₢AkAAK (rbw→rbk directory rename) to avoid renaming twice

## Not in scope

- Changing step script behavior (pure rename/renumber)
- Changing stitch function logic (only step_defs array entries)

## Acceptance Criteria

- All step scripts follow a consistent naming convention
- No overlapping step numbers between pipelines
- Convention documented (paddock or inline comments in stitch function)
- Stitch function step_defs updated to match new filenames

### rbscip-linked-term-consideration (₢AkAAa) [complete]

**[260309-1001] complete**

Drafted from ₢AlAAD in ₣Al.

Consider replacing prose references to propagation functions in RBSCIP-IamPropagation.adoc with MCM linked terms.

Currently the document references `rbgu_poll_until_ok`, `rbgi_add_project_iam_role`, etc. as inline code literals. These could be linked terms with proper anchors and attribute references in RBS0, giving cross-reference integrity.

## Scope

- Review whether RBSCIP function references warrant linked terms or are fine as prose
- If warranted: define anchors in RBS0, create attribute references, update RBSCIP
- Consider whether the rbbc_* control flow terms (rbbc_poll, rbbc_call, etc.) should reference specific implementing functions

## Not in scope

- Changing any runtime behavior
- Refactoring propagation code

**[260305-1619] rough**

Drafted from ₢AlAAD in ₣Al.

Consider replacing prose references to propagation functions in RBSCIP-IamPropagation.adoc with MCM linked terms.

Currently the document references `rbgu_poll_until_ok`, `rbgi_add_project_iam_role`, etc. as inline code literals. These could be linked terms with proper anchors and attribute references in RBS0, giving cross-reference integrity.

## Scope

- Review whether RBSCIP function references warrant linked terms or are fine as prose
- If warranted: define anchors in RBS0, create attribute references, update RBSCIP
- Consider whether the rbbc_* control flow terms (rbbc_poll, rbbc_call, etc.) should reference specific implementing functions

## Not in scope

- Changing any runtime behavior
- Refactoring propagation code

**[260304-1157] rough**

Consider replacing prose references to propagation functions in RBSCIP-IamPropagation.adoc with MCM linked terms.

Currently the document references `rbgu_poll_until_ok`, `rbgi_add_project_iam_role`, etc. as inline code literals. These could be linked terms with proper anchors and attribute references in RBS0, giving cross-reference integrity.

## Scope

- Review whether RBSCIP function references warrant linked terms or are fine as prose
- If warranted: define anchors in RBS0, create attribute references, update RBSCIP
- Consider whether the rbbc_* control flow terms (rbbc_poll, rbbc_call, etc.) should reference specific implementing functions

## Not in scope

- Changing any runtime behavior
- Refactoring propagation code

### remove-spurious-poll-method-parameter (₢AkAAb) [complete]

**[260308-1442] complete**

Remove the spurious method parameter from rbgu_poll_until_ok. The parameter was added based on a misunderstanding that Secret Manager getIamPolicy uses POST (it uses GET). All 8 callers pass "GET" — the parameter carries zero information. Hardcode GET inside the function, drop the parameter, update all call sites, update RBSCIP-IamPropagation.adoc lens.

**[260308-1351] rough**

Remove the spurious method parameter from rbgu_poll_until_ok. The parameter was added based on a misunderstanding that Secret Manager getIamPolicy uses POST (it uses GET). All 8 callers pass "GET" — the parameter carries zero information. Hardcode GET inside the function, drop the parameter, update all call sites, update RBSCIP-IamPropagation.adoc lens.

**[260308-1351] rough**

Drafted from ₢AlAAF in ₣Al.

Reconsider the rbgu_poll_until_ok method parameter factoring.

## Context

The refactor in cf3406a6 changed rbgu_poll_get_until_ok (GET-only) to
rbgu_poll_until_ok with a method parameter to support POST for Secret Manager
getIamPolicy. This turned out to be wrong — Secret Manager getIamPolicy uses
GET, not POST (unlike CRM getIamPolicy which uses POST).

The method parameter made it easy to pass the wrong verb without any compile-time
or structural safety. All existing callers use GET. The parameterization was
premature — introduced for a use case that didn't exist.

## Questions to resolve

1. Should we revert to rbgu_poll_get_until_ok (GET-only)?
2. Or keep the parameterization but document which GCP APIs use GET vs POST for
   getIamPolicy? (CRM uses POST, Secret Manager uses GET, IAM uses POST)
3. Consider whether rbgu_http_json callers should have a wrapper per GCP API
   that encodes the correct method, rather than raw method strings at call sites.

## Not in scope

- Changing runtime behavior (the fix is already landed)
- Refactoring rbgu_http_json itself

**[260305-1619] rough**

Drafted from ₢AlAAF in ₣Al.

Reconsider the rbgu_poll_until_ok method parameter factoring.

## Context

The refactor in cf3406a6 changed rbgu_poll_get_until_ok (GET-only) to
rbgu_poll_until_ok with a method parameter to support POST for Secret Manager
getIamPolicy. This turned out to be wrong — Secret Manager getIamPolicy uses
GET, not POST (unlike CRM getIamPolicy which uses POST).

The method parameter made it easy to pass the wrong verb without any compile-time
or structural safety. All existing callers use GET. The parameterization was
premature — introduced for a use case that didn't exist.

## Questions to resolve

1. Should we revert to rbgu_poll_get_until_ok (GET-only)?
2. Or keep the parameterization but document which GCP APIs use GET vs POST for
   getIamPolicy? (CRM uses POST, Secret Manager uses GET, IAM uses POST)
3. Consider whether rbgu_http_json callers should have a wrapper per GCP API
   that encodes the correct method, rather than raw method strings at call sites.

## Not in scope

- Changing runtime behavior (the fix is already landed)
- Refactoring rbgu_http_json itself

**[260304-1538] rough**

Reconsider the rbgu_poll_until_ok method parameter factoring.

## Context

The refactor in cf3406a6 changed rbgu_poll_get_until_ok (GET-only) to
rbgu_poll_until_ok with a method parameter to support POST for Secret Manager
getIamPolicy. This turned out to be wrong — Secret Manager getIamPolicy uses
GET, not POST (unlike CRM getIamPolicy which uses POST).

The method parameter made it easy to pass the wrong verb without any compile-time
or structural safety. All existing callers use GET. The parameterization was
premature — introduced for a use case that didn't exist.

## Questions to resolve

1. Should we revert to rbgu_poll_get_until_ok (GET-only)?
2. Or keep the parameterization but document which GCP APIs use GET vs POST for
   getIamPolicy? (CRM uses POST, Secret Manager uses GET, IAM uses POST)
3. Consider whether rbgu_http_json callers should have a wrapper per GCP API
   that encodes the correct method, rather than raw method strings at call sites.

## Not in scope

- Changing runtime behavior (the fix is already landed)
- Refactoring rbgu_http_json itself

### retire-dispatch-tier-evidence-machinery (₢AkAAc) [complete]

**[260307-1945] complete**

Remove the dispatch tier (`bute_dispatch`, `bute_init_dispatch`,
`bute_init_evidence`, `bute_get_step_*`) from `bute_engine.sh`.

## Context

The dispatch tier was a stepping stone. Its only unique value — evidence
harvesting (copying BURD_OUTPUT_DIR/current/ into evidence dirs) — is
superseded by `ZBUTO_BURV_OUTPUT` (added in ₢AlAAY), which exposes the
BURV output root directly after `zbuto_invoke`.

Three consumers exist:
- `butcde_DispatchExercise.sh` — tests the dispatch machinery itself (remove)
- `butcrg_RegimeSmoke.sh` — convert to `buto_tt_expect_ok`
- `butcrg_RegimeCredentials.sh` — convert to `buto_tt_expect_ok`

The regime tests currently use 4 lines of dispatch boilerplate to check
what `buto_tt_expect_ok` does in one line.

## Scope

### Remove from bute_engine.sh
- `bute_init_dispatch()`, `zbute_dispatch_sentinel()`
- `zbute_resolve_tabtarget_capture()`
- `bute_init_evidence()`
- `bute_dispatch()`
- `bute_last_step_capture()`, `bute_get_step_exit_capture()`, `bute_get_step_output_capture()`
- All `zbute_step_*` array declarations
- Keep `zbute_tcase()` — that's the case boundary runner, not dispatch

### Convert butcrg_RegimeSmoke.sh
- Replace `zbutcrg_init` + `zbutcrg_dispatch_ok` with `buto_tt_expect_ok`
- Remove `bute_init_dispatch`/`bute_init_evidence` calls
- Pattern: `buto_tt_expect_ok "rbw-rnr" "${z_moniker}"`

### Convert butcrg_RegimeCredentials.sh
- Same conversion pattern

### Remove butcde_DispatchExercise.sh
- Delete file
- Remove `source` line from `rbtb_testbench.sh`
- Remove fixture enrollment from `rbtb_kindle()`

## Prerequisite
- ₢AlAAY (ZBUTO_BURV_OUTPUT must exist before dispatch tier removal)

## Acceptance Criteria
- No `bute_dispatch` calls anywhere in codebase
- `butcde_DispatchExercise.sh` deleted
- Regime smoke/credential tests pass using tabtarget tier
- `bute_engine.sh` contains only `zbute_tcase` and its helpers
- All tests pass (`rbw-ta`)

**[260306-0909] rough**

Remove the dispatch tier (`bute_dispatch`, `bute_init_dispatch`,
`bute_init_evidence`, `bute_get_step_*`) from `bute_engine.sh`.

## Context

The dispatch tier was a stepping stone. Its only unique value — evidence
harvesting (copying BURD_OUTPUT_DIR/current/ into evidence dirs) — is
superseded by `ZBUTO_BURV_OUTPUT` (added in ₢AlAAY), which exposes the
BURV output root directly after `zbuto_invoke`.

Three consumers exist:
- `butcde_DispatchExercise.sh` — tests the dispatch machinery itself (remove)
- `butcrg_RegimeSmoke.sh` — convert to `buto_tt_expect_ok`
- `butcrg_RegimeCredentials.sh` — convert to `buto_tt_expect_ok`

The regime tests currently use 4 lines of dispatch boilerplate to check
what `buto_tt_expect_ok` does in one line.

## Scope

### Remove from bute_engine.sh
- `bute_init_dispatch()`, `zbute_dispatch_sentinel()`
- `zbute_resolve_tabtarget_capture()`
- `bute_init_evidence()`
- `bute_dispatch()`
- `bute_last_step_capture()`, `bute_get_step_exit_capture()`, `bute_get_step_output_capture()`
- All `zbute_step_*` array declarations
- Keep `zbute_tcase()` — that's the case boundary runner, not dispatch

### Convert butcrg_RegimeSmoke.sh
- Replace `zbutcrg_init` + `zbutcrg_dispatch_ok` with `buto_tt_expect_ok`
- Remove `bute_init_dispatch`/`bute_init_evidence` calls
- Pattern: `buto_tt_expect_ok "rbw-rnr" "${z_moniker}"`

### Convert butcrg_RegimeCredentials.sh
- Same conversion pattern

### Remove butcde_DispatchExercise.sh
- Delete file
- Remove `source` line from `rbtb_testbench.sh`
- Remove fixture enrollment from `rbtb_kindle()`

## Prerequisite
- ₢AlAAY (ZBUTO_BURV_OUTPUT must exist before dispatch tier removal)

## Acceptance Criteria
- No `bute_dispatch` calls anywhere in codebase
- `butcde_DispatchExercise.sh` deleted
- Regime smoke/credential tests pass using tabtarget tier
- `bute_engine.sh` contains only `zbute_tcase` and its helpers
- All tests pass (`rbw-ta`)

### bus0-test-invocation-vocabulary (₢AkAAd) [complete]

**[260307-2010] complete**

Codify the test invocation tiers and inter-step communication pattern
in BUS0-BashUtilitiesSpec.adoc.

**STATUS: Needs final go/no-go decision before execution.** The vocabulary
was surveyed and a recommendation drafted (see below), but the scope and
naming have not been fully ratified. Review the recommendation and decide
whether to proceed, narrow, or defer.

## Context

BUS0 currently documents the registry model (fixture/case/suite) and dispatch
entry points, but says nothing about invocation tiers or BURV isolation.
Three invocation tiers exist in code with stable APIs and consistent usage
across all test files. The dispatch tier has been removed (₢AkAAc).

## Recommended Additions

### 1. Invocation Tiers (buto_operations.sh)

| Tier | API | Resolves via |
|------|-----|-------------|
| Unit | `buto_unit_expect_ok` | Raw function/command |
| Tabtarget | `buto_tt_expect_ok` | Colophon → tt/{colophon}.*.sh |
| Launch | `buto_launch_expect_ok` | Workbench direct (no tabtarget file) |

All share `zbuto_invoke` under the hood. Each has `_expect_ok` and
`_expect_fatal` variants.

### 2. Capture Globals (zbuto_invoke contract)

- `ZBUTO_STDOUT` — captured stdout
- `ZBUTO_STDERR` — captured stderr
- `ZBUTO_STATUS` — exit code
- `ZBUTO_BURV_OUTPUT` — BURV output root (empty if no BURV)

### 3. Case Boundary (zbute_tcase)

- Runs case function in isolation subshell
- Sets `BUT_TEMP_DIR` per-case
- Sets `BUTE_BURV_ROOT` per-case (enables BURV for all tiers)

### 4. Fact-File Pattern (inter-step communication)

Producer writes `${BURD_OUTPUT_DIR}/${CONSTANT}`, consumer reads
`${ZBUTO_BURV_OUTPUT}/current/${CONSTANT}`. Kindle constant is the contract.

### 5. Suite Semantics Clarification

Suites classify by dependency tier (what environment must exist), not by
test methodology. A suite is a membership tag.

## What NOT to document

- `but_test.sh` compatibility shim (migration detail)
- Specific fixture names (per-testbench, not BUK vocabulary)

## Prerequisite
- ₢AkAAc (dispatch tier removed — no point specifying dead code)

## Acceptance Criteria
- BUS0 documents invocation tiers with linked terms
- BUS0 documents capture globals as zbuto_invoke contract
- BUS0 documents case boundary runner
- BUS0 documents fact-file pattern
- No reference to retired dispatch tier

**[260306-0909] rough**

Codify the test invocation tiers and inter-step communication pattern
in BUS0-BashUtilitiesSpec.adoc.

**STATUS: Needs final go/no-go decision before execution.** The vocabulary
was surveyed and a recommendation drafted (see below), but the scope and
naming have not been fully ratified. Review the recommendation and decide
whether to proceed, narrow, or defer.

## Context

BUS0 currently documents the registry model (fixture/case/suite) and dispatch
entry points, but says nothing about invocation tiers or BURV isolation.
Three invocation tiers exist in code with stable APIs and consistent usage
across all test files. The dispatch tier has been removed (₢AkAAc).

## Recommended Additions

### 1. Invocation Tiers (buto_operations.sh)

| Tier | API | Resolves via |
|------|-----|-------------|
| Unit | `buto_unit_expect_ok` | Raw function/command |
| Tabtarget | `buto_tt_expect_ok` | Colophon → tt/{colophon}.*.sh |
| Launch | `buto_launch_expect_ok` | Workbench direct (no tabtarget file) |

All share `zbuto_invoke` under the hood. Each has `_expect_ok` and
`_expect_fatal` variants.

### 2. Capture Globals (zbuto_invoke contract)

- `ZBUTO_STDOUT` — captured stdout
- `ZBUTO_STDERR` — captured stderr
- `ZBUTO_STATUS` — exit code
- `ZBUTO_BURV_OUTPUT` — BURV output root (empty if no BURV)

### 3. Case Boundary (zbute_tcase)

- Runs case function in isolation subshell
- Sets `BUT_TEMP_DIR` per-case
- Sets `BUTE_BURV_ROOT` per-case (enables BURV for all tiers)

### 4. Fact-File Pattern (inter-step communication)

Producer writes `${BURD_OUTPUT_DIR}/${CONSTANT}`, consumer reads
`${ZBUTO_BURV_OUTPUT}/current/${CONSTANT}`. Kindle constant is the contract.

### 5. Suite Semantics Clarification

Suites classify by dependency tier (what environment must exist), not by
test methodology. A suite is a membership tag.

## What NOT to document

- `but_test.sh` compatibility shim (migration detail)
- Specific fixture names (per-testbench, not BUK vocabulary)

## Prerequisite
- ₢AkAAc (dispatch tier removed — no point specifying dead code)

## Acceptance Criteria
- BUS0 documents invocation tiers with linked terms
- BUS0 documents capture globals as zbuto_invoke contract
- BUS0 documents case boundary runner
- BUS0 documents fact-file pattern
- No reference to retired dispatch tier

### quota-preflight-concurrent-builds (₢AkAAi) [complete]

**[260307-2011] complete**

Add automated quota preflight check for concurrent_private_pool_build_cpus.

## Problem

RBRR_GCB_MIN_CONCURRENT_BUILDS=3 is declared and validated but never consumed. When depot quota is too low (e.g. 2 vCPU with e2-standard-2 = 1 concurrent build), builds silently queue instead of failing fast. Discovered during demo1025 vessel rebuilds where 3 builds serialized on a 2-vCPU quota.

## Design

Query Service Usage consumer quota API for `concurrent_private_pool_build_cpus` in the depot project/region. Extract vCPU count from RBRR_GCB_MACHINE_TYPE string. Compute max_concurrent = quota_value / vcpus. Compare against RBRR_GCB_MIN_CONCURRENT_BUILDS.

## Insertion Points

1. **Hard gate in rbf_build** — die before trigger dispatch if quota < required. Use buc_tabtarget to point user to rbw-gqb (QuotaBuild guide) in the error message.
2. **Soft advisory in rbf_list** — warn after listing if quota looks insufficient. Use buc_tabtarget to point user to rbw-gqb.

## Quota Guide Review

The existing rbgm_quota_build procedure (rbw-gqb) was written before private pools with SLSA VERIFIED. Review and update the guide to:
- Clarify that the relevant metric is `concurrent_private_pool_build_cpus` (not default pool)
- Show the vCPU math: quota_value / machine_vcpus = max concurrent builds
- Reference RBRR_GCB_MIN_CONCURRENT_BUILDS as the target
- Confirm the guide works for the current demo1025 depot setup

## Notes

- Metric is private-pool-specific (not default pool concurrent builds)
- Service Usage API infrastructure already in rbgd_DepotConstants.sh and rbgu_Utility.sh
- Director credentials already available in both rbf_build and rbf_list paths
- Consider extracting the check as a shared function in rbf_Foundry.sh
- Use buc_tabtarget to emit clickable tabtarget hints (same pattern as rbob_start missing-image hints)

**[260307-1334] rough**

Add automated quota preflight check for concurrent_private_pool_build_cpus.

## Problem

RBRR_GCB_MIN_CONCURRENT_BUILDS=3 is declared and validated but never consumed. When depot quota is too low (e.g. 2 vCPU with e2-standard-2 = 1 concurrent build), builds silently queue instead of failing fast. Discovered during demo1025 vessel rebuilds where 3 builds serialized on a 2-vCPU quota.

## Design

Query Service Usage consumer quota API for `concurrent_private_pool_build_cpus` in the depot project/region. Extract vCPU count from RBRR_GCB_MACHINE_TYPE string. Compute max_concurrent = quota_value / vcpus. Compare against RBRR_GCB_MIN_CONCURRENT_BUILDS.

## Insertion Points

1. **Hard gate in rbf_build** — die before trigger dispatch if quota < required. Use buc_tabtarget to point user to rbw-gqb (QuotaBuild guide) in the error message.
2. **Soft advisory in rbf_list** — warn after listing if quota looks insufficient. Use buc_tabtarget to point user to rbw-gqb.

## Quota Guide Review

The existing rbgm_quota_build procedure (rbw-gqb) was written before private pools with SLSA VERIFIED. Review and update the guide to:
- Clarify that the relevant metric is `concurrent_private_pool_build_cpus` (not default pool)
- Show the vCPU math: quota_value / machine_vcpus = max concurrent builds
- Reference RBRR_GCB_MIN_CONCURRENT_BUILDS as the target
- Confirm the guide works for the current demo1025 depot setup

## Notes

- Metric is private-pool-specific (not default pool concurrent builds)
- Service Usage API infrastructure already in rbgd_DepotConstants.sh and rbgu_Utility.sh
- Director credentials already available in both rbf_build and rbf_list paths
- Consider extracting the check as a shared function in rbf_Foundry.sh
- Use buc_tabtarget to emit clickable tabtarget hints (same pattern as rbob_start missing-image hints)

**[260307-1326] rough**

Add automated quota preflight check for concurrent_private_pool_build_cpus.

## Problem

RBRR_GCB_MIN_CONCURRENT_BUILDS=3 is declared and validated but never consumed. When depot quota is too low (e.g. 2 vCPU with e2-standard-2 = 1 concurrent build), builds silently queue instead of failing fast. Discovered during demo1025 vessel rebuilds where 3 builds serialized on a 2-vCPU quota.

## Design

Query Service Usage consumer quota API for `concurrent_private_pool_build_cpus` in the depot project/region. Extract vCPU count from RBRR_GCB_MACHINE_TYPE string. Compute max_concurrent = quota_value / vcpus. Compare against RBRR_GCB_MIN_CONCURRENT_BUILDS.

## Insertion Points

1. **Hard gate in rbf_build** — die before trigger dispatch if quota < required
2. **Soft advisory in rbf_list** — warn after listing if quota looks insufficient

## Notes

- Metric is private-pool-specific (not default pool concurrent builds)
- Service Usage API infrastructure already in rbgd_DepotConstants.sh and rbgu_Utility.sh
- Director credentials already available in both rbf_build and rbf_list paths
- Consider extracting the check as a shared function in rbf_Foundry.sh

### nameplate-env-file-location (₢AkAAk) [complete]

**[260310-1240] complete**

Investigate and fix the location of nameplate .env files (rbrn_nsproto.env, rbrn_srjcl.env, rbrn_pluml.env). These are currently in Tools/rbw/ alongside implementation code, but they're user-authored configuration (regime files) — not tooling. Determine the correct location, move them, and update all consumers.

## Questions to answer

- Where do other regime .env files live? (rbrr.env, rbrp.env, etc.)
- What references these files? (RBCC literal constants, furnish functions, testbench)
- What is the principled location for per-instance regime config?

## Scope

- Move the 3 nameplate .env files to the correct location
- Update all path references (likely RBCC/RBBC literal constants)
- Verify all nameplate tests still pass after the move

**[260307-1845] rough**

Investigate and fix the location of nameplate .env files (rbrn_nsproto.env, rbrn_srjcl.env, rbrn_pluml.env). These are currently in Tools/rbw/ alongside implementation code, but they're user-authored configuration (regime files) — not tooling. Determine the correct location, move them, and update all consumers.

## Questions to answer

- Where do other regime .env files live? (rbrr.env, rbrp.env, etc.)
- What references these files? (RBCC literal constants, furnish functions, testbench)
- What is the principled location for per-instance regime config?

## Scope

- Move the 3 nameplate .env files to the correct location
- Update all path references (likely RBCC/RBBC literal constants)
- Verify all nameplate tests still pass after the move

### bcg-ban-test-and-break-idiom (₢AkAAl) [complete]

**[260308-1425] complete**

The `test X && break` idiom used in while loops for comma-separated string parsing is technically safe under `set -e` (bash exempts `&&`/`||` lists from errexit), but relies on humans remembering that exemption rule. This is fragile.

## Deliverables

1. Add BCG guidance banning `test ... && break` (and similar `cmd && control-flow` patterns where the exemption matters). Recommend explicit `if`/`then`/`break`/`fi` instead.
2. Repair all instances in the codebase. Known sites in `rbf_Foundry.sh` (currently 4 occurrences at lines ~211, ~301, ~1592, ~2014). Search broadly for other occurrences.

## Rationale

Code correctness should not depend on remembering language-lawyer exemption rules. An explicit `if` block is one line longer but immediately obvious to any reader.

**[260307-2005] rough**

The `test X && break` idiom used in while loops for comma-separated string parsing is technically safe under `set -e` (bash exempts `&&`/`||` lists from errexit), but relies on humans remembering that exemption rule. This is fragile.

## Deliverables

1. Add BCG guidance banning `test ... && break` (and similar `cmd && control-flow` patterns where the exemption matters). Recommend explicit `if`/`then`/`break`/`fi` instead.
2. Repair all instances in the codebase. Known sites in `rbf_Foundry.sh` (currently 4 occurrences at lines ~211, ~301, ~1592, ~2014). Search broadly for other occurrences.

## Rationale

Code correctness should not depend on remembering language-lawyer exemption rules. An explicit `if` block is one line longer but immediately obvious to any reader.

### conjure-poll-retry-hardening (₢AkAAn) [complete]

**[260308-0928] complete**

## Problem

The build status poll loop in `zrbf_wait_build_completion` (rbf_Foundry.sh:475-504) dies on a single transient curl failure (`buc_die` at line 489). Observed in production: sentry build poll crashed at attempt 5 due to network blip during concurrent builds.

## Fix

Tolerate N consecutive curl failures (e.g., 3) before dying. On a single failure, log a warning and continue to the next 5s poll cycle. Also validate the HTTP response body — a curl exit 0 with a 401/403 JSON error should be handled gracefully.

## Files

- `Tools/rbw/rbf_Foundry.sh` — `zrbf_wait_build_completion` function

## Acceptance criteria

- Single transient curl failure does not abort the poll loop
- 3+ consecutive failures still abort (not a silent infinite loop)
- 401/403 responses logged as warnings, not silently ignored

**[260308-0841] rough**

## Problem

The build status poll loop in `zrbf_wait_build_completion` (rbf_Foundry.sh:475-504) dies on a single transient curl failure (`buc_die` at line 489). Observed in production: sentry build poll crashed at attempt 5 due to network blip during concurrent builds.

## Fix

Tolerate N consecutive curl failures (e.g., 3) before dying. On a single failure, log a warning and continue to the next 5s poll cycle. Also validate the HTTP response body — a curl exit 0 with a 401/403 JSON error should be handled gracefully.

## Files

- `Tools/rbw/rbf_Foundry.sh` — `zrbf_wait_build_completion` function

## Acceptance criteria

- Single transient curl failure does not abort the poll loop
- 3+ consecutive failures still abort (not a silent infinite loop)
- 401/403 responses logged as warnings, not silently ignored

### consider-inscribe-to-rubric-migration (₢AkAAq) [complete]

**[260310-1236] complete**

### Remove local cloudbuild.json copies; inscribe generates directly into rubric

The `cloudbuild.json` files in `rbev-vessels/*/` are generated artifacts committed locally, then copied to the rubric repo with placeholders filled. This two-copy model creates a generate→review→commit→re-run ceremony that doesn't carry weight: the real sources (step scripts, Dockerfiles, pins) are already reviewed and committed. The rubric repo commit history provides the audit trail.

**Change:** Inscribe clones rubric repo first, copies build context per vessel, then stitch generates JSON directly into the rubric clone. No local cloudbuild.json files exist in the recipe bottle repo.

## Code changes

1. **`zrbf_stitch_build_json()`** in `rbf_Foundry.sh`:
   - Add required output path parameter: `zrbf_stitch_build_json "${output_path}"`
   - Final `mv` writes to `z_output_path` instead of `ZRBF_STITCHED_BUILD_FILE`
   - Remove `ZRBF_STITCHED_BUILD_FILE` constant from sentinel

2. **`rbf_rubric_inscribe()`** in `rbf_Foundry.sh`:
   - Move rubric clone to before generation (Phase 2 becomes: clone first)
   - Merged generate+sync loop: for each vessel:
     - `rm -rf` rubric vessel dir (critical: purge stale content)
     - `cp -R` vessel dir to rubric clone dir (entire build context including rbrv.env — no exclusions)
     - `zrbf_stitch_build_json "${rubric_vessel_dir}/cloudbuild.json"` inside isolation subshell
   - Delete Phase 3 (verify all committed) entirely
   - Remove the `cp` from `ZRBF_STITCHED_BUILD_FILE` to vessel dir (gone)
   - Fill values in rubric clone copies, commit+push — unchanged
   - The `_RBGY_RUBRIC_COMMIT` amend dance stays (chicken-and-egg with commit hash)

3. **Delete committed JSON files**: `git rm rbev-vessels/*/cloudbuild.json`

4. **`.gitignore`**: add `rbev-vessels/*/cloudbuild.json` (defensive, in case stitch is run outside inscribe context)

## Spec repairs

5. **RBSRI-rubric_inscribe.adoc** — rewrite:
   - Delete Step 4 (Verify All Committed)
   - Merge Steps 3+5: clone rubric repo, then for each vessel: delete stale rubric vessel dir, copy entire vessel dir as build context, stitch JSON directly into rubric clone
   - Remove all "committed main-repo copy" and "placeholder in committed copy" framing

6. **RBSTB-trigger_build.adoc** — moderate edits:
   - Substitution table Provenance section (~lines 198–203): remove "Placeholder in committed copy" language; values are generated at inscribe time
   - Script Inlining section (~lines 216–222): remove two-phase "committed JSON in main repo" description

7. **RBS0-SpecTop.adoc** — targeted edits:
   - Inscribe summary (~line 938): remove "verifies committed in main repo" clause
   - `{rbtgr_build_json}` definition (~line 1567): "committed" → "generated"
   - `{rbtgr_rubric}` definition (~line 1555): remove "committed" framing

8. **RBSCB-CloudBuildRoadmap.adoc** — small edits:
   - "Each vessel has a committed `cloudbuild.json`" → generated at inscribe time directly into rubric
   - Simplify "refresh, commit, and re-inscribe" language

## Not in scope

- `_RBGY_RUBRIC_COMMIT` amend pattern (inherent chicken-and-egg)
- Dockerfiles stay in vessel dirs (hand-written source)
- `rbrv.env` stays in vessel dirs (regime config) — also copied to rubric, harmless
- `rbf_trigger_build()` unchanged
- Step scripts (`rbgjb/*.sh`) unchanged
- No dry-run capability

## Verification

- Inscribe runs single-invocation: clone → copy build context → stitch directly → fill → push
- No `cloudbuild.json` in `rbev-vessels/*/` tracked by git
- Rubric repo receives identical content as before (full vessel dir + generated JSON per vessel)
- `rbf_trigger_build` dispatches successfully against inscribed rubric
- No temp file for stitched JSON — stitch writes directly to final destination

**[260310-1155] rough**

### Remove local cloudbuild.json copies; inscribe generates directly into rubric

The `cloudbuild.json` files in `rbev-vessels/*/` are generated artifacts committed locally, then copied to the rubric repo with placeholders filled. This two-copy model creates a generate→review→commit→re-run ceremony that doesn't carry weight: the real sources (step scripts, Dockerfiles, pins) are already reviewed and committed. The rubric repo commit history provides the audit trail.

**Change:** Inscribe clones rubric repo first, copies build context per vessel, then stitch generates JSON directly into the rubric clone. No local cloudbuild.json files exist in the recipe bottle repo.

## Code changes

1. **`zrbf_stitch_build_json()`** in `rbf_Foundry.sh`:
   - Add required output path parameter: `zrbf_stitch_build_json "${output_path}"`
   - Final `mv` writes to `z_output_path` instead of `ZRBF_STITCHED_BUILD_FILE`
   - Remove `ZRBF_STITCHED_BUILD_FILE` constant from sentinel

2. **`rbf_rubric_inscribe()`** in `rbf_Foundry.sh`:
   - Move rubric clone to before generation (Phase 2 becomes: clone first)
   - Merged generate+sync loop: for each vessel:
     - `rm -rf` rubric vessel dir (critical: purge stale content)
     - `cp -R` vessel dir to rubric clone dir (entire build context including rbrv.env — no exclusions)
     - `zrbf_stitch_build_json "${rubric_vessel_dir}/cloudbuild.json"` inside isolation subshell
   - Delete Phase 3 (verify all committed) entirely
   - Remove the `cp` from `ZRBF_STITCHED_BUILD_FILE` to vessel dir (gone)
   - Fill values in rubric clone copies, commit+push — unchanged
   - The `_RBGY_RUBRIC_COMMIT` amend dance stays (chicken-and-egg with commit hash)

3. **Delete committed JSON files**: `git rm rbev-vessels/*/cloudbuild.json`

4. **`.gitignore`**: add `rbev-vessels/*/cloudbuild.json` (defensive, in case stitch is run outside inscribe context)

## Spec repairs

5. **RBSRI-rubric_inscribe.adoc** — rewrite:
   - Delete Step 4 (Verify All Committed)
   - Merge Steps 3+5: clone rubric repo, then for each vessel: delete stale rubric vessel dir, copy entire vessel dir as build context, stitch JSON directly into rubric clone
   - Remove all "committed main-repo copy" and "placeholder in committed copy" framing

6. **RBSTB-trigger_build.adoc** — moderate edits:
   - Substitution table Provenance section (~lines 198–203): remove "Placeholder in committed copy" language; values are generated at inscribe time
   - Script Inlining section (~lines 216–222): remove two-phase "committed JSON in main repo" description

7. **RBS0-SpecTop.adoc** — targeted edits:
   - Inscribe summary (~line 938): remove "verifies committed in main repo" clause
   - `{rbtgr_build_json}` definition (~line 1567): "committed" → "generated"
   - `{rbtgr_rubric}` definition (~line 1555): remove "committed" framing

8. **RBSCB-CloudBuildRoadmap.adoc** — small edits:
   - "Each vessel has a committed `cloudbuild.json`" → generated at inscribe time directly into rubric
   - Simplify "refresh, commit, and re-inscribe" language

## Not in scope

- `_RBGY_RUBRIC_COMMIT` amend pattern (inherent chicken-and-egg)
- Dockerfiles stay in vessel dirs (hand-written source)
- `rbrv.env` stays in vessel dirs (regime config) — also copied to rubric, harmless
- `rbf_trigger_build()` unchanged
- Step scripts (`rbgjb/*.sh`) unchanged
- No dry-run capability

## Verification

- Inscribe runs single-invocation: clone → copy build context → stitch directly → fill → push
- No `cloudbuild.json` in `rbev-vessels/*/` tracked by git
- Rubric repo receives identical content as before (full vessel dir + generated JSON per vessel)
- `rbf_trigger_build` dispatches successfully against inscribed rubric
- No temp file for stitched JSON — stitch writes directly to final destination

**[260309-1922] rough**

### Remove local cloudbuild.json copies; inscribe generates directly into rubric

The `cloudbuild.json` files in `rbev-vessels/*/` are generated artifacts committed locally, then copied to the rubric repo with placeholders filled. This two-copy model creates a generate→review→commit→re-run ceremony that doesn't carry weight: the real sources (step scripts, Dockerfiles, pins) are already reviewed and committed. The rubric repo commit history provides the audit trail.

**Change:** Inscribe generates JSON ephemerally and publishes directly to the rubric. The recipe bottle repo stops committing derived build output.

## Code changes

1. **`rbf_rubric_inscribe()`** in `rbf_Foundry.sh`:
   - Move rubric clone to before generation (clone first, then generate into it)
   - Merged generate+sync phase: for each vessel, copy Dockerfile to rubric clone dir (delete target dir first to avoid overlapping stale content), then generate JSON directly into rubric clone dir via `zrbf_stitch_build_json()`
   - Delete Phase 3 (verify all committed) entirely
   - Fill values in rubric clone copies, commit+push
   - The `_RBGY_RUBRIC_COMMIT` amend dance stays (chicken-and-egg with commit hash)

2. **Delete committed JSON files**: remove `rbev-vessels/*/cloudbuild.json` from git

3. **`.gitignore`**: add `rbev-vessels/*/cloudbuild.json` (defensive)

4. **`zrbf_stitch_build_json()`**: no change needed — still generates with `__INSCRIBE_*__` placeholders, caller writes output to rubric clone dir instead of vessel dir

## Spec repairs

5. **RBSRI-rubric_inscribe.adoc** — rewrite:
   - Delete Step 3 (Verify All Committed)
   - Merge Steps 2+4: clone rubric repo, then for each vessel: delete stale rubric vessel dir, copy Dockerfile, generate JSON directly into rubric clone
   - Remove all "committed main-repo copy" and "placeholder in committed copy" framing

6. **RBSTB-trigger_build.adoc** — moderate edits:
   - Substitution table (~lines 198–203): remove "Placeholder in committed copy" language; values are generated at inscribe time
   - Script Inlining section (~lines 216–222): remove two-phase flow description

7. **RBS0-SpecTop.adoc** — targeted edits:
   - Inscribe summary (~line 938): remove "verifies committed in main repo" clause
   - `{rbtgr_build_json}` definition (~line 1567): "committed" → "generated"
   - `{rbtgr_rubric}` definition (~line 1555): same

8. **RBSCB-CloudBuildRoadmap.adoc** — small edits:
   - "Each vessel has a committed `cloudbuild.json`" → generated at inscribe time into rubric
   - Simplify "refresh, commit, and re-inscribe" language

## Not in scope

- `_RBGY_RUBRIC_COMMIT` amend pattern (inherent chicken-and-egg)
- Dockerfiles stay in vessel dirs (hand-written source)
- `rbrv.env` stays in vessel dirs (regime config)
- `rbf_trigger_build()` unchanged
- Step scripts (`rbgjb/*.sh`) unchanged
- No dry-run capability

## Verification

- Inscribe runs single-invocation: gather inputs → generate → push
- No `cloudbuild.json` in `rbev-vessels/*/` tracked by git
- Rubric repo receives identical content as before (Dockerfile + filled JSON per vessel)
- `rbf_trigger_build` dispatches successfully against inscribed rubric

**[260308-1134] rough**

Evaluate moving all inscribe-related functionality out of the local repo and into the rubric.

## Context

Inscribe aspects are currently split across the local repo and the rubric. As the system grows more complex, this split creates moving-part overhead. This pace is a design conversation to assess:

1. **What inscribe functionality currently lives in the local repo** — enumerate files, tabtargets, and dependencies
2. **What would need to move to the rubric** — and what rubric changes that implies
3. **What breaks** — tabtarget references, CLAUDE.md mappings, test infrastructure, regime interactions
4. **Is it worth it** — simplification vs migration cost

## Not bridleable

This is a design conversation with too many interacting concerns to plan autonomously. Needs interactive exploration of the dependency graph.

### install-load-bearing-principle (₢AkAAv) [complete]

**[260309-1137] complete**

## Goal

Install the load-bearing complexity principle in BCG, RCG, and CLAUDE.md. Keep each installation simple and in the voice of its host document.

## The Principle

Every element in a system — every spec definition line, every function extraction, every pattern variant, every structural distinction — must carry weight. An element is load-bearing when its removal would create a gap between intent and behavior. When similar things differ, ask whether the difference is load-bearing: if yes, document why; if no, homogenize. Non-load-bearing elements increase cognitive cost without increasing correctness.

## Where and How

BCG (Tools/buk/vov_veiled/BCG-BashConsoleGuide.md): End of Core Philosophy section. BCG has the gap — it has aesthetic (boring over clever) and specific tools (Zeroes Theory) but lacks the general decision criterion. Use concrete bash/GCP examples in BCG voice.

RCG (Tools/vok/vov_veiled/RCG-RustCodingGuide.md): Light touch. RCG is deliberately minimal and Rust's type system enforces much of this mechanically. Note that existing disciplines (Interface Contamination, Constant Discipline, Constructor Discipline) are instantiations of this principle. Brief paragraph only.

CLAUDE.md: Project-wide cross-cutting reference near the Working Preferences or as a new Design Principles section. This is the interim home until a proper top-level design principles document is created.

## Important Context

We do not yet have a top-level design principles document spanning all project languages and artifacts. This principle is the first signal that such a document is needed. For now, install it where it fits and accept that the final home will be determined later. The goal is retention, not perfection of placement.

## Concrete Examples (from the session that surfaced this)

- NOT adding boilerplate three-part structure to api_enable spec definition: the structure would not be load-bearing there
- Keeping 4 separate IAM grant functions: their differences ARE load-bearing (different GCP API failure modes)
- Identifying 3x copy-pasted secret IAM code as NOT load-bearing variation: should be extracted
- Inline Secret Manager IAM staying separate from extracted rbgi_add_* functions: the difference IS load-bearing (different failure class)

## Files

- Tools/buk/vov_veiled/BCG-BashConsoleGuide.md
- Tools/vok/vov_veiled/RCG-RustCodingGuide.md
- CLAUDE.md

**[260309-1000] rough**

## Goal

Install the load-bearing complexity principle in BCG, RCG, and CLAUDE.md. Keep each installation simple and in the voice of its host document.

## The Principle

Every element in a system — every spec definition line, every function extraction, every pattern variant, every structural distinction — must carry weight. An element is load-bearing when its removal would create a gap between intent and behavior. When similar things differ, ask whether the difference is load-bearing: if yes, document why; if no, homogenize. Non-load-bearing elements increase cognitive cost without increasing correctness.

## Where and How

BCG (Tools/buk/vov_veiled/BCG-BashConsoleGuide.md): End of Core Philosophy section. BCG has the gap — it has aesthetic (boring over clever) and specific tools (Zeroes Theory) but lacks the general decision criterion. Use concrete bash/GCP examples in BCG voice.

RCG (Tools/vok/vov_veiled/RCG-RustCodingGuide.md): Light touch. RCG is deliberately minimal and Rust's type system enforces much of this mechanically. Note that existing disciplines (Interface Contamination, Constant Discipline, Constructor Discipline) are instantiations of this principle. Brief paragraph only.

CLAUDE.md: Project-wide cross-cutting reference near the Working Preferences or as a new Design Principles section. This is the interim home until a proper top-level design principles document is created.

## Important Context

We do not yet have a top-level design principles document spanning all project languages and artifacts. This principle is the first signal that such a document is needed. For now, install it where it fits and accept that the final home will be determined later. The goal is retention, not perfection of placement.

## Concrete Examples (from the session that surfaced this)

- NOT adding boilerplate three-part structure to api_enable spec definition: the structure would not be load-bearing there
- Keeping 4 separate IAM grant functions: their differences ARE load-bearing (different GCP API failure modes)
- Identifying 3x copy-pasted secret IAM code as NOT load-bearing variation: should be extracted
- Inline Secret Manager IAM staying separate from extracted rbgi_add_* functions: the difference IS load-bearing (different failure class)

## Files

- Tools/buk/vov_veiled/BCG-BashConsoleGuide.md
- Tools/vok/vov_veiled/RCG-RustCodingGuide.md
- CLAUDE.md

### notch-staged-deletion-support (₢AkAAw) [complete]

**[260309-1925] complete**

## Goal

Fix jjx_record (notch) to handle files that are already staged deletions (via git rm).

## Problem

When a file is git rm'd before jjx_record, the file list validation correctly detects it as a staged deletion, but the subsequent git add step fails because the file doesn't exist on disk. This forces the user to omit deleted files from the file list, which then appear as warnings.

## Fix

In jjrnc_notch.rs, track which files are staged deletions during the validation loop, then skip them in the git add step (they are already staged). Collect into a staged_deletions vec during validation, filter them out of the git add arg list.

## Test

1. Create a temp file, git add + git commit it
2. git rm the file
3. Run jjx_record with the deleted file in the file list
4. Verify commit succeeds and includes the deletion

## Files

- Tools/jjk/vov_veiled/src/jjrnc_notch.rs

**[260309-1812] rough**

## Goal

Fix jjx_record (notch) to handle files that are already staged deletions (via git rm).

## Problem

When a file is git rm'd before jjx_record, the file list validation correctly detects it as a staged deletion, but the subsequent git add step fails because the file doesn't exist on disk. This forces the user to omit deleted files from the file list, which then appear as warnings.

## Fix

In jjrnc_notch.rs, track which files are staged deletions during the validation loop, then skip them in the git add step (they are already staged). Collect into a staged_deletions vec during validation, filter them out of the git add arg list.

## Test

1. Create a temp file, git add + git commit it
2. git rm the file
3. Run jjx_record with the deleted file in the file list
4. Verify commit succeeds and includes the deletion

## Files

- Tools/jjk/vov_veiled/src/jjrnc_notch.rs

### buw-workbench-zipper-migration (₢AkAAy) [complete]

**[260310-1117] complete**

Migrate buw_workbench.sh from hardcoded case-statement routing to buz_zipper dispatch, matching the pattern already used by rbw_workbench.sh.

## Context

rbw_workbench.sh uses buz_zipper: enrolls colophons via rbz_zipper.sh kindle, then dispatches with a single buz_exec_lookup call. buw_workbench.sh predates this pattern and uses a manual case statement where colophon strings appear as duplicate literals.

## Deliverables

1. Create buwz_zipper.sh (or equivalent) with kindle function that enrolls all BUK workbench colophons (buw-tt-*, buw-rc*, buw-rs*, buw-re*, buw-qsc)
2. Refactor buw_workbench.sh to source buz_zipper.sh, kindle, and dispatch via buz_exec_lookup
3. Remove the hardcoded case statement and local z_*_cli variables
4. Verify all existing buw-* tabtargets still work
5. Run buz_healthcheck to validate colophon-tabtarget alignment

**[260309-2013] rough**

Migrate buw_workbench.sh from hardcoded case-statement routing to buz_zipper dispatch, matching the pattern already used by rbw_workbench.sh.

## Context

rbw_workbench.sh uses buz_zipper: enrolls colophons via rbz_zipper.sh kindle, then dispatches with a single buz_exec_lookup call. buw_workbench.sh predates this pattern and uses a manual case statement where colophon strings appear as duplicate literals.

## Deliverables

1. Create buwz_zipper.sh (or equivalent) with kindle function that enrolls all BUK workbench colophons (buw-tt-*, buw-rc*, buw-rs*, buw-re*, buw-qsc)
2. Refactor buw_workbench.sh to source buz_zipper.sh, kindle, and dispatch via buz_exec_lookup
3. Remove the hardcoded case statement and local z_*_cli variables
4. Verify all existing buw-* tabtargets still work
5. Run buz_healthcheck to validate colophon-tabtarget alignment

### shellcheck-finding-repair (₢AkAAz) [complete]

**[260310-1310] complete**

Fix the 154 genuine shellcheck findings surfaced by buw-qsc so the qualification target passes clean.

## Scope

21 unique shellcheck codes across Tools/. Breakdown by code:
- SC2086 (88): Unquoted variable — BCG mandates "${var}" everywhere
- SC2295 (11): Unquoted expansion in ${..%pattern}
- SC2046 (11): Word splitting from unquoted command substitution (crgv.validate.sh)
- SC2188 (9): Bare file truncation — new BCG rule requires : > or redesign
- SC2016 (5): Expressions in single quotes (review: may be intentional patterns)
- SC2015 (4): A && B || C is not if-then-else (ABANDONED-github/)
- SC1083 (4): Literal braces (ABANDONED-github/)
- SC2129 (3): Consolidate redirects
- SC2059 (3): Variable in printf format string
- SC3048 (2): SIG prefix on signal names
- SC2012 (2): ls vs find
- SC2004 (2): Unnecessary $ in arithmetic
- SC1110 (2), SC2148 (1), SC2035 (1), SC1088 (1), SC1036 (1): All from rbmP_laterSecurityAudit.sh (memo file with .sh extension — rename or exclude)
- SC1001 (1), SC3040 (1), SC2254 (1), SC2206 (1): One-offs

## Approach

Fix by domain: BUK core, RBW modules, legacy/abandoned, other kits. Run buw-qsc after each domain to confirm progress.

## Verification

tt/buw-qsc.QualifyShellCheck.sh exits 0 (zero findings).

**[260309-2025] rough**

Fix the 154 genuine shellcheck findings surfaced by buw-qsc so the qualification target passes clean.

## Scope

21 unique shellcheck codes across Tools/. Breakdown by code:
- SC2086 (88): Unquoted variable — BCG mandates "${var}" everywhere
- SC2295 (11): Unquoted expansion in ${..%pattern}
- SC2046 (11): Word splitting from unquoted command substitution (crgv.validate.sh)
- SC2188 (9): Bare file truncation — new BCG rule requires : > or redesign
- SC2016 (5): Expressions in single quotes (review: may be intentional patterns)
- SC2015 (4): A && B || C is not if-then-else (ABANDONED-github/)
- SC1083 (4): Literal braces (ABANDONED-github/)
- SC2129 (3): Consolidate redirects
- SC2059 (3): Variable in printf format string
- SC3048 (2): SIG prefix on signal names
- SC2012 (2): ls vs find
- SC2004 (2): Unnecessary $ in arithmetic
- SC1110 (2), SC2148 (1), SC2035 (1), SC1088 (1), SC1036 (1): All from rbmP_laterSecurityAudit.sh (memo file with .sh extension — rename or exclude)
- SC1001 (1), SC3040 (1), SC2254 (1), SC2206 (1): One-offs

## Approach

Fix by domain: BUK core, RBW modules, legacy/abandoned, other kits. Run buw-qsc after each domain to confirm progress.

## Verification

tt/buw-qsc.QualifyShellCheck.sh exits 0 (zero findings).

### shellcheck-prerelease-gate (₢AkAA0) [complete]

**[260310-1326] complete**

Wire shellcheck qualification into the prerelease process so it runs as part of rbw-qa (QualifyAll) or an equivalent release gate.

## Deliverables

1. Add buq_shellcheck call to rbq_qualify_all() so rbw-qa includes shellcheck
2. Verify buw-qsc passes clean (depends on ¢AkAAz completing first)
3. Document in RBS0 Release Compliance section (depends on that section existing)

## Sequencing

Depends on ¢AkAAz (shellcheck-finding-repair) — findings must be fixed before gating on them.

**[260309-2025] rough**

Wire shellcheck qualification into the prerelease process so it runs as part of rbw-qa (QualifyAll) or an equivalent release gate.

## Deliverables

1. Add buq_shellcheck call to rbq_qualify_all() so rbw-qa includes shellcheck
2. Verify buw-qsc passes clean (depends on ¢AkAAz completing first)
3. Document in RBS0 Release Compliance section (depends on that section existing)

## Sequencing

Depends on ¢AkAAz (shellcheck-finding-repair) — findings must be fixed before gating on them.

### consolidate-ark-lifecycle-vouch-and-run (₢AkAA2) [complete]

**[260310-1410] complete**

Consolidate ark-lifecycle and slsa-provenance test fixtures into a single ark-lifecycle fixture that exercises the full supply chain.

## Test Sequence (9 steps)

1. Conjure -- build ark from rbev-busybox vessel
2. Check present -- RBZ_CHECK_CONSECRATIONS, verify -about and -image tags exist
3. Vouch -- call rbf_vouch() (single vessel, single consecration) to verify SLSA provenance and publish -vouch artifact. May take minutes (Cloud Build).
4. Check health classification -- RBZ_CHECK_CONSECRATIONS again, verify health shows vouched
5. Verify consumer vouch gate -- call extracted rbf_vouch_gate() to confirm consumer-side enforcement passes
6. Retrieve -- pull image via RBZ_RETRIEVE_IMAGE
7. Run and verify -- docker run --rm the image, assert output contains expected busybox CMD string (hardcoded docker, not regime runtime)
8. Cleanup image -- docker rmi the pulled image
9. Abjure -- delete ark with --force
10. Check absent -- verify consecration gone

## Scope

### 1. Extract vouch gate from rbob to foundry
Move the vouch-checking logic (HEAD request for -vouch tag, lines 329-345 of rbob_bottle.sh) into a new public rbf_vouch_gate() function in rbf_Foundry.sh. Takes vessel and consecration, verifies -vouch tag exists via registry API HEAD request. Dependencies are all foundry/regime-level (RBGD_GAR_*, RBDC_*_RBRA_FILE, RBCC_CURL_*, rbgo_get_token_capture) -- no rbob state needed.

Refactor zrbob_vouch_gate_and_summon() to call rbf_vouch_gate() internally before pulling -- one vouch-check implementation, two callers.

### 2. Expand ark baste for foundry access
The current zrbtb_ark_baste() only does zrbgc_kindle. The new test steps (vouch, vouch gate, retrieve) need foundry infrastructure. Expand baste to kindle: zrbgc_kindle, zrbgd_kindle, zrbdc_kindle, zrbf_kindle. This requires RBRR regime and derived constants to be sourced and kindled. Verify the testbench sourcing hierarchy (rbtb_testbench.sh lines 36-49) already sources these modules -- the baste just needs to kindle them.

### 3. Rewrite ark-lifecycle test case
Replace rbtcal_ArkLifecycle.sh with the 10-step sequence above. Uses rbf_vouch() not rbf_batch_vouch() -- single vessel, single consecration, no cross-vessel noise. Runtime for step 7 hardcoded to docker (litmus predicate already checks docker version). Step 8 cleans up the pulled image from local store. Not parameterizable by vessel -- hardcoded to rbev-busybox as proof-of-life for simple build.

### 4. Remove slsa-provenance fixture
Delete rbtcsl_SlsaProvenance.sh. Remove its fixture enrollment and case enrollment from rbtb_testbench.sh. Remove source line from rbtb_testbench.sh. Delete tt/rbw-tf.TestFixture.slsa-provenance.sh.

### 5. Update testbench enrollment
Update rbtb_testbench.sh: remove slsa-provenance fixture/case enrollment, keep ark-lifecycle enrollment pointing to the rewritten test case.

## Files
- Tools/rbw/rbf_Foundry.sh -- add rbf_vouch_gate()
- Tools/rbw/rbob_bottle.sh -- refactor zrbob_vouch_gate_and_summon() to call rbf_vouch_gate()
- Tools/rbw/rbtb_testbench.sh -- expand baste, remove slsa-provenance enrollment and source line
- Tools/rbw/rbts/rbtcal_ArkLifecycle.sh -- rewrite with 10-step sequence
- Tools/rbw/rbts/rbtcsl_SlsaProvenance.sh -- delete
- tt/rbw-tf.TestFixture.slsa-provenance.sh -- delete

## Acceptance
- rbf_vouch_gate() exists as public foundry function
- zrbob_vouch_gate_and_summon() calls rbf_vouch_gate() (no behavioral change to bottle start)
- ark-lifecycle fixture exercises all 10 steps including vouch, vouch gate, run, and image cleanup
- slsa-provenance fixture, test case file, source line, and tabtarget are gone
- No new vessel or nameplate required

**[260310-1344] rough**

Consolidate ark-lifecycle and slsa-provenance test fixtures into a single ark-lifecycle fixture that exercises the full supply chain.

## Test Sequence (9 steps)

1. Conjure -- build ark from rbev-busybox vessel
2. Check present -- RBZ_CHECK_CONSECRATIONS, verify -about and -image tags exist
3. Vouch -- call rbf_vouch() (single vessel, single consecration) to verify SLSA provenance and publish -vouch artifact. May take minutes (Cloud Build).
4. Check health classification -- RBZ_CHECK_CONSECRATIONS again, verify health shows vouched
5. Verify consumer vouch gate -- call extracted rbf_vouch_gate() to confirm consumer-side enforcement passes
6. Retrieve -- pull image via RBZ_RETRIEVE_IMAGE
7. Run and verify -- docker run --rm the image, assert output contains expected busybox CMD string (hardcoded docker, not regime runtime)
8. Cleanup image -- docker rmi the pulled image
9. Abjure -- delete ark with --force
10. Check absent -- verify consecration gone

## Scope

### 1. Extract vouch gate from rbob to foundry
Move the vouch-checking logic (HEAD request for -vouch tag, lines 329-345 of rbob_bottle.sh) into a new public rbf_vouch_gate() function in rbf_Foundry.sh. Takes vessel and consecration, verifies -vouch tag exists via registry API HEAD request. Dependencies are all foundry/regime-level (RBGD_GAR_*, RBDC_*_RBRA_FILE, RBCC_CURL_*, rbgo_get_token_capture) -- no rbob state needed.

Refactor zrbob_vouch_gate_and_summon() to call rbf_vouch_gate() internally before pulling -- one vouch-check implementation, two callers.

### 2. Expand ark baste for foundry access
The current zrbtb_ark_baste() only does zrbgc_kindle. The new test steps (vouch, vouch gate, retrieve) need foundry infrastructure. Expand baste to kindle: zrbgc_kindle, zrbgd_kindle, zrbdc_kindle, zrbf_kindle. This requires RBRR regime and derived constants to be sourced and kindled. Verify the testbench sourcing hierarchy (rbtb_testbench.sh lines 36-49) already sources these modules -- the baste just needs to kindle them.

### 3. Rewrite ark-lifecycle test case
Replace rbtcal_ArkLifecycle.sh with the 10-step sequence above. Uses rbf_vouch() not rbf_batch_vouch() -- single vessel, single consecration, no cross-vessel noise. Runtime for step 7 hardcoded to docker (litmus predicate already checks docker version). Step 8 cleans up the pulled image from local store. Not parameterizable by vessel -- hardcoded to rbev-busybox as proof-of-life for simple build.

### 4. Remove slsa-provenance fixture
Delete rbtcsl_SlsaProvenance.sh. Remove its fixture enrollment and case enrollment from rbtb_testbench.sh. Remove source line from rbtb_testbench.sh. Delete tt/rbw-tf.TestFixture.slsa-provenance.sh.

### 5. Update testbench enrollment
Update rbtb_testbench.sh: remove slsa-provenance fixture/case enrollment, keep ark-lifecycle enrollment pointing to the rewritten test case.

## Files
- Tools/rbw/rbf_Foundry.sh -- add rbf_vouch_gate()
- Tools/rbw/rbob_bottle.sh -- refactor zrbob_vouch_gate_and_summon() to call rbf_vouch_gate()
- Tools/rbw/rbtb_testbench.sh -- expand baste, remove slsa-provenance enrollment and source line
- Tools/rbw/rbts/rbtcal_ArkLifecycle.sh -- rewrite with 10-step sequence
- Tools/rbw/rbts/rbtcsl_SlsaProvenance.sh -- delete
- tt/rbw-tf.TestFixture.slsa-provenance.sh -- delete

## Acceptance
- rbf_vouch_gate() exists as public foundry function
- zrbob_vouch_gate_and_summon() calls rbf_vouch_gate() (no behavioral change to bottle start)
- ark-lifecycle fixture exercises all 10 steps including vouch, vouch gate, run, and image cleanup
- slsa-provenance fixture, test case file, source line, and tabtarget are gone
- No new vessel or nameplate required

**[260310-1337] rough**

Consolidate ark-lifecycle and slsa-provenance test fixtures into a single 8-step ark-lifecycle fixture that exercises the full supply chain: conjure, check present, vouch (batch), check vouched, retrieve, run & verify, abjure, check absent.

## Scope

### 1. Extract vouch gate from rbob to foundry
Move the vouch-checking logic (HEAD request for -vouch tag) from zrbob_vouch_gate_and_summon() in rbob_bottle.sh into a new public rbf_vouch_gate() function in rbf_Foundry.sh. The function takes vessel, consecration, and verifies the -vouch tag exists via registry API. rbob's zrbob_vouch_gate_and_summon() then calls rbf_vouch_gate() internally before pulling — one implementation, two callers.

Dependencies are all foundry/regime-level (RBGD_GAR_*, RBDC_*_RBRA_FILE, RBCC_CURL_*, rbgo_get_token_capture) — no rbob state needed.

### 2. Rewrite ark-lifecycle test case
Replace rbtcal_ArkLifecycle.sh with an 8-step test:
1. Conjure ark from rbev-busybox vessel
2. Check consecrations — verify -about and -image tags present
3. Vouch — call RBZ_VOUCH_ARK (batch vouch, may take minutes)
4. Check vouched — verify -vouch tag exists, health shows vouched
5. Vouch gate — call extracted rbf_vouch_gate() to verify gate passes
6. Retrieve — pull image via RBZ_RETRIEVE_IMAGE
7. Run and verify — RBRN_RUNTIME run --rm the image, assert output contains expected busybox CMD string
8. Abjure — delete ark with --force
9. Check absent — verify consecration gone

Runtime for step 7 uses regime-chosen RBRN_RUNTIME. Not parameterizable by vessel — hardcoded to rbev-busybox as proof-of-life for simple build.

### 3. Remove slsa-provenance fixture
Delete rbtcsl_SlsaProvenance.sh. Remove its fixture enrollment and case enrollment from rbtb_testbench.sh. Remove the slsa-provenance tabtarget from tt/.

### 4. Update testbench enrollment
Update rbtb_testbench.sh: remove slsa-provenance fixture/case enrollment, keep ark-lifecycle enrollment pointing to the rewritten test case.

## Files
- Tools/rbw/rbf_Foundry.sh — add rbf_vouch_gate()
- Tools/rbw/rbob_bottle.sh — refactor zrbob_vouch_gate_and_summon() to call rbf_vouch_gate()
- Tools/rbw/rbts/rbtcal_ArkLifecycle.sh — rewrite with 8-step sequence
- Tools/rbw/rbts/rbtcsl_SlsaProvenance.sh — delete
- Tools/rbw/rbtb_testbench.sh — remove slsa-provenance enrollment
- tt/rbw-tf.TestFixture.slsa-provenance.sh — delete

## Acceptance
- rbf_vouch_gate() exists as public foundry function
- rbob_bottle.sh vouch gate calls rbf_vouch_gate() (no behavioral change)
- ark-lifecycle fixture exercises all 8 steps including vouch and run
- slsa-provenance fixture and tabtarget are gone
- No new vessel or nameplate required

### rename-rbw-to-rbk-move-lenses (₢AkAAK) [complete]

**[260310-1434] complete**

Rename the `Tools/rbw/` directory to `Tools/rbk/` and move all lens files from `lenses/` to their correct veiled directories within the kit structure.

## Deliverables

1. Rename `Tools/rbw/` → `Tools/rbk/`
2. Move lens .adoc files from `lenses/` to their appropriate `vov_veiled/` directories
3. Update all references (CLAUDE.md, imports, tabtargets, etc.)
4. Rename `z_rbw_kit_dir` → `z_rbk_kit_dir` across all cli files (14 files use this local variable pattern with `"${BURD_TOOLS_DIR}/rbw"` — both the variable name and path string need updating)

## RBS* Files to Move (37 total)
- `RBS-Specification.adoc` (main spec)
- `RBS0-SpecTop.adoc`
- 35 subdocuments: RBSAA, RBSAX, RBSBC, RBSBK, RBSBL, RBSBR, RBSBS, RBSCE, RBSCO, RBSDC, RBSDD, RBSDI, RBSDL, RBSDS, RBSGR, RBSGS, RBSID, RBSIL, RBSIP, RBSIR, RBSNC, RBSNX, RBSOB, RBSPE, RBSPI, RBSPR, RBSPT, RBSRC, RBSRV, RBSSC, RBSSD, RBSSL, RBSSS, RBSTB, RBSVC, RBSVM

## Cross-reference Updates
- Grep remaining `lenses/` files for references to moved RBS* files and update paths
- Grep all shell scripts, tabtargets, and CLAUDE.md for old paths

## Verification
- All 37 RBS* files exist under `Tools/rbk/vov_veiled/`
- No RBS* files remain in `lenses/`
- `lenses/` retains only non-RBS files (CRR, RBRN, RBRR, RBWMBX, axl-AXMCM)
- CLAUDE.md mappings point to new paths
- No broken cross-references

**[260308-1229] rough**

Rename the `Tools/rbw/` directory to `Tools/rbk/` and move all lens files from `lenses/` to their correct veiled directories within the kit structure.

## Deliverables

1. Rename `Tools/rbw/` → `Tools/rbk/`
2. Move lens .adoc files from `lenses/` to their appropriate `vov_veiled/` directories
3. Update all references (CLAUDE.md, imports, tabtargets, etc.)
4. Rename `z_rbw_kit_dir` → `z_rbk_kit_dir` across all cli files (14 files use this local variable pattern with `"${BURD_TOOLS_DIR}/rbw"` — both the variable name and path string need updating)

## RBS* Files to Move (37 total)
- `RBS-Specification.adoc` (main spec)
- `RBS0-SpecTop.adoc`
- 35 subdocuments: RBSAA, RBSAX, RBSBC, RBSBK, RBSBL, RBSBR, RBSBS, RBSCE, RBSCO, RBSDC, RBSDD, RBSDI, RBSDL, RBSDS, RBSGR, RBSGS, RBSID, RBSIL, RBSIP, RBSIR, RBSNC, RBSNX, RBSOB, RBSPE, RBSPI, RBSPR, RBSPT, RBSRC, RBSRV, RBSSC, RBSSD, RBSSL, RBSSS, RBSTB, RBSVC, RBSVM

## Cross-reference Updates
- Grep remaining `lenses/` files for references to moved RBS* files and update paths
- Grep all shell scripts, tabtargets, and CLAUDE.md for old paths

## Verification
- All 37 RBS* files exist under `Tools/rbk/vov_veiled/`
- No RBS* files remain in `lenses/`
- `lenses/` retains only non-RBS files (CRR, RBRN, RBRR, RBWMBX, axl-AXMCM)
- CLAUDE.md mappings point to new paths
- No broken cross-references

**[260302-0713] rough**

Drafted from ₢AiAAh in ₣Ai.

Drafted from ₢AkAAB in ₣Ak.

Rename the `Tools/rbw/` directory to `Tools/rbk/` and move all lens files from `lenses/` to their correct veiled directories within the kit structure.

## Deliverables

1. Rename `Tools/rbw/` → `Tools/rbk/`
2. Move lens .adoc files from `lenses/` to their appropriate `vov_veiled/` directories
3. Update all references (CLAUDE.md, imports, tabtargets, etc.)
4. Rename `z_rbw_kit_dir` → `z_rbk_kit_dir` across all cli files (14 files use this local variable pattern with `"${BURD_TOOLS_DIR}/rbw"` — both the variable name and path string need updating)

**[260225-1922] rough**

Drafted from ₢AkAAB in ₣Ak.

Rename the `Tools/rbw/` directory to `Tools/rbk/` and move all lens files from `lenses/` to their correct veiled directories within the kit structure.

## Deliverables

1. Rename `Tools/rbw/` → `Tools/rbk/`
2. Move lens .adoc files from `lenses/` to their appropriate `vov_veiled/` directories
3. Update all references (CLAUDE.md, imports, tabtargets, etc.)
4. Rename `z_rbw_kit_dir` → `z_rbk_kit_dir` across all cli files (14 files use this local variable pattern with `"${BURD_TOOLS_DIR}/rbw"` — both the variable name and path string need updating)

**[260224-1458] rough**

Rename the `Tools/rbw/` directory to `Tools/rbk/` and move all lens files from `lenses/` to their correct veiled directories within the kit structure.

## Deliverables

1. Rename `Tools/rbw/` → `Tools/rbk/`
2. Move lens .adoc files from `lenses/` to their appropriate `vov_veiled/` directories
3. Update all references (CLAUDE.md, imports, tabtargets, etc.)
4. Rename `z_rbw_kit_dir` → `z_rbk_kit_dir` across all cli files (14 files use this local variable pattern with `"${BURD_TOOLS_DIR}/rbw"` — both the variable name and path string need updating)

**[260224-0821] rough**

Rename the `Tools/rbw/` directory to `Tools/rbk/` and move all lens files from `lenses/` to their correct veiled directories within the kit structure.

## Deliverables

1. Rename `Tools/rbw/` → `Tools/rbk/`
2. Move lens .adoc files from `lenses/` to their appropriate `vov_veiled/` directories
3. Update all references (CLAUDE.md, imports, tabtargets, etc.)

### baseline-test-sweep (₢AkAAx) [complete]

**[260310-1446] complete**

Full test sweep to establish green baseline before any mvp-2 work begins. Run all test suites, record results. This is the exit gate for mvp-1 and the entry proof for mvp-2.

**[260309-1945] rough**

Full test sweep to establish green baseline before any mvp-2 work begins. Run all test suites, record results. This is the exit gate for mvp-1 and the entry proof for mvp-2.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 1 harden-test-sweep-reliability
  2 t delete-retriever-list-images
  3 m fix-images-field-tag-base-mismatch
  4 j fix-consumer-image-tag-uses-full-consecration
  5 h extract-rbdc-derived-constants
  6 g rebuild-vessels-fix-nameplate-tests
  7 f regime-lock-in-kindle-antipattern
  8 r vouch-implementation-from-spec
  9 s director-batch-vouch-and-health-display
  10 p gcb-pin-slsa-verifier
  11 o vouch-slsa-verifier-architecture
  12 X bottle-start-auto-summon-missing
  13 W rename-buc-next-and-fix-payor-refresh
  14 D readonly-bcg-trial
  15 E readonly-widespread
  16 G bootstrap-constants-secrets-consolidation
  17 H secrets-dir-credential-consolidation
  18 C add-test-sweep-suites
  19 F kindle-mutable-state-naming
  20 A suite-infrastructure-preconditions
  21 Q shellcheck-bcg-exemption-catalogue
  22 Y extract-rbrg-rbrm-from-rbrr
  23 Z rationalize-build-step-naming
  24 a rbscip-linked-term-consideration
  25 b remove-spurious-poll-method-parameter
  26 c retire-dispatch-tier-evidence-machinery
  27 d bus0-test-invocation-vocabulary
  28 i quota-preflight-concurrent-builds
  29 k nameplate-env-file-location
  30 l bcg-ban-test-and-break-idiom
  31 n conjure-poll-retry-hardening
  32 q consider-inscribe-to-rubric-migration
  33 v install-load-bearing-principle
  34 w notch-staged-deletion-support
  35 y buw-workbench-zipper-migration
  36 z shellcheck-finding-repair
  37 0 shellcheck-prerelease-gate
  38 2 consolidate-ark-lifecycle-vouch-and-run
  39 K rename-rbw-to-rbk-move-lenses
  40 x baseline-test-sweep

1tmjhgfrspoXWDEGHCFAQYZabcdiklnqvwyz02Kx
xxxxx··xxxxx······x··xx······xxx···x·xx· rbf_Foundry.sh
xx··x·x······x·x·x····x··x··x········xx· rbtb_testbench.sh
·xx·····xxx·····x·x··x·x·······x······x· RBS0-SpecTop.adoc
········x····x·x·xxxx········x··x··x···· BCG-BashConsoleGuide.md
xx······xxx······xx·x···············x·x· rbz_zipper.sh
····x·x·········x·x·····x····x·····x··x· rbgp_Payor.sh
····x·x····x·x·x············x········xx· rbob_cli.sh
····x·············x·x···x····x·····x··x· rbgu_Utility.sh
····x·x······x·x············x······x··x· rbrn_cli.sh
···············xx·x········x·······x··x· rbgm_ManualProcedures.sh
·····xx···········x·········x······x··x· rbrn_regime.sh
····x·x·········x·x··x················x· rbrr_regime.sh
····x·x······x·x·····x················x· rbf_cli.sh
····x·x······x·xx········x·············· butcrg_RegimeCredentials.sh
····x·x··x·····x·····x················x· rbrr_cli.sh
·············x···xx··········x·····x···· buv_validation.sh
···········x······x················x·xx· rbob_bottle.sh
·········x········x··x·········x······x· RBSRI-rubric_inscribe.adoc
····x·x···········x·····x·············x· rbgg_Governor.sh
····x·x·········x·x···················x· rbro_regime.sh
····x·x······x·······x················x· rbv_cli.sh
····x·x······x·x······················x· rbgg_cli.sh, rbgm_cli.sh, rbgp_cli.sh, rbra_cli.sh, rbrv_cli.sh, rbtcap_AccessProbe.sh
····x·x······x·x·········x·············· butcrg_RegimeSmoke.sh
··x·················x·x············x··x· rbgjb09-imagetools-create.sh
··x·······x···········x········x······x· RBSCB-CloudBuildRoadmap.adoc
x··············xx·x···················x· rbcc_Constants.sh
xx········x·······x···················x· rbgc_Constants.sh
··················x··x·············x··x· rbv_PodmanVM.sh
················x·x··x················x· RBSRR-RegimeRepo.adoc
·········x···········xx········x········ rbrg.env
······x········xx····x·················· rbrr.env
····x········x····x···················x· rbi_Image.sh
····x···x·······················x·····x· CLAUDE.md
····x·x·········x·····················x· rbro_cli.sh
··x···················x············x··x· rbgjb07-build-info-per-platform.sh
··x···················x········x······x· rbgjb06-syft-per-platform.sh
xx···································xx· rbtcal_ArkLifecycle.sh
·····························x·····x··x· rbupmis_Scrub.sh
·····················x·············x··x· rbgjb02-qemu-binfmt.sh
··················x·················x·x· rbq_Qualify.sh
··················x················x··x· rbgd_DepotConstants.sh, rboo_observe.sh
··················x·x·················x· rbgo_OAuth.sh
·················xxx···················· butr_registry.sh
···············x·············x·····x···· bud_dispatch.sh
···············x·····x················x· rbbc_constants.sh
·········x···········x················x· RBSRG-RegimeGcbPins.adoc, rbrg_regime.sh
·······x··x···························x· RBSAV-ark_vouch.adoc
······x···········x···················x· rbra_regime.sh, rbrp_regime.sh, rbrs_regime.sh, rbrv_regime.sh
······x········x······················x· rbrp_cli.sh
······x········x··x····················· burd_regime.sh
·····x·····x················x··········· rbrn_nsproto.env
····x·············x···················x· rbap_AccessProbe.sh
···x·················xx················· rbgjb04-sbom-and-summary.sh, rbgjb05-assemble-metadata.sh, rbgjbm06-syft-per-platform.sh, rbgjbm07-build-info-per-platform.sh
··x····························x······x· RBSTB-trigger_build.adoc
··x···················x···············x· rbgjb04-per-platform-pullback.sh, rbgjb05-push-per-platform.sh, rbgjb08-buildx-push-about.sh
··x···················x········x········ cloudbuild.json
·x········x··························x·· rbtcsl_SlsaProvenance.sh
xx···························x·········· buto_operations.sh
···································x··x· rbmP_laterSecurityAudit.md, rbo.observe.sh, rbss.sentry.sh, rbtcsj_SrjclJupyter.sh
·····························x·····x···· buc_command.sh, crgv.validate.sh
···························x··········x· RBSQB-quota_build.adoc
·······················x··············x· RBSDC-depot_create.adoc, RBSDI-director_create.adoc, RBSGR-governor_reset.adoc, RBSRC-retriever_create.adoc
······················x···············x· rbgjb03-buildx-push-multi.sh
·····················x················x· RBSPV-PodmanVmSupplyChain.adoc, RBSRM-RegimeMachine.adoc, rbrm_regime.sh
····················x···············x··· buq_cli.sh
····················x··············x···· busc_shellcheckrc, vslw_workbench.sh
····················x·············x····· buw_workbench.sh
··················x···················x· rbga_ArtifactRegistry.sh, rbgb_Buckets.sh, rbgi_IAM.sh
··················x················x···· buut_tabtarget.sh, rbhcr_GithubContainerRegistry.sh, rbhh_GithubHost.sh, rbhr_GithubRemote.sh
·················x········x············· BUS0-BashUtilitiesSpec.adoc
·················x·x···················· butd_dispatch.sh
················x·····················x· RBSRA-CredentialFormat.adoc
········x·····························x· RBSDV-director_vouch.adoc
········x·x····························· rbw-Rv.RetrieverVouchesArk.sh
·······x······························x· rbgjv01-download-verifier.sh, rbgjv02-verify-provenance.sh, rbgjv03-assemble-push-vouch.sh
······x·······························x· rbrs_cli.sh
······x··················x·············· butcde_DispatchExercise.sh, bute_engine.sh
······x···········x····················· burc_regime.sh, bure_regime.sh, burs_regime.sh
······x········x························ bul_launcher.sh
·····x······················x··········· rbrn_pluml.env, rbrn_srjcl.env
···x··················x················· rbgjb03-build-and-load.sh, rbgjbm04-per-platform-pullback.sh, rbgjbm05-push-per-platform.sh, rbgjbm09-imagetools-create.sh
··x···································x· rbgjb01-derive-tag-base.sh
x····································x·· rbw-tf.TestFixture.slsa-provenance.sh
x················x······················ rbw-tw.TestSweep.sh
······································x· RBRN-RegimeNameplate.adoc, RBSAA-ark_abjure.adoc, RBSAB-ark_beseech.adoc, RBSAC-ark_conjure.adoc, RBSAJ-access_jwt_probe.adoc, RBSAO-access_oauth_probe.adoc, RBSAS-ark_summon.adoc, RBSAX-access_setup.adoc, RBSBC-bottle_create.adoc, RBSBK-bottle_cleanup.adoc, RBSBL-bottle_launch.adoc, RBSBR-bottle_run.adoc, RBSBS-bottle_start.adoc, RBSCE-command_exec.adoc, RBSCIP-IamPropagation.adoc, RBSCJ-CloudBuildJson.adoc, RBSCO-CosmologyIntro.adoc, RBSCTD-CloudBuildTriggerDispatch.adoc, RBSDD-depot_destroy.adoc, RBSDL-depot_list.adoc, RBSDN-depot_initialize.adoc, RBSDS-dns_step.adoc, RBSGD-gdc_establish.adoc, RBSGS-GettingStarted.adoc, RBSID-image_delete.adoc, RBSIP-iptables_init.adoc, RBSIR-image_retrieve.adoc, RBSNC-network_create.adoc, RBSNX-network_connect.adoc, RBSOB-oci_layout_bridge.adoc, RBSPE-payor_establish.adoc, RBSPI-payor_install.adoc, RBSPR-payor_refresh.adoc, RBSPT-port_setup.adoc, RBSRO-RegimeOauth.adoc, RBSRP-RegimePayor.adoc, RBSRS-RegimeStation.adoc, RBSRV-RegimeVessel.adoc, RBSSC-security_config.adoc, RBSSD-sa_delete.adoc, RBSSL-sa_list.adoc, RBSSR-sentry_run.adoc, RBSSS-sentry_start.adoc, README.md, launcher.rbw_workbench.sh, rbcr_render.sh, rbdc_DerivedConstants.sh, rbga_cli.sh, rbgb_cli.sh, rbq_cli.sh, rbtckk_KickTires.sh, rbtcns_NsproSecurity.sh, rbtcpl_PlumlDiagram.sh, rbtcqa_QualifyAll.sh, rbw_workbench.sh
····································x··· buq_qualify.sh
···································x···· entrypoint.sh, rbmP_laterSecurityAudit.sh, strip.sh, xxx_rbn.info.sh
··································x····· buwz_zipper.sh
·································x······ _test_notch_del_alpha.txt, _test_notch_del_bravo.txt, _test_notch_del_charlie.txt, jjrnc_notch.rs
································x······· RCG-RustCodingGuide.md
·····························x·········· bundle.sh
······················x················· Dockerfile, rbgjb06-build-and-push-metadata.sh, rbgjbm03-buildx-push-multi.sh, rbgjbm08-buildx-push-about.sh, rbrv.env
·····················x·················· rbrm.env
····················x··················· but_test.sh, buw-qsc.QualifyShellCheck.sh
··················x····················· bug_guide.sh, bupr_PresentationRegime.sh, butcev_ChoiceTypes.sh, butcev_EnforceReport.sh, butcev_GateEnroll.sh, butcev_LengthTypes.sh, butcev_ListTypes.sh, butcev_NumericTypes.sh, butcev_RefTypes.sh, buz_zipper.sh, rbha_GithubActions.sh, rbhim_GithubContainerRegistry.sh, rgbs_ServiceAccounts.sh, vob_build.sh, vof_features.sh, vvb_bash.sh
···············x························ rbrp.env
·········x······························ rbw-DP.DirectorRefreshesPins.sh, rbw-DPB.DirectorRefreshesBinaryPins.sh, rbw-DPG.DirectorRefreshesGcbPins.sh, rbw-rgr.RenderPinsRegime.sh, rbw-rgv.ValidatePinsRegime.sh
········x······························· rbw-DV.DirectorVouchesArk.sh
······x································· burc_cli.sh, bure_cli.sh, burs_cli.sh
··x····································· RBWMBX-BuildxMultiPlatformAuth.adoc
·x······································ RBSIL-image_list.adoc, rbw-Rl.RetrieverListsImages.sh
x······································· rbw-ta.TestAll.sh, rbw-tf.TestFixture.access-probe.sh, rbw-tf.TestFixture.ark-lifecycle.sh, rbw-tf.TestFixture.enrollment-validation.sh, rbw-tf.TestFixture.kick-tires.sh, rbw-tf.TestFixture.nsproto-security.sh, rbw-tf.TestFixture.pluml-diagram.sh, rbw-tf.TestFixture.qualify-all.sh, rbw-tf.TestFixture.regime-credentials.sh, rbw-tf.TestFixture.regime-smoke.sh, rbw-tf.TestFixture.srjcl-jupyter.sh, rbw-tn.TestNameplate.nsproto.sh, rbw-tn.TestNameplate.pluml.sh, rbw-tn.TestNameplate.srjcl.sh, rbw-trc.TestRegimeCredentials.sh, rbw-trg.TestRegimeSmoke.sh, rbw-ts.TestSuite.complete.sh, rbw-ts.TestSuite.fast.sh, rbw-ts.TestSuite.service.sh, rbw-ts.TestSuite.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 335 commits)

  1 1 harden-test-sweep-reliability
  2 q consider-inscribe-to-rubric-migration
  3 k nameplate-env-file-location
  4 z shellcheck-finding-repair
  5 0 shellcheck-prerelease-gate
  6 x baseline-test-sweep
  7 2 consolidate-ark-lifecycle-vouch-and-run
  8 K rename-rbw-to-rbk-move-lenses

123456789abcdefghijklmnopqrstuvwxyz
xx···x·····························  1  3c
···xx·xxxxx························  q  7c
···········x·x·····················  k  2c
············x··xxx·················  z  4c
··············x····xx··············  0  3c
························x·······xx·  x  3c
··························xxx······  2  3c
·····························xxx···  K  3c
```

## Steeplechase

### 2026-03-10 14:49 - Heat - n

Remove stale /jjc-* slash command references from AGENT_RESPONSE messages in wrap, orient, parade, and tally Rust source, and from wrap spec. Replace with plain verb forms (mount, retire) matching CLAUDE.md verb table.

### 2026-03-10 14:46 - ₢AkAAx - W

Ran complete test suite (10 fixtures, 84+ cases). All pass: kick-tires, qualify-all, access-probe, ark-lifecycle, nsproto-security, srjcl-jupyter, pluml-diagram, regime-smoke, regime-credentials, enrollment-validation. One transient srjcl-jupyter failure on first run (amd64/arm64 platform warning) passed cleanly on rerun. Green baseline established as mvp-1 exit gate.

### 2026-03-10 14:35 - ₢AkAAx - A

Run all test suites to establish green baseline. Identify tabtargets, execute test-sweep/test-all, capture and record results as mvp-1 exit evidence.

### 2026-03-10 14:34 - ₢AkAAK - W

Renamed Tools/rbw/ to Tools/rbk/, moved 58 RBS* lens files from lenses/ to Tools/rbk/vov_veiled/, updated all cross-references (CLAUDE.md, BUK README, launcher, lens docs). Consolidated magic string "rbk" into single RBBC_kit_subdir bootstrap constant in .buk/rbbc_constants.sh — all 14 cli files and RBCC_KIT_DIR now compose from it. Fixed RBCC_KIT_DIR and ZRBQ_RBW_DIR to use constants instead of hardcoded paths. Regime-smoke (7 cases) and qualify-all pass.

### 2026-03-10 14:34 - ₢AkAAK - n

git mv Tools/rbw→rbk, git mv 37 RBS* lenses→rbk/vov_veiled, update z_rbw_kit_dir→z_rbk_kit_dir in 14 cli files, update CLAUDE.md and all cross-references

### 2026-03-10 14:11 - ₢AkAAK - A

git mv Tools/rbw→rbk, git mv 37 RBS* lenses→rbk/vov_veiled, update z_rbw_kit_dir→z_rbk_kit_dir in 14 cli files, update CLAUDE.md and all cross-references

### 2026-03-10 14:10 - ₢AkAA2 - W

Consolidated ark-lifecycle and slsa-provenance test fixtures into a single 10-step ark-lifecycle fixture exercising the full supply chain (conjure, check, vouch, health check, vouch gate, retrieve, run, cleanup, abjure, check absent). Extracted rbf_vouch_gate() from rbob into foundry as standalone consumer-side vouch verification. Refactored zrbob_vouch_gate_and_summon() to delegate vouch check. Expanded ark baste with full foundry kindle chain. Deleted slsa-provenance fixture, test case, and tabtarget. All 10 steps pass against live GCP.

### 2026-03-10 14:06 - ₢AkAA2 - n

Extract rbf_vouch_gate from rbob to foundry, expand ark baste kindle chain, rewrite ark-lifecycle 10-step test, delete slsa-provenance fixture and tabtarget

### 2026-03-10 13:47 - ₢AkAA2 - A

Extract rbf_vouch_gate from rbob to foundry, expand ark baste kindle chain, rewrite ark-lifecycle 10-step test, delete slsa-provenance fixture and tabtarget, update testbench enrollment

### 2026-03-10 13:46 - Heat - r

moved AkAAx to last

### 2026-03-10 13:46 - ₢AkAAx - A

Run full test sweep to establish green baseline. Identify all test suites, execute via test-all/test-sweep tabtargets, capture results as mvp-1 exit gate evidence.

### 2026-03-10 13:45 - Heat - r

moved AkAAK to last

### 2026-03-10 13:44 - Heat - T

consolidate-ark-lifecycle-vouch-and-run

### 2026-03-10 13:37 - Heat - S

consolidate-ark-lifecycle-vouch-and-run

### 2026-03-10 13:26 - ₢AkAA0 - W

Wired shellcheck into prerelease qualification gate: extracted buq_shellcheck into buq_qualify.sh module with BURD_* fallback defaults, added to rbq_qualify_all so rbw-qa runs shellcheck alongside tabtarget and colophon checks. Renamed buq_qualify_tabtargets to buq_tabtargets. Fixed pre-existing rbw-qa dispatch bug (zipper enrolled qualify_all without rbq_ prefix). All three qualification steps pass clean.

### 2026-03-10 13:26 - ₢AkAA0 - n

Extract buq_shellcheck into buq_qualify.sh module with BURD_* fallbacks, wire into rbq_qualify_all gate, rename buq_qualify_tabtargets to buq_tabtargets, fix pre-existing rbw-qa dispatch bug (missing rbq_ prefix in zipper enrollment)

### 2026-03-10 13:13 - Heat - n

Reword size guard discipline from negative prohibition to positive action template with specific ask phrasing, paralleling wrap discipline structure

### 2026-03-10 13:10 - ₢AkAAz - W

Fixed 156 shellcheck findings across 31 files to pass buw-qsc clean (134 files, 0 findings). Added 3 structural suppressions to busc_shellcheckrc (SC2016 template strings, SC2254 glob case patterns, SC2059 printf format constants). Converted word-splitting string variables to arrays (TCPDUMP_OPTS, CENSER_NETWORK_ARGS, SOURCE_IMAGES). Renamed rbmP memo from .sh to .md. Tightened BCG with parameter expansion quoting rule, A&&B||C prohibition, and updated Shellcheck Integration table. Fixed rbo.observe.sh shebang.

### 2026-03-10 13:09 - ₢AkAAz - n

BCG tightenings from shellcheck audit: document SC2016/SC2254/SC2059 in Shellcheck Integration table, add parameter expansion quoting rule (SC2295), add A&&B||C pseudo-ternary prohibition (SC2015). Fix rbo.observe.sh shebang from sh to bash.

### 2026-03-10 13:05 - ₢AkAAz - n

Fix 156 shellcheck findings across 31 files: SC2086 quoting, SC2295 parameter expansion quoting, SC2188 bare truncation to colon-truncation, SC2015 if-then-else rewrites, SC1083 brace escaping, SC2129 redirect grouping, array conversions for word-splitting patterns (TCPDUMP_OPTS, CENSER_NETWORK_ARGS, SOURCE_IMAGES), structural suppressions (SC2016 SC2254 SC2059) in busc_shellcheckrc, rbmP memo renamed .sh to .md. buw-qsc passes clean: 134 files, 0 findings.

### 2026-03-10 12:41 - ₢AkAA0 - A

Read rbq_Qualify.sh and buw_workbench.sh to understand qualification structure, add shellcheck gate to rbq_qualify_all(), verify with buw-qsc, document in RBS0 if applicable

### 2026-03-10 12:40 - ₢AkAAk - W

Verified nameplate .env file relocation: all 3 files (.rbk/rbrn_nsproto.env, rbrn_srjcl.env, rbrn_pluml.env) confirmed in .rbk/, removed from Tools/rbw/, all live code references updated to RBBC_dot_dir. Only stale ref is in a historical memo.

### 2026-03-10 12:40 - ₢AkAAz - A

Run baseline shellcheck, fix 154 findings by domain (BUK core, RBW modules, legacy, other kits), verify with buw-qsc after each batch

### 2026-03-10 12:40 - ₢AkAAk - A

Verify nameplate .env move: check file locations, grep for stale references, advise on status

### 2026-03-10 12:36 - ₢AkAAq - W

Inscribe generates directly into rubric clone: stitch takes output path param, clone-first flow with cp -R of entire vessel dir, Phase 3 (verify committed) deleted, 6 committed cloudbuild.json files removed from git. BCG repairs in rbf_Foundry.sh: temp file for base64 decode, jq-to-file then iterate pattern, case statements replacing [[ ]] glob matches. Added vouch suffix to beseech tag classification. Migrated syft SBOM from docker: to registry: transport with OAuth2 token from GCB metadata server — eliminates Docker daemon API version coupling that broke on pin refresh. Updated 4 specs (RBSRI, RBSTB, RBS0, RBSCB).

### 2026-03-10 12:32 - ₢AkAAq - n

Replace jq with sed for metadata token parsing — jq not available in cloud-builders/docker image

### 2026-03-10 12:30 - ₢AkAAq - n

Switch syft from docker: to registry: transport with OAuth2 token from GCB metadata server, eliminating Docker daemon API version coupling. Update RBSCB and RBSTB docs.

### 2026-03-10 12:13 - ₢AkAAq - n

Add vouch suffix to beseech tag classification and correlation display

### 2026-03-10 12:08 - ₢AkAAq - n

Refresh GCB pins: syft and skopeo digests updated

### 2026-03-10 12:06 - ₢AkAA1 - W

Three test reliability fixes: (1) Ark lifecycle test replaced fragile list-counting with per-consecration fact files using test -f presence/absence assertions, added RBCC_FACT_CONSEC_INFIX constant and buto_tt_previous_output_capture encapsulation. (2) Bottle service baste functions now call rbob_start with RBCC_BOTTLE_TEST_READINESS_DELAY_SEC sleep for srjcl/pluml. (3) Modernized test tabtargets: rbw-tf (fixture by imprint), rbw-ts (suite by imprint), rbw-to (list-and-die), removed 8 redundant tabtargets. Sequential execution guard deferred.

### 2026-03-10 12:05 - ₢AkAAq - n

Inscribe generates directly into rubric clone: stitch takes output path param, clone-first flow, Phase 3 deleted, committed cloudbuild.json files removed. BCG repairs: temp file for base64 decode, jq-to-file then iterate, case instead of [[ ]] globs. Four spec files updated to match.

### 2026-03-10 11:57 - ₢AkAAq - A

Revised docket: stitch takes output path, clone-first flow, cp -R entire vessel dir, delete Phase 3, rm committed JSONs, update 4 specs

### 2026-03-10 11:55 - Heat - T

consider-inscribe-to-rubric-migration

### 2026-03-10 11:55 - ₢AkAA1 - n

Modernize test tabtargets: rbw-tf (fixture by imprint), rbw-ts (suite by imprint), rbw-to (single function with list-and-die). Remove 8 redundant tabtargets (rbw-ta, rbw-tn.*, rbw-tw, rbw-trc, rbw-trg, old rbw-ts). Clean up zipper enrollments.

### 2026-03-10 11:39 - ₢AkAA1 - n

Rename RBCC_GENERIC_SERVICE_START_SECONDS to RBCC_BOTTLE_TEST_READINESS_DELAY_SEC with apologia comment explaining the delay-and-pray trade-off.

### 2026-03-10 11:39 - ₢AkAAq - A

Refactor rbf_rubric_inscribe() to generate JSON directly into rubric clone, delete committed cloudbuild.json files, update 4 specs

### 2026-03-10 11:37 - ₢AkAA1 - n

Bottle service baste functions start bottles and sleep for service readiness. All three service fixtures (nsproto, srjcl, pluml) now pass.

### 2026-03-10 11:23 - ₢AkAA1 - n

Bottle service fixtures: add rbob_start to nsproto, srjcl, pluml baste functions so bottle services are running before test cases execute.

### 2026-03-10 11:17 - ₢AkAAy - W

Migrated buw_workbench.sh from hardcoded case-statement routing to buz_zipper dispatch. Created buwz_zipper.sh enrolling all 13 BUK colophons (tt-*, rc*, rs*, re*, qsc). Verified with buw-tt-ll, buw-rcv, buw-rsv tabtargets.

### 2026-03-10 11:14 - ₢AkAAy - n

Migrate buw_workbench.sh from hardcoded case-statement routing to buz_zipper dispatch. Created buwz_zipper.sh enrolling all 13 BUK colophons; refactored workbench to source buz/buwz zippers, kindle, and dispatch via buz_exec_lookup.

### 2026-03-10 11:12 - ₢AkAA1 - n

Ark lifecycle test: replace list-counting with per-consecration fact files (test -f presence/absence). Add RBCC_FACT_CONSEC_INFIX constant, buto_tt_previous_output_capture encapsulation, remove dead RBF_FACT_CONSECRATIONS constant.

### 2026-03-10 11:10 - ₢AkAAy - A

Create buwz_zipper.sh with zbuwz_kindle enrolling 13 BUK colophons; refactor buw_workbench.sh to source buz/buwz zippers, kindle, and dispatch via buz_exec_lookup; verify tabtargets.

### 2026-03-10 10:37 - ₢AkAA1 - A

Sequential opus. Three independent issues: (1) ark-lifecycle fact file design discussion, (2) bottle service stop-before-start fixtures, (3) sequential execution guard for sweep. Start with design discussion on issue 1, then implement 2 and 3.

### 2026-03-09 20:50 - Heat - T

harden-test-sweep-reliability

### 2026-03-09 20:48 - Heat - S

harden-test-sweep-reliability

### 2026-03-09 20:45 - ₢AkAAQ - W

Shellcheck discovery and integration: ran shellcheck across Tools/, classified all warnings as BCG-structural (7 codes) vs genuine (21 codes, 154 findings). Built busc_shellcheckrc suppress list, buq_cli.sh qualification CLI, buw-qsc workbench route and tabtarget. Added three BCG prohibited constructs: bare file truncation, inline shellcheck directives, commit-message comments. Removed all 7 existing inline shellcheck directives. Slated follow-on paces for finding repair (₢AkAAz) and prerelease gate (₢AkAA0).

### 2026-03-09 20:25 - Heat - S

shellcheck-prerelease-gate

### 2026-03-09 20:25 - Heat - S

shellcheck-finding-repair

### 2026-03-09 20:17 - ₢AkAAQ - n

Add commit-message comment prohibition to BCG (definition, examples, litmus test, checklist item); fix buw_workbench.sh to use z_buq_cli local variable matching surrounding pattern

### 2026-03-09 20:13 - Heat - S

buw-workbench-zipper-migration

### 2026-03-09 20:10 - ₢AkAAQ - n

Add shellcheck qualification: busc_shellcheckrc (BCG-structural suppressions), buq_cli.sh (BUK qualify CLI), buw-qsc tabtarget and workbench route, BCG rules for bare truncation and inline directives, remove all 7 existing inline shellcheck directives

### 2026-03-09 19:45 - Heat - S

baseline-test-sweep

### 2026-03-09 19:42 - Heat - f

silks=rbk-mvp-1-finalization

### 2026-03-09 19:33 - ₢AkAAQ - A

Discovery: run shellcheck on rbgu_Utility.sh and a _cli.sh file, classify warnings as BCG-intentional vs genuine, defer integration decisions for human conversation

### 2026-03-09 19:25 - ₢AkAAw - W

Fixed jjrnc_notch.rs to handle staged deletions: tracks git-rm'd files during validation, filters them from git add args. Verified with 3 tests: baseline add, pure staged deletion, and mixed deletion+modification.

### 2026-03-09 19:22 - Heat - T

consider-inscribe-to-rubric-migration

### 2026-03-09 19:19 - ₢AkAAw - n

Track staged deletions during validation, filter from git add args so notch handles git-rm'd files

### 2026-03-09 19:19 - ₢AkAAw - n

Test cleanup: remove remaining test file

### 2026-03-09 19:18 - ₢AkAAw - n

Test 3b: mixed notch — one staged deletion, one modified file

### 2026-03-09 19:17 - ₢AkAAw - n

Test 3a: commit two files for mixed deletion test

### 2026-03-09 19:16 - ₢AkAAw - n

Test 2: notch a staged deletion (pure)

### 2026-03-09 19:13 - ₢AkAAw - n

Test 1: baseline notch of new file

### 2026-03-09 19:10 - ₢AkAAq - A

Enumerate inscribe surface area in local repo (files, functions, tabtargets, specs), map rubric-side presence, trace dependency graph, then interactive design discussion on migration trade-offs.

### 2026-03-09 19:10 - ₢AkAAw - A

Read jjrnc_notch.rs, add staged_deletions tracking during validation, filter from git add args, build and test

### 2026-03-09 18:43 - ₢AkAAs - W

Implemented rbw-DV batch vouch and enhanced rbw-Dc check-consecrations with health states. Renamed tabtarget rbw-Rv to rbw-DV, created RBSDV spec subdocument. Batch vouch enumerates all vessels, vouches pending consecrations with isolation-subshell resilience for provenance failures. Check-consecrations displays vouched/pending/incomplete health per consecration with buc_tabtarget recommendations. Fixed rbf_abjure empty-array crash on orphaned-about arks. Sharpened BCG array safety guidance. Validated against live registry: batch vouch, abjure cleanup, and clean re-run all working.

### 2026-03-09 18:34 - ₢AkAAs - n

Sharpened BCG array safety guidance: removed primary/acceptable hierarchy, reframed as two equal patterns chosen by whether index is needed. Tightened prose, removed redundant comments in examples.

### 2026-03-09 18:32 - ₢AkAAs - n

Fixed rbf_abjure empty-array unbound-variable crash when abjuring orphaned -about artifacts (no image tags). Applied BCG guarded value iteration pattern at all three z_existing_image_tags loops.

### 2026-03-09 18:14 - ₢AkAAs - A

Run tt/rbw-DV.DirectorVouchesArk.sh to validate batch vouch with provenance resilience, then wrap if successful

### 2026-03-09 18:12 - Heat - S

notch-staged-deletion-support

### 2026-03-09 18:06 - ₢AkAAs - n

Renamed tabtarget rbw-Rv to rbw-DV, updated zipper. Rewrote rbf_check_consecrations as zero-arg all-vessels with health states (vouched/pending/incomplete) and buc_tabtarget recommendations. Added rbf_batch_vouch with isolation-subshell resilience. Created RBSDV spec subdocument and RBS0 include.

### 2026-03-09 11:37 - ₢AkAAv - W

Installed load-bearing complexity principle in three documents: BCG Core Philosophy (concrete bash/GCP examples — four rbgi_add_* functions load-bearing, three Secret Manager IAM blocks not), RCG (brief paragraph noting existing disciplines are instantiations), CLAUDE.md (new Design Principles section with cross-cutting reference).

### 2026-03-09 11:37 - ₢AkAAX - W

Replaced die-with-tabtarget-hint image preflight in rbob_start with vouch-gate + auto-summon. Missing images are now auto-pulled after verifying the Director has vouched the consecration; unvouched consecrations hard-fail with a clear message. Added rbgo_OAuth and burd_regime sourcing to rbob_cli.sh. Updated nsproto consecrations to vouched i20260308 builds. Made zrbf_wait_build_completion max_attempts a required parameter: vouch 36 (3 min), conjure 960 (80 min). Validated all paths: auto-summon happy, vouch gate hard-fail, full nsproto start success.

### 2026-03-09 11:36 - ₢AkAAX - n

Make zrbf_wait_build_completion max_attempts a required parameter. Vouch passes 36 (3 min), conjure passes 960 (80 min).

### 2026-03-09 11:33 - ₢AkAAX - n

Replace die-with-hint image preflight with vouch-gate + auto-summon. Sentry and bottle auto-pull on start when missing locally, hard-fail on unvouched consecrations. Updated nsproto consecrations to vouched i20260308 builds. Validated: auto-summon happy path, vouch gate hard-fail, full nsproto start success.

### 2026-03-09 11:26 - ₢AkAAs - A

Read rbf_vouch, rbf_check_consecrations, rbw-Rv tabtarget, rbz_zipper. Mint spec subdocument name. Rename tabtarget rbw-Rv→rbw-DV, update zipper. Implement batch vouch with provenance resilience. Enhance check_consecrations with 4 health states + buc_tabtarget recommendations. Update RBS0 include.

### 2026-03-09 11:17 - ₢AkAAv - n

Install load-bearing complexity principle in BCG (Core Philosophy with concrete bash/GCP examples), RCG (brief paragraph linking existing disciplines), and CLAUDE.md (new Design Principles section with cross-cutting reference).

### 2026-03-09 11:14 - ₢AkAAv - A

Read BCG, RCG, CLAUDE.md; install load-bearing complexity principle in each document's voice. BCG gets concrete examples in Core Philosophy. RCG gets brief paragraph. CLAUDE.md gets new Design Principles section.

### 2026-03-09 11:14 - ₢AkAAX - A

Read rbob_bottle.sh die-with-hint blocks + rbf_summon. Replace with vouch gate (HEAD for -vouch tag, hard fail) + auto-summon. Update rbrn_nsproto.env consecrations to i20260308_082033 builds.

### 2026-03-09 10:01 - ₢AkAAa - W

Assessed RBSCIP function references for linked-term warrant. Discovered existing rbtoe_ category for subordinate operations. Created 5 new rbtoe_ linked terms (poll_readiness, iam_grant_project/repo/sa/bucket) with inline definitions anchored to RBSCIP backoff profile. Replaced 10 inline {rbbc_call} setIamPolicy references across 4 subdocuments (RBSDC, RBSDI, RBSGR, RBSRC). Investigated Secret Manager IAM and complete-policy-write patterns, determined they are intentionally different (different failure classes). Slated iam-grant-homogenization pace (AkAAu) for future unification of 4 rbgi_add_* functions with BCG-compliant helper extraction. Surfaced load-bearing complexity as a cross-cutting design principle and slated installation pace (AkAAv) for BCG, RCG, and CLAUDE.md.

### 2026-03-09 10:00 - Heat - S

install-load-bearing-principle

### 2026-03-09 09:48 - Heat - T

iam-grant-homogenization

### 2026-03-09 09:45 - Heat - S

iam-grant-homogenization

### 2026-03-09 09:31 - ₢AkAAa - n

Replace inline {rbbc_call} setIamPolicy references with rbtoe_iam_grant_* linked terms in 4 subdocuments (10 replacements total). Secret Manager and complete-policy-write sections left untouched.

### 2026-03-09 09:19 - ₢AkAAa - n

Add 5 rbtoe_ linked terms for IAM grant patterns and poll readiness: mapping entries and inline definitions with RBSCIP backoff profile anchors

### 2026-03-09 08:59 - ₢AkAAr - W

Implemented and debugged rbf_vouch() end-to-end. Key fixes: switched step 2 from alpine to gcloud builder image, added --provenance-path and --builder-id flags (slsa-verifier doesn't auto-discover GCB provenance), stripped .git suffix and git+ prefix from source URI, replaced jq with python3 for JSON parsing. Updated RBSAV spec to match. Tested successfully on two vessels (busybox 3-platform, sentry 2-platform). Reslated next pace with no-provenance resilience requirements.

### 2026-03-09 08:59 - Heat - T

director-batch-vouch-and-health-display

### 2026-03-09 08:52 - ₢AkAAr - n

Update RBSAV spec to match working implementation: gcloud image for step 2, --provenance-path/--builder-id flags, source URI without git+ prefix, Container Analysis provenance discovery notes

### 2026-03-09 08:47 - ₢AkAAr - n

Drop git+ prefix from source URI — slsa-verifier with --provenance-path prepends https:// itself, provenance records bare URL

### 2026-03-09 08:46 - ₢AkAAr - n

Replace jq with python3 for JSON parsing (ships with gcloud image), simplify manifest iteration with read loop

### 2026-03-09 08:44 - ₢AkAAr - n

Rewrite verify-provenance step: switch to gcloud builder image, use gcloud to fetch provenance, pass --provenance-path and --builder-id to slsa-verifier per documented GCB workflow

### 2026-03-09 08:39 - ₢AkAAr - n

Broaden CA API diagnostic: query all occurrence kinds for digest, plus all DSSE_ATTESTATION in project (no resource filter)

### 2026-03-09 08:35 - ₢AkAAr - n

Add slsa-verifier version check and DSSE_ATTESTATION-specific CA API query to diagnostics

### 2026-03-09 08:32 - ₢AkAAr - n

Fix CA API diagnostic: use shell intermediary for GAR_PATH parameter expansion (Cloud Build subs dont support %% modifiers)

### 2026-03-09 08:30 - ₢AkAAr - n

Add Container Analysis API diagnostic curl before slsa-verifier call to check attestation reachability

### 2026-03-09 08:29 - ₢AkAAr - n

Add diagnostic output to verify-provenance step: echo ref and source-uri, capture slsa-verifier stderr on failure

### 2026-03-09 08:23 - ₢AkAAr - n

Add vouch Cloud Build step scripts (download verifier, verify provenance, assemble and push vouch artifact)

### 2026-03-09 08:22 - ₢AkAAr - n

Strip .git suffix from source URI to match Cloud Build provenance format, fixing slsa-verifier 'no matching attestations' failure

### 2026-03-09 08:11 - ₢AkAAr - A

Read RBSAV spec and existing rbf_vouch() from prior attempt in Foundry. Implement/refine rbf_vouch() per spec: validate, OAuth, HEAD gating, Cloud Build JSON (3 steps), POST+poll, display. Sequential opus.

### 2026-03-08 14:42 - ₢AkAAb - W

Removed spurious method parameter from rbgu_poll_until_ok. Hardcoded GET inside the function, dropped the first parameter, updated all 8 call sites across rbgp_Payor.sh (7) and rbgg_Governor.sh (1). RBSCIP lens unchanged — prose references function name, not parameter list.

### 2026-03-08 14:42 - ₢AkAAb - n

Hardcode GET in rbgu_poll_until_ok, remove method param, update 8 callers

### 2026-03-08 14:36 - ₢AkAAb - A

Hardcode GET in rbgu_poll_until_ok, remove method param, update 8 callers, update RBSCIP lens

### 2026-03-08 14:27 - ₢AkAAa - A

Read RBSCIP and RBS0, inventory function references, assess linked-term warrant, implement if justified

### 2026-03-08 14:26 - ₢AkAAr - A

Read RBSAV spec, study rbf_conjure/rbf_beseech patterns in Foundry, implement rbf_vouch() with Cloud Build JSON (3 steps), OAuth, gating HEADs, polling, display. Sequential opus.

### 2026-03-08 14:25 - ₢AkAAl - W

Added BCG ban on test && break/continue/return idiom (banned-pattern section, quick-reference row, checklist entry). Transformed all 27 violations across 10 files to inverted-test || form.

### 2026-03-08 14:25 - ₢AkAAt - W

Deleted rbw-Rl tabtarget, rbf_list() function, RBZ_LIST_IMAGES zipper enrollment, and RBSIL-image_list spec section/lens. Added RBF_FACT_CONSECRATION(S) kindle constants. Rewrote ark-lifecycle test to use check-consecrations fact files and abjure instead of flat locator listing. Added ZBUTO_BURV_OUTPUT_DIR to eliminate current/ magic string in test BURV paths. Fixed rbtcsl hardcoded consecration filename. Added zrbgc_kindle to ark-lifecycle baste. All tests pass.

### 2026-03-08 14:20 - ₢AkAAt - n

Add zrbgc_kindle to ark-lifecycle baste so RBF_FACT_* constants are available in test shell

### 2026-03-08 14:19 - ₢AkAAl - n

Transform 27 banned && break/continue/return patterns to inverted-test || form across 10 files

### 2026-03-08 14:16 - ₢AkAAt - n

Delete rbw-Rl/rbf_list/RBZ_LIST_IMAGES, remove spec section and lens file, add RBF_FACT_CONSECRATION(S) constants, rewrite ark lifecycle test to use check-consecrations fact files and abjure, add ZBUTO_BURV_OUTPUT_DIR to eliminate current/ magic string in tests

### 2026-03-08 14:11 - ₢AkAAl - n

Add BCG ban on test && break/continue/return idiom: banned-pattern section, quick-reference row, checklist entry

### 2026-03-08 14:01 - ₢AkAAl - A

Verify completeness of prior commit: grep for remaining &&break/&&continue/&&return patterns, confirm BCG guidance in place, report status

### 2026-03-08 14:00 - Heat - n

Defensive deserialization for stringified MCP params, plus CLAUDE.md guidance reinforcing params must be a JSON object

### 2026-03-08 13:56 - ₢AkAAl - A

BCG ban + repair: 6x &&break, 2x &&continue, ~17x &&return across 8 files. Parallel file edits.

### 2026-03-08 13:53 - ₢AkAAt - A

Delete tt/rbw-Rl tabtarget, remove rbf_list() from Foundry, remove RBZ_LIST_IMAGES zipper enrollment, grep-clean remaining references

### 2026-03-08 13:52 - Heat - r

moved AkAAs after AkAAr

### 2026-03-08 13:52 - Heat - T

gcb-pin-slsa-verifier

### 2026-03-08 13:51 - Heat - T

remove-spurious-poll-method-parameter

### 2026-03-08 13:51 - Heat - T

remove-spurious-poll-method-parameter

### 2026-03-08 13:49 - Heat - T

vouch-implementation-from-spec

### 2026-03-08 13:48 - Heat - S

delete-retriever-list-images

### 2026-03-08 13:47 - Heat - n

Fix jjx_transfer coronets param documentation: clarify it takes a JSON-encoded string, not a native array (spook from restring attempt)

### 2026-03-08 13:44 - Heat - S

director-batch-vouch-and-health-display

### 2026-03-08 13:44 - Heat - T

bottle-start-auto-summon-missing

### 2026-03-08 13:41 - Heat - T

rename-rubric-inscribe-tabtarget

### 2026-03-08 13:24 - ₢AkAAX - A

Auto-summon missing images in rbob_start instead of dying with tabtarget hints

### 2026-03-08 13:12 - ₢AkAAk - n

Move nameplate .env files from Tools/rbw/ to .rbk/ alongside other regime config. Update all path constructions from RBCC_KIT_DIR to RBBC_dot_dir in 4 consumer files.

### 2026-03-08 13:07 - ₢AkAAr - n

Remove caller-side digest discovery step; fold into build step 2 which discovers digests from manifest list internally. Drop _RBGV_PLATFORM_DIGESTS substitution variable. Build is now self-contained: only needs registry coordinates, verifier identity, and source URI.

### 2026-03-08 13:04 - ₢AkAAk - A

Investigate nameplate .env file locations: survey regime .env pattern, trace all references, determine principled location

### 2026-03-08 13:03 - ₢AkAAr - n

Revise RBSAV to match sibling subdoc style: guillemet placeholders, role-subject introduction, split monolithic step 4 into discrete steps (digest discovery, submit, 3 build steps, wait), add semantic markers, specify per-platform digest discovery from manifest list, add substitution variable inventory, document source URI format (git+ prefix from provenance memo)

### 2026-03-08 12:53 - ₢AkAAr - A

Review RBSAV spec (two versions), critically assess improvement suggestions for simplification, discuss with user, then implement rbf_vouch + wiring (constant, zipper, abjure, check_consecrations, tabtarget)

### 2026-03-08 12:36 - Heat - r

moved AkAAr to first

### 2026-03-08 12:36 - ₢AkAAo - W

Spec-first pivot after failed implementation attempt. Produced RBSAV-ark_vouch.adoc subdoc capturing vouch architecture and discovered GCB constraints (default pool, metadata server auth, no gcloud, dollar escaping). Reverted implementation code. Slated AkAAr for spec-reviewed implementation.

### 2026-03-08 12:36 - Heat - n

Install gcloud CLI prohibition as axk_premise (rbsk_no_gcloud) in RBS0 Key Premises section. Replace inline policy restatements in RBSAV and vouch constraints with premise reference. Annotate rbrg_gcloud_image_ref as convenience-shell-only.

### 2026-03-08 12:35 - Heat - T

vouch-implementation-from-spec

### 2026-03-08 12:31 - ₢AkAAo - n

Revise RBSAV to operation-level abstraction matching RBSAC style: remove shell-level detail, add vouch content schema, specify source-uri matching and re-vouch idempotency, elevate no-gcloud constraint, reference pinned image terms

### 2026-03-08 12:29 - Heat - T

rename-rbw-to-rbk-move-lenses

### 2026-03-08 12:24 - Heat - S

vouch-implementation-from-spec

### 2026-03-08 12:24 - ₢AkAAo - n

Add RBSAV vouch operation subdoc with discovered constraints from failed implementation (default pool, metadata server auth, no gcloud, GCB escaping). Include directive in RBS0.

### 2026-03-08 12:07 - ₢AkAAo - n

Remove rbf_vouch implementation (316 lines). Mechanical wiring retained (constant, zipper, abjure, check_consecrations vouch column, tabtarget). Implementation deferred to new pace after spec captures lessons: default pool vs private pool permissions, no gcloud in CB steps (metadata server auth), slsa-verifier builder-id requirements.

### 2026-03-08 11:55 - ₢AkAAo - n

Drop private worker pool from vouch build — use default (public) pool to avoid cloudbuild.workerpools.use permission requirement on Director SA

### 2026-03-08 11:51 - ₢AkAAo - n

Fix vouch build submission: specify Mason service account in Build JSON (serviceAccount field) so builds.create uses Mason instead of legacy default CB SA. Remove unused _RBGV_ARK_SUFFIX_ABOUT substitution that GCB rejected.

### 2026-03-08 11:47 - ₢AkAAp - n

Add RBRG render and validate commands (rbrr_render_pins, rbrr_validate_pins) with rbw-rgr and rbw-rgv tabtargets

### 2026-03-08 11:43 - ₢AkAAo - n

Reintroduce vouch infrastructure with slsa-verifier binary tool architecture. Restore RBGC_ARK_SUFFIX_VOUCH constant, zipper enrollment, vouch check/delete in abjure, vouch column in check_consecrations. New rbf_vouch function submits 3-step Cloud Build job via RunBuild API: download+verify slsa-verifier binary (SHA256 pinned), run verify-image per platform, push scratch vouch container. New rbw-Rv tabtarget.

### 2026-03-08 11:37 - ₢AkAAp - n

Split RBRG_PINS_REFRESHED_AT into independent IMAGE and BINARY timestamps. Update regime enrollment, shared file writer, both refresh commands, inscribe staleness gate, and all spec/subdoc references.

### 2026-03-08 11:34 - Heat - S

consider-inscribe-to-rubric-migration

### 2026-03-08 11:27 - ₢AkAAo - A

Reintroduce vouch using slsa-verifier as binary tool (not container image per ₢AkAAp update). CB job: alpine step downloads binary via RBRG_SLSA_VERIFIER_URL, verifies SHA256, runs verify-image. Restore constants/zipper/abjure, implement rbf_vouch as RunBuild caller, update specs.

### 2026-03-08 11:26 - ₢AkAAp - n

Pin slsa-verifier as binary tool (URL+SHA256) following crane precedent. Split pin refresh into rbw-DPG (container images) and rbw-DPB (binary tools with belt-and-suspenders checksum verification against SLSA provenance attestation). Shared BCG-compliant file rewriter preserves both sections.

### 2026-03-08 10:39 - Heat - T

vouch-slsa-verifier-architecture

### 2026-03-08 10:38 - ₢AkAAp - n

Add slsa-verifier as managed GCB pin: placeholder digest in rbrg.env, GHCR semver discovery in refresh logic, regime enrollment, spec and attribute definitions

### 2026-03-08 10:34 - ₢AkAAp - A

Replicate ORAS ghcr.io pin pattern for slsa-verifier: add pin entry to rbrg.env, add refresh logic to rbrr_cli.sh (bearer token + semver discovery), add regime validation in rbrg_regime.sh, update RBSRG spec

### 2026-03-08 10:32 - Heat - T

vouch-slsa-verifier-architecture

### 2026-03-08 10:32 - Heat - S

gcb-pin-slsa-verifier

### 2026-03-08 10:31 - Heat - T

eliminate-vouch-provenance-on-demand

### 2026-03-08 10:19 - ₢AkAAo - n

Delete vouch tabtarget (part of vouch infrastructure removal)

### 2026-03-08 10:19 - ₢AkAAo - n

Remove vouch infrastructure and simplify list to tag-only display. Deleted rbf_vouch function, RBGC_ARK_SUFFIX_VOUCH constant, vouch zipper enrollment, vouch tabtarget. Stripped vouch check/delete from rbf_abjure. Overhauled rbf_check_consecrations to parse full consecrations (not inscription-only grouping) showing platforms and about status without provenance API calls. Simplified SLSA test to conjure/check/abjure without vouch assertions. Updated RBS0 and RBSCB specs. Prepares clean baseline for slsa-verifier vouch architecture.

### 2026-03-08 10:19 - Heat - T

eliminate-vouch-provenance-on-demand

### 2026-03-08 09:36 - ₢AkAAo - A

Remove vouch infrastructure (function, constant, zipper, tabtarget), overhaul rbf_check_consecrations to query Container Analysis for SLSA provenance on demand per consecration, update abjure and specs

### 2026-03-08 09:34 - Heat - r

moved AkAAo to first

### 2026-03-08 09:34 - Heat - r

moved AkAAo before AkAAn

### 2026-03-08 09:34 - Heat - S

eliminate-vouch-provenance-on-demand

### 2026-03-08 09:28 - ₢AkAAn - W

Hardened build poll loop in zrbf_wait_build_completion to tolerate 3 consecutive curl/HTTP failures before dying. Single transient failure logs warning and continues. HTTP error responses (401/403) detected via temp file pattern per BCG. Not yet battle-tested on full sentry run but code change is contained.

### 2026-03-08 09:28 - ₢AkAAm - W

Renamed TAG_BASE to CONSECRATION across all 7 build steps and foundry. Added images: field with inscribe-time SLSA alias tags for CB provenance generation. Added SLSA alias docker tag in step 04 pullback. Fixed inscribe to fill __INSCRIBE_TIMESTAMP__ in images: array. Updated RBS0 spec, RBSCB, RBSTB, RBWMBX docs. Validated end-to-end: busybox conjure SUCCESS with SLSA Level 3 on all 3 platforms, vouch passed. Bottle-ubuntu-test also built successfully.

### 2026-03-08 09:26 - Heat - n

Add vvco_Output context-aware output abstraction to VVC. Replace all jjbuf!/eprintln! with vvco_out!/vvco_err! macros across VVC and JJK. Console mode routes to stdout/stderr; buffer mode accumulates for MCP transport. Fixes silent MCP failures where eprintln! output was swallowed. Adds axe_console/axe_mcp_transport AXLA terms, vose_output VOS0 entity, updates vosr_commit and vosr_guard specs.

### 2026-03-08 09:11 - Heat - T

bottle-start-auto-summon-missing

### 2026-03-08 08:54 - ₢AkAAn - n

Harden build poll loop: tolerate 3 consecutive curl/HTTP failures before dying, BCG-compliant temp file pattern

### 2026-03-08 08:50 - ₢AkAAn - A

Add retry tolerance to poll loop: 3 consecutive curl failures before die, warn on transient errors and HTTP error responses

### 2026-03-08 08:41 - Heat - S

conjure-poll-retry-hardening

### 2026-03-08 08:24 - ₢AkAAm - A

VALIDATED: Busybox conjure SUCCESS, SLSA Level 3 on all 3 platforms (amd64/arm64/armv7), vouch passed. Consecration i20260308_082033-b20260308_152144. Dual-tag scheme works end-to-end.

### 2026-03-08 08:20 - ₢AkAAm - n

Fix inscribe: fill __INSCRIBE_TIMESTAMP__ placeholder in images: array, not just substitutions

### 2026-03-08 08:15 - ₢AkAAX - A

test mark

### 2026-03-08 08:14 - ₢AkAAm - n

Re-stitch rbev-ubu-safety cloudbuild.json

### 2026-03-08 08:14 - ₢AkAAm - n

Re-stitch rbev-sentry-ubuntu-large cloudbuild.json

### 2026-03-08 08:14 - ₢AkAAm - n

Re-stitch rbev-bottle-anthropic-jupyter cloudbuild.json

### 2026-03-08 08:14 - ₢AkAAm - n

Re-stitch rbev-bottle-ubuntu-test cloudbuild.json

### 2026-03-08 08:14 - ₢AkAAm - n

Re-stitch rbev-bottle-plantuml cloudbuild.json

### 2026-03-08 08:14 - ₢AkAAm - n

Re-stitch rbev-busybox cloudbuild.json with images: field and CONSECRATION naming

### 2026-03-08 08:13 - ₢AkAAm - n

Rename TAG_BASE to CONSECRATION across build steps, add images: field with SLSA alias tags in stitch, update spec and docs

### 2026-03-08 07:47 - ₢AkAAm - A

Read tag lifecycle across rbgjb01/04/05 and stitch. Design dual-tag scheme: images:-compatible (inscribe_ts) for SLSA provenance + TAG_BASE (build_ts) for uniqueness. Update stitch, pullback, push steps.

### 2026-03-07 21:02 - ₢AkAAZ - n

Re-stitch cloudbuild.json without images field (revert to working state, no SLSA provenance until ₢AkAAm).

### 2026-03-07 21:02 - ₢AkAAZ - n

Revert images field restoration — TAG_BASE mismatch causes build failures. Slated ₢AkAAm for proper fix. Comment documents the situation.

### 2026-03-07 21:01 - Heat - S

fix-images-field-tag-base-mismatch

### 2026-03-07 20:46 - ₢AkAAZ - n

Re-stitch cloudbuild.json with restored images field for SLSA provenance.

### 2026-03-07 20:45 - ₢AkAAZ - n

Restore images field in stitch function. Regression from ₢AkAAj removed images: from jq template, breaking CB-native SLSA provenance generation.

### 2026-03-07 20:30 - ₢AkAAZ - n

Refresh GCB pins: skopeo digest updated.

### 2026-03-07 20:16 - ₢AkAAZ - n

Fix ark-lifecycle test fixture: replace deleted trbim-macos vessel with rbev-busybox.

### 2026-03-07 20:11 - ₢AkAAi - W

Added zrbf_quota_preflight() to rbf_Foundry.sh -- queries Service Usage v1beta1 consumer quota API for concurrent_private_pool_build_cpus, extracts vCPUs from RBRR_GCB_MACHINE_TYPE, computes max concurrent builds, compares against RBRR_GCB_MIN_CONCURRENT_BUILDS. Wired as hard gate in rbf_build (dies before trigger dispatch) and soft advisory in rbf_list (warns after listing). Updated rbgm_quota_build guide and RBSQB-quota_build.adoc spec to reference the min concurrent builds regime variable and document the automated preflight.

### 2026-03-07 20:11 - ₢AkAAY - W

Extracted RBRG (GCB image pins) and RBRM (Podman VM supply chain) regimes from RBRR. Created env files, regime modules with kindle/validate/render, spec docs (RBSRG/RBSRM), rewrote pin refresh as BCG-compliant full-file rewrite, updated all consumers and sourcing chains, cleaned stale definitions from RBSRR spec doc.

### 2026-03-07 20:10 - ₢AkAAd - W

Added test invocation vocabulary to BUS0-BashUtilitiesSpec.adoc: 5 new sections (invocation tiers, capture globals, case boundary, fact-file pattern, suite semantics clarification) with 13 linked terms across 5 new MCM categories (busi, busit_, busig_, busx_, busff_). No reference to retired dispatch tier.

### 2026-03-07 20:10 - ₢AkAAZ - W

Deleted single-arch build pipeline (4 scripts) and trbim-macos test vessel. Renamed rbgjbm03-09 to rbgjb03-09 (dropped m infix). Collapsed stitch function from dual-pipeline if/else to single pipeline, removing ~50 lines of duplicated jq. Fixed stale binfmt step number in RBSCB lens. Also slated ₢AkAAl for BCG guidance on test-and-break idiom spotted during work.

### 2026-03-07 20:10 - ₢AkAAi - n

Add automated quota preflight check for concurrent_private_pool_build_cpus. Hard gate in rbf_build dies before dispatch if quota insufficient; soft advisory in rbf_list warns after listing. Updated rbgm_quota_build guide and RBSQB spec to reference RBRR_GCB_MIN_CONCURRENT_BUILDS.

### 2026-03-07 20:09 - ₢AkAAd - n

Add test invocation vocabulary to BUS0: 5 new sections (invocation tiers, capture globals, case boundary, fact-file pattern, suite semantics clarification) with 13 linked terms across 5 categories (busi, busit_, busig_, busx_, busff_).

### 2026-03-07 20:08 - ₢AkAAZ - n

Delete single-arch build pipeline (4 scripts) and trbim-macos test vessel (only consumer). Rename rbgjbm03-09 to rbgjb03-09 (drop m infix). Collapse stitch function dual-pipeline branching to single pipeline. Fix stale binfmt step number in RBSCB lens.

### 2026-03-07 20:05 - Heat - S

bcg-ban-test-and-break-idiom

### 2026-03-07 20:01 - Heat - T

rationalize-build-step-naming

### 2026-03-07 19:53 - ₢AkAAi - A

Study Service Usage API patterns in rbgd/rbgu, implement shared quota check in rbf_Foundry.sh, wire hard gate in rbf_build + soft advisory in rbf_list with buc_tabtarget hints, review/update rbgm_quota_build guide.

### 2026-03-07 19:51 - ₢AkAAY - n

Remove stale RBRM_/RBRG_ variable definitions from RBSRR spec doc — these now live in RBSRG and RBSRM. Update build_configuration group description to cross-reference the new regimes.

### 2026-03-07 19:51 - ₢AkAAZ - A

Proceed despite AkAAK not done — double-rename cost accepted. Survey step inventory, map constraints, design convention (present options), execute renames.

### 2026-03-07 19:49 - ₢AkAAd - A

Read BUS0, buto_operations.sh, bute_engine.sh, fixture examples. Add 5 sections: invocation tiers, capture globals, case boundary, fact-file pattern, suite semantics. MCM linked terms throughout.

### 2026-03-07 19:45 - ₢AkAAf - W

Moved buv_lock into kindle for all 11 regimes (RBRN already done + 10 remaining: BURD, RBRR, RBRO, RBRA, RBRP, RBRS, RBRV, BURC, BURS, BURE). Deleted all zXXX_lock() functions and ~48 caller sites across 22 files. Stripped readonly from rbrr.env (26 vars), fixed 5 grep/sed sites in rbrr_cli.sh, fixed rbgp_Payor.sh copy-paste output. Fixed test infrastructure: bute_init_evidence uses BUT_TEMP_DIR instead of locked BURD_TEMP_DIR, added BURD_/BURV_ environment scrubbing in bute_dispatch via compgen+case pattern so inner tabtargets run exactly as configured. Fixed RBRO render readonly conflict with direct redacted output. Restrung ₢AkAAO and ₢AkAAR to new stabled heat ₣Aq (rbk-post-release-ideas).

### 2026-03-07 19:45 - ₢AkAAc - W

Retired dispatch tier from test engine. Converted regime-smoke (7 cases) and regime-credentials (3 cases) fixtures from bute_dispatch boilerplate to buto_tt_expect_ok. Deleted butcde_DispatchExercise.sh and its enrollment. Stripped 130 lines of dispatch/evidence machinery from bute_engine.sh, leaving only zbute_tcase. Fast sweep green: 4 fixtures, 57 cases.

### 2026-03-07 19:44 - ₢AkAAY - n

Extract RBRG and RBRM regimes from RBRR

### 2026-03-07 19:36 - ₢AkAAc - n

Delete butcde_DispatchExercise.sh (dispatch exercise test fixture, no longer needed).

### 2026-03-07 19:35 - ₢AkAAc - n

Retire dispatch tier from test engine. Convert regime-smoke and regime-credentials fixtures to buto_tt_expect_ok, delete butcde_DispatchExercise.sh and its enrollment, strip 130 lines of dispatch/evidence machinery from bute_engine.sh. Fast sweep passes: 4 fixtures, 57 cases.

### 2026-03-07 19:28 - ₢AkAAf - n

Fix RBRO render readonly conflict: replace variable mutation masking with direct redacted output, no longer calls buv_render for secret fields.

### 2026-03-07 19:27 - ₢AkAAc - A

Convert 2 regime test fixtures to buto_tt_expect_ok, delete DispatchExercise, strip dispatch tier from bute_engine.sh, verify tests pass.

### 2026-03-07 19:26 - ₢AkAAY - A

Sequential sonnet. Create RBRG/RBRM regime files, rename variables, create regime modules and spec docs, update RBSRR/RBS0.

### 2026-03-07 19:25 - ₢AkAAA - W

Introduced litmus predicates and baste functions to test framework, replacing unused init/tsuite_setup vocabulary. Renamed init→litmus and tsuite_setup→baste across butr_registry.sh, butd_dispatch.sh, rbtb_testbench.sh, and BCG. Defined 3 composable litmus predicates (container runtime, clean git, composite). Wired litmus into 5 container-dependent fixture enrollments. Removed git-clean checks from ark/slsa baste (litmus handles it). Consolidated 5 no-op baste functions into single reusable zrbtb_noop_baste. Fast sweep passes: 5 fixtures, 60 cases.

### 2026-03-07 19:23 - ₢AkAAf - n

Fix test infrastructure for lock-in-kindle: bute_init_evidence uses BUT_TEMP_DIR instead of locked BURD_TEMP_DIR, remove BURD_TEMP_DIR mutation from 3 test init functions, scrub inherited BURD_/BURV_ from environment in bute_dispatch so inner tabtargets run exactly as configured.

### 2026-03-07 19:20 - Heat - T

extract-rbrg-rbrm-from-rbrr

### 2026-03-07 19:19 - Heat - T

extract-gcb-pins-config-regime

### 2026-03-07 19:07 - Heat - T

extract-gcb-pins-config-regime

### 2026-03-07 18:51 - ₢AkAAA - n

Introduce litmus predicates and baste functions to test framework. Rename init→litmus and tsuite_setup→baste across registry, dispatch, testbench, and BCG. Define 3 composable litmus predicates (container runtime, clean git, composite). Wire litmus into 5 container-dependent fixture enrollments. Remove git-clean checks from ark/slsa baste (litmus handles it). Consolidate 5 no-op baste functions into single reusable zrbtb_noop_baste.

### 2026-03-07 18:51 - ₢AkAAf - n

Move buv_lock into kindle for all 10 remaining regimes (BURD, RBRR, RBRO, RBRA, RBRP, RBRS, RBRV, BURC, BURS, BURE), delete all zXXX_lock() functions, remove ~48 caller sites across 22 consumer files. RBRR docker env construction also moved into kindle.

### 2026-03-07 18:46 - ₢AkAAA - A

Rename init→litmus and tsuite_setup→baste across butr_registry.sh, butd_dispatch.sh, rbtb_testbench.sh, BCG. Define 3 litmus predicates, wire into fixture enrollment, remove git-clean from ark/slsa baste. Sequential sonnet — interdependent renames across 4 files.

### 2026-03-07 18:45 - Heat - S

nameplate-env-file-location

### 2026-03-07 18:41 - Heat - T

suite-infrastructure-preconditions

### 2026-03-07 18:31 - ₢AkAAf - n

Strip readonly prefix from all 26 rbrr.env variables, fix 5 grep/sed sites in rbrr_cli.sh to not match/emit readonly, fix 5 copy-paste help lines in rbgp_Payor.sh. All 6 passing test fixtures remain green; RBRO readonly failure is pre-existing.

### 2026-03-07 18:19 - ₢AkAAj - W

Changed all image tags from inscribe-only to full consecration (TAG_BASE). Updated 5 multi-platform step templates (rbgjbm04/05/06/07/09) and 3 single-platform step templates (rbgjb03/04/05) to read TAG_BASE from .tag_base. Added explicit docker push to single-platform step 03 since images: field removed. Removed images: array from both stitch paths in rbf_Foundry.sh (TAG_BASE is runtime-derived, not a CB substitution). Updated 4 consumer functions: rbf_summon, rbf_abjure, rbf_trigger_build, and vouch to use full consecration for image tag lookup. rbob_bottle.sh needed no changes — already uses RBRN_*_CONSECRATION directly.

### 2026-03-07 18:19 - ₢AkAAj - n

Read representative vessel cloudbuild.json to understand TAG_BASE flow, fix steps 04/05/09 across all 7 vessel files to use TAG_BASE instead of _RBGY_INSCRIBE_TIMESTAMP, verify stitch function and rbob_bottle.sh assumptions

### 2026-03-07 18:03 - ₢AkAAj - A

Read representative vessel cloudbuild.json to understand TAG_BASE flow, fix steps 04/05/09 across all 7 vessel files to use TAG_BASE instead of _RBGY_INSCRIBE_TIMESTAMP, verify stitch function and rbob_bottle.sh assumptions

### 2026-03-07 18:02 - ₢AkAAf - A

Start with readonly cleanup (rbrr.env, rbrr_cli.sh, rbgp_Payor.sh), then RBRR regime transform, then 9 simple regimes, then RBRO render conflict last

### 2026-03-07 13:51 - ₢AkAAg - W

Rebuilt all 4 vessel images (1 sentry + 3 bottles) on demo1025 depot via Cloud Build. Updated 3 nameplate consecrations (nsproto, srjcl, pluml). Fixed rbrn_preflight readonly regression from lock-in-kindle work using bash -c isolation. All 3 nameplate tests pass: nsproto-security (22 cases), srjcl-jupyter (3), pluml-diagram (5). Discovered and slated two issues: consumer image tag drops build timestamp (₢AkAAj), and concurrent build quota not checked (₢AkAAi).

### 2026-03-07 13:49 - ₢AkAAg - n

Rebuild all vessel images on demo1025 depot, update 3 nameplate consecrations to new inscribe timestamps, fix rbrn_preflight readonly regression (bash -c isolation). All 3 nameplate tests pass: nsproto-security (22), srjcl-jupyter (3), pluml-diagram (5).

### 2026-03-07 13:44 - Heat - S

fix-consumer-image-tag-uses-full-consecration

### 2026-03-07 13:34 - Heat - T

quota-preflight-concurrent-builds

### 2026-03-07 13:26 - Heat - S

quota-preflight-concurrent-builds

### 2026-03-07 12:52 - ₢AkAAg - A

Check vessel state, rebuild/re-summon nsproto+srjcl+pluml vessels, verify all 3 nameplate fixtures pass

### 2026-03-07 12:51 - ₢AkAAh - W

Created RBDC (Derived Constants) module to hold credential file path constants. Moved 4 path derivations (GOVERNOR/RETRIEVER/DIRECTOR RBRA_FILE, PAYOR_RBRO_FILE) from rbrr_regime lock into new rbdc_DerivedConstants.sh with zrbdc_kindle/zrbdc_sentinel. Renamed 57 consumer references across 11 files, wired source+kindle into 15 cli/test files. All functional tests pass.

### 2026-03-07 12:50 - ₢AkAAh - n

Extract RBDC derived constants module: credential file paths moved from rbrr_regime lock to new rbdc_DerivedConstants.sh, renamed 57 refs across 11 consumer files, wired source+kindle into 15 cli/test files

### 2026-03-07 12:34 - ₢AkAAh - A

Create rbdc_DerivedConstants.sh, remove 4 path derivations from rbrr lock, rename ~40 consumer sites RBRR_*→RBDC_*, wire into cli furnish functions, update CLAUDE.md

### 2026-03-07 12:33 - Heat - S

extract-rbdc-derived-constants

### 2026-03-07 12:19 - Heat - S

rebuild-vessels-fix-nameplate-tests

### 2026-03-07 12:18 - Heat - T

regime-lock-in-kindle-antipattern

### 2026-03-07 11:53 - ₢AkAAf - n

RBRN regime: move buv_lock into kindle, move buv_docker_env into enforce, delete zrbrn_lock function, remove 3 caller sites

### 2026-03-07 11:47 - ₢AkAAf - A

Survey lock functions, fix RBRN first (most coverage), validate with tests, then apply to remaining regimes (RBRR, RBRA, BURD, BURS). RBRO already done.

### 2026-03-07 11:45 - Heat - T

regime-lock-in-kindle-antipattern

### 2026-03-06 13:53 - Heat - S

regime-lock-in-kindle-antipattern

### 2026-03-06 13:47 - Heat - S

regime-lock-in-kindle-antipattern

### 2026-03-06 09:09 - Heat - S

bus0-test-invocation-vocabulary

### 2026-03-06 09:09 - Heat - S

retire-dispatch-tier-evidence-machinery

### 2026-03-05 16:19 - Heat - D

AlAAF → ₢AkAAb

### 2026-03-05 16:19 - Heat - D

AlAAD → ₢AkAAa

### 2026-03-05 15:40 - Heat - S

rationalize-build-step-naming

### 2026-03-05 13:33 - Heat - S

extract-gcb-pins-config-regime

### 2026-03-04 20:23 - Heat - T

evict-http-legacy-and-evaluate-capture-unification

### 2026-03-04 20:23 - Heat - T

evict-legacy-http-capture-functions

### 2026-03-04 19:59 - Heat - T

bottle-start-auto-summon-missing

### 2026-03-04 19:59 - Heat - T

muse-retriever-list-consecrations

### 2026-03-04 19:41 - Heat - S

muse-retriever-list-consecrations

### 2026-03-04 19:09 - ₢AkAAW - W

Renamed buc_next to buc_tabtarget (5 files, 10 sites) and replaced raw rbgp_depot_list in payor_refresh step 3 with buc_tabtarget hint

### 2026-03-04 19:08 - Heat - n

Rename buc_next to buc_tabtarget across all call sites; replace raw rbgp_depot_list in payor_refresh step 3 with buc_tabtarget hint

### 2026-03-04 18:53 - Heat - T

rename-buc-next-and-fix-payor-refresh

### 2026-03-04 18:52 - Heat - T

payor-refresh-use-tabtarget

### 2026-03-04 18:50 - Heat - S

payor-refresh-use-tabtarget

### 2026-03-04 18:18 - Heat - S

design-lro-polling-as-remit

### 2026-03-04 18:18 - Heat - S

investigate-or-true-suppression-patterns

### 2026-03-04 18:18 - Heat - T

evict-legacy-http-capture-functions

### 2026-03-04 18:17 - Heat - T

migrate-rbgu-callers-to-remit

### 2026-03-04 18:07 - Heat - T

evict-legacy-http-capture-functions

### 2026-03-04 18:06 - Heat - T

migrate-rbgu-callers-to-remit

### 2026-03-04 18:03 - Heat - S

evict-legacy-http-capture-functions

### 2026-03-04 18:03 - Heat - S

migrate-rbgu-callers-to-remit

### 2026-03-04 15:02 - Heat - T

shellcheck-bcg-exemption-catalogue

### 2026-03-04 15:01 - Heat - T

regime-variable-completeness-check

### 2026-03-04 15:01 - Heat - T

premise-headerdoc-inscription

### 2026-03-04 15:00 - Heat - T

rbs0-release-compliance-premises

### 2026-03-04 14:46 - Heat - S

premise-headerdoc-inscription

### 2026-03-04 14:45 - Heat - T

shellcheck-bcg-exemption-catalogue

### 2026-03-04 14:44 - Heat - T

regime-variable-completeness-check

### 2026-03-04 14:41 - Heat - T

rbs0-release-compliance-premises

### 2026-03-04 14:29 - Heat - S

shellcheck-bcg-exemption-catalogue

### 2026-03-04 14:28 - Heat - S

regime-variable-completeness-check

### 2026-03-04 14:28 - Heat - S

rbs0-release-compliance-premises

### 2026-03-03 13:23 - Heat - S

rename-bcg-document-identity

### 2026-03-02 07:13 - Heat - D

AiAAs → ₢AkAAM

### 2026-03-02 07:13 - Heat - D

AiAAm → ₢AkAAL

### 2026-03-02 07:13 - Heat - D

AiAAh → ₢AkAAK

### 2026-02-25 19:08 - ₢AkAAF - W

Readonly KINDLED sentinels across 45 modules, lowercase z_prefix_name convention for mutable kindle state, BCG documentation, zbuv_reset_enrollment for tests, missing readonly on 25 constants

### 2026-02-25 19:08 - ₢AkAAF - W

Readonly KINDLED sentinels across 45 modules, lowercase z_prefix_name convention for mutable kindle state, BCG documentation, zbuv_reset_enrollment for tests, missing readonly on 25 constants

### 2026-02-25 19:08 - ₢AkAAF - n

Evolve rubric_inscribe to batch-all-vessels with auto-refresh pins: process all conjure vessels in one pass, rate-limit pin refresh to 1/day instead of failing on stale, single rubric repo commit covers all vessels

### 2026-02-25 19:08 - ₢AkAAF - n

Mutable kindle state naming: readonly KINDLED sentinels, lowercase z_prefix_name for mutable state, BCG convention, reset_enrollment for tests, missing readonly on ~25 constants

### 2026-02-25 18:46 - ₢AkAAF - A

lowercase z_prefix_name for mutable kindle state; add missing readonly to ~25 vars; document in BCG

### 2026-02-25 18:41 - Heat - r

moved AkAAF to first

### 2026-02-25 18:35 - ₢AkAAC - W

Reworked BUK test registry: renamed suite→fixture enrollment, added sweep suites (fast/service/complete) with N:M case mapping, added butd_run_sweep dispatch, categorized 98 cases, fixed buv readonly kindle collision

### 2026-02-25 18:35 - ₢AkAAC - n

Rename suite→fixture, add sweep suites (fast/service/complete), sweep dispatch, categorize ~99 cases

### 2026-02-25 18:05 - ₢AkAAC - A

Rename suite→fixture, add sweep suites (fast/service/complete), sweep dispatch, categorize ~99 cases

### 2026-02-25 18:02 - Heat - r

moved AkAAC to first

### 2026-02-24 16:51 - Heat - n

Add RBRP_OPERATOR_EMAIL to payor regime config

### 2026-02-24 16:50 - Heat - n

Fix DevConnect secret manager grant in depot_create, document GitHub OAuth flow steps in depot_initialize guide and spec

### 2026-02-24 16:32 - Heat - S

align-rbs0-depot-init-changes

### 2026-02-24 16:32 - Heat - n

Operator email discovery via userinfo, Developer Connect console grant in depot_initialize, rbgi_IAM sourced in rbgm_cli

### 2026-02-24 16:14 - Heat - n

governor_reset: propagation delay, RBRR-based depot ID, copy-pasteable RBRA install command

### 2026-02-24 16:07 - Heat - n

depot_initialize: fix OAuth re-source crash, governor_reset uses RBRR_DEPOT_PROJECT_ID, fix depot_create next-step guidance, update specs

### 2026-02-24 15:54 - Heat - n

Depot initialize: fix bug_tc crash, expand rubric repo setup as GitHub reference procedure, fix stale final validation to use git ls-remote

### 2026-02-24 15:30 - Heat - T

rbrr-load-helper

### 2026-02-24 15:30 - Heat - S

rbrr-load-helper

### 2026-02-24 15:29 - ₢AkAAH - W

RBRR_SECRETS_DIR consolidation: regime restructure with kindle constants, RBRO rewrite to use RBRR_PAYOR_RBRO_FILE, RBCC cleanup, consumer and spec updates, smoke tests pass

### 2026-02-24 14:58 - Heat - T

rename-rbw-to-rbk-move-lenses

### 2026-02-24 14:23 - ₢AkAAH - n

RBRR_SECRETS_DIR consolidation: regime restructure, RBRO rewrite, RBCC cleanup, consumer+spec updates

### 2026-02-24 14:16 - ₢AkAAH - A

Sequential: regime restructure, RBRO rewrite, RBCC cleanup, consumer+spec updates, manual credential copy

### 2026-02-24 14:16 - ₢AkAAG - W

Created RBBC bootstrap constants, added BURD_CONFIG_DIR to launcher/dispatch/regime, git mv regime files to .rbk/, renamed RBCC→RBBC consumers across 20+ files, smoke tests pass

### 2026-02-24 14:15 - ₢AkAAG - n

RBBC bootstrap constants, BURD_CONFIG_DIR, git mv to .rbk/, consumer renames RBCC→RBBC across 20 files

### 2026-02-24 14:09 - ₢AkAAG - A

Mechanical: git mv to .rbk/, create RBBC, update launcher BURD_CONFIG_DIR, consumer renames across 15+ files

### 2026-02-24 14:03 - Heat - T

secrets-dir-credential-consolidation

### 2026-02-24 14:03 - Heat - T

bootstrap-constants-secrets-consolidation

### 2026-02-24 13:52 - Heat - S

secrets-dir-credential-consolidation

### 2026-02-24 13:52 - Heat - T

bootstrap-constants-secrets-consolidation

### 2026-02-24 13:48 - Heat - T

bootstrap-constants-secrets-consolidation

### 2026-02-24 13:30 - Heat - T

bootstrap-constants-secrets-consolidation

### 2026-02-24 13:26 - Heat - T

bootstrap-constants-secrets-consolidation

### 2026-02-24 13:20 - ₢AkAAG - A

Sequential: BUBC+RBBC creation, RBRR_SECRETS_DIR enrollment, consumer rename RBRR→RBBC across 15+ files

### 2026-02-24 13:18 - Heat - T

bootstrap-constants-secrets-consolidation

### 2026-02-24 13:18 - Heat - T

consolidate-secrets-dir

### 2026-02-24 12:48 - ₢AkAAG - A

Sequential sonnet: buv_dir_enroll RBRR_SECRETS_DIR, kindle constants for 4 credential paths, eliminate hardcoded ~/.rbw

### 2026-02-24 12:47 - Heat - r

moved AkAAG before AkAAB

### 2026-02-24 12:47 - Heat - S

consolidate-secrets-dir

### 2026-02-24 11:52 - ₢AkAAE - W

Widespread readonly applied: all regime lock functions + caller updates, all module kindle constants across rbw/buk. Work landed via concurrent officium commits.

### 2026-02-24 11:28 - ₢AkAAE - A

Bridleable sonnet: mechanical readonly application across all regimes and modules following AkAAD patterns

### 2026-02-24 11:27 - ₢AkAAD - W

BCG readonly patterns (local -r default, every kindle constant, lock-after-enforce archetype), buv unset detection, buv_lock, removed rbrr kindle defaults, zrbrr_lock with DOCKER_ENV post-enforce, trial on buv+rbi_Image, slated follow-on for kindle mutable state naming

### 2026-02-24 11:24 - Heat - S

kindle-mutable-state-naming

### 2026-02-24 11:12 - ₢AkAAD - n

BCG: strengthen readonly rule to every kindle constant; apply readonly to all rbi_Image.sh kindle vars

### 2026-02-24 11:05 - ₢AkAAD - n

buv unset detection, remove kindle defaults, buv_lock/zrbrr_lock with DOCKER_ENV after enforce, all 16 call sites updated

### 2026-02-24 11:00 - ₢AkAAD - n

BCG: add Readonly Patterns section (local -r default, lock-after-enforce archetype, re-source trap) and update Regime Module Archetype with lock step

### 2026-02-24 10:46 - ₢AkAAD - A

Sequential sonnet: BCG readonly section, remove kindle defaults, buv unset detection, post-enforce readonly+DOCKER_ENV move, trial targets

### 2026-02-24 10:39 - Heat - r

moved AkAAB after AkAAE

### 2026-02-24 08:47 - Heat - n

Spook fix: groom now shows full dockets by default (--detail --remaining)

### 2026-02-24 08:45 - Heat - r

moved AkAAB to first

### 2026-02-24 08:43 - Heat - S

readonly-widespread

### 2026-02-24 08:43 - Heat - S

readonly-bcg-trial

### 2026-02-24 08:43 - Heat - S

add-test-sweep-suites

### 2026-02-24 08:23 - Heat - r

moved AkAAB to first

### 2026-02-24 08:21 - Heat - S

rename-rbw-to-rbk-move-lenses

### 2026-02-24 08:21 - Heat - f

silks=rbk-mvp-finalization

### 2026-02-24 07:24 - Heat - f

racing

### 2026-02-24 07:22 - Heat - D

restring 1 paces from ₣AP

### 2026-02-24 07:22 - Heat - N

rbk-mvp-verification

