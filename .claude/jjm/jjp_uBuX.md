## Heat nature

Behavior-preserving refactor heat (with one small, declared behavior-adding sliver — the recipe validator; see Premises). Unify the fetched-side capture machinery — a host assembly spine plus a shared cloud step-library — extracted from the capture instances that exist today, so future Lode kinds attach as thin per-kind bodies rather than copy-pasted forks. No new Lode kinds. Prerequisite for ₣BH (the Lode kind universe), which builds its kind verticals on this foundation.

## Why this heat exists

The bole pilot is an unfactored fork: its host build-submit is largely duplicated with reliquary inscribe's, and its cloud capture step is a copy of enshrine's. Across the cloud step family the duplication is pervasive — the metadata-token fetch in every skopeo step, the inspect→digest→fingerprint→copy block shared by enshrine and ensconce, the FROM-scratch vouch-push shared by Lode (rbgjl02) and hallmark-verify (rbgjv03). Greenfielding the remaining kinds off the pilot would clone that mess. We are at the rule of three — bole, reliquary, and enshrine are three live instances of the one capture pattern — which is the moment to abstract from instances we have rather than speculate.

## Premises

**Behavior-preserving is the rail — and its meaning is precise.** Every machinery change is a refactor whose oracle is the test suite. "Behavior-preserving" means the produced artifacts and their SLSA provenance verdicts are identical before and after — NOT that the generated build JSON is byte-identical (step ids, ordering, and inlined script bytes will change). The heat must confirm SLSA provenance is insensitive to step restructuring (provenance records build inputs/outputs, not step cosmetics); that confirmation is part of done. This rail sorts behavior-adding work out of the heat by construction: ₣BH owns the ensconce collision guard and the reliquary namespace cutover.

**The one declared exception: the recipe validator.** A dispatch-time validator — checks each step's declared requires against substitutions plus prior steps' provides, failing a bad composition before the expensive build — is genuinely new code. Its accept-path is behavior-preserving (existing recipes pass through unchanged), but its reject-path is exercised by no current fixture, so it carries its own targeted unit tests. It guards composition, not step logic; a logic regression still surfaces only in the full cook.

**Why it validates the kind mechanism without speculation.** Unifying the surviving capture callers — bole ensconce and reliquary inscribe — rides two real, test-covered, structurally-divergent bodies over one spine, stronger evidence it holds a third kind than any stub. enshrine is a third reference instance but is doomed at ₣BH's bole cutover, so it informs the shape and is not migrated (see Scope).

**Why anticipation stays honest.** Generalize against the instances we have; never stub the ones we don't. Discrete files hold real code — spine, bole body, reliquary body, shared library — with no placeholder files or recipes for the unbuilt kinds. Generality is proven by two bodies riding the spine, not by placeholders.

**Why we synthesize cloud reuse rather than adopt a vendor mechanism.** Cloud Build offers no native step reuse (no anchors, no includes). Its blessed mechanism — custom builder images — conflicts with the established posture that the cloud is just labor and the Director controls what enters the build: a custom builder holding our logic would be a new vouched supply-chain artifact, the opposite of inline-from-repo. So composition is a deliberate membrane, and it aligns with the existing stitch vocabulary rather than coining a competing term. Recorded in RBSCB.

**Why the metadata token stays, in-memory.** Manual metadata-server token fetch is the canonical skopeo-to-GAR authentication; docker steps auto-auth, but skopeo cannot use the gcloud helper, and docker-credential-gcr would graft a docker-ecosystem dependency into the skopeo container for the same token, with documented friction. The token is correct and irreducible; only the copy-paste is wrong. Invariant: /workspace carries non-secret data only — secrets are step-local and re-minted. Recorded in RBSCB.

**Why the contract plus validator are the load-bearing artifact.** Reuse runs over two channels: a composed code preamble (in-memory, e.g. the canonical token, authored once) and /workspace shared registers (non-secret data, with explicit per-step requires/provides). The disease being cured is undocumented-pattern-by-example; the cure is a documented contract plus the validator, so the codebase teaches "compose from the library," not "copy the script." Documentation is part of done, not polish.

## Shape

- **Host capture-assembly spine** — a separate assembly engine that aligns with stitch's approach (merging it with the made-side conjure stitch is horizon, not this heat). Composes a build from a recipe (ordered step IDs) plus a substitutions blob, submits, polls, extracts capture-files. Owns "submit these steps with this blob, poll, write capture-files"; never knows origins/stamps/tools.
- **Shared cloud step-library** — steps named by function (not family-plus-number), each with an explicit requires/provides contract over /workspace plus substitutions plus buildStepOutputs. Genuinely-shared steps unified (skopeo-capture, vouch-push); kind-specific steps stay specific (inscribe's docker-mirror differs from the skopeo capture).
- **Composed code preamble** for in-step cross-cutting (the canonical token fetch, authored once).
- **Dispatch-time recipe validator** — the declared behavior-adding sliver; cheap guard against the expensive build; carries its own tests.
- **Shared lifecycle helpers** — divine (enumerate) and banish (delete) are direct GAR-REST host ops, NOT build-composition; they are a distinct dedup target (shared REST helpers), kept separate from the assembly spine.
- **Per-kind bodies** — bole ensconce and reliquary inscribe as thin recipe plus substitutions plus intent, riding the spine.
- **Discrete files** — spine module, bole body, reliquary body, shared library.

## Scope

**In**: the capture plus vouch machinery — host spine, the cloud capture/vouch steps for ensconce and inscribe, and the vouch-push step unification (which legitimately touches one made-side step, the hallmark-verify vouch-push).

**Out**: the conjure build pipeline (rbgjb) and the about/syft pipeline (rbgja); the vouch verify steps (download-verifier, verify-provenance) — only the vouch-push step is in; any new Lode kind; any behavior change beyond the declared validator; tool swaps (skopeo to crane); custom-builder-image migration; codebase-wide token dedup beyond the capture family; merging the capture spine with the made-side conjure stitch (horizon).

**Locked — inscribe is unified.** Inscribe's machinery is unified onto the spine behavior-preservingly (it keeps its current namespace and cohort behavior). This is not double work: ₣BH's later cutover re-points inscribe's recipe data, not its machinery; the thin body persists. The payoff is two validating callers over the spine rather than one.

**enshrine is reference, not migrated.** Because enshrine is deleted at ₣BH's bole cutover, the refactor leaves its cloud step's copy in place rather than migrate doomed code; the capture-cousin dedup is therefore deliberately partial until ₣BH removes enshrine.

## Verification

Cheap inner loop: shellcheck plus targeted fixtures, plus the validator's own unit tests. Milestone gate: full skirmish plus the service lode-lifecycle fixture, green with artifacts and provenance verdicts identical to pre-refactor. Operational precondition for the gate: live GCP credentials and a standing canonical depot must be provisioned — the gate is not free-standing. The roughly 100-minute skirmish cook is a milestone, not a per-commit cost; the validator catches composition errors before the cook, but logic regressions still require it.

## Relationship to ₣BH

This heat is ₣BH's prerequisite: its kind verticals attach as thin bodies once the spine and library land. Ordering matters where they touch the same capture step — the refactor lands first, so ₣BH's collision-guard work then targets the unified step rather than the pilot's fork. ₣BH's scaffold correspondingly shrinks to building kind bodies, the cutovers, the spec, and onboarding.

## Open items to mint at first mount

Spine module name, per-kind body file names, and the shared-step namespace (which breaks the rbgj family-letter scheme, since a shared step belongs to no single family) — mint deliberately, enumerating namespaces per the project's minting discipline. Also: whether the normative step-contract (requires/provides plus validator behavior) gets its own spec letter or a section in the Cloud Build JSON trade study.

## Capstone — harvest CBG (Cloud Build Guide)

A required output of this heat, produced at landing by distilling the patterns this refactor establishes — not a speculative pre-write. CBG fills the same slot for the Cloud-Build-container stack that WSG fills for the ssh-to-Windows stack: a foreign-environment sibling to BCG, sharing its philosophy (crash-fast, no silent failures, load-bearing complexity, interface contamination) but diverging entirely on mechanics. CBG operates at single-script precision — it governs the inside of one cloud step, NOT component wiring or architecture; those lessons live in the spec and this paddock.

Contents are the cloud dialect's per-script discipline: crash-fast in the cloud idiom, the metadata-token preamble, JSON-construction rules (when hand-rolled printf is acceptable vs when jq is required), substitution naming and double-dollar escaping, idempotency under Cloud Build retry, the pure-logic/network-op split that makes a step unit-testable, and the /workspace constraints — foremost what must NOT go in it (secrets; the invariant recorded in RBSCB). CBG points at the executable contract and validator rather than restating them.

The guide is only as good as the code it distills, so the spine, library, and per-kind bodies are written as its exemplars — quality now is what makes CBG worth harvesting. Name CBG chosen; confirm against the BCG/RCG/WSG sibling set at mint, and read WSG before authoring so the sibling framing rests on the document, not its label.

## Done

Bole ensconce and reliquary inscribe ride one host spine and one shared cloud step-library with explicit per-step contracts; the canonical token is composed once (no copy-paste); the dispatch validator is in place with its own tests; full skirmish plus service pass with artifacts and provenance verdicts identical to pre-refactor (provenance-insensitivity-to-restructuring confirmed); no new Lode kinds exist; the rationales are recorded in the Cloud Build spec — posture in RBSCB, the composition/contract decision in RBSCJ; and CBG (the Cloud Build Guide) is harvested from the patterns this heat establishes — see Capstone.

## References

- ₣BH — the Lode kind universe heat this unblocks.
- RBSCB CloudBuildPosture — holds the skopeo-token/credential-helper and capture-duplication posture.
- RBSCJ CloudBuildJson — JSON-composition trade study; eventual home for the spine/contract decision (currently trigger-era stale — refresh when touched).
- WSG WindowsScriptingGuide — the foreign-environment sibling guide to BCG (ssh-to-Windows stack); structural precedent for CBG, to be read before CBG is authored.
- The bole pilot, the reliquary and enshrine predecessors, and the cloud step family.