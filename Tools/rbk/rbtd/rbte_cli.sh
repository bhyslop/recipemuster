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
# RBTE CLI - Theurge test engine CLI entry point with differential furnish
#
# Sourced by rbtw_workbench.sh. Provides rbte_dispatch() for command routing.
# Light furnish: buv + burd + buz + rbz + rbte (all commands)
# Heavy furnish: regime + constants + OAuth + IAM + Payor (probe only)

set -euo pipefail

ZRBTE_CLI_DIR="${BASH_SOURCE[0]%/*}"

######################################################################
# Light furnish — source and kindle for all commands

source "${BURD_BUK_DIR}/buv_validation.sh"
source "${BURD_BUK_DIR}/burd_regime.sh"
source "${BURD_BUK_DIR}/buz_zipper.sh"
source "${ZRBTE_CLI_DIR}/../rbz_zipper.sh"
source "${ZRBTE_CLI_DIR}/rbte_engine.sh"

zbuv_kindle
zburd_kindle
zbuz_kindle
zrbz_kindle
zrbte_kindle

######################################################################
# Heavy furnish — regime + full module chain for access probe

zrbte_furnish_probe() {
  zburd_sentinel

  local z_rbk="${ZRBTE_CLI_DIR}/.."
  source "${z_rbk}/rbrr_regime.sh"
  source "${z_rbk}/rbrp_regime.sh"
  source "${z_rbk}/rbcc_Constants.sh"
  source "${z_rbk}/rbgc_Constants.sh"
  source "${z_rbk}/rbdc_DerivedConstants.sh"
  source "${z_rbk}/rbgo_OAuth.sh"
  source "${z_rbk}/rbgu_Utility.sh"
  source "${z_rbk}/rbgi_IAM.sh"
  source "${z_rbk}/rbgp_Payor.sh"
  source "${z_rbk}/rbgv_AccessProbe.sh"

  source "${RBBC_rbrr_file}" || buc_die "Failed to source ${RBBC_rbrr_file}"
  zrbrr_kindle
  zrbrr_enforce
  zrbcc_kindle
  zrbdc_kindle
  zrbgc_kindle
  zrbgo_kindle
  zrbgu_kindle
  zrbgi_kindle
  zrbgp_kindle
  zrbgv_kindle
}

######################################################################
# Dispatch

rbte_dispatch() {
  local z_command="$1"
  shift

  zburd_sentinel
  zrbte_sentinel

  case "${z_command}" in

    rbtd-b)
      rbte_build "$@"
      ;;

    rbtd-t)
      rbte_test "$@"
      ;;

    rbtd-r)
      rbte_run "$@"
      ;;

    rbtd-s)
      local z_frontispiece="${BURD_TOKEN_2:-}"
      case "${z_frontispiece}" in
        TestSuite)   rbte_suite "$@" ;;
        SingleCase)  rbte_single "$@" ;;
        *)           buc_die "Unknown frontispiece for rbtd-s: ${z_frontispiece} (expected TestSuite|SingleCase)" ;;
      esac
      ;;

    rbtd-ap)
      zrbte_furnish_probe
      rbte_probe "$@"
      ;;

    *)
      buc_die "Unknown command: ${z_command}"
      ;;

  esac
}

# eof
