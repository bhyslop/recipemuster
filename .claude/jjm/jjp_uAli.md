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
