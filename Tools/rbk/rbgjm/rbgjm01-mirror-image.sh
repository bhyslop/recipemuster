#!/bin/bash
# RBGJM Step 01: Mirror image from upstream to GAR via skopeo
# Builder: RBRG_SKOPEO_IMAGE_REF (quay.io/skopeo/stable)
# Substitutions: _RBGA_GAR_HOST, _RBGA_GAR_PATH, _RBGA_VESSEL,
#                _RBGA_CONSECRATION, _RBGA_BIND_SOURCE, _RBGA_ARK_SUFFIX_IMAGE
#
# Registry-to-registry copy preserving multi-platform manifest lists.
# Mason SA ambient auth via Cloud Build metadata server for GAR destination.
# Public upstream images need no auth.

set -euo pipefail

test -n "${_RBGA_GAR_HOST}"         || { echo "_RBGA_GAR_HOST missing"         >&2; exit 1; }
test -n "${_RBGA_GAR_PATH}"         || { echo "_RBGA_GAR_PATH missing"         >&2; exit 1; }
test -n "${_RBGA_VESSEL}"           || { echo "_RBGA_VESSEL missing"           >&2; exit 1; }
test -n "${_RBGA_CONSECRATION}"     || { echo "_RBGA_CONSECRATION missing"     >&2; exit 1; }
test -n "${_RBGA_BIND_SOURCE}"      || { echo "_RBGA_BIND_SOURCE missing"      >&2; exit 1; }
test -n "${_RBGA_ARK_SUFFIX_IMAGE}" || { echo "_RBGA_ARK_SUFFIX_IMAGE missing" >&2; exit 1; }

DEST_TAG="${_RBGA_CONSECRATION}${_RBGA_ARK_SUFFIX_IMAGE}"
DEST_REF="${_RBGA_GAR_HOST}/${_RBGA_GAR_PATH}/${_RBGA_VESSEL}:${DEST_TAG}"

echo "=== Mirroring bind image via skopeo ==="
echo "Source: ${_RBGA_BIND_SOURCE}"
echo "Dest:   ${DEST_REF}"

# Obtain OAuth2 token from Cloud Build metadata server (Mason SA)
echo "Fetching OAuth2 token from metadata server"
TOKEN_JSON=$(curl -sf -H "Metadata-Flavor: Google" \
  "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token") \
  || { echo "Failed to fetch OAuth2 token from metadata server" >&2; exit 1; }

# Extract access_token (no jq dependency — use grep+cut)
TOKEN=$(printf '%s' "${TOKEN_JSON}" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
test -n "${TOKEN}" || { echo "Failed to extract access_token from metadata response" >&2; exit 1; }

# Registry-to-registry copy preserving multi-platform manifest
skopeo copy --all \
  "docker://${_RBGA_BIND_SOURCE}" \
  "docker://${DEST_REF}" \
  --dest-creds "oauth2accesstoken:${TOKEN}" \
  || { echo "skopeo copy failed" >&2; exit 1; }

echo "Image mirrored: ${DEST_REF}"
echo "=== Mirror step complete ==="
