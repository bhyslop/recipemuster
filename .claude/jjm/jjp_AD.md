# Paddock: vos-spec-and-tooling

## Context

Specification quality and enforcement: refine VOS/AXLA concept models, reconcile spec with implementation, and build tooling to enforce naming discipline.

**Problem:** VOS has accumulated rough edges during rapid development:
- Liturgy vocabulary has legacy references and unclear definitions
- Envelope concept (suffix vs frame) deferred but needed
- State machine patterns in JJD lack AXLA formalization
- Spec vs implementation gaps after MVP work
- Prefix naming discipline relies on Claude remembering rules

**Solution:** Two-pronged approach:
1. **Spec refinement** — Clean up VOS/AXLA, formalize deferred concepts
2. **Tooling** — Build `vvx check` to enforce prefix trees at build time

## Architecture

**Completed:**
- ₢ADAAG — liturgy-dispatch-dispersion-study (verified extraction safe)
- ₢ADAAF — vls-bus-extraction (created VLS + BUS specs)

**Spec Cleanup (3 paces):**
- ₢ADAAH — vos-cleanup-legacy-refs (vost_cipher → voslc_cipher)
- ₢ADAAI — vls-vesture-cleanup (fix vesture definitions)
- ₢ADAAJ — bus-dispatch-cleanup (fix colophon/tabtarget definitions)

**Concept Development (3 paces):**
- ₢ADAAA — define-envelope-vesture-component (formalize in VLS)
- ₢ADAAC — liturgy-state-machine-vocabulary (AXLA patterns)
- ₢ADAAD — vos-implementation-reconciliation (spec vs reality)

**Enforcement Tooling (1 pace):**
- ₢ADAAE — vof-prefix-tree-checker (`vvx check` for CI)

**Abandoned:**
- ₢ADAAB — vos-liturgy-cleanup-batch (superseded by ADAAH/ADAAI/ADAAJ split)

## References

- Tools/vok/vov_veiled/VOS-VoxObscuraSpec.adoc — VOS spec (kit distribution)
- Tools/vok/vov_veiled/VLS-VoxLiturgicalSpec.adoc — VLS spec (naming vocabulary)
- Tools/buk/vov_veiled/BUS-BashUtilitiesSpec.adoc — BUS spec (dispatch vocabulary)
- Tools/cmk/vov_veiled/AXLA-Lexicon.adoc — AXLA shared vocabulary
- Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc — MCM patterns
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc — JJD (state machine exemplar)
- Tools/vok/vof/ — VOF crate (prefix registry)

## Key Constraints

1. **Spec-first for concepts** — Formalize in VOS/AXLA before building tooling
2. **Implementation reconciliation last** — Do after MVP so we compare against real code
3. **Tooling reduces context cost** — Goal is zero prefix rules in CLAUDE.md

## Steeplechase

### 2026-01-17 - Heat Created

Restrung 5 paces from ₣AA (vok-fresh-install-release) to separate spec/tooling work from MVP delivery.
