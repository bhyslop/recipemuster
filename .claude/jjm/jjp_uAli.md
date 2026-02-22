# Paddock: gcb-trigger-migration-tier2

## Context

Migrate the Recipe Bottle build pipeline from ad-hoc `builds.create` API calls
to Cloud Build trigger-based builds. This is Tier 2 of the RBSCB Cloud Build
Roadmap, selected as MVP-required for SLSA v1.0 provenance.

### Key Design Decisions

- **Main repo as trigger source** — triggers connect to the existing GitHub repo via Developer Connect, not per-vessel repos. Eliminates Courier PAT, vessel repo management, and publish ceremony. SLSA provenance records the main repo commit hash — acceptable for alpha (the commit contains the vessel's build definition even if it also contains other changes).
- **GitHub as host** (Developer Connect + OAuth GitHub App, no CLI dependencies, no PAT)
- **Hardcoded YAML** — per-vessel cloudbuild.yaml committed to the main repo is the build artifact
- **Single-phase build**: Foundry verifies yaml is committed, then dispatches via `triggers.run`
- **Dockerfiles stay in parent repo** — alongside their cloudbuild.yaml in vessel directories
- **Phased rollout** — rbev-busybox first, verify SLSA provenance, then migrate remaining conjure vessels
- **Tier 1 log hygiene** folded into YAML authoring (audit during script inlining)
- **All API calls via curl/REST** — consistent with existing Foundry pattern, no gcloud/gh dependency
- **One Developer Connect connection** to the main repo serves all vessel triggers
- **GDC prefix** for Developer Connect RBRR variables (Google service, like GCB/GAR/GCP)
- **OCI Layout Bridge pattern** survives unchanged (build/push/SBOM phase split)

### Decision Rationale

**Why GitHub (not GitLab, CSR, or SSM)?**
Evaluated four options. Cloud Source Repositories is End of Sale (June 2024) — unavailable
to new customers. Secure Source Manager requires 100-user minimum at $10/user = $1,000/month —
enterprise pricing, non-starter. GitLab was more automatable (PAT-only, no OAuth browser step)
but GitHub was chosen because the project already lives there. GitLab remains a viable
future migration if GitHub relationship deteriorates.

**Why main repo triggers (not per-vessel repos)?**
The original design used per-vessel GitHub repos for provenance isolation: one commit = one
vessel's build definition. But this requires a Courier PAT regime, N repo creation/management
scripts, a publish ceremony, and pin-refresh integration per vessel — substantial infrastructure
for alpha. With main repo triggers: Developer Connect uses OAuth GitHub App (one-time browser
auth, no PAT lifecycle), `triggers.run` dispatches against the main repo commit, and Google
still generates SLSA v1.0 provenance recording the exact commit hash. The provenance commit
may include non-vessel changes, but for a single-author alpha project this is acceptable.
Per-vessel repos remain a future option if provenance isolation becomes a real requirement.

**Why GDC prefix for Developer Connect variables?**
Developer Connect is a Google service, not a GitHub concept. Existing RBRR pattern uses
service acronyms (GCB for Cloud Build, GAR for Artifact Registry, GCP for platform-level).
Developer Connect gets GDC to maintain this convention.

### Constraints

**Build context scope**: With main repo triggers, Cloud Build checks out the entire repo,
not a minimal tarball. The builder sees all files. Acceptable since the repo contains no
secrets and Dockerfile/context paths are parameterized via substitution variables.

**Developer Connect setup**: Requires one-time interactive browser OAuth to authorize the
Google Developer Connect GitHub App on the main repo. Not scriptable end-to-end — the
human must complete the GitHub authorization flow once. Subsequent trigger operations are
fully automated via REST API.

### What Dies

- `zrbf_stitch_build_json()` and the rbgjb/*.sh script files
- GCS tarball upload chain (package, compose name, upload)
- `zrbf_compose_build_request_json()` and `builds.create` API path
- The `$$` dollar-escaping dance for Cloud Build substitutions
- Per-vessel GitHub repos concept (no vessel repos needed)
- Courier credential regime concept (no GitHub PAT needed)

### What's Born

- Per-vessel cloudbuild.yaml files with inline step scripts (committed to main repo)
- Developer Connect connection to main GitHub repo (OAuth GitHub App)
- Per-vessel Cloud Build triggers pointing to main repo
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
- SLSA provenance on Cloud Build: https://docs.google.com/build/docs/securing-builds/generate-validate-build-provenance
