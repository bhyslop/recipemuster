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
# RBTW Workbench - Routes theurge Rust build and test commands
#
# Fully orthogonal from VOW (vvk/jjk/cmk kit pipeline).
# Theurge is Recipe Bottle's own test infrastructure — permanently part of rbk.

set -euo pipefail

RBTW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buv_validation.sh"
source "${BURD_BUK_DIR}/burd_regime.sh"

buc_context "${0##*/}"

zbuv_kindle
zburd_kindle

readonly RBTW_MANIFEST="${RBTW_SCRIPT_DIR}/Cargo.toml"

rbtw_route() {
  local z_command="$1"
  shift

  zburd_sentinel

  case "${z_command}" in

    rbtd-b)
      buc_step "Building theurge"
      buc_log_args "Manifest: ${RBTW_MANIFEST}"
      cargo build --manifest-path "${RBTW_MANIFEST}" "$@" || buc_die "cargo build failed"
      buc_success "Theurge built"
      ;;

    rbtd-t)
      buc_step "Testing theurge"
      buc_log_args "Manifest: ${RBTW_MANIFEST}"
      cargo test --manifest-path "${RBTW_MANIFEST}" "$@" || buc_die "cargo test failed"
      buc_success "All theurge tests passed"
      ;;

    rbtd-r)
      local z_fixture="${BURD_TOKEN_3:-}"
      test -n "${z_fixture}" || buc_die "No fixture imprint — use tabtarget with imprint (e.g. rbtd-r.Run.tadmor.sh)"

      buc_step "Building theurge"
      cargo build --manifest-path "${RBTW_MANIFEST}" || buc_die "cargo build failed"

      local z_binary="${RBTW_SCRIPT_DIR}/target/debug/rbtd"
      test -x "${z_binary}" || buc_die "Theurge binary not found: ${z_binary}"

      local z_manifest
      case "${z_fixture}" in
        tadmor|srjcl|pluml)
          z_manifest="rbw-cC rbw-cQ rbw-cw rbw-cf rbw-cb"
          ;;
        four-mode)
          z_manifest="rbw-DO rbw-DA rbw-Rw rbw-Dt rbw-ak"
          ;;
        access-probe)
          z_manifest="rbtd-ap"
          ;;
        *)
          buc_die "Unknown fixture: ${z_fixture}"
          ;;
      esac

      buc_step "Running theurge fixture '${z_fixture}'"
      "${z_binary}" "${z_manifest}" "${z_fixture}"
      ;;

    rbtd-ap)
      local z_role="${BURD_TOKEN_3:-}"
      test -n "${z_role}" || buc_die "No role imprint — use tabtarget with imprint (e.g. rbtd-ap.AccessProbe.governor.sh)"

      zburd_sentinel

      # Source RBK modules (regime + constants + probe chain)
      local z_rbk="${RBTW_SCRIPT_DIR}/.."
      source "${z_rbk}/rbrr_regime.sh"
      source "${z_rbk}/rbrp_regime.sh"
      source "${z_rbk}/rbcc_Constants.sh"
      source "${z_rbk}/rbgc_Constants.sh"
      source "${z_rbk}/rbdc_DerivedConstants.sh"
      source "${z_rbk}/rbgo_OAuth.sh"
      source "${z_rbk}/rbgu_Utility.sh"
      source "${z_rbk}/rbgi_IAM.sh"
      source "${z_rbk}/rbgp_Payor.sh"
      source "${z_rbk}/rbap_AccessProbe.sh"

      # Load regime env and kindle module chain
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
      zrbap_kindle

      local z_iterations=5
      local z_delay_ms=1500

      case "${z_role}" in
        governor|director|retriever)
          buc_step "JWT SA access probe: ${z_role}"
          rbap_jwt_sa_probe "${z_role}" "${z_iterations}" "${z_delay_ms}"
          buc_success "${z_role} JWT access probe passed"
          ;;
        payor)
          buc_step "Payor OAuth access probe"
          source "${RBBC_rbrp_file}" || buc_die "Failed to source RBRP: ${RBBC_rbrp_file}"
          zrbrp_kindle
          zrbrp_enforce
          rbap_payor_oauth_probe "${z_iterations}" "${z_delay_ms}"
          buc_success "Payor OAuth access probe passed"
          ;;
        *)
          buc_die "Unknown access-probe role: ${z_role} (expected governor|director|retriever|payor)"
          ;;
      esac
      ;;

    *)
      buc_die "Unknown command: ${z_command}"
      ;;
  esac
}

rbtw_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  rbtw_route "${z_command}" "$@"
}

rbtw_main "$@"

# eof
