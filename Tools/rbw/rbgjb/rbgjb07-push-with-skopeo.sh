#!/bin/bash
# RBGJB Step 07: Push OCI layout to GAR using Skopeo
# Builder: quay.io/skopeo/stable:latest
# Substitutions: _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY, _RBGY_MONIKER
#
# OCI Layout Bridge Phase 2: Push the multi-platform OCI archive from /workspace/oci-layout.tar
# to Artifact Registry using Skopeo with proper authentication.
#
# Why Skopeo: Skopeo runs in a container that has access to Cloud Build's metadata server,
# allowing it to fetch GAR credentials. This solves the BuildKit credential isolation problem.

set -euo pipefail

# Required inputs
test -n "${_RBGY_GAR_LOCATION}"   || (echo "_RBGY_GAR_LOCATION missing"   >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"    || (echo "_RBGY_GAR_PROJECT missing"    >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}" || (echo "_RBGY_GAR_REPOSITORY missing" >&2; exit 1)
test -n "${_RBGY_MONIKER}"        || (echo "_RBGY_MONIKER missing"        >&2; exit 1)
test -s .tag_base                  || (echo "tag base not derived"         >&2; exit 1)
test -f /workspace/oci-layout.tar  || (echo "OCI archive not found"        >&2; exit 1)

TAG_BASE="$(cat .tag_base)"
IMAGE_URI="${_RBGY_GAR_LOCATION}-docker.pkg.dev/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}-img"

echo "Fetching access token from metadata server..."
TOKEN_JSON=$(curl -sf -H "Metadata-Flavor: Google" \
  http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token)

if [ -z "$TOKEN_JSON" ]; then
  echo "Failed to fetch access token from metadata server" >&2
  exit 1
fi

# Extract access_token field from JSON (simple grep/sed approach for Cloud Build)
AR_TOKEN=$(echo "$TOKEN_JSON" | grep -o '"access_token":"[^"]*"' | sed 's/"access_token":"\([^"]*\)"/\1/')

if [ -z "$AR_TOKEN" ]; then
  echo "Failed to parse access token from response" >&2
  exit 1
fi

echo "Pushing OCI archive to ${IMAGE_URI}..."
skopeo copy \
  --all \
  --dest-creds="oauth2accesstoken:${AR_TOKEN}" \
  "oci-archive:/workspace/oci-layout.tar" \
  "docker://${IMAGE_URI}"

echo "Push successful. Writing IMAGE_URI for downstream steps..."
echo "${IMAGE_URI}" > .image_uri
echo "Done. Image available at: ${IMAGE_URI}"
