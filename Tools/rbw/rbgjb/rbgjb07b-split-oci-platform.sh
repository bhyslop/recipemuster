#!/bin/sh
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
# RBGJB Step 07b: Split multi-platform OCI archive to single-platform layout
# Builder: quay.io/skopeo/stable (via RBRR_GCB_SKOPEO_IMAGE_REF)
#
# OCI Layout Bridge Phase 2b: Extract amd64 OCI layout from multi-platform archive.
# Syft cannot scan multi-platform OCI layouts (anchore/syft#1545).
# Skopeo extracts a single-platform layout so Syft can analyze it.

set -eu

test -d /workspace/oci-layout || { echo "/workspace/oci-layout not found" >&2; exit 1; }

echo "Split multi-platform OCI layout to linux/amd64"
skopeo --override-os linux --override-arch amd64 \
  copy oci:/workspace/oci-layout oci:/workspace/oci-amd64 \
  || { echo "skopeo copy failed" >&2; exit 1; }

test -d /workspace/oci-amd64 || { echo "/workspace/oci-amd64 not created by skopeo" >&2; exit 1; }
