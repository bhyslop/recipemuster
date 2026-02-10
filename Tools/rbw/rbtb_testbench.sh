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
# RBTB Testbench - Recipe Bottle test framework testbench

set -euo pipefail

RBTB_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${RBTB_SCRIPT_DIR}/../buk/buc_command.sh"
source "${RBTB_SCRIPT_DIR}/../buk/buto_operations.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butr_registry.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butd_dispatch.sh"
source "${RBTB_SCRIPT_DIR}/../buk/buz_zipper.sh"
source "${RBTB_SCRIPT_DIR}/rbz_zipper.sh"

# Source test case files
source "${RBTB_SCRIPT_DIR}/rbtckk_KickTires.sh"
source "${RBTB_SCRIPT_DIR}/rbtcal_ArkLifecycle.sh"

buc_context "${0##*/}"

######################################################################
# Setup functions

zrbtb_setup_kick() {
  buto_trace "Setup for kick-tires suite (no-op)"
}

zrbtb_setup_ark() {
  buto_trace "Setup for ark-lifecycle suite"
  zbuz_kindle
  zrbz_kindle
  buto_init_dispatch
  buto_init_evidence
  ZRBTB_ARK_VESSEL_SIGIL="trbim-macos"
}

######################################################################
# Registration

rbtb_kindle() {
  butr_kindle
  butr_register "kick-tires" "rbtckk_" "zrbtb_setup_kick" "fast"
  butr_register "ark-lifecycle" "rbtcal_" "zrbtb_setup_ark" "slow"
}

######################################################################
# Routing

rbtb_route() {
  local z_command="${1:-}"
  shift || true

  test -n "${BURD_TEMP_DIR:-}"   || buc_die "BURD_TEMP_DIR not set - must be called from BURD"
  test -n "${BURD_NOW_STAMP:-}"  || buc_die "BURD_NOW_STAMP not set - must be called from BURD"

  rbtb_kindle

  export ZBUTO_ROOT_TEMP_DIR="${BURD_TEMP_DIR}"
  export BUT_VERBOSE="${BUT_VERBOSE:-0}"

  case "${z_command}" in
    rbtb-ta) butd_run_all ;;
    rbtb-ts) butd_run_suite "${1:-}" ;;
    rbtb-to) butd_run_one "${1:-}" ;;
    *)       buc_die "Unknown command: ${z_command}" ;;
  esac
}

rbtb_main() {
  local z_command="${1:-}"
  shift || true
  test -n "${z_command}" || buc_die "No command specified"
  rbtb_route "${z_command}" "$@"
}

rbtb_main "$@"

# eof
