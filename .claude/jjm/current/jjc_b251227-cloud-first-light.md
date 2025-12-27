# Steeplechase: Cloud First Light

---
### 2025-12-27 08:15 - validate-tabtarget-to-rbags-mapping - APPROACH
**Mode**: manual
**Proposed approach**:
- Locate tabtarget scripts in `tt/` directory matching `rbw-*.sh` pattern
- Read the RBAGS specification to understand the canonical operation names
- Create a mapping table showing tabtarget script → RBAGS operation
- Flag any mismatches or unclear associations

### 2025-12-27 08:30 - validate-tabtarget-to-rbags-mapping - COMPLETE
**Findings**:
- 11 operations verified with correct tabtargets
- governor_reset: missing tabtarget, intended `rbw-PG.PayorGovernorReset.sh`
- image_list: unimplemented, intended `rbw-il.ImageList.sh`
- image_retrieve: uses old mbd.dispatch, needs modernization to BUD
- Updated heat Operation Status table with tabtarget mappings
---

---
### 2025-12-27 10:47 - exercise-payor-install - APPROACH
**Mode**: manual
**Proposed approach**:
- Verify prerequisites: OAuth JSON file from establish procedure, RBRP_BILLING_ACCOUNT_ID set
- Run `tt/rbw-PI.PayorInstall.sh <path-to-oauth-json>`
- Complete browser authorization flow (3 screens: unverified app warning, permission grant, auth code)
- Paste authorization code when prompted
- Confirm credentials stored in `~/.rbw/rbro.env` and OAuth test succeeds

### 2025-12-27 12:21 - exercise-payor-install - COMPLETE
**Result**: Success - existing credentials detected, OAuth test passed, payor project access verified
---

---
### 2025-12-27 13:45 - exercise-depot-create-practice - APPROACH
**Mode**: manual
**Proposed approach**:
- Run `tt/rbw-PC.PayorDepotCreate.sh test us-central1` to create a practice depot
- Observe operation sequence: OAuth auth → project creation → billing link → API enablement → bucket → repository → Mason SA
- Watch for HTTP errors or permission issues at each step
- On success, note the generated `RBRR_DEPOT_PROJECT_ID` (pattern: `rbw-test-YYYYMMDDHHMM`)

### 2025-12-27 13:50 - exercise-depot-create-practice - BUG FOUND
**Issue**: Billing link failed with HTTP 400 - wrong API endpoint
- Code used CRM API `:setBillingInfo` but should use Cloud Billing API `/billingInfo`
- Payload had extra fields `projectId` and `billingEnabled` not recognized by billing API

**Abandoned resource**: Project `rbw-test-YYYYMMDDHHMM` created without billing (exact ID in GCP console)

**Code fixes applied**:
- Added `RBGC_API_ROOT_CLOUDBILLING` and `RBGC_CLOUDBILLING_V1` to `rbgc_Constants.sh`
- Fixed URL and payload in `rbgp_Payor.sh:666-676`

**Spec fix needed**: Update RBAGS RBSDC operation to reference Cloud Billing API (deferred per heat guidelines - apply before pace complete)
---
