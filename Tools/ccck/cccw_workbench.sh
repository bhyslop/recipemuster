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

set -euo pipefail

zccck_kit_dir="${BASH_SOURCE[0]%/*}"
zccck_buk_directory="${zccck_kit_dir}/../buk"
source "${zccck_buk_directory}/bcu_BashCommandUtility.sh"

bcu_context "cccw_workbench"


zccck_docker_compose() {
  docker-compose --env-file "${zccck_kit_dir}/../cccr.env" -f "${zccck_kit_dir}/docker-compose.yml" "$@"
}

# Connect to CCBX container with remote command
zccck_connect() {
  local z_remote_command="$1"

  test -n "${z_remote_command}" || bcu_die "Remote command required for zccck_connect"

  bcu_step "Connecting to CCCK container with command: ${z_remote_command}"

  # Connect via SSH with configured port and execute remote command
  ssh -p "${CCCR_SSH_PORT}" -t claude@localhost "${z_remote_command}"
}

# Simple routing function
zccck_route() {
  local z_command="$1"
  shift
  local z_args="$*"

  bcu_step "Routing command: $z_command with args: $z_args"

  # Verify BDU environment variables are present
  test -n "${BDU_TEMP_DIR:-}" || bcu_die "BDU_TEMP_DIR not set - must be called from BDU"
  test -n "${BDU_NOW_STAMP:-}" || bcu_die "BDU_NOW_STAMP not set - must be called from BDU"

  source "${zccck_kit_dir}/../cccr.env"

  test -n "${CCCR_SSH_PORT:-}" || bcu_die "CCCR_SSH_PORT not set in cccr.env"
  test -n "${CCCR_CLAUDE_CONFIG_DIR:-}" || bcu_die "CCCR_CLAUDE_CONFIG_DIR not set in cccr.env"

  bcu_step "BDU environment verified: TEMP_DIR=$BDU_TEMP_DIR, NOW_STAMP=$BDU_NOW_STAMP, CCCR_SSH_PORT=${CCCR_SSH_PORT} CCCR_WEB_PORT=${CCCR_WEB_PORT} CCCR_CLAUDE_CONFIG_DIR=${CCCR_CLAUDE_CONFIG_DIR}"

  # Route based on command prefix
  case "$z_command" in

    # Claude Code Container Kit (ccck) Docker commands
    ccck-a)
      bcu_step "Creating Claude config directory if needed"
      mkdir -p "${zccck_kit_dir}/${CCCR_CLAUDE_CONFIG_DIR}"
      test -d "${zccck_kit_dir}/${CCCR_CLAUDE_CONFIG_DIR}" || bcu_die "Failed to create Claude config directory: ${zccck_kit_dir}/${CCCR_CLAUDE_CONFIG_DIR}"
      
      zccck_docker_compose up -d

      bcu_step "Setting up git configuration in container"

      bcu_step "Setting git safe directories"
      zccck_connect "git config --global --add safe.directory /workspace/brm_recipemuster"
      zccck_connect "git config --global --add safe.directory /workspace/cnmp_CellNodeMessagePrototype"
      zccck_connect "git config --global --add safe.directory /workspace/recipebottle-admin"

      bcu_step "Setting git global configuration"
      zccck_connect "git config --global user.email 'bhyslop@scaleinvariant.org'"
      zccck_connect "git config --global user.name  'Claude Code with Brad Hyslop'"

      bcu_step "Container started and configured"
      ;;
    ccck-z)  zccck_docker_compose down                                                        ;;
    ccck-B)  zccck_docker_compose build --no-cache                                            ;;
    ccck-c)  zccck_connect "cd /workspace/brm_recipemuster  &&  claude-code"                ;;
    ccck-s)  zccck_connect "cd /workspace/brm_recipemuster  &&  bash"                         ;;
    ccck-g)  zccck_connect "cd /workspace/brm_recipemuster  &&  git status"                   ;;
    ccck-R)
      # Full reset: clean Docker resources and SSH keys
      bcu_step "Stopping and removing container"
      docker stop ClaudeCodeBox 2>/dev/null || true
      docker rm   ClaudeCodeBox 2>/dev/null || true

      bcu_step "Removing Docker network"
      docker network rm claude-network 2>/dev/null || true

      bcu_step "Removing SSH host key for localhost:${CCCR_SSH_PORT}"
      ssh-keygen -R "[localhost]:${CCCR_SSH_PORT}" 2>/dev/null || true

      bcu_step "Full reset complete - ready for fresh start"
      ;;

    # Unknown command
    *)
      bcu_die "Unknown command: $z_command"
      ;;
  esac
}

zccck_main() {
  local z_command="${1:-}"
  shift || true

  test -n "$z_command" || bcu_die "No command specified"

  zccck_route "$z_command" "$@"
}

zccck_main "$@"


# eof

