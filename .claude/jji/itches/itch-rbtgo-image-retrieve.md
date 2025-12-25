# Itch: Implement rbtgo_image_retrieve

Design and implement the image retrieval operation - currently has neither spec nor implementation.

## Context

Identified during RBAGS audit (heat jjh-b251225-rbags-manual-proc-spec, 2025-12-25) as one of two missing implementations in the Director-triggered remote build flow.

## Prerequisites

Before implementation:
1. Specify rbtgo_image_retrieve in RBAGS following completeness criteria
2. Verify API calls against GCP REST documentation

## Open Questions

- Which GCP API retrieves container images from Artifact Registry?
- What authentication pattern - Governor RBRA or Director token?
- Output format - tarball, OCI manifest, or streaming pull?
- Destination - local file, pipe to podman, or registry mirror?

## Related

- `rbtgo_trigger_build` - triggers the build that creates images
- `rbtgo_image_delete` - removes images (has implementation in rbf_Foundry.sh)
