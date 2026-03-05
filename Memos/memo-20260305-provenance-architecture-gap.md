# Cloud Build Provenance Architecture Gap

**Date:** 2026-03-05
**Heat:** rbw-e2e-cbv2-provenance (Al)
**Pace:** research-cloudbuild-provenance-mechanics (AlAAI)
**Status:** Working hypotheses — not yet validated by experiment

## Summary

Cloud Build's native SLSA v1.0 provenance is structurally incompatible with the
OCI Layout Bridge push architecture. Adding `requestedVerifyOption: VERIFIED` or
the `images:` field to the current pipeline would **break builds**, not just skip
provenance. The primary path forward is to restructure the push so Cloud Build
can generate native SLSA provenance. Cosign attestation is a fallback if the
native path proves impossible.

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

## Working Hypotheses (Untested)

### Hypothesis A: buildx --push eliminates OCI Layout Bridge (prerequisite)

Multiple documented examples show `docker buildx build --push` working in Cloud
Build without explicit `docker login`, using ADC (Application Default
Credentials) from the `gcr.io/cloud-builders/docker` image. The BuildKit
`authprovider` session mechanism forwards credentials from the buildx client to
the BuildKit daemon container via gRPC.

If this works, crane (step 07) and the OCI Layout Bridge become unnecessary.

**Known risks:**
- 1-hour token expiry for long builds ([docker/buildx#1205](https://github.com/docker/buildx/issues/1205))
- DIND credential isolation edge cases ([docker/buildx#3050](https://github.com/docker/buildx/issues/3050))
- Docker 28.3.0+ `DOCKER_AUTH_CONFIG` regression ([docker/cli#6156](https://github.com/docker/cli/issues/6156))
- `DOCKER_CONFIG` breaks buildx plugin discovery ([docker/cli#5477](https://github.com/docker/cli/issues/5477) — still open)

**Does not solve provenance alone** — even with `--push`, the image is not in
Docker's local store, so `images:` still can't push it. But this is the
prerequisite for Hypothesis B.

### Hypothesis B: pull-back + images: + VERIFIED = CB-native SLSA (primary goal)

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

### Hypothesis C: cosign keyless signing (fallback only)

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

## Experiment Plan

Priority: exhaust CB-native SLSA path before considering cosign.

1. **test-buildx-push-gar** — Run busybox build with `--push` instead of OCI
   export + crane on demo1025. Validates Hypothesis A (prerequisite for B).
2. **test-pullback-images-verified** — If experiment 1 succeeds: pull image back
   to local daemon, add `images:` + `VERIFIED`, test whether CB generates SLSA
   provenance on the pulled-back digest. Validates Hypothesis B (primary goal).
3. **stitch-provenance-fix** — Apply validated approach to
   `zrbf_stitch_build_json`. Content depends on experiment results.
4. **rbscb-provenance-posture-update** — Crystallize provenance posture in RBSCB
   roadmap based on confirmed results.
5. **test-cosign-keyless-signing** — Only if experiments 1+2 fail to produce
   CB-native SLSA. Validates Hypothesis C (fallback).
