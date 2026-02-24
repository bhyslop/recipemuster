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
# VOB - VOK Build Module - Command Line Interface

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

zvob_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env "BURD_TEMP_DIR         " "Temporary directory for intermediate files"
  buc_doc_env "BURC_TOOLS_DIR        " "Directory for tools"
  buc_doc_env_done || return 0

  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_TOOLS_DIR}/vok/vof_features.sh"
  source "${BURD_TOOLS_DIR}/vvk/vvb_bash.sh"
  source "${BURD_TOOLS_DIR}/vok/vob_build.sh"

  zbuv_kindle
  zburd_kindle

  # Load BURC configuration
  local z_burc_file="${PWD}/.buk/burc.env"
  buv_file_exists "${z_burc_file}"
  source "${z_burc_file}" || buc_die "Failed to source BURC file"

  zvof_kindle
  zvvb_kindle
  zvob_kindle
}

buc_execute vob_ "VOK Build Operations" zvob_furnish "$@"

# eof
