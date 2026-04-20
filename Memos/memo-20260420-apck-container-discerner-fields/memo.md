<!--
Copyright 2026 Scale Invariant, Inc.
SPDX-License-Identifier: Apache-2.0
See LICENSE for terms.
-->

# APCK Container — Discerner Field Reference

**Date:** 2026-04-20
**Pace:** ₢A9AAf (container-discerner-fields)
**Prerequisite:** `Memos/memo-20260420-apck-container-fixture-assay/` (establishes "what is happening"; this memo establishes "what does each field mean")
**Feeds:** ₢A9AAV (combining) — design reference, not a rule list
**Fixtures referenced:** `epic_initial.json` (index 10001), `epic_geriatric.json` (index 10002), sibling to the fixture-assay memo

## Scope

Every field the container emits for each of Stanford, scispaCy, and Stanza,
with three layers per field:

1. **Authoritative semantics** — what the library says the field represents.
2. **Observed behavior** — what actually appears in the two fixtures.
3. **Utility assessment** — what the field could signal in combining, plus
   known failure modes and redundancies.

Out of scope: combining rules (₢A9AAV), quoin minting in APCS0 (deferred
until combining earns each field), Python changes, wire-vocabulary changes.

## Version pinning

Inherited from the fixture-assay memo. This reference is valid only for:

| Library | Version | Model / pipeline |
|---|---|---|
| `transformers` | 4.46.3 | `StanfordAIMI/stanford-deidentifier-base` |
| `torch` | 2.5.1-cpu | (CPU inference) |
| `scispacy` | 0.5.5 | `en_core_sci_md` |
| `stanza` | 1.9.2 | English UD EWT + OntoNotes NER |

Future upgrades date the memo; refresh in a separate pace.

## Wire envelope (reminder)

Per `APCPS` Container Architecture + `APCS0` Wire Protocol, each `{N}.json`
the container writes has shape:

```
{ "index": <int>,
  "stanford": { "findings": [ ... ] },
  "spacy":    { "findings": [ ... ] },
  "stanza":   { "findings": [ ... ] }
}
```

Each discerner contributes a `findings` list. The shape of each finding
record is per-discerner and documented below.

All `start` / `end` offsets throughout are half-open character offsets
into the normalized-text input (`{N}-in.txt`), UTF-8 code-unit counts
over bytes that are almost entirely ASCII in Epic clipboards.

---

## 1. Stanford — `StanfordAIMI/stanford-deidentifier-base`

### 1.1 Identity

| Attribute | Value |
|---|---|
| Model ID | `StanfordAIMI/stanford-deidentifier-base` |
| Base architecture | PubMedBERT (BERT-uncased, biomedical pretraining) with a linear token-classification head |
| Task | Token-level PHI classification |
| Training data | 999 Stanford chest X-ray / CT reports (Nov 2019 – Nov 2020) + 3,001 previously-labeled X-ray reports + 2,193 previously-labeled clinical notes; evaluated on i2b2 2006 + i2b2 2014 |
| Head shape | 8 logits per subword token → softmax → argmax |
| Tokenizer | WordPiece (PubMedBERT-uncased); max sequence 512, greedy chunking between sentences |
| Paper | Chambon, Wu, Steinkamp, Adleberg, Cook, Langlotz. *Automated deidentification of radiology reports combining transformer and "hide in plain sight" rule-based methods.* JAMIA 30(2), 2022. [PMC9846681](https://pmc.ncbi.nlm.nih.gov/articles/PMC9846681/) |
| Model card | [huggingface.co/StanfordAIMI/stanford-deidentifier-base](https://huggingface.co/StanfordAIMI/stanford-deidentifier-base) (accessed 2026-04-20) |
| Config source | [config.json id2label](https://huggingface.co/StanfordAIMI/stanford-deidentifier-base/blob/main/config.json) (accessed 2026-04-20) |

### 1.2 Container-side span fusion

The Stanford model emits one prediction per WordPiece subtoken. Our
wrapper (`Tools/apck/apcd/container/discerners/stanford.py`) post-processes:

1. Drop `(0, 0)` offset tokens (CLS / SEP / PAD).
2. Drop O-labeled tokens.
3. Fuse consecutive non-O tokens that share the same label (and are
   adjacent in source text) into a single span.
4. Span `confidence` = mean of the fused tokens' confidences.

This post-processing is why the finding list is flat (no BIO prefixes)
and why each record already represents a coalesced span rather than a
subword.

### 1.3 Per-field catalog

All finding records share the same shape — no `kind` discriminator.

| Field | Type | Semantics | Observed in fixtures |
|---|---|---|---|
| `text` | string | The character span `normalized_text[start:end]`, reproduced verbatim. | 2–44 chars per label; no newlines; 34 records contain internal spaces. |
| `start` | int | Half-open start offset into the normalized-text input. | Monotone non-decreasing within a finding list (container emits left-to-right by chunk). |
| `end` | int | Exclusive end offset. `text` length always equals `end − start`. | Confirmed: zero offset / length mismatches across 70 findings. |
| `label` | string | One of the 8 native labels (see §1.4). O is never emitted (our wrapper filters it). | 7 of 7 non-O labels observed. |
| `confidence` | float | Mean softmax probability across the fused subtokens for the predicted label. | Range `[0.6225, 1.0000]`. Concentrated above 0.95 (66/70). |

### 1.4 Value-space: the 8-label taxonomy

The model's head is 8-wide. Taken verbatim from the HuggingFace
`config.json` `id2label` mapping:

| id | label | Paper category | HIPAA Safe Harbor relation |
|---|---|---|---|
| 0 | `O` | (not-PHI) | n/a |
| 1 | `VENDOR` | Vendor and software names | **Above Safe Harbor** — institutional-preference addition |
| 2 | `DATE` | Dates | Yes (Safe Harbor §(C)) — all date elements smaller than year |
| 3 | `HCW` | Provider names | Yes (names of individuals) |
| 4 | `HOSPITAL` | Locations | Yes (geographic subdivisions smaller than state) — also catches institution names and addresses generically |
| 5 | `ID` | IDs | Yes (medical record numbers, account numbers, SSNs, device identifiers, etc.) |
| 6 | `PATIENT` | Patient names | Yes (names of individuals) |
| 7 | `PHONE` | Phone numbers | Yes |

The paper explicitly calls out that PROVIDER NAMES (our `HCW`) and
VENDOR AND SOFTWARE NAMES (our `VENDOR`) "exceed HIPAA Safe Harbor
requirements but reflect institutional preferences." AGE is **not** a
label in this model; it must be inferred from `DATE` evidence or from
Rust-side regex.

The taxonomy is **flat** — no B-/I-/E-/S-/O-prefix scheme. The model
places a single label per subtoken; our wrapper fuses adjacent identical
labels into spans.

### 1.5 Per-label observed behavior (fixture evidence)

| Label | Count | Conf. min/max/mean | Width min/max/mean | Notes |
|---|---|---|---|---|
| `PATIENT` | 12 | 0.972 / 0.999 / 0.996 | 10 / 20 / 14.8 | Full names, portal-ID fragments (`m.thornton47`, `thornton-m`), honorifics. |
| `HCW` | 12 | 0.996 / 1.000 / 0.999 | 9 / 22 / 15.4 | "Dr." / "MD" / "PA-C" credentialed names. Tightest confidence band of any label. |
| `DATE` | 15 | 0.937 / 1.000 / 0.993 | 4 / 10 / 8.4 | DOBs, visit dates, freestanding years (`2026`, `1961`), month-year stubs (`04/2026`). |
| `ID` | 11 | 0.983 / 1.000 / 0.998 | 4 / 18 / 10.2 | MRN, SSN, BCBS plan, device serials. Fragments on hyphenated IDs (see §1.6). |
| `PHONE` | 5 | 0.998 / 0.999 / 0.999 | 12 / 14 / 12.8 | Parenthesized and hyphenated formats. |
| `HOSPITAL` | 11 | 0.623 / 1.000 / 0.954 | 2 / 44 / 26.2 | Facility names, street addresses, bare street names. Weakest confidence floor. |
| `VENDOR` | 4 | 0.760 / 0.923 / 0.846 | 3 / 11 / 6.5 | `gmail`, `com`, `mychart`, `mainehealth` — email / URL fragments. Weakest mean. |

Calibration note: the paper's macro-F1 per category (Penn test set)
ranks categories similarly — VENDOR at 65.0 was by far the weakest,
HOSPITAL (labeled LOCATIONS in the paper) at 89.4 was the second-weakest,
all others ≥95.6. Our observed confidence band rank-orders the same way.

### 1.6 Utility assessment — what each Stanford finding can signal

- **`confidence ≥ 0.99`** — the high-signal band. Of 70 findings, 51 sit here.
  On the fixtures every confidence-≥0.99 finding is either unambiguously
  PHI or a fragment of PHI.
- **`confidence ∈ [0.95, 0.99)`** — mid-signal. 15 of 70 findings. In the
  fixtures these are mostly multi-word spans where one subtoken
  registered weaker (e.g., long addresses).
- **`confidence < 0.95`** — low-signal. 4 of 70. All are short-fragment
  VENDOR / HOSPITAL matches (`SS` at 0.623, `com` at 0.760).
- **`label = HOSPITAL` triple-duty.** Covers (a) real facilities like
  "Maine Medical Center", (b) street addresses, (c) bare street-name
  fragments. Any combining rule distinguishing these must triangulate
  with regex address shapes, Stanza `FAC`, Stanza `GPE`, or both —
  Stanford alone cannot.
- **`label = ID` fragmentation on hyphenated forms.** Example from
  fixture 10002: `ENC-2026-0051203` splits into `ENC-` (conf 0.98) and
  `-0051203` (conf 1.00), each a separate `ID` record. Any span with
  `text` ending or starting with `-` should be suspected a fragment and
  cross-checked against regex evidence.
- **`label = PATIENT` on email local-part / URL path.** Emails
  fragment: `m.thornton47@gmail.com` becomes three records — `PATIENT
  m.thornton47` + `VENDOR gmail` + `VENDOR com`. Portal URLs fragment
  similarly. Regex for email / URL shapes is the cleaner signal at the
  same span.
- **`label = VENDOR` as low-entropy strings.** The entire observed set
  is `{gmail, com, mychart, mainehealth}`. Whether these are PHI under
  Safe Harbor is an APCAS question; the signal exists either way.
- **Span character offsets are trustworthy.** Zero mismatches across 70
  findings between `end − start` and `len(text)`. Same-span overlap with
  Rust-side regex findings is a reliable combining primitive.

---

## 2. scispaCy — `en_core_sci_md`

### 2.1 Identity

| Attribute | Value |
|---|---|
| Model ID | `en_core_sci_md` |
| Size | ~360k vocabulary, 50k word vectors |
| Pipeline components | tokenizer, POS tagger, dependency parser, entity span detector ("mention detector"), lemmatizer |
| POS / parser training | GENIA 1.0 Treebank (converted to Universal Dependencies) + OntoNotes 5.0 |
| NER training | MedMentions (mention spans only — no fine-grained types) |
| Tokenizer | Custom rules layered on spaCy's English rule-based tokenizer, tuned for biomedical abbreviations and chemical/measurement strings |
| Paper | Neumann, King, Beltagy, Ammar. *ScispaCy: Fast and Robust Models for Biomedical Natural Language Processing.* BioNLP 2019. [aclanthology.org/W19-5034](https://aclanthology.org/W19-5034.pdf) |
| Model index | [allenai.github.io/scispacy](https://allenai.github.io/scispacy/) (accessed 2026-04-20) |
| Source | [github.com/allenai/scispacy](https://github.com/allenai/scispacy) (accessed 2026-04-20) |

### 2.2 Why NER emits a single generic `ENTITY` label

scispaCy distinguishes two model families:

- **Full-pipeline "sci" models** (`en_core_sci_sm/md/lg/scibert`) — include
  a *mention detector* trained on MedMentions, which labels spans only
  with a single generic type because MedMentions was built for entity
  linking (downstream UMLS match) rather than typed classification.
- **Specialized NER models** (`en_ner_craft_md`, `en_ner_jnlpba_md`,
  `en_ner_bc5cdr_md`, `en_ner_bionlp13cg_md`) — typed, per-corpus
  (chemicals, diseases, proteins, etc.).

Our container uses the full-pipeline `en_core_sci_md`, so every
`kind: entity` finding carries `label: "ENTITY"`. This is a design
consequence of MedMentions, not a defect.

### 2.3 Per-field catalog — `kind: "token"`

Emitted by iterating `nlp(text)` and skipping `tok.is_space` tokens
(see `spacy_scan.py:22`). Each record has 10 keys:

| Field | Type | Semantics | Observed in fixtures |
|---|---|---|---|
| `kind` | string constant | Literal `"token"`. | 1,950 token records across the two fixtures. |
| `text` | string | `tok.text` — the surface form. | Includes punctuation, abbreviations, medical units. |
| `start` | int | `tok.idx` — character offset of token's first char in the full document. | Monotone non-decreasing. |
| `end` | int | `tok.idx + len(tok.text)`. `end − start` always equals `len(text)` (0 mismatches observed). | |
| `pos` | string | Universal POS tag (`tok.pos_`). 17 possible values (see §2.5). | 16 distinct observed (INTJ absent). |
| `tag` | string | Fine-grained language-specific tag (`tok.tag_`). English models use an extended Penn Treebank set. | 33 distinct observed. |
| `morph` | string | `str(tok.morph)` — pipe-separated `Key=Value` pairs from the Universal Features inventory, empty string when no features apply. | 29 distinct values, 318 empties. |
| `lemma` | string | `tok.lemma_` — dictionary base form. | Lowercase for most content words; punctuation lemma matches surface. |
| `head` | int | `tok.head.i` — **absolute document-level token index** of the syntactic governor. Self-reference (`head == self_index`) marks a ROOT token. | Range `[0, 1092]` on fixture 10001 (total tokens 1,008); max can exceed token count because the index is over the pre-filter `Doc` (space tokens included). |
| `dep` | string | `tok.dep_` — UD v2 dependency relation to `head`. `ROOT` when the token is a sentence root. | 33 distinct observed. |

### 2.4 Per-field catalog — `kind: "entity"`

Emitted by iterating `doc.ents` (see `spacy_scan.py:37`). Each record
has 5 keys:

| Field | Type | Semantics | Observed |
|---|---|---|---|
| `kind` | string constant | Literal `"entity"`. | 573 entity records. |
| `text` | string | `ent.text` — span surface text. | |
| `start` | int | `ent.start_char` — absolute character offset. | |
| `end` | int | `ent.end_char`. | |
| `label` | string | `ent.label_`. Always `"ENTITY"` for this pipeline (see §2.2). | 573/573 = `ENTITY`. |

### 2.5 Value-space notes

#### `pos` — Universal POS (UPOS) v2

All 17 UPOS tags ([universaldependencies.org/u/pos](https://universaldependencies.org/u/pos/), accessed 2026-04-20):

- **Open class** — ADJ, ADV, INTJ, NOUN, PROPN, VERB
- **Closed class** — ADP, AUX, CCONJ, DET, NUM, PART, PRON, SCONJ
- **Other** — PUNCT, SYM, X

Observed in fixtures: 16 of 17 (INTJ absent — consistent with clinical
prose).

#### `tag` — Penn-Treebank-derived (extended)

33 distinct observed: `$ , -LRB- -RRB- . : CC CD DT HYPH IN JJ MD NN
NNP NNPS NNS POS PRP PRP$ RB RP SYM TO VB VBD VBG VBN VBP VBZ WP WRB
XX`. Extensions beyond strict PTB include `HYPH` (hyphen), `XX`
(unknown), and `$ SYM` as disjoint categories. Standard PTB reference
aside, spaCy's effective English inventory is documented in its tag
schema JSON shipped with the model.

#### `dep` — Universal Dependencies v2

All UD relations ([universaldependencies.org/u/dep](https://universaldependencies.org/u/dep/), accessed 2026-04-20), 33 observed:

Core arguments — `nsubj obj iobj csubj ccomp xcomp`
Non-core dependents — `obl vocative expl dislocated advcl advmod discourse aux cop mark`
Nominal dependents — `nmod appos nummod acl amod det clf case`
Coordination — `conj cc`
MWE — `fixed flat compound`
Loose — `list parataxis orphan goeswith reparandum`
Special — `root punct dep`

Subtype colon-suffixed forms observed in our fixtures: `acl:relcl`,
`compound:prt`, `nmod:npmod`, `nmod:poss`, `nmod:tmod`. Also the
variant `auxpass` / `nsubjpass` (UD v1 legacy names still emitted by
spaCy's GENIA-trained parser — compare Stanza's UD-v2 `aux:pass`,
`nsubj:pass`). This difference matters when cross-referencing spaCy
and Stanza dep labels.

Note: spaCy emits `ROOT` (all caps); Stanza emits `root` (lowercase).
Both mean the same thing but are not byte-equal.

#### `morph` — Universal Features

Pipe-separated `Key=Value` pairs, empty string when no features apply.
All 29 distinct values observed (top 15 by count):

| Value | Count |
|---|---|
| `Number=Sing` | 761 |
| `` (empty) | 318 |
| `NumType=Card` | 255 |
| `PunctType=Peri` | 128 |
| `Degree=Pos` | 113 |
| `PunctType=Comm` | 68 |
| `Number=Plur` | 56 |
| `PunctSide=Fin\|PunctType=Brck` | 35 |
| `PunctType=Dash` | 30 |
| `Aspect=Perf\|Tense=Past\|VerbForm=Part` | 30 |
| `PunctSide=Ini\|PunctType=Brck` | 29 |
| `ConjType=Cmp` | 22 |
| `Number=Sing\|Person=3\|Tense=Pres\|VerbForm=Fin` | 14 |
| `VerbForm=Inf` | 14 |
| `Aspect=Prog\|Tense=Pres\|VerbForm=Part` | 13 |

The inventory is open-ended — it can grow as the pipeline encounters
more complex English. Any Rust-side consumer should parse the string
at `|` then `=` boundaries rather than assume a fixed lookup set.

#### `head` — document-absolute token index

spaCy's `token.head` returns a token object; `tok.head.i` is its
absolute position in the spaCy `Doc`. A root token self-references
(`head == self_index`). On fixture 10001: 5 tokens self-reference (the
5 ROOT sentences). Head indices can exceed our filtered token count
because the `Doc` iterator includes space tokens that we drop — so an
apparent "head points past the end of findings" is expected behavior,
not corruption.

**Cross-library caveat.** Stanza's `head` uses CoNLL-U convention
(sentence-relative, 1-based, 0 = root). These two conventions are
incompatible byte-for-byte. Any combining primitive that walks
dependency trees needs per-library handling. See §4.1.

### 2.6 Observed behavior (fixture evidence)

- **Token count.** 1,008 (fixture 10001) + 942 (fixture 10002). Token
  findings account for ~77% of all spaCy finding records.
- **Entity count.** 283 (10001) + 290 (10002). All 573 carry the
  `ENTITY` label.
- **Empty `morph`.** 318 of 1,950 tokens (16.3%). These are mostly
  `PUNCT` / `SYM` / `X` where inflection does not apply.
- **`pos=PROPN` density.** 251 tokens = 12.9% — high, reflecting the
  clinical-prose domain (patient names, facility names, provider
  names, city names, drug names).
- **Token-level start/end integrity.** Zero mismatches between
  `end − start` and `len(text)`.
- **Root count vs sentence count.** 5 sentences in fixture 10001 per
  `head==self_index` self-references among the filtered findings.
  (Stanza reports 125 sentence roots on the same text — the two
  sentence segmenters disagree dramatically; see §4.2.)

### 2.7 Utility assessment

- **Syntactic features are the reason to keep spaCy.** The mention
  detector's single-label output means spaCy entities are weight-zero
  for PHI classification. But `pos`, `tag`, `morph`, and the dependency
  tree give combining homograph-disambiguation handles that neither
  Stanford nor Stanza expose at the token level in quite the same way.
- **`pos=PROPN` is the strongest single-token PHI prior.** 251 tokens
  in the fixtures; the overwhelming majority correspond to Stanford
  `PATIENT` / `HCW` / `HOSPITAL` spans. A token outside any Stanford
  span that nonetheless has `pos=PROPN` is a candidate "missed
  proper noun" to cross-check.
- **`tag=NNP` / `NNPS` redundant with `pos=PROPN`.** One of the two can
  carry the signal; keeping both costs JSON bytes but simplifies
  per-library lookup by the Rust consumer. Not load-bearing yet — will
  remain until combining has reason to prefer one.
- **`dep=compound` clusters are a name-boundary signal.** 388 tokens in
  the fixtures labeled `compound`. A contiguous run of `compound`-related
  tokens all with `pos=PROPN` is structural evidence for a multi-word
  name, even when the NER models miss the boundary.
- **`morph=Number=Sing` on a number-looking token is a weak signal it
  is being treated as a noun, not a digit.** Useful for disambiguating
  "2" in "age 2" (NUM, NumType=Card) from "2" in "H2" (X or NOUN
  depending on context).
- **Entities are net noise for PHI.** 573 generic `ENTITY` records
  carrying no type information duplicate token offsets and cost JSON
  bytes. The container could safely emit them under a suppression flag
  if JSON size becomes a concern; today they are kept for completeness.
- **`start` / `end` trustworthy.** No offset mismatches. Same-span
  cross-check with Stanford and Rust-regex evidence is a reliable
  combining primitive.

---

## 3. Stanza — English UD + OntoNotes NER

### 3.1 Identity

| Attribute | Value |
|---|---|
| Library | `stanza` 1.9.2 |
| Language | English (`lang="en"`) |
| Processors loaded | `tokenize, pos, lemma, depparse, ner` (entrypoint forces these five) |
| Tokenize / POS / lemma / depparse corpus | Universal Dependencies English EWT (default `package="default"` for the UD stack) |
| NER corpus | OntoNotes 5.0 (default English NER model when the package resolves the general pipeline) |
| NER encoding | **BIOES** (Begin, Inside, Outside, End, Single) |
| Data version | UD v2.12 feature inventory |
| Documentation | [stanfordnlp.github.io/stanza](https://stanfordnlp.github.io/stanza/) (accessed 2026-04-20) |
| NER docs | [stanza/ner_models.html](https://stanfordnlp.github.io/stanza/ner_models.html) (accessed 2026-04-20) |
| POS docs | [stanza/pos.html](https://stanfordnlp.github.io/stanza/pos.html) (accessed 2026-04-20) |
| Depparse docs | [stanza/depparse.html](https://stanfordnlp.github.io/stanza/depparse.html) (accessed 2026-04-20) |

### 3.2 Iteration model

Our wrapper (`stanza_scan.py`) iterates `doc.sentences`, then `sent.words`.
For each `Word` we consult its owning `Token` (`word.parent`) to recover
*character* offsets (word objects themselves carry only sentence-local
indices) and the token-level NER tag.

A consequence: in English multi-word-token (MWT) cases — specifically
possessive enclitics — two `Word` records share the *same* `parent` and
therefore the same character span. This is visible in our fixtures as
six records where `end − start ≠ len(text)`:

| Fixture | Token span chars | Word text | Word length |
|---|---|---|---|
| 10001 | 1837..1846 (9) | `patient` | 7 |
| 10001 | 1837..1846 (9) | `'s` | 2 |
| 10001 | 2032..2039 (7) | `Anand` | 5 |
| 10001 | 2032..2039 (7) | `'s` | 2 |
| 10002 | 855..864 (9) | `patient` | 7 |
| 10002 | 855..864 (9) | `'s` | 2 |

Combining must not assume Stanza `start`/`end`/`text` form a tight
invariant the way Stanford and spaCy do. A pair of Stanza tokens with
**identical start/end** is the MWT-sibling signature.

### 3.3 Per-field catalog — `kind: "token"`

| Field | Type | Semantics | Observed |
|---|---|---|---|
| `kind` | string constant | Literal `"token"`. | 1,953 token records. |
| `text` | string | `word.text` — the surface form of the syntactic word (after MWT expansion). | See §3.2 for MWT caveat. |
| `start` | int | `parent.start_char` — character offset of the **owning token** (not the word). | Monotone within sentence; identical for MWT siblings. |
| `end` | int | `parent.end_char` — same caveat. | |
| `upos` | string | Universal POS tag. 17 possible values. | 16 distinct (INTJ absent). |
| `xpos` | string | Language-specific POS — for English, Penn-Treebank-derived tags from EWT. | 34 distinct. |
| `feats` | string | Universal Features, pipe-separated `Key=Value`, empty string if none. | 31 distinct. |
| `lemma` | string | Dictionary base form. | Lowercase for most content words. |
| `head` | int | **Sentence-relative 1-based index**; `0` marks the root. Per CoNLL-U convention. | Range `[0, 49]` on fixture 10001 (max sentence length in words); 125 roots on that fixture. |
| `deprel` | string | UD v2 dependency relation. Subtype-colon-suffixed forms appear (`aux:pass`, `nsubj:pass`, `compound:prt`, etc.). | 38 distinct. |
| `ner` | string | Token-level BIOES-encoded NER tag (`O` or `{B,I,E,S}-LABEL`). Inherited from `parent.ner`; identical for MWT siblings. | 36 distinct composite values. |

### 3.4 Per-field catalog — `kind: "entity"`

Emitted by iterating `doc.entities` (see `stanza_scan.py:51`):

| Field | Type | Semantics | Observed |
|---|---|---|---|
| `kind` | string constant | Literal `"entity"`. | 254 entity records. |
| `text` | string | `ent.text` — surface text of the entity span. | |
| `start` | int | `ent.start_char`. | |
| `end` | int | `ent.end_char`. | |
| `label` | string | `ent.type` — one of the OntoNotes 18-label NER inventory. | 10 of 18 distinct observed. |

### 3.5 Value-space notes

#### `upos` — same UPOS v2 inventory as scispaCy

17 tags, 16 observed (INTJ absent). See §2.5 for the list.

#### `xpos` — Penn-Treebank-derived (EWT-trained)

34 distinct observed: `, -LRB- -RRB- . : ADD CC CD DT HYPH IN JJ LS
MD NFP NN NNP NNPS NNS POS PRP PRP$ RB RP SYM TO VB VBD VBG VBN VBP
VBZ WP WRB`. EWT-specific extensions include `ADD` (web address /
email), `LS` (list item marker), `NFP` (superfluous punctuation),
`HYPH` (hyphen).

**spaCy vs Stanza tag inventories differ**:

| Only spaCy `tag` | Only Stanza `xpos` | Both |
|---|---|---|
| `$`, `XX` | `ADD`, `LS`, `NFP` | the other 31 |

Do not assume spaCy `tag` and Stanza `xpos` are byte-equal even when
they agree in meaning.

#### `feats` — Universal Features (EWT-trained)

31 distinct observed (top 15 by count):

| Value | Count |
|---|---|
| `Number=Sing` | 756 |
| `` (empty) | 675 |
| `NumForm=Digit\|NumType=Card` | 172 |
| `Degree=Pos` | 100 |
| `Number=Plur` | 75 |
| `Tense=Past\|VerbForm=Part` | 32 |
| `Mood=Ind\|Number=Sing\|Person=3\|Tense=Pres\|VerbForm=Fin` | 20 |
| `Mood=Imp\|VerbForm=Fin` | 20 |
| `NumForm=Digit\|NumType=Frac` | 17 |
| `VerbForm=Ger` | 15 |
| `Definite=Def\|PronType=Art` | 13 |
| `Mood=Ind\|Number=Sing\|Person=3\|Tense=Past\|VerbForm=Fin` | 13 |
| `Case=Gen\|Gender=Fem\|Number=Sing\|Person=3\|Poss=Yes\|PronType=Prs` | 6 |
| `Definite=Ind\|PronType=Art` | 5 |
| `NumForm=Word\|NumType=Card` | 5 |

**`NumForm=Digit` is load-bearing.** Unlike spaCy, Stanza EWT splits
cardinal numerals into `NumForm=Digit` vs `NumForm=Word`. Every
structured identifier candidate (SSN fragments, phone fragments, MRN
digits, dates-as-digits) carries `NumForm=Digit|NumType=Card`. This is
a token-level "is this a digit sequence" signal that is cheaper than
regex.

Note the empty-count (675) is much higher than spaCy's (318). Part of
the difference is MWT siblings (6 records) and part is EWT's slightly
more conservative feature annotation for non-inflected word classes.

#### `deprel` — UD v2 (EWT inventory)

38 distinct observed, including: `acl acl:relcl advcl advmod amod
appos aux aux:pass case cc ccomp compound compound:prt conj cop dep
det fixed flat list mark nmod nmod:npmod nmod:poss nmod:tmod nsubj
nsubj:pass nummod obj obl obl:agent obl:npmod obl:tmod parataxis
punct root vocative xcomp`.

Differences from spaCy (our observed sets):

| spaCy only | Stanza only | Meaning |
|---|---|---|
| `auxpass`, `nsubjpass`, `dobj`, `prep`, `intj`, `neg` | `aux:pass`, `nsubj:pass`, `obj`, `flat`, `list`, `obl`, `obl:agent`, `obl:npmod`, `obl:tmod`, `fixed`, `vocative` | spaCy's English parser emits UD-v1-style relation names; Stanza EWT emits UD-v2-style relation names. |

For combining, any rule that keys on `dep` / `deprel` must either
normalize across the two libraries or key on UPOS/XPOS instead. A
cheap normalization: `replace("auxpass","aux:pass"); replace("nsubjpass","nsubj:pass"); replace("dobj","obj"); replace("prep","case")`.

#### `ner` — BIOES-encoded OntoNotes

Token-level NER tag has shape `O` | `B-LABEL` | `I-LABEL` | `E-LABEL`
| `S-LABEL`. **BIOES** rather than plain BIO:

- `B-` begin a multi-token entity
- `I-` interior of a multi-token entity
- `E-` end of a multi-token entity
- `S-` single-token entity (no B/E pair needed)
- `O` not in any entity

Observed prefix counts on fixture set: `O` 1,534, `B` 100, `I` 64,
`E` 100, `S` 155. `B`-count equals `E`-count (as invariant demands) —
every multi-token entity has both endpoints. `S`-count (155) is
larger than `B`-count (100), so most entities in this clinical text
are single-token.

Distinct composite values observed (36 total): `O` plus B/I/E/S
variants of `CARDINAL, DATE, FAC, GPE, ORG, PERCENT, PERSON, PRODUCT,
QUANTITY, TIME`. Zero hits for `I-GPE`, `B-TIME`, `I-CARDINAL` etc. —
expected for short spans.

#### Entity `label` — OntoNotes 5.0, 18 possible

Full OntoNotes inventory: `PERSON NORP FAC ORG GPE LOC PRODUCT EVENT
WORK_OF_ART LAW LANGUAGE DATE TIME PERCENT MONEY QUANTITY ORDINAL
CARDINAL`.

Observed in our fixtures: `PERSON CARDINAL FAC GPE ORG DATE QUANTITY
TIME PRODUCT PERCENT` (10 of 18). Absent from fixtures but possible
in other Epic text: `NORP` (ethnicities/nationalities), `LOC`
(non-GPE locations — bodies of water, landforms), `EVENT`,
`WORK_OF_ART`, `LAW`, `LANGUAGE`, `MONEY`, `ORDINAL`. Any combining
rule that whitelists or blacklists labels should enumerate the full
18, not only the observed 10.

#### `head` — CoNLL-U sentence-relative

**Not compatible with spaCy's `head`**. Range in fixtures was `[0,
49]` (max sentence length in words). Value `0` = "this word is its
sentence's root." To dereference into an absolute position: walk
`doc.sentences`, track each sentence's first-word absolute index, add
`head − 1`. In `stanza_scan.py` we flatten to `findings` without
preserving sentence boundaries, so reconstructing absolute head
indices requires a Rust-side pass that segments by `deprel == "root"`
(sentence roots) and relinks. If combining does not care about
dependency-tree walking, the raw value is still useful as a local
ordinal.

### 3.6 Observed behavior (fixture evidence)

- **Sentence count discrepancy vs spaCy.** 125 `head=0` records on
  fixture 10001 (Stanza sentence count) vs 5 `head==self_index`
  records from spaCy on the same text. Stanza's EWT tokenizer /
  sentence segmenter is aggressive — it treats many line-break and
  bullet-formatted clinical fragments as separate sentences where
  spaCy's scientific tokenizer treats the document more holistically.
- **Entity / token ratio.** 254 entities across 1,953 tokens (13%) —
  significantly lower than spaCy's 29.4%, because Stanza's OntoNotes
  model is pickier about what constitutes an entity, while spaCy's
  MedMentions mention detector is permissive.
- **PERSON corroboration.** 41 `PERSON` entities on the two fixtures
  cover essentially the same spans as Stanford's `PATIENT` + `HCW`.
  This is the intended redundancy.
- **FAC vs GPE vs ORG triangulation.** `FAC` (10) = addresses + "Maine
  Medical Center"; `GPE` (12) = cities + state abbreviations; `ORG`
  (29) = Maine Medical Center + medical abbreviations treated as
  organizations (NSTEMI, EMS, ED, CBC). Stanford conflates all three
  into `HOSPITAL` — Stanza gives combining three distinct signals.
- **CARDINAL leakage onto PHI.** 100 `S-CARDINAL` tokens include SSN
  fragments (`471-83-2956` as CARDINAL) and phone-number fragments
  (`207` / `555-0143`). Structured-ID regex remains the clean signal
  at those spans.
- **TIME leakage.** `207-555-0488` (a phone) and `18.2 seconds` (a
  measurement) both tagged `TIME`. Stanza's TIME label is under-
  disciplined on hyphenated numbers.
- **Offset integrity elsewhere.** Only 6 records show
  `end − start ≠ len(text)`, all accounted for by MWT expansion. No
  other offset corruption.

### 3.7 Utility assessment

- **`entity.label = PERSON`** — high-signal PHI corroborator; near-
  1:1 overlap with Stanford PATIENT / HCW. A PERSON span outside any
  Stanford span flags a potential miss.
- **`entity.label = GPE` / `FAC`** — the disambiguation Stanford lacks.
  Combining can distinguish "home address" from "hospital name" from
  "city mention" using GPE + FAC + regex address shape together.
- **`entity.label = ORG`** — **conservative weight**. Real organizations
  (Maine Medical Center, Blue Cross) co-occur with medical-
  abbreviation false positives (NSTEMI, ED, EMS, CBC). Any combining
  rule on ORG should cross-check against a medical-abbreviation
  whitelist before acting.
- **`entity.label = CARDINAL` / `TIME` / `PRODUCT`** — **suppressive
  weight**. These leak onto structured identifiers and drug names
  that regex + Stanford already cover better. Use them as tie-
  breakers, not primary signals.
- **Token-level `ner`** — redundant with `entity` findings except in
  BIOES precision: the token tag distinguishes endpoints, which
  entity findings do not. If combining ever needs "is this token the
  first subtoken of an entity?" the `B-` / `S-` prefix answers cheaply.
- **`feats` — `NumForm=Digit`** — load-bearing for "is this surely a
  digit sequence, not a word-form?". Cheaper than regex for the
  "contains-only-digits" check, and distinguishable from `NumForm=Word`
  ("two") which a regex `\d+` would miss.
- **`upos=PROPN` density** — comparable to spaCy's signal (257 in
  Stanza vs 251 in spaCy) at effectively the same tokens. Either
  library's `PROPN` tag is a usable proper-noun prior; picking one
  for combining keeps JSON size down.
- **`deprel`** — usable, but normalize across UD v1 / v2 naming (see
  §3.5 deprel table). `deprel=flat` and `deprel=compound` on
  `upos=PROPN` sequences are strong name-boundary signals.

---

## 4. Cross-library semantic differences worth flagging

### 4.1 `head` is not the same field in spaCy and Stanza

| Library | Indexing | Zero means | Range |
|---|---|---|---|
| spaCy | Absolute document token index (`tok.head.i`) | First token in document is its own ROOT | `[0, doc_token_count_inclusive_of_spaces]` |
| Stanza | Sentence-relative, 1-based (CoNLL-U convention) | Current word is the sentence root | `[0, max_sentence_length_in_words]` |

Any Rust-side tree walk must branch on discerner. The empirical tell
is that Stanza's `head` is bounded by sentence length (~50) while
spaCy's is bounded by document length (~1000+).

### 4.2 Sentence segmentation disagrees

spaCy's scientific tokenizer and Stanza's EWT tokenizer produce
dramatically different sentence counts on the same normalized Epic
text (5 vs 125 on fixture 10001). Neither is "wrong"; each reflects
its training corpus's conventions. Combining rules keyed on "is this
token in the same sentence as X" will give different answers per
library.

### 4.3 UD dependency relation names differ in version

spaCy emits UD v1 names (`auxpass`, `nsubjpass`, `dobj`, `prep`);
Stanza EWT emits UD v2 names (`aux:pass`, `nsubj:pass`, `obj`,
`case`). Bytes do not match even when meaning does. Normalize before
comparing.

### 4.4 Penn Treebank tag inventories differ

spaCy has `$` and `XX`; Stanza has `ADD`, `LS`, `NFP`. No combining
rule should assume "spaCy `tag` and Stanza `xpos` are interchangeable."

### 4.5 ROOT capitalization

spaCy emits `dep="ROOT"`; Stanza emits `deprel="root"`. Rust-side
combining should use case-insensitive comparison or normalize to one
form.

### 4.6 Entity label conventions differ

| Library | Label inventory | Observed distinct labels |
|---|---|---|
| Stanford | 8 flat native labels | 7 (no O in output) |
| scispaCy | Single `ENTITY` label | 1 |
| Stanza | OntoNotes 18 (via BIOES) | 10 |

scispaCy entities cannot be filtered by type; Stanza entities can.
Stanford has its own vocabulary. Treat the three as three independent
label spaces — do not attempt a unified label enum across them.

---

## 5. Consolidated field index

Quick-lookup table. If you are staring at a record in `epic_*.json`
and want to know what a field means, use this as the jump point.

| Discerner | Shape | Field | Jump to |
|---|---|---|---|
| stanford | (flat) | `text` | §1.3 |
| stanford | (flat) | `start` / `end` | §1.3 |
| stanford | (flat) | `label` | §1.4 |
| stanford | (flat) | `confidence` | §1.3, §1.5 |
| spacy | token | `text`, `start`, `end` | §2.3 |
| spacy | token | `pos` | §2.3, §2.5 |
| spacy | token | `tag` | §2.3, §2.5 |
| spacy | token | `morph` | §2.3, §2.5 |
| spacy | token | `lemma` | §2.3 |
| spacy | token | `head` | §2.3, §2.5, §4.1 |
| spacy | token | `dep` | §2.3, §2.5 |
| spacy | entity | `label` | §2.2, §2.4, §4.6 |
| stanza | token | `text`, `start`, `end` | §3.2, §3.3 |
| stanza | token | `upos` | §3.3, §3.5 |
| stanza | token | `xpos` | §3.3, §3.5, §4.4 |
| stanza | token | `feats` | §3.3, §3.5 |
| stanza | token | `lemma` | §3.3 |
| stanza | token | `head` | §3.3, §3.5, §4.1 |
| stanza | token | `deprel` | §3.3, §3.5, §4.3 |
| stanza | token | `ner` | §3.3, §3.5 |
| stanza | entity | `label` | §3.4, §3.5, §4.6 |

---

## 6. Exit criteria checklist

- [x] Every field present in `epic_initial.json` and `epic_geriatric.json`
  has a reference entry (see §1.3, §2.3, §2.4, §3.3, §3.4 and §5).
- [x] Each library's authoritative documentation is cited with URL and
  access date (§1.1, §2.1, §3.1, and cross-refs in §2.5, §3.5).
- [x] Cross-library semantic differences (which will bite combining)
  are called out as a dedicated section (§4).
- [x] Memo locks to pinned versions (§"Version pinning").
- [x] Out-of-scope items not touched: no combining rules written, no
  APCS0 quoin minting, no Python changes.
