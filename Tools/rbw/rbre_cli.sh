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
# RBRE CLI - Command line interface for RBRE ECR operations

set -euo pipefail

ZRBRE_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZRBRE_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBRE_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBRE_CLI_SCRIPT_DIR}/rbre_regime.sh"
source "${ZRBRE_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBRE_CLI_SCRIPT_DIR}/rbcr_render.sh"

zrbre_cli_kindle() {
  test -z "${ZRBRE_CLI_KINDLED:-}" || buc_die "RBRE CLI already kindled"
  ZRBRE_CLI_KINDLED=1
}

rbre_validate() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbre_validate: file argument required"
  test -f "${z_file}" || buc_die "rbre_validate: file not found: ${z_file}"
  buc_step "Validating RBRE ECR config file: ${z_file}"
  source "${z_file}" || buc_die "rbre_validate: failed to source ${z_file}"
  zrbre_kindle
  buc_step "RBRE ECR config valid"
}

rbre_render() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbre_render: file argument required"
  test -f "${z_file}" || buc_die "rbre_render: file not found: ${z_file}"
  source "${z_file}" || buc_die "rbre_render: failed to source ${z_file}"
  zrbre_kindle
  zrbcr_kindle

  echo ""
  echo "${ZBUC_WHITE}RBRE - Recipe Bottle ECR Regime${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${z_file}${ZBUC_RESET}"
  echo ""

  rbcr_section_begin "AWS Identity"
  rbcr_section_item RBRE_AWS_CREDENTIALS_ENV   string  req  "AWS credentials environment name"
  rbcr_section_item RBRE_AWS_ACCESS_KEY_ID     string  req  "AWS access key identifier"

  # SECURITY: Mask the secret access key
  local z_key_status="[NOT SET]"
  local z_key_color="${ZBUC_RED}"
  if test -n "${RBRE_AWS_SECRET_ACCESS_KEY:-}"; then
    z_key_status="[REDACTED - ${#RBRE_AWS_SECRET_ACCESS_KEY} chars]"
    z_key_color="${ZBUC_GREEN}"
  fi
  printf "  ${z_key_color}%-30s${ZBUC_RESET} %s\n" "RBRE_AWS_SECRET_ACCESS_KEY" "${z_key_status}"

  rbcr_section_item RBRE_AWS_ACCOUNT_ID        string  req  "AWS account identifier"
  rbcr_section_end

  rbcr_section_begin "Region & Repository"
  rbcr_section_item RBRE_AWS_REGION            string  req  "AWS region for ECR"
  rbcr_section_item RBRE_REPOSITORY_NAME       xname   req  "ECR repository name"
  rbcr_section_end

  echo "${ZBUC_GREEN}RBRE ECR config valid${ZBUC_RESET}"
}

# Main dispatch
zrbre_cli_kindle
zrbcc_kindle

z_command="${1:-}"
case "${z_command}" in
  validate)
    z_file="${2:-}"
    test -n "${z_file}" || buc_die "rbre_cli.sh validate: file argument required"
    rbre_validate "${z_file}"
    ;;
  render)
    z_file="${2:-}"
    test -n "${z_file}" || buc_die "rbre_cli.sh render: file argument required"
    rbre_render "${z_file}"
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbre_cli.sh {validate|render} [file]"
    ;;
esac

# eof
