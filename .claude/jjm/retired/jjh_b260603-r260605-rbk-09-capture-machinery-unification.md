# Heat Trophy: rbk-09-capture-machinery-unification

**Firemark:** ₣BX
**Created:** 260603
**Retired:** 260605
**Status:** retired

## Paddock

## Heat nature

Behavior-preserving refactor heat (one small declared behavior-adding sliver — the recipe validator; see Premises). Unify the fetched-side capture machinery — a host assembly spine plus a shared cloud step-library — and decompose the monolithic host modules into small, well-prefixed files, all extracted from the capture instances that exist today, so future Lode kinds attach as thin per-kind body files rather than copy-pasted forks. No new Lode kinds in this heat.

## Why this heat exists

The bole pilot is an unfactored fork: its host build-submit largely duplicates reliquary inscribe's (the comment at rbld_Lode.sh even reads "Mirrors zrbfl_inscribe_submit"), and its cloud capture step is a copy of enshrine's. Across the cloud step family the duplication is pervasive — the metadata-token fetch in every skopeo step, the inspect→digest→fingerprint→copy block shared by enshrine and ensconce, the FROM-scratch vouch-push shared by Lode (rbgjl02) and hallmark-verify (rbgjv03). On top of the forks, the host modules are monolithic (rbld_Lode.sh ~490 lines, rbfl_FoundryLedger.sh ~900, rbfc_FoundryCore.sh ~1576), so reuse means sourcing a slab. Three live instances — bole, reliquary-inscribe, and enshrine — inform the one capture pattern, which is the moment to abstract from instances we have rather than speculate. We migrate only the survivor (bole); the two doomed instances inform the design and are left for later removal (see Premises, Scope).

## Premises

**Behavior-preserving is the rail — and its meaning is precise.** Every machinery change is a refactor whose oracle is the test suite. "Behavior-preserving" means the produced artifacts and their SLSA provenance verdicts are identical before and after — NOT that the generated build JSON is byte-identical (step ids, ordering, and inlined script bytes will change). The heat must confirm SLSA provenance is insensitive to step restructuring (provenance records build inputs/outputs, not step cosmetics); that confirmation is part of done. This rail sorts behavior-adding work out of the heat by construction: the ensconce collision guard and the reliquary namespace cutover are behavior-adding and out of scope.

**The one declared exception: the recipe validator.** A dispatch-time validator — checks each step's declared requires against substitutions plus prior steps' provides, failing a bad composition before the expensive build — is genuinely new code. Its accept-path is behavior-preserving (existing recipes pass through unchanged), but its reject-path is exercised by no current fixture, so it carries its own targeted unit tests. It guards composition, not step logic; a logic regression still surfaces only in the full cook. Its value multiplies as more kinds compose recipes against it.

**Read three to design, migrate one — why this still avoids speculation.** The spine's generality is grounded three ways: bole's existing tests exercise the rich path (skopeo capture + members[] envelope + buildStepOutputs extract) that every later kind copies; the spine is designed against the capture domain's known axes (N-member envelopes, tool-variety, opaque-blob payloads); and the first real downstream kind is the empirical second body. inscribe is read as a third instance to inform the spine's shape but is NOT migrated: it is slated for later removal, and it validates the wrong axis — a single docker step with no vouch envelope and no extract, a shape no Lode kind reproduces. Migrating it would be throwaway work proving an off-axis slice. So inscribe joins enshrine as reference-not-migrated; bole alone is migrated. This relaxes the older "prove generality with two riding bodies" heuristic deliberately — the only available second body is both doomed and off-axis, so the heuristic does not fit here.

**Why anticipation stays honest.** Generalize against the instances we have; never stub the ones we don't. Real code lives in discrete files — the spine, the bole body, the shared cloud library, and the decomposed host modules — with no placeholder files or recipes for the unbuilt kinds. Reserving a submint letter for a future kind in the legend is an allocation (documented, per the project mint discipline), not a stub: no file is created until the kind is.

**Why the spine takes data, not kinds.** A new kind is a new body file the spine consumes as data — an ordered recipe of step ids plus a substitutions blob — never a branch inside the spine. The spine carries no per-kind conditional and no kind-registration or discovery mechanism; registering kinds is left to the consumer, not baked into the spine. Keeping the spine registration-agnostic is what lets future kinds attach as file-disjoint bodies. Bole proves the spine takes data.

**Why small, well-prefixed files over monoliths.** Decomposition along reuse seams is the foundation for granular reuse: a consumer sources the two helpers it needs, not a 1500-line slab. The cost is near-zero modulo per-file BCG ceremony — a sourced-guard always, a kindle/sentinel only where the file holds state. The load-bearing investment is prefix minting: good unique prefixes make file-for-function discoverable from the prefix alone. The decomposition is pure relocation — zero logic edits ride along — so behavior-preservation stays trivially true and skirmish is a sufficient oracle. Annealing logic while in there is a separate, named pace, never smuggled into a move.

**Why we synthesize cloud reuse rather than adopt a vendor mechanism.** Cloud Build offers no native step reuse (no anchors, no includes). Its blessed mechanism — custom builder images — conflicts with the established posture that the cloud is just labor and the Director controls what enters the build: a custom builder holding our logic would be a new vouched supply-chain artifact, the opposite of inline-from-repo. So composition is a deliberate membrane, aligned with the existing stitch vocabulary rather than a competing term. Recorded in RBSCB.

**Why the metadata token stays, in-memory.** Manual metadata-server token fetch is the canonical skopeo-to-GAR authentication; docker steps auto-auth, but skopeo cannot use the gcloud helper, and docker-credential-gcr would graft a docker-ecosystem dependency into the skopeo container for the same token, with documented friction. The token is correct and irreducible; only the copy-paste is wrong — it becomes a composed snippet authored once. Invariant: /workspace carries non-secret data only — secrets are step-local and re-minted. Recorded in RBSCB.

**Why the contract plus validator are the load-bearing artifact.** Reuse runs over two channels: composed code snippets (in-memory — e.g. the canonical token and the skopeo copy/digest helper, authored once) and /workspace shared registers (non-secret data, with explicit per-step requires/provides). The disease being cured is undocumented-pattern-by-example; the cure is a documented contract plus the validator, so the codebase teaches "compose from the library," not "copy the script." The contract is recorded as a section of RBSCJ (CloudBuildJson) — cross-cutting cloud-build-composition infra, distinct from the Lode-domain spec (RBSL). Documentation is part of done, not polish.

## Shape

- **Host capture-assembly spine** (`rblds_`) — composes a build from a recipe (ordered step ids) plus a substitutions blob, submits, polls, and optionally extracts buildStepOutputs into per-capture capture-files. Takes the recipe as data; owns no kind knowledge. Lode-family, since post-inscribe every caller is a Lode kind. Built on the relocated Foundry-core build primitives (`rbfcb_`: write-script-body, wait-build-completion, native-path, git-metadata), which it calls rather than absorbs.
- **Shared cloud step-library** — the genuinely-identical whole step is **vouch-push** (rbgjl02 + rbgjv03, two live callers, the one made-side touch), unified as one library step. Shared in-step logic — the metadata-token fetch and the skopeo copy/digest block — becomes **composed snippets** the host stitches into a kind's capture step; envelope authoring (the members[] tail) stays per-kind, because each kind authors a different envelope. Kind-specific steps (inscribe's docker-mirror) stay specific and unmigrated.
- **Dispatch-time recipe validator** — the declared behavior-adding sliver; cheap guard against the expensive build; carries its own tests.
- **Shared lifecycle REST** (`rbldl_`) — divine (enumerate) and banish (delete) are direct GAR-REST host ops, NOT build-composition; kept as a distinct file from the assembly spine.
- **Per-kind body** — bole ensconce (`rbldb_`) as a thin recipe + substitutions + envelope intent riding the spine. Future kinds add sibling body files on the reserved kind-letters.
- **Host-module decomposition** (relocation only — see Scope):
  - `rbld → rbldX_` (Lode capture): `rbld0_` 0-top CLI, `rblds_` spine, `rbldb_` bole body, `rbldl_` lifecycle REST, `rbldk_` kindle-entry. Reserved for future kinds (legend only, no files this heat): `rbldt_` tool, `rbldr_` reliquary/conclave, `rbldw_` wsl, `rbldv_` podvm — kind-letters matching the Lode GAR kind-letters.
  - `rbfl → rbflX_` (Foundry Ledger): `rbfl0_` 0-top CLI plus gesture clusters (inscribe / yoke / delete / inventory / wrest) and `rbflk_` kindle-entry; letters allocated at the mint pace, cluster boundaries refined at the explosion pace.
  - `rbfc → rbfcX_` (Foundry Core, the 1576-line monolith): `rbfc0_` 0-top CLI, `rbfcv_` vessel-resolution, `rbfcb_` build-host primitives, `rbfca_` step-assembly, `rbfcg_` GAR-REST, `rbfcp_` plumb (~640 lines), `rbfck_` kindle-entry. The plumb extraction is the single biggest file win.

## Scope

**In**: the capture + vouch machinery — host spine (from bole), the cloud capture/vouch steps for ensconce, and the vouch-push unification (which legitimately touches one made-side step, hallmark-verify's rbgjv03); plus the host-module decomposition (rbld fully; rbfl and rbfc as relocation).

**Out**: logic changes to the conjure build pipeline (rbgjb) and the about/syft pipeline (rbgja); the vouch verify steps (download-verifier, verify-provenance) — only the vouch-push step is in; any new Lode kind; any behavior change beyond the declared validator; tool swaps (skopeo to crane); custom-builder-image migration; merging the capture spine with the made-side conjure stitch (horizon).

**Cinched — inscribe is NOT migrated.** Inscribe's machinery stays a fork; it is relocated into its own file (relocation, not unification) and left for later cutover work to remove whole. Migrating its logic onto the spine would be throwaway and off-axis (see Premises). It joins enshrine as a doomed reference that informs the spine's shape.

**Cinched — enshrine is reference, not migrated.** Left for later cutover work to delete; its cloud step's copy stays in place rather than migrate doomed code. The capture-cousin dedup is therefore deliberately partial until that removal.

**Cinched — the Foundry explosion is relocation only.** Decomposing rbfl/rbfc reaches made-side machinery (step-assembly, plumb); this is acceptable because zero logic edits ride along, so behavior is trivially preserved and skirmish is a sufficient oracle. The discipline is absolute — a move carries no logic change. Annealing is a separate named pace.

## Relocation discipline (cinched)

Discovered in pre-implementation review; binds both host-module explosion paces — the decomposition is pure relocation only if the scaffolding is treated as copy-verbatim atoms, not as code to redistribute.

- **Kindle/sentinel/guard is one indivisible atom per module.** Each module keeps a single entry file (`rbldk_` for rbld, `rbfck_` for rbfc) that holds the one inclusion-guard and the whole kindle and sources the guard-free body clusters; the readonly consts the kindle sets stay there and are read globally. Do not mint a guard per cluster file and do not split the kindle — both are logic edits, and per-cluster kindles break the single `ZRBFC_KINDLED` idempotency gate. (This is what "a sourced-guard always" means in practice: one guard per source-unit, on the entry file — not one per cluster.)
- **Leaked globals move verbatim.** The tool-image vars (`z_rbfc_tool_*`) are global-by-bare-assignment and read cross-module (rbfd reads them); adding `local`/`readonly`/`declare` while relocating silently breaks step assembly. Copy them untouched.
- **Flat placement only.** All cluster files stay flat siblings in `Tools/rbk/`; `BASH_SOURCE`-relative path derivations break if a cluster is nested.
- **Terminal exclusivity is file-level; dispatch functions keep the container prefix.** The decomposition retires the old `prefix_*.sh` files, but command and helper functions keep the container prefix — `rbld_divine`, `zrbfc_native_path_capture` move into cluster files *unchanged* (precedent: `rbho_crash_course` lives in `rbhocc_*.sh`; `buc_execute` dispatches on the single container prefix). So function CALLS are not rewritten — the forced sweep is only source-line repoints where a deleted file was named: the zipper enrollment, the Rust test driver's `source` of the old module (`rbtdrf_fast.rs`), and sibling context docs (acronym map, tabtarget / theurge context). This collapses the foundry explosion's sweep from a cross-language rename to a handful of `source`-line edits.
- **The multifacet CLI homes on the `0`-top, not the kindle.** When a container gains children its bare `prefix_cli.sh` retires onto `prefix0_cli.sh` (keeping `zprefix_furnish` and `buc_execute prefix_`) — the `0`-top, per the rbh0 precedent (`rbho0_cli.sh`, separate from `rbhob_base.sh`): `rbld0_`, `rbfc0_`, `rbfl0_`. The kindle-entry (`rbldk_`/`rbfck_`/`rbflk_`) is not the CLI home.

## Verification

Provenance-insensitivity is spiked early — read the verifier (rbgjv02) to confirm it keys on build inputs/outputs not step bytes, with one targeted cook only if ambiguous — so the spine and library invest on confirmed ground rather than discovering a broken rail at the gate. Cheap inner loop thereafter: shellcheck plus targeted fixtures, plus the validator's own unit tests; the explosion's inner loop is shellcheck plus a fixture subset, since relocation-only makes behavior trivially preserved. Milestone gate: full skirmish plus the service lode-lifecycle fixture, green with artifacts and provenance verdicts identical to pre-refactor (provenance-insensitivity-to-restructuring confirmed end-to-end). Operational precondition for the gate: live GCP credentials and a standing canonical depot must be provisioned — the gate is not free-standing. The ~100-minute skirmish cook is a milestone, not a per-commit cost; the heat pays one full cook, at the gate.

## Minting — decided

The open naming items are settled here: spine and bodies in the `rbldX_` family (legend in Shape); host modules decompose `rbXY_ → rbXYZ_`, the old name becoming a container and stopping naming a thing (terminal exclusivity preserved); the composition contract lives in an RBSCJ section, not a new spec letter and not RBSL (the Lode-domain spec). Precise sub-letters are not load-bearing — uniqueness and consistency are; the full allocation is recorded at the mint pace so the explosion paces inherit stable, documented targets — the explosion paces may refine cluster boundaries (which function lands in which file) but not the allocated letters. The shared cloud-step home (for vouch-push and the composed snippets) breaks the rbgj family-letter scheme since a shared step belongs to no single family — minted at the library pace.

## Capstone — harvest CBG (Cloud Build Guide)

A required output of this heat, produced at landing by distilling the patterns this refactor establishes — not a speculative pre-write. CBG fills the same slot for the Cloud-Build-container stack that WSG fills for the ssh-to-Windows stack: a foreign-environment sibling to BCG, sharing its philosophy (crash-fast, no silent failures, load-bearing complexity, interface contamination) but diverging entirely on mechanics. CBG operates at single-script precision — it governs the inside of one cloud step, NOT component wiring or architecture; those lessons live in the spec and this paddock.

Contents are the cloud dialect's per-script discipline: crash-fast in the cloud idiom, the metadata-token snippet, JSON-construction rules (when hand-rolled printf is acceptable vs when jq is required), substitution naming and double-dollar escaping, idempotency under Cloud Build retry, the pure-logic/network-op split that makes a step unit-testable, and the /workspace constraints — foremost what must NOT go in it (secrets; the invariant recorded in RBSCB). CBG points at the executable contract and validator rather than restating them.

The guide is only as good as the code it distills, so the spine, library, and per-kind bodies are written as its exemplars — quality now is what makes CBG worth harvesting. Name CBG chosen; confirm against the BCG/RCG/WSG sibling set at mint, and read WSG before authoring so the sibling framing rests on the document, not its label.

## Done

Bole ensconce rides one host spine and one shared cloud step-library with explicit per-step contracts; inscribe and enshrine are left as reference (not migrated); the host modules are decomposed into small, well-prefixed files (relocation only, no logic edits); the canonical token and the skopeo copy/digest block are composed snippets authored once; vouch-push is unified across Lode and hallmark-verify; the dispatch validator is in place with its own tests; full skirmish plus service pass with artifacts and provenance verdicts identical to pre-refactor (provenance-insensitivity confirmed); no new Lode kinds exist; the rationales are recorded in the Cloud Build spec — posture in RBSCB, the composition/contract decision in an RBSCJ section; and CBG (the Cloud Build Guide) is harvested from the patterns this heat establishes — see Capstone.

## References

- RBSCB CloudBuildPosture — skopeo-token/credential-helper and capture-duplication posture; the /workspace-no-secrets invariant.
- RBSCJ CloudBuildJson — JSON-composition trade study; home for the spine/contract decision (currently trigger-era stale — refresh when touched).
- WSG WindowsScriptingGuide — the foreign-environment sibling guide to BCG (ssh-to-Windows stack); structural precedent for CBG, to be read before CBG is authored.
- The bole pilot, the reliquary/inscribe and enshrine predecessors, and the cloud step family.

## Paces

### skirmish-centos-baseline (₢BXAAL) [complete]

**[260603-1533] complete**

## Character
Operator-driven, heavyweight — manual host setup, long cook.

## Goal
Capture the pre-refactor green baseline on the centos host (the platform the milestone gate re-runs on), so "behavior-preserving" has a real before/after to diff against.

## Cinched (paddock)
- Operator sets up the host: live GCP creds + standing canonical depot. Run under tmux so the ~100-min cook survives disconnect.
- Same suites the gate uses: full skirmish + service lode-lifecycle.

## Done
- skirmish + service green on centos before any refactor; artifacts and provenance verdicts recorded as the baseline the gate compares against.

**[260603-0605] rough**

## Character
Operator-driven, heavyweight — manual host setup, long cook.

## Goal
Capture the pre-refactor green baseline on the centos host (the platform the milestone gate re-runs on), so "behavior-preserving" has a real before/after to diff against.

## Cinched (paddock)
- Operator sets up the host: live GCP creds + standing canonical depot. Run under tmux so the ~100-min cook survives disconnect.
- Same suites the gate uses: full skirmish + service lode-lifecycle.

## Done
- skirmish + service green on centos before any refactor; artifacts and provenance verdicts recorded as the baseline the gate compares against.

### mint-and-step-contract (₢BXAAA) [complete]

**[260604-1016] complete**

## Character
Mechanical registry transcription; gates the explosion paces.

## Goal
Record the host-module submint allocation in the rbk acronyms map (per the paddock legend) so the explosion paces have stable target names. Each old name becomes a container (terminal exclusivity). The shared-cloud-step home and the contract FORMAT are NOT minted here — they land where consumed.

## Cinched (paddock)
- Submint pattern rbXY_ → rbXYZ_; the old name becomes a container; precise sub-letters are not load-bearing — uniqueness and consistency are. All sub-letters (rbld, rbfl, rbfc) are allocated here — that is the gate's purpose; the explosion paces honor the recorded letters and may refine only cluster boundaries (which function lands in which file).
- The contract's home is an RBSCJ section (not a new spec letter, not RBSL); its requires/provides FORMAT is designed where consumed — the spine pace and the library pace — not here.
- The shared cloud-step home is minted at the library pace, not here.

## Done
- The submint allocation is fully recorded in the rbk acronyms map per the paddock legend — old names become containers and each planned `rbXYZ_` child entered with a brief description — so the explosion paces have stable, documented targets.

**[260604-0920] rough**

## Character
Mechanical registry transcription; gates the explosion paces.

## Goal
Record the host-module submint allocation in the rbk acronyms map (per the paddock legend) so the explosion paces have stable target names. Each old name becomes a container (terminal exclusivity). The shared-cloud-step home and the contract FORMAT are NOT minted here — they land where consumed.

## Cinched (paddock)
- Submint pattern rbXY_ → rbXYZ_; the old name becomes a container; precise sub-letters are not load-bearing — uniqueness and consistency are. All sub-letters (rbld, rbfl, rbfc) are allocated here — that is the gate's purpose; the explosion paces honor the recorded letters and may refine only cluster boundaries (which function lands in which file).
- The contract's home is an RBSCJ section (not a new spec letter, not RBSL); its requires/provides FORMAT is designed where consumed — the spine pace and the library pace — not here.
- The shared cloud-step home is minted at the library pace, not here.

## Done
- The submint allocation is fully recorded in the rbk acronyms map per the paddock legend — old names become containers and each planned `rbXYZ_` child entered with a brief description — so the explosion paces have stable, documented targets.

**[260603-1116] rough**

## Character
Mechanical registry transcription; gates the explosion paces.

## Goal
Record the host-module submint allocation in the rbk acronyms map (per the paddock legend) so the explosion paces have stable target names. Each old name flips to a container (terminal exclusivity). The shared-cloud-step home and the contract FORMAT are NOT minted here — they land where consumed.

## Cinched (paddock)
- Submint pattern rbXY_ → rbXYZ_; the old name becomes a container; precise sub-letters are not load-bearing — uniqueness and consistency are. rbld is definitive; the exact intra-rbfc/rbfl split firms at the foundry explosion.
- The contract's home is an RBSCJ section (not a new spec letter, not RBSL); its requires/provides FORMAT is designed where consumed — the spine pace and the library pace — not here.
- The shared cloud-step home is minted at the library pace, not here.

## Done
- The submint allocation is fully recorded in the rbk acronyms map per the paddock legend — old names flipped to containers and each planned `rbXYZ_` child entered with a brief description — so the explosion paces have stable, documented targets.

**[260603-1047] rough**

## Character
Mechanical registry transcription; gates the explosion paces.

## Goal
Record the host-module submint allocation in the rbk acronyms map (per the paddock legend) so the explosion paces have stable target names. Each old name flips to a container (terminal exclusivity). The shared-cloud-step home and the contract FORMAT are NOT minted here — they land where consumed.

## Cinched (paddock)
- Submint pattern rbXY_ → rbXYZ_; the old name becomes a container; precise sub-letters are not load-bearing — uniqueness and consistency are. rbld is definitive; the exact intra-rbfc/rbfl split firms at the foundry explosion.
- The contract's home is an RBSCJ section (not a new spec letter, not RBSL); its requires/provides FORMAT is designed where consumed — the spine pace and the library pace — not here.
- The shared cloud-step home is minted at the library pace, not here.

## Done
- The submint allocation is recorded in the rbk acronyms map per the paddock legend, with old names flipped to containers; the explosion paces have stable targets.

**[260603-0542] rough**

## Character
Design — judgment; gates all.

## Goal
Mint the stable names and site the contract: the submint allocation (per the paddock legend), the shared-cloud-step namespace, and the contract's spec home.

## Cinched (paddock)
- The contract is SITED in an RBSCJ section here; its requires/provides FORMAT is designed with the spine (AAD) and the step declarations (AAE) where it is consumed — not minted standalone.
- Submint pattern rbXY_ → rbXYZ_; precise letters are not load-bearing.

## Done
- Submint allocation recorded in the rbk acronyms map; the shared-cloud-step home minted (breaking the rbgj family-letter scheme); RBSCJ named as the contract's home.

**[260603-0515] rough**

## Character
Design conversation requiring judgment.

## Goal
Finalize the cross-cutting pieces the rest of the heat composes against: the step requires/provides contract, its spec home, and the shared-cloud-step namespace.

## Cinched (paddock)
- Contract lives in an RBSCJ section — not RBSL, not a new spec letter.
- Submint pattern rbXY_ → rbXYZ_ and the rbldX_ legend are set; precise letters are not load-bearing — uniqueness and consistency are.

## Done
- The step contract is specified: each step declares requires/provides over /workspace registers + substitutions + buildStepOutputs, in a form the validator can check.
- The shared-cloud-step home (for vouch-push + composed snippets) is minted, breaking the rbgj family-letter scheme deliberately.
- The submint allocation is recorded where future minters look (the rbk acronyms map).

### provenance-insensitivity-spike (₢BXAAJ) [complete]

**[260604-1026] complete**

## Character
Cheap de-risk of the heat's load-bearing assumption — gates the spine/library investment.

## Goal
Confirm SLSA provenance survives cloud-step restructuring BEFORE investing in the spine and library, by reading the verifier (rbgjv02-verify-provenance.py).

## Cinched (paddock)
- The behavior-preserving rail rests on this; if it is false, the spine/library approach changes — so it gates them, rather than being discovered at the final gate.

## Found (pre-impl review)
- Verdict reached, NO cook needed: the verifier gates only on DSSE keyid, openssl signature validity, and builder.id == GoogleHostedWorker — all builder-identity. The one recipe-ish field (buildDefinition.buildType) is read but never asserted; nothing reads steps/buildConfig/externalParameters. Restructuring steps leaves the verdict invariant; the conjure read is decisive.
- This review already performed the read, so mounting AAJ is confirm-and-record, not fresh investigation. Pale caveat: this vouches what our verifier asserts, not Google's upstream signing — but we gate on identity+signature, so our verdict holds regardless.

## Done
- The verdict (provenance insensitive to step restructuring) is recorded so the spine and library proceed on confirmed ground.

**[260603-1136] rough**

## Character
Cheap de-risk of the heat's load-bearing assumption — gates the spine/library investment.

## Goal
Confirm SLSA provenance survives cloud-step restructuring BEFORE investing in the spine and library, by reading the verifier (rbgjv02-verify-provenance.py).

## Cinched (paddock)
- The behavior-preserving rail rests on this; if it is false, the spine/library approach changes — so it gates them, rather than being discovered at the final gate.

## Found (pre-impl review)
- Verdict reached, NO cook needed: the verifier gates only on DSSE keyid, openssl signature validity, and builder.id == GoogleHostedWorker — all builder-identity. The one recipe-ish field (buildDefinition.buildType) is read but never asserted; nothing reads steps/buildConfig/externalParameters. Restructuring steps leaves the verdict invariant; the conjure read is decisive.
- This review already performed the read, so mounting AAJ is confirm-and-record, not fresh investigation. Pale caveat: this vouches what our verifier asserts, not Google's upstream signing — but we gate on identity+signature, so our verdict holds regardless.

## Done
- The verdict (provenance insensitive to step restructuring) is recorded so the spine and library proceed on confirmed ground.

**[260603-0543] rough**

## Character
Cheap de-risk of the heat's load-bearing assumption — gates the spine/library investment.

## Goal
Confirm SLSA provenance survives cloud-step restructuring BEFORE investing in the spine and library. Read the verifier (rbgjv02-verify-provenance.py) to confirm it keys on build inputs/outputs, not step bytes or order; one targeted cook (restructure a step trivially, compare the verdict) only if the code is ambiguous.

## Cinched (paddock)
- The behavior-preserving rail rests on this; if it is false, the spine/library approach changes — so it gates them, rather than being discovered at the final gate.

## Done
- A clear verdict that provenance is insensitive to step restructuring (verifier analysis, plus one confirming cook if needed), recorded so the spine and library proceed on confirmed ground — or, if sensitive, the approach is revisited before any investment.

### lode-module-explosion (₢BXAAB) [complete]

**[260604-1338] complete**

## Character
Mechanical, agent-choreographed. Relocation only — no smuggling.

## Goal
Extract the boundary-independent slices of rbld into their own files — lifecycle REST (rbldl_), kindle/shared-consts (rbldk_) — plus the capture-relevant Foundry build-primitives (rbfcb_). The spine/body carve (rblds_/rbldb_) is NOT here: that boundary is designed in the spine pace, so it does not get pre-split mechanically.

## Cinched (paddock)
- Relocation only: zero logic edits ride along; lode-lifecycle is the oracle.

## Found (pre-impl review)
- Bound by the paddock's Relocation discipline (kindle atom on the entry file, verbatim globals, flat placement, forced cross-language call-site rewrites).
- rbfcb_ specifically keeps sourcing the kindle for the ZRBFC_ constants its primitives read — pure relocation; making any primitive kindle-independent for unit-testing is a separate anneal, not this move.

## Execution
Per-file sonnet agents (each handed the old module + its slice); review the set, then remove relocated content; haiku for call-site rewrites.

## Done
- rbldl_, rbldk_, rbfcb_ exist and match their acronym entries; their content is removed from the monoliths; all call sites updated, and sibling context docs that named the old module (tabtarget / theurge context and the like) swept so nothing points at a deleted file; lode-lifecycle green.

**[260603-1145] rough**

## Character
Mechanical, agent-choreographed. Relocation only — no smuggling.

## Goal
Extract the boundary-independent slices of rbld into their own files — lifecycle REST (rbldl_), kindle/shared-consts (rbldk_) — plus the capture-relevant Foundry build-primitives (rbfcb_). The spine/body carve (rblds_/rbldb_) is NOT here: that boundary is designed in the spine pace, so it does not get pre-split mechanically.

## Cinched (paddock)
- Relocation only: zero logic edits ride along; lode-lifecycle is the oracle.

## Found (pre-impl review)
- Bound by the paddock's Relocation discipline (kindle atom on the entry file, verbatim globals, flat placement, forced cross-language call-site rewrites).
- rbfcb_ specifically keeps sourcing the kindle for the ZRBFC_ constants its primitives read — pure relocation; making any primitive kindle-independent for unit-testing is a separate anneal, not this move.

## Execution
Per-file sonnet agents (each handed the old module + its slice); review the set, then remove relocated content; haiku for call-site rewrites.

## Done
- rbldl_, rbldk_, rbfcb_ exist and match their acronym entries; their content is removed from the monoliths; all call sites updated, and sibling context docs that named the old module (tabtarget / theurge context and the like) swept so nothing points at a deleted file; lode-lifecycle green.

**[260603-1138] rough**

## Character
Mechanical, agent-choreographed. Relocation only — no smuggling.

## Goal
Extract the boundary-independent slices of rbld into their own files — lifecycle REST (rbldl_), kindle/shared-consts (rbldk_) — plus the capture-relevant Foundry build-primitives (rbfcb_). The spine/body carve (rblds_/rbldb_) is NOT here: that boundary is designed in the spine pace, so it does not get pre-split mechanically.

## Cinched (paddock)
- Relocation only: zero logic edits ride along; lode-lifecycle is the oracle.

## Found (pre-impl review)
- Kindle/sentinel/guard is one indivisible atom — do NOT distribute it: rbldk_ is the entry file holding rbld's single guard + kindle and sourcing its guard-free siblings; cluster files stay flat in Tools/rbk/ (BASH_SOURCE-relative paths break if nested).
- rbfcb_ keeps sourcing the kindle for the ZRBFC_ constants its primitives read — pure relocation; making any primitive kindle-independent for unit-testing is a separate anneal, not this move.

## Execution
Per-file sonnet agents (each handed the old module + its slice); review the set, then remove relocated content; haiku for call-site rewrites.

## Done
- rbldl_, rbldk_, rbfcb_ exist and match their acronym entries; their content is removed from the monoliths; all call sites updated, and sibling context docs that named the old module (tabtarget / theurge context and the like) swept so nothing points at a deleted file; lode-lifecycle green.

**[260603-1136] rough**

## Character
Mechanical, agent-choreographed. Relocation only — no smuggling.

## Goal
Extract the boundary-independent slices of rbld into their own files — lifecycle REST (rbldl_), kindle/shared-consts (rbldk_) — plus the capture-relevant Foundry build-primitives (rbfcb_). The spine/body carve (rblds_/rbldb_) is NOT here: that boundary is designed in the spine pace, so it does not get pre-split mechanically.

## Cinched (paddock)
- Relocation only: zero logic edits ride along; lode-lifecycle is the oracle.

## Found (pre-impl review)
- Kindle/sentinel/guard is one indivisible atom — do NOT distribute it: rbldk_ is the entry file holding rbld's single guard + kindle and sourcing its guard-free siblings; cluster files stay flat in Tools/rbk/ (BASH_SOURCE-relative paths break if nested).
- rbfcb_ keeps sourcing the kindle for the ZRBFC_ constants its primitives read — pure relocation; making any primitive kindle-independent for unit-testing is a separate anneal, not this move.

## Execution
Per-file sonnet agents (each handed the old module + its slice); review the set, then remove relocated content; haiku for call-site rewrites.

## Done
- rbldl_, rbldk_, rbfcb_ exist; their content is removed from the monoliths; all call sites updated; lode-lifecycle green.

**[260603-1116] rough**

## Character
Mechanical, agent-choreographed. Relocation only — no smuggling.

## Goal
Extract the boundary-independent slices of rbld into their own files — lifecycle REST (rbldl_), kindle/shared-consts (rbldk_) — plus the capture-relevant Foundry build-primitives (rbfcb_). The spine/body carve (rblds_/rbldb_) is NOT here: that boundary is designed in the spine pace, so it does not get pre-split mechanically.

## Cinched (paddock)
- Relocation only: zero logic edits ride along; lode-lifecycle is the oracle.

## Execution
Per-file sonnet agents (each handed the old module + its slice); review the set, then remove relocated content; haiku for call-site rewrites.

## Done
- rbldl_, rbldk_, rbfcb_ exist and match their acronym entries; their content is removed from the monoliths; all call sites updated, and sibling context docs that named the old module (tabtarget / theurge context and the like) swept so nothing points at a deleted file; lode-lifecycle green.

**[260603-0542] rough**

## Character
Mechanical, agent-choreographed. Relocation only — no smuggling.

## Goal
Extract the boundary-independent slices of rbld into their own files — lifecycle REST (rbldl_), kindle/shared-consts (rbldk_) — plus the capture-relevant Foundry build-primitives (rbfcb_). The spine/body carve (rblds_/rbldb_) is NOT here: that boundary is designed in the spine pace, so it does not get pre-split mechanically.

## Cinched (paddock)
- Relocation only: zero logic edits ride along; lode-lifecycle is the oracle.

## Execution
Per-file sonnet agents (each handed the old module + its slice); review the set, then remove relocated content; haiku for call-site rewrites.

## Done
- rbldl_, rbldk_, rbfcb_ exist; their content is removed from the monoliths; all call sites updated; lode-lifecycle green.

**[260603-0516] rough**

## Character
Mechanical, agent-choreographed. Relocation only — no smuggling.

## Goal
Decompose rbld_Lode.sh into the rbldX_ files per the paddock legend, so bole's machinery lives in small, well-prefixed files — and extract the capture-relevant Foundry build-primitives the spine rides into rbfcb_.

## Cinched (paddock)
- Relocation only: zero logic edits ride along; lode-lifecycle is the oracle.
- Legend: rblds_ spine, rbldb_ bole body, rbldl_ lifecycle REST, rbldk_ kindle/shared; reserved kind-letters allocated but no files.

## Execution
One sonnet agent per new file (each handed the old module + its slice); review the full set, then delete the old; haiku for call-site rewrites.

## Done
- rbld_Lode.sh is decomposed into the legend files and the rbfcb_ build-primitives are extracted; old content removed after review.
- All call sites updated; lode-lifecycle green with identical behavior.

### foundry-module-explosion (₢BXAAC) [complete]

**[260604-1448] complete**

## Character
Mechanical, broad, agent-choreographed. Relocation only — no smuggling.

## Goal
Decompose the rest of rbfc and rbfl into their rbfcX_/rbflX_ clusters per the paddock legend. rbfcb_BuildHost.sh already exists (the build-host primitives, sourced by rbfc_FoundryCore.sh); this pace finishes the two monoliths. Reaches made-side files (step-assembly, plumb); acceptable as pure relocation.

## Cinched (paddock)
- Relocation only: a move carries no logic change.
- Terminal exclusivity is file-level: dispatch/helper functions keep the zrbfc_/zrbfl_ container prefix, so there are NO function-call rewrites — only source-line repoints where the deleted monoliths were named.
- The multifacet CLI homes on the 0-top — rbfc0_cli.sh / rbfl0_cli.sh (zrbfc_furnish / zrbfl_furnish), NOT the kindle child. The kindle-entry (rbfck_/rbflk_) is not the CLI home.
- Inner-loop oracle is shellcheck + the fast/service fixtures plus one conjure path; the FULL skirmish is NOT re-run here — the milestone gate carries it.

## Found (pre-impl review)
- Bound by the Relocation discipline — the rbfc/rbfl kindle atoms land in their entry files (rbfck_/rbflk_); the z_rbfc_tool_* globals move verbatim.
- rbfc_FoundryCore.sh currently sources rbfcb_BuildHost.sh; when this pace deletes the monolith, that `source rbfcb_` rehomes into the new rbfc entry (rbfck_), which the 0-top CLI then sources.
- The forced sweep is source-line repoints only: the Rust test driver (rbtdrf_fast.rs sources rbfc_FoundryCore.sh by name to drive the kindle-independent zrbfc_native_path_capture — repoint it to whatever then provides rbfcb_, e.g. rbfcb_BuildHost.sh directly), the zipper enrollments, and sibling context docs (acronym map, tabtarget / theurge). Function calls are unchanged.

## Execution
Per-file sonnet agents; review the set, then delete the old monoliths; the source-line repoints are mechanical (haiku).

## Done
- rbfc and rbfl decomposed into their clusters, matching their acronym entries; old monoliths deleted and the cli partners re-homed onto the 0-top (rbfc0_cli/rbfl0_cli) after review, so no bare rbfc_/rbfl_ file survives — the terminal-exclusivity violation this heat opens is closed; the deleted-monolith source-lines (Rust driver, zipper) and context docs are repointed; the targeted fixtures green.

**[260604-1113] rough**

## Character
Mechanical, broad, agent-choreographed. Relocation only — no smuggling.

## Goal
Decompose the rest of rbfc and rbfl into their rbfcX_/rbflX_ clusters per the paddock legend. rbfcb_BuildHost.sh already exists (the build-host primitives, sourced by rbfc_FoundryCore.sh); this pace finishes the two monoliths. Reaches made-side files (step-assembly, plumb); acceptable as pure relocation.

## Cinched (paddock)
- Relocation only: a move carries no logic change.
- Terminal exclusivity is file-level: dispatch/helper functions keep the zrbfc_/zrbfl_ container prefix, so there are NO function-call rewrites — only source-line repoints where the deleted monoliths were named.
- The multifacet CLI homes on the 0-top — rbfc0_cli.sh / rbfl0_cli.sh (zrbfc_furnish / zrbfl_furnish), NOT the kindle child. The kindle-entry (rbfck_/rbflk_) is not the CLI home.
- Inner-loop oracle is shellcheck + the fast/service fixtures plus one conjure path; the FULL skirmish is NOT re-run here — the milestone gate carries it.

## Found (pre-impl review)
- Bound by the Relocation discipline — the rbfc/rbfl kindle atoms land in their entry files (rbfck_/rbflk_); the z_rbfc_tool_* globals move verbatim.
- rbfc_FoundryCore.sh currently sources rbfcb_BuildHost.sh; when this pace deletes the monolith, that `source rbfcb_` rehomes into the new rbfc entry (rbfck_), which the 0-top CLI then sources.
- The forced sweep is source-line repoints only: the Rust test driver (rbtdrf_fast.rs sources rbfc_FoundryCore.sh by name to drive the kindle-independent zrbfc_native_path_capture — repoint it to whatever then provides rbfcb_, e.g. rbfcb_BuildHost.sh directly), the zipper enrollments, and sibling context docs (acronym map, tabtarget / theurge). Function calls are unchanged.

## Execution
Per-file sonnet agents; review the set, then delete the old monoliths; the source-line repoints are mechanical (haiku).

## Done
- rbfc and rbfl decomposed into their clusters, matching their acronym entries; old monoliths deleted and the cli partners re-homed onto the 0-top (rbfc0_cli/rbfl0_cli) after review, so no bare rbfc_/rbfl_ file survives — the terminal-exclusivity violation this heat opens is closed; the deleted-monolith source-lines (Rust driver, zipper) and context docs are repointed; the targeted fixtures green.

**[260604-1001] rough**

## Character
Mechanical, broad, agent-choreographed. Relocation only — no smuggling.

## Goal
Decompose the rest of rbfc and rbfl into their rbfcX_/rbflX_ clusters per the paddock legend (rbfcb_ already extracted in the lode-extraction pace). Reaches made-side files (step-assembly, plumb); acceptable as pure relocation.

## Cinched (paddock)
- Relocation only: a move carries no logic change.
- Inner-loop oracle is shellcheck + the fast/service fixtures plus one conjure path; the FULL skirmish is NOT re-run here — the milestone gate carries it, so the heat pays one full cook, not two.

## Found (pre-impl review)
- Bound by the paddock's Relocation discipline — the rbfc and rbfl kindle atoms (→ rbfck_ / rbflk_ entry files), the verbatim z_rbfc_tool_* globals, and the cross-language call-site sweep (incl. the Rust driver) all land here.
- Terminal-exclusivity closure: once rbfc/rbfl have children they may name nothing, so EVERY bare rbfc_/rbfl_ name must retire here — not just the deleted monoliths but the cli partners. Re-home the furnish dispatch onto the entry child (rbfck_cli / rbflk_cli, zrbfck_/zrbflk_furnish) and reroute the zipper enrollments. Precedent: the rbh0 handbook decomposition rides its clis on rbho0_/rbhp0_, never a bare rbh_cli.

## Execution
Per-file sonnet agents; review the set, then delete the old files; haiku for call-site rewrites.

## Done
- rbfc and rbfl decomposed into their clusters, matching their acronym entries; old monoliths deleted and the cli partners re-homed (rbfck_cli/rbflk_cli) after review, so no bare rbfc_/rbfl_ file or function survives — the terminal-exclusivity violation this heat opens is closed; all call sites and zipper enrollments updated, and sibling context docs naming the old modules swept so nothing points at a deleted file; the targeted fixtures green.

**[260603-1145] rough**

## Character
Mechanical, broad, agent-choreographed. Relocation only — no smuggling.

## Goal
Decompose the rest of rbfc and rbfl into their rbfcX_/rbflX_ clusters per the paddock legend (rbfcb_ already extracted in the lode-extraction pace). Reaches made-side files (step-assembly, plumb); acceptable as pure relocation.

## Cinched (paddock)
- Relocation only: a move carries no logic change.
- Inner-loop oracle is shellcheck + the fast/service fixtures plus one conjure path; the FULL skirmish is NOT re-run here — the milestone gate carries it, so the heat pays one full cook, not two.

## Found (pre-impl review)
- Bound by the paddock's Relocation discipline — the rbfc kindle atom (→ rbfck_ entry), the verbatim z_rbfc_tool_* globals, and the cross-language call-site sweep (incl. the Rust driver) all land here.

## Execution
Per-file sonnet agents; review the set, then delete the old files; haiku for call-site rewrites.

## Done
- rbfc and rbfl decomposed into their clusters, matching their acronym entries; old files deleted after review; all call sites updated, and sibling context docs naming the old modules swept so nothing points at a deleted file; the targeted fixtures green.

**[260603-1138] rough**

## Character
Mechanical, broad, agent-choreographed. Relocation only — no smuggling.

## Goal
Decompose the rest of rbfc and rbfl into their rbfcX_/rbflX_ clusters per the paddock legend (rbfcb_ already extracted in the lode-extraction pace). Reaches made-side files (step-assembly, plumb); acceptable as pure relocation.

## Cinched (paddock)
- Relocation only: a move carries no logic change.
- Inner-loop oracle is shellcheck + the fast/service fixtures plus one conjure path; the FULL skirmish is NOT re-run here — the milestone gate carries it, so the heat pays one full cook, not two.

## Found (pre-impl review)
- Kindle/sentinel/guard is one indivisible atom: rbfck_ becomes the entry file holding rbfc's single guard + whole kindle and sourcing the guard-free body clusters; flat in Tools/rbk/ only.
- Copy the leaked tool-image globals (z_rbfc_tool_*) verbatim — global-by-bare-assignment, read cross-module by rbfd; adding local/readonly/declare while moving them silently breaks step assembly.
- Call-site rewrites are forced and cross-language: terminal exclusivity deletes rbfc_FoundryCore.sh, so all source sites AND the Rust test driver (rbtdrf_fast.rs, sources it by name) repoint; rbfd reads rbfc internals, so the entry file must keep sourcing the full cluster set.

## Execution
Per-file sonnet agents; review the set, then delete the old files; haiku for call-site rewrites.

## Done
- rbfc and rbfl decomposed into their clusters, matching their acronym entries; old files deleted after review; all call sites updated, and sibling context docs naming the old modules swept so nothing points at a deleted file; the targeted fixtures green.

**[260603-1136] rough**

## Character
Mechanical, broad, agent-choreographed. Relocation only — no smuggling.

## Goal
Decompose the rest of rbfc and rbfl into their rbfcX_/rbflX_ clusters per the paddock legend (rbfcb_ already extracted in the lode-extraction pace). Reaches made-side files (step-assembly, plumb); acceptable as pure relocation.

## Cinched (paddock)
- Relocation only: a move carries no logic change.
- Inner-loop oracle is shellcheck + the fast/service fixtures plus one conjure path; the FULL skirmish is NOT re-run here — the milestone gate carries it.

## Found (pre-impl review)
- Kindle/sentinel/guard is one indivisible atom: rbfck_ becomes the entry file holding rbfc's single guard + whole kindle and sourcing the guard-free body clusters; flat in Tools/rbk/ only.
- Copy the leaked tool-image globals (z_rbfc_tool_*) verbatim — global-by-bare-assignment, read cross-module by rbfd; adding local/readonly/declare while moving them silently breaks step assembly.
- Call-site rewrites are forced and cross-language: terminal exclusivity deletes rbfc_FoundryCore.sh, so all source sites AND the Rust test driver (rbtdrf_fast.rs, sources it by name) repoint; rbfd reads rbfc internals, so the entry file must keep sourcing the full cluster set.

## Execution
Per-file sonnet agents; review the set, then delete the old files; haiku for call-site rewrites.

## Done
- rbfc and rbfl decomposed into their clusters; old files deleted after review; all call sites updated; the targeted fixtures green.

**[260603-1116] rough**

## Character
Mechanical, broad, agent-choreographed. Relocation only — no smuggling.

## Goal
Decompose the rest of rbfc and rbfl into their rbfcX_/rbflX_ clusters per the paddock legend (rbfcb_ already extracted in the lode-extraction pace). Reaches made-side files (step-assembly, plumb); acceptable as pure relocation.

## Cinched (paddock)
- Relocation only: a move carries no logic change.
- Inner-loop oracle is shellcheck + the fast/service fixtures plus one conjure path; the FULL skirmish is NOT re-run here — the milestone gate carries it, so the heat pays one full cook, not two.

## Execution
Per-file sonnet agents; review the set, then delete the old files; haiku for call-site rewrites.

## Done
- rbfc and rbfl decomposed into their clusters, matching their acronym entries; old files deleted after review; all call sites updated, and sibling context docs naming the old modules swept so nothing points at a deleted file; the targeted fixtures green.

**[260603-0542] rough**

## Character
Mechanical, broad, agent-choreographed. Relocation only — no smuggling.

## Goal
Decompose the rest of rbfc and rbfl into their rbfcX_/rbflX_ clusters per the paddock legend (rbfcb_ already extracted in the lode-extraction pace). Reaches made-side files (step-assembly, plumb); acceptable as pure relocation.

## Cinched (paddock)
- Relocation only: a move carries no logic change.
- Inner-loop oracle is shellcheck + the fast/service fixtures plus one conjure path; the FULL skirmish is NOT re-run here — the milestone gate carries it, so the heat pays one full cook, not two.

## Execution
Per-file sonnet agents; review the set, then delete the old files; haiku for call-site rewrites.

## Done
- rbfc and rbfl decomposed into their clusters; old files deleted after review; all call sites updated; the targeted fixtures green.

**[260603-0516] rough**

## Character
Mechanical, broad, agent-choreographed. Relocation only — no smuggling.

## Goal
Decompose rbfc_FoundryCore.sh and rbfl_FoundryLedger.sh into their rbfcX_/rbflX_ clusters per the paddock legend. Reaches made-side files (step-assembly, plumb); acceptable as pure relocation.

## Cinched (paddock)
- Relocation only: a move carries no logic change; full skirmish is the oracle.
- rbfc clusters rbfcv_/rbfca_/rbfcg_/rbfcp_/rbfck_ (rbfcb_ already extracted in lode explosion); rbfl by gesture (inscribe / yoke / delete / inventory / wrest). Exact intra-file split firms here.

## Execution
One sonnet agent per new file; review the set, then delete the old; haiku for call-site rewrites.

## Done
- rbfc and rbfl decomposed; old files deleted after review; all call sites updated.
- Full skirmish green with verdicts identical to pre-refactor.

### capture-spine-data-driven (₢BXAAD) [complete]

**[260604-2027] complete**

## Character
The heat's core — design plus implementation.

## Goal
Design the spine/body boundary and the recipe + substitutions contract shape, then carve rblds_ (spine) out of rbldb_ (bole body) accordingly and make the spine data-driven; bole rides it.

## Cinched (paddock)
- Registration-agnostic: the spine takes the recipe as data — no per-kind branch, no kind-registration or discovery mechanism; registering kinds is left to the consumer, not baked into the spine.
- The spine/body boundary is decided HERE, not pre-split mechanically; the requires/provides contract FORMAT is designed here against the capture domain's known axes (N-member envelopes, tool-variety, opaque-blob payloads) and recorded in the contract's RBSCJ section.

## Found (pre-impl review)
- The Lode family's terminal-exclusivity closure is already done: rbld_Lode.sh is gone (renamed to rbldb_Bole.sh, which holds the full ensconce path); the CLI is the 0-top rbld0_cli.sh; rbldk_ (kindle-entry) and rbldl_ (lifecycle) exist; the zipper is rerouted. This pace does NOT touch CLI/closure — it carves the spine out of rbldb_.
- A recipe row is (script_path, builder_image, id, entrypoint) handed pre-resolved by the body — NOT bare step ids the spine resolves (resolving would re-import kind knowledge). The substitutions blob stays opaque to the spine: it slots into an identical jq envelope, reads no key.
- Extract cut-line: the spine decodes buildStepOutputs[N] to a file (N the one generic param); the per-slot envelope / stamp / buf_write_fact_multi loop is body. Today's zrbld_ensconce_extract (now in rbldb_Bole.sh) threads both — this is a real carve, not relocation.
- The poll ceiling is kind-flavored (ensconce borrows the enshrine ceiling); the body/recipe passes it — the spine does not own the ceiling table.

## Done
- rblds_ carved out of rbldb_ along the designed boundary; bole ensconce composes through the spine via recipe + substitutions, with no kind knowledge in the spine; behavior-preserving (lode-lifecycle green, provenance verdicts identical). Lode-family terminal exclusivity was already closed before this pace.

**[260604-1113] rough**

## Character
The heat's core — design plus implementation.

## Goal
Design the spine/body boundary and the recipe + substitutions contract shape, then carve rblds_ (spine) out of rbldb_ (bole body) accordingly and make the spine data-driven; bole rides it.

## Cinched (paddock)
- Registration-agnostic: the spine takes the recipe as data — no per-kind branch, no kind-registration or discovery mechanism; registering kinds is left to the consumer, not baked into the spine.
- The spine/body boundary is decided HERE, not pre-split mechanically; the requires/provides contract FORMAT is designed here against the capture domain's known axes (N-member envelopes, tool-variety, opaque-blob payloads) and recorded in the contract's RBSCJ section.

## Found (pre-impl review)
- The Lode family's terminal-exclusivity closure is already done: rbld_Lode.sh is gone (renamed to rbldb_Bole.sh, which holds the full ensconce path); the CLI is the 0-top rbld0_cli.sh; rbldk_ (kindle-entry) and rbldl_ (lifecycle) exist; the zipper is rerouted. This pace does NOT touch CLI/closure — it carves the spine out of rbldb_.
- A recipe row is (script_path, builder_image, id, entrypoint) handed pre-resolved by the body — NOT bare step ids the spine resolves (resolving would re-import kind knowledge). The substitutions blob stays opaque to the spine: it slots into an identical jq envelope, reads no key.
- Extract cut-line: the spine decodes buildStepOutputs[N] to a file (N the one generic param); the per-slot envelope / stamp / buf_write_fact_multi loop is body. Today's zrbld_ensconce_extract (now in rbldb_Bole.sh) threads both — this is a real carve, not relocation.
- The poll ceiling is kind-flavored (ensconce borrows the enshrine ceiling); the body/recipe passes it — the spine does not own the ceiling table.

## Done
- rblds_ carved out of rbldb_ along the designed boundary; bole ensconce composes through the spine via recipe + substitutions, with no kind knowledge in the spine; behavior-preserving (lode-lifecycle green, provenance verdicts identical). Lode-family terminal exclusivity was already closed before this pace.

**[260604-1014] rough**

## Character
The heat's core — design plus implementation.

## Goal
Design the spine/body boundary and the recipe + substitutions contract shape, then carve rblds_ (spine) and rbldb_ (bole body) accordingly and make the spine data-driven; bole rides it.

## Cinched (paddock)
- Registration-agnostic: the spine takes the recipe as data — no per-kind branch, no kind-registration or discovery mechanism; registering kinds is left to the consumer, not baked into the spine.
- The spine/body boundary is decided HERE, not pre-split mechanically at the lode explosion; the requires/provides contract FORMAT is designed here against the capture domain's known axes (N-member envelopes, tool-variety, opaque-blob payloads) and recorded in the contract's RBSCJ section.

## Found (pre-impl review)
- A recipe row is (script_path, builder_image, id, entrypoint) handed pre-resolved by the body — NOT bare step ids the spine resolves (resolving would re-import kind knowledge). The substitutions blob stays opaque to the spine: it slots into an identical jq envelope, reads no key.
- Extract cut-line: the spine decodes buildStepOutputs[N] to a file (N the one generic param); the per-slot envelope / stamp / buf_write_fact_multi loop is body. Today's zrbld_ensconce_extract threads both — this is a real carve, not relocation.
- The poll ceiling is kind-flavored (ensconce borrows the enshrine ceiling); the body/recipe passes it — the spine does not own the ceiling table.
- Terminal-exclusivity closure for the Lode family: this carve deletes rbld_Lode.sh and makes rbld a pure container (its children rbldl_/rbldk_ already exist from the lode explosion), so the last bare rbld_ name — the cli partner — retires too. Re-home the furnish onto the entry child (rbldk_cli / zrbldk_furnish), retiring rbld_cli.sh and rerouting the zipper enrollment. Same shape as the rbfc/rbfl cli re-home at the foundry explosion; precedent is the rbh0 handbook decomposition (clis ride rbho0_/rbhp0_, never a bare rbh_cli).

## Done
- rblds_/rbldb_ carved along the designed boundary; bole ensconce composes through the spine via recipe + substitutions, with no kind knowledge in the spine; behavior-preserving (lode-lifecycle green, provenance verdicts identical); rbld_Lode.sh fully decomposed and rbld_cli.sh re-homed (rbldk_cli), so no bare rbld_ name survives — terminal exclusivity closed for the Lode family.

**[260603-1136] rough**

## Character
The heat's core — design plus implementation.

## Goal
Design the spine/body boundary and the recipe + substitutions contract shape, then carve rblds_ (spine) and rbldb_ (bole body) accordingly and make the spine data-driven; bole rides it.

## Cinched (paddock)
- Registration-agnostic: the spine takes the recipe as data — no per-kind branch, no kind-registration or discovery mechanism; registering kinds is left to the consumer, not baked into the spine.
- The spine/body boundary is decided HERE, not pre-split mechanically at the lode explosion; the requires/provides contract FORMAT is designed here against the capture domain's known axes (N-member envelopes, tool-variety, opaque-blob payloads) and recorded in the contract's RBSCJ section.

## Found (pre-impl review)
- A recipe row is (script_path, builder_image, id, entrypoint) handed pre-resolved by the body — NOT bare step ids the spine resolves (resolving would re-import kind knowledge). The substitutions blob stays opaque to the spine: it slots into an identical jq envelope, reads no key.
- Extract cut-line: the spine decodes buildStepOutputs[N] to a file (N the one generic param); the per-slot envelope / stamp / buf_write_fact_multi loop is body. Today's zrbld_ensconce_extract threads both — this is a real carve, not relocation.
- The poll ceiling is kind-flavored (ensconce borrows the enshrine ceiling); the body/recipe passes it — the spine does not own the ceiling table.

## Done
- rblds_/rbldb_ carved along the designed boundary; bole ensconce composes through the spine via recipe + substitutions, with no kind knowledge in the spine; behavior-preserving (lode-lifecycle green, provenance verdicts identical).

**[260603-1047] rough**

## Character
The heat's core — design plus implementation.

## Goal
Design the spine/body boundary and the recipe + substitutions contract shape, then carve rblds_ (spine) and rbldb_ (bole body) accordingly and make the spine data-driven; bole rides it.

## Cinched (paddock)
- Registration-agnostic: the spine takes the recipe as data — no per-kind branch, no kind-registration or discovery mechanism; registering kinds is left to the consumer, not baked into the spine.
- The spine/body boundary is decided HERE, not pre-split mechanically at the lode explosion; the requires/provides contract FORMAT is designed here against the capture domain's known axes (N-member envelopes, tool-variety, opaque-blob payloads) and recorded in the contract's RBSCJ section.

## Done
- rblds_/rbldb_ carved along the designed boundary; bole ensconce composes through the spine via recipe + substitutions, with no kind knowledge in the spine; behavior-preserving (lode-lifecycle green, provenance verdicts identical).

**[260603-0542] rough**

## Character
The heat's core — design plus implementation.

## Goal
Design the spine/body boundary and the recipe + substitutions contract shape, then carve rblds_ (spine) and rbldb_ (bole body) accordingly and make the spine data-driven; bole rides it.

## Cinched (paddock)
- Registration-agnostic: the spine takes the recipe as data — no per-kind branch, no kind-registration/discovery (that is ₣BH's scaffold).
- The spine/body boundary is decided HERE, not pre-split mechanically; the requires/provides contract FORMAT is designed here against ₣BH's locked axes and recorded in the RBSCJ section sited by AAA.

## Done
- rblds_/rbldb_ carved along the designed boundary; bole ensconce composes through the spine via recipe + substitutions, with no kind knowledge in the spine; behavior-preserving (lode-lifecycle green, provenance verdicts identical).

**[260603-0516] rough**

## Character
The heat's core — design plus implementation.

## Goal
Make the spine (rblds_) compose a build from a recipe (ordered step ids) plus a substitutions blob taken as data, riding rbfcb_; bole rides it as a thin body (rbldb_).

## Cinched (paddock)
- Registration-agnostic: no per-kind branch, no kind-registration/discovery mechanism (that is ₣BH's scaffold).
- Designed against ₣BH's locked axes (N-member envelopes, tool-variety, opaque-blob payloads); the recipe + substitutions shape is the ₣BH attach point.

## Done
- bole ensconce composes through the spine via recipe + substitutions, with no kind knowledge in the spine.
- Behavior-preserving: lode-lifecycle green, provenance verdicts identical to pre-refactor.

### inscribe-rides-spine (₢BXAAK) [abandoned]

**[260603-1048] abandoned**

## Character
Mechanical follow-on — a second body onto the proven spine.

## Goal
Make reliquary inscribe a thin body riding the host spine (rblds_): its build-submit composes through the spine instead of carrying its own copy. Kills the inscribe↔bole submit duplication and gives the spine its second validating caller.

## Cinched (paddock)
- Behavior-preserving: inscribe keeps its namespace, cohort, and docker-mirror behavior; only its submit path is rewired. The rbi_rq→rbi_ld conversion stays ₣BH's cutover.
- inscribe rides the host spine (submit/poll/extract); its docker-mirror stays a kind-specific cloud step (no token snippet — docker auto-auths).

## Done
- inscribe composes its build through the spine via recipe + substitutions as a thin body; the duplicated submit logic is gone; the inscribe/reliquary path is behavior-identical (proven at the skirmish gate).

**[260603-0601] rough**

## Character
Mechanical follow-on — a second body onto the proven spine.

## Goal
Make reliquary inscribe a thin body riding the host spine (rblds_): its build-submit composes through the spine instead of carrying its own copy. Kills the inscribe↔bole submit duplication and gives the spine its second validating caller.

## Cinched (paddock)
- Behavior-preserving: inscribe keeps its namespace, cohort, and docker-mirror behavior; only its submit path is rewired. The rbi_rq→rbi_ld conversion stays ₣BH's cutover.
- inscribe rides the host spine (submit/poll/extract); its docker-mirror stays a kind-specific cloud step (no token snippet — docker auto-auths).

## Done
- inscribe composes its build through the spine via recipe + substitutions as a thin body; the duplicated submit logic is gone; the inscribe/reliquary path is behavior-identical (proven at the skirmish gate).

### cloud-step-library (₢BXAAE) [complete]

**[260604-2204] complete**

## Character
Extraction touching one made-side step; mints the shared-cloud-step home.

## Goal
Stand up the shared cloud step-library: mint its home namespace (which breaks the rbgj family-letter scheme, since a shared step belongs to no single family), then unify the FROM-scratch vouch-push across Lode (rbgjl02) and hallmark-verify (rbgjv03); author the metadata-token fetch and the skopeo copy/digest block as composed snippets; keep envelope authoring per-kind. Each library step declares its requires/provides, refining the contract shape from the spine pace.

## Cinched (paddock)
- The shared cloud-step home is minted HERE (the library pace) — it breaks the rbgj family-letter scheme since a shared step belongs to no single family.
- vouch-push is the one whole shared step (two live callers); rbgjv03 is the only made-side touch.
- Shared in-step logic rides the composed-snippet channel; the members[] envelope tail stays per-kind.
- The ensconce collision guard is out of scope (behavior-adding); this pace neither adds it nor forecloses it — keep the capture step's pre-copy point clean.

## Found (pre-impl review)
- OPEN, resolve before executing: the vouch-push "one whole shared step" claim is challenged — l02 vs v03 diverge on five load-bearing axes (loop-N vs single, URI arity, platform literal-vs-file, conditional Dockerfile, disjoint _RBGL_/_RBGV_ substitutions); only a 3-line buildx bootstrap is verbatim-shared, so unifying as cinched injects ~4 branches to delete ~15 lines. Settle keep-vs-share-bootstrap-only (a paddock decision) first.
- Composed-snippet contract to decide: the metadata-token fetch is verbatim-shareable (param-free); the skopeo inspect→digest→fingerprint→first-copy block shares only if raw-file path + destination URI are parameters (the Lode-only retag loop and the per-kind envelope stay out). Decide whether a snippet reads host-injected _RBGx_ substitutions or only shell vars the host sets before inlining.
- Snippet payoff is mostly forward: the token/skopeo block's other consumer is enshrine (e01), which is NOT migrated — so in-heat the snippet has one live caller (bole). Consistent with the paddock's partial-dedup, but state it.

## Done
- The shared-cloud-step home is minted and recorded; the Lode and hallmark vouch-push paths ride one library step; token + copy/digest composed once; each step's requires/provides declared; lode-lifecycle and a conjure path green with verdicts identical to pre-refactor.

**[260603-1136] rough**

## Character
Extraction touching one made-side step; mints the shared-cloud-step home.

## Goal
Stand up the shared cloud step-library: mint its home namespace (which breaks the rbgj family-letter scheme, since a shared step belongs to no single family), then unify the FROM-scratch vouch-push across Lode (rbgjl02) and hallmark-verify (rbgjv03); author the metadata-token fetch and the skopeo copy/digest block as composed snippets; keep envelope authoring per-kind. Each library step declares its requires/provides, refining the contract shape from the spine pace.

## Cinched (paddock)
- The shared cloud-step home is minted HERE (the library pace) — it breaks the rbgj family-letter scheme since a shared step belongs to no single family.
- vouch-push is the one whole shared step (two live callers); rbgjv03 is the only made-side touch.
- Shared in-step logic rides the composed-snippet channel; the members[] envelope tail stays per-kind.
- The ensconce collision guard is out of scope (behavior-adding); this pace neither adds it nor forecloses it — keep the capture step's pre-copy point clean.

## Found (pre-impl review)
- OPEN, resolve before executing: the vouch-push "one whole shared step" claim is challenged — l02 vs v03 diverge on five load-bearing axes (loop-N vs single, URI arity, platform literal-vs-file, conditional Dockerfile, disjoint _RBGL_/_RBGV_ substitutions); only a 3-line buildx bootstrap is verbatim-shared, so unifying as cinched injects ~4 branches to delete ~15 lines. Settle keep-vs-share-bootstrap-only (a paddock decision) first.
- Composed-snippet contract to decide: the metadata-token fetch is verbatim-shareable (param-free); the skopeo inspect→digest→fingerprint→first-copy block shares only if raw-file path + destination URI are parameters (the Lode-only retag loop and the per-kind envelope stay out). Decide whether a snippet reads host-injected _RBGx_ substitutions or only shell vars the host sets before inlining.
- Snippet payoff is mostly forward: the token/skopeo block's other consumer is enshrine (e01), which is NOT migrated — so in-heat the snippet has one live caller (bole). Consistent with the paddock's partial-dedup, but state it.

## Done
- The shared-cloud-step home is minted and recorded; the Lode and hallmark vouch-push paths ride one library step; token + copy/digest composed once; each step's requires/provides declared; lode-lifecycle and a conjure path green with verdicts identical to pre-refactor.

**[260603-1047] rough**

## Character
Extraction touching one made-side step; mints the shared-cloud-step home.

## Goal
Stand up the shared cloud step-library: mint its home namespace (which breaks the rbgj family-letter scheme, since a shared step belongs to no single family), then unify the FROM-scratch vouch-push across Lode (rbgjl02) and hallmark-verify (rbgjv03); author the metadata-token fetch and the skopeo copy/digest block as composed snippets; keep envelope authoring per-kind. Each library step declares its requires/provides, refining the contract shape from the spine pace.

## Cinched (paddock)
- The shared cloud-step home is minted HERE (the library pace) — it breaks the rbgj family-letter scheme since a shared step belongs to no single family.
- vouch-push is the one whole shared step (two live callers); rbgjv03 is the only made-side touch.
- Shared in-step logic rides the composed-snippet channel; the members[] envelope tail stays per-kind.
- The ensconce collision guard is out of scope (behavior-adding); this pace neither adds it nor forecloses it — keep the capture step's pre-copy point clean.

## Done
- The shared-cloud-step home is minted and recorded; the Lode and hallmark vouch-push paths ride one library step; token + copy/digest composed once; each step's requires/provides declared; lode-lifecycle and a conjure path green with verdicts identical to pre-refactor.

**[260603-0542] rough**

## Character
Extraction touching one made-side step.

## Goal
Stand up the shared cloud step-library: unify the FROM-scratch vouch-push across Lode (rbgjl02) and hallmark-verify (rbgjv03); author the metadata-token fetch and the skopeo copy/digest block as composed snippets; keep envelope authoring per-kind. Each library step declares its requires/provides, refining the contract shape from the spine pace.

## Cinched (paddock)
- vouch-push is the one whole shared step (two live callers); rbgjv03 is the only made-side touch.
- Shared in-step logic rides the composed-snippet channel; the members[] envelope tail stays per-kind.
- Leave a clean pre-copy seam in the capture step for ₣BH's collision-guard.

## Done
- The Lode and hallmark vouch-push paths ride one library step; token + copy/digest composed once; each step's requires/provides declared; lode-lifecycle and a conjure path green with verdicts identical to pre-refactor.

**[260603-0516] rough**

## Character
Extraction touching one made-side step.

## Goal
Stand up the shared cloud step-library: unify the FROM-scratch vouch-push across Lode (rbgjl02) and hallmark-verify (rbgjv03); author the metadata-token fetch and the skopeo copy/digest block as composed snippets stitched into capture steps; keep envelope authoring per-kind.

## Cinched (paddock)
- vouch-push is the one whole shared step (two live callers); rbgjv03 is the only made-side touch.
- Shared in-step logic rides the composed-snippet channel; the members[] envelope tail stays per-kind.
- Leave a clean pre-copy seam in the capture step for ₣BH's collision-guard.

## Done
- The Lode and hallmark vouch-push paths ride one library step; token + copy/digest are composed once.
- lode-lifecycle and a conjure path green with verdicts identical to pre-refactor.

### dispatch-recipe-validator (₢BXAAF) [complete]

**[260604-2251] complete**

## Character
New code with its own tests — the heat's one declared behavior-adding sliver.

## Goal
A dispatch-time validator that checks each step's declared requires against the substitutions plus prior steps' provides, failing a bad composition before the expensive build.

## Cinched (paddock)
- The only behavior-adding work in the heat; the accept-path leaves every existing recipe unchanged.
- The reject-path is exercised by no current fixture, so it carries its own targeted unit tests.

## Done
- A bad composition fails at dispatch with a clear message; every existing recipe passes unchanged; unit tests cover the reject-path.

**[260603-0517] rough**

## Character
New code with its own tests — the heat's one declared behavior-adding sliver.

## Goal
A dispatch-time validator that checks each step's declared requires against the substitutions plus prior steps' provides, failing a bad composition before the expensive build.

## Cinched (paddock)
- The only behavior-adding work in the heat; the accept-path leaves every existing recipe unchanged.
- The reject-path is exercised by no current fixture, so it carries its own targeted unit tests.

## Done
- A bad composition fails at dispatch with a clear message; every existing recipe passes unchanged; unit tests cover the reject-path.

### cloud-build-spec-record (₢BXAAG) [complete]

**[260604-2303] complete**

## Character
Prose — recording cinched rationale alongside the code.

## Goal
Record the heat's rationale in the Cloud Build specs: posture in RBSCB, the composition/contract decision as an RBSCJ section.

## Cinched (paddock)
- RBSCB holds the token / credential-helper posture, the capture-duplication posture, and the /workspace-no-secrets invariant.
- RBSCJ is trigger-era stale; refresh it where touched.

## Done
- RBSCB and RBSCJ state the cinched rationales; the step contract is documented in the RBSCJ section.

**[260603-0517] rough**

## Character
Prose — recording cinched rationale alongside the code.

## Goal
Record the heat's rationale in the Cloud Build specs: posture in RBSCB, the composition/contract decision as an RBSCJ section.

## Cinched (paddock)
- RBSCB holds the token / credential-helper posture, the capture-duplication posture, and the /workspace-no-secrets invariant.
- RBSCJ is trigger-era stale; refresh it where touched.

## Done
- RBSCB and RBSCJ state the cinched rationales; the step contract is documented in the RBSCJ section.

### milestone-skirmish-gate (₢BXAAH) [complete]

**[260605-1449] complete**

## Character
Operator-driven, heavyweight verification.

## Goal
Prove the whole refactor behavior-preserving against the live system.

## Cinched (paddock)
- Requires live GCP credentials and a standing canonical depot — not free-standing.
- The ~100-minute skirmish cook is a milestone, not a per-commit cost.

## Done
- Full skirmish + service lode-lifecycle green; artifacts and provenance verdicts identical to pre-refactor; provenance-insensitivity-to-restructuring confirmed.

**[260603-0517] rough**

## Character
Operator-driven, heavyweight verification.

## Goal
Prove the whole refactor behavior-preserving against the live system.

## Cinched (paddock)
- Requires live GCP credentials and a standing canonical depot — not free-standing.
- The ~100-minute skirmish cook is a milestone, not a per-commit cost.

## Done
- Full skirmish + service lode-lifecycle green; artifacts and provenance verdicts identical to pre-refactor; provenance-insensitivity-to-restructuring confirmed.

### cbg-capstone-harvest (₢BXAAI) [complete]

**[260605-0828] complete**

## Character
Authoring — distillation from the code this heat wrote.

## Goal
Harvest CBG (the Cloud Build Guide) by distilling the per-script cloud discipline this refactor established; the spine, library, and bodies are its exemplars.

## Cinched (paddock)
- Read WSG first so the sibling framing rests on the document, not its label.
- CBG governs single-script precision (inside one cloud step), not component wiring or architecture.

## Done
- CBG is authored from the heat's exemplars; the name is confirmed against the BCG/RCG/WSG sibling set.

**[260603-0517] rough**

## Character
Authoring — distillation from the code this heat wrote.

## Goal
Harvest CBG (the Cloud Build Guide) by distilling the per-script cloud discipline this refactor established; the spine, library, and bodies are its exemplars.

## Cinched (paddock)
- Read WSG first so the sibling framing rests on the document, not its label.
- CBG governs single-script precision (inside one cloud step), not component wiring or architecture.

## Done
- CBG is authored from the heat's exemplars; the name is confirmed against the BCG/RCG/WSG sibling set.

### workspace-quoin (₢BXAAM) [complete]

**[260604-2339] complete**

## Character
MCM vocabulary mint + reference threading — deliberate spec work, low logic risk.

## Goal
Promote `/workspace` — the Cloud Build shared inter-step filesystem mount, the non-secret-only register channel — from free-floating prose to a formal quoin in the RBS0 concept model.

## Cinched
- Registered AND defined in RBS0 only (mapping section attribute ref + anchor + definition text). Subdocuments only reference the linked term — never define, never host a mapping section.
- Prefix starts with `rb` and follows the minting rules + MCM Quoin Sub-Letter Discipline (CLAUDE.md "Quoin Sub-Letter Discipline" + Memos/memo-20260110-acronym-selection-study.md). Category likely the build family (`rbtgr_`, home of build_json/provenance) — settle at mint.
- Distinct from the existing `rbrr_bottle_workspace` (a bottle regime var); the definition disambiguates the Cloud Build mount.
- The non-secret-only invariant (currently prose in RBSCB) becomes the canonical definition text.

## Done
`/workspace` registered + defined as a quoin in RBS0; the spec sites that mention it (discovery: `grep -rln workspace Tools/rbk/vov_veiled/*.adoc`) reference the linked term where load-bearing; RBS0 renders clean.

**[260604-2252] rough**

## Character
MCM vocabulary mint + reference threading — deliberate spec work, low logic risk.

## Goal
Promote `/workspace` — the Cloud Build shared inter-step filesystem mount, the non-secret-only register channel — from free-floating prose to a formal quoin in the RBS0 concept model.

## Cinched
- Registered AND defined in RBS0 only (mapping section attribute ref + anchor + definition text). Subdocuments only reference the linked term — never define, never host a mapping section.
- Prefix starts with `rb` and follows the minting rules + MCM Quoin Sub-Letter Discipline (CLAUDE.md "Quoin Sub-Letter Discipline" + Memos/memo-20260110-acronym-selection-study.md). Category likely the build family (`rbtgr_`, home of build_json/provenance) — settle at mint.
- Distinct from the existing `rbrr_bottle_workspace` (a bottle regime var); the definition disambiguates the Cloud Build mount.
- The non-secret-only invariant (currently prose in RBSCB) becomes the canonical definition text.

## Done
`/workspace` registered + defined as a quoin in RBS0; the spec sites that mention it (discovery: `grep -rln workspace Tools/rbk/vov_veiled/*.adoc`) reference the linked term where load-bearing; RBS0 renders clean.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 L skirmish-centos-baseline
  2 A mint-and-step-contract
  3 J provenance-insensitivity-spike
  4 B lode-module-explosion
  5 C foundry-module-explosion
  6 D capture-spine-data-driven
  7 E cloud-step-library
  8 F dispatch-recipe-validator
  9 G cloud-build-spec-record
  10 H milestone-skirmish-gate
  11 I cbg-capstone-harvest
  12 M workspace-quoin

LAJBCDEFGHIM
·x·xxxx····· rbk-claude-acronyms.md
·····xxx···x RBSCJ-CloudBuildJson.adoc
·······xx··x RBSCB-CloudBuildPosture.adoc
·····xxx···· rblds_Spine.sh
···xx·x····· rbfcb_BuildHost.sh
···xxx······ rbldk_Kindle.sh
····x··x···· rbtdrf_fast.rs
····x·x····· rbfca_StepAssembly.sh, rbfck_Kindle.sh
···x·x······ rbldb_Bole.sh
···xx······· rbfc_FoundryCore.sh, rbz_zipper.sh
···········x RBS0-SpecTop.adoc, RBSLE-lode_ensconce.adoc, RBSOB-oci_layout_bridge.adoc
··········x· CBG-CloudBuildGuide.md, WSG-WindowsScriptingGuide.md, claude-rbk-acronyms.md
·······x···· rbtdrc_crucible.rs, rbtdrm_manifest.rs
······x····· rbgjl01-ensconce-capture.sh, rbgjl02-assemble-push-vouch.sh, rbgjs-buildx-bootstrap.sh, rbgjs-buildx-push.sh, rbgjs-skopeo-fingerprint.sh, rbgjs-token-fetch.sh, rbgjv03-assemble-push-vouch.sh
····x······· rbfc0_cli.sh, rbfc_cli.sh, rbfcg_GarRest.sh, rbfcp_Plumb.sh, rbfcv_VesselResolution.sh, rbfd_FoundryDirectorBuild.sh, rbfh_cli.sh, rbfk_kludge.sh, rbfl0_cli.sh, rbfl_FoundryLedger.sh, rbfl_cli.sh, rbfld_Delete.sh, rbfli_Inscribe.sh, rbflk_Kindle.sh, rbfln_Inventory.sh, rbflw_Wrest.sh, rbfly_Yoke.sh, rbfr_FoundryRetriever.sh, rbfv_FoundryVerify.sh, rbndb_base.sh, rbob_bottle.sh
···x········ jjk-roster.md, rbld0_cli.sh, rbld_Lode.sh, rbld_cli.sh, rbldl_Lifecycle.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 49 commits)

  1 L skirmish-centos-baseline
  2 A mint-and-step-contract
  3 J provenance-insensitivity-spike
  4 B lode-module-explosion
  5 C foundry-module-explosion
  6 D capture-spine-data-driven
  7 E cloud-step-library
  8 F dispatch-recipe-validator
  9 G cloud-build-spec-record
  10 M workspace-quoin
  11 H milestone-skirmish-gate
  12 I cbg-capstone-harvest

123456789abcdefghijklmnopqrstuvwxyz
·····x·····························  L  1c
·········xx························  A  2c
···········x·······················  J  1c
············x·x····················  B  2c
···············xx··················  C  2c
·················xx················  D  2c
···················xx··············  E  2c
·····················xxx···········  F  3c
·························xx········  G  2c
···························x··x····  M  2c
····························x······  H  1c
·······························xxxx  I  4c
```

## Steeplechase

### 2026-06-05 08:28 - ₢BXAAI - W

Harvested CBG (Cloud Build Guide) from the heat's cloud-step exemplars: a polyglot (bash/sh + python) step-body discipline, foreign-environment sibling to BCG/RCG/WSG, at Tools/rbk/vov_veiled/CBG-CloudBuildGuide.md. Structured as two genres on purpose — authored disciplines as named prose (crash-fast idioms, the Native-Serializer Rule, logic/network split, step skeleton, GAR-auth idiom) and a lean cited-rules catalog (~10) kept only where an external citer exists: cloud-Pale invariants (CBi_), the relaxed-$() divergence from BCG (CBb_), the python preamble reuse-gap scar with a removal condition (CBp_), and the host-seam contracts (CBh_). Confirmed name against the sibling set (read WSG first per the Capstone). Settled in conversation: scope is polyglot (python steps are real and spine-wired, not speculative); rule designators are load-bearing only as citable handles (BCG left ID-free, WSG kept its catalog); designators converted to sprue form (_ separator, mixed-case prefix + 3-digit number) across CBG and WSG for globally-unique grep tokens. Registered CBG in the RBK acronym map.

### 2026-06-05 08:25 - ₢BXAAI - n

Convert rule designators to sprue form (- to _) across CBG (34 designators) and WSG (60: WSp_/WSs_/WSt_), plus the CBG family-prefix line in the RBK acronym map. Mixed-case prefix + underscore + 3-digit number yields a globally-unique grep token (CBb_101, WSt_104) that matches the designator and nothing else, across code comments, memos, and prose. Content-only convention change, behavior-irrelevant; counts verified unchanged with zero hyphen stragglers.

### 2026-06-05 08:19 - ₢BXAAI - n

Prune CBG rule designators to the citable core. Apply the discriminator (catalog what is foreign, systematize what is yours): keep numbered rules only where an external citer exists — the cloud-Pale invariants (CBi), the relaxed-$() divergence (CBb), the python preamble reuse-gap scar with a removal condition (CBp), and the host-seam contracts (CBh) — ~10 rules down from ~24. Demote owned/interderivable discipline (crash-fast idioms, the Native-Serializer Rule, logic/network split, step-body skeleton, GAR-auth idiom) to BCG-style named prose. Add an explicit two-genre framing so the discriminator is legible to future readers. BCG and WSG left untouched.

### 2026-06-05 08:08 - ₢BXAAI - n

Harvest CBG (Cloud Build Guide) first draft from the heat's cloud-step exemplars. Polyglot step-body discipline (bash/sh + python) as a foreign-environment sibling to BCG/RCG/WSG: invariants core (CBi-) plus per-language dialects (CBb- bash, CBp- python) and a host-composition seam (CBh-). Distilled load-bearing insights: the native-serializer rule (json.dump vs printf, same rule/opposite mechanic), the characterized relaxation of BCG's temp-file mandate inside ephemeral steps, and the recorded python helper-preamble reuse gap. Registered CBG in the RBK acronym map.

### 2026-06-04 23:39 - ₢BXAAM - W

Minted /workspace as the naked quoin rbtgr_workspace (Build Workspace) in RBS0's build-definitions family beside rbtgr_build_json/rbtgr_provenance. Definition carries the non-secret-only invariant as canonical text and disambiguates from rbrr_bottle_workspace (bottle container working dir). No //axl_voices motif per operator — too soon for an AXLA mint, left naked. Threaded the linked term into four load-bearing concept sites: RBSCB posture invariant, RBSCJ Invariant(RBSCB) restatement, RBSOB native-/workspace bridge, RBSLE stage-on-workspace; left path literals, code examples, and the freshly-authored RBSCJ substitution-coverage deep-dive untouched; skipped RBSTB (commented out of RBS0 includes — since fully retired). Verified structurally clean (anchor unique, all 5 refs resolve, disambiguation target resolves, no typo variants); full asciidoctor render not run (no CLI on station, GAD is a browser differ). Code commit a68267924.

### 2026-06-04 23:38 - Heat - n

Retire the RBSTB trigger-build spec — full retirement (option a) of the operation spec for the eliminated trigger-dispatch mechanism (superseded by builds.create + ark pouch). Assessed safe: RBSTB defines no anchors any other doc references, holds no unique live knowledge (its Concurrent Session Safety note merely back-references the RBS0 invariant), and has no live code or test dependency (sole code vestige is the dead ZRBFC_TRIGGERS_URL constant). Durable rationale is already retained without it — the RBSCTD trade study (marked SUPERSEDED), the RBS0 ELIMINATED tombstone section, and the RBWMBX multi-platform-auth memo link. Changes: delete RBSTB-trigger_build.adoc; remove the now-dangling commented include in RBS0 (the ELIMINATED prose stays as breadcrumb); drop the acronym-map entry; retarget RBSCB's internal reference from '(inactive)' to 'retired', pointing at RBSCTD. Load-bearing split: operation spec (dead how) retired, trade study (durable why) kept.

### 2026-06-05 14:49 - ₢BXAAH - W

Milestone skirmish gate cleared — capture-machinery-unification refactor proven behavior-preserving against the live system. Evidence (this session, HEAD carrying all 9 BX refactor commits): full skirmish 254/254 passed (1 skip) covering the build/ordain path incl. cloud ordain + 4 crucible charges; service lode-lifecycle 1/1 passed against live GAR (ensconce busybox into fresh Lode -> enumerate -> inspect confirms member tags + vouch envelope rode in -> banish -> enumerate confirms registry restored). Both cinched preconditions held live: GCP auth proven (dogfight/skirmish/canonical-invest credential cases all green after operator reauth), standing canonical depot present (canonical-invest passed). Provenance verdicts intact across both the conjure/ordain path (skirmish vouch assertions) and the Lode capture path (lode-lifecycle vouch-envelope inspect) — provenance-insensitivity-to-restructuring confirmed operationally; the literal before/after baseline diff was the earlier provenance-insensitivity spike's scope.

### 2026-06-04 23:34 - ₢BXAAM - n

Mint /workspace as the naked quoin rbtgr_workspace (Build Workspace) in RBS0's build-definitions family, beside rbtgr_build_json/rbtgr_provenance. Definition carries the non-secret-only invariant as canonical text and disambiguates from rbrr_bottle_workspace (the bottle container working dir). No //axl_voices motif per operator (too soon for an AXLA mint; left naked). Thread the linked term into four load-bearing concept sites — RBSCB posture invariant, RBSCJ Invariant(RBSCB) restatement, RBSOB native-/workspace bridge, RBSLE stage-on-workspace — leaving path literals, code examples, and the freshly-authored RBSCJ substitution-coverage deep-dive untouched. RBSTB skipped (commented out of RBS0 includes).

### 2026-06-04 23:03 - ₢BXAAG - W

Audit-and-confirm pass on the heat's Cloud Build spec recording. CONFIRMED coherent and complete: every cross-reference the recorded sections assert is live and correctly located — all 10 named functions resolve post-decomposition (rbfl_tally->rbfln_Inventory, rbfc_plumb_*->rbfcp_Plumb, zrbfd_stitch_build_json->rbfd_, zrbfc_wait/expand/write->rbfcb_BuildHost, zrbld_spine_validate/_dispatch->rblds_Spine, zrbld_ensconce_submit->rbldb_Bole); the ZBXAAF substitution-coverage rename left no requires/provides or ordered-walk remnants (the one requires/provides hit is the rbgjs snippet shell-var contract, a distinct current channel); RBSCB<->RBSCJ cross-spec coherence holds on decision A (derivation + /workspace deferred-to-in-step); and every paddock recording-promise maps to live spec text. APPLIED the one in-scope fix: RBSCB's capture-machinery-duplication Deferred item retensed from 'Planned remedy' to remedy-built-this-heat (host spine + rbgjs compose-once library + dispatch validator), noting dedup is partial-by-design (only bole rides the spine; enshrine/inscribe forks persist until cutover) and behavior-preservation is gated on the milestone skirmish. SURFACED three adjacent pre-existing items, left unfixed as out-of-pace-scope: (1) RBSCB line ~16 references acronym RBSDC which names no file (likely RBSDI depot_inscribe — acronym judgment deferred to operator); (2) RBSCJ trigger-era Decision section names zrbf_stitch_build_json, the dead pre-rename function (live is zrbfd_); (3) the paddock premise still describes /workspace with explicit per-step requires/provides, predating decision A's deferral of that dispatch contract.

### 2026-06-04 23:03 - ₢BXAAG - n

Refresh RBSCB's capture-machinery-duplication remedy from planned to built: rename to "Remedy (built in the capture-machinery-unification refactor)", name the shared cloud step-library (rbgjs) of compose-once snippets alongside the host assembly spine and substitution-coverage validator, and record that dedup is deliberately partial — only the survivor (bole) rides the spine while the doomed enshrine/inscribe forks (and their duplicated copies above) persist until later cutover. Replace the dedicated-refactor-heat scoping line with the behavior-preservation claim confirmed at the heat's milestone skirmish gate (full skirmish + service).

### 2026-06-04 22:52 - Heat - S

workspace-quoin

### 2026-06-04 22:51 - ₢BXAAF - W

Dispatch-time recipe substitution-coverage validator (zrbld_spine_validate) implemented in the Lode capture-assembly spine and verified — the heat's one declared behavior-adding sliver. Per operator-cinched decision A: the substitution channel is validated by DERIVATION (scan each step's include-expanded body for _RBGL_* references — the references ARE the requires, nothing declared, nothing to drift — and fail dispatch before submit at the first reference absent from the substitutions blob's keys); coverage is flat (substitutions automap into every step: no recipe order, no cross-step provides); the /workspace inter-step channel is deferred to the steps' own in-build guards (test -f), not safely static-derivable. BCG-faithful after reading the guide in full: pure builtins ([[ =~ ]] extraction, case membership, load-then-iterate — no grep/sort, both evicted), no silent failures, sentinel-free return-1 primitive matching the rbfcb_ build primitives the dispatch loop already rides. recipe-validation theurge fixture (10 cases, reject-path heavy) 10/10 green; shellcheck 200 files clean. Contract docs refreshed to match: RBSCJ section renamed 'Substitution-coverage check' with the derivation model, RBSCB planned-remedy amended off the deferred /workspace over-claim. Commits a276cc1 (code+test), f6893bb (docs). Live milestone cook (full skirmish + service) remains the separate heat gate.

### 2026-06-04 22:49 - ₢BXAAF - n

Refresh the Capture Composition Contract's coverage section in RBSCJ to match the implemented validator (decision A). Rename the section 'Requires/provides format' -> 'Substitution-coverage check' and replace the declaration + ordered-walk model with derivation: the _RBGL_* references in a step body ARE its substitution-requires (nothing declared, nothing to drift), coverage is flat (substitutions automap into every step — no recipe order, no cross-step provides), and the scan is blind to the snippet shell-var channel. Document that the /workspace inter-step channel is deferred to the steps' own in-build guards rather than a dispatch-time contract, because its reads/writes are not safely static-derivable (templated paths, read-vs-write ambiguity). Update the matching cross-ref in rblds_Spine.sh's validator comment. Amend RBSCB's capture-machinery-duplication planned-remedy to drop the now-deferred 'explicit /workspace pre/post contracts' over-claim and point at the substitution-coverage derivation.

### 2026-06-04 22:36 - ₢BXAAF - n

Implement the dispatch-time recipe substitution-coverage validator (zrbld_spine_validate) in the Lode capture-assembly spine — the heat's one declared behavior-adding sliver. Per operator-cinched decision A: validate the substitution channel by DERIVATION, not declaration — scan each step's include-expanded body for _RBGL_* references (the references ARE the requires, no comment to drift) and fail dispatch before submit at the first reference absent from the substitutions blob's keys; the /workspace inter-step channel is deferred to the steps' own in-build runtime guards (test -f), which already crash-fast. Coverage is flat (substitutions automap into every step — no ordering, no cross-step provides). zrbld_spine_dispatch reads the blob keys once (jq keys[] to a temp file) and calls the check per step inside the composition loop where each body is already include-expanded, gating before the rbuh_json submit. BCG-faithful rewrite after reading the guide in full: pure builtins (no grep/sort — both evicted), [[ =~ ]] token extraction + case whole-line membership + load-then-iterate; no silent failures (dropped a || true); sentinel-free return-1 primitive matching the rbfcb_ build primitives the dispatch loop already rides, caller buc_die's with the step identity. Add the recipe-validation theurge fixture (10 rbtdrf_rc_* cases, reject-path heavy: missing-key, substring-discipline, multi-token, empty-keys) modeled on the foundry-path pure-bash-unit pattern (source buv+rblds, zbuv_kindle, call the function, assert exit code); wired into the RBTDRC_FIXTURES lookup registry and the fast/service/crucible/complete suites, plus the no-prereqs manifest arm. Verified: bash -n clean, shellcheck 200 files clean, scratch harness 10/10 against live buv+spine, theurge compiles.

### 2026-06-04 22:04 - ₢BXAAE - W

Shared cloud-step snippet library (rbgjs) stood up and live-verified. Minted Tools/rbk/rbgjs/ — the no-family member of the rbgj<family>/ scheme — with four composed-once snippets (token-fetch, skopeo-fingerprint, buildx-bootstrap, buildx-push), each spliced at #@rbgjs_include markers by a new verbatim splicer zrbfc_expand_includes (rbfcb_, additive — no existing logic touched). Snippets read shell vars the kind sets before the marker, never _RBGx_ substitutions, so one snippet serves disjoint namespaces (_RBGL_* and _RBGV_*). Resolved the OPEN vouch-push challenge against the bytes (operator-cinched share-the-primitive): vouch-push is NOT one whole shared step — rbgjl02/rbgjv03 diverge on five load-bearing axes, only the buildx bootstrap+push are identical; the skopeo snippet is narrowed to the measurement core (inspect->digest->fingerprint), first-copy stays per-kind since its tag is digest-derived. One sanctioned made-side touch: rbgjv03 + the expander wiring in zrbfc_assemble_vouch_steps. Contract recorded in RBSCJ (Composed-snippet library); RBGJS minted in the acronym map. The include-expander kept deliberately verbatim — assembly does no content editing; the inlined-header concern was solved at authoring time per operator.

### 2026-06-04 21:17 - ₢BXAAE - n

Stand up the shared cloud-step snippet library (rbgjs) — the no-family member of the rbgj<family>/ cloud-step scheme, since a shared snippet belongs to no made-side family. Mint Tools/rbk/rbgjs/ holding four composed-once fragments: rbgjs-token-fetch (metadata-server OAuth token; provides TOKEN), rbgjs-skopeo-fingerprint (inspect->digest->fingerprint; requires ORIGIN+RAW_FILE, provides SHA+FINGERPRINT), rbgjs-buildx-bootstrap (the rb-builder buildx builder), rbgjs-buildx-push (the irreducible FROM-scratch push; requires PUSH_URI+PUSH_PLATFORMS+PUSH_CTX). Each carries a bash shebang for shellcheck only and a header that reads true whether in-file or inlined.

### 2026-06-04 20:27 - ₢BXAAD - W

Carved the data-driven capture-assembly spine (rblds_Spine.sh) out of the bole body along the designed boundary. zrbld_spine_dispatch composes a Cloud Build from a recipe of pre-resolved script_path|builder|id|entrypoint rows plus an opaque substitutions blob, submits, and polls; zrbld_spine_extract decodes buildStepOutputs[N] to a file. Boundary decision: the spine owns the capture-domain build constants (mason SA, TETHER pool, regime timeout) because every Lode capture fetches upstream bytes as mason; the body passes only the kind-flavored poll ceiling. rbldb_ thinned to a body that builds the ensconce recipe + substitutions blob and rides the spine, keeping the per-member slot extract loop. Capture Composition Contract recorded in RBSCJ: spine/body boundary, recipe-row format, opaque substitutions, extract contract, and the requires/provides format designed against the three capture axes (N-member envelopes, tool-variety, opaque-blob) for the later recipe-validator pace. Spine wired into rbldk_ kindle entry; RBLDS/RBLDB/RBLDK acronym entries refreshed. Two BCG violations the agent had first rationalized away were then fixed at operator insistence: the build-id capture moved to the canonical two-line || buc_die, and the slot-stamp jq moved from $() to a temp-file read (crash-fast on malformed output). Verification: bash -n clean, shellcheck 196 files clean, byte-identical build JSON achievable. Live lode-lifecycle green + provenance-verdict-identical confirmation deferred to the heat-gate cook per the paddock verification model and operator decision.

### 2026-06-04 20:24 - ₢BXAAD - n

Carve the data-driven capture-assembly spine out of the bole body. New rblds_Spine.sh holds zrbld_spine_dispatch (compose a Cloud Build from a recipe of pre-resolved script_path|builder|id|entrypoint rows plus an opaque substitutions blob, submit, persist build id, poll) and zrbld_spine_extract (decode buildStepOutputs[N] to a file). Designed boundary: the spine owns the capture-domain build constants (mason SA, TETHER pool, regime timeout) since every Lode capture fetches upstream bytes as mason; the body passes only the kind-flavored poll ceiling. rbldb_Bole.sh thinned to a body that builds the ensconce recipe + substitutions blob and rides the spine, keeping the per-member slot extract loop. Record the Capture Composition Contract in RBSCJ: spine/body boundary, recipe-row format, opaque substitutions, extract contract, and the requires/provides format designed against the capture axes (N-member envelopes, tool-variety, opaque-blob) for the later validator. Wire the spine into rbldk_ kindle entry; refresh RBLDS/RBLDB/RBLDK acronym entries. BCG-compliant: capture uses canonical two-line || buc_die, jq stamp read moved to a temp file (crash-fast on malformed output). Static verification: bash -n clean, shellcheck 196 files clean. Live lode-lifecycle/provenance confirmation deferred to the heat gate per operator.

### 2026-06-04 14:48 - ₢BXAAC - W

Foundry-module-explosion landed (relocation only). rbfc_FoundryCore.sh decomposed into rbfck_Kindle (entry: silent ZRBFC_SOURCED guard + zrbfc_kindle/sentinel + z_rbfc_tool_* globals) plus guard-free clusters rbfcv_VesselResolution/rbfca_StepAssembly/rbfcg_GarRest/rbfcp_Plumb, with 0-top rbfc0_cli. rbfl_FoundryLedger.sh decomposed into rbflk_Kindle (entry: buc_die ZRBFL_SOURCED guard + zrbfl_kindle/sentinel) plus guard-free clusters rbfli_Inscribe/rbfly_Yoke/rbfld_Delete/rbfln_Inventory/rbflw_Wrest, with 0-top rbfl0_cli. Functions kept their container prefix and moved verbatim, so zero call-site rewrites; the forced sweep was source-line repoints only: consumers (rbfr_/rbfk_/rbfh_cli/rbfd_/rbfv_/rbldk_) source rbfck_Kindle.sh, the Rust fast-path driver sources rbfcb_BuildHost.sh directly (var rbfc->rbfcb), rbz_zipper points at rbfc0_/rbfl0_cli.sh, and the acronym map + stale ownership/native-path comments were refreshed. Old monoliths and bare CLIs deleted — no bare rbfc_/rbfl_ file survives, closing this heat's terminal-exclusivity violation. Verified green: verbatim code-line diff (zero dropped lines, both monoliths), shellcheck (195 files clean), theurge compiles, fast suite (136 passed), and full service suite (11 fixtures, 143 passed, 0 failed) including hallmark-lifecycle (real conjure+vouch Cloud Build with provenance verified), lode-lifecycle (rbld + rbfcb_ primitives), and batch_vouch_lifecycle (rbfv) — behavior-preservation confirmed across the credentialed surface. Standalone explicit conjure skipped as redundant since hallmark-lifecycle already exercised that path. Relocation committed in 536fe58; the full skirmish remains for the milestone gate.

### 2026-06-04 14:14 - ₢BXAAC - n

Foundry-module-explosion (relocation only): decompose rbfc_FoundryCore.sh into rbfck_Kindle.sh (entry: silent ZRBFC_SOURCED guard + zrbfc_kindle/sentinel + leaked z_rbfc_tool_* globals; sources rbfcb_ and the body clusters) plus guard-free clusters rbfcv_VesselResolution, rbfca_StepAssembly, rbfcg_GarRest, rbfcp_Plumb; retire rbfc_cli.sh for the 0-top rbfc0_cli.sh. Decompose rbfl_FoundryLedger.sh into rbflk_Kindle.sh (entry: buc_die ZRBFL_SOURCED guard + zrbfl_kindle/sentinel; sources rbfck_ and the body clusters) plus guard-free clusters rbfli_Inscribe, rbfly_Yoke, rbfld_Delete, rbfln_Inventory, rbflw_Wrest; retire rbfl_cli.sh for rbfl0_cli.sh. Functions keep their container prefix (zrbfc_/zrbfl_/rbfc_/rbfl_) and move verbatim, so call sites are unchanged; the forced sweep is source-line repoints only: consumers (rbfr_, rbfk_, rbfh_cli, rbfd_, rbfv_, rbldk_) source rbfck_Kindle.sh; the Rust fast-path driver rbtdrf_fast.rs sources rbfcb_BuildHost.sh directly (var rbfc->rbfcb); rbz_zipper enrollments point at rbfc0_/rbfl0_cli.sh; rbk-claude-acronyms RBFC/RBFL container+child entries refreshed; stale native-path comments (rbob_, rbndb_ -> rbfcb_) and rbfd_ ownership comments repointed; rbfcb_ provenance header updated. 0-top CLIs set 755 to match convention. Verified: verbatim code-line diff (zero dropped lines, both monoliths), bash -n + shellcheck (195 files clean), theurge compiles, and rbfc/rbfl source-kindle-dispatch chains green via rbw-fpc/rbw-fA. Theurge fast/service fixtures deferred (clean-tree gate).

### 2026-06-04 13:38 - ₢BXAAB - W

Lode-module-explosion verified green post-landing: dogfight (4 passed — canonical-invest re-mantle/re-invest + conjure ordain→summon→run→abjure, exercising rbfc→rbfcb_), lode-lifecycle (1 passed — decomposed rbld path ensconce→divine→banish end-to-end against live GAR, registry restored), siege (60 passed — local kludge-tadmor + tadmor crucible security suite). The Done bullet's lode-lifecycle-green criterion now actually proven; rbfcb_ relocation confirmed on both made-side (ordain) and fetched-side (ensconce) sourcing chains.

### 2026-06-04 11:11 - Heat - d

paddock curried: 0-trick CLI + file-level terminal exclusivity (functions keep container prefix)

### 2026-06-04 11:06 - ₢BXAAB - n

Lode-module-explosion (relocation only): decompose rbld_Lode.sh into rbldk_Kindle.sh (entry — the single ZRBLD_SOURCED guard + kindle/sentinel + ZRBLD_ consts, sources the guard-free clusters), rbldl_Lifecycle.sh (divine/banish), and rbldb_Bole.sh (the full ensconce path — bole body holding the residual until the capture-spine carve). Retire rbld_cli.sh for the 0-top CLI rbld0_cli.sh, and pull the four Foundry build-primitives (wait-build-completion, git-metadata, write-script-body, native-path) into rbfcb_BuildHost.sh, sourced by rbfc_FoundryCore.sh so every consumer (incl. the Rust driver that sources rbfc by name) reaches them unchanged. Functions keep the container prefix — terminal exclusivity is enforced at the file-name level per the rbh0 precedent — so call sites are unchanged. Correct the acronym-map mint record: 0-trick CLIs (rbld0/rbfc0/rbfl0) replace the misapplied re-home-onto-kindle; rbfcb_ entry reflects existence+sourcing. Repoint rbz_zipper enrollment to rbld0_cli.sh. Verified: bash -n + shellcheck (project rc) clean across all files, 137 theurge unit tests pass, runtime sourcing chain (rbldk->rbfc->rbfcb+rbldl+rbldb) and kindle-independent primitives validated. Also lands the jjk-roster slash command.

### 2026-06-04 10:26 - ₢BXAAJ - W

Provenance-insensitivity spike: CONFIRMED the SLSA verdict is invariant to cloud-build step restructuring, by reading rbgjv02-verify-provenance.py in full. The conjure path (_verify_conjure) gates on exactly three content assertions — DSSE keyid == Google's google-hosted-worker key (:230-232), openssl signature validity over the PAE-wrapped payload (:252-260), and builder.id == GoogleHostedWorker (:263-266) — all signer/builder identity, all invariant to what the build steps did. buildType (:268) and subject-digest (:269) are read but never asserted; steps/buildConfig/externalParameters/internalParameters/resolvedDependencies are never read at all. Bind mode is digest-pin and graft is a GRAFTED stamp (method:none) — neither touches provenance, so conjure is the decisive read. Verdict matches the pre-impl prediction, re-derived independently from source, adding the nuance that TWO fields (buildType + subject-digest) are read-not-asserted, not one. The behavior-preserving rail holds: restructuring steps cannot move a provenance verdict, so the spine (rblds_) and shared cloud step-library proceed on confirmed ground. Pale caveat: this vouches what our verifier asserts (Google identity + signature validity), not Google's upstream signing — but that is the gate we rely on, so the verdict holds regardless of recipe restructuring.

### 2026-06-04 10:16 - ₢BXAAA - W

Recorded the host-module submint allocation in the rbk acronym map: flipped rbld/rbfc/rbfl to containers with their rbXYZ_ children (rbld: rblds_ spine, rbldb_ bole body, rbldl_ lifecycle, rbldk_ kindle + reserved rbldt_/r_/w_/v_ kind-letters; rbfc: rbfcv_/rbfcb_/rbfca_/rbfcg_/rbfcp_/rbfck_; rbfl: rbfli_/rbfly_/rbfld_/rbfln_/rbflw_ + the rbflk_ kindle the paddock legend had omitted). Fixed a paddock/docket flaw: sub-letters are allocated at this mint gate, not deferred to the explosion (the gate's whole purpose is stable targets). Fold-fix: repointed the phantom RBF entry (rbf_Foundry.sh was renamed to rbfd_FoundryDirectorBuild.sh in heat A0) to a non-terminal container declaration, and backfilled the four undocumented Foundry leaves rbfd/rbfk/rbfr/rbfv. Caught a terminal-exclusivity gap the heat opens: once rbld/rbfc/rbfl gain children, their bare cli partners (rbld_cli/rbfc_cli/rbfl_cli, *_furnish) violate exclusivity, so each re-homes onto its kindle-entry child (rbldk_cli/rbfck_cli/rbflk_cli) per the rbh0 handbook precedent; recorded the re-homes in the allocation and wired the closure requirement into the foundry-explosion (BXAAC) and capture-spine (BXAAD) dockets so no bare rbfc_/rbfl_/rbld_ name survives heat end.

### 2026-06-04 10:16 - ₢BXAAA - n

RBLDK acronym refinement: recast from "kindle/shared-consts" to "kindle-entry" module that re-homes the CLI dispatch as `rbldk_cli` (`zrbldk_furnish`), retiring bare `rbld_cli.sh` so the container prefix `rbld` names nothing once it has children (terminal-exclusivity). Parallels the prior RBFCK/RBFLK re-homing.

### 2026-06-04 09:20 - Heat - d

paddock curried: fix Minting/Shape flaw: sub-letters allocated at mint pace (the gate), not deferred to explosion; explosion refines only cluster boundaries

### 2026-06-03 11:44 - Heat - d

paddock curried: add Relocation discipline section (consolidated from BXAAB/BXAAC pre-impl-review findings)

### 2026-06-03 10:48 - Heat - T

inscribe-rides-spine

### 2026-06-03 15:33 - ₢BXAAL - W

Captured the pre-refactor green baseline on cerebro (commit 5acf72f83) — the platform the BX milestone gate re-runs on. Full skirmish (11 fixtures: 252 passed / 0 failed / 1 skipped; the skip is rbtdrf_rs_rbrs, conditional) plus the service lode-lifecycle fixture (rbtdrc_lode_lifecycle: 1 passed) both green. Provenance verdicts green-by-assertion via the ordain chain (conjure SLSA, bind digest-pin, graft GRAFTED) and the airgap compare-plumb; lode-lifecycle exercised the bole ensconce->divine->banish capture machinery this heat refactors. Durable records in logs-buk: hist-rbw-ts-skirmish-20260603-132620-1043297-236.txt and hist-rbw-tf-sh-20260603-152447-1168275-281.txt. This is the frozen before-state the gate diffs against post-refactor. Passed on cerebro.

### 2026-06-03 06:05 - Heat - S

skirmish-centos-baseline

### 2026-06-03 06:01 - Heat - S

inscribe-rides-spine

### 2026-06-03 05:49 - Heat - d

paddock curried: BX paddock: add early provenance-insensitivity spike to Verification (one full cook, at the gate); scrub cross-heat state presumption — drop Relationship-to-BH section, re-ground forward-looking design in own terms (future kinds / later cutover work)

### 2026-06-03 05:43 - Heat - S

provenance-insensitivity-spike

### 2026-06-03 05:17 - Heat - S

cbg-capstone-harvest

### 2026-06-03 05:17 - Heat - S

milestone-skirmish-gate

### 2026-06-03 05:17 - Heat - S

cloud-build-spec-record

### 2026-06-03 05:17 - Heat - S

dispatch-recipe-validator

### 2026-06-03 05:16 - Heat - S

cloud-step-library

### 2026-06-03 05:16 - Heat - S

capture-spine-data-driven

### 2026-06-03 05:16 - Heat - S

foundry-module-explosion

### 2026-06-03 05:16 - Heat - S

lode-module-explosion

### 2026-06-03 05:15 - Heat - S

mint-and-step-contract

### 2026-06-03 05:04 - Heat - d

paddock curried: BX paddock: inscribe-not-migrated (read-three-migrate-one), host-bash decomposition + rbldX submint legend, contract to RBSCJ, registration-agnostic spine, cinch vocab, relocation-only Foundry explosion

### 2026-06-03 03:48 - Heat - f

racing

### 2026-06-03 03:47 - Heat - f

silks=rbk-09-capture-machinery-unification

### 2026-06-03 03:39 - Heat - d

paddock curried: add CBG capstone — Cloud Build Guide as a required output harvested from this heat's bash; record WSG as its structural precedent

### 2026-06-03 03:18 - Heat - d

paddock curried: initial paddock — capture-machinery unification heat, honed from BH groom session

### 2026-06-03 03:17 - Heat - N

rbk-capture-machinery-unification

