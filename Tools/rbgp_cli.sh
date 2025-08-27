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
# Recipe Bottle GCP Payor - Billing and Destructive Operations CLI

set -euo pipefail

ZRBGP_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source all dependencies
source "${ZRBGP_CLI_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBGP_CLI_SCRIPT_DIR}/bvu_BashValidationUtility.sh"
source "${ZRBGP_CLI_SCRIPT_DIR}/rbl_Locator.sh"
source "${ZRBGP_CLI_SCRIPT_DIR}/rbgc_Constants.sh"
source "${ZRBGP_CLI_SCRIPT_DIR}/rbgo_GoogleOAuth.sh"
source "${ZRBGP_CLI_SCRIPT_DIR}/rbgu_Utility.sh"
source "${ZRBGP_CLI_SCRIPT_DIR}/rbgp_Payor.sh"

# Initialize modules
rbl_kindle_all

bdu_dispatch "$@"

# eof
