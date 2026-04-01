# Heat Trophy: rbk-egress-lockdown-research

**Firemark:** ₣Av
**Created:** 260315
**Retired:** 260331
**Status:** retired

## Paddock

## Character
Architectural research spike evolving into air-gapped build design.

## Research Findings (2026-03-15)

### GCB Pin Images: GAR Mirroring
All 7 RBRG-pinned tool images measured at ~1.97 GB compressed (linux/amd64). gcloud is 63% alone at 1234 MB. All actively used. GAR storage cost ~$0.20/month.

### GCB Worker Image Availability Under NO_PUBLIC_EGRESS
NO_PUBLIC_EGRESS blocks public internet, not Google APIs. Workers retain Private Google Access — `gcr.io/cloud-builders/*` images (gcloud, docker) remain pullable via internal routes. Docker Hub origins (binfmt, syft, alpine, oras, skopeo) are blocked. The pre-cached image set on private pool workers is undocumented; cache misses on Google images still resolve, cache misses on Docker Hub fail.

### APT/pip Under NO_PUBLIC_EGRESS
AR APT remote repos are Preview with bootstrap catch-22. Recommended: fat base image vessel pre-baking OS+Python deps. AR Python remote/virtual repo for app-specific pip tail only.

### SLSA Provenance: builds.create Achieves Build L3 (SETTLED, ₢AvAAB)
builds.create without git source produces GCB-signed v0.1 AND v1 provenance (Google docs incorrectly claim only v0.1 for non-trigger builds — empirically disproven on depot10030). GCB reports `slsa_build_level: 3`. slsa-verifier rejects this (requires `buildConfigSource` in `externalParameters`), but slsa-verifier is wrong: it conflates SLSA's Build track with the not-yet-existing Source track.

**SLSA v1.0 spec analysis**: Build L3 requires only platform hardening (isolation, ephemeral environments, unforgeable provenance). Source verification is deferred to a future Source track: "SLSA v1.0 does not address source threats." `resolvedDependencies` is explicitly optional. No requirement for `buildConfigSource` — that is a GCB convention consumed by slsa-verifier, not a SLSA requirement.

**slsa-verifier incompatibility**: The v1.0 handler unconditionally reads `externalParameters.buildConfigSource` and the CLI requires `--source-uri`. No flag to skip source verification. Issue #309 confirms maintainers consider this by-design. The tool carries forward v0.2 assumptions that v1.0 formally dropped.

**Previous rubric repo gave cosmetic source verification**: The old trigger path verified a commit in a generated staging repo (rubric), not actual source code. The new builds.create path is more honest — it doesn't pretend to verify source.

**Verification solution**: Direct DSSE envelope signature verification using jq + openssl (no Python, no slsa-verifier). Empirically proven on depot10030 `rbev-busybox@sha256:91114537...` (builds.create, arm64). All three signatures verified:

| Provenance | Key | Method | Result |
|---|---|---|---|
| v1.0 | `google-hosted-worker` (global) | DSSE PAE | Verified OK |
| v0.1 sig 1 | `provenanceSigner` (global) | DSSE PAE | Verified OK |
| v0.1 sig 2 | `builtByGCB` (us-central1) | Legacy raw | Verified OK |

**Public key access**: Attestor keys in `projects/verified-builder/` KMS are broadly accessible to any authenticated GCP identity via `cloudkms.cryptoKeyVersions.viewPublicKey`. slsa-verifier embeds them as 22 PEM files at compile time (documented as "temporary solution"). Google documents the manual gcloud + openssl verification process in their provenance docs.

The experiment's `direct_verify.py` checked claims without cryptographic verification — replaced by DSSE signature verification which proves GCB signed the provenance.

### Gotchas (PROVEN)
Platform must match builder (RBGC_BUILD_RUNNER_PLATFORM) even for FROM SCRATCH data-only images — first experiment commit failed without --platform flag, fix commit added it. Scratch image needs dummy CMD for docker create. Mason SA must be explicit in build JSON. GCB strict substitution matching.

## Vocabulary

### Reliquary
A co-versioned, datestamped, immutable snapshot of all GCB step/tool images, emplaced in GAR. Single concept — no separate term for the instance vs the type (tested against load-bearing principle: unlike vessel/consecration, there is no persistent parent entity that accumulates instances). Each reliquary is identified by a datestamped string. Required for all conjure builds. Does NOT include vessel base images — those are handled independently by enshrine/anchors (see Enshrine/Inscribe Separation below).

### Pouch
The build context packaged as a FROM SCRATCH OCI image and pushed to GAR. Required for all conjure builds — with triggers eliminated, builds.create has no other context delivery mechanism. The pouch IS how build context reaches GCB, regardless of egress mode. This is independent of the reliquary requirement: the reliquary provides tool images, the pouch provides the Dockerfile and build context files.

Tagged as `{vessel}:{consecration}-pouch`, making it a first-class ark artifact alongside `-image`, `-about`, `-vouch`. Cleaned up by abjure with the rest of the consecration's artifacts.

### Anchor
The human-readable, content-addressed GAR tag for an enshrined base image. Format: `{sanitized-origin}-{10-char-sha256}` (e.g., `python-3.11-slim-abc123def4`). Serves as both the GAR image tag and the value stored in the vessel regime. Immutable by convention.

## Design Decisions (2026-03-21)

### Reliquaries Are Required
All conjure builds require a reliquary. This is not just an air-gap feature — it is a universal robustness measure. The original pain (upstream registries rotating images, breaking digest pins) affects all builds. Required reliquaries solve this for everyone: tool versions are frozen in GAR, immune to upstream churn. The inscribe-before-conjure ceremony replaces the current pin-refresh ceremony and is actually less overhead: inscribe once (when you choose to update tools), conjure many times against a stable reliquary.

Simplification cascade: one stitch path, one verification path, one download method, one consecration format semantics. No conditional logic for "reliquary present vs absent."

### Air-Gap vs Open-Egress Is Network Policy, Not Image Policy
With required reliquaries and enshrined base images, every conjure pulls tool images and base images from GAR mirrors regardless of egress mode (tool images via reliquary, base images via anchors). The distinction between air-gap and open-egress is purely network enforcement: whether NO_PUBLIC_EGRESS is set on the private pool. GAR-mirroring is the default behavior at the image layer. Open-egress is a permissive network posture, not a different image sourcing strategy.

### Enshrine/Inscribe Separation (settled 2026-03-23)
Base images and tool images are handled by separate operations with different scoping, triggering, and tracking:

| | Enshrine (base images) | Inscribe (tool images) |
|---|---|---|
| **Scope** | Per-vessel | Fleet-wide |
| **Trigger** | Vessel author choice | Depot-level ceremony |
| **Tracking** | `RBRV_IMAGE_n_ANCHOR` in vessel regime | Datestamped reliquary namespace in GAR |
| **Co-versioning** | Independent per vessel | All tool images co-versioned in one pass |
| **GAR tag format** | `{sanitized-origin}-{10-char-sha256}` | `{reliquary-datestamp}/{image}` |

This separation is load-bearing: base images evolve per-vessel (a Python vessel updates its base independently of an Alpine vessel), while tool images must be co-versioned (all GCB steps in a build use the same reliquary). Conflating them would force fleet-wide updates when only one vessel's base image changes.

### Anchors Live in Vessel GAR Repository (settled 2026-03-23)
Enshrined base images are tagged in the same GAR repository as vessel consecration images (`RBRR_GAR_REPOSITORY`). No separate base-image namespace. Anchor tags (`python-3.11-slim-abc123def4`) and consecration tags (`c20260315-r20260323-image`) coexist in the same tag list, distinguishable by pattern. `rbi_list` already sees both; filtering by pattern is trivial.

Rationale: simplest thing that works. A dedicated base-image package would add a new namespace to create during depot initialization, a new target for IAM grants, and a new parameter for skopeo copy — all for a separation that carries no load. The tag patterns don't collide, and the same `rbi_show` infrastructure (which already handles multi-platform manifest lists) inspects both.

### Skopeo for Enshrine Copy (settled 2026-03-23)
Enshrine uses `skopeo copy --all` to mirror upstream base images to GAR. This preserves manifest lists and all per-platform manifests atomically. Single-platform images work identically — `--all` is a no-op on the platform dimension when only one platform exists.

**Tool choice**: skopeo over crane (not in toolchain, would add a dependency for no new capability) and oras (designed for OCI artifacts, not image mirroring — wrong tool class). Skopeo was already pinned in RBRG with "retained for potential future use"; enshrine is that use.

**Anchor digest source**: The sha256 digest used in anchor construction is the manifest list digest for multi-platform images, or the single manifest digest for single-platform images. This is what `skopeo inspect --raw` returns. One anchor per ORIGIN regardless of platform count.

**Multi-platform conjure**: No stitching required. The upstream manifest list arrives intact in GAR. When conjure's Dockerfile says `FROM ${RBF_IMAGE_1}`, the builder pulls the correct platform from the anchored manifest list automatically. The `RBRV_CONJURE_PLATFORMS` variable controls which platforms conjure builds for; the anchored base image just needs to contain those platforms (enshrine mirrors all of them).

### RBRV_IMAGE Variables (settled 2026-03-23)
Vessel regime variables declaring base image dependencies. Up to 3 per vessel (multi-stage Dockerfile support).

- `RBRV_IMAGE_[n]_ORIGIN` — upstream tag declaration (e.g., `python:3.11-slim`). Vessel author declares intent.
- `RBRV_IMAGE_[n]_ANCHOR` — GAR-mirrored, content-addressed reference. Written by enshrine.

**Anchor format**: `{sanitized-origin}-{10-char-sha256}` — e.g., `python-3.11-slim-abc123def4`

Construction (BCG parameter expansion, no external tools):
```
sanitized="${origin//[:\/]/-}"
short="${digest#sha256:}"
anchor="${sanitized}-${short:0:10}"
```

10 hex chars = 40 bits. Astronomically collision-safe at base-image scale.

**The anchor IS the GAR tag.** Enshrine pushes the mirrored image to GAR tagged with the anchor string. The regime variable holds the same string. One name serves as both human-readable identifier and pull reference. Immutable by convention (same as consecration tags).

**Enshrine writes the anchor.** Enshrine resolves ORIGIN → copies upstream to GAR via `skopeo copy --all` with anchor tag → writes ANCHOR back to the vessel regime. No separate manual step. The regime file diff shows exactly what changed and what it resolved to.

**Dockerfile usage:**
```dockerfile
ARG RBF_IMAGE_1
FROM ${RBF_IMAGE_1}
```
The Foundry substitutes the resolved reference via `--build-arg` at conjure time.

### RBRV_RELIQUARY (universal, settled 2026-03-25)
Required vessel regime variable for ALL vessel modes (conjure, bind, graft). All GCB submissions — conjure build, about, vouch, enshrine, mirror, inscribe — pull step images from the reliquary. This settles the bind/graft open question: universal reliquary is the right answer because bind/graft also submit GCB jobs for about+vouch metadata, and those jobs use tool images. Different vessels may reference different reliquaries — images evolve independently, Recipe Bottle is not opinionated.

### RBRG Replaced by Reliquary
RBRG (regime holding upstream tool image pins with freshness gates) is replaced by the reliquary. The < 1 day freshness gate that currently blocks inscribe is eliminated — you inscribe when you choose, not when a timer forces you. What remains of the upstream source information is a static manifest consumed by inscribe (a list of upstream image references to mirror), not a regime with validation and freshness enforcement.

### Inscribe Reclaimed
With GitLab rubric repo eliminated, inscribe is reclaimed as the reliquary generation operation. Reads the upstream tool image source manifest, pulls all tool images from upstream, pushes the complete set to a datestamped GAR namespace, and produces the reliquary identifier. Co-versioning is enforced by the operation — all tool images in one pass, one datestamp. Inscribe becomes a required step in depot initialization alongside governor/director/depot creation.

### Build = Conjure Execution
Build (conjure) does: load vessel regime, resolve base images from `RBRV_IMAGE_n_ANCHOR` (or pass ORIGIN through), assign consecration, push pouch to GAR, stitch JSON (single path — all step image references from reliquary), submit via builds.create, wait, vouch. Clean separation from both inscribe (tool images) and enshrine (base images) — no overlap.

### No More Triggers
Trigger path fully removed. Stitch generates clean builds.create JSON natively. No rubric repo substitutions generated, no post-hoc jq surgery. GitLab elimination is complete: no GitLab account, PAT, Secret Manager entries, CB v2 connection, triggers, or RBRR_RUBRIC_REPO_URL.

### Consecration Format Under builds.create
The dual-timestamp consecration format `{mode}{T1}-r{T2}` gains universal meaning. T1 is always the reliquary datestamp (when the build toolchain was inscribed). T2 is always the actual build time (when conjure ran). The consecration encodes both WHAT tools built the image and WHEN. No conditional semantics.

### Consecration Minted Locally
With builds.create replacing triggers.run, the Director (not GCB) is the build initiator. Consecration timestamps assigned on the Director's workstation. Formalized as input to stitch, not discovered from build step output. Director knows the complete artifact tag set before submission — naming authority and verification authority are the same entity; the cloud is just labor.

### Explicit Verify Method
Build JSON includes explicit `_RBGV_VERIFY_METHOD` substitution declaring verification intent. Vouch step executes the declared method rather than inferring from empty/present source URI.

### Single-Platform Conjure Valid
The multi-platform assertion in rbgjv02 (line 69-71) is a simplification artifact from ₢AtAAD, not an invariant. Single-platform conjure vessels are legitimate. The vouch verification loop iterates once.

### Director IAM Surface Changes
With triggers and rubric repo eliminated, the Director's IAM grants need reassessment:
- **Remove**: Secret Manager access for GitLab PAT (3 secrets gone), build bucket objectCreator/objectViewer (source upload no longer needed — pouch goes via docker push to GAR)
- **Retain**: cloudbuild.builds.editor (builds.create submission), cloudbuild.workerPoolUser (private pool), artifactregistry.repoAdmin (image management), iam.serviceAccountUser on Mason (actAs for build execution)
- **Assess**: Does docker push for pouch delivery use the Director's existing GAR repoAdmin grant, or does it need a distinct permission? The Director already has repoAdmin for image management — pouch push may be covered.

This is a security surface reduction: fewer secrets, fewer cross-service grants, simpler audit.

### Vouch Verification Architecture (₢AvAAB)
slsa-verifier is dropped from the verification pipeline. The vouch GCB step verifies provenance via DSSE envelope signature verification using standard tools:

1. **Fetch provenance**: Container Analysis REST API → DSSE envelope (payload + signatures)
2. **Extract components**: jq extracts payload (base64), signature (base64), keyid
3. **Decode**: base64 decode payload and signature (standard base64, not url-safe)
4. **Reconstruct PAE**: `printf "DSSEv1 28 application/vnd.in-toto+json %d " $LEN; cat payload` — binary-safe shell construction
5. **Verify signature**: `openssl dgst -sha256 -verify key.pub -signature sig.bin pae.bin`
6. **Check provenance fields**: jq reads builder.id, buildType, invocationId, subject digest from now-trusted payload

**Public key strategy**: Embed attestor PEM keys in the reliquary at inscribe time (fetched from `projects/verified-builder/` KMS). No runtime KMS dependency. Air-gap compatible. Keys change rarely — analogous to slsa-verifier's embedded approach.

**Three keys to embed per region**:
- `google-hosted-worker` (global) — v1.0 DSSE PAE
- `provenanceSigner` (global) — v0.1 DSSE PAE
- `builtByGCB` (regional) — v0.1 legacy raw payload

**Dependencies**: jq (~1.5 MB static binary) + openssl + base64 + printf. Alpine image with `apk add jq openssl`. No Python, no slsa-verifier, no cosign.

**Binary Authorization**: BinAuth SLSA check provides deploy-time enforcement independent of vouch. It trusts GCB's `slsa_build_level: 3` directly. Complementary, not competing.

### RBSHR Update Required
The Horizon Roadmap egress lockdown entry (RBSHR line 87-93) describes the old architecture. Update to reflect reliquary/pouch/builds.create architecture, or graduate the item out of RBSHR entirely since it is now active heat work.

## Implementation Findings (2026-03-27, ₢AvAAH)

### Foundry Pool Routing Fix
The dual-pool implementation (₢AvAAL) put `RBRV_EGRESS_MODE → ZRBF_CONJURE_POOL` routing in `zrbf_kindle`, which runs for all foundry commands. This broke inscribe (fleet-wide, no vessel context). Fixed: removed from kindle, compute pool locally in conjure stitch where vessel is loaded. Enshrine was also using `ZRBF_CONJURE_POOL` — fixed to always use `RBDC_POOL_TETHER` (enshrine pulls from upstream, needs internet).

### jq Airgap Incompatibility
All three jq acquisition paths in GCB step scripts fail on airgap:
1. `apt-get install jq` (rbgja01, gcloud image) — blocked by NO_PUBLIC_EGRESS
2. `apk add jq` (rbgja03, alpine image) — blocked by NO_PUBLIC_EGRESS
3. `wget` static binary from github.com (rbgjv01) — blocked by NO_PUBLIC_EGRESS

No reliquary image ships jq. Solution: replace jq with Python 3 (`json` module), which is preinstalled in the gcloud reliquary image. See ₢AvAAW for implementation.

## Open Questions

### ~~SLSA Level 3 with builds.create (₢AvAAB)~~ SETTLED
Yes. builds.create achieves Build L3 by spec and by GCB's own assessment. slsa-verifier is incompatible (conflates Build and Source tracks) and is dropped. Vouch step uses DSSE signature verification: jq + openssl against `verified-builder` KMS public keys. See Research Findings section for full analysis.

### ~~Bind/Graft Vouch and About Step Images~~ SETTLED
Yes. RBRV_RELIQUARY is required for all vessel modes. Bind/graft submit GCB jobs for about+vouch metadata, and those jobs use tool images (gcloud, alpine, docker). Universal reliquary is the consistent answer. All 8 vessels updated with r260324201411.

### Oras Eliminated from Reliquary (settled 2026-03-25)
Oras was never used as a GCB step image — it was a CLI tool for OCI artifact operations, not a build step container. The reliquary mirrors only images that appear as GCB step `name` fields: gcloud, docker, alpine, skopeo, syft, binfmt. Removing oras reduces reliquary size and eliminates a stale-pin maintenance burden for an unused image.

### GCB Script Field Migration (settled 2026-03-25)
All GCB step assembly migrated from `entrypoint` + `args: ["-lc", script]` to `script` field with shebang prefix. Motivation: about step scripts (~8KB) exceed GCB's 10K per-arg limit when inlined as args[1]. The `script` field is designed for multi-line scripts with a much higher limit.

Key facts:
- `script` field ignores `entrypoint` — GCB writes script to a file and executes it
- Shell selection via shebang: `#!/bin/bash` or `#!/bin/sh` prepended based on step def tuple's entrypoint field
- `script` field does NOT support direct GCB substitution expansion — `$$` escaping is irrelevant (and harmful: `$$` in shell is PID). The entire escape/un-escape dance is eliminated.
- `automapSubstitutions: true` added to all 7 build-level options blocks — maps all GCB substitutions to environment variables, so `${_RBGY_FOO}` works as a shell env var reference
- All 6 step-assembly sites migrated: stitch (conjure), inscribe, enshrine, mirror, about helper, vouch helper
- Net simplification: ~18 lines of escaping code removed, substitution references work naturally as shell env vars

## Paces

### enshrine-gcb-correction (₢AvAAO) [complete]

**[260324-0759] complete**

## Character
Architectural correction — enshrine must run as a GCB job, not Director-local skopeo.

## Goal
Rewrite enshrine to submit a Cloud Build job (like vouch does) instead of running skopeo on the Director's workstation. The GCB step does inspect + copy, returns anchor strings via buildStepOutputs, Director writes them to rbrv.env.

## Steps
1. Create GCB step script `rbgje/rbgje01-enshrine-copy.sh`: for each slot, skopeo inspect --raw (digest), construct anchor, skopeo copy --all to GAR. Auth via metadata server token. Write JSON anchor results to /builder/outputs/output.
2. Rewrite `rbf_enshrine` in Foundry: load vessel, assemble enshrine step, compose builds.create JSON with _RBGE_* substitutions (GAR coords + ORIGIN values), submit, wait, extract anchor JSON from buildStepOutputs.
3. Delete `zrbf_enshrine_slot` — logic now lives in GCB step script.
4. Parse anchor results, write each RBRV_IMAGE_n_ANCHOR back to rbrv.env (regime writeback stays local).
5. Update RBSAE spec to describe GCB submission pattern.
6. Remove `command -v skopeo` local requirement check.

## Verification
- Enshrine submits GCB job and returns anchor strings
- No local skopeo dependency
- Anchor writeback to rbrv.env still works
- Re-run produces same anchor (idempotent)

**[260324-0741] rough**

## Character
Architectural correction — enshrine must run as a GCB job, not Director-local skopeo.

## Goal
Rewrite enshrine to submit a Cloud Build job (like vouch does) instead of running skopeo on the Director's workstation. The GCB step does inspect + copy, returns anchor strings via buildStepOutputs, Director writes them to rbrv.env.

## Steps
1. Create GCB step script `rbgje/rbgje01-enshrine-copy.sh`: for each slot, skopeo inspect --raw (digest), construct anchor, skopeo copy --all to GAR. Auth via metadata server token. Write JSON anchor results to /builder/outputs/output.
2. Rewrite `rbf_enshrine` in Foundry: load vessel, assemble enshrine step, compose builds.create JSON with _RBGE_* substitutions (GAR coords + ORIGIN values), submit, wait, extract anchor JSON from buildStepOutputs.
3. Delete `zrbf_enshrine_slot` — logic now lives in GCB step script.
4. Parse anchor results, write each RBRV_IMAGE_n_ANCHOR back to rbrv.env (regime writeback stays local).
5. Update RBSAE spec to describe GCB submission pattern.
6. Remove `command -v skopeo` local requirement check.

## Verification
- Enshrine submits GCB job and returns anchor strings
- No local skopeo dependency
- Anchor writeback to rbrv.env still works
- Re-run produces same anchor (idempotent)

### bcg-sed-to-parameter-expansion (₢AvAAP) [complete]

**[260324-0840] complete**

## Character
Mechanical cleanup — apply BCG-compliant bash-native patterns.

## Goal
Replace remaining `sed` dollar-escaping in Foundry GCB submission functions with bash parameter expansion, matching the pattern established in enshrine.

## Scope
Four functions use the `sed 's/\$/\$\$/g; s/\$\${_PREFIX_/${_PREFIX_/g'` pattern:
- `zrbf_stitch_build_json` (conjure, line ~407, `_RBGY_`)
- `zrbf_mirror_submit` (mirror, line ~1473, `_RBGA_`)
- `zrbf_about_submit` (about, line ~2811, `_RBGA_`)
- `zrbf_vouch_submit` (vouch, line ~3181, `_RBGV_`)

Also `sed` at line ~452 for about step post-processing and line ~4169 for recipe display.

## Pattern
Replace:
```bash
printf '%s' "${z_body}" | sed 's/\$/\$\$/g; s/\$\${_PREFIX_/${_PREFIX_/g' > "${z_escaped_file}"
```
With:
```bash
z_body="${z_body//\$/\$\$}"
z_body="${z_body//\$\${_PREFIX_/\${_PREFIX_}"
printf '%s' "${z_body}" > "${z_escaped_file}"
```

## Verification
- No `sed` in dollar-escaping paths
- Existing test fixtures pass unchanged

**[260324-0755] rough**

## Character
Mechanical cleanup — apply BCG-compliant bash-native patterns.

## Goal
Replace remaining `sed` dollar-escaping in Foundry GCB submission functions with bash parameter expansion, matching the pattern established in enshrine.

## Scope
Four functions use the `sed 's/\$/\$\$/g; s/\$\${_PREFIX_/${_PREFIX_/g'` pattern:
- `zrbf_stitch_build_json` (conjure, line ~407, `_RBGY_`)
- `zrbf_mirror_submit` (mirror, line ~1473, `_RBGA_`)
- `zrbf_about_submit` (about, line ~2811, `_RBGA_`)
- `zrbf_vouch_submit` (vouch, line ~3181, `_RBGV_`)

Also `sed` at line ~452 for about step post-processing and line ~4169 for recipe display.

## Pattern
Replace:
```bash
printf '%s' "${z_body}" | sed 's/\$/\$\$/g; s/\$\${_PREFIX_/${_PREFIX_/g' > "${z_escaped_file}"
```
With:
```bash
z_body="${z_body//\$/\$\$}"
z_body="${z_body//\$\${_PREFIX_/\${_PREFIX_}"
printf '%s' "${z_body}" > "${z_escaped_file}"
```

## Verification
- No `sed` in dollar-escaping paths
- Existing test fixtures pass unchanged

### build-poll-label-parameter (₢AvAAR) [complete]

**[260324-0847] complete**

## Character
Small UX improvement — mechanical.

## Goal
Add a label parameter to `zrbf_wait_build_completion` so poll status lines identify the build type.

## Steps
1. Change signature: `zrbf_wait_build_completion max_polls label`
2. Use label in the `buc_info` line: `"${z_label}: ${z_status} (poll ${z_polls}/${z_max_polls})"`
3. Update all call sites to pass their identity:
   - `rbf_build` (conjure): `"Conjure"`
   - `zrbf_mirror_submit`: `"Mirror"`
   - `zrbf_about_submit`: `"About"`
   - `zrbf_vouch_submit`: `"Vouch"`
   - `zrbf_enshrine_submit`: `"Enshrine"`
   - `zrbf_graft_submit` (if exists): `"Graft"`
4. Also update the success/failure messages to use the label.

## Verification
- regime-validation passes
- Spot-check one build to confirm label appears in output

**[260324-0839] rough**

## Character
Small UX improvement — mechanical.

## Goal
Add a label parameter to `zrbf_wait_build_completion` so poll status lines identify the build type.

## Steps
1. Change signature: `zrbf_wait_build_completion max_polls label`
2. Use label in the `buc_info` line: `"${z_label}: ${z_status} (poll ${z_polls}/${z_max_polls})"`
3. Update all call sites to pass their identity:
   - `rbf_build` (conjure): `"Conjure"`
   - `zrbf_mirror_submit`: `"Mirror"`
   - `zrbf_about_submit`: `"About"`
   - `zrbf_vouch_submit`: `"Vouch"`
   - `zrbf_enshrine_submit`: `"Enshrine"`
   - `zrbf_graft_submit` (if exists): `"Graft"`
4. Also update the success/failure messages to use the label.

## Verification
- regime-validation passes
- Spot-check one build to confirm label appears in output

### slsa-provenance-builds-create-experiment (₢AvAAA) [complete]

**[260321-1146] complete**

## Character
Deliberate and cautious. The experiment succeeded but ran outside normal step discipline. This pace restores safety before planning forward.

## Concern
The experiment modified shared production code (rbf_Foundry.sh, rbgjv01, rbgjv02) under iterate-fast pressure. These changes affect ALL conjure vessels, not just the test target. The default posture must be: **revert experiment code, preserve findings as documentation, reimplement under proper discipline.**

The experiment answered WHAT to build. The experiment code is NOT the foundation to build on.

## Findings to preserve (proven empirically)
- `builds.create` without git source produces GCB-signed v1 provenance
- builder.id: `GoogleHostedWorker`, buildType: `google-worker/v1`
- resolvedDependencies: step images only (no git source in materials)
- `slsa-verifier` rejects this (missing buildConfigSource) — direct verification needed
- Context image delivery: FROM SCRATCH OCI to GAR, extract in step 0
- Gotchas: platform must match builder (RBGC_BUILD_RUNNER_PLATFORM), scratch image needs dummy CMD for docker create, mason SA must be explicit in build JSON, GCB strict substitution matching
- Full build+vouch pipeline works end-to-end (3 platforms)

## Task

1. **Revert experiment commits**: Return rbf_Foundry.sh, rbgjv01, rbgjv02 to their pre-experiment state. The trigger-based conjure path must be restored as the working production path.

2. **Capture findings**: Update the heat paddock with the empirical results above so no knowledge is lost.

3. **Rubric repo dependency inventory**: Enumerate every touchpoint — code, regime vars, secrets, IAM grants, specs, tabtargets — that references the rubric repo or GitLab. This is the demolition manifest for future work.

4. **Spec impact assessment**: Which specs need updates? RBSAC, RBSAV, RBSRI, RBSTB, RBSCB, RBSCTD at minimum.

5. **Pace proposals**: Draft paces for a follow-on heat that executes the eviction cleanly, with proper spec-first discipline.

## Not in scope
- Implementing the builds.create path properly (that's a future pace)
- Running more GCB builds
- Changing regime configuration

**[260317-2147] rough**

## Character
Deliberate and cautious. The experiment succeeded but ran outside normal step discipline. This pace restores safety before planning forward.

## Concern
The experiment modified shared production code (rbf_Foundry.sh, rbgjv01, rbgjv02) under iterate-fast pressure. These changes affect ALL conjure vessels, not just the test target. The default posture must be: **revert experiment code, preserve findings as documentation, reimplement under proper discipline.**

The experiment answered WHAT to build. The experiment code is NOT the foundation to build on.

## Findings to preserve (proven empirically)
- `builds.create` without git source produces GCB-signed v1 provenance
- builder.id: `GoogleHostedWorker`, buildType: `google-worker/v1`
- resolvedDependencies: step images only (no git source in materials)
- `slsa-verifier` rejects this (missing buildConfigSource) — direct verification needed
- Context image delivery: FROM SCRATCH OCI to GAR, extract in step 0
- Gotchas: platform must match builder (RBGC_BUILD_RUNNER_PLATFORM), scratch image needs dummy CMD for docker create, mason SA must be explicit in build JSON, GCB strict substitution matching
- Full build+vouch pipeline works end-to-end (3 platforms)

## Task

1. **Revert experiment commits**: Return rbf_Foundry.sh, rbgjv01, rbgjv02 to their pre-experiment state. The trigger-based conjure path must be restored as the working production path.

2. **Capture findings**: Update the heat paddock with the empirical results above so no knowledge is lost.

3. **Rubric repo dependency inventory**: Enumerate every touchpoint — code, regime vars, secrets, IAM grants, specs, tabtargets — that references the rubric repo or GitLab. This is the demolition manifest for future work.

4. **Spec impact assessment**: Which specs need updates? RBSAC, RBSAV, RBSRI, RBSTB, RBSCB, RBSCTD at minimum.

5. **Pace proposals**: Draft paces for a follow-on heat that executes the eviction cleanly, with proper spec-first discipline.

## Not in scope
- Implementing the builds.create path properly (that's a future pace)
- Running more GCB builds
- Changing regime configuration

**[260317-2145] rough**

## Character
Deliberate and reflective. The hacking is done; now we assess what we built, what we learned, and what the clean path forward looks like.

## Situation
Experiment succeeded beyond original scope:
- `builds.create` produces GCB-signed v1 provenance (builder.id: GoogleHostedWorker)
- No git source in provenance — resolvedDependencies are step images only
- `slsa-verifier` doesn't support google-worker/v1 buildType (missing buildConfigSource)
- Direct provenance verification (builder identity + dependency check) works and is arguably more honest
- Full build+vouch pipeline passes end-to-end for rbev-busybox (3 platforms)

Code changes made during experiment (2 commits on ₢AvAAA):
- rbf_Foundry.sh: `rbf_build()` rewired to builds.create, `zrbf_push_build_context()` added, post-processing removes unused subs, adds mason SA, replaces _RBGA_BUILD_ID with $BUILD_ID built-in, context image uses RBGC_BUILD_RUNNER_PLATFORM
- rbgjv01: writes direct_verify.py to /workspace
- rbgjv02: compacted to fit 10K arg limit, provenance-direct branch when source URI empty
- rbf_Foundry.sh vouch: z_source_uri cleared for conjure (triggers direct path)

## Task
Produce a consolidated assessment that enables clean execution:

1. **Diff inventory**: Review the 2 experiment commits. For each change, classify as: keep (production-ready), reshape (right idea, needs cleanup), or revert (experiment-only hack)

2. **Rubric repo dependency inventory**: Enumerate every touchpoint — code functions, regime vars, secrets, IAM grants, specs, tabtargets — that references the rubric repo or GitLab. This is the demolition manifest.

3. **Spec impact assessment**: Which specs need updates? At minimum: RBSAC (conjure), RBSAV (vouch), RBSRI (inscribe — to be retired), RBSTB (trigger — to be retired), RBSCB (posture), RBSCTD (trigger dispatch). What new spec content is needed for the builds.create path?

4. **Paddock update**: Curry findings into the heat paddock so future paces have context.

5. **Pace proposals**: Draft 3-5 paces for a follow-on heat (or continuation of ₣Av) that execute the eviction cleanly.

## Not in scope
- Actually removing code or changing specs (that's for the proposed paces)
- Running more GCB builds
- Changing regime configuration

**[260317-2003] rough**

## Character
Empirical — iterative hacking against live GCP with fast feedback loops. The research is done; this is hands-on verification.

## Thesis
SLSA Build Level 3 attests that an image was built on a hardened builder (GCB). It does NOT require source traceability (v1.0 spec explicitly defers Source track). Therefore `builds.create` without a git source should produce valid SLSA provenance on output images, enabling elimination of the rubric repo and GitLab dependency.

## Evidence gathered
- SLSA v1.0 removed source requirements from Build track: "Source aspects were removed to focus on the Build track"
- GCB generates v0.1 provenance for `builds.create` (v1.0 requires triggers — but current conjure already uses v0.1)
- Conjure provenance is driven by `images:` field + `requestedVerifyOption: "VERIFIED"` in Build resource
- Bind images have NO provenance because mirror build lacks both fields — not because of builds.create
- Current slsa-verifier invocation uses `--source-uri` matching against provenance materials — may need adaptation

## Experiment

### Step 1: Package build context as OCI image
- On host: `FROM SCRATCH`, add Dockerfile + build context for rbev-busybox
- Include the stitched cloudbuild.json (or generate it fresh)
- `docker push` to GAR (reuse graft push pattern)

### Step 2: Submit builds.create
- First step: pull context image from GAR, extract to /workspace
- Remaining steps: existing rbgjb01-07 + rbgja01-04 (same scripts)
- Build resource includes `images:` field + `requestedVerifyOption: "VERIFIED"`
- NO `source` field (no repoSource, no storageSource)
- Use existing mason SA, private pool, timeout

### Step 3: Check provenance
- `gcloud artifacts docker images describe --show-provenance` on per-platform output
- Record: `slsa_build_level`, `materials` content, `builder.id`, `sourceProvenance`
- Run `slsa-verifier verify-image` — note what `--source-uri` value (if any) satisfies it

### Step 4: Assess
- If Level 3 + slsa-verifier passes: rubric repo elimination is clear
- If Level 3 but slsa-verifier needs adaptation: document what vouch changes are needed
- If no provenance: document why, assess workarounds (StorageSource fallback)

## Scope
One vessel only: rbev-busybox (multi-platform: amd64, arm64, armv7). Manual curl/gcloud commands for submission — no production code changes. Iterate as needed.

## Not in scope
- Full integration into rbf_create pipeline
- Vouch adaptation
- Rubric repo removal

**[260315-1146] rough**

Test whether builds.create API (no trigger, no repo) retains SLSA Level 3 provenance on built images.

Experiment:
1. Take an existing vessel that currently conjures successfully via triggers.run with SLSA Level 3
2. Submit the same cloudbuild.json via builds.create REST API (curl) with source staged differently — either inline or pulled from GAR in first step
3. Check resulting image provenance with gcloud artifacts docker images describe --show-provenance
4. Compare provenance fields: buildType, source references, slsa_build_level
5. Run slsa-verifier verify-image against both images and compare verdicts

If Level 3 survives: path is clear to eliminate rubric repo and GitLab entirely.
If Level 3 is lost: document what level we get, assess whether the tradeoff is acceptable, and whether trigger-based builds can work with GAR-staged source instead of git repo source.

### slsa-level3-builds-create-verification (₢AvAAB) [complete]

**[260321-1304] complete**

## Character
Research-first with potential implementation tail. Heavy web research, careful reading of SLSA specifications, and empirical verification against our existing build artifacts.

## Goal
Definitively determine whether `builds.create` without a git source can achieve SLSA Build Level 3 provenance, and if so, make `slsa-verifier` work with it.

## Available Empirical Artifacts
The experiment produced three conjure consecrations on rbev-busybox (March 17 2026):
- `c260317210526-r260318040810` (3-platform)
- `c260317210019-r260318040308` (3-platform)
- `c260317205112-r260318035317` (3-platform)

Provenance is queryable via `gcloud artifacts docker images describe --show-provenance` against per-platform digests in GAR.

**Important**: `gcloud` is available on the workstation for diagnostics and provenance inspection during this research pace. It is NEVER used in Recipe Bottle bash source code — all GCP interactions in production code use direct REST API calls via curl.

## Research Phase

1. **SLSA v1.0 Build Level 3 specification**: Read the authoritative SLSA spec (not summaries). What exactly does Level 3 require? Is `buildConfigSource` (or equivalent verified-source property) a hard requirement, or is it one way to satisfy a broader requirement?

2. **GCB provenance for builds.create**: Examine provenance from the experiment consecrations above. What predicate version and fields does GCB emit for builds.create vs triggers.run? Diff the provenance structures.

3. **slsa-verifier rejection analysis**: Why exactly does slsa-verifier reject builds.create provenance? Is it checking a SLSA requirement, or is it enforcing an assumption about GCB trigger-based builds that isn't in the SLSA spec itself? Read slsa-verifier source if needed.

4. **GCB documentation**: Does Google document the SLSA level achieved by builds.create vs triggers.run? Any Google guidance on direct-submission provenance?

## Decision Gate

If Level 3 IS achievable with builds.create:
- Work out how to configure slsa-verifier (flags, options, custom policies) to accept the provenance
- Or identify an alternative formal verifier
- Validate against our existing build artifacts

If Level 3 is NOT achievable:
- Document exactly which SLSA requirement fails and why
- Characterize what level IS achieved (likely Level 2)
- Draft the architectural rationale for accepting this level
- Assess whether the rubric repo's Level 3 was substantive or cosmetic (generated staging repo)

## Not in scope
- Changing the builds.create submission path
- Implementing reliquary/inscribe
- Any production code changes

**[260321-1145] rough**

## Character
Research-first with potential implementation tail. Heavy web research, careful reading of SLSA specifications, and empirical verification against our existing build artifacts.

## Goal
Definitively determine whether `builds.create` without a git source can achieve SLSA Build Level 3 provenance, and if so, make `slsa-verifier` work with it.

## Available Empirical Artifacts
The experiment produced three conjure consecrations on rbev-busybox (March 17 2026):
- `c260317210526-r260318040810` (3-platform)
- `c260317210019-r260318040308` (3-platform)
- `c260317205112-r260318035317` (3-platform)

Provenance is queryable via `gcloud artifacts docker images describe --show-provenance` against per-platform digests in GAR.

**Important**: `gcloud` is available on the workstation for diagnostics and provenance inspection during this research pace. It is NEVER used in Recipe Bottle bash source code — all GCP interactions in production code use direct REST API calls via curl.

## Research Phase

1. **SLSA v1.0 Build Level 3 specification**: Read the authoritative SLSA spec (not summaries). What exactly does Level 3 require? Is `buildConfigSource` (or equivalent verified-source property) a hard requirement, or is it one way to satisfy a broader requirement?

2. **GCB provenance for builds.create**: Examine provenance from the experiment consecrations above. What predicate version and fields does GCB emit for builds.create vs triggers.run? Diff the provenance structures.

3. **slsa-verifier rejection analysis**: Why exactly does slsa-verifier reject builds.create provenance? Is it checking a SLSA requirement, or is it enforcing an assumption about GCB trigger-based builds that isn't in the SLSA spec itself? Read slsa-verifier source if needed.

4. **GCB documentation**: Does Google document the SLSA level achieved by builds.create vs triggers.run? Any Google guidance on direct-submission provenance?

## Decision Gate

If Level 3 IS achievable with builds.create:
- Work out how to configure slsa-verifier (flags, options, custom policies) to accept the provenance
- Or identify an alternative formal verifier
- Validate against our existing build artifacts

If Level 3 is NOT achievable:
- Document exactly which SLSA requirement fails and why
- Characterize what level IS achieved (likely Level 2)
- Draft the architectural rationale for accepting this level
- Assess whether the rubric repo's Level 3 was substantive or cosmetic (generated staging repo)

## Not in scope
- Changing the builds.create submission path
- Implementing reliquary/inscribe
- Any production code changes

**[260321-1123] rough**

## Character
Research-first with potential implementation tail. Heavy web research, careful reading of SLSA specifications, and empirical verification against our existing build artifacts.

## Goal
Definitively determine whether `builds.create` without a git source can achieve SLSA Build Level 3 provenance, and if so, make `slsa-verifier` work with it.

## Research Phase

1. **SLSA v1.0 Build Level 3 specification**: Read the authoritative SLSA spec (not summaries). What exactly does Level 3 require? Is `buildConfigSource` (or equivalent verified-source property) a hard requirement, or is it one way to satisfy a broader requirement?

2. **GCB provenance for builds.create**: Examine our existing provenance artifacts from the experiment (build IDs `04f73057`, `3cd95eac` era — check paddock for actual GCB build IDs `16d3b60f`, `fc36b970`, `9180c42a`, `2e172ce0`). What predicate version and fields does GCB emit for builds.create vs triggers.run? Diff the provenance structures.

3. **slsa-verifier rejection analysis**: Why exactly does slsa-verifier reject builds.create provenance? Is it checking a SLSA requirement, or is it enforcing an assumption about GCB trigger-based builds that isn't in the SLSA spec itself? Read slsa-verifier source if needed.

4. **GCB documentation**: Does Google document the SLSA level achieved by builds.create vs triggers.run? Any Google guidance on direct-submission provenance?

## Decision Gate

If Level 3 IS achievable with builds.create:
- Work out how to configure slsa-verifier (flags, options, custom policies) to accept the provenance
- Or identify an alternative formal verifier
- Validate against our existing build artifacts

If Level 3 is NOT achievable:
- Document exactly which SLSA requirement fails and why
- Characterize what level IS achieved (likely Level 2)
- Draft the architectural rationale for accepting this level
- Assess whether the rubric repo's Level 3 was substantive or cosmetic (generated staging repo)

## Not in scope
- Changing the builds.create submission path
- Implementing reliquary/inscribe
- Any production code changes

### revert-experiment-restore-trigger-path (₢AvAAC) [abandoned]

**[260321-1315] abandoned**

## Character
Mechanical and careful. Reverting shared production code that was modified under experiment pressure.

## Task
Revert rbf_Foundry.sh, rbgjv01-download-verifier.sh, and rbgjv02-verify-provenance.sh to their pre-experiment state (before ₢AvAAA commits 3cd95eac and 04f73057). The trigger-based conjure path must be restored as the working production path until the new architecture is implemented under proper discipline.

## Rationale
The experiment answered WHAT to build. The experiment code is NOT the foundation to build on. Design decisions are captured in the heat paddock; implementation will follow in properly scoped paces.

## Not in scope
- Implementing any of the new design (pouch, reliquary, builds.create)
- Changing any code beyond the three experiment-modified files

**[260321-1133] rough**

## Character
Mechanical and careful. Reverting shared production code that was modified under experiment pressure.

## Task
Revert rbf_Foundry.sh, rbgjv01-download-verifier.sh, and rbgjv02-verify-provenance.sh to their pre-experiment state (before ₢AvAAA commits 3cd95eac and 04f73057). The trigger-based conjure path must be restored as the working production path until the new architecture is implemented under proper discipline.

## Rationale
The experiment answered WHAT to build. The experiment code is NOT the foundation to build on. Design decisions are captured in the heat paddock; implementation will follow in properly scoped paces.

## Not in scope
- Implementing any of the new design (pouch, reliquary, builds.create)
- Changing any code beyond the three experiment-modified files

### gitlab-elimination-pouch-builds-create (₢AvAAD) [complete]

**[260321-1501] complete**

## Character
Architectural transition with broad file surface. Replaces trigger-based conjure with builds.create + pouch. Requires judgment on what to preserve vs rebuild, and discipline to stop at the baby step boundary.

## Goal
Eliminate GitLab dependency entirely. Replace trigger-based build dispatch with builds.create API + pouch context delivery. Adapt vouch for builds.create provenance (DSSE jq+openssl). Keep RBRG pin mechanism for step images (no reliquary yet). Result: a working, releasable conjure pipeline with no GitLab, no triggers, no rubric repo.

## Scope

### In scope
- **Stitch**: Rewrite `zrbf_stitch_build_json` for native builds.create JSON. No rubric substitutions, no post-hoc jq surgery. Pouch reference as first build step (extract context from GAR image). Step image refs from existing RBRG pins.
- **Pouch**: New function to package build context as FROM SCRATCH OCI image, push to GAR. Tagged as `{vessel}:{inscribe_timestamp}-context` (or similar). Created as part of conjure flow.
- **Conjure**: Replace triggers.run dispatch with builds.create REST API call + wait. Consecration minted locally (Director assigns timestamps). No trigger-ensure, no rubric commit, no dispatch indirection.
- **Vouch**: Replace slsa-verifier with DSSE envelope signature verification (jq + base64 + openssl). Verify against embedded or fetched verified-builder KMS public keys. Proven approach from ₢AvAAB research.
- **Regime cleanup**: Remove RBRR_RUBRIC_REPO_URL, GitLab PAT secrets references, CB v2 connection references from regime and onboarding flows.
- **Absorbs ₢AvAAC**: Experiment modifications to rbf_Foundry.sh, rbgjv01, rbgjv02 are overwritten by new implementation (no separate revert step).

### Deferred (not in scope)
- Reliquary (co-versioned tool image snapshots)
- RBRG replacement — existing pin mechanism stays
- RBRV_BASE_IMAGE_* resolution against reliquary
- Air-gap / NO_PUBLIC_EGRESS enforcement
- Inscribe as reliquary generation
- Director IAM grant cleanup (separate pace)

## Key files
- `Tools/rbk/rbf_Foundry.sh` — stitch, conjure, pouch (major rewrite)
- `Tools/rbk/rbgjv/rbgjv01-download-verifier.sh` — replace slsa-verifier download with DSSE setup
- `Tools/rbk/rbgjv/rbgjv02-verify-provenance.sh` — replace slsa-verifier/direct_verify with DSSE chain
- `Tools/rbk/rbrr_regime.sh` / `.rbk/rbrr.env` — remove GitLab regime variables
- Tabtargets: rbw-DI (inscribe), rbw-DC (conjure) may need adaptation

## Verification
- End-to-end: conjure a vessel on depot10030 via the new pipeline
- Vouch: DSSE signature verification passes on the resulting image
- No GitLab references remain in active code paths
- Existing bind and graft modes unaffected

**[260321-1315] rough**

## Character
Architectural transition with broad file surface. Replaces trigger-based conjure with builds.create + pouch. Requires judgment on what to preserve vs rebuild, and discipline to stop at the baby step boundary.

## Goal
Eliminate GitLab dependency entirely. Replace trigger-based build dispatch with builds.create API + pouch context delivery. Adapt vouch for builds.create provenance (DSSE jq+openssl). Keep RBRG pin mechanism for step images (no reliquary yet). Result: a working, releasable conjure pipeline with no GitLab, no triggers, no rubric repo.

## Scope

### In scope
- **Stitch**: Rewrite `zrbf_stitch_build_json` for native builds.create JSON. No rubric substitutions, no post-hoc jq surgery. Pouch reference as first build step (extract context from GAR image). Step image refs from existing RBRG pins.
- **Pouch**: New function to package build context as FROM SCRATCH OCI image, push to GAR. Tagged as `{vessel}:{inscribe_timestamp}-context` (or similar). Created as part of conjure flow.
- **Conjure**: Replace triggers.run dispatch with builds.create REST API call + wait. Consecration minted locally (Director assigns timestamps). No trigger-ensure, no rubric commit, no dispatch indirection.
- **Vouch**: Replace slsa-verifier with DSSE envelope signature verification (jq + base64 + openssl). Verify against embedded or fetched verified-builder KMS public keys. Proven approach from ₢AvAAB research.
- **Regime cleanup**: Remove RBRR_RUBRIC_REPO_URL, GitLab PAT secrets references, CB v2 connection references from regime and onboarding flows.
- **Absorbs ₢AvAAC**: Experiment modifications to rbf_Foundry.sh, rbgjv01, rbgjv02 are overwritten by new implementation (no separate revert step).

### Deferred (not in scope)
- Reliquary (co-versioned tool image snapshots)
- RBRG replacement — existing pin mechanism stays
- RBRV_BASE_IMAGE_* resolution against reliquary
- Air-gap / NO_PUBLIC_EGRESS enforcement
- Inscribe as reliquary generation
- Director IAM grant cleanup (separate pace)

## Key files
- `Tools/rbk/rbf_Foundry.sh` — stitch, conjure, pouch (major rewrite)
- `Tools/rbk/rbgjv/rbgjv01-download-verifier.sh` — replace slsa-verifier download with DSSE setup
- `Tools/rbk/rbgjv/rbgjv02-verify-provenance.sh` — replace slsa-verifier/direct_verify with DSSE chain
- `Tools/rbk/rbrr_regime.sh` / `.rbk/rbrr.env` — remove GitLab regime variables
- Tabtargets: rbw-DI (inscribe), rbw-DC (conjure) may need adaptation

## Verification
- End-to-end: conjure a vessel on depot10030 via the new pipeline
- Vouch: DSSE signature verification passes on the resulting image
- No GitLab references remain in active code paths
- Existing bind and graft modes unaffected

### spec-depot-lifecycle-gitlab-removal (₢AvAAE) [complete]

**[260321-1542] complete**

## Character
Systematic sweep with broad file surface but shallow depth per file. Mostly mechanical removals guided by the RBS0 survey from ₢AvAAD. Requires care to not break bind/graft modes or depot operations that remain valid.

## Goal
Remove all GitLab/trigger/slsa-verifier references from specifications and depot lifecycle code. Result: spec and code consistently reflect the builds.create + pouch + DSSE architecture. New depot creation works without GitLab.

## Scope

### Specifications (RBS0 + sub-documents)
- Remove definitions: rubric repo, vessel trigger, vessel directory, CB v2 connection/repository
- Remove regime variables: RBRR_CBV2_CONNECTION_NAME, RBRR_RUBRIC_REPO_URL, RBRG_SLSA_VERIFIER_URL/SHA256, CBV2 group
- Remove operations: trigger_build (RBSTB). Update rubric_inscribe (RBSRI) to reflect deferred/stub status.
- Update: ark_vouch (RBSAV) slsa-verifier → DSSE. ark_conjure (RBSAC) trigger → builds.create. Provenance definition.
- Remove: GitLab setup tabtarget, rubric repo URL check, CB v2 connection check patterns
- Review sub-documents: RBSRI, RBSTB, RBSAV, RBSAC, RBSRR

### Depot lifecycle code
- Payor (rbgp): Remove GitLab setup from depot_create, remove CB v2 connection/repository creation, remove Secret Manager PAT storage
- Governor (rbgg): Remove rubric repo URL check, CBV2 connection check
- Manual Procedures (rbgm): Remove GitLab setup guide
- Utility (rbgu): Remove rubric repo URL validation function
- Regime validation (rbrr_regime.sh): Remove or make optional the emptied variables
- Lifecycle Marshal (rblm): Remove CBV2/rubric from reset template

### Not in scope
- Reliquary implementation
- RBRG replacement
- New inscribe ceremony
- Air-gap / NO_PUBLIC_EGRESS

## Verification
- Regime validation passes with empty GitLab variables
- No references to slsa-verifier in active code paths
- Spec definitions consistent with implemented architecture
- Existing bind and graft modes unaffected

**[260321-1501] rough**

## Character
Systematic sweep with broad file surface but shallow depth per file. Mostly mechanical removals guided by the RBS0 survey from ₢AvAAD. Requires care to not break bind/graft modes or depot operations that remain valid.

## Goal
Remove all GitLab/trigger/slsa-verifier references from specifications and depot lifecycle code. Result: spec and code consistently reflect the builds.create + pouch + DSSE architecture. New depot creation works without GitLab.

## Scope

### Specifications (RBS0 + sub-documents)
- Remove definitions: rubric repo, vessel trigger, vessel directory, CB v2 connection/repository
- Remove regime variables: RBRR_CBV2_CONNECTION_NAME, RBRR_RUBRIC_REPO_URL, RBRG_SLSA_VERIFIER_URL/SHA256, CBV2 group
- Remove operations: trigger_build (RBSTB). Update rubric_inscribe (RBSRI) to reflect deferred/stub status.
- Update: ark_vouch (RBSAV) slsa-verifier → DSSE. ark_conjure (RBSAC) trigger → builds.create. Provenance definition.
- Remove: GitLab setup tabtarget, rubric repo URL check, CB v2 connection check patterns
- Review sub-documents: RBSRI, RBSTB, RBSAV, RBSAC, RBSRR

### Depot lifecycle code
- Payor (rbgp): Remove GitLab setup from depot_create, remove CB v2 connection/repository creation, remove Secret Manager PAT storage
- Governor (rbgg): Remove rubric repo URL check, CBV2 connection check
- Manual Procedures (rbgm): Remove GitLab setup guide
- Utility (rbgu): Remove rubric repo URL validation function
- Regime validation (rbrr_regime.sh): Remove or make optional the emptied variables
- Lifecycle Marshal (rblm): Remove CBV2/rubric from reset template

### Not in scope
- Reliquary implementation
- RBRG replacement
- New inscribe ceremony
- Air-gap / NO_PUBLIC_EGRESS

## Verification
- Regime validation passes with empty GitLab variables
- No references to slsa-verifier in active code paths
- Spec definitions consistent with implemented architecture
- Existing bind and graft modes unaffected

### payor-gitlab-dead-code-removal (₢AvAAF) [complete]

**[260323-0935] complete**

## Character
Mechanical code cleanup in a single large file. Low risk — the dead paths already handle empty variables gracefully.

## Goal
Remove GitLab-specific dead code from rbgp_Payor.sh: CB v2 connection/repository creation, Secret Manager PAT storage, GitLab setup validation. Remove rbgu_check_rubric_repo_url and zrbgu_gitlab_tokens_url_capture from rbgu_Utility.sh.

## Verification
- Depot creation tabtarget runs without error on active depot (no GitLab steps attempted)
- No references to RBRR_RUBRIC_REPO_URL or RBRR_CBV2_CONNECTION_NAME in active code paths

**[260321-1542] rough**

## Character
Mechanical code cleanup in a single large file. Low risk — the dead paths already handle empty variables gracefully.

## Goal
Remove GitLab-specific dead code from rbgp_Payor.sh: CB v2 connection/repository creation, Secret Manager PAT storage, GitLab setup validation. Remove rbgu_check_rubric_repo_url and zrbgu_gitlab_tokens_url_capture from rbgu_Utility.sh.

## Verification
- Depot creation tabtarget runs without error on active depot (no GitLab steps attempted)
- No references to RBRR_RUBRIC_REPO_URL or RBRR_CBV2_CONNECTION_NAME in active code paths

### spec-subdocument-gitlab-sweep (₢AvAAG) [complete]

**[260323-1000] complete**

## Character
Systematic review of included AsciiDoc sub-documents. Read each, identify stale GitLab/trigger/slsa-verifier references, rewrite to reflect builds.create + pouch + DSSE.

## Goal
Update RBSAV (ark_vouch), RBSAC (ark_conjure), and RBSRR (RegimeRepo) sub-documents to consistently reflect the new architecture. Remove or update any internal references to eliminated terms.

## Verification
- grep for eliminated terms across all .adoc files returns zero hits in active content
- Spec renders consistently (no broken cross-references or stale descriptions)

**[260321-1542] rough**

## Character
Systematic review of included AsciiDoc sub-documents. Read each, identify stale GitLab/trigger/slsa-verifier references, rewrite to reflect builds.create + pouch + DSSE.

## Goal
Update RBSAV (ark_vouch), RBSAC (ark_conjure), and RBSRR (RegimeRepo) sub-documents to consistently reflect the new architecture. Remove or update any internal references to eliminated terms.

## Verification
- grep for eliminated terms across all .adoc files returns zero hits in active content
- Spec renders consistently (no broken cross-references or stale descriptions)

### spec-base-image-enshrine-operation (₢AvAAI) [complete]

**[260323-1807] complete**

## Character
Design conversation with spec writing. Requires judgment about operation boundaries and interaction with existing conjure/stitch path.

## Goal
Define ark_enshrine operation in RBS0 spec and update RBSRV (RegimeVessel) with RBRV_IMAGE_[n]_{ORIGIN,ANCHOR} variables. Update RBSAC (ark_conjure) to describe anchor resolution path.

## Settled Design (from paddock)
- `RBRV_IMAGE_[n]_ORIGIN` — upstream tag declaration (e.g., `python:3.11-slim`)
- `RBRV_IMAGE_[n]_ANCHOR` — GAR-mirrored reference, format: `{sanitized-origin}-{10-char-sha256}`
- Anchor doubles as GAR tag — one name for humans and machines
- Enshrine writes anchor back to vessel regime
- Up to 3 per vessel (multi-stage)

## Spec Work
1. RBS0: linked terms for enshrine, RBRV_IMAGE variables, anchor format
2. RBSRV: add RBRV_IMAGE_[n]_ORIGIN and RBRV_IMAGE_[n]_ANCHOR regime variables
3. RBSAC: update conjure to describe anchor resolution (ANCHOR present → GAR reference, ANCHOR absent → pass ORIGIN through for development)
4. New spec subdocument for ark_enshrine operation

## Verification
- RBS0 linked terms defined
- RBSRV updated with new regime variables
- Conjure spec updated
- Paddock updated to reflect any remaining design discoveries

**[260323-1750] rough**

## Character
Design conversation with spec writing. Requires judgment about operation boundaries and interaction with existing conjure/stitch path.

## Goal
Define ark_enshrine operation in RBS0 spec and update RBSRV (RegimeVessel) with RBRV_IMAGE_[n]_{ORIGIN,ANCHOR} variables. Update RBSAC (ark_conjure) to describe anchor resolution path.

## Settled Design (from paddock)
- `RBRV_IMAGE_[n]_ORIGIN` — upstream tag declaration (e.g., `python:3.11-slim`)
- `RBRV_IMAGE_[n]_ANCHOR` — GAR-mirrored reference, format: `{sanitized-origin}-{10-char-sha256}`
- Anchor doubles as GAR tag — one name for humans and machines
- Enshrine writes anchor back to vessel regime
- Up to 3 per vessel (multi-stage)

## Spec Work
1. RBS0: linked terms for enshrine, RBRV_IMAGE variables, anchor format
2. RBSRV: add RBRV_IMAGE_[n]_ORIGIN and RBRV_IMAGE_[n]_ANCHOR regime variables
3. RBSAC: update conjure to describe anchor resolution (ANCHOR present → GAR reference, ANCHOR absent → pass ORIGIN through for development)
4. New spec subdocument for ark_enshrine operation

## Verification
- RBS0 linked terms defined
- RBSRV updated with new regime variables
- Conjure spec updated
- Paddock updated to reflect any remaining design discoveries

**[260323-1032] rough**

## Character
Design conversation with spec writing. Requires judgment about variable naming, operation boundaries, and interaction with existing conjure/stitch path.

## Goal
Define ark_enshrine operation in RBS0 spec and update RBSRV (RegimeVessel) with RBRV_BASE_IMAGE_[123]_{TAG,SHA} variables. Update RBSAC (ark_conjure) to describe optional SHA resolution path.

## Design Decisions to Capture
- RBRV_BASE_IMAGE_[n]_TAG: upstream tag declaration (e.g., python:3.11-slim)
- RBRV_BASE_IMAGE_[n]_SHA: optional GAR-mirrored digest pin
- SHA empty = pull from upstream at build time (development/getting-started path)
- SHA set = use GAR digest reference (reproducible/air-gap path)
- Enshrine operation: GCB job (builds.create) pulls upstream, pushes to GAR, emits digest
- Step image: gcr.io/cloud-builders/docker (available via Private Google Access)
- Vouch records which path was taken (pinned vs upstream)
- Reliquary holds tool images only (not base images)

## Verification
- RBS0 linked terms defined for enshrine, RBRV_BASE_IMAGE variables
- RBSRV updated with new regime variables
- Conjure spec updated to describe optional SHA resolution
- Paddock updated to reflect settled base image design

### implement-enshrine-core (₢AvAAJ) [complete]

**[260323-1845] complete**

## Character
Mechanical implementation building on settled spec and design decisions from ₢AvAAI and paddock.

## Goal
Implement the enshrine core: regime variables, skopeo copy operation, anchor construction, regime writeback. After this pace, `enshrine <vessel>` works standalone — you can enshrine a base image and see the anchor written.

## Settled Design (from paddock)
- Skopeo copy --all preserves manifest lists (multi-platform free)
- Anchor digest = manifest list digest (multi-platform) or manifest digest (single-platform)
- Anchor format: `{sanitized-origin}-{10-char-sha256}` via BCG parameter expansion
- Anchors live in same GAR repo as vessel images (RBRR_GAR_REPOSITORY)
- Skopeo already pinned in RBRG

## Steps
1. Add RBRV_IMAGE_[123]_{ORIGIN,ANCHOR} to rbrv_regime.sh — buv enrollment, conditional validation (conjure-mode only), render section, readonly lock
2. Settle execution model: Director-local skopeo vs GCB job with skopeo step image (spec says Director; RBRG pin is a container image)
3. Implement enshrine: skopeo inspect --raw for digest, skopeo copy --all to GAR with anchor tag, anchor construction, ANCHOR writeback to rbrv.env
4. Add enshrine tabtarget and workbench routing

## Verification
- Regime validates ORIGIN/ANCHOR fields for conjure-mode vessels
- Enshrine copies upstream image to GAR, anchor tag visible in rbi_list
- ANCHOR written back to rbrv.env with correct format

**[260323-1827] rough**

## Character
Mechanical implementation building on settled spec and design decisions from ₢AvAAI and paddock.

## Goal
Implement the enshrine core: regime variables, skopeo copy operation, anchor construction, regime writeback. After this pace, `enshrine <vessel>` works standalone — you can enshrine a base image and see the anchor written.

## Settled Design (from paddock)
- Skopeo copy --all preserves manifest lists (multi-platform free)
- Anchor digest = manifest list digest (multi-platform) or manifest digest (single-platform)
- Anchor format: `{sanitized-origin}-{10-char-sha256}` via BCG parameter expansion
- Anchors live in same GAR repo as vessel images (RBRR_GAR_REPOSITORY)
- Skopeo already pinned in RBRG

## Steps
1. Add RBRV_IMAGE_[123]_{ORIGIN,ANCHOR} to rbrv_regime.sh — buv enrollment, conditional validation (conjure-mode only), render section, readonly lock
2. Settle execution model: Director-local skopeo vs GCB job with skopeo step image (spec says Director; RBRG pin is a container image)
3. Implement enshrine: skopeo inspect --raw for digest, skopeo copy --all to GAR with anchor tag, anchor construction, ANCHOR writeback to rbrv.env
4. Add enshrine tabtarget and workbench routing

## Verification
- Regime validates ORIGIN/ANCHOR fields for conjure-mode vessels
- Enshrine copies upstream image to GAR, anchor tag visible in rbi_list
- ANCHOR written back to rbrv.env with correct format

**[260323-1827] rough**

## Character
Mechanical implementation building on settled spec and design decisions from ₢AvAAI and paddock.

## Goal
Implement the enshrine core: regime variables, skopeo copy operation, anchor construction, regime writeback. After this pace, `enshrine <vessel>` works standalone — you can enshrine a base image and see the anchor written.

## Settled Design (from paddock)
- Skopeo copy --all preserves manifest lists (multi-platform free)
- Anchor digest = manifest list digest (multi-platform) or manifest digest (single-platform)
- Anchor format: `{sanitized-origin}-{10-char-sha256}` via BCG parameter expansion
- Anchors live in same GAR repo as vessel images (RBRR_GAR_REPOSITORY)
- Skopeo already pinned in RBRG

## Steps
1. Add RBRV_IMAGE_[123]_{ORIGIN,ANCHOR} to rbrv_regime.sh — buv enrollment, conditional validation (conjure-mode only), render section, readonly lock
2. Settle execution model: Director-local skopeo vs GCB job with skopeo step image (spec says Director; RBRG pin is a container image)
3. Implement enshrine: skopeo inspect --raw for digest, skopeo copy --all to GAR with anchor tag, anchor construction, ANCHOR writeback to rbrv.env
4. Add enshrine tabtarget and workbench routing

## Verification
- Regime validates ORIGIN/ANCHOR fields for conjure-mode vessels
- Enshrine copies upstream image to GAR, anchor tag visible in rbi_list
- ANCHOR written back to rbrv.env with correct format

**[260323-1750] rough**

## Character
Implementation work building on settled spec from ₢AvAAI. Mechanical once the spec is clear.

## Goal
Implement ark_enshrine: GCB job that pulls upstream base image by ORIGIN tag, pushes to GAR with anchor tag, writes ANCHOR back to vessel regime. Update stitch to resolve ANCHOR at conjure time. Update vouch to record resolution path.

## Settled Design (from paddock)
- ORIGIN holds upstream tag: `python:3.11-slim`
- ANCHOR holds `{sanitized-origin}-{10-char-sha256}`: `python-3.11-slim-abc123def4`
- Anchor construction: BCG parameter expansion (`${origin//[:\/]/-}` + `${digest:0:10}`)
- Anchor IS the GAR tag — enshrine pushes with this tag, conjure pulls by this tag
- Enshrine writes ANCHOR back to rbrv regime file

## Steps
1. Add RBRV_IMAGE_[123]_{ORIGIN,ANCHOR} to vessel regime (rbrv_regime.sh)
2. Implement enshrine: stitch JSON for GCB job (docker pull ORIGIN, docker tag with anchor, docker push to GAR)
3. Post-enshrine: read GCB output digest, construct anchor string, write to regime
4. Add enshrine tabtarget and workbench routing
5. Update conjure stitch: substitute GAR reference (from ANCHOR) into Dockerfile ARGs
6. Update vouch to record base image provenance (anchored vs pass-through)
7. Test: enshrine python:3.11-slim, verify anchor written, conjure a vessel using it

## Verification
- Enshrine pulls upstream image, pushes to GAR with anchor tag, writes ANCHOR to regime
- Conjure with ANCHOR set uses GAR reference
- Conjure with ANCHOR empty passes ORIGIN through (development path)
- Vouch records which path was taken

**[260323-1032] rough**

## Character
Implementation work building on settled spec from ₢AvAAI. Mechanical once the spec is clear.

## Goal
Implement ark_enshrine: GCB job that pulls upstream base image by tag, pushes to GAR, outputs digest. Add RBRV_BASE_IMAGE_[n]_{TAG,SHA} regime variables. Update stitch to resolve base images (SHA present → GAR digest, SHA absent → pass TAG through). Update vouch to record resolution path.

## Steps
1. Add RBRV_BASE_IMAGE_[123]_{TAG,SHA} to vessel regime (rbrv_regime.sh)
2. Implement enshrine GCB job builder (stitch JSON for pull+push)
3. Add enshrine tabtarget and workbench routing
4. Update conjure stitch to substitute GAR digest when SHA is set
5. Update vouch to record base image provenance path
6. Test: enshrine python:3.11-slim, conjure a vessel using it

## Verification
- Enshrine pulls upstream image and pushes to GAR
- Conjure with SHA set uses GAR reference
- Conjure with SHA empty uses upstream tag
- Vouch records which path was taken

### wire-anchor-into-conjure-vouch (₢AvAAM) [complete]

**[260323-1900] complete**

## Character
Integration work connecting enshrine outputs to conjure and vouch pipelines.

## Goal
Conjure resolves ANCHOR to full GAR reference in Dockerfile ARGs. Vouch records which provenance path was taken.

## Steps
1. Update conjure stitch: when RBRV_IMAGE_n_ANCHOR is set, resolve to full GAR image reference (`REGISTRY/REPO/VESSEL:ANCHOR`) and substitute into Dockerfile ARG
2. When ANCHOR is empty, pass ORIGIN through as upstream tag (development path)
3. Update vouch to record base image provenance: anchored (GAR-mirrored) vs pass-through (upstream direct)
4. Ensure vouch summary includes anchor string when present

## Verification
- Conjure with ANCHOR set uses GAR reference in build
- Conjure with ANCHOR empty passes ORIGIN through
- Vouch records which path was taken

**[260323-1828] rough**

## Character
Integration work connecting enshrine outputs to conjure and vouch pipelines.

## Goal
Conjure resolves ANCHOR to full GAR reference in Dockerfile ARGs. Vouch records which provenance path was taken.

## Steps
1. Update conjure stitch: when RBRV_IMAGE_n_ANCHOR is set, resolve to full GAR image reference (`REGISTRY/REPO/VESSEL:ANCHOR`) and substitute into Dockerfile ARG
2. When ANCHOR is empty, pass ORIGIN through as upstream tag (development path)
3. Update vouch to record base image provenance: anchored (GAR-mirrored) vs pass-through (upstream direct)
4. Ensure vouch summary includes anchor string when present

## Verification
- Conjure with ANCHOR set uses GAR reference in build
- Conjure with ANCHOR empty passes ORIGIN through
- Vouch records which path was taken

### director-vessel-arg-cleanup (₢AvAAS) [complete]

**[260324-1913] complete**

## Character
Mechanical cleanup — consistent vessel arg handling and vocabulary alignment.

## Goals
1. Extract `zrbf_resolve_vessel` shared helper (accepts sigil or path, lists on error)
2. Replace duplicated list-and-die blocks in rbf_enshrine, rbf_create, rbf_abjure, rbf_about
3. Rename Ark→Consecration in tabtarget frontispieces (DC, DA)
4. Rename rbw-DE frontispiece: DirectorEnshrinesBaseImages → DirectorEnshrinesVessel
5. Delete rbw-Db tabtarget (about is automatic; standalone rbf_about remains for graft internal use)
6. Update spec references (RBSAE, RBSAC, RBS0) if they mention Ark in tabtarget context

## Verification
- All vessel-arg commands accept both sigil and full path
- No-arg and invalid-arg both list available vessels
- No tabtarget frontispiece references Ark
- rbw-Db tabtarget gone
- Regime validation passes

**[260324-1853] rough**

## Character
Mechanical cleanup — consistent vessel arg handling and vocabulary alignment.

## Goals
1. Extract `zrbf_resolve_vessel` shared helper (accepts sigil or path, lists on error)
2. Replace duplicated list-and-die blocks in rbf_enshrine, rbf_create, rbf_abjure, rbf_about
3. Rename Ark→Consecration in tabtarget frontispieces (DC, DA)
4. Rename rbw-DE frontispiece: DirectorEnshrinesBaseImages → DirectorEnshrinesVessel
5. Delete rbw-Db tabtarget (about is automatic; standalone rbf_about remains for graft internal use)
6. Update spec references (RBSAE, RBSAC, RBS0) if they mention Ark in tabtarget context

## Verification
- All vessel-arg commands accept both sigil and full path
- No-arg and invalid-arg both list available vessels
- No tabtarget frontispiece references Ark
- rbw-Db tabtarget gone
- Regime validation passes

### graft-combine-about-vouch-gcb-job (₢AvAAT) [complete]

**[260324-1928] complete**

## Character
Structural simplification — eliminate graft's standalone about GCB job by combining about+vouch into a single GCB submission.

## Problem
Graft is the only mode with a 3-phase flow: local push → about GCB → vouch GCB. Conjure and bind embed about in their image GCB job (2 phases: image+about GCB → vouch GCB). This asymmetry creates an orphan gap: if the session dies between about and vouch, graft has an about-without-vouch state that batch vouch must clean up.

## Design
Combine graft's about steps and vouch steps into a single GCB submission. The flow becomes:
- **Graft**: local docker push → combined about+vouch GCB job (1 cloud job)
- **Conjure**: combined image+about GCB job → vouch GCB job (2 cloud jobs)
- **Bind**: combined mirror+about GCB job → vouch GCB job (2 cloud jobs)

All modes now have exactly one failure gap (between the image-producing phase and the metadata phase). No mode can have about-without-vouch.

## Implementation
1. In `rbf_create` chain (line ~1064): remove the graft-specific `rbf_about` call and instead assemble a combined about+vouch GCB job
2. New internal function `zrbf_graft_metadata_submit`: assembles about steps + vouch steps into one Build resource JSON, submits via builds.create
3. The about gate in `rbf_vouch` (HEAD check for -about existence) is unnecessary when about and vouch are in the same job — about steps run first by step ordering
4. Remove standalone `rbf_about` tabtarget path from the graft chain; `rbf_about` remains for manual recovery use
5. Update `rbf_batch_vouch` — graft consecrations should no longer appear as about-without-vouch orphans

## Spec Updates
- **RBS0** line ~1523: Update graft flow description (no longer 'degenerate about-only job' + separate vouch)
- **RBS0** line ~1597: Update about artifact creation description
- **RBSAB**: Note that graft about runs in combined about+vouch job, not standalone
- **RBSCB**: Update Cloud Build posture if it describes graft's job count
- **RBSDV**: director_vouch spec — graft vouch is now part of combined job

## Verification
- Three-mode test fixture (`rbw-tf.TestFixture.three-mode.sh`) passes
- Graft produces -image, -about, -vouch in exactly 1 GCB job (was 2)
- Batch vouch finds no orphaned graft consecrations
- Conjure and bind flows unchanged

**[260324-1912] rough**

## Character
Structural simplification — eliminate graft's standalone about GCB job by combining about+vouch into a single GCB submission.

## Problem
Graft is the only mode with a 3-phase flow: local push → about GCB → vouch GCB. Conjure and bind embed about in their image GCB job (2 phases: image+about GCB → vouch GCB). This asymmetry creates an orphan gap: if the session dies between about and vouch, graft has an about-without-vouch state that batch vouch must clean up.

## Design
Combine graft's about steps and vouch steps into a single GCB submission. The flow becomes:
- **Graft**: local docker push → combined about+vouch GCB job (1 cloud job)
- **Conjure**: combined image+about GCB job → vouch GCB job (2 cloud jobs)
- **Bind**: combined mirror+about GCB job → vouch GCB job (2 cloud jobs)

All modes now have exactly one failure gap (between the image-producing phase and the metadata phase). No mode can have about-without-vouch.

## Implementation
1. In `rbf_create` chain (line ~1064): remove the graft-specific `rbf_about` call and instead assemble a combined about+vouch GCB job
2. New internal function `zrbf_graft_metadata_submit`: assembles about steps + vouch steps into one Build resource JSON, submits via builds.create
3. The about gate in `rbf_vouch` (HEAD check for -about existence) is unnecessary when about and vouch are in the same job — about steps run first by step ordering
4. Remove standalone `rbf_about` tabtarget path from the graft chain; `rbf_about` remains for manual recovery use
5. Update `rbf_batch_vouch` — graft consecrations should no longer appear as about-without-vouch orphans

## Spec Updates
- **RBS0** line ~1523: Update graft flow description (no longer 'degenerate about-only job' + separate vouch)
- **RBS0** line ~1597: Update about artifact creation description
- **RBSAB**: Note that graft about runs in combined about+vouch job, not standalone
- **RBSCB**: Update Cloud Build posture if it describes graft's job count
- **RBSDV**: director_vouch spec — graft vouch is now part of combined job

## Verification
- Three-mode test fixture (`rbw-tf.TestFixture.three-mode.sh`) passes
- Graft produces -image, -about, -vouch in exactly 1 GCB job (was 2)
- Batch vouch finds no orphaned graft consecrations
- Conjure and bind flows unchanged

### implement-reliquary-inscribe-tool-images (₢AvAAK) [complete]

**[260326-2056] complete**

## Character
Implementation work with GCP infrastructure. Builds on settled reliquary design from paddock.

## Accomplished
- Inscribe operation implemented and proven (6 tool images mirrored to GAR reliquary r260324201411)
- zrbf_resolve_tool_images() replaces all 19 RBRG_*_IMAGE_REF usages with reliquary GAR paths
- RBRV_RELIQUARY required for all vessel modes (universal reliquary — settles paddock open question)
- Enshrine uses reliquary skopeo — stale-pin problem solved
- Oras dropped from reliquary (not used as GCB step image)
- Sentry vessel updated: RBRV_RELIQUARY=r260324201411, RBRV_IMAGE_1_ANCHOR=ubuntu-24.04-186072bba1

## Remaining

### A. Switch stitch from args to script field
Conjure fails because about step scripts (8KB) exceed GCB's 10K per-arg limit when inlined as args[1]. The GCB `script` field is designed for multi-line scripts and likely has a much higher limit (verify).

1. Research GCB `script` field — confirm: substitution expansion works, size limit (believed 100KB but VERIFY), shell selection behavior
2. Key fact: `script` field ignores `entrypoint` and uses image default shell. Steps currently distinguish bash vs sh via entrypoint. Fix: prepend #!/bin/bash or #!/bin/sh based on the step def tuple's entrypoint field
3. Key fact: dollar-escaping pattern ($->$$, then un-escape $${_RBG -> ${_RBG) should work unchanged — `script` field has same substitution semantics as `args`
4. Modify stitch step-assembly loop (zrbf_stitch_build_json): change jq from `args: [$flag, $script]` to `script: $script` with shebang prefix
5. Modify about helper (zrbf_assemble_about_steps): same change
6. Modify vouch helper (zrbf_assemble_vouch_steps): same change
7. Consider enshrine/mirror/inscribe submit functions too (smaller scripts, not urgent, but consistency)

### B. Update all vessels with RBRV_RELIQUARY
All vessels in rbev-vessels/ need RBRV_RELIQUARY=r260324201411 to pass regime validation. Not just sentry — bind and graft vessels too.

### C. Paddock updates
Record in paddock: oras eliminated from reliquary, universal reliquary decision (all modes), script field migration rationale.

### D. Test conjure end-to-end
1. Test conjure on rbev-sentry-ubuntu-large (full inscribe -> enshrine -> conjure -> vouch)
2. Verify vouch records anchored provenance

## Verification
- Inscribe -> enshrine -> conjure -> vouch full pipeline succeeds
- All tool images pulled from reliquary GAR paths
- Step scripts delivered via `script` field
- All vessels pass regime validation with RBRV_RELIQUARY

**[260324-2035] rough**

## Character
Implementation work with GCP infrastructure. Builds on settled reliquary design from paddock.

## Accomplished
- Inscribe operation implemented and proven (6 tool images mirrored to GAR reliquary r260324201411)
- zrbf_resolve_tool_images() replaces all 19 RBRG_*_IMAGE_REF usages with reliquary GAR paths
- RBRV_RELIQUARY required for all vessel modes (universal reliquary — settles paddock open question)
- Enshrine uses reliquary skopeo — stale-pin problem solved
- Oras dropped from reliquary (not used as GCB step image)
- Sentry vessel updated: RBRV_RELIQUARY=r260324201411, RBRV_IMAGE_1_ANCHOR=ubuntu-24.04-186072bba1

## Remaining

### A. Switch stitch from args to script field
Conjure fails because about step scripts (8KB) exceed GCB's 10K per-arg limit when inlined as args[1]. The GCB `script` field is designed for multi-line scripts and likely has a much higher limit (verify).

1. Research GCB `script` field — confirm: substitution expansion works, size limit (believed 100KB but VERIFY), shell selection behavior
2. Key fact: `script` field ignores `entrypoint` and uses image default shell. Steps currently distinguish bash vs sh via entrypoint. Fix: prepend #!/bin/bash or #!/bin/sh based on the step def tuple's entrypoint field
3. Key fact: dollar-escaping pattern ($->$$, then un-escape $${_RBG -> ${_RBG) should work unchanged — `script` field has same substitution semantics as `args`
4. Modify stitch step-assembly loop (zrbf_stitch_build_json): change jq from `args: [$flag, $script]` to `script: $script` with shebang prefix
5. Modify about helper (zrbf_assemble_about_steps): same change
6. Modify vouch helper (zrbf_assemble_vouch_steps): same change
7. Consider enshrine/mirror/inscribe submit functions too (smaller scripts, not urgent, but consistency)

### B. Update all vessels with RBRV_RELIQUARY
All vessels in rbev-vessels/ need RBRV_RELIQUARY=r260324201411 to pass regime validation. Not just sentry — bind and graft vessels too.

### C. Paddock updates
Record in paddock: oras eliminated from reliquary, universal reliquary decision (all modes), script field migration rationale.

### D. Test conjure end-to-end
1. Test conjure on rbev-sentry-ubuntu-large (full inscribe -> enshrine -> conjure -> vouch)
2. Verify vouch records anchored provenance

## Verification
- Inscribe -> enshrine -> conjure -> vouch full pipeline succeeds
- All tool images pulled from reliquary GAR paths
- Step scripts delivered via `script` field
- All vessels pass regime validation with RBRV_RELIQUARY

**[260324-2034] rough**

## Character
Implementation work with GCP infrastructure. Builds on settled reliquary design from paddock.

## Accomplished
- Inscribe operation implemented and proven (6 tool images mirrored to GAR reliquary r260324201411)
- zrbf_resolve_tool_images() replaces all 19 RBRG_*_IMAGE_REF usages with reliquary GAR paths
- RBRV_RELIQUARY required for all vessel modes (universal reliquary — settles paddock open question)
- Enshrine uses reliquary skopeo — stale-pin problem solved
- Oras dropped from reliquary (not used as GCB step image)

## Remaining: Switch stitch from args to script field
Conjure fails because about step scripts (8KB) exceed GCB's 10K per-arg limit when inlined as args[1]. GCB's `script` field has a 100KB limit and is designed for multi-line scripts.

### Steps
1. Research GCB `script` field: confirm substitution expansion, shell selection, and 100KB limit
2. Modify stitch step-assembly loop: change from `args: [$flag, $script]` to `script: $script`
3. Modify about helper (zrbf_assemble_about_steps): same args-to-script change
4. Modify vouch helper (zrbf_assemble_vouch_steps): same args-to-script change
5. Verify entrypoint behavior: `script` field uses the image's default shell — may need #!/bin/bash prefix for bash steps
6. Test conjure end-to-end on rbev-sentry-ubuntu-large
7. Test vouch end-to-end (embedded in conjure flow)

## Verification
- Inscribe -> enshrine -> conjure -> vouch full pipeline succeeds
- All tool images pulled from reliquary GAR paths (no RBRG refs)
- Step scripts delivered via `script` field, no 10K arg limit

**[260323-1034] rough**

## Character
Implementation work with GCP infrastructure. Builds on settled reliquary design from paddock.

## Goal
Implement inscribe operation: mirror all tool images (gcloud, docker, binfmt, syft, alpine, oras, skopeo) from upstream registries to a datestamped GAR namespace. Produce reliquary identifier. Update stitch to resolve step image references from reliquary instead of upstream digest pins.

## Steps
1. Define reliquary GAR namespace scheme (e.g., {repo}/reliquary/{datestamp}/...)
2. Implement inscribe GCB job: pull each tool image from upstream, tag for GAR, push, record digests
3. Produce reliquary manifest (datestamp + image-to-digest mapping)
4. Add RBRV_RELIQUARY vessel regime variable
5. Update stitch: when RBRV_RELIQUARY is set, resolve all step image refs from reliquary GAR paths instead of upstream RBRG pins
6. Add inscribe tabtarget and workbench routing
7. Test: inscribe, then conjure rbev-busybox using reliquary tool images

## Verification
- Inscribe mirrors all tool images to GAR
- Conjure with RBRV_RELIQUARY set uses GAR tool image references
- Built images are identical (same behavior) to upstream-pin builds

### enshrine-end-to-end-verification (₢AvAAN) [complete]

**[260326-2102] complete**

## Character
End-to-end validation of the full enshrine → conjure → vouch pipeline.

## Goal
Prove the enshrine feature works against a real upstream image with multi-platform content.

## Steps
1. Enshrine a real multi-platform image (e.g., python:3.11-slim) for a test vessel
2. Verify anchor written to rbrv.env with correct format
3. Verify image present in GAR with all upstream platforms (rbi_show on anchor tag)
4. Conjure the vessel using the anchored base image
5. Verify vouch records anchored provenance path

## Verification
- Full round-trip: enshrine → conjure → vouch with anchored base image
- rbi_list shows anchor tag alongside consecration tags
- Multi-platform manifest list preserved in GAR

**[260323-1828] rough**

## Character
End-to-end validation of the full enshrine → conjure → vouch pipeline.

## Goal
Prove the enshrine feature works against a real upstream image with multi-platform content.

## Steps
1. Enshrine a real multi-platform image (e.g., python:3.11-slim) for a test vessel
2. Verify anchor written to rbrv.env with correct format
3. Verify image present in GAR with all upstream platforms (rbi_show on anchor tag)
4. Conjure the vessel using the anchored base image
5. Verify vouch records anchored provenance path

## Verification
- Full round-trip: enshrine → conjure → vouch with anchored base image
- rbi_list shows anchor tag alongside consecration tags
- Multi-platform manifest list preserved in GAR

### rbrg-remove-slsa-verifier-dead-code (₢AvAAQ) [complete]

**[260327-0808] complete**

## Character
Cleanup — remove dead infrastructure from a dropped dependency. Mechanical.

## Goal
Remove all slsa-verifier binary pin infrastructure from rbrg_cli.sh. The slsa-verifier was dropped in ₢AvAAB (DSSE verification replaces it). The binary pin machinery is now dead code that breaks under `set -u` when the empty array workaround is removed.

## Scope
- `Tools/rbk/rbrg_cli.sh`: Remove `ZRBRG_BINARY_LINES` and `ZRBRG_BINARY_PINS_REFRESHED_AT` module variables. Remove binary section from `zrbrg_write_rbrg`. Remove `rbrg_refresh_binary_pins` function entirely. Remove binary pass-through from `rbrg_refresh_gcb_pins`. Remove the guard added in ₢AvAAP (temporary workaround).
- `Tools/rbk/rbrg_regime.sh`: Check for any slsa-verifier enrollment references.
- Zipper/tabtarget: Remove `rbw-DPB.DirectorRefreshesBinaryPins.sh` tabtarget and its zipper entry — the underlying command is gone.
- `.rbk/rbrg.env`: Already clean (no binary pin lines). Confirm no regression.

## Verification
- `BURE_COUNTDOWN=skip tt/rbw-DPG.DirectorRefreshesGcbPins.sh` succeeds
- No references to `SLSA_VERIFIER` in rbrg_cli.sh
- No `rbw-DPB` tabtarget remains
- regime-validation passes

**[260327-0744] rough**

## Character
Cleanup — remove dead infrastructure from a dropped dependency. Mechanical.

## Goal
Remove all slsa-verifier binary pin infrastructure from rbrg_cli.sh. The slsa-verifier was dropped in ₢AvAAB (DSSE verification replaces it). The binary pin machinery is now dead code that breaks under `set -u` when the empty array workaround is removed.

## Scope
- `Tools/rbk/rbrg_cli.sh`: Remove `ZRBRG_BINARY_LINES` and `ZRBRG_BINARY_PINS_REFRESHED_AT` module variables. Remove binary section from `zrbrg_write_rbrg`. Remove `rbrg_refresh_binary_pins` function entirely. Remove binary pass-through from `rbrg_refresh_gcb_pins`. Remove the guard added in ₢AvAAP (temporary workaround).
- `Tools/rbk/rbrg_regime.sh`: Check for any slsa-verifier enrollment references.
- Zipper/tabtarget: Remove `rbw-DPB.DirectorRefreshesBinaryPins.sh` tabtarget and its zipper entry — the underlying command is gone.
- `.rbk/rbrg.env`: Already clean (no binary pin lines). Confirm no regression.

## Verification
- `BURE_COUNTDOWN=skip tt/rbw-DPG.DirectorRefreshesGcbPins.sh` succeeds
- No references to `SLSA_VERIFIER` in rbrg_cli.sh
- No `rbw-DPB` tabtarget remains
- regime-validation passes

**[260324-0821] rough**

## Character
Cleanup — remove dead infrastructure from a dropped dependency.

## Goal
Remove all slsa-verifier binary pin infrastructure from rbrg_cli.sh. The slsa-verifier was dropped in ₢AvAAB (DSSE verification replaces it). The binary pin machinery is now dead code that breaks under `set -u` when the empty array workaround is removed.

## Scope
- `Tools/rbk/rbrg_cli.sh`: Remove `ZRBRG_BINARY_LINES` and `ZRBRG_BINARY_PINS_REFRESHED_AT` module variables. Remove binary section from `zrbrg_write_rbrg`. Remove `rbrg_refresh_binary_pins` function entirely. Remove binary pass-through from `rbrg_refresh_gcb_pins`. Remove the guard added in ₢AvAAP (temporary workaround).
- `Tools/rbk/rbrg_regime.sh`: Check for any slsa-verifier enrollment references.
- Zipper/tabtarget: Check if `rbw-DPB.DirectorRefreshesBinaryPins.sh` references a dead command.
- `.rbk/rbrg.env`: Already clean (no binary pin lines). Confirm no regression.

## Verification
- `BURE_COUNTDOWN=skip tt/rbw-DPG.DirectorRefreshesGcbPins.sh` succeeds
- No references to `SLSA_VERIFIER` in rbrg_cli.sh
- regime-validation passes

### dual-pool-regime-normalization (₢AvAAL) [complete]

**[260327-0837] complete**

## Character
Architectural implementation — cohesive but wide-reaching. Every change serves one goal: vessel-level egress routing via dual pools. Requires careful attention to spec/code consistency across ~20 files, but each individual change is mechanical.

## Goal
Implement dual-pool architecture (tether/airgap) with vessel-level egress mode and operation-level pool routing. Normalize RBRR pool regime variable to eliminate duplication. All code and spec changes — no live GCP operations. Depot is already destroyed before this pace begins.

## Design Decisions

### Dual Pools
- Depot gets two pools: `{stem}-tether` (public egress) and `{stem}-airgap` (NO_PUBLIC_EGRESS)
- Pool suffixes: `-tether` (public) / `-airgap` (NO_PUBLIC_EGRESS)

### Regime Normalization
- `RBRR_GCB_WORKER_POOL` replaced by `RBRR_GCB_POOL_STEM` (just the pool base name)
- Full pool paths derived at kindle time: `RBDC_POOL_TETHER` / `RBDC_POOL_AIRGAP`
- Eliminates duplication of project ID and region in the pool path

### Vessel Egress Mode
- Vessel declares `RBRV_EGRESS_MODE=tether|airgap`
- Controls which pool the primary build operation (conjure/bind) uses

### Operation Pool Routing
| Operation | Pool | Why |
|-----------|------|-----|
| Inscribe | always tether | Pulls tool images from upstream registries |
| Enshrine | always tether | Pulls base images from upstream via skopeo (GCB-only) |
| Conjure/Bind | vessel's RBRV_EGRESS_MODE | Build-time deps vary per vessel |
| About | always airgap | Only needs GAR + Google APIs (Private Google Access) |
| Vouch | always airgap | Only needs GAR + Google APIs (Private Google Access) |

About/vouch always-airgap is a security property: verification can never be influenced by a compromised public registry, even for tether vessels.

### Vessel Assignments
| Vessel | Mode | Rationale |
|--------|------|-----------|
| rbev-busybox | airgap | Enshrine busybox:latest, no apt-get, positive airgap test |
| rbev-busybox-native-verify | tether | Hardcoded FROM busybox:latest in Dockerfile |
| rbev-busybox-graft | tether | Graft mode |
| rbev-sentry-ubuntu-large | tether | apt-get in Dockerfile needs internet |
| rbev-bottle-ubuntu-test | tether | apt-get in Dockerfile |
| rbev-bottle-anthropic-jupyter | tether | pip install in Dockerfile |
| rbev-ubu-safety | tether | apt-get in Dockerfile |
| rbev-busybox-airgap-negative-canary | airgap | NEW: no anchor, designed to fail — permanent regression sentinel |

## Code Changes
1. `rbgc_Constants.sh` — add RBGC_POOL_SUFFIX_TETHER / _AIRGAP
2. `.rbk/rbrr.env` — rename RBRR_GCB_WORKER_POOL → RBRR_GCB_POOL_STEM (value: `rbw-depot10040-pool`, will gain suffixes on next depot create)
3. `rbrr_regime.sh` — new validation for stem format (no full path)
4. `rbdc_DerivedConstants.sh` — derive RBDC_POOL_TETHER / RBDC_POOL_AIRGAP from stem + project + region
5. `rbrv_regime.sh` — add RBRV_EGRESS_MODE (tether|airgap) validation
6. All 8 existing vessel `rbrv.env` files — add RBRV_EGRESS_MODE per assignment table
7. Create `rbev-vessels/rbev-busybox-airgap-negative-canary/` — rbrv.env (airgap, no anchor) + Dockerfile (FROM $RBF_IMAGE_1)
8. `rbf_Foundry.sh` — pool routing: conjure/bind use RBRV_EGRESS_MODE, about always RBDC_POOL_AIRGAP, vouch always RBDC_POOL_AIRGAP, inscribe always RBDC_POOL_TETHER, enshrine always RBDC_POOL_TETHER
9. `rbgp_Payor.sh` — create both pools (tether with default egress, airgap with NO_PUBLIC_EGRESS networkConfig), delete both pools
10. `rbgm_ManualProcedures.sh` — dual pool troubleshooting guidance

## Spec Changes
11. `RBS0-SpecTop.adoc` — mapping section (new linked terms for RBRV_EGRESS_MODE, RBRR_GCB_POOL_STEM, RBDC_POOL_*), variable definitions (~line 2488), vouch pool constraint (~line 1649), operation pool routing table
12. `RBSRR-RegimeRepo.adoc` — RBRR_GCB_POOL_STEM definition replacing RBRR_GCB_WORKER_POOL
13. `RBSRV-RegimeVessel.adoc` — add RBRV_EGRESS_MODE variable definition
14. `RBSDC-depot_create.adoc` — dual pool creation procedure
15. `RBSDD-depot_destroy.adoc` — dual pool deletion procedure
16. `RBSCB-CloudBuildPosture.adoc` — rewrite Current Posture (dual pools, operation routing) and Egress Lockdown (now implemented, no longer potential)
17. `RBSHR-HorizonRoadmap.adoc` — graduate egress lockdown from deferred
18. `RBSAB-ark_about.adoc` — always-airgap pool routing
19. `RBSAV-ark_vouch.adoc` — always-airgap pool routing
20. `RBSAC-ark_conjure.adoc` — vessel-routed pool
21. `RBSQB-quota_build.adoc` — dual pool capacity considerations

## Verification
- `tt/buw-rer.RenderEnvironmentRegime.sh` renders without error (derived constants)
- `tt/buw-rev.ValidateEnvironmentRegime.sh` passes
- `tt/rbw-rvv.ValidateVesselRegime.sh` passes for all vessels including canary
- `tt/rbw-rrv.ValidateRepoRegime.sh` passes with new stem variable
- No references to RBRR_GCB_WORKER_POOL remain (grep clean)
- All spec cross-references consistent

Note: No live depot exists during this pace. Regime validates structurally but pool paths won't resolve until ₢AvAAH creates the new depot.

**[260327-0744] rough**

## Character
Architectural implementation — cohesive but wide-reaching. Every change serves one goal: vessel-level egress routing via dual pools. Requires careful attention to spec/code consistency across ~20 files, but each individual change is mechanical.

## Goal
Implement dual-pool architecture (tether/airgap) with vessel-level egress mode and operation-level pool routing. Normalize RBRR pool regime variable to eliminate duplication. All code and spec changes — no live GCP operations. Depot is already destroyed before this pace begins.

## Design Decisions

### Dual Pools
- Depot gets two pools: `{stem}-tether` (public egress) and `{stem}-airgap` (NO_PUBLIC_EGRESS)
- Pool suffixes: `-tether` (public) / `-airgap` (NO_PUBLIC_EGRESS)

### Regime Normalization
- `RBRR_GCB_WORKER_POOL` replaced by `RBRR_GCB_POOL_STEM` (just the pool base name)
- Full pool paths derived at kindle time: `RBDC_POOL_TETHER` / `RBDC_POOL_AIRGAP`
- Eliminates duplication of project ID and region in the pool path

### Vessel Egress Mode
- Vessel declares `RBRV_EGRESS_MODE=tether|airgap`
- Controls which pool the primary build operation (conjure/bind) uses

### Operation Pool Routing
| Operation | Pool | Why |
|-----------|------|-----|
| Inscribe | always tether | Pulls tool images from upstream registries |
| Enshrine | always tether | Pulls base images from upstream via skopeo (GCB-only) |
| Conjure/Bind | vessel's RBRV_EGRESS_MODE | Build-time deps vary per vessel |
| About | always airgap | Only needs GAR + Google APIs (Private Google Access) |
| Vouch | always airgap | Only needs GAR + Google APIs (Private Google Access) |

About/vouch always-airgap is a security property: verification can never be influenced by a compromised public registry, even for tether vessels.

### Vessel Assignments
| Vessel | Mode | Rationale |
|--------|------|-----------|
| rbev-busybox | airgap | Enshrine busybox:latest, no apt-get, positive airgap test |
| rbev-busybox-native-verify | tether | Hardcoded FROM busybox:latest in Dockerfile |
| rbev-busybox-graft | tether | Graft mode |
| rbev-sentry-ubuntu-large | tether | apt-get in Dockerfile needs internet |
| rbev-bottle-ubuntu-test | tether | apt-get in Dockerfile |
| rbev-bottle-anthropic-jupyter | tether | pip install in Dockerfile |
| rbev-ubu-safety | tether | apt-get in Dockerfile |
| rbev-busybox-airgap-negative-canary | airgap | NEW: no anchor, designed to fail — permanent regression sentinel |

## Code Changes
1. `rbgc_Constants.sh` — add RBGC_POOL_SUFFIX_TETHER / _AIRGAP
2. `.rbk/rbrr.env` — rename RBRR_GCB_WORKER_POOL → RBRR_GCB_POOL_STEM (value: `rbw-depot10040-pool`, will gain suffixes on next depot create)
3. `rbrr_regime.sh` — new validation for stem format (no full path)
4. `rbdc_DerivedConstants.sh` — derive RBDC_POOL_TETHER / RBDC_POOL_AIRGAP from stem + project + region
5. `rbrv_regime.sh` — add RBRV_EGRESS_MODE (tether|airgap) validation
6. All 8 existing vessel `rbrv.env` files — add RBRV_EGRESS_MODE per assignment table
7. Create `rbev-vessels/rbev-busybox-airgap-negative-canary/` — rbrv.env (airgap, no anchor) + Dockerfile (FROM $RBF_IMAGE_1)
8. `rbf_Foundry.sh` — pool routing: conjure/bind use RBRV_EGRESS_MODE, about always RBDC_POOL_AIRGAP, vouch always RBDC_POOL_AIRGAP, inscribe always RBDC_POOL_TETHER, enshrine always RBDC_POOL_TETHER
9. `rbgp_Payor.sh` — create both pools (tether with default egress, airgap with NO_PUBLIC_EGRESS networkConfig), delete both pools
10. `rbgm_ManualProcedures.sh` — dual pool troubleshooting guidance

## Spec Changes
11. `RBS0-SpecTop.adoc` — mapping section (new linked terms for RBRV_EGRESS_MODE, RBRR_GCB_POOL_STEM, RBDC_POOL_*), variable definitions (~line 2488), vouch pool constraint (~line 1649), operation pool routing table
12. `RBSRR-RegimeRepo.adoc` — RBRR_GCB_POOL_STEM definition replacing RBRR_GCB_WORKER_POOL
13. `RBSRV-RegimeVessel.adoc` — add RBRV_EGRESS_MODE variable definition
14. `RBSDC-depot_create.adoc` — dual pool creation procedure
15. `RBSDD-depot_destroy.adoc` — dual pool deletion procedure
16. `RBSCB-CloudBuildPosture.adoc` — rewrite Current Posture (dual pools, operation routing) and Egress Lockdown (now implemented, no longer potential)
17. `RBSHR-HorizonRoadmap.adoc` — graduate egress lockdown from deferred
18. `RBSAB-ark_about.adoc` — always-airgap pool routing
19. `RBSAV-ark_vouch.adoc` — always-airgap pool routing
20. `RBSAC-ark_conjure.adoc` — vessel-routed pool
21. `RBSQB-quota_build.adoc` — dual pool capacity considerations

## Verification
- `tt/buw-rer.RenderEnvironmentRegime.sh` renders without error (derived constants)
- `tt/buw-rev.ValidateEnvironmentRegime.sh` passes
- `tt/rbw-rvv.ValidateVesselRegime.sh` passes for all vessels including canary
- `tt/rbw-rrv.ValidateRepoRegime.sh` passes with new stem variable
- No references to RBRR_GCB_WORKER_POOL remain (grep clean)
- All spec cross-references consistent

Note: No live depot exists during this pace. Regime validates structurally but pool paths won't resolve until ₢AvAAH creates the new depot.

**[260327-0720] rough**

## Character
Architectural implementation — cohesive but wide-reaching. Every change serves one goal: vessel-level egress routing via dual pools. Requires careful attention to spec/code consistency across ~20 files, but each individual change is mechanical.

## Goal
Implement dual-pool architecture (tether/airgap) with vessel-level egress mode selection. Normalize RBRR pool regime variable to eliminate duplication. All code and spec changes — no live GCP operations.

## Design Decisions
- Depot gets two pools: `{stem}-tether` (public egress) and `{stem}-airgap` (NO_PUBLIC_EGRESS)
- `RBRR_GCB_WORKER_POOL` replaced by `RBRR_GCB_POOL_STEM` (just the pool base name)
- Full pool paths derived at kindle time: `RBDC_POOL_TETHER` / `RBDC_POOL_AIRGAP`
- Vessel declares `RBRV_EGRESS_MODE=tether|airgap`
- Foundry reads vessel egress mode, selects derived pool constant
- Pool suffixes: `-tether` (public) / `-airgap` (NO_PUBLIC_EGRESS)

## Code Changes
1. `rbgc_Constants.sh` — add RBGC_POOL_SUFFIX_TETHER / _AIRGAP
2. `.rbk/rbrr.env` — rename RBRR_GCB_WORKER_POOL → RBRR_GCB_POOL_STEM
3. `rbrr_regime.sh` — new validation for stem format (no full path)
4. `rbdc_DerivedConstants.sh` — derive RBDC_POOL_TETHER / RBDC_POOL_AIRGAP from stem + project + region
5. `rbrv_regime.sh` — add RBRV_EGRESS_MODE (tether|airgap) validation
6. All 8 vessel `rbrv.env` files — add RBRV_EGRESS_MODE
7. `rbf_Foundry.sh` — 7+ sites: resolve pool from RBRV_EGRESS_MODE → derived constant
8. `rbgp_Payor.sh` — create both pools (tether with default egress, airgap with NO_PUBLIC_EGRESS), delete both pools
9. `rbgm_ManualProcedures.sh` — dual pool troubleshooting guidance

## Spec Changes
10. `RBS0-SpecTop.adoc` — mapping section (new linked terms), variable definitions (~line 2488), vouch pool constraint (~line 1649)
11. `RBSRR-RegimeRepo.adoc` — RBRR_GCB_POOL_STEM definition replacing RBRR_GCB_WORKER_POOL
12. `RBSRV-RegimeVessel.adoc` — add RBRV_EGRESS_MODE variable definition
13. `RBSDC-depot_create.adoc` — dual pool creation procedure
14. `RBSDD-depot_destroy.adoc` — dual pool deletion procedure
15. `RBSCB-CloudBuildPosture.adoc` — rewrite Current Posture (dual pools) and Egress Lockdown (no longer potential)
16. `RBSHR-HorizonRoadmap.adoc` — graduate egress lockdown from deferred
17. `RBSAB-ark_about.adoc`, `RBSAV-ark_vouch.adoc`, `RBSAC-ark_conjure.adoc`, `RBSQB-quota_build.adoc` — pool reference updates

## Verification
- `tt/buw-rer.RenderEnvironmentRegime.sh` renders without error (derived constants)
- `tt/buw-rev.ValidateEnvironmentRegime.sh` passes
- `tt/rbw-rvv.ValidateVesselRegime.sh` passes for all vessels
- `tt/rbw-rrv.ValidateRepoRegime.sh` passes with new stem variable
- No references to RBRR_GCB_WORKER_POOL remain (grep clean)
- All spec cross-references consistent

**[260327-0720] rough**

## Character
Architectural implementation — cohesive but wide-reaching. Every change serves one goal: vessel-level egress routing via dual pools. Requires careful attention to spec/code consistency across ~20 files, but each individual change is mechanical.

## Goal
Implement dual-pool architecture (tether/airgap) with vessel-level egress mode selection. Normalize RBRR pool regime variable to eliminate duplication. All code and spec changes — no live GCP operations.

## Design Decisions
- Depot gets two pools: `{stem}-tether` (public egress) and `{stem}-airgap` (NO_PUBLIC_EGRESS)
- `RBRR_GCB_WORKER_POOL` replaced by `RBRR_GCB_POOL_STEM` (just the pool base name)
- Full pool paths derived at kindle time: `RBDC_POOL_TETHER` / `RBDC_POOL_AIRGAP`
- Vessel declares `RBRV_EGRESS_MODE=tether|airgap`
- Foundry reads vessel egress mode, selects derived pool constant
- Pool suffixes: `-tether` (public) / `-airgap` (NO_PUBLIC_EGRESS)

## Code Changes
1. `rbgc_Constants.sh` — add RBGC_POOL_SUFFIX_TETHER / _AIRGAP
2. `.rbk/rbrr.env` — rename RBRR_GCB_WORKER_POOL → RBRR_GCB_POOL_STEM
3. `rbrr_regime.sh` — new validation for stem format (no full path)
4. `rbdc_DerivedConstants.sh` — derive RBDC_POOL_TETHER / RBDC_POOL_AIRGAP from stem + project + region
5. `rbrv_regime.sh` — add RBRV_EGRESS_MODE (tether|airgap) validation
6. All 8 vessel `rbrv.env` files — add RBRV_EGRESS_MODE
7. `rbf_Foundry.sh` — 7+ sites: resolve pool from RBRV_EGRESS_MODE → derived constant
8. `rbgp_Payor.sh` — create both pools (tether with default egress, airgap with NO_PUBLIC_EGRESS), delete both pools
9. `rbgm_ManualProcedures.sh` — dual pool troubleshooting guidance

## Spec Changes
10. `RBS0-SpecTop.adoc` — mapping section (new linked terms), variable definitions (~line 2488), vouch pool constraint (~line 1649)
11. `RBSRR-RegimeRepo.adoc` — RBRR_GCB_POOL_STEM definition replacing RBRR_GCB_WORKER_POOL
12. `RBSRV-RegimeVessel.adoc` — add RBRV_EGRESS_MODE variable definition
13. `RBSDC-depot_create.adoc` — dual pool creation procedure
14. `RBSDD-depot_destroy.adoc` — dual pool deletion procedure
15. `RBSCB-CloudBuildPosture.adoc` — rewrite Current Posture (dual pools) and Egress Lockdown (no longer potential)
16. `RBSHR-HorizonRoadmap.adoc` — graduate egress lockdown from deferred
17. `RBSAB-ark_about.adoc`, `RBSAV-ark_vouch.adoc`, `RBSAC-ark_conjure.adoc`, `RBSQB-quota_build.adoc` — pool reference updates

## Verification
- `tt/buw-rer.RenderEnvironmentRegime.sh` renders without error (derived constants)
- `tt/buw-rev.ValidateEnvironmentRegime.sh` passes
- `tt/rbw-rvv.ValidateVesselRegime.sh` passes for all vessels
- `tt/rbw-rrv.ValidateRepoRegime.sh` passes with new stem variable
- No references to RBRR_GCB_WORKER_POOL remain (grep clean)
- All spec cross-references consistent

**[260323-1034] rough**

## Character
High-stakes infrastructure change with end-to-end verification. The culmination of the entire heat.

## Goal
Enable NO_PUBLIC_EGRESS on the private pool worker and verify the full pipeline works air-gapped: inscribe (already done), enshrine base images, conjure with reliquary + enshrined base images, DSSE vouch — all without public internet access from the build worker.

## Steps
1. Ensure reliquary is inscribed (₢AvAAK prerequisite)
2. Ensure base images are enshrined for test vessels (₢AvAAJ prerequisite)
3. Update private pool config: set egressOption to NO_PUBLIC_EGRESS
4. Conjure rbev-busybox (FROM scratch, no base image) — verify success
5. Conjure a vessel with enshrined base image — verify success
6. Verify vouch (DSSE) works under lockdown (Container Analysis API is a Google API, should work via Private Google Access)
7. Test failure mode: attempt conjure with unenshrined base image — confirm clean failure

## Verification
- Private pool has NO_PUBLIC_EGRESS set
- Conjure succeeds for vessels with all dependencies mirrored
- Conjure fails cleanly for vessels with unmirrored dependencies
- Full pipeline (inscribe → enshrine → conjure → vouch) works air-gapped
- RBSHR egress lockdown item can be graduated from roadmap

### gcb-python-wedge (₢AvAAW) [complete]

**[260327-1952] complete**

## Character
Careful rewrite requiring attention to binary data handling and subprocess orchestration. Three GCB step scripts replace jq with Python 3 (json module, preinstalled in gcloud reliquary image). One script partially eliminated.

## Goal
Eliminate all jq and runtime package-install dependencies from GCB step scripts so about/vouch pipelines work on the airgap pool (NO_PUBLIC_EGRESS).

## Scripts to Rewrite
1. `Tools/rbk/rbgja/rbgja01-discover-platforms.sh` → `.py` — Parse OCI manifests, platform discovery, diags extraction. 11 jq calls. Image: gcloud. Currently `apt-get install jq`.
2. `Tools/rbk/rbgja/rbgja03-build-info-per-platform.sh` → `.py` — Generate per-platform build_info JSON for conjure/bind/graft modes. 3 `jq -n` calls. Image: alpine → switch to gcloud. Currently `apk add jq`.
3. `Tools/rbk/rbgjv/rbgjv02-verify-provenance.sh` → `.py` — DSSE PAE construction, base64 url-safe decoding, vouch summary composition. ~20 jq calls across conjure/bind/graft modes. Image: gcloud. Currently uses static jq binary downloaded by step 01. Note: openssl subprocess call for ECDSA signature verification is preserved as-is (independent of jq, already in gcloud image).

## Script to Rewrite (partial)
4. `Tools/rbk/rbgjv/rbgjv01-download-verifier.sh` — Remove jq download (wget from github.com). **Retain** the GCB attestor public key provisioning logic (writes ECDSA PEM to /workspace/keys/). Image stays alpine (key write is pure shell, no jq).

## Foundry Changes
- Verify GCB `script` field shebang handling works for `#!/usr/bin/env python3`
- Update step image assignment: rbgja03 switches from alpine to gcloud
- Confirmed: gcloud image has Python 3.10 + json module + OpenSSL 3.0

## Python Considerations
- openssl ECDSA verify via `subprocess.run` — not changed, just called from Python instead of bash
- base64 url-safe → standard conversion (jq scripts used `tr '_-' '/+'`; Python has `base64.urlsafe_b64decode`)
- Binary PAE construction (DSSEv1 prefix + payload) — Python bytes handle this cleanly
- All scripts are standalone .py files in flat directories (same pattern as .sh)

## Verification
- Airgap conjure (busybox) completes including about pipeline
- Standalone vouch completes on airgap pool
- Tether conjure (sentry/bottle) still works

**[260327-1914] rough**

## Character
Careful rewrite requiring attention to binary data handling and subprocess orchestration. Three GCB step scripts replace jq with Python 3 (json module, preinstalled in gcloud reliquary image). One script partially eliminated.

## Goal
Eliminate all jq and runtime package-install dependencies from GCB step scripts so about/vouch pipelines work on the airgap pool (NO_PUBLIC_EGRESS).

## Scripts to Rewrite
1. `Tools/rbk/rbgja/rbgja01-discover-platforms.sh` → `.py` — Parse OCI manifests, platform discovery, diags extraction. 11 jq calls. Image: gcloud. Currently `apt-get install jq`.
2. `Tools/rbk/rbgja/rbgja03-build-info-per-platform.sh` → `.py` — Generate per-platform build_info JSON for conjure/bind/graft modes. 3 `jq -n` calls. Image: alpine → switch to gcloud. Currently `apk add jq`.
3. `Tools/rbk/rbgjv/rbgjv02-verify-provenance.sh` → `.py` — DSSE PAE construction, base64 url-safe decoding, vouch summary composition. ~20 jq calls across conjure/bind/graft modes. Image: gcloud. Currently uses static jq binary downloaded by step 01. Note: openssl subprocess call for ECDSA signature verification is preserved as-is (independent of jq, already in gcloud image).

## Script to Rewrite (partial)
4. `Tools/rbk/rbgjv/rbgjv01-download-verifier.sh` — Remove jq download (wget from github.com). **Retain** the GCB attestor public key provisioning logic (writes ECDSA PEM to /workspace/keys/). Image stays alpine (key write is pure shell, no jq).

## Foundry Changes
- Verify GCB `script` field shebang handling works for `#!/usr/bin/env python3`
- Update step image assignment: rbgja03 switches from alpine to gcloud
- Confirmed: gcloud image has Python 3.10 + json module + OpenSSL 3.0

## Python Considerations
- openssl ECDSA verify via `subprocess.run` — not changed, just called from Python instead of bash
- base64 url-safe → standard conversion (jq scripts used `tr '_-' '/+'`; Python has `base64.urlsafe_b64decode`)
- Binary PAE construction (DSSEv1 prefix + payload) — Python bytes handle this cleanly
- All scripts are standalone .py files in flat directories (same pattern as .sh)

## Verification
- Airgap conjure (busybox) completes including about pipeline
- Standalone vouch completes on airgap pool
- Tether conjure (sentry/bottle) still works

**[260327-1913] rough**

## Character
Careful rewrite requiring attention to binary data handling and subprocess orchestration. Three GCB step scripts replace jq with Python 3 (json module, preinstalled in gcloud reliquary image). One script partially eliminated.

## Goal
Eliminate all jq and runtime package-install dependencies from GCB step scripts so about/vouch pipelines work on the airgap pool (NO_PUBLIC_EGRESS).

## Scripts to Rewrite
1. `Tools/rbk/rbgja/rbgja01-discover-platforms.sh` → `.py` — Parse OCI manifests, platform discovery, diags extraction. 11 jq calls. Image: gcloud. Currently `apt-get install jq`.
2. `Tools/rbk/rbgja/rbgja03-build-info-per-platform.sh` → `.py` — Generate per-platform build_info JSON for conjure/bind/graft modes. 3 `jq -n` calls. Image: alpine → switch to gcloud. Currently `apk add jq`.
3. `Tools/rbk/rbgjv/rbgjv02-verify-provenance.sh` → `.py` — DSSE PAE construction, base64 url-safe decoding, openssl signature verification via subprocess, vouch summary composition. ~20 jq calls across conjure/bind/graft modes. Image: gcloud. Currently uses static jq binary downloaded by step 01.

## Script to Rewrite (partial)
4. `Tools/rbk/rbgjv/rbgjv01-download-verifier.sh` — Remove jq download (wget from github.com). **Retain** the GCB attestor public key provisioning logic (writes ECDSA PEM to /workspace/keys/). Image stays alpine (key write is pure shell, no jq).

## Foundry Changes
- Verify GCB `script` field shebang handling works for `#!/usr/bin/env python3`
- Update step image assignment: rbgja03 switches from alpine to gcloud
- Confirmed: gcloud image has Python 3.10 + json module + OpenSSL 3.0

## Python Considerations
- openssl invocations via `subprocess.run` (DSSE verify in rbgjv02)
- base64 url-safe → standard conversion (jq did `tr '_-' '/+'`; Python has `base64.urlsafe_b64decode`)
- Binary PAE construction (DSSEv1 prefix + payload) — Python bytes handle this cleanly
- All scripts are standalone .py files in flat directories (same pattern as .sh)

## Verification
- Airgap conjure (busybox) completes including about pipeline
- Standalone vouch completes on airgap pool
- Tether conjure (sentry/bottle) still works

**[260327-1911] rough**

## Character
Mechanical rewrite with judgment on Python idioms. Three GCB step scripts need jq replaced with Python 3 (available in gcloud reliquary image). One script eliminated.

## Goal
Eliminate all jq dependencies from GCB step scripts so about/vouch pipelines work on the airgap pool (NO_PUBLIC_EGRESS). Replace with Python 3 using the json module, which is preinstalled in the gcloud reliquary image.

## Scripts to Rewrite
1. `Tools/rbk/rbgja/rbgja01-discover-platforms.sh` → `.py` — Parse OCI manifests (11 jq calls). Image: gcloud. Currently apt-get installs jq.
2. `Tools/rbk/rbgja/rbgja03-build-info-per-platform.sh` → `.py` — Generate build_info JSON (3 jq -n calls). Image: alpine → switch to gcloud. Currently apk adds jq.
3. `Tools/rbk/rbgjv/rbgjv02-verify-provenance.sh` → `.py` — DSSE envelope parsing + openssl orchestration. Image: gcloud. Currently uses static jq binary from github.com.

## Script to Eliminate
4. `Tools/rbk/rbgjv/rbgjv01-download-verifier.sh` — Downloads static jq from github.com. No longer needed.

## Foundry Changes
- Verify script field shebang handling works for .py files (or adapt)
- Update step image assignment: rbgja03 switches from alpine to gcloud
- Remove rbgjv01 step from vouch build assembly

## Verification
- Airgap conjure (busybox) completes with about pipeline
- Standalone vouch completes on airgap pool
- Tether conjure (sentry/bottle) still works

### end-to-end-dual-pool-depot-verification (₢AvAAH) [complete]

**[260327-2118] complete**

## Character
End-to-end verification following the onboarding guide (rbgm level sequence). Hands-on, sequential. Resuming after ₢AvAAW Python wedge fix unblocked airgap about/vouch steps.

## Context
Depot10041 exists with dual pools (tether=PUBLIC_EGRESS, airgap=NO_PUBLIC_EGRESS). Governor/Director/Retriever provisioned. Reliquary r260327172456 inscribed. Busybox enshrined (anchor busybox-latest-1487d0af5f). Prior conjure attempt: image build succeeded on airgap but about failed (jq blocked by NO_PUBLIC_EGRESS). ₢AvAAW rewrote GCB scripts to Python 3, fixing the blocker.

## Method
Follow the onboarding guide levels sequentially. The guide is the source of truth for operation order — the docket tracks progress through its levels, not a separate step list.

## Remaining Work
1. Locate current position in onboarding guide (level files in rbgm)
2. Resume from the level where busybox conjure+about needs retry with Python scripts
3. Walk through remaining onboarding levels: busybox full pipeline, sentry enshrine+conjure, bottle vessels, deploy, test
4. Negative canary test (airgap without anchor)
5. Full ark artifact verification
6. Graduate RBSHR egress lockdown from roadmap

## Verification
- All onboarding levels complete on depot10041
- Airgap pool: busybox conjure+about+vouch succeed with Python scripts
- Tether pool: sentry conjure succeeds with public internet
- Airgap pool: about/vouch succeed for BOTH vessels (always-airgap routing)
- All ark artifacts present
- RBSHR egress lockdown graduated

**[260327-2012] rough**

## Character
End-to-end verification following the onboarding guide (rbgm level sequence). Hands-on, sequential. Resuming after ₢AvAAW Python wedge fix unblocked airgap about/vouch steps.

## Context
Depot10041 exists with dual pools (tether=PUBLIC_EGRESS, airgap=NO_PUBLIC_EGRESS). Governor/Director/Retriever provisioned. Reliquary r260327172456 inscribed. Busybox enshrined (anchor busybox-latest-1487d0af5f). Prior conjure attempt: image build succeeded on airgap but about failed (jq blocked by NO_PUBLIC_EGRESS). ₢AvAAW rewrote GCB scripts to Python 3, fixing the blocker.

## Method
Follow the onboarding guide levels sequentially. The guide is the source of truth for operation order — the docket tracks progress through its levels, not a separate step list.

## Remaining Work
1. Locate current position in onboarding guide (level files in rbgm)
2. Resume from the level where busybox conjure+about needs retry with Python scripts
3. Walk through remaining onboarding levels: busybox full pipeline, sentry enshrine+conjure, bottle vessels, deploy, test
4. Negative canary test (airgap without anchor)
5. Full ark artifact verification
6. Graduate RBSHR egress lockdown from roadmap

## Verification
- All onboarding levels complete on depot10041
- Airgap pool: busybox conjure+about+vouch succeed with Python scripts
- Tether pool: sentry conjure succeeds with public internet
- Airgap pool: about/vouch succeed for BOTH vessels (always-airgap routing)
- All ark artifacts present
- RBSHR egress lockdown graduated

**[260327-0745] rough**

## Character
End-to-end integration verification requiring a live GCP depot. Hands-on, sequential, high-confidence. Single depot creation exercises the entire ℱAv implementation including dual-pool architecture.

## Goal
Create depot with dual-pool infrastructure, then verify the complete pipeline: inscribe, enshrine, conjure on both tether and airgap pools, about (always airgap), vouch (always airgap), and negative canary test.

## Prerequisites
- Depot10040 destroyed before ₢AvAAQ began (old single-pool depot is gone)
- ₢AvAAQ (rbrg dead code removal) complete
- ₢AvAAL (dual-pool-regime-normalization) complete

## Steps
1. Create fresh depot — confirm tether + airgap pools created with correct egress config (diagnostic gcloud to verify egressOption on each pool)
2. Governor reset, create director/retriever — confirm IAM covers both pools (workerPoolUser at project scope)
3. Inscribe reliquary on tether pool — verify tool images land in GAR
4. Enshrine busybox base image (busybox:latest) on tether pool — verify anchor written to rbev-busybox rbrv.env
5. Enshrine sentry base image (ubuntu:24.04) on tether pool — verify anchor written
6. Conjure rbev-busybox on airgap pool (RBRV_EGRESS_MODE=airgap, enshrined anchor) — all deps from GAR, no public internet
7. About for busybox — runs on airgap pool (always-airgap routing)
8. Vouch for busybox — runs on airgap pool, DSSE verification via Private Google Access
9. Conjure rbev-sentry-ubuntu-large on tether pool (RBRV_EGRESS_MODE=tether, apt-get needs internet) — verify success
10. About + vouch for sentry — about/vouch run on airgap pool despite vessel being tether
11. Conjure rbev-busybox-airgap-negative-canary on airgap pool — confirm clean failure (no anchor, Docker Hub unreachable)
12. Full ark artifact verification (image, about, vouch) for busybox and sentry
13. Verify regime renders: pool stem in rbrr.env, derived constants resolve, vessel egress modes valid

## Verification
- Depot lifecycle clean of all eliminated code (no GitLab, no triggers, no RBRG binary pins)
- Two pools visible in depot with correct egress settings (diagnostic gcloud)
- Airgap pool: busybox conjure succeeds with all deps mirrored
- Tether pool: sentry conjure succeeds with public internet access
- Airgap pool: about/vouch succeed for BOTH vessels (always-airgap routing proven)
- Airgap pool: canary conjure fails cleanly (Docker Hub unreachable)
- All ark artifacts present for busybox and sentry
- RBSHR egress lockdown graduated from roadmap
- Full pipeline verified end-to-end with all ℱAv changes

**[260327-0721] rough**

## Character
End-to-end integration verification requiring a live GCP depot. Hands-on, sequential, high-confidence. Single depot recreation exercises the entire ℱAv implementation including dual-pool architecture.

## Goal
Destroy and recreate depot with dual-pool infrastructure, then verify the complete pipeline: inscribe, enshrine, conjure on both tether and airgap pools, vouch, and negative test.

## Prerequisites
- ¢AvAAL (dual-pool-regime-normalization) complete
- ¢AvAAQ (rbrg dead code removal) complete

## Steps
1. Destroy depot10040 — confirm both pools deleted cleanly
2. Create fresh depot — confirm tether + airgap pools created with correct egress config
3. Governor reset, create director/retriever — confirm IAM covers both pools
4. Inscribe reliquary — verify tool images land in GAR
5. Enshrine sentry base image (ubuntu:24.04) — verify anchor written
6. Conjure on tether pool: rbev-busybox (RBRV_EGRESS_MODE=tether, no anchor) — pulls busybox:latest from Docker Hub
7. Conjure on airgap pool: rbev-sentry-ubuntu-large (RBRV_EGRESS_MODE=airgap, enshrined anchor) — all deps from GAR
8. Vouch both consecrations — DSSE verification works on both pools
9. Negative test: temporarily set busybox to RBRV_EGRESS_MODE=airgap without anchor — confirm clean failure (Docker Hub unreachable)
10. Full ark artifact verification (image, about, vouch) across both pools
11. Verify regime renders: pool stem in rbrr.env, derived constants resolve, vessel egress modes valid

## Verification
- Depot lifecycle clean of all eliminated code (no GitLab, no triggers, no RBRG pins)
- Two pools visible in depot with correct egress settings (diagnostic gcloud)
- Tether pool: builds succeed with public internet access
- Airgap pool: builds succeed with all deps mirrored, no public egress
- Airgap pool: builds fail cleanly when deps not mirrored
- All ark artifacts present for both test vessels
- RBSHR egress lockdown graduated from roadmap
- Full pipeline verified end-to-end with all ℱAv changes

**[260327-0721] rough**

## Character
End-to-end integration verification requiring a live GCP depot. Hands-on, sequential, high-confidence. Single depot recreation exercises the entire ℱAv implementation including dual-pool architecture.

## Goal
Destroy and recreate depot with dual-pool infrastructure, then verify the complete pipeline: inscribe, enshrine, conjure on both tether and airgap pools, vouch, and negative test.

## Prerequisites
- ¢AvAAL (dual-pool-regime-normalization) complete
- ¢AvAAQ (rbrg dead code removal) complete

## Steps
1. Destroy depot10040 — confirm both pools deleted cleanly
2. Create fresh depot — confirm tether + airgap pools created with correct egress config
3. Governor reset, create director/retriever — confirm IAM covers both pools
4. Inscribe reliquary — verify tool images land in GAR
5. Enshrine sentry base image (ubuntu:24.04) — verify anchor written
6. Conjure on tether pool: rbev-busybox (RBRV_EGRESS_MODE=tether, no anchor) — pulls busybox:latest from Docker Hub
7. Conjure on airgap pool: rbev-sentry-ubuntu-large (RBRV_EGRESS_MODE=airgap, enshrined anchor) — all deps from GAR
8. Vouch both consecrations — DSSE verification works on both pools
9. Negative test: temporarily set busybox to RBRV_EGRESS_MODE=airgap without anchor — confirm clean failure (Docker Hub unreachable)
10. Full ark artifact verification (image, about, vouch) across both pools
11. Verify regime renders: pool stem in rbrr.env, derived constants resolve, vessel egress modes valid

## Verification
- Depot lifecycle clean of all eliminated code (no GitLab, no triggers, no RBRG pins)
- Two pools visible in depot with correct egress settings (diagnostic gcloud)
- Tether pool: builds succeed with public internet access
- Airgap pool: builds succeed with all deps mirrored, no public egress
- Airgap pool: builds fail cleanly when deps not mirrored
- All ark artifacts present for both test vessels
- RBSHR egress lockdown graduated from roadmap
- Full pipeline verified end-to-end with all ℱAv changes

**[260323-1035] rough**

## Character
End-to-end integration verification requiring a live GCP depot. Hands-on, sequential, high-confidence. Final pace of the heat.

## Goal
Destroy and recreate depot, reset governor, create director/retriever, and conjure builds for all service vessels to verify the complete ₣Av implementation: GitLab elimination, enshrine, reliquary, and (if ₢AvAAL completed) egress lockdown.

## Steps
1. Destroy current depot, create fresh depot — confirm no GitLab/SM/CBv2 steps
2. Reset governor, create director/retriever — confirm clean IAM grants
3. Conjure rbev-busybox (FROM scratch) — baseline verification
4. Conjure the four bottle/sentry service images (sentry-ubuntu-large, bottle-anthropic-jupyter, bottle-plantuml, bottle-ubuntu-test)
5. If reliquary implemented: verify conjure uses GAR tool images
6. If enshrine implemented: verify base image resolution from GAR
7. If egress lockdown enabled: verify air-gapped builds
8. Verify ark artifacts (image, about, vouch) present for all vessels

## Verification
- All depot lifecycle operations clean of eliminated code
- All service vessels build successfully
- Full pipeline verified end-to-end with all ₣Av changes

**[260323-0935] rough**

## Character
End-to-end integration verification requiring a live GCP depot. Hands-on, sequential, high-confidence.

## Goal
Destroy existing depot, recreate from scratch, reset governor, and carry out builds to verify all ₣Av changes (GitLab elimination, builds.create path, DSSE vouch) work end-to-end.

## Steps
1. Destroy current depot via depot_destroy tabtarget
2. Create new depot via depot_create — confirm no GitLab prompts, no Secret Manager steps, no CB v2 connection
3. Reset governor — confirm no CB v2 connectionViewer grant or verification
4. Run a conjure build on at least one vessel — verify builds.create + pouch + DSSE vouch pipeline
5. Verify ark artifacts (image, about, vouch) are present in GAR

## Verification
- Depot creation completes without GitLab/Secret Manager/CB v2 steps
- Governor reset completes without CB v2 verification
- At least one successful conjure build with DSSE-verified provenance

### rbrg-regime-elimination (₢AvAAV) [complete]

**[260327-2139] complete**

## Character
Cleanup — remove obsoleted infrastructure. Mechanical but wide-reaching (regime module, CLI, env file, tabtargets, zipper entries, furnish dependencies, spec references).

## Goal
Eliminate the RBRG regime entirely. With reliquaries required and inscribe owning the upstream-to-GAR mirroring, RBRG's role as authoritative tool image pin source is dead. The refresh command, regime module, env file, and all references should be removed.

## Scope
- `Tools/rbk/rbrg_cli.sh`: Delete entirely
- `Tools/rbk/rbrg_regime.sh`: Delete entirely
- `.rbk/rbrg.env`: Delete entirely
- `Tools/rbk/rbz_zipper.sh`: Remove RBZ_REFRESH_GCB_PINS entry and rbrg_cli.sh module assignment
- `tt/rbw-DPG.DirectorRefreshesGcbPins.sh`: Delete tabtarget
- `tt/rbw-rgv.ValidatePinsRegime.sh`: Delete tabtarget (if exists)
- `tt/rbw-rgr.RenderPinsRegime.sh`: Delete tabtarget (if exists)
- Any CLI that sources rbrg_regime.sh or rbrg.env in furnish: remove those source lines and the kindle/enforce calls
- Spec references: RBS0-SpecTop.adoc (rbtc_refresh_gcb_pins colophon, RBRG linked terms, RBRG variable definitions), RBSRG-RegimeGcbPins.adoc (assess: delete or gut), README.consumer.md, CLAUDE.consumer.md
- Grep for RBRG_ across codebase to find any remaining consumers

## Prerequisites
- AvAAH end-to-end verification confirms inscribe works independently of RBRG

## Verification
- No RBRG_ references remain outside retired history
- All remaining regime validations pass
- Inscribe still works (uses its own upstream manifest, not RBRG)

**[260327-0805] rough**

## Character
Cleanup — remove obsoleted infrastructure. Mechanical but wide-reaching (regime module, CLI, env file, tabtargets, zipper entries, furnish dependencies, spec references).

## Goal
Eliminate the RBRG regime entirely. With reliquaries required and inscribe owning the upstream-to-GAR mirroring, RBRG's role as authoritative tool image pin source is dead. The refresh command, regime module, env file, and all references should be removed.

## Scope
- `Tools/rbk/rbrg_cli.sh`: Delete entirely
- `Tools/rbk/rbrg_regime.sh`: Delete entirely
- `.rbk/rbrg.env`: Delete entirely
- `Tools/rbk/rbz_zipper.sh`: Remove RBZ_REFRESH_GCB_PINS entry and rbrg_cli.sh module assignment
- `tt/rbw-DPG.DirectorRefreshesGcbPins.sh`: Delete tabtarget
- `tt/rbw-rgv.ValidatePinsRegime.sh`: Delete tabtarget (if exists)
- `tt/rbw-rgr.RenderPinsRegime.sh`: Delete tabtarget (if exists)
- Any CLI that sources rbrg_regime.sh or rbrg.env in furnish: remove those source lines and the kindle/enforce calls
- Spec references: RBS0-SpecTop.adoc (rbtc_refresh_gcb_pins colophon, RBRG linked terms, RBRG variable definitions), RBSRG-RegimeGcbPins.adoc (assess: delete or gut), README.consumer.md, CLAUDE.consumer.md
- Grep for RBRG_ across codebase to find any remaining consumers

## Prerequisites
- AvAAH end-to-end verification confirms inscribe works independently of RBRG

## Verification
- No RBRG_ references remain outside retired history
- All remaining regime validations pass
- Inscribe still works (uses its own upstream manifest, not RBRG)

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 O enshrine-gcb-correction
  2 P bcg-sed-to-parameter-expansion
  3 R build-poll-label-parameter
  4 A slsa-provenance-builds-create-experiment
  5 B slsa-level3-builds-create-verification
  6 D gitlab-elimination-pouch-builds-create
  7 E spec-depot-lifecycle-gitlab-removal
  8 F payor-gitlab-dead-code-removal
  9 G spec-subdocument-gitlab-sweep
  10 I spec-base-image-enshrine-operation
  11 J implement-enshrine-core
  12 M wire-anchor-into-conjure-vouch
  13 S director-vessel-arg-cleanup
  14 T graft-combine-about-vouch-gcb-job
  15 K implement-reliquary-inscribe-tool-images
  16 N enshrine-end-to-end-verification
  17 Q rbrg-remove-slsa-verifier-dead-code
  18 L dual-pool-regime-normalization
  19 W gcb-python-wedge
  20 H end-to-end-dual-pool-depot-verification
  21 V rbrg-regime-elimination

OPRABDEFGIJMSTKNQLWHV
xxxx·x····xxxxx··xxxx rbf_Foundry.sh
······x·xxx··x·xxxx·x RBS0-SpecTop.adoc
······xx····x···xx·x· rbgm_ManualProcedures.sh
··········x·x·x·x···x rbz_zipper.sh
··············xx·x·x· rbrv.env
·············x···xx·x RBSAB-ark_about.adoc
···········x·····xx·x RBSAV-ark_vouch.adoc
········x·x·····x···x RBSRG-RegimeGcbPins.adoc
·····xx··········x·x· rbrr.env
···x·x············x·x rbgjv01-download-verifier.sh
···x·x·····x······x·· rbgjv02-verify-provenance.sh
x········xx·········x RBSAE-ark_enshrine.adoc
···············x·x·x· Dockerfile
············x···x···x CLAUDE.consumer.md, README.consumer.md
··········x···x··x··· rbrv_regime.sh
·········x·····x·x··· RBSAC-ark_conjure.adoc
········x········x··x RBSRR-RegimeRepo.adoc
······x··········x·x· rblm_cli.sh
·x··············x···x rbrg_cli.sh
·x····x·············x rbrg.env
··················xx· rbgja01-discover-platforms.py, rbgja03-build-info-per-platform.py, rbgjv02-verify-provenance.py
·················x··x rbtb_testbench.sh
·············x·····x· CLAUDE.md
···········x···x····· rbgjb03-buildx-push-multi.sh
··········x·x········ rbw-DE.DirectorEnshrinesBaseImages.sh
·········x·······x··· RBSRV-RegimeVessel.adoc
········x········x··· RBSCB-CloudBuildPosture.adoc, RBSDC-depot_create.adoc, RBSQB-quota_build.adoc
········x····x······· RBSAG-ark_graft.adoc
·······x·········x··· rbgp_Payor.sh
······x·············x rbrg_regime.sh
······x············x· rbgg_Governor.sh
······x··········x··· rbrr_regime.sh
x···················x rbgje01-enshrine-copy.sh
····················x RBSRI-rubric_inscribe.adoc, rbbc_constants.sh, rbf_cli.sh, rbgja02-syft-per-platform.sh, rbgjb02-qemu-binfmt.sh, rbgjm01-mirror-image.sh, rbgjv03-assemble-push-vouch.sh, rbk-prep-release.md, rbrr_cli.sh, rbw-DPG.DirectorRefreshesGcbPins.sh, rbw-rgr.RenderPinsRegime.sh, rbw-rgv.ValidatePinsRegime.sh
···················x· rbrn_nsproto.env, rbrn_pluml.env, rbrn_srjcl.env
··················x·· rbgja01-discover-platforms.sh, rbgja03-build-info-per-platform.sh
·················x··· RBSDD-depot_destroy.adoc, RBSHR-HorizonRoadmap.adoc, rbdc_DerivedConstants.sh, rbgc_Constants.sh, rbtcrv_RegimeValidation.sh
················x···· rbw-DPB.DirectorRefreshesBinaryPins.sh
··············x······ rbgji01-inscribe-mirror.sh, rbw-DI.DirectorInscribesReliquary.sh, rbw-DI.DirectorInscribesRubric.sh
·············x······· JJS0_JobJockeySpec.adoc, JJSCTL-tally.adoc, jjrgc_get_coronets.rs, jjrm_mcp.rs, jjrtl_tally.rs, vocjjmc_core.md
············x········ rbob_bottle.sh, rbtctm_ThreeMode.sh, rbw-DA.DirectorAbjuresArk.sh, rbw-DA.DirectorAbjuresConsecration.sh, rbw-DC.DirectorCreatesArk.sh, rbw-DC.DirectorCreatesConsecration.sh, rbw-DE.DirectorEnshrinesVessel.sh, rbw-Db.DirectorBuildsAbout.sh, rbw-Rs.RetrieverSummonsArk.sh, rbw-Rs.RetrieverSummonsConsecration.sh, rbw_workbench.sh
········x············ RBSCIP-IamPropagation.adoc, RBSCTD-CloudBuildTriggerDispatch.adoc, RBSDI-director_create.adoc, RBSGR-governor_reset.adoc, RBSGS-GettingStarted.adoc, RBSRC-retriever_create.adoc
·······x············· rbgu_Utility.sh
···x················· JJS0-GallopsData.adoc, JJSCGZ-gazette.adoc

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 126 commits)

  1 K implement-reliquary-inscribe-tool-images
  2 N enshrine-end-to-end-verification
  3 Q rbrg-remove-slsa-verifier-dead-code
  4 L dual-pool-regime-normalization
  5 H end-to-end-dual-pool-depot-verification
  6 W gcb-python-wedge
  7 V rbrg-regime-elimination

123456789abcdefghijklmnopqrstuvwxyz
x··································  K  1c
·x·································  N  1c
·······xx··························  Q  2c
·········xxx·······················  L  3c
············xxxxxx········xxxxxx···  H  12c
····················xxxxxx·········  W  6c
································xx·  V  2c
```

## Steeplechase

### 2026-03-27 21:45 - Heat - n

Add timing expectations to onboarding guide dashboard and step instructions. Durations measured from e2-standard-2 depot10041 run: depot create ~2min, inscribe ~6min, airgap busybox ~10min, bottle/sentry ~15min each.

### 2026-03-27 21:39 - ₢AvAAV - W

Eliminate RBRG regime entirely. Deleted rbrg_cli.sh, rbrg_regime.sh, .rbk/rbrg.env, RBSRG spec doc, and 3 tabtargets (DPG/rgr/rgv). Removed all source/kindle/enforce from furnish consumers (rbf_cli, rbrr_cli, rbtb_testbench). Renamed step script template markers from RBRG_* to ZRBF_TOOL_*. Updated all spec attribute references to plain text reliquary descriptions. Verified: airgap busybox conjure+about+vouch succeeded on depot10041 with no RBRG dependency.

### 2026-03-27 21:29 - ₢AvAAV - n

Eliminate RBRG regime entirely. Delete rbrg_cli.sh, rbrg_regime.sh, .rbk/rbrg.env, RBSRG spec, and 3 tabtargets. Remove all source/kindle/enforce from furnish consumers (rbf_cli, rbrr_cli, rbtb_testbench). Rename step script template markers from RBRG_* to ZRBF_TOOL_*. Update all spec attribute references to plain text reliquary descriptions.

### 2026-03-27 21:18 - ₢AvAAH - W

End-to-end dual-pool depot verification on depot10041. Fixed two Python wedge integration issues (GCB substitution anchor comments, consecration/BUILD_ID file fallback for combined conjure). Busybox conjure+about+vouch on airgap pool, bottle+sentry conjure+about+vouch on tether pool, both summoned locally, nsproto security tests 22/22 passed. Documented gazette wire format spook in CLAUDE.md.

### 2026-03-27 21:17 - ₢AvAAH - n

Document gazette wire format for setter commands in JJK CLAUDE.md section. Spook: agent didn't know # slug lede header format, burned a round-trip to JJSCGZ spec.

### 2026-03-27 21:09 - ₢AvAAH - n

Record bottle and sentry consecrations in nsproto nameplate. Both vessels summoned locally (image+about+vouch). Onboarding levels 11-12 complete.

### 2026-03-27 21:08 - ₢AvAAH - n

Enshrine ubuntu:24.04 anchors for bottle and sentry vessels (ubuntu-24.04-186072bba1). Conjure+about+vouch succeeded: bottle on tether (c260327203832-r260328034032), sentry on tether (c260327205505-r260328035705). Onboarding levels 9-10 complete.

### 2026-03-27 20:26 - ₢AvAAH - n

Fix Python about scripts for combined conjure context: consecration from .consecration file fallback (not available as substitution in combined builds), BUILD_ID from GCB built-in fallback. Both paths still work for standalone about builds via env var.

### 2026-03-27 20:24 - ₢AvAAH - n

Add GCB substitution anchor comments to Python scripts. GCB matcher requires ${_RBGA_*} syntax in step templates — Python os.environ[] not recognized. Comments provide match targets while automapSubstitutions delivers values at runtime.

### 2026-03-27 19:52 - ₢AvAAW - W

Rewrote three GCB step scripts from bash+jq to Python 3 for airgap NO_PUBLIC_EGRESS compatibility: rbgja01 (platform discovery), rbgja03 (build_info generation, alpine→gcloud), rbgjv02 (provenance verification). Replaced gcloud auth with GCE metadata server token fetch — gcloud surface narrowed to single provenance retrieval call (conjure only). Removed jq download from rbgjv01. Updated foundry for .py files, python3 entrypoint, prepare-keys step ID. Comprehensive spec sync: rbsk_no_gcloud exception for provenance fetch, vouch step architecture (3 steps), nested JSON structure for build_info and vouch_summary documented, RBRG_GCLOUD_IMAGE_REF description, RBSAB/RBSAV subdocuments updated.

### 2026-03-27 19:49 - ₢AvAAW - n

Fix pre-existing spec drift: document actual nested JSON structure for build_info.json (git/build/slsa/bind/graft objects, consecration field) and vouch_summary.json (verification object for bind/graft, per-platform verdict structure for conjure). Spec now matches implementation that was unchanged by the Python rewrite.

### 2026-03-27 19:46 - ₢AvAAW - n

Replace gcloud auth print-access-token with GCE metadata server token fetch in rbgja01 and rbgjv02. Narrows rbsk_no_gcloud exception to single call: gcloud artifacts docker images describe (provenance retrieval, conjure only). Update specs to reflect metadata server for all OAuth tokens.

### 2026-03-27 19:41 - ₢AvAAW - n

Spec sync for Python GCB rewrite: update rbsk_no_gcloud premise with about/vouch exception, rewrite vouch step architecture (3 steps: alpine keys, gcloud/Python verify, docker push), update RBSAV vouch operation (jq→Python, step renumbering), update RBSAB about step 1 and 3 (Python 3 notes, alpine→gcloud for build_info), update RBRG_GCLOUD_IMAGE_REF description, update rbbc_store definition

### 2026-03-27 19:34 - ₢AvAAW - n

Review fixes: pretty-print JSON output (indent=2) in all Python GCB scripts, add require_env to rbgjv02 for consistent error messages, version-guard tarfile extractall filter for Python 3.10 compat, rename vouch step from download-verifier to prepare-keys

### 2026-03-27 19:30 - ₢AvAAW - n

Rewrite three GCB step scripts from bash+jq to Python 3 (rbgja01 platform discovery, rbgja03 build_info generation, rbgjv02 provenance verification). Remove jq download from rbgjv01. Update foundry step definitions for .py files, python3 entrypoint/shebang, and rbgja03 image switch from alpine to gcloud. Eliminates all jq and runtime package-install dependencies for airgap NO_PUBLIC_EGRESS compatibility.

### 2026-03-27 19:18 - Heat - d

paddock curried

### 2026-03-27 19:11 - Heat - S

gcb-python-wedge

### 2026-03-27 18:37 - ₢AvAAH - n

Enshrine busybox-latest-1487d0af5f succeeded. Conjure on airgap pool: image build steps 0-7 succeeded (consecration c260327183013-r260328013315 in GAR), but about step 8 failed — gcloud image apt-get install jq blocked by NO_PUBLIC_EGRESS. Airgap-incompatible about step identified.

### 2026-03-27 18:23 - ₢AvAAH - n

Rewrite onboarding guide: 14 levels (0-13), airgap-first pedagogy (busybox→bottle→sentry→deploy→test), one file per level, vessel-specific probes. Fix bottle-ubuntu-test to use anchor pattern (ARG RBF_IMAGE_1/FROM, RBRV_IMAGE_1_ORIGIN).

### 2026-03-27 17:32 - ₢AvAAH - n

Fix foundry pool routing: remove vessel-scoped RBRV_EGRESS_MODE from universal kindle, route conjure pool locally in stitch, fix enshrine to always use tether pool. Inscribe reliquary r260327172456 succeeded. Set RBRV_RELIQUARY across all 9 vessels.

### 2026-03-27 17:18 - ₢AvAAH - n

Depot10041 created with dual pools verified (tether=PUBLIC_EGRESS, airgap=NO_PUBLIC_EGRESS). Governor/Director/Retriever provisioned. RBRR populated. Fix stale RBZ_RUBRIC_INSCRIBE zipper reference in onboarding guide.

### 2026-03-27 17:01 - ₢AvAAH - n

Eliminate stale GitLab/rubric references from onboarding guide (renumber 9 levels to 8, update inscribe/conjure guidance for reliquary/enshrine/DSSE architecture). Extend marshal reset to blank depot-scoped vessel fields (RBRV_RELIQUARY, RBRV_IMAGE_*_ANCHOR) across all 9 vessels.

### 2026-03-27 16:50 - ₢AvAAH - n

Marshal reset: blank depot fields, delete stale credentials, blank vessel consecrations for onboarding walkthrough

### 2026-03-27 08:37 - ₢AvAAL - W

Implement dual-pool architecture (tether/airgap) with vessel-level egress routing. Rename RBRR_GCB_WORKER_POOL to RBRR_GCB_POOL_STEM, derive RBDC_POOL_TETHER/AIRGAP. Add RBRV_EGRESS_MODE enum to all 9 vessels (8 existing + airgap negative canary). Foundry routes inscribe/enshrine to tether, about/vouch to airgap, conjure/bind per vessel mode. Payor creates/deletes dual pools with NO_PUBLIC_EGRESS on airgap. All 21 regime-validation test cases pass. Full spec coverage: RBSCB posture rewrite (egress lockdown implemented), RBSHR graduated, RBSDD/RBSDC/RBSRV/RBSAC/RBSQB/RBSAB/RBSAV/RBSRR/RBS0 updated.

### 2026-03-27 08:34 - ₢AvAAL - n

Complete spec coverage for dual-pool architecture: RBSDD dual pool deletion, RBSCB posture rewrite (egress lockdown implemented not potential), RBSHR graduate egress lockdown, RBSRV add RBRV_EGRESS_MODE variable, RBSAC vessel-routed pool note, RBSQB dual pool quota note.

### 2026-03-27 08:27 - ₢AvAAL - n

Implement dual-pool architecture (tether/airgap) with vessel-level egress routing. Rename RBRR_GCB_WORKER_POOL to RBRR_GCB_POOL_STEM, derive RBDC_POOL_TETHER/AIRGAP full paths. Add RBRV_EGRESS_MODE to all 9 vessels. Foundry routes inscribe/enshrine to tether, about/vouch to airgap, conjure/bind per vessel mode. Payor creates/deletes dual pools with NO_PUBLIC_EGRESS on airgap. Create airgap negative canary vessel. All 21 regime-validation test cases pass.

### 2026-03-27 08:08 - ₢AvAAQ - W

Removed all slsa-verifier binary pin infrastructure: deleted rbrg_refresh_binary_pins function and ZRBRG_BINARY_LINES/ZRBRG_BINARY_PINS_REFRESHED_AT state from rbrg_cli.sh, removed DPB zipper entry and tabtarget, cleaned binary pin references from rbgm_ManualProcedures.sh (renumbered steps), RBS0-SpecTop.adoc, RBSRG-RegimeGcbPins.adoc, README.consumer.md, and CLAUDE.consumer.md. Slated follow-on ₢AvAAV for full RBRG regime elimination.

### 2026-03-27 08:08 - ₢AvAAQ - n

Eliminate slsa-verifier binary pin infrastructure from RBRG regime, removing refresh command, tabtarget, zipper enrollment, and all documentation references to simplify pin management to GCB container image pins only.

### 2026-03-27 08:05 - Heat - S

rbrg-regime-elimination

### 2026-03-27 07:54 - Heat - S

claudemd-firemark-coronet-case-sensitivity

### 2026-03-27 07:21 - Heat - T

end-to-end-dual-pool-depot-verification

### 2026-03-27 07:20 - Heat - T

dual-pool-regime-normalization

### 2026-03-27 07:20 - Heat - r

moved AvAAQ to first

### 2026-03-26 21:02 - ₢AvAAN - W

Enshrine end-to-end verified: vouch artifact for c260326203841-r260327034113 records anchored provenance (_RBGV_IMAGE_1_PROVENANCE=anchored, full GAR path with anchor tag ubuntu-24.04-186072bba1). All 5 docket verification points confirmed — no new code changes needed, ₢AvAAK work already satisfied this pace.

### 2026-03-26 20:57 - ₢AvAAK - W

Migrate all GCB step assembly from entrypoint+args to script field. Remove dollar-escaping (incompatible with script field), add automapSubstitutions for substitution-to-env-var mapping, fix BUILD_ID gsub to target .script. Add RBRV_RELIQUARY=r260324201411 to all 8 vessels (universal reliquary). Conjure end-to-end proven: c260326203841-r260327034113 on rbev-sentry-ubuntu-large.

### 2026-03-26 20:38 - ₢AvAAK - n

Fix conjure BUILD_ID gsub: add elif .script branch to jq post-processing that bakes $BUILD_ID into about steps. The gsub only targeted .args (now extinct); .script steps were silently skipped, leaving ${_RBGA_BUILD_ID} unset.

### 2026-03-25 08:00 - ₢AvAAK - n

Fix conjure about post-processing: remove $$ escaping from $(cat .consecration) — script field does not de-escape $$ like args did. First conjure test revealed $$ became PID (1) instead of literal $.

### 2026-03-25 07:39 - ₢AvAAK - n

Migrate all GCB step assembly from entrypoint+args to script field with shebang prefix and automapSubstitutions. Remove dollar-escaping (incompatible with script field where $$ is shell PID). Add RBRV_RELIQUARY=r260324201411 to all 8 vessels (universal reliquary). Update paddock with settled design decisions.

### 2026-03-24 20:33 - ₢AvAAK - n

Drop oras from reliquary (not used as GCB step image), remove kindle oras validation, update sentry vessel with reliquary r260324201411 and enshrine anchor ubuntu-24.04-186072bba1. Inscribe and enshrine proven end-to-end.

### 2026-03-24 20:02 - ₢AvAAK - n

Remove old rubric inscribe tabtarget (replaced by DirectorInscribesReliquary)

### 2026-03-24 20:01 - ₢AvAAK - n

Implement reliquary inscribe operation and rewire all tool image references from RBRG digest pins to reliquary GAR paths. Add rbf_inscribe/zrbf_inscribe_submit (GCB job using gcr.io/cloud-builders/docker), zrbf_resolve_tool_images (required reliquary, no RBRG fallback), RBRV_RELIQUARY vessel regime variable (non-gated, all modes). Replace 19 RBRG_*_IMAGE_REF usages in stitch/about/vouch/enshrine/mirror with ZRBF_TOOL_* resolved refs.

### 2026-03-24 19:37 - Heat - r

moved AvAAK to first

### 2026-03-24 19:28 - ₢AvAAT - W

Combined graft about+vouch into single GCB submission, eliminating orphan gap. Extracted zrbf_assemble_vouch_steps helper (parallels about helper), added zrbf_graft_metadata_submit with merged _RBGA_/_RBGV_ substitutions. Rewired rbf_create so graft uses one cloud job (was two). Updated specs RBS0, RBSAB, RBSAG. Standalone rbf_about/rbf_vouch retained for manual recovery.

### 2026-03-24 19:27 - ₢AvAAT - n

Combine graft about+vouch into single GCB submission. Extract zrbf_assemble_vouch_steps helper, add zrbf_graft_metadata_submit with merged _RBGA_/_RBGV_ substitutions. Rewire rbf_create graft path. Update specs (RBS0, RBSAB, RBSAG) to reflect combined job.

### 2026-03-24 19:14 - Heat - r

moved AvAAT before AvAAN

### 2026-03-24 19:13 - ₢AvAAS - W

Extracted zrbf_resolve_vessel BCG-compliant helper (accepts sigil or path, writes to temp file, lists vessels on error). Replaced duplicated vessel-arg validation in rbf_enshrine, rbf_create, rbf_abjure, rbf_about. Renamed Ark→Consecration across all tabtarget frontispieces (DC, DA, Rs, DE→Vessel), zipper constants, user-facing Foundry messages, and consumer docs. Deleted rbw-Db tabtarget (about is automatic). Renamed RBZ_SUMMON_ARK in rbob_bottle. Regime validation passes (21 cases).

### 2026-03-24 19:12 - Heat - S

graft-combine-about-vouch-gcb-job

### 2026-03-24 19:08 - ₢AvAAS - n

Remove old tabtarget files (renamed to Consecration/Vessel variants)

### 2026-03-24 19:07 - ₢AvAAS - n

Extract zrbf_resolve_vessel (BCG-compliant, accepts sigil or path, lists on error). Replace duplicated vessel-arg blocks in 4 Foundry functions. Rename Ark to Consecration in tabtarget frontispieces (DC, DA, Rs, DE), zipper constants, user-facing messages, and consumer docs. Delete rbw-Db tabtarget. Regime validation passes (21 cases).

### 2026-03-24 18:53 - Heat - S

director-vessel-arg-cleanup

### 2026-03-24 18:43 - ₢AvAAN - n

Prepare sentry vessel for enshrine: add RBRV_IMAGE_1_ORIGIN=ubuntu:24.04 and parameterize Dockerfile FROM with ARG RBF_IMAGE_1

### 2026-03-24 08:47 - ₢AvAAR - W

Added label parameter to zrbf_wait_build_completion so poll status lines identify the build type (Enshrine/Conjure/Mirror/About/Vouch). Label appears in step header, poll status, timeout, failure, and success messages. All 5 call sites updated. Regime-validation passes (21 cases).

### 2026-03-24 08:47 - ₢AvAAR - n

Add label parameter to zrbf_wait_build_completion for pipeline-specific log messages (Enshrine/Conjure/Mirror/About/Vouch)

### 2026-03-24 08:40 - ₢AvAAP - W

Replaced all 6 sed usages in Foundry with bash-native alternatives: 4 dollar-escaping sites use parameter expansion, about post-processing uses parameter expansion on file content, recipe display uses read loop. Zero sed calls remain in rbf_Foundry.sh. Also fixed rbrg_cli.sh dead slsa-verifier binary pin pass-through (unbound variable under set -u) and refreshed all 7 GCB image pins. Live bind build verified: mirror + vouch both SUCCESS on GCB.

### 2026-03-24 08:39 - Heat - S

build-poll-label-parameter

### 2026-03-24 08:21 - ₢AvAAP - n

Fix rbrg_refresh_gcb_pins: remove dead slsa-verifier binary pin pass-through (unbound variable under set -u), guard empty array in writer. Refresh all 7 GCB image pins (4 updated: gcloud, docker, syft, skopeo).

### 2026-03-24 08:21 - Heat - S

rbrg-remove-slsa-verifier-dead-code

### 2026-03-24 08:02 - ₢AvAAP - n

Replace all 6 sed usages in Foundry with bash-native alternatives: 4 dollar-escaping sites (stitch/mirror/about/vouch) use parameter expansion, about post-processing uses parameter expansion on file content, recipe display uses read loop. Zero sed calls remain.

### 2026-03-24 07:59 - ₢AvAAO - W

Rewrote enshrine from Director-local skopeo to Cloud Build job submission. New GCB step script rbgje01-enshrine-copy.sh does inspect+copy per slot, returns anchor JSON via buildStepOutputs. Foundry extracts anchors and writes regime. BCG-compliant bash-native alternatives: parameter expansion for dollar-escaping, read loop for regime writeback. No local skopeo dependency. RBSAE spec updated. All test fixtures pass.

### 2026-03-24 07:55 - ₢AvAAO - n

Rewrite enshrine to submit Cloud Build job instead of running skopeo locally. GCB step does inspect+copy per slot, returns anchors via buildStepOutputs. Foundry extracts anchors and writes regime. BCG-compliant: bash parameter expansion for dollar-escaping, read loop for regime writeback. No local skopeo dependency.

### 2026-03-24 07:55 - Heat - S

bcg-sed-to-parameter-expansion

### 2026-03-24 07:43 - Heat - n

Fix paddock: update Dockerfile usage examples from RBRV_IMAGE_1 to RBF_IMAGE_1 to match the rename.

### 2026-03-24 07:41 - Heat - S

enshrine-gcb-correction

### 2026-03-24 07:31 - ₢AvAAN - n

Adapt rbev-busybox to ARG/FROM pattern: RBRV_IMAGE_1_ORIGIN=busybox:latest with empty ANCHOR, Dockerfile uses ARG RBF_IMAGE_1 / FROM ${RBF_IMAGE_1}. Pass-through preserves current behavior; enshrine will fill ANCHOR for GAR resolution.

### 2026-03-24 07:29 - ₢AvAAN - n

Rename Dockerfile build-arg from RBRV_IMAGE_n to RBF_IMAGE_n — the Foundry resolves the reference, so the name should reflect Foundry provenance, not regime provenance.

### 2026-03-23 19:00 - ₢AvAAM - W

Wired ANCHOR into conjure and vouch pipelines. Stitch resolves RBRV_IMAGE_n_ANCHOR to full GAR ref (or passes ORIGIN through) via _RBGY_IMAGE_n substitutions. rbgjb03 passes --build-arg RBRV_IMAGE_n to docker buildx build for Dockerfile ARG/FROM pattern. Vouch records base_images array in vouch_summary.json with per-slot ref and provenance type (anchored/pass-through). Six new _RBGV_IMAGE_n/_RBGV_IMAGE_n_PROVENANCE substitution variables. RBSAV spec updated.

### 2026-03-23 18:58 - ₢AvAAM - n

Wire ANCHOR into conjure and vouch pipelines. Stitch resolves RBRV_IMAGE_n_ANCHOR to full GAR reference (or passes ORIGIN through) as _RBGY_IMAGE_n substitutions. rbgjb03 passes --build-arg RBRV_IMAGE_n for Dockerfile ARG/FROM pattern. Vouch records base_images array in summary with per-slot provenance (anchored vs pass-through). RBSAV spec updated with 6 new substitution variables.

### 2026-03-23 18:45 - ₢AvAAJ - W

Enshrine core implemented end-to-end. Spec corrected from docker pull/tag/push to skopeo copy --all with manifest list preservation. Three settled design decisions: skopeo over crane/oras, anchors in vessel GAR repo, Director-local execution. Regime: 6 RBRV_IMAGE_[123]_{ORIGIN,ANCHOR} variables enrolled in Image Group (conjure-gated). Foundry: rbf_enshrine loads vessel, authenticates Director, iterates slots 1-3 calling zrbf_enshrine_slot (inspect upstream, compute manifest digest, construct anchor, skopeo copy --all to GAR, write ANCHOR back to rbrv.env). Wired as rbw-DE with tabtarget. RBSRG skopeo description updated from retained-for-future-use to active enshrine role.

### 2026-03-23 18:39 - ₢AvAAJ - n

Implement enshrine core: 6 RBRV_IMAGE_[123]_{ORIGIN,ANCHOR} variables enrolled in vessel regime (Image Group, conjure-gated). rbf_enshrine + zrbf_enshrine_slot in Foundry — Director-local skopeo inspect/copy with OAuth token, anchor construction via BCG parameter expansion, regime writeback. Zipper enrollment rbw-DE and tabtarget.

### 2026-03-23 18:28 - Heat - S

enshrine-end-to-end-verification

### 2026-03-23 18:28 - Heat - S

wire-anchor-into-conjure-vouch

### 2026-03-23 18:27 - Heat - T

implement-enshrine-core

### 2026-03-23 18:25 - ₢AvAAJ - n

Enshrine spec updated for skopeo copy --all multi-platform manifest list preservation. RBSAE: replaced docker pull/tag/push with skopeo copy --all, digest source is manifest list digest, single-platform handled identically. RBS0+RBSRG: skopeo variable description updated from retained-for-future-use to active enshrine role. Paddock: two new settled decisions — anchors live in vessel GAR repo (same namespace, pattern-distinguishable), skopeo chosen over crane/oras (already pinned, purpose-built for image mirroring).

### 2026-03-23 18:07 - ₢AvAAI - W

Defined ark_enshrine operation and RBRV_IMAGE_[n]_{ORIGIN,ANCHOR} regime variables across four spec files. RBS0: operation mapping, variable mappings, operation section with include, variable definitions. RBSRV: image group with ORIGIN/ANCHOR in conjuring section. RBSAC: base image resolution step added to conjure flow. RBSAE: new enshrine operation subdocument (validate, enshrine-each-slot loop). Paddock updated: reliquary vocabulary corrected to tool-images-only, inscribe/enshrine separation documented as load-bearing design decision with comparison table, all stale BASE_IMAGE/TAG/SHA references eliminated.

### 2026-03-23 18:06 - Heat - d

paddock curried

### 2026-03-23 18:02 - ₢AvAAI - n

Define ark_enshrine operation and RBRV_IMAGE_[n]_{ORIGIN,ANCHOR} regime variables in spec. RBS0: operation mapping, variable mappings, operation section, variable definitions. RBSRV: image group with ORIGIN/ANCHOR in conjuring section. RBSAC: base image resolution step. RBSAE: new enshrine operation subdocument.

### 2026-03-23 17:54 - Heat - d

paddock curried

### 2026-03-23 10:35 - Heat - r

moved AvAAH to last

### 2026-03-23 10:34 - Heat - S

egress-lockdown-flip-and-verify

### 2026-03-23 10:34 - Heat - S

implement-reliquary-inscribe-tool-images

### 2026-03-23 10:32 - Heat - S

implement-ark-enshrine-operation

### 2026-03-23 10:32 - Heat - S

spec-base-image-enshrine-operation

### 2026-03-23 10:00 - ₢AvAAG - W

Swept eliminated terms (GitLab, triggers, slsa-verifier, CB v2, rubric repo) from 14 spec sub-documents plus RBS0 active content. RBSAV rewritten for DSSE envelope verification (3→2 build steps, new substitution vars). RBSAC rewritten for builds.create + pouch + local consecration minting. RBSRR CBv2 group removed. RBSDC heavy rewrite removing 7 GitLab/CBv2 steps. RBSCB Current Posture updated for pouch architecture. RBSDI/RBSRC preconditions removed. RBSGR connectionViewer eliminated. RBSRG slsa-verifier vars eliminated. RBSAG/RBSGS/RBSQB trigger_build refs updated. RBSCTD marked superseded. Verification grep confirms zero eliminated terms in active content.

### 2026-03-23 09:58 - ₢AvAAG - n

Sweep eliminated terms (GitLab, triggers, slsa-verifier, CB v2, rubric repo) from 14 spec sub-documents. RBSDC heavy rewrite (removed 7 GitLab/CBv2 steps). RBSCB Current Posture updated for builds.create + pouch + DSSE. RBSDI/RBSRC preconditions removed. RBSGR connectionViewer eliminated. RBSRG slsa-verifier vars eliminated. RBSAG/RBSGS/RBSQB trigger_build references updated. RBSCTD marked superseded. RBS0 active-content fixes (vouch verdict, concurrency section, step architecture).

### 2026-03-23 09:54 - ₢AvAAH - n

Remove GitLab dead code from Governor create_director (Secret Manager grant, bucket grants, project number fetch). Update rbrr.env for new depot depot10040. End-to-end verified: depot destroy, create (no GitLab/SM/CBv2), governor reset (no CBv2 verification), director create (no SM/bucket grants), conjure build + DSSE vouch on rbev-busybox — all successful.

### 2026-03-23 09:35 - Heat - S

end-to-end-depot-lifecycle-verification

### 2026-03-23 09:35 - ₢AvAAF - W

Removed GitLab dead code from rbgp_Payor.sh (depot_create, depot_destroy, governor_reset), rbgu_Utility.sh (two functions), and rbgm_ManualProcedures.sh (unreachable guide text). ~350 lines eliminated: CB v2 connection/repository lifecycle, Secret Manager PAT storage, GitLab URL validation, connection viewer grant/verification. Remaining references in rbf_Foundry.sh are trigger-path code (separate elimination scope).

### 2026-03-23 09:31 - ₢AvAAF - n

Remove GitLab dead code from Payor: CB v2 connection/repository creation, Secret Manager PAT storage (3 secrets), GitLab URL validation, connection viewer grant/verification. Remove rbgu_check_rubric_repo_url and zrbgu_gitlab_tokens_url_capture from Utility. Remove unreachable dead code from gitlab_setup manual procedure.

### 2026-03-21 15:42 - ₢AvAAE - W

Spec and depot lifecycle GitLab removal complete. Code: regime enrollments and env vars removed (RBRR_CBV2, RBRR_RUBRIC, RBRG_SLSA_VERIFIER, RBRG_BINARY_PINS), Governor rubric preflight gutted to credential-only, Marshal reset strips eliminated vars, Manual Procedures gitlab_setup returns immediately, onboarding level 3 auto-passes. Spec: RBS0 comprehensively updated — all eliminated terms (vessel trigger, rubric repo, vessel dir, CB v2, slsa-verifier) removed from body references, definitions rewritten or removed, operations updated (inscribe deferred, trigger_build eliminated, conjure/vouch updated to builds.create + DSSE). End-to-end verified after all changes.

### 2026-03-21 15:42 - Heat - S

spec-subdocument-gitlab-sweep

### 2026-03-21 15:42 - Heat - S

payor-gitlab-dead-code-removal

### 2026-03-21 15:34 - ₢AvAAE - n

Complete RBS0 spec cleanup: rewrite all body references to eliminated terms (vessel trigger, rubric repo, vessel dir, slsa-verifier, CB v2). Remove check_rubric_repo_url and check_cbv2_connection procedure definitions. Update vouch architecture description to DSSE. Update rubric definition to reflect builds.create + pouch. Zero remaining references to eliminated terms.

### 2026-03-21 15:27 - ₢AvAAE - n

RBS0 spec definition sections: eliminate rubric repo, vessel trigger, vessel directory, CB v2 connection/repository definitions. Eliminate slsa-verifier and binary pin variable definitions. Update provenance, build_json, RBRG regime descriptions. Comment out eliminated sub-document includes (RBSRI, RBSTB). Mark arrival paths for builds.create + DSSE.

### 2026-03-21 15:25 - ₢AvAAE - n

RBS0 spec updates: mark eliminated attributes (CB v2, rubric repo, vessel trigger, slsa-verifier, binary pins) as '(eliminated)' in linked term mappings. Update operation descriptions: rubric_inscribe deferred, trigger_build eliminated, ark_conjure updated to builds.create + pouch, ark_vouch updated to DSSE. Comment out eliminated sub-document includes.

### 2026-03-21 15:20 - ₢AvAAE - n

Remove GitLab/slsa-verifier from depot lifecycle code: regime enrollments removed (RBRR_CBV2, RBRR_RUBRIC, RBRG_SLSA_VERIFIER, RBRG_BINARY_PINS), variables removed from env files, Governor rubric preflight gutted to credential-only check, Marshal reset strips eliminated variables, Manual Procedures GitLab setup returns immediately, onboarding level 3 auto-passes.

### 2026-03-21 15:12 - ₢AvAAD - n

Fix bind/graft vouch bug (jq installed for all modes, not just conjure). Fix stitch refactor bug (restore _RBGA_GIT_REPO substitution for about steps). Both fixes verified end-to-end: refactored stitch + DSSE vouch on 3 platforms.

### 2026-03-21 15:01 - ₢AvAAD - W

GitLab elimination baby step complete. Vouch pipeline rewritten: slsa-verifier and Python eliminated, replaced with DSSE envelope signature verification (jq + base64 + openssl). Three GCB issues resolved during testing: static jq binary (musl/glibc compat), jq -j for clean base64 pipe, base64url-to-standard conversion for signatures. Stitch function cleaned up: placeholders eliminated, generates complete builds.create JSON directly (context extraction step, mason SA, all substitutions resolved). rbf_rubric_inscribe gutted with die explaining deferral. Regime GitLab variables emptied. End-to-end verified: builds.create conjure + DSSE vouch on rbev-busybox, 3 platforms (amd64, arm64, armv7) all Verified OK.

### 2026-03-21 15:01 - Heat - S

spec-depot-lifecycle-gitlab-removal

### 2026-03-21 14:56 - ₢AvAAD - n

Stitch cleanup: eliminate placeholder/jq-surgery pattern. Stitch now accepts inscribe_ts and context_tag directly, generates complete builds.create JSON (context extraction step, mason SA, all substitutions resolved). rbf_build simplified to single stitch call with no post-processing. rbf_rubric_inscribe gutted with die explaining deferral to reliquary.

### 2026-03-21 14:33 - ₢AvAAD - n

Fix vouch GCB step: static jq binary from GitHub (musl/glibc compat), jq -j for clean base64 pipe, base64url-to-standard conversion for signature (tr), step arg under 10K limit. End-to-end verified: builds.create conjure + DSSE vouch on 3 platforms (amd64, arm64, armv7).

### 2026-03-21 13:35 - ₢AvAAD - n

Replace slsa-verifier and direct_verify.py with DSSE envelope signature verification (jq + base64 + openssl). Eliminate all Python from vouch pipeline. Remove slsa-verifier substitutions from vouch build JSON. Clear GitLab regime variables (CBV2 connection, rubric repo URL). Bind and graft vouch modes rewritten from Python to jq.

### 2026-03-21 13:15 - Heat - S

gitlab-elimination-pouch-builds-create

### 2026-03-21 13:15 - Heat - T

revert-experiment-restore-trigger-path

### 2026-03-21 13:04 - ₢AvAAB - W

SLSA L3 question definitively answered: builds.create achieves Build L3 by v1.0 spec (source verification is separate track) and by GCB's own assessment (slsa_build_level: 3 confirmed on depot10030). slsa-verifier dropped — conflates Build and Source tracks, requires buildConfigSource that doesn't exist for builds.create. Google docs wrong about v0.1-only for non-trigger builds (empirically disproven). Replacement: DSSE envelope signature verification using jq + openssl, empirically proven against rbev-busybox arm64 image — all 3 signatures (v1.0 google-hosted-worker, v0.1 provenanceSigner PAE, v0.1 builtByGCB legacy) verified OK. Public keys accessible from verified-builder KMS; embedded in reliquary at inscribe time for air-gap compatibility.

### 2026-03-21 13:04 - ₢AvAAB - n

Settle SLSA L3 question: builds.create achieves Build L3 by spec, drop slsa-verifier for DSSE envelope signature verification via jq+openssl, document vouch verification architecture in paddock

### 2026-03-21 11:46 - ₢AvAAA - W

Experiment proved builds.create viability (3-platform conjure end-to-end). Design conversation produced comprehensive architecture: reliquary (required co-versioned GAR image sets), pouch (build context as ark artifact), inscribe reclaimed for reliquary generation, triggers fully eliminated, consecration minted locally, RBRG replaced by reliquary. All findings and decisions captured in heat paddock. Experiment code intentionally NOT kept as implementation foundation — revert slated as ₢AvAAC.

### 2026-03-21 11:46 - ₢AvAAA - n

jjk: gazette spec restructure — lift invariants and wire format to top-level sections, add slug linked term, deduplicate method descriptions

### 2026-03-21 11:43 - Heat - d

paddock curried

### 2026-03-21 11:39 - Heat - d

paddock curried

### 2026-03-21 11:34 - Heat - d

paddock curried

### 2026-03-21 11:33 - Heat - S

revert-experiment-restore-trigger-path

### 2026-03-21 11:26 - Heat - d

paddock curried

### 2026-03-21 11:24 - Heat - d

paddock curried

### 2026-03-21 11:23 - Heat - S

slsa-level3-builds-create-verification

### 2026-03-21 10:09 - Heat - d

paddock curried

### 2026-03-17 21:47 - Heat - T

slsa-provenance-builds-create-experiment

### 2026-03-17 21:45 - Heat - T

slsa-provenance-builds-create-experiment

### 2026-03-17 21:20 - ₢AvAAA - n

Fix builds.create submission: strict substitution matching, mason SA, context image platform/cmd, direct provenance verification for vouch without git source

### 2026-03-17 20:25 - ₢AvAAA - n

Replace trigger-based conjure with builds.create: added zrbf_push_build_context for GAR context delivery, rewrote rbf_build to stitch+submit directly, fixed CB dollar escaping in extraction step

### 2026-03-17 20:03 - Heat - T

slsa-provenance-builds-create-experiment

### 2026-03-17 19:37 - Heat - f

racing

### 2026-03-15 11:46 - Heat - f

stabled

### 2026-03-15 11:46 - Heat - S

slsa-provenance-builds-create-experiment

### 2026-03-15 11:46 - Heat - d

paddock curried

### 2026-03-15 11:46 - Heat - d

paddock curried

### 2026-03-15 11:44 - Heat - N

rbk-egress-lockdown-research

