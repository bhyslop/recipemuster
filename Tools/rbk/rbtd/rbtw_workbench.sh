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
      local z_nameplate="${BURD_TOKEN_3:-}"
      test -n "${z_nameplate}" || buc_die "No nameplate imprint — use tabtarget with imprint (e.g. rbtd-r.Run.tadmor.sh)"

      buc_step "Building theurge"
      cargo build --manifest-path "${RBTW_MANIFEST}" || buc_die "cargo build failed"

      local z_binary="${RBTW_SCRIPT_DIR}/target/debug/rbtd"
      test -x "${z_binary}" || buc_die "Theurge binary not found: ${z_binary}"

      local z_manifest="rbw-cC rbw-cQ rbw-cw rbw-cf rbw-cb"

      buc_step "Running theurge against nameplate '${z_nameplate}'"
      "${z_binary}" "${z_manifest}" "${z_nameplate}"
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
