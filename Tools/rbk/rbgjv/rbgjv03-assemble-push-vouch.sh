#!/bin/bash
# RBGJV Step 03: Build FROM scratch container with verification results, push as -vouch
# Builder: docker (via RBRG_DOCKER_IMAGE_REF)
# Entrypoint: bash
# Substitutions: _RBGV_GAR_HOST, _RBGV_GAR_PATH, _RBGV_VESSEL,
#                _RBGV_CONSECRATION, _RBGV_ARK_SUFFIX_VOUCH
#
# Note: The Dockerfile heredoc below is intentional — this script runs inside
# a Cloud Build container, not under BCG module discipline.
#
# Platforms are read from /workspace/vouch_platforms.txt (written by step 02).
# Multi-platform via buildx. No TARGETARCH needed — vouch content (JSON files)
# is architecture-independent. Multi-platform is for pull ergonomics only.

set -euo pipefail

echo "=== Assemble and push vouch artifact ==="

# Read platforms from step 02 output
test -f /workspace/vouch_platforms.txt \
  || { echo "FATAL: /workspace/vouch_platforms.txt not found — step 02 must run first" >&2; exit 1; }
PLATFORMS=$(cat /workspace/vouch_platforms.txt)
test -n "${PLATFORMS}" || { echo "FATAL: vouch_platforms.txt is empty" >&2; exit 1; }

VOUCH_TAG="${_RBGV_GAR_HOST}/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}:${_RBGV_CONSECRATION}${_RBGV_ARK_SUFFIX_VOUCH}"
echo "Platforms: ${PLATFORMS}"
echo "Target: ${VOUCH_TAG}"

mkdir -p /workspace/vouch_ctx
cp /workspace/vouch_summary.json /workspace/vouch_ctx/

# Copy per-platform verification files if they exist (conjure only)
# shellcheck disable=SC2086
if ls /workspace/verify-*.json >/dev/null 2>&1; then
  cp /workspace/verify-*.json /workspace/vouch_ctx/
  echo "FROM scratch" > /workspace/vouch_ctx/Dockerfile
  echo "COPY vouch_summary.json /" >> /workspace/vouch_ctx/Dockerfile
  echo "COPY verify-*.json /" >> /workspace/vouch_ctx/Dockerfile
else
  echo "FROM scratch" > /workspace/vouch_ctx/Dockerfile
  echo "COPY vouch_summary.json /" >> /workspace/vouch_ctx/Dockerfile
fi

docker buildx inspect rb-builder >/dev/null 2>&1 \
  || docker buildx create --driver docker-container --name rb-builder
docker buildx use rb-builder

docker buildx build \
  --push \
  --platform="${PLATFORMS}" \
  --tag "${VOUCH_TAG}" \
  /workspace/vouch_ctx

echo "Vouch artifact pushed: ${VOUCH_TAG}"
