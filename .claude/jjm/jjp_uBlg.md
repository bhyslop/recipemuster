## Character

Design conversation, settled at the concept tier; requires judgment.
Downstream work is heavy specification-and-vocabulary authoring, not mechanical.

## Context

This initiative builds the Matricula — the chosen entity quoin for a transient inscription
census, an "instant temporary database" re-derived on demand from repository source — that
subsumes the ad-hoc grepping an agent does to answer naming questions.
It serves three jobs: choosing an unused prefix, preventing a rename from creating a
collision, and planning a re-mint.
A fourth falls out: generating the acronym index that the claude-*.md files maintain by
hand today.

The organizing reframe: every persistent identifier — what we informally call quoins,
sprues, rivets, and filenames — is an Inscription under a Vesture in the VLS Liturgy
vocabulary.
The Matricula is therefore one vesture-parameterized scanner, not many separate extractors.
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

The Matricula is an additive sibling, not a rework.
The distribution code (release / install / uninstall / freshen) is mature and
load-bearing: the binary is `vvr` (`Tools/vok/src/vorm_main.rs`), installed as the
canonical `vvx` that also serves the jjx MCP server, so it runs on every jjx call.
The Matricula inherits foundations: VOF as input grammar, a generalization of the release
tree-walk (`vofr_collect`, which already walks the forge filtering by cipher and veiled),
and emission through the voff managed-section rail.

## Cinched

Matricula is transient by design — hard lock.
A matricula has no persistent existence; it lives only for the call that summons it,
re-derived from source each time and discarded.
Reading the whole tree is instant relative to an agent's grep laps, so nothing is stored —
only the grammar (the small, stable, hand-held cipher and vesture set) is maintained.
This decision holds for the long horizon: revisitable someday, but not for a long time.
Scope is a clean Job Jockey concept, not a standing index — most jjx MCP interactions need
no matricula at all; one is summoned only for the explicit naming interactions.
Transience is also what defers the heavier maintained-registry layer to ₣BF.

The Matricula is mechanical, never linguistic — hard constraint.
It performs set operations over inscriptions — membership, enumeration, set-difference,
vesture-grammar role-form expansion — and never generates words, fuzzy-matches, or interprets
meaning.
The principle: the Matricula reports the constraint space; the operator supplies the meaning.

Cross-repo containment: reprefix is legal only in a signet's single writable home — the forge
model, one writable repo per signet — and movements stay within a signet.
This cleanly covers interior signets (private and repo-local names nothing external
references), which are safe to act on locally.
Exported signets (the public surface consumer repos reference) are out of scope today, handled
by discipline; their rename rides the release/install pipeline, and the Matricula's verdict for
them is "safe here; consumers require re-distribution", never a bare green light.

Additive sibling under the `vo` cipher, beside VOF and `vvr` — not a rework of the mature
distribution code.

The Inscription-under-Vesture model is the unifying frame; quoins, sprues, rivets, and
filenames are vestures, not separate subsystems.

Classify-by-subtraction, with residue treated as a product.

Two front doors, both proven by existing wiring: a `vvr` subcommand with a tabtarget
(as freshen has), and/or an MCP tool through `run_mcp` so jjx/vvx invoke it directly.

The grammar/inventory split: grammar small and maintained, inventory large and derived.
The ours/foreign gate is the project cipher, invariant across the `_` and `-` separators.

The work strongly includes building out supportive quoins for the Matricula concept and the
AsciiDoc subdoc operations that home them.

## Engagement and determinations

The Matricula surfaces through two determinations, not three — Brief and Report are collapsed.
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
It is a join of two oracles — the Matricula for naming and path references, the gallops for
coronet and silks references — and it lives on the JJK side as a consumer of the Matricula
query, not as Matricula machinery.

Register seam: both the entity and its operator-facing verbs draw from the Vox Obscura
ecclesiastical/inscriptional register, deliberately distinct from the equestrian horse words
that manage heats — register-as-domain-signal, joining the Liturgy and the diptych
maintenance words.
The entity quoin is chosen — Matricula; its part/facet quoins are deferred by operator
decision; the operator-facing verbs are under active selection, kept mechanical and simple.
The implicit determinations mint no verb — the ambient one rides officium-open, the gate
rides notch; only the explicit interactions need verbs.

## Explicit interactions

The operator-facing interactions are mechanical set-ops; the metaphor-aligned verb words are
deferred — placeholders below are plain.
Two primitives and one apex composition cover the explicit needs.

CHECK — is a candidate token free and exclusivity-clean? Answers free, or taken-by-X.
LIST — what is used at or under a prefix (or across the top signets)? The occupied set; the
operator reads the gaps and supplies the word.
TRACE — the full reach of a prefix: every source role-form (`x_`, `zx_`, `X_`, filenames) and
every hierarchy level beneath it.

SAFE-REPREFIX is the apex composition — CHECK(destination) plus TRACE(source) plus gestalt
reconciliation — answering whether OLD can become NEW without breakage.
"Safe" has three faces: conflict-free (the destination, in every form, collides with nothing),
gestalt-coherent (the relocated meaning clashes with no meaning declared at the destination),
and complete (every use is captured, none left orphaned).
This is the use case that most justifies the Matricula over grep, because safe requires
completeness and grep is always partial.

Reprefix has a detect variant and a guarded execute variant.
Detect is read-only: prove safety, emit the complete change-set.
Execute is opt-in and guarded — fail unless the tree is fully committed; perform the rename;
auto-notch it as one atomic, revertable commit; verify by an independent post-scan (zero OLD
remains, NEW conflict-free), not by a self-generated report.
Auto-execute is confined to the assertable cases — mechanical renames in known vestures; any
change-set touching the advisory zone (gestalt semantic-fit, foreign namespaces, best-effort
sprue/rivet) refuses to auto-run and drops back to detect.
Clean-tree makes a rename recoverable, not correct — correctness rests on detection
completeness, so the assertable-only rule is load-bearing.

## Gestalt labels — concept, word TBD

A gestalt label is the epithet of a non-terminal: the declared meaning of a signet that
terminal exclusivity forbids from being a name.
Deep hierarchies carry meaning at every interior level (AXLA runs `ax` → `axh` → `axhr` →
`axhrg` → `axhrgc`), and today those meanings live only in rotting mapping-section comments
(`// axhr*_: Axial Hierarchy Regime`) with no forcing function.
The concept: an in-source, lexable markup that declares a level's gestalt — comment-host
agnostic (strip comment-lead and whitespace, then match a signet + sub-letters + marker + label
clue), so one mechanism documents both the quoin-prefix tree and the filename/directory tree.
The Matricula harvests these and reconciles them — flagging missing labels where children
exist, conflicting labels for one level, and drifted umbrellas.
Two boundaries: declared collisions are assertable, semantic fit is advisory (the linguistic
line); and labels must expose over-deep chains as a smell, not legitimize depth the minting
discipline would flatten.
The label is user-visible (ashlar) and needs its own MCM Word Selection mint, as Matricula
got — "gestalt label" is the working descriptor only.

## Open questions

Spec home — consolidate-and-complete the vesture catalog in VLS plus a new census/validator
spec, versus growing it inside VOS0 as the diptych vision nominated.
VLS is static grammar; VOS0 is distribution; the dynamic Matricula fits neither cleanly today.

File-selection scope — extensions may suffice now, but projects that build distributions
produce trees we must not index (build outputs, parcels, vendored copies).
Whether a project-level wildcarding / ignore mechanism is needed, beyond the existing
veiled-exclusion convention, is open.

Cross-repo soundness — resolved for current scope: reprefix is confined to a signet's single
writable home and stays within a signet, so interior reprefix is sound locally; exported-signet
renames are deferred to discipline (see Cinched and Explicit interactions).
A matricula still sees only its own repo, so multi-repo census remains a someday, not a now.

Gestalt-label home — one authoritative declaration per level (the umbrella's `0` home) versus
distributed declarations collected and reconciled across files.
The "flag discrepancies" use case only earns its keep under the distributed model.

v1 scan scope — quoins-only (high fidelity, fast) versus all classes (matches the full ask
with honest coverage gaps on sprues and rivets).

Classifier growth — enumerate all classifiers up front versus bootstrap from the cipher
gate plus a few and let residue drive which classifier to write next.

Gloss handling — the gestalt-label concept is the emerging answer: declared gestalts give the
hand-maintained claude-*.md descriptions an in-source home the scan harvests.
Open residue: terminal-name glosses below the umbrella level, and whether any description
stays hand-held.

Access pattern — an always-`@`-included generated index versus an on-demand tool;
in-context wins for hot lookups, on-demand is the only scalable option for the long tail.

Vesture gaps — the sprue (wire-key) domain and the mixed-case rivet form (`RBr_`) are not
covered by the current six-plus-one vestures.

Naming — entity quoin chosen (Matricula); part/facet quoins deferred by operator decision.
Still to mint, all user-visible (ashlar) and only after the use-case set settles: the explicit
verb words (for CHECK / LIST / TRACE / reprefix) and the "gestalt label" word.
All signets wait on the spec-home decision.

## Concerns

The ours/foreign boundary is fuzzier than the cipher gate implies — we mint into foreign
namespaces (service-account emails, registry paths) that carry our cipher in dash-only
hosts, and we consume foreign schemas that carry no cipher.
The gate is a good first cut, but collision-checking that trusts it risks false confidence;
sprue/rivet checks must stay advisory, never assertions.

Execution amplifies detection's gaps — guarded reprefix-execution is now in scope (see Explicit
interactions), so any detection blind spot (foreign namespaces, advisory gestalt-fit) becomes a
committed change the moment it auto-runs.
The guards are the clean-tree precondition, the atomic auto-notch, the independent post-scan
invariant, and the assertable-only rule; never relax the last into auto-acting on advisory
confidence.

Gloss-migration is a hidden cost — declaring gestalts for the whole tree is the migration of
hundreds of curated descriptions into in-source labels; real work, easy to under-estimate,
unless we accept index-only output for a first pass.

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
The base concept is delivered when the Matricula answers its core interrogations — CHECK,
LIST, TRACE, and safe-reprefix — and the acronym index is generated rather than
hand-maintained.