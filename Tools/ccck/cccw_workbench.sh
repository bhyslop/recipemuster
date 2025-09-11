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

echo "BRADTRACE: Entering workbench..."

# Get script directory
z_script_dir="${BASH_SOURCE[0]%/*}"

zcccw_show() { echo "CCCWSHOW: $*"; }

# Connect to CCBX container with optional remote command
zccck_connect() {
  local z_remote_command="${1:-}"
  
  zcccw_show "Connecting to CCCK container with command: ${z_remote_command:-default}"
  zcccw_show "z_script_dir=${z_script_dir}"
  zcccw_show "PWD=${PWD}"
  
  source "${z_script_dir}/../cccr.env"
  
  # Validate required environment variables
  if [ -z "${CCCR_SSH_PORT:-}" ]; then
    echo "ERROR: CCCR_SSH_PORT not set in cccr.env" >&2
    exit 1
  fi
  
  # Default command if none provided
  if [ -z "${z_remote_command}" ]; then
    z_remote_command="cd /workspace/brm_recipemuster  &&  claude-code"
  fi
  
  # Connect via SSH with configured port and execute remote command
  ssh -p "${CCCR_SSH_PORT}" -t claude@localhost "${z_remote_command}"
}

# Simple routing function
zccck_route() {
  local z_command="$1"
  shift
  local z_args="$*"

  zcccw_show "Routing command: $z_command with args: $z_args"

  # Verify BDU environment variables are present
  if [ -z "${BDU_TEMP_DIR:-}" ]; then
    echo "ERROR: BDU_TEMP_DIR not set - must be called from BDU" >&2
    exit 1
  fi

  if [ -z "${BDU_NOW_STAMP:-}" ]; then
    echo "ERROR: BDU_NOW_STAMP not set - must be called from BDU" >&2
    exit 1
  fi

  zcccw_show "BDU environment verified: TEMP_DIR=$BDU_TEMP_DIR, NOW_STAMP=$BDU_NOW_STAMP"

  # Route based on command prefix
  case "$z_command" in

    # Claude Code Container Kit (ccck) Docker commands
    ccck-a)  
      cd "${z_script_dir}" && docker-compose up -d
      
      zcccw_show "Setting up git configuration in container"
      
      zcccw_show "Setting git safe directories"
      zccck_connect "git config --global --add safe.directory /workspace/brm_recipemuster"
      zccck_connect "git config --global --add safe.directory /workspace/cnmp_CellNodeMessagePrototype"
      zccck_connect "git config --global --add safe.directory /workspace/recipebottle-admin"
      
      zcccw_show "Setting git global configuration"
      zccck_connect "git config --global user.email 'bhyslop@scaleinvariant.org'"
      zccck_connect "git config --global user.name  'Claude Code with Brad Hyslop'"
      
      zcccw_show "Container started and configured"
      ;;
    ccck-z)  cd "${z_script_dir}" && docker-compose down                                      ;;
    ccck-B)  cd "${z_script_dir}" && docker-compose build --no-cache                          ;;
    ccck-c)  zccck_connect                                                                    ;;
    ccck-s)  zccck_connect "cd /workspace/brm_recipemuster  &&  bash"                         ;;
    ccck-g)  zccck_connect "cd /workspace/brm_recipemuster  &&  git status"                   ;;
    ccck-R)
      # Reset SSH connection: remove SSH host keys for clean reconnection
      zcccw_show "Removing SSH host key for localhost:${CCCR_SSH_PORT}"
      ssh-keygen -R "[localhost]:${CCCR_SSH_PORT}" 2>/dev/null || true
      
      zcccw_show "SSH reset complete - try connecting again"
      ;;

    # Unknown command
    *)
      echo "ERROR: Unknown command: $z_command" >&2
      exit 1
      ;;
  esac
}

zccck_main() {
  local z_command="${1:-}"
  shift || true

  if [ -z "$z_command" ]; then
    echo "ERROR: No command specified" >&2
    exit 1
  fi

  zccck_route "$z_command" "$@"
}

zccck_main "$@"


# eof

