#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
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
# VOW Workbench - Routes VOK management commands via zipper registry
#
# All commands dispatch via buz_exec_lookup (see voz_zipper.sh for colophon
# mapping). vow-r is the lone exception — a raw vvr-binary passthrough routed
# ahead of the registry (see vow_route).

set -euo pipefail

# Get script directory
VOW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buv_validation.sh"
source "${BURD_BUK_DIR}/burd_regime.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"
source "${BURD_BUK_DIR}/buz_zipper.sh"
source "${VOW_SCRIPT_DIR}/voz_zipper.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

# Kindle dispatch and zipper registry
zbuv_kindle
zburd_kindle
zbuz_kindle
zvoz_kindle

# Verbose output if BURE_VERBOSE is set
vow_show() {
  test "${BURE_VERBOSE:-0}" != "1" || echo "VOWSHOW: $*"
}

# Routing
vow_route() {
  local z_command="$1"
  shift

  vow_show "Routing command: ${z_command} with args: $*"

  zburd_sentinel
  zvoz_healthcheck

  vow_show "BUD environment verified"

  # vow-r runs the freshly-built vvr binary directly with arbitrary args — a raw
  # dev-binary passthrough that does not fit the zipper's (module, command)
  # model, so it is dispatched ahead of the registry (cf. rbw's pre-lookup gate).
  case "${z_command}" in
    vow-r)  exec "${VOW_SCRIPT_DIR}/target/release/vvr" "$@" ;;
  esac

  buz_exec_lookup "${z_command}" "${VOW_SCRIPT_DIR}" "$@"
}

vow_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  vow_route "${z_command}" "$@"
}

vow_main "$@"

# eof
