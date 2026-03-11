# Paddock: rbk-mvp-2-iam-grant-unification

## Context

Unify the four IAM grant variants in rbgi_IAM.sh into a single function with a superset behavioral contract. This is a refactor for clarity and maintainability — runtime behavior is preserved. The four variants (project, repo, sa, bucket) evolved independently and now have arbitrary-feeling differences in retry, etag, verification, and error handling. Collapsing to one pattern reduces cognitive load and spec surface area.

## Prior Art

MVP-1 (₣Ak, retired) established the spec definition aesthetic: every line in a rbtoe_ definition should be load-bearing. The unified rbtoe_iam_grant definition follows that principle. The BCG compliance finding (no bash dynamic scoping) was also settled in ₣Ak.

## Key Design Decisions (Settled)

- **Superset contract**: every grant gets every guarantee (etag, propagation retry, transient retry, read-back verify, vanilla policy init, SA preflight). Harmless where not needed, protective everywhere.
- **BCG-compliant Option B**: extracted step helpers (zrbgi_read_policy, zrbgi_compose_binding, zrbgi_write_policy, zrbgi_verify_binding) called by per-resource outer functions owning retry loops.
- **Two failure classes**: member-SA propagation (retry inside grant) vs resource propagation (rbgu_poll_until_ok before grant). Do not conflate.

## References

- Tools/rbk/rbgi_IAM.sh — the four grant functions to unify
- Tools/rbk/rbgp_Payor.sh — majority of call sites (~30 total across 4 files)
- Tools/rbk/rbgg_Governor.sh, Tools/rbk/rbga_ArtifactRegistry.sh, Tools/rbk/rbgb_Buckets.sh — remaining call sites
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — 4 rbtoe_iam_grant_* definitions to collapse to 1
- Tools/rbk/vov_veiled/RBSDC-depot_create.adoc, RBSDI-director_create.adoc, RBSGR-governor_reset.adoc, RBSRC-retriever_create.adoc — subdocuments referencing grant terms
- Tools/rbk/vov_veiled/RBSCIP-IamPropagation.adoc — may reference unified term