## Premises

This section exists so a returning session can re-orient against the *why* of the model, not just the *what*. The choices below are load-bearing. When this heat mounts, everything here is re-investigated — these premises are the starting frame, not a frozen contract.

**Why this heat exists.** Three capture needs share gestalt: today's narrow base enshrinement, WSL substrate (the admin-distro replacement needed to take caparison-w off Microsoft's mutable appx supply), and podman VM (defensive mirror against quay's hostile lifecycle — new images every few hours, retention measured in days). The rule-of-three made verb-generalization load-bearing rather than premature. This revision absorbed a fourth case — today's reliquary "mirror" operation — because its gesture (pull upstream → store in GAR with provenance) is identical. Refusing to unify it once the three-kinds story was honest would have been arbitrary.

**Why "Lode" earns the universal recipe role.** All four+ capture cases share one verb shape. The realistic alternative was parallel per-kind infrastructure with parallel vocabulary per kind; unification is the actual simplification, not a forced one. Kinds carry the cardinality information (one member vs many), so collapsing to one recipe noun loses no precision — the kind enum holds what the parallel vocabulary would have.

**Why acquisition runs cloud-side, never the workstation — co-equal with no-FQIN for regression-proneness.** Every kind fetches its bytes over the network from within GCP (skopeo / crane / oras / curl on Cloud Build, egress-permitted pool), trusting the GCP environment and network over the operator's potentially-compromised workstation. The WSL rootfs is fetched from the vendor's published HTTPS endpoint (Canonical cloud-images — the distro tarball the Store appx merely wraps) and verified against a published checksum; it is NOT exported from a workstation-installed distribution. The podman-VM prototype's "ignite VM" (a disposable podman machine that existed only to host crane because the workstation lacked it) dissolves entirely: Cloud Build carries crane/skopeo/oras natively. If a future pace proposes touching the workstation to *acquire* bytes, the trust model is being violated — the workstation only ever *consumes* already-verified artifacts.

**Why the provenance envelope is the true unifying invariant — not the storage shape.** What justifies one recipe noun across heterogeneous bytes is the acquisition-provenance envelope every instance carries: source coordinate, upstream digest/checksum, acquired-at, acquiring identity (the Cloud Build SA), and a signed vouch. This is the fetched-side parallel to the made-side SLSA — and it is a *different claim*: capture-fidelity ("we faithfully copied upstream X at digest D"), not build-integrity ("we built this securely"). The envelope has two honest trust grades, declared per kind: *verified-against-published* (base/tool/wsl, where upstream publishes a checksum to verify before vouching) and *recorded-at-acquisition* (podvm, where quay publishes no stable checksum, so you attest the digest you captured — trust-on-first-acquisition is the best the hostile upstream permits). The prototype's brand-file (origin / fqin / digest / identity) is this envelope in embryo; the model promotes it from an unsigned text file written at consumption time to a signed artifact emitted at acquisition time. Compound provenance is batch-level (the cohort as a whole); members stay individually manipulable for post-bug cleanup but need not each carry independent attestation.

**Why GAR's `package` is the substrate AND the atomic-delete unit.** Each Lode instance is one GAR `package`. This is not merely "stored as a package" — `packages delete` removes the package and all its member versions in a single operation (verified against GAR's delete granularities: package / version / tag). That gives Director administration atomic whole-Lode delete for free, and per-member cleanup (`versions delete`, `tags delete`) for fixing bugs — with no custom reference-counting, because GAR content-addresses blobs and garbage-collects unreferenced ones. The consequence that reshapes the model: **single and compound Lodes are the same shape — a cardinality attribute, not a structural distinction.** A single-image Lode is a 1-member package; a reliquary or podvm cohort is an N-member package. Same delete primitive, same provenance attachment point, same wire shape. The earlier "compound carries a manifest-of-references payload" framing is retired — there is no separate refs object, so the "interior-hole / dangling-reference" regression class it created evaporates. Pre-deployment, layout is free to choose: members live as versions/tags *within one package*, not as sibling image-names, because GAR has no subtree delete — one package is what makes the single-call atomic delete possible.

**Why the Vessel:Lode :: Hallmark:{INSTANCE_NOUN} analogy is load-bearing — and where it deliberately stops.** It is the orientation handle for acquisition, GAR-substrate, and delete questions: "how should operation X work for Lodes?" usually answers to "find X for Vessels/Hallmarks and transfer." The instance-noun's lexical choice is open; the structural role is locked. **The analogy does NOT extend to consumption.** A made image (Hallmark) and a fetched artifact (Lode instance) are consumed by entirely different actors — and a felt asymmetry *there* is real, not a sign of asking the question wrong. (This narrows a prior premise that told future sessions to distrust any asymmetry; that guidance was over-broad.)

**Why consumption stays per-kind and out of Lode's scope.** Lode unifies acquisition, the GAR substrate, provenance, and delete. It does not unify consumption: a made image is consumed as a Cloud Build FROM-line or build-mount; a `wsl` instance by the host's `wsl --import`; a podvm instance by the host's `podman machine init` — some consumers are not even on Cloud Build. Lode's value at consumption is not one verb; it is that the bytes the host consumes are cloud-acquired, verified, and vouched instead of locally-exported and uncontrolled. The `wsl` seed is the sharp case: today it is born from `wsl --export` of a Store-installed distro (the corruptible-workstation path); under this model the workload's `wsl --import` consumes a verified seed pulled from GAR — same consumption verb, trusted seed. A future session must resist inventing a consumption-side unification (a "yoke for hosts"); the consumer asymmetry is intended.

**Why registry-level operations get parallel cult verbs per side, not shared verbs.** Made-side `abjure` and the fetched-side delete operation share one registry primitive (`packages delete`), but the recipe/instance vocabulary split keeps them as parallel verbs — distinct name, colophon, spec, and surface vocabulary, with the shared mechanism living in registry-utility code. The analogy carries through verbs; it does not collapse at the substrate. A future session tempted to unify because "the code is identical" should resist — the parallelism is load-bearing for the recipe model even where the implementation is not.

**Why the .env carries no FQIN — most regression-prone principle.** When quay, docker-hub, or Canonical reorganizes upstream paths, kind-pipeline code changes; .env files do not. If a future pace proposes adding resolved-coordinate detail to a Lode .env, that is a smell — the principle is being violated. RBSRV's ORIGIN/ANCHOR pattern is the precedent: ORIGIN as author intent, ANCHOR as resolved-by-pipeline. The Lode model generalizes that pattern, not weakens it. Each kind's upstream-source convention lives in pipeline code where it can be fixed in one place.

**Why reliquary survives as a kind but not a top-level noun.** Vessels constrain Cloud Build toolsets by date-cohort identity; the cohort concept itself is empirically load-bearing — removing it would dissolve a real capability. But the project's cult-vocabulary count is high, and reductions are precious. Demotion to a single kind enum slot preserves the cohort semantic while dropping one top-level noun. Keep the word, drop the weight. An earlier framing flirted with promoting Reliquary to a universal grouping abstraction; rejected as wrong-direction because it made a cult noun *more* central, not less.

**Why six kinds, not four.** `rbv_PodmanVM.sh` revealed that podman-vm fan-out is two-family at quay (`machine-os-wsl` for the Windows-via-WSL host path, `machine-os` for the macOS/Linux native host path) with asymmetric host-platform consumer paths (caparison-w for the WSL flavor; nothing analogous for native). Splitting honors structural truth, not classification preference. The `tool` and `reliquary` kinds enter because mirror absorption required them — `tool` is the single-member case, `reliquary` the date-cohort multi-member.

**Why podvm kinds retain selectively, not in bulk.** The upstream manifest lists at quay span 5-15 GB across all host platforms (architecture × disktype × family). Project storage cost scales only to selected variants — a compound Lode's package holds exactly the curated member set. The prototype already commits to this shape via `crane manifest` and `crane blob` digest navigation; the model preserves it. Trade-off worth knowing: widen the captured platform set defensively at acquisition time, because quay's hostile retention can make later expansion impossible once an upstream version ages out. The pipeline should support "expand selected set against same upstream version" as a first-class refresh mode, separate from "bump to new podman version." Resist regressing toward full-mirror out of a misguided fidelity instinct — cohort identity is project-controlled, vouching attests to the curated subset, and per-platform reproducibility from upstream digests is preserved either way.

**Why generator expressions, not enumerations.** A list of "verbs affected" or "files to change" written today is wrong tomorrow — every unrelated touch ages it. A discovery recipe survives those churns. Future mount sessions should resist expanding recipes into lists until paces actually approach the work. Paddock complexity compounds, and complex paddocks become unread — which defeats their purpose. Discipline, not preference.

## Shape

Generalize today's narrow `enshrine` (base OCI mirror) into a universal verb for project-controlled capture of upstream artifacts. The verb spans heterogeneous content kinds — base images, build-time tools, WSL substrate, and podman machine images — under a single recipe noun, **Lode**, paralleling Vessel on the made side.

The analogy (acquisition / substrate / delete only — see premises):

| Made side | Fetched side |
|-----------|--------------|
| Vessel (recipe) | Lode (recipe) |
| Hallmark (specific build) | {INSTANCE_NOUN} (specific capture) |

A Lode declares author intent (kind + kind-specific primitives + sigil); the enshrine pipeline, running on Cloud Build, resolves upstream coordinates from kind conventions, fetches bytes from within GCP, verifies them (against a published checksum where the upstream offers one), captures them into a GAR package, and emits a signed provenance envelope. Lode .env stays declarative-only — no FQIN, no resolved-identity field; kindle code computes upstream coordinates and GAR tags from primitives at module-startup, mirroring the prototype's `ZRBV_VMIMAGE_TAG_PREFIX` pattern. Variable groups within the Lode regime gate on the kind enum exactly as RBRV's groups gate on `RBRV_VESSEL_MODE`.

Scope is the fetched side only. The same package / atomic-delete / provenance-envelope model is mechanically true of the made side (Hallmark), and the symmetry is confirmed (`abjure` is essentially a hallmark `packages delete`) — but retrofitting made-images is left to a sibling heat to keep this one bounded.

## Package layout

One Lode instance = one GAR `package`. Members are versions/tags within that single package; batch-level provenance (about / vouch) attaches to the same package — reserved tag names, or the OCI referrers API if GAR support is mature enough (pace-time choice). Atomic whole-Lode delete is `packages delete` (one call); per-member cleanup is `versions delete` / `tags delete`.

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
- **Refresh cadence** is per-kind and may diverge (podvm-wsl vs podvm-native).
- **The `mirror` verb retires entirely**, absorbed into enshrine on the `tool` / `reliquary` kinds. Tabtarget shape (unified `rbw-dE` with a kind argument vs siblings) is a pace-time decision.

## Discovery recipes — work to enumerate when paces approach

- **Verb redesign blast radius.** Scan `Tools/rbk/vov_veiled/RBSA*.adoc` and `RBSI*.adoc` for verbs touching captured artifacts; for each, classify per-kind variance (unchanged / split / retired). Yoke is the headline because consumer-landing differs per kind; other splits expected. Recipe, not enumeration — premature listing ages poorly.
- **Non-vessel consumer landing.** For `wsl` and `podvm-*`, the resolved instance has no vessel rbrv.env to yoke into — a *consumption-side* (host config) question, outside Lode's core acquisition scope. Decide when the first non-vessel Lode pace mounts; candidates include station-regime extensions or a host-config regime.
- **Spec letter allocation** for the new Lode spec — consult CLAUDE.md Prefix Naming and Quoin Sub-Letter Discipline; check terminal exclusivity against the RBS* tree.
- **GAR provenance attachment mechanism** — reserved tag names vs OCI referrers API; verify GAR referrers maturity before committing.

## Test coverage gate before deploy

Lifecycle fixtures against the new Lode surface must be slated and landed before deploy: capture / list / inspect / wrest / delete end-to-end against live GAR per kind, bookended by prereq-sweep at head and muster-absent at tail. Whole-Lode delete is now a single `packages delete` — the regression risk shifts to the **per-member delete path** (the cleanup-after-bugs case): multi-member kinds must carry concrete member-absence assertions after a partial delete, since that is where a package can be left in a partial state. The fixture surface scales with the operation surface; test paces are not optional polish. Manual operator testing during planning is the bridge, not a substitute.

## Vocabulary parked

- **Lode** chosen as the universal recipe noun. **Unction** held as strongest rejected alternative; **Oblate** rejected (RBRO_ collides with OAuth). The AWS-side prose occupant of "Lode" in RBSHR's Cross-cloud cluster was renamed to **Bullion**, clearing the namespace.
- **Reliquary** demoted from top-level noun to a kind enum value. Word survives; weight reduced.
- **{INSTANCE_NOUN}** (specific-capture noun, parallel to Hallmark) is open; letter-collision is tight — review before naming. Anchor floated but uncertain.
- **Fetched-side delete verb** (parallel to `abjure`) is open — Forge/Solomonic register, lexically distinct, must parse against the prospective instance-noun; colophon in the depot family `rbw-d?`; shared `packages delete` backbone, distinct verb.
- **"Made" vs "fetched"** (the side distinction) — load-bearing concept (did Recipe Bottle build the bytes, or pull someone else's); the working labels are descriptive placeholders (prior unreviewed labels were "built/captured") and open to a better pair when minting.

## Heat nature

Planning/design heat — no paces yet. Paddock encodes the model and the recipes for follow-up work; verb-fate, landing-regime, and instance-noun decisions defer until paces commit, when everything here is re-investigated. Scope locked to the fetched side (made-images out of scope; see Shape). Heat silks `rbk-29-win-enshrine-triple` now mismatches scope and wants relabeling toward the universal-capture / Lode framing once the model is approved.

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