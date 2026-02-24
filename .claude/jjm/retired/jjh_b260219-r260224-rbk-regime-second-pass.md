# Heat Trophy: rbk-regime-second-pass

**Firemark:** ₣Af
**Created:** 260219
**Retired:** 260224
**Status:** retired

## Paddock

# Regime Second-Pass Inventory

Complete regime list with per-regime outcomes from the second-pass transformation.
Each regime annotated with AXLA cardinality: Singleton (one assignment source) or Manifold (multiple instances).

All regimes now use buv enrollment infrastructure: `buv_*_enroll` for field contracts, `buv_scope_sentinel` for stray detection, `z*_enforce` (buv_vet + custom checks) as ironclad gate in furnish, and `buv_report`/`buv_render` for CLI validate/render commands.

## BUK Domain

### BURC — Configuration Regime [Singleton]
- Spec: inline in BUS0
- Module: burc_regime.sh / burc_cli.sh
- Scrubbed: ₢AfAAi — enrollment, enforce, scope_sentinel

### BURS — Station Regime [Singleton]
- Spec: inline in BUS0
- Module: burs_regime.sh / burs_cli.sh
- Scrubbed: ₢AfAAf — enrollment, enforce, scope_sentinel

### BURD — Dispatch Runtime [Singleton]
- Spec: BUSD-DispatchRuntime.adoc (included from BUS0)
- Module: burd_regime.sh (runtime-only, no CLI — by design)
- Scrubbed: ₢AfAAc — enrollment applied to runtime variables

### BURE — Environment Regime [Singleton]
- Spec: inline in BUS0
- Module: bure_regime.sh / bure_cli.sh
- Scrubbed: ₢AfAAj — enrollment for VERBOSE, COLOR, COUNTDOWN; ambient vars relocated from BURD (₢AfAAB)

## RBW Domain

### RBRR — Repository Regime [Singleton]
- Spec: RBSRR-RegimeRepo.adoc (included from RBS0)
- Module: rbrr_regime.sh / rbrr_cli.sh
- Scrubbed: ₢AfAAC — exemplar singleton; enrollment, enforce, scope_sentinel, validate, render

### RBRN — Nameplate Regime [Manifold: nsproto, srjcl, pluml]
- Spec: RBRN-RegimeNameplate.adoc (included from RBS0)
- Module: rbrn_regime.sh / rbrn_cli.sh
- Scrubbed: ₢AfAAE — exemplar manifold; enrollment, enforce, differential furnish with folio-conditional kindle

### RBRP — Payor Regime [Singleton]
- Spec: RBSRP-RegimePayor.adoc (included from RBS0)
- Module: rbrp_regime.sh / rbrp_cli.sh
- Scrubbed: ₢AfAAh — enrollment, enforce, grep -qE replaced

### RBRO — OAuth Regime [Singleton]
- Spec: RBSRO-RegimeOauth.adoc (included from RBS0)
- Module: rbro_regime.sh / rbro_cli.sh
- Scrubbed: ₢AfAAO — new module created (previously spec-only), enrollment, enforce, consumers validated

### ~~RBRE — ECR Regime [Singleton]~~ REMOVED
- Removed: ₢AfAAo — ghost regime (spec'd, never implemented), spec and linked terms deleted

### ~~RBRG — GitHub Regime [Singleton]~~ REMOVED
- Removed: ₢AfAAA — ghost regime (spec'd, never implemented), consumers migrated

### RBRS — Station Regime [Singleton]
- Spec: RBSRS-RegimeStation.adoc (included from RBS0)
- Module: rbrs_regime.sh / rbrs_cli.sh
- Scrubbed: ₢AfAAe — enrollment, enforce

### RBRV — Vessel Regime [Manifold: 6 vessels in rbev-vessels/]
- Spec: RBSRV-RegimeVessel.adoc (included from RBS0)
- Module: rbrv_regime.sh / rbrv_cli.sh
- Scrubbed: ₢AfAAI — structural rebuild; enrollment, enforce, buc_execute, differential furnish

### RBRA — Authentication/Credential Regime [Manifold: governor, retriever, director]
- Spec: RBSRA-CredentialFormat.adoc (included from RBS0)
- Module: rbra_regime.sh / rbra_cli.sh
- Scrubbed: ₢AfAAk — enrollment, enforce, grep -qE replaced

## Workbench Ad-Hoc Imprint Translation — RESOLVED

The rbw workbench ad-hoc case arms for `BURD_TOKEN_3` → `RBOB_MONIKER` translation have been eliminated by the buz channel mechanism (₢AfAAD) and RBRN scrub (₢AfAAE). All bottle colophons now route through `zbuz_exec_lookup`.

## Cross-Cutting Concerns — RESOLVED

All cross-cutting concerns from the initial audit have been resolved by the enrollment transformation:

- ~~Unquoted ${#ARRAY[@]}~~ — eliminated; buv_scope_sentinel handles array safety internally
- ~~`[[ == ]]` misuse~~ — replaced with `[[ =~ ]]` or `test =` in all regime scrubs
- ~~`grep -qE` misuse~~ — replaced with `[[ =~ ]]` in RBRP and RBRA scrubs
- ~~RBRO has spec but no implementation~~ — rbro_regime.sh created (₢AfAAO)
- ~~BURE missing from regime census~~ — added and scrubbed (₢AfAAj)
- ~~`rbrn_load_moniker` consumers~~ — eliminated; RBRN uses standard enrollment pattern
- ~~Spec-validator drift~~ — enrollment declarations are the single source of truth
- CRR glossary documents — not created (out of scope for this heat)

## Infrastructure Paces

Key non-regime paces that enabled the transformation:

- ₢AfAAN — buv enrollment infrastructure (buv_*_enroll, buv_vet, buv_report, buv_scope_sentinel)
- ₢AfAAD — buz channel infrastructure (imprint translation)
- ₢AfAAQ — buv enrollment tests
- ₢AfAAO — audit legacy buv callers
- ₢AfAAP — delete legacy buv_val_*/buv_env_*/buv_opt_* functions
- ₢AfAAJ — BCG updates (regime archetype section, enforce boilerplate, stale buv references removed)

## Paces

### decide-zbuv-check-error-pattern (₢AfAAn) [complete]

**[260223-0735] complete**

DECISION NEEDED: ZBUV_CHECK_ERROR naming and BCG function taxonomy.

## Issue

zbuv_check_predicate in buv_validation.sh returns 0/1 (predicate) but also sets ZBUV_CHECK_ERROR as a side-channel detail string. ZBUV_CHECK_ERROR uses kindle constant naming (Z«PREFIX»_SCREAMING) but is assigned in ~40 branches inside zbuv_check_predicate, not in kindle.

## Options identified

1. **Simple rename** — z_zbuv_check_predicate_error (enroll return-var convention, though this isn't enroll)
2. **Convert to capture** — return error on stdout, lose predicate-in-conditional usage
3. **New BCG pattern** — "predicate with detail" is a legitimate taxonomy gap; define naming convention for it
4. **Accept as-is** — document as an intentional exception

## Context

- All ~40 assignments are inside one function (case branches), not scattered
- Consumers: buv_vet (line 613) and buv_report (lines 639, 645)
- Similar pattern may exist elsewhere: ZBUV_GRP_GATE_VAR/ZBUV_GRP_GATE_VAL in zbuv_group_gate_recite

## Acceptance

- Explicit decision recorded
- If BCG update: new pattern documented
- If rename: mechanical change applied

**[260222-1632] rough**

DECISION NEEDED: ZBUV_CHECK_ERROR naming and BCG function taxonomy.

## Issue

zbuv_check_predicate in buv_validation.sh returns 0/1 (predicate) but also sets ZBUV_CHECK_ERROR as a side-channel detail string. ZBUV_CHECK_ERROR uses kindle constant naming (Z«PREFIX»_SCREAMING) but is assigned in ~40 branches inside zbuv_check_predicate, not in kindle.

## Options identified

1. **Simple rename** — z_zbuv_check_predicate_error (enroll return-var convention, though this isn't enroll)
2. **Convert to capture** — return error on stdout, lose predicate-in-conditional usage
3. **New BCG pattern** — "predicate with detail" is a legitimate taxonomy gap; define naming convention for it
4. **Accept as-is** — document as an intentional exception

## Context

- All ~40 assignments are inside one function (case branches), not scattered
- Consumers: buv_vet (line 613) and buv_report (lines 639, 645)
- Similar pattern may exist elsewhere: ZBUV_GRP_GATE_VAR/ZBUV_GRP_GATE_VAL in zbuv_group_gate_recite

## Acceptance

- Explicit decision recorded
- If BCG update: new pattern documented
- If rename: mechanical change applied

### fix-rbrp-load-callers (₢AfAAm) [complete]

**[260223-0753] complete**

Fix 3 callers of removed rbrp_load() function.

## Context

₢AfAAh (scrub-rbrp-singleton) removed rbrp_load() from rbrp_regime.sh during enrollment migration but left 3 callers behind. These CLIs will die at runtime when they try to call the missing function.

## BLOCKER NOTE

₢AiAAS on ₣Ai (gcb-trigger-migration-tier2) is blocked by this fix. That pace needs rbw-gpe.PayorEstablishment.sh and rbw-gge.GdcEstablishment.sh to work, which both route through rbgp_cli.sh and rbgm_cli.sh — the broken callers.

## Callers to fix

- Tools/rbw/rbgp_cli.sh:49
- Tools/rbw/rbgm_cli.sh:49
- Tools/rbw/rbts/rbtcap_AccessProbe.sh:96

## Pattern

Replace rbrp_load with the standard regime archetype furnish sequence: source the regime file, then call zrbrp_kindle + zrbrp_enforce. Follow the pattern established in rbrp_cli.sh furnish.

## Acceptance

- No references to rbrp_load remain in codebase
- All 3 callers use kindle+enforce pattern
- Regime smoke tests pass
- Notify ₣Ai engineer that blocker is cleared

**[260222-1637] rough**

Fix 3 callers of removed rbrp_load() function.

## Context

₢AfAAh (scrub-rbrp-singleton) removed rbrp_load() from rbrp_regime.sh during enrollment migration but left 3 callers behind. These CLIs will die at runtime when they try to call the missing function.

## BLOCKER NOTE

₢AiAAS on ₣Ai (gcb-trigger-migration-tier2) is blocked by this fix. That pace needs rbw-gpe.PayorEstablishment.sh and rbw-gge.GdcEstablishment.sh to work, which both route through rbgp_cli.sh and rbgm_cli.sh — the broken callers.

## Callers to fix

- Tools/rbw/rbgp_cli.sh:49
- Tools/rbw/rbgm_cli.sh:49
- Tools/rbw/rbts/rbtcap_AccessProbe.sh:96

## Pattern

Replace rbrp_load with the standard regime archetype furnish sequence: source the regime file, then call zrbrp_kindle + zrbrp_enforce. Follow the pattern established in rbrp_cli.sh furnish.

## Acceptance

- No references to rbrp_load remain in codebase
- All 3 callers use kindle+enforce pattern
- Regime smoke tests pass
- Notify ₣Ai engineer that blocker is cleared

**[260222-1630] rough**

Fix 3 callers of removed rbrp_load() function.

## Context

₢AfAAh (scrub-rbrp-singleton) removed rbrp_load() from rbrp_regime.sh during enrollment migration but left 3 callers behind. These CLIs will die at runtime when they try to call the missing function.

## Callers to fix

- Tools/rbw/rbgp_cli.sh:49
- Tools/rbw/rbgm_cli.sh:49
- Tools/rbw/rbts/rbtcap_AccessProbe.sh:96

## Pattern

Replace `rbrp_load` with the standard regime archetype furnish sequence: source the regime file, then call zrbrp_kindle + zrbrp_enforce. Follow the pattern established in rbrp_cli.sh furnish.

## Acceptance

- No references to rbrp_load remain in codebase
- All 3 callers use kindle+enforce pattern
- Regime smoke tests pass

### buv-enrollment-render-unification (₢AfAAa) [complete]

**[260222-1033] complete**

Unify regime variable enrollment and rendering so that enrollment carries description and section context, eliminating the duplicated render specifications in CLI files.

## Design

1. **buv_regime_start SCOPE** — sets current enrollment scope (read by subsequent enrolls)
2. **buv_regime_section "Title" [GATE_VAR GATE_VALUE]** — sets current section context, with optional gate for conditional sections (maps to axhrgc_gate)
3. **buv_*_enroll** calls gain a trailing description string parameter; scope is read from buv_regime_start state rather than passed as $1
4. **buv_render SCOPE "Label"** — new function that walks enrollment rolls, groups by section, applies gates, and outputs via bupr_ formatting

## Roll Changes

- New columns: z_buv_section_roll (section title), z_buv_desc_roll (description)
- Section-level gate info in a separate section registry (two-level registry with foreign-key, per BCG line 664)

## Regime Updates

- **RBRR** (singleton exemplar): convert kindle enrollments to new API, collapse rbrr_render to one-line buv_render call
- **RBRN** (manifold exemplar): convert kindle enrollments to new API, collapse rbrn_render similarly

## Acceptance

- buv_regime_start / buv_regime_section / buv_render implemented and working
- RBRR and RBRN kindles use new enrollment API
- rbrr_render and rbrn_render collapse to buv_render calls
- buv_report (validate) continues to work unchanged
- No BCG amendment needed for context-setters (they are normal kindle state, not _enroll functions)

**[260222-0855] rough**

Unify regime variable enrollment and rendering so that enrollment carries description and section context, eliminating the duplicated render specifications in CLI files.

## Design

1. **buv_regime_start SCOPE** — sets current enrollment scope (read by subsequent enrolls)
2. **buv_regime_section "Title" [GATE_VAR GATE_VALUE]** — sets current section context, with optional gate for conditional sections (maps to axhrgc_gate)
3. **buv_*_enroll** calls gain a trailing description string parameter; scope is read from buv_regime_start state rather than passed as $1
4. **buv_render SCOPE "Label"** — new function that walks enrollment rolls, groups by section, applies gates, and outputs via bupr_ formatting

## Roll Changes

- New columns: z_buv_section_roll (section title), z_buv_desc_roll (description)
- Section-level gate info in a separate section registry (two-level registry with foreign-key, per BCG line 664)

## Regime Updates

- **RBRR** (singleton exemplar): convert kindle enrollments to new API, collapse rbrr_render to one-line buv_render call
- **RBRN** (manifold exemplar): convert kindle enrollments to new API, collapse rbrn_render similarly

## Acceptance

- buv_regime_start / buv_regime_section / buv_render implemented and working
- RBRR and RBRN kindles use new enrollment API
- rbrr_render and rbrn_render collapse to buv_render calls
- buv_report (validate) continues to work unchanged
- No BCG amendment needed for context-setters (they are normal kindle state, not _enroll functions)

### rbts-test-suites (₢AfAAT) [complete]

**[260221-0637] complete**

Move all rbtc* test case files from Tools/rbw/ to Tools/rbw/rbtd/ subdirectory.

## Scope

- Create `Tools/rbw/rbtd/` directory
- Move all `rbtc*` files: rbtcap_AccessProbe.sh, rbtcal_ArkLifecycle.sh, rbtckk_KickTires.sh, rbtcqa_QualifyAll.sh, rbtcns_NsproSecurity.sh, rbtcsj_SrjclJupyter.sh, rbtcpl_PlumlDiagram.sh
- Update all `source "${RBTB_SCRIPT_DIR}/rbtc*"` lines in rbtb_testbench.sh to `source "${RBTB_SCRIPT_DIR}/rbtd/rbtc*"`
- BUK test cases (butcde_, butcrg_, butcvu_) stay in Tools/buk/ — not in scope
- Testbench itself (rbtb_testbench.sh) stays in Tools/rbw/

## Verification

- Run `./tt/rbw-ts.TestSuite.sh access-probe` to confirm test suite still passes after move
- Run `./tt/rbw-ts.TestSuite.sh kick-tires` as a quick smoke test

## NOT in scope

- Renaming any files or functions
- Moving BUK test case files
- Moving the testbench router itself

**[260221-0636] rough**

Move all rbtc* test case files from Tools/rbw/ to Tools/rbw/rbtd/ subdirectory.

## Scope

- Create `Tools/rbw/rbtd/` directory
- Move all `rbtc*` files: rbtcap_AccessProbe.sh, rbtcal_ArkLifecycle.sh, rbtckk_KickTires.sh, rbtcqa_QualifyAll.sh, rbtcns_NsproSecurity.sh, rbtcsj_SrjclJupyter.sh, rbtcpl_PlumlDiagram.sh
- Update all `source "${RBTB_SCRIPT_DIR}/rbtc*"` lines in rbtb_testbench.sh to `source "${RBTB_SCRIPT_DIR}/rbtd/rbtc*"`
- BUK test cases (butcde_, butcrg_, butcvu_) stay in Tools/buk/ — not in scope
- Testbench itself (rbtb_testbench.sh) stays in Tools/rbw/

## Verification

- Run `./tt/rbw-ts.TestSuite.sh access-probe` to confirm test suite still passes after move
- Run `./tt/rbw-ts.TestSuite.sh kick-tires` as a quick smoke test

## NOT in scope

- Renaming any files or functions
- Moving BUK test case files
- Moving the testbench router itself

**[260221-0626] rough**

Move all rbtc* test case files from Tools/rbw/ to Tools/rbw/rbtd/ subdirectory.

## Scope

- Create `Tools/rbw/rbtd/` directory
- Move all `rbtc*` files: rbtcap_AccessProbe.sh, rbtcal_ArkLifecycle.sh, rbtckk_KickTires.sh, rbtcqa_QualifyAll.sh, rbtcns_NsproSecurity.sh, rbtcsj_SrjclJupyter.sh, rbtcpl_PlumlDiagram.sh
- Update all `source "${RBTB_SCRIPT_DIR}/rbtc*"` lines in rbtb_testbench.sh to `source "${RBTB_SCRIPT_DIR}/rbtd/rbtc*"`
- BUK test cases (butcde_, butcrg_, butcvu_) stay in Tools/buk/ — not in scope
- Testbench itself (rbtb_testbench.sh) stays in Tools/rbw/

## Verification

- Run `./tt/rbw-ts.TestSuite.sh access-probe` to confirm test suite still passes after move
- Run `./tt/rbw-ts.TestSuite.sh kick-tires` as a quick smoke test

## NOT in scope

- Renaming any files or functions
- Moving BUK test case files
- Moving the testbench router itself

### access-probe-specs-and-tests (₢AfAAS) [complete]

**[260221-0637] complete**

Define two RBS0 access probe operations and implement test cases for each.

## Spec 1: JWT SA Access Probe (RBSAJ-access_jwt_probe.adoc)

Single operation parameterized by role (Governor, Director, Retriever). For each iteration:
1. Exchange JWT for OAuth token via rbgo_get_token_capture with role's RBRA file
2. Call Artifact Registry packages.list to verify token grants read access
3. Sleep for configured delay

Parameters:
- role: governor | director | retriever (selects RBRA file and display name)
- count: number of iterations (default 1)
- delay_ms: milliseconds between iterations (default 0)

Verification API: GET artifactregistry.googleapis.com/v1/projects/{id}/locations/{loc}/repositories/{repo}/packages
- All three roles have read access (Governor=Owner, Director=repoAdmin, Retriever=reader)
- Fast: single HTTP GET, ~200ms response
- Proves both token validity AND role-specific permission

Include in RBS0 under "Multi Role Operations" section.

## Spec 2: Payor OAuth Access Probe (RBSAO-access_oauth_probe.adoc)

Payor-specific operation using OAuth refresh token flow. For each iteration:
1. Authenticate via zrbgp_authenticate_capture (RBRO refresh token → access token)
2. Call CRM projects.get on payor project to verify token works
3. Sleep for configured delay

Parameters:
- count: number of iterations (default 1)
- delay_ms: milliseconds between iterations (default 0)

Verification API: GET cloudresourcemanager.googleapis.com/v1/projects/{payor_project_id}
- Exercises Payor's CRM access
- Fast: single HTTP GET

Include in RBS0 under "Payor Operations" section.

## Test Cases

Create test cases in rbtb testbench that run BEFORE the ark-lifecycle suite:
- JWT probe: 5 iterations × 3 roles (Governor, Director, Retriever), 1500ms between calls
- Payor probe: 5 iterations, 1500ms between calls
- Total runtime: ~4 roles × 5 × 1.5s ≈ 30 seconds
- Failure here catches missing RBRA files or OAuth issues before the 4-minute ark-lifecycle

These probes also serve as regression tests for the rbgo_OAuth.sh stderr-capture fix (₢AfAAR).

## Naming

Mint check:
- RBSAJ: no conflict (RBSA* namespace has AA/AB/AC/AS/AX)
- RBSAO: no conflict
- Tabtarget colophons: TBD during implementation (likely rbw-ap or similar)

## NOT in scope
- Retry/backoff logic (separate concern)
- Aggressive stress testing (this is smoke + light soak)

**[260221-0605] rough**

Define two RBS0 access probe operations and implement test cases for each.

## Spec 1: JWT SA Access Probe (RBSAJ-access_jwt_probe.adoc)

Single operation parameterized by role (Governor, Director, Retriever). For each iteration:
1. Exchange JWT for OAuth token via rbgo_get_token_capture with role's RBRA file
2. Call Artifact Registry packages.list to verify token grants read access
3. Sleep for configured delay

Parameters:
- role: governor | director | retriever (selects RBRA file and display name)
- count: number of iterations (default 1)
- delay_ms: milliseconds between iterations (default 0)

Verification API: GET artifactregistry.googleapis.com/v1/projects/{id}/locations/{loc}/repositories/{repo}/packages
- All three roles have read access (Governor=Owner, Director=repoAdmin, Retriever=reader)
- Fast: single HTTP GET, ~200ms response
- Proves both token validity AND role-specific permission

Include in RBS0 under "Multi Role Operations" section.

## Spec 2: Payor OAuth Access Probe (RBSAO-access_oauth_probe.adoc)

Payor-specific operation using OAuth refresh token flow. For each iteration:
1. Authenticate via zrbgp_authenticate_capture (RBRO refresh token → access token)
2. Call CRM projects.get on payor project to verify token works
3. Sleep for configured delay

Parameters:
- count: number of iterations (default 1)
- delay_ms: milliseconds between iterations (default 0)

Verification API: GET cloudresourcemanager.googleapis.com/v1/projects/{payor_project_id}
- Exercises Payor's CRM access
- Fast: single HTTP GET

Include in RBS0 under "Payor Operations" section.

## Test Cases

Create test cases in rbtb testbench that run BEFORE the ark-lifecycle suite:
- JWT probe: 5 iterations × 3 roles (Governor, Director, Retriever), 1500ms between calls
- Payor probe: 5 iterations, 1500ms between calls
- Total runtime: ~4 roles × 5 × 1.5s ≈ 30 seconds
- Failure here catches missing RBRA files or OAuth issues before the 4-minute ark-lifecycle

These probes also serve as regression tests for the rbgo_OAuth.sh stderr-capture fix (₢AfAAR).

## Naming

Mint check:
- RBSAJ: no conflict (RBSA* namespace has AA/AB/AC/AS/AX)
- RBSAO: no conflict
- Tabtarget colophons: TBD during implementation (likely rbw-ap or similar)

## NOT in scope
- Retry/backoff logic (separate concern)
- Aggressive stress testing (this is smoke + light soak)

### debug-ark-lifecycle-oauth-failure (₢AfAAR) [complete]

**[260221-0607] complete**

Diagnose why the ark-lifecycle test gets an empty OAuth response on the third tabtarget invocation (Step 3: post-conjure image list).

## Observed Behavior

Run: `./tt/rbw-ts.TestSuite.sh ark-lifecycle`

- Step 1 (baseline list via rbw-il): SUCCEEDS — gets valid OAuth token, lists 24 locators
- Step 2 (conjure via rbw-aC): SUCCEEDS — GCB build completes (9 polls, ~3.5 minutes)
- Step 3 (post-conjure list via rbw-il): FAILS — `rbgo_oauth_response.json` is 0 bytes, `rbf_list` dies with "Failed to get OAuth token"

The same `tt/rbw-il.ImageList.sh` works perfectly from the terminal.

## Forensic Evidence

All artifacts preserved at:
```
/Users/bhyslop/projects/temp-buk/temp-20260220-085552-33723-846/rbtcal_lifecycle_tcase/burv/
```

- `invoke-10000/` — Step 1 (list baseline) — SUCCEEDED
  - `rbgo_oauth_response.json`: valid token (ya29.c...)
  - `transcript.txt`: full successful flow

- `invoke-10001/` — Step 2 (conjure) — SUCCEEDED  
  - Full GCB build artifacts present
  - `rbgo_oauth_response.json`: valid token used for build submission

- `invoke-10002/` — Step 3 (list post-conjure) — FAILED
  - `rbgo_oauth_response.json`: 0 bytes (empty!)
  - `rbgo_jwt_claims.json`: valid claims (iat=1771606698, exp=1771608498, 30-min window)
  - `rbgo_jwt_header.json`, `rbgo_jwt_unsigned.txt`, `rbgo_jwt_signature.txt`: all present
  - `transcript.txt`: JWT build completed normally through line 40, then exchange returned empty

## Key Code Path

`Tools/rbw/rbgo_OAuth.sh`:
- `rbgo_get_token_capture()` (line 157) → `zrbgo_build_jwt_capture()` → `zrbgo_exchange_jwt_capture()`
- The curl call at line 134-137 wrote 0 bytes to the response file
- `curl -s ... 2>/dev/null` suppresses both stderr and verbose output — if curl itself failed (DNS, connection refused, timeout), we'd never see why
- The `|| return 1` on curl only fires on curl process failure, not on HTTP errors

## Investigation Leads

1. **curl suppressing the real error**: The `2>/dev/null` hides any curl diagnostic. The `-s` flag hides progress. If there was a network error, TLS issue, or DNS failure, it's invisible. Consider: did the curl process succeed (exit 0) but return empty body? Or did it fail and the empty file is from the redirect `>`?

2. **Test harness environment**: Each invocation goes through `zbuto_invoke()` in `buto_operations.sh` (line 144-155) which runs the tabtarget in a subshell with `set +e`, capturing stdout/stderr to temp files. The tabtarget itself runs through full BUD dispatch. Check whether the subshell context loses something the third time.

3. **BURD_TEMP_DIR isolation**: Each dispatch creates a unique BURD_TEMP_DIR. The kindle constants (ZRBGO_*) point to files in that temp dir. No cross-invocation file collision should occur. Verified: invoke-10002 has its own temp dir `temp-20260220-085817-35758-194`.

4. **Process substitution in subshell**: The openssl signing at line 115-118 uses `<(printf '%b' "${RBRA_PRIVATE_KEY}\n")` — process substitution. This works in bash but may behave differently in nested subshells. However, the JWT was built successfully (signature file exists), so this isn't the issue.

5. **RBRA credential sourcing in _capture function**: `zrbgo_build_jwt_capture()` sources the RBRA file (line 78). Since it runs inside `$()` (called from `rbgo_get_token_capture` line 172), the sourced RBRA_* variables die with the subshell. But the JWT is returned as stdout, then passed to `zrbgo_exchange_jwt_capture()` — so the JWT itself should be fine. The question: could the JWT string get corrupted or truncated passing through the variable?

6. **JWT size in -d argument**: The JWT is passed as part of a `-d` curl argument. If the JWT is very long, could shell argument limits be hit? Unlikely at ~1KB, but worth checking the actual size.

7. **Google rate limiting**: Three service-account JWT exchanges in ~4 minutes from the same SA. Google may throttle or reject. An HTTP 429 or 403 response would be informative — but we got 0 bytes, not an error response.

8. **Compare successful vs failing JWT**: The JWT claims look valid. But compare the actual unsigned JWT and signature between invoke-10000 and invoke-10002 to see if something is subtly different.

## Recommended First Steps

1. Re-run the test and capture curl's stderr (temporarily remove `2>/dev/null` from line 137, or add `--verbose` to curl flags)
2. Add `curl --write-out '%{http_code}'` to capture the HTTP status code
3. Compare the JWT strings between invoke-10000 and invoke-10002
4. Check if Google's OAuth endpoint returned an HTTP error with empty body

## NOT in scope

- Fixing the issue (this pace is diagnosis only)
- Changing the enrollment infrastructure or BCG docs

**[260220-0910] rough**

Diagnose why the ark-lifecycle test gets an empty OAuth response on the third tabtarget invocation (Step 3: post-conjure image list).

## Observed Behavior

Run: `./tt/rbw-ts.TestSuite.sh ark-lifecycle`

- Step 1 (baseline list via rbw-il): SUCCEEDS — gets valid OAuth token, lists 24 locators
- Step 2 (conjure via rbw-aC): SUCCEEDS — GCB build completes (9 polls, ~3.5 minutes)
- Step 3 (post-conjure list via rbw-il): FAILS — `rbgo_oauth_response.json` is 0 bytes, `rbf_list` dies with "Failed to get OAuth token"

The same `tt/rbw-il.ImageList.sh` works perfectly from the terminal.

## Forensic Evidence

All artifacts preserved at:
```
/Users/bhyslop/projects/temp-buk/temp-20260220-085552-33723-846/rbtcal_lifecycle_tcase/burv/
```

- `invoke-10000/` — Step 1 (list baseline) — SUCCEEDED
  - `rbgo_oauth_response.json`: valid token (ya29.c...)
  - `transcript.txt`: full successful flow

- `invoke-10001/` — Step 2 (conjure) — SUCCEEDED  
  - Full GCB build artifacts present
  - `rbgo_oauth_response.json`: valid token used for build submission

- `invoke-10002/` — Step 3 (list post-conjure) — FAILED
  - `rbgo_oauth_response.json`: 0 bytes (empty!)
  - `rbgo_jwt_claims.json`: valid claims (iat=1771606698, exp=1771608498, 30-min window)
  - `rbgo_jwt_header.json`, `rbgo_jwt_unsigned.txt`, `rbgo_jwt_signature.txt`: all present
  - `transcript.txt`: JWT build completed normally through line 40, then exchange returned empty

## Key Code Path

`Tools/rbw/rbgo_OAuth.sh`:
- `rbgo_get_token_capture()` (line 157) → `zrbgo_build_jwt_capture()` → `zrbgo_exchange_jwt_capture()`
- The curl call at line 134-137 wrote 0 bytes to the response file
- `curl -s ... 2>/dev/null` suppresses both stderr and verbose output — if curl itself failed (DNS, connection refused, timeout), we'd never see why
- The `|| return 1` on curl only fires on curl process failure, not on HTTP errors

## Investigation Leads

1. **curl suppressing the real error**: The `2>/dev/null` hides any curl diagnostic. The `-s` flag hides progress. If there was a network error, TLS issue, or DNS failure, it's invisible. Consider: did the curl process succeed (exit 0) but return empty body? Or did it fail and the empty file is from the redirect `>`?

2. **Test harness environment**: Each invocation goes through `zbuto_invoke()` in `buto_operations.sh` (line 144-155) which runs the tabtarget in a subshell with `set +e`, capturing stdout/stderr to temp files. The tabtarget itself runs through full BUD dispatch. Check whether the subshell context loses something the third time.

3. **BURD_TEMP_DIR isolation**: Each dispatch creates a unique BURD_TEMP_DIR. The kindle constants (ZRBGO_*) point to files in that temp dir. No cross-invocation file collision should occur. Verified: invoke-10002 has its own temp dir `temp-20260220-085817-35758-194`.

4. **Process substitution in subshell**: The openssl signing at line 115-118 uses `<(printf '%b' "${RBRA_PRIVATE_KEY}\n")` — process substitution. This works in bash but may behave differently in nested subshells. However, the JWT was built successfully (signature file exists), so this isn't the issue.

5. **RBRA credential sourcing in _capture function**: `zrbgo_build_jwt_capture()` sources the RBRA file (line 78). Since it runs inside `$()` (called from `rbgo_get_token_capture` line 172), the sourced RBRA_* variables die with the subshell. But the JWT is returned as stdout, then passed to `zrbgo_exchange_jwt_capture()` — so the JWT itself should be fine. The question: could the JWT string get corrupted or truncated passing through the variable?

6. **JWT size in -d argument**: The JWT is passed as part of a `-d` curl argument. If the JWT is very long, could shell argument limits be hit? Unlikely at ~1KB, but worth checking the actual size.

7. **Google rate limiting**: Three service-account JWT exchanges in ~4 minutes from the same SA. Google may throttle or reject. An HTTP 429 or 403 response would be informative — but we got 0 bytes, not an error response.

8. **Compare successful vs failing JWT**: The JWT claims look valid. But compare the actual unsigned JWT and signature between invoke-10000 and invoke-10002 to see if something is subtly different.

## Recommended First Steps

1. Re-run the test and capture curl's stderr (temporarily remove `2>/dev/null` from line 137, or add `--verbose` to curl flags)
2. Add `curl --write-out '%{http_code}'` to capture the HTTP status code
3. Compare the JWT strings between invoke-10000 and invoke-10002
4. Check if Google's OAuth endpoint returned an HTTP error with empty body

## NOT in scope

- Fixing the issue (this pace is diagnosis only)
- Changing the enrollment infrastructure or BCG docs

### remove-rbrg-regime (₢AfAAA) [complete]

**[260220-0758] complete**

Remove RBRG (GitHub Regime) from active codebase.

Scope:
- Remove RBRG voicings from RBSA mapping section (rbrg_prefix, rbrg_regime, rbrg_username, rbrg_pat)
- Remove the === GitHub Regime (RBRG) section from RBSA
- Remove the include::RBSRG-RegimeGithub.adoc[] directive from RBSA
- Delete lenses/RBSRG-RegimeGithub.adoc
- Remove RBRG_PAT and RBRG_USERNAME references from Tools/rbw/rbv_PodmanVM.sh
  - This includes the zrbv_kindle_github() function and its call site
  - Also the RBRG_USERNAME/RBRG_PAT usage in the GitHub registry login
- Remove rbrg_prefix from the Configuration Regimes prefix list in RBSA

DO NOT touch:
- Tools/ABANDONED-github/ — legacy references stay as-is
- Study/ directory files — historical analysis stays
- Memos/ — historical records stay

After removal, verify no live code references remain (excluding ABANDONED/Study/Memos).

**[260219-0827] rough**

Remove RBRG (GitHub Regime) from active codebase.

Scope:
- Remove RBRG voicings from RBSA mapping section (rbrg_prefix, rbrg_regime, rbrg_username, rbrg_pat)
- Remove the === GitHub Regime (RBRG) section from RBSA
- Remove the include::RBSRG-RegimeGithub.adoc[] directive from RBSA
- Delete lenses/RBSRG-RegimeGithub.adoc
- Remove RBRG_PAT and RBRG_USERNAME references from Tools/rbw/rbv_PodmanVM.sh
  - This includes the zrbv_kindle_github() function and its call site
  - Also the RBRG_USERNAME/RBRG_PAT usage in the GitHub registry login
- Remove rbrg_prefix from the Configuration Regimes prefix list in RBSA

DO NOT touch:
- Tools/ABANDONED-github/ — legacy references stay as-is
- Study/ directory files — historical analysis stays
- Memos/ — historical records stay

After removal, verify no live code references remain (excluding ABANDONED/Study/Memos).

### relocate-ambient-burd-vars-to-bure (₢AfAAB) [complete]

**[260220-0815] complete**

Relocate two ambient caller-override BURD_ variables to BURE (environment regime).

## Context

Two BURD_ variables are ambient caller preferences that can be set before dispatch.
They belong in BURE alongside BURE_COUNTDOWN, not in BURD which is dispatch infrastructure.

Three variables from the original scope do NOT move:
- BURD_NO_LOG — set by tabtarget (buut_tabtarget.sh), not caller
- BURD_INTERACTIVE — set by tabtarget (buut_tabtarget.sh), not caller
- BURD_TERM_COLS — set by launcher (bul_launcher.sh), not caller

## Variables to relocate

- BURD_VERBOSE → BURE_VERBOSE
- BURD_COLOR → BURE_COLOR

## AsciiDoc changes (MCM linked terms)

### BUSA mapping section

Remove from BURD mapping block (~line 88, 92):
- `:burd_verbose:  <<burd_verbose,BURD_VERBOSE>>`
- `:burd_color:    <<burd_color,BURD_COLOR>>`

Add to BURE mapping block (after line 121, before `// end::mapping-section[]`):
- `:bure_verbose:  <<bure_verbose,BURE_VERBOSE>>`
- `:bure_color:    <<bure_color,BURE_COLOR>>`

### BUSA BURD regime section (inline, ~line 485–513)

Remove the `[[burd_verbose]]` anchor+annotation+definition block (lines 487–491).
Remove the `[[burd_color]]` anchor+annotation+definition block (lines 508–513).

### BUSA BURE regime section (inline, ~line 631+)

Add two new anchor+annotation+definition blocks:

```
[[bure_verbose]]
// ⟦axl_voices axrg_variable axtu_integer⟧
{bure_verbose}::
Verbosity level controlling diagnostic output.
0=silent, 1=show messages, 2=enable bash trace.

[[bure_color]]
// ⟦axl_voices axrg_variable axtu_string⟧
{bure_color}::
Color output policy.
Input as auto/0/1; resolved to 0 or 1 during dispatch.
Respects NO_COLOR environment convention.
```

### BUSD-DispatchRuntime.adoc

Remove the `{burd_verbose}` variable entry (lines 14–19).
Remove the `{burd_color}` variable entry (lines 38–43).
Update the group_input description (line 12) to note that ambient overrides moved to BURE.

## Bash changes

1. Add BURE_VERBOSE and BURE_COLOR to bure_regime.sh (kindle defaults, unexpected detection, validation)
2. Rename all usages across codebase (bud_dispatch.sh, buc_command.sh, workbenches, etc.)
3. Remove old BURD_VERBOSE and BURD_COLOR from zburd unexpected variable detection (z_known list)
4. Remove from burd_regime.sh validate and render sections

## Acceptance

- Both variables use BURE_ prefix everywhere in active code
- BURE grows from 1 to 3 variables (COUNTDOWN, VERBOSE, COLOR)
- BURD shrinks by 2
- All BUSA/BUSD linked terms updated (no dangling attribute references or anchors)
- Qualification passes

**[260219-0918] rough**

Relocate two ambient caller-override BURD_ variables to BURE (environment regime).

## Context

Two BURD_ variables are ambient caller preferences that can be set before dispatch.
They belong in BURE alongside BURE_COUNTDOWN, not in BURD which is dispatch infrastructure.

Three variables from the original scope do NOT move:
- BURD_NO_LOG — set by tabtarget (buut_tabtarget.sh), not caller
- BURD_INTERACTIVE — set by tabtarget (buut_tabtarget.sh), not caller
- BURD_TERM_COLS — set by launcher (bul_launcher.sh), not caller

## Variables to relocate

- BURD_VERBOSE → BURE_VERBOSE
- BURD_COLOR → BURE_COLOR

## AsciiDoc changes (MCM linked terms)

### BUSA mapping section

Remove from BURD mapping block (~line 88, 92):
- `:burd_verbose:  <<burd_verbose,BURD_VERBOSE>>`
- `:burd_color:    <<burd_color,BURD_COLOR>>`

Add to BURE mapping block (after line 121, before `// end::mapping-section[]`):
- `:bure_verbose:  <<bure_verbose,BURE_VERBOSE>>`
- `:bure_color:    <<bure_color,BURE_COLOR>>`

### BUSA BURD regime section (inline, ~line 485–513)

Remove the `[[burd_verbose]]` anchor+annotation+definition block (lines 487–491).
Remove the `[[burd_color]]` anchor+annotation+definition block (lines 508–513).

### BUSA BURE regime section (inline, ~line 631+)

Add two new anchor+annotation+definition blocks:

```
[[bure_verbose]]
// ⟦axl_voices axrg_variable axtu_integer⟧
{bure_verbose}::
Verbosity level controlling diagnostic output.
0=silent, 1=show messages, 2=enable bash trace.

[[bure_color]]
// ⟦axl_voices axrg_variable axtu_string⟧
{bure_color}::
Color output policy.
Input as auto/0/1; resolved to 0 or 1 during dispatch.
Respects NO_COLOR environment convention.
```

### BUSD-DispatchRuntime.adoc

Remove the `{burd_verbose}` variable entry (lines 14–19).
Remove the `{burd_color}` variable entry (lines 38–43).
Update the group_input description (line 12) to note that ambient overrides moved to BURE.

## Bash changes

1. Add BURE_VERBOSE and BURE_COLOR to bure_regime.sh (kindle defaults, unexpected detection, validation)
2. Rename all usages across codebase (bud_dispatch.sh, buc_command.sh, workbenches, etc.)
3. Remove old BURD_VERBOSE and BURD_COLOR from zburd unexpected variable detection (z_known list)
4. Remove from burd_regime.sh validate and render sections

## Acceptance

- Both variables use BURE_ prefix everywhere in active code
- BURE grows from 1 to 3 variables (COUNTDOWN, VERBOSE, COLOR)
- BURD shrinks by 2
- All BUSA/BUSD linked terms updated (no dangling attribute references or anchors)
- Qualification passes

**[260219-0847] rough**

Relocate two ambient caller-override BURD_ variables to BURE (environment regime).

## Context

Two BURD_ variables are ambient caller preferences that can be set before dispatch.
They belong in BURE alongside BURE_COUNTDOWN, not in BURD which is dispatch infrastructure.

Three variables from the original scope do NOT move:
- BURD_NO_LOG — set by tabtarget (buut_tabtarget.sh), not caller
- BURD_INTERACTIVE — set by tabtarget (buut_tabtarget.sh), not caller
- BURD_TERM_COLS — set by launcher (bul_launcher.sh), not caller

## Variables to relocate

- BURD_VERBOSE → BURE_VERBOSE
- BURD_COLOR → BURE_COLOR

## Scope

1. Add BURE_VERBOSE and BURE_COLOR to bure_regime.sh (kindle defaults, unexpected detection, validation)
2. Update BUSA inline BURE section with variable voicings
3. Update BUSD-DispatchRuntime.adoc to remove VERBOSE and COLOR from BURD variable list
4. Update BUSA inline BURD section to remove these two
5. Rename all usages across codebase (bud_dispatch.sh, buc_command.sh, workbenches, etc.)
6. Remove old BURD_VERBOSE and BURD_COLOR from zburd unexpected variable detection
7. Update burd_regime.sh render section to remove these two

## Acceptance

- Both variables use BURE_ prefix everywhere in active code
- BURE grows from 1 to 3 variables (COUNTDOWN, VERBOSE, COLOR)
- BURD shrinks by 2
- Qualification passes

**[260219-0842] rough**

Drafted from ₢AOAAN in ₣AO.

Relocate caller-intent BURD_ variables to BURE (ambient environment regime).

## Context

Five BURD_ "input" variables are actually ambient caller preferences, not dispatch infrastructure. They belong in BURE alongside BURE_COUNTDOWN.

## Variables to relocate

- BURD_VERBOSE → BURE_VERBOSE
- BURD_NO_LOG → BURE_NO_LOG
- BURD_INTERACTIVE → BURE_INTERACTIVE
- BURD_COLOR → BURE_COLOR
- BURD_TERM_COLS → BURE_TERM_COLS

## Scope

1. Add new variables to bure_regime.sh (kindle defaults, unexpected detection, validation)
2. Update BUSE-EnvironmentRuntime.adoc (or equivalent spec doc) with variable voicings
3. Update BUSD-DispatchRuntime.adoc to remove relocated variables
4. Update BUSA mappings for both regimes
5. Rename all usages across codebase (bud_dispatch.sh, bul_launcher.sh, bupr_PresentationRegime.sh, etc.)
6. Remove old BURD_ names from zburd unexpected variable detection

## Acceptance

- All five variables use BURE_ prefix everywhere
- BURD drops from 24 to 19 variables
- BURE grows from 1 to 6 variables
- Qualification passes

**[260217-1625] rough**

Relocate caller-intent BURD_ variables to BURE (ambient environment regime).

## Context

Five BURD_ "input" variables are actually ambient caller preferences, not dispatch infrastructure. They belong in BURE alongside BURE_COUNTDOWN.

## Variables to relocate

- BURD_VERBOSE → BURE_VERBOSE
- BURD_NO_LOG → BURE_NO_LOG
- BURD_INTERACTIVE → BURE_INTERACTIVE
- BURD_COLOR → BURE_COLOR
- BURD_TERM_COLS → BURE_TERM_COLS

## Scope

1. Add new variables to bure_regime.sh (kindle defaults, unexpected detection, validation)
2. Update BUSE-EnvironmentRuntime.adoc (or equivalent spec doc) with variable voicings
3. Update BUSD-DispatchRuntime.adoc to remove relocated variables
4. Update BUSA mappings for both regimes
5. Rename all usages across codebase (bud_dispatch.sh, bul_launcher.sh, bupr_PresentationRegime.sh, etc.)
6. Remove old BURD_ names from zburd unexpected variable detection

## Acceptance

- All five variables use BURE_ prefix everywhere
- BURD drops from 24 to 19 variables
- BURE grows from 1 to 6 variables
- Qualification passes

### buq-exfiltrate-qualify (₢AfAAM) [complete]

**[260220-0813] complete**

Exfiltrate buv_qualify_tabtargets to new buq_ (qualification) module.

## Context

buv_validation.sh is being restructured from a bag of stateless validators into a kindle-aware enrollment registry. The buv_qualify_tabtargets function validates tabtarget file structure (shebangs, BURD_LAUNCHER lines) — a different concern from variable validation. Moving it to its own module clears the way for clean buv_ restructuring.

## Pre-flight

Run `./tt/rbw-qa.QualifyAll.sh` and capture output. This is the baseline — the post-flight must match.

## Scope

1. Create Tools/buk/buq_qualify.sh
2. Move buv_qualify_tabtargets → buq_qualify_tabtargets
3. Add sourcing/kindle boilerplate as needed
4. Update caller in Tools/rbw/rbq_Qualify.sh (line 131)
5. Ensure rbq_Qualify.sh sources the new module
6. Remove the function from buv_validation.sh

## Post-flight

Run `./tt/rbw-qa.QualifyAll.sh` again. Output must match pre-flight exactly.

## Acceptance

- buv_qualify_tabtargets no longer exists in buv_validation.sh
- buq_qualify_tabtargets works identically in new buq_qualify.sh
- All callers updated
- `./tt/rbw-qa.QualifyAll.sh` produces identical output before and after

**[260220-0805] rough**

Exfiltrate buv_qualify_tabtargets to new buq_ (qualification) module.

## Context

buv_validation.sh is being restructured from a bag of stateless validators into a kindle-aware enrollment registry. The buv_qualify_tabtargets function validates tabtarget file structure (shebangs, BURD_LAUNCHER lines) — a different concern from variable validation. Moving it to its own module clears the way for clean buv_ restructuring.

## Pre-flight

Run `./tt/rbw-qa.QualifyAll.sh` and capture output. This is the baseline — the post-flight must match.

## Scope

1. Create Tools/buk/buq_qualify.sh
2. Move buv_qualify_tabtargets → buq_qualify_tabtargets
3. Add sourcing/kindle boilerplate as needed
4. Update caller in Tools/rbw/rbq_Qualify.sh (line 131)
5. Ensure rbq_Qualify.sh sources the new module
6. Remove the function from buv_validation.sh

## Post-flight

Run `./tt/rbw-qa.QualifyAll.sh` again. Output must match pre-flight exactly.

## Acceptance

- buv_qualify_tabtargets no longer exists in buv_validation.sh
- buq_qualify_tabtargets works identically in new buq_qualify.sh
- All callers updated
- `./tt/rbw-qa.QualifyAll.sh` produces identical output before and after

**[260219-1852] rough**

Exfiltrate buv_qualify_tabtargets to new buq_ (qualification) module.

## Context

buv_validation.sh is being restructured from a bag of stateless validators into a kindle-aware enrollment registry. The buv_qualify_tabtargets function validates tabtarget file structure (shebangs, BURD_LAUNCHER lines) — a different concern from variable validation. Moving it to its own module clears the way for clean buv_ restructuring.

## Scope

1. Create Tools/buk/buq_qualify.sh
2. Move buv_qualify_tabtargets → buq_qualify_tabtargets
3. Add sourcing/kindle boilerplate as needed
4. Update all callers (likely rbq_Qualify.sh and test harnesses)
5. Remove the function from buv_validation.sh

## Acceptance

- buv_qualify_tabtargets no longer exists
- buq_qualify_tabtargets works identically
- All callers updated
- Qualification passes

### buv-enrollment-infrastructure (₢AfAAN) [complete]

**[260220-0833] complete**

Create the buv_ enrollment system: declarative regime variable registration with dual consumption paths.

## Design

Regime variables are enrolled during kindle with type-specific functions. Two consumption paths read the same enrolled data:

- buv_enforce SCOPE — silent, dies on first failure (internal gate)
- buv_report SCOPE "Label" — rich per-variable display, returns non-zero if any failed (CLI validate command)

## Kindle Graph

buv_ gets a zbuv_kindle that initializes empty enrollment rolls (following buz/rbz precedent). Every CLI furnish that uses enrollment MUST kindle buv_ first:

  zrbrr_furnish() {
    zbuv_kindle        # init enrollment rolls — MUST be first
    source config
    zrbrr_kindle       # enrolls RBRR variables via buv_*_enroll
    buv_enforce RBRR
  }

Each process runs exactly one furnish, so buv_ kindle is per-process. BCG's die-on-re-kindle prevents accidental double-kindle within the same furnish.

For multi-regime furnishes (e.g. RBRN CLI needs RBRR context too):

  zrbrn_furnish() {
    zbuv_kindle
    source config
    zrbrr_kindle       # enrolls RBRR
    buv_enforce RBRR
    # resolve folio, source nameplate...
    zrbrn_kindle       # enrolls RBRN
    buv_enforce RBRN
  }

## Coexistence

Existing buv_val_*, buv_env_*, buv_opt_* functions remain in place during the transition. The enrollment system adds new functions alongside legacy. Deletion of legacy functions is a separate late pace (delete-legacy-buv-functions) after all regimes are migrated and audited.

## Enrollment Functions (kindle-only, BCG _enroll suffix)

Parameter shape: SCOPE VARNAME GATE_VAR GATE_VAL [type-specific-params...]

Scalar types:
  buv_string_enroll   RBRR  RBRR_PROJECT_ID    ""  ""  3 50
  buv_xname_enroll    RBRR  RBRR_SA_NAME       ""  ""  3 50
  buv_gname_enroll    RBRR  RBRR_REGION        ""  ""  3 30
  buv_fqin_enroll     RBRR  RBRR_IMAGE_REF     ""  ""  5 200
  buv_bool_enroll     RBRR  RBRR_ENABLED       ""  ""
  buv_enum_enroll     RBRR  RBRR_MODE          ""  ""  create delete
  buv_decimal_enroll  RBRR  RBRR_TIMEOUT       ""  ""  1 3600
  buv_odref_enroll    RBRR  RBRR_PINNED_IMAGE  ""  ""

Gated (conditionally required):
  buv_gname_enroll  RBRR  RBRR_BUCKET  RBRR_MODE  create  3 50

List variants (space-separated elements, each element validated as scalar type):
  buv_list_string_enroll  RBRN  RBRN_TAGS          ""  ""  1 20
  buv_list_ipv4_enroll    RBRN  RBRN_DNS_SERVERS   ""  ""
  buv_list_gname_enroll   RBRN  RBRN_ZONES         ""  ""  3 30

No value parameter — variable name is enrolled, value read via ${!varname} at check time.

## Gating

GATE_VAR and GATE_VAL follow SCOPE and VARNAME:
- "" "" = ungated (always required)
- GATE_VAR GATE_VAL = only check when ${!GATE_VAR} equals GATE_VAL
- Report shows gated-out items as "skipped"
- Enforce silently skips gated-out items

## Infrastructure

1. zbuv_kindle — initializes empty enrollment rolls
2. 7 parallel rolls: scope, varname, type, gate_var, gate_val, p1, p2
3. Internal predicate for type checking — returns 0/1, sets error detail variable
4. Both enforce and report call predicate, diverge on behavior
5. Enum choices packed space-separated in p1 (safe: regime values have no spaces)

## Consumption

buv_enforce SCOPE — iterates enrolled variables for SCOPE, calls internal predicate, buc_die on first failure.

buv_report SCOPE "Label" — iterates enrolled variables for SCOPE, prints rich diagnostics per variable (name, value, type, constraint, pass/fail), returns non-zero if any failed.

## Acceptance

- All enrollment functions store correctly into rolls
- buv_enforce dies on bad data, passes silently on good data
- buv_report shows readable per-variable diagnostics
- Gating works (skips when gate does not match)
- List variants validate each element
- zbuv_kindle initializes rolls properly
- No dependency on legacy buv_val_* functions (self-contained checking)
- Legacy buv_val_* functions remain untouched (coexistence)

**[260219-1908] rough**

Create the buv_ enrollment system: declarative regime variable registration with dual consumption paths.

## Design

Regime variables are enrolled during kindle with type-specific functions. Two consumption paths read the same enrolled data:

- buv_enforce SCOPE — silent, dies on first failure (internal gate)
- buv_report SCOPE "Label" — rich per-variable display, returns non-zero if any failed (CLI validate command)

## Kindle Graph

buv_ gets a zbuv_kindle that initializes empty enrollment rolls (following buz/rbz precedent). Every CLI furnish that uses enrollment MUST kindle buv_ first:

  zrbrr_furnish() {
    zbuv_kindle        # init enrollment rolls — MUST be first
    source config
    zrbrr_kindle       # enrolls RBRR variables via buv_*_enroll
    buv_enforce RBRR
  }

Each process runs exactly one furnish, so buv_ kindle is per-process. BCG's die-on-re-kindle prevents accidental double-kindle within the same furnish.

For multi-regime furnishes (e.g. RBRN CLI needs RBRR context too):

  zrbrn_furnish() {
    zbuv_kindle
    source config
    zrbrr_kindle       # enrolls RBRR
    buv_enforce RBRR
    # resolve folio, source nameplate...
    zrbrn_kindle       # enrolls RBRN
    buv_enforce RBRN
  }

## Coexistence

Existing buv_val_*, buv_env_*, buv_opt_* functions remain in place during the transition. The enrollment system adds new functions alongside legacy. Deletion of legacy functions is a separate late pace (delete-legacy-buv-functions) after all regimes are migrated and audited.

## Enrollment Functions (kindle-only, BCG _enroll suffix)

Parameter shape: SCOPE VARNAME GATE_VAR GATE_VAL [type-specific-params...]

Scalar types:
  buv_string_enroll   RBRR  RBRR_PROJECT_ID    ""  ""  3 50
  buv_xname_enroll    RBRR  RBRR_SA_NAME       ""  ""  3 50
  buv_gname_enroll    RBRR  RBRR_REGION        ""  ""  3 30
  buv_fqin_enroll     RBRR  RBRR_IMAGE_REF     ""  ""  5 200
  buv_bool_enroll     RBRR  RBRR_ENABLED       ""  ""
  buv_enum_enroll     RBRR  RBRR_MODE          ""  ""  create delete
  buv_decimal_enroll  RBRR  RBRR_TIMEOUT       ""  ""  1 3600
  buv_odref_enroll    RBRR  RBRR_PINNED_IMAGE  ""  ""

Gated (conditionally required):
  buv_gname_enroll  RBRR  RBRR_BUCKET  RBRR_MODE  create  3 50

List variants (space-separated elements, each element validated as scalar type):
  buv_list_string_enroll  RBRN  RBRN_TAGS          ""  ""  1 20
  buv_list_ipv4_enroll    RBRN  RBRN_DNS_SERVERS   ""  ""
  buv_list_gname_enroll   RBRN  RBRN_ZONES         ""  ""  3 30

No value parameter — variable name is enrolled, value read via ${!varname} at check time.

## Gating

GATE_VAR and GATE_VAL follow SCOPE and VARNAME:
- "" "" = ungated (always required)
- GATE_VAR GATE_VAL = only check when ${!GATE_VAR} equals GATE_VAL
- Report shows gated-out items as "skipped"
- Enforce silently skips gated-out items

## Infrastructure

1. zbuv_kindle — initializes empty enrollment rolls
2. 7 parallel rolls: scope, varname, type, gate_var, gate_val, p1, p2
3. Internal predicate for type checking — returns 0/1, sets error detail variable
4. Both enforce and report call predicate, diverge on behavior
5. Enum choices packed space-separated in p1 (safe: regime values have no spaces)

## Consumption

buv_enforce SCOPE — iterates enrolled variables for SCOPE, calls internal predicate, buc_die on first failure.

buv_report SCOPE "Label" — iterates enrolled variables for SCOPE, prints rich diagnostics per variable (name, value, type, constraint, pass/fail), returns non-zero if any failed.

## Acceptance

- All enrollment functions store correctly into rolls
- buv_enforce dies on bad data, passes silently on good data
- buv_report shows readable per-variable diagnostics
- Gating works (skips when gate does not match)
- List variants validate each element
- zbuv_kindle initializes rolls properly
- No dependency on legacy buv_val_* functions (self-contained checking)
- Legacy buv_val_* functions remain untouched (coexistence)

**[260219-1853] rough**

Create the buv_ enrollment system: declarative regime variable registration with dual consumption paths.

## Design

Regime variables are enrolled during kindle with type-specific functions. Two consumption paths read the same enrolled data:

- buv_enforce SCOPE — silent, dies on first failure (internal gate)
- buv_report SCOPE "Label" — rich per-variable display, returns non-zero if any failed (CLI validate command)

## Enrollment Functions (kindle-only, BCG _enroll suffix)

Parameter shape: SCOPE VARNAME GATE_VAR GATE_VAL [type-specific-params...]

Scalar types:
  buv_string_enroll   RBRR  RBRR_PROJECT_ID    ""  ""  3 50
  buv_xname_enroll    RBRR  RBRR_SA_NAME       ""  ""  3 50
  buv_gname_enroll    RBRR  RBRR_REGION        ""  ""  3 30
  buv_fqin_enroll     RBRR  RBRR_IMAGE_REF     ""  ""  5 200
  buv_bool_enroll     RBRR  RBRR_ENABLED       ""  ""
  buv_enum_enroll     RBRR  RBRR_MODE          ""  ""  create delete
  buv_decimal_enroll  RBRR  RBRR_TIMEOUT       ""  ""  1 3600
  buv_odref_enroll    RBRR  RBRR_PINNED_IMAGE  ""  ""

Gated (conditionally required):
  buv_gname_enroll  RBRR  RBRR_BUCKET  RBRR_MODE  create  3 50

List variants (space-separated elements, each element validated as scalar type):
  buv_list_string_enroll  RBRN  RBRN_TAGS          ""  ""  1 20
  buv_list_ipv4_enroll    RBRN  RBRN_DNS_SERVERS   ""  ""
  buv_list_gname_enroll   RBRN  RBRN_ZONES         ""  ""  3 30

No value parameter — variable name is enrolled, value read via ${!varname} at check time.

## Gating

GATE_VAR and GATE_VAL follow SCOPE and VARNAME:
- "" "" = ungated (always required)
- GATE_VAR GATE_VAL = only check when ${!GATE_VAR} equals GATE_VAL
- Report shows gated-out items as "skipped"
- Enforce silently skips gated-out items

## Infrastructure

1. zbuv_kindle — initializes empty enrollment rolls (following buz/rbz precedent)
2. 7 parallel rolls: scope, varname, type, gate_var, gate_val, p1, p2
3. Internal predicate for type checking — returns 0/1, sets error detail variable
4. Both enforce and report call predicate, diverge on behavior
5. Enum choices packed space-separated in p1 (safe: regime values have no spaces)

## Consumption

buv_enforce SCOPE — iterates enrolled variables for SCOPE, calls internal predicate, buc_die on first failure.

buv_report SCOPE "Label" — iterates enrolled variables for SCOPE, prints rich diagnostics per variable (name, value, type, constraint, pass/fail), returns non-zero if any failed.

## Acceptance

- All enrollment functions store correctly into rolls
- buv_enforce dies on bad data, passes silently on good data
- buv_report shows readable per-variable diagnostics
- Gating works (skips when gate does not match)
- List variants validate each element
- zbuv_kindle initializes rolls properly
- No dependency on legacy buv_val_* functions (self-contained checking)

### bcg-add-enforce-pattern (₢AfAAH) [abandoned]

**[260220-0853] abandoned**

Add enforcement pattern to BCG using buv_ enrollment system.

## Context

The enforce function is the post-kindle invariant gate. Rather than each module implementing its own z-prefix_enforce(), enforcement is handled by the buv_ enrollment system: variables are enrolled during kindle with type constraints, then buv_enforce SCOPE checks all enrolled variables and dies on first violation.

## Key Design Points

- Not all CLI paths call enforce. A CLI that receives no folio (manifold) or no subcommand may choose to list valid options and exit gracefully rather than enforcing.
- The furnish-kindle-enforce sequence is the happy path, not the only path.
- Enrollment happens in kindle. Enforce is a separate call from furnish, only on paths that need it.

## Scope

1. **BCG boilerplate table** — Add enforce row:
   - Location: Furnish (not Implementation)
   - Purpose: Verify all enrolled variables meet contracts, die on failure
   - Pattern: buv_enforce SCOPE
   - Not per-module boilerplate — delegates to buv_ enrollment system

2. **BCG furnish template** — Update to show enforce call after kindle:
   z-prefix_furnish() {
     buc_doc_env ...
     source config
     z-prefix_kindle       # enrolls variables with buv_*_enroll
     buv_enforce PREFIX    # checks all enrolled, dies on first failure
   }

3. **BCG enrollment documentation** — Add section documenting the enrollment pattern:
   - buv_*_enroll functions called during kindle
   - buv_enforce for internal gates
   - buv_report for CLI validate commands
   - Gating for conditionally-required variables

4. **Module maturity checklist** — Add enrollment/enforce checkpoints

## NOT in scope

- Creating the enrollment infrastructure (that is the prior buv-enrollment-infrastructure pace)
- AXLA voicings

## Acceptance

- BCG documents enforce via buv_ enrollment, not per-module boilerplate
- Furnish template shows buv_enforce call
- Enrollment pattern documented
- Checklist updated

**[260219-1855] rough**

Add enforcement pattern to BCG using buv_ enrollment system.

## Context

The enforce function is the post-kindle invariant gate. Rather than each module implementing its own z-prefix_enforce(), enforcement is handled by the buv_ enrollment system: variables are enrolled during kindle with type constraints, then buv_enforce SCOPE checks all enrolled variables and dies on first violation.

## Key Design Points

- Not all CLI paths call enforce. A CLI that receives no folio (manifold) or no subcommand may choose to list valid options and exit gracefully rather than enforcing.
- The furnish-kindle-enforce sequence is the happy path, not the only path.
- Enrollment happens in kindle. Enforce is a separate call from furnish, only on paths that need it.

## Scope

1. **BCG boilerplate table** — Add enforce row:
   - Location: Furnish (not Implementation)
   - Purpose: Verify all enrolled variables meet contracts, die on failure
   - Pattern: buv_enforce SCOPE
   - Not per-module boilerplate — delegates to buv_ enrollment system

2. **BCG furnish template** — Update to show enforce call after kindle:
   z-prefix_furnish() {
     buc_doc_env ...
     source config
     z-prefix_kindle       # enrolls variables with buv_*_enroll
     buv_enforce PREFIX    # checks all enrolled, dies on first failure
   }

3. **BCG enrollment documentation** — Add section documenting the enrollment pattern:
   - buv_*_enroll functions called during kindle
   - buv_enforce for internal gates
   - buv_report for CLI validate commands
   - Gating for conditionally-required variables

4. **Module maturity checklist** — Add enrollment/enforce checkpoints

## NOT in scope

- Creating the enrollment infrastructure (that is the prior buv-enrollment-infrastructure pace)
- AXLA voicings

## Acceptance

- BCG documents enforce via buv_ enrollment, not per-module boilerplate
- Furnish template shows buv_enforce call
- Enrollment pattern documented
- Checklist updated

**[260219-0909] rough**

Add `z«prefix»_enforce` to BCG as a standard boilerplate function.

## Context

The enforce function is the post-kindle invariant gate. It verifies kindled state meets field contracts and dies on first violation. Currently this role is filled by ad-hoc `_validate_fields` functions — "enforce" gives it a first-class name distinct from the human-facing "validate" CLI command.

## Key Design Point

Not all CLI paths call enforce. A CLI that receives no folio (manifold) or no subcommand may choose to list valid options and exit gracefully rather than enforcing. The furnish→kindle→enforce sequence is the *happy path*, not the only path. The CLI owns presentation logic including "here are your valid choices."

## Scope

1. **BCG boilerplate table** — Add enforce row:
   - Location: Implementation
   - First line: `z«prefix»_sentinel` (kindle must be complete)
   - Purpose: Verify kindled state meets field contracts, die on failure
   - Can source: No
   - buc_step: No (use buc_log_*)

2. **BCG furnish template** — Update to show enforce call after kindle:
   ```
   z«prefix»_furnish() {
     buc_doc_env ...
     source config
     z«prefix»_kindle
     z«prefix»_enforce
   }
   ```

3. **BCG naming table** — Add enforce row to naming conventions

4. **Module maturity checklist** — Add enforce checkpoint

## NOT in scope

- AXLA voicings (enforce is a BCG pattern, not a domain concept)
- Renaming existing _validate_fields functions (that happens in the exemplar paces)

## Acceptance

- BCG documents enforce as standard boilerplate alongside kindle/sentinel/furnish
- Templates updated
- Checklist updated

### scrub-rbrr-singleton-exemplar (₢AfAAC) [complete]

**[260221-0804] complete**

Full BCG scrub of RBRR (Repository Regime) as the singleton exemplar.

## Before Anchor

Commit 57b1ff99 is the baseline before any regime second-pass work.

## Scope

1. **rbrr_regime.sh** — Restructure validation using enrollment:
   - Add buv_*_enroll calls in zrbrr_kindle for all RBRR variables
   - Eliminate rbrr_validate / zrbrr_validate_fields entirely — replaced by buv_enforce/buv_report
   - Eliminate rbrr_load() entirely (surrogate furnish anti-pattern)
   - Review kindle constants — are defaults still needed or can enrollment handle missing fields?

2. **rbrr_cli.sh** — Restructure to proper BCG CLI template:
   - Add zrbrr_furnish() with buc_doc_env for all required env vars
   - Furnish kindles buv_ first, then sources config, calls zrbrr_kindle (which enrolls), calls buv_enforce RBRR
   - Use buc_execute rbrr_ "Recipe Bottle Repository Regime" zrbrr_furnish "$@"
   - Replace validate command body with buv_report RBRR "Repository Regime"
   - All public command functions get buc_doc_brief/buc_doc_param/buc_doc_shown blocks

3. **Consuming CLIs** — Inline the kindle sequence in each furnish:
   - rbgg_cli.sh, rbf_cli.sh, rbgm_cli.sh, rbgp_cli.sh
   - Replace rbrr_load with: zbuv_kindle + source config + zrbrr_kindle + buv_enforce RBRR
   - Remove redundant test -f guards (kindle handles this)
   - NOT: rbrn_cli.sh, rbrv_cli.sh, rbob_cli.sh — these have dedicated paces (AfAAE, AfAAI) that handle their full restructuring including rbrr_load elimination

4. **Test harnesses** — Update butcrg_RegimeSmoke.sh, butcrg_RegimeCredentials.sh, rbtb_testbench.sh
   - BCG grants test harness latitude for direct kindle calls
   - Still eliminate rbrr_load in favor of explicit source + kindle

## Acceptance

- rbrr_load deleted from codebase (confirmed across all files, not just this pace's scope)
- No per-module validate_fields function — enrollment handles all checking
- rbrr_cli.sh uses buc_execute pattern
- Validate command calls buv_report RBRR "Repository Regime"
- Furnish calls buv_enforce RBRR after kindle
- All public functions have buc_doc_* blocks
- Qualification passes
- All test suites that were passing still pass

**[260219-1909] rough**

Full BCG scrub of RBRR (Repository Regime) as the singleton exemplar.

## Before Anchor

Commit 57b1ff99 is the baseline before any regime second-pass work.

## Scope

1. **rbrr_regime.sh** — Restructure validation using enrollment:
   - Add buv_*_enroll calls in zrbrr_kindle for all RBRR variables
   - Eliminate rbrr_validate / zrbrr_validate_fields entirely — replaced by buv_enforce/buv_report
   - Eliminate rbrr_load() entirely (surrogate furnish anti-pattern)
   - Review kindle constants — are defaults still needed or can enrollment handle missing fields?

2. **rbrr_cli.sh** — Restructure to proper BCG CLI template:
   - Add zrbrr_furnish() with buc_doc_env for all required env vars
   - Furnish kindles buv_ first, then sources config, calls zrbrr_kindle (which enrolls), calls buv_enforce RBRR
   - Use buc_execute rbrr_ "Recipe Bottle Repository Regime" zrbrr_furnish "$@"
   - Replace validate command body with buv_report RBRR "Repository Regime"
   - All public command functions get buc_doc_brief/buc_doc_param/buc_doc_shown blocks

3. **Consuming CLIs** — Inline the kindle sequence in each furnish:
   - rbgg_cli.sh, rbf_cli.sh, rbgm_cli.sh, rbgp_cli.sh
   - Replace rbrr_load with: zbuv_kindle + source config + zrbrr_kindle + buv_enforce RBRR
   - Remove redundant test -f guards (kindle handles this)
   - NOT: rbrn_cli.sh, rbrv_cli.sh, rbob_cli.sh — these have dedicated paces (AfAAE, AfAAI) that handle their full restructuring including rbrr_load elimination

4. **Test harnesses** — Update butcrg_RegimeSmoke.sh, butcrg_RegimeCredentials.sh, rbtb_testbench.sh
   - BCG grants test harness latitude for direct kindle calls
   - Still eliminate rbrr_load in favor of explicit source + kindle

## Acceptance

- rbrr_load deleted from codebase (confirmed across all files, not just this pace's scope)
- No per-module validate_fields function — enrollment handles all checking
- rbrr_cli.sh uses buc_execute pattern
- Validate command calls buv_report RBRR "Repository Regime"
- Furnish calls buv_enforce RBRR after kindle
- All public functions have buc_doc_* blocks
- Qualification passes
- All test suites that were passing still pass

**[260219-1855] rough**

Full BCG scrub of RBRR (Repository Regime) as the singleton exemplar.

## Before Anchor

Commit 57b1ff99 is the baseline before any regime second-pass work.

## Scope

1. **rbrr_regime.sh** — Restructure validation using enrollment:
   - Add buv_*_enroll calls in zrbrr_kindle for all RBRR variables
   - Eliminate rbrr_validate / zrbrr_validate_fields entirely — replaced by buv_enforce/buv_report
   - Eliminate rbrr_load() entirely (surrogate furnish anti-pattern)
   - Review kindle constants — are defaults still needed or can enrollment handle missing fields?

2. **rbrr_cli.sh** — Restructure to proper BCG CLI template:
   - Add zrbrr_furnish() with buc_doc_env for all required env vars
   - Furnish sources config, calls zrbrr_kindle (which enrolls), calls buv_enforce RBRR
   - Use buc_execute rbrr_ "Recipe Bottle Repository Regime" zrbrr_furnish "$@"
   - Replace validate command body with buv_report RBRR "Repository Regime"
   - All public command functions get buc_doc_brief/buc_doc_param/buc_doc_shown blocks

3. **Consuming CLIs** — Inline the kindle sequence in each furnish:
   - rbgg_cli.sh, rbf_cli.sh, rbgm_cli.sh, rbgp_cli.sh, rbob_cli.sh, rbrn_cli.sh, rbrv_cli.sh
   - Replace rbrr_load with: source config + zrbrr_kindle + buv_enforce RBRR
   - Remove redundant test -f guards (kindle handles this)

4. **Test harnesses** — Update butcrg_RegimeSmoke.sh, butcrg_RegimeCredentials.sh, rbtb_testbench.sh
   - BCG grants test harness latitude for direct kindle calls
   - Still eliminate rbrr_load in favor of explicit source + kindle

## Acceptance

- rbrr_load deleted from codebase
- No per-module validate_fields function — enrollment handles all checking
- rbrr_cli.sh uses buc_execute pattern
- Validate command calls buv_report RBRR "Repository Regime"
- Furnish calls buv_enforce RBRR after kindle
- All public functions have buc_doc_* blocks
- Qualification passes
- All test suites that were passing still pass

**[260219-0900] rough**

Full BCG scrub of RBRR (Repository Regime) as the singleton exemplar.

## Before Anchor

Commit `57b1ff99` is the baseline before any regime second-pass work.

## Scope

1. **rbrr_cli.sh** — Restructure to proper BCG CLI template:
   - Add `zrbrr_furnish()` with `buc_doc_env` for all required env vars
   - Furnish sources config, calls `zrbrr_kindle`, calls validation
   - Use `buc_execute rbrr_ "Recipe Bottle Repository Regime" zrbrr_furnish "$@"`
   - All public command functions get `buc_doc_brief`/`buc_doc_param`/`buc_doc_shown` blocks

2. **rbrr_regime.sh** — Clean up word cancer:
   - Eliminate `rbrr_load()` entirely (surrogate furnish anti-pattern)
   - Resolve `rbrr_validate` vs `zrbrr_validate_fields` naming confusion
   - Review kindle constants — are defaults (lines 34-57) still needed or can validation handle missing fields?
   - Fix any remaining BCG violations (comments-as-runtime-info, etc.)

3. **Consuming CLIs** — Inline the kindle sequence in each furnish:
   - rbgg_cli.sh, rbf_cli.sh, rbgm_cli.sh, rbgp_cli.sh, rbob_cli.sh, rbrn_cli.sh, rbrv_cli.sh
   - Replace `rbrr_load` with: source config + `zrbrr_kindle` + validation call
   - Remove redundant `test -f` guards (kindle handles this)

4. **Test harnesses** — Update butcrg_RegimeSmoke.sh, butcrg_RegimeCredentials.sh, rbtb_testbench.sh
   - BCG grants test harness latitude for direct kindle calls
   - Still eliminate `rbrr_load` in favor of explicit source + kindle

## Acceptance

- `rbrr_load` deleted from codebase
- rbrr_cli.sh uses buc_execute pattern
- All public functions have buc_doc_* blocks
- Qualification passes
- All test suites that were passing still pass

### buz-channel-infrastructure (₢AfAAD) [complete]

**[260221-0826] complete**

Add RBR0_FOLIO regime-selection channel to the zipper infrastructure.

## Concepts

- **RBR0_FOLIO**: Environment variable holding the selected manifold instance identifier. The `0` in `RBR0_` guarantees no collision with alphabetic regime prefixes (rbrr, rbrn, rbrv...).
- **Channel**: Per-colophon declaration of how folio arrives: `""` (singleton, no selection), `"imprint"` (from tabtarget filename suffix), `"param1"` (from first positional arg, consumed/shifted by decode).

## Scope

1. **buz_zipper.sh** — Extend blazon registration:
   - `buz_blazon` gains optional 5th parameter: channel (`""`, `"imprint"`, `"param1"`)
   - New roll: `z_buz_channel_roll`
   - Default to `""` when 5th param omitted (backward compatible)

2. **zbuz_decode_folio** — New internal function:
   - Reads channel from roll for the matched colophon
   - `"imprint"`: sets `RBR0_FOLIO` from `BURD_IMPRINT`
   - `"param1"`: sets `RBR0_FOLIO` from `$1`, eats the arg (shifts remaining args)
   - `""`: no-op
   - Decode and eat are atomic — single function, single responsibility

3. **zbuz_exec_lookup** — Call `zbuz_decode_folio` before exec'ing CLI

4. **rbz_zipper.sh** — No changes yet (channels added when RBRN scrub lands)

## Acceptance

- `buz_blazon` accepts 5 args, backward compatible with 4
- `zbuz_decode_folio` sets RBR0_FOLIO and shifts correctly
- Existing dispatch unchanged (all current colophons have empty channel)
- Qualification passes

**[260219-0901] rough**

Add RBR0_FOLIO regime-selection channel to the zipper infrastructure.

## Concepts

- **RBR0_FOLIO**: Environment variable holding the selected manifold instance identifier. The `0` in `RBR0_` guarantees no collision with alphabetic regime prefixes (rbrr, rbrn, rbrv...).
- **Channel**: Per-colophon declaration of how folio arrives: `""` (singleton, no selection), `"imprint"` (from tabtarget filename suffix), `"param1"` (from first positional arg, consumed/shifted by decode).

## Scope

1. **buz_zipper.sh** — Extend blazon registration:
   - `buz_blazon` gains optional 5th parameter: channel (`""`, `"imprint"`, `"param1"`)
   - New roll: `z_buz_channel_roll`
   - Default to `""` when 5th param omitted (backward compatible)

2. **zbuz_decode_folio** — New internal function:
   - Reads channel from roll for the matched colophon
   - `"imprint"`: sets `RBR0_FOLIO` from `BURD_IMPRINT`
   - `"param1"`: sets `RBR0_FOLIO` from `$1`, eats the arg (shifts remaining args)
   - `""`: no-op
   - Decode and eat are atomic — single function, single responsibility

3. **zbuz_exec_lookup** — Call `zbuz_decode_folio` before exec'ing CLI

4. **rbz_zipper.sh** — No changes yet (channels added when RBRN scrub lands)

## Acceptance

- `buz_blazon` accepts 5 args, backward compatible with 4
- `zbuz_decode_folio` sets RBR0_FOLIO and shifts correctly
- Existing dispatch unchanged (all current colophons have empty channel)
- Qualification passes

### debug-ark-lifecycle-silent-failure (₢AfAAU) [complete]

**[260221-0844] complete**

Diagnose why the ark-lifecycle test suite fails with exit 1 and produces no output.

## Symptoms

- `./tt/rbw-ts.TestSuite.sh ark-lifecycle` exits 1 with zero stdout/stderr
- Evidence directory is empty (no files produced)
- In full test run (rbw-ta), transcript shows only: `Suite 'ark-lifecycle' failed with status 1`
- No buto_trace, buto_section, or buto_fatal output appears
- All other 10 suites pass

## Context

- Suite setup: `zrbtb_ark_tsuite_setup` sets `ZRBTB_ARK_VESSEL_SIGIL="trbim-macos"`
- Single case: `rbtcal_lifecycle_tcase` in `Tools/rbw/rbts/rbtcal_ArkLifecycle.sh`
- First action in test: `buto_tt_expect_ok "${RBZ_LIST_IMAGES}"` (colophon rbw-il)
- Suite runs inside nested subshells with `set -e` — silent exit if any command fails before output functions
- Pre-existing issue (pace ₢AfAAR "debug-ark-lifecycle-oauth-failure" already exists with 3 commits)

## Investigation approach

1. Run suite with BUT_VERBOSE=2 to enable trace output
2. Check if failure is in suite setup, case setup, or within the test case itself
3. Check if `RBZ_LIST_IMAGES` / `rbw-il` tabtarget resolves correctly in test context
4. Check if the `set -e` + subshell combination swallows the actual error
5. Cross-reference with existing pace ₢AfAAR work

## Acceptance

- Root cause identified
- Fix implemented or clear explanation of external dependency (e.g., expired OAuth token)

**[260221-0825] rough**

Diagnose why the ark-lifecycle test suite fails with exit 1 and produces no output.

## Symptoms

- `./tt/rbw-ts.TestSuite.sh ark-lifecycle` exits 1 with zero stdout/stderr
- Evidence directory is empty (no files produced)
- In full test run (rbw-ta), transcript shows only: `Suite 'ark-lifecycle' failed with status 1`
- No buto_trace, buto_section, or buto_fatal output appears
- All other 10 suites pass

## Context

- Suite setup: `zrbtb_ark_tsuite_setup` sets `ZRBTB_ARK_VESSEL_SIGIL="trbim-macos"`
- Single case: `rbtcal_lifecycle_tcase` in `Tools/rbw/rbts/rbtcal_ArkLifecycle.sh`
- First action in test: `buto_tt_expect_ok "${RBZ_LIST_IMAGES}"` (colophon rbw-il)
- Suite runs inside nested subshells with `set -e` — silent exit if any command fails before output functions
- Pre-existing issue (pace ₢AfAAR "debug-ark-lifecycle-oauth-failure" already exists with 3 commits)

## Investigation approach

1. Run suite with BUT_VERBOSE=2 to enable trace output
2. Check if failure is in suite setup, case setup, or within the test case itself
3. Check if `RBZ_LIST_IMAGES` / `rbw-il` tabtarget resolves correctly in test context
4. Check if the `set -e` + subshell combination swallows the actual error
5. Cross-reference with existing pace ₢AfAAR work

## Acceptance

- Root cause identified
- Fix implemented or clear explanation of external dependency (e.g., expired OAuth token)

### scrub-rbrn-manifold-exemplar (₢AfAAE) [complete]

**[260221-1829] complete**

Full BCG scrub of RBRN (Nameplate Regime) as the manifold exemplar.

## Completed

All original scope items have been implemented and tests pass (22 nameplate + 7 regime smoke):

- rbrn_regime.sh: full buv_*_enroll restructure, zrbrn_enforce with buv_vet, deleted rbrn_load_moniker/rbrn_load_file/ZRBRN_ROLLUP/zrbrn_validate_fields
- rbrn_cli.sh: buc_execute pattern with zrbrn_furnish, RBR0_FOLIO-based nameplate resolution
- rbz_zipper.sh: imprint channels on all bottle and nameplate colophons
- rbw_workbench.sh: collapsed bottle case arms to zbuz_exec_lookup
- rbob_cli.sh + rbtb_testbench.sh: RBR0_FOLIO replaces rbrn_load_moniker
- buv_validation.sh: added buv_port_enroll, buv_list_cidr_enroll, buv_list_domain_enroll
- BCG tier 1+2 fixes: zrbcr_kindle moved to furnish in both rbrn_cli.sh and rbrr_cli.sh; added buv_scope_sentinel and buv_docker_env to BUK; rbrn_regime.sh uses both

## Remaining: Tier 3 — survey/audit BCG compliance (design decision required)

`rbrn_survey` and `rbrn_audit` in rbrn_cli.sh source and kindle a heavyweight dep set
inside the command function bodies — a BCG violation. These deps are:
  - rbgc_Constants.sh + zrbgc_kindle
  - rbgd_DepotConstants.sh + zrbgd_kindle
  - rbrr_regime.sh + source ${RBCC_rbrr_file} + zrbrr_kindle + zrbrr_enforce
  - rbgo_OAuth.sh + zrbgo_kindle

Naively pushing these into zrbrn_furnish would load the full GCP/OAuth/RBRR stack for
every RBRN command (validate, render, list), which is wrong.

**Design options discussed:**

1. **Split CLI** — move rbrn_survey and rbrn_audit to a new CLI (e.g. rbna_cli.sh) with
   its own furnish. Clean separation: fleet/audit operations are genuinely distinct from
   single-nameplate regime operations. Preferred direction.

2. **Tiered furnish** — define a BCG-blessed pattern for commands with heavy optional deps.
   More machinery, keeps everything in one CLI.

Decision needed before implementation. Once decided, also update rbrr_cli.sh as the
singleton exemplar if the chosen pattern applies there too.

## Acceptance (remaining)

- rbrn_survey and rbrn_audit are BCG-compliant (no source/kindle in command bodies)
- Chosen pattern documented and consistent with BCG
- All tests still pass

**[260221-1233] rough**

Full BCG scrub of RBRN (Nameplate Regime) as the manifold exemplar.

## Completed

All original scope items have been implemented and tests pass (22 nameplate + 7 regime smoke):

- rbrn_regime.sh: full buv_*_enroll restructure, zrbrn_enforce with buv_vet, deleted rbrn_load_moniker/rbrn_load_file/ZRBRN_ROLLUP/zrbrn_validate_fields
- rbrn_cli.sh: buc_execute pattern with zrbrn_furnish, RBR0_FOLIO-based nameplate resolution
- rbz_zipper.sh: imprint channels on all bottle and nameplate colophons
- rbw_workbench.sh: collapsed bottle case arms to zbuz_exec_lookup
- rbob_cli.sh + rbtb_testbench.sh: RBR0_FOLIO replaces rbrn_load_moniker
- buv_validation.sh: added buv_port_enroll, buv_list_cidr_enroll, buv_list_domain_enroll
- BCG tier 1+2 fixes: zrbcr_kindle moved to furnish in both rbrn_cli.sh and rbrr_cli.sh; added buv_scope_sentinel and buv_docker_env to BUK; rbrn_regime.sh uses both

## Remaining: Tier 3 — survey/audit BCG compliance (design decision required)

`rbrn_survey` and `rbrn_audit` in rbrn_cli.sh source and kindle a heavyweight dep set
inside the command function bodies — a BCG violation. These deps are:
  - rbgc_Constants.sh + zrbgc_kindle
  - rbgd_DepotConstants.sh + zrbgd_kindle
  - rbrr_regime.sh + source ${RBCC_rbrr_file} + zrbrr_kindle + zrbrr_enforce
  - rbgo_OAuth.sh + zrbgo_kindle

Naively pushing these into zrbrn_furnish would load the full GCP/OAuth/RBRR stack for
every RBRN command (validate, render, list), which is wrong.

**Design options discussed:**

1. **Split CLI** — move rbrn_survey and rbrn_audit to a new CLI (e.g. rbna_cli.sh) with
   its own furnish. Clean separation: fleet/audit operations are genuinely distinct from
   single-nameplate regime operations. Preferred direction.

2. **Tiered furnish** — define a BCG-blessed pattern for commands with heavy optional deps.
   More machinery, keeps everything in one CLI.

Decision needed before implementation. Once decided, also update rbrr_cli.sh as the
singleton exemplar if the chosen pattern applies there too.

## Acceptance (remaining)

- rbrn_survey and rbrn_audit are BCG-compliant (no source/kindle in command bodies)
- Chosen pattern documented and consistent with BCG
- All tests still pass

**[260221-0918] bridled**

Full BCG scrub of RBRN (Nameplate Regime) as the manifold exemplar.

## Context

RBRN is manifold with instances: nsproto, srjcl, pluml. Currently selected via rbrn_load_moniker() which is the manifold equivalent of the rbrr_load() anti-pattern. After buz channel infrastructure lands, manifold selection flows through RBR0_FOLIO.

## Known Consumers of rbrn_load_moniker

- rbob_cli.sh — loads nameplate for bottle operations
- rbtb_testbench.sh — loads nameplate for test suites
- Definition: rbrn_regime.sh

## Scope

1. **rbrn_regime.sh** — Restructure validation using enrollment:
   - Add buv_*_enroll calls in zrbrn_kindle for all RBRN variables, including buv_list_*_enroll for list-type variables
   - Eliminate rbrn_load_moniker() (manifold surrogate furnish)
   - Eliminate per-module validate_fields — replaced by buv_enforce/buv_report
   - Use gated enrollment for variables conditional on RBRN mode/flags
   - Fix BCG violations from paddock audit: multiple [[ == ]] sites, unquoted array, spec-validator drift

2. **rbrn_cli.sh** — Restructure to proper BCG CLI template:
   - Add zrbrn_furnish() with buc_doc_env (including RBR0_FOLIO)
   - Furnish reads RBR0_FOLIO to resolve nameplate file, sources it, kindles (which enrolls)
   - Call buv_enforce RBRN on paths that need it
   - Use buc_execute rbrn_ "Recipe Bottle Nameplate Regime" zrbrn_furnish "$@"
   - Replace validate command body with buv_report RBRN "Nameplate Regime"
   - All public commands get buc_doc_* blocks

3. **rbz_zipper.sh** — Declare channels on RBRN colophons:
   - Nameplate operations: channel = "imprint" or "param1" as appropriate
   - Bottle operations: channel = "imprint"
   - Remove ad-hoc workbench case arms that currently do imprint translation

4. **Consuming CLIs** — Inline kindle sequence in furnish:
   - rbob_cli.sh — use RBR0_FOLIO instead of rbrn_load_moniker
   - rbtb_testbench.sh — test harness, BCG latitude but eliminate _load_moniker

5. **Test harnesses** — Update nameplate test suites

## Acceptance

- rbrn_load_moniker deleted from codebase
- RBR0_FOLIO flows through buz channel mechanism
- Bottle operations no longer need ad-hoc workbench imprint translation
- All validation via buv_ enrollment (buv_enforce in furnish, buv_report in CLI validate)
- List-type variables use buv_list_*_enroll
- rbrn_cli.sh uses buc_execute pattern with buc_doc_* throughout
- All nameplate test suites still pass
- Qualification passes

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/buk/buv_validation.sh, Tools/rbw/rbrn_regime.sh, Tools/rbw/rbrn_cli.sh, Tools/rbw/rbz_zipper.sh, Tools/rbw/rbw_workbench.sh, Tools/rbw/rbob_cli.sh, Tools/rbw/rbtb_testbench.sh (7 files) | Steps: 1. Read rbrr_regime.sh and rbrr_cli.sh as the singleton exemplar before touching anything. 2. In buv_validation.sh add buv_port_enroll wrapping buv_val_port via zbuv_enroll, and buv_list_cidr_enroll and buv_list_domain_enroll following the buv_list_string_enroll pattern exactly. 3. Restructure rbrn_regime.sh: in zrbrn_kindle replace ZRBRN_ROLLUP build and the validate_fields call-site with buv_*_enroll calls -- buv_xname_enroll for MONIKER, buv_string_enroll min=0 for DESCRIPTION and VOLUME_MOUNTS, buv_enum_enroll for RUNTIME with docker/podman, buv_enum_enroll for ENTRY_MODE with disabled/enabled, buv_fqin_enroll for SENTRY_VESSEL/BOTTLE_VESSEL/SENTRY_CONSECRATION/BOTTLE_CONSECRATION, buv_ipv4_enroll for ENCLAVE_BASE_IP/SENTRY_IP/BOTTLE_IP, buv_decimal_enroll 8-30 for NETMASK, buv_port_enroll for UPLINK_PORT_MIN, gated buv_port_enroll for ENTRY_PORT_WORKSTATION and ENTRY_PORT_ENCLAVE with gate_var=RBRN_ENTRY_MODE gate_val=enabled, buv_enum_enroll for UPLINK_DNS_MODE with disabled/global/allowlist, buv_enum_enroll for UPLINK_ACCESS_MODE with disabled/global/allowlist, gated buv_list_cidr_enroll for ALLOWED_CIDRS with gate_var=RBRN_UPLINK_ACCESS_MODE gate_val=allowlist, gated buv_list_domain_enroll for ALLOWED_DOMAINS with gate_var=RBRN_UPLINK_DNS_MODE gate_val=allowlist; add zrbrn_enforce calling buv_vet RBRN then keep the subnet arithmetic checks and cross-port check from current zrbrn_validate_fields; delete rbrn_load_moniker, rbrn_load_file, zrbrn_validate_fields, ZRBRN_ROLLUP; keep ZRBRN_DOCKER_ENV untouched. 4. Restructure rbrn_cli.sh: add source for burd_regime.sh before rbrn_regime.sh in the source block; add zrbrn_furnish that calls zbuv_kindle then zburd_kindle then zrbcc_kindle then if RBR0_FOLIO is non-empty constructs path from RBCC_KIT_DIR plus RBCC_rbrn_prefix plus RBR0_FOLIO plus RBCC_rbrn_ext then sources it and calls zrbrn_kindle and zrbrn_enforce; update rbrn_validate to call buv_report RBRN Nameplate Regime instead of rbrn_load_file; survey and audit commands source their additional deps internally as they do now; replace the ad-hoc case dispatch block at the bottom with buc_execute rbrn_ Recipe Bottle Nameplate Regime zrbrn_furnish. 5. In rbz_zipper.sh add imprint as the 5th arg to buz_blazon for RBZ_RENDER_NAMEPLATE and RBZ_VALIDATE_NAMEPLATE; add a missing RBZ_BOTTLE_STOP entry for rbw-z with imprint channel; add imprint as 5th arg to all existing bottle colophons RBZ_BOTTLE_START/SENTRY/CENSER/CONNECT/OBSERVE. 6. In rbw_workbench.sh collapse the bottle case arm: keep rbw-s with qualification gate calling rbq_cli.sh qualify_all then route via zbuz_exec_lookup; route rbw-z/rbw-S/rbw-C/rbw-B/rbw-o directly via zbuz_exec_lookup in the default arm; remove the RBOB_MONIKER translation since zbuz_decode_folio now sets RBR0_FOLIO from the imprint channel. 7. In rbob_cli.sh replace the RBOB_MONIKER env var and rbrn_load_moniker call in zbob_furnish with a RBR0_FOLIO-based path construction using RBCC constants then source and kindle; in rbtb_testbench.sh replace the rbrn_load_moniker call the same way. | Verify: ./tt/rbw-trg.TestRegime.sh and ./tt/rbw-tn.TestNameplate.nsproto.sh

**[260219-1856] rough**

Full BCG scrub of RBRN (Nameplate Regime) as the manifold exemplar.

## Context

RBRN is manifold with instances: nsproto, srjcl, pluml. Currently selected via rbrn_load_moniker() which is the manifold equivalent of the rbrr_load() anti-pattern. After buz channel infrastructure lands, manifold selection flows through RBR0_FOLIO.

## Known Consumers of rbrn_load_moniker

- rbob_cli.sh — loads nameplate for bottle operations
- rbtb_testbench.sh — loads nameplate for test suites
- Definition: rbrn_regime.sh

## Scope

1. **rbrn_regime.sh** — Restructure validation using enrollment:
   - Add buv_*_enroll calls in zrbrn_kindle for all RBRN variables, including buv_list_*_enroll for list-type variables
   - Eliminate rbrn_load_moniker() (manifold surrogate furnish)
   - Eliminate per-module validate_fields — replaced by buv_enforce/buv_report
   - Use gated enrollment for variables conditional on RBRN mode/flags
   - Fix BCG violations from paddock audit: multiple [[ == ]] sites, unquoted array, spec-validator drift

2. **rbrn_cli.sh** — Restructure to proper BCG CLI template:
   - Add zrbrn_furnish() with buc_doc_env (including RBR0_FOLIO)
   - Furnish reads RBR0_FOLIO to resolve nameplate file, sources it, kindles (which enrolls)
   - Call buv_enforce RBRN on paths that need it
   - Use buc_execute rbrn_ "Recipe Bottle Nameplate Regime" zrbrn_furnish "$@"
   - Replace validate command body with buv_report RBRN "Nameplate Regime"
   - All public commands get buc_doc_* blocks

3. **rbz_zipper.sh** — Declare channels on RBRN colophons:
   - Nameplate operations: channel = "imprint" or "param1" as appropriate
   - Bottle operations: channel = "imprint"
   - Remove ad-hoc workbench case arms that currently do imprint translation

4. **Consuming CLIs** — Inline kindle sequence in furnish:
   - rbob_cli.sh — use RBR0_FOLIO instead of rbrn_load_moniker
   - rbtb_testbench.sh — test harness, BCG latitude but eliminate _load_moniker

5. **Test harnesses** — Update nameplate test suites

## Acceptance

- rbrn_load_moniker deleted from codebase
- RBR0_FOLIO flows through buz channel mechanism
- Bottle operations no longer need ad-hoc workbench imprint translation
- All validation via buv_ enrollment (buv_enforce in furnish, buv_report in CLI validate)
- List-type variables use buv_list_*_enroll
- rbrn_cli.sh uses buc_execute pattern with buc_doc_* throughout
- All nameplate test suites still pass
- Qualification passes

**[260219-0919] rough**

Full BCG scrub of RBRN (Nameplate Regime) as the manifold exemplar.

## Context

RBRN is manifold with instances: nsproto, srjcl, pluml. Currently selected via `rbrn_load_moniker()` which is the manifold equivalent of the `rbrr_load()` anti-pattern. After buz channel infrastructure lands, manifold selection flows through RBR0_FOLIO.

## Known Consumers of rbrn_load_moniker

- `rbob_cli.sh:113` — loads nameplate for bottle operations
- `rbtb_testbench.sh:72` — loads nameplate for test suites
- Definition: `rbrn_regime.sh:203`

## Scope

1. **rbrn_cli.sh** — Restructure to proper BCG CLI template:
   - Add `zrbrn_furnish()` with `buc_doc_env` (including RBR0_FOLIO)
   - Furnish reads RBR0_FOLIO to resolve nameplate file, sources it, kindles
   - Use `buc_execute rbrn_ "Recipe Bottle Nameplate Regime" zrbrn_furnish "$@"`
   - All public commands get buc_doc_* blocks

2. **rbrn_regime.sh** — Clean up:
   - Eliminate `rbrn_load_moniker()` (manifold surrogate furnish)
   - Clean naming confusion between public/internal validate functions
   - Rename `_validate_fields` to `_enforce`
   - Fix BCG violations from paddock audit: multiple `[[ == ]]` sites, unquoted array, spec-validator drift

3. **rbz_zipper.sh** — Declare channels on RBRN colophons:
   - Nameplate operations (rbw-rnr, rbw-rnv): channel = `"imprint"` or `"param1"` as appropriate
   - Bottle operations (rbw-s, rbw-S, rbw-C, rbw-B, rbw-o): channel = `"imprint"`
   - Remove ad-hoc workbench case arms that currently do imprint translation

4. **Consuming CLIs** — Inline kindle sequence in furnish:
   - `rbob_cli.sh` — currently calls `rbrn_load_moniker "${z_moniker}"`, must use RBR0_FOLIO instead
   - `rbtb_testbench.sh` — test harness, BCG latitude applies but still eliminate _load_moniker

5. **Test harnesses** — Update nameplate test suites

## Acceptance

- `rbrn_load_moniker` deleted from codebase
- RBR0_FOLIO flows through buz channel mechanism
- Bottle operations no longer need ad-hoc workbench imprint translation
- rbrn_cli.sh uses buc_execute pattern with buc_doc_* throughout
- All nameplate test suites still pass
- Qualification passes

**[260219-0901] rough**

Full BCG scrub of RBRN (Nameplate Regime) as the manifold exemplar.

## Context

RBRN is manifold with instances: nsproto, srjcl, pluml. Currently selected via `rbrn_load_moniker()` which is the manifold equivalent of the `rbrr_load()` anti-pattern. After buz channel infrastructure lands, manifold selection flows through RBR0_FOLIO.

## Scope

1. **rbrn_cli.sh** — Restructure to proper BCG CLI template:
   - Add `zrbrn_furnish()` with `buc_doc_env` (including RBR0_FOLIO)
   - Furnish reads RBR0_FOLIO to resolve nameplate file, sources it, kindles
   - Use `buc_execute rbrn_ "Recipe Bottle Nameplate Regime" zrbrn_furnish "$@"`
   - All public commands get buc_doc_* blocks

2. **rbrn_regime.sh** — Clean up:
   - Eliminate `rbrn_load_moniker()` (manifold surrogate furnish)
   - Clean naming confusion between public/internal validate functions
   - Fix BCG violations from paddock audit: 8+ `[[ == ]]` sites, unquoted array, spec-validator drift

3. **rbz_zipper.sh** — Declare channels on RBRN colophons:
   - Nameplate operations (rbw-rnr, rbw-rnv): channel = `"imprint"` or `"param1"` as appropriate
   - Bottle operations (rbw-s, rbw-S, rbw-C, rbw-B, rbw-o): channel = `"imprint"`
   - Remove ad-hoc workbench case arms that currently do imprint translation

4. **Consuming CLIs** — Update any CLI that loaded RBRN via `rbrn_load_moniker`

5. **Test harnesses** — Update nameplate test suites

## Acceptance

- `rbrn_load_moniker` deleted from codebase
- RBR0_FOLIO flows through buz channel mechanism
- Bottle operations no longer need ad-hoc workbench imprint translation
- rbrn_cli.sh uses buc_execute pattern with buc_doc_* throughout
- All nameplate test suites still pass
- Qualification passes

### bcg-weaken-module-cli-unity (₢AfAAV) [complete]

**[260221-1840] complete**

Update BCG to formalize the two-tier CLI pattern and establish the rbc{regime} naming scheme.

## Rule Change: Weaken Module-CLI Unity

BCG currently implies a 1:1 relationship between a module and its CLI (shared prefix).
This breaks down when a module needs two CLIs with structurally different furnish functions.

Weaken to a convention:

> A module's CLI typically shares its prefix (convention, not requirement). A module may be
> served by additional CLIs under independent prefixes when furnish requirements differ
> structurally.

## Two-Tier CLI Pattern

Two tiers, defined by furnish scope:

- **Common CLI** (`rbc{r}c_cli.sh`) — light furnish, stock ops (validate, render, list)
- **Extended CLI** (`rbc{r}x_cli.sh`) — heavy furnish, regime-specific auxiliary ops

Where `{r}` is the single-letter regime identifier:

| Regime     | Definition (stays)   | Common CLI        | Extended CLI      |
|------------|----------------------|-------------------|-------------------|
| Nameplate  | `rbrn_regime.sh`     | `rbcnc_cli.sh`    | `rbcnx_cli.sh`    |
| Repository | `rbrr_regime.sh`     | `rbcrc_cli.sh`    | `rbcrx_cli.sh`    |
| Payor      | `rbrp_regime.sh`     | `rbcpc_cli.sh`    | `rbcpx_cli.sh`    |
| OAuth      | `rbro_regime.sh`     | `rbcoc_cli.sh`    | `rbcox_cli.sh`    |
| Station    | `rbrs_regime.sh`     | `rbcsc_cli.sh`    | `rbcsx_cli.sh`    |
| Vessel     | `rbrv_regime.sh`     | `rbcvc_cli.sh`    | `rbcvx_cli.sh`    |
| Auth/Cred  | `rbra_regime.sh`     | `rbcac_cli.sh`    | `rbcax_cli.sh`    |

This cleanly separates two namespaces: `rbr` = regime definitions, `rbc` = regime CLIs.
The conflation in `rbrn_cli.sh`-style names goes away entirely.

## rbcr Prefix: Free and Repurpose

`rbcr_render.sh` is a backward-compat wrapper around `bupr_PresentationRegime.sh` with 7
consumers in rbw regime CLIs. As each regime CLI is scrubbed (already planned paces in this
heat), that pace should include migrating its `rbcr_*` calls to `bupr_*` directly. Once all
7 consumers are migrated, delete `rbcr_render.sh`.

This frees `rbcr` (currently terminal) to become the Repository regime CLI parent, hosting
`rbcrc_cli.sh` and `rbcrx_cli.sh`.

## Scope

- Update `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`:
  - Amend module-CLI relationship section: unity is convention not requirement
  - Document two-tier pattern with furnish-tier rationale
  - Define `rbc{regime}c` / `rbc{regime}x` naming convention with full regime table
  - Note `rbcr_render.sh` / `bupr_*` as precedent for cross-module CLI utilities
- Coordinate: each remaining regime scrub pace in this heat should include
  `rbcr_*` → `bupr_*` migration for that regime's CLI

## Acceptance

- BCG clearly states prefix-unity is a convention, not a requirement
- Two-tier pattern documented with RBRN as first exemplar
- `rbc{regime}c` / `rbc{regime}x` naming convention defined and tabulated
- Migration path for `rbcr_render.sh` documented

**[260221-1749] rough**

Update BCG to formalize the two-tier CLI pattern and establish the rbc{regime} naming scheme.

## Rule Change: Weaken Module-CLI Unity

BCG currently implies a 1:1 relationship between a module and its CLI (shared prefix).
This breaks down when a module needs two CLIs with structurally different furnish functions.

Weaken to a convention:

> A module's CLI typically shares its prefix (convention, not requirement). A module may be
> served by additional CLIs under independent prefixes when furnish requirements differ
> structurally.

## Two-Tier CLI Pattern

Two tiers, defined by furnish scope:

- **Common CLI** (`rbc{r}c_cli.sh`) — light furnish, stock ops (validate, render, list)
- **Extended CLI** (`rbc{r}x_cli.sh`) — heavy furnish, regime-specific auxiliary ops

Where `{r}` is the single-letter regime identifier:

| Regime     | Definition (stays)   | Common CLI        | Extended CLI      |
|------------|----------------------|-------------------|-------------------|
| Nameplate  | `rbrn_regime.sh`     | `rbcnc_cli.sh`    | `rbcnx_cli.sh`    |
| Repository | `rbrr_regime.sh`     | `rbcrc_cli.sh`    | `rbcrx_cli.sh`    |
| Payor      | `rbrp_regime.sh`     | `rbcpc_cli.sh`    | `rbcpx_cli.sh`    |
| OAuth      | `rbro_regime.sh`     | `rbcoc_cli.sh`    | `rbcox_cli.sh`    |
| Station    | `rbrs_regime.sh`     | `rbcsc_cli.sh`    | `rbcsx_cli.sh`    |
| Vessel     | `rbrv_regime.sh`     | `rbcvc_cli.sh`    | `rbcvx_cli.sh`    |
| Auth/Cred  | `rbra_regime.sh`     | `rbcac_cli.sh`    | `rbcax_cli.sh`    |

This cleanly separates two namespaces: `rbr` = regime definitions, `rbc` = regime CLIs.
The conflation in `rbrn_cli.sh`-style names goes away entirely.

## rbcr Prefix: Free and Repurpose

`rbcr_render.sh` is a backward-compat wrapper around `bupr_PresentationRegime.sh` with 7
consumers in rbw regime CLIs. As each regime CLI is scrubbed (already planned paces in this
heat), that pace should include migrating its `rbcr_*` calls to `bupr_*` directly. Once all
7 consumers are migrated, delete `rbcr_render.sh`.

This frees `rbcr` (currently terminal) to become the Repository regime CLI parent, hosting
`rbcrc_cli.sh` and `rbcrx_cli.sh`.

## Scope

- Update `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`:
  - Amend module-CLI relationship section: unity is convention not requirement
  - Document two-tier pattern with furnish-tier rationale
  - Define `rbc{regime}c` / `rbc{regime}x` naming convention with full regime table
  - Note `rbcr_render.sh` / `bupr_*` as precedent for cross-module CLI utilities
- Coordinate: each remaining regime scrub pace in this heat should include
  `rbcr_*` → `bupr_*` migration for that regime's CLI

## Acceptance

- BCG clearly states prefix-unity is a convention, not a requirement
- Two-tier pattern documented with RBRN as first exemplar
- `rbc{regime}c` / `rbc{regime}x` naming convention defined and tabulated
- Migration path for `rbcr_render.sh` documented

**[260221-1742] rough**

Update BCG to weaken the module-CLI prefix-unity rule: make it a convention, not a requirement.

## Background

BCG currently implies a 1:1 relationship between a module and its CLI: `rbrn_regime.sh`
and `rbrn_cli.sh` share a prefix. This assumption breaks down when a module needs two CLIs
with structurally different furnish functions — e.g., one light (regime file only) and one
heavy (GCP + OAuth + RBRR stack). Forcing both into a single CLI requires sourcing/kindling
heavy deps inside command function bodies, which is itself a BCG violation.

## Rule Change

Weaken the module-CLI unity rule from required to optional:

> A module's CLI typically shares its prefix (convention). But a module may be served by
> additional CLIs under independent prefixes when furnish requirements differ structurally.

## Implications

- Auxiliary CLIs choose their prefix for what they ARE, not which module they serve
- Cross-module CLIs become legitimate (one heavy-furnish CLI spanning multiple regimes)
- Minting: auxiliary CLIs are minted independently, no terminal exclusivity conflict with
  the module prefix
- Retroactively explains rbcr_render.sh: a cross-module utility used by seven CLIs,
  owned by none

## Scope

- Update BCG-BashConsoleGuide.md: add section or amend module-CLI relationship language
- Document the two-tier pattern (primary CLI / auxiliary CLI) with RBRN as the exemplar
- Note that rbcr_render.sh is a precedent for cross-module CLI utilities

## Acceptance

- BCG text clearly states prefix-unity is a convention, not a requirement
- Two-tier pattern is documented with exemplar
- No existing BCG rules contradicted

### buz-zipper-bcg-compliance (₢AfAAb) [complete]

**[260222-1138] complete**

Bring buz_zipper.sh into BCG compliance and simplify.

## Context

buz_zipper.sh provides colophon registry and folio decode infrastructure for workbench dispatch.
It currently has BCG violations and unnecessary complexity for what is fundamentally a simple
registry-and-lookup module.

## Work

- Audit buz_zipper.sh against BCG module maturity checklist
- Fix violations: sentinel guards, variable quoting, function patterns, naming
- Simplify zbuz_decode_folio — the param1/imprint/empty channel dispatch is overengineered
- Ensure all consumers (rbz_zipper.sh, buw_workbench.sh) still work after cleanup
- Run existing tests to validate

## Acceptance

- buz_zipper.sh passes BCG module maturity checklist
- All workbench dispatch paths functional (render, validate, bottle ops, test ops)
- No behavioral changes to consumers

**[260222-1027] rough**

Bring buz_zipper.sh into BCG compliance and simplify.

## Context

buz_zipper.sh provides colophon registry and folio decode infrastructure for workbench dispatch.
It currently has BCG violations and unnecessary complexity for what is fundamentally a simple
registry-and-lookup module.

## Work

- Audit buz_zipper.sh against BCG module maturity checklist
- Fix violations: sentinel guards, variable quoting, function patterns, naming
- Simplify zbuz_decode_folio — the param1/imprint/empty channel dispatch is overengineered
- Ensure all consumers (rbz_zipper.sh, buw_workbench.sh) still work after cleanup
- Run existing tests to validate

## Acceptance

- buz_zipper.sh passes BCG module maturity checklist
- All workbench dispatch paths functional (render, validate, bottle ops, test ops)
- No behavioral changes to consumers

### rbrn-sources-in-furnish (₢AfAAY) [complete]

**[260222-1152] complete**

Apply sources-in-furnish pattern to RBRN CLI files only (rbcnc_cli.sh, rbcnx_cli.sh).

## Context

After burd-export-consolidation (₢AfAAW), BURD_BUK_DIR and BURD_TOOLS_DIR are available in CLI processes. After buc-execute-differential-furnish (₢AfAAX), furnish receives the command name. RBRN CLIs can now move all source commands (except buc_command.sh bootstrap) into furnish, eliminating ../buk/ relative paths.

This is scoped to RBRN only — proving the pattern before expanding to other CLIs. Other CLIs will be migrated after human review confirms the pattern works well.

## Pattern

Before:
```bash
ZRBCNC_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
source "${ZRBCNC_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/../buk/burd_regime.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/rbcc_Constants.sh"

zrbrn_furnish() {
  zbuv_kindle
  zburd_kindle
  zrbcc_kindle
}
```

After:
```bash
source "${BURD_BUK_DIR}/buc_command.sh"

zrbrn_furnish() {
  # Sources
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_TOOLS_DIR}/rbw/rbcc_Constants.sh"
  source "${BURD_TOOLS_DIR}/rbw/rbrn_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"

  # Kindles
  zbuv_kindle
  zburd_kindle
  zrbcc_kindle
  zbupr_kindle
}
```

## Scope

- Tools/rbw/rbcnc_cli.sh — move sources into furnish, use BURD_ paths
- Tools/rbw/rbcnx_cli.sh — move sources into furnish, use BURD_ paths
- Also update BURC_BUK_DIR → BURD_BUK_DIR in rbw_workbench.sh (needed for the RBRN dispatch path)

## Acceptance

- rbcnc_cli.sh and rbcnx_cli.sh use sources-in-furnish pattern
- No ../buk/ paths in these two files
- RBRN tabtargets still work (rbw-rnr, rbw-rnv, rbw-ni, rbw-nv)

**[260222-0756] rough**

Apply sources-in-furnish pattern to RBRN CLI files only (rbcnc_cli.sh, rbcnx_cli.sh).

## Context

After burd-export-consolidation (₢AfAAW), BURD_BUK_DIR and BURD_TOOLS_DIR are available in CLI processes. After buc-execute-differential-furnish (₢AfAAX), furnish receives the command name. RBRN CLIs can now move all source commands (except buc_command.sh bootstrap) into furnish, eliminating ../buk/ relative paths.

This is scoped to RBRN only — proving the pattern before expanding to other CLIs. Other CLIs will be migrated after human review confirms the pattern works well.

## Pattern

Before:
```bash
ZRBCNC_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
source "${ZRBCNC_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/../buk/burd_regime.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/rbcc_Constants.sh"

zrbrn_furnish() {
  zbuv_kindle
  zburd_kindle
  zrbcc_kindle
}
```

After:
```bash
source "${BURD_BUK_DIR}/buc_command.sh"

zrbrn_furnish() {
  # Sources
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_TOOLS_DIR}/rbw/rbcc_Constants.sh"
  source "${BURD_TOOLS_DIR}/rbw/rbrn_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"

  # Kindles
  zbuv_kindle
  zburd_kindle
  zrbcc_kindle
  zbupr_kindle
}
```

## Scope

- Tools/rbw/rbcnc_cli.sh — move sources into furnish, use BURD_ paths
- Tools/rbw/rbcnx_cli.sh — move sources into furnish, use BURD_ paths
- Also update BURC_BUK_DIR → BURD_BUK_DIR in rbw_workbench.sh (needed for the RBRN dispatch path)

## Acceptance

- rbcnc_cli.sh and rbcnx_cli.sh use sources-in-furnish pattern
- No ../buk/ paths in these two files
- RBRN tabtargets still work (rbw-rnr, rbw-rnv, rbw-ni, rbw-nv)

**[260222-0756] rough**

Apply sources-in-furnish pattern to RBRN CLI files only (rbcnc_cli.sh, rbcnx_cli.sh).

## Context

After burd-export-consolidation (₢AfAAW), BURD_BUK_DIR and BURD_TOOLS_DIR are available in CLI processes. After buc-execute-differential-furnish (₢AfAAX), furnish receives the command name. RBRN CLIs can now move all source commands (except buc_command.sh bootstrap) into furnish, eliminating ../buk/ relative paths.

This is scoped to RBRN only — proving the pattern before expanding to other CLIs. Other CLIs will be migrated after human review confirms the pattern works well.

## Pattern

Before:
```bash
ZRBCNC_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
source "${ZRBCNC_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/../buk/burd_regime.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/rbcc_Constants.sh"

zrbrn_furnish() {
  zbuv_kindle
  zburd_kindle
  zrbcc_kindle
}
```

After:
```bash
source "${BURD_BUK_DIR}/buc_command.sh"

zrbrn_furnish() {
  # Sources
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_TOOLS_DIR}/rbw/rbcc_Constants.sh"
  source "${BURD_TOOLS_DIR}/rbw/rbrn_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"

  # Kindles
  zbuv_kindle
  zburd_kindle
  zrbcc_kindle
  zbupr_kindle
}
```

## Scope

- Tools/rbw/rbcnc_cli.sh — move sources into furnish, use BURD_ paths
- Tools/rbw/rbcnx_cli.sh — move sources into furnish, use BURD_ paths
- Also update BURC_BUK_DIR → BURD_BUK_DIR in rbw_workbench.sh (needed for the RBRN dispatch path)

## Acceptance

- rbcnc_cli.sh and rbcnx_cli.sh use sources-in-furnish pattern
- No ../buk/ paths in these two files
- RBRN tabtargets still work (rbw-rnr, rbw-rnv, rbw-ni, rbw-nv)

**[260222-0738] rough**

Move top-level source commands into furnish across all CLIs, using BURD_BUK_DIR/BURD_TOOLS_DIR.

## Context

After burd-export-consolidation (₢AfAAW), BURD_BUK_DIR and BURD_TOOLS_DIR are available in CLI processes. After buc-execute-differential-furnish (₢AfAAX), furnish receives the command name. CLIs can now move all source commands (except buc_command.sh bootstrap) into furnish, eliminating all ../buk/ relative paths.

## Pattern

Before:
```bash
ZFOO_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
source "${ZFOO_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZFOO_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZFOO_CLI_SCRIPT_DIR}/../buk/burd_regime.sh"
source "${ZFOO_CLI_SCRIPT_DIR}/rbcc_Constants.sh"

zfoo_furnish() {
  zbuv_kindle
  zburd_kindle
  zrbcc_kindle
}
```

After:
```bash
source "${BURD_BUK_DIR}/buc_command.sh"

zfoo_furnish() {
  # Sources
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_TOOLS_DIR}/rbw/rbcc_Constants.sh"

  # Kindles
  zbuv_kindle
  zburd_kindle
  zrbcc_kindle
}
```

## Scope

All 22 CLI/workbench files across 6 kits that use ../buk/ paths:
- Tools/rbw/ (14 files): rbcnc, rbcnx, rbrr, rbrp, rbro, rbrs, rbra, rbrv, rbob, rbq, rbq_cli, rbcr, rbw_workbench, rbtb_testbench
- Tools/vok/ (2): vob_cli, vow_workbench
- Tools/vvk/ (2): vvb_cli, vvw_workbench
- Tools/jjk/ (2): jjw_workbench, jja_arcanum
- Tools/vslk/ (1): vslw_workbench
- Tools/cmk/ (1): cmw_workbench

Sources sorted separately from kindles within furnish. SCRIPT_DIR variables eliminated where BURD_TOOLS_DIR suffices.

## Acceptance

- Zero ../buk/ paths remain in CLI files
- All tabtargets still work
- Qualification passes

### rbrn-cli-reunification (₢AfAAZ) [complete]

**[260222-1201] complete**

Merge rbcnx_cli.sh back into rbrn_cli.sh using differential furnish.

## Context

rbcnc_cli.sh and rbcnx_cli.sh were split because survey/audit commands need heavy deps (GCP/OAuth/RBRR) while validate/render/list need only light deps. With differential furnish (₢AfAAX), a single furnish function can conditionally source/kindle based on command name.

## Scope

1. Create unified rbrn_cli.sh containing all commands from both rbcnc and rbcnx:
   - Light: rbrn_validate, rbrn_render (from rbcnc)
   - Heavy: rbrn_survey, rbrn_audit (from rbcnx)
   - New: rbrn_list_nameplates (from ₢AfAAK, may already exist)

2. Furnish uses $1 (command name) to gate heavy sourcing/kindling:
   ```bash
   zrbrn_furnish() {
     local z_command="${1:-}"
     # Light sources (always)
     ...
     # Heavy sources (survey/audit only)
     case "${z_command}" in
       rbrn_survey|rbrn_audit) ... ;;
     esac
     # Light kindles (always)
     ...
     # Heavy kindles (conditional)
     case "${z_command}" in
       rbrn_survey|rbrn_audit) ... ;;
     esac
   }
   ```

3. Update rbz_zipper.sh: change module references from rbcnc_cli.sh/rbcnx_cli.sh to rbrn_cli.sh

4. Delete rbcnc_cli.sh and rbcnx_cli.sh

## Acceptance

- Single rbrn_cli.sh handles all nameplate commands
- Light commands (validate, render) don't load GCP/OAuth deps
- Heavy commands (survey, audit) still work
- All tabtargets function correctly
- Qualification passes

**[260222-0739] rough**

Merge rbcnx_cli.sh back into rbrn_cli.sh using differential furnish.

## Context

rbcnc_cli.sh and rbcnx_cli.sh were split because survey/audit commands need heavy deps (GCP/OAuth/RBRR) while validate/render/list need only light deps. With differential furnish (₢AfAAX), a single furnish function can conditionally source/kindle based on command name.

## Scope

1. Create unified rbrn_cli.sh containing all commands from both rbcnc and rbcnx:
   - Light: rbrn_validate, rbrn_render (from rbcnc)
   - Heavy: rbrn_survey, rbrn_audit (from rbcnx)
   - New: rbrn_list_nameplates (from ₢AfAAK, may already exist)

2. Furnish uses $1 (command name) to gate heavy sourcing/kindling:
   ```bash
   zrbrn_furnish() {
     local z_command="${1:-}"
     # Light sources (always)
     ...
     # Heavy sources (survey/audit only)
     case "${z_command}" in
       rbrn_survey|rbrn_audit) ... ;;
     esac
     # Light kindles (always)
     ...
     # Heavy kindles (conditional)
     case "${z_command}" in
       rbrn_survey|rbrn_audit) ... ;;
     esac
   }
   ```

3. Update rbz_zipper.sh: change module references from rbcnc_cli.sh/rbcnx_cli.sh to rbrn_cli.sh

4. Delete rbcnc_cli.sh and rbcnx_cli.sh

## Acceptance

- Single rbrn_cli.sh handles all nameplate commands
- Light commands (validate, render) don't load GCP/OAuth deps
- Heavy commands (survey, audit) still work
- All tabtargets function correctly
- Qualification passes

### add-nameplate-list-tabtarget (₢AfAAK) [complete]

**[260222-1234] complete**

Add a tabtarget that lists all available nameplates, plumbed through to rbrn_cli.

## Context

This exercises the "no folio → list valid options" graceful path. When a manifold CLI receives no RBR0_FOLIO (or an invalid one), it should list the available instances and exit cleanly WITHOUT calling buv_enforce. This is a presentation-layer concern owned by the CLI.

## Design Rule

The buv_enforce call is gated on having a valid folio. The flow is:
- Furnish kindles buv_, then kindles RBRR (to get RBRR_VESSEL_DIR and nameplate paths)
- If RBR0_FOLIO is empty/missing: list available nameplates, exit 0
- If RBR0_FOLIO is present: resolve file, source, kindle RBRN, buv_enforce RBRN, dispatch command

This establishes the pattern for all manifold CLIs: the "list what's available" path is a first-class CLI feature, not an error.

## Scope

1. Add rbrn_list (or similar) command function in rbrn_cli.sh
2. Create tabtarget (colophon TBD — possibly rbw-nl for nameplate-list)
3. Register in rbz_zipper.sh (singleton channel — no folio needed to list)
4. Wire through workbench dispatch

## Acceptance

- tt/rbw-nl.ListNameplates.sh (or similar) works from terminal
- Lists available nameplate monikers from RBRR_VESSEL_DIR
- Does not require RBR0_FOLIO
- Does not call buv_enforce RBRN
- Qualification passes

**[260219-1909] rough**

Add a tabtarget that lists all available nameplates, plumbed through to rbrn_cli.

## Context

This exercises the "no folio → list valid options" graceful path. When a manifold CLI receives no RBR0_FOLIO (or an invalid one), it should list the available instances and exit cleanly WITHOUT calling buv_enforce. This is a presentation-layer concern owned by the CLI.

## Design Rule

The buv_enforce call is gated on having a valid folio. The flow is:
- Furnish kindles buv_, then kindles RBRR (to get RBRR_VESSEL_DIR and nameplate paths)
- If RBR0_FOLIO is empty/missing: list available nameplates, exit 0
- If RBR0_FOLIO is present: resolve file, source, kindle RBRN, buv_enforce RBRN, dispatch command

This establishes the pattern for all manifold CLIs: the "list what's available" path is a first-class CLI feature, not an error.

## Scope

1. Add rbrn_list (or similar) command function in rbrn_cli.sh
2. Create tabtarget (colophon TBD — possibly rbw-nl for nameplate-list)
3. Register in rbz_zipper.sh (singleton channel — no folio needed to list)
4. Wire through workbench dispatch

## Acceptance

- tt/rbw-nl.ListNameplates.sh (or similar) works from terminal
- Lists available nameplate monikers from RBRR_VESSEL_DIR
- Does not require RBR0_FOLIO
- Does not call buv_enforce RBRN
- Qualification passes

**[260219-0919] rough**

Add a tabtarget that lists all available nameplates, plumbed through to rbrn_cli.

## Context

This exercises the "no folio → list valid options" graceful path. When a manifold CLI receives no RBR0_FOLIO (or an invalid one), it should list the available instances and exit cleanly WITHOUT calling enforce. This is a presentation-layer concern owned by the CLI.

## Design Rule

The enforce call is gated on having a valid folio. The flow is:
- Furnish kindles RBRR (to get RBRR_VESSEL_DIR and nameplate paths)
- If RBR0_FOLIO is empty/missing: list available nameplates, exit 0
- If RBR0_FOLIO is present: resolve file, source, kindle RBRN, enforce, dispatch command

This establishes the pattern for all manifold CLIs: the "list what's available" path is a first-class CLI feature, not an error.

## Scope

1. Add `rbrn_list` (or similar) command function in rbrn_cli.sh
2. Create tabtarget (colophon TBD — possibly `rbw-nl` for nameplate-list)
3. Register in rbz_zipper.sh (singleton channel — no folio needed to list)
4. Wire through workbench dispatch

## Acceptance

- `tt/rbw-nl.ListNameplates.sh` (or similar) works from terminal
- Lists available nameplate monikers from RBRR_VESSEL_DIR
- Does not require RBR0_FOLIO
- Does not call enforce
- Qualification passes

### scrub-rbrv-manifold-structural (₢AfAAI) [complete]

**[260222-1258] complete**

Full BCG scrub of RBRV (Vessel Regime) as a second manifold exemplar with structural repair.

## Why Separate From apply-to-remaining

rbrv_cli.sh has deeper structural debt than other consuming CLIs:
- No furnish() function at all
- No buc_execute — uses ad-hoc case dispatch at module level
- zrbcc_kindle called at module level, outside any furnish
- rbrr_load called conditionally inside case branches
- Direct source and zrbrv_kindle calls inside command functions

This is not a mechanical transform from the checklist — it needs structural redesign.

## Scope

1. **rbrv_cli.sh** — Full restructure to BCG CLI template:
   - Proper zrbrv_furnish() with buc_doc_env
   - buc_execute dispatch replacing the case statement
   - Furnish reads RBR0_FOLIO for vessel selection
   - Call buv_enforce RBRV on paths that need it
   - Validate command calls buv_report RBRV "Vessel Regime"
   - Render and validate as proper buc_doc_* documented commands

2. **rbrv_regime.sh** — Restructure validation using enrollment:
   - Add buv_*_enroll calls in zrbrv_kindle for all RBRV variables
   - Eliminate per-module validate_fields — replaced by buv_enforce/buv_report
   - Fix [[ == ]] sites, unquoted ${#ARRAY[@]}
   - Use gated enrollment where applicable

3. **rbz_zipper.sh** — Declare channel for vessel colophons

## Acceptance

- rbrv_cli.sh uses buc_execute with proper furnish
- No ad-hoc case dispatch or module-level kindle
- All validation via buv_ enrollment
- RBR0_FOLIO flows through buz channel for vessel selection
- All vessel test suites still pass
- Qualification passes

**[260219-1856] rough**

Full BCG scrub of RBRV (Vessel Regime) as a second manifold exemplar with structural repair.

## Why Separate From apply-to-remaining

rbrv_cli.sh has deeper structural debt than other consuming CLIs:
- No furnish() function at all
- No buc_execute — uses ad-hoc case dispatch at module level
- zrbcc_kindle called at module level, outside any furnish
- rbrr_load called conditionally inside case branches
- Direct source and zrbrv_kindle calls inside command functions

This is not a mechanical transform from the checklist — it needs structural redesign.

## Scope

1. **rbrv_cli.sh** — Full restructure to BCG CLI template:
   - Proper zrbrv_furnish() with buc_doc_env
   - buc_execute dispatch replacing the case statement
   - Furnish reads RBR0_FOLIO for vessel selection
   - Call buv_enforce RBRV on paths that need it
   - Validate command calls buv_report RBRV "Vessel Regime"
   - Render and validate as proper buc_doc_* documented commands

2. **rbrv_regime.sh** — Restructure validation using enrollment:
   - Add buv_*_enroll calls in zrbrv_kindle for all RBRV variables
   - Eliminate per-module validate_fields — replaced by buv_enforce/buv_report
   - Fix [[ == ]] sites, unquoted ${#ARRAY[@]}
   - Use gated enrollment where applicable

3. **rbz_zipper.sh** — Declare channel for vessel colophons

## Acceptance

- rbrv_cli.sh uses buc_execute with proper furnish
- No ad-hoc case dispatch or module-level kindle
- All validation via buv_ enrollment
- RBR0_FOLIO flows through buz channel for vessel selection
- All vessel test suites still pass
- Qualification passes

**[260219-0918] rough**

Full BCG scrub of RBRV (Vessel Regime) as a second manifold exemplar with structural repair.

## Why Separate From apply-to-remaining

rbrv_cli.sh has deeper structural debt than other consuming CLIs:
- No `furnish()` function at all
- No `buc_execute` — uses ad-hoc `case` dispatch at module level
- `zrbcc_kindle` called at module level (line 111), outside any furnish
- `rbrr_load` called conditionally inside case branches instead of in furnish
- Direct `source` and `zrbrv_kindle` calls inside command functions (rbrv_validate, rbrv_render)

This is not a mechanical transform from the checklist — it needs structural redesign of the CLI to separate the presentation layer (render, validate commands) from the regime module, and to properly route RBR0_FOLIO for vessel selection.

## Scope

1. **rbrv_cli.sh** — Full restructure to BCG CLI template:
   - Proper `zrbrv_furnish()` with buc_doc_env
   - `buc_execute` dispatch replacing the case statement
   - Furnish reads RBR0_FOLIO for vessel selection
   - Render and validate as proper buc_doc_* documented commands

2. **rbrv_regime.sh** — Clean up BCG violations:
   - Fix `[[ == ]]` sites (lines ~93, ~98 per paddock audit)
   - Fix unquoted `${#ARRAY[@]}` (line ~78)
   - Rename `_validate_fields` to `_enforce`

3. **rbz_zipper.sh** — Declare channel for vessel colophons (rbw-rvr, rbw-rvv)

## Acceptance

- rbrv_cli.sh uses buc_execute with proper furnish
- No ad-hoc case dispatch or module-level kindle
- RBR0_FOLIO flows through buz channel for vessel selection
- All vessel test suites still pass
- Qualification passes

### buv-enrollment-tests (₢AfAAQ) [complete]

**[260222-1328] complete**

Build test suite for buv_ enrollment infrastructure: kindle, enroll, check predicate, enforce, report, gating.

## Organization

7 test case files, grouped by test shape:

### Type-group files (5)

| File | Types | Test shape |
|------|-------|-----------|
| butcev_LengthTypes.sh | string, xname, gname, fqin | empty, length bounds, pattern valid/invalid per type |
| butcev_ChoiceTypes.sh | bool, enum | valid/invalid choices, empty |
| butcev_NumericTypes.sh | decimal | range bounds, non-integer, empty |
| butcev_RefTypes.sh | odref | digest-pinned format, malformed refs, empty |
| butcev_ListTypes.sh | list_string, list_ipv4, list_gname | element validation, empty list, mixed good/bad elements |

### Integration files (2)

| File | Concern |
|------|---------|
| butcev_GateEnroll.sh | gated-in passes, gated-out skips, gate var mismatch |
| butcev_EnforceReport.sh | enforce dies on first bad, report returns status with rich output, multi-scope filtering, mixed pass/fail |

## Test pattern

Each _tcase runs in a subshell (clean state). Every case does its own:
1. zbuv_kindle (init rolls)
2. buv_*_enroll calls (register variables)
3. Set environment variables to test values
4. buv_enforce or buv_report (consume)

Use buto_unit_expect_ok for positive cases, buto_unit_expect_fatal for negative cases (expected failures).

Consider a small helper function to reduce kindle+enroll boilerplate across cases.

## Suite registration

Add an "enrollment-validation" suite to rbtb_testbench.sh:
- Source all 7 butcev_*.sh files
- Add setup function zrbtb_enrollment_tsuite_setup (likely no-op or just trace)
- Enroll all _tcase functions under the suite

## Acceptance

- All positive cases pass (valid data, enforce succeeds)
- All negative cases pass (invalid data, enforce/report correctly rejects)
- Gating: gated-out vars skipped, gated-in vars checked
- Report: shows PASS/FAIL/SKIP per variable, returns correct status
- Suite runs via: ./tt/rbw-ta.TestAll.sh (or rbw-ts enrollment-validation)

**[260220-0833] rough**

Build test suite for buv_ enrollment infrastructure: kindle, enroll, check predicate, enforce, report, gating.

## Organization

7 test case files, grouped by test shape:

### Type-group files (5)

| File | Types | Test shape |
|------|-------|-----------|
| butcev_LengthTypes.sh | string, xname, gname, fqin | empty, length bounds, pattern valid/invalid per type |
| butcev_ChoiceTypes.sh | bool, enum | valid/invalid choices, empty |
| butcev_NumericTypes.sh | decimal | range bounds, non-integer, empty |
| butcev_RefTypes.sh | odref | digest-pinned format, malformed refs, empty |
| butcev_ListTypes.sh | list_string, list_ipv4, list_gname | element validation, empty list, mixed good/bad elements |

### Integration files (2)

| File | Concern |
|------|---------|
| butcev_GateEnroll.sh | gated-in passes, gated-out skips, gate var mismatch |
| butcev_EnforceReport.sh | enforce dies on first bad, report returns status with rich output, multi-scope filtering, mixed pass/fail |

## Test pattern

Each _tcase runs in a subshell (clean state). Every case does its own:
1. zbuv_kindle (init rolls)
2. buv_*_enroll calls (register variables)
3. Set environment variables to test values
4. buv_enforce or buv_report (consume)

Use buto_unit_expect_ok for positive cases, buto_unit_expect_fatal for negative cases (expected failures).

Consider a small helper function to reduce kindle+enroll boilerplate across cases.

## Suite registration

Add an "enrollment-validation" suite to rbtb_testbench.sh:
- Source all 7 butcev_*.sh files
- Add setup function zrbtb_enrollment_tsuite_setup (likely no-op or just trace)
- Enroll all _tcase functions under the suite

## Acceptance

- All positive cases pass (valid data, enforce succeeds)
- All negative cases pass (invalid data, enforce/report correctly rejects)
- Gating: gated-out vars skipped, gated-in vars checked
- Report: shows PASS/FAIL/SKIP per variable, returns correct status
- Suite runs via: ./tt/rbw-ta.TestAll.sh (or rbw-ts enrollment-validation)

### scrub-rbrs-singleton (₢AfAAe) [complete]

**[260222-1440] complete**

Apply T1-T13 transformation recipes to RBRS (Station Regime).

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, RBRS section.

## Current State

- `Tools/rbw/rbrs_regime.sh` (45 lines) — 3 variables, UNEXPECTED array, validation inside kindle
- `Tools/rbw/rbrs_cli.sh` (98 lines) — SCRIPT_DIR pattern, manual case dispatch, rbcr_section_* render

## Variables (3, single group)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| RBRS_PODMAN_ROOT_DIR | buv_string_enroll 1 64 | |
| RBRS_VMIMAGE_CACHE_DIR | buv_string_enroll 1 64 | |
| RBRS_VM_PLATFORM | buv_string_enroll 1 64 | |

## Transformations

- T1: CLI restructure — buc_execute + furnish. Verify RBCC_rbrs_file constant exists (CLI currently hardcodes `../station-files/rbrs.env`).
- T3: buv enrollment — single group "Station Paths", 3 string enrollments
- T3 special: Current kindle calls buv_env_string directly (validation inside kindle). Must split: kindle sets defaults + enrolls, new enforce calls buv_vet.
- T5: buv_report / buv_render replace manual render
- T6: buv_scope_sentinel replaces UNEXPECTED array
- T9: buc_doc_* preamble on validate, render
- T12: furnish receives command name
- T13: bupr replaces rbcr

No T2 (_load), T4 (gating), T7 (rollup), T8 (grep/[[==]]), T10 (channel), T11 (list_capture) needed.

## Verification

1. `./tt/rbw-trg.RegimeSmoke.sh`
2. `./tt/rbw-rsv.ValidateStation.sh` and `./tt/rbw-rsr.RenderStation.sh` (if these tabtargets exist; check first)

## Acceptance

- rbrs_regime.sh uses enrollment + scope_sentinel + enforce
- rbrs_cli.sh uses buc_execute + furnish + buv_report/buv_render
- No manual rbcr_section_* calls
- Regime smoke tests pass

**[260222-1410] rough**

Apply T1-T13 transformation recipes to RBRS (Station Regime).

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, RBRS section.

## Current State

- `Tools/rbw/rbrs_regime.sh` (45 lines) — 3 variables, UNEXPECTED array, validation inside kindle
- `Tools/rbw/rbrs_cli.sh` (98 lines) — SCRIPT_DIR pattern, manual case dispatch, rbcr_section_* render

## Variables (3, single group)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| RBRS_PODMAN_ROOT_DIR | buv_string_enroll 1 64 | |
| RBRS_VMIMAGE_CACHE_DIR | buv_string_enroll 1 64 | |
| RBRS_VM_PLATFORM | buv_string_enroll 1 64 | |

## Transformations

- T1: CLI restructure — buc_execute + furnish. Verify RBCC_rbrs_file constant exists (CLI currently hardcodes `../station-files/rbrs.env`).
- T3: buv enrollment — single group "Station Paths", 3 string enrollments
- T3 special: Current kindle calls buv_env_string directly (validation inside kindle). Must split: kindle sets defaults + enrolls, new enforce calls buv_vet.
- T5: buv_report / buv_render replace manual render
- T6: buv_scope_sentinel replaces UNEXPECTED array
- T9: buc_doc_* preamble on validate, render
- T12: furnish receives command name
- T13: bupr replaces rbcr

No T2 (_load), T4 (gating), T7 (rollup), T8 (grep/[[==]]), T10 (channel), T11 (list_capture) needed.

## Verification

1. `./tt/rbw-trg.RegimeSmoke.sh`
2. `./tt/rbw-rsv.ValidateStation.sh` and `./tt/rbw-rsr.RenderStation.sh` (if these tabtargets exist; check first)

## Acceptance

- rbrs_regime.sh uses enrollment + scope_sentinel + enforce
- rbrs_cli.sh uses buc_execute + furnish + buv_report/buv_render
- No manual rbcr_section_* calls
- Regime smoke tests pass

### scrub-burs-singleton (₢AfAAf) [complete]

**[260222-1448] complete**

Apply T1-T13 transformation recipes to BURS (Station Regime, BUK domain).

## Exemplar

Follow RBRN cli pattern (sources inside furnish via `BURD_BUK_DIR`/`BURD_TOOLS_DIR`), NOT RBRR (which still uses top-level SCRIPT_DIR sourcing). See `Tools/rbw/rbrn_cli.sh` for reference.

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, BURS section.

## Current State

- `Tools/buk/burs_regime.sh` (66 lines) — 1 variable, UNEXPECTED array, separate validate_fields
- `Tools/buk/burs_cli.sh` (108 lines) — SCRIPT_DIR pattern, manual case dispatch, 1 rbcr_section group

## Variables (1, single group)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| BURS_LOG_DIR | buv_string_enroll 1 512 | |

## Transformations

- T1: CLI restructure — BUK-domain. Source `"${BURD_BUK_DIR}/buc_command.sh"`. Furnish sources buv, burd, burs_regime, bupr.
- T3: buv enrollment — single group "Developer Logging", 1 string enrollment
- T5: buv_report / buv_render
- T6: buv_scope_sentinel replaces ZBURS_UNEXPECTED array
- T9: buc_doc_* preamble on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

BURS values are already in environment from burd dispatch — no file to source in furnish, just kindle + enforce.

## Dispatch Update

`buw_workbench.sh` (lines 73-74) dispatches BURS with bare command names:
```
buw-rsv) exec "${z_burs_cli}" validate ;;
buw-rsr) exec "${z_burs_cli}" render ;;
```
After buc_execute restructure, these must become prefixed:
```
buw-rsv) exec "${z_burs_cli}" burs_validate ;;
buw-rsr) exec "${z_burs_cli}" burs_render ;;
```
This is required — buc_execute validates commands match the `burs_` prefix.

## Verification

1. `./tt/rbw-trg.TestRegimeSmoke.sh`

## Acceptance

- burs_regime.sh uses enrollment + scope_sentinel + enforce
- burs_cli.sh uses buc_execute + furnish + buv_report/buv_render
- buw_workbench.sh dispatch uses prefixed command names
- Regime smoke tests pass

**[260222-1425] rough**

Apply T1-T13 transformation recipes to BURS (Station Regime, BUK domain).

## Exemplar

Follow RBRN cli pattern (sources inside furnish via `BURD_BUK_DIR`/`BURD_TOOLS_DIR`), NOT RBRR (which still uses top-level SCRIPT_DIR sourcing). See `Tools/rbw/rbrn_cli.sh` for reference.

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, BURS section.

## Current State

- `Tools/buk/burs_regime.sh` (66 lines) — 1 variable, UNEXPECTED array, separate validate_fields
- `Tools/buk/burs_cli.sh` (108 lines) — SCRIPT_DIR pattern, manual case dispatch, 1 rbcr_section group

## Variables (1, single group)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| BURS_LOG_DIR | buv_string_enroll 1 512 | |

## Transformations

- T1: CLI restructure — BUK-domain. Source `"${BURD_BUK_DIR}/buc_command.sh"`. Furnish sources buv, burd, burs_regime, bupr.
- T3: buv enrollment — single group "Developer Logging", 1 string enrollment
- T5: buv_report / buv_render
- T6: buv_scope_sentinel replaces ZBURS_UNEXPECTED array
- T9: buc_doc_* preamble on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

BURS values are already in environment from burd dispatch — no file to source in furnish, just kindle + enforce.

## Dispatch Update

`buw_workbench.sh` (lines 73-74) dispatches BURS with bare command names:
```
buw-rsv) exec "${z_burs_cli}" validate ;;
buw-rsr) exec "${z_burs_cli}" render ;;
```
After buc_execute restructure, these must become prefixed:
```
buw-rsv) exec "${z_burs_cli}" burs_validate ;;
buw-rsr) exec "${z_burs_cli}" burs_render ;;
```
This is required — buc_execute validates commands match the `burs_` prefix.

## Verification

1. `./tt/rbw-trg.TestRegimeSmoke.sh`

## Acceptance

- burs_regime.sh uses enrollment + scope_sentinel + enforce
- burs_cli.sh uses buc_execute + furnish + buv_report/buv_render
- buw_workbench.sh dispatch uses prefixed command names
- Regime smoke tests pass

**[260222-1410] rough**

Apply T1-T13 transformation recipes to BURS (Station Regime, BUK domain).

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, BURS section.

## Current State

- `Tools/buk/burs_regime.sh` (66 lines) — 1 variable, UNEXPECTED array, separate validate_fields
- `Tools/buk/burs_cli.sh` (108 lines) — SCRIPT_DIR pattern, manual case dispatch, 1 rbcr_section group

## Variables (1, single group)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| BURS_LOG_DIR | buv_string_enroll 1 512 | |

## Transformations

- T1: CLI restructure — BUK-domain. Source `"${BURD_BUK_DIR}/buc_command.sh"`. Furnish sources buv, burd, burs_regime, bupr.
- T3: buv enrollment — single group "Developer Logging", 1 string enrollment
- T5: buv_report / buv_render
- T6: buv_scope_sentinel replaces ZBURS_UNEXPECTED array
- T9: buc_doc_* preamble on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

BURS values are already in environment from burd dispatch — no file to source in furnish, just kindle + enforce.

## Verification

1. `./tt/rbw-trg.RegimeSmoke.sh`

## Acceptance

- burs_regime.sh uses enrollment + scope_sentinel + enforce
- burs_cli.sh uses buc_execute + furnish + buv_report/buv_render
- Regime smoke tests pass

### scrub-rbre-singleton (₢AfAAg) [complete]

**[260222-1449] complete**

Apply T1-T13 transformation recipes to RBRE (ECR Regime).

## Exemplar

Follow RBRN cli pattern (sources inside furnish via `BURD_BUK_DIR`/`BURD_TOOLS_DIR`), NOT RBRR (which still uses top-level SCRIPT_DIR sourcing). See `Tools/rbw/rbrn_cli.sh` for reference.

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, RBRE section.

## Current State

- `Tools/rbw/rbre_regime.sh` (93 lines) — 7 variables, UNEXPECTED array, separate validate_fields, rbre_load()
- `Tools/rbw/rbre_cli.sh` (106 lines) — SCRIPT_DIR pattern, manual case dispatch, 2 rbcr_section groups

## Variables (7, two groups)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| RBRE_AWS_ACCOUNT_ID | buv_string_enroll req | |
| RBRE_AWS_REGION | buv_string_enroll req | |
| RBRE_ECR_REGISTRY | buv_string_enroll req | |
| RBRE_ECR_REPOSITORY_PREFIX | buv_string_enroll req | |
| RBRE_AWS_ACCESS_KEY_ID | buv_string_enroll req | |
| RBRE_AWS_SECRET_ACCESS_KEY | buv_string_enroll req | |
| RBRE_AWS_SESSION_TOKEN | buv_string_enroll 0 (opt) | Optional STS token |

Groups: "ECR Identity" (first 4), "ECR Access" (last 3).

## RBCC Constant

No `RBCC_rbre_file` constant exists in `rbcc_Constants.sh`. Must add it before furnish can reference it. Determine the correct env file path (check existing rbre dispatch or test code for the current hardcoded path).

## Transformations

- T1: CLI restructure — buc_execute + furnish. Furnish sources buv, burd, rbcc, rbre_regime, bupr, then `source "${RBCC_rbre_file}"` + kindle + enforce.
- T2: Delete `rbre_load()` from regime module. Furnish inlines source + kindle + enforce.
- T3: buv enrollment — 7 variables, 2 groups, all buv_string_enroll. SESSION_TOKEN uses min=0.
- T5: buv_report / buv_render
- T6: buv_scope_sentinel replaces ZRBRE_UNEXPECTED
- T9: buc_doc_* on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

No T4 (gating), T7 (rollup), T8 (no grep or [[ == ]]), T10 (channel), T11 (list) needed.

Caution: RBRE contains AWS secrets. Render displaying them is existing behavior — no change in security posture.

## Dispatch Update

RBRE has NO zipper entry in `rbz_zipper.sh` and NO workbench dispatch in `buw_workbench.sh`. It has no dedicated tabtargets either. Check how RBRE is currently invoked — likely only from test code (`butcrg_RegimeSmoke.sh` or similar) or direct CLI invocation. The buc_execute restructure is self-contained but verify no caller passes bare `validate`/`render`.

## Verification

1. `./tt/rbw-trg.TestRegimeSmoke.sh`

## Acceptance

- rbre_regime.sh uses enrollment + scope_sentinel + enforce, no rbre_load()
- rbre_cli.sh uses buc_execute + furnish + buv_report/buv_render
- RBCC_rbre_file constant added to rbcc_Constants.sh
- Regime smoke tests pass

**[260222-1425] rough**

Apply T1-T13 transformation recipes to RBRE (ECR Regime).

## Exemplar

Follow RBRN cli pattern (sources inside furnish via `BURD_BUK_DIR`/`BURD_TOOLS_DIR`), NOT RBRR (which still uses top-level SCRIPT_DIR sourcing). See `Tools/rbw/rbrn_cli.sh` for reference.

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, RBRE section.

## Current State

- `Tools/rbw/rbre_regime.sh` (93 lines) — 7 variables, UNEXPECTED array, separate validate_fields, rbre_load()
- `Tools/rbw/rbre_cli.sh` (106 lines) — SCRIPT_DIR pattern, manual case dispatch, 2 rbcr_section groups

## Variables (7, two groups)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| RBRE_AWS_ACCOUNT_ID | buv_string_enroll req | |
| RBRE_AWS_REGION | buv_string_enroll req | |
| RBRE_ECR_REGISTRY | buv_string_enroll req | |
| RBRE_ECR_REPOSITORY_PREFIX | buv_string_enroll req | |
| RBRE_AWS_ACCESS_KEY_ID | buv_string_enroll req | |
| RBRE_AWS_SECRET_ACCESS_KEY | buv_string_enroll req | |
| RBRE_AWS_SESSION_TOKEN | buv_string_enroll 0 (opt) | Optional STS token |

Groups: "ECR Identity" (first 4), "ECR Access" (last 3).

## RBCC Constant

No `RBCC_rbre_file` constant exists in `rbcc_Constants.sh`. Must add it before furnish can reference it. Determine the correct env file path (check existing rbre dispatch or test code for the current hardcoded path).

## Transformations

- T1: CLI restructure — buc_execute + furnish. Furnish sources buv, burd, rbcc, rbre_regime, bupr, then `source "${RBCC_rbre_file}"` + kindle + enforce.
- T2: Delete `rbre_load()` from regime module. Furnish inlines source + kindle + enforce.
- T3: buv enrollment — 7 variables, 2 groups, all buv_string_enroll. SESSION_TOKEN uses min=0.
- T5: buv_report / buv_render
- T6: buv_scope_sentinel replaces ZRBRE_UNEXPECTED
- T9: buc_doc_* on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

No T4 (gating), T7 (rollup), T8 (no grep or [[ == ]]), T10 (channel), T11 (list) needed.

Caution: RBRE contains AWS secrets. Render displaying them is existing behavior — no change in security posture.

## Dispatch Update

RBRE has NO zipper entry in `rbz_zipper.sh` and NO workbench dispatch in `buw_workbench.sh`. It has no dedicated tabtargets either. Check how RBRE is currently invoked — likely only from test code (`butcrg_RegimeSmoke.sh` or similar) or direct CLI invocation. The buc_execute restructure is self-contained but verify no caller passes bare `validate`/`render`.

## Verification

1. `./tt/rbw-trg.TestRegimeSmoke.sh`

## Acceptance

- rbre_regime.sh uses enrollment + scope_sentinel + enforce, no rbre_load()
- rbre_cli.sh uses buc_execute + furnish + buv_report/buv_render
- RBCC_rbre_file constant added to rbcc_Constants.sh
- Regime smoke tests pass

**[260222-1410] rough**

Apply T1-T13 transformation recipes to RBRE (ECR Regime).

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, RBRE section.

## Current State

- `Tools/rbw/rbre_regime.sh` (93 lines) — 7 variables, UNEXPECTED array, separate validate_fields, rbre_load()
- `Tools/rbw/rbre_cli.sh` (106 lines) — SCRIPT_DIR pattern, manual case dispatch, 2 rbcr_section groups

## Variables (7, two groups)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| RBRE_AWS_ACCOUNT_ID | buv_string_enroll req | |
| RBRE_AWS_REGION | buv_string_enroll req | |
| RBRE_ECR_REGISTRY | buv_string_enroll req | |
| RBRE_ECR_REPOSITORY_PREFIX | buv_string_enroll req | |
| RBRE_AWS_ACCESS_KEY_ID | buv_string_enroll req | |
| RBRE_AWS_SECRET_ACCESS_KEY | buv_string_enroll req | |
| RBRE_AWS_SESSION_TOKEN | buv_string_enroll 0 (opt) | Optional STS token |

Groups: "ECR Identity" (first 4), "ECR Access" (last 3).

## Transformations

- T1: CLI restructure — buc_execute + furnish. Furnish sources buv, burd, rbcc, rbre_regime, bupr, then `source "${RBCC_rbre_file}"` + kindle + enforce.
- T2: Delete `rbre_load()` from regime module. Furnish inlines source + kindle + enforce.
- T3: buv enrollment — 7 variables, 2 groups, all buv_string_enroll. SESSION_TOKEN uses min=0.
- T5: buv_report / buv_render
- T6: buv_scope_sentinel replaces ZRBRE_UNEXPECTED
- T9: buc_doc_* on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

No T4 (gating), T7 (rollup), T8 (no grep or [[ == ]]), T10 (channel), T11 (list) needed.

Caution: RBRE contains AWS secrets. Render displaying them is existing behavior — no change in security posture.

## Verification

1. `./tt/rbw-trg.RegimeSmoke.sh`

## Acceptance

- rbre_regime.sh uses enrollment + scope_sentinel + enforce, no rbre_load()
- rbre_cli.sh uses buc_execute + furnish + buv_report/buv_render
- Regime smoke tests pass

### scrub-rbrp-singleton (₢AfAAh) [complete]

**[260222-1453] complete**

Apply T1-T13 transformation recipes to RBRP (Payor Regime).

## Exemplar

Follow RBRN cli pattern (sources inside furnish via `BURD_BUK_DIR`/`BURD_TOOLS_DIR`), NOT RBRR (which still uses top-level SCRIPT_DIR sourcing). See `Tools/rbw/rbrn_cli.sh` for reference.

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, RBRP section.

## Current State

- `Tools/rbw/rbrp_regime.sh` (75 lines) — 4 variables, validation inline in kindle via grep -qE, rbrp_load()
- `Tools/rbw/rbrp_cli.sh` (112 lines) — SCRIPT_DIR pattern, manual case dispatch, 3 rbcr_section groups

## Variables (4 total)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| RBRP_PAYOR_PROJECT_ID | buv_string_enroll 1 128 req | Custom enforce: must match RBGC_GLOBAL_PAYOR_REGEX |
| RBRP_BILLING_ACCOUNT_ID | buv_string_enroll 0 18 opt | Custom enforce: if non-empty, ^[A-Z0-9]{6}-[A-Z0-9]{6}-[A-Z0-9]{6}$ |
| RBRP_OAUTH_CLIENT_ID | buv_string_enroll 0 256 opt | Custom enforce: if non-empty, must end .apps.googleusercontent.com |
| RBRP_OAUTH_REDIRECT_URI | buv_string_enroll 0 512 opt | Currently in render but NOT validated — enroll it |

Groups: "Payor Project Identity" (PROJECT_ID), "Billing Configuration" (BILLING_ACCOUNT_ID), "OAuth Configuration" (CLIENT_ID, REDIRECT_URI).

## Transformations

- T1: CLI restructure — buc_execute + furnish. Furnish sources buv, burd, rbcc, rbgc, rbrp_regime, bupr, then `source "${RBCC_rbrp_file}"` + kindle + enforce.
- T2: Delete `rbrp_load()` from regime module.
- T3: buv enrollment — 4 variables, 3 groups. Optionals use min=0.
- T5: buv_report / buv_render
- T6: buv_scope_sentinel (no UNEXPECTED array currently — this is new)
- T8b: Replace 3 `printf | grep -qE` sites with [[ =~ ]] in zrbrp_enforce():
  ```bash
  zrbrp_enforce() {
    zrbrp_sentinel
    buv_vet RBRP
    zrbgc_sentinel
    [[ "${RBRP_PAYOR_PROJECT_ID}" =~ ${RBGC_GLOBAL_PAYOR_REGEX} ]] \
      || buc_die "RBRP_PAYOR_PROJECT_ID format invalid"
    if test -n "${RBRP_BILLING_ACCOUNT_ID}"; then
      [[ "${RBRP_BILLING_ACCOUNT_ID}" =~ ^[A-Z0-9]{6}-[A-Z0-9]{6}-[A-Z0-9]{6}$ ]] \
        || buc_die "RBRP_BILLING_ACCOUNT_ID format invalid"
    fi
    if test -n "${RBRP_OAUTH_CLIENT_ID}"; then
      [[ "${RBRP_OAUTH_CLIENT_ID}" =~ \.apps\.googleusercontent\.com$ ]] \
        || buc_die "RBRP_OAUTH_CLIENT_ID format invalid"
    fi
  }
  ```
- T9: buc_doc_* on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

**Dependency:** RBGC must be kindled before enforce (provides RBGC_GLOBAL_PAYOR_REGEX). Furnish must kindle rbgc before rbrp.

## Dispatch Update

`rbz_zipper.sh` (lines 94-96) dispatches RBRP with bare command names:
```
buz_enroll RBZ_RENDER_PAYOR   "rbw-rpr" "${z_mod}" "render"
buz_enroll RBZ_VALIDATE_PAYOR "rbw-rpv" "${z_mod}" "validate"
```
After buc_execute restructure, these must become prefixed:
```
buz_enroll RBZ_RENDER_PAYOR   "rbw-rpr" "${z_mod}" "rbrp_render"
buz_enroll RBZ_VALIDATE_PAYOR "rbw-rpv" "${z_mod}" "rbrp_validate"
```
This is required — buc_execute validates commands match the `rbrp_` prefix.

## Verification

1. `./tt/rbw-trg.TestRegimeSmoke.sh`
2. `./tt/rbw-rpv.ValidatePayor.sh` and `./tt/rbw-rpr.RenderPayor.sh`

## Acceptance

- rbrp_regime.sh uses enrollment + scope_sentinel + enforce with [[ =~ ]] custom checks
- rbrp_cli.sh uses buc_execute + furnish + buv_report/buv_render
- rbz_zipper.sh dispatch uses prefixed command names
- No grep -qE in regime module
- RBRP_OAUTH_REDIRECT_URI is now enrolled (was missing from validation)
- Regime smoke tests pass

**[260222-1432] rough**

Apply T1-T13 transformation recipes to RBRP (Payor Regime).

## Exemplar

Follow RBRN cli pattern (sources inside furnish via `BURD_BUK_DIR`/`BURD_TOOLS_DIR`), NOT RBRR (which still uses top-level SCRIPT_DIR sourcing). See `Tools/rbw/rbrn_cli.sh` for reference.

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, RBRP section.

## Current State

- `Tools/rbw/rbrp_regime.sh` (75 lines) — 4 variables, validation inline in kindle via grep -qE, rbrp_load()
- `Tools/rbw/rbrp_cli.sh` (112 lines) — SCRIPT_DIR pattern, manual case dispatch, 3 rbcr_section groups

## Variables (4 total)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| RBRP_PAYOR_PROJECT_ID | buv_string_enroll 1 128 req | Custom enforce: must match RBGC_GLOBAL_PAYOR_REGEX |
| RBRP_BILLING_ACCOUNT_ID | buv_string_enroll 0 18 opt | Custom enforce: if non-empty, ^[A-Z0-9]{6}-[A-Z0-9]{6}-[A-Z0-9]{6}$ |
| RBRP_OAUTH_CLIENT_ID | buv_string_enroll 0 256 opt | Custom enforce: if non-empty, must end .apps.googleusercontent.com |
| RBRP_OAUTH_REDIRECT_URI | buv_string_enroll 0 512 opt | Currently in render but NOT validated — enroll it |

Groups: "Payor Project Identity" (PROJECT_ID), "Billing Configuration" (BILLING_ACCOUNT_ID), "OAuth Configuration" (CLIENT_ID, REDIRECT_URI).

## Transformations

- T1: CLI restructure — buc_execute + furnish. Furnish sources buv, burd, rbcc, rbgc, rbrp_regime, bupr, then `source "${RBCC_rbrp_file}"` + kindle + enforce.
- T2: Delete `rbrp_load()` from regime module.
- T3: buv enrollment — 4 variables, 3 groups. Optionals use min=0.
- T5: buv_report / buv_render
- T6: buv_scope_sentinel (no UNEXPECTED array currently — this is new)
- T8b: Replace 3 `printf | grep -qE` sites with [[ =~ ]] in zrbrp_enforce():
  ```bash
  zrbrp_enforce() {
    zrbrp_sentinel
    buv_vet RBRP
    zrbgc_sentinel
    [[ "${RBRP_PAYOR_PROJECT_ID}" =~ ${RBGC_GLOBAL_PAYOR_REGEX} ]] \
      || buc_die "RBRP_PAYOR_PROJECT_ID format invalid"
    if test -n "${RBRP_BILLING_ACCOUNT_ID}"; then
      [[ "${RBRP_BILLING_ACCOUNT_ID}" =~ ^[A-Z0-9]{6}-[A-Z0-9]{6}-[A-Z0-9]{6}$ ]] \
        || buc_die "RBRP_BILLING_ACCOUNT_ID format invalid"
    fi
    if test -n "${RBRP_OAUTH_CLIENT_ID}"; then
      [[ "${RBRP_OAUTH_CLIENT_ID}" =~ \.apps\.googleusercontent\.com$ ]] \
        || buc_die "RBRP_OAUTH_CLIENT_ID format invalid"
    fi
  }
  ```
- T9: buc_doc_* on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

**Dependency:** RBGC must be kindled before enforce (provides RBGC_GLOBAL_PAYOR_REGEX). Furnish must kindle rbgc before rbrp.

## Dispatch Update

`rbz_zipper.sh` (lines 94-96) dispatches RBRP with bare command names:
```
buz_enroll RBZ_RENDER_PAYOR   "rbw-rpr" "${z_mod}" "render"
buz_enroll RBZ_VALIDATE_PAYOR "rbw-rpv" "${z_mod}" "validate"
```
After buc_execute restructure, these must become prefixed:
```
buz_enroll RBZ_RENDER_PAYOR   "rbw-rpr" "${z_mod}" "rbrp_render"
buz_enroll RBZ_VALIDATE_PAYOR "rbw-rpv" "${z_mod}" "rbrp_validate"
```
This is required — buc_execute validates commands match the `rbrp_` prefix.

## Verification

1. `./tt/rbw-trg.TestRegimeSmoke.sh`
2. `./tt/rbw-rpv.ValidatePayor.sh` and `./tt/rbw-rpr.RenderPayor.sh`

## Acceptance

- rbrp_regime.sh uses enrollment + scope_sentinel + enforce with [[ =~ ]] custom checks
- rbrp_cli.sh uses buc_execute + furnish + buv_report/buv_render
- rbz_zipper.sh dispatch uses prefixed command names
- No grep -qE in regime module
- RBRP_OAUTH_REDIRECT_URI is now enrolled (was missing from validation)
- Regime smoke tests pass

**[260222-1410] rough**

Apply T1-T13 transformation recipes to RBRP (Payor Regime).

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, RBRP section.

## Current State

- `Tools/rbw/rbrp_regime.sh` (75 lines) — 4 variables, validation inline in kindle via grep -qE, rbrp_load()
- `Tools/rbw/rbrp_cli.sh` (112 lines) — SCRIPT_DIR pattern, manual case dispatch, 3 rbcr_section groups

## Variables (4 total)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| RBRP_PAYOR_PROJECT_ID | buv_string_enroll 1 128 req | Custom enforce: must match RBGC_GLOBAL_PAYOR_REGEX |
| RBRP_BILLING_ACCOUNT_ID | buv_string_enroll 0 18 opt | Custom enforce: if non-empty, ^[A-Z0-9]{6}-[A-Z0-9]{6}-[A-Z0-9]{6}$ |
| RBRP_OAUTH_CLIENT_ID | buv_string_enroll 0 256 opt | Custom enforce: if non-empty, must end .apps.googleusercontent.com |
| RBRP_OAUTH_REDIRECT_URI | buv_string_enroll 0 512 opt | Currently in render but NOT validated — enroll it |

Groups: "Payor Project Identity" (PROJECT_ID), "Billing Configuration" (BILLING_ACCOUNT_ID), "OAuth Configuration" (CLIENT_ID, REDIRECT_URI).

## Transformations

- T1: CLI restructure — buc_execute + furnish. Furnish sources buv, burd, rbcc, rbgc, rbrp_regime, bupr, then `source "${RBCC_rbrp_file}"` + kindle + enforce.
- T2: Delete `rbrp_load()` from regime module.
- T3: buv enrollment — 4 variables, 3 groups. Optionals use min=0.
- T5: buv_report / buv_render
- T6: buv_scope_sentinel (no UNEXPECTED array currently — this is new)
- T8b: Replace 3 `printf | grep -qE` sites with [[ =~ ]] in zrbrp_enforce():
  ```bash
  zrbrp_enforce() {
    zrbrp_sentinel
    buv_vet RBRP
    zrbgc_sentinel
    [[ "${RBRP_PAYOR_PROJECT_ID}" =~ ${RBGC_GLOBAL_PAYOR_REGEX} ]] \
      || buc_die "RBRP_PAYOR_PROJECT_ID format invalid"
    if test -n "${RBRP_BILLING_ACCOUNT_ID}"; then
      [[ "${RBRP_BILLING_ACCOUNT_ID}" =~ ^[A-Z0-9]{6}-[A-Z0-9]{6}-[A-Z0-9]{6}$ ]] \
        || buc_die "RBRP_BILLING_ACCOUNT_ID format invalid"
    fi
    if test -n "${RBRP_OAUTH_CLIENT_ID}"; then
      [[ "${RBRP_OAUTH_CLIENT_ID}" =~ \.apps\.googleusercontent\.com$ ]] \
        || buc_die "RBRP_OAUTH_CLIENT_ID format invalid"
    fi
  }
  ```
- T9: buc_doc_* on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

**Dependency:** RBGC must be kindled before enforce (provides RBGC_GLOBAL_PAYOR_REGEX). Furnish must kindle rbgc before rbrp.

## Verification

1. `./tt/rbw-trg.RegimeSmoke.sh`
2. `./tt/rbw-rpv.ValidatePayor.sh` and `./tt/rbw-rpr.RenderPayor.sh`

## Acceptance

- rbrp_regime.sh uses enrollment + scope_sentinel + enforce with [[ =~ ]] custom checks
- rbrp_cli.sh uses buc_execute + furnish + buv_report/buv_render
- No grep -qE in regime module
- RBRP_OAUTH_REDIRECT_URI is now enrolled (was missing from validation)
- Regime smoke tests pass

### scrub-burc-singleton (₢AfAAi) [complete]

**[260222-1459] complete**

Apply T1-T13 transformation recipes to BURC (Configuration Regime, BUK domain).

## Exemplar

Follow RBRN cli pattern (sources inside furnish via `BURD_BUK_DIR`/`BURD_TOOLS_DIR`), NOT RBRR (which still uses top-level SCRIPT_DIR sourcing). See `Tools/rbw/rbrn_cli.sh` for reference.

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, BURC section.

## Current State

- `Tools/buk/burc_regime.sh` (90 lines) — 11 variables (9 validated + 2 derived), UNEXPECTED array, exports 3 vars
- `Tools/buk/burc_cli.sh` (133 lines) — SCRIPT_DIR pattern, manual case dispatch, 5 rbcr_section groups

## Variables (11 total, 5 groups)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| BURC_STATION_FILE | buv_string_enroll 1 512 | |
| BURC_TABTARGET_DIR | buv_string_enroll 1 128 | Exported |
| BURC_TABTARGET_DELIMITER | buv_string_enroll 1 1 | Single char |
| BURC_TOOLS_DIR | buv_string_enroll 1 128 | Exported |
| BURC_PROJECT_ROOT | buv_string_enroll 1 512 | |
| BURC_MANAGED_KITS | buv_string_enroll 1 512 | |
| BURC_TEMP_ROOT_DIR | buv_string_enroll 1 512 | |
| BURC_OUTPUT_ROOT_DIR | buv_string_enroll 1 512 | |
| BURC_LOG_LAST | buv_xname_enroll 1 64 | |
| BURC_LOG_EXT | buv_xname_enroll 1 16 | |
| BURC_BUK_DIR | DERIVED | = ${BURC_TOOLS_DIR}/buk, exported |

Groups: "Station Reference", "Tabtarget Infrastructure", "Project Structure", "Build Output", "Logging".

## Design Decision: BURC_BUK_DIR

`BURC_BUK_DIR` is derived (`${BURC_TOOLS_DIR}/buk`), not user-configured, but has the `BURC_` prefix. `buv_scope_sentinel` would flag it as unexpected unless handled.

**Decision:** Enroll BURC_BUK_DIR as `buv_string_enroll BURC_BUK_DIR 1 256 "Derived: BUK directory"` in the Logging group (or a new "Derived Paths" group). It will be set in kindle before enrollment. This way scope_sentinel sees it as known, and render/report display it. The value is always derived — the enrollment just acknowledges its existence.

The `export` lines for BURC_TABTARGET_DIR, BURC_TOOLS_DIR, BURC_BUK_DIR must be preserved in kindle after enrollment. Enrollment doesn't handle export.

## Transformations

- T1: CLI restructure — BUK-domain. `source "${BURD_BUK_DIR}/buc_command.sh"`. Furnish sources buv, burd, burc_regime, bupr. BURC values already in environment from burd dispatch.
- T3: buv enrollment — 11 variables including derived BURC_BUK_DIR, 5-6 groups
- T5: buv_report / buv_render
- T6: buv_scope_sentinel replaces ZBURC_UNEXPECTED
- T9: buc_doc_* on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

## Dispatch Update

`buw_workbench.sh` (lines 69-70) dispatches BURC with bare command names:
```
buw-rcv) exec "${z_burc_cli}" validate ;;
buw-rcr) exec "${z_burc_cli}" render ;;
```
After buc_execute restructure, these must become prefixed:
```
buw-rcv) exec "${z_burc_cli}" burc_validate ;;
buw-rcr) exec "${z_burc_cli}" burc_render ;;
```
This is required — buc_execute validates commands match the `burc_` prefix.

## Verification

1. `./tt/rbw-trg.TestRegimeSmoke.sh`
2. `./tt/buw-rcv.ValidateConfig.sh` (if exists)

## Acceptance

- burc_regime.sh uses enrollment + scope_sentinel + enforce
- BURC_BUK_DIR enrolled as derived, export preserved
- burc_cli.sh uses buc_execute + furnish + buv_report/buv_render
- buw_workbench.sh dispatch uses prefixed command names
- Regime smoke tests pass

**[260222-1439] rough**

Apply T1-T13 transformation recipes to BURC (Configuration Regime, BUK domain).

## Exemplar

Follow RBRN cli pattern (sources inside furnish via `BURD_BUK_DIR`/`BURD_TOOLS_DIR`), NOT RBRR (which still uses top-level SCRIPT_DIR sourcing). See `Tools/rbw/rbrn_cli.sh` for reference.

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, BURC section.

## Current State

- `Tools/buk/burc_regime.sh` (90 lines) — 11 variables (9 validated + 2 derived), UNEXPECTED array, exports 3 vars
- `Tools/buk/burc_cli.sh` (133 lines) — SCRIPT_DIR pattern, manual case dispatch, 5 rbcr_section groups

## Variables (11 total, 5 groups)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| BURC_STATION_FILE | buv_string_enroll 1 512 | |
| BURC_TABTARGET_DIR | buv_string_enroll 1 128 | Exported |
| BURC_TABTARGET_DELIMITER | buv_string_enroll 1 1 | Single char |
| BURC_TOOLS_DIR | buv_string_enroll 1 128 | Exported |
| BURC_PROJECT_ROOT | buv_string_enroll 1 512 | |
| BURC_MANAGED_KITS | buv_string_enroll 1 512 | |
| BURC_TEMP_ROOT_DIR | buv_string_enroll 1 512 | |
| BURC_OUTPUT_ROOT_DIR | buv_string_enroll 1 512 | |
| BURC_LOG_LAST | buv_xname_enroll 1 64 | |
| BURC_LOG_EXT | buv_xname_enroll 1 16 | |
| BURC_BUK_DIR | DERIVED | = ${BURC_TOOLS_DIR}/buk, exported |

Groups: "Station Reference", "Tabtarget Infrastructure", "Project Structure", "Build Output", "Logging".

## Design Decision: BURC_BUK_DIR

`BURC_BUK_DIR` is derived (`${BURC_TOOLS_DIR}/buk`), not user-configured, but has the `BURC_` prefix. `buv_scope_sentinel` would flag it as unexpected unless handled.

**Decision:** Enroll BURC_BUK_DIR as `buv_string_enroll BURC_BUK_DIR 1 256 "Derived: BUK directory"` in the Logging group (or a new "Derived Paths" group). It will be set in kindle before enrollment. This way scope_sentinel sees it as known, and render/report display it. The value is always derived — the enrollment just acknowledges its existence.

The `export` lines for BURC_TABTARGET_DIR, BURC_TOOLS_DIR, BURC_BUK_DIR must be preserved in kindle after enrollment. Enrollment doesn't handle export.

## Transformations

- T1: CLI restructure — BUK-domain. `source "${BURD_BUK_DIR}/buc_command.sh"`. Furnish sources buv, burd, burc_regime, bupr. BURC values already in environment from burd dispatch.
- T3: buv enrollment — 11 variables including derived BURC_BUK_DIR, 5-6 groups
- T5: buv_report / buv_render
- T6: buv_scope_sentinel replaces ZBURC_UNEXPECTED
- T9: buc_doc_* on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

## Dispatch Update

`buw_workbench.sh` (lines 69-70) dispatches BURC with bare command names:
```
buw-rcv) exec "${z_burc_cli}" validate ;;
buw-rcr) exec "${z_burc_cli}" render ;;
```
After buc_execute restructure, these must become prefixed:
```
buw-rcv) exec "${z_burc_cli}" burc_validate ;;
buw-rcr) exec "${z_burc_cli}" burc_render ;;
```
This is required — buc_execute validates commands match the `burc_` prefix.

## Verification

1. `./tt/rbw-trg.TestRegimeSmoke.sh`
2. `./tt/buw-rcv.ValidateConfig.sh` (if exists)

## Acceptance

- burc_regime.sh uses enrollment + scope_sentinel + enforce
- BURC_BUK_DIR enrolled as derived, export preserved
- burc_cli.sh uses buc_execute + furnish + buv_report/buv_render
- buw_workbench.sh dispatch uses prefixed command names
- Regime smoke tests pass

**[260222-1410] rough**

Apply T1-T13 transformation recipes to BURC (Configuration Regime, BUK domain).

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, BURC section.

## Current State

- `Tools/buk/burc_regime.sh` (90 lines) — 11 variables (9 validated + 2 derived), UNEXPECTED array, exports 3 vars
- `Tools/buk/burc_cli.sh` (133 lines) — SCRIPT_DIR pattern, manual case dispatch, 5 rbcr_section groups

## Variables (11 total, 5 groups)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| BURC_STATION_FILE | buv_string_enroll 1 512 | |
| BURC_TABTARGET_DIR | buv_string_enroll 1 128 | Exported |
| BURC_TABTARGET_DELIMITER | buv_string_enroll 1 1 | Single char |
| BURC_TOOLS_DIR | buv_string_enroll 1 128 | Exported |
| BURC_PROJECT_ROOT | buv_string_enroll 1 512 | |
| BURC_MANAGED_KITS | buv_string_enroll 1 512 | |
| BURC_TEMP_ROOT_DIR | buv_string_enroll 1 512 | |
| BURC_OUTPUT_ROOT_DIR | buv_string_enroll 1 512 | |
| BURC_LOG_LAST | buv_xname_enroll 1 64 | |
| BURC_LOG_EXT | buv_xname_enroll 1 16 | |
| BURC_BUK_DIR | DERIVED | = ${BURC_TOOLS_DIR}/buk, exported |

Groups: "Station Reference", "Tabtarget Infrastructure", "Project Structure", "Build Output", "Logging".

## Design Decision: BURC_BUK_DIR

`BURC_BUK_DIR` is derived (`${BURC_TOOLS_DIR}/buk`), not user-configured, but has the `BURC_` prefix. `buv_scope_sentinel` would flag it as unexpected unless handled.

**Decision:** Enroll BURC_BUK_DIR as `buv_string_enroll BURC_BUK_DIR 1 256 "Derived: BUK directory"` in the Logging group (or a new "Derived Paths" group). It will be set in kindle before enrollment. This way scope_sentinel sees it as known, and render/report display it. The value is always derived — the enrollment just acknowledges its existence.

The `export` lines for BURC_TABTARGET_DIR, BURC_TOOLS_DIR, BURC_BUK_DIR must be preserved in kindle after enrollment. Enrollment doesn't handle export.

## Transformations

- T1: CLI restructure — BUK-domain. `source "${BURD_BUK_DIR}/buc_command.sh"`. Furnish sources buv, burd, burc_regime, bupr. BURC values already in environment from burd dispatch.
- T3: buv enrollment — 11 variables including derived BURC_BUK_DIR, 5-6 groups
- T5: buv_report / buv_render
- T6: buv_scope_sentinel replaces ZBURC_UNEXPECTED
- T9: buc_doc_* on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

## Verification

1. `./tt/rbw-trg.RegimeSmoke.sh`
2. `./tt/buw-rcv.ValidateConfig.sh` (if exists)

## Acceptance

- burc_regime.sh uses enrollment + scope_sentinel + enforce
- BURC_BUK_DIR enrolled as derived, export preserved
- burc_cli.sh uses buc_execute + furnish + buv_report/buv_render
- Regime smoke tests pass

### scrub-bure-ambient (₢AfAAj) [complete]

**[260222-1502] complete**

Apply T1-T13 transformation recipes to BURE (Environment Regime, BUK domain, ambient).

## Exemplar

Follow RBRN cli pattern (sources inside furnish via `BURD_BUK_DIR`/`BURD_TOOLS_DIR`), NOT RBRR (which still uses top-level SCRIPT_DIR sourcing). See `Tools/rbw/rbrn_cli.sh` for reference.

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, BURE section.

## Current State

- `Tools/buk/bure_regime.sh` (73 lines) — 3 variables, UNEXPECTED array, uses buv_opt_enum and buv_env_enum
- `Tools/buk/bure_cli.sh` (94 lines) — SCRIPT_DIR pattern, manual case dispatch, 1 rbcr_section group

## Variables (3 total, 1 group)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| BURE_COUNTDOWN | buv_string_enroll 0 4 opt | Custom enforce: if non-empty, must be "skip" |
| BURE_VERBOSE | buv_enum_enroll req | Values: 0 1 2 3 |
| BURE_COLOR | buv_string_enroll 1 4 req | Check if valid values are "auto", "on", "off" — if so, use buv_enum_enroll instead |

Group: "Behavioral Overrides".

## Design Decision: Optional Enum (BURE_COUNTDOWN)

The enrollment system has no `buv_opt_enum`. Currently validated by `buv_opt_enum BURE_COUNTDOWN skip`.

**Decision:** Enroll as `buv_string_enroll BURE_COUNTDOWN 0 4 "Countdown override (skip to disable)"`. The min=0 makes it optional. Add custom enforce:
```bash
if test -n "${BURE_COUNTDOWN}"; then
  test "${BURE_COUNTDOWN}" = "skip" \
    || buc_die "BURE_COUNTDOWN must be 'skip' or empty, got '${BURE_COUNTDOWN}'"
fi
```

## Transformations

- T1: CLI restructure — BUK-domain ambient. `source "${BURD_BUK_DIR}/buc_command.sh"`. Furnish sources buv, burd, bure_regime, bupr. No env file to source — BURE is ambient (caller environment). Furnish just calls kindle + enforce.
- T3: buv enrollment — 3 variables, 1 group. See design decision above for COUNTDOWN.
- T5: buv_report / buv_render. Render header should still say "ambient (caller environment)" — handle in furnish or command function.
- T6: buv_scope_sentinel replaces ZBURE_UNEXPECTED
- T9: buc_doc_* on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

## Dispatch Update

`buw_workbench.sh` (lines 77-78) dispatches BURE with bare command names:
```
buw-rev) exec "${z_bure_cli}" validate ;;
buw-rer) exec "${z_bure_cli}" render ;;
```
After buc_execute restructure, these must become prefixed:
```
buw-rev) exec "${z_bure_cli}" bure_validate ;;
buw-rer) exec "${z_bure_cli}" bure_render ;;
```
This is required — buc_execute validates commands match the `bure_` prefix.

## Verification

1. `./tt/rbw-trg.TestRegimeSmoke.sh`

## Acceptance

- bure_regime.sh uses enrollment + scope_sentinel + enforce
- BURE_COUNTDOWN handled as optional string with custom enforce check
- bure_cli.sh uses buc_execute + furnish + buv_report/buv_render
- buw_workbench.sh dispatch uses prefixed command names
- Regime smoke tests pass

**[260222-1439] rough**

Apply T1-T13 transformation recipes to BURE (Environment Regime, BUK domain, ambient).

## Exemplar

Follow RBRN cli pattern (sources inside furnish via `BURD_BUK_DIR`/`BURD_TOOLS_DIR`), NOT RBRR (which still uses top-level SCRIPT_DIR sourcing). See `Tools/rbw/rbrn_cli.sh` for reference.

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, BURE section.

## Current State

- `Tools/buk/bure_regime.sh` (73 lines) — 3 variables, UNEXPECTED array, uses buv_opt_enum and buv_env_enum
- `Tools/buk/bure_cli.sh` (94 lines) — SCRIPT_DIR pattern, manual case dispatch, 1 rbcr_section group

## Variables (3 total, 1 group)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| BURE_COUNTDOWN | buv_string_enroll 0 4 opt | Custom enforce: if non-empty, must be "skip" |
| BURE_VERBOSE | buv_enum_enroll req | Values: 0 1 2 3 |
| BURE_COLOR | buv_string_enroll 1 4 req | Check if valid values are "auto", "on", "off" — if so, use buv_enum_enroll instead |

Group: "Behavioral Overrides".

## Design Decision: Optional Enum (BURE_COUNTDOWN)

The enrollment system has no `buv_opt_enum`. Currently validated by `buv_opt_enum BURE_COUNTDOWN skip`.

**Decision:** Enroll as `buv_string_enroll BURE_COUNTDOWN 0 4 "Countdown override (skip to disable)"`. The min=0 makes it optional. Add custom enforce:
```bash
if test -n "${BURE_COUNTDOWN}"; then
  test "${BURE_COUNTDOWN}" = "skip" \
    || buc_die "BURE_COUNTDOWN must be 'skip' or empty, got '${BURE_COUNTDOWN}'"
fi
```

## Transformations

- T1: CLI restructure — BUK-domain ambient. `source "${BURD_BUK_DIR}/buc_command.sh"`. Furnish sources buv, burd, bure_regime, bupr. No env file to source — BURE is ambient (caller environment). Furnish just calls kindle + enforce.
- T3: buv enrollment — 3 variables, 1 group. See design decision above for COUNTDOWN.
- T5: buv_report / buv_render. Render header should still say "ambient (caller environment)" — handle in furnish or command function.
- T6: buv_scope_sentinel replaces ZBURE_UNEXPECTED
- T9: buc_doc_* on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

## Dispatch Update

`buw_workbench.sh` (lines 77-78) dispatches BURE with bare command names:
```
buw-rev) exec "${z_bure_cli}" validate ;;
buw-rer) exec "${z_bure_cli}" render ;;
```
After buc_execute restructure, these must become prefixed:
```
buw-rev) exec "${z_bure_cli}" bure_validate ;;
buw-rer) exec "${z_bure_cli}" bure_render ;;
```
This is required — buc_execute validates commands match the `bure_` prefix.

## Verification

1. `./tt/rbw-trg.TestRegimeSmoke.sh`

## Acceptance

- bure_regime.sh uses enrollment + scope_sentinel + enforce
- BURE_COUNTDOWN handled as optional string with custom enforce check
- bure_cli.sh uses buc_execute + furnish + buv_report/buv_render
- buw_workbench.sh dispatch uses prefixed command names
- Regime smoke tests pass

**[260222-1410] rough**

Apply T1-T13 transformation recipes to BURE (Environment Regime, BUK domain, ambient).

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, BURE section.

## Current State

- `Tools/buk/bure_regime.sh` (73 lines) — 3 variables, UNEXPECTED array, uses buv_opt_enum and buv_env_enum
- `Tools/buk/bure_cli.sh` (94 lines) — SCRIPT_DIR pattern, manual case dispatch, 1 rbcr_section group

## Variables (3 total, 1 group)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| BURE_COUNTDOWN | buv_string_enroll 0 4 opt | Custom enforce: if non-empty, must be "skip" |
| BURE_VERBOSE | buv_enum_enroll req | Values: 0 1 2 3 |
| BURE_COLOR | buv_string_enroll 1 4 req | Check if valid values are "auto", "on", "off" — if so, use buv_enum_enroll instead |

Group: "Behavioral Overrides".

## Design Decision: Optional Enum (BURE_COUNTDOWN)

The enrollment system has no `buv_opt_enum`. Currently validated by `buv_opt_enum BURE_COUNTDOWN skip`.

**Decision:** Enroll as `buv_string_enroll BURE_COUNTDOWN 0 4 "Countdown override (skip to disable)"`. The min=0 makes it optional. Add custom enforce:
```bash
if test -n "${BURE_COUNTDOWN}"; then
  test "${BURE_COUNTDOWN}" = "skip" \
    || buc_die "BURE_COUNTDOWN must be 'skip' or empty, got '${BURE_COUNTDOWN}'"
fi
```

## Transformations

- T1: CLI restructure — BUK-domain ambient. `source "${BURD_BUK_DIR}/buc_command.sh"`. Furnish sources buv, burd, bure_regime, bupr. No env file to source — BURE is ambient (caller environment). Furnish just calls kindle + enforce.
- T3: buv enrollment — 3 variables, 1 group. See design decision above for COUNTDOWN.
- T5: buv_report / buv_render. Render header should still say "ambient (caller environment)" — handle in furnish or command function.
- T6: buv_scope_sentinel replaces ZBURE_UNEXPECTED
- T9: buc_doc_* on validate, render
- T12/T13: furnish receives command, bupr replaces rbcr

## Verification

1. `./tt/rbw-trg.RegimeSmoke.sh`

## Acceptance

- bure_regime.sh uses enrollment + scope_sentinel + enforce
- BURE_COUNTDOWN handled as optional string with custom enforce check
- bure_cli.sh uses buc_execute + furnish + buv_report/buv_render
- Regime smoke tests pass

### scrub-rbra-manifold (₢AfAAk) [complete]

**[260222-1506] complete**

Apply T1-T13 transformation recipes to RBRA (Authentication/Credential Regime, manifold: governor, retriever, director).

## Exemplar

Follow RBRN cli pattern (sources inside furnish via `BURD_BUK_DIR`/`BURD_TOOLS_DIR`), NOT RBRR (which still uses top-level SCRIPT_DIR sourcing). See `Tools/rbw/rbrn_cli.sh` for reference.

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, RBRA section.

## Current State

- `Tools/rbw/rbra_regime.sh` (92 lines) — 4 variables, UNEXPECTED array, ZRBRA_ROLLUP with [REDACTED] key, 2 grep -qE sites
- `Tools/rbw/rbra_cli.sh` (189 lines) — SCRIPT_DIR pattern, manual case dispatch (validate|render|list), role resolution via case arms

## Variables (4 total, 1 group)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| RBRA_CLIENT_EMAIL | buv_string_enroll 1 256 | Custom enforce: must end .iam.gserviceaccount.com |
| RBRA_PRIVATE_KEY | buv_string_enroll 1 4096 | Custom enforce: must contain PEM BEGIN marker |
| RBRA_PROJECT_ID | buv_string_enroll 1 64 | |
| RBRA_TOKEN_LIFETIME_SEC | buv_decimal_enroll 300 3600 | |

Group: "Service Account Credentials".

## Manifold Mechanics

RBRA is manifold but NOT glob-discovered (unlike RBRN/RBRV). Three fixed roles: governor, retriever, director. Each role's file path comes from RBRR: RBRR_GOVERNOR_RBRA_FILE, RBRR_RETRIEVER_RBRA_FILE, RBRR_DIRECTOR_RBRA_FILE.

**BUZ_FOLIO carries role name.** Furnish resolves folio->RBRR variable->file path:
```bash
zrbra_resolve_role() {
  local z_role="${1:-}"
  case "${z_role}" in
    governor)  echo "${RBRR_GOVERNOR_RBRA_FILE}" ;;
    retriever) echo "${RBRR_RETRIEVER_RBRA_FILE}" ;;
    director)  echo "${RBRR_DIRECTOR_RBRA_FILE}" ;;
    *) buc_die "Unknown RBRA role: ${z_role}" ;;
  esac
}
```

## Design Decisions

### RBRA_PRIVATE_KEY in render
buv_render will display the raw private key value. Current render shows length + [REDACTED].

**Decision:** Accept raw display. This is a local-only diagnostic tool. The key is already on disk. If redaction is desired later, add a `redacted` enrollment type — but not in this pace.

### ZRBRA_ROLLUP
Currently builds `ZRBRA_ROLLUP` with `RBRA_PRIVATE_KEY='[REDACTED]'`. Check if consumed anywhere outside regime/cli. If dead code, delete. If consumed, replace with buv_docker_env or keep manually for the redaction.

**Search:** `grep -r ZRBRA_ROLLUP` in Tools/ to determine consumers before deleting.

### Channel wiring
Zipper currently registers RBRA commands without channel. After restructure: `buz_enroll RBZ_RENDER_AUTH "rbw-rar" "${z_mod}" "rbra_render" "param1"` — BUZ_FOLIO carries role name.

## Transformations

- T1: CLI restructure — manifold. Furnish sources buv, burd, rbcc, rbrr_regime, rbra_regime, bupr. Loads RBRR, then if BUZ_FOLIO set, resolves role->file->source->kindle->enforce.
- T2: No _load exists. Keep role-resolution function (refactored).
- T3: buv enrollment — 4 variables, 1 group
- T5: buv_report / buv_render
- T6: buv_scope_sentinel replaces ZRBRA_UNEXPECTED
- T7: Delete ZRBRA_ROLLUP if dead code; replace with buv_docker_env if consumed
- T8b: Replace 2 grep -qE + 1 grep -q sites with [[ =~ ]] in enforce:
  ```bash
  zrbra_enforce() {
    zrbra_sentinel
    buv_vet RBRA
    [[ "${RBRA_CLIENT_EMAIL}" =~ \.iam\.gserviceaccount\.com$ ]] \
      || buc_die "RBRA_CLIENT_EMAIL must end with .iam.gserviceaccount.com"
    [[ "${RBRA_PRIVATE_KEY}" =~ BEGIN ]] \
      || buc_die "RBRA_PRIVATE_KEY must contain PEM BEGIN marker"
  }
  ```
- T9: buc_doc_* on validate, render, list
- T10: Update rbz_zipper.sh — add "param1" channel to RBRA colophon registrations
- T11: rbra_list stays as explicit function (3 fixed roles, not glob-discovered)
- T12/T13: furnish receives command, bupr replaces rbcr

## Dispatch Update

`rbz_zipper.sh` (lines 109-112) dispatches RBRA with bare command names:
```
buz_enroll RBZ_RENDER_AUTH   "rbw-rar" "${z_mod}" "render"
buz_enroll RBZ_VALIDATE_AUTH "rbw-rav" "${z_mod}" "validate"
buz_enroll RBZ_LIST_AUTH     "rbw-ral" "${z_mod}" "list"
```
After buc_execute restructure, these must become prefixed AND channel-wired:
```
buz_enroll RBZ_RENDER_AUTH   "rbw-rar" "${z_mod}" "rbra_render"   "param1"
buz_enroll RBZ_VALIDATE_AUTH "rbw-rav" "${z_mod}" "rbra_validate" "param1"
buz_enroll RBZ_LIST_AUTH     "rbw-ral" "${z_mod}" "rbra_list"
```
The command prefix AND channel wiring are both required in the same edit.

## Verification

1. `./tt/rbw-trg.TestRegimeSmoke.sh`
2. `./tt/rbw-trc.RegimeCredentials.sh` (credential test suite)
3. Verify `./tt/rbw-rar.RenderAuth.sh governor` still works after channel wiring

## Acceptance

- rbra_regime.sh uses enrollment + scope_sentinel + enforce with [[ =~ ]]
- rbra_cli.sh uses buc_execute + furnish + buv_report/buv_render
- BUZ_FOLIO carries role name; channel wired in rbz_zipper.sh
- rbz_zipper.sh dispatch uses prefixed command names with param1 channel
- ZRBRA_ROLLUP eliminated or replaced
- No grep in regime module
- Regime smoke and credential tests pass

**[260222-1440] rough**

Apply T1-T13 transformation recipes to RBRA (Authentication/Credential Regime, manifold: governor, retriever, director).

## Exemplar

Follow RBRN cli pattern (sources inside furnish via `BURD_BUK_DIR`/`BURD_TOOLS_DIR`), NOT RBRR (which still uses top-level SCRIPT_DIR sourcing). See `Tools/rbw/rbrn_cli.sh` for reference.

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, RBRA section.

## Current State

- `Tools/rbw/rbra_regime.sh` (92 lines) — 4 variables, UNEXPECTED array, ZRBRA_ROLLUP with [REDACTED] key, 2 grep -qE sites
- `Tools/rbw/rbra_cli.sh` (189 lines) — SCRIPT_DIR pattern, manual case dispatch (validate|render|list), role resolution via case arms

## Variables (4 total, 1 group)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| RBRA_CLIENT_EMAIL | buv_string_enroll 1 256 | Custom enforce: must end .iam.gserviceaccount.com |
| RBRA_PRIVATE_KEY | buv_string_enroll 1 4096 | Custom enforce: must contain PEM BEGIN marker |
| RBRA_PROJECT_ID | buv_string_enroll 1 64 | |
| RBRA_TOKEN_LIFETIME_SEC | buv_decimal_enroll 300 3600 | |

Group: "Service Account Credentials".

## Manifold Mechanics

RBRA is manifold but NOT glob-discovered (unlike RBRN/RBRV). Three fixed roles: governor, retriever, director. Each role's file path comes from RBRR: RBRR_GOVERNOR_RBRA_FILE, RBRR_RETRIEVER_RBRA_FILE, RBRR_DIRECTOR_RBRA_FILE.

**BUZ_FOLIO carries role name.** Furnish resolves folio->RBRR variable->file path:
```bash
zrbra_resolve_role() {
  local z_role="${1:-}"
  case "${z_role}" in
    governor)  echo "${RBRR_GOVERNOR_RBRA_FILE}" ;;
    retriever) echo "${RBRR_RETRIEVER_RBRA_FILE}" ;;
    director)  echo "${RBRR_DIRECTOR_RBRA_FILE}" ;;
    *) buc_die "Unknown RBRA role: ${z_role}" ;;
  esac
}
```

## Design Decisions

### RBRA_PRIVATE_KEY in render
buv_render will display the raw private key value. Current render shows length + [REDACTED].

**Decision:** Accept raw display. This is a local-only diagnostic tool. The key is already on disk. If redaction is desired later, add a `redacted` enrollment type — but not in this pace.

### ZRBRA_ROLLUP
Currently builds `ZRBRA_ROLLUP` with `RBRA_PRIVATE_KEY='[REDACTED]'`. Check if consumed anywhere outside regime/cli. If dead code, delete. If consumed, replace with buv_docker_env or keep manually for the redaction.

**Search:** `grep -r ZRBRA_ROLLUP` in Tools/ to determine consumers before deleting.

### Channel wiring
Zipper currently registers RBRA commands without channel. After restructure: `buz_enroll RBZ_RENDER_AUTH "rbw-rar" "${z_mod}" "rbra_render" "param1"` — BUZ_FOLIO carries role name.

## Transformations

- T1: CLI restructure — manifold. Furnish sources buv, burd, rbcc, rbrr_regime, rbra_regime, bupr. Loads RBRR, then if BUZ_FOLIO set, resolves role->file->source->kindle->enforce.
- T2: No _load exists. Keep role-resolution function (refactored).
- T3: buv enrollment — 4 variables, 1 group
- T5: buv_report / buv_render
- T6: buv_scope_sentinel replaces ZRBRA_UNEXPECTED
- T7: Delete ZRBRA_ROLLUP if dead code; replace with buv_docker_env if consumed
- T8b: Replace 2 grep -qE + 1 grep -q sites with [[ =~ ]] in enforce:
  ```bash
  zrbra_enforce() {
    zrbra_sentinel
    buv_vet RBRA
    [[ "${RBRA_CLIENT_EMAIL}" =~ \.iam\.gserviceaccount\.com$ ]] \
      || buc_die "RBRA_CLIENT_EMAIL must end with .iam.gserviceaccount.com"
    [[ "${RBRA_PRIVATE_KEY}" =~ BEGIN ]] \
      || buc_die "RBRA_PRIVATE_KEY must contain PEM BEGIN marker"
  }
  ```
- T9: buc_doc_* on validate, render, list
- T10: Update rbz_zipper.sh — add "param1" channel to RBRA colophon registrations
- T11: rbra_list stays as explicit function (3 fixed roles, not glob-discovered)
- T12/T13: furnish receives command, bupr replaces rbcr

## Dispatch Update

`rbz_zipper.sh` (lines 109-112) dispatches RBRA with bare command names:
```
buz_enroll RBZ_RENDER_AUTH   "rbw-rar" "${z_mod}" "render"
buz_enroll RBZ_VALIDATE_AUTH "rbw-rav" "${z_mod}" "validate"
buz_enroll RBZ_LIST_AUTH     "rbw-ral" "${z_mod}" "list"
```
After buc_execute restructure, these must become prefixed AND channel-wired:
```
buz_enroll RBZ_RENDER_AUTH   "rbw-rar" "${z_mod}" "rbra_render"   "param1"
buz_enroll RBZ_VALIDATE_AUTH "rbw-rav" "${z_mod}" "rbra_validate" "param1"
buz_enroll RBZ_LIST_AUTH     "rbw-ral" "${z_mod}" "rbra_list"
```
The command prefix AND channel wiring are both required in the same edit.

## Verification

1. `./tt/rbw-trg.TestRegimeSmoke.sh`
2. `./tt/rbw-trc.RegimeCredentials.sh` (credential test suite)
3. Verify `./tt/rbw-rar.RenderAuth.sh governor` still works after channel wiring

## Acceptance

- rbra_regime.sh uses enrollment + scope_sentinel + enforce with [[ =~ ]]
- rbra_cli.sh uses buc_execute + furnish + buv_report/buv_render
- BUZ_FOLIO carries role name; channel wired in rbz_zipper.sh
- rbz_zipper.sh dispatch uses prefixed command names with param1 channel
- ZRBRA_ROLLUP eliminated or replaced
- No grep in regime module
- Regime smoke and credential tests pass

**[260222-1410] rough**

Apply T1-T13 transformation recipes to RBRA (Authentication/Credential Regime, manifold: governor, retriever, director).

## Catalog Reference

Read `.claude/jjm/jjp_uAlf_catalog.md` Part 2, RBRA section.

## Current State

- `Tools/rbw/rbra_regime.sh` (92 lines) — 4 variables, UNEXPECTED array, ZRBRA_ROLLUP with [REDACTED] key, 2 grep -qE sites
- `Tools/rbw/rbra_cli.sh` (189 lines) — SCRIPT_DIR pattern, manual case dispatch (validate|render|list), role resolution via case arms

## Variables (4 total, 1 group)

| Variable | Enrollment | Notes |
|----------|-----------|-------|
| RBRA_CLIENT_EMAIL | buv_string_enroll 1 256 | Custom enforce: must end .iam.gserviceaccount.com |
| RBRA_PRIVATE_KEY | buv_string_enroll 1 4096 | Custom enforce: must contain PEM BEGIN marker |
| RBRA_PROJECT_ID | buv_string_enroll 1 64 | |
| RBRA_TOKEN_LIFETIME_SEC | buv_decimal_enroll 300 3600 | |

Group: "Service Account Credentials".

## Manifold Mechanics

RBRA is manifold but NOT glob-discovered (unlike RBRN/RBRV). Three fixed roles: governor, retriever, director. Each role's file path comes from RBRR: RBRR_GOVERNOR_RBRA_FILE, RBRR_RETRIEVER_RBRA_FILE, RBRR_DIRECTOR_RBRA_FILE.

**BUZ_FOLIO carries role name.** Furnish resolves folio→RBRR variable→file path:
```bash
zrbra_resolve_role() {
  local z_role="${1:-}"
  case "${z_role}" in
    governor)  echo "${RBRR_GOVERNOR_RBRA_FILE}" ;;
    retriever) echo "${RBRR_RETRIEVER_RBRA_FILE}" ;;
    director)  echo "${RBRR_DIRECTOR_RBRA_FILE}" ;;
    *) buc_die "Unknown RBRA role: ${z_role}" ;;
  esac
}
```

## Design Decisions

### RBRA_PRIVATE_KEY in render
buv_render will display the raw private key value. Current render shows length + [REDACTED].

**Decision:** Accept raw display. This is a local-only diagnostic tool. The key is already on disk. If redaction is desired later, add a `redacted` enrollment type — but not in this pace.

### ZRBRA_ROLLUP
Currently builds `ZRBRA_ROLLUP` with `RBRA_PRIVATE_KEY='[REDACTED]'`. Check if consumed anywhere outside regime/cli. If dead code, delete. If consumed, replace with buv_docker_env or keep manually for the redaction.

**Search:** `grep -r ZRBRA_ROLLUP` in Tools/ to determine consumers before deleting.

### Channel wiring
Zipper currently registers RBRA commands without channel. After restructure: `buz_enroll RBZ_RENDER_AUTH "rbw-rar" "${z_mod}" "rbra_render" "param1"` — BUZ_FOLIO carries role name.

## Transformations

- T1: CLI restructure — manifold. Furnish sources buv, burd, rbcc, rbrr_regime, rbra_regime, bupr. Loads RBRR, then if BUZ_FOLIO set, resolves role→file→source→kindle→enforce.
- T2: No _load exists. Keep role-resolution function (refactored).
- T3: buv enrollment — 4 variables, 1 group
- T5: buv_report / buv_render
- T6: buv_scope_sentinel replaces ZRBRA_UNEXPECTED
- T7: Delete ZRBRA_ROLLUP if dead code; replace with buv_docker_env if consumed
- T8b: Replace 2 grep -qE + 1 grep -q sites with [[ =~ ]] in enforce:
  ```bash
  zrbra_enforce() {
    zrbra_sentinel
    buv_vet RBRA
    [[ "${RBRA_CLIENT_EMAIL}" =~ \.iam\.gserviceaccount\.com$ ]] \
      || buc_die "RBRA_CLIENT_EMAIL must end with .iam.gserviceaccount.com"
    [[ "${RBRA_PRIVATE_KEY}" =~ BEGIN ]] \
      || buc_die "RBRA_PRIVATE_KEY must contain PEM BEGIN marker"
  }
  ```
- T9: buc_doc_* on validate, render, list
- T10: Update rbz_zipper.sh — add "param1" channel to RBRA colophon registrations
- T11: rbra_list stays as explicit function (3 fixed roles, not glob-discovered)
- T12/T13: furnish receives command, bupr replaces rbcr

## Verification

1. `./tt/rbw-trg.RegimeSmoke.sh`
2. `./tt/rbw-trc.RegimeCredentials.sh` (credential test suite)
3. Verify `./tt/rbw-rar.RenderAuth.sh governor` still works after channel wiring

## Acceptance

- rbra_regime.sh uses enrollment + scope_sentinel + enforce with [[ =~ ]]
- rbra_cli.sh uses buc_execute + furnish + buv_report/buv_render
- BUZ_FOLIO carries role name; channel wired in rbz_zipper.sh
- ZRBRA_ROLLUP eliminated or replaced
- No grep in regime module
- Regime smoke and credential tests pass

### document-regime-transformations (₢AfAAF) [complete]

**[260222-1413] complete**

Catalog all transformation patterns from the RBRR (singleton) and RBRN (manifold) exemplars, including buv_ enrollment patterns. This is the human review gate — user judges both exemplars before mechanical application proceeds.

## Before Anchor

Commit 57b1ff99 is the baseline before any regime second-pass work began.

## Scope

1. **Diff analysis**: Compare before-anchor to post-exemplar state for both RBRR and RBRN
2. **Catalog transformations**: Document each change pattern as a repeatable recipe:
   - CLI restructure: case dispatch to buc_execute + furnish
   - Eliminate _load/_load_moniker: inline source + kindle in furnish
   - buv_ enrollment: kindle registers variables with buv_*_enroll, furnish calls buv_enforce
   - CLI validate command: replace per-module validate body with buv_report call
   - Gated enrollment for conditionally-required variables
   - List enrollment for multi-value variables
   - BCG violations: [[ == ]] to test, grep to [[ =~ ]], unquoted arrays
   - buc_doc_* addition to all public functions
   - Channel declaration for manifold regimes in rbz
3. **Produce checklist**: Per-regime checklist suitable for bridled paces

## Acceptance

- Transformation catalog reviewed and approved by human
- Checklist covers singleton and manifold cases, including enrollment patterns
- Remaining regimes can be mechanically transformed using the catalog

**[260219-1856] rough**

Catalog all transformation patterns from the RBRR (singleton) and RBRN (manifold) exemplars, including buv_ enrollment patterns. This is the human review gate — user judges both exemplars before mechanical application proceeds.

## Before Anchor

Commit 57b1ff99 is the baseline before any regime second-pass work began.

## Scope

1. **Diff analysis**: Compare before-anchor to post-exemplar state for both RBRR and RBRN
2. **Catalog transformations**: Document each change pattern as a repeatable recipe:
   - CLI restructure: case dispatch to buc_execute + furnish
   - Eliminate _load/_load_moniker: inline source + kindle in furnish
   - buv_ enrollment: kindle registers variables with buv_*_enroll, furnish calls buv_enforce
   - CLI validate command: replace per-module validate body with buv_report call
   - Gated enrollment for conditionally-required variables
   - List enrollment for multi-value variables
   - BCG violations: [[ == ]] to test, grep to [[ =~ ]], unquoted arrays
   - buc_doc_* addition to all public functions
   - Channel declaration for manifold regimes in rbz
3. **Produce checklist**: Per-regime checklist suitable for bridled paces

## Acceptance

- Transformation catalog reviewed and approved by human
- Checklist covers singleton and manifold cases, including enrollment patterns
- Remaining regimes can be mechanically transformed using the catalog

**[260219-0902] rough**

Catalog all transformation patterns from the RBRR (singleton) and RBRN (manifold) exemplars. This is the human review gate — user judges both exemplars before mechanical application proceeds.

## Before Anchor

Commit `57b1ff99` is the baseline before any regime second-pass work began.

## Scope

1. **Diff analysis**: Compare before-anchor to post-exemplar state for both RBRR and RBRN
2. **Catalog transformations**: Document each change pattern as a repeatable recipe:
   - CLI restructure: case dispatch → buc_execute + furnish
   - Eliminate _load/_load_moniker → inline source + kindle in furnish
   - Naming cleanup: resolve duplicate validate functions
   - BCG violations: [[ == ]] → test, grep → [[ =~ ]], unquoted arrays, etc.
   - buc_doc_* addition to all public functions
   - Channel declaration for manifold regimes in rbz
3. **Produce checklist**: Per-regime checklist suitable for bridled paces

## Acceptance

- Transformation catalog reviewed and approved by human
- Checklist covers singleton and manifold cases
- Remaining regimes can be mechanically transformed using the catalog

### apply-to-remaining-regimes (₢AfAAG) [abandoned]

**[260222-1405] abandoned**

Mechanical application of documented transformation patterns to all remaining regimes, using buv_ enrollment system.

## Target Regimes

### RBW Domain (singletons)
- RBRP — Payor Regime (3 grep-to-regex sites)
- RBRO — OAuth Regime (spec exists but no validator — create minimal rbro_regime.sh with kindle+enrollment)
- RBRE — ECR Regime (needs audit)
- RBRS — Station Regime (appears near-compliant)

### RBW Domain (manifold)
- RBRA — Authentication Regime (2 grep-to-regex sites, unquoted array, channel declaration needed)

### BUK Domain (singletons)
- BURC — Configuration Regime (unquoted array)
- BURS — Station Regime (unquoted array)
- BURE — Environment Regime (unquoted array)

## Furnish Pattern Review Gate

By the time this pace is reached, the sources-in-furnish pattern (₢AfAAY/₢AfAAZ) will have been proven on RBRN. Before applying transformations to remaining regimes, human review should decide:

1. Should remaining regime CLIs also adopt sources-in-furnish? (move source commands into furnish, use BURD_BUK_DIR/BURD_TOOLS_DIR, eliminate ../buk/ paths)
2. Should remaining regime CLIs also adopt differential furnish? (conditional source/kindle based on command name)
3. Should the BURC_ → BURD_ renaming be applied across all post-exec code at this point?

These are separate from the enrollment/validation transformation but could be bundled for efficiency. Human decides scope.

## Approach

Apply transformation checklist from the document-regime-transformations pace to each regime:
- Add buv_*_enroll in kindle for all regime variables
- Replace per-module validate_fields with buv_enforce (in furnish) and buv_report (in CLI validate)
- Replace rbcr_render.sh sourcing with buv_render calls
- Use buv_group_enroll / buv_gate_enroll for conditionally-required variables
- Fix BCG violations (grep-to-regex, unquoted arrays, [[ == ]])
- Add buc_doc_* blocks to public functions
- Declare buz channels for manifold regimes

This pace is likely splittable into per-regime sub-paces once the checklist is reviewed.

## Acceptance

- All regime CLIs use buc_execute pattern
- All _load functions eliminated
- All public functions have buc_doc_* blocks
- All manifold regimes use RBR0_FOLIO via buz channel
- All validation via buv_ enrollment
- All rendering via buv_render (no rbcr_render.sh sourcing)
- rbcr_render.sh deleted (zero consumers after conversion)
- BCG reference to rbcr_render.sh updated or removed
- RBRO has a minimal rbro_regime.sh with enrollment
- No [[ == ]], no grep for validation, no unquoted arrays
- Qualification passes
- All previously-passing test suites still pass

**[260222-1035] rough**

Mechanical application of documented transformation patterns to all remaining regimes, using buv_ enrollment system.

## Target Regimes

### RBW Domain (singletons)
- RBRP — Payor Regime (3 grep-to-regex sites)
- RBRO — OAuth Regime (spec exists but no validator — create minimal rbro_regime.sh with kindle+enrollment)
- RBRE — ECR Regime (needs audit)
- RBRS — Station Regime (appears near-compliant)

### RBW Domain (manifold)
- RBRA — Authentication Regime (2 grep-to-regex sites, unquoted array, channel declaration needed)

### BUK Domain (singletons)
- BURC — Configuration Regime (unquoted array)
- BURS — Station Regime (unquoted array)
- BURE — Environment Regime (unquoted array)

## Furnish Pattern Review Gate

By the time this pace is reached, the sources-in-furnish pattern (₢AfAAY/₢AfAAZ) will have been proven on RBRN. Before applying transformations to remaining regimes, human review should decide:

1. Should remaining regime CLIs also adopt sources-in-furnish? (move source commands into furnish, use BURD_BUK_DIR/BURD_TOOLS_DIR, eliminate ../buk/ paths)
2. Should remaining regime CLIs also adopt differential furnish? (conditional source/kindle based on command name)
3. Should the BURC_ → BURD_ renaming be applied across all post-exec code at this point?

These are separate from the enrollment/validation transformation but could be bundled for efficiency. Human decides scope.

## Approach

Apply transformation checklist from the document-regime-transformations pace to each regime:
- Add buv_*_enroll in kindle for all regime variables
- Replace per-module validate_fields with buv_enforce (in furnish) and buv_report (in CLI validate)
- Replace rbcr_render.sh sourcing with buv_render calls
- Use buv_group_enroll / buv_gate_enroll for conditionally-required variables
- Fix BCG violations (grep-to-regex, unquoted arrays, [[ == ]])
- Add buc_doc_* blocks to public functions
- Declare buz channels for manifold regimes

This pace is likely splittable into per-regime sub-paces once the checklist is reviewed.

## Acceptance

- All regime CLIs use buc_execute pattern
- All _load functions eliminated
- All public functions have buc_doc_* blocks
- All manifold regimes use RBR0_FOLIO via buz channel
- All validation via buv_ enrollment
- All rendering via buv_render (no rbcr_render.sh sourcing)
- rbcr_render.sh deleted (zero consumers after conversion)
- BCG reference to rbcr_render.sh updated or removed
- RBRO has a minimal rbro_regime.sh with enrollment
- No [[ == ]], no grep for validation, no unquoted arrays
- Qualification passes
- All previously-passing test suites still pass

**[260222-0757] rough**

Mechanical application of documented transformation patterns to all remaining regimes, using buv_ enrollment system.

## Target Regimes

### RBW Domain (singletons)
- RBRP — Payor Regime (3 grep-to-regex sites)
- RBRO — OAuth Regime (spec exists but no validator — create minimal rbro_regime.sh with kindle+enrollment)
- RBRE — ECR Regime (needs audit)
- RBRS — Station Regime (appears near-compliant)

### RBW Domain (manifold)
- RBRA — Authentication Regime (2 grep-to-regex sites, unquoted array, channel declaration needed)

### BUK Domain (singletons)
- BURC — Configuration Regime (unquoted array)
- BURS — Station Regime (unquoted array)
- BURE — Environment Regime (unquoted array)

## Furnish Pattern Review Gate

By the time this pace is reached, the sources-in-furnish pattern (₢AfAAY/₢AfAAZ) will have been proven on RBRN. Before applying transformations to remaining regimes, human review should decide:

1. Should remaining regime CLIs also adopt sources-in-furnish? (move source commands into furnish, use BURD_BUK_DIR/BURD_TOOLS_DIR, eliminate ../buk/ paths)
2. Should remaining regime CLIs also adopt differential furnish? (conditional source/kindle based on command name)
3. Should the BURC_ → BURD_ renaming be applied across all post-exec code at this point?

These are separate from the enrollment/validation transformation but could be bundled for efficiency. Human decides scope.

## Approach

Apply transformation checklist from the document-regime-transformations pace to each regime:
- Add buv_*_enroll in kindle for all regime variables
- Replace per-module validate_fields with buv_enforce (in furnish) and buv_report (in CLI validate)
- Use gated enrollment for conditionally-required variables
- Fix BCG violations (grep-to-regex, unquoted arrays, [[ == ]])
- Add buc_doc_* blocks to public functions
- Declare buz channels for manifold regimes

This pace is likely splittable into per-regime sub-paces once the checklist is reviewed.

## Acceptance

- All regime CLIs use buc_execute pattern
- All _load functions eliminated
- All public functions have buc_doc_* blocks
- All manifold regimes use RBR0_FOLIO via buz channel
- All validation via buv_ enrollment
- RBRO has a minimal rbro_regime.sh with enrollment
- No [[ == ]], no grep for validation, no unquoted arrays
- Qualification passes
- All previously-passing test suites still pass

**[260219-1910] rough**

Mechanical application of documented transformation patterns to all remaining regimes, using buv_ enrollment system.

## Target Regimes

### RBW Domain (singletons)
- RBRP — Payor Regime (3 grep-to-regex sites)
- RBRO — OAuth Regime (spec exists but no validator — create minimal rbro_regime.sh with kindle+enrollment to close the gap)
- RBRE — ECR Regime (needs audit)
- RBRS — Station Regime (appears near-compliant)

### RBW Domain (manifold)
- RBRA — Authentication Regime (2 grep-to-regex sites, unquoted array, channel declaration needed)

### BUK Domain (singletons)
- BURC — Configuration Regime (unquoted array)
- BURS — Station Regime (unquoted array)
- BURE — Environment Regime (unquoted array)

## RBRO Note

RBRO has a spec (RBSRO-RegimeOauth.adoc) defining its variables but was never implemented. With enrollment, creating the "validator" is just a minimal rbro_regime.sh with kindle+enrollment — dramatically less code than the old validate pattern. This closes the spec-implementation gap while following the same pattern as every other regime.

## Approach

Apply transformation checklist from the document-regime-transformations pace to each regime:
- Add buv_*_enroll in kindle for all regime variables
- Replace per-module validate_fields with buv_enforce (in furnish) and buv_report (in CLI validate)
- Use gated enrollment for conditionally-required variables
- Fix BCG violations (grep-to-regex, unquoted arrays, [[ == ]])
- Add buc_doc_* blocks to public functions
- Declare buz channels for manifold regimes

This pace is likely splittable into per-regime sub-paces once the checklist is reviewed.

## Acceptance

- All regime CLIs use buc_execute pattern
- All _load functions eliminated
- All public functions have buc_doc_* blocks
- All manifold regimes use RBR0_FOLIO via buz channel
- All validation via buv_ enrollment
- RBRO has a minimal rbro_regime.sh with enrollment
- No [[ == ]], no grep for validation, no unquoted arrays
- Qualification passes
- All previously-passing test suites still pass

**[260219-1856] rough**

Mechanical application of documented transformation patterns to all remaining regimes, using buv_ enrollment system.

## Target Regimes

### RBW Domain (singletons)
- RBRP — Payor Regime (3 grep-to-regex sites)
- RBRO — OAuth Regime (no validator exists — enrollment replaces need for one)
- RBRE — ECR Regime (needs audit)
- RBRS — Station Regime (appears near-compliant)

### RBW Domain (manifold)
- RBRA — Authentication Regime (2 grep-to-regex sites, unquoted array, channel declaration needed)

### BUK Domain (singletons)
- BURC — Configuration Regime (unquoted array)
- BURS — Station Regime (unquoted array)
- BURE — Environment Regime (unquoted array)

## Approach

Apply transformation checklist from the document-regime-transformations pace to each regime:
- Add buv_*_enroll in kindle for all regime variables
- Replace per-module validate_fields with buv_enforce (in furnish) and buv_report (in CLI validate)
- Use gated enrollment for conditionally-required variables
- Fix BCG violations (grep-to-regex, unquoted arrays, [[ == ]])
- Add buc_doc_* blocks to public functions
- Declare buz channels for manifold regimes

This pace is likely splittable into per-regime sub-paces once the checklist is reviewed.

## Acceptance

- All regime CLIs use buc_execute pattern
- All _load functions eliminated
- All public functions have buc_doc_* blocks
- All manifold regimes use RBR0_FOLIO via buz channel
- All validation via buv_ enrollment
- No [[ == ]], no grep for validation, no unquoted arrays
- Qualification passes
- All previously-passing test suites still pass

**[260219-0902] rough**

Mechanical application of documented transformation patterns to all remaining regimes.

## Target Regimes

### RBW Domain (singletons)
- RBRP — Payor Regime (3 grep→regex sites)
- RBRO — OAuth Regime (no validator exists — may need creation)
- RBRE — ECR Regime (needs audit)
- RBRS — Station Regime (appears near-compliant)

### RBW Domain (manifold)
- RBRV — Vessel Regime (2 [[ == ]] sites, unquoted array, channel declaration needed)
- RBRA — Authentication Regime (2 grep→regex sites, unquoted array, channel declaration needed)

### BUK Domain (singletons)
- BURC — Configuration Regime (unquoted array)
- BURS — Station Regime (unquoted array)
- BURE — Environment Regime (unquoted array)

## Approach

Apply transformation checklist from ₢AfAAF to each regime. This pace is likely splittable into per-regime sub-paces once the checklist is reviewed — the human review gate will determine if these should be individual bridled paces or batched.

## Acceptance

- All regime CLIs use buc_execute pattern
- All _load functions eliminated
- All public functions have buc_doc_* blocks
- All manifold regimes use RBR0_FOLIO via buz channel
- No [[ == ]], no grep for validation, no unquoted arrays
- Qualification passes
- All previously-passing test suites still pass

### apply-enrollment-to-burd-runtime (₢AfAAc) [complete]

**[260222-1515] complete**

Enroll BURD dispatch runtime variables in buv enrollment and add enforce.

## Context

BURD is a runtime regime — values come from the dispatch environment, not a file.env.
The paddock incorrectly says "runtime-only, by design" with no validator. It needs
enrollment so buv_vet/buv_report can validate variables are properly populated at runtime.

## Scope

1. **burd_regime.sh** — Add buv_*_enroll calls in zburd_kindle for all BURD_ variables
   - buv_regime_enroll BURD
   - Enroll all BURD_ variables with appropriate types
   - buv_scope_sentinel BURD BURD_
   - Add zburd_enforce() calling buv_vet BURD

2. **Paddock** — Update BURD entry to remove "no validator" annotation, mark as done

## Acceptance

- zburd_kindle enrolls all BURD_ variables
- zburd_enforce validates via buv_vet
- Regime smoke tests pass
- Paddock updated

**[260222-1301] rough**

Enroll BURD dispatch runtime variables in buv enrollment and add enforce.

## Context

BURD is a runtime regime — values come from the dispatch environment, not a file.env.
The paddock incorrectly says "runtime-only, by design" with no validator. It needs
enrollment so buv_vet/buv_report can validate variables are properly populated at runtime.

## Scope

1. **burd_regime.sh** — Add buv_*_enroll calls in zburd_kindle for all BURD_ variables
   - buv_regime_enroll BURD
   - Enroll all BURD_ variables with appropriate types
   - buv_scope_sentinel BURD BURD_
   - Add zburd_enforce() calling buv_vet BURD

2. **Paddock** — Update BURD entry to remove "no validator" annotation, mark as done

## Acceptance

- zburd_kindle enrolls all BURD_ variables
- zburd_enforce validates via buv_vet
- Regime smoke tests pass
- Paddock updated

### apply-enrollment-to-buk-regimes (₢AfAAd) [abandoned]

**[260222-1405] abandoned**

Apply buv enrollment pattern to BURC, BURS, and BURE regimes.

## Context

These three BUK regimes all have the same cross-cutting concern: unquoted ${#ARRAY[@]}
and lack of enrollment-based validation. Follow the proven pattern from RBRN/RBRV/RBRR.

## Scope

1. **burc_regime.sh** — Add buv_*_enroll calls in zburc_kindle, buv_scope_sentinel,
   zburc_enforce() with buv_vet. Fix unquoted ${#ZBURC_UNEXPECTED[@]}.

2. **burs_regime.sh** — Same treatment as BURC.

3. **bure_regime.sh** — Same treatment as BURC. Also add to regime census
   (paddock notes BURE is missing from census documents).

4. **butcrg_RegimeSmoke.sh** — Verify existing BURC/BURS tests still pass;
   add BURE test case if missing.

## Acceptance

- All three regimes use buv enrollment pattern
- No unquoted ${#ARRAY[@]} in any BUK validator
- BURE appears in regime census
- Regime smoke tests pass

**[260222-1301] rough**

Apply buv enrollment pattern to BURC, BURS, and BURE regimes.

## Context

These three BUK regimes all have the same cross-cutting concern: unquoted ${#ARRAY[@]}
and lack of enrollment-based validation. Follow the proven pattern from RBRN/RBRV/RBRR.

## Scope

1. **burc_regime.sh** — Add buv_*_enroll calls in zburc_kindle, buv_scope_sentinel,
   zburc_enforce() with buv_vet. Fix unquoted ${#ZBURC_UNEXPECTED[@]}.

2. **burs_regime.sh** — Same treatment as BURC.

3. **bure_regime.sh** — Same treatment as BURC. Also add to regime census
   (paddock notes BURE is missing from census documents).

4. **butcrg_RegimeSmoke.sh** — Verify existing BURC/BURS tests still pass;
   add BURE test case if missing.

## Acceptance

- All three regimes use buv enrollment pattern
- No unquoted ${#ARRAY[@]} in any BUK validator
- BURE appears in regime census
- Regime smoke tests pass

### audit-legacy-buv-callers (₢AfAAO) [complete]

**[260222-1529] complete**

Audit for surviving callers of legacy buv_val_*, buv_env_*, buv_opt_* functions.

## Context

After regime scrubs convert all validation to the buv_ enrollment system, some straggler callers of the old buv_val_*/buv_env_*/buv_opt_* functions may remain. This pace audits and migrates them before deletion.

## Scope

1. grep for buv_val_, buv_env_, buv_opt_ across active codebase (excluding ABANDONED/, Study/, Memos/)
2. For each surviving caller: migrate to enrollment pattern or inline test/buc_die equivalent
3. Verify no runtime references remain

## Acceptance

- Zero active callers of buv_val_*, buv_env_*, buv_opt_* functions
- All validation uses enrollment or inline patterns

**[260219-1853] rough**

Audit for surviving callers of legacy buv_val_*, buv_env_*, buv_opt_* functions.

## Context

After regime scrubs convert all validation to the buv_ enrollment system, some straggler callers of the old buv_val_*/buv_env_*/buv_opt_* functions may remain. This pace audits and migrates them before deletion.

## Scope

1. grep for buv_val_, buv_env_, buv_opt_ across active codebase (excluding ABANDONED/, Study/, Memos/)
2. For each surviving caller: migrate to enrollment pattern or inline test/buc_die equivalent
3. Verify no runtime references remain

## Acceptance

- Zero active callers of buv_val_*, buv_env_*, buv_opt_* functions
- All validation uses enrollment or inline patterns

### delete-legacy-buv-functions (₢AfAAP) [complete]

**[260222-1532] complete**

Delete legacy buv_val_*, buv_env_*, buv_opt_* functions from buv_validation.sh.

## Context

After enrollment system is complete and all callers migrated (audit-legacy-buv-callers pace), the old stateless validator functions are dead code. Remove them.

## Scope

1. Delete all buv_val_* function definitions from buv_validation.sh
2. Delete all buv_env_* function definitions
3. Delete all buv_opt_* function definitions
4. Delete any internal helper functions only used by the above (zbuv_env_wrapper, etc.)
5. Keep buv_validation.sh as the enrollment system module
6. Update any documentation references

## Acceptance

- buv_validation.sh contains only enrollment infrastructure (kindle, enroll, enforce, report)
- No buv_val_*, buv_env_*, buv_opt_* functions exist
- Qualification passes

**[260219-1854] rough**

Delete legacy buv_val_*, buv_env_*, buv_opt_* functions from buv_validation.sh.

## Context

After enrollment system is complete and all callers migrated (audit-legacy-buv-callers pace), the old stateless validator functions are dead code. Remove them.

## Scope

1. Delete all buv_val_* function definitions from buv_validation.sh
2. Delete all buv_env_* function definitions
3. Delete all buv_opt_* function definitions
4. Delete any internal helper functions only used by the above (zbuv_env_wrapper, etc.)
5. Keep buv_validation.sh as the enrollment system module
6. Update any documentation references

## Acceptance

- buv_validation.sh contains only enrollment infrastructure (kindle, enroll, enforce, report)
- No buv_val_*, buv_env_*, buv_opt_* functions exist
- Qualification passes

### bcg-updates-from-proven-patterns (₢AfAAJ) [complete]

**[260222-1547] complete**

Consider what belongs in BCG based on proven patterns from the regime second-pass.

## Context

During the Af heat, we established design principles and infrastructure:

1. **Regime module** (_regime.sh) = reusable data/state layer: kindle (with buv_ enrollment), sentinel, state readers. Sourced by multiple CLIs. Enrollment declares field contracts; the module does not validate or enforce itself.
2. **CLI** (_cli.sh) = human-facing presentation layer: calls buv_enforce in furnish (ironclad gate), calls buv_report for validate command (explained display), lists valid options gracefully. Only used at the terminal.
3. **Graceful path**: When a CLI receives no folio or no subcommand, it may list valid options and exit without calling buv_enforce. Not all paths kindle-then-enforce.
4. **Enrollment infrastructure**: buv_*_enroll functions, buv_enforce, buv_report, gating — all in buv_validation.sh.

By the time this pace executes, we will have applied these principles across all regimes and will have real experience with how well they hold up.

## Scope

### From original AfAAJ scope
1. Review how the CLI/regime split rule played out across singleton and manifold regimes
2. Decide if these principles are general enough for BCG or specific to the regime pattern
3. If BCG-worthy: draft additions to the Module Architecture section
4. If not: document as an RBW convention in the paddock or a memo

### Absorbed from dropped AfAAH (bcg-add-enforce-pattern)
5. **BCG boilerplate table** — Add enforce row if warranted by experience
6. **BCG furnish template** — Update to show buv_enforce call after kindle if the pattern proved universal
7. **BCG enrollment documentation** — Document enrollment types, gating, enforce/report consumption paths
8. **Module maturity checklist** — Add enrollment/enforce checkpoints

The key principle: document what we proved works, not what we speculate will work. All BCG additions must reflect battle-tested patterns from actual regime conversions.

## Acceptance

- Explicit decision recorded per concern: BCG update, RBW convention, or memo
- If BCG: enrollment pattern documented (types, gating, enforce, report)
- If BCG: furnish template and maturity checklist updated
- All BCG additions reflect patterns proven across multiple regimes

**[260220-0854] rough**

Consider what belongs in BCG based on proven patterns from the regime second-pass.

## Context

During the Af heat, we established design principles and infrastructure:

1. **Regime module** (_regime.sh) = reusable data/state layer: kindle (with buv_ enrollment), sentinel, state readers. Sourced by multiple CLIs. Enrollment declares field contracts; the module does not validate or enforce itself.
2. **CLI** (_cli.sh) = human-facing presentation layer: calls buv_enforce in furnish (ironclad gate), calls buv_report for validate command (explained display), lists valid options gracefully. Only used at the terminal.
3. **Graceful path**: When a CLI receives no folio or no subcommand, it may list valid options and exit without calling buv_enforce. Not all paths kindle-then-enforce.
4. **Enrollment infrastructure**: buv_*_enroll functions, buv_enforce, buv_report, gating — all in buv_validation.sh.

By the time this pace executes, we will have applied these principles across all regimes and will have real experience with how well they hold up.

## Scope

### From original AfAAJ scope
1. Review how the CLI/regime split rule played out across singleton and manifold regimes
2. Decide if these principles are general enough for BCG or specific to the regime pattern
3. If BCG-worthy: draft additions to the Module Architecture section
4. If not: document as an RBW convention in the paddock or a memo

### Absorbed from dropped AfAAH (bcg-add-enforce-pattern)
5. **BCG boilerplate table** — Add enforce row if warranted by experience
6. **BCG furnish template** — Update to show buv_enforce call after kindle if the pattern proved universal
7. **BCG enrollment documentation** — Document enrollment types, gating, enforce/report consumption paths
8. **Module maturity checklist** — Add enrollment/enforce checkpoints

The key principle: document what we proved works, not what we speculate will work. All BCG additions must reflect battle-tested patterns from actual regime conversions.

## Acceptance

- Explicit decision recorded per concern: BCG update, RBW convention, or memo
- If BCG: enrollment pattern documented (types, gating, enforce, report)
- If BCG: furnish template and maturity checklist updated
- All BCG additions reflect patterns proven across multiple regimes

**[260220-0854] rough**

Consider what belongs in BCG based on proven patterns from the regime second-pass.

## Context

During the Af heat, we established design principles and infrastructure:

1. **Regime module** (_regime.sh) = reusable data/state layer: kindle (with buv_ enrollment), sentinel, state readers. Sourced by multiple CLIs. Enrollment declares field contracts; the module does not validate or enforce itself.
2. **CLI** (_cli.sh) = human-facing presentation layer: calls buv_enforce in furnish (ironclad gate), calls buv_report for validate command (explained display), lists valid options gracefully. Only used at the terminal.
3. **Graceful path**: When a CLI receives no folio or no subcommand, it may list valid options and exit without calling buv_enforce. Not all paths kindle-then-enforce.
4. **Enrollment infrastructure**: buv_*_enroll functions, buv_enforce, buv_report, gating — all in buv_validation.sh.

By the time this pace executes, we will have applied these principles across all regimes and will have real experience with how well they hold up.

## Scope

### From original AfAAJ scope
1. Review how the CLI/regime split rule played out across singleton and manifold regimes
2. Decide if these principles are general enough for BCG or specific to the regime pattern
3. If BCG-worthy: draft additions to the Module Architecture section
4. If not: document as an RBW convention in the paddock or a memo

### Absorbed from dropped AfAAH (bcg-add-enforce-pattern)
5. **BCG boilerplate table** — Add enforce row if warranted by experience
6. **BCG furnish template** — Update to show buv_enforce call after kindle if the pattern proved universal
7. **BCG enrollment documentation** — Document enrollment types, gating, enforce/report consumption paths
8. **Module maturity checklist** — Add enrollment/enforce checkpoints

The key principle: document what we proved works, not what we speculate will work. All BCG additions must reflect battle-tested patterns from actual regime conversions.

## Acceptance

- Explicit decision recorded per concern: BCG update, RBW convention, or memo
- If BCG: enrollment pattern documented (types, gating, enforce, report)
- If BCG: furnish template and maturity checklist updated
- All BCG additions reflect patterns proven across multiple regimes

**[260219-1910] rough**

Consider whether the CLI-vs-regime-module split rule and the graceful-list-options path belong in BCG.

## Context

During the Af heat, we established design principles that guided the exemplar work:

1. **Regime module** (_regime.sh) = reusable data/state layer: kindle (with buv_ enrollment), sentinel, state readers. Sourced by multiple CLIs. Enrollment declares field contracts; the module does not validate or enforce itself.
2. **CLI** (_cli.sh) = human-facing presentation layer: calls buv_enforce in furnish (ironclad gate), calls buv_report for validate command (explained display), lists valid options gracefully. Only used at the terminal.
3. **Graceful path**: When a CLI receives no folio or no subcommand, it may list valid options and exit without calling buv_enforce. Not all paths kindle-then-enforce.

By the time this pace executes, we will have applied these principles across all regimes and will have real experience with how well they hold up.

## Scope

1. Review how the split rule played out across singleton and manifold regimes
2. Decide if these principles are general enough for BCG or specific to the regime pattern
3. If BCG-worthy: draft additions to the Module Architecture section
4. If not: document as an RBW convention in the paddock or a memo

## Acceptance

- Explicit decision recorded: BCG update, RBW convention, or memo
- If BCG: changes drafted (may become a follow-on pace)

**[260219-0918] rough**

Consider whether the CLI-vs-regime-module split rule and the graceful-list-options path belong in BCG.

## Context

During ₣Af planning, we established design principles that guided the exemplar work:

1. **Regime module** (`_regime.sh`) = reusable data/state layer: kindle, enforce, sentinel, state readers. Sourced by multiple CLIs.
2. **CLI** (`_cli.sh`) = human-facing presentation layer: render, list valid options, validate-as-command. Only used at the terminal.
3. **Graceful path**: When a CLI receives no folio or no subcommand, it may list valid options and exit without calling enforce. Not all paths kindle→enforce.

By the time this pace executes, we'll have applied these principles across all regimes and will have real experience with how well they hold up.

## Scope

1. Review how the split rule played out across singleton and manifold regimes
2. Decide if these principles are general enough for BCG or specific to the regime pattern
3. If BCG-worthy: draft additions to the Module Architecture section
4. If not: document as an RBW convention in the paddock or a memo

## Acceptance

- Explicit decision recorded: BCG update, RBW convention, or memo
- If BCG: changes drafted (may become a follow-on pace)

### final-paddock-hygiene-review (₢AfAAL) [complete]

**[260222-1552] complete**

Final review of paddock and docket content for stale references.

## Context

The paddock was written before the buv_ enrollment transformation and with specific line numbers from an initial audit. As paces execute and code changes, references become stale and resolved concerns accumulate.

## Scope

1. **Enrollment era update** — Add cross-cutting documentation about the enrollment transformation:
   - buv_ enrollment replaces per-module validate_fields/validate patterns
   - buv_enforce (ironclad gate) and buv_report (explained display) replace hand-written checking
   - Gating replaces optional variable handling
   - Cross-cutting concerns like "Unquoted ${#ARRAY[@]}" are subsumed by enrollment (buv_ handles all type checking internally)

2. **Stale references** — Review paddock per-regime concern lists:
   - Remove resolved items (mark with strikethrough or delete)
   - Replace any remaining specific line numbers with "search for pattern X" guidance
   - Update "Consumers of rbrn_load_moniker" — should be eliminated by now

3. **Regime inventory** — Verify all regimes are accounted for in completed paces:
   - RBRO should now have rbro_regime.sh
   - BURE should have enrollment for VERBOSE, COLOR, COUNTDOWN
   - All regimes should use enrollment

4. **Docket staleness** — Spot-check remaining docket content against actual state

## Acceptance

- No stale line numbers in paddock
- All resolved concerns marked as such
- Enrollment transformation documented in cross-cutting section
- Paddock reflects actual post-work state

**[260219-1911] rough**

Final review of paddock and docket content for stale references.

## Context

The paddock was written before the buv_ enrollment transformation and with specific line numbers from an initial audit. As paces execute and code changes, references become stale and resolved concerns accumulate.

## Scope

1. **Enrollment era update** — Add cross-cutting documentation about the enrollment transformation:
   - buv_ enrollment replaces per-module validate_fields/validate patterns
   - buv_enforce (ironclad gate) and buv_report (explained display) replace hand-written checking
   - Gating replaces optional variable handling
   - Cross-cutting concerns like "Unquoted ${#ARRAY[@]}" are subsumed by enrollment (buv_ handles all type checking internally)

2. **Stale references** — Review paddock per-regime concern lists:
   - Remove resolved items (mark with strikethrough or delete)
   - Replace any remaining specific line numbers with "search for pattern X" guidance
   - Update "Consumers of rbrn_load_moniker" — should be eliminated by now

3. **Regime inventory** — Verify all regimes are accounted for in completed paces:
   - RBRO should now have rbro_regime.sh
   - BURE should have enrollment for VERBOSE, COLOR, COUNTDOWN
   - All regimes should use enrollment

4. **Docket staleness** — Spot-check remaining docket content against actual state

## Acceptance

- No stale line numbers in paddock
- All resolved concerns marked as such
- Enrollment transformation documented in cross-cutting section
- Paddock reflects actual post-work state

**[260219-0919] rough**

Final review of paddock and docket content for stale references.

## Context

The paddock was written with specific line numbers from an initial audit. As paces execute and code changes, these line references become stale. Additionally, some concerns listed in the paddock will have been resolved by earlier paces.

## Scope

1. Review paddock per-regime concern lists — remove resolved items, update stale references
2. Replace any remaining specific line numbers with "search for pattern X" guidance
3. Verify all regimes in the paddock inventory are accounted for in completed paces
4. Clean up any other staleness from the planning phase

## Acceptance

- No stale line numbers in paddock
- All resolved concerns marked as such
- Paddock reflects actual post-work state

### burd-export-consolidation (₢AfAAW) [complete]

**[260222-0751] complete**

Add BURD_TOOLS_DIR and BURD_BUK_DIR to dispatch; remove the 3 BURC exports.

## Context

CLIs are exec'd (buz_zipper.sh:168), so exported env vars cross the exec boundary but in-process state does not. Currently BURC exports 3 variables (BURC_TABTARGET_DIR, BURC_TOOLS_DIR, BURC_BUK_DIR) solely to survive this exec. Architecturally, BURD is the exec-crossing contract — BURC should stay in-process.

## Scope

1. In bud_dispatch.sh zbud_setup(): after sourcing burc.env and validating BURC_TOOLS_DIR, set and export:
   - BURD_TOOLS_DIR="${BURC_TOOLS_DIR}"
   - BURD_BUK_DIR="${BURC_TOOLS_DIR}/buk"

2. In burd_regime.sh: add BURD_TOOLS_DIR and BURD_BUK_DIR to z_known list in zburd_kindle() and zburd_validate_fields()

3. In burc_regime.sh: remove the 3 export statements (lines 58-60)

4. In bul_launcher.sh: the launcher runs BEFORE exec, so it still uses BURC_TOOLS_DIR directly (lines 44-45, 51, 70) — no change needed there

5. Update BCG to document BURD_TOOLS_DIR and BURD_BUK_DIR as dispatch-provided variables

6. Update any code that references BURC_BUK_DIR or BURC_TOOLS_DIR after the exec boundary to use BURD_ prefix instead

## Acceptance

- BURD_TOOLS_DIR and BURD_BUK_DIR available in CLI processes after exec
- BURC exports nothing
- All tabtargets still work (qualification passes)

**[260222-0738] rough**

Add BURD_TOOLS_DIR and BURD_BUK_DIR to dispatch; remove the 3 BURC exports.

## Context

CLIs are exec'd (buz_zipper.sh:168), so exported env vars cross the exec boundary but in-process state does not. Currently BURC exports 3 variables (BURC_TABTARGET_DIR, BURC_TOOLS_DIR, BURC_BUK_DIR) solely to survive this exec. Architecturally, BURD is the exec-crossing contract — BURC should stay in-process.

## Scope

1. In bud_dispatch.sh zbud_setup(): after sourcing burc.env and validating BURC_TOOLS_DIR, set and export:
   - BURD_TOOLS_DIR="${BURC_TOOLS_DIR}"
   - BURD_BUK_DIR="${BURC_TOOLS_DIR}/buk"

2. In burd_regime.sh: add BURD_TOOLS_DIR and BURD_BUK_DIR to z_known list in zburd_kindle() and zburd_validate_fields()

3. In burc_regime.sh: remove the 3 export statements (lines 58-60)

4. In bul_launcher.sh: the launcher runs BEFORE exec, so it still uses BURC_TOOLS_DIR directly (lines 44-45, 51, 70) — no change needed there

5. Update BCG to document BURD_TOOLS_DIR and BURD_BUK_DIR as dispatch-provided variables

6. Update any code that references BURC_BUK_DIR or BURC_TOOLS_DIR after the exec boundary to use BURD_ prefix instead

## Acceptance

- BURD_TOOLS_DIR and BURD_BUK_DIR available in CLI processes after exec
- BURC exports nothing
- All tabtargets still work (qualification passes)

### buc-execute-differential-furnish (₢AfAAX) [complete]

**[260222-0756] complete**

One-line change to buc_execute: pass command name as $1 to furnish function.

## Context

Currently buc_execute calls furnish with no arguments (buc_command.sh:342). Furnish cannot know which command is about to run, forcing CLIs with differential kindling needs (light vs heavy deps) to split into separate files (e.g., rbcnc_cli.sh vs rbcnx_cli.sh).

Passing the command name to furnish enables conditional source/kindle based on which command will execute.

## Scope

1. In buc_command.sh buc_execute(), change line 342 from:
   [ -n "${env_func}" ] && "${env_func}"
   to:
   [ -n "${env_func}" ] && "${env_func}" "${command}"

2. Update BCG to document that furnish receives the command name as $1

3. Existing furnish functions that don't use $1 are unaffected (extra arg is ignored in bash)

## Acceptance

- buc_execute passes command name to furnish as $1
- BCG documents the furnish($1) convention
- All existing CLIs continue to work (backward compatible — unused $1 is harmless)

**[260222-0738] rough**

One-line change to buc_execute: pass command name as $1 to furnish function.

## Context

Currently buc_execute calls furnish with no arguments (buc_command.sh:342). Furnish cannot know which command is about to run, forcing CLIs with differential kindling needs (light vs heavy deps) to split into separate files (e.g., rbcnc_cli.sh vs rbcnx_cli.sh).

Passing the command name to furnish enables conditional source/kindle based on which command will execute.

## Scope

1. In buc_command.sh buc_execute(), change line 342 from:
   [ -n "${env_func}" ] && "${env_func}"
   to:
   [ -n "${env_func}" ] && "${env_func}" "${command}"

2. Update BCG to document that furnish receives the command name as $1

3. Existing furnish functions that don't use $1 are unaffected (extra arg is ignored in bash)

## Acceptance

- buc_execute passes command name to furnish as $1
- BCG documents the furnish($1) convention
- All existing CLIs continue to work (backward compatible — unused $1 is harmless)

### fix-zbuv-check-error-naming (₢AfAAl) [abandoned]

**[260222-1632] abandoned**

Rename ZBUV_CHECK_ERROR to z_zbuv_check_predicate_error in buv_validation.sh.

## Context

ZBUV_CHECK_ERROR uses kindle constant naming (Z«PREFIX»_SCREAMING) but is assigned in ~40 places outside kindle — it's a mutable return variable from zbuv_check_predicate. Per BCG, kindle constants must be defined exclusively in kindle.

## Scope

1. Rename ZBUV_CHECK_ERROR → z_zbuv_check_predicate_error throughout buv_validation.sh
2. Check for any external consumers of this variable

## Acceptance

- No ZBUV_CHECK_ERROR references remain
- All consumers use the new name
- Regime smoke tests pass

**[260222-1629] rough**

Rename ZBUV_CHECK_ERROR to z_zbuv_check_predicate_error in buv_validation.sh.

## Context

ZBUV_CHECK_ERROR uses kindle constant naming (Z«PREFIX»_SCREAMING) but is assigned in ~40 places outside kindle — it's a mutable return variable from zbuv_check_predicate. Per BCG, kindle constants must be defined exclusively in kindle.

## Scope

1. Rename ZBUV_CHECK_ERROR → z_zbuv_check_predicate_error throughout buv_validation.sh
2. Check for any external consumers of this variable

## Acceptance

- No ZBUV_CHECK_ERROR references remain
- All consumers use the new name
- Regime smoke tests pass

### remove-rbre-ghost-regime (₢AfAAo) [complete]

**[260223-1750] complete**

Remove RBRE (ECR Regime) ghost — spec exists with zero implementation or consumers.

## Files to remove/edit

- DELETE `lenses/RBSRE-RegimeEcr.adoc` — the regime spec
- EDIT `lenses/RBS0-SpecTop.adoc` — remove all RBRE linked terms (attributes, anchors, definitions) from mapping section and ECR Regime section
- EDIT paddock `jjp_uAlf.md` — update RBRE entry to note removal (like RBRG)

## Context

Same situation as RBRG (₢AfAAA): spec'd, never implemented, no shell code, no consumers.
Six RBRE_ variables defined in spec, zero references in Tools/.

## Acceptance

- RBSRE-RegimeEcr.adoc deleted
- No rbre_ attributes or anchors remain in RBS0
- Paddock updated
- cma-validate passes on RBS0

**[260223-0808] rough**

Remove RBRE (ECR Regime) ghost — spec exists with zero implementation or consumers.

## Files to remove/edit

- DELETE `lenses/RBSRE-RegimeEcr.adoc` — the regime spec
- EDIT `lenses/RBS0-SpecTop.adoc` — remove all RBRE linked terms (attributes, anchors, definitions) from mapping section and ECR Regime section
- EDIT paddock `jjp_uAlf.md` — update RBRE entry to note removal (like RBRG)

## Context

Same situation as RBRG (₢AfAAA): spec'd, never implemented, no shell code, no consumers.
Six RBRE_ variables defined in spec, zero references in Tools/.

## Acceptance

- RBSRE-RegimeEcr.adoc deleted
- No rbre_ attributes or anchors remain in RBS0
- Paddock updated
- cma-validate passes on RBS0

### fix-rbv-cli-missing-rbrs-enforce (₢AfAAp) [complete]

**[260223-1801] complete**

Add missing zrbrs_enforce call in rbv_cli.sh after zrbrs_kindle on line 51.

## Context

rbv_cli.sh sources RBRS station file and calls zrbrs_kindle but never calls zrbrs_enforce.
RBRS variables are then consumed without validation. rbrs_cli.sh does this correctly — rbv_cli.sh missed it.

## Fix

Add `zrbrs_enforce` after line 51 in Tools/rbw/rbv_cli.sh, following the pattern in rbrs_cli.sh.

## Acceptance

- zrbrs_enforce called after zrbrs_kindle in rbv_cli.sh
- Regime smoke tests pass

**[260223-0813] rough**

Add missing zrbrs_enforce call in rbv_cli.sh after zrbrs_kindle on line 51.

## Context

rbv_cli.sh sources RBRS station file and calls zrbrs_kindle but never calls zrbrs_enforce.
RBRS variables are then consumed without validation. rbrs_cli.sh does this correctly — rbv_cli.sh missed it.

## Fix

Add `zrbrs_enforce` after line 51 in Tools/rbw/rbv_cli.sh, following the pattern in rbrs_cli.sh.

## Acceptance

- zrbrs_enforce called after zrbrs_kindle in rbv_cli.sh
- Regime smoke tests pass

### fix-bul-launcher-missing-enforce (₢AfAAq) [complete]

**[260223-1823] complete**

Add missing zburc_enforce and zburs_enforce calls in bul_launcher.sh.

## Context

bul_launcher.sh kindles BURC (line 50) and BURS (line 56) but skips enforce for both.
BURC_* and BURS_* variables are consumed without validation (lines 39-45, 68-74).
Currently documented as "bootstrap exception" but variables are used unchecked.

## Fix

Add `zburc_enforce` after zburc_kindle and `zburs_enforce` after zburs_kindle in Tools/buk/bul_launcher.sh.

## Acceptance

- zburc_enforce called after zburc_kindle
- zburs_enforce called after zburs_kindle
- Tabtarget dispatch still works (kick-tires test passes)

**[260223-0813] rough**

Add missing zburc_enforce and zburs_enforce calls in bul_launcher.sh.

## Context

bul_launcher.sh kindles BURC (line 50) and BURS (line 56) but skips enforce for both.
BURC_* and BURS_* variables are consumed without validation (lines 39-45, 68-74).
Currently documented as "bootstrap exception" but variables are used unchecked.

## Fix

Add `zburc_enforce` after zburc_kindle and `zburs_enforce` after zburs_kindle in Tools/buk/bul_launcher.sh.

## Acceptance

- zburc_enforce called after zburc_kindle
- zburs_enforce called after zburs_kindle
- Tabtarget dispatch still works (kick-tires test passes)

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 n decide-zbuv-check-error-pattern
  2 m fix-rbrp-load-callers
  3 a buv-enrollment-render-unification
  4 T rbts-test-suites
  5 S access-probe-specs-and-tests
  6 R debug-ark-lifecycle-oauth-failure
  7 A remove-rbrg-regime
  8 B relocate-ambient-burd-vars-to-bure
  9 M buq-exfiltrate-qualify
  10 N buv-enrollment-infrastructure
  11 H bcg-add-enforce-pattern
  12 C scrub-rbrr-singleton-exemplar
  13 D buz-channel-infrastructure
  14 U debug-ark-lifecycle-silent-failure
  15 E scrub-rbrn-manifold-exemplar
  16 V bcg-weaken-module-cli-unity
  17 b buz-zipper-bcg-compliance
  18 Y rbrn-sources-in-furnish
  19 Z rbrn-cli-reunification
  20 K add-nameplate-list-tabtarget
  21 I scrub-rbrv-manifold-structural
  22 Q buv-enrollment-tests
  23 e scrub-rbrs-singleton
  24 f scrub-burs-singleton
  25 g scrub-rbre-singleton
  26 h scrub-rbrp-singleton
  27 i scrub-burc-singleton
  28 j scrub-bure-ambient
  29 k scrub-rbra-manifold
  30 F document-regime-transformations
  31 c apply-enrollment-to-burd-runtime
  32 O audit-legacy-buv-callers
  33 P delete-legacy-buv-functions
  34 J bcg-updates-from-proven-patterns
  35 L final-paddock-hygiene-review
  36 W burd-export-consolidation
  37 X buc-execute-differential-furnish
  38 o remove-rbre-ghost-regime
  39 p fix-rbv-cli-missing-rbrs-enforce
  40 q fix-bul-launcher-missing-enforce

nmaTSRABMNHCDUEVbYZKIQefghijkFcOPJLWXopq
··x········x··x·x·xxx·x··x··x··x········ rbz_zipper.sh
···x····x··x··x·x····x··········x······· rbtb_testbench.sh
x·x·····xx·x··x·················x······· buv_validation.sh
···········x······xx·············x·xx··· BCG-BashConsoleGuide.md
···········x··x···xxx··················· rbrn_cli.sh
··x···········x·xxx····················· rbcnc_cli.sh
··x········x··x····xx··················· rbrn_regime.sh
·······················x··x···x········x bul_launcher.sh
········x··············x··xx············ buw_workbench.sh
········x·····x·xx······················ rbw_workbench.sh
················xxx····················· rbcnx_cli.sh
···········x··········x········x········ rbcc_Constants.sh
···········x··x·x······················· rbob_cli.sh
···x·······x········x··················· butcrg_RegimeSmoke.sh
··x·········x···x······················· buz_zipper.sh
··x········x··x························· rbrr_cli.sh
·x·x·······x···························· rbtcap_AccessProbe.sh
x·x········x···························· rbrr_regime.sh
···················xx··················· rbw-rnl.ListNameplateRegime.sh
···········x··························x· rbv_cli.sh
···········x················x··········· rbra_cli.sh
···········x·············x·············· rbrp_cli.sh, rbrp_regime.sh
···········x········x··················· rbrv_cli.sh
···········xx··························· main.rs, study_workbench.sh
········x···························x··· buc_command.sh
········x··························x···· BUSD-DispatchRuntime.adoc, bud_dispatch.sh, burd_regime.sh
········x·····················x········· jjw_workbench.sh, vow_workbench.sh
········x··················x············ bure_cli.sh, bure_regime.sh
········x··x···························· rbgm_ManualProcedures.sh
····x································x·· RBS0-SpecTop.adoc
···x····························x······· butcvu_XnameValidation.sh
···x·······x···························· butcrg_RegimeCredentials.sh
·x·········x···························· rbgm_cli.sh, rbgp_cli.sh
·····································x·· RBSRE-RegimeEcr.adoc
·································x······ cloudbuild.yaml
·······························x········ rbgp_Payor.sh, rbro_cli.sh, rbro_regime.sh
······························x········· cccw_workbench.sh
····························x··········· rbra_regime.sh
··························x············· burc_cli.sh, burc_regime.sh
·······················x················ burs_cli.sh, burs_regime.sh
······················x················· rbrs_cli.sh, rbrs_regime.sh
·····················x·················· butcev_ChoiceTypes.sh, butcev_EnforceReport.sh, butcev_GateEnroll.sh, butcev_LengthTypes.sh, butcev_ListTypes.sh, butcev_NumericTypes.sh, butcev_RefTypes.sh
····················x··················· rbrv_regime.sh
··············x························· rbw-z.Stop.nsproto.sh, rbw-z.Stop.pluml.sh, rbw-z.Stop.srjcl.sh
···········x···························· Cargo.lock, Cargo.toml, launcher.study_workbench.sh, rbf_cli.sh, rbgg_cli.sh, study-mpt.Run.FULL.sh, study-mpt.Run.smoke.sh
········x······························· BUSA-BashUtilitiesSpec.adoc, README.md, buq_qualify.sh, buut_tabtarget.sh, rbq_Qualify.sh, vslw_workbench.sh
······x································· RBSA-SpecTop.adoc, RBSRG-RegimeGithub.adoc, rbv_PodmanVM.sh
·····x·································· rbgo_OAuth.sh
····x··································· RBSAJ-access_jwt_probe.adoc, RBSAO-access_oauth_probe.adoc, rbap_AccessProbe.sh
···x···································· butcde_DispatchExercise.sh, rbtcal_ArkLifecycle.sh, rbtckk_KickTires.sh, rbtcns_NsproSecurity.sh, rbtcpl_PlumlDiagram.sh, rbtcqa_QualifyAll.sh, rbtcsj_SrjclJupyter.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 241 commits)

  1 P delete-legacy-buv-functions
  2 J bcg-updates-from-proven-patterns
  3 L final-paddock-hygiene-review
  4 n decide-zbuv-check-error-pattern
  5 m fix-rbrp-load-callers
  6 o remove-rbre-ghost-regime
  7 p fix-rbv-cli-missing-rbrs-enforce
  8 q fix-bul-launcher-missing-enforce

123456789abcdefghijklmnopqrstuvwxyz
xxx································  P  3c
···xxxx····························  J  4c
·······xxx·························  L  3c
·················xxx···············  n  3c
····················xxx············  m  3c
··························xxx······  o  3c
·····························xxx···  p  3c
································xxx  q  3c
```

## Steeplechase

### 2026-02-23 18:23 - ₢AfAAq - W

Added zburc_enforce and zburs_enforce calls; consistent source guards with bootstrap-zone comments

### 2026-02-23 18:23 - ₢AfAAq - n

Add missing enforce calls and consistent source guards in bul_launcher.sh

### 2026-02-23 18:14 - ₢AfAAq - A

Add zburc_enforce after line 50 and zburs_enforce after line 56 in bul_launcher.sh

### 2026-02-23 18:01 - ₢AfAAp - W

Added missing zrbrs_enforce call after zrbrs_kindle in rbv_cli.sh, matching rbrs_cli.sh pattern

### 2026-02-23 18:01 - ₢AfAAp - n

Add missing zrbrs_enforce after zrbrs_kindle in rbv_cli.sh

### 2026-02-23 17:57 - ₢AfAAp - A

Add zrbrs_enforce after zrbrs_kindle line 51 in rbv_cli.sh — one-line fix matching rbrs_cli.sh pattern

### 2026-02-23 17:50 - ₢AfAAo - W

Removed RBRE ghost regime — deleted RBSRE-RegimeEcr.adoc, stripped all rbre_ linked terms from RBS0, updated paddock

### 2026-02-23 17:50 - ₢AfAAo - n

Remove RBRE ghost regime: delete spec, strip linked terms from RBS0, update paddock

### 2026-02-23 17:32 - ₢AfAAo - A

Delete RBSRE spec, strip rbre_ terms from RBS0, update paddock — mirrors RBRG removal pattern

### 2026-02-23 08:13 - Heat - S

fix-bul-launcher-missing-enforce

### 2026-02-23 08:13 - Heat - S

fix-rbv-cli-missing-rbrs-enforce

### 2026-02-23 08:08 - Heat - S

remove-rbre-ghost-regime

### 2026-02-23 07:53 - ₢AfAAm - W

Replaced rbrp_load with source+kindle+enforce in rbgp_cli.sh, rbgm_cli.sh, rbtcap_AccessProbe.sh; regime-smoke passes

### 2026-02-23 07:53 - ₢AfAAm - n

Replace rbrp_load with source+kindle+enforce in 3 callers following rbrp_cli.sh pattern

### 2026-02-23 07:45 - ₢AfAAm - A

Replace rbrp_load with source+kindle+enforce in 3 callers following rbrp_cli.sh pattern

### 2026-02-23 07:35 - ₢AfAAn - W

Converted zbuv_check_predicate to zbuv_check_capture: eliminated ZBUV_CHECK_ERROR side-channel, added ZBUV_CHECK_GATED/ZBUV_CHECK_FAIL kindle constants, updated 2 consumers

### 2026-02-23 07:35 - ₢AfAAn - n

Refactor buv check_predicate to capture pattern, eliminating ZBUV_CHECK_ERROR global in favor of stdout-based result protocol with ZBUV_CHECK_GATED/ZBUV_CHECK_FAIL constants; update buv_vet and buv_report callers; align rbrr GCB enrollment whitespace

### 2026-02-23 07:13 - ₢AfAAn - A

Recommend option 4 (accept as-is) with lightweight BCG annotation for predicate-with-detail pattern

### 2026-02-22 16:37 - Heat - T

fix-rbrp-load-callers

### 2026-02-22 16:36 - Heat - r

moved AfAAm after AfAAn

### 2026-02-22 16:32 - Heat - S

decide-zbuv-check-error-pattern

### 2026-02-22 16:32 - Heat - T

fix-zbuv-check-error-naming

### 2026-02-22 16:30 - Heat - S

fix-rbrp-load-callers

### 2026-02-22 16:29 - Heat - S

fix-zbuv-check-error-naming

### 2026-02-22 16:21 - Heat - n

Add missing zbuv_kindle to buut_cli.sh and vob_cli.sh furnish functions — both source buv_validation.sh and call buv_file_exists but never kindled buv

### 2026-02-22 15:52 - ₢AfAAL - W

Updated paddock to reflect all resolved concerns, removed stale references, documented enrollment transformation outcomes across all regimes

### 2026-02-22 15:52 - ₢AfAAL - n

Read paddock, cross-reference concerns against completed paces, update resolved items, add enrollment transformation notes

### 2026-02-22 15:48 - ₢AfAAL - A

Read paddock, cross-reference concerns against completed paces, update resolved items, add enrollment transformation notes

### 2026-02-22 15:47 - ₢AfAAJ - W

Added regime archetype section, enforce boilerplate, removed obsolete buv_env/buv_val references, fixed stale BVU terminology

### 2026-02-22 15:47 - ₢AfAAJ - n

BCG regime archetype documentation and rbev-busybox cloudbuild.yaml: added enrollment infrastructure section, enforce function pattern, furnish template kindle-enforce sequence, regime archetype checklist; removed legacy buv_val/buv_env examples; created Cloud Build definition with OCI layout bridge, multi-arch buildx, SBOM generation, and metadata container

### 2026-02-22 15:37 - ₢AfAAJ - A

Four BCG edits: boilerplate enforce row, enrollment infrastructure section, furnish template kindle-enforce, maturity checklist enrollment checkpoints

### 2026-02-22 15:32 - ₢AfAAJ - A

Read BCG, assess proven patterns from 12 regime conversions, draft BCG additions for enrollment/enforce/furnish

### 2026-02-22 15:32 - ₢AfAAP - W

Deleted all legacy buv_val_*/buv_env_*/buv_opt_* functions, wrappers, xname validation tests, and testbench enrollment

### 2026-02-22 15:31 - ₢AfAAP - n

Delete legacy buv_val_*/buv_env_*/buv_opt_* functions and wrappers from buv_validation.sh, delete butcvu_XnameValidation.sh test file, remove xname-validation suite from testbench

### 2026-02-22 15:29 - ₢AfAAP - A

Delete buv_val_*/buv_env_*/buv_opt_* from buv_validation.sh, delete butcvu test file, remove testbench enrollment

### 2026-02-22 15:29 - ₢AfAAO - W

Migrated RBRO regime to enrollment+enforce, scrubbed CLI to BCG compliance, attached regime validation at all consumer sites

### 2026-02-22 15:29 - ₢AfAAO - n

Migrate RBRO regime to enrollment+scope_sentinel+enforce, scrub CLI to BCG compliance, attach regime validation to all RBRO consumers, remove redundant inline validation in rbgp_Payor

### 2026-02-22 15:21 - ₢AfAAO - A

Revised: migrate rbro_regime+cli+rbgp_Payor from buv_env_string to enrollment, attach regime validation to all RBRO consumers

### 2026-02-22 15:18 - ₢AfAAO - A

Migrate rbro_regime.sh from buv_env_string to enrollment pattern; butcvu tests deferred to AfAAP

### 2026-02-22 15:15 - ₢AfAAc - W

BURD enrollment+scope_sentinel+enforce, buv sourced in all 7 workbenches and testbench for exec boundary, zburd_enforce added to 9 scrubbed CLI furnish functions

### 2026-02-22 15:15 - ₢AfAAc - n

Remove stale enrollment-ordering comments from launcher and workbench kindle sections

### 2026-02-22 15:08 - ₢AfAAc - A

Enrollment+scope_sentinel+enforce in burd_regime, zburd_enforce added to all scrubbed CLI furnish functions

### 2026-02-22 15:06 - ₢AfAAk - W

Manifold scrub: enrollment+scope_sentinel+enforce([[ =~ ]]) in regime, buc_execute+furnish+buv_report/render in CLI, param1 channel+prefixed commands in zipper, deleted ZRBRA_ROLLUP

### 2026-02-22 15:06 - ₢AfAAk - n

Manifold scrub following RBRN exemplar: enrollment+scope_sentinel+enforce([[ =~ ]]) in regime, buc_execute+furnish+buv_report/render in CLI, param1 channel+prefixed commands in zipper, delete ZRBRA_ROLLUP

### 2026-02-22 15:03 - ₢AfAAk - A

Manifold scrub following RBRN exemplar: enrollment+scope_sentinel+enforce([[ =~ ]]) in regime, buc_execute+furnish+buv_report/render in CLI, param1 channel+prefixed commands in zipper, delete ZRBRA_ROLLUP

### 2026-02-22 15:02 - ₢AfAAj - W

Enrollment+scope_sentinel+enforce(custom COUNTDOWN check) in regime, buc_execute+buv_report/render in CLI, prefixed dispatch, BURE_COLOR upgraded to enum

### 2026-02-22 15:02 - ₢AfAAj - n

Mechanical scrub: enrollment+scope_sentinel+enforce(custom COUNTDOWN check) in regime, buc_execute+buv_report/render in CLI, prefixed dispatch

### 2026-02-22 15:00 - ₢AfAAj - A

Mechanical scrub: enrollment+scope_sentinel+enforce(custom COUNTDOWN check) in regime, buc_execute+buv_report/render in CLI, prefixed dispatch

### 2026-02-22 14:59 - ₢AfAAi - W

Enrollment+scope_sentinel+enforce in regime, buc_execute+buv_report/render in CLI, prefixed dispatch in workbench, reordered launcher for buv-before-burc kindle

### 2026-02-22 14:59 - ₢AfAAi - n

Migrate BURC regime to buv enrollment infrastructure: replace manual validation with buv_enroll/buv_vet/buv_scope_sentinel, refactor CLI to buc_execute dispatch, reorder launcher sourcing so buv kindles before regime

### 2026-02-22 14:55 - ₢AfAAi - A

Mechanical scrub following BURS/RBRS exemplar: enrollment+scope_sentinel in regime, buc_execute+buv_report/render in CLI, prefixed dispatch in workbench

### 2026-02-22 14:53 - ₢AfAAh - W

Scrubbed RBRP: enrollment+scope_sentinel+enforce with custom regex checks in regime, buc_execute+buv_report/render in CLI, updated zipper prefixed commands, enrolled discovered PARENT_TYPE/PARENT_ID vars, deleted rbrp_load

### 2026-02-22 14:53 - ₢AfAAh - n

Migrate RBRP CLI to buc_execute/furnish dispatch and buv enrollment with kindle/enforce separation

### 2026-02-22 14:49 - ₢AfAAh - A

Enrollment+enforce with custom regex checks, RBGC dep, delete rbrp_load, enroll missing REDIRECT_URI, update zipper

### 2026-02-22 14:49 - ₢AfAAg - W

RBRE files do not exist — regime was never implemented, nothing to scrub

### 2026-02-22 14:48 - ₢AfAAf - W

Scrubbed BURS: enrollment+scope_sentinel+enforce in regime, buc_execute+buv_report/render in CLI, updated buw_workbench prefixed commands, added buv kindle to bul_launcher for bootstrap

### 2026-02-22 14:48 - ₢AfAAf - n

Mechanical scrub following RBRN exemplar: enrollment+scope_sentinel in regime, buc_execute+buv_report/render in CLI, update buw_workbench dispatch

### 2026-02-22 14:41 - ₢AfAAf - A

Mechanical scrub following RBRN exemplar: enrollment+scope_sentinel in regime, buc_execute+buv_report/render in CLI, update buw_workbench dispatch

### 2026-02-22 14:40 - ₢AfAAe - W

Scrubbed RBRS: enrollment+scope_sentinel+enforce in regime, buc_execute+buv_report/render in CLI, added RBCC_rbrs_file, updated zipper prefixed commands

### 2026-02-22 14:40 - ₢AfAAe - n

scrub-rbrs-station: migrate to enrollment validation and buc_execute dispatch

### 2026-02-22 14:40 - Heat - T

scrub-rbra-manifold

### 2026-02-22 14:39 - Heat - T

scrub-bure-ambient

### 2026-02-22 14:39 - Heat - T

scrub-burc-singleton

### 2026-02-22 14:32 - Heat - T

scrub-rbrp-singleton

### 2026-02-22 14:25 - Heat - T

scrub-rbre-singleton

### 2026-02-22 14:25 - Heat - T

scrub-burs-singleton

### 2026-02-22 14:14 - ₢AfAAe - A

Mechanical scrub following RBRR exemplar: enrollment+scope_sentinel in regime, buc_execute+buv_report/render in CLI, add RBCC_rbrs_file

### 2026-02-22 14:13 - ₢AfAAF - W

Wrote transformation catalog with 13 recipes and deep per-regime briefs; restructured remaining paces into 7 per-regime paces replacing 2 omnibus paces

### 2026-02-22 14:13 - ₢AfAAF - n

Catalog regime second-pass transformations with per-module briefs and open design decisions

### 2026-02-22 14:10 - Heat - r

moved AfAAk after AfAAj

### 2026-02-22 14:10 - Heat - r

moved AfAAj after AfAAi

### 2026-02-22 14:10 - Heat - r

moved AfAAi after AfAAh

### 2026-02-22 14:10 - Heat - r

moved AfAAh after AfAAg

### 2026-02-22 14:10 - Heat - r

moved AfAAg after AfAAf

### 2026-02-22 14:10 - Heat - r

moved AfAAf after AfAAe

### 2026-02-22 14:10 - Heat - r

moved AfAAe to first

### 2026-02-22 14:10 - Heat - S

scrub-rbra-manifold

### 2026-02-22 14:10 - Heat - S

scrub-bure-ambient

### 2026-02-22 14:10 - Heat - S

scrub-burc-singleton

### 2026-02-22 14:10 - Heat - S

scrub-rbrp-singleton

### 2026-02-22 14:10 - Heat - S

scrub-rbre-singleton

### 2026-02-22 14:10 - Heat - S

scrub-burs-singleton

### 2026-02-22 14:10 - Heat - S

scrub-rbrs-singleton

### 2026-02-22 14:05 - Heat - T

apply-enrollment-to-buk-regimes

### 2026-02-22 14:05 - Heat - T

apply-to-remaining-regimes

### 2026-02-22 13:35 - Heat - n

Post-approval formatting cleanup in rbrr_regime, rbrv_cli, rbrv_regime

### 2026-02-22 13:32 - ₢AfAAF - A

Diff exemplars, catalog transformation recipes, produce per-regime checklist

### 2026-02-22 13:28 - ₢AfAAQ - W

Built 47-case enrollment-validation test suite: 7 test files covering all buv_ types, gating, enforce/report, and multi-scope filtering

### 2026-02-22 13:28 - ₢AfAAQ - n

7 test files + suite registration following butcvu exemplar pattern

### 2026-02-22 13:23 - ₢AfAAQ - A

7 test files + suite registration following butcvu exemplar pattern

### 2026-02-22 13:01 - Heat - S

apply-enrollment-to-buk-regimes

### 2026-02-22 13:01 - Heat - S

apply-enrollment-to-burd-runtime

### 2026-02-22 12:58 - ₢AfAAI - W

BCG scrub of RBRV: enrollment pattern, CLI restructure with buc_execute, list_capture functions for RBRN+RBRV, regime smoke tests fixed

### 2026-02-22 12:58 - ₢AfAAI - n

RBRV enrollment conversion + CLI restructure following rbrn exemplar pattern; added list_capture functions for rbrn/rbrv; updated smoke tests to iterate nameplates/vessels with render+validate per item

### 2026-02-22 12:50 - ₢AfAAI - n

Show available nameplates before dying when moniker missing in rbrn_render and rbrn_validate

### 2026-02-22 12:43 - ₢AfAAI - n

Suppress log files for rbw-rnl ListNameplateRegime tabtarget; listing nameplates does not need logging

### 2026-02-22 12:43 - ₢AfAAI - n

Remove spurious buc_doc_env BUZ_FOLIO warning from rbrn_cli furnish; variable is legitimately empty for list/survey/audit commands

### 2026-02-22 12:36 - ₢AfAAI - A

Enrollment conversion + CLI restructure following rbrn exemplar pattern

### 2026-02-22 12:34 - ₢AfAAK - W

Added rbw-rnl ListNameplateRegime tabtarget; BCG-complied rbrn_regime callers with isolation subshells; moved presentation functions (list, survey) to CLI; added BCG isolation subshell section

### 2026-02-22 12:33 - ₢AfAAK - n

Move fleet_survey to CLI (presentation); inline fleet_audit into rbrn_audit; clean stale regime comments

### 2026-02-22 12:30 - ₢AfAAK - n

Inline zrbrn_list_monikers into CLI rbrn_list with BCG index iteration; delete from regime (zero callers remain)

### 2026-02-22 12:28 - ₢AfAAK - n

BCG-comply rbrn_regime callers: inlined glob, isolation subshells with error handling, format string constant; document isolation+capture hybrid in BCG

### 2026-02-22 12:19 - ₢AfAAK - n

Add rbw-rnl ListNameplateRegime tabtarget; rename rbrn_list→zrbrn_list_monikers; add BCG isolation subshell section

### 2026-02-22 12:03 - ₢AfAAK - A

Rename rbrn_list→zrbrn_list_monikers in regime; add CLI rbrn_list + rbw-nl zipper enrollment + tabtarget

### 2026-02-22 12:01 - ₢AfAAZ - W

Reunified rbcnc/rbcnx into rbrn_cli.sh with differential furnish; removed aspirational two-tier CLI pattern from BCG

### 2026-02-22 12:01 - ₢AfAAZ - n

Merge rbcnc+rbcnx into rbrn_cli.sh with differential furnish gating on command name

### 2026-02-22 11:54 - ₢AfAAZ - A

Merge rbcnc+rbcnx into rbrn_cli.sh with differential furnish gating on command name

### 2026-02-22 11:52 - ₢AfAAY - W

Sources-in-furnish applied to rbcnc/rbcnx CLIs with z_rbw_kit_dir pattern and RBCC_KIT_DIR guard; fixed pre-existing missing rbrr_regime.sh in rbcnx; BURC→BURD in workbench

### 2026-02-22 11:51 - ₢AfAAY - n

Sources-in-furnish: move top-level sources into furnish using BURD_/z_rbw_kit_dir paths, fix BURC→BURD in workbench, add missing rbrr_regime.sh source in rbcnx

### 2026-02-22 11:40 - ₢AfAAY - A

Sources-in-furnish: move top-level sources into furnish using BURD_ paths for rbcnc/rbcnx CLIs plus BURC→BURD fix in workbench

### 2026-02-22 11:38 - ₢AfAAb - W

BCG compliance: inclusion guard, grep→regex, bracket→test, blazon→enroll rename, inline decode_folio, RBR0_FOLIO→BUZ_FOLIO, publicize exec_lookup, find-then-act loop, simplify workbench routing

### 2026-02-22 11:37 - ₢AfAAb - n

Simplify rbw_workbench: qualification gate then single buz_exec_lookup, remove dead help display

### 2026-02-22 11:32 - ₢AfAAb - n

Inline decode_folio, rename RBR0_FOLIO→BUZ_FOLIO, publicize buz_exec_lookup, find-then-act loop, die on not-found

### 2026-02-22 11:04 - ₢AfAAb - n

Move buz_enroll test fixture from _tsuite_setup into rbtb_kindle for BCG compliance

### 2026-02-22 10:53 - ₢AfAAb - n

BCG compliance: inclusion guard die, grep→regex, bracket→test, buz_blazon→buz_enroll rename

### 2026-02-22 10:45 - ₢AfAAb - A

BCG checklist fixes: inclusion guard, grep→regex, blazon→enroll rename, decode_folio simplify, bracket→test

### 2026-02-22 10:43 - Heat - n

Spook: remove stale jjx_reorder order mode from CLAUDE.md, vocjjmc_core.md, and jjc-heat-rail slash command

### 2026-02-22 10:40 - Heat - r

moved AfAAQ after AfAAI

### 2026-02-22 10:40 - Heat - r

moved AfAAI after AfAAK

### 2026-02-22 10:40 - Heat - r

moved AfAAK after AfAAZ

### 2026-02-22 10:40 - Heat - r

moved AfAAZ after AfAAY

### 2026-02-22 10:40 - Heat - r

moved AfAAY after AfAAb

### 2026-02-22 10:40 - Heat - r

moved AfAAb to first

### 2026-02-22 10:35 - Heat - T

apply-to-remaining-regimes

### 2026-02-22 10:33 - ₢AfAAa - W

buv enrollment API unified: regime/group/gate ceremony aligned with AXLA hierarchy; per-variable gates eliminated; RBRR+RBRN converted; nameplate param1 channel fixed

### 2026-02-22 10:33 - ₢AfAAa - n

buv enrollment API: regime/group/gate ceremony, strip per-variable gates, AXLA-aligned; fix nameplate param1 channel

### 2026-02-22 10:27 - Heat - S

buz-zipper-bcg-compliance

### 2026-02-22 09:09 - ₢AfAAa - A

Functional test: run rbrr/rbrn render, verify gated sections, assess wrappability

### 2026-02-22 09:08 - ₢AfAAa - n

BCG-compliant buv_render: locals at top, recite helpers, opt/cond/req derivation; drop rbcr from rbrr_cli

### 2026-02-22 09:05 - ₢AfAAa - n

buv enrollment-render unification: regime_start/section/render infra, convert RBRR+RBRN

### 2026-02-22 09:00 - ₢AfAAa - A

Extend buv with regime_start/section/render, convert RBRR+RBRN enrollments

### 2026-02-22 08:55 - Heat - S

buv-enrollment-render-unification

### 2026-02-22 08:34 - Heat - n

rbrr: stderr capture to temp files, kindle prefixes, integer-indexed loop files

### 2026-02-22 08:29 - Heat - n

BCG: add stderr-capture rule and trailing-newline guard; rbrr_cli: eliminate zrbrr_cli_kindle

### 2026-02-22 08:00 - ₢AfAAF - A

Diff exemplars from 57b1ff99, catalog transformation patterns, produce per-regime checklists

### 2026-02-22 07:59 - Heat - r

moved AfAAK after AfAAZ

### 2026-02-22 07:59 - Heat - r

moved AfAAZ after AfAAY

### 2026-02-22 07:59 - Heat - r

moved AfAAY after AfAAI

### 2026-02-22 07:57 - Heat - T

apply-to-remaining-regimes

### 2026-02-22 07:56 - Heat - T

rbrn-sources-in-furnish

### 2026-02-22 07:56 - Heat - T

sources-in-furnish-migration

### 2026-02-22 07:56 - ₢AfAAX - W

buc_execute passes command name to furnish; BCG documents differential furnish pattern

### 2026-02-22 07:56 - ₢AfAAX - n

Pass command name to furnish as $1 in buc_execute; document differential furnish pattern in BCG

### 2026-02-22 07:52 - ₢AfAAX - A

One-line buc_execute change + BCG furnish docs

### 2026-02-22 07:51 - ₢AfAAW - W

Added BURD_TOOLS_DIR, BURD_BUK_DIR, BURD_TABTARGET_DIR to dispatch setup and regime; documented in BCG and BUSD spec

### 2026-02-22 07:51 - ₢AfAAW - n

Add BURD_TOOLS_DIR, BURD_BUK_DIR, BURD_TABTARGET_DIR as dispatch-provided variables for exec boundary crossing

### 2026-02-22 07:45 - ₢AfAAW - A

Additive: set BURD_TOOLS_DIR/BUK_DIR/TABTARGET_DIR in dispatch, add to burd_regime, BCG; keep BURC exports for now

### 2026-02-22 07:39 - Heat - r

moved AfAAK after AfAAZ

### 2026-02-22 07:39 - Heat - S

rbrn-cli-reunification

### 2026-02-22 07:38 - Heat - S

sources-in-furnish-migration

### 2026-02-22 07:38 - Heat - S

buc-execute-differential-furnish

### 2026-02-22 07:38 - Heat - S

burd-export-consolidation

### 2026-02-22 07:14 - ₢AfAAK - A

CLI rbrn_list_nameplates in rbcnc, blazon rbw-nl singleton, tabtarget, workbench help

### 2026-02-21 18:40 - ₢AfAAV - W

BCG updated with two-tier CLI pattern, weakened module-CLI unity rule, and rbc{regime}c/rbc{regime}x naming convention; RBRN is the first exemplar

### 2026-02-21 18:29 - ₢AfAAE - W

Split rbrn_cli.sh into two-tier BCG-compliant CLIs (rbcnc/rbcnx); resolved Tier 3 survey/audit furnish violation; established RBRN as manifold exemplar

### 2026-02-21 18:29 - ₢AfAAE - n

Fix stale furnish comment in rbcnc_cli.sh: remove survey/audit which now live in rbcnx_cli.sh

### 2026-02-21 17:49 - Heat - T

bcg-weaken-module-cli-unity

### 2026-02-21 17:42 - Heat - S

bcg-weaken-module-cli-unity

### 2026-02-21 12:33 - Heat - T

scrub-rbrn-manifold-exemplar

### 2026-02-21 12:25 - ₢AfAAE - n

BCG cleanup: move zrbcr_kindle to furnish; add buv_scope_sentinel and buv_docker_env to BUK

### 2026-02-21 12:03 - ₢AfAAE - n

Restructure RBRN as manifold exemplar: buv enrollment, RBR0_FOLIO channel, buc_execute pattern, workbench collapse

### 2026-02-21 11:28 - ₢AfAAE - F

Executing bridled pace via sonnet agent

### 2026-02-21 09:18 - ₢AfAAE - B

arm | scrub-rbrn-manifold-exemplar

### 2026-02-21 09:18 - Heat - T

scrub-rbrn-manifold-exemplar

### 2026-02-21 08:44 - ₢AfAAU - W

Root cause: uncommitted Study/ changes caused rbf_build to abort silently inside ConjureArk; test passes with clean working tree

### 2026-02-21 08:30 - ₢AfAAU - A

Diagnose: run with BUT_VERBOSE, read test code, check AfAAR context, trace set-e subshell interaction

### 2026-02-21 08:28 - Heat - r

moved AfAAU to first

### 2026-02-21 08:26 - ₢AfAAD - W

Added z_buz_channel_roll, zbuz_decode_folio, extended buz_blazon with optional 5th channel param, integrated into zbuz_exec_lookup

### 2026-02-21 08:26 - ₢AfAAD - n

Add repeat execution with statistical aggregation to SMPT trials

### 2026-02-21 08:26 - ₢AfAAD - n

Add channel infrastructure: buz_blazon 5th param, zbuz_decode_folio, RBR0_FOLIO regime-selection

### 2026-02-21 08:25 - Heat - S

debug-ark-lifecycle-silent-failure

### 2026-02-21 08:10 - ₢AfAAD - A

Extend buz_blazon 5th param channel, add zbuz_decode_folio, integrate into zbuz_exec_lookup

### 2026-02-21 08:04 - ₢AfAAC - W

BCG literal constants concept, buv_enforce→buv_vet rename, zrbrr_enforce, RBCC source-time constants, CLI source ordering fixes, all regime tests passing

### 2026-02-21 08:04 - ₢AfAAC - n

Add SMPT study: model prompt tuning experiment with BUK workbench integration

### 2026-02-21 08:01 - ₢AfAAC - n

BCG literal constants, buv_enforce→buv_vet rename, zrbrr_enforce, RBCC source-time constants, fix CLI source ordering

### 2026-02-21 06:42 - ₢AfAAC - A

Enrollment-based BCG scrub: restructure kindle with buv_enroll, delete rbrr_load, buc_execute CLI template, update 10+ callers

### 2026-02-21 06:37 - ₢AfAAT - W

Moved rbtc* test cases to Tools/rbw/rbts/, butc* to Tools/buk/buts/; added RBTB_RBTS_DIR and RBTB_BUTS_DIR DRY constants

### 2026-02-21 06:37 - ₢AfAAS - W

Added RBSAJ/RBSAO access probe specs, rbap_AccessProbe.sh implementation, rbtcap test cases; all 4 test cases pass (20/20 iterations HTTP 200)

### 2026-02-21 06:37 - ₢AfAAT - n

Remove old file locations after move to rbts/ and buts/

### 2026-02-21 06:37 - ₢AfAAT - n

Move test cases to rbts/ and buts/ directories; add DRY constants RBTB_RBTS_DIR and RBTB_BUTS_DIR; add access-probe test suite wiring

### 2026-02-21 06:37 - ₢AfAAS - n

Add RBSAJ/RBSAO access probe specs, rbap_AccessProbe.sh implementation module, wire into RBS0

### 2026-02-21 06:36 - Heat - T

rbts-test-suites

### 2026-02-21 06:26 - Heat - S

rbtd-test-directory

### 2026-02-21 06:09 - ₢AfAAS - A

Read existing spec patterns, create RBSAJ + RBSAO specs, wire into RBS0, add testbench probes before ark-lifecycle

### 2026-02-21 06:07 - ₢AfAAR - W

Diagnosed curl nonzero exit with 0-byte OAuth response; replaced 3 x 2>/dev/null with kindle-constant stderr temp files for forensic visibility; slated access-probe-specs-and-tests pace

### 2026-02-21 06:06 - Heat - r

moved AfAAS to first

### 2026-02-21 06:06 - ₢AfAAR - n

Replace 2>/dev/null with kindle-constant stderr temp files for forensic visibility

### 2026-02-21 06:05 - Heat - S

access-probe-specs-and-tests

### 2026-02-21 05:35 - ₢AfAAR - A

Read OAuth code path, test harness, compare forensic artifacts, identify root cause

### 2026-02-20 09:10 - Heat - S

debug-ark-lifecycle-oauth-failure

### 2026-02-20 08:54 - Heat - T

bcg-updates-from-proven-patterns

### 2026-02-20 08:54 - Heat - T

consider-cli-regime-split-for-bcg

### 2026-02-20 08:53 - Heat - T

bcg-add-enforce-pattern

### 2026-02-20 08:34 - ₢AfAAH - A

Update BCG-BashConsoleGuide.md: boilerplate table, furnish template, enrollment section, maturity checklist

### 2026-02-20 08:33 - ₢AfAAN - W

Added buv_ enrollment infrastructure: zbuv_kindle, 7 parallel rolls, 11 enroll functions, zbuv_check_predicate, buv_enforce, buv_report

### 2026-02-20 08:33 - ₢AfAAN - n

Add enrollment infrastructure to buv_validation.sh: kindle/sentinel, 7 rolls, 11 enroll functions, internal predicate, enforce/report dual consumption

### 2026-02-20 08:33 - Heat - S

buv-enrollment-tests

### 2026-02-20 08:20 - ₢AfAAN - A

Add enrollment infrastructure to buv_validation.sh: kindle/sentinel, 7 rolls, 11 enroll functions, internal predicate, enforce/report dual consumption

### 2026-02-20 08:15 - ₢AfAAB - W

Relocated BURD_VERBOSE and BURD_COLOR to BURE_ prefix across 16 files (bash, adoc, md); fixed nonexistent buv_env_integer to buv_env_enum; added level 3 to enum; qualification passes

### 2026-02-20 08:13 - ₢AfAAM - W

Exfiltrated buv_qualify_tabtargets to new buq_qualify.sh module, updated rbq_Qualify.sh caller, QualifyAll passes identically

### 2026-02-20 08:13 - ₢AfAAM - n

Move VERBOSE/COLOR from BURD to BURE regime, extract tabtarget qualification into buq_qualify.sh

### 2026-02-20 08:05 - Heat - T

buq-exfiltrate-qualify

### 2026-02-20 07:58 - ₢AfAAA - W

Removed RBRG ghost regime: cleared RBSA voicings/definitions/section, deleted RBSRG spec, stripped validate_pat/login_ghcr from rbv_PodmanVM.sh

### 2026-02-20 07:58 - ₢AfAAA - n

Remove RBRG from RBSA mappings+definitions+section, delete RBSRG spec, remove validate_pat/login_ghcr from rbv_PodmanVM.sh

### 2026-02-20 07:54 - ₢AfAAA - A

Remove RBRG from RBSA mappings+definitions+section, delete RBSRG spec, remove validate_pat/login_ghcr from rbv_PodmanVM.sh

### 2026-02-19 19:11 - Heat - T

final-paddock-hygiene-review

### 2026-02-19 19:10 - Heat - T

apply-to-remaining-regimes

### 2026-02-19 19:10 - Heat - T

consider-cli-regime-split-for-bcg

### 2026-02-19 19:09 - Heat - T

add-nameplate-list-tabtarget

### 2026-02-19 19:09 - Heat - T

scrub-rbrr-singleton-exemplar

### 2026-02-19 19:08 - Heat - T

buv-enrollment-infrastructure

### 2026-02-19 18:56 - Heat - T

apply-to-remaining-regimes

### 2026-02-19 18:56 - Heat - T

scrub-rbrv-manifold-structural

### 2026-02-19 18:56 - Heat - T

document-regime-transformations

### 2026-02-19 18:56 - Heat - T

scrub-rbrn-manifold-exemplar

### 2026-02-19 18:55 - Heat - T

scrub-rbrr-singleton-exemplar

### 2026-02-19 18:55 - Heat - T

bcg-add-enforce-pattern

### 2026-02-19 18:54 - Heat - S

delete-legacy-buv-functions

### 2026-02-19 18:53 - Heat - S

audit-legacy-buv-callers

### 2026-02-19 18:53 - Heat - S

buv-enrollment-infrastructure

### 2026-02-19 18:52 - Heat - S

buq-exfiltrate-qualify

### 2026-02-19 09:22 - Heat - f

racing

### 2026-02-19 09:21 - Heat - n

Update paddock: add workbench imprint translation docs, replace line numbers with search patterns, mark RBRR grep fix as done, add RBRV structural debt note

### 2026-02-19 09:20 - Heat - T

scrub-rbrn-manifold-exemplar

### 2026-02-19 09:19 - Heat - S

final-paddock-hygiene-review

### 2026-02-19 09:19 - Heat - S

add-nameplate-list-tabtarget

### 2026-02-19 09:18 - Heat - S

consider-cli-regime-split-for-bcg

### 2026-02-19 09:18 - Heat - T

relocate-ambient-burd-vars-to-bure

### 2026-02-19 09:18 - Heat - S

scrub-rbrv-manifold-structural

### 2026-02-19 09:09 - Heat - S

bcg-add-enforce-pattern

### 2026-02-19 09:02 - Heat - S

apply-to-remaining-regimes

### 2026-02-19 09:02 - Heat - S

document-regime-transformations

### 2026-02-19 09:01 - Heat - S

scrub-rbrn-manifold-exemplar

### 2026-02-19 09:01 - Heat - S

buz-channel-infrastructure

### 2026-02-19 09:00 - Heat - S

scrub-rbrr-singleton-exemplar

### 2026-02-19 08:47 - Heat - T

relocate-ambient-burd-vars-to-bure

### 2026-02-19 08:42 - Heat - D

restring 1 paces from ₣AO

### 2026-02-19 08:27 - Heat - S

remove-rbrg-regime

### 2026-02-19 08:26 - Heat - d

paddock curried

### 2026-02-19 08:22 - Heat - d

paddock curried

### 2026-02-19 08:21 - Heat - N

rbk-regime-second-pass

