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

set -e

ZRBRR_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBRR_SCRIPT_DIR}/bvu_BashValidationUtility.sh"

bvu_env_xname       RBRR_REGISTRY_OWNER          2     64
bvu_env_xname       RBRR_REGISTRY_NAME           2     64
bvu_env_string      RBRR_GITHUB_PAT_ENV          1    255

bvu_env_string      RBRR_BUILD_ARCHITECTURES     1    255
bvu_env_string      RBRR_HISTORY_DIR             1    255
bvu_env_ipv4        RBRR_DNS_SERVER
bvu_env_string      RBRR_NAMEPLATE_PATH          1    255

bvu_env_string      RBRR_IGNITE_MACHINE_NAME     1     64
bvu_env_string      RBRR_DEPLOY_MACHINE_NAME     1     64
bvu_env_string      RBRR_CRANE_TAR_GZ            1    512
bvu_env_string      RBRR_CHOSEN_PODMAN_VERSION   1     16
bvu_env_string      RBRR_CHOSEN_VMIMAGE_ORIGIN   1    128
bvu_env_string      RBRR_CHOSEN_IDENTITY         1    128


bvu_dir_exists "${RBRR_HISTORY_DIR}"
bvu_dir_exists "${RBRR_NAMEPLATE_PATH}"

for zrbrr_arch in $RBRR_BUILD_ARCHITECTURES; do
    if ! echo "${zrbrr_arch}" | grep -q '^[a-z0-9]\+/[a-z0-9]\+$'; then
        bcu_die "Invalid architecture format in RBRR_BUILD_ARCHITECTURES: ${zrbrr_arch}. Expected format: os/arch (e.g., linux/amd64)"
    fi
done

# eof

