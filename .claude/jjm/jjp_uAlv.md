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