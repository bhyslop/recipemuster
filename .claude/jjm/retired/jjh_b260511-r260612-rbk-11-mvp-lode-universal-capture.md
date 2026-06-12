# Heat Trophy: rbk-11-mvp-lode-universal-capture

**Firemark:** ₣BH
**Created:** 260511
**Retired:** 260612
**Status:** retired

## Paddock

## Vocabulary

The noun system. Three roles per side; all primary words are mutually **independent** — no word is a morphological derivative of another, matching the made-side discipline of `conjure` / `ark` / `hallmark`.

| Role | Made side | Fetched side |
|------|-----------|--------------|
| Recipe / intent (authored, reused) | Vessel | *(no unified noun — distributed per kind; see premise)* |
| Package (GAR substrate, atomic-delete unit, ~internal) | Ark | **Lode** |
| Identifier (the handle a consumer pins) | Hallmark | **Touchmark** |
| Capture/build verb (produces the package) | ordain / conjure | **per-kind** — `ensconce`/`conclave`/`underpin`/`immure` (NOT one verb; `enshrine` retired) |
| Show verb (read-only Lode enumeration) | tally / audit | **`divine`** (`rbw-ld`, lowercase) |
| Inspect verb (read-only, single Lode) | plumb | **`augur`** (`rbw-la`, lowercase) |
| Delete verb (acts on the package) | abjure | **`banish`** |

- **Capture is per-kind, not one verb.** `enshrine` does not survive — the made side is `conjure`/`ordain`, the fetched side forks per kind. Each kind gets its own verb so an operator speaks one word and means exactly one thing, with no kind-argument indirection. Per-kind *names* do not become per-kind *code paths* because the unification lives off the verb name, on the **capture-file + the `Lode` noun** (the shared *output*): every per-kind verb emits the same-shaped provenance fact-file into the same package noun. That sameness *is* "one shape," and it is the guard against the parallel-infrastructure the model rejects. The fetched verbs sit in an occult **evocation/binding-of-the-foreign** register — *call forth what already exists elsewhere and bind it in our hold* — deliberately distinct from the made side's **conjuration/creation** (`conjure`/`ordain`); the split is audibly *create* vs *summon-and-bind-the-foreign*. The register is distinctive on purpose: the plain custodial words (`bank`/`vault`/`harbor`/`garner`) stay free for other kits, and this codebase is already a grimoire (`scry`/`ifrit`/`conjure`/`pentacle`/`sigil`), so the occult register is native. Verbs and their kinds:
  - `bole` → **ensconce** (`rbw-lE`) — the foundation settled securely in reserve
  - `reliquary` → **conclave** (`rbw-lC`) — convene the date-cohort
  - `wsl` → **underpin** (`rbw-lU`) — the substrate everything rests upon (chosen for meaning-clarity over the shorter dock-words moor/berth/harbor; the rootfs literally underpins the whole Windows-workload stack)
  - `podvm` → **immure** (`rbw-lI`) — wall in the opaque VM blob; **one verb spanning both quay families** (`machine-os-wsl` / `machine-os`) via an archive argument, not two verbs
- **`banish`** names the whole-Lode delete (parallel to made-side `abjure`). Banishment is the demonological counter-rite to summoning — cast *out* the foreign spirit you had bound — which fits a Lode (foreign captured bytes) better than a generic delete word, and reads cleanly against the made side: **ab**jure (swear *away* what we made) ‖ **banish** (cast *out* what we captured). Shared `packages delete` backbone with `abjure`, distinct verb/colophon/spec. `jettison` keeps its narrow per-member meaning on both sides — **no double duty**; whole-package delete is `abjure`/`banish`, single-member is `jettison`.
- **`divine`** (`rbw-ld`, **lowercase** — read-only) names the **show/enumerate-the-Lodes** op — *divination as occult observation*: read what has been bound into the hold without touching it, the perceiving counterpart to the binding verbs. It is **lifecycle-grain** (enumerate Lodes by touchmark), which is exactly what the `l` family is for — so it does **not** contradict the member/tag-grain maintenance backdoor (`audit`/`rekon`) that stays in `rbw-i*` (see *Carried forward*); the split is *grain*, not redundancy. Per-Lode member + `:rbi_vouch` *inspect* detail is its own verb — `augur` (below) — not a mode of divine; the two split by **grain**, not redundancy.
- **`augur`** (`rbw-la`, **lowercase** — read-only) names the **inspect-one-Lode** op — *augury as reading the entrails of a single capture*: its member tags and decoded `:rbi_vouch` envelope. The perceiving counterpart to `divine` at single-Lode grain — divine surveys the whole hold, augur scries one deposit — and the fetched-side parallel of made-side `plumb`. Lowercase: it touches no GAR/cost state.
- **Lode command family is `l`; spec is `RBSL` (`L` verified free).** `l` is an *artifact-keyed* command family — a departure from the place-keyed scheme (Crucible/Depot/Foundry), pioneered fetched-first; **Ark earns its own family (`k`) in the sibling made-side retrofit heat**, closing the package parallel. Colophon **capitalization is load-bearing**: a capital letter marks an op that touches persistent GAR/cost state in *either* direction (create *or* destroy) — so the capture verbs and `banish` are all capital (`rbw-lE/lC/lU/lI` capture; `rbw-lB` banish); read-only and local-only ops stay lowercase (`rbw-ld` divine enumeration, `rbw-la` augur inspection).
- **`Lode`** names the **package** (Ark-parallel). The ore-deposit sense fits a stored body of bytes, and it gives the package an *independent* word that owes nothing to `enshrine`, preserving verb/noun independence.
- **`Touchmark`** names the **identifier** (Hallmark-parallel). The shared `-mark` suffix with `hallmark` is a deliberate **role-signal**: any `*mark` is a specific-instance handle; the first syllable carries the side (`hall-` = what we forged, `touch-` = what we captured). `mint`/`mintmark` rejected — `mint` is load-bearing as the meta-verb for naming.
- **`enshrinement`** does not survive as a quoin — with capture forked into per-kind verbs and the package named `Lode`, no slot remains for the noun. The `rbi_es` namespace and the `*Enshrinements` tabtargets re-orient to Lode in the rename map.

## Premises

This section exists so a returning session can re-orient against the *why* of the model, not just the *what*. The choices below are load-bearing. When this heat mounts, everything here is re-investigated — these premises are the starting frame, not a frozen contract.

**Why this heat exists.** Four capture needs share gestalt: the existing narrow base enshrinement, the existing reliquary tool-mirror operation, WSL substrate (the admin-distro replacement needed to take caparison-w off Microsoft's mutable appx supply), and podman VM (defensive mirror against quay's hostile lifecycle — new images every few hours, retention measured in days). The rule-of-three makes the unification load-bearing rather than premature — the unification of the *noun and capture-file*, not the verb (the capture verbs fork per kind; see Vocabulary). The reliquary tool-mirror belongs because its gesture (pull upstream → store in GAR with provenance) is identical to the rest; excluding it would be arbitrary.

**Why "Lode" names the package, not the recipe.** All capture cases share one capture-file shape, and the realistic alternative is parallel per-kind *infrastructure* — separate pipelines, nouns, and provenance per kind; unifying the *infrastructure* (one Lode noun, one envelope shape) is the actual simplification, not a forced one. (Per-kind *vocabulary* — the verbs themselves — is a separate choice and is compatible: the shared capture-file + Lode noun is what keeps per-kind names from regressing into per-kind code paths. See Vocabulary.) But the unified noun belongs at the **package** layer, not the recipe layer: what one verb genuinely produces across heterogeneous kinds is one *stored, provenance-wrapped GAR package*. Kinds carry the cardinality (1 vs N members), so one package noun loses no precision. The ore-deposit semantics and the verb/noun-independence discipline both place Lode in the Ark-parallel package slot, not the Vessel-parallel recipe slot.

**Why there is no unified recipe noun — intent forks per kind, as consumption does.** The natural owner of "what to capture" differs by kind: a base image's identity is intrinsic to the **vessel** built from it; the reliquary toolset is a build-infrastructure constant; wsl/podvm versions are host-substrate config. Forcing these into one recipe artifact is a false unity — the Load-Bearing Complexity smell, and it cuts against the consumption-forks premise. It works *without* a unified recipe because **enumeration lives on the artifact side**: "show me everything we capture" is an audit of the GAR Lodes by touchmark, never a recipe file. The kind-gated-variable-group *pattern* (RBRV gating on `RBRV_VESSEL_MODE`) still applies within any regime that happens to hold several kinds; only the unified *file* dissolves.

**Why the Director dictates the full version closure via the nameplate — "works for me" is a versioning failure.** Substrate touchmarks (`wsl` / `podvm`) are Director-pinned at the nameplate and materialized at charge, never left to Retriever or local-station choice. This is the fetched-side voicing of the system's existing anti-ambient-state ethos — cloud-side acquisition, content-addressed anchors, the depot tripwire, SLSA vouch: a result that depends on whatever the operator happened to have installed is a failure of strong versioning, not a convenience. The nameplate becomes the complete Director-dictated runtime closure (`hallmark + substrate touchmarks + network/ports`); the Retriever *receives* pinned versions rather than choosing them. The principle reaches the full **Lode closure** — everything capturable as a Lode (the podman-machine image, the WSL rootfs) — and stops at the **Pale**: the bare host OS, kernel, and hardware that *run* WSL and podman are a neighbor, not a subject. State the boundary so the principle does not over-claim control of the metal beneath it.

**Why acquisition runs cloud-side, never the workstation — co-equal with no-FQIN for regression-proneness.** Every kind fetches its bytes over the network from within GCP (crane / curl on Cloud Build, egress-permitted pool), trusting the GCP environment and network over the operator's potentially-compromised workstation. The WSL rootfs is fetched from the vendor's published HTTPS endpoint (Canonical cloud-images — the distro tarball the Store appx merely wraps) and verified against a published checksum; it is NOT exported from a workstation-installed distribution. The podman-VM prototype's "ignite VM" (a disposable podman machine that existed only to host crane because the workstation lacked it) dissolves entirely: Cloud Build carries crane natively. If a future pace proposes touching the workstation to *acquire* bytes, the trust model is being violated — the workstation only ever *consumes* already-verified artifacts.

**Why the provenance envelope is the true unifying invariant — not the storage shape.** What justifies one package noun across heterogeneous bytes is the acquisition-provenance envelope every instance carries: source coordinate, upstream digest/checksum, acquired-at, acquiring identity (the Cloud Build SA), and a signed vouch. This is the fetched-side parallel to the made-side SLSA — and it is a *different claim*: capture-fidelity ("we faithfully copied upstream X at digest D"), not build-integrity ("we built this securely"). The envelope has two honest trust grades, declared per kind: *verified-against-published* (bole/reliquary/wsl, where upstream publishes a checksum to verify before vouching) and *recorded-at-acquisition* (podvm, where quay publishes no stable checksum, so you attest the digest you captured — trust-on-first-acquisition is the best the hostile upstream permits). The prototype's brand-file (origin / fqin / digest / identity) is this envelope in embryo; the model promotes it from an unsigned text file written at consumption time to a signed artifact emitted at acquisition time. Compound provenance is batch-level (the cohort as a whole); members stay individually manipulable for post-bug cleanup but need not each carry independent attestation.

**Why GAR's `package` is the substrate AND the atomic-delete unit.** Each Lode is one GAR `package` — this is literal, since Lode *names* the package. `packages delete` removes the package and all its member versions in a single operation (verified against GAR's delete granularities: package / version / tag). That gives Director administration atomic whole-Lode delete for free, and per-member cleanup (`versions delete`, `tags delete`) for fixing bugs — with no custom reference-counting, because GAR content-addresses blobs and garbage-collects unreferenced ones. The consequence that reshapes the model: **single and compound Lodes are the same shape — a cardinality attribute, not a structural distinction.** A single-image Lode is a 1-member package; a reliquary or podvm cohort is an N-member package. Same delete primitive, same provenance attachment point, same wire shape. There is no separate refs object — so the "interior-hole / dangling-reference" regression class evaporates. Pre-deployment, layout is free to choose: members live as versions/tags *within one package*, not as sibling image-names, because GAR has no subtree delete — one package is what makes the single-call atomic delete possible.

**Empirical correction to the atomic-delete premise (2026-06-09; the delete mechanism re-corrected from a live probe).**
GAR will not delete a child manifest while its parent index still exists: a delete of a referenced child returns FAILED_PRECONDITION ("manifest is referenced by parent manifests"), and a single `packages delete` of a multi-arch web removes NOTHING — the package survives whole.
An earlier reading of this same debris guessed an LRO terminating NOT_FOUND (code 5) "even when the delete effectively completed"; a direct probe disproved it — the package does not delete at all under one call (characterized at commit 619882ee2; the standing 55-version reliquary web was the corpse).
This is GAR's documented parent-before-child behavior, and the reason Google's own gcr-cleaner ships a `--skip-errors` flag rather than ordering the topology.
Atomic whole-Lode delete therefore holds for flat packages but NOT for index-webs, which require convergence.
conclave now captures single-platform (`--platform linux/amd64`) to dodge the web — conclave-only; bole/wsl keep full-fidelity capture and so index-shaped Lodes remain a live shape.
The cloud-dispatch delete path (landed) deletes by convergence: each round it fires a delete at every remaining version (force=true) and the package shell, skips the per-round "referenced by parent" preconditions, and polls the package GET until 404 — absence the only truth, a deadline the only failsafe, the same shape as host `rbuh_poll_until_gone`.
Live-proven on two index-web reliquary banishes and a six-package conjure-hallmark abjure; canon in RBSCB.

**Why tool-plane GAR deletes dispatch cloud-side (cinched 2026-06).**
`banish` and `abjure` move off host-issued trust-200 REST onto cloud-side delete-builds:
the workstation dispatches and blocks, conjure-shaped; the in-pool step deletes by convergence (see the empirical-correction premise) and verifies absence, so the build outcome IS the delete outcome.
The line is tool-plane vs control-plane: GAR package deletes go cloud; SA / project / depot / lien / bucket deletes stay host-side REST.
The raw maintenance backdoor (see *Carried forward*) deliberately stays host-side with honest LRO handling — a cleanup tool of last resort must not depend on the pipeline it cleans up after.

**Why the Vessel:Ark :: Hallmark:Touchmark analogy is load-bearing — and where it deliberately stops.** The analogy runs three-role: Vessel : *(distributed recipe)*, **Ark : Lode** (package), **Hallmark : Touchmark** (identifier). It is the orientation handle for acquisition, GAR-substrate, and delete questions: "how should operation X work for Lodes?" usually answers to "find X for Arks/Hallmarks and transfer." **The analogy does NOT extend to consumption.** A made image (Hallmark) and a fetched artifact (Touchmark) are consumed by entirely different actors — and a felt asymmetry *there* is real, not a sign of asking the question wrong. The distrust-any-asymmetry instinct is over-broad: the consumption asymmetry is genuine and expected.

**Why consumption stays per-kind and out of Lode's scope.** Lode unifies acquisition, the GAR substrate, provenance, and delete. It does not unify consumption: a made image is consumed as a Cloud Build FROM-line or build-mount; a `wsl` instance by the host's `wsl --import`; a podvm instance by the host's `podman machine init` — some consumers are not even on Cloud Build. Lode's value at consumption is not one verb; it is that the bytes the host consumes are cloud-acquired, verified, and vouched instead of locally-exported and uncontrolled. **Touchmark election co-locates with consumption and forks the same way** (see Touchmark Election). The `wsl` seed is the sharp case: today it is born from `wsl --export` of a Store-installed distro (the corruptible-workstation path); under this model the workload's `wsl --import` consumes a verified seed pulled from GAR — same consumption verb, trusted seed. A future session must resist inventing a *consumption-verb* unification (a "yoke for hosts"); the consumer asymmetry is intended — but note this is distinct from the determinism premise, which *does* unify the *authority* (Director, via nameplate) over substrate election.

**Why registry-level operations get parallel cult verbs per side, not shared verbs.** Made-side `abjure` and fetched-side `banish` share one registry primitive (`packages delete`), but the noun-split keeps them as parallel verbs — distinct name, colophon, spec, and surface vocabulary, with the shared mechanism living in registry-utility code. The analogy carries through verbs; it does not collapse at the substrate. A future session tempted to unify because "the code is identical" should resist — the parallelism is load-bearing for the model even where the implementation is not.

**Why capture and election are separate layers — no mixing of the GAR copy with the install into a final resting place.** Capture (an `ensconce`/`conclave`/… verb) is **capture-pure**: it mints the Lode, emits the signed provenance envelope, resolves and *reports* the touchmark — and writes **no consumer config**. Election (where a consumer pins a touchmark) is a separate consumption-side layer that *reads* what capture emitted. The interface between the layers is the **capture-file**: one provenance fact-file per capture, in the existing `rbf_fact_*` presence-as-fact idiom (NOT one mega-JSON, NOT per-aspect explosion — one file per captured touchmark, the roster precedent). It is the brand-file-to-signed-envelope promotion named in the provenance-invariant premise, here given its concrete handoff role. The payoff is concrete: election *chains* off the capture-file, retries independently, and **survives an election failure** (the Lode + file persist); and a fan-out election (one touchmark → N vessels) iterates by *reading the file*, not by a verb taking N vessel arguments. The made side already half-violates this for ergonomics (the kludge cycle drives a hallmark into a nameplate in one gesture) — so the rule is at the **verb/spec** level (`ensconce` etc. stay pure); a *cycle tabtarget* may still bundle capture+elect for dev ergonomics, exactly as kludge does. Bundling lives in `tt/`, never in the verb's meaning.

**Why the per-kind intent carries no FQIN — most regression-prone principle.** When quay, docker-hub, or Canonical reorganizes upstream paths, kind-pipeline code changes; the per-kind intent declaration does not. If a future pace proposes adding resolved-coordinate detail to a kind's intent file, that is a smell — the principle is being violated. RBSRV's ORIGIN/ANCHOR pattern is the precedent: ORIGIN as author intent, ANCHOR as resolved-by-pipeline (and, on the fetched side, the resolved ANCHOR *is* the elected touchmark). The Lode model generalizes that pattern, not weakens it. Each kind's upstream-source convention lives in pipeline code where it can be fixed in one place.

**Why reliquary survives as a kind but not a top-level noun.** Vessels constrain Cloud Build toolsets by date-cohort identity; the cohort concept itself is empirically load-bearing — removing it would dissolve a real capability. But the project's cult-vocabulary count is high, and reductions are precious. Demotion to a single kind enum slot preserves the cohort semantic while dropping one top-level noun. Keep the word, drop the weight. Reliquary must not be promoted to a universal grouping abstraction — that would make a cult noun *more* central, the wrong direction. The old "reliquary stamp" was a separate identifier only because reliquary was its own noun — it is now simply the **touchmark** of a `reliquary`-kind Lode. No distinct cohort-identifier survives.

**Why five kinds, not four.** Podman-vm fan-out is two-family at quay (`machine-os-wsl` for the Windows-via-WSL host path, `machine-os` for the macOS/Linux native host path) with asymmetric host-platform consumer paths (caparison-w for the WSL flavor; nothing analogous for native). That single split — the one expansion past the four founding needs — honors structural truth, not classification preference. The `reliquary` kind absorbs the tool-mirror operation as its date-cohort case; a separate single-tool kind was **dropped as non-load-bearing** — it split from `reliquary` on cardinality alone, has no consumer (tool images are only ever consumed as a co-versioned cohort), and `bole` already proves single-image-in-one-package. The standing rule it leaves: kinds earn separation on **acquisition / payload / consumer**, never on cardinality — the package layer already carries 1-vs-N for free.

**Why podvm kinds retain selectively, not in bulk.** The upstream manifest lists at quay span 5-15 GB across all host platforms (architecture × disktype × family). Project storage cost scales only to selected variants — a compound Lode's package holds exactly the curated member set. The prototype already commits to this shape via `crane manifest` and `crane blob` digest navigation; the model preserves it. Trade-off worth knowing: widen the captured platform set defensively at acquisition time, because quay's hostile retention can make later expansion impossible once an upstream version ages out. The pipeline should support "expand selected set against same upstream version" as a first-class refresh mode, separate from "bump to new podman version." Resist regressing toward full-mirror out of a misguided fidelity instinct — cohort identity is project-controlled, vouching attests to the curated subset, and per-platform reproducibility from upstream digests is preserved either way.

**Why generator expressions, not enumerations.** A list of "verbs affected" or "files to change" written today is wrong tomorrow — every unrelated touch ages it. A discovery recipe survives those churns. Future mount sessions should resist expanding recipes into lists until paces actually approach the work. Paddock complexity compounds, and complex paddocks become unread — which defeats their purpose. Discipline, not preference.

## Shape

Broaden the existing narrow capture — the base-OCI-mirror operation `enshrine` performs — into project-controlled capture of upstream artifacts across heterogeneous content kinds — base images, build-time tools, WSL substrate, and podman machine images — via **per-kind capture verbs** (`ensconce`/`conclave`/`underpin`/`immure`, NOT one verb; the `enshrine` verb itself retires). What unifies them is the *output*, not the verb: a single **package** noun, **Lode**, paralleling Ark on the made side, plus a single capture-file envelope shape; each Lode is referred to by its **Touchmark**, paralleling Hallmark.

The analogy (acquisition / substrate / delete only — see premises):

| Role | Made side | Fetched side |
|------|-----------|--------------|
| Recipe / intent | Vessel | *(distributed per kind — no unified noun)* |
| Package | Ark | **Lode** |
| Identifier | Hallmark | **Touchmark** |

Each kind's capture pipeline, running on Cloud Build, resolves upstream coordinates from kind conventions, fetches bytes from within GCP, verifies them (against a published checksum where the upstream offers one), captures them into a GAR package (the Lode), and emits a signed provenance envelope as the per-capture fact-file — writing no consumer config (capture-pure; see the no-mixing premise). **Intent declarations are distributed, not unified:** each kind's "what to capture" lives in its naturally-owning regime — `bole` in the vessel (`RBRV_IMAGE_n_ORIGIN`), `reliquary` as a build-infrastructure constant, `wsl`/`podvm` in host-tier config. Wherever intent lives it stays declarative-only — no FQIN, no resolved-identity field; kindle code computes upstream coordinates and GAR tags from primitives at module-startup, mirroring the prototype's `ZRBV_VMIMAGE_TAG_PREFIX` pattern.

Scope is the fetched side only. The same package / atomic-delete / provenance-envelope model is mechanically true of the made side (the Ark/Hallmark already are this), and the symmetry is confirmed (`abjure` is essentially an Ark `packages delete`) — but retrofitting made-images is left to a sibling heat to keep this one bounded.

## Touchmark election

A touchmark is *elected* where a consumer pins a specific captured deposit to use. Election is a consumption-side act, so it forks per kind — and it co-locates with intent in the same naturally-owning regime. The fork tracks **build-time vs runtime**, which maps cleanly onto **vessel vs nameplate**:

| Kind | Consumed | Elected in | Mechanism |
|------|----------|-----------|-----------|
| `bole` | build-time (FROM) | vessel (the ANCHOR slot, renamed) | **derived — the vessel *pulls* the resolved coordinate** from the capture-file; the capture verb writes no vessel config (no mixing) |
| `reliquary` | build-time (syft/skopeo) | vessel (the RELIQUARY slot) | **yoke stamps it** across all vessels |
| `wsl` | runtime substrate | **nameplate** (Director-dictated) | charge-time provision (deferred) |
| `podvm-*` | runtime substrate | **nameplate** (Director-dictated) | charge-time provision (deferred) |

The election *mechanism* differs even where the *location* is the same — `bole` election is **derived-pull** (the vessel reaches for the resolved coordinate the capture-file reports; capture stays pure and writes nothing), `reliquary` election is an **explicit yoke-stamp** across vessels — and the rename map must preserve that distinction (do not collapse derived-pull-ANCHOR and yoke-stamp-RELIQUARY because both land in the vessel). Nameplate is **not** a touchmark site for the build-time kinds; for the runtime-substrate kinds it *is* the election site, under Director authority, per the determinism premise. The charge-time provisioning mechanism for substrate touchmarks is consumption-side and deferred this heat (see Heat nature).

## Lode registry layout & naming (locked)

GAR category is **`rbi_ld`** (2-char, consistent with `rbi_hm`/`rq`/`es`/`df`). One Lode = one GAR `package` named **`rbi_ld/<kind-letter><stamp>`**, and that package *is* the atomic-delete unit. Kind-letters: `b` bole, `r` reliquary, `w` wsl, `vw`/`vn` podvm-{wsl,native} (`v`=VM, echoing the retired `rbv_PodmanVM.sh`; `b=bole` shares a letter with hallmark `b=bind` but the namespaces are separate, so it's accepted). Stamp matches the hallmark second-granular form (`YYMMDDHHMMSS`).

Members and provenance ride as **tags within that one package — never `/`-path-segments.** GAR has no subtree delete, so a slash makes sibling packages that can't be removed atomically (the exact regression today's reliquary `rbi_rq/<date>/<tool>` layout suffers). `packages delete rbi_ld/<kind><stamp>` removes the whole Lode in one call; per-member cleanup is `tags delete` / `versions delete`. ("Atomic" = single operation, not transactional rollback; partial-delete fixtures still assert member absence.) If GAR ever refuses a single-call delete of a mixed-artifact package, the fallback is a GCP-run delete loop over the package's tags — effectively atomic for our purposes. (This condition has now fired for index-webs — see the atomic-delete correction premise.)

**The sprue.** `rbi_*` is RB's reserved tag prefix; the Director's semantic names take everything else (RB refuses Director tags beginning `rbi_`). The sprue marks strings from **RB's domain** — RB's authored lexicon (`bole`, `vouch`, `sha256`, the reliquary tool words) **and** RB-measured-from-content values (the digest). It does **not** mark **foreign-cued** strings — anything derived from the vessel ORIGIN, the nameplate, or Director input. Hence `rbi_<name>` is forbidden (the name is a vessel cue) while `rbi_sha256-<digest>` is fine (the digest is RB-measured).

Member tags:

- **bole** (singleton): `:rbi_bole` (uniform greppable handle) + `:rbi_sha256-<full-hex>` (canonical OCI digest — matches what every tool reports, exact cross-Lode dedup) + `:<sanitized-origin>-<sha10>` (UNSPRUED — origin is a vessel cue; name + glance-fingerprint, = today's enshrine anchor, so cutover is a near-rename) + 0..N Director semantic names (unsprued) + `:rbi_vouch`.
- **reliquary** (cohort): `:rbi_<tool>` per member + `:rbi_vouch` (closed RB vocabulary; no Director/digest layer — kept clean).

**Provenance envelope (`:rbi_vouch`).** One per Lode, batch-level — the prototype brand-file promoted from a consumption-time text file to an acquisition-time artifact. One canonical home: the in-GAR `:rbi_vouch` tag (the Lode self-describing in the registry). The host-side capture handoff that election's derived-pull reads now emits only bare single-form chaining facts (a touchmark value-fact + a kind-brand enum) — it carries no provenance content, so reading host-side provenance now costs a GAR call (accepted; see the fact-chaining premise). Fields: lode identity (kind/package), `acquired_at`, `acquired_by` (Cloud Build SA), `capture_build` (the capture's own provenance), `trust_grade`, and `members[]` each carrying `name`/`origin`/`digest`/`verification`/its assigned `tags` — so the Director's naming is itself attested. `members[]` is the cardinality axis (length 1 for singletons, N for cohorts — same shape). Near-term unsigned (`signature: null`) but `schema`-versioned; future signing via cosign or the OCI referrers API (reserved-tag now, referrers once GAR maturity is confirmed). Exact field serialization is pace-time, firmed against `fast`-test fixtures — the shape above is the lock.

**Two trust grades, declared per Lode — Pale honesty, never over-claim:**

- **verified-against-published** (bole/reliquary/wsl): bytes re-checkable against a *persistent* upstream — OCI content-address on a durable registry, or an out-of-band published checksum (Canonical's SHA-256 for `wsl`).
- **recorded-at-acquisition** (podvm-*): upstream offers no durable re-checkable reference (quay rotates podvm out within days), so RB attests only the digest observed at capture — trust-on-first-acquisition.

## Kinds

| Kind | Members | Upstream source | Digest grade |
|------|---------|-----------------|--------------|
| `bole` | 1 | upstream OCI registry, consumed as FROM line | verified |
| `reliquary` | N | date-cohort of build-tool images from OCI registries (replaces today's reliquary noun) | verified per member |
| `wsl` | 1 | vendor-published rootfs tarball over HTTPS (e.g. Canonical cloud-images) | verified |
| `podvm-wsl` | N | `quay.io/podman/machine-os-wsl` — platform fan-out, consumed by Windows podman | recorded |
| `podvm-native` | N | `quay.io/podman/machine-os` — platform fan-out, consumed by macOS/Linux podman | recorded |

Capture verb per kind (all capital-colophon `rbw-l*`, GAR-blob creators): `bole`→`ensconce` (`lE`), `reliquary`→`conclave` (`lC`), `wsl`→`underpin` (`lU`), `podvm-wsl`+`podvm-native`→`immure` (`lI`, one verb, quay archive as argument). Five kinds, four capture verbs.

Member count is the only structural variable — every kind is "a GAR package holding 1..N members plus batch provenance." `wsl` and `podvm-wsl` both touch WSL but are entirely distinct: `wsl` is the Linux distro rootfs that hosts everything on the Windows workload, `podvm-wsl` is the podman machine image that runs *inside* that distro. Names kept visibly different.

Payload shapes (orthogonal to member count): native layered OCI image (`bole`, `reliquary`) vs opaque-blob-wrapped-as-OCI (`wsl`, `podvm-*` — a rootfs tar or disk blob in a scratch wrapper). The provenance envelope rides uniformly on both.

## Capture-tool posture (crane embrace) — locked

crane is the **sole image/registry tool** across every capture kind; skopeo, docker, and buildx are evicted from the capture path (separate eviction paces, bole first). The irreducible floor is three non-overlapping tools: **crane** (every registry/image op — `cp` by digest, `tag`, `append` to wrap an opaque blob), **curl** (non-registry HTTPS fetch — OAuth token, vendor rootfs tarball, the blob-residency HEAD guard), and **gpg** (vendor signature verify). All three are daemonless static binaries — the consolidation also removes the docker daemon from the capture path.

- **oras** is documented equal-fidelity fallback only, not wired. The one future that reopens it: signed provenance via the OCI **referrers** API (oras is its reference implementation) — a *named* fork, never a silent inheritance of the crane pick.
- **skopeo's rule-out rationale is corrected** (cerebro 2026-06): it fatals *loud* on empty-config OCI artifacts (`oci.empty.v1+json`), not the silent foreign-layer skip first assumed — the disk blobs are `application/zstd`, distributable.
- **crane auth — resolved: gcrane's ambient Google keychain.** Plain `crane` has no per-command creds and reads only the docker-config keychain; the registry steps instead invoke **`gcrane`** (crane's Google-auth sibling, identical `cp`/`manifest`/`tag` engine) from `gcr.io/go-containerregistry/gcrane:debug`, whose `google.Keychain` matches `*.pkg.dev` and draws ADC → GCE-metadata-server credentials — so it auths GAR ambiently as the Mason SA with **no `crane auth login`, no credential-helper image, no in-memory token-fetch, no new IAM/repo grant**. This generalizes conclave's existing ambient docker auth (`gcr.io/cloud-builders/docker`) to crane; the durable canon lands in RBSCB. (curl still mints a metadata token for the non-registry GAR blob-residency HEAD guard — curl's floor role, not gcrane's.)
- **skopeo eviction is total, including the cohort.** The made-side bind mirror (`rbfd_mirror` / `rbgjm01`) is the last skopeo holdout — a bare `skopeo copy --all` that converts cleanly to `crane cp`. Converting it is a *narrow, deliberate exception* to the capture-only boundary: it touches one made-side tool invocation, NOT the made-image package retrofit (Ark/Hallmark/abjure stay deferred to the sibling heat). Once the mirror is on crane and every capture consumer is gone, skopeo is dropped from the reliquary cohort entirely — the terminal functional pace, just before the vocabulary scrub.
- **Pinning boundary — generation may pull unpinned; sealed-reliquary builds may not (operator rule, 2026-06).** Reliquary *generation* (inscribe/conclave minting the sealed cohort) may pull unpinned upstream — the only phase permitted to. Every build *consuming* a sealed reliquary carries **zero unpinned aspects**: each tool resolves pinned from the cohort, whose durable home is our own Artifact Registry, never gcr.io. Consequence as landed: **gcrane is a reliquary cohort member**, and bole — vessel-adjacent, with a reliquary slot to resolve from — rides the *pinned* gcrane.
conclave rides the floating bootstrap `ZRBLD_GCRANE_BUILDER` by permission (it *builds* the reliquary and so cannot resolve from it).
wsl — and podvm when it lands — are **vessel-less**, so they have no reliquary slot and also ride the floating bootstrap; pinning them is the bootstrap-digest-pin itch (RBS0 `rbsk_pinning_boundary`), accepted and recorded, not a violation. gcr.io is bootstrap-only and non-load-bearing: RB stores everything in Artifact Registry, exempt from the Container Registry shutdown (durable canon + refs now in RBSCB).

## Carried forward / consequences

- **caparison-w / garrison-w for the `wsl` kind.** The workload's `rbtww-main` is registered via `wsl --import` from a seed tarball; today that seed is created by `wsl --export` of admin's installed distro (GarrisonWsl). Under the `wsl` kind, that export path retires — the seed becomes a verified artifact pulled from GAR. The pre-existing revert pace in ₣A- targeting the WSL-stage DEV CACHE shortcut becomes obsolete and wants dropping or transferring when this heat commits to development.
- **Windows onboarding is stale and deferred.** The `rbhw*` Windows handbook tracks (Docker Desktop, context discipline) are stale and incomplete, and `wsl`/`podvm` consumption is deferred — so their onboarding has nothing live to teach. Lean: mark those tracks loudly NOT-available / deferred rather than delete, so the eventual `wsl`-kind consumption work rereads the host-side prose rather than reconstructing it. Onboarding follows *consumption*, not *acquisition*: the Lode acquisition unification does NOT unify the onboarding surface; substrate-kind onboarding lands in the host/Windows tracks, never in the Director build-lifecycle tracks.
- **`inscribe` is verb-overloaded — disambiguate, don't conflate.** `rbfl_inscribe` (reliquary tool-mirror, `rbw-dI`) is **in scope** — the operation absorbed into the `reliquary` kind's capture verb (`conclave`). `rbrd_inscribe` (depot tripwire, `rbw-rdi`) is a **different operation, out of scope, stays put**. There is no `mirror` verb to retire: `rbfd_mirror` is the bind vessel-mode copy (made-side, also out of scope). What is absorbed is `inscribe`/reliquary, not `mirror`.
- **Onboarding rename surface (tri-surfaced vs greenfield).** `bole`/`reliquary` are fully present across bash, adoc, and the Director onboarding tracks (`rbw-Odf`/`Oda`/`Odb`) — for these the cost is *rename* (only two typed linked terms, `RBYC_ENSHRINE` + `RBYC_RELIQUARY`, drive the typed references; the rest is hand-edited prose). `wsl`/`podvm-*` are surfaced *nowhere* today — for these the cost is *greenfield*, not rename. Do not estimate the onboarding work as uniform.
- **Refresh cadence** is per-kind and may diverge (podvm-wsl vs podvm-native).
- **Maintenance verbs stay in the `image` family (`rbw-i*`), deliberately.** `rekon` / `audit` / `wrest` / `jettison` survive intact and do **not** move into `rbw-l*`. They are a low-level backdoor — primitives for cleaning up bad things at the member/tag grain — and that's where they belong; the `l` family is for the first-class Lode lifecycle (capture, the read-only `divine` enumeration and `augur` inspection, and `banish`). The split from the maintenance verbs is **grain, not redundancy**: `divine` enumerates **Lodes** by touchmark and `augur` inspects **one Lode** (both lifecycle-grain, `rbw-l*`); `audit`/`rekon` enumerate **members/tags within** a Lode (maintenance-grain, `rbw-i*`). Their fetched-side domain variants retire in favor of a **path-polymorphic raw layer** (groomed 2026-06-09): three type-blind verbs — list / wrest / jettison — operating on raw GAR paths.
The disambiguation rule is total because GAR's deletable leaves are exactly tags and versions: a parameter carrying `:tag` or `@sha256:` is an image — act on it; anything else is a path — list its children (no argument → the top namespaces; a prefix → its packages; a package → its tags and versions). Discovery by iterative narrowing.
Package-grain delete stays with the semantic verbs (`banish`/`abjure`); the backdoor cuts below package grain only.
It is **envelope-independent by construction** — it must work on half-deleted debris, a corrupt envelope, legacy `rbi_es` artifacts that were never Lodes, and any future kind with no new verbs — and it stays host-side REST with honest LRO handling, never depending on the cloud pipeline it exists to clean up after.
The made-side Hallmark variants are out of scope, untouched — `rekon` carries a semantic canonical-member contract (the hallmark-lifecycle fixture asserts its non-zero exit) that belongs to the made-side retrofit heat.
- **Tabtarget shape is decided, not open:** per-kind capture verbs under the new `l` (Lode) command family — `rbw-lE/lC/lU/lI` capture, `rbw-ld` divine (read-only show), `rbw-lB` banish — superseding the earlier "unified `rbw-dE` with a kind argument" framing. (These move out of the Depot `d` family where `enshrine`/`inscribe`/`yoke` live today.)
- **Election (`yoke` + the deferred substrate verb) lives with its consumer, NOT in `l` — lowercase, and does not commit.** Election mutates the *consumer* (vessel for build-time kinds, nameplate for substrate kinds), not the Lode — the touchmark is its *argument*, the consumer its *target* — so it belongs in the consumer's family, paralleling made-side hallmark election (which lives with the nameplate/kludge machinery, never with ark/hallmark). `l` stays reserved for GAR create/destroy of the Lode itself. Three axes separate election from `l`-capture: it targets the **consumer** (not the Lode); it is **lowercase** (cheap reversible config, not persistent GAR/cost state); and it is **operator-committed** — it writes the config and emits a "commit with your usual workflow" hint, never self-committing. That last is the near-universal RBK convention: `yoke` today is explicit — RBSDY, *"this primitive does not commit; the caller commits"* — and the only RBK bash command that self-commits is Marshal-Zero (`rblm`), a wholesale reset ceremony that still gates a clean tree first. Election also does **not self-gate** on a dirty tree; the clean-tree guard lives at the downstream consumer (the build reading the vessel, charge reading the nameplate — `rbob_bottle` already gates the nameplate clean). Slated as a terminal pace this heat: `yoke` moves out of Depot `d` and de-capitalizes (`rbw-dY` → a lowercase consumer-family colophon); exact destination letter is a layout choice. (Capture verbs raise no git question at all — they write GAR, not git-tracked config.)

## Discovery recipes — work to enumerate when paces approach

- **Verb redesign blast radius.** Scan `Tools/rbk/vov_veiled/RBSA*.adoc` and `RBSI*.adoc` for verbs touching captured artifacts; for each, classify per-kind variance (unchanged / split / retired). Yoke is the headline because consumer-landing differs per kind; other splits expected. Recipe, not enumeration — premature listing ages poorly. (Quoin anchors live in `RBS0-SpecTop.adoc`: `rbtgo_ark_enshrine`, `rbtgo_depot_inscribe`, `rbtgo_director_yoke`, `gar_enshrines_namespace`/`gar_reliquaries_namespace`, `rbst_reliquary_stamp`, `rbf_fact_reliquary` are the capture cluster.)
- **Substrate election landing.** For `wsl` and `podvm-*`, the touchmark is Director-pinned at the nameplate (determinism premise), materialized at charge — but *which* host-tier regime backs the node-side provisioning (station regime vs BURN node profile vs a new host-config regime) is open. Decide when the first non-vessel Lode consumption pace mounts.
- **Spec letter `RBSL`; code-module prefix `rbld` (ratified).** `L` verified free against the RBS* tree. The Lode *code* landed (post-₣BX) as the **`rbld*` family** — `rbld0_cli`/`rbld0_Lode` (CLI + 0-trick gestalt), `rblds_Spine`, `rbldb_Bole`, `rbldl_Lifecycle` — not a single module (constants `RBLD_*`, private `zrbld_*`), completing the system-wide Lode naming set: colophon `l` (`rbw-l*`), spec `RBSL`, GAR namespace `rbi_ld`, code `rbld`. This consciously relaxes the earlier "other than `rbl`" caution: `rbl` is a *pure container* — its only child is `rblm` Lifecycle Marshal, and it never itself names a thing — so terminal exclusivity holds and no coherent theme is diluted; `rbld_` and `rblm_` never collide at the 4-char grep grain. (`rbld_` code vs `rbi_ld` namespace are intentionally close — the module produces the namespace.)
- **GAR layout & provenance attachment — DECIDED** (see *Lode registry layout & naming*): single `rbi_ld`, one-package-per-Lode, members + reserved `:rbi_vouch` tag; `rbi_es`/`rbi_rq` migrate into `rbi_ld` at the deferred cutover. The OCI referrers API remains the future upgrade for signed attestation once GAR maturity is confirmed.

## Test coverage gate before deploy

Lifecycle fixtures against the new Lode surface must be slated and landed before deploy: capture / list / inspect / wrest / delete end-to-end against live GAR per kind, bookended by prereq-sweep at head and muster-absent at tail. The bar forks by kind: `bole`/`reliquary` carry their consumption-adjacent paths; `wsl`/`podvm` stop at the registry — **capture → list → inspect → per-member + whole-Lode delete against live GAR, no host in the loop** (consumption deferred). Whole-Lode delete is a single `packages delete` — the regression risk shifts to the **per-member delete path** (the cleanup-after-bugs case): multi-member kinds must carry concrete member-absence assertions after a partial delete, since that is where a package can be left in a partial state. The fixture surface scales with the operation surface; test paces are not optional polish. Manual operator testing during planning is the bridge, not a substitute.

**Capture-fidelity is a service-tier property, not a fast-tier one — settled on `bole`, holds for every kind.** The recorded digest is measured cloud-side (the in-pool `crane manifest | sha256` step), so an offline/deterministic test cannot verify `recorded == upstream` without either re-fetching upstream (the Pale — nondeterministic) or running the cloud-side script on the workstation (forbidden: cloud and local bash stay unshared). The live lifecycle fixture verifies fidelity *by construction* — the same `crane manifest` yields both the recorded value and the truth. So do **not** author a "fast fidelity" layer for any kind; it can only fake the check or test a reimplementation. What `fast` *could* honestly cover is host orchestration (right build composed, guards fire) — a different assertion needing a no-submit seam on the capture verb; deferred, not required. Empirical anchor: the `lode-lifecycle` service fixture (capture → enumerate → inspect → banish → restored) passed first-run against live GAR.

## Open issues before mounting paces

Remaining open items are pace-time choices, settled by the first relevant pace:

1. **Host-tier regime** for substrate touchmark election/provisioning (`wsl`/`podvm`) — station regime vs BURN node profile vs a new host-config regime. Decide when the first substrate-kind consumption pace mounts.

(GAR layout and provenance attachment, formerly open here, are now decided — see *Lode registry layout & naming*: single `rbi_ld`, one-package-per-Lode, reserved `:rbi_vouch` tag.)

## Heat nature

Design heat with the **bole, reliquary, and wsl verticals landed** (capture bodies riding the ₣BX spine, cloud steps, service fixtures) and the **crane embrace landed pending its service gate** — capture runs on gcrane + curl + gpg with docker/buildx evicted.
The **delete architecture is cinched and first in pace order**: cloud-dispatch banish/abjure replacing host trust-200 REST (see the cloud-dispatch premise).
Remaining greenfield: the `podvm` kind (immure), the divine/augur split realization, the path-polymorphic image backdoor, and the serial cutovers (inscribe → conclave repoint; skopeo terminal eviction).
The `RBSL` cluster is landed through RBSLA/B/C/D/E/U, leaving only the podvm subdoc to ride its vertical; the remaining landing-regime decisions defer until the verticals commit, when everything here is re-investigated. Scope locked to the fetched side (made-images out of scope; see Shape) and to acquisition (wsl/podvm consumption deferred — this heat captures their bytes into GAR and proves control there; host consumption is the locked-but-deferred sequel).

## Cross-heat dependency — ₣BX built the capture spine (LANDED, gate discharged)

The remaining implementation work rides infrastructure the sibling heat **₣BX** (rbk-09-capture-machinery-unification) **built and landed** — not this one. ₣BX was a behavior-preserving refactor: it decomposed the host monoliths (`rbld`/`rbfl`/`rbfc`) into well-prefixed cluster files and stood up a **data-driven capture spine** (`rblds_Spine.sh`) plus a shared cloud step-library (`rbgjs/`), where a kind is a recipe (data) + a substitutions blob the spine consumes — no per-kind branch, no shared-file edit. ₣BX migrated bole and **reserved the per-kind body letters (`rbldr_`/`rbldw_`/`rbldv_`) — verified free** for this heat's kinds.

Consequence for this heat's shape: the "scaffold" the verticals were to ride is ₣BX's deliverable, **now landed**. Each kind here attaches as a **thin body file riding the spine — file-disjoint by construction** (the spine header is explicit: it owns no kind knowledge; recipe + substitutions are data), which dissolves the stub-vs-serialize question outright; this heat's own scaffold shrinks to the kind-registration surface ₣BX did **not** touch (colophons, kind-letter constants, the theurge registry, the divine-legend kinds-loop). ₣BX **preserved** the seams this heat needs: the capture step's pre-copy point stays clean for the ensconce collision guard, and inscribe/enshrine remain **live forks** (`RBSAE`, `rbfli_Inscribe.sh`) for this heat's cutovers to remove — and it relocated the chokepoints those cutovers repoint: **`zrbfc_resolve_tool_images` now lives in `rbfca_StepAssembly.sh`** (out of the former `rbfc` monolith).

**Sequencing (load-bearing — gate now satisfied):** the bole `RBSL` spec work was independent of ₣BX and landed first; everything else here was downstream of ₣BX. **₣BX has wrapped**, so the "mount only after ₣BX wraps" gate is **lifted** — the re-plan checkpoint and every implementation pace are mount-ready, re-baselined against ₣BX's landed layout (confirmed at this groom: spine registration-agnostic, body-letters free, RBSL cluster A/B/D/E present with C/U/I greenfield, cloud steps `rbgjl01`/`rbgjl02` + `rbgjs/` intact, enshrine/inscribe forks live). The constraint that guarded against regressing into the copy-paste forks ₣BX exists to abolish is discharged.

## Execution posture (groomed 2026-06-09)

- **Depot is disposable.** It will be destroyed and reformed after this heat; depot cleanup is never a goal — only loud delete correctness is. Surviving debris is test material (the 55-version cascade corpse), not duty.
- **Verification batching.** Cloud fixtures dominate wall-clock; adjacent paces share one service-tier run where the later pace's gate supersets the earlier one's (the delete-architecture pace delegates its live verification to the crane gate's single run). Fixture runs serialize globally — across officia too.
- **Side lane.** The Windows-deferral housekeeping, the clean-tree onboarding teaching, and the enshrine-spec retirement are file-disjoint from the spine and gated on nothing outstanding — mountable in a second officium during spine cloud waits.
  Wraps serialize (wrap sweeps the whole tree); the spec retirement should land before the inscribe cutover mounts (both touch the RBS0 mapping section).
- **Driver tiering.** Dockets carry a tier hint in their Character section; the opus driver delegates mechanical bodies to cheaper subagents and verifies, reserving its own judgment for where the hint says so.

## References

- ₣AV (retired) — predecessor GAR-mirroring heat
- ₣Az (retired) — deferred ark-concept-debt heat, closed by the GAR-package substrate finding
- RBSAE `ark_enshrine` — current narrow-scope spec; becomes the `bole`-kind `ensconce` pipeline (`enshrine` the verb retires into per-kind capture)
- RBSAS `ark_summon`, RBSDI `depot_inscribe`, RBSDY `director_yoke` — verbs within blast radius
- RBSRV `RegimeVessel` — ORIGIN/ANCHOR precedent; enum-gated-variable-groups pattern
- `Tools/rbk/vov_veiled/FUTURE/rbv_PodmanVM.sh` + RBSPV — concrete podman-vm pipeline; reveals fan-out, the two-source quay split, the brand-file provenance embryo, and the ignite-VM machination that cloud-side acquisition dissolves
- BUSJCW CaparisonWindows, BUSJGW GarrisonWsl, BUSJIW InvigilateWindows — `wsl` kind consumption and the `wsl --export` seed path it retires
- WSL rootfs is vendor-published over HTTPS (Canonical cloud-images; Microsoft `DistributionInfo.json` carries per-distro URL + SHA-256) — basis for cloud-side `wsl` acquisition with no Windows in the loop
- GAR delete granularity (package / version / tag) verified — `packages delete` is the atomic whole-Lode delete unit
- CLAUDE.md "Prefix Naming Discipline" and "Quoin Sub-Letter Discipline"
- RBSHR-HorizonRoadmap.adoc — public-facing horizon entries

## Paces

### conclave-live-verify-banish (₢BHAAe) [complete]

**[260610-1811] complete**

## Character
Operator-gated live verification, then the banish-last tail of the inscribe→conclave cutover.
One cloud-spend sequence; the failure breadcrumb below makes diagnosis mechanical.
Runs FIRST among remaining: the build path is inoperable until it does,
and the bind-build, README-conversion, and vocab-scrub paces all stand downstream of a green verify.

## Mount orientation — why the system is half-cut
Every vessel rbrv.env still yokes an rbi_rq-era reliquary touchmark;
the repointed resolver looks that touchmark up in rbi_ld, where it does not exist.
Any ordain fails until conclave→yoke replaces it.
rbrv.env is moorings-side and survives depot lifecycle — re-yoking is required even on a fresh depot.

## Goal
Exercise the committed conclave repoint end-to-end against live GCP — the cutover's deferred verify gate —
then retire the legacy rbi_rq namespace, cinched delete-old-last.

## Done when
- A fresh conclave Lode is minted (rbw-lC), its touchmark yoked across every vessel (rbw-dY),
  and one conjure ordain runs green against it — the verify gate.
- The legacy rbi_rq GAR packages are gone.
  Depot delete+recreate satisfies this by construction (the operator's standing plan) —
  do not banish packages on a depot about to be unmade; on a recreated depot this line is vacuous-true.
- The dead constants are removed regardless of depot path
  (discovery: `grep -rn "RBGC_GAR_CATEGORY_RELIQUARIES\|RBGL_RELIQUARIES_ROOT\|rbi_rq" Tools/rbk` —
  zero live consumers confirmed at slate; only the definitions and comment examples remain).
- The yoked rbrv.env change and a service-suite pass committed for the record.

## Failure breadcrumb
A cloud-preflight death `FATAL: _RBGR_TAG_SPRUE missing` means a host wiring site missed the substitution rename —
grep `_RBGR_TAG_SPRUE` / `_RBGR_LODES_ROOT` across the rbfd/rbfv host composition.
On a pristine depot, also watch the first Director-run delete build for the self-actAs propagation flap
(memo-20260610-heat-BH-fable-recommendation-actas-propagation-flap — this bring-up is its first real test venue).

## Sources
Cutover commit a510073d4 and its closing wrap summary (the two deferred criteria carried here);
paddock Cutover technique (rebuild-and-repoint, delete-old-last).

**[260610-1446] rough**

## Character
Operator-gated live verification, then the banish-last tail of the inscribe→conclave cutover.
One cloud-spend sequence; the failure breadcrumb below makes diagnosis mechanical.
Runs FIRST among remaining: the build path is inoperable until it does,
and the bind-build, README-conversion, and vocab-scrub paces all stand downstream of a green verify.

## Mount orientation — why the system is half-cut
Every vessel rbrv.env still yokes an rbi_rq-era reliquary touchmark;
the repointed resolver looks that touchmark up in rbi_ld, where it does not exist.
Any ordain fails until conclave→yoke replaces it.
rbrv.env is moorings-side and survives depot lifecycle — re-yoking is required even on a fresh depot.

## Goal
Exercise the committed conclave repoint end-to-end against live GCP — the cutover's deferred verify gate —
then retire the legacy rbi_rq namespace, cinched delete-old-last.

## Done when
- A fresh conclave Lode is minted (rbw-lC), its touchmark yoked across every vessel (rbw-dY),
  and one conjure ordain runs green against it — the verify gate.
- The legacy rbi_rq GAR packages are gone.
  Depot delete+recreate satisfies this by construction (the operator's standing plan) —
  do not banish packages on a depot about to be unmade; on a recreated depot this line is vacuous-true.
- The dead constants are removed regardless of depot path
  (discovery: `grep -rn "RBGC_GAR_CATEGORY_RELIQUARIES\|RBGL_RELIQUARIES_ROOT\|rbi_rq" Tools/rbk` —
  zero live consumers confirmed at slate; only the definitions and comment examples remain).
- The yoked rbrv.env change and a service-suite pass committed for the record.

## Failure breadcrumb
A cloud-preflight death `FATAL: _RBGR_TAG_SPRUE missing` means a host wiring site missed the substitution rename —
grep `_RBGR_TAG_SPRUE` / `_RBGR_LODES_ROOT` across the rbfd/rbfv host composition.
On a pristine depot, also watch the first Director-run delete build for the self-actAs propagation flap
(memo-20260610-heat-BH-fable-recommendation-actas-propagation-flap — this bring-up is its first real test venue).

## Sources
Cutover commit a510073d4 and its closing wrap summary (the two deferred criteria carried here);
paddock Cutover technique (rebuild-and-repoint, delete-old-last).

**[260610-1444] rough**

## Character
Operator-gated live verification, then the banish-last tail of the inscribe→conclave cutover.
One cloud-spend sequence; the failure breadcrumb below makes diagnosis mechanical.
Runs FIRST among remaining: the build path is inoperable until it does,
and the bind-build, README-conversion, and vocab-scrub paces all stand downstream of a green verify.

## Mount orientation — why the system is half-cut
Every vessel rbrv.env still yokes an rbi_rq-era reliquary touchmark;
the repointed resolver looks that touchmark up in rbi_ld, where it does not exist.
Any ordain fails until conclave→yoke replaces it.

## Goal
Exercise the committed conclave repoint end-to-end against live GCP — the cutover's deferred verify gate —
then banish the legacy rbi_rq namespace, cinched delete-old-last.

## Done when
- A fresh conclave Lode is minted (rbw-lC), its touchmark yoked across every vessel (rbw-dY),
  and one conjure ordain runs green against it — the verify gate.
- Then banish-last: the legacy rbi_rq GAR packages deleted,
  and the dead constants removed
  (discovery: `grep -rn "RBGC_GAR_CATEGORY_RELIQUARIES\|RBGL_RELIQUARIES_ROOT\|rbi_rq" Tools/rbk` —
  zero live consumers confirmed at slate; only the definitions and comment examples remain).
- The yoked rbrv.env change and a service-suite pass committed for the record.

## Failure breadcrumb
A cloud-preflight death `FATAL: _RBGR_TAG_SPRUE missing` means a host wiring site missed the substitution rename —
grep `_RBGR_TAG_SPRUE` / `_RBGR_LODES_ROOT` across the rbfd/rbfv host composition.

## Sources
Cutover commit a510073d4 and its closing wrap summary (the two deferred criteria carried here);
paddock Cutover technique (rebuild-and-repoint, delete-old-last).

### buk-fact-chaining-prev-and-git-gate (₢BHAAQ) [complete]

**[260608-0801] complete**

## Character
Behavior-preserving BUK upgrade: the depth-1 fact-chaining substrate (landed) plus a clean-tree gate homed in a new `bug` git module. Mechanical, green at each step.

## Goal
Close the install-then-forgot-to-commit spook with a uniform "tools never commit, gate on a clean tree" posture, homed in a new BUK bash-git module `bug`. The `previous/` chaining substrate and consume primitives already landed (Stage 1). Per-kind consumer cutovers and the ensconce fact-emission are NOT this pace.

## Locked
- Module `bug` (BUK bash git utilities); first inhabitant `bug_require_clean_tree` (guard `ZBUG_SOURCED`, consts `BUG_*`). Bash-only — Rust git use is outside BUK framing.
- Generalize the two whole-tree `git diff --quiet` gates (rbfk kludge, rbfd mirror) into `bug_require_clean_tree`; document the convention in the CLAUDE.md git section. rbob's path-scoped charge gate stays separate by design.
- Single-form chaining discipline is established (Stage 1 BUS0). The `<stamp>.lode` removal + ensconce single-form emission + RBSLE reslate land with the bole-enshrine cutover (₢BHAAH), not here.
- Consumer cutovers are out: bole ANCHOR→fact rides the bole-enshrine cutover (₢BHAAH); reliquary RBRV_RELIQUARY/yoke rides the reliquary-inscribe cutover (₢BHAAM). The rbob nameplate drive is a sanctioned same-dispatch cycle (reads `current/`) — no migration owed.
- Broader bash-git consolidation (bud_dispatch describe, rbfcb build-context, rblm, jjfp, lmci) is out of scope — separate hygiene, not this heat.

## Done
- `previous/` + `buf_relay`/`buf_read_fact` + BUS0 [landed].
- `bug` module exists; rbfk + rbfd gate through `bug_require_clean_tree`; convention documented.
- Green: BUK self-test + fast + siege.

**[260608-0731] rough**

## Character
Behavior-preserving BUK upgrade: the depth-1 fact-chaining substrate (landed) plus a clean-tree gate homed in a new `bug` git module. Mechanical, green at each step.

## Goal
Close the install-then-forgot-to-commit spook with a uniform "tools never commit, gate on a clean tree" posture, homed in a new BUK bash-git module `bug`. The `previous/` chaining substrate and consume primitives already landed (Stage 1). Per-kind consumer cutovers and the ensconce fact-emission are NOT this pace.

## Locked
- Module `bug` (BUK bash git utilities); first inhabitant `bug_require_clean_tree` (guard `ZBUG_SOURCED`, consts `BUG_*`). Bash-only — Rust git use is outside BUK framing.
- Generalize the two whole-tree `git diff --quiet` gates (rbfk kludge, rbfd mirror) into `bug_require_clean_tree`; document the convention in the CLAUDE.md git section. rbob's path-scoped charge gate stays separate by design.
- Single-form chaining discipline is established (Stage 1 BUS0). The `<stamp>.lode` removal + ensconce single-form emission + RBSLE reslate land with the bole-enshrine cutover (₢BHAAH), not here.
- Consumer cutovers are out: bole ANCHOR→fact rides the bole-enshrine cutover (₢BHAAH); reliquary RBRV_RELIQUARY/yoke rides the reliquary-inscribe cutover (₢BHAAM). The rbob nameplate drive is a sanctioned same-dispatch cycle (reads `current/`) — no migration owed.
- Broader bash-git consolidation (bud_dispatch describe, rbfcb build-context, rblm, jjfp, lmci) is out of scope — separate hygiene, not this heat.

## Done
- `previous/` + `buf_relay`/`buf_read_fact` + BUS0 [landed].
- `bug` module exists; rbfk + rbfd gate through `bug_require_clean_tree`; convention documented.
- Green: BUK self-test + fast + siege.

**[260606-1143] rough**

## Character
BUK practice upgrade: design judgment on the fact-file contract plus a behavior-preserving refactor of dispatch-level machinery, then a mechanical sweep across producers/installers. High blast radius — the dispatch change touches every tabtarget — so sequence it like a ₣BX-style behavior-preserving refactor, green at each step.

## Goal
Introduce a depth-1 previous-output directory — `previous/`, a sibling of `current/` under the output root, exposed as `BURD_PREVIOUS_DIR` — promoted at the dispatch-start clear seam, and formalize the fact-file consume side, so cross-tabtarget chaining is precise — and close the install-then-forgot-to-commit spook class by adopting a uniform "tools never commit, but gate on a clean tree" posture. (No PREV exists today: production `rm -rf`'s `current/` at dispatch start, and BURV's per-invocation isolated roots are a random-access model, not a depth-1 PREV — so `previous/` is greenfield at the clear seam.)

## Locked (settled in design conversation; do not re-litigate)
- Tools NEVER commit in the consumer's codebase. Tools MAY presume git and refuse downstream steps on a dirty tree. Generalize the existing clean-tree gates (discover: grep `git diff --quiet`) into one uniform, well-messaged gate. This reverses an earlier auto-commit musing; the convention change is documented deliberately (CLAUDE.md git section + the ₣BH paddock provenance premise).
- Depth-1 chaining only — NO random-access fact store. Fan-out is ONE atomic install dispatch (one read, N writes), per the existing yoke-all precedent. The durable accumulator is the git-tracked consumer config (rbrv.env / nameplate), never the fact channel.
- `previous/` (`BURD_PREVIOUS_DIR`) is promoted from the prior dispatch's `current/` at dispatch start, EXIT-STATUS-INDEPENDENT, so a fail-after-forward preserves the baton. Naming is the self-documenting `current/` + `previous/` pair (BUK register; no decoding).
- Chaining facts are single-form: constant-named, bare singular-string content, no in-file parsing — the existing single-form discipline. The over-built `<stamp>.lode` multi-form is removed in favor of it: nothing parses its content host-side, provenance is canonical in GAR `:rbi_vouch`, and host-side provenance now costs a GAR call (accepted). RBSLE reslated, the `rbf_fact_lode` quoin retired.
- Type model: generic value-fact + enum kind-brand-fact, NOT per-type fact names. Per-type names would re-noun the lode kinds, contradicting the cinched "kinds are an enum." Made-vs-fetched by fact name (HALLMARK vs TOUCHMARK); kind-within-fetched by a sprue enum brand reusing the existing `rbi_<kind>` tokens.
- Consume side: `buf_read_fact` (read a named fact from `previous/`) + `buf_relay` (copy `previous/` → `current/`, the baton forward). Install ordering invariant: `buf_relay` FIRST, then read-and-assert-type, then fail-hard on wrong/absent.
- Multi-form facts stay for ENUMERATION (roster/audit/divine "list all"); not chaining handoffs, out of scope to change.
- Sequencing (MUST): build `previous/` + the consume primitives + extend the BUK self-test first (green, no consumer changes); then the chaining-test design — the harness isolates each invocation in its own root, so PREV promotion never naturally fires in tests, and a chaining test must share one root across producer+installer or seed `previous/` directly (precedent: the harness already seeds `current/` for fact-present tests); then migrate consumers one verb at a time.

## Done
- BURD owns `previous/` (`BURD_PREVIOUS_DIR`) promoted at dispatch start; the buf consume side exists (`buf_read_fact` + `buf_relay`); both carry robust BUS0*.adoc treatment.
- Single-form chaining-fact discipline established; `<stamp>.lode` removed; ensconce emits the touchmark + kind-brand chaining facts; RBSLE reslated.
- Uniform never-commit/clean-tree-gate posture applied across the install verbs and documented as a deliberate convention change.
- Producer/installer sweep landed (discover producers: grep `buf_write_fact_*`; installers: nameplate drive-hallmark + vessel rbrv.env writes); the kludge spook path closed.
- Test suite green (fast minimum; crucible for the lode/kludge paths).

**[260606-1127] rough**

## Character
BUK practice upgrade: design judgment on the fact-file contract plus a behavior-preserving refactor of dispatch-level machinery, then a mechanical sweep across producers/installers. High blast radius — the dispatch change touches every tabtarget — so sequence it like a ₣BX-style behavior-preserving refactor, green at each step.

## Goal
Introduce a depth-1 "previous output" (PREV) at the dispatch-start output-clear seam and formalize the fact-file consume side, so cross-tabtarget chaining is precise — and close the install-then-forgot-to-commit spook class by adopting a uniform "tools never commit, but gate on a clean tree" posture. (No such PREV exists today: production `rm -rf`'s `current/` at dispatch start, and BURV's per-invocation isolated roots are a random-access model, not a depth-1 PREV — so PREV is greenfield at the clear seam.)

## Locked (settled in design conversation; do not re-litigate)
- Tools NEVER commit in the consumer's codebase. Tools MAY presume git and refuse downstream steps on a dirty tree. Generalize the existing clean-tree gates (discover: grep `git diff --quiet`) into one uniform, well-messaged gate. This reverses an earlier auto-commit musing; the convention change is documented deliberately (CLAUDE.md git section + the ₣BH paddock provenance premise).
- Depth-1 chaining only — NO random-access fact store. Fan-out is ONE atomic install dispatch (one read, N writes), per the existing yoke-all precedent. The durable accumulator is the git-tracked consumer config (rbrv.env / nameplate), never the fact channel.
- PREV is promoted from the prior dispatch's `current/` at dispatch start, EXIT-STATUS-INDEPENDENT, so a fail-after-forward preserves the baton.
- Chaining facts are single-form: constant-named, bare singular-string content, no in-file parsing — the existing single-form discipline. The over-built `<stamp>.lode` multi-form is removed in favor of it: nothing parses its content host-side, provenance is canonical in GAR `:rbi_vouch`, and host-side provenance now costs a GAR call (accepted). RBSLE reslated, the `rbf_fact_lode` quoin retired.
- Type model: generic value-fact + enum kind-brand-fact, NOT per-type fact names. Per-type names would re-noun the lode kinds, contradicting the cinched "kinds are an enum." Made-vs-fetched by fact name (HALLMARK vs TOUCHMARK); kind-within-fetched by a sprue enum brand reusing the existing `rbi_<kind>` tokens.
- Install ordering invariant: forward-prev FIRST, then read-and-assert-type, then fail-hard on wrong/absent.
- Multi-form facts stay for ENUMERATION (roster/audit/divine "list all"); not chaining handoffs, out of scope to change.
- Sequencing (MUST): build PREV + consume primitives + extend the BUK self-test first (green, no consumer changes); then the chaining-test design — the harness isolates each invocation in its own root, so PREV promotion never naturally fires in tests, and a chaining test must share one root across producer+installer or seed `previous/` directly (precedent: the harness already seeds `current/` for fact-present tests); then migrate consumers one verb at a time.

## Done
- BURD owns a dispatch-start PREV; buf consume side exists (read-by-named-fact + forward-prev); both carry robust BUS0*.adoc treatment.
- Single-form chaining-fact discipline established; `<stamp>.lode` removed; ensconce emits touchmark + kind-brand chaining facts; RBSLE reslated.
- Uniform never-commit/clean-tree-gate posture applied across the install verbs and documented as a deliberate convention change.
- Producer/installer sweep landed (discover producers: grep `buf_write_fact_*`; installers: nameplate drive-hallmark + vessel rbrv.env writes); the kludge spook path closed.
- Test suite green (fast minimum; crucible for the lode/kludge paths).

**[260606-1121] rough**

## Character
BUK practice upgrade: design judgment on the fact-file contract plus a behavior-preserving refactor of dispatch-level machinery, then a mechanical sweep across producers/installers. High blast radius — the dispatch change touches every tabtarget — so sequence it like a ₣BX-style behavior-preserving refactor, green at each step.

## Goal
Promote the test-harness "previous output" concept (BURV `current/`) into a first-class production BURD primitive and formalize the fact-file consume side, so cross-tabtarget chaining is precise — and close the install-then-forgot-to-commit spook class by adopting a uniform "tools never commit, but gate on a clean tree" posture.

## Locked (settled in design conversation; do not re-litigate)
- Tools NEVER commit in the consumer's codebase. Tools MAY presume git and refuse downstream steps on a dirty tree. Generalize the existing clean-tree gates (discover: grep `git diff --quiet`) into one uniform, well-messaged gate. This reverses an earlier auto-commit musing; the convention change is documented deliberately (CLAUDE.md git section + the ₣BH paddock provenance premise).
- Depth-1 chaining only — NO random-access fact store. Fan-out is ONE atomic install dispatch (one read, N writes), per the existing yoke-all precedent. The durable accumulator is the git-tracked consumer config (rbrv.env / nameplate), never the fact channel.
- PREV is promoted from the prior dispatch's OUTPUT at dispatch start, EXIT-STATUS-INDEPENDENT, so a fail-after-forward preserves the baton.
- Chaining facts are single-form: constant-named, bare singular-string content, no in-file parsing — the existing single-form discipline. The over-built `<stamp>.lode` multi-form is removed in favor of it: nothing parses its content host-side, provenance is canonical in GAR `:rbi_vouch`, and host-side provenance now costs a GAR call (accepted). RBSLE reslated, the `rbf_fact_lode` quoin retired.
- Type model: generic value-fact + enum kind-brand-fact, NOT per-type fact names. Per-type names would re-noun the lode kinds, contradicting the cinched "kinds are an enum." Made-vs-fetched by fact name (HALLMARK vs TOUCHMARK); kind-within-fetched by a sprue enum brand reusing the existing `rbi_<kind>` tokens.
- Install ordering invariant: forward-prev FIRST, then read-and-assert-type, then fail-hard on wrong/absent.
- Multi-form facts stay for ENUMERATION (roster/audit/divine "list all"); not chaining handoffs, out of scope to change.
- Sequencing (MUST): build PREV + consume primitives + extend the BUK self-test first (green, no consumer changes); reconcile BURV `current/` to production PREV as its own tested step (the load-bearing risk); then migrate consumers one verb at a time.

## Done
- BURD owns a dispatch-start PREV; buf consume side exists (read-by-named-fact + forward-prev); both carry robust BUS0*.adoc treatment.
- Single-form chaining-fact discipline established; `<stamp>.lode` removed; ensconce emits touchmark + kind-brand chaining facts; RBSLE reslated.
- Uniform never-commit/clean-tree-gate posture applied across the install verbs and documented as a deliberate convention change.
- Producer/installer sweep landed (discover producers: grep `buf_write_fact_*`; installers: nameplate drive-hallmark + vessel rbrv.env writes); the kludge spook path closed.
- Test suite green (fast minimum; crucible for the lode/kludge paths).

### buo-tweak-sprue (₢BHAAF) [complete]

**[260605-1005] complete**

## Character
Infrastructure precursor to the ensconce collision-guard (₢BHAAC's live-GAR test needs a stamp-pin tweak). Touches a shared BUK regime — mechanical but exacting. Discovered mid-mount: BURE tweak names are ungoverned free strings (only `threemodegraft` exists today), so a test/consumer name typo silently no-ops and the test passes for the wrong reason.

## Goal
Give BURE tweak names a sprue so an unregistered/typo'd name fails loud instead of silently no-op'ing, and bring every existing tweak under it.

## Locked (settled in conversation)
- **Sprue letter `buo`** (free, neutral). Shape `buo<proj>_<name>` (e.g. `buorb_…`). The `_`-bearing sprue IS a virtual registry: `grep buo` finds every tweak, `grep buo<proj>_` one project's — no central enum/list to rot.
- **BUK enforces SHAPE, never names.** `zbure_enforce`: a non-empty `BURE_TWEAK_NAME` must match `^buo[a-z]+_`, else die. Keeps BURE generic (no consumer-name coupling) and preserves the BUS0 "BUK doesn't interpret tweak semantics" line — shape ≠ semantics.
- **Suffix-typo guard is the shared constant**, not the sprue: each consumer defines its tweak name as a constant; the test references the mirror (existing mirror discipline). Sprue governs the namespace; the constant kills spelling drift.
- **BUK's own (self-test) tweaks** use a reserved segment, not a consumer project (`buobu_` is degenerate) — settle the segment at mount.

## Transform all existing tweaks
None left bare. Discovery: `grep -rn 'BURE_TWEAK_NAME' Tools/` — covers the `threemodegraft` consumer (rbfd), its setter in the theurge harness, and the BURE self-test's example values.

## Register the convention
CLAUDE.md mint discipline (Extended Namespace Checklist — tweak names join env-vars / refs / slash-commands) + the BUS0 tweak section.

## Done
BURE rejects a non-sprued tweak name loud; every existing tweak is `buo`-sprued; BUK self-test + the rbfd consumer path stay green; the convention is documented; a sprued stamp-pin tweak is available for ₢BHAAC.

**[260605-0950] rough**

## Character
Infrastructure precursor to the ensconce collision-guard (₢BHAAC's live-GAR test needs a stamp-pin tweak). Touches a shared BUK regime — mechanical but exacting. Discovered mid-mount: BURE tweak names are ungoverned free strings (only `threemodegraft` exists today), so a test/consumer name typo silently no-ops and the test passes for the wrong reason.

## Goal
Give BURE tweak names a sprue so an unregistered/typo'd name fails loud instead of silently no-op'ing, and bring every existing tweak under it.

## Locked (settled in conversation)
- **Sprue letter `buo`** (free, neutral). Shape `buo<proj>_<name>` (e.g. `buorb_…`). The `_`-bearing sprue IS a virtual registry: `grep buo` finds every tweak, `grep buo<proj>_` one project's — no central enum/list to rot.
- **BUK enforces SHAPE, never names.** `zbure_enforce`: a non-empty `BURE_TWEAK_NAME` must match `^buo[a-z]+_`, else die. Keeps BURE generic (no consumer-name coupling) and preserves the BUS0 "BUK doesn't interpret tweak semantics" line — shape ≠ semantics.
- **Suffix-typo guard is the shared constant**, not the sprue: each consumer defines its tweak name as a constant; the test references the mirror (existing mirror discipline). Sprue governs the namespace; the constant kills spelling drift.
- **BUK's own (self-test) tweaks** use a reserved segment, not a consumer project (`buobu_` is degenerate) — settle the segment at mount.

## Transform all existing tweaks
None left bare. Discovery: `grep -rn 'BURE_TWEAK_NAME' Tools/` — covers the `threemodegraft` consumer (rbfd), its setter in the theurge harness, and the BURE self-test's example values.

## Register the convention
CLAUDE.md mint discipline (Extended Namespace Checklist — tweak names join env-vars / refs / slash-commands) + the BUS0 tweak section.

## Done
BURE rejects a non-sprued tweak name loud; every existing tweak is `buo`-sprued; BUK self-test + the rbfd consumer path stay green; the convention is documented; a sprued stamp-pin tweak is available for ₢BHAAC.

### lode-base-pilot-sketch (₢BHAAA) [complete]

**[260602-1316] complete**

## Character
Ergonomics finish on the base-kind Lode surface (divine/banish). Close to wrappable: one diagnosed fix to apply plus live verification. Hands-on — operator runs the tabtargets; agent directs and edits.

## State
ensconce/divine/banish landed and live-verified against GAR (busybox and python:3.12.7-slim-bookworm captures this session). Committed this session: divine-enumerate enrichment — a Kinds legend (touchmark-prefix key) plus a TOUCHMARK/IMAGE table; IMAGE is the unsprued fingerprint tag (<sanitized-origin>-<sha10>), found via the sha10 from the rbi_sha256- member tag (one tags-list per Lode). BCG-compliant. NOT run since the rewrite.

## Remaining
- **Banish confirm-prompt fix — diagnosed, NOT applied.** `tt/rbw-lB.DirectorBanishesLode.sh` is a bare tabtarget missing `BURD_INTERACTIVE`. buc_require prints its prompt to stderr then `read </dev/tty`; in the non-interactive dispatch branch (the bud_dispatch curation pipe) stderr is buffered behind the logging pipe and does not flush before read blocks — operator sees "Type yes to confirm:" only AFTER pressing enter (empty input, confirm fails). Fix: add `export BURD_INTERACTIVE=1` to the tabtarget, mirroring rbw-cr / rbw-ch / rbw-cs / rbw-gPI, which route through the line-buffering-preserving dispatch branch. Confirm the prompt shows before input and `yes` banishes.
- **Verify divine enrichment live** — run `rbw-ld` against a captured Lode; confirm the legend and the fingerprint IMAGE column render.
- Optional polish: in rbld_Lode.sh ensconce, the log line "Ensconce build submitted" floats "build" free of the Cloud Build proper noun → "Cloud Build submitted". The other "build" lines correctly name the GCB resource — leave them.

## Watch (likely not this pace)
No-arg `rbw-lB` once died `buf_write_fact_single: preexists ... burx.env` — a dispatch/output-dir symptom, seen once. Reproduce and decide ownership before attributing it to banish.

## Done
Banish prompt shows before input and accepts `yes`; divine enrichment confirmed live; the vouch-word polish is applied or explicitly deferred. Then wrappable.

**[260602-0826] rough**

## Character
Ergonomics finish on the base-kind Lode surface (divine/banish). Close to wrappable: one diagnosed fix to apply plus live verification. Hands-on — operator runs the tabtargets; agent directs and edits.

## State
ensconce/divine/banish landed and live-verified against GAR (busybox and python:3.12.7-slim-bookworm captures this session). Committed this session: divine-enumerate enrichment — a Kinds legend (touchmark-prefix key) plus a TOUCHMARK/IMAGE table; IMAGE is the unsprued fingerprint tag (<sanitized-origin>-<sha10>), found via the sha10 from the rbi_sha256- member tag (one tags-list per Lode). BCG-compliant. NOT run since the rewrite.

## Remaining
- **Banish confirm-prompt fix — diagnosed, NOT applied.** `tt/rbw-lB.DirectorBanishesLode.sh` is a bare tabtarget missing `BURD_INTERACTIVE`. buc_require prints its prompt to stderr then `read </dev/tty`; in the non-interactive dispatch branch (the bud_dispatch curation pipe) stderr is buffered behind the logging pipe and does not flush before read blocks — operator sees "Type yes to confirm:" only AFTER pressing enter (empty input, confirm fails). Fix: add `export BURD_INTERACTIVE=1` to the tabtarget, mirroring rbw-cr / rbw-ch / rbw-cs / rbw-gPI, which route through the line-buffering-preserving dispatch branch. Confirm the prompt shows before input and `yes` banishes.
- **Verify divine enrichment live** — run `rbw-ld` against a captured Lode; confirm the legend and the fingerprint IMAGE column render.
- Optional polish: in rbld_Lode.sh ensconce, the log line "Ensconce build submitted" floats "build" free of the Cloud Build proper noun → "Cloud Build submitted". The other "build" lines correctly name the GCB resource — leave them.

## Watch (likely not this pace)
No-arg `rbw-lB` once died `buf_write_fact_single: preexists ... burx.env` — a dispatch/output-dir symptom, seen once. Reproduce and decide ownership before attributing it to banish.

## Done
Banish prompt shows before input and accepts `yes`; divine enrichment confirmed live; the vouch-word polish is applied or explicitly deferred. Then wrappable.

**[260602-0634] rough**

## Character
First kind sets the reusable shape; build with live-GAR verification. Sequential.

## Goal
Implement base-kind capture (`ensconce`) as an additive module, parallel-safe alongside live `enshrine`, per the locked Lode shape in the paddock. No cutover.

## Starting state
Gate passed — `enshrine` is cleanly separable: write path is namespace-parameterized (`RBGL_ENSHRINES_ROOT`, no hardcoded `rbi_es`), ANCHOR consumption is read-only, audit enumerators are independent per-namespace functions (a new namespace stays invisible until its own enumerator exists — see the `rbi_df` precedent), and conjure's in-pool preflight is not namespace-aware. Design locked in paddock *Lode registry layout & naming* — build to it, don't re-litigate.

## Build
- **Three first-class `l`-family commands this pace**, not capture alone: `ensconce` (`lE`, capture), `divine` (`ld`, lowercase read-only enumerate-by-touchmark), `banish` (`lB`, whole-Lode `packages delete`). The `service` fixture drives the real commands, not test-only scaffolding.
- Code-module prefix is **`rbld_`** (ratified — see paddock); spec letter `RBSL` reserved.
- Capture-pure `ensconce`: resolve coordinate from the vessel ORIGIN convention -> fetch + measure digest inside Cloud Build (egress pool, never the workstation) -> push the `rbi_ld` Lode with its member tags + `:rbi_vouch` -> emit the host capture-file. Writes NO consumer config; never touches `enshrine` or ANCHOR (the enshrine rbrv.env writeback is exactly what capture-pure drops).
- Design the 1-member package so an N-member cohort slots in without restructuring (the cheap test of "cardinality is just an attribute").
- `RBSL` spec stub only if useful — do not finish it; RBS0* edits are out of scope (deferred to the remainder pace).

## Build recipe — templates to copy-adapt
`ensconce` mirrors `enshrine` structurally. Templates: the `rbfd_enshrine` orchestrator family in `rbfd_FoundryDirectorBuild.sh` (host orchestrate -> stitch build JSON -> extract output) and its in-pool step `rbgje/rbgje01-enshrine-copy.sh`; the depot-family enrollment block in `rbz_zipper.sh` (mirror as a new `rbw-l*` Lode group); a `tt/rbw-dE.*.sh` launcher; and the `fast`/`service` fixtures in `rbtd/src/rbtdrf_fast.rs` / `rbtdrc_crucible.rs` (+ fixture-name + required-colophons in `rbtdrm_manifest.rs`, then register in the `RBTDRC_FIXTURES` array). The Lode category constant slots beside the existing four in `rbgc_Constants.sh` + `rbgl_GarLayout.sh`. For sourcing/kindle, copy a `*_cli.sh` furnish wrapper.

## Tests (backbone, not coverage)
- `fast` (deterministic): coordinate resolution, digest verification, envelope-shape over fixtures — assert capture-fidelity (recorded digest == fixture upstream digest), not existence.
- `service` (live GAR, guaranteed teardown): capture -> list -> inspect -> whole-Lode delete; assert the mixed-artifact package (member tags + `:rbi_vouch`) round-trips and is reaped. Single-call `packages delete` preferred; a GCP-run tag-delete loop is an acceptable fallback if GAR refuses.
- minimal upstream smoke — upstream is the Pale; contain its nondeterminism at one membrane.

## Done
`ensconce` captures a base into a parallel `rbi_ld` Lode with member tags + a real `:rbi_vouch` envelope + host capture-file; `divine` lists / inspects and `banish` deletes, all working against live GAR; the fast/service/smoke layers exist for base; live `enshrine` and all conjure builds remain untouched and green.

**[260601-0842] rough**

## Character
First kind sets the reusable shape; build with live-GAR verification. Sequential.

## Goal
Implement base-kind capture (`ensconce`) as an additive module, parallel-safe alongside live `enshrine`, per the locked Lode shape in the paddock. No cutover.

## Starting state
Gate passed — `enshrine` is cleanly separable: write path is namespace-parameterized (`RBGL_ENSHRINES_ROOT`, no hardcoded `rbi_es`), ANCHOR consumption is read-only, audit enumerators are independent per-namespace functions (a new namespace stays invisible until its own enumerator exists — see the `rbi_df` precedent), and conjure's in-pool preflight is not namespace-aware. Design locked in paddock *Lode registry layout & naming* — build to it, don't re-litigate.

## Build
- Code-module prefix is **`rbld_`** (ratified — see paddock); spec letter `RBSL` reserved.
- Capture-pure `ensconce`: resolve coordinate from the vessel ORIGIN convention -> fetch + measure digest inside Cloud Build (egress pool, never the workstation) -> push the `rbi_ld` Lode with its member tags + `:rbi_vouch` -> emit the host capture-file. Writes NO consumer config; never touches `enshrine` or ANCHOR (the enshrine rbrv.env writeback is exactly what capture-pure drops).
- Design the 1-member package so an N-member cohort slots in without restructuring (the cheap test of "cardinality is just an attribute").
- `RBSL` spec stub only if useful — do not finish it; RBS0* edits are out of scope (deferred to the remainder pace).

## Build recipe — templates to copy-adapt
`ensconce` mirrors `enshrine` structurally. Templates: the `rbfd_enshrine` orchestrator family in `rbfd_FoundryDirectorBuild.sh` (host orchestrate -> stitch build JSON -> extract output) and its in-pool step `rbgje/rbgje01-enshrine-copy.sh`; the depot-family enrollment block in `rbz_zipper.sh` (mirror as a new `rbw-l*` Lode group); a `tt/rbw-dE.*.sh` launcher; and the `fast`/`service` fixtures in `rbtd/src/rbtdrf_fast.rs` / `rbtdrc_crucible.rs` (+ fixture-name + required-colophons in `rbtdrm_manifest.rs`, then register in the `RBTDRC_FIXTURES` array). The Lode category constant slots beside the existing four in `rbgc_Constants.sh` + `rbgl_GarLayout.sh`. For sourcing/kindle, copy a `*_cli.sh` furnish wrapper.

## Tests (backbone, not coverage)
- `fast` (deterministic): coordinate resolution, digest verification, envelope-shape over fixtures — assert capture-fidelity (recorded digest == fixture upstream digest), not existence.
- `service` (live GAR, guaranteed teardown): capture -> list -> inspect -> whole-Lode delete; assert the mixed-artifact package (member tags + `:rbi_vouch`) round-trips and is reaped. Single-call `packages delete` preferred; a GCP-run tag-delete loop is an acceptable fallback if GAR refuses.
- minimal upstream smoke — upstream is the Pale; contain its nondeterminism at one membrane.

## Done
`ensconce` captures a base into a parallel `rbi_ld` Lode with member tags + a real `:rbi_vouch` envelope + host capture-file; list / inspect / delete work against live GAR; the fast/service/smoke layers exist for base; live `enshrine` and all conjure builds remain untouched and green.

**[260601-0822] rough**

## Character
First kind sets the reusable shape; build with live-GAR verification. Sequential.

## Goal
Implement base-kind capture (`ensconce`) as an additive module, parallel-safe alongside live `enshrine`, per the locked Lode shape in the paddock. No cutover.

## Starting state
Gate passed — `enshrine` is cleanly separable: write path is namespace-parameterized (`RBGL_ENSHRINES_ROOT`, no hardcoded `rbi_es`), ANCHOR consumption is read-only, audit enumerators are independent per-namespace functions (a new namespace stays invisible until its own enumerator exists — see the `rbi_df` precedent), and conjure's in-pool preflight is not namespace-aware. Design locked in paddock *Lode registry layout & naming* — build to it, don't re-litigate.

## Build
- Mint the code-module prefix (the one open mint — avoid `rbl`/`rblm`; spec letter `RBSL` reserved). Enumerate namespaces per CLAUDE.md before settling.
- Capture-pure `ensconce`: resolve coordinate from the vessel ORIGIN convention -> fetch + measure digest inside Cloud Build (egress pool, never the workstation) -> push the `rbi_ld` Lode with its member tags + `:rbi_vouch` -> emit the host capture-file. Writes NO consumer config; never touches `enshrine` or ANCHOR.
- Design the 1-member package so an N-member cohort slots in without restructuring (the cheap test of "cardinality is just an attribute").
- `RBSL` spec stub only if useful — do not finish it; RBS0* edits are out of scope (deferred to the remainder pace).

## Tests (backbone, not coverage)
- `fast` (deterministic): coordinate resolution, digest verification, envelope-shape over fixtures — assert capture-fidelity (recorded digest == fixture upstream digest), not existence.
- `service` (live GAR, guaranteed teardown): capture -> list -> inspect -> whole-Lode delete; assert the mixed-artifact package (member tags + `:rbi_vouch`) round-trips and is reaped. Single-call `packages delete` preferred; a GCP-run tag-delete loop is an acceptable fallback if GAR refuses.
- minimal upstream smoke — upstream is the Pale; contain its nondeterminism at one membrane.

## Done
`ensconce` captures a base into a parallel `rbi_ld` Lode with member tags + a real `:rbi_vouch` envelope + host capture-file; list / inspect / delete work against live GAR; the fast/service/smoke layers exist for base; live `enshrine` and all conjure builds remain untouched and green.

**[260601-0619] rough**

## Character
Design conversation requiring judgment — the first kind locks the reusable shape for all six. Opus, conversational mount.

## Goal
Sketch base-kind capture (`ensconce`) running parallel-safe alongside live `enshrine`, and through that slice settle the reusable Lode backbone. No cutover.

## First action — gate
Read-only blast-radius scan to verify the daylight the whole strategy rests on: confirm `enshrine` capture (writes `rbi_es`, feeds the vessel ANCHOR consumption checked by conjure's in-pool preflight) is cleanly separable from a new additive Lode module. If hidden coupling surfaces (namespace enumerators, shared kindle/regime state, preflight assumptions), surface it before writing code — the strategy may need narrowing or a pivot to a greenfield kind.

## Locked constraints
- Base-first because small artifacts make the backbone-tuning loop cheap.
- Capture-pure: `ensconce` mints a Lode + capture-file and writes NO consumer config. This is the daylight guarantee (paddock no-mixing premise) — it must not touch `enshrine` code or ANCHOR consumption.
- Distinct GAR namespace, never `rbi_es`. Namespace collapse is a cutover act, deferred.
- Settle through this slice: GAR package naming, the capture-file envelope shape, the code-module prefix.
- Test architecture is backbone (not coverage): push logic to deterministic `fast` (resolve-coordinate, verify-checksum, envelope-shape over fixtures); thin `service` live-GAR lifecycle (capture → list → inspect → banish, guaranteed teardown); minimal upstream smoke. Upstream is the Pale — contain its nondeterminism at one membrane. Assert capture-fidelity (digest matches upstream), not mere existence.
- Design the 1-member package so an N-member cohort would slot in without restructuring — the cheap test of the paddock's "cardinality is just an attribute" claim.
- `RBSL` spec stub only if useful; do not finish it. RBS0* edits are out of scope (deferred to the remainder pace).

## Done
`ensconce` captures a base into a parallel Lode with a real provenance envelope + capture-file; list / inspect / banish work against live GAR; the fast/service/smoke layers exist for base; live `enshrine` and all conjure builds remain untouched and green.

### lode-ensconce-collision-guard (₢BHAAC) [complete]

**[260605-1053] complete**

## Character
Design settled this session — implementation + spec sync, largely mechanical. The one subtlety is retry-idempotency.

## Goal
Ensconce must never silently clobber an existing Lode. Add a digest-aware collision guard on the cloud capture step (rbgjl01): before the skopeo copy, if the touchmark package already exists, fail loud — unless it holds the identical digest, in which case proceed.

## Locked
- Cloud-side only. The guard must be atomic with the GAR write; a host pre-check is TOCTOU plus a multi-second pre-submit window (the "acquisition runs cloud-side" premise). Do not add a host collision check.
- Digest-aware, not naive fail-if-exists — a Cloud Build retry re-copies the same digest and must still pass.
- Scope is touchmark collision only. Content dedup (same origin@digest under a different touchmark — a host read-before-submit cost-saver) is a separate optional pace, explicitly out of scope here.

## Spec
Document the guard in **RBSLE** (the ensconce operation spec, written by the preceding spec pace) — NOT RBSAE, which is the retiring enshrine spec and stays untouched. Spec and code land together.

## Done
A second ensconce reusing a touchmark fails loud cloud-side with a clear message; an identical-digest retry still succeeds; the RBSLE contract states it; the lifecycle fixture (or a new case) exercises the collision path.

**[260603-1133] rough**

## Character
Design settled this session — implementation + spec sync, largely mechanical. The one subtlety is retry-idempotency.

## Goal
Ensconce must never silently clobber an existing Lode. Add a digest-aware collision guard on the cloud capture step (rbgjl01): before the skopeo copy, if the touchmark package already exists, fail loud — unless it holds the identical digest, in which case proceed.

## Locked
- Cloud-side only. The guard must be atomic with the GAR write; a host pre-check is TOCTOU plus a multi-second pre-submit window (the "acquisition runs cloud-side" premise). Do not add a host collision check.
- Digest-aware, not naive fail-if-exists — a Cloud Build retry re-copies the same digest and must still pass.
- Scope is touchmark collision only. Content dedup (same origin@digest under a different touchmark — a host read-before-submit cost-saver) is a separate optional pace, explicitly out of scope here.

## Spec
Document the guard in **RBSLE** (the ensconce operation spec, written by the preceding spec pace) — NOT RBSAE, which is the retiring enshrine spec and stays untouched. Spec and code land together.

## Done
A second ensconce reusing a touchmark fails loud cloud-side with a clear message; an identical-digest retry still succeeds; the RBSLE contract states it; the lifecycle fixture (or a new case) exercises the collision path.

**[260602-1313] rough**

## Character
Design settled this session — implementation + spec sync, largely mechanical. The one subtlety is retry-idempotency.

## Goal
Ensconce must never silently clobber an existing Lode. Add a digest-aware collision guard on the cloud capture step (rbgjl01): before the skopeo copy, if the touchmark package already exists, fail loud — unless it holds the identical digest, in which case proceed.

## Locked
- Cloud-side only. The guard must be atomic with the GAR write; a host pre-check is TOCTOU plus a multi-second pre-submit window (the "acquisition runs cloud-side" premise). Do not add a host collision check.
- Digest-aware, not naive fail-if-exists — a Cloud Build retry re-copies the same digest and must still pass.
- Scope is touchmark collision only. Content dedup (same origin@digest under a different touchmark — a host read-before-submit cost-saver) is a separate optional pace, explicitly out of scope here.

## Spec
Update the RBS0* spec surface for the ensconce/Lode capture contract to document the guard alongside the code (RBSAE is today's ensconce-pipeline spec per the paddock; RBSL reserved/unwritten). Spec and code land together.

## Done
A second ensconce reusing a touchmark fails loud cloud-side with a clear message; an identical-digest retry still succeeds; the RBS0* contract states it; the lifecycle fixture (or a new case) exercises the collision path.

### lode-remainder-replan (₢BHAAB) [complete]

**[260605-1136] complete**

## Character
**₣BX has wrapped — this pace is mount-ready.** The "mount only after ₣BX wraps" gate is discharged (see paddock "Cross-heat dependency"). ₣BX built the capture spine + host-module decomposition this heat's verticals ride, and this groom re-baselined the docket/paddock against ₣BX's landed layout — so mount goes straight to slating, not re-discovery. The decomposition axis (by kind-vertical) and the cutover technique were settled at groom and stay; what ₣BX changed was the *scaffold*, now its landed deliverable, not ours. Still a planning pace: the deliverable is concrete slated paces, not code.

## Groom re-baseline (confirmed against ₣BX's landed tree — no re-confirm needed at mount)
- Spine **registration-agnostic**: `rblds_Spine.sh` header states it owns no kind knowledge; recipe + substitutions are data. The stub-vs-serialize question is moot.
- Reserved body-letters `rbldt_`/`rbldr_`/`rbldw_`/`rbldv_` (tool/reliquary/wsl/podvm) **free**.
- RBSL spec cluster **A/B/D/E present**; F/C/U/I greenfield (a vertical writes its one capture subdoc on the existing RBSLE skeleton).
- Cloud steps `rbgjl01`/`rbgjl02` + shared `rbgjs/` snippet library intact.
- Cutover chokepoint **`zrbfc_resolve_tool_images` now in `rbfca_StepAssembly.sh`** (out of the former `rbfc` monolith).
- enshrine/inscribe forks **live** (`RBSAE`, `rbfli_Inscribe.sh`) — the cutover rollback path survives.

## Goal
Identify and slate the full Lode remainder as concrete paces, decomposed for parallel-chat execution per the directives below.

## Decomposition directive (settled at groom — locked)
Decompose **by kind-vertical, not by file type.** Each surface (bash / theurge-rust / RBS0-spec) splits into per-kind *body* files plus a small shared *registry spine*; the per-kind dependency runs bash-verb -> theurge-fixture-that-invokes-it -> spec (theurge's colophon-const file regenerates from the bash zipper), so a by-file-type split stalls each kind on the lane upstream of it. A kind vertical keeps that dependency inside one chat. Parallel chats share one working tree, so parallel paces **must touch disjoint files.** (That map covers three surfaces; **onboarding is a fourth** — the handbook plus its `rbtdro` sequence fixture — and it rides the cutovers, never the greenfield verticals; see Cutover technique.)

Three pace classes:
- **Scaffold (serial, first):** ₣BX **delivered** the spine (`rblds_`), the per-kind body-file slots (`rbldt_`/`rbldr_`/`rbldw_`/`rbldv_`), the shared cloud step-library, and the host-module decomposition; the shared RBS0 Lode quoin schema is **already allocated** (bole spec work). So the scaffold here shrinks to the kind-registration surface ₣BX did NOT touch — new colophons (`rbw-lF/lC/lU/lI`), kind-letter constants, the theurge fixture/suite registry entries, and the divine-legend kinds-loop. Re-derive the exact residue against ₣BX's landed layout; retires nothing, the live `enshrine`/`inscribe` paths stay intact.
- **Greenfield verticals (parallel chats, after scaffold):** each capture verb that writes `rbi_ld` and touches no live path — `tool`/`fetter`, `reliquary`/`conclave`, `wsl`/`underpin`, `podvm`/`immure`. Each **rides ₣BX's spine as a thin body file** (recipe + substitutions + envelope) on its reserved letter, touching only its own body files. On the spec surface a vertical writes its ONE capture subdoc on the **existing** `RBSLE` skeleton (`RBSLF`/`RBSLC`/`RBSLU`/`RBSLI`) plus its capture-verb quoin — never a full RBSL set; `divine`/`augur`/`banish` are kind-general and **already exist** (RBSLD/A/B).
- **Cutover paces (serial, after the matching capture verb exists):** retire a live path — the bole/`enshrine` cutover and the reliquary/`inscribe` cutover. See Cutover technique.

**The verticals are file-disjoint by construction** (spine confirmed registration-agnostic at groom — a kind is data the spine consumes), so the old stub-vs-serialize question is moot.

**Slating constraint — scrub is terminal.** A vocabulary-finalization scrub pace already sits last in the heat (the delete-old-last tail); slate every scaffold / vertical / cutover BEFORE it (`jjx_enroll before:<scrub>`), never after — nothing scrubs superseded vocabulary until that vocabulary's cutover has landed. (The bole-verb RBSL spec work has since landed; its RBS0-quoin overlap with the scaffold was resolved by **sequencing** — the shared Lode concept quoins now exist, so the scaffold must not re-allocate them.)

## Cutover technique (settled at groom — locked)
Both cutovers follow **rebuild-and-repoint, delete-old-last** — NOT a dual-read compatibility bridge. Grounding: the reliquary is regenerable (re-mirrored from upstream by `inscribe`), consumed through a single chokepoint (`zrbfc_resolve_tool_images` — now in `rbfca_StepAssembly.sh` after ₣BX's relocation), and elected through one variable (`RBRV_RELIQUARY`, written by `yoke`); the bole/base path is analogous (re-`ensconce`-able; the base coordinate is a resolved derived-pull, not a hardcode). A single-operator monorepo has no rolling-deployment lag for a bridge to guard, and a bridge would resurrect the known-bad sibling-package layout this heat exists to kill.

Each cutover: build/refresh the capture into `rbi_ld` -> repoint the chokepoint(s) and re-elect (`yoke` / the base ANCHOR) in one commit -> verify one live `conjure` build green -> banish the old namespace (`rbi_rq` / `rbi_es`) **last.** The surviving old path is the rollback.

Reliquary therefore splits the way bole already did: **`conclave`-build is a greenfield vertical (parallel); the reliquary-cutover is its own small serial pace** — mirroring the pilot (`ensconce` built; `enshrine` cutover still pending). The two cutovers are siblings of one shape and slate as such.

**Onboarding rides the cutovers — bind it into each cutover's done.** The handbook is the fourth surface (teaching, not the three-surface map). The `enshrine` track (`rbhoda`) + `RBYC_ENSHRINE` flip with the bole cutover; the `inscribe`/`reliquary` tracks (`rbhodf`/`rbhodb`/`rbhodg`) + `RBYC_RELIQUARY` + the literal `inscribe` prose flip with the reliquary cutover; the start-here hub (`rbho0`) and `rbyc_common.sh` are touched by both (fine — the cutovers serialize). Each cutover's done carries (a) its track vocabulary flip and (b) the matching `rbtdro_onboarding.rs` update — because enforcement is asymmetric: regenerating the verb consts compile-breaks the fixture (self-guarding), but the handbook-render fixture only asserts exit-0, so stale *prose* (onboarding teaching a deleted command) passes the suite silently. No test guards the prose; the cutover's done is the only guard.

Flags for the cutover docket: `inscribe` mirrors `:latest`, so a fresh re-inscribe bumps the toolchain — if a byte-identical cohort is wanted, copy the old digests into `rbi_ld` rather than re-pull. And decide deliberately whether to vocabularize the new verbs as `RBYC_*` constants (consistency with `RBYC_ENSHRINE`/`RBYC_RELIQUARY`) or leave them prose.

## Topics the slated paces must cover
Durable inventory — fold each into a pace of the right class above.

- RBS0* reconciliation: the bole `RBSL` cluster + shared Lode quoins are written; the per-kind capture subdocs ride the verticals; cutovers retire the old `enshrine`/`enshrinement` quoins and rewire cross-refs.
- GAR namespace collapse: whether `rbi_es` + `rbi_rq` fold into one Lode shape. (cutovers)
- Cutover: rewire the base derived-pull ANCHOR to read the capture-file; retire the `enshrine` path. (bole cutover)
- Remaining kinds — `tool`, `reliquary`, `wsl`, `podvm-wsl`, `podvm-native` — each its own capture verb + tests; `reliquary`/`podvm` prove the multi-member package layout. (greenfield verticals; reliquary also gets a cutover pace)
- Provenance attachment mechanism: reserved tags vs OCI referrers (verify GAR maturity).
- Full test coverage per kind — the pilot set the architecture; coverage scales out here. (rides each vertical/cutover; note service-tier live-GAR test *runs* serialize even when editing parallelizes — they share regime state)
- Onboarding (fourth surface): the Director-track vocabulary flip + the `rbtdro` update ride the cutovers — see Cutover technique. Separately, wsl/podvm onboarding is greenfield *and* deferred (host consumption out of scope this heat) → a standalone housekeeping action that marks the stale Windows tracks (`rbhw*`) NOT-available/deferred rather than writing them. Resist a unified Lode-onboarding track — onboarding stays distributed by consumer.
- Public-facing concept docs: `README.md` glossary + Roadmap + project page. (rides the cutovers, paired with the code that makes each entry true; whether the glossary stays concept-level or enumerates per-kind verbs decides whether any vertical must touch README — settle with the disjointness goal in mind)
- The ₣A- WSL-stage DEV CACHE revert pace: drop or transfer once wsl-kind work commits.

## Done
The remainder is slated as concrete paces across the three classes — scaffold-residue, greenfield verticals, cutovers — each with a docket, decomposed so the verticals are file-disjoint and parallel-chat-drivable (riding ₣BX's registration-agnostic spine, so the old stub-vs-serialize question is moot). This checkpoint wraps.

**[260605-0857] rough**

## Character
**₣BX has wrapped — this pace is mount-ready.** The "mount only after ₣BX wraps" gate is discharged (see paddock "Cross-heat dependency"). ₣BX built the capture spine + host-module decomposition this heat's verticals ride, and this groom re-baselined the docket/paddock against ₣BX's landed layout — so mount goes straight to slating, not re-discovery. The decomposition axis (by kind-vertical) and the cutover technique were settled at groom and stay; what ₣BX changed was the *scaffold*, now its landed deliverable, not ours. Still a planning pace: the deliverable is concrete slated paces, not code.

## Groom re-baseline (confirmed against ₣BX's landed tree — no re-confirm needed at mount)
- Spine **registration-agnostic**: `rblds_Spine.sh` header states it owns no kind knowledge; recipe + substitutions are data. The stub-vs-serialize question is moot.
- Reserved body-letters `rbldt_`/`rbldr_`/`rbldw_`/`rbldv_` (tool/reliquary/wsl/podvm) **free**.
- RBSL spec cluster **A/B/D/E present**; F/C/U/I greenfield (a vertical writes its one capture subdoc on the existing RBSLE skeleton).
- Cloud steps `rbgjl01`/`rbgjl02` + shared `rbgjs/` snippet library intact.
- Cutover chokepoint **`zrbfc_resolve_tool_images` now in `rbfca_StepAssembly.sh`** (out of the former `rbfc` monolith).
- enshrine/inscribe forks **live** (`RBSAE`, `rbfli_Inscribe.sh`) — the cutover rollback path survives.

## Goal
Identify and slate the full Lode remainder as concrete paces, decomposed for parallel-chat execution per the directives below.

## Decomposition directive (settled at groom — locked)
Decompose **by kind-vertical, not by file type.** Each surface (bash / theurge-rust / RBS0-spec) splits into per-kind *body* files plus a small shared *registry spine*; the per-kind dependency runs bash-verb -> theurge-fixture-that-invokes-it -> spec (theurge's colophon-const file regenerates from the bash zipper), so a by-file-type split stalls each kind on the lane upstream of it. A kind vertical keeps that dependency inside one chat. Parallel chats share one working tree, so parallel paces **must touch disjoint files.** (That map covers three surfaces; **onboarding is a fourth** — the handbook plus its `rbtdro` sequence fixture — and it rides the cutovers, never the greenfield verticals; see Cutover technique.)

Three pace classes:
- **Scaffold (serial, first):** ₣BX **delivered** the spine (`rblds_`), the per-kind body-file slots (`rbldt_`/`rbldr_`/`rbldw_`/`rbldv_`), the shared cloud step-library, and the host-module decomposition; the shared RBS0 Lode quoin schema is **already allocated** (bole spec work). So the scaffold here shrinks to the kind-registration surface ₣BX did NOT touch — new colophons (`rbw-lF/lC/lU/lI`), kind-letter constants, the theurge fixture/suite registry entries, and the divine-legend kinds-loop. Re-derive the exact residue against ₣BX's landed layout; retires nothing, the live `enshrine`/`inscribe` paths stay intact.
- **Greenfield verticals (parallel chats, after scaffold):** each capture verb that writes `rbi_ld` and touches no live path — `tool`/`fetter`, `reliquary`/`conclave`, `wsl`/`underpin`, `podvm`/`immure`. Each **rides ₣BX's spine as a thin body file** (recipe + substitutions + envelope) on its reserved letter, touching only its own body files. On the spec surface a vertical writes its ONE capture subdoc on the **existing** `RBSLE` skeleton (`RBSLF`/`RBSLC`/`RBSLU`/`RBSLI`) plus its capture-verb quoin — never a full RBSL set; `divine`/`augur`/`banish` are kind-general and **already exist** (RBSLD/A/B).
- **Cutover paces (serial, after the matching capture verb exists):** retire a live path — the bole/`enshrine` cutover and the reliquary/`inscribe` cutover. See Cutover technique.

**The verticals are file-disjoint by construction** (spine confirmed registration-agnostic at groom — a kind is data the spine consumes), so the old stub-vs-serialize question is moot.

**Slating constraint — scrub is terminal.** A vocabulary-finalization scrub pace already sits last in the heat (the delete-old-last tail); slate every scaffold / vertical / cutover BEFORE it (`jjx_enroll before:<scrub>`), never after — nothing scrubs superseded vocabulary until that vocabulary's cutover has landed. (The bole-verb RBSL spec work has since landed; its RBS0-quoin overlap with the scaffold was resolved by **sequencing** — the shared Lode concept quoins now exist, so the scaffold must not re-allocate them.)

## Cutover technique (settled at groom — locked)
Both cutovers follow **rebuild-and-repoint, delete-old-last** — NOT a dual-read compatibility bridge. Grounding: the reliquary is regenerable (re-mirrored from upstream by `inscribe`), consumed through a single chokepoint (`zrbfc_resolve_tool_images` — now in `rbfca_StepAssembly.sh` after ₣BX's relocation), and elected through one variable (`RBRV_RELIQUARY`, written by `yoke`); the bole/base path is analogous (re-`ensconce`-able; the base coordinate is a resolved derived-pull, not a hardcode). A single-operator monorepo has no rolling-deployment lag for a bridge to guard, and a bridge would resurrect the known-bad sibling-package layout this heat exists to kill.

Each cutover: build/refresh the capture into `rbi_ld` -> repoint the chokepoint(s) and re-elect (`yoke` / the base ANCHOR) in one commit -> verify one live `conjure` build green -> banish the old namespace (`rbi_rq` / `rbi_es`) **last.** The surviving old path is the rollback.

Reliquary therefore splits the way bole already did: **`conclave`-build is a greenfield vertical (parallel); the reliquary-cutover is its own small serial pace** — mirroring the pilot (`ensconce` built; `enshrine` cutover still pending). The two cutovers are siblings of one shape and slate as such.

**Onboarding rides the cutovers — bind it into each cutover's done.** The handbook is the fourth surface (teaching, not the three-surface map). The `enshrine` track (`rbhoda`) + `RBYC_ENSHRINE` flip with the bole cutover; the `inscribe`/`reliquary` tracks (`rbhodf`/`rbhodb`/`rbhodg`) + `RBYC_RELIQUARY` + the literal `inscribe` prose flip with the reliquary cutover; the start-here hub (`rbho0`) and `rbyc_common.sh` are touched by both (fine — the cutovers serialize). Each cutover's done carries (a) its track vocabulary flip and (b) the matching `rbtdro_onboarding.rs` update — because enforcement is asymmetric: regenerating the verb consts compile-breaks the fixture (self-guarding), but the handbook-render fixture only asserts exit-0, so stale *prose* (onboarding teaching a deleted command) passes the suite silently. No test guards the prose; the cutover's done is the only guard.

Flags for the cutover docket: `inscribe` mirrors `:latest`, so a fresh re-inscribe bumps the toolchain — if a byte-identical cohort is wanted, copy the old digests into `rbi_ld` rather than re-pull. And decide deliberately whether to vocabularize the new verbs as `RBYC_*` constants (consistency with `RBYC_ENSHRINE`/`RBYC_RELIQUARY`) or leave them prose.

## Topics the slated paces must cover
Durable inventory — fold each into a pace of the right class above.

- RBS0* reconciliation: the bole `RBSL` cluster + shared Lode quoins are written; the per-kind capture subdocs ride the verticals; cutovers retire the old `enshrine`/`enshrinement` quoins and rewire cross-refs.
- GAR namespace collapse: whether `rbi_es` + `rbi_rq` fold into one Lode shape. (cutovers)
- Cutover: rewire the base derived-pull ANCHOR to read the capture-file; retire the `enshrine` path. (bole cutover)
- Remaining kinds — `tool`, `reliquary`, `wsl`, `podvm-wsl`, `podvm-native` — each its own capture verb + tests; `reliquary`/`podvm` prove the multi-member package layout. (greenfield verticals; reliquary also gets a cutover pace)
- Provenance attachment mechanism: reserved tags vs OCI referrers (verify GAR maturity).
- Full test coverage per kind — the pilot set the architecture; coverage scales out here. (rides each vertical/cutover; note service-tier live-GAR test *runs* serialize even when editing parallelizes — they share regime state)
- Onboarding (fourth surface): the Director-track vocabulary flip + the `rbtdro` update ride the cutovers — see Cutover technique. Separately, wsl/podvm onboarding is greenfield *and* deferred (host consumption out of scope this heat) → a standalone housekeeping action that marks the stale Windows tracks (`rbhw*`) NOT-available/deferred rather than writing them. Resist a unified Lode-onboarding track — onboarding stays distributed by consumer.
- Public-facing concept docs: `README.md` glossary + Roadmap + project page. (rides the cutovers, paired with the code that makes each entry true; whether the glossary stays concept-level or enumerates per-kind verbs decides whether any vertical must touch README — settle with the disjointness goal in mind)
- The ₣A- WSL-stage DEV CACHE revert pace: drop or transfer once wsl-kind work commits.

## Done
The remainder is slated as concrete paces across the three classes — scaffold-residue, greenfield verticals, cutovers — each with a docket, decomposed so the verticals are file-disjoint and parallel-chat-drivable (riding ₣BX's registration-agnostic spine, so the old stub-vs-serialize question is moot). This checkpoint wraps.

**[260604-1033] rough**

## Character
**Mount only after ₣BX wraps** — it builds the capture spine + host-module decomposition this heat's verticals ride; re-baseline this docket and the paddock against ₣BX's landed file layout before slating (see paddock "Cross-heat dependency — ₣BX"). Re-plan checkpoint: the decomposition axis (by kind-vertical) and the cutover technique were settled at groom and stay — what ₣BX changes is the *scaffold*, most of which is now its deliverable, not ours. Still a planning pace: the deliverable is concrete slated paces, not code.

## Goal
Identify and slate the full Lode remainder as concrete paces, decomposed for parallel-chat execution per the directives below.

## Decomposition directive (settled at groom — locked)
Decompose **by kind-vertical, not by file type.** Each surface (bash / theurge-rust / RBS0-spec) splits into per-kind *body* files plus a small shared *registry spine*; the per-kind dependency runs bash-verb -> theurge-fixture-that-invokes-it -> spec (theurge's colophon-const file regenerates from the bash zipper), so a by-file-type split stalls each kind on the lane upstream of it. A kind vertical keeps that dependency inside one chat. Parallel chats share one working tree, so parallel paces **must touch disjoint files.** (That map covers three surfaces; **onboarding is a fourth** — the handbook plus its `rbtdro` sequence fixture — and it rides the cutovers, never the greenfield verticals; see Cutover technique.)

Three pace classes:
- **Scaffold (serial, first):** ₣BX delivers the spine (`rblds_`), the per-kind body-file slots (`rbldt_`/`rbldr_`/`rbldw_`/`rbldv_`), the shared cloud step-library, and the host-module decomposition; the shared RBS0 Lode quoin schema is **already allocated** (bole spec work). So the scaffold here shrinks to the kind-registration surface ₣BX does NOT touch — new colophons (`rbw-lF/lC/lU/lI`), kind-letter constants, the theurge fixture/suite registry entries, and the divine-legend kinds-loop. Re-derive the exact residue against ₣BX's landed layout; retires nothing, the live `enshrine`/`inscribe` paths stay intact.
- **Greenfield verticals (parallel chats, after scaffold):** each capture verb that writes `rbi_ld` and touches no live path — `tool`/`fetter`, `reliquary`/`conclave`, `wsl`/`underpin`, `podvm`/`immure`. Each **rides ₣BX's spine as a thin body file** (recipe + substitutions + envelope) on its reserved letter, touching only its own body files. On the spec surface a vertical writes its ONE capture subdoc on the **existing** `RBSLE` skeleton (`RBSLF`/`RBSLC`/`RBSLU`/`RBSLI`) plus its capture-verb quoin — never a full RBSL set; `divine`/`augur`/`banish` are kind-general and **already exist** (RBSLD/A/B).
- **Cutover paces (serial, after the matching capture verb exists):** retire a live path — the bole/`enshrine` cutover and the reliquary/`inscribe` cutover. See Cutover technique.

**Settle at mount:** ₣BX's spine is registration-agnostic — a kind is a body file the spine consumes as data — so the verticals are file-disjoint by construction and the old stub-vs-serialize question is **moot**. At mount, confirm the spine landed registration-agnostic and the reserved body-letters are free.

**Slating constraint — scrub is terminal.** A vocabulary-finalization scrub pace already sits last in the heat (the delete-old-last tail); slate every scaffold / vertical / cutover BEFORE it (`jjx_enroll before:<scrub>`), never after — nothing scrubs superseded vocabulary until that vocabulary's cutover has landed. (The bole-verb RBSL spec work has since landed; its RBS0-quoin overlap with the scaffold was resolved by **sequencing** — the shared Lode concept quoins now exist, so the scaffold must not re-allocate them.)

## Cutover technique (settled at groom — locked)
Both cutovers follow **rebuild-and-repoint, delete-old-last** — NOT a dual-read compatibility bridge. Grounding: the reliquary is regenerable (re-mirrored from upstream by `inscribe`), consumed through a single chokepoint (`zrbfc_resolve_tool_images` — ₣BX relocates it out of the `rbfc` monolith into its cluster file; find it at mount), and elected through one variable (`RBRV_RELIQUARY`, written by `yoke`); the bole/base path is analogous (re-`ensconce`-able; the base coordinate is a resolved derived-pull, not a hardcode). A single-operator monorepo has no rolling-deployment lag for a bridge to guard, and a bridge would resurrect the known-bad sibling-package layout this heat exists to kill.

Each cutover: build/refresh the capture into `rbi_ld` -> repoint the chokepoint(s) and re-elect (`yoke` / the base ANCHOR) in one commit -> verify one live `conjure` build green -> banish the old namespace (`rbi_rq` / `rbi_es`) **last.** The surviving old path is the rollback.

Reliquary therefore splits the way bole already did: **`conclave`-build is a greenfield vertical (parallel); the reliquary-cutover is its own small serial pace** — mirroring the pilot (`ensconce` built; `enshrine` cutover still pending). The two cutovers are siblings of one shape and slate as such.

**Onboarding rides the cutovers — bind it into each cutover's done.** The handbook is the fourth surface (teaching, not the three-surface map). The `enshrine` track (`rbhoda`) + `RBYC_ENSHRINE` flip with the bole cutover; the `inscribe`/`reliquary` tracks (`rbhodf`/`rbhodb`/`rbhodg`) + `RBYC_RELIQUARY` + the literal `inscribe` prose flip with the reliquary cutover; the start-here hub (`rbho0`) and `rbyc_common.sh` are touched by both (fine — the cutovers serialize). Each cutover's done carries (a) its track vocabulary flip and (b) the matching `rbtdro_onboarding.rs` update — because enforcement is asymmetric: regenerating the verb consts compile-breaks the fixture (self-guarding), but the handbook-render fixture only asserts exit-0, so stale *prose* (onboarding teaching a deleted command) passes the suite silently. No test guards the prose; the cutover's done is the only guard.

Flags for the cutover docket: `inscribe` mirrors `:latest`, so a fresh re-inscribe bumps the toolchain — if a byte-identical cohort is wanted, copy the old digests into `rbi_ld` rather than re-pull. And decide deliberately whether to vocabularize the new verbs as `RBYC_*` constants (consistency with `RBYC_ENSHRINE`/`RBYC_RELIQUARY`) or leave them prose.

## Topics the slated paces must cover
Durable inventory — fold each into a pace of the right class above.

- RBS0* reconciliation: the bole `RBSL` cluster + shared Lode quoins are written; the per-kind capture subdocs ride the verticals; cutovers retire the old `enshrine`/`enshrinement` quoins and rewire cross-refs.
- GAR namespace collapse: whether `rbi_es` + `rbi_rq` fold into one Lode shape. (cutovers)
- Cutover: rewire the base derived-pull ANCHOR to read the capture-file; retire the `enshrine` path. (bole cutover)
- Remaining kinds — `tool`, `reliquary`, `wsl`, `podvm-wsl`, `podvm-native` — each its own capture verb + tests; `reliquary`/`podvm` prove the multi-member package layout. (greenfield verticals; reliquary also gets a cutover pace)
- Provenance attachment mechanism: reserved tags vs OCI referrers (verify GAR maturity).
- Full test coverage per kind — the pilot set the architecture; coverage scales out here. (rides each vertical/cutover; note service-tier live-GAR test *runs* serialize even when editing parallelizes — they share regime state)
- Onboarding (fourth surface): the Director-track vocabulary flip + the `rbtdro` update ride the cutovers — see Cutover technique. Separately, wsl/podvm onboarding is greenfield *and* deferred (host consumption out of scope this heat) → a standalone housekeeping action that marks the stale Windows tracks (`rbhw*`) NOT-available/deferred rather than writing them. Resist a unified Lode-onboarding track — onboarding stays distributed by consumer.
- Public-facing concept docs: `README.md` glossary + Roadmap + project page. (rides the cutovers, paired with the code that makes each entry true; whether the glossary stays concept-level or enumerates per-kind verbs decides whether any vertical must touch README — settle with the disjointness goal in mind)
- The ₣A- WSL-stage DEV CACHE revert pace: drop or transfer once wsl-kind work commits.

## Done
The remainder is slated as concrete paces across the three classes — scaffold-residue, greenfield verticals, cutovers — each with a docket, decomposed so the verticals are file-disjoint and parallel-chat-drivable (riding ₣BX's registration-agnostic spine, so the old stub-vs-serialize question is moot). This checkpoint wraps.

**[260604-1014] rough**

## Character
Re-plan checkpoint. The pilot has landed and the decomposition axis + cutover technique were settled at groom against a code study — apply them, don't re-derive them. Still a planning pace: the deliverable is concrete slated paces, not code. One real choice is deferred to mount (stub-scaffold vs serialize-spine, below).

## Goal
Identify and slate the full Lode remainder as concrete paces, decomposed for parallel-chat execution per the directives below.

## Decomposition directive (settled at groom — locked)
Decompose **by kind-vertical, not by file type.** Each surface (bash / theurge-rust / RBS0-spec) splits into per-kind *body* files plus a small shared *registry spine*; the per-kind dependency runs bash-verb -> theurge-fixture-that-invokes-it -> spec (theurge's colophon-const file regenerates from the bash zipper), so a by-file-type split stalls each kind on the lane upstream of it. A kind vertical keeps that dependency inside one chat. Parallel chats share one working tree, so parallel paces **must touch disjoint files.** (That map covers three surfaces; **onboarding is a fourth** — the handbook plus its `rbtdro` sequence fixture — and it rides the cutovers, never the greenfield verticals; see Cutover technique.)

Three pace classes:
- **Scaffold (serial, first, purely additive):** reserve every shared spine slot — colophons, kind-letter constants, the theurge fixture/suite registry + lib module entries — and refactor the divine-legend to a kinds-loop. The shared RBS0 Lode quoin schema is **already allocated** (the bole spec work landed it); the scaffold adds only per-kind spine slots, never the shared concept quoins. Retires nothing; the live `enshrine`/`inscribe` paths stay intact. This is what unblocks the parallel verticals.
- **Greenfield verticals (parallel chats, after scaffold):** each capture verb that writes `rbi_ld` and touches no live path — `tool`/`fetter`, `reliquary`/`conclave`, `wsl`/`underpin`, `podvm`/`immure`. Each touches only its own body files. On the spec surface a vertical writes its ONE capture subdoc on the **existing** `RBSLE` skeleton (`RBSLF`/`RBSLC`/`RBSLU`/`RBSLI`) plus its capture-verb quoin — never a full RBSL set; `divine`/`augur`/`banish` are kind-general and **already exist** (RBSLD/A/B).
- **Cutover paces (serial, after the matching capture verb exists):** retire a live path — the bole/`enshrine` cutover and the reliquary/`inscribe` cutover. See Cutover technique.

**Settle at mount:** stub-scaffold (scaffold pre-wires spines against stub bodies, so verticals never edit a shared file) vs serialize-spine (verticals make tiny spine edits, landed sequentially). Only stub-scaffold makes the parallel chats fully independent; pick deliberately and record why.

**Slating constraint — scrub is terminal.** A vocabulary-finalization scrub pace already sits last in the heat (the delete-old-last tail); slate every scaffold / vertical / cutover BEFORE it (`jjx_enroll before:<scrub>`), never after — nothing scrubs superseded vocabulary until that vocabulary's cutover has landed. (The bole-verb RBSL spec work has since landed; its RBS0-quoin overlap with the scaffold was resolved by **sequencing** — the shared Lode concept quoins now exist, so the scaffold must not re-allocate them.)

## Cutover technique (settled at groom — locked)
Both cutovers follow **rebuild-and-repoint, delete-old-last** — NOT a dual-read compatibility bridge. Grounding: the reliquary is regenerable (re-mirrored from upstream by `inscribe`), consumed through a single chokepoint (`zrbfc_resolve_tool_images` in `rbfc_FoundryCore.sh`), and elected through one variable (`RBRV_RELIQUARY`, written by `yoke`); the bole/base path is analogous (re-`ensconce`-able; the base coordinate is a resolved derived-pull, not a hardcode). A single-operator monorepo has no rolling-deployment lag for a bridge to guard, and a bridge would resurrect the known-bad sibling-package layout this heat exists to kill.

Each cutover: build/refresh the capture into `rbi_ld` -> repoint the chokepoint(s) and re-elect (`yoke` / the base ANCHOR) in one commit -> verify one live `conjure` build green -> banish the old namespace (`rbi_rq` / `rbi_es`) **last.** The surviving old path is the rollback.

Reliquary therefore splits the way bole already did: **`conclave`-build is a greenfield vertical (parallel); the reliquary-cutover is its own small serial pace** — mirroring the pilot (`ensconce` built; `enshrine` cutover still pending). The two cutovers are siblings of one shape and slate as such.

**Onboarding rides the cutovers — bind it into each cutover's done.** The handbook is the fourth surface (teaching, not the three-surface map). The `enshrine` track (`rbhoda`) + `RBYC_ENSHRINE` flip with the bole cutover; the `inscribe`/`reliquary` tracks (`rbhodf`/`rbhodb`/`rbhodg`) + `RBYC_RELIQUARY` + the literal `inscribe` prose flip with the reliquary cutover; the start-here hub (`rbho0`) and `rbyc_common.sh` are touched by both (fine — the cutovers serialize). Each cutover's done carries (a) its track vocabulary flip and (b) the matching `rbtdro_onboarding.rs` update — because enforcement is asymmetric: regenerating the verb consts compile-breaks the fixture (self-guarding), but the handbook-render fixture only asserts exit-0, so stale *prose* (onboarding teaching a deleted command) passes the suite silently. No test guards the prose; the cutover's done is the only guard.

Flags for the cutover docket: `inscribe` mirrors `:latest`, so a fresh re-inscribe bumps the toolchain — if a byte-identical cohort is wanted, copy the old digests into `rbi_ld` rather than re-pull. And decide deliberately whether to vocabularize the new verbs as `RBYC_*` constants (consistency with `RBYC_ENSHRINE`/`RBYC_RELIQUARY`) or leave them prose.

## Topics the slated paces must cover
Durable inventory — fold each into a pace of the right class above.

- RBS0* reconciliation: the bole `RBSL` cluster + shared Lode quoins are written; the per-kind capture subdocs ride the verticals; cutovers retire the old `enshrine`/`enshrinement` quoins and rewire cross-refs.
- GAR namespace collapse: whether `rbi_es` + `rbi_rq` fold into one Lode shape. (cutovers)
- Cutover: rewire the base derived-pull ANCHOR to read the capture-file; retire the `enshrine` path. (bole cutover)
- Remaining kinds — `tool`, `reliquary`, `wsl`, `podvm-wsl`, `podvm-native` — each its own capture verb + tests; `reliquary`/`podvm` prove the multi-member package layout. (greenfield verticals; reliquary also gets a cutover pace)
- Provenance attachment mechanism: reserved tags vs OCI referrers (verify GAR maturity).
- Full test coverage per kind — the pilot set the architecture; coverage scales out here. (rides each vertical/cutover; note service-tier live-GAR test *runs* serialize even when editing parallelizes — they share regime state)
- Onboarding (fourth surface): the Director-track vocabulary flip + the `rbtdro` update ride the cutovers — see Cutover technique. Separately, wsl/podvm onboarding is greenfield *and* deferred (host consumption out of scope this heat) → a standalone housekeeping action that marks the stale Windows tracks (`rbhw*`) NOT-available/deferred rather than writing them. Resist a unified Lode-onboarding track — onboarding stays distributed by consumer.
- Public-facing concept docs: `README.md` glossary + Roadmap + project page. (rides the cutovers, paired with the code that makes each entry true; whether the glossary stays concept-level or enumerates per-kind verbs decides whether any vertical must touch README — settle with the disjointness goal in mind)
- The ₣A- WSL-stage DEV CACHE revert pace: drop or transfer once wsl-kind work commits.

## Done
The remainder is slated as concrete paces across the three classes — scaffold, greenfield verticals, cutovers — each with a docket, decomposed so the verticals are file-disjoint and parallel-chat-drivable. The stub-vs-serialize choice is made and recorded. This checkpoint wraps.

**[260603-1124] rough**

## Character
Re-plan checkpoint. The pilot has landed and the decomposition axis + cutover technique were settled at groom against a code study — apply them, don't re-derive them. Still a planning pace: the deliverable is concrete slated paces, not code. One real choice is deferred to mount (stub-scaffold vs serialize-spine, below).

## Goal
Identify and slate the full Lode remainder as concrete paces, decomposed for parallel-chat execution per the directives below.

## Decomposition directive (settled at groom — locked)
Decompose **by kind-vertical, not by file type.** Each surface (bash / theurge-rust / RBS0-spec) splits into per-kind *body* files plus a small shared *registry spine*; the per-kind dependency runs bash-verb -> theurge-fixture-that-invokes-it -> spec (theurge's colophon-const file regenerates from the bash zipper), so a by-file-type split stalls each kind on the lane upstream of it. A kind vertical keeps that dependency inside one chat. Parallel chats share one working tree, so parallel paces **must touch disjoint files.** (That map covers three surfaces; **onboarding is a fourth** — the handbook plus its `rbtdro` sequence fixture — and it rides the cutovers, never the greenfield verticals; see Cutover technique.)

Three pace classes:
- **Scaffold (serial, first, purely additive):** reserve every shared spine slot — colophons, kind-letter constants, the theurge fixture/suite registry + lib module entries, the RBS0 mapping-section Lode quoin schema — and refactor the divine-legend to a kinds-loop. Retires nothing; the live `enshrine`/`inscribe` paths stay intact. This is what unblocks the parallel verticals.
- **Greenfield verticals (parallel chats, after scaffold):** each capture verb that writes `rbi_ld` and touches no live path — `tool`/`fetter`, `reliquary`/`conclave`, `wsl`/`underpin`, `podvm`/`immure`. Each touches only its own body files. On the spec surface a vertical writes its ONE capture subdoc on the `RBSLE` skeleton (`RBSLF`/`RBSLC`/`RBSLU`/`RBSLI`) plus its capture-verb quoin — never a full RBSL set; `divine`/`augur`/`banish` are kind-general and land once in the bole spec pace.
- **Cutover paces (serial, after the matching capture verb exists):** retire a live path — the bole/`enshrine` cutover and the reliquary/`inscribe` cutover. See Cutover technique.

**Settle at mount:** stub-scaffold (scaffold pre-wires spines against stub bodies, so verticals never edit a shared file) vs serialize-spine (verticals make tiny spine edits, landed sequentially). Only stub-scaffold makes the parallel chats fully independent; pick deliberately and record why.

**Slating constraint — scrub is terminal.** A vocabulary-finalization scrub pace already sits last in the heat (the delete-old-last tail); slate every scaffold / vertical / cutover BEFORE it (`jjx_enroll before:<scrub>`), never after — nothing scrubs superseded vocabulary until that vocabulary's cutover has landed. (A dedicated bole-verb RBSL spec pace is likewise already slated; its RBS0 quoin work overlaps the scaffold — sequence or merge, do not parallel on RBS0.)

## Cutover technique (settled at groom — locked)
Both cutovers follow **rebuild-and-repoint, delete-old-last** — NOT a dual-read compatibility bridge. Grounding: the reliquary is regenerable (re-mirrored from upstream by `inscribe`), consumed through a single chokepoint (`zrbfc_resolve_tool_images` in `rbfc_FoundryCore.sh`), and elected through one variable (`RBRV_RELIQUARY`, written by `yoke`); the bole/base path is analogous (re-`ensconce`-able; the base coordinate is a resolved derived-pull, not a hardcode). A single-operator monorepo has no rolling-deployment lag for a bridge to guard, and a bridge would resurrect the known-bad sibling-package layout this heat exists to kill.

Each cutover: build/refresh the capture into `rbi_ld` -> repoint the chokepoint(s) and re-elect (`yoke` / the base ANCHOR) in one commit -> verify one live `conjure` build green -> banish the old namespace (`rbi_rq` / `rbi_es`) **last.** The surviving old path is the rollback.

Reliquary therefore splits the way bole already did: **`conclave`-build is a greenfield vertical (parallel); the reliquary-cutover is its own small serial pace** — mirroring the pilot (`ensconce` built; `enshrine` cutover still pending). The two cutovers are siblings of one shape and slate as such.

**Onboarding rides the cutovers — bind it into each cutover's done.** The handbook is the fourth surface (teaching, not the three-surface map). The `enshrine` track (`rbhoda`) + `RBYC_ENSHRINE` flip with the bole cutover; the `inscribe`/`reliquary` tracks (`rbhodf`/`rbhodb`/`rbhodg`) + `RBYC_RELIQUARY` + the literal `inscribe` prose flip with the reliquary cutover; the start-here hub (`rbho0`) and `rbyc_common.sh` are touched by both (fine — the cutovers serialize). Each cutover's done carries (a) its track vocabulary flip and (b) the matching `rbtdro_onboarding.rs` update — because enforcement is asymmetric: regenerating the verb consts compile-breaks the fixture (self-guarding), but the handbook-render fixture only asserts exit-0, so stale *prose* (onboarding teaching a deleted command) passes the suite silently. No test guards the prose; the cutover's done is the only guard.

Flags for the cutover docket: `inscribe` mirrors `:latest`, so a fresh re-inscribe bumps the toolchain — if a byte-identical cohort is wanted, copy the old digests into `rbi_ld` rather than re-pull. And decide deliberately whether to vocabularize the new verbs as `RBYC_*` constants (consistency with `RBYC_ENSHRINE`/`RBYC_RELIQUARY`) or leave them prose.

## Topics the slated paces must cover
Durable inventory — fold each into a pace of the right class above.

- RBS0* reconciliation: write the `RBSL` spec; retire `enshrine`/`enshrinement` quoins, rewire cross-refs. (scaffold allocates new quoins; cutovers retire old; the bole-verb subset is slated separately as its own spec pace)
- GAR namespace collapse: whether `rbi_es` + `rbi_rq` fold into one Lode shape. (cutovers)
- Cutover: rewire the base derived-pull ANCHOR to read the capture-file; retire the `enshrine` path. (bole cutover)
- Remaining kinds — `tool`, `reliquary`, `wsl`, `podvm-wsl`, `podvm-native` — each its own capture verb + tests; `reliquary`/`podvm` prove the multi-member package layout. (greenfield verticals; reliquary also gets a cutover pace)
- Provenance attachment mechanism: reserved tags vs OCI referrers (verify GAR maturity).
- Full test coverage per kind — the pilot set the architecture; coverage scales out here. (rides each vertical/cutover; note service-tier live-GAR test *runs* serialize even when editing parallelizes — they share regime state)
- Onboarding (fourth surface): the Director-track vocabulary flip + the `rbtdro` update ride the cutovers — see Cutover technique. Separately, wsl/podvm onboarding is greenfield *and* deferred (host consumption out of scope this heat) → a standalone housekeeping action that marks the stale Windows tracks (`rbhw*`) NOT-available/deferred rather than writing them. Resist a unified Lode-onboarding track — onboarding stays distributed by consumer.
- Public-facing concept docs: `README.md` glossary + Roadmap + project page. (rides the cutovers, paired with the code that makes each entry true; whether the glossary stays concept-level or enumerates per-kind verbs decides whether any vertical must touch README — settle with the disjointness goal in mind)
- The ₣A- WSL-stage DEV CACHE revert pace: drop or transfer once wsl-kind work commits.

## Done
The remainder is slated as concrete paces across the three classes — scaffold, greenfield verticals, cutovers — each with a docket, decomposed so the verticals are file-disjoint and parallel-chat-drivable. The stub-vs-serialize choice is made and recorded. This checkpoint wraps.

**[260603-1116] rough**

## Character
Re-plan checkpoint. The pilot has landed and the decomposition axis + cutover technique were settled at groom against a code study — apply them, don't re-derive them. Still a planning pace: the deliverable is concrete slated paces, not code. One real choice is deferred to mount (stub-scaffold vs serialize-spine, below).

## Goal
Identify and slate the full Lode remainder as concrete paces, decomposed for parallel-chat execution per the directives below.

## Decomposition directive (settled at groom — locked)
Decompose **by kind-vertical, not by file type.** Each surface (bash / theurge-rust / RBS0-spec) splits into per-kind *body* files plus a small shared *registry spine*; the per-kind dependency runs bash-verb -> theurge-fixture-that-invokes-it -> spec (theurge's colophon-const file regenerates from the bash zipper), so a by-file-type split stalls each kind on the lane upstream of it. A kind vertical keeps that dependency inside one chat. Parallel chats share one working tree, so parallel paces **must touch disjoint files.** (That map covers three surfaces; **onboarding is a fourth** — the handbook plus its `rbtdro` sequence fixture — and it rides the cutovers, never the greenfield verticals; see Cutover technique.)

Three pace classes:
- **Scaffold (serial, first, purely additive):** reserve every shared spine slot — colophons, kind-letter constants, the theurge fixture/suite registry + lib module entries, the RBS0 mapping-section Lode quoin schema — and refactor the divine-legend to a kinds-loop. Retires nothing; the live `enshrine`/`inscribe` paths stay intact. This is what unblocks the parallel verticals.
- **Greenfield verticals (parallel chats, after scaffold):** each capture verb that writes `rbi_ld` and touches no live path — `tool`/`fetter`, `reliquary`/`conclave`, `wsl`/`underpin`, `podvm`/`immure`. Each touches only its own body files.
- **Cutover paces (serial, after the matching capture verb exists):** retire a live path — the bole/`enshrine` cutover and the reliquary/`inscribe` cutover. See Cutover technique.

**Settle at mount:** stub-scaffold (scaffold pre-wires spines against stub bodies, so verticals never edit a shared file) vs serialize-spine (verticals make tiny spine edits, landed sequentially). Only stub-scaffold makes the parallel chats fully independent; pick deliberately and record why.

**Slating constraint — scrub is terminal.** A vocabulary-finalization scrub pace already sits last in the heat (the delete-old-last tail); slate every scaffold / vertical / cutover BEFORE it (`jjx_enroll before:<scrub>`), never after — nothing scrubs superseded vocabulary until that vocabulary's cutover has landed. (A dedicated bole-verb RBSL spec pace is likewise already slated; its RBS0 quoin work overlaps the scaffold — sequence or merge, do not parallel on RBS0.)

## Cutover technique (settled at groom — locked)
Both cutovers follow **rebuild-and-repoint, delete-old-last** — NOT a dual-read compatibility bridge. Grounding: the reliquary is regenerable (re-mirrored from upstream by `inscribe`), consumed through a single chokepoint (`zrbfc_resolve_tool_images` in `rbfc_FoundryCore.sh`), and elected through one variable (`RBRV_RELIQUARY`, written by `yoke`); the bole/base path is analogous (re-`ensconce`-able; the base coordinate is a resolved derived-pull, not a hardcode). A single-operator monorepo has no rolling-deployment lag for a bridge to guard, and a bridge would resurrect the known-bad sibling-package layout this heat exists to kill.

Each cutover: build/refresh the capture into `rbi_ld` -> repoint the chokepoint(s) and re-elect (`yoke` / the base ANCHOR) in one commit -> verify one live `conjure` build green -> banish the old namespace (`rbi_rq` / `rbi_es`) **last.** The surviving old path is the rollback.

Reliquary therefore splits the way bole already did: **`conclave`-build is a greenfield vertical (parallel); the reliquary-cutover is its own small serial pace** — mirroring the pilot (`ensconce` built; `enshrine` cutover still pending). The two cutovers are siblings of one shape and slate as such.

**Onboarding rides the cutovers — bind it into each cutover's done.** The handbook is the fourth surface (teaching, not the three-surface map). The `enshrine` track (`rbhoda`) + `RBYC_ENSHRINE` flip with the bole cutover; the `inscribe`/`reliquary` tracks (`rbhodf`/`rbhodb`/`rbhodg`) + `RBYC_RELIQUARY` + the literal `inscribe` prose flip with the reliquary cutover; the start-here hub (`rbho0`) and `rbyc_common.sh` are touched by both (fine — the cutovers serialize). Each cutover's done carries (a) its track vocabulary flip and (b) the matching `rbtdro_onboarding.rs` update — because enforcement is asymmetric: regenerating the verb consts compile-breaks the fixture (self-guarding), but the handbook-render fixture only asserts exit-0, so stale *prose* (onboarding teaching a deleted command) passes the suite silently. No test guards the prose; the cutover's done is the only guard.

Flags for the cutover docket: `inscribe` mirrors `:latest`, so a fresh re-inscribe bumps the toolchain — if a byte-identical cohort is wanted, copy the old digests into `rbi_ld` rather than re-pull. And decide deliberately whether to vocabularize the new verbs as `RBYC_*` constants (consistency with `RBYC_ENSHRINE`/`RBYC_RELIQUARY`) or leave them prose.

## Topics the slated paces must cover
Durable inventory — fold each into a pace of the right class above.

- RBS0* reconciliation: write the `RBSL` spec; retire `enshrine`/`enshrinement` quoins, rewire cross-refs. (scaffold allocates new quoins; cutovers retire old; the bole-verb subset is slated separately as its own spec pace)
- GAR namespace collapse: whether `rbi_es` + `rbi_rq` fold into one Lode shape. (cutovers)
- Cutover: rewire the base derived-pull ANCHOR to read the capture-file; retire the `enshrine` path. (bole cutover)
- Remaining kinds — `tool`, `reliquary`, `wsl`, `podvm-wsl`, `podvm-native` — each its own capture verb + tests; `reliquary`/`podvm` prove the multi-member package layout. (greenfield verticals; reliquary also gets a cutover pace)
- Provenance attachment mechanism: reserved tags vs OCI referrers (verify GAR maturity).
- Full test coverage per kind — the pilot set the architecture; coverage scales out here. (rides each vertical/cutover; note service-tier live-GAR test *runs* serialize even when editing parallelizes — they share regime state)
- Onboarding (fourth surface): the Director-track vocabulary flip + the `rbtdro` update ride the cutovers — see Cutover technique. Separately, wsl/podvm onboarding is greenfield *and* deferred (host consumption out of scope this heat) → a standalone housekeeping action that marks the stale Windows tracks (`rbhw*`) NOT-available/deferred rather than writing them. Resist a unified Lode-onboarding track — onboarding stays distributed by consumer.
- Public-facing concept docs: `README.md` glossary + Roadmap + project page. (rides the cutovers, paired with the code that makes each entry true; whether the glossary stays concept-level or enumerates per-kind verbs decides whether any vertical must touch README — settle with the disjointness goal in mind)
- The ₣A- WSL-stage DEV CACHE revert pace: drop or transfer once wsl-kind work commits.

## Done
The remainder is slated as concrete paces across the three classes — scaffold, greenfield verticals, cutovers — each with a docket, decomposed so the verticals are file-disjoint and parallel-chat-drivable. The stub-vs-serialize choice is made and recorded. This checkpoint wraps.

**[260602-1411] rough**

## Character
Re-plan checkpoint. The pilot has landed and the decomposition axis + cutover technique were settled at groom against a code study — apply them, don't re-derive them. Still a planning pace: the deliverable is concrete slated paces, not code. One real choice is deferred to mount (stub-scaffold vs serialize-spine, below).

## Goal
Identify and slate the full Lode remainder as concrete paces, decomposed for parallel-chat execution per the directives below.

## Decomposition directive (settled at groom — locked)
Decompose **by kind-vertical, not by file type.** Each surface (bash / theurge-rust / RBS0-spec) splits into per-kind *body* files plus a small shared *registry spine*; the per-kind dependency runs bash-verb -> theurge-fixture-that-invokes-it -> spec (theurge's colophon-const file regenerates from the bash zipper), so a by-file-type split stalls each kind on the lane upstream of it. A kind vertical keeps that dependency inside one chat. Parallel chats share one working tree, so parallel paces **must touch disjoint files.** (That map covers three surfaces; **onboarding is a fourth** — the handbook plus its `rbtdro` sequence fixture — and it rides the cutovers, never the greenfield verticals; see Cutover technique.)

Three pace classes:
- **Scaffold (serial, first, purely additive):** reserve every shared spine slot — colophons, kind-letter constants, the theurge fixture/suite registry + lib module entries, the RBS0 mapping-section Lode quoin schema — and refactor the divine-legend to a kinds-loop. Retires nothing; the live `enshrine`/`inscribe` paths stay intact. This is what unblocks the parallel verticals.
- **Greenfield verticals (parallel chats, after scaffold):** each capture verb that writes `rbi_ld` and touches no live path — `tool`/`fetter`, `reliquary`/`conclave`, `wsl`/`underpin`, `podvm`/`immure`. Each touches only its own body files.
- **Cutover paces (serial, after the matching capture verb exists):** retire a live path — the bole/`enshrine` cutover and the reliquary/`inscribe` cutover. See Cutover technique.

**Settle at mount:** stub-scaffold (scaffold pre-wires spines against stub bodies, so verticals never edit a shared file) vs serialize-spine (verticals make tiny spine edits, landed sequentially). Only stub-scaffold makes the parallel chats fully independent; pick deliberately and record why.

## Cutover technique (settled at groom — locked)
Both cutovers follow **rebuild-and-repoint, delete-old-last** — NOT a dual-read compatibility bridge. Grounding: the reliquary is regenerable (re-mirrored from upstream by `inscribe`), consumed through a single chokepoint (`zrbfc_resolve_tool_images` in `rbfc_FoundryCore.sh`), and elected through one variable (`RBRV_RELIQUARY`, written by `yoke`); the bole/base path is analogous (re-`ensconce`-able; the base coordinate is a resolved derived-pull, not a hardcode). A single-operator monorepo has no rolling-deployment lag for a bridge to guard, and a bridge would resurrect the known-bad sibling-package layout this heat exists to kill.

Each cutover: build/refresh the capture into `rbi_ld` -> repoint the chokepoint(s) and re-elect (`yoke` / the base ANCHOR) in one commit -> verify one live `conjure` build green -> banish the old namespace (`rbi_rq` / `rbi_es`) **last.** The surviving old path is the rollback.

Reliquary therefore splits the way bole already did: **`conclave`-build is a greenfield vertical (parallel); the reliquary-cutover is its own small serial pace** — mirroring the pilot (`ensconce` built; `enshrine` cutover still pending). The two cutovers are siblings of one shape and slate as such.

**Onboarding rides the cutovers — bind it into each cutover's done.** The handbook is the fourth surface (teaching, not the three-surface map). The `enshrine` track (`rbhoda`) + `RBYC_ENSHRINE` flip with the bole cutover; the `inscribe`/`reliquary` tracks (`rbhodf`/`rbhodb`/`rbhodg`) + `RBYC_RELIQUARY` + the literal `inscribe` prose flip with the reliquary cutover; the start-here hub (`rbho0`) and `rbyc_common.sh` are touched by both (fine — the cutovers serialize). Each cutover's done carries (a) its track vocabulary flip and (b) the matching `rbtdro_onboarding.rs` update — because enforcement is asymmetric: regenerating the verb consts compile-breaks the fixture (self-guarding), but the handbook-render fixture only asserts exit-0, so stale *prose* (onboarding teaching a deleted command) passes the suite silently. No test guards the prose; the cutover's done is the only guard.

Flags for the cutover docket: `inscribe` mirrors `:latest`, so a fresh re-inscribe bumps the toolchain — if a byte-identical cohort is wanted, copy the old digests into `rbi_ld` rather than re-pull. And decide deliberately whether to vocabularize the new verbs as `RBYC_*` constants (consistency with `RBYC_ENSHRINE`/`RBYC_RELIQUARY`) or leave them prose.

## Topics the slated paces must cover
Durable inventory — fold each into a pace of the right class above.

- RBS0* reconciliation: write the `RBSL` spec; retire `enshrine`/`enshrinement` quoins, rewire cross-refs. (scaffold allocates new quoins; cutovers retire old)
- GAR namespace collapse: whether `rbi_es` + `rbi_rq` fold into one Lode shape. (cutovers)
- Cutover: rewire the base derived-pull ANCHOR to read the capture-file; retire the `enshrine` path. (bole cutover)
- Remaining kinds — `tool`, `reliquary`, `wsl`, `podvm-wsl`, `podvm-native` — each its own capture verb + tests; `reliquary`/`podvm` prove the multi-member package layout. (greenfield verticals; reliquary also gets a cutover pace)
- Provenance attachment mechanism: reserved tags vs OCI referrers (verify GAR maturity).
- Full test coverage per kind — the pilot set the architecture; coverage scales out here. (rides each vertical/cutover; note service-tier live-GAR test *runs* serialize even when editing parallelizes — they share regime state)
- Onboarding (fourth surface): the Director-track vocabulary flip + the `rbtdro` update ride the cutovers — see Cutover technique. Separately, wsl/podvm onboarding is greenfield *and* deferred (host consumption out of scope this heat) → a standalone housekeeping action that marks the stale Windows tracks (`rbhw*`) NOT-available/deferred rather than writing them. Resist a unified Lode-onboarding track — onboarding stays distributed by consumer.
- Public-facing concept docs: `README.md` glossary + Roadmap + project page. (rides the cutovers, paired with the code that makes each entry true; whether the glossary stays concept-level or enumerates per-kind verbs decides whether any vertical must touch README — settle with the disjointness goal in mind)
- The ₣A- WSL-stage DEV CACHE revert pace: drop or transfer once wsl-kind work commits.

## Done
The remainder is slated as concrete paces across the three classes — scaffold, greenfield verticals, cutovers — each with a docket, decomposed so the verticals are file-disjoint and parallel-chat-drivable. The stub-vs-serialize choice is made and recorded. This checkpoint wraps.

**[260602-1402] rough**

## Character
Re-plan checkpoint. The pilot has landed and the decomposition axis + cutover technique were settled at groom against a code study — apply them, don't re-derive them. Still a planning pace: the deliverable is concrete slated paces, not code. One real choice is deferred to mount (stub-scaffold vs serialize-spine, below).

## Goal
Identify and slate the full Lode remainder as concrete paces, decomposed for parallel-chat execution per the directives below.

## Decomposition directive (settled at groom — locked)
Decompose **by kind-vertical, not by file type.** Each surface (bash / theurge-rust / RBS0-spec) splits into per-kind *body* files plus a small shared *registry spine*; the per-kind dependency runs bash-verb -> theurge-fixture-that-invokes-it -> spec (theurge's colophon-const file regenerates from the bash zipper), so a by-file-type split stalls each kind on the lane upstream of it. A kind vertical keeps that dependency inside one chat. Parallel chats share one working tree, so parallel paces **must touch disjoint files.**

Three pace classes:
- **Scaffold (serial, first, purely additive):** reserve every shared spine slot — colophons, kind-letter constants, the theurge fixture/suite registry + lib module entries, the RBS0 mapping-section Lode quoin schema — and refactor the divine-legend to a kinds-loop. Retires nothing; the live `enshrine`/`inscribe` paths stay intact. This is what unblocks the parallel verticals.
- **Greenfield verticals (parallel chats, after scaffold):** each capture verb that writes `rbi_ld` and touches no live path — `tool`/`fetter`, `reliquary`/`conclave`, `wsl`/`underpin`, `podvm`/`immure`. Each touches only its own body files.
- **Cutover paces (serial, after the matching capture verb exists):** retire a live path — the bole/`enshrine` cutover and the reliquary/`inscribe` cutover. See Cutover technique.

**Settle at mount:** stub-scaffold (scaffold pre-wires spines against stub bodies, so verticals never edit a shared file) vs serialize-spine (verticals make tiny spine edits, landed sequentially). Only stub-scaffold makes the parallel chats fully independent; pick deliberately and record why.

## Cutover technique (settled at groom — locked)
Both cutovers follow **rebuild-and-repoint, delete-old-last** — NOT a dual-read compatibility bridge. Grounding: the reliquary is regenerable (re-mirrored from upstream by `inscribe`), consumed through a single chokepoint (`zrbfc_resolve_tool_images` in `rbfc_FoundryCore.sh`), and elected through one variable (`RBRV_RELIQUARY`, written by `yoke`); the bole/base path is analogous (re-`ensconce`-able; the base coordinate is a resolved derived-pull, not a hardcode). A single-operator monorepo has no rolling-deployment lag for a bridge to guard, and a bridge would resurrect the known-bad sibling-package layout this heat exists to kill.

Each cutover: build/refresh the capture into `rbi_ld` -> repoint the chokepoint(s) and re-elect (`yoke` / the base ANCHOR) in one commit -> verify one live `conjure` build green -> banish the old namespace (`rbi_rq` / `rbi_es`) **last.** The surviving old path is the rollback.

Reliquary therefore splits the way bole already did: **`conclave`-build is a greenfield vertical (parallel); the reliquary-cutover is its own small serial pace** — mirroring the pilot (`ensconce` built; `enshrine` cutover still pending). The two cutovers are siblings of one shape and slate as such.

Flag for the cutover docket: `inscribe` mirrors `:latest`, so a fresh re-inscribe bumps the toolchain. If a byte-identical cohort is wanted, copy the old digests into `rbi_ld` rather than re-pull.

## Topics the slated paces must cover
Durable inventory — fold each into a pace of the right class above.

- RBS0* reconciliation: write the `RBSL` spec; retire `enshrine`/`enshrinement` quoins, rewire cross-refs. (scaffold allocates new quoins; cutovers retire old)
- GAR namespace collapse: whether `rbi_es` + `rbi_rq` fold into one Lode shape. (cutovers)
- Cutover: rewire the base derived-pull ANCHOR to read the capture-file; retire the `enshrine` path. (bole cutover)
- Remaining kinds — `tool`, `reliquary`, `wsl`, `podvm-wsl`, `podvm-native` — each its own capture verb + tests; `reliquary`/`podvm` prove the multi-member package layout. (greenfield verticals; reliquary also gets a cutover pace)
- Provenance attachment mechanism: reserved tags vs OCI referrers (verify GAR maturity).
- Full test coverage per kind — the pilot set the architecture; coverage scales out here. (rides each vertical/cutover; note service-tier live-GAR test *runs* serialize even when editing parallelizes — they share regime state)
- Onboarding surface: rename tri-surfaced `base`/`tool`/`reliquary`; greenfield `wsl`/`podvm`; mark stale Windows tracks deferred.
- Public-facing concept docs: `README.md` glossary + Roadmap + project page. (rides the cutovers, paired with the code that makes each entry true; whether the glossary stays concept-level or enumerates per-kind verbs decides whether any vertical must touch README — settle with the disjointness goal in mind)
- The ₣A- WSL-stage DEV CACHE revert pace: drop or transfer once wsl-kind work commits.

## Done
The remainder is slated as concrete paces across the three classes — scaffold, greenfield verticals, cutovers — each with a docket, decomposed so the verticals are file-disjoint and parallel-chat-drivable. The stub-vs-serialize choice is made and recorded. This checkpoint wraps.

**[260602-1347] rough**

## Character
Re-plan checkpoint. The pilot has landed and the decomposition axis was settled at groom against a code study — apply it, don't re-derive it. Still a planning pace: the deliverable is concrete slated paces, not code. One real choice is deferred to mount (stub-scaffold vs serialize-spine, below).

## Goal
Identify and slate the full Lode remainder as concrete paces, decomposed for parallel-chat execution per the directive below.

## Decomposition directive (settled at groom — locked)
Decompose **by kind-vertical, not by file type.** Grounding from the code study: each surface (bash / theurge-rust / RBS0-spec) splits into per-kind *body* files plus a small shared *registry spine*. The per-kind dependency runs bash-verb -> theurge-fixture-that-invokes-it -> spec, and theurge's colophon-const file is regenerated from the bash zipper — so a by-file-type split (a "rust lane", a "README lane") stalls each kind on the lane upstream of it and risks splitting the single enshrine->Lode conceptual flip across lanes. A kind vertical keeps that dependency inside one chat.

Parallel chats share one working tree, so parallel paces **must touch disjoint files.** Hence the shape:

- **One serial foundation pace** owns *every* shared spine file — the colophon zipper, the kind-letter constants, the theurge fixture/suite registries + lib module list, the RBS0 mapping section — and **folds in the enshrine retirement and the README/project-page conceptual flip**, which touch shared files and are one semantic act, not separate lanes. The base derived-pull ANCHOR rewire belongs here too.
- **Per-kind verticals** (`tool`, `reliquary`, `wsl`, `podvm-*`) that, once the foundation has frozen the spines, touch only their own body files across all three surfaces — parallel-chat-safe, one kind per chat.

**Settle at mount:** stub-scaffold (foundation pre-wires the spines against stub bodies, so verticals never edit a shared file) vs serialize-spine (verticals make tiny spine edits, landed sequentially by an integration step). Only stub-scaffold makes the parallel chats fully independent; pick deliberately and record the reasoning.

## Topics the slated paces must cover
Durable inventory — fold each into the foundation pace or a kind vertical per the directive.

- RBS0* reconciliation: retire `enshrine`/`enshrinement` quoins, rewire cross-refs; write the `RBSL` spec. (foundation)
- GAR namespace collapse: whether `rbi_es` + `rbi_rq` fold into one Lode shape.
- Cutover: rewire the base derived-pull ANCHOR to read the capture-file; retire the `enshrine` path. (foundation)
- Remaining kinds — `tool`, `reliquary`, `wsl`, `podvm-wsl`, `podvm-native` — each its own capture verb + tests; `reliquary`/`podvm` prove the multi-member package layout. (verticals)
- Provenance attachment mechanism: reserved tags vs OCI referrers (verify GAR maturity).
- Full test coverage per kind — the pilot set the architecture; coverage scales out here. (rides each vertical)
- Onboarding surface: rename tri-surfaced `base`/`tool`/`reliquary`; greenfield `wsl`/`podvm`; mark stale Windows tracks deferred.
- Public-facing concept docs: `README.md` glossary + Roadmap + project page — the conceptual cutover, flipped once. (foundation)
- The ₣A- WSL-stage DEV CACHE revert pace: drop or transfer once wsl-kind work commits.

## Done
The remainder is slated as concrete paces — one serial foundation pace plus the per-kind verticals — each with a docket, decomposed so the verticals are file-disjoint and parallel-chat-drivable. The stub-vs-serialize choice is made and recorded. This checkpoint wraps.

**[260602-0637] rough**

## Character
Re-plan checkpoint, not work. Mount only after the pilot lands; re-groom the remainder informed by what the pilot taught — do not pre-decide here.

## Goal
Identify and slate the full Lode remainder, now that the pilot has settled the backbone shape (naming, envelope, test architecture).

## Topics to re-investigate (durable list — settle at re-groom, not now)
- RBS0* reconciliation: retire `enshrine`/`enshrinement` quoins, rewire cross-refs; write/finish the `RBSL` spec.
- GAR namespace collapse: whether `rbi_es` + `rbi_rq` fold into one Lode shape.
- Cutover: rewire the base derived-pull ANCHOR to read the capture-file; retire the `enshrine` path.
- Remaining kinds — `tool`, `reliquary`, `wsl`, `podvm-wsl`, `podvm-native` — each its own capture verb + tests; `reliquary`/`podvm` prove the multi-member package layout.
- Provenance attachment mechanism: reserved tags vs OCI referrers (verify GAR maturity).
- Full test coverage per kind (the pilot sets architecture; coverage scales out here).
- Onboarding surface: rename tri-surfaced `base`/`tool`/`reliquary`; greenfield `wsl`/`podvm`; mark stale Windows tracks deferred.
- Public-facing concept docs: `README.md` glossary (retire `Enshrine`/`Reliquary` entries -> `Lode`/`Touchmark` + per-kind verbs), the Roadmap section, and the project page — the conceptual cutover, flipped once when `enshrine` retires, never per-kind.
- The ₣A- WSL-stage DEV CACHE revert pace: drop or transfer once wsl-kind work commits.

## Done
Remainder is groomed and slated as concrete paces; this checkpoint wraps.

**[260601-0620] rough**

## Character
Re-plan checkpoint, not work. Mount only after the pilot lands; re-groom the remainder informed by what the pilot taught — do not pre-decide here.

## Goal
Identify and slate the full Lode remainder, now that the pilot has settled the backbone shape (naming, envelope, test architecture).

## Topics to re-investigate (durable list — settle at re-groom, not now)
- RBS0* reconciliation: retire `enshrine`/`enshrinement` quoins, rewire cross-refs; write/finish the `RBSL` spec.
- GAR namespace collapse: whether `rbi_es` + `rbi_rq` fold into one Lode shape.
- Cutover: rewire the base derived-pull ANCHOR to read the capture-file; retire the `enshrine` path.
- Remaining kinds — `tool`, `reliquary`, `wsl`, `podvm-wsl`, `podvm-native` — each its own capture verb + tests; `reliquary`/`podvm` prove the multi-member package layout.
- Provenance attachment mechanism: reserved tags vs OCI referrers (verify GAR maturity).
- Full test coverage per kind (the pilot sets architecture; coverage scales out here).
- Onboarding surface: rename tri-surfaced `base`/`tool`/`reliquary`; greenfield `wsl`/`podvm`; mark stale Windows tracks deferred.
- The ₣A- WSL-stage DEV CACHE revert pace: drop or transfer once wsl-kind work commits.

## Done
Remainder is groomed and slated as concrete paces; this checkpoint wraps.

### lode-rbsl-bole-verb-specs (₢BHAAE) [complete]

**[260604-1005] complete**

## Character
Greenfield spec authoring — additive, no live path touched. Judgment on the verb-topology (per-kind vs kind-general) and the augur mint; the rest follows the house DSL.

## Goal
Write the RBSL* operation subdocuments that strongly define the bole-kind Lode verbs in the house `//axhob_operation` form (steps via `{rbbc_*}`, `//axhoot_typed_output`, `//axhoc_completion`, linked terms resolved from RBS0), and establish the cluster as the template the forthcoming kinds reuse. Precedents read at groom: RBSDI, RBSDR, RBSAP, RBSAA.

## Subdocuments — one operation per file, RBSL<verb-letter>
- **RBSLE — ensconce** (bole capture). Model on RBSDI (inscribe): auth → resolve upstream → submit capture Cloud Build → wait → `//axhoot_typed_output` writes the capture-file / `:rbi_vouch` envelope → completion. This is the **canonical capture skeleton** later kinds copy.
- **RBSLD — divine** (enumerate, read-only). Model on RBSDR (roster). Written **kind-agnostic** — decodes kind from the touchmark prefix. NOT multiplied per kind.
- **RBSLA — augur** (inspect one Lode, read-only). The inspect verb minted here — `rbw-la`, divination register, confirmed first-letter-distinct across the full Lode verb set. Model on RBSAP (plumb): single-target provenance read that grows to decode `:rbi_vouch`. Kind-agnostic.
- **RBSLB — banish** (whole-Lode delete). Model on RBSAA (ark_abjure). Kind-agnostic.

## Cinched
- augur is the inspect verb; divine = enumerate/list. Both settled at groom.
- DO NOT touch or adapt RBSAE — it is a read-only reference only (Ark cluster ≠ Lode cluster). RBSAE retires at the bole cutover (delete-old-last); the terminal scrub finishes it.
- Shared concepts (`lode`, `touchmark`, the capture-file, `:rbi_vouch`, members[]) get quoins allocated ONCE in RBS0's mapping section and referenced from every subdoc — this overlaps the scaffold's RBS0 quoin work, so sequence or merge with the scaffold, never parallel on RBS0.
- The three lifecycle specs (RBSLD/RBSLA/RBSLB) are kind-general by construction: a forthcoming kind adds only its capture subdoc (RBSLF/C/U/I) plus its capture-verb quoin — it never re-specs divine/augur/banish.

## Done
RBSLE/RBSLD/RBSLA/RBSLB exist in the house DSL; their quoins resolve from RBS0; RBSAE untouched; the capture-skeleton-plus-kind-general-lifecycle topology is explicit enough that a kind vertical can copy RBSLE and add one quoin. Quoin cross-refs resolve clean.

**[260603-1115] rough**

## Character
Greenfield spec authoring — additive, no live path touched. Judgment on the verb-topology (per-kind vs kind-general) and the augur mint; the rest follows the house DSL.

## Goal
Write the RBSL* operation subdocuments that strongly define the bole-kind Lode verbs in the house `//axhob_operation` form (steps via `{rbbc_*}`, `//axhoot_typed_output`, `//axhoc_completion`, linked terms resolved from RBS0), and establish the cluster as the template the forthcoming kinds reuse. Precedents read at groom: RBSDI, RBSDR, RBSAP, RBSAA.

## Subdocuments — one operation per file, RBSL<verb-letter>
- **RBSLE — ensconce** (bole capture). Model on RBSDI (inscribe): auth → resolve upstream → submit capture Cloud Build → wait → `//axhoot_typed_output` writes the capture-file / `:rbi_vouch` envelope → completion. This is the **canonical capture skeleton** later kinds copy.
- **RBSLD — divine** (enumerate, read-only). Model on RBSDR (roster). Written **kind-agnostic** — decodes kind from the touchmark prefix. NOT multiplied per kind.
- **RBSLA — augur** (inspect one Lode, read-only). The inspect verb minted here — `rbw-la`, divination register, confirmed first-letter-distinct across the full Lode verb set. Model on RBSAP (plumb): single-target provenance read that grows to decode `:rbi_vouch`. Kind-agnostic.
- **RBSLB — banish** (whole-Lode delete). Model on RBSAA (ark_abjure). Kind-agnostic.

## Cinched
- augur is the inspect verb; divine = enumerate/list. Both settled at groom.
- DO NOT touch or adapt RBSAE — it is a read-only reference only (Ark cluster ≠ Lode cluster). RBSAE retires at the bole cutover (delete-old-last); the terminal scrub finishes it.
- Shared concepts (`lode`, `touchmark`, the capture-file, `:rbi_vouch`, members[]) get quoins allocated ONCE in RBS0's mapping section and referenced from every subdoc — this overlaps the scaffold's RBS0 quoin work, so sequence or merge with the scaffold, never parallel on RBS0.
- The three lifecycle specs (RBSLD/RBSLA/RBSLB) are kind-general by construction: a forthcoming kind adds only its capture subdoc (RBSLF/C/U/I) plus its capture-verb quoin — it never re-specs divine/augur/banish.

## Done
RBSLE/RBSLD/RBSLA/RBSLB exist in the house DSL; their quoins resolve from RBS0; RBSAE untouched; the capture-skeleton-plus-kind-general-lifecycle topology is explicit enough that a kind vertical can copy RBSLE and add one quoin. Quoin cross-refs resolve clean.

### lode-scaffold-kind-letters (₢BHAAG) [complete]

**[260605-1840] complete**

## Character
Mechanical — a one-file constant promotion. Lands first; every vertical mints stamps off these.

## Goal
Promote the reserved Lode kind-letter set in rbgc_Constants.sh from comment to actual constants, mirroring the existing RBGC_LODE_KIND_BOLE. Cover tool, reliquary, wsl, and podvm — podvm carries two letters (its two quay families), per the paddock Lode registry layout.

## Locked
- Scope is the kind-letter constants only. Capture colophons (rbw-lF/lC/lU/lI) and divine-legend rows are NOT here — they ride each vertical (a colophon points at a verb that does not yet exist; a legend row for an unimplemented kind would lie).

## Done
The full kind-letter constant set exists in rbgc_Constants.sh; shellcheck green.

**[260605-1117] rough**

## Character
Mechanical — a one-file constant promotion. Lands first; every vertical mints stamps off these.

## Goal
Promote the reserved Lode kind-letter set in rbgc_Constants.sh from comment to actual constants, mirroring the existing RBGC_LODE_KIND_BOLE. Cover tool, reliquary, wsl, and podvm — podvm carries two letters (its two quay families), per the paddock Lode registry layout.

## Locked
- Scope is the kind-letter constants only. Capture colophons (rbw-lF/lC/lU/lI) and divine-legend rows are NOT here — they ride each vertical (a colophon points at a verb that does not yet exist; a legend row for an unimplemented kind would lie).

## Done
The full kind-letter constant set exists in rbgc_Constants.sh; shellcheck green.

### lode-bole-enshrine-cutover (₢BHAAH) [complete]

**[260608-1056] complete**

## Character
Serial cutover, run early — it hardens the bole pilot and proves the rebuild-and-repoint technique before the other verticals copy it. Live-path, verify-gated.

## Goal
Retire the enshrine base-mirror path and make the landed ensconce (bole) capture the real source: repoint the conjure base ANCHOR to resolve from the touchmark the bole capture emits as a single-form chaining fact.

## Locked
- Follow the paddock Cutover technique (rebuild-and-repoint, delete-old-last — no dual-read bridge).
- The conjure ANCHOR consumer is already namespace-agnostic, so a rbi_ld locator flows through unchanged — the work is the ANCHOR-population SOURCE, not the consumer. The bole locator needs the touchmark read from the single-form chaining fact the bole capture emits (the old enshrine anchor could be derived from origin+digest; the Lode one cannot).
- Onboarding rides this done: flip the enshrine handbook track + RBYC_ENSHRINE + the matching rbtdro_onboarding fixture. The handbook render only asserts exit-0, so stale prose passes silently — this done is the only guard.
- Verify gate: one airgap conjure build green (airgap is the strict anchor consumer; a tether build's pass-through would mask a broken anchor).
- Discover enshrine's secondary consumers by grepping functional RBGC_GAR_CATEGORY_ENSHRINES / enshrine uses, not the literal rbi_es (comments and doc-examples are noise).

## Done
Enshrine path retired (writer + rbw-dE + secondary consumers), base ANCHOR resolves from the bole capture's single-form touchmark chaining fact, one airgap conjure green, rbi_es banished last.

**[260606-1153] rough**

## Character
Serial cutover, run early — it hardens the bole pilot and proves the rebuild-and-repoint technique before the other verticals copy it. Live-path, verify-gated.

## Goal
Retire the enshrine base-mirror path and make the landed ensconce (bole) capture the real source: repoint the conjure base ANCHOR to resolve from the touchmark the bole capture emits as a single-form chaining fact.

## Locked
- Follow the paddock Cutover technique (rebuild-and-repoint, delete-old-last — no dual-read bridge).
- The conjure ANCHOR consumer is already namespace-agnostic, so a rbi_ld locator flows through unchanged — the work is the ANCHOR-population SOURCE, not the consumer. The bole locator needs the touchmark read from the single-form chaining fact the bole capture emits (the old enshrine anchor could be derived from origin+digest; the Lode one cannot).
- Onboarding rides this done: flip the enshrine handbook track + RBYC_ENSHRINE + the matching rbtdro_onboarding fixture. The handbook render only asserts exit-0, so stale prose passes silently — this done is the only guard.
- Verify gate: one airgap conjure build green (airgap is the strict anchor consumer; a tether build's pass-through would mask a broken anchor).
- Discover enshrine's secondary consumers by grepping functional RBGC_GAR_CATEGORY_ENSHRINES / enshrine uses, not the literal rbi_es (comments and doc-examples are noise).

## Done
Enshrine path retired (writer + rbw-dE + secondary consumers), base ANCHOR resolves from the bole capture's single-form touchmark chaining fact, one airgap conjure green, rbi_es banished last.

**[260605-1117] rough**

## Character
Serial cutover, run early — it hardens the bole pilot and proves the rebuild-and-repoint technique before the other verticals copy it. Live-path, verify-gated.

## Goal
Retire the enshrine base-mirror path and make the landed ensconce (bole) capture the real source: repoint the conjure base ANCHOR to resolve from the bole capture-file's touchmark locator.

## Locked
- Follow the paddock Cutover technique (rebuild-and-repoint, delete-old-last — no dual-read bridge).
- The conjure ANCHOR consumer is already namespace-agnostic, so a rbi_ld locator flows through unchanged — the work is the ANCHOR-population SOURCE, not the consumer. The bole locator needs the touchmark read from the capture-file (the old enshrine anchor could be derived from origin+digest; the Lode one cannot).
- Onboarding rides this done: flip the enshrine handbook track + RBYC_ENSHRINE + the matching rbtdro_onboarding fixture. The handbook render only asserts exit-0, so stale prose passes silently — this done is the only guard.
- Verify gate: one airgap conjure build green (airgap is the strict anchor consumer; a tether build's pass-through would mask a broken anchor).
- Discover enshrine's secondary consumers by grepping functional RBGC_GAR_CATEGORY_ENSHRINES / enshrine uses, not the literal rbi_es (comments and doc-examples are noise).

## Done
Enshrine path retired (writer + rbw-dE + secondary consumers), base ANCHOR resolves from the bole capture-file, one airgap conjure green, rbi_es banished last.

### lode-reliquary-capture (₢BHAAI) [complete]

**[260608-1215] complete**

## Character
Greenfield vertical, parallel-chat — one thin body on the landed spine, absorbing logic that already exists.

## Goal
Land the reliquary (conclave) capture kind: the build-tool cohort mirrored into one rbi_ld package as N member tags plus one rbi_vouch envelope. Absorb today's inscribe pull machinery (rbfli_Inscribe + rbgji01) — retargeted from the rbi_rq/<date>/<tool> sibling-package layout to rbi_ld/<stamp>:rbi_<tool> member tags in a single package. The single-tool kind that once shared this pace is dropped as non-load-bearing — see paddock "Why five kinds, not four".

## Locked
- reliquary/conclave is the sole kind this pace lands; tool/fetter is gone.
- Acquisition rides the Google-hosted cloud-builders/docker builder — no reliquary bootstrap. The rbi_vouch envelope, not the acquisition tool, is the unifier, so a docker-pull path alongside bole's skopeo is fine.
- Ride rblds_Spine as a thin body on the reserved rbldr_ letter; emit the touchmark + reliquary kind-brand as the single-form chaining facts (no multi-form capture-file). Reuse rbgjl02 (kind-agnostic vouch push) as-is; author one new cohort-capture cloud step. Author the rbw-lC colophon + its zipper/theurge consts with the body.
- Members are the locked clean scheme: :rbi_<tool> per member + :rbi_vouch (no digest/fingerprint layer).
- divine/banish are kind-general and exist. divine's enumeration column reads bole's digest tag, so a reliquary shows "(no fingerprint)" — decide kind-aware display vs accept at mount.
- Write RBSLC on the RBSLE skeleton + the conclave verb quoin. Do NOT retire inscribe (separate reliquary cutover). File-disjoint from the other verticals.

## Done
conclave captures the build-tool cohort into one rbi_ld Lode (N member tags + rbi_vouch provenance); a theurge service fixture proves capture -> divine (enumerate + inspect members) -> banish -> absent live against GAR; RBSLC written; inscribe still live.

**[260608-1129] rough**

## Character
Greenfield vertical, parallel-chat — one thin body on the landed spine, absorbing logic that already exists.

## Goal
Land the reliquary (conclave) capture kind: the build-tool cohort mirrored into one rbi_ld package as N member tags plus one rbi_vouch envelope. Absorb today's inscribe pull machinery (rbfli_Inscribe + rbgji01) — retargeted from the rbi_rq/<date>/<tool> sibling-package layout to rbi_ld/<stamp>:rbi_<tool> member tags in a single package. The single-tool kind that once shared this pace is dropped as non-load-bearing — see paddock "Why five kinds, not four".

## Locked
- reliquary/conclave is the sole kind this pace lands; tool/fetter is gone.
- Acquisition rides the Google-hosted cloud-builders/docker builder — no reliquary bootstrap. The rbi_vouch envelope, not the acquisition tool, is the unifier, so a docker-pull path alongside bole's skopeo is fine.
- Ride rblds_Spine as a thin body on the reserved rbldr_ letter; emit the touchmark + reliquary kind-brand as the single-form chaining facts (no multi-form capture-file). Reuse rbgjl02 (kind-agnostic vouch push) as-is; author one new cohort-capture cloud step. Author the rbw-lC colophon + its zipper/theurge consts with the body.
- Members are the locked clean scheme: :rbi_<tool> per member + :rbi_vouch (no digest/fingerprint layer).
- divine/banish are kind-general and exist. divine's enumeration column reads bole's digest tag, so a reliquary shows "(no fingerprint)" — decide kind-aware display vs accept at mount.
- Write RBSLC on the RBSLE skeleton + the conclave verb quoin. Do NOT retire inscribe (separate reliquary cutover). File-disjoint from the other verticals.

## Done
conclave captures the build-tool cohort into one rbi_ld Lode (N member tags + rbi_vouch provenance); a theurge service fixture proves capture -> divine (enumerate + inspect members) -> banish -> absent live against GAR; RBSLC written; inscribe still live.

**[260608-1129] rough**

## Character
Greenfield vertical, parallel-chat — one thin body on the landed spine, absorbing logic that already exists.

## Goal
Land the reliquary (conclave) capture kind: the build-tool cohort mirrored into one rbi_ld package as N member tags plus one rbi_vouch envelope. Absorb today's inscribe pull machinery (rbfli_Inscribe + rbgji01) — retargeted from the rbi_rq/<date>/<tool> sibling-package layout to rbi_ld/<stamp>:rbi_<tool> member tags in a single package. The single-tool kind that once shared this pace is dropped as non-load-bearing — see paddock "Why five kinds, not four".

## Locked
- reliquary/conclave is the sole kind this pace lands; tool/fetter is gone.
- Acquisition rides the Google-hosted cloud-builders/docker builder — no reliquary bootstrap. The rbi_vouch envelope, not the acquisition tool, is the unifier, so a docker-pull path alongside bole's skopeo is fine.
- Ride rblds_Spine as a thin body on the reserved rbldr_ letter; emit the touchmark + reliquary kind-brand as the single-form chaining facts (no multi-form capture-file). Reuse rbgjl02 (kind-agnostic vouch push) as-is; author one new cohort-capture cloud step. Author the rbw-lC colophon + its zipper/theurge consts with the body.
- Members are the locked clean scheme: :rbi_<tool> per member + :rbi_vouch (no digest/fingerprint layer).
- divine/banish are kind-general and exist. divine's enumeration column reads bole's digest tag, so a reliquary shows "(no fingerprint)" — decide kind-aware display vs accept at mount.
- Write RBSLC on the RBSLE skeleton + the conclave verb quoin. Do NOT retire inscribe (separate reliquary cutover). File-disjoint from the other verticals.

## Done
conclave captures the build-tool cohort into one rbi_ld Lode (N member tags + rbi_vouch provenance); a theurge service fixture proves capture -> divine (enumerate + inspect members) -> banish -> absent live against GAR; RBSLC written; inscribe still live.

**[260606-1153] rough**

## Character
Greenfield vertical, parallel-chat — thin bodies on the landed spine. Mostly absorbing logic that already exists.

## Goal
Land the tool (fetter) and reliquary (conclave) capture kinds. The docker-pull machinery they need already exists as today's inscribe (rbfli_Inscribe + its inscribe-mirror cloud step) — absorb it onto the spine. tool is the single-image case; reliquary is the date-cohort (N-member) case that proves the multi-member package layout.

## Locked
- Ride the landed spine (rblds_Spine) as thin body files on the reserved rbldt_/rbldr_ letters; reuse the rbi_vouch envelope and emit each capture's touchmark + kind-brand as the single-form chaining fact the fact-chaining pace establishes (no multi-form capture-file). Author the rbw-lF/lC colophons + their zipper/theurge consts with the bodies.
- Write the RBSLF/RBSLC capture subdocs on the existing RBSLE skeleton + each capture-verb quoin. divine/augur/banish are kind-general and already exist.
- Do NOT retire inscribe here — that is the separate reliquary cutover. This vertical leaves the inscribe fork live.
- File-disjoint from the other verticals.

## Done
fetter and conclave capture into rbi_ld with provenance; a theurge fixture proves capture -> divine -> banish live against GAR; RBSLF/RBSLC written; inscribe still live.

**[260605-1118] rough**

## Character
Greenfield vertical, parallel-chat — thin bodies on the landed spine. Mostly absorbing logic that already exists.

## Goal
Land the tool (fetter) and reliquary (conclave) capture kinds. The docker-pull machinery they need already exists as today's inscribe (rbfli_Inscribe + its inscribe-mirror cloud step) — absorb it onto the spine. tool is the single-image case; reliquary is the date-cohort (N-member) case that proves the multi-member package layout.

## Locked
- Ride the landed spine (rblds_Spine) as thin body files on the reserved rbldt_/rbldr_ letters; reuse the rbi_vouch envelope + capture-file pattern. Author the rbw-lF/lC colophons + their zipper/theurge consts with the bodies.
- Write the RBSLF/RBSLC capture subdocs on the existing RBSLE skeleton + each capture-verb quoin. divine/augur/banish are kind-general and already exist.
- Do NOT retire inscribe here — that is the separate reliquary cutover. This vertical leaves the inscribe fork live.
- File-disjoint from the other verticals.

## Done
fetter and conclave capture into rbi_ld with provenance; a theurge fixture proves capture -> divine -> banish live against GAR; RBSLF/RBSLC written; inscribe still live.

### lode-wsl-capture (₢BHAAJ) [complete]

**[260608-1306] complete**

## Character
Greenfield vertical — the structural outlier. Needs a new cloud capture step, not the skopeo registry path.

## Goal
Land the wsl (underpin) capture kind: a project-controlled WSL rootfs captured into a Lode. wsl fetches a vendor-published rootfs tarball over HTTPS and verifies it against the published checksum (Canonical cloud-images SHA-256), then wraps the opaque tarball as a Lode member.

## Cinched
- This is genuinely verified-against-published (the published checksum is real here). Acquisition runs cloud-side — no workstation export (paddock acquisition premise).
- Needs a NEW cloud capture step (curl + checksum-verify + opaque-blob wrap); it does not reuse the registry-pull step. Ride the spine envelope on the rbldw_ letter.
- Consumption (wsl --import) is deferred this heat — stop at the registry. File-disjoint. Write RBSLU.

## Done
underpin captures a checksum-verified rootfs into rbi_ld with provenance; a theurge fixture proves capture -> divine -> banish live (no host consumption).

**[260606-1012] rough**

## Character
Greenfield vertical — the structural outlier. Needs a new cloud capture step, not the skopeo registry path.

## Goal
Land the wsl (underpin) capture kind: a project-controlled WSL rootfs captured into a Lode. wsl fetches a vendor-published rootfs tarball over HTTPS and verifies it against the published checksum (Canonical cloud-images SHA-256), then wraps the opaque tarball as a Lode member.

## Cinched
- This is genuinely verified-against-published (the published checksum is real here). Acquisition runs cloud-side — no workstation export (paddock acquisition premise).
- Needs a NEW cloud capture step (curl + checksum-verify + opaque-blob wrap); it does not reuse the registry-pull step. Ride the spine envelope on the rbldw_ letter.
- Consumption (wsl --import) is deferred this heat — stop at the registry. File-disjoint. Write RBSLU.

## Done
underpin captures a checksum-verified rootfs into rbi_ld with provenance; a theurge fixture proves capture -> divine -> banish live (no host consumption).

**[260605-1119] rough**

## Character
Greenfield vertical — the structural outlier. Needs a new cloud capture step, not the skopeo registry path.

## Goal
Land the wsl (underpin) capture kind: a project-controlled WSL rootfs captured into a Lode. wsl fetches a vendor-published rootfs tarball over HTTPS and verifies it against the published checksum (Canonical cloud-images SHA-256), then wraps the opaque tarball as a Lode member.

## Locked
- This is genuinely verified-against-published (the published checksum is real here). Acquisition runs cloud-side — no workstation export (paddock acquisition premise).
- Needs a NEW cloud capture step (curl + checksum-verify + opaque-blob wrap); it does not reuse the registry-pull step. Ride the spine envelope on the rbldw_ letter.
- Consumption (wsl --import) is deferred this heat — stop at the registry. File-disjoint. Write RBSLU.

## Done
underpin captures a checksum-verified rootfs into rbi_ld with provenance; a theurge fixture proves capture -> divine -> banish live (no host consumption).

### lode-podvm-cerebro-experiment (₢BHAAK) [complete]

**[260608-2024] complete**

## Character
Review + decision pace, cold mount on macOS. The cerebro experiment is DONE and
recorded; this is judgment about ramifications and finalizing spec landing — not
re-running the experiment. The core review needs NO cloud credentials (reading the
memo, editing specs).

## Goal
Review the podvm cerebro-experiment conclusion and settle its ramifications for the
project's capture-tool posture and spec surface, then land whatever spec changes the
review warrants. Read the memo first — it is the whole record.

## Done already — do NOT redo
- Experiment executed on cerebro: both quay families (machine-os = vn,
  machine-os-wsl = vw) characterized at podman 5.6; captured into rbi_ld with
  rbld-vouch-1 provenance; proven controllable via host-side divine + banish;
  anti-hollow-mirror guard green (GAR registry-v2 blob HEAD, REST only).
- Findings + cloud-layer decision recorded: the memo (full record) + a "2026-06
  cerebro characterization" section appended to RBSPV.
- cerebro torn down to as-found: throwaway crane/oras/skopeo removed, both evidence
  Lodes banished (GAR holds only the 2 pre-existing rust-slim boles). The Director
  RBRA was LEFT in place — it pre-existed the pace (standing cerebro infra), not
  pace-introduced.

## Remaining — decide on macOS
- **Capture-tool posture (the headline).** Decision on record is crane-primary /
  oras equal-fidelity-fallback / skopeo-out. Settle concretely: does the kit ADD
  oras as a sanctioned capture tool, or stay crane-only with oras documented but
  unused? Does skopeo get EJECTED from any spec/onboarding text that currently names
  or assumes it for capture? (skopeo is already out for podvm — but its recorded
  failure mode was corrected: it fatals LOUD on empty-config artifacts, it does not
  silently skip; any text repeating the silent-skip rationale should be fixed.)
- **RBSPV promotion.** Decide whether to promote RBSPV out of FUTURE/ now, or leave
  it until the follow-up cloud pace.
- **RBSL subdoc / RBS0 landing.** The memo + RBSPV carry the characterization;
  decide whether any RBSL Lode subdoc or RBS0 quoin needs the podvm shape recorded
  now, or whether that rides the follow-up cloud pace.
- **Slate the follow-up cloud pace?** Production capture (a new rbgjl05 immure step +
  rbldv_ opaque-blob × multi-member body + a podvm-lifecycle theurge fixture, riding
  the spine) is now unblocked. Decide whether to slate it as a sibling pace — full
  build sheet is in the memo §7.

## Operator-owned — confirm, do not perform
- Credential hygiene: payor reauth + governor remantle, to invalidate the fresh
  Director key minted off-station on cerebro this session (a Governor mantle +
  Director invest were run on cerebro to repair a collapsed credential chain).

## Sources
- `Memos/memo-20260608-lode-podvm-cerebro-experiment.md` (commit b47cbdcd21ea) —
  full record: tool versions, quay index digests, leaf shapes, tool verdict,
  process spooks, and the follow-up cloud build sheet (§7).
- `Tools/rbk/vov_veiled/FUTURE/RBSPV-PodmanVmSupplyChain.adoc` — "2026-06 cerebro
  characterization" section.
- Paddock ₣BH: Vocabulary (immure verb, podvm-native/podvm-wsl kinds), Kinds table,
  the two trust grades.

**[260608-2158] rough**

## Character
Review + decision pace, cold mount on macOS. The cerebro experiment is DONE and
recorded; this is judgment about ramifications and finalizing spec landing — not
re-running the experiment. The core review needs NO cloud credentials (reading the
memo, editing specs).

## Goal
Review the podvm cerebro-experiment conclusion and settle its ramifications for the
project's capture-tool posture and spec surface, then land whatever spec changes the
review warrants. Read the memo first — it is the whole record.

## Done already — do NOT redo
- Experiment executed on cerebro: both quay families (machine-os = vn,
  machine-os-wsl = vw) characterized at podman 5.6; captured into rbi_ld with
  rbld-vouch-1 provenance; proven controllable via host-side divine + banish;
  anti-hollow-mirror guard green (GAR registry-v2 blob HEAD, REST only).
- Findings + cloud-layer decision recorded: the memo (full record) + a "2026-06
  cerebro characterization" section appended to RBSPV.
- cerebro torn down to as-found: throwaway crane/oras/skopeo removed, both evidence
  Lodes banished (GAR holds only the 2 pre-existing rust-slim boles). The Director
  RBRA was LEFT in place — it pre-existed the pace (standing cerebro infra), not
  pace-introduced.

## Remaining — decide on macOS
- **Capture-tool posture (the headline).** Decision on record is crane-primary /
  oras equal-fidelity-fallback / skopeo-out. Settle concretely: does the kit ADD
  oras as a sanctioned capture tool, or stay crane-only with oras documented but
  unused? Does skopeo get EJECTED from any spec/onboarding text that currently names
  or assumes it for capture? (skopeo is already out for podvm — but its recorded
  failure mode was corrected: it fatals LOUD on empty-config artifacts, it does not
  silently skip; any text repeating the silent-skip rationale should be fixed.)
- **RBSPV promotion.** Decide whether to promote RBSPV out of FUTURE/ now, or leave
  it until the follow-up cloud pace.
- **RBSL subdoc / RBS0 landing.** The memo + RBSPV carry the characterization;
  decide whether any RBSL Lode subdoc or RBS0 quoin needs the podvm shape recorded
  now, or whether that rides the follow-up cloud pace.
- **Slate the follow-up cloud pace?** Production capture (a new rbgjl05 immure step +
  rbldv_ opaque-blob × multi-member body + a podvm-lifecycle theurge fixture, riding
  the spine) is now unblocked. Decide whether to slate it as a sibling pace — full
  build sheet is in the memo §7.

## Operator-owned — confirm, do not perform
- Credential hygiene: payor reauth + governor remantle, to invalidate the fresh
  Director key minted off-station on cerebro this session (a Governor mantle +
  Director invest were run on cerebro to repair a collapsed credential chain).

## Sources
- `Memos/memo-20260608-lode-podvm-cerebro-experiment.md` (commit b47cbdcd21ea) —
  full record: tool versions, quay index digests, leaf shapes, tool verdict,
  process spooks, and the follow-up cloud build sheet (§7).
- `Tools/rbk/vov_veiled/FUTURE/RBSPV-PodmanVmSupplyChain.adoc` — "2026-06 cerebro
  characterization" section.
- Paddock ₣BH: Vocabulary (immure verb, podvm-native/podvm-wsl kinds), Kinds table,
  the two trust grades.

**[260608-1349] rough**

## Character
Hands-on registry-tool experiment on the cerebro Linux host requiring judgment, then spec authoring and a defer-or-implement decision. The deliverable is evidence + a recorded decision, NOT working cloud code. Mount cold: this docket carries the whole 260608 design chat because that chat is gone.

## Goal
Establish — by experiment on cerebro, not by inheriting the GHCR prototype — how Recipe Bottle brings podman-VM machine-os images under its control in GAR. Temp-install crane/skopeo/oras on cerebro, capture BOTH quay families into the depot `rbi_ld` GAR with provenance, characterize the live artifact shape and which tool handles it cleanly, then record findings into the specs and write the cloud-layer implementation decision. The production cloud capture (a future `rbgjl05` step + `rbldv_` body + theurge fixture, riding the spine like underpin/conclave) is a SEPARATE follow-up pace this experiment unblocks — do NOT build it here.

## Cinched
- **Venue: cerebro, by express exception.** The workstation is `bash`/`curl`/`openssl`/`jq` + coreutils ONLY (README.md opening — the tiny-dependency surface is a stated product value); it cannot and must not host image tools. cerebro (`ssh cerebro`, intel-Linux, Claude Code installed) gets express, exceptional permission to download image-manipulation apps (crane, skopeo, oras). They are throwaway — removal is a Done-when gate.
- **skopeo is ruled OUT as the podvm capture tool** — wrong failure profile for a vendor-churned format: it hard-rejects benign OCI/Docker media-type mixing (go-containerregistry issue 1608) and SILENTLY skips foreign/non-distributable layers by default (skopeo issue 545) → hollow-mirror risk under the recorded grade. The real contest is **crane vs oras**; prior lean is **crane** (Google's own go-containerregistry tool; already proven on our GCB/GAR in the retired RBSOB OCI-layout bridge; `crane blob` is get-by-digest-or-error, the loud failure the recorded grade wants; robust across podman's image→OCI-artifact migration). The experiment confirms or overturns this — it does not assume it.
- **Cloud-side acquisition premise STANDS.** cerebro is the lab bench for THIS experiment only; production capture remains cloud-side (paddock "acquisition runs cloud-side, never the workstation" is untouched). cerebro is never the production capture path. If the experiment tempts a cerebro-hosted production capture, that is a premise-level reversal — escalate, don't drift.
- **Both quay families**, captured + characterized: `quay.io/podman/machine-os` (native, kind-letter `vn`) and `quay.io/podman/machine-os-wsl` (wsl, kind-letter `vw`). They diverge structurally; one does not characterize the other.
- **Auth: copy the Director RBRA to cerebro** for the GAR push (simplest; the identity host-side `divine`/`banish` already use against `rbi_ld`). The copied RBRA is sensitive and temporary — removal is part of teardown. Operator owns the post-pace credential hygiene (payor reauthorization + governor remantle) as a consequence of exposing the Director RBRA on cerebro — NOT a pace step.
- **Curated platform subset per family, never a full mirror** (paddock podvm-selective premise — quay's index is 5-15 GB). **recorded-at-acquisition** trust grade (quay publishes no durable checksum; rotates within days).
- **Consumption is NOT attempted** — no `podman machine init`/boot of the captured image. Capture + control-in-GAR only.
- **Captured Lodes are banished at teardown** (clean experiment; proves the full capture → divine → banish lifecycle).

## Method (experiment recipe — adapt at mount, not a rigid script)
- On cerebro: install crane + skopeo + oras as throwaway static binaries into a scratch dir (cleanest teardown — crane/oras ship static Go binaries); copy the Director RBRA; confirm egress to `quay.io` (pull) and GAR (push).
- For EACH family at a chosen podman version: inspect the upstream manifest with each of the three tools. Record the durable characterization — is it an image-manifest or an OCI artifact-manifest (`artifactType`)? what media types do the disk blobs carry? any foreign/non-distributable markers? which tool reads/extracts it cleanly, which errors? This characterization is the core finding the cloud decision rests on.
- Capture a curated platform subset of each family into `rbi_ld/<vn|vw stamp>` in the depot GAR, with a provenance envelope mirroring the wsl/reliquary `:rbi_vouch` shape (members[] is the cardinality axis).
- Prove control from the WORKSTATION: the existing `rbw-ld` (divine) enumerates the captured Lodes and `rbw-lB` (banish) deletes them — these are curl/REST host ops, tool-agnostic about how the Lode was made, so a cerebro-captured Lode is workstation-controllable today.
- Verify the anti-hollow-mirror guard: confirm each disk blob is actually present/sized in GAR via REST read, not merely a manifest. (When this goes cloud-side later, this guard must be GAR-REST/curl host-side, never a host tool.)

## Done when
- Both families captured into the depot `rbi_ld` GAR from cerebro with provenance, and shown controllable via the existing host-side `divine` + `banish`.
- The per-family characterization (manifest shape, disk-blob media types, foreign markers, crane-vs-oras-vs-skopeo verdict) is recorded into RBSPV (`Tools/rbk/vov_veiled/FUTURE/RBSPV-PodmanVmSupplyChain.adoc` — consider promoting out of FUTURE/ as podvm activates) and the relevant RBS* Lode subdocuments; a memo is written only if findings warrant durable capture beyond the spec edits.
- The cloud-layer implementation decision is written down plainly: which tool, what the future `rbgjl05` must do, what `rbldv_` body + theurge fixture the follow-up pace builds — enough that a cold mount executes it without re-deriving.
- Teardown verified: image tools removed from cerebro (`which crane skopeo oras` → nothing), copied Director RBRA removed, captured experimental Lodes banished. GAR and cerebro left as found.

## Sources (re-derive nothing)
- Prototype: `Tools/rbk/vov_veiled/FUTURE/rbv_PodmanVM.sh` + RBSPV — crane manifest/blob navigation, two-family split, FROM-scratch repackage, the ignite-VM that cloud-side dissolves.
- Cloud shape to copy in the FOLLOW-UP pace (not here): `rbldw_Underpin.sh` (opaque-blob body), `rbldr_Reliquary.sh` (multi-member cohort body — podvm is opaque-blob × multi-member, a blend of these two), `rblds_Spine.sh` (capture-spine contract, RBSCJ), cloud steps `rbgjl02`/`03`/`04`, shared snippets `rbgjs/`.
- Durable web anchors: skopeo strict media-type (github.com/google/go-containerregistry issues/1608); skopeo silent foreign-layer skip (github.com/containers/skopeo issues/545); podman→OCI-artifact direction (podman 5.0 `--artifact`, 5.4 preview `podman artifact`, 5.5 `artifact extract`; containers/podman issues/24785 disk-artifact manifest type); oras artifact-native (oras.land).
- Registration surface the FOLLOW-UP cloud pace touches (kind-letters already reserved `RBGC_LODE_KIND_PODVM_NATIVE="vn"` / `_PODVM_WSL="vw"`): `rbgc_Constants.sh` brands/tags, `rbldl_Lifecycle.sh` divine legend, and the theurge fixture registry across `rbtdrc_crucible.rs` + `rbtdrm_manifest.rs` — including the single-fixture `RBTDRC_FIXTURES` lookup registry that the 12f48cbab spook proved is easy to miss.
- Workstation restriction: README.md opening. cerebro access: CLAUDE.md "Test Environments". Paddock premises: cloud-side acquisition, podvm-selective retention, recorded grade, Palisade honesty.

**[260608-1348] rough**

## Character
Greenfield vertical, the largest — crane-based opaque-blob capture, ported from the GHCR prototype to GAR.

## Goal
Land the podvm (immure) capture machinery: port the podman-machine-image capture (crane manifest/blob navigation, FROM-scratch repackage) from the prototype (rbv_PodmanVM, RBSPV) onto the spine and into rbi_ld. This pace builds the single-family capture path; the two-family fan-out is the next pace.

## Cinched
- crane (not skopeo/docker) for opaque non-layered blobs; recorded-at-acquisition trust grade (quay publishes no durable checksum). Cloud-side acquisition. Ride the spine on the rbldv_ letter.
- Multi-member package holding a curated platform subset, never a full mirror (paddock podvm-selective premise). File-disjoint. Write RBSLI.

## Done
immure captures one quay family's curated platform set into rbi_ld with provenance; a theurge fixture proves capture -> divine -> banish live.

**[260606-1012] rough**

## Character
Greenfield vertical, the largest — crane-based opaque-blob capture, ported from the GHCR prototype to GAR.

## Goal
Land the podvm (immure) capture machinery: port the podman-machine-image capture (crane manifest/blob navigation, FROM-scratch repackage) from the prototype (rbv_PodmanVM, RBSPV) onto the spine and into rbi_ld. This pace builds the single-family capture path; the two-family fan-out is the next pace.

## Cinched
- crane (not skopeo/docker) for opaque non-layered blobs; recorded-at-acquisition trust grade (quay publishes no durable checksum). Cloud-side acquisition. Ride the spine on the rbldv_ letter.
- Multi-member package holding a curated platform subset, never a full mirror (paddock podvm-selective premise). File-disjoint. Write RBSLI.

## Done
immure captures one quay family's curated platform set into rbi_ld with provenance; a theurge fixture proves capture -> divine -> banish live.

**[260605-1119] rough**

## Character
Greenfield vertical, the largest — crane-based opaque-blob capture, ported from the GHCR prototype to GAR.

## Goal
Land the podvm (immure) capture machinery: port the podman-machine-image capture (crane manifest/blob navigation, FROM-scratch repackage) from the prototype (rbv_PodmanVM, RBSPV) onto the spine and into rbi_ld. This pace builds the single-family capture path; the two-family fan-out is the next pace.

## Locked
- crane (not skopeo/docker) for opaque non-layered blobs; recorded-at-acquisition trust grade (quay publishes no durable checksum). Cloud-side acquisition. Ride the spine on the rbldv_ letter.
- Multi-member package holding a curated platform subset, never a full mirror (paddock podvm-selective premise). File-disjoint. Write RBSLI.

## Done
immure captures one quay family's curated platform set into rbi_ld with provenance; a theurge fixture proves capture -> divine -> banish live.

### gar-delete-host-cloud-boundary (₢BHAAZ) [complete]

**[260609-2013] complete**

## Character
Implementation with the architecture cinched — no longer a decision pace.
Intricate (touches the delete path on both fetched and made sides) but settled: don't re-open the model.
Tier: architectural — the driver works this directly; the in-pool membrane and grants need judgment.

## Goal
Move GAR-image deletes off host-issued REST onto cloud-side delete-builds — the workstation dispatches a build and blocks until it is terminal, conjure-shaped — for both `banish` (Lode) and `abjure` (hallmark).
The cloud step closes the trust-200 LRO gap by construction: it polls `packages.delete` to terminal in-pool and verifies absence, so the build's outcome IS the delete outcome.

## Cinched
- Scope is GAR package deletes only: banish + abjure.
  Control-plane deletes (SA / project / depot / lien / bucket) are untouched — they stay host-side REST.
  The recorded canon is the tool-vs-control-plane line itself, not an all-deletes-go-cloud trajectory.
  The path-polymorphic image backdoor (its own pace) is deliberately NOT on this model — it stays host-side.
- The delete-build runs as Director, not Mason.
  Director already holds repoAdmin (delete), so there is no new GAR grant and no widening of the capture identity that executes untrusted upstream bytes; Mason stays writer-only.
- abjure converts here despite being made-side — "all GAR images" was scoped, and leaving it keeps the identical trust-200 bug live.
  This is a narrow, deliberate cross into made-side delete; it does NOT pull in the made-image package retrofit.
- abjure deletes N packages per hallmark (RBSAA enumerates the subtree): the step takes a package LIST and loops in-pool with a per-package poll — one build per abjure, never one build per package.
- Palisade membrane for the GAR cascade bug (characterized at commit 619882ee2; paddock atomic-delete correction premise):
  on a parent-index/child-manifest web, `packages.delete` auto-removes protected children, the cascade reaches an already-gone child, and the LRO terminates NOT_FOUND (code 5) even when the delete effectively completed — structural, reproduced on a settled package.
  The step absorbs exactly this signature: after a terminal LRO, verify package absence and treat verified-absent as success; any other failure dies loud.
  bole/wsl keep full-fidelity multi-arch capture, so index-web Lodes remain a live shape — the membrane is load-bearing.
  If the debris banish (below) shows a web can survive the cascade entirely, implement the paddock's named fallback inside the same step — per-version delete loop, then package delete — depth decided at mount from observed behavior.

## Done when
- A cloud delete step (GAR `packages.delete` + honest in-pool LRO poll + absence-verify, ambient Director auth) rides the capture spine, which gains a caller-supplied serviceAccount in place of its Mason constant.
- banish and abjure both dispatch that step and block on completion — no host-issued DELETE survives in either path.
- Director carries the build-capability grant it needs (workerPoolUser + act-as/builds on the TETHER pool); investiture updated.
- Posture recorded in RBSCB — including the cascade membrane and the accepted service-tier cost (every fixture banish/abjure becomes a build); the banish and abjure delete specs updated to the cloud-dispatch model.
- The standing 55-version multi-arch debris package is banished through the new path as the cascade test corpse — verification material, not cleanup duty (depot is disposable).
- Full live fixture verification DELEGATES to the next pace's single service-tier run; here, smoke only the debris banish plus one lode-lifecycle banish leg.

## Sources
rbldl_Lifecycle.sh `rbld_banish`; rbfld_Delete.sh `rbfl_abjure`; rblds_Spine.sh (serviceAccount in the build-envelope compose; rides zrbfc_wait_build_completion); rbge_Rest.sh `rbge_lro_ok` (honest LRO-poll reference to mirror in-pool); rbgg_Governor.sh (Mason/Director grants); RBSCB (posture home); commit 619882ee2 (cascade characterization).

**[260609-1616] rough**

## Character
Implementation with the architecture cinched — no longer a decision pace.
Intricate (touches the delete path on both fetched and made sides) but settled: don't re-open the model.
Tier: architectural — the driver works this directly; the in-pool membrane and grants need judgment.

## Goal
Move GAR-image deletes off host-issued REST onto cloud-side delete-builds — the workstation dispatches a build and blocks until it is terminal, conjure-shaped — for both `banish` (Lode) and `abjure` (hallmark).
The cloud step closes the trust-200 LRO gap by construction: it polls `packages.delete` to terminal in-pool and verifies absence, so the build's outcome IS the delete outcome.

## Cinched
- Scope is GAR package deletes only: banish + abjure.
  Control-plane deletes (SA / project / depot / lien / bucket) are untouched — they stay host-side REST.
  The recorded canon is the tool-vs-control-plane line itself, not an all-deletes-go-cloud trajectory.
  The path-polymorphic image backdoor (its own pace) is deliberately NOT on this model — it stays host-side.
- The delete-build runs as Director, not Mason.
  Director already holds repoAdmin (delete), so there is no new GAR grant and no widening of the capture identity that executes untrusted upstream bytes; Mason stays writer-only.
- abjure converts here despite being made-side — "all GAR images" was scoped, and leaving it keeps the identical trust-200 bug live.
  This is a narrow, deliberate cross into made-side delete; it does NOT pull in the made-image package retrofit.
- abjure deletes N packages per hallmark (RBSAA enumerates the subtree): the step takes a package LIST and loops in-pool with a per-package poll — one build per abjure, never one build per package.
- Palisade membrane for the GAR cascade bug (characterized at commit 619882ee2; paddock atomic-delete correction premise):
  on a parent-index/child-manifest web, `packages.delete` auto-removes protected children, the cascade reaches an already-gone child, and the LRO terminates NOT_FOUND (code 5) even when the delete effectively completed — structural, reproduced on a settled package.
  The step absorbs exactly this signature: after a terminal LRO, verify package absence and treat verified-absent as success; any other failure dies loud.
  bole/wsl keep full-fidelity multi-arch capture, so index-web Lodes remain a live shape — the membrane is load-bearing.
  If the debris banish (below) shows a web can survive the cascade entirely, implement the paddock's named fallback inside the same step — per-version delete loop, then package delete — depth decided at mount from observed behavior.

## Done when
- A cloud delete step (GAR `packages.delete` + honest in-pool LRO poll + absence-verify, ambient Director auth) rides the capture spine, which gains a caller-supplied serviceAccount in place of its Mason constant.
- banish and abjure both dispatch that step and block on completion — no host-issued DELETE survives in either path.
- Director carries the build-capability grant it needs (workerPoolUser + act-as/builds on the TETHER pool); investiture updated.
- Posture recorded in RBSCB — including the cascade membrane and the accepted service-tier cost (every fixture banish/abjure becomes a build); the banish and abjure delete specs updated to the cloud-dispatch model.
- The standing 55-version multi-arch debris package is banished through the new path as the cascade test corpse — verification material, not cleanup duty (depot is disposable).
- Full live fixture verification DELEGATES to the next pace's single service-tier run; here, smoke only the debris banish plus one lode-lifecycle banish leg.

## Sources
rbldl_Lifecycle.sh `rbld_banish`; rbfld_Delete.sh `rbfl_abjure`; rblds_Spine.sh (serviceAccount in the build-envelope compose; rides zrbfc_wait_build_completion); rbge_Rest.sh `rbge_lro_ok` (honest LRO-poll reference to mirror in-pool); rbgg_Governor.sh (Mason/Director grants); RBSCB (posture home); commit 619882ee2 (cascade characterization).

**[260609-1443] rough**

## Character
Implementation with the architecture cinched — no longer a decision pace.
Intricate (touches the delete path on both fetched and made sides) but settled: don't re-open the model.

## Goal
Move GAR-image deletes off host-issued REST onto cloud-side delete-builds — the workstation dispatches a build and blocks until it is terminal, conjure-shaped — for both `banish` (Lode) and `abjure` (hallmark).
The cloud step closes the trust-200 LRO gap by construction: it polls `packages.delete` to terminal in-pool, so the build's success or failure IS the delete outcome.

## Cinched
- Scope is GAR-image deletes only: banish + abjure.
  Control-plane deletes (SA / project / depot / lien / bucket) are untouched — they stay host-side REST.
  The recorded canon is the tool-vs-control-plane line itself, not an all-deletes-go-cloud trajectory.
- The delete-build runs as Director, not Mason.
  Director already holds repoAdmin (delete), so there is no new GAR grant and no widening of the capture identity that executes untrusted upstream bytes; Mason stays writer-only.
- abjure converts here despite being made-side — "all GAR images" was scoped, and leaving it keeps the identical trust-200 bug live.
  This is a narrow, deliberate cross into made-side delete; it does NOT pull in the made-image package retrofit (Ark/Hallmark stay deferred to the sibling heat).

## Done when
- A cloud delete step (GAR `packages.delete` + honest in-pool LRO poll, ambient Director auth) rides the capture spine, which gains a caller-supplied serviceAccount in place of its Mason constant.
- banish and abjure both dispatch that step and block on completion — no host-issued DELETE survives in either path.
- Director carries the build-capability grant it needs (workerPoolUser + act-as/builds on the TETHER pool); investiture updated.
- Posture recorded in RBSCB; the banish and abjure delete specs updated to the cloud-dispatch model.
- Both paths verified live against GAR — banish through lode-lifecycle, abjure through hallmark-lifecycle.

## Sources
rbldl_Lifecycle.sh `rbld_banish`; rbfld_Delete.sh `rbfl_abjure`; rblds_Spine.sh (serviceAccount in the build-envelope compose; rides zrbfc_wait_build_completion); rbge_Rest.sh `rbge_lro_ok` (honest LRO-poll reference to mirror in-pool); rbgg_Governor.sh (Mason/Director grants); RBSCB (posture home).

**[260609-1137] rough**

## Character
Architectural decision requiring fresh judgment — stop-the-presses, parked deliberately mid-eviction.
Broad blast radius (the whole delete family), not a mechanical change.

## Decision needed
Should GAR/GCP control-plane DELETEs stay host-issued REST (as they are system-wide today), or be proxied through cloud-side (pool) scripts for a clean host/cloud boundary?
Then make the chosen model consistent and record it in the specs.

## Why now (trigger, from the crane-embrace eviction)
- GAR `packages delete` completes-with-error (NOT_FOUND) on multi-arch parent/child manifest webs, and the host-side delete trusts the up-front 200 — it never polls the long-running operation — so a failed delete reports success.
  The made-side abjure (rbfld_Delete.sh) carries the identical trust-200 pattern.
- The operator expected deletes to be proxied; they are NOT.
  Host-direct REST DELETE is pervasive: banish, abjure, governor SA/project, payor depot/lien, buckets, GAR artifacts.
  The distinction that actually exists is tool-vs-control-plane — container-tool image work runs cloud-side (captures), pure control-plane REST runs host-side.

## Done when
The host-vs-cloud-proxy question is decided and recorded (RBSCB posture; the affected delete specs), and the trust-200 LRO honesty gap is closed for the delete family (banish + abjure) in whichever model wins.
If proxy is chosen, the migration path is scoped.
Likely graduates to its own heat — the blast radius exceeds Lode capture.

## Sources
rbldl_Lifecycle.sh `rbld_banish`; rbfld_Delete.sh (abjure, same trust-200); rbge_Rest.sh `rbge_lro_ok` (the LRO-poll machinery to reuse); GAR cleanup-policy doc (parent/child manifest deletion rule).

### crane-embrace-eviction (₢BHAAT) [complete]

**[260610-0557] complete**

## Character
Gate-verification + wrap only.
Implementation is landed and pushed; this is run-the-fixtures, debug-if-a-cloud-build-fails, wrap.
Do NOT re-open the eviction design or the cinched decisions.
Tier: sonnet-delegable — the debug recipe is below; escalate only on a novel failure shape.

## Goal
Capture on gcrane + curl + gpg (docker/buildx evicted); bole pins a reliquary gcrane; conclave/wsl ride floating gcrane.
Prove it via the three service-tier capture fixtures, live against GAR.
This single run is ALSO the live verification for the preceding delete-architecture pace — the fixtures' banish/abjure legs exercise the cloud-dispatch delete path; one run proves both paces.

## State at reslate
Verified live this heat: lode-lifecycle GREEN, hallmark-lifecycle GREEN (the all-vessel toolchain bump is safe) — both predate the delete-architecture conversion, so their banish/abjure legs re-verify here on the new path.
Gate prerequisite done: the reliquary was re-inscribed to carry gcrane (r260609093011) and re-yoked into all vessels (that rbrv.env change is committed).
conclave busybox-array fix and conclave single-platform fix are both landed.

## Done when
Service-tier lode + reliquary + wsl fixtures green against live GAR, run once.
- reliquary-lifecycle: re-run via `tt/rbw-tf.FixtureRun.sh reliquary-lifecycle`.
  The conclave single-platform fix (rbgjl03 `gcrane --platform linux/amd64 cp`) is UNVERIFIED live — if it fails, the first suspect is the `gcrane:debug` `--platform` flag syntax.
  Diagnose from the depot Cloud Build log: `gcloud builds log <id> --region=us-central1 --project=<depot>`, authenticated with an operator OAuth token minted from the station-files OAuth client + payor refresh token (the local gmail account lacks depot permission).
- wsl-lifecycle: never run yet — status unknown; run `tt/rbw-tf.FixtureRun.sh wsl-lifecycle`.
- Then one clean `tt/rbw-ts.TestSuite.service.sh` pass for the run-once record, then ask to wrap.
- Cloud waits dominate this pace: while polls run, the side-lane paces (paddock Execution posture) are mountable in a second officium — wraps serialize.

## Carry-forward (not blockers)
- Test debris: reliquary Lode r260609104734 (a 55-version multi-arch package) survived banish — the GAR cascade bug; the preceding delete-architecture pace consumes it as its cascade test corpse.
  Depot is disposable — no cleanup duty here.
- The delete model (cloud-dispatch banish/abjure) is cinched by the preceding pace — this gate verifies it as the fixtures run; don't re-open it.

**[260609-1616] rough**

## Character
Gate-verification + wrap only.
Implementation is landed and pushed; this is run-the-fixtures, debug-if-a-cloud-build-fails, wrap.
Do NOT re-open the eviction design or the cinched decisions.
Tier: sonnet-delegable — the debug recipe is below; escalate only on a novel failure shape.

## Goal
Capture on gcrane + curl + gpg (docker/buildx evicted); bole pins a reliquary gcrane; conclave/wsl ride floating gcrane.
Prove it via the three service-tier capture fixtures, live against GAR.
This single run is ALSO the live verification for the preceding delete-architecture pace — the fixtures' banish/abjure legs exercise the cloud-dispatch delete path; one run proves both paces.

## State at reslate
Verified live this heat: lode-lifecycle GREEN, hallmark-lifecycle GREEN (the all-vessel toolchain bump is safe) — both predate the delete-architecture conversion, so their banish/abjure legs re-verify here on the new path.
Gate prerequisite done: the reliquary was re-inscribed to carry gcrane (r260609093011) and re-yoked into all vessels (that rbrv.env change is committed).
conclave busybox-array fix and conclave single-platform fix are both landed.

## Done when
Service-tier lode + reliquary + wsl fixtures green against live GAR, run once.
- reliquary-lifecycle: re-run via `tt/rbw-tf.FixtureRun.sh reliquary-lifecycle`.
  The conclave single-platform fix (rbgjl03 `gcrane --platform linux/amd64 cp`) is UNVERIFIED live — if it fails, the first suspect is the `gcrane:debug` `--platform` flag syntax.
  Diagnose from the depot Cloud Build log: `gcloud builds log <id> --region=us-central1 --project=<depot>`, authenticated with an operator OAuth token minted from the station-files OAuth client + payor refresh token (the local gmail account lacks depot permission).
- wsl-lifecycle: never run yet — status unknown; run `tt/rbw-tf.FixtureRun.sh wsl-lifecycle`.
- Then one clean `tt/rbw-ts.TestSuite.service.sh` pass for the run-once record, then ask to wrap.
- Cloud waits dominate this pace: while polls run, the side-lane paces (paddock Execution posture) are mountable in a second officium — wraps serialize.

## Carry-forward (not blockers)
- Test debris: reliquary Lode r260609104734 (a 55-version multi-arch package) survived banish — the GAR cascade bug; the preceding delete-architecture pace consumes it as its cascade test corpse.
  Depot is disposable — no cleanup duty here.
- The delete model (cloud-dispatch banish/abjure) is cinched by the preceding pace — this gate verifies it as the fixtures run; don't re-open it.

**[260609-1443] rough**

## Character
Gate-verification + wrap only.
Implementation is landed and pushed; this is run-the-fixtures, debug-if-a-cloud-build-fails, wrap.
Do NOT re-open the eviction design or the cinched decisions.

## Goal
Capture on gcrane + curl + gpg (docker/buildx evicted); bole pins a reliquary gcrane; conclave/wsl ride floating gcrane.
Prove it via the three service-tier capture fixtures, live against GAR.

## State at reslate
Verified live this heat: lode-lifecycle GREEN, hallmark-lifecycle GREEN (the all-vessel toolchain bump is safe).
Gate prerequisite done: the reliquary was re-inscribed to carry gcrane (r260609093011) and re-yoked into all vessels (that rbrv.env change is committed).
conclave busybox-array fix and conclave single-platform fix are both landed.

## Done when
Service-tier lode + reliquary + wsl fixtures green against live GAR, run once.
- reliquary-lifecycle: re-run via `tt/rbw-tf.FixtureRun.sh reliquary-lifecycle`.
  The conclave single-platform fix (rbgjl03 `gcrane --platform linux/amd64 cp`) is UNVERIFIED live — if it fails, the first suspect is the `gcrane:debug` `--platform` flag syntax.
  Diagnose from the depot Cloud Build log: `gcloud builds log <id> --region=us-central1 --project=<depot>`, authenticated with an operator OAuth token minted from the station-files OAuth client + payor refresh token (the local gmail account lacks depot permission).
- wsl-lifecycle: never run yet — status unknown; run `tt/rbw-tf.FixtureRun.sh wsl-lifecycle`.
- Then one clean `tt/rbw-ts.TestSuite.service.sh` pass for the run-once record, then ask to wrap.

## Carry-forward (not blockers)
- Test debris: reliquary Lode r260609104734 (a 55-version multi-arch package) survived banish — the GAR multi-arch packages-delete cascade bug, not a separate failure.
  Clean it up parent-index-first (or per-version) when convenient; the cloud-dispatch banish that now precedes this pace makes any such failure loud (honest in-pool LRO poll) rather than a silent 200.
- The banish trust-200 gap and the host-vs-cloud delete-architecture question are settled and landed by the prior pace — banish and abjure now dispatch cloud-side delete-builds.
  So this gate verifies the converted banish as it runs; don't re-open the delete model.

**[260609-1136] rough**

## Character
Gate-verification + wrap only.
Implementation is landed and pushed; this is run-the-fixtures, debug-if-a-cloud-build-fails, wrap.
Do NOT re-open the eviction design or the cinched decisions.

## Goal
Capture on gcrane + curl + gpg (docker/buildx evicted); bole pins a reliquary gcrane; conclave/wsl ride floating gcrane.
Prove it via the three service-tier capture fixtures, live against GAR.

## State at reslate
Verified live this heat: lode-lifecycle GREEN, hallmark-lifecycle GREEN (the all-vessel toolchain bump is safe).
Gate prerequisite done: the reliquary was re-inscribed to carry gcrane (r260609093011) and re-yoked into all vessels (that rbrv.env change is committed).
conclave busybox-array fix and conclave single-platform fix are both landed.

## Done when
Service-tier lode + reliquary + wsl fixtures green against live GAR, run once.
- reliquary-lifecycle: re-run via `tt/rbw-tf.FixtureRun.sh reliquary-lifecycle`.
  The conclave single-platform fix (rbgjl03 `gcrane --platform linux/amd64 cp`) is UNVERIFIED live — if it fails, the first suspect is the `gcrane:debug` `--platform` flag syntax.
  Diagnose from the depot Cloud Build log: `gcloud builds log <id> --region=us-central1 --project=<depot>`, authenticated with an operator OAuth token minted from the station-files OAuth client + payor refresh token (the local gmail account lacks depot permission).
- wsl-lifecycle: never run yet — status unknown; run `tt/rbw-tf.FixtureRun.sh wsl-lifecycle`.
- Then one clean `tt/rbw-ts.TestSuite.service.sh` pass for the run-once record, then ask to wrap.

## Carry-forward (not blockers)
- Test debris: reliquary Lode r260609104734 (a 55-version multi-arch package) survived banish — it IS the GAR multi-arch `packages delete` bug, not a separate failure.
  Clean it up parent-index-first (or per-version) when convenient.
- The banish trust-200 LRO gap and the host-vs-cloud delete-architecture question are a separate reassess pace.
  ₢BHAAT does not depend on it: single-platform conclave makes the host `packages delete` succeed on its own.

**[260609-0904] rough**

## Character
Behavior-preserving tool-eviction, re-scoped under the operator's pinning rule.
The gcrane-append spike CLEARED (FROM-scratch single-layer build+push verified live on gcrane:debug, cerebro 2026-06-09), so the buildx front is unblocked.
Eviction (docker/buildx out of the capture path) is the whole-pace goal; PINNING the capture's gcrane to a reliquary applies only to bole, the one vessel-adjacent capture.

## Goal
Capture runs on gcrane + curl + gpg only — skopeo/docker/buildx evicted from the capture path.
bole additionally resolves a PINNED gcrane from its vessel's reliquary cohort.
conclave (generation) and the vessel-less substrate captures (wsl/podvm) ride the floating bootstrap gcrane; their tool-pinning is the bootstrap-builder digest-pin itch, NOT a reliquary.

## Front A — docker eviction (conclave; generation-tier, bootstrap OK)
conclave is capture's only docker consumer and is itself reliquary generation, so it keeps the unpinned bootstrap gcrane.
gcrane cp replaces docker pull/tag/push (daemonless, registry->registry); gcrane digest replaces docker inspect.
Builder-row co-edit: flip ZRBLD_GOOGLE_DOCKER_BUILDER->ZRBLD_GCRANE_BUILDER, entrypoint bash->busybox.
Canonical captures keep sprued member tags (:rbi_<tool> via RBGC_LODE_TAG_SPRUE).
Sources: grep docker across the conclave step in Tools/rbk/rbgjl; builder row in rbldr_Reliquary.sh.

## Front B-prereq — gcrane joins the reliquary cohort (LANDED)
RBGC_RELIQUARY_TOOL_GCRANE + z_rbfc_tool_gcrane resolver (rbfc0/rbfca), gcrane in the inscribe MANIFEST (rbgji01, :debug variant), yoke requires gcrane.
The pinning floor for bole.

## Front B — buildx eviction (spike CLEARED)
Move the two FROM-scratch builds onto gcrane append: the shared vouch push (rbgjl02) and the underpin opaque-blob wrap.
bole's rows resolve the PINNED reliquary gcrane (it has a vessel).
conclave's vouch row rides floating bootstrap gcrane.
wsl/underpin: EVICT buildx onto floating bootstrap gcrane (same tier as conclave) — underpin is vessel-less with no reliquary source, so its pinning defers to the bootstrap-digest-pin itch.
The underpin step splits: curl+gpg fetch/verify on a curl/gpg-capable builder, the gcrane-append wrap on a gcrane builder (gcrane:debug busybox lacks curl/gpg).
Shared-snippet boundary: rbgjl02's buildx snippets are also @rbgjs_include'd by made-side hallmark vouch (rbgjv03), outside the Lode family — fork a Lode-only gcrane-append snippet rather than convert-shared (rbgjv03 is multi-platform and out of scope).

## Cinched
- Pinning rule: reliquary GENERATION may pull unpinned upstream; any build consuming a SEALED reliquary carries zero unpinned aspects.
- gcrane (ZRBLD_GCRANE_BUILDER), never plain crane (no metadata-server auth) — cite RBSCB ambient-auth canon.
- conclave/inscribe = bootstrap floating gcrane (generation).
- bole = pinned reliquary gcrane (vessel-adjacent — RBRV_RELIQUARY in scope; durable home is our AR, not gcr.io).
- wsl/podvm = evicted onto floating gcrane this pace; pinned LATER via the bootstrap-builder digest-pin itch, NOT a reliquary.
  A reliquary is a vessel build-toolchain cohort — the wrong home for a vessel-less substrate capture's tools — and substrate capture-fidelity is the GPG/checksum verification, not tool versions.

## Done when
No docker or buildx anywhere in the capture path.
bole resolves gcrane from its reliquary; conclave and wsl ride floating gcrane.
Service-tier lode + reliquary + wsl fixtures green against live GAR, run once (rbgjl02 shared across all three — structural superset); member digests unchanged for identical upstreams.
wsl/podvm capture-tool pinning is the bootstrap-builder digest-pin itch, not this pace.

**[260609-0754] rough**

## Character
Behavior-preserving tool-eviction, re-scoped under the operator's pinning rule.
The gcrane-append spike CLEARED (FROM-scratch single-layer build+push verified live
on gcrane:debug, cerebro 2026-06-09), so the buildx front is unblocked — but landing
it on the floating ZRBLD_GCRANE_BUILDER would regress RBSCB's standing "all GCB step
images come from reliquaries" invariant. So consumption captures must resolve a
PINNED gcrane from the sealed reliquary; only conclave (generation) keeps the
bootstrap floating gcrane.

## Goal
Capture runs on gcrane + curl + gpg only (skopeo/docker/buildx evicted), AND every
sealed-reliquary-consuming capture resolves gcrane from the reliquary cohort, not the
floating bootstrap constant. Floating gcr.io gcrane survives only in reliquary
generation (conclave/inscribe), accepted as unpinned.

## Front A — docker eviction (conclave; generation-tier, bootstrap OK)
conclave is capture's only docker consumer and is itself reliquary generation, so it
keeps the unpinned bootstrap gcrane. gcrane cp replaces docker pull/tag/push
(daemonless, registry->registry); gcrane digest replaces docker inspect. Builder-row
co-edit: flip ZRBLD_GOOGLE_DOCKER_BUILDER->ZRBLD_GCRANE_BUILDER, entrypoint
bash->busybox. Sources: grep docker across the conclave step in Tools/rbk/rbgjl;
builder row in rbldr_Reliquary.sh.

## Front B-prereq — gcrane joins the reliquary cohort (the pinning floor)
Mint a RBGC_RELIQUARY_TOOL_* member for crane/gcrane so sealed captures resolve a
pinned, project-GAR gcrane (the durable home; gcr.io is bootstrap-only). inscribe/
conclave mirror it at cohort-build; verify zrbfc_resolve_tool_images carries it to
the vouch builder substitution. Front B lands on this floor — without it Front B
regresses the reliquary-sourced invariant.

## Front B — buildx eviction (spike CLEARED)
Move the two FROM-scratch builds (rbgjl02 shared vouch push; underpin opaque-blob
wrap) onto gcrane append. Consumption rows (bole/wsl) resolve the PINNED reliquary
gcrane; conclave's row may ride bootstrap. curl + gpg stay in underpin — but
gcrane:debug busybox lacks curl/gpg, so confirm the underpin builder still supplies
them or the step splits. Shared-snippet boundary, decide once: rbgjl02's buildx
snippets are also @rbgjs_include'd by made-side hallmark vouch (rbgjv03), outside the
Lode family and unexercised by the Lode fixtures — convert-shared (then rbgjv03
bind-mode needs its OWN re-verify, NOT in the gate) or fork a Lode-only snippet.

## Cinched
- Pinning rule (governs this pace): reliquary GENERATION may pull unpinned upstream;
  any build consuming a SEALED reliquary carries zero unpinned aspects.
- Durable gcrane home is our Artifact Registry (the reliquary), not gcr.io.
- gcrane (ZRBLD_GCRANE_BUILDER), never plain crane (no metadata-server auth) — cite
  RBSCB ambient-auth canon.
- conclave/inscribe = bootstrap floating gcrane; bole/wsl/podvm = pinned reliquary.

## Done when
No docker or buildx anywhere in the capture path; every sealed-reliquary capture
resolves gcrane from the reliquary. Service-tier lode + reliquary + wsl fixtures
green against live GAR, run once (rbgjl02 shared across all three — structural
superset); member digests unchanged for identical upstreams. Bootstrap-builder
digest-pin / our-AR mirror is a sibling itch, not this pace.

**[260609-0651] rough**

## Character
Behavior-preserving cutover, consolidated from three gate-sharing evictions — reaching
the locked crane-embrace floor. Bole/skopeo eviction already landed in code; this pace
adds the docker and buildx evictions, which share one verification gate. Single danger:
the gcrane-append capability (zero repo precedent). Everything else is mechanical
tool-swap across two file-disjoint coding fronts — fan to sub-agents once the spike clears.

## Goal
Capture runs on gcrane + curl + gpg only — skopeo, docker, and buildx all evicted from
the capture path. Skopeo is already gone from the bole path (landed). Evict docker from
conclave and buildx from the two FROM-scratch image builds.

## Already landed (the floor this builds on)
Bole/skopeo eviction is committed: the bole capture path is fully gcrane, the gcrane
fingerprint snippet replaced skopeo's, the token-fetch include dropped. Its
lode-lifecycle verification rides this pace's broad gate — no separate run or wrap. The
gcrane ambient-auth mechanism it set (the single-source ZRBLD_GCRANE_BUILDER, the
floating gcr.io/go-containerregistry/gcrane:debug tag, the RBSCB auth canon) is what
every eviction below reuses — no crane auth login, no token-fetch, no new IAM grant.

## Front A — docker eviction (conclave)
conclave is capture's only docker consumer. gcrane cp copies registry->registry
(daemonless, no pull-then-push); gcrane digest replaces docker inspect. Builder-row
co-edit, not just a step body: the conclave builder row flips the Google docker builder
to ZRBLD_GCRANE_BUILDER and its entrypoint bash->busybox (gcrane:debug is distroless,
only /busybox/sh). Sources: grep docker across Tools/rbk/rbgjl for the conclave step;
the builder row is in rbldr_Reliquary.sh.

## Front B — buildx eviction (verify-first)
Move the two FROM-scratch image builds onto gcrane append: the shared vouch push step
(rbgjl02, included by every kind) and the underpin opaque-blob wrap. curl + gpg stay in
underpin. BEFORE committing this front, confirm gcrane append can build+push a
FROM-scratch single-layer image on gcrane:debug — it has zero repo precedent and was
never cerebro-tested. If it cannot, STOP: descope this front, re-fork buildx-eviction as
its own pace, and land this pace as docker+bole with a {lode, reliquary} gate.
Shared-snippet boundary, decide once: the buildx snippets are also included by the
made-side hallmark vouch (rbgjv03), OUTSIDE the Lode family and unexercised by the three
Lode fixtures — either convert the shared snippet (then rbgjv03's bind-mode blast radius
needs its own re-verify, NOT covered by the gate below) or fork a Lode-only snippet.
Underpin's builder-row co-edit (image + bash->busybox) lives in rbldw_Underpin.sh.

## Cinched
- gcrane (ZRBLD_GCRANE_BUILDER), never plain crane (no metadata-server auth); cite the
  RBSCB ambient-auth canon, don't re-derive.
- The two fronts are file-disjoint (A: conclave step + rbldr_Reliquary.sh; B: rbgjl02 +
  rbldw_Underpin.sh) — parallel sub-agents once the spike clears.
- Floor reached when capture is gcrane + curl + gpg only.

## Done when
No docker or buildx invocation anywhere in the capture path. The service-tier lode +
reliquary + wsl lifecycle fixtures all green against live GAR, run once: rbgjl02 is
shared across all three kinds, so this single broad pass is the structural superset that
validates the bole, docker, and buildx evictions together; member digests unchanged for
identical upstreams. If the spike failed and buildx was descoped, the gate narrows to
{lode, reliquary} and the re-forked buildx pace owns wsl + the floor claim.

**[260609-0651] rough**

## Character
Behavior-preserving cutover, consolidated from three gate-sharing evictions — reaching
the locked crane-embrace floor. Bole/skopeo eviction already landed in code; this pace
adds the docker and buildx evictions, which share one verification gate. Single danger:
the gcrane-append capability (zero repo precedent). Everything else is mechanical
tool-swap across two file-disjoint coding fronts — fan to sub-agents once the spike clears.

## Goal
Capture runs on gcrane + curl + gpg only — skopeo, docker, and buildx all evicted from
the capture path. Skopeo is already gone from the bole path (landed). Evict docker from
conclave and buildx from the two FROM-scratch image builds.

## Already landed (the floor this builds on)
Bole/skopeo eviction is committed: the bole capture path is fully gcrane, the gcrane
fingerprint snippet replaced skopeo's, the token-fetch include dropped. Its
lode-lifecycle verification rides this pace's broad gate — no separate run or wrap. The
gcrane ambient-auth mechanism it set (the single-source ZRBLD_GCRANE_BUILDER, the
floating gcr.io/go-containerregistry/gcrane:debug tag, the RBSCB auth canon) is what
every eviction below reuses — no crane auth login, no token-fetch, no new IAM grant.

## Front A — docker eviction (conclave)
conclave is capture's only docker consumer. gcrane cp copies registry->registry
(daemonless, no pull-then-push); gcrane digest replaces docker inspect. Builder-row
co-edit, not just a step body: the conclave builder row flips the Google docker builder
to ZRBLD_GCRANE_BUILDER and its entrypoint bash->busybox (gcrane:debug is distroless,
only /busybox/sh). Sources: grep docker across Tools/rbk/rbgjl for the conclave step;
the builder row is in rbldr_Reliquary.sh.

## Front B — buildx eviction (verify-first)
Move the two FROM-scratch image builds onto gcrane append: the shared vouch push step
(rbgjl02, included by every kind) and the underpin opaque-blob wrap. curl + gpg stay in
underpin. BEFORE committing this front, confirm gcrane append can build+push a
FROM-scratch single-layer image on gcrane:debug — it has zero repo precedent and was
never cerebro-tested. If it cannot, STOP: descope this front, re-fork buildx-eviction as
its own pace, and land this pace as docker+bole with a {lode, reliquary} gate.
Shared-snippet boundary, decide once: the buildx snippets are also included by the
made-side hallmark vouch (rbgjv03), OUTSIDE the Lode family and unexercised by the three
Lode fixtures — either convert the shared snippet (then rbgjv03's bind-mode blast radius
needs its own re-verify, NOT covered by the gate below) or fork a Lode-only snippet.
Underpin's builder-row co-edit (image + bash->busybox) lives in rbldw_Underpin.sh.

## Cinched
- gcrane (ZRBLD_GCRANE_BUILDER), never plain crane (no metadata-server auth); cite the
  RBSCB ambient-auth canon, don't re-derive.
- The two fronts are file-disjoint (A: conclave step + rbldr_Reliquary.sh; B: rbgjl02 +
  rbldw_Underpin.sh) — parallel sub-agents once the spike clears.
- Floor reached when capture is gcrane + curl + gpg only.

## Done when
No docker or buildx invocation anywhere in the capture path. The service-tier lode +
reliquary + wsl lifecycle fixtures all green against live GAR, run once: rbgjl02 is
shared across all three kinds, so this single broad pass is the structural superset that
validates the bole, docker, and buildx evictions together; member digests unchanged for
identical upstreams. If the spike failed and buildx was descoped, the gate narrows to
{lode, reliquary} and the re-forked buildx pace owns wsl + the floor claim.

**[260609-0617] rough**

## Character
Behavior-preserving, fidelity-critical cutover — DONE IN CODE. First of the
crane-embrace set; the gcrane ambient-auth mechanism it set is inherited by every
later eviction pace.

## Status
Code eviction committed: the bole capture path is fully gcrane (manifest/cp/tag), the
gcrane fingerprint snippet replaced the skopeo one, the token-fetch include is dropped.
Remaining work is ONLY the live-GAR fixture run + wrap — no code.

## Done when
- lode-lifecycle service fixture green against live GAR, the :rbi_sha256 digest tag
  unchanged for an identical upstream (fidelity preserved).

## Cinched
- gcrane, not plain crane (plain crane has no metadata-server auth); builder named by
  the single source-of-truth ZRBLD_GCRANE_BUILDER constant.
- Builder uses the FLOATING gcr.io/go-containerregistry/gcrane:debug tag, NOT
  pinned-by-digest (operator-blessed: version-freezing belongs to the reliquary gather,
  not a scattered bash digest). Supersedes the earlier "pinned by digest" cinch.
- Auth canon landed in RBSCB; ensconce-spec auth update in RBSLE.

**[260608-2043] rough**

## Character
Behavior-preserving, fidelity-critical cutover. First of the crane-embrace set:
proves the gcrane pattern on the smallest kind; the auth mechanism it sets is
inherited by every later eviction pace.

## Goal
Evict skopeo from the bole capture path by moving it onto gcrane (crane's
Google-auth sibling — same cp/manifest/tag engine): skopeo inspect/copy/retag ->
gcrane manifest/cp/tag (gcrane tag replaces the GAR->GAR retag round-trip), the
shared fingerprint snippet rebased on gcrane and renamed off "skopeo", and the bole
step's builder swapped to the gcrane:debug image with the token-fetch include
dropped (gcrane auths GAR ambiently). bole is the only skopeo consumer in the
capture path; docker and buildx get their own paces.

## Done when
- No skopeo invocation or token-fetch include in the bole capture path; fingerprint
  snippet gcrane-based and renamed.
- lode-lifecycle service fixture green against live GAR, the :rbi_sha256 digest tag
  unchanged for an identical upstream (fidelity preserved).

## Cinched
- Tool is gcrane, not plain crane (plain crane has no metadata-server auth); do not
  reopen oras. Builder gcr.io/go-containerregistry/gcrane:debug (non-debug is
  distroless — no shell), pinned by digest.
- Auth is SETTLED, inherited downstream: gcrane's google.Keychain auths GAR
  (*.pkg.dev) ambiently from the GCE metadata server as the Mason SA — no
  crane-auth-login, no credential-helper image, no token-fetch, NO new IAM grant.
  The earlier explicit-login-vs-cred-helper fork is resolved by a third path.
- Durable auth canon lands in RBSCB (supersedes its skopeo token-fetch rationale);
  ensconce-spec auth update in RBSLE.

## Sources
- memo-20260608-lode-podvm-cerebro-experiment.md §9 — gcrane ambient-auth decision +
  evidence; §1/§4 crane fidelity + corrected skopeo failure mode.
- grep skopeo across Tools/rbk/rbgjl + Tools/rbk/rbgjs for the bole sites.

**[260608-1950] rough**

## Character
Behavior-preserving, fidelity-critical cutover. First of the crane-embrace set:
proves the crane pattern on the smallest kind, sets the auth mechanism later paces inherit.

## Goal
Evict skopeo by moving bole capture onto crane — the ensconce capture step + its shared
fingerprint snippet, skopeo inspect/copy/retag -> crane manifest/cp/tag (crane tag
replaces the GAR->GAR retag round-trip), and swap the bole step off the skopeo builder.
bole is the only skopeo consumer; docker and buildx get their own eviction paces.

## Done when
- No skopeo invocation in the bole capture path; fingerprint snippet crane-based and renamed.
- lode-lifecycle service fixture green against live GAR, the :rbi_sha256 digest tag unchanged
  for an identical upstream (fidelity preserved).

## Cinched
- crane is settled; do not reopen oras.
- Auth mechanism is decided HERE and inherited downstream: explicit per-step `crane auth
  login` vs a credential-helper builder image. Same Mason SA + short-lived metadata token,
  NO new IAM/repo grant either way.

## Sources
- memo-20260608-lode-podvm-cerebro-experiment.md — crane decision, corrected skopeo failure mode.
- grep skopeo across Tools/rbk/rbgjl + Tools/rbk/rbgjs to find the bole sites.

**[260608-1937] rough**

## Character
Behavior-preserving cloud-step cutover. Mechanical but fidelity-critical — the bole
capture digest must not move.

## Goal
Evict skopeo from the Lode capture path by moving bole capture onto crane. Bole is
the last skopeo consumer in capture (conclave already uses docker, vouch uses
buildx), so converting it — plus its shared fingerprint snippet and its builder
image — removes skopeo from the capture path entirely. Drop skopeo from the
reliquary tool cohort if nothing else still consumes it.

## Done when
- No skopeo invocation remains in the Lode capture path.
- The lode-lifecycle service fixture passes against live GAR with the canonical
  :rbi_sha256 digest tag unchanged for an identical upstream (fidelity preserved).
- skopeo is either dropped from the reliquary tool cohort, or its remaining
  consumer is noted.

## Cinched
- Skopeo eviction ONLY. Moving conclave (docker) and vouch (buildx) onto crane —
  the broader crane-everywhere consolidation — is explicitly out of scope.
- crane is the settled capture tool; do not reopen oras (documented-fallback only).

## Sources
- Memo memo-20260608-lode-podvm-cerebro-experiment.md — crane decision and the
  corrected skopeo failure mode (loud fatal on empty-config, not silent skip).
- Discovery recipe: grep skopeo across Tools/rbk/rbgjl and Tools/rbk/rbgjs;
  classify each hit convert / rename / drop. Crane builder: ride a podvm capture
  pace's if one has landed, else introduce crane (a static ELF, cheap to add).

### image-backdoor-path-verbs (₢BHAAa) [complete]

**[260610-0703] complete**

## Character
Net-new low-level service — the type-blind raw maintenance layer beneath the semantic Lode verbs (paddock Carried-forward, path-polymorphic raw layer).
Tier: sonnet-delegable once the cinches below are honored; the design is settled here.

## Goal
Reconceive the image-family maintenance verbs as three path-polymorphic verbs on raw GAR paths — list, wrest, jettison — so ultimate cleanup after surprises needs no type awareness, no envelope, and no per-kind verbs.
list with no argument shows the top namespaces; with a prefix, the packages beneath; with a full package, its tags and versions — discovery by iterative narrowing.
wrest and jettison act on a full image ref.

## Cinched
- Disambiguation rule (total, because GAR's deletable leaves are exactly tags and versions): a parameter carrying `:tag` or `@sha256:` is an image — act on it; anything else is a path — list its children.
- Package-grain delete stays with the semantic verbs (banish/abjure, cloud-dispatched). The backdoor deletes below package grain only.
- Host-side REST with honest LRO handling (`rbge_lro_ok` where GAR returns an LRO; tag deletes are synchronous) — a cleanup tool of last resort must not depend on the cloud pipeline it cleans up after.
  This is deliberately NOT the cloud-dispatch model of the delete-architecture pace.
- Envelope-independent by construction: must work on a half-deleted package, a corrupt envelope, legacy rbi_es artifacts that were never Lodes, and any future kind.
- Made-side iah/irh/iwh/iJh untouched this heat — rekon carries a semantic canonical-member contract (hallmark-lifecycle asserts its non-zero exit) that belongs to the made-side retrofit heat.
- iwe/iJe (enshrinement wrest/jettison) retire here — subsumed by the generic verbs (depot is disposable; rbi_es needs no named variants).
- iar/irr/iwr/iJr retire at the inscribe cutover with rbi_rq, not here — until then they still serve the standing reliquary.
- Colophons: mint at mount against the existing rbw-i* tree; jettison keeps the capital convention of its siblings (GAR-destroying), list and wrest stay lowercase.

## Done when
- The three verbs work against live GAR at every grain (namespaces -> packages -> tags/versions -> act on a full ref).
- A theurge case proves member-grain jettison: capture a Lode, jettison one member tag/version via the generic verb, assert the member absent and the Lode otherwise intact — the per-member-delete assertion the multi-member kinds need (the podvm fixture builds on it).
- iwe/iJe gone; tabtarget-context doc updated.
- `fast` green; the new case rides the service tier, batched with adjacent paces (paddock Execution posture).

**[260609-1616] rough**

## Character
Net-new low-level service — the type-blind raw maintenance layer beneath the semantic Lode verbs (paddock Carried-forward, path-polymorphic raw layer).
Tier: sonnet-delegable once the cinches below are honored; the design is settled here.

## Goal
Reconceive the image-family maintenance verbs as three path-polymorphic verbs on raw GAR paths — list, wrest, jettison — so ultimate cleanup after surprises needs no type awareness, no envelope, and no per-kind verbs.
list with no argument shows the top namespaces; with a prefix, the packages beneath; with a full package, its tags and versions — discovery by iterative narrowing.
wrest and jettison act on a full image ref.

## Cinched
- Disambiguation rule (total, because GAR's deletable leaves are exactly tags and versions): a parameter carrying `:tag` or `@sha256:` is an image — act on it; anything else is a path — list its children.
- Package-grain delete stays with the semantic verbs (banish/abjure, cloud-dispatched). The backdoor deletes below package grain only.
- Host-side REST with honest LRO handling (`rbge_lro_ok` where GAR returns an LRO; tag deletes are synchronous) — a cleanup tool of last resort must not depend on the cloud pipeline it cleans up after.
  This is deliberately NOT the cloud-dispatch model of the delete-architecture pace.
- Envelope-independent by construction: must work on a half-deleted package, a corrupt envelope, legacy rbi_es artifacts that were never Lodes, and any future kind.
- Made-side iah/irh/iwh/iJh untouched this heat — rekon carries a semantic canonical-member contract (hallmark-lifecycle asserts its non-zero exit) that belongs to the made-side retrofit heat.
- iwe/iJe (enshrinement wrest/jettison) retire here — subsumed by the generic verbs (depot is disposable; rbi_es needs no named variants).
- iar/irr/iwr/iJr retire at the inscribe cutover with rbi_rq, not here — until then they still serve the standing reliquary.
- Colophons: mint at mount against the existing rbw-i* tree; jettison keeps the capital convention of its siblings (GAR-destroying), list and wrest stay lowercase.

## Done when
- The three verbs work against live GAR at every grain (namespaces -> packages -> tags/versions -> act on a full ref).
- A theurge case proves member-grain jettison: capture a Lode, jettison one member tag/version via the generic verb, assert the member absent and the Lode otherwise intact — the per-member-delete assertion the multi-member kinds need (the podvm fixture builds on it).
- iwe/iJe gone; tabtarget-context doc updated.
- `fast` green; the new case rides the service tier, batched with adjacent paces (paddock Execution posture).

### lode-augur-inspect-split (₢BHAAN) [complete]

**[260610-0833] complete**

## Character
Small standalone — split a read verb out of divine, plus the net-new envelope decode the spec mandates.
Kind-agnostic; blocks no vertical, but runs BEFORE the podvm vertical so its fixture is authored against the final verb surface.
Tier: sonnet-delegable — specs are written; the repoint list below is exhaustive.

## Goal
Realize the divine/augur grain split per the landed RBSLA/RBSLD specs: extract single-Lode inspect out of divine into augur (rbw-la), trim divine to enumerate-only, and implement the augur substance — decoding the rbi_vouch provenance envelope, which divine's inspect branch never did.

## Cinched
- Code to RBSLA/RBSLD (already written). The verb split is mechanical (mirror the rbw-ld trampoline; require the touchmark folio as banish does). The envelope decode (the provenance fields + the verified-vs-recorded posture) is the real new logic.
- augur is kind-agnostic — no per-kind work.
- The split breaks every existing divine-inspect consumer: the three lifecycle fixtures (lode, reliquary, wsl) call divine-inspect in rbtdrc_crucible.rs to assert member tags — grep `divine-inspect` there for the exact sites.
  Repoint all three to augur in this pace.

## Done when
- rbw-la augur decodes and displays one Lode's provenance envelope; divine is enumerate-only.
- theurge covers the split with an explicit augur case: a lifecycle fixture asserts augur decodes the :rbi_vouch envelope (the new logic), not merely that divine enumerates.
- All existing divine-inspect call sites repointed; verified via one service-tier run, batched with adjacent paces at operator discretion (paddock Execution posture).

**[260609-1616] rough**

## Character
Small standalone — split a read verb out of divine, plus the net-new envelope decode the spec mandates.
Kind-agnostic; blocks no vertical, but runs BEFORE the podvm vertical so its fixture is authored against the final verb surface.
Tier: sonnet-delegable — specs are written; the repoint list below is exhaustive.

## Goal
Realize the divine/augur grain split per the landed RBSLA/RBSLD specs: extract single-Lode inspect out of divine into augur (rbw-la), trim divine to enumerate-only, and implement the augur substance — decoding the rbi_vouch provenance envelope, which divine's inspect branch never did.

## Cinched
- Code to RBSLA/RBSLD (already written). The verb split is mechanical (mirror the rbw-ld trampoline; require the touchmark folio as banish does). The envelope decode (the provenance fields + the verified-vs-recorded posture) is the real new logic.
- augur is kind-agnostic — no per-kind work.
- The split breaks every existing divine-inspect consumer: the three lifecycle fixtures (lode, reliquary, wsl) call divine-inspect in rbtdrc_crucible.rs to assert member tags — grep `divine-inspect` there for the exact sites.
  Repoint all three to augur in this pace.

## Done when
- rbw-la augur decodes and displays one Lode's provenance envelope; divine is enumerate-only.
- theurge covers the split with an explicit augur case: a lifecycle fixture asserts augur decodes the :rbi_vouch envelope (the new logic), not merely that divine enumerates.
- All existing divine-inspect call sites repointed; verified via one service-tier run, batched with adjacent paces at operator discretion (paddock Execution posture).

**[260609-0617] rough**

## Character
Small standalone — split a read verb out of divine, plus the net-new envelope decode the
spec mandates. Kind-agnostic; blocks no vertical.

## Goal
Realize the divine/augur grain split per the landed RBSLA/RBSLD specs: extract
single-Lode inspect out of divine into augur (rbw-la), trim divine to enumerate-only, and
implement the augur substance — decoding the rbi_vouch provenance envelope, which
divine's inspect branch never did.

## Cinched
- Code to RBSLA/RBSLD (already written). The verb split is mechanical (mirror the rbw-ld
  trampoline; require the touchmark folio as banish does). The envelope decode (the
  provenance fields + the verified-vs-recorded posture) is the real new logic.
- augur is kind-agnostic — no per-kind work.

## Done when
- rbw-la augur decodes and displays one Lode's provenance envelope; divine is
  enumerate-only.
- theurge covers the split with an explicit augur case: a lifecycle fixture asserts augur
  decodes the :rbi_vouch envelope (the new logic), not merely that divine enumerates. The
  existing divine coverage does not exercise the envelope decode.

**[260606-1012] rough**

## Character
Small standalone — split a read verb out of divine, plus the net-new envelope decode the spec mandates. Kind-agnostic; blocks no vertical.

## Goal
Realize the divine/augur grain split per the landed RBSLA/RBSLD specs: extract single-Lode inspect out of divine into augur (rbw-la), trim divine to enumerate-only, and implement the augur substance — decoding the rbi_vouch provenance envelope, which divine's inspect branch never did.

## Cinched
- Code to RBSLA/RBSLD (already written). The verb split is mechanical (mirror the rbw-ld trampoline; require the touchmark folio as banish does). The envelope decode (the provenance fields + the verified-vs-recorded posture) is the real new logic.
- augur is kind-agnostic — no per-kind work.

## Done
rbw-la augur decodes and displays one Lode's provenance envelope; divine is enumerate-only; theurge covers the split.

**[260605-1120] rough**

## Character
Small standalone — split a read verb out of divine, plus the net-new envelope decode the spec mandates. Kind-agnostic; blocks no vertical.

## Goal
Realize the divine/augur grain split per the landed RBSLA/RBSLD specs: extract single-Lode inspect out of divine into augur (rbw-la), trim divine to enumerate-only, and implement the augur substance — decoding the rbi_vouch provenance envelope, which divine's inspect branch never did.

## Locked
- Code to RBSLA/RBSLD (already written). The verb split is mechanical (mirror the rbw-ld trampoline; require the touchmark folio as banish does). The envelope decode (the provenance fields + the verified-vs-recorded posture) is the real new logic.
- augur is kind-agnostic — no per-kind work.

## Done
rbw-la augur decodes and displays one Lode's provenance envelope; divine is enumerate-only; theurge covers the split.

### lode-enshrine-spec-retire (₢BHAAY) [complete]

**[260610-0859] complete**

## Character
Focused spec retirement — lifted out of the vocab scrub because deleting a full operation spec is heavier than a word-level sweep and deserves its own visibility.
Side-lane pace: file-disjoint from the spine, mountable in a second officium — but must land BEFORE the inscribe cutover mounts (both touch the RBS0 mapping section).
Tier: haiku-capable with the recipe below.

## Goal
Retire RBSAE-ark_enshrine.adoc — a LIVE spec describing the dead skopeo-driven enshrine-CREATION path. The bole cutover already deleted its step scripts (rbgje*) and the rbfd_enshrine creation gesture; only the spec remains, describing a path nothing runs, with no superseded banner. Enshrine-as-creation is superseded by bole gcrane Lode capture.

## Cinched
- Enshrine-as-creation IS retired: only wrest/jettison of already-existing enshrinements survive, as registry artifacts, served by the path-polymorphic image backdoor (depot is disposable — rbi_es needs no named variants). The CREATION spec dies.
- Default hard delete (the vocab-scrub's original intent: "delete it, do not adapt it"). At mount, decide whether a brief superseded-banner-then-stub better serves a reader who finds an old enshrinement — operator's call.
- Also retire the rbtgo_ark_enshrine quoin in RBS0.

## Recipe
Delete (or stub) RBSAE; remove the rbtgo_ark_enshrine quoin from the RBS0 mapping section and its definition site; `grep -rn 'RBSAE\|rbtgo_ark_enshrine\|ark_enshrine' Tools/` until zero un-guarded hits; run `fast`.

## Done when
RBSAE no longer describes a live enshrine-creation operation (deleted or stubbed-as-retired); the rbtgo_ark_enshrine quoin is gone; no cross-ref dangles; `fast` green.

**[260609-1616] rough**

## Character
Focused spec retirement — lifted out of the vocab scrub because deleting a full operation spec is heavier than a word-level sweep and deserves its own visibility.
Side-lane pace: file-disjoint from the spine, mountable in a second officium — but must land BEFORE the inscribe cutover mounts (both touch the RBS0 mapping section).
Tier: haiku-capable with the recipe below.

## Goal
Retire RBSAE-ark_enshrine.adoc — a LIVE spec describing the dead skopeo-driven enshrine-CREATION path. The bole cutover already deleted its step scripts (rbgje*) and the rbfd_enshrine creation gesture; only the spec remains, describing a path nothing runs, with no superseded banner. Enshrine-as-creation is superseded by bole gcrane Lode capture.

## Cinched
- Enshrine-as-creation IS retired: only wrest/jettison of already-existing enshrinements survive, as registry artifacts, served by the path-polymorphic image backdoor (depot is disposable — rbi_es needs no named variants). The CREATION spec dies.
- Default hard delete (the vocab-scrub's original intent: "delete it, do not adapt it"). At mount, decide whether a brief superseded-banner-then-stub better serves a reader who finds an old enshrinement — operator's call.
- Also retire the rbtgo_ark_enshrine quoin in RBS0.

## Recipe
Delete (or stub) RBSAE; remove the rbtgo_ark_enshrine quoin from the RBS0 mapping section and its definition site; `grep -rn 'RBSAE\|rbtgo_ark_enshrine\|ark_enshrine' Tools/` until zero un-guarded hits; run `fast`.

## Done when
RBSAE no longer describes a live enshrine-creation operation (deleted or stubbed-as-retired); the rbtgo_ark_enshrine quoin is gone; no cross-ref dangles; `fast` green.

**[260609-0616] rough**

## Character
Focused spec retirement — lifted out of the vocab scrub because deleting a full
operation spec is heavier than a word-level sweep and deserves its own visibility.

## Goal
Retire RBSAE-ark_enshrine.adoc — a LIVE spec describing the dead skopeo-driven
enshrine-CREATION path. The bole cutover already deleted its step scripts (rbgje*)
and the rbfd_enshrine creation gesture; only the spec remains, describing a path
nothing runs, with no superseded banner. Enshrine-as-creation is superseded by bole
gcrane Lode capture.

## Cinched
- Enshrine-as-creation IS retired: only wrest/jettison of already-existing
  enshrinements survive (rbw-iwe / rbw-iJe), as registry artifacts, until the deferred
  rbi_es -> rbi_ld migration. The CREATION spec dies; the artifact-maintenance verbs
  are out of scope here.
- Default hard delete (the vocab-scrub's original intent: "delete it, do not adapt
  it"). At mount, decide whether a brief superseded-banner-then-stub better serves a
  reader who finds an old enshrinement — operator's call.
- Also retire the rbtgo_ark_enshrine quoin in RBS0.

## Done when
RBSAE no longer describes a live enshrine-creation operation (deleted or stubbed-as-
retired); the rbtgo_ark_enshrine quoin is gone; no cross-ref dangles; `fast` green.

### theurge-fixture-fact-chain-fix (₢BHAAS) [complete]

**[260609-0713] complete**

## Character
Test-infra fix. The cutover's verify gate is structurally blind — make it see. Root cause is known and proven live; the mechanism is mount's call.

## Goal
Make the airgap-chain fixture genuinely exercise the bole derived-pull election, so the cutover gains a real automated regression guard. Today it false-greens.

## Problem
`zrbfd_elect_base_anchor` reads the bole touchmark from the depth-1 BURD `previous/` chain (ensconce emits it; the next ordain dispatch reads it). theurge isolates every tabtarget invocation in its own `burv_output` root (`rbtdri_invoke_impl`), so `bud_dispatch`'s `current/`->`previous/` promotion never chains across invokes — ordain's `BURD_PREVIOUS_DIR` is empty, the election takes its no-op guard, and the conjure builds from the lingering `rbi_es` base: green but meaningless. The real operator flow chains because sequential tabtargets share `../output-buk`.

## Done
The fixture drives ensconce->ordain so the touchmark reaches the election, then asserts the forge anchor became `rbi_ld/<touchmark>:rbi_bole` and the conjure built from it. A non-firing election fails the fixture.

## Cinched
- Restore depth-1 chaining only for invokes meant to chain; don't collapse `burv` isolation for the rest.
- The election itself is correct — proven live this heat: back-to-back ensconce->ordain on a forge vessel repointed the anchor `rbi_es`->`rbi_ld` and the conjure built+vouched green from it. The defect is in harness invoke-isolation, not the election.

**[260608-1056] rough**

## Character
Test-infra fix. The cutover's verify gate is structurally blind — make it see. Root cause is known and proven live; the mechanism is mount's call.

## Goal
Make the airgap-chain fixture genuinely exercise the bole derived-pull election, so the cutover gains a real automated regression guard. Today it false-greens.

## Problem
`zrbfd_elect_base_anchor` reads the bole touchmark from the depth-1 BURD `previous/` chain (ensconce emits it; the next ordain dispatch reads it). theurge isolates every tabtarget invocation in its own `burv_output` root (`rbtdri_invoke_impl`), so `bud_dispatch`'s `current/`->`previous/` promotion never chains across invokes — ordain's `BURD_PREVIOUS_DIR` is empty, the election takes its no-op guard, and the conjure builds from the lingering `rbi_es` base: green but meaningless. The real operator flow chains because sequential tabtargets share `../output-buk`.

## Done
The fixture drives ensconce->ordain so the touchmark reaches the election, then asserts the forge anchor became `rbi_ld/<touchmark>:rbi_bole` and the conjure built from it. A non-firing election fails the fixture.

## Cinched
- Restore depth-1 chaining only for invokes meant to chain; don't collapse `burv` isolation for the rest.
- The election itself is correct — proven live this heat: back-to-back ensconce->ordain on a forge vessel repointed the anchor `rbi_es`->`rbi_ld` and the conjure built+vouched green from it. The defect is in harness invoke-isolation, not the election.

### lode-docker-eviction (₢BHAAU) [abandoned]

**[260609-0652] abandoned**

## Character
Behavior-preserving cutover. Removes the docker daemon dependency from conclave.

## Goal
Evict docker from capture by moving conclave onto gcrane. The conclave capture step
does docker pull/tag/push per tool; gcrane cp copies registry->registry directly
(daemonless, no pull-then-push), and gcrane digest replaces docker inspect for the
recorded digest. conclave is the only docker consumer in capture.

## Done when
- No docker invocation in the conclave capture path; the step runs on the gcrane
  builder. reliquary-lifecycle service fixture green against live GAR, member digests
  unchanged.

## Cinched
- Tool is gcrane (named by ZRBLD_GCRANE_BUILDER), NOT plain crane — reuse the
  ambient-auth mechanism the skopeo-eviction pace set.
- The eviction is a builder-row co-edit, not just a step-body edit: the conclave builder
  row in rbldr_Reliquary.sh (the ZRBLD_GOOGLE_DOCKER_BUILDER + bash pair) flips to
  ZRBLD_GCRANE_BUILDER, and the entrypoint bash -> busybox (gcrane:debug is distroless,
  carries only /busybox/sh).

## Sources
- grep docker across Tools/rbk/rbgjl for the conclave capture step; the builder row is
  in rbldr_Reliquary.sh.

**[260609-0617] rough**

## Character
Behavior-preserving cutover. Removes the docker daemon dependency from conclave.

## Goal
Evict docker from capture by moving conclave onto gcrane. The conclave capture step
does docker pull/tag/push per tool; gcrane cp copies registry->registry directly
(daemonless, no pull-then-push), and gcrane digest replaces docker inspect for the
recorded digest. conclave is the only docker consumer in capture.

## Done when
- No docker invocation in the conclave capture path; the step runs on the gcrane
  builder. reliquary-lifecycle service fixture green against live GAR, member digests
  unchanged.

## Cinched
- Tool is gcrane (named by ZRBLD_GCRANE_BUILDER), NOT plain crane — reuse the
  ambient-auth mechanism the skopeo-eviction pace set.
- The eviction is a builder-row co-edit, not just a step-body edit: the conclave builder
  row in rbldr_Reliquary.sh (the ZRBLD_GOOGLE_DOCKER_BUILDER + bash pair) flips to
  ZRBLD_GCRANE_BUILDER, and the entrypoint bash -> busybox (gcrane:debug is distroless,
  carries only /busybox/sh).

## Sources
- grep docker across Tools/rbk/rbgjl for the conclave capture step; the builder row is
  in rbldr_Reliquary.sh.

**[260608-1950] rough**

## Character
Behavior-preserving cutover. Removes the docker daemon dependency from conclave.

## Goal
Evict docker from capture by moving conclave onto crane. The conclave capture step does
docker pull/tag/push per tool; crane cp copies registry->registry directly (daemonless,
no pull-then-push), and crane digest replaces docker inspect for the recorded digest.
conclave is the only docker consumer in capture.

## Done when
- No docker invocation in the conclave capture path; the step runs on a crane builder.
- reliquary-lifecycle service fixture green against live GAR, member digests unchanged.

## Cinched
- crane is settled. Reuse the auth mechanism chosen in the skopeo-eviction pace.

## Sources
- grep docker across Tools/rbk/rbgjl to find the conclave capture step.

### lode-buildx-eviction (₢BHAAV) [abandoned]

**[260609-0652] abandoned**

## Character
Behavior-preserving cutover. Proves gcrane append (tarball->image, daemonless) and
removes the last docker-daemon dependency from capture. The shared-vouch site is the
highest blast radius — every kind's vouch push changes. Carries the heat's single
highest capability risk (see Verify-first).

## Goal
Evict buildx by moving the two FROM-scratch image builds onto gcrane append: the shared
vouch push step (used by every kind) and the underpin opaque-blob wrap. curl and gpg
stay in underpin — HTTPS fetch and signature verify are not registry ops.

## Verify-first (do BEFORE committing the approach)
"gcrane append" has ZERO precedent in this repo and was never tested on cerebro (only
cp/manifest were). Confirm gcrane append can build+push a FROM-scratch single-layer
image on gcr.io/go-containerregistry/gcrane:debug FIRST. If it cannot, STOP and raise
it — the pace forks (keep buildx for vouch, or a different gcrane mechanism). Do not
assume the superset claim holds for append.

## Done when
- No buildx/docker invocation in vouch or underpin; both use gcrane append.
- lode-lifecycle, reliquary-lifecycle, and wsl-lifecycle fixtures all green against live
  GAR (vouch is shared, so every kind re-validates).
- Floor reached: capture is gcrane (all registry/image ops) + curl + gpg only.

## Shared-snippet boundary (decide once)
The buildx snippets (rbgjs-buildx-push.sh / rbgjs-buildx-bootstrap.sh) are ALSO included
by rbgjv03-assemble-push-vouch.sh — hallmark-verify, OUTSIDE the Lode family, and
unexercised by the three Lode fixtures. Decide: convert the shared snippet (reaching
rbgjv03, which then needs its own re-verify) or fork a Lode-only snippet.

## Cinched
- gcrane (ZRBLD_GCRANE_BUILDER), not plain crane — reuse the skopeo-eviction auth
  mechanism. Underpin's builder-row co-edit (image + bash->busybox) lives in
  rbldw_Underpin.sh, not just the step body.

## Sources
- grep buildx across Tools/rbk/rbgjl + Tools/rbk/rbgjs for the vouch + underpin-wrap
  sites; rbgjv03 for the shared-snippet caller.

**[260609-0617] rough**

## Character
Behavior-preserving cutover. Proves gcrane append (tarball->image, daemonless) and
removes the last docker-daemon dependency from capture. The shared-vouch site is the
highest blast radius — every kind's vouch push changes. Carries the heat's single
highest capability risk (see Verify-first).

## Goal
Evict buildx by moving the two FROM-scratch image builds onto gcrane append: the shared
vouch push step (used by every kind) and the underpin opaque-blob wrap. curl and gpg
stay in underpin — HTTPS fetch and signature verify are not registry ops.

## Verify-first (do BEFORE committing the approach)
"gcrane append" has ZERO precedent in this repo and was never tested on cerebro (only
cp/manifest were). Confirm gcrane append can build+push a FROM-scratch single-layer
image on gcr.io/go-containerregistry/gcrane:debug FIRST. If it cannot, STOP and raise
it — the pace forks (keep buildx for vouch, or a different gcrane mechanism). Do not
assume the superset claim holds for append.

## Done when
- No buildx/docker invocation in vouch or underpin; both use gcrane append.
- lode-lifecycle, reliquary-lifecycle, and wsl-lifecycle fixtures all green against live
  GAR (vouch is shared, so every kind re-validates).
- Floor reached: capture is gcrane (all registry/image ops) + curl + gpg only.

## Shared-snippet boundary (decide once)
The buildx snippets (rbgjs-buildx-push.sh / rbgjs-buildx-bootstrap.sh) are ALSO included
by rbgjv03-assemble-push-vouch.sh — hallmark-verify, OUTSIDE the Lode family, and
unexercised by the three Lode fixtures. Decide: convert the shared snippet (reaching
rbgjv03, which then needs its own re-verify) or fork a Lode-only snippet.

## Cinched
- gcrane (ZRBLD_GCRANE_BUILDER), not plain crane — reuse the skopeo-eviction auth
  mechanism. Underpin's builder-row co-edit (image + bash->busybox) lives in
  rbldw_Underpin.sh, not just the step body.

## Sources
- grep buildx across Tools/rbk/rbgjl + Tools/rbk/rbgjs for the vouch + underpin-wrap
  sites; rbgjv03 for the shared-snippet caller.

**[260608-1950] rough**

## Character
Behavior-preserving cutover. Proves crane append (tarball->image, daemonless) and removes
the last docker-daemon dependency from capture. The shared-vouch site is the highest blast
radius — every kind's vouch push changes.

## Goal
Evict buildx by moving the two FROM-scratch image builds onto crane append: the shared
vouch push step (used by every kind) and the underpin opaque-blob wrap. curl and gpg stay
in underpin — HTTPS fetch and signature verify are not registry ops crane can do.

## Done when
- No buildx/docker invocation in vouch or underpin; both use crane append.
- lode-lifecycle, reliquary-lifecycle, and wsl-lifecycle fixtures all green against live
  GAR (vouch is shared, so every kind re-validates).
- Floor reached: capture is crane (all registry/image ops) + curl + gpg only.

## Cinched
- crane is settled. Reuse the auth mechanism from the skopeo-eviction pace.

## Sources
- grep buildx across Tools/rbk/rbgjl + Tools/rbk/rbgjs to find the vouch + underpin-wrap sites.

### lode-podvm-immure (₢BHAAW) [complete]

**[260610-1024] complete**

## Character
Net-new kind, base immure machinery — proves the podvm vertical on ONE family in an already-crane world.
The two-family fan-out + curated multi-platform selection is the FOLLOWING pace.
Tier: sonnet-delegable for body and step (heavily templated — see copy-shape cinch); the spec subdoc wants driver judgment.

## Goal
Stand up the podvm immure vertical (verb rbw-lI): a new in-pool step that gcrane-cp's selected disk leaves BY DIGEST into rbi_ld/<vn|vw><stamp>, runs the blob-residency guard, and reuses the gcrane vouch step; the new rbldv_ body (opaque-blob x multi-member); and the kind-registration surface (divine legend + RBTDRC_FIXTURES).
Prove it end-to-end on one family. Land the podvm RBSL subdoc + RBS0 quoin (spec covers BOTH families) and promote RBSPV out of FUTURE/.

## Done when
- immure captures one family end-to-end; a podvm-lifecycle service fixture green (immure -> divine cohort -> augur members + :rbi_vouch -> per-member jettison via the generic image backdoor -> whole-Lode banish -> absent), in service + complete suites; RBTDRC_FIXTURES registers it; divine legend carries the kind-letter.
- RBSL podvm subdoc + RBS0 quoin landed; RBSPV promoted out of FUTURE/.

## Cinched
- recorded-at-acquisition grade. Selective leaf capture, declarative intent (no FQIN); selection keys on the index descriptor platform + disktype, never the layer filename. Cloud-side only.
- Tool is gcrane via the floating bootstrap (ZRBLD_GCRANE_BUILDER) — podvm is vessel-less like wsl; pinning is the bootstrap-digest-pin itch, not this pace (paddock pinning boundary).
- Copy-shape: rbldw_Underpin.sh is the literal template for the body + spine registration; rbgjl03 (cohort selection loop) and rbgjl05 (gcrane wrap) for the step shape; the memo §7 build sheet is the recipe.
- Selected leaves are single-platform manifests (no index web), so banish stays flat-package safe by construction — keep it that way.
- RBSPV is authored in plain "crane" throughout — convert that prose to gcrane on promotion, and cite the existing RBSCB ambient-auth canon rather than re-deriving it.
  Do NOT resurrect the legacy FUTURE/rbv_PodmanVM.sh ignite-VM plain-crane design — cloud-side gcrane supersedes it entirely.
- Second family + curated multi-platform set + refresh mode = the FOLLOWING pace.

## Sources
- memo-20260608-lode-podvm-cerebro-experiment.md §7 (build sheet); RBSPV cerebro section; paddock Vocabulary + Kinds.

**[260609-1616] rough**

## Character
Net-new kind, base immure machinery — proves the podvm vertical on ONE family in an already-crane world.
The two-family fan-out + curated multi-platform selection is the FOLLOWING pace.
Tier: sonnet-delegable for body and step (heavily templated — see copy-shape cinch); the spec subdoc wants driver judgment.

## Goal
Stand up the podvm immure vertical (verb rbw-lI): a new in-pool step that gcrane-cp's selected disk leaves BY DIGEST into rbi_ld/<vn|vw><stamp>, runs the blob-residency guard, and reuses the gcrane vouch step; the new rbldv_ body (opaque-blob x multi-member); and the kind-registration surface (divine legend + RBTDRC_FIXTURES).
Prove it end-to-end on one family. Land the podvm RBSL subdoc + RBS0 quoin (spec covers BOTH families) and promote RBSPV out of FUTURE/.

## Done when
- immure captures one family end-to-end; a podvm-lifecycle service fixture green (immure -> divine cohort -> augur members + :rbi_vouch -> per-member jettison via the generic image backdoor -> whole-Lode banish -> absent), in service + complete suites; RBTDRC_FIXTURES registers it; divine legend carries the kind-letter.
- RBSL podvm subdoc + RBS0 quoin landed; RBSPV promoted out of FUTURE/.

## Cinched
- recorded-at-acquisition grade. Selective leaf capture, declarative intent (no FQIN); selection keys on the index descriptor platform + disktype, never the layer filename. Cloud-side only.
- Tool is gcrane via the floating bootstrap (ZRBLD_GCRANE_BUILDER) — podvm is vessel-less like wsl; pinning is the bootstrap-digest-pin itch, not this pace (paddock pinning boundary).
- Copy-shape: rbldw_Underpin.sh is the literal template for the body + spine registration; rbgjl03 (cohort selection loop) and rbgjl05 (gcrane wrap) for the step shape; the memo §7 build sheet is the recipe.
- Selected leaves are single-platform manifests (no index web), so banish stays flat-package safe by construction — keep it that way.
- RBSPV is authored in plain "crane" throughout — convert that prose to gcrane on promotion, and cite the existing RBSCB ambient-auth canon rather than re-deriving it.
  Do NOT resurrect the legacy FUTURE/rbv_PodmanVM.sh ignite-VM plain-crane design — cloud-side gcrane supersedes it entirely.
- Second family + curated multi-platform set + refresh mode = the FOLLOWING pace.

## Sources
- memo-20260608-lode-podvm-cerebro-experiment.md §7 (build sheet); RBSPV cerebro section; paddock Vocabulary + Kinds.

**[260609-0617] rough**

## Character
Net-new kind, base immure machinery — proves the podvm vertical on ONE family in an
already-crane world. The two-family fan-out + curated multi-platform selection is the
FOLLOWING pace.

## Goal
Stand up the podvm immure vertical (verb rbw-lI): a new in-pool step that gcrane-cp's
selected disk leaves BY DIGEST into rbi_ld/<vn|vw><stamp>, runs the blob-residency
guard, and reuses the gcrane vouch step; the new rbldv_ body (opaque-blob x
multi-member); and the kind-registration surface (divine legend + RBTDRC_FIXTURES).
Prove it end-to-end on one family. Land the podvm RBSL subdoc + RBS0 quoin (spec covers
BOTH families) and promote RBSPV out of FUTURE/.

## Done when
- immure captures one family end-to-end; a podvm-lifecycle service fixture green
  (immure -> divine cohort -> inspect members + :rbi_vouch -> per-member + whole-Lode
  banish -> absent), in service + complete suites; RBTDRC_FIXTURES registers it; divine
  legend carries the kind-letter.
- RBSL podvm subdoc + RBS0 quoin landed; RBSPV promoted out of FUTURE/.

## Cinched
- recorded-at-acquisition grade. Selective leaf capture, declarative intent (no FQIN);
  selection keys on the index descriptor platform + disktype, never the layer filename.
  Cloud-side only.
- Tool is gcrane (ZRBLD_GCRANE_BUILDER), not plain crane. RBSPV is authored in plain
  "crane" throughout — convert that prose to gcrane on promotion, and cite the existing
  RBSCB ambient-auth canon rather than re-deriving it. Do NOT resurrect the legacy
  FUTURE/rbv_PodmanVM.sh ignite-VM plain-crane design — cloud-side gcrane supersedes it
  entirely.
- Second family + curated multi-platform set + refresh mode = the FOLLOWING pace.

## Sources
- memo-20260608-lode-podvm-cerebro-experiment.md §7 (build sheet); RBSPV cerebro
  section; paddock Vocabulary + Kinds.

**[260608-2020] rough**

## Character
Net-new kind, base immure machinery — proves the podvm vertical on ONE family in an
already-crane world. The two-family fan-out + curated multi-platform selection is the FOLLOWING pace.

## Goal
Stand up the podvm immure vertical (verb rbw-lI): a new in-pool step that crane-cp's selected
disk leaves BY DIGEST into rbi_ld/<vn|vw><stamp>, runs the blob-residency guard, and reuses the
crane vouch step; the new rbldv_ body (opaque-blob x multi-member); and the kind-registration
surface (divine legend + RBTDRC_FIXTURES). Prove it end-to-end on one family. Land the podvm RBSL
subdoc + RBS0 quoin (spec covers BOTH families) and promote RBSPV out of FUTURE/ — deferred here
from the review pace.

## Done when
- immure captures one family end-to-end; a podvm-lifecycle service fixture green (immure -> divine
  cohort -> inspect members + :rbi_vouch -> per-member + whole-Lode banish -> absent), in service +
  complete suites; RBTDRC_FIXTURES registers it; divine legend carries the kind-letter.
- RBSL podvm subdoc + RBS0 quoin landed; RBSPV promoted out of FUTURE/.

## Cinched
- recorded-at-acquisition grade. Selective leaf capture, declarative intent (no FQIN); selection
  keys on the index descriptor platform + disktype, never the layer filename. Cloud-side only.
- Second family + curated multi-platform set + refresh mode = the FOLLOWING pace, not this one.

## Sources
- memo-20260608-lode-podvm-cerebro-experiment.md §7 (build sheet); RBSPV cerebro section; paddock Vocabulary + Kinds.

**[260608-1951] rough**

## Character
Net-new kind on the crane-uniform spine — the last Lode kind, completing universal capture.
Lands in an already-crane world, riding the proven crane builder + auth pattern.

## Goal
Capture podman-VM machine-os images via the immure verb (rbw-lI) — one verb spanning both
quay families (machine-os = vn, machine-os-wsl = vw) by archive argument. A new in-pool
immure step crane-cp's selected {disktype x arch} leaves BY DIGEST into rbi_ld/<vn|vw><stamp>,
runs the blob-residency guard, and reuses the (crane) vouch step. New rbldv_ body —
opaque-blob x multi-member. Carries the podvm RBSL subdoc + RBS0 quoin landing and the
RBSPV promotion out of FUTURE/ (deferred here from the review pace).

## Done when
- immure captures both families; podvm-lifecycle service fixture green end-to-end
  (immure -> divine cohort -> inspect members + :rbi_vouch -> per-member + whole-Lode banish
  -> absent), added to service + complete suites.
- divine legend carries vn/vw; the single-fixture registry (RBTDRC_FIXTURES) registers it.
- RBSL podvm subdoc + RBS0 quoin landed; RBSPV promoted out of FUTURE/.

## Cinched
- recorded-at-acquisition grade (quay publishes no durable checksum, rotates within days).
- Selective leaf capture (curated {disktype x arch}), declarative intent (no FQIN); selection
  keys on the index descriptor platform + disktype, never the layer filename.
- Cloud-side acquisition only; never touch the workstation to acquire bytes.

## Sources
- memo-20260608-lode-podvm-cerebro-experiment.md, the follow-up cloud pace section (§7) —
  full build sheet.
- RBSPV cerebro characterization section; paddock Vocabulary + Kinds + trust grades.

### lode-podvm-platform-fanout (₢BHAAL) [complete]

**[260610-1214] complete**

## Character
Greenfield follow-on to the podvm capture machinery — the two-family, multi-platform selection layer.
Tier: sonnet-delegable — extends machinery the preceding pace just built.

## Goal
Extend immure to span both quay families (machine-os-wsl and machine-os-native) via an archive argument, with curated per-platform variant selection.
The families share the gcrane capture path and differ only in quay repo + variant set.

## Cinched
- One verb spanning both families, not two verbs (paddock Kinds).
  Support an "expand selected set against the same upstream version" refresh mode, distinct from a version bump (paddock podvm-selective premise).
- Builds on the podvm capture machinery (preceding pace).

## Done when
- immure captures both families' curated sets into their respective Lodes; a fixture proves the fan-out AND the refresh-mode (expand-selected-set) path — extend the preceding pace's podvm-lifecycle fixture or add a fanout/refresh case; decide which at mount.

**[260609-1616] rough**

## Character
Greenfield follow-on to the podvm capture machinery — the two-family, multi-platform selection layer.
Tier: sonnet-delegable — extends machinery the preceding pace just built.

## Goal
Extend immure to span both quay families (machine-os-wsl and machine-os-native) via an archive argument, with curated per-platform variant selection.
The families share the gcrane capture path and differ only in quay repo + variant set.

## Cinched
- One verb spanning both families, not two verbs (paddock Kinds).
  Support an "expand selected set against the same upstream version" refresh mode, distinct from a version bump (paddock podvm-selective premise).
- Builds on the podvm capture machinery (preceding pace).

## Done when
- immure captures both families' curated sets into their respective Lodes; a fixture proves the fan-out AND the refresh-mode (expand-selected-set) path — extend the preceding pace's podvm-lifecycle fixture or add a fanout/refresh case; decide which at mount.

**[260609-0617] rough**

## Character
Greenfield follow-on to the podvm capture machinery — the two-family, multi-platform
selection layer.

## Goal
Extend immure to span both quay families (machine-os-wsl and machine-os-native) via an
archive argument, with curated per-platform variant selection. The families share the
gcrane capture path and differ only in quay repo + variant set.

## Cinched
- One verb spanning both families, not two verbs (paddock Kinds). Support an "expand
  selected set against the same upstream version" refresh mode, distinct from a version
  bump (paddock podvm-selective premise).
- Builds on the podvm capture machinery (preceding pace).

## Done when
- immure captures both families' curated sets into their respective Lodes; a fixture
  proves the fan-out AND the refresh-mode (expand-selected-set) path — extend the
  preceding pace's podvm-lifecycle fixture or add a fanout/refresh case; decide which at
  mount.

**[260606-1012] rough**

## Character
Greenfield follow-on to the podvm capture machinery — the two-family, multi-platform selection layer.

## Goal
Extend immure to span both quay families (machine-os-wsl and machine-os-native) via an archive argument, with curated per-platform variant selection. The families share the crane capture path and differ only in quay repo + variant set.

## Cinched
- One verb spanning both families, not two verbs (paddock Kinds). Support an "expand selected set against the same upstream version" refresh mode, distinct from a version bump (paddock podvm-selective premise).
- Builds on the podvm capture machinery (preceding pace).

## Done
immure captures both families' curated sets into their respective Lodes; a fixture proves the fan-out.

**[260605-1119] rough**

## Character
Greenfield follow-on to the podvm capture machinery — the two-family, multi-platform selection layer.

## Goal
Extend immure to span both quay families (machine-os-wsl and machine-os-native) via an archive argument, with curated per-platform variant selection. The families share the crane capture path and differ only in quay repo + variant set.

## Locked
- One verb spanning both families, not two verbs (paddock Kinds). Support an "expand selected set against the same upstream version" refresh mode, distinct from a version bump (paddock podvm-selective premise).
- Builds on the podvm capture machinery (preceding pace).

## Done
immure captures both families' curated sets into their respective Lodes; a fixture proves the fan-out.

### lode-reliquary-inscribe-cutover (₢BHAAM) [complete]

**[260610-1315] complete**

## Character
Serial cutover — sibling of the bole cutover.
Gated on the conclave vertical existing.
Now carries BOTH consumption-side elections — the reliquary yoke-stamp AND the folded-in bole derived-pull ANCHOR populator — since both land vessel config and the bole one was owned by no pace.
Tier: architectural — the driver works this directly; widest blast radius remaining in the heat.

## Goal
Retire the inscribe reliquary-mirror path and repoint RBRV_RELIQUARY onto a reliquary-kind (conclave) Lode, retiring the rbi_rq namespace and everything that addresses it.
Also land the folded-in bole election: build the conjure ANCHOR populator that derives RBRV_IMAGE_n_ANCHOR from the bole capture handoff, so a fresh vessel's base ANCHOR is populated by a landed mechanism now that enshrine — its former writer — is retired.

## Cinched
- Follow the paddock Cutover technique (rebuild-and-repoint, delete-old-last).
  The single repoint is the tool-resolution chokepoint zrbfc_resolve_tool_images (in rbfca_StepAssembly), which feeds conjure/about/vouch step assembly and bole's pinned gcrane.
- Using the sprue is critical — the repoint must adopt sprued member-tag addressing, never the legacy bare-name form.
  A conclave Lode names its cohort members as sprued `:rbi_<tool>` tags on one package; the repointed resolver must compose RBGC_LODE_TAG_SPRUE onto each RBGC_RELIQUARY_TOOL_* seed to address them, replacing the legacy rbi_rq bare-basename-per-package addressing.
  The bare RBGC_RELIQUARY_TOOL_* constants stay as seeds (inputs to the sprue); the resolved ref a build consumes is always the sprued `rbi_<tool>` member tag.
- The rbi_rq-addressed maintenance verbs retire with the namespace: rbw-iar (AuditsReliquaries), rbw-irr (RekonsReliquary), rbw-iwr (WrestsReliquaryImage), rbw-iJr (JettisonsReliquaryImage) and their backing paths in rbfln_Inventory/rbflw_Wrest/rbfcg_GarRest — the path-polymorphic image backdoor (landed earlier this heat) subsumes them.
- Spec blast radius rides the done: retire RBSDI-depot_inscribe.adoc (a live spec of the retired operation) and the rbtgo_depot_inscribe quoin; update RBSDY to sprued member-tag addressing and the derived cohort roster.
- Onboarding rides the done: flip the inscribe/reliquary handbook tracks + RBYC_RELIQUARY + the literal inscribe prose + the matching rbtdro fixture (prose is not test-guarded; the done is the only guard).
- rbrd_inscribe (depot tripwire, rbw-rdi) is a DIFFERENT operation — out of scope, leave it.
- Flag: inscribe mirrors :latest, so a fresh re-inscribe bumps the toolchain — copy old digests into rbi_ld if a byte-identical cohort is wanted.
- Verify gate: one conjure build green.

### Folded-in bole derived-pull ANCHOR election
- The bole capture body (rbldb_Bole.sh) hands its touchmark forward to "a later derived-pull election (the conjure ANCHOR populator)"; build that populator here.
- Mechanism is DERIVED-PULL, NOT yoke-stamp: the consumption side reads the resolved coordinate from the bole capture handoff fact and populates RBRV_IMAGE_n_ANCHOR — distinct from reliquary's explicit yoke-stamp.
  The paddock Touchmark-election distinction (derived-pull ANCHOR vs yoke-stamp RELIQUARY) must survive even though both elections land in this one cutover.
- Capture stays pure (ensconce writes no vessel config); the populator is the consumption-side reader.
- Honors the election rule: writes vessel config, operator-commits, never self-commits, does not self-gate on a dirty tree (the downstream consumer gates).
- The vessel-regime ANCHOR spec prose (RBSRV, RBS0 rbrv_image_anchor/rbrv_image_origin, and the rbrv_regime.sh enrollment strings) was left in a transitional "populator pending" state by the enshrine-spec retirement — finalize it here to the wired derived-pull state, dropping the last enshrines-namespace residue in favor of the bole Lode rbi_ld locator.

## Done when
inscribe retired; RBRV_RELIQUARY resolves from a conclave Lode through sprued `:rbi_<tool>` member-tag addressing; the four rbi_rq maintenance verbs gone; RBSDI retired + RBSDY updated + quoin gone, no cross-ref dangles; a fresh vessel's RBRV_IMAGE_n_ANCHOR is populated via the bole derived-pull election with no reliance on the retired enshrine writer; the vessel-regime ANCHOR spec prose finalized to the wired state; conjure green; rbi_rq banished last.

**[260610-0849] rough**

## Character
Serial cutover — sibling of the bole cutover.
Gated on the conclave vertical existing.
Now carries BOTH consumption-side elections — the reliquary yoke-stamp AND the folded-in bole derived-pull ANCHOR populator — since both land vessel config and the bole one was owned by no pace.
Tier: architectural — the driver works this directly; widest blast radius remaining in the heat.

## Goal
Retire the inscribe reliquary-mirror path and repoint RBRV_RELIQUARY onto a reliquary-kind (conclave) Lode, retiring the rbi_rq namespace and everything that addresses it.
Also land the folded-in bole election: build the conjure ANCHOR populator that derives RBRV_IMAGE_n_ANCHOR from the bole capture handoff, so a fresh vessel's base ANCHOR is populated by a landed mechanism now that enshrine — its former writer — is retired.

## Cinched
- Follow the paddock Cutover technique (rebuild-and-repoint, delete-old-last).
  The single repoint is the tool-resolution chokepoint zrbfc_resolve_tool_images (in rbfca_StepAssembly), which feeds conjure/about/vouch step assembly and bole's pinned gcrane.
- Using the sprue is critical — the repoint must adopt sprued member-tag addressing, never the legacy bare-name form.
  A conclave Lode names its cohort members as sprued `:rbi_<tool>` tags on one package; the repointed resolver must compose RBGC_LODE_TAG_SPRUE onto each RBGC_RELIQUARY_TOOL_* seed to address them, replacing the legacy rbi_rq bare-basename-per-package addressing.
  The bare RBGC_RELIQUARY_TOOL_* constants stay as seeds (inputs to the sprue); the resolved ref a build consumes is always the sprued `rbi_<tool>` member tag.
- The rbi_rq-addressed maintenance verbs retire with the namespace: rbw-iar (AuditsReliquaries), rbw-irr (RekonsReliquary), rbw-iwr (WrestsReliquaryImage), rbw-iJr (JettisonsReliquaryImage) and their backing paths in rbfln_Inventory/rbflw_Wrest/rbfcg_GarRest — the path-polymorphic image backdoor (landed earlier this heat) subsumes them.
- Spec blast radius rides the done: retire RBSDI-depot_inscribe.adoc (a live spec of the retired operation) and the rbtgo_depot_inscribe quoin; update RBSDY to sprued member-tag addressing and the derived cohort roster.
- Onboarding rides the done: flip the inscribe/reliquary handbook tracks + RBYC_RELIQUARY + the literal inscribe prose + the matching rbtdro fixture (prose is not test-guarded; the done is the only guard).
- rbrd_inscribe (depot tripwire, rbw-rdi) is a DIFFERENT operation — out of scope, leave it.
- Flag: inscribe mirrors :latest, so a fresh re-inscribe bumps the toolchain — copy old digests into rbi_ld if a byte-identical cohort is wanted.
- Verify gate: one conjure build green.

### Folded-in bole derived-pull ANCHOR election
- The bole capture body (rbldb_Bole.sh) hands its touchmark forward to "a later derived-pull election (the conjure ANCHOR populator)"; build that populator here.
- Mechanism is DERIVED-PULL, NOT yoke-stamp: the consumption side reads the resolved coordinate from the bole capture handoff fact and populates RBRV_IMAGE_n_ANCHOR — distinct from reliquary's explicit yoke-stamp.
  The paddock Touchmark-election distinction (derived-pull ANCHOR vs yoke-stamp RELIQUARY) must survive even though both elections land in this one cutover.
- Capture stays pure (ensconce writes no vessel config); the populator is the consumption-side reader.
- Honors the election rule: writes vessel config, operator-commits, never self-commits, does not self-gate on a dirty tree (the downstream consumer gates).
- The vessel-regime ANCHOR spec prose (RBSRV, RBS0 rbrv_image_anchor/rbrv_image_origin, and the rbrv_regime.sh enrollment strings) was left in a transitional "populator pending" state by the enshrine-spec retirement — finalize it here to the wired derived-pull state, dropping the last enshrines-namespace residue in favor of the bole Lode rbi_ld locator.

## Done when
inscribe retired; RBRV_RELIQUARY resolves from a conclave Lode through sprued `:rbi_<tool>` member-tag addressing; the four rbi_rq maintenance verbs gone; RBSDI retired + RBSDY updated + quoin gone, no cross-ref dangles; a fresh vessel's RBRV_IMAGE_n_ANCHOR is populated via the bole derived-pull election with no reliance on the retired enshrine writer; the vessel-regime ANCHOR spec prose finalized to the wired state; conjure green; rbi_rq banished last.

**[260609-1616] rough**

## Character
Serial cutover — sibling of the bole cutover.
Gated on the conclave vertical existing.
Tier: architectural — the driver works this directly; widest blast radius remaining in the heat.

## Goal
Retire the inscribe reliquary-mirror path and repoint RBRV_RELIQUARY onto a reliquary-kind (conclave) Lode, retiring the rbi_rq namespace and everything that addresses it.

## Cinched
- Follow the paddock Cutover technique (rebuild-and-repoint, delete-old-last).
  The single repoint is the tool-resolution chokepoint zrbfc_resolve_tool_images (in rbfca_StepAssembly), which feeds conjure/about/vouch step assembly and bole's pinned gcrane.
- Using the sprue is critical — the repoint must adopt sprued member-tag addressing, never the legacy bare-name form.
  A conclave Lode names its cohort members as sprued `:rbi_<tool>` tags on one package; the repointed resolver must compose RBGC_LODE_TAG_SPRUE onto each RBGC_RELIQUARY_TOOL_* seed to address them, replacing the legacy rbi_rq bare-basename-per-package addressing.
  The bare RBGC_RELIQUARY_TOOL_* constants stay as seeds (inputs to the sprue); the resolved ref a build consumes is always the sprued `rbi_<tool>` member tag.
- The rbi_rq-addressed maintenance verbs retire with the namespace: rbw-iar (AuditsReliquaries), rbw-irr (RekonsReliquary), rbw-iwr (WrestsReliquaryImage), rbw-iJr (JettisonsReliquaryImage) and their backing paths in rbfln_Inventory/rbflw_Wrest/rbfcg_GarRest — the path-polymorphic image backdoor (landed earlier this heat) subsumes them.
- Spec blast radius rides the done: retire RBSDI-depot_inscribe.adoc (a live spec of the retired operation) and the rbtgo_depot_inscribe quoin; update RBSDY to sprued member-tag addressing and the derived cohort roster.
- Onboarding rides the done: flip the inscribe/reliquary handbook tracks + RBYC_RELIQUARY + the literal inscribe prose + the matching rbtdro fixture (prose is not test-guarded; the done is the only guard).
- rbrd_inscribe (depot tripwire, rbw-rdi) is a DIFFERENT operation — out of scope, leave it.
- Flag: inscribe mirrors :latest, so a fresh re-inscribe bumps the toolchain — copy old digests into rbi_ld if a byte-identical cohort is wanted.
- Verify gate: one conjure build green.

## Done when
inscribe retired; RBRV_RELIQUARY resolves from a conclave Lode through sprued `:rbi_<tool>` member-tag addressing; the four rbi_rq maintenance verbs gone; RBSDI retired + RBSDY updated + quoin gone, no cross-ref dangles; conjure green; rbi_rq banished last.

**[260609-0847] rough**

## Character
Serial cutover — sibling of the bole cutover.
Gated on the conclave vertical existing.

## Goal
Retire the inscribe reliquary-mirror path and repoint RBRV_RELIQUARY onto a reliquary-kind (conclave) Lode.

## Cinched
- Follow the paddock Cutover technique (rebuild-and-repoint, delete-old-last).
  The single repoint is the tool-resolution chokepoint zrbfc_resolve_tool_images (in rbfca_StepAssembly), which feeds conjure/about/vouch/enshrine and bole ensconce.
- Using the sprue is critical — the repoint must adopt sprued member-tag addressing, never the legacy bare-name form.
  A conclave Lode names its cohort members as sprued `:rbi_<tool>` tags on one package; the repointed resolver must compose RBGC_LODE_TAG_SPRUE onto each RBGC_RELIQUARY_TOOL_* seed to address them, replacing the legacy rbi_rq bare-basename-per-package addressing.
  The bare RBGC_RELIQUARY_TOOL_* constants stay as seeds (inputs to the sprue); the resolved ref a build consumes is always the sprued `rbi_<tool>` member tag.
- Onboarding rides the done: flip the inscribe/reliquary handbook tracks + RBYC_RELIQUARY + the literal inscribe prose + the matching rbtdro fixture (prose is not test-guarded; the done is the only guard).
- rbrd_inscribe (depot tripwire, rbw-rdi) is a DIFFERENT operation — out of scope, leave it.
- Flag: inscribe mirrors :latest, so a fresh re-inscribe bumps the toolchain — copy old digests into rbi_ld if a byte-identical cohort is wanted.
- Verify gate: one conjure build green.

## Done when
inscribe retired; RBRV_RELIQUARY resolves from a conclave Lode through sprued `:rbi_<tool>` member-tag addressing; conjure green; rbi_rq banished last.

**[260606-1012] rough**

## Character
Serial cutover — sibling of the bole cutover. Gated on the conclave vertical existing.

## Goal
Retire the inscribe reliquary-mirror path and repoint RBRV_RELIQUARY onto a reliquary-kind (conclave) Lode.

## Cinched
- Follow the paddock Cutover technique (rebuild-and-repoint, delete-old-last). The single repoint is the tool-resolution chokepoint zrbfc_resolve_tool_images (in rbfca_StepAssembly), which feeds conjure/about/vouch/enshrine and bole ensconce.
- Onboarding rides the done: flip the inscribe/reliquary handbook tracks + RBYC_RELIQUARY + the literal inscribe prose + the matching rbtdro fixture (prose is not test-guarded; the done is the only guard).
- rbrd_inscribe (depot tripwire, rbw-rdi) is a DIFFERENT operation — out of scope, leave it.
- Flag: inscribe mirrors :latest, so a fresh re-inscribe bumps the toolchain — copy old digests into rbi_ld if a byte-identical cohort is wanted.
- Verify gate: one conjure build green.

## Done
inscribe retired, RBRV_RELIQUARY resolves from a conclave Lode, conjure green, rbi_rq banished last.

**[260605-1120] rough**

## Character
Serial cutover — sibling of the bole cutover. Gated on the conclave vertical existing.

## Goal
Retire the inscribe reliquary-mirror path and repoint RBRV_RELIQUARY onto a reliquary-kind (conclave) Lode.

## Locked
- Follow the paddock Cutover technique (rebuild-and-repoint, delete-old-last). The single repoint is the tool-resolution chokepoint zrbfc_resolve_tool_images (in rbfca_StepAssembly), which feeds conjure/about/vouch/enshrine and bole ensconce.
- Onboarding rides the done: flip the inscribe/reliquary handbook tracks + RBYC_RELIQUARY + the literal inscribe prose + the matching rbtdro fixture (prose is not test-guarded; the done is the only guard).
- rbrd_inscribe (depot tripwire, rbw-rdi) is a DIFFERENT operation — out of scope, leave it.
- Flag: inscribe mirrors :latest, so a fresh re-inscribe bumps the toolchain — copy old digests into rbi_ld if a byte-identical cohort is wanted.
- Verify gate: one conjure build green.

## Done
inscribe retired, RBRV_RELIQUARY resolves from a conclave Lode, conjure green, rbi_rq banished last.

### lode-public-docs-concept (₢BHAAO) [complete]

**[260610-1450] complete**

## Character
Serial — rides the reliquary cutover. The supply-chain NARRATIVE conversion pass; the additive Lode/Touchmark concept surface was banked early, off-chain.
Tier: architectural — public prose coherence; the driver writes this directly.

## Goal
Convert the README supply-chain narrative from Enshrine/Reliquary language to Lode language as one coherent rewrite, once builder-tool capture is also a Lode (conclave). Refresh the stale RBSHR "revised enshrine" frame to the per-kind-verb model. Confirm the early-banked Lode/Touchmark concept entries still read correctly against the landed system.

## Cinched
- ADD half DONE (commit ca625c13d): `### Lode` + `### Touchmark` concept entries in the Foundry section, canonical `<a id="Lode">` relocated there, churn appendix reseamed. This pace authors no concept entries.
- Convert as ONE unit, not base-first. Base capture already ships as a Lode (`rbw-lE`/ensconce; `rbw-dE` gone), so its prose is stale-not-forward — but "Enshrine" is a woven narrative term (Airgap explanation, Establishment, the GCB bullet, the Supply Chain glossary, and "the Enshrine ceremony" in the Build Isolation appendix). Converting only base while tools are still Reliquary fragments the narrative into a Lode/Enshrine/Reliquary mix — worse public prose than uniform-old now or uniform-Lode later. Gate on tools also being Lodes.
- Concept-level only — no per-kind verbs in README (the verb catalog lives in CLAUDE.md).
- RBSHR is a spec edit, not README.

## Discovery recipe
`grep -n '#Enshrine\|#Reliquary' README.md` → the conversion sites; re-grep at mount, the list shrinks as cutovers land.

## Done when
The README supply-chain narrative speaks of Lodes uniformly; RBSHR frame refreshed; banked concept entries verified against landed vocabulary; no Enshrine/Reliquary capture vocabulary survives in README beyond what the terminal vocabulary scrub sweeps.

**[260610-1131] rough**

## Character
Serial — rides the reliquary cutover. The supply-chain NARRATIVE conversion pass; the additive Lode/Touchmark concept surface was banked early, off-chain.
Tier: architectural — public prose coherence; the driver writes this directly.

## Goal
Convert the README supply-chain narrative from Enshrine/Reliquary language to Lode language as one coherent rewrite, once builder-tool capture is also a Lode (conclave). Refresh the stale RBSHR "revised enshrine" frame to the per-kind-verb model. Confirm the early-banked Lode/Touchmark concept entries still read correctly against the landed system.

## Cinched
- ADD half DONE (commit ca625c13d): `### Lode` + `### Touchmark` concept entries in the Foundry section, canonical `<a id="Lode">` relocated there, churn appendix reseamed. This pace authors no concept entries.
- Convert as ONE unit, not base-first. Base capture already ships as a Lode (`rbw-lE`/ensconce; `rbw-dE` gone), so its prose is stale-not-forward — but "Enshrine" is a woven narrative term (Airgap explanation, Establishment, the GCB bullet, the Supply Chain glossary, and "the Enshrine ceremony" in the Build Isolation appendix). Converting only base while tools are still Reliquary fragments the narrative into a Lode/Enshrine/Reliquary mix — worse public prose than uniform-old now or uniform-Lode later. Gate on tools also being Lodes.
- Concept-level only — no per-kind verbs in README (the verb catalog lives in CLAUDE.md).
- RBSHR is a spec edit, not README.

## Discovery recipe
`grep -n '#Enshrine\|#Reliquary' README.md` → the conversion sites; re-grep at mount, the list shrinks as cutovers land.

## Done when
The README supply-chain narrative speaks of Lodes uniformly; RBSHR frame refreshed; banked concept entries verified against landed vocabulary; no Enshrine/Reliquary capture vocabulary survives in README beyond what the terminal vocabulary scrub sweeps.

**[260609-1616] rough**

## Character
Serial — rides the reliquary cutover. The supply-chain NARRATIVE conversion pass; the additive Lode/Touchmark concept surface was banked early, off-chain.
Tier: architectural — public prose coherence; the driver writes this directly.

## Goal
Convert the README supply-chain narrative from Enshrine/Reliquary language to Lode language as one coherent rewrite, once builder-tool capture is also a Lode (conclave). Refresh the stale RBSHR "revised enshrine" frame to the per-kind-verb model. Confirm the early-banked Lode/Touchmark concept entries still read correctly against the landed system.

## Cinched
- ADD half DONE (commit ca625c13d): `### Lode` + `### Touchmark` concept entries in the Foundry section, canonical `<a id="Lode">` relocated there, churn appendix reseamed. This pace authors no concept entries.
- Convert as ONE unit, not base-first. Base capture already ships as a Lode (`rbw-lE`/ensconce; `rbw-dE` gone), so its prose is stale-not-forward — but "Enshrine" is a woven narrative term (Airgap explanation, Establishment, the GCB bullet, the Supply Chain glossary, and "the Enshrine ceremony" in the Build Isolation appendix). Converting only base while tools are still Reliquary fragments the narrative into a Lode/Enshrine/Reliquary mix — worse public prose than uniform-old now or uniform-Lode later. Gate on tools also being Lodes.
- Concept-level only — no per-kind verbs in README (the verb catalog lives in CLAUDE.md).
- RBSHR is a spec edit, not README.

## Discovery recipe
`grep -n '#Enshrine\|#Reliquary' README.md` → the conversion sites; re-grep at mount, the list shrinks as cutovers land.

## Done
The README supply-chain narrative speaks of Lodes uniformly; RBSHR frame refreshed; banked concept entries verified against landed vocabulary; no Enshrine/Reliquary capture vocabulary survives in README beyond what the terminal vocabulary scrub sweeps.

**[260608-1450] rough**

## Character
Serial — rides the reliquary cutover. The supply-chain NARRATIVE conversion pass; the additive Lode/Touchmark concept surface was banked early, off-chain.

## Goal
Convert the README supply-chain narrative from Enshrine/Reliquary language to Lode language as one coherent rewrite, once builder-tool capture is also a Lode (conclave). Refresh the stale RBSHR "revised enshrine" frame to the per-kind-verb model. Confirm the early-banked Lode/Touchmark concept entries still read correctly against the landed system.

## Cinched
- ADD half DONE (commit ca625c13d): `### Lode` + `### Touchmark` concept entries in the Foundry section, canonical `<a id="Lode">` relocated there, churn appendix reseamed. This pace authors no concept entries.
- Convert as ONE unit, not base-first. Base capture already ships as a Lode (`rbw-lE`/ensconce; `rbw-dE` gone), so its prose is stale-not-forward — but "Enshrine" is a woven narrative term (Airgap explanation, Establishment, the GCB bullet, the Supply Chain glossary, and "the Enshrine ceremony" in the Build Isolation appendix). Converting only base while tools are still Reliquary fragments the narrative into a Lode/Enshrine/Reliquary mix — worse public prose than uniform-old now or uniform-Lode later. Gate on tools also being Lodes.
- Concept-level only — no per-kind verbs in README (the verb catalog lives in CLAUDE.md).
- RBSHR is a spec edit, not README.

## Discovery recipe
`grep -n '#Enshrine\|#Reliquary' README.md` → the conversion sites; re-grep at mount, the list shrinks as cutovers land.

## Done
The README supply-chain narrative speaks of Lodes uniformly; RBSHR frame refreshed; banked concept entries verified against landed vocabulary; no Enshrine/Reliquary capture vocabulary survives in README beyond what the terminal vocabulary scrub sweeps.

**[260606-1012] rough**

## Character
Serial — rides the cutovers. The public concept surface, not a verb catalog.

## Goal
Surface Lode and touchmark in the public README as concept-level glossary entries (parallel to Vessel/Hallmark), retire the public Enshrine/Reliquary entries, and refresh the stale RBSHR "revised enshrine" frame to the per-kind-verb model.

## Cinched
- Concept-level only — do NOT enumerate per-kind verbs in README. The README delegates the command/verb catalog to CLAUDE.md; concept-level entries are kind-invariant, so the public surface stabilizes once and no vertical reopens it.
- Meaningful only after the cutovers land (else README would describe two parallel capture systems).

## Done
README glossary reflects Lode/touchmark at concept level, superseded public capture entries retired, RBSHR frame refreshed.

**[260605-1121] rough**

## Character
Serial — rides the cutovers. The public concept surface, not a verb catalog.

## Goal
Surface Lode and touchmark in the public README as concept-level glossary entries (parallel to Vessel/Hallmark), retire the public Enshrine/Reliquary entries, and refresh the stale RBSHR "revised enshrine" frame to the per-kind-verb model.

## Locked
- Concept-level only — do NOT enumerate per-kind verbs in README. The README delegates the command/verb catalog to CLAUDE.md; concept-level entries are kind-invariant, so the public surface stabilizes once and no vertical reopens it.
- Meaningful only after the cutovers land (else README would describe two parallel capture systems).

## Done
README glossary reflects Lode/touchmark at concept level, superseded public capture entries retired, RBSHR frame refreshed.

### lode-housekeeping-deferrals (₢BHAAP) [complete]

**[260610-1838] complete**

## Character
Small housekeeping — mark deferred surfaces; settle one cross-heat revert.
Side-lane pace: file-disjoint from the spine, mountable in a second officium during spine cloud waits (paddock Execution posture).
Tier: haiku-capable body — the jjx drop/transfer is the driver's.

## Goal
Mark the stale Windows onboarding tracks (rbhw*) loudly NOT-available/deferred rather than writing them, and resolve the heat ₣A- WSL-stage DEV CACHE revert pace now that wsl-kind work is committed to.

## Cinched
- Onboarding follows consumption, not acquisition: substrate-kind (wsl/podvm) host consumption is out of scope this heat, so mark-deferred — do not write the tracks. Resist a unified Lode-onboarding track; onboarding stays distributed by consumer (paddock Carried-forward).
- The DEV-CACHE revert pace lives in heat ₣A- — drop or transfer it (operator's choice at mount).

## Done when
Stale Windows tracks marked deferred; the ₣A- WSL-stage DEV CACHE revert pace dropped or transferred.

**[260610-1554] rough**

## Character
Small housekeeping — mark deferred surfaces; settle one cross-heat revert.
Side-lane pace: file-disjoint from the spine, mountable in a second officium during spine cloud waits (paddock Execution posture).
Tier: haiku-capable body — the jjx drop/transfer is the driver's.

## Goal
Mark the stale Windows onboarding tracks (rbhw*) loudly NOT-available/deferred rather than writing them, and resolve the heat ₣A- WSL-stage DEV CACHE revert pace now that wsl-kind work is committed to.

## Cinched
- Onboarding follows consumption, not acquisition: substrate-kind (wsl/podvm) host consumption is out of scope this heat, so mark-deferred — do not write the tracks. Resist a unified Lode-onboarding track; onboarding stays distributed by consumer (paddock Carried-forward).
- The DEV-CACHE revert pace lives in heat ₣A- — drop or transfer it (operator's choice at mount).

## Done when
Stale Windows tracks marked deferred; the ₣A- WSL-stage DEV CACHE revert pace dropped or transferred.

**[260609-1616] rough**

## Character
Small housekeeping — mark deferred surfaces; settle one cross-heat revert.
Side-lane pace: file-disjoint from the spine, mountable in a second officium during spine cloud waits (paddock Execution posture).
Tier: haiku-capable body — the jjx drop/transfer is the driver's.

## Goal
Mark the stale Windows onboarding tracks (rbhw*) loudly NOT-available/deferred rather than writing them, and resolve the heat ₣A- WSL-stage DEV CACHE revert pace now that wsl-kind work is committed to.

## Cinched
- Onboarding follows consumption, not acquisition: substrate-kind (wsl/podvm) host consumption is out of scope this heat, so mark-deferred — do not write the tracks. Resist a unified Lode-onboarding track; onboarding stays distributed by consumer (paddock Carried-forward).
- The DEV-CACHE revert pace lives in heat ₣A- — drop or transfer it (operator's choice at mount).

## Done
Stale Windows tracks marked deferred; the ₣A- WSL-stage DEV CACHE revert pace dropped or transferred.

**[260606-1012] rough**

## Character
Small housekeeping — mark deferred surfaces; settle one cross-heat revert.

## Goal
Mark the stale Windows onboarding tracks (rbhw*) loudly NOT-available/deferred rather than writing them, and resolve the heat ₣A- WSL-stage DEV CACHE revert pace now that wsl-kind work is committed to.

## Cinched
- Onboarding follows consumption, not acquisition: substrate-kind (wsl/podvm) host consumption is out of scope this heat, so mark-deferred — do not write the tracks. Resist a unified Lode-onboarding track; onboarding stays distributed by consumer (paddock Carried-forward).
- The DEV-CACHE revert pace lives in heat ₣A- — drop or transfer it (operator's choice at mount).

## Done
Stale Windows tracks marked deferred; the ₣A- WSL-stage DEV CACHE revert pace dropped or transferred.

**[260605-1121] rough**

## Character
Small housekeeping — mark deferred surfaces; settle one cross-heat revert.

## Goal
Mark the stale Windows onboarding tracks (rbhw*) loudly NOT-available/deferred rather than writing them, and resolve the heat ₣A- WSL-stage DEV CACHE revert pace now that wsl-kind work is committed to.

## Locked
- Onboarding follows consumption, not acquisition: substrate-kind (wsl/podvm) host consumption is out of scope this heat, so mark-deferred — do not write the tracks. Resist a unified Lode-onboarding track; onboarding stays distributed by consumer (paddock Carried-forward).
- The DEV-CACHE revert pace lives in heat ₣A- — drop or transfer it (operator's choice at mount).

## Done
Stale Windows tracks marked deferred; the ₣A- WSL-stage DEV CACHE revert pace dropped or transferred.

### lode-skopeo-reliquary-eviction (₢BHAAX) [complete]

**[260610-2125] complete**

## Character
The terminal skopeo removal — converts the last skopeo consumer (the made-side bind mirror) to gcrane, then purges skopeo from the reliquary cohort AND its test/spec surface.
A narrow, deliberate crossing of the capture-only line: ONE made-side invocation, NOT the made-image package retrofit.
Runs after every other skopeo consumer is gone.
Tier: sonnet-delegable — the hidden scope below is exhaustively enumerated; the verify delegation is settled below.

## Goal
Convert the bind mirror (rbgjm01 / rbfd_mirror / zrbfd_mirror_submit) from `skopeo copy --all` to `gcrane cp` — a bare registry-to-registry multi-platform copy, get-by-digest-or-error matching bind's pin-exactly semantic.
Then drop skopeo from the cohort and everything that asserts its presence.

## Done when
- No skopeo invocation in any Cloud Build step, capture or made-side; build + shellcheck + fast + service green.
- The bind-mode live verify DELEGATES to the heat's terminal endgame-verification pace
  (skirmish on the standing depot — operator-cinched: skirmish, not gauntlet; the deferral is structural, not ambient).
  This pace wraps without it; the delegation is the record.
- Dropping skopeo from the cohort MANIFEST takes live effect only through a fresh conclave + yoke —
  run both here so the post-eviction cohort is what the endgame skirmish meets.
- skopeo gone from the cohort: the conclave MANIFEST entry, z_rbfc_tool_skopeo plumbing (rbfc), the RBGC_RELIQUARY_TOOL_SKOPEO constant, the reliquary preflight check, and help/count strings (counts in rbfly_Yoke.sh are now derived — verify they track).
  The cohort provisions gcrane where build steps need it.

## Hidden scope this pace MUST also close (eviction consequences, not vocab)
A skopeo-grep surfaces these but mis-classifies them — they are must-fix:
- reliquary-lifecycle fixture asserts the conclave cohort CONTAINS rbi_skopeo (RBTDRC_RELIQUARY_TAG_SKOPEO in rbtdrc_crucible.rs) — it goes RED when the cohort drops skopeo. Update the fixture + remove the const.
- The cupel supply-chain conformance allowlist (ZRBTDRU_GCB_ALLOWED in rbtdru_cupel.rs) still permits "skopeo" — remove it, else the no-skopeo check is dead-permissive.
- rbgjs-token-fetch.sh has no remaining #@rbgjs_include caller once the mirror's inline fetch lands — delete the dead snippet (and its CBG snippet-table row).
- Live posture claims in RBSCB that anticipate full eviction go false — update them (keep the rejected-credential-helper memo + reference links as history).
- The bind-track operator handbook (rbhodb_director_bind.sh) narrates "Cloud Build runs skopeo to copy" — update the narrative.
These sites are a floor, not the census: per the partial-sweep drift lesson
(the rbls_ sprue sweep missed one of three parallel extract copies and broke the service suite hours later),
finish with a full `grep -rn skopeo Tools/ tt/` classification against the vocab scrub's KEEP guards —
never declare done from this list alone.

## Cinched
- The upstream gates have landed (capture skopeo eviction; inscribe retirement) — only this pace's own mirror conversion remains before the drop.
- gcrane (ZRBLD_GCRANE_BUILDER), not plain crane. Made-image package retrofit stays out of scope — this converts a tool invocation, not the made-side model.
- The inscribe cutover has landed: remove the z_rbfc_tool_skopeo line against the repointed resolver in rbfca_StepAssembly.sh as it now stands.

## Sources
- rbgjm01-mirror-image.sh, rbfd_FoundryDirectorBuild.sh (zrbfd_mirror_submit, z_rbfc_tool_skopeo). grep skopeo across Tools/rbk + tt/ for the cohort/plumbing/test sites.

**[260610-1605] rough**

## Character
The terminal skopeo removal — converts the last skopeo consumer (the made-side bind mirror) to gcrane, then purges skopeo from the reliquary cohort AND its test/spec surface.
A narrow, deliberate crossing of the capture-only line: ONE made-side invocation, NOT the made-image package retrofit.
Runs after every other skopeo consumer is gone.
Tier: sonnet-delegable — the hidden scope below is exhaustively enumerated; the verify delegation is settled below.

## Goal
Convert the bind mirror (rbgjm01 / rbfd_mirror / zrbfd_mirror_submit) from `skopeo copy --all` to `gcrane cp` — a bare registry-to-registry multi-platform copy, get-by-digest-or-error matching bind's pin-exactly semantic.
Then drop skopeo from the cohort and everything that asserts its presence.

## Done when
- No skopeo invocation in any Cloud Build step, capture or made-side; build + shellcheck + fast + service green.
- The bind-mode live verify DELEGATES to the heat's terminal endgame-verification pace
  (skirmish on the standing depot — operator-cinched: skirmish, not gauntlet; the deferral is structural, not ambient).
  This pace wraps without it; the delegation is the record.
- Dropping skopeo from the cohort MANIFEST takes live effect only through a fresh conclave + yoke —
  run both here so the post-eviction cohort is what the endgame skirmish meets.
- skopeo gone from the cohort: the conclave MANIFEST entry, z_rbfc_tool_skopeo plumbing (rbfc), the RBGC_RELIQUARY_TOOL_SKOPEO constant, the reliquary preflight check, and help/count strings (counts in rbfly_Yoke.sh are now derived — verify they track).
  The cohort provisions gcrane where build steps need it.

## Hidden scope this pace MUST also close (eviction consequences, not vocab)
A skopeo-grep surfaces these but mis-classifies them — they are must-fix:
- reliquary-lifecycle fixture asserts the conclave cohort CONTAINS rbi_skopeo (RBTDRC_RELIQUARY_TAG_SKOPEO in rbtdrc_crucible.rs) — it goes RED when the cohort drops skopeo. Update the fixture + remove the const.
- The cupel supply-chain conformance allowlist (ZRBTDRU_GCB_ALLOWED in rbtdru_cupel.rs) still permits "skopeo" — remove it, else the no-skopeo check is dead-permissive.
- rbgjs-token-fetch.sh has no remaining #@rbgjs_include caller once the mirror's inline fetch lands — delete the dead snippet (and its CBG snippet-table row).
- Live posture claims in RBSCB that anticipate full eviction go false — update them (keep the rejected-credential-helper memo + reference links as history).
- The bind-track operator handbook (rbhodb_director_bind.sh) narrates "Cloud Build runs skopeo to copy" — update the narrative.
These sites are a floor, not the census: per the partial-sweep drift lesson
(the rbls_ sprue sweep missed one of three parallel extract copies and broke the service suite hours later),
finish with a full `grep -rn skopeo Tools/ tt/` classification against the vocab scrub's KEEP guards —
never declare done from this list alone.

## Cinched
- The upstream gates have landed (capture skopeo eviction; inscribe retirement) — only this pace's own mirror conversion remains before the drop.
- gcrane (ZRBLD_GCRANE_BUILDER), not plain crane. Made-image package retrofit stays out of scope — this converts a tool invocation, not the made-side model.
- The inscribe cutover has landed: remove the z_rbfc_tool_skopeo line against the repointed resolver in rbfca_StepAssembly.sh as it now stands.

## Sources
- rbgjm01-mirror-image.sh, rbfd_FoundryDirectorBuild.sh (zrbfd_mirror_submit, z_rbfc_tool_skopeo). grep skopeo across Tools/rbk + tt/ for the cohort/plumbing/test sites.

**[260610-1554] rough**

## Character
The terminal skopeo removal — converts the last skopeo consumer (the made-side bind mirror) to gcrane, then purges skopeo from the reliquary cohort AND its test/spec surface.
A narrow, deliberate crossing of the capture-only line: ONE made-side invocation, NOT the made-image package retrofit.
Runs after every other skopeo consumer is gone.
Tier: sonnet-delegable — the hidden scope below is exhaustively enumerated; the gauntlet-tier verify is the driver's call.

## Goal
Convert the bind mirror (rbgjm01 / rbfd_mirror / zrbfd_mirror_submit) from `skopeo copy --all` to `gcrane cp` — a bare registry-to-registry multi-platform copy, get-by-digest-or-error matching bind's pin-exactly semantic.
Then drop skopeo from the cohort and everything that asserts its presence.

## Done when
- No skopeo invocation in any Cloud Build step, capture or made-side; a bind-mode build runs green on gcrane.
- Dropping skopeo from the cohort MANIFEST takes live effect only through a fresh conclave + yoke —
  run both before the bind-mode verify, so the preflight checks the post-eviction cohort.
- skopeo gone from the cohort: the conclave MANIFEST entry, z_rbfc_tool_skopeo plumbing (rbfc), the RBGC_RELIQUARY_TOOL_SKOPEO constant, the reliquary preflight check, and help/count strings (counts in rbfly_Yoke.sh are now derived — verify they track).
  The cohort provisions gcrane where build steps need it.

## Hidden scope this pace MUST also close (eviction consequences, not vocab)
A skopeo-grep surfaces these but mis-classifies them — they are must-fix:
- reliquary-lifecycle fixture asserts the conclave cohort CONTAINS rbi_skopeo (RBTDRC_RELIQUARY_TAG_SKOPEO in rbtdrc_crucible.rs) — it goes RED when the cohort drops skopeo. Update the fixture + remove the const.
- The cupel supply-chain conformance allowlist (ZRBTDRU_GCB_ALLOWED in rbtdru_cupel.rs) still permits "skopeo" — remove it, else the no-skopeo check is dead-permissive.
- rbgjs-token-fetch.sh has no remaining #@rbgjs_include caller once the mirror's inline fetch lands — delete the dead snippet (and its CBG snippet-table row).
- Live posture claims in RBSCB that anticipate full eviction go false — update them (keep the rejected-credential-helper memo + reference links as history).
- The bind-track operator handbook (rbhodb_director_bind.sh) narrates "Cloud Build runs skopeo to copy" — update the narrative.
These sites are a floor, not the census: per the partial-sweep drift lesson
(the rbls_ sprue sweep missed one of three parallel extract copies and broke the service suite hours later),
finish with a full `grep -rn skopeo Tools/ tt/` classification against the vocab scrub's KEEP guards —
never declare done from this list alone.

## Cinched
- The upstream gates have landed (capture skopeo eviction; inscribe retirement) — only this pace's own mirror conversion remains before the drop.
- gcrane (ZRBLD_GCRANE_BUILDER), not plain crane. Made-image package retrofit stays out of scope — this converts a tool invocation, not the made-side model.
- The inscribe cutover has landed: remove the z_rbfc_tool_skopeo line against the repointed resolver in rbfca_StepAssembly.sh as it now stands.

## Verify-cost note
The "bind-mode build runs green" gate is an onboarding-sequence case that runs only in the gauntlet/skirmish cloud-spending ladders, NOT the cheap service tier. Wrap criteria must budget for that.

## Sources
- rbgjm01-mirror-image.sh, rbfd_FoundryDirectorBuild.sh (zrbfd_mirror_submit, z_rbfc_tool_skopeo). grep skopeo across Tools/rbk + tt/ for the cohort/plumbing/test sites.

**[260609-1616] rough**

## Character
The terminal skopeo removal — converts the last skopeo consumer (the made-side bind mirror) to gcrane, then purges skopeo from the reliquary cohort AND its test/spec surface.
A narrow, deliberate crossing of the capture-only line: ONE made-side invocation, NOT the made-image package retrofit.
Runs after every other skopeo consumer is gone.
Tier: sonnet-delegable — the hidden scope below is exhaustively enumerated; the gauntlet-tier verify is the driver's call.

## Goal
Convert the bind mirror (rbgjm01 / rbfd_mirror / zrbfd_mirror_submit) from `skopeo copy --all` to `gcrane cp` — a bare registry-to-registry multi-platform copy, get-by-digest-or-error matching bind's pin-exactly semantic.
Then drop skopeo from the cohort and everything that asserts its presence.

## Done when
- No skopeo invocation in any Cloud Build step, capture or made-side; a bind-mode build runs green on gcrane.
- skopeo gone from the cohort: the conclave MANIFEST entry, z_rbfc_tool_skopeo plumbing (rbfc), the RBGC_RELIQUARY_TOOL_SKOPEO constant, the reliquary preflight check, and help/count strings (counts in rbfly_Yoke.sh are now derived — verify they track).
  The cohort provisions gcrane where build steps need it.

## Hidden scope this pace MUST also close (eviction consequences, not vocab)
A skopeo-grep surfaces these but mis-classifies them — they are must-fix:
- reliquary-lifecycle fixture asserts the conclave cohort CONTAINS rbi_skopeo (RBTDRC_RELIQUARY_TAG_SKOPEO in rbtdrc_crucible.rs) — it goes RED when the cohort drops skopeo. Update the fixture + remove the const.
- The cupel supply-chain conformance allowlist (ZRBTDRU_GCB_ALLOWED in rbtdru_cupel.rs) still permits "skopeo" — remove it, else the no-skopeo check is dead-permissive.
- rbgjs-token-fetch.sh has no remaining #@rbgjs_include caller once the mirror's inline fetch lands — delete the dead snippet (and its CBG snippet-table row).
- Live posture claims in RBSCB that anticipate full eviction go false — update them (keep the rejected-credential-helper memo + reference links as history).
- The bind-track operator handbook (rbhodb_director_bind.sh) narrates "Cloud Build runs skopeo to copy" — update the narrative.

## Cinched
- Gated on the capture skopeo eviction + inscribe retirement + this pace's own mirror conversion all landing first — every consumer gone before the tool is dropped.
- gcrane (ZRBLD_GCRANE_BUILDER), not plain crane. Made-image package retrofit stays out of scope — this converts a tool invocation, not the made-side model.
- Shares rbfca_StepAssembly.sh with the inscribe-cutover pace (that one repoints the resolver, this one removes the z_rbfc_tool_skopeo line) — whichever lands second reconciles the merge.

## Verify-cost note
The "bind-mode build runs green" gate is an onboarding-sequence case that runs only in the gauntlet/skirmish cloud-spending ladders, NOT the cheap service tier. Wrap criteria must budget for that.

## Sources
- rbgjm01-mirror-image.sh, rbfd_FoundryDirectorBuild.sh (zrbfd_mirror_submit, z_rbfc_tool_skopeo). grep skopeo across Tools/rbk + tt/ for the cohort/plumbing/test sites.

**[260609-0617] rough**

## Character
The terminal skopeo removal — converts the last skopeo consumer (the made-side bind
mirror) to gcrane, then purges skopeo from the reliquary cohort AND its test/spec
surface. A narrow, deliberate crossing of the capture-only line: ONE made-side
invocation, NOT the made-image package retrofit (Ark/Hallmark/abjure stay deferred).
Runs after every other skopeo consumer is gone.

## Goal
Convert the bind mirror (rbgjm01 / rbfd_mirror / zrbfd_mirror_submit) from
`skopeo copy --all` to `gcrane cp` — a bare registry-to-registry multi-platform copy,
get-by-digest-or-error matching bind's pin-exactly semantic. Then drop skopeo from the
cohort and everything that asserts its presence.

## Done when
- No skopeo invocation in any Cloud Build step, capture or made-side; a bind-mode build
  runs green on gcrane.
- skopeo gone from the cohort: the conclave MANIFEST entry, z_rbfc_tool_skopeo plumbing
  (rbfc), the RBGC_RELIQUARY_TOOL_SKOPEO constant, the reliquary preflight check, and
  help/count strings (the "all N tool images present" message in rbfly_Yoke.sh drops a
  count). The cohort provisions gcrane where build steps need it.

## Hidden scope this pace MUST also close (eviction consequences, not vocab)
A skopeo-grep surfaces these but mis-classifies them — they are must-fix:
- reliquary-lifecycle fixture asserts the conclave cohort CONTAINS rbi_skopeo
  (RBTDRC_RELIQUARY_TAG_SKOPEO in rbtdrc_crucible.rs) — it goes RED when the cohort drops
  skopeo. Update the fixture + remove the const.
- The cupel supply-chain conformance allowlist (ZRBTDRU_GCB_ALLOWED in rbtdru_cupel.rs)
  still permits "skopeo" — remove it, else the no-skopeo check is dead-permissive.
- rbgjs-token-fetch.sh has no remaining #@rbgjs_include caller once the mirror's inline
  fetch lands — delete the dead snippet.
- Live posture claims in RBSCB that anticipate full eviction go false — update them (keep
  the rejected-credential-helper memo + reference links as history).
- The bind-track operator handbook (rbhodb_director_bind.sh) narrates "Cloud Build runs
  skopeo to copy" — update the narrative.

## Cinched
- Gated on the capture skopeo eviction + inscribe retirement + this pace's own mirror
  conversion all landing first — every consumer gone before the tool is dropped.
- gcrane (ZRBLD_GCRANE_BUILDER), not plain crane. Made-image package retrofit stays out
  of scope — this converts a tool invocation, not the made-side model.
- Shares rbfca_StepAssembly.sh with the inscribe-cutover pace (that one repoints the
  resolver, this one removes the z_rbfc_tool_skopeo line) — whichever lands second
  reconciles the merge.

## Verify-cost note
The "bind-mode build runs green" gate is an onboarding-sequence case that runs only in
the gauntlet/skirmish cloud-spending ladders, NOT the cheap service tier. Wrap criteria
must budget for that.

## Sources
- rbgjm01-mirror-image.sh, rbfd_FoundryDirectorBuild.sh (zrbfd_mirror_submit,
  z_rbfc_tool_skopeo). grep skopeo across Tools/rbk + tt/ for the cohort/plumbing/test
  sites.

**[260608-2011] rough**

## Character
The terminal skopeo removal — converts the last skopeo consumer (the made-side bind
mirror) to crane, then purges skopeo from the reliquary cohort. A narrow, deliberate
crossing of the capture-only line: ONE made-side invocation, NOT the made-image package
retrofit (Ark/Hallmark/abjure stay deferred). Runs after every other skopeo consumer is gone.

## Goal
Convert the bind mirror (rbgjm01 / rbfd_mirror) from `skopeo copy --all` to `crane cp` —
a bare registry-to-registry multi-platform copy, get-by-digest-or-error matching bind's
pin-exactly semantic. Then drop skopeo from the cohort: the conclave MANIFEST tool entry,
the z_rbfc_tool_skopeo plumbing, the RBGC constant, the reliquary preflight check, and the
help-string mentions.

## Done when
- No skopeo invocation in any Cloud Build step, capture or made-side; a bind-mode build
  runs green on crane.
- skopeo gone from the cohort (MANIFEST, plumbing, constant, preflight, help); the cohort
  provisions crane where build steps need it.

## Cinched
- Gated on the capture skopeo eviction + inscribe retirement + this pace's own mirror
  conversion all landing first — every consumer gone before the tool is dropped.
- crane settled; reuse the chosen auth mechanism. Made-image package retrofit stays out of
  scope — this converts a tool invocation, not the made-side model.

## Sources
- rbgjm01-mirror-image.sh, rbfd_FoundryDirectorBuild.sh (zrbfd_mirror_submit,
  z_rbfc_tool_skopeo). grep skopeo across Tools/rbk + tt/ for the cohort/plumbing sites.

### suite-subset-and-test-table-repair (₢BHAAg) [complete]

**[260610-2158] complete**

## Character
Mechanical registry edit; membership is compile-checked.
Tier: sonnet-delegable.
Must land before the endgame ladder runs — the overnight skirmish should run the corrected subset.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
This pace runs in parallel-lane Chat 1; other lanes are editing disjoint files concurrently.
- Code, shellcheck, and the theurge build (`tt/rbw-tb.Build.sh`) are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — fixture runs serialize globally;
  one consolidated build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap —
  wrap sweeps the whole tree; the operator triggers each wrap after the central gate is green.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: report ready, then proceed directly to ₢BHAAm in this same session (same crate).

## Goal
Repair the hand-copied fast subset embedded in the gauntlet and skirmish suites,
and refresh the stale CLAUDE.md test-execution table.

## Done when
- gauntlet and skirmish in RBTDRC_SUITES (rbtdrc_crucible.rs) carry the full fast-fixture set
  (both currently miss handbook-render, foundry-path, recipe-validation).
- The CLAUDE.md test-execution table matches the registry composition
  and names the five release/probe suites it currently omits.
- Build green; fast green via the central consolidated gate.

## Sources
Memos/memo-20260610-heat-BH-lode-operation-durations.md (suite registry inventory section).

**[260610-2132] rough**

## Character
Mechanical registry edit; membership is compile-checked.
Tier: sonnet-delegable.
Must land before the endgame ladder runs — the overnight skirmish should run the corrected subset.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
This pace runs in parallel-lane Chat 1; other lanes are editing disjoint files concurrently.
- Code, shellcheck, and the theurge build (`tt/rbw-tb.Build.sh`) are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — fixture runs serialize globally;
  one consolidated build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap —
  wrap sweeps the whole tree; the operator triggers each wrap after the central gate is green.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: report ready, then proceed directly to ₢BHAAm in this same session (same crate).

## Goal
Repair the hand-copied fast subset embedded in the gauntlet and skirmish suites,
and refresh the stale CLAUDE.md test-execution table.

## Done when
- gauntlet and skirmish in RBTDRC_SUITES (rbtdrc_crucible.rs) carry the full fast-fixture set
  (both currently miss handbook-render, foundry-path, recipe-validation).
- The CLAUDE.md test-execution table matches the registry composition
  and names the five release/probe suites it currently omits.
- Build green; fast green via the central consolidated gate.

## Sources
Memos/memo-20260610-heat-BH-lode-operation-durations.md (suite registry inventory section).

**[260610-2114] rough**

## Character
Mechanical registry edit; membership is compile-checked.
Tier: sonnet-delegable.
Must land before the endgame ladder runs — the overnight skirmish should run the corrected subset.

## Goal
Repair the hand-copied fast subset embedded in the gauntlet and skirmish suites,
and refresh the stale CLAUDE.md test-execution table.

## Done when
- gauntlet and skirmish in RBTDRC_SUITES (rbtdrc_crucible.rs) carry the full fast-fixture set
  (both currently miss handbook-render, foundry-path, recipe-validation).
- The CLAUDE.md test-execution table matches the registry composition
  and names the five release/probe suites it currently omits.
- Build green; fast green.

## Sources
Memos/memo-20260610-heat-BH-lode-operation-durations.md (suite registry inventory section).

### cupel-python-import-allowlist (₢BHAAm) [complete]

**[260610-2158] complete**

## Character
Test-infrastructure extension; no cloud surface, no ladder dependency.
Tier: sonnet-delegable.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
Second pace of parallel-lane Chat 1, mounted in the same session as the suite-subset repair.
- Code, shellcheck, and the theurge build (`tt/rbw-tb.Build.sh`) are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — one consolidated
  build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: report lane done and STOP.

## Goal
Extend the cupel supply-chain conformance to *.py cloud steps
(it walks *.sh only today; the python steps are entirely unscanned —
the live specimen is rbgjv02 subprocess-running gcloud invisibly):
- import allowlist anchored on the module root of every import;
  stdlib floor per the memo; importlib / __import__ / exec / eval banned outright.
- subprocess argv[0] literals scanned against ZRBTDRU_GCB_ALLOWED —
  one tool floor, two languages; adjudicate rbgjv02's gcloud by allowlisting it
  with a dated comment (no REST conversion this pace).
- ten-minute probe: is zrbfc_expand_includes language-blind
  (the #@rbgjs_include marker is a valid python comment)?
  Record the answer against CBG CBp_101 (the preamble-duplication gap).

## Done when
The cupel walks python step files; an unsanctioned import or subprocess target fails
the cupel fixture; fast green via the central consolidated gate.

## Sources
Memos/memo-20260610-heat-BH-fable-recommendation-python-import-allowlist.md
(carries the empirical import floor and the CBG addenda, including the
authoritative-floor-lives-on-the-constant decision).

**[260610-2132] rough**

## Character
Test-infrastructure extension; no cloud surface, no ladder dependency.
Tier: sonnet-delegable.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
Second pace of parallel-lane Chat 1, mounted in the same session as the suite-subset repair.
- Code, shellcheck, and the theurge build (`tt/rbw-tb.Build.sh`) are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — one consolidated
  build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: report lane done and STOP.

## Goal
Extend the cupel supply-chain conformance to *.py cloud steps
(it walks *.sh only today; the python steps are entirely unscanned —
the live specimen is rbgjv02 subprocess-running gcloud invisibly):
- import allowlist anchored on the module root of every import;
  stdlib floor per the memo; importlib / __import__ / exec / eval banned outright.
- subprocess argv[0] literals scanned against ZRBTDRU_GCB_ALLOWED —
  one tool floor, two languages; adjudicate rbgjv02's gcloud by allowlisting it
  with a dated comment (no REST conversion this pace).
- ten-minute probe: is zrbfc_expand_includes language-blind
  (the #@rbgjs_include marker is a valid python comment)?
  Record the answer against CBG CBp_101 (the preamble-duplication gap).

## Done when
The cupel walks python step files; an unsanctioned import or subprocess target fails
the cupel fixture; fast green via the central consolidated gate.

## Sources
Memos/memo-20260610-heat-BH-fable-recommendation-python-import-allowlist.md
(carries the empirical import floor and the CBG addenda, including the
authoritative-floor-lives-on-the-constant decision).

**[260610-2117] rough**

## Character
Test-infrastructure extension; no cloud surface, no ladder dependency.
Tier: sonnet-delegable.

## Goal
Extend the cupel supply-chain conformance to *.py cloud steps
(it walks *.sh only today; the python steps are entirely unscanned —
the live specimen is rbgjv02 subprocess-running gcloud invisibly):
- import allowlist anchored on the module root of every import;
  stdlib floor per the memo; importlib / __import__ / exec / eval banned outright.
- subprocess argv[0] literals scanned against ZRBTDRU_GCB_ALLOWED —
  one tool floor, two languages; adjudicate rbgjv02's gcloud by allowlisting it
  with a dated comment (no REST conversion this pace).
- ten-minute probe: is zrbfc_expand_includes language-blind
  (the #@rbgjs_include marker is a valid python comment)?
  Record the answer against CBG CBp_101 (the preamble-duplication gap).

## Done when
The cupel walks python step files; an unsanctioned import or subprocess target fails
the cupel fixture; fast green.

## Sources
Memos/memo-20260610-heat-BH-fable-recommendation-python-import-allowlist.md
(carries the empirical import floor and the CBG addenda, including the
authoritative-floor-lives-on-the-constant decision).

### cloud-delete-hardening (₢BHAAh) [complete]

**[260610-2157] complete**

## Character
Four small repairs on one delete surface, bundled for one review and one live gate
(the endgame ladder's banish/abjure traffic covers them).
Tier: sonnet-delegable.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
First pace of parallel-lane Chat 2; other lanes are editing disjoint files concurrently.
- Code and shellcheck are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — fixture runs serialize
  globally; one consolidated build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap —
  the operator triggers each wrap after the central gate is green.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: proceed directly to ₢BHAAj, then ₢BHAAk, in this same session.

## Goal
- Pin ZRBFC_DELETE_BUILDER by digest (rbfc0_FoundryCore.sh) — it is the only floating
  builder running under a delete-privileged identity (Director SA);
  record the pin in RBSCB's cloud-dispatch posture section.
- rbfl_jettison (rbfld_Delete.sh): tolerate 404 as success — make the code match the
  "Jettisoned or nonexistent" message; idempotent delete is the house shape.
- rbgjl06-package-delete.py fire_delete: catch URLError alongside HTTPError
  (HTTPError subclasses URLError — structure the handler accordingly),
  log in the reconciling form, continue; the absence poll arbitrates.
- rbgjl06: explicit timeout (~30s) on every urlopen; fire_delete tolerates the timeout,
  truth-readers (package_absent, list_version_ids) and metadata_token die loud.

## Done when
All four landed; shellcheck green; fast green via the central consolidated gate.
Live coverage is structurally delegated to the endgame-verification ladder pace.

## Sources
Memos/memo-20260610-heat-BH-fable-recommendation-pin-delete-builder.md
Memos/memo-20260610-heat-BH-fable-recommendation-jettison-404-honesty.md
Memos/memo-20260610-heat-BH-fable-recommendation-urlerror-tolerance.md
Memos/memo-20260610-heat-BH-fable-recommendation-urllib-timeout.md

**[260610-2132] rough**

## Character
Four small repairs on one delete surface, bundled for one review and one live gate
(the endgame ladder's banish/abjure traffic covers them).
Tier: sonnet-delegable.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
First pace of parallel-lane Chat 2; other lanes are editing disjoint files concurrently.
- Code and shellcheck are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — fixture runs serialize
  globally; one consolidated build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap —
  the operator triggers each wrap after the central gate is green.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: proceed directly to ₢BHAAj, then ₢BHAAk, in this same session.

## Goal
- Pin ZRBFC_DELETE_BUILDER by digest (rbfc0_FoundryCore.sh) — it is the only floating
  builder running under a delete-privileged identity (Director SA);
  record the pin in RBSCB's cloud-dispatch posture section.
- rbfl_jettison (rbfld_Delete.sh): tolerate 404 as success — make the code match the
  "Jettisoned or nonexistent" message; idempotent delete is the house shape.
- rbgjl06-package-delete.py fire_delete: catch URLError alongside HTTPError
  (HTTPError subclasses URLError — structure the handler accordingly),
  log in the reconciling form, continue; the absence poll arbitrates.
- rbgjl06: explicit timeout (~30s) on every urlopen; fire_delete tolerates the timeout,
  truth-readers (package_absent, list_version_ids) and metadata_token die loud.

## Done when
All four landed; shellcheck green; fast green via the central consolidated gate.
Live coverage is structurally delegated to the endgame-verification ladder pace.

## Sources
Memos/memo-20260610-heat-BH-fable-recommendation-pin-delete-builder.md
Memos/memo-20260610-heat-BH-fable-recommendation-jettison-404-honesty.md
Memos/memo-20260610-heat-BH-fable-recommendation-urlerror-tolerance.md
Memos/memo-20260610-heat-BH-fable-recommendation-urllib-timeout.md

**[260610-2115] rough**

## Character
Four small repairs on one delete surface, bundled for one review and one live gate
(the endgame ladder's banish/abjure traffic covers them).
Tier: sonnet-delegable.

## Goal
- Pin ZRBFC_DELETE_BUILDER by digest (rbfc0_FoundryCore.sh) — it is the only floating
  builder running under a delete-privileged identity (Director SA);
  record the pin in RBSCB's cloud-dispatch posture section.
- rbfl_jettison (rbfld_Delete.sh): tolerate 404 as success — make the code match the
  "Jettisoned or nonexistent" message; idempotent delete is the house shape.
- rbgjl06-package-delete.py fire_delete: catch URLError alongside HTTPError
  (HTTPError subclasses URLError — structure the handler accordingly),
  log in the reconciling form, continue; the absence poll arbitrates.
- rbgjl06: explicit timeout (~30s) on every urlopen; fire_delete tolerates the timeout,
  truth-readers (package_absent, list_version_ids) and metadata_token die loud.

## Done when
All four landed; build + shellcheck + fast green.
Live coverage is structurally delegated to the endgame-verification ladder pace.

## Sources
Memos/memo-20260610-heat-BH-fable-recommendation-pin-delete-builder.md
Memos/memo-20260610-heat-BH-fable-recommendation-jettison-404-honesty.md
Memos/memo-20260610-heat-BH-fable-recommendation-urlerror-tolerance.md
Memos/memo-20260610-heat-BH-fable-recommendation-urllib-timeout.md

### elect-anchor-slot-count-soften (₢BHAAj) [complete]

**[260610-2157] complete**

## Character
Tiny surgical repair mirroring the landed kind-brand fix (commit 91666a97b).
Tier: sonnet-delegable.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
Second pace of parallel-lane Chat 2, same session as the cloud-delete hardening.
- Code and shellcheck are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — one consolidated
  build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: proceed directly to ₢BHAAk in this same session.

## Goal
zrbfd_elect_base_anchor (rbfd_FoundryDirectorBuild.sh): when the populated
RBRV_IMAGE_n_ORIGIN slot count is not exactly one, log-and-leave-the-ANCHOR
(loud buc_log_args), not buc_die — absence of election is already a normal outcome
of this function, and the die kills a multi-origin ordain behind a chained bole fact
(dormant today: no live vessel populates slots 2-3; fires on the first that does).
Decide at mount whether count==0 warrants a distinct message from count>1.

## Done when
The die is softened to log-and-leave; fast green via the central consolidated gate;
a fixture case covers the multi-slot no-op if one is cheap to add.

## Sources
Memos/memo-20260610-heat-BH-fable-recommendation-elect-anchor-slot-count.md

**[260610-2132] rough**

## Character
Tiny surgical repair mirroring the landed kind-brand fix (commit 91666a97b).
Tier: sonnet-delegable.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
Second pace of parallel-lane Chat 2, same session as the cloud-delete hardening.
- Code and shellcheck are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — one consolidated
  build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: proceed directly to ₢BHAAk in this same session.

## Goal
zrbfd_elect_base_anchor (rbfd_FoundryDirectorBuild.sh): when the populated
RBRV_IMAGE_n_ORIGIN slot count is not exactly one, log-and-leave-the-ANCHOR
(loud buc_log_args), not buc_die — absence of election is already a normal outcome
of this function, and the die kills a multi-origin ordain behind a chained bole fact
(dormant today: no live vessel populates slots 2-3; fires on the first that does).
Decide at mount whether count==0 warrants a distinct message from count>1.

## Done when
The die is softened to log-and-leave; fast green via the central consolidated gate;
a fixture case covers the multi-slot no-op if one is cheap to add.

## Sources
Memos/memo-20260610-heat-BH-fable-recommendation-elect-anchor-slot-count.md

**[260610-2115] rough**

## Character
Tiny surgical repair mirroring the landed kind-brand fix (commit 91666a97b).
Tier: sonnet-delegable.

## Goal
zrbfd_elect_base_anchor (rbfd_FoundryDirectorBuild.sh): when the populated
RBRV_IMAGE_n_ORIGIN slot count is not exactly one, log-and-leave-the-ANCHOR
(loud buc_log_args), not buc_die — absence of election is already a normal outcome
of this function, and the die kills a multi-origin ordain behind a chained bole fact
(dormant today: no live vessel populates slots 2-3; fires on the first that does).
Decide at mount whether count==0 warrants a distinct message from count>1.

## Done when
The die is softened to log-and-leave; fast green;
a fixture case covers the multi-slot no-op if one is cheap to add.

## Sources
Memos/memo-20260610-heat-BH-fable-recommendation-elect-anchor-slot-count.md

### invest-actas-readback-gate (₢BHAAk) [complete]

**[260610-2157] complete**

## Character
Small consistency extension of the rbk-08 read-back-verification pattern.
Tier: sonnet-delegable.
The live flap signature is unobservable on the standing depot;
tonight's overnight skirmish exercises the invest path live via canonical-invest.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
Third and last pace of parallel-lane Chat 2.
- Code and shellcheck are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — one consolidated
  build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: report lane done and STOP.

## Goal
rbgg_invest_director: after granting the Director SA self-actAs
(roles/iam.serviceAccountUser on itself), poll the SA IAM policy until the binding
is visible before declaring invest complete — closes the Class-C propagation flap
at the first post-invest builds.create (the spine dispatch dies on the first
PERMISSION_DENIED today).

## Done when
Read-back gate landed matching the existing read-back pattern;
fast green via the central consolidated gate;
the invest path unbroken against the standing depot (skirmish's canonical-invest is the live proof).

## Sources
Memos/memo-20260610-heat-BH-fable-recommendation-actas-propagation-flap.md
(repair option 1 — invest-side gate — chosen at triage; submit-side tolerance declined).

**[260610-2132] rough**

## Character
Small consistency extension of the rbk-08 read-back-verification pattern.
Tier: sonnet-delegable.
The live flap signature is unobservable on the standing depot;
tonight's overnight skirmish exercises the invest path live via canonical-invest.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
Third and last pace of parallel-lane Chat 2.
- Code and shellcheck are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — one consolidated
  build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: report lane done and STOP.

## Goal
rbgg_invest_director: after granting the Director SA self-actAs
(roles/iam.serviceAccountUser on itself), poll the SA IAM policy until the binding
is visible before declaring invest complete — closes the Class-C propagation flap
at the first post-invest builds.create (the spine dispatch dies on the first
PERMISSION_DENIED today).

## Done when
Read-back gate landed matching the existing read-back pattern;
fast green via the central consolidated gate;
the invest path unbroken against the standing depot (skirmish's canonical-invest is the live proof).

## Sources
Memos/memo-20260610-heat-BH-fable-recommendation-actas-propagation-flap.md
(repair option 1 — invest-side gate — chosen at triage; submit-side tolerance declined).

**[260610-2116] rough**

## Character
Small consistency extension of the rbk-08 read-back-verification pattern.
Tier: sonnet-delegable.
The live flap signature is unobservable on the standing depot;
live proof rides the operator's depot-recreate plan, not this heat.

## Goal
rbgg_invest_director: after granting the Director SA self-actAs
(roles/iam.serviceAccountUser on itself), poll the SA IAM policy until the binding
is visible before declaring invest complete — closes the Class-C propagation flap
at the first post-invest builds.create (the spine dispatch dies on the first
PERMISSION_DENIED today).

## Done when
Read-back gate landed matching the existing read-back pattern; fast green;
the invest path unbroken against the standing depot.

## Sources
Memos/memo-20260610-heat-BH-fable-recommendation-actas-propagation-flap.md
(repair option 1 — invest-side gate — chosen at triage; submit-side tolerance declined).

### immure-capture-residue (₢BHAAi) [complete]

**[260610-2155] complete**

## Character
Mechanical bundle from the immure pre-wrap review. Tier: sonnet-delegable.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
First pace of parallel-lane Chat 3; other lanes are editing disjoint files concurrently.
- Code and shellcheck are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — fixture runs serialize
  globally; one consolidated build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap —
  the operator triggers each wrap after the central gate is green.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: report lane done and STOP — ₢BHAAo follows in this session only if the
  operator explicitly elects the stretch; otherwise it is morning work.

## Goal
- rbgjl07 immure-select: assert both selection-entry fields non-empty
  (an empty disktype currently matches any annotation-less descriptor silently).
- rbgjl08/09: `< /dev/null` on network-tool calls inside while-read loops
  (the house-documented stdin hazard; busybox sh has no arrays, so the belt beats load-then-iterate).
- rbw-di egress inventory: add a line for quay.io's blob-CDN redirect hosts.
- Size-check the podvm-native 8-leaf envelope against the buildStepOutputs 4KB contract
  (CBG CBh_103); record the arithmetic where the envelope shape is documented.
- claude-rbk-acronyms.md: catalogue the RBSL spec family (one entry block).

## Done when
All five landed; shellcheck green; fast green via the central consolidated gate.

## Sources
Memos/memo-20260610-heat-BH-fable-review-immure-noncritical.md (items 1, 2, 6, 7, 8;
items 3, 4, 5 are records only — explicitly declined at triage).

**[260610-2132] rough**

## Character
Mechanical bundle from the immure pre-wrap review. Tier: sonnet-delegable.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
First pace of parallel-lane Chat 3; other lanes are editing disjoint files concurrently.
- Code and shellcheck are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — fixture runs serialize
  globally; one consolidated build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap —
  the operator triggers each wrap after the central gate is green.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: report lane done and STOP — ₢BHAAo follows in this session only if the
  operator explicitly elects the stretch; otherwise it is morning work.

## Goal
- rbgjl07 immure-select: assert both selection-entry fields non-empty
  (an empty disktype currently matches any annotation-less descriptor silently).
- rbgjl08/09: `< /dev/null` on network-tool calls inside while-read loops
  (the house-documented stdin hazard; busybox sh has no arrays, so the belt beats load-then-iterate).
- rbw-di egress inventory: add a line for quay.io's blob-CDN redirect hosts.
- Size-check the podvm-native 8-leaf envelope against the buildStepOutputs 4KB contract
  (CBG CBh_103); record the arithmetic where the envelope shape is documented.
- claude-rbk-acronyms.md: catalogue the RBSL spec family (one entry block).

## Done when
All five landed; shellcheck green; fast green via the central consolidated gate.

## Sources
Memos/memo-20260610-heat-BH-fable-review-immure-noncritical.md (items 1, 2, 6, 7, 8;
items 3, 4, 5 are records only — explicitly declined at triage).

**[260610-2115] rough**

## Character
Mechanical bundle from the immure pre-wrap review. Tier: sonnet-delegable.

## Goal
- rbgjl07 immure-select: assert both selection-entry fields non-empty
  (an empty disktype currently matches any annotation-less descriptor silently).
- rbgjl08/09: `< /dev/null` on network-tool calls inside while-read loops
  (the house-documented stdin hazard; busybox sh has no arrays, so the belt beats load-then-iterate).
- rbw-di egress inventory: add a line for quay.io's blob-CDN redirect hosts.
- Size-check the podvm-native 8-leaf envelope against the buildStepOutputs 4KB contract
  (CBG CBh_103); record the arithmetic where the envelope shape is documented.
- claude-rbk-acronyms.md: catalogue the RBSL spec family (one entry block).

## Done when
All five landed; shellcheck + fast green.

## Sources
Memos/memo-20260610-heat-BH-fable-review-immure-noncritical.md (items 1, 2, 6, 7, 8;
items 3, 4, 5 are records only — explicitly declined at triage).

### enshrine-spec-structural-residue (₢BHAAn) [complete]

**[260610-2200] complete**

## Character
Small spec-hygiene pace; dangle-safe quoin surgery.
Must land before the vocab-finalization scrub so the scrub sweeps its word-level tail.
Tier: sonnet-delegable.

## Goal
- Retire RBSIA's Enshrinements-Audit section and the gar_enshrines_namespace quoin
  (mapping attribute + definition + every ref) — the depot is disposable,
  so auditing legacy rbi_es artifacts has no future subject;
  the iae tabtargets are already gone (Wave 1 of the image-family retirement).
- Collapse rbst_reliquary_stamp into rbst_touchmark: one quoin survives, refs repoint;
  both already match the same stamp form.

## Done when
Quoins resolve clean — no dangling attribute refs or anchors;
the later scrub's discovery grep finds no structural residue from these two items;
fast green.

## Sources
Memos/memo-20260610-heat-BH-enshrine-structural-residue.md (items 2 and 3;
item 1 confirmed owned by the terminal vocab scrub's KILL list — no action here).

**[260610-2117] rough**

## Character
Small spec-hygiene pace; dangle-safe quoin surgery.
Must land before the vocab-finalization scrub so the scrub sweeps its word-level tail.
Tier: sonnet-delegable.

## Goal
- Retire RBSIA's Enshrinements-Audit section and the gar_enshrines_namespace quoin
  (mapping attribute + definition + every ref) — the depot is disposable,
  so auditing legacy rbi_es artifacts has no future subject;
  the iae tabtargets are already gone (Wave 1 of the image-family retirement).
- Collapse rbst_reliquary_stamp into rbst_touchmark: one quoin survives, refs repoint;
  both already match the same stamp form.

## Done when
Quoins resolve clean — no dangling attribute refs or anchors;
the later scrub's discovery grep finds no structural residue from these two items;
fast green.

## Sources
Memos/memo-20260610-heat-BH-enshrine-structural-residue.md (items 2 and 3;
item 1 confirmed owned by the terminal vocab scrub's KILL list — no action here).

### single-slot-extract-consolidation (₢BHAAo) [complete]

**[260610-2236] complete**

## Character
Behavior-preserving consolidation; the judgment was settled at triage:
the per-kind-verbs premise protects vocabulary, not extract implementation,
and the propagation-drift class fired on exactly this block
(the rbls_ sprue sweep missed one of the three copies and broke the service suite same-day).
Tier: sonnet-delegable, opus-grade review.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
Optional stretch pace of parallel-lane Chat 3 — mount tonight only if the operator
explicitly elects it; otherwise this is morning work and the protocol below still applies
if other paces are in flight.
- Code and shellcheck are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — one consolidated
  build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: report lane done and STOP.

## Goal
One shared single-slot extract (prefix / brand / label parameterized) consumed by
underpin, conclave, and immure — replacing the three byte-parallel copies in
rbldw_Underpin.sh, rbldr_Reliquary.sh, rbldv_Immure.sh, including the keys-dump diagnostic.
rbldb_Bole.sh stays separate: its multi-slot 1..3 continue-on-empty loop is genuinely
different shape.
The helper's home is a mount-time choice — kind data arrives as parameters,
so a spine-adjacent home does not violate spine-owns-no-kind-knowledge.

## Done when
Three copies replaced by one; build + shellcheck green; fast green via the central
consolidated gate.
Live coverage: if landed tonight before the ladder, the complete leg's lode fixtures cover it;
otherwise it shares the next service-tier run (verification batching) — never a dedicated run.

## Sources
Memos/memo-20260610-heat-BH-extract-keys-triplication.md (option 2 chosen at triage).

**[260610-2132] rough**

## Character
Behavior-preserving consolidation; the judgment was settled at triage:
the per-kind-verbs premise protects vocabulary, not extract implementation,
and the propagation-drift class fired on exactly this block
(the rbls_ sprue sweep missed one of the three copies and broke the service suite same-day).
Tier: sonnet-delegable, opus-grade review.

## Tonight's parallel-lane protocol (operator-cinched 2026-06-10)
Optional stretch pace of parallel-lane Chat 3 — mount tonight only if the operator
explicitly elects it; otherwise this is morning work and the protocol below still applies
if other paces are in flight.
- Code and shellcheck are allowed.
- Do NOT run `fast`, any `rbw-ts`/`rbw-tf`/`rbw-tc`, or any fixture — one consolidated
  build+fast gate runs centrally after all lanes report.
- Notch via jjx_record with an explicit file list when code-complete. Do NOT wrap.
- Touch only this pace's files; other lanes' uncommitted work is not yours.
- When notched: report lane done and STOP.

## Goal
One shared single-slot extract (prefix / brand / label parameterized) consumed by
underpin, conclave, and immure — replacing the three byte-parallel copies in
rbldw_Underpin.sh, rbldr_Reliquary.sh, rbldv_Immure.sh, including the keys-dump diagnostic.
rbldb_Bole.sh stays separate: its multi-slot 1..3 continue-on-empty loop is genuinely
different shape.
The helper's home is a mount-time choice — kind data arrives as parameters,
so a spine-adjacent home does not violate spine-owns-no-kind-knowledge.

## Done when
Three copies replaced by one; build + shellcheck green; fast green via the central
consolidated gate.
Live coverage: if landed tonight before the ladder, the complete leg's lode fixtures cover it;
otherwise it shares the next service-tier run (verification batching) — never a dedicated run.

## Sources
Memos/memo-20260610-heat-BH-extract-keys-triplication.md (option 2 chosen at triage).

**[260610-2117] rough**

## Character
Behavior-preserving consolidation; the judgment was settled at triage:
the per-kind-verbs premise protects vocabulary, not extract implementation,
and the propagation-drift class fired on exactly this block
(the rbls_ sprue sweep missed one of the three copies and broke the service suite same-day).
Tier: sonnet-delegable, opus-grade review.

## Goal
One shared single-slot extract (prefix / brand / label parameterized) consumed by
underpin, conclave, and immure — replacing the three byte-parallel copies in
rbldw_Underpin.sh, rbldr_Reliquary.sh, rbldv_Immure.sh, including the keys-dump diagnostic.
rbldb_Bole.sh stays separate: its multi-slot 1..3 continue-on-empty loop is genuinely
different shape.
The helper's home is a mount-time choice — kind data arrives as parameters,
so a spine-adjacent home does not violate spine-owns-no-kind-knowledge.

## Done when
Three copies replaced by one; build + shellcheck + fast green.
Live coverage shares the next service-tier run (verification batching),
never a dedicated run of its own.

## Sources
Memos/memo-20260610-heat-BH-extract-keys-triplication.md (option 2 chosen at triage).

### burv-invoke-dir-isolation-verify (₢BHAAc) [complete]

**[260610-2200] complete**

## Character
Mechanical residue check, no code change expected.
Mount after a post-repair service-tier suite run exists.
Tier: haiku/sonnet-delegable.

## Goal
Confirm the theurge suite-loop invoke-counter fix (commit 010ec044a) gives each tabtarget invocation its own BURV dir — no cross-fixture invoke-dir collision — so a non-chained invoke's `previous/` is genuinely empty.
That collision was the leak that fed wsl-lifecycle's `rbf_fact_lode_brand=wsl` into batch-vouch's ordain election.

## Done when
On the latest post-repair service-suite trace dir (`../temp-buk/temp-*/rbtd`):
- BURV invoke-dir count ≈ invocation count, where reuse of a dir more than once happens only where chaining is intentional.
  The service suite uses no `chain_next_invoke`, so the two counts should be equal.
- batch-vouch's ordain invoke `previous/` carries no foreign `rbf_fact_lode_brand`.

Recipe:
`ls -1d <run>/rbtd/burv-output/invoke-* | wc -l` vs `find <run>/rbtd/burv-temp/invoke-* -maxdepth 1 -name 'temp-*' | wc -l`

Pre-repair baseline for contrast (the failing run): 10 dirs / 32 invocations, slots reused up to 6x.
If the relevant trace dir has been reaped, mount after the next service run rather than paying for a fresh one.

## Cinched
The repair approach (suite-monotonic counter, not resetting per fixture) is settled and landed; this pace verifies it, it does not re-open it.

**[260610-0556] rough**

## Character
Mechanical residue check, no code change expected.
Mount after a post-repair service-tier suite run exists.
Tier: haiku/sonnet-delegable.

## Goal
Confirm the theurge suite-loop invoke-counter fix (commit 010ec044a) gives each tabtarget invocation its own BURV dir — no cross-fixture invoke-dir collision — so a non-chained invoke's `previous/` is genuinely empty.
That collision was the leak that fed wsl-lifecycle's `rbf_fact_lode_brand=wsl` into batch-vouch's ordain election.

## Done when
On the latest post-repair service-suite trace dir (`../temp-buk/temp-*/rbtd`):
- BURV invoke-dir count ≈ invocation count, where reuse of a dir more than once happens only where chaining is intentional.
  The service suite uses no `chain_next_invoke`, so the two counts should be equal.
- batch-vouch's ordain invoke `previous/` carries no foreign `rbf_fact_lode_brand`.

Recipe:
`ls -1d <run>/rbtd/burv-output/invoke-* | wc -l` vs `find <run>/rbtd/burv-temp/invoke-* -maxdepth 1 -name 'temp-*' | wc -l`

Pre-repair baseline for contrast (the failing run): 10 dirs / 32 invocations, slots reused up to 6x.
If the relevant trace dir has been reaped, mount after the next service run rather than paying for a fresh one.

## Cinched
The repair approach (suite-monotonic counter, not resetting per fixture) is settled and landed; this pace verifies it, it does not re-open it.

### endgame-verification-ladder (₢BHAAf) [complete]

**[260611-0730] complete**

## Character
The heat's batched cloud-spend verification — one ladder pass discharging every deferred live gate.
Operator-gated spend; everything before it in the rail is statically green by construction.
Order inversion cinched at triage (2026-06-10): this ladder runs the night the triage-slated mechanical fixes land,
AHEAD of the heat's remaining host-side / spec / test-infra paces
(the reproducibility audit, the credless-by-construction guard, the cupel python walk, the yoke colophon move, the vocab scrub) —
those verify at fast/static tier and gain nothing from this spend;
any of them that turns out to warrant live proof shares the next service-tier run, never a rerun of this ladder.

## Goal
Discharge the deferred live gates in one batch on the standing depot:
the skopeo-eviction bind-mode verify (delegated here by that pace's docket)
plus live coverage of the triage-slated fixes that landed ahead of it
(the cloud-delete hardening rides the complete leg's banish/abjure traffic;
the invest read-back gate rides skirmish's canonical-invest).

## Cinched
- skirmish, not gauntlet — same bind-leg coverage, no project churn;
  fresh-project coverage belongs to the operator's standing depot-recreate plan, not this heat.
- `complete` rides first (service ∪ crucible — the cheap non-ladder tier), then skirmish.
- This run is also the durations re-baseline (triage 2026-06-10 confirmed the disposition):
  capture this batch's timings as the new record;
  further per-operation extraction was declined — the durations memo stands as the manifest.
- The stale fast subset in gauntlet/skirmish is repaired by the suite-subset pace railed first tonight;
  if this ladder somehow runs before that lands, the `complete` leg covers the missing three — note it in the wrap either way.

## Open at mount
- Whether blockade rides too: airgap-mode conjure (the resolver repoint and ANCHOR derived-pull both sit under it)
  may have no post-cutover live verification — check the heat's run record at mount;
  if uncovered, blockade is the venue and earns its spend.

## Done when
`complete` green; skirmish green with the bind-mode build as the named gate;
blockade green if elected at mount;
timings recorded for the durations re-baseline;
results in the wrap.

**[260610-2128] rough**

## Character
The heat's batched cloud-spend verification — one ladder pass discharging every deferred live gate.
Operator-gated spend; everything before it in the rail is statically green by construction.
Order inversion cinched at triage (2026-06-10): this ladder runs the night the triage-slated mechanical fixes land,
AHEAD of the heat's remaining host-side / spec / test-infra paces
(the reproducibility audit, the credless-by-construction guard, the cupel python walk, the yoke colophon move, the vocab scrub) —
those verify at fast/static tier and gain nothing from this spend;
any of them that turns out to warrant live proof shares the next service-tier run, never a rerun of this ladder.

## Goal
Discharge the deferred live gates in one batch on the standing depot:
the skopeo-eviction bind-mode verify (delegated here by that pace's docket)
plus live coverage of the triage-slated fixes that landed ahead of it
(the cloud-delete hardening rides the complete leg's banish/abjure traffic;
the invest read-back gate rides skirmish's canonical-invest).

## Cinched
- skirmish, not gauntlet — same bind-leg coverage, no project churn;
  fresh-project coverage belongs to the operator's standing depot-recreate plan, not this heat.
- `complete` rides first (service ∪ crucible — the cheap non-ladder tier), then skirmish.
- This run is also the durations re-baseline (triage 2026-06-10 confirmed the disposition):
  capture this batch's timings as the new record;
  further per-operation extraction was declined — the durations memo stands as the manifest.
- The stale fast subset in gauntlet/skirmish is repaired by the suite-subset pace railed first tonight;
  if this ladder somehow runs before that lands, the `complete` leg covers the missing three — note it in the wrap either way.

## Open at mount
- Whether blockade rides too: airgap-mode conjure (the resolver repoint and ANCHOR derived-pull both sit under it)
  may have no post-cutover live verification — check the heat's run record at mount;
  if uncovered, blockade is the venue and earns its spend.

## Done when
`complete` green; skirmish green with the bind-mode build as the named gate;
blockade green if elected at mount;
timings recorded for the durations re-baseline;
results in the wrap.

**[260610-1605] rough**

## Character
The heat's batched cloud-spend verification — one ladder pass discharging every deferred live gate.
Runs LAST of all: after the memo-triage pace and after any fix paces it slates have landed.
Operator-gated spend; everything before it in the heat is statically green by construction.

## Goal
Discharge the deferred live gates in one batch on the standing depot:
the skopeo-eviction bind-mode verify (delegated here by that pace's docket)
plus live coverage of whatever the triage-slated fixes touched.

## Cinched
- skirmish, not gauntlet — same bind-leg coverage, no project churn;
  fresh-project coverage belongs to the operator's standing depot-recreate plan, not this heat.
- `complete` rides first (service ∪ crucible — the cheap non-ladder tier), then skirmish.
- This run is also the durations re-baseline: the heat invalidated prior suite timings
  (the durations memo records the inventory and pins the prior baseline);
  capture this batch's timings as the new record, per whatever disposition triage gave that memo.
- Known wrinkle: gauntlet/skirmish embed a stale hand-copied fast subset
  (three of fast's ten fixtures missing — durations memo finding).
  If triage's disposition lands before this runs, the subset is fixed; if not,
  the `complete` leg already covers the missing three — note it in the wrap either way.

## Open at mount
- Whether blockade rides too: airgap-mode conjure (the resolver repoint and ANCHOR derived-pull both sit under it)
  may have no post-cutover live verification — check the heat's run record at mount;
  if uncovered, blockade is the venue and earns its spend.

## Done when
`complete` green; skirmish green with the bind-mode build as the named gate;
blockade green if elected at mount;
timings recorded for the durations re-baseline;
results in the wrap.

### image-clean-commit-reproducibility-audit (₢BHAAR) [complete]

**[260610-2128] complete**

## Character
Reproducibility-correctness audit across every image construction and acquisition path — design and judgment, not mechanical prose.
The original "teach the onboarding clean-tree convention" framing is subsumed: a premise it rested on — that the clean-tree gate is already uniform across the Director tracks — proved false at slate-time, so the real work is making the guarantee certain in code first, then teaching only what the code enforces.
**MUST be worked by Fable (claude-fable-5).** Operator-required tier; do not delegate the audit judgment to a cheaper model.

## Order note (triage 2026-06-10)
The endgame verification ladder runs before this pace by operator cinch —
do not expect ladder coverage for the gates this audit lands.
They verify at fast/static tier (a dirty-tree refusal is honestly testable without cloud);
any change here that genuinely warrants live proof shares the next service-tier run
(verification batching), never a ladder rerun.
Rider from the eviction pace wrap: the 2026-06-10 conclave run incidentally live-verified the capture-side gate.

## Governing rule
Every image RB constructs or acquires must trace to a clean git commit, or it is not reproducible.
Make that certain: every path that builds or acquires an image refuses on a dirty working tree — before any step stamps git provenance or ships bytes to GAR — and closes any gap by refusing, never by auto-committing.

## Starting map — confirm, do not trust
The gate is not uniform today.
`bug_require_clean_tree` has only two callers — kludge and bind-mode mirror.
Conjure- and graft-mode `rbfd_ordain` do not gate; the conjure base-anchor election (`zrbfd_elect_base_anchor`) writes rbrv.env ungated; inscribe (reliquary, depot tripwire) does not gate despite the mirror guard's "same as inscribe" comment; the Lode capture verbs write GAR but their clean-tree story is unverified; the only near-universal refusal is the path-scoped `rbob_charge` nameplate gate.
Re-derive this map from scratch — line-level specifics drift between slate and mount.

## Discovery recipe — locate every image-construction or -acquisition path
- Made side: `rbfd_ordain` (conjure / bind / graft modes), `rbfk_kludge`, the cloud `builds.create` submission, and `zrbfc_ensure_git_metadata` — the chokepoint that stamps the git commit into provenance, so anything reaching it on a dirty tree records a commit that does not match the bytes built.
- Fetched side / Lode processing: every capture verb (`ensconce`/`conclave`/`underpin`/`immure`) and the shared spine `rblds_Spine.sh` — decide whether a captured Lode's reproducibility depends on RB's tree at all, and whether a dirty tree can corrupt its provenance envelope.
- For each path, classify: gates-today / must-gate / correctly-ungated-with-stated-reason (graft's "image already built, git state irrelevant" is the model for the last).

## Cinched
- The invariant is reproducibility, not ergonomics — a path that can emit or acquire an image from a dirty tree is the bug, however rarely it is hit.
- Tools never commit; the fix is always to refuse, and Marshal-Zero stays the sole sanctioned self-commit.
- Onboarding prose follows the code: teach "commit before <step>" only at the tracks that actually gate once this audit lands, never a fiction.

## Done when
Every image-construction and -acquisition path, made and fetched, is accounted for — each either refuses on a dirty tree before the act, or carries a stated, defensible reason it need not.
The guarantee "every RB image traces to a clean commit" is certain across both sides, and the onboarding convention is surfaced only where the code now enforces it.

**[260610-1952] complete**

## Character
Reproducibility-correctness audit across every image construction and acquisition path — design and judgment, not mechanical prose.
The original "teach the onboarding clean-tree convention" framing is subsumed: a premise it rested on — that the clean-tree gate is already uniform across the Director tracks — proved false at slate-time, so the real work is making the guarantee certain in code first, then teaching only what the code enforces.
**MUST be worked by Fable (claude-fable-5).** Operator-required tier; do not delegate the audit judgment to a cheaper model.

## Governing rule
Every image RB constructs or acquires must trace to a clean git commit, or it is not reproducible.
Make that certain: every path that builds or acquires an image refuses on a dirty working tree — before any step stamps git provenance or ships bytes to GAR — and closes any gap by refusing, never by auto-committing.

## Starting map — confirm, do not trust
The gate is not uniform today.
`bug_require_clean_tree` has only two callers — kludge and bind-mode mirror.
Conjure- and graft-mode `rbfd_ordain` do not gate; the conjure base-anchor election (`zrbfd_elect_base_anchor`) writes rbrv.env ungated; inscribe (reliquary, depot tripwire) does not gate despite the mirror guard's "same as inscribe" comment; the Lode capture verbs write GAR but their clean-tree story is unverified; the only near-universal refusal is the path-scoped `rbob_charge` nameplate gate.
Re-derive this map from scratch — line-level specifics drift between slate and mount.

## Discovery recipe — locate every image-construction or -acquisition path
- Made side: `rbfd_ordain` (conjure / bind / graft modes), `rbfk_kludge`, the cloud `builds.create` submission, and `zrbfc_ensure_git_metadata` — the chokepoint that stamps the git commit into provenance, so anything reaching it on a dirty tree records a commit that does not match the bytes built.
- Fetched side / Lode processing: every capture verb (`ensconce`/`conclave`/`underpin`/`immure`) and the shared spine `rblds_Spine.sh` — decide whether a captured Lode's reproducibility depends on RB's tree at all, and whether a dirty tree can corrupt its provenance envelope.
- For each path, classify: gates-today / must-gate / correctly-ungated-with-stated-reason (graft's "image already built, git state irrelevant" is the model for the last).

## Cinched
- The invariant is reproducibility, not ergonomics — a path that can emit or acquire an image from a dirty tree is the bug, however rarely it is hit.
- Tools never commit; the fix is always to refuse, and Marshal-Zero stays the sole sanctioned self-commit.
- Onboarding prose follows the code: teach "commit before <step>" only at the tracks that actually gate once this audit lands, never a fiction.

## Done when
Every image-construction and -acquisition path, made and fetched, is accounted for — each either refuses on a dirty tree before the act, or carries a stated, defensible reason it need not.
The guarantee "every RB image traces to a clean commit" is certain across both sides, and the onboarding convention is surfaced only where the code now enforces it.

**[260610-1852] rough**

## Character
Reproducibility-correctness audit across every image construction and acquisition path — design and judgment, not mechanical prose.
The original "teach the onboarding clean-tree convention" framing is subsumed: a premise it rested on — that the clean-tree gate is already uniform across the Director tracks — proved false at slate-time, so the real work is making the guarantee certain in code first, then teaching only what the code enforces.
**MUST be worked by Fable (claude-fable-5).** Operator-required tier; do not delegate the audit judgment to a cheaper model.

## Governing rule
Every image RB constructs or acquires must trace to a clean git commit, or it is not reproducible.
Make that certain: every path that builds or acquires an image refuses on a dirty working tree — before any step stamps git provenance or ships bytes to GAR — and closes any gap by refusing, never by auto-committing.

## Starting map — confirm, do not trust
The gate is not uniform today.
`bug_require_clean_tree` has only two callers — kludge and bind-mode mirror.
Conjure- and graft-mode `rbfd_ordain` do not gate; the conjure base-anchor election (`zrbfd_elect_base_anchor`) writes rbrv.env ungated; inscribe (reliquary, depot tripwire) does not gate despite the mirror guard's "same as inscribe" comment; the Lode capture verbs write GAR but their clean-tree story is unverified; the only near-universal refusal is the path-scoped `rbob_charge` nameplate gate.
Re-derive this map from scratch — line-level specifics drift between slate and mount.

## Discovery recipe — locate every image-construction or -acquisition path
- Made side: `rbfd_ordain` (conjure / bind / graft modes), `rbfk_kludge`, the cloud `builds.create` submission, and `zrbfc_ensure_git_metadata` — the chokepoint that stamps the git commit into provenance, so anything reaching it on a dirty tree records a commit that does not match the bytes built.
- Fetched side / Lode processing: every capture verb (`ensconce`/`conclave`/`underpin`/`immure`) and the shared spine `rblds_Spine.sh` — decide whether a captured Lode's reproducibility depends on RB's tree at all, and whether a dirty tree can corrupt its provenance envelope.
- For each path, classify: gates-today / must-gate / correctly-ungated-with-stated-reason (graft's "image already built, git state irrelevant" is the model for the last).

## Cinched
- The invariant is reproducibility, not ergonomics — a path that can emit or acquire an image from a dirty tree is the bug, however rarely it is hit.
- Tools never commit; the fix is always to refuse, and Marshal-Zero stays the sole sanctioned self-commit.
- Onboarding prose follows the code: teach "commit before <step>" only at the tracks that actually gate once this audit lands, never a fiction.

## Done when
Every image-construction and -acquisition path, made and fetched, is accounted for — each either refuses on a dirty tree before the act, or carries a stated, defensible reason it need not.
The guarantee "every RB image traces to a clean commit" is certain across both sides, and the onboarding convention is surfaced only where the code now enforces it.

**[260610-1852] rough**

## Character
Reproducibility-correctness audit across every image construction and acquisition path — design and judgment, not mechanical prose.
The original "teach the onboarding clean-tree convention" framing is subsumed: a premise it rested on — that the clean-tree gate is already uniform across the Director tracks — proved false at slate-time, so the real work is making the guarantee certain in code first, then teaching only what the code enforces.
**MUST be worked by Fable (claude-fable-5).** Operator-required tier; do not delegate the audit judgment to a cheaper model.

## Governing rule
Every image RB constructs or acquires must trace to a clean git commit, or it is not reproducible.
Make that certain: every path that builds or acquires an image refuses on a dirty working tree — before any step stamps git provenance or ships bytes to GAR — and closes any gap by refusing, never by auto-committing.

## Starting map — confirm, do not trust
The gate is not uniform today.
`bug_require_clean_tree` has only two callers — kludge and bind-mode mirror.
Conjure- and graft-mode `rbfd_ordain` do not gate; the conjure base-anchor election (`zrbfd_elect_base_anchor`) writes rbrv.env ungated; inscribe (reliquary, depot tripwire) does not gate despite the mirror guard's "same as inscribe" comment; the Lode capture verbs write GAR but their clean-tree story is unverified; the only near-universal refusal is the path-scoped `rbob_charge` nameplate gate.
Re-derive this map from scratch — line-level specifics drift between slate and mount.

## Discovery recipe — locate every image-construction or -acquisition path
- Made side: `rbfd_ordain` (conjure / bind / graft modes), `rbfk_kludge`, the cloud `builds.create` submission, and `zrbfc_ensure_git_metadata` — the chokepoint that stamps the git commit into provenance, so anything reaching it on a dirty tree records a commit that does not match the bytes built.
- Fetched side / Lode processing: every capture verb (`ensconce`/`conclave`/`underpin`/`immure`) and the shared spine `rblds_Spine.sh` — decide whether a captured Lode's reproducibility depends on RB's tree at all, and whether a dirty tree can corrupt its provenance envelope.
- For each path, classify: gates-today / must-gate / correctly-ungated-with-stated-reason (graft's "image already built, git state irrelevant" is the model for the last).

## Cinched
- The invariant is reproducibility, not ergonomics — a path that can emit or acquire an image from a dirty tree is the bug, however rarely it is hit.
- Tools never commit; the fix is always to refuse, and Marshal-Zero stays the sole sanctioned self-commit.
- Onboarding prose follows the code: teach "commit before <step>" only at the tracks that actually gate once this audit lands, never a fiction.

## Done when
Every image-construction and -acquisition path, made and fetched, is accounted for — each either refuses on a dirty tree before the act, or carries a stated, defensible reason it need not.
The guarantee "every RB image traces to a clean commit" is certain across both sides, and the onboarding convention is surfaced only where the code now enforces it.

**[260610-1554] rough**

## Character
Cross-track onboarding edit — one operator-facing convention across tracks, not a one-track flip. Mechanical; both upstream design calls have landed.
Side-lane pace: file-disjoint from the spine, mountable in a second officium during spine cloud waits (paddock Execution posture).
Tier: sonnet-delegable.

## Goal
Teach the uniform "tools never commit, gate on a clean tree" convention in onboarding. Today "commit first" is taught only in the kludge track (rbhoct) because kludge was the sole gating verb; the fact-chaining work made the gate uniform, so the Director install-gesture tracks newly refuse on a dirty tree with no prose warning.

## Cinched
- Operator-facing half only — the clean-tree gate. The PREV/fact-chaining mechanism is dispatch-internal; do NOT teach it in onboarding.
- Crash Course (rbhocc, already owns "diagnostic failure") is the concept home; surface the commit-between-steps prompt at the Director tracks (first-build/airgap/bind/graft).

## Scope confirmation — both upstream calls landed; read them before writing prose
- Gate placement: confirm at mount whether the landed gate is pre-condition (refuse-before-write, the kludge model) or downstream-only (install leaves the tree dirty, only charge refuses) — the affected surface shifts accordingly.
- Election shape: confirm whether the bole cutover made election a separate operator step (cutover tracks owe a new step) or a cycle (rename-flip + this gate teaching suffices).

## Discovery recipe
grep the handbook for existing commit/clean-tree prose (rbhoct is the known site); enumerate the Director install-gesture tracks under rbho* that newly gate.

## Done when
The convention is taught once as a Crash Course concept and surfaced at every onboarding track that newly gates; a new operator following the Director tracks is warned to commit before the next gated step rather than hitting an unexplained refusal.

**[260609-1616] rough**

## Character
Cross-track onboarding edit — one operator-facing convention across tracks, not a one-track flip. Mechanical; both upstream design calls have landed.
Side-lane pace: file-disjoint from the spine, mountable in a second officium during spine cloud waits (paddock Execution posture).
Tier: sonnet-delegable.

## Goal
Teach the uniform "tools never commit, gate on a clean tree" convention in onboarding. Today "commit first" is taught only in the kludge track (rbhoct) because kludge was the sole gating verb; the fact-chaining work made the gate uniform, so the Director install-gesture tracks newly refuse on a dirty tree with no prose warning.

## Cinched
- Operator-facing half only — the clean-tree gate. The PREV/fact-chaining mechanism is dispatch-internal; do NOT teach it in onboarding.
- Crash Course (rbhocc, already owns "diagnostic failure") is the concept home; surface the commit-between-steps prompt at the Director tracks (first-build/airgap/bind/graft).

## Scope confirmation — both upstream calls landed; read them before writing prose
- Gate placement: confirm at mount whether the landed gate is pre-condition (refuse-before-write, the kludge model) or downstream-only (install leaves the tree dirty, only charge refuses) — the affected surface shifts accordingly.
- Election shape: confirm whether the bole cutover made election a separate operator step (cutover tracks owe a new step) or a cycle (rename-flip + this gate teaching suffices).

## Discovery recipe
grep the handbook for existing commit/clean-tree prose (rbhoct is the known site); enumerate the Director install-gesture tracks under rbho* that newly gate.

## Done
The convention is taught once as a Crash Course concept and surfaced at every onboarding track that newly gates; a new operator following the Director tracks is warned to commit before the next gated step rather than hitting an unexplained refusal.

**[260606-1158] rough**

## Character
Cross-track onboarding edit — one operator-facing convention across tracks, not a one-track flip. Mechanical once scope confirms; scope rides two upstream design calls.

## Goal
Teach the uniform "tools never commit, gate on a clean tree" convention in onboarding. Today "commit first" is taught only in the kludge track (rbhoct) because kludge is the sole gating verb; once the fact-chaining pace makes the gate uniform, the Director install-gesture tracks newly refuse on a dirty tree with no prose warning.

## Locked
- Operator-facing half only — the clean-tree gate. The PREV/fact-chaining mechanism is dispatch-internal; do NOT teach it in onboarding.
- Crash Course (rbhocc, already owns "diagnostic failure") is the concept home; surface the commit-between-steps prompt at the Director tracks (first-build/airgap/bind/graft).

## Scope contingencies — both upstream, both landed by mount; read them before writing prose
- Gate placement: this pace assumes a pre-condition gate (refuse-before-write, the kludge model). If the fact-chaining pace placed the gate downstream-only (install leaves the tree dirty, only charge refuses — the rbob_charge model), the affected surface shifts.
- Election shape: if the bole cutover made election a SEPARATE operator step, the cutover tracks owe the operator a new step (not just a vocab rename) and this pace's scope widens. If election is a cycle (one command, kludge-style), rename-flip + this gate teaching suffices.

## Discovery recipe
grep the handbook for existing commit/clean-tree prose (rbhoct is the known site); enumerate the Director install-gesture tracks under rbho* that newly gate.

## Done
The convention is taught once as a Crash Course concept and surfaced at every onboarding track that newly gates; a new operator following the Director tracks is warned to commit before the next gated step rather than hitting an unexplained refusal.

### fast-tier-credless-by-construction (₢BHAAl) [abandoned]

**[260611-1316] abandoned**

## Character
Hazard-closure design + implement; small but cross-cutting (theurge suite runner +
credential load path). Tier: sonnet-delegable with care —
the failure mode it closes is a PASSING fast suite that spends money and mutates the depot.

## Goal
Make fast-tier credlessness structural rather than conventional:
the fast suite runner exports a poison marker and the credential-load path refuses under it
(loud die naming the violation), so a fast case that reaches a credential load fails the
suite instead of firing a live build.
Exact shape is a mount-time choice (poison env var is the cheap candidate from the memo);
whatever lands, state the convention where an author of a new fast case will read it.

## Done when
A fast case reaching credential load dies loud by construction; fast suite green;
the convention documented in the theurge/test authoring context.

## Sources
Memos/memo-20260610-heat-BH-fast-tier-credless-by-convention.md
(the near-miss record: the podvm resolve case was one inspection away from live multi-GB
submissions on every fast run; the buorb_immure_resolve_only seam was the spot fix).

**[260610-2116] rough**

## Character
Hazard-closure design + implement; small but cross-cutting (theurge suite runner +
credential load path). Tier: sonnet-delegable with care —
the failure mode it closes is a PASSING fast suite that spends money and mutates the depot.

## Goal
Make fast-tier credlessness structural rather than conventional:
the fast suite runner exports a poison marker and the credential-load path refuses under it
(loud die naming the violation), so a fast case that reaches a credential load fails the
suite instead of firing a live build.
Exact shape is a mount-time choice (poison env var is the cheap candidate from the memo);
whatever lands, state the convention where an author of a new fast case will read it.

## Done when
A fast case reaching credential load dies loud by construction; fast suite green;
the convention documented in the theurge/test authoring context.

## Sources
Memos/memo-20260610-heat-BH-fast-tier-credless-by-convention.md
(the near-miss record: the podvm resolve case was one inspection away from live multi-GB
submissions on every fast run; the buorb_immure_resolve_only seam was the spot fix).

### yoke-colophon-consumer-move (₢BHAAb) [complete]

**[260612-0809] complete**

## Character
Deferred small rename — the election-verb colophon move recorded in the paddock (election lives with its consumer), slated terminal so the gordian knot clears first.
Runs before the vocab scrub so the scrub sweeps any rename stragglers.
Tier: sonnet-delegable.

## Goal
Move yoke out of the Depot family and de-capitalize it: rbw-dY -> a lowercase consumer-family colophon, per the paddock Carried-forward election rule (election is cheap reversible config, operator-committed, never self-committing).

## Cinched
- Exact destination letter is a mount-time layout choice, minted against the live colophon tree.
- Rename only — no behavior change; yoke stays non-committing (RBSDY: the caller commits).
- Sweep the rename across the zipper enrollment, tabtarget-context doc, and onboarding prose.

## Done when
The new colophon routes; rbw-dY gone; zipper/docs/onboarding repointed; `fast` green.
Completion is the grep census, not the site list (the partial-sweep drift lesson):
`grep -rn "rbw-dY" Tools/ tt/` returns zero.

**[260610-1554] rough**

## Character
Deferred small rename — the election-verb colophon move recorded in the paddock (election lives with its consumer), slated terminal so the gordian knot clears first.
Runs before the vocab scrub so the scrub sweeps any rename stragglers.
Tier: sonnet-delegable.

## Goal
Move yoke out of the Depot family and de-capitalize it: rbw-dY -> a lowercase consumer-family colophon, per the paddock Carried-forward election rule (election is cheap reversible config, operator-committed, never self-committing).

## Cinched
- Exact destination letter is a mount-time layout choice, minted against the live colophon tree.
- Rename only — no behavior change; yoke stays non-committing (RBSDY: the caller commits).
- Sweep the rename across the zipper enrollment, tabtarget-context doc, and onboarding prose.

## Done when
The new colophon routes; rbw-dY gone; zipper/docs/onboarding repointed; `fast` green.
Completion is the grep census, not the site list (the partial-sweep drift lesson):
`grep -rn "rbw-dY" Tools/ tt/` returns zero.

**[260609-1617] rough**

## Character
Deferred small rename — the election-verb colophon move recorded in the paddock (election lives with its consumer), slated terminal so the gordian knot clears first.
Runs before the vocab scrub so the scrub sweeps any rename stragglers.
Tier: sonnet-delegable.

## Goal
Move yoke out of the Depot family and de-capitalize it: rbw-dY -> a lowercase consumer-family colophon, per the paddock Carried-forward election rule (election is cheap reversible config, operator-committed, never self-committing).

## Cinched
- Exact destination letter is a mount-time layout choice, minted against the live colophon tree.
- Rename only — no behavior change; yoke stays non-committing (RBSDY: the caller commits).
- Sweep the rename across the zipper enrollment, tabtarget-context doc, and onboarding prose.

## Done when
The new colophon routes; rbw-dY gone; zipper/docs/onboarding repointed; `fast` green.

### terminal-memo-triage (₢BHAAd) [complete]

**[260612-1005] complete**

## Character
Planning pass, not implementation — judgment over a memo corpus, exercised with the operator.
Operator commitment: this pace mounts under a **Fable-class agent only**.

## State (2026-06-11 morning — main triage complete, pace held open for stragglers + closing duties)
The 15-memo corpus is dispositioned with the operator and every memo carries an in-file
`TRIAGED 2026-06-10:` stamp (greppable: `grep -L "TRIAGED" Memos/memo-2026061*.md` finds
any unhandled memo — including the two 2026-06-11 night-ledger memos, which are deliberately
unstamped and await disposition here).
This pace stays open to disposition stragglers and execute the closing duties before the heat retires.

## Straggler list (accumulated from the parallel-lane reports and the ladder night)
- The two 2026-06-11 night-ledger memos (Class-C setIamPolicy write flap;
  vouch poll-ceiling QUEUED conflation) — each poses repair shapes, judge and arrange.
- claude-rbk-acronyms.md RBLD entry: the reserved kind-letters clause still says
  rbldr/rbldw/rbldv are "reserved, no files this heat" — the files exist now.
- RBSLI: "carrying the host-minted touchmark" elides that the slot also carries the full
  envelope staged for the vouch push — one-line prose correction.
- rbtdru_cupel.rs is ~1330 lines, past the RCG 800 stop-and-ask threshold — split decision
  (e.g., python scan into its own module) is an operator call; arrange a pace or decline explicitly.
- Reproducibility-audit wrap residue: the Lode provenance envelope stamps no git fact —
  envelope commit-stamping recorded as a possible itch; disposition it.
- rbgjv02 still subprocess-runs gcloud (adjudicated onto the allowlist with a dated comment);
  the REST/urllib conversion was declined-for-now — decide whether that decline is permanent.
- Dead fact residue from the ladder-surfaced fix: RBF_FACT_RELIQUARY in rbgc_Constants.sh
  (zero consumers) and the rbf_fact_reliquary quoin in RBS0 — inert; assign to the scrub or here.
- Suite-head precondition probes: skirmish leads with canonical-invest, so one Payor RAPT
  expiry or one stochastic IAM flap costs a whole suite attempt — judge a suite-head probe.
- Two orphan conclave Lodes in GAR from failed skirmish attempts (never yoked) — banishable
  cleanup, or left as depot-disposable debris; explicit verdict either way.

## Closing duties (operator-approved 2026-06-10/11)
- Memo shelving: `git mv` fully-dispositioned memos to `Memos/retired/` (basename unchanged) —
  only memos nothing pending references; sweep references first
  (`grep -rn "«basename»" Tools/ Memos/ CLAUDE.md`) and repoint hits (RBSHR cites several);
  add the one-line convention to CLAUDE.md ("a memo path that no longer resolves has retired —
  same basename under Memos/retired/").

## Cinched
- Triage and pace-arrangement only; no fixes land in this pace.
- Memos about non-RB concerns are ignorable; skipping is a valid verdict.
- Every memo gets a recorded disposition — a slated pace, an RBSHR/itch entry, or an
  explicit decline — never silent dismissal.
- Disposition stamps live in-file, never in filenames (path references must not break).
- Shelving directory is `Memos/retired/` — reuses the heat lifecycle word; operator-ratified.

## Done when
Every memo matching `ls Memos/memo-2026061*.md` carries a disposition stamp,
the straggler list above is dispositioned with the operator,
the closing duties have run,
and any paces arising are enrolled in their proper heats.

**[260611-0729] rough**

## Character
Planning pass, not implementation — judgment over a memo corpus, exercised with the operator.
Operator commitment: this pace mounts under a **Fable-class agent only**.

## State (2026-06-11 morning — main triage complete, pace held open for stragglers + closing duties)
The 15-memo corpus is dispositioned with the operator and every memo carries an in-file
`TRIAGED 2026-06-10:` stamp (greppable: `grep -L "TRIAGED" Memos/memo-2026061*.md` finds
any unhandled memo — including the two 2026-06-11 night-ledger memos, which are deliberately
unstamped and await disposition here).
This pace stays open to disposition stragglers and execute the closing duties before the heat retires.

## Straggler list (accumulated from the parallel-lane reports and the ladder night)
- The two 2026-06-11 night-ledger memos (Class-C setIamPolicy write flap;
  vouch poll-ceiling QUEUED conflation) — each poses repair shapes, judge and arrange.
- claude-rbk-acronyms.md RBLD entry: the reserved kind-letters clause still says
  rbldr/rbldw/rbldv are "reserved, no files this heat" — the files exist now.
- RBSLI: "carrying the host-minted touchmark" elides that the slot also carries the full
  envelope staged for the vouch push — one-line prose correction.
- rbtdru_cupel.rs is ~1330 lines, past the RCG 800 stop-and-ask threshold — split decision
  (e.g., python scan into its own module) is an operator call; arrange a pace or decline explicitly.
- Reproducibility-audit wrap residue: the Lode provenance envelope stamps no git fact —
  envelope commit-stamping recorded as a possible itch; disposition it.
- rbgjv02 still subprocess-runs gcloud (adjudicated onto the allowlist with a dated comment);
  the REST/urllib conversion was declined-for-now — decide whether that decline is permanent.
- Dead fact residue from the ladder-surfaced fix: RBF_FACT_RELIQUARY in rbgc_Constants.sh
  (zero consumers) and the rbf_fact_reliquary quoin in RBS0 — inert; assign to the scrub or here.
- Suite-head precondition probes: skirmish leads with canonical-invest, so one Payor RAPT
  expiry or one stochastic IAM flap costs a whole suite attempt — judge a suite-head probe.
- Two orphan conclave Lodes in GAR from failed skirmish attempts (never yoked) — banishable
  cleanup, or left as depot-disposable debris; explicit verdict either way.

## Closing duties (operator-approved 2026-06-10/11)
- Memo shelving: `git mv` fully-dispositioned memos to `Memos/retired/` (basename unchanged) —
  only memos nothing pending references; sweep references first
  (`grep -rn "«basename»" Tools/ Memos/ CLAUDE.md`) and repoint hits (RBSHR cites several);
  add the one-line convention to CLAUDE.md ("a memo path that no longer resolves has retired —
  same basename under Memos/retired/").

## Cinched
- Triage and pace-arrangement only; no fixes land in this pace.
- Memos about non-RB concerns are ignorable; skipping is a valid verdict.
- Every memo gets a recorded disposition — a slated pace, an RBSHR/itch entry, or an
  explicit decline — never silent dismissal.
- Disposition stamps live in-file, never in filenames (path references must not break).
- Shelving directory is `Memos/retired/` — reuses the heat lifecycle word; operator-ratified.

## Done when
Every memo matching `ls Memos/memo-2026061*.md` carries a disposition stamp,
the straggler list above is dispositioned with the operator,
the closing duties have run,
and any paces arising are enrolled in their proper heats.

**[260610-2247] rough**

## Character
Planning pass, not implementation — judgment over a memo corpus, exercised with the operator.
Operator commitment: this pace mounts under a **Fable-class agent only**.

## State (2026-06-10 late evening — main triage complete, pace held open for stragglers)
The 15-memo corpus is dispositioned with the operator and every memo carries an in-file
`TRIAGED 2026-06-10:` stamp (greppable: `grep -L "TRIAGED" Memos/memo-2026061*.md` finds
any unhandled memo). Nine slated-and-landed, one slated-pending, two RBSHR entries,
three declined-with-record.
This pace stays open to disposition stragglers before the heat retires.

## Straggler list (accumulated from the parallel-lane reports and late wraps)
- claude-rbk-acronyms.md RBLD entry: the reserved kind-letters clause still says
  rbldr/rbldw/rbldv are "reserved, no files this heat" — the files exist now.
- RBSLI: "carrying the host-minted touchmark" elides that the slot also carries the full
  envelope staged for the vouch push — one-line prose correction.
- rbtdru_cupel.rs is ~1330 lines, past the RCG 800 stop-and-ask threshold (was 987 before
  the python walk) — split decision (e.g., python scan into its own module) is an
  operator call; arrange a pace or decline explicitly.
- Reproducibility-audit wrap residue: the Lode provenance envelope stamps no git fact —
  envelope commit-stamping recorded as a possible itch; disposition it.
- rbgjv02 still subprocess-runs gcloud (adjudicated onto the allowlist with a dated
  comment at the cupel pace); the REST/urllib conversion was declined-for-now — decide
  whether that decline is permanent (itch/RBSHR) or silent-expiry.

## Cinched
- Triage and pace-arrangement only; no fixes land in this pace.
- Memos about non-RB concerns are ignorable; skipping is a valid verdict.
- Every memo gets a recorded disposition — a slated pace, an RBSHR/itch entry, or an
  explicit decline — never silent dismissal.
- Disposition stamps live in-file, never in filenames (path references must not break).

## Done when
Every memo matching `ls Memos/memo-2026061*.md` carries a disposition stamp,
the straggler list above is dispositioned with the operator,
and any paces arising are enrolled in their proper heats.

**[260610-1554] rough**

## Character
Planning pass, not implementation — judgment over a memo corpus, exercised with the operator.
Operator commitment: this pace mounts under a **Fable-class agent only**.
A second terminal pace (the vocab-finalization scrub) also claims heat-end;
multiple distinct terminal paces are operator-accepted — order between them is the operator's call at mount.

## Goal
Walk every memo the heat tail produced — stamped 20260610 or later
(discovery recipe: `ls Memos/memo-2026061*.md` — the glob covers the tail window if work slips past the 10th),
judge with the operator which findings are worth fixing,
and arrange paces — in this heat's successors or in other heats — for the ones that are.

## Cinched
- Triage and pace-arrangement only; no fixes land in this pace.
- Memos about non-RB concerns are ignorable; skipping is a valid verdict.
- Every memo gets a recorded disposition — a slated pace, an RBSHR/itch entry, or an explicit decline — never silent dismissal.
- Judgment is deferred to this pace by design: the memos were written without verdicts so triage happens once, with the operator, with the whole corpus visible.

## Done when
Every memo matching the discovery recipe carries an operator-agreed disposition,
and any paces arising are enrolled in their proper heats.

**[260610-1018] rough**

## Character
Planning pass, not implementation — judgment over a memo corpus, exercised with the operator.
Operator commitment: this pace mounts under a **Fable-class agent only**.
A second terminal pace (the vocab-finalization scrub) also claims heat-end;
multiple distinct terminal paces are operator-accepted — order between them is the operator's call at mount.

## Goal
Walk every memo stamped with today's date
(discovery recipe: `ls Memos/memo-20260610-*`),
judge with the operator which findings are worth fixing,
and arrange paces — in this heat's successors or in other heats — for the ones that are.

## Cinched
- Triage and pace-arrangement only; no fixes land in this pace.
- Memos about non-RB concerns are ignorable; skipping is a valid verdict.
- Every memo gets a recorded disposition — a slated pace, an RBSHR/itch entry, or an explicit decline — never silent dismissal.
- Judgment is deferred to this pace by design: the memos were written without verdicts so triage happens once, with the operator, with the whole corpus visible.

## Done when
Every memo matching the discovery recipe carries an operator-agreed disposition,
and any paces arising are enrolled in their proper heats.

### envelope-git-commit-stamp (₢BHAAq) [complete]

**[260612-1042] complete**

## Character
Small, chokepointed addition across host substitution and one shared cloud step;
mechanical once the injection point is read.
Sonnet-delegable body with driver verification; one service-tier proof run.

## Goal
The Lode provenance envelope (`:rbi_vouch`) records no git fact,
so it cannot answer which repo state dispatched the capture
(residue from the reproducibility-audit pace; operator elected the improvement at the 2026-06-12 triage).
Add the dispatching HEAD commit to the envelope.
Honesty is already guaranteed upstream:
every capture verb gates `bug_require_clean_tree` before composing its steps,
so HEAD is the product of committed code by construction.

## Cinched
- Inject at the single shared chokepoint — the assemble-and-push cloud step all kinds ride —
  fed by one new substitution through the spine blob,
  so every kind inherits the field from one edit and one fixture proves the mechanism for all.
- Reuse the existing host git-metadata primitive; no new git plumbing.
- Bump the envelope schema version; additive field only — no existing field changes.
- No tidemark in specs; the field-list spec gains the field in present tense.

## Done when
A freshly captured Lode's decoded envelope carries the dispatching commit;
the schema version is bumped and the envelope field-list spec agrees;
`fast` is green and one service-tier lode-lifecycle run proves the field end-to-end against live GAR.

**[260612-0927] rough**

## Character
Small, chokepointed addition across host substitution and one shared cloud step;
mechanical once the injection point is read.
Sonnet-delegable body with driver verification; one service-tier proof run.

## Goal
The Lode provenance envelope (`:rbi_vouch`) records no git fact,
so it cannot answer which repo state dispatched the capture
(residue from the reproducibility-audit pace; operator elected the improvement at the 2026-06-12 triage).
Add the dispatching HEAD commit to the envelope.
Honesty is already guaranteed upstream:
every capture verb gates `bug_require_clean_tree` before composing its steps,
so HEAD is the product of committed code by construction.

## Cinched
- Inject at the single shared chokepoint — the assemble-and-push cloud step all kinds ride —
  fed by one new substitution through the spine blob,
  so every kind inherits the field from one edit and one fixture proves the mechanism for all.
- Reuse the existing host git-metadata primitive; no new git plumbing.
- Bump the envelope schema version; additive field only — no existing field changes.
- No tidemark in specs; the field-list spec gains the field in present tense.

## Done when
A freshly captured Lode's decoded envelope carries the dispatching commit;
the schema version is bumped and the envelope field-list spec agrees;
`fast` is green and one service-tier lode-lifecycle run proves the field end-to-end against live GAR.

### bash-filename-case-consolidation (₢BHAAp) [complete]

**[260612-1055] complete**

Drafted from ₢BbAAL in ₣Bb.

Consolidate every bash implementation filename to one rule:
`«prefix»_«snake_name».sh`, all lowercase, prefer a single word —
retiring BCG's PascalCase carve-outs so no mixed-case `.sh` file remains under `Tools/`.

## Character
Mechanical throughout — the word-shortening judgment calls are already settled (see the ratified name map below).

## Cinched
- One rule, no carve-outs:
  plain modules, decomposition entries (`«prefix»0_`), decomposition clusters, and test-case files all go lowercase.
- Doc files (`ACRONYM-Words.ext`) and tabtarget frontispieces are separate naming grammars — untouched.
- Name map ratified with the operator 2026-06-11.
  Single-word PascalCase basenames simply lowercase (e.g. `rba_Auth` → `rba_auth`).
  The multi-word slimmings are settled — these are decisions, not site discovery,
  and any new offender the mount sweep finds follows the same slim-to-one-word rule:
  `rbfd_FoundryDirectorBuild`→`rbfd_director`, `rbdc_DerivedConstants`→`rbdc_derived`,
  `rbgd_DepotConstants`→`rbgd_depot`, `rbfc0_FoundryCore`→`rbfc0_core`,
  `rbfca_StepAssembly`→`rbfca_assembly`, `rbfcb_BuildHost`→`rbfcb_host`,
  `rbfcg_GarRest`→`rbfcg_gar`, `rbfcv_VesselResolution`→`rbfcv_resolve`,
  `rbfh_FoundryHygiene`→`rbfh_hygiene`, `rbfl0_FoundryLedger`→`rbfl0_ledger`,
  `rbfr_FoundryRetriever`→`rbfr_retriever`, `rbfv_FoundryVerify`→`rbfv_verify`,
  `rbga_ArtifactRegistry`→`rbga_registry`, `rbgl_GarLayout`→`rbgl_layout`,
  `rbgv_AccessProbe`→`rbgv_probe`,
  `bupr_PresentationRegime`→`bupr_regime` (matches the regime-file sibling pattern),
  and the buts test cases `butcbe_bure`, `butcbx_burx`, `butcfc_facts`,
  `butckk_kick`, `butclc_links`, `butcym_yelp`.
- While updating the rbk acronym map's RBLD entries for the new basenames:
  the reserved-letters clause still claims `rbldr`/`rbldw`/`rbldv` have no files —
  promote those three to live entries with their new lowercase basenames;
  `RBLDT` alone stays reserved.
  (Triage disposition 2026-06-12: the map must not claim files don't exist that do.)
- The `ABANDONED-github/` and `FUTURE/` files are already renamed
  (zero-consumer slice landed ahead of this pace) — out of the population.
- Records are exempt from the rename sweep and from the retired-basename grep:
  JJK-managed artifacts (paddocks, dockets, gallops state) and `Memos/` reference
  current basenames as of their writing, by design.
  Other heats are gated on this pace and carry their own resolve-stale-basename guidance —
  do not chase renames into JJK artifacts or memos, and do not count their hits as sweep failures.
  Git history likewise stays untouched.

## Done when
- A case-sensitive sweep for uppercase letters in `.sh` basenames under `Tools/` lands empty.
- A repo-wide grep for each retired basename lands clean in living surfaces —
  sourcing lines, zipper registry, specs, acronym maps, launcher configs —
  with hits in the exempt record surfaces (JJK artifacts, `Memos/`) disregarded per the Cinched exemption.
- BCG's Naming Convention Patterns table states the single lowercase rule plainly,
  with the PascalCase rows (decomposition entry, decomposition cluster, test-case file) recut to match.
- The CLAUDE.md mint table's code-file row (`prefix_Word.ext`) and the rbk acronym map agree with the new names.
- The `fast` test suite passes; run a broader tier if touched files warrant it.

First step at mount: rebuild the population fresh —
enumerate offenders with a case-sensitive sweep (e.g. `find Tools -name '*.sh' | grep '[A-Z]'`)
rather than trusting any slate-time list;
the names themselves are cinched above, so no operator confirmation round is needed
unless the sweep surfaces an offender absent from the map whose slimming is ambiguous.
Case-only renames on macOS's case-insensitive filesystem want `git mv`.
Single-operator caution: land renames and reference updates in the same commit so the tree never sources a missing file.

**[260612-0917] rough**

Drafted from ₢BbAAL in ₣Bb.

Consolidate every bash implementation filename to one rule:
`«prefix»_«snake_name».sh`, all lowercase, prefer a single word —
retiring BCG's PascalCase carve-outs so no mixed-case `.sh` file remains under `Tools/`.

## Character
Mechanical throughout — the word-shortening judgment calls are already settled (see the ratified name map below).

## Cinched
- One rule, no carve-outs:
  plain modules, decomposition entries (`«prefix»0_`), decomposition clusters, and test-case files all go lowercase.
- Doc files (`ACRONYM-Words.ext`) and tabtarget frontispieces are separate naming grammars — untouched.
- Name map ratified with the operator 2026-06-11.
  Single-word PascalCase basenames simply lowercase (e.g. `rba_Auth` → `rba_auth`).
  The multi-word slimmings are settled — these are decisions, not site discovery,
  and any new offender the mount sweep finds follows the same slim-to-one-word rule:
  `rbfd_FoundryDirectorBuild`→`rbfd_director`, `rbdc_DerivedConstants`→`rbdc_derived`,
  `rbgd_DepotConstants`→`rbgd_depot`, `rbfc0_FoundryCore`→`rbfc0_core`,
  `rbfca_StepAssembly`→`rbfca_assembly`, `rbfcb_BuildHost`→`rbfcb_host`,
  `rbfcg_GarRest`→`rbfcg_gar`, `rbfcv_VesselResolution`→`rbfcv_resolve`,
  `rbfh_FoundryHygiene`→`rbfh_hygiene`, `rbfl0_FoundryLedger`→`rbfl0_ledger`,
  `rbfr_FoundryRetriever`→`rbfr_retriever`, `rbfv_FoundryVerify`→`rbfv_verify`,
  `rbga_ArtifactRegistry`→`rbga_registry`, `rbgl_GarLayout`→`rbgl_layout`,
  `rbgv_AccessProbe`→`rbgv_probe`,
  `bupr_PresentationRegime`→`bupr_regime` (matches the regime-file sibling pattern),
  and the buts test cases `butcbe_bure`, `butcbx_burx`, `butcfc_facts`,
  `butckk_kick`, `butclc_links`, `butcym_yelp`.
- While updating the rbk acronym map's RBLD entries for the new basenames:
  the reserved-letters clause still claims `rbldr`/`rbldw`/`rbldv` have no files —
  promote those three to live entries with their new lowercase basenames;
  `RBLDT` alone stays reserved.
  (Triage disposition 2026-06-12: the map must not claim files don't exist that do.)
- The `ABANDONED-github/` and `FUTURE/` files are already renamed
  (zero-consumer slice landed ahead of this pace) — out of the population.
- Records are exempt from the rename sweep and from the retired-basename grep:
  JJK-managed artifacts (paddocks, dockets, gallops state) and `Memos/` reference
  current basenames as of their writing, by design.
  Other heats are gated on this pace and carry their own resolve-stale-basename guidance —
  do not chase renames into JJK artifacts or memos, and do not count their hits as sweep failures.
  Git history likewise stays untouched.

## Done when
- A case-sensitive sweep for uppercase letters in `.sh` basenames under `Tools/` lands empty.
- A repo-wide grep for each retired basename lands clean in living surfaces —
  sourcing lines, zipper registry, specs, acronym maps, launcher configs —
  with hits in the exempt record surfaces (JJK artifacts, `Memos/`) disregarded per the Cinched exemption.
- BCG's Naming Convention Patterns table states the single lowercase rule plainly,
  with the PascalCase rows (decomposition entry, decomposition cluster, test-case file) recut to match.
- The CLAUDE.md mint table's code-file row (`prefix_Word.ext`) and the rbk acronym map agree with the new names.
- The `fast` test suite passes; run a broader tier if touched files warrant it.

First step at mount: rebuild the population fresh —
enumerate offenders with a case-sensitive sweep (e.g. `find Tools -name '*.sh' | grep '[A-Z]'`)
rather than trusting any slate-time list;
the names themselves are cinched above, so no operator confirmation round is needed
unless the sweep surfaces an offender absent from the map whose slimming is ambiguous.
Case-only renames on macOS's case-insensitive filesystem want `git mv`.
Single-operator caution: land renames and reference updates in the same commit so the tree never sources a missing file.

**[260611-1339] rough**

Drafted from ₢BbAAL in ₣Bb.

Consolidate every bash implementation filename to one rule:
`«prefix»_«snake_name».sh`, all lowercase, prefer a single word —
retiring BCG's PascalCase carve-outs so no mixed-case `.sh` file remains under `Tools/`.

## Character
Mechanical throughout — the word-shortening judgment calls are already settled (see the ratified name map below).

## Cinched
- One rule, no carve-outs:
  plain modules, decomposition entries (`«prefix»0_`), decomposition clusters, and test-case files all go lowercase.
- Doc files (`ACRONYM-Words.ext`) and tabtarget frontispieces are separate naming grammars — untouched.
- Name map ratified with the operator 2026-06-11.
  Single-word PascalCase basenames simply lowercase (e.g. `rba_Auth` → `rba_auth`).
  The multi-word slimmings are settled — these are decisions, not site discovery,
  and any new offender the mount sweep finds follows the same slim-to-one-word rule:
  `rbfd_FoundryDirectorBuild`→`rbfd_director`, `rbdc_DerivedConstants`→`rbdc_derived`,
  `rbgd_DepotConstants`→`rbgd_depot`, `rbfc0_FoundryCore`→`rbfc0_core`,
  `rbfca_StepAssembly`→`rbfca_assembly`, `rbfcb_BuildHost`→`rbfcb_host`,
  `rbfcg_GarRest`→`rbfcg_gar`, `rbfcv_VesselResolution`→`rbfcv_resolve`,
  `rbfh_FoundryHygiene`→`rbfh_hygiene`, `rbfl0_FoundryLedger`→`rbfl0_ledger`,
  `rbfr_FoundryRetriever`→`rbfr_retriever`, `rbfv_FoundryVerify`→`rbfv_verify`,
  `rbga_ArtifactRegistry`→`rbga_registry`, `rbgl_GarLayout`→`rbgl_layout`,
  `rbgv_AccessProbe`→`rbgv_probe`,
  `bupr_PresentationRegime`→`bupr_regime` (matches the regime-file sibling pattern),
  and the buts test cases `butcbe_bure`, `butcbx_burx`, `butcfc_facts`,
  `butckk_kick`, `butclc_links`, `butcym_yelp`.
- The `ABANDONED-github/` and `FUTURE/` files are already renamed
  (zero-consumer slice landed ahead of this pace) — out of the population.
- Records are exempt from the rename sweep and from the retired-basename grep:
  JJK-managed artifacts (paddocks, dockets, gallops state) and `Memos/` reference
  current basenames as of their writing, by design.
  Other heats are gated on this pace and carry their own resolve-stale-basename guidance —
  do not chase renames into JJK artifacts or memos, and do not count their hits as sweep failures.
  Git history likewise stays untouched.

## Done when
- A case-sensitive sweep for uppercase letters in `.sh` basenames under `Tools/` lands empty.
- A repo-wide grep for each retired basename lands clean in living surfaces —
  sourcing lines, zipper registry, specs, acronym maps, launcher configs —
  with hits in the exempt record surfaces (JJK artifacts, `Memos/`) disregarded per the Cinched exemption.
- BCG's Naming Convention Patterns table states the single lowercase rule plainly,
  with the PascalCase rows (decomposition entry, decomposition cluster, test-case file) recut to match.
- The CLAUDE.md mint table's code-file row (`prefix_Word.ext`) and the rbk acronym map agree with the new names.
- The `fast` test suite passes; run a broader tier if touched files warrant it.

First step at mount: rebuild the population fresh —
enumerate offenders with a case-sensitive sweep (e.g. `find Tools -name '*.sh' | grep '[A-Z]'`)
rather than trusting any slate-time list;
the names themselves are cinched above, so no operator confirmation round is needed
unless the sweep surfaces an offender absent from the map whose slimming is ambiguous.
Case-only renames on macOS's case-insensitive filesystem want `git mv`.
Single-operator caution: land renames and reference updates in the same commit so the tree never sources a missing file.

**[260611-1334] rough**

Consolidate every bash implementation filename to one rule:
`«prefix»_«snake_name».sh`, all lowercase, prefer a single word —
retiring BCG's PascalCase carve-outs so no mixed-case `.sh` file remains under `Tools/`.

## Character
Mechanical throughout — the word-shortening judgment calls are already settled (see the ratified name map below).

## Cinched
- One rule, no carve-outs:
  plain modules, decomposition entries (`«prefix»0_`), decomposition clusters, and test-case files all go lowercase.
- Doc files (`ACRONYM-Words.ext`) and tabtarget frontispieces are separate naming grammars — untouched.
- Name map ratified with the operator 2026-06-11.
  Single-word PascalCase basenames simply lowercase (e.g. `rba_Auth` → `rba_auth`).
  The multi-word slimmings are settled — these are decisions, not site discovery,
  and any new offender the mount sweep finds follows the same slim-to-one-word rule:
  `rbfd_FoundryDirectorBuild`→`rbfd_director`, `rbdc_DerivedConstants`→`rbdc_derived`,
  `rbgd_DepotConstants`→`rbgd_depot`, `rbfc0_FoundryCore`→`rbfc0_core`,
  `rbfca_StepAssembly`→`rbfca_assembly`, `rbfcb_BuildHost`→`rbfcb_host`,
  `rbfcg_GarRest`→`rbfcg_gar`, `rbfcv_VesselResolution`→`rbfcv_resolve`,
  `rbfh_FoundryHygiene`→`rbfh_hygiene`, `rbfl0_FoundryLedger`→`rbfl0_ledger`,
  `rbfr_FoundryRetriever`→`rbfr_retriever`, `rbfv_FoundryVerify`→`rbfv_verify`,
  `rbga_ArtifactRegistry`→`rbga_registry`, `rbgl_GarLayout`→`rbgl_layout`,
  `rbgv_AccessProbe`→`rbgv_probe`,
  `bupr_PresentationRegime`→`bupr_regime` (matches the regime-file sibling pattern),
  and the buts test cases `butcbe_bure`, `butcbx_burx`, `butcfc_facts`,
  `butckk_kick`, `butclc_links`, `butcym_yelp`.
- The `ABANDONED-github/` and `FUTURE/` files are already renamed
  (zero-consumer slice landed ahead of this pace) — out of the population.
- Records are exempt from the rename sweep and from the retired-basename grep:
  JJK-managed artifacts (paddocks, dockets, gallops state) and `Memos/` reference
  current basenames as of their writing, by design.
  Other heats are gated on this pace and carry their own resolve-stale-basename guidance —
  do not chase renames into JJK artifacts or memos, and do not count their hits as sweep failures.
  Git history likewise stays untouched.

## Done when
- A case-sensitive sweep for uppercase letters in `.sh` basenames under `Tools/` lands empty.
- A repo-wide grep for each retired basename lands clean in living surfaces —
  sourcing lines, zipper registry, specs, acronym maps, launcher configs —
  with hits in the exempt record surfaces (JJK artifacts, `Memos/`) disregarded per the Cinched exemption.
- BCG's Naming Convention Patterns table states the single lowercase rule plainly,
  with the PascalCase rows (decomposition entry, decomposition cluster, test-case file) recut to match.
- The CLAUDE.md mint table's code-file row (`prefix_Word.ext`) and the rbk acronym map agree with the new names.
- The `fast` test suite passes; run a broader tier if touched files warrant it.

First step at mount: rebuild the population fresh —
enumerate offenders with a case-sensitive sweep (e.g. `find Tools -name '*.sh' | grep '[A-Z]'`)
rather than trusting any slate-time list;
the names themselves are cinched above, so no operator confirmation round is needed
unless the sweep surfaces an offender absent from the map whose slimming is ambiguous.
Case-only renames on macOS's case-insensitive filesystem want `git mv`.
Single-operator caution: land renames and reference updates in the same commit so the tree never sources a missing file.

**[260611-1324] rough**

Consolidate every bash implementation filename to one rule:
`«prefix»_«snake_name».sh`, all lowercase, prefer a single word —
retiring BCG's PascalCase carve-outs so no mixed-case `.sh` file remains under `Tools/`.

## Character
Intricate but mechanical;
the only judgment calls are the word-shortening choices on multi-word names.

## Cinched
- One rule, no carve-outs:
  plain modules, decomposition entries (`«prefix»0_`), decomposition clusters, and test-case files all go lowercase.
- Where a PascalCase name is multi-word and the prefix already carries the meaning,
  slim the word part to a single word rather than transliterating to long snake_case
  (the foundry-director-build module is the poster child).
- Doc files (`ACRONYM-Words.ext`) and tabtarget frontispieces are separate naming grammars — untouched.
- The distributed kits (buk/jjk/cmk/vvk/vok/apck) are already compliant except a single buk stray;
  the rename population is essentially rbk-internal, which is why lowercase won over uppercase as the consolidation direction.
- Records are exempt from the rename sweep and from the retired-basename grep:
  JJK-managed artifacts (paddocks, dockets, gallops state) and `Memos/` reference
  current basenames as of their writing, by design.
  Other heats are gated on this pace and carry their own resolve-stale-basename guidance —
  do not chase renames into JJK artifacts or memos, and do not count their hits as sweep failures.
  Git history likewise stays untouched.

## Done when
- A case-sensitive sweep for uppercase letters in `.sh` basenames under `Tools/` lands empty.
- A repo-wide grep for each retired basename lands clean in living surfaces —
  sourcing lines, zipper registry, specs, acronym maps, launcher configs —
  with hits in the exempt record surfaces (JJK artifacts, `Memos/`) disregarded per the Cinched exemption.
- BCG's Naming Convention Patterns table states the single lowercase rule plainly,
  with the PascalCase rows (decomposition entry, decomposition cluster, test-case file) recut to match.
- The CLAUDE.md mint table's code-file row (`prefix_Word.ext`) and the rbk acronym map agree with the new names.
- The `fast` test suite passes; run a broader tier if touched files warrant it.

First step at mount: rebuild the rename map fresh —
enumerate offenders with a case-sensitive sweep (e.g. `find Tools -name '*.sh' | grep '[A-Z]'`)
rather than trusting any slate-time list, then confirm the map with the operator before renaming.
Case-only renames on macOS's case-insensitive filesystem want `git mv`.
Single-operator caution: land renames and reference updates in the same commit so the tree never sources a missing file.

**[260610-1044] rough**

Consolidate every bash implementation filename to one rule:
`«prefix»_«snake_name».sh`, all lowercase, prefer a single word —
retiring BCG's PascalCase carve-outs so no mixed-case `.sh` file remains under `Tools/`.

## Character
Intricate but mechanical;
the only judgment calls are the word-shortening choices on multi-word names.

## Cinched
- One rule, no carve-outs:
  plain modules, decomposition entries (`«prefix»0_`), decomposition clusters, and test-case files all go lowercase.
- Where a PascalCase name is multi-word and the prefix already carries the meaning,
  slim the word part to a single word rather than transliterating to long snake_case
  (the foundry-director-build module is the poster child).
- Doc files (`ACRONYM-Words.ext`) and tabtarget frontispieces are separate naming grammars — untouched.
- The distributed kits (buk/jjk/cmk/vvk/vok/apck) are already compliant except a single buk stray;
  the rename population is essentially rbk-internal, which is why lowercase won over uppercase as the consolidation direction.

## Done when
- A case-sensitive sweep for uppercase letters in `.sh` basenames under `Tools/` lands empty.
- A repo-wide grep for each retired basename lands clean —
  sourcing lines, zipper registry, specs, acronym maps, launcher configs.
- BCG's Naming Convention Patterns table states the single lowercase rule plainly,
  with the PascalCase rows (decomposition entry, decomposition cluster, test-case file) recut to match.
- The CLAUDE.md mint table's code-file row (`prefix_Word.ext`) and the rbk acronym map agree with the new names.
- The `fast` test suite passes; run a broader tier if touched files warrant it.

First step at mount: rebuild the rename map fresh —
enumerate offenders with a case-sensitive sweep (e.g. `find Tools -name '*.sh' | grep '[A-Z]'`)
rather than trusting any slate-time list, then confirm the map with the operator before renaming.
Case-only renames on macOS's case-insensitive filesystem want `git mv`.
Single-operator caution: land renames and reference updates in the same commit so the tree never sources a missing file.

### lode-vocab-finalization-scrub (₢BHAAD) [complete]

**[260612-1133] complete**

## Character
Terminal heat-wide pass — mechanical but exacting. The single danger is a kill-stem that also lives inside a word we keep; the KEEP guards below are load-bearing, not courtesy.
Word-level cross-check ONLY — structural removals (the RBSAE spec, the cupel allowlist, the dead token-fetch snippet) are owned by their own paces, not here, with small NAMED exceptions in the KILL and accreted lists below.
Runs LAST, after every cutover. Never before.
Tier: sonnet-delegable — mechanical sweep, but the KEEP-guard hand-classification needs real reading; not haiku.

## Goal
Once all cutovers have repointed their chokepoints and banished their old GAR namespaces, sweep the whole repo so NO vocabulary superseded by this heat survives — in code, specs, tabtargets, onboarding prose, comments, or constant names — while leaving the introduced Lode vocabulary and the legitimately-overloaded survivors untouched.

## Why this pace is terminal (do not front-run)
This is the delete-old-last tail of the heat. Each cutover deletes its own namespace in-place; until a cutover lands, its surviving old vocabulary IS the rollback. This pace catches only what the per-cutover deletes could not reach — stray prose, comments, cross-refs, a vestigial constant name. It MUST remain the final pace.

## KILL — these stems must not survive (except where a KEEP guard applies)
- `enshrine`, `enshrinement`, `Enshrines`, `*Enshrinements` (tabtarget frontispieces) — superseded by `ensconce` / `bole`
- `rbi_es` (enshrinement GAR namespace) and `rbi_rq` (reliquary GAR namespace) — folded into `rbi_ld`.
  Code-side rbi_rq is already clean (the conclave-verify pace removed the constants and comment examples).
  The two spec-side residues are this pace's NAMED structural exceptions, sanctioned because the cutover left them here by record:
  the RBS0 `gar_reliquaries_namespace` quoin (mapping attribute + definition + any refs) and RBSDE's sibling-namespace prose.
- `reliquary stamp`, and `reliquary` used as a standalone identifier-noun — replaced by the reliquary-kind touchmark
- `rbfl_inscribe` and the reliquary-mirror sense of `inscribe` — absorbed into `conclave`
- constant names carrying `ENSHRINE` (e.g. `ZRBFC_BUILD_POLL_CEILING_ENSHRINE`) — verb-neutralize the name
- `rbf_fact_lode` quoin and the `<stamp>.lode` multi-form capture-file — retired in favor of single-form chaining facts; catch any stragglers.
- `RBF_FACT_RELIQUARY` constant (rbgc_Constants.sh, zero consumers) and the `rbf_fact_reliquary` quoin (RBS0, full three-part removal) — dead since 2274b2ac2; sibling of the rbf_fact_lode kill. Clean removal, no skidmark — the commit carries the history.
- Stale "skopeo" prose describing the BOLE path (now gcrane) — the crane-embrace pace converted the code but not the cross-referencing prose: grep "skopeo" for bole references (RBS0, RBSLC, RBSLU, claude-rbk-acronyms.md) and correct them.
- Retired image-family variants surviving in prose/docs: `iar`/`irr`/`iwr`/`iJr` (gone at the inscribe cutover) and `iwe`/`iJe` (subsumed by the path-polymorphic backdoor) — catch stragglers only; the removals are those paces' work.

## Accreted at triage — small named corrections riding the sweep
- RBSLI: the slot described as "carrying the host-minted touchmark" also carries the full provenance envelope staged for the vouch push — one-line prose correction so the spec states the whole data flow.
- Cupel gcloud allowlist comment (rbtdru, ZRBTDRU_GCB_ALLOWED): drain the skidmark — keep the present-tense rationale (native in the cloud-sdk builder), drop the adjudication date and the "REST conversion deferred" clause; the exception is accepted permanently and the commit message carries that history.

## KEEP — never flag these (introduced or legitimately retained)
- New Lode vocab: `lode`, `rbi_ld`, `rbld*`, `RBSL*`, `bole`, `ensconce`, `divine`, `banish`, `touchmark`, `augur` (`rbw-la`), the path-polymorphic image-backdoor verbs and their colophons, and the introduced RBS0 quoins.
- `reliquary` as a kind name survives (it is `conclave`'s kind) — only the standalone noun / "stamp" identifier dies.
- `rbrd_inscribe` (depot tripwire, `rbw-rdi`) — different operation, OUT of scope.
- `rbfd_mirror` (bind vessel-mode copy, made-side) — the verb stays; it is now gcrane-based.
- Sanctioned tool floor: `crane`/`gcrane`, `curl`, `gpg`. The `gcloud` allowlist entry is a permanent accepted exception (see Accreted) — never flag it.
- Surviving skopeo MENTIONS that are NOT cohort/invocation: FUTURE/RBSPV's podvm tool-survey (candidate/rejected), CBG's non-ambient-auth teaching example, RBSOB/RBSCB historical "superseded" notes, RBSIJ's generic multi-platform-pusher example. EXEMPT all.

## Tool cross-check (crane embrace) — INVOCATIONS only
- zero `skopeo` INVOCATION anywhere in Cloud Build steps, capture or made-side, AND zero skopeo cohort MANIFEST tool-ref (the skopeo-reliquary-eviction removed both). The cupel allowlist + dead token-fetch snippet are that pace's job, not this one.
- zero `docker pull/tag/push` or `buildx` INVOCATION in capture + mirror steps (`Tools/rbk/rbgjl`, `rbgjs`, `rbgjm`). NOTE: `docker` may legitimately persist as a captured cohort tool-ref (MANIFEST data) — that is data, not an invocation; do not flag.
- Recipe: `grep -rnE 'skopeo|docker (pull|tag|push)|buildx' Tools/rbk/rbgjl Tools/rbk/rbgjs Tools/rbk/rbgjm tt/` → classify each hit invocation-vs-cohort-ref.

## Discovery recipe
`grep -rinE 'enshrine|rbi_es|rbi_rq|reliquary stamp|rbfl_inscribe|rbf_fact_lode|rbf_fact_reliquary|\.lode\b' Tools/ tt/` → expect zero. For bare `inscribe`, bare `reliquary`, and bare `skopeo` (all overloaded), grep and hand-classify every hit against the KEEP guards. This list ACCRETES as cutovers land.

## Done when
The recipe returns zero un-guarded hits; the invocation cross-check is clean; the accreted corrections have landed; every KEEP term (including the exempt skopeo mentions and the gcloud allowlist entry) is intact; `fast` plus the relevant suites stay green.

**[260612-0951] rough**

## Character
Terminal heat-wide pass — mechanical but exacting. The single danger is a kill-stem that also lives inside a word we keep; the KEEP guards below are load-bearing, not courtesy.
Word-level cross-check ONLY — structural removals (the RBSAE spec, the cupel allowlist, the dead token-fetch snippet) are owned by their own paces, not here, with small NAMED exceptions in the KILL and accreted lists below.
Runs LAST, after every cutover. Never before.
Tier: sonnet-delegable — mechanical sweep, but the KEEP-guard hand-classification needs real reading; not haiku.

## Goal
Once all cutovers have repointed their chokepoints and banished their old GAR namespaces, sweep the whole repo so NO vocabulary superseded by this heat survives — in code, specs, tabtargets, onboarding prose, comments, or constant names — while leaving the introduced Lode vocabulary and the legitimately-overloaded survivors untouched.

## Why this pace is terminal (do not front-run)
This is the delete-old-last tail of the heat. Each cutover deletes its own namespace in-place; until a cutover lands, its surviving old vocabulary IS the rollback. This pace catches only what the per-cutover deletes could not reach — stray prose, comments, cross-refs, a vestigial constant name. It MUST remain the final pace.

## KILL — these stems must not survive (except where a KEEP guard applies)
- `enshrine`, `enshrinement`, `Enshrines`, `*Enshrinements` (tabtarget frontispieces) — superseded by `ensconce` / `bole`
- `rbi_es` (enshrinement GAR namespace) and `rbi_rq` (reliquary GAR namespace) — folded into `rbi_ld`.
  Code-side rbi_rq is already clean (the conclave-verify pace removed the constants and comment examples).
  The two spec-side residues are this pace's NAMED structural exceptions, sanctioned because the cutover left them here by record:
  the RBS0 `gar_reliquaries_namespace` quoin (mapping attribute + definition + any refs) and RBSDE's sibling-namespace prose.
- `reliquary stamp`, and `reliquary` used as a standalone identifier-noun — replaced by the reliquary-kind touchmark
- `rbfl_inscribe` and the reliquary-mirror sense of `inscribe` — absorbed into `conclave`
- constant names carrying `ENSHRINE` (e.g. `ZRBFC_BUILD_POLL_CEILING_ENSHRINE`) — verb-neutralize the name
- `rbf_fact_lode` quoin and the `<stamp>.lode` multi-form capture-file — retired in favor of single-form chaining facts; catch any stragglers.
- `RBF_FACT_RELIQUARY` constant (rbgc_Constants.sh, zero consumers) and the `rbf_fact_reliquary` quoin (RBS0, full three-part removal) — dead since 2274b2ac2; sibling of the rbf_fact_lode kill. Clean removal, no skidmark — the commit carries the history.
- Stale "skopeo" prose describing the BOLE path (now gcrane) — the crane-embrace pace converted the code but not the cross-referencing prose: grep "skopeo" for bole references (RBS0, RBSLC, RBSLU, claude-rbk-acronyms.md) and correct them.
- Retired image-family variants surviving in prose/docs: `iar`/`irr`/`iwr`/`iJr` (gone at the inscribe cutover) and `iwe`/`iJe` (subsumed by the path-polymorphic backdoor) — catch stragglers only; the removals are those paces' work.

## Accreted at triage — small named corrections riding the sweep
- RBSLI: the slot described as "carrying the host-minted touchmark" also carries the full provenance envelope staged for the vouch push — one-line prose correction so the spec states the whole data flow.
- Cupel gcloud allowlist comment (rbtdru, ZRBTDRU_GCB_ALLOWED): drain the skidmark — keep the present-tense rationale (native in the cloud-sdk builder), drop the adjudication date and the "REST conversion deferred" clause; the exception is accepted permanently and the commit message carries that history.

## KEEP — never flag these (introduced or legitimately retained)
- New Lode vocab: `lode`, `rbi_ld`, `rbld*`, `RBSL*`, `bole`, `ensconce`, `divine`, `banish`, `touchmark`, `augur` (`rbw-la`), the path-polymorphic image-backdoor verbs and their colophons, and the introduced RBS0 quoins.
- `reliquary` as a kind name survives (it is `conclave`'s kind) — only the standalone noun / "stamp" identifier dies.
- `rbrd_inscribe` (depot tripwire, `rbw-rdi`) — different operation, OUT of scope.
- `rbfd_mirror` (bind vessel-mode copy, made-side) — the verb stays; it is now gcrane-based.
- Sanctioned tool floor: `crane`/`gcrane`, `curl`, `gpg`. The `gcloud` allowlist entry is a permanent accepted exception (see Accreted) — never flag it.
- Surviving skopeo MENTIONS that are NOT cohort/invocation: FUTURE/RBSPV's podvm tool-survey (candidate/rejected), CBG's non-ambient-auth teaching example, RBSOB/RBSCB historical "superseded" notes, RBSIJ's generic multi-platform-pusher example. EXEMPT all.

## Tool cross-check (crane embrace) — INVOCATIONS only
- zero `skopeo` INVOCATION anywhere in Cloud Build steps, capture or made-side, AND zero skopeo cohort MANIFEST tool-ref (the skopeo-reliquary-eviction removed both). The cupel allowlist + dead token-fetch snippet are that pace's job, not this one.
- zero `docker pull/tag/push` or `buildx` INVOCATION in capture + mirror steps (`Tools/rbk/rbgjl`, `rbgjs`, `rbgjm`). NOTE: `docker` may legitimately persist as a captured cohort tool-ref (MANIFEST data) — that is data, not an invocation; do not flag.
- Recipe: `grep -rnE 'skopeo|docker (pull|tag|push)|buildx' Tools/rbk/rbgjl Tools/rbk/rbgjs Tools/rbk/rbgjm tt/` → classify each hit invocation-vs-cohort-ref.

## Discovery recipe
`grep -rinE 'enshrine|rbi_es|rbi_rq|reliquary stamp|rbfl_inscribe|rbf_fact_lode|rbf_fact_reliquary|\.lode\b' Tools/ tt/` → expect zero. For bare `inscribe`, bare `reliquary`, and bare `skopeo` (all overloaded), grep and hand-classify every hit against the KEEP guards. This list ACCRETES as cutovers land.

## Done when
The recipe returns zero un-guarded hits; the invocation cross-check is clean; the accreted corrections have landed; every KEEP term (including the exempt skopeo mentions and the gcloud allowlist entry) is intact; `fast` plus the relevant suites stay green.

**[260610-1554] rough**

## Character
Terminal heat-wide pass — mechanical but exacting. The single danger is a kill-stem that also lives inside a word we keep; the KEEP guards below are load-bearing, not courtesy.
Word-level cross-check ONLY — structural removals (the RBSAE spec, the cupel allowlist, the dead token-fetch snippet) are owned by their own paces, not here, with two small NAMED exceptions in the KILL list below.
Runs LAST, after every cutover. Never before.
Tier: sonnet-delegable — mechanical sweep, but the KEEP-guard hand-classification needs real reading; not haiku.

## Goal
Once all cutovers have repointed their chokepoints and banished their old GAR namespaces, sweep the whole repo so NO vocabulary superseded by this heat survives — in code, specs, tabtargets, onboarding prose, comments, or constant names — while leaving the introduced Lode vocabulary and the legitimately-overloaded survivors untouched.

## Why this pace is terminal (do not front-run)
This is the delete-old-last tail of the heat. Each cutover deletes its own namespace in-place; until a cutover lands, its surviving old vocabulary IS the rollback. This pace catches only what the per-cutover deletes could not reach — stray prose, comments, cross-refs, a vestigial constant name. It MUST remain the final pace.

## KILL — these stems must not survive (except where a KEEP guard applies)
- `enshrine`, `enshrinement`, `Enshrines`, `*Enshrinements` (tabtarget frontispieces) — superseded by `ensconce` / `bole`
- `rbi_es` (enshrinement GAR namespace) and `rbi_rq` (reliquary GAR namespace) — folded into `rbi_ld`.
  Code-side rbi_rq is already clean (the conclave-verify pace removed the constants and comment examples).
  The two spec-side residues are this pace's NAMED structural exceptions, sanctioned because the cutover left them here by record:
  the RBS0 `gar_reliquaries_namespace` quoin (mapping attribute + definition + any refs) and RBSDE's sibling-namespace prose.
- `reliquary stamp`, and `reliquary` used as a standalone identifier-noun — replaced by the reliquary-kind touchmark
- `rbfl_inscribe` and the reliquary-mirror sense of `inscribe` — absorbed into `conclave`
- constant names carrying `ENSHRINE` (e.g. `ZRBFC_BUILD_POLL_CEILING_ENSHRINE`) — verb-neutralize the name
- `rbf_fact_lode` quoin and the `<stamp>.lode` multi-form capture-file — retired in favor of single-form chaining facts; catch any stragglers.
- Stale "skopeo" prose describing the BOLE path (now gcrane) — the crane-embrace pace converted the code but not the cross-referencing prose: grep "skopeo" for bole references (RBS0, RBSLC, RBSLU, claude-rbk-acronyms.md) and correct them.
- Retired image-family variants surviving in prose/docs: `iar`/`irr`/`iwr`/`iJr` (gone at the inscribe cutover) and `iwe`/`iJe` (subsumed by the path-polymorphic backdoor) — catch stragglers only; the removals are those paces' work.

## KEEP — never flag these (introduced or legitimately retained)
- New Lode vocab: `lode`, `rbi_ld`, `rbld*`, `RBSL*`, `bole`, `ensconce`, `divine`, `banish`, `touchmark`, `augur` (`rbw-la`), the path-polymorphic image-backdoor verbs and their colophons, and the introduced RBS0 quoins.
- `reliquary` as a kind name survives (it is `conclave`'s kind) — only the standalone noun / "stamp" identifier dies.
- `rbrd_inscribe` (depot tripwire, `rbw-rdi`) — different operation, OUT of scope.
- `rbfd_mirror` (bind vessel-mode copy, made-side) — the verb stays; it is now gcrane-based.
- Sanctioned tool floor: `crane`/`gcrane`, `curl`, `gpg`.
- Surviving skopeo MENTIONS that are NOT cohort/invocation: FUTURE/RBSPV's podvm tool-survey (candidate/rejected), CBG's non-ambient-auth teaching example, RBSOB/RBSCB historical "superseded" notes, RBSIJ's generic multi-platform-pusher example. EXEMPT all.

## Tool cross-check (crane embrace) — INVOCATIONS only
- zero `skopeo` INVOCATION anywhere in Cloud Build steps, capture or made-side, AND zero skopeo cohort MANIFEST tool-ref (the skopeo-reliquary-eviction removed both). The cupel allowlist + dead token-fetch snippet are that pace's job, not this one.
- zero `docker pull/tag/push` or `buildx` INVOCATION in capture + mirror steps (`Tools/rbk/rbgjl`, `rbgjs`, `rbgjm`). NOTE: `docker` may legitimately persist as a captured cohort tool-ref (MANIFEST data) — that is data, not an invocation; do not flag.
- Recipe: `grep -rnE 'skopeo|docker (pull|tag|push)|buildx' Tools/rbk/rbgjl Tools/rbk/rbgjs Tools/rbk/rbgjm tt/` → classify each hit invocation-vs-cohort-ref.

## Discovery recipe
`grep -rinE 'enshrine|rbi_es|rbi_rq|reliquary stamp|rbfl_inscribe|rbf_fact_lode|\.lode\b' Tools/ tt/` → expect zero. For bare `inscribe`, bare `reliquary`, and bare `skopeo` (all overloaded), grep and hand-classify every hit against the KEEP guards. This list ACCRETES as cutovers land.

## Done when
The recipe returns zero un-guarded hits; the invocation cross-check is clean; every KEEP term (including the exempt skopeo mentions) is intact; `fast` plus the relevant suites stay green.

**[260609-1616] rough**

## Character
Terminal heat-wide pass — mechanical but exacting. The single danger is a kill-stem that also lives inside a word we keep; the KEEP guards below are load-bearing, not courtesy.
Word-level cross-check ONLY — structural removals (the RBSAE spec, the cupel allowlist, the dead token-fetch snippet) are owned by their own paces, not here. Runs LAST, after every cutover. Never before.
Tier: sonnet-delegable — mechanical sweep, but the KEEP-guard hand-classification needs real reading; not haiku.

## Goal
Once all cutovers have repointed their chokepoints and banished their old GAR namespaces, sweep the whole repo so NO vocabulary superseded by this heat survives — in code, specs, tabtargets, onboarding prose, comments, or constant names — while leaving the introduced Lode vocabulary and the legitimately-overloaded survivors untouched.

## Why this pace is terminal (do not front-run)
This is the delete-old-last tail of the heat. Each cutover deletes its own namespace in-place; until a cutover lands, its surviving old vocabulary IS the rollback. This pace catches only what the per-cutover deletes could not reach — stray prose, comments, cross-refs, a vestigial constant name. It MUST remain the final pace.

## KILL — these stems must not survive (except where a KEEP guard applies)
- `enshrine`, `enshrinement`, `Enshrines`, `*Enshrinements` (tabtarget frontispieces) — superseded by `ensconce` / `bole`
- `rbi_es` (enshrinement GAR namespace) and `rbi_rq` (reliquary GAR namespace) — folded into `rbi_ld`
- `reliquary stamp`, and `reliquary` used as a standalone identifier-noun — replaced by the reliquary-kind touchmark
- `rbfl_inscribe` and the reliquary-mirror sense of `inscribe` — absorbed into `conclave`
- constant names carrying `ENSHRINE` (e.g. `ZRBFC_BUILD_POLL_CEILING_ENSHRINE`) — verb-neutralize the name
- `rbf_fact_lode` quoin and the `<stamp>.lode` multi-form capture-file — retired in favor of single-form chaining facts; catch any stragglers.
- Stale "skopeo" prose describing the BOLE path (now gcrane) — the skopeo-eviction pace converted the code but not the cross-referencing prose: grep "skopeo" for bole references (RBS0, RBSLC, RBSLU, claude-rbk-acronyms.md) and correct them.
- Retired image-family variants surviving in prose/docs: `iar`/`irr`/`iwr`/`iJr` (gone at the inscribe cutover) and `iwe`/`iJe` (subsumed by the path-polymorphic backdoor) — catch stragglers only; the removals are those paces' work.

## KEEP — never flag these (introduced or legitimately retained)
- New Lode vocab: `lode`, `rbi_ld`, `rbld*`, `RBSL*`, `bole`, `ensconce`, `divine`, `banish`, `touchmark`, `augur` (`rbw-la`), the path-polymorphic image-backdoor verbs and their colophons, and the introduced RBS0 quoins.
- `reliquary` as a kind name survives (it is `conclave`'s kind) — only the standalone noun / "stamp" identifier dies.
- `rbrd_inscribe` (depot tripwire, `rbw-rdi`) — different operation, OUT of scope.
- `rbfd_mirror` (bind vessel-mode copy, made-side) — the verb stays; it is now gcrane-based.
- Sanctioned tool floor: `crane`/`gcrane`, `curl`, `gpg`.
- Surviving skopeo MENTIONS that are NOT cohort/invocation: FUTURE/RBSPV's podvm tool-survey (candidate/rejected), CBG's non-ambient-auth teaching example, RBSOB/RBSCB historical "superseded" notes, RBSIJ's generic multi-platform-pusher example. EXEMPT all.

## Tool cross-check (crane embrace) — INVOCATIONS only
- zero `skopeo` INVOCATION anywhere in Cloud Build steps, capture or made-side, AND zero skopeo cohort MANIFEST tool-ref (the skopeo-reliquary-eviction removed both). The cupel allowlist + dead token-fetch snippet are that pace's job, not this one.
- zero `docker pull/tag/push` or `buildx` INVOCATION in capture + mirror steps (`Tools/rbk/rbgjl`, `rbgjs`, `rbgjm`). NOTE: `docker` may legitimately persist as a captured cohort tool-ref (MANIFEST data) — that is data, not an invocation; do not flag.
- Recipe: `grep -rnE 'skopeo|docker (pull|tag|push)|buildx' Tools/rbk/rbgjl Tools/rbk/rbgjs Tools/rbk/rbgjm tt/` → classify each hit invocation-vs-cohort-ref.

## Discovery recipe
`grep -rinE 'enshrine|rbi_es|rbi_rq|reliquary stamp|rbfl_inscribe|rbf_fact_lode|\.lode\b' Tools/ tt/` → expect zero. For bare `inscribe`, bare `reliquary`, and bare `skopeo` (all overloaded), grep and hand-classify every hit against the KEEP guards. This list ACCRETES as cutovers land.

## Done when
The recipe returns zero un-guarded hits; the invocation cross-check is clean; every KEEP term (including the exempt skopeo mentions) is intact; `fast` plus the relevant suites stay green.

**[260609-0617] rough**

## Character
Terminal heat-wide pass — mechanical but exacting. The single danger is a kill-stem that
also lives inside a word we keep; the KEEP guards below are load-bearing, not courtesy.
Word-level cross-check ONLY — structural removals (the RBSAE spec, the cupel allowlist,
the dead token-fetch snippet) are owned by their own paces, not here. Runs LAST, after
every cutover. Never before.

## Goal
Once all cutovers have repointed their chokepoints and banished their old GAR namespaces,
sweep the whole repo so NO vocabulary superseded by this heat survives — in code, specs,
tabtargets, onboarding prose, comments, or constant names — while leaving the introduced
Lode vocabulary and the legitimately-overloaded survivors untouched.

## Why this pace is terminal (do not front-run)
This is the delete-old-last tail of the heat. Each cutover deletes its own namespace
in-place; until a cutover lands, its surviving old vocabulary IS the rollback. This pace
catches only what the per-cutover deletes could not reach — stray prose, comments,
cross-refs, a vestigial constant name. It MUST remain the final pace.

## KILL — these stems must not survive (except where a KEEP guard applies)
- `enshrine`, `enshrinement`, `Enshrines`, `*Enshrinements` (tabtarget frontispieces) —
  superseded by `ensconce` / `bole`
- `rbi_es` (enshrinement GAR namespace) and `rbi_rq` (reliquary GAR namespace) — folded
  into `rbi_ld`
- `reliquary stamp`, and `reliquary` used as a standalone identifier-noun — replaced by
  the reliquary-kind touchmark
- `rbfl_inscribe` and the reliquary-mirror sense of `inscribe` — absorbed into `conclave`
- constant names carrying `ENSHRINE` (e.g. `ZRBFC_BUILD_POLL_CEILING_ENSHRINE`) —
  verb-neutralize the name
- `rbf_fact_lode` quoin and the `<stamp>.lode` multi-form capture-file — retired in favor
  of single-form chaining facts; catch any stragglers.
- Stale "skopeo" prose describing the BOLE path (now gcrane) — the skopeo-eviction pace
  converted the code but not the cross-referencing prose: grep "skopeo" for bole
  references (RBS0, RBSLC, RBSLU, claude-rbk-acronyms.md) and correct them.

## KEEP — never flag these (introduced or legitimately retained)
- New Lode vocab: `lode`, `rbi_ld`, `rbld*`, `RBSL*`, `bole`, `ensconce`, `divine`,
  `banish`, `touchmark`, `augur` (`rbw-la`), and the introduced RBS0 quoins.
- `reliquary` as a kind name survives (it is `conclave`'s kind) — only the standalone
  noun / "stamp" identifier dies.
- `rbrd_inscribe` (depot tripwire, `rbw-rdi`) — different operation, OUT of scope.
- `rbfd_mirror` (bind vessel-mode copy, made-side) — the verb stays; it is now
  gcrane-based.
- Sanctioned tool floor: `crane`/`gcrane`, `curl`, `gpg`.
- Surviving skopeo MENTIONS that are NOT cohort/invocation: FUTURE/RBSPV's podvm
  tool-survey (candidate/rejected), CBG's non-ambient-auth teaching example, RBSOB/RBSCB
  historical "superseded" notes, RBSIJ's generic multi-platform-pusher example. EXEMPT
  all.

## Tool cross-check (crane embrace) — INVOCATIONS only
- zero `skopeo` INVOCATION anywhere in Cloud Build steps, capture or made-side, AND zero
  skopeo cohort MANIFEST tool-ref (the skopeo-reliquary-eviction removed both). The cupel
  allowlist + dead token-fetch snippet are that pace's job, not this one.
- zero `docker pull/tag/push` or `buildx` INVOCATION in capture + mirror steps
  (`Tools/rbk/rbgjl`, `rbgjs`, `rbgjm`). NOTE: `docker` may legitimately persist as a
  captured cohort tool-ref (MANIFEST data) — that is data, not an invocation; do not flag.
- Recipe: `grep -rnE 'skopeo|docker (pull|tag|push)|buildx' Tools/rbk/rbgjl Tools/rbk/rbgjs Tools/rbk/rbgjm tt/` → classify each hit invocation-vs-cohort-ref.

## Discovery recipe
`grep -rinE 'enshrine|rbi_es|rbi_rq|reliquary stamp|rbfl_inscribe|rbf_fact_lode|\.lode\b' Tools/ tt/` → expect zero. For bare `inscribe`, bare `reliquary`, and bare `skopeo` (all overloaded), grep and hand-classify every hit against the KEEP guards. This list ACCRETES as cutovers land.

## Done when
The recipe returns zero un-guarded hits; the invocation cross-check is clean; every KEEP
term (including the exempt skopeo mentions) is intact; `fast` plus the relevant suites
stay green.

**[260608-2021] rough**

## Character
Terminal heat-wide pass — mechanical but exacting. The single danger is a kill-stem that also lives inside a word we keep; the KEEP guards below are load-bearing, not courtesy. Runs LAST, after every cutover. Never before.

## Goal
Once all cutovers have repointed their chokepoints and banished their old GAR namespaces, sweep the whole repo so NO vocabulary superseded by this heat survives — in code, specs, tabtargets, onboarding prose, comments, or constant names — while leaving the introduced Lode vocabulary and the legitimately-overloaded survivors untouched.

## Why this pace is terminal (do not front-run)
This is the delete-old-last tail of the heat. Each cutover deletes its own namespace in-place; until a cutover lands, its surviving old vocabulary IS the rollback. This pace catches only what the per-cutover deletes could not reach — stray prose, comments, cross-refs, a mis-filed spec, a vestigial constant name. It MUST remain the final pace; nothing scrubs shared vocabulary before that vocabulary's cutover has landed.

## KILL — these stems must not survive (except where a KEEP guard applies)
- `enshrine`, `enshrinement`, `Enshrines`, `*Enshrinements` (tabtarget frontispieces) — superseded by `ensconce` / `bole`
- `rbi_es` — enshrinement GAR namespace, folded into `rbi_ld`
- `rbi_rq` — reliquary GAR namespace, folded into `rbi_ld`
- `reliquary stamp`, and `reliquary` used as a standalone identifier-noun — replaced by the reliquary-kind **touchmark**
- `rbfl_inscribe` and the reliquary-mirror sense of `inscribe` — absorbed into `conclave`
- `RBSAE-ark_enshrine` (whole file) and its `rbtgo_ark_enshrine` quoin — superseded by `RBSLE` / `rbtgo_lode_ensconce`. (RBSAE is mis-filed under `ark_` though it is fetched-side base capture; delete it, do not adapt it.)
- constant names carrying `ENSHRINE` (e.g. `ZRBFC_BUILD_POLL_CEILING_ENSHRINE`) — verb-neutralize the name
- `rbf_fact_lode` quoin and the `<stamp>.lode` multi-form capture-file — retired in favor of single-form chaining facts (a touchmark value-fact + a kind-brand enum); catch any stragglers the fact-chaining pace's RBSLE reslate left in place.

## KEEP — never flag these (introduced or legitimately retained)
- New Lode vocab: `lode`, `rbi_ld`, `rbld*` (the family: rbld0_/rblds_/rbldb_/rbldl_), `RBSL*`, `bole`, `ensconce`, `divine`, `banish`, `touchmark`, the inspect verb `augur` (`rbw-la`, now minted), and the introduced RBS0 quoins (`rbtga_lode`, `rbtga_touchmark`, `rbtga_lode_vouch`, `gar_lodes_namespace`, `rbst_touchmark`, `rbtgog_lode`, `rbtgo_lode_*`)
- `reliquary` as a **kind name** survives (it is `conclave`'s kind) — only the standalone noun / "stamp" identifier dies
- `rbrd_inscribe` (depot tripwire, `rbw-rdi`) — different operation, OUT of scope, stays
- `rbfd_mirror` (bind vessel-mode copy, made-side) — the verb stays; it is now crane-based (the skopeo-reliquary eviction converted it off skopeo). Only its skopeo dependency was removed, not the operation.
- Sanctioned tool floor: `crane`, `curl`, `gpg` — never flag these (see Tool cross-check).

## Tool cross-check (crane embrace)
Beyond vocabulary, confirm the crane embrace fully landed:
- **zero `skopeo`** anywhere in Cloud Build steps — invocation OR captured cohort tool-ref. The reliquary eviction removed both; a surviving `skopeo` is a missed eviction.
- **zero `docker pull/tag/push` or `buildx` invocations** in the capture + mirror steps (`Tools/rbk/rbgjl`, `rbgjs`, `rbgjm`) — all crane now. NOTE: `docker` may legitimately persist as a captured cohort *tool-ref* (MANIFEST data the made-side build consumes) — that is data, not an invocation; do not flag it.
- Recipe: `grep -rnE 'skopeo|docker (pull|tag|push)|buildx' Tools/rbk/rbgjl Tools/rbk/rbgjs Tools/rbk/rbgjm tt/` → classify each hit invocation-vs-cohort-ref against the note above.

## Discovery recipe
`grep -rinE 'enshrine|rbi_es|rbi_rq|reliquary stamp|rbfl_inscribe|rbf_fact_lode|\.lode\b' Tools/ tt/` → expect zero. For bare `inscribe` and bare `reliquary` (both overloaded), grep and hand-classify every hit against the KEEP guards above. This list ACCRETES: as each cutover lands, append any stragglers it could not delete in place.

## Done
The recipe returns zero un-guarded hits; the tool cross-check is clean; every KEEP term is intact; `fast` plus the relevant suites stay green. The heat's superseded vocabulary and the evicted tools are gone from the repo.

**[260606-1153] rough**

## Character
Terminal heat-wide pass — mechanical but exacting. The single danger is a kill-stem that also lives inside a word we keep; the KEEP guards below are load-bearing, not courtesy. Runs LAST, after every cutover. Never before.

## Goal
Once all cutovers have repointed their chokepoints and banished their old GAR namespaces, sweep the whole repo so NO vocabulary superseded by this heat survives — in code, specs, tabtargets, onboarding prose, comments, or constant names — while leaving the introduced Lode vocabulary and the legitimately-overloaded survivors untouched.

## Why this pace is terminal (do not front-run)
This is the delete-old-last tail of the heat. Each cutover deletes its own namespace in-place; until a cutover lands, its surviving old vocabulary IS the rollback. This pace catches only what the per-cutover deletes could not reach — stray prose, comments, cross-refs, a mis-filed spec, a vestigial constant name. It MUST remain the final pace; nothing scrubs shared vocabulary before that vocabulary's cutover has landed.

## KILL — these stems must not survive (except where a KEEP guard applies)
- `enshrine`, `enshrinement`, `Enshrines`, `*Enshrinements` (tabtarget frontispieces) — superseded by `ensconce` / `bole`
- `rbi_es` — enshrinement GAR namespace, folded into `rbi_ld`
- `rbi_rq` — reliquary GAR namespace, folded into `rbi_ld`
- `reliquary stamp`, and `reliquary` used as a standalone identifier-noun — replaced by the reliquary-kind **touchmark**
- `rbfl_inscribe` and the reliquary-mirror sense of `inscribe` — absorbed into `fetter` / `conclave`
- `RBSAE-ark_enshrine` (whole file) and its `rbtgo_ark_enshrine` quoin — superseded by `RBSLE` / `rbtgo_lode_ensconce`. (RBSAE is mis-filed under `ark_` though it is fetched-side base capture; delete it, do not adapt it.)
- constant names carrying `ENSHRINE` (e.g. `ZRBFC_BUILD_POLL_CEILING_ENSHRINE`) — verb-neutralize the name
- `rbf_fact_lode` quoin and the `<stamp>.lode` multi-form capture-file — retired in favor of single-form chaining facts (a touchmark value-fact + a kind-brand enum); catch any stragglers the fact-chaining pace's RBSLE reslate left in place.

## KEEP — never flag these (introduced or legitimately retained)
- New Lode vocab: `lode`, `rbi_ld`, `rbld*` (the family: rbld0_/rblds_/rbldb_/rbldl_), `RBSL*`, `bole`, `ensconce`, `divine`, `banish`, `touchmark`, the inspect verb `augur` (`rbw-la`, now minted), and the introduced RBS0 quoins (`rbtga_lode`, `rbtga_touchmark`, `rbtga_lode_vouch`, `gar_lodes_namespace`, `rbst_touchmark`, `rbtgog_lode`, `rbtgo_lode_*`)
- `reliquary` as a **kind name** survives (it is `conclave`'s kind) — only the standalone noun / "stamp" identifier dies
- `rbrd_inscribe` (depot tripwire, `rbw-rdi`) — different operation, OUT of scope, stays
- `rbfd_mirror` (bind vessel-mode copy, made-side) — OUT of scope, stays

## Discovery recipe
`grep -rinE 'enshrine|rbi_es|rbi_rq|reliquary stamp|rbfl_inscribe|rbf_fact_lode|\.lode\b' Tools/ tt/` → expect zero. For bare `inscribe` and bare `reliquary` (both overloaded), grep and hand-classify every hit against the KEEP guards above. This list ACCRETES: as each cutover lands, append any stragglers it could not delete in place.

## Done
The recipe returns zero un-guarded hits; every KEEP term is intact; `fast` plus the relevant suites stay green. The heat's superseded vocabulary is gone from the repo.

**[260605-0905] rough**

## Character
Terminal heat-wide pass — mechanical but exacting. The single danger is a kill-stem that also lives inside a word we keep; the KEEP guards below are load-bearing, not courtesy. Runs LAST, after every cutover. Never before.

## Goal
Once all cutovers have repointed their chokepoints and banished their old GAR namespaces, sweep the whole repo so NO vocabulary superseded by this heat survives — in code, specs, tabtargets, onboarding prose, comments, or constant names — while leaving the introduced Lode vocabulary and the legitimately-overloaded survivors untouched.

## Why this pace is terminal (do not front-run)
This is the delete-old-last tail of the heat. Each cutover deletes its own namespace in-place; until a cutover lands, its surviving old vocabulary IS the rollback. This pace catches only what the per-cutover deletes could not reach — stray prose, comments, cross-refs, a mis-filed spec, a vestigial constant name. It MUST remain the final pace; nothing scrubs shared vocabulary before that vocabulary's cutover has landed.

## KILL — these stems must not survive (except where a KEEP guard applies)
- `enshrine`, `enshrinement`, `Enshrines`, `*Enshrinements` (tabtarget frontispieces) — superseded by `ensconce` / `bole`
- `rbi_es` — enshrinement GAR namespace, folded into `rbi_ld`
- `rbi_rq` — reliquary GAR namespace, folded into `rbi_ld`
- `reliquary stamp`, and `reliquary` used as a standalone identifier-noun — replaced by the reliquary-kind **touchmark**
- `rbfl_inscribe` and the reliquary-mirror sense of `inscribe` — absorbed into `fetter` / `conclave`
- `RBSAE-ark_enshrine` (whole file) and its `rbtgo_ark_enshrine` quoin — superseded by `RBSLE` / `rbtgo_lode_ensconce`. (RBSAE is mis-filed under `ark_` though it is fetched-side base capture; delete it, do not adapt it.)
- constant names carrying `ENSHRINE` (e.g. `ZRBFC_BUILD_POLL_CEILING_ENSHRINE`) — verb-neutralize the name

## KEEP — never flag these (introduced or legitimately retained)
- New Lode vocab: `lode`, `rbi_ld`, `rbld*` (the family: rbld0_/rblds_/rbldb_/rbldl_), `RBSL*`, `bole`, `ensconce`, `divine`, `banish`, `touchmark`, the inspect verb `augur` (`rbw-la`, now minted), and the introduced RBS0 quoins (`rbtga_lode`, `rbtga_touchmark`, `rbtga_lode_vouch`, `gar_lodes_namespace`, `rbf_fact_lode`, `rbst_touchmark`, `rbtgog_lode`, `rbtgo_lode_*`)
- `reliquary` as a **kind name** survives (it is `conclave`'s kind) — only the standalone noun / "stamp" identifier dies
- `rbrd_inscribe` (depot tripwire, `rbw-rdi`) — different operation, OUT of scope, stays
- `rbfd_mirror` (bind vessel-mode copy, made-side) — OUT of scope, stays

## Discovery recipe
`grep -rinE 'enshrine|rbi_es|rbi_rq|reliquary stamp|rbfl_inscribe' Tools/ tt/` → expect zero. For bare `inscribe` and bare `reliquary` (both overloaded), grep and hand-classify every hit against the KEEP guards above. This list ACCRETES: as each cutover lands, append any stragglers it could not delete in place.

## Done
The recipe returns zero un-guarded hits; every KEEP term is intact; `fast` plus the relevant suites stay green. The heat's superseded vocabulary is gone from the repo.

**[260604-1014] rough**

## Character
Terminal heat-wide pass — mechanical but exacting. The single danger is a kill-stem that also lives inside a word we keep; the KEEP guards below are load-bearing, not courtesy. Runs LAST, after every cutover. Never before.

## Goal
Once all cutovers have repointed their chokepoints and banished their old GAR namespaces, sweep the whole repo so NO vocabulary superseded by this heat survives — in code, specs, tabtargets, onboarding prose, comments, or constant names — while leaving the introduced Lode vocabulary and the legitimately-overloaded survivors untouched.

## Why this pace is terminal (do not front-run)
This is the delete-old-last tail of the heat. Each cutover deletes its own namespace in-place; until a cutover lands, its surviving old vocabulary IS the rollback. This pace catches only what the per-cutover deletes could not reach — stray prose, comments, cross-refs, a mis-filed spec, a vestigial constant name. It MUST remain the final pace; nothing scrubs shared vocabulary before that vocabulary's cutover has landed.

## KILL — these stems must not survive (except where a KEEP guard applies)
- `enshrine`, `enshrinement`, `Enshrines`, `*Enshrinements` (tabtarget frontispieces) — superseded by `ensconce` / `bole`
- `rbi_es` — enshrinement GAR namespace, folded into `rbi_ld`
- `rbi_rq` — reliquary GAR namespace, folded into `rbi_ld`
- `reliquary stamp`, and `reliquary` used as a standalone identifier-noun — replaced by the reliquary-kind **touchmark**
- `rbfl_inscribe` and the reliquary-mirror sense of `inscribe` — absorbed into `fetter` / `conclave`
- `RBSAE-ark_enshrine` (whole file) and its `rbtgo_ark_enshrine` quoin — superseded by `RBSLE` / `rbtgo_lode_ensconce`. (RBSAE is mis-filed under `ark_` though it is fetched-side base capture; delete it, do not adapt it.)
- constant names carrying `ENSHRINE` (e.g. `ZRBFC_BUILD_POLL_CEILING_ENSHRINE`) — verb-neutralize the name

## KEEP — never flag these (introduced or legitimately retained)
- New Lode vocab: `lode`, `rbi_ld`, `rbld_`, `RBSL*`, `bole`, `ensconce`, `divine`, `banish`, `touchmark`, the inspect verb `augur` (`rbw-la`, now minted), and the introduced RBS0 quoins (`rbtga_lode`, `rbtga_touchmark`, `rbtga_lode_vouch`, `gar_lodes_namespace`, `rbf_fact_lode`, `rbst_touchmark`, `rbtgog_lode`, `rbtgo_lode_*`)
- `reliquary` as a **kind name** survives (it is `conclave`'s kind) — only the standalone noun / "stamp" identifier dies
- `rbrd_inscribe` (depot tripwire, `rbw-rdi`) — different operation, OUT of scope, stays
- `rbfd_mirror` (bind vessel-mode copy, made-side) — OUT of scope, stays

## Discovery recipe
`grep -rinE 'enshrine|rbi_es|rbi_rq|reliquary stamp|rbfl_inscribe' Tools/ tt/` → expect zero. For bare `inscribe` and bare `reliquary` (both overloaded), grep and hand-classify every hit against the KEEP guards above. This list ACCRETES: as each cutover lands, append any stragglers it could not delete in place.

## Done
The recipe returns zero un-guarded hits; every KEEP term is intact; `fast` plus the relevant suites stay green. The heat's superseded vocabulary is gone from the repo.

**[260603-1102] rough**

## Character
Terminal heat-wide pass — mechanical but exacting. The single danger is a kill-stem that also lives inside a word we keep; the KEEP guards below are load-bearing, not courtesy. Runs LAST, after every cutover. Never before.

## Goal
Once all cutovers have repointed their chokepoints and banished their old GAR namespaces, sweep the whole repo so NO vocabulary superseded by this heat survives — in code, specs, tabtargets, onboarding prose, comments, or constant names — while leaving the introduced Lode vocabulary and the legitimately-overloaded survivors untouched.

## Why this pace is terminal (do not front-run)
This is the delete-old-last tail of the heat. Each cutover deletes its own namespace in-place; until a cutover lands, its surviving old vocabulary IS the rollback. This pace catches only what the per-cutover deletes could not reach — stray prose, comments, cross-refs, a mis-filed spec, a vestigial constant name. It MUST remain the final pace; nothing scrubs shared vocabulary before that vocabulary's cutover has landed.

## KILL — these stems must not survive (except where a KEEP guard applies)
- `enshrine`, `enshrinement`, `Enshrines`, `*Enshrinements` (tabtarget frontispieces) — superseded by `ensconce` / `bole`
- `rbi_es` — enshrinement GAR namespace, folded into `rbi_ld`
- `rbi_rq` — reliquary GAR namespace, folded into `rbi_ld`
- `reliquary stamp`, and `reliquary` used as a standalone identifier-noun — replaced by the reliquary-kind **touchmark**
- `rbfl_inscribe` and the reliquary-mirror sense of `inscribe` — absorbed into `fetter` / `conclave`
- `RBSAE-ark_enshrine` (whole file) and its `rbtgo_ark_enshrine` quoin — superseded by `RBSLE` / `rbtgo_lode_ensconce`. (RBSAE is mis-filed under `ark_` though it is fetched-side base capture; delete it, do not adapt it.)
- constant names carrying `ENSHRINE` (e.g. `ZRBFC_BUILD_POLL_CEILING_ENSHRINE`) — verb-neutralize the name

## KEEP — never flag these (introduced or legitimately retained)
- New Lode vocab: `lode`, `rbi_ld`, `rbld_`, `RBSL*`, `bole`, `ensconce`, `divine`, `banish`, `touchmark`, and the inspect verb (`augur` leading — pending final mint in the spec pace)
- `reliquary` as a **kind name** survives (it is `conclave`'s kind) — only the standalone noun / "stamp" identifier dies
- `rbrd_inscribe` (depot tripwire, `rbw-rdi`) — different operation, OUT of scope, stays
- `rbfd_mirror` (bind vessel-mode copy, made-side) — OUT of scope, stays

## Discovery recipe
`grep -rinE 'enshrine|rbi_es|rbi_rq|reliquary stamp|rbfl_inscribe' Tools/ tt/` → expect zero. For bare `inscribe` and bare `reliquary` (both overloaded), grep and hand-classify every hit against the KEEP guards above. This list ACCRETES: as each cutover lands, append any stragglers it could not delete in place.

## Done
The recipe returns zero un-guarded hits; every KEEP term is intact; `fast` plus the relevant suites stay green. The heat's superseded vocabulary is gone from the repo.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 e conclave-live-verify-banish
  2 Q buk-fact-chaining-prev-and-git-gate
  3 F buo-tweak-sprue
  4 A lode-base-pilot-sketch
  5 C lode-ensconce-collision-guard
  6 B lode-remainder-replan
  7 E lode-rbsl-bole-verb-specs
  8 G lode-scaffold-kind-letters
  9 H lode-bole-enshrine-cutover
  10 I lode-reliquary-capture
  11 J lode-wsl-capture
  12 K lode-podvm-cerebro-experiment
  13 Z gar-delete-host-cloud-boundary
  14 T crane-embrace-eviction
  15 a image-backdoor-path-verbs
  16 N lode-augur-inspect-split
  17 Y lode-enshrine-spec-retire
  18 S theurge-fixture-fact-chain-fix
  19 W lode-podvm-immure
  20 L lode-podvm-platform-fanout
  21 M lode-reliquary-inscribe-cutover
  22 O lode-public-docs-concept
  23 P lode-housekeeping-deferrals
  24 X lode-skopeo-reliquary-eviction
  25 g suite-subset-and-test-table-repair
  26 m cupel-python-import-allowlist
  27 h cloud-delete-hardening
  28 j elect-anchor-slot-count-soften
  29 k invest-actas-readback-gate
  30 i immure-capture-residue
  31 n enshrine-spec-structural-residue
  32 o single-slot-extract-consolidation
  33 c burv-invoke-dir-isolation-verify
  34 f endgame-verification-ladder
  35 R image-clean-commit-reproducibility-audit
  36 b yoke-colophon-consumer-move
  37 d terminal-memo-triage
  38 q envelope-git-commit-stamp
  39 p bash-filename-case-consolidation
  40 D lode-vocab-finalization-scrub

eQFACBEGHIJKZTaNYSWLMOPXgmhjkinocfRbdqpD
···xx···xxx···xx··xx···xx········x···xx· rbtdrc_crucible.rs
x··x···xxxx··x····xxx··x·············xx· rbgc_Constants.sh
······x··xx·····x·xxx··x······x······x·x RBS0-SpecTop.adoc
···x····xxx···xx··x·x··············xx··x rbz_zipper.sh
···x····xxx···xx··x·x··············x··x· rbtdgc_consts.rs
········xxx···xx··x·x··············x···x claude-rbk-tabtarget-context.md
···x····xxx·······xxx·················xx rbtdrm_manifest.rs
·xx·····x····x······x··x···x······x···x· rbfd_FoundryDirectorBuild.sh
x········x··xx·····xx··········x··x···x· rbldr_Reliquary.sh
·········xx·x··x··xx·················xx· rbldl_Lifecycle.sh
··x·····x········x··x··x·········x····xx rbtdro_onboarding.rs
x·········x·xx·········x·······x··x···x· rbldw_Underpin.sh
·············xx·x······x·····x········xx claude-rbk-acronyms.md
·········x···x·····x···x··········x··x·x RBSLC-lode_conclave.adoc
·········xx·xx·x··x···················x· rbld0_Lode.sh
····x···x···xx·····x··············x···x· rbldb_Bole.sh
····x·x·x····x·····x··············x··x·· RBSLE-lode_ensconce.adoc
··················xx·········x····x··x·x RBSLI-lode_immure.adoc
············xx·········x··x···········xx RBSCB-CloudBuildPosture.adoc
············xx····x············x·····xx· rblds_Spine.sh
·········x···x·····xx··x···············x rbgjl03-conclave-capture.sh
·····x··········x··x·x··············x··x RBSHR-HorizonRoadmap.adoc
x·················xx···········x··x···x· rbldv_Immure.sh
·············x·········x·············xxx RBSCJ-CloudBuildJson.adoc
·············x·········x·x···········xx· CBG-CloudBuildGuide.md
············xx·········x··x···········x· rbfc0_FoundryCore.sh
··········x········x···x··········x··x·· RBSLU-lode_underpin.adoc
···xx········x·····x···················x rbgjl01-ensconce-capture.sh
x············x······x··x··············x· rbfca_StepAssembly.sh
····················x··x··········x····x rbhodb_director_bind.sh, rbhodf_director_first_build.sh
·············x·········x·x·············x rbtdru_cupel.rs
·············x······x··x··············x· rbfly_Yoke.sh
············x·x···········x···········x· rbfld_Delete.sh
··········x··x·····x···x················ rbgjl04-underpin-capture.sh
········x···········x·············x···x· rbfv_FoundryVerify.sh
········x·······x·················x····x RBSAC-ark_conjure.adoc
········x·····x·····x·················x· rbfln_Inventory.sh
···x····x···········x·················x· rbcc_Constants.sh
··x·····················x···········x·x· CLAUDE.md
x··················x·············x··x··· memo-20260610-heat-BH-lode-operation-durations.md
x·······x·····x·······················x· rbflw_Wrest.sh
x·······x····x·········x················ rbrv.env
x··x····x·····························x· rbgl_GarLayout.sh
····················x·········x········x RBSIJ-image_jettison.adoc
····················x··x···············x RBSDY-director_yoke.adoc
···················x················x·x· ACG-AllocationCodingGuide.md
··················xx·········x·········· rbgjl07-immure-select.py, rbgjl08-immure-capture.sh, rbgjl09-immure-residency.sh
················x···x··················x RBSIR-image_rekon.adoc
············x···············x·········x· rbgg_Governor.sh
············x·······x·················x· rbfl0_FoundryLedger.sh
········x···········x·············x····· rbhoda_director_airgap.sh
········x···········x··x················ rbgjr01-reliquary-preflight.sh
······x············x·················x·· RBSLA-lode_augur.adoc
···x·········x·······················x·· rbgjl02-assemble-push-vouch.sh
·x····································xx rbfk_kludge.sh
·xx···································x· BUS0-BashUtilitiesSpec.adoc
x·······x·····························x· rbfcg_GarRest.sh
······································xx rbfc0_core.sh, rbfca_assembly.sh, rbfcg_gar.sh, rbfd_director.sh, rbfly_yoke.sh, rbgc_constants.sh, rbldb_bole.sh, rbldr_reliquary.sh, rbldv_immure.sh, rbldw_underpin.sh, rblm_cli.sh
··································x····x RBSDE-depot_levy.adoc, RBSRT-RegimeDepot.adoc
··································x···x· rbgp_cli.sh, rbld0_cli.sh, rbrd_cli.sh
······························x········x RBSIW-image_wrest.adoc
·····························x········x· rbgp_Payor.sh
····························x·········x· rbgi_IAM.sh
····················x··················x rbhodg_director_graft.sh
····················x·········x········· RBSIA-image_audit.adoc
···················x··················x· rbtdrf_fast.rs
················x···x··················· RBSRV-RegimeVessel.adoc
··············x·····················x··· memo-20260610-heat-BH-image-tabtarget-cleanup.md
··············x··x······················ rbtdri_invocation.rs
·············x·························x rbgjs-gcrane-fingerprint.sh
·············x······x··················· rbgji01-inscribe-mirror.sh
·············xx························· main.rs
············x·························x· rbldd_Delete.sh
············x·············x············· rbgjl06-package-delete.py
···········x······x····················· RBSPV-PodmanVmSupplyChain.adoc
······x·····x··························· RBSLB-lode_banish.adoc
··x···································x· butcbe_BureEnvironment.sh
··x··············x······················ rbtdti_invocation.rs
·x····································x· butcfc_FactChaining.sh, butt_testbench.sh, rbfd_cli.sh, rbfk_cli.sh, rbob_cli.sh
·······································x CLAUDE.consumer.md, RBSAK-ark_kludge.adoc, RBSAV-ark_vouch.adoc, RBSCL-consecration_tally.adoc, RBSIP-ifrit_pentester.adoc, rbgjs-token-fetch.sh, rbho0_start_here.sh, rbyc_common.sh
······································x· BCG-BashConsoleGuide.md, RBSCIG-IamGrantContracts.adoc, RBSCIP-IamPropagation.adoc, bujb_cli.sh, bupr_PresentationRegime.sh, bupr_regime.sh, burc_cli.sh, bure_cli.sh, burn_cli.sh, burp_cli.sh, burs_cli.sh, busc_shellcheckrc, butcbe_bure.sh, butcbx_BurxExchange.sh, butcbx_burx.sh, butcfc_facts.sh, butckk_KickTires.sh, butckk_kick.sh, butclc_LinkCombinator.sh, butclc_links.sh, butcym_YelpModule.sh, butcym_yelp.sh, rba_Auth.sh, rba_auth.sh, rbcc_constants.sh, rbcr_render.sh, rbdc_DerivedConstants.sh, rbdc_derived.sh, rbfc0_cli.sh, rbfcb_BuildHost.sh, rbfcb_host.sh, rbfcp_Plumb.sh, rbfcp_plumb.sh, rbfcv_VesselResolution.sh, rbfcv_resolve.sh, rbfh_FoundryHygiene.sh, rbfh_cli.sh, rbfh_hygiene.sh, rbfl0_cli.sh, rbfl0_ledger.sh, rbfld_delete.sh, rbfln_inventory.sh, rbflw_wrest.sh, rbfr_FoundryRetriever.sh, rbfr_cli.sh, rbfr_retriever.sh, rbfv_cli.sh, rbfv_verify.sh, rbga_ArtifactRegistry.sh, rbga_registry.sh, rbgb_Buckets.sh, rbgb_buckets.sh, rbgd_DepotConstants.sh, rbgd_depot.sh, rbge_Rest.sh, rbge_rest.sh, rbgg_cli.sh, rbgg_governor.sh, rbgi_iam.sh, rbgl_layout.sh, rbgo_OAuth.sh, rbgo_oauth.sh, rbgp_payor.sh, rbgv_AccessProbe.sh, rbgv_cli.sh, rbgv_probe.sh, rbho0_Onboarding.sh, rbho0_cli.sh, rbho0_onboarding.sh, rbhp0_Payor.sh, rbhp0_cli.sh, rbhp0_payor.sh, rbhw0_Windows.sh, rbhw0_cli.sh, rbhw0_windows.sh, rbld0_lode.sh, rbldd_delete.sh, rbldl_lifecycle.sh, rblds_spine.sh, rbq_Qualify.sh, rbq_cli.sh, rbq_qualify.sh, rbra_cli.sh, rbrn_cli.sh, rbro_cli.sh, rbrp_cli.sh, rbrp_regime.sh, rbrr.env, rbrr_cli.sh, rbrs_cli.sh, rbrv_cli.sh, rbtdrp_pristine.rs, rbtdtk_canonical.rs, rbtdtm_manifest.rs, rbtdtp_pristine.rs, rbte_cli.sh, rbuh_Http.sh, rbuh_http.sh, rbupmis_Scrub.sh, rbupmis_scrub.sh, rbv_cli.sh, rbv_podvm.sh
····································x··· MCM-MetaConceptModel.adoc, memo-20260610-heat-BH-enshrine-structural-residue.md, memo-20260610-heat-BH-extract-keys-triplication.md, memo-20260610-heat-BH-fable-recommendation-actas-propagation-flap.md, memo-20260610-heat-BH-fable-recommendation-convergence-deadline-shape.md, memo-20260610-heat-BH-fable-recommendation-elect-anchor-slot-count.md, memo-20260610-heat-BH-fable-recommendation-jettison-404-honesty.md, memo-20260610-heat-BH-fable-recommendation-pin-delete-builder.md, memo-20260610-heat-BH-fable-recommendation-python-import-allowlist.md, memo-20260610-heat-BH-fable-recommendation-urlerror-tolerance.md, memo-20260610-heat-BH-fable-recommendation-urllib-timeout.md, memo-20260610-heat-BH-fable-review-immure-noncritical.md, memo-20260610-heat-BH-fast-tier-credless-by-convention.md, memo-20260610-quoin-minting-introspection.md, memo-20260611-google-impersonation-preference.md, memo-20260611-heat-BH-class-c-setiampolicy-write-flap.md, memo-20260611-heat-BH-vouch-poll-ceiling-queued-burst.md, memo-20260611-token-custody-context-enforcement.md, memo-20260612-office-federation-conversion.md
··································x····· RBSAG-ark_graft.adoc, rbhopw_payor_wrapper.sh, rbndb_base.sh
·························x·············· rbtdtu_cupel.rs
·······················x················ rbgjm01-mirror-image.sh, rbtdto_onboarding.rs
······················x················· rbhwcd_docker_context_discipline.sh
·····················x·················· README.md
····················x··················· RBSDI-depot_inscribe.adoc, rbfli_Inscribe.sh, rbrv_regime.sh, rbw-dI.DirectorInscribesReliquary.sh, rbw-iJr.DirectorJettisonsReliquaryImage.sh, rbw-iar.DirectorAuditsReliquaries.sh, rbw-irr.DirectorRekonsReliquary.sh, rbw-iwr.DirectorWrestsReliquaryImage.sh
··················x····················· rbgjl07-immure-select.sh, rbw-lI.DirectorImmuresPodvm.sh
················x······················· RBSAE-ark_enshrine.adoc, RBSFH-dockerfile_hygiene.adoc
···············x························ rbw-la.DirectorAugursLode.sh
··············x························· rbw-iJ.DirectorJettisonsImage.sh, rbw-iJe.DirectorJettisonsEnshrinement.sh, rbw-il.DirectorListsRegistry.sh, rbw-iw.DirectorWrestsImage.sh, rbw-iwe.DirectorWrestsEnshrinedImage.sh
·············x·························· rbgjl05-underpin-wrap.sh, rbgjs-gcrane-append.sh, rbgjs-skopeo-fingerprint.sh, rbtdre_engine.rs
············x··························· RBSAA-ark_abjure.adoc
···········x···························· memo-20260608-lode-podvm-cerebro-experiment.md
··········x····························· rbgjs-gpg-verify-sums.sh, rbw-lU.DirectorUnderpinsWsl.sh
·········x······························ rbw-lC.DirectorConclavesReliquary.sh
········x······························· rbgje01-enshrine-copy.sh, rbw-dE.DirectorEnshrinesVessel.sh, rbw-iae.DirectorAuditsEnshrinements.sh
······x································· RBSLD-lode_divine.adoc
·····x·································· memo-20260605-ultracode-replan-process.md
···x···································· rbk-claude-acronyms.md, rbk-claude-tabtarget-context.md, rbld_Lode.sh, rbld_cli.sh, rbw-lB.DirectorBanishesLode.sh, rbw-lE.DirectorEnsconcesBase.sh, rbw-lE.DirectorEnsconcesBole.sh, rbw-ld.DirectorDivinesLodes.sh
··x····································· bure_regime.sh
·x······································ bud_dispatch.sh, buf_fact.sh, bug_git.sh, burd_regime.sh, claude-buk-core.md

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 256 commits)

  1 h cloud-delete-hardening
  2 j elect-anchor-slot-count-soften
  3 k invest-actas-readback-gate
  4 g suite-subset-and-test-table-repair
  5 m cupel-python-import-allowlist
  6 n enshrine-spec-structural-residue
  7 c burv-invoke-dir-isolation-verify
  8 o single-slot-extract-consolidation
  9 d terminal-memo-triage
  10 f endgame-verification-ladder
  11 b yoke-colophon-consumer-move
  12 q envelope-git-commit-stamp
  13 p bash-filename-case-consolidation
  14 D lode-vocab-finalization-scrub

123456789abcdefghijklmnopqrstuvwxyz
x··································  h  1c
·x·································  j  1c
··x································  k  1c
···x·······························  g  1c
····x······························  m  1c
·····x·····························  n  1c
······x····························  c  1c
·······xx··························  o  2c
·········x·············x·xxx·······  d  5c
··········x··xx····················  f  3c
···················xx··············  b  2c
····························xx·····  q  2c
······························xx···  p  2c
································xxx  D  3c
```

## Steeplechase

### 2026-06-12 11:33 - ₢BHAAD - W

Terminal vocabulary scrub landed: every kill-stem swept from living surfaces — enshrine/enshrinement out of code, specs, handbook, and consumer doc; rbi_rq/rbi_es spec residues removed (gar_reliquaries_namespace + rbf_fact_reliquary quoins three-part removed); reliquary-stamp replaced by touchmark throughout including the rbtdro probe identifiers; the reliquary-mirror inscribe sense drained with RBSAC recut to the member-tag Lode layout; poll ceilings verb-neutralized to CAPTURE_LIGHT/CAPTURE_HEAVY; stale skopeo-as-bole prose corrected and the RBSCJ snippet inventory made accurate (gcrane-append/gpg-verify-sums rows added); latent unbound RBZ_WREST_ENSHRINED_IMAGE in rbfk fixed; accreted RBSLI and cupel-comment corrections landed; consumer-doc tabtarget staleness repaired. Judgment call recorded: rbf_fact_lode quoin kept and retitled Lode Capture Facts (definition already described the live single-form facts). KEEP guards intact; tool cross-check clean; fast 153/153, shellcheck 210 clean.

### 2026-06-12 11:32 - ₢BHAAD - n

Consumer-doc tabtarget staleness repaired alongside the sweep: the onboarding entry point corrected to tt/rbw-o.ONBOARDING.sh (rbw-go.OnboardMAIN never survived the handbook restart), and the Troubleshooting rows now name the real launchers — rbw-tq.QualifyFast for tabtarget health and rbw-ft.RetrieverTalliesHallmarks for build status.

### 2026-06-12 11:28 - ₢BHAAD - n

Terminal vocabulary scrub: every kill-stem swept from living surfaces — enshrine/enshrinement out of all code comments, specs, handbook prose, and the consumer doc (RBYC_ENSHRINE anchor retired; onboarding speaks Ensconce/Conclave); rbi_rq/rbi_es spec residues removed (gar_reliquaries_namespace quoin three-part removal, RBSDE sibling-namespace prose now rbi_hm/rbi_ld); reliquary-stamp identifier replaced by touchmark everywhere (rbfd ashlar error, yoke hints, handbook probes, rbtdro probe fn + case names + fixture scratch file); rbfl_inscribe sense of inscribe drained (RBSLC stale remains-live sentence dropped; RBSAC reliquary check recut to the member-tag Lode layout); dead RBF_FACT_RELIQUARY constant + rbf_fact_reliquary quoin removed; rbf_fact_lode quoin kept but retitled Lode Capture Facts (definition already described the live single-form facts); poll ceilings verb-neutralized ENSHRINE/INSCRIBE -> CAPTURE_LIGHT/CAPTURE_HEAVY; stale skopeo-as-bole prose corrected (RBS0 conclave now gcrane; snippet headers recut; RBSCJ caller inventory fixed and gcrane-append/gpg-verify-sums rows added); retired image-variant prose drained (zipper comment, RBSHR waves); latent unbound RBZ_WREST_ENSHRINED_IMAGE in rbfk repointed to RBZ_WREST_IMAGE; accreted corrections landed (RBSLI slot carries envelope too; cupel gcloud comment skidmark drained). Build + shellcheck (210) clean.

### 2026-06-12 10:55 - ₢BHAAp - W

Bash filename case consolidation landed: all 50 mixed-case .sh basenames under Tools/ renamed to the single lowercase rule via git mv (ratified map — single-word lowers + settled multi-word slimmings), every living reference updated in the same commit. BCG naming table recut to state the lowercase rule (PascalCase carve-outs retired), CLAUDE.md mint row updated, rbk acronym map agrees throughout with RBLDR/RBLDW/RBLDV promoted to live entries (RBLDT alone reserved). Sweep and retired-basename grep land clean in living surfaces; build + shellcheck (210 clean) + fast 153/153 green.

### 2026-06-12 10:52 - ₢BHAAp - n

Bash filename case consolidation: all 50 mixed-case .sh basenames under Tools/ renamed to the single lowercase rule (ratified map — single-word lowers plus the settled multi-word slimmings), with every living reference updated in the same commit: sourcing lines, zipper-regenerated consts/context, specs, acronym maps, shellcheckrc, launcher CLIs. BCG's Naming Convention Patterns table recut (decomposition entry/cluster and test-case rows now state the lowercase rule), CLAUDE.md mint-table code-file row updated, and the rbk acronym map's RBLD reserved clause corrected — rbldr/rbldw/rbldv promoted to live entries, RBLDT alone reserved. Sweep and retired-basename grep land clean; build + shellcheck green

### 2026-06-12 10:42 - ₢BHAAq - W

Envelope git-commit stamp landed and live-proven: the spine injects _RBGL_GIT_COMMIT (dispatching HEAD via the existing rbfcb git-metadata primitive) into every substitutions blob, and the shared vouch-push step rbgjl02 splices rblv_git_commit into each staged envelope before push — one edit inherited by all kinds, honesty guaranteed by the upstream clean-tree gate on all four capture verbs. Schema bumped rbld-vouch-2 -> rbld-vouch-3; augur displays the field; the lode-lifecycle fixture asserts the literal HEAD hash survived host -> substitution -> splice -> GAR -> augur decode, and the service run returned exactly the landing commit from live GAR. Specs present-tense: RBS0 field list, RBSLA/E/C/U/I, RBSCJ blob-opacity contract and CBG CBh_102 amended to record the one spine-added key. fast 153/153, shellcheck clean

### 2026-06-12 10:27 - ₢BHAAq - n

Envelope git-commit stamp landed: the spine injects _RBGL_GIT_COMMIT (dispatching HEAD, via the existing rbfcb git-metadata primitive) into every substitutions blob, and the shared vouch-push step rbgjl02 splices rblv_git_commit into each staged envelope before push — one edit inherited by all kinds, clean-tree-gated honesty upstream. Schema bumped rbld-vouch-2 -> rbld-vouch-3; augur displays the field; lode-lifecycle fixture asserts the literal HEAD hash survived host -> substitution -> splice -> GAR -> augur decode. Specs updated present-tense (RBS0 field list, RBSLA/E/C/U/I, RBSCJ blob-opacity contract, CBG CBh_102). Build + shellcheck green

### 2026-06-12 10:05 - ₢BHAAd - W

Terminal memo triage complete: corpus glob fully stamped; all ten stragglers dispositioned with the operator — Class-C setIamPolicy write flap and suite-head credential probe declined as superseded by the ₣BZ office-federation conversion (hot-path IAM mutation and the keyfile probes retire with the RBRA estate; bridge attrition accepted, manual rbw-acp pre-flight is the bridge defense); vouch poll-ceiling QUEUED conflation elected as the two-bounded-clocks fix slated ₢BBABY in ₣BB with an 8-week log-mined incidence addendum (2 timeouts + 1 near-miss in 531 builds, all pure queue weather); cupel oversize split slated ₢BeAAJ in ₣Be; envelope git-commit stamp slated ₢BHAAq here (clean-tree gate already guarantees honesty; single shared-chokepoint injection); RBLD stale reserved-letters repair accreted into the rename pace ₢BHAAp; RBSLI slot prose, rbf_fact_reliquary dead constant+quoin kill, and the cupel gcloud comment skidmark-drain accreted into the scrub ₢BHAAD; gcloud allowlist exception accepted permanently; two orphan conclave Lodes left as depot-disposable debris by explicit verdict. Closing duties run: retired-memo convention added to CLAUDE.md, sixteen fully-dispositioned memos shelved to Memos/retired/ with five durable citations repointed (RBSHR x2, rbz_zipper, MCM, ACG), vouch memo and the three ₣BZ memos deliberately held. En route: mcm_tidemark renamed mcm_skidmark across MCM/ACG under ₣Bd. Heat runway after this wrap: envelope stamp, filename lowercase, terminal scrub

### 2026-06-12 10:04 - ₢BHAAd - n

Memo shelving executed per the retired-memo convention: sixteen fully-dispositioned ₣BH memos git-mv'd to Memos/retired/ basename-unchanged, with the five durable citations repointed first (RBSHR x2, rbz_zipper, MCM, ACG). Held back: the vouch poll-ceiling memo (live evidence for a pending ₣BB pace) and the three ₣BZ-owned memos. Stale-path grep over Tools/ and CLAUDE.md lands clean

### 2026-06-12 09:57 - ₢BHAAd - n

Closing duties: the three ₣BZ evidence memos stamped owned-by-₣BZ so the triage glob-gate lands clean without ₣BH claiming their content; the retired-memo convention added to CLAUDE.md (fully-dispositioned memos move to Memos/retired/ basename-unchanged; an unresolvable memo path means retired; historical record, never resurrect without operator direction)

### 2026-06-12 09:27 - Heat - S

envelope-git-commit-stamp

### 2026-06-12 09:14 - ₢BHAAd - n

Triage dispositions for the two night-ledger memos: the Class-C setIamPolicy write flap declined as superseded by the ₣BZ office-federation conversion (hot-path IAM mutation retires with the RBRA estate; bridge attrition accepted); the vouch poll-ceiling QUEUED conflation elected shape (a) two-bounded-clocks with an incidence addendum mined from 8 weeks of logs (2 timeouts + 1 near-miss in 531 builds, all pure queue weather, confined to small-budget kinds)

### 2026-06-12 08:13 - Heat - r

moved BHAAd before BHAAp

### 2026-06-12 08:13 - Heat - r

moved BHAAD to last

### 2026-06-12 08:09 - ₢BHAAb - W

Yoke colophon moved to its consumer family and de-capitalized: rbw-dY -> rbw-rvy, enrolled in the Regime group's vessel cluster per the election rule. Colophon string lived only in the zipper; bash (RBZ_YOKE_RELIQUARY) and theurge (RBTDGC_YOKE_RELIQUARY) consumers repointed via regeneration. Grep census zero for rbw-dY; new colophon routing proven live; fast suite green (153/0). Tabtarget file rename was swept into the concurrent BZ officium's paddock commit f7443c8ea — content correct, provenance smudge only.

### 2026-06-12 08:06 - ₢BHAAb - n

Yoke colophon moved to its consumer family and de-capitalized: rbw-dY -> rbw-rvy, enrolled in the Regime group's vessel cluster per the election rule (cheap reversible config, operator-committed); consts and tabtarget-context regenerated

### 2026-06-11 13:39 - Heat - r

moved BHAAp after BHAAD

### 2026-06-11 13:39 - Heat - D

restring 1 paces from ₣Bb

### 2026-06-11 13:19 - Heat - n

Restamped the fast-tier-credless memo: ₢BHAAl dropped at the ₣Be groom, hazard closure recreated as ₢BeAAI riding the BUS0 tweak doctrine's suite-reservation rule with a band rejection code at the token-mint chokepoint

### 2026-06-11 13:16 - Heat - T

fast-tier-credless-by-construction

### 2026-06-11 07:30 - ₢BHAAf - W

Endgame verification ladder green end to end: complete 234/0 (71m49s), skirmish 297/0 (96m47s) with the bind-mode gcrane build — the skopeo-eviction's delegated named gate — passing live (rbtdro_onboarding_ordain_bind_plantuml), and blockade 66/0 (12m07s) elected at mount for the airgap credential-self-heal angle. Durations re-baseline recorded in the manifest memo with pinned log filenames. The ladder surfaced and this pace fixed one real bug: the onboarding conclave case read the retired rbf_fact_reliquary fact (cutover propagation miss, invisible until gauntlet/skirmish ran post-cutover) — repointed to the touchmark fact, live-verified by the green run. Night ledger: five skirmish attempts, four failures, four distinct causes, zero product-code defects — Payor RAPT expiry (operator reauth), Class-C setIamPolicy WRITE flap (new surface falsifying the recorded lean-write premise — memo), the stale fact reader (fixed in-pace), and a vouch poll-ceiling QUEUED-burst conflation (memo). Both new memos and all stragglers banked in the triage pace docket for disposition.

### 2026-06-11 07:28 - ₢BHAAf - n

Ladder durations re-baseline recorded in the manifest memo per the triage disposition: complete 71m49s (234/0), skirmish 96m47s (297/0, attempt 5), blockade 12m07s (66/0), with log filenames pinned

### 2026-06-11 05:00 - Heat - n

Memo: skirmish attempt 4 died at the jupyter ordain vouch — the build sat QUEUED 47 of 50 polls (pool weather) then the host ceiling expired 3 polls into WORKING. The poll ceiling conflates queue-wait with execution; repair shapes posed for triage. Fourth distinct failure cause of the night, zero product-code defects among them

### 2026-06-11 04:05 - Heat - n

Memo: skirmish attempt 2's divest 403 is a NEW Class-C surface, not a recurrence — the recorded fix tolerates the repo getIamPolicy read (which converged at 41s in the trace) but deliberately left the setIamPolicy write lean, and the write drew the 403 one second after the read went clean. Falsifies the lean-write premise recorded in the churn memo's addendum; repair shapes posed for triage, no verdict

### 2026-06-11 04:00 - ₢BHAAf - n

Ladder-surfaced fix: the onboarding conclave case read the retired rbf_fact_reliquary chaining fact, dead since the conclave cutover emits rbf_fact_lode_touchmark — onboarding-sequence lives only in gauntlet/skirmish, which never ran post-cutover, so the stale reader was invisible until the ladder's skirmish leg failed on it twice (conclave build SUCCESS, fact read fail). Repointed to the file's existing RBTDRO_FACT_LODE_TOUCHMARK and removed the dead RBTDRC_FACT_RELIQUARY const. Residue banked for triage: the dead RBF_FACT_RELIQUARY bash constant and the RBS0 rbf_fact_reliquary quoin are inert and go to the scrub/triage

### 2026-06-10 22:46 - ₢BHAAd - n

All 15 triage-corpus memos stamped with greppable in-file TRIAGED dispositions (filename renames rejected — dockets and commit messages reference memos by path). Nine slated-and-landed, one slated-pending (credless poison), two RBSHR horizon entries, three declined-with-record; grep -L TRIAGED Memos/memo-2026061*.md now finds any unhandled straggler

### 2026-06-10 22:36 - ₢BHAAo - W

Consolidated the three byte-parallel single-slot capture extracts (underpin/conclave/immure) into one shared zrbld_spine_extract_single (prefix/brand/label parameterized, keys-dump diagnostic included), homed in the spine beside zrbld_spine_extract: it rides zrbfc_sentinel and takes all kind data as parameters, so the spine still owns no kind knowledge and stays callable from both furnishing processes (the rbgc fact keys and buf_ fact machinery it reads are furnished by rbld0_cli and rbfl0_cli alike — spine header updated to say so). Bole keeps its own multi-slot 1..3 extract, a genuinely different shape. Net -52 lines. Verified by shellcheck (210 clean), fast (153/0), and a live wsl-lifecycle smoke (1/1 — the shared extract's first live run, first-time green). Closes the propagation-drift class that fired when the rbls_ sprue sweep missed one of the three copies and broke the service suite same-day.

### 2026-06-10 22:07 - ₢BHAAo - n

Consolidate the three byte-parallel single-slot capture extracts (underpin/conclave/immure) into one shared zrbld_spine_extract_single (prefix/brand/label parameterized, keys-dump diagnostic included), homed in the spine beside zrbld_spine_extract: it rides zrbfc_sentinel and takes all kind data as parameters, so the spine still owns no kind knowledge and stays callable from both furnishing processes (the rbgc fact keys and buf_ fact machinery it reads are furnished by rbld0_cli and rbfl0_cli alike — spine header updated to say so). Bole keeps its own multi-slot 1..3 extract, a genuinely different shape. Closes the propagation-drift class that fired when the rbls_ sprue sweep missed one of the three copies and broke the service suite same-day

### 2026-06-10 22:00 - ₢BHAAc - W

BURV invoke-dir isolation verified on tonight's post-repair service trace (temp-20260610-202914, the 163/0 run): 50 invoke dirs for 50 invocations with one temp slot each and zero reuse, against the pre-repair baseline of 10 dirs serving 32 invocations with slots reused up to 6x. The five capture invocations each hold their rbf_fact_lode_brand/_touchmark facts in their own private slot, and zero previous/ directories exist anywhere — the unchained service suite has no chaining surface, so the leak that fed wsl's brand into batch-vouch's ordain election is structurally absent. No code change; the suite-monotonic counter repair stands verified.

### 2026-06-10 22:00 - ₢BHAAn - W

Enshrine spec structural residue retired: gar_enshrines_namespace quoin removed dangle-free across RBS0 mapping/definition/fold-prose and RBSIJ/RBSIW locator examples; RBSIA Enshrinements-Audit section retired with the preamble singularized (also fixing its pre-existing two-vs-three-domains drift); rbst_reliquary_stamp collapsed into rbst_touchmark with the conclave member-tag detail confirmed homed in RBSLC before deletion. Verified by repo-wide grep census (zero hits on both quoins) and the central consolidated gate (build + fast 153/0).

### 2026-06-10 21:58 - ₢BHAAm - W

Extended the cupel supply-chain conformance to python cloud steps: new rbtdru_gcb_python case walks rbgj*/*.py with a module-root import allowlist (ZRBTDRU_PY_IMPORT_ALLOWED is the floor's authoritative home — twelve stdlib roots, the empirical corpus union), an outright ban on importlib/__import__/exec/eval, and subprocess argv[0] literal scanning against the shared GCB tool floor (one floor, two languages); gcloud adjudicated onto ZRBTDRU_GCB_ALLOWED with a dated comment; from-subprocess imports rejected so argv[0] stays scannable. 17 new unit tests; live corpus green via the central consolidated gate. Probe: zrbfc_expand_includes is language-blind except the hardcoded rbgjs-name.sh snippet suffix — recorded in CBG CBp_101, and CBp_102 flipped to its fired removal condition with a pointer to the constant.

### 2026-06-10 21:58 - ₢BHAAg - W

Repaired fast-subset drift in the gauntlet and skirmish suites (both gained handbook-render, foundry-path, recipe-validation, completing the ten-fixture fast set) and rewrote the stale CLAUDE.md test-execution table to match RBTDRC_SUITES: dependency tiers by fixture composition (fast 10, service 17, crucible 13, complete 20) plus a new table naming the five release/probe suites (gauntlet, skirmish, dogfight, siege, blockade) with their operator preconditions. Verified by the central consolidated build+fast gate.

### 2026-06-10 21:57 - ₢BHAAk - W

Landed the invest-side actAs read-back gate: rbgg_invest_director polls the Director SA IAM policy after the self-actAs grant until the binding is visible before declaring invest complete, closing the Class-C propagation flap at the first post-invest builds.create. New rbgi_poll_sa_iam_binding helper (rbuh_poll_until_ok shape, bounded by RBGC_MAX_CONSISTENCY_SEC), with rationale distinguishing this load-bearing SA-scope read-back from the removed project-scope one. Central gate green; live proof rides skirmish canonical-invest.

### 2026-06-10 21:57 - ₢BHAAj - W

Softened the base-anchor election slot-count die to log-and-leave, mirroring the kind-brand fix: a bole fact chained ahead of a multi-origin or origin-less vessel ordain now no-ops loudly instead of killing the ordain pre-submit. count==0 and count>1 carry distinct messages; header contract updated. No fast fixture case — the function is sentinel-guarded behind the full rbfd kindle, so the cheap dependency-free pattern does not apply; the live election witness stays in the service-tier onboarding fixture. Central gate green.

### 2026-06-10 21:57 - ₢BHAAh - W

Hardened the cloud-dispatch delete surface with four bundled repairs: ZRBFC_DELETE_BUILDER digest-pinned so the Director-run delete build never executes floating bytes (pin recorded in RBSCB posture), rbfl_jettison tolerates 404 as success matching its idempotent message, rbgjl06 fire_delete tolerates URLError/timeout in the reconciling form while truth-readers and metadata_token die loud, and every urlopen carries a 30s timeout so a hung socket names its stall. Central gate green (build + fast 153/0); live coverage delegated to the endgame ladder's banish/abjure traffic.

### 2026-06-10 21:55 - ₢BHAAi - W

Immure review noncritical bundle, all five items: rbgjl07 non-empty selection-field assert; rbgjl08/09 stdin-protection (</dev/null) on in-loop gcrane/curl; quay blob-CDN reachability probe added to the rbw-di tether egress inventory; buildStepOutputs 4KB arithmetic recorded in RBSLI (8-leaf podvm-native worst case 3211 bytes, 885 headroom, holds to ~10 leaves); RBSL spec family catalogued in claude-rbk-acronyms.md with the stale RBLD reserved-letter clause repointed. Shellcheck green; fast green via the central consolidated gate.

### 2026-06-10 21:51 - ₢BHAAm - n

Extended the cupel supply-chain conformance to python cloud steps: new rbtdru_gcb_python case walks rbgj*/*.py with a module-root import allowlist (ZRBTDRU_PY_IMPORT_ALLOWED, the floor's authoritative home), an outright ban on importlib/__import__/exec/eval, and subprocess argv[0] literal scanning against the shared GCB tool floor; gcloud adjudicated onto ZRBTDRU_GCB_ALLOWED with a dated comment; from-subprocess imports rejected so argv[0] stays scannable; 17 new unit tests. Probed zrbfc_expand_includes language-blindness (only bash-shaped assumption is the hardcoded rbgjs-name.sh snippet suffix) and recorded it in CBG CBp_101; flipped CBp_102 to its fired removal condition with a pointer to the constant

### 2026-06-10 21:49 - ₢BHAAk - n

Invest-side actAs read-back gate: rbgg_invest_director now polls the Director SA's IAM policy after the self-actAs grant until the binding is visible before declaring invest complete, closing the Class-C propagation flap at the first post-invest builds.create. New rbgi_poll_sa_iam_binding helper (rbuh_poll_until_ok shape, bounded by RBGC_MAX_CONSISTENCY_SEC) with a comment distinguishing this load-bearing SA-scope read-back from the removed project-scope one: an SA's policy dies with the SA, so it reads back tombstone-free, and the in-session consumer races the grant. Live proof delegated to skirmish's canonical-invest per docket

### 2026-06-10 21:46 - ₢BHAAi - n

Immure review noncritical bundle: rbgjl07 asserts both selection-entry fields non-empty (empty disktype no longer silently matches annotation-less descriptors); rbgjl08/09 stdin-protect in-loop gcrane/curl calls with </dev/null per the house while-read hazard; rbw-di tether egress inventory gains a quay blob-CDN reachability probe (cdn01.quay.io, first breakage point under tightened egress); RBSLI records the buildStepOutputs 4KB arithmetic for the 8-leaf podvm-native envelope (3211 bytes worst case, 885 headroom, holds to ~10 leaves); claude-rbk-acronyms.md catalogues the RBSL spec family (A/B/C/D/E/I/U) and repoints the stale RBLD reserved-letter clause

### 2026-06-10 21:45 - ₢BHAAj - n

Base-anchor election no-ops on a slot count other than one instead of fatal-dying, mirroring the kind-brand soften: a bole fact chained ahead of a multi-origin (or origin-less) vessel ordain now logs loud and leaves the ANCHORs as-is rather than killing the ordain pre-submit. count==0 and count>1 carry distinct messages (no ORIGIN declared vs election cannot disambiguate); header contract updated. No fast fixture case added — the function is sentinel-guarded behind the full rbfd kindle, so the cheap dependency-free pattern does not apply; the live election witness stays in the service-tier onboarding fixture

### 2026-06-10 21:42 - ₢BHAAh - n

Cloud-delete hardening: ZRBFC_DELETE_BUILDER digest-pinned (Director-run build never executes floating bytes; pin recorded in RBSCB posture), rbfl_jettison tolerates 404 as success (idempotent delete matches its message), rbgjl06 fire_delete tolerates URLError/timeout in reconciling form while truth-readers and metadata_token die loud, and every urlopen carries a 30s timeout so a hung socket names its stall instead of riding to the build timeout

### 2026-06-10 21:42 - ₢BHAAn - n

Enshrine spec structural residue retired: gar_enshrines_namespace quoin removed dangle-free (mapping, definition, historical fold-prose, RBSIJ/RBSIW locator examples), RBSIA Enshrinements-Audit section retired with preamble singularized (also fixing its pre-existing two-vs-three-domains drift), rbst_reliquary_stamp collapsed into rbst_touchmark (conclave member-tag detail confirmed homed in RBSLC before delete). RBSIW locator example also dropped the dead reliquaries-namespace path in the same line, trimming one ref ahead of the scrub's sanctioned quoin removal

### 2026-06-10 21:39 - ₢BHAAg - n

Repaired fast-subset drift in gauntlet and skirmish suites (added handbook-render, foundry-path, recipe-validation to both) and rewrote the stale CLAUDE.md test-execution table to match the registry: dependency-tier compositions by fixture name (fast 10, service 17, crucible 13, complete 20) plus a new table naming the five release/probe suites (gauntlet, skirmish, dogfight, siege, blockade) with preconditions

### 2026-06-10 21:27 - Heat - r

moved BHAAl after BHAAR

### 2026-06-10 21:27 - Heat - r

moved BHAAR after BHAAf

### 2026-06-10 21:27 - Heat - r

moved BHAAf after BHAAc

### 2026-06-10 21:27 - Heat - r

moved BHAAc after BHAAo

### 2026-06-10 21:26 - Heat - r

moved BHAAi after BHAAk

### 2026-06-10 21:26 - Heat - r

moved BHAAm after BHAAg

### 2026-06-10 21:25 - ₢BHAAX - W

Terminal skopeo eviction landed: bind mirror converted to gcrane cp on the pinned reliquary gcrane (busybox shebang, ambient google.Keychain auth, token machinery deleted); skopeo purged from cohort manifest, constants, resolver plumbing, both preflights (gcrane now actually checked), yoke counts, fixture tags, cupel allowlist, and all handbook/spec narrative. Sonnet-delegated sweep with Fable review: review caught and repaired the token-fetch snippet deletion (live caller rbgjl09 immure residency — docket premise predated the immure pace), the in-pool preflight dropping gcrane against the RBS0 contract, and RBSDY's stale seven-count; subagent's git-stash self-test flagged. Stale rbtdto unit tests from the inscribe cutover repaired (141/141). Fresh conclave r260610202716 (6 members) yoked across 9 vessels. Shellcheck 210, fast 152/0, service 163/0 green; bind-mode live verify delegated to the endgame skirmish per the cinch. Full census: every surviving skopeo mention is history or tool-landscape prose.

### 2026-06-10 21:18 - ₢BHAAd - n

Memo triage RBSHR dispositions: image-family Wave 3 hallmark-variant retirement (made-side retrofit heat trigger) and progress-aware convergence deadline for cloud-side package deletes (latent, 55-version worst case vs ~1000 threshold) recorded as horizon entries rather than heat paces

### 2026-06-10 21:17 - Heat - S

single-slot-extract-consolidation

### 2026-06-10 21:17 - Heat - S

enshrine-spec-structural-residue

### 2026-06-10 21:17 - Heat - S

cupel-python-import-allowlist

### 2026-06-10 21:16 - Heat - S

fast-tier-credless-by-construction

### 2026-06-10 21:16 - Heat - S

invest-actas-readback-gate

### 2026-06-10 21:15 - Heat - S

elect-anchor-slot-count-soften

### 2026-06-10 21:15 - Heat - S

immure-capture-residue

### 2026-06-10 21:15 - Heat - S

cloud-delete-hardening

### 2026-06-10 21:14 - Heat - S

suite-subset-and-test-table-repair

### 2026-06-10 20:28 - ₢BHAAX - n

Yoke the fresh post-eviction conclave Lode r260610202716 (six members, gcrane in, skopeo out — 20s build) across all 9 vessels, so the endgame skirmish meets the skopeo-free cohort. Yoke's derived count validated all six member tags against GAR before writing.

### 2026-06-10 20:26 - ₢BHAAX - n

Terminal skopeo eviction: the bind mirror (rbgjm01) converts from skopeo copy --all with inline metadata-token fetch to gcrane cp on the PINNED reliquary gcrane (busybox shebang composed in zrbfd_mirror_submit; ambient google.Keychain auth, no token). skopeo drops from the cohort everywhere: conclave MANIFEST, RBGC_RELIQUARY_TOOL_SKOPEO, z_rbfc_tool_skopeo resolver plumbing + leaked global, host preflight array (gcrane takes the slot — previously unchecked), in-pool preflight loop (now 5 of 6: gcloud/docker/syft/binfmt/gcrane, matching the RBS0 quoin — the sweep's first cut had dropped gcrane there on a bind-only rationale, reviewer-corrected), yoke expected-set (RBSDY seven→six including the NOTE count the grep could not catch), reliquary-lifecycle fixture tag (rbi_skopeo→rbi_gcrane), cupel GCB allowlist, and handbook/spec narratives (bind track, first-build cohort list, CBG GAR-auth idiom rewritten to ambient gcrane with skopeo token pattern kept as history, RBSCB posture moved to past-tense with the credential-helper rejection record closed as moot). Reviewer-caught restore: rbgjs-token-fetch.sh is NOT caller-less — the immure residency guard (rbgjl09) includes it for its curl blob-HEAD bearer token (curl's floor role; the docket premise predated the immure pace) — file restored byte-identical and the three retired-claims in RBSCJ/RBSCB corrected to name the surviving caller. Stale-test repair rode along: rbtdto onboarding unit tests still asserted the pre-cutover inscribe_reliquary case name (red since the inscribe→conclave cutover; rbw-tt had not run since) — renamed to conclave with ordering assertions intact, 141/141 green. Shellcheck 210 clean; theurge builds.

### 2026-06-10 19:52 - ₢BHAAR - W

Reproducibility audit complete: every image-construction and -acquisition path now refuses on a dirty tree or carries a stated reason in code+spec. Gates added to conjure (rbfd_build, before the relocated base-anchor election — the one sanctioned post-gate write), all four Lode capture verbs (step bodies compose from the tree and write the envelope), and the depot tripwire inscribe. Graft/deletes/standalone-about/elections classified correctly-ungated with reasons recorded. Onboarding teaches the convention only at gated steps (first-build, airgap, bind, payor levy). Shellcheck 210 clean, fast suite 152/0. Residual observation: the Lode envelope stamps no git fact — envelope commit-stamping left as a possible itch.

### 2026-06-10 19:51 - ₢BHAAR - n

Record standalone about's correctly-ungated classification at the function: it constructs metadata for an already-stored image, not an image; its commit stamp cannot be made truthful by gating, and the ordain paths produce about inside their gated builds.

### 2026-06-10 19:49 - ₢BHAAR - n

Close the clean-tree gate gaps the reproducibility audit found. Conjure now gates: bug_require_clean_tree at the top of rbfd_build, before the base-anchor election and before anything (pouch, git stamp, step bodies) leaves the host — election relocates from rbfd_ordain into rbfd_build as the one sanctioned post-gate write (bounded one-line attested rewrite, recorded independently in _RBGR_BASE_LOCATOR_n, surfaced for immediate commit; matches the gauntlet fixture's existing commit-after-ordain design). All four Lode capture verbs gate (ensconce/conclave/underpin/immure) — capture composes its cloud step bodies from the working tree and those steps write the provenance envelope, so the envelope must be the product of committed code; the spine's delete builds stay deliberately ungated. Depot tripwire inscribe gates — it ships tracked rbrd.env bytes as the depot's permanent drift reference. Graft stays ungated with its stated reason extended (GRAFTED verdict already disclaims provenance); mirror's stale 'same as inscribe' comment rewritten to stand alone. bug_git.sh sourced into the three CLI furnishes that lacked it (rbld0, rbrd, rbgp). Specs document each gate (RBSAC full note, RBSLE canonical + three sibling pointers, RBSAG deliberate-exception note, RBSDE/RBSRT tripwire precondition). Onboarding teaches the convention only where code now enforces it: first-build (conclave + yoke-commit), airgap (ensconce/ordain pair + elected-anchor commit), bind (mirror refusal), payor (levy-end tripwire). Shellcheck 210 clean.

### 2026-06-10 18:52 - Heat - T

image-clean-commit-reproducibility-audit

### 2026-06-10 18:38 - ₢BHAAP - W

Windows-deferral half abandoned on a false premise. Git proved the Cygwin+Docker-Desktop and WSL-hosted-docker paths run green recently (₢BVAAJ siege 62/0/0 on BOTH substrates, ₢BVAAB crucible tier, ₢BVAAC dogfight 4/4, ₣BB skirmish 252 under WSL), so the rbhw* Docker handbook tracks document a live, tested config — not stale, nothing to defer. The deferred-banner edits were written, then fully reverted (operator-caught: the paddock's 'stale/nothing live to teach' claim was taken as fact rather than checked). The one genuine staleness found was repaired: rbhwcd's 'Native dockerd in WSL (docker-wsl-native procedure)' precondition cited a handbook procedure that does not exist; repointed to the vendor-docs idiom used elsewhere in the Windows track. Second half done: the ₣A- WSL-stage DEV CACHE revert pace (₢A-ABJ) dropped as obsolete — the wsl Lode kind retires the local 'wsl --export' seed path it formalized; transfer rejected since the only home (₣BH) defers wsl-consumption. Framing lesson banked: a loud DEFERRED banner is the wrong instrument for genuinely-deferred surfaces — silence (don't write greenfield onboarding that doesn't exist) beats advertising a deferral. Shellcheck 210 clean; all three Windows Docker displays render at exit 0.

### 2026-06-10 18:38 - ₢BHAAP - n

Repoint WSL native dockerd precondition from the retired docker-wsl-native procedure to vendor docs in the distro

### 2026-06-10 18:11 - ₢BHAAe - W

Conclave live verify gate discharged and rbi_rq retired. Fresh conclave Lode r260610145233 minted, touchmark yoked across all 9 vessels, conjure ordain of rbev-busybox green end-to-end (build 2m27s + vouch 1m46s) — neither breadcrumb failure mode fired. Dead RBGC/RBGL reliquary constants and all code-side rbi_rq comment examples removed (spec-side residue left to the vocab scrub); legacy rbi_rq GAR packages resolve by the standing depot delete+recreate plan per the banish-last cinch. Service suite green for the record: 17 fixtures, 163 passed, 0 failed, 52m49s — also the first duration baseline of the post-cutover suite, recorded in the durations memo. Surprise found and fixed en route: the rbls_ wire-format sprue sweep had missed the underpin host extract (read unsprued .slot_1.stamp against the cloud author's rbls_ keys) — deterministic wsl-lifecycle failure on first post-sweep run; repointed, and all three single-slot extract die sites now report the keys actually present so a future shape mismatch names itself in the hist log. Two memos banked for the terminal triage: suite-registry inventory (gauntlet/skirmish fast-subset drift, stale CLAUDE.md suite table) and the extract keys-dump triplication cleanup question.

### 2026-06-10 18:10 - ₢BHAAe - n

Service-suite pass for the record: post-cutover service suite green end-to-end — 17 fixtures, 163 passed, 0 failed, 52m49s wall clock against the yoked conclave Lode r260610145233 — discharging the docket's final Done-when criterion. Durations memo baseline section finalized: names the passing run's log as the baseline with its span, demotes the failed 15:05 run to per-fixture-grain-only with the sprue-miss explanation and fix commit pointer, and marks the dirty-gate invocation as unminable.

### 2026-06-10 16:05 - Heat - S

endgame-verification-ladder

### 2026-06-10 15:50 - Heat - n

Triplication memo for the cleanup judgment: the keys-dump diagnostic landed as three byte-parallel copies in the underpin/conclave/immure extracts (operator-flagged should-have-been-a-subfunction). Records why the copies were written (extract bodies are already parallel per-kind by spine design; diagnostic added under a live verify gate, out of extract-architecture scope), the three cleanup options in increasing reach (shared keys helper / one shared single-slot extract with bole's multi-slot loop staying separate / leave-as-is if per-kind-body ownership is judged load-bearing), and the live evidence: the rbls_ sprue sweep missed exactly one parallel copy and broke the service suite hours later — same propagation-drift class as the gauntlet/skirmish fast-subset finding. Dated for the terminal triage sweep.

### 2026-06-10 15:47 - ₢BHAAe - n

Fix the service-suite wsl-lifecycle failure: the rbls_ wire-format sprue sweep missed the underpin host extract, which still read unsprued .slot_1.stamp while the cloud author rbgjl04 writes rbls_slot_1.rbls_stamp — deterministic empty-stamp die on underpin's first post-sweep run (the build itself succeeded; root-caused from the surviving burv-temp output.json, which carried the full sprued payload). Repointed the underpin extract to the sprued keys. Diagnosability: all three single-slot extract die sites (underpin/conclave/immure) now report the keys actually present in the decoded output, so a future key-shape mismatch names itself in the hist log instead of requiring a temp-dir dig; keys captured via the guarded temp-file pattern per BCG (first attempt used an inline unguarded $() in the die message — operator-caught, repaired before commit). Shellcheck 210 clean.

### 2026-06-10 15:39 - Heat - n

Durations memo gains the suite-registry inventory and the service-baseline pointer, both swept by the terminal memo triage. Inventory (operator-reviewed at the conclave-verify mount): all nine suites assessed distinct on load-bearing axes — inclination keep-all — with two findings for disposition: gauntlet/skirmish embed a hand-copied fast subset missing three of fast's current ten fixtures (handbook-render, foundry-path, recipe-validation — reads as propagation drift, which the compile-checked-member rationale does not guard), and CLAUDE.md's test-execution table is stale (counts and the five release/probe suites missing). Service-baseline section records that this heat invalidated prior service durations (suite gained the four Lode lifecycle fixtures) and pins the first post-cutover run's log filename before rotation.

### 2026-06-10 15:05 - ₢BHAAe - n

Conclave live verify gate passed and rbi_rq retired from code. Fresh conclave Lode r260610145233 minted (rbw-lC, 23s), touchmark yoked across all 9 vessels (rbw-dY), and a conjure ordain of rbev-busybox ran green end-to-end against it (build 2m27s + vouch 1m46s) — the cutover's deferred verify gate is discharged; no _RBGR_TAG_SPRUE or actAs flap surfaced. Dead constants removed: RBGC_GAR_CATEGORY_RELIQUARIES and RBGL_RELIQUARIES_ROOT definitions plus every code-side rbi_rq comment example (wrest doc-param, GarRest arg-doc examples repointed to live constants, conclave/step-assembly legacy-layout contrast clauses trimmed). Code-side rbi_rq grep lands clean; the two spec-side hits (RBS0 gar_reliquaries_namespace quoin, RBSDE sibling-namespace prose) stay for the terminal vocab scrub. Shellcheck 210 clean.

### 2026-06-10 14:50 - ₢BHAAO - W

README supply-chain narrative converted from Enshrine/Reliquary to uniform Lode language as one unit (7 sites): glossary entries replaced by a single concept-level Capture entry (per-kind verbs kept out per cinch; the old Reliquary semantic survives as the co-versioned builder-tool Lode), Build Isolation appendix now states builder tools enter the SAME capture gate rather than a parallel one, Airgap/Establishment/GCB-bullet/vessel-table sites repointed to Lode/Capture anchors. Zero enshrine/reliquary/inscribe residue in README; no repo deep-links to the removed anchors. RBSHR stale 'Revised enshrine' horizon entry rewritten as 'Substrate Lode consumption' — records capture as landed (per-kind verbs, Lode as Ark-parallel package noun) and keeps only the genuine horizon (wsl/podvm host-side election/provisioning, Director-pinned nameplate touchmarks, open host-tier-regime question), references repointed to RBSLU/RBSLI/RBSDY; also fixed the stale verb line in the Egress-lockdown entry. Banked Lode/Touchmark concept entries verified against the landed system — read correctly, no edits; noted their Hallmark (not Ark) parallel is honest at README altitude since Ark is not public vocabulary.

### 2026-06-10 14:50 - ₢BHAAO - n

README vocabulary cutover to Capture/Lode plus RBSHR roadmap refresh. README: retired the Enshrine and Reliquary entries in favor of a unified Capture verb producing Lodes — supply-chain section now defines Capture as the single gate (per-kind operations, one co-versioned builder-tool Lode, touchmark naming), with airgap rationale, workflow prose, and the file-tree annotation repointed accordingly. RBSHR: the "revised enshrine" roadmap entry is superseded by landed reality — rewritten as "Substrate Lode consumption" covering the remaining wsl/podvm host-side election/provisioning half (Director-pinned touchmarks, open host-tier regime question, refs to RBSLU/RBSLI/RBSDY); egress-lockdown entry updated to Lode-capture vocabulary.

### 2026-06-10 14:45 - Heat - n

Triage memo from processing the inscribe-cutover handoff: the ~30 enshrine/rbi_es word-level spec hits are already owned by the terminal vocab scrub's KILL list (confirmed, no action); the structural leftovers the scrub disclaims — RBSIA's Enshrinements-Audit section + gar_enshrines_namespace quoin, and the rbst_reliquary_stamp/rbst_touchmark collapse — are recorded with a suggested one-pace disposition shape for the terminal triage to judge. The handoff's HIGH items (deferred live verify, half-cut yoke state, rbi_rq banish residual) were housed directly: new pace slated first among remaining, carrying the conclave-yoke-conjure verify gate, the banish-last tail, and the _RBGR_TAG_SPRUE failure breadcrumb.

### 2026-06-10 14:44 - Heat - S

conclave-live-verify-banish

### 2026-06-10 13:15 - ₢BHAAM - W

Reliquary inscribe->conclave cutover, landed and committed (a510073). Repointed all six tool-addressing sites onto the conclave Lode (one rbi_ld/<touchmark> package, sprued :rbi_<tool> member tags): the zrbfc_resolve_tool_images chokepoint, the host curl preflight, the cloud rbgjr01 preflight (new _RBGR_LODES_ROOT/_RBGR_TAG_SPRUE subs from rbfd/rbfv), and the yoke GAR validation. Retired the inscribe path (rbfli_Inscribe, rbgji01, rbw-dI, RBZ_INSCRIBE_RELIQUARY, rbfl0 RBGJI/reliquary kindle constants) and the four rbi_rq maintenance verbs (rbw-iar/irr/iwr/iJr + rbfl_rekon_reliquary/rbfl_audit_reliquaries; wrest/jettison stay generic path-polymorphic). Retired RBSDI + the rbtgo_depot_inscribe quoin; finalized the ANCHOR derived-pull spec/regime prose (RBSRV, RBS0, rbrv_regime) — the populator zrbfd_elect_base_anchor was already the landed writer from the bole vertical, so #2 was verification not construction; updated RBSDY/RBSIA/RBSIR/RBSIJ and repaired every cross-ref dangle. Flipped onboarding to conclave (rbho* + rbtdro fixture + rbtdrm manifest). VERIFIED GREEN: theurge build (consts regenerated), shellcheck 210, fast suite 152/152, independent BCG/CBG compliance review 0 violations. Surprise found: docket's 'single repoint' was really six coordinated sites. NOT DONE (operator-gated, deferred per the banish-last cinch which orders deletion after the live verify): (1) live conjure-green verify = conclave -> yoke -> ordain(conjure) against live GCP; (2) rbi_rq banish = delete the legacy rbi_rq GAR packages + remove the now-dead RBGC_GAR_CATEGORY_RELIQUARIES/RBGL_RELIQUARIES_ROOT constants (confirmed zero live code refs; only definitions + rbfcg comment examples remain). Also noted out-of-scope: ~30 enshrine/rbi_es spec residue hits (tail of the prior enshrine-retirement pace). Wrapped at operator direction with the live verify + banish carried forward.

### 2026-06-10 13:08 - ₢BHAAM - n

Reliquary inscribe->conclave cutover. Repoint all tool-image addressing onto the conclave Lode (one rbi_ld/<touchmark> package, sprued :rbi_<tool> member tags) across the six coordinated sites: zrbfc_resolve_tool_images chokepoint, the host curl preflight + cloud rbgjr01 preflight (new _RBGR_LODES_ROOT/_RBGR_TAG_SPRUE subs from rbfd/rbfv), and the yoke GAR validation (now lists the Lode's member tags). Retire the inscribe path (rbfli_Inscribe, rbgji01-inscribe-mirror, rbw-dI, RBZ_INSCRIBE_RELIQUARY, the rbfl0 RBGJI/reliquary kindle constants) and the four rbi_rq maintenance verbs (rbw-iar/irr/iwr/iJr + rbfl_rekon_reliquary/rbfl_audit_reliquaries; wrest/jettison stay generic). Retire RBSDI-depot_inscribe.adoc + the rbtgo_depot_inscribe quoin; finalize the ANCHOR derived-pull spec/regime prose (RBSRV, RBS0, rbrv_regime) now that zrbfd_elect_base_anchor is the landed writer; update RBSDY/RBSIA/RBSIR/RBSIJ and repair every cross-ref dangle. Flip onboarding to conclave (rbho* + rbtdro fixture + rbtdrm manifest). rbi_rq GAR namespace + RBGC/RBGL reliquary constants intentionally retained for banish-last after the live conjure verify. Build + shellcheck (210) green.

### 2026-06-10 12:14 - ₢BHAAL - W

Podvm platform-fanout + refresh — landed and live-verified. Native full 8-leaf curation (applehv/hyperv/qemu/wsl x x86_64/aarch64). immure --refresh: add-only/preserve-originals widening of an existing touchmark at its locked version (version derived from the envelope, never passed); host computes the present-set via augur + GAR versions with honest crash-orphan recovery (digest from tag->version, acquired_at from createTime); select splices preserved members without re-resolving the rotating upstream. Operator-cinched wire-format sprues: vouch keys -> rblv_ at schema rbld-vouch-2, buildStepOutputs slot -> rbls_, swept across all four capture authors, augur, body extracts, and the RBSL specs (RBS0 carries the podvm-only per-member-times + dual-position legend; ACG records rbls_ as ACGm_108's second worked application). Empty-selection convergence fix: rbgjl08/09 guard -s->-f. Credentialed-fast-tier footgun disarmed via the buorb_immure_resolve_only resolve-only seam (the fast podvm-resolve case asserts the seam marker so a silently-ignored seam fails rather than firing a live build). Verified: shellcheck 212 clean, theurge builds clean, fast suite 152 passed; gate #1 podvm-lifecycle refresh live-green against GAR; gate #2 native 8-leaf captured (build 5m04s) -> augur decoded the rblv_ v2 envelope end-to-end (8 members, recorded grade) -> banished clean. Known non-tests: orphan recovery is inspection-only (no crash-injection fixture); fast-tier-credless hazard class recorded in the operator memo for the triage pace. Also lands a lode-operation duration log-manifest memo.

### 2026-06-10 12:14 - ₢BHAAL - n

Lode operation-duration log manifest memo: today's passed-test log filenames captured before `../logs-buk/` rotation ages them out, so a later pace can extract per-operation duration expectations. Wall-clock figures already in hand are recorded inline (immure wsl ~12-22s, native 8-leaf 5m04s, ensconce ~10-16s, conclave 38s, underpin 15s, banish ~11-14s FLAT); read-only ops (augur/divine) and lifecycle-fixture spans left as timestamp-span extraction with the method noted (grep 'Wall clock' for cloud ops, last-minus-first bracket timestamp otherwise).

### 2026-06-10 11:51 - ₢BHAAL - n

Podvm platform-fanout + refresh mode. Native full 8-leaf curation lands (applehv/hyperv/qemu/wsl x x86_64/aarch64, memo §5). immure gains --refresh: add-only/preserve-originals widening of an existing touchmark at its locked version (version derived from the envelope, never passed — structurally not a version bump); the host computes the present-set via augur + GAR versions, recovering crash-orphan tags honestly (digest from the tag->version ref, acquired_at from version createTime); the select step splices preserved members without re-resolving the rotating upstream. Operator-cinched wire-format sprues: vouch-envelope keys become rblv_ at schema rbld-vouch-2 and the buildStepOutputs slot becomes rbls_, swept across all four capture authors, augur, the body extracts, and the RBSL specs (RBS0 canonical field def carries the per-member-times-are-podvm-only + dual-position-self-disclosing legend); ACG records rbls_ as ACGm_108's second worked application. Empty-selection convergence fix: rbgjl08/09 guard -s->-f so an all-preserved refresh no-ops instead of dying. Fixtures: service podvm-lifecycle gains a wsl refresh proof (preserve-originals, no membership drift); new fast podvm-resolve asserts both brand mappings host-side under the buorb_immure_resolve_only seam, disarming a credentialed-fast-tier footgun where the colophon would otherwise fire a live build. shellcheck 212 clean, theurge builds clean.

### 2026-06-10 11:41 - Heat - n

Standing-hazard memo from the podvm refresh pace's near-miss: fast-tier credlessness is convention, not construction — the first GCP-capable colophon to enter a fast fixture would have fired a live multi-GB build on a credentialed workstation from inside the fast suite. The instance is being fixed with a buorb_ test-seam; the memo records the structural question (poison marker, egress assert, per-case declaration, or stated convention) for the terminal triage pace.

### 2026-06-10 10:53 - Heat - n

Wire-format key-sprue discipline lands in its three homes, operator-cinched (veiled correct): ACG gains the fourth clause (wire formats are named homes — one minted key-sprue per RB-authored JSON format, foreign schemas keep foreign keys at the Palisade boundary, prospective not retroactive) plus catalog move ACGm_108 (bare wire-key to minted sprue; mutate-now when author and consumers convert in one suite-verified move; first worked application the Lode vouch envelope, rblv_ sprue, schema bump to rbld-vouch-2) and an acronym-registry row. CLAUDE.md's Extended Namespace Checklist gains the JSON-wire-keys row so every future mint enumerates it. CBG's Native-Serializer Rule gains a pointer to the ACG clause — pointer, never a copy.

### 2026-06-10 10:24 - ₢BHAAW - W

Stood up the podvm immure capture vertical (verb rbw-lI; families podvm-wsl/podvm-native via a family argument) and proved it end-to-end live. Deliverables: rbgc_Constants podvm brands + quay family refs + per-family curated leaf-set selections (recorded-at-acquisition grade); the four-step cloud pipeline riding the spine (rbgjl07 python select / rbgjl08 gcrane cp-by-digest / rbgjl09 blob-residency HEAD / reused rbgjl02 vouch); the rbldv_Immure body (opaque-blob x multi-member); divine legend, zipper colophon, tabtarget; RBSLI subdoc + RBS0 rbtgo_lode_immure quoin (covers both families); RBSPV promoted out of FUTURE/ (crane->gcrane, RBSCB cited, ignite-VM marked superseded); and the podvm-lifecycle service fixture (service + complete suites). MECHANISM DEVIATED FROM THE DOCKET'S BASH SKETCH BY DESIGN: the select step is python3 on the gcloud builder, not bash+jq. Parsing the structured upstream OCI index (correlate platform.architecture + annotations.disktype within a descriptor) belongs in python per CBG CBp_, which the no-jq bash GCB allowlist deliberately does not cover (the rbgjl06-package-delete.py precedent); the bash+jq draft tripped the rbtdru_gcb_bash cupel and would have apt-installed an unpinned binary into a privileged build. One live-found bug fixed at the spine: a mixed python+bash recipe needs options.substitutionOption ALLOW_LOOSE (the python step reads automapped env subs, not textual ${} refs, so Cloud Build's strict MUST_MATCH rejected them) — safe because the spine's own coverage check is the replacement protection. Verified: build green, shellcheck clean (212), fast green (151/0); manual lifecycle + the podvm-lifecycle fixture both green live against the disposable depot (capture vw<stamp> 2-member, digests byte-identical to upstream, divine/augur/per-member-jettison/banish/absent); depot left clean. podvm-native wired with a 2-leaf default but fixture-unproven: full curation + curated multi-platform set + refresh mode are the FOLLOWING pace. Five Fable noncritical review findings + three architect follow-ups captured in memo-20260610-heat-BH-fable-review-immure-noncritical.md for terminal triage pace BHAAd (not re-filed as itches).

### 2026-06-10 10:22 - ₢BHAAW - n

Clear stale comments that still described the superseded bash+jq select-step draft as current mechanism (the residue class this heat keeps paying to re-find). rbgjl08 header: 'Step 07 (Debian builder: curl + jq)' -> 'Step 07 (python3 on the gcloud builder)', and the split rationale 'index selection needs jq and gcrane:debug carries neither jq nor curl' -> 'parsing the structured upstream index belongs in python (CBG CBp_ rules), which the gcrane:debug busybox shell cannot host'. rbldv zrbld_immure_submit recipe comment: 'Select + residency on the Debian Google builder (curl + apt jq)' -> 'Select on the gcloud builder (python3 - index parse); residency on the Debian docker builder (curl HEAD)', matching the already-correct function header above it. Sweep confirmed no other immure stragglers: the surviving apt-install/curl+gpg hits in rbgjl04/rbgjl05 correctly describe the underpin/wsl steps, the rbldv host-side jq calls are legitimate workstation jq (the no-jq constraint is cloud-step-only), and rbldv:82 + RBSLI describe the no-jq bash GCB discipline as the reason python is used (accurate, not stale). Comment-only; shellcheck clean (212).

### 2026-06-10 10:18 - Heat - S

terminal-memo-triage

### 2026-06-10 10:17 - Heat - n

Pre-wrap Fable review of the immure pace, noncritical layer: CBp_102's python import floor gains re (it accreted within a day of authoring — the rule now states the enumeration is the temporary home pending the mechanical walk), and a noncritical-findings memo captures five review findings (selection-entry empty-field assert, in-loop stdin hazard belt, ALLOW_LOOSE dead-key forfeit, the floor-accretion evidence, two new Director-RBRA manifold sites for the accessor-seam census) plus the implementing session's three wrap-time follow-ups (rbw-di quay-CDN egress line, 4KB envelope arithmetic before the native fan-out, RBSL acronym-map catalog gap) so the terminal memo-walk pace sees them all.

### 2026-06-10 10:07 - ₢BHAAW - n

Land the podvm specs and the podvm-lifecycle service fixture. New RBSLI-lode_immure.adoc: the immure operation spec covering BOTH quay families (podvm-wsl/podvm-native via the family argument), the declarative no-FQIN family+version intent, cloud-side acquisition, recorded-at-acquisition trust grade, descriptor-keyed leaf selection, and the four-step pipeline (python select / gcrane cp-by-digest / blob-residency HEAD / vouch push) with the recipe-order and capture-pure contracts. RBS0: register the rbtgo_lode_immure quoin (mapping attribute + operation section + include) between underpin and divine. Promote RBSPV out of FUTURE/ to vov_veiled/ (the RBS0 include already pointed there): drop its level-0 doc title so it is a clean RBS0 include fragment (include now carries leveloffset=+2), add a status banner marking the Lode model (RBSLI) as the live design and the ignite-VM/GHCR sections as superseded historical record, and convert the cerebro section's production-tool prose crane->gcrane citing the RBSCB ambient-auth canon, updating the Cloud-direction bullet from the future-tense prototype plan to the landed immure reality (rbgjl07/08/09 + rbgjl02). Theurge podvm-lifecycle service fixture (subagent-authored, opus-reviewed): RBTDRM_FIXTURE_PODVM_LIFECYCLE + its six required colophons (immure/divine/augur/list/jettison/banish), and rbtdrc_podvm_lifecycle in RBTDRC_FIXTURES + the service and complete suites, encoding the verified lifecycle immure podvm-wsl 5.6 -> divine cohort(2) -> augur (kind podvm-wsl, recorded grade, both wsl member tags, origin machine-os-wsl:5.6, no digest-hex assertion since the upstream rotates) -> per-member jettison of rbi_wsl-aarch64 leaving rbi_wsl-x86_64 + rbi_vouch -> banish -> absent. Build clean. The live service-tier run of this fixture follows on the clean tree.

### 2026-06-10 09:58 - ₢BHAAW - n

Spine: add options.substitutionOption ALLOW_LOOSE so a capture build may MIX a python step (reads its substitutions as automapped env vars, never textual ${_RBGL_*}) with bash steps (textual-ref subs). Live-found: the immure build (python select + bash cp/residency/vouch) failed builds.create with HTTP 400 'key ... not matched in the template' for the 7 select-only substitutions — the bash steps' textual refs flip Cloud Build into strict MUST_MATCH, which then rejects the python-only keys as unreferenced (the all-python delete build escapes this because automap suppresses the check when nothing is textually referenced). ALLOW_LOOSE lifts the strict check; it is safe because the spine's own dispatch-time substitution-coverage scan independently catches the inverse fault (a step ${}-referencing a key the blob omits) that MUST_MATCH would have caught. Doc comments updated (the coverage-check rationale now names ALLOW_LOOSE as its paired relaxation; the envelope-shape list gains options.substitutionOption). rbldl divine legend column widened %-10s -> %-13s so 'podvm-native' (12 chars) no longer overflows into the description. Both verified by a full live lifecycle against the disposable depot: immure podvm-wsl 5.6 -> 2-member Lode vw260610095327 (build SUCCESS 22s, digests byte-identical to upstream), divine cohort + legend, augur envelope decode (recorded grade, honest trust posture), per-member jettison (aarch64 removed, x86_64 + vouch survive), banish (flat-package delete 12s), divine absent.

### 2026-06-10 09:52 - Heat - n

CBG exemplar-list de-enumeration per operator pitch: the Snippet & Exemplar Reference drops its per-step filename list (drift-by-construction; the CBh_101 snippet table stays — it carries the requires/provides contract and matches disk) for directory-grain pointers. CBh_103 gains the API-contract cap: buildStepOutputs stores only the first 4KB, ordered by step index. The python-allowlist memo gains the exchange addenda: tool-floor survey verdict (ZRBTDRU_GCB_ALLOWED doc comment is the single authoritative statement; CBG should point, never copy), the vacuous-pass explanation for rbgjv02's unsanctioned gcloud, the expander language-blind probe note, and the 4KB contract citation.

### 2026-06-10 09:43 - ₢BHAAW - n

Rewrite the immure select step in python3 (rbgjl07-immure-select.py), resolving its collision with the no-jq bash GCB discipline. The select step PARSES a structured upstream OCI index (correlate platform.architecture + annotations.disktype within a child descriptor, extract a third field), which the bash 'author-by-hand with grep+cut' allowlist does not cover; the prior bash+jq draft tripped the rbtdru_gcb_bash cupel (jq absent from ZRBTDRU_GCB_ALLOWED) and would have apt-installed an unpinned binary into a privileged build. Python's stdlib json/urllib are the native tools and a .py step is outside the bash command-allowlist (CBG polyglot, CBp_ rules) — the rbgjl06-package-delete.py precedent, confirmed by an architect pass. The python step does the registry-v2 anonymous Bearer flow by Www-Authenticate discovery, validates the family reference is a multi-arch index, selects the curated leaves by descriptor (literal alt-arch spelling x86_64/aarch64, dying with the full available (architecture, disktype) inventory on no-match so a quay rotation reads as a clear diff), asserts exactly one disk-blob layer per leaf, authors the recorded-grade envelope with json.dumps, and writes the selection list (truncate-then-write, idempotent under GCB retry) + stamps + the slot-0 buildStepOutputs the spine extracts. New ZRBLD_GCLOUD_BUILDER (gcr.io/cloud-builders/gcloud:latest) in the kindle; rbldv recipe row repointed to the .py/gcloud/python3, its header reworked to the four-steps-three-builders topology with the recipe-order contract noted (vouch strictly after the residency guard). rbgjl09 residency stays bash (curl allowlisted, header read only) with its tail/tr replaced by an awk END parse to clear the allowlist. py_compile + shellcheck (212 files) clean.

### 2026-06-10 09:42 - Heat - n

Four deft CBG improvements from the Fable review findings: new CBi_105 (a step uses only the tools its builder ships — never apt-get/pip install at build time; a missing tool is a builder-selection problem; first citer the reworked immure select draft), new CBp_102 (python bodies sit outside the *.sh-only conformance walk — hold the stdlib import floor and the subprocess tool floor by hand until the cupel python walk lands, removal condition recorded), a CBh_103 sentence pinning that buildStepOutputs slots are addressed by step index so recipe reordering silently shifts them, and rbgjl06 added to the python-step exemplar list.

### 2026-06-10 09:37 - Heat - n

Fable recommendation memo: extend the cupel with a python import allowlist (cupel walks *.sh only; four .py steps outside conformance; live specimen rbgjv02 subprocess-runs gcloud, absent from ZRBTDRU_GCB_ALLOWED) plus a subprocess policy bridging the two tool floors. Also swept two more disproven NOT_FOUND-cascade residue sites the first sweep missed: rbgjl03's single-platform rationale comment and RBSLC's matching spec passage, both now stating the proven FAILED_PRECONDITION parent-before-child mechanism and pointing at the landed convergence delete.

### 2026-06-10 09:25 - ₢BHAAW - n

Stand up the podvm immure capture vertical (verb rbw-lI; families podvm-wsl/podvm-native selected by a family argument). New rbgc_Constants: podvm brands, the two quay family refs (machine-os-wsl/machine-os), and per-family curated leaf-set selections — recorded-at-acquisition grade (quay rotates podvm out within days, no durable checksum). Three new in-pool cloud steps riding the spine, with rbgjl02 vouch-push reused: rbgjl07-immure-select (Debian, curl+apt-jq — anon-reads the quay family index over the registry-v2 API and selects the curated {disktype x arch} leaves by index child DESCRIPTOR platform.architecture + annotations.disktype, never the unreliable layer filename; fetches each leaf manifest for its single zstd blob digest+size; authors the recorded-grade members[] envelope); rbgjl08-immure-capture (gcrane:debug — gcrane cp each leaf BY DIGEST into rbi_ld/<vw|vn><stamp>:rbi_<disktype>-<arch>, with a cheap manifest digest-equality readback; single-platform leaves keep the package flat so banish stays single-call atomic); rbgjl09-immure-residency (Debian, curl — registry-v2 blob HEAD Content-Length == declared layer size, the anti-hollow-mirror guard the recorded grade demands, Mason-SA token via the metadata server). New rbldv_Immure body (opaque-blob x multi-member, blending rbldw+rbldr): resolves the family argument to (kind-letter, quay family, selection, brand), composes the 4-step recipe + substitutions blob, rides zrbld_spine_dispatch/_extract, and emits the two bare single-form chaining facts (touchmark + brand). Wired: rbld0_Lode sources the body + defines ZRBLD_IMMURE_PREFIX; rbldl divine legend gains the vn/vw kind lines; rbz_zipper enrolls rbw-lI (param1, capital colophon — capture mutates GAR); new trampoline tt/rbw-lI.DirectorImmuresPodvm.sh. rbtdgc_consts.rs (RBTDGC_IMMURE_PODVM) + claude-rbk-tabtarget-context.md build-regenerated from the zipper. podvm-wsl is the fixture-proof target (both wsl-disktype leaves -> a genuine 2-member Lode exercising multi-member + per-member jettison); podvm-native is wired with the cerebro experiment's modest 2-leaf default, its full 8-leaf curation + same-version refresh mode deferred to the FOLLOWING pace. Build green, shellcheck clean (213 files). The cloud pipeline is only provable by a live service-tier Cloud Build (in-pool busybox is not locally exercisable) — the podvm-lifecycle fixture is the gate, still to land alongside the RBSLI/RBS0 specs and the RBSPV promotion.

### 2026-06-10 09:10 - Heat - n

Two Fable recommendations from the second post-wrap review pass: the elect_base_anchor exactly-one-ORIGIN die is a dormant sibling of the fixed brand-leak ordain killer (fires when a multi-origin vessel ordains behind a chained bole fact; no such vessel exists today); rbfl_jettison's 'Jettisoned or nonexistent' success message contradicts its die-on-404 code — recommend tolerating 404 to match the idempotent-delete house philosophy.

### 2026-06-10 08:59 - ₢BHAAY - W

Retired the dead enshrine-creation operation. Deleted RBSAE-ark_enshrine.adoc and the rbtgo_ark_enshrine quoin (mapping attribute, RBS0 operation section + include, acronym entry). Repointed the live cross-refs off the dead quoin: conjure's 404 directive and RBSFH's supply-chain-anchor chain to the bole ensconce quoin (rbtgo_lode_ensconce); RBSIR's omitted-domain note to plain legacy-enshrinement prose; RBSHR's roadmap pointer to RBSLE. Reworded the vessel-regime ANCHOR consumption prose (RBS0 rbrv_image_origin/rbrv_image_anchor + RBSRV) off the dead enshrine to-enshrines-namespace flow onto the bole-Lode capture + derived-pull-election model, matching the landed rbfd code comment — spec now slightly leads code (the populator is unwired). Zero hits on the three retired patterns repo-wide; all referenced quoins defined; fast green 151/0. Plan-gap survey surfaced that the bole derived-pull ANCHOR populator (code comments call it 'a later election') was owned by no pace; per operator decision it was folded into the reliquary inscribe-cutover, which also finalizes the ANCHOR prose against the wired populator.

### 2026-06-10 08:58 - ₢BHAAY - n

Retire the dead enshrine-creation operation. Delete RBSAE-ark_enshrine.adoc and the rbtgo_ark_enshrine quoin (mapping-section attribute, the RBS0 operation section + include, and the acronym-map entry) — the bole cutover already deleted its step scripts and the rbfd_enshrine gesture, leaving only a spec describing a path nothing runs. Repoint the live cross-references off the dead quoin: conjure's 404 directive and RBSFH's supply-chain-anchor chain now name the bole ensconce quoin (rbtgo_lode_ensconce); RBSIR's omitted-domain note becomes plain legacy-enshrinement prose; RBSHR's roadmap pointer moves to RBSLE-lode_ensconce.adoc. Reword the vessel-regime ANCHOR consumption prose (RBS0 rbrv_image_origin/rbrv_image_anchor + RBSRV) off the dead enshrine to-enshrines-namespace flow onto the bole-Lode capture + derived-pull-election model, matching the landed rbfd_FoundryDirectorBuild comment. Zero hits on the three retired patterns repo-wide; every newly-referenced quoin (rbtgo_lode_ensconce, rbtga_lode, rbtga_touchmark, gar_lodes_namespace) is defined. The bole ANCHOR populator wiring and final ANCHOR-prose finalization were folded into the reliquary inscribe cutover per the plan-gap survey; this pace leaves that prose at the bole-model level. fast verification follows on the now-clean tree.

### 2026-06-10 08:33 - ₢BHAAN - W

Divine/augur grain split landed (code at cdde7812c) and comprehensively live-verified. augur (rbw-la) is the new read-only single-Lode inspect verb: it lists member tags and decodes the :rbi_vouch provenance envelope (kind/acquired/trust_grade/members[] with origin/digest/verification) via zrbfc_gar_extract_artifact on rbgjl02's vouch.json layer, reporting honest Pale trust-grade posture; divine is trimmed to enumerate-only. Kind name reads from each envelope's own kind field, so a new Lode kind needs no augur change. All three divine-inspect call sites repointed to augur.

### 2026-06-10 07:47 - ₢BHAAN - n

Realize the divine/augur grain split: split single-Lode inspect out of divine into the new read-only augur verb (rbw-la), trim divine to enumerate-only, and implement the augur substance the spec mandates — decoding the :rbi_vouch provenance envelope (kind/acquired_at/acquired_by/capture_build/trust_grade/members[]) which divine's inspect branch never read. augur validates the touchmark folio (as banish requires it) and its <kind><YYMMDDHHMMSS> format, fetches the package member tags, then extracts the :rbi_vouch FROM-scratch artifact (rbgjl02's vouch.json layer at image root) via zrbfc_gar_extract_artifact, renders the decoded fields, and reports honest trust-grade posture — verified-against-published vs recorded-at-acquisition, Pale-honest, never over-claiming what the upstream permits. Kind name is read from the envelope's own kind field, so augur stays kind-agnostic (a new Lode kind needs no change here). New ZRBLD_AUGUR_PREFIX in the kindle; zipper enrolls rbw-la (param1) and drops divine's folio channel to none (enumerate-only); new read-only tabtarget tt/rbw-la.DirectorAugursLode.sh mirrors the divine trampoline (no BURD_INTERACTIVE). Repointed all three lifecycle divine-inspect call sites (lode/reliquary/wsl) to augur in rbtdrc_crucible.rs: the lode case carries the explicit envelope-decode assertions (trust grade + a member's oci-digest verification — markers that live inside vouch.json, never in a tag listing, so they prove decode rather than enumeration), and the reliquary case asserts the cohort N-member envelope decodes. rbtdgc_consts.rs (RBTDGC_AUGUR_LODE) and claude-rbk-tabtarget-context.md build-regenerated from the zipper. Build green, shellcheck clean (209 files); fast suite and service-tier live GAR verification follow this clean-tree commit, the live run batched with adjacent paces at operator discretion per the paddock Execution posture.

### 2026-06-10 05:58 - Heat - n

Five Fable review recommendations from the cloud-dispatch delete architecture review, one memo per correctable behavior: pin the delete builder by digest (Director-privileged floating builder), progress-aware convergence deadline (fixed 180s wrong-shaped for large webs), URLError tolerance in fire_delete, urllib socket timeouts, and the self-actAs propagation flap awaiting the first pristine gauntlet.

### 2026-06-10 07:03 - ₢BHAAa - W

Image-backdoor path-polymorphic verbs landed and live-verified. Minted the type-blind raw trio on final-form bare-letter colophons: rbw-il (new rbfl_list — iterative GAR-path narrowing, envelope-independent, shows walking-dead debris by design), rbw-iw (generic wrest), rbw-iJ (generic jettison, now parsing both :tag and @sha256: version refs). Retired enshrinement variants iwe/iJe; the surviving hallmark/reliquary variants and the two waves that clear the deliberate iw/iJ terminal-exclusivity violation are recorded in memo-20260610-heat-BH-image-tabtarget-cleanup. Fixed the deferred 010ec044a theurge build break (invoke_count pub(crate) unreachable from the bin crate) via invoke_count()/set_invoke_count() accessors. Added the reliquary-lifecycle member-jettison theurge case (rbw-il + rbw-iJ prove tag-grain delete removes one member, leaves the sibling + Lode intact). Discovered rbtdgc_consts.rs + claude-rbk-tabtarget-context.md are build-regenerated from the zipper; added a claude-rbk-acronyms.md note to head off future confusion. fast green (151/0). Verified live against the disposable depot across the full list/wrest/jettison permutation matrix — list at every grain, wrest by tag+digest, tag-jettison (tag-only, aliases survive), digest-jettison (version reaped), cross-package independence, honest-fail on tag-referenced digest delete — all correct, zero surprises. The new theurge case's standalone live run is batched per the execution posture.

### 2026-06-10 07:00 - ₢BHAAa - n

Image-backdoor path-polymorphic verbs: mint the type-blind raw trio on final-form bare verb letters — rbw-il (list: iterative GAR-path narrowing, new rbfl_list, envelope-independent), rbw-iw (wrest any ref), rbw-iJ (jettison a tag or @sha256: version; rbfl_jettison gains digest-ref parsing). Retire enshrinement variants iwe/iJe; hallmark/reliquary variants survive per the staged cleanup. Terminal exclusivity deliberately violated on iw/iJ (operator-blessed final form; memo-20260610-heat-BH-image-tabtarget-cleanup records the two waves that clear it). Fix the deferred 010ec044a theurge build break (invoke_count pub(crate) unreachable from the bin crate) via invoke_count()/set_invoke_count() accessors routed through the suite loop. Add the reliquary-lifecycle member-jettison case: rbw-il + rbw-iJ prove tag-grain delete removes one member and leaves siblings intact. rbtdgc_consts.rs + claude-rbk-tabtarget-context.md are build-regenerated from the zipper; claude-rbk-acronyms.md gains a note flagging that to head off future confusion. Verified live against the disposable depot across list/wrest/jettison permutations — all correct.

### 2026-06-10 05:57 - ₢BHAAT - W

Crane-embrace gate verified live against GAR: capture on gcrane+curl+gpg (docker/buildx evicted) proven by the three service-tier fixtures (lode/reliquary/wsl) green, with the unverified-live conclave single-platform fix holding. The same run discharged live verification for the preceding cloud-dispatch delete-architecture pace via the fixtures' banish/abjure legs. Surfaced and fixed a real bug the gate exposed: the bole base-anchor election (zrbfd_elect_base_anchor) buc_die'd on any non-bole Lode kind-brand, so a wsl underpin or reliquary conclave chained ahead of a conjure killed the ordain pre-submit (a real operator sequence; here batch-vouch's leaked-fact path). Fixed to no-op on reliquary/wsl brands, leave the base ANCHOR as-is, reserve buc_die for a brand outside the authored enum (commit 91666a97b); verified end-to-end by a clean service suite (161 passed, 0 failed) whose batch-vouch ordain exercised the new arm with a real wsl brand in previous/. Also repaired the theurge BURV invoke-dir isolation leak that fed the leak (per-fixture invoke-counter reset collided every fixture's first invoke on invoke-00000, so bud's current/->previous/ promotion bled prior facts into a non-chained invoke; suite loop now threads the counter monotonically, commit 010ec044a, compile/run-verification deferred to slated pace ₢BHAAc).

### 2026-06-10 05:56 - Heat - S

burv-invoke-dir-isolation-verify

### 2026-06-10 05:55 - ₢BHAAT - n

Thread the BURV invoke counter across fixtures in the theurge suite loop so per-invoke dir names are suite-monotonic instead of resetting to invoke-00000 each fixture. The per-fixture reset made every fixture's first invoke reuse invoke-00000; bud's start-of-dispatch current/->previous/ promotion then leaked the prior fixture's chaining facts into a non-chained invoke's previous/ — the mechanism by which wsl-lifecycle's lode facts reached batch-vouch's ordain election and killed it. Only the suite loop carries the count forward; Context::new still defaults invoke_count to 0, so the rbtdti unit tests and the single-fixture/single-case paths are unchanged, and the chain_next logic (relative count-1) still resolves within-fixture. NOT yet compiled or run per operator instruction — verification deferred to a slated pace that checks the post-repair BURV dir count against the invocation count (pre-repair residue: 10 dirs / 32 invocations).

### 2026-06-09 21:20 - ₢BHAAT - n

Base-anchor election no-ops on non-bole Lode brands instead of fatal-dying. zrbfd_elect_base_anchor read the chaining-fact kind-brand and buc_die'd on anything but bole, so a wsl underpin or reliquary conclave chained ahead of a conjure (a real operator sequence — and the batch-vouch fixture's leaked-fact path) killed the ordain at one-second-in, pre-submit. Now reliquary/wsl brands leave the base ANCHOR as-is (only a bole carries a base image to elect), reserving buc_die for a brand outside the authored enum; the brand is read before the Electing step so it no longer mislogs Electing-from-bole before dying. Header contract updated to match.

### 2026-06-09 20:13 - ₢BHAAZ - W

Cloud-dispatch GAR package delete landed and live-proven. banish (Lode) + abjure (hallmark) now dispatch a Director-run delete-build via the build-assembly spine (caller-supplied serviceAccount; spine decoupled to rbfc-level so abjure cross-sources it from the rbfl process — the cinch-blessed narrow cross into made-side delete); Director gained serviceAccountUser-on-self.

### 2026-06-09 18:43 - ₢BHAAZ - n

Review residue sweep: purge the two superseded delete-mechanism descriptions (per-package in-pool LRO poll; the disproven code-5 NOT_FOUND membrane) from shell comments and spec step bodies, aligning all prose with the landed rbgjl06 convergence mechanism (fire deletes, absence-poll to 404). Seven sites; rbgjl06 itself was already clean.

### 2026-06-09 18:32 - ₢BHAAZ - n

Cloud-dispatch GAR package deletes: banish (Lode) and abjure (hallmark) move off host trust-200 REST onto Director-run delete-builds. The build-assembly spine gains a caller-supplied serviceAccount (Mason for capture, Director for delete) and decouples to rbfc-level (zrbfc_sentinel + guard) so abjure rides it from the separate rbfl process via cross-source — the cinch-blessed narrow cross into made-side delete. New in-pool python step rbgjl06 deletes by convergence: the real GAR constraint is FAILED_PRECONDITION parent-before-child (a single packages.delete of a multi-arch web removes nothing), not the paddock's guessed LRO NOT_FOUND code 5 — so each round fires a delete at every version (force=true) and the package shell, skips the per-round referenced-by-parent preconditions, and polls the package GET until 404 (absence the only truth, deadline the only failsafe; the gcr-cleaner --skip-errors shape, like host rbuh_poll_until_gone). New shared delete body rbldd_ resolves the Director run-as SA and composes the one-row delete recipe; banish/abjure rewired to it; Director gains serviceAccountUser-on-self in invest/divest. Specs RBSLB/RBSAA/RBSCB rewritten to the convergence mechanism. Live-proven against depot canest3bhm100001: two index-web reliquary banishes (16s) + a six-package conjure-hallmark abjure (55s), all converged in-pool.

### 2026-06-09 18:31 - Heat - d

paddock curried: correct atomic-delete premise: real GAR failure is FAILED_PRECONDITION parent-before-child (code 9), not LRO NOT_FOUND (code 5); delete is by convergence, live-proven

### 2026-06-09 16:17 - Heat - r

moved BHAAY after BHAAN

### 2026-06-09 16:17 - Heat - r

moved BHAAN after BHAAa

### 2026-06-09 16:17 - Heat - S

yoke-colophon-consumer-move

### 2026-06-09 16:16 - Heat - S

image-backdoor-path-verbs

### 2026-06-09 16:14 - Heat - d

paddock curried: groom 2026-06-09: cascade-bug correction + cloud-dispatch delete cinch, landed pinning boundary, path-polymorphic backdoor reconception, execution posture (disposable depot, batching, side lane, tiering), heat-nature refresh

### 2026-06-09 14:53 - Heat - n

Recognize Fable as a frontier model tier and admit it through the JJK model gate alongside opus — zjjrm_extract_tier now returns "fable" and zjjrm_check_model_gate accepts opus or fable

### 2026-06-09 15:38 - Heat - n

Memo: delete-architecture regression as a spec-hygiene anchor. Forensic record that image deletion runs locally (workstation curl DELETE against GAR) when intended cloud-dispatched; the cloud-dispatched shape existed in the GitHub Actions era (rbgh_delete_workflow -> rbga_dispatch/rbga_wait_completion), was removed outright at the 2025-08-07 Cloud Build pivot (e1a9f16f8, 'unnecessary anymore'), and was reintroduced local a week later (0b6662676, 2025-08-14) -- wrong for ~10 months. Root cause framed as spec-hygiene: RBSAA/RBSIJ were authored/aligned to match the already-regressed code (alignment commit 525b7fac pulled implementation into spec), so the spec blessed the regression and could never flag it. Improvement hypotheses per spec system: RBS0 execution-locus Key Premise (precedent rbsk_pinning_boundary), MCM operation slot to voice binding invariants, AXLA positive dual of axk_premise (locus motif), BUS0 open question on surfacing locus at dispatch layer (weakest link, recorded as question), JJS0 removal-should-leave-a-scar plus the documented --force docket-drift instance. Ties to the CMK roe covenant (spec must lead code, not transcribe it). Anchor only; the concrete delete repair is the deferred reassess pace already noted in BH.

### 2026-06-09 14:43 - Heat - r

moved BHAAZ before BHAAT

### 2026-06-09 11:37 - Heat - S

gar-delete-host-cloud-boundary

### 2026-06-09 11:35 - ₢BHAAT - n

Conclave captures single-platform (linux/amd64) — restore intent and dodge the GAR multi-arch packages-delete failure. rbgjl03: gcrane cp -> gcrane --platform linux/amd64 cp, so each cohort member is one amd64 manifest (the cohort is consumed only as GCB step images on amd64 workers; other platforms are dead bytes). The full multi-arch index made the reliquary Lode package a 55-version parent-index/child-manifest web that GAR `packages delete` cannot unwind in one cascade — it deletes a parent, GAR auto-removes the protected children, then the cascade reaches an already-gone child and the LRO completes with NOT_FOUND (code 5); reproduced on a fully-settled package, so structural, not a race. gcrane digest stays index-digest (unchanged from docker RepoDigests). Conclave-only — the --platform flag must not leak to bole/wsl, which keep full-fidelity capture. RBSLC updated to the gcrane mechanism (was still describing docker pull/tag/push), with the phenomenon, rationale, the conclave-only boundary, and the GAR cleanup-policy reference retained. Verified via depot Cloud Build log + a direct REST delete experiment (operator OAuth token); web-confirmed against GAR's parent/child manifest deletion rule. NOT yet run live — reliquary-lifecycle re-run is the next gate step. The banish trust-200 LRO gap and the host-vs-cloud delete-architecture question are deferred to a separate reassess pace. Shellcheck 208 clean.

### 2026-06-09 10:47 - ₢BHAAT - n

Fix the conclave Cloud Build failure the reliquary-lifecycle gate surfaced: 'script: line 39: syntax error: unexpected (' — a bash array under busybox sh. Front A flipped conclave's builder to gcr.io/go-containerregistry/gcrane:debug and its entrypoint to busybox, but left the cohort as a bash MANIFEST=( ... ) array with ${MANIFEST[@]} expansion; gcrane:debug's only shell is /busybox/sh (POSIX, no arrays), so the step died at parse. Convert the cohort to a |-split heredoc consumed by a while IFS='|' read -r NAME UPSTREAM loop — the same busybox-compatible iteration idiom rbgjl02's stamp loop already uses; cohort content is byte-identical (still mirrors rbgji01's 7 tools verbatim). lode-lifecycle passed because rbgjl01/02/05 carry no arrays, so conclave was the lone step that took a bashism into the busybox shell. Diagnosed from the depot Cloud Build log (build 65f87e1a, step 0 conclave-capture). Shellcheck 208 files clean (shellcheck lints these as bash so it could not have caught the busybox-array divergence; the gate did).

### 2026-06-09 10:06 - ₢BHAAT - n

Repair the cupel gcb-bash gate failure the service-suite run surfaced. (1) Add the 7 legitimate GCB container tools the capture scripts use to ZRBTDRU_GCB_ALLOWED: gcrane/tar (this pace's crane-append) plus shasum/openssl/apt-get/gpg/head (the wsl-underpin fetch+verify — a latent gap, since BHAAJ ran fast-qualify but not the cupel-bearing fast suite, so its commands never hit the gate). Refresh the allowlist provenance comment — openssl/gpg/apt-get are now legitimately present on the Debian wsl-underpin fetch builder and gcrane/tar in gcrane:debug busybox, while jq stays absent everywhere; membership stays per-container-presence, not declared-dep inheritance. (2) Engine: rbtdre_Verdict::Fail(msg) was matched as Fail(_), discarding the verdict message; bind and echo it so every failing case surfaces its detail on-console, not only in the trace file — general win for all fixtures. (3) Sync CBG: add gcrane-append and gpg-verify-sums to the CBh_101 snippet contract table and the snippet-library enumeration, both of which had drifted. No BCG edit — its host-portability eviction table is correctly disjoint from the GCB container-presence allowlist.

### 2026-06-09 09:46 - ₢BHAAT - n

Re-yoke all 9 vessels to reliquary r260609093011 — the re-inscribed cohort now carries gcrane (yoke validation confirmed all 7 tools present), which bole's zrbfc_resolve_tool_images pins for its capture-path gcrane. Gate prerequisite for the crane-embrace-eviction service-tier run: the standing r260605074843 predated gcrane joining the cohort, so the suite's lode-lifecycle bole capture would fail to resolve a pinned gcrane against it. Toolchain :latest bumped r260605074843 -> r260609093011 across all vessels (re-inscribe pulls :latest). Legacy rbi_rq bare-name addressing retained per cinch; sprued :rbi_<tool> addressing is the deferred BHAAM cutover.

### 2026-06-09 09:20 - ₢BHAAT - n

Front B — evict buildx from the capture path; move the two FROM-scratch builds onto gcrane append. New forked Lode-only snippet rbgjs-gcrane-append (tar the ctx dir + gcrane append --oci-empty-base) replaces buildx-bootstrap/buildx-push for the Lode callers; the buildx snippets stay for made-side multi-platform rbgjv03 (NOT converted — out of scope). rbgjl02 vouch-push: buildx -> gcrane append, busybox. Underpin SPLIT to resolve the curl/gpg-on-busybox wrinkle: rbgjl04 becomes fetch+GPG-verify+stage+envelope only (stays on the Debian builder for curl+gpg, no longer pushes), new rbgjl05 gcrane-appends the staged rootfs.tar (busybox gcrane builder); slot-0 buildStepOutputs extract unchanged (rbgjl04 still authors it). rbldb_Bole: both rows -> PINNED z_rbfc_tool_gcrane (bole is vessel-adjacent, zero unpinned aspects), busybox. rbldw_Underpin: 3-row recipe (fetch on Debian / wrap + vouch-push on FLOATING gcrane) — wsl evicted but NOT pinned (vessel-less; pinning is the bootstrap-digest-pin itch). Also reworded 'docker/gcrane vouch' -> 'vouch-push' across all three bodies (vouch names the artifact, not a tool-modified thing; the recipe rows declare the builder). 208 files shellcheck-clean. Not yet run live — the service-tier gate (lode+reliquary+wsl) is the operator's next step; both fronts must land before any conclave/bole/underpin build executes (rbgjl02 row entrypoints assume the gcrane conversion).

### 2026-06-09 09:08 - ₢BHAAT - n

Front A — evict docker from the conclave capture step. rbgjl03: docker pull/tag/push -> gcrane cp (daemonless registry->registry, manifest digest preserved), docker inspect RepoDigests -> gcrane digest (same canonical sha256, CBb_101 guarded $() in-step), and add gcrane to the cohort MANIFEST as the :debug variant so the conclave-built rbi_ld cohort carries gcrane with a busybox shell; member tags stay sprued (:rbi_<tool> via _RBGL_TAG_SPRUE). rbldr_Reliquary: both recipe rows flip ZRBLD_GOOGLE_DOCKER_BUILDER->ZRBLD_GCRANE_BUILDER, entrypoint bash->busybox; conclave is generation-tier so it keeps the FLOATING bootstrap gcrane (the one phase the pinning rule permits unpinned), not a reliquary-resolved builder. Transient: the rbgjl02 vouch row now declares busybox but rbgjl02 is still buildx until Front B converts it — never executed before the gate (both fronts land first). 206 files shellcheck-clean.

### 2026-06-09 08:56 - ₢BHAAT - n

De-magick the inscribe-tabtarget hint in yoke's missing-cohort buc_die: replace the hardcoded tt/rbw-dI.DirectorInscribesReliquary.sh literal (colophon + frontispiece, rots on rename) with buyy_tt_yawp over its canonical colophon home RBZ_INSCRIBE_RELIQUARY. buc_die resolves the diastema-wrapped yelp through buyf_format_yawp (zbuc_print -> zbuc_tint), so it renders clean; buym is kindled (buc_die itself yawps) and zrbz_kindle runs in rbfl0_cli furnish, so both deps are reachable. Carried forward from the original verbatim in the prior notch; corrected on review.

### 2026-06-09 08:49 - ₢BHAAT - n

Front B-prereq: gcrane joins the reliquary cohort (the pinning floor). Mint RBGC_RELIQUARY_TOOL_GCRANE plus its z_rbfc_tool_gcrane resolver global (rbfc0) and resolution line (zrbfc_resolve_tool_images in rbfca), so a sealed-reliquary-consuming capture can resolve a PINNED, project-GAR gcrane rather than the floating gcr.io bootstrap (RBS0 rbsk_pinning_boundary / RBSCB). Add gcrane to the legacy inscribe MANIFEST (rbgji01) — consumers pin the inscribe-built rbi_rq reliquary pre-cutover, so the standing reliquary must carry gcrane for the gate; mirrored as the :debug variant so the resolved builder keeps its busybox shell. Make yoke validation (rbfly_Yoke) require gcrane and de-magick the hardcoded tool count/roster (derive both from the expected array) so the message stays accurate across future cohort changes (e.g. the skopeo eviction). Bare RBGC_RELIQUARY_TOOL_* stay as sprue seeds; the sprued :rbi_<tool> member-tag addressing is the cutover's job (₢BHAAM reslated to cinch that). Pinning floor only — the consumption rows that resolve pinned gcrane land in Fronts A/B.

### 2026-06-09 08:03 - Heat - n

Consolidate the supply-chain pinning canon into the specs. Mint RBS0 Key Premise rbsk_pinning_boundary: a build consuming a sealed reliquary carries zero unpinned aspects (every tool/base resolves content-pinned from project GAR), while reliquary generation (inscribe/conclave) is the one phase permitted to pull unpinned upstream; the capture family obeys this by making gcrane a reliquary cohort member, leaving the floating bootstrap gcrane only in conclave. Record the matching RBSCB posture (Supply-chain pinning boundary in Current Posture), the gcr.io provenance decision (Container Registry storage shut down but Google-owned gcr.io images are AR-served and exempt; RB stores everything in Artifact Registry so the shutdown is non-load-bearing; bootstrap pins are a sibling itch), and the registry-transition references. Heat-affiliated: this is governing canon recorded outside the reslated eviction pace.

### 2026-06-09 07:58 - Heat - d

paddock curried: record operator pinning rule: generation may be unpinned, sealed-reliquary builds fully pinned; gcrane joins reliquary cohort; gcr.io bootstrap-only

### 2026-06-09 07:13 - ₢BHAAS - W

Made the airgap-chain fixture genuinely exercise the bole derived-pull election, closing the structural false-green. A one-shot chain_next flag on rbtdri_Context reuses the prior invoke's BURV root, so bud_dispatch's current/->previous/ promotion crosses the marked invoke and ordain's election fires instead of hitting its empty-previous no-op; the airgap-chain case asserts the forge anchor became rbi_ld/<touchmark>:rbi_bole. Verified LIVE against GAR (election rewrote the forge anchor to a fresh touchmark; ensconce ran on the gcrane path). Reconciled onto origin via cherry-pick from the parallel chat's verified-live branch (code at 7e3770c6f + 4d89aa018, plus 3 ordain-airgap regime commits); the superseded skopeo-eviction wrap was left behind per the crane-embrace consolidation.

### 2026-06-09 06:16 - ₢BHAAS - n

Make the airgap-chain fixture genuinely exercise the bole derived-pull election, closing the structural blindness that false-greened it. Root cause: theurge gives every tabtarget invoke its own BURV_OUTPUT_ROOT_DIR, so bud_dispatch's current/->previous/ promotion never crosses invokes — ordain's previous/ was always empty, the election hit its no-op guard, and the conjure built from the stale rbi_es base. Fix (rbtdri_invocation.rs): a one-shot chain_next flag on rbtdri_Context (chain_next_invoke()); a marked invoke reuses the immediately-prior invoke's BURV root instead of minting a fresh one, so bud promotes the predecessor's current/ into the marked invoke's previous/ — restoring the operator's shared ../output-buk depth-1 chain for exactly the invokes that need it, leaving all other isolation intact (the cinch). Fixture (rbtdro_onboarding.rs): rbtdro_ensconce now returns the fresh touchmark, read off current/ before the chained ordain promotes it away; the airgap-chain case marks ordain-forge to chain off the ensconce, then asserts the forge RBRV_IMAGE_1_ANCHOR became rbi_ld/<touchmark>:rbi_bole — a non-firing election leaves the older committed stamp and fails the case. Added a symmetric rbtdro_read_vessel_env reader for the witness. Guards (rbtdti_invocation.rs): chained invoke reuses the prior root without advancing the counter and isolation resumes after; chain_next with no prior invoke errors loud. Mechanism was mount's call per the docket: chose the context one-shot flag over a threaded bool (would churn ~40 call sites) or twin _chained helpers (parallel infrastructure the heat rejects). Build + 141 theurge unit tests green; live onboarding fixture not yet run.

### 2026-06-09 05:51 - ₢BHAAT - n

Fix: add the missing busybox case arm to the spine's entrypoint->shebang switch (zrbld_spine_dispatch). The prior notch updated the doc comment to list busybox but omitted the actual case branch, so dispatch buc_die'd 'Unknown entrypoint busybox' on the bole gcrane recipe row. Now busybox -> #!/busybox/sh, matching the gcrane:debug builder's only shell. Caught by the lode-lifecycle fixture.

### 2026-06-09 06:52 - Heat - T

lode-buildx-eviction

### 2026-06-09 06:52 - Heat - T

lode-docker-eviction

### 2026-06-09 06:51 - Heat - T

crane-embrace-eviction

### 2026-06-09 06:44 - ₢BHAAS - W

Made the airgap-chain fixture genuinely exercise the bole derived-pull election, closing the structural false-green. Root cause: theurge gives every invoke its own BURV_OUTPUT_ROOT_DIR, so bud_dispatch's current/->previous/ promotion never crossed invokes and ordain's election always hit its empty-previous no-op guard. Fix: a one-shot chain_next flag on rbtdri_Context (chain_next_invoke()) — a marked invoke reuses the immediately-prior invoke's BURV root, so bud promotes the predecessor's current/ into the marked invoke's previous/, restoring the operator's shared-../output-buk depth-1 chain for exactly the invokes that need it while leaving all other isolation intact (the cinch). rbtdro_ensconce now returns the fresh touchmark (read off current/ before the chained ordain promotes it away); the airgap-chain case marks ordain-forge to chain off the ensconce, then asserts the forge RBRV_IMAGE_1_ANCHOR became rbi_ld/<touchmark>:rbi_bole — a non-firing election leaves the older committed stamp and fails the case. Added symmetric rbtdro_read_vessel_env reader + two unit guards (chained invoke reuses prior root without advancing counter, isolation resumes after; chain_next with no prior invoke errors loud). Mechanism was mount's call: chose the context one-shot flag over a threaded bool (~40-site churn) or twin _chained helpers (parallel infrastructure the heat rejects). Code notched at 707b7bec2 (build + 141 theurge unit tests green). VERIFIED LIVE: the case PASSED through theurge against live GAR — the election rewrote the forge anchor from the stale rbi_ld/b260608102223 to this run's fresh rbi_ld/b260609062006:rbi_bole, proving the chain fired (the assertion would have failed otherwise); ensconce ran on the gcrane capture path. The live run added 3 plain fixture commits advancing forge/airgap-bottle/moriah regimes to today's artifacts.

### 2026-06-09 06:16 - Heat - S

lode-enshrine-spec-retire

### 2026-06-09 06:16 - ₢BHAAS - n

Make the airgap-chain fixture genuinely exercise the bole derived-pull election, closing the structural blindness that false-greened it. Root cause: theurge gives every tabtarget invoke its own BURV_OUTPUT_ROOT_DIR, so bud_dispatch's current/->previous/ promotion never crosses invokes — ordain's previous/ was always empty, the election hit its no-op guard, and the conjure built from the stale rbi_es base. Fix (rbtdri_invocation.rs): a one-shot chain_next flag on rbtdri_Context (chain_next_invoke()); a marked invoke reuses the immediately-prior invoke's BURV root instead of minting a fresh one, so bud promotes the predecessor's current/ into the marked invoke's previous/ — restoring the operator's shared ../output-buk depth-1 chain for exactly the invokes that need it, leaving all other isolation intact (the cinch). Fixture (rbtdro_onboarding.rs): rbtdro_ensconce now returns the fresh touchmark, read off current/ before the chained ordain promotes it away; the airgap-chain case marks ordain-forge to chain off the ensconce, then asserts the forge RBRV_IMAGE_1_ANCHOR became rbi_ld/<touchmark>:rbi_bole — a non-firing election leaves the older committed stamp and fails the case. Added a symmetric rbtdro_read_vessel_env reader for the witness. Guards (rbtdti_invocation.rs): chained invoke reuses the prior root without advancing the counter and isolation resumes after; chain_next with no prior invoke errors loud. Mechanism was mount's call per the docket: chose the context one-shot flag over a threaded bool (would churn ~40 call sites) or twin _chained helpers (parallel infrastructure the heat rejects). Build + 141 theurge unit tests green; live onboarding fixture not yet run.

### 2026-06-09 05:59 - ₢BHAAT - W

Evicted skopeo from the bole capture path onto gcrane, verified green against live GAR. rbgjl01: skopeo inspect/copy/retag -> gcrane manifest/cp/tag with ambient google.Keychain GAR auth (token-fetch include dropped); shared fingerprint snippet rebased on `gcrane manifest` and renamed skopeo->gcrane-fingerprint; bole step-01 builder -> Google-hosted gcr.io/go-containerregistry/gcrane:debug via new ZRBLD_GCRANE_BUILDER + busybox spine entrypoint (#!/busybox/sh). Builder is a FLOATING name like its cloud-builders/docker sibling, NOT a bash-frozen digest -- corrected the docket's pinned-by-digest cinch (operator-blessed): version-freezing belongs to the reliquary gather. Auth canon superseded in RBSCB (+ digest-pin posture softened, cred-helper rejection scoped to remaining made-side skopeo) and RBSLE; rename-forced reference fixes in CBG/RBSCJ/rbk-acronyms. lode-lifecycle + lode-collision fixtures both green against live GAR: ambient auth (no token), busybox+pipefail, and :rbi_sha256 digest fidelity all confirmed empirically. Required clearing the §6 credential collapse first (re-mantled governor, re-keyed director canest-dir). Mid-pace: caught and fixed a self-inflicted miss -- the busybox case arm was documented in the spine comment but omitted from the actual switch (fixed, fixture caught it).

### 2026-06-09 05:51 - ₢BHAAT - n

Fix: add the missing busybox case arm to the spine's entrypoint->shebang switch (zrbld_spine_dispatch). The prior notch updated the doc comment to list busybox but omitted the actual case branch, so dispatch buc_die'd 'Unknown entrypoint busybox' on the bole gcrane recipe row. Now busybox -> #!/busybox/sh, matching the gcrane:debug builder's only shell. Caught by the lode-lifecycle fixture.

### 2026-06-09 05:39 - ₢BHAAT - n

Evict skopeo from the bole capture path onto gcrane. rbgjl01: skopeo inspect/copy/retag -> gcrane manifest/cp/tag, ambient google.Keychain GAR auth (Mason SA via metadata server), token-fetch include dropped. Shared fingerprint snippet rebased on `gcrane manifest` (bytes verbatim, digest byte-identical) and renamed skopeo->gcrane-fingerprint. bole step-01 builder swapped to the Google-hosted gcr.io/go-containerregistry/gcrane:debug via new ZRBLD_GCRANE_BUILDER constant + a busybox spine entrypoint (#!/busybox/sh, the only shell in distroless :debug). NOTE the builder is a FLOATING name like its cloud-builders/docker sibling, NOT a bash-frozen digest -- this corrects the docket's 'pinned by digest' cinch (operator-blessed): version-freezing belongs to the reliquary gather, not a scattered bash digest. Auth canon updated in RBSCB (supersedes the skopeo token-fetch rationale; softens the overstated digest-pin posture line; scopes the cred-helper rejection to remaining made-side skopeo) and RBSLE (capture verbs + ambient auth). Rename-forced reference fixes in CBG (snippet name x3 + busybox entrypoint), RBSCJ (snippet contract table + caller accounting), and the rbk acronym doc. Shellcheck clean (206 files); live-GAR lode-lifecycle fixture pending.

### 2026-06-08 20:44 - Heat - d

paddock curried: curry crane-embrace auth bullet: resolved to gcrane ambient Google keychain

### 2026-06-08 20:40 - Heat - n

Record the cloud-side capture auth decision in the cerebro memo: use gcrane (not plain crane) from gcr.io/go-containerregistry/gcrane:debug for the bole/crane-embrace capture steps. gcrane authenticates GAR ambiently via google.Keychain (matches *.pkg.dev, draws ADC->GCE metadata-server creds as the Mason SA), so the in-memory token-fetch dance is dropped entirely — no crane auth login, no credential-helper image. Corrects stale section-7.1 builder line (was 'crane or cloud-builders image carrying crane'), adds an auth bullet to the section-1 bottom line, and a new section-9 evidence record (crane-vs-gcrane binary distinction, keychain.go host-matching + metadata-server credential source, Cloud SDK image does not bundle crane, conclave ambient-auth symmetry, RBSCB as the durable-canon home). Resolves the auth axis the bole-eviction pace cinched as decided-here-inherited-downstream.

### 2026-06-08 20:24 - ₢BHAAK - W

Reviewed the podvm cerebro-experiment conclusion and settled the kit's capture-tool posture: FULL CRANE EMBRACE. crane becomes the sole image/registry tool; skopeo/docker/buildx are evicted, leaving the irreducible non-overlapping floor crane + curl (HTTPS fetch) + gpg (signature verify) — all daemonless static binaries, removing the docker daemon from capture. oras is documented equal-fidelity fallback only (reopens only if signed provenance goes via OCI referrers); skopeo's rule-out rationale corrected (loud fatal on empty-config OCI artifacts, NOT the silent foreign-layer skip first assumed). Decided skopeo eviction is TOTAL including the reliquary cohort, by converting the made-side bind mirror (rbfd_mirror/rbgjm01 — a bare skopeo copy --all) to crane cp: a narrow, deliberate exception to the capture-only boundary, not the made-image package retrofit. Crane-vs-oras and crane-vs-skopeo trade-offs worked through (fidelity proven byte-identical in the experiment; crane wins on go-containerregistry/GAR vendor-fit + one-tool-for-all-shapes; auth is the one sharp edge — no per-command creds, login or helper, but same Mason SA + short-lived token, no new IAM grant). Recorded the crane-embrace cinch in the paddock and reconciled four stale skopeo/oras references. Slated the eviction-by-tool paces (BHAAT skopeo/bole, BHAAU docker/conclave, BHAAV buildx/vouch+underpin), podvm base immure machinery (BHAAW) + two-family fan-out (BHAAL), and the terminal skopeo-reliquary-eviction (BHAAX). Repaired pre-existing paces for the new posture: railed the airgap bole-election guard (BHAAS) beside the bole work, restored the vocabulary scrub (BHAAD) to terminal and augmented it with a crane-embrace tool cross-check + crane/curl/gpg KEEP, dropped a stale 'fetter' verb. RBSPV promotion + RBSL/RBS0 podvm spec landing homed in BHAAW. Credential hygiene (payor reauth + governor remantle) left to the operator.

### 2026-06-08 20:20 - Heat - r

moved BHAAS after BHAAT

### 2026-06-08 20:15 - Heat - d

paddock curried: skopeo eviction now total incl cohort via mirror->crane exception

### 2026-06-08 20:11 - Heat - S

lode-skopeo-reliquary-eviction

### 2026-06-08 19:57 - Heat - d

paddock curried: crane-embrace cinch + skopeo/oras reference reconciliation

### 2026-06-08 19:51 - Heat - S

lode-podvm-immure

### 2026-06-08 19:50 - Heat - S

lode-buildx-eviction

### 2026-06-08 19:50 - Heat - S

lode-docker-eviction

### 2026-06-08 19:37 - Heat - S

lode-skopeo-eviction

### 2026-06-08 21:57 - ₢BHAAK - n

Record podvm cerebro-experiment findings + cloud-layer decision. Memo (memo-20260608-lode-podvm-cerebro-experiment.md): full empirical record — both quay families (machine-os vn / machine-os-wsl vw) characterized at podman 5.6; disk leaves are OCI artifacts (oci.empty.v1+json config + single application/zstd blob, distributable, no foreign markers); native index mixes real container images with disk artifacts; families diverge in version coverage + tag scheme (native dropped per-arch tags at 5.4). Tool verdict: crane cp AND oras cp both digest-faithful; skopeo cooked ops FATAL on empty-config artifact (loud, NOT the silent-foreign-skip the cinch assumed — blobs are distributable). Decision: crane primary (go-containerregistry, get-by-digest-or-error matches recorded grade, uniform over artifacts+images), oras equal-fidelity fallback, skopeo out. Both families captured into rbi_ld with rbld-vouch-1 provenance, proven controllable via host-side divine+banish, anti-hollow-mirror guard green via GAR registry-v2 blob HEAD (REST, no image tool). Follow-up cloud pace build sheet: rbgjl05 immure step + rbldv_ opaque-blob*multi-member body + podvm-lifecycle theurge fixture, riding the spine. RBSPV gains a 2026-06 cerebro-characterization section recording the same findings and superseding the ignite-VM prototype with cloud-side crane. Experiment evidence banished, cerebro torn down to as-found.

### 2026-06-08 14:49 - Heat - n

Bank the additive Lode/Touchmark public concept surface off the single-threaded BH chain: add `### Lode` and `### Touchmark` concept entries to the Foundry section (parallel to Vessel/Hallmark, concept-level, no per-kind verbs), relocate the canonical `<a id="Lode">` anchor from the churn appendix to the Lode entry, and reseam the churn appendix to a plain `[Lode](#Lode)` reference (one anchor, no duplicate-id breakage). Defers only the Enshrine/Reliquary narrative conversion, which is gated on the reliquary->conclave cutover and reads cleanly only as one coherent rewrite once tools are also Lodes.

### 2026-06-08 14:27 - Heat - n

Add README appendix 'Registry Churn and Disappearing Upstream Images' — a vendor-grievance section paralleling Eventual Consistency, registering Quay's rapid podman machine-os churn (new images every few hours, retention in days, no durable re-checkable reference) as the failure that motivates project capture. Introduces the public Lode concept anchor at concept grade — general project-owned capture of an upstream artifact (base image, build tool, OS substrate, VM disk image) into the Depot registry with provenance — and the two honest trust grades (verified-against-published vs recorded-at-acquisition). Concept-level only, no per-kind verbs, so it sits ahead of the deferred ₢BHAAO glossary pass without describing two parallel capture systems.

### 2026-06-08 13:48 - Heat - T

lode-podvm-cerebro-experiment

### 2026-06-08 13:06 - ₢BHAAJ - W

Landed the wsl (underpin) capture kind — the structural-outlier Lode vertical. rbld_underpin (rbldw_Underpin.sh) takes two declarative version args (release point), assembles the cdimage Ubuntu-base rootfs URL from a path-convention template (no FQIN, no pinned digest), and rides rblds_Spine via the new rbgjl04 cloud step: curl fetch + GPG-verified published checksum (vendor SHA256SUMS verified against a pinned CD-Image signing-key fingerprint via the reusable rbgjs-gpg-verify-sums snippet, cloud-side never on the workstation) + opaque-blob OCI wrap (FROM scratch + COPY, never ADD), reusing rbgjl02 vouch push as-is. Both steps ride the Google-hosted docker builder. RBGC adds wsl brand + rbi_rootfs member tag + URL-template/arch-default/signing-fingerprint constants; divine gains the w->wsl legend; rbw-lU colophon (zipper param1 release+point, tabtarget); RBSLU-lode_underpin.adoc on the RBSLC skeleton + rbtgo_lode_underpin quoin in RBS0; wsl-lifecycle theurge fixture in service+complete suites. Static gate green: shellcheck 206 files clean, theurge build under deny(warnings), 137 unit tests, fast qualify. Live wsl-lifecycle PASSED against live GAR (build 3e0309bc SUCCESS in 21s): captured Lode rbi_ld/w260608130305 with rbi_rootfs+rbi_vouch member tags, divine enumerate+inspect -> banish -> absent confirmed registry restored. Design evolved in-conversation with operator: an initial pinned URL+SHA256 was reverted to declarative version args + cloud-discovered checksum (no-FQIN premise), and GPG signature verification was added cloud-side against a pinned fingerprint (verified-against-published, strongest form). Consumption (wsl --import) and nameplate substrate election remain deferred per the heat. Two commits: aa9fb6de (vertical) + 12f48cba (single-fixture-registry fix).

### 2026-06-08 13:03 - ₢BHAAJ - n

Register wsl-lifecycle in the RBTDRC_FIXTURES single-fixture lookup registry (had wired the service+complete suite arrays but not the per-name registry rbtdrc_lookup_fixture searches, so rbw-tf wsl-lifecycle reported no bound Fixture). One-line registry insert alongside the suite membership.

### 2026-06-08 13:01 - ₢BHAAJ - n

Land the wsl (underpin) capture kind on the Lode spine — the structural-outlier vertical. New rbld_underpin verb (rbldw_Underpin.sh) takes two declarative version args (release point), assembles the cdimage Ubuntu-base rootfs URL from a path-convention template (no FQIN, no pinned digest — intent stays declarative), and rides rblds_Spine via the new rbgjl04 cloud step: curl fetch + GPG-verified published checksum (vendor SHA256SUMS verified against the pinned CD-Image signing-key fingerprint via the reusable rbgjs-gpg-verify-sums snippet — verification happens cloud-side, never on the workstation) + opaque-blob OCI wrap (FROM scratch + COPY, never ADD), reusing rbgjl02 vouch push as-is. Both steps ride the Google-hosted docker builder (curl/gpg/buildx) — no skopeo, no reliquary bootstrap. RBGC adds wsl brand + rbi_rootfs member tag + URL-template/arch-default/signing-fingerprint constants; divine gains the w->wsl legend line; rbw-lU colophon (zipper enrollment param1 release+point, tabtarget). RBSLU-lode_underpin.adoc on the RBSLC skeleton + rbtgo_lode_underpin quoin/section in RBS0. New wsl-lifecycle theurge service fixture (underpin -> divine enumerate+inspect rbi_rootfs/rbi_vouch -> banish -> absent) in service+complete suites. Static gate green: shellcheck 206 files clean, theurge build under deny(warnings), 137 unit tests, fast qualify (tabtarget/colophon/generated-freshness). Live service-tier fixture run pending. BCG/CBG/RCG honored.

### 2026-06-08 12:15 - ₢BHAAI - W

Landed the reliquary (conclave) capture kind on the Lode spine. rbld_conclave (rbldr_Reliquary.sh) rides rblds_Spine to capture the build-tool cohort into one rbi_ld/<stamp> package as N :rbi_<tool> member tags + the :rbi_vouch envelope, via the new rbgjl03 docker cohort-capture step (reuses rbgjl02 vouch push as-is; both ride the Google-hosted cloud-builders/docker builder — no reliquary bootstrap, since conclave captures the reliquary tools themselves). Absorbs inscribe's rbgji01 pull manifest, retargeted from the rbi_rq/<date>/<tool> sibling-package layout to one package; inscribe left live for the separate cutover. Added rbw-lC colophon (zipper enrollment, tabtarget, divine kind-legend), RBGC reliquary brand + tag sprue, and dropped the dead non-load-bearing tool kind. RBSLC-lode_conclave.adoc on the RBSLE skeleton + rbtgo_lode_conclave quoin/section in RBS0. New reliquary-lifecycle theurge service fixture (conclave -> divine enumerate+inspect members -> banish -> absent) in service+complete suites. Static gate green: shellcheck 203 files clean, theurge build under deny(warnings), 137 unit tests, fast qualify. Live service-tier fixture PASSED against live GAR (~180s) — confirms the full chain including rbgjl02 buildx vouch-push on the un-mirrored Google docker builder. BCG/CBG/RCG honored. Implementation notched at 43d16304ec.

### 2026-06-08 12:09 - ₢BHAAI - n

Land the reliquary (conclave) capture kind on the Lode spine. New rbld_conclave verb (rbldr_Reliquary.sh) rides rblds_Spine: captures the build-tool cohort into one rbi_ld/<stamp> package as N :rbi_<tool> member tags + the :rbi_vouch envelope, via the new rbgjl03 docker cohort-capture cloud step (reuses rbgjl02 vouch push as-is). Both steps ride the Google-hosted cloud-builders/docker builder — no reliquary bootstrap, since conclave is what captures the reliquary tools. Absorbs inscribe's pull machinery (rbgji01 manifest) retargeted from the rbi_rq/<date>/<tool> sibling layout to one package; inscribe left live for the separate cutover. Adds rbw-lC colophon (zipper enrollment, tabtarget, divine kind-legend line), RBGC reliquary brand + tag sprue, and drops the dead non-load-bearing tool kind. RBSLC-lode_conclave.adoc written on the RBSLE skeleton + rbtgo_lode_conclave quoin/section in RBS0. New reliquary-lifecycle theurge service fixture (conclave -> divine enumerate+inspect members -> banish -> absent) wired into service + complete suites. Static gate green: shellcheck 203 files clean, theurge build under deny(warnings), 137 unit tests pass, fast qualify (tabtarget/colophon/generated-freshness) pass. Live service-tier fixture run pending. BCG/CBG/RCG honored.

### 2026-06-08 11:36 - ₢BHAAI - n

Make divine's enumeration IMAGE column scheme-aware so clean-scheme cohort Lodes read legibly. A single-image Lode (bole) still shows its unsprued fingerprint tag; a Lode with no digest/fingerprint layer (the locked reliquary member scheme: :rbi_<tool> per member + :rbi_vouch) now reports '(cohort: N members)' instead of the misleading '(no fingerprint)'. The jq branches on fingerprint-presence — kind-agnostic, so no premature reliquary wiring (legend entry lands with the kind itself) — and counts every non-:rbi_vouch tag via the readonly RBGC_LODE_TAG_VOUCH constant. Behavior-preserving for bole (fingerprint present -> identical output). Prep for the conclave/reliquary build (BHAAI). shellcheck 201 clean; jq logic verified against synthetic bole- and cohort-shaped tag responses.

### 2026-06-08 11:29 - Heat - T

lode-reliquary-capture

### 2026-06-08 11:28 - Heat - d

paddock curried: drop non-load-bearing tool kind; six->five kinds, five->four capture verbs

### 2026-06-08 10:56 - Heat - S

theurge-fixture-fact-chain-fix

### 2026-06-08 10:56 - ₢BHAAH - W

Live verification of the bole enshrine->ensconce cutover. The derived-pull election is proven on the real operator fact-chain path: back-to-back ensconce->ordain on rbev-bottle-ifrit-forge logged 'Elected base ANCHOR slot 1: rbi_ld/b260608102223:rbi_bole', the forge conjure preflight-verified that bole Lode and built+vouched green from it; the elected anchor (rbi_es->rbi_ld) was committed (ca74d3328). Gates: fast 146/146; lode-lifecycle + lode-collision 2/2 against live GAR (validates bare single-form touchmark-fact emission + the fixture read-from-fact cutover). Discovered limitation: the airgap-chain fixture is a false-green -- theurge's per-invoke burv_output isolation severs the depth-1 fact chain, so the election never fires there and the conjure builds from the lingering rbi_es base; the automated verify gate is non-functional. Verify gate met substantively via the forge conjure (direct, observable election with logged anchor value + preflight verification), not via the fixture. Follow-up pace slated to make the fixture genuinely exercise the election.

### 2026-06-08 10:38 - ₢BHAAH - n

Elect bole touchmark into forge vessel base anchor: RBRV_IMAGE_1_ANCHOR repointed rbi_es/rust-slim-bookworm-b5f842fac1 -> rbi_ld/b260608102223:rbi_bole. This is the operator-commit of the derived-pull election, proven live via back-to-back ensconce->ordain on rbev-bottle-ifrit-forge (election logged 'Electing conjure base ANCHOR from bole capture' / 'Elected base ANCHOR slot 1'); conjure then built and vouched green FROM the rbi_ld base (hallmark c260608102417-r260608172422). Confirms the cutover end-to-end on the real fact-chain path that the airgap-chain fixture cannot exercise (theurge per-invoke output isolation severs the depth-1 chain).

### 2026-06-08 09:35 - ₢BHAAH - n

Bole enshrine->ensconce cutover. Ensconce emits two bare single-form chaining facts (RBF_FACT_LODE_TOUCHMARK + RBF_FACT_LODE_BRAND/RBGC_LODE_BRAND_BOLE) and drops the <stamp>.lode envelope (provenance lives only in GAR :rbi_vouch); RBSLE + the RBS0 rbf_fact_lode quoin reslated (RBS0 swept into a concurrent BZ commit 0a3fc6748, content correct). New zrbfd_elect_base_anchor derived-pull persists RBRV_IMAGE_n_ANCHOR=rbi_ld/<touchmark>:rbi_bole inside rbfd_ordain's conjure branch (folded, no new verb), with operator-commit hint; conjure tolerates the mid-ordain dirty tree (no clean-tree gate on the build, verified). Enshrine writer fully retired: rbfd_enshrine/_submit/_extract_anchors, rbgje01 step, rbw-dE tabtarget+dispatch, writer-only kindle constants; registry preflight de-enshrined + repointed to rbw-lE; _RBGR_ENSHRINE_LOCATOR -> _RBGR_BASE_LOCATOR across rbfd/rbfv/rbgjr/RBSAC. Theurge lode fixtures read the new touchmark fact (single-form, not .lode roots); onboarding airgap-chain fixture + rbhoda track flipped to ensconce (commit moved after ordain-forge since election now writes ANCHOR mid-ordain); generated consts regenerated. rbi_es banished: RBGC_GAR_CATEGORY_ENSHRINES, RBGL_ENSHRINES_ROOT, rbfl_audit_enshrinements + rbw-iae enrollment/tabtarget (generic wrest/jettison-enshrinement verbs survive per paddock, rbfk_kludge depends on wrest). Deferred to owning paces: RBSAE spec + rbtgo_ark_enshrine quoin (BHAAD), README Enshrine concept + RBYC_ENSHRINE (BHAAO/BHAAD), clean-tree/commit onboarding teaching (BHAAR -- both upstream deps resolved: gate=downstream-only/rbob_charge model, election=derived-pull not a separate step), vocab stragglers incl ZRBFC_BUILD_POLL_CEILING_ENSHRINE survivor (BHAAD). Static-verified: shellcheck 201, theurge build + 137 unit tests. Live gates pending (service tier): one airgap conjure green (verify gate), lode-lifecycle + lode-collision against live GAR.

### 2026-06-08 08:01 - ₢BHAAQ - W

BUK fact-chaining substrate + uniform clean-tree gate (trimmed spine). Stage 1: depth-1 previous/ output dir (BURD_PREVIOUS_DIR promoted from current/ at the dispatch clear seam via mv, exit-status-independent), the consume primitives buf_relay (no-clobber baton forward) + buf_read_fact (fail-hard single read), BURD enrollment, a fact-chaining self-test fixture (5 cases), and BUS0 treatment. Gate: new bug BUK module (bug_require_clean_tree, guard ZBUG_SOURCED) homing the tools-never-commit/gate-on-clean-tree convention; the two byte-identical git diff --quiet gates in rbfk kludge + rbfd mirror collapsed onto it; all three gateways sourcing it (rbfk_cli/rbfd_cli/rbob_cli — rbob_cli initially missed, caught by siege as command-not-found on the kludge path and fixed); convention + BUG acronym documented in claude-buk-core.md. rbob's path-scoped charge gate left separate by design. Verified green: shellcheck 202, BUK self-test 41, fast 146, siege (kludge-tadmor + tadmor security). Scope trimmed mid-pace: the per-kind consumer cutovers (bole ANCHOR->fact, reliquary RBRV_RELIQUARY/yoke) and the ensconce single-form emission / <stamp>.lode removal / RBSLE reslate moved to their dedicated cutover paces (bole-enshrine ₢BHAAH, reliquary-inscribe ₢BHAAM); the rbob nameplate drive is a sanctioned same-dispatch cycle needing no migration.

### 2026-06-08 07:51 - ₢BHAAQ - n

Source bug_git.sh in rbob_cli furnish. rbob_cli is the third gateway into rbfk_kludge/rbfd (rbob_kludge/_kludge_sentry/_ordain delegate to them), so it must source the bug module like rbfk_cli and rbfd_cli do — without it, bug_require_clean_tree was undefined on the kludge path and siege's kludge-tadmor failed with command-not-found at rbfk_kludge.sh:92. Caught by siege (fast does not exercise the rbob kludge path).

### 2026-06-08 07:36 - ₢BHAAQ - n

Uniform clean-tree gate via new BUG module. New Tools/buk/bug_git.sh holds bug_require_clean_tree (the tools-never-commit/gate-on-clean-tree convention, guard ZBUG_SOURCED); rbfk and rbfd CLIs source it; the two byte-identical whole-tree git diff --quiet gates in rbfk_kludge (kludge) and rbfd mirror collapse to bug_require_clean_tree calls. rbob's path-scoped charge gate left separate by design. Convention + BUG acronym documented in claude-buk-core.md. Bash-only; behavior-preserving.

### 2026-06-06 12:23 - ₢BHAAQ - n

Stage 1 of fact-chaining: depth-1 previous/ output dir and the consume primitives. bud_dispatch promotes prior current/ to previous/ (BURD_PREVIOUS_DIR) at the dispatch-start clear seam via mv, exit-status-independent; buf_fact gains buf_relay (no-clobber baton forward, previous->current) and buf_read_fact (fail-hard single-fact read from previous); BURD_PREVIOUS_DIR enrolled in the BURD regime (readonly-locked); new fact-chaining self-test fixture (5 cases, 36->41); BUS0 treatment for the var, both primitives, and a cross-tabtarget chaining note. No consumer changes.

### 2026-06-06 11:58 - Heat - S

onboarding-clean-tree-gate

### 2026-06-06 11:56 - Heat - d

paddock curried: provenance envelope: one canonical home (GAR :rbi_vouch); host-side now bare single-form chaining facts

### 2026-06-06 11:21 - Heat - S

buk-fact-chaining-prev-and-git-gate

### 2026-06-06 09:59 - Heat - n

Install gradient-delivery discipline at JJK mount/groom (stun and molehill antipattern names) in the veiled context; cull duplicated commit-discipline guidance and the redundant gazette worked-examples from the core context; reconcile Mount step-6 with the gradient rule; freshen the model-id example; and sharpen the wrap-is-unscoped known-bug reminder.

### 2026-06-05 18:40 - ₢BHAAG - W

Promoted the reserved Lode kind-letter set to readonly constants in zrbgc_kindle — RBGC_LODE_KIND_{TOOL=t,RELIQUARY=r,WSL=w,PODVM_WSL=vw,PODVM_NATIVE=vn} — mirroring the existing _BOLE; retired the stale 'only bole is implemented this pace' comment qualifier. Scope held to constants only; shellcheck green; BCG kindle-constant discipline verified.

### 2026-06-05 18:39 - ₢BHAAG - n

Promote the reserved Lode kind-letter set from comment to readonly constants in zrbgc_kindle: RBGC_LODE_KIND_{TOOL=t,RELIQUARY=r,WSL=w,PODVM_WSL=vw,PODVM_NATIVE=vn}, mirroring the existing _BOLE. Retire the stale 'only bole is implemented this pace' qualifier. Scope held to the constants only — no capture colophons, no divine-legend rows. Shellcheck green.

### 2026-06-05 11:36 - ₢BHAAB - W

Replan checkpoint complete: the Lode remainder is slated as 10 concrete paces (₢BHAAG-₢BHAAP) across the three classes, all before the terminal scrub ₢BHAAD. Order: scaffold kind-letters (G) -> bole/enshrine cutover early, severed from reliquary (H) -> three file-disjoint parallel-chat verticals tool+reliquary/wsl/podvm-machinery (I/J/K) -> podvm platform fan-out (L) -> reliquary/inscribe cutover gated on conclave (M) -> augur split incl. envelope decode (N) -> concept-level public-docs riding the cutovers (O) -> housekeeping deferrals (P). Recommendations were ground-truthed via an ultracode workflow (five investigate+verify subagent pairs against landed code); key verified findings drove the cuts: bole cutover is independent of all verticals (its only precursor, ensconce, is landed) so it goes early; tool+reliquary share inscribe's docker-pull machinery so they group; wsl is the HTTPS+checksum outlier; podvm splits capture-vs-fan-out; Lode is absent from public README so verticals don't collide there and docs stay concept-level. Referrers/cosign provenance signing deliberately NOT slated — added as an RBSHR horizon note gated on external distribution. Process documented in Memos/memo-20260605-ultracode-replan-process.md (itself refined by a review panel; honest scorecard: verification bought confidence, reversed zero recommendations, flipped one over-confident fact). Two operator defaults applied: augur is one pace; scaffold authors capture colophons per-vertical.

### 2026-06-05 11:35 - ₢BHAAB - n

Educational memo on the ultracode process behind the ₢BHAAB replan (five investigate+verify subagent pairs ground-truthing four planning calls against landed code, then manual synthesis into the 10-pace slate). Memo was itself refined by an ultracode review panel that caught two real over-claims — corrected to an honest scorecard (skeptics reversed zero recommendations, flipped one over-confident fact) and an accurate false-positive mechanism (a prompt asymmetry I authored, not a verifier failure; the review panel then repeated the same paddock-context gap, making the lesson recurse). Also add the RBSHR horizon note (accepted in lieu of a pace): confirm GAR OCI-referrers maturity and decide cosign-vs-referrers for Lode :rbi_vouch signing, gated on the same external-distribution trigger as Cosign.

### 2026-06-05 11:21 - Heat - S

lode-housekeeping-deferrals

### 2026-06-05 11:21 - Heat - S

lode-public-docs-concept

### 2026-06-05 11:20 - Heat - S

lode-augur-inspect-split

### 2026-06-05 11:20 - Heat - S

lode-reliquary-inscribe-cutover

### 2026-06-05 11:19 - Heat - S

lode-podvm-platform-fanout

### 2026-06-05 11:19 - Heat - S

lode-podvm-capture-machinery

### 2026-06-05 11:19 - Heat - S

lode-wsl-capture

### 2026-06-05 11:18 - Heat - S

lode-tool-reliquary-capture

### 2026-06-05 11:17 - Heat - S

lode-bole-enshrine-cutover

### 2026-06-05 11:17 - Heat - S

lode-scaffold-kind-letters

### 2026-06-05 10:53 - ₢BHAAC - W

Collision-guard test landed and verified green against live GAR. Guard code+spec were already committed (a9a97c93); this pace built the missing test. Stamp-pin tweak in rbldb_Bole.sh: rbld_ensconce honors BURE_TWEAK_NAME=buorb_ensconce_stamp to pin the Lode stamp (mirrors the rbfd graft-tweak precedent z_graft_tweak_name) — without a pin the seconds-grained mint never reuses a touchmark, so the guard's branches can't fire from the CLI. New rbtdrc_lode_collision case in the lode-lifecycle fixture (+ RBTDRC_ENSCONCE_STAMP_TWEAK_NAME rust mirror + rbev-sentry-deb-tether vessel const) drives all three branches on one pinned touchmark: fresh busybox -> SUCCESS; busybox pinned to S (identical digest) -> idempotent branch SUCCESS (positive control); debian pinned to S (different base, same touchmark) -> cloud build FAILURE, host exit non-zero (collision branch); banish + divine confirm cleanup. Collision verdict rests on host exit code since the guard message is CLOUD_LOGGING_ONLY (build FAILURE propagates via rbfcb status!=SUCCESS -> buc_die). Verified: theurge build, 137 unit tests, shellcheck 200 files, and live-GAR fixture 2 passed/0 failed (touchmark b260605103342). Live run was initially blocked by a stale Director SA credential (Invalid JWT Signature) — operator re-mantled Governor + re-invested director-bobbie; rbw-acd green, then the fixture passed. Follow-ups noted, not actioned: (1) token-mint retry mislabels persistent Invalid JWT Signature as an SA propagation race, burning 104s; (2) lode-lifecycle is now 2 cases — verify no rbw-ts suite hard-asserts a case count.

### 2026-06-05 10:21 - ₢BHAAC - n

Drive the ensconce collision-guard test (clean-tree gate forces commit before the live-GAR run). Stamp-pin tweak in rbldb_Bole.sh: rbld_ensconce now honors BURE_TWEAK_NAME=buorb_ensconce_stamp to pin the Lode stamp (BURE_TWEAK_VALUE), mirroring the rbfd graft-tweak precedent (z_graft_tweak_name) — without a pin the seconds-grained mint puts every CLI ensconce on a distinct touchmark, so the cloud guard's branches never fire. New lode-collision case in the lode-lifecycle fixture (rbtdrc_crucible.rs, + RBTDRC_ENSCONCE_STAMP_TWEAK_NAME rust mirror + rbev-sentry-deb-tether vessel const): ensconce busybox naturally -> read back touchmark S; ensconce busybox pinned to S (identical digest -> idempotent branch, exit 0, positive control); ensconce debian pinned to S (different base, same touchmark -> collision branch, host exit non-zero); banish S. Collision verdict rests on host exit code (guard message is CLOUD_LOGGING_ONLY; build FAILURE propagates via rbfcb status!=SUCCESS -> buc_die). Static checks green: theurge build, 137 unit tests, shellcheck 200 files. Live-GAR fixture run is the immediate next step.

### 2026-06-05 10:05 - ₢BHAAF - W

Established the buo tweak sprue for BURE_TWEAK_NAME. zbure_enforce requires a non-empty tweak name to match buo<segment>_ — BUK validates the SHAPE only (generic, no consumer-name coupling), so an unregistered/typo'd tweak fails loud instead of silently no-op'ing; the _-bearing sprue is a grep-registry (grep buo), no central list. Transformed every existing tweak: threemodegraft -> buorb_graft_image (rbfd consumer via shared const z_graft_tweak_name + rbtdro rust mirror); self-test + harness placeholders -> buost_example (reserved buost_ segment for BUK/test-stubs). Documented in CLAUDE.md mint Extended Namespace Checklist + BUS0 tweak section. Verified green: shellcheck 200 files, BUK self-test 9 bure-tweak cases, theurge 137 unit tests.

### 2026-06-05 10:02 - ₢BHAAF - n

Establish the buo tweak sprue for BURE_TWEAK_NAME. zbure_enforce now requires a non-empty tweak name to match buo<segment>_ — BUK validates the SHAPE only (generic, no consumer-name coupling), so a typo'd/unregistered tweak fails loud instead of silently no-op'ing; the _-bearing sprue is a grep-registry (no central list). Transformed every existing tweak: threemodegraft -> buorb_graft_image (rbfd consumer via named const z_graft_tweak_name, rbtdro rust mirror const); self-test + harness placeholders -> buost_example (reserved BUK/test-stub segment). Documented in CLAUDE.md mint Extended Namespace Checklist + BUS0 tweak section. Verified green: shellcheck 200 files, BUK self-test 9 bure-tweak cases, theurge 137 unit tests (incl. rbtdti sprued-tweak passthrough).

### 2026-06-05 09:50 - Heat - S

buo-tweak-sprue

### 2026-06-05 09:50 - ₢BHAAC - n

Ensconce collision guard (code+spec; live-GAR test deferred): digest-aware bole-handle inspect before the cloud capture copy in rbgjl01 — fresh proceeds, identical-digest retry proceeds idempotently, a different digest under the same touchmark fails loud cloud-side. RBSLE states the contract (require+fatal). Lints clean. Collision test deferred behind the buo-tweak-sprue precursor (needs a sprued stamp-pin tweak to drive two captures onto one touchmark).

### 2026-06-05 09:08 - Heat - r

moved ₢BHAAC before ₢BHAAB

### 2026-06-05 09:05 - Heat - n

groom: correct stale rbldk_ -> rbld0_Lode sourcing attribution in three Lode guard-free cluster headers (BX residue; gestalt entry landed as rbld0_Lode.sh, not the planned rbldk_)

### 2026-06-05 08:55 - Heat - d

paddock curried: groom: discharge ₣BX cross-heat gate (landed), pin chokepoint + rbld family

### 2026-06-04 10:31 - Heat - d

paddock curried: add Cross-heat dependency section: BX builds the capture spine + decomposition; BH verticals ride it as file-disjoint body files; sequencing gate (mount BH remainder only after BX wraps)

### 2026-06-04 10:12 - Heat - d

paddock curried: Heat nature: bole RBSL cluster + shared RBS0 Lode quoins now landed; narrow the spec-writing follow-up to the per-kind capture subdocs

### 2026-06-04 10:05 - ₢BHAAE - W

Authored the four bole-cluster Lode operation specs in the house //axhob_operation DSL, faithful to the landed bole code: RBSLE ensconce (canonical capture skeleton — two-step TETHER build as Director/Mason, host-minted touchmark, member tags, rbld-vouch-1 envelope), RBSLD divine (kind-agnostic enumerate), RBSLA augur (kind-agnostic single-Lode inspect, minted here as the separate inspect verb, parallel of ark_inspect/plumb), RBSLB banish (atomic whole-Lode packages-delete). Allocated shared Lode quoins in RBS0: rbtga_lode / rbtga_touchmark / rbtga_lode_vouch concepts (new Lode Definitions section), gar_lodes_namespace (rbi_ld), rbf_fact_lode capture-file, rbst_touchmark type, rbtgog_lode group, four rbtgo_lode_* operations with include blocks. Recast specs and paddock to the future form — no transitional 'divine currently does inspect' framing. RBSAE untouched; all quoin cross-refs resolve. rbld_augur is a deliberate forward-reference (verb-extraction is a later pace).

### 2026-06-04 10:03 - Heat - n

Acronyms-registry refinement (out-of-band): RBFCK and new RBFLK become kindle-entry modules that re-home the CLI dispatch (rbfck_cli/zrbfck_furnish, rbflk_cli/zrbflk_furnish), retiring bare rbfc_cli.sh / rbfl_cli.sh so the container prefixes rbfc/rbfl name nothing once they have children (terminal-exclusivity). Heat-scenery; committed at operator request.

### 2026-06-04 09:54 - Heat - n

Acronyms-registry allocation note (authored out-of-band by linter/operator): record the RBFC Foundry-Core and RBFL Foundry-Ledger family decompositions (explosion targets) and the RBLD Lode capture family decomposition (rblds spine / rbldb bole / rbldl lifecycle / rbldk kindle, plus reserved RBLDT/R/W/V kind letters), with the RBSL spec letter reserved. Heat-scenery accompanying the Lode spec work; committed at operator request.

### 2026-06-04 09:46 - ₢BHAAE - n

Recast the Lode specs to the future form only — no transitional/current-state framing. RBSLA augur described purely as the single-Lode inspect verb (parallel of ark_inspect/plumb), dropping the 'inspect currently lives in divine / until the split lands' note. RBS0 augur block reworded from 'split out from divine' to inspect-counterpart; drop 'only kind landed' / 'reserved set; only bole implemented' / 'currently b bole' hedges in the rbtga_lode, rbtga_touchmark, rbst_touchmark, and divine-legend prose so the specs read as the target design.

### 2026-06-04 09:43 - Heat - d

paddock curried: future-form augur: add augur as the inspect verb (rbw-la), retire the divine-does-inspect framing

### 2026-06-04 09:37 - ₢BHAAE - n

Author RBSL* bole-cluster Lode operation specs (RBSLE ensconce canonical skeleton, RBSLD divine enumerate, RBSLA augur inspect, RBSLB banish) in the house //axhob_operation DSL, faithful to the landed bole code. Allocate Lode quoins in RBS0: rbtga_lode / rbtga_touchmark / rbtga_lode_vouch concepts (new Lode Definitions section), gar_lodes_namespace (rbi_ld), rbf_fact_lode capture-file, rbst_touchmark type, rbtgog_lode group, and four rbtgo_lode_* operations with include blocks. augur minted as separate inspect verb (spec leads code; rbld_augur not yet implemented). RBSAE untouched. All quoin cross-refs resolve.

### 2026-06-03 11:33 - Heat - r

moved BHAAE before BHAAC

### 2026-06-03 11:32 - Heat - d

paddock curried: strip heat silks from paddock prose — firemarks/coronets are lifecycle-bound, silks evolve

### 2026-06-03 11:15 - Heat - S

lode-rbsl-bole-verb-specs

### 2026-06-03 11:02 - Heat - S

lode-vocab-finalization-scrub

### 2026-06-03 03:10 - Heat - n

RBSCB: record skopeo metadata-token auth as the canonical GAR pattern (vs the evaluated-and-rejected docker-credential-gcr helper), the /workspace-never-holds-a-secret invariant, and the deliberate capture-machinery duplication with its planned spine+step-library remedy; add skopeo/credential-helper reference URLs.

### 2026-06-02 13:16 - ₢BHAAA - W

Bole-kind ergonomics finish + vocabulary rename. (1) Banish confirm-prompt fix: BURD_INTERACTIVE=1 on rbw-lB so the prompt flushes before read; live-verified (prompt-before-input, yes banishes). (2) Ensconce vouch-word polish: 'build submitted' -> 'Cloud Build submitted'. (3) Divine-enrichment live-verified: Kinds legend + TOUCHMARK/IMAGE fingerprint column render against live GAR. (4) Renamed the base kind to bole (de-genericize: 'base' collided with universal Docker jargon and our own prose; kept kind-letter 'b'). Paddock vocabulary lock + code surface (kind-letter/handle consts, cloud-substitution var, envelope kind field, divine legend, zipper, theurge consts, tabtarget frontispiece git-rename; generic 'base image' prose preserved). Fast suite 131/131 green; live-proven end-to-end via a full ensconce->divine->banish cycle (busybox), legend rendering 'bole'. Follow-on ensconce touchmark-collision guard slated as BHAAC.

### 2026-06-02 13:13 - Heat - S

lode-ensconce-collision-guard

### 2026-06-02 12:54 - ₢BHAAA - n

Rename the base kind to bole across the bole-Lode code surface (mirrors paddock vocabulary lock). Kind-letter const RBGC_LODE_KIND_BASE->_BOLE (value 'b' kept), handle tag RBGC_LODE_TAG_BASE='rbi_base'->_BOLE='rbi_bole', cloud-sub var _RBGL_TAG_BASE->_BOLE and the provenance envelope kind field 'base'->'bole' (rbgjl01), divine legend 'base'->'bole', zipper RBZ_ENSCONCE_BASE->_BOLE, theurge RBTDGC_ENSCONCE_BASE->_BOLE + RBTDRC_LODE_TAG_BASE='rbi_base'->_BOLE='rbi_bole', tabtarget frontispiece DirectorEnsconcesBase->Bole (git rename; colophon rbw-lE unchanged). Generic 'base image' prose left intact per the discrimination rule.

### 2026-06-02 11:33 - Heat - d

paddock curried: rename base kind -> bole (de-genericize the kind word; keep b letter, kept generic 'base image' prose)

### 2026-06-02 11:08 - ₢BHAAA - n

Banish confirm-prompt fix: add BURD_INTERACTIVE=1 to rbw-lB tabtarget so the prompt routes through the line-buffering-preserving dispatch branch and flushes before read (mirrors rbw-cr/ch/cs/gPI). Plus ensconce log-line polish: 'Ensconce build submitted' -> 'Ensconce Cloud Build submitted', naming the GCB proper noun.

### 2026-06-02 08:26 - ₢BHAAA - n

Divine enumerate enrichment: a Kinds legend (touchmark-prefix key) plus a TOUCHMARK/IMAGE table whose IMAGE is the unsprued fingerprint tag (<sanitized-origin>-<sha10>), located via the sha10 taken from the rbi_sha256- member tag (one tags-list per Lode). BCG-compliant: load-then-iterate (curl-in-loop stdin hazard), jq output to temp file read via $(<file), per-index temp files for forensics, shared local -r printf format constants. Pending one live rbw-ld run to confirm rendering.

### 2026-06-02 07:41 - ₢BHAAA - n

Register minted names: RBLD -> rbld_Lode.sh (Lode capture module), rbi_ld GAR namespace, rbw-l* colophon family, RBSL spec letter reserved. Minting-discipline hygiene for the navigable acronym map.

### 2026-06-02 07:40 - Heat - d

paddock curried: ₢BHAAA: record fidelity-is-service-tier finding (no offline fast layer; holds for all kinds) + flip Heat nature to base-kind-landed

### 2026-06-02 07:14 - ₢BHAAA - n

lode-lifecycle service fixture (ensconce -> divine enumerate/inspect -> banish -> restored, live GAR) in rbtdrc_crucible.rs; registered in manifest + service/complete suites. Regenerated rbtdgc_consts.rs (RBTDGC_ENSCONCE_BASE/DIVINE_LODES/BANISH_LODE) and tabtarget-context Command Reference for the rbw-l Lode group. Theurge builds clean.

### 2026-06-02 07:06 - ₢BHAAA - n

Lode base-kind capture: rbld module (ensconce/divine/banish) + rbi_ld GAR layout + two in-pool steps (skopeo capture into one rbi_ld/b<stamp> package under member tags; docker buildx :rbi_vouch envelope) + l-family zipper group + 3 tabtargets. divine enumerate path verified live against GAR (empty rbi_ld/, exit 0). enshrine untouched.

### 2026-06-02 06:32 - Heat - d

paddock curried: lock divine (rbw-ld, lowercase read-only) as the l-family Lode show/enumerate verb; clarify divine/audit grain split

### 2026-06-02 06:35 - Heat - f

silks=rbk-11-mvp-lode-universal-capture

### 2026-06-01 08:33 - Heat - d

paddock curried: ratify code-module prefix rbld_ (relaxes other-than-rbl caution: rbl is a pure container, no theme diluted)

### 2026-06-01 08:21 - Heat - d

paddock curried: lock Lode registry layout & naming from base-pilot design session: rbi_ld category, one-package-per-Lode atomic delete (with GCP-loop fallback), rbi_* sprue, base/reliquary member-tag taxonomy, :rbi_vouch envelope + two trust grades; resolves GAR-layout and provenance-attachment open issues

### 2026-06-01 06:20 - Heat - S

lode-remainder-replan

### 2026-06-01 06:19 - Heat - S

lode-base-pilot-sketch

### 2026-06-01 06:09 - Heat - f

silks=rbk-mvp-lode-universal-capture

### 2026-06-01 05:48 - Heat - d

paddock curried: de-historicize: strip decision-journal narration + datestamps to present-tense state; remove resolved side-label open issue; keep forward-looking constraint-warnings

### 2026-06-01 05:33 - Heat - d

paddock curried: 260601: lock final verb set (ensconce/fetter/conclave/underpin/immure capture, banish delete); revise fetched register to occult evocation/binding

### 2026-05-31 13:26 - Heat - d

paddock curried: 260531: record election decision in Carried forward — yoke + deferred substrate verb live with their consumer (vessel/nameplate), not in l; lowercase; operator-committed (RBSDY does-not-commit precedent), no self-gate (downstream consumer gates clean); yoke moves out of Depot d pace-time

### 2026-05-31 13:14 - Heat - d

paddock curried: 260531 harmonization pass: fixed flipped (below)->(above) cross-refs, reconciled two premises that still asserted verb-generalization/parallel-vocab against the per-kind reversal, fixed enshrinement-retirement reasoning citing retired verb, consolidated three-way open-issue tracking into one section, de-duplicated brand-file promotion claim, cleaned Shape/table-header wording. Yoke placement left for discussion.

### 2026-05-31 12:52 - Heat - d

paddock curried: 260531 design session: per-kind capture verbs (bank/sheathe/garner/harbor/vault, enshrine retired), expel delete verb, capture/election no-mixing + capture-file handoff, l command family + capital-colophon convention, RBSL spec letter, Ark-k-in-sibling rider, maintenance verbs stay in image

### 2026-05-31 07:48 - Heat - d

paddock curried: 260531 design session: Lode re-pointed to package (Ark-parallel), Touchmark minted as identifier, enshrine kept, no-unified-recipe-noun, Director-dictates-via-nameplate determinism premise, touchmark election map, silks/mirror/inscribe corrections

### 2026-05-29 08:01 - Heat - f

racing, silks=rbk-16-mvp-lode-universal-capture

### 2026-05-29 07:58 - Heat - d

paddock curried: fold in cloud-side acquisition, provenance-envelope invariant, GAR-package-as-atomic-delete-unit (single=compound by cardinality), consumption-scoping; fetched-side only

### 2026-05-14 11:20 - Heat - d

paddock curried: add test-coverage gate section requiring lifecycle fixtures before deploy

### 2026-05-13 14:37 - Heat - d

paddock curried: add GAR-package substrate + parallel cult verbs premises; lock captured-side delete-verb design

### 2026-05-13 12:21 - Heat - n

Sweep the remaining firemark and coronet glyphs from the RBS*.adoc family to honor the 'no JJ-state identifiers in public spec documents' discipline file-wide. Seventeen sites across five files: status markers in entry headings ('NOTE: SUPERSEDED (₣Av)', 'ELIMINATED (₣Av):'), inline AsciiDoc comments noting feature elimination (e.g. '// RBRG regime eliminated (₣Av) — reliquaries replace pin-based tool image management', appearing thrice in RBS0), historical relocation notes ('relocated during ₣A5 (rbk-mvp-3-public-docs-refresh)'), and two research-evidence coronet pointers in RBS0 body text ('(₢AvAAB research confirmed)', '(₣Av ₢AvAAD)'). All sites had the firemark/coronet parenthetical mechanically removed; the audit-trail content survives in git history and the elimination notices remain functional as comments without the heat identifier. Companion to the prior RBSHR cleanup in 1e658a30 — together those two commits leave the entire RBS*.adoc constellation free of firemark and coronet glyphs.

### 2026-05-13 12:18 - Heat - d

paddock curried: record Bullion-clearance for Lode namespace; add RBSHR/README to references

### 2026-05-13 12:12 - Heat - n

Correct README's host platform scope and add Windows-support roadmap entry. README previously claimed Windows host as release-1 qualified (Docker Desktop with WSL2 backend), but the project has not been tested on Windows — the claim was inaccurate. Shrinks Supported Platforms to Linux + macOS (two host families, not three), removes the Windows bullet, and adds a top-of-README Host-platform-scope note immediately after the IMPORTANT block stating the scope and linking to the Roadmap appendix. Adds a Windows host support entry to the Roadmap appendix that hints at — without detailing — the two strands of work required: (a) the planned revised-enshrine ceremony that generalizes today's narrow base-image mirror into a universal capture verb spanning build-time tools, project-controlled WSL distribution, and Podman VM disk images (closing the Microsoft-Store-appx-mutability gap); (b) bash transport stack hardening across the cmd.exe/PowerShell/Cygwin/wsl.exe legs that all Windows-host tabtarget invocations traverse. Adds an explicit anchor on the Roadmap heading so the new top-note and existing in-section pointers can link cleanly; replaces the stale §Future Work reference for Podman with a proper [Roadmap](#Roadmap) link. Implements the README half of the BH-heat-related documentation work — preparing the public-facing claims for the revised-enshrine rework signaled in RBSHR by the prior two commits.

### 2026-05-13 12:10 - Heat - n

Add two new horizon-roadmap entries to RBSHR signaling work that will eventually mount as paces: (a) revised enshrine — generalizes today's narrow base-OCI-mirror verb into a universal capture verb under a single recipe noun (Lode, parallel to Vessel), with a six-kind enum (base/tool/reliquary/wsl/podvm-wsl/podvm-native) that gates pipeline behavior; demotes reliquary from top-level noun to kind enum slot; retires the mirror verb (absorbed into enshrine on tool/reliquary kinds); preserves RBRV's ORIGIN/ANCHOR pattern so Lode .env carries author intent only. (b) Windows host workstation support — flags shell transport hardening required for Cygwin/PowerShell/wsl.exe/cmd.exe stack, cross-references WSG empirical rules and Windows workload-side specs. Also deletes the prior 'VM image supply chain targets GHCR, not GAR' entry — fully subsumed by the revised-enshrine entry's podvm-wsl/podvm-native kinds. Cleans up four pre-existing firemark/coronet references in the file (Heat AU, A1/A1AAj, Av status marker, and the new entries' references) to honor the discipline of keeping such JJ-state identifiers out of the public RBS* document constellation. Diff is entirely within RBSHR; no other files touched.

### 2026-05-13 12:04 - Heat - n

Rename AWS-side artifact 'Lode' to 'Bullion' in RBSHR horizon roadmap, freeing the Lode namespace for BH heat's planned use as the universal recipe noun parallel to Vessel. AWS-side artifact's bullion register (vault-stored, content-addressed, retention-locked) maps cleanly to S3+Object-Lock semantics and sits naturally next to the existing Payor/Manor/Levy financial vocabulary. Touches six prose sites across the Cross-cloud, Bullion entry, and Chantry entries — Lode was never a quoin so the rename is purely prose-level. Clears the way for the upcoming horizon entry that signals the revised-enshrine work (universal capture verb spanning base/tool/reliquary/wsl/podvm kinds) under the BH heat's intent.

### 2026-05-12 20:19 - Heat - f

stabled

### 2026-05-12 20:19 - Heat - f

stabled

### 2026-05-12 09:52 - Heat - d

paddock curried: Add Premises bullet: why podvm kinds retain selectively (curated subset, not full upstream manifest mirror). Captures the storage trade-off and warns against future regression toward full-mirror fidelity strawman.

### 2026-05-12 09:37 - Heat - d

paddock curried: Add Premises section: durable why-context (why this heat, why Lode, why the analogy, why no FQIN, why reliquary-as-kind, why six kinds, why generator-expressions) — to ease re-entry weeks from now

### 2026-05-12 09:31 - Heat - d

paddock curried: Lode model: universal capture noun, RBRV-pattern kind-gated regime, reliquary demoted to kind enum value, podman-vm split into wsl/native, generator expressions in place of premature enumerations

### 2026-05-12 08:15 - Heat - f

racing

### 2026-05-11 15:27 - Heat - f

silks=rbk-29-win-enshrine-triple

### 2026-05-11 12:59 - Heat - d

paddock curried: initial paddock draft capturing triple-enshrine design thinking

### 2026-05-11 12:58 - Heat - N

rbk-postmvp-1-triple-enshrine

