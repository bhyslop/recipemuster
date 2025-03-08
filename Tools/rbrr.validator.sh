#!/bin/bash

# Copyright 2025 Scale Invariant, Inc.
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

# Repository Configuration Validator

set -e  # Exit immediately if a command exits with non-zero status

# Find tools in same directory
SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "$SCRIPT_DIR/crgv.validate.sh"

# Registry Configuration
crgv_xname               RBRR_REGISTRY_OWNER 2 64
crgv_xname               RBRR_REGISTRY_NAME 2 64
crgv_string              RBRR_GITHUB_PAT_ENV 1 255

# Build Configuration
crgv_string              RBRR_BUILD_ARCHITECTURES 1 255
crgv_string              RBRR_HISTORY_DIR 1 255
crgv_ipv4                RBRR_DNS_SERVER
crgv_string              RBRR_NAMEPLATE_PATH 1 255

# Podman configuration
crgv_string              RBRR_MACHINE_NAME  1 64
crgv_xname               RBRR_VMDIST_TAG 1 128
crgv_xname               RBRR_VMDIST_RAW_ARCH 1 64
crgv_xname               RBRR_VMDIST_SKOPEO_ARCH 1 64
crgv_xname               RBRR_VMDIST_BLOB_SHA 64 64

# Verify directories exist
if [ ! -d "$RBRR_HISTORY_DIR" ]; then
    crgv_print_and_die "RBRR_HISTORY_DIR directory '$RBRR_HISTORY_DIR' does not exist"
fi

if [ ! -d "$RBRR_NAMEPLATE_PATH" ]; then
    crgv_print_and_die "RBRR_NAMEPLATE_PATH directory '$RBRR_NAMEPLATE_PATH' does not exist"
fi

# Validate build architectures format (platform identifiers)
for arch in $RBRR_BUILD_ARCHITECTURES; do
    if ! echo "$arch" | grep -q '^[a-z0-9]\+/[a-z0-9]\+$'; then
        crgv_print_and_die "Invalid architecture format in RBRR_BUILD_ARCHITECTURES: $arch. Expected format: os/arch (e.g., linux/amd64)"
    fi
done

# Success
exit 0


# eof
