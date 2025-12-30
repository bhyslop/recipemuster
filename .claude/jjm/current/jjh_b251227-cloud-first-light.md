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

## Current

- **Exercise trigger_build** — Submit container build to Cloud Build. Mason executes, publishes image to repository.
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

(execution log begins here)
