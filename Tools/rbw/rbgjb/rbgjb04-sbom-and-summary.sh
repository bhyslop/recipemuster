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
# RBGJB Step 04: Generate SBOM and package summary with Syft
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_MONIKER, _RBGY_GAR_HOST_SUFFIX, _RBGY_ARK_SUFFIX_IMAGE,
#                _RBGY_PLATFORMS
#
# Syft scans the image from the local Docker daemon via docker: transport.
# The image was loaded by step 03 (build-and-load). Syft runs as a sibling
# container sharing the Docker daemon socket.

set -euo pipefail

SYFT_IMAGE="${RBRR_GCB_SYFT_IMAGE_REF}"

test -s .tag_base || (echo "tag base not derived" >&2; exit 1)
TAG_BASE="$(cat .tag_base)"

IMAGE_URI="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}"

# Derive platform label for SBOM filenames (linux/amd64 → linux_amd64)
PLATFORM_LABEL="$(echo "${_RBGY_PLATFORMS}" | tr '/' '_')"

echo "SBOM generation: ${SYFT_IMAGE} scanning docker:${IMAGE_URI}"

docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  "${SYFT_IMAGE}" "docker:${IMAGE_URI}" -o json > "sbom.${PLATFORM_LABEL}.spdx.json" \
  || { echo "Syft JSON generation failed" >&2; exit 1; }

docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  "${SYFT_IMAGE}" "docker:${IMAGE_URI}" -o table > "package_summary.${PLATFORM_LABEL}.txt" \
  || { echo "Syft table generation failed" >&2; exit 1; }

test -s "sbom.${PLATFORM_LABEL}.spdx.json" || { echo "SBOM output empty" >&2; exit 1; }
