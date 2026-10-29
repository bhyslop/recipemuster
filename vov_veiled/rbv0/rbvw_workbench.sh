#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# RBVW Workbench - routes hierophant commands to the CLI via the hierophant's
# own zipper (rbvz). VEILED: reached only through the withheld
# launcher.rbvw_workbench.sh, so the whole tool stays off every shipped manifest.

set -euo pipefail

RBVW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buz_zipper.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"
source "${RBVW_SCRIPT_DIR}/rbvz_zipper.sh"

buc_context "${0##*/}"

# The workbench's head: a name of the kit standing exported is refused save
# the hand-offs every level declares.
buv_export_sentinel "${BURD_TARGET}"

zbuz_kindle
zrbvz_kindle

######################################################################
# Routing

rbvw_route() {
  local z_command="$1"
  shift

  zburd_sentinel
  zrbvz_healthcheck

  buz_exec_lookup "${z_command}" "${RBVW_SCRIPT_DIR}" "$@"
}

rbvw_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die_now "No command specified"

  rbvw_route "${z_command}" "$@"
}

rbvw_main "$@"

# eof
