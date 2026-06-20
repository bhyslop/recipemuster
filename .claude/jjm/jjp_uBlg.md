## Character

Design conversation, settled at the concept tier; requires judgment.
The MVP is now scoped and the object pattern chosen, so the next pace is heavy specification authoring, not mechanical.

## Context

This initiative builds the Matricula — the chosen entity quoin for a transient inscription census, an "instant temporary database" re-derived on demand from repository source — that subsumes the ad-hoc grepping an agent does to answer naming questions.
It serves three jobs: choosing an unused prefix, preventing a rename from creating a collision, and planning a re-mint.
A fourth falls out: generating the acronym index that the claude-*.md files maintain by hand today.

The organizing reframe: every persistent identifier — what we informally call quoins, sprues, rivets, and filenames — is an Inscription under a Vesture in the VLS Liturgy vocabulary.
The Matricula is therefore one vesture-parameterized scanner, not many separate extractors.
Its method is classify-by-subtraction: cast the widest net over `_`/`-`-shaped tokens, let each known grammar claim what it owns, and treat the unclaimed residue (ours-but-unclassified) as a product rather than waste — it surfaces misminting, mint-gaps, and forgotten categories.

### MVP shape

The MVP is a whole-environment scan that reports violations, delivered through a vvr subcommand + tabtarget (the voff_freshen precedent), with no MCP surface required.
The first violation class is the assertable minting pair — terminal-exclusivity breach and exact collision — both mechanical, honoring the never-linguistic lock.
Residue is reported as its own section, a product, never folded into violations.
The acronym-index generation falls out of the same frozen census.

### Architecture map — spec-bound findings (see Owed to a spec home)

Four layers; three already exist, one is this heat.
Grammar source-of-truth EXISTS: VOF (`Tools/vok/vof/src/vofc_registry.rs`) holds the cipher registry and the kit Conclave plus `vofc_validate_terminal_exclusivity()`.
Naming vocabulary EXISTS: VLS (Cipher / Signet / Epithet / Inscription / Vesture), imported wholesale by VOS0, which adds the tabtarget vesture.
Delivery rail EXISTS: `voff_freshen.rs` (managed-section upsert, unit-tested) and the freshen operation, which regenerate CLAUDE.md's include region from the registry.
The inventory scanner DOES NOT EXIST — that is this heat: census every in-use inscription below the cipher tier, build the signet trie, surface residue, check collisions.

The Matricula is an additive sibling, not a rework.
The distribution code (release / install / uninstall / freshen) is mature and load-bearing: the binary is `vvr` (`Tools/vok/src/vorm_main.rs`), installed as the canonical `vvx` that also serves the jjx MCP server.
The Matricula inherits foundations: VOF as input grammar, a generalization of the release tree-walk (`vofr_collect`), and emission through the voff managed-section rail.

## Cinched

Placement is vo/vv — corrected from an earlier JJS0 drift.
Entity, grammar, and spec live under the `vo` cipher (VLS or a new vo census subdoc, code sibling to VOF); the runtime surface ships through the `vv` binary (`vvr`/`vvx`), exactly as `vorm_main.rs` source already ships as `vvx`.
JJS0 holds only the JJK-facing consumer hooks (the determinations, the integrity check) — no machinery, no entity quoin.

Single MCP server, never a distinct one.
When an MCP surface is eventually wanted, it is a new peer tool beside jjx in the existing vvx server (dispatched via run_mcp), stateless and officium-free — a distinct server would fork the mature distribution machinery and give jjx a cross-process liveness dependency.
The MVP needs no MCP surface; the tabtarget is the front door, and is sufficient.

Matricula is transient by design — hard lock.
A matricula has no persistent existence; it lives only for the call that summons it, re-derived from source each time and discarded.
Reading the whole tree is instant relative to an agent's grep laps, so nothing is stored — only the grammar (the small, stable cipher and vesture set) is maintained.
This decision holds for the long horizon: revisitable someday, but not for a long time.
Transience is what defers the heavier maintained-registry layer to ₣BF.

The Matricula is mechanical, never linguistic — hard constraint.
It performs set operations over inscriptions — membership, enumeration, set-difference, vesture-grammar role-form expansion — and never generates words, fuzzy-matches, or interprets meaning.
The Matricula reports the constraint space; the operator supplies the meaning.

The object pattern is freeze/builder — collections of immutables, accumulated mutable, then frozen into immutable structures safe to read.
The reference artifact is `Memos/memo-20260620-freeze-builder-pattern/` (the operator's gbu_* codegen library, provenance only).
In Rust this is a compiler-enforced typestate: a Builder the scan owns and mutates, consumed by freeze into an immutable census struct.
The census lifecycle is exactly this — raise (mutable scan) → freeze → query → discard.

File selection is an explicit allowlist in a distinct Rust file — `.md`, `.adoc`, `.rs`, `.sh` at minimum, recursive — narrowed by a git-tracked-only gate (excludes build outputs, vendored copies, and untracked artifacts for free) and the cipher/veiled gate.
The allowlist is grammar: small, hand-held, version-controlled, sibling to VOF's cipher registry.

Cross-repo containment: reprefix is legal only in a signet's single writable home — the forge model, one writable repo per signet — and movements stay within a signet.
Interior signets are safe to act on locally; exported signets are out of scope today, handled by discipline, their verdict always "safe here; consumers require re-distribution".

Classify-by-subtraction, with residue treated as a product.
The ours/foreign gate is the project cipher, invariant across the `_` and `-` separators.

## Decomposition

Four tiers, with the freeze line between Tier 1 and Tier 2.

Tier 0 — Grammar (maintained, frozen at load): the Cipher registry (VOF, exists), the Vesture set (VLS, exists), the file-selection allowlist (new).
Tier 1 — Raising the census (mutable builder): walk (git-tracked ∩ allowlist) → tokenize (wide net on `_`/`-`) → classify-by-subtraction (each Vesture claims; unclaimed-ours becomes Residue) → seat each Inscription into the Signet trie; then freeze.
Tier 2 — The frozen Matricula (immutable): the Signet trie (each node carrying prefix, Epithet, terminal-vs-container status, seated Inscriptions), the Residue set, query indices.
Tier 3 — Reading it (pure over the frozen census): the validators, the report render + acronym-index generation, and later the CHECK / LIST / TRACE primitives; SAFE-REPREFIX much later.

The parts are mostly allocation, not new minting: Inscription, Signet, Vesture, Cipher, and Epithet are existing VLS quoins reused.
Only three are new vo work — Residue (the unclassified-but-ours product), Finding (a validator's output record), and a register-aligned word for the freeze verb (Matricula is ecclesiastical — a seal/engross/enroll/close-family word, grep-gated, not the literal "freeze").

The MVP validators fall out of the freeze invariants rather than being separate passes.
Exact collision is the frozen-map "same key, different value" conflict observed at seat-time (the reference library's IceDictionary invariant).
Terminal-exclusivity is a Signet that is both seated and has children — a property of the trie node read after freeze.

## Engagement and determinations

The Matricula surfaces through two determinations, not three — Brief and Report are collapsed.
The ambient determination fires at officium granularity, joining the work-period probe the invitatory already runs, and surfaces naming health: residue count, mint-gaps, collision risks.
The gate determination fires at the durable-write boundary (notch, wrap): it warns loud on advisory findings and hard-blocks only the high-confidence ones — cipher terminal-exclusivity, exact quoin collision — in the precision exit-code band.
Both are downstream of the scan-report MVP; they consume the same frozen census the tabtarget builds.

A downstream consumer falls out: a paddock/docket integrity check at mount and groom, validating that an artifact's references still resolve.
It is a join of two oracles — the Matricula for naming and path references, the gallops for coronet and silks references — and it lives on the JJK side as a consumer of the Matricula query, not as Matricula machinery.

Register seam: both the entity and its operator-facing verbs draw from the Vox Obscura ecclesiastical/inscriptional register, deliberately distinct from the equestrian horse words that manage heats.
The entity quoin is chosen — Matricula; its part quoins are largely VLS reuse (see Decomposition); the operator-facing verbs are under active selection, kept mechanical and simple.
The implicit determinations mint no verb — the ambient one rides officium-open, the gate rides notch.

## Explicit interactions

The operator-facing interactions are mechanical set-ops; the metaphor-aligned verb words are roughcut-selected from the ecclesiastical register (see Verb roughcut), pending a settled operation set.
These are downstream of the MVP — specced in the subdoc, built later.
Two primitives and one apex composition cover the explicit needs.

CHECK — is a candidate token free and exclusivity-clean? Answers free, or taken-by-X.
LIST — what is used at or under a prefix (or across the top signets)? The occupied set; the operator reads the gaps and supplies the word.
TRACE — the full reach of a prefix: every source role-form (`x_`, `zx_`, `X_`, filenames) and every hierarchy level beneath it.

SAFE-REPREFIX is the apex composition — CHECK(destination) plus TRACE(source) plus gestalt reconciliation — answering whether OLD can become NEW without breakage.
"Safe" has three faces: conflict-free (the destination, in every form, collides with nothing), gestalt-coherent (the relocated meaning clashes with no meaning declared at the destination), and complete (every use is captured, none left orphaned).
This is the use case that most justifies the Matricula over grep, because safe requires completeness and grep is always partial.

Reprefix has a detect variant and a guarded execute variant.
Detect is read-only: prove safety, emit the complete change-set.
Execute is opt-in and guarded — fail unless the tree is fully committed; perform the rename; auto-notch it as one atomic, revertable commit; verify by an independent post-scan (zero OLD remains, NEW conflict-free), not by a self-generated report.
Auto-execute is confined to the assertable cases — mechanical renames in known vestures; any change-set touching the advisory zone refuses to auto-run and drops back to detect.
Clean-tree makes a rename recoverable, not correct — correctness rests on detection completeness, so the assertable-only rule is load-bearing.

## Verb roughcut — register-selected, operations unsettled

The operator-facing verbs were drawn from the ecclesiastical/inscriptional register by a blind cold-probe pass over independently-generated candidates.
The set of operations that will actually seed development is not yet settled, so these are recommendations, not commitments.
- check (is a candidate token free and exclusivity-clean): scrutine — the teller's examination of an entry before it is admitted to the roll; fresh readers recovered "check / validate", and it landed clean on the grep gate.
- trace (the full reach of a prefix, every form and level): perlustrate — to survey the whole compass thoroughly, with completeness carried inside the word itself; the strongest result of the pass, and the cleanest distinction from list.
- reprefix (safe rename capturing every use): transume or rescript — an open choice between separation and first-contact warmth.
- list (the occupied set at or under a prefix): open — the register-favoured recense is withdrawn, because it collides with the diptych vision's recension (its candidate word for tool-executed global renaming).
The three settled words keep first letters distinct — scrutine, perlustrate, transume — the separation the heightened care these operator-facing verbs warrant deliberately preserves.

## Gestalt labels — concept, word TBD

A gestalt label is the epithet of a non-terminal: the declared meaning of a signet that terminal exclusivity forbids from being a name.
Deep hierarchies carry meaning at every interior level (AXLA runs `ax` → `axh` → `axhr` → `axhrg` → `axhrgc`), and today those meanings live only in rotting mapping-section comments with no forcing function.
The concept: an in-source, lexable markup that declares a level's gestalt — comment-host agnostic — so one mechanism documents both the quoin-prefix tree and the filename/directory tree.
The Matricula harvests these and reconciles them — flagging missing labels where children exist, conflicting labels for one level, and drifted umbrellas.
Two boundaries: declared collisions are assertable, semantic fit is advisory (the linguistic line); and labels must expose over-deep chains as a smell, not legitimize depth the minting discipline would flatten.
The label is user-visible (ashlar) and needs its own MCM Lapidary mint — "gestalt label" is the working descriptor only.

## Open questions

v1 scan scope — quoins-and-signets (high fidelity, matches the minting-violation MVP) versus all classes (matches the full ask with honest coverage gaps on sprues and rivets).
Classifier growth — enumerate all classifiers up front versus bootstrap from the cipher gate plus a few and let residue drive which classifier to write next.
Gestalt-label home — one authoritative declaration per level (the umbrella's `0` home) versus distributed declarations collected and reconciled across files.
Gloss handling — the gestalt-label concept is the emerging answer; open residue is terminal-name glosses below the umbrella level, and whether any description stays hand-held.
Vesture gaps — the sprue (wire-key) domain and the mixed-case rivet form (`RBr_`) are not covered by the current six-plus-one vestures.
Naming — entity quoin chosen (Matricula); parts reuse VLS (Inscription, Signet, Vesture, Cipher, Epithet); the subdoc must still mint Residue, Finding, and the freeze verb, plus the deferred operator-facing verbs and the gestalt-label word, each through the grep gate and MCM Lapidary.

## Concerns

The ours/foreign boundary is fuzzier than the cipher gate implies — we mint into foreign namespaces (service-account emails, registry paths) that carry our cipher in dash-only hosts, and we consume foreign schemas that carry no cipher.
The gate is a good first cut, but collision-checking that trusts it risks false confidence; sprue/rivet checks must stay advisory, never assertions.

Execution amplifies detection's gaps — guarded reprefix-execution is in scope (downstream of the MVP), so any detection blind spot becomes a committed change the moment it auto-runs.
The guards are the clean-tree precondition, the atomic auto-notch, the independent post-scan invariant, and the assertable-only rule; never relax the last into auto-acting on advisory confidence.

Gloss-migration is a hidden cost — declaring gestalts for the whole tree is the migration of hundreds of curated descriptions into in-source labels; real work, easy to under-estimate, unless we accept index-only output for a first pass.

Recursive vocabulary care — this is a minting tool, so its own quoins set precedent and sit under maximum scrutiny, and the subdoc work touches already-dense quoined specs (VLS, VOS0) governed by sub-letter discipline and terminal exclusivity.

## Owed to a spec home

The architecture map and decomposition above are codebase fact that must remain true after this heat retires.
The spec home is now decided — the vo subdoc this heat authors (see Done when) — so these facts migrate there as the heat does its spec work.
The paddock is a way-station, not their home.

## Provenance — cite, do not restate or edit

`Memos/memo-20260620-freeze-builder-pattern/` — the operator's gbu_* freezable/immutable/ice-collection library; the reference shape for the raise→freeze→query census lifecycle.
`Memos/memo-20260209-diptych-vision.md` — the codec / validator / recension vision; The Spine, the removal-conditions ledger, the canon-versus-cross-universe recension boundary.
`Memos/memo-20260110-acronym-selection-study.md` — the founding empirical prefix study (Patterns A–K); stale on specifics, predates the quoin/sprue/rivet vocabulary.
`Memos/memo-20260610-quoin-minting-introspection.md` — the introspective rule feedstock (its section 7) for the eventual validator.
Specs: VLS (Liturgy vocabulary), VOS0 (distribution plus filename-only validation), CLAUDE.md "Prefix Naming Discipline" (the current operational catalog, self-declared non-exhaustive).

## Done when

The MVP is delivered when a whole-environment scan, run through a vvr subcommand + tabtarget, reports the two minting violations (terminal-exclusivity, exact collision) plus a residue section, and the acronym index is generated from the same frozen census rather than hand-maintained.
The concept widens later to the CHECK / LIST / TRACE interrogations and safe-reprefix — specced in the subdoc, out of the MVP.
The immediate next step is narrower: author the vo subdoc (the first slated pace) that homes the entity nature, the decomposition, and the MVP method set.