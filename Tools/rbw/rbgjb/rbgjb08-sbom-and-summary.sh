#!/bin/bash
# RBGJB Step 08: Generate SBOM and package summary with Syft
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_SYFT_REF
#
# OCI Layout Bridge Phase 3: Read from local OCI layout instead of registry.
# This is faster (no network) and analyzes exactly what was built locally.

set -euo pipefail

test -n "${_RBGY_SYFT_REF}" || (echo "_RBGY_SYFT_REF missing" >&2; exit 1)
test -d /workspace/oci-layout || (echo "OCI layout not found" >&2; exit 1)

docker run --rm -v /workspace:/workspace "${_RBGY_SYFT_REF}" oci-dir:/workspace/oci-layout -o json  > syft_analysis.json
docker run --rm -v /workspace:/workspace "${_RBGY_SYFT_REF}" oci-dir:/workspace/oci-layout -o table > package_summary.txt
