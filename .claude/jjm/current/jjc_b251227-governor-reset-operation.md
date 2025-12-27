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
