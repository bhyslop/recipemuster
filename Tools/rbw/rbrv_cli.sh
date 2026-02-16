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
source "${ZRBRV_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBRV_CLI_SCRIPT_DIR}/rbrr_regime.sh"
source "${ZRBRV_CLI_SCRIPT_DIR}/rbcr_render.sh"

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

# Command: render - diagnostic display then validate
rbrv_render() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrv_render: file argument required"
  test -f "${z_file}" || buc_die "rbrv_render: file not found: ${z_file}"

  # Source and kindle (no dying — show all fields before validation)
  source "${z_file}" || buc_die "rbrv_render: failed to source ${z_file}"
  zrbrv_kindle
  zrbcr_kindle

  # Display header
  echo ""
  echo "${ZBUC_WHITE}RBRV - Recipe Bottle Regime Vessel${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${z_file}${ZBUC_RESET}"
  echo ""

  # Core Vessel Identity
  rbcr_section_begin "Core Vessel Identity"
  rbcr_section_item RBRV_SIGIL        xname   req  "Unique identifier (must match directory name)"
  rbcr_section_item RBRV_DESCRIPTION  string  opt  "Human-readable description"
  rbcr_section_item RBRV_VESSEL_MODE  enum    req  "Operation mode: bind or conjure"
  rbcr_section_end

  # Binding Configuration (conditional: RBRV_VESSEL_MODE=bind)
  rbcr_section_begin "Binding Configuration" RBRV_VESSEL_MODE bind
  rbcr_section_item RBRV_BIND_IMAGE  fqin  req  "Source image to copy from registry"
  rbcr_section_end

  # Conjuring Configuration (conditional: RBRV_VESSEL_MODE=conjure)
  rbcr_section_begin "Conjuring Configuration" RBRV_VESSEL_MODE conjure
  rbcr_section_item RBRV_CONJURE_DOCKERFILE     string  req  "Dockerfile path relative to repo root"
  rbcr_section_item RBRV_CONJURE_BLDCONTEXT     string  req  "Build context relative to repo root"
  rbcr_section_item RBRV_CONJURE_PLATFORMS      string  req  "Space-separated target platforms"
  rbcr_section_item RBRV_CONJURE_BINFMT_POLICY  enum    req  "Cross-platform policy: allow or forbid"
  rbcr_section_end

  # Unexpected variables (from kindle, not gated)
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
  echo "${ZBUC_GREEN}RBRV vessel valid${ZBUC_RESET}"
}

######################################################################
# Main dispatch

zrbrv_cli_kindle
zrbcc_kindle

z_command="${1:-}"
z_sigil="${2:-}"

case "${z_command}" in
  validate|render)
    if test -z "${z_sigil}"; then
      # Load RBRR to get RBRR_VESSEL_DIR
      test -f "${RBCC_RBRR_FILE}" || buc_die "RBRR regime file not found: ${RBCC_RBRR_FILE}"
      rbrr_load
      buc_step "Available vessels:"
      rbrv_list | while read -r z_s; do
        echo "  ${z_s}"
      done
    else
      # Load RBRR to get RBRR_VESSEL_DIR for path resolution
      test -f "${RBCC_RBRR_FILE}" || buc_die "RBRR regime file not found: ${RBCC_RBRR_FILE}"
      rbrr_load
      z_file="${RBRR_VESSEL_DIR}/${z_sigil}/rbrv.env"
      test -f "${z_file}" || buc_die "Vessel not found: ${z_file}"
      case "${z_command}" in
        validate) rbrv_validate "${z_file}" ;;
        render)   rbrv_render "${z_file}" ;;
      esac
    fi
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbrv_cli.sh {validate|render} [sigil]"
    ;;
esac

# eof
