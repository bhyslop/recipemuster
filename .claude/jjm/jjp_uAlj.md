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

Subdocuments open with //axhob_operation + operation linked term.

### Detail-site lookahead policy (settled in ₢AjAAg)

| Marker | Lookahead | Notes |
|--------|-----------|-------|
| `axhob_operation` | Yes | Operation linked term |
| `axhop_parameter_from_type` | Yes | Reuses existing domain type term (zero new terms) |
| `axhop_parameter_from_arg` | Yes | Reuses shared CLI argument term (small vocabulary per spec) |
| `axhos_step` | No | Bare marker, straight to content |
| `axhos_waymark` | Local anchor only | Rare; branch-targetable step. No S0 elevation |
| `axhoq_precondition` | No | |
| `axhoo_output_of_type` | Yes | Reuses existing domain type term (parallel to parameter pattern) |
| `axhog_guarantee` | No | |
| `axhoc_completion` | No | |

Markers with lookahead reference existing domain terms — they do NOT mint
operation-specific S0 terms. The exemplar paces (AAE/AAF) pre-date this
policy and have 34 wrong-elevation terms that require a repair pace.

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
- Step labels / branching — handled via axhos_waymark (settled in ₢AjAAg)
- Diptych syntax (₣AZ) — adjacent heat, connection noted in ₣AZ paddock
- Non-rbtgo operations in RBS0 (mkr_*, opss_*, opbs_*, opbr_*, scr_*)

## Subdocument Transformation Notes

The exemplar paces (₢AjAAE depot_create, ₢AjAAF depot_initialize) pre-date the
settled lookahead policy and contain 34 wrong-elevation terms. A repair pace
will apply the corrected policy to these two subdocuments.

Subsequent subdocument transformations follow the settled policy above:
parameters and outputs get lookaheads referencing existing domain terms;
steps, preconditions, guarantees, and completion do not. This makes most
transformations more mechanical — later paces may be bridleable.

Depot_initialize is the attended exemplar: it has preconditions, human-interactive
OAuth flow, and branch targets (handled via `axhos_waymark` with local anchors).

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