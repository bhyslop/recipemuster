#!/bin/bash
# RBGJB Step 08: Generate SBOM and package summary with Syft
# Builder: gcr.io/cloud-builders/docker
#
# OCI Layout Bridge Phase 3: Generate SBOM from pushed image.
# Note: Syft can't handle multi-platform OCI archives directly (GitHub #1545),
# so we pull a single-platform image and analyze that.

set -euo pipefail

# Syft image pinned via RBRR regime variable
SYFT_IMAGE="${RBRR_GCB_SYFT_IMAGE_REF}"

test -s .image_uri || (echo ".image_uri not found" >&2; exit 1)

IMAGE_URI="$(cat .image_uri)"
echo "Pulling image for SBOM analysis: ${IMAGE_URI}..."

# Pull the image (docker will use Cloud Build credentials and select current platform)
docker pull "${IMAGE_URI}"

# Analyze the pulled image
echo "Generating SBOM with ${SYFT_IMAGE}..."
docker run --rm -v /workspace:/workspace -v /var/run/docker.sock:/var/run/docker.sock \
  "${SYFT_IMAGE}" "docker:${IMAGE_URI}" -o json  > syft_analysis.json
docker run --rm -v /workspace:/workspace -v /var/run/docker.sock:/var/run/docker.sock \
  "${SYFT_IMAGE}" "docker:${IMAGE_URI}" -o table > package_summary.txt

echo "SBOM generation complete"
