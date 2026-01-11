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
# VVG CLI - Command line interface for VVK Git utilities

set -euo pipefail

ZVVG_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZVVG_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZVVG_CLI_SCRIPT_DIR}/vvg_git.sh"

zvvg_furnish() {
  zvvg_kindle
}

buc_execute vvg_ "VVK Git Utilities" zvvg_furnish "$@"

# eof
