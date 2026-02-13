## Intent

Eliminate external registry fragility by mirroring all GCB builder images into project-controlled GAR (Google Artifact Registry) within the depot.

## Problem

GCB pipelines pin third-party images by digest. These digests can be garbage-collected by upstream registries at any time:
- **quay.io**: Aggressively GCs within hours (broke us today with skopeo)
- **docker.io**: Rate-limited pulls; digest persistence better but not guaranteed
- **ghcr.io**: Relatively stable but still third-party

Only `gcr.io` (Google cloud-builders) images are fully under Google's control and reliably persistent.

## Current Exposure

| Registry | Images | Risk |
|----------|--------|------|
| gcr.io | gcrane, gcloud, docker | Low (Google-controlled) |
| ghcr.io | oras | Medium |
| docker.io | alpine, syft, binfmt | Medium (rate limits + eventual GC) |
| quay.io | skopeo (being eliminated in ₣AP) | High (aggressive GC) |

Additionally, podman VM images from quay.io are used locally (not GCB-critical but same fragility).

## Prior Art

Podman VM mirroring is already prototyped — see `tt/rbw-m.MirrorLatestPodmanVM.sh` and `Tools/rbw/rbv_PodmanVM.sh`. This work demonstrated the pattern of pulling from upstream, pushing to GAR, and pinning the GAR digest.

## Strategy

1. Create a GAR repository for builder tool images within the depot
2. Build a "stash" operation that pulls each pinned image and pushes to GAR
3. Update RBRR pin constants to reference GAR copies
4. RefreshGcbPins becomes: pull latest upstream → push to GAR → pin GAR digest
5. GCB steps reference only gcr.io and project GAR — zero external registry pulls at build time

## Post-MVP

This heat is post-MVP. The immediate skopeo→gcrane substitution (₣AP) removes the acute risk. This heat addresses the structural fragility for all remaining third-party images.