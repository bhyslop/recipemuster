#!/bin/bash
# RBGJI Step 01: Inscribe tool images to GAR reliquary
# Builder: gcr.io/cloud-builders/docker (always pullable — Google-hosted)
# Substitutions: _RBGN_GAR_HOST, _RBGN_GAR_PATH,
#                _RBGN_RELIQUARIES_ROOT, _RBGN_RELIQUARY
#
# Mirrors all 6 GCB step/tool images from upstream registries to a datestamped
# GAR namespace (reliquary). Co-versioned: all images in one pass, one datestamp.
# Single-platform (linux/amd64) — tool images run as GCB steps on amd64 workers.
# Outputs JSON manifest with tool→digest mapping via /builder/outputs/output.
#
# Image URI shape: <host>/<path>/<RELIQUARIES_ROOT>/<RELIQUARY>/<NAME>:<RELIQUARY>
# Tag = the reliquary datestamp itself (immutable-datestamp-as-tag).

set -euo pipefail
echo "=== Inscribe tool images to GAR reliquary ==="
echo "Reliquary: ${_RBGN_RELIQUARY}"

# Tool image manifest: short-name|upstream-ref
# This is the authoritative list of tool images for GCB step execution.
# gcloud and docker are Google-hosted (gcr.io); rest are third-party.
MANIFEST=(
  "gcloud|gcr.io/cloud-builders/gcloud:latest"
  "docker|gcr.io/cloud-builders/docker:latest"
  "alpine|docker.io/library/alpine:latest"
  "syft|docker.io/anchore/syft:latest"
  "binfmt|docker.io/tonistiigi/binfmt:latest"
  "skopeo|quay.io/skopeo/stable:latest"
)

RESULT='{'
FIRST=true

for ENTRY in "${MANIFEST[@]}"; do
  NAME="${ENTRY%%|*}"
  UPSTREAM="${ENTRY#*|}"

  echo "--- ${NAME}: ${UPSTREAM} ---"

  DEST="${_RBGN_GAR_HOST}/${_RBGN_GAR_PATH}/${_RBGN_RELIQUARIES_ROOT}/${_RBGN_RELIQUARY}/${NAME}:${_RBGN_RELIQUARY}"
  echo "Dest: ${DEST}"

  docker pull "${UPSTREAM}" \
    || { echo "FATAL: Failed to pull ${UPSTREAM}" >&2; exit 1; }

  # Extract pulled digest from RepoDigests (format: repo@sha256:...)
  DIGEST=$(docker inspect --format='{{index .RepoDigests 0}}' "${UPSTREAM}" 2>/dev/null) || DIGEST=""
  DIGEST="${DIGEST##*@}"

  docker tag "${UPSTREAM}" "${DEST}" \
    || { echo "FATAL: Failed to tag ${NAME}" >&2; exit 1; }

  docker push "${DEST}" \
    || { echo "FATAL: Failed to push ${NAME} to GAR" >&2; exit 1; }

  echo "${NAME} inscribed: ${DEST} (${DIGEST})"

  # Accumulate JSON result
  if [ "${FIRST}" = "true" ]; then
    FIRST=false
  else
    RESULT="${RESULT},"
  fi
  RESULT="${RESULT}\"${NAME}\":{\"upstream\":\"${UPSTREAM}\",\"digest\":\"${DIGEST}\"}"
done

RESULT="${RESULT}}"

echo "=== Writing inscribe results ==="
echo "${RESULT}"

mkdir -p /builder/outputs
printf '%s' "${RESULT}" > /builder/outputs/output

echo "=== Inscribe step complete ==="
