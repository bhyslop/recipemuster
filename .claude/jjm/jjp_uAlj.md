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

## Annotation Line Purity Rule

Annotation lines contain ONLY AXLA terms. Project-specific linked terms appear in
the definition text via positional requirements imposed by voicings:
- axvo_method: 2nd linked term in definition text = entity
- axvo_group: 2nd linked term in definition text = entity
- axd_grouped: next available position = group

This matches the established regime pattern (axvr_variable requires parent regime
as 2nd linked term in definition text).

## Proposed Pace Plan

### Phase 1: RBS0 S0-level work

1. **define-operation-groups-in-s0**
   Create rbtgog_* group linked terms with axvo_group voicings in RBS0-SpecTop.adoc.
   Groups to define: rbtgog_depot, rbtgog_ark, rbtgog_image, rbtgog_payor,
   rbtgog_governor, rbtgog_sa, rbtgog_rubric, rbtgog_access, others as discovered.
   Each group: linked term + axvo_group voicing + entity reference in definition.

2. **transform-s0-definition-sites**
   Convert all ~25 rbtgo_* command annotations and ~5 guide annotations from
   `// ⟦axl_voices axo_command axe_bash_interactive⟧` to
   `// ⟦axvo_method axd_transient⟧` (or axvo_procedure). Assign each to its group
   via definition text. Add axd_attended where applicable (depot_initialize,
   payor_establish, payor_refresh, gdc_establish, quota_build).

3. **design-rbtoe-internal-routines**
   The ~13 rbtoe_* routines need group affiliation decisions. Proposal:
   role-specific routines (rbtoe_payor_authenticate etc.) affiliate with their
   role's group as axvo_method axd_internal. Cross-cutting utilities
   (rbtoe_jwt_oauth_exchange, rbtoe_rbra_load, rbtoe_oauth_refresh) become
   axvo_procedure axd_internal (standalone, no entity). Transform annotations.

### Phase 2: Regime subdocument bracket repair

4. **repair-regime-subdoc-bracket-syntax**
   Convert all 7 regime subdocuments from Strachey bracket form
   `// ⟦axhrb_regime⟧` to bare prefix form `//axhrb_regime` per AXLA spec.
   Files: RBSRR, RBSRA, RBSRP, RBSRO, RBSRS, RBSRV, RBRN.
   Mechanical, bridleable.

### Phase 3: RBS0 operation subdocuments

Convert from old axs_* section markers to new axho_* individual markers.
Each subdocument needs: axhob_operation opener, axhop_parameter per parameter,
axhos_step per step, axhoo_output per output, axhoq_precondition / axhog_guarantee
where applicable, axhoc_completion for signaling contract.

Complex subdocuments (one pace each):
5. **transform-RBSDC-depot-create** — exemplar, largest subdoc (~205 lines)
6. **transform-RBSDN-depot-initialize** — attended exemplar (~140 lines)
7. **transform-RBSPE-payor-establish**
8. **transform-RBSPI-payor-install**
9. **transform-RBSPR-payor-refresh**
10. **transform-RBSGR-governor-reset**
11. **transform-RBSRC-retriever-create**
12. **transform-RBSDI-director-create**
13. **transform-RBSGD-gdc-establish**
14. **transform-RBSRI-rubric-inscribe**
15. **transform-RBSTB-trigger-build**
16. **transform-RBSQB-quota-build**

Smaller subdocuments (batched):
17. **transform-RBSDD-depot-destroy**
18. **transform-RBSDL-depot-list**
19. **transform-ark-subdocs** — RBSAA, RBSAB, RBSAC, RBSAS (4 files)
20. **transform-image-subdocs** — RBSIL, RBSID, RBSIR (3 files)
21. **transform-sa-subdocs** — RBSSL, RBSSD (2 files)
22. **transform-probe-subdocs** — RBSAO, RBSAJ (2 files)

### Phase 4: Other specifications

23. **transform-VOS0-operations**
    4 axo_command (release, install, uninstall, freshen — note axe_rust_impl dropped),
    5 axo_routine (lock, commit, guard, probe, init),
    3 ghost axo_operation (allocate, invitatory, release — never defined in AXLA,
    determine correct voicing in new model).
    Also review VOS0's own voss_* section motifs that voice the deprecated axs_* terms.

24. **transform-JJS0-routines**
    4 axo_routine (load, save, persist, wrap) → axvo_procedure or axvo_method.

25. **transform-JJS0-interface-layer**
    ~18 axi_cc_claudemd_verb, ~15 axi_cc_slash_command, ~25 axi_cli_subcommand,
    plus axa_cli_option/flag terms. Review against new model. These are interface
    voicings not operation voicings — may be unaffected, but verify and decide.

26. **verify-BUS0-unaffected**
    BUS0 uses axo_entity and axrg_* regime voicings. Confirm no operation voicings
    need transformation. Quick verification pace.

### Phase 5: Finalize

27. **rewrite-completeness-checklists**
    Replace Procedure/Command/Guide/Lifecycle completeness sections in AXLA with
    new model checklists based on axvo_procedure, axvo_method, axvo_group, and
    axho_* markers.

28. **delete-legacy-terms**
    Remove all deprecated terms listed in AXLA deprecation appendix:
    axo_command, axo_guide, axo_routine, axo_sequence, axs_inputs, axs_behavior,
    axs_outputs, axs_completion, axs_preconditions, axs_postconditions, axs_errors,
    axe_bash_interactive, axe_bash_scripted, axe_bash_unattended, axe_human_guide,
    and the old completeness checklists. Remove deprecation appendix itself.

29. **note-future-heat-mkr-opss-scr**
    The mkr_*, opss_*, opbs_*, opbr_*, scr_* operations in RBS0 (bottle, network,
    sentry, security) have non-rbtgo prefixes and were out of scope. Slate an itch
    or future heat to bring them up to the standards established in this heat.

## Pacing Notes

- Paces 1-3 must be sequential (groups before definition-sites before internals)
- Pace 4 (regime bracket repair) is independent, can run anytime
- Paces 5-6 are interactive exemplars establishing the pattern
- Paces 7-22 are likely bridleable once exemplars prove the pattern
- Paces 23-26 are independent of each other and of RBS0 subdocument work
- Phase 5 depends on all prior phases completing

## Remaining Open Questions

- Exact RBS0 operation group inventory (Phase 1 will discover the full list)
- Whether axhob_operation should carry entity/group reference or rely on
  definition-site voicing having established it
- axs_errors disposition: individual markers or fold into step prose?
- VOS0 voss_* section motif disposition (voices deprecated axs_* terms)
- JJS0 interface layer: are axi_cc_claudemd_verb / axi_cc_slash_command affected?

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
- `Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc` — ghost axo_operation, axe_rust_impl
- `Tools/jjk/vov_veiled/JJS0-GallopsData.adoc` — routines + interface layer
- `Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc` — verify-only
- ₣AZ `cmk-diptych-prototype` — adjacent heat