#!/bin/bash

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
# Compatible with Bash 3.2 (e.g., macOS default shell)

z_project_root_dir="${0%/*}/.."
cd                        "${z_project_root_dir}" || exit 1
export    BDU_REGIME_FILE="${z_project_root_dir}/.buk/burc.env"
source "${BDU_REGIME_FILE}" || exit 1
export    BDU_COORDINATOR_SCRIPT="${BURC_TOOLS_DIR}/ccbx/ccbx_coordinator.sh"
exec "${BURC_TOOLS_DIR}/buk/bdu_BashDispatchUtility.sh" "${1##*/}" "${@:2}"
