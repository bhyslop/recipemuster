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
  Added voicings to all rbtoe_* patterns. All voice `axo_pattern axe_bash_interactive`.

- [x] **Role Definitions** (manual)
  Added axo_identity and axo_actor to AXL. rbtr_role voices axo_identity (abstract concept). Specific actors (payor, governor, retriever, mason, director) voice axo_actor. Key insight: guide-ness lives on operations, not actors.

- [x] **Regime Infrastructure in AXL** (manual)
  Major AXL additions this session:
  - **axf_* (Axial Format)**: axf_format, axf_bash, axf_makefile, axf_json, axf_yaml - serialization formats for assignments
  - **axrg_* (Axial Regime)**: New category for configuration system patterns
    - axrg_regime: structured config system with spec/validation/rendering
    - axrg_variable: named config element at schema level (replaces "slot")
    - axrg_assignment: specific value bound to variable at instance level
  - Reviewed CRR (crg-CRR-ConfigRegimeRequirements.adoc) for regime semantics

## Paces

- [ ] **P1: Regime Definitions** (manual)
  Add voicings to *_regime definitions in RBAGS.
  Voice pattern: `// ⟦axl_voices axrg_regime axf_bash⟧`
  (All RBAGS regimes use bash-sourceable .env format)

- [ ] **P2: Regime Variable Definitions** (manual)
  Add voicings to individual regime variables (RBRR_*, RBRA_*, RBRP_*, RBRO_*).
  Voice pattern: `// ⟦axl_voices axrg_variable axt_string⟧` (with appropriate type)

- [ ] **P3: GCP and Infrastructure Definitions** (manual)
  Add voicings to gcp_*, giam_*, gar_*, gcs_*, gcb_*, rbtgi_* definitions.
  Need to determine appropriate motifs for cloud infrastructure concepts.

- [ ] **P4: Cross-Reference and Support Definitions** (manual)
  Add voicings to at_* cross-reference definitions.

- [ ] **P5: Review and Refine** (manual)
  Review all voicings for consistency. Document friction points.

## Architectural Insights (This Session)

### Type Model Layering
- **Schema level**: axrg_variable defines what can be configured
- **Instance level**: axrg_assignment binds values to variables
- **Carriage**: axf_* formats specify how assignments are serialized

### Category Clarity
- axrg_* = Axial Regime (configuration systems)
- axf_* = Axial Format (serialization, reusable beyond regimes)
- axe_* = Axial Environment (execution contexts for operations)
- axo_* = Axial Operation (command, guide, pattern, identity, actor, dependency)

### Key Design Decisions
1. "Variable" preferred over "slot" - aligns with CRR vocabulary
2. Format (axf_) separate from regime (axrg_) - formats are reusable
3. Actors don't carry "human required" - that's on their operations
4. Regimes are bash-carried in RBAGS (axf_bash dimension)

## Itches Created

1. **itch-crg-to-cmk.md**: Move CRG to concept-model-kit.md as sibling pattern to MCM
2. **itch-axo-relevel.md**: axo_* has accumulated disparate concepts (operation types, identity, dependency) - consider releveling

## Notes

- Annotation format: `// ⟦axl_voices primary [dimension ...]⟧`
- Annotations go between `[[anchor]]` and `{term}::` lines
- CRR document (crg-CRR-ConfigRegimeRequirements.adoc) is authoritative for regime patterns
- Next session: Start with P1 (regime definitions), then P2 (regime variables)
