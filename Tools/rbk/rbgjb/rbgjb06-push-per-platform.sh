#!/bin/bash
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
# RBGJB Step 06: Push each per-platform attest tag to registry
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_PLATFORM_SUFFIXES,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GAR_HOST_SUFFIX, _RBGY_HALLMARKS_ROOT, _RBGY_HALLMARK,
#                _RBGY_ARK_BASENAME_ATTEST
#
# Pushes each attest tag (one per platform on the single attest package) to
# registry so they are available for the CB images: field SLSA provenance
# generation.

set -euo pipefail

test -n "${_RBGY_PLATFORM_SUFFIXES}"   || (echo "_RBGY_PLATFORM_SUFFIXES missing"   >&2; exit 1)
test -n "${_RBGY_GAR_LOCATION}"        || (echo "_RBGY_GAR_LOCATION missing"        >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"         || (echo "_RBGY_GAR_PROJECT missing"         >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}"      || (echo "_RBGY_GAR_REPOSITORY missing"      >&2; exit 1)
test -n "${_RBGY_HALLMARKS_ROOT}"      || (echo "_RBGY_HALLMARKS_ROOT missing"      >&2; exit 1)
test -n "${_RBGY_HALLMARK}"            || (echo "_RBGY_HALLMARK missing"            >&2; exit 1)
test -n "${_RBGY_ARK_BASENAME_ATTEST}" || (echo "_RBGY_ARK_BASENAME_ATTEST missing" >&2; exit 1)

test -s .hallmark || (echo "hallmark not derived" >&2; exit 1)
HALLMARK="$(cat .hallmark)"

GAR_REPO_BASE="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}"
ATTEST_BASE="${GAR_REPO_BASE}/${_RBGY_HALLMARKS_ROOT}/${HALLMARK}/${_RBGY_ARK_BASENAME_ATTEST}"

IFS=',' read -ra SUFFIXES <<< "${_RBGY_PLATFORM_SUFFIXES}"

echo "=== Pushing per-platform attest tags to registry ==="
for SUFFIX in "${SUFFIXES[@]}"; do
  ATTEST_URI="${ATTEST_BASE}:${HALLMARK}${SUFFIX}"
  echo "Pushing: ${ATTEST_URI}"
  docker push "${ATTEST_URI}"
done
# Capture Docker daemon state after pushes (forwarded to about via diags)
# Format: {"timestamp":"...","host_daemon_images":[...]} matching inspect's jq queries
CACHE_TS_AFTER="$(date -u +%FT%TZ)"
docker images --no-trunc --format '{{json .}}' > cache_after_raw.txt
{
  printf '{"timestamp":"%s","host_daemon_images":[' "${CACHE_TS_AFTER}"
  awk 'NR>1{printf ","}{printf "%s",$0}' cache_after_raw.txt
  printf ']}'
} > cache_after.json
rm -f cache_after_raw.txt
echo "cache_after.json written ($(wc -c < cache_after.json | tr -d ' ') bytes)"

echo "=== Push complete ==="
