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
# Recipe Bottle OAuth Regime - Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRO_SOURCED:-}" || buc_die "Module rbro multiply sourced - check sourcing hierarchy"
ZRBRO_SOURCED=1

######################################################################
# Internal Functions (zrbro_*)

zrbro_kindle() {
  test -z "${ZRBRO_KINDLED:-}" || buc_die "Module rbro already kindled"

  # Set defaults for all fields (validate enforces required-ness)
  RBRO_CLIENT_SECRET="${RBRO_CLIENT_SECRET:-}"
  RBRO_REFRESH_TOKEN="${RBRO_REFRESH_TOKEN:-}"

  # Detect unexpected RBRO_ variables
  local z_known="RBRO_CLIENT_SECRET RBRO_REFRESH_TOKEN"
  ZRBRO_UNEXPECTED=()
  local z_var
  for z_var in $(compgen -v RBRO_); do
    case " ${z_known} " in
      *" ${z_var} "*) : ;;
      *) ZRBRO_UNEXPECTED+=("${z_var}") ;;
    esac
  done

  # Build rollup of all RBRO_ variables for passing to scripts/containers
  # CRITICAL SECURITY: mask both secret values
  ZRBRO_ROLLUP=""
  ZRBRO_ROLLUP+="RBRO_CLIENT_SECRET='[REDACTED]' "
  ZRBRO_ROLLUP+="RBRO_REFRESH_TOKEN='[REDACTED]'"

  ZRBRO_KINDLED=1
}

zrbro_sentinel() {
  test "${ZRBRO_KINDLED:-}" = "1" || buc_die "Module rbro not kindled - call zrbro_kindle first"
}

# Validate RBRO variables via buv_env_* (dies on first error)
# Prerequisite: kindle must have been called; buv_validation.sh must be sourced
zrbro_validate_fields() {
  zrbro_sentinel

  # Die on unexpected variables
  if test ${#ZRBRO_UNEXPECTED[@]} -gt 0; then
    buc_die "Unexpected RBRO_ variables: ${ZRBRO_UNEXPECTED[*]}"
  fi

  # Both credentials are required strings
  buv_env_string RBRO_CLIENT_SECRET 1 512
  buv_env_string RBRO_REFRESH_TOKEN 1 512
}

######################################################################
# Public Functions (rbro_*)

# Load RBRO from the canonical location (~/.rbw/rbro.env)
rbro_load() {
  local z_rbro_dir="${HOME}/.rbw"
  local z_rbro_file="${z_rbro_dir}/rbro.env"

  # Check directory and file exist
  test -d "${z_rbro_dir}" || buc_die "RBRO directory missing (~/.rbw) - run rbgp_payor_install"
  test -f "${z_rbro_file}" || buc_die "RBRO credentials missing (~/.rbw/rbro.env) - run rbgp_payor_install"
  test -r "${z_rbro_file}" || buc_die "RBRO file not readable - check permissions"

  # Source and validate
  source "${z_rbro_file}" || buc_die "Failed to source RBRO credentials"
  zrbro_kindle
  zrbro_validate_fields
}

# eof
