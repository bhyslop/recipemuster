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
# Recipe Bottle GCP Artifact Registry - Command Line Interface

set -euo pipefail

ZRBGA_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source all dependencies
source "${ZRBGA_CLI_SCRIPT_DIR}/buc_command.sh"
source "${ZRBGA_CLI_SCRIPT_DIR}/buv_validation.sh"
source "${ZRBGA_CLI_SCRIPT_DIR}/rbl_Locator.sh"
source "${ZRBGA_CLI_SCRIPT_DIR}/rbgc_Constants.sh"
source "${ZRBGA_CLI_SCRIPT_DIR}/rbgo_OAuth.sh"
source "${ZRBGA_CLI_SCRIPT_DIR}/rbgu_Utility.sh"
source "${ZRBGA_CLI_SCRIPT_DIR}/rbgi_IAM.sh"
source "${ZRBGA_CLI_SCRIPT_DIR}/rbga_ArtifactRegistry.sh"

# Initialize modules
rbl_kindle_all

burd_dispatch "$@"

# eof
