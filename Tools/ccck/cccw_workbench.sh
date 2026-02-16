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
source "${zccck_buk_directory}/buc_command.sh"

# Show filename on each displayed line
buc_context "${0##*/}"


zccck_docker_compose() {
  docker-compose --env-file "${zccck_kit_dir}/../cccr.env" -f "${zccck_kit_dir}/docker-compose.yml" "$@"
}

# Connect to CCBX container with remote command
zccck_connect() {
  local z_remote_command="$1"

  test -n "${z_remote_command}" || buc_die "Remote command required for zccck_connect"

  buc_step "Connecting to CCCK container with command: ${z_remote_command}"

  # Connect via SSH with configured port and execute remote command
  ssh -p "${CCCR_SSH_PORT}" -tt claude@localhost "${z_remote_command}"
}

# Simple routing function
zccck_route() {
  local z_command="$1"
  shift

  buc_step "Routing command: $z_command with args: $*"

  # Verify BDU environment variables are present
  test -n "${BURD_TEMP_DIR:-}"  || buc_die "BURD_TEMP_DIR not set - must be called from BUD"
  test -n "${BURD_NOW_STAMP:-}" || buc_die "BURD_NOW_STAMP not set - must be called from BUD"

  source "${zccck_kit_dir}/../cccr.env"

  test -n "${CCCR_SSH_PORT:-}"          || buc_die "CCCR_SSH_PORT not set in cccr.env"
  test -n "${CCCR_CLAUDE_CONFIG_DIR:-}" || buc_die "CCCR_CLAUDE_CONFIG_DIR not set in cccr.env"

  buc_step "BUD environment verified: TEMP_DIR=$BURD_TEMP_DIR, NOW_STAMP=$BURD_NOW_STAMP, CCCR_SSH_PORT=${CCCR_SSH_PORT} CCCR_WEB_PORT=${CCCR_WEB_PORT} CCCR_CLAUDE_CONFIG_DIR=${CCCR_CLAUDE_CONFIG_DIR}"

  # Route based on command prefix
  case "$z_command" in

    # Claude Code Container Kit (ccck) Docker commands
    ccck-a)
      buc_step "Creating Claude config directory if needed"
      mkdir -p "${zccck_kit_dir}/${CCCR_CLAUDE_CONFIG_DIR}"
      test -d "${zccck_kit_dir}/${CCCR_CLAUDE_CONFIG_DIR}" || buc_die "Failed to create Claude config directory: ${zccck_kit_dir}/${CCCR_CLAUDE_CONFIG_DIR}"
      
      zccck_docker_compose up -d

      buc_step "Setting up git configuration in container"

      buc_step "Setting git safe directories"
      zccck_connect "git config --global --add safe.directory /workspace/brm_recipebottle"
      zccck_connect "git config --global --add safe.directory /workspace/cnmp_CellNodeMessagePrototype"
      zccck_connect "git config --global --add safe.directory /workspace/recipebottle-admin"

      buc_step "Setting git global configuration"
      zccck_connect "git config --global user.email 'bhyslop@scaleinvariant.org'"
      zccck_connect "git config --global user.name  'Claude Code with Brad Hyslop'"

      buc_step "Container started and configured"
      ;;
    ccck-z)  zccck_docker_compose down                                                        ;;
    ccck-B)  zccck_docker_compose build --no-cache                                            ;;
    ccck-c)  zccck_connect "cd /workspace/brm_recipebottle  &&  claude"                       ;;
    ccck-s)  zccck_connect "cd /workspace/brm_recipebottle  &&  bash"                         ;;
    ccck-g)  zccck_connect "cd /workspace/brm_recipebottle  &&  git status"                   ;;
    ccck-R)
      # Full reset: clean Docker resources and SSH keys
      buc_step "Stopping and removing container"
      docker stop ClaudeCodeBox 2>/dev/null || true
      docker rm   ClaudeCodeBox 2>/dev/null || true

      buc_step "Removing Docker network"
      docker network rm claude-network 2>/dev/null || true

      buc_step "Removing SSH host key for localhost:${CCCR_SSH_PORT}"
      ssh-keygen -R "[localhost]:${CCCR_SSH_PORT}" 2>/dev/null || true

      buc_step "Full reset complete - ready for fresh start"
      ;;

    # GAD (Git AsciiDoc Diff) commands
    gadf-f)
      # Run GAD factory in ccbx container with hardcoded parameters
      zccck_connect "cd /workspace/brm_recipebottle && python3 -u Tools/gad/gadf_factory.py --file ../recipebottle-admin/rbw-RBZG-gadTest.adoc --directory ../gad-working-dir --branch main --max-distinct-renders 3 --once --port ${CCCR_WEB_PORT}"
      ;;
    gadcf)
      # Run GAD factory locally (inside container) with hardcoded parameters
      echo "Running GAD factory locally"
      python3 -u Tools/gad/gadf_factory.py --file ../cnmp_CellNodeMessagePrototype/lenses/gad-GADS-GoogleAsciidocDifferSpecification.adoc --directory ../gad-working-dir --branch bth-20240623-1-flaps --max-distinct-renders 5 --once --port "${CCCR_WEB_PORT}"
      ;;
    gadi-i)
      # Open GAD inspector in browser (served by factory HTTP server)
      echo "GAD inspector available at http://localhost:${CCCR_WEB_PORT}/"
      echo "Note: Ensure GAD factory is running to serve Inspector interface"
      echo "The factory provides integrated HTTP server on port ${CCCR_WEB_PORT}"
      ;;

    # Unknown command
    *)
      buc_die "Unknown command: $z_command"
      ;;
  esac
}

zccck_main() {
  local z_command="${1:-}"
  shift || true

  test -n "$z_command" || buc_die "No command specified"

  zccck_route "$z_command" "$@"
}

zccck_main "$@"


# eof

