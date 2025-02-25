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

# Include the config file here for easier github action workflow integration
include rbrr.repo.mk

# Container environment arguments for Assignment Variables
RBRR__ROLLUP_ENVIRONMENT_VAR = \
  RBRR_REGISTRY_SERVER='$(RBRR_REGISTRY_SERVER)' \
  RBRR_REGISTRY_OWNER='$(RBRR_REGISTRY_OWNER)' \
  RBRR_REGISTRY_NAME='$(RBRR_REGISTRY_NAME)' \
  RBRR_BUILD_ARCHITECTURES='$(RBRR_BUILD_ARCHITECTURES)' \
  RBRR_HISTORY_DIR='$(RBRR_HISTORY_DIR)' \
  RBRR_DNS_SERVER='$(RBRR_DNS_SERVER)' \
  RBRR_NAMEPLATE_PATH='$(RBRR_NAMEPLATE_PATH)' \
  RBRR_DNS_SERVER='$(RBRR_DNS_SERVER)' \
  RBRR_GITHUB_PAT_ENV='$(RBRR_GITHUB_PAT_ENV)' \

# Core validation target that other parts of the system expect
rbrr_validate:
	$(MBC_START) "Validating RBRR repository configuration"
	$(RBRR__ROLLUP_ENVIRONMENT_VAR) $(MBV_TOOLS_DIR)/rbrr.validator.sh
	$(MBC_PASS) "No validation errors."

# GitHub Actions environment export function - explicit version
rbrr_export_github_env:
	@echo 'echo "RBRR_REGISTRY_SERVER=$(RBRR_REGISTRY_SERVER)"         >> $$GITHUB_ENV'
	@echo 'echo "RBRR_REGISTRY_OWNER=$(RBRR_REGISTRY_OWNER)"           >> $$GITHUB_ENV'
	@echo 'echo "RBRR_REGISTRY_NAME=$(RBRR_REGISTRY_NAME)"             >> $$GITHUB_ENV'
	@echo 'echo "RBRR_BUILD_ARCHITECTURES=$(RBRR_BUILD_ARCHITECTURES)" >> $$GITHUB_ENV'
	@echo 'echo "RBRR_HISTORY_DIR=$(RBRR_HISTORY_DIR)"                 >> $$GITHUB_ENV'
	@echo 'echo "RBRR_NAMEPLATE_PATH=$(RBRR_NAMEPLATE_PATH)"           >> $$GITHUB_ENV'
	@echo 'echo "RBRR_DNS_SERVER=$(RBRR_DNS_SERVER)"                   >> $$GITHUB_ENV'
	@echo 'echo "RBRR_GITHUB_PAT_ENV=$(RBRR_GITHUB_PAT_ENV)"           >> $$GITHUB_ENV'

# eof
