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