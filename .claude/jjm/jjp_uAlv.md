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
A co-versioned, datestamped, immutable snapshot of all tool images and vessel base images, emplaced in GAR. Single concept — no separate term for the instance vs the type (tested against load-bearing principle: unlike vessel/consecration, there is no persistent parent entity that accumulates instances). Each reliquary is identified by a datestamped string. Required for all conjure builds.

### Pouch
The build context packaged as a FROM SCRATCH OCI image and pushed to GAR. Tagged as `{vessel}:{consecration}-pouch`, making it a first-class ark artifact alongside `-image`, `-about`, `-vouch`. The pouch carries the Dockerfile and supporting files to the GCB worker, replacing the rubric repo's context delivery role. Cleaned up by abjure with the rest of the consecration's artifacts.

## Design Decisions (2026-03-21)

### Reliquaries Are Required
All conjure builds require a reliquary. This is not just an air-gap feature — it is a universal robustness measure. The original pain (upstream registries rotating images, breaking digest pins) affects all builds. Required reliquaries solve this for everyone: tool versions are frozen in GAR, immune to upstream churn. The inscribe-before-conjure ceremony replaces the current pin-refresh ceremony and is actually less overhead: inscribe once (when you choose to update tools), conjure many times against a stable reliquary.

Simplification cascade: one stitch path, one verification path, one download method, one consecration format semantics. No conditional logic for "reliquary present vs absent."

### Air-Gap vs Open-Egress Is Network Policy, Not Image Policy
With required reliquaries, every conjure pulls tool images and base images from GAR mirrors regardless of egress mode. The distinction between air-gap and open-egress is purely network enforcement: whether NO_PUBLIC_EGRESS is set on the private pool. The reliquary makes air-gap the default behavior at the image layer. Open-egress is a permissive network posture, not a different image sourcing strategy.

### RBRV_BASE_IMAGE_[123]
Optional vessel regime variables declaring base image dependencies in tag form (e.g., `python:3.11-slim`). Up to 3 per vessel (multi-stage Dockerfile support). The vessel author declares intent; the build system resolves to concrete references. The Dockerfile uses `ARG RBRV_BASE_IMAGE_1` / `FROM ${RBRV_BASE_IMAGE_1}` — same name, value substituted at build time with GAR-mirrored digest from the reliquary.

### RBRV_RELIQUARY
Required vessel regime variable for conjure mode. Identifies which reliquary provides tool images and resolved base images for the build. Different vessels may reference different reliquaries — images evolve independently, Recipe Bottle is not opinionated.

### RBRG Replaced by Reliquary
RBRG (regime holding upstream tool image pins with freshness gates) is replaced by the reliquary. The < 1 day freshness gate that currently blocks inscribe is eliminated — you inscribe when you choose, not when a timer forces you. What remains of the upstream source information is a static manifest consumed by inscribe (a list of upstream image references to mirror), not a regime with validation and freshness enforcement.

### Inscribe Reclaimed
With GitLab rubric repo eliminated, inscribe is reclaimed as the reliquary generation operation. Walks the vessel fleet, reads RBRV_BASE_IMAGE_* declarations and the upstream source manifest, pulls everything from upstream, pushes the complete set to a namespaced GAR location, and produces the reliquary identifier. Co-versioning is enforced by the operation — everything in one pass, one datestamp. Inscribe becomes a required step in depot initialization alongside governor/director/depot creation.

### Build = Conjure Execution
Build (conjure) does: load vessel regime, resolve base images against reliquary, assign consecration, push pouch to GAR, stitch JSON (single path — all step image references from reliquary), submit via builds.create, wait, vouch. Clean separation from inscribe — no overlap.

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

### RBSHR Update Required
The Horizon Roadmap egress lockdown entry (RBSHR line 87-93) describes the old architecture. Update to reflect reliquary/pouch/builds.create architecture, or graduate the item out of RBSHR entirely since it is now active heat work.

## Open Questions

### SLSA Level 3 with builds.create (₢AvAAB)
Does builds.create without git source achieve SLSA Build Level 3? The experiment's slsa-verifier rejection could mean Level 3 is structurally impossible (no verified source), or that slsa-verifier enforces assumptions beyond the SLSA spec. Definitive answer needed before finalizing verification architecture.

### Bind/Graft Vouch and About Step Images
Vouch and about GCB jobs for bind and graft modes also use step images (gcloud, alpine). Should these modes reference a reliquary for their GCB step images, making the reliquary the universal step-image source for ALL GCB interactions? This would be consistent but may be overkill for simpler modes. If yes, RBRV_RELIQUARY becomes required for all vessel modes, not just conjure.