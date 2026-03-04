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

**Build pipeline (10 steps, stitched by `zrbf_stitch_build_json`):**
1. derive-tag-base (gcloud)
2. get-docker-token (gcloud)
3. docker-login-gar (docker)
4. qemu-binfmt (docker)
5. build-and-export (docker) — OCI Layout Bridge Phase 1
6. push-with-crane (alpine) — OCI Layout Bridge Phase 2
7. **split-oci-platform (skopeo)** — extracts linux/amd64 from multi-platform layout
8. sbom-and-summary (docker) — Syft scans single-platform layout
9. assemble-metadata (alpine)
10. build-and-push-metadata (docker)

**Skopeo split (₢AiABE):** Step 07b uses skopeo to extract a single-platform
OCI layout from the multi-platform archive so Syft can generate SBOMs
(workaround for anchore/syft#1545). SBOM output is per-platform:
`sbom.linux_amd64.spdx.json`.

**Key regime variables:**
- `RBRR_RUBRIC_REPO_URL` — plain HTTPS URL to rubric repo (no credentials)
- `RBRR_CBV2_CONNECTION_NAME` — CB v2 connection identifier
- `RBRR_GCB_SKOPEO_IMAGE_REF` — skopeo image pin (needs refresh on depot create)
- PAT in Secret Manager (3 secrets: `RBGC_CBV2_API_TOKEN_SECRET_NAME`,
  `RBGC_CBV2_READ_TOKEN_SECRET_NAME`, `RBGC_CBV2_WEBHOOK_SECRET_NAME`)

### Decision: Private Pool Always (2026-03-03)

**Burn the default-pool bridge.** Every depot gets a private pool. No conditional
path in stitch, no "optional" regime variable, no two configurations to debug.

**Rationale:** Private pools are the prerequisite for higher security tiers.
VPC Service Controls (data exfiltration perimeter) **only works with private pools**.
NO_PUBLIC_EGRESS (build worker network isolation) **requires private pools**.
The roadmap (RBSCB) treats private pools as current posture and both hardening
tiers as future stages — every depot must be on private pools to keep that path open.

Additionally, pricing is essentially identical (~$0.003/vCPU-min, 1.0-1.13x ratio),
and the conditional in `zrbf_stitch_build_json` created two code paths, two depot
configurations, two things to debug. One path, well-tested, is better.

**What changed** _(completed by ₢AlAAB)_**:**
- `RBRR_GCB_WORKER_POOL` became **required** in regime (1-512 chars, not 0-512)
- `zrbf_stitch_build_json`: conditional removed, always emits `pool.name`
- `rbgd_DepotConstants.sh`: default-pool quota check path removed
- `depot_create`: always creates worker pool (workerPools API)
- `depot_destroy`: always deletes worker pool
- `RBRR_GCB_MACHINE_TYPE`: survives with changed meaning — Compute Engine type
  (e.g. `e2-standard-2`) consumed at pool creation time, not build-time enum

### Current State

**Depot demo1015** exists with CB v2 GitLab connection. Rubric repo at
`gitlab.com/bhyslop/rb-rubric.git`. 7 vessel triggers created.
**Will be destroyed and rebuilt on GitLab with private pool.**

**All known issues from ₣Ai e2e (2026-03-03) are FIXED:**
1. Push triggers fired on inscribe push — FIXED (₢AiABC: unmatchable branch filter)
2. IAM read-modify-write race — FIXED (₢AiABD: declarative policy writes)
3. Syft multi-platform OCI layout — FIXED (₢AiABE: skopeo split)
4. Build step `dir` field missing — FIXED in ₣Ai

### What This Heat Verifies

- Full lifecycle: destroy → create (with private pool) → roles → pins → inscribe → dispatch → provenance
- GitLab CB v2 connection (validated, per ₣Ai migration)
- Private pool created by depot_create, used by all builds
- SLSA v1.0 provenance exists on trigger-invoked builds
- IAM bindings survive the full depot lifecycle (declarative policy writes)
- Skopeo split + Syft SBOM succeeds on multi-platform builds
- No auto-fired builds during inscribe (unmatchable push filter)
- PAT never appears in logs or transcripts
- All 7 vessels inscribe and at least busybox dispatches successfully

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
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — BCG patterns
- CB v2 API: https://cloud.google.com/build/docs/api/reference/rest/v2/projects.locations.connections
- SLSA provenance: https://cloud.google.com/build/docs/securing-builds/generate-validate-build-provenance
