#!/bin/bash
# RBGJB Step 07: Push OCI layout to GAR using crane
# Builder: gcr.io/go-containerregistry/gcrane (via RBRR_GCB_GCRANE_IMAGE_REF)
# Substitutions: _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY, _RBGY_MONIKER
#
# OCI Layout Bridge Phase 2: Push the multi-platform OCI archive from /workspace/oci-layout.tar
# to Artifact Registry using crane. Auth provided by gcloud auth configure-docker (step 03).

set -euo pipefail

test -n "${_RBGY_GAR_LOCATION}"   || { echo "_RBGY_GAR_LOCATION missing"   >&2; exit 1; }
test -n "${_RBGY_GAR_PROJECT}"    || { echo "_RBGY_GAR_PROJECT missing"    >&2; exit 1; }
test -n "${_RBGY_GAR_REPOSITORY}" || { echo "_RBGY_GAR_REPOSITORY missing" >&2; exit 1; }
test -n "${_RBGY_MONIKER}"        || { echo "_RBGY_MONIKER missing"        >&2; exit 1; }
test -s .tag_base                  || { echo "tag base not derived"         >&2; exit 1; }
test -f /workspace/oci-layout.tar  || { echo "OCI archive not found"        >&2; exit 1; }

TAG_BASE="$(cat .tag_base)"
IMAGE_URI="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}"

mkdir -p /workspace/oci-layout
tar xf /workspace/oci-layout.tar -C /workspace/oci-layout

echo "Pushing OCI layout to ${IMAGE_URI}..."
crane push /workspace/oci-layout "${IMAGE_URI}" --index

echo "${IMAGE_URI}" > .image_uri
echo "Done. Image available at: ${IMAGE_URI}"
