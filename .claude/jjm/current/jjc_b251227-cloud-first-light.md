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

---
### 2025-12-27 14:30 - add-iam-preflight-verification - APPROACH
**Mode**: manual
**Proposed approach**:
- Add IAM pre-flight verification step after API enablement in `rbgp_depot_create`
- Poll `artifactregistry.repositories.list` endpoint until HTTP 200 or timeout
- Use existing `RBGC_EVENTUAL_CONSISTENCY_SEC` and `RBGC_MAX_CONSISTENCY_SEC` constants for timing
- After code fix, update RBSDC spec with: (1) IAM pre-flight step, (2) billing API correction, (3) CRM v3 project number extraction, (4) GCS constant reference

### 2025-12-27 15:25 - add-iam-preflight-verification - COMPLETE
**Code changes**:
- Added IAM pre-flight polling to `rbgp_Payor.sh` lines 697-721
- Polls `artifactregistry.repositories.list` after API enablement
- Uses `RBGC_EVENTUAL_CONSISTENCY_SEC` (3s) and `RBGC_MAX_CONSISTENCY_SEC` (90s)

**Spec changes** (`lenses/rbw-RBSDC-depot_create.adoc`):
- Fixed project number extraction for CRM v3 (`.name` field)
- Replaced obsolete "Grant Payor Permissions" step with "Verify IAM Propagation" step

**Testing notes**:
- First test run: IAM pre-flight passed, failed at bucket creation (bucket existed from prior run)
- Second test run (test2): Hit billing quota limit (external account issue)
- Third test run: 403 on payor project - user accidentally deleted `rbrp-payor-proto-3` during cleanup

**Result**: Code fix verified working (IAM step passed in first test). Payor project must be re-established before further testing.

**Heat restructured**: Removed prior "Exercise payor_install" from Done (invalidated by payor deletion). Heat now restarts cloud exercise from payor_establish. Code/spec work preserved in Done.
---

---
### 2025-12-28 - exercise-payor-establish - APPROACH
**Mode**: manual
**Proposed approach**:
- Follow RBSPE procedure: create GCP project, configure billing, enable APIs, setup OAuth consent, create OAuth client
- Project naming: use `rbw-payor` if available, else `rbw-payor-NNNN` suffix
- Record the billing account ID (XXXXXX-XXXXXX-XXXXXX format)
- Download the OAuth client JSON file (critical - secret only visible once)
- Save JSON file in a known location for subsequent `payor_install` pace

### 2025-12-28 - exercise-payor-establish - WRAP
**Mode**: manual
**Outcome**: Fixed guide display bugs, documented IAM delay, added RBGC_PAYOR_APP_NAME constant
---

---
### 2025-12-28 - exercise-payor-install - APPROACH
**Mode**: manual
**Proposed approach**:
- Run `tt/rbw-PI.PayorInstall.sh <path-to-downloaded-oauth-json>`
- Complete browser OAuth flow (unverified app warning → permissions → auth code)
- Paste authorization code when prompted
- Verify credentials stored in `~/.rbw/rbro.env` and OAuth test succeeds

### 2025-12-28 09:55 - exercise-payor-install - WRAP
**Mode**: manual
**Outcome**: Created BUG module, fixed token security, updated OAuth flow docs, refactored payor_establish
---

---
### 2025-12-28 09:57 - exercise-depot-create-practice - RUN 1
**Bug Found**: Mason service account creation failed with HTTP 400: "Unknown name 'displayName'"
- **Root Cause**: JSON payload had incorrect structure - `displayName` was at top level instead of nested in `serviceAccount` object
- **Working Pattern**: Governor SA creation (rbgg_Governor.sh) uses correct structure: `{ accountId, serviceAccount: { displayName, description } }`
- **Broken Pattern**: Mason SA creation (rbgp_Payor.sh:750-756) used flat structure: `{ accountId, displayName }`
- **Fix Applied**: Wrapped `displayName` in `serviceAccount` object to match IAM API requirements
- **Code Location**: rbgp_Payor.sh lines 754-757

**Retrying with fix...**
---

---
### 2025-12-28 10:00 - exercise-depot-create-practice - RUN 2
**Bug Found**: Mason service account creation failed with HTTP 403: IAM API not enabled in project
- **Root Cause**: IAM API was enabled but hadn't propagated to all systems yet. Existing preflight check only validates ArtifactRegistry, not IAM itself. Service account creation proceeded before IAM was ready.
- **Google Cloud Fact**: IAM policy changes can take 2-7+ minutes to propagate across GCP systems
- **Fix Applied**: Added IAM-specific preflight check (rbgp_Payor.sh:745-769) that:
  - Polls `projects.serviceAccounts.list` endpoint (safe, read-only operation)
  - Uses same eventual consistency pattern as ArtifactRegistry check (3s intervals, 90s timeout)
  - Waits for HTTP 200 before proceeding to service account creation
- **Code Location**: rbgp_Payor.sh lines 745-769

**Retrying with both fixes...**
---

---
### 2025-12-28 10:26 - exercise-depot-create-practice - SESSION 1 COMPLETE
**Status**: PAUSED at billing account link error

**All bugs fixed and committed**:
1. ✓ Mason SA JSON payload (serviceAccount wrapper) - rbgp_Payor.sh:779-784
2. ✓ IAM API preflight check (quota project propagation) - rbgp_Payor.sh:745-769
3. ✓ IAM module integration (source + kindle + sentinel) - rbgp_cli.sh, rbgp_Payor.sh

**Last error**: HTTP 400 "Precondition check failed" on billing account link
- Response: `cloudbilling.googleapis.com/v1/projects/rbwg-d-test-*/billingInfo`
- Possible causes: quota limits, billing account constraints, concurrent project creation

**Resume Plan**:
1. Check GCP billing account constraints
2. Verify no project quota exhaustion
3. Cleanup any abandoned test projects from earlier runs
4. Review "Precondition check failed" error details
5. Resume depot_create exercise with all three bug fixes in place

**Key Context for Next Session**:
- Payor project: rbwg-p-251228075220
- IAM API now enabled in payor project (quota project requirement)
- All Module architecture fixes verified against BCG standards
- Practice depot name: "test"
---

---
### 2025-12-28 10:55 - exercise-depot-create-practice - SESSION 2
**Status**: PAUSED - billing quota propagation delay

**Root cause identified**: Cloud billing quota exceeded for account 0173BC-6A77FA-3796BC
- Error buried in API response as `google.rpc.QuotaFailure`
- Created itch `rbgp-billing-quota-detection` for better error reporting

**Remediation performed**:
1. Installed gcloud CLI via Homebrew (required python3 symlink fix)
2. Listed 16 projects in DELETE_REQUESTED state consuming quota
3. Unlinked all 18 projects from billing account via `gcloud billing projects unlink`
4. Now only payor project (rbwg-p-251228075220) linked to billing

**Still blocked**: Billing quota changes need propagation time (10-15 minutes per Google docs, up to 36 hours in rare cases)

**Resume Plan**:
1. Wait for billing quota propagation
2. Retry `tt/rbw-PC.PayorDepotCreate.sh test us-central1`
3. If still blocked, may need to request quota increase at: https://support.google.com/code/contact/billing_quota_increase

**Useful commands installed**:
- `gcloud billing projects list --billing-account=0173BC-6A77FA-3796BC` - check linked projects
- `gcloud projects list --filter="lifecycleState:DELETE_REQUESTED"` - list pending deletion
---

---
### 2025-12-28 17:48 - exercise-depot-create-practice - WRAP
**Mode**: manual
**Outcome**: Fixed repo IAM policy (use correct depot project ID), fixed depot tracking (CRM v1 instead of v3). Practice depot rbwg-d-test-251228174809 created successfully with all infrastructure.
---

---
### 2025-12-28 17:50 - exercise-depot-list - APPROACH
**Mode**: manual
**Proposed approach**:
- Run `tt/rbw-ld.DepotList.sh` to list all accessible depot projects
- Verify the newly-created practice depot `rbwg-d-test-251228174809` appears in the listing
- Confirm it shows correct metadata (project ID, region, repository name)

### 2025-12-28 18:00 - exercise-depot-list - WRAP
**Mode**: manual
**Outcome**: Fixed missing HTTP status check function (use rbgu_http_code_capture). Listed 8 depot projects; 3 complete with Mason SA, 5 broken. Practice depot correctly shows as COMPLETE.
---

---
### 2025-12-28 18:15 - depot-destroy-refinement - PRE-WORK
**Mode**: manual (out-of-band, before exercise-depot-destroy pace)

**Context**: User learned during testing that deleting projects without unlinking billing first causes 30-day quota consumption. Decided to fix depot_destroy before debugging it.

**Research performed**:
- Reviewed current `rbgp_depot_destroy` implementation (lines 825-984)
- Found billing step was AFTER deletion (wrong order) and used wrong API
- Web searched Cloud Billing API documentation
- Confirmed correct endpoint: `PUT cloudbilling.googleapis.com/v1/projects/{id}/billingInfo`
- Confirmed correct body: `{"billingAccountName":""}`

**Implementation changes** (`rbgp_Payor.sh`):
- Added new step "Unlink billing account" BEFORE project deletion
- Uses Cloud Billing API with correct endpoint and body format
- Fixed `rbgu_http_is_ok` bug (same as depot_list) using `rbgu_http_code_capture`
- Removed old incorrect post-deletion billing step

**Spec update** (`rbw-RBSDD-depot_destroy.adoc`):
- Moved billing unlink from optional step 7 to mandatory step 5
- Documented rationale: releases quota immediately vs 30-day hold
- Removed obsolete `«DELETION_TIME»` storage

**Commits**:
1. `a48c660` - Fix depot_destroy: unlink billing BEFORE delete
2. `1fd8bbd` - Update RBSDD spec: billing unlink now mandatory
---

---
### 2025-12-28 - exercise-depot-destroy - APPROACH
**Mode**: manual
**Proposed approach**:
- Run `tt/rbw-PD.PayorDepotDestroy.sh rbwg-d-test-251228174809` to destroy the practice depot
- Watch sequence: billing unlink → project delete request → tracking file update
- Verify HTTP responses show success at each step
- Confirm depot removed from `depot_list` output afterward

### 2025-12-28 18:48 - exercise-depot-destroy - WRAP
**Mode**: manual
**Outcome**: Replaced DEBUG_ONLY with RBGP_CONFIRM_DESTROY, fixed CRM v3 `.state` field parsing, deleted 4 test depots.

**Commits**:
1. `80ee902` - Replace DEBUG_ONLY with RBGP_CONFIRM_DESTROY for depot_destroy
2. `e6d58d2` - Fix CRM v3 API field: .lifecycleState → .state in JSON parsing
---

---
### 2025-12-28 18:55 - exercise-depot-create-for-keeps - APPROACH
**Mode**: manual
**Proposed approach**:
- Run `tt/rbw-PC.PayorDepotCreate.sh proto us-central1` using Keeper Depot settings
- Verify all steps complete: project creation, billing link, APIs, bucket, repository, Mason SA
- Record generated Project ID in heat's Keeper Depot section
- Confirm depot appears as COMPLETE in `depot_list` output
---

---
### 2025-12-30 09:00 - exercise-depot-create-for-keeps - SESSION 2
**Context**: User reports quota increase approved. Retrying keeper depot creation.
**Proposed approach**:
- Run `tt/rbw-PC.PayorDepotCreate.sh proto us-central1`
- Monitor for quota errors vs normal progress
- On success, record Project ID in heat file
- Verify via `tt/rbw-ld.DepotList.sh`

**Bug Found**: Mason SA propagation race condition
- SA creation returned HTTP 200 but setIamPolicy failed with HTTP 400 "Service account does not exist"
- Root cause: SA created but not yet visible in IAM system

**Fix Applied**:
1. Added `rbgu_poll_get_until_ok` helper in rbgu_Utility.sh:450-477 (BCG-compliant polling)
2. Added propagation check in rbgp_Payor.sh:794-796 after Mason SA creation

**Result**: Depot creation succeeded on retry
- Keeper depot: `rbwg-d-proto-251230073755`
- Propagation delay observed: 3 seconds (HTTP 404 → 200)
- Orphaned depots from failed attempts: `rbwg-d-proto-251230072516`, `rbwg-d-proto-251230073558`
---
