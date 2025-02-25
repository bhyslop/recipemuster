# Needed generally
RBV_REGISTRY_OWNER      = bhyslop
RBV_REGISTRY_NAME       = recipemuster
RBV_BUILD_ARCHITECTURES = linux/amd64
RBV_HISTORY_DIR         = RBM-history

# File containing user specific secrets for accessing the container registry.  Must define:
#
# RBV_USERNAME: GitHub username required for container registry (ghcr.io) login
# RBV_PAT: GitHub Personal Access Token used for both:
#          1. GitHub API authentication (for building/listing/deleting images)
#          2. Container registry authentication (for pulling images)
#          Generate this token at https://github.com/settings/tokens with scopes:
#          - read:packages, write:packages, delete:packages
#          - repo (for workflow dispatch)
RBV_GITHUB_PAT_ENV      = ../secrets/github-ghcr-play.env

