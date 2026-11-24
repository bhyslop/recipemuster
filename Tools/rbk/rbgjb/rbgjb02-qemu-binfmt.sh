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
# RBGJB Step 02: Register QEMU for cross-platform builds
# Builder: gcr.io/cloud-builders/docker

set -euo pipefail

# Register QEMU for cross-platform builds (arm64, arm/v7)
docker run --privileged --rm "${ZRBF_TOOL_BINFMT}" --install arm64,arm
