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
# Recipe Bottle Payor Regime - Validator

set -euo pipefail

ZRBRP_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Kindle RBGC if not already done (needed for pattern constants)
if [ -z "${ZRBGC_KINDLED:-}" ]; then
  source "${ZRBRP_SCRIPT_DIR}/rbgc_Constants.sh"
  zrbgc_kindle
fi

# Validate RBRP_PAYOR_PROJECT_ID against global naming pattern
if [ -z "${RBRP_PAYOR_PROJECT_ID:-}" ]; then
  buc_die "RBRP_PAYOR_PROJECT_ID is not set.
To set a fresh value, run:
  sed -i '' \"s/^RBRP_PAYOR_PROJECT_ID=.*/RBRP_PAYOR_PROJECT_ID=${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-\$(date ${RBGC_GLOBAL_TIMESTAMP_FORMAT})/\" rbrp.env
Or add to rbrp.env:
  RBRP_PAYOR_PROJECT_ID=${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-\$(date ${RBGC_GLOBAL_TIMESTAMP_FORMAT})"
fi

if ! printf '%s' "${RBRP_PAYOR_PROJECT_ID}" | grep -qE "${RBGC_GLOBAL_PAYOR_REGEX}"; then
  buc_die "RBRP_PAYOR_PROJECT_ID '${RBRP_PAYOR_PROJECT_ID}' does not match required pattern.
Expected pattern: ${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-YYMMDDHHMMSS
To set a fresh value, run:
  sed -i '' \"s/^RBRP_PAYOR_PROJECT_ID=.*/RBRP_PAYOR_PROJECT_ID=${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-\$(date ${RBGC_GLOBAL_TIMESTAMP_FORMAT})/\" rbrp.env"
fi

# Validate RBRP_BILLING_ACCOUNT_ID format (XXXXXX-XXXXXX-XXXXXX)
# This is optional during initial payor establishment
if [ -n "${RBRP_BILLING_ACCOUNT_ID:-}" ]; then
  if ! printf '%s' "${RBRP_BILLING_ACCOUNT_ID}" | grep -qE '^[A-Z0-9]{6}-[A-Z0-9]{6}-[A-Z0-9]{6}$'; then
    buc_die "RBRP_BILLING_ACCOUNT_ID '${RBRP_BILLING_ACCOUNT_ID}' does not match required format.
Expected format: XXXXXX-XXXXXX-XXXXXX (e.g., 01A2B3-4C5D6E-7F8G9H)"
  fi
fi

# Validate RBRP_OAUTH_CLIENT_ID format (ends with .apps.googleusercontent.com)
# This is optional during initial payor establishment
if [ -n "${RBRP_OAUTH_CLIENT_ID:-}" ]; then
  if ! printf '%s' "${RBRP_OAUTH_CLIENT_ID}" | grep -qE '\.apps\.googleusercontent\.com$'; then
    buc_die "RBRP_OAUTH_CLIENT_ID '${RBRP_OAUTH_CLIENT_ID}' does not match required format.
Expected format: <client-id>.apps.googleusercontent.com"
  fi
fi

# eof
