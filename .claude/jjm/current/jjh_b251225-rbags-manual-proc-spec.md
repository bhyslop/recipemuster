# Heat: RBAGS Manual Procedure Specification Alignment

## Context

RBAGS (lenses/rbw-RBAGS-AdminGoogleSpec.adoc) specifies Recipe Bottle's Google Cloud operations. Implementation exists in Tools/rbw/ but spec sections are incomplete or misaligned.

Goal: Complete and align specification for the Director-triggered remote build flow. Then factor RBAGS into top document + includable sequence files for context-efficient loading.

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

6. **Specify rbtgo_director_create** - Expanded from prose to 11-step sequence with 8 API links. Added `rbtoe_governor_authenticate` pattern (parallel to payor). Documented Mason impersonation and build bucket access requirements.

7. **Specify rbtgo_trigger_build** - Expanded to 11-step sequence with 3 API links; added rbtoe_director_authenticate pattern

8. **Specify rbtgo_image_delete** - 6-step sequence using Docker Registry API V2; 2 OCI Distribution Spec links

9. **Specify rbtgo_sa_list** - 4-step multi-role operation; 1 IAM API link; caller-provided auth pattern

10. **Specify rbtgo_sa_delete** - 4-step multi-role operation; 1 IAM API link; handles 404 gracefully

11. **Specify rbtgo_image_retrieve** - 6-step Retriever operation; added rbtoe_retriever_authenticate pattern; container runtime auth

12. **Fix CMK normalizer and normalize RBAGS** - Fixed normalizer backtick rules, normalized RBAGS; created itch for Rust replacement due to LLM unreliability

## Current

13. **Extract prototype sequences DC and TB** (delegated) - Create `lenses/rbw-RBSDC-depot_create.adoc` and `lenses/rbw-RBSTB-trigger_build.adoc`. Modify RBAGS to keep anchor + terse def, add include directive for sequence body. Verify AsciiDoc rendering works.

## Remaining

14. **Review prototype extraction** (manual) - Human reviews extracted files and include syntax. Confirm approach before bulk extraction. Green-light remaining 11 sequences.

15. **Extract remaining 11 RBAGS sequences** (delegated) - Apply same pattern to: PE (payor_establish), PR (payor_refresh), DD (depot_destroy), DL (depot_list), GC (governor_create), RC (retriever_create), DI (director_create), ID (image_delete), IR (image_retrieve), SL (sa_list), SD (sa_delete).

### Subsection Naming Convention

Files use pattern `rbw-RBSxx-operation_name.adoc` where xx is 2-letter code (no codes start with A, reserved for future RBSA top file):

| Code | Filename |
|------|----------|
| PE | `rbw-RBSPE-payor_establish.adoc` |
| PR | `rbw-RBSPR-payor_refresh.adoc` |
| DC | `rbw-RBSDC-depot_create.adoc` |
| DD | `rbw-RBSDD-depot_destroy.adoc` |
| DL | `rbw-RBSDL-depot_list.adoc` |
| GC | `rbw-RBSGC-governor_create.adoc` |
| RC | `rbw-RBSRC-retriever_create.adoc` |
| DI | `rbw-RBSDI-director_create.adoc` |
| TB | `rbw-RBSTB-trigger_build.adoc` |
| ID | `rbw-RBSID-image_delete.adoc` |
| IR | `rbw-RBSIR-image_retrieve.adoc` |
| SL | `rbw-RBSSL-sa_list.adoc` |
| SD | `rbw-RBSSD-sa_delete.adoc` |
