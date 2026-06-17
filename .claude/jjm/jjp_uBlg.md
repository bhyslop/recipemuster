## Character

Design conversation, settled at the concept tier; requires judgment.
Downstream work is heavy specification-and-vocabulary authoring, not mechanical.

## Context

This initiative builds a stateless inscription census — an "instant temporary database"
re-derived on demand from repository source — that subsumes the ad-hoc grepping an agent
does to answer naming questions.
It serves three jobs: choosing an unused prefix, preventing a rename from creating a
collision, and planning a re-mint.
A fourth falls out: generating the acronym index that the claude-*.md files maintain by
hand today.

The organizing reframe: every persistent identifier — what we informally call quoins,
sprues, rivets, and filenames — is an Inscription under a Vesture in the VLS Liturgy
vocabulary.
The census is therefore one vesture-parameterized scanner, not many separate extractors.
Its method is classify-by-subtraction: cast the widest net over `_`/`-`-shaped tokens,
let each known grammar claim what it owns, and treat the unclaimed residue
(ours-but-unclassified) as a product rather than waste — it surfaces misminting,
mint-gaps, and forgotten categories.

### Architecture map — spec-bound findings (see Owed to a spec home)

Four layers; three already exist, one is this heat.
Grammar source-of-truth EXISTS: VOF (`Tools/vok/vof/src/vofc_registry.rs`) holds the
cipher registry and the kit Conclave plus `vofc_validate_terminal_exclusivity()`.
Naming vocabulary EXISTS: VLS (Cipher / Signet / Epithet / Inscription / Vesture),
imported wholesale by VOS0, which adds the tabtarget vesture.
Delivery rail EXISTS: `voff_freshen.rs` (managed-section upsert, unit-tested) and the
freshen operation, which regenerate CLAUDE.md's include region from the registry.
The inventory scanner DOES NOT EXIST — that is this heat: census every in-use inscription
below the cipher tier, build the signet trie, surface residue, check collisions.

The census is an additive sibling, not a rework.
The distribution code (release / install / uninstall / freshen) is mature and
load-bearing: the binary is `vvr` (`Tools/vok/src/vorm_main.rs`), installed as the
canonical `vvx` that also serves the jjx MCP server, so it runs on every jjx call.
The census inherits foundations: VOF as input grammar, a generalization of the release
tree-walk (`vofr_collect`, which already walks the forge filtering by cipher and veiled),
and emission through the voff managed-section rail.

## Cinched

Stateless re-derivation is the architecture.
Reading the whole tree is instant relative to an agent's grep laps, so the inventory is
never stored — only the grammar (the small, stable, hand-held cipher and vesture set) is
maintained; the inventory is always derived.
Statelessness is what defers the heavier maintained-registry layer to ₣BF.

Additive sibling under the `vo` cipher, beside VOF and `vvr` — not a rework of the mature
distribution code.

The Inscription-under-Vesture model is the unifying frame; quoins, sprues, rivets, and
filenames are vestures, not separate subsystems.

Classify-by-subtraction, with residue treated as a product.

Two front doors, both proven by existing wiring: a `vvr` subcommand with a tabtarget
(as freshen has), and/or an MCP tool through `run_mcp` so jjx/vvx invoke it directly.

The grammar/inventory split: grammar small and maintained, inventory large and derived.
The ours/foreign gate is the project cipher, invariant across the `_` and `-` separators.

The work strongly includes building out supportive quoins for the census concept and the
AsciiDoc subdoc operations that home them.

## Engagement and determinations

The census surfaces through two determinations, not three — Brief and Report are collapsed.
The ambient determination fires at officium granularity, joining the work-period probe the
invitatory already runs, and surfaces naming health: residue count, mint-gaps, collision risks.
A pace-scoped briefing — the former Brief — would require linking a pace or heat to the
prefixes it touches, wit the gallops do not yet carry; that linkage belongs to ₣BF (the
deferred maintained project-context layer), so a scoped briefing is downstream of that heat,
not this one.
The gate determination fires at the durable-write boundary (notch, wrap): it warns loud on
advisory findings and hard-blocks only the high-confidence ones — cipher terminal-exclusivity,
exact quoin collision — in the precision exit-code band.

A downstream consumer falls out: a paddock/docket integrity check at mount and groom,
validating that an artifact's references still resolve.
It is a join of two oracles — the census for naming and path references, the gallops for
coronet and silks references — and it lives on the JJK side as a consumer of the census query,
not as census machinery.

Register seam (vocabulary not yet minted): operator-facing invocation verbs draw from the
equestrian register, joining the existing engagement words; the VO-side concepts — the database
entity and its determinations — draw from the ecclesiastical/inscriptional register, joining the
Liturgy.
Only one new invocation verb is actually needed, an on-demand probe; the ambient determination
rides officium-open and the gate rides notch, so they mint no new verb.

## Open questions

Spec home — consolidate-and-complete the vesture catalog in VLS plus a new census/validator
spec, versus growing it inside VOS0 as the diptych vision nominated.
VLS is static grammar; VOS0 is distribution; the dynamic census fits neither cleanly today.

File-selection scope — extensions may suffice now, but projects that build distributions
produce trees we must not index (build outputs, parcels, vendored copies).
Whether a project-level wildcarding / ignore mechanism is needed, beyond the existing
veiled-exclusion convention, is open.

Cross-repo soundness — the cipher registry is global across the VO/VV ecosystem, but a
census run in one repo sees only that repo's inscriptions.
Portable-kit inscriptions are authoritative in their forge (this repo); ciphers forged
elsewhere are invisible, so collision-checking and choose-unused against those is unsound
from here.
Accept repo-local scope and declare it, or pursue multi-repo census.

v1 scan scope — quoins-only (high fidelity, fast) versus all classes (matches the full ask
with honest coverage gaps on sprues and rivets).

Classifier growth — enumerate all classifiers up front versus bootstrap from the cipher
gate plus a few and let residue drive which classifier to write next.

Gloss handling — migrate the rich claude-*.md descriptions into a one-line artifact-header
synopsis the scan extracts, versus leaving them hand-held; only the index half is
mechanically derivable.

Access pattern — an always-`@`-included generated index versus an on-demand tool;
in-context wins for hot lookups, on-demand is the only scalable option for the long tail.

Vesture gaps — the sprue (wire-key) domain and the mixed-case rivet form (`RBr_`) are not
covered by the current six-plus-one vestures.

Naming — the database entity's quoin and the on-demand probe verb are unminted (see
Engagement and determinations for the register seam).
The entity word is choosable now; its signet waits on the spec-home decision.

## Concerns

The ours/foreign boundary is fuzzier than the cipher gate implies — we mint into foreign
namespaces (service-account emails, registry paths) that carry our cipher in dash-only
hosts, and we consume foreign schemas that carry no cipher.
The gate is a good first cut, but collision-checking that trusts it risks false confidence;
sprue/rivet checks must stay advisory, never assertions.

Scope-creep toward recension's write side — re-mint *planning* is read-only and in scope;
re-mint *execution* (tool-driven global rename) is the dangerous cross-universe half the
diptych vision deferred.
Hold the heat to the read-only census; resist the pull into rename machinery here.

Gloss-migration is a hidden cost — promising to replace the hand-maintained acronym lists
implicitly commits to migrating hundreds of curated descriptions into artifact headers,
unless we accept index-only output.

Recursive vocabulary care — this is a minting tool, so its own quoins set precedent and sit
under maximum scrutiny, and the subdoc work touches already-dense quoined specs (VLS, VOS0)
governed by sub-letter discipline and terminal exclusivity.

## Owed to a spec home

The architecture map above is codebase fact that must remain true after this heat retires;
by the "memos and paddocks are provenance, never authority" rule it needs a spec home
(VLS / VOS0 / a new census spec — see the spec-home open question).
Recorded as a removal-condition: these facts migrate to spec when the heat does its spec
work; the paddock is a way-station, not their home.

## Provenance — cite, do not restate or edit

`Memos/memo-20260209-diptych-vision.md` — the codec / validator / recension vision; The
Spine, the removal-conditions ledger, the canon-versus-cross-universe recension boundary.
`Memos/memo-20260110-acronym-selection-study.md` — the founding empirical prefix study
(thirteen assessments, Patterns A–K); stale on specifics, predates the quoin/sprue/rivet
vocabulary.
`Memos/memo-20260610-quoin-minting-introspection.md` — the introspective rule feedstock
(its section 7) for the eventual validator.
Specs: VLS (Liturgy vocabulary), VOS0 (distribution plus filename-only validation),
CLAUDE.md "Prefix Naming Discipline" (the current operational catalog, self-declared
non-exhaustive).

## Done when

Deferred to capability level; paces not yet identified.
The base concept is delivered when the census answers its four queries — lexicon,
candidate-probe, sites-of, residue — and the acronym index is generated rather than
hand-maintained.