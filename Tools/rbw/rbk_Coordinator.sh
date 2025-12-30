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
    # Image management commands (rbf)
    rbw-l)
      rbk_show "Routing to rbf_cli.sh rbf_list"
      exec "$RBK_SCRIPT_DIR/rbf_cli.sh" rbf_list $z_args
      ;;
    rbw-II)
      rbk_show "Routing to rbf_cli.sh rbf_image_info"
      exec "$RBK_SCRIPT_DIR/rbf_cli.sh" rbf_image_info $z_args
      ;;
    rbw-r)
      rbk_show "Routing to rbf_cli.sh rbf_retrieve"
      exec "$RBK_SCRIPT_DIR/rbf_cli.sh" rbf_retrieve $z_args
      ;;

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
    rbw-GR)  exec "$RBK_SCRIPT_DIR/rbgg_cli.sh" rbgg_retriever_create         $z_args ;;
    rbw-GD)  exec "$RBK_SCRIPT_DIR/rbgg_cli.sh" rbgg_director_create          $z_args ;;

    # Google admin commands (legacy)
    rbw-ps)  exec "$RBK_SCRIPT_DIR/rbgm_cli.sh" rbgm_payor_establish          $z_args ;;
    rbw-aIA) exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_initialize_admin         $z_args ;;
    rbw-aID) exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_destroy_admin            $z_args ;;
    rbw-al)  exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_list_service_accounts    $z_args ;;
    rbw-aCR) exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_create_retriever         $z_args ;;
    rbw-aCD) exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_create_director          $z_args ;;
    rbw-aDS) exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_delete_service_account   $z_args ;;
    rbw-aPO) exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_destroy_project          $z_args ;;
    rbw-aPr) exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_restore_project          $z_args ;;

    # Foundry commands
    rbw-fB)  exec "$RBK_SCRIPT_DIR/rbf_cli.sh" rbf_build  $z_args ;;
    rbw-fD)  exec "$RBK_SCRIPT_DIR/rbf_cli.sh" rbf_delete $z_args ;;
    rbw-fS)  exec "$RBK_SCRIPT_DIR/rbf_cli.sh" rbf_study  $z_args ;;

    # Help/documentation commands
    rbw-him)
      rbk_show  "Routing to rbf_cli.sh (help)"
      exec "$RBK_SCRIPT_DIR/rbf_cli.sh" $z_args
      ;;
    rbw-hga)
      rbk_show  "Routing to rbga_cli.sh (help)"
      exec "$RBK_SCRIPT_DIR/rbga_cli.sh" $z_args
      ;;

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

