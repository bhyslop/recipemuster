# Paddock: gcb-trigger-migration-tier2

## Context

Migrate the Recipe Bottle build pipeline from ad-hoc `builds.create` API calls
to Cloud Build trigger-based builds. This is Tier 2 of the RBSCB Cloud Build
Roadmap, selected as MVP-required for SLSA v1.0 provenance.

### Key Design Decisions

- **Per-vessel GitHub repos** for build definitions (strong 1:1 provenance: commit = build definition version)
- **GitHub as host** (Developer Connect + REST API, no CLI dependencies)
- **Courier role** (`rbhk` prefix) — new GitHub-backed identity, distinct from Google RBRA credentials
- **Hardcoded YAML** — cloudbuild.yaml is the artifact, not generated at build time
- **Two-phase build**: prepare (publish to vessel repo) then dispatch (triggers.run)
- **Dockerfiles stay in parent repo** — vessel repos are derived artifacts, never hand-edited
- **`.rbk/` directory** at project root (gitignored) holds cloned vessel repos, parallels `.buk/`
- **Phased rollout** — rbev-busybox first, verify SLSA provenance, then migrate remaining conjure vessels
- **Tier 1 log hygiene** folded into YAML authoring (audit during script inlining)
- **All API calls via curl/REST** — consistent with existing Foundry pattern, no gcloud/gh dependency
- **One Developer Connect connection** serves all vessel repos (not per-vessel connections)
- **GDC prefix** for Developer Connect RBRR variables (Google service, like GCB/GAR/GCP)
- **OCI Layout Bridge pattern** survives unchanged (build/push/SBOM phase split)

### Decision Rationale

**Why GitHub (not GitLab, CSR, or SSM)?**
Evaluated four options. Cloud Source Repositories is End of Sale (June 2024) — unavailable
to new customers. Secure Source Manager requires 100-user minimum at $10/user = $1,000/month —
enterprise pricing, non-starter. GitLab was more automatable (PAT-only, no OAuth browser step)
but GitHub was chosen because the project already lives there. GitLab remains a viable
future migration if GitHub relationship deteriorates.

**Why per-vessel repos (not one shared repo)?**
SLSA provenance records the source commit that produced an image. With a shared repo,
that commit may include changes to other vessels — the commit hash doesn't isolate the
vessel's build definition. With per-vessel repos, the provenance commit IS the vessel's
build definition version. Clean 1:1 mapping. The management overhead (N repos, N triggers,
N pin-refresh commits) is scriptable and justified by provenance clarity.

**Why NOT RBRA for Courier credentials?**
RBRA is specifically Google service account credentials: CLIENT_EMAIL, PRIVATE_KEY,
PROJECT_ID, TOKEN_LIFETIME_SEC. A GitHub PAT is a single opaque token — fundamentally
different provider, different trust boundary, different format. Needs its own credential
regime design (₢AiAAB). Also note: GitHub PATs expire and need refresh/rotation — the
credential regime must account for this lifecycle.

**Why GDC prefix for Developer Connect variables?**
Developer Connect is a Google service, not a GitHub concept. Existing RBRR pattern uses
service acronyms (GCB for Cloud Build, GAR for Artifact Registry, GCP for platform-level).
Developer Connect gets GDC to maintain this convention.

### Constraints

**Build context isolation**: Vessel repos contain ONLY that vessel's Dockerfile, build
context, and cloudbuild.yaml. If a Dockerfile in the parent repo references files outside
its vessel directory (e.g., shared base configs), the trigger build will fail because those
files won't exist in the vessel repo. Vessel Dockerfiles must be self-contained within
their build context.

**Courier PAT scope**: GitHub PATs with `repo` scope grant read/write to ALL repos under
the account — broader than needed for individual vessel repos. Fine-grained PATs (per-repo)
exist but add complexity. Scope decision is part of ₢AiAAB (credential regime design).

### What Dies

- `zrbf_stitch_build_json()` and the rbgjb/*.sh script files
- GCS tarball upload chain (package, compose name, upload)
- `zrbf_compose_build_request_json()` and `builds.create` API path
- The `$$` dollar-escaping dance for Cloud Build substitutions

### What's Born

- Per-vessel cloudbuild.yaml files with inline step scripts
- Per-vessel GitHub repos (e.g., `rbk-build-rbev-busybox`)
- Courier credential regime (GitHub PAT, distinct from RBRA)
- Developer Connect connection (Google-side GitHub bridge)
- Prepare step in Foundry (publish ceremony with pin staleness check)
- Dispatch step via `triggers.run` API
- SLSA v1.0 provenance on all trigger-built images

### Vessel Modes

Currently implementing conjure mode only. Repo structure designed to accommodate:
- **Conjure**: cloudbuild.yaml + Dockerfile + context (build from scratch)
- **Mirror**: cloudbuild.yaml + manifest (pull public, push to GAR) — future
- **Local push**: cloudbuild.yaml + OCI tarball reference — future

## Build Requirements

This heat is primarily bash/GCP tooling (RBW). No Rust builds required for most paces.
BCG compliance is mandatory for all new bash code — enterprise-grade orchestration.

## References

- `lenses/RBSCB-CloudBuildRoadmap.adoc` — Tier definitions and decision log
- `Tools/rbw/rbf_Foundry.sh` — Current build pipeline (being replaced)
- `Tools/rbw/rbgjb/*.sh` — Current step scripts (content moving to YAML)
- `rbrr.env` — Regime configuration (being extended)
- `Tools/rbw/rbrr_regime.sh` — Regime validator (being extended)
- `lenses/RBSTB-trigger_build.adoc` — Trigger build spec (being rewritten)
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — BCG patterns (mandatory)
- Developer Connect REST API: https://docs.google.com/developer-connect/docs/api/reference/rest
- Cloud Build triggers REST API: https://cloud.google.com/build/docs/api/reference/rest/v1/projects.triggers
- GitHub repos REST API: https://docs.github.com/en/rest/repos/repos
- SLSA provenance on Cloud Build: https://docs.google.com/build/docs/securing-builds/generate-validate-build-provenance
