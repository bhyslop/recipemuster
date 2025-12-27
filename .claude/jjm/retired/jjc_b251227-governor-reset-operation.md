# Steeplechase: Governor Reset Operation

---
### 2025-12-27 - standardize-sa-naming-patterns - APPROACH
**Mode**: manual
**Proposed approach**:
- Update .adoc specs first (RBSDC, RBSDL) to use `mason-{depot_name}` pattern
- Add RBGC_GOVERNOR_PREFIX and RBGC_MASON_PREFIX constants
- Update rbgp_depot_create to use `mason-{depot_name}` for Mason SA
- Update rbgp_depot_list validation to match new pattern
---

---
### 2025-12-27 - standardize-sa-naming-patterns - COMPLETE
**Changes made**:
- `lenses/rbw-RBSDC-depot_create.adoc`: Mason accountId `rbw-«NAME»-mason` → `mason-«NAME»`
- `lenses/rbw-RBSDL-depot_list.adoc`: Mason SA check pattern updated
- `Tools/rbw/rbgc_Constants.sh`: Added `RBGC_GOVERNOR_PREFIX` and `RBGC_MASON_PREFIX`, removed obsolete `RBGC_MASON_NAME`
- `Tools/rbw/rbgd_DepotConstants.sh`: Added `RBGD_DEPOT_NAME` extraction, updated `RBGD_MASON_EMAIL` to use prefix
- `Tools/rbw/rbgp_Payor.sh`: Updated `rbgp_depot_create` and `rbgp_depot_list` to use `${RBGC_MASON_PREFIX}-${depot_name}`
---

---
### 2025-12-27 - revise-rbsgs-payor-coverage - APPROACH
**Mode**: manual
**Proposed approach**:
- Add `{rbtgo_governor_reset}` to RBAGS mapping section
- Update Depots and Roles to clarify Payor creates depots AND governors
- Update Phase 2: Setup to clarify governor_create is a Payor operation
- Add depot lifecycle operations (depot_list, depot_destroy) to Phase 2 or new section
- Add governor_reset to Recovery section for credential rotation
---

---
### 2025-12-27 - revise-rbsgs-payor-coverage - COMPLETE
**Changes made**:
- `lenses/rbw-RBAGS-AdminGoogleSpec.adoc`: Added `{rbtgo_governor_reset}` mapping
- `lenses/rbw-RBSGS-GettingStarted.adoc`:
  - Depots and Roles: Clarified Payor creates depots AND governors; Governor creates directors/retrievers
  - Phase 2: Setup: Clarified governor_create is a Payor operation
  - Phase 2: Setup: Added depot_list and depot_destroy mentions
  - Recovery: Added governor_reset for credential rotation
---

---
### 2025-12-27 - revise-governor-spec-reset-semantics - APPROACH
**Mode**: manual
**Proposed approach**:
- Rename file: RBSGC → RBSGR
- Rewrite intro to describe reset semantics (idempotent create-or-replace)
- Add «INPUT_DEPOT_PROJECT_ID» as required argument
- Add {rbtoe_payor_authenticate} step at start
- Add step to find/delete existing governor-* SAs before creating new one
- Specify governor SA naming: governor-{timestamp}
- Fix typo line 77
- Update RBAGS mapping and include directive
---

---
### 2025-12-27 - revise-governor-spec-reset-semantics - COMPLETE
**Changes made**:
- Created `lenses/rbw-RBSGR-governor_reset.adoc` with reset semantics:
  - Idempotent create-or-replace operation
  - `«INPUT_DEPOT_PROJECT_ID»` as required argument
  - `{rbtoe_payor_authenticate}` step at start
  - Find/delete existing `governor-*` SAs before creating new one
  - Governor SA naming: `governor-{timestamp}`
  - Fixed typo (was `{{rbbc_store}`)
- Deleted `lenses/rbw-RBSGC-governor_create.adoc`
- Updated `lenses/rbw-RBAGS-AdminGoogleSpec.adoc`:
  - Removed `rbtgo_governor_create` mapping
  - Updated section anchor and include directive
  - Updated error message reference
- Updated `lenses/rbw-RBSGS-GettingStarted.adoc` to use `rbtgo_governor_reset`
---

---
### 2025-12-27 - implement-rbgp-governor-reset - APPROACH
**Mode**: manual
**Proposed approach**:
- Add function to rbgp_Payor.sh following existing patterns
- Use zrbgp_authenticate_capture for OAuth token
- List/delete existing governor-* SAs before creating new one
- Create governor-{timestamp} SA with roles/owner
- Generate RBRA file with key, similar to zrbgg_create_service_account_with_key
- Add new infix constants for HTTP operations
---

---
### 2025-12-27 - implement-rbgp-governor-reset - COMPLETE
**Changes made**:
- Added infix constants: GOV_LIST_SA, GOV_DELETE_SA, GOV_CREATE_SA, GOV_VERIFY_SA, GOV_KEY, GOV_IAM
- Implemented `rbgp_governor_reset` function (~160 lines):
  - Validates depot_project_id argument (pattern: rbw-{name}-{timestamp})
  - Authenticates as Payor via OAuth
  - Validates depot project exists and is ACTIVE
  - Lists and deletes existing governor-* service accounts
  - Creates governor-{timestamp} service account
  - Grants roles/owner on depot project
  - Generates RBRA file with service account key
---

---
### 2025-12-27 - update-itch - COMPLETE
**Changes made**:
- Moved itch `rbgp-create-governor` to scars
- Added lesson learned: pristine-state vs idempotent semantics

**Heat complete** - all paces done.
---
