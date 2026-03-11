# Heat Trophy: rbw-e2e-cbv2-provenance

**Firemark:** ₣Al
**Created:** 260303
**Retired:** 260310
**Status:** retired

## Paddock

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

### Multi-Platform Provenance: VALIDATED (₢AlAAQ, 2026-03-05)

**Full path proven.** Per-platform pullback produces SLSA v1.0 Level 3 on all
platforms within a single build invocation.

**Experiment 4** (build `b3fd60c7`): `buildx --push` 3 platforms → `docker pull
--platform` each arch → tag individually → `images:` declares all 3 →
SLSA Level 3 on amd64, arm64, armv7. Same `buildInvocationId` proves
single-build origin. Uses Variant A (`--platform` flag), not digest-based
pulls (Variant B `docker manifest inspect` failed with "no such manifest").

**Experiment 5** (build `8cd7b713`): `imagetools create` reassembles per-platform
images into multi-platform manifest list. Per-platform SLSA provenance preserved.
Combined manifest itself has `slsa_build_level: unknown` (expected — CB didn't
build the manifest list), but consumers get transparent platform resolution to
attested per-platform images.

**Key findings:**
- `docker pull --platform` works on CB workers (Docker 20.10.24, Experimental: true)
- Foreign-arch images pushed successfully by `images:` (CB pushes bytes, not execution)
- `$${VAR}` escaping required for shell variables in cloudbuild.json
- `jq` not available in `gcr.io/cloud-builders/docker`

**Full evidence:** `Memos/memo-20260305-provenance-architecture-gap.md`
(sections: "Multi-Platform Provenance Experiment Results", "Validated Multi-Platform Provenance Architecture")

### Architectural Intent: Bifurcation No Longer Necessary

Vessel bifurcation (`rbev-busybox-amd64`, `rbev-busybox-arm64`) was scaffolding
for the single-arch milestone. With ₢AlAAQ validated, multi-platform vessels can
retain `RBRV_CONJURE_PLATFORMS` with multiple platforms. The stitch function
needs two code paths: `--load` for single-platform vessels, `--push` +
per-platform pullback + multi-URI `images:` + `imagetools create` reassembly
for multi-platform vessels.

### Decision: Single-Build Reassembly (Experiment 6, 2026-03-05)

**`imagetools create` runs within the same CB build — no post-build step needed.**
Build `6661d0cd` (33s) proved the full pipeline in one invocation:

1. `buildx --push` (3 platforms) → `:varG-multi`
2. Per-platform pullback (`docker pull --platform`)
3. `docker push` each per-platform tag (pre-pushes to registry)
4. `imagetools create` assembles combined manifest list (reads from registry)
5. `images:` field re-pushes same per-platform tags → SLSA Level 3 (idempotent)

**Key findings:**
- `docker push` works mid-build (pre-populated GAR credentials)
- `imagetools create` can reference images pushed by `docker push` in prior steps
- `images:` re-push is idempotent — same content, same digest, provenance generated
- Combined manifest list (`:varG-combined`, `slsa_build_level: unknown`) survives
  the `images:` re-push intact
- Zero new dependencies — only `docker` + `buildx` in `gcr.io/cloud-builders/docker`

**This eliminates options (b) Pub/Sub, (c) local post-dispatch, (d) separate gcloud
build.** The operator dispatches, cloud executes — no local environment in the
provenance chain.

**Config:** `Memos/experiments/cloudbuild-test-single-build-reassembly.json`

### Decisions: Multi-Platform SBOM, Tags, Metadata (₢AlAAS design, 2026-03-05)

**SBOM strategy:** Per-platform Syft scans → per-platform SBOMs. No weakening
of the per-image documentation premise. Each architecture gets its own SBOM
describing exactly that platform image's dependencies.

**Tag scheme — platform-transparent consumer tags:**

| Tag | Purpose |
|-----|---------|
| `{INSCRIBE_TS}-multi` | Intermediate `buildx --push` target |
| `{INSCRIBE_TS}{ARK_SUFFIX}-amd64` | Per-platform (`images:` field, SLSA) |
| `{INSCRIBE_TS}{ARK_SUFFIX}` | Consumer-facing (reassembled manifest list) |
| `{TAG_BASE}-about` | Metadata container (multi-platform) |

Platform suffix: `linux/amd64` → `-amd64`, `linux/arm64` → `-arm64`,
`linux/arm/v7` → `-armv7`. Computed at inscribe time from `RBRV_CONJURE_PLATFORMS`.

**Multi-platform `-about` build:** `FROM scratch` + buildx `TARGETARCH`/
`TARGETVARIANT` auto-args select per-platform files in one `buildx --push`
invocation. No QEMU needed. Changes `-about` from `docker build` + `docker push`
to `buildx --push`.

**build_info.json:** Per-platform (not shared). Per-platform fields: platform
string, image digest, QEMU used. Shared fields: build ID, timestamps, git
commit, vessel name. SLSA summary fields added: `slsa_build_level`,
`build_invocation_id`, `provenance_predicate_types`, `provenance_builder_id`.

### Multi-Platform Pipeline Shape (target for ₢AlAAS)

1. derive-tag-base
2. qemu-binfmt
3. buildx --push (all platforms → `-multi` tag)
4. per-platform pullback (docker pull --platform → docker tag)
5. docker push per-platform tags (pre-push for imagetools)
6. syft scan each per-platform image sequentially (docker: transport)
7. generate per-platform build_info.json with SLSA summary
8. buildx --push multi-platform -about (FROM scratch + TARGETARCH)
9. imagetools create → consumer-facing manifest list (in-build step)
+ `images:` re-pushes per-platform tags (idempotent) → SLSA Level 3
+ `requestedVerifyOption: VERIFIED`

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

### E2E Results: SLSA v1.0 Build Level 3 Achieved (₢AlAAO, 2026-03-05)

Full production pipeline: pin refresh → inscribe → trigger dispatch → verify.
Inscribe timestamp `i20260305_133650`, rubric commit `392b95ec`.

| Vessel | Build ID | SLSA Level | Predicates |
|---|---|---|---|
| rbev-busybox-amd64 | `16d3b60f` | **3** | v0.1 + v1 |
| rbev-busybox-arm64 | `fc36b970` | **3** | v0.1 + v1 |
| trbim-macos | `9180c42a` | **3** | v0.1 + v1 |

Inscribe created triggers for new vessels automatically (no depot destroy/create).
Two bugs fixed during e2e: crane grep `readonly` mismatch, inscribe die-on-multiplatform.

Full evidence: `Memos/memo-20260305-provenance-architecture-gap.md` (Production Pipeline Results)

### Current State

**Depot demo1025** exists with CB v2 GitLab connection and private pool.
Rubric repo at `gitlab.com/bhyslop/rb-rubric.git`. 9 vessel triggers (7 original + 2 new).
Three single-arch vessels building with SLSA v1.0 Build Level 3 provenance.

**Spec updated (₢AlAAR):** RBS0 `rbtgr_provenance` now documents the validated
multi-platform architecture (per-platform pullback + `images:` + `imagetools create`
reassembly) with target pipeline step sequence. RBSOB superseded notice updated
to reflect experimental validation (full supersession pending ₢AlAAT production e2e).

**All known issues from ₣Ai e2e (2026-03-03) are FIXED:**
1. Push triggers fired on inscribe push — FIXED (₢AiABC: unmatchable branch filter)
2. IAM read-modify-write race — FIXED (₢AiABD: declarative policy writes)
3. Syft multi-platform OCI layout — FIXED (₢AiABE: skopeo split)
4. Build step `dir` field missing — FIXED in ₣Ai

### Decision: SLSA Vouch and Consecration Check (₢AlAAV, 2026-03-05)

**Two role-separated tabtargets** for SLSA provenance verification:

- `rbw-Dc.DirectorChecksConsecrations.sh <vessel-dir>` — lists consecrations
  from GAR tags. No-arg lists vessels. Director auth.
- `rbw-Rv.RetrieverVouchesArk.sh <vessel-dir>` — vouches most recent
  consecration via Container Analysis API. No-arg lists vessels.

**Role split rationale:** Director lists consecrations ("what did I build?").
Retriever vouches provenance ("is this safe to consume?"). SLSA exists for
consumers — vouch is naturally a Retriever operation.

**Auth strategy (interim):** Both use Director RBRA credentials for now.
Director has `roles/viewer` which includes `containeranalysis.occurrences.list`.
Retriever auth path deferred — swap RBRA file loading when ready.

**IAM landed:** `roles/containeranalysis.occurrences.viewer` added to
`rbgg_create_retriever()` and `RBGC_ROLE_CONTAINERANALYSIS_OCCURRENCES_VIEWER`
constant added. Existing Retrievers need manual grant or depot cycle.

**Colophon rationale:** `rbw-Dc` lowercase `c` = read-only check. `rbw-Rv`
lowercase `v` = read-only vouch. Both diagnostic, no state alteration.

**Implementation status (in-progress):**
- Tabtarget files created and executable
- Zipper enrollments added (`RBZ_CHECK_CONSECRATIONS`, `RBZ_VOUCH_ARK`)
- Draft functions in `rbf_Foundry.sh` — need BCG review (stderr capture,
  temp file patterns, `local -r`, no piped while-read subshells)
- The consecration check is simpler (one curl + tag parsing); vouch is
  more complex (loop over per-platform images, Container Analysis API)

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

## Paces

### fix-consecration-discovery-strong-tie (₢AlAAb) [complete]

**[260306-1304] complete**

Fix consecration discovery to use strong build-ID tie instead of inferential GAR tag scanning.

## Problem

`rbf_build` discovers the consecration by scanning GAR tags for the first about tag matching the inscribe timestamp. When multiple builds share the same inscribe timestamp (same rubric, different dispatches), this finds stale about tags from previous builds that were never abjured.

## Repair A — Root cause: step output (rbf_Foundry.sh + step 01)

Cloud Build steps can write up to 50 bytes to `/builder/outputs/output`, which appears in `results.buildStepOutputs[N]` (base64-encoded). Step 01 (derive-tag-base) already computes TAG_BASE (= consecration). Add one line to step 01:

    echo -n "${TAG_BASE}" > /builder/outputs/output

Then replace the GAR tag scanning in `rbf_build` consecration discovery (lines ~672-694) with:

    z_found_consecration=$(echo "${step0_output}" | base64 -d)

This eliminates GAR scanning entirely. Direct, zero inference.

Requires: re-inscribe after step script change (stitch regenerates cloudbuild.json).

## Repair B — Test safety net: build ID assertion (rbtcsl_SlsaProvenance.sh)

1. Add `RBF_FACT_BUILD_ID` constant to rbgc_Constants.sh
2. Have `rbf_build` write the dispatched build ID to `${BURD_OUTPUT_DIR}/${RBF_FACT_BUILD_ID}`
3. Have `rbf_vouch` write the provenance `buildInvocationId` to `${BURD_OUTPUT_DIR}/${RBF_FACT_BUILD_ID}`
4. In `rbtcsl_provenance_tcase`, assert conjure's build ID == vouch's build ID

This catches stale-consecration bugs at test time even if Repair A has a regression.

## Spec assessment

RBS0 `rbtgr_provenance` section documents the multi-platform pipeline steps. Step 01 output behavior should be noted there. The `rbtga_ark_vouch` section should note the build-ID cross-check.

## Acceptance Criteria

- Step 01 writes TAG_BASE to `/builder/outputs/output`
- `rbf_build` reads consecration from `buildStepOutputs[0]`, no GAR tag scan
- Test asserts conjure build ID matches vouch provenance build ID
- RBS0 updated with step output and build-ID cross-check documentation
- Re-inscribe + dispatch + vouch passes on rbev-busybox

**[260306-1235] rough**

Fix consecration discovery to use strong build-ID tie instead of inferential GAR tag scanning.

## Problem

`rbf_build` discovers the consecration by scanning GAR tags for the first about tag matching the inscribe timestamp. When multiple builds share the same inscribe timestamp (same rubric, different dispatches), this finds stale about tags from previous builds that were never abjured.

## Repair A — Root cause: step output (rbf_Foundry.sh + step 01)

Cloud Build steps can write up to 50 bytes to `/builder/outputs/output`, which appears in `results.buildStepOutputs[N]` (base64-encoded). Step 01 (derive-tag-base) already computes TAG_BASE (= consecration). Add one line to step 01:

    echo -n "${TAG_BASE}" > /builder/outputs/output

Then replace the GAR tag scanning in `rbf_build` consecration discovery (lines ~672-694) with:

    z_found_consecration=$(echo "${step0_output}" | base64 -d)

This eliminates GAR scanning entirely. Direct, zero inference.

Requires: re-inscribe after step script change (stitch regenerates cloudbuild.json).

## Repair B — Test safety net: build ID assertion (rbtcsl_SlsaProvenance.sh)

1. Add `RBF_FACT_BUILD_ID` constant to rbgc_Constants.sh
2. Have `rbf_build` write the dispatched build ID to `${BURD_OUTPUT_DIR}/${RBF_FACT_BUILD_ID}`
3. Have `rbf_vouch` write the provenance `buildInvocationId` to `${BURD_OUTPUT_DIR}/${RBF_FACT_BUILD_ID}`
4. In `rbtcsl_provenance_tcase`, assert conjure's build ID == vouch's build ID

This catches stale-consecration bugs at test time even if Repair A has a regression.

## Spec assessment

RBS0 `rbtgr_provenance` section documents the multi-platform pipeline steps. Step 01 output behavior should be noted there. The `rbtga_ark_vouch` section should note the build-ID cross-check.

## Acceptance Criteria

- Step 01 writes TAG_BASE to `/builder/outputs/output`
- `rbf_build` reads consecration from `buildStepOutputs[0]`, no GAR tag scan
- Test asserts conjure build ID matches vouch provenance build ID
- RBS0 updated with step output and build-ID cross-check documentation
- Re-inscribe + dispatch + vouch passes on rbev-busybox

### research-cloudbuild-provenance-mechanics (₢AlAAI) [complete]

**[260305-1141] complete**

Investigate Cloud Build SLSA provenance generation requirements.

## Context

Busybox build succeeded on demo1025 private pool (build ID `683848ee-6b97-42cf-819d-cba8af792e8e`) but `slsa_build_level: "unknown"` — no provenance attestation generated. Two gaps identified:

1. `options.requestedVerifyOption: VERIFIED` is not set in the build config generated by `zrbf_stitch_build_json` in `Tools/rbw/rbf_Foundry.sh`
2. The top-level `images:` field is empty because we push via crane/skopeo (OCI Layout Bridge) rather than Cloud Build's native push. Cloud Build provenance may be tied to images declared in `images:`.

## OCI Layout Bridge Architecture

The build pipeline uses a two-phase push to work around BuildKit credential isolation:
- Phase 1 (step 05, docker buildx): builds multi-platform image, exports to `/workspace/oci-layout.tar`
- Phase 2 (step 06, crane/skopeo): pushes OCI archive to GAR

This means Cloud Build never performs the image push itself — build steps do it. The `images:` field in the build config is empty, so Cloud Build doesn't track what digest was produced.

## Key Files

- `Tools/rbw/rbf_Foundry.sh` — `zrbf_stitch_build_json` generates `cloudbuild.json`
- `Tools/rbw/rbgjb/*.sh` — individual build step scripts (source of truth for step content)
- `rbev-vessels/rbev-busybox/cloudbuild.json` — example generated build config
- `lenses/RBS0-SpecTop.adoc` — top-level spec (provenance requirements go here)
- `lenses/RBSOB-oci_layout_bridge.adoc` — OCI Layout Bridge spec

## Diagnostic Commands

Inspect the successful build:
```
gcloud builds describe 683848ee-6b97-42cf-819d-cba8af792e8e --project=rbwg-d-demo1025-260304183118 --region=us-central1 --format=json
```

Check provenance on image:
```
gcloud artifacts docker images describe us-central1-docker.pkg.dev/rbwg-d-demo1025-260304183118/rbw-demo1025-repository/rbev-busybox:i20260304_193126-b20260305_033239-image --show-provenance --format=json
```

## Research Questions

1. Does adding `requestedVerifyOption: VERIFIED` alone generate provenance for trigger-invoked builds on private pools?
2. Does the top-level `images:` field need to be populated for provenance to attach? If so, can it reference an image URI pattern (with tag) that was pushed by a build step?
3. Are there alternative provenance mechanisms (e.g., `--requested-verify-option` on trigger creation, or Artifact Analysis API attestations)?
4. What is the interaction between private pools and provenance generation? Any limitations?
5. Does the CB v2 (GitLab) connection type affect provenance availability?

## Acceptance Criteria

- Clear answers to all 5 research questions with GCP documentation references
- Update RBS0 with provenance requirements (new section or update existing)
- Recommendation for stitch changes needed in ₢AlAAJ

**[260304-1942] rough**

Investigate Cloud Build SLSA provenance generation requirements.

## Context

Busybox build succeeded on demo1025 private pool (build ID `683848ee-6b97-42cf-819d-cba8af792e8e`) but `slsa_build_level: "unknown"` — no provenance attestation generated. Two gaps identified:

1. `options.requestedVerifyOption: VERIFIED` is not set in the build config generated by `zrbf_stitch_build_json` in `Tools/rbw/rbf_Foundry.sh`
2. The top-level `images:` field is empty because we push via crane/skopeo (OCI Layout Bridge) rather than Cloud Build's native push. Cloud Build provenance may be tied to images declared in `images:`.

## OCI Layout Bridge Architecture

The build pipeline uses a two-phase push to work around BuildKit credential isolation:
- Phase 1 (step 05, docker buildx): builds multi-platform image, exports to `/workspace/oci-layout.tar`
- Phase 2 (step 06, crane/skopeo): pushes OCI archive to GAR

This means Cloud Build never performs the image push itself — build steps do it. The `images:` field in the build config is empty, so Cloud Build doesn't track what digest was produced.

## Key Files

- `Tools/rbw/rbf_Foundry.sh` — `zrbf_stitch_build_json` generates `cloudbuild.json`
- `Tools/rbw/rbgjb/*.sh` — individual build step scripts (source of truth for step content)
- `rbev-vessels/rbev-busybox/cloudbuild.json` — example generated build config
- `lenses/RBS0-SpecTop.adoc` — top-level spec (provenance requirements go here)
- `lenses/RBSOB-oci_layout_bridge.adoc` — OCI Layout Bridge spec

## Diagnostic Commands

Inspect the successful build:
```
gcloud builds describe 683848ee-6b97-42cf-819d-cba8af792e8e --project=rbwg-d-demo1025-260304183118 --region=us-central1 --format=json
```

Check provenance on image:
```
gcloud artifacts docker images describe us-central1-docker.pkg.dev/rbwg-d-demo1025-260304183118/rbw-demo1025-repository/rbev-busybox:i20260304_193126-b20260305_033239-image --show-provenance --format=json
```

## Research Questions

1. Does adding `requestedVerifyOption: VERIFIED` alone generate provenance for trigger-invoked builds on private pools?
2. Does the top-level `images:` field need to be populated for provenance to attach? If so, can it reference an image URI pattern (with tag) that was pushed by a build step?
3. Are there alternative provenance mechanisms (e.g., `--requested-verify-option` on trigger creation, or Artifact Analysis API attestations)?
4. What is the interaction between private pools and provenance generation? Any limitations?
5. Does the CB v2 (GitLab) connection type affect provenance availability?

## Acceptance Criteria

- Clear answers to all 5 research questions with GCP documentation references
- Update RBS0 with provenance requirements (new section or update existing)
- Recommendation for stitch changes needed in ₢AlAAJ

**[260304-1941] rough**

Investigate Cloud Build SLSA provenance generation requirements.

## Context

Busybox build succeeded on demo1025 private pool but `slsa_build_level: "unknown"` — no provenance attestation generated. Two gaps identified:

1. `options.requestedVerifyOption: VERIFIED` is not set in the build config
2. The `images:` field is empty because we push via crane/skopeo (OCI Layout Bridge) rather than Cloud Build's native push. Cloud Build provenance is tied to images declared in `images:`.

## Research Questions

1. Does adding `requestedVerifyOption: VERIFIED` alone generate provenance for trigger-invoked builds on private pools?
2. Does the top-level `images:` field need to be populated for provenance to attach? If so, can it reference an image digest that was pushed by a build step (not by Cloud Build's native push)?
3. Are there alternative provenance mechanisms (e.g., `--requested-verify-option` on trigger creation, or Artifact Analysis API attestations)?
4. What is the interaction between private pools and provenance generation? Any limitations?
5. Does the CB v2 (GitLab) connection type affect provenance availability?

## Acceptance Criteria

- Clear answers to all 5 research questions with GCP documentation references
- Update RBS0 with provenance requirements (new section or update existing)
- Recommendation for stitch changes needed

### test-buildx-push-gar (₢AlAAK) [complete]

**[260305-1209] complete**

Test whether docker buildx build --push can push multi-platform images directly
to GAR from Cloud Build, eliminating the OCI Layout Bridge.

## Context

Research (Memos/memo-20260305-provenance-architecture-gap.md) found multiple
documented examples of buildx --push working in Cloud Build using ADC from
gcr.io/cloud-builders/docker. If confirmed, this eliminates crane (step 07)
and simplifies the pipeline. Prerequisite for CB-native SLSA provenance path.

The RBSOB trade study assumed buildx docker-container driver can't push to GAR
due to credential isolation. BuildKit's authprovider session mechanism may
invalidate this — the buildx client reads Docker config and forwards credentials
to the BuildKit daemon via gRPC callback.

**Evidence supporting this hypothesis:**
- Google's Dataflow multi-arch container guide uses buildx --push in Cloud Build:
  https://docs.cloud.google.com/dataflow/docs/guides/multi-architecture-container
- docker/buildx#3050 maintainer confirmed (March 2025): "You don't need to do
  anything extra apart from docker login" for standard (non-DIND) usage
- Multiple community blog posts document working buildx --push to GAR in CB

## Experiment Mechanics

Use gcloud builds submit with an inline cloudbuild.json on demo1025. Do NOT
modify the committed vessel configs — this is a throwaway test build.

Two variants to test:

**Variant A (with existing docker login):**
Steps 01-03 as normal (derive tag, get token, docker login to GAR), then:
- docker buildx create --driver docker-container --name rb-builder --use
- docker buildx build --push --platform=linux/amd64,linux/arm64,linux/arm/v7 --tag IMAGE_URI .

**Variant B (ADC only, no docker login):**
Skip steps 02-03 entirely. Rely on Cloud Build service account ADC:
- docker buildx create --driver docker-container --name rb-builder --use
- docker buildx build --push --platform=linux/amd64,linux/arm64,linux/arm/v7 --tag IMAGE_URI .

Testing both tells us whether the OCI Layout Bridge workaround AND the
docker-login step are both unnecessary.

## Downstream Impact

If --push works, the skopeo split step (07b) and SBOM step (08) currently read
from /workspace/oci-layout which won't exist. These steps would need to pull
from the registry instead. Not in scope for this experiment — just verify push.

## Verification

- Image appears in GAR with correct multi-platform manifest
- All 3 platforms present: gcloud artifacts docker images list ... --include-tags
- Build completes without 401/403 errors
- Note which variant(s) succeeded

## Key Risks

- 1-hour token expiry (docker/buildx#1205) — busybox is fast, note for larger vessels
- Docker 28.3.0+ DOCKER_AUTH_CONFIG regression (docker/cli#6156)
- DIND credential isolation edge cases (docker/buildx#3050)

## Acceptance Criteria

- Multi-platform image successfully pushed to GAR via buildx --push (either variant)
- OR: documented failure mode with error details for both variants

**[260304-2027] rough**

Test whether docker buildx build --push can push multi-platform images directly
to GAR from Cloud Build, eliminating the OCI Layout Bridge.

## Context

Research (Memos/memo-20260305-provenance-architecture-gap.md) found multiple
documented examples of buildx --push working in Cloud Build using ADC from
gcr.io/cloud-builders/docker. If confirmed, this eliminates crane (step 07)
and simplifies the pipeline. Prerequisite for CB-native SLSA provenance path.

The RBSOB trade study assumed buildx docker-container driver can't push to GAR
due to credential isolation. BuildKit's authprovider session mechanism may
invalidate this — the buildx client reads Docker config and forwards credentials
to the BuildKit daemon via gRPC callback.

**Evidence supporting this hypothesis:**
- Google's Dataflow multi-arch container guide uses buildx --push in Cloud Build:
  https://docs.cloud.google.com/dataflow/docs/guides/multi-architecture-container
- docker/buildx#3050 maintainer confirmed (March 2025): "You don't need to do
  anything extra apart from docker login" for standard (non-DIND) usage
- Multiple community blog posts document working buildx --push to GAR in CB

## Experiment Mechanics

Use gcloud builds submit with an inline cloudbuild.json on demo1025. Do NOT
modify the committed vessel configs — this is a throwaway test build.

Two variants to test:

**Variant A (with existing docker login):**
Steps 01-03 as normal (derive tag, get token, docker login to GAR), then:
- docker buildx create --driver docker-container --name rb-builder --use
- docker buildx build --push --platform=linux/amd64,linux/arm64,linux/arm/v7 --tag IMAGE_URI .

**Variant B (ADC only, no docker login):**
Skip steps 02-03 entirely. Rely on Cloud Build service account ADC:
- docker buildx create --driver docker-container --name rb-builder --use
- docker buildx build --push --platform=linux/amd64,linux/arm64,linux/arm/v7 --tag IMAGE_URI .

Testing both tells us whether the OCI Layout Bridge workaround AND the
docker-login step are both unnecessary.

## Downstream Impact

If --push works, the skopeo split step (07b) and SBOM step (08) currently read
from /workspace/oci-layout which won't exist. These steps would need to pull
from the registry instead. Not in scope for this experiment — just verify push.

## Verification

- Image appears in GAR with correct multi-platform manifest
- All 3 platforms present: gcloud artifacts docker images list ... --include-tags
- Build completes without 401/403 errors
- Note which variant(s) succeeded

## Key Risks

- 1-hour token expiry (docker/buildx#1205) — busybox is fast, note for larger vessels
- Docker 28.3.0+ DOCKER_AUTH_CONFIG regression (docker/cli#6156)
- DIND credential isolation edge cases (docker/buildx#3050)

## Acceptance Criteria

- Multi-platform image successfully pushed to GAR via buildx --push (either variant)
- OR: documented failure mode with error details for both variants

**[260304-2017] rough**

Test whether docker buildx build --push can push multi-platform images directly
to GAR from Cloud Build, eliminating the OCI Layout Bridge.

## Context

Research (Memos/memo-20260305-provenance-architecture-gap.md) found multiple
documented examples of buildx --push working in Cloud Build using ADC from
gcr.io/cloud-builders/docker. If confirmed, this eliminates crane (step 07)
and simplifies the pipeline. Prerequisite for CB-native SLSA provenance path.

The RBSOB trade study assumed buildx docker-container driver can't push to GAR
due to credential isolation. BuildKit's authprovider session mechanism may
invalidate this — the buildx client reads Docker config and forwards credentials
to the BuildKit daemon via gRPC callback.

## Experiment Mechanics

Use gcloud builds submit with an inline cloudbuild.json on demo1025. Do NOT
modify the committed vessel configs — this is a throwaway test build.

Two variants to test:

**Variant A (with existing docker login):**
Steps 01-03 as normal (derive tag, get token, docker login to GAR), then:
- docker buildx create --driver docker-container --name rb-builder --use
- docker buildx build --push --platform=linux/amd64,linux/arm64,linux/arm/v7 --tag IMAGE_URI .

**Variant B (ADC only, no docker login):**
Skip steps 02-03 entirely. Rely on Cloud Build service account ADC:
- docker buildx create --driver docker-container --name rb-builder --use
- docker buildx build --push --platform=linux/amd64,linux/arm64,linux/arm/v7 --tag IMAGE_URI .

Testing both tells us whether the OCI Layout Bridge workaround AND the
docker-login step are both unnecessary.

## Downstream Impact

If --push works, the skopeo split step (07b) and SBOM step (08) currently read
from /workspace/oci-layout which won't exist. These steps would need to pull
from the registry instead. Not in scope for this experiment — just verify push.

## Verification

- Image appears in GAR with correct multi-platform manifest
- All 3 platforms present: gcloud artifacts docker images list ... --include-tags
- Build completes without 401/403 errors
- Note which variant(s) succeeded

## Key Risks

- 1-hour token expiry (docker/buildx#1205) — busybox is fast, note for larger vessels
- Docker 28.3.0+ DOCKER_AUTH_CONFIG regression (docker/cli#6156)
- DIND credential isolation edge cases (docker/buildx#3050)

## Acceptance Criteria

- Multi-platform image successfully pushed to GAR via buildx --push (either variant)
- OR: documented failure mode with error details for both variants

**[260304-2014] rough**

Test whether docker buildx build --push can push multi-platform images directly
to GAR from Cloud Build, eliminating the OCI Layout Bridge.

## Context

Research (Memos/memo-20260305-provenance-architecture-gap.md) found multiple
documented examples of buildx --push working in Cloud Build using ADC from
gcr.io/cloud-builders/docker. If confirmed, this eliminates crane (step 07)
and simplifies the pipeline. Prerequisite for CB-native SLSA provenance path.

## Experiment

On demo1025, manually submit a build that:
1. Uses existing steps 01-04 (tag, token, login, qemu)
2. Replaces steps 05+06 with: docker buildx build --push --platform=linux/amd64,linux/arm64,linux/arm/v7 --tag IMAGE_URI
3. Keeps steps 07b-09 (skopeo split, SBOM, metadata) reading from registry
   instead of /workspace — OR skip those and just verify push succeeded

## Verification

- Image appears in GAR with correct multi-platform manifest
- All 3 platforms present: `gcloud artifacts docker images list ... --include-tags`
- Build completes without 401/403 errors

## Key Risk

1-hour token expiry (docker/buildx#1205) — busybox build is fast so not a
concern, but note for larger vessels.

## Acceptance Criteria

- Multi-platform image successfully pushed to GAR via buildx --push
- OR: documented failure mode with error details

### test-pullback-images-verified (₢AlAAL) [complete]

**[260305-1209] complete**

Test whether pulling a buildx-pushed image back into Docker's local store enables
CB-native SLSA provenance via images: + requestedVerifyOption: VERIFIED.

## Prerequisites

- test-buildx-push-gar (₢AlAAK) succeeded

## Context

CB-native SLSA provenance requires the image in Docker's local store for the
images: field push. docker pull of a multi-platform manifest pulls only the host
platform (linux/amd64 on CB workers). If CB can push this single-platform image
via images: and generate provenance, we preserve Google's native SLSA trust model.

**VERIFIED is a hard gate:** When requestedVerifyOption: VERIFIED is set, "Builds
will only be marked successful if provenance is generated." If provenance cannot
be generated for the pulled-back image, the build fails entirely — not a silent
skip. This means failure is unambiguous: no need to check --show-provenance on a
failed build. The experiment gives a binary yes/no answer.

## CRITICAL: Dual-Tag Scheme Required

The images: field push MUST use a DIFFERENT tag from the --push multi-platform
image. If CB pushes a single-platform image under the same tag, it overwrites
the multi-platform manifest (destructive).

Proposed scheme:
- TAG-image: multi-platform manifest pushed by buildx --push (the real image)
- TAG-attested: single-platform image declared in images: for provenance

## Experiment

On demo1025, use gcloud builds submit with inline config:
1. Steps 01-04 as normal
2. buildx --push multi-platform image as TAG-image
3. docker pull TAG-image (pulls linux/amd64 into local daemon)
4. docker tag TAG-image TAG-attested (retag for images: field)
5. Declare TAG-attested in images: field
6. Set requestedVerifyOption: VERIFIED in options

## Key Questions

- Does docker pull succeed on the just-pushed multi-platform manifest?
- Does CB push TAG-attested without disturbing TAG-image?
- Does provenance attach to TAG-attested's digest?
- What slsa_build_level is reported?
- Is TAG-attested's digest the same as the linux/amd64 platform digest within
  the TAG-image manifest list? (Would let consumers cross-reference)

## Downstream Impact

If this works, the skopeo split step (07b) and SBOM step (08) need adjustment.
Currently they read from /workspace/oci-layout (crane output). With --push,
they'd need to pull from registry or use the docker-pull'd local image.
Not in scope for this experiment — address in stitch-provenance-fix (₢AlAAJ).

## Verification

```
gcloud artifacts docker images describe \
  us-central1-docker.pkg.dev/DEPOT/REPO/rbev-busybox:TAG-attested \
  --show-provenance --format=json
```

Also verify TAG-image still has all 3 platforms:
```
docker manifest inspect us-central1-docker.pkg.dev/DEPOT/REPO/rbev-busybox:TAG-image
```

## Acceptance Criteria

- slsa_build_level is NOT "unknown" on TAG-attested
- TAG-image multi-platform manifest survives intact (all 3 platforms)
- OR: documented failure mode explaining exactly where the chain breaks

**[260304-2028] rough**

Test whether pulling a buildx-pushed image back into Docker's local store enables
CB-native SLSA provenance via images: + requestedVerifyOption: VERIFIED.

## Prerequisites

- test-buildx-push-gar (₢AlAAK) succeeded

## Context

CB-native SLSA provenance requires the image in Docker's local store for the
images: field push. docker pull of a multi-platform manifest pulls only the host
platform (linux/amd64 on CB workers). If CB can push this single-platform image
via images: and generate provenance, we preserve Google's native SLSA trust model.

**VERIFIED is a hard gate:** When requestedVerifyOption: VERIFIED is set, "Builds
will only be marked successful if provenance is generated." If provenance cannot
be generated for the pulled-back image, the build fails entirely — not a silent
skip. This means failure is unambiguous: no need to check --show-provenance on a
failed build. The experiment gives a binary yes/no answer.

## CRITICAL: Dual-Tag Scheme Required

The images: field push MUST use a DIFFERENT tag from the --push multi-platform
image. If CB pushes a single-platform image under the same tag, it overwrites
the multi-platform manifest (destructive).

Proposed scheme:
- TAG-image: multi-platform manifest pushed by buildx --push (the real image)
- TAG-attested: single-platform image declared in images: for provenance

## Experiment

On demo1025, use gcloud builds submit with inline config:
1. Steps 01-04 as normal
2. buildx --push multi-platform image as TAG-image
3. docker pull TAG-image (pulls linux/amd64 into local daemon)
4. docker tag TAG-image TAG-attested (retag for images: field)
5. Declare TAG-attested in images: field
6. Set requestedVerifyOption: VERIFIED in options

## Key Questions

- Does docker pull succeed on the just-pushed multi-platform manifest?
- Does CB push TAG-attested without disturbing TAG-image?
- Does provenance attach to TAG-attested's digest?
- What slsa_build_level is reported?
- Is TAG-attested's digest the same as the linux/amd64 platform digest within
  the TAG-image manifest list? (Would let consumers cross-reference)

## Downstream Impact

If this works, the skopeo split step (07b) and SBOM step (08) need adjustment.
Currently they read from /workspace/oci-layout (crane output). With --push,
they'd need to pull from registry or use the docker-pull'd local image.
Not in scope for this experiment — address in stitch-provenance-fix (₢AlAAJ).

## Verification

```
gcloud artifacts docker images describe \
  us-central1-docker.pkg.dev/DEPOT/REPO/rbev-busybox:TAG-attested \
  --show-provenance --format=json
```

Also verify TAG-image still has all 3 platforms:
```
docker manifest inspect us-central1-docker.pkg.dev/DEPOT/REPO/rbev-busybox:TAG-image
```

## Acceptance Criteria

- slsa_build_level is NOT "unknown" on TAG-attested
- TAG-image multi-platform manifest survives intact (all 3 platforms)
- OR: documented failure mode explaining exactly where the chain breaks

**[260304-2017] rough**

Test whether pulling a buildx-pushed image back into Docker's local store enables
CB-native SLSA provenance via images: + requestedVerifyOption: VERIFIED.

## Prerequisites

- test-buildx-push-gar (₢AlAAK) succeeded

## Context

CB-native SLSA provenance requires the image in Docker's local store for the
images: field push. docker pull of a multi-platform manifest pulls only the host
platform (linux/amd64 on CB workers). If CB can push this single-platform image
via images: and generate provenance, we preserve Google's native SLSA trust model.

## CRITICAL: Dual-Tag Scheme Required

The images: field push MUST use a DIFFERENT tag from the --push multi-platform
image. If CB pushes a single-platform image under the same tag, it overwrites
the multi-platform manifest (destructive).

Proposed scheme:
- TAG-image: multi-platform manifest pushed by buildx --push (the real image)
- TAG-attested: single-platform image declared in images: for provenance

## Experiment

On demo1025, use gcloud builds submit with inline config:
1. Steps 01-04 as normal
2. buildx --push multi-platform image as TAG-image
3. docker pull TAG-image (pulls linux/amd64 into local daemon)
4. docker tag TAG-image TAG-attested (retag for images: field)
5. Declare TAG-attested in images: field
6. Set requestedVerifyOption: VERIFIED in options

## Key Questions

- Does docker pull succeed on the just-pushed multi-platform manifest?
- Does CB push TAG-attested without disturbing TAG-image?
- Does provenance attach to TAG-attested's digest?
- What slsa_build_level is reported?
- Is TAG-attested's digest the same as the linux/amd64 platform digest within
  the TAG-image manifest list? (Would let consumers cross-reference)

## Downstream Impact

If this works, the skopeo split step (07b) and SBOM step (08) need adjustment.
Currently they read from /workspace/oci-layout (crane output). With --push,
they'd need to pull from registry or use the docker-pull'd local image.
Not in scope for this experiment — address in stitch-provenance-fix (₢AlAAJ).

## Verification

```
gcloud artifacts docker images describe \
  us-central1-docker.pkg.dev/DEPOT/REPO/rbev-busybox:TAG-attested \
  --show-provenance --format=json
```

Also verify TAG-image still has all 3 platforms:
```
docker manifest inspect us-central1-docker.pkg.dev/DEPOT/REPO/rbev-busybox:TAG-image
```

## Acceptance Criteria

- slsa_build_level is NOT "unknown" on TAG-attested
- TAG-image multi-platform manifest survives intact (all 3 platforms)
- OR: documented failure mode explaining exactly where the chain breaks

**[260304-2014] rough**

Test whether pulling a buildx-pushed image back into Docker's local store enables
CB-native SLSA provenance via images: + requestedVerifyOption: VERIFIED.

## Prerequisites

- test-buildx-push-gar (₢AlAAK) succeeded

## Context

CB-native SLSA provenance requires the image in Docker's local store for the
images: field push. docker pull of a multi-platform manifest pulls only the host
platform (linux/amd64 on CB workers). If CB can push this single-platform image
via images: and generate provenance, we may use a dual-tag scheme.

## Experiment

On demo1025, manually submit a build that:
1. Steps 01-04 as normal
2. buildx --push multi-platform image as TAG-image (per ₢AlAAK)
3. docker pull IMAGE_URI (pulls linux/amd64 into local daemon)
4. Declare pulled image in images: field
5. Set requestedVerifyOption: VERIFIED in options

## Key Questions

- Does docker pull succeed on the just-pushed multi-platform manifest?
- Does CB's images: push overwrite the multi-platform manifest? (destructive?)
- Does provenance attach to the image digest?
- What slsa_build_level is reported?

## Verification

```
gcloud artifacts docker images describe IMAGE --show-provenance --format=json
```

## Acceptance Criteria

- slsa_build_level is NOT "unknown"
- Multi-platform manifest (from --push) survives alongside provenance
- OR: documented failure mode explaining exactly where the chain breaks

### stitch-single-arch-slsa (₢AlAAJ) [complete]

**[260305-1320] complete**

Restructure zrbf_stitch_build_json and build step scripts for single-architecture
vessels with SLSA v1.0 provenance. Code changes only — no vessel modifications,
no infrastructure operations, no spec updates.

## Strategy

Single-architecture vessels first. This pace handles the pipeline code that
makes it work. Vessel bifurcation, infrastructure, specs, and verification
are separate paces.

## What Experiments Proved (₢AlAAK, ₢AlAAL)

- `docker buildx build --push` works in CB (ADC-only, no docker login needed)
- Pull-back + `images:` + `VERIFIED` produces SLSA Build Level 3
- Cloud Build pre-populates Docker credentials for all GAR regions
- See `Memos/memo-20260305-provenance-architecture-gap.md` for full evidence

## Key Insight: --load for single-arch

For single-architecture vessels, `docker buildx build --load` puts the image
directly into Docker's local daemon. No --push + pull-back round trip needed.
The `images:` field + `VERIFIED` then generates provenance natively.

For cross-arch (arm64 vessel on amd64 worker), `--load` with
`--platform=linux/arm64` should work with QEMU. Verify during e2e testing.

**Fallback if --load fails for cross-arch:** Use the proven `--push` +
`docker pull` pullback pattern (Experiment 3, build 48b818ed). The stitch
function should be structured so that swapping --load for --push+pullback
is a localized change in the build-and-load step script, not a stitch
function rewrite.

## Scope

### Step script changes (Tools/rbw/rbgjb/)

1. **Delete rbgjb02-get-docker-token.sh** — CB pre-populates credentials
2. **Delete rbgjb03-docker-login-gar.sh** — CB pre-populates credentials
3. **Replace rbgjb06-build-and-export.sh** with build-and-load variant —
   `docker buildx build --load` instead of `--output type=oci,dest=...`
   Single platform only (from RBRV_CONJURE_PLATFORMS substitution).
4. **Delete rbgjb07-push-with-crane.sh** — OCI Layout Bridge eliminated
5. **Delete rbgjb07b-split-oci-platform.sh** — no local OCI archive exists;
   skopeo split was only needed for multi-platform SBOM extraction.
6. **Rework rbgjb08-sbom-and-summary.sh** — change Syft source from
   `oci-dir:/workspace/oci-amd64` to `docker:IMAGE_URI`. The Syft step
   runs inside the docker builder image (`RBRR_GCB_DOCKER_IMAGE_REF`),
   which has Docker socket access — Syft can read directly from the local
   daemon via the shared `/var/run/docker.sock`. No volume mount changes
   needed; CB shares the daemon across all steps using this builder image.
7. **Rework rbgjb10-assemble-metadata.sh** — derive `.image_uri` from
   substitutions instead of reading file written by crane:
   `${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}`
8. **Renumber step scripts** — filenames and stitch step_defs array.
   Leave exact naming to implementation.

### Stitch function changes (Tools/rbw/rbf_Foundry.sh)

1. **Update step_defs array** — remove deleted steps, add new ones, renumber
2. **Add `images:` field** to stitched cloudbuild.json output
3. **Add `requestedVerifyOption: VERIFIED`** to options block
4. **Single-arch validation gate** — stitch must reject vessels where
   `RBRV_CONJURE_PLATFORMS` contains more than one platform, with clear
   error message pointing to multi-platform provenance as future work
5. **Remove crane substitution baking** — lines ~201-203 in stitch function
   bake `RBRR_GCB_SYFT_IMAGE_REF`, `RBRR_GCB_BINFMT_IMAGE_REF`, and
   `RBRR_GCB_SKOPEO_IMAGE_REF` into step text. Remove skopeo baking
   (step 07b deleted). Syft and binfmt baking remain.
6. **Remove `_RBGY_CRANE_TAR_GZ` substitution** — crane step deleted, this
   substitution is no longer consumed by any step. Remove from the jq
   composition block (~line 251 and ~line 272).

### Regime variable disposition

| Variable | Status | Reason |
|---|---|---|
| `RBRR_CRANE_TAR_GZ` | Keep in rbrr.env, remove from stitch | Crane step deleted |
| `RBRR_GCB_SKOPEO_IMAGE_REF` | Keep in rbrr.env, remove from stitch | Skopeo step deleted for single-arch |
| `RBRR_GCB_SYFT_IMAGE_REF` | Keep, still baked | Syft step survives |
| `RBRR_GCB_BINFMT_IMAGE_REF` | Keep, still baked | Binfmt step survives |
| `RBRR_GCB_ALPINE_IMAGE_REF` | Keep in step_defs | Metadata step (10) uses alpine |

Regime vars stay in rbrr.env even when unused — they may be needed for
future multi-platform path. Stitch just stops referencing them.

### NOT in scope

- Vessel directory changes (₢AlAAN)
- Depot operations (₢AlAAO)
- Spec updates to RBS0 or RBSOB (₢AlAAP)
- Inscribe, dispatch, or verification (₢AlAAO)

## Key Files

- `Tools/rbw/rbf_Foundry.sh` — `zrbf_stitch_build_json` (~lines 108-289)
- `Tools/rbw/rbgjb/*.sh` — build step scripts
- `Memos/memo-20260305-provenance-architecture-gap.md` — experiment evidence

## Acceptance Criteria

- Stitch produces cloudbuild.json with `images:` and `VERIFIED` for single-arch
- Stitch rejects multi-platform vessels with actionable error
- Deleted steps (02, 03, 07, 07b) no longer in step_defs
- Step 06 replacement uses `--load` instead of OCI export
- Syft step uses `docker:IMAGE_URI` transport (not `oci-dir:`)
- Metadata assembly derives image URI from substitutions
- `_RBGY_CRANE_TAR_GZ` substitution removed from JSON composition
- Skopeo baking removed from substitution baking block
- Code compiles/runs (stitch can be exercised locally if possible)

**[260305-1247] rough**

Restructure zrbf_stitch_build_json and build step scripts for single-architecture
vessels with SLSA v1.0 provenance. Code changes only — no vessel modifications,
no infrastructure operations, no spec updates.

## Strategy

Single-architecture vessels first. This pace handles the pipeline code that
makes it work. Vessel bifurcation, infrastructure, specs, and verification
are separate paces.

## What Experiments Proved (₢AlAAK, ₢AlAAL)

- `docker buildx build --push` works in CB (ADC-only, no docker login needed)
- Pull-back + `images:` + `VERIFIED` produces SLSA Build Level 3
- Cloud Build pre-populates Docker credentials for all GAR regions
- See `Memos/memo-20260305-provenance-architecture-gap.md` for full evidence

## Key Insight: --load for single-arch

For single-architecture vessels, `docker buildx build --load` puts the image
directly into Docker's local daemon. No --push + pull-back round trip needed.
The `images:` field + `VERIFIED` then generates provenance natively.

For cross-arch (arm64 vessel on amd64 worker), `--load` with
`--platform=linux/arm64` should work with QEMU. Verify during e2e testing.

**Fallback if --load fails for cross-arch:** Use the proven `--push` +
`docker pull` pullback pattern (Experiment 3, build 48b818ed). The stitch
function should be structured so that swapping --load for --push+pullback
is a localized change in the build-and-load step script, not a stitch
function rewrite.

## Scope

### Step script changes (Tools/rbw/rbgjb/)

1. **Delete rbgjb02-get-docker-token.sh** — CB pre-populates credentials
2. **Delete rbgjb03-docker-login-gar.sh** — CB pre-populates credentials
3. **Replace rbgjb06-build-and-export.sh** with build-and-load variant —
   `docker buildx build --load` instead of `--output type=oci,dest=...`
   Single platform only (from RBRV_CONJURE_PLATFORMS substitution).
4. **Delete rbgjb07-push-with-crane.sh** — OCI Layout Bridge eliminated
5. **Delete rbgjb07b-split-oci-platform.sh** — no local OCI archive exists;
   skopeo split was only needed for multi-platform SBOM extraction.
6. **Rework rbgjb08-sbom-and-summary.sh** — change Syft source from
   `oci-dir:/workspace/oci-amd64` to `docker:IMAGE_URI`. The Syft step
   runs inside the docker builder image (`RBRR_GCB_DOCKER_IMAGE_REF`),
   which has Docker socket access — Syft can read directly from the local
   daemon via the shared `/var/run/docker.sock`. No volume mount changes
   needed; CB shares the daemon across all steps using this builder image.
7. **Rework rbgjb10-assemble-metadata.sh** — derive `.image_uri` from
   substitutions instead of reading file written by crane:
   `${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}`
8. **Renumber step scripts** — filenames and stitch step_defs array.
   Leave exact naming to implementation.

### Stitch function changes (Tools/rbw/rbf_Foundry.sh)

1. **Update step_defs array** — remove deleted steps, add new ones, renumber
2. **Add `images:` field** to stitched cloudbuild.json output
3. **Add `requestedVerifyOption: VERIFIED`** to options block
4. **Single-arch validation gate** — stitch must reject vessels where
   `RBRV_CONJURE_PLATFORMS` contains more than one platform, with clear
   error message pointing to multi-platform provenance as future work
5. **Remove crane substitution baking** — lines ~201-203 in stitch function
   bake `RBRR_GCB_SYFT_IMAGE_REF`, `RBRR_GCB_BINFMT_IMAGE_REF`, and
   `RBRR_GCB_SKOPEO_IMAGE_REF` into step text. Remove skopeo baking
   (step 07b deleted). Syft and binfmt baking remain.
6. **Remove `_RBGY_CRANE_TAR_GZ` substitution** — crane step deleted, this
   substitution is no longer consumed by any step. Remove from the jq
   composition block (~line 251 and ~line 272).

### Regime variable disposition

| Variable | Status | Reason |
|---|---|---|
| `RBRR_CRANE_TAR_GZ` | Keep in rbrr.env, remove from stitch | Crane step deleted |
| `RBRR_GCB_SKOPEO_IMAGE_REF` | Keep in rbrr.env, remove from stitch | Skopeo step deleted for single-arch |
| `RBRR_GCB_SYFT_IMAGE_REF` | Keep, still baked | Syft step survives |
| `RBRR_GCB_BINFMT_IMAGE_REF` | Keep, still baked | Binfmt step survives |
| `RBRR_GCB_ALPINE_IMAGE_REF` | Keep in step_defs | Metadata step (10) uses alpine |

Regime vars stay in rbrr.env even when unused — they may be needed for
future multi-platform path. Stitch just stops referencing them.

### NOT in scope

- Vessel directory changes (₢AlAAN)
- Depot operations (₢AlAAO)
- Spec updates to RBS0 or RBSOB (₢AlAAP)
- Inscribe, dispatch, or verification (₢AlAAO)

## Key Files

- `Tools/rbw/rbf_Foundry.sh` — `zrbf_stitch_build_json` (~lines 108-289)
- `Tools/rbw/rbgjb/*.sh` — build step scripts
- `Memos/memo-20260305-provenance-architecture-gap.md` — experiment evidence

## Acceptance Criteria

- Stitch produces cloudbuild.json with `images:` and `VERIFIED` for single-arch
- Stitch rejects multi-platform vessels with actionable error
- Deleted steps (02, 03, 07, 07b) no longer in step_defs
- Step 06 replacement uses `--load` instead of OCI export
- Syft step uses `docker:IMAGE_URI` transport (not `oci-dir:`)
- Metadata assembly derives image URI from substitutions
- `_RBGY_CRANE_TAR_GZ` substitution removed from JSON composition
- Skopeo baking removed from substitution baking block
- Code compiles/runs (stitch can be exercised locally if possible)

**[260305-1226] rough**

Restructure zrbf_stitch_build_json and build step scripts for single-architecture
vessels with SLSA v1.0 provenance. Code changes only — no vessel modifications,
no infrastructure operations, no spec updates.

## Strategy

Single-architecture vessels first. This pace handles the pipeline code that
makes it work. Vessel bifurcation, infrastructure, specs, and verification
are separate paces.

## What Experiments Proved (₢AlAAK, ₢AlAAL)

- `docker buildx build --push` works in CB (ADC-only, no docker login needed)
- Pull-back + `images:` + `VERIFIED` produces SLSA Build Level 3
- Cloud Build pre-populates Docker credentials for all GAR regions
- See `Memos/memo-20260305-provenance-architecture-gap.md` for full evidence

## Key Insight: --load for single-arch

For single-architecture vessels, `docker buildx build --load` puts the image
directly into Docker's local daemon. No --push + pull-back round trip needed.
The `images:` field + `VERIFIED` then generates provenance natively.

For cross-arch (arm64 vessel on amd64 worker), `--load` with
`--platform=linux/arm64` should work with QEMU. Verify during e2e testing.

**Fallback if --load fails for cross-arch:** Use the proven `--push` +
`docker pull` pullback pattern (Experiment 3, build 48b818ed). The stitch
function should be structured so that swapping --load for --push+pullback
is a localized change in the build-and-load step script, not a stitch
function rewrite.

## Scope

### Step script changes (Tools/rbw/rbgjb/)

1. **Delete rbgjb02-get-docker-token.sh** — CB pre-populates credentials
2. **Delete rbgjb03-docker-login-gar.sh** — CB pre-populates credentials
3. **Replace rbgjb06-build-and-export.sh** with build-and-load variant —
   `docker buildx build --load` instead of `--output type=oci,dest=...`
   Single platform only (from RBRV_CONJURE_PLATFORMS substitution).
4. **Delete rbgjb07-push-with-crane.sh** — OCI Layout Bridge eliminated
5. **Rework rbgjb07b-split-oci-platform.sh** — no local OCI archive exists;
   either eliminate (Syft can scan local daemon image) or replace with
   registry pull for Syft input. Determine correct Syft transport.
6. **Rework rbgjb08-sbom-and-summary.sh** — adjust Syft source from
   `oci-dir:/workspace/oci-amd64` to `docker:IMAGE_URI` or equivalent.
7. **Rework rbgjb10-assemble-metadata.sh** — derives `.image_uri` from
   substitutions instead of reading file written by crane.
8. **Renumber step scripts** — filenames and stitch step_defs array.

### Stitch function changes (Tools/rbw/rbf_Foundry.sh)

1. **Update step_defs array** — remove deleted steps, add new ones, renumber
2. **Add `images:` field** to stitched cloudbuild.json output
3. **Add `requestedVerifyOption: VERIFIED`** to options block
4. **Single-arch validation gate** — stitch must reject vessels where
   `RBRV_CONJURE_PLATFORMS` contains more than one platform, with clear
   error message pointing to multi-platform provenance as future work
5. **Remove crane-related substitution baking** — `RBRR_CRANE_TAR_GZ` no
   longer needed in step text; `RBRR_GCB_SKOPEO_IMAGE_REF` status TBD

### NOT in scope

- Vessel directory changes (₢AlAAN)
- Depot operations (₢AlAAO)
- Spec updates to RBS0 or RBSOB (₢AlAAP)
- Inscribe, dispatch, or verification (₢AlAAO)

## Key Files

- `Tools/rbw/rbf_Foundry.sh` — `zrbf_stitch_build_json`
- `Tools/rbw/rbgjb/*.sh` — build step scripts
- `Memos/memo-20260305-provenance-architecture-gap.md` — experiment evidence

## Acceptance Criteria

- Stitch produces cloudbuild.json with `images:` and `VERIFIED` for single-arch
- Stitch rejects multi-platform vessels with actionable error
- Deleted steps (02, 03, 07) no longer in step_defs
- Step 06 replacement uses `--load` instead of OCI export
- SBOM/Syft step adjusted for local daemon image source
- Metadata assembly derives image URI from substitutions
- Code compiles/runs (stitch can be exercised locally if possible)

**[260305-1216] rough**

Restructure zrbf_stitch_build_json and build step scripts for single-architecture
vessels with SLSA v1.0 provenance. Code changes only — no vessel modifications,
no infrastructure operations, no spec updates.

## Strategy

Single-architecture vessels first. This pace handles the pipeline code that
makes it work. Vessel bifurcation, infrastructure, specs, and verification
are separate paces.

## What Experiments Proved (₢AlAAK, ₢AlAAL)

- `docker buildx build --push` works in CB (ADC-only, no docker login needed)
- Pull-back + `images:` + `VERIFIED` produces SLSA Build Level 3
- Cloud Build pre-populates Docker credentials for all GAR regions
- See `Memos/memo-20260305-provenance-architecture-gap.md` for full evidence

## Key Insight: --load for single-arch

For single-architecture vessels, `docker buildx build --load` puts the image
directly into Docker's local daemon. No --push + pull-back round trip needed.
The `images:` field + `VERIFIED` then generates provenance natively.

For cross-arch (arm64 vessel on amd64 worker), `--load` with
`--platform=linux/arm64` should work with QEMU. Verify during e2e testing.

## Scope

### Step script changes (Tools/rbw/rbgjb/)

1. **Delete rbgjb02-get-docker-token.sh** — CB pre-populates credentials
2. **Delete rbgjb03-docker-login-gar.sh** — CB pre-populates credentials
3. **Replace rbgjb06-build-and-export.sh** with build-and-load variant —
   `docker buildx build --load` instead of `--output type=oci,dest=...`
   Single platform only (from RBRV_CONJURE_PLATFORMS substitution).
4. **Delete rbgjb07-push-with-crane.sh** — OCI Layout Bridge eliminated
5. **Rework rbgjb07b-split-oci-platform.sh** — no local OCI archive exists;
   either eliminate (Syft can scan local daemon image) or replace with
   registry pull for Syft input. Determine correct Syft transport.
6. **Rework rbgjb08-sbom-and-summary.sh** — adjust Syft source from
   `oci-dir:/workspace/oci-amd64` to `docker:IMAGE_URI` or equivalent.
7. **Rework rbgjb10-assemble-metadata.sh** — derives `.image_uri` from
   substitutions instead of reading file written by crane.
8. **Renumber step scripts** — filenames and stitch step_defs array.

### Stitch function changes (Tools/rbw/rbf_Foundry.sh)

1. **Update step_defs array** — remove deleted steps, add new ones, renumber
2. **Add `images:` field** to stitched cloudbuild.json output
3. **Add `requestedVerifyOption: VERIFIED`** to options block
4. **Single-arch validation gate** — stitch must reject vessels where
   `RBRV_CONJURE_PLATFORMS` contains more than one platform, with clear
   error message pointing to multi-platform provenance as future work
5. **Remove crane-related substitution baking** — `RBRR_CRANE_TAR_GZ` no
   longer needed in step text; `RBRR_GCB_SKOPEO_IMAGE_REF` status TBD

### NOT in scope

- Vessel directory changes (separate pace)
- Depot operations (separate pace)
- Spec updates to RBS0 or RBSOB (separate pace)
- Inscribe, dispatch, or verification (separate pace)

## Key Files

- `Tools/rbw/rbf_Foundry.sh` — `zrbf_stitch_build_json`
- `Tools/rbw/rbgjb/*.sh` — build step scripts
- `Memos/memo-20260305-provenance-architecture-gap.md` — experiment evidence

## Acceptance Criteria

- Stitch produces cloudbuild.json with `images:` and `VERIFIED` for single-arch
- Stitch rejects multi-platform vessels with actionable error
- Deleted steps (02, 03, 07) no longer in step_defs
- Step 06 replacement uses `--load` instead of OCI export
- SBOM/Syft step adjusted for local daemon image source
- Metadata assembly derives image URI from substitutions
- Code compiles/runs (stitch can be exercised locally if possible)

**[260305-1212] rough**

Restructure zrbf_stitch_build_json for single-architecture vessels with full
SLSA v1.0 provenance. Update RBS0 specification to reflect validated architecture.

## Strategy

Commit to single-architecture vessel support first. Build a strong baseline of
statistically many SLSA-certified images before tackling multi-platform. This pace
does NOT attempt multi-platform provenance — that is a separate future initiative.

## What Experiments Proved (₢AlAAK, ₢AlAAL)

- `docker buildx build --push` works in CB (ADC-only, no docker login needed)
- Pull-back + `images:` + `VERIFIED` produces SLSA Build Level 3
- Cloud Build pre-populates Docker credentials for all GAR regions
- See `Memos/memo-20260305-provenance-architecture-gap.md` for full evidence

## Key Insight: --load eliminates pullback for single-arch

For single-architecture vessels, `docker buildx build --load` puts the image
directly into Docker's local daemon. No need for --push + pull-back. The
`images:` field + `VERIFIED` then generates provenance natively.

For cross-arch (e.g., arm64 vessel on amd64 CB worker), `--load` with
`--platform=linux/arm64` should work with QEMU registered. This needs
verification during implementation.

## Scope

### Pipeline changes (zrbf_stitch_build_json + step scripts)

1. **Remove steps 02 (get-docker-token) and 03 (docker-login-gar)** — CB
   pre-populates credentials; ADC sufficient (Experiment 2, build 4de1467a)
2. **Replace step 06 (build-and-export) with build-and-load** — use
   `docker buildx build --load` instead of `--output type=oci,dest=...`
   For single-platform, --load puts image in local daemon directly.
3. **Remove step 07 (push-with-crane)** — OCI Layout Bridge eliminated
4. **Skopeo split (step 07b)** — no longer needed for single-arch; Syft can
   scan the local daemon image directly via `docker:` transport
5. **Add `images:` field** to stitched cloudbuild.json with the image URI
6. **Add `requestedVerifyOption: VERIFIED`** to options block
7. **Derive `.image_uri`** from substitutions (crane no longer writes it)
8. **Step renumbering** — update filenames and step_defs array

### Vessel configuration

- `RBRV_CONJURE_PLATFORMS` must be exactly one platform for this path
  (e.g., `linux/amd64` or `linux/arm64`)
- Add validation: stitch rejects multi-platform vessels with clear error
  until multi-platform provenance support is implemented
- Busybox vessel: temporarily set to single-platform for validation

### Specification updates

- **RBS0-SpecTop.adoc**: Update `rbtgr_provenance` definition — remove
  "architectural constraint" language, replace with confirmed working pattern.
  Reference experiment results. Note single-arch limitation.
- **RBSOB-oci_layout_bridge.adoc**: Mark OCI Layout Bridge as superseded for
  single-arch vessels. Note it may be needed for future multi-platform path.

### Validation: two architectures

Prove single-arch SLSA works for both platform families:

**Test A (amd64):**
- Set busybox to `RBRV_CONJURE_PLATFORMS="linux/amd64"`
- Inscribe → dispatch → verify `slsa_build_level: 3`

**Test B (arm64):**
- Set busybox to `RBRV_CONJURE_PLATFORMS="linux/arm64"`
- Inscribe → dispatch → verify `slsa_build_level: 3`
- This proves cross-architecture SLSA works (arm64 built on amd64 worker via QEMU)

## Operational Sequence

1. Modify step scripts and stitch function
2. Set busybox to single-platform (amd64)
3. Run inscribe: `tt/rbw-DI.DirectorInscribesRubric.sh`
4. Commit regenerated cloudbuild.json files
5. Re-run inscribe (pushes to rubric repo)
6. Dispatch busybox, verify SLSA provenance (Test A)
7. Switch busybox to arm64, repeat inscribe+dispatch+verify (Test B)
8. Switch busybox back to amd64 (or leave as appropriate)
9. Update RBS0 and RBSOB specs

## Key Files

- `Tools/rbw/rbf_Foundry.sh` — `zrbf_stitch_build_json`
- `Tools/rbw/rbgjb/*.sh` — build step scripts
- `rbev-vessels/rbev-busybox/rbrv.env` — vessel platform config
- `lenses/RBS0-SpecTop.adoc` — provenance definition (lines 1455-1472)
- `lenses/RBSOB-oci_layout_bridge.adoc` — OCI Layout Bridge spec
- `Memos/memo-20260305-provenance-architecture-gap.md` — experiment evidence

## Acceptance Criteria

- `slsa_build_level: 3` on trigger-dispatched amd64 busybox image
- `slsa_build_level: 3` on trigger-dispatched arm64 busybox image
- Stitch rejects multi-platform vessels with actionable error message
- RBS0 provenance definition updated with confirmed facts
- RBSOB updated to reflect OCI Layout Bridge superseded status
- Steps 02, 03, 07 eliminated from pipeline
- All vessel cloudbuild.json files regenerated

**[260305-1212] rough**

Restructure zrbf_stitch_build_json for single-architecture vessels with full
SLSA v1.0 provenance. Update RBS0 specification to reflect validated architecture.

## Strategy

Commit to single-architecture vessel support first. Build a strong baseline of
statistically many SLSA-certified images before tackling multi-platform. This pace
does NOT attempt multi-platform provenance — that is a separate future initiative.

## What Experiments Proved (₢AlAAK, ₢AlAAL)

- `docker buildx build --push` works in CB (ADC-only, no docker login needed)
- Pull-back + `images:` + `VERIFIED` produces SLSA Build Level 3
- Cloud Build pre-populates Docker credentials for all GAR regions
- See `Memos/memo-20260305-provenance-architecture-gap.md` for full evidence

## Key Insight: --load eliminates pullback for single-arch

For single-architecture vessels, `docker buildx build --load` puts the image
directly into Docker's local daemon. No need for --push + pull-back. The
`images:` field + `VERIFIED` then generates provenance natively.

For cross-arch (e.g., arm64 vessel on amd64 CB worker), `--load` with
`--platform=linux/arm64` should work with QEMU registered. This needs
verification during implementation.

## Scope

### Pipeline changes (zrbf_stitch_build_json + step scripts)

1. **Remove steps 02 (get-docker-token) and 03 (docker-login-gar)** — CB
   pre-populates credentials; ADC sufficient (Experiment 2, build 4de1467a)
2. **Replace step 06 (build-and-export) with build-and-load** — use
   `docker buildx build --load` instead of `--output type=oci,dest=...`
   For single-platform, --load puts image in local daemon directly.
3. **Remove step 07 (push-with-crane)** — OCI Layout Bridge eliminated
4. **Skopeo split (step 07b)** — no longer needed for single-arch; Syft can
   scan the local daemon image directly via `docker:` transport
5. **Add `images:` field** to stitched cloudbuild.json with the image URI
6. **Add `requestedVerifyOption: VERIFIED`** to options block
7. **Derive `.image_uri`** from substitutions (crane no longer writes it)
8. **Step renumbering** — update filenames and step_defs array

### Vessel configuration

- `RBRV_CONJURE_PLATFORMS` must be exactly one platform for this path
  (e.g., `linux/amd64` or `linux/arm64`)
- Add validation: stitch rejects multi-platform vessels with clear error
  until multi-platform provenance support is implemented
- Busybox vessel: temporarily set to single-platform for validation

### Specification updates

- **RBS0-SpecTop.adoc**: Update `rbtgr_provenance` definition — remove
  "architectural constraint" language, replace with confirmed working pattern.
  Reference experiment results. Note single-arch limitation.
- **RBSOB-oci_layout_bridge.adoc**: Mark OCI Layout Bridge as superseded for
  single-arch vessels. Note it may be needed for future multi-platform path.

### Validation: two architectures

Prove single-arch SLSA works for both platform families:

**Test A (amd64):**
- Set busybox to `RBRV_CONJURE_PLATFORMS="linux/amd64"`
- Inscribe → dispatch → verify `slsa_build_level: 3`

**Test B (arm64):**
- Set busybox to `RBRV_CONJURE_PLATFORMS="linux/arm64"`
- Inscribe → dispatch → verify `slsa_build_level: 3`
- This proves cross-architecture SLSA works (arm64 built on amd64 worker via QEMU)

## Operational Sequence

1. Modify step scripts and stitch function
2. Set busybox to single-platform (amd64)
3. Run inscribe: `tt/rbw-DI.DirectorInscribesRubric.sh`
4. Commit regenerated cloudbuild.json files
5. Re-run inscribe (pushes to rubric repo)
6. Dispatch busybox, verify SLSA provenance (Test A)
7. Switch busybox to arm64, repeat inscribe+dispatch+verify (Test B)
8. Switch busybox back to amd64 (or leave as appropriate)
9. Update RBS0 and RBSOB specs

## Key Files

- `Tools/rbw/rbf_Foundry.sh` — `zrbf_stitch_build_json`
- `Tools/rbw/rbgjb/*.sh` — build step scripts
- `rbev-vessels/rbev-busybox/rbrv.env` — vessel platform config
- `lenses/RBS0-SpecTop.adoc` — provenance definition (lines 1455-1472)
- `lenses/RBSOB-oci_layout_bridge.adoc` — OCI Layout Bridge spec
- `Memos/memo-20260305-provenance-architecture-gap.md` — experiment evidence

## Acceptance Criteria

- `slsa_build_level: 3` on trigger-dispatched amd64 busybox image
- `slsa_build_level: 3` on trigger-dispatched arm64 busybox image
- Stitch rejects multi-platform vessels with actionable error message
- RBS0 provenance definition updated with confirmed facts
- RBSOB updated to reflect OCI Layout Bridge superseded status
- Steps 02, 03, 07 eliminated from pipeline
- All vessel cloudbuild.json files regenerated

**[260304-2028] rough**

Apply validated push+provenance approach to zrbf_stitch_build_json.

## Prerequisites

- ₢AlAAI research complete (see Memos/memo-20260305-provenance-architecture-gap.md)
- Experiment paces complete with confirmed results:
  - test-buildx-push-gar: Can buildx --push replace OCI Layout Bridge?
  - test-pullback-images-verified: Can pull-back + images: + VERIFIED produce CB-native SLSA?

## What We Know (from ₢AlAAI research)

- Adding `requestedVerifyOption: VERIFIED` to current pipeline BREAKS builds
  (provenance required but can't be generated for step-pushed images)
- Adding `images:` field BREAKS builds (image not in Docker local store)
- CB-native SLSA requires CB to push via `images:` from local Docker daemon
- Multi-platform manifests can't exist in Docker local store
- OCI Layout Bridge (crane push) bypasses CB's native push entirely

## Scope (depends on experiment results)

**If buildx --push + pull-back + images: works (preferred):**
- Replace OCI Layout Bridge steps (06 build-export + 07 crane-push) with
  single buildx --push step
- Add pull-back step: `docker pull IMAGE_URI` (gets host-platform image into daemon)
- Add docker tag step: retag as TAG-attested for images: field (dual-tag scheme)
- Add `images:` field with TAG-attested URI
- Add `requestedVerifyOption: VERIFIED` to options
- Update RBSOB spec (OCI Layout Bridge retired or narrowed)

### Downstream step impacts (Path 1 specifics):

1. **`.image_uri` file disappears** — Step 09 (assemble-metadata) reads
   `.image_uri` written by crane's push step. With buildx --push, nothing
   writes this file. Derive the URI from substitutions instead:
   `${_RBGY_GAR_LOCATION}-docker.pkg.dev/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}-image`

2. **Skopeo split source changes** — Step 07b currently reads from
   `/workspace/oci-layout` (crane's extracted OCI directory). With --push,
   no local OCI archive exists. Skopeo must pull from registry instead:
   `skopeo copy docker://IMAGE_URI oci:/workspace/oci-amd64`

3. **Syft scan source unchanged** — Step 08 reads `oci-dir:/workspace/oci-amd64`.
   If skopeo pulls from registry to the same path (above), Syft works as-is.
   Confirm during implementation.

4. **Crane step eliminated, alpine builder survives** — Step 07 (crane push)
   and its `RBRR_GCB_ALPINE_IMAGE_REF` pin are eliminated as a push mechanism.
   However, step 10 (assemble-metadata) also uses the alpine builder, so the
   pin survives.

5. **Step renumbering** — With crane step removed and pull-back + retag added,
   the step sequence changes. Update step script filenames and stitch step_defs
   array accordingly.

**If only buildx --push works (no CB-native provenance):**
- Replace OCI Layout Bridge with --push (pipeline simplification)
- Same downstream impacts 1-5 above apply
- Document provenance gap in RBSCB
- Evaluate cosign as separate future pace

**If buildx --push fails:**
- Keep OCI Layout Bridge as-is
- Document that CB-native SLSA is impossible with current architecture
- Evaluate cosign as separate future pace

## Key Files

- `Tools/rbw/rbf_Foundry.sh` — `zrbf_stitch_build_json`
- `Tools/rbw/rbgjb/*.sh` — build step scripts
- `rbev-vessels/*/cloudbuild.json` — generated configs (7 vessels)
- `lenses/RBSOB-oci_layout_bridge.adoc` — OCI Layout Bridge spec
- `Memos/memo-20260305-provenance-architecture-gap.md` — research findings

## Verification

```
gcloud artifacts docker images describe \
  us-central1-docker.pkg.dev/DEPOT_PROJECT/REPO/rbev-busybox:TAG-attested \
  --show-provenance --format=json
```

Also verify multi-platform manifest intact:
```
docker manifest inspect \
  us-central1-docker.pkg.dev/DEPOT_PROJECT/REPO/rbev-busybox:TAG-image
```

## Acceptance Criteria

- slsa_build_level is not "unknown" on TAG-attested (if CB-native path works)
- TAG-image multi-platform manifest survives intact (all 3 platforms)
- OR: documented architectural decision with clear fallback plan
- All 7 vessel cloudbuild.json regenerated and committed
- Busybox build+push+provenance verified on demo1025

**[260304-2014] rough**

Apply validated push+provenance approach to zrbf_stitch_build_json.

## Prerequisites

- ₢AlAAI research complete (see Memos/memo-20260305-provenance-architecture-gap.md)
- Experiment paces complete with confirmed results:
  - test-buildx-push-gar: Can buildx --push replace OCI Layout Bridge?
  - test-pullback-images-verified: Can pull-back + images: + VERIFIED produce CB-native SLSA?

## What We Know (from ₢AlAAI research)

- Adding `requestedVerifyOption: VERIFIED` to current pipeline BREAKS builds
  (provenance required but can't be generated for step-pushed images)
- Adding `images:` field BREAKS builds (image not in Docker local store)
- CB-native SLSA requires CB to push via `images:` from local Docker daemon
- Multi-platform manifests can't exist in Docker local store
- OCI Layout Bridge (crane push) bypasses CB's native push entirely

## Scope (depends on experiment results)

**If buildx --push + pull-back + images: works (preferred):**
- Replace OCI Layout Bridge steps (06 build-export + 07 crane-push) with
  single buildx --push step
- Add pull-back step: `docker pull IMAGE_URI` (gets host-platform image into daemon)
- Add `images:` field with pulled-back image URI
- Add `requestedVerifyOption: VERIFIED` to options
- Update RBSOB spec (OCI Layout Bridge retired or narrowed)
- Adjust skopeo split step (07b) — may read from registry instead of /workspace

**If only buildx --push works (no CB-native provenance):**
- Replace OCI Layout Bridge with --push (pipeline simplification)
- Document provenance gap in RBSCB
- Evaluate cosign as separate future pace

**If buildx --push fails:**
- Keep OCI Layout Bridge as-is
- Document that CB-native SLSA is impossible with current architecture
- Evaluate cosign as separate future pace

## Key Files

- `Tools/rbw/rbf_Foundry.sh` — `zrbf_stitch_build_json`
- `Tools/rbw/rbgjb/*.sh` — build step scripts
- `rbev-vessels/*/cloudbuild.json` — generated configs (7 vessels)
- `lenses/RBSOB-oci_layout_bridge.adoc` — OCI Layout Bridge spec
- `Memos/memo-20260305-provenance-architecture-gap.md` — research findings

## Verification

```
gcloud artifacts docker images describe \
  us-central1-docker.pkg.dev/DEPOT_PROJECT/REPO/rbev-busybox:TAG \
  --show-provenance --format=json
```

## Acceptance Criteria

- slsa_build_level is not "unknown" (if CB-native path works)
- OR: documented architectural decision with clear fallback plan
- All 7 vessel cloudbuild.json regenerated and committed
- Busybox build+push+provenance verified on demo1025

**[260304-1942] rough**

Apply provenance fix to zrbf_stitch_build_json based on research findings from ₢AlAAI.

## Prerequisites

- ₢AlAAI (research-cloudbuild-provenance-mechanics) complete with clear recommendation

## Current State

- Depot demo1025 is live with private pool, all SAs created, triggers active
- Busybox builds and pushes successfully — only provenance is missing
- No depot recreation needed — just stitch change → re-inscribe → dispatch

## Key Files

- `Tools/rbw/rbf_Foundry.sh` — `zrbf_stitch_build_json` generates the `cloudbuild.json` per vessel
- `Tools/rbw/rbgjb/*.sh` — individual build step scripts
- `rbev-vessels/*/cloudbuild.json` — generated build configs (7 vessels)
- `lenses/RBSOB-oci_layout_bridge.adoc` — OCI Layout Bridge spec (may need update if push strategy changes)

## Scope

- Update `zrbf_stitch_build_json` in `Tools/rbw/rbf_Foundry.sh` per ₢AlAAI findings
- At minimum: add `requestedVerifyOption: VERIFIED` to build options
- If `images:` field needed: determine how to declare the pushed image URI/digest
- If OCI Layout Bridge push strategy needs restructuring, update RBSOB spec

## Operational Sequence (on current depot)

1. Make stitch code changes
2. Run inscribe: `tt/rbw-DI.DirectorInscribesRubric.sh` — will detect JSON mismatch, regenerate, ask to commit
3. Commit the 7 regenerated `cloudbuild.json` files (size guard override ~67KB needed)
4. Re-run inscribe — pushes corrected JSONs to rubric repo
5. Dispatch busybox: `tt/rbw-DC.DirectorConjuresArk.sh rbev-vessels/rbev-busybox`
6. Verify provenance (see below)

## Verification

```
gcloud artifacts docker images describe us-central1-docker.pkg.dev/rbwg-d-demo1025-260304183118/rbw-demo1025-repository/rbev-busybox:<TAG> --show-provenance --format=json
```

## Acceptance Criteria

- `gcloud artifacts docker images describe ... --show-provenance` returns provenance data
- `slsa_build_level` is not "unknown"
- Provenance includes builder.id, source metadata, trigger URI
- All 7 vessel cloudbuild.json regenerated and committed

**[260304-1941] rough**

Apply provenance fix to zrbf_stitch_build_json based on research findings from ₢AlAAI.

## Prerequisites

- ₢AlAAI (research-cloudbuild-provenance-mechanics) complete with clear recommendation

## Scope

- Update `zrbf_stitch_build_json` in `Tools/rbw/rbf_Foundry.sh` per research findings
- At minimum: add `requestedVerifyOption: VERIFIED` to build options
- If `images:` field needed: determine how to declare the pushed image URI/digest
- Re-inscribe on current demo1025 depot (no depot recreation needed)
- Dispatch busybox build and verify SLSA provenance appears on the image

## Acceptance Criteria

- `gcloud artifacts docker images describe ... --show-provenance` returns provenance data
- `slsa_build_level` is not "unknown"
- Provenance includes builder.id, source metadata, trigger URI
- All 7 vessel cloudbuild.json regenerated and committed

### bifurcate-busybox-single-arch (₢AlAAN) [complete]

**[260305-1327] complete**

Create single-architecture busybox vessels for SLSA v1.0 provenance validation.

## Context

The stitch function (₢AlAAJ) supports single-arch vessels only. Before we
can verify SLSA provenance on live infrastructure, we need vessels configured
for single platforms.

## Current Vessel Landscape

| Vessel | Platforms | Notes |
|---|---|---|
| rbev-busybox | amd64, arm64, arm/v7 | 3-platform, needs bifurcation |
| rbev-bottle-anthropic-jupyter | amd64, arm64 | 2-platform |
| rbev-bottle-plantuml | amd64, arm64 | 2-platform |
| rbev-bottle-ubuntu-test | amd64, arm64 | 2-platform |
| rbev-sentry-ubuntu-large | amd64, arm64 | 2-platform |
| rbev-ubu-safety | amd64, arm64 | 2-platform |
| rbev-nginx-ward | (bind mode) | No build |
| trbim-macos | arm64 only | Already single-arch — confirmed conjure vessel |

## Scope

1. **Create `rbev-busybox-amd64`** — new vessel directory, rbrv.env with
   `RBRV_CONJURE_PLATFORMS="linux/amd64"`, same Dockerfile as busybox
2. **Create `rbev-busybox-arm64`** — same but `linux/arm64`
3. **Decide policy for original `rbev-busybox`** — keep with multi-platform
   config (stitch will reject it until multi-platform is implemented)?
   Remove? Document decision in paddock.
4. **Decide policy for other multi-platform vessels** — not bifurcated in
   this pace, but document the plan. They remain multi-platform and stitch
   will reject them until multi-platform support is added.
5. **Note trbim-macos** — already arm64-only (`linux/arm64`), available as
   additional arm64 test target without any changes. Has Dockerfile and
   cloudbuild.json already.

## Infrastructure Note

New vessel sigils (`rbev-busybox-amd64`, `rbev-busybox-arm64`) will need
triggers created in the depot. This requires a depot destroy+create cycle,
handled in ₢AlAAO (verify-single-arch-slsa-e2e). This pace creates the
vessel directories only — no depot operations.

## Acceptance Criteria

- rbev-busybox-amd64 exists with valid rbrv.env, single platform
- rbev-busybox-arm64 exists with valid rbrv.env, single platform
- Policy for original busybox and other multi-platform vessels documented
- Paddock updated with vessel bifurcation status

**[260305-1226] rough**

Create single-architecture busybox vessels for SLSA v1.0 provenance validation.

## Context

The stitch function (₢AlAAJ) supports single-arch vessels only. Before we
can verify SLSA provenance on live infrastructure, we need vessels configured
for single platforms.

## Current Vessel Landscape

| Vessel | Platforms | Notes |
|---|---|---|
| rbev-busybox | amd64, arm64, arm/v7 | 3-platform, needs bifurcation |
| rbev-bottle-anthropic-jupyter | amd64, arm64 | 2-platform |
| rbev-bottle-plantuml | amd64, arm64 | 2-platform |
| rbev-bottle-ubuntu-test | amd64, arm64 | 2-platform |
| rbev-sentry-ubuntu-large | amd64, arm64 | 2-platform |
| rbev-ubu-safety | amd64, arm64 | 2-platform |
| rbev-nginx-ward | (bind mode) | No build |
| trbim-macos | arm64 only | Already single-arch — confirmed conjure vessel |

## Scope

1. **Create `rbev-busybox-amd64`** — new vessel directory, rbrv.env with
   `RBRV_CONJURE_PLATFORMS="linux/amd64"`, same Dockerfile as busybox
2. **Create `rbev-busybox-arm64`** — same but `linux/arm64`
3. **Decide policy for original `rbev-busybox`** — keep with multi-platform
   config (stitch will reject it until multi-platform is implemented)?
   Remove? Document decision in paddock.
4. **Decide policy for other multi-platform vessels** — not bifurcated in
   this pace, but document the plan. They remain multi-platform and stitch
   will reject them until multi-platform support is added.
5. **Note trbim-macos** — already arm64-only (`linux/arm64`), available as
   additional arm64 test target without any changes. Has Dockerfile and
   cloudbuild.json already.

## Infrastructure Note

New vessel sigils (`rbev-busybox-amd64`, `rbev-busybox-arm64`) will need
triggers created in the depot. This requires a depot destroy+create cycle,
handled in ₢AlAAO (verify-single-arch-slsa-e2e). This pace creates the
vessel directories only — no depot operations.

## Acceptance Criteria

- rbev-busybox-amd64 exists with valid rbrv.env, single platform
- rbev-busybox-arm64 exists with valid rbrv.env, single platform
- Policy for original busybox and other multi-platform vessels documented
- Paddock updated with vessel bifurcation status

**[260305-1217] rough**

Create single-architecture busybox vessels for SLSA v1.0 provenance validation.

## Context

The stitch function (₢AlAAJ) now supports single-arch vessels only. Before we
can verify SLSA provenance on live infrastructure, we need vessels configured
for single platforms.

## Current Vessel Landscape

| Vessel | Platforms | Notes |
|---|---|---|
| rbev-busybox | amd64, arm64, arm/v7 | 3-platform, needs bifurcation |
| rbev-bottle-anthropic-jupyter | amd64, arm64 | 2-platform |
| rbev-bottle-plantuml | amd64, arm64 | 2-platform |
| rbev-bottle-ubuntu-test | amd64, arm64 | 2-platform |
| rbev-sentry-ubuntu-large | amd64, arm64 | 2-platform |
| rbev-ubu-safety | amd64, arm64 | 2-platform |
| rbev-nginx-ward | (bind mode) | No build |
| trbim-macos | arm64 only | Already single-arch! |

## Scope

1. **Create `rbev-busybox-amd64`** — new vessel directory, rbrv.env with
   `RBRV_CONJURE_PLATFORMS="linux/amd64"`, same Dockerfile as busybox
2. **Create `rbev-busybox-arm64`** — same but `linux/arm64`
3. **Decide policy for original `rbev-busybox`** — keep with multi-platform
   config (stitch will reject it until multi-platform is implemented)?
   Remove? Document decision in paddock.
4. **Decide policy for other multi-platform vessels** — not bifurcated in
   this pace, but document the plan. They remain multi-platform and stitch
   will reject them until multi-platform support is added.
5. **Note trbim-macos** — already arm64-only, available as additional
   arm64 test target without any changes.

## Acceptance Criteria

- rbev-busybox-amd64 exists with valid rbrv.env, single platform
- rbev-busybox-arm64 exists with valid rbrv.env, single platform
- Policy for original busybox and other multi-platform vessels documented
- Paddock updated with vessel bifurcation status

### verify-single-arch-slsa-e2e (₢AlAAO) [complete]

**[260305-1400] complete**

End-to-end verification of single-arch SLSA v1.0 provenance on live infrastructure.

## Prerequisites

- ₢AlAAJ (stitch-single-arch-slsa) complete — pipeline code ready
- ₢AlAAN (bifurcate-busybox-single-arch) complete — vessels exist

## Context

Pipeline code and vessels are ready. This pace exercises them on demo1025
(or a fresh depot if needed) to prove SLSA Level 3 on trigger-dispatched builds.

## Implementation Notes from ₢AlAAJ

**Image tag format changed.** The primary image tag now uses
`_RBGY_INSCRIBE_TIMESTAMP` + `-image` suffix (e.g., `i20260224_153022-image`)
instead of the old dual-timestamp TAG_BASE format. This was required because
CB's `images:` field can only reference CB substitutions, not runtime-computed
shell variables. TAG_BASE remains for metadata/logging only.

**Syft transport changed.** SBOM generation now uses `docker:IMAGE_URI`
transport (reads from local Docker daemon via socket mount) instead of
`oci-dir:/workspace/oci-amd64`. Verify Syft can access the daemon image.

**SBOM filenames are platform-derived.** Filenames use `linux_amd64` or
`linux_arm64` (derived from `_RBGY_PLATFORMS` via `tr`), not hardcoded.

## Scope

### Infrastructure preparation

- If new vessel sigils need triggers: depot destroy + create cycle
  (user pre-approved depot reconfiguration)
- Pin refresh during depot create
- Governor reset, director/retriever creation as needed

### Test A: amd64 SLSA

1. Inscribe rbev-busybox-amd64: `tt/rbw-DI.DirectorInscribesRubric.sh`
2. Dispatch: `tt/rbw-DC.DirectorConjuresArk.sh rbev-vessels/rbev-busybox-amd64`
3. Verify: `gcloud artifacts docker images describe ... --show-provenance`
4. Confirm: `slsa_build_level: 3`

### Test B: arm64 SLSA (cross-arch on amd64 worker)

1. Inscribe rbev-busybox-arm64
2. Dispatch
3. Verify SLSA provenance
4. This proves QEMU + buildx --load + images: works for foreign arch

### Test C: trbim-macos (existing arm64 vessel)

1. Inscribe trbim-macos (already single-arch arm64)
2. Dispatch
3. Verify SLSA provenance
4. Proves the pipeline works on a pre-existing single-arch vessel

### Verification checklist (per image)

- `slsa_build_level: 3`
- Both v0.1 and v1 provenance occurrences present
- Builder ID is `GoogleHostedWorker`
- Build steps recorded in provenance predicate
- SBOM generated correctly from local daemon image (docker: transport)
- Image tag uses inscribe-timestamp-only format (not dual-timestamp)
- Metadata container tag still uses dual-timestamp TAG_BASE
- SBOM filenames match platform (`sbom.linux_amd64.spdx.json` or `linux_arm64`)

## Acceptance Criteria

- SLSA Level 3 on trigger-dispatched amd64 busybox
- SLSA Level 3 on trigger-dispatched arm64 busybox
- SLSA Level 3 on trigger-dispatched trbim-macos
- All three images inspectable via `--show-provenance`
- No 401/403 auth errors in build logs
- SBOM present on all three images
- Results documented in paddock for downstream spec paces

**[260305-1320] rough**

End-to-end verification of single-arch SLSA v1.0 provenance on live infrastructure.

## Prerequisites

- ₢AlAAJ (stitch-single-arch-slsa) complete — pipeline code ready
- ₢AlAAN (bifurcate-busybox-single-arch) complete — vessels exist

## Context

Pipeline code and vessels are ready. This pace exercises them on demo1025
(or a fresh depot if needed) to prove SLSA Level 3 on trigger-dispatched builds.

## Implementation Notes from ₢AlAAJ

**Image tag format changed.** The primary image tag now uses
`_RBGY_INSCRIBE_TIMESTAMP` + `-image` suffix (e.g., `i20260224_153022-image`)
instead of the old dual-timestamp TAG_BASE format. This was required because
CB's `images:` field can only reference CB substitutions, not runtime-computed
shell variables. TAG_BASE remains for metadata/logging only.

**Syft transport changed.** SBOM generation now uses `docker:IMAGE_URI`
transport (reads from local Docker daemon via socket mount) instead of
`oci-dir:/workspace/oci-amd64`. Verify Syft can access the daemon image.

**SBOM filenames are platform-derived.** Filenames use `linux_amd64` or
`linux_arm64` (derived from `_RBGY_PLATFORMS` via `tr`), not hardcoded.

## Scope

### Infrastructure preparation

- If new vessel sigils need triggers: depot destroy + create cycle
  (user pre-approved depot reconfiguration)
- Pin refresh during depot create
- Governor reset, director/retriever creation as needed

### Test A: amd64 SLSA

1. Inscribe rbev-busybox-amd64: `tt/rbw-DI.DirectorInscribesRubric.sh`
2. Dispatch: `tt/rbw-DC.DirectorConjuresArk.sh rbev-vessels/rbev-busybox-amd64`
3. Verify: `gcloud artifacts docker images describe ... --show-provenance`
4. Confirm: `slsa_build_level: 3`

### Test B: arm64 SLSA (cross-arch on amd64 worker)

1. Inscribe rbev-busybox-arm64
2. Dispatch
3. Verify SLSA provenance
4. This proves QEMU + buildx --load + images: works for foreign arch

### Test C: trbim-macos (existing arm64 vessel)

1. Inscribe trbim-macos (already single-arch arm64)
2. Dispatch
3. Verify SLSA provenance
4. Proves the pipeline works on a pre-existing single-arch vessel

### Verification checklist (per image)

- `slsa_build_level: 3`
- Both v0.1 and v1 provenance occurrences present
- Builder ID is `GoogleHostedWorker`
- Build steps recorded in provenance predicate
- SBOM generated correctly from local daemon image (docker: transport)
- Image tag uses inscribe-timestamp-only format (not dual-timestamp)
- Metadata container tag still uses dual-timestamp TAG_BASE
- SBOM filenames match platform (`sbom.linux_amd64.spdx.json` or `linux_arm64`)

## Acceptance Criteria

- SLSA Level 3 on trigger-dispatched amd64 busybox
- SLSA Level 3 on trigger-dispatched arm64 busybox
- SLSA Level 3 on trigger-dispatched trbim-macos
- All three images inspectable via `--show-provenance`
- No 401/403 auth errors in build logs
- SBOM present on all three images
- Results documented in paddock for downstream spec paces

**[260305-1217] rough**

End-to-end verification of single-arch SLSA v1.0 provenance on live infrastructure.

## Prerequisites

- ₢AlAAJ (stitch-single-arch-slsa) complete — pipeline code ready
- ₢AlAAN (bifurcate-busybox-single-arch) complete — vessels exist

## Context

Pipeline code and vessels are ready. This pace exercises them on demo1025
(or a fresh depot if needed) to prove SLSA Level 3 on trigger-dispatched builds.

## Scope

### Infrastructure preparation

- If new vessel sigils need triggers: depot destroy + create cycle
  (user pre-approved depot reconfiguration)
- Pin refresh during depot create
- Governor reset, director/retriever creation as needed

### Test A: amd64 SLSA

1. Inscribe rbev-busybox-amd64: `tt/rbw-DI.DirectorInscribesRubric.sh`
2. Dispatch: `tt/rbw-DC.DirectorConjuresArk.sh rbev-vessels/rbev-busybox-amd64`
3. Verify: `gcloud artifacts docker images describe ... --show-provenance`
4. Confirm: `slsa_build_level: 3`

### Test B: arm64 SLSA (cross-arch on amd64 worker)

1. Inscribe rbev-busybox-arm64
2. Dispatch
3. Verify SLSA provenance
4. This proves QEMU + buildx --load + images: works for foreign arch

### Test C: trbim-macos (existing arm64 vessel)

1. Inscribe trbim-macos (already single-arch arm64)
2. Dispatch
3. Verify SLSA provenance
4. Proves the pipeline works on a pre-existing single-arch vessel

### Verification checklist (per image)

- `slsa_build_level: 3`
- Both v0.1 and v1 provenance occurrences present
- Builder ID is `GoogleHostedWorker`
- Build steps recorded in provenance predicate
- SBOM generated correctly from local daemon image

## Acceptance Criteria

- SLSA Level 3 on trigger-dispatched amd64 busybox
- SLSA Level 3 on trigger-dispatched arm64 busybox
- SLSA Level 3 on trigger-dispatched trbim-macos
- All three images inspectable via `--show-provenance`
- No 401/403 auth errors in build logs
- SBOM present on all three images
- Results documented in paddock for downstream spec paces

### spec-single-arch-provenance (₢AlAAP) [complete]

**[260305-1408] complete**

Update specifications to reflect validated single-arch SLSA v1.0 provenance.

## Prerequisites

- ₢AlAAO (verify-single-arch-slsa-e2e) complete — confirmed facts only

## Context

With SLSA Level 3 confirmed on live trigger-dispatched builds for both amd64
and arm64, update specs to reflect reality. State only confirmed facts.

## Scope

### RBS0-SpecTop.adoc (lines ~1455-1472)

- Remove "Architectural constraint" paragraph about OCI Layout Bridge
- Update `rbtgr_provenance` definition: single-arch vessels produce SLSA
  Level 3 via `--load` + `images:` + `VERIFIED`
- Document that multi-platform provenance is not yet supported
- Reference experiment evidence (memo, build IDs)
- Note: CB pre-populates Docker credentials (steps 02/03 eliminated)

### RBSOB-oci_layout_bridge.adoc

- Mark OCI Layout Bridge as superseded for single-arch vessels
- Explain why: buildx --load puts image in local daemon directly
- Note: may be revisited for multi-platform provenance path
- Do not delete — historical record and potential future use

### Paddock update

- Record spec changes and their basis in verified experiment results

## Acceptance Criteria

- RBS0 provenance definition states only confirmed, tested facts
- No "we hope" or "should work" language in specs
- RBSOB clearly marks superseded status with rationale
- All spec assertions traceable to experiment build IDs

**[260305-1408] complete**

Update specifications to reflect validated single-arch SLSA v1.0 provenance.

## Prerequisites

- ₢AlAAO (verify-single-arch-slsa-e2e) complete — confirmed facts only

## Context

With SLSA Level 3 confirmed on live trigger-dispatched builds for both amd64
and arm64, update specs to reflect reality. State only confirmed facts.

## Scope

### RBS0-SpecTop.adoc (lines ~1455-1472)

- Remove "Architectural constraint" paragraph about OCI Layout Bridge
- Update `rbtgr_provenance` definition: single-arch vessels produce SLSA
  Level 3 via `--load` + `images:` + `VERIFIED`
- Document that multi-platform provenance is not yet supported
- Reference experiment evidence (memo, build IDs)
- Note: CB pre-populates Docker credentials (steps 02/03 eliminated)

### RBSOB-oci_layout_bridge.adoc

- Mark OCI Layout Bridge as superseded for single-arch vessels
- Explain why: buildx --load puts image in local daemon directly
- Note: may be revisited for multi-platform provenance path
- Do not delete — historical record and potential future use

### Paddock update

- Record spec changes and their basis in verified experiment results

## Acceptance Criteria

- RBS0 provenance definition states only confirmed, tested facts
- No "we hope" or "should work" language in specs
- RBSOB clearly marks superseded status with rationale
- All spec assertions traceable to experiment build IDs

**[260305-1218] rough**

Update specifications to reflect validated single-arch SLSA v1.0 provenance.

## Prerequisites

- ₢AlAAO (verify-single-arch-slsa-e2e) complete — confirmed facts only

## Context

With SLSA Level 3 confirmed on live trigger-dispatched builds for both amd64
and arm64, update specs to reflect reality. State only confirmed facts.

## Scope

### RBS0-SpecTop.adoc (lines ~1455-1472)

- Remove "Architectural constraint" paragraph about OCI Layout Bridge
- Update `rbtgr_provenance` definition: single-arch vessels produce SLSA
  Level 3 via `--load` + `images:` + `VERIFIED`
- Document that multi-platform provenance is not yet supported
- Reference experiment evidence (memo, build IDs)
- Note: CB pre-populates Docker credentials (steps 02/03 eliminated)

### RBSOB-oci_layout_bridge.adoc

- Mark OCI Layout Bridge as superseded for single-arch vessels
- Explain why: buildx --load puts image in local daemon directly
- Note: may be revisited for multi-platform provenance path
- Do not delete — historical record and potential future use

### Paddock update

- Record spec changes and their basis in verified experiment results

## Acceptance Criteria

- RBS0 provenance definition states only confirmed, tested facts
- No "we hope" or "should work" language in specs
- RBSOB clearly marks superseded status with rationale
- All spec assertions traceable to experiment build IDs

### experiment-multiplatform-slsa-provenance (₢AlAAQ) [complete]

**[260305-1428] complete**

Test whether per-platform pullback can produce SLSA v1.0 provenance on all
sub-images of a multi-platform build.

## Context

Single-arch SLSA is proven (₢AlAAK, ₢AlAAL experiments on 2026-03-05).
The structural constraint: CB's `images:` field pushes from Docker's local
daemon, which is single-platform only. Multi-platform manifests cannot exist
in the local daemon.

**Core question:** Can we build multi-platform via `buildx --push`, then pull
back EACH platform individually into the local daemon, and get SLSA provenance
on each platform's digest — all within a single build invocation?

## What We Know (from prior experiments)

**Experiment 3 (build 48b818ed, 2026-03-05):**
- `docker pull` of a multi-platform manifest on amd64 worker pulls amd64 only
- Pulled image enters local daemon, `images:` pushes it, SLSA Level 3 generated
- Tag is overwritten (multi-platform manifest replaced by single-platform)
- Provenance attaches to the pulled-back single-platform digest
- Provenance predicate records ALL build steps (including multi-platform buildx)

**Confirmed toolchain (must use same builder image for reproducibility):**
- Docker Engine 20.10.24, API 1.41, Experimental: true (client AND server)
- docker buildx v0.23.0 (28c90ea)
- BuildKit moby/buildkit:buildx-stable-1
- Builder image: `gcr.io/cloud-builders/docker@sha256:efdbd755476e7e5eb1077ed6e4bf691f87b38fbded575e9b825f9480374f8f4b`
- binfmt: `docker.io/tonistiigi/binfmt@sha256:8db0f28060565399642110b798c6c35efcac7c5b3b48c56d36503d3b4d8f93c8`

**ADC pre-population:** CB pre-populates `/builder/home/.docker/config.json`
with oauth2accesstoken for all GAR regions. No docker login step needed.
(Experiment 2, build 4de1467a.)

**Provenance signing keys (from Experiment 3):**
- v0.1: `provenanceSigner` (global), `builtByGCB` (us-central1)
- v1: `google-hosted-worker` (global)

## Web Research Findings (2026-03-05)

Research conducted before this experiment to assess viability of the
multi-platform rejoining path. Key findings:

1. **CB `images:` accepts a list** — each URI gets independent SLSA provenance.
   Documented in CB build config schema and provenance docs. A single build
   invocation can declare TAG-amd64, TAG-arm64, TAG-armv7 and each gets
   SLSA Level 3 with the same buildInvocationId.

2. **`docker buildx imagetools create` preserves attestations** — per buildx
   PR #3433 and Docker docs, `imagetools create` now persists attestation
   manifests when combining per-platform images into a multi-platform index.
   Operates registry-side, no local daemon needed for reassembly. This is
   the mechanism for reconstructing a multi-platform manifest list without
   losing provenance.

3. **CB workers use classic Docker image store** (Docker 20.10.24, not
   containerd). Classic store is single-platform only. If Google upgrades
   to containerd in the future, the pullback strategy becomes unnecessary.

4. **`docker pull --platform` has edge cases** — reported silent fallback
   to host arch on Docker Desktop (github.com/docker/for-mac/issues/5625).
   CB workers are Linux daemon (not Desktop), but this reinforces why
   Variant B (digest-based pulls) is preferred over Variant A.

5. **BuildKit attaches per-platform attestations on multi-platform builds** —
   stored in OCI image index under `unknown/unknown` platform entries. CB-native
   SLSA is additive on top of BuildKit's own attestations.

Full evidence: `Memos/memo-20260305-provenance-architecture-gap.md`
(section: "Multi-Platform Rejoining Research")

## Key Unknowns

1. Does `docker pull --platform linux/arm64` work on an amd64 CB worker?
   The `--platform` flag on `docker pull` requires Experimental: true on the
   client side — confirmed available in our CB worker (Docker 20.10.24,
   `Experimental: true` observed in Experiment 1 build log).
   **Primary risk:** Docker may accept the flag but `images:` pushing a
   foreign-arch image from the local daemon is untested.

2. Does `docker manifest inspect` work on CB workers? Docker 20.10.24 with
   Experimental: true should support it. This enables digest-based pulls
   which are more reliable than `--platform` flag resolution.

3. Can `images:` declare multiple image URIs and generate provenance on each?
   Research confirms this is documented behavior — but untested by us with
   multiple entries.

## Experiment Method

Use `gcloud builds submit --no-source` on demo1025 (same as prior experiments).
Use the proven inline-Dockerfile-via-step pattern from build 48b818ed:
step 0 writes busybox Dockerfile to /workspace via heredoc. Same private pool
and builder images.

## Variant B: Manifest inspect + digest-based pull (PREFERRED — run first)

This is the most robust approach: extract per-platform digests from the
manifest list, then pull each by exact digest. No platform resolution ambiguity.

Steps:
1. create-dockerfile (inline heredoc)
2. qemu-binfmt (register arm64, arm)
3. buildx --push multi-platform → TAG-multi
4. docker manifest inspect IMAGE:TAG-multi
   Parse with jq: `.manifests[] | select(.platform.architecture=="amd64") | .digest`
   (fields: `.manifests[].platform.architecture`, `.manifests[].platform.os`,
   `.manifests[].platform.variant` for arm/v7)
5. docker pull IMAGE@sha256:<amd64-digest>; docker tag → IMAGE:TAG-amd64
6. docker pull IMAGE@sha256:<arm64-digest>; docker tag → IMAGE:TAG-arm64
7. docker pull IMAGE@sha256:<armv7-digest>; docker tag → IMAGE:TAG-armv7

cloudbuild.json `images:` declares all three tagged URIs.
`requestedVerifyOption: VERIFIED`.

**Why preferred:** Pulling by digest is unambiguous. Docker doesn't need to
resolve platform matching — the digest identifies exactly one manifest.
`docker manifest inspect` is confirmed available (Experimental: true in our
CB worker). If this works, it's the production path.

**Risk:** `docker manifest inspect` output format may differ across Docker
versions. Fallback: Variant D uses `buildx imagetools inspect --raw` which
produces a more stable OCI-standard JSON.

## Variant D: buildx imagetools inspect (fallback for manifest inspect)

If `docker manifest inspect` fails or has unexpected output format, try
`docker buildx imagetools inspect --raw IMAGE:TAG-multi | jq` to extract
per-platform digests. Uses buildx's registry inspection which produces
OCI-standard manifest list JSON. Otherwise identical to Variant B.

## Variant A: Per-platform pull with --platform flag

Steps:
1. create-dockerfile (inline heredoc)
2. qemu-binfmt (register arm64, arm)
3. buildx --push multi-platform (amd64, arm64, arm/v7) → TAG-multi
4. docker pull --platform linux/amd64 IMAGE:TAG-multi; docker tag → IMAGE:TAG-amd64
5. docker pull --platform linux/arm64 IMAGE:TAG-multi; docker tag → IMAGE:TAG-arm64
6. docker pull --platform linux/arm/v7 IMAGE:TAG-multi; docker tag → IMAGE:TAG-armv7

cloudbuild.json `images:` declares all three tagged URIs.
`requestedVerifyOption: VERIFIED`.

**What this tests:** Whether `--platform` flag on `docker pull` resolves the
correct platform from a multi-platform manifest, and whether `images:` can
push foreign-arch images from the local daemon.

**Risk:** `--platform` flag requires Experimental: true (confirmed available).
But the resulting foreign-arch image in the local store may confuse `images:` push.
Research found silent fallback bugs on Docker Desktop; CB Linux daemon may differ.

## Variant C: Multiple single-platform --load builds (DOCUMENT ONLY)

**DO NOT TREAT AS A VIABLE PRODUCTION PATH.** Run only if A/B/D all fail,
and only to document what CB does — not as a candidate architecture.

**Why demoted:** With Variants A/B/D, the provenance recipe for every platform
image records the FULL multi-platform `buildx --push` step. A verifier can
confirm all platforms were built together in one invocation (same
buildInvocationId). With Variant C, each platform is a SEPARATE build — there
is NO single build invocation tying them together. A verifier cannot confirm
the images share the same source commit. This is a fundamentally different
trust model, not just a lesser version of A/B/D.

Steps (if needed for documentation):
1. create-dockerfile (inline heredoc)
2. qemu-binfmt (register arm64, arm)
3. buildx build --load --platform=linux/amd64 --tag IMAGE:TAG-amd64 .
4. buildx build --load --platform=linux/arm64 --tag IMAGE:TAG-arm64 .
5. buildx build --load --platform=linux/arm/v7 --tag IMAGE:TAG-armv7 .

## Execution Order

1. Run Variant B first (preferred — digest-based, most robust)
2. If B fails on manifest inspect: try Variant D (imagetools fallback)
3. If B/D fail on digest pull: try Variant A (--platform flag)
4. Run Variant C only if A/B/D all fail — document failure mode only
5. Document all results regardless of success/failure

## Post-Experiment: Manifest List Reassembly (if provenance succeeds)

If any variant produces per-platform SLSA provenance, test manifest list
reassembly as a follow-up experiment in the same session:

```
docker buildx imagetools create \
  -t IMAGE:TAG \
  IMAGE:TAG-amd64 \
  IMAGE:TAG-arm64 \
  IMAGE:TAG-armv7
```

Per buildx PR #3433, `imagetools create` preserves attestation manifests
when combining per-platform images. Verify:
- `IMAGE:TAG` resolves as a multi-platform manifest list
- `docker pull IMAGE:TAG` on amd64 gets the amd64 variant
- `gcloud artifacts docker images describe IMAGE:TAG-amd64 --show-provenance`
  still shows SLSA Level 3 (provenance not lost by reassembly)
- The manifest list itself at IMAGE:TAG — does it have provenance?
  (Probably not, but the per-platform images referenced by it should retain theirs)

This tests the full rejoining path: build multi-platform → attest each
platform → reconstruct the manifest list → consumers get transparent
platform resolution with per-platform provenance.

## Downstream Questions (if experiment succeeds)

- Is per-platform tagging acceptable for production consumers, or do they
  need the transparent `docker pull IMAGE:TAG` resolution?
  (Research suggests `imagetools create` solves this — manifest list reassembly
  gives consumers the transparent pull experience)
- Should the stitch function have two code paths (single-arch --load vs
  multi-platform --push+pullback), or should all vessels use --push+pullback?
- What is the tag scheme? TAG-multi (build artifact), TAG-amd64/TAG-arm64
  (attested per-platform), TAG (reassembled manifest list)?

## Verification (per variant)

For each platform image, verify via:
```
gcloud artifacts docker images describe \
  IMAGE:TAG-<platform> --show-provenance --format=yaml
```

Check:
- `slsa_build_level: 3`
- Provenance `subject.digest` matches the platform-specific image digest
- `metadata.buildInvocationId` is IDENTICAL across all platform images
  (Variants A/B/D only — proves single-build origin)
- Provenance `recipe.arguments.steps` contains the buildx step with
  `--platform=linux/amd64,linux/arm64,linux/arm/v7` (proves full
  multi-platform build is recorded, not just a single-platform build)
- Provenance signatures use same keyids as Experiment 3:
  - v0.1: `provenanceSigner/cryptoKeyVersions/1` (global) and
    `builtByGCB/cryptoKeyVersions/1` (us-central1)
  - v1: `google-hosted-worker/cryptoKeyVersions/1` (global)
- `resolvedDependencies` lists same builder image digest
  (`sha256:efdbd755...`)
- No 401/403 errors
- Build completes under VERIFIED constraint

For Variant C specifically (if run):
- `buildInvocationId` will DIFFER per platform (expected — document this)
- Each recipe records only that platform's build step (expected limitation)
- This confirms Variant C is NOT viable for same-source verification

## Key Files

- `Memos/memo-20260305-provenance-architecture-gap.md` — prior experiment evidence + rejoining research
- `Memos/experiments/cloudbuild-test-provenance.json` — Experiment 3 config (template)

## Acceptance Criteria

- At least one variant produces SLSA Level 3 on ALL platform sub-images
  (amd64, arm64, arm/v7)
- For Variants A/B/D: same buildInvocationId across all platform provenance
  records, AND provenance predicate contains full multi-platform build steps,
  AND signing keys match Experiment 3 trust chain
- OR: documented failure modes for all variants with exact error messages,
  toolchain versions, and build IDs — same evidence standard as prior experiments
- If provenance succeeds: manifest list reassembly via `imagetools create`
  tested and results documented (success or failure)
- Results added to provenance memo with full detail (build IDs, digests,
  provenance YAML extracts, log evidence)

**[260305-1323] rough**

Test whether per-platform pullback can produce SLSA v1.0 provenance on all
sub-images of a multi-platform build.

## Context

Single-arch SLSA is proven (₢AlAAK, ₢AlAAL experiments on 2026-03-05).
The structural constraint: CB's `images:` field pushes from Docker's local
daemon, which is single-platform only. Multi-platform manifests cannot exist
in the local daemon.

**Core question:** Can we build multi-platform via `buildx --push`, then pull
back EACH platform individually into the local daemon, and get SLSA provenance
on each platform's digest — all within a single build invocation?

## What We Know (from prior experiments)

**Experiment 3 (build 48b818ed, 2026-03-05):**
- `docker pull` of a multi-platform manifest on amd64 worker pulls amd64 only
- Pulled image enters local daemon, `images:` pushes it, SLSA Level 3 generated
- Tag is overwritten (multi-platform manifest replaced by single-platform)
- Provenance attaches to the pulled-back single-platform digest
- Provenance predicate records ALL build steps (including multi-platform buildx)

**Confirmed toolchain (must use same builder image for reproducibility):**
- Docker Engine 20.10.24, API 1.41, Experimental: true (client AND server)
- docker buildx v0.23.0 (28c90ea)
- BuildKit moby/buildkit:buildx-stable-1
- Builder image: `gcr.io/cloud-builders/docker@sha256:efdbd755476e7e5eb1077ed6e4bf691f87b38fbded575e9b825f9480374f8f4b`
- binfmt: `docker.io/tonistiigi/binfmt@sha256:8db0f28060565399642110b798c6c35efcac7c5b3b48c56d36503d3b4d8f93c8`

**ADC pre-population:** CB pre-populates `/builder/home/.docker/config.json`
with oauth2accesstoken for all GAR regions. No docker login step needed.
(Experiment 2, build 4de1467a.)

**Provenance signing keys (from Experiment 3):**
- v0.1: `provenanceSigner` (global), `builtByGCB` (us-central1)
- v1: `google-hosted-worker` (global)

## Web Research Findings (2026-03-05)

Research conducted before this experiment to assess viability of the
multi-platform rejoining path. Key findings:

1. **CB `images:` accepts a list** — each URI gets independent SLSA provenance.
   Documented in CB build config schema and provenance docs. A single build
   invocation can declare TAG-amd64, TAG-arm64, TAG-armv7 and each gets
   SLSA Level 3 with the same buildInvocationId.

2. **`docker buildx imagetools create` preserves attestations** — per buildx
   PR #3433 and Docker docs, `imagetools create` now persists attestation
   manifests when combining per-platform images into a multi-platform index.
   Operates registry-side, no local daemon needed for reassembly. This is
   the mechanism for reconstructing a multi-platform manifest list without
   losing provenance.

3. **CB workers use classic Docker image store** (Docker 20.10.24, not
   containerd). Classic store is single-platform only. If Google upgrades
   to containerd in the future, the pullback strategy becomes unnecessary.

4. **`docker pull --platform` has edge cases** — reported silent fallback
   to host arch on Docker Desktop (github.com/docker/for-mac/issues/5625).
   CB workers are Linux daemon (not Desktop), but this reinforces why
   Variant B (digest-based pulls) is preferred over Variant A.

5. **BuildKit attaches per-platform attestations on multi-platform builds** —
   stored in OCI image index under `unknown/unknown` platform entries. CB-native
   SLSA is additive on top of BuildKit's own attestations.

Full evidence: `Memos/memo-20260305-provenance-architecture-gap.md`
(section: "Multi-Platform Rejoining Research")

## Key Unknowns

1. Does `docker pull --platform linux/arm64` work on an amd64 CB worker?
   The `--platform` flag on `docker pull` requires Experimental: true on the
   client side — confirmed available in our CB worker (Docker 20.10.24,
   `Experimental: true` observed in Experiment 1 build log).
   **Primary risk:** Docker may accept the flag but `images:` pushing a
   foreign-arch image from the local daemon is untested.

2. Does `docker manifest inspect` work on CB workers? Docker 20.10.24 with
   Experimental: true should support it. This enables digest-based pulls
   which are more reliable than `--platform` flag resolution.

3. Can `images:` declare multiple image URIs and generate provenance on each?
   Research confirms this is documented behavior — but untested by us with
   multiple entries.

## Experiment Method

Use `gcloud builds submit --no-source` on demo1025 (same as prior experiments).
Use the proven inline-Dockerfile-via-step pattern from build 48b818ed:
step 0 writes busybox Dockerfile to /workspace via heredoc. Same private pool
and builder images.

## Variant B: Manifest inspect + digest-based pull (PREFERRED — run first)

This is the most robust approach: extract per-platform digests from the
manifest list, then pull each by exact digest. No platform resolution ambiguity.

Steps:
1. create-dockerfile (inline heredoc)
2. qemu-binfmt (register arm64, arm)
3. buildx --push multi-platform → TAG-multi
4. docker manifest inspect IMAGE:TAG-multi
   Parse with jq: `.manifests[] | select(.platform.architecture=="amd64") | .digest`
   (fields: `.manifests[].platform.architecture`, `.manifests[].platform.os`,
   `.manifests[].platform.variant` for arm/v7)
5. docker pull IMAGE@sha256:<amd64-digest>; docker tag → IMAGE:TAG-amd64
6. docker pull IMAGE@sha256:<arm64-digest>; docker tag → IMAGE:TAG-arm64
7. docker pull IMAGE@sha256:<armv7-digest>; docker tag → IMAGE:TAG-armv7

cloudbuild.json `images:` declares all three tagged URIs.
`requestedVerifyOption: VERIFIED`.

**Why preferred:** Pulling by digest is unambiguous. Docker doesn't need to
resolve platform matching — the digest identifies exactly one manifest.
`docker manifest inspect` is confirmed available (Experimental: true in our
CB worker). If this works, it's the production path.

**Risk:** `docker manifest inspect` output format may differ across Docker
versions. Fallback: Variant D uses `buildx imagetools inspect --raw` which
produces a more stable OCI-standard JSON.

## Variant D: buildx imagetools inspect (fallback for manifest inspect)

If `docker manifest inspect` fails or has unexpected output format, try
`docker buildx imagetools inspect --raw IMAGE:TAG-multi | jq` to extract
per-platform digests. Uses buildx's registry inspection which produces
OCI-standard manifest list JSON. Otherwise identical to Variant B.

## Variant A: Per-platform pull with --platform flag

Steps:
1. create-dockerfile (inline heredoc)
2. qemu-binfmt (register arm64, arm)
3. buildx --push multi-platform (amd64, arm64, arm/v7) → TAG-multi
4. docker pull --platform linux/amd64 IMAGE:TAG-multi; docker tag → IMAGE:TAG-amd64
5. docker pull --platform linux/arm64 IMAGE:TAG-multi; docker tag → IMAGE:TAG-arm64
6. docker pull --platform linux/arm/v7 IMAGE:TAG-multi; docker tag → IMAGE:TAG-armv7

cloudbuild.json `images:` declares all three tagged URIs.
`requestedVerifyOption: VERIFIED`.

**What this tests:** Whether `--platform` flag on `docker pull` resolves the
correct platform from a multi-platform manifest, and whether `images:` can
push foreign-arch images from the local daemon.

**Risk:** `--platform` flag requires Experimental: true (confirmed available).
But the resulting foreign-arch image in the local store may confuse `images:` push.
Research found silent fallback bugs on Docker Desktop; CB Linux daemon may differ.

## Variant C: Multiple single-platform --load builds (DOCUMENT ONLY)

**DO NOT TREAT AS A VIABLE PRODUCTION PATH.** Run only if A/B/D all fail,
and only to document what CB does — not as a candidate architecture.

**Why demoted:** With Variants A/B/D, the provenance recipe for every platform
image records the FULL multi-platform `buildx --push` step. A verifier can
confirm all platforms were built together in one invocation (same
buildInvocationId). With Variant C, each platform is a SEPARATE build — there
is NO single build invocation tying them together. A verifier cannot confirm
the images share the same source commit. This is a fundamentally different
trust model, not just a lesser version of A/B/D.

Steps (if needed for documentation):
1. create-dockerfile (inline heredoc)
2. qemu-binfmt (register arm64, arm)
3. buildx build --load --platform=linux/amd64 --tag IMAGE:TAG-amd64 .
4. buildx build --load --platform=linux/arm64 --tag IMAGE:TAG-arm64 .
5. buildx build --load --platform=linux/arm/v7 --tag IMAGE:TAG-armv7 .

## Execution Order

1. Run Variant B first (preferred — digest-based, most robust)
2. If B fails on manifest inspect: try Variant D (imagetools fallback)
3. If B/D fail on digest pull: try Variant A (--platform flag)
4. Run Variant C only if A/B/D all fail — document failure mode only
5. Document all results regardless of success/failure

## Post-Experiment: Manifest List Reassembly (if provenance succeeds)

If any variant produces per-platform SLSA provenance, test manifest list
reassembly as a follow-up experiment in the same session:

```
docker buildx imagetools create \
  -t IMAGE:TAG \
  IMAGE:TAG-amd64 \
  IMAGE:TAG-arm64 \
  IMAGE:TAG-armv7
```

Per buildx PR #3433, `imagetools create` preserves attestation manifests
when combining per-platform images. Verify:
- `IMAGE:TAG` resolves as a multi-platform manifest list
- `docker pull IMAGE:TAG` on amd64 gets the amd64 variant
- `gcloud artifacts docker images describe IMAGE:TAG-amd64 --show-provenance`
  still shows SLSA Level 3 (provenance not lost by reassembly)
- The manifest list itself at IMAGE:TAG — does it have provenance?
  (Probably not, but the per-platform images referenced by it should retain theirs)

This tests the full rejoining path: build multi-platform → attest each
platform → reconstruct the manifest list → consumers get transparent
platform resolution with per-platform provenance.

## Downstream Questions (if experiment succeeds)

- Is per-platform tagging acceptable for production consumers, or do they
  need the transparent `docker pull IMAGE:TAG` resolution?
  (Research suggests `imagetools create` solves this — manifest list reassembly
  gives consumers the transparent pull experience)
- Should the stitch function have two code paths (single-arch --load vs
  multi-platform --push+pullback), or should all vessels use --push+pullback?
- What is the tag scheme? TAG-multi (build artifact), TAG-amd64/TAG-arm64
  (attested per-platform), TAG (reassembled manifest list)?

## Verification (per variant)

For each platform image, verify via:
```
gcloud artifacts docker images describe \
  IMAGE:TAG-<platform> --show-provenance --format=yaml
```

Check:
- `slsa_build_level: 3`
- Provenance `subject.digest` matches the platform-specific image digest
- `metadata.buildInvocationId` is IDENTICAL across all platform images
  (Variants A/B/D only — proves single-build origin)
- Provenance `recipe.arguments.steps` contains the buildx step with
  `--platform=linux/amd64,linux/arm64,linux/arm/v7` (proves full
  multi-platform build is recorded, not just a single-platform build)
- Provenance signatures use same keyids as Experiment 3:
  - v0.1: `provenanceSigner/cryptoKeyVersions/1` (global) and
    `builtByGCB/cryptoKeyVersions/1` (us-central1)
  - v1: `google-hosted-worker/cryptoKeyVersions/1` (global)
- `resolvedDependencies` lists same builder image digest
  (`sha256:efdbd755...`)
- No 401/403 errors
- Build completes under VERIFIED constraint

For Variant C specifically (if run):
- `buildInvocationId` will DIFFER per platform (expected — document this)
- Each recipe records only that platform's build step (expected limitation)
- This confirms Variant C is NOT viable for same-source verification

## Key Files

- `Memos/memo-20260305-provenance-architecture-gap.md` — prior experiment evidence + rejoining research
- `Memos/experiments/cloudbuild-test-provenance.json` — Experiment 3 config (template)

## Acceptance Criteria

- At least one variant produces SLSA Level 3 on ALL platform sub-images
  (amd64, arm64, arm/v7)
- For Variants A/B/D: same buildInvocationId across all platform provenance
  records, AND provenance predicate contains full multi-platform build steps,
  AND signing keys match Experiment 3 trust chain
- OR: documented failure modes for all variants with exact error messages,
  toolchain versions, and build IDs — same evidence standard as prior experiments
- If provenance succeeds: manifest list reassembly via `imagetools create`
  tested and results documented (success or failure)
- Results added to provenance memo with full detail (build IDs, digests,
  provenance YAML extracts, log evidence)

**[260305-1246] rough**

Test whether per-platform pullback can produce SLSA v1.0 provenance on all
sub-images of a multi-platform build.

## Context

Single-arch SLSA is proven (₢AlAAK, ₢AlAAL experiments on 2026-03-05).
The structural constraint: CB's `images:` field pushes from Docker's local
daemon, which is single-platform only. Multi-platform manifests cannot exist
in the local daemon.

**Core question:** Can we build multi-platform via `buildx --push`, then pull
back EACH platform individually into the local daemon, and get SLSA provenance
on each platform's digest — all within a single build invocation?

## What We Know (from prior experiments)

**Experiment 3 (build 48b818ed, 2026-03-05):**
- `docker pull` of a multi-platform manifest on amd64 worker pulls amd64 only
- Pulled image enters local daemon, `images:` pushes it, SLSA Level 3 generated
- Tag is overwritten (multi-platform manifest replaced by single-platform)
- Provenance attaches to the pulled-back single-platform digest
- Provenance predicate records ALL build steps (including multi-platform buildx)

**Confirmed toolchain (must use same builder image for reproducibility):**
- Docker Engine 20.10.24, API 1.41, Experimental: true (client AND server)
- docker buildx v0.23.0 (28c90ea)
- BuildKit moby/buildkit:buildx-stable-1
- Builder image: `gcr.io/cloud-builders/docker@sha256:efdbd755476e7e5eb1077ed6e4bf691f87b38fbded575e9b825f9480374f8f4b`
- binfmt: `docker.io/tonistiigi/binfmt@sha256:8db0f28060565399642110b798c6c35efcac7c5b3b48c56d36503d3b4d8f93c8`

**ADC pre-population:** CB pre-populates `/builder/home/.docker/config.json`
with oauth2accesstoken for all GAR regions. No docker login step needed.
(Experiment 2, build 4de1467a.)

**Provenance signing keys (from Experiment 3):**
- v0.1: `provenanceSigner` (global), `builtByGCB` (us-central1)
- v1: `google-hosted-worker` (global)

## Key Unknowns

1. Does `docker pull --platform linux/arm64` work on an amd64 CB worker?
   The `--platform` flag on `docker pull` requires Experimental: true on the
   client side — confirmed available in our CB worker (Docker 20.10.24,
   `Experimental: true` observed in Experiment 1 build log).
   **Primary risk:** Docker may accept the flag but `images:` pushing a
   foreign-arch image from the local daemon is untested.

2. Does `docker manifest inspect` work on CB workers? Docker 20.10.24 with
   Experimental: true should support it. This enables digest-based pulls
   which are more reliable than `--platform` flag resolution.

3. Can `images:` declare multiple image URIs and generate provenance on each?
   The `images:` field accepts a list. Each declared image should get its own
   provenance occurrence. Untested with multiple entries.

## Experiment Method

Use `gcloud builds submit --no-source` on demo1025 (same as prior experiments).
Use the proven inline-Dockerfile-via-step pattern from build 48b818ed:
step 0 writes busybox Dockerfile to /workspace via heredoc. Same private pool
and builder images.

## Variant B: Manifest inspect + digest-based pull (PREFERRED — run first)

This is the most robust approach: extract per-platform digests from the
manifest list, then pull each by exact digest. No platform resolution ambiguity.

Steps:
1. create-dockerfile (inline heredoc)
2. qemu-binfmt (register arm64, arm)
3. buildx --push multi-platform → TAG-multi
4. docker manifest inspect IMAGE:TAG-multi
   Parse with jq: `.manifests[] | select(.platform.architecture=="amd64") | .digest`
   (fields: `.manifests[].platform.architecture`, `.manifests[].platform.os`,
   `.manifests[].platform.variant` for arm/v7)
5. docker pull IMAGE@sha256:<amd64-digest>; docker tag → IMAGE:TAG-amd64
6. docker pull IMAGE@sha256:<arm64-digest>; docker tag → IMAGE:TAG-arm64
7. docker pull IMAGE@sha256:<armv7-digest>; docker tag → IMAGE:TAG-armv7

cloudbuild.json `images:` declares all three tagged URIs.
`requestedVerifyOption: VERIFIED`.

**Why preferred:** Pulling by digest is unambiguous. Docker doesn't need to
resolve platform matching — the digest identifies exactly one manifest.
`docker manifest inspect` is confirmed available (Experimental: true in our
CB worker). If this works, it's the production path.

**Risk:** `docker manifest inspect` output format may differ across Docker
versions. Fallback: Variant D uses `buildx imagetools inspect --raw` which
produces a more stable OCI-standard JSON.

## Variant D: buildx imagetools inspect (fallback for manifest inspect)

If `docker manifest inspect` fails or has unexpected output format, try
`docker buildx imagetools inspect --raw IMAGE:TAG-multi | jq` to extract
per-platform digests. Uses buildx's registry inspection which produces
OCI-standard manifest list JSON. Otherwise identical to Variant B.

## Variant A: Per-platform pull with --platform flag

Steps:
1. create-dockerfile (inline heredoc)
2. qemu-binfmt (register arm64, arm)
3. buildx --push multi-platform (amd64, arm64, arm/v7) → TAG-multi
4. docker pull --platform linux/amd64 IMAGE:TAG-multi; docker tag → IMAGE:TAG-amd64
5. docker pull --platform linux/arm64 IMAGE:TAG-multi; docker tag → IMAGE:TAG-arm64
6. docker pull --platform linux/arm/v7 IMAGE:TAG-multi; docker tag → IMAGE:TAG-armv7

cloudbuild.json `images:` declares all three tagged URIs.
`requestedVerifyOption: VERIFIED`.

**What this tests:** Whether `--platform` flag on `docker pull` resolves the
correct platform from a multi-platform manifest, and whether `images:` can
push foreign-arch images from the local daemon.

**Risk:** `--platform` flag requires Experimental: true (confirmed available).
But the resulting foreign-arch image in the local store may confuse `images:` push.

## Variant C: Multiple single-platform --load builds (DOCUMENT ONLY)

**DO NOT TREAT AS A VIABLE PRODUCTION PATH.** Run only if A/B/D all fail,
and only to document what CB does — not as a candidate architecture.

**Why demoted:** With Variants A/B/D, the provenance recipe for every platform
image records the FULL multi-platform `buildx --push` step. A verifier can
confirm all platforms were built together in one invocation (same
buildInvocationId). With Variant C, each platform is a SEPARATE build — there
is NO single build invocation tying them together. A verifier cannot confirm
the images share the same source commit. This is a fundamentally different
trust model, not just a lesser version of A/B/D.

Steps (if needed for documentation):
1. create-dockerfile (inline heredoc)
2. qemu-binfmt (register arm64, arm)
3. buildx build --load --platform=linux/amd64 --tag IMAGE:TAG-amd64 .
4. buildx build --load --platform=linux/arm64 --tag IMAGE:TAG-arm64 .
5. buildx build --load --platform=linux/arm/v7 --tag IMAGE:TAG-armv7 .

## Execution Order

1. Run Variant B first (preferred — digest-based, most robust)
2. If B fails on manifest inspect: try Variant D (imagetools fallback)
3. If B/D fail on digest pull: try Variant A (--platform flag)
4. Run Variant C only if A/B/D all fail — document failure mode only
5. Document all results regardless of success/failure

## Downstream Questions (if experiment succeeds)

- Does `docker manifest create` + `docker manifest push` reconstruct a
  multi-platform manifest from individually-tagged images?
- Does the reconstructed manifest get provenance? (Probably not — but do
  the per-platform images retain theirs?)
- Is per-platform tagging acceptable for production consumers?

## Verification (per variant)

For each platform image, verify via:
```
gcloud artifacts docker images describe \
  IMAGE:TAG-<platform> --show-provenance --format=yaml
```

Check:
- `slsa_build_level: 3`
- Provenance `subject.digest` matches the platform-specific image digest
- `metadata.buildInvocationId` is IDENTICAL across all platform images
  (Variants A/B/D only — proves single-build origin)
- Provenance `recipe.arguments.steps` contains the buildx step with
  `--platform=linux/amd64,linux/arm64,linux/arm/v7` (proves full
  multi-platform build is recorded, not just a single-platform build)
- Provenance signatures use same keyids as Experiment 3:
  - v0.1: `provenanceSigner/cryptoKeyVersions/1` (global) and
    `builtByGCB/cryptoKeyVersions/1` (us-central1)
  - v1: `google-hosted-worker/cryptoKeyVersions/1` (global)
- `resolvedDependencies` lists same builder image digest
  (`sha256:efdbd755...`)
- No 401/403 errors
- Build completes under VERIFIED constraint

For Variant C specifically (if run):
- `buildInvocationId` will DIFFER per platform (expected — document this)
- Each recipe records only that platform's build step (expected limitation)
- This confirms Variant C is NOT viable for same-source verification

## Key Files

- `Memos/memo-20260305-provenance-architecture-gap.md` — prior experiment evidence
- `Memos/experiments/cloudbuild-test-provenance.json` — Experiment 3 config (template)

## Acceptance Criteria

- At least one variant produces SLSA Level 3 on ALL platform sub-images
  (amd64, arm64, arm/v7)
- For Variants A/B/D: same buildInvocationId across all platform provenance
  records, AND provenance predicate contains full multi-platform build steps,
  AND signing keys match Experiment 3 trust chain
- OR: documented failure modes for all variants with exact error messages,
  toolchain versions, and build IDs — same evidence standard as prior experiments
- Results added to provenance memo with full detail (build IDs, digests,
  provenance YAML extracts, log evidence)

**[260305-1238] rough**

Test whether per-platform pullback can produce SLSA v1.0 provenance on all
sub-images of a multi-platform build.

## Context

Single-arch SLSA is proven (₢AlAAK, ₢AlAAL experiments on 2026-03-05).
The structural constraint: CB's `images:` field pushes from Docker's local
daemon, which is single-platform only. Multi-platform manifests cannot exist
in the local daemon.

**Core question:** Can we build multi-platform via `buildx --push`, then pull
back EACH platform individually into the local daemon, and get SLSA provenance
on each platform's digest — all within a single build invocation?

## What We Know (from prior experiments)

**Experiment 3 (build 48b818ed, 2026-03-05):**
- `docker pull` of a multi-platform manifest on amd64 worker pulls amd64 only
- Pulled image enters local daemon, `images:` pushes it, SLSA Level 3 generated
- Tag is overwritten (multi-platform manifest replaced by single-platform)
- Provenance attaches to the pulled-back single-platform digest
- Provenance predicate records ALL build steps (including multi-platform buildx)

**Confirmed toolchain (must use same builder image for reproducibility):**
- Docker Engine 20.10.24, API 1.41, Experimental: true (client)
- docker buildx v0.23.0 (28c90ea)
- BuildKit moby/buildkit:buildx-stable-1
- Builder image: `gcr.io/cloud-builders/docker@sha256:efdbd755476e7e5eb1077ed6e4bf691f87b38fbded575e9b825f9480374f8f4b`
- binfmt: `docker.io/tonistiigi/binfmt@sha256:8db0f28060565399642110b798c6c35efcac7c5b3b48c56d36503d3b4d8f93c8`

**ADC pre-population:** CB pre-populates `/builder/home/.docker/config.json`
with oauth2accesstoken for all GAR regions. No docker login step needed.
(Experiment 2, build 4de1467a.)

## Key Unknowns

1. Does `docker pull --platform linux/arm64` work on an amd64 CB worker?
   The `--platform` flag on `docker pull` is available since Docker 17.x
   experimental, GA in 20.10. Our CB worker has API 1.41 and Experimental: true.
   The flag should be accepted. **Primary risk:** Docker may store the
   foreign-arch image but the behavior of `images:` pushing a foreign-arch
   image from the local daemon is untested.

2. Does `docker manifest inspect` work on CB workers? Docker 20.10.24 with
   Experimental: true should support it. This enables digest-based pulls
   which are more reliable than `--platform` flag resolution.

3. Can `images:` declare multiple image URIs and generate provenance on each?
   The `images:` field accepts a list. Each declared image should get its own
   provenance occurrence. Untested with multiple entries.

## Experiment Method

Use `gcloud builds submit --no-source` on demo1025 (same as prior experiments).
Use the proven inline-Dockerfile-via-step pattern from build 48b818ed:
step 0 writes busybox Dockerfile to /workspace via heredoc. Same private pool
and builder images.

## Variant A: Per-platform pull with --platform flag

Steps:
1. create-dockerfile (inline heredoc to /workspace/Dockerfile)
2. qemu-binfmt (register arm64, arm)
3. buildx --push multi-platform (amd64, arm64, arm/v7) → TAG-multi
4. docker pull --platform linux/amd64 IMAGE:TAG-multi; docker tag → IMAGE:TAG-amd64
5. docker pull --platform linux/arm64 IMAGE:TAG-multi; docker tag → IMAGE:TAG-arm64
6. docker pull --platform linux/arm/v7 IMAGE:TAG-multi; docker tag → IMAGE:TAG-armv7

cloudbuild.json `images:` declares all three tagged URIs.
`requestedVerifyOption: VERIFIED`.

**What this tests:** Whether `--platform` flag on `docker pull` resolves the
correct platform from a multi-platform manifest, and whether `images:` can
push foreign-arch images from the local daemon.

**Risk:** `--platform` flag may not work correctly for foreign architectures
on Docker 20.10.24. The flag is accepted but the resulting image in the local
store may confuse `images:` push.

**Success:** Each platform image gets SLSA Level 3 with same buildInvocationId.
**Failure modes:** --platform flag rejected, foreign-arch image not storable,
`images:` push fails for foreign-arch, VERIFIED fails.

## Variant B: Manifest inspect + digest-based pull (PREFERRED)

This is the most robust approach: extract per-platform digests from the
manifest list, then pull each by exact digest. No platform resolution ambiguity.

Steps:
1. create-dockerfile (inline heredoc)
2. qemu-binfmt (register arm64, arm)
3. buildx --push multi-platform → TAG-multi
4. docker manifest inspect IMAGE:TAG-multi (requires Experimental: true, confirmed available)
   Parse output to extract per-platform digests (jq or grep)
5. docker pull IMAGE@sha256:<amd64-digest>; docker tag → IMAGE:TAG-amd64
6. docker pull IMAGE@sha256:<arm64-digest>; docker tag → IMAGE:TAG-arm64
7. docker pull IMAGE@sha256:<armv7-digest>; docker tag → IMAGE:TAG-armv7

cloudbuild.json `images:` declares all three tagged URIs.
`requestedVerifyOption: VERIFIED`.

**Why preferred:** Pulling by digest is unambiguous. Docker doesn't need to
resolve platform matching — the digest identifies exactly one manifest.
`docker manifest inspect` is confirmed available (Experimental: true in our
CB worker). If this works, it's the production path.

**Risk:** `docker manifest inspect` might not be available despite Experimental
flag, or might not parse cleanly. Pulling by foreign-arch digest into local
daemon might fail. Fallback: Variant A.

## Variant C: Multiple single-platform --load builds

Instead of --push + pullback, build each platform separately with --load:

Steps:
1. create-dockerfile (inline heredoc)
2. qemu-binfmt (register arm64, arm)
3. buildx build --load --platform=linux/amd64 --tag IMAGE:TAG-amd64 .
4. buildx build --load --platform=linux/arm64 --tag IMAGE:TAG-arm64 .
5. buildx build --load --platform=linux/arm/v7 --tag IMAGE:TAG-armv7 .

cloudbuild.json `images:` declares all three tagged URIs.
`requestedVerifyOption: VERIFIED`.

**SIGNIFICANT DOWNSIDE: Provenance chain fragmentation.** With Variants A/B,
the provenance recipe for every platform image records the FULL multi-platform
`buildx --push --platform=linux/amd64,linux/arm64,linux/arm/v7` step. A
verifier can confirm all platforms were built together in one invocation
(same buildInvocationId). With Variant C, each platform has its own build
steps in the recipe, and there is NO single build invocation tying them
together. A verifier cannot confirm the images share the same source commit.
This is a **major trade-off** — prefer Variants A/B if either works.

**Upside:** Simplest mechanism, no --push+pullback complexity, no manifest
inspection. Each --load puts a single-platform image directly in the daemon.

**Additional downside:** Three separate Dockerfile builds (slower). No
multi-platform manifest exists — would need `docker manifest create` to
reconstruct one, and that reconstructed manifest would NOT have provenance.

## Variant D: buildx imagetools inspect (fallback for manifest inspect)

If `docker manifest inspect` fails in Variant B, try `docker buildx
imagetools inspect --raw IMAGE:TAG-multi | jq` to extract per-platform
digests. This uses buildx's registry inspection which doesn't require
the experimental Docker CLI manifest commands. Otherwise identical to
Variant B.

## Execution Order

1. Run Variant B first (preferred — most robust)
2. If B fails on manifest inspect: try Variant D (imagetools fallback)
3. If B/D fail on digest pull: try Variant A (--platform flag)
4. Run Variant C only if A/B/D all fail on foreign-arch pull
5. Document all results regardless of success/failure

## Downstream Questions (if experiment succeeds)

- Does `docker manifest create` + `docker manifest push` reconstruct a
  multi-platform manifest from individually-tagged images?
- Does the reconstructed manifest get provenance? (Probably not — but do
  the per-platform images retain theirs?)
- Is per-platform tagging acceptable for production consumers?
- Should we accept TAG-multi (multi-platform, no provenance) + TAG-amd64 /
  TAG-arm64 / TAG-armv7 (single-platform, with provenance) as the shipping
  format?

## Verification (per variant)

For each platform image, verify via:
```
gcloud artifacts docker images describe \
  IMAGE:TAG-<platform> --show-provenance --format=yaml
```

Check:
- `slsa_build_level: 3`
- Provenance `subject.digest` matches the platform-specific image digest
- `metadata.buildInvocationId` is IDENTICAL across all platform images
  (Variants A/B/D only — proves single-build origin)
- Provenance `recipe.arguments.steps` contains the buildx step with
  `--platform=linux/amd64,linux/arm64,linux/arm/v7` (proves full
  multi-platform build is recorded, not just a single-platform build)
- No 401/403 errors
- Build completes under VERIFIED constraint

For Variant C specifically:
- `buildInvocationId` will DIFFER per platform (expected — document this)
- Each recipe records only that platform's build step (expected limitation)

## Key Files

- `Memos/memo-20260305-provenance-architecture-gap.md` — prior experiment evidence
- `cloudbuild-test-provenance.json` — Experiment 3 config (template)

## Acceptance Criteria

- At least one variant produces SLSA Level 3 on ALL platform sub-images
  (amd64, arm64, arm/v7)
- For Variants A/B/D: same buildInvocationId across all platform provenance
  records, AND provenance predicate contains full multi-platform build steps
- OR: documented failure modes for all variants with exact error messages,
  toolchain versions, and build IDs — same evidence standard as prior experiments
- Results added to provenance memo with full detail (build IDs, digests,
  provenance YAML extracts, log evidence)

**[260305-1233] rough**

Test whether per-platform pullback can produce SLSA v1.0 provenance on all
sub-images of a multi-platform build.

## Context

Single-arch SLSA is proven (₢AlAAK, ₢AlAAL experiments). The structural
constraint is: CB's `images:` field pushes from Docker's local daemon, which
is single-platform only. Multi-platform manifests cannot exist in the local
daemon.

**Core question:** Can we build multi-platform via `buildx --push`, then pull
back EACH platform individually into the local daemon, and get SLSA provenance
on each platform's digest?

## What We Know

From Experiment 3 (build 48b818ed):
- `docker pull` of a multi-platform manifest on amd64 worker pulls amd64 only
- Pulled image enters local daemon, `images:` pushes it, SLSA Level 3 generated
- Tag is overwritten (multi-platform manifest replaced by single-platform)
- Provenance attaches to the pulled-back single-platform digest

**Unknown:**
- Does `docker pull --platform linux/arm64` work on an amd64 CB worker?
- If so, does the arm64 image enter the local daemon correctly?
- Can `images:` push a foreign-arch image from the local daemon?
- Can we tag each platform distinctly and declare ALL in `images:`?
- Does CB generate provenance on each declared image?

## Experiment Design

Use `gcloud builds submit --no-source` on demo1025 (same method as prior
experiments). Busybox image, same toolchain versions.

### Variant A: Per-platform pull with --platform flag

```
1. buildx --push multi-platform (amd64, arm64, arm/v7) → TAG-multi
2. docker pull --platform linux/amd64 IMAGE:TAG-multi
3. docker tag → IMAGE:TAG-amd64
4. docker pull --platform linux/arm64 IMAGE:TAG-multi
5. docker tag → IMAGE:TAG-arm64
6. docker pull --platform linux/arm/v7 IMAGE:TAG-multi
7. docker tag → IMAGE:TAG-armv7
```

cloudbuild.json `images:` declares all three tagged URIs.
`requestedVerifyOption: VERIFIED`.

**Success:** Each platform image gets SLSA Level 3.
**Failure modes:** --platform flag rejected, foreign-arch pull fails,
`images:` push fails for foreign-arch image, VERIFIED fails.

### Variant B: Per-platform pull via digest

If --platform flag doesn't work on `docker pull`, try pulling by
platform-specific digest (extracted from the manifest list):

```
1. buildx --push multi-platform → TAG-multi
2. docker manifest inspect IMAGE:TAG-multi → extract per-platform digests
3. docker pull IMAGE@sha256:<amd64-digest>
4. docker tag → IMAGE:TAG-amd64
5. (repeat for arm64, arm/v7)
```

### Variant C: Single build, multiple --load runs

Instead of --push + pullback, try multiple single-platform builds with --load:

```
1. buildx build --load --platform=linux/amd64 --tag IMAGE:TAG-amd64 .
2. buildx build --load --platform=linux/arm64 --tag IMAGE:TAG-arm64 .
3. buildx build --load --platform=linux/arm/v7 --tag IMAGE:TAG-armv7 .
```

All three in local daemon. `images:` declares all three.
Downside: three separate builds (slower), no multi-platform manifest.
Upside: no --push+pullback complexity.

## Downstream Questions (if experiment succeeds)

- How to reconstruct a multi-platform manifest from individually-tagged images?
  (`docker manifest create` + `docker manifest push`?)
- Does the reconstructed manifest get provenance? (Probably not — but do the
  per-platform images retain theirs?)
- Is the per-platform tagging scheme acceptable for consumers?
- What does `docker pull IMAGE:TAG-multi` resolve to after reconstruction?

## Verification (per variant)

- Each platform image has `slsa_build_level: 3`
- Provenance predicate records the correct platform
- No 401/403 errors
- Build completes under VERIFIED constraint

## Key Files

- `Memos/memo-20260305-provenance-architecture-gap.md` — prior experiment evidence
- `cloudbuild-test-provenance.json` — Experiment 3 config (template)

## Acceptance Criteria

- At least one variant produces SLSA Level 3 on all platform sub-images
- OR: documented failure modes for all variants with clear error evidence
- Results added to provenance memo with same level of detail as prior experiments

### spec-multiplatform-provenance (₢AlAAR) [complete]

**[260305-1440] complete**

Update RBS0 specification to document the validated multi-platform SLSA v1.0
provenance architecture. Narrow RBSOB update to reflect experiment validation
without claiming full supersession (that comes after ₢AlAAT e2e proof).

## Prerequisites

- ₢AlAAQ (experiment-multiplatform-slsa-provenance) complete — confirmed facts only

## Scope

### RBS0-SpecTop.adoc

- Update `rbtgr_provenance` definition: replace "not yet supported" with the
  validated multi-platform architecture
- Document: `buildx --push` → per-platform pullback via `docker pull --platform`
  → per-platform tags in `images:` → SLSA Level 3 on each → `imagetools create`
  for manifest list reassembly
- Reference experiment build IDs: `b3fd60c7` (provenance), `8cd7b713` (reassembly)
- Note: combined manifest list has `slsa_build_level: unknown` (expected);
  per-platform images retain Level 3 through reassembly
- Document the multi-platform pipeline step sequence (target shape for ₢AlAAS):
  1. derive-tag-base
  2. qemu-binfmt
  3. buildx --push (all platforms) → intermediate multi tag
  4. per-platform pullback (docker pull --platform → docker tag)
  5. sbom (strategy TBD — per-platform or combined)
  6. assemble-metadata
  7. build-and-push-metadata
  + `images:` lists all per-platform tags → CB push + SLSA provenance
  + post-build: `imagetools create` reassembles multi-platform manifest

### RBSOB-oci_layout_bridge.adoc

- Update superseded notice to acknowledge multi-platform path is experimentally
  validated (not just single-arch)
- Do NOT claim full supersession yet — that requires ₢AlAAT production e2e proof
- Language: "experimentally validated for all vessel types" not "superseded for all"

### Paddock update

- Record spec changes and pipeline shape

## Acceptance Criteria

- RBS0 multi-platform provenance states only confirmed, tested facts
- RBS0 includes target multi-platform pipeline step sequence
- No speculative language; experiment-validated vs production-proven distinction clear
- RBSOB reflects validated path without overclaiming

**[260305-1432] rough**

Update RBS0 specification to document the validated multi-platform SLSA v1.0
provenance architecture. Narrow RBSOB update to reflect experiment validation
without claiming full supersession (that comes after ₢AlAAT e2e proof).

## Prerequisites

- ₢AlAAQ (experiment-multiplatform-slsa-provenance) complete — confirmed facts only

## Scope

### RBS0-SpecTop.adoc

- Update `rbtgr_provenance` definition: replace "not yet supported" with the
  validated multi-platform architecture
- Document: `buildx --push` → per-platform pullback via `docker pull --platform`
  → per-platform tags in `images:` → SLSA Level 3 on each → `imagetools create`
  for manifest list reassembly
- Reference experiment build IDs: `b3fd60c7` (provenance), `8cd7b713` (reassembly)
- Note: combined manifest list has `slsa_build_level: unknown` (expected);
  per-platform images retain Level 3 through reassembly
- Document the multi-platform pipeline step sequence (target shape for ₢AlAAS):
  1. derive-tag-base
  2. qemu-binfmt
  3. buildx --push (all platforms) → intermediate multi tag
  4. per-platform pullback (docker pull --platform → docker tag)
  5. sbom (strategy TBD — per-platform or combined)
  6. assemble-metadata
  7. build-and-push-metadata
  + `images:` lists all per-platform tags → CB push + SLSA provenance
  + post-build: `imagetools create` reassembles multi-platform manifest

### RBSOB-oci_layout_bridge.adoc

- Update superseded notice to acknowledge multi-platform path is experimentally
  validated (not just single-arch)
- Do NOT claim full supersession yet — that requires ₢AlAAT production e2e proof
- Language: "experimentally validated for all vessel types" not "superseded for all"

### Paddock update

- Record spec changes and pipeline shape

## Acceptance Criteria

- RBS0 multi-platform provenance states only confirmed, tested facts
- RBS0 includes target multi-platform pipeline step sequence
- No speculative language; experiment-validated vs production-proven distinction clear
- RBSOB reflects validated path without overclaiming

**[260305-1429] rough**

Update RBS0 and RBSOB specifications to document the validated multi-platform
SLSA v1.0 provenance architecture.

## Prerequisites

- ₢AlAAQ (experiment-multiplatform-slsa-provenance) complete — confirmed facts only

## Scope

### RBS0-SpecTop.adoc

- Update `rbtgr_provenance` definition to add multi-platform path as validated
  (currently says "not yet supported")
- Document the architecture: `buildx --push` → per-platform pullback via
  `docker pull --platform` → per-platform tags in `images:` → SLSA Level 3
  on each → `imagetools create` for manifest list reassembly
- Reference experiment build IDs: `b3fd60c7` (provenance), `8cd7b713` (reassembly)
- Note: combined manifest list has `slsa_build_level: unknown` (expected);
  per-platform images retain Level 3 through reassembly

### RBSOB-oci_layout_bridge.adoc

- Update superseded notice: now superseded for ALL vessels, not just single-arch
- Multi-platform path no longer needs the bridge either

### Paddock update

- Record spec changes

## Acceptance Criteria

- RBS0 multi-platform provenance states only confirmed, tested facts
- No speculative language
- RBSOB clearly marks full supersession

### spec-multiplatform-metadata (₢AlAAU) [complete]

**[260305-1522] complete**

Update RBS0 specification to document the resolved multi-platform metadata
architecture decisions from ₢AlAAS design discussions.

## Prerequisites

- ₢AlAAR (spec-multiplatform-provenance) complete — base architecture documented

## Context

₢AlAAR documented the multi-platform provenance pipeline architecture in RBS0
(buildx --push, pullback, imagetools create, experiment build IDs). However,
several decisions were resolved AFTER ₢AlAAR was wrapped:

- Per-platform SBOMs in multi-platform -about container (Decision 2)
- FROM scratch + TARGETARCH/TARGETVARIANT metadata container build (Decision 5)
- Per-platform build_info.json with SLSA summary fields (Decision 6)
- The -about container becoming multi-platform (mirroring -image)
- Tag scheme for -about (same {TAG_BASE}-about tag, now multi-platform)

These are recorded in the paddock and ₢AlAAS docket but not yet in RBS0.

## Scope

### RBS0-SpecTop.adoc

Update or add definitions for:

- rbtga_ark_about: update to reflect multi-platform -about container with
  per-platform content (SBOM + build_info per architecture)
- rbtgi_metadata: update to note per-platform SBOMs and SLSA summary fields
  in build_info.json
- Document build_info.json fields: platform string, image digest, QEMU used,
  slsa_build_level, build_invocation_id, provenance_predicate_types,
  provenance_builder_id (shared fields: build ID, timestamps, git commit,
  vessel name)
- Document the FROM scratch + TARGETARCH/TARGETVARIANT build pattern for
  metadata containers
- Note: -about uses buildx --push (not docker build + docker push) for
  multi-platform vessels; reuses the buildx builder instance from the
  main image build

### What NOT to change

- The rbtgr_provenance definition (already updated by ₢AlAAR)
- The multi-platform pipeline step sequence (already updated post-₢AlAAR)
- Single-arch vessel behavior documentation

## Acceptance Criteria

- RBS0 metadata/about definitions reflect multi-platform architecture
- build_info.json field inventory documented
- FROM scratch + TARGETARCH pattern documented
- No speculative language — only confirmed design decisions
- Consistent with paddock Decisions 2-6 section

**[260305-1518] rough**

Update RBS0 specification to document the resolved multi-platform metadata
architecture decisions from ₢AlAAS design discussions.

## Prerequisites

- ₢AlAAR (spec-multiplatform-provenance) complete — base architecture documented

## Context

₢AlAAR documented the multi-platform provenance pipeline architecture in RBS0
(buildx --push, pullback, imagetools create, experiment build IDs). However,
several decisions were resolved AFTER ₢AlAAR was wrapped:

- Per-platform SBOMs in multi-platform -about container (Decision 2)
- FROM scratch + TARGETARCH/TARGETVARIANT metadata container build (Decision 5)
- Per-platform build_info.json with SLSA summary fields (Decision 6)
- The -about container becoming multi-platform (mirroring -image)
- Tag scheme for -about (same {TAG_BASE}-about tag, now multi-platform)

These are recorded in the paddock and ₢AlAAS docket but not yet in RBS0.

## Scope

### RBS0-SpecTop.adoc

Update or add definitions for:

- rbtga_ark_about: update to reflect multi-platform -about container with
  per-platform content (SBOM + build_info per architecture)
- rbtgi_metadata: update to note per-platform SBOMs and SLSA summary fields
  in build_info.json
- Document build_info.json fields: platform string, image digest, QEMU used,
  slsa_build_level, build_invocation_id, provenance_predicate_types,
  provenance_builder_id (shared fields: build ID, timestamps, git commit,
  vessel name)
- Document the FROM scratch + TARGETARCH/TARGETVARIANT build pattern for
  metadata containers
- Note: -about uses buildx --push (not docker build + docker push) for
  multi-platform vessels; reuses the buildx builder instance from the
  main image build

### What NOT to change

- The rbtgr_provenance definition (already updated by ₢AlAAR)
- The multi-platform pipeline step sequence (already updated post-₢AlAAR)
- Single-arch vessel behavior documentation

## Acceptance Criteria

- RBS0 metadata/about definitions reflect multi-platform architecture
- build_info.json field inventory documented
- FROM scratch + TARGETARCH pattern documented
- No speculative language — only confirmed design decisions
- Consistent with paddock Decisions 2-6 section

### stitch-multiplatform-provenance (₢AlAAS) [complete]

**[260305-1535] complete**

Implement multi-platform SLSA provenance code path in the stitch function
and step scripts.

## Prerequisites

- ₢AlAAR (spec-multiplatform-provenance) complete — pipeline shape defined

## Resolved Decisions

### 1. imagetools create placement — single-build (option G)

Experiment 6 (build `6661d0cd`, 33s) proved `imagetools create` runs within
the same CB build. `docker push` per-platform tags mid-build → `imagetools
create` assembles manifest list → `images:` re-pushes same per-platform tags
(idempotent, same digest) + generates SLSA provenance. Zero new dependencies.
No post-build step, no local environment in the provenance chain.

Config: `Memos/experiments/cloudbuild-test-single-build-reassembly.json`

### 2. Syft/SBOM strategy — per-platform SBOMs in multi-platform -about

Each per-platform image gets its own Syft scan → per-platform SBOM. Scans
run sequentially (no step parallelization — simplicity over speed). The
metadata container (`-about`) becomes a multi-platform image mirroring the
`-image` structure. Retrievers pull `-about` for their platform and get the
SBOM for exactly their architecture. No weakening of the per-image
documentation premise.

### 3. Tag scheme — platform-transparent consumer tags

Consumer-facing tags are identical to single-arch — platform is invisible
to Directors and Retrievers. Per-platform tags are internal build plumbing.

| Tag | Purpose |
|-----|---------|
| `{INSCRIBE_TS}-multi` | Intermediate `buildx --push` target |
| `{INSCRIBE_TS}{ARK_SUFFIX}-amd64` | Per-platform (`images:` field, SLSA) |
| `{INSCRIBE_TS}{ARK_SUFFIX}` | Consumer-facing (reassembled manifest list) |
| `{TAG_BASE}-about` | Metadata container (multi-platform) |

Platform suffix derivation: `linux/amd64` → `-amd64`, `linux/arm64` → `-arm64`,
`linux/arm/v7` → `-armv7`. Computed at inscribe time from `RBRV_CONJURE_PLATFORMS`.
All tags constructable from CB substitutions.

### 4. Intermediate tag lifecycle — keep

The `-multi` tag is the intermediate source for pullback. No cleanup needed.

### 5. Multi-platform `-about` build mechanics — single buildx invocation

The `-about` container uses `FROM scratch` + buildx automatic `TARGETARCH`/
`TARGETVARIANT` args to select per-platform files in one `buildx --push`
invocation. No QEMU needed (scratch has no executables — just file copies
with platform annotations).

```dockerfile
FROM scratch
ARG TARGETARCH
ARG TARGETVARIANT
COPY sbom-${TARGETARCH}${TARGETVARIANT}.json /sbom.json
COPY build_info-${TARGETARCH}${TARGETVARIANT}.json /build_info.json
```

Buildx auto-args resolve:
- `linux/amd64` → `sbom-amd64.json`, `build_info-amd64.json`
- `linux/arm64` → `sbom-arm64.json`, `build_info-arm64.json`
- `linux/arm/v7` → `sbom-armv7.json`, `build_info-armv7.json`

NOTE: This changes the `-about` build from `docker build` + `docker push`
(single-arch) to `buildx --push` (multi-platform). Requires the buildx
builder instance created earlier in the pipeline.

### 6. build_info.json — per-platform with SLSA summary

Each platform's `build_info.json` describes that specific platform image.
Per-platform fields: platform string, image digest, QEMU used (boolean).
Shared fields: build ID, timestamps, git commit, vessel name.

SLSA provenance summary fields added:
- `slsa_build_level`: 3
- `build_invocation_id`: CB build ID (shared — proves single-build origin)
- `provenance_predicate_types`: v0.1 + v1
- `provenance_builder_id`: GoogleHostedWorker URL

Consumers can check SLSA facts from build_info.json without querying
Container Analysis API.

## Scope

### rbf_Foundry.sh — stitch function

- Detect platform count from RBRV_CONJURE_PLATFORMS
- Single-platform (1 platform): existing `--load` path (unchanged)
- Multi-platform (2+ platforms): new `--push` + pullback + reassembly path:
  1. `buildx --push` with all platforms → intermediate `-multi` tag
  2. Per-platform `docker pull --platform <PLAT>` → `docker tag` per-platform
  3. `docker push` each per-platform tag (pre-push for imagetools)
  4. Syft scan each per-platform image sequentially (`docker:` transport)
  5. Generate per-platform `build_info-{arch}{variant}.json` with SLSA summary
  6. `buildx --push` multi-platform `-about` container (`FROM scratch` +
     TARGETARCH/TARGETVARIANT selects per-platform files)
  7. `imagetools create` assembles consumer-facing `-image` manifest list
  8. `images:` field lists all per-platform tags (SLSA provenance)
  9. `requestedVerifyOption: VERIFIED`
- Remove the single-arch gate that currently rejects multi-platform vessels
  at inscribe time
- `$${}` escaping for shell variables in generated cloudbuild.json

### Step scripts (Tools/rbw/rbgjb/)

- Update or create step scripts for: pullback, push, syft-per-platform,
  build-info-per-platform, about-buildx, and imagetools sequences

### cloudbuild.json generation

- `images:` field must list all per-platform tagged URIs
- Private pool + VERIFIED options (same as single-arch)

## Acceptance Criteria

- `rbf_stitch` generates valid cloudbuild.json for both single and multi-platform
- Multi-platform cloudbuild.json includes pullback, push, syft, about, and
  imagetools steps
- Multi-platform `images:` field lists all per-platform tags
- Multi-platform `-about` container has per-platform SBOMs and build_info
- build_info.json includes SLSA provenance summary fields
- Consumer-facing tag is platform-transparent (same as single-arch)
- No changes to single-platform vessel behavior
- BCG compliance for all new bash code

**[260305-1516] bridled**

Implement multi-platform SLSA provenance code path in the stitch function
and step scripts.

## Prerequisites

- ₢AlAAR (spec-multiplatform-provenance) complete — pipeline shape defined

## Resolved Decisions

### 1. imagetools create placement — single-build (option G)

Experiment 6 (build `6661d0cd`, 33s) proved `imagetools create` runs within
the same CB build. `docker push` per-platform tags mid-build → `imagetools
create` assembles manifest list → `images:` re-pushes same per-platform tags
(idempotent, same digest) + generates SLSA provenance. Zero new dependencies.
No post-build step, no local environment in the provenance chain.

Config: `Memos/experiments/cloudbuild-test-single-build-reassembly.json`

### 2. Syft/SBOM strategy — per-platform SBOMs in multi-platform -about

Each per-platform image gets its own Syft scan → per-platform SBOM. Scans
run sequentially (no step parallelization — simplicity over speed). The
metadata container (`-about`) becomes a multi-platform image mirroring the
`-image` structure. Retrievers pull `-about` for their platform and get the
SBOM for exactly their architecture. No weakening of the per-image
documentation premise.

### 3. Tag scheme — platform-transparent consumer tags

Consumer-facing tags are identical to single-arch — platform is invisible
to Directors and Retrievers. Per-platform tags are internal build plumbing.

| Tag | Purpose |
|-----|---------|
| `{INSCRIBE_TS}-multi` | Intermediate `buildx --push` target |
| `{INSCRIBE_TS}{ARK_SUFFIX}-amd64` | Per-platform (`images:` field, SLSA) |
| `{INSCRIBE_TS}{ARK_SUFFIX}` | Consumer-facing (reassembled manifest list) |
| `{TAG_BASE}-about` | Metadata container (multi-platform) |

Platform suffix derivation: `linux/amd64` → `-amd64`, `linux/arm64` → `-arm64`,
`linux/arm/v7` → `-armv7`. Computed at inscribe time from `RBRV_CONJURE_PLATFORMS`.
All tags constructable from CB substitutions.

### 4. Intermediate tag lifecycle — keep

The `-multi` tag is the intermediate source for pullback. No cleanup needed.

### 5. Multi-platform `-about` build mechanics — single buildx invocation

The `-about` container uses `FROM scratch` + buildx automatic `TARGETARCH`/
`TARGETVARIANT` args to select per-platform files in one `buildx --push`
invocation. No QEMU needed (scratch has no executables — just file copies
with platform annotations).

```dockerfile
FROM scratch
ARG TARGETARCH
ARG TARGETVARIANT
COPY sbom-${TARGETARCH}${TARGETVARIANT}.json /sbom.json
COPY build_info-${TARGETARCH}${TARGETVARIANT}.json /build_info.json
```

Buildx auto-args resolve:
- `linux/amd64` → `sbom-amd64.json`, `build_info-amd64.json`
- `linux/arm64` → `sbom-arm64.json`, `build_info-arm64.json`
- `linux/arm/v7` → `sbom-armv7.json`, `build_info-armv7.json`

NOTE: This changes the `-about` build from `docker build` + `docker push`
(single-arch) to `buildx --push` (multi-platform). Requires the buildx
builder instance created earlier in the pipeline.

### 6. build_info.json — per-platform with SLSA summary

Each platform's `build_info.json` describes that specific platform image.
Per-platform fields: platform string, image digest, QEMU used (boolean).
Shared fields: build ID, timestamps, git commit, vessel name.

SLSA provenance summary fields added:
- `slsa_build_level`: 3
- `build_invocation_id`: CB build ID (shared — proves single-build origin)
- `provenance_predicate_types`: v0.1 + v1
- `provenance_builder_id`: GoogleHostedWorker URL

Consumers can check SLSA facts from build_info.json without querying
Container Analysis API.

## Scope

### rbf_Foundry.sh — stitch function

- Detect platform count from RBRV_CONJURE_PLATFORMS
- Single-platform (1 platform): existing `--load` path (unchanged)
- Multi-platform (2+ platforms): new `--push` + pullback + reassembly path:
  1. `buildx --push` with all platforms → intermediate `-multi` tag
  2. Per-platform `docker pull --platform <PLAT>` → `docker tag` per-platform
  3. `docker push` each per-platform tag (pre-push for imagetools)
  4. Syft scan each per-platform image sequentially (`docker:` transport)
  5. Generate per-platform `build_info-{arch}{variant}.json` with SLSA summary
  6. `buildx --push` multi-platform `-about` container (`FROM scratch` +
     TARGETARCH/TARGETVARIANT selects per-platform files)
  7. `imagetools create` assembles consumer-facing `-image` manifest list
  8. `images:` field lists all per-platform tags (SLSA provenance)
  9. `requestedVerifyOption: VERIFIED`
- Remove the single-arch gate that currently rejects multi-platform vessels
  at inscribe time
- `$${}` escaping for shell variables in generated cloudbuild.json

### Step scripts (Tools/rbw/rbgjb/)

- Update or create step scripts for: pullback, push, syft-per-platform,
  build-info-per-platform, about-buildx, and imagetools sequences

### cloudbuild.json generation

- `images:` field must list all per-platform tagged URIs
- Private pool + VERIFIED options (same as single-arch)

## Acceptance Criteria

- `rbf_stitch` generates valid cloudbuild.json for both single and multi-platform
- Multi-platform cloudbuild.json includes pullback, push, syft, about, and
  imagetools steps
- Multi-platform `images:` field lists all per-platform tags
- Multi-platform `-about` container has per-platform SBOMs and build_info
- build_info.json includes SLSA provenance summary fields
- Consumer-facing tag is platform-transparent (same as single-arch)
- No changes to single-platform vessel behavior
- BCG compliance for all new bash code

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/rbw/rbf_Foundry.sh, Tools/rbw/rbgjb/ step scripts new and modified, Memos/experiments/cloudbuild-test-single-build-reassembly.json reference (10+ files) | Steps: 1. Read rbf_Foundry.sh stitch function and existing step scripts for single-arch pattern 2. Read experiment 6 config for validated pipeline structure 3. Add platform count detection in zrbf_stitch_build_json branching single vs multi-platform 4. Generate multi-platform cloudbuild.json steps: buildx --push, pullback, docker push, syft per-platform, build-info per-platform, about-buildx FROM scratch with TARGETARCH, imagetools create 5. Generate multi-platform images field listing all per-platform tags 6. Create step scripts for new steps following existing BCG patterns and escaping shell vars with double-dollar in cloudbuild.json 7. Remove the single-arch gate that rejects multi-platform vessels at inscribe time 8. Ensure single-platform path unchanged | Verify: bash -n Tools/rbw/rbf_Foundry.sh and inspect generated cloudbuild.json structure for a multi-platform vessel

**[260305-1508] rough**

Implement multi-platform SLSA provenance code path in the stitch function
and step scripts.

## Prerequisites

- ₢AlAAR (spec-multiplatform-provenance) complete — pipeline shape defined

## Resolved Decisions

### 1. imagetools create placement — single-build (option G)

Experiment 6 (build `6661d0cd`, 33s) proved `imagetools create` runs within
the same CB build. `docker push` per-platform tags mid-build → `imagetools
create` assembles manifest list → `images:` re-pushes same per-platform tags
(idempotent, same digest) + generates SLSA provenance. Zero new dependencies.
No post-build step, no local environment in the provenance chain.

Config: `Memos/experiments/cloudbuild-test-single-build-reassembly.json`

### 2. Syft/SBOM strategy — per-platform SBOMs in multi-platform -about

Each per-platform image gets its own Syft scan → per-platform SBOM. Scans
run sequentially (no step parallelization — simplicity over speed). The
metadata container (`-about`) becomes a multi-platform image mirroring the
`-image` structure. Retrievers pull `-about` for their platform and get the
SBOM for exactly their architecture. No weakening of the per-image
documentation premise.

### 3. Tag scheme — platform-transparent consumer tags

Consumer-facing tags are identical to single-arch — platform is invisible
to Directors and Retrievers. Per-platform tags are internal build plumbing.

| Tag | Purpose |
|-----|---------|
| `{INSCRIBE_TS}-multi` | Intermediate `buildx --push` target |
| `{INSCRIBE_TS}{ARK_SUFFIX}-amd64` | Per-platform (`images:` field, SLSA) |
| `{INSCRIBE_TS}{ARK_SUFFIX}` | Consumer-facing (reassembled manifest list) |
| `{TAG_BASE}-about` | Metadata container (multi-platform) |

Platform suffix derivation: `linux/amd64` → `-amd64`, `linux/arm64` → `-arm64`,
`linux/arm/v7` → `-armv7`. Computed at inscribe time from `RBRV_CONJURE_PLATFORMS`.
All tags constructable from CB substitutions.

### 4. Intermediate tag lifecycle — keep

The `-multi` tag is the intermediate source for pullback. No cleanup needed.

### 5. Multi-platform `-about` build mechanics — single buildx invocation

The `-about` container uses `FROM scratch` + buildx automatic `TARGETARCH`/
`TARGETVARIANT` args to select per-platform files in one `buildx --push`
invocation. No QEMU needed (scratch has no executables — just file copies
with platform annotations).

```dockerfile
FROM scratch
ARG TARGETARCH
ARG TARGETVARIANT
COPY sbom-${TARGETARCH}${TARGETVARIANT}.json /sbom.json
COPY build_info-${TARGETARCH}${TARGETVARIANT}.json /build_info.json
```

Buildx auto-args resolve:
- `linux/amd64` → `sbom-amd64.json`, `build_info-amd64.json`
- `linux/arm64` → `sbom-arm64.json`, `build_info-arm64.json`
- `linux/arm/v7` → `sbom-armv7.json`, `build_info-armv7.json`

NOTE: This changes the `-about` build from `docker build` + `docker push`
(single-arch) to `buildx --push` (multi-platform). Requires the buildx
builder instance created earlier in the pipeline.

### 6. build_info.json — per-platform with SLSA summary

Each platform's `build_info.json` describes that specific platform image.
Per-platform fields: platform string, image digest, QEMU used (boolean).
Shared fields: build ID, timestamps, git commit, vessel name.

SLSA provenance summary fields added:
- `slsa_build_level`: 3
- `build_invocation_id`: CB build ID (shared — proves single-build origin)
- `provenance_predicate_types`: v0.1 + v1
- `provenance_builder_id`: GoogleHostedWorker URL

Consumers can check SLSA facts from build_info.json without querying
Container Analysis API.

## Scope

### rbf_Foundry.sh — stitch function

- Detect platform count from RBRV_CONJURE_PLATFORMS
- Single-platform (1 platform): existing `--load` path (unchanged)
- Multi-platform (2+ platforms): new `--push` + pullback + reassembly path:
  1. `buildx --push` with all platforms → intermediate `-multi` tag
  2. Per-platform `docker pull --platform <PLAT>` → `docker tag` per-platform
  3. `docker push` each per-platform tag (pre-push for imagetools)
  4. Syft scan each per-platform image sequentially (`docker:` transport)
  5. Generate per-platform `build_info-{arch}{variant}.json` with SLSA summary
  6. `buildx --push` multi-platform `-about` container (`FROM scratch` +
     TARGETARCH/TARGETVARIANT selects per-platform files)
  7. `imagetools create` assembles consumer-facing `-image` manifest list
  8. `images:` field lists all per-platform tags (SLSA provenance)
  9. `requestedVerifyOption: VERIFIED`
- Remove the single-arch gate that currently rejects multi-platform vessels
  at inscribe time
- `$${}` escaping for shell variables in generated cloudbuild.json

### Step scripts (Tools/rbw/rbgjb/)

- Update or create step scripts for: pullback, push, syft-per-platform,
  build-info-per-platform, about-buildx, and imagetools sequences

### cloudbuild.json generation

- `images:` field must list all per-platform tagged URIs
- Private pool + VERIFIED options (same as single-arch)

## Acceptance Criteria

- `rbf_stitch` generates valid cloudbuild.json for both single and multi-platform
- Multi-platform cloudbuild.json includes pullback, push, syft, about, and
  imagetools steps
- Multi-platform `images:` field lists all per-platform tags
- Multi-platform `-about` container has per-platform SBOMs and build_info
- build_info.json includes SLSA provenance summary fields
- Consumer-facing tag is platform-transparent (same as single-arch)
- No changes to single-platform vessel behavior
- BCG compliance for all new bash code

**[260305-1501] rough**

Implement multi-platform SLSA provenance code path in the stitch function
and step scripts.

## Prerequisites

- ₢AlAAR (spec-multiplatform-provenance) complete — pipeline shape defined

## Resolved Decisions

### 1. imagetools create placement — single-build (option G)

Experiment 6 (build `6661d0cd`, 33s) proved `imagetools create` runs within
the same CB build. `docker push` per-platform tags mid-build → `imagetools
create` assembles manifest list → `images:` re-pushes same per-platform tags
(idempotent, same digest) + generates SLSA provenance. Zero new dependencies.
No post-build step, no local environment in the provenance chain.

Config: `Memos/experiments/cloudbuild-test-single-build-reassembly.json`

### 2. Syft/SBOM strategy — per-platform SBOMs in multi-platform -about

Each per-platform image gets its own Syft scan → per-platform SBOM. The
metadata container (`-about`) becomes a multi-platform image mirroring the
`-image` structure. Retrievers pull `-about` for their platform and get the
SBOM for exactly their architecture. No weakening of the per-image
documentation premise.

### 3. Tag scheme — platform-transparent consumer tags

Consumer-facing tags are identical to single-arch — platform is invisible
to Directors and Retrievers. Per-platform tags are internal build plumbing.

| Tag | Purpose |
|-----|---------|
| `{INSCRIBE_TS}-multi` | Intermediate `buildx --push` target |
| `{INSCRIBE_TS}{ARK_SUFFIX}-amd64` | Per-platform (`images:` field, SLSA) |
| `{INSCRIBE_TS}{ARK_SUFFIX}` | Consumer-facing (reassembled manifest list) |
| `{TAG_BASE}-about` | Metadata container (multi-platform) |

Platform suffix derivation: `linux/amd64` → `-amd64`, `linux/arm64` → `-arm64`,
`linux/arm/v7` → `-armv7`. Computed at inscribe time from `RBRV_CONJURE_PLATFORMS`.
All tags constructable from CB substitutions.

### 4. Intermediate tag lifecycle — keep

The `-multi` tag is the intermediate source for pullback. No cleanup needed.

## Scope

### rbf_Foundry.sh — stitch function

- Detect platform count from RBRV_CONJURE_PLATFORMS
- Single-platform (1 platform): existing `--load` path (unchanged)
- Multi-platform (2+ platforms): new `--push` + pullback + reassembly path:
  1. `buildx --push` with all platforms → intermediate `-multi` tag
  2. Per-platform `docker pull --platform <PLAT>` → `docker tag` per-platform
  3. `docker push` each per-platform tag (pre-push for imagetools)
  4. Syft scan each per-platform image from local daemon (`docker:` transport)
  5. `imagetools create` assembles consumer-facing manifest list
  6. `images:` field lists all per-platform tags (SLSA provenance)
  7. `requestedVerifyOption: VERIFIED`
- Remove the single-arch gate that currently rejects multi-platform vessels
  at inscribe time
- `$${}` escaping for shell variables in generated cloudbuild.json

### Step scripts (Tools/rbw/rbgjb/)

- Update or create step scripts for pullback, push, syft-per-platform,
  and imagetools sequences

### Metadata container build

- Build multi-platform `-about` image with per-platform SBOMs
- Each platform layer: `sbom.json` + `build_info.json`
- Pushed as multi-platform manifest (same pattern as `-image`)

### cloudbuild.json generation

- `images:` field must list all per-platform tagged URIs
- Private pool + VERIFIED options (same as single-arch)

## Acceptance Criteria

- `rbf_stitch` generates valid cloudbuild.json for both single and multi-platform
- Multi-platform cloudbuild.json includes pullback, push, syft, and imagetools steps
- Multi-platform `images:` field lists all per-platform tags
- Multi-platform `-about` container has per-platform SBOMs
- Consumer-facing tag is platform-transparent (same as single-arch)
- No changes to single-platform vessel behavior
- BCG compliance for all new bash code

**[260305-1453] rough**

Implement multi-platform SLSA provenance code path in the stitch function
and step scripts.

## Prerequisites

- ₢AlAAR (spec-multiplatform-provenance) complete — pipeline shape defined

## Resolved Decisions

### imagetools create placement — RESOLVED: single-build (option G)

Experiment 6 (build `6661d0cd`, 33s) proved `imagetools create` runs within
the same CB build. The `images:` re-push is idempotent (same content, same
digest), so `docker push` per-platform tags mid-build → `imagetools create`
→ `images:` re-push + SLSA provenance all work in one invocation.

Zero new dependencies. No post-build step, no Pub/Sub, no local environment
in the provenance chain. Config: `Memos/experiments/cloudbuild-test-single-build-reassembly.json`

### Intermediate tag lifecycle — RESOLVED: keep

The `-multi` tag is the intermediate source for pullback. No cleanup needed —
it's a registry artifact with no consumer impact. Avoids adding a delete step.

## Open Decisions (resolve during implementation)

### Syft/SBOM strategy

Single-arch pipeline scans the local daemon image (step 04, `docker:IMAGE_URI`
transport). For multi-platform, after pullback there are N platform images in
the local daemon. Options:

- Scan each per-platform image separately → N SBOMs
- Scan only the native-arch image (amd64) → 1 SBOM (platforms share layers)
- Scan the `-multi` registry tag via `docker://` transport

Must align with existing SBOM/metadata container format.

### Tag scheme

Must integrate with existing `_RBGY_INSCRIBE_TIMESTAMP` + `_RBGY_ARK_SUFFIX_IMAGE`
pattern. Proposed scheme:

- Multi-platform push: `{INSCRIBE_TS}-multi` (intermediate, registry only)
- Per-platform tags: `{INSCRIBE_TS}{ARK_SUFFIX}-amd64`, `...arm64`, `...armv7`
- Reassembled manifest: `{INSCRIBE_TS}{ARK_SUFFIX}` (final consumer-facing tag)
- Metadata container: `{TAG_BASE}-about` (unchanged)

Per-platform suffix must be derivable from `RBRV_CONJURE_PLATFORMS` entries.
Platform string `linux/amd64` → suffix `-amd64`, `linux/arm64` → `-arm64`,
`linux/arm/v7` → `-armv7`.

## Scope

### rbf_Foundry.sh — stitch function

- Detect platform count from RBRV_CONJURE_PLATFORMS
- Single-platform (1 platform): existing `--load` path (unchanged)
- Multi-platform (2+ platforms): new `--push` + pullback + reassembly path:
  1. `buildx --push` with all platforms → intermediate `-multi` tag
  2. Per-platform `docker pull --platform <PLAT>` → `docker tag` per-platform
  3. `docker push` each per-platform tag (pre-push for imagetools)
  4. `imagetools create` assembles consumer-facing manifest list
  5. `images:` field lists all per-platform tags (SLSA provenance)
  6. `requestedVerifyOption: VERIFIED`
- Remove the single-arch gate that currently rejects multi-platform vessels
  at inscribe time
- `$${}` escaping for shell variables in generated cloudbuild.json

### Step scripts (Tools/rbw/rbgjb/)

- Update or create step scripts for pullback, push, and imagetools sequences
- Address Syft/SBOM strategy per open decision above

### cloudbuild.json generation

- `images:` field must list all per-platform tagged URIs
- Private pool + VERIFIED options (same as single-arch)

## Acceptance Criteria

- `rbf_stitch` generates valid cloudbuild.json for both single and multi-platform
- Multi-platform cloudbuild.json includes pullback, push, and imagetools steps
- Multi-platform `images:` field lists all per-platform tags
- No changes to single-platform vessel behavior
- BCG compliance for all new bash code

**[260305-1433] rough**

Implement multi-platform SLSA provenance code path in the stitch function
and step scripts.

## Prerequisites

- ₢AlAAR (spec-multiplatform-provenance) complete — pipeline shape defined

## Design Decisions (resolve during implementation)

### imagetools create placement

`imagetools create` reassembles per-platform images into a multi-platform
manifest list. It must run AFTER `images:` pushes the per-platform images
(since `images:` push happens after all steps complete). Options:

- (a) Additional CB step that runs before `images:` push — won't work,
  `images:` push is post-step
- (b) Post-build step triggered by build success (e.g., Pub/Sub → Cloud Function)
- (c) Local post-inscribe step in the dispatch/inscribe command
- (d) Separate `gcloud builds submit` invocation after the main build

Recommendation: evaluate (c) or (d) — simplest approaches. The reassembly
is a single `docker buildx imagetools create` command operating registry-side.

### Syft/SBOM strategy

Single-arch pipeline scans the local daemon image (step 04). For multi-platform,
after pullback there are N platform images in the local daemon. Options:

- Scan each per-platform image separately → N SBOMs
- Scan only the native-arch image (amd64) → 1 SBOM (platforms share layers)
- Scan the `-multi` registry tag via `docker://` transport

Must align with existing SBOM/metadata container format.

### Tag scheme

Must integrate with existing `_RBGY_INSCRIBE_TIMESTAMP` + `_RBGY_ARK_SUFFIX_IMAGE`
pattern. Proposed scheme:

- Multi-platform push: `{INSCRIBE_TS}-multi` (intermediate, registry only)
- Per-platform tags: `{INSCRIBE_TS}{ARK_SUFFIX}-amd64`, `...arm64`, `...armv7`
- Reassembled manifest: `{INSCRIBE_TS}{ARK_SUFFIX}` (final consumer-facing tag)
- Metadata container: `{TAG_BASE}-about` (unchanged)

Per-platform suffix must be derivable from `RBRV_CONJURE_PLATFORMS` entries.
Platform string `linux/amd64` → suffix `-amd64`, `linux/arm64` → `-arm64`,
`linux/arm/v7` → `-armv7`.

### Intermediate tag lifecycle

The `-multi` tag pushed by buildx contains BuildKit attestations. After
`imagetools create` reassembles the final manifest list, the `-multi` tag
is no longer needed by consumers. Options: keep (registry clutter), delete
(clean but adds a step), or overwrite with `imagetools create` output.

## Scope

### rbf_Foundry.sh — stitch function

- Detect platform count from RBRV_CONJURE_PLATFORMS
- Single-platform (1 platform): existing `--load` path (unchanged)
- Multi-platform (2+ platforms): new `--push` + pullback path:
  1. `buildx --push` with all platforms → intermediate multi tag
  2. Per-platform `docker pull --platform <PLAT>` → `docker tag` per-platform
  3. `images:` field lists all per-platform tags
  4. `requestedVerifyOption: VERIFIED`
- Remove the single-arch gate that currently rejects multi-platform vessels
  at inscribe time

### Step scripts (Tools/rbw/rbgjb/)

- Update or create step scripts for the pullback sequence
- Ensure `$${}` escaping for shell variables in generated cloudbuild.json
- Address Syft/SBOM strategy per design decision above

### cloudbuild.json generation

- `images:` field must list all per-platform tagged URIs
- Private pool + VERIFIED options (same as single-arch)

### Post-build reassembly

- Implement `imagetools create` invocation per design decision above
- Verify manifest list is correct after reassembly

## Acceptance Criteria

- `rbf_stitch` generates valid cloudbuild.json for both single and multi-platform
- Multi-platform cloudbuild.json includes pullback steps and multi-URI images field
- No changes to single-platform vessel behavior
- imagetools reassembly mechanism implemented and tested locally
- BCG compliance for all new bash code

**[260305-1430] rough**

Implement multi-platform SLSA provenance code path in the stitch function
and step scripts.

## Prerequisites

- ₢AlAAR (spec-multiplatform-provenance) complete

## Scope

### rbf_Foundry.sh — stitch function

- Detect platform count from RBRV_CONJURE_PLATFORMS
- Single-platform (1 platform): existing `--load` path (unchanged)
- Multi-platform (2+ platforms): new `--push` + pullback path:
  1. `buildx --push` with all platforms → push to GAR with a `-multi` tag
  2. Per-platform `docker pull --platform <PLAT>` → `docker tag` to per-platform tags
  3. `images:` field lists all per-platform tags
  4. `requestedVerifyOption: VERIFIED`
  5. Post-build: `imagetools create` reassembles multi-platform manifest list
- Remove the single-arch gate that currently rejects multi-platform vessels
  at inscribe time

### Step scripts (Tools/rbw/rbgjb/)

- Update or create step scripts for the pullback sequence
- Ensure `$${}` escaping for shell variables in generated cloudbuild.json
- Tag scheme: inscribe_timestamp + platform suffix (e.g., `-amd64`, `-arm64`)

### cloudbuild.json generation

- `images:` field must list all per-platform tagged URIs
- Private pool + VERIFIED options (same as single-arch)

## Acceptance Criteria

- `rbf_stitch` generates valid cloudbuild.json for both single and multi-platform vessels
- Multi-platform cloudbuild.json includes pullback steps and multi-URI images field
- No changes to single-platform vessel behavior
- BCG compliance for all new bash code

### verify-multiplatform-provenance-e2e (₢AlAAT) [complete]

**[260305-1620] complete**

End-to-end verification of multi-platform SLSA provenance on a live
trigger-dispatched build.

## Prerequisites

- ₢AlAAS (stitch-multiplatform-provenance) complete
- demo1025 depot with CB v2 connection and private pool

## Scope

1. Inscribe the original `rbev-busybox` vessel (3 platforms: amd64, arm64, arm/v7)
   using the updated stitch function
2. Verify generated cloudbuild.json has correct multi-platform structure:
   - `buildx --push` with `-multi` intermediate tag
   - Pullback steps for each platform
   - `docker push` per-platform pre-push step
   - Per-platform Syft scan steps (sequential)
   - Per-platform build_info generation
   - Multi-platform `-about` buildx step (`FROM scratch` + TARGETARCH)
   - `imagetools create` in-build reassembly step
   - `images:` lists all per-platform tags
   - `requestedVerifyOption: VERIFIED`
3. Trigger dispatch (push to rubric repo)
4. Wait for build completion
5. Verify SLSA Level 3 on each per-platform image
6. Verify same buildInvocationId across all platforms
7. Verify manifest list reassembly produced transparent multi-platform tag
8. Verify `docker pull` of the combined tag resolves correctly

## Metadata verification

Verify multi-platform `-about` container:
- `-about` is a multi-platform manifest list (same platforms as `-image`)
- Pull `-about` on different platforms yields platform-specific content
- Each platform's `sbom.json` describes that platform's image (not another arch)
- Each platform's `build_info.json` contains:
  - Correct platform string
  - Image digest matching the per-platform `-image` digest
  - QEMU used boolean (true for arm64/armv7, false for amd64)
  - `slsa_build_level`: 3
  - `build_invocation_id`: matching the CB build ID
  - `provenance_predicate_types`: v0.1 + v1
  - `provenance_builder_id`: GoogleHostedWorker URL

## Trigger interaction awareness

Inscribe creates triggers for vessels that have `cloudbuild.json`. When the
single-arch gate is removed (₢AlAAS), inscribe will generate cloudbuild.json
for `rbev-busybox` (3-platform). The bifurcated vessels (rbev-busybox-amd64,
rbev-busybox-arm64) also have their own cloudbuild.json and existing triggers.

During this e2e test:
- All vessels with cloudbuild.json will get triggers on inscribe
- The bifurcated vessel triggers will still fire — this is expected and harmless
- Focus verification on the multi-platform rbev-busybox build

Bifurcated vessel cleanup (removal of rbev-busybox-amd64/arm64 directories,
trigger cleanup) is deferred to ₢AlAAM (posture update).
Do not mix cleanup with verification.

## Acceptance Criteria

- rbev-busybox builds with 3 platforms via trigger dispatch
- SLSA Level 3 on all 3 per-platform images
- Same buildInvocationId proves single-build origin
- Multi-platform manifest list resolves correctly
- Multi-platform `-about` has per-platform SBOMs and build_info with SLSA fields
- Consumer-facing tag is platform-transparent
- Results documented in memo and paddock

**[260305-1513] rough**

End-to-end verification of multi-platform SLSA provenance on a live
trigger-dispatched build.

## Prerequisites

- ₢AlAAS (stitch-multiplatform-provenance) complete
- demo1025 depot with CB v2 connection and private pool

## Scope

1. Inscribe the original `rbev-busybox` vessel (3 platforms: amd64, arm64, arm/v7)
   using the updated stitch function
2. Verify generated cloudbuild.json has correct multi-platform structure:
   - `buildx --push` with `-multi` intermediate tag
   - Pullback steps for each platform
   - `docker push` per-platform pre-push step
   - Per-platform Syft scan steps (sequential)
   - Per-platform build_info generation
   - Multi-platform `-about` buildx step (`FROM scratch` + TARGETARCH)
   - `imagetools create` in-build reassembly step
   - `images:` lists all per-platform tags
   - `requestedVerifyOption: VERIFIED`
3. Trigger dispatch (push to rubric repo)
4. Wait for build completion
5. Verify SLSA Level 3 on each per-platform image
6. Verify same buildInvocationId across all platforms
7. Verify manifest list reassembly produced transparent multi-platform tag
8. Verify `docker pull` of the combined tag resolves correctly

## Metadata verification

Verify multi-platform `-about` container:
- `-about` is a multi-platform manifest list (same platforms as `-image`)
- Pull `-about` on different platforms yields platform-specific content
- Each platform's `sbom.json` describes that platform's image (not another arch)
- Each platform's `build_info.json` contains:
  - Correct platform string
  - Image digest matching the per-platform `-image` digest
  - QEMU used boolean (true for arm64/armv7, false for amd64)
  - `slsa_build_level`: 3
  - `build_invocation_id`: matching the CB build ID
  - `provenance_predicate_types`: v0.1 + v1
  - `provenance_builder_id`: GoogleHostedWorker URL

## Trigger interaction awareness

Inscribe creates triggers for vessels that have `cloudbuild.json`. When the
single-arch gate is removed (₢AlAAS), inscribe will generate cloudbuild.json
for `rbev-busybox` (3-platform). The bifurcated vessels (rbev-busybox-amd64,
rbev-busybox-arm64) also have their own cloudbuild.json and existing triggers.

During this e2e test:
- All vessels with cloudbuild.json will get triggers on inscribe
- The bifurcated vessel triggers will still fire — this is expected and harmless
- Focus verification on the multi-platform rbev-busybox build

Bifurcated vessel cleanup (removal of rbev-busybox-amd64/arm64 directories,
trigger cleanup) is deferred to ₢AlAAM (posture update).
Do not mix cleanup with verification.

## Acceptance Criteria

- rbev-busybox builds with 3 platforms via trigger dispatch
- SLSA Level 3 on all 3 per-platform images
- Same buildInvocationId proves single-build origin
- Multi-platform manifest list resolves correctly
- Multi-platform `-about` has per-platform SBOMs and build_info with SLSA fields
- Consumer-facing tag is platform-transparent
- Results documented in memo and paddock

**[260305-1433] rough**

End-to-end verification of multi-platform SLSA provenance on a live
trigger-dispatched build.

## Prerequisites

- ₢AlAAS (stitch-multiplatform-provenance) complete
- demo1025 depot with CB v2 connection and private pool

## Scope

1. Inscribe the original `rbev-busybox` vessel (3 platforms: amd64, arm64, arm/v7)
   using the updated stitch function
2. Verify generated cloudbuild.json has correct multi-platform structure:
   - Pullback steps for each platform
   - `images:` lists all per-platform tags
   - `requestedVerifyOption: VERIFIED`
3. Trigger dispatch (push to rubric repo)
4. Wait for build completion
5. Verify SLSA Level 3 on each per-platform image
6. Verify same buildInvocationId across all platforms
7. Verify manifest list reassembly produced transparent multi-platform tag
8. Verify `docker pull` of the combined tag resolves correctly

## Trigger interaction awareness

Inscribe creates triggers for vessels that have `cloudbuild.json`. When the
single-arch gate is removed (₢AlAAS), inscribe will generate cloudbuild.json
for `rbev-busybox` (3-platform). The bifurcated vessels (rbev-busybox-amd64,
rbev-busybox-arm64) also have their own cloudbuild.json and existing triggers.

During this e2e test:
- All vessels with cloudbuild.json will get triggers on inscribe
- The bifurcated vessel triggers will still fire — this is expected and harmless
- Focus verification on the multi-platform rbev-busybox build

Bifurcated vessel cleanup (removal of rbev-busybox-amd64/arm64 directories,
trigger cleanup) is deferred to ₢AlAAM (posture update) or a separate pace.
Do not mix cleanup with verification.

## SBOM and metadata verification

Whatever Syft/SBOM strategy ₢AlAAS implements, verify:
- SBOM is generated without errors
- Metadata container is built and pushed correctly
- `build_info.json` content is correct for multi-platform build

## Acceptance Criteria

- rbev-busybox builds with 3 platforms via trigger dispatch
- SLSA Level 3 on all 3 per-platform images
- Same buildInvocationId proves single-build origin
- Multi-platform manifest list resolves correctly
- SBOM and metadata containers are correct
- Results documented in memo and paddock

**[260305-1430] rough**

End-to-end verification of multi-platform SLSA provenance on a live
trigger-dispatched build.

## Prerequisites

- ₢AlAAS (stitch-multiplatform-provenance) complete
- demo1025 depot with CB v2 connection and private pool

## Scope

1. Inscribe the original `rbev-busybox` vessel (3 platforms: amd64, arm64, arm/v7)
   using the updated stitch function
2. Verify generated cloudbuild.json has correct multi-platform structure
3. Trigger dispatch (push to rubric repo)
4. Wait for build completion
5. Verify SLSA Level 3 on each per-platform image
6. Verify same buildInvocationId across all platforms
7. Verify manifest list reassembly produced transparent multi-platform tag
8. Verify `docker pull` of the combined tag resolves correctly

## Cleanup considerations

- Bifurcated vessels (rbev-busybox-amd64, rbev-busybox-arm64) can be removed
  after successful verification, or retained as single-arch test targets
- Decision deferred to this pace

## Acceptance Criteria

- rbev-busybox builds with 3 platforms via trigger dispatch
- SLSA Level 3 on all 3 per-platform images
- Same buildInvocationId proves single-build origin
- Multi-platform manifest list resolves correctly
- Results documented in memo

### slsa-vouch-and-consecration-check (₢AlAAV) [complete]

**[260306-0812] complete**

Two role-separated tabtargets for SLSA provenance verification, plus
IAM addition to grant Retriever the containeranalysis read permission.

## Tabtarget 1: rbw-Dc.DirectorChecksConsecrations.sh

Director lists consecrations for a vessel by querying GAR tags.

- No-arg: lists available vessel directories (same pattern as rbf_build)
- With vessel-dir arg: queries GAR tags, filters for inscribe-timestamp
  pattern (i\d{8}_\d{6}), groups by consecration, shows tabular report:
  ```
  Vessel: rbev-busybox
    Consecration         Platforms              Tags
    i20260305_133650     amd64, arm64, armv7    3 image + 1 about
    i20260305_154104     amd64, arm64, armv7    3 image + 1 about
  ```
- Auth: Director RBRA credentials + OAuth token
- Function: rbf_director_checks_consecrations() in rbf_Foundry.sh
- Zipper: buz_enroll RBZ_CHECK_CONSECRATIONS "rbw-Dc" rbf_cli.sh

## Tabtarget 2: rbw-Rv.RetrieverVouchesArk.sh

Retriever verifies SLSA provenance on a vessel's per-platform images.

- No-arg: lists available vessel directories
- With vessel-dir arg: identifies most recent consecration, then for each
  per-platform image tag:
  1. Resolve image digest from GAR (Docker Registry v2 API)
  2. Query Container Analysis REST API for BUILD occurrences:
     GET .../v1/projects/{PROJECT}/occurrences
     filter: kind="BUILD" AND resourceUrl="https://{IMAGE_URI}@{DIGEST}"
  3. Extract slsaBuildLevel and buildInvocationId from provenance
  4. Display tabular report:
     ```
     Vessel: rbev-busybox
     Consecration: i20260305_154104

       Platform     Digest (short)   SLSA Level   Build ID
       linux/amd64  d8ddccf8ea1f     3            2e172ce0
       linux/arm64  d085e8e47c62     3            2e172ce0
       linux/arm/v7 289dbc68d93a     3            2e172ce0

       Build origin: single invocation (all IDs match)
     ```
- Auth: Retriever RBRA credentials + OAuth token
- Function: new function in rbf_Foundry.sh (or separate module if cleaner,
  since Retriever auth path differs from Director)
- Zipper: buz_enroll RBZ_VOUCH_ARK "rbw-Rv" ...
- No gcloud dependency — pure REST API + OAuth

## IAM Addition (done separately, before this pace)

Add containeranalysis.occurrences.viewer (or equivalent minimal role) to
rbgg_create_retriever() in rbgg_Governor.sh, so new Retrievers can query
Container Analysis API.

## Implementation Notes

- rbw-Dc reuses GAR tag listing pattern from rbf_beseech
- rbw-Rv needs Container Analysis API (containeranalysis.googleapis.com)
- Both parse JSON with jq (available locally)
- BCG compliant
- Both follow zero-or-one argument tabtarget discipline

## Acceptance Criteria

- rbw-Dc lists consecrations for a vessel from GAR tags
- rbw-Rv displays SLSA level for each per-platform image
- rbw-Rv reports shared buildInvocationId (single-build origin check)
- Both work with single-platform and multi-platform vessels
- Both handle no-arg case with vessel listing
- Retriever IAM includes containeranalysis read permission
- Tested against rbev-busybox consecration i20260305_154104

**[260305-1646] rough**

Two role-separated tabtargets for SLSA provenance verification, plus
IAM addition to grant Retriever the containeranalysis read permission.

## Tabtarget 1: rbw-Dc.DirectorChecksConsecrations.sh

Director lists consecrations for a vessel by querying GAR tags.

- No-arg: lists available vessel directories (same pattern as rbf_build)
- With vessel-dir arg: queries GAR tags, filters for inscribe-timestamp
  pattern (i\d{8}_\d{6}), groups by consecration, shows tabular report:
  ```
  Vessel: rbev-busybox
    Consecration         Platforms              Tags
    i20260305_133650     amd64, arm64, armv7    3 image + 1 about
    i20260305_154104     amd64, arm64, armv7    3 image + 1 about
  ```
- Auth: Director RBRA credentials + OAuth token
- Function: rbf_director_checks_consecrations() in rbf_Foundry.sh
- Zipper: buz_enroll RBZ_CHECK_CONSECRATIONS "rbw-Dc" rbf_cli.sh

## Tabtarget 2: rbw-Rv.RetrieverVouchesArk.sh

Retriever verifies SLSA provenance on a vessel's per-platform images.

- No-arg: lists available vessel directories
- With vessel-dir arg: identifies most recent consecration, then for each
  per-platform image tag:
  1. Resolve image digest from GAR (Docker Registry v2 API)
  2. Query Container Analysis REST API for BUILD occurrences:
     GET .../v1/projects/{PROJECT}/occurrences
     filter: kind="BUILD" AND resourceUrl="https://{IMAGE_URI}@{DIGEST}"
  3. Extract slsaBuildLevel and buildInvocationId from provenance
  4. Display tabular report:
     ```
     Vessel: rbev-busybox
     Consecration: i20260305_154104

       Platform     Digest (short)   SLSA Level   Build ID
       linux/amd64  d8ddccf8ea1f     3            2e172ce0
       linux/arm64  d085e8e47c62     3            2e172ce0
       linux/arm/v7 289dbc68d93a     3            2e172ce0

       Build origin: single invocation (all IDs match)
     ```
- Auth: Retriever RBRA credentials + OAuth token
- Function: new function in rbf_Foundry.sh (or separate module if cleaner,
  since Retriever auth path differs from Director)
- Zipper: buz_enroll RBZ_VOUCH_ARK "rbw-Rv" ...
- No gcloud dependency — pure REST API + OAuth

## IAM Addition (done separately, before this pace)

Add containeranalysis.occurrences.viewer (or equivalent minimal role) to
rbgg_create_retriever() in rbgg_Governor.sh, so new Retrievers can query
Container Analysis API.

## Implementation Notes

- rbw-Dc reuses GAR tag listing pattern from rbf_beseech
- rbw-Rv needs Container Analysis API (containeranalysis.googleapis.com)
- Both parse JSON with jq (available locally)
- BCG compliant
- Both follow zero-or-one argument tabtarget discipline

## Acceptance Criteria

- rbw-Dc lists consecrations for a vessel from GAR tags
- rbw-Rv displays SLSA level for each per-platform image
- rbw-Rv reports shared buildInvocationId (single-build origin check)
- Both work with single-platform and multi-platform vessels
- Both handle no-arg case with vessel listing
- Retriever IAM includes containeranalysis read permission
- Tested against rbev-busybox consecration i20260305_154104

**[260305-1645] rough**

Two role-separated tabtargets for SLSA provenance verification, plus
IAM addition to grant Retriever the containeranalysis read permission.

## Tabtarget 1: rbw-Dc.DirectorChecksConsecrations.sh

Director lists consecrations for a vessel by querying GAR tags.

- No-arg: lists available vessel directories (same pattern as rbf_build)
- With vessel-dir arg: queries GAR tags, filters for inscribe-timestamp
  pattern (i\d{8}_\d{6}), groups by consecration, shows tabular report:
  ```
  Vessel: rbev-busybox
    Consecration         Platforms              Tags
    i20260305_133650     amd64, arm64, armv7    3 image + 1 about
    i20260305_154104     amd64, arm64, armv7    3 image + 1 about
  ```
- Auth: Director RBRA credentials + OAuth token
- Function: rbf_director_checks_consecrations() in rbf_Foundry.sh
- Zipper: buz_enroll RBZ_CHECK_CONSECRATIONS "rbw-Dc" rbf_cli.sh

## Tabtarget 2: rbw-Rv.RetrieverVouchesArk.sh

Retriever verifies SLSA provenance on a vessel's per-platform images.

- No-arg: lists available vessel directories
- With vessel-dir arg: identifies most recent consecration, then for each
  per-platform image tag:
  1. Resolve image digest from GAR (Docker Registry v2 API)
  2. Query Container Analysis REST API for BUILD occurrences:
     GET .../v1/projects/{PROJECT}/occurrences
     filter: kind="BUILD" AND resourceUrl="https://{IMAGE_URI}@{DIGEST}"
  3. Extract slsaBuildLevel and buildInvocationId from provenance
  4. Display tabular report:
     ```
     Vessel: rbev-busybox
     Consecration: i20260305_154104

       Platform     Digest (short)   SLSA Level   Build ID
       linux/amd64  d8ddccf8ea1f     3            2e172ce0
       linux/arm64  d085e8e47c62     3            2e172ce0
       linux/arm/v7 289dbc68d93a     3            2e172ce0

       Build origin: single invocation (all IDs match)
     ```
- Auth: Retriever RBRA credentials + OAuth token
- Function: new function in rbf_Foundry.sh (or separate module if cleaner,
  since Retriever auth path differs from Director)
- Zipper: buz_enroll RBZ_VOUCH_ARK "rbw-Rv" ...
- No gcloud dependency — pure REST API + OAuth

## IAM Addition (done separately, before this pace)

Add containeranalysis.occurrences.viewer (or equivalent minimal role) to
rbgg_create_retriever() in rbgg_Governor.sh, so new Retrievers can query
Container Analysis API.

## Implementation Notes

- rbw-Dc reuses GAR tag listing pattern from rbf_beseech
- rbw-Rv needs Container Analysis API (containeranalysis.googleapis.com)
- Both parse JSON with jq (available locally)
- BCG compliant
- Both follow zero-or-one argument tabtarget discipline

## Acceptance Criteria

- rbw-Dc lists consecrations for a vessel from GAR tags
- rbw-Rv displays SLSA level for each per-platform image
- rbw-Rv reports shared buildInvocationId (single-build origin check)
- Both work with single-platform and multi-platform vessels
- Both handle no-arg case with vessel listing
- Retriever IAM includes containeranalysis read permission
- Tested against rbev-busybox consecration i20260305_154104

**[260305-1607] rough**

Create a verification tabtarget that queries Container Analysis REST API
for SLSA provenance facts on a vessel's per-platform images.

## Context

After a trigger-dispatched build completes, the operator needs to confirm
that Google actually stamped SLSA Level 3 on each per-platform image. This
tabtarget provides that ground truth using only OAuth/curl (no gcloud).

## Design

Tabtarget: `tt/rbw-DV.DirectorVouchesArk.sh <vessel-dir>`

Operates on a vessel directory. Determines the most recent consecration
(or accepts one as optional argument). For each per-platform image tag:

1. Resolve the image digest from GAR (Docker Registry v2 API or GAR REST API)
2. Query Container Analysis REST API for BUILD occurrences:
   `GET https://containeranalysis.googleapis.com/v1/projects/{PROJECT}/occurrences`
   with filter: `kind="BUILD" AND resourceUrl="https://{IMAGE_URI}@{DIGEST}"`
3. Extract `slsaBuildLevel` and `buildInvocationId` from provenance
4. Display tabular report:
   ```
   Vessel: rbev-busybox
   Consecration: i20260305_154104-b20260305_234254

     Platform     Digest (short)   SLSA Level   Build ID
     linux/amd64  d8ddccf8ea1f     3            2e172ce0
     linux/arm64  d085e8e47c62     3            2e172ce0
     linux/arm/v7 289dbc68d93a     3            2e172ce0

     Build origin: single invocation (all IDs match)
   ```

## Authentication

Uses Director RBRA credentials + OAuth access token, same pattern as
existing rbgu_http_remit_* functions. No gcloud dependency.

## Implementation

- New function in rbf_Foundry.sh (or new module if cleaner)
- Reuse rbgu_http_remit_get for REST calls
- Parse JSON responses with jq (available locally, unlike CB workers)
- BCG compliant

## Acceptance Criteria

- Tabtarget displays SLSA level for each per-platform image
- Reports shared buildInvocationId (single-build origin check)
- Works with both single-platform and multi-platform vessels
- No gcloud dependency — pure REST API + OAuth
- Tested against rbev-busybox consecration i20260305_154104

### ark-vouch-artifact-and-spec (₢AlAAW) [complete]

**[260306-1027] complete**

Define and implement the `-vouch` ark artifact — a post-build container
recording verified SLSA provenance facts about the `-image` artifacts.

## Context

The `-about` container is built during the CB build, before SLSA provenance
is generated. It can only record precondition intent, not verified facts.
The `-vouch` container is built after the build completes, recording what
Container Analysis actually attested.

## Design

### New term: rbtga_ark_vouch

The verification artifact within an ark, tagged with the `-vouch` suffix.
Contains post-build SLSA provenance assessment for each per-platform image,
queried from Container Analysis after Cloud Build generates provenance.
Built locally by the Director after trigger-dispatched build completion.
Single-platform (`FROM scratch`, linux/amd64) — content is platform-independent
JSON describing per-platform SLSA facts.

### New constant: RBGC_ARK_SUFFIX_VOUCH="-vouch"

### Ark definition update (rbtga_ark in RBS0)

Update `rbtga_ark` from "paired set" to include `-vouch` as an optional
third component. The ark comprises `-image` and `-about` (built by CB),
with an optional `-vouch` (built locally post-CB) recording verified
provenance facts. Update the `rbtga_consecration` tag examples accordingly.

### Container contents

Single file: `slsa_vouch.json` containing:
```json
{
  "vessel": "rbev-busybox",
  "consecration": "i20260305_154104-b20260305_234254",
  "vouch_timestamp": "2026-03-05T15:50:00Z",
  "platforms": [
    {
      "platform": "linux/amd64",
      "image_digest": "sha256:d8ddccf8...",
      "slsa_build_level": 3,
      "build_invocation_id": "2e172ce0-...",
      "predicate_types": ["v0.1", "v1"]
    }
  ],
  "single_build_origin": true
}
```

### Tag scheme

`{CONSECRATION}-vouch` (e.g., `i20260305_154104-b20260305_234254-vouch`)
Uses the full consecration (inscribe + build timestamps) since the vouch
is specific to a particular build, not just an inscribe.

### Build mechanism

Local `docker build` + `docker push` using Director credentials.
Integrated into the vouch tabtarget (₢AlAAV) — after displaying the
report, optionally builds and pushes the `-vouch` container.

## Scope

### RBS0-SpecTop.adoc
- Add `rbtga_ark_vouch` attribute reference and definition
- Update `rbtga_ark` definition from "paired set" to include optional `-vouch`
- Update `rbtga_consecration` tag examples to include `-vouch`

### Tools/rbw/rbgc_Constants.sh
- Add `RBGC_ARK_SUFFIX_VOUCH="-vouch"`

### Tools/rbw/rbf_Foundry.sh (or vouch tabtarget)
- Build `FROM scratch` container with `slsa_vouch.json`
- Push to GAR with consecration-vouch tag

## Acceptance Criteria

- `rbtga_ark` definition updated to include `-vouch` as optional third component
- `rbtga_ark_vouch` term defined in RBS0 with attribute reference
- `-vouch` container pushed to GAR after successful vouch verification
- Contains accurate SLSA facts matching Container Analysis query results
- Single-platform FROM scratch (no QEMU, no buildx)
- No gcloud dependency

**[260305-1614] rough**

Define and implement the `-vouch` ark artifact — a post-build container
recording verified SLSA provenance facts about the `-image` artifacts.

## Context

The `-about` container is built during the CB build, before SLSA provenance
is generated. It can only record precondition intent, not verified facts.
The `-vouch` container is built after the build completes, recording what
Container Analysis actually attested.

## Design

### New term: rbtga_ark_vouch

The verification artifact within an ark, tagged with the `-vouch` suffix.
Contains post-build SLSA provenance assessment for each per-platform image,
queried from Container Analysis after Cloud Build generates provenance.
Built locally by the Director after trigger-dispatched build completion.
Single-platform (`FROM scratch`, linux/amd64) — content is platform-independent
JSON describing per-platform SLSA facts.

### New constant: RBGC_ARK_SUFFIX_VOUCH="-vouch"

### Ark definition update (rbtga_ark in RBS0)

Update `rbtga_ark` from "paired set" to include `-vouch` as an optional
third component. The ark comprises `-image` and `-about` (built by CB),
with an optional `-vouch` (built locally post-CB) recording verified
provenance facts. Update the `rbtga_consecration` tag examples accordingly.

### Container contents

Single file: `slsa_vouch.json` containing:
```json
{
  "vessel": "rbev-busybox",
  "consecration": "i20260305_154104-b20260305_234254",
  "vouch_timestamp": "2026-03-05T15:50:00Z",
  "platforms": [
    {
      "platform": "linux/amd64",
      "image_digest": "sha256:d8ddccf8...",
      "slsa_build_level": 3,
      "build_invocation_id": "2e172ce0-...",
      "predicate_types": ["v0.1", "v1"]
    }
  ],
  "single_build_origin": true
}
```

### Tag scheme

`{CONSECRATION}-vouch` (e.g., `i20260305_154104-b20260305_234254-vouch`)
Uses the full consecration (inscribe + build timestamps) since the vouch
is specific to a particular build, not just an inscribe.

### Build mechanism

Local `docker build` + `docker push` using Director credentials.
Integrated into the vouch tabtarget (₢AlAAV) — after displaying the
report, optionally builds and pushes the `-vouch` container.

## Scope

### RBS0-SpecTop.adoc
- Add `rbtga_ark_vouch` attribute reference and definition
- Update `rbtga_ark` definition from "paired set" to include optional `-vouch`
- Update `rbtga_consecration` tag examples to include `-vouch`

### Tools/rbw/rbgc_Constants.sh
- Add `RBGC_ARK_SUFFIX_VOUCH="-vouch"`

### Tools/rbw/rbf_Foundry.sh (or vouch tabtarget)
- Build `FROM scratch` container with `slsa_vouch.json`
- Push to GAR with consecration-vouch tag

## Acceptance Criteria

- `rbtga_ark` definition updated to include `-vouch` as optional third component
- `rbtga_ark_vouch` term defined in RBS0 with attribute reference
- `-vouch` container pushed to GAR after successful vouch verification
- Contains accurate SLSA facts matching Container Analysis query results
- Single-platform FROM scratch (no QEMU, no buildx)
- No gcloud dependency

**[260305-1613] rough**

Define and implement the `-vouch` ark artifact — a post-build container
recording verified SLSA provenance facts about the `-image` artifacts.

## Context

The `-about` container is built during the CB build, before SLSA provenance
is generated. It can only record precondition intent, not verified facts.
The `-vouch` container is built after the build completes, recording what
Container Analysis actually attested.

## Design

### New term: rbtga_ark_vouch

The verification artifact within an ark, tagged with the `-vouch` suffix.
Contains post-build SLSA provenance assessment for each per-platform image,
queried from Container Analysis after Cloud Build generates provenance.
Built locally by the Director after trigger-dispatched build completion.
Single-platform (`FROM scratch`, linux/amd64) — content is platform-independent
JSON describing per-platform SLSA facts.

### New constant: RBGC_ARK_SUFFIX_VOUCH="-vouch"

### Ark definition update (rbtga_ark in RBS0)

Update `rbtga_ark` from "paired set" to include `-vouch` as an optional
third component. The ark comprises `-image` and `-about` (built by CB),
with an optional `-vouch` (built locally post-CB) recording verified
provenance facts. Update the `rbtga_consecration` tag examples accordingly.

### Container contents

Single file: `slsa_vouch.json` containing:
```json
{
  "vessel": "rbev-busybox",
  "consecration": "i20260305_154104-b20260305_234254",
  "vouch_timestamp": "2026-03-05T15:50:00Z",
  "platforms": [
    {
      "platform": "linux/amd64",
      "image_digest": "sha256:d8ddccf8...",
      "slsa_build_level": 3,
      "build_invocation_id": "2e172ce0-...",
      "predicate_types": ["v0.1", "v1"]
    }
  ],
  "single_build_origin": true
}
```

### Tag scheme

`{CONSECRATION}-vouch` (e.g., `i20260305_154104-b20260305_234254-vouch`)
Uses the full consecration (inscribe + build timestamps) since the vouch
is specific to a particular build, not just an inscribe.

### Build mechanism

Local `docker build` + `docker push` using Director credentials.
Integrated into the vouch tabtarget (₢AlAAV) — after displaying the
report, optionally builds and pushes the `-vouch` container.

## Scope

### RBS0-SpecTop.adoc
- Add `rbtga_ark_vouch` attribute reference and definition
- Update `rbtga_ark` definition from "paired set" to include optional `-vouch`
- Update `rbtga_consecration` tag examples to include `-vouch`

### Tools/rbw/rbgc_Constants.sh
- Add `RBGC_ARK_SUFFIX_VOUCH="-vouch"`

### Tools/rbw/rbf_Foundry.sh (or vouch tabtarget)
- Build `FROM scratch` container with `slsa_vouch.json`
- Push to GAR with consecration-vouch tag

## Acceptance Criteria

- `rbtga_ark` definition updated to include `-vouch` as optional third component
- `rbtga_ark_vouch` term defined in RBS0 with attribute reference
- `-vouch` container pushed to GAR after successful vouch verification
- Contains accurate SLSA facts matching Container Analysis query results
- Single-platform FROM scratch (no QEMU, no buildx)
- No gcloud dependency

**[260305-1608] rough**

Define and implement the `-vouch` ark artifact — a post-build container
recording verified SLSA provenance facts about the `-image` artifacts.

## Context

The `-about` container is built during the CB build, before SLSA provenance
is generated. It can only record precondition intent, not verified facts.
The `-vouch` container is built after the build completes, recording what
Container Analysis actually attested.

## Design

### New term: rbtga_ark_vouch

The verification artifact within an ark, tagged with the `-vouch` suffix.
Contains post-build SLSA provenance assessment for each per-platform image,
queried from Container Analysis after Cloud Build generates provenance.
Built locally by the Director after trigger-dispatched build completion.
Single-platform (`FROM scratch`, linux/amd64) — content is platform-independent
JSON describing per-platform SLSA facts.

### New constant: RBGC_ARK_SUFFIX_VOUCH="-vouch"

### Ark expansion

Arks expand from pairs to triples:
- `-image` — container layers (built by CB)
- `-about` — SBOM, build_info, recipe (built by CB)
- `-vouch` — SLSA verification record (built locally, post-CB)

### Container contents

Single file: `slsa_vouch.json` containing:
```json
{
  "vessel": "rbev-busybox",
  "consecration": "i20260305_154104-b20260305_234254",
  "vouch_timestamp": "2026-03-05T15:50:00Z",
  "platforms": [
    {
      "platform": "linux/amd64",
      "image_digest": "sha256:d8ddccf8...",
      "slsa_build_level": 3,
      "build_invocation_id": "2e172ce0-...",
      "predicate_types": ["v0.1", "v1"]
    }
  ],
  "single_build_origin": true
}
```

### Tag scheme

`{CONSECRATION}-vouch` (e.g., `i20260305_154104-b20260305_234254-vouch`)
Uses the full consecration (inscribe + build timestamps) since the vouch
is specific to a particular build, not just an inscribe.

### Build mechanism

Local `docker build` + `docker push` using Director credentials.
Integrated into the vouch tabtarget (₢AlAAV) — after displaying the
report, optionally builds and pushes the `-vouch` container.

## Scope

### RBS0-SpecTop.adoc
- Add `rbtga_ark_vouch` attribute reference and definition
- Update `rbtga_ark` definition to include `-vouch` as third component

### Tools/rbw/rbgc_Constants.sh
- Add `RBGC_ARK_SUFFIX_VOUCH="-vouch"`

### Tools/rbw/rbf_Foundry.sh (or vouch tabtarget)
- Build `FROM scratch` container with `slsa_vouch.json`
- Push to GAR with consecration-vouch tag

## Acceptance Criteria

- `-vouch` container pushed to GAR after successful vouch verification
- Contains accurate SLSA facts matching Container Analysis query results
- RBS0 documents the `-vouch` artifact and expanded ark definition
- Single-platform FROM scratch (no QEMU, no buildx)
- No gcloud dependency

### fix-abjure-orphan-vouch-hardening (₢AlAAZ) [complete]

**[260306-1052] complete**

Fix three issues discovered in ₢AlAAW review:

1. **Abjure orphans suffixed image tags**: After stitch unification,
   single-platform vessels have both suffixed and bare image tags.
   Abjure only deletes the bare tag, orphaning the suffixed one.
   Fix: add vessel_dir parameter to abjure (or enumerate tags from
   GAR matching inscribe_ts) so all image tags are deleted.

2. **Vouch soft-skips missing platform tags**: rbf_vouch continues
   silently when a deterministically-constructed platform tag is
   missing (FETCH_ERROR/NO_DIGEST). Since tags are derived from
   vessel config, a missing tag means the build is incomplete.
   Change continue to buc_die.

3. **RBS0 consecration tag examples inaccurate**: Line showing
   CONSECRATION-image is wrong — image tags use inscribe TS only
   and now include platform suffixes. Update examples to reflect
   the actual tag scheme post-unification.

**[260306-1026] rough**

Fix three issues discovered in ₢AlAAW review:

1. **Abjure orphans suffixed image tags**: After stitch unification,
   single-platform vessels have both suffixed and bare image tags.
   Abjure only deletes the bare tag, orphaning the suffixed one.
   Fix: add vessel_dir parameter to abjure (or enumerate tags from
   GAR matching inscribe_ts) so all image tags are deleted.

2. **Vouch soft-skips missing platform tags**: rbf_vouch continues
   silently when a deterministically-constructed platform tag is
   missing (FETCH_ERROR/NO_DIGEST). Since tags are derived from
   vessel config, a missing tag means the build is incomplete.
   Change continue to buc_die.

3. **RBS0 consecration tag examples inaccurate**: Line showing
   CONSECRATION-image is wrong — image tags use inscribe TS only
   and now include platform suffixes. Update examples to reflect
   the actual tag scheme post-unification.

### test-all-slsa-busybox-provenance (₢AlAAY) [complete]

**[260306-1111] complete**

Add an automated test to the COMPLETE suite that exercises the full SLSA
v1.0 provenance pipeline: conjure a busybox ark, then vouch its provenance.

## Infrastructure Prerequisite: ZBUTO_BURV_OUTPUT

Add one global variable to `zbuto_invoke` in `Tools/buk/buto_operations.sh`:

```bash
ZBUTO_BURV_OUTPUT="${z_burv_output}"   # expose BURV output root to caller
```

Set alongside existing `ZBUTO_STDOUT`, `ZBUTO_STDERR`, `ZBUTO_STATUS` after
invocation completes. When `BUTE_BURV_ROOT` is unset (no BURV isolation),
set to empty string.

This enables tests to read files written by a tabtarget to `BURD_OUTPUT_DIR`
via `${ZBUTO_BURV_OUTPUT}/current/<filename>`.

## Inter-Step Communication: Kindle-Constant Fact Files

Tabtargets that produce facts for downstream consumption write one file per
fact to `BURD_OUTPUT_DIR`, using a kindle constant as the filename.

**Producer** (in tabtarget command function):
```bash
echo "${z_image_ref}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_IMAGE_REF}"
```

**Consumer** (in test case, after `buto_tt_expect_ok`):
```bash
z_image_ref=$(<"${ZBUTO_BURV_OUTPUT}/current/${RBF_FACT_IMAGE_REF}")
```

No JSON, no parsing — one fact, one file, one constant. The kindle constant
is the contract between producer and consumer.

### Design Note: Fact Files vs ZBUTO_STDOUT

The ark-lifecycle test uses `ZBUTO_STDOUT` to capture tabtarget output and
parse it in the test. The fact-file pattern is chosen here instead because:

- Conjure output is complex (multi-step build log) — parsing for image ref
  is fragile
- Vouch output is a human-readable report — extracting a single SLSA level
  requires knowing the display format
- Fact files are format-independent: the producer writes the datum directly,
  the consumer reads it directly, with no coupling to display format

This means `rbf_build()` and `rbf_vouch()` gain fact-file writes — production
code writing data consumed by tests. This is intentional: the facts are
useful beyond testing (scripts, CI) and the writes are unconditional because
`BURD_OUTPUT_DIR` always exists during dispatch.

### Constant Location

Fact-file constants go in `Tools/rbw/rbgc_Constants.sh` (not rbf_Foundry.sh).
The `RBF_FACT_` prefix is new within `rbgc`; it follows terminal exclusivity
because `RBF` has no children in the constant namespace today.

Constants:
- `RBF_FACT_IMAGE_REF` — full image URI from conjure
- `RBF_FACT_SLSA_LEVEL` — SLSA build level string from vouch

## Test Case: rbtcsl_SlsaProvenance.sh

New file: `Tools/rbw/rbts/rbtcsl_SlsaProvenance.sh`

Pattern follows `rbtcal_ArkLifecycle.sh` — uses tabtarget tier
(`buto_tt_expect_ok`), NOT dispatch tier (`bute_dispatch`).

### Key Assumption: Conjure Is Synchronous

`rbf_build()` calls `zrbf_wait_build_completion()` internally, polling the
Cloud Build API until the build finishes. From the test's perspective, conjure
blocks until the image is built and pushed. No separate polling step needed.

### Steps

1. **Conjure** — `buto_tt_expect_ok "${RBZ_CONJURE_ARK}" "${z_vessel_dir}"`
   Read `${ZBUTO_BURV_OUTPUT}/current/${RBF_FACT_IMAGE_REF}` for image ref.

2. **Check consecrations** — `buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}" "${z_vessel_dir}"`
   Verify conjured image appears in GAR tag listing via `ZBUTO_STDOUT`.

3. **Vouch** — `buto_tt_expect_ok "${RBZ_VOUCH_ARK}" "${z_vessel_dir}"`
   Read `${ZBUTO_BURV_OUTPUT}/current/${RBF_FACT_SLSA_LEVEL}` and assert "3".

4. **Cleanup** — Delete the conjured image to restore baseline.

### Enrollment

- Test case function: `rbtcsl_provenance_tcase`
- Enrolled in `BUTR_SUITE_COMPLETE` (requires live GCP + depot + triggers)
- Fixture setup: needs `ZRBTB_ARK_VESSEL_SIGIL` (same as ark-lifecycle)
- Graceful skip when depot infrastructure unavailable

## Scope: What This Pace Does NOT Do

- Does NOT retire the dispatch tier (see ₢AkAAc)
- Does NOT convert regime smoke/credential tests from dispatch to tabtarget
- Does NOT add test-type vocabulary to BUS0 (see ₢AkAAd)

## Acceptance Criteria

- `ZBUTO_BURV_OUTPUT` exposed in `buto_operations.sh` after `zbuto_invoke`
- Fact-file constants in `rbgc_Constants.sh` with `RBF_FACT_` prefix
- `rbf_build()` writes `RBF_FACT_IMAGE_REF` to `BURD_OUTPUT_DIR`
- `rbf_vouch()` writes `RBF_FACT_SLSA_LEVEL` to `BURD_OUTPUT_DIR`
- Test dispatches busybox build and asserts SLSA Level 3 provenance
- Test passes on demo1025 depot
- Test enrolled in COMPLETE suite
- No dispatch-tier dependencies (pure tabtarget tier)

**[260306-0919] rough**

Add an automated test to the COMPLETE suite that exercises the full SLSA
v1.0 provenance pipeline: conjure a busybox ark, then vouch its provenance.

## Infrastructure Prerequisite: ZBUTO_BURV_OUTPUT

Add one global variable to `zbuto_invoke` in `Tools/buk/buto_operations.sh`:

```bash
ZBUTO_BURV_OUTPUT="${z_burv_output}"   # expose BURV output root to caller
```

Set alongside existing `ZBUTO_STDOUT`, `ZBUTO_STDERR`, `ZBUTO_STATUS` after
invocation completes. When `BUTE_BURV_ROOT` is unset (no BURV isolation),
set to empty string.

This enables tests to read files written by a tabtarget to `BURD_OUTPUT_DIR`
via `${ZBUTO_BURV_OUTPUT}/current/<filename>`.

## Inter-Step Communication: Kindle-Constant Fact Files

Tabtargets that produce facts for downstream consumption write one file per
fact to `BURD_OUTPUT_DIR`, using a kindle constant as the filename.

**Producer** (in tabtarget command function):
```bash
echo "${z_image_ref}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_IMAGE_REF}"
```

**Consumer** (in test case, after `buto_tt_expect_ok`):
```bash
z_image_ref=$(<"${ZBUTO_BURV_OUTPUT}/current/${RBF_FACT_IMAGE_REF}")
```

No JSON, no parsing — one fact, one file, one constant. The kindle constant
is the contract between producer and consumer.

### Design Note: Fact Files vs ZBUTO_STDOUT

The ark-lifecycle test uses `ZBUTO_STDOUT` to capture tabtarget output and
parse it in the test. The fact-file pattern is chosen here instead because:

- Conjure output is complex (multi-step build log) — parsing for image ref
  is fragile
- Vouch output is a human-readable report — extracting a single SLSA level
  requires knowing the display format
- Fact files are format-independent: the producer writes the datum directly,
  the consumer reads it directly, with no coupling to display format

This means `rbf_build()` and `rbf_vouch()` gain fact-file writes — production
code writing data consumed by tests. This is intentional: the facts are
useful beyond testing (scripts, CI) and the writes are unconditional because
`BURD_OUTPUT_DIR` always exists during dispatch.

### Constant Location

Fact-file constants go in `Tools/rbw/rbgc_Constants.sh` (not rbf_Foundry.sh).
The `RBF_FACT_` prefix is new within `rbgc`; it follows terminal exclusivity
because `RBF` has no children in the constant namespace today.

Constants:
- `RBF_FACT_IMAGE_REF` — full image URI from conjure
- `RBF_FACT_SLSA_LEVEL` — SLSA build level string from vouch

## Test Case: rbtcsl_SlsaProvenance.sh

New file: `Tools/rbw/rbts/rbtcsl_SlsaProvenance.sh`

Pattern follows `rbtcal_ArkLifecycle.sh` — uses tabtarget tier
(`buto_tt_expect_ok`), NOT dispatch tier (`bute_dispatch`).

### Key Assumption: Conjure Is Synchronous

`rbf_build()` calls `zrbf_wait_build_completion()` internally, polling the
Cloud Build API until the build finishes. From the test's perspective, conjure
blocks until the image is built and pushed. No separate polling step needed.

### Steps

1. **Conjure** — `buto_tt_expect_ok "${RBZ_CONJURE_ARK}" "${z_vessel_dir}"`
   Read `${ZBUTO_BURV_OUTPUT}/current/${RBF_FACT_IMAGE_REF}` for image ref.

2. **Check consecrations** — `buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}" "${z_vessel_dir}"`
   Verify conjured image appears in GAR tag listing via `ZBUTO_STDOUT`.

3. **Vouch** — `buto_tt_expect_ok "${RBZ_VOUCH_ARK}" "${z_vessel_dir}"`
   Read `${ZBUTO_BURV_OUTPUT}/current/${RBF_FACT_SLSA_LEVEL}` and assert "3".

4. **Cleanup** — Delete the conjured image to restore baseline.

### Enrollment

- Test case function: `rbtcsl_provenance_tcase`
- Enrolled in `BUTR_SUITE_COMPLETE` (requires live GCP + depot + triggers)
- Fixture setup: needs `ZRBTB_ARK_VESSEL_SIGIL` (same as ark-lifecycle)
- Graceful skip when depot infrastructure unavailable

## Scope: What This Pace Does NOT Do

- Does NOT retire the dispatch tier (see ₢AkAAc)
- Does NOT convert regime smoke/credential tests from dispatch to tabtarget
- Does NOT add test-type vocabulary to BUS0 (see ₢AkAAd)

## Acceptance Criteria

- `ZBUTO_BURV_OUTPUT` exposed in `buto_operations.sh` after `zbuto_invoke`
- Fact-file constants in `rbgc_Constants.sh` with `RBF_FACT_` prefix
- `rbf_build()` writes `RBF_FACT_IMAGE_REF` to `BURD_OUTPUT_DIR`
- `rbf_vouch()` writes `RBF_FACT_SLSA_LEVEL` to `BURD_OUTPUT_DIR`
- Test dispatches busybox build and asserts SLSA Level 3 provenance
- Test passes on demo1025 depot
- Test enrolled in COMPLETE suite
- No dispatch-tier dependencies (pure tabtarget tier)

**[260306-0903] rough**

Add an automated test to the COMPLETE suite that exercises the full SLSA
v1.0 provenance pipeline: conjure a busybox ark, then vouch its provenance.

## Infrastructure Prerequisite: ZBUTO_BURV_OUTPUT

Add one global variable to `zbuto_invoke` in `Tools/buk/buto_operations.sh`:

```bash
ZBUTO_BURV_OUTPUT="${z_burv_output}"   # expose BURV output root to caller
```

Set alongside existing `ZBUTO_STDOUT`, `ZBUTO_STDERR`, `ZBUTO_STATUS` after
invocation completes. When `BUTE_BURV_ROOT` is unset (no BURV isolation),
set to empty string.

This enables tests to read files written by a tabtarget to `BURD_OUTPUT_DIR`
via `${ZBUTO_BURV_OUTPUT}/current/<filename>`.

## Inter-Step Communication: Kindle-Constant Fact Files

Tabtargets that produce facts for downstream consumption write one file per
fact to `BURD_OUTPUT_DIR`, using a kindle constant as the filename.

**Producer** (in tabtarget command function):
```bash
echo "${z_image_ref}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_IMAGE_REF}"
```

**Consumer** (in test case, after `buto_tt_expect_ok`):
```bash
z_image_ref=$(<"${ZBUTO_BURV_OUTPUT}/current/${RBF_FACT_IMAGE_REF}")
```

No JSON, no parsing — one fact, one file, one constant. The kindle constant
is the contract between producer and consumer.

New constants needed in `rbf_Foundry.sh` (or `rbgc_Constants.sh`):
- `RBF_FACT_IMAGE_REF` — full image URI from conjure
- `RBF_FACT_SLSA_LEVEL` — SLSA build level string from vouch
- Additional fact constants as needed by the test assertions

## Test Case: rbtcsl_SlsaProvenance.sh

New file: `Tools/rbw/rbts/rbtcsl_SlsaProvenance.sh`

Pattern follows `rbtcal_ArkLifecycle.sh` — uses tabtarget tier
(`buto_tt_expect_ok`), NOT dispatch tier (`bute_dispatch`).

### Steps

1. **Conjure** — `buto_tt_expect_ok "${RBZ_CONJURE_ARK}" "${z_vessel_dir}"`
   Read `${ZBUTO_BURV_OUTPUT}/current/${RBF_FACT_IMAGE_REF}` for image ref.

2. **Check consecrations** — `buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}"`
   Verify conjured image appears in GAR tag listing.

3. **Vouch** — `buto_tt_expect_ok "${RBZ_VOUCH_ARK}"`
   Read `${ZBUTO_BURV_OUTPUT}/current/${RBF_FACT_SLSA_LEVEL}` and assert "3".

4. **Cleanup** — Delete the conjured image to restore baseline.

### Enrollment

- Test case function: `rbtcsl_provenance_tcase`
- Enrolled in `BUTR_SUITE_COMPLETE` (requires live GCP + depot + triggers)
- Fixture setup: needs `ZRBTB_ARK_VESSEL_SIGIL` (same as ark-lifecycle)
- Graceful skip when depot infrastructure unavailable

## Scope: What This Pace Does NOT Do

- Does NOT retire the dispatch tier (`bute_dispatch` / `bute_engine.sh`)
- Does NOT convert regime smoke/credential tests from dispatch to tabtarget
- Does NOT add test-type vocabulary to BUS0 (separate discussion)
- Does NOT modify the conjure/vouch tabtargets beyond adding fact-file writes

## Acceptance Criteria

- `ZBUTO_BURV_OUTPUT` exposed in `buto_operations.sh` after `zbuto_invoke`
- At least one fact file written by conjure, readable by test via BURV path
- Test dispatches busybox build and asserts SLSA Level 3 provenance
- Test passes on demo1025 depot
- Test enrolled in COMPLETE suite
- No dispatch-tier dependencies (pure tabtarget tier)

**[260306-0817] rough**

Add an automated test to the test-all suite that builds SLSA v1.0
busybox and validates its provenance.

## Scope

Add a test case to the rbw test suite that exercises the full SLSA
provenance pipeline end-to-end:

1. Inscribe + dispatch a busybox build (rbev-busybox or single-arch variant)
2. Wait for build completion
3. Run rbw-Dc (check consecrations) to verify tags appeared in GAR
4. Run rbw-Rv (vouch) to verify SLSA Level 3 provenance via Container Analysis
5. Optionally verify the -vouch artifact (if ₢AlAAW is complete)

## Context

The SLSA pipeline has been manually validated in ₢AlAAO (single-arch)
and ₢AlAAT (multi-platform). This pace adds automated regression
coverage so future changes to stitch, step scripts, or infrastructure
are caught by test-all.

## Implementation Notes

- Requires live GCP infrastructure (depot, triggers, private pool)
- Uses Director credentials for build dispatch and provenance query
- Test should be skippable when no depot is configured (graceful skip)
- May be a separate test target (rbw-tslsa or similar) if too heavy
  for the main test-all sweep

## Acceptance Criteria

- Test dispatches a busybox build and verifies SLSA Level 3 provenance
- Test passes on the current demo1025 depot
- Test is integrated into the test suite (rbw-ta or dedicated target)
- Test skips gracefully when depot infrastructure is unavailable

### cleanup-bifurcated-vessels (₢AlAAX) [complete]

**[260306-1129] complete**

Remove bifurcated vessel scaffolding that is no longer needed now that
multi-platform provenance is production-validated.

## Scope

- Remove `rbev-vessels/rbev-busybox-amd64/` directory
- Remove `rbev-vessels/rbev-busybox-arm64/` directory
- Clean up their triggers if still present (inscribe will skip them
  once directories are gone; triggers can be deleted via API or left
  to be cleaned up on next depot destroy/create cycle)
- Retain original `rbev-busybox` with 3-platform config as the canonical vessel

## Context

These were created in ₢AlAAN as single-arch test targets for the initial
SLSA provenance e2e (₢AlAAO). With multi-platform provenance validated
(₢AlAAT, build 2e172ce0), the original `rbev-busybox` handles all three
platforms natively.

## Acceptance Criteria

- rbev-busybox-amd64 and rbev-busybox-arm64 directories removed
- Inscribe succeeds with only the original multi-platform vessels
- No orphaned triggers blocking future depot operations

**[260305-1619] rough**

Remove bifurcated vessel scaffolding that is no longer needed now that
multi-platform provenance is production-validated.

## Scope

- Remove `rbev-vessels/rbev-busybox-amd64/` directory
- Remove `rbev-vessels/rbev-busybox-arm64/` directory
- Clean up their triggers if still present (inscribe will skip them
  once directories are gone; triggers can be deleted via API or left
  to be cleaned up on next depot destroy/create cycle)
- Retain original `rbev-busybox` with 3-platform config as the canonical vessel

## Context

These were created in ₢AlAAN as single-arch test targets for the initial
SLSA provenance e2e (₢AlAAO). With multi-platform provenance validated
(₢AlAAT, build 2e172ce0), the original `rbev-busybox` handles all three
platforms natively.

## Acceptance Criteria

- rbev-busybox-amd64 and rbev-busybox-arm64 directories removed
- Inscribe succeeds with only the original multi-platform vessels
- No orphaned triggers blocking future depot operations

### e2e-slsa-provenance-test-run (₢AlAAa) [complete]

**[260306-1304] complete**

Run the SLSA provenance e2e test (rbtcsl_SlsaProvenance) against live GCP
infrastructure on depot demo1025. This exercises conjure → check consecrations
→ vouch (assert Level 3) → abjure on rbev-busybox (3-platform).

Fix any issues discovered during the run.

## Acceptance Criteria

- rbtcsl_provenance_tcase passes on rbev-busybox
- Any code fixes committed before wrapping

**[260306-1136] rough**

Run the SLSA provenance e2e test (rbtcsl_SlsaProvenance) against live GCP
infrastructure on depot demo1025. This exercises conjure → check consecrations
→ vouch (assert Level 3) → abjure on rbev-busybox (3-platform).

Fix any issues discovered during the run.

## Acceptance Criteria

- rbtcsl_provenance_tcase passes on rbev-busybox
- Any code fixes committed before wrapping

### rbscb-provenance-posture-update (₢AlAAM) [complete]

**[260306-1309] complete**

Update RBSCB-CloudBuildRoadmap.adoc with crystallized provenance posture based on
confirmed production results, including the -vouch verification artifact.

## Prerequisites

- ₢AlAAT (verify-multiplatform-provenance-e2e) complete
- ₢AlAAV (slsa-verification-tabtarget) complete
- ₢AlAAW (ark-vouch-artifact-and-spec) complete
- ₢AlAAX (cleanup-bifurcated-vessels) complete

## Scope

- Correct "SLSA v1.0 provenance is automatic" in Current Posture
- Update OCI Layout Bridge reference — fully superseded for all vessel types
- Update Deferred Items: cosign — not needed, CB-native SLSA sufficient
- Update Deferred Items: BinAuth — now achievable with CB-native SLSA
- Add single-arch provenance architecture section with confirmed 6-step pipeline
- Add multi-platform provenance architecture section with confirmed 9-step pipeline
- Document -vouch verification artifact as part of the provenance posture
- Document tag scheme: platform-transparent consumer tags, internal per-platform tags
- Reference all production build IDs
- No "we are still trying" language in the roadmap

## Acceptance Criteria

- RBSCB states only confirmed facts, not hypotheses
- Both pipeline architectures documented with build IDs
- -vouch artifact documented as post-build verification capability
- Deferred Items reflect current reality

**[260305-1619] rough**

Update RBSCB-CloudBuildRoadmap.adoc with crystallized provenance posture based on
confirmed production results, including the -vouch verification artifact.

## Prerequisites

- ₢AlAAT (verify-multiplatform-provenance-e2e) complete
- ₢AlAAV (slsa-verification-tabtarget) complete
- ₢AlAAW (ark-vouch-artifact-and-spec) complete
- ₢AlAAX (cleanup-bifurcated-vessels) complete

## Scope

- Correct "SLSA v1.0 provenance is automatic" in Current Posture
- Update OCI Layout Bridge reference — fully superseded for all vessel types
- Update Deferred Items: cosign — not needed, CB-native SLSA sufficient
- Update Deferred Items: BinAuth — now achievable with CB-native SLSA
- Add single-arch provenance architecture section with confirmed 6-step pipeline
- Add multi-platform provenance architecture section with confirmed 9-step pipeline
- Document -vouch verification artifact as part of the provenance posture
- Document tag scheme: platform-transparent consumer tags, internal per-platform tags
- Reference all production build IDs
- No "we are still trying" language in the roadmap

## Acceptance Criteria

- RBSCB states only confirmed facts, not hypotheses
- Both pipeline architectures documented with build IDs
- -vouch artifact documented as post-build verification capability
- Deferred Items reflect current reality

**[260305-1513] rough**

Update RBSCB-CloudBuildRoadmap.adoc with crystallized provenance posture based on
confirmed production results.

## Prerequisites

- ₢AlAAP (spec-single-arch-provenance) complete — confirmed
- ₢AlAAQ (experiment-multiplatform-slsa-provenance) complete — confirmed,
  multi-platform provenance validated (experiments 4-6)
- ₢AlAAT (verify-multiplatform-provenance-e2e) complete — production proof

## Scope

- Correct "SLSA v1.0 provenance is automatic" in Current Posture (line 29-33)
- Update OCI Layout Bridge reference (line 68-71) — fully superseded for all
  vessel types (single-arch proven ₢AlAAO, multi-platform proven ₢AlAAT)
- Update Deferred Items: cosign (157-166) — not needed, CB-native SLSA sufficient
- Update Deferred Items: BinAuth (168-171) — now achievable with CB-native SLSA
- Add single-arch provenance architecture section with confirmed pipeline:
  6-step pipeline + images: + VERIFIED, --load (not --push+pullback)
- Add multi-platform provenance architecture section:
  9-step pipeline (single-build reassembly, experiment 6 validated),
  per-platform SBOMs, multi-platform -about container, SLSA summary in build_info
- Document tag scheme: platform-transparent consumer tags, internal per-platform tags
- Reference experiment build IDs: single-arch (16d3b60f, fc36b970, 9180c42a),
  multi-platform (b3fd60c7, 8cd7b713, 6661d0cd), and ₢AlAAT production builds
- Reference memo-20260305-provenance-architecture-gap.md

## Bifurcated vessel cleanup

Explicitly owned by this pace:
- Remove `rbev-busybox-amd64` and `rbev-busybox-arm64` vessel directories
  (scaffolding for single-arch milestone, no longer needed)
- Clean up their triggers if still present after inscribe
- Retain original `rbev-busybox` with 3-platform config as the canonical vessel

## Acceptance Criteria

- RBSCB states only confirmed facts, not hypotheses
- Single-arch provenance path documented as current capability with build IDs
- Multi-platform provenance documented as current capability with build IDs
- Metadata container architecture (multi-platform -about, per-platform SBOMs,
  SLSA summary in build_info) documented
- Deferred Items reflect current reality
- Bifurcated vessel directories removed
- No "we are still trying" language in the roadmap

**[260305-1404] rough**

Update RBSCB-CloudBuildRoadmap.adoc with crystallized provenance posture based on
confirmed production results and verified single-arch SLSA.

## Prerequisites

- ₢AlAAP (spec-single-arch-provenance) complete — RBS0 and RBSOB updated with
  confirmed facts from live verification
- ₢AlAAQ (experiment-multiplatform-slsa-provenance) complete OR documented failure
  — roadmap needs to state multi-platform path as confirmed or blocked

₢AlAAO is complete: SLSA v1.0 Build Level 3 confirmed on three trigger-dispatched
builds (2026-03-05):
- rbev-busybox-amd64: build 16d3b60f, SLSA Level 3
- rbev-busybox-arm64: build fc36b970, SLSA Level 3
- trbim-macos: build 9180c42a, SLSA Level 3

## Scope

- Correct "SLSA v1.0 provenance is automatic" in Current Posture (line 29-33)
- Update OCI Layout Bridge reference (line 68-71) — superseded for single-arch
- Update Deferred Items: cosign (157-166) — not needed for single-arch SLSA
- Update Deferred Items: BinAuth (168-171) — now achievable with CB-native SLSA
- Add single-arch provenance architecture section with confirmed pipeline:
  6-step pipeline + images: + VERIFIED, --load (not --push+pullback)
- Document multi-platform provenance based on ₢AlAAQ results:
  success → describe rejoining path; failure → document constraints
- Reference memo-20260305-provenance-architecture-gap.md (Production Pipeline Results)

## Acceptance Criteria

- RBSCB states only confirmed facts, not hypotheses
- Single-arch provenance path documented as current capability with build IDs
- Multi-platform provenance documented based on ₢AlAAQ outcome
- Deferred Items reflect current reality
- No "we are still trying" language in the roadmap

**[260305-1226] rough**

Update RBSCB-CloudBuildRoadmap.adoc with crystallized provenance posture based on
confirmed experiment results and verified single-arch SLSA.

## Prerequisites

- ₢AlAAP (spec-single-arch-provenance) complete — RBS0 and RBSOB updated with
  confirmed facts from live verification
- ₢AlAAO (verify-single-arch-slsa-e2e) complete — SLSA Level 3 confirmed on
  trigger-dispatched builds for both amd64 and arm64

## Scope

- Correct "SLSA v1.0 provenance is automatic" in Current Posture (line 29-33)
- Update OCI Layout Bridge reference (line 68-71) — superseded for single-arch
- Update Deferred Items: cosign (157-166) — not needed for single-arch SLSA
- Update Deferred Items: BinAuth (168-171) — now achievable with CB-native SLSA
- Add single-arch provenance architecture section with confirmed pipeline
- Document multi-platform provenance as future work with known constraints
- Reference memo-20260305-provenance-architecture-gap.md as research source

## Acceptance Criteria

- RBSCB states only confirmed facts, not hypotheses
- Single-arch provenance path documented as current capability
- Multi-platform provenance documented as future work with clear constraints
- Deferred Items reflect current reality
- No "we are still trying" language in the roadmap

**[260304-2015] rough**

Update RBSCB-CloudBuildRoadmap.adoc with crystallized provenance posture based on
confirmed experiment results.

## Prerequisites

- test-buildx-push-gar (₢AlAAK), test-pullback-images-verified (₢AlAAL),
  and stitch-provenance-fix (₢AlAAJ) all complete

## Scope

- Correct "SLSA v1.0 provenance is automatic" in Current Posture (line 29-33)
- Update OCI Layout Bridge reference (line 68-71) with push path outcome
- Update Deferred Items: cosign (157-166) and BinAuth (168-171) based on results
- Add provenance architecture section if experiment results warrant it
- Reference memo-20260305-provenance-architecture-gap.md as research source

## Acceptance Criteria

- RBSCB states only confirmed facts, not hypotheses
- Deferred Items reflect current reality
- No "we are still trying" language in the roadmap

### curl-timeout-bounded-transport (₢AlAAH) [complete]

**[260304-1909] complete**

Add --connect-timeout and --max-time to all 26 actionable curl sites across 7 files, using new RBCC kindle constants.

## Problem

Curl calls have no timeout. The webhook secret curl hung indefinitely during depot create (PID sat for minutes until manually killed). The OAuth refresh curl uses -s (not -sS) so failures produce no diagnostic output.

## Solution

Add two kindle constants to rbcc_Constants.sh:
```
readonly RBCC_CURL_CONNECT_TIMEOUT_SEC=10
readonly RBCC_CURL_MAX_TIME_SEC=60
```

Then add `--connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" --max-time "${RBCC_CURL_MAX_TIME_SEC}"` to all 26 curl invocations across these 7 files:

| File | Curl count | Notes |
|------|-----------|-------|
| rbgu_Utility.sh | 4 | rbgu_http_json + rbgu_http_json_remit (2 each: body/no-body) |
| rbgp_Payor.sh | 2 | OAuth refresh (zrbgp_refresh_capture, zrbgp_authorization_capture); also fix -s to -sS |
| rbgo_OAuth.sh | 1 | OAuth token exchange |
| rbf_Foundry.sh | 12 | Trigger dispatch, delete, crane downloads, HEAD checks |
| rbi_Image.sh | 2 | Image operations |
| rbrr_cli.sh | 3 | Crane downloads, registry check |
| rbap_AccessProbe.sh | 2 | Access probe checks |

## Additional fix

Change `-s` to `-sS` in rbgp_Payor.sh OAuth curls (lines 95, 469) so curl errors reach stderr.

## Effect

- Curl exit code 28 (timeout) is already retryable in rbgu_http_json retry loop
- Worst case: 3 retries x 60s = 180s bounded failure, not infinite hang
- OAuth curls fail fast (30s max) with visible error output

## Not in scope

- Test file curls (rbts/): different context
- VM string curl (rbv_PodmanVM.sh): runs inside VM

**[260304-1855] rough**

Add --connect-timeout and --max-time to all 26 actionable curl sites across 7 files, using new RBCC kindle constants.

## Problem

Curl calls have no timeout. The webhook secret curl hung indefinitely during depot create (PID sat for minutes until manually killed). The OAuth refresh curl uses -s (not -sS) so failures produce no diagnostic output.

## Solution

Add two kindle constants to rbcc_Constants.sh:
```
readonly RBCC_CURL_CONNECT_TIMEOUT_SEC=10
readonly RBCC_CURL_MAX_TIME_SEC=60
```

Then add `--connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" --max-time "${RBCC_CURL_MAX_TIME_SEC}"` to all 26 curl invocations across these 7 files:

| File | Curl count | Notes |
|------|-----------|-------|
| rbgu_Utility.sh | 4 | rbgu_http_json + rbgu_http_json_remit (2 each: body/no-body) |
| rbgp_Payor.sh | 2 | OAuth refresh (zrbgp_refresh_capture, zrbgp_authorization_capture); also fix -s to -sS |
| rbgo_OAuth.sh | 1 | OAuth token exchange |
| rbf_Foundry.sh | 12 | Trigger dispatch, delete, crane downloads, HEAD checks |
| rbi_Image.sh | 2 | Image operations |
| rbrr_cli.sh | 3 | Crane downloads, registry check |
| rbap_AccessProbe.sh | 2 | Access probe checks |

## Additional fix

Change `-s` to `-sS` in rbgp_Payor.sh OAuth curls (lines 95, 469) so curl errors reach stderr.

## Effect

- Curl exit code 28 (timeout) is already retryable in rbgu_http_json retry loop
- Worst case: 3 retries x 60s = 180s bounded failure, not infinite hang
- OAuth curls fail fast (30s max) with visible error output

## Not in scope

- Test file curls (rbts/): different context
- VM string curl (rbv_PodmanVM.sh): runs inside VM

### rbgu-http-remit-functions (₢AlAAG) [complete]

**[260304-1830] complete**

Add rbgu_http_json_remit and rbgu_http_ok_remit as new standalone functions using BCG _remit pattern. Also fix the broken legacy rbgu_http_json.

## What changed (this pace, so far)

BCG now defines the `_remit` function pattern:
- `BUC_REMIT_VALID`, `BUC_REMIT_DELIMITER`, `buc_remit_assert` in buc_command.sh
- Full documentation, contract, examples (fixed-arity + variable-length) in BCG
- Error handling decision table, checklists updated

## Remaining work

### 0. Fix broken legacy rbgu_http_json (BUG FIX)

rbgu_http_json is currently broken on main. The retry commit (39b5695f) writes to
`{infix}_{N}` paths but capture functions reconstruct from bare `{infix}`. EVERY
call is broken — not just retries — because `_1` is always appended even on first
attempt. All 90+ direct callers are affected.

Fix: after the retry loop succeeds, `cp` the successful attempt's files to the
bare-infix paths. Two lines. Per-attempt `_{N}` files survive for forensics.
Capture functions find what they expect. Zero caller changes.

### 1. Add `rbgu_http_json_remit(method, url, token, infix, [body_file])`

New function in rbgu_Utility.sh:
- Independent implementation (copy curl+retry logic, not a wrapper)
- Writes to bare `{infix}` paths (no `_{N}` suffix — new function, no legacy)
- On success: emits `REMIT_OK|{code}|{resp_path}` via `printf '%s'` (no trailing newline)
- On failure: `return 1` (no buc_die — _remit contract)
- Logs forensic details via buc_log_args (writes to transcript, not stdout — safe in subshell)
- Curl stderr captured to temp file (forensic evidence, since buc_remit_assert
  die message won't carry curl-specific errors)

### 2. Add `rbgu_http_ok_remit(label, token, method, url, infix, body, [warn_code], [warn_msg])`

- Calls rbgu_http_json_remit internally, checks sentinel
- Checks code against 200/201/204: success -> emits `REMIT_OK|{code}|{resp_path}`
- warn_code match: emits `REMIT_OK|{code}|{resp_path}` — NO buc_warn inside.
  Callers receive the code and decide whether to warn.
- HTTP error: `return 1` (logs error details to transcript before returning)

### 3. Leave rbgu_http_json and all capture functions in place

Legacy callers work via the cp fix. New code uses _remit functions.

## Stdin body pattern (secret handling)

Some callers pipe secrets via stdin: `echo "${secret}" | rbgu_http_json ... "-"`
The remit version works because the pipe runs inside `$()`:
```
IFS="${BUC_REMIT_DELIMITER}" read -r z_remit_valid z_code z_resp \
  <<< "$(echo "${z_secret}" | rbgu_http_json_remit "POST" "${url}" "${token}" "infix" "-")"
buc_remit_assert "${z_remit_valid}" "context"
```
Verify this works during implementation.

## CRITICAL: Why _remit and not modified rbgu_http_json

`rbgu_http_json_remit` is called inside `$()` by callers. If it used `buc_die`,
the `exit 1` would terminate the subshell (not the script), stderr message reaches
user, but script continues with empty variables and no sentinel. This is WORSE
than the exit-status-swallowing problem. The _remit contract (return 1, no buc_die)
exists precisely for this reason. The absent sentinel is the failure signal.

## Key decisions (from design session)

- Legacy fix is `cp` only — minimal, no API change
- New _remit functions are independent implementations, not wrappers
- Caller pattern: `IFS="${BUC_REMIT_DELIMITER}" read -r z_remit_valid z_code z_resp <<< "$(rbgu_http_json_remit ...)"` then `buc_remit_assert "${z_remit_valid}" "context"`
- Callers access JSON fields directly: `jq -r '.field' "${z_resp}"`
- No buc_warn inside _remit functions — callers receive data and decide

## Not in scope

- Migrating existing rbgu_http_json callers (₢AkAAS on MVP heat)
- Retiring capture functions (₢AkAAT on MVP heat)
- Changing retry policy or HTTP semantics

**[260304-1824] rough**

Add rbgu_http_json_remit and rbgu_http_ok_remit as new standalone functions using BCG _remit pattern. Also fix the broken legacy rbgu_http_json.

## What changed (this pace, so far)

BCG now defines the `_remit` function pattern:
- `BUC_REMIT_VALID`, `BUC_REMIT_DELIMITER`, `buc_remit_assert` in buc_command.sh
- Full documentation, contract, examples (fixed-arity + variable-length) in BCG
- Error handling decision table, checklists updated

## Remaining work

### 0. Fix broken legacy rbgu_http_json (BUG FIX)

rbgu_http_json is currently broken on main. The retry commit (39b5695f) writes to
`{infix}_{N}` paths but capture functions reconstruct from bare `{infix}`. EVERY
call is broken — not just retries — because `_1` is always appended even on first
attempt. All 90+ direct callers are affected.

Fix: after the retry loop succeeds, `cp` the successful attempt's files to the
bare-infix paths. Two lines. Per-attempt `_{N}` files survive for forensics.
Capture functions find what they expect. Zero caller changes.

### 1. Add `rbgu_http_json_remit(method, url, token, infix, [body_file])`

New function in rbgu_Utility.sh:
- Independent implementation (copy curl+retry logic, not a wrapper)
- Writes to bare `{infix}` paths (no `_{N}` suffix — new function, no legacy)
- On success: emits `REMIT_OK|{code}|{resp_path}` via `printf '%s'` (no trailing newline)
- On failure: `return 1` (no buc_die — _remit contract)
- Logs forensic details via buc_log_args (writes to transcript, not stdout — safe in subshell)
- Curl stderr captured to temp file (forensic evidence, since buc_remit_assert
  die message won't carry curl-specific errors)

### 2. Add `rbgu_http_ok_remit(label, token, method, url, infix, body, [warn_code], [warn_msg])`

- Calls rbgu_http_json_remit internally, checks sentinel
- Checks code against 200/201/204: success -> emits `REMIT_OK|{code}|{resp_path}`
- warn_code match: emits `REMIT_OK|{code}|{resp_path}` — NO buc_warn inside.
  Callers receive the code and decide whether to warn.
- HTTP error: `return 1` (logs error details to transcript before returning)

### 3. Leave rbgu_http_json and all capture functions in place

Legacy callers work via the cp fix. New code uses _remit functions.

## Stdin body pattern (secret handling)

Some callers pipe secrets via stdin: `echo "${secret}" | rbgu_http_json ... "-"`
The remit version works because the pipe runs inside `$()`:
```
IFS="${BUC_REMIT_DELIMITER}" read -r z_remit_valid z_code z_resp \
  <<< "$(echo "${z_secret}" | rbgu_http_json_remit "POST" "${url}" "${token}" "infix" "-")"
buc_remit_assert "${z_remit_valid}" "context"
```
Verify this works during implementation.

## CRITICAL: Why _remit and not modified rbgu_http_json

`rbgu_http_json_remit` is called inside `$()` by callers. If it used `buc_die`,
the `exit 1` would terminate the subshell (not the script), stderr message reaches
user, but script continues with empty variables and no sentinel. This is WORSE
than the exit-status-swallowing problem. The _remit contract (return 1, no buc_die)
exists precisely for this reason. The absent sentinel is the failure signal.

## Key decisions (from design session)

- Legacy fix is `cp` only — minimal, no API change
- New _remit functions are independent implementations, not wrappers
- Caller pattern: `IFS="${BUC_REMIT_DELIMITER}" read -r z_remit_valid z_code z_resp <<< "$(rbgu_http_json_remit ...)"` then `buc_remit_assert "${z_remit_valid}" "context"`
- Callers access JSON fields directly: `jq -r '.field' "${z_resp}"`
- No buc_warn inside _remit functions — callers receive data and decide

## Not in scope

- Migrating existing rbgu_http_json callers (₢AkAAS on MVP heat)
- Retiring capture functions (₢AkAAT on MVP heat)
- Changing retry policy or HTTP semantics

**[260304-1817] rough**

Add rbgu_http_json_remit and rbgu_http_ok_remit as new standalone functions using BCG _remit pattern.

## What changed (this pace, so far)

BCG now defines the `_remit` function pattern:
- `BUC_REMIT_VALID`, `BUC_REMIT_DELIMITER`, `buc_remit_assert` in buc_command.sh
- Full documentation, contract, examples (fixed-arity + variable-length) in BCG
- Error handling decision table, checklists updated

## Remaining work

1. Add `rbgu_http_json_remit(method, url, token, infix, [body_file])` to rbgu_Utility.sh
   - Copy curl+retry logic from rbgu_http_json (independent implementation)
   - Writes to bare `{infix}` paths (no `_{N}` suffix — new function, no legacy)
   - On success: emits `REMIT_OK|{code}|{resp_path}` via `printf '%s'` (no trailing newline)
   - On failure: `return 1` (no buc_die — _remit contract)
   - Logs forensic details via buc_log_args (writes to transcript, not stdout — safe in subshell)
   - Curl stderr captured to temp file (forensic evidence, since buc_remit_assert
     die message won't carry curl-specific errors)

2. Add `rbgu_http_ok_remit(label, token, method, url, infix, body, [warn_code], [warn_msg])`
   - Calls rbgu_http_json_remit internally, checks sentinel
   - Checks code against 200/201/204: success -> emits `REMIT_OK|{code}|{resp_path}`
   - warn_code match: emits `REMIT_OK|{code}|{resp_path}` — NO buc_warn inside the
     function. The caller receives the code and decides whether to warn. Functions
     succeed or fail; they don't editorialize.
   - HTTP error: `return 1` (logs error details to transcript before returning)

3. Leave rbgu_http_json and all capture functions untouched — legacy callers unaffected

## Stdin body pattern (secret handling)

Some callers pipe secrets via stdin: `echo "${secret}" | rbgu_http_json ... "-"`
The remit version works because the pipe runs inside `$()`:
```
IFS="${BUC_REMIT_DELIMITER}" read -r z_remit_valid z_code z_resp \
  <<< "$(echo "${z_secret}" | rbgu_http_json_remit "POST" "${url}" "${token}" "infix" "-")"
buc_remit_assert "${z_remit_valid}" "context"
```
The pipe connects echo to rbgu_http_json_remit's stdin inside the subshell.
Curl reads the body from stdin via `-d @-`. Verify this works during implementation.

## CRITICAL: Why _remit and not modified rbgu_http_json

`rbgu_http_json_remit` is called inside `$()` by callers. If it used `buc_die`,
the `exit 1` would terminate the subshell (not the script), stderr message reaches
user, but script continues with empty variables and no sentinel. This is WORSE
than the exit-status-swallowing problem. The _remit contract (return 1, no buc_die)
exists precisely for this reason. The absent sentinel is the failure signal.

## Key decisions (from design session)

- No `cp` hack — new functions write to bare infix, old function keeps `_{N}`
- `rbgu_http_json_remit` is independent implementation, not a wrapper around legacy
- Caller pattern: `IFS="${BUC_REMIT_DELIMITER}" read -r z_remit_valid z_code z_resp <<< "$(rbgu_http_json_remit ...)"` then `buc_remit_assert "${z_remit_valid}" "context"`
- Callers access JSON fields directly: `jq -r '.field' "${z_resp}"`
- No buc_warn inside _remit functions — callers receive data and decide

## Not in scope

- Migrating existing rbgu_http_json callers (₢AkAAS on MVP heat)
- Retiring capture functions (₢AkAAT on MVP heat)
- Changing retry policy or HTTP semantics
- Resolving `|| true` suppression patterns (separate ₣Ak pace)
- LRO polling wrapper design (separate ₣Ak pace)

**[260304-1806] rough**

Add rbgu_http_json_remit and rbgu_http_ok_remit as new standalone functions using BCG _remit pattern.

## What changed (this pace, so far)

BCG now defines the `_remit` function pattern:
- `BUC_REMIT_VALID`, `BUC_REMIT_DELIMITER`, `buc_remit_assert` in buc_command.sh
- Full documentation, contract, examples (fixed-arity + variable-length) in BCG
- Error handling decision table, checklists updated

## Remaining work

1. Add `rbgu_http_json_remit(method, url, token, infix, [body_file])` to rbgu_Utility.sh
   - Copy curl+retry logic from rbgu_http_json (independent implementation)
   - Writes to bare `{infix}` paths (no `_{N}` suffix — new function, no legacy)
   - On success: emits `REMIT_OK|{code}|{resp_path}` via `printf '%s'` (no trailing newline)
   - On failure: `return 1` (no buc_die — _remit contract)
   - Logs forensic details via buc_log_args as current function does
   - Curl stderr captured to temp file (forensic evidence of transport failures,
     since buc_remit_assert die message won't carry curl-specific errors)

2. Add `rbgu_http_ok_remit(label, token, method, url, infix, body, [warn_code], [warn_msg])`
   - Calls rbgu_http_json_remit internally, checks sentinel
   - Checks code against 200/201/204: success → emits `REMIT_OK|{code}|{resp_path}`
   - warn_code match: emits `REMIT_OK|{code}|{resp_path}` (caller gets code, decides)
   - HTTP error: `return 1` (logs error details to transcript before returning)

3. Leave rbgu_http_json and all capture functions untouched — legacy callers unaffected

## CRITICAL: Why _remit and not modified rbgu_http_json

`rbgu_http_json_remit` is called inside `$()` by callers. If it used `buc_die`,
the `exit 1` would terminate the subshell (not the script), stderr message reaches
user, but script continues with empty variables and no sentinel. This is WORSE
than the exit-status-swallowing problem. The _remit contract (return 1, no buc_die)
exists precisely for this reason. The absent sentinel is the failure signal.

## Key decisions (from design session)

- No `cp` hack — new functions write to bare infix, old function keeps `_{N}`
- `rbgu_http_json_remit` is independent implementation, not a wrapper around legacy
- Caller pattern: `IFS="${BUC_REMIT_DELIMITER}" read -r z_remit_valid z_code z_resp <<< "$(rbgu_http_json_remit ...)"` then `buc_remit_assert "${z_remit_valid}" "context"`
- Callers access JSON fields directly: `jq -r '.field' "${z_resp}"`
- Curl stderr file path: use kindle constant prefix + infix discriminator

## Not in scope

- Migrating existing rbgu_http_json callers (₢AkAAS on MVP heat)
- Retiring capture functions (₢AkAAT on MVP heat)
- Changing retry policy or HTTP semantics

**[260304-1802] rough**

Add rbgu_http_json_remit and rbgu_http_ok_remit as new standalone functions using BCG _remit pattern.

## What changed (this pace, so far)

BCG now defines the `_remit` function pattern:
- `BUC_REMIT_VALID`, `BUC_REMIT_DELIMITER`, `buc_remit_assert` in buc_command.sh
- Full documentation, contract, examples (fixed-arity + variable-length) in BCG
- Error handling decision table, checklists updated

## Remaining work

1. Add `rbgu_http_json_remit(method, url, token, infix, [body_file])` to rbgu_Utility.sh
   - Copy curl+retry logic from rbgu_http_json (independent implementation)
   - Writes to bare `{infix}` paths (no `_{N}` suffix — new function, no legacy)
   - On success: emits `REMIT_OK|{code}|{resp_path}` via printf
   - On failure: return 1 (no buc_die — _remit contract)
   - Logs forensic details via buc_log_args as current function does

2. Add `rbgu_http_ok_remit(label, token, method, url, infix, body, [warn_code], [warn_msg])`
   - Calls rbgu_http_json_remit internally
   - Checks code against 200/201/204 (and optional warn_code)
   - On HTTP error: return 1
   - On success: emits `REMIT_OK|{code}|{resp_path}`

3. Leave rbgu_http_json and all capture functions untouched — legacy callers unaffected

## Key decisions (from design session)

- No `cp` hack — new functions write to bare infix, old function keeps `_{N}`
- `rbgu_http_json_remit` is independent implementation, not a wrapper around legacy
- Caller pattern: `IFS="${BUC_REMIT_DELIMITER}" read -r z_remit_valid z_code z_resp <<< "$(rbgu_http_json_remit ...)"` then `buc_remit_assert "${z_remit_valid}" "context"`
- Callers access JSON fields directly: `jq -r '.field' "${z_resp}"`

## Not in scope

- Migrating existing rbgu_http_json callers (future MVP pace)
- Retiring capture functions (future MVP pace)
- Changing retry policy or HTTP semantics

**[260304-1802] rough**

Add rbgu_http_json_remit and rbgu_http_ok_remit as new standalone functions using BCG _remit pattern.

## What changed (this pace, so far)

BCG now defines the `_remit` function pattern:
- `BUC_REMIT_VALID`, `BUC_REMIT_DELIMITER`, `buc_remit_assert` in buc_command.sh
- Full documentation, contract, examples (fixed-arity + variable-length) in BCG
- Error handling decision table, checklists updated

## Remaining work

1. Add `rbgu_http_json_remit(method, url, token, infix, [body_file])` to rbgu_Utility.sh
   - Copy curl+retry logic from rbgu_http_json (independent implementation)
   - Writes to bare `{infix}` paths (no `_{N}` suffix — new function, no legacy)
   - On success: emits `REMIT_OK|{code}|{resp_path}` via printf
   - On failure: return 1 (no buc_die — _remit contract)
   - Logs forensic details via buc_log_args as current function does

2. Add `rbgu_http_ok_remit(label, token, method, url, infix, body, [warn_code], [warn_msg])`
   - Calls rbgu_http_json_remit internally
   - Checks code against 200/201/204 (and optional warn_code)
   - On HTTP error: return 1
   - On success: emits `REMIT_OK|{code}|{resp_path}`

3. Leave rbgu_http_json and all capture functions untouched — legacy callers unaffected

## Key decisions (from design session)

- No `cp` hack — new functions write to bare infix, old function keeps `_{N}`
- `rbgu_http_json_remit` is independent implementation, not a wrapper around legacy
- Caller pattern: `IFS="${BUC_REMIT_DELIMITER}" read -r z_remit_valid z_code z_resp <<< "$(rbgu_http_json_remit ...)"` then `buc_remit_assert "${z_remit_valid}" "context"`
- Callers access JSON fields directly: `jq -r '.field' "${z_resp}"`

## Not in scope

- Migrating existing rbgu_http_json callers (future MVP pace)
- Retiring capture functions (future MVP pace)
- Changing retry policy or HTTP semantics

**[260304-1613] rough**

Refactor rbgu_http_json temp file handling to write-once discipline with BCG return pattern.

## Problem

rbgu_http_json writes temp files keyed by an "infix" string. 118 callers reconstruct
the same file paths via capture functions (rbgu_http_code_capture, rbgu_json_field_capture).
Adding curl retry (for transient transport errors like SSL_ERROR_SYSCALL) means multiple
file versions per call, but the path-reconstruction pattern leaks the suffix convention
across all 118 callers.

## Current state (half-baked — will break on retry)

- rbgu_http_json writes to `{infix}_{N}` per attempt (write-once, good)
- But capture functions reconstruct paths from bare infix (no `_N` suffix)
- If attempt 1 succeeds, files land at `{infix}_1` — callers look for `{infix}` — MISMATCH
- The retry code landed in commits 39b5695f and 7e6980aa but is NOT wired to callers
- **This must be fixed before the e2e run (₢AlAAA) or any transport retry will cause
  a confusing failure, and even non-retry calls are broken because `_1` suffix is always appended**

## CRITICAL: Read before coding

1. Read BCG temp-file-then-read pattern: `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`
   Search for line ~456: `read -r z_result < "${Z_MODULE_TEMP2}"` — this is the return pattern.
2. Read current rbgu_http_json: `Tools/rbw/rbgu_Utility.sh` ~line 282
3. Read the two capture functions being retired: same file ~lines 190 and 201
4. Grep for all callers: `rbgu_http_code_capture` and `rbgu_json_field_capture` across Tools/rbw/

## Design direction

Use BCG temp-file-then-read pattern. rbgu_http_json writes its return values
(HTTP code string, response file path) to a result temp file after the retry loop.
Callers read with `read -r z_code z_resp < "${ZRBGU_RESULT_FILE}"` or similar.

This eliminates:
- rbgu_http_code_capture (~118 call sites → read-r pattern)
- rbgu_json_field_capture (callers jq directly from z_resp)
- All file path reconstruction outside rbgu_http_json
- Any suffix/naming convention leaking beyond the function

The file path stitching lives in ONE place: rbgu_http_json's loop body.

## Internal callers to adapt FIRST (before the 118 external sites)

These functions inside rbgu_Utility.sh call rbgu_http_json and then use capture
functions. Fix these first since they establish the pattern for everything else:

- rbgu_http_require_ok (~line 376) — reads code via capture
- rbgu_http_json_lro_ok (~line 393) — calls http_json in a loop, reads code + JSON
- rbgu_poll_until_ok (~line 519) — calls http_json in a loop, reads code
- rbgu_newly_created_delay (~line 549) — reads code via capture
- Various helpers that use rbgu_json_field_capture

## Migration plan (external callers — 7 files)

Mechanical transform at each site. Per-file migration is independent (no cross-file deps):

| File | Approx sites | Notes |
|------|-------------|-------|
| rbgp_Payor.sh | ~47 | Largest — consider splitting |
| rbgg_Governor.sh | ~22 | |
| rbgu_Utility.sh | ~21 | Internal — do first |
| rbgi_IAM.sh | ~12 | |
| rbgb_Buckets.sh | ~7 | |
| rbf_Foundry.sh | ~5 | |
| rbga_ArtifactRegistry.sh | ~4 | |

Each file can be bridled as a sub-pace if desired. The transform is:
```
BEFORE:
  rbgu_http_json "METHOD" "${url}" "${token}" "infix" ["${body}"]
  z_code=$(rbgu_http_code_capture "infix") || z_code=""

AFTER:
  rbgu_http_json "METHOD" "${url}" "${token}" "infix" ["${body}"]
  read -r z_code z_resp < "${ZRBGU_RESULT_FILE}"   # or whatever the final convention is
```

## Concerns to resolve during pace

- Exact result file format: space-delimited single line? Two lines?
- ZRBGU_RESULT_FILE: kindle constant pointing to a fixed path that gets overwritten
  per call? (This is NOT a write-once file — it's a return-value channel, like a register.
  The per-attempt files remain write-once.)
- How rbgu_json_field_capture callers adapt: they need the resp file path to run jq.
  Do they inline `jq -r '.field' "${z_resp}"` or do we keep a helper?
- rbgu_write_vanilla_json and rbgu_jq_add_member_to_role_capture also read by infix

## Scope

1. Refactor rbgu_http_json to write result file with code + resp path after loop
2. Adapt internal rbgu callers (establish pattern)
3. Retire rbgu_http_code_capture and rbgu_json_field_capture
4. Migrate all external callers (mechanical, per-file)
5. Each curl attempt writes uniquely-suffixed files — no overwrites, no copies, no moves

## Not in scope

- Changing retry policy (3 attempts, 3s sleep, codes 7/28/35/56 — already landed)
- Changing any HTTP call semantics
- RBS0 spec changes beyond rbbc_call (already updated for retry)

**[260304-1611] rough**

Refactor rbgu_http_json temp file handling to write-once discipline with BCG return pattern.

## Problem

rbgu_http_json writes temp files keyed by an "infix" string. 118 callers reconstruct
the same file paths via capture functions (rbgu_http_code_capture, rbgu_json_field_capture).
Adding curl retry (for transient transport errors like SSL_ERROR_SYSCALL) means multiple
file versions per call, but the path-reconstruction pattern leaks the suffix convention
across all 118 callers.

## Current state (broken)

- rbgu_http_json writes to `{infix}_{N}` per attempt (write-once, good)
- But capture functions reconstruct paths from the original infix (no suffix)
- Callers have no way to find the right file without knowing the suffix convention
- This is a design debt from the original single-attempt architecture

## Design direction

Use BCG temp-file-then-read pattern (BCG line 456): rbgu_http_json writes its
return values (HTTP code, response file path) to a result temp file. Callers
read with `read -r z_code z_resp_file < "${ZRBGU_RESULT_FILE}"`.

This eliminates:
- rbgu_http_code_capture (118 call sites → read -r pattern)
- rbgu_json_field_capture (callers jq directly from z_resp_file)
- All path reconstruction outside rbgu_http_json
- Any suffix/naming convention leaking beyond the function

The file path stitching lives in ONE place: rbgu_http_json's loop body.

## Scope

1. Refactor rbgu_http_json to write result temp file with code + resp path
2. Retire rbgu_http_code_capture and rbgu_json_field_capture
3. Migrate all 118 callers to read-r pattern (mechanical, bridleable per-file)
4. Verify curl retry write-once discipline works end-to-end
5. Each attempt writes uniquely-suffixed files, no overwrites, no copies, no moves

## Concerns to resolve during pace

- Exact temp file format (space-delimited? line-per-value?)
- Whether ZRBGU_RESULT_FILE is a kindle constant or derived per-call
- How rbgu_http_json_lro_ok and rbgu_poll_until_ok adapt (they call rbgu_http_json in loops)
- Whether to split migration by file for parallel execution
- Interaction with the rbgu_write_vanilla_json and similar helpers that read infixes

## Not in scope

- Changing retry policy (3 attempts, 3s sleep, codes 7/28/35/56 — already landed)
- Changing any HTTP call semantics
- RBS0 spec changes beyond rbbc_call (already updated for retry)

### retire-rbrr-validator (₢AlAAE) [complete]

**[260304-1516] complete**

Retire rbrr.validator.sh by migrating its two regex checks into buv_* enrollment types.

## Context

rbrr.validator.sh is a pre-BCG legacy validator with 5 checks. Three are already
covered by buv_*_enroll in rbrr_regime.sh. Two regex checks remain:

1. RBRR_GCB_MACHINE_TYPE: CE format ^[a-z][a-z0-9-]+$
   FIX: Change from buv_string_enroll to buv_gname_enroll in rbrr_regime.sh.
   buv_gname validates ^[a-z][a-z0-9-]*[a-z0-9]$ which is stricter (no trailing hyphen).

2. RBRR_GCB_WORKER_POOL: resource path ^projects/[^/]+/locations/[^/]+/workerPools/[^/]+$
   FIX: Needs either a new buv enroll type or a post-enrollment validation call.
   Design decision: new buv type vs inline regex in rbrr_regime.sh.

## Deliverables

1. Change RBRR_GCB_MACHINE_TYPE to buv_gname_enroll in rbrr_regime.sh
2. Add worker pool path validation (decide approach during pace)
3. Delete rbrr.validator.sh
4. Remove source call from rbgd_DepotConstants.sh
5. Verify depot_create still works (the validator was a pre-operation gate)

## Sequencing

Do this BEFORE the next e2e test run (₢AlAAA).

**[260304-1459] rough**

Retire rbrr.validator.sh by migrating its two regex checks into buv_* enrollment types.

## Context

rbrr.validator.sh is a pre-BCG legacy validator with 5 checks. Three are already
covered by buv_*_enroll in rbrr_regime.sh. Two regex checks remain:

1. RBRR_GCB_MACHINE_TYPE: CE format ^[a-z][a-z0-9-]+$
   FIX: Change from buv_string_enroll to buv_gname_enroll in rbrr_regime.sh.
   buv_gname validates ^[a-z][a-z0-9-]*[a-z0-9]$ which is stricter (no trailing hyphen).

2. RBRR_GCB_WORKER_POOL: resource path ^projects/[^/]+/locations/[^/]+/workerPools/[^/]+$
   FIX: Needs either a new buv enroll type or a post-enrollment validation call.
   Design decision: new buv type vs inline regex in rbrr_regime.sh.

## Deliverables

1. Change RBRR_GCB_MACHINE_TYPE to buv_gname_enroll in rbrr_regime.sh
2. Add worker pool path validation (decide approach during pace)
3. Delete rbrr.validator.sh
4. Remove source call from rbgd_DepotConstants.sh
5. Verify depot_create still works (the validator was a pre-operation gate)

## Sequencing

Do this BEFORE the next e2e test run (₢AlAAA).

### restructure-cbv2-gitlab-to-github (₢AlAAC) [abandoned]

**[260303-1956] abandoned**

Restructure CB v2 constants from GitLab three-secret model to GitHub single-PAT model.

## What changes

In `rbgc_Constants.sh`:
- `RBGC_CBV2_API_TOKEN_SECRET_NAME="rbw-gitlab-api-token"` → `"rb-github-pat"`
- `RBGC_CBV2_READ_TOKEN_SECRET_NAME` → **delete**
- `RBGC_CBV2_WEBHOOK_SECRET_NAME` → **delete**
- `RBGC_CBV2_CONNECTION_SUFFIX="-gitlab"` → `"-github"`
- Update comment block (line 151)

## Consumers to update

Mechanical search-and-destroy for deleted constants across:
- `rbgp_Payor.sh` — depot_create (3 secrets → 1), depot_destroy (3 deletes → 1)
- `rbf_Foundry.sh` — inscribe secret access (verify only uses API_TOKEN)
- `rbgg_Governor.sh` — create_director secret IAM (verify only uses API_TOKEN)
- `rbrr_regime.sh` / `rbrr.validator.sh` — remove enrollment of deleted variables
- `rbrr.env` — remove deleted variables
- `rbgm_ManualProcedures.sh` — update display text

## CB v2 connection body

`depot_create` constructs the CB v2 connection JSON body. Currently uses
`gitLabConfig` with `authorizerCredential`, `readAuthorizerCredential`, and
`webhookSecretSecretVersion`. Must change to `githubConfig` with
`authorizerCredential` and `appInstallationId`. The GitHub model is simpler
(one secret, no webhook secret, no read token).

## Acceptance criteria

- No "gitlab" string remains in `rbgc_Constants.sh` CB v2 section
- `depot_create` constructs `githubConfig` connection body
- Only one secret created/deleted in depot lifecycle
- Regime validates only the surviving variables
- Build + existing tests pass

**[260303-1942] rough**

Restructure CB v2 constants from GitLab three-secret model to GitHub single-PAT model.

## What changes

In `rbgc_Constants.sh`:
- `RBGC_CBV2_API_TOKEN_SECRET_NAME="rbw-gitlab-api-token"` → `"rb-github-pat"`
- `RBGC_CBV2_READ_TOKEN_SECRET_NAME` → **delete**
- `RBGC_CBV2_WEBHOOK_SECRET_NAME` → **delete**
- `RBGC_CBV2_CONNECTION_SUFFIX="-gitlab"` → `"-github"`
- Update comment block (line 151)

## Consumers to update

Mechanical search-and-destroy for deleted constants across:
- `rbgp_Payor.sh` — depot_create (3 secrets → 1), depot_destroy (3 deletes → 1)
- `rbf_Foundry.sh` — inscribe secret access (verify only uses API_TOKEN)
- `rbgg_Governor.sh` — create_director secret IAM (verify only uses API_TOKEN)
- `rbrr_regime.sh` / `rbrr.validator.sh` — remove enrollment of deleted variables
- `rbrr.env` — remove deleted variables
- `rbgm_ManualProcedures.sh` — update display text

## CB v2 connection body

`depot_create` constructs the CB v2 connection JSON body. Currently uses
`gitLabConfig` with `authorizerCredential`, `readAuthorizerCredential`, and
`webhookSecretSecretVersion`. Must change to `githubConfig` with
`authorizerCredential` and `appInstallationId`. The GitHub model is simpler
(one secret, no webhook secret, no read token).

## Acceptance criteria

- No "gitlab" string remains in `rbgc_Constants.sh` CB v2 section
- `depot_create` constructs `githubConfig` connection body
- Only one secret created/deleted in depot lifecycle
- Regime validates only the surviving variables
- Build + existing tests pass

**[260303-1936] rough**

Restructure CB v2 constants from GitLab three-secret model to GitHub single-PAT model.

## What changes

In `rbgc_Constants.sh`:
- `RBGC_CBV2_API_TOKEN_SECRET_NAME="rbw-gitlab-api-token"` → `"rb-github-pat"`
- `RBGC_CBV2_READ_TOKEN_SECRET_NAME` → **delete**
- `RBGC_CBV2_WEBHOOK_SECRET_NAME` → **delete**
- `RBGC_CBV2_CONNECTION_SUFFIX="-gitlab"` → `"-github"`
- Update comment block (line 151)

## Consumers to update

Mechanical search-and-destroy for deleted constants across:
- `rbgp_Payor.sh` — depot_create (3 secrets → 1), depot_destroy (3 deletes → 1)
- `rbf_Foundry.sh` — inscribe secret access (verify only uses API_TOKEN)
- `rbgg_Governor.sh` — create_director secret IAM (verify only uses API_TOKEN)
- `rbrr_regime.sh` / `rbrr.validator.sh` — remove enrollment of deleted variables
- `rbrr.env` — remove deleted variables
- `rbgm_ManualProcedures.sh` — update display text

## CB v2 connection body

`depot_create` constructs the CB v2 connection JSON body. Currently uses
`gitLabConfig` with `authorizerCredential`, `readAuthorizerCredential`, and
`webhookSecretSecretVersion`. Must change to `githubConfig` with
`authorizerCredential` and `appInstallationId`. The GitHub model is simpler
(one secret, no webhook secret, no read token).

## Acceptance criteria

- No "gitlab" string remains in `rbgc_Constants.sh` CB v2 section
- `depot_create` constructs `githubConfig` connection body
- Only one secret created/deleted in depot lifecycle
- Regime validates only the surviving variables
- Build + existing tests pass

### private-pool-always-burn-default-bridge (₢AlAAB) [complete]

**[260304-1121] complete**

Make private pool mandatory — burn the default-pool bridge.

## Decision: Machine Type at Pool Creation, Not Build Time (2026-03-03)

Two separate machine type systems exist in Cloud Build:
- **Default pool**: `options.machineType` (enum: `UNSPECIFIED`, `E2_HIGHCPU_8`, etc.)
- **Private pool**: `workerConfig.machineType` at pool creation (Compute Engine types:
  `e2-standard-2`, `e2-highcpu-32`, `n2d-*`, `c3-*`)

When using a private pool, `options.machineType` is NOT used — builds get whatever
machine type the pool was created with. The build config only needs `options.pool.name`.

**`RBRR_GCB_MACHINE_TYPE` survives but changes meaning:**
- Was: default-pool enum consumed at build time by stitch (`UNSPECIFIED`)
- Now: Compute Engine machine type consumed at pool creation by depot_create (`e2-standard-2`)
- Stitch no longer references it — stitch always emits `{ pool: { name: pool } }`

Sources:
- Private pool config schema: https://docs.cloud.google.com/build/docs/private-pools/private-pool-config-file-schema
- BuildOptions API: https://docs.cloud.google.com/build/docs/api/reference/rest/v1/projects.builds#buildoptions
- workerPools.create API: https://cloud.google.com/build/docs/api/reference/rest/v1/projects.locations.workerPools/create

## Regime

- `RBRR_GCB_WORKER_POOL` becomes **required** (1-512 chars, not 0-512)
- `RBRR_GCB_MACHINE_TYPE` survives, default changes from `UNSPECIFIED` to `e2-standard-2`
  — consumed at pool creation time, not build time
- Validator: `RBRR_GCB_MACHINE_TYPE` must look like a Compute Engine type (lowercase
  with hyphens), not a default-pool enum (uppercase with underscores)

## Stitch (rbf_Foundry.sh)

Remove the conditional:
```
if pool != "" then { pool: { name: pool } }
else { machineType: mtype } end
```
Replace with: always emit `{ pool: { name: pool } }`. Delete the `machineType`
jq arg from stitch entirely — stitch no longer needs `RBRR_GCB_MACHINE_TYPE`.

## Depot lifecycle (rbgp_Payor.sh)

**depot_create:** After API enablement, create worker pool:
- POST `cloudbuild.googleapis.com/v1/projects/{P}/locations/{R}/workerPools?workerPoolId={ID}`
- Body: `{ privatePoolV1Config: { workerConfig: { machineType: RBRR_GCB_MACHINE_TYPE } } }`
- Pool scales to zero (no idle cost)
- Idempotent: 409 CONFLICT = already exists = success

**depot_destroy:** Before project teardown, delete worker pool:
- DELETE `cloudbuild.googleapis.com/v1/projects/{P}/locations/{R}/workerPools/{ID}`
- Tolerate 404 (already gone)

## Quota check (rbgd_DepotConstants.sh)

Remove the default-pool quota check path entirely. Private pool quota is
managed on the pool's host project — the existing quota check logic for
default pools is dead code.

## Constants (rbgc_Constants.sh)

- Add `RBGC_WORKER_POOL_ID` constant (pool name within the project)
- Add API endpoint template for workerPools

## Specs

- **RBSDC-depot_create.adoc** — add pool creation step
- **RBSDD-depot_destroy.adoc** — add pool deletion step
- **RBS0 or subordinate .adoc** — document the private-pool-always decision chain:
  why default pool was eliminated, the two machine type systems, RBRR_GCB_MACHINE_TYPE
  meaning change (build-time enum → infrastructure-time CE type), pricing parity rationale.
  This is a durable architectural decision, not just a changelog entry.

## Files

- Tools/rbw/rbf_Foundry.sh — remove conditional + machineType arg, always pool
- Tools/rbw/rbgp_Payor.sh — pool create/delete
- Tools/rbw/rbgc_Constants.sh — pool constants
- Tools/rbw/rbgd_DepotConstants.sh — remove default-pool quota path
- Tools/rbw/rbrr_regime.sh — make pool required, validate CE machine type format
- .rbk/rbrr.env — update RBRR_GCB_MACHINE_TYPE default to e2-standard-2
- lenses/RBSDC-depot_create.adoc — pool creation step
- lenses/RBSDD-depot_destroy.adoc — pool deletion step
- lenses/RBS0-SpecTop.adoc or subordinate — decision documentation

## Acceptance criteria

- No default-pool code path remains in stitch or depot constants
- `RBRR_GCB_WORKER_POOL` is required (regime validator rejects empty)
- `RBRR_GCB_MACHINE_TYPE` default is `e2-standard-2`, validated as CE type
- Stitch emits only `pool.name`, never `machineType` in options
- depot_create creates pool using RBRR_GCB_MACHINE_TYPE, depot_destroy deletes pool
- Decision chain documented in spec (not just paddock)
- Build passes

**[260303-1942] rough**

Make private pool mandatory — burn the default-pool bridge.

## Decision: Machine Type at Pool Creation, Not Build Time (2026-03-03)

Two separate machine type systems exist in Cloud Build:
- **Default pool**: `options.machineType` (enum: `UNSPECIFIED`, `E2_HIGHCPU_8`, etc.)
- **Private pool**: `workerConfig.machineType` at pool creation (Compute Engine types:
  `e2-standard-2`, `e2-highcpu-32`, `n2d-*`, `c3-*`)

When using a private pool, `options.machineType` is NOT used — builds get whatever
machine type the pool was created with. The build config only needs `options.pool.name`.

**`RBRR_GCB_MACHINE_TYPE` survives but changes meaning:**
- Was: default-pool enum consumed at build time by stitch (`UNSPECIFIED`)
- Now: Compute Engine machine type consumed at pool creation by depot_create (`e2-standard-2`)
- Stitch no longer references it — stitch always emits `{ pool: { name: pool } }`

Sources:
- Private pool config schema: https://docs.cloud.google.com/build/docs/private-pools/private-pool-config-file-schema
- BuildOptions API: https://docs.cloud.google.com/build/docs/api/reference/rest/v1/projects.builds#buildoptions
- workerPools.create API: https://cloud.google.com/build/docs/api/reference/rest/v1/projects.locations.workerPools/create

## Regime

- `RBRR_GCB_WORKER_POOL` becomes **required** (1-512 chars, not 0-512)
- `RBRR_GCB_MACHINE_TYPE` survives, default changes from `UNSPECIFIED` to `e2-standard-2`
  — consumed at pool creation time, not build time
- Validator: `RBRR_GCB_MACHINE_TYPE` must look like a Compute Engine type (lowercase
  with hyphens), not a default-pool enum (uppercase with underscores)

## Stitch (rbf_Foundry.sh)

Remove the conditional:
```
if pool != "" then { pool: { name: pool } }
else { machineType: mtype } end
```
Replace with: always emit `{ pool: { name: pool } }`. Delete the `machineType`
jq arg from stitch entirely — stitch no longer needs `RBRR_GCB_MACHINE_TYPE`.

## Depot lifecycle (rbgp_Payor.sh)

**depot_create:** After API enablement, create worker pool:
- POST `cloudbuild.googleapis.com/v1/projects/{P}/locations/{R}/workerPools?workerPoolId={ID}`
- Body: `{ privatePoolV1Config: { workerConfig: { machineType: RBRR_GCB_MACHINE_TYPE } } }`
- Pool scales to zero (no idle cost)
- Idempotent: 409 CONFLICT = already exists = success

**depot_destroy:** Before project teardown, delete worker pool:
- DELETE `cloudbuild.googleapis.com/v1/projects/{P}/locations/{R}/workerPools/{ID}`
- Tolerate 404 (already gone)

## Quota check (rbgd_DepotConstants.sh)

Remove the default-pool quota check path entirely. Private pool quota is
managed on the pool's host project — the existing quota check logic for
default pools is dead code.

## Constants (rbgc_Constants.sh)

- Add `RBGC_WORKER_POOL_ID` constant (pool name within the project)
- Add API endpoint template for workerPools

## Specs

- **RBSDC-depot_create.adoc** — add pool creation step
- **RBSDD-depot_destroy.adoc** — add pool deletion step
- **RBS0 or subordinate .adoc** — document the private-pool-always decision chain:
  why default pool was eliminated, the two machine type systems, RBRR_GCB_MACHINE_TYPE
  meaning change (build-time enum → infrastructure-time CE type), pricing parity rationale.
  This is a durable architectural decision, not just a changelog entry.

## Files

- Tools/rbw/rbf_Foundry.sh — remove conditional + machineType arg, always pool
- Tools/rbw/rbgp_Payor.sh — pool create/delete
- Tools/rbw/rbgc_Constants.sh — pool constants
- Tools/rbw/rbgd_DepotConstants.sh — remove default-pool quota path
- Tools/rbw/rbrr_regime.sh — make pool required, validate CE machine type format
- .rbk/rbrr.env — update RBRR_GCB_MACHINE_TYPE default to e2-standard-2
- lenses/RBSDC-depot_create.adoc — pool creation step
- lenses/RBSDD-depot_destroy.adoc — pool deletion step
- lenses/RBS0-SpecTop.adoc or subordinate — decision documentation

## Acceptance criteria

- No default-pool code path remains in stitch or depot constants
- `RBRR_GCB_WORKER_POOL` is required (regime validator rejects empty)
- `RBRR_GCB_MACHINE_TYPE` default is `e2-standard-2`, validated as CE type
- Stitch emits only `pool.name`, never `machineType` in options
- depot_create creates pool using RBRR_GCB_MACHINE_TYPE, depot_destroy deletes pool
- Decision chain documented in spec (not just paddock)
- Build passes

**[260303-1937] rough**

Make private pool mandatory — burn the default-pool bridge.

## Regime

- `RBRR_GCB_WORKER_POOL` becomes **required** (1-512 chars, not 0-512)
- Evaluate whether `RBRR_GCB_MACHINE_TYPE` is still needed (pool config
  specifies machine type via `machineConfig`) — if dead, remove from regime

## Stitch (rbf_Foundry.sh)

Remove the conditional:
```
if pool != "" then { pool: { name: pool } }
else { machineType: mtype } end
```
Replace with: always emit `{ pool: { name: pool } }`. Delete the `machineType`
jq arg if confirmed dead.

## Depot lifecycle (rbgp_Payor.sh)

**depot_create:** After API enablement, create worker pool:
- POST `cloudbuild.googleapis.com/v1/projects/{P}/locations/{R}/workerPools?workerPoolId={ID}`
- Body: `{ privatePoolV1Config: { workerConfig: { machineType: "e2-standard-2" } } }`
- Pool scales to zero (no idle cost)
- Idempotent: 409 CONFLICT = already exists = success

**depot_destroy:** Before project teardown, delete worker pool:
- DELETE `cloudbuild.googleapis.com/v1/projects/{P}/locations/{R}/workerPools/{ID}`
- Tolerate 404 (already gone)

## Quota check (rbgd_DepotConstants.sh)

Remove the default-pool quota check path entirely. Private pool quota is
managed on the pool's host project — the existing quota check logic for
default pools is dead code.

## Constants (rbgc_Constants.sh)

- Add `RBGC_WORKER_POOL_ID` constant (pool name within the project)
- Add API endpoint template for workerPools

## Specs

- RBSDC-depot_create.adoc — add pool creation step
- RBSDD-depot_destroy.adoc — add pool deletion step
- RBS0 — update regime variable description (required, not optional)

## Files

- Tools/rbw/rbf_Foundry.sh — remove conditional, always pool
- Tools/rbw/rbgp_Payor.sh — pool create/delete
- Tools/rbw/rbgc_Constants.sh — pool constants
- Tools/rbw/rbgd_DepotConstants.sh — remove default-pool quota path
- Tools/rbw/rbrr_regime.sh — make pool required
- Tools/rbw/rbrr.env — update variable
- lenses/RBSDC-depot_create.adoc
- lenses/RBSDD-depot_destroy.adoc

## Acceptance criteria

- No default-pool code path remains in stitch or depot constants
- `RBRR_GCB_WORKER_POOL` is required (regime validator rejects empty)
- depot_create creates pool, depot_destroy deletes pool
- Build passes

**[260303-1937] rough**

Make private pool mandatory — burn the default-pool bridge.

## Regime

- `RBRR_GCB_WORKER_POOL` becomes **required** (1-512 chars, not 0-512)
- Evaluate whether `RBRR_GCB_MACHINE_TYPE` is still needed (pool config
  specifies machine type via `machineConfig`) — if dead, remove from regime

## Stitch (rbf_Foundry.sh)

Remove the conditional:
```
if pool != "" then { pool: { name: pool } }
else { machineType: mtype } end
```
Replace with: always emit `{ pool: { name: pool } }`. Delete the `machineType`
jq arg if confirmed dead.

## Depot lifecycle (rbgp_Payor.sh)

**depot_create:** After API enablement, create worker pool:
- POST `cloudbuild.googleapis.com/v1/projects/{P}/locations/{R}/workerPools?workerPoolId={ID}`
- Body: `{ privatePoolV1Config: { workerConfig: { machineType: "e2-standard-2" } } }`
- Pool scales to zero (no idle cost)
- Idempotent: 409 CONFLICT = already exists = success

**depot_destroy:** Before project teardown, delete worker pool:
- DELETE `cloudbuild.googleapis.com/v1/projects/{P}/locations/{R}/workerPools/{ID}`
- Tolerate 404 (already gone)

## Quota check (rbgd_DepotConstants.sh)

Remove the default-pool quota check path entirely. Private pool quota is
managed on the pool's host project — the existing quota check logic for
default pools is dead code.

## Constants (rbgc_Constants.sh)

- Add `RBGC_WORKER_POOL_ID` constant (pool name within the project)
- Add API endpoint template for workerPools

## Specs

- RBSDC-depot_create.adoc — add pool creation step
- RBSDD-depot_destroy.adoc — add pool deletion step
- RBS0 — update regime variable description (required, not optional)

## Files

- Tools/rbw/rbf_Foundry.sh — remove conditional, always pool
- Tools/rbw/rbgp_Payor.sh — pool create/delete
- Tools/rbw/rbgc_Constants.sh — pool constants
- Tools/rbw/rbgd_DepotConstants.sh — remove default-pool quota path
- Tools/rbw/rbrr_regime.sh — make pool required
- Tools/rbw/rbrr.env — update variable
- lenses/RBSDC-depot_create.adoc
- lenses/RBSDD-depot_destroy.adoc

## Acceptance criteria

- No default-pool code path remains in stitch or depot constants
- `RBRR_GCB_WORKER_POOL` is required (regime validator rejects empty)
- depot_create creates pool, depot_destroy deletes pool
- Build passes

**[260303-1814] rough**

Add Private Pool creation and deletion to the depot lifecycle.

## Regime variable

Add RBRR_GCB_PRIVATE_POOL_NAME to regime (optional, 0-64 chars).
Empty means default pool (no private pool). When set, depot_create
creates the pool and depot_destroy deletes it.

## Constants

Add RBGC_API_WORKER_POOLS endpoint template to rbgc_Constants.sh.

## depot_create

After API enablement, if RBRR_GCB_PRIVATE_POOL_NAME is non-empty:
- POST to workerPools API to create the pool
- Use e2 machine config from RBRR_GCB_MACHINE_TYPE
- Pool scales to zero (no idle cost)
- Idempotent: skip if pool already exists (409 CONFLICT = success)

## depot_destroy

Before project deletion, if RBRR_GCB_PRIVATE_POOL_NAME is non-empty:
- DELETE the worker pool
- Tolerate 404 (already gone)

## Quota check

rbgd_DepotConstants.sh: when private pool configured, check
concurrent_private_pool_build_cpus instead of concurrent_public_pool_build_cpus.

## Files

- Tools/rbw/rbrr_regime.sh — add optional enrollment
- Tools/rbw/rbrr.env — add variable
- Tools/rbw/rbgp_Payor.sh — pool create/delete
- Tools/rbw/rbgc_Constants.sh — API endpoint constant
- Tools/rbw/rbgd_DepotConstants.sh — conditional quota metric
- lenses/RBSDC-depot_create.adoc — document pool creation step
- lenses/RBSDD-depot_destroy.adoc — document pool deletion step

## Code Standards

All new bash code must be BCG-compliant.

### e2e-full-depot-lifecycle-provenance (₢AlAAA) [complete]

**[260305-1225] complete**

Full sequential e2e from clean state with SLSA v1.0 provenance verification.

## Prerequisites

- ₢AiABD (declarative IAM policy writes) landed in ₣Ai
- ₢AlAAB (private pool always) landed — pool creation in depot_create, pool-only stitch
- ₢AlAAE (retire-rbrr-validator) landed — buv_* enrollment is sole validation path
- All 7 vessel cloudbuild.json regenerated after skopeo split step addition
- Pin refresh will occur during depot create

## Recent changes landing with this e2e (first live test)

- Secret propagation polling: rbgu_poll_until_ok "POST" on getIamPolicy before
  IAM grants (fixes HTTP 404 on newly created secrets, cf3406a6)
- rbgu_poll_get_until_ok renamed to rbgu_poll_until_ok with method parameter
  (affects rbgp_Payor.sh, rbgg_Governor.sh, rbgu_Utility.sh)
- Payor UX: token hint shown before prompt, gitlab tokens URL via
  zrbgu_gitlab_tokens_url_capture (875c07b5)
- RBSCIP-IamPropagation.adoc updated with secret resource propagation pattern

## Procedure

Operator-guided, phase-gated:

**Phase 1: Destroy + Create**
1. Destroy existing demo1015 depot
2. Verify RBSDD spec steps match actual depot_destroy behavior (CB v2 repo, connection, secrets, pool — all explicitly deleted before project deletion) — update spec if gaps found
3. Create depot with CB v2 GitLab connection (project access token via stdin, 3 secrets: api, read_api, webhook)
4. Verify: 3 Secret Manager secrets created, CB v2 GitLab connection COMPLETE, CB v2 repository exists, private worker pool created
5. Update rbrr.env with new depot config (RBRR_DEPOT_PROJECT_ID, RBRR_GCB_WORKER_POOL, etc.)

**Phase 2: Roles + Pins**
6. Governor reset, create director, create retriever
7. Verify: Director SA has secretAccessor on GitLab API token secret (₢AiABD fix)
8. Refresh pins — verify RBRR_GCB_SKOPEO_IMAGE_REF gets real digest
9. Commit updated pins

**Phase 3: Inscribe + Dispatch**
10. Inscribe — verify no auto-fired builds (₢AiABC fix)
11. Dispatch busybox build via triggers.run
12. Wait for build success — all steps including skopeo split + SBOM
13. Verify build used private pool (options.pool.name in build config, not options.machineType)

**Phase 4: Provenance**
14. Query Container Analysis API for SLSA v1.0 provenance on image digest
15. Confirm inTotoSlsaProvenanceV1 exists
16. Confirm builder.id, source metadata, trigger URI present

**Phase 5: Fleet**
17. Dispatch additional vessels beyond busybox

## Acceptance Criteria

- Busybox build succeeds via triggers.run
- SLSA v1.0 provenance confirmed on produced artifact
- IAM bindings survive full lifecycle (no stale-read overwrites)
- No auto-fired builds during inscribe
- Skopeo split + Syft SBOM succeeds
- PAT never appears in logs or transcripts
- Private pool used for all builds (no default pool fallback)
- RBSDD spec updated to match actual destroy behavior
- Secret propagation polling succeeds (no 404 on getIamPolicy)

**[260304-1506] rough**

Full sequential e2e from clean state with SLSA v1.0 provenance verification.

## Prerequisites

- ₢AiABD (declarative IAM policy writes) landed in ₣Ai
- ₢AlAAB (private pool always) landed — pool creation in depot_create, pool-only stitch
- ₢AlAAE (retire-rbrr-validator) landed — buv_* enrollment is sole validation path
- All 7 vessel cloudbuild.json regenerated after skopeo split step addition
- Pin refresh will occur during depot create

## Recent changes landing with this e2e (first live test)

- Secret propagation polling: rbgu_poll_until_ok "POST" on getIamPolicy before
  IAM grants (fixes HTTP 404 on newly created secrets, cf3406a6)
- rbgu_poll_get_until_ok renamed to rbgu_poll_until_ok with method parameter
  (affects rbgp_Payor.sh, rbgg_Governor.sh, rbgu_Utility.sh)
- Payor UX: token hint shown before prompt, gitlab tokens URL via
  zrbgu_gitlab_tokens_url_capture (875c07b5)
- RBSCIP-IamPropagation.adoc updated with secret resource propagation pattern

## Procedure

Operator-guided, phase-gated:

**Phase 1: Destroy + Create**
1. Destroy existing demo1015 depot
2. Verify RBSDD spec steps match actual depot_destroy behavior (CB v2 repo, connection, secrets, pool — all explicitly deleted before project deletion) — update spec if gaps found
3. Create depot with CB v2 GitLab connection (project access token via stdin, 3 secrets: api, read_api, webhook)
4. Verify: 3 Secret Manager secrets created, CB v2 GitLab connection COMPLETE, CB v2 repository exists, private worker pool created
5. Update rbrr.env with new depot config (RBRR_DEPOT_PROJECT_ID, RBRR_GCB_WORKER_POOL, etc.)

**Phase 2: Roles + Pins**
6. Governor reset, create director, create retriever
7. Verify: Director SA has secretAccessor on GitLab API token secret (₢AiABD fix)
8. Refresh pins — verify RBRR_GCB_SKOPEO_IMAGE_REF gets real digest
9. Commit updated pins

**Phase 3: Inscribe + Dispatch**
10. Inscribe — verify no auto-fired builds (₢AiABC fix)
11. Dispatch busybox build via triggers.run
12. Wait for build success — all steps including skopeo split + SBOM
13. Verify build used private pool (options.pool.name in build config, not options.machineType)

**Phase 4: Provenance**
14. Query Container Analysis API for SLSA v1.0 provenance on image digest
15. Confirm inTotoSlsaProvenanceV1 exists
16. Confirm builder.id, source metadata, trigger URI present

**Phase 5: Fleet**
17. Dispatch additional vessels beyond busybox

## Acceptance Criteria

- Busybox build succeeds via triggers.run
- SLSA v1.0 provenance confirmed on produced artifact
- IAM bindings survive full lifecycle (no stale-read overwrites)
- No auto-fired builds during inscribe
- Skopeo split + Syft SBOM succeeds
- PAT never appears in logs or transcripts
- Private pool used for all builds (no default pool fallback)
- RBSDD spec updated to match actual destroy behavior
- Secret propagation polling succeeds (no 404 on getIamPolicy)

**[260303-2034] rough**

Full sequential e2e from clean state with SLSA v1.0 provenance verification.

## Prerequisites

- ₢AiABD (declarative IAM policy writes) landed in ₣Ai
- ₢AlAAB (private pool always) landed — pool creation in depot_create, pool-only stitch
- All 7 vessel cloudbuild.json regenerated after skopeo split step addition
- Pin refresh will occur during depot create

## Procedure

Operator-guided, phase-gated:

**Phase 1: Destroy + Create**
1. Destroy existing demo1015 depot
2. Verify RBSDD spec steps match actual depot_destroy behavior (CB v2 repo, connection, secrets, pool — all explicitly deleted before project deletion) — update spec if gaps found
3. Create depot with CB v2 GitLab connection (project access token via stdin, 3 secrets: api, read_api, webhook)
4. Verify: 3 Secret Manager secrets created, CB v2 GitLab connection COMPLETE, CB v2 repository exists, private worker pool created
5. Update rbrr.env with new depot config (RBRR_DEPOT_PROJECT_ID, RBRR_GCB_WORKER_POOL, etc.)

**Phase 2: Roles + Pins**
6. Governor reset, create director, create retriever
7. Verify: Director SA has secretAccessor on GitLab API token secret (₢AiABD fix)
8. Refresh pins — verify RBRR_GCB_SKOPEO_IMAGE_REF gets real digest
9. Commit updated pins

**Phase 3: Inscribe + Dispatch**
10. Inscribe — verify no auto-fired builds (₢AiABC fix)
11. Dispatch busybox build via triggers.run
12. Wait for build success — all steps including skopeo split + SBOM
13. Verify build used private pool (options.pool.name in build config, not options.machineType)

**Phase 4: Provenance**
14. Query Container Analysis API for SLSA v1.0 provenance on image digest
15. Confirm inTotoSlsaProvenanceV1 exists
16. Confirm builder.id, source metadata, trigger URI present

**Phase 5: Fleet**
17. Dispatch additional vessels beyond busybox

## Acceptance Criteria

- Busybox build succeeds via triggers.run
- SLSA v1.0 provenance confirmed on produced artifact
- IAM bindings survive full lifecycle (no stale-read overwrites)
- No auto-fired builds during inscribe
- Skopeo split + Syft SBOM succeeds
- PAT never appears in logs or transcripts
- Private pool used for all builds (no default pool fallback)
- RBSDD spec updated to match actual destroy behavior

**[260303-2026] rough**

Full sequential e2e from clean state with SLSA v1.0 provenance verification.

## Prerequisites

- ₢AiABD (declarative IAM policy writes) landed in ₣Ai
- ₢AlAAB (private pool always) landed — pool creation in depot_create, pool-only stitch
- All 7 vessel cloudbuild.json regenerated after skopeo split step addition
- Pin refresh will occur during depot create

## Procedure

Operator-guided, phase-gated:

**Phase 1: Destroy + Create**
1. Destroy existing demo1015 depot
2. Verify RBSDD spec steps match actual depot_destroy behavior (CB v2 repo, connection, secrets, pool — all explicitly deleted before project deletion)
3. Create depot with CB v2 GitLab connection (project access token via stdin, 3 secrets: api, read_api, webhook)
4. Verify: 3 Secret Manager secrets created, CB v2 GitLab connection COMPLETE, CB v2 repository exists, private worker pool created
5. Update rbrr.env with new depot config (RBRR_DEPOT_PROJECT_ID, RBRR_GCB_WORKER_POOL, etc.)

**Phase 2: Roles + Pins**
6. Governor reset, create director, create retriever
7. Verify: Director SA has secretAccessor on GitLab API token secret (₢AiABD fix)
8. Refresh pins — verify RBRR_GCB_SKOPEO_IMAGE_REF gets real digest
9. Commit updated pins

**Phase 3: Inscribe + Dispatch**
10. Inscribe — verify no auto-fired builds (₢AiABC fix)
11. Dispatch busybox build via triggers.run
12. Wait for build success — all steps including skopeo split + SBOM
13. Verify build used private pool (options.pool.name in build config, not options.machineType)

**Phase 4: Provenance**
14. Query Container Analysis API for SLSA v1.0 provenance on image digest
15. Confirm inTotoSlsaProvenanceV1 exists
16. Confirm builder.id, source metadata, trigger URI present

**Phase 5: Fleet**
17. Dispatch additional vessels beyond busybox

## Acceptance Criteria

- Busybox build succeeds via triggers.run
- SLSA v1.0 provenance confirmed on produced artifact
- IAM bindings survive full lifecycle (no stale-read overwrites)
- No auto-fired builds during inscribe
- Skopeo split + Syft SBOM succeeds
- PAT never appears in logs or transcripts
- Private pool used for all builds (no default pool fallback)
- RBSDD spec verified against actual destroy behavior

**[260303-1814] rough**

Full sequential e2e from clean state with SLSA v1.0 provenance verification.

## Prerequisites

- ₢AiABD (declarative IAM policy writes) landed in ₣Ai
- All 7 vessel cloudbuild.json regenerated after skopeo split step addition
- Pin refresh will occur during depot create

## Procedure

Operator-guided, phase-gated:

**Phase 1: Destroy + Create**
1. Destroy existing demo1015 depot
2. Create depot with CB v2 connection (PAT + installation ID via stdin)
3. Verify: Secret Manager secret, CB v2 connection COMPLETE, CB v2 repository exists

**Phase 2: Roles + Pins**
4. Governor reset, create director, create retriever
5. Verify: Director SA has secretAccessor on PAT secret (₢AiABD fix)
6. Refresh pins — verify RBRR_GCB_SKOPEO_IMAGE_REF gets real digest
7. Commit updated pins

**Phase 3: Inscribe + Dispatch**
8. Inscribe — verify no auto-fired builds (₢AiABC fix)
9. Dispatch busybox build via triggers.run
10. Wait for build success — all steps including skopeo split + SBOM

**Phase 4: Provenance**
11. Query Container Analysis API for SLSA v1.0 provenance on image digest
12. Confirm inTotoSlsaProvenanceV1 exists
13. Confirm builder.id, source metadata, trigger URI present

**Phase 5: Fleet**
14. Dispatch additional vessels beyond busybox

## Acceptance Criteria

- Busybox build succeeds via triggers.run
- SLSA v1.0 provenance confirmed on produced artifact
- IAM bindings survive full lifecycle (no stale-read overwrites)
- No auto-fired builds during inscribe
- Skopeo split + Syft SBOM succeeds
- PAT never appears in logs or transcripts

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 b fix-consecration-discovery-strong-tie
  2 I research-cloudbuild-provenance-mechanics
  3 K test-buildx-push-gar
  4 L test-pullback-images-verified
  5 J stitch-single-arch-slsa
  6 N bifurcate-busybox-single-arch
  7 O verify-single-arch-slsa-e2e
  8 P spec-single-arch-provenance
  9 Q experiment-multiplatform-slsa-provenance
  10 R spec-multiplatform-provenance
  11 U spec-multiplatform-metadata
  12 S stitch-multiplatform-provenance
  13 T verify-multiplatform-provenance-e2e
  14 V slsa-vouch-and-consecration-check
  15 W ark-vouch-artifact-and-spec
  16 Z fix-abjure-orphan-vouch-hardening
  17 Y test-all-slsa-busybox-provenance
  18 X cleanup-bifurcated-vessels
  19 a e2e-slsa-provenance-test-run
  20 M rbscb-provenance-posture-update
  21 H curl-timeout-bounded-transport
  22 G rbgu-http-remit-functions
  23 E retire-rbrr-validator
  24 C restructure-cbv2-gitlab-to-github
  25 B private-pool-always-burn-default-bridge
  26 A e2e-full-depot-lifecycle-provenance

bIKLJNOPQRUSTVWZYXaMHGECBA
x···x·x····x·x·xx·x·x···x· rbf_Foundry.sh
xx·····x·xx·x··x··x·····xx RBS0-SpecTop.adoc
·xx··xx·x················· memo-20260305-provenance-architecture-gap.md
······x·····x·····x······x cloudbuild.json
····················x···xx rbgp_Payor.sh
····················xx···x rbgu_Utility.sh
·······x·x··x············· RBSOB-oci_layout_bridge.adoc
······x·················xx rbrr.env
······x·············x····x rbrr_cli.sh
x···············x·······x· rbgc_Constants.sh
······················x·x· rbgd_DepotConstants.sh, rbrr.validator.sh, rbrr_regime.sh
················x·x······· rbtb_testbench.sh
x···············x········· rbtcsl_SlsaProvenance.sh
·························x RBSDI-director_create.adoc, RBSRC-retriever_create.adoc, rbgg_Governor.sh
························x· RBSDC-depot_create.adoc, RBSDD-depot_destroy.adoc, RBSQB-quota_build.adoc, RBSRR-RegimeRepo.adoc, rbgm_ManualProcedures.sh, rbrn_cli.sh
·····················x···· BCG-BashConsoleGuide.md, buc_command.sh
····················x····· rbap_AccessProbe.sh, rbcc_Constants.sh, rbgo_OAuth.sh, rbi_Image.sh
···················x······ RBSCB-CloudBuildRoadmap.adoc, vocjjmc_core.md
················x········· buto_operations.sh
··············x··········· JJS0-GallopsData.adoc, JJSCLD-landing.adoc, JJSCRS-restring.adoc, JJSCSL-slate.adoc, JJSRWP-wrap.adoc
·············x············ rbw-Dc.DirectorChecksConsecrations.sh, rbw-Rv.RetrieverVouchesArk.sh, rbz_zipper.sh
···········x·············· rbgjbm03-buildx-push-multi.sh, rbgjbm04-per-platform-pullback.sh, rbgjbm05-push-per-platform.sh, rbgjbm06-syft-per-platform.sh, rbgjbm07-build-info-per-platform.sh, rbgjbm08-buildx-push-about.sh, rbgjbm09-imagetools-create.sh
········x················· cloudbuild-test-multiplatform-provenance-varB.json, cloudbuild-test-multiplatform-reassembly.json
·····x···················· Dockerfile, proof.txt, rbrv.env
····x····················· rbgjb02-get-docker-token.sh, rbgjb02-qemu-binfmt.sh, rbgjb03-build-and-load.sh, rbgjb03-docker-login-gar.sh, rbgjb04-qemu-binfmt.sh, rbgjb04-sbom-and-summary.sh, rbgjb05-assemble-metadata.sh, rbgjb06-build-and-export.sh, rbgjb06-build-and-push-metadata.sh, rbgjb07-push-with-crane.sh, rbgjb07b-split-oci-platform.sh, rbgjb08-sbom-and-summary.sh, rbgjb09-build-and-push-metadata.sh, rbgjb10-assemble-metadata.sh
··x······················· cloudbuild-test-buildx-a.json, cloudbuild-test-buildx-b.json, cloudbuild-test-provenance.json
x························· rbgjb01-derive-tag-base.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 194 commits)

  1 W ark-vouch-artifact-and-spec
  2 Z fix-abjure-orphan-vouch-hardening
  3 Y test-all-slsa-busybox-provenance
  4 X cleanup-bifurcated-vessels
  5 M rbscb-provenance-posture-update
  6 a e2e-slsa-provenance-test-run
  7 b fix-consecration-discovery-strong-tie

123456789abcdefghijklmnopqrstuvwxyz
···x·xx····························  W  3c
········xxx························  Z  3c
···········xxxx····················  Y  4c
···············xx··················  X  2c
·················xx·············xxx  M  5c
····················xxxx···xxx·x···  a  8c
·························xx···x····  b  3c
```

## Steeplechase

### 2026-03-06 13:09 - ₢AlAAM - W

pace rbscb-provenance-posture-update complete

### 2026-03-06 13:09 - ₢AlAAM - n

Update vocjjmc_core.md to MCP tool interface: replace CLI/bash patterns with mcp__vvx__jjx tool calls, JSON params, and command reference table

### 2026-03-06 13:07 - ₢AlAAM - A

Update RBSCB with crystallized provenance posture: both pipelines, vouch artifact, tag scheme, deferred items — confirmed facts only

### 2026-03-06 13:04 - ₢AlAAa - W

pace e2e-slsa-provenance-test-run complete

### 2026-03-06 13:04 - ₢AlAAb - W

pace fix-consecration-discovery-strong-tie complete

### 2026-03-06 12:55 - ₢AlAAa - n

Fix build ID cross-check: strip Container Analysis URI prefix to bare UUID for comparison with conjure's dispatched build ID

### 2026-03-06 12:51 - ₢AlAAa - n

Regenerate all vessel cloudbuild.json: step 01 writes TAG_BASE to /builder/outputs/output, skopeo pin updated

### 2026-03-06 12:49 - ₢AlAAa - A

E2E re-run: pin refresh → inscribe → dispatch → verify rbtcsl_provenance_tcase with new strong-tie consecration + build ID cross-check

### 2026-03-06 12:45 - ₢AlAAb - n

Replace GAR tag scanning with strong build-ID tie: step 01 writes TAG_BASE to /builder/outputs/output, rbf_build reads from buildStepOutputs[0]; add RBF_FACT_BUILD_ID cross-check between conjure and vouch; update RBS0 spec

### 2026-03-06 12:38 - ₢AlAAb - A

Repair A: step 01 writes TAG_BASE to /builder/outputs/output, rbf_build reads from buildStepOutputs[0]. Repair B: RBF_FACT_BUILD_ID constant, build/vouch emit fact, test asserts match. Spec: RBS0 step output + build-ID cross-check.

### 2026-03-06 12:35 - Heat - S

fix-consecration-discovery-strong-tie

### 2026-03-06 11:58 - ₢AlAAa - n

Document SLSA level inference limitation in RBS0 vouch section: Container Analysis has no explicit field, slsa-verifier integration deferred

### 2026-03-06 11:54 - ₢AlAAa - n

Fix SLSA level extraction: infer from inTotoSlsaProvenanceV1 presence (Level 3) since Container Analysis API has no explicit field; fix predicate type and build ID extraction to check both v0.1 and v1 paths; add zrbgc_kindle to slsa-provenance fixture setup

### 2026-03-06 11:44 - ₢AlAAa - n

Split slsa-provenance fixture setup to target rbev-busybox (3-platform) instead of trbim-macos

### 2026-03-06 11:42 - ₢AlAAa - A

Run rbtcsl_SlsaProvenance on rbev-busybox against demo1025; diagnose-fix-retry loop, operator-gated

### 2026-03-06 11:36 - Heat - S

e2e-slsa-provenance-test-run

### 2026-03-06 11:32 - ₢AlAAM - n

Crystallize RBSCB provenance posture: SLSA Level 3 verified with build IDs, both pipeline architectures, tag scheme, -vouch artifact, OCI bridge superseded, deferred items updated

### 2026-03-06 11:30 - ₢AlAAM - A

Read RBSCB, update posture/pipelines/vouch/tags/deferred with confirmed production facts

### 2026-03-06 11:29 - ₢AlAAX - W

Removed bifurcated busybox vessel directories (amd64 + arm64), original 3-platform rbev-busybox retained

### 2026-03-06 11:13 - ₢AlAAX - A

Delete bifurcated busybox vessel dirs, verify no stale references, confirm original rbev-busybox intact

### 2026-03-06 11:11 - ₢AlAAY - W

Review and BCG fix for SLSA provenance test infrastructure: verified ZBUTO_BURV_OUTPUT scoping, image-ref string matching, min-level tracker, fact-file path coupling, positional arg alignment; fixed 3 local+file-read splits

### 2026-03-06 11:09 - ₢AlAAY - n

BCG fix: split local declaration from file-read assignment in 3 instances so set -e can catch failures

### 2026-03-06 11:01 - ₢AlAAY - n

SLSA provenance test infrastructure: ZBUTO_BURV_OUTPUT in buto, RBF_FACT_ constants, fact-file writes in rbf_build/rbf_vouch, new rbtcsl_SlsaProvenance test case

### 2026-03-06 10:54 - ₢AlAAY - A

Sequential: (1) ZBUTO_BURV_OUTPUT in buto_operations, (2) RBF_FACT_ constants in rbgc, (3) fact-file writes in rbf_build/rbf_vouch, (4) new rbtcsl_SlsaProvenance.sh test case

### 2026-03-06 10:52 - ₢AlAAZ - W

Abjure: vessel_dir + per-platform tag deletion; vouch: die on missing tags; RBS0: tag examples corrected

### 2026-03-06 10:52 - ₢AlAAZ - n

Fix abjure orphaned suffixed image tags (vessel_dir + platform-aware deletion), harden vouch to die on missing platform tags, update RBS0 consecration tag examples

### 2026-03-06 10:40 - ₢AlAAZ - A

Items 1+2 (abjure orphan tags, vouch die-on-missing) sequential in main context on rbf_Foundry.sh; item 3 (RBS0 tag examples) delegated to subagent

### 2026-03-06 10:32 - Heat - r

moved AlAAZ to first

### 2026-03-06 10:27 - ₢AlAAW - W

Implemented -vouch ark artifact: spec terms, vouch container build+push, stitch tag unification (always-suffixed images:), summon/abjure/check tag fixes, build consecration output to BURD_OUTPUT_DIR

### 2026-03-06 10:27 - ₢AlAAW - n

Revise Gallops spec for MCP-only transport: add jjdx_ executor terms, MCP Transport section, convert stdin to structured tool parameters, remove --file exposure, update Upper API and operation signatures for tool-based invocation

### 2026-03-06 10:26 - Heat - S

fix-abjure-orphan-vouch-hardening

### 2026-03-06 09:22 - ₢AlAAW - A

Sequential: (1) RBGC constant, (2) RBS0 spec terms, (3) vouch container build+push in Foundry, (4) wire into rbw-Rv tabtarget

### 2026-03-06 09:19 - Heat - T

test-all-slsa-busybox-provenance

### 2026-03-06 09:03 - Heat - T

test-all-slsa-busybox-provenance

### 2026-03-06 08:17 - Heat - S

test-all-slsa-busybox-provenance

### 2026-03-06 08:12 - ₢AlAAV - W

BCG compliance: load-then-iterate, remove dead jq code, clean redirect patterns

### 2026-03-06 08:12 - ₢AlAAV - n

Fix BCG violations in Foundry: piped while-read to load-then-iterate, simplify SLSA level extraction in vouch

### 2026-03-06 08:01 - ₢AlAAV - A

Fix BCG violations: piped while-read to load-then-iterate, add trailing-newline guards, remove dead jq code in vouch

### 2026-03-05 17:09 - ₢AlAAV - n

Scaffold rbw-Dc and rbw-Rv tabtargets, zipper enrollments, draft functions; paddock captures design decisions and BCG review needed

### 2026-03-05 16:54 - ₢AlAAV - A

Build both functions with Director auth; defer Retriever auth split to later

### 2026-03-05 16:50 - ₢AlAAV - A

Two tabtargets (rbw-Dc director, rbw-Rv retriever); IAM already done; key decision: Retriever CLI module structure

### 2026-03-05 16:46 - Heat - n

Grant Retriever containeranalysis.occurrences.viewer role for SLSA provenance verification

### 2026-03-05 16:46 - Heat - T

slsa-vouch-and-consecration-check

### 2026-03-05 16:45 - Heat - T

slsa-verification-tabtarget

### 2026-03-05 16:20 - ₢AlAAT - W

E2E verified: rbev-busybox 3-platform build 2e172ce0 achieved SLSA Level 3 on all per-platform images with shared buildInvocationId; spec aligned with production facts; RBSOB fully superseded; slated vouch/verification paces

### 2026-03-05 16:19 - Heat - T

rbscb-provenance-posture-update

### 2026-03-05 16:19 - Heat - S

cleanup-bifurcated-vessels

### 2026-03-05 16:15 - ₢AlAAT - n

Align RBS0 and RBSOB with production-validated multi-platform SLSA facts: update provenance status, ark_image definition, consecration tags, skopeo pin, single-arch pipeline steps, RBSOB full supersession

### 2026-03-05 16:14 - Heat - T

ark-vouch-artifact-and-spec

### 2026-03-05 16:13 - Heat - T

ark-vouch-artifact

### 2026-03-05 16:08 - Heat - S

ark-vouch-artifact

### 2026-03-05 16:07 - Heat - S

slsa-verification-tabtarget

### 2026-03-05 15:54 - ₢AlAAT - A

E2E verified: rbev-busybox 3-platform build 2e172ce0 — SLSA Level 3 on amd64/arm64/armv7, same buildInvocationId, consumer manifest list, -about metadata container

### 2026-03-05 15:40 - ₢AlAAT - n

Inscribe generates multi-platform cloudbuild.json for all multi-platform vessels

### 2026-03-05 15:35 - ₢AlAAS - W

Implemented multi-platform SLSA provenance pipeline: platform detection in stitch, 7 new step scripts (rbgjbm03-09), multi-platform images/substitutions/JSON, removed single-arch inscribe gate

### 2026-03-05 15:35 - ₢AlAAS - n

Add multi-platform Cloud Build pipeline with per-platform SBOM, build_info, and manifest list assembly (steps m03-m09); remove single-arch gate from stitch and inscribe

### 2026-03-05 15:33 - ₢AlAAS - L

sonnet landed

### 2026-03-05 15:23 - ₢AlAAS - F

Executing bridled pace via sonnet agent

### 2026-03-05 15:22 - ₢AlAAU - W

Updated rbtgi_metadata and rbtga_ark_about definitions with multi-platform metadata architecture; added build_info.json field inventory

### 2026-03-05 15:22 - ₢AlAAU - n

Update rbtgi_metadata and rbtga_ark_about definitions with multi-platform metadata architecture; add build_info.json field inventory

### 2026-03-05 15:20 - ₢AlAAU - A

Update rbtgi_metadata and rbtga_ark_about definitions with multi-platform metadata architecture; add build_info.json field inventory

### 2026-03-05 15:18 - Heat - S

spec-multiplatform-metadata

### 2026-03-05 15:16 - ₢AlAAS - B

arm | stitch-multiplatform-provenance

### 2026-03-05 15:16 - Heat - T

stitch-multiplatform-provenance

### 2026-03-05 15:13 - Heat - T

rbscb-provenance-posture-update

### 2026-03-05 15:13 - Heat - T

verify-multiplatform-provenance-e2e

### 2026-03-05 15:08 - Heat - T

stitch-multiplatform-provenance

### 2026-03-05 15:01 - Heat - T

stitch-multiplatform-provenance

### 2026-03-05 14:53 - Heat - T

stitch-multiplatform-provenance

### 2026-03-05 14:40 - ₢AlAAR - W

Updated RBS0 rbtgr_provenance with validated multi-platform architecture and target pipeline; updated RBSOB superseded notice; updated paddock

### 2026-03-05 14:40 - ₢AlAAR - n

Update rbtgr_provenance multi-platform section + RBSOB notice from experiment facts

### 2026-03-05 14:35 - ₢AlAAR - A

Update rbtgr_provenance multi-platform section + RBSOB notice from experiment facts

### 2026-03-05 14:33 - Heat - T

verify-multiplatform-provenance-e2e

### 2026-03-05 14:33 - Heat - T

stitch-multiplatform-provenance

### 2026-03-05 14:32 - Heat - T

spec-multiplatform-provenance

### 2026-03-05 14:30 - Heat - S

verify-multiplatform-provenance-e2e

### 2026-03-05 14:30 - Heat - S

stitch-multiplatform-provenance

### 2026-03-05 14:29 - Heat - S

spec-multiplatform-provenance

### 2026-03-05 14:28 - ₢AlAAQ - W

Multi-platform SLSA v1.0 Level 3 validated: 3/3 platforms attested with same buildInvocationId, manifest reassembly preserves provenance

### 2026-03-05 14:27 - ₢AlAAQ - n

Document multi-platform SLSA v1.0 Level 3 experiment results — 3/3 platforms verified, manifest reassembly proven

### 2026-03-05 14:09 - ₢AlAAQ - A

Interactive opus: Variant B first (manifest inspect + digest pull), fallback D/A, document all results

### 2026-03-05 14:08 - ₢AlAAP - W

pace spec-single-arch-provenance complete

### 2026-03-05 14:08 - ₢AlAAP - W

Updated RBS0 provenance definition and marked RBSOB superseded based on confirmed SLSA Level 3 results

### 2026-03-05 14:08 - ₢AlAAP - n

Update RBS0 provenance section for verified single-arch SLSA v1.0 Level 3 results and mark RBSOB superseded

### 2026-03-05 14:05 - ₢AlAAP - A

Update RBS0 provenance definition and mark RBSOB superseded based on confirmed SLSA Level 3 results

### 2026-03-05 14:04 - Heat - T

rbscb-provenance-posture-update

### 2026-03-05 14:00 - ₢AlAAO - W

Verified SLSA v1.0 Build Level 3 on 3 trigger-dispatched vessels (amd64, arm64, trbim-macos), fixed crane grep and inscribe multi-platform skip, documented results in memo and paddock

### 2026-03-05 13:59 - ₢AlAAO - n

Document production SLSA v1.0 Level 3 results in memo and paddock — 3/3 vessels verified

### 2026-03-05 13:36 - ₢AlAAO - n

Commit stitched cloudbuild.json for 3 single-arch vessels and refreshed skopeo pin

### 2026-03-05 13:36 - ₢AlAAO - n

Skip multi-platform vessels during inscribe enumeration instead of dying on stitch rejection

### 2026-03-05 13:33 - ₢AlAAO - n

Fix crane pin refresh grep to match readonly prefix in rbrr.env

### 2026-03-05 13:27 - ₢AlAAN - W

Created rbev-busybox-amd64 and rbev-busybox-arm64 vessels, documented bifurcation policy in paddock

### 2026-03-05 13:27 - ₢AlAAN - n

Copy busybox vessel pattern into two single-arch dirs (amd64/arm64), document policy for originals, update paddock

### 2026-03-05 13:23 - ₢AlAAN - A

Copy busybox vessel pattern into two single-arch dirs (amd64/arm64), document policy for originals, update paddock

### 2026-03-05 13:23 - Heat - T

experiment-multiplatform-slsa-provenance

### 2026-03-05 13:20 - ₢AlAAJ - W

Restructured stitch + step scripts for single-arch SLSA v1.0: deleted 4 steps (docker-login, get-token, crane, skopeo-split), replaced build-and-export with --load, reworked Syft to docker: transport, added images:+VERIFIED to stitch, single-arch gate, inscribe-timestamp image tag for CB images: compatibility

### 2026-03-05 13:20 - ₢AlAAJ - n

Delete 4 steps (02,03,07,07b), replace 06 with --load, rework 08+10, renumber to 01-06, add images:+VERIFIED to stitch, single-arch gate, remove crane/skopeo substitutions

### 2026-03-05 13:20 - Heat - T

verify-single-arch-slsa-e2e

### 2026-03-05 13:01 - ₢AlAAJ - A

Delete 4 steps (02,03,07,07b), replace 06 with --load, rework 08+10, renumber to 01-06, add images:+VERIFIED to stitch, single-arch gate, remove crane/skopeo substitutions

### 2026-03-05 12:47 - ₢AlAAQ - n

Cleanup: memo summary rewrite (lead with resolution), relocate experiment configs to Memos/experiments/, review+reslate AlAAQ (demote Variant C, add signing key verification, jq paths), review+reslate AlAAJ (Syft docker: transport, regime var disposition table, crane substitution removal)

### 2026-03-05 12:47 - Heat - T

stitch-single-arch-slsa

### 2026-03-05 12:46 - Heat - T

experiment-multiplatform-slsa-provenance

### 2026-03-05 12:38 - Heat - T

experiment-multiplatform-slsa-provenance

### 2026-03-05 12:35 - Heat - d

paddock curried

### 2026-03-05 12:33 - Heat - S

experiment-multiplatform-slsa-provenance

### 2026-03-05 12:26 - Heat - T

bifurcate-busybox-single-arch

### 2026-03-05 12:26 - Heat - T

stitch-single-arch-slsa

### 2026-03-05 12:26 - Heat - T

rbscb-provenance-posture-update

### 2026-03-05 12:25 - ₢AlAAA - W

Superseded by ₢AlAAO (verify-single-arch-slsa-e2e) which covers depot lifecycle + provenance verification

### 2026-03-05 12:19 - Heat - d

paddock curried

### 2026-03-05 12:18 - Heat - S

spec-single-arch-provenance

### 2026-03-05 12:17 - Heat - S

verify-single-arch-slsa-e2e

### 2026-03-05 12:17 - Heat - S

bifurcate-busybox-single-arch

### 2026-03-05 12:16 - Heat - T

stitch-single-arch-slsa

### 2026-03-05 12:12 - Heat - T

stitch-single-arch-slsa

### 2026-03-05 12:12 - Heat - T

stitch-provenance-fix

### 2026-03-05 12:09 - ₢AlAAL - W

Validated pullback+images+VERIFIED produces SLSA Build Level 3 with dual signatures. Provenance v0.1 and v1 both generated. Tag overwrite behavior documented — dual-tag scheme required for production.

### 2026-03-05 12:09 - ₢AlAAK - W

Validated buildx --push to GAR: both docker-login and ADC-only variants succeed. Memo updated with full experiment evidence.

### 2026-03-05 11:59 - ₢AlAAK - n

Experiments validated: buildx --push to GAR (with login and ADC-only), pullback+images+VERIFIED produces SLSA Build Level 3. Memo updated with full evidence trail.

### 2026-03-05 11:42 - ₢AlAAK - A

Interactive: craft inline cloudbuild.json for buildx --push variants A (docker login) and B (ADC only), submit via gcloud builds submit on demo1025, verify GAR multi-platform manifest

### 2026-03-05 11:41 - ₢AlAAI - W

Researched CB provenance gap: OCI Layout Bridge incompatible with native SLSA. Updated memo, RBS0 spec, paddock, and 4 downstream pace dockets with experiment plan.

### 2026-03-05 11:41 - ₢AlAAI - n

provenance memo: add buildx ADC evidence and sigstore key-based constraint

### 2026-03-04 20:28 - Heat - T

stitch-provenance-fix

### 2026-03-04 20:28 - Heat - T

test-pullback-images-verified

### 2026-03-04 20:27 - Heat - T

test-buildx-push-gar

### 2026-03-04 20:19 - ₢AlAAI - n

Provenance research: memo, RBS0 provenance definition, paddock update, experiment paces slated

### 2026-03-04 20:17 - Heat - T

test-pullback-images-verified

### 2026-03-04 20:17 - Heat - T

test-buildx-push-gar

### 2026-03-04 20:15 - Heat - S

rbscb-provenance-posture-update

### 2026-03-04 20:14 - Heat - S

test-pullback-images-verified

### 2026-03-04 20:14 - Heat - S

test-buildx-push-gar

### 2026-03-04 20:14 - Heat - T

stitch-provenance-fix

### 2026-03-04 19:43 - ₢AlAAI - A

Research provenance via GCP docs + live diagnostic commands, then update RBS0 and recommend stitch changes

### 2026-03-04 19:42 - Heat - T

stitch-provenance-fix

### 2026-03-04 19:42 - Heat - T

research-cloudbuild-provenance-mechanics

### 2026-03-04 19:41 - Heat - S

stitch-provenance-fix

### 2026-03-04 19:41 - Heat - S

research-cloudbuild-provenance-mechanics

### 2026-03-04 19:30 - ₢AlAAA - n

Add readonly to all rbrr.env assignments (prevents duplicate-override bugs); remove stale demo1015 worker pool; update depot_create guidance and pin refresh sed patterns to preserve readonly

### 2026-03-04 19:23 - ₢AlAAA - n

Regenerate all 7 vessel cloudbuild.json with updated skopeo pin digest

### 2026-03-04 19:22 - ₢AlAAA - n

Pin refresh: skopeo digest updated to c79ee77c

### 2026-03-04 19:16 - ₢AlAAA - n

Fix SA propagation poll: use email instead of numeric UID in shared create_service_account_with_key; update RBSDI and RBSRC specs to match

### 2026-03-04 19:10 - ₢AlAAA - A

Operator-guided 5-phase e2e: destroy/create demo1015 with private pool, roles+pins, inscribe+dispatch busybox, SLSA provenance, fleet

### 2026-03-04 19:09 - ₢AlAAH - W

Added curl timeout bounds (connect 10s, max 60s) to all 26 actionable curl sites across 7 files via RBCC kindle constants; fixed -s to -sS in OAuth curls

### 2026-03-04 19:05 - ₢AlAAH - n

Add curl timeout bounds (connect 10s, max 60s) to all 26 actionable curl sites across 7 files; fix -s to -sS in OAuth curls

### 2026-03-04 18:56 - ₢AlAAH - A

Add RBCC_CURL_CONNECT/MAX_TIME constants, apply to 26 curl sites in 7 files, fix -s to -sS in OAuth curls

### 2026-03-04 18:55 - Heat - S

curl-timeout-bounded-transport

### 2026-03-04 18:44 - ₢AlAAA - n

Update rbrr.env for demo1025 depot: project ID, GAR repo, CBv2 connection, worker pool

### 2026-03-04 18:30 - ₢AlAAG - W

Fix legacy rbgu_http_json bare-infix cp + add rbgu_http_json_remit and rbgu_http_ok_remit

### 2026-03-04 18:29 - ₢AlAAG - n

Fix broken legacy rbgu_http_json (cp to bare-infix paths) and add rbgu_http_json_remit + rbgu_http_ok_remit using BCG _remit pattern

### 2026-03-04 18:25 - ₢AlAAG - A

cp-fix for legacy + two new _remit functions (json_remit, ok_remit) in rbgu_Utility.sh

### 2026-03-04 18:24 - Heat - T

rbgu-http-remit-functions

### 2026-03-04 18:17 - Heat - T

rbgu-http-remit-functions

### 2026-03-04 18:06 - Heat - T

rbgu-http-remit-functions

### 2026-03-04 18:02 - Heat - T

rbgu-http-remit-functions

### 2026-03-04 18:02 - Heat - T

rbgu-http-json-write-once-cleanup

### 2026-03-04 17:49 - ₢AlAAG - n

Add _remit to error handling decision table; clarify return-1-on-failure contract for _remit functions

### 2026-03-04 17:35 - ₢AlAAG - n

Define _remit function pattern in BCG: sentinel-first structured return with BUC_REMIT_VALID, BUC_REMIT_DELIMITER, buc_remit_assert; add infrastructure to buc_command.sh

### 2026-03-04 16:29 - ₢AlAAG - A

Sequential sonnet: result-file register pattern, internal callers first, then 6 external files

### 2026-03-04 16:13 - Heat - T

rbgu-http-json-write-once-cleanup

### 2026-03-04 16:12 - ₢AlAAA - n

WIP: curl retry with write-once file suffixing, not yet wired to callers — cleanup slated as AlAAG

### 2026-03-04 16:11 - Heat - S

rbgu-http-json-write-once-cleanup

### 2026-03-04 15:51 - ₢AlAAA - n

Preserve per-attempt temp files on curl retry; copy successful attempt to base infix for caller compatibility

### 2026-03-04 15:48 - ₢AlAAA - n

Add transport-level retry (3 attempts, 3s sleep) for transient curl errors (7/28/35/56) in rbgu_http_json; update RBS0 rbbc_call definition

### 2026-03-04 15:38 - ₢AlAAA - n

Fix Secret Manager getIamPolicy: POST→GET across rbgp (6 calls) and rbgg (1 call), wrong method masked since c9d7efb0 by vanilla-json fallback

### 2026-03-04 15:38 - Heat - S

reconsider-poll-method-parameterization

### 2026-03-04 15:17 - ₢AlAAA - A

Operator-guided 5-phase e2e: destroy, create+pool, roles+pins, inscribe+dispatch+provenance, fleet

### 2026-03-04 15:16 - ₢AlAAE - W

Retired rbrr.validator.sh: machine type to buv_gname_enroll, worker pool path regex to zrbrr_enforce, removed source from rbgd

### 2026-03-04 15:16 - ₢AlAAE - n

Migrate RBRR validation into regime enrollment and enforcement, delete standalone validator

### 2026-03-04 15:15 - ₢AlAAE - A

gname for machine type, enforce regex for worker pool, delete validator, remove source from rbgd

### 2026-03-04 15:06 - Heat - T

e2e-full-depot-lifecycle-provenance

### 2026-03-04 14:59 - Heat - S

retire-rbrr-validator

### 2026-03-04 11:57 - Heat - n

secret propagation fix: refactor rbgu_poll_get_until_ok to rbgu_poll_until_ok with method param, add secret getIamPolicy polling before IAM grants, update RBSCIP spec

### 2026-03-04 11:57 - Heat - S

rbscip-linked-term-consideration

### 2026-03-04 11:36 - Heat - n

payor UX: show token hint before prompt, add gitlab tokens URL capture, clickable link in setup guide

### 2026-03-04 11:22 - ₢AlAAA - A

Operator-guided e2e: 5-phase gate-checked lifecycle with live GCP

### 2026-03-04 11:21 - ₢AlAAB - W

Spec repairs: RBSRR CE machine types, RBS0 private-pool decision chain + pricing + two-type-system, RBSQB private-pool quota rewrite, RBSDD deletion rationale

### 2026-03-04 11:21 - ₢AlAAB - n

Burn default-pool references: rewrite quota, machine-type, and concurrency docs for private-pool-only model

### 2026-03-03 20:34 - Heat - T

e2e-full-depot-lifecycle-provenance

### 2026-03-03 20:28 - ₢AlAAB - n

burn default-pool bridge: stitch pool-only, regime required+CE-validator, depot pool create/delete, quota check removed, specs updated, reviewed fixes applied

### 2026-03-03 20:26 - Heat - T

e2e-full-depot-lifecycle-provenance

### 2026-03-03 20:04 - ₢AlAAB - A

Burn default pool: stitch→pool-only, regime→required+CE-validator, depot-create→workerPool-create, depot-destroy→workerPool-delete, constants→pool-id+api-path, quota-check→remove-default-path, specs→decision-doc

### 2026-03-03 20:02 - Heat - n

repair paddock: delete bogus GitLab→GitHub migration section, fix constant refs, strengthen private-pool rationale with security-tier justification

### 2026-03-03 19:56 - Heat - T

restructure-cbv2-gitlab-to-github

### 2026-03-03 19:46 - ₢AlAAC - A

4-step: constants rename/delete, payor 3-to-1 secret + githubConfig, consumer comment updates, regime/manual guide rewrite

### 2026-03-03 19:42 - Heat - T

restructure-cbv2-gitlab-to-github

### 2026-03-03 19:42 - Heat - T

private-pool-always-burn-default-bridge

### 2026-03-03 19:37 - Heat - r

moved AlAAA to last

### 2026-03-03 19:37 - Heat - T

private-pool-always-burn-default-bridge

### 2026-03-03 19:37 - Heat - T

private-pool-depot-lifecycle

### 2026-03-03 19:36 - Heat - S

gitlab-to-github-secret-constants

### 2026-03-03 18:14 - Heat - S

private-pool-depot-lifecycle

### 2026-03-03 18:14 - Heat - S

e2e-full-depot-lifecycle-provenance

### 2026-03-03 18:13 - Heat - f

racing

### 2026-03-03 18:13 - Heat - N

rbw-e2e-cbv2-provenance

