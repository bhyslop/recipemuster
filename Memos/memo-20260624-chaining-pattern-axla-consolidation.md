# Memo — The chaining pattern: duplicated discipline-prose, and a consolidation to critique

Date: 2026-06-24
Target heat: ₣Bb (rbk-10-mvp-FABLE-acg-axla-scrub)
Audience: the future LLM (Fable) that mounts the consolidation pace, plus the human editor who reviews.

This memo describes a recurring spec-hygiene problem and offers ONE proposed
consolidation model. The model is a starting point to be critically reviewed and
improved, not a mandate — see "Mandate" at the end. Provenance only: durable
facts live in the specs and code; this is the reasoning trail and the issue
description.

## The problem — one discipline, restated in ~7 places

RB's "chaining" verbs hand a value forward between tabtargets, and the *discipline*
governing that hand-off is written as near-verbatim prose in many subdocs rather
than defined once and cited. The heaviest duplication is the **transient-consumer**
paragraph — restated almost word-for-word across RBSAP (plumb), RBSAS (summon),
RBSIR (rekon), RBSLA (augur): "resolved express-or-chain for a transient X only —
writes no durable config — broken chain dies loud as an ordinary fatal,
categorically lower severity than the durable-leak links … must never be extended
to write config without joining that durable-leak surface and its depth-1 no-relay
plus named-band-reject discipline." The **chain-head** prose (RBSAC) and the
**durable-leak link** prose (RBSDF feoff, RBSDY yoke; anoint) repeat the same
nature from the other two roles.

Discovery recipe (sites drift; grep, don't trust a list):

    grep -rniE "chain(ing)?[- ]?(fact|head|link)|durable[- ]?leak|express-or-chain|no-relay|named-band-reject" Tools/rbk/vov_veiled/*.adoc

This shape **recurred several times** — which is the point: the duplication is not
a one-off, and a robust consolidation model here is likely reusable for the next
duplicated cross-subdoc discipline.

## The pattern's full nature — every facet the consolidation must carry

The discipline is more than "a value passes forward." The complete nature, the
parts that must survive consolidation:

1. **Fact transport / dispatch-output-directory behavior.** A head writes its
   output as a single atomic value in a well-known named file in the dispatch
   output directory; the consumer reads it verbatim; the constant name is the
   producer↔consumer contract. (AXLA already states exactly this as `axd_fact`.)
   The facts' lifecycle in that directory — when written, overwritten, and read
   across tabtargets in one operator session — is part of the pattern.
2. **Roles.** *Chain head* (writes a fact, reads none, builds only from committed
   config); *durable-leak link* (reads one chained value, writes one durable
   config field, terminates the chain); *transient consumer* (reads for a
   transient action, writes no durable config).
3. **Resolution invariant.** Express-or-chain (an express arg wins, else the prior
   build's fact), **depth-1**, **no-relay** (read terminally, never forwarded — a
   relayed value would leak a stale capture into a later unrelated election).
4. **Git commit boundary — integral, not incidental.** The durable-leak link
   writes config the *operator* commits; the tool never commits. Heads gate on a
   clean tree (build only from committed config); links deliberately do NOT (they
   author the very change the operator is about to commit). The commit is where
   the chain's durable effect is sealed AND reviewed — the safety is the triad
   *operator-trust + loud-on-typecheck output + the commit-review gate*, with the
   clean-tree gate only an ergonomic backstop. This boundary must be documented as
   part of the nature.
5. **On-error behavior.** A broken chain dies loud — transient consumers as an
   ordinary fatal (lower severity), durable-leak links via a **named band-reject**
   (never bare nonzero). What happens to the fact/output-directory state on a
   failed or partial run (stale-fact hazard, atomicity of the write) is part of
   the pattern and should be pinned, not left implicit.

## Provenance — where the pieces already live

- **BUS0** owns the dispatch plumbing: the tabtarget→tabtarget fact-file hand-off
  mechanism and the output directory. The transport *substrate* is BUS0/BUK.
- **AXLA already carries the transport's nature**, which is why the continuity is
  through MCM/AXLA, not BUS0: `axd_fact` (operation output as a named fact file,
  constant-name contract), `axd_file_bus` (the richer file-wire protocol),
  `axpof_fact` (product-of-operation fact), and the `axvc_` rivet-kind voicing
  family (`axvc_dictum` invariant, `axvc_camber` deviation, `axvc_spoor` foreign
  signature, `axvc_membrane` Palisade membrane).
- **RBS0** owns the RB instantiation (which verbs play which role, over which facts
  — hallmark, touchmark).

## Proposed model (ONE recommendation — critique and improve it)

The repeated prose decomposes into four reusable claims, each with a natural home:

| Recurring claim | Proposed home |
|---|---|
| Head **writes a fact**, reads none | voice `axpof_fact` on the heads (AXLA exists) |
| **Transient consumer**: express-or-chain, no durable write, die loud | a **new** constraint motif |
| **Durable-leak link**: read-one, write-one-durable, terminate | a **new** constraint motif |
| **No-relay / depth-1 / named-band-reject / commit-boundary** | `mcm_rivet`(s) voiced `axvc_dictum`, cited |

The dedup move: define the nature once (motif definitions), and replace each
subdoc's paragraph with a one-line voicing annotation (`//primary axd_… …`); the
subdoc keeps only its verb-specific detail (which folio arg, which fact). Seven
paragraphs collapse to seven one-liners plus a few definitions — ACG "reference
the home, don't recreate," MCM rivet, and AXLA motif, applied as designed.

**Home (AXLA vs RBS0) is deliberately left open.** The fact-transport is plainly
AXLA (it is already there). The role/discipline motifs are a genuine
cross-model-vs-RB-only judgment, and AXLA's own rule is to avoid bloat — so the
test is whether a second concept model (JJK foray hand-off? a BUK folio chain?)
shares the read-once/no-relay/leak-surface shape. Recommend the home per-piece.

**Hazard — do NOT reuse `axd_transient`.** In AXLA it already means *"procedure
executes and completes (vs long-running)"* — that is why feoff/yoke correctly
voice it despite writing durable config. The chaining sense of "transient"
(writes-no-durable-config) is a DIFFERENT meaning; reusing the token would be a
monosemy collision (the JJS0 `rd` failure). The new motif needs its own word —
mint into the `axvc_` asterism's register (ecclesiastical/diplomatic), grep gate
clean.

## Coupled pace — a fourth durable-leak member is incoming (₢BiAAa, ₣Bi)

₣Bi pace **₢BiAAa** (nameplate-hallmark-drive-chain-link) is settled to add a
**fourth durable-leak link**: the drive that writes a freshly-built hallmark into a
nameplate's `RBRN_*_HALLMARK`. Today that durable write is three ad-hoc copies —
bash `zrbob_drive_hallmark` fused into the local kludge wrappers, a Rust
reimplementation (`rbtdro_drive_hallmark`) for onboarding, and a handbook
hand-edit instruction — all outside the discipline. ₢BiAAa brings it onto the
durable-leak surface with the same no-relay / named-band-reject / commit-boundary
treatment feoff/anoint/yoke carry.

Consequence for this consolidation: the durable-leak link is a **role whose
membership is about to grow from three to four**, and its members span two regime
families — feoff/anoint/yoke write `RBRV_*` (vessel); the new drive writes
`RBRN_*` (nameplate). Consolidate the role's *nature*, not a fixed roster of three
`RBRV_*`-writing verbs: any enumeration or "only three verbs / writes one durable
config field" wording must generalize so the fourth member slots in without
re-opening the motif. The two paces are order-independent; whichever lands second
reconciles against the other.

## Mandate (for the pace)

Treat the model above as a proposal that recurred enough to be worth getting
general. Make your best recommendation — critically review and improve it, decide
the homes, mint the motifs, and carry every facet of the nature (including the
commit boundary, the output-directory behavior, and the on-error behavior). Then
work with the human editor, who reviews and interacts to find the right level
before it lands. Describe and recommend; do not assume the proposal is fixed.
