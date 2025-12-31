# Heat: Cloud First Light

## Context

Exercise and debug each critical RBSGS (Getting Started) API operation in sequence. Some operations worked before common file refactoring; others have never been tested. This heat walks the full setup path from payor_install through image retrieval.

### Approach

Each pace follows an iterative pattern:
- Human runs tabtarget repeatedly
- Both human and Claude watch for errors and correct function
- Claude may advise on readiness to proceed
- Human makes final determination to advance

All paces are manual mode given this collaborative debugging workflow.

Update Operation Status section upon pace completion.

### Heat-Wide Guidelines

**Bug Discovery Protocol**: When a bug is found, stop and clearly explain: (1) the observed failure, (2) the root cause analysis, (3) the proposed fix. Wait for human approval before implementing. Claude lacks full development context; human determines if fix is architecturally appropriate.

**Debug Technique**: When a tabtarget fails, check the transcript file at the path shown in error output (e.g., `../temp-buk/temp-YYYYMMDD-HHMMSS-PID-XXX/transcript.txt`). Also check response JSON files in the same directory for API error details.

**Spec Updates**: When code repairs are made during a pace, note the spec updates needed and apply them before marking the pace complete. Do not defer spec fixes to a separate pace.

**Resource Cleanup**: Before advancing past a failed operation, ensure any partially-created resources (projects, buckets, service accounts) are cleaned up. Use `depot_destroy` or manual console deletion as appropriate. Track abandoned resources in the steeplechase if cleanup is deferred.

### First Pace Note

Before exercising operations, validate the mapping between tabtarget script names and RBAGS operation names to prevent confusion throughout the heat.

### Keeper Depot

Permanent depot for use throughout remaining paces and beyond.

- **Name**: proto
- **Region**: us-central1
- **Project ID**: rbwg-d-proto-251230080456

## Operation Status

| Bash Function | Tabtarget | Spec | Status |
|---|---|---|---|
| `rbgp_payor_install` | `rbw-PI` | RBSPI | working |
| `rbgp_depot_create` | `rbw-PC` | RBSDC | working |
| `rbgp_depot_list` | `rbw-ld` | RBSDL | working |
| `rbgp_depot_destroy` | `rbw-PD` | RBSDD | working |
| `rbgp_governor_reset` | `rbw-PG` | RBSGR | working |
| `rbgg_director_create` | `rbw-GD` | RBSDI | working |
| `rbgg_retriever_create` | `rbw-GR` | RBSRC | working |
| `rbgg_list_service_accounts` | `rbw-al` | RBSSL | working |
| `rbgg_delete_service_account` | `rbw-aDS` | RBSSD | working |
| `rbf_build` | `rbw-fB` | RBSTB | untested |
| — | `rbw-il` | — | unimplemented |
| `rbf_delete` | `rbw-fD` | RBSID | untested |
| — | `rbw-r` | RBSIR | old dispatcher |
| `rbgm_payor_refresh` | `rbw-PR` | RBSPR | untested |

## Done

- **Validate tabtarget-to-RBAGS mapping** — Verified 14 operations. Found 11 OK, 1 missing (governor_reset), 1 unimplemented (image_list), 1 needs modernization (image_retrieve).

- **Add IAM pre-flight verification to depot_create** — Code added to `rbgp_Payor.sh` (lines 697-721), spec updated in `rbw-RBSDC-depot_create.adoc`. Fix verified working in test run (passed IAM step before unrelated failure).

- **Exercise payor_establish** — Fixed guide display bugs, documented IAM delay, added RBGC_PAYOR_APP_NAME constant.

- **Exercise payor_install** — Created BUG module, fixed token security, updated OAuth flow docs, refactored payor_establish.

- **Exercise depot_create (practice)** — Fixed repo IAM policy (use correct project ID), fixed depot tracking (CRM v1 instead of v3). Practice depot rbwg-d-test-251228174809 created successfully.

- **Exercise depot_list** — Listed 8 depot projects; 3 complete (with Mason SA), 5 broken. Practice depot appears as COMPLETE.

- **Exercise depot_destroy** — Replaced DEBUG_ONLY with RBGP_CONFIRM_DESTROY, fixed CRM v3 `.state` field parsing, deleted 4 test depots.

- **Exercise depot_create (for keeps)** — Added rbgu_poll_get_until_ok helper, bucket IAM retry logic. Keeper depot: rbwg-d-proto-251230080456

- **Exercise governor_reset** — Created tabtarget, exercised on keeper depot, RBRA file produced successfully.

- **Exercise director_create** — Fixed 6 bugs across rbgp/rbgg/rbgi/rbgd; created director-eta successfully.

- **Exercise retriever_create** — Created retriever-proto successfully, first try.

- **Exercise sa_list** — Fixed coordinator routing and tabtarget pattern; listed 12 SAs successfully.

- **Exercise sa_delete** — Fixed tabtarget pattern; deleted director-default successfully.

- **Exercise retriever_create (restore)** — SKIPPED: deleted director-default instead of retriever; no restore needed.

- **Test trigger_build with GCR** — GCR test executed (build ddfb01b9). Multi-platform compilation succeeded (all 3 platforms built), but push failed with same 403 Forbidden error as GAR. Proved: (1) buildx driver config correct, (2) auth problem universal to docker-container isolation, (3) GCP auto-auth only works for host daemon. Updated RBWMBX memo with findings. Conclusion: must provide credentials inside BuildKit container.

- **Revert GCR test changes** — Restored GAR targets in rbgjb06 and rbgjb09. Reverted IMAGE_URI and META_URI to `${_RBGY_GAR_LOCATION}-docker.pkg.dev/...` format. Removed docker-container driver creation. Committed as b3b5737.

- **Implement OCI Layout Bridge (Phase 1: Export)** — Created rbgjb06-build-and-export.sh. Replaced `--push` with `--output type=oci,dest=/workspace/oci-layout`. Removed `.image_uri` output. Preserved all labels and metadata. OCI export avoids authentication (no push = no credentials needed). Committed as 3863307.

- **Implement OCI Layout Bridge (Phase 2: Push)** — Created rbgjb07-push-with-skopeo.sh. Uses quay.io/skopeo/stable:latest builder. Fetches GAR access token from Cloud Build metadata server. Pushes OCI layout to GAR with `skopeo copy --all --dest-creds`. Writes IMAGE_URI to .image_uri. Solves credential isolation problem. Committed as 97e392d.

- **Adjust SBOM generation for OCI layout** — Updated rbgjb08-sbom-and-summary.sh to read from OCI layout instead of registry. Changed Syft source to `oci-dir:/workspace/oci-layout`. Added volume mount for Syft access. Faster (no network), more accurate (analyzes local build). Committed as 6484d51.

- **Update build stitcher for new steps** — Updated zrbf_stitch_build_json() in rbf_Foundry.sh. Renamed rbgjb07-assemble-metadata.sh → rbgjb10-assemble-metadata.sh. Added rbgjb07-push-with-skopeo.sh with quay.io/skopeo builder. Updated step 06 reference to rbgjb06-build-and-export.sh. Execution order: 06→07→08→10→09. Committed as 5377690.

## Current

- **Test complete OCI bridge workflow** — Run full build with busybox vessel. Verify: (1) multi-platform OCI layout created, (2) Skopeo push succeeds to GAR, (3) all 3 platforms present in manifest, (4) SBOM generated correctly, (5) metadata container pushed. Check image pullable from GAR with correct platforms.
  mode: manual

- **Update RBSTB specification** — Document OCI Layout Bridge in rbw-RBSTB-trigger_build.adoc. Include: (1) why direct push fails (BuildKit isolation), (2) OCI layout bridge pattern (build → /workspace → push), (3) Skopeo authentication via metadata server, (4) multi-platform manifest handling, (5) step-by-step Cloud Build structure, (6) reference RBWMBX memo for decision history and alternatives.
  mode: manual

- **Implement image_list** — Add basic image listing operation (noted missing in RBSGS). Scope and implement as `rbw-il.ImageList.sh`.
  mode: manual

- **Exercise image_delete** — Remove built image from repository.
  mode: manual

- **Exercise trigger_build (rebuild)** — Rebuild image for ongoing use after deletion exercise.
  mode: manual

- **Exercise image_retrieve** — Pull image from repository to local workstation. Note: `rbw-r.RetrieveImage.sh` uses old mbd.dispatch; must modernize to BUD bash-style dispatch first.
  mode: manual

- **Audit dead code in rbga/rbgb/rbgp modules** — Determine if rbga_*, rbgb_*, and zrbgp_billing_* functions are dead code. Remove or document why retained.
  mode: manual

- **Exercise payor_refresh** — Obtain fresh OAuth credentials. Validates recovery path.
  mode: manual

## Steeplechase

---
### 2025-12-31 05:55 - GCR Test - EXECUTION

**Action**: Executed GCR test to validate if multi-platform buildx works with gcr.io instead of Artifact Registry.

**Changes**:
- Modified rbgjb06 and rbgjb09 to use `gcr.io/${PROJECT_ID}` format
- Re-enabled docker-container driver creation
- Committed as 10d1e2a

**Result**: Build ddfb01b9-72fb-49bb-a996-9955285a6e22
- ✓ Multi-platform compilation succeeded (amd64, arm64, arm/v7)
- ✗ Push failed with 403 Forbidden (same as GAR)

**Conclusion**: Auth problem is universal to docker-container isolation, not GAR-specific. GCP auto-auth only works for host daemon, not isolated BuildKit containers.

---
### 2025-12-31 06:30 - Authentication Research - APPROACH

**Action**: Comprehensive web search and analysis of authentication solutions for multi-platform builds in Cloud Build.

**Key findings**:
1. **Kaniko is dead** - Archived Jan 2025, no longer maintained
2. **DOCKER_CONFIG broken** - Issue #5477: breaks buildx command parsing
3. **Issue #1205 documents our exact problem** - GCP OAuth timeout with buildx
4. **No driver-opt for auth** - Docker-container driver has no credential options
5. **BuildKit registry auth ongoing issue** - Multiple open issues, no solutions

**Critical discovery**: Skopeo + OCI Layout Bridge
- Google's own blog endorses Skopeo for Cloud Build multi-platform
- BuildKit can export to OCI layout (no push, no auth needed)
- Skopeo can copy from OCI layout to registry (with GCP auth)
- Separates build phase from push phase using /workspace bridge

---
### 2025-12-31 07:00 - Documentation Updates - COMPLETE

**Action**: Updated RBWMBX memo and heat paces with OCI Layout Bridge solution.

**RBWMBX changes** (commits d07be5c, bdac6d9):
- Added Option 6: OCI Layout Bridge (recommended solution)
- Documented why Options 1 and 3 eliminated
- Added comprehensive Skopeo/OCI references
- Updated Next Steps with implementation plan

**Heat pace changes** (commit 283cd69):
- Removed outdated DOCKER_CONFIG/buildkitd.toml attempts
- Added 6 focused OCI bridge implementation paces
- Clear path: revert → implement export → implement push → test → document

**Next session**: Begin with "Revert GCR test changes" pace, then implement OCI Layout Bridge.

---
