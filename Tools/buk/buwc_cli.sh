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
# BUK Windows Commands - Command Line Interface
#
# Furnish: needs buhw tinder (Windows constants) and buc_require.
# No regime stack — commands consume constants directly.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

zbuwc_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_BUK_DIR}/buhw_windows.sh"       || buc_die "Failed to source buhw_windows.sh"
  source "${BURD_BUK_DIR}/buwc_windows.sh"       || buc_die "Failed to source buwc_windows.sh"

  zbuhw_kindle
  zbuwc_kindle
}

buc_execute buwc_ "Windows Commands" zbuwc_furnish "$@"

# eof
