## Character

Specification authoring; concept-tier judgment, not mechanical.
The MVP is scoped, the object pattern is chosen, and the placement/distribution direction is now settled (standalone VOK executable), so the next pace is heavy specification authoring.
The post-MVP vision has been extracted to a separate prospective heat (₣Bj) so this heat stays lean on the MVP.

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

Placement is VOK — entity, grammar, and spec under the `vo` cipher (VLS or a new vo census subdoc), code as a standalone VOK executable over the `vof` library.
The verified build structure decides this: VOK ships nothing as source (it is not in the managed-kit list, so its tree is never collected into a parcel), which makes it a tighter vault than CMK — CMK ships through two outbound pipelines (the parcel and the open-source upstream) and is doc-only with no crate.

CMK is the doctrine home, cited and never re-homed.
The minting rules the Matricula enforces — terminal-exclusivity, collision, MCM Lapidary word-selection — are CMK/MCM doctrine; the vo subdoc cites them as the authority and does not restate them, so the rules keep one home.

The runtime is a standalone independent VOK executable, not a `vvr` subcommand.
The theurge crate (`rbtd`) is the precedent: its own build/test/run tabtargets, outside the VOW pipeline, shipping nothing.
This supersedes the earlier cinch that the runtime would ship as a `vvr` subcommand inside `vvx`.

Structural secrecy is the distribution boundary, and it is load-bearing security.
The Matricula never becomes a `vvr` dependency and never enters the release parcel: its source never ships because VOK is unmanaged, and its compiled behavior never ships because it is not linked into `vvx`.
Inclusion would require a deliberate, auditable file-copy into the parcel — never a silent compile flag — which is the whole point of the standalone-binary choice over a feature-stripped subcommand.
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

## Near consumer — the integrity check

One downstream consumer stays in view because it constrains the binary architecture now: a paddock/docket integrity check at mount and groom, validating that an artifact's references still resolve.
It is a join of two oracles — the Matricula for naming and path references, the gallops for coronet and silks references — and it lives on the JJK side as a consumer of the Matricula query, not as Matricula machinery.
Because JJK ships (it is the jjx server) and the Matricula must not, the standalone-binary choice forces this check to reach the Matricula by shelling out to the unshipped binary — present in the operator's repo, absent for consumers, where it degrades cleanly to the gallops-only oracle.
The richer engagement (the ambient and gate determinations) is scoped to ₣Bj.

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

## Concerns

The ours/foreign boundary is fuzzier than the cipher gate implies — we mint into foreign namespaces (service-account emails, registry paths) that carry our cipher in dash-only hosts, and we consume foreign schemas that carry no cipher.
The gate is a good first cut, but collision-checking that trusts it risks false confidence; sprue/rivet checks must stay advisory, never assertions.

Recursive vocabulary care — this is a minting tool, so its own quoins set precedent and sit under maximum scrutiny, and the subdoc work touches already-dense quoined specs (VLS, VOS0) governed by sub-letter discipline and terminal exclusivity.

## Owed to a spec home

The architecture map and decomposition above are codebase fact that must remain true after this heat retires.
The spec home is decided — the vo subdoc this heat authors (see Done when) — so these facts migrate there as the heat does its spec work.
The paddock is a way-station, not their home.

## Provenance — cite, do not restate or edit

`Memos/memo-20260620-freeze-builder-pattern/` — the operator's gbu_* freezable/immutable/ice-collection library; the reference shape for the raise→freeze→query census lifecycle.
`Memos/memo-20260110-acronym-selection-study.md` — the founding empirical prefix study (Patterns A–K); stale on specifics, predates the quoin/sprue/rivet vocabulary.
`Memos/memo-20260610-quoin-minting-introspection.md` — the introspective rule feedstock (its section 7) for the eventual validator.
Specs: VLS (Liturgy vocabulary), VOS0 (distribution plus filename-only validation), CLAUDE.md "Prefix Naming Discipline" (the current operational catalog, self-declared non-exhaustive).

## Done when

The MVP is delivered when a whole-environment scan, run as a standalone VOK executable through its own tabtarget, reports the two minting violations (terminal-exclusivity, exact collision) plus a residue section, and the acronym index is generated from the same frozen census rather than hand-maintained.
The concept widens later — the CHECK / LIST / TRACE interrogations, safe-reprefix, the engagement determinations, and the gestalt-label subsystem — but that widening is scoped to ₣Bj, not this heat.
The immediate next step is narrower: author the vo subdoc (the first slated pace) that homes the entity nature, the decomposition, and the MVP method set.