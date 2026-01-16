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

**The Superset**: `axo_procedure` captures what ALL procedural articulations share:
- Identity (name, description)
- Inputs (what it needs)
- Behavior (what happens)
- Outputs (what it produces)
- Completion (how it ends)

**Section Motifs**: New `axs_` category for Axial Section motifs:
- `axs_inputs` - what procedure requires
- `axs_behavior` - what procedure does
- `axs_outputs` - what procedure produces
- `axs_completion` - how procedure ends
- `axs_errors` - failure modes (optional)

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
