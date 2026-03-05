# Paddock: rbw-e2e-cbv2-provenance

## Context

End-to-end verification of the Cloud Build v2 trigger pipeline with SLSA v1.0
provenance. This heat validates the infrastructure built in ₣Ai
(gcb-trigger-migration-tier3).

### System Architecture (built by ₣Ai)

**Rubric architecture:** Per-vessel `cloudbuild.json` committed to main repo,
synced to a shared rubric repo (`RBRR_RUBRIC_REPO_URL`) by the inscribe command.
Cloud Build sees only the rubric repo via CB v2 connection — the main repo is
never exposed to Google. Zero custom substitution overrides; all values baked
at inscribe time.

**CB v2 connections** (replaced Developer Connect): Fully programmatic setup via
REST API. Uses GitLab `gitlabConfig` with project access token (3 secrets in
Secret Manager). No browser OAuth consent flow. Connection + repository created
during `depot_create`. GitLab chosen over GitHub for repository-scoped PAT
security (see ₣Ai trophy, `rbgm_gitlab_setup()`).

**Key regime variables:**
- `RBRR_RUBRIC_REPO_URL` — plain HTTPS URL to rubric repo (no credentials)
- `RBRR_CBV2_CONNECTION_NAME` — CB v2 connection identifier
- `RBRR_GCB_SKOPEO_IMAGE_REF` — skopeo image pin (needs refresh on depot create)
- PAT in Secret Manager (3 secrets: `RBGC_CBV2_API_TOKEN_SECRET_NAME`,
  `RBGC_CBV2_READ_TOKEN_SECRET_NAME`, `RBGC_CBV2_WEBHOOK_SECRET_NAME`)

### Decision: Private Pool Always (2026-03-03)

**Burn the default-pool bridge.** Every depot gets a private pool. No conditional
path in stitch, no "optional" regime variable, no two configurations to debug.
See ₢AlAAB for details.

### Decision: Single-Arch First (2026-03-05)

**Build SLSA provenance capability incrementally.** Single-architecture vessels
get full SLSA v1.0 Level 3 provenance first, establishing a proven baseline.

**Rationale:** CB-native SLSA requires images in Docker's local store via
`images:` field. Docker's local store is single-platform only. Multi-platform
manifests cannot exist in the local store. Single-arch vessels sidestep this
entirely — `docker buildx build --load` puts the image in the local daemon,
`images:` + `VERIFIED` generates SLSA Level 3.

**What this means for vessels:**
- Single-arch vessels get full SLSA provenance immediately
- Multi-platform vessels are rejected by stitch with a clear error
- Busybox bifurcated into rbev-busybox-amd64 and rbev-busybox-arm64
- trbim-macos is already arm64-only — free test target
- Other multi-platform vessels (5 total) remain as-is but cannot build
  until either: (a) they are bifurcated, or (b) multi-platform provenance
  is implemented

### Decision: Image Tag Uses Inscribe Timestamp Only (₢AlAAJ, 2026-03-05)

**CB `images:` field requires a tag constructable from CB substitutions.** The
`images:` field is static config — it cannot reference runtime-computed values
like `TAG_BASE` (which includes the build timestamp from step 01). Therefore:

- **Primary image tag:** `${_RBGY_INSCRIBE_TIMESTAMP}${_RBGY_ARK_SUFFIX_IMAGE}`
  (e.g., `i20260224_153022-image`). All components are CB substitutions.
- **Metadata container tag:** still uses dual-timestamp `TAG_BASE` + `-about`
  suffix, pushed by `docker push` in step 06 (not via `images:`).
- **TAG_BASE** (`inscribe_ts-bBUILD_TS`) remains for metadata/logging in
  `build_info.json` but is no longer the primary image tag.

**Syft transport changed:** Syft now scans via `docker:IMAGE_URI` transport
(reads from local daemon via Docker socket) instead of `oci-dir:` (which
required the OCI Layout Bridge). Step 04 mounts `/var/run/docker.sock`.

### Multi-Platform Provenance: Viable Path Identified (research 2026-03-05)

CB structural constraint: `images:` can only push single-platform from the
Docker daemon (classic image store, Docker 20.10.24). The goal remains full
SLSA v1.0 on multi-platform images.

**Web research (2026-03-05) identified a viable rejoining path:**

1. CB `images:` field accepts a list — each URI gets independent SLSA
   provenance (documented in CB build config schema and provenance docs).
2. `docker buildx imagetools create` now preserves attestation manifests when
   combining per-platform images into a multi-platform index (buildx PR #3433).
   Operates registry-side, no local daemon needed for reassembly.
3. BuildKit already generates per-platform attestations on `--push`
   multi-platform builds. CB-native SLSA (from `images:`) is additive.

**Synthesized path (₢AlAAQ will validate):**
- `buildx --push` multi-platform → extract per-platform digests via
  `imagetools inspect --raw` → pull each by digest into local daemon →
  tag individually → declare all in `images:` → CB provenance on each →
  then `imagetools create` to reconstruct multi-platform manifest list

**Key untested link:** Can `images:` push a foreign-arch image from the local
daemon? This is exactly what ₢AlAAQ Variant B tests.

**Full research evidence:** `Memos/memo-20260305-provenance-architecture-gap.md`
(section: "Multi-Platform Rejoining Research")

### Architectural Intent: Bifurcation Is Temporary

Vessel bifurcation (`rbev-busybox-amd64`, `rbev-busybox-arm64`) is scaffolding
for the single-arch provenance milestone. The target architecture is that
vessels remain multi-platform with `RBRV_CONJURE_PLATFORMS` containing multiple
platforms. If ₢AlAAQ succeeds, the stitch function gains a multi-platform code
path: `--push` + per-platform pullback + multi-URI `images:` + manifest
reassembly via `imagetools create`. Single-platform vessels keep the simpler
`--load` path.

Whether results lead to implementation in this heat or a successor heat
is a decision for after the experiment.

### Provenance Experiments Validated (₢AlAAK, ₢AlAAL, 2026-03-05)

Three experiments on demo1025 via `gcloud builds submit --no-source`:

| Experiment | Build ID | Result |
|---|---|---|
| Variant A: buildx --push with docker login | a3e5c2d7 | SUCCESS |
| Variant B: buildx --push ADC only | 4de1467a | SUCCESS (docker login unnecessary) |
| Provenance: --push + pullback + images: + VERIFIED | 48b818ed | SLSA Level 3 |

**Critical finding:** Cloud Build pre-populates `/builder/home/.docker/config.json`
with oauth2accesstoken for ALL GAR regions. Steps 02 (get-docker-token) and
03 (docker-login-gar) are completely unnecessary.

**Toolchain versions (confirmed working):**
- Docker Engine 20.10.24, buildx v0.23.0, BuildKit moby/buildkit:buildx-stable-1
- Builder image: gcr.io/cloud-builders/docker@sha256:efdbd755...

**Full evidence:** `Memos/memo-20260305-provenance-architecture-gap.md`

### Pipeline: Before and After

**Before (10 steps, OCI Layout Bridge):**
1. derive-tag-base (gcloud)
2. get-docker-token (gcloud) — REMOVING
3. docker-login-gar (docker) — REMOVING
4. qemu-binfmt (docker)
5. build-and-export (docker → OCI tar) — REPLACING with --load
6. push-with-crane (alpine) — REMOVING
7. split-oci-platform (skopeo) — REMOVING (single-arch)
8. sbom-and-summary (docker/syft) — REWORKING (scan local daemon image)
9. assemble-metadata (alpine) — REWORKING (derive URI from substitutions)
10. build-and-push-metadata (docker)

**After (single-arch, ~6 steps + images: push):**
1. derive-tag-base
2. qemu-binfmt (if cross-arch, e.g., arm64 on amd64 worker)
3. build-and-load (buildx --load, single platform)
4. sbom (Syft scans local daemon image)
5. assemble-metadata (derive URI from substitutions)
6. build-and-push-metadata
+ `images:` field triggers CB-native push + SLSA provenance
+ `requestedVerifyOption: VERIFIED` in options

### Pace Threading

```
₢AlAAJ stitch-single-arch-slsa      Pipeline code (stitch + step scripts)
  │
  ├─→ ₢AlAAN bifurcate-busybox       Vessel directories (can parallel with AlAAJ)
  │     │
  │     v
  └──→ ₢AlAAO verify-single-arch-e2e  Live infrastructure: inscribe → dispatch → verify
          │
          v
        ₢AlAAP spec-single-arch-prov   RBS0, RBSOB updates (confirmed facts only)
          │
          ├─→ ₢AlAAQ experiment-multiplatform-slsa  Per-platform provenance experiment
          │
          v
        ₢AlAAM rbscb-posture-update    Roadmap (after single-arch AND experiment results)
```

₢AlAAJ and ₢AlAAN are independent — can execute in parallel.
₢AlAAO requires both complete + possibly depot cycle for new triggers.
₢AlAAP and ₢AlAAQ can execute in parallel after ₢AlAAO.
₢AlAAM waits for both spec and experiment results before crystallizing roadmap.

### Vessel Landscape

| Vessel | Current Platforms | Status |
|---|---|---|
| rbev-busybox | amd64, arm64, arm/v7 | Retained multi-platform; stitch rejects until multi-platform support |
| rbev-busybox-amd64 | linux/amd64 | CREATED (₢AlAAN) — binfmt deny (native on CB workers) |
| rbev-busybox-arm64 | linux/arm64 | CREATED (₢AlAAN) — binfmt allow (QEMU on amd64 workers) |
| trbim-macos | arm64 | Already single-arch, free test target |
| rbev-bottle-anthropic-jupyter | amd64, arm64 | Multi-platform, deferred |
| rbev-bottle-plantuml | amd64, arm64 | Multi-platform, deferred |
| rbev-bottle-ubuntu-test | amd64, arm64 | Multi-platform, deferred |
| rbev-sentry-ubuntu-large | amd64, arm64 | Multi-platform, deferred |
| rbev-ubu-safety | amd64, arm64 | Multi-platform, deferred |
| rbev-nginx-ward | (bind mode) | No build |

### Bifurcation Policy (₢AlAAN, 2026-03-05)

**Original `rbev-busybox`:** Kept as-is with 3-platform config. The single-arch
gate in stitch (₢AlAAJ) rejects it at inscribe time. It remains available for
future multi-platform provenance work (₢AlAAQ path). No trigger needed until
multi-platform support lands.

**Other multi-platform vessels** (5 total: anthropic-jupyter, plantuml,
ubuntu-test, sentry-ubuntu-large, ubu-safety): Unchanged. Same stitch gate
rejects them. They are NOT bifurcated — bifurcation is only for the busybox
test target. Production vessels will wait for native multi-platform provenance.

**Single-arch test targets for ₢AlAAO e2e:**
- `rbev-busybox-amd64` — native build on CB amd64 workers (no QEMU)
- `rbev-busybox-arm64` — cross-build via QEMU on CB amd64 workers
- `trbim-macos` — existing arm64-only vessel (zero changes needed)

### Current State

**Depot demo1025** exists with CB v2 GitLab connection and private pool.
Rubric repo at `gitlab.com/bhyslop/rb-rubric.git`. 7 vessel triggers created.
Busybox builds and pushes successfully (no provenance yet — pipeline not updated).

**All known issues from ₣Ai e2e (2026-03-03) are FIXED:**
1. Push triggers fired on inscribe push — FIXED (₢AiABC: unmatchable branch filter)
2. IAM read-modify-write race — FIXED (₢AiABD: declarative policy writes)
3. Syft multi-platform OCI layout — FIXED (₢AiABE: skopeo split)
4. Build step `dir` field missing — FIXED in ₣Ai

**Depot may need destroy+create cycle** for new vessel triggers (busybox-amd64,
busybox-arm64). User pre-approved depot reconfiguration (2026-03-05).

## Build Requirements

This heat is primarily operator-guided e2e execution (live GCP infrastructure).
BCG compliance mandatory for any new bash code. Pin refresh will occur during
depot create — `RBRR_GCB_SKOPEO_IMAGE_REF` will get a real digest at that time.

## References

- `lenses/RBSCB-CloudBuildRoadmap.adoc` — Tier definitions
- `lenses/RBSTB-trigger_build.adoc` — Trigger build spec
- `lenses/RBSRI-rubric_inscribe.adoc` — Inscribe spec
- `lenses/RBSDC-depot_create.adoc` — Depot create spec
- `Tools/rbw/rbf_Foundry.sh` — Stitch function + build dispatch
- `Tools/rbw/rbgjb/*.sh` — Step scripts (source of truth)
- `Memos/memo-20260303-cloudbuild-trigger-anatomy.md` — Trigger body research
- `Memos/memo-20260305-provenance-architecture-gap.md` — Provenance research + experiment results
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — BCG patterns
- CB v2 API: https://cloud.google.com/build/docs/api/reference/rest/v2/projects.locations.connections
- SLSA provenance: https://cloud.google.com/build/docs/securing-builds/generate-validate-build-provenance