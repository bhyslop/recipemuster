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

13. **Extract prototype sequences DC and TB** (delegated) - Created `lenses/rbw-RBSDC-depot_create.adoc` (162 lines, 15 steps) and `lenses/rbw-RBSTB-trigger_build.adoc` (198 lines, 12 steps). Modified RBAGS to keep anchor + terse def + include directive. Verified rendering.

14. **Review prototype extraction** (manual) - Human reviewed extracted files and include syntax. Confirmed approach for bulk extraction.

## Current

15. **Extract remaining 11 RBAGS sequences** (delegated) - Apply same pattern to: PE (payor_establish), PR (payor_refresh), DD (depot_destroy), DL (depot_list), GC (governor_create), RC (retriever_create), DI (director_create), ID (image_delete), IR (image_retrieve), SL (sa_list), SD (sa_delete).

## Remaining

(none)

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

## Steeplechase

# Steeplechase: RBAGS Manual Procedure Specification Alignment

---
### 2025-12-25 - fix-cmk-normalizer-and-normalize-rbags - APPROACH
**Mode**: manual
**Proposed approach**:
- Add explicit exception to Phase 1 rule #2 for section headers (lines starting with `=`)
- Clarify that term isolation applies to prose paragraphs and definition entries, not structural elements
- Run `/cma-normalize` on RBAGS and verify section headers remain intact
- Validate the normalized output has no broken headers
---

---
### 2025-12-25 21:15 - fix-cmk-normalizer-and-normalize-rbags - WRAP
**Mode**: manual
**Outcome**: Fixed normalizer backtick rules, normalized RBAGS; created itch for Rust replacement due to LLM unreliability
---

---
### 2025-12-26 05:37 - extract-prototype-sequences-dc-and-tb - APPROACH
**Mode**: delegated
**Proposed approach**:
- Extract rbtgo_depot_create body (lines ~355-483) to `lenses/rbw-RBSDC-depot_create.adoc`
- Extract rbtgo_trigger_build body (lines ~814-968) to `lenses/rbw-RBSTB-trigger_build.adoc`
- In RBAGS, replace full sequence with: anchor + terse 1-2 line definition + AsciiDoc include directive
- Test that cross-references still resolve
- Verify full document renders correctly with includes
---

---
### 2025-12-26 05:49 - extract-prototype-sequences-dc-and-tb - DELEGATE
**Spec**:
- Objective: Extract DC and TB sequence bodies to includable files
- Scope: 2 files created, RBAGS modified
- Success: Files contain complete sequences, includes render correctly
- On failure: Stop and report

**Execution trace**:
- Read RBAGS lines 355-520 (depot_create body)
- Read RBAGS lines 912-1113 (trigger_build body)
- Created `lenses/rbw-RBSDC-depot_create.adoc` (162 lines, 15 steps)
- Created `lenses/rbw-RBSTB-trigger_build.adoc` (198 lines, 12 steps)
- Modified RBAGS: replaced depot_create body with terse def + include
- Modified RBAGS: replaced trigger_build body with terse def + include

**Result**: success
Haiku extracted both sequences correctly. Opus reviewed and confirmed nothing lost.

**Modified files**:
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSDC-depot_create.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSTB-trigger_build.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBAGS-AdminGoogleSpec.adoc
---

---
### 2025-12-26 05:49 - review-prototype-extraction - WRAP
**Mode**: manual
**Outcome**: Human reviewed extracted files, verified AsciiDoc rendering, confirmed approach for remaining 11 sequences.
---

---
### 2025-12-26 - extract-remaining-11-rbags-sequences - DELEGATE
**Spec**:
- Objective: Extract 11 remaining sequence sections from RBAGS to includable files
- Scope: PE, PR, DD, DL, GC, RC, DI, ID, IR, SL, SD operations
- Success: All 11 files created with correct content boundaries, RBAGS modified with include directives
- On failure: Report which operations completed, which failed, and why

**Execution trace**:
- Read full RBAGS (1997 lines) to understand structure
- Extracted section boundaries for all 11 remaining operations
- Created `lenses/rbw-RBSPE-payor_establish.adoc` (63 lines, PE operation)
- Created `lenses/rbw-RBSPR-payor_refresh.adoc` (21 lines, PR operation)
- Created `lenses/rbw-RBSDD-depot_destroy.adoc` (79 lines, DD operation)
- Created `lenses/rbw-RBSDL-depot_list.adoc` (35 lines, DL operation)
- Created `lenses/rbw-RBSGC-governor_create.adoc` (93 lines, GC operation)
- Created `lenses/rbw-RBSRC-retriever_create.adoc` (10 lines, RC operation)
- Created `lenses/rbw-RBSDI-director_create.adoc` (133 lines, DI operation)
- Created `lenses/rbw-RBSID-image_delete.adoc` (71 lines, ID operation)
- Created `lenses/rbw-RBSIR-image_retrieve.adoc` (61 lines, IR operation)
- Created `lenses/rbw-RBSSL-sa_list.adoc` (42 lines, SL operation)
- Created `lenses/rbw-RBSSD-sa_delete.adoc` (34 lines, SD operation)
- Modified RBAGS: replaced payor_establish body with anchor + include
- Modified RBAGS: replaced payor_refresh body with anchor + include
- Modified RBAGS: replaced depot_destroy body with anchor + include
- Modified RBAGS: replaced depot_list body with anchor + include
- Modified RBAGS: replaced governor_create body with anchor + include
- Modified RBAGS: replaced retriever_create body with anchor + include
- Modified RBAGS: replaced director_create body with anchor + include
- Modified RBAGS: replaced image_delete body with anchor + include
- Modified RBAGS: replaced image_retrieve body with anchor + include
- Modified RBAGS: replaced sa_list body with anchor + include
- Modified RBAGS: replaced sa_delete body with anchor + include

**Result**: success
All 11 operations extracted. All files created with clean boundaries. RBAGS now includes all 13 sequences (DC, TB, PE, PR, DD, DL, GC, RC, DI, ID, IR, SL, SD).

**Modified files**:
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSPE-payor_establish.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSPR-payor_refresh.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSDD-depot_destroy.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSDL-depot_list.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSGC-governor_create.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSRC-retriever_create.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSDI-director_create.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSID-image_delete.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSIR-image_retrieve.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSSL-sa_list.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSSD-sa_delete.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBAGS-AdminGoogleSpec.adoc
---
