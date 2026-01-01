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

**Additional fixes after bucket IAM failure**:
- Refactored 2 existing propagation checks in rbgp_Payor.sh to use new helper
- Added retry logic to `rbgi_add_bucket_iam_role` for "SA does not exist" errors
- Final keeper depot: `rbwg-d-proto-251230080456`
- Cleaned up 4 orphaned depots
---

---
### 2025-12-30 08:15 - exercise-depot-create-for-keeps - WRAP
**Outcome**: Added rbgu_poll_get_until_ok helper, bucket IAM retry logic. Keeper depot: rbwg-d-proto-251230080456
---

---
### 2025-12-30 08:20 - exercise-governor-reset - APPROACH
**Mode**: manual
**Proposed approach**:
- Create tabtarget `tt/rbw-PG.PayorGovernorReset.sh` (pattern from rbw-PC)
- Run against keeper depot: `tt/rbw-PG.PayorGovernorReset.sh rbwg-d-proto-251230080456`
- Verify RBRA file produced at RBRR_GOVERNOR_RBRA_FILE path
- Debug any issues following heat protocol

### 2025-12-30 08:45 - exercise-governor-reset - WRAP
**Outcome**: Created tabtarget, exercised on keeper depot, RBRA file produced successfully.
---

---
### 2025-12-30 09:10 - exercise-director-create - APPROACH
**Mode**: manual
**Proposed approach**:
- Fix coordinator routing mismatch (rbgg_director_create → rbgg_create_director)
- Run `tt/rbw-GD.GovernorDirectorCreate.sh rbwg-d-proto-251230080456`
- Verify RBRA file produced
- Debug any issues following heat protocol

### 2025-12-30 09:30 - exercise-director-create - BLOCKED
**Issue**: Deep architectural mismatch discovered between code and spec.

**Findings**:
1. Code uses `RBRR_ADMIN_RBRA_FILE` but spec defines `RBRR_GOVERNOR_RBRA_FILE`
2. `rbgu_get_admin_token_capture()` reads from RBRR_ADMIN_RBRA_FILE
3. `rbgg_cli.sh` uses old validator path (rbrr.validator.sh) not new regime pattern (rbrr_regime.sh)
4. Several functions in rbga_ArtifactRegistry.sh, rbgb_Buckets.sh, rbgp_Payor.sh appear to be dead code

**Required fixes before proceeding**:
1. Align RBRR_ADMIN_RBRA_FILE → RBRR_GOVERNOR_RBRA_FILE throughout codebase
2. Wire rbgg_cli.sh to use proper regime validation
3. Investigate and remove/document dead code

**Deferred to**: Fresh session with focused prompt. Added dead code audit pace.
---

---
### 2025-12-30 10:58 - exercise-director-create - WRAP
**Outcome**: Fixed 6 bugs across rbgp/rbgg/rbgi/rbgd; created director-eta successfully.

**Bugs fixed**:
1. rbgp_Payor.sh: Added cloudresourcemanager to depot API list
2. rbgg_Governor.sh: Fixed token/label parameter order in rbgi_add_project_iam_role calls (3 occurrences)
3. rbgg_Governor.sh: Changed RBGC_MASON_EMAIL → RBGD_MASON_EMAIL
4. rbgg_Governor.sh: Added missing token argument to rbgi_add_sa_iam_role call
5. rbgi_IAM.sh: Clarified z_member_email param naming (function adds serviceAccount: prefix)
6. rbgd_DepotConstants.sh: Fixed RBGD_GCS_BUCKET derivation to match depot_create naming pattern

**Itches added**:
- rbsdi-instance-constraints: Document INPUT_INSTANCE parameter constraints
- rbsdi-sa-prefix-mismatch: Fix spec to match code SA naming prefixes

**Leftover SAs** (for sa_list/sa_delete testing): proto, alpha, beta, gamma, delta, epsilon, zeta, eta
---

---
### 2025-12-30 11:00 - exercise-retriever-create - APPROACH
**Proposed approach**:
- Run `tt/rbw-GR.GovernorRetrieverCreate.sh proto` (using same instance name as keeper depot)
- Retriever is simpler than Director (single IAM grant vs multiple)
- Code already fixed by director_create bug fixes (same patterns)
- Verify RBRA file at `../output-buk/current/proto.rbra`

### 2025-12-30 11:06 - exercise-retriever-create - WRAP
**Outcome**: Created retriever-proto successfully, first try. No bugs encountered.
---

---
### 2025-12-30 11:06 - exercise-sa-list - APPROACH
**Proposed approach**:
- Run `tt/rbw-al.ListServiceAccounts.sh` (if exists) or find correct tabtarget
- Expect to see: governor, mason, director-* (8 instances), retriever-proto
- Verify listing format and completeness

### 2025-12-30 11:14 - exercise-sa-list - WRAP
**Outcome**: Fixed coordinator routing (rbga→rbgg) and tabtarget pattern; listed 12 SAs successfully.

**Bugs fixed**:
1. rbk_Coordinator.sh: rbw-al and rbw-aDS routed to wrong CLI (rbga instead of rbgg)
2. tt/rbw-al.ListServiceAccounts.sh: Used old dispatch pattern instead of coordinator launcher
---

---
### 2025-12-30 11:14 - exercise-sa-delete - APPROACH
**Proposed approach**:
- Run `tt/rbw-aDS.DeleteServiceAccount.sh <email>` to delete one of the leftover directors
- Use director-default as the target (least useful of the bunch)
- Verify deletion via sa_list afterward

### 2025-12-30 11:16 - exercise-sa-delete - WRAP
**Outcome**: Fixed tabtarget pattern; deleted director-default successfully. Count 12→11.
---

---
### 2025-12-30 11:16 - exercise-retriever-create-restore - APPROACH
**Proposed approach**:
- Note: The original pace description said "delete retriever" but we deleted director-default instead
- Retriever-proto still exists, no restoration needed
- Skip this pace as unnecessary, or repurpose if user wants

### 2025-12-30 11:18 - exercise-retriever-create-restore - SKIPPED
**Reason**: Deleted director-default instead of retriever; no restore needed.
---

---
### 2025-12-30 11:18 - exercise-trigger-build - APPROACH (preliminary)
**Proposed approach**:
- Find trigger_build tabtarget (likely rbw-fB based on operation table)
- Review what inputs are required (Dockerfile path, image name, etc.)
- Run build against keeper depot
- Verify image appears in Artifact Registry
---

---
### 2025-12-30 - exercise-trigger-build - APPROACH (detailed)
**Proposed approach**:
- Run `tt/rbw-fB.BuildVessel.sh rbev-vessels/rbev-busybox` to build the busybox vessel
- Build uses Google Cloud Build via Director SA credentials
- Process: verify git clean → package tarball → upload to GCS → submit build → poll for completion
- Vessel builds multi-arch image (linux/amd64, linux/arm64, linux/arm/v7)
- On success, image tagged as `rbev-busybox.YYMMDDHHMMSS` in Artifact Registry
- Watch for: permission errors, API propagation issues, or Mason SA problems

### 2025-12-30 13:35 - trigger-build - STITCHER REFACTOR COMPLETE
**Task**: Refactor Cloud Build config from static JSON to dynamically-stitched step scripts

**Completed**:
1. Created 9 step scripts in `Tools/rbw/rbgjb/`:
   - rbgjb01-derive-tag-base.sh through rbgjb09-build-and-push-metadata.sh
   - Each is a pure, lintable bash file with documentation comments
2. Added `zrbf_stitch_build_json()` function in rbf_Foundry.sh:
   - Inline metadata mapping (builder image, entrypoint, step ID per script)
   - Reads scripts, escapes `$` → `$$` for Cloud Build (preserving `${_RBGY_*}` substitutions)
   - Outputs valid JSON matching original structure
3. Updated `zrbf_kindle()` with `ZRBF_RBGJB_STEPS_DIR` and `ZRBF_STITCHED_BUILD_FILE`
4. Updated `zrbf_compose_build_request_json()` to call stitcher
5. Deleted old static `rbgjb_build.json`

**Test result**: Build submitted successfully, first 5 steps executed correctly (stitcher working).
Step 6 (build-and-push) failed - Docker buildx issue unrelated to stitcher refactor.

**Commits**:
- `72f24de` - Add stitcher function to dynamically generate Cloud Build JSON
- `07c3abc` - Fix stitcher $ escaping to preserve Cloud Build substitutions
- `2397405` - Remove old static rbgjb_build.json

**Next**: Debug Docker buildx failure in step 6 (separate issue from stitcher work)
---

---
### 2025-12-30 - exercise-trigger-build - APPROACH (debug buildx)
**Proposed approach**:
- Re-run build to capture the exact Docker buildx error message
- Likely causes: Cloud Builders docker image lacks buildx or multi-arch support
- Possible fixes: (1) use different builder image, (2) add qemu/binfmt step, (3) fall back to single-arch
- Debug per heat protocol: identify error, analyze root cause, propose fix, wait for approval

### 2025-12-30 14:00 - exercise-trigger-build - SESSION PAUSED
**Issue**: Docker buildx multi-arch build failing in Cloud Build step 4 (build-and-push)

**Root cause identified**: Cloud Build steps run in isolated containers. Buildx builder state doesn't persist across steps.

**Attempted fixes** (all failed with exit code 1):
1. Merged buildx-create into build-and-push step (step 5 deleted)
2. Removed --driver docker-container (default driver doesn't support multi-platform)
3. Added --driver-opt network=host

**Likely actual issue**: The docker-container buildx driver spawns a new container to do builds, but this container-in-container approach may not work in Cloud Build's environment due to Docker daemon access restrictions.

**Research found** (see [GKE multi-arch docs](https://cloud.google.com/kubernetes-engine/docs/how-to/build-multi-arch-for-arm)):
- Need to see actual error from Cloud Build logs (currently disabled)
- May need to enable Cloud Logging or set up logs bucket
- Alternative: build each arch separately and create manifest list

**Next steps when resuming**:
1. Enable Cloud Build logging to capture actual error message
2. Consider alternative approaches:
   - Single-arch build first (just amd64) to get something working
   - Separate builds per arch + manifest list creation
   - Use Kaniko for builds instead of docker buildx
---

---
### 2025-12-31 - exercise-trigger-build - APPROACH (resolve buildx)
**Proposed approach**:
1. Enable Cloud Build logging to see actual buildx error details
2. Research Cloud Build multi-arch patterns and Google's recommended approach
3. Implement simplest working solution first (likely single-arch amd64 to unblock)
4. Test and verify image appears in Artifact Registry
5. Create itch for multi-arch support if needed

### 2025-12-31 - exercise-trigger-build - ROOT CAUSE IDENTIFIED
**Issue**: 403 Forbidden when pushing multi-arch image to GAR

**Analysis**:
- Multi-arch build succeeded (all 3 platforms built correctly)
- Docker login in step 3 authenticated host docker daemon
- Buildx docker-container driver runs in isolated container
- Isolated builder container doesn't inherit host's docker credentials
- Push failed: "failed to fetch oauth token ... 403 Forbidden"

**Catch-22 Discovered**:
- docker (default) driver: Has GAR credentials ✓, but NO multi-platform support ✗
- docker-container driver: Supports multi-platform ✓, but NO access to credentials ✗

**Research Conducted**:
- Extensive web research on buildx authentication, BuildKit credential handling
- Investigated GCR vs GAR authentication differences
- Explored config.json mounting, buildkitd.toml, driver options
- Documented findings in RBWMBX memo (lenses/rbw-RBWMBX-BuildxMultiPlatformAuth.adoc)

**Solution Path Identified**:
Created 5 sequential paces in heat:
1. Test with GCR (validates buildx works)
2. Implement OAuth config.json for GAR (proper solution)
3. Evaluate buildkitd.toml (alternative approach)
4. Document decision (capture outcome)
5. Update RBSTB specification (incorporate learnings)

Each pace has clear success/skip criteria to avoid unnecessary work.

**Git State**:
- Code committed but non-functional for multi-platform builds
- Last commit: f021d54 "Fix buildx: use default builder, don't create new one"
- Stitcher refactor complete (rbgjb01-09 scripts working)
- Steps 1-4 execute successfully (auth, QEMU)
- Step 6 fails (default driver doesn't support multi-platform)

**Session Paused**: Ready to begin pace 1 (Test trigger_build with GCR) in next session
---

---
### 2025-12-31 07:15 - revert-gcr-test-changes - APPROACH
**Proposed approach**:
- Revert rbgjb06-build-and-push.sh to restore GAR target:
  - Change IMAGE_URI from `gcr.io/${_RBGY_GAR_PROJECT}/${_RBGY_MONIKER}:...` back to `${_RBGY_GAR_LOCATION}-docker.pkg.dev/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:...`
  - Remove docker-container driver creation (lines 32-33)
  - Restore default builder comment
  - Update file header from "GCR (testing)" to "GAR"
- Revert rbgjb09-build-and-push-metadata.sh to restore GAR target:
  - Change META_URI from `gcr.io/${_RBGY_GAR_PROJECT}/${_RBGY_MONIKER}:...` back to `${_RBGY_GAR_LOCATION}-docker.pkg.dev/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:...`
  - Restore full substitutions list in header
  - Update file header from "GCR (testing)" to original
- Commit reversion with message explaining GCR test complete, returning to GAR for OCI bridge implementation
- GCR test results already documented in RBWMBX memo (commit d07be5c)

### 2025-12-31 07:20 - revert-gcr-test-changes - COMPLETE
**Outcome**: Reverted both scripts to GAR targets, committed as b3b5737.

**Changes**:
- rbgjb06: Restored `${_RBGY_GAR_LOCATION}-docker.pkg.dev/...` format for IMAGE_URI
- rbgjb06: Removed docker-container driver creation, restored default builder comment
- rbgjb09: Restored `${_RBGY_GAR_LOCATION}-docker.pkg.dev/...` format for META_URI
- rbgjb09: Restored full substitutions list in header

**Next**: Ready to implement OCI Layout Bridge Phase 1 (Export).
---

---
### 2025-12-31 07:25 - oci-bridge-phase1-export - APPROACH
**Proposed approach**:
- Read current rbgjb06-build-and-push.sh to understand full structure
- Modify the buildx command:
  - Replace `--push` with `--output type=oci,dest=/workspace/oci-layout`
  - Keep all `--platform`, `--tag`, and `--label` flags intact
  - Preserve all Git metadata labels
- Remove the `.image_uri` output (no push yet, that's Phase 2)
- Rename file from `rbgjb06-build-and-push.sh` to `rbgjb06-build-and-export.sh`
- Update header comment to reflect new purpose: "Build multi-arch OCI layout"
- The default buildx builder should work fine for OCI export (no push, no auth needed)
- Commit changes with clear explanation of OCI bridge Phase 1

### 2025-12-31 07:30 - oci-bridge-phase1-export - COMPLETE
**Outcome**: Created rbgjb06-build-and-export.sh with OCI layout export.

**Changes**:
- Renamed `rbgjb06-build-and-push.sh` → `rbgjb06-build-and-export.sh`
- Replaced `--push` with `--output type=oci,dest=/workspace/oci-layout`
- Removed `.image_uri` output file (Phase 2 will create this)
- Updated header: "Build multi-arch OCI layout"
- Added explanatory comment about OCI Layout Bridge pattern
- Preserved all labels and metadata (moniker, git.commit, git.branch)
- Kept IMAGE_URI for --tag (becomes OCI layout metadata)

**Key insight**: OCI export avoids authentication entirely - no push means no credentials needed. The `/workspace/` directory persists across Cloud Build steps, acting as the bridge to Phase 2 (Skopeo push).

**Next**: Phase 2 - Create rbgjb07-push-with-skopeo.sh to push OCI layout to GAR.
---

---
### 2025-12-31 07:35 - oci-bridge-phase2-push - APPROACH
**Proposed approach**:
- Create new script rbgjb07-push-with-skopeo.sh
- Header: Builder is quay.io/skopeo/stable:latest
- Substitutions needed: _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY, _RBGY_MONIKER
- Script logic:
  1. Read TAG_BASE from .tag_base file
  2. Construct IMAGE_URI (same format as Phase 1)
  3. Get access token from Cloud Build metadata server: `curl -H "Metadata-Flavor: Google" http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token`
  4. Parse token from JSON response
  5. Use Skopeo to copy OCI layout to GAR: `skopeo copy --all --dest-creds="oauth2accesstoken:${AR_TOKEN}" oci:/workspace/oci-layout docker://${IMAGE_URI}`
  6. Write IMAGE_URI to .image_uri for downstream steps (rbgjb08, rbgjb09)
- Add error handling for metadata server and Skopeo operations

### 2025-12-31 07:40 - oci-bridge-phase2-push - COMPLETE
**Outcome**: Created rbgjb07-push-with-skopeo.sh with Skopeo-based GAR push.

**Changes**:
- New script: Tools/rbw/rbgjb/rbgjb07-push-with-skopeo.sh
- Builder: quay.io/skopeo/stable:latest
- Authentication: Fetches access token from Cloud Build metadata server
- Token parsing: Simple grep/sed extraction from JSON (no jq dependency)
- Skopeo command: `skopeo copy --all --dest-creds="oauth2accesstoken:${AR_TOKEN}" oci:/workspace/oci-layout docker://${IMAGE_URI}`
- Output: Writes IMAGE_URI to .image_uri for downstream steps
- Error handling: Validates OCI layout exists, metadata server response, token extraction

**Key design**: Skopeo runs in a container with metadata server access, solving the credential isolation problem. The `--all` flag ensures all platforms from the OCI layout are pushed to the multi-platform manifest.

**Next**: Phase 3 - Adjust SBOM generation (rbgjb08) to read from OCI layout instead of registry.
---

---
### 2025-12-31 07:45 - oci-bridge-phase3-sbom - APPROACH
**Proposed approach**:
- Read current rbgjb08-sbom-and-summary.sh to understand structure
- Change Syft source from registry image to OCI layout
- Replace `"${IMAGE_URI}"` with `oci-dir:/workspace/oci-layout` in both Syft commands
- Keep both output formats (JSON for analysis, table for summary)
- Keep _RBGY_SYFT_REF substitution and docker builder
- Note: The OCI layout is local, so analysis is faster and doesn't require network access
- This analyzes exactly what was built, before any potential registry corruption

### 2025-12-31 07:50 - oci-bridge-phase3-sbom - COMPLETE
**Outcome**: Updated rbgjb08-sbom-and-summary.sh to read from OCI layout.

**Changes**:
- Replaced `"${IMAGE_URI}"` with `oci-dir:/workspace/oci-layout` in both Syft commands
- Added volume mount: `-v /workspace:/workspace` to give Syft container access to OCI layout
- Added validation: `test -d /workspace/oci-layout` before running Syft
- Updated header comment to explain OCI Layout Bridge Phase 3
- Removed IMAGE_URI read (no longer needed)

**Benefits**:
- Faster: No network pull from registry required
- Accurate: Analyzes exactly what was built locally
- Reliable: No dependency on registry availability

**Note**: rbgjb09 (metadata container) remains unchanged - it's a single-platform scratch image that doesn't need the OCI bridge pattern. It uses the standard docker build/push workflow.

**Next**: Update build stitcher (rbf_Foundry.sh) to integrate new step structure.
---

---
### 2025-12-31 07:55 - update-build-stitcher - APPROACH
**Proposed approach**:
- Read zrbf_stitch_build_json() function in Tools/rbw/rbf_Foundry.sh
- Update step 06 metadata:
  - Change filename from rbgjb06-build-and-push.sh to rbgjb06-build-and-export.sh
  - Update step ID from "build-and-push" to "build-and-export"
- Add new step 07 metadata:
  - Filename: rbgjb07-push-with-skopeo.sh
  - Builder: quay.io/skopeo/stable:latest
  - Entrypoint: bash
  - Step ID: "push-with-skopeo"
  - Dependencies: wait for step 06 (build-and-export)
- Verify step 08 and 09 metadata still correct
- Test by running the stitcher function mentally to ensure valid JSON structure

### 2025-12-31 08:00 - update-build-stitcher - COMPLETE
**Outcome**: Updated stitcher with new OCI bridge step structure.

**Changes**:
- Renamed rbgjb07-assemble-metadata.sh → rbgjb10-assemble-metadata.sh (avoid number collision)
- Updated rbf_Foundry.sh step definitions:
  - Step 06: rbgjb06-build-and-export.sh (updated from build-and-push)
  - Step 07: rbgjb07-push-with-skopeo.sh (NEW - uses quay.io/skopeo/stable:latest)
  - Step 08: rbgjb08-sbom-and-summary.sh (unchanged)
  - Step 10: rbgjb10-assemble-metadata.sh (renumbered from 07)
  - Step 09: rbgjb09-build-and-push-metadata.sh (unchanged, runs last)
- Added OCI Layout Bridge comment block explaining the pattern
- Execution order ensures dependencies: 06 → 07 → 08 → 10 → 09

**Key insight**: Step 10 (assemble-metadata) must run before step 9 (build-and-push-metadata) because step 9 needs build_info.json created by step 10.

**Next**: Test complete OCI bridge workflow with actual build.
---

---
### 2025-12-31 08:05 - test-oci-bridge-workflow - APPROACH
**Proposed approach**:
- Run `tt/rbw-fB.BuildVessel.sh rbev-vessels/rbev-busybox` to trigger a build
- Monitor Cloud Build execution to verify all steps complete successfully
- Check that OCI Layout Bridge phases execute in correct order:
  - Step 06: buildx exports to /workspace/oci-layout
  - Step 07: Skopeo pushes from oci-layout to GAR
  - Step 08: Syft analyzes from oci-layout
  - Step 10: Assembles metadata JSON
  - Step 09: Builds and pushes metadata container
- After build completes, verify:
  - Image pushed to GAR with correct tag (rbev-busybox.YYMMDDHHMMSS-img)
  - Multi-platform manifest contains all 3 platforms (amd64, arm64, arm/v7)
  - Metadata container pushed (rbev-busybox.YYMMDDHHMMSS-meta)
- Follow heat protocol for any bugs: stop, explain, wait for approval before fixing
---

---
### 2025-12-31 09:00 - research-docker-container-oci-output - APPROACH
**Proposed approach**:
- Search Docker buildx docs for OCI output destination with docker-container driver
- Search BuildKit issues/discussions for /workspace output behavior
- Look for Cloud Build examples using buildx OCI export
- Update RBWMBX memo with findings
- If confirmed working: recommend adding docker buildx create back to rbgjb06
- If not working: document workaround

### 2025-12-31 09:30 - research-docker-container-oci-output - COMPLETE
**Key finding**: OCI output writes to CLIENT filesystem, not BuildKit container

**Evidence**:
- Docker docs: "allows export of results directly to the client's filesystem"
- BuildKit docs: "The local client will copy the files directly to the client"
- Mechanism: BuildKit transfers results back via gRPC (FileSend/diffcopy)

**Critical requirements discovered**:
1. docker-container driver IS REQUIRED for both OCI export and multi-platform
2. Default docker driver supports neither
3. tar=false needed for directory output (Skopeo expects directory, not tarball)
4. GitHub Issue #1672 (missing oci-layout file) FIXED in buildkit PR #3729

**Google Cloud validation**: Official Dataflow docs show exact pattern we need

**Changes made**:
- RBWMBX memo: Added "OCI Output Path Research" section with architecture diagram
- rbgjb06: Added `docker buildx create --driver docker-container --name rb-builder --use`
- rbgjb06: Changed output to `type=oci,tar=false,dest=/workspace/oci-layout`

**Next**: Test full OCI bridge workflow
---

---
### 2025-12-31 08:58 - exercise-image-delete - WRAP
**Outcome**: Fixed rbf_delete: added moniker param, tag-based deletion (GAR rejects digest-based), Director needs repoAdmin role, updated tabtarget to new launcher form.

**Fixes applied**:
- rbf_Foundry.sh: Added moniker parameter, simplified to tag-based DELETE (GAR returns GOOGLE_MANIFEST_DANGLING_TAG error on digest-based delete)
- rbgg_Governor.sh: Added artifactregistry.repoAdmin grant for Director SA
- rbgp_Payor.sh: Changed Mason from artifactregistry.admin to artifactregistry.writer
- tt/rbw-fD.DeleteImage.sh: Updated to new launcher form (was old bud_dispatch form)
- Manual IAM grant to director-theta for testing

**Tested**: Deleted tag 20251231T155752Z-img successfully
---

---
### 2025-12-31 09:09 - exercise-trigger-build-rebuild - WRAP
**Outcome**: Rebuilt rbev-busybox image after deletion exercise.

**Build**: 1c228d46-0eee-4f83-9978-0aa99feaa3dd
**Tags**: 20251231T170819Z-img, 20251231T170819Z-meta

**Verified**: Image list shows new tags in GAR.
---

---
### 2026-01-01 - exercise-image-retrieve - APPROACH
**Mode**: manual
**Proposed approach**:
- Modernize tabtarget: Update `tt/rbw-r.RetrieveImage.sh` from old mbd.dispatch to new BUD launcher form
- Implement `rbf_retrieve` in `Tools/rbw/rbf_Foundry.sh` following RBSIR spec:
  - Validate input (moniker:tag or moniker@digest)
  - Authenticate using Retriever SA credentials (RBRR_RETRIEVER_RBRA_FILE)
  - Login to container runtime with access token
  - Pull image using docker/podman
  - Display image ID on success
- Test by pulling the rebuilt busybox image (rbev-busybox:20251231T170819Z-img)

### 2026-01-01 05:19 - exercise-image-retrieve - WRAP
**Outcome**: Implemented and tested successfully on first try.

**Changes**:
- Modernized `tt/rbw-r.RetrieveImage.sh` to BUD launcher form
- Implemented `rbf_retrieve()` in `Tools/rbw/rbf_Foundry.sh` (lines 794-867)
  - Parses moniker:tag or moniker@digest format
  - Prefers Retriever credentials, falls back to Director
  - Docker login with OAuth token
  - Docker pull with full image reference
  - Displays local image ID on success

**Test result**:
- Image: `rbev-busybox:20251231T170819Z-img`
- Pulled successfully from GAR
- Local ID: `sha256:38c8281f1a975034bc48f03b4c9376d97275fd726ad306aae7faa4374a50e810`

**Note**: Used Director credentials (Retriever SA not yet installed locally).
---

---
### 2026-01-01 - exercise-payor-refresh - APPROACH
**Mode**: manual
**Proposed approach**:
- Run `tt/rbw-PR.PayorRefresh.sh` to display the manual procedure
- Verify instructions are correct and clear
- Note: This is a manual procedure (displays instructions), not an automated operation
- The actual refresh would use `rbgp_payor_install` with fresh OAuth JSON

### 2026-01-01 06:15 - exercise-payor-refresh - WRAP
**Outcome**: Successfully refreshed OAuth credentials using new secret rotation flow.

**Fixes applied**:
- Updated GCP UI instructions: RESET SECRET → "+ Add secret" flow
- Fixed zrbgm_dm → zrbgm_dmd for 3-arg display (secret deletion text)
- Indented "Browser downloads" as consequence of download step
- Fixed CRM v1 API field: `.state` → `.lifecycleState` in payor_install
- Made "Google hasn't verified" screen conditional (may not appear)
- Cleaned up rbrp.env (one comment per variable)

**Test result**:
- Rotated client secret via GCP Console
- Ran `tt/rbw-PI.PayorInstall.sh` with new JSON
- `~/.rbw/rbro.env` timestamp updated: Dec 28 09:40 → Jan 1 06:01
- `tt/rbw-ld.ListDepots.sh` succeeded: 1 depot (proto) COMPLETE

**Itch added**: payor-install-rbrp-check (validation checklist with guide colors)
---
