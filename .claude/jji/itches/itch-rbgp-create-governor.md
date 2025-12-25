# Itch: Implement rbgp_create_governor

Create `rbgp_create_governor()` in `Tools/rbw/rbgp_Payor.sh` following RBAGS spec lines 579-653.

## Implementation Guide

**Critical reference:** BCG (`../cnmp_CellNodeMessagePrototype/lenses/bpu-BCG-BashConsoleGuide.md`) for bash style, error handling, and control flow patterns.

**Placement:** Add function after `rbgp_depot_list()` (around line 1012), before `rbgp_payor_oauth_refresh()`.

**Pattern:** Follow `rbgp_depot_create` (zrbgp_sentinel, OAuth auth via zrbgp_authenticate_capture).

**Helpers available:** rbgu_http_json, rbgi_add_project_iam_role, rbgo_rbra_generate_from_key.

## Steps per Spec

1. Validate RBRR_DEPOT_PROJECT_ID exists and != RBRP_PAYOR_PROJECT_ID
2. Create SA via iam.serviceAccounts.create in depot project
3. Poll/verify SA accessible via iam.serviceAccounts.get (3-5s intervals, 30s max)
4. Check no USER_MANAGED keys exist via serviceAccounts.keys.list
5. Grant roles/owner via projects.setIamPolicy (policy version 3)
6. Create key via serviceAccounts.keys.create and generate RBRA file

## API Verification

All 5 GCP REST APIs verified against spec (2025-12-25):
- iam.serviceAccounts.create/get
- serviceAccounts.keys.list/create
- projects.setIamPolicy (v3)

No discrepancies found.

## Success Criteria

Function exists, follows spec steps, uses correct auth pattern, adheres to BCG.

## Context

Extracted from heat jjh-b251225-rbags-manual-proc-spec during scope refinement, 2025-12-25. Spec is complete and API-verified; ready for implementation when prioritized.
