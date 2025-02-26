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

# Allow higher level make to include the correct nameplate

# Container environment arguments for Assignment Variables
RBRN__ROLLUP_ENVIRONMENT_VAR = \
  RBRN_MONIKER='$(RBRN_MONIKER)' \
  RBRN_DESCRIPTION='$(RBRN_DESCRIPTION)' \
  RBRN_SENTRY_REPO_PATH='$(RBRN_SENTRY_REPO_PATH)' \
  RBRN_BOTTLE_REPO_PATH='$(RBRN_BOTTLE_REPO_PATH)' \
  RBRN_SENTRY_IMAGE_TAG='$(RBRN_SENTRY_IMAGE_TAG)' \
  RBRN_BOTTLE_IMAGE_TAG='$(RBRN_BOTTLE_IMAGE_TAG)' \
  RBRN_ENTRY_ENABLED='$(RBRN_ENTRY_ENABLED)' \
  RBRN_ENTRY_PORT_WORKSTATION='$(RBRN_ENTRY_PORT_WORKSTATION)' \
  RBRN_ENTRY_PORT_ENCLAVE='$(RBRN_ENTRY_PORT_ENCLAVE)' \
  RBRN_ENCLAVE_BASE_IP='$(RBRN_ENCLAVE_BASE_IP)' \
  RBRN_ENCLAVE_NETMASK='$(RBRN_ENCLAVE_NETMASK)' \
  RBRN_ENCLAVE_BOTTLE_IP='$(RBRN_ENCLAVE_BOTTLE_IP)' \
  RBRN_ENCLAVE_SENTRY_IP='$(RBRN_ENCLAVE_SENTRY_IP)' \
  RBRN_UPLINK_PORT_MIN='$(RBRN_UPLINK_PORT_MIN)' \
  RBRN_UPLINK_DNS_ENABLED='$(RBRN_UPLINK_DNS_ENABLED)' \
  RBRN_UPLINK_ACCESS_ENABLED='$(RBRN_UPLINK_ACCESS_ENABLED)' \
  RBRRN_UPLINK_DNS_GLOBAL='$(RBRN_UPLINK_DNS_GLOBAL)' \
  RBRN_UPLINK_ACCESS_GLOBAL='$(RBRN_UPLINK_ACCESS_GLOBAL)' \
  RBRN_UPLINK_ALLOWED_CIDRS='$(RBRN_UPLINK_ALLOWED_CIDRS)' \
  RBRN_UPLINK_ALLOWED_DOMAINS='$(RBRN_UPLINK_ALLOWED_DOMAINS)' \
  RBRN_VOLUME_MOUNTS='$(RBRN_VOLUME_MOUNTS)' \

# Core validation target that other parts of the system expect
rbn_validate:
	$(MBC_START) "Validating RBN nameplate configuration with RBM_MONIKER as" $(RBM_MONIKER)
	$(RBRN__ROLLUP_ENVIRONMENT_VAR) $(MBV_TOOLS_DIR)/rbn.validator.sh
	$(MBC_PASS) "No validation errors."


# eof
