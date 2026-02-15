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
# Recipe Bottle Authentication Regime - Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRA_SOURCED:-}" || buc_die "Module rbra multiply sourced - check sourcing hierarchy"
ZRBRA_SOURCED=1

######################################################################
# Internal Functions (zrbra_*)

zrbra_kindle() {
  test -z "${ZRBRA_KINDLED:-}" || buc_die "Module rbra already kindled"

  # Set defaults for all fields (validate enforces required-ness)
  RBRA_CLIENT_EMAIL="${RBRA_CLIENT_EMAIL:-}"
  RBRA_PRIVATE_KEY="${RBRA_PRIVATE_KEY:-}"
  RBRA_PROJECT_ID="${RBRA_PROJECT_ID:-}"
  RBRA_TOKEN_LIFETIME_SEC="${RBRA_TOKEN_LIFETIME_SEC:-}"

  # Detect unexpected RBRA_ variables
  local z_known="RBRA_CLIENT_EMAIL RBRA_PRIVATE_KEY RBRA_PROJECT_ID RBRA_TOKEN_LIFETIME_SEC"
  ZRBRA_UNEXPECTED=()
  local z_var
  for z_var in $(compgen -v RBRA_); do
    case " ${z_known} " in
      *" ${z_var} "*) : ;;
      *) ZRBRA_UNEXPECTED+=("${z_var}") ;;
    esac
  done

  # Build rollup of all RBRA_ variables for passing to scripts/containers
  # CRITICAL SECURITY: mask RBRA_PRIVATE_KEY
  ZRBRA_ROLLUP=""
  ZRBRA_ROLLUP+="RBRA_CLIENT_EMAIL='${RBRA_CLIENT_EMAIL}' "
  ZRBRA_ROLLUP+="RBRA_PRIVATE_KEY='[REDACTED]' "
  ZRBRA_ROLLUP+="RBRA_PROJECT_ID='${RBRA_PROJECT_ID}' "
  ZRBRA_ROLLUP+="RBRA_TOKEN_LIFETIME_SEC='${RBRA_TOKEN_LIFETIME_SEC}'"

  ZRBRA_KINDLED=1
}

zrbra_sentinel() {
  test "${ZRBRA_KINDLED:-}" = "1" || buc_die "Module rbra not kindled - call zrbra_kindle first"
}

# Validate RBRA variables via buv_env_* (dies on first error)
# Prerequisite: kindle must have been called; buv_validation.sh must be sourced
zrbra_validate_fields() {
  zrbra_sentinel

  # Die on unexpected variables
  if test ${#ZRBRA_UNEXPECTED[@]} -gt 0; then
    buc_die "Unexpected RBRA_ variables: ${ZRBRA_UNEXPECTED[*]}"
  fi

  # Core credential fields
  buv_env_string RBRA_CLIENT_EMAIL 1 256
  buv_env_string RBRA_PRIVATE_KEY 1 4096
  buv_env_string RBRA_PROJECT_ID 1 64
  buv_env_decimal RBRA_TOKEN_LIFETIME_SEC 300 3600

  # Additional format validation: client email must match service account pattern
  if ! printf '%s' "${RBRA_CLIENT_EMAIL}" | grep -qE '\.iam\.gserviceaccount\.com$'; then
    buc_die "RBRA_CLIENT_EMAIL does not match service account pattern (*.iam.gserviceaccount.com): '${RBRA_CLIENT_EMAIL}'"
  fi

  # Additional format validation: private key must contain PEM key material
  if ! printf '%s' "${RBRA_PRIVATE_KEY}" | grep -q 'BEGIN'; then
    buc_die "RBRA_PRIVATE_KEY does not contain PEM key material"
  fi
}

######################################################################
# Public Functions (rbra_*)

# NO rbra_load() function - RBRA is multi-instance (manifold)
# Loading happens through RBRR references; CLI handles per-file operations

# eof
