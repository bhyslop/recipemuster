#!/bin/bash

# Copyright 2024 Scale Invariant, Inc.
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

# Nameplate Validator

set -e  # Exit immediately if a command exits with non-zero status

# Find tools in same directory
SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "$SCRIPT_DIR/bvu_BashValidationUtility.sh"

# Core Service Identity
bvu_env_xname       RBRN_MONIKER 2 12
bvu_env_string      RBRN_DESCRIPTION 0 120
bvu_env_fqin        RBRN_SENTRY_REPO_PATH 1 128
bvu_env_fqin        RBRN_BOTTLE_REPO_PATH 1 128
bvu_env_fqin        RBRN_SENTRY_IMAGE_TAG 1 128
bvu_env_fqin        RBRN_BOTTLE_IMAGE_TAG 1 128
bvu_env_bool        RBRN_ENTRY_ENABLED

# Enclave Network Configuration
bvu_env_ipv4        RBRN_ENCLAVE_BASE_IP
bvu_env_decimal     RBRN_ENCLAVE_NETMASK 8 30
bvu_env_ipv4        RBRN_ENCLAVE_SENTRY_IP
bvu_env_ipv4        RBRN_ENCLAVE_BOTTLE_IP

# Uplink Configuration
bvu_env_port        RBRN_UPLINK_PORT_MIN
bvu_env_bool        RBRN_UPLINK_DNS_ENABLED
bvu_env_bool        RBRN_UPLINK_ACCESS_ENABLED
bvu_env_bool        RBRN_UPLINK_DNS_GLOBAL
bvu_env_bool        RBRN_UPLINK_ACCESS_GLOBAL

if [[ $RBRN_ENTRY_ENABLED == 1 ]]; then
    bvu_env_port    RBRN_ENTRY_PORT_WORKSTATION
    bvu_env_port    RBRN_ENTRY_PORT_ENCLAVE
    
    # Can I weaken below check?
    test       ${RBRN_ENTRY_PORT_WORKSTATION} -lt            ${RBRN_UPLINK_PORT_MIN} || \
        bcu_die "RBRN_ENTRY_PORT_WORKSTATION must be less than RBRN_UPLINK_PORT_MIN"
    test       ${RBRN_ENTRY_PORT_ENCLAVE} -lt            ${RBRN_UPLINK_PORT_MIN} || \
        bcu_die "RBRN_ENTRY_PORT_ENCLAVE must be less than RBRN_UPLINK_PORT_MIN"
fi

# These two are mutually exclusive
if [[ ${RBRN_UPLINK_ACCESS_ENABLED} == 1 && ${RBRN_UPLINK_ACCESS_GLOBAL} == 0 ]]; then
    bvu_env_list_cidr RBRN_UPLINK_ALLOWED_CIDRS
fi
if [[ ${RBRN_UPLINK_DNS_ENABLED} == 1 && ${RBRN_UPLINK_DNS_GLOBAL} == 0 ]]; then
    bvu_env_list_domain RBRN_UPLINK_ALLOWED_DOMAINS
fi

bvu_env_string      RBRN_VOLUME_MOUNTS 0 240

# Success

# eof

