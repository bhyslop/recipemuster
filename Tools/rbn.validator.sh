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
source "$SCRIPT_DIR/crgv.validate.sh"

# Core Service Identity
crgv_xname              RBN_MONIKER 2 12
crgv_string             RBN_DESCRIPTION  0 120
crgv_fqin               RBN_SENTRY_REPO_PATH 1 128
crgv_fqin               RBN_BOTTLE_REPO_PATH 1 128
crgv_fqin               RBN_SENTRY_IMAGE_TAG 1 128
crgv_fqin               RBN_BOTTLE_IMAGE_TAG 1 128
crgv_bool               RBN_ENTRY_ENABLED

# Enclave Network Configuration
crgv_ipv4               RBN_ENCLAVE_BASE_IP
crgv_decimal            RBN_ENCLAVE_NETMASK 8 30
crgv_ipv4               RBN_ENCLAVE_SENTRY_IP
crgv_ipv4               RBN_ENCLAVE_BOTTLE_IP

# Uplink Configuration
crgv_port               RBN_UPLINK_PORT_MIN
crgv_bool               RBN_UPLINK_DNS_ENABLED
crgv_bool               RBN_UPLINK_ACCESS_ENABLED
crgv_bool               RBN_UPLINK_DNS_GLOBAL
crgv_bool               RBN_UPLINK_ACCESS_GLOBAL

if [[ $RBN_ENTRY_ENABLED == 1 ]]; then
    crgv_port           RBN_ENTRY_PORT_WORKSTATION
    crgv_port           RBN_ENTRY_PORT_ENCLAVE
    
    # Can I weaken below check?
    test $RBN_ENTRY_PORT_WORKSTATION -lt $RBN_UPLINK_PORT_MIN || \
        crgv_print_and_die RBN_ENTRY_PORT_WORKSTATION "must be less than" RBN_UPLINK_PORT_MIN
    test $RBN_ENTRY_PORT_ENCLAVE     -lt $RBN_UPLINK_PORT_MIN || \
        crgv_print_and_die RBN_ENTRY_PORT_ENCLAVE "must be less than" RBN_UPLINK_PORT_MIN
fi

# These two are mutually exclusive, can do this better
if [[ $RBN_UPLINK_ACCESS_ENABLED == 1 && $RBN_UPLINK_ACCESS_GLOBAL == 0 ]]; then
    crgv_list_cidr      RBN_UPLINK_ALLOWED_CIDRS
fi
if [[ $RBN_UPLINK_DNS_ENABLED == 1 && $RBN_UPLINK_DNS_GLOBAL == 0 ]]; then
    crgv_list_domain    RBN_UPLINK_ALLOWED_DOMAINS
fi

crgv_string             RBN_VOLUME_MOUNTS 0 240

# Success
exit 0


# eof
