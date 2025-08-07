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
  which jq      >/dev/null 2>&1 || bcu_die "jq not found - required for JSON processing"
  which openssl >/dev/null 2>&1 || bcu_die "openssl not found - required for JWT signing"
  which curl    >/dev/null 2>&1 || bcu_die "curl not found - required for OAuth exchange"

  bcu_log "Check environment"
  test -n "${BDU_TEMP_DIR:-}" || bcu_die "BDU_TEMP_DIR not set"
  test -d "${BDU_TEMP_DIR}"   || bcu_die "BDU_TEMP_DIR not a directory"

  bcu_log "Validate RBRR configuration variables"
  test -n "${RBRR_GAR_RBRA_FILE:-}" || bcu_die "RBRR_GAR_RBRA_FILE not set"
  test -n "${RBRR_GCB_RBRA_FILE:-}" || bcu_die "RBRR_GCB_RBRA_FILE not set"

  bcu_log "Set Module Variables (ZRBGO_*)"
  ZRBGO_JWT_HEADER_FILE="${BDU_TEMP_DIR}/rbgo_jwt_header.json"
  ZRBGO_JWT_CLAIMS_FILE="${BDU_TEMP_DIR}/rbgo_jwt_claims.json"
  ZRBGO_JWT_TIMESTAMP_FILE="${BDU_TEMP_DIR}/rbgo_jwt_timestamp.txt"
  ZRBGO_OAUTH_RESPONSE_FILE="${BDU_TEMP_DIR}/rbgo_oauth_response.json"
  ZRBGO_VALIDATION_PREFIX="${BDU_TEMP_DIR}/rbgo_validation_"

  bcu_log "OAuth endpoint"
  ZRBGO_OAUTH_TOKEN_URL="https://oauth2.googleapis.com/token"

  bcu_log "Default scope for all Google Cloud services"
  ZRBGO_DEFAULT_SCOPE="https://www.googleapis.com/auth/cloud-platform"

  bcu_log "Check service account environment files"
  if test -f "${RBRR_GAR_RBRA_FILE:-}"; then
    bcu_info "Google Artifact Registry service account env file: present"
  else
    bcu_info "Google Artifact Registry service account env file: ABSENT"
  fi

  if test -f "${RBRR_GCB_RBRA_FILE:-}"; then
    bcu_info "Google Cloud Build service account env file: present"
  else
    bcu_info "Google Cloud Build service account env file: absent"
  fi

  ZRBGO_KINDLED=1
}

zrbgo_sentinel() {
  test "${ZRBGO_KINDLED:-}" = "1" || bcu_die "Module rbgo not kindled - call zrbgo_kindle first"
}

zrbgo_base64url_encode_capture() {
  zrbgo_sentinel

  local z_input="$1"

  bcu_log "Base64 encode and convert to URL-safe format"
  local z_b64
  z_b64=$(printf "%s" "${z_input}" | base64 -w 0) || return 1

  bcu_log "Convert to URL-safe: + to -, / to _, remove ="
  local z_result="${z_b64}"
  z_result="${z_result//+/-}"
  z_result="${z_result//\//_}"
  z_result="${z_result//=/}"

  echo "${z_result}"
}

zrbgo_sign_jwt_capture() {
  zrbgo_sentinel

  local z_jwt_unsigned="$1"
  local z_key_json="$2"

  bcu_log "Extract private key from service account JSON"
  local z_private_key
  z_private_key=$(jq -r '.private_key' "${z_key_json}" 2>/dev/null) || return 1
  test -n "${z_private_key}" || return 1
  test "${z_private_key}" != "null" || return 1

  bcu_log "Sign with RSA256 and encode"
  local z_signature
  z_signature=$(echo -n "${z_jwt_unsigned}" | \
    openssl dgst -sha256 -sign <(echo "${z_private_key}") -binary 2>/dev/null | \
    base64 -w 0 | tr '+/' '-_' | tr -d '=') || return 1

  test -n "${z_signature}" || return 1

  echo "${z_signature}"
}

zrbgo_build_jwt_capture() {
  zrbgo_sentinel

  local z_key_json="$1"
  local z_lifetime_seconds="$2"

  bcu_log "Validate service account JSON structure"
  local z_sa_email
  z_sa_email=$(jq -r '.client_email' "${z_key_json}" 2>/dev/null) || return 1
  test -n "${z_sa_email}"                                         || return 1
  test "${z_sa_email}" != "null"                                  || return 1

  bcu_log "Build JWT header"
  local z_header='{"alg":"RS256","typ":"JWT"}'

  bcu_log "Get current timestamp"
  date +%s >     "${ZRBGO_JWT_TIMESTAMP_FILE}" || return 1
  local z_now=$(<"${ZRBGO_JWT_TIMESTAMP_FILE}")
  test -n "${z_now}"                           || return 1

  local z_exp=$((z_now + z_lifetime_seconds))

  bcu_log "Build JWT claims"
  jq -n                                  \
    --arg iss "${z_sa_email}"            \
    --arg scope "${ZRBGO_DEFAULT_SCOPE}" \
    --arg aud "${ZRBGO_OAUTH_TOKEN_URL}" \
    --arg iat "${z_now}"                 \
    --arg exp "${z_exp}"                 \
    '{"iss":$iss,"scope":$scope,"aud":$aud,"iat":($iat|tonumber),"exp":($exp|tonumber)}' \
    > "${ZRBGO_JWT_CLAIMS_FILE}" 2>/dev/null || return 1

  local z_claims=$(<"${ZRBGO_JWT_CLAIMS_FILE}")
  test -n "${z_claims}" || return 1

  bcu_log "Encode header and claims"
  local z_header_enc
  z_header_enc=$(zrbgo_base64url_encode_capture "${z_header}") || return 1

  local z_claims_enc
  z_claims_enc=$(zrbgo_base64url_encode_capture "${z_claims}") || return 1

  bcu_log "Create unsigned JWT"
  local z_jwt_unsigned="${z_header_enc}.${z_claims_enc}"

  bcu_log "Sign JWT"
  local z_signature
  z_signature=$(zrbgo_sign_jwt_capture "${z_jwt_unsigned}" "${z_key_json}") || return 1

  bcu_log "Return complete JWT"
  echo "${z_jwt_unsigned}.${z_signature}"
}

zrbgo_exchange_jwt_capture() {
  zrbgo_sentinel

  local z_jwt="$1"

  bcu_log "Exchange JWT for OAuth token"
  curl -s -X POST "${ZRBGO_OAUTH_TOKEN_URL}"                                       \
    -H "Content-Type: application/x-www-form-urlencoded"                           \
    -d "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion=${z_jwt}" \
    > "${ZRBGO_OAUTH_RESPONSE_FILE}" 2>/dev/null || return 1

  bcu_log "Extract access token"
  local z_token
  z_token=$(jq -r '.access_token' "${ZRBGO_OAUTH_RESPONSE_FILE}" 2>/dev/null) || return 1
  test -n "${z_token}" || return 1
  test "${z_token}" != "null" || return 1

  echo "${z_token}"
}

######################################################################
# External Functions (rbgo_*)

rbgo_get_token_capture() {
  zrbgo_sentinel

  local z_service_env_file="$1"

  bcu_log "Source service account credentials"
  source "${z_service_env_file}" || return 1

  bcu_log "Validate required RBRS variables"
  test -n "${RBRA_SERVICE_ACCOUNT_KEY:-}" || return 1
  test -n "${RBRA_TOKEN_LIFETIME_SEC:-}"  || return 1

  bcu_log "Validate lifetime bounds (Google allows 300-3600)"
  test "${RBRA_TOKEN_LIFETIME_SEC}" -ge 300  || return 1
  test "${RBRA_TOKEN_LIFETIME_SEC}" -le 3600 || return 1

  bcu_log "Validate key file exists"
  test -f "${RBRA_SERVICE_ACCOUNT_KEY}" || return 1

  bcu_log "Validate JSON structure"
  local z_validation_file="${ZRBGO_VALIDATION_PREFIX}structure.txt"
  jq -r 'has("client_email") and has("private_key")' "${RBRA_SERVICE_ACCOUNT_KEY}" \
    > "${z_validation_file}" 2>/dev/null || return 1

  local z_valid=$(<"${z_validation_file}")
  test "${z_valid}" = "true" || return 1

  bcu_log "Build JWT"
  local z_jwt
  z_jwt=$(zrbgo_build_jwt_capture "${RBRA_SERVICE_ACCOUNT_KEY}" "${RBRA_TOKEN_LIFETIME_SEC}") || return 1

  bcu_log "Exchange for OAuth token"
  local z_token
  z_token=$(zrbgo_exchange_jwt_capture "${z_jwt}") || return 1

  echo "${z_token}"
}

# eof

