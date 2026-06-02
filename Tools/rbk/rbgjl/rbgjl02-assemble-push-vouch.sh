#!/bin/bash
# RBGJL Step 02: Push each Lode's provenance envelope as its :rbi_vouch artifact
# Builder: docker (from reliquary)
# Entrypoint: bash
# Substitutions: _RBGL_GAR_HOST, _RBGL_GAR_PATH, _RBGL_LODES_ROOT, _RBGL_TAG_VOUCH
#
# Note: The Dockerfile echo-sequence below is intentional — this script runs
# inside a Cloud Build container, not under BCG module discipline.
#
# Step 01 (skopeo) staged one envelope per captured Lode at
# /workspace/lode_<stamp>_vouch.json and listed the stamps in
# /workspace/lode_stamps.txt. Each envelope rides into the SAME package as the
# base manifest, under the :rbi_vouch tag — a distinct manifest, one per Lode.
# Vouch content is architecture-independent; single-platform is sufficient.

set -euo pipefail

echo "=== Assemble and push Lode vouch artifacts ==="

test -f /workspace/lode_stamps.txt \
  || { echo "FATAL: /workspace/lode_stamps.txt not found — step 01 must run first" >&2; exit 1; }
test -s /workspace/lode_stamps.txt \
  || { echo "FATAL: /workspace/lode_stamps.txt is empty — nothing ensconced" >&2; exit 1; }

test -n "${_RBGL_TAG_VOUCH}" || { echo "FATAL: _RBGL_TAG_VOUCH missing" >&2; exit 1; }

docker buildx inspect rb-builder >/dev/null 2>&1 \
  || docker buildx create --driver docker-container --name rb-builder
docker buildx use rb-builder

while IFS= read -r STAMP || test -n "${STAMP}"; do
  test -n "${STAMP}" || continue

  ENVELOPE_FILE="/workspace/lode_${STAMP}_vouch.json"
  test -f "${ENVELOPE_FILE}" \
    || { echo "FATAL: envelope not staged for ${STAMP}: ${ENVELOPE_FILE}" >&2; exit 1; }

  VOUCH_URI="${_RBGL_GAR_HOST}/${_RBGL_GAR_PATH}/${_RBGL_LODES_ROOT}/${STAMP}:${_RBGL_TAG_VOUCH}"
  echo "--- Vouch for ${STAMP} -> ${VOUCH_URI} ---"

  CTX="/workspace/vouch_ctx_${STAMP}"
  mkdir -p "${CTX}"
  cp "${ENVELOPE_FILE}" "${CTX}/vouch.json"
  echo "FROM scratch"        >  "${CTX}/Dockerfile"
  echo "COPY vouch.json /"   >> "${CTX}/Dockerfile"

  docker buildx build \
    --push \
    --platform="linux/amd64" \
    --tag "${VOUCH_URI}" \
    "${CTX}" \
    || { echo "FATAL: buildx push failed for vouch ${STAMP}" >&2; exit 1; }

  echo "Vouch pushed: ${VOUCH_URI}"
done < /workspace/lode_stamps.txt

echo "=== Vouch push step complete ==="
