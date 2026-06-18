## Concept Model Kit Configuration

Concept Model Kit (CMK) is installed for managing concept model documents.

**Configuration:**
- Lenses directory: `lenses`
- Kit path: `Tools/cmk/README.md`
- Upstream remote: `OPEN_SOURCE_UPSTREAM`

### CMK File Acronyms

| Acronym | File | Description |
|---------|------|-------------|
| **MCM** | `cmk/vov_veiled/MCM-MetaConceptModel.adoc` | Meta Concept Model — the spec governing all concept model documents |
| **AXLA** | `cmk/vov_veiled/AXLA-Lexicon.adoc` | Axial Lexicon — shared vocabulary of motifs reusable across concept models |
| **AXMCM** | `cmk/vov_veiled/AXMCM-ClaudeMarkConceptMemo.md` | ClaudeMark concept memo |
| **ACG** | `cmk/vov_veiled/ACG-AllocationCodingGuide.md` | Allocation Coding Guide — source-side complement to MCM: reference the home, don't recreate (values→constants, concepts→quoin-refs). Veiled/proprietary move-catalog; guide-family sibling to BCG/RCG/WSG/CBG. |
| **GMG** | `cmk/vov_veiled/GMG-GuideMetaGuide.md` | Guide Meta-Guide — the guide for writing guides: the canonical section skeleton and the family's shared framing conventions (foreign-environment sibling, two-genres split, cited-rule scheme). Homes the guide-family form once so a guide cites the convention rather than re-deriving it. The spec-side analogue of MCM/AXLA, for guides. |

### MCM Vocabulary

| Term | Prefix | Meaning |
|------|--------|---------|
| **Quoin** | `mcm_` | A formal identifier with full cataloguing: attribute reference in the mapping section, anchor at the definition site, and definition text. The cornerstone — addressable from anywhere, tracked across the constellation. |
| **Rivet** | `mcm_` | A formal identifier for a *normative proposition* (invariant, deliberate deviation, foreign-behavior signature, Palisade membrane) the model defines once and code/tests cite by ID. **Format:** `{proj}r_<opaque-tail>` — e.g. `RBr_a3f`, `JJr_a7c`. Unlike a quoin, a rivet ID is **opaque**: the tail carries no meaning, so it leaks no semantics into the open code that ships without the closed spec (a *readable* name is a quoin, never a rivet). Kind is declared at the definition site by an `axvc_` voicing, never encoded in the ID. **Uniqueness:** generate a tail, `grep` the full ID repo-wide, adopt on zero hits — grep is both the uniqueness check and the census; no registry. Emittable: in comment-free dialects the citation rides the execution-time announcement (JDG `JDo_101`). Formal definition: MCM `mcm_rivet`; allocation shape: ACG "cited-constraint anchor". |
| **Mapping Section** | `mcm_` | The block of attribute references at the top of a document, between `tag::mapping-section[]` markers. Where quoins are registered. |
| **Concept Model** | `mcm_` | A unified constellation of linked terms whose relationships give meaning to a domain. Manifested as an AsciiDoc document following MCM patterns. |
| **Category** | `mcm_` | The prefix group (e.g., `bzsn_`, `bzsdp_`, `mcm_`) that scopes a set of related quoins. |
| **Variant** | `mcm_` | Suffix modifiers on attribute references: `_s` plural, `_p` possessive, `_ed` past, `_ing` progressive. |
| **Annotation** | `mcm_` | An AsciiDoc comment line carrying structured metadata as space-separated prefix terms. Position determines role: primary motif first, dimensions following; the voices relationship is implicit. Example: `//axk_premise`. Legacy `//axl_voices …` form is read-only — never author it anew. |
| **Ashlar** | `mcm_` | A quoin that faces the operator — must be fair-faced (first-contact actionable), draws from the coffer (bounded operator vocabulary), registers on the project's broadside. Words in error output are ashlar. Definition-site marker: `axd_ashlar`. |
| **Hearting** | `mcm_` | Interior names — uncatalogued, prefix discipline only. Deliberate plainness so ashlar care stays affordable. |
| **Broadside** | `mcm_` | The project's single public sheet of ashlar vocabulary; registration there completes a mint. |

### AXLA Vocabulary

| Term | Prefix | Meaning |
|------|--------|---------|
| **Motif** | `axl_` | A reusable conceptual pattern from AXLA that a quoin can voice. The shared vocabulary across concept models. |
| **Voicing** | `axl_` | An annotation at a definition site declaring which AXLA motif a quoin instantiates (e.g., `//axk_premise`). |
| **Premise** | `axk_` | A motif representing a declared design constraint that bounds system complexity by stating what the system will not handle. |
| **Definition Site** | `axl_` | The location where a quoin's `[[anchor]]` and definition text appear. Where voicings are declared. |

### Concept Model Patterns

- **Linked Terms**: `{category_term}` — references defined vocabulary
- **Attribute References**: `:category_term: <<anchor,Display Text>>` — in mapping section
- **Anchors**: `[[anchor_name]]` — definition targets
- **Annotations**: `//axk_premise` — prefix-discriminated comment lines (no space after `//`, letter distinguishes from regular comments); primary motif first, `axd_` dimensions follow, voices relationship implicit. Legacy `//axl_voices …` form is read-only.

### Minting Guidance

Before introducing new quoin prefixes in an MCM document, consult the CLAUDE.md "Quoin Sub-Letter Discipline" subsection and minting memo Pattern J (`Memos/memo-20260110-acronym-selection-study.md`). The rule: uniform `prefixXY_word` shape, hard 2-letter ceiling, within-domain Y monosemy, documented sub-letter legend in the spec's mapping section.

**Available commands:**
- `/cma-prep-pr` - Prepare upstream contribution

For full MCM specification, see `Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc`.
