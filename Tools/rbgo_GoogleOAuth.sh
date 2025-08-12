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
# Recipe Bottle Google OAuth - Service account JWT authentication

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGO_SOURCED:-}" || bcu_die "Module rbgo multiply sourced - check sourcing hierarchy"
ZRBGO_SOURCED=1

######################################################################
# Internal Functions (zrbgo_*)

zrbgo_kindle() {
  test -z "${ZRBGO_KINDLED:-}" || bcu_die "Module rbgo already kindled"

  # Validate environment
  bvu_dir_exists "${BDU_TEMP_DIR}"
  test -n "${BDU_NOW_STAMP:-}" || bcu_die "BDU_NOW_STAMP is unset or empty"

  # Validate required tools
  which openssl >/dev/null 2>&1 || bcu_die "openssl not found - required for JWT signing"
  which curl    >/dev/null 2>&1 || bcu_die "curl not found - required for OAuth exchange"
  which base64  >/dev/null 2>&1 || bcu_die "base64 not found - required for encoding"
  which jq      >/dev/null 2>&1 || bcu_die "jq not found - required for JSON parsing"

  # OAuth configuration
  ZRBGO_OAUTH_TOKEN_URL="https://oauth2.googleapis.com/token"
  ZRBGO_DEFAULT_SCOPE="https://www.googleapis.com/auth/cloud-platform"

  # Non-sensitive temp files only
  ZRBGO_CURL_OUTPUT_FILE="${BDU_TEMP_DIR}/rbgo_curl_output.txt"
  ZRBGO_DEBUG_RESPONSE_FILE="${BDU_TEMP_DIR}/rbgo_debug_response.json"

  ZRBGO_KINDLED=1
}

zrbgo_sentinel() {
  test "${ZRBGO_KINDLED:-}" = "1" || bcu_die "Module rbgo not kindled - call zrbgo_kindle first"
}

zrbgo_base64url_encode_capture() {
  zrbgo_sentinel

  local z_input="$1"

  # Base64 encode and convert to URL-safe format in one pass
  echo -n "${z_input}" | base64 -w 0 | tr '+/' '-_' | tr -d '=' || return 1
}

zrbgo_build_jwt_capture() {
  zrbgo_sentinel

  local z_rbra_file="$1"

  # Source RBRA file for credentials (never write to disk)
  source "${z_rbra_file}" || return 1

  # Validate required variables
  test -n "${RBRA_CLIENT_EMAIL:-}" || return 1
  test -n "${RBRA_PRIVATE_KEY:-}" || return 1
  test -n "${RBRA_TOKEN_LIFETIME_SEC:-}" || return 1

  # Build JWT header in memory
  local z_header='{"alg":"RS256","typ":"JWT"}'

  # Calculate timestamps using builtin arithmetic
  local z_now
  z_now=$(date +%s) || return 1
  local z_exp=$((z_now + RBRA_TOKEN_LIFETIME_SEC))

  # Build JWT claims in memory
  local z_claims
  z_claims='{'
  z_claims="${z_claims}\"iss\":\"${RBRA_CLIENT_EMAIL}\","
  z_claims="${z_claims}\"scope\":\"${ZRBGO_DEFAULT_SCOPE}\","
  z_claims="${z_claims}\"aud\":\"${ZRBGO_OAUTH_TOKEN_URL}\","
  z_claims="${z_claims}\"iat\":${z_now},"
  z_claims="${z_claims}\"exp\":${z_exp}"
  z_claims="${z_claims}}"

  # Encode header and claims
  local z_header_enc
  z_header_enc=$(zrbgo_base64url_encode_capture "${z_header}") || return 1

  local z_claims_enc
  z_claims_enc=$(zrbgo_base64url_encode_capture "${z_claims}") || return 1

  # Create unsigned JWT in memory
  local z_unsigned="${z_header_enc}.${z_claims_enc}"

  # Sign JWT using openssl with private key from stdin (never touches disk)
  local z_signature_b64
  z_signature_b64=$(
    echo -n "${z_unsigned}" | \
    openssl dgst -sha256 -sign <(printf '%s\n' "${RBRA_PRIVATE_KEY}") -binary 2>/dev/null | \
    base64 -w 0 | tr '+/' '-_' | tr -d '='
  ) || return 1

  # Return complete JWT
  echo "${z_header_enc}.${z_claims_enc}.${z_signature_b64}"
}

zrbgo_exchange_jwt_capture() {
  zrbgo_sentinel

  local z_jwt="$1"

  # Exchange JWT for OAuth token, capturing full response
  curl -s -X POST "${ZRBGO_OAUTH_TOKEN_URL}" \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion=${z_jwt}" \
    > "${ZRBGO_CURL_OUTPUT_FILE}" 2>/dev/null || return 1

  # Copy response for debugging (without sensitive fields)
  jq 'del(.access_token, .refresh_token) | with_entries(select(.key | test("token|secret|key|password"; "i") | not))' \
    "${ZRBGO_CURL_OUTPUT_FILE}" > "${ZRBGO_DEBUG_RESPONSE_FILE}" 2>/dev/null || true

  # Log sanitized response for forensics
  test -f "${ZRBGO_DEBUG_RESPONSE_FILE}" && cat "${ZRBGO_DEBUG_RESPONSE_FILE}" | bcu_log_pipe

  # Extract and validate access token
  local z_token
  z_token=$(jq -r '.access_token // empty' "${ZRBGO_CURL_OUTPUT_FILE}" 2>/dev/null) || return 1
  test -n "${z_token}" || return 1

  echo "${z_token}"
}

zrbgo_get_token_capture() {
  zrbgo_sentinel

  local z_rbra_file="$1"

  # Validate RBRA file exists
  test -f "${z_rbra_file}" || return 1

  bcu_log_args "Building JWT from service account"
  local z_jwt
  z_jwt=$(zrbgo_build_jwt_capture "${z_rbra_file}") || return 1

  bcu_log_args "Exchanging JWT for OAuth token"
  local z_token
  z_token=$(zrbgo_exchange_jwt_capture "${z_jwt}") || return 1

  echo "${z_token}"
}

######################################################################
# External Functions (rbgo_*)

rbgo_get_token() {
  zrbgo_sentinel

  local z_rbra_file="${1:-}"
  local z_output_var="${2:-}"

  # Documentation block
  bcu_doc_brief "Obtain Google OAuth access token from service account credentials"
  bcu_doc_param "rbra_file" "Path to RBRA credentials file containing service account key"
  bcu_doc_oparm "output_var" "Variable name to store token (prints to stdout if omitted)"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_rbra_file}" || bcu_die "Parameter 'rbra_file' is required"
  test -f "${z_rbra_file}" || bcu_die "RBRA file not found: ${z_rbra_file}"

  bcu_step "Authenticating with Google OAuth"

  # Get token using capture function
  local z_token
  z_token=$(zrbgo_get_token_capture "${z_rbra_file}") || bcu_die "Failed to obtain OAuth token"

  # Output handling
  if test -n "${z_output_var}"; then
    # Store in variable (caller's responsibility to handle)
    printf -v "${z_output_var}" '%s' "${z_token}"
    bcu_success "OAuth token stored in ${z_output_var}"
  else
    # Output to stdout for capture
    echo "${z_token}"
  fi
}

rbgo_validate_token() {
  zrbgo_sentinel

  local z_token="${1:-}"

  # Documentation block
  bcu_doc_brief "Validate a Google OAuth access token"
  bcu_doc_param "token" "OAuth access token to validate"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_token}" || bcu_die "Parameter 'token' is required"

  bcu_step "Validating OAuth token"

  # Check token with Google's tokeninfo endpoint
  local z_response
  curl -s "https://oauth2.googleapis.com/tokeninfo?access_token=${z_token}" \
    > "${ZRBGO_CURL_OUTPUT_FILE}" 2>/dev/null || bcu_die "Failed to contact validation endpoint"

  # Check for error in response
  local z_error
  z_error=$(jq -r '.error // empty' "${ZRBGO_CURL_OUTPUT_FILE}" 2>/dev/null) || true

  if test -n "${z_error}"; then
    bcu_die "Token validation failed: ${z_error}"
  fi

  # Log token info (sanitized)
  jq 'del(.access_token) | with_entries(select(.key | test("token|secret|key|password"; "i") | not))' \
    "${ZRBGO_CURL_OUTPUT_FILE}" 2>/dev/null | bcu_log_pipe

  bcu_success "Token is valid"
}

# eof

