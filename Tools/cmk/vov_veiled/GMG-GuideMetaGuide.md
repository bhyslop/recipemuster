<!--
Copyright 2026 Scale Invariant, Inc.
SPDX-License-Identifier: Apache-2.0
See LICENSE for terms.
-->

# Guide Meta-Guide (GMG) — How a Guide Is Written

## Purpose

GMG is the guide for writing guides. The project's guides — BCG, RCG, WSG, HCG,
CBG, PCG, ACG, JDG — recur in shape and framing; GMG homes that shared form once
so a guide can **cite the convention rather than re-derive it**, and so a new
guide starts from a known skeleton instead of a blank page. It is ACG's
"statements → definition sites" turned on the guide family itself: the
duplicated framing gets a home.

v1 posture: **bless and name what the family already does.** GMG mandates
nothing new and re-explains nothing already homed elsewhere — it points. The
living exemplars of every convention below are CBG, PCG, JDG (modern form) and
ACG (the catalog variant); read them, don't re-read their patterns here.

## What a guide is

A guide is a **veiled `.md` document that codifies durable authoring discipline**
— how *we* write a class of artifact. Distinct from its two neighbors:

- A **spec** (`.adoc`, MCM) says what the *system* does. A guide says how the
  *author* should work. One file answers to exactly one **guide** (ACG Related
  Guides); a guide and a spec sit on orthogonal axes, not in competition.
- A **handbook** (HCG's domain) is an operator-facing *procedure*. A guide is
  author-facing *discipline*.

Family invariants: named `{ACRONYM}-{Words}.md` with the acronym ending in `G`
and registered in the owning kit's `claude-*.md` acronym map; prose-first
(**codification, not abstraction** — fair-faced to a first-time reader); and
downstream of BCG's headwater philosophy (crash-fast, no silent failures,
load-bearing complexity), which guides **cite, never restate**.

## The canonical skeleton — where a first draft starts

Under the standard license-header + `# {Name} ({ACRONYM}) — {tagline}` opening, a
first draft needs four sections:

1. **`## Purpose`** — what it codifies and which guides it is sibling to.
2. **`## Core Philosophy`** — the load-bearing stances for this subject.
3. **`## Authored Disciplines`** — the internalized patterns; the heart. Add a
   **`## Cited Rules`** section only when something will point at a numbered rule.
4. **`## Related Guides`** — siblings + one-file-one-guide; close with an
   **Acronym Registry** when the guide mints terms.

Heavier scaffolding — a "How this document is organized" genre declaration, a
fault-domain environment table — is earned later, not first-draft furniture.

## The shared conventions — voice the ones that fit

Each is a reusable framing motif. A guide adopts those its subject warrants and
ignores the rest; HCG and RCG, for instance, voice few of them.

- **Foreign-environment sibling** — declare when the guide governs an
  environment BCG's host helpers cannot reach (WSG, CBG, PCG, JDG). State the
  shared philosophy, then the divergent mechanics. The crossing is the point.
- **Two genres on purpose** — split **Authored Disciplines** (prose, no IDs,
  internalize) from **Cited Rules** (numbered, each pointed at). The governing
  rule: *an ID earns its existence only when a citer will exist.* (CBG states
  this in full; cite it.)
- **Cited-rule scheme** — a rule ID is `{ACRONYM}{family}_{NNN}`: a guide
  acronym, a semantic family letter, an underscore, a sequence (`CBi_101`,
  `PCr_101`, `JDo_101`). Mark `✅` active / `❌` known-gap; close with
  `*Cited by:*` and, for a blessed-but-unrepaired deviation, `*Known
  deviation:*`. The handle is **semantic by design** (legible in a terse code
  comment) — the prose-guide, semantic-ID kin of MCM's **rivet** (`mcm_rivet`,
  the `.adoc`/opaque-ID species); both are citable definition sites and share
  the `axvc_` voicing vocabulary, differing only on opacity and home.
- **Reference the home; gloss in one line** — invoke a shared concept (Palisade,
  crash-fast, load-bearing complexity) with a one-line self-sufficient gloss
  *plus* a citation to its home, never a re-derivation. Preserves cold-reading
  while killing drift.
- **One file, one guide** — the allocation rule; cite ACG Related Guides.

## Citing a shared concept

The family leans on a few cross-cutting concepts with established homes — the
**Palisade** and the membrane pattern (CMK Rules of Engagement), **crash-fast /
load-bearing complexity** (BCG, CLAUDE.md), **reference the home** (ACG). Do not
relocate these into a guide and do not re-derive them: state a one-line gloss the
cold reader can stand on, then cite the home. The home owns the depth; the guide
owns the one-line bridge to its own subject.

## Related Guides

- **ACG** — the allocation discipline GMG instantiates on the guide family
  ("statements → definition sites"; one file, one guide).
- **BCG** — the headwater philosophy every guide is downstream of.
- **MCM**, **AXLA** — the spec-side analogues: MCM is to specs what GMG is to
  guides; AXLA homes spec motifs as GMG homes guide-framing motifs. The rivet
  (`mcm_rivet`) is the spec-side kin of the cited-rule scheme.
- **CBG**, **PCG**, **JDG** — the modern-form exemplars that bear every
  convention above.

## Acronym Registry

| Term | Expansion |
|------|-----------|
| GMG | Guide Meta-Guide (this document) |
| Guide | A veiled `.md` codifying durable authoring discipline; `{ACRONYM}-{Words}.md`, acronym ends in `G` |
| Canonical skeleton | The suggested first-pass section list a new guide starts from |
| Authored Disciplines | The prose, uncited, internalized patterns genre |
| Cited Rules | The numbered `{ACRONYM}{family}_{NNN}` genre; each has a citer |
| Foreign-environment sibling | A guide governing an environment BCG's host helpers cannot reach |
