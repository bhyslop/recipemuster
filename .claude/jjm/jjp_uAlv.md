## Character
Architectural research spike evolving into air-gapped build design.

## Research Findings (2026-03-15)

### GCB Pin Images: GAR Mirroring
All 7 RBRG-pinned tool images measured at ~1.97 GB compressed (linux/amd64). gcloud is 63% alone at 1234 MB. All actively used. GAR storage cost ~$0.20/month.

### GCB Worker Image Availability Under NO_PUBLIC_EGRESS
NO_PUBLIC_EGRESS blocks public internet, not Google APIs. Workers retain Private Google Access — `gcr.io/cloud-builders/*` images (gcloud, docker) remain pullable via internal routes. Docker Hub origins (binfmt, syft, alpine, oras, skopeo) are blocked. The pre-cached image set on private pool workers is undocumented; cache misses on Google images still resolve, cache misses on Docker Hub fail.

### APT/pip Under NO_PUBLIC_EGRESS
AR APT remote repos are Preview with bootstrap catch-22. Recommended: fat base image vessel pre-baking OS+Python deps. AR Python remote/virtual repo for app-specific pip tail only.

### SLSA Provenance: builds.create vs triggers.run (EMPIRICAL, NOT SETTLED)
builds.create without git source produces GCB-signed v1 provenance. builder.id: GoogleHostedWorker, buildType: google-worker/v1. resolvedDependencies: step images only (no git source in materials). slsa-verifier rejects this (missing buildConfigSource). Whether this means Level 3 is unachievable or slsa-verifier is overly strict is an OPEN QUESTION (₢AvAAB).

The experiment's `direct_verify.py` workaround reads provenance JSON fields without cryptographic signature verification — it checks claims, not proofs. This is NOT a substitute for formal SLSA verification and must not ship.

### Gotchas (PROVEN)
Platform must match builder (RBGC_BUILD_RUNNER_PLATFORM) even for FROM SCRATCH data-only images — first experiment commit failed without --platform flag, fix commit added it. Scratch image needs dummy CMD for docker create. Mason SA must be explicit in build JSON. GCB strict substitution matching.

## Vocabulary

### Reliquary
A co-versioned, datestamped, immutable snapshot of all tool images and vessel base images, emplaced in GAR. Single concept — no separate term for the instance vs the type (tested against load-bearing principle: unlike vessel/consecration, there is no persistent parent entity that accumulates instances). Each reliquary is identified by a datestamped string.

### Pouch
The build context packaged as a FROM SCRATCH OCI image and pushed to GAR. Tagged as `{vessel}:{consecration}-pouch`, making it a first-class ark artifact alongside `-image`, `-about`, `-vouch`. The pouch carries the Dockerfile and supporting files to the GCB worker, replacing the rubric repo's context delivery role. Cleaned up by abjure with the rest of the consecration's artifacts.

## Design Decisions (2026-03-21)

### RBRV_BASE_IMAGE_[123]
Optional vessel regime variables declaring base image dependencies in tag form (e.g., `python:3.11-slim`). Up to 3 per vessel (multi-stage Dockerfile support). The vessel author declares intent; the build system resolves to concrete references. The Dockerfile uses `ARG RBRV_BASE_IMAGE_1` / `FROM ${RBRV_BASE_IMAGE_1}` — same name, value substituted at build time (tag under open-egress, GAR digest under air-gap). The substitution is not misdirection; it is what build args exist for.

### RBRV_RELIQUARY
Optional vessel regime variable. If populated, the vessel builds air-gapped using that reliquary's images (NO_PUBLIC_EGRESS on private pool, all references resolved to GAR mirrors). If empty, open-egress build (today's behavior). The presence of the value IS the mode gate — no separate boolean or enum. Different vessels may reference different reliquaries — images evolve independently, Recipe Bottle is not opinionated.

### Inscribe Reclaimed
With GitLab rubric repo eliminated, inscribe is reclaimed as the reliquary generation operation. Walks the vessel fleet, reads RBRV_BASE_IMAGE_* declarations and RBRG tool pins, pulls everything from upstream, pushes the complete set to a namespaced GAR location, and produces the reliquary identifier. Co-versioning is enforced by the operation — everything in one pass, one datestamp.

### Build = Conjure Execution
Build (conjure) does: load vessel regime, resolve base images against reliquary (if RBRV_RELIQUARY set), assign consecration, push pouch to GAR, stitch JSON, submit via builds.create, wait, vouch. Clean separation from inscribe — no overlap.

### No More Triggers
Trigger path fully removed. Stitch generates clean builds.create JSON natively. No rubric repo substitutions generated, no post-hoc jq surgery. GitLab elimination is complete: no GitLab account, PAT, Secret Manager entries, CB v2 connection, triggers, or RBRR_RUBRIC_REPO_URL.

### Consecration Format Under builds.create
The dual-timestamp consecration format `{mode}{T1}-r{T2}` gains new meaning. T1 becomes the reliquary datestamp (when the build toolchain was inscribed). T2 remains the actual build time (when conjure ran). This cleanly encodes provenance: the consecration tells you both WHAT tools built the image and WHEN. For open-egress builds without a reliquary, T1 semantics revert to inscribe-time as before. If all conjures require reliquaries (see Open Questions), the format becomes universally consistent.

### Consecration Minted Locally
With builds.create replacing triggers.run, the Director (not GCB) is the build initiator. Consecration timestamps assigned on the Director's workstation. Formalized as input to stitch, not discovered from build step output. Director knows the complete artifact tag set before submission — naming authority and verification authority are the same entity; the cloud is just labor.

### Explicit Verify Method
Build JSON includes explicit `_RBGV_VERIFY_METHOD` substitution declaring verification intent. Vouch step executes the declared method rather than inferring from empty/present source URI.

### Single-Platform Conjure Valid
The multi-platform assertion in rbgjv02 (line 69-71) is a simplification artifact from ₢AtAAD, not an invariant. Single-platform conjure vessels are legitimate. The vouch verification loop iterates once.

### Open-Egress Preserved
Open-egress (connected) builds remain fully supported. Required because open-egress builds produce images that may become base images for air-gapped builds. The two modes coexist per-vessel via RBRV_RELIQUARY presence.

### RBRG May Become Obsolete
If all conjures require reliquaries, RBRG's role (holding upstream tool image pins) may be fully absorbed by the reliquary. The reliquary contains the same images, already resolved and mirrored. RBRG would only be needed as the "upstream source list" for the inscribe operation — and even that could be a static configuration rather than a regime with freshness gates. Pending resolution of the all-conjures-require-reliquary question.

### Director IAM Surface Changes
With triggers and rubric repo eliminated, the Director's IAM grants need reassessment:
- **Remove**: Secret Manager access for GitLab PAT (3 secrets gone), build bucket objectCreator/objectViewer (source upload no longer needed if pouch goes via docker push to GAR)
- **Retain**: cloudbuild.builds.editor (builds.create submission), cloudbuild.workerPoolUser (private pool), artifactregistry.repoAdmin (image management), iam.serviceAccountUser on Mason (actAs for build execution)
- **Assess**: Does docker push for pouch delivery use the Director's existing GAR repoAdmin grant, or does it need a distinct permission? The Director already has repoAdmin for image management — pouch push may be covered.

This is a security surface reduction: fewer secrets, fewer cross-service grants, simpler audit.

### RBSHR Update Required
The Horizon Roadmap egress lockdown entry (RBSHR line 87-93) describes the old architecture. When this heat's design stabilizes, update RBSHR to reflect the new reliquary/pouch/builds.create architecture — or graduate the item out of RBSHR entirely since it is now active heat work, not deferred.

## Open Questions

### SLSA Level 3 with builds.create (₢AvAAB)
Does builds.create without git source achieve SLSA Build Level 3? The experiment's slsa-verifier rejection could mean Level 3 is structurally impossible (no verified source), or that slsa-verifier enforces assumptions beyond the SLSA spec. Definitive answer needed before finalizing verification architecture.

### All Conjures Require Reliquary?
If all conjure builds required RBRV_RELIQUARY (no open-egress conjure path), the system simplifies significantly: one verification path, one download method (everything from GAR), one stitch mode, one consecration format semantics, potential RBRG elimination. Cost: inscribe-before-conjure always, even for quick iterations. This eliminates the dual slsa-verifier-download problem (GitHub fetch vs GAR-resident). Discuss after ₢AvAAB resolves the SLSA question.