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
# BUK Windows Handbook - Command Line Interface
#
# Thin furnish: handbook display procedures need only buh_* combinators
# and the buhw module tinder constants. No regime, no validation stack.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

zbuhw_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_BUK_DIR}/buym_yelp.sh"          || buc_die "Failed to source buym_yelp.sh"
  source "${BURD_BUK_DIR}/buh_handbook.sh"       || buc_die "Failed to source buh_handbook.sh"
  source "${BURD_BUK_DIR}/buhw_windows.sh"       || buc_die "Failed to source buhw_windows.sh"

  zbuhw_kindle
}

buc_execute buhw_ "Windows Handbook" zbuhw_furnish "$@"

# eof
