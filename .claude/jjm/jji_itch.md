# Itches

## rbags-retriever-spec
Specify rbtgo_retriever_create and rbtgo_image_retrieve in RBAGS.

## rbags-validation-run
End-to-end test of remote build with real infrastructure.

## rbags-payor-spec-review
Verify rbtgo_payor_establish/refresh spec matches rbgm_ManualProcedures.sh.

## rbags-api-audit
Verify remaining RBAGS operations against GCP REST API docs (depot_create, depot_destroy, retriever_create).

## rbgg-rename
Rename rbgg_Governor.sh to better reflect scope (handles all depot service accounts + project lifecycle, not just Governor role).

## axo-relevel
The axo_ (Axial Operation) category has accumulated disparate concepts:

1. **Operation execution types** (original intent): command, guide, pattern, sequence
2. **Identity concepts**: role, identity, actor
3. **Configuration infrastructure**: regime, slot, assignment
4. **Dependencies**: dependency

### Proposed First Step
Sort all axo_ attribute mappings into coherent subgroups to understand the natural clustering before deciding on category splits.

### Options to Consider
- Keep axo_ as broad "operational infrastructure"
- Split identity concepts to new prefix (axi_ is taken for Interface)
- Move regime/slot to axr_ (structural parallel with record/member)
- New category for configuration concepts

### Context
Emerged during RBAGS AXL voicing heat, session 2025-12-23.

## crg-to-cmk
Consider adding CRG (Configuration Regime Requirements) to concept-model-kit.md as a sibling pattern to MCM. CRG is a meta-specification for defining configuration regimes - reusable across projects, not Recipe Bottle specific.

Reference: lenses/crg-CRR-ConfigRegimeRequirements.adoc

## rbgp-create-governor
Create `rbgp_create_governor()` in `Tools/rbw/rbgp_Payor.sh` following RBAGS spec lines 579-653.

### Implementation Guide

**Critical reference:** BCG (`../cnmp_CellNodeMessagePrototype/lenses/bpu-BCG-BashConsoleGuide.md`) for bash style, error handling, and control flow patterns.

**Placement:** Add function after `rbgp_depot_list()` (around line 1012), before `rbgp_payor_oauth_refresh()`.

**Pattern:** Follow `rbgp_depot_create` (zrbgp_sentinel, OAuth auth via zrbgp_authenticate_capture).

**Helpers available:** rbgu_http_json, rbgi_add_project_iam_role, rbgo_rbra_generate_from_key.

### Steps per Spec

1. Validate RBRR_DEPOT_PROJECT_ID exists and != RBRP_PAYOR_PROJECT_ID
2. Create SA via iam.serviceAccounts.create in depot project
3. Poll/verify SA accessible via iam.serviceAccounts.get (3-5s intervals, 30s max)
4. Check no USER_MANAGED keys exist via serviceAccounts.keys.list
5. Grant roles/owner via projects.setIamPolicy (policy version 3)
6. Create key via serviceAccounts.keys.create and generate RBRA file

### API Verification

All 5 GCP REST APIs verified against spec (2025-12-25):
- iam.serviceAccounts.create/get
- serviceAccounts.keys.list/create
- projects.setIamPolicy (v3)

No discrepancies found.

### Success Criteria

Function exists, follows spec steps, uses correct auth pattern, adheres to BCG.

### Context

Extracted from heat jjh-b251225-rbags-manual-proc-spec during scope refinement, 2025-12-25. Spec is complete and API-verified; ready for implementation when prioritized.

## rbtgo-image-retrieve
Design and implement the image retrieval operation - currently has neither spec nor implementation.

### Context

Identified during RBAGS audit (heat jjh-b251225-rbags-manual-proc-spec, 2025-12-25) as one of two missing implementations in the Director-triggered remote build flow.

### Prerequisites

Before implementation:
1. Specify rbtgo_image_retrieve in RBAGS following completeness criteria
2. Verify API calls against GCP REST documentation

### Open Questions

- Which GCP API retrieves container images from Artifact Registry?
- What authentication pattern - Governor RBRA or Director token?
- Output format - tarball, OCI manifest, or streaming pull?
- Destination - local file, pipe to podman, or registry mirror?

### Related

- `rbtgo_trigger_build` - triggers the build that creates images
- `rbtgo_image_delete` - removes images (has implementation in rbf_Foundry.sh)
