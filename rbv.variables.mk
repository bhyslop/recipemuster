# Needed generally
export RBV_REGISTRY_OWNER      := bhyslop
export RBV_REGISTRY_NAME       := recipemuster
export RBV_BUILD_ARCHITECTURES := linux/amd64
export RBV_HISTORY_DIR         := RBM-history

# File containing user specific secrets for accessing the container registry.  Must define:
#
# RBV_USERNAME: Your GitHub username for authentication with GitHub Container Registry (ghcr.io)
# RBV_PAT: Personal Access Token with appropriate permissions for GitHub Container Registry operations
#          Generate this token at https://github.com/settings/tokens with at least the following scopes:
#            - read:packages, write:packages, delete:packages (for container operations)
#            - repo (for workflow dispatch)
export RBV_GITHUB_PAT_ENV      := ../secrets/github-ghcr-play.env

