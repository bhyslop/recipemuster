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

# RBN Configuration Validator

source crgv.validate.sh

# Core Service Identity
crgl_is_boolean RBN_MONIKER
crgl_is_boolean RBN_DESCRIPTION

# Container Image Configuration
crgl_is_domain RBN_SENTRY_REPO_PATH
crgl_is_domain RBN_BOTTLE_REPO_PATH
# TODO: Need specialized validation for image tags
crgl_is_boolean RBN_SENTRY_IMAGE_TAG
crgl_is_boolean RBN_BOTTLE_IMAGE_TAG

# Entry Service Configuration
crgl_is_boolean RBN_ENTRY_ENABLED

if [[ $RBN_ENTRY_ENABLED == 1 ]]; then
    crgl_is_port RBN_ENTRY_PORT_WORKSTATION
    crgl_is_port RBN_ENTRY_PORT_ENCLAVE
    
    # TODO: Entry ports must be lower than RBN_UPLINK_PORT_MIN
    test $RBN_ENTRY_PORT_WORKSTATION -lt $RBN_UPLINK_PORT_MIN || \
        crgl_die "RBN_ENTRY_PORT_WORKSTATION must be less than RBN_UPLINK_PORT_MIN"
    test $RBN_ENTRY_PORT_ENCLAVE -lt $RBN_UPLINK_PORT_MIN || \
        crgl_die "RBN_ENTRY_PORT_ENCLAVE must be less than RBN_UPLINK_PORT_MIN"
fi

# Enclave Network Configuration
crgl_is_ipv4 RBN_ENCLAVE_BASE_IP
crgl_in_range RBN_ENCLAVE_NETMASK 8 30
crgl_is_ipv4 RBN_ENCLAVE_SENTRY_IP
crgl_is_ipv4 RBN_ENCLAVE_BOTTLE_IP

# TODO: Validate IPs are within network defined by BASE_IP/NETMASK

# Uplink Configuration
crgl_is_port RBN_UPLINK_PORT_MIN
crgl_is_boolean RBN_UPLINK_DNS_ENABLED
crgl_is_boolean RBN_UPLINK_ACCESS_ENABLED
crgl_is_boolean RBN_UPLINK_DNS_GLOBAL
crgl_is_boolean RBN_UPLINK_ACCESS_GLOBAL

if [[ $RBN_UPLINK_ACCESS_ENABLED == 1 && $RBN_UPLINK_ACCESS_GLOBAL == 0 ]]; then
    crgl_is_cidr_list RBN_UPLINK_ALLOWED_CIDRS
fi

if [[ $RBN_UPLINK_DNS_ENABLED == 1 && $RBN_UPLINK_DNS_GLOBAL == 0 ]]; then
    crgl_is_domain_list RBN_UPLINK_ALLOWED_DOMAINS
fi

# Volume Mount Configuration
# Note: Complex format validation deferred to podman

# Success
exit 0

# eof
