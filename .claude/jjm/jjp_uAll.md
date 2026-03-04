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
REST API. Uses "Google Cloud Build" GitHub/GitLab App + classic PAT stored in
Secret Manager. No browser OAuth consent flow. Connection + repository created
during `depot_create`.

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
- PAT in Secret Manager (`RBGC_CBV2_PAT_SECRET_NAME = rb-github-pat`)

### GitLab→GitHub Secret Constant Migration

**WARNING (from ₣Ai ₢AiABB):** `rbgc_Constants.sh` lines 151-157 contain four
GitLab-specific constants that must be restructured for GitHub CB v2:

- `RBGC_CBV2_API_TOKEN_SECRET_NAME="rbw-gitlab-api-token"` → rename to `"rb-github-pat"` (single PAT)
- `RBGC_CBV2_READ_TOKEN_SECRET_NAME="rbw-gitlab-read-token"` → **delete** (GitLab-only, GitHub uses one PAT)
- `RBGC_CBV2_WEBHOOK_SECRET_NAME="rbw-gitlab-webhook-secret"` → **delete** (GitHub App handles webhooks)
- `RBGC_CBV2_CONNECTION_SUFFIX="-gitlab"` → rename to `"-github"` (or remove suffix concept)
- Comment on line 151 references "GitLab" → update

The three-secret model (api-token + read-token + webhook) is GitLab CB v2 specific.
GitHub CB v2 needs only one secret (classic PAT). All consumers of the deleted
constants (`rbgp_Payor.sh`, `rbf_Foundry.sh`) must be updated during depot rebuild.

### Current State

**Depot demo1015** exists with CB v2 GitLab connection. Rubric repo at
`gitlab.com/bhyslop/rb-rubric.git`. 7 vessel triggers created.

**Known issues from ₣Ai e2e attempt (2026-03-03):**
1. Push triggers fired on inscribe push (7-build burst) — **FIXED** (₢AiABC:
   unmatchable `^MANUAL-DISPATCH-ONLY$` branch filter)
2. IAM read-modify-write race destroys bindings — **fix pending** (₢AiABD in ₣Ai:
   declarative policy writes)
3. Syft multi-platform OCI layout failure — **FIXED** (₢AiABE: skopeo split)
4. Build step `dir` field was missing from stitched JSON — **FIXED** in ₣Ai

**Prerequisite from ₣Ai:** ₢AiABD (fix-iam-policy-declarative-assertions) must
land before a clean e2e run. IAM stale-read overwrites cause binding loss during
`create_director`.

### What This Heat Verifies

- Full lifecycle: destroy → create → roles → pins → inscribe → dispatch → provenance
- SLSA v1.0 provenance exists on trigger-invoked builds
- IAM bindings survive the full depot lifecycle (post-₢AiABD fix)
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
