#!/bin/bash
# RBGJB Step 08: Generate SBOM and package summary with Syft
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_SYFT_REF

set -euo pipefail

test -n "${_RBGY_SYFT_REF}" || (echo "_RBGY_SYFT_REF missing" >&2; exit 1)
IMAGE_URI="$(cat .image_uri)"

docker run --rm "${_RBGY_SYFT_REF}" "${IMAGE_URI}" -o json  > syft_analysis.json
docker run --rm "${_RBGY_SYFT_REF}" "${IMAGE_URI}" -o table > package_summary.txt
