#!/bin/bash
# RBGJAM Step 01: Discover platforms from -image manifest in GAR
# Builder: gcr.io/cloud-builders/gcloud
# Substitutions: _RBGA_GAR_HOST, _RBGA_GAR_PATH, _RBGA_VESSEL,
#                _RBGA_CONSECRATION, _RBGA_VESSEL_MODE
#
# Queries the -image manifest to discover what platforms are present.
# Handles OCI image index (multi-platform) vs single OCI/Docker manifest.
# Writes platform info to /workspace for subsequent steps:
#   platforms.txt          - comma-separated platform list (linux/amd64,linux/arm64)
#   platform_suffixes.txt  - comma-separated suffix list (-amd64,-arm64)
#   platform_count.txt     - number of platforms
#   platform_digests.txt   - suffix-to-digest mapping (one line per platform: -amd64 sha256:...)

set -euo pipefail

command -v jq >/dev/null 2>&1 || { apt-get update -qq >/dev/null && apt-get install -y -qq jq >/dev/null; }

test -n "${_RBGA_GAR_HOST}"       || { echo "_RBGA_GAR_HOST missing"       >&2; exit 1; }
test -n "${_RBGA_GAR_PATH}"       || { echo "_RBGA_GAR_PATH missing"       >&2; exit 1; }
test -n "${_RBGA_VESSEL}"         || { echo "_RBGA_VESSEL missing"         >&2; exit 1; }
test -n "${_RBGA_CONSECRATION}"   || { echo "_RBGA_CONSECRATION missing"   >&2; exit 1; }
test -n "${_RBGA_VESSEL_MODE}"    || { echo "_RBGA_VESSEL_MODE missing"    >&2; exit 1; }

IMAGE_TAG="${_RBGA_CONSECRATION}-image"
REGISTRY_BASE="https://${_RBGA_GAR_HOST}/v2/${_RBGA_GAR_PATH}/${_RBGA_VESSEL}"

echo "Fetching OAuth2 token via gcloud"
TOKEN=$(gcloud auth print-access-token) \
  || { echo "Failed to get OAuth2 token" >&2; exit 1; }

ACCEPT="application/vnd.oci.image.index.v1+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.oci.image.manifest.v1+json,application/vnd.docker.distribution.manifest.v2+json"

echo "=== Discovering platforms for ${_RBGA_VESSEL}:${IMAGE_TAG} ==="

MANIFEST=$(curl -sf \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Accept: ${ACCEPT}" \
  "${REGISTRY_BASE}/manifests/${IMAGE_TAG}") \
  || { echo "Failed to fetch manifest for ${IMAGE_TAG}" >&2; exit 1; }

MEDIA_TYPE=$(printf '%s' "${MANIFEST}" | jq -r '.mediaType // empty')
echo "Manifest media type: ${MEDIA_TYPE}"

IS_INDEX="false"
case "${MEDIA_TYPE}" in
  *manifest.list*|*image.index*)
    IS_INDEX="true" ;;
esac

if [ "${IS_INDEX}" = "true" ]; then
  echo "Multi-platform manifest detected"

  # Extract platform, digest, and suffix from manifest list
  # Output: tab-separated lines: os/arch[/variant]\tdigest\t-arch[variant]
  printf '%s' "${MANIFEST}" | jq -r '
    .manifests[] | select(.platform) |
    (.platform.os + "/" + .platform.architecture +
      (if .platform.variant then "/" + .platform.variant else "" end)) +
    "\t" + .digest +
    "\t-" + (.platform.architecture +
      (if .platform.variant then .platform.variant else "" end))
  ' > platform_entries.txt

  PLATFORMS=""
  SUFFIXES=""
  > platform_digests.txt
  while IFS=$'\t' read -r PLAT DIGEST SUFFIX; do
    echo "${SUFFIX} ${DIGEST}" >> platform_digests.txt
    if [ -n "${PLATFORMS}" ]; then
      PLATFORMS="${PLATFORMS},${PLAT}"
      SUFFIXES="${SUFFIXES},${SUFFIX}"
    else
      PLATFORMS="${PLAT}"
      SUFFIXES="${SUFFIX}"
    fi
  done < platform_entries.txt

  echo "${PLATFORMS}" > platforms.txt
  echo "${SUFFIXES}" > platform_suffixes.txt
  wc -l < platform_entries.txt | tr -d ' ' > platform_count.txt

else
  echo "Single manifest detected"

  # Fetch config blob to extract platform info
  CONFIG_DIGEST=$(printf '%s' "${MANIFEST}" | jq -r '.config.digest')
  test -n "${CONFIG_DIGEST}" || { echo "No config digest in manifest" >&2; exit 1; }

  CONFIG=$(curl -sf \
    -H "Authorization: Bearer ${TOKEN}" \
    "${REGISTRY_BASE}/blobs/${CONFIG_DIGEST}") \
    || { echo "Failed to fetch config blob" >&2; exit 1; }

  OS=$(printf '%s' "${CONFIG}" | jq -r '.os')
  ARCH=$(printf '%s' "${CONFIG}" | jq -r '.architecture')
  VARIANT=$(printf '%s' "${CONFIG}" | jq -r '.variant // empty')

  if [ -n "${VARIANT}" ]; then
    PLATFORM="${OS}/${ARCH}/${VARIANT}"
    SUFFIX="-${ARCH}${VARIANT}"
  else
    PLATFORM="${OS}/${ARCH}"
    SUFFIX="-${ARCH}"
  fi

  # Get manifest digest from HEAD response
  DIGEST_HEADERS=$(curl -sI \
    -H "Authorization: Bearer ${TOKEN}" \
    -H "Accept: ${ACCEPT}" \
    "${REGISTRY_BASE}/manifests/${IMAGE_TAG}") \
    || { echo "Failed HEAD for manifest digest" >&2; exit 1; }
  MANIFEST_DIGEST=$(printf '%s' "${DIGEST_HEADERS}" | grep -i "docker-content-digest" | sed 's/.*: *//' | tr -d '\r')
  test -n "${MANIFEST_DIGEST}" || { echo "No Docker-Content-Digest in HEAD response" >&2; exit 1; }

  echo "${PLATFORM}" > platforms.txt
  echo "${SUFFIX}" > platform_suffixes.txt
  echo "1" > platform_count.txt
  echo "${SUFFIX} ${MANIFEST_DIGEST}" > platform_digests.txt
fi

echo "Platforms: $(cat platforms.txt)"
echo "Suffixes: $(cat platform_suffixes.txt)"
echo "Count: $(cat platform_count.txt)"
echo "=== Platform discovery complete ==="

# === Attempt -diags extraction (conjure-only artifact) ===
DIAGS_TAG="${_RBGA_CONSECRATION}-diags"
echo ""
echo "=== Checking for -diags artifact: ${DIAGS_TAG} ==="

DIAGS_ACCEPT="application/vnd.oci.image.manifest.v1+json,application/vnd.docker.distribution.manifest.v2+json"

DIAGS_STATUS=$(curl -s -o /dev/null -w "%{http_code}" \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Accept: ${DIAGS_ACCEPT}" \
  "${REGISTRY_BASE}/manifests/${DIAGS_TAG}") || DIAGS_STATUS="000"

if [ "${DIAGS_STATUS}" = "200" ]; then
  echo "Found -diags artifact, extracting..."

  DIAGS_MANIFEST=$(curl -sf \
    -H "Authorization: Bearer ${TOKEN}" \
    -H "Accept: ${DIAGS_ACCEPT}" \
    "${REGISTRY_BASE}/manifests/${DIAGS_TAG}") \
    || { echo "WARN: Failed to fetch -diags manifest — skipping extraction" >&2; DIAGS_MANIFEST=""; }

  if [ -n "${DIAGS_MANIFEST}" ]; then
    printf '%s' "${DIAGS_MANIFEST}" | jq -r '.layers[].digest' > diags_layers.txt

    while IFS= read -r LAYER_DIGEST || [ -n "${LAYER_DIGEST}" ]; do
      echo "Extracting -diags layer: ${LAYER_DIGEST}"
      curl -sf \
        -H "Authorization: Bearer ${TOKEN}" \
        "${REGISTRY_BASE}/blobs/${LAYER_DIGEST}" \
        > diags_layer.tar.gz
      tar xzf diags_layer.tar.gz \
        || echo "WARN: Failed to extract -diags layer ${LAYER_DIGEST}" >&2
      rm -f diags_layer.tar.gz
    done < diags_layers.txt

    # Report extracted files
    for DFILE in buildkit_metadata.json cache_before.json cache_after.json recipe.txt; do
      if [ -f "${DFILE}" ]; then
        echo "Extracted from -diags: ${DFILE} ($(wc -c < "${DFILE}" | tr -d ' ') bytes)"
      fi
    done
  fi
else
  echo "No -diags artifact found (HTTP ${DIAGS_STATUS}) — expected for bind/graft"
fi

echo "=== -diags check complete ==="
