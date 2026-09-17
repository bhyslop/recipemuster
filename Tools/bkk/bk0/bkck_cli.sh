#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
# BKCK CLI - dispatch entry point for the kennelman
#
# Enrolled in bkz_zipper and dispatched by bkw_workbench through
# buz_exec_lookup, which execs this script fresh - so it self-sources its
# dependencies. The engine's own functions live in bkck_kennelman.sh.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"

# The gateway (the estate's bash guide, "CLI as Module Gateway"): every dependency is sourced and
# kindled in the furnish itself, which is the one place outside a module's own
# CLI that may reach its internals.
zbkck_furnish() {
  buc_doc_env_row "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_row "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env_row "BURD_TEMP_DIR         " "Temporary directory the lures stand under (dispatch-provided)"
  buc_doc_env_row "BURD_TACKROOM         " "The station's shared toolchain and registry store (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  # The pin, shared with the whistle. The kennelman builds the crate too, and
  # build.rs owes both of them the door law and the position - refused there once
  # rather than in each door's bash.
  source "${BURD_TOOLS_DIR}/bkk/bk0/bkcp_position.sh"
  source "${BURD_TOOLS_DIR}/bkk/bk0/bkcb_tackroom.sh"

  source "${BURD_TOOLS_DIR}/bkk/bk0/bkck_kennelman.sh"

  zbuv_kindle
  zburd_kindle
  zbkcb_kindle
  zbkck_kindle
}


buc_execute bkck_ "The kennelman: the kennel's own suite, run outside the kennel" zbkck_furnish "$@"

# eof
