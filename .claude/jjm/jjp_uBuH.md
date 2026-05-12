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