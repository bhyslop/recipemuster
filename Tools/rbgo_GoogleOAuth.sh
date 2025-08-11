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

  bcu_log "Validate required tools"
  which openssl >/dev/null 2>&1 || bcu_die "openssl not found - required for JWT signing"
  which curl    >/dev/null 2>&1 || bcu_die "curl not found - required for OAuth exchange"
  which base64  >/dev/null 2>&1 || bcu_die "base64 not found - required for encoding"

  bcu_log "Check environment"
  test -n "${BDU_TEMP_DIR:-}" || bcu_die "BDU_TEMP_DIR not set"
  test -d "${BDU_TEMP_DIR}"   || bcu_die "BDU_TEMP_DIR not a directory"

  bcu_log "Set Module Variables (ZRBGO_*)"
  ZRBGO_JWT_HEADER_FILE="${BDU_TEMP_DIR}/rbgo_jwt_header.json"
  ZRBGO_JWT_CLAIMS_FILE="${BDU_TEMP_DIR}/rbgo_jwt_claims.json"
  ZRBGO_JWT_UNSIGNED_FILE="${BDU_TEMP_DIR}/rbgo_jwt_unsigned.txt"
  ZRBGO_JWT_SIGNATURE_FILE="${BDU_TEMP_DIR}/rbgo_jwt_signature.txt"
  ZRBGO_OAUTH_RESPONSE_FILE="${BDU_TEMP_DIR}/rbgo_oauth_response.json"
  ZRBGO_PRIVATE_KEY_FILE="${BDU_TEMP_DIR}/rbgo_private_key.pem"

  bcu_log "OAuth endpoint"
  ZRBGO_OAUTH_TOKEN_URL="https://oauth2.googleapis.com/token"

  bcu_log "Default scope for all Google Cloud services"
  ZRBGO_DEFAULT_SCOPE="https://www.googleapis.com/auth/cloud-platform"

  ZRBGO_KINDLED=1
}

zrbgo_sentinel() {
  test "${ZRBGO_KINDLED:-}" = "1" || bcu_die "Module rbgo not kindled - call zrbgo_kindle first"
}

zrbgo_base64url_encode_capture() {
  zrbgo_sentinel

  local z_input="$1"

  # Base64 encode and convert to URL-safe format
  echo -n "${z_input}" | base64 -w 0 | tr '+/' '-_' | tr -d '='
}

zrbgo_build_jwt_capture() {
  zrbgo_sentinel

  local z_rbra_file="$1"

  # Source RBRA file
  source "${z_rbra_file}" || return 1

  # Validate required variables
  test -n "${RBRA_CLIENT_EMAIL:-}" || return 1
  test -n "${RBRA_PRIVATE_KEY:-}" || return 1
  test -n "${RBRA_TOKEN_LIFETIME_SEC:-}" || return 1

  # Build JWT header
  echo '{"alg":"RS256","typ":"JWT"}' > "${ZRBGO_JWT_HEADER_FILE}"

  # Calculate timestamps
  local z_now
  z_now=$(date +%s) || return 1
  local z_exp=$((z_now + RBRA_TOKEN_LIFETIME_SEC))

  # Build JWT claims
  cat > "${ZRBGO_JWT_CLAIMS_FILE}" << EOF
{
  "iss": "${RBRA_CLIENT_EMAIL}",
  "scope": "${ZRBGO_DEFAULT_SCOPE}",
  "aud": "${ZRBGO_OAUTH_TOKEN_URL}",
  "iat": ${z_now},
  "exp": ${z_exp}
}
EOF

  # Encode header and claims
  local z_header_enc
  z_header_enc=$(zrbgo_base64url_encode_capture "$(<"${ZRBGO_JWT_HEADER_FILE}")") || return 1

  local z_claims_enc
  z_claims_enc=$(zrbgo_base64url_encode_capture "$(<"${ZRBGO_JWT_CLAIMS_FILE}")") || return 1

  # Create unsigned JWT
  echo "${z_header_enc}.${z_claims_enc}" > "${ZRBGO_JWT_UNSIGNED_FILE}"

  # Convert literal \n to actual newlines for openssl
  echo -e "${RBRA_PRIVATE_KEY}" > "${ZRBGO_PRIVATE_KEY_FILE}"

  # Sign with RSA256
  openssl dgst -sha256 -sign "${ZRBGO_PRIVATE_KEY_FILE}" \
    -out "${ZRBGO_JWT_SIGNATURE_FILE}" \
    "${ZRBGO_JWT_UNSIGNED_FILE}" 2>/dev/null || return 1

  # Base64url encode signature
  local z_signature
  z_signature=$(base64 -w 0 < "${ZRBGO_JWT_SIGNATURE_FILE}" | tr '+/' '-_' | tr -d '=') || return 1

  # Return complete JWT
  echo "${z_header_enc}.${z_claims_enc}.${z_signature}"
}

zrbgo_exchange_jwt_capture() {
  zrbgo_sentinel

  local z_jwt="$1"

  # Exchange JWT for OAuth token
  curl -s -X POST "${ZRBGO_OAUTH_TOKEN_URL}" \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion=${z_jwt}" \
    > "${ZRBGO_OAUTH_RESPONSE_FILE}" 2>/dev/null || return 1

  # Debug: Show the actual response
  bcu_log "OAuth Response: $(cat "${ZRBGO_OAUTH_RESPONSE_FILE}")"

  # Extract access token
  local z_token
  z_token=$(grep -o '"access_token":"[^"]*' "${ZRBGO_OAUTH_RESPONSE_FILE}" | cut -d'"' -f4) || return 1
  test -n "${z_token}" || return 1

  echo "${z_token}"
}

######################################################################
# External Functions (rbgo_*)

rbgo_get_token_capture() {
  zrbgo_sentinel

  local z_rbra_file="$1"

  bcu_log "Validate RBRA file exists"
  test -f "${z_rbra_file}" || return 1

  bcu_log "Build JWT"
  local z_jwt
  z_jwt=$(zrbgo_build_jwt_capture "${z_rbra_file}") || return 1

  bcu_log "Exchange for OAuth token"
  local z_token
  z_token=$(zrbgo_exchange_jwt_capture "${z_jwt}") || return 1

  echo "${z_token}"
}

# eof

