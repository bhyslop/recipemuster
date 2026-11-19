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
# RBGJS buildx-push — push a prepared FROM-scratch build context as an image via
# the shared buildx builder (see buildx-bootstrap). The irreducible push shared
# by the FROM-scratch vouch-push callers (Lode rbgjl02, hallmark-verify rbgjv03):
# push cardinality, the destination URI, the platform set, and the context
# assembly (which JSON, conditional Dockerfile) are all done by the kind.
#   requires: PUSH_URI        full destination image ref including tag
#             PUSH_PLATFORMS  buildx --platform value (e.g. linux/amd64)
#             PUSH_CTX        build context dir holding the Dockerfile
#   provides: the image pushed to PUSH_URI
docker buildx build \
  --push \
  --platform="${PUSH_PLATFORMS}" \
  --tag "${PUSH_URI}" \
  "${PUSH_CTX}" \
  || { echo "FATAL: buildx push failed for ${PUSH_URI}" >&2; exit 1; }
