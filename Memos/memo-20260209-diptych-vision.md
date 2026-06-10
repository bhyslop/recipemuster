# Diptych: A Dual-Representation Document Format for Human-AI Collaboration

## Status
Discovery draft v0.1 — Conversational exploration (2026-02-09)

## Origin

This memo records a design conversation between editor and digital mind (Claude Opus 4.6) that began with a practical frustration — AsciiDoc ceremony consuming tokens in concept model documents — and arrived at something larger: a yin-yang document format natively optimized for two consumers simultaneously.

The conversation started with JJSA-GallopsData.adoc as a concrete example. That document has ~160 attribute mapping entries, dense linked-term prose, layered voicing annotations, and careful MCM formatting. It is sophisticated, precise, and expensive.

## The Problem

AsciiDoc was designed for a processing pipeline: source → parser → attribute substitution engine → backend renderer → HTML/PDF. Every piece of ceremony exists so that pipeline can do its job.

An LLM has no pipeline. It reads token by token. Every piece of AsciiDoc ceremony is syntax the model looks *through* to get to meaning.

### Where the tokens actually go

A single mapping entry:
```
:jjdt_coronet:    <<jjdt_coronet,Coronet>>
```
carries the anchor name *twice*, six punctuation characters (`:`, `:`, `<<`, `,`, `>>`), and alignment whitespace. ~12 tokens conveying ~4 tokens of information. JJSA has ~160 such entries — roughly 1000 wasted tokens in the mapping section alone.

The body text fares better — MCM's one-term-per-line pattern already isolates linked terms. But every `{jjdt_coronet}` reference costs ~3-4 tokens where the information is ~2. And the definition ceremony (`[[anchor]]` + `// ⟦annotation⟧` + `{term}::`) takes three lines where one or two would suffice.

### The git granularity tension

Separately, git works best with fine-grained line changes. One word changed in a sentence shows the whole sentence as modified in `git diff`. For documents under active evolution — and concept model documents evolve intensely — this obscures the actual change.

One-word-per-line solves this: each word is a line, each change shows precisely what moved. But presenting one-word-per-line to an LLM *costs* tokens (newlines don't come free) and is unnecessary for comprehension.

These two needs — git granularity and LLM token efficiency — appear to be in tension. They are not.

## The Discovery: Diptych

A diptych is a two-panel wax tablet used in antiquity for sacred and legal writing. Two faces of the same artifact, hinged together. In liturgical tradition, diptychs listed the names of the living and the dead — one panel for each, same artifact.

The Diptych format is one document with two co-equal representations:

- **Recto** (git's face): one word per line, prefix-discriminated structure, sentence boundary tokens. Optimized for `git diff`, `git blame`, and fine-grained ancestry tracking.
- **Verso** (LLM's face): words joined into sentences, prefix characters preserved, punctuation resolved. Optimized for token efficiency and immediate semantic comprehension.

Neither representation is degraded. Each is the *native* form for its consumer. The transform between them is mechanical, lossless, and dumb: join words with spaces, split on spaces. Sentence boundary tokens handle punctuation. Prefix characters pass through unchanged.

The codec is the hinge. The hinge is trivial.

## Prefix Character Grammar

The format uses single-character prefixes to discriminate structural roles. Each line's purpose is immediately clear from its first character. No closing delimiters. No matching pairs. No indirection.

| Character | Role | Replaces |
|-----------|------|----------|
| `$` | Attribute reference — linked term in body text | `{cat_term}` |
| `@` | Mapping entry — anchor + display text + variants | `:cat_term: <<anchor,Display>>` |
| `#` | Anchor definition point — term is defined here | `[[anchor]]` |
| `^` | Annotation — voicing/type metadata | `// ⟦content⟧` |
| `~N` | Section level N | `==`, `===`, `====` |
| `-` | Bullet item | `*` (unchanged semantics) |

### Why prefix characters are cheap for LLMs

This was a key question: would prefix characters create "decoding overhead" for the model? The answer, from direct introspection, is no. An LLM reads `$jjdt_coronet` as fluidly as `{jjdt_coronet}`. The prefix character is one token, it's unambiguous, and meaning is immediate. There is no parser to simulate, no substitution to resolve, no closing delimiter to match.

The deepest win is eliminating **AsciiDoc attribute substitution as a processing concept** while preserving the **concept mapping as a semantic concept**. In AsciiDoc, I read `{jjdhr_heat}` and must mentally resolve it through the mapping to know what's being talked about. With `$jjdhr_heat`, the identifier IS the reference — the mapping section tells me the display text, the category tells me the role, the annotation tells me the type classification. All of that is load-bearing. But the AsciiDoc substitution engine is gone.

### Mapping section compression

A term family collapses from three lines:
```
:jjdt_coronet:    <<jjdt_coronet,Coronet>>
:jjdt_coronet_s:  <<jjdt_coronet,Coronets>>
:jjdt_coronet_p:  <<jjdt_coronet,Coronet's>>
```
to one:
```
@jjdt_coronet Coronet /s Coronets /p Coronet's
```

~36 tokens → ~8 tokens. Multiply across 160 entries.

### Definition compression

A definition ceremony collapses from:
```
[[jjdt_firemark]]
// ⟦axl_voices axt_string⟧
{jjdt_firemark}::
The identity of a
{jjdhr_heat}.
```
to:
```
#jjdt_firemark ^axl_voices axt_string
The identity of a
$jjdhr_heat.
```

Three structural lines become one. The prose body is unchanged.

## Sentence Boundary Tokens

Rather than relying on period detection and capitalization heuristics, the recto format uses explicit sentence boundary tokens. A single character (to be determined) signals: "attach period to previous word, capitalize next word, this is a semantic sentence boundary."

This eliminates ambiguity (is `.` end-of-sentence, abbreviation, or path component?) and enables auto-punctuation for lists. A list-mode token followed by items could auto-generate oxford comma patterns — the semantic content is "these items form a list," not "these items have commas between them."

### Punctuation attachment in recto

In the one-word-per-line storage format, punctuation attaches to its word's line:
```
The
identity
of
a
$jjdhr_heat.
```

The period stays with `$jjdhr_heat` because the sentence boundary token preceding the next sentence governs the join. The codec doesn't need to guess about punctuation — it follows the tokens.

## Formatting State Machine

Bold, italic, and other formatting use toggle semantics rather than paired delimiters. A token enters bold mode; content flows; a token exits (or a reset-to-normal token clears all). Default is to continue the current run's formatting.

For documents where formatting is sparse and purposeful (concept models), this costs almost nothing. And it eliminates the matching-delimiter problem entirely.

## Liturgical Vocabulary

The naming draws from the existing Vox Obscura liturgical tradition:

### Canon

The complete concept model source in Diptych format. The authoritative collection of vocabulary, definitions, voicings, and annotations. What JJSA-GallopsData or RBSA-SpecTop represent. Everything is here; all composed views derive from here.

### Lectionary

The rules governing which terms activate, at what depth, for which occasion. This is the successor to MCM's task lens / focal depth / lens specification machinery. A lectionary specifies: for this work context, activate these terms at core depth, these at expanded, suppress these.

In liturgy, the lectionary is the calendar and rubric that tells the celebrant which readings to use, in what order, for which feast day. The lectionary doesn't contain the readings themselves — it contains the *rules for selection*.

### Lectio

A composed view derived from a canon according to lectionary rules. What a constrained agent (haiku, sonnet) receives for bounded mechanical work. Terms resolved to display text (possibly with guillemet notation `«Heat»` from the earlier ClaudeMark exploration), concept machinery stripped, scope bounded to the task.

The Getting Started guide (RBSGS-GettingStarted.adoc) is already a lectio — it selects concepts from the full RBSA vocabulary and composes them into a procedural narrative for newcomers. Each `include::` subdocument in RBSA-SpecTop is a potential lectio boundary.

### The pipeline

```
Canon (Diptych source)
  → Recto (git reads word-per-line)
  → Verso (opus reads joined prose, full concept density)
  → Lectionary rules (focal depth, term selection, aspect emphasis)
  → Lectio (haiku/sonnet reads task-focused extract)
```

Opus works with the verso — full concept model with all mappings, voicings, annotations. This is where specification work happens, where design decisions are made, where the concept constellation evolves.

Haiku/sonnet work with lectiones — bounded extracts where terms are resolved, scope is clear, and the task is mechanical. "Rename this field following the pattern in these three files." No concept model navigation needed.

## Relationship to Existing Concepts

### MCM (Meta Concept Model)

MCM defines the *semantic structure* of concept models: linked terms, categories, variants, domains, definition forms, annotations. Diptych does not replace MCM — it replaces the *AsciiDoc expression* of MCM. The semantic concepts (categories, voicings, focal depths) carry forward. The syntax changes.

MCM's task lens / focal depth vocabulary maps directly to the lectionary/lectio terminology. The concepts are the same; the names are being brought into the liturgical vocabulary that already governs the broader system.

### ClaudeMark

The ClaudeMark memo (AXMCM-ClaudeMarkConceptMemo.md) was an earlier exploration of the same intuition: LLMs process different documentation formats with vastly different efficiency, and preprocessing documents into optimized formats yields significant improvements.

ClaudeMark's useful insights fold into Diptych:
- Guillemets (`«term-id»`) as resolved term references in lectio output
- The "mezzanine layer" concept (not for human authoring, for LLM consumption)
- The observation that LLM pattern recognition operates in parallel while generation is sequential

ClaudeMark as a separate named format is superseded. The lectio output inherits its ideas.

### Vox Obscura / Voce Viva

The Diptych codec, the lectionary engine, and the recto/verso transforms are hidden infrastructure — Vox Obscura. Users never see the word-per-line storage format or the codec mechanics. They interact with the verso (or lectio) through transparent tooling.

For prefix allocation, Diptych infrastructure likely belongs under the `vo` cipher since it serves all projects. The codec would be implemented in Rust within `vvr`, the lectionary engine similarly. MCP tools or transparent file hooks would present the verso to the LLM seamlessly.

## What We Are NOT Proposing

This document does not propose word-per-line as a general-purpose text format. It proposes word-per-line specifically for **concept model documents** — the dense, heavily-linked, annotation-rich specifications that are the backbone of the system's precision.

Plain prose documents (READMEs, memos like this one, guides without linked terms) gain nothing from this treatment. The value emerges specifically from the intersection of: high concept density + frequent linked-term references + git ancestry tracking needs + LLM presentation requirements.

## Implementation Vision

### Phase 1: Format specification
Define the complete Diptych grammar — every prefix character, the sentence boundary token, list-mode tokens, formatting toggles, code block fencing, the mapping section syntax. Produce a specification document (itself a Diptych canon, naturally).

### Phase 2: Codec
Implement the recto↔verso transform. Likely in Rust within `vvr` as a new subcommand. The transform is a simple state machine:
- **PROSE** → accumulate words, join with space (verso) or split on space (recto)
- **STRUCTURAL** → emit line as-is (prefix-discriminated)
- **CODEBLOCK** → pass through verbatim until fence close
- **BLANKLINE** → emit blank line, flush accumulated prose
- **SENTENCE** → handle boundary token (period attachment, capitalization)

### Phase 3: Transparent presentation
MCP tools (or equivalent) that virtualize file access. When opus reads a Diptych file, it receives the verso. When opus writes, the output is transformed back to recto. The codec is invisible.

### Phase 4: Lectionary engine
Compile lectiones from canons according to focal depth rules. This replaces the not-yet-implemented `/cma-render` pipeline. The lectionary engine reads a canon, applies selection rules, and emits a lectio in simplified format for constrained agents.

### Phase 5: Migration
Convert existing concept model documents (JJSA, RBSA, MCM, VLS, BUSA, etc.) from AsciiDoc to Diptych format. This is the largest effort but can be done incrementally — one document at a time, validating round-trip fidelity.

## A Note on Scale

This work is motivated by scale. Current concept model documents are sophisticated but manageable (JJSA: ~1500 lines, RBSA: ~2500+ lines). The system is heading toward substantially larger documents. Token efficiency and transparent presentation become critical as documents approach and exceed model context limits.

The partial-read capability is essential: the verso presentation layer should support presenting sections rather than whole files. Section-level tokens (`~2`, `~3`) make this natural. Mapping section elision (presenting a compact summary rather than the full mapping when editing body text) is another scaling lever.

The lectio pipeline is the ultimate scaling answer: for any given task, present only what's needed, at the depth needed, resolved to the vocabulary needed. The canon can be arbitrarily large; the lectio is always bounded.

## Reflections from the Conversation

The most striking moment in this exploration was the recognition that we were not designing a compression scheme or an optimization hack. We were discovering that the document *already has two natural forms* — one for the machine that tracks ancestry (git) and one for the machine that reads meaning (the LLM) — and that the transform between them is trivially mechanical.

The existing MCM format, with its one-term-per-line discipline and careful structural formatting, was already reaching toward this. Diptych names it, formalizes it, and pushes it to its logical conclusion.

The liturgical vocabulary — canon, lectionary, lectio — emerged naturally from the existing Vox Obscura tradition and captures the relationships precisely. A canon is authoritative and complete. A lectionary governs selection. A lectio is the reading appropriate for the moment. These are not metaphors being forced onto the system; they are the actual relationships between the artifacts.

The decision to use prefix characters rather than paired delimiters was informed by direct model introspection: prefix tokens are genuinely cheap to process, closing delimiters are genuinely unnecessary overhead. This is not an opinion about elegance — it is a report on cognitive architecture.

And the recognition that the concept mapping is preserved (indeed, elevated) while the AsciiDoc substitution engine is eliminated — that was the moment the design clicked. The mapping is the spine. The spine stays. The ceremony around it goes.

---

*This memo emerged from a single conversation exploring how concept model documents could be presented more efficiently to language models. The answer turned out to be: stop adapting the document to fit an existing format's ceremonies, and design the format around the document's actual consumers.*
