#!/bin/bash
# RBGJV Step 03: Build FROM scratch container with verification results, push as -vouch
# Builder: docker (via RBRG_DOCKER_IMAGE_REF)
# Entrypoint: bash
# Substitutions: _RBGV_GAR_HOST, _RBGV_GAR_PATH, _RBGV_VESSEL, _RBGV_CONSECRATION
#
# Note: The Dockerfile heredoc below is intentional — this script runs inside
# a Cloud Build container, not under BCG module discipline.

set -euo pipefail
echo "=== Assemble and push vouch artifact ==="
VOUCH_TAG="${_RBGV_GAR_HOST}/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}:${_RBGV_CONSECRATION}-vouch"
mkdir -p /workspace/vouch_ctx
cp /workspace/vouch_summary.json /workspace/vouch_ctx/
cp /workspace/verify-*.json /workspace/vouch_ctx/
cat > /workspace/vouch_ctx/Dockerfile <<'DEOF'
FROM scratch
COPY vouch_summary.json /
COPY verify-*.json /
DEOF
docker build -t "${VOUCH_TAG}" /workspace/vouch_ctx
docker push "${VOUCH_TAG}"
echo "Vouch artifact pushed: ${VOUCH_TAG}"
