## Character
Architectural research spike evolving into air-gapped build design.

## Research Findings (2026-03-15)

### GCB Pin Images: GAR Mirroring
All 7 RBRG-pinned tool images measured at ~1.97 GB compressed (linux/amd64). gcloud is 63% alone at 1234 MB. All actively used. GAR storage cost ~$0.20/month.

### GCB Worker Image Availability Under NO_PUBLIC_EGRESS
NO_PUBLIC_EGRESS blocks public internet, not Google APIs. Workers retain Private Google Access — `gcr.io/cloud-builders/*` images (gcloud, docker) remain pullable via internal routes. Docker Hub origins (binfmt, syft, alpine, oras, skopeo) are blocked. The pre-cached image set on private pool workers is undocumented; cache misses on Google images still resolve, cache misses on Docker Hub fail.

### APT/pip Under NO_PUBLIC_EGRESS
AR APT remote repos are Preview with bootstrap catch-22. Recommended: fat base image vessel pre-baking OS+Python deps. AR Python remote/virtual repo for app-specific pip tail only.

### SLSA Provenance: builds.create vs triggers.run (PROVEN)
builds.create without git source produces GCB-signed v1 provenance. builder.id: GoogleHostedWorker, buildType: google-worker/v1. resolvedDependencies: step images only (no git source in materials). slsa-verifier rejects this (missing buildConfigSource) — direct provenance verification needed. Context image delivery: FROM SCRATCH OCI to GAR, extract in step 0. Full build+vouch pipeline works end-to-end (3 platforms).

### Gotchas (PROVEN)
Platform must match builder (RBGC_BUILD_RUNNER_PLATFORM). Scratch image needs dummy CMD for docker create. Mason SA must be explicit in build JSON. GCB strict substitution matching.

## Design Decisions (2026-03-21)

### Reliquary
A co-versioned, datestamped, immutable snapshot of all tool images and vessel base images, emplaced in GAR. Single concept — no separate term for the instance vs the type (tested against load-bearing principle: unlike vessel/consecration, there is no persistent parent entity that accumulates instances). Each reliquary is identified by a datestamped string.

### RBRV_BASE_IMAGE_[123]
Optional vessel regime variables declaring base image dependencies in tag form (e.g., `python:3.11-slim`). Up to 3 per vessel (multi-stage Dockerfile support). The vessel author declares intent; the build system resolves to concrete references. The Dockerfile uses `ARG RBRV_BASE_IMAGE_1` / `FROM ${RBRV_BASE_IMAGE_1}` — same name, value substituted at build time (tag under open-egress, GAR digest under air-gap). The substitution is not misdirection; it is what build args exist for.

### RBRV_RELIQUARY
Optional vessel regime variable. If populated, the vessel builds air-gapped using that reliquary's images (NO_PUBLIC_EGRESS on private pool, all references resolved to GAR mirrors). If empty, open-egress build (today's behavior). The presence of the value IS the mode gate — no separate boolean or enum. Different vessels may reference different reliquaries — images evolve independently, Recipe Bottle is not opinionated.

### Inscribe Reclaimed
With GitLab rubric repo eliminated, inscribe is reclaimed as the reliquary generation operation. Walks the vessel fleet, reads RBRV_BASE_IMAGE_* declarations and RBRG tool pins, pulls everything from upstream, pushes the complete set to a namespaced GAR location, and produces the reliquary identifier. Co-versioning is enforced by the operation — everything in one pass, one datestamp.

### Consecration Minted Locally
With builds.create replacing triggers.run, the Director (not GCB) is the build initiator. Consecration timestamps should be assigned on the Director's local workstation, not in the cloud. This means the Director knows the complete artifact tag set before submission, can pre-compute all tags, embed them in build JSON, and verify fulfillment. Naming authority and verification authority are the same entity; the cloud is just labor.

### GitLab Elimination
builds.create with OCI-packaged build context replaces triggers.run with rubric repo. Removes: GitLab account, PAT, 3 Secret Manager entries, CB v2 connection/repo, triggers, RBRR_RUBRIC_REPO_URL. This is a simplification play that also happens to be a prerequisite for air-gap.

### Open-Egress Preserved
Open-egress (connected) builds remain fully supported. Required because open-egress builds produce images that may become base images for air-gapped builds. The two modes coexist per-vessel via RBRV_RELIQUARY presence.