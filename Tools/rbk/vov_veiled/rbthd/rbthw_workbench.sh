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
# RBTHW Workbench - routes hierophant commands to the CLI via the hierophant's
# own zipper (rbthz). VEILED: reached only through the withheld
# launcher.rbthw_workbench.sh, so the whole tool stays off every shipped manifest.

set -euo pipefail

RBTHW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buv_validation.sh"
source "${BURD_BUK_DIR}/burd_regime.sh"
source "${BURD_BUK_DIR}/buz_zipper.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"
source "${RBTHW_SCRIPT_DIR}/rbthz_zipper.sh"

buc_context "${0##*/}"

zbuv_kindle
zburd_kindle
zbuz_kindle
zrbthz_kindle

######################################################################
# Routing

rbthw_route() {
  local z_command="$1"
  shift

  zburd_sentinel
  zrbthz_healthcheck

  buz_exec_lookup "${z_command}" "${RBTHW_SCRIPT_DIR}" "$@"
}

rbthw_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  rbthw_route "${z_command}" "$@"
}

rbthw_main "$@"

# eof
