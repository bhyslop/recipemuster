# Paddock: rbk-axla-term-voicing-scrub

## Goal

Replace AXLA's fragmented operation vocabulary with the new model based on
axo_procedure/axo_method duality, operation groups, and detail-site hierarchy markers.
Transform all specification documents from legacy patterns to new patterns.

## Design Authority

All new terms and their specifications live in AXLA-Lexicon.adoc itself.
The deprecation appendix in AXLA maps old terms to new replacements.
This paddock provides execution context only — not term definitions.

## Key AXLA Sections for Pace Execution

- **Definition-site voicings** (axvo_*): procedure, method, group — see AXLA §Definition-Site Voicing Annotations
- **Detail-site markers** (axho_*): operation, parameter, step, output, precondition, guarantee, completion — see AXLA §Hierarchy Operation Markers
- **New dimensions**: axd_attended, axd_internal, axd_grouped, axd_tabtarget, axd_slash_command — see AXLA definitions
- **Deprecation appendix**: old→new mapping at end of AXLA

## Annotation Line Purity Rule

Annotation lines contain ONLY AXLA terms. Project-specific linked terms appear in
definition text via positional requirements imposed by voicings. This matches the
established regime pattern (axvr_variable requires parent regime as 2nd linked term
in definition text).

## Transformation Patterns

### S0 definition-site: old → new

```
// ⟦axl_voices axo_command axe_bash_interactive⟧    →  // ⟦axvo_method axd_transient⟧
// ⟦axl_voices axo_guide axe_human_guide⟧           →  // ⟦axvo_method axd_transient axd_attended⟧
// ⟦axl_voices axo_routine axe_bash_interactive⟧    →  // ⟦axvo_method axd_transient axd_internal⟧  (or axvo_procedure if standalone)
// ⟦axl_voices axo_routine axe_bash_scripted⟧       →  // ⟦axvo_procedure axd_transient axd_internal⟧
```

### Subdocument detail-site: old → new

```
// ⟦axs_inputs⟧          →  individual //axhop_parameter markers
// ⟦axs_preconditions⟧   →  individual //axhoq_precondition markers
// ⟦axs_behavior⟧        →  individual //axhos_step markers
// ⟦axs_outputs⟧         →  individual //axhoo_output markers
// ⟦axs_postconditions⟧  →  individual //axhog_guarantee markers
// ⟦axs_completion⟧      →  //axhoc_completion
```

Each individual marker requires a project-specific linked term on the next line (lookahead).
Subdocuments open with //axhob_operation + operation linked term.

### Regime subdocument bracket repair

```
// ⟦axhrb_regime⟧   →  //axhrb_regime
// ⟦axhrgv_variable⟧ →  //axhrgv_variable
```

Remove space and Strachey brackets from all regime hierarchy markers in all 7 regime subdocs.

## Scope Boundaries

**In scope**: RBS0 operations and subdocuments, VOS0 operations, JJS0 routines,
BUS0 verification, regime subdocument bracket syntax, AXLA completeness checklists,
legacy term deletion.

**Out of scope (separate heats)**:
- Regime operation markers (axhro_kindle etc.) — future reconciliation
- Control terms (axc_*/rbbc_*) — step-level vocabulary
- Step labels / branching — prose handles branching
- Diptych syntax (₣AZ) — adjacent heat, connection noted in ₣AZ paddock
- Non-rbtgo operations in RBS0 (mkr_*, opss_*, opbs_*, opbr_*, scr_*)

## Subdocument Transformation Notes

The exemplar paces (₢AjAAE depot_create, ₢AjAAF depot_initialize) establish the
pattern. Subsequent subdocument transformations follow that pattern.

Each subdocument transformation requires minting linked terms for parameters, steps,
and outputs. These are new vocabulary — not mechanical replacement. The exemplar
paces are interactive (rough); later paces may be bridleable once the pattern is proven.

Depot_initialize is the attended exemplar: it has preconditions, human-interactive
OAuth flow, and "go to step N" branching (handled as prose cross-references to
step linked terms).

## Cross-cutting Decisions

**rbtoe_* internal routines**: Role-specific routines (rbtoe_payor_authenticate etc.)
affiliate with their role's group as axvo_method axd_internal. Cross-cutting utilities
(rbtoe_jwt_oauth_exchange, rbtoe_rbra_load) become axvo_procedure axd_internal.

**VOS0 ghost axo_operation**: Three uses (allocate, invitatory, release) reference a
term never defined in AXLA. Determine correct voicing in new model during VOS0 pace.

**JJS0 interface layer**: axi_cc_slash_command, axi_cli_subcommand etc. are interface
voicings not operation voicings — may be unaffected. Review and confirm.

## References

- `Tools/cmk/vov_veiled/AXLA-Lexicon.adoc` — design authority (new terms + deprecation appendix)
- `lenses/RBSRR-RegimeRepo.adoc` — regime subdoc pattern (the precedent for markers)
- ₣AZ `cmk-diptych-prototype` — adjacent heat (syntax layer)