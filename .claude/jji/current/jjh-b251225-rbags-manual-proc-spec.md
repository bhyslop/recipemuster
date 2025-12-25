# Heat: RBAGS Manual Procedure Specification Alignment

## Context

RBAGS (lenses/rbw-RBAGS-AdminGoogleSpec.adoc) specifies Recipe Bottle's Google Cloud operations. Implementation exists in Tools/rbw/ but spec sections are incomplete or misaligned.

Goal: Complete and align specification for the Director-triggered remote build flow.

### Reference Files

**Specification:**
- `lenses/rbw-RBAGS-AdminGoogleSpec.adoc` - master spec

**Implementation:**
- `Tools/rbw/rbgm_ManualProcedures.sh` - payor establish/refresh display
- `Tools/rbw/rbgg_Governor.sh` - director/retriever creation
- `Tools/rbw/rbf_Foundry.sh` - trigger build, image delete
- `Tools/rbw/rbgo_OAuth.sh` - JWT exchange
- `Tools/rbw/rbgu_Utility.sh` - RBRA load, API enable

**Regime Configuration:**
- `rbrr_RecipeBottleRegimeRepo.sh` - master regime
- `rbrp.env` - payor config

**Legacy (deleted):**
- ~~`Tools/rbw/rbmp_ManualProcedures-PCG005.md`~~

### Completeness Criteria

A fully specified `axo_command` operation must have:
1. Anchor `[[rbtgo_*]]` and voicing annotation `// ⟦axl_voices axo_command axe_bash_interactive⟧`
2. Numbered step sequence (not just prose description)
3. Each step uses appropriate control term (`{rbbc_require}`, `{rbbc_store}`, `{rbbc_call}`, `{rbbc_submit}`, `{rbbc_await}`, `{rbbc_show}`, `{rbbc_fatal}`, `{rbbc_warn}`)
4. Variables use `«NAME»` notation with balanced store/use pairs
5. API calls include REST API documentation links
6. Error conditions explicitly use `{rbbc_fatal}` or `{rbbc_warn}`

**Reference:** `rbtgo_depot_create` (RBAGS lines 348-471)

## Done

1. **Delete legacy rbmp_ManualProcedures-PCG005.md** - Deleted `Tools/rbw/rbmp_ManualProcedures-PCG005.md` via git rm

2. **Define specification completeness criteria for RBAGS operations** - Defined 6-point completeness criteria (anchor, steps, control terms, variables, API links, errors). Audited 13 operations: found 7 name mismatches, 2 missing implementations, 6 incomplete specs. Reorganized paces into Alignment/Completion/Finalization phases. Added 5 API verification itches.

3. **Fix RBAGS attribute mappings** - Updated 7 attribute definitions in RBAGS lines 67-80 to match implementation function names (rbmp→rbgm, rbgg_*_create→rbgg_create_*, rbgo→rbf, rbgs→rbgg)

4. **Implement rbtgo_governor_create** - Analyzed spec and precedent. Fixed mapping to `rbgp_create_governor`. Determined: lives in rbgp_Payor.sh (Payor operation, OAuth auth), follows rbgp_depot_create pattern, grants roles/owner. Created delegated paces for verification (opus) and implementation (sonnet).

5. **Verify rbtgo_governor_create spec against GCP APIs** - Verified 5 GCP REST APIs (serviceAccounts.create/get, keys.list/create, projects.setIamPolicy) against RBAGS spec lines 579-653. All request/response fields, enum values (KEY_ALG_RSA_2048, TYPE_GOOGLE_CREDENTIALS_FILE, USER_MANAGED), and endpoint patterns confirmed accurate. No discrepancies found.

## Current

### Create rbgp_create_governor implementation
- **Mode:** delegated
- **Objective:** Implement `rbgp_create_governor()` in `Tools/rbw/rbgp_Payor.sh` following RBAGS spec lines 579-653
- **Critical guide:** BCG (`../cnmp_CellNodeMessagePrototype/lenses/bpu-BCG-BashConsoleGuide.md`) for bash style, error handling, and control flow patterns
- **Scope:**
  - Add function after `rbgp_depot_list()` (around line 1012), before `rbgp_payor_oauth_refresh()`
  - Follow `rbgp_depot_create` pattern (zrbgp_sentinel, OAuth auth via zrbgp_authenticate_capture)
  - Use existing helpers: rbgu_http_json, rbgi_add_project_iam_role, rbgo_rbra_generate_from_key
- **Steps per spec:**
  1. Validate RBRR_DEPOT_PROJECT_ID exists and ≠ RBRP_PAYOR_PROJECT_ID
  2. Create SA via iam.serviceAccounts.create in depot project
  3. Poll/verify SA accessible via iam.serviceAccounts.get (3-5s intervals, 30s max)
  4. Check no USER_MANAGED keys exist via serviceAccounts.keys.list
  5. Grant roles/owner via projects.setIamPolicy (policy version 3)
  6. Create key via serviceAccounts.keys.create and generate RBRA file
- **Success:** Function exists, follows spec steps, uses correct auth pattern, adheres to BCG
- **Failure:** If helper functions missing or pattern unclear, stop and report
- **Model hint:** needs-sonnet (complex multi-step API integration)

- Implement rbtgo_image_retrieve (missing implementation)

### Specification Completion (per Completeness Criteria)

**Extract from implementation:**
- Specify rbtgo_director_create (extract from `rbgg_create_director`, has prose but needs step sequence)
- Specify rbtgo_trigger_build (extract from `rbf_build`)
- Specify rbtgo_image_delete (extract from `rbf_delete`)
- Specify rbtgo_sa_list (extract from `rbgg_list_service_accounts`)
- Specify rbtgo_sa_delete (extract from `rbgg_delete_service_account`)

**Design new:**
- Specify rbtgo_image_retrieve (no implementation exists)

### Finalization
- Normalize and validate RBAGS

## Itches

- Verify rbtgo_depot_create API calls against current Google Cloud REST API documentation
- Rename rbgg_Governor.sh to better reflect scope (handles all depot service accounts + project lifecycle, not just Governor role)
- Verify rbtgo_depot_destroy API calls against current Google Cloud REST API documentation
- Verify rbtgo_retriever_create API calls against current Google Cloud REST API documentation
- Audit all RBAGS operations for API correctness via websearch
