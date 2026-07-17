## Character

Concept-tier judgment, not mechanical.
The aspirant matricula spec is authored and homed in VOSMM-entity.adoc; what remains is the single terminal pace that bounds the aspirant into a coherent gestalt a future build heat can adopt without drift.
The post-MVP vision lives in a separate prospective heat (₣Bj), keeping this heat lean on the MVP aspirant.

## Context

This initiative builds the Matricula — the chosen entity quoin for a transient inscription census, an "instant temporary database" re-derived on demand from repository source — that subsumes the ad-hoc grepping an agent does to answer naming questions.
It serves three jobs: choosing an unused prefix, preventing a rename from creating a collision, and planning a re-mint.
A fourth falls out: generating the acronym index that the claude-*.md files maintain by hand today.

The organizing reframe: every persistent identifier — what we informally call quoins, sprues, rivets, and filenames — is an Inscription under a Vesture in the VLS Liturgy vocabulary.
The Matricula is therefore one vesture-parameterized scanner, not many separate extractors.
Its method is classify-by-subtraction: cast the widest net over `_`/`-`-shaped tokens, let each known grammar claim what it owns, and treat the unclaimed residue (ours-but-unclassified) as a product rather than waste — it surfaces misminting, mint-gaps, and forgotten categories.

The interrogation primitives, the safe-rename apex, the engagement determinations, the operator-facing verbs, and the gestalt-label subsystem all live in ₣Bj now; this heat is the scan plus the simple integrity check on `xxx_yyyyy`-formed inscriptions.

### MVP shape

The MVP is a whole-environment scan that reports violations, delivered as a standalone VOK executable driven by its own tabtarget, with no MCP surface required.
The first violation class is the assertable minting pair — terminal-exclusivity breach and exact collision — both mechanical, honoring the never-linguistic lock.
Residue is reported as its own section, a product, never folded into violations.
The acronym-index generation falls out of the same frozen census.

### Architecture map — spec-bound findings (see Owed to a spec home)

Four layers; three already exist, one is this heat.
Grammar source-of-truth EXISTS: VOF (`Tools/vok/vof/src/vofc_registry.rs`) holds the cipher registry and the kit Conclave plus `vofc_validate_terminal_exclusivity()`.
Naming vocabulary EXISTS: VLS (Cipher / Signet / Epithet / Inscription / Vesture), imported wholesale by VOS0, which adds the tabtarget vesture.
Delivery rail EXISTS: `voff_freshen.rs` (managed-section upsert, unit-tested) and the freshen operation, which regenerate CLAUDE.md's include region from the registry.
The inventory scanner DOES NOT EXIST — that is this heat: census every in-use inscription below the cipher tier, build the signet trie, surface residue, check collisions.

The Matricula is an additive sibling, not a rework, and it reuses the existing `vof` library as a path dependency.
The `vof` crate already factors the reusable pieces — the cipher registry (`vofc_registry`), the release tree-walk (`vofr_collect`), and the managed-section rail (`voff_freshen`) — that the scanner builds on; `vvr`/`vvx` is a thin binary over that library.
The Matricula inherits these foundations: VOF as input grammar, a generalization of the release tree-walk (git-tracked ∩ allowlist rather than per-kit staging copy), and emission through the voff managed-section rail.

## Cinched

The vo-family gestalt is force-consolidated under Vox now — re-gestalt the `vo` cipher as Vox, the naming-system whole, cleaning up the fossil thought for a better forward springboard.
VOS0 becomes the true Vox cosmology SpecTop following the RBS0/JJS0/BUS0 precedent — a `0`-top plus `include::` subdocs.
"Obscura" demotes from claiming the whole to one branch (or retires), fixing VOS0's over-claiming `0`.
Fully embrace the Liturgy on the RBS0 pattern: ALL quoins centralize at the cosmology top (top defines attribute refs, anchors, and definition text; subdocs purely consume linked terms — verified against RBS0, whose subdocs define zero quoins).
VLS-as-separate-file dissolves into the top's vocabulary; the Matricula, the commit/lock family (the VOSR* specs), and any future branches become subdocs consuming it.
Matricula-specific quoins (Residue, Finding, the freeze verb) are defined at the cosmology top too, not in the subdoc.
The secrecy gradient is preserved by tier, not by file: vocabulary lives at the top (veil-strength, already travels to the test fleet), method lives in the Matricula subdoc (veil-strength), and the exploit lives in the binary (gitignore-strength, never travels) — so BUK citing the cosmology top for vocabulary (BUS0 already does, by name) touches only the vocabulary layer, never the method.
JJK contains no Vox implementation — JJK↔Vox couplings are purpose-built JJK operations with subtle Vox interactions, designed per-need (the paddock/docket integrity check is the first instance, not a template), never a blanket proxy of the whole Vox interface; much of that design likely lands in ₣Bj or future JJK heats.
The re-gestalt-plus-consolidation surgery is its own slated pace(s), not absorbed into the authoring pace; the bootstrap-collision risk during the mass rename is accepted as bounded — the Liturgy is barely adopted yet (it was a distillation of a prior conversation, not yet in use), so repair is iterative as intimate problems surface, and the Matricula itself becomes the eventual net.

Placement is VOK — entity, grammar, and spec under the `vo` cipher, homed in the re-gestalted Vox cosmology (VOS0 top plus VOSMM-entity.adoc, the Matricula subdoc), code as a standalone VOK executable over the `vof` library.
The verified build structure decides this: VOK ships nothing as source (it is not in the managed-kit list, so its tree is never collected into a parcel), which makes it a tighter vault than CMK — CMK ships through two outbound pipelines (the parcel and the open-source upstream) and is doc-only with no crate.

CMK is the doctrine home, cited and never re-homed.
The minting rules the Matricula enforces — terminal-exclusivity, collision, MCM Lapidary word-selection — are CMK/MCM doctrine; VOSMM cites them as the authority and does not restate them, so the rules keep one home.

The runtime is a standalone independent VOK executable, not a `vvr` subcommand.
The theurge crate (`rbtd`) is the precedent: its own build/test/run tabtargets, outside the VOW pipeline, shipping nothing.
This supersedes the earlier cinch that the runtime would ship as a `vvr` subcommand inside `vvx`.

Structural secrecy is the distribution boundary, and it is load-bearing security.
The Matricula source lives committed under a veiled directory (`Tools/vok/vov_veiled/<crate>/`): version-controlled, but excluded from the parcel by the veil and by VOK being unmanaged.
The built binary lands in that crate's gitignored `target/release/` — itself a subdirectory of the veil, already covered by the `**/target/` rule — and its tabtarget execs it directly from there (the `vow-r` precedent), never copying it to a committed `bin/`.
So the boundary stands on four independent guards: the source is not collected (VOK unmanaged + veiled), the behavior is not linked into `vvx` (never a `vvr` dependency), and the binary is never even committed (gitignored).
Inclusion would require a deliberate, auditable act — a new managed-kit entry, a new `vvr` dependency, or an explicit file-copy into the parcel — never a silent compile flag, which is the whole point of the standalone-binary choice over a feature-stripped subcommand.
This boundary is a candidate for a normative rivet in the subdoc.

The organizing principle is: share libraries, ship only the substrate.
Logic lives in library crates; executables are thin frontends; a tool ships only if it is part of the consumer substrate (the `vvx` installer/jjx-server/freshen binary), otherwise it is an independent, unshipped binary.
Do not put Matricula logic in `vof` — `vof` is linked into the shipped `vvx`, so anything there ships as behavior; the Matricula goes in a sibling crate that depends on `vof`.

Matricula is transient by design — hard lock.
A matricula has no persistent existence; it lives only for the call that summons it, re-derived from source each time and discarded.
Reading the whole tree is instant relative to an agent's grep laps, so nothing is stored — only the grammar (the small, stable cipher and vesture set) is maintained.
Transience is what defers the heavier maintained-registry layer.

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
Interior signets are safe to act on locally; exported signets are out of scope, handled by discipline, their verdict always "safe here; consumers require re-distribution".

Classify-by-subtraction, with residue treated as a product.
The ours/foreign gate is the project cipher, invariant across the `_` and `-` separators.

The MCP future is deferred, not designed in.
If any Matricula interaction ever wants an MCP surface, it is a peer beside jjx in the operator's local-only `vvx` (feature-stripped from the shipped binary), never a distinct server and never shipped — and that bill is paid only then.

## Landscape shift — farrier re-grounding

The revision-control re-architecture (canon: JJSVF-farrier.adoc, JJSVC-cosmology.adoc) moves the matricula's ground:
the VOK/VVK baseline that anchors it — the `vof` library and its workspace — homes in the JJ app repo (the `jjqa_app` infield resident), not this monorepo.
JJ app delivery is artifact-shaped: a parcel generator distributes the JJ binary, never the source repo,
so repo membership stops being the distribution boundary and parcel composition becomes the boundary.

Ruled: matricula source co-resides in the JJ app repo workspace —
a separate crate producing a separate command-line binary the JJ engine reaches by shell-out —
so JJ distributes without the matricula.
The standing cinches (standalone binary, never a link dependency of the shipped binary, shell-out seam with clean degradation) stand unchanged;
only their repo ground moves.

Load-bearing premise: the parcel composes by addition — it enumerates what ships, so nothing rides undeliberately.
If the generator proves subtraction-shaped, the remedy is fixing the generator, never relocating the matricula.
Boundary condition: co-residence is safe while JJ distribution stays artifact-shaped;
a source-distribution modality (open-sourcing, upstream contribution) reopens the question.

Two guards owed at the build seam, because co-residence puts the never-link rule one manifest line from breach:
a mechanical dependency-graph assertion that the shipped binary excludes the matricula crate
(resolving the anti-leak open question below toward guard, not structure alone);
and delivered-context name hygiene — no parcel-delivered context file may name the matricula (owner needed, mechanical or authorial).

Reconciliation owed at the fence walk:
VOr_q4f's guard wording (parcel-and-veil terms re-derive to parcel-composition terms),
the VOK placement cinch's repo ground,
the theurge-precedent tabtarget story (build/test/run tabtargets follow the workspace),
the crate-shape and anti-leak open questions (both now carrying simpler answers),
and the Tier-0 tackle-projection note, which lands better same-workspace.

## Decomposition

Four tiers, with the freeze line between Tier 1 and Tier 2.

Tier 0 — Grammar (maintained, frozen at load): the Cipher registry (VOF, exists), the Vesture set (VLS, exists), the file-selection allowlist (new).
Tier 1 — Raising the census (mutable builder): walk (git-tracked ∩ allowlist) → tokenize (wide net on `_`/`-`) → classify-by-subtraction (each Vesture claims; unclaimed-ours becomes Residue) → seat each Inscription into the Signet trie; then freeze.
Tier 2 — The frozen Matricula (immutable): the Signet trie (each node carrying prefix, Epithet, terminal-vs-container status, seated Inscriptions), the Residue set, query indices.
Tier 3 — Reading it (pure over the frozen census): the MVP validators and the report render + acronym-index generation. The richer interrogations (CHECK / LIST / TRACE) and SAFE-REPREFIX read the same frozen census but are scoped to ₣Bj.

The parts are mostly allocation, not new minting: Inscription, Signet, Vesture, Cipher, and Epithet are existing VLS quoins reused.
Only three are new vo work — Residue (the unclassified-but-ours product), Finding (a validator's output record), and a register-aligned word for the freeze verb (Matricula is ecclesiastical — a seal/engross/enroll/close-family word, grep-gated, not the literal "freeze").

The MVP validators fall out of the freeze invariants rather than being separate passes.
Exact collision is the frozen-map "same key, different value" conflict observed at seat-time (the reference library's IceDictionary invariant).
Terminal-exclusivity is a Signet that is both seated and has children — a property of the trie node read after freeze.

## Scan mechanics — declaration recognition

The scan distinguishes declaration sites from reference sites; this axis, not name-resolution, is what the validators ride.
A file bears declarations, references, or both: code (`.rs`/`.sh`) and specs (`.adoc`) bear both; Memos and prose `.md` bear references only.
Collision and terminal-exclusivity validate declarations (collision is two distinct declarations of one token, never an occurrence count); references feed TRACE later (in ₣Bj).

Declaration recognition is structural and per-Vesture — each Vesture carries its own decl-site recognizer (`fn prefix_…`, `prefix_…() {`, `[[anchor]]`), never whole-program name-resolution.
No semantic analyzer (rust-analyzer / `rustc_private`): the hard part of language tooling is resolution, which the Matricula does not need, and the transience lock prices a per-call semantic build out of the "instant" budget.
Tooling ladder: per-Vesture line patterns suffice for the MVP (the codebase's RCG/BCG discipline keeps decl sites pattern-clean); adopt tree-sitter (rust + bash, one model across file types) incrementally where a Vesture needs real structure; not syn (rust-only).

Reference-finding is not a resolution problem here: global-uniqueness (one meaning per word, repo-wide) makes "find every reference" an exact-token search, complete by construction.
Grep's real partiality is role-form coverage — the same name as `x_`, `zx_`, `X_`, filename, anchor, attribute — which the Vesture grammar owns; that, not resolution, is the Matricula's edge over grep.

Correctness rests on the grep-as-registry axiom: a minted name must be textually present at its declaration.
A name born from a macro (`paste!`/`concat_idents!`) or bash `eval` is invisible to a structural scan, so a referenced-but-never-declared token is itself a Finding, not something to chase with an expander.
The codebase empirically already honors this (zero identifier-synthesis); whether RCG/BCG should codify it is open.

File-role layer: atop the allowlist + git-tracked candidate gate, a path carries a role — declaration-bearing, reference-only, index-of-record (CLAUDE.md acronym tables, which the MVP will generate), or generated (e.g. the tabtarget-context).
Memos are the first reference-only case: excluded from the MVP declaration scan, with a removal condition — they re-enter as a reference corpus when TRACE lands in ₣Bj, because a memo citing an old name is a reference a rename should know about.
`.md` is the heterogeneous class that forces this layer — prose, index-of-record, and generated output cannot be scanned as one thing.

## Worked seed — a chapbook marker-law checker (memo, 260705)

Operator-directed capture of a validator written and discarded in the README-currency heat (₣Bt), during the provenance-chapbook ratification pace: a ~120-line Python script that mechanically validated the chapbook sheaf's annotation skeleton against AXLA's hierarchy-chapbook law.
It is this paddock's concerns made concrete — a per-vesture line-pattern recognizer that proved sufficient without any AST or resolution — so its shape is recorded here as evidence and feedstock; the script itself lived in session scratchpad and is gone (deliberately unhomed: minting it a durable home is validator-inventory design work, not a ratification pace's).

Mechanics, sufficient to rebuild in an hour:
each `//ax…` annotation line opens a read window that the next annotation line closes;
within a window the backtick-token stream and the attribute-reference stream are collected independently (three regexes: annotation `^//(ax\w+)((?:\s+\w+)*)$`, backtick inlay, `{attr}` reference);
declared reads consume each stream's first tokens in declared order, and tokens beyond declared arity are prose.
Checks, essentially clause-for-clause from the AXLA law text (quoined cast voicing only; the sprued arm was not needed):
opener carries exactly one voicing and one sprue inlay;
venue and legend inlays lex under the opener's sprue;
actor lanes carry exactly one kind dimension and one attribute read, with an injective cast;
interactions read from/to/contract on the attribute stream (contract waived under a license dimension), from and to resolving only to previously declared lanes;
legend read first on the backtick stream;
define-before-reference ordering (actors precede interactions, venues close at the first interaction);
open/closed vertical groups pair LIFO with the close's from-lane validated against the open's to-lane, dangling opens detected at end.

Result: across 35 interactions, 8 lanes, 3 venues, and 7 groups it surfaced exactly the sheaf's one documented covenant breach with zero false positives.

Bearing on this heat, three observations:
the annotation grammar joins RCG/BCG-disciplined code as evidence that per-vesture line patterns suffice at MVP grain;
the window-scoped two-stream read is the recognizer shape an annotation-borne vesture would need if `//ax` inscriptions ever enter the census;
and a checker of this class is a natural Finding producer should chapbook-law validation graduate to a maintained validator — whose home (matricula validator inventory versus CMK-side tooling) is a future determination, deliberately not cinched here.

## Near consumer — the integrity check

One downstream consumer stays in view because it constrains the binary architecture now: a paddock/docket integrity check at mount and groom, validating that an artifact's references still resolve.
It is a join of two oracles — the Matricula for naming and path references, the gallops for coronet and silks references — and it lives on the JJK side as a consumer of the Matricula query, not as Matricula machinery.
Because JJK ships (it is the jjx server) and the Matricula must not, the standalone-binary choice forces this check to reach the Matricula by shelling out to the unshipped binary — present in the operator's repo, absent for consumers, where it degrades cleanly to the gallops-only oracle.
The richer engagement (the ambient and gate determinations) is scoped to ₣Bj.

## Empirical input — the ₣Br mint-census pace (260705)

A full mint census ran by hand in ₣Br's word-picking pace;
its experience feeds the terminal fence-walk pace and is banked here so the fence is drawn against field evidence, not supposition.

- Frequency fact: the word-surface gate (the grimoire's mint-gate job) fired roughly twenty times;
  the MVP seating validators would have fired once (an eviction proof).
  Weigh at the deferred-scope walk — it re-weights what CHECK must mean, not necessarily the grimoire's deferral.
- The gate's pass criterion was never zero hits but all-hits-classified: provenance, aspirant surface, or incidental prose.
  The file-role layer wants provenance-corpus (Memos, retired heats, chat-archive — one gate dumped 101KB of archive JSONL)
  and data/dictionary (APCK wordlists) roles,
  plus a bindingness stratum read mechanically from the axvd_sheaf genre line,
  so CHECK answers free / taken-binding / taken-aspirant / prose-echo(n).
- A third seating state is real: *reserved* — dormant paper allocations
  (README prefix tables; JJS0 legend lines reserved-at-census)
  bind CHECK without being in-use inscriptions.
- CHECK needs a word-part index over seated inscriptions, not the prefix trie alone:
  four census kills were word-parts inside larger inscriptions
  (plant in jjx_plant, livery in RBTDRM_FIXTURE_CHAINING_LIVERY, blazon in buz_blazon, escutcheon in axvs_escutcheon).
- Troddenness has a mechanical advisory signal: raw prose hit-frequency
  (stall and dress died on count-shaped evidence).
  Report counts, never judge — VOr_m7w holds.
- Negative evidence: semantic-field adjacency (lodge against the billet's lodging gloss)
  was pure operator judgment, ledgered in prose; nothing argues for crossing the linguistic line.
- Tier-0 direction: the file-selection allowlist is anticipated as a projection of JJ's tackle table
  (aspirant tackle sheaf, representation fork, 260705) —
  one file-universe table read by a shared substrate crate for both JJ dispatch and the matricula.
  VOSMM Tier 0 records it.
- VOSMM's grimoire sketch was extended same-day with the self-maintaining-row mechanism,
  the two-verdict prose lint, the retroactive-sweep obligation, the acceptance-record open question,
  the make-room eviction-cost census, and the register tag.

## Grimoire — mulling satellite (absorbed from ₣By)

This heat is the grimoire's JJ-side mulling residence,
absorbed from the retired grimoire mulling heat (₣By)
because the grimoire is not yet an aspirant of its own —
it lives as VOSMM's deferred-scope sketch,
and that sketch remains the design accretion point: leans land there, not here.

The grimoire is the human-presentation layer of the naming system:
one flat table, one row per minted word that surfaces in prose,
each row carrying the word's literal surface forms plus categorical tags
(register, home, and the toothing subject/object slot tags per MCM's clause grammar).
Residence is the studbook as its third tenant, per the sketch's residence lean;
rows append under the journal ceremony, reads ride the lockless path.

Promotion to an aspirant sheaf of its own follows the studbook–farrier MVP heat and the matricula MVP —
banked in the sketch, inherited here.
First racing content at promotion:
the pre-validated row schema,
the population census as a defined deployment step
(reprieve-shaped: new mints comply from the doctrine's landing; the standing stock converges in one deliberate act, never big-bang),
the probe harness,
and the lint jobs (missed-citation, printed-string, blocked-word membership).

Slate-time authority for the first pace-cutting at promotion:
the sample census runs first, before any implementation pace —
a hand-run of the row schema over a dozen standing verbs
(authority-bearing, operator-plain, intransitive, plain-compound, the destruction family, the read family),
its findings amending the doctrine home (MCM's clause grammar) before code consumes it.

## Open questions

The JJK integrity-check seam — shell-out (the clean degradation, the lean) versus in-process (which would link the Matricula into the shipped `vvx` and break the secrecy boundary); and how the jjx server locates a binary that only exists on the operator's machine. Lean: shell-out, Matricula-half best-effort.
The library boundary — what rises into `vof` (and thus ships as behavior in `vvx`) versus what stays Matricula-only (the Vesture grammars and validators, which are the secret); where exactly the tree-walk generalization lands.
Crate shape — one `rbtd`-style lib+bin crate (sufficient for pure shell-out) versus a separate lib crate plus a thin bin (needed only if a real in-process consumer appears). Lean: single crate until one does.
Tabtarget surface and colophon — its own build/test/run tabtargets, and what colophon names them, under the recursive scrutiny a minting tool's own names warrant.
Whether to add a mechanical anti-leak guard (a qualify-style check that fails if the Matricula ever becomes a `vvr` dep or enters the parcel) versus relying on the structure alone.
v1 scan scope — quoins-and-signets (high fidelity, matches the minting-violation MVP) versus all classes (matches the full ask with honest coverage gaps on sprues and rivets).
Classifier growth — enumerate all classifiers up front versus bootstrap from the cipher gate plus a few and let residue drive which classifier to write next.
Vesture gaps — the sprue (wire-key) domain and the mixed-case rivet form (`RBr_`) are not covered by the current six-plus-one vestures.
Declaration-recognition fidelity — per-Vesture line patterns (MVP-cheap, leans on RCG/BCG discipline) versus a tree-sitter threshold; when does a Vesture's structure defeat a regex?
Macro/eval greppability enforcement — whether RCG and BCG should add a narrow rule forbidding identifier-synthesis, now that the Matricula's correctness leans on the grep-as-registry axiom.
Reference-placement lint — should the matricula flag inscription literals sitting where they don't belong: a colophon hardcoded in a comment or operator-facing string where it should route through its zipper constant (the reference-hygiene antipattern), distinct from the declaration-freshness check `rbq_qualify` already does. Reference-site, so it rides TRACE in ₣Bj, not the MVP — consciously place it in the future heat's fence.
Facade lint — should the matricula mechanize the revetment face law (₣Bx): flag a non-ashlar interior name or the lapidary meta-register surfacing on a face file (a spall) and confirm the face's display-anchors resolve. Its pieces seat onto the constellation sketched above — face files as a new file-role; the display-form README anchors (the `<a id>`↔`](#X)` pair) as a non-cipher vesture extending the Vesture-gaps seat; the inscription-leak check as a presentment producer over face files; the anchor sweep as the second consumer of the artifact-integrity check (the paddock/docket check being the first). Word-surface leaks — the leaked word itself, not the inscription — stay grimoire-tier and deferred with the grimoire, so the fence walk does not resurrect them. Doctrine home is ₣Bx's MCM Lapidary "The Revetment" subsection (mcm_revetment / mcm_spall / the four-invariant face law); the lint cites it, never restates. Post-MVP like the reference-placement lint above — consciously place it in the future heat's (₣Bj) fence.

## Concerns

The ours/foreign boundary is fuzzier than the cipher gate implies — we mint into foreign namespaces (service-account emails, registry paths) that carry our cipher in dash-only hosts, and we consume foreign schemas that carry no cipher.
The gate is a good first cut, but collision-checking that trusts it risks false confidence; sprue/rivet checks must stay advisory, never assertions.

Recursive vocabulary care — this is a minting tool, so its own quoins set precedent and sit under maximum scrutiny, and the subdoc work touches already-dense quoined specs (VLS, VOS0) governed by sub-letter discipline and terminal exclusivity.

## Owed to a spec home

The architecture map and decomposition above are codebase fact that must remain true after this heat retires.
These facts have migrated to their spec home — VOSMM-entity.adoc (the entity nature, census lifecycle, invariants, decomposition, and scan mechanics now live there), which states the migration in its own Overview.
The paddock is a way-station; its copy is now redundant with VOSMM and survives only as heat-shape context.

## Provenance — cite, do not restate or edit

`Memos/memo-20260620-freeze-builder-pattern/` — the operator's gbu_* freezable/immutable/ice-collection library; the reference shape for the raise→freeze→query census lifecycle.
`Memos/memo-20260110-acronym-selection-study.md` — the founding empirical prefix study (Patterns A–K); stale on specifics, predates the quoin/sprue/rivet vocabulary.
`Memos/memo-20260610-quoin-minting-introspection.md` — the introspective rule feedstock (its section 7) for the eventual validator.
Specs: VLS (Liturgy vocabulary), VOS0 (distribution plus filename-only validation), CLAUDE.md "Prefix Naming Discipline" (the current operational catalog, self-declared non-exhaustive).

## Done when

This is the aspirant-shaping heat, not the build heat.
It closes when the matricula aspirant — homed in VOSMM-entity.adoc, with the entity nature, decomposition, invariants, and MVP method set already authored there — has gelled into a coherent, bounded gestalt: a focus a future heat can take up and build without infinite drift.
The terminus is the fence, not a finished design — drawing it, not resolving every open question (which is its own drift trap), is the work.
The deferred/aspirant boundary is consciously set: what the future heat pursues versus what stays deferred (the grimoire among the deferred, by standing decision), with open design questions allowed to remain open and answered inside the future heat's scope rather than as a precondition to lighting it.
Out of scope here: building the executable (a future heat, nominated at the terminal pace) and the post-MVP widening — the richer interrogations, safe-reprefix, the engagement determinations, the gestalt-label subsystem — which is ₣Bj's.
The terminal pace draws the fence and nominates the future build heat; nothing is slated after it.
At the fence this heat does not retire:
it furloughs (stables) as the VOSMM sheaf's standing mulling residence,
carrying the grimoire satellite above,
and retires only when the grimoire promotes to its own aspirant sheaf and heat.