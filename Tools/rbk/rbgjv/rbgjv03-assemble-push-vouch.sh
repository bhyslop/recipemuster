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
# RBGJV Step 03: Build FROM scratch container with verification results, push as vouch
# Builder: docker (from reliquary)
# Entrypoint: bash
# Substitutions: _RBGV_GAR_HOST, _RBGV_GAR_PATH, _RBGV_HALLMARKS_ROOT,
#                _RBGV_HALLMARK, _RBGV_ARK_BASENAME_VOUCH
#
# Note: The Dockerfile heredoc below is intentional — this script runs inside
# a Cloud Build container, not under BCG module discipline.
#
# Platforms are read from /workspace/vouch_platforms.txt (written by step 02).
# Multi-platform via buildx. No TARGETARCH needed — vouch content (JSON files)
# is architecture-independent. Multi-platform is for pull ergonomics only.

set -euo pipefail

echo "=== Assemble and push vouch artifact ==="

# Read platforms from step 02 output
test -f /workspace/vouch_platforms.txt \
  || { echo "FATAL: /workspace/vouch_platforms.txt not found — step 02 must run first" >&2; exit 1; }
PLATFORMS=$(cat /workspace/vouch_platforms.txt)
test -n "${PLATFORMS}" || { echo "FATAL: vouch_platforms.txt is empty" >&2; exit 1; }

test -n "${_RBGV_ARK_BASENAME_VOUCH}" || { echo "FATAL: _RBGV_ARK_BASENAME_VOUCH missing" >&2; exit 1; }

VOUCH_URI="${_RBGV_GAR_HOST}/${_RBGV_GAR_PATH}/${_RBGV_HALLMARKS_ROOT}/${_RBGV_HALLMARK}/${_RBGV_ARK_BASENAME_VOUCH}:${_RBGV_HALLMARK}"
echo "Platforms: ${PLATFORMS}"
echo "Target: ${VOUCH_URI}"

mkdir -p /workspace/vouch_ctx
cp /workspace/vouch_summary.json /workspace/vouch_ctx/

# Copy per-platform verification files if they exist (conjure only)
if ls /workspace/verify-*.json >/dev/null 2>&1; then
  cp /workspace/verify-*.json /workspace/vouch_ctx/
  echo "FROM scratch" > /workspace/vouch_ctx/Dockerfile
  echo "COPY vouch_summary.json /" >> /workspace/vouch_ctx/Dockerfile
  echo "COPY verify-*.json /" >> /workspace/vouch_ctx/Dockerfile
else
  echo "FROM scratch" > /workspace/vouch_ctx/Dockerfile
  echo "COPY vouch_summary.json /" >> /workspace/vouch_ctx/Dockerfile
fi

# Ensure the shared buildx builder — shared library snippet (run once).
#@rbgjs_include buildx-bootstrap

# Push the FROM-scratch vouch context — shared library snippet.
PUSH_URI="${VOUCH_URI}"
PUSH_PLATFORMS="${PLATFORMS}"
PUSH_CTX="/workspace/vouch_ctx"
#@rbgjs_include buildx-push

echo "Vouch artifact pushed: ${VOUCH_URI}"
