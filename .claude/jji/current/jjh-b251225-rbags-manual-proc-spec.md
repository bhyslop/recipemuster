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

## Current

### Fix RBAGS attribute mappings
7 name mismatches: rbmp→rbgm_payor_refresh, rbgg_retriever_create→rbgg_create_retriever, rbgg_director_create→rbgg_create_director, rbgo_trigger_build→rbf_build, "Image Deletion"→rbf_delete, rbgs_list_service_accounts→rbgg_list_service_accounts, rbgs_list_service_accounts→rbgg_delete_service_account

## Remaining

### Spec-Implementation Alignment
- Implement rbtgo_governor_create (missing implementation)
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
- Verify rbtgo_depot_destroy API calls against current Google Cloud REST API documentation
- Verify rbtgo_governor_create API calls against current Google Cloud REST API documentation
- Verify rbtgo_retriever_create API calls against current Google Cloud REST API documentation
- Audit all RBAGS operations for API correctness via websearch
