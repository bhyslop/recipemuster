# Heat Trophy: rbk-mvp-2-iam-grant-unification

**Firemark:** ₣As
**Created:** 260309
**Retired:** 260313
**Status:** retired

## Paddock

# Paddock: rbk-mvp-2-iam-grant-unification

## Context

Correct and improve the four IAM grant variants in rbgi_IAM.sh based on verified API contract research (RBSCIG-IamGrantContracts.adoc). The original goal of collapsing to a single superset contract was abandoned after research revealed that the per-API behavioral differences are load-bearing, not arbitrary.

## Prior Art

MVP-1 (₣Ak, retired) established the spec definition aesthetic: every line in a rbtoe_ definition should be load-bearing.

## Key Design Decisions (Settled)

- **No superset contract**: Research (RBSCIG, 2026-03-11) proved that vanilla-init, read-back verify, and SA preflight are not safe to universalize. Each API has genuinely different error semantics. Keep 4 separate rbtoe_iam_grant_* definitions.
- **Etag everywhere**: All 5 APIs always return etag. Extracting and passing it intentionally converts accidental passthrough/unconditional overwrites to concurrency-safe writes. Safe convergence.
- **Transient retry everywhere**: Adding 429/5xx retry to repo/SA/bucket SET operations is universally correct. Safe convergence.
- **Fix false error handlers**: Repo 404 handler and SA non-200 vanilla-init mask real errors (API never returns these for existing resources). Remove them.
- **409 not 412**: CRM/IAM/AR/SecretManager return 409 ABORTED on etag mismatch. Only Storage returns 412. Fix project code's 412 handler.
- **BCG-compliant Option B**: Extracted step helpers called by per-resource outer functions owning retry loops. Mechanical DRY without behavioral unification.
- **Two failure classes**: member-SA propagation (retry inside grant) vs resource propagation (rbgu_poll_until_ok before grant). Do not conflate.

## Research Findings Summary

See RBSCIG-IamGrantContracts.adoc for full evidence (17 sourced claims). Key findings:
- Every API always returns etag; omitting it allows unconditional overwrite that can destroy IAM Conditions
- No API returns 404 for "no IAM policy" — all resources are born with default policies
- Etag mismatch: 409 ABORTED (CRM, IAM, AR, SecretManager) vs 412 Precondition Failed (Storage only)
- AR getIamPolicy proto specifies GET, not POST (current code uses POST via transcoding tolerance)
- Propagation error strings ("does not exist") are empirically stable but not contractually guaranteed
- Google-internal concurrent writers can modify project IAM outside single-writer invariant

## References

- Tools/rbk/rbgi_IAM.sh — the four grant functions
- Tools/rbk/rbgp_Payor.sh — majority of call sites + 3x inline Secret Manager grants
- Tools/rbk/rbgg_Governor.sh, Tools/rbk/rbga_ArtifactRegistry.sh, Tools/rbk/rbgb_Buckets.sh — remaining call sites
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — 4 rbtoe_iam_grant_* definitions (corrected in ₢AsAAA)
- Tools/rbk/vov_veiled/RBSCIG-IamGrantContracts.adoc — research memo (created in ₢AsAAA)
- Tools/rbk/vov_veiled/RBSCIP-IamPropagation.adoc — propagation retry backoff profile

## Paces

### pre-refactor-baseline-sweep (₢AsAAB) [complete]

**[260310-2024] complete**

Repeat full test sweep at the start of mvp-2 to confirm green baseline still holds before touching IAM grant code. Must match mvp-1 exit results.

**[260309-1945] rough**

Repeat full test sweep at the start of mvp-2 to confirm green baseline still holds before touching IAM grant code. Must match mvp-1 exit results.

### iam-grant-homogenization (₢AsAAA) [complete]

**[260311-1052] complete**

Drafted from ¢AkAAu in ₣Ak.

## Goal

Unify the IAM grant implementation in rbgi_IAM.sh and its specification in RBS0 to reduce cognitive load from patchwork evolution. Collapse 4 rbtoe_iam_grant_* linked terms to 1 rbtoe_iam_grant with a uniform superset behavioral contract.

## Current State: 4 Variants With Arbitrary-Feeling Differences

| Dimension | project | repo | sa | bucket |
|---|---|---|---|---|
| API | CRM v1 | Artifact Registry v1 | IAM v1 | Storage JSON v1 |
| GET method | POST + version body | POST + empty body | POST + empty body | GET (no body) |
| SET method | POST | POST | POST | PUT |
| Empty policy | errors | 404 -> vanilla init | non-200 -> vanilla init | non-200 -> vanilla init |
| Etag concurrency | yes | no | no | yes (when present) |
| Inner transient retry | yes (429/5xx) | no | no | no |
| Read-back verify | yes | no | no | no |
| Pre-flight check | no | no | SA existence | no |

All 4 share: propagation retry on HTTP 400 "does not exist" per RBSCIP backoff profile (3s initial, 2x, 20s cap, 420s deadline). Call sites: ~30 across rbgp_Payor.sh (majority), rbgg_Governor.sh, rbga_ArtifactRegistry.sh, rbgb_Buckets.sh.

## Two Failure Classes (Do Not Conflate)

Member-SA propagation: A newly-created SA is visible via GET but not yet usable as a member in setIamPolicy (returns 400 "does not exist"). You cannot pre-poll for this — the only test is attempting the policy operation. This is why retry lives INSIDE the grant function. All 4 rbgi_add_* handle this.

Resource propagation: A newly-created resource (secret, SA) is not yet visible at all. Handled BEFORE the grant by rbgu_poll_until_ok (rbtoe_poll_readiness). The Secret Manager inline grants in rbgp_Payor.sh rely on this pre-poll and therefore need no propagation retry of their own.

rbtoe_poll_readiness is independent from the IAM grant pattern and should not be folded into this work.

## Unified Superset Contract

The refactored rbtoe_iam_grant guarantees:
- Read-modify-write via resource-type-appropriate API (GET method, SET method, URL construction vary)
- Etag optimistic concurrency (extract and send when API provides it)
- Propagation retry on 400 "does not exist" per RBSCIP backoff profile
- Transient retry on 429/5xx with bounded wait
- Read-back verification confirming the granted role is observable
- Vanilla policy initialization when resource has no existing policy
- Pre-flight SA existence check when member is a service account

Every grant gets every guarantee. Harmless where not strictly needed, protective everywhere.

## BCG Compliance Finding

A design using bash dynamic scoping (local -r ZRBGI_* in outer wrappers visible to inner function) was rejected as non-BCG. Two BCG-compliant approaches identified:

Option A: One inner function with 10+ explicit positional parameters. Verbose but honest.

Option B (favored): Extracted step helpers (zrbgi_read_policy, zrbgi_compose_binding, zrbgi_write_policy, zrbgi_verify_binding) called by per-resource outer functions that own their retry loops. The flow is visible in each function; the HTTP/jq mechanics are shared. More BCG-idiomatic.

Final selection deferred to this pace.

## Spec Definition Aesthetic

From AkAAa we established: every line in a rbtoe_ definition should be load-bearing. No boilerplate sections. Content complexity tracks behavioral complexity. Closing context lines (like RBSCIP backoff references) appear only when they earn their place. The unified rbtoe_iam_grant definition should follow api_enable as baseline inspiration, with richer contract sentence reflecting the superset guarantees.

## Implementation Plan

1. Refactor rbgi_IAM.sh: extract shared helpers, rewrite 4 functions as thin wrappers calling helpers
2. Add rbgi_grant_secret_iam wrapper for Secret Manager (eliminates 3x copy-paste in rbgp_Payor.sh)
3. Update all ~30 call sites if function signatures change
4. Collapse 4 rbtoe_iam_grant_* mapping entries and definitions to 1 rbtoe_iam_grant in RBS0
5. Update 4 subdocuments (RBSDC, RBSDI, RBSGR, RBSRC) to use unified term
6. Consider whether RBSCIP prose benefits from referencing the unified term

## Files

- Tools/rbk/rbgi_IAM.sh: refactor 4 functions into shared helpers plus thin wrappers
- Tools/rbk/rbgp_Payor.sh: extract 3x inline secret IAM copy-paste, update call sites
- Tools/rbk/rbgg_Governor.sh: update call sites
- Tools/rbk/rbga_ArtifactRegistry.sh: update call sites
- Tools/rbk/rbgb_Buckets.sh: update call sites
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc: collapse 4 rbtoe_iam_grant_* to 1 rbtoe_iam_grant
- Tools/rbk/vov_veiled/RBSDC-depot_create.adoc, RBSDI-director_create.adoc, RBSGR-governor_reset.adoc, RBSRC-retriever_create.adoc: update term references
- Tools/rbk/vov_veiled/RBSCIP-IamPropagation.adoc: may benefit from referencing unified term

## Not In Scope

- Complete-policy-write patterns in rbgg_Governor.sh (multi-binding atomic writes, genuinely different operation)
- rbtoe_poll_readiness (independent concern, different failure class)
- Changing runtime behavior: this is a refactor for clarity not functionality
- BCG-BashConsoleGuide.md changes

**[260310-2030] bridled**

Drafted from ¢AkAAu in ₣Ak.

## Goal

Unify the IAM grant implementation in rbgi_IAM.sh and its specification in RBS0 to reduce cognitive load from patchwork evolution. Collapse 4 rbtoe_iam_grant_* linked terms to 1 rbtoe_iam_grant with a uniform superset behavioral contract.

## Current State: 4 Variants With Arbitrary-Feeling Differences

| Dimension | project | repo | sa | bucket |
|---|---|---|---|---|
| API | CRM v1 | Artifact Registry v1 | IAM v1 | Storage JSON v1 |
| GET method | POST + version body | POST + empty body | POST + empty body | GET (no body) |
| SET method | POST | POST | POST | PUT |
| Empty policy | errors | 404 -> vanilla init | non-200 -> vanilla init | non-200 -> vanilla init |
| Etag concurrency | yes | no | no | yes (when present) |
| Inner transient retry | yes (429/5xx) | no | no | no |
| Read-back verify | yes | no | no | no |
| Pre-flight check | no | no | SA existence | no |

All 4 share: propagation retry on HTTP 400 "does not exist" per RBSCIP backoff profile (3s initial, 2x, 20s cap, 420s deadline). Call sites: ~30 across rbgp_Payor.sh (majority), rbgg_Governor.sh, rbga_ArtifactRegistry.sh, rbgb_Buckets.sh.

## Two Failure Classes (Do Not Conflate)

Member-SA propagation: A newly-created SA is visible via GET but not yet usable as a member in setIamPolicy (returns 400 "does not exist"). You cannot pre-poll for this — the only test is attempting the policy operation. This is why retry lives INSIDE the grant function. All 4 rbgi_add_* handle this.

Resource propagation: A newly-created resource (secret, SA) is not yet visible at all. Handled BEFORE the grant by rbgu_poll_until_ok (rbtoe_poll_readiness). The Secret Manager inline grants in rbgp_Payor.sh rely on this pre-poll and therefore need no propagation retry of their own.

rbtoe_poll_readiness is independent from the IAM grant pattern and should not be folded into this work.

## Unified Superset Contract

The refactored rbtoe_iam_grant guarantees:
- Read-modify-write via resource-type-appropriate API (GET method, SET method, URL construction vary)
- Etag optimistic concurrency (extract and send when API provides it)
- Propagation retry on 400 "does not exist" per RBSCIP backoff profile
- Transient retry on 429/5xx with bounded wait
- Read-back verification confirming the granted role is observable
- Vanilla policy initialization when resource has no existing policy
- Pre-flight SA existence check when member is a service account

Every grant gets every guarantee. Harmless where not strictly needed, protective everywhere.

## BCG Compliance Finding

A design using bash dynamic scoping (local -r ZRBGI_* in outer wrappers visible to inner function) was rejected as non-BCG. Two BCG-compliant approaches identified:

Option A: One inner function with 10+ explicit positional parameters. Verbose but honest.

Option B (favored): Extracted step helpers (zrbgi_read_policy, zrbgi_compose_binding, zrbgi_write_policy, zrbgi_verify_binding) called by per-resource outer functions that own their retry loops. The flow is visible in each function; the HTTP/jq mechanics are shared. More BCG-idiomatic.

Final selection deferred to this pace.

## Spec Definition Aesthetic

From AkAAa we established: every line in a rbtoe_ definition should be load-bearing. No boilerplate sections. Content complexity tracks behavioral complexity. Closing context lines (like RBSCIP backoff references) appear only when they earn their place. The unified rbtoe_iam_grant definition should follow api_enable as baseline inspiration, with richer contract sentence reflecting the superset guarantees.

## Implementation Plan

1. Refactor rbgi_IAM.sh: extract shared helpers, rewrite 4 functions as thin wrappers calling helpers
2. Add rbgi_grant_secret_iam wrapper for Secret Manager (eliminates 3x copy-paste in rbgp_Payor.sh)
3. Update all ~30 call sites if function signatures change
4. Collapse 4 rbtoe_iam_grant_* mapping entries and definitions to 1 rbtoe_iam_grant in RBS0
5. Update 4 subdocuments (RBSDC, RBSDI, RBSGR, RBSRC) to use unified term
6. Consider whether RBSCIP prose benefits from referencing the unified term

## Files

- Tools/rbk/rbgi_IAM.sh: refactor 4 functions into shared helpers plus thin wrappers
- Tools/rbk/rbgp_Payor.sh: extract 3x inline secret IAM copy-paste, update call sites
- Tools/rbk/rbgg_Governor.sh: update call sites
- Tools/rbk/rbga_ArtifactRegistry.sh: update call sites
- Tools/rbk/rbgb_Buckets.sh: update call sites
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc: collapse 4 rbtoe_iam_grant_* to 1 rbtoe_iam_grant
- Tools/rbk/vov_veiled/RBSDC-depot_create.adoc, RBSDI-director_create.adoc, RBSGR-governor_reset.adoc, RBSRC-retriever_create.adoc: update term references
- Tools/rbk/vov_veiled/RBSCIP-IamPropagation.adoc: may benefit from referencing unified term

## Not In Scope

- Complete-policy-write patterns in rbgg_Governor.sh (multi-binding atomic writes, genuinely different operation)
- rbtoe_poll_readiness (independent concern, different failure class)
- Changing runtime behavior: this is a refactor for clarity not functionality
- BCG-BashConsoleGuide.md changes

*Direction:* Settled design: BCG-compliant Option B with extracted step helpers (zrbgi_read_policy, zrbgi_compose_binding, zrbgi_write_policy, zrbgi_verify_binding) called by per-resource outer functions owning retry loops. Superset contract: every grant gets etag, propagation retry, transient retry, read-back verify, vanilla policy init, SA preflight. Add rbgi_grant_secret_iam for Secret Manager. Collapse 4 rbtoe_iam_grant_* to 1 rbtoe_iam_grant in spec. ~30 call sites across 4 files. Runtime behavior preserved — refactor only.

**[260310-1935] rough**

Drafted from ¢AkAAu in ₣Ak.

## Goal

Unify the IAM grant implementation in rbgi_IAM.sh and its specification in RBS0 to reduce cognitive load from patchwork evolution. Collapse 4 rbtoe_iam_grant_* linked terms to 1 rbtoe_iam_grant with a uniform superset behavioral contract.

## Current State: 4 Variants With Arbitrary-Feeling Differences

| Dimension | project | repo | sa | bucket |
|---|---|---|---|---|
| API | CRM v1 | Artifact Registry v1 | IAM v1 | Storage JSON v1 |
| GET method | POST + version body | POST + empty body | POST + empty body | GET (no body) |
| SET method | POST | POST | POST | PUT |
| Empty policy | errors | 404 -> vanilla init | non-200 -> vanilla init | non-200 -> vanilla init |
| Etag concurrency | yes | no | no | yes (when present) |
| Inner transient retry | yes (429/5xx) | no | no | no |
| Read-back verify | yes | no | no | no |
| Pre-flight check | no | no | SA existence | no |

All 4 share: propagation retry on HTTP 400 "does not exist" per RBSCIP backoff profile (3s initial, 2x, 20s cap, 420s deadline). Call sites: ~30 across rbgp_Payor.sh (majority), rbgg_Governor.sh, rbga_ArtifactRegistry.sh, rbgb_Buckets.sh.

## Two Failure Classes (Do Not Conflate)

Member-SA propagation: A newly-created SA is visible via GET but not yet usable as a member in setIamPolicy (returns 400 "does not exist"). You cannot pre-poll for this — the only test is attempting the policy operation. This is why retry lives INSIDE the grant function. All 4 rbgi_add_* handle this.

Resource propagation: A newly-created resource (secret, SA) is not yet visible at all. Handled BEFORE the grant by rbgu_poll_until_ok (rbtoe_poll_readiness). The Secret Manager inline grants in rbgp_Payor.sh rely on this pre-poll and therefore need no propagation retry of their own.

rbtoe_poll_readiness is independent from the IAM grant pattern and should not be folded into this work.

## Unified Superset Contract

The refactored rbtoe_iam_grant guarantees:
- Read-modify-write via resource-type-appropriate API (GET method, SET method, URL construction vary)
- Etag optimistic concurrency (extract and send when API provides it)
- Propagation retry on 400 "does not exist" per RBSCIP backoff profile
- Transient retry on 429/5xx with bounded wait
- Read-back verification confirming the granted role is observable
- Vanilla policy initialization when resource has no existing policy
- Pre-flight SA existence check when member is a service account

Every grant gets every guarantee. Harmless where not strictly needed, protective everywhere.

## BCG Compliance Finding

A design using bash dynamic scoping (local -r ZRBGI_* in outer wrappers visible to inner function) was rejected as non-BCG. Two BCG-compliant approaches identified:

Option A: One inner function with 10+ explicit positional parameters. Verbose but honest.

Option B (favored): Extracted step helpers (zrbgi_read_policy, zrbgi_compose_binding, zrbgi_write_policy, zrbgi_verify_binding) called by per-resource outer functions that own their retry loops. The flow is visible in each function; the HTTP/jq mechanics are shared. More BCG-idiomatic.

Final selection deferred to this pace.

## Spec Definition Aesthetic

From AkAAa we established: every line in a rbtoe_ definition should be load-bearing. No boilerplate sections. Content complexity tracks behavioral complexity. Closing context lines (like RBSCIP backoff references) appear only when they earn their place. The unified rbtoe_iam_grant definition should follow api_enable as baseline inspiration, with richer contract sentence reflecting the superset guarantees.

## Implementation Plan

1. Refactor rbgi_IAM.sh: extract shared helpers, rewrite 4 functions as thin wrappers calling helpers
2. Add rbgi_grant_secret_iam wrapper for Secret Manager (eliminates 3x copy-paste in rbgp_Payor.sh)
3. Update all ~30 call sites if function signatures change
4. Collapse 4 rbtoe_iam_grant_* mapping entries and definitions to 1 rbtoe_iam_grant in RBS0
5. Update 4 subdocuments (RBSDC, RBSDI, RBSGR, RBSRC) to use unified term
6. Consider whether RBSCIP prose benefits from referencing the unified term

## Files

- Tools/rbk/rbgi_IAM.sh: refactor 4 functions into shared helpers plus thin wrappers
- Tools/rbk/rbgp_Payor.sh: extract 3x inline secret IAM copy-paste, update call sites
- Tools/rbk/rbgg_Governor.sh: update call sites
- Tools/rbk/rbga_ArtifactRegistry.sh: update call sites
- Tools/rbk/rbgb_Buckets.sh: update call sites
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc: collapse 4 rbtoe_iam_grant_* to 1 rbtoe_iam_grant
- Tools/rbk/vov_veiled/RBSDC-depot_create.adoc, RBSDI-director_create.adoc, RBSGR-governor_reset.adoc, RBSRC-retriever_create.adoc: update term references
- Tools/rbk/vov_veiled/RBSCIP-IamPropagation.adoc: may benefit from referencing unified term

## Not In Scope

- Complete-policy-write patterns in rbgg_Governor.sh (multi-binding atomic writes, genuinely different operation)
- rbtoe_poll_readiness (independent concern, different failure class)
- Changing runtime behavior: this is a refactor for clarity not functionality
- BCG-BashConsoleGuide.md changes

**[260309-1943] rough**

Drafted from ₢AkAAu in ₣Ak.

## Goal

Unify the IAM grant implementation in rbgi_IAM.sh and its specification in RBS0 to reduce cognitive load from patchwork evolution. Collapse 4 rbtoe_iam_grant_* linked terms to 1 rbtoe_iam_grant with a uniform superset behavioral contract.

## Current State: 4 Variants With Arbitrary-Feeling Differences

| Dimension | project | repo | sa | bucket |
|---|---|---|---|---|
| API | CRM v1 | Artifact Registry v1 | IAM v1 | Storage JSON v1 |
| GET method | POST + version body | POST + empty body | POST + empty body | GET (no body) |
| SET method | POST | POST | POST | PUT |
| Empty policy | errors | 404 -> vanilla init | non-200 -> vanilla init | non-200 -> vanilla init |
| Etag concurrency | yes | no | no | yes (when present) |
| Inner transient retry | yes (429/5xx) | no | no | no |
| Read-back verify | yes | no | no | no |
| Pre-flight check | no | no | SA existence | no |

All 4 share: propagation retry on HTTP 400 "does not exist" per RBSCIP backoff profile (3s initial, 2x, 20s cap, 420s deadline). Call sites: ~30 across rbgp_Payor.sh (majority), rbgg_Governor.sh, rbga_ArtifactRegistry.sh, rbgb_Buckets.sh.

## Two Failure Classes (Do Not Conflate)

Member-SA propagation: A newly-created SA is visible via GET but not yet usable as a member in setIamPolicy (returns 400 "does not exist"). You cannot pre-poll for this — the only test is attempting the policy operation. This is why retry lives INSIDE the grant function. All 4 rbgi_add_* handle this.

Resource propagation: A newly-created resource (secret, SA) is not yet visible at all. Handled BEFORE the grant by rbgu_poll_until_ok (rbtoe_poll_readiness). The Secret Manager inline grants in rbgp_Payor.sh rely on this pre-poll and therefore need no propagation retry of their own.

rbtoe_poll_readiness is independent from the IAM grant pattern and should not be folded into this work.

## Unified Superset Contract

The refactored rbtoe_iam_grant guarantees:
- Read-modify-write via resource-type-appropriate API (GET method, SET method, URL construction vary)
- Etag optimistic concurrency (extract and send when API provides it)
- Propagation retry on 400 "does not exist" per RBSCIP backoff profile
- Transient retry on 429/5xx with bounded wait
- Read-back verification confirming the granted role is observable
- Vanilla policy initialization when resource has no existing policy
- Pre-flight SA existence check when member is a service account

Every grant gets every guarantee. Harmless where not strictly needed, protective everywhere.

## BCG Compliance Finding

A design using bash dynamic scoping (local -r ZRBGI_* in outer wrappers visible to inner function) was rejected as non-BCG. Two BCG-compliant approaches identified:

Option A: One inner function with 10+ explicit positional parameters. Verbose but honest.

Option B (favored): Extracted step helpers (zrbgi_read_policy, zrbgi_compose_binding, zrbgi_write_policy, zrbgi_verify_binding) called by per-resource outer functions that own their retry loops. The flow is visible in each function; the HTTP/jq mechanics are shared. More BCG-idiomatic.

Final selection deferred to this pace.

## Spec Definition Aesthetic

From AkAAa we established: every line in a rbtoe_ definition should be load-bearing. No boilerplate sections. Content complexity tracks behavioral complexity. Closing context lines (like RBSCIP backoff references) appear only when they earn their place. The unified rbtoe_iam_grant definition should follow api_enable as baseline inspiration, with richer contract sentence reflecting the superset guarantees.

## Implementation Plan

1. Refactor rbgi_IAM.sh: extract shared helpers, rewrite 4 functions as thin wrappers calling helpers
2. Add rbgi_grant_secret_iam wrapper for Secret Manager (eliminates 3x copy-paste in rbgp_Payor.sh)
3. Update all ~30 call sites if function signatures change
4. Collapse 4 rbtoe_iam_grant_* mapping entries and definitions to 1 rbtoe_iam_grant in RBS0
5. Update 4 subdocuments (RBSDC, RBSDI, RBSGR, RBSRC) to use unified term
6. Consider whether RBSCIP prose benefits from referencing the unified term

## Files

- Tools/rbw/rbgi_IAM.sh: refactor 4 functions into shared helpers plus thin wrappers
- Tools/rbw/rbgp_Payor.sh: extract 3x inline secret IAM copy-paste, update call sites
- Tools/rbw/rbgg_Governor.sh: update call sites
- Tools/rbw/rbga_ArtifactRegistry.sh: update call sites
- Tools/rbw/rbgb_Buckets.sh: update call sites
- lenses/RBS0-SpecTop.adoc: collapse 4 rbtoe_iam_grant_* to 1 rbtoe_iam_grant
- lenses/RBSDC-depot_create.adoc, RBSDI-director_create.adoc, RBSGR-governor_reset.adoc, RBSRC-retriever_create.adoc: update term references
- lenses/RBSCIP-IamPropagation.adoc: may benefit from referencing unified term

## Not In Scope

- Complete-policy-write patterns in rbgg_Governor.sh (multi-binding atomic writes, genuinely different operation)
- rbtoe_poll_readiness (independent concern, different failure class)
- Changing runtime behavior: this is a refactor for clarity not functionality
- BCG-BashConsoleGuide.md changes

**[260309-0948] rough**

## Goal

Unify the IAM grant implementation in rbgi_IAM.sh and its specification in RBS0 to reduce cognitive load from patchwork evolution. Collapse 4 rbtoe_iam_grant_* linked terms to 1 rbtoe_iam_grant with a uniform superset behavioral contract.

## Current State: 4 Variants With Arbitrary-Feeling Differences

| Dimension | project | repo | sa | bucket |
|---|---|---|---|---|
| API | CRM v1 | Artifact Registry v1 | IAM v1 | Storage JSON v1 |
| GET method | POST + version body | POST + empty body | POST + empty body | GET (no body) |
| SET method | POST | POST | POST | PUT |
| Empty policy | errors | 404 -> vanilla init | non-200 -> vanilla init | non-200 -> vanilla init |
| Etag concurrency | yes | no | no | yes (when present) |
| Inner transient retry | yes (429/5xx) | no | no | no |
| Read-back verify | yes | no | no | no |
| Pre-flight check | no | no | SA existence | no |

All 4 share: propagation retry on HTTP 400 "does not exist" per RBSCIP backoff profile (3s initial, 2x, 20s cap, 420s deadline). Call sites: ~30 across rbgp_Payor.sh (majority), rbgg_Governor.sh, rbga_ArtifactRegistry.sh, rbgb_Buckets.sh.

## Two Failure Classes (Do Not Conflate)

Member-SA propagation: A newly-created SA is visible via GET but not yet usable as a member in setIamPolicy (returns 400 "does not exist"). You cannot pre-poll for this — the only test is attempting the policy operation. This is why retry lives INSIDE the grant function. All 4 rbgi_add_* handle this.

Resource propagation: A newly-created resource (secret, SA) is not yet visible at all. Handled BEFORE the grant by rbgu_poll_until_ok (rbtoe_poll_readiness). The Secret Manager inline grants in rbgp_Payor.sh rely on this pre-poll and therefore need no propagation retry of their own.

rbtoe_poll_readiness is independent from the IAM grant pattern and should not be folded into this work.

## Unified Superset Contract

The refactored rbtoe_iam_grant guarantees:
- Read-modify-write via resource-type-appropriate API (GET method, SET method, URL construction vary)
- Etag optimistic concurrency (extract and send when API provides it)
- Propagation retry on 400 "does not exist" per RBSCIP backoff profile
- Transient retry on 429/5xx with bounded wait
- Read-back verification confirming the granted role is observable
- Vanilla policy initialization when resource has no existing policy
- Pre-flight SA existence check when member is a service account

Every grant gets every guarantee. Harmless where not strictly needed, protective everywhere.

## BCG Compliance Finding

A design using bash dynamic scoping (local -r ZRBGI_* in outer wrappers visible to inner function) was rejected as non-BCG. Two BCG-compliant approaches identified:

Option A: One inner function with 10+ explicit positional parameters. Verbose but honest.

Option B (favored): Extracted step helpers (zrbgi_read_policy, zrbgi_compose_binding, zrbgi_write_policy, zrbgi_verify_binding) called by per-resource outer functions that own their retry loops. The flow is visible in each function; the HTTP/jq mechanics are shared. More BCG-idiomatic.

Final selection deferred to this pace.

## Spec Definition Aesthetic

From AkAAa we established: every line in a rbtoe_ definition should be load-bearing. No boilerplate sections. Content complexity tracks behavioral complexity. Closing context lines (like RBSCIP backoff references) appear only when they earn their place. The unified rbtoe_iam_grant definition should follow api_enable as baseline inspiration, with richer contract sentence reflecting the superset guarantees.

## Implementation Plan

1. Refactor rbgi_IAM.sh: extract shared helpers, rewrite 4 functions as thin wrappers calling helpers
2. Add rbgi_grant_secret_iam wrapper for Secret Manager (eliminates 3x copy-paste in rbgp_Payor.sh)
3. Update all ~30 call sites if function signatures change
4. Collapse 4 rbtoe_iam_grant_* mapping entries and definitions to 1 rbtoe_iam_grant in RBS0
5. Update 4 subdocuments (RBSDC, RBSDI, RBSGR, RBSRC) to use unified term
6. Consider whether RBSCIP prose benefits from referencing the unified term

## Files

- Tools/rbw/rbgi_IAM.sh: refactor 4 functions into shared helpers plus thin wrappers
- Tools/rbw/rbgp_Payor.sh: extract 3x inline secret IAM copy-paste, update call sites
- Tools/rbw/rbgg_Governor.sh: update call sites
- Tools/rbw/rbga_ArtifactRegistry.sh: update call sites
- Tools/rbw/rbgb_Buckets.sh: update call sites
- lenses/RBS0-SpecTop.adoc: collapse 4 rbtoe_iam_grant_* to 1 rbtoe_iam_grant
- lenses/RBSDC-depot_create.adoc, RBSDI-director_create.adoc, RBSGR-governor_reset.adoc, RBSRC-retriever_create.adoc: update term references
- lenses/RBSCIP-IamPropagation.adoc: may benefit from referencing unified term

## Not In Scope

- Complete-policy-write patterns in rbgg_Governor.sh (multi-binding atomic writes, genuinely different operation)
- rbtoe_poll_readiness (independent concern, different failure class)
- Changing runtime behavior: this is a refactor for clarity not functionality
- BCG-BashConsoleGuide.md changes

**[260309-0945] rough**

## Goal

Unify the IAM grant implementation in rbgi_IAM.sh and its specification in RBS0 to reduce cognitive load from patchwork evolution.

## Context

During AkAAa (rbscip-linked-term-consideration), we created 5 rbtoe_ linked terms for IAM patterns and installed them in 4 subdocuments. We discovered the 4 rbgi_add_* functions share a contract (read-modify-write with propagation retry per RBSCIP) but differ in ways that feel arbitrary without context: only project has read-back verification and inner transient retry, only sa has pre-flight existence check, bucket uses GET/PUT where others use POST/POST, project and bucket use etag while repo and sa do not. We also found inline Secret Manager IAM grants (3x copy-paste in rbgp_Payor.sh) that should be extracted, and complete-policy-write patterns in rbgg_Governor.sh that are genuinely different (multi-binding atomic) and should stay separate.

## Key Design Insight

The 4 variants handle the SAME failure class (member-SA propagation in IAM policy ops) and should converge to uniform behavioral guarantees. The Secret Manager inline grants handle a DIFFERENT failure class (resource propagation, handled by poll_readiness beforehand) but could still use the shared implementation harmlessly. The complete-policy-write pattern (multi-binding atomic writes to prevent stale-read races) is genuinely different and stays separate.

## BCG Compliance Finding

A design using bash dynamic scoping (local -r ZRBGI_* in outer wrappers visible to inner function) was rejected as non-BCG. Two BCG-compliant approaches identified: (A) one inner function with 10+ explicit positional parameters, or (B) extracted step helpers (zrbgi_read_policy, zrbgi_compose_binding, zrbgi_write_policy, zrbgi_verify_binding) called by per-resource outer functions that own their retry loops. Option B was favored as more BCG-idiomatic but final selection deferred to this pace.

## Spec Convergence

If implementation unifies, collapse 4 rbtoe_iam_grant_* terms to 1 rbtoe_iam_grant with superset contract. Resource-type API details become implementation, not spec.

## Files

- Tools/rbw/rbgi_IAM.sh: refactor 4 functions into shared helpers plus thin wrappers
- Tools/rbw/rbgp_Payor.sh: extract 3x inline secret IAM copy-paste
- lenses/RBS0-SpecTop.adoc: potentially collapse 4 rbtoe_iam_grant_* to 1
- lenses/RBSDC, RBSDI, RBSGR, RBSRC subdocs: update if spec terms change
- lenses/RBSCIP-IamPropagation.adoc: may benefit from referencing new unified term

## Not In Scope

- Complete-policy-write patterns (genuinely different operation)
- Changing runtime behavior: this is a refactor for clarity not functionality
- BCG-BashConsoleGuide.md changes

### rbgi-etag-and-error-compliance (₢AsAAF) [complete]

**[260311-1131] complete**

## Goal

Bring the 4 grant functions in rbgi_IAM.sh into compliance with the corrected RBS0 spec definitions and RBSCIG research findings.

## Changes

### Operational Invariant Comment (lines 21-32)
- Current comment explains 409 only as "resource already exists (state drift)" — true for resource creation, but for setIamPolicy 409 means concurrent policy change (etag mismatch). Add a note distinguishing resource-creation 409 from policy-concurrency 409, since this pace makes the project function's 409 handler explicitly about etag mismatch.

### rbtoe_iam_grant_project (rbgi_add_project_iam_role)
- Remove dead 412 case (line 199) — CRM never returns 412; etag mismatch is 409 ABORTED
- Update 409 case message (line 201) from "HTTP 409 Conflict (fatal by invariant)" to note this is the etag mismatch code for CRM
- Update comment at line 180 ("fatal on 409/412 by policy") to reflect 412 removal
- Existing etag handling is already correct (extract, assert non-empty, pass)
- Already requests requestedPolicyVersion=3 — no change needed

### rbtoe_iam_grant_repo (rbgi_add_repo_iam_role)
- Remove 404 -> vanilla-init handler (lines 304-307) — API never returns 404 for existing repos; 404 would mean repo doesn't exist and vanilla-init masks that real error. Replace with rbgu_http_require_ok
- Extract etag explicitly: rbgu_json_field_capture + assert non-empty (same pattern as project function). Pass to compose function instead of empty string (line 315)
- HIGHEST-RISK CHANGE: Change HTTP method from POST to GET for getIamPolicy (line 286). AR proto specifies GET. rbgu_http_json supports GET (bucket already uses it). Omit body_file argument. Cannot be unit-tested — only validated by AsAAC live depot test
- Add `?options.requestedPolicyVersion=3` query parameter to GET URL — protects against silent IAM Condition destruction per RBSCIG. Note: for GET endpoints the version goes as a query param, not in body
- Replace flat SET path (lines 322-340) with inner transient retry loop modeled on project function (lines 184-208). Case statement handles: 200 (success), 409 ABORTED (fatal, etag mismatch), 429/5xx (transient retry), default (rbgu_http_require_ok). SET infix must include inner elapsed time to avoid temp file collision: `"${z_set_infix}-${z_set_elapsed}s"` pattern

### rbtoe_iam_grant_sa (rbgi_add_sa_iam_role)
- Remove non-200 -> vanilla-init handler (lines 411-413) — API returns 200 with empty policy for existing SAs; non-200 means SA doesn't exist or access denied. Replace with rbgu_http_require_ok
- Extract etag explicitly: same rbgu_json_field_capture pattern. Pass to compose function instead of empty string (line 419)
- Add requestedPolicyVersion=3 in POST body for getIamPolicy — currently sends ZRBGI_EMPTY_JSON (which is `{}`); change to `{"options":{"requestedPolicyVersion":3}}`. Implementation: add ZRBGI_VERSION3_BODY constant in zrbgi_kindle (shared with project function's body pattern at line 143), or create a local body file per the project pattern
- Replace flat SET path (lines 427-446) with inner transient retry loop. Case statement: 200 (success), 409 ABORTED (fatal, etag mismatch), 429/5xx (transient retry), default (rbgu_http_require_ok)

### rbtoe_iam_grant_bucket (rbgi_add_bucket_iam_role)
- Remove non-200 -> vanilla-init handler (lines 500-503) — buckets always have IAM policies (200 with inherited bindings per RBSCIG); non-200 is a real error. Replace with rbgu_http_require_ok
- Assert etag non-empty instead of fallback to empty (line 507) — API always returns etag; change `|| z_etag=""` to `|| buc_die "Missing etag"` and add `test -n "${z_etag}" || buc_die "Empty etag"`
- Add `?options.requestedPolicyVersion=3` query parameter to GET URL (line 482)
- Replace flat SET path (lines 518-536) with inner transient retry loop. Case statement: 200 (success), 412 Precondition Failed (fatal, etag mismatch — Storage uniquely uses 412 not 409 per RBSCIG), 409 ABORTED (fatal, defensive — Storage shouldn't return this but handle it), 429/5xx (transient retry), default (rbgu_http_require_ok). NOTE: bucket currently has NO case statement and NO explicit 412 handler — this is ADDING one, not preserving one

## Files
- Tools/rbk/rbgi_IAM.sh — all changes in this file

## Risk Assessment

Medium overall. Highest risk: AR POST->GET method change.

- AR POST->GET (highest risk): Only validatable by AsAAC live depot test. rbgu_http_json supports GET with optional body_file (5th arg). Bucket already uses GET for getIamPolicy (line 482). If AR rejects GET despite proto saying GET, fallback is reverting to POST.
- False handler removal (medium risk): Repo 404, SA non-200, bucket non-200 handlers currently mask real errors by vanilla-initing. Removing them means we die on previously-silent errors. This is correct (masking is worse) but could surface latent issues that were previously invisible.
- Etag behavioral change (low risk): Adding etag to repo/SA setIamPolicy means concurrent writes (from Google-internal auto-provisioning) now produce fatal 409 instead of silent overwrites. Under single-writer operational invariant this shouldn't happen from RB operations. If it does, the 409 surfaces a real concurrency issue that was previously silently clobbered.
- Transient retry addition (zero risk): If APIs never return 429/5xx, the code never fires. If they do, retry is strictly more resilient than dying.
- requestedPolicyVersion=3 (zero risk): For resources without IAM Conditions, server returns version 1 regardless. For resources with conditions, prevents silent destruction.

Green baseline from AsAAB validates current happy path. All behavioral changes are in the direction of correctness.

**[260311-1119] bridled**

## Goal

Bring the 4 grant functions in rbgi_IAM.sh into compliance with the corrected RBS0 spec definitions and RBSCIG research findings.

## Changes

### Operational Invariant Comment (lines 21-32)
- Current comment explains 409 only as "resource already exists (state drift)" — true for resource creation, but for setIamPolicy 409 means concurrent policy change (etag mismatch). Add a note distinguishing resource-creation 409 from policy-concurrency 409, since this pace makes the project function's 409 handler explicitly about etag mismatch.

### rbtoe_iam_grant_project (rbgi_add_project_iam_role)
- Remove dead 412 case (line 199) — CRM never returns 412; etag mismatch is 409 ABORTED
- Update 409 case message (line 201) from "HTTP 409 Conflict (fatal by invariant)" to note this is the etag mismatch code for CRM
- Update comment at line 180 ("fatal on 409/412 by policy") to reflect 412 removal
- Existing etag handling is already correct (extract, assert non-empty, pass)
- Already requests requestedPolicyVersion=3 — no change needed

### rbtoe_iam_grant_repo (rbgi_add_repo_iam_role)
- Remove 404 -> vanilla-init handler (lines 304-307) — API never returns 404 for existing repos; 404 would mean repo doesn't exist and vanilla-init masks that real error. Replace with rbgu_http_require_ok
- Extract etag explicitly: rbgu_json_field_capture + assert non-empty (same pattern as project function). Pass to compose function instead of empty string (line 315)
- HIGHEST-RISK CHANGE: Change HTTP method from POST to GET for getIamPolicy (line 286). AR proto specifies GET. rbgu_http_json supports GET (bucket already uses it). Omit body_file argument. Cannot be unit-tested — only validated by AsAAC live depot test
- Add `?options.requestedPolicyVersion=3` query parameter to GET URL — protects against silent IAM Condition destruction per RBSCIG. Note: for GET endpoints the version goes as a query param, not in body
- Replace flat SET path (lines 322-340) with inner transient retry loop modeled on project function (lines 184-208). Case statement handles: 200 (success), 409 ABORTED (fatal, etag mismatch), 429/5xx (transient retry), default (rbgu_http_require_ok). SET infix must include inner elapsed time to avoid temp file collision: `"${z_set_infix}-${z_set_elapsed}s"` pattern

### rbtoe_iam_grant_sa (rbgi_add_sa_iam_role)
- Remove non-200 -> vanilla-init handler (lines 411-413) — API returns 200 with empty policy for existing SAs; non-200 means SA doesn't exist or access denied. Replace with rbgu_http_require_ok
- Extract etag explicitly: same rbgu_json_field_capture pattern. Pass to compose function instead of empty string (line 419)
- Add requestedPolicyVersion=3 in POST body for getIamPolicy — currently sends ZRBGI_EMPTY_JSON (which is `{}`); change to `{"options":{"requestedPolicyVersion":3}}`. Implementation: add ZRBGI_VERSION3_BODY constant in zrbgi_kindle (shared with project function's body pattern at line 143), or create a local body file per the project pattern
- Replace flat SET path (lines 427-446) with inner transient retry loop. Case statement: 200 (success), 409 ABORTED (fatal, etag mismatch), 429/5xx (transient retry), default (rbgu_http_require_ok)

### rbtoe_iam_grant_bucket (rbgi_add_bucket_iam_role)
- Remove non-200 -> vanilla-init handler (lines 500-503) — buckets always have IAM policies (200 with inherited bindings per RBSCIG); non-200 is a real error. Replace with rbgu_http_require_ok
- Assert etag non-empty instead of fallback to empty (line 507) — API always returns etag; change `|| z_etag=""` to `|| buc_die "Missing etag"` and add `test -n "${z_etag}" || buc_die "Empty etag"`
- Add `?options.requestedPolicyVersion=3` query parameter to GET URL (line 482)
- Replace flat SET path (lines 518-536) with inner transient retry loop. Case statement: 200 (success), 412 Precondition Failed (fatal, etag mismatch — Storage uniquely uses 412 not 409 per RBSCIG), 409 ABORTED (fatal, defensive — Storage shouldn't return this but handle it), 429/5xx (transient retry), default (rbgu_http_require_ok). NOTE: bucket currently has NO case statement and NO explicit 412 handler — this is ADDING one, not preserving one

## Files
- Tools/rbk/rbgi_IAM.sh — all changes in this file

## Risk Assessment

Medium overall. Highest risk: AR POST->GET method change.

- AR POST->GET (highest risk): Only validatable by AsAAC live depot test. rbgu_http_json supports GET with optional body_file (5th arg). Bucket already uses GET for getIamPolicy (line 482). If AR rejects GET despite proto saying GET, fallback is reverting to POST.
- False handler removal (medium risk): Repo 404, SA non-200, bucket non-200 handlers currently mask real errors by vanilla-initing. Removing them means we die on previously-silent errors. This is correct (masking is worse) but could surface latent issues that were previously invisible.
- Etag behavioral change (low risk): Adding etag to repo/SA setIamPolicy means concurrent writes (from Google-internal auto-provisioning) now produce fatal 409 instead of silent overwrites. Under single-writer operational invariant this shouldn't happen from RB operations. If it does, the 409 surfaces a real concurrency issue that was previously silently clobbered.
- Transient retry addition (zero risk): If APIs never return 429/5xx, the code never fires. If they do, retry is strictly more resilient than dying.
- requestedPolicyVersion=3 (zero risk): For resources without IAM Conditions, server returns version 1 regardless. For resources with conditions, prevents silent destruction.

Green baseline from AsAAB validates current happy path. All behavioral changes are in the direction of correctness.

*Direction:* Docket is comprehensive with clear per-function changes, risk assessment, and validation strategy. Project function retry loop provides template. All behavioral changes move toward correctness.

**[260311-1115] rough**

## Goal

Bring the 4 grant functions in rbgi_IAM.sh into compliance with the corrected RBS0 spec definitions and RBSCIG research findings.

## Changes

### Operational Invariant Comment (lines 21-32)
- Current comment explains 409 only as "resource already exists (state drift)" — true for resource creation, but for setIamPolicy 409 means concurrent policy change (etag mismatch). Add a note distinguishing resource-creation 409 from policy-concurrency 409, since this pace makes the project function's 409 handler explicitly about etag mismatch.

### rbtoe_iam_grant_project (rbgi_add_project_iam_role)
- Remove dead 412 case (line 199) — CRM never returns 412; etag mismatch is 409 ABORTED
- Update 409 case message (line 201) from "HTTP 409 Conflict (fatal by invariant)" to note this is the etag mismatch code for CRM
- Update comment at line 180 ("fatal on 409/412 by policy") to reflect 412 removal
- Existing etag handling is already correct (extract, assert non-empty, pass)
- Already requests requestedPolicyVersion=3 — no change needed

### rbtoe_iam_grant_repo (rbgi_add_repo_iam_role)
- Remove 404 -> vanilla-init handler (lines 304-307) — API never returns 404 for existing repos; 404 would mean repo doesn't exist and vanilla-init masks that real error. Replace with rbgu_http_require_ok
- Extract etag explicitly: rbgu_json_field_capture + assert non-empty (same pattern as project function). Pass to compose function instead of empty string (line 315)
- HIGHEST-RISK CHANGE: Change HTTP method from POST to GET for getIamPolicy (line 286). AR proto specifies GET. rbgu_http_json supports GET (bucket already uses it). Omit body_file argument. Cannot be unit-tested — only validated by AsAAC live depot test
- Add `?options.requestedPolicyVersion=3` query parameter to GET URL — protects against silent IAM Condition destruction per RBSCIG. Note: for GET endpoints the version goes as a query param, not in body
- Replace flat SET path (lines 322-340) with inner transient retry loop modeled on project function (lines 184-208). Case statement handles: 200 (success), 409 ABORTED (fatal, etag mismatch), 429/5xx (transient retry), default (rbgu_http_require_ok). SET infix must include inner elapsed time to avoid temp file collision: `"${z_set_infix}-${z_set_elapsed}s"` pattern

### rbtoe_iam_grant_sa (rbgi_add_sa_iam_role)
- Remove non-200 -> vanilla-init handler (lines 411-413) — API returns 200 with empty policy for existing SAs; non-200 means SA doesn't exist or access denied. Replace with rbgu_http_require_ok
- Extract etag explicitly: same rbgu_json_field_capture pattern. Pass to compose function instead of empty string (line 419)
- Add requestedPolicyVersion=3 in POST body for getIamPolicy — currently sends ZRBGI_EMPTY_JSON (which is `{}`); change to `{"options":{"requestedPolicyVersion":3}}`. Implementation: add ZRBGI_VERSION3_BODY constant in zrbgi_kindle (shared with project function's body pattern at line 143), or create a local body file per the project pattern
- Replace flat SET path (lines 427-446) with inner transient retry loop. Case statement: 200 (success), 409 ABORTED (fatal, etag mismatch), 429/5xx (transient retry), default (rbgu_http_require_ok)

### rbtoe_iam_grant_bucket (rbgi_add_bucket_iam_role)
- Remove non-200 -> vanilla-init handler (lines 500-503) — buckets always have IAM policies (200 with inherited bindings per RBSCIG); non-200 is a real error. Replace with rbgu_http_require_ok
- Assert etag non-empty instead of fallback to empty (line 507) — API always returns etag; change `|| z_etag=""` to `|| buc_die "Missing etag"` and add `test -n "${z_etag}" || buc_die "Empty etag"`
- Add `?options.requestedPolicyVersion=3` query parameter to GET URL (line 482)
- Replace flat SET path (lines 518-536) with inner transient retry loop. Case statement: 200 (success), 412 Precondition Failed (fatal, etag mismatch — Storage uniquely uses 412 not 409 per RBSCIG), 409 ABORTED (fatal, defensive — Storage shouldn't return this but handle it), 429/5xx (transient retry), default (rbgu_http_require_ok). NOTE: bucket currently has NO case statement and NO explicit 412 handler — this is ADDING one, not preserving one

## Files
- Tools/rbk/rbgi_IAM.sh — all changes in this file

## Risk Assessment

Medium overall. Highest risk: AR POST->GET method change.

- AR POST->GET (highest risk): Only validatable by AsAAC live depot test. rbgu_http_json supports GET with optional body_file (5th arg). Bucket already uses GET for getIamPolicy (line 482). If AR rejects GET despite proto saying GET, fallback is reverting to POST.
- False handler removal (medium risk): Repo 404, SA non-200, bucket non-200 handlers currently mask real errors by vanilla-initing. Removing them means we die on previously-silent errors. This is correct (masking is worse) but could surface latent issues that were previously invisible.
- Etag behavioral change (low risk): Adding etag to repo/SA setIamPolicy means concurrent writes (from Google-internal auto-provisioning) now produce fatal 409 instead of silent overwrites. Under single-writer operational invariant this shouldn't happen from RB operations. If it does, the 409 surfaces a real concurrency issue that was previously silently clobbered.
- Transient retry addition (zero risk): If APIs never return 429/5xx, the code never fires. If they do, retry is strictly more resilient than dying.
- requestedPolicyVersion=3 (zero risk): For resources without IAM Conditions, server returns version 1 regardless. For resources with conditions, prevents silent destruction.

Green baseline from AsAAB validates current happy path. All behavioral changes are in the direction of correctness.

**[260311-1110] rough**

## Goal

Bring the 4 grant functions in rbgi_IAM.sh into compliance with the corrected RBS0 spec definitions and RBSCIG research findings.

## Changes

### rbtoe_iam_grant_project (rbgi_add_project_iam_role)
- Remove dead 412 case (line 199) — CRM never returns 412; etag mismatch is 409 ABORTED
- Update 409 case message (line 201) from "HTTP 409 Conflict (fatal by invariant)" to note this is the etag mismatch code for CRM
- Existing etag handling is already correct (extract, assert non-empty, pass)
- Already requests requestedPolicyVersion=3 — no change needed

### rbtoe_iam_grant_repo (rbgi_add_repo_iam_role)
- Remove 404 -> vanilla-init handler (lines 304-307) — API never returns 404 for existing repos; 404 would mean repo doesn't exist and vanilla-init masks that real error. Replace with rbgu_http_require_ok
- Extract etag explicitly: rbgu_json_field_capture + assert non-empty (same pattern as project function). Pass to compose function instead of empty string (line 315)
- Add 409 ABORTED handler in SET response path (fatal, etag mismatch) — insert case before rbgu_http_require_ok at line 340
- HIGHEST-RISK CHANGE: Change HTTP method from POST to GET for getIamPolicy (line 286). AR proto specifies GET. rbgu_http_json supports GET (bucket already uses it). Omit body_file argument. Cannot be unit-tested — only validated by AsAAC live depot test
- Add requestedPolicyVersion=3 as query parameter on GET URL — protects against silent IAM Condition destruction per RBSCIG
- Add 429/5xx transient retry on SET with inner loop (currently absent; pattern from project function lines 184-208)

### rbtoe_iam_grant_sa (rbgi_add_sa_iam_role)
- Remove non-200 -> vanilla-init handler (lines 411-413) — API returns 200 with empty policy for existing SAs; non-200 means SA doesn't exist or access denied. Replace with rbgu_http_require_ok
- Extract etag explicitly: same rbgu_json_field_capture pattern. Pass to compose function instead of empty string (line 419)
- Add 409 ABORTED handler in SET response path (fatal, etag mismatch) — insert case before rbgu_http_require_ok at line 446
- Add requestedPolicyVersion=3 in POST body for getIamPolicy — currently sends empty body {}; change to {"options":{"requestedPolicyVersion":3}}
- Add 429/5xx transient retry on SET with inner loop (currently absent)

### rbtoe_iam_grant_bucket (rbgi_add_bucket_iam_role)
- Remove non-200 -> vanilla-init handler (lines 500-503) — buckets always have IAM policies (200 with inherited bindings per RBSCIG); non-200 is a real error. Replace with rbgu_http_require_ok
- Assert etag non-empty instead of fallback to empty (line 507) — API always returns etag; change fallback to buc_die
- Add requestedPolicyVersion=3 as query parameter on GET URL
- Add 429/5xx transient retry on SET with inner loop (currently absent)
- 412 Precondition Failed handler is correct for Storage and unique to this API — preserve as-is
- Add 409 handler as fatal (Storage shouldn't return it, but defensive)

## Files
- Tools/rbk/rbgi_IAM.sh — all changes in this file

## Risk Assessment

Medium overall. Highest risk: AR POST->GET method change.

- AR POST->GET (highest risk): Only validatable by AsAAC live depot test. rbgu_http_json supports GET. Bucket already uses GET for getIamPolicy. If AR rejects GET despite proto saying GET, fallback is reverting to POST.
- False handler removal (medium risk): Repo 404, SA non-200, bucket non-200 handlers currently mask real errors by vanilla-initing. Removing them means we die on previously-silent errors. This is correct (masking is worse) but could surface latent issues that were previously invisible.
- Etag behavioral change (low risk): Adding etag to repo/SA setIamPolicy means concurrent writes (from Google-internal auto-provisioning) now produce fatal 409 instead of silent overwrites. Under single-writer operational invariant this shouldn't happen from RB operations. If it does, the 409 surfaces a real concurrency issue that was previously silently clobbered.
- Transient retry addition (zero risk): If APIs never return 429/5xx, the code never fires. If they do, retry is strictly more resilient than dying.
- requestedPolicyVersion=3 (zero risk): For resources without IAM Conditions, server returns version 1 regardless. For resources with conditions, prevents silent destruction.

Green baseline from AsAAB validates current happy path. All behavioral changes are in the direction of correctness.

**[260311-1053] rough**

## Goal

Bring the 4 grant functions in rbgi_IAM.sh into compliance with the corrected RBS0 spec definitions and RBSCIG research findings.

## Changes

### rbtoe_iam_grant_project (rbgi_add_project_iam_role)
- Change 412 handler (line 199) to 409 ABORTED — CRM returns 409, not 412
- The 412 case should become a dead-code removal or unreachable assertion
- Existing etag handling is already correct (extract, assert non-empty, pass)

### rbtoe_iam_grant_repo (rbgi_add_repo_iam_role)
- Remove 404 → vanilla-init handler (line 304-307) — API never returns 404 for existing repos; replace with rbgu_http_require_ok
- Extract etag explicitly from getIamPolicy response; pass to compose function
- Add 409 ABORTED handler (fatal, etag mismatch)
- Change HTTP method from POST to GET for getIamPolicy (proto specifies GET)
- Add 429/5xx transient retry on SET (currently absent)

### rbtoe_iam_grant_sa (rbgi_add_sa_iam_role)
- Remove non-200 → vanilla-init handler (line 411-413) — API returns 200 for existing SAs; non-200 is real error; replace with rbgu_http_require_ok
- Extract etag explicitly from getIamPolicy response; pass to compose function
- Add 409 ABORTED handler (fatal, etag mismatch)
- Add 429/5xx transient retry on SET (currently absent)

### rbtoe_iam_grant_bucket (rbgi_add_bucket_iam_role)
- Assert etag non-empty instead of fallback to empty (line 507) — API always returns etag
- Add 429/5xx transient retry on SET (currently absent)
- 412 handler is correct for Storage (unique to this API)

## Files
- Tools/rbk/rbgi_IAM.sh — all changes in this file

## Risk
Medium. Removing false error handlers means we die on previously-silent errors. This is correct behavior (masking errors is worse) but could surface latent issues. The green baseline from AsAAB validates current happy path.

### rbgi-secret-manager-extraction (₢AsAAG) [complete]

**[260311-1139] complete**

## Goal

Extract the 3x copy-pasted Secret Manager IAM grants from rbgp_Payor.sh (lines 936-994) into a new rbgi_grant_secret_iam function in rbgi_IAM.sh. The new function follows the corrected contract pattern from the start: explicit etag, correct error handling, propagation retry.

## Current State

Three inline blocks in rbgp_depot_create grant roles/secretmanager.secretAccessor to the Cloud Build service agent on three secrets (api token, read token, webhook). Each block does getIamPolicy (GET) -> modify -> setIamPolicy (POST) with no etag passed to compose (accidental passthrough via jq), no propagation retry, and no transient retry. The inline code already uses GET correctly for getIamPolicy (lines 938, 958, 978).

Preceding the inline grants, three rbgu_poll_until_ok calls (lines 924-932) poll each secret's getIamPolicy endpoint until 200. These handle RESOURCE propagation (secret not yet visible after creation). They are a different failure class from MEMBER-SA propagation (SA not yet usable in setIamPolicy) and MUST be preserved as-is in rbgp_Payor.sh. Do NOT fold them into rbgi_grant_secret_iam. The buc_step at line 934 also remains.

## Function Signature

rbgi_grant_secret_iam token secret_resource_path member role parent_infix

Where:
- token: OAuth bearer token
- secret_resource_path: full resource path, e.g. projects/PROJECT_ID/secrets/SECRET_NAME (caller constructs from z_secret_parent + secret name constant, as at lines 921-923)
- member: full member string with prefix, e.g. serviceAccount:email@... (caller adds serviceAccount: prefix, matching inline pattern at lines 946, 966, 986)
- role: e.g. roles/secretmanager.secretAccessor (parameterized, not hardcoded)
- parent_infix: temp file infix for HTTP response tracking

URL construction inside function: ${RBGC_API_ROOT_SECRETMANAGER}${RBGC_SECRETMANAGER_V1}/${secret_resource_path}:getIamPolicy (and :setIamPolicy)

## New Function Contract
- GET getIamPolicy with `?options.requestedPolicyVersion=3` query parameter (Secret Manager proto specifies GET; inline code already uses GET)
- Require HTTP 200 (secrets always have policies once created per RBSCIG; non-200 is real error) via rbgu_http_require_ok
- Extract etag explicitly via rbgu_json_field_capture + assert non-empty (current inline code passes empty string to compose at lines 946, 966, 986 — change to explicit extraction)
- Compose binding with etag for concurrency-safe write via rbgu_jq_add_member_to_role_capture
- POST setIamPolicy with {"policy":...} envelope
- Inner transient retry loop on SET with case statement: 200 (success), 409 ABORTED (fatal, etag mismatch), 429/5xx (transient retry), default (rbgu_http_require_ok)
- Outer propagation retry per RBSCIP backoff profile (3s/2x/20s/420s) — NOTE: this is a BEHAVIORAL ADDITION, not just extraction. The inline code has zero propagation retry. Adding it aligns Secret Manager grants with the other 4 grant functions and handles the member-SA propagation failure class that the preceding rbgu_poll_until_ok does NOT cover.

## Call Site Replacement

Lines 936-994 of rbgp_Payor.sh (three blocks) become three calls:
```
rbgi_grant_secret_iam "${z_token}" "${z_api_secret_resource}" \
  "serviceAccount:${z_cb_service_agent}" "roles/secretmanager.secretAccessor" \
  "depot_secret_iam_api"
```
(similarly for read and webhook, with appropriate resource path and infix)

Lines 920-932 (buc_step + rbgu_poll_until_ok calls) remain UNTOUCHED.

## Spec and Doc Updates
- Add rbtoe_iam_grant_secret mapping + definition in RBS0 (following corrected contract pattern; reference RBSCIG)
- Update RBSDC-depot_create.adoc lines 220-229 to use {rbtoe_iam_grant_secret} linked term instead of inline API call description
- Update RBSCIG-IamGrantContracts.adoc: change "inline in depot_create" references to reference new function; update HTTP Method table and other tables to show Secret Manager as a first-class grant pattern
- Add ZRBGI_INFIX_SECRET_IAM and ZRBGI_INFIX_SECRET_IAM_SET constants to zrbgi_kindle

## Files
- Tools/rbk/rbgi_IAM.sh — add rbgi_grant_secret_iam + kindle constants
- Tools/rbk/rbgp_Payor.sh — replace 3x inline blocks (lines 936-994) with 3 calls; preserve poll calls (lines 920-932)
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — add rbtoe_iam_grant_secret mapping + definition
- Tools/rbk/vov_veiled/RBSDC-depot_create.adoc — use new linked term
- Tools/rbk/vov_veiled/RBSCIG-IamGrantContracts.adoc — promote Secret Manager from inline to first-class grant pattern

## Risk Assessment

Low-medium. The inline code works in production but lacks protections the other grant functions have.

- Extraction itself (low risk): Mechanical replacement of 3x copy-paste with 3 function calls. Same HTTP methods, same URLs, same body format.
- Etag addition (low risk): Same rationale as AsAAF — converts accidental passthrough to intentional concurrency safety.
- Propagation retry addition (medium risk): Behavioral change — operations that previously died immediately on member-SA propagation error will now retry for up to 420s. This is correct behavior (matches the other 4 grant functions) but changes timing. The preceding rbgu_poll_until_ok should have already resolved resource propagation, so member-SA propagation errors should be rare.
- Transient retry addition (zero risk): Same as AsAAF.
- requestedPolicyVersion=3 (zero risk): Same as AsAAF.

**[260311-1116] rough**

## Goal

Extract the 3x copy-pasted Secret Manager IAM grants from rbgp_Payor.sh (lines 936-994) into a new rbgi_grant_secret_iam function in rbgi_IAM.sh. The new function follows the corrected contract pattern from the start: explicit etag, correct error handling, propagation retry.

## Current State

Three inline blocks in rbgp_depot_create grant roles/secretmanager.secretAccessor to the Cloud Build service agent on three secrets (api token, read token, webhook). Each block does getIamPolicy (GET) -> modify -> setIamPolicy (POST) with no etag passed to compose (accidental passthrough via jq), no propagation retry, and no transient retry. The inline code already uses GET correctly for getIamPolicy (lines 938, 958, 978).

Preceding the inline grants, three rbgu_poll_until_ok calls (lines 924-932) poll each secret's getIamPolicy endpoint until 200. These handle RESOURCE propagation (secret not yet visible after creation). They are a different failure class from MEMBER-SA propagation (SA not yet usable in setIamPolicy) and MUST be preserved as-is in rbgp_Payor.sh. Do NOT fold them into rbgi_grant_secret_iam. The buc_step at line 934 also remains.

## Function Signature

rbgi_grant_secret_iam token secret_resource_path member role parent_infix

Where:
- token: OAuth bearer token
- secret_resource_path: full resource path, e.g. projects/PROJECT_ID/secrets/SECRET_NAME (caller constructs from z_secret_parent + secret name constant, as at lines 921-923)
- member: full member string with prefix, e.g. serviceAccount:email@... (caller adds serviceAccount: prefix, matching inline pattern at lines 946, 966, 986)
- role: e.g. roles/secretmanager.secretAccessor (parameterized, not hardcoded)
- parent_infix: temp file infix for HTTP response tracking

URL construction inside function: ${RBGC_API_ROOT_SECRETMANAGER}${RBGC_SECRETMANAGER_V1}/${secret_resource_path}:getIamPolicy (and :setIamPolicy)

## New Function Contract
- GET getIamPolicy with `?options.requestedPolicyVersion=3` query parameter (Secret Manager proto specifies GET; inline code already uses GET)
- Require HTTP 200 (secrets always have policies once created per RBSCIG; non-200 is real error) via rbgu_http_require_ok
- Extract etag explicitly via rbgu_json_field_capture + assert non-empty (current inline code passes empty string to compose at lines 946, 966, 986 — change to explicit extraction)
- Compose binding with etag for concurrency-safe write via rbgu_jq_add_member_to_role_capture
- POST setIamPolicy with {"policy":...} envelope
- Inner transient retry loop on SET with case statement: 200 (success), 409 ABORTED (fatal, etag mismatch), 429/5xx (transient retry), default (rbgu_http_require_ok)
- Outer propagation retry per RBSCIP backoff profile (3s/2x/20s/420s) — NOTE: this is a BEHAVIORAL ADDITION, not just extraction. The inline code has zero propagation retry. Adding it aligns Secret Manager grants with the other 4 grant functions and handles the member-SA propagation failure class that the preceding rbgu_poll_until_ok does NOT cover.

## Call Site Replacement

Lines 936-994 of rbgp_Payor.sh (three blocks) become three calls:
```
rbgi_grant_secret_iam "${z_token}" "${z_api_secret_resource}" \
  "serviceAccount:${z_cb_service_agent}" "roles/secretmanager.secretAccessor" \
  "depot_secret_iam_api"
```
(similarly for read and webhook, with appropriate resource path and infix)

Lines 920-932 (buc_step + rbgu_poll_until_ok calls) remain UNTOUCHED.

## Spec and Doc Updates
- Add rbtoe_iam_grant_secret mapping + definition in RBS0 (following corrected contract pattern; reference RBSCIG)
- Update RBSDC-depot_create.adoc lines 220-229 to use {rbtoe_iam_grant_secret} linked term instead of inline API call description
- Update RBSCIG-IamGrantContracts.adoc: change "inline in depot_create" references to reference new function; update HTTP Method table and other tables to show Secret Manager as a first-class grant pattern
- Add ZRBGI_INFIX_SECRET_IAM and ZRBGI_INFIX_SECRET_IAM_SET constants to zrbgi_kindle

## Files
- Tools/rbk/rbgi_IAM.sh — add rbgi_grant_secret_iam + kindle constants
- Tools/rbk/rbgp_Payor.sh — replace 3x inline blocks (lines 936-994) with 3 calls; preserve poll calls (lines 920-932)
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — add rbtoe_iam_grant_secret mapping + definition
- Tools/rbk/vov_veiled/RBSDC-depot_create.adoc — use new linked term
- Tools/rbk/vov_veiled/RBSCIG-IamGrantContracts.adoc — promote Secret Manager from inline to first-class grant pattern

## Risk Assessment

Low-medium. The inline code works in production but lacks protections the other grant functions have.

- Extraction itself (low risk): Mechanical replacement of 3x copy-paste with 3 function calls. Same HTTP methods, same URLs, same body format.
- Etag addition (low risk): Same rationale as AsAAF — converts accidental passthrough to intentional concurrency safety.
- Propagation retry addition (medium risk): Behavioral change — operations that previously died immediately on member-SA propagation error will now retry for up to 420s. This is correct behavior (matches the other 4 grant functions) but changes timing. The preceding rbgu_poll_until_ok should have already resolved resource propagation, so member-SA propagation errors should be rare.
- Transient retry addition (zero risk): Same as AsAAF.
- requestedPolicyVersion=3 (zero risk): Same as AsAAF.

**[260311-1111] rough**

## Goal

Extract the 3x copy-pasted Secret Manager IAM grants from rbgp_Payor.sh (lines 936-994) into a new rbgi_grant_secret_iam function in rbgi_IAM.sh. The new function follows the corrected contract pattern from the start: explicit etag, correct error handling, propagation retry.

## Current State

Three inline blocks in rbgp_depot_create grant roles/secretmanager.secretAccessor to the Cloud Build service agent on three secrets (api token, read token, webhook). Each block does getIamPolicy (GET) -> modify -> setIamPolicy (POST) with no etag passed to compose (accidental passthrough via jq), no propagation retry, and no transient retry. The inline code already uses GET correctly for getIamPolicy (line 938).

Preceding the inline grants, three rbgu_poll_until_ok calls (lines 923-932) poll each secret's getIamPolicy endpoint until 200. These handle RESOURCE propagation (secret not yet visible after creation). They are a different failure class from MEMBER-SA propagation (SA not yet usable in setIamPolicy) and MUST be preserved as-is in rbgp_Payor.sh. Do NOT fold them into rbgi_grant_secret_iam.

## Function Signature

rbgi_grant_secret_iam token secret_resource_path member role parent_infix

Where:
- token: OAuth bearer token
- secret_resource_path: e.g. projects/PROJECT_ID/secrets/SECRET_NAME
- member: e.g. serviceAccount:email@... (caller adds prefix)
- role: e.g. roles/secretmanager.secretAccessor
- parent_infix: temp file infix for HTTP response tracking

## New Function Contract
- GET getIamPolicy (Secret Manager proto specifies GET; inline code already uses GET)
- Require HTTP 200 (secrets always have policies once created per RBSCIG; non-200 is real error)
- Extract etag explicitly via rbgu_json_field_capture + assert non-empty (current inline code passes empty string to compose at lines 946, 966, 986 — change to explicit extraction)
- Compose binding with etag for concurrency-safe write
- POST setIamPolicy with {"policy":...} envelope
- 409 ABORTED fatal (etag mismatch)
- 429/5xx transient retry on SET with inner loop
- Propagation retry per RBSCIP backoff profile (3s/2x/20s/420s) — NOTE: this is a BEHAVIORAL ADDITION, not just extraction. The inline code has zero propagation retry. Adding it aligns Secret Manager grants with the other 4 grant functions and handles the member-SA propagation failure class that the preceding rbgu_poll_until_ok does NOT cover.
- Add requestedPolicyVersion=3 as query parameter on GET URL

## Spec and Doc Updates
- Add rbtoe_iam_grant_secret mapping + definition in RBS0 (following corrected contract pattern; reference RBSCIG)
- Update RBSDC-depot_create.adoc lines 220-229 to use {rbtoe_iam_grant_secret} linked term instead of inline API call description
- Update RBSCIG-IamGrantContracts.adoc: change "inline in depot_create" references to reference new function; update HTTP Method table and other tables to show Secret Manager as a first-class grant pattern
- Add ZRBGI_INFIX_SECRET_IAM and ZRBGI_INFIX_SECRET_IAM_SET constants to zrbgi_kindle

## Files
- Tools/rbk/rbgi_IAM.sh — add rbgi_grant_secret_iam + kindle constants
- Tools/rbk/rbgp_Payor.sh — replace 3x inline blocks (lines 936-994) with 3 calls to rbgi_grant_secret_iam; preserve rbgu_poll_until_ok calls (lines 923-932)
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — add rbtoe_iam_grant_secret mapping + definition
- Tools/rbk/vov_veiled/RBSDC-depot_create.adoc — use new linked term
- Tools/rbk/vov_veiled/RBSCIG-IamGrantContracts.adoc — promote Secret Manager from inline to first-class grant pattern

## Risk Assessment

Low-medium. The inline code works in production but lacks protections the other grant functions have.

- Extraction itself (low risk): Mechanical replacement of 3x copy-paste with 3 function calls. Same HTTP methods, same URLs, same body format.
- Etag addition (low risk): Same rationale as AsAAF — converts accidental passthrough to intentional concurrency safety.
- Propagation retry addition (medium risk): Behavioral change — operations that previously died immediately on member-SA propagation error will now retry for up to 420s. This is correct behavior (matches the other 4 grant functions) but changes timing. The preceding rbgu_poll_until_ok should have already resolved resource propagation, so member-SA propagation errors should be rare.
- Transient retry addition (zero risk): Same as AsAAF.

**[260311-1053] rough**

## Goal

Extract the 3x copy-pasted Secret Manager IAM grants from rbgp_Payor.sh (lines 936-994) into a new rbgi_grant_secret_iam function in rbgi_IAM.sh. The new function follows the corrected contract pattern from the start: explicit etag, correct error handling, propagation retry.

## Current State

Three inline blocks in rbgp_depot_create grant roles/secretmanager.secretAccessor to the Cloud Build service agent on three secrets (api token, read token, webhook). Each block does getIamPolicy (GET) → modify → setIamPolicy (POST) with no etag, no propagation retry, and no transient retry.

## New Function Contract
- GET getIamPolicy (Secret Manager proto specifies GET)
- Require HTTP 200 (secrets always have policies once created; non-200 is real error)
- Extract and pass etag (API always returns it)
- Compose binding with etag for concurrency-safe write
- POST setIamPolicy with {"policy":...} envelope
- 409 ABORTED fatal (etag mismatch)
- 429/5xx transient retry on SET
- Propagation retry per RBSCIP backoff profile
- Add rbtoe_iam_grant_secret mapping/definition in RBS0
- Update RBSDC-depot_create.adoc to use the new linked term

## Files
- Tools/rbk/rbgi_IAM.sh — add rbgi_grant_secret_iam + kindle constants
- Tools/rbk/rbgp_Payor.sh — replace 3x inline blocks with function calls
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — add rbtoe_iam_grant_secret definition
- Tools/rbk/vov_veiled/RBSDC-depot_create.adoc — use new linked term
- Tools/rbk/vov_veiled/RBSCIG-IamGrantContracts.adoc — update to reference new function

### adaptive-getting-started-guide (₢AsAAI) [complete]

**[260311-1606] complete**

## Goal

Create the adaptive onboarding guide (rbw-PO.PayorOnboarding.sh) that reads current regime state and displays the right next step for a new user.

## Tabtarget
rbw-PO.PayorOnboarding.sh (capital O: guides users through impactful infrastructure changes)

## Deliverables

rbgm_payor_onboarding() in rbgm_ManualProcedures.sh, wired to rbw-PO tabtarget. Reads regime state, shows progress status, emits next steps. Does NOT require enforced RBRR.

## Key UX: status summary
Guide opens with a status dashboard showing all phases with checkmark/blank indicators. User sees where they are in the journey before the guide dives into the next incomplete step. This is the core differentiator from existing single-purpose guides.

## Rendering convention
Use bug_* functions (bug_section, bug_t, bug_tc, bug_tW, buc_tabtarget) following the newer guide pattern (see rbgm_payor_establish, rbgm_gitlab_setup). Not the older zrbgm_* helpers.

## Pre-kindle probing
Guide must work WITHOUT kindled regime. Probes read raw files and env vars directly (test -f, source-and-check), not regime constants that require kindle. The whole point is guiding users who don't yet have a valid regime.

## State probes (file/variable checks, no API calls):
- rbrp.env exists? RBRP_PAYOR_PROJECT_ID set?
- rbro-payor.env exists? (OAuth complete?)
- RBRR_RUBRIC_REPO_URL set? (GitLab done?)
- Depot exists? (RBRR_DEPOT_PROJECT_ID set?)
- Governor RBRA exists?
- Director/retriever RBRAs exist?
- Vessels conjured? Vouched?

## Phases guided
Payor establish, payor install, gitlab setup, depot create, governor reset, director/retriever create, conjure, vouch, start bottle, tour.

## Workbench routing
Needs a command name enrolled in rbz_zipper that maps to rbgm_payor_onboarding. Determine the right command string during implementation.

## Design constraints
- Existing guides (rbgm_payor_establish, rbgm_gitlab_setup) keep integrity; onboarding points to them via buc_tabtarget
- Each probe is simple file/variable check
- Lives in rbgm for now
- College try: iteration expected in follow-up pace (AsAAC)

## Files
- Tools/rbk/rbgm_ManualProcedures.sh
- Tools/rbk/rbz_zipper.sh (enroll colophon)
- tt/rbw-PO.PayorOnboarding.sh

**[260311-1535] rough**

## Goal

Create the adaptive onboarding guide (rbw-PO.PayorOnboarding.sh) that reads current regime state and displays the right next step for a new user.

## Tabtarget
rbw-PO.PayorOnboarding.sh (capital O: guides users through impactful infrastructure changes)

## Deliverables

rbgm_payor_onboarding() in rbgm_ManualProcedures.sh, wired to rbw-PO tabtarget. Reads regime state, shows progress status, emits next steps. Does NOT require enforced RBRR.

## Key UX: status summary
Guide opens with a status dashboard showing all phases with checkmark/blank indicators. User sees where they are in the journey before the guide dives into the next incomplete step. This is the core differentiator from existing single-purpose guides.

## Rendering convention
Use bug_* functions (bug_section, bug_t, bug_tc, bug_tW, buc_tabtarget) following the newer guide pattern (see rbgm_payor_establish, rbgm_gitlab_setup). Not the older zrbgm_* helpers.

## Pre-kindle probing
Guide must work WITHOUT kindled regime. Probes read raw files and env vars directly (test -f, source-and-check), not regime constants that require kindle. The whole point is guiding users who don't yet have a valid regime.

## State probes (file/variable checks, no API calls):
- rbrp.env exists? RBRP_PAYOR_PROJECT_ID set?
- rbro-payor.env exists? (OAuth complete?)
- RBRR_RUBRIC_REPO_URL set? (GitLab done?)
- Depot exists? (RBRR_DEPOT_PROJECT_ID set?)
- Governor RBRA exists?
- Director/retriever RBRAs exist?
- Vessels conjured? Vouched?

## Phases guided
Payor establish, payor install, gitlab setup, depot create, governor reset, director/retriever create, conjure, vouch, start bottle, tour.

## Workbench routing
Needs a command name enrolled in rbz_zipper that maps to rbgm_payor_onboarding. Determine the right command string during implementation.

## Design constraints
- Existing guides (rbgm_payor_establish, rbgm_gitlab_setup) keep integrity; onboarding points to them via buc_tabtarget
- Each probe is simple file/variable check
- Lives in rbgm for now
- College try: iteration expected in follow-up pace (AsAAC)

## Files
- Tools/rbk/rbgm_ManualProcedures.sh
- Tools/rbk/rbz_zipper.sh (enroll colophon)
- tt/rbw-PO.PayorOnboarding.sh

**[260311-1530] rough**

## Goal

Create the adaptive onboarding guide (rbw-PO.PayorOnboarding.sh) that reads current regime state and displays the right next step for a new user.

## Tabtarget
rbw-PO.PayorOnboarding.sh (capital O: this guides users through impactful infrastructure changes)

## Deliverables

rbgm_payor_onboarding() in rbgm_ManualProcedures.sh, wired to rbw-PO tabtarget. Reads regime state, shows progress status, emits next steps. Does NOT require enforced RBRR.

State probes (file/variable checks, no API calls):
- rbrp.env exists? RBRP_PAYOR_PROJECT_ID set?
- rbro-payor.env exists? (OAuth complete?)
- RBRR_RUBRIC_REPO_URL set? (GitLab done?)
- Depot exists? (RBRR_DEPOT_PROJECT_ID set?)
- Governor RBRA exists?
- Director/retriever RBRAs exist?
- Vessels conjured? Vouched?

Phases guided: payor establish, payor install, gitlab setup, depot create, governor reset, director/retriever create, conjure, vouch, start bottle, tour.

## Design constraints
- Existing guides (rbgm_payor_establish, rbgm_gitlab_setup) keep integrity; onboarding points to them via buc_tabtarget
- Each probe is simple file/variable check
- Lives in rbgm for now
- College try: iteration expected in follow-up pace

## Files
- Tools/rbk/rbgm_ManualProcedures.sh
- Tools/rbk/rbz_zipper.sh (enroll colophon)
- tt/rbw-PO.PayorOnboarding.sh

**[260311-1516] rough**

## Goal

Create the adaptive onboarding guide (rbw-Po.PayorOnboarding.sh) that reads current RBRR/regime state and displays the right next step for a new user.

## Deliverables

rbgm_payor_onboarding() in rbgm_ManualProcedures.sh, wired to rbw-Po tabtarget. Reads regime state, shows progress status, emits next steps. Does NOT require enforced RBRR.

State probes (file/variable checks, no API calls):
- rbrp.env exists? RBRP_PAYOR_PROJECT_ID set?
- rbro-payor.env exists? (OAuth complete?)
- RBRR_RUBRIC_REPO_URL set? (GitLab done?)
- Depot exists? (RBRR_DEPOT_PROJECT_ID set?)
- Governor RBRA exists?
- Director/retriever RBRAs exist?
- Vessels conjured? Vouched?

Phases guided: payor establish, payor install, gitlab setup, depot create, governor reset, director/retriever create, conjure, vouch, start bottle, tour.

## Design constraints
- Existing guides keep integrity; onboarding points to them
- Each probe is simple file/variable check
- Lives in rbgm for now
- College try: iteration expected in follow-up pace
- RBRR reset (rbw-PO) is separate release-prep concern on AU

## Files
- Tools/rbk/rbgm_ManualProcedures.sh
- Tools/rbk/rbz_zipper.sh (enroll colophon)
- tt/rbw-Po.PayorOnboarding.sh

**[260311-1452] rough**

## Goal

Create adaptive getting-started guide (rbgm_getting_started) and RBRR reset tabtarget forming the new-user onboarding funnel.

## Deliverables

### 1. RBRR reset mechanism
Tabtarget producing valid-but-incomplete RBRR: every field present with empty/sentinel values. Guide can probe without set -u failures; real operations correctly reject.

### 2. Adaptive getting-started guide
rbgm_getting_started() in rbgm_ManualProcedures.sh, wired to tabtarget. Reads RBRR state, shows progress status, emits next steps. Does NOT require enforced RBRR.

Phases detected and guided:
- Payor establish (points to existing rbgm_payor_establish)
- Payor install (OAuth handshake)
- GitLab rubric repo setup (points to existing rbgm_gitlab_setup)
- Depot create
- Governor reset + credential distribution
- Director/retriever create
- Conjure vessels (nsproto images)
- Vouch arks
- Start bottle, run test case
- Tour: show vessels, images, what you built

## Design constraints
- Existing guides keep current integrity; guide points to them, does not rewrite
- Each probe is simple file/variable check, no API calls for state detection
- Guide and reset co-designed: reset defines starting state, guide defines progress
- Lives in rbgm for now

## Files
- Tools/rbk/rbgm_ManualProcedures.sh
- tt/ (new tabtargets for guide + reset)
- Regime template files as needed

### release-rbrr-reset (₢AsAAJ) [complete]

**[260311-1640] complete**

## Goal

Create rbw-MR.MarshalReset.sh that resets regime to a blank-but-valid template suitable for shipping in a release.

## New Role: Marshal

Marshal is a new formal role in RBS0 (rbtr_marshal). The Marshal qualifies and prepares releases. Unlike other roles, the Marshal has no GCP cloud identity. The role taxonomy is about operational authority, not authentication type. Marshal operations are invisible to customers.

RBS0 treatment is lightweight for now: linked term + definition in the role section. No operation group section yet (grows with future Marshal operations).

## Tabtarget
rbw-MR.MarshalReset.sh (M = Marshal colophon prefix, uppercase: impactful regime change)

## Scope: which regime files?

RBRR (repo regime) is the primary target: blank all fields so onboarding guide can walk users through populating them. Payor regime (rbrp) is the user's own project config and probably should NOT be blanked. Clarify scope during implementation: rbrr only, or also rbrn/rbrv vessel and nameplate configs?

## Credential files

Reset clears regime config files but RBRA credential files in secrets dir are a separate concern. The reset should NOT touch credential files. Document this boundary clearly.

## Deliverables

1. Add rbtr_marshal linked term and definition to RBS0-SpecTop.adoc
2. Determine blank RBRR shape: every field present with empty/sentinel values so set -u survives and rbw-PO can probe, but real operations correctly reject
3. Implement reset function and wire to rbw-MR tabtarget
4. Exercise: run rbw-MR then rbw-PO and verify the guide correctly detects blank state and emits first steps

## Design constraints
- Part of release qualification sequence
- Must be exercised before every release to verify onboarding guide works from blank
- Marshal operations use M colophon prefix in tabtarget namespace

## Files
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc (Marshal role definition)
- Tools/rbk/rbgm_ManualProcedures.sh or new marshal module
- Tools/rbk/rbz_zipper.sh (enroll colophon)
- tt/rbw-MR.MarshalReset.sh

**[260311-1535] rough**

## Goal

Create rbw-MR.MarshalReset.sh that resets regime to a blank-but-valid template suitable for shipping in a release.

## New Role: Marshal

Marshal is a new formal role in RBS0 (rbtr_marshal). The Marshal qualifies and prepares releases. Unlike other roles, the Marshal has no GCP cloud identity. The role taxonomy is about operational authority, not authentication type. Marshal operations are invisible to customers.

RBS0 treatment is lightweight for now: linked term + definition in the role section. No operation group section yet (grows with future Marshal operations).

## Tabtarget
rbw-MR.MarshalReset.sh (M = Marshal colophon prefix, uppercase: impactful regime change)

## Scope: which regime files?

RBRR (repo regime) is the primary target: blank all fields so onboarding guide can walk users through populating them. Payor regime (rbrp) is the user's own project config and probably should NOT be blanked. Clarify scope during implementation: rbrr only, or also rbrn/rbrv vessel and nameplate configs?

## Credential files

Reset clears regime config files but RBRA credential files in secrets dir are a separate concern. The reset should NOT touch credential files. Document this boundary clearly.

## Deliverables

1. Add rbtr_marshal linked term and definition to RBS0-SpecTop.adoc
2. Determine blank RBRR shape: every field present with empty/sentinel values so set -u survives and rbw-PO can probe, but real operations correctly reject
3. Implement reset function and wire to rbw-MR tabtarget
4. Exercise: run rbw-MR then rbw-PO and verify the guide correctly detects blank state and emits first steps

## Design constraints
- Part of release qualification sequence
- Must be exercised before every release to verify onboarding guide works from blank
- Marshal operations use M colophon prefix in tabtarget namespace

## Files
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc (Marshal role definition)
- Tools/rbk/rbgm_ManualProcedures.sh or new marshal module
- Tools/rbk/rbz_zipper.sh (enroll colophon)
- tt/rbw-MR.MarshalReset.sh

**[260311-1531] rough**

## Goal

Create rbw-MR.MarshalReset.sh that resets RBRR to a blank-but-valid template suitable for shipping in a release.

## New Role: Marshal

Marshal is a new formal role in RBS0 (rbtr_marshal). The Marshal qualifies and prepares releases. Unlike other roles, the Marshal has no GCP cloud identity — this is a development-side role, invisible to customers. The role taxonomy is about operational authority, not authentication type.

This pace adds the rbtr_marshal linked term and definition to RBS0.

## Tabtarget
rbw-MR.MarshalReset.sh (M = Marshal colophon prefix, uppercase: impactful regime change)

## Deliverables

1. Add rbtr_marshal linked term and definition to RBS0-SpecTop.adoc
2. Determine what a blank RBRR looks like: every field present with empty/sentinel values so set -u survives and the onboarding guide (rbw-PO) can probe, but real operations correctly reject
3. Implement reset function and wire to rbw-MR tabtarget
4. Exercise: run rbw-MR then rbw-PO and verify the guide correctly detects blank state and emits first steps

## Design constraints
- Part of release qualification sequence
- Must be exercised before every release to verify onboarding guide works from blank
- Marshal operations use M colophon prefix in tabtarget namespace

## Files
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc (Marshal role definition)
- Tools/rbk/rbgm_ManualProcedures.sh or new marshal module
- Tools/rbk/rbz_zipper.sh (enroll colophon)
- tt/rbw-MR.MarshalReset.sh

**[260311-1516] rough**

Drafted from ₢AUAAM in ₣AU.

## Goal

Create rbw-PO.PayorOverwritesRegime.sh (capital O, impactful) that resets RBRR to a blank-but-valid template suitable for shipping in a release.

## Context

Every Recipe Bottle release must ship with an RBRR that new users can start from. The onboarding guide (rbw-Po, on heat As) reads this RBRR and walks users forward. This pace produces the blank canvas that the guide consumes.

## Deliverables

1. Determine what a blank RBRR looks like: every field present with empty or sentinel values so set -u survives and the onboarding guide can probe, but real operations correctly reject.
2. rbw-PO tabtarget that overwrites rbrr.env with the blank template.
3. Exercise: run rbw-PO then rbw-Po and verify the guide correctly detects blank state and emits first steps.

## Design constraints
- Capital O: this changes regime state, intentionally destructive
- Part of release qualification sequence (relates to AUAAI release-qualification-gate)
- Must be exercised before every release to verify guide works from blank

## Files
- Tools/rbk/rbgp_Payor.sh or rbgm_ManualProcedures.sh (reset function)
- Tools/rbk/rbz_zipper.sh (enroll colophon)
- tt/rbw-PO.PayorOverwritesRegime.sh

**[260311-1516] rough**

## Goal

Create rbw-PO.PayorOverwritesRegime.sh (capital O, impactful) that resets RBRR to a blank-but-valid template suitable for shipping in a release.

## Context

Every Recipe Bottle release must ship with an RBRR that new users can start from. The onboarding guide (rbw-Po, on heat As) reads this RBRR and walks users forward. This pace produces the blank canvas that the guide consumes.

## Deliverables

1. Determine what a blank RBRR looks like: every field present with empty or sentinel values so set -u survives and the onboarding guide can probe, but real operations correctly reject.
2. rbw-PO tabtarget that overwrites rbrr.env with the blank template.
3. Exercise: run rbw-PO then rbw-Po and verify the guide correctly detects blank state and emits first steps.

## Design constraints
- Capital O: this changes regime state, intentionally destructive
- Part of release qualification sequence (relates to AUAAI release-qualification-gate)
- Must be exercised before every release to verify guide works from blank

## Files
- Tools/rbk/rbgp_Payor.sh or rbgm_ManualProcedures.sh (reset function)
- Tools/rbk/rbz_zipper.sh (enroll colophon)
- tt/rbw-PO.PayorOverwritesRegime.sh

### sa-key-propagation-retry (₢AsAAK) [complete]

**[260312-1631] complete**

## Goal

Add RBSCIP-compliant propagation retry to SA key generation in zrbgg_create_service_account_with_key.

## Diagnosis

SA creation returns 200 with uid. The existing rbgu_poll_until_ok confirms SA is readable via GET. But POST to /keys returns 404 because the keys sub-resource endpoint has additional propagation delay. Google documents: "you might need to wait 60 seconds or more before you use the service account."

This is a pre-existing latent race (Governor.sh unchanged by heat), surfaced during fresh depot onboarding walkthrough on 2026-03-12.

## Required Context

- RBSCIP-IamPropagation.adoc — backoff profile (3s initial, 2x multiplier, 20s cap, 420s deadline), pattern precedent
- BCG-BashConsoleGuide.md — enterprise retry patterns, temp file discipline, load-bearing complexity
- RBS0-SpecTop.adoc — spec definition for zrbgg_create_service_account_with_key if exists
- Tools/rbk/rbgi_IAM.sh — existing propagation retry loops in IAM grant functions (pattern to follow)
- Tools/rbk/rbgu_Utility.sh — rbgu_poll_until_ok and rbgu_http_json (understand their contracts)
- Tools/rbk/rbgg_Governor.sh — the key generation step at line 249-254

## Design Questions

1. Should rbgu_poll_until_ok be enhanced to support POST-with-retry, or does key generation need its own retry in rbgg?
2. The RBSCIP backoff profile uses exponential backoff — should key retry use the same profile or is flat interval sufficient given the 404 is a different failure class (resource propagation, not IAM grant propagation)?
3. Should the retry be specific to 404 only, or also handle 503/429?

## Files
- Tools/rbk/rbgg_Governor.sh
- Tools/rbk/rbgu_Utility.sh (if poll_until_ok is enhanced)
- Tools/rbk/vov_veiled/RBSCIP-IamPropagation.adoc (reference)
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc (spec update if needed)

**[260312-1553] rough**

## Goal

Add RBSCIP-compliant propagation retry to SA key generation in zrbgg_create_service_account_with_key.

## Diagnosis

SA creation returns 200 with uid. The existing rbgu_poll_until_ok confirms SA is readable via GET. But POST to /keys returns 404 because the keys sub-resource endpoint has additional propagation delay. Google documents: "you might need to wait 60 seconds or more before you use the service account."

This is a pre-existing latent race (Governor.sh unchanged by heat), surfaced during fresh depot onboarding walkthrough on 2026-03-12.

## Required Context

- RBSCIP-IamPropagation.adoc — backoff profile (3s initial, 2x multiplier, 20s cap, 420s deadline), pattern precedent
- BCG-BashConsoleGuide.md — enterprise retry patterns, temp file discipline, load-bearing complexity
- RBS0-SpecTop.adoc — spec definition for zrbgg_create_service_account_with_key if exists
- Tools/rbk/rbgi_IAM.sh — existing propagation retry loops in IAM grant functions (pattern to follow)
- Tools/rbk/rbgu_Utility.sh — rbgu_poll_until_ok and rbgu_http_json (understand their contracts)
- Tools/rbk/rbgg_Governor.sh — the key generation step at line 249-254

## Design Questions

1. Should rbgu_poll_until_ok be enhanced to support POST-with-retry, or does key generation need its own retry in rbgg?
2. The RBSCIP backoff profile uses exponential backoff — should key retry use the same profile or is flat interval sufficient given the 404 is a different failure class (resource propagation, not IAM grant propagation)?
3. Should the retry be specific to 404 only, or also handle 503/429?

## Files
- Tools/rbk/rbgg_Governor.sh
- Tools/rbk/rbgu_Utility.sh (if poll_until_ok is enhanced)
- Tools/rbk/vov_veiled/RBSCIP-IamPropagation.adoc (reference)
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc (spec update if needed)

### fresh-depot-conjure-vouch (₢AsAAC) [complete]

**[260312-1852] complete**

## Goal

Walk through the full onboarding sequence using rbw-PO and rbw-MR, exercising every step from blank RBRR through working bottle. Fix whatever the college-try paces (AsAAI, AsAAJ) got wrong.

## Process

1. Run rbw-MR to reset RBRR to blank
2. Run rbw-PO, follow its guidance step by step
3. At each step: does the guide emit the right next action? Does the tabtarget work? Does the state probe correctly advance?
4. Fix guide text, probe logic, and regime template issues as encountered
5. Continue through: payor establish, payor install, gitlab setup, depot create, governor reset, director/retriever create, conjure, vouch, start bottle
6. This exercises every IAM grant path on freshly-provisioned resources (the original acid test goal)

## Specific validation targets from AsAAI implementation

- Pre-kindle CLI dispatch chain: does rbw-PO survive the workbench healthcheck, exec to rbgm_onboard_cli.sh, and furnish without regime?
- grep probes: do `^KEY=.\+` patterns match actual env file formats after each operation? Watch for quoted values, trailing comments, or multi-line values that grep misses.
- `zrbgm_po_extract_capture` for RBRR_SECRETS_DIR: the value is a relative path (`../station-files/secrets`). Does it resolve correctly from CWD (project root) when used in `test -f` probes?
- Lightweight zipper kindle: does `buc_tabtarget` resolve all 10 referenced colophons (RBZ_PAYOR_ESTABLISH through RBZ_BOTTLE_START) from the pre-kindle CLI context?
- Dashboard advancement: after each real operation, does re-running rbw-PO correctly flip the corresponding `[*]` marker?
- Phase 9 (Start & Tour) is hardcoded `[ ]` — no file-based probe exists. Decide if this matters or is acceptable.

## Cross-pace file touches
This pace may touch files from AsAAI and AsAAJ in the same commits. That is expected and correct: this is the integration/polish pass.

## Acceptance

A new user with a blank RBRR can follow rbw-PO from start to running bottle with no undocumented steps.

## Files
- Tools/rbk/rbgm_ManualProcedures.sh
- Tools/rbk/rbgm_onboard_cli.sh
- Tools/rbk/rbz_zipper.sh
- tt/rbw-PO.PayorOnboarding.sh
- Plus anything discovered during walkthrough

**[260311-1606] rough**

## Goal

Walk through the full onboarding sequence using rbw-PO and rbw-MR, exercising every step from blank RBRR through working bottle. Fix whatever the college-try paces (AsAAI, AsAAJ) got wrong.

## Process

1. Run rbw-MR to reset RBRR to blank
2. Run rbw-PO, follow its guidance step by step
3. At each step: does the guide emit the right next action? Does the tabtarget work? Does the state probe correctly advance?
4. Fix guide text, probe logic, and regime template issues as encountered
5. Continue through: payor establish, payor install, gitlab setup, depot create, governor reset, director/retriever create, conjure, vouch, start bottle
6. This exercises every IAM grant path on freshly-provisioned resources (the original acid test goal)

## Specific validation targets from AsAAI implementation

- Pre-kindle CLI dispatch chain: does rbw-PO survive the workbench healthcheck, exec to rbgm_onboard_cli.sh, and furnish without regime?
- grep probes: do `^KEY=.\+` patterns match actual env file formats after each operation? Watch for quoted values, trailing comments, or multi-line values that grep misses.
- `zrbgm_po_extract_capture` for RBRR_SECRETS_DIR: the value is a relative path (`../station-files/secrets`). Does it resolve correctly from CWD (project root) when used in `test -f` probes?
- Lightweight zipper kindle: does `buc_tabtarget` resolve all 10 referenced colophons (RBZ_PAYOR_ESTABLISH through RBZ_BOTTLE_START) from the pre-kindle CLI context?
- Dashboard advancement: after each real operation, does re-running rbw-PO correctly flip the corresponding `[*]` marker?
- Phase 9 (Start & Tour) is hardcoded `[ ]` — no file-based probe exists. Decide if this matters or is acceptable.

## Cross-pace file touches
This pace may touch files from AsAAI and AsAAJ in the same commits. That is expected and correct: this is the integration/polish pass.

## Acceptance

A new user with a blank RBRR can follow rbw-PO from start to running bottle with no undocumented steps.

## Files
- Tools/rbk/rbgm_ManualProcedures.sh
- Tools/rbk/rbgm_onboard_cli.sh
- Tools/rbk/rbz_zipper.sh
- tt/rbw-PO.PayorOnboarding.sh
- Plus anything discovered during walkthrough

**[260311-1535] rough**

## Goal

Walk through the full onboarding sequence using rbw-PO and rbw-MR, exercising every step from blank RBRR through working bottle. Fix whatever the college-try paces (AsAAI, AsAAJ) got wrong.

## Process

1. Run rbw-MR to reset RBRR to blank
2. Run rbw-PO, follow its guidance step by step
3. At each step: does the guide emit the right next action? Does the tabtarget work? Does the state probe correctly advance?
4. Fix guide text, probe logic, and regime template issues as encountered
5. Continue through: payor establish, payor install, gitlab setup, depot create, governor reset, director/retriever create, conjure, vouch, start bottle
6. This exercises every IAM grant path on freshly-provisioned resources (the original acid test goal)

## Cross-pace file touches
This pace may touch files from AsAAI and AsAAJ in the same commits. That is expected and correct: this is the integration/polish pass.

## Acceptance

A new user with a blank RBRR can follow rbw-PO from start to running bottle with no undocumented steps.

## Files
- Whatever AsAAI and AsAAJ touched, plus anything discovered during walkthrough

**[260311-1533] rough**

## Goal

Walk through the full onboarding sequence using rbw-PO and rbw-MR, exercising every step from blank RBRR through working bottle. Fix whatever the college-try paces (AsAAI, AsAAJ) got wrong.

## Process

1. Run rbw-MR to reset RBRR to blank
2. Run rbw-PO, follow its guidance step by step
3. At each step: does the guide emit the right next action? Does the tabtarget work? Does the state probe correctly advance?
4. Fix guide text, probe logic, and regime template issues as encountered
5. Continue through: payor establish, payor install, gitlab setup, depot create, governor reset, director/retriever create, conjure, vouch, start bottle
6. This exercises every IAM grant path on freshly-provisioned resources (the original acid test goal)

## Acceptance

A new user with a blank RBRR can follow rbw-PO from start to running bottle with no undocumented steps.

## Files
- Whatever AsAAI and AsAAJ touched, plus anything discovered during walkthrough

**[260309-1945] rough**

After IAM grant refactor lands: create a fresh depot, conjure all bottle service vessels, vouch all arks. This exercises every IAM grant path with real propagation delays on freshly-provisioned resources. The acid test for the unified grant contract.

### post-validation-test-sweep (₢AsAAD) [complete]

**[260313-1039] complete**

Final full test sweep after fresh depot + conjure + vouch succeeds. Confirms the entire system is green with the unified IAM grant contract under real-world conditions.

**[260309-1945] rough**

Final full test sweep after fresh depot + conjure + vouch succeeds. Confirms the entire system is green with the unified IAM grant contract under real-world conditions.

### rbgi-kindle-and-capture-cleanup (₢AsAAH) [complete]

**[260311-1421] complete**

## Goal

Clean up two minor inconsistencies in rbgi_IAM.sh discovered during AsAAF review.

## Changes

### 1. Homogenize z_get_code capture to die-on-failure

Project function (line ~155) dies on HTTP code capture failure: `|| buc_die "No HTTP code from getIamPolicy"`. Repo and SA outer GET loops fall back to empty string: `|| z_get_code=""`. The empty-string fallback works (propagation predicate rejects non-400, then rbgu_http_require_ok dies) but is an indirect path to the same crash. Change repo and SA to match project's die-on-failure pattern.

### 2. Remove dead kindle constants

`ZRBGI_INFIX_REPO_POLICY` and `ZRBGI_INFIX_RPOLICY_SET` in zrbgi_kindle appear unused — repo function uses `ZRBGI_INFIX_REPO_ROLE` and `ZRBGI_INFIX_REPO_ROLE_SET` instead. Verify no callers, then remove.

## Files
- Tools/rbk/rbgi_IAM.sh

## Risk Assessment
Zero. Capture-failure homogenization changes behavior only when temp files are missing (already fatal). Dead constant removal is pure cleanup.

**[260311-1129] rough**

## Goal

Clean up two minor inconsistencies in rbgi_IAM.sh discovered during AsAAF review.

## Changes

### 1. Homogenize z_get_code capture to die-on-failure

Project function (line ~155) dies on HTTP code capture failure: `|| buc_die "No HTTP code from getIamPolicy"`. Repo and SA outer GET loops fall back to empty string: `|| z_get_code=""`. The empty-string fallback works (propagation predicate rejects non-400, then rbgu_http_require_ok dies) but is an indirect path to the same crash. Change repo and SA to match project's die-on-failure pattern.

### 2. Remove dead kindle constants

`ZRBGI_INFIX_REPO_POLICY` and `ZRBGI_INFIX_RPOLICY_SET` in zrbgi_kindle appear unused — repo function uses `ZRBGI_INFIX_REPO_ROLE` and `ZRBGI_INFIX_REPO_ROLE_SET` instead. Verify no callers, then remove.

## Files
- Tools/rbk/rbgi_IAM.sh

## Risk Assessment
Zero. Capture-failure homogenization changes behavior only when temp files are missing (already fatal). Dead constant removal is pure cleanup.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 B pre-refactor-baseline-sweep
  2 A iam-grant-homogenization
  3 F rbgi-etag-and-error-compliance
  4 G rbgi-secret-manager-extraction
  5 I adaptive-getting-started-guide
  6 J release-rbrr-reset
  7 K sa-key-propagation-retry
  8 C fresh-depot-conjure-vouch
  9 D post-validation-test-sweep
  10 H rbgi-kindle-and-capture-cleanup

BAFGIJKCDH
··xx·····x rbgi_IAM.sh
·x·x···x·· RBS0-SpecTop.adoc
·x·x··x··· RBSCIG-IamGrantContracts.adoc
······xx·· rbgg_Governor.sh
····x··x·· rbgm_ManualProcedures.sh, rbgm_onboard_cli.sh, rbw-PO.PayorOnboarding.sh, rbz_zipper.sh
···x···x·· RBSDC-depot_create.adoc
···x··x··· rbgp_Payor.sh
········x· rbrn_pluml.env, rbrn_srjcl.env, rbtb_testbench.sh
·······x·· BCG-BashConsoleGuide.md, RBSCB-CloudBuildPosture.adoc, RBSPV-PodmanVmSupplyChain.adoc, RBSQB-quota_build.adoc, RBSRG-RegimeGcbPins.adoc, RBSRI-rubric_inscribe.adoc, RBSRR-RegimeRepo.adoc, bute_engine.sh, rbdc_DerivedConstants.sh, rbgm_cli.sh, rbgp_cli.sh, rbob_bottle.sh, rbrg.env, rbrm.env, rbrm_regime.sh, rbrr.env, rbrr_cli.sh, rbrr_regime.sh, rbrr_reset_cli.sh, rbss.sentry.sh, rbv_PodmanVM.sh, rbw-DS.DirectorSummonsArk.sh, rbw-PI.PayorInstallation.sh, rbw-PR.PayorRefreshesOAuth.sh, rbw-Pg.PayorEstablishmentGuide.sh, rbw-Pgl.PayorGitlabSetupGuide.sh, rbw-Rs.RetrieverSummonsArk.sh, rbw-gO.Onboarding.sh, rbw-gPE.PayorEstablish.sh, rbw-gPI.PayorInstall.sh, rbw-gPL.GitLabSetup.sh, rbw-gPR.PayorRefresh.sh, rbw-gPo.PayorOnboarding.sh, rbw-gq.QuotaBuild.sh, rbw-gqb.QuotaBuild.sh
······x··· RBSDI-director_create.adoc, RBSGR-governor_reset.adoc, RBSRC-retriever_create.adoc, rbgc_Constants.sh
·x········ CLAUDE.md, RBSCIP-IamPropagation.adoc

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 106 commits)

  1 K sa-key-propagation-retry
  2 C fresh-depot-conjure-vouch
  3 D post-validation-test-sweep

123456789abcdefghijklmnopqrstuvwxyz
···xxxx····························  K  4c
·······xxxxxxxxxxxxxxxxxxxxxxxxxx··  C  26c
·································xx  D  2c
```

## Steeplechase

### 2026-03-13 10:39 - ₢AsAAD - W

Fixed missing zrbgo_kindle in testbench rbtb_load_nameplate — auto-summon vouch gate requires OAuth. Updated srjcl and pluml nameplates to current vouched consecrations. All 10 fixtures in complete test suite now pass.

### 2026-03-13 10:38 - ₢AsAAD - n

Fixed missing zrbgo_kindle in testbench rbtb_load_nameplate (auto-summon vouch gate requires OAuth). Updated srjcl and pluml nameplates to current vouched consecrations.

### 2026-03-12 18:52 - ₢AsAAC - W

Full onboarding walkthrough from blank RBRR to running bottle — all steps validated end-to-end. Fixed guide structure (role affiliations, 9-step dashboard, explanation-then-steps pattern), renamed guide rbw-gPo→rbw-gO and summon rbw-DS→rbw-Rs for correct role ownership, added nameplate update hint with fake consecration in guide colors, made test engine show per-case PASSED unconditionally, suppressed sentry setup and Docker CLI chatter to per-role log files in temp dir.

### 2026-03-12 18:48 - ₢AsAAC - n

Replaced all >/dev/null with per-role consolidated logs in BURD_TEMP_DIR. Container create logs stay separate in BURD_OUTPUT_DIR. Sentry setup log folded into sentry consolidated log. Nothing is discarded.

### 2026-03-12 18:46 - ₢AsAAC - n

Suppressed Docker CLI stdout chatter: container create/run output to kindle-constant log files in BURD_OUTPUT_DIR, cleanup and network commands to /dev/null, ARP flush to /dev/null

### 2026-03-12 18:39 - ₢AsAAC - n

Redirected sentry setup script stdout to ZRBOB_SENTRY_SETUP_LOG temp file — eliminates RBSp phase markers and dnsmasq banner chatter from test output

### 2026-03-12 18:34 - ₢AsAAC - n

Gated set -x behind RBSS_VERBOSE env var in sentry setup script — eliminates command trace noise during tests while keeping RBSp phase markers visible

### 2026-03-12 18:33 - ₢AsAAC - n

Made per-case PASSED printout unconditional in test engine — no longer gated behind BUT_VERBOSE

### 2026-03-12 18:29 - ₢AsAAC - n

Fixed nsproto-security test tabtarget in summon and start-bottle guidance — use imprint-based filename instead of broken RBZ_TEST_FIXTURE parameter form

### 2026-03-12 18:24 - ₢AsAAC - n

Added nsproto-security test suite as step 3 in Summon guidance

### 2026-03-12 18:23 - ₢AsAAC - n

Removed step 9 (Nameplate Update) from dashboard — it's inline guidance in case 7, not a separate step. Back to 9 dashboard steps with Summon as step 9.

### 2026-03-12 18:20 - ₢AsAAC - n

Split onboarding into 10 steps: added step 9 (Nameplate Update) with visual hint showing fake consecration values in guide colors, step 10 (Summon) now Retriever role. Steps 8+9 co-gate on rbrn probe; case 7 shows both sections.

### 2026-03-12 18:06 - ₢AsAAC - n

Fixed summon colophon rbw-DS→rbw-Rs (Retriever role, lowercase=non-mutating). Restructured onboarding: moved vouch into step 8 (Director), moved rbrn update+summon into step 9 (Retriever). Aligned role column for Retriever width.

### 2026-03-12 17:59 - ₢AsAAC - n

Renamed onboarding guide from rbw-gPo.PayorOnboarding to rbw-gO.Onboarding — guide spans Payor, Governor, and Director roles, not just Payor

### 2026-03-12 17:59 - ₢AsAAC - n

Sharpened Inscribe & Conjure guide section: consolidated explanation at top, numbered all steps including new step 7 (check consecrations) and step 8 (update rbrn). Added role affiliations (Payor/Governor/Director) to all 9 dashboard summary lines.

### 2026-03-12 17:50 - ₢AsAAC - n

Added time expectation (10-20 minutes) to conjure steps in onboarding guide

### 2026-03-12 17:37 - ₢AsAAC - n

Fixed git command lines in onboarding guide to render entirely in cyan (moved full command into colored argument)

### 2026-03-12 17:33 - ₢AsAAC - n

Added explanatory context for GCB pins (reproducible builds via locked digests) and inscribe (translates vessels to Cloud Build instructions, pushes to rubric repo) in onboarding guide

### 2026-03-12 17:32 - ₢AsAAC - n

Added GCB image pin refresh, binary pin refresh, and git commit steps before inscribe in onboarding guide — inscribe requires fresh committed pins

### 2026-03-12 17:31 - ₢AsAAC - n

Added rubric inscribe step before conjure in onboarding guide — inscribe pushes build definitions and creates Cloud Build triggers, which conjure requires

### 2026-03-12 17:27 - ₢AsAAC - n

Lowered RBRR_GCB_MIN_CONCURRENT_BUILDS default from 3 to 1 (fresh projects get 2 vCPU system limit). Rewrote quota guide to document that private pool CPU is a non-adjustable system limit due to Google anti-abuse policy — Console Edit Quotas does not work, only support ticket or Issue Tracker. Updated RBSQB spec and RBS0 NOTE with corrected metric name and increase path.

### 2026-03-12 17:11 - ₢AsAAC - n

Fixed conjure guidance to include RBRR_VESSEL_DIR prefix in vessel path (bare vessel name causes 'directory not found')

### 2026-03-12 17:10 - ₢AsAAC - n

Added tilde-in-quotes rule to BCG: narrative section (Expansion Requirements) and checklist (Quoting & Expansion)

### 2026-03-12 17:08 - ₢AsAAC - n

Fixed SC2088 shellcheck warning: replaced tilde with $HOME in payor install guidance string

### 2026-03-12 17:05 - ₢AsAAC - n

Marshal reset now shows full inventory of all deletions, amendments, and preserved files before buc_require confirmation prompt

### 2026-03-12 17:01 - ₢AsAAC - n

Added nsproto-security test fixture guidance to terminal onboarding phase after bottle start

### 2026-03-12 16:57 - ₢AsAAC - n

Split onboarding step 8 into Conjure (step 8) and Vouch & Summon (step 9). Conjure probes nsproto consecrations via source+test; Vouch & Summon probes local runtime for vouch images. Marshal reset blanks consecrations in all vessel nameplates. Conjure guidance directs both sentry and bottle vessels; vouch guidance directs batch vouch then summon with actual vessel/consecration values.

### 2026-03-12 16:50 - ₢AsAAC - n

Marshal reset now blanks consecrations in all vessel nameplates; step 8 probe changed from nameplate-exists to nsproto-consecrations-present (source+test, no grep)

### 2026-03-12 16:31 - ₢AsAAK - W

Added keys.create POST retry (7x, 10s apart) for SA write-path propagation delay — observed 404 on 60% of fresh SA creations despite read paths succeeding. Fixed inline director AR getIamPolicy from POST to GET per proto (was intermittently 404ing from transcoding rejection), added requestedPolicyVersion=3, removed 404 vanilla-init mask. Updated RBSCIG with live evidence, new propagation class finding, and resolved discrepancy. All three spec documents (RBSRC, RBSDI, RBSGR) updated with retry documentation and Google SA creation 60s propagation reference link.

### 2026-03-12 16:31 - ₢AsAAK - n

Increased SA key retry from 3 to 7 (70s total, covering Google's documented 60s+ propagation window). Added Google SA creation doc link to all three spec NOTEs.

### 2026-03-12 16:25 - ₢AsAAK - n

Fixed inline director AR getIamPolicy: POST→GET per proto (observed 404 from transcoding rejection), added requestedPolicyVersion=3, removed 404 vanilla-init mask, BCG-compliant loop variables. Updated RBSCIG with live evidence and resolved discrepancy.

### 2026-03-12 16:11 - ₢AsAAK - n

Added keys.create POST retry on 404 for SA write-path propagation delay — 3 attempts with 10s delay between, covering director/retriever (rbgg) and governor (rbgp) key generation paths. Documented new propagation class in RBSCIG and retry spec in RBSRC/RBSDI/RBSGR.

### 2026-03-12 15:56 - Heat - r

moved AsAAK to first

### 2026-03-12 15:53 - Heat - r

moved AsAAK after AsAAC

### 2026-03-12 15:53 - Heat - S

sa-key-propagation-retry

### 2026-03-12 15:48 - ₢AsAAC - n

Added propagation retry loop around SA key generation POST — pre-existing latent race where GET on SA succeeds but keys endpoint has additional propagation delay

### 2026-03-12 15:42 - ₢AsAAC - n

Added instance name placeholder to director and retriever next-step guidance in onboarding guide

### 2026-03-12 15:38 - ₢AsAAC - n

Restored rbrr.env depot configuration values after Marshal Reset test

### 2026-03-12 15:37 - ₢AsAAC - n

Marshal Reset now deletes depot-scoped RBRA files (governor/director/retriever) to prevent stale credentials fooling onboarding probes, added buc_require confirmation gate before destructive operations

### 2026-03-12 15:33 - ₢AsAAC - n

Depot depot10030 created successfully, RBRR populated with depot project ID, GAR repository, CB v2 connection, and worker pool

### 2026-03-12 15:30 - ₢AsAAC - n

Differential furnish for rbgp_depot_create: skip zrbrr_enforce since depot_create establishes the depot project ID and other fields that enforce requires

### 2026-03-12 15:23 - ₢AsAAC - n

Added 'Add new token' button click step to GitLab access token creation flow

### 2026-03-12 15:23 - ₢AsAAC - n

Set RBRR_RUBRIC_REPO_URL for onboarding, fixed hardcoded rbrr.env string to use RBBC_rbrr_file constant in GitLab setup guide

### 2026-03-12 15:14 - ₢AsAAC - n

Alignment cleanup of zipper enrollment columns and section grouping

### 2026-03-12 15:10 - ₢AsAAC - n

Merged rbgm_onboard_cli.sh into rbgm_cli.sh with differential furnish (RBRN pattern). Split zrbgm_kindle/enforce so guide commands work pre-regime. Fixes rbw-gPL crash when RBRR_DEPOT_PROJECT_ID is blank.

### 2026-03-12 15:00 - ₢AsAAC - n

Expanded GitLab Setup next-step guidance: explains rubric repo as security boundary, inscribe generation flow, and GitLab token scoping rationale

### 2026-03-12 14:46 - ₢AsAAC - n

Marshal Reset output and onboarding tabtarget from earlier session work

### 2026-03-12 14:45 - ₢AsAAC - n

Onboarding guide: renamed level 2 from generic OAuth to precise RBRA payor credential emplacement, added RBRR configuration review before depot create commitment point

### 2026-03-12 14:31 - ₢AsAAC - n

Bumped RBRR_GCB_TIMEOUT reset default from 1200s (20m) to 2700s (45m)

### 2026-03-12 14:24 - ₢AsAAC - n

Moved IGNITE_MACHINE_NAME and DEPLOY_MACHINE_NAME from RBRR repo regime to RBRM podman VM regime where they logically belong, updated all code references, spec definitions, and linked terms

### 2026-03-12 14:18 - ₢AsAAC - n

Alignment cleanup of case statement columns in rbrr_reset

### 2026-03-12 14:14 - ₢AsAAC - n

Merged rbrr_reset into rbrr_cli.sh with differential furnish (rbrn pattern), replaced heredoc template with line-by-line case transform, deleted standalone rbrr_reset_cli.sh

### 2026-03-12 14:04 - ₢AsAAC - n

Marshal Reset pre-fills 7 sensible defaults (DNS, machine type, timeout, concurrent builds, vessel dir, region, secrets dir), blanks only site-specific fields. Added configuration review as onboarding level 0: shows RBRR defaults with per-item orientation plus BURC project structure. Added mkdir -p RBRR_SECRETS_DIR at kindle time in rbdc. Removed Marshal Reset tabtarget from level 8 completion message.

### 2026-03-12 13:42 - ₢AsAAC - n

Refactored onboarding guide: while-break ladder computes strict-sequential level integer replacing 8 independent booleans and if-chain. Added per-level frontmatter orientation, dropped step 9 (no probe), split level 7/8 for conjure+vouch vs start. Added BURD_NO_LOG to tabtarget.

### 2026-03-12 13:27 - ₢AsAAC - n

Unified guide tabtarget colophons under rbw-g prefix with case convention (uppercase=cloud-affecting, lowercase=safe), added rbtc_ linked term category for tabtarget colophon references in spec files eliminating 11 hardcoded colophon strings across 5 spec documents

### 2026-03-11 16:40 - ₢AsAAJ - W

Implemented Marshal Reset (rbw-MR) as pre-kindle CLI with blank RBRR template generation. Added rbtr_marshal linked term to RBS0-SpecTop.adoc. Exercised rbw-MR → rbw-PO flow confirming onboarding guide correctly detects blank regime state.

### 2026-03-11 16:06 - ₢AsAAI - W

Implemented adaptive onboarding guide (rbw-PO) with BCG-compliant pre-kindle CLI, grep-based state probes for 8 onboarding phases, status dashboard with guided next-step routing, and zipper enrollment via dedicated rbgm_onboard_cli.sh. College try — validation deferred to AsAAC integration pass.

### 2026-03-11 16:06 - Heat - T

fresh-depot-conjure-vouch

### 2026-03-11 16:05 - ₢AsAAI - n

Implement adaptive onboarding guide with BCG-compliant pre-kindle CLI, grep-based state probes, and zipper enrollment

### 2026-03-11 15:35 - Heat - T

fresh-depot-conjure-vouch

### 2026-03-11 15:35 - Heat - T

release-rbrr-reset

### 2026-03-11 15:35 - Heat - T

adaptive-getting-started-guide

### 2026-03-11 15:33 - Heat - T

fresh-depot-conjure-vouch

### 2026-03-11 15:31 - Heat - r

moved AsAAJ after AsAAI

### 2026-03-11 15:31 - Heat - T

release-rbrr-reset

### 2026-03-11 15:30 - Heat - T

adaptive-getting-started-guide

### 2026-03-11 15:16 - Heat - D

restring 1 paces from ₣AU

### 2026-03-11 15:16 - Heat - T

adaptive-getting-started-guide

### 2026-03-11 14:52 - Heat - S

adaptive-getting-started-guide

### 2026-03-11 14:21 - ₢AsAAH - W

Homogenized 5 HTTP code capture fallbacks from || z_*="" to || buc_die in repo/SA-verify/SA/bucket/secret GET paths. Removed dead kindle constants ZRBGI_INFIX_REPO_POLICY and ZRBGI_INFIX_RPOLICY_SET (zero callers confirmed).

### 2026-03-11 14:21 - ₢AsAAH - n

Homogenize z_get_code to die-on-failure in repo/SA, remove dead ZRBGI_INFIX_REPO_POLICY/RPOLICY_SET kindle constants

### 2026-03-11 14:16 - ₢AsAAH - A

Homogenize z_get_code to die-on-failure in repo/SA, remove dead ZRBGI_INFIX_REPO_POLICY/RPOLICY_SET kindle constants

### 2026-03-11 11:39 - ₢AsAAG - W

Extracted 3x copy-pasted Secret Manager IAM grants from rbgp_Payor.sh into new rbgi_grant_secret_iam function in rbgi_IAM.sh. New function follows corrected contract pattern: GET getIamPolicy with requestedPolicyVersion=3, explicit etag extraction+assert, 409 ABORTED fatal, transient retry (429/5xx), propagation retry (3s/2x/20s/420s). Added rbtoe_iam_grant_secret spec definition in RBS0, linked term in RBSDC, updated RBSCIG references from inline to first-class grant pattern.

### 2026-03-11 11:38 - ₢AsAAG - n

Extract 3x inline Secret Manager IAM grants from rbgp_Payor.sh into new rbgi_grant_secret_iam function in rbgi_IAM.sh with full contract compliance (explicit etag, GET getIamPolicy, requestedPolicyVersion=3, 409 ABORTED, transient retry, propagation retry). Update RBS0 spec definition, RBSDC linked term, RBSCIG references.

### 2026-03-11 11:33 - ₢AsAAG - A

Read all 5 files, write rbgi_grant_secret_iam following AsAAF pattern (etag/retry/409), replace 3x inline in rbgp_Payor.sh, update RBS0+RBSDC+RBSCIG specs. Sequential sonnet.

### 2026-03-11 11:31 - ₢AsAAF - W

Brought 4 IAM grant functions into etag/error/retry compliance per RBSCIG research. Project: removed dead 412 case, updated 409 to ABORTED message. Repo: POST->GET for getIamPolicy, added requestedPolicyVersion=3 query param, etag extraction+assert, inner transient retry loop with 409 fatal. SA: version3 body for getIamPolicy, etag extraction+assert, inner transient retry loop. Bucket: requestedPolicyVersion=3 (flat Storage API form), etag assert (die not fallback), inner transient retry loop with 412 fatal + 409 defensive. Fixed 5 pre-existing BCG test...&& violations across all functions.

### 2026-03-11 11:29 - ₢AsAAF - n

Fix bucket getIamPolicy query param from gRPC-transcoding dot form (options.requestedPolicyVersion) to Storage JSON API flat form (optionsRequestedPolicyVersion).

### 2026-03-11 11:29 - Heat - S

rbgi-kindle-and-capture-cleanup

### 2026-03-11 11:27 - ₢AsAAF - n

Bring 4 IAM grant functions into etag/error/retry compliance per RBSCIG research. Project: remove dead 412, update 409 message. Repo: POST->GET, requestedPolicyVersion=3, etag extraction, inner retry loop. SA: version3 body, etag extraction, inner retry loop. Bucket: requestedPolicyVersion=3, etag assert, inner retry loop with 412. Fix 5 pre-existing BCG test...&& violations across all functions.

### 2026-03-11 11:19 - ₢AsAAF - A

Sequential single-file edit of rbgi_IAM.sh. Project function fixes first (minimal), then repo (highest risk POST->GET), then sa, then bucket. Project retry loop is template for other 3.

### 2026-03-11 11:19 - ₢AsAAF - B

arm | rbgi-etag-and-error-compliance

### 2026-03-11 11:19 - Heat - T

rbgi-etag-and-error-compliance

### 2026-03-11 11:16 - Heat - T

rbgi-secret-manager-extraction

### 2026-03-11 11:15 - Heat - T

rbgi-etag-and-error-compliance

### 2026-03-11 11:11 - Heat - T

rbgi-secret-manager-extraction

### 2026-03-11 11:10 - Heat - T

rbgi-etag-and-error-compliance

### 2026-03-11 11:05 - Heat - n

Create RBSHR-HorizonRoadmap.adoc as single collection point for defined-but-unscoped future work (IAM identity, Podman runtime, build pipeline security). Rename RBSCB CloudBuildRoadmap to CloudBuildPosture (accurately reflects content). RBSCIG convergence analysis deferred items now cross-reference RBSHR.

### 2026-03-11 10:53 - Heat - S

rbgi-secret-manager-extraction

### 2026-03-11 10:53 - Heat - S

rbgi-etag-and-error-compliance

### 2026-03-11 10:52 - ₢AsAAA - W

Research phase: created RBSCIG-IamGrantContracts.adoc documenting verified behavioral contracts of 5 Google Cloud IAM policy APIs (CRM, IAM SA, Artifact Registry, Storage, Secret Manager). Corrected 4 rbtoe_iam_grant_* definitions in RBS0 to match actual API behavior (409 not 412, etag always present, no API returns 404 for no-policy, correct HTTP methods). Fixed RBSCIP attribute references. Paddock rewritten: superset contract abandoned after research proved per-API differences are load-bearing. 17 sourced evidence claims in RBSCIG evidence table.

### 2026-03-11 10:52 - Heat - d

paddock curried: Paddock rewritten to reflect RBSCIG research findings. Superset contract abandoned; per-API differences are load-bearing.

### 2026-03-11 10:48 - ₢AsAAA - n

Add RBSCIG research memo documenting verified IAM grant API contracts across 5 Google Cloud APIs (CRM, IAM SA, Artifact Registry, Storage, Secret Manager). Correct 4 rbtoe_iam_grant_* definitions in RBS0 to match actual API behavior: 409 not 412 for etag mismatch, etag always present, no API returns 404 for no-policy. Fix RBSCIP attribute references. Evidence table with 17 sourced claims.

### 2026-03-10 20:30 - ₢AsAAA - A

Phase 1: Read all 4 grant functions in rbgi_IAM.sh, catalog exact API differences (GET/SET methods, URL construction, etag handling, error shapes). Phase 2: Implement 4 step helpers + rewrite 4 outer functions as thin wrappers + add rbgi_grant_secret_iam. Phase 3: Update ~30 call sites (rbgp_Payor.sh majority, plus rbgg/rbga/rbgb). Phase 4: Collapse spec (RBS0 4->1 definition, update RBSDC/RBSDI/RBSGR/RBSRC subdocuments). Sequential opus, each phase informs the next.

### 2026-03-10 20:30 - ₢AsAAA - B

arm | iam-grant-homogenization

### 2026-03-10 20:30 - Heat - T

iam-grant-homogenization

### 2026-03-10 20:24 - ₢AsAAB - W

Full test sweep passed: 10 fixtures, 95 cases all green. Baseline confirmed before IAM grant refactor. ~7.5 min wall-clock.

### 2026-03-10 20:01 - ₢AsAAB - A

Run full test sweep matching mvp-1 exit results to confirm green baseline before IAM grant refactor

### 2026-03-10 19:35 - Heat - T

iam-grant-homogenization

### 2026-03-10 19:35 - Heat - d

paddock curried

### 2026-03-10 18:45 - Heat - f

racing

### 2026-03-09 19:47 - Heat - D

restring 1 paces from ₣Ak

### 2026-03-09 19:45 - Heat - S

post-validation-test-sweep

### 2026-03-09 19:45 - Heat - S

fresh-depot-conjure-vouch

### 2026-03-09 19:45 - Heat - S

pre-refactor-baseline-sweep

### 2026-03-09 19:43 - Heat - D

restring 1 paces from ₣Ak

### 2026-03-09 19:42 - Heat - N

rbk-mvp-2-iam-grant-unification

