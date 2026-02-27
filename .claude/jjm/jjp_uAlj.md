# Paddock: rbk-axla-term-voicing-scrub

## Goal

Redesign AXLA's operation vocabulary to replace the fragmented procedure hierarchy
(axo_command, axo_guide, axo_routine, axo_sequence) with a clean model based on
two primary voicings, operation groups, and hierarchy markers for subdocuments.

## Design Decisions — LANDED IN AXLA

All design decisions from sessions 2026-02-27 (axla-procedure-repair) and
2026-03-02 (operations-for-the-win) have been implemented in AXLA-Lexicon.adoc.

Summary of what landed:

- **axo_procedure / axo_method** — peer motifs under the "operation" umbrella
  (neither voices the other)
- **axvo_procedure / axvo_method / axvo_group** — definition-site voicings
- **axhob_operation, axhop_parameter, axhoo_output, axhoq_precondition,
  axhog_guarantee, axhos_step, axhoc_completion** — detail-site hierarchy markers
- **axd_attended** — optional dimension (absence = unattended)
- **axd_internal** — optional dimension (absence = external)
- **axd_grouped** — dimension imposing positional requirement on definition text
  (2nd linked term for procedure, 3rd for method)
- **axd_tabtarget / axd_slash_command** — group-only exposure dimensions
- **axl_definition_site / axl_detail_site** — structural terms for the two locales
- Lifecycle dimensions (axd_transient, axd_longrunning, axd_periodic) retained unchanged

Full design rationale is in AXLA itself (term definitions + deprecation appendix).

## Remaining Work

### Phase 1: Define RBS0 operation groups at S0 level

Identify and create linked terms for operation groups in RBS0-SpecTop.adoc:
- rbtgog_depot (create, initialize, destroy, list)
- rbtgog_ark (conjure, summon, abjure, beseech)
- rbtgog_image (list, retrieve, delete)
- rbtgog_payor (establish, install, refresh)
- rbtgog_governor (reset)
- rbtgog_sa (create_retriever, create_director, list, delete)
- Others as discovered during transformation

Each group needs: linked term, axvo_group voicing, entity reference.

### Phase 2: Transform RBS0 definition-site annotations

Convert all `// ⟦axl_voices axo_command axe_bash_interactive⟧` annotations in
RBS0-SpecTop.adoc to new `// ⟦axvo_method axd_transient⟧` (or axvo_procedure)
form. Assign each operation to its group. Add axd_attended / axd_internal where
appropriate.

~25 command annotations + ~13 routine annotations + ~5 guide annotations + ~8 sequence
annotations in RBS0.

### Phase 3: Transform RBS0 operation subdocuments

Convert all operation subdocuments (RBSDC, RBSDN, RBSAC, etc.) from old
`// ⟦axs_inputs⟧` / `// ⟦axs_behavior⟧` section markers to new axho_* individual
markers. Each subdocument needs:
- axhob_operation opener
- axhop_parameter for each parameter
- axhos_step for each behavioral step
- axhoo_output for each output
- axhoq_precondition / axhog_guarantee where applicable
- axhoc_completion for signaling contract

**ALSO**: When processing each subdocument, repair any Strachey bracket usage on
regime hierarchy markers. Regime subdocuments currently use `// ⟦axhrb_regime⟧`
but should use bare prefix form `//axhrb_regime` per the AXLA spec. Fix both
operation voicings AND regime marker syntax in the same pass.

### Phase 4: Transform other specifications

- **VOS0** — Has ghost `axo_operation` references (used on voscaa_allocate,
  voscai_invitatory, voscar_release) that were NEVER defined in AXLA. These need
  careful treatment: determine what these action codes actually are in the new
  model (likely axo_procedure or axo_method), then transform. Do NOT assume they
  map 1:1 with any legacy term — `axo_operation` was undeclared vocabulary.
- **BUS0** — Uses axo_entity voicings (likely unaffected, but verify)
- **JJS0** — Uses axo_routine voicings that need transformation

### Phase 5: Finalize AXLA

- Rewrite completeness checklists for new model
- Delete all legacy terms listed in deprecation appendix
- Remove deprecation appendix itself

## Open Questions (resolve during pacing/execution)

- Exact RBS0 operation group inventory (Phase 1 will discover the full list)
- Whether axhob_operation should carry entity/group reference or rely on
  definition-site voicing having established it
- axs_errors disposition: individual markers or fold into step prose?

## Deferred (separate heats)

- **Regime operation markers** (axhro_kindle etc.) — whether regime operations
  are axo_methods on a regime entity is a future reconciliation
- **Control terms** (axc_*/rbbc_*) — step-level vocabulary, separate heat
- **Step labels / branching** — prose handles branching for now
- **Diptych syntax** (₣AZ) — designs syntax layer; connection noted in ₣AZ paddock
- **axe_rest_api / axe_daemon_runtime** — under review, not resolved in this heat

## References

- `Tools/cmk/vov_veiled/AXLA-Lexicon.adoc` — new terms and deprecation appendix
- `lenses/RBS0-SpecTop.adoc` — primary consumer, highest transformation volume
- `lenses/RBSDC-depot_create.adoc` — exemplar operation subdocument
- `lenses/RBSDN-depot_initialize.adoc` — exemplar attended operation
- `lenses/RBSRR-RegimeRepo.adoc` — regime subdoc pattern (the precedent)
- ₣AZ `cmk-diptych-prototype` — adjacent heat