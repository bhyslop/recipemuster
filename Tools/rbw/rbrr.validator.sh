#!/bin/bash
#
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
#

set -euo pipefail

ZRBRR_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
source "${ZRBRR_SCRIPT_DIR}/buv_validation.sh"

# Container Registry Configuration
buv_env_xname       RBRR_REGISTRY_OWNER          2     64
buv_env_xname       RBRR_REGISTRY_NAME           2     64
buv_env_string      RBRR_HISTORY_DIR             1    255
buv_env_string      RBRR_NAMEPLATE_PATH          1    255
buv_env_string      RBRR_VESSEL_DIR              1    255
buv_env_ipv4        RBRR_DNS_SERVER

# Machine Configuration
buv_env_xname       RBRR_IGNITE_MACHINE_NAME     1     64
buv_env_xname       RBRR_DEPLOY_MACHINE_NAME     1     64
buv_env_string      RBRR_CRANE_TAR_GZ            1    512
buv_env_string      RBRR_MANIFEST_PLATFORMS      1    512
buv_env_string      RBRR_CHOSEN_PODMAN_VERSION   1     16
buv_env_fqin        RBRR_CHOSEN_VMIMAGE_ORIGIN   1    256
buv_env_string      RBRR_CHOSEN_IDENTITY         1    128
buv_env_gname       RBRR_GCP_PROJECT_ID          6     63  # Base GCP project ID
buv_env_gname       RBRR_GCP_REGION              1     32  # Base GCP region

# Google Artifact Registry Configuration
buv_env_gname       RBRR_GAR_REPOSITORY          1     63  # GAR repo id (loose gname check; API enforces real rules)

# Google Cloud Build Configuration
buv_env_string      RBRR_GCB_MACHINE_TYPE        3     64  # Cloud Build machine type (API-native form, e.g., E2_HIGHCPU_8)
buv_env_string      RBRR_GCB_TIMEOUT             2     10  # Cloud Build timeout (seconds form, e.g., 600s, 1200s)

# Service Account Configuration Files
buv_env_string      RBRR_ADMIN_RBRA_FILE         1    512  # Path to administrative service account env
buv_env_string      RBRR_RETRIEVER_RBRA_FILE     1    512  # Path to GAR service account env
buv_env_string      RBRR_DIRECTOR_RBRA_FILE      1    512  # Path to GCB service account env

# Validating GCB image pins (digest-pinned)
buv_env_odref       RBRR_GCB_JQ_IMAGE_REF
buv_env_odref       RBRR_GCB_SYFT_IMAGE_REF
buv_env_odref       RBRR_GCB_GCRANE_IMAGE_REF
buv_env_odref       RBRR_GCB_ORAS_IMAGE_REF

# Validate directories exist
buv_dir_exists "${RBRR_HISTORY_DIR}"
buv_dir_exists "${RBRR_NAMEPLATE_PATH}"

# Validate manifest platforms format (space-separated identifiers)
for zrbrr_platform in ${RBRR_MANIFEST_PLATFORMS}; do
    if ! echo "${zrbrr_platform}" | grep -q '^[a-z0-9_]\+$'; then
        buc_die "Invalid platform format in RBRR_MANIFEST_PLATFORMS: ${zrbrr_platform}. Expected format: lowercase alphanumeric with underscores"
    fi
done

# Validate timeout format (number followed by 's' for seconds)
if ! echo "${RBRR_GCB_TIMEOUT}" | grep -q '^[0-9]\+s$'; then
    buc_die "Invalid RBRR_GCB_TIMEOUT format. Must be a number followed by 's' (e.g., 1200s)"
fi

# Validate Podman version format (e.g., 5.5 or 5.5.1)
if ! echo "${RBRR_CHOSEN_PODMAN_VERSION}" | grep -q '^[0-9]\+\.[0-9]\+\(\.[0-9]\+\)\?$'; then
    buc_die "Invalid RBRR_CHOSEN_PODMAN_VERSION format. Expected semantic version like 5.5 or 5.5.1"
fi


# eof

