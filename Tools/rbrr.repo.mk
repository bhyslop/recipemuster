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

# Regime Prefix: rbrr_
# Assignment Prefix: RBRR_

# Core validation target that other parts of the system expect
rbrr_validate:
	$(MBC_START) "Validating RBRR repository configuration"
	$(MBV_TOOLS_DIR)/rbrr.validator.sh
	$(MBC_PASS) "No validation errors."


# Container environment arguments for Assignment Variables
RBRR__ROLLUP_ENVIRONMENT_VAR = \
  RBRR_REGISTRY_SERVER='$(RBRR_REGISTRY_SERVER)' \
  RBRR_REGISTRY_OWNER='$(RBRR_REGISTRY_OWNER)' \
  RBRR_REGISTRY_NAME='$(RBRR_REGISTRY_NAME)' \
  RBRR_BUILD_ARCHITECTURES='$(RBRR_BUILD_ARCHITECTURES)' \
  RBRR_HISTORY_DIR='$(RBRR_HISTORY_DIR)' \
  RBRR_DNS_SERVER='$(RBRR_DNS_SERVER)' \
  RBRR_NAMEPLATE_PATH='$(RBRR_NAMEPLATE_PATH)'


# eof
