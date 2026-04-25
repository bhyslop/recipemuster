#!/bin/bash
# RBGJM Step 01: Mirror image from upstream to GAR via skopeo
# Builder: skopeo (from reliquary)
# Substitutions: _RBGA_GAR_HOST, _RBGA_GAR_PATH, _RBGA_HALLMARKS_ROOT,
#                _RBGA_HALLMARK, _RBGA_BIND_SOURCE
#
# Registry-to-registry copy preserving multi-platform manifest lists.
# Mason SA ambient auth via Cloud Build metadata server for GAR destination.
# Public upstream images need no auth.
#
# Destination uses canonical AAK shape:
#   ${gar_host}/${gar_path}/${HALLMARKS_ROOT}/${hallmark}/image:${hallmark}
# matching rbgja02 (syft), rbgja04 (about), rbgjv03 (vouch).

set -euo pipefail

test -n "${_RBGA_GAR_HOST}"        || { echo "_RBGA_GAR_HOST missing"        >&2; exit 1; }
test -n "${_RBGA_GAR_PATH}"        || { echo "_RBGA_GAR_PATH missing"        >&2; exit 1; }
test -n "${_RBGA_HALLMARKS_ROOT}"  || { echo "_RBGA_HALLMARKS_ROOT missing"  >&2; exit 1; }
test -n "${_RBGA_HALLMARK}"        || { echo "_RBGA_HALLMARK missing"        >&2; exit 1; }
test -n "${_RBGA_BIND_SOURCE}"     || { echo "_RBGA_BIND_SOURCE missing"     >&2; exit 1; }

DEST_REF="${_RBGA_GAR_HOST}/${_RBGA_GAR_PATH}/${_RBGA_HALLMARKS_ROOT}/${_RBGA_HALLMARK}/image:${_RBGA_HALLMARK}"

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
