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

# Regime Prefix: rbn_
# Assignment Prefix: RBN_

# Core validation target that other parts of the system expect
rbn_validate:
	$(MBC_START) "Validating RBN nameplate configuration with RBM_MONIKER as" $(RBM_MONIKER)
	$(MBV_TOOLS_DIR)/rbn.validator.sh
	$(MBC_PASS) "No validation errors."

# Container environment arguments for Assignment Variables
RBN__ROLLUP_ENVIRONMENT_VAR = \
  RBN_MONIKER='$(RBN_MONIKER)' \
  RBN_DESCRIPTION='$(RBN_DESCRIPTION)' \
  RBN_SENTRY_REPO_PATH='$(RBN_SENTRY_REPO_PATH)' \
  RBN_BOTTLE_REPO_PATH='$(RBN_BOTTLE_REPO_PATH)' \
  RBN_SENTRY_IMAGE_TAG='$(RBN_SENTRY_IMAGE_TAG)' \
  RBN_BOTTLE_IMAGE_TAG='$(RBN_BOTTLE_IMAGE_TAG)' \
  RBN_ENTRY_ENABLED='$(RBN_ENTRY_ENABLED)' \
  RBN_ENTRY_PORT_WORKSTATION='$(RBN_ENTRY_PORT_WORKSTATION)' \
  RBN_ENTRY_PORT_ENCLAVE='$(RBN_ENTRY_PORT_ENCLAVE)' \
  RBN_ENCLAVE_BASE_IP='$(RBN_ENCLAVE_BASE_IP)' \
  RBN_ENCLAVE_NETMASK='$(RBN_ENCLAVE_NETMASK)' \
  RBN_ENCLAVE_BOTTLE_IP='$(RBN_ENCLAVE_BOTTLE_IP)' \
  RBN_ENCLAVE_SENTRY_IP='$(RBN_ENCLAVE_SENTRY_IP)' \
  RBN_UPLINK_PORT_MIN='$(RBN_UPLINK_PORT_MIN)' \
  RBN_UPLINK_DNS_ENABLED='$(RBN_UPLINK_DNS_ENABLED)' \
  RBN_UPLINK_ACCESS_ENABLED='$(RBN_UPLINK_ACCESS_ENABLED)' \
  RBN_UPLINK_DNS_GLOBAL='$(RBN_UPLINK_DNS_GLOBAL)' \
  RBN_UPLINK_ACCESS_GLOBAL='$(RBN_UPLINK_ACCESS_GLOBAL)' \
  RBN_UPLINK_ALLOWED_CIDRS='$(RBN_UPLINK_ALLOWED_CIDRS)' \
  RBN_UPLINK_ALLOWED_DOMAINS='$(RBN_UPLINK_ALLOWED_DOMAINS)' \
  RBN_VOLUME_MOUNTS='$(RBN_VOLUME_MOUNTS)'


# eof
