# Heat Trophy: rbk-axla-term-voicing-scrub

**Firemark:** ₣Aj
**Created:** 260223
**Retired:** 260310
**Status:** retired

## Paddock

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

## Paces

### repair-regime-subdoc-brackets (₢AjAAA) [complete]

**[260227-1407] complete**

Convert all 7 regime subdocuments from Strachey bracket form
`// ⟦axhrb_regime⟧` to bare prefix form `//axhrb_regime` per AXLA spec.

Files: RBSRR-RegimeRepo.adoc, RBSRA-CredentialFormat.adoc, RBSRP-RegimePayor.adoc,
RBSRO-RegimeOauth.adoc, RBSRS-RegimeStation.adoc, RBSRV-RegimeVessel.adoc,
RBRN-RegimeNameplate.adoc.

All markers: axhrb_regime, axhrgb_group, axhrgv_variable, axhrv_variable,
axhrgc_gate, axhro_kindle, axhro_validate, axhro_render, axhro_list,
axhro_survey, axhro_audit.

Mechanical find-and-replace: `// ⟦axhrX_Y⟧` → `//axhrX_Y` (remove space, Strachey brackets).
Verify with grep that no Strachey bracket forms remain in any regime subdoc.

**[260227-1053] rough**

Convert all 7 regime subdocuments from Strachey bracket form
`// ⟦axhrb_regime⟧` to bare prefix form `//axhrb_regime` per AXLA spec.

Files: RBSRR-RegimeRepo.adoc, RBSRA-CredentialFormat.adoc, RBSRP-RegimePayor.adoc,
RBSRO-RegimeOauth.adoc, RBSRS-RegimeStation.adoc, RBSRV-RegimeVessel.adoc,
RBRN-RegimeNameplate.adoc.

All markers: axhrb_regime, axhrgb_group, axhrgv_variable, axhrv_variable,
axhrgc_gate, axhro_kindle, axhro_validate, axhro_render, axhro_list,
axhro_survey, axhro_audit.

Mechanical find-and-replace: `// ⟦axhrX_Y⟧` → `//axhrX_Y` (remove space, Strachey brackets).
Verify with grep that no Strachey bracket forms remain in any regime subdoc.

### define-operation-groups-in-s0 (₢AjAAB) [complete]

**[260227-1417] complete**

Create rbtgog_* operation group linked terms in RBS0-SpecTop.adoc using axvo_group voicings.

Groups to define (discover full list by scanning existing rbtgo_* definitions):
- rbtgog_depot — operations on {rbtge_depot}
- rbtgog_ark — operations on {rbtga_ark}
- rbtgog_image — operations on container images
- rbtgog_payor — operations by {rbtr_payor}
- rbtgog_governor — operations by {rbtr_governor}
- rbtgog_sa — service account operations
- rbtgog_rubric — rubric lifecycle operations
- rbtgog_access — authentication/probe operations
- Others as discovered during inventory

Each group: anchor + `//axvo_group` voicing (with axd_tabtarget where applicable) +
definition text with entity as 2nd linked term.

Place group definitions before their member operations in the S0 document structure.

**[260227-1054] rough**

Create rbtgog_* operation group linked terms in RBS0-SpecTop.adoc using axvo_group voicings.

Groups to define (discover full list by scanning existing rbtgo_* definitions):
- rbtgog_depot — operations on {rbtge_depot}
- rbtgog_ark — operations on {rbtga_ark}
- rbtgog_image — operations on container images
- rbtgog_payor — operations by {rbtr_payor}
- rbtgog_governor — operations by {rbtr_governor}
- rbtgog_sa — service account operations
- rbtgog_rubric — rubric lifecycle operations
- rbtgog_access — authentication/probe operations
- Others as discovered during inventory

Each group: anchor + `//axvo_group` voicing (with axd_tabtarget where applicable) +
definition text with entity as 2nd linked term.

Place group definitions before their member operations in the S0 document structure.

### transform-s0-definition-sites (₢AjAAC) [complete]

**[260227-1424] complete**

Convert all ~25 rbtgo_* operation annotations in RBS0-SpecTop.adoc from legacy
`// ⟦axl_voices axo_command axe_bash_interactive⟧` pattern to new voicings.

For each operation:
- Determine axvo_method (entity-affiliated) vs axvo_procedure (standalone)
- Assign lifecycle dimension (axd_transient for most)
- Add axd_attended where applicable (depot_initialize, payor_establish, payor_refresh, gdc_establish, quota_build)
- Add axd_grouped referencing the group from prior pace
- Ensure 2nd linked term in definition text is entity (for methods) or group (for grouped procedures)

Also convert ~5 guide annotations and ~8 sequence annotations.
Verify no legacy `axl_voices axo_command` or `axl_voices axo_guide` patterns remain.

**[260227-1054] rough**

Convert all ~25 rbtgo_* operation annotations in RBS0-SpecTop.adoc from legacy
`// ⟦axl_voices axo_command axe_bash_interactive⟧` pattern to new voicings.

For each operation:
- Determine axvo_method (entity-affiliated) vs axvo_procedure (standalone)
- Assign lifecycle dimension (axd_transient for most)
- Add axd_attended where applicable (depot_initialize, payor_establish, payor_refresh, gdc_establish, quota_build)
- Add axd_grouped referencing the group from prior pace
- Ensure 2nd linked term in definition text is entity (for methods) or group (for grouped procedures)

Also convert ~5 guide annotations and ~8 sequence annotations.
Verify no legacy `axl_voices axo_command` or `axl_voices axo_guide` patterns remain.

### design-rbtoe-internal-routines (₢AjAAD) [complete]

**[260227-1427] complete**

The ~13 rbtoe_* routines in RBS0 need group affiliation decisions.

Proposed split:
- Role-specific routines (rbtoe_payor_authenticate, rbtoe_director_authenticate, etc.)
  → axvo_method axd_transient axd_internal, affiliated with their role's group
- Cross-cutting utilities (rbtoe_jwt_oauth_exchange, rbtoe_rbra_load, rbtoe_oauth_refresh)
  → axvo_procedure axd_transient axd_internal (standalone, no entity)

Transform their annotations from `// ⟦axl_voices axo_routine axe_bash_interactive⟧`
to new voicings. Each routine needs individual assessment for entity affiliation.
Verify no legacy axo_routine annotations remain in RBS0.

**[260227-1054] rough**

The ~13 rbtoe_* routines in RBS0 need group affiliation decisions.

Proposed split:
- Role-specific routines (rbtoe_payor_authenticate, rbtoe_director_authenticate, etc.)
  → axvo_method axd_transient axd_internal, affiliated with their role's group
- Cross-cutting utilities (rbtoe_jwt_oauth_exchange, rbtoe_rbra_load, rbtoe_oauth_refresh)
  → axvo_procedure axd_transient axd_internal (standalone, no entity)

Transform their annotations from `// ⟦axl_voices axo_routine axe_bash_interactive⟧`
to new voicings. Each routine needs individual assessment for entity affiliation.
Verify no legacy axo_routine annotations remain in RBS0.

### transform-rbsdc-depot-create (₢AjAAE) [complete]

**[260227-1437] complete**

Exemplar transformation: convert RBSDC-depot_create.adoc from legacy axs_* section
markers to new axho_* individual hierarchy markers.

This is the largest operation subdocument (~205 lines) and serves as the pattern
for all subsequent subdocument transformations.

Transform:
- Add //axhob_operation opener with {rbtgo_depot_create} lookahead
- Replace `// ⟦axs_inputs⟧` with individual //axhop_parameter markers per parameter
- Replace `// ⟦axs_behavior⟧` with individual //axhos_step markers per step
- Replace `// ⟦axs_outputs⟧` with individual //axhoo_output markers per output
- Replace `// ⟦axs_completion⟧` with //axhoc_completion
- Add //axhoq_precondition and //axhog_guarantee where applicable

Each parameter, step, and output needs a project-specific linked term minted
(e.g., rbtgo_depot_create_name, rbtgo_depot_create_link_billing).
These linked terms must also be added to S0 or the subdoc mapping section.

Document the transformation pattern for subsequent paces to follow.

**[260227-1054] rough**

Exemplar transformation: convert RBSDC-depot_create.adoc from legacy axs_* section
markers to new axho_* individual hierarchy markers.

This is the largest operation subdocument (~205 lines) and serves as the pattern
for all subsequent subdocument transformations.

Transform:
- Add //axhob_operation opener with {rbtgo_depot_create} lookahead
- Replace `// ⟦axs_inputs⟧` with individual //axhop_parameter markers per parameter
- Replace `// ⟦axs_behavior⟧` with individual //axhos_step markers per step
- Replace `// ⟦axs_outputs⟧` with individual //axhoo_output markers per output
- Replace `// ⟦axs_completion⟧` with //axhoc_completion
- Add //axhoq_precondition and //axhog_guarantee where applicable

Each parameter, step, and output needs a project-specific linked term minted
(e.g., rbtgo_depot_create_name, rbtgo_depot_create_link_billing).
These linked terms must also be added to S0 or the subdoc mapping section.

Document the transformation pattern for subsequent paces to follow.

### transform-rbsdn-depot-initialize (₢AjAAF) [complete]

**[260301-0814] complete**

Exemplar transformation: convert RBSDN-depot_initialize.adoc (~140 lines) from legacy axs_* section markers to new axho_* hierarchy markers. Attended exemplar (axd_attended). Follow pattern from transform-rbsdc-depot-create (₢AjAAE). Has axs_preconditions, human-interactive OAuth flow, "Go to step 7" branching. Mint linked terms for steps, preconditions, outputs.

**[260227-1056] rough**

Exemplar transformation: convert RBSDN-depot_initialize.adoc (~140 lines) from legacy axs_* section markers to new axho_* hierarchy markers. Attended exemplar (axd_attended). Follow pattern from transform-rbsdc-depot-create (₢AjAAE). Has axs_preconditions, human-interactive OAuth flow, "Go to step 7" branching. Mint linked terms for steps, preconditions, outputs.

### clarify-detail-site-lookahead-policy (₢AjAAg) [complete]

**[260301-0840] complete**

Resume design clarification for axho_* marker lookahead policy. Captures
learning from ₢AjAAE and ₢AjAAF exemplar work.

## Resolved so far

- Parameters: DO need S0-level terms, but via two AXLA marker variants:
  - axhop_parameter_from_type — lookahead reuses existing domain type term (zero new terms)
  - axhop_parameter_from_arg — lookahead points at shared CLI argument term (JJ precedent: jjda_* pattern; small vocabulary per spec)
- Steps: do NOT need lookaheads (no current customer for lectionary extraction)
- Preconditions: do NOT need operation-specific S0 terms (dead weight)

## Still to resolve

- Outputs: same question as preconditions — do they need lookaheads?
- Guarantees: same question
- Completion: currently no lookahead (0 arity) — confirm this stays
- Branch targets: if steps don't get lookaheads, how do cross-references
  like "Go to step 7" work? Local anchors? Inline labels? Different marker?
- What to do with the 34 wrong-elevation terms from AAE/AAF — repair pace
  scope and timing
- Paddock update: revise "minting linked terms for parameters, steps, and
  outputs" premise once policy is settled
- AXLA spec update: define new parameter marker variants, remove lookahead
  requirement from step/precondition markers — separate pace or same pace?

## Method

Interactive discussion to complete the resolution list, then hand off to
an AXLA update pace and a repair pace for AAE/AAF.

**[260227-1529] rough**

Resume design clarification for axho_* marker lookahead policy. Captures
learning from ₢AjAAE and ₢AjAAF exemplar work.

## Resolved so far

- Parameters: DO need S0-level terms, but via two AXLA marker variants:
  - axhop_parameter_from_type — lookahead reuses existing domain type term (zero new terms)
  - axhop_parameter_from_arg — lookahead points at shared CLI argument term (JJ precedent: jjda_* pattern; small vocabulary per spec)
- Steps: do NOT need lookaheads (no current customer for lectionary extraction)
- Preconditions: do NOT need operation-specific S0 terms (dead weight)

## Still to resolve

- Outputs: same question as preconditions — do they need lookaheads?
- Guarantees: same question
- Completion: currently no lookahead (0 arity) — confirm this stays
- Branch targets: if steps don't get lookaheads, how do cross-references
  like "Go to step 7" work? Local anchors? Inline labels? Different marker?
- What to do with the 34 wrong-elevation terms from AAE/AAF — repair pace
  scope and timing
- Paddock update: revise "minting linked terms for parameters, steps, and
  outputs" premise once policy is settled
- AXLA spec update: define new parameter marker variants, remove lookahead
  requirement from step/precondition markers — separate pace or same pace?

## Method

Interactive discussion to complete the resolution list, then hand off to
an AXLA update pace and a repair pace for AAE/AAF.

### update-axla-lookahead-spec (₢AjAAh) [complete]

**[260301-0900] complete**

Define new marker variants and codify the settled lookahead policy in AXLA-Lexicon.adoc.

## New markers to define
- axhop_parameter_from_type — parameter lookahead reuses existing domain type term
- axhop_parameter_from_arg — parameter lookahead reuses shared CLI argument term
- axhos_waymark — branch-targetable step with local anchor (no S0 elevation)
- axhoo_output_of_type — output lookahead reuses existing domain type term

## Policy changes to codify
- axhos_step: no lookahead (bare marker)
- axhoq_precondition: no lookahead
- axhog_guarantee: no lookahead
- axhoc_completion: no lookahead
- Remove any existing lookahead requirement from step/precondition/guarantee/completion marker definitions

## Design authority
- Settled in ₢AjAAg (clarify-detail-site-lookahead-policy)
- Paddock lookahead policy table is the source of truth

**[260301-0838] rough**

Define new marker variants and codify the settled lookahead policy in AXLA-Lexicon.adoc.

## New markers to define
- axhop_parameter_from_type — parameter lookahead reuses existing domain type term
- axhop_parameter_from_arg — parameter lookahead reuses shared CLI argument term
- axhos_waymark — branch-targetable step with local anchor (no S0 elevation)
- axhoo_output_of_type — output lookahead reuses existing domain type term

## Policy changes to codify
- axhos_step: no lookahead (bare marker)
- axhoq_precondition: no lookahead
- axhog_guarantee: no lookahead
- axhoc_completion: no lookahead
- Remove any existing lookahead requirement from step/precondition/guarantee/completion marker definitions

## Design authority
- Settled in ₢AjAAg (clarify-detail-site-lookahead-policy)
- Paddock lookahead policy table is the source of truth

### repair-exemplar-wrong-elevation-terms (₢AjAAi) [complete]

**[260301-0908] complete**

Remove 34 wrong-elevation terms from AAE (depot_create) and AAF (depot_initialize)
exemplar subdocuments and their S0 mappings. Apply the settled lookahead policy.

## Scope
- RBSDC-depot_create.adoc: ~18 wrong-elevation terms (step + output terms)
- RBSDN-depot_initialize.adoc: ~16 wrong-elevation terms (step + precondition + output terms)
- RBS0-SpecTop.adoc: remove corresponding attribute mappings

## Transformations
- Steps: remove [[anchor]] and {attribute} lines, leave bare //axhos_step + content
- Preconditions: remove [[anchor]] and {attribute} lines, leave bare //axhoq_precondition + content
- Waymark steps (branch targets in RBSDN): convert to //axhos_waymark with local anchor, no S0 mapping
- Parameters: convert to axhop_parameter_from_type or axhop_parameter_from_arg with existing domain terms
- Outputs: convert to axhoo_output_of_type with existing domain terms
- Completion: remove any [[anchor]] and {attribute} lines if present

## Dependencies
- Requires ₢AjAAh (update-axla-lookahead-spec) to be complete first

**[260301-0839] rough**

Remove 34 wrong-elevation terms from AAE (depot_create) and AAF (depot_initialize)
exemplar subdocuments and their S0 mappings. Apply the settled lookahead policy.

## Scope
- RBSDC-depot_create.adoc: ~18 wrong-elevation terms (step + output terms)
- RBSDN-depot_initialize.adoc: ~16 wrong-elevation terms (step + precondition + output terms)
- RBS0-SpecTop.adoc: remove corresponding attribute mappings

## Transformations
- Steps: remove [[anchor]] and {attribute} lines, leave bare //axhos_step + content
- Preconditions: remove [[anchor]] and {attribute} lines, leave bare //axhoq_precondition + content
- Waymark steps (branch targets in RBSDN): convert to //axhos_waymark with local anchor, no S0 mapping
- Parameters: convert to axhop_parameter_from_type or axhop_parameter_from_arg with existing domain terms
- Outputs: convert to axhoo_output_of_type with existing domain terms
- Completion: remove any [[anchor]] and {attribute} lines if present

## Dependencies
- Requires ₢AjAAh (update-axla-lookahead-spec) to be complete first

### transform-payor-subdocs (₢AjAAG) [complete]

**[260301-0916] complete**

Convert RBSPE-payor_establish.adoc, RBSPI-payor_install.adoc, RBSPR-payor_refresh.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern. Payor establish and refresh are attended.

**[260227-1056] rough**

Convert RBSPE-payor_establish.adoc, RBSPI-payor_install.adoc, RBSPR-payor_refresh.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern. Payor establish and refresh are attended.

### transform-rbsgr-governor-reset (₢AjAAH) [complete]

**[260301-0925] complete**

Convert RBSGR-governor_reset.adoc from legacy axs_* to axho_* markers.

**[260227-1056] rough**

Convert RBSGR-governor_reset.adoc from legacy axs_* to axho_* markers.

### transform-rbsrc-retriever-create (₢AjAAI) [complete]

**[260301-0928] complete**

Convert RBSRC-retriever_create.adoc from legacy axs_* to axho_* markers.

**[260227-1056] rough**

Convert RBSRC-retriever_create.adoc from legacy axs_* to axho_* markers.

### transform-rbsdi-director-create (₢AjAAJ) [complete]

**[260301-0931] complete**

Convert RBSDI-director_create.adoc from legacy axs_* to axho_* markers.

**[260227-1057] rough**

Convert RBSDI-director_create.adoc from legacy axs_* to axho_* markers.

### transform-rbsgd-gdc-establish (₢AjAAK) [complete]

**[260301-0932] complete**

Convert RBSGD-gdc_establish.adoc from legacy axs_* to axho_* markers.

**[260227-1057] rough**

Convert RBSGD-gdc_establish.adoc from legacy axs_* to axho_* markers.

### transform-rbsri-rubric-inscribe (₢AjAAO) [complete]

**[260301-0938] complete**

Convert RBSRI-rubric_inscribe.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

**[260227-1057] rough**

Convert RBSRI-rubric_inscribe.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

### transform-rbstb-trigger-build (₢AjAAS) [complete]

**[260301-0944] complete**

Convert RBSTB-trigger_build.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

**[260301-0939] bridled**

Convert RBSTB-trigger_build.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

*Direction:* Agent: sonnet

## Context
Read the paddock at .claude/jjm/jjp_uAlj.md for transformation rules.
Read exemplar: lenses/RBSDC-depot_create.adoc (fully transformed subdoc with parameters, steps, outputs).
Read exemplar: lenses/RBSPI-payor_install.adoc (transformed subdoc with preconditions).

## Steps
1. Read lenses/RBSTB-trigger_build.adoc
2. Grep RBS0-SpecTop.adoc for rbtgo_trigger_build to find the operation linked term
3. Apply transformation: add //axhob_operation + {rbtgo_trigger_build} header, convert axs_* Strachey brackets to axho_* bare markers per paddock policy (inputs→parameters, preconditions→axhoq_precondition, behavior→axhos_step, outputs→axhoo_output_of_type with domain terms, completion→axhoc_completion). Remove empty axs_inputs if axd_none.
4. Write the transformed file

## Verification
Grep the transformed file for '⟦axs_' — must return zero matches.

**[260227-1058] rough**

Convert RBSTB-trigger_build.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

### transform-rbsqb-quota-build (₢AjAAW) [complete]

**[260301-0945] complete**

Convert RBSQB-quota_build.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

**[260301-0939] bridled**

Convert RBSQB-quota_build.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

*Direction:* Agent: sonnet

## Context
Read the paddock at .claude/jjm/jjp_uAlj.md for transformation rules.
Read exemplar: lenses/RBSDC-depot_create.adoc (fully transformed subdoc with parameters, steps, outputs).
Read exemplar: lenses/RBSPI-payor_install.adoc (transformed subdoc with preconditions).

## Steps
1. Read lenses/RBSQB-quota_build.adoc
2. Grep RBS0-SpecTop.adoc for rbtgo_quota_build to find the operation linked term
3. Apply transformation per paddock policy
4. Write the transformed file

## Verification
Grep the transformed file for '⟦axs_' — must return zero matches.

**[260227-1058] rough**

Convert RBSQB-quota_build.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

### transform-depot-minor-subdocs (₢AjAAa) [complete]

**[260301-0948] complete**

Convert RBSDD-depot_destroy.adoc and RBSDL-depot_list.adoc from legacy axs_* to axho_* markers. Small subdocs, batch together.

**[260301-0940] bridled**

Convert RBSDD-depot_destroy.adoc and RBSDL-depot_list.adoc from legacy axs_* to axho_* markers. Small subdocs, batch together.

*Direction:* Agent: sonnet

## Context
Read the paddock at .claude/jjm/jjp_uAlj.md for transformation rules.
Read exemplar: lenses/RBSDC-depot_create.adoc (fully transformed subdoc).
Read exemplar: lenses/RBSRC-retriever_create.adoc (small transformed subdoc).

## Steps
1. Read lenses/RBSDD-depot_destroy.adoc and lenses/RBSDL-depot_list.adoc
2. Grep RBS0-SpecTop.adoc for rbtgo_depot_destroy and rbtgo_depot_list to find operation linked terms
3. Apply transformation to both files per paddock policy
4. Write both transformed files

## Verification
Grep both transformed files for '⟦axs_' — must return zero matches.

**[260227-1058] rough**

Convert RBSDD-depot_destroy.adoc and RBSDL-depot_list.adoc from legacy axs_* to axho_* markers. Small subdocs, batch together.

### transform-ark-subdocs (₢AjAAN) [complete]

**[260301-0950] complete**

Convert RBSAA-ark_abjure.adoc, RBSAB-ark_beseech.adoc, RBSAC-ark_conjure.adoc, RBSAS-ark_summon.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

**[260301-0941] bridled**

Convert RBSAA-ark_abjure.adoc, RBSAB-ark_beseech.adoc, RBSAC-ark_conjure.adoc, RBSAS-ark_summon.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

*Direction:* Agent: sonnet

## Context
Read the paddock at .claude/jjm/jjp_uAlj.md for transformation rules.
Read exemplar: lenses/RBSDC-depot_create.adoc (fully transformed subdoc with parameters, steps, outputs).
Read exemplar: lenses/RBSPI-payor_install.adoc (transformed subdoc with preconditions).

## Steps
1. Read all 4 ark subdocs: lenses/RBSAA-ark_abjure.adoc, lenses/RBSAB-ark_beseech.adoc, lenses/RBSAC-ark_conjure.adoc, lenses/RBSAS-ark_summon.adoc
2. Grep RBS0-SpecTop.adoc for rbtgo_ark_abjure, rbtgo_ark_beseech, rbtgo_ark_conjure, rbtgo_ark_summon to find operation linked terms
3. Apply transformation to all 4 files per paddock policy
4. Write all 4 transformed files

## Verification
Grep all 4 transformed files for '⟦axs_' — must return zero matches.

**[260227-1057] rough**

Convert RBSAA-ark_abjure.adoc, RBSAB-ark_beseech.adoc, RBSAC-ark_conjure.adoc, RBSAS-ark_summon.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

### transform-image-subdocs (₢AjAAR) [complete]

**[260301-0954] complete**

Convert RBSIL-image_list.adoc, RBSID-image_delete.adoc, RBSIR-image_retrieve.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

**[260301-0941] bridled**

Convert RBSIL-image_list.adoc, RBSID-image_delete.adoc, RBSIR-image_retrieve.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

*Direction:* Agent: sonnet

## Context
Read the paddock at .claude/jjm/jjp_uAlj.md for transformation rules.
Read exemplar: lenses/RBSDC-depot_create.adoc (fully transformed subdoc).
Read exemplar: lenses/RBSRC-retriever_create.adoc (small transformed subdoc).

## Steps
1. Read all 3 image subdocs: lenses/RBSIL-image_list.adoc, lenses/RBSID-image_delete.adoc, lenses/RBSIR-image_retrieve.adoc
2. Grep RBS0-SpecTop.adoc for rbtgo_image_list, rbtgo_image_delete, rbtgo_image_retrieve to find operation linked terms
3. Apply transformation to all 3 files per paddock policy
4. Write all 3 transformed files

## Verification
Grep all 3 transformed files for '⟦axs_' — must return zero matches.

**[260227-1057] rough**

Convert RBSIL-image_list.adoc, RBSID-image_delete.adoc, RBSIR-image_retrieve.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

### transform-sa-subdocs (₢AjAAV) [complete]

**[260301-1002] complete**

Convert RBSSL-sa_list.adoc, RBSSD-sa_delete.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

**[260301-0941] bridled**

Convert RBSSL-sa_list.adoc, RBSSD-sa_delete.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

*Direction:* Agent: sonnet

## Context
Read the paddock at .claude/jjm/jjp_uAlj.md for transformation rules.
Read exemplar: lenses/RBSDC-depot_create.adoc (fully transformed subdoc).
Read exemplar: lenses/RBSRC-retriever_create.adoc (small transformed subdoc).

## Steps
1. Read both SA subdocs: lenses/RBSSL-sa_list.adoc, lenses/RBSSD-sa_delete.adoc
2. Grep RBS0-SpecTop.adoc for rbtgo_sa_list, rbtgo_sa_delete to find operation linked terms
3. Apply transformation to both files per paddock policy
4. Write both transformed files

## Verification
Grep both transformed files for '⟦axs_' — must return zero matches.

**[260227-1058] rough**

Convert RBSSL-sa_list.adoc, RBSSD-sa_delete.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

### transform-probe-subdocs (₢AjAAZ) [complete]

**[260301-1013] complete**

Convert RBSAO-access_oauth_probe.adoc, RBSAJ-access_jwt_probe.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

**[260301-0942] bridled**

Convert RBSAO-access_oauth_probe.adoc, RBSAJ-access_jwt_probe.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

*Direction:* Agent: sonnet

## Context
Read the paddock at .claude/jjm/jjp_uAlj.md for transformation rules.
Read exemplar: lenses/RBSDC-depot_create.adoc (fully transformed subdoc).
Read exemplar: lenses/RBSRC-retriever_create.adoc (small transformed subdoc).

## Steps
1. Read both probe subdocs: lenses/RBSAO-access_oauth_probe.adoc, lenses/RBSAJ-access_jwt_probe.adoc
2. Grep RBS0-SpecTop.adoc for rbtgo_access_oauth_probe, rbtgo_access_jwt_probe to find operation linked terms
3. Apply transformation to both files per paddock policy
4. Write both transformed files

## Verification
Grep both transformed files for '⟦axs_' — must return zero matches.

**[260227-1058] rough**

Convert RBSAO-access_oauth_probe.adoc, RBSAJ-access_jwt_probe.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

### transform-vos0-operations (₢AjAAd) [complete]

**[260301-1034] complete**

Transform VOS0-VoxObscuraSpec.adoc: 4 axo_command (with axe_rust_impl — drop language), 5 axo_routine, 3 ghost axo_operation (allocate, invitatory, release — determine correct voicing). Also review voss_* section motifs that voice deprecated axs_* terms.

**[260227-1058] rough**

Transform VOS0-VoxObscuraSpec.adoc: 4 axo_command (with axe_rust_impl — drop language), 5 axo_routine, 3 ghost axo_operation (allocate, invitatory, release — determine correct voicing). Also review voss_* section motifs that voice deprecated axs_* terms.

### transform-jjs0-routines (₢AjAAQ) [complete]

**[260301-1037] complete**

Transform JJS0-GallopsData.adoc: 4 axo_routine (load, save, persist, wrap) to axvo_procedure or axvo_method. Determine entity affiliation for each.

**[260227-1057] rough**

Transform JJS0-GallopsData.adoc: 4 axo_routine (load, save, persist, wrap) to axvo_procedure or axvo_method. Determine entity affiliation for each.

### review-jjs0-interface-layer (₢AjAAU) [complete]

**[260301-1039] complete**

Review JJS0 interface terms: ~18 axi_cc_claudemd_verb, ~15 axi_cc_slash_command, ~25 axi_cli_subcommand, plus axa_cli_option/flag terms. Determine if these are affected by the new model or remain as-is.

**[260227-1058] rough**

Review JJS0 interface terms: ~18 axi_cc_claudemd_verb, ~15 axi_cc_slash_command, ~25 axi_cli_subcommand, plus axa_cli_option/flag terms. Determine if these are affected by the new model or remain as-is.

### verify-bus0-unaffected (₢AjAAY) [complete]

**[260301-1039] complete**

Verify BUS0-BashUtilitiesSpec.adoc uses only axo_entity and axrg_* regime voicings with no operation voicings needing transformation. Quick verification.

**[260227-1058] rough**

Verify BUS0-BashUtilitiesSpec.adoc uses only axo_entity and axrg_* regime voicings with no operation voicings needing transformation. Quick verification.

### rewrite-completeness-checklists (₢AjAAc) [complete]

**[260301-1042] complete**

Replace Procedure/Command/Guide/Lifecycle Documentation Completeness sections in AXLA with new checklists based on axvo_procedure, axvo_method, axvo_group, and axho_* markers.

**[260227-1058] rough**

Replace Procedure/Command/Guide/Lifecycle Documentation Completeness sections in AXLA with new checklists based on axvo_procedure, axvo_method, axvo_group, and axho_* markers.

### transform-rbs0-non-rbtgo-operations (₢AjAAl) [complete]

**[260301-1049] complete**

Transform remaining legacy annotations in RBS0-SpecTop.adoc that block deprecated term deletion:

1. Non-rbtgo operation voicings (~16 instances):
   - 3 axo_command (lines 1090, 1121, 1143)
   - 11 axo_sequence (lines 1101-1159, 3270-3296)
   - 1 axo_routine (line 1169)
   - 1 axo_command axe_bash_scripted (line 3262)
   Apply same transformation patterns as rbtgo operations (axvo_method/axvo_procedure with appropriate dimensions).

2. Control term environment annotations (~19 instances):
   - ~14 axc_* with axe_bash_interactive (lines 2896-2972)
   - ~5 axc_* with axe_human_guide (lines 2986-3014)
   Determine correct handling: these are step-level vocabulary with environment context. May need new pattern or may simply drop the axe_* dimension.

3. Mapping entries: axe_bash_interactive and axe_human_guide attribute references (lines 456-457)

Read RBS0 context around each cluster to determine correct voicing. Consult paddock for transformation patterns.

**[260301-1046] rough**

Transform remaining legacy annotations in RBS0-SpecTop.adoc that block deprecated term deletion:

1. Non-rbtgo operation voicings (~16 instances):
   - 3 axo_command (lines 1090, 1121, 1143)
   - 11 axo_sequence (lines 1101-1159, 3270-3296)
   - 1 axo_routine (line 1169)
   - 1 axo_command axe_bash_scripted (line 3262)
   Apply same transformation patterns as rbtgo operations (axvo_method/axvo_procedure with appropriate dimensions).

2. Control term environment annotations (~19 instances):
   - ~14 axc_* with axe_bash_interactive (lines 2896-2972)
   - ~5 axc_* with axe_human_guide (lines 2986-3014)
   Determine correct handling: these are step-level vocabulary with environment context. May need new pattern or may simply drop the axe_* dimension.

3. Mapping entries: axe_bash_interactive and axe_human_guide attribute references (lines 456-457)

Read RBS0 context around each cluster to determine correct voicing. Consult paddock for transformation patterns.

### delete-legacy-terms (₢AjAAe) [complete]

**[260301-1057] complete**

Remove all deprecated terms listed in AXLA deprecation appendix. Remove the deprecation appendix itself. Remove legacy Procedure Hierarchy section and old completeness checklists. Verify no remaining references to deleted terms across all specs.

**[260227-1058] rough**

Remove all deprecated terms listed in AXLA deprecation appendix. Remove the deprecation appendix itself. Remove legacy Procedure Hierarchy section and old completeness checklists. Verify no remaining references to deleted terms across all specs.

### note-future-heat-non-rbtgo-ops (₢AjAAf) [complete]

**[260301-1217] complete**

The mkr_*, opss_*, opbs_*, opbr_*, scr_* operations in RBS0 (bottle, network, sentry, security) have non-rbtgo prefixes and were out of scope. Slate an itch or create a note for a future heat to bring them up to standards established in this heat.

**[260301-1200] rough**

The mkr_*, opss_*, opbs_*, opbr_*, scr_* operations in RBS0 (bottle, network, sentry, security) have non-rbtgo prefixes and were out of scope. Slate an itch or create a note for a future heat to bring them up to standards established in this heat.

**[260227-1058] rough**

The mkr_*, opss_*, opbs_*, opbr_*, scr_* operations in RBS0 (bottle, network, sentry, security) have non-rbtgo prefixes and were out of scope. Slate an itch or create a note for a future heat to bring them up to standards established in this heat.

### transform-rbsdn-depot-initialize (₢AjAAL) [abandoned]

**[260227-1058] abandoned**

Exemplar transformation: convert RBSDN-depot_initialize.adoc (~140 lines) from legacy axs_* section markers to new axho_* hierarchy markers. Attended exemplar (axd_attended). Follow pattern from transform-rbsdc-depot-create. Has axs_preconditions, human-interactive OAuth flow, "Go to step 7" branching. Mint linked terms for steps, preconditions, outputs.

**[260227-1057] rough**

Exemplar transformation: convert RBSDN-depot_initialize.adoc (~140 lines) from legacy axs_* section markers to new axho_* hierarchy markers. Attended exemplar (axd_attended). Follow pattern from transform-rbsdc-depot-create. Has axs_preconditions, human-interactive OAuth flow, "Go to step 7" branching. Mint linked terms for steps, preconditions, outputs.

### transform-rbsgd-gdc-establish (₢AjAAM) [abandoned]

**[260227-1058] abandoned**

Convert RBSGD-gdc_establish.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

**[260227-1057] rough**

Convert RBSGD-gdc_establish.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

### transform-payor-subdocs (₢AjAAP) [abandoned]

**[260227-1058] abandoned**

Convert RBSPE-payor_establish.adoc, RBSPI-payor_install.adoc, RBSPR-payor_refresh.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern. Payor establish and refresh are attended.

**[260227-1057] rough**

Convert RBSPE-payor_establish.adoc, RBSPI-payor_install.adoc, RBSPR-payor_refresh.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern. Payor establish and refresh are attended.

### transform-rbsgr-governor-reset (₢AjAAT) [abandoned]

**[260227-1058] abandoned**

Convert RBSGR-governor_reset.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

**[260227-1058] rough**

Convert RBSGR-governor_reset.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

### transform-rbsrc-retriever-create (₢AjAAX) [abandoned]

**[260227-1058] abandoned**

Convert RBSRC-retriever_create.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

**[260227-1058] rough**

Convert RBSRC-retriever_create.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

### transform-rbsdi-director-create (₢AjAAb) [abandoned]

**[260227-1058] abandoned**

Convert RBSDI-director_create.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

**[260227-1058] rough**

Convert RBSDI-director_create.adoc from legacy axs_* to axho_* markers. Follow established exemplar pattern.

### explore-definition-site-bare-markers (₢AjAAj) [complete]

**[260301-1517] complete**

Explore converting definition-site voicing annotations from Strachey bracket syntax (// ⟦axvo_method axd_transient⟧) to bare marker syntax (//axvo_method axd_transient) for consistency with detail-site hierarchy markers. Assess impact across all transformed documents, AXLA spec changes needed, and whether this unification is desirable or whether the syntactic distinction between definition-site and detail-site serves a useful purpose.

**[260301-1028] rough**

Explore converting definition-site voicing annotations from Strachey bracket syntax (// ⟦axvo_method axd_transient⟧) to bare marker syntax (//axvo_method axd_transient) for consistency with detail-site hierarchy markers. Assess impact across all transformed documents, AXLA spec changes needed, and whether this unification is desirable or whether the syntactic distinction between definition-site and detail-site serves a useful purpose.

### convert-vos0-subdocs-bare-markers (₢AjAAk) [complete]

**[260301-1534] complete**

Convert VOS0 operation subdocuments (VOSRL-lock.adoc, VOSRC-commit.adoc, VOSRG-guard.adoc, VOSRP-probe.adoc, VOSRI-init.adoc, and the inline vosor_release detail) from voss_* section headers to bare //axho_* detail-site markers, matching the pattern established in RBS0 subdocuments. Remove voss_* term definitions from VOS0 mapping section once all usage is eliminated.

**[260301-1030] rough**

Convert VOS0 operation subdocuments (VOSRL-lock.adoc, VOSRC-commit.adoc, VOSRG-guard.adoc, VOSRP-probe.adoc, VOSRI-init.adoc, and the inline vosor_release detail) from voss_* section headers to bare //axho_* detail-site markers, matching the pattern established in RBS0 subdocuments. Remove voss_* term definitions from VOS0 mapping section once all usage is eliminated.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 A repair-regime-subdoc-brackets
  2 B define-operation-groups-in-s0
  3 C transform-s0-definition-sites
  4 D design-rbtoe-internal-routines
  5 E transform-rbsdc-depot-create
  6 F transform-rbsdn-depot-initialize
  7 g clarify-detail-site-lookahead-policy
  8 h update-axla-lookahead-spec
  9 i repair-exemplar-wrong-elevation-terms
  10 G transform-payor-subdocs
  11 H transform-rbsgr-governor-reset
  12 I transform-rbsrc-retriever-create
  13 J transform-rbsdi-director-create
  14 K transform-rbsgd-gdc-establish
  15 O transform-rbsri-rubric-inscribe
  16 S transform-rbstb-trigger-build
  17 W transform-rbsqb-quota-build
  18 a transform-depot-minor-subdocs
  19 N transform-ark-subdocs
  20 R transform-image-subdocs
  21 V transform-sa-subdocs
  22 Z transform-probe-subdocs
  23 d transform-vos0-operations
  24 Q transform-jjs0-routines
  25 U review-jjs0-interface-layer
  26 Y verify-bus0-unaffected
  27 c rewrite-completeness-checklists
  28 l transform-rbs0-non-rbtgo-operations
  29 e delete-legacy-terms
  30 f note-future-heat-non-rbtgo-ops
  31 j explore-definition-site-bare-markers
  32 k convert-vos0-subdocs-bare-markers

ABCDEFghiGHIJKOSWaNRVZdQUYclefjk
·xxxx···x··················x·xx· RBS0-SpecTop.adoc
·······x··················x·x·x· AXLA-Lexicon.adoc
··x············x·············x·x rbf_Foundry.sh
······················x·······xx VOS0-VoxObscuraSpec.adoc
······························xx VOSRI-init.adoc, VOSRL-lock.adoc, VOSRP-probe.adoc
·······················x······x· JJS0-GallopsData.adoc
······················x·····x··· rbgg_Governor.sh, rbgp_Payor.sh
····x···x······················· RBSDC-depot_create.adoc
·······························x VOSRC-commit.adoc, VOSRG-guard.adoc, rbob_bottle.sh, rbw-DA.DirectorAbjuresArk.sh, rbw-DB.DirectorBeseechesArk.sh, rbw-DC.DirectorConjuresArk.sh, rbw-DD.DirectorDeletesImage.sh, rbw-DI.DirectorInscribesRubric.sh, rbw-DP.DirectorRefreshesPins.sh, rbw-DS.DirectorSummonsArk.sh, rbw-Dl.DirectorListsImages.sh, rbw-Dr.DirectorRetrievesImage.sh, rbw-RI.RubricInscribe.sh, rbw-aA.AbjureArk.sh, rbw-aC.ConjureArk.sh, rbw-ab.BeseechArk.sh, rbw-as.SummonArk.sh, rbw-iB.BuildImageRemotely.sh, rbw-iD.DeleteImage.sh, rbw-il.ImageList.sh, rbw-ir.RetrieveImage.sh, rbw-rrg.RefreshGcbPins.sh, rbw_workbench.sh, rbz_zipper.sh
······························x· BUS0-BashUtilitiesSpec.adoc, BUSD-DispatchRuntime.adoc, JJSCCH-chalk.adoc, JJSCDR-draft.adoc, JJSCFU-furlough.adoc, JJSCGC-get-coronets.adoc, JJSCGL-garland.adoc, JJSCGS-get-spec.adoc, JJSCLD-landing.adoc, JJSCMU-muster.adoc, JJSCNC-notch.adoc, JJSCNO-nominate.adoc, JJSCPD-parade.adoc, JJSCRL-rail.adoc, JJSCRN-rein.adoc, JJSCRS-restring.adoc, JJSCRT-retire.adoc, JJSCSC-scout.adoc, JJSCSD-saddle.adoc, JJSCSL-slate.adoc, JJSCTL-tally.adoc, JJSCVL-validate.adoc, JJSRLD-load.adoc, JJSRPS-persist.adoc, JJSRSV-save.adoc, JJSRWP-wrap.adoc, VLS-VoxLiturgicalSpec.adoc
·····························x·· RBSAX-access_setup.adoc, RBSBC-bottle_create.adoc, RBSBK-bottle_cleanup.adoc, RBSBL-bottle_launch.adoc, RBSBR-bottle_run.adoc, RBSBS-bottle_start.adoc, RBSCE-command_exec.adoc, RBSDS-dns_step.adoc, RBSIP-iptables_init.adoc, RBSNC-network_create.adoc, RBSNX-network_connect.adoc, RBSPT-port_setup.adoc, RBSSC-security_config.adoc, RBSSR-sentry_run.adoc, RBSSS-sentry_start.adoc
······················x········· rbgi_IAM.sh
·····················x·········· RBSAJ-access_jwt_probe.adoc, RBSAO-access_oauth_probe.adoc
····················x··········· RBSSD-sa_delete.adoc, RBSSL-sa_list.adoc
···················x············ RBSID-image_delete.adoc, RBSIL-image_list.adoc, RBSIR-image_retrieve.adoc
··················x············· RBSAA-ark_abjure.adoc, RBSAB-ark_beseech.adoc, RBSAC-ark_conjure.adoc, RBSAS-ark_summon.adoc
·················x·············· RBSDD-depot_destroy.adoc, RBSDL-depot_list.adoc
················x··············· RBSQB-quota_build.adoc
···············x················ RBSTB-trigger_build.adoc
··············x················· RBSRI-rubric_inscribe.adoc
·············x·················· RBSGD-gdc_establish.adoc
············x··················· RBSDI-director_create.adoc
···········x···················· RBSRC-retriever_create.adoc
··········x····················· RBSGR-governor_reset.adoc
·········x······················ RBSPE-payor_establish.adoc, RBSPI-payor_install.adoc, RBSPR-payor_refresh.adoc
········x······················· RBSDN-depot_initialize.adoc
··x····························· cloudbuild.json
·x······························ rbf_cli.sh, rbrr.env
x······························· RBRN-RegimeNameplate.adoc, RBSRA-CredentialFormat.adoc, RBSRO-RegimeOauth.adoc, RBSRP-RegimePayor.adoc, RBSRR-RegimeRepo.adoc, RBSRS-RegimeStation.adoc, RBSRV-RegimeVessel.adoc

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 182 commits)

  1 Z transform-probe-subdocs
  2 d transform-vos0-operations
  3 Q transform-jjs0-routines
  4 U review-jjs0-interface-layer
  5 Y verify-bus0-unaffected
  6 c rewrite-completeness-checklists
  7 e delete-legacy-terms
  8 l transform-rbs0-non-rbtgo-operations
  9 f note-future-heat-non-rbtgo-ops
  10 j explore-definition-site-bare-markers
  11 k convert-vos0-subdocs-bare-markers

123456789abcdefghijklmnopqrstuvwxyz
xx·································  Z  2c
··x··xx····························  d  3c
·······xxx·························  Q  3c
··········xx·······················  U  2c
············xx·····················  Y  2c
··············xxx··················  c  3c
·················x···xx············  e  3c
···················xx··············  l  2c
·······················x·xxxx······  f  5c
·····························xxx···  j  3c
································xxx  k  3c
```

## Steeplechase

### 2026-03-01 15:34 - ₢AjAAk - W

Convert VOS0 subdocuments and inline operations from voss_* section headers to bare axho_* detail-site markers; remove voss_* definitions

### 2026-03-01 15:34 - ₢AjAAk - n

Rename Director tabtargets to rbw-D* prefix, consolidate zipper enrollments; transform 5 VOS0 subdocs + inline ops from voss_* section headers to individual axho_* bare markers; remove voss_* definitions from VOS0

### 2026-03-01 15:22 - ₢AjAAk - A

Transform 5 VOS0 subdocs + inline ops from voss_* section headers to individual axho_* bare markers; remove voss_* definitions from VOS0

### 2026-03-01 15:17 - ₢AjAAj - W

Converted 636 Strachey bracket annotations to bare marker syntax across 35 files (RBS0, JJS0, VOS0, BUS0, BUSD, VLS, AXLA, 24 JJK subdocs, 3 VOS subdocs); GADS skipped

### 2026-03-01 15:17 - ₢AjAAj - n

Strip Strachey brackets from annotation comments across all concept model documents

### 2026-03-01 15:01 - ₢AjAAj - A

Survey definition-site bracket annotations across transformed docs, assess whether syntactic distinction from detail-site markers serves purpose, deliver unify-or-preserve recommendation

### 2026-03-01 12:17 - ₢AjAAf - W

Transformed 15 non-rbtgo subdocuments (mkr_*, opss_*, opbs_*, opbr_*, scr_*) to detail-site hierarchy markers; created RBSSR-sentry_run.adoc subdocument; updated RBS0 include

### 2026-03-01 12:17 - ₢AjAAf - n

Diagnose: experiment with triggers.run body formats, then pivot to builds.create+developerConnectConfig if needed

### 2026-03-01 12:16 - ₢AjAAf - n

Transform 15 non-rbtgo subdocuments to detail-site hierarchy markers; create RBSSR-sentry_run.adoc; update RBS0 include

### 2026-03-01 12:00 - ₢AjAAf - A

Scan RBS0 for non-rbtgo ops, write itch entry in jji_itch.md

### 2026-03-01 12:00 - Heat - T

note-future-heat-non-rbtgo-ops

### 2026-03-01 10:59 - ₢AjAAf - A

Add itch entry for non-rbtgo operation voicing transformation

### 2026-03-01 10:57 - ₢AjAAe - W

Deleted deprecated terms, deprecation appendix, Procedure Hierarchy section, and legacy mapping entries from AXLA. Cross-spec verification confirmed remaining axs_* refs are only in out-of-scope non-rbtgo subdocs.

### 2026-03-01 10:57 - ₢AjAAe - n

Remove legacy axe_*/axo_*/axs_* terms and migration plan from AXLA after vocabulary transformation; fix IAM binding calls in Governor and Payor to use service account email instead of numeric UID for bucket and repo roles

### 2026-03-01 10:49 - ₢AjAAl - W

Transformed RBS0 non-rbtgo operations: 16 legacy voicings to axvo_procedure, dropped axe_* dimensions from 18 control terms, removed axe_* mapping entries

### 2026-03-01 10:49 - ₢AjAAl - n

Remove axe_bash_interactive/axe_human_guide environment terms; migrate annotations to axvo_procedure/axd_transient/axd_internal vocabulary

### 2026-03-01 10:46 - Heat - S

transform-rbs0-non-rbtgo-operations

### 2026-03-01 10:43 - ₢AjAAe - A

Delete Procedure Hierarchy defs, Migration Plan section, deprecated mapping entries; cross-spec verification

### 2026-03-01 10:42 - ₢AjAAc - W

Replaced 6 legacy completeness checklists (Procedure/Command/Guide/Lifecycle/Long-running/Periodic) with new Operation Detail-Site Completeness using axvo_*/axho_* vocabulary

### 2026-03-01 10:42 - ₢AjAAc - n

Delete legacy checklists (3051-3167), add detail-site completeness checklist alongside existing new voicing checklists

### 2026-03-01 10:40 - ₢AjAAc - A

Delete legacy checklists (3051-3167), add detail-site completeness checklist alongside existing new voicing checklists

### 2026-03-01 10:39 - ₢AjAAY - W

Verified BUS0 has zero operation voicings (axo_command/routine/operation/guide) or axs_* section markers — unaffected by transformation

### 2026-03-01 10:39 - ₢AjAAY - A

Verified: BUS0 has zero axo_command/routine/operation/guide or axs_* — unaffected

### 2026-03-01 10:39 - ₢AjAAU - W

Confirmed ~75 axi_*/axa_* interface voicings in JJS0 are unaffected by operation model change — no transformation needed

### 2026-03-01 10:37 - ₢AjAAU - A

Review-only: ~75 axi_*/axa_* interface voicings confirmed unaffected by operation model change

### 2026-03-01 10:37 - ₢AjAAQ - W

Transformed JJS0: 4 routines (load, save, persist, wrap) from axo_routine to axvo_procedure axd_internal

### 2026-03-01 10:37 - ₢AjAAQ - n

Mechanical: 4 axo_routine → axvo_procedure axd_internal, same as VOS0 pattern

### 2026-03-01 10:35 - ₢AjAAQ - A

Mechanical: 4 axo_routine → axvo_procedure axd_internal, same as VOS0 pattern

### 2026-03-01 10:34 - ₢AjAAd - W

Transformed VOS0: 4 commands→procedure, 5 routines→procedure internal, 3 ghost action codes→procedure transient internal, 4 voss_* motifs→axho_* voicings

### 2026-03-01 10:34 - ₢AjAAd - n

Numeric ID for IAM bindings: capture uniqueId from SA create, remove polls/sleeps, pass numeric ID to all member bindings, delete dead code

### 2026-03-01 10:30 - Heat - S

convert-vos0-subdocs-bare-markers

### 2026-03-01 10:28 - Heat - S

explore-definition-site-bare-markers

### 2026-03-01 10:19 - ₢AjAAd - A

Sequential opus: 4 command→method, 5 routine→procedure, 3 ghost axo_operation need voicing decision, voss_* section motifs need deprecation decision

### 2026-03-01 10:13 - ₢AjAAZ - W

Transformed 2 probe subdocs (RBSAO, RBSAJ) from legacy axs_* to axho_* bare markers per settled policy

### 2026-03-01 10:13 - ₢AjAAZ - n

Transformed 2 access probe subdocs (RBSAJ, RBSAO) from legacy axs_* Strachey brackets to axho_* bare markers with operation headers

### 2026-03-01 10:09 - ₢AjAAZ - L

sonnet landed

### 2026-03-01 10:06 - ₢AjAAZ - F

Executing bridled pace via sonnet agent

### 2026-03-01 10:02 - ₢AjAAV - W

Transformed 2 SA subdocs (RBSSL, RBSSD) from legacy axs_* to axho_* bare markers per settled policy

### 2026-03-01 10:02 - ₢AjAAV - n

Transformed 2 service account subdocs (RBSSD, RBSSL) from legacy axs_* to axho_* bare markers with typed parameters and operation headers

### 2026-03-01 09:59 - ₢AjAAV - L

sonnet landed

### 2026-03-01 09:56 - ₢AjAAV - F

Executing bridled pace via sonnet agent

### 2026-03-01 09:54 - ₢AjAAR - W

Transformed 3 image subdocs (RBSIL, RBSID, RBSIR) from legacy axs_* to axho_* bare markers with typed parameters and operation headers

### 2026-03-01 09:54 - ₢AjAAR - n

Transformed 3 image subdocs from legacy axs_* to axho_* bare markers per settled policy

### 2026-03-01 09:54 - ₢AjAAR - L

sonnet landed

### 2026-03-01 09:52 - ₢AjAAR - F

Executing bridled pace via sonnet agent

### 2026-03-01 09:50 - ₢AjAAN - W

Transformed 4 ark subdocs from legacy axs_* to axho_* bare markers per settled policy

### 2026-03-01 09:50 - ₢AjAAN - n

Transformed RBSAA, RBSAB, RBSAC, RBSAS from legacy axs_* to axho_* bare markers with typed parameters and operation headers

### 2026-03-01 09:50 - ₢AjAAN - L

sonnet landed

### 2026-03-01 09:48 - ₢AjAAN - F

Executing bridled pace via sonnet agent

### 2026-03-01 09:48 - ₢AjAAa - W

Transformed RBSDD and RBSDL from legacy axs_* to axho_* bare markers per settled policy

### 2026-03-01 09:48 - ₢AjAAa - n

Transformed RBSDD and RBSDL from legacy axs_* to axho_* bare markers: 2 operation headers, 1 parameter, 8 steps, 1 output_of_type, 2 completions

### 2026-03-01 09:47 - ₢AjAAa - L

sonnet landed

### 2026-03-01 09:45 - ₢AjAAa - F

Executing bridled pace via sonnet agent

### 2026-03-01 09:45 - ₢AjAAW - W

Transformed RBSQB from legacy axs_* to axho_* bare markers: 1 operation header, 3 preconditions, 3 steps, 1 output_of_type, 1 completion

### 2026-03-01 09:45 - ₢AjAAW - n

Transformed RBSQB from legacy axs_* to axho_* bare markers: 1 operation header, 3 preconditions, 3 steps, 1 output_of_type, 1 completion

### 2026-03-01 09:45 - ₢AjAAW - L

sonnet landed

### 2026-03-01 09:44 - ₢AjAAW - F

Executing bridled pace via sonnet agent

### 2026-03-01 09:44 - ₢AjAAS - W

Transformed RBSTB from legacy axs_* to axho_* bare markers: 1 operation header, 1 parameter_from_type, 5 steps, 1 output_of_type, 1 completion

### 2026-03-01 09:44 - ₢AjAAS - n

Fix trigger build repository→uri field rename and migrate RBSTB annotations to axho taxonomy

### 2026-03-01 09:44 - ₢AjAAS - L

sonnet landed

### 2026-03-01 09:42 - ₢AjAAS - F

Executing bridled pace via sonnet agent

### 2026-03-01 09:42 - ₢AjAAZ - B

arm | transform-probe-subdocs

### 2026-03-01 09:42 - Heat - T

transform-probe-subdocs

### 2026-03-01 09:41 - ₢AjAAV - B

arm | transform-sa-subdocs

### 2026-03-01 09:41 - Heat - T

transform-sa-subdocs

### 2026-03-01 09:41 - ₢AjAAR - B

arm | transform-image-subdocs

### 2026-03-01 09:41 - Heat - T

transform-image-subdocs

### 2026-03-01 09:41 - ₢AjAAN - B

arm | transform-ark-subdocs

### 2026-03-01 09:41 - Heat - T

transform-ark-subdocs

### 2026-03-01 09:40 - ₢AjAAa - B

arm | transform-depot-minor-subdocs

### 2026-03-01 09:40 - Heat - T

transform-depot-minor-subdocs

### 2026-03-01 09:39 - ₢AjAAW - B

arm | transform-rbsqb-quota-build

### 2026-03-01 09:39 - Heat - T

transform-rbsqb-quota-build

### 2026-03-01 09:39 - ₢AjAAS - B

arm | transform-rbstb-trigger-build

### 2026-03-01 09:39 - Heat - T

transform-rbstb-trigger-build

### 2026-03-01 09:38 - ₢AjAAO - W

Transformed RBSRI from legacy axs_* Strachey brackets to axho_* bare markers: 1 operation header, 3 preconditions, 5 steps, 1 output_of_type rubric_repo, 1 completion

### 2026-03-01 09:38 - ₢AjAAO - n

Transformed RBSRI from legacy axs_* Strachey brackets to axho_* bare markers: 1 operation header, 3 preconditions, 5 steps, 1 output_of_type rubric_repo, 1 completion

### 2026-03-01 09:32 - ₢AjAAK - W

Transformed RBSGD from legacy axs_* Strachey brackets to axho_* bare markers: 1 operation header, 2 preconditions, 1 step, 1 output_of_type source_connection, 1 completion

### 2026-03-01 09:32 - ₢AjAAK - n

Transformed RBSGD from legacy axs_* Strachey brackets to axho_* bare markers: 1 operation header, 2 preconditions, 1 step, 1 output_of_type source_connection, 1 completion

### 2026-03-01 09:32 - ₢AjAAK - A

axhob header + 2 preconditions + 1 step + output_of_type source_connection + completion

### 2026-03-01 09:31 - ₢AjAAJ - W

Transformed RBSDI from legacy axs_* Strachey brackets to axho_* bare markers: 1 operation header, 3 preconditions, 1 parameter_from_type director, 9 steps, 1 output_of_type rbra_file, 1 completion

### 2026-03-01 09:31 - ₢AjAAJ - n

Transformed RBSDI director_create from legacy axs_* Strachey brackets to axho_* bare markers: 1 operation header, 3 preconditions, 1 parameter_from_type, 9 steps, 1 output_of_type rbra_file, 1 completion

### 2026-03-01 09:29 - ₢AjAAJ - A

axhob header + 2 preconditions + 1 parameter + 8 steps + output_of_type rbra_file + completion

### 2026-03-01 09:28 - ₢AjAAI - W

Transformed RBSRC from legacy axs_* Strachey brackets to axho_* bare markers: 1 operation header, 2 preconditions, 1 step, 1 output_of_type, 1 completion

### 2026-03-01 09:28 - ₢AjAAI - n

Transformed RBSRC retriever_create from legacy axs_* Strachey brackets to axho_* bare markers: 1 operation header, 2 preconditions, 1 step, 1 output_of_type rbra_file, 1 completion

### 2026-03-01 09:27 - ₢AjAAI - A

axhob header + 2 preconditions + 1 step + output_of_type rbra_file + completion

### 2026-03-01 09:25 - ₢AjAAH - W

Transformed RBSGR from legacy axs_* Strachey brackets to axho_* bare markers: 1 operation header, 1 parameter_from_type, 8 steps, 1 output_of_type, 1 completion

### 2026-03-01 09:25 - ₢AjAAH - n

Transform RBSGR governor_reset from legacy axs_* Strachey brackets to axho_* bare markers: add operation header, parameter_from_type for depot_project, 8 step markers, output_of_type governor, and completion marker

### 2026-03-01 09:23 - ₢AjAAH - A

axhob header + parameter_from_type for depot_project + 8 axhos_step + output_of_type governor + completion

### 2026-03-01 09:16 - ₢AjAAG - W

Transformed 3 payor subdocs (RBSPE/RBSPI/RBSPR) from legacy axs_* to axho_* markers per settled policy: 9 steps, 4 outputs, 2 preconditions, 1 guarantee, 3 completions

### 2026-03-01 09:16 - ₢AjAAG - n

Transform 3 payor subdocs (RBSPE/RBSPI/RBSPR) from legacy axs_* Strachey brackets to axho_* bare markers: convert inputs to preconditions, outputs to of_type with domain terms, add operation headers, and restructure completion/guarantee sections per settled policy

### 2026-03-01 09:09 - ₢AjAAG - A

Sonnet agent: transform 3 payor subdocs from legacy axs_* to axho_* markers per settled policy

### 2026-03-01 09:08 - ₢AjAAi - W

Removed 34 wrong-elevation terms from RBSDC/RBSDN/S0: demoted steps/preconditions to bare markers, converted waymark with local anchor, converted parameters to from_type/from_arg, converted outputs to of_type with existing domain terms

### 2026-03-01 09:08 - ₢AjAAi - n

Remove 34 wrong-elevation terms, apply settled policy to RBSDC/RBSDN/S0

### 2026-03-01 09:01 - ₢AjAAi - A

Sonnet agent: remove 34 wrong-elevation terms, apply settled policy to RBSDC/RBSDN/S0

### 2026-03-01 09:00 - ₢AjAAh - W

Defined axhop_parameter_from_type, axhop_parameter_from_arg, axhos_waymark, axhoo_output_of_type in AXLA; codified no-lookahead policy for step/precondition/guarantee/completion; demoted base parameter/output forms to abstract parents

### 2026-03-01 09:00 - ₢AjAAh - n

Refine AXLA operation detail markers: split parameter/output into typed variants, convert precondition/guarantee/step to bare 0-arity markers, add waymark for branch-targetable steps

### 2026-03-01 08:46 - ₢AjAAh - A

Spawning sonnet agent for AXLA spec edits

### 2026-03-01 08:40 - ₢AjAAg - W

Settled lookahead policy for all 9 detail-site markers, introduced axhos_waymark for branch targets, updated paddock, slated AXLA update and exemplar repair paces

### 2026-03-01 08:39 - Heat - S

repair-exemplar-wrong-elevation-terms

### 2026-03-01 08:38 - Heat - S

update-axla-lookahead-spec

### 2026-03-01 08:15 - ₢AjAAg - A

Interactive policy resolution: outputs, guarantees, completion, branch targets, 34 wrong-elevation terms, paddock+AXLA updates

### 2026-03-01 08:14 - ₢AjAAF - W

Subdoc transform already landed via ₢AiAAj; wrapping to advance heat

### 2026-02-27 15:29 - Heat - S

clarify-detail-site-lookahead-policy

### 2026-02-27 14:39 - ₢AjAAF - A

Attended exemplar: axhob opener, 5 preconditions, 8 steps (branching via linked term), 3 outputs, completion + 16 terms in S0

### 2026-02-27 14:37 - ₢AjAAE - W

Exemplar subdoc transform: replaced 4 legacy axs_* section markers with 19 individual axho_* hierarchy markers (2 params, 14 steps, 2 outputs, 1 completion), minted 18 rbtgo_depot_create_* linked terms in S0

### 2026-02-27 14:37 - ₢AjAAE - n

Depot create subdoc transform: axhob opener, 2 params, 14 steps, 2 outputs, completion + 18 linked terms in S0

### 2026-02-27 14:29 - ₢AjAAE - A

Exemplar subdoc transform: axhob opener, 2 params, 14 steps, 2 outputs, completion + 18 linked terms in S0

### 2026-02-27 14:27 - ₢AjAAD - W

Classified and transformed 13 rbtoe_* routines: 5 axvo_method axd_internal (4 auth + depot_list_update), 8 axvo_procedure axd_internal (cross-cutting utilities)

### 2026-02-27 14:27 - ₢AjAAD - n

Transformed 12 rbtoe_* orchestration pattern voicings to axvo_method/axvo_procedure with axd_transient/axd_internal dimensions, added linked term for depot in rbtoe_depot_list_update

### 2026-02-27 14:24 - ₢AjAAC - W

Transformed 25 rbtgo_* definition-site voicings to axvo_method with axd_transient/axd_attended/axd_grouped dimensions, added entity+group positional refs to all operation bodies

### 2026-02-27 14:24 - ₢AjAAC - n

₢AjAAC: Isolation subshell for vessel loading, axvo_method voicing transform, and inscribe-generated cloudbuild.json for all 8 vessels

### 2026-02-27 14:17 - ₢AjAAC - A

Per-operation voicing transform: classify method/procedure, assign dimensions, fix positional refs

### 2026-02-27 14:17 - ₢AjAAB - W

Added 8 rbtgog_* operation group definitions with axvo_group axd_tabtarget voicings in S0 Operation Groups section

### 2026-02-27 14:17 - ₢AjAAB - n

Inventory entities, mint rbtgog_* groups with axvo_group voicing, place before member ops in S0

### 2026-02-27 14:09 - ₢AjAAB - A

Inventory entities, mint rbtgog_* groups with axvo_group voicing, place before member ops in S0

### 2026-02-27 14:07 - ₢AjAAA - W

Converted 107 Strachey bracket annotations to bare prefix form across 7 regime subdocs

### 2026-02-27 14:07 - ₢AjAAA - n

Mechanical find-replace: Strachey bracket forms to bare prefix in 7 regime subdocs

### 2026-02-27 14:05 - ₢AjAAA - A

Mechanical find-replace: Strachey bracket forms to bare prefix in 7 regime subdocs

### 2026-02-27 11:05 - Heat - d

paddock curried

### 2026-02-27 10:59 - Heat - r

moved AjAAf after AjAAe

### 2026-02-27 10:59 - Heat - r

moved AjAAe after AjAAc

### 2026-02-27 10:59 - Heat - r

moved AjAAc after AjAAY

### 2026-02-27 10:59 - Heat - r

moved AjAAY after AjAAU

### 2026-02-27 10:59 - Heat - r

moved AjAAU after AjAAQ

### 2026-02-27 10:59 - Heat - r

moved AjAAQ after AjAAd

### 2026-02-27 10:59 - Heat - r

moved AjAAd after AjAAZ

### 2026-02-27 10:59 - Heat - r

moved AjAAZ after AjAAV

### 2026-02-27 10:59 - Heat - r

moved AjAAV after AjAAR

### 2026-02-27 10:59 - Heat - r

moved AjAAR after AjAAN

### 2026-02-27 10:59 - Heat - r

moved AjAAN after AjAAa

### 2026-02-27 10:59 - Heat - r

moved AjAAa after AjAAW

### 2026-02-27 10:59 - Heat - r

moved AjAAW after AjAAS

### 2026-02-27 10:59 - Heat - r

moved AjAAS after AjAAO

### 2026-02-27 10:59 - Heat - r

moved AjAAO after AjAAK

### 2026-02-27 10:58 - Heat - T

transform-rbsdi-director-create

### 2026-02-27 10:58 - Heat - T

transform-rbsrc-retriever-create

### 2026-02-27 10:58 - Heat - T

transform-rbsgr-governor-reset

### 2026-02-27 10:58 - Heat - T

transform-payor-subdocs

### 2026-02-27 10:58 - Heat - T

transform-rbsgd-gdc-establish

### 2026-02-27 10:58 - Heat - T

transform-rbsdn-depot-initialize

### 2026-02-27 10:58 - Heat - S

note-future-heat-bottle-ops

### 2026-02-27 10:58 - Heat - S

delete-legacy-terms

### 2026-02-27 10:58 - Heat - S

transform-vos0-operations

### 2026-02-27 10:58 - Heat - S

rewrite-completeness-checklists

### 2026-02-27 10:58 - Heat - S

transform-rbsdi-director-create

### 2026-02-27 10:58 - Heat - S

transform-depot-minor-subdocs

### 2026-02-27 10:58 - Heat - S

transform-probe-subdocs

### 2026-02-27 10:58 - Heat - S

verify-bus0-unaffected

### 2026-02-27 10:58 - Heat - S

transform-rbsrc-retriever-create

### 2026-02-27 10:58 - Heat - S

transform-rbsqb-quota-build

### 2026-02-27 10:58 - Heat - S

transform-sa-subdocs

### 2026-02-27 10:58 - Heat - S

review-jjs0-interface-layer

### 2026-02-27 10:58 - Heat - S

transform-rbsgr-governor-reset

### 2026-02-27 10:58 - Heat - S

transform-rbstb-trigger-build

### 2026-02-27 10:57 - Heat - S

transform-image-subdocs

### 2026-02-27 10:57 - Heat - S

transform-jjs0-routines

### 2026-02-27 10:57 - Heat - S

transform-payor-subdocs

### 2026-02-27 10:57 - Heat - S

transform-rbsri-rubric-inscribe

### 2026-02-27 10:57 - Heat - S

transform-ark-subdocs

### 2026-02-27 10:57 - Heat - S

transform-rbsgd-gdc-establish

### 2026-02-27 10:57 - Heat - S

transform-rbsdn-depot-initialize

### 2026-02-27 10:57 - Heat - S

transform-rbsgd-gdc-establish

### 2026-02-27 10:57 - Heat - S

transform-rbsdi-director-create

### 2026-02-27 10:56 - Heat - S

transform-rbsrc-retriever-create

### 2026-02-27 10:56 - Heat - S

transform-rbsgr-governor-reset

### 2026-02-27 10:56 - Heat - S

transform-payor-subdocs

### 2026-02-27 10:56 - Heat - S

transform-rbsdn-depot-initialize

### 2026-02-27 10:54 - Heat - S

transform-rbsdc-depot-create

### 2026-02-27 10:54 - Heat - S

design-rbtoe-internal-routines

### 2026-02-27 10:54 - Heat - S

transform-s0-definition-sites

### 2026-02-27 10:54 - Heat - S

define-operation-groups-in-s0

### 2026-02-27 10:53 - Heat - S

repair-regime-subdoc-brackets

### 2026-02-27 10:49 - Heat - d

paddock curried

### 2026-02-27 10:31 - Heat - n

fix axo_method as peer to axo_procedure (not voices), clarify axvo_*/axo_* relationships, update paddock with Strachey repair and VOS0 ghost obligations

### 2026-02-27 10:31 - Heat - d

paddock curried

### 2026-02-27 10:20 - Heat - d

paddock curried

### 2026-02-27 09:59 - Heat - d

paddock curried

### 2026-02-25 19:37 - Heat - f

racing

### 2026-02-23 18:40 - Heat - d

paddock curried

### 2026-02-23 07:42 - Heat - d

paddock curried

### 2026-02-23 07:41 - Heat - N

rbk-axla-term-voicing-scrub

