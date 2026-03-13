#!/bin/bash
# RBGJV Step 03: Build FROM scratch container with verification results, push as -vouch
# Builder: docker (via RBRG_DOCKER_IMAGE_REF)
# Entrypoint: bash
# Substitutions: _RBGV_GAR_HOST, _RBGV_GAR_PATH, _RBGV_VESSEL, _RBGV_CONSECRATION, _RBGV_PLATFORMS
#
# Note: The Dockerfile heredoc below is intentional — this script runs inside
# a Cloud Build container, not under BCG module discipline.
#
# Multi-platform via buildx. No TARGETARCH needed — vouch content (JSON files)
# is architecture-independent. Multi-platform is for pull ergonomics only.

set -euo pipefail

test -n "${_RBGV_PLATFORMS}" || (echo "_RBGV_PLATFORMS missing" >&2; exit 1)

echo "=== Assemble and push vouch artifact ==="
VOUCH_TAG="${_RBGV_GAR_HOST}/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}:${_RBGV_CONSECRATION}-vouch"
echo "Platforms: ${_RBGV_PLATFORMS}"
echo "Target: ${VOUCH_TAG}"

mkdir -p /workspace/vouch_ctx
cp /workspace/vouch_summary.json /workspace/vouch_ctx/
cp /workspace/verify-*.json /workspace/vouch_ctx/
cat > /workspace/vouch_ctx/Dockerfile <<'DEOF'
FROM scratch
COPY vouch_summary.json /
COPY verify-*.json /
DEOF

docker buildx inspect rb-builder >/dev/null 2>&1 \
  || docker buildx create --driver docker-container --name rb-builder
docker buildx use rb-builder

docker buildx build \
  --push \
  --platform="${_RBGV_PLATFORMS}" \
  --tag "${VOUCH_TAG}" \
  -f Dockerfile \
  /workspace/vouch_ctx

echo "Vouch artifact pushed: ${VOUCH_TAG}"
