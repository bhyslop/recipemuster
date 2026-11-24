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
# RBGJS buildx-bootstrap — ensure the shared docker-container buildx builder
# exists and is selected. Idempotent under Cloud Build retry: inspect-or-create,
# then use. Run once per step before any push; safe to re-run (a second inspect
# succeeds, create is skipped, use is a no-op).
#   requires: (none)
#   provides: the "rb-builder" buildx builder, created if absent and selected
docker buildx inspect rb-builder >/dev/null 2>&1 \
  || docker buildx create --driver docker-container --name rb-builder
docker buildx use rb-builder
