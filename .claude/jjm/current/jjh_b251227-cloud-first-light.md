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
| `rbf_build` | `rbw-fB` | RBSTB | working |
| `rbf_list` | `rbw-il` | — | working |
| `rbf_delete` | `rbw-fD` | RBSID | working |
| `rbf_retrieve` | `rbw-r` | RBSIR | working |
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

- **Research: docker-container driver + OCI output** — Confirmed OCI output writes to CLIENT filesystem (Cloud Build step), not BuildKit container. BuildKit transfers via gRPC. Docker-container driver IS REQUIRED for both OCI export and multi-platform (default driver supports neither). Added driver creation and tar=false to rbgjb06. Updated RBWMBX memo with architecture diagram.

- **Test complete OCI bridge workflow** — Build 3b544930 succeeded. Fixed: Alpine+jq for shell (distroless jq had no shell), corrected vessel path format (rbev-vessels/rbev-busybox). All 9 steps pass: build-and-export creates OCI tarball, Skopeo pushes to GAR, Syft generates SBOM via docker socket, Alpine+jq assembles metadata. OCI Layout Bridge pattern fully operational.

- **Implement image_list** — Created rbf_list() in rbf_Foundry.sh. Two modes: (1) no args lists all images using GAR REST API packages endpoint, (2) with moniker lists tags using Docker Registry v2 API. Added rbw-il routing and tabtarget. Uses Director credentials (Retriever SA not yet provisioned). Tested: shows 1 image (rbev-busybox) and 6 tags.

- **Exercise image_delete** — Fixed rbf_delete: added moniker param, tag-based deletion (GAR rejects digest-based), Director needs repoAdmin role, updated tabtarget to new launcher form.

- **Exercise trigger_build (rebuild)** — Rebuilt rbev-busybox image (build 1c228d46, tags 20251231T170819Z-img/meta).

- **Exercise image_retrieve** — Modernized tabtarget to BUD launcher, implemented rbf_retrieve in rbf_Foundry.sh. Uses Retriever credentials when available, falls back to Director. Tested: pulled rbev-busybox:20251231T170819Z-img successfully.

- **Choose name for design decision documentation** — Selected "Trade Study" as section name. Captures: intense research → coherent synthesis, constraint/tradeoff analysis, no discovery narrative, defensible conclusions. Formal engineering term with aerospace heritage.

- **Add Trade Study section to RBAGS** — Created "Trade Studies" section in rbw-RBAGS-AdminGoogleSpec.adoc with OCI Layout Bridge as first entry. Documents: problem (BuildKit credential isolation), constraints (driver catch-22, eliminated options), alternatives evaluated (5 options with dispositions), chosen solution (OCI Layout Bridge), rationale (5 points), implementation details (build/push steps, critical notes), references (RBWMBX memo, build ID).

## Current

- **Audit dead code in rbga/rbgb/rbgp modules** — Determine if rbga_*, rbgb_*, and zrbgp_billing_* functions are dead code. Remove or document why retained.
  mode: manual

- **Exercise payor_refresh** — Obtain fresh OAuth credentials. Validates recovery path.
  mode: manual

- **Add GAR repository name validation** — Build failed silently because RBRR_GAR_REPOSITORY (brm-gar) didn't match actual depot repository (rbw-proto-repository). Root cause: RBRR_GAR_REPOSITORY is static manual config, but repository name is determined at depot_create time. Options: (1) Add runtime validation in rbf_build to verify repository exists before build, (2) Change depot_create to write RBRR_GAR_REPOSITORY, (3) Derive repository name from depot project ID. Evaluate which approach prevents future desync.
  mode: manual

- **Update RBSTB specification** — Document OCI Layout Bridge in rbw-RBSTB-trigger_build.adoc. Include: (1) why direct push fails (BuildKit isolation), (2) OCI layout bridge pattern (build → /workspace → push), (3) Skopeo authentication via metadata server, (4) multi-platform manifest handling, (5) step-by-step Cloud Build structure, (6) reference RBWMBX memo for decision history and alternatives.
  mode: manual

- **Audit tabtargets for log/no-log correctness** — Review all tt/*.sh files for correct BUD_NO_LOG usage. Tabtargets that handle secrets should disable logging with comment: `export BUD_NO_LOG=1  # Disabled: prevents secrets in log files`. Tabtargets worth logging should omit BUD_NO_LOG entirely. Ensure old-form tabtargets (using bud_dispatch.sh directly) are updated to new launcher form.
  mode: manual

- **Audit director/repository configuration process** — Verify setup scripts correctly configure: (1) Director SA gets artifactregistry.repoAdmin on GAR repo during director_create, (2) Mason SA gets artifactregistry.writer (not admin) during depot_create, (3) RBRR_GAR_REPOSITORY value derivation/validation, (4) Director RBRA file installation instructions are clear. Ensure no manual IAM grants needed between operations.
  mode: manual

- **Exercise full depot lifecycle (test depot)** — Create throwaway test depot and run complete flow: depot_create → governor_reset → director_create → trigger_build → image_list → image_delete → depot_destroy. Verify all operations succeed without manual IAM interventions. Validates fixes to Mason/Director permissions and repository configuration.
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
### 2025-12-31 08:13 - OCI Bridge Workflow SUCCESS

**Build ID**: 3b544930-8880-4a04-bf41-94dc1afc31fb

**Final fixes applied**:
1. jq distroless → Alpine+jq: `ghcr.io/jqlang/jq:latest` is distroless with no shell. Changed to `alpine:latest` with `apk add --no-cache jq` at script start.
2. Vessel path format: Command expects directory path (`rbev-vessels/rbev-busybox`), not just vessel name.

**Verified working**:
- Step 5: docker-container driver creates OCI tarball at /workspace/oci-layout.tar
- Step 6: Skopeo pushes with `--dest-registry-token` to GAR
- Step 7: Syft analyzes via docker socket after pulling image
- Step 8: Alpine installs jq, assembles metadata JSON
- Step 9: Metadata container built and pushed

**OCI Layout Bridge pattern complete**: Build → OCI tarball → Skopeo push → Downstream analysis. Solves BuildKit credential isolation problem.

---
