#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
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
# RBGJB Step 01: Receive host-minted hallmark and write to workspace
# Builder: gcr.io/cloud-builders/gcloud
#
# Hallmark format: [cbgk]YYMMDDHHMMSS-rYYMMDDHHMMSS (conjure/bind/graft)
#                  kYYMMDDHHMMSS-{git_context} (kludge)
# Host mints the full hallmark and passes it via _RBGY_HALLMARK substitution.
# This step writes it to .hallmark (workspace) and /builder/outputs/output
# (Cloud Build step output mechanism for host-side consistency assertion).

set -euo pipefail

echo "Build strategy: ${ZRBF_BUILD_STRATEGY}"

HALLMARK="${_RBGY_HALLMARK}"
test -n "${HALLMARK}" || { echo "_RBGY_HALLMARK empty" >&2; exit 1; }
echo "${HALLMARK}" > .hallmark

# Expose hallmark via Cloud Build step output mechanism
# Results appear in results.buildStepOutputs (base64-encoded, max 50 bytes)
echo -n "${HALLMARK}" > /builder/outputs/output
