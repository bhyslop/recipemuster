# Choosing Quoins Well: An LLM Introspective Report on Prefix-Identifier Minting

## Status

Discovery draft v0.1 — Introspective report (2026-06-10)

## Origin

This memo records introspections offered by Claude (Fable 5) during a review of MCM,
AXLA, and their consuming specifications (BUS0, RBS0, JJS0), in conversation with the
editor. The editor's stated goal: choose minting conventions that let a model hold
more of the system in context with fewer symbols. This memo is the model's attempt to
report, as honestly as it can, what prefix-discriminated identifiers (quoins) actually
do to its processing — and what minting choices follow.

Sibling to `memo-20260110-acronym-selection-study.md` (the empirical minting study);
where that memo reasons from observed collisions, this one reasons from the inside.

## Epistemic Stance

Everything below is hypothesis, not measurement. An LLM's reports about its own
processing are plausible reconstructions consistent with its architecture and its
observed behavior — they are not privileged access. Where a claim is backed by
behavior observable in the originating conversation, the specimen is named. Where it
is architectural reasoning, it is marked as such. The Diptych memo's phrase "direct
introspection" deserves the same caveat retroactively.

Confidence ordering of the four core claims, highest first: interference (§4),
certain coreference (§2), front-loaded learning (§3), cross-kit transfer (§5).

## 1. The Frame: Ambiguity Count, Not Token Count

The intuitive model of context economy — fewer tokens, more room — is not where
quoins pay. The expensive thing about holding a large specification in context is the
number of *unresolved bindings* the reader must keep alive. Plain prose says "the
record," "that field," "the heat's entry," and each such phrase forces maintenance of
a candidate-referent cloud that collapses only when later context disambiguates — or
never collapses, and silently mis-binds.

A quoin collapses the cloud at the point of reading. `jjdhm_silks` is exactly one
thing; every occurrence is certainly the same thing; attention can bind all
occurrences together without spending anything on coreference hypotheses. The word
`silks` alone would collide (Heat member and Tack member both carry one); the prefix
settles it immediately.

**Claim: a quoin converts probabilistic coreference into certain coreference, and
certainty — not brevity — is what frees capacity.** This is the deep win of the
mapping-as-spine design, and it survives any change of surface syntax (AsciiDoc,
Diptych recto/verso, guillemet lectio).

## 2. How Prefixes Are Actually Read

### Tokenization honesty

A prefix like `jjdhm_` is not one token. It is two or three odd subword chunks, and
which chunks is tokenizer-dependent and unintuitive. The Diptych memo's token
arithmetic is directionally right about ceremony but should not be extended to assume
prefixes are atomic.

What matters more than the count: **the same string always tokenizes the same way**.
A prefix family is a stable composite shape. The first few encounters are spelled out
against the legend; after that the shape is recognized, not decoded. The cost is
front-loaded and amortizes across occurrences.

### Two consequences for minting

- **Legend reachability.** A prefix is only free when its legend is in context.
  Partial reads of large documents — and Diptych's planned section-level
  presentation — can strand body text away from its category declarations. Whatever
  the format becomes, the legend must travel with the slice. (Lectio compilation
  should treat the relevant legend rows as mandatory cargo.)
- **Zipf-shaped brevity.** Quoins used constantly deserve the shortest, most
  distinctive names, because their learning cost amortizes across hundreds of
  bindings. Rare quoins can afford long descriptive names; they will be spelled out
  on every encounter anyway, so their length should be spent on self-description.

## 3. Interference: Distinctiveness Beats Systematicity

This is the highest-confidence claim, and the one with teeth.

The mechanisms that make an LLM good at repeated-pattern recall key on string
*shape*. Reading proceeds as sequence early and as shape once familiar — and the
transition happens around one chunk (roughly 3–5 characters). Two long identifiers
sharing a long common head with a single discriminating letter in the interior are
nearly the same retrieval key. That is not merely "hard to read" — it is a setup for
genuine mis-binding, where attention retrieves the wrong family's context and the
model confidently continues down the wrong branch.

Specimens from the originating review:

- `axhempt_typed_parameter` vs `axhopt_typed_parameter` (AXLA hierarchy markers):
  shared head `axh`, shared tail `pt_typed_parameter`, discriminator one interior
  letter. The model flagged these as requiring deliberate spelling-out on every
  encounter.
- JJS0's historical `rd` collision (three quoins, three meanings — recorded in the
  acronym-selection study as Pattern J's founding failure): the same failure mode at
  length two.

The taxonomic letter-chain (`h`+`e`+`m`+`p`+`t` = hierarchy→entity→method→
parameter→typed) optimizes for the legend-writer: every edge of the tree is encoded.
But the reader's machinery wants **maximal early divergence between things that mean
differently**. If two families differ in meaning, they should differ in their first
chunk, not their fourth letter.

**Corollary — don't re-encode positional context.** Hierarchy markers appear nested
under their parent marker; the entity-vs-operation branch is already carried by
document position. Re-encoding ancestry in the child's name recreates information
that position already provides (the naming analogue of ACG's allocation discipline:
reference the home, don't recreate). Deep taxa should drop inherited letters and
keep only the local discriminator.

**Corollary — the ceiling is chunkability, not count.** A 3-letter ceiling and a
5-letter ceiling are both proxies. The honest constraints are: (a) a prefix should
parse as at most ~2 chunks (in practice ≤4–5 characters after the lexicon root), and
(b) same-length prefixes within one document should differ by more than a single
interior letter. Both are mechanically checkable — see §7.

## 4. Transfer: Cross-Kit Rhyme Is the Cheapest Multiplier

Within each kit the sub-letters are monosemous (JJS0's discipline), but the schemes
only partially rhyme *across* kits: JJS0 speaks `r` record / `m` member / `e` enum;
RBS0's older strata speak `rbtga_`/`rbtgo_`; BUS0 has its own weave.

Observed behavior: where kit letters rhyme with AXLA's category letters (`r` ~
`axr_`, `t` ~ `axt_`, `o` ~ `axo_`), a never-before-seen kit's mapping section reads
nearly cold — JJS0's record/member/enum families were instantly legible because the
convention transferred from AXLA.

**Recommendation: promote a small repo-global sub-letter alphabet — the AXLA
category letters as the canonical rhyme — so that every new kit's vocabulary arrives
pre-learned.** Within-kit monosemy protects one document; repo-global monosemy makes
the whole system's vocabulary one learning event instead of N.

## 5. Word-Part Craft

- **Real-word stems after the underscore.** `_silks`, `_ensconce`, `_paddock`
  tokenize as real words and carry meaning intrinsically. Abbreviated stems (`_slk`)
  save characters and cost recognition — the wrong trade everywhere except the very
  highest-frequency quoins.
- **The liturgical/equestrian vocabulary is genuinely good for the model** — this
  deserves saying because it could look like whimsy. Rare-but-real words (ensconce,
  reliquary, coronet, lectio) are simultaneously distinctive (low collision with
  ambient technical prose, strong retrieval keys) and meaningful (the metaphor does
  real semantic work). Common technical words (`manager`, `handler`, `info`) are the
  opposite on both axes.
- **Attribute-name/display-text divergence is a carried cost.** Each divergence
  (`jjdcm_basis` → "commit") is one extra translation pair held in context.
  Sometimes it is load-bearing — the quoin/sprue distinction between conceptual name
  and wire name exists precisely for this. Where it is not load-bearing, convergence
  is free capacity.

## 6. Distilled Minting Guidance

Mint for:

1. **Certain binding** — global uniqueness, one string per concept, terminal
   exclusivity (existing discipline; it is the foundation and it is right).
2. **Early divergence** — different meanings diverge in the first chunk; interior
   single-letter discrimination between long shared heads is forbidden.
3. **Transferable convention** — one repo-global sub-letter alphabet, anchored to
   AXLA's category letters; within-domain monosemy becomes cross-domain monosemy.
4. **Zipf-shaped brevity** — short and sharp for the frequent, long and
   self-describing for the rare.
5. **Real words in the word-part** — distinctive rare words over abbreviations;
   spend depth in the word, not in the prefix letters.
6. **Position over re-encoding** — nested/hierarchical markers carry only their
   local discriminator; ancestry lives in document structure.

## 7. Mechanical Rules for a Future Validator

Each guidance point above has a checkable form; recorded here so the eventual VOS
flowering can pick them up:

- **Legend coverage**: every category prefix used in a mapping section appears in
  the category-declaration legend (the JJS0 `jjsz_`/`jjezs_` drift class).
- **Anchor↔attribute bijection**: every attribute reference resolves to an anchor
  and vice versa (the MCM `mcm_mapping_scrub` dangling-reference class).
- **Interior edit distance**: no two same-length prefixes in one document differ by
  exactly one interior character (the `axhempt_`/`axhopt_` and `rd` classes).
- **Chunk ceiling**: prefix body after the project root ≤ 4–5 characters; longer
  chains must justify themselves or move discrimination into the word-part.
- **Legend cargo** (Diptych-era): any compiled partial view (lectio, section slice)
  includes the legend rows for every prefix family present in the slice.

## 8. Relation to Diptych and Reminting

Nothing here argues for renaming anything now. The Diptych direction — identifier-is-
the-reference, no substitution engine — makes a future remint a mechanical rewrite,
and the editor's intended repo-global reminting tooling makes accumulated misminting
(e.g., the `jjezs_` oddity, the AXLA hierarchy chains) cheap to sweep later. The
right moment to apply this memo is at that tooling's design, and at each fresh mint
between now and then.

The one principle that should not wait: **new mints can follow §6 immediately**, at
zero migration cost — the guidance constrains births, not the living.

---

*Written by Claude Fable 5 at the editor's request, first session of that model in
this repository. The memo attempts to honor the house rule that introspective
reports be offered as observable weather, not certified fact: every claim above is
falsifiable in principle by behavioral testing, and the confidence ordering in the
Epistemic Stance section is part of the report.*
