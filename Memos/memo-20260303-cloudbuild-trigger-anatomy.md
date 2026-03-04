# Cloud Build Trigger Body Anatomy

**Date**: 2026-03-03
**Status**: Research complete — live E2E testing done
**Context**: CB v2 trigger creation for GitLab SaaS rubric repository (₣Ai heat, pace ₢AiAA0)

## Overview

During E2E verification of Cloud Build v2 trigger creation for GitLab SaaS, trigger creation failed with HTTP 400. Root cause: incorrect trigger body shape. Research and live testing established the correct configuration and identified subtle API distinctions between two parallel Google Cloud connection systems.

## Two Parallel Connection Systems

Google Cloud provides two separate systems for connecting to external git repositories:

| Aspect | Cloud Build 2nd gen | Developer Connect |
|--------|---|---|
| **API endpoint** | `cloudbuild.googleapis.com/v2` | `developerconnect.googleapis.com/v1` |
| **Sub-resources** | `connections/*/repositories/*` | `connections/*/gitRepositoryLinks/*` |
| **Trigger field** | `repositoryEventConfig` | `developerConnectEventConfig` |

These systems are NOT interchangeable. Each trigger field expects the resource path format from its corresponding API.

## Trigger Body Architecture

A Cloud Build trigger consists of two independent union/oneof fields:

### Event Source (pick one)

- `repositoryEventConfig` — Cloud Build v2 Repository API (push/PR events)
- `developerConnectEventConfig` — Developer Connect (push/PR events)
- `github` — GitHub App integration (legacy)
- `triggerTemplate` — Cloud Source Repositories (legacy)
- `pubsubConfig`, `webhookConfig` — non-SCM event sources

### Build Template (pick one)

- `filename` — path to build config in the repo (standard for SCM push triggers)
- `build` — inline Build object
- `gitFileSource` — build config from specific repo/ref (for non-SCM triggers)
- `autodetect` — auto-detect cloudbuild.yaml/yml/json/Dockerfile

**Critical distinction**: `sourceToBuild` and `gitFileSource` are for **non-SCM-event triggers only** (webhook, pubsub, manual, cron). When using `repositoryEventConfig` or `developerConnectEventConfig`, the build source is the commit that triggered the event. Use `filename` to specify the build config path in the repository.

## repositoryEventConfig.repositoryType is Read-Only

The `repositoryType` field in `repositoryEventConfig` is **output-only** according to the API discovery document. The server infers repository type from the connection type. Setting it in the request body causes HTTP 400.

Enum values (for reference):
- `REPOSITORY_TYPE_UNSPECIFIED`
- `GITHUB`
- `GITHUB_ENTERPRISE`
- `GITLAB_ENTERPRISE`
- `BITBUCKET_DATA_CENTER`
- `BITBUCKET_CLOUD`

Note: There is no plain `GITLAB` enum value. GitLab SaaS works through Cloud Build v2; the server presumably sets `GITLAB_ENTERPRISE` for both SaaS and self-hosted variants.

## GitLab SaaS Works with CB 2nd gen

Despite Google's trigger documentation favoring `developerConnectEventConfig` examples for GitLab, Cloud Build 2nd gen (`repositoryEventConfig` with `cloudbuild.googleapis.com/v2`) works correctly for GitLab SaaS. This was confirmed by live E2E testing on 2026-03-03 using a GitLab SaaS connection created with `gitlabConfig` connection type.

## Correct Minimal Trigger Body (CB 2nd gen + GitLab SaaS)

```json
{
  "name": "trigger-name",
  "description": "optional trigger description",
  "repositoryEventConfig": {
    "repository": "projects/PROJECT_ID/locations/REGION/connections/CONNECTION_NAME/repositories/REPO_NAME",
    "push": {
      "branch": "^main$"
    }
  },
  "filename": "cloudbuild.json",
  "serviceAccount": "projects/PROJECT_ID/serviceAccounts/SERVICE_ACCOUNT@PROJECT_ID.iam.gserviceaccount.com"
}
```

Key points:
- `repositoryEventConfig.repositoryType` is omitted (read-only, server-inferred)
- `filename` specifies the build config path in the repository
- `push` specifies the branch filter regex — **required** (`repositoryEventConfig` filter is a union requiring `push` or `pullRequest`)
- `serviceAccount` must be the full resource name
- For manual-dispatch-only triggers, use an unmatchable branch pattern (e.g., `^MANUAL-DISPATCH-ONLY$`) rather than omitting `push` — the API rejects triggers with no filter

## What Failed (Root Cause Analysis)

The original trigger body had three errors:

1. **`repositoryEventConfig.repositoryType: "GITLAB"`**
   - This field is read-only (output-only per API discovery)
   - `"GITLAB"` is not a valid enum value (should be `GITLAB_ENTERPRISE` if settable, but it isn't)
   - Server returns HTTP 400 when present

2. **`sourceToBuild.repositoryType`**
   - Field name is `repoType`, not `repositoryType`
   - `sourceToBuild` should not be used for push/PR triggers from `repositoryEventConfig`
   - `sourceToBuild` is only valid for non-SCM-event triggers

3. **`gitFileSource.repositoryType`**
   - Same field name mismatch
   - `gitFileSource` is only valid for non-SCM-event triggers
   - For push/PR triggers, use `filename` instead

## API References

- [Cloud Build Triggers API Reference (REST v1)](https://cloud.google.com/build/docs/api/reference/rest/v1/projects.locations.triggers)
- [Cloud Build Repositories Overview](https://cloud.google.com/build/docs/repositories)
- [Build repos from GitLab](https://cloud.google.com/build/docs/automating-builds/gitlab/build-repos-from-gitlab)
- [Developer Connect API Reference](https://cloud.google.com/developer-connect/docs/api/reference/rest)
