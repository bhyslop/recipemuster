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

### MCM Vocabulary

| Term | Prefix | Meaning |
|------|--------|---------|
| **Quoin** | `mcm_` | A formal identifier with full cataloguing: attribute reference in the mapping section, anchor at the definition site, and definition text. The cornerstone — addressable from anywhere, tracked across the constellation. |
| **Mapping Section** | `mcm_` | The block of attribute references at the top of a document, between `tag::mapping-section[]` markers. Where quoins are registered. |
| **Concept Model** | `mcm_` | A unified constellation of linked terms whose relationships give meaning to a domain. Manifested as an AsciiDoc document following MCM patterns. |
| **Category** | `mcm_` | The prefix group (e.g., `bzsn_`, `bzsdp_`, `mcm_`) that scopes a set of related quoins. |
| **Variant** | `mcm_` | Suffix modifiers on attribute references: `_s` plural, `_p` possessive, `_ed` past, `_ing` progressive. |
| **Annotation** | `mcm_` | An AsciiDoc comment line carrying structured metadata as space-separated prefix terms. Position determines role: relationship first, primary motif second, dimensions following. Example: `//axl_voices axk_premise`. |

### AXLA Vocabulary

| Term | Prefix | Meaning |
|------|--------|---------|
| **Motif** | `axl_` | A reusable conceptual pattern from AXLA that a quoin can voice. The shared vocabulary across concept models. |
| **Voicing** | `axl_` | An annotation at a definition site declaring which AXLA motif a quoin instantiates (e.g., `// axl_voices axk_premise`). |
| **Premise** | `axk_` | A motif representing a declared design constraint that bounds system complexity by stating what the system will not handle. |
| **Definition Site** | `axl_` | The location where a quoin's `[[anchor]]` and definition text appear. Where voicings are declared. |

### Concept Model Patterns

- **Linked Terms**: `{category_term}` — references defined vocabulary
- **Attribute References**: `:category_term: <<anchor,Display Text>>` — in mapping section
- **Anchors**: `[[anchor_name]]` — definition targets
- **Annotations**: `//axl_voices axk_premise` — prefix-discriminated comment lines (no space after `//`, letter distinguishes from regular comments)

### Minting Guidance

Before introducing new quoin prefixes in an MCM document, consult the CLAUDE.md "Quoin Sub-Letter Discipline" subsection and minting memo Pattern J (`Memos/memo-20260110-acronym-selection-study.md`). The rule: uniform `prefixXY_word` shape, hard 2-letter ceiling, within-domain Y monosemy, documented sub-letter legend in the spec's mapping section.

**Available commands:**
- `/cma-normalize` - Apply full MCM normalization (haiku)
- `/cma-render` - Transform to ClaudeMark (sonnet)
- `/cma-validate` - Check links and annotations
- `/cma-prep-pr` - Prepare upstream contribution
- `/cma-doctor` - Validate installation

**Subagents:**
- `cmsa-normalizer` - Haiku-enforced MCM normalization (text, mapping, validation)

For full MCM specification, see `Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc`.

**Important**: Restart Claude Code session after installation for new commands and subagents to become available.
