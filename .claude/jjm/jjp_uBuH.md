## Premises

This section exists so a returning session can re-orient against the *why* of the model, not just the *what*. The choices below are load-bearing; understanding them is necessary to keep follow-up paces aligned with the heat's intent.

**Why this heat exists.** Three capture needs share gestalt: today's narrow base enshrinement, WSL substrate (the admin-distro replacement needed to take caparison-w off Microsoft's mutable appx supply), and podman VM (defensive mirror against quay's hostile lifecycle — new images every few hours, retention measured in days). The rule-of-three test made verb-generalization load-bearing rather than premature abstraction. This revision absorbed a fourth case — today's reliquary "mirror" operation — because its gesture (pull upstream → SLSA-wrap → store in GAR) is identical to the others. Refusing to unify it once the three-kinds story was honest would have been arbitrary.

**Why "Lode" earns the universal recipe role.** All four+ capture cases share one verb shape. The realistic alternative was parallel per-kind infrastructure with parallel vocabulary per kind; unification is the actual simplification, not a forced one. Kinds carry the cardinality information (atomic 1:1 vs compound 1:N for the fan-out cases), so collapsing to one recipe noun loses no precision — the kind enum holds what the parallel vocabulary would have.

**Why the Vessel:Lode :: Hallmark:{INSTANCE_NOUN} analogy is load-bearing.** It is the orientation handle for design questions not yet in the paddock. When a future pace asks "how should operation X work for Lodes?", the answer is usually "find X for Vessels and transfer." The instance-noun's lexical choice is open; the structural role is locked. If a future session is tempted to break the analogy because it feels asymmetric, that asymmetry is almost certainly a sign the question is being asked wrong.

**Why the .env carries no FQIN — most regression-prone principle.** When quay or docker-hub reorganizes upstream paths, kind-pipeline code changes; .env files do not. If a future pace proposes adding resolved-coordinate detail to a Lode .env, that is a smell — the principle is being violated. RBSRV lines 119-144 are the existing precedent: ORIGIN as author intent, ANCHOR as resolved-by-pipeline. The Lode model generalizes that pattern, not weakens it. Each kind's upstream-source convention lives in pipeline code where it can be fixed in one place.

**Why reliquary survives as a kind but not as a top-level noun.** Vessels constrain Cloud Build toolsets by date-cohort identity; the cohort concept itself is empirically load-bearing — removing it would dissolve a real capability. But the project's cult-vocabulary count is high, and reductions are precious. Demotion to a single kind enum slot preserves the cohort semantic while dropping one top-level noun. Keep the word, drop the weight. An earlier framing of this revision flirted with promoting Reliquary to a universal grouping abstraction; that was rejected as wrong-direction because it made a cult noun *more* central, not less.

**Why six kinds, not four.** `rbv_PodmanVM.sh` revealed that podman-vm fan-out is two-family at quay (`machine-os-wsl` for the Windows-via-WSL host path, `machine-os` for the macOS/Linux native host path) with asymmetric host-platform consumer paths (caparison-w for the WSL flavor; nothing analogous for native). Splitting honors structural truth, not classification preference. The `tool` and `reliquary` kinds enter because mirror absorption required them — `tool` is the atomic per-image case, `reliquary` is the date-cohort compound.

**Why podvm kinds retain selectively, not in bulk.** The upstream manifest lists at quay span 5-15 GB across all host platforms (architecture × disktype × family). Project storage cost scales only to selected variants — a compound Lode's manifest *is* the curated subset. The existing `rbv_PodmanVM.sh` already commits to this shape via `crane manifest` and `crane blob` digest navigation; the model preserves it. Trade-off worth knowing: widen the captured platform set defensively at enshrine time, because quay's hostile retention (new images every few hours, days of retention) can make later expansion impossible once an upstream version has aged out. The pipeline should support "expand selected set against same upstream version" as a first-class refresh mode, separate from "bump to new podman version." A future session may be tempted to regress toward full-mirror out of a misguided fidelity instinct ("we mirror what quay shipped") — resist: cohort identity is project-controlled, vouching attests to the curated subset, and per-platform reproducibility from upstream digests is preserved either way.

**Why generator expressions, not enumerations.** A list of "verbs affected" or "files to change" written today is wrong tomorrow — every unrelated touch ages it. A discovery recipe ("scan RBSA*.adoc and RBSI*.adoc for verbs touching captured artifacts") survives those churns. Future mount sessions should resist the urge to expand recipes into lists until paces actually approach the work. Paddock complexity compounds, and complex paddocks become unread — which defeats their purpose. Treat this as discipline, not preference.

## Shape

Generalize today's narrow `enshrine` (base OCI mirror) into a universal verb for project-controlled capture of upstream artifacts. The verb spans heterogeneous content kinds — base images, build-time tools, WSL substrate, and podman machine images — under a single recipe noun, **Lode**, paralleling Vessel on the built side.

The analogy:

| Built side | Captured side |
|-----------|---------------|
| Vessel (recipe) | Lode (recipe) |
| Hallmark (specific build) | {INSTANCE_NOUN} (specific capture) |

A Lode declares author intent (kind + kind-specific primitives + sigil); the enshrine pipeline resolves upstream coordinates from kind conventions, captures bytes into GAR, and emits a resolved {INSTANCE_NOUN} that consumers reference. Lode .env stays declarative-only — no FQIN field, no resolved-identity field. Kindle code computes upstream FQINs and GAR tags from primitives at module-startup, mirroring the existing `ZRBV_VMIMAGE_TAG_PREFIX` pattern in `rbv_PodmanVM.sh`.

Variable groups within the Lode regime gate on the kind enum exactly as RBRV's groups gate on `RBRV_VESSEL_MODE`. RBRV's existing ORIGIN/ANCHOR pattern (RBSRV lines 119-144) — author declares ORIGIN intent, foundry resolves and writes back ANCHOR — is the universal precedent the Lode model generalizes: Lode .env carries author intent only; enshrine writes the resolved capture identity to wherever consumers read it.

## Kinds

| Kind | Atomic / Compound | Upstream source |
|------|-------------------|-----------------|
| `base` | atomic | upstream OCI registry, consumed as FROM line |
| `tool` | atomic | upstream OCI registry, consumed at build time |
| `reliquary` | compound | date-cohort of `tool` sub-Lodes (replaces today's reliquary noun) |
| `wsl` | atomic | rootfs tar from Microsoft Store distro via `wsl --export` |
| `podvm-wsl` | compound | `quay.io/podman/machine-os-wsl` — platform fan-out, consumed by Windows podman |
| `podvm-native` | compound | `quay.io/podman/machine-os` — platform fan-out, consumed by macOS/Linux podman |

Compound Lodes hold a manifest of sub-Lode references; sub-Lodes are first-class addressable (each has its own {INSTANCE_NOUN}) but the compound is the typical reference target.

`wsl` and `podvm-wsl` both touch WSL but are entirely distinct: `wsl` is the Linux distro tarball that hosts everything on the Windows workload, `podvm-wsl` is the podman machine image that runs *inside* that distro. Names kept visibly different.

## Revised from prior paddock

- Three kinds (base / wsl / podman-vm) → six kinds. `tool` and `reliquary` enter scope (subsume today's reliquary-as-noun and the mirror verb); `podman-vm` splits into `podvm-wsl` and `podvm-native` to honor the structural asymmetry confirmed by `rbv_PodmanVM.sh`.
- Sibling specs per kind → unified Lode regime with kind-dispatched pipelines (mirrors RBRV's enum-gated-groups pattern).
- Sibling tabtargets `rbw-dEb/w/v` → leaning toward unified `rbw-dE` with kind argument; pending firm decision.
- Single-layer OCI universal wire format survives, but compound Lodes carry a manifest-of-references payload rather than a single payload byte-stream.

## Carried forward from prior paddock

- Caparison-w / garrison-w consequences for `wsl` kind: admin's WSL distribution disappears when the kind lands; the pre-existing revert pace in ₣A- targeting the WSL-stage DEV CACHE shortcut becomes obsolete and wants dropping or transferring when this heat commits to development.
- Refresh cadence variability (now per-kind, may diverge between `podvm-wsl` and `podvm-native`).
- "Mirror" verb retires entirely; absorbed into enshrine + `tool` / `reliquary` kinds.

## Discovery recipes — work to enumerate when paces approach

- **Verb redesign blast radius unaudited.** Discovery recipe: scan `Tools/rbk/vov_veiled/RBSA*.adoc` and `RBSI*.adoc` for verbs touching captured artifacts; for each, classify per-kind variance (unchanged / split / retired). Yoke is the headline because consumer-landing target differs per kind; other splits expected but uncatalogued. Keeping this as a recipe rather than an enumeration is deliberate — premature listing inflates the paddock and ages poorly.
- **Non-vessel consumer landing regime.** For `wsl` and `podvm-*` kinds, the resolved {INSTANCE_NOUN} has no vessel rbrv.env to yoke into. Decision deferred until the first non-vessel Lode pace mounts and forces the choice; candidates include station-regime extensions, a Lode-side current-resolution field, or a new host-config regime.
- **Spec letter allocation for the new Lode spec.** Consult CLAUDE.md Prefix Naming Discipline and Quoin Sub-Letter Discipline before minting; check terminal exclusivity against the existing RBS* tree.

## Vocabulary parked

- **Lode** chosen as the universal recipe noun. **Unction** held as the strongest rejected alternative (religious register, RBRU_ free, but verb-coupled meaning). **Oblate** rejected — RBRO_ collides with OAuth regime.
- **Reliquary** demoted from top-level noun to a kind enum value. Word survives; cult-vocabulary weight reduced.
- **{INSTANCE_NOUN}** (the specific-capture noun, parallel to Hallmark) is open. Letter-collision constraint with established project nouns leaves few unused letters; review needed before naming. Anchor was floated as the natural-preserving choice but flagged as uncertain.
- Three cohort options considered: α Reliquary-as-grouping abstraction, β Compound Lode, γ no grouping concept. γ rejected by date-cohort load-bearing test (vessels constrain Cloud Build toolsets by cohort); α rejected as wrong-direction (promotes a cult noun rather than reducing); β chosen.

## Heat nature

Planning/design heat — no paces yet. Paddock encodes the model and the recipes for follow-up work; specific verb-fate, landing-regime, and instance-noun decisions defer until paces commit. Heat silks (`rbk-29-win-enshrine-triple`) now mismatches scope and wants relabeling once the model is approved.

## References

- ₣AV `rbw-implement-gar-mirroring` (retired 260511) — predecessor heat
- RBSAE `ark_enshrine` — current narrow-scope spec; becomes the `base`-kind pipeline under unification
- RBSAS `ark_summon`, RBSDI `depot_inscribe`, RBSDY `director_yoke` — verbs within the blast radius
- RBSRV `RegimeVessel` (lines 119-144 in particular) — ORIGIN/ANCHOR precedent; enum-gated-variable-groups pattern is the structural model
- `Tools/rbk/vov_veiled/FUTURE/rbv_PodmanVM.sh` — concrete podman-vm pipeline; reveals fan-out structure and two-source quay split
- BUSJCW CaparisonWindows, BUSJGW GarrisonWsl, BUSJIW InvigilateWindows — `wsl` kind consequences on Windows workload
- CLAUDE.md "Prefix Naming Discipline" and "Quoin Sub-Letter Discipline" — guide spec letter allocation