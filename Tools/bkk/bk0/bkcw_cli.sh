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
# BKCW CLI - dispatch entry point for the whistle
#
# Enrolled in bkz_zipper and dispatched by bkw_workbench through
# buz_exec_lookup, which execs this script fresh - so it self-sources its
# dependencies. The engine's own functions live in bkcw_whistle.sh.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"

# The gateway: a module's internals are reached by its own CLI, or by another
# CLI's FURNISH, and by nothing further in. The registry execs this script fresh,
# so the workbench's kindling is gone by the time we run and every dependency
# below is ours to raise - raised HERE rather than a level down, where the
# gateway would no longer reach.
zbkcw_furnish() {
  buc_doc_env_row "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_row "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env_row "BURD_TEMP_DIR         " "Temporary directory for intermediate files (dispatch-provided)"
  buc_doc_env_row "BURD_TACKROOM         " "The station's shared toolchain and registry store (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  # The pin, the one reading this kit's two bash doors still share: the door law
  # and the position moved into build.rs, which is where cargo's own choices are
  # observable and where a build that cannot state a position dies.
  source "${BURD_TOOLS_DIR}/bkk/bk0/bkcp_position.sh"
  # The fence, stated under this kit's own head. Nothing here reaches outside the
  # kit and the substrate, because no parcel carries the tree this line used to
  # name and the bootstrap died at it on every receiving station.
  source "${BURD_TOOLS_DIR}/bkk/bk0/bkcb_tackroom.sh"

  source "${BURD_TOOLS_DIR}/bkk/bk0/bkcw_whistle.sh"

  zbuv_kindle
  zburd_kindle
  zbkcb_kindle
  zbkcw_kindle
}


buc_execute bkcw_ "The Bash Kennel Master's whistle" zbkcw_furnish "$@"

# eof
