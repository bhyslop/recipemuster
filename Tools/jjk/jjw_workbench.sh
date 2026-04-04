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

    # Remote dispatch integration tests (JJSTF profiles)
    jjw-tR)
      local z_profile="${BURD_TOKEN_3}"
      test -n "${z_profile}" || buc_die "jjw-tR: no profile in imprint (BURD_TOKEN_3)"

      # Resolve profile configuration and run preflight probe
      case "${z_profile}" in
        hairpin)
          local z_user="rbtest"
          local z_host="localhost"
          local z_reldir="projects/rbm_alpha_recipemuster"
          if ! ssh -o ConnectTimeout=5 -o BatchMode=yes "${z_user}@${z_host}" exit 0 2>/dev/null; then
            echo "SKIP: ${z_profile}: SSH to ${z_user}@${z_host} failed"; exit 0
          fi
          if ! ssh -o ConnectTimeout=5 -o BatchMode=yes "${z_user}@${z_host}" "test -d \$HOME/${z_reldir}" 2>/dev/null; then
            echo "SKIP: ${z_profile}: RELDIR ${z_reldir} not found on ${z_host}"; exit 0
          fi
          if ! ssh -o ConnectTimeout=5 -o BatchMode=yes "${z_user}@${z_host}" "test -f \$HOME/${z_reldir}/.buk/burc.env" 2>/dev/null; then
            echo "SKIP: ${z_profile}: BUK not installed at ${z_reldir} on ${z_host}"; exit 0
          fi
          echo "Preflight PASS: ${z_profile} (${z_user}@${z_host}:~/${z_reldir})"
          ;;
        cerebro)
          local z_host="${JJTEST_CEREBRO_HOST:-}"
          test -n "${z_host}" || { echo "SKIP: ${z_profile}: JJTEST_CEREBRO_HOST not set"; exit 0; }
          local z_user="rbtest"
          local z_reldir="projects/rbm_alpha_recipemuster"
          if ! ssh -o ConnectTimeout=5 -o BatchMode=yes "${z_user}@${z_host}" exit 0 2>/dev/null; then
            echo "SKIP: ${z_profile}: SSH to ${z_user}@${z_host} failed"; exit 0
          fi
          if ! ssh -o ConnectTimeout=5 -o BatchMode=yes "${z_user}@${z_host}" "test -d \$HOME/${z_reldir}" 2>/dev/null; then
            echo "SKIP: ${z_profile}: RELDIR ${z_reldir} not found on ${z_host}"; exit 0
          fi
          if ! ssh -o ConnectTimeout=5 -o BatchMode=yes "${z_user}@${z_host}" "test -f \$HOME/${z_reldir}/.buk/burc.env" 2>/dev/null; then
            echo "SKIP: ${z_profile}: BUK not installed at ${z_reldir} on ${z_host}"; exit 0
          fi
          echo "Preflight PASS: ${z_profile} (${z_user}@${z_host}:~/${z_reldir})"
          ;;
        nokey)
          local z_user="${JJTEST_NOKEY_USER:-nobody}"
          if ! id "${z_user}" >/dev/null 2>&1; then
            echo "SKIP: ${z_profile}: user '${z_user}' does not exist"; exit 0
          fi
          echo "Preflight PASS: ${z_profile} (user ${z_user} exists, expecting SSH auth failure)"
          ;;
        norepo)
          if ! ssh -o ConnectTimeout=5 -o BatchMode=yes "rbtest@localhost" exit 0 2>/dev/null; then
            echo "SKIP: ${z_profile}: SSH to rbtest@localhost failed"; exit 0
          fi
          echo "Preflight PASS: ${z_profile} (rbtest@localhost with nonexistent reldir)"
          ;;
        nogit)
          local z_user="${JJTEST_NOGIT_USER:-rbtest_nogit}"
          local z_reldir="${JJTEST_NOGIT_RELDIR:-projects/rbm_alpha_nogit}"
          if ! ssh -o ConnectTimeout=5 -o BatchMode=yes "${z_user}@localhost" exit 0 2>/dev/null; then
            echo "SKIP: ${z_profile}: SSH to ${z_user}@localhost failed"; exit 0
          fi
          if ! ssh -o ConnectTimeout=5 -o BatchMode=yes "${z_user}@localhost" "test -d \$HOME/${z_reldir}" 2>/dev/null; then
            echo "SKIP: ${z_profile}: RELDIR ${z_reldir} not found"; exit 0
          fi
          if ! ssh -o ConnectTimeout=5 -o BatchMode=yes "${z_user}@localhost" "test -f \$HOME/${z_reldir}/.buk/burc.env" 2>/dev/null; then
            echo "SKIP: ${z_profile}: BUK not installed at ${z_reldir}"; exit 0
          fi
          echo "Preflight PASS: ${z_profile} (${z_user}@localhost:~/${z_reldir})"
          ;;
        *)
          buc_die "jjw-tR: unknown profile: ${z_profile}"
          ;;
      esac

      # Run integration tests for this profile
      exec cargo test \
        --manifest-path "${JJW_SCRIPT_DIR}/vov_veiled/Cargo.toml" \
        --test remote_dispatch \
        -- --test-threads=1 --ignored "${z_profile}::"
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
