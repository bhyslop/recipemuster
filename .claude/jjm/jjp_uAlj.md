# Paddock: rbk-axla-term-voicing-scrub

## Goal

Redesign AXLA's operation vocabulary to replace the fragmented procedure hierarchy
(axo_command, axo_guide, axo_routine, axo_sequence) with a clean model based on
two primary voicings, operation groups, and hierarchy markers for subdocuments.

## Design Decisions (sessions: 2026-02-27 axla-procedure-repair, 2026-03-02 operations-for-the-win)

### 1. Collapse procedure hierarchy to two primary voicings

**`axo_procedure`** — standalone executable operation. No entity affiliation required.
**`axo_method`** — entity-affiliated executable operation. Grammar rule: the definition
text's second linked term must identify the affiliated entity (or group, if axd_grouped).

These replace: axo_command, axo_guide, axo_routine. axo_sequence was already "not a
procedure type" per AXLA but was used as one — it gets retired too.

The method voicing is a **syntactic constraint** (grammar production rule), not just
a classification. A linter enforces that methods declare their entity.

### 2. Drop language/environment dimension

axe_bash_interactive, axe_bash_scripted, axe_bash_unattended are retired. Implementation
language is not a behavioral characteristic. Projects live in steady state where languages
don't change. The voicing should not know "this is bash."

### 3. Two surviving dimensions from the old hierarchy

**`axd_attended`** (optional) — human presence required during execution. Absence means
unattended. This is the real kernel of the old command/guide split.

**`axd_internal`** (optional) — implementation building block, not consumer-facing.
Absence means external. This is BCG's external/internal contract surface distinction.

### 4. Lifecycle dimensions retained

`axd_transient`, `axd_longrunning`, `axd_periodic` — exactly one required on any
procedure or method. Orthogonal to the new model, unchanged.

### 5. Operation groups (`axo_group`)

A new primary voicing. Groups are named collections of operations sharing an affiliated
entity. Defined at S0 level as linked terms.

Definition-site voicing: `axvo_group`
- 1st linked term: self
- 2nd linked term: affiliated entity (must voice axo_entity)

Groups can carry dimensions that apply to their members:
- `axd_tabtarget` — members exposed as tabtargets
- `axd_slash_command` — members exposed as slash commands
- `axd_rest_endpoint` — members exposed as REST endpoints

These are group-only dimensions (valid only on axvo_group). Precedent: axd_conditional
is valid only on axvr_group; axf_bash valid only on axvr_regime.

### 6. axd_grouped dimension

A dimension applicable to both procedure and method. Imposes a positional requirement
on the definition text, but the position shifts based on the base voicing:

- On `axvo_procedure`: 2nd linked term must be an operation group
- On `axvo_method`: 3rd linked term must be an operation group
  (because 2nd is already the entity, required by the method voicing itself)

Absence of axd_grouped means the operation is not in a named group. Methods without
axd_grouped simply have entity in 2nd position, no group.

### 7. Definition-site voicing annotations (axvo_*)

Parallel to axvr_* for regimes. Pure AXLA terms on annotation line — NO mixing with
project-specific linked terms. Project terms appear via lookahead in definition text.

```
// ⟦axvo_group⟧
{rbtgog_depot}:: Operations on the {rbtge_depot} lifecycle.

// ⟦axvo_method axd_transient⟧
{rbtgo_depot_create}:: A {rbtgog_depot} operation that creates...

// ⟦axvo_method axd_transient axd_attended⟧
{rbtgo_depot_initialize}:: A {rbtgog_depot} operation that completes...

// ⟦axvo_method axd_transient axd_internal⟧
{rbtoe_payor_authenticate}:: A {rbtgog_payor} operation that authenticates...

// ⟦axvo_procedure axd_transient⟧
{rbtgo_backup_all}:: Backs up all depot state...
```

### 8. Detail-site hierarchy markers (axho_*)

Parallel to axhr* for regimes. Bare prefix form (no Strachey brackets — though note
regime subdocs currently still USE brackets; the bare form is AXLA's specified form).

| Marker | Purpose | Lookahead |
|---|---|---|
| axhob_operation | Opens detail block, identifies which operation | 1 linked term + dimensions on marker line |
| axhop_parameter | A named input parameter | 1 linked term |
| axhoo_output | A named output | 1 linked term |
| axhoq_precondition | Required state before execution | 1 linked term |
| axhog_guarantee | Promised state after execution | 1 linked term |
| axhos_step | A behavioral step (addressable) | 1 linked term |

axhob_operation carries lifecycle and attended dimensions:
```
//axhob_operation axd_transient
{rbtgo_depot_create}
```

These replace the old axs_* section motifs (axs_inputs, axs_behavior, axs_outputs,
axs_completion, axs_preconditions, axs_postconditions, axs_errors). Individual markers
per item instead of section wrappers.

### 9. New AXLA linked terms for structural locales

`axl_definition_site` — where a linked term is defined (anchor + voicing annotation +
definition text) in the parent document.

`axl_detail_site` — where a linked term's details are elaborated (hierarchy markers +
prose). Typically a subdocument but not necessarily.

These replace informal language ("parent document", "subdocument", "between anchor
and definition") with precise, referenceable vocabulary.

### 10. Migration strategy

1. Add new terms to AXLA (purely additive, no existing content changes)
2. Write mapping appendix in AXLA (old→new, obsolescence plan)
3. Transform RBS0 operation subdocuments from old to new
4. Transform BUS0, JJS0, VOS0
5. Delete obsoleted terms from AXLA

## Deferred

- **Regime operation markers** (axhro_kindle etc.) — leave as-is. Whether regime
  operations are axo_methods on a regime entity is a future reconciliation.
- **Control terms** (axc_*/rbbc_*: call, require, store, fatal, etc.) — step-level
  vocabulary, orthogonal to this structural redesign. Separate heat.
- **Step labels / branching / goto** — prose handles branching for now.
  Step linked terms provide cross-reference targets.
- **Diptych syntax** (₣AZ) — ₣Aj designs voicing semantics, ₣AZ designs syntax.
  Connection noted in ₣AZ paddock.
- **Ghost axo_operation in VOS0** — clean up during VOS0 transformation pass.
- **axe_human_guide** — absorbed into axd_attended; remove during transformation.

## Open Questions (resolve during execution)

- Exact RBS0 operation groups to define (depot, ark, image, payor, governor, sa, etc.)
- Whether axhob_operation should carry the entity/group reference or rely on the
  definition-site voicing having already established it
- Naming for the grouping dimension if axd_grouped proves awkward

## References

- `Tools/cmk/vov_veiled/AXLA-Lexicon.adoc` — target document for new terms
- `lenses/RBS0-SpecTop.adoc` — primary consumer, ~25 operations to transform
- `lenses/RBSDC-depot_create.adoc` — exemplar operation subdocument
- `lenses/RBSDN-depot_initialize.adoc` — exemplar attended operation
- `lenses/RBSRR-RegimeRepo.adoc` — regime subdoc pattern to parallel
- `Memos/memo-20260209-diptych-format-study.md` — adjacent ₣AZ vision
- ₣AZ `cmk-diptych-prototype` — adjacent heat, syntax layer