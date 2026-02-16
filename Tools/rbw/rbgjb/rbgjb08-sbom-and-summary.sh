#!/bin/bash
# RBGJB Step 08: Generate SBOM and package summary with Syft
# Builder: gcr.io/cloud-builders/docker
#
# OCI Layout Bridge Phase 3: Generate SBOM from OCI layout directory.
# Syft analyzes /workspace/oci-layout directly via oci-dir: scheme,
# avoiding platform mismatch when build arch differs from GCB worker arch.

set -euo pipefail

# Syft image pinned via RBRR regime variable
SYFT_IMAGE="${RBRR_GCB_SYFT_IMAGE_REF}"

test -d /workspace/oci-layout || (echo "/workspace/oci-layout not found" >&2; exit 1)

echo "Generating SBOM with ${SYFT_IMAGE}..."
docker run --rm -v /workspace:/workspace \
  "${SYFT_IMAGE}" "oci-dir:/workspace/oci-layout" -o json  > syft_analysis.json
docker run --rm -v /workspace:/workspace \
  "${SYFT_IMAGE}" "oci-dir:/workspace/oci-layout" -o table > package_summary.txt

echo "SBOM generation complete"
