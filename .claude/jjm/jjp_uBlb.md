## Shape

Pre-MVP source/spec hygiene, feathered in pace-at-a-time alongside MVP feature
finalization. Each pace carries a targeted blast radius that merges trivially
into final feature work — no cross-cutting refactors. A seed, not a tree: scope
stays tight to the source/spec allocation family (magic-string -> constant
sweeps, decision-detail hoisting, source-documentation standards). Resist
becoming the junk drawer for unrelated cleanups.

## Governing idea

The ACG (Allocation Coding Guide) spine: don't recreate inline what has a named
home; reference the home. Values -> constants; concepts -> quoin-refs. MCM builds
the named homes; ACG governs source's obligation to reference rather than
recreate, and where prose with no spec-home actually goes.

Three homes by when-read: design-time -> spec, edit-time -> source comment
(operational mechanics only — language idiom + Pale/foreign-boundary),
execution-time -> runtime announcement. Most conceptual comments are temporal
misallocation — design-time or execution-time knowledge dumped into the
edit-time medium, where it rots for lack of a forcing function.

## Cinched

- ACG = "Allocation Coding Guide" (not "Asciidoc" — the guide governs source
  authoring/allocation, barely touches asciidoc; that is MCM's domain).
  Veiled/proprietary, guide-family sibling to BCG/RCG/WSG/CBG. Not released at
  MVP.
- v1 posture: bless and name emergent practices, state the spine. Do NOT mandate
  universals ("every verb must announce") — those are candidates, confirmed
  pace-by-pace. No qualify-enforcement in v1.
- Two source-doc forms are distinct and both blessed: contract header (bounded
  comment, edit-time) vs intent announcement (runtime printout, execution-time).
  Failure-path option disclosure (the missing-param-shows-options practice, live
  in rbfc_require_vessel_sigil) is blessed alongside happy-path announcement.

## Sequencing & lanes

The ACG guide and the eviction mechanism are heat-internal and buildable now, no
cross-heat dependency. The vocabulary work splits by who owns the terms:

- Eviction (removing superseded terms) is gated on ₣BH — the lode-capture heat
  curates the KILL/KEEP list and retires the terms; the mechanism pace builds the
  standing theurge engine, and its rows populate per-cluster behind ₣BH's
  cutovers. ACG names that engine as the canonical home for evicted-term
  assertions (ACG eating its own dog food — a named home, referenced not
  recreated).
- Cluster repair (curating living vocabulary to a single home) is disjoint from
  eviction and never removes terms. The slated noun clusters are BH/BZ-safe in
  their terms but file-gated on ₣BH, since their descriptions share rbz_zipper.sh
  and the doc_briefs ₣BH is rewriting.

Deliberately not slated until their owning heats land: reliquary / Lode / Hallmark
vocabulary (₣BH's lane), Manor / Depot / account-role vocabulary (₣BZ's lane,
citizen-model), and verb clusters (₣BH curates its own verb descriptions during
cutover). Slating against terms two hot heats are still moving would only
re-create the drift this heat exists to kill.

## Done looks like

ACG v1 exists and is reachable from BCG/RCG; the rbrv.env literal has a single
named home in both bash and rust as the first worked application of the
magic-string discipline.