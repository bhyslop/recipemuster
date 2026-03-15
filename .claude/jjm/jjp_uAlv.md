## Character
Architectural research spike with one concrete SLSA experiment.

## Research Summary (2026-03-15)

### GCB Pin Images: GAR Mirroring
All 7 RBRG-pinned tool images measured at ~1.97 GB compressed (linux/amd64). gcloud is 63% alone at 1234 MB. All actively used. GAR storage cost ~$0.20/month.

### Egress Lockdown Roadmap (RBSHR)
Beyond 7 tool images, builds also fetch: slsa-verifier binary, source repo (GitLab), vessel base images, APT packages, PyPI packages.

### APT/pip Under NO_PUBLIC_EGRESS
AR APT remote repos are Preview with bootstrap catch-22. Recommended: fat base image vessel pre-baking OS+Python deps. AR Python remote/virtual repo for app-specific pip tail only.

### New Mode vs Flag on Conjure
Flag on conjure is cleaner than new mode. Same CB, same SLSA, same consecration prefix c. Vessel regime flag RBRV_EGRESS_REQUIRED=false selects pool posture.

### Eliminating GitLab
Build context packaged as OCI artifact in GAR (docker push, no gcloud). Replaces: GitLab account, PAT, 3 Secret Manager entries, CB v2 connection/repo, triggers, RBRR_RUBRIC_REPO_URL. Uses builds.create API with inline JSON instead of triggers.run.

### SLSA Provenance Risk (KEY EXPERIMENT)
builds.create vs triggers.run may affect SLSA Level 3 attestation. Rubric repo was never real source anyway (generated staging). GAR digest arguably more truthful. This heat tests that question empirically.