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

### First Pace Note

Before exercising operations, validate the mapping between tabtarget script names and RBAGS operation names to prevent confusion throughout the heat.

## Operation Status

- payor_install: untested
- depot_create: untested
- depot_list: untested
- depot_destroy: untested
- governor_reset: untested
- director_create: untested
- retriever_create: untested
- sa_list: untested
- sa_delete: untested
- trigger_build: untested
- image_list: unimplemented
- image_delete: untested
- image_retrieve: untested
- payor_refresh: untested

## Done

(none yet)

## Current

(waiting to start)

## Remaining

1. **Validate tabtarget-to-RBAGS mapping** — Review all tabtarget scripts that will be exercised in this heat. Confirm each has clear association to its RBAGS operation name. Document any naming inconsistencies.
   mode: manual

2. **Exercise payor_install** — Complete OAuth credential installation. Browser authorization flow, store refresh token to ~/.rbw/rbro.env.
   mode: manual

3. **Exercise depot_create (practice)** — Provision depot infrastructure: project, bucket, repository, Mason service account. This is a practice run.
   mode: manual

4. **Exercise depot_list** — Verify practice depot appears in listing.
   mode: manual

5. **Exercise depot_destroy** — Remove practice depot and all its resources.
   mode: manual

6. **Exercise depot_create (for keeps)** — Provision depot infrastructure for ongoing use.
   mode: manual

7. **Exercise governor_reset** — Create Governor service account within the depot. Produces RBRA file at RBRR_GOVERNOR_RBRA_FILE path.
   mode: manual

8. **Exercise director_create** — Provision Director service account. Produces RBRA file at RBRR_DIRECTOR_RBRA_FILE path.
   mode: manual

9. **Exercise retriever_create** — Provision Retriever service account. Produces RBRA file at RBRR_RETRIEVER_RBRA_FILE path.
   mode: manual

10. **Exercise sa_list** — Verify all created service accounts appear in roster.
    mode: manual

11. **Exercise sa_delete** — Delete one service account (retriever) to exercise deletion path.
    mode: manual

12. **Exercise retriever_create (restore)** — Recreate retriever after deletion exercise.
    mode: manual

13. **Exercise trigger_build** — Submit container build to Cloud Build. Mason executes, publishes image to repository.
    mode: manual

14. **Implement image_list** — Add basic image listing operation (noted missing in RBSGS). Scope and implement.
    mode: manual

15. **Exercise image_delete** — Remove built image from repository.
    mode: manual

16. **Exercise trigger_build (rebuild)** — Rebuild image for ongoing use after deletion exercise.
    mode: manual

17. **Exercise image_retrieve** — Pull image from repository to local workstation.
    mode: manual

18. **Exercise payor_refresh** — Obtain fresh OAuth credentials. Validates recovery path.
    mode: manual

## Steeplechase

(execution log begins here)
