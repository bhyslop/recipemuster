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
# Recipe Bottle Payor Regime - Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRP_SOURCED:-}" || buc_die "Module rbrp multiply sourced - check sourcing hierarchy"
ZRBRP_SOURCED=1

######################################################################
# Internal Functions (zrbrp_*)

zrbrp_kindle() {
  test -z "${ZRBRP_KINDLED:-}" || buc_die "Module rbrp already kindled"

  # Requires RBGC for pattern constants
  zrbgc_sentinel

  # Precompute suggested project ID for error messages
  local z_suggested_id="${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-$(date ${RBGC_GLOBAL_TIMESTAMP_FORMAT})"

  # Validate RBRP_PAYOR_PROJECT_ID exists
  if [ -z "${RBRP_PAYOR_PROJECT_ID:-}" ]; then
    buc_warn "RBRP_PAYOR_PROJECT_ID is not set"
    buc_step "To set a fresh value, run:"
    buc_code "sed -i '' 's/^RBRP_PAYOR_PROJECT_ID=.*/RBRP_PAYOR_PROJECT_ID=${z_suggested_id}/' rbrp.env"
    buc_die "Cannot proceed without RBRP_PAYOR_PROJECT_ID"
  fi

  # Validate RBRP_PAYOR_PROJECT_ID matches global naming pattern
  if ! printf '%s' "${RBRP_PAYOR_PROJECT_ID}" | grep -qE "${RBGC_GLOBAL_PAYOR_REGEX}"; then
    buc_warn "RBRP_PAYOR_PROJECT_ID '${RBRP_PAYOR_PROJECT_ID}' does not match required pattern"
    buc_step "Expected pattern: ${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-YYMMDDHHMMSS"
    buc_step "To set a fresh value, run the following and then git commit:"
    buc_code "sed -i '' 's/^RBRP_PAYOR_PROJECT_ID=.*/RBRP_PAYOR_PROJECT_ID=${z_suggested_id}/' rbrp.env"
    buc_die "Invalid RBRP_PAYOR_PROJECT_ID format"
  fi

  # Validate RBRP_BILLING_ACCOUNT_ID format (optional during initial setup)
  if [ -n "${RBRP_BILLING_ACCOUNT_ID:-}" ]; then
    if ! printf '%s' "${RBRP_BILLING_ACCOUNT_ID}" | grep -qE '^[A-Z0-9]{6}-[A-Z0-9]{6}-[A-Z0-9]{6}$'; then
      buc_warn "RBRP_BILLING_ACCOUNT_ID '${RBRP_BILLING_ACCOUNT_ID}' does not match required format"
      buc_step "Expected format: XXXXXX-XXXXXX-XXXXXX (e.g., 01A2B3-4C5D6E-7F8G9H)"
      buc_die "Invalid RBRP_BILLING_ACCOUNT_ID format"
    fi
  fi

  # Validate RBRP_OAUTH_CLIENT_ID format (optional during initial setup)
  if [ -n "${RBRP_OAUTH_CLIENT_ID:-}" ]; then
    if ! printf '%s' "${RBRP_OAUTH_CLIENT_ID}" | grep -qE '\.apps\.googleusercontent\.com$'; then
      buc_warn "RBRP_OAUTH_CLIENT_ID '${RBRP_OAUTH_CLIENT_ID}' does not match required format"
      buc_step "Expected format: <client-id>.apps.googleusercontent.com"
      buc_die "Invalid RBRP_OAUTH_CLIENT_ID format"
    fi
  fi

  ZRBRP_KINDLED=1
}

zrbrp_sentinel() {
  test "${ZRBRP_KINDLED:-}" = "1" || buc_die "Module rbrp not kindled - call zrbrp_kindle first"
}

######################################################################
# Public Functions (rbrp_*)

# Load RBRP regime
# Usage: rbrp_load
# Prerequisite: RBCC must be kindled (needs RBCC_rbrp_file), RBGC must be kindled (zrbrp_kindle uses RBGC patterns)
rbrp_load() {
  local z_rbrp_file="${RBCC_rbrp_file}"
  test -f "${z_rbrp_file}" || buc_die "RBRP config not found: ${z_rbrp_file}"
  source "${z_rbrp_file}" || buc_die "Failed to source RBRP config: ${z_rbrp_file}"
  zrbrp_kindle
}

# eof
