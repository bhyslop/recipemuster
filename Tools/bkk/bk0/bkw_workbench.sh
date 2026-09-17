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
# BKW Workbench - Routes Bash Kennel Master commands via the zipper registry
#
# Every command dispatches through buz_exec_lookup; see bkz_zipper.sh for the
# colophon mapping. There is no pre-registry passthrough arm here and there is
# not meant to be one: the kennel's own binary is reached through the whistle,
# which is itself an enrolled command, so the raw-binary escape other workbenches
# carry would be a second road to the door the whistle exists to be.

set -euo pipefail

BKW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buv_validation.sh"
source "${BURD_BUK_DIR}/burd_regime.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"
source "${BURD_BUK_DIR}/buz_zipper.sh"
source "${BKW_SCRIPT_DIR}/bkz_zipper.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

# Kindle dispatch and zipper registry
zbuv_kindle
zburd_kindle
zbuz_kindle
zbkz_kindle

bkw_route() {
  local z_command="$1"
  shift

  zburd_sentinel
  zbkz_healthcheck

  buz_exec_lookup "${z_command}" "${BKW_SCRIPT_DIR}" "$@"
}

bkw_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die_now "No command specified"

  bkw_route "${z_command}" "$@"
}

bkw_main "$@"

# eof
