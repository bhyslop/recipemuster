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
# Recipe Bottle Payor Onboarding - Minimal CLI (pre-kindle)
#
# This CLI module deliberately skips regime kindle/enforce so that the
# onboarding guide can run before a valid regime exists.  State probes
# in the guide function read raw files directly.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

zrbgm_onboard_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_CONFIG_DIR       " "Configuration directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_CONFIG_DIR}/rbbc_constants.sh"  || buc_die "Failed to source rbbc_constants.sh"
  source "${BURD_BUK_DIR}/bug_guide.sh"           || buc_die "Failed to source bug_guide.sh"

  # Kindle zipper for tabtarget colophon references (lightweight, no regime dependency)
  source "${BURD_BUK_DIR}/buz_zipper.sh"                             || buc_die "Failed to source buz_zipper.sh"
  source "${BURD_TOOLS_DIR}/${RBBC_kit_subdir}/rbz_zipper.sh"        || buc_die "Failed to source rbz_zipper.sh"
  zbuz_kindle
  zrbz_kindle

  source "${BURD_TOOLS_DIR}/${RBBC_kit_subdir}/rbgm_ManualProcedures.sh" || buc_die "Failed to source rbgm_ManualProcedures.sh"
}

buc_execute rbgm_ "Payor Onboarding" zrbgm_onboard_furnish "$@"

# eof
