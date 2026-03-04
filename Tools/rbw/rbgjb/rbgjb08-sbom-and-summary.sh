#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# RBGJB Step 08: Generate SBOM and package summary with Syft
# Builder: gcr.io/cloud-builders/docker
#
# OCI Layout Bridge Phase 3: Generate SBOM from single-platform OCI layout.
# Step 07b extracts linux/amd64 layout from multi-platform archive;
# Syft analyzes the single-platform layout via oci-dir: scheme.
# SBOM is labeled with platform (amd64).

set -euo pipefail

SYFT_IMAGE="${RBRR_GCB_SYFT_IMAGE_REF}"

test -d /workspace/oci-amd64 || { echo "/workspace/oci-amd64 not found (step 07b must run first)" >&2; exit 1; }

echo "SBOM generation: ${SYFT_IMAGE}"
docker run --rm -v /workspace:/workspace \
  "${SYFT_IMAGE}" "oci-dir:/workspace/oci-amd64" -o json > sbom.linux_amd64.spdx.json \
  || { echo "Syft JSON generation failed" >&2; exit 1; }

docker run --rm -v /workspace:/workspace \
  "${SYFT_IMAGE}" "oci-dir:/workspace/oci-amd64" -o table > package_summary.linux_amd64.txt \
  || { echo "Syft table generation failed" >&2; exit 1; }

test -s sbom.linux_amd64.spdx.json || { echo "SBOM output empty" >&2; exit 1; }
