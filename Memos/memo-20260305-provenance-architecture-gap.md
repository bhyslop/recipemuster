# Cloud Build Provenance Architecture Gap

**Date:** 2026-03-05
**Heat:** rbw-e2e-cbv2-provenance (Al)
**Pace:** research-cloudbuild-provenance-mechanics (AlAAI), test-buildx-push-gar (AlAAK), test-pullback-images-verified (AlAAL)
**Status:** Hypotheses A and B experimentally validated on 2026-03-05

## Summary

Cloud Build native SLSA v1.0 provenance (Level 3) is achievable by replacing the
OCI Layout Bridge with `docker buildx build --push` and pulling the image back
into Docker's local daemon for the `images:` field. Three experiments on
2026-03-05 (builds `a3e5c2d7`, `4de1467a`, `48b818ed`) validated the full chain:
buildx pushes multi-platform images to GAR using pre-populated ADC credentials
(no docker login required), and the pullback + `images:` + `requestedVerifyOption:
VERIFIED` pattern produces dual-signed SLSA Level 3 provenance.

The OCI Layout Bridge (crane push) was structurally incompatible with CB-native
provenance because `images:` requires images in Docker's local daemon, and crane
pushed directly to the registry. Adding `VERIFIED` or `images:` to the crane-based
pipeline would have broken builds, not just skipped provenance. The buildx --push +
pullback pattern resolves this. Cosign is not needed.

## Why the OCI Layout Bridge Exists

The build pipeline uses crane to push images because Docker BuildKit's
`docker-container` driver (required for multi-platform builds) runs in an
isolated container that cannot access host Docker credentials. This was
documented as an architectural limitation in the RBSOB trade study — credentials
configured via `docker login` in one Cloud Build step are not available inside
the BuildKit container in subsequent steps.

**However:** Research found multiple documented examples of `docker buildx build
--push` working in Cloud Build without explicit `docker login`, using Application
Default Credentials (ADC) from the `gcr.io/cloud-builders/docker` image.
BuildKit's `authprovider` session mechanism forwards credentials from the buildx
client to the daemon via gRPC. This challenges the RBSOB assumption and is the
basis for Hypothesis A below.

See `lenses/RBSOB-oci_layout_bridge.adoc` for the original trade study.

## Root Cause

Cloud Build generates provenance only when it performs the image push itself, via
the top-level `images:` field in `cloudbuild.json`. The `images:` field triggers
Cloud Build to push from Docker's local image store after all steps complete.

The OCI Layout Bridge pushes via crane in build step 07, bypassing Cloud Build's
native push entirely. Three structural blockers:

1. **`images:` requires local daemon image** — Cloud Build checks Docker's local
   store, not the registry. If the image was pushed by crane but doesn't exist
   locally, the build fails with "failed to find one or more images after
   execution of build steps" (documented in
   [cloud-builders-community#212](https://github.com/GoogleCloudPlatform/cloud-builders-community/issues/212)).

2. **Multi-platform manifests can't exist in Docker's local store** — Docker's
   image store is single-platform only. `docker buildx build` with the
   `docker-container` driver exports to OCI archive or pushes directly to
   registry; it cannot `--load` multi-platform images locally.

3. **`requestedVerifyOption: VERIFIED` fails the build** — When set, "Builds
   will only be marked successful if provenance is generated." Since provenance
   can't be generated for step-pushed images, VERIFIED + crane push = build
   failure. This is not a silent skip.

## Two Separate Provenance Worlds

| | CB-native provenance | Cosign attestations |
|---|---|---|
| Storage | Container Analysis (Grafeas) | OCI registry (`.sig`/`.att` tags) |
| Visible in `--show-provenance`? | Yes | No |
| BinAuth SLSA check? | Yes (only trusted builder) | No |
| BinAuth Sigstore CV check? | No | Yes (GAR only) |
| Enforcement | Deploy-time via BinAuth SLSA | Deploy-time via BinAuth Sigstore |

These systems do not cross-populate. Cosign will never make `--show-provenance`
work. Binary Authorization's SLSA check is locked to Cloud Build: "The only
trusted builder that the SLSA check supports is Cloud Build."

## Hypotheses

### Hypothesis A: buildx --push eliminates OCI Layout Bridge (prerequisite) — CONFIRMED

Multiple documented examples show `docker buildx build --push` working in Cloud
Build without explicit `docker login`, using ADC (Application Default
Credentials) from the `gcr.io/cloud-builders/docker` image. The BuildKit
`authprovider` session mechanism forwards credentials from the buildx client to
the BuildKit daemon container via gRPC. Google's own
[Dataflow multi-arch container guide](https://docs.cloud.google.com/dataflow/docs/guides/multi-architecture-container)
uses this pattern. The docker/buildx#3050 maintainer confirmed (March 2025):
"You don't need to do anything extra apart from `docker login`" for standard
(non-DIND) usage.

If this works, crane (step 07) and the OCI Layout Bridge become unnecessary.

**Known risks:**
- 1-hour token expiry for long builds ([docker/buildx#1205](https://github.com/docker/buildx/issues/1205))
- DIND credential isolation edge cases ([docker/buildx#3050](https://github.com/docker/buildx/issues/3050))
- Docker 28.3.0+ `DOCKER_AUTH_CONFIG` regression ([docker/cli#6156](https://github.com/docker/cli/issues/6156))
- `DOCKER_CONFIG` breaks buildx plugin discovery ([docker/cli#5477](https://github.com/docker/cli/issues/5477) — still open)

**Does not solve provenance alone** — even with `--push`, the image is not in
Docker's local store, so `images:` still can't push it. But this is the
prerequisite for Hypothesis B.

**Validated:** See Experiment 1 (Variant A) and Experiment 2 (Variant B) below.

### Hypothesis B: pull-back + images: + VERIFIED = CB-native SLSA (primary goal) — CONFIRMED

If Hypothesis A succeeds (buildx pushes directly to GAR), add a follow-up step
that `docker pull`s the image back into Docker's local store, then declare it in
`images:` with `requestedVerifyOption: VERIFIED`.

**Key question:** `docker pull` of a multi-platform manifest list pulls only the
host platform's image (linux/amd64 on Cloud Build workers). Cloud Build would
then push this single-platform image via `images:`. Would this:
- Overwrite the multi-platform manifest with a single-platform one? (destructive)
- Push a new tag alongside it? (safe if we use a separate provenance tag)
- Generate provenance on the pulled-back digest?

If provenance attaches to the single-platform digest, and the multi-platform
manifest pushed by buildx remains intact under its own tag, this could work with
a dual-tag scheme: `TAG-image` (multi-platform via --push) and `TAG-attested`
(single-platform via images: for provenance).

**This is the primary experiment path.** CB-native SLSA is strongly preferred
over cosign because it preserves the existing trust model (builder-identity-based,
Google-signed, integrated with `--show-provenance` and BinAuth SLSA check).

**Validated:** See Experiment 3 below. The pullback + `images:` + `VERIFIED`
pattern produces **SLSA Build Level 3** provenance with dual signatures.

**Answers to key questions:**
- The `images:` push **does overwrite** the multi-platform manifest with single-platform.
  A production pipeline must use a dual-tag scheme to preserve both.
- Provenance attaches to the **pulled-back single-platform digest** (`sha256:ae13bcc...`),
  distinct from the buildx multi-platform digest (`sha256:45b0dad...`).
- `VERIFIED` works correctly — the build would have failed if provenance could not be generated.

### Hypothesis C: cosign keyless signing (fallback only) — NOT NEEDED

**Only pursue if Hypotheses A+B fail.** Cosign is a fundamentally different trust
model (key-based attestor vs builder-identity-based SLSA). Adopting it means
abandoning CB-native SLSA and `--show-provenance` integration.

Cosign can sign images from Cloud Build using OIDC identity tokens via service
account impersonation. Required setup:

1. Create signing service account (its email becomes the certificate SAN)
2. Grant CB service account `roles/iam.serviceAccountTokenCreator` on signing SA
3. Build step: `cosign sign --identity-token=$(gcloud auth print-identity-token --audiences=sigstore --impersonate-service-account=SIGNING_SA) IMAGE@DIGEST`

Working reference: [salrashid123/cosign_bazel_cloud_build](https://github.com/salrashid123/cosign_bazel_cloud_build)

Binary Authorization's Sigstore CV check can then enforce that only signed images
deploy. This provides cryptographic deploy-time enforcement but in a different
trust model from CB-native SLSA.

**Key constraint:** The Sigstore CV check uses **ECDSA key-based verification**,
not OIDC identity patterns. The policy references the cosign public key
directly, not the service account email. This means provisioning and managing a
signing keypair (or KMS key), not just configuring identity matching.

## Dead Ends (Confirmed)

| Approach | Why it fails |
|---|---|
| Add `images:` with crane-pushed URI | Build fails — image not in local Docker daemon |
| Add `VERIFIED` to current pipeline | Build fails — provenance can't be generated for step-pushed images |
| Artifact Analysis API manual provenance | API works, but BinAuth SLSA check rejects non-CB provenance |
| aactl import tool | Archived January 2026 |
| `artifacts:` field for containers | Only handles non-container artifacts (Maven, npm, etc.) |
| Tekton Chains | Kubernetes-resident; can't observe Cloud Build |

## Key Documentation

- [Generate and validate build provenance](https://docs.cloud.google.com/build/docs/securing-builds/generate-validate-build-provenance) — canonical provenance docs
- [Securing image deployments (private pool attestations)](https://docs.cloud.google.com/build/docs/securing-builds/secure-deployments-to-run-gke) — VERIFIED option
- [Binary Authorization Sigstore check](https://docs.cloud.google.com/binary-authorization/docs/cv-sigstore-check) — cosign enforcement
- [Binary Authorization SLSA check](https://docs.cloud.google.com/binary-authorization/docs/cv-slsa-check) — CB-only provenance enforcement
- [Build config schema](https://docs.cloud.google.com/build/docs/build-config-file-schema) — `images:` field semantics
- [cloud-builders-community#212](https://github.com/GoogleCloudPlatform/cloud-builders-community/issues/212) — "image not found locally" failure
- [Google Issue Tracker #264950908](https://issuetracker.google.com/issues/264950908) — request for step-pushed image recording
- [docker/buildx#3050](https://github.com/docker/buildx/issues/3050) — DIND credential propagation
- [docker/buildx#1205](https://github.com/docker/buildx/issues/1205) — 1-hour token expiry
- [docker/cli#5477](https://github.com/docker/cli/issues/5477) — DOCKER_CONFIG breaks buildx (open)
- [salrashid123/cosign_bazel_cloud_build](https://github.com/salrashid123/cosign_bazel_cloud_build) — working cosign+CB example

## Experiment Results (2026-03-05)

All experiments run on depot `demo1025` using `gcloud builds submit --no-source`
(inline cloudbuild.json, no source upload). Private worker pool. Busybox test
image (multi-platform: linux/amd64, linux/arm64, linux/arm/v7).

### Common Infrastructure

| Component | Value |
|---|---|
| GCP project | `rbwg-d-demo1025-260304183118` |
| Region | `us-central1` |
| GAR repository | `us-central1-docker.pkg.dev/rbwg-d-demo1025-260304183118/rbw-demo1025-repository` |
| Worker pool | `projects/rbwg-d-demo1025-260304183118/locations/us-central1/workerPools/rbw-demo1025-pool` |
| Pool machine type | `e2-standard-2` |
| Service account | `134185315774-compute@developer.gserviceaccount.com` |

### Toolchain Versions (observed in build logs)

| Tool | Version | Image / Source |
|---|---|---|
| Docker Engine | 20.10.24 (API 1.41) | `gcr.io/cloud-builders/docker@sha256:efdbd755476e7e5eb1077ed6e4bf691f87b38fbded575e9b825f9480374f8f4b` |
| docker buildx | v0.23.0 (28c90ea) | bundled in docker builder image |
| BuildKit | moby/buildkit:buildx-stable-1 | pulled by docker-container driver at build time |
| containerd | v2.2.1 | in docker builder image |
| runc | 1.3.4 | in docker builder image |
| gcloud CLI | (digest-pinned) | `gcr.io/cloud-builders/gcloud@sha256:4a58b9883e286608d1084cf21e5aed1cbb0817ac6adf41b1293cf2ef46c4942e` |
| binfmt (QEMU) | (digest-pinned) | `docker.io/tonistiigi/binfmt@sha256:8db0f28060565399642110b798c6c35efcac7c5b3b48c56d36503d3b4d8f93c8` |

### Experiment 1: Variant A — buildx --push with docker login (Hypothesis A)

**Purpose:** Test whether `docker buildx build --push` can push multi-platform
images directly to GAR from Cloud Build, with explicit `docker login` using an
oauth2 access token.

**Build ID:** `a3e5c2d7-68a0-4c02-b336-6315d6f4c871`
**Submitted:** 2026-03-05T19:46:32Z
**Duration:** 28 seconds
**Status:** SUCCESS

**Steps:**
1. `get-docker-token` — `gcloud auth print-access-token > /workspace/.docker-token`
2. `docker-login-gar` — `docker login -u oauth2accesstoken` to `us-central1-docker.pkg.dev`
3. `qemu-binfmt` — register arm64, arm via `tonistiigi/binfmt`
4. `buildx-push` — `docker buildx create --driver docker-container` then
   `docker buildx build --push --platform=linux/amd64,linux/arm64,linux/arm/v7`

**Image tag:** `rbev-busybox:buildx-push-test-a-image`
**Manifest digest:** `sha256:026ba6c23d493a1ff0de40642a3fb7bd58a3ed1aab0ab6a5ee8c620384b41a7c`

**Key log evidence:**
```
#17 [auth] rbwg-d-demo1025-260304183118/rbw-demo1025-repository/rbev-busybox:pull,push token for us-central1-docker.pkg.dev
#16 pushing manifest for .../rbev-busybox:buildx-push-test-a-image@sha256:026ba6c23d493...
```

BuildKit obtained `pull,push` credentials via the authprovider session mechanism,
forwarding the Docker config credentials from the buildx client (Cloud Build
step container) to the BuildKit daemon container via gRPC.

**Conclusion:** `docker buildx build --push` works in Cloud Build with explicit
`docker login`. The OCI Layout Bridge is unnecessary.

### Experiment 2: Variant B — buildx --push with ADC only (Hypothesis A)

**Purpose:** Test whether the explicit `docker login` step can be eliminated
entirely, relying only on Application Default Credentials.

**Build ID:** `4de1467a-35a7-4e0d-8a98-03de6f16e4b2`
**Submitted:** 2026-03-05T19:48:53Z
**Duration:** 20 seconds
**Status:** SUCCESS

**Steps:**
1. `create-dockerfile` — write Dockerfile inline (--no-source build)
2. `qemu-binfmt` — register arm64, arm
3. `buildx-push-adc` — buildx create + build --push (NO docker login step)

**Image tag:** `rbev-busybox:buildx-push-test-b-image`

**Critical finding: Cloud Build pre-populates Docker credentials.**

The build logged the contents of `/builder/home/.docker/config.json` BEFORE any
`docker login` was executed. It contained `oauth2accesstoken` credentials for
**every GAR regional endpoint** — `africa-south1-docker.pkg.dev`,
`asia-docker.pkg.dev`, `asia-east1-docker.pkg.dev`, `us-central1-docker.pkg.dev`,
etc. (dozens of entries).

This means Cloud Build automatically provisions Docker registry credentials
for the build service account across all GAR regions at build start. The
authprovider session mechanism in buildx then forwards these pre-provisioned
credentials to the BuildKit daemon.

**Implication for pipeline:** Steps 02 (`gcloud auth print-access-token`) and
03 (`docker login`) can be **completely eliminated** from the production build
pipeline. This removes two steps and eliminates the `/workspace/.docker-token`
intermediate file.

**Conclusion:** ADC is sufficient. No explicit `docker login` is needed.

### Experiment 3: Provenance — buildx --push + pullback + images: + VERIFIED (Hypothesis B)

**Purpose:** Test whether pulling the buildx-pushed image back to the local
Docker daemon, then declaring it in `images:` with `requestedVerifyOption:
VERIFIED`, causes Cloud Build to generate native SLSA provenance.

**Build ID:** `48b818ed-6642-4ce9-94f5-5b67d1264dca`
**Submitted:** 2026-03-05T19:52:14Z
**Duration:** 24 seconds
**Status:** SUCCESS

**Steps:**
1. `create-dockerfile` — write Dockerfile inline
2. `qemu-binfmt` — register arm64, arm
3. `buildx-push` — `docker buildx build --push` multi-platform to GAR
4. `pullback` — `docker pull` the image back to local daemon (pulls linux/amd64 only)

**cloudbuild.json fields (beyond steps):**
```json
"images": ["us-central1-docker.pkg.dev/.../rbev-busybox:buildx-prov-test-image"],
"options": { "requestedVerifyOption": "VERIFIED" }
```

**Digest chain:**
- buildx multi-platform manifest: `sha256:45b0dad731bbc7358390535940c245146cac65a281168f01347dc4f9508467c2`
- `docker pull` pulled: `sha256:45b0dad...` (resolved to linux/amd64 platform)
- `images:` pushed (single-platform): `sha256:ae13bcc0735409f45217e94fffce9011c703f334492a228fc6efa1ba55f65f67`

**SLSA Provenance — `gcloud artifacts docker images describe --show-provenance`:**

```
slsa_build_level: 3
```

Two provenance occurrences were generated:

**Occurrence 1 — in-toto Statement v0.1 / SLSA Provenance v0.1:**
```yaml
_type: https://in-toto.io/Statement/v0.1
predicateType: https://slsa.dev/provenance/v0.1
builder.id: https://cloudbuild.googleapis.com/GoogleHostedWorker@v0.3
metadata.buildInvocationId: 48b818ed-6642-4ce9-94f5-5b67d1264dca
subject.digest.sha256: ae13bcc0735409f45217e94fffce9011c703f334492a228fc6efa1ba55f65f67
```

Signatures:
- `projects/verified-builder/locations/global/keyRings/attestor/cryptoKeys/provenanceSigner/cryptoKeyVersions/1`
  sig: `MEQCIHDFHS4nWneL22RhTp68xJ9JuqYB-ET0Y0x-r-5AbqanAiB0_pdy3-L7_0PC1ZcDAhIUg-a_sKBWMpQGxFirC_sGYw==`
- `projects/verified-builder/locations/us-central1/keyRings/attestor/cryptoKeys/builtByGCB/cryptoKeyVersions/1`
  sig: `MEYCIQD-Yaphr2CDft6-wFkIJjot6luB5Zns5Ly6HsAbAiG_cAIhAOZOGCV1mEKUHZcIxbPwaaspM7pTeGN8la8SrOhTPc94`

**Occurrence 2 — in-toto Statement v1 / SLSA Provenance v1:**
```yaml
_type: https://in-toto.io/Statement/v1
predicateType: https://slsa.dev/provenance/v1
buildType: https://cloud.google.com/build/gcb-buildtypes/google-worker/v1
builder.id: https://cloudbuild.googleapis.com/GoogleHostedWorker
subject.digest.sha256: ae13bcc0735409f45217e94fffce9011c703f334492a228fc6efa1ba55f65f67
```

Signature:
- `projects/verified-builder/locations/global/keyRings/attestor/cryptoKeys/google-hosted-worker/cryptoKeyVersions/1`
  sig: `MEYCIQC63KXLupq9APIUPHDhxygZtrszH9tfwj1PF_-uuOTQcQIhAM9Wr3Uj7XhTUPS-SUvwrvxTHJrXVnuJ4wSpSHpJ0y1I`

Internal parameters recorded in v1 provenance:
```yaml
SERVICE_ACCOUNT: projects/rbwg-d-demo1025-260304183118/serviceAccounts/134185315774-compute@developer.gserviceaccount.com
BUILD_ID: 48b818ed-6642-4ce9-94f5-5b67d1264dca
LOCATION: us-central1
PROJECT_NUMBER: '134185315774'
```

**Build log evidence (PUSH phase):**
```
PUSH
Pushing us-central1-docker.pkg.dev/.../rbev-busybox:buildx-prov-test-image
The push refers to repository [us-central1-docker.pkg.dev/.../rbev-busybox]
23f74b8f7b68: Layer already exists
7e9dfc5b4c68: Layer already exists
495ba00f2547: Layer already exists
buildx-prov-test-image: digest: sha256:ae13bcc0735409f45217e94fffce9011c703f334492a228fc6efa1ba55f65f67 size: 941
DONE
```

**Tag overwrite behavior:** The `images:` push overwrote the tag
`buildx-prov-test-image` — the multi-platform manifest (`sha256:45b0dad...`)
was replaced by the single-platform amd64 image (`sha256:ae13bcc...`).
A production pipeline MUST use dual tags: one for the multi-platform image
(e.g., `TAG-image`) and one for the provenance-bearing image (e.g., `TAG-attested`).

**Conclusion:** The pullback pattern works. Cloud Build generates SLSA Build
Level 3 provenance with both v0.1 and v1 predicate formats, dual-signed by
Google's verified-builder attestor keys. Cosign fallback (Hypothesis C) is
not needed.

### Summary of Pipeline Changes Required

Based on these experiments, the production pipeline changes are:

| Current Step | New Step | Change |
|---|---|---|
| 02: get-docker-token | (removed) | CB pre-populates credentials |
| 03: docker-login-gar | (removed) | CB pre-populates credentials |
| 06: build-and-export (OCI tar) | build-and-push (buildx --push) | Direct push to GAR |
| 07: push-with-crane | (removed) | Replaced by buildx --push |
| 07b: split-oci-platform (skopeo) | (redesign needed) | No local OCI layout; SBOM must pull from registry |
| 08: sbom-and-summary (syft) | (redesign needed) | Must analyze pulled image or registry image |
| (none) | pullback | `docker pull` for `images:` provenance |
| (none) | `images:` + `VERIFIED` in config | Triggers CB-native SLSA provenance |

Net effect: remove 4 steps (02, 03, 07, 07b), modify 2 (06, 08), add 1 (pullback).
Plus config changes (`images:`, `requestedVerifyOption`).

### Test Configuration Files

The cloudbuild.json files used for these experiments are committed to the repo:
- `Memos/experiments/cloudbuild-test-buildx-a.json` — Experiment 1 (Variant A, with docker login)
- `Memos/experiments/cloudbuild-test-buildx-b.json` — Experiment 2 (Variant B, ADC only)
- `Memos/experiments/cloudbuild-test-provenance.json` — Experiment 3 (provenance)

These are throwaway test configs, not production build definitions.

## Experiment Plan

Priority: exhaust CB-native SLSA path before considering cosign.

1. **test-buildx-push-gar** — DONE (2026-03-05). Both variants succeeded.
   Build IDs: `a3e5c2d7` (Variant A), `4de1467a` (Variant B).
2. **test-pullback-images-verified** — DONE (2026-03-05). SLSA Build Level 3 confirmed.
   Build ID: `48b818ed`.
3. **stitch-provenance-fix** — NEXT. Apply validated approach to
   `zrbf_stitch_build_json`. Remove steps 02, 03, 07; replace 06 with --push;
   add pullback step; add `images:` and `requestedVerifyOption: VERIFIED`.
   SBOM/skopeo steps (07b, 08) need redesign for registry-based analysis.
4. **rbscb-provenance-posture-update** — Crystallize provenance posture in RBSCB
   roadmap based on confirmed results.
5. **test-cosign-keyless-signing** — NOT NEEDED. CB-native SLSA path succeeded.
