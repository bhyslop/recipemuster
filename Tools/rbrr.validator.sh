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
source "${ZRBRR_SCRIPT_DIR}/bvu_BashValidationUtility.sh"

# Container Registry Configuration
bvu_env_xname       RBRR_REGISTRY_OWNER          2     64
bvu_env_xname       RBRR_REGISTRY_NAME           2     64
bvu_env_string      RBRR_BUILD_ARCHITECTURES     1    255
bvu_env_string      RBRR_HISTORY_DIR             1    255
bvu_env_string      RBRR_NAMEPLATE_PATH          1    255
bvu_env_ipv4        RBRR_DNS_SERVER

# Machine Configuration
bvu_env_xname       RBRR_IGNITE_MACHINE_NAME     1     64
bvu_env_xname       RBRR_DEPLOY_MACHINE_NAME     1     64
bvu_env_string      RBRR_CRANE_TAR_GZ            1    512
bvu_env_string      RBRR_MANIFEST_PLATFORMS      1    512
bvu_env_string      RBRR_CHOSEN_PODMAN_VERSION   1     16
bvu_env_fqin        RBRR_CHOSEN_VMIMAGE_ORIGIN   1    256
bvu_env_string      RBRR_CHOSEN_IDENTITY         1    128

# Google Artifact Registry Configuration
bvu_env_gname       RBRR_GAR_PROJECT_ID          6     63  # GCP project ID (loose gname check; API enforces real rules)
bvu_env_gname       RBRR_GAR_LOCATION            1     32  # GCP region (loose gname check; API enforces real rules)
bvu_env_gname       RBRR_GAR_REPOSITORY          1     63  # GAR repo id (loose gname check; API enforces real rules)

# Google Cloud Build Configuration
bvu_env_gname       RBRR_GCB_PROJECT_ID          6     63  # Usually same as GAR project (loose gname check)
bvu_env_gname       RBRR_GCB_REGION              1     32  # GCP build region (loose gname check; API enforces real rules)
bvu_env_gname       RBRR_GCB_MACHINE_TYPE        1     64  # Machine type like e2-highcpu-8 (loose gname check)
bvu_env_string      RBRR_GCB_TIMEOUT             2     10  # e.g., 1200s
bvu_env_string      RBRR_GCB_STAGING_BUCKET      5    255  # gs://bucket-name

# Service Account Configuration Files
bvu_env_string      RBRR_RETRIEVER_RBRA_FILE     1    512  # Path to GAR service account env
bvu_env_string      RBRR_DIRECTOR_RBRA_FILE      1    512  # Path to GCB service account env

# Validate directories exist
bvu_dir_exists "${RBRR_HISTORY_DIR}"
bvu_dir_exists "${RBRR_NAMEPLATE_PATH}"

# Validate architecture format (os/arch)
for zrbrr_arch in ${RBRR_BUILD_ARCHITECTURES}; do
    if ! echo "${zrbrr_arch}" | grep -q '^[a-z0-9]\+/[a-z0-9_]\+$'; then
        bcu_die "Invalid architecture format in RBRR_BUILD_ARCHITECTURES: ${zrbrr_arch}. Expected format: os/arch (e.g., linux/amd64)"
    fi
done

# Validate manifest platforms format (space-separated identifiers)
for zrbrr_platform in ${RBRR_MANIFEST_PLATFORMS}; do
    if ! echo "${zrbrr_platform}" | grep -q '^[a-z0-9_]\+$'; then
        bcu_die "Invalid platform format in RBRR_MANIFEST_PLATFORMS: ${zrbrr_platform}. Expected format: lowercase alphanumeric with underscores"
    fi
done

# Validate timeout format (number followed by 's' for seconds)
if ! echo "${RBRR_GCB_TIMEOUT}" | grep -q '^[0-9]\+s$'; then
    bcu_die "Invalid RBRR_GCB_TIMEOUT format. Must be a number followed by 's' (e.g., 1200s)"
fi

# Validate GCS bucket format (must start with gs://)
if ! echo "${RBRR_GCB_STAGING_BUCKET}" | grep -q '^gs://[a-z0-9][a-z0-9._-]*[a-z0-9]$'; then
    bcu_die "Invalid RBRR_GCB_STAGING_BUCKET format. Must start with gs:// and follow GCS naming rules"
fi

# Validate Podman version format (e.g., 5.5 or 5.5.1)
if ! echo "${RBRR_CHOSEN_PODMAN_VERSION}" | grep -q '^[0-9]\+\.[0-9]\+\(\.[0-9]\+\)\?$'; then
    bcu_die "Invalid RBRR_CHOSEN_PODMAN_VERSION format. Expected semantic version like 5.5 or 5.5.1"
fi

# eof

