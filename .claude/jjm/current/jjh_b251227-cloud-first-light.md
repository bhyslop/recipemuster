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

## Operation Status

| Bash Function | Tabtarget | Spec | Status |
|---|---|---|---|
| `rbgp_payor_install` | `rbw-PI` | RBSPI | working |
| `rbgp_depot_create` | `rbw-PC` | RBSDC | untested |
| `rbgp_depot_list` | `rbw-ld` | RBSDL | untested |
| `rbgp_depot_destroy` | `rbw-PD` | RBSDD | untested |
| `rbgp_governor_reset` | `rbw-PG` | RBSGR | missing |
| `rbgg_director_create` | `rbw-GD` | RBSDI | untested |
| `rbgg_retriever_create` | `rbw-GR` | RBSRC | untested |
| `rbgg_list_service_accounts` | `rbw-al` | RBSSL | untested |
| `rbgg_delete_service_account` | `rbw-aDS` | RBSSD | untested |
| `rbf_build` | `rbw-fB` | RBSTB | untested |
| — | `rbw-il` | — | unimplemented |
| `rbf_delete` | `rbw-fD` | RBSID | untested |
| — | `rbw-r` | RBSIR | old dispatcher |
| `rbgm_payor_refresh` | `rbw-PR` | RBSPR | untested |

## Done

- **Validate tabtarget-to-RBAGS mapping** — Verified 14 operations. Found 11 OK, 1 missing (governor_reset), 1 unimplemented (image_list), 1 needs modernization (image_retrieve).

- **Add IAM pre-flight verification to depot_create** — Code added to `rbgp_Payor.sh` (lines 697-721), spec updated in `rbw-RBSDC-depot_create.adoc`. Fix verified working in test run (passed IAM step before unrelated failure).

- **Exercise payor_establish** — Fixed guide display bugs, documented IAM delay, added RBGC_PAYOR_APP_NAME constant.

## Current

- **Exercise payor_install** — Run `tt/rbw-PI.PayorInstall.sh <oauth-json-file>` to complete OAuth authorization flow and store credentials.
  mode: manual

## Remaining

- **Exercise depot_create (practice)** — Provision depot infrastructure: project, bucket, repository, Mason service account. This is a practice run.
  mode: manual

- **Repair: Remove incorrect export RBRP_ directives** — The `zrbgp_depot_list_update()` function will detect the incorrect `export RBRP_DEPOT_PROJECT_IDS` directives, print the exact sed command needed to fix them, and exit with Unix error. Apply the suggested sed command, commit the fix, then re-run depot_create to verify the repair.
  mode: manual

- **Exercise depot_list** — Verify practice depot appears in listing.
  mode: manual

- **Exercise depot_destroy** — Remove practice depot and all its resources.
  mode: manual

- **Exercise depot_create (for keeps)** — Provision depot infrastructure for ongoing use.
  mode: manual

- **Exercise governor_reset** — Create Governor service account within the depot. Produces RBRA file at RBRR_GOVERNOR_RBRA_FILE path. Note: tabtarget `rbw-PG.PayorGovernorReset.sh` must be created first.
  mode: manual

- **Exercise director_create** — Provision Director service account. Produces RBRA file at RBRR_DIRECTOR_RBRA_FILE path.
  mode: manual

- **Exercise retriever_create** — Provision Retriever service account. Produces RBRA file at RBRR_RETRIEVER_RBRA_FILE path.
  mode: manual

- **Exercise sa_list** — Verify all created service accounts appear in roster.
  mode: manual

- **Exercise sa_delete** — Delete one service account (retriever) to exercise deletion path.
  mode: manual

- **Exercise retriever_create (restore)** — Recreate retriever after deletion exercise.
  mode: manual

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

- **Exercise payor_refresh** — Obtain fresh OAuth credentials. Validates recovery path.
  mode: manual

## Steeplechase

(execution log begins here)
