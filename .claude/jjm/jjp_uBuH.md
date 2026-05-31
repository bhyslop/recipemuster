## Vocabulary — decided (260531 session)

The noun system, settled in conversation. Three roles per side; all primary words are mutually **independent** — no word is a morphological derivative of another, matching the made-side discipline of `conjure` / `ark` / `hallmark`.

| Role | Made side | Fetched side |
|------|-----------|--------------|
| Recipe / intent (authored, reused) | Vessel | *(no unified noun — distributed per kind; see premise)* |
| Package (GAR substrate, atomic-delete unit, ~internal) | Ark | **Lode** |
| Identifier (the handle a consumer pins) | Hallmark | **Touchmark** |
| Capture/build verb (recipe → package) | ordain / conjure | **enshrine** |
| Delete verb (acts on the package) | abjure | *(open — next question)* |

- **`enshrine`** kept as the universal capture verb, generalized from base-only. The reading holds across the hostile-upstream kinds: you enshrine *your attested copy* into the depot under provenance — not a vouch for upstream's virtue.
- **`Lode`** names the **package** (Ark-parallel), re-pointed this session from the earlier recipe framing. The ore-deposit sense fits a stored body of bytes, and it gives the package an *independent* word that owes nothing to `enshrine`, preserving verb/noun independence.
- **`Touchmark`** names the **identifier** (Hallmark-parallel), minted this session. The shared `-mark` suffix with `hallmark` is a deliberate **role-signal**: any `*mark` is a specific-instance handle; the first syllable carries the side (`hall-` = what we forged, `touch-` = what we captured). `mint`/`mintmark` rejected — `mint` is load-bearing as the meta-verb for naming.
- **`enshrinement`** retires as a quoin — with the verb `enshrine` and the package `Lode`, no slot remains for the noun. The `rbi_es` namespace and the `*Enshrinements` tabtargets re-orient to Lode in the rename map.

## Premises

This section exists so a returning session can re-orient against the *why* of the model, not just the *what*. The choices below are load-bearing. When this heat mounts, everything here is re-investigated — these premises are the starting frame, not a frozen contract.

**Why this heat exists.** Three capture needs share gestalt: today's narrow base enshrinement, WSL substrate (the admin-distro replacement needed to take caparison-w off Microsoft's mutable appx supply), and podman VM (defensive mirror against quay's hostile lifecycle — new images every few hours, retention measured in days). The rule-of-three made verb-generalization load-bearing rather than premature. This revision absorbed a fourth case — today's reliquary tool-mirror operation — because its gesture (pull upstream → store in GAR with provenance) is identical. Refusing to unify it once the three-kinds story was honest would have been arbitrary.

**Why "Lode" names the package, not the recipe.** All capture cases share one verb shape, and the realistic alternative was parallel per-kind infrastructure with parallel vocabulary per kind; unification is the actual simplification, not a forced one. But the unified noun belongs at the **package** layer, not the recipe layer: what one verb genuinely produces across heterogeneous kinds is one *stored, provenance-wrapped GAR package*. Kinds carry the cardinality (1 vs N members), so one package noun loses no precision. (Re-pointed this session: the earlier framing placed Lode in the recipe slot, paralleling Vessel; the ore-deposit semantics and the verb/noun-independence discipline both pull it to the Ark-parallel package slot instead.)

**Why there is no unified recipe noun — intent forks per kind, as consumption does.** The natural owner of "what to capture" differs by kind: a base image's identity is intrinsic to the **vessel** built from it; the reliquary toolset is a build-infrastructure constant; wsl/podvm versions are host-substrate config. Forcing these into one recipe artifact is a false unity — the Load-Bearing Complexity smell, and it cuts against the consumption-forks premise. It works *without* a unified recipe because **enumeration lives on the artifact side**: "show me everything we capture" is an audit of the GAR Lodes by touchmark, never a recipe file. The kind-gated-variable-group *pattern* (RBRV gating on `RBRV_VESSEL_MODE`) still applies within any regime that happens to hold several kinds; only the unified *file* dissolves. (This revises the earlier "variable groups within the Lode regime gate on the kind enum" assumption, which presumed a single recipe regime existed.)

**Why the Director dictates the full version closure via the nameplate — "works for me" is a versioning failure.** Substrate touchmarks (`wsl` / `podvm`) are Director-pinned at the nameplate and materialized at charge, never left to Retriever or local-station choice. This is the fetched-side voicing of the system's existing anti-ambient-state ethos — cloud-side acquisition, content-addressed anchors, the depot tripwire, SLSA vouch: a result that depends on whatever the operator happened to have installed is a failure of strong versioning, not a convenience. The nameplate becomes the complete Director-dictated runtime closure (`hallmark + substrate touchmarks + network/ports`); the Retriever *receives* pinned versions rather than choosing them. The principle reaches the full **Lode closure** — everything capturable as a Lode (the podman-machine image, the WSL rootfs) — and stops at the **Pale**: the bare host OS, kernel, and hardware that *run* WSL and podman are a neighbor, not a subject. State the boundary so the principle does not over-claim control of the metal beneath it.

**Why acquisition runs cloud-side, never the workstation — co-equal with no-FQIN for regression-proneness.** Every kind fetches its bytes over the network from within GCP (skopeo / crane / oras / curl on Cloud Build, egress-permitted pool), trusting the GCP environment and network over the operator's potentially-compromised workstation. The WSL rootfs is fetched from the vendor's published HTTPS endpoint (Canonical cloud-images — the distro tarball the Store appx merely wraps) and verified against a published checksum; it is NOT exported from a workstation-installed distribution. The podman-VM prototype's "ignite VM" (a disposable podman machine that existed only to host crane because the workstation lacked it) dissolves entirely: Cloud Build carries crane/skopeo/oras natively. If a future pace proposes touching the workstation to *acquire* bytes, the trust model is being violated — the workstation only ever *consumes* already-verified artifacts.

**Why the provenance envelope is the true unifying invariant — not the storage shape.** What justifies one package noun across heterogeneous bytes is the acquisition-provenance envelope every instance carries: source coordinate, upstream digest/checksum, acquired-at, acquiring identity (the Cloud Build SA), and a signed vouch. This is the fetched-side parallel to the made-side SLSA — and it is a *different claim*: capture-fidelity ("we faithfully copied upstream X at digest D"), not build-integrity ("we built this securely"). The envelope has two honest trust grades, declared per kind: *verified-against-published* (base/tool/wsl, where upstream publishes a checksum to verify before vouching) and *recorded-at-acquisition* (podvm, where quay publishes no stable checksum, so you attest the digest you captured — trust-on-first-acquisition is the best the hostile upstream permits). The prototype's brand-file (origin / fqin / digest / identity) is this envelope in embryo; the model promotes it from an unsigned text file written at consumption time to a signed artifact emitted at acquisition time. Compound provenance is batch-level (the cohort as a whole); members stay individually manipulable for post-bug cleanup but need not each carry independent attestation.

**Why GAR's `package` is the substrate AND the atomic-delete unit.** Each Lode is one GAR `package` — this is now literal, since Lode *names* the package. `packages delete` removes the package and all its member versions in a single operation (verified against GAR's delete granularities: package / version / tag). That gives Director administration atomic whole-Lode delete for free, and per-member cleanup (`versions delete`, `tags delete`) for fixing bugs — with no custom reference-counting, because GAR content-addresses blobs and garbage-collects unreferenced ones. The consequence that reshapes the model: **single and compound Lodes are the same shape — a cardinality attribute, not a structural distinction.** A single-image Lode is a 1-member package; a reliquary or podvm cohort is an N-member package. Same delete primitive, same provenance attachment point, same wire shape. The earlier "compound carries a manifest-of-references payload" framing is retired — there is no separate refs object, so the "interior-hole / dangling-reference" regression class it created evaporates. Pre-deployment, layout is free to choose: members live as versions/tags *within one package*, not as sibling image-names, because GAR has no subtree delete — one package is what makes the single-call atomic delete possible.

**Why the Vessel:Ark :: Hallmark:Touchmark analogy is load-bearing — and where it deliberately stops.** The analogy now runs three-role: Vessel : *(distributed recipe)*, **Ark : Lode** (package), **Hallmark : Touchmark** (identifier). It is the orientation handle for acquisition, GAR-substrate, and delete questions: "how should operation X work for Lodes?" usually answers to "find X for Arks/Hallmarks and transfer." **The analogy does NOT extend to consumption.** A made image (Hallmark) and a fetched artifact (Touchmark) are consumed by entirely different actors — and a felt asymmetry *there* is real, not a sign of asking the question wrong. (This narrows a prior premise that told future sessions to distrust any asymmetry; that guidance was over-broad.)

**Why consumption stays per-kind and out of Lode's scope.** Lode unifies acquisition, the GAR substrate, provenance, and delete. It does not unify consumption: a made image is consumed as a Cloud Build FROM-line or build-mount; a `wsl` instance by the host's `wsl --import`; a podvm instance by the host's `podman machine init` — some consumers are not even on Cloud Build. Lode's value at consumption is not one verb; it is that the bytes the host consumes are cloud-acquired, verified, and vouched instead of locally-exported and uncontrolled. **Touchmark election co-locates with consumption and forks the same way** (see Touchmark Election). The `wsl` seed is the sharp case: today it is born from `wsl --export` of a Store-installed distro (the corruptible-workstation path); under this model the workload's `wsl --import` consumes a verified seed pulled from GAR — same consumption verb, trusted seed. A future session must resist inventing a *consumption-verb* unification (a "yoke for hosts"); the consumer asymmetry is intended — but note this is distinct from the determinism premise, which *does* unify the *authority* (Director, via nameplate) over substrate election.

**Why registry-level operations get parallel cult verbs per side, not shared verbs.** Made-side `abjure` and the fetched-side delete operation share one registry primitive (`packages delete`), but the noun-split keeps them as parallel verbs — distinct name, colophon, spec, and surface vocabulary, with the shared mechanism living in registry-utility code. The analogy carries through verbs; it does not collapse at the substrate. A future session tempted to unify because "the code is identical" should resist — the parallelism is load-bearing for the model even where the implementation is not.

**Why the per-kind intent carries no FQIN — most regression-prone principle.** When quay, docker-hub, or Canonical reorganizes upstream paths, kind-pipeline code changes; the per-kind intent declaration does not. If a future pace proposes adding resolved-coordinate detail to a kind's intent file, that is a smell — the principle is being violated. RBSRV's ORIGIN/ANCHOR pattern is the precedent: ORIGIN as author intent, ANCHOR as resolved-by-pipeline (and, on the fetched side, the resolved ANCHOR *is* the elected touchmark). The Lode model generalizes that pattern, not weakens it. Each kind's upstream-source convention lives in pipeline code where it can be fixed in one place.

**Why reliquary survives as a kind but not a top-level noun.** Vessels constrain Cloud Build toolsets by date-cohort identity; the cohort concept itself is empirically load-bearing — removing it would dissolve a real capability. But the project's cult-vocabulary count is high, and reductions are precious. Demotion to a single kind enum slot preserves the cohort semantic while dropping one top-level noun. Keep the word, drop the weight. An earlier framing flirted with promoting Reliquary to a universal grouping abstraction; rejected as wrong-direction because it made a cult noun *more* central, not less. (Consequence: the old "reliquary stamp" was a separate identifier only because reliquary was its own noun — it is now simply the **touchmark** of a `reliquary`-kind Lode. No distinct cohort-identifier survives.)

**Why six kinds, not four.** `rbv_PodmanVM.sh` revealed that podman-vm fan-out is two-family at quay (`machine-os-wsl` for the Windows-via-WSL host path, `machine-os` for the macOS/Linux native host path) with asymmetric host-platform consumer paths (caparison-w for the WSL flavor; nothing analogous for native). Splitting honors structural truth, not classification preference. The `tool` and `reliquary` kinds enter because tool-mirror absorption required them — `tool` is the single-member case, `reliquary` the date-cohort multi-member.

**Why podvm kinds retain selectively, not in bulk.** The upstream manifest lists at quay span 5-15 GB across all host platforms (architecture × disktype × family). Project storage cost scales only to selected variants — a compound Lode's package holds exactly the curated member set. The prototype already commits to this shape via `crane manifest` and `crane blob` digest navigation; the model preserves it. Trade-off worth knowing: widen the captured platform set defensively at acquisition time, because quay's hostile retention can make later expansion impossible once an upstream version ages out. The pipeline should support "expand selected set against same upstream version" as a first-class refresh mode, separate from "bump to new podman version." Resist regressing toward full-mirror out of a misguided fidelity instinct — cohort identity is project-controlled, vouching attests to the curated subset, and per-platform reproducibility from upstream digests is preserved either way.

**Why generator expressions, not enumerations.** A list of "verbs affected" or "files to change" written today is wrong tomorrow — every unrelated touch ages it. A discovery recipe survives those churns. Future mount sessions should resist expanding recipes into lists until paces actually approach the work. Paddock complexity compounds, and complex paddocks become unread — which defeats their purpose. Discipline, not preference.

## Shape

Generalize today's narrow `enshrine` (base OCI mirror) into a universal verb for project-controlled capture of upstream artifacts. The verb spans heterogeneous content kinds — base images, build-time tools, WSL substrate, and podman machine images — producing a single **package** noun, **Lode**, paralleling Ark on the made side; each Lode is referred to by its **Touchmark**, paralleling Hallmark.

The analogy (acquisition / substrate / delete only — see premises):

| Role | Made side | Fetched side |
|------|-----------|--------------|
| Recipe / intent | Vessel | *(distributed per kind — no unified noun)* |
| Package | Ark | **Lode** |
| Identifier | Hallmark | **Touchmark** |

The enshrine pipeline, running on Cloud Build, resolves upstream coordinates from kind conventions, fetches bytes from within GCP, verifies them (against a published checksum where the upstream offers one), captures them into a GAR package (the Lode), and emits a signed provenance envelope. **Intent declarations are distributed, not unified:** each kind's "what to capture" lives in its naturally-owning regime — `base` in the vessel (`RBRV_IMAGE_n_ORIGIN`), `tool`/`reliquary` as a build-infrastructure constant, `wsl`/`podvm` in host-tier config. Wherever intent lives it stays declarative-only — no FQIN, no resolved-identity field; kindle code computes upstream coordinates and GAR tags from primitives at module-startup, mirroring the prototype's `ZRBV_VMIMAGE_TAG_PREFIX` pattern.

Scope is the fetched side only. The same package / atomic-delete / provenance-envelope model is mechanically true of the made side (the Ark/Hallmark already are this), and the symmetry is confirmed (`abjure` is essentially an Ark `packages delete`) — but retrofitting made-images is left to a sibling heat to keep this one bounded.

## Touchmark election

A touchmark is *elected* where a consumer pins a specific captured deposit to use. Election is a consumption-side act, so it forks per kind — and it co-locates with intent in the same naturally-owning regime. The fork tracks **build-time vs runtime**, which maps cleanly onto **vessel vs nameplate**:

| Kind | Consumed | Elected in | Mechanism |
|------|----------|-----------|-----------|
| `base` | build-time (FROM) | vessel (the ANCHOR slot, renamed) | **enshrine writes it** — derived, not operator-pinned |
| `tool` / `reliquary` | build-time (syft/skopeo) | vessel (the RELIQUARY slot) | **yoke stamps it** across all vessels |
| `wsl` | runtime substrate | **nameplate** (Director-dictated) | charge-time provision (deferred) |
| `podvm-*` | runtime substrate | **nameplate** (Director-dictated) | charge-time provision (deferred) |

The election *mechanism* differs even where the *location* is the same — `base` is auto-written by enshrine, `reliquary` is explicitly stamped by yoke — and the rename map must preserve that distinction (do not collapse "enshrine writes ANCHOR" and "yoke writes RELIQUARY" because both land in the vessel). Nameplate is **not** a touchmark site for the build-time kinds; for the runtime-substrate kinds it *is* the election site, under Director authority, per the determinism premise. The charge-time provisioning mechanism for substrate touchmarks is consumption-side and deferred this heat (see Heat nature).

## Package layout

One Lode = one GAR `package`. Members are versions/tags within that single package; batch-level provenance (about / vouch) attaches to the same package — reserved tag names, or the OCI referrers API if GAR support is mature enough (pace-time choice). Atomic whole-Lode delete is `packages delete` (one call); per-member cleanup is `versions delete` / `tags delete`.

- Single-member kinds (`base`, `tool`, `wsl`): a 1-member package + its provenance.
- Multi-member kinds (`reliquary`, `podvm-wsl`, `podvm-native`): an N-member package + whole-batch provenance; each member individually addressable and deletable for post-bug cleanup.

"Atomic" here means a single delete operation, not transactional rollback — no registry guarantees clean state if a bulk delete fails mid-flight; per-member fixtures must assert member absence after partial deletes.

## Kinds

| Kind | Members | Upstream source | Digest grade |
|------|---------|-----------------|--------------|
| `base` | 1 | upstream OCI registry, consumed as FROM line | verified |
| `tool` | 1 | upstream OCI registry, consumed at build time | verified |
| `reliquary` | N | date-cohort of `tool` members (replaces today's reliquary noun) | verified per member |
| `wsl` | 1 | vendor-published rootfs tarball over HTTPS (e.g. Canonical cloud-images) | verified |
| `podvm-wsl` | N | `quay.io/podman/machine-os-wsl` — platform fan-out, consumed by Windows podman | recorded |
| `podvm-native` | N | `quay.io/podman/machine-os` — platform fan-out, consumed by macOS/Linux podman | recorded |

Member count is the only structural variable — every kind is "a GAR package holding 1..N members plus batch provenance." `wsl` and `podvm-wsl` both touch WSL but are entirely distinct: `wsl` is the Linux distro rootfs that hosts everything on the Windows workload, `podvm-wsl` is the podman machine image that runs *inside* that distro. Names kept visibly different.

Payload shapes (orthogonal to member count): native layered OCI image (`base`, `tool`) vs opaque-blob-wrapped-as-OCI (`wsl`, `podvm-*` — a rootfs tar or disk blob in a scratch/oras wrapper). The provenance envelope rides uniformly on both.

## Carried forward / consequences

- **caparison-w / garrison-w for the `wsl` kind.** The workload's `rbtww-main` is registered via `wsl --import` from a seed tarball; today that seed is created by `wsl --export` of admin's installed distro (GarrisonWsl). Under the `wsl` kind, that export path retires — the seed becomes a verified artifact pulled from GAR. The pre-existing revert pace in ₣A- targeting the WSL-stage DEV CACHE shortcut becomes obsolete and wants dropping or transferring when this heat commits to development.
- **Windows onboarding is stale and deferred.** The `rbhw*` Windows handbook tracks (Docker Desktop, context discipline) are stale and incomplete, and `wsl`/`podvm` consumption is deferred — so their onboarding has nothing live to teach. Lean: mark those tracks loudly NOT-available / deferred rather than delete, so the eventual `wsl`-kind consumption work rereads the host-side prose rather than reconstructing it. Onboarding follows *consumption*, not *acquisition*: the Lode acquisition unification does NOT unify the onboarding surface; substrate-kind onboarding lands in the host/Windows tracks, never in the Director build-lifecycle tracks.
- **`inscribe` is verb-overloaded — disambiguate, don't conflate.** `rbfl_inscribe` (reliquary tool-mirror, `rbw-dI`) is **in scope** — it is the operation absorbed into enshrine on the `tool`/`reliquary` kinds. `rbrd_inscribe` (depot tripwire, `rbw-rdi`) is a **different operation, out of scope, stays put**. (Strike the earlier "the `mirror` verb retires" wording: at quoin level there is no mirror verb — `rbfd_mirror` is the bind vessel-mode copy, made-side, also out of scope. What is absorbed is `inscribe`/reliquary, not "mirror.")
- **Onboarding rename surface (tri-surfaced vs greenfield).** `base`/`tool`/`reliquary` are fully present across bash, adoc, and the Director onboarding tracks (`rbw-Odf`/`Oda`/`Odb`) — for these the cost is *rename* (only two typed linked terms, `RBYC_ENSHRINE` + `RBYC_RELIQUARY`, drive the typed references; the rest is hand-edited prose). `wsl`/`podvm-*` are surfaced *nowhere* today — for these the cost is *greenfield*, not rename. Do not estimate the onboarding work as uniform.
- **Refresh cadence** is per-kind and may diverge (podvm-wsl vs podvm-native).
- Tabtarget shape for the universal verb (unified `rbw-dE` with a kind argument vs siblings) is a pace-time decision.

## Discovery recipes — work to enumerate when paces approach

- **Verb redesign blast radius.** Scan `Tools/rbk/vov_veiled/RBSA*.adoc` and `RBSI*.adoc` for verbs touching captured artifacts; for each, classify per-kind variance (unchanged / split / retired). Yoke is the headline because consumer-landing differs per kind; other splits expected. Recipe, not enumeration — premature listing ages poorly. (Quoin anchors live in `RBS0-SpecTop.adoc`: `rbtgo_ark_enshrine`, `rbtgo_depot_inscribe`, `rbtgo_director_yoke`, `gar_enshrines_namespace`/`gar_reliquaries_namespace`, `rbst_reliquary_stamp`, `rbf_fact_reliquary` are the capture cluster.)
- **Substrate election landing.** For `wsl` and `podvm-*`, the touchmark is Director-pinned at the nameplate (determinism premise), materialized at charge — but *which* host-tier regime backs the node-side provisioning (station regime vs BURN node profile vs a new host-config regime) is open. Decide when the first non-vessel Lode consumption pace mounts.
- **Spec letter allocation** for the new Lode spec — consult CLAUDE.md Prefix Naming and Quoin Sub-Letter Discipline; check terminal exclusivity against the RBS* tree.
- **GAR provenance attachment mechanism** — reserved tag names vs OCI referrers API; verify GAR referrers maturity before committing.
- **GAR layout collapse** — whether `rbi_es` (enshrines) and `rbi_rq` (reliquaries) collapse into one Lode namespace shape is a pace-time layout choice; one-package-per-Lode is the only fixed constraint.

## Test coverage gate before deploy

Lifecycle fixtures against the new Lode surface must be slated and landed before deploy: capture / list / inspect / wrest / delete end-to-end against live GAR per kind, bookended by prereq-sweep at head and muster-absent at tail. The bar forks by kind: `base`/`tool`/`reliquary` carry their consumption-adjacent paths; `wsl`/`podvm` stop at the registry — **capture → list → inspect → per-member + whole-Lode delete against live GAR, no host in the loop** (consumption deferred). Whole-Lode delete is a single `packages delete` — the regression risk shifts to the **per-member delete path** (the cleanup-after-bugs case): multi-member kinds must carry concrete member-absence assertions after a partial delete, since that is where a package can be left in a partial state. The fixture surface scales with the operation surface; test paces are not optional polish. Manual operator testing during planning is the bridge, not a substitute.

## Vocabulary — still parked / next questions

Resolved this session (now in Vocabulary — decided): the package noun (Lode), the identifier (Touchmark), keeping `enshrine`, the no-unified-recipe-noun call, reliquary→kind, the `-mark` role-suffix. The cohort-stamp term dissolved (it is the touchmark of a `reliquary` Lode). Still open:

- **Fetched-side delete verb** (parallel to `abjure`) — open. Forge/Solomonic register, lexically distinct, must parse against `Lode`/`Touchmark`; colophon in the depot family `rbw-d?`; shared `packages delete` backbone, distinct verb. Note: this verb is *new* (no current identifier retires into it).
- **"Made" vs "fetched"** (the side distinction) — load-bearing concept (did Recipe Bottle build the bytes, or pull someone else's); the working labels are descriptive placeholders (prior unreviewed labels were "built/captured") and open to a better pair when minting.
- **Spec letter** for the new Lode spec — gates every *new* `rbXX_` quoin prefix; terminal-exclusivity check against the RBS* tree (a sub-agent sweep suggested several free single letters but it is unverified — treat as pace-time).
- **GAR layout** — the `rbi_es` + `rbi_rq` collapse question (above).
- **Host-tier regime for substrate election** — which regime backs nameplate-pinned `wsl`/`podvm` provisioning (above).

## Next questions for refinement (interview order)

The conversational interview that produced this revision settled Q1–Q3 + the recipe-vacancy + the determinism premise. Remaining, in the same order:

1. **Fetched-side delete verb** (parallel to `abjure`) — the natural next mint.
2. **Made/fetched side-labels** — the words for the whole distinction.
3. **Spec letter** for the Lode spec.
4. **GAR layout** — namespace collapse.
5. **Host-tier regime** for substrate touchmark election/provisioning.

(Items 3–5 are genuinely pace-time and may be left to the first relevant pace rather than settled in conversation.)

## Heat nature

Planning/design heat — no paces yet. Paddock encodes the model and the recipes for follow-up work; the remaining naming and landing-regime decisions defer until paces commit, when everything here is re-investigated. Scope locked to the fetched side (made-images out of scope; see Shape) and to acquisition (wsl/podvm consumption deferred — this heat captures their bytes into GAR and proves control there; host consumption is the locked-but-deferred sequel). Heat silks are `rbk-16-mvp-lode-universal-capture` — already aligned with the universal-capture / Lode framing.

## References

- ₣AV `rbw-implement-gar-mirroring` (retired) — predecessor heat
- ₣Az `rbk-90-deferred-ark-concept-debt` (retired) — closed by the GAR-package substrate finding
- RBSAE `ark_enshrine` — current narrow-scope spec; becomes the `base`-kind pipeline
- RBSAS `ark_summon`, RBSDI `depot_inscribe`, RBSDY `director_yoke` — verbs within blast radius
- RBSRV `RegimeVessel` — ORIGIN/ANCHOR precedent; enum-gated-variable-groups pattern
- `Tools/rbk/vov_veiled/FUTURE/rbv_PodmanVM.sh` + RBSPV — concrete podman-vm pipeline; reveals fan-out, the two-source quay split, the brand-file provenance embryo, and the ignite-VM machination that cloud-side acquisition dissolves
- BUSJCW CaparisonWindows, BUSJGW GarrisonWsl, BUSJIW InvigilateWindows — `wsl` kind consumption and the `wsl --export` seed path it retires
- WSL rootfs is vendor-published over HTTPS (Canonical cloud-images; Microsoft `DistributionInfo.json` carries per-distro URL + SHA-256) — basis for cloud-side `wsl` acquisition with no Windows in the loop
- GAR delete granularity (package / version / tag) verified — `packages delete` is the atomic whole-Lode delete unit
- CLAUDE.md "Prefix Naming Discipline" and "Quoin Sub-Letter Discipline"
- RBSHR-HorizonRoadmap.adoc — public-facing horizon entries