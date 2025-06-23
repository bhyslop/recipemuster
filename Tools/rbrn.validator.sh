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

CONTEXT="NAMEPLATE"

# Core Service Identity
crgv_xname       "$CONTEXT"       RBRN_MONIKER 2 12
crgv_string      "$CONTEXT"       RBRN_DESCRIPTION  0 120
crgv_fqin        "$CONTEXT"       RBRN_SENTRY_REPO_PATH 1 128
crgv_fqin        "$CONTEXT"       RBRN_BOTTLE_REPO_PATH 1 128
crgv_fqin        "$CONTEXT"       RBRN_SENTRY_IMAGE_TAG 1 128
crgv_fqin        "$CONTEXT"       RBRN_BOTTLE_IMAGE_TAG 1 128
crgv_bool        "$CONTEXT"       RBRN_ENTRY_ENABLED

# Uplink Configuration
crgv_port        "$CONTEXT"       RBRN_UPLINK_PORT_MIN
crgv_bool        "$CONTEXT"       RBRN_UPLINK_DNS_ENABLED
crgv_bool        "$CONTEXT"       RBRN_UPLINK_ACCESS_ENABLED
crgv_bool        "$CONTEXT"       RBRN_UPLINK_DNS_GLOBAL
crgv_bool        "$CONTEXT"       RBRN_UPLINK_ACCESS_GLOBAL

if [[ $RBRN_ENTRY_ENABLED == 1 ]]; then
    crgv_port    "$CONTEXT"       RBRN_ENTRY_PORT_WORKSTATION
    crgv_port    "$CONTEXT"       RBRN_ENTRY_PORT_ENCLAVE
    
    # Can I weaken below check?
    test $RBRN_ENTRY_PORT_WORKSTATION -lt                                 $RBRN_UPLINK_PORT_MIN || \
        crgv_print_and_die RBRN_ENTRY_PORT_WORKSTATION "must be less than" RBRN_UPLINK_PORT_MIN
    test $RBRN_ENTRY_PORT_ENCLAVE     -lt                                 $RBRN_UPLINK_PORT_MIN || \
        crgv_print_and_die RBRN_ENTRY_PORT_ENCLAVE     "must be less than" RBRN_UPLINK_PORT_MIN
fi

# These two are mutually exclusive, can do this better
if [[ $RBRN_UPLINK_ACCESS_ENABLED == 1 && $RBRN_UPLINK_ACCESS_GLOBAL == 0 ]]; then
    crgv_list_cidr    "$CONTEXT"  RBRN_UPLINK_ALLOWED_CIDRS
fi
if [[ $RBRN_UPLINK_DNS_ENABLED == 1 && $RBRN_UPLINK_DNS_GLOBAL == 0 ]]; then
    crgv_list_domain  "$CONTEXT"  RBRN_UPLINK_ALLOWED_DOMAINS
fi

crgv_string      "$CONTEXT"       RBRN_VOLUME_MOUNTS 0 240

# Success
exit 0


# eof
