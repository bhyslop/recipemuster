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

# Command: validate - source file and validate (dies on first error)
rbrv_validate() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrv_validate: file argument required"
  test -f "${z_file}" || buc_die "rbrv_validate: file not found: ${z_file}"

  buc_step "Validating RBRV vessel file: ${z_file}"

  # Source the assignment file
  source "${z_file}" || buc_die "rbrv_validate: failed to source ${z_file}"

  # Prepare state (no dying)
  zrbrv_kindle

  # Strict validation (dies on error)
  zrbrv_validate_fields

  buc_step "RBRV vessel valid"
}

# Display one field: name, description, value
zrbrv_render_field() {
  local z_name="$1"
  local z_desc="$2"
  local z_value="${!z_name:-}"

  if test -n "${z_value}"; then
    printf "  ${ZBUC_GREEN}%-30s${ZBUC_RESET} %s\n" "${z_name}" "${z_value}"
  else
    printf "  ${ZBUC_YELLOW}%-30s${ZBUC_RESET} ${ZBUC_CYAN}(not set)${ZBUC_RESET}\n" "${z_name}"
  fi
  printf "  ${ZBUC_CYAN}%-30s %s${ZBUC_RESET}\n" "" "${z_desc}"
}

# Command: render - diagnostic display then validate
rbrv_render() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrv_render: file argument required"
  test -f "${z_file}" || buc_die "rbrv_render: file not found: ${z_file}"

  # Source and kindle (no dying)
  source "${z_file}" || buc_die "rbrv_render: failed to source ${z_file}"
  zrbrv_kindle

  # Display header
  echo ""
  echo "${ZBUC_CYAN}========================================${ZBUC_RESET}"
  echo "${ZBUC_WHITE}RBRV - Recipe Bottle Regime Vessel${ZBUC_RESET}"
  echo "${ZBUC_CYAN}========================================${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${z_file}${ZBUC_RESET}"
  echo ""

  # Core Vessel Identity
  echo "${ZBUC_YELLOW}Core Vessel Identity${ZBUC_RESET}"
  zrbrv_render_field RBRV_SIGIL                  "Unique identifier (must match directory name) — xname 1-64, Required"
  zrbrv_render_field RBRV_DESCRIPTION            "Human-readable description — string 0-512, Optional"
  echo ""

  # Binding Configuration
  echo "${ZBUC_YELLOW}Binding Configuration${ZBUC_RESET}"
  zrbrv_render_field RBRV_BIND_IMAGE             "Source image to copy from registry — fqin 1-512, Required for bind mode"
  echo ""

  # Conjuring Configuration
  echo "${ZBUC_YELLOW}Conjuring Configuration${ZBUC_RESET}"
  zrbrv_render_field RBRV_CONJURE_DOCKERFILE     "Dockerfile path relative to repo root — string 1-512, Required for conjure mode"
  zrbrv_render_field RBRV_CONJURE_BLDCONTEXT     "Build context relative to repo root — string 1-512, Required for conjure mode"
  zrbrv_render_field RBRV_CONJURE_PLATFORMS      "Space-separated target platforms — string 1-512, Required for conjure mode"
  zrbrv_render_field RBRV_CONJURE_BINFMT_POLICY  "Cross-platform policy: allow or forbid — string 1-16, Required for conjure mode"
  echo ""

  # Unexpected variables
  if test ${#ZRBRV_UNEXPECTED[@]} -gt 0; then
    echo "${ZBUC_RED}Unexpected RBRV_ variables:${ZBUC_RESET}"
    local z_var
    for z_var in "${ZRBRV_UNEXPECTED[@]}"; do
      printf "  ${ZBUC_RED}%-30s${ZBUC_RESET} = %s\n" "${z_var}" "${!z_var:-}"
    done
    echo ""
  fi

  # Validate (dies on first error, after full display)
  zrbrv_validate_fields
  buc_step "RBRV vessel valid"
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
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbrv_cli.sh {validate|render} [args]"
    ;;
esac

# eof
