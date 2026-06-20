## Character

Far-horizon design conversation, concept-tier, requiring judgment.
Prospective and stabled by design: this heat is the post-MVP refinement of the Matricula, extracted from ₣Bg so the MVP stays lean.
It is not yet scoped for execution — it homes the vision so it stays discoverable, to be revisited once the ₣Bg scanner exists.

## Context

This heat collects the Matricula ideas that reach beyond the simple integrity check on `xxx_yyyyy`-formed inscriptions.
₣Bg builds the MVP: the transient census scan plus the two seating-invariant validators (terminal-exclusivity, exact collision), residue, and acronym-index generation, delivered as a standalone VOK binary.
Everything here is downstream of that frozen census — the interrogation primitives, the safe-rename apex, the engagement determinations, the operator-facing verbs, and the gestalt-label subsystem.
Each consumes the same frozen Matricula the MVP builds; none is Matricula machinery in its own right.

## Cinched

The MVP is the foundation, and this heat does not start until it lands (₣Bg).
The hard constraints inherited from ₣Bg hold here without restatement: the Matricula is mechanical, never linguistic; it is transient, re-derived from source per call; and its code lives as a standalone VOK executable that never enters the shipped binary or parcel.
Anything in this heat that would surface to consumers stays subject to that distribution boundary.

## Explicit interactions

The operator-facing interactions are mechanical set-ops over the frozen census; the metaphor-aligned verb words are roughcut-selected from the ecclesiastical register (see Verb roughcut), pending a settled operation set.
Two primitives and one apex composition cover the explicit needs.

CHECK — is a candidate token free and exclusivity-clean? Answers free, or taken-by-X.
LIST — what is used at or under a prefix (or across the top signets)? The occupied set; the operator reads the gaps and supplies the word.
TRACE — the full reach of a prefix: every source role-form (`x_`, `zx_`, `X_`, filenames) and every hierarchy level beneath it.

## SAFE-REPREFIX — the apex composition

SAFE-REPREFIX is the apex: CHECK(destination) plus TRACE(source) plus gestalt reconciliation — answering whether OLD can become NEW without breakage.
"Safe" has three faces: conflict-free (the destination, in every form, collides with nothing), gestalt-coherent (the relocated meaning clashes with no meaning declared at the destination), and complete (every use is captured, none left orphaned).
This is the use case that most justifies the Matricula over grep, because safe requires completeness and grep is always partial.

Reprefix has a detect variant and a guarded execute variant.
Detect is read-only: prove safety, emit the complete change-set.
Execute is opt-in and guarded — fail unless the tree is fully committed; perform the rename; auto-notch it as one atomic, revertable commit; verify by an independent post-scan (zero OLD remains, NEW conflict-free), not by a self-generated report.
Auto-execute is confined to the assertable cases — mechanical renames in known vestures; any change-set touching the advisory zone refuses to auto-run and drops back to detect.
Clean-tree makes a rename recoverable, not correct — correctness rests on detection completeness, so the assertable-only rule is load-bearing.

## Engagement and determinations

Beyond the simple integrity check, the Matricula surfaces through two determinations — Brief and Report are collapsed.
The ambient determination fires at officium granularity, joining the work-period probe the invitatory already runs, and surfaces naming health: residue count, mint-gaps, collision risks.
The gate determination fires at the durable-write boundary (notch, wrap): it warns loud on advisory findings and hard-blocks only the high-confidence ones — cipher terminal-exclusivity, exact quoin collision — in the precision exit-code band.
Both are downstream of the scan-report MVP; they consume the same frozen census the MVP binary builds.
The implicit determinations mint no verb — the ambient one rides officium-open, the gate rides notch.

Register seam: both the entity and its operator-facing verbs draw from the Vox Obscura ecclesiastical/inscriptional register, deliberately distinct from the equestrian horse words that manage heats.

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

Gestalt-label home — one authoritative declaration per level (the umbrella's `0` home) versus distributed declarations collected and reconciled across files.
Gloss handling — the gestalt-label concept is the emerging answer; open residue is terminal-name glosses below the umbrella level, and whether any description stays hand-held.
The operation set that seeds development — which of CHECK / LIST / TRACE / SAFE-REPREFIX is built first, and whether the verbs settle before or after the operations do.
The MCP question — if any of these interactions wants an MCP surface, it is a peer beside jjx in the local-only vvx (feature-stripped from the shipped binary), never a distinct server and never shipped.

## Concerns

Execution amplifies detection's gaps — guarded reprefix-execution turns any detection blind spot into a committed change the moment it auto-runs.
The guards are the clean-tree precondition, the atomic auto-notch, the independent post-scan invariant, and the assertable-only rule; never relax the last into auto-acting on advisory confidence.

Gloss-migration is a hidden cost — declaring gestalts for the whole tree is the migration of hundreds of curated descriptions into in-source labels; real work, easy to under-estimate, unless we accept index-only output for a first pass.

Recursive vocabulary care — this remains a minting tool, so its own quoins set precedent and sit under maximum scrutiny, and the verb/gestalt work touches already-dense quoined specs (VLS, VOS0) governed by sub-letter discipline and terminal exclusivity.

## Provenance — cite, do not restate or edit

`Memos/memo-20260209-diptych-vision.md` — the codec / validator / recension vision; The Spine, the removal-conditions ledger, the canon-versus-cross-universe recension boundary. The home of the recension concept that the verb roughcut defers around.
`Memos/memo-20260110-acronym-selection-study.md` — the founding empirical prefix study (Patterns A–K); stale on specifics, predates the quoin/sprue/rivet vocabulary.
`Memos/memo-20260610-quoin-minting-introspection.md` — the introspective rule feedstock (its section 7) for the eventual validators beyond the MVP pair.

## Done when

Deliberately unscoped — this heat is prospective.
It homes the post-MVP Matricula vision for separate refinement once the ₣Bg scanner exists; scoping happens at that revisit, not now.