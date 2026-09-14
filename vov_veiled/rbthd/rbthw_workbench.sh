#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
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

  test -n "${z_command}" || buc_die_now "No command specified"

  rbthw_route "${z_command}" "$@"
}

rbthw_main "$@"

# eof
