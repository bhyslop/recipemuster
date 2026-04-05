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
# JJW Workbench - Routes Job Jockey commands

set -euo pipefail

# Get script directory
JJW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buv_validation.sh"
source "${BURD_BUK_DIR}/burd_regime.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

# Verify dispatch completed
zbuv_kindle
zburd_kindle

# Verbose output if BURE_VERBOSE is set
jjw_show() {
  test "${BURE_VERBOSE:-0}" != "1" || echo "JJWSHOW: $*"
}

# Simple routing function
jjw_route() {
  local z_command="$1"
  shift

  jjw_show "Routing command: ${z_command} with args: $*"

  zburd_sentinel

  jjw_show "BDU environment verified"

  # Route based on command - only arcanum commands remain
  # All jjw-* commands deprecated: use /jjc-* slash commands instead
  case "${z_command}" in

    # Arcanum commands (install/check/uninstall)
    jja-c|jja-i|jja-u)
      jjw_show "Delegating to arcanum: ${z_command}"
      exec "${JJW_SCRIPT_DIR}/jja_arcanum.sh" "${z_command}" "$@"
      ;;

    # Fundus scenario suite — all profiles against a host (JJSTF)
    jjw-tfs)
      local z_host="${BURD_TOKEN_3}"
      test -n "${z_host}" || buc_die "jjw-tfs: no host in imprint (BURD_TOKEN_3)"
      export JJTEST_HOST="${z_host}"
      exec cargo test \
        --manifest-path "${JJW_SCRIPT_DIR}/vov_veiled/Cargo.toml" \
        --test fundus_scenario \
        -- --test-threads=1 --ignored
      ;;

    # Fundus scenario single test — one function against a host
    jjw-tfS)
      local z_host="${BURD_TOKEN_3}"
      test -n "${z_host}" || buc_die "jjw-tfS: no host in imprint (BURD_TOKEN_3)"
      local z_test="${1:-}"
      test -n "${z_test}" || buc_die "jjw-tfS: no test function name (pass as argument)"
      export JJTEST_HOST="${z_host}"
      exec cargo test \
        --manifest-path "${JJW_SCRIPT_DIR}/vov_veiled/Cargo.toml" \
        --test fundus_scenario \
        -- --test-threads=1 --ignored "${z_test}"
      ;;

    *)
      buc_die "Unknown command: ${z_command} (jjw-* commands removed; use /jjc-* slash commands)"
      ;;
  esac
}

# Main entry point
jjw_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  jjw_route "${z_command}" "$@"
}

jjw_main "$@"

# eof
