# Heat Trophy: axla-procedure-section-motifs

**Firemark:** ₣AB
**Created:** 260116
**Retired:** 260116
**Status:** retired

## Paddock

# Paddock: axla-procedure-section-motifs

## Context

Introduce a formal taxonomy for procedural articulations in AXLA, establishing `axo_procedure` as the superset motif and `axs_` as a new category for documentation section motifs.

### Problem Statement

AXLA has incrementally accumulated procedural terms (`axo_command`, `axo_guide`, `axo_pattern`, `axo_sequence`) without a unifying superset. JJD uses extensive voicings on CLI operations with structured sections (Arguments, Behavior, Exit Status), while RBAGS operations lack this rigor. There's no formal vocabulary for "what sections does a documented procedure require?"

### Key Insights

**Dimensional Analysis**: Procedural articulations have orthogonal concerns:
- **Executor**: machine (command) vs human (guide)
- **Structure**: atomic vs sequential vs branching
- **Reusability**: terminal vs composite vs pattern
- **Interface**: CLI, REST, console, programmatic
- **Lifecycle**: transient (executes and completes), long-running (continues until stopped), periodic (executes on schedule) — TBD how to represent

**The Superset**: `axo_procedure` captures what ALL procedural articulations share:
- Identity (name, description)
- Inputs (what it needs)
- Behavior (what happens)
- Outputs (what it produces)
- Completion (how it ends)

**Section Motifs**: New `axs_` category for Axial Section motifs:
- `axs_inputs` - what procedure requires (arguments, parameters)
- `axs_preconditions` - environmental state requirements before execution (optional)
- `axs_behavior` - what procedure does (steps)
- `axs_outputs` - what procedure produces (results)
- `axs_postconditions` - environmental state guarantees after success (optional)
- `axs_completion` - how procedure ends (success/failure criteria)
- `axs_errors` - failure modes (optional)

**Scope Exclusion**: `axo_procedure` is the superset for procedural articulations ONLY. These `axo_` terms are NOT procedures and remain outside the hierarchy:
- `axo_entity` - objects with structure and behavior (builder patterns, registries)
- `axo_role`, `axo_identity`, `axo_actor` - identity patterns
- `axo_dependency` - infrastructure requirements

**Structural vs Procedural**: `axo_sequence` describes HOW behavior is organized (ordered steps), not WHAT kind of procedure something is. Most procedures ARE sequential. Thus `axo_sequence` is a structural characteristic, not a peer of `axo_command`/`axo_guide`.

**Naming Constraint**: Cannot use `axos_` because `axo_sequence` exists and terminal exclusivity forbids a prefix having both direct children and child prefixes. Using `axs_` (Axial Section) as separate top-level category.

**Compliance Cascade**: Rules flow from general to specific:
- `axo_procedure` requires: Inputs, Behavior, Outputs, Completion sections
- `axo_command` adds: environment dimension required
- `axo_guide` adds: human-guide control voicings in steps
- `axi_cli_subcommand` adds: Arguments voiced as axa_argument_list, Completion voiced as axa_exit_*

### Voicing Relationships

Interface-specific sections voice general sections:
- `axa_argument_list` voices `axs_inputs` in CLI context
- `axa_exit_uniform`/`axa_exit_enumerated` voices `axs_completion` in CLI context
- `jjds_arguments` voices `axa_argument_list` voices `axs_inputs` (document-local)

## References

- `Tools/cmk/AXLA-Lexicon.adoc` - Target for new motifs
- `Tools/cmk/MCM-MetaConceptModel.adoc` - Form patterns (mcm_form_deflist, mcm_form_section)
- `Tools/jjk/JJD-GallopsData.adoc` - Example of well-structured CLI operations with jjds_* section headers
- `lenses/RBAGS-AdminGoogleSpec.adoc` - Operations needing structure (axo_command, axo_guide)

## Future Work (Out of Scope)

- Retrofit RBAGS operations to comply with new section structure
- Retrofit RBAGS patterns (rbtoe_*) to use axs_* voicings
- Consider REST API section motifs (request/response voicing axs_inputs/axs_outputs)

## Paces

### axla-mapping-section-updates (₢ABAAA) [complete]

**[260116-0913] complete**

Agent: haiku
Strategy: Add mapping section entries only (not definitions). Three edits:
1. Line ~16: Add category declaration: // axs_: Axial Section (documentation section motifs) → mcm_form_deflist
2. After line 157 (axo_sequence_s): Add :axo_procedure: and :axo_procedure_s: mappings
3. After line 165 (axo_dependency_s): Add // Axial Section Terms comment block with axs_inputs, axs_preconditions, axs_behavior, axs_outputs, axs_postconditions, axs_completion, axs_errors mappings
Key files: Tools/cmk/AXLA-Lexicon.adoc
Notes: Follow existing patterns exactly. No definition text - mappings only.

**[260116-0912] primed**

Agent: haiku
Strategy: Add mapping section entries only (not definitions). Three edits:
1. Line ~16: Add category declaration: // axs_: Axial Section (documentation section motifs) → mcm_form_deflist
2. After line 157 (axo_sequence_s): Add :axo_procedure: and :axo_procedure_s: mappings
3. After line 165 (axo_dependency_s): Add // Axial Section Terms comment block with axs_inputs, axs_preconditions, axs_behavior, axs_outputs, axs_postconditions, axs_completion, axs_errors mappings
Key files: Tools/cmk/AXLA-Lexicon.adoc
Notes: Follow existing patterns exactly. No definition text - mappings only.

*Direction:* -

**[260116-0814] rough**

Add axo_procedure definition and axs_ category declaration to AXLA mapping section header

### add-axs-form-expectations (₢ABAAF) [complete]

**[260116-0914] complete**

Agent: haiku
Strategy: Add single bullet to Form Expectations section (around line 386-389).
1. Add: * {axs_*} section terms expect {xref_MCM} `mcm_form_section` for section headers
Key files: Tools/cmk/AXLA-Lexicon.adoc
Notes: Follow existing bullet pattern in Form Expectations section. Single mechanical addition.

**[260116-0912] primed**

Agent: haiku
Strategy: Add single bullet to Form Expectations section (around line 386-389).
1. Add: * {axs_*} section terms expect {xref_MCM} `mcm_form_section` for section headers
Key files: Tools/cmk/AXLA-Lexicon.adoc
Notes: Follow existing bullet pattern in Form Expectations section. Single mechanical addition.

*Direction:* -

**[260116-0814] rough**

Add Form Expectations for axs_* category in AXLA (likely mcm_form_section for most section voicings)

### update-jjd-section-voicings (₢ABAAG) [complete]

**[260116-0915] complete**

Agent: haiku
Strategy: Update three existing definitions to add voicing annotations:
1. axa_argument_list (line ~954): Add note that it voices axs_inputs in CLI context
2. axa_exit_uniform (line ~1013): Add note that it voices axs_completion in CLI context
3. axa_exit_enumerated (line ~1041): Add note that it voices axs_completion in CLI context
Key files: Tools/cmk/AXLA-Lexicon.adoc
Notes: Add voicing relationship to definition prose, not annotations. Keep existing content.

**[260116-0912] primed**

Agent: haiku
Strategy: Update three existing definitions to add voicing annotations:
1. axa_argument_list (line ~954): Add note that it voices axs_inputs in CLI context
2. axa_exit_uniform (line ~1013): Add note that it voices axs_completion in CLI context
3. axa_exit_enumerated (line ~1041): Add note that it voices axs_completion in CLI context
Key files: Tools/cmk/AXLA-Lexicon.adoc
Notes: Add voicing relationship to definition prose, not annotations. Keep existing content.

*Direction:* -

**[260116-0820] rough**

Clarification: We are NOT changing JJD annotations - voicing is to immediate parent only. The change is in AXLA: update axa_argument_list to voice axs_inputs, update axa_exit_uniform/enumerated to voice axs_completion. JJD's jjds_arguments continues to voice axa_argument_list; the chain to axs_inputs is implicit through AXLA. Verify JJD annotations remain correct after AXLA changes.

**[260116-0814] rough**

Update JJD jjds_* term annotations to show voicing relationship: jjds_arguments voices axa_argument_list (which voices axs_inputs), jjds_exit_* voices axa_exit_* (which voices axs_completion)

### axs-section-motif-definitions (₢ABAAB) [complete]

**[260116-0932] complete**

EXPANDED: Define ALL axs_* section motifs: axs_inputs, axs_preconditions (optional), axs_behavior, axs_outputs, axs_postconditions (optional), axs_completion, axs_errors (optional). Include guidance on interface-specific voicings for each.

**[260116-0827] rough**

EXPANDED: Define ALL axs_* section motifs: axs_inputs, axs_preconditions (optional), axs_behavior, axs_outputs, axs_postconditions (optional), axs_completion, axs_errors (optional). Include guidance on interface-specific voicings for each.

**[260116-0814] rough**

Define axs_inputs, axs_behavior, axs_outputs, axs_completion, axs_errors section motifs with guidance on interface-specific voicings

### refactor-procedural-term-definitions (₢ABAAC) [complete]

**[260116-0934] complete**

Refactor axo_command, axo_guide, axo_pattern definitions to reference axo_procedure as superset. Critically: reposition axo_sequence as STRUCTURAL CHARACTERISTIC not procedure type - most procedures ARE sequential, so axo_sequence describes HOW behavior is organized, not WHAT kind of procedure it is. Also explicitly note that axo_entity, axo_role, axo_identity, axo_actor, axo_dependency are OUTSIDE the procedure hierarchy.

**[260116-0820] rough**

Refactor axo_command, axo_guide, axo_pattern definitions to reference axo_procedure as superset. Critically: reposition axo_sequence as STRUCTURAL CHARACTERISTIC not procedure type - most procedures ARE sequential, so axo_sequence describes HOW behavior is organized, not WHAT kind of procedure it is. Also explicitly note that axo_entity, axo_role, axo_identity, axo_actor, axo_dependency are OUTSIDE the procedure hierarchy.

**[260116-0814] rough**

Refactor axo_command, axo_guide, axo_pattern definitions to reference axo_procedure as superset; clarify axo_sequence as structural characteristic

### add-procedure-compliance-rules (₢ABAAD) [complete]

**[260116-0935] complete**

Add Compliance Rules: Procedure Documentation Completeness, Command Documentation Completeness, Guide Documentation Completeness

**[260116-0814] rough**

Add Compliance Rules: Procedure Documentation Completeness, Command Documentation Completeness, Guide Documentation Completeness

### contextualize-cli-compliance-rules (₢ABAAE) [complete]

**[260116-0936] complete**

Contextualize existing CLI Subcommand Completeness rule to show axa_argument_list voices axs_inputs and axa_exit_* voices axs_completion

**[260116-0814] rough**

Contextualize existing CLI Subcommand Completeness rule to show axa_argument_list voices axs_inputs and axa_exit_* voices axs_completion

### review-mcm-compatibility (₢ABAAH) [complete]

**[260116-0937] complete**

Review MCM for any needed updates: verify form patterns support procedure section structure, confirm annotation grammar handles voicing chains

**[260116-0814] rough**

Review MCM for any needed updates: verify form patterns support procedure section structure, confirm annotation grammar handles voicing chains

### propose-lifecycle-dimension (₢ABAAJ) [complete]

**[260116-0948] complete**

Propose lifecycle dimension for procedures: transient (executes and completes), long-running (continues until stopped), periodic (executes on schedule). Determine if this is a dimension annotation, a new category, or section content. Iterate with editor on design.

**[260116-0948] complete**

Propose lifecycle dimension for procedures: transient (executes and completes), long-running (continues until stopped), periodic (executes on schedule). Determine if this is a dimension annotation, a new category, or section content. Iterate with editor on design.

**[260116-0820] rough**

Propose lifecycle dimension for procedures: transient (executes and completes), long-running (continues until stopped), periodic (executes on schedule). Determine if this is a dimension annotation, a new category, or section content. Iterate with editor on design.

### reconsider-rbags-retrofit-options (₢ABAAK) [complete]

**[260116-1021] complete**

Option B retrofit complete: minted axd_none, annotated 14 RBAGS files with parallel Haiku agents, established section annotation pattern.

**[260116-1021] rough**

COMPLETED: Option B annotation-only retrofit. Minted axd_none dimension for procedures with no explicit inputs. Annotated 14 RBAGS operation files (axs_inputs, axs_behavior, axs_outputs, axs_completion). Used parallel Haiku agents in batches. Pattern: annotations mark section locations, completion criteria as document prose. Skipped RBSOB (trade study). Commit: e3310ec.

**[260116-0838] rough**

Reconsider RBAGS retrofit options after AXLA taxonomy is complete. Option B (Haiku, annotation-only, ~1-2 hrs) vs Option A (Sonnet, full restructure, ~3-5 hrs). Both parallelizable across 16 files + 11 inline patterns. Decide based on how compliance rules turned out and whether explicit section structure adds enough value over metadata-only approach.

### add-precondition-postcondition-sections (₢ABAAI) [abandoned]

**[260116-0936] abandoned**

SUPERSEDED: Content merged into pace ABAAB (axs-section-motif-definitions). Skip this pace.

**[260116-0827] rough**

SUPERSEDED: Content merged into pace ABAAB (axs-section-motif-definitions). Skip this pace.

**[260116-0820] rough**

Add axs_preconditions (environmental state requirements before execution) and axs_postconditions (environmental state guarantees after success) as optional section motifs, distinct from inputs/outputs

## Steeplechase

(no entries)

