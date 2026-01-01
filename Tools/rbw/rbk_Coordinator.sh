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
# Recipe Bottle Kludge - Routes commands to appropriate CLI scripts

set -euo pipefail

# Get script directory
RBK_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Verbose output if BUD_VERBOSE is set
rbk_show() {
  test "${BUD_VERBOSE:-0}" != "1" || echo "RBKSHOW: $*"
}

# Simple routing function
rbk_route() {
  local z_command="$1"
  shift
  local z_args="$*"

  rbk_show "Routing command: $z_command with args: $z_args"

  # Verify BDU environment variables are present
  if [ -z "${BUD_TEMP_DIR:-}" ]; then
    echo "ERROR: BUD_TEMP_DIR not set - must be called from BDU" >&2
    exit 1
  fi

  if [ -z "${BUD_NOW_STAMP:-}" ]; then
    echo "ERROR: BUD_NOW_STAMP not set - must be called from BDU" >&2
    exit 1
  fi

  rbk_show "BDU environment verified: TEMP_DIR=$BUD_TEMP_DIR, NOW_STAMP=$BUD_NOW_STAMP"

  # Route based on command prefix
  case "$z_command" in
    # Payor commands (high-privilege)
    rbw-PC)  exec "$RBK_SCRIPT_DIR/rbgp_cli.sh" rbgp_depot_create             $z_args ;;
    rbw-PI)  exec "$RBK_SCRIPT_DIR/rbgp_cli.sh" rbgp_payor_install            $z_args ;;
    rbw-PD)  exec "$RBK_SCRIPT_DIR/rbgp_cli.sh" rbgp_depot_destroy            $z_args ;;
    rbw-PE)  exec "$RBK_SCRIPT_DIR/rbgm_cli.sh" rbgm_payor_establish          $z_args ;;
    rbw-PG)  exec "$RBK_SCRIPT_DIR/rbgp_cli.sh" rbgp_governor_reset           $z_args ;;
    rbw-PR)  exec "$RBK_SCRIPT_DIR/rbgm_cli.sh" rbgm_payor_refresh            $z_args ;;

    # General depot operations
    rbw-ld)  exec "$RBK_SCRIPT_DIR/rbgp_cli.sh" rbgp_depot_list               $z_args ;;

    # Governor commands
    rbw-GR)  exec "$RBK_SCRIPT_DIR/rbgg_cli.sh" rbgg_create_retriever         $z_args ;;
    rbw-GD)  exec "$RBK_SCRIPT_DIR/rbgg_cli.sh" rbgg_create_director          $z_args ;;

    # Admin commands
    rbw-ps)  exec "$RBK_SCRIPT_DIR/rbgm_cli.sh" rbgm_payor_establish          $z_args ;;
    rbw-al)  exec "$RBK_SCRIPT_DIR/rbgg_cli.sh" rbgg_list_service_accounts     $z_args ;;
    rbw-aDS) exec "$RBK_SCRIPT_DIR/rbgg_cli.sh" rbgg_delete_service_account   $z_args ;;

    # Foundry commands
    rbw-fB)  exec "$RBK_SCRIPT_DIR/rbf_cli.sh" rbf_build  $z_args ;;
    rbw-fD)  exec "$RBK_SCRIPT_DIR/rbf_cli.sh" rbf_delete $z_args ;;
    rbw-il)  exec "$RBK_SCRIPT_DIR/rbf_cli.sh" rbf_list   $z_args ;;

    # Unknown command
    *)
      echo "ERROR: Unknown command: $z_command" >&2
      exit 1
      ;;
  esac
}

rbk_main() {
  local z_command="${1:-}"
  shift || true

  if [ -z "$z_command" ]; then
    echo "ERROR: No command specified" >&2
    exit 1
  fi

  rbk_route "$z_command" "$@"
}

rbk_main "$@"


# eof

