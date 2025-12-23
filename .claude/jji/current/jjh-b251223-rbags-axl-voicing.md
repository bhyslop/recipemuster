# Heat: RBAGS AXL Voicing Integration

## Context

Add AXL voicing annotations to all definitions in RBAGS (lenses/rbw-RBAGS-AdminGoogleSpec.adoc) to formalize the type model. This exercises the AXL system with real content and will expose friction points for refinement.

Goal: Every definition in RBAGS gets a `// ⟦axl_voices motif [dimensions]⟧` annotation line between anchor and definition.

## Completed

- [x] **Orchestration Control Terms** (manual)
  Retired oct_* prefix entirely. Abstract control motifs (axc_*) remain in AXL. Created environment-specific voicings: rbbc_* (bash console) and rbhg_* (human guide) with axe_* environment dimensions.

- [x] **Operation Definitions** (manual)
  Added voicings to all rbtgo_* operations. COMMAND operations voice `axo_command axe_bash_interactive`. GUIDE operations voice `axo_guide axe_human_guide`. Format: anchor + voicing annotation + heading.

- [x] **Pattern Definitions** (manual)
  Added voicings to all rbtoe_* patterns (rbtoe_rbra_generate, rbtoe_rbra_load, rbtoe_jwt_oauth_exchange, rbtoe_api_enable, rbtoe_oauth_refresh, rbtoe_payor_authenticate, rbtoe_rbro_load, rbtoe_depot_list_update). All voice `axo_pattern axe_bash_interactive`.

## Paces

- [ ] **P1: Role Definitions** (manual)
  Add voicings to rbtr_* definitions (rbtr_role, rbtr_payor, rbtr_governor, rbtr_mason, rbtr_director, rbtr_retriever). These voice axo_role.

- [ ] **P2: Regime Definitions** (manual)
  Add voicings to *_regime definitions (rbrr_regime, rbrp_regime, rbro_regime, rbrv_regime, rbra_regime, rbev_regime, at_regime). These voice axo_regime.

- [ ] **P3: Regime Variable Definitions** (manual)
  Add voicings to regime variables (rbrr_*, rbra_*, rbrp_*, rbro_* individual variables). These voice axr_member with appropriate dimensions (axd_required, axt_string, etc.).

- [ ] **P4: GCP and Infrastructure Definitions** (manual)
  Add voicings to GCP definitions (gcp_*, giam_*, gar_*, gcs_*, gcb_*) and RB instance definitions (rbtgi_*). Determine appropriate motifs for cloud infrastructure concepts.

- [ ] **P5: Cross-Reference and Support Definitions** (manual)
  Add voicings to at_* cross-reference definitions and remaining support infrastructure definitions.

- [ ] **P6: Review and Refine** (manual)
  Review all voicings for consistency. Document any friction points encountered. Propose refinements to AXL system based on practical application.

## Notes

- Annotation format: `// ⟦axl_voices primary [dimension ...]⟧`
- Annotations go between `[[anchor]]` and `{term}::` lines
- Some definitions may reveal gaps in AXL motif coverage
- Architectural pivot: oct_* retired, replaced by environment-specific voicings (rbbc_*, rbhg_*) with axe_* dimensions added to AXL
