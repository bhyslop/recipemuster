#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# VOME CLI - Matricula (vom) build/test/run entry point
#
# Enrolled in voz_zipper, dispatched by vow_workbench via buz_exec_lookup, which
# execs this script fresh (so it self-sources its deps). Public functions
# (vome_build/test/run) live in vome_engine.sh.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"

zvome_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_done || return 0

  local z_cli_dir="${BASH_SOURCE[0]%/*}"
  source "${z_cli_dir}/vome_engine.sh"

  zvome_kindle
}

buc_execute vome_ "Matricula build/test/run engine" zvome_furnish "$@"

# eof
