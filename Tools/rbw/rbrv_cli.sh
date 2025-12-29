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
# RBRV CLI - Command line interface for RBRV vessel operations

set -euo pipefail

ZRBRV_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZRBRV_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBRV_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBRV_CLI_SCRIPT_DIR}/rbrv_regime.sh"

######################################################################
# CLI Functions

zrbrv_cli_kindle() {
  test -z "${ZRBRV_CLI_KINDLED:-}" || buc_die "RBRV CLI already kindled"
  ZRBRV_CLI_KINDLED=1
}

# Command: validate - source file and validate
rbrv_validate() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrv_validate: file argument required"
  test -f "${z_file}" || buc_die "rbrv_validate: file not found: ${z_file}"

  buc_step "Validating RBRV vessel file: ${z_file}"

  # Source the assignment file
  source "${z_file}" || buc_die "rbrv_validate: failed to source ${z_file}"

  # Validate via kindle
  zrbrv_kindle

  buc_step "RBRV vessel valid"
}

# Command: render - display configuration values
rbrv_render() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrv_render: file argument required"
  test -f "${z_file}" || buc_die "rbrv_render: file not found: ${z_file}"

  buc_step "RBRV Vessel: ${z_file}"

  # Source the assignment file
  source "${z_file}" || buc_die "rbrv_render: failed to source ${z_file}"

  # Core Vessel Identity
  printf "%-30s %s\n" "RBRV_SIGIL" "${RBRV_SIGIL:-<not set>}"
  printf "%-30s %s\n" "RBRV_DESCRIPTION" "${RBRV_DESCRIPTION:-<not set>}"

  # Binding Configuration
  printf "%-30s %s\n" "RBRV_BIND_IMAGE" "${RBRV_BIND_IMAGE:-<not set>}"

  # Conjuring Configuration
  printf "%-30s %s\n" "RBRV_CONJURE_DOCKERFILE" "${RBRV_CONJURE_DOCKERFILE:-<not set>}"
  printf "%-30s %s\n" "RBRV_CONJURE_BLDCONTEXT" "${RBRV_CONJURE_BLDCONTEXT:-<not set>}"
  printf "%-30s %s\n" "RBRV_CONJURE_PLATFORMS" "${RBRV_CONJURE_PLATFORMS:-<not set>}"
  printf "%-30s %s\n" "RBRV_CONJURE_BINFMT_POLICY" "${RBRV_CONJURE_BINFMT_POLICY:-<not set>}"
}

# Command: info - display specification (formatted for terminal)
rbrv_info() {
  cat <<EOF

${ZBUC_CYAN}========================================${ZBUC_RESET}
${ZBUC_WHITE}RBRV - Recipe Bottle Regime Vessel${ZBUC_RESET}
${ZBUC_CYAN}========================================${ZBUC_RESET}

${ZBUC_YELLOW}Overview${ZBUC_RESET}
Defines a Vessel configuration for container image management.
Vessels can be configured for binding (copying from registry) or
conjuring (building from source). At least one mode must be configured.

${ZBUC_YELLOW}Core Vessel Identity${ZBUC_RESET}

  ${ZBUC_GREEN}RBRV_SIGIL${ZBUC_RESET}
    Unique identifier for this vessel (must match directory name)
    Type: xname (1-64 chars), Required: Yes

  ${ZBUC_GREEN}RBRV_DESCRIPTION${ZBUC_RESET}
    Human-readable description of vessel purpose
    Type: string (0-512 chars), Required: No

${ZBUC_YELLOW}Binding Configuration${ZBUC_RESET}

  ${ZBUC_GREEN}RBRV_BIND_IMAGE${ZBUC_RESET}
    Source image to copy from registry
    Type: fqin (1-512 chars), Required: When using bind mode

${ZBUC_YELLOW}Conjuring Configuration${ZBUC_RESET}

  ${ZBUC_GREEN}RBRV_CONJURE_DOCKERFILE${ZBUC_RESET}
    Path to Dockerfile relative to repo root
    Type: string (1-512 chars), Required: When using conjure mode

  ${ZBUC_GREEN}RBRV_CONJURE_BLDCONTEXT${ZBUC_RESET}
    Build context path relative to repo root
    Type: string (1-512 chars), Required: When using conjure mode

  ${ZBUC_GREEN}RBRV_CONJURE_PLATFORMS${ZBUC_RESET}
    Space-separated target platforms (e.g., "linux/amd64 linux/arm64")
    Type: string (1-512 chars), Required: When using conjure mode

  ${ZBUC_GREEN}RBRV_CONJURE_BINFMT_POLICY${ZBUC_RESET}
    Policy for cross-platform builds: "allow" or "forbid"
    Type: string (1-16 chars), Required: When using conjure mode

EOF
}

######################################################################
# Main dispatch

zrbrv_cli_kindle

z_command="${1:-}"

case "${z_command}" in
  validate)
    shift
    rbrv_validate "${@}"
    ;;
  render)
    shift
    rbrv_render "${@}"
    ;;
  info)
    rbrv_info
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbrv_cli.sh {validate|render|info} [args]"
    ;;
esac

# eof
