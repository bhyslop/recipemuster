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
test -z "${ZRBGO_SOURCED:-}" || buc_die "Module rbgo multiply sourced - check sourcing hierarchy"
ZRBGO_SOURCED=1

######################################################################
# Internal Functions (zrbgo_*)

zrbgo_kindle() {
  test -z "${ZRBGO_KINDLED:-}" || buc_die "Module rbgo already kindled"

  # Validate required tools (rehomed from rbl_Locator.sh)
  command -v openssl >/dev/null 2>&1 || buc_die "openssl not found - required for JWT signing and encoding"
  command -v curl    >/dev/null 2>&1 || buc_die "curl not found - required for OAuth exchange"
  command -v jq      >/dev/null 2>&1 || buc_die "jq not found - required for JSON parsing"

  buc_log_args "Ensure RBGC is kindled first"
  zrbgc_sentinel

  buc_log_args "Check environment"
  zburd_sentinel

  buc_log_args "Set Module Variables (ZRBGO_*)"
  readonly ZRBGO_JWT_HEADER_FILE="${BURD_TEMP_DIR}/rbgo_jwt_header.json"
  readonly ZRBGO_JWT_CLAIMS_FILE="${BURD_TEMP_DIR}/rbgo_jwt_claims.json"
  readonly ZRBGO_JWT_UNSIGNED_FILE="${BURD_TEMP_DIR}/rbgo_jwt_unsigned.txt"
  readonly ZRBGO_JWT_SIGNATURE_FILE="${BURD_TEMP_DIR}/rbgo_jwt_signature.txt"
  readonly ZRBGO_OAUTH_RESPONSE_FILE="${BURD_TEMP_DIR}/rbgo_oauth_response.json"
  readonly ZRBGO_PRIVATE_KEY_FILE="${BURD_TEMP_DIR}/rbgo_private_key.pem"
  readonly ZRBGO_OPENSSL_STDERR_FILE="${BURD_TEMP_DIR}/rbgo_openssl_stderr.txt"
  readonly ZRBGO_CURL_STDERR_FILE="${BURD_TEMP_DIR}/rbgo_curl_stderr.txt"
  readonly ZRBGO_JQ_STDERR_FILE="${BURD_TEMP_DIR}/rbgo_jq_stderr.txt"
  readonly ZRBGO_PROBE_STDERR_FILE="${BURD_TEMP_DIR}/rbgo_probe_stderr.txt"

  readonly ZRBGO_KINDLED=1
}

zrbgo_sentinel() {
  test "${ZRBGO_KINDLED:-}" = "1" || buc_die "Module rbgo not kindled - call zrbgo_kindle first"
}

zrbgo_base64url_encode_capture() {
  zrbgo_sentinel

  local -r z_input="$1"

  # Base64url encode: -A suppresses line wrapping, then URL-safe transform and remove padding
  local z_encoded
  z_encoded=$(printf '%s' "${z_input}" | openssl enc -base64 -A) || return 1
  z_encoded="${z_encoded//+/-}"
  z_encoded="${z_encoded//\//_}"
  z_encoded="${z_encoded//=/}"
  printf '%s' "${z_encoded}"
}

zrbgo_build_jwt_capture() {
  zrbgo_sentinel

  local -r z_rbra_file="$1"

  # RBRA_* expected: CLIENT_EMAIL, PRIVATE_KEY, TOKEN_LIFETIME_SEC
  # RBRA_PRIVATE_KEY contains \n sequences that must become real newlines for openssl
  # Only source if not already loaded (avoids readonly conflict in subshells)
  if test -z "${RBRA_CLIENT_EMAIL:-}"; then
    buc_log_args "Source RBRA file"
    source "${z_rbra_file}" || return 1
  fi

  buc_log_args "Validate required variables"
  test -n "${RBRA_CLIENT_EMAIL:-}"       || return 1
  test -n "${RBRA_PRIVATE_KEY:-}"        || return 1
  test -n "${RBRA_TOKEN_LIFETIME_SEC:-}" || return 1

  buc_log_args "Build JWT header"
  printf '%s' '{"alg":"RS256","typ":"JWT"}' > "${ZRBGO_JWT_HEADER_FILE}"

  buc_log_args "Calculate timestamps"
  local z_now
  z_now=$(date +%s) || return 1
  local -r z_exp=$((z_now + RBRA_TOKEN_LIFETIME_SEC))

  buc_log_args "Build JWT claims"
  jq -n                                         \
    --arg iss   "${RBRA_CLIENT_EMAIL}"          \
    --arg scope "${RBGC_SCOPE_CLOUD_PLATFORM}"  \
    --arg aud   "${RBGC_OAUTH_TOKEN_URL}"       \
    --argjson iat "${z_now}"                    \
    --argjson exp "${z_exp}"                    \
    '{iss: $iss, scope: $scope, aud: $aud, iat: $iat, exp: $exp}' \
    > "${ZRBGO_JWT_CLAIMS_FILE}" || return 1

  buc_log_args "Encode header and claims"
  local z_header_enc
  z_header_enc=$(zrbgo_base64url_encode_capture "$(<"${ZRBGO_JWT_HEADER_FILE}")") || return 1

  local z_claims_enc
  z_claims_enc=$(zrbgo_base64url_encode_capture "$(<"${ZRBGO_JWT_CLAIMS_FILE}")") || return 1

  buc_log_args "Create unsigned JWT"
  printf '%s' "${z_header_enc}.${z_claims_enc}" > "${ZRBGO_JWT_UNSIGNED_FILE}"

  buc_log_args "Sign with RSA256 (without writing key to disk)"
  # Convert literal \n sequences to real newlines via printf %b and feed via process substitution
  openssl dgst -sha256 \
    -sign <(printf '%b' "${RBRA_PRIVATE_KEY}\n") \
    -out "${ZRBGO_JWT_SIGNATURE_FILE}" \
    "${ZRBGO_JWT_UNSIGNED_FILE}" 2>"${ZRBGO_OPENSSL_STDERR_FILE}" || return 1

  buc_log_args "Base64url encode signature"
  local z_signature
  z_signature=$(openssl enc -base64 -A < "${ZRBGO_JWT_SIGNATURE_FILE}") || return 1
  z_signature="${z_signature//+/-}"
  z_signature="${z_signature//\//_}"
  z_signature="${z_signature//=/}"

  buc_log_args "Return complete JWT"
  printf '%s\n' "${z_header_enc}.${z_claims_enc}.${z_signature}"
}

zrbgo_exchange_jwt_capture() {
  zrbgo_sentinel

  local -r z_jwt="$1"

  buc_log_args "Exchange JWT for OAuth token"
  curl -sS -X POST "${RBGC_OAUTH_TOKEN_URL}"                                        \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}"                           \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"                                         \
    -H "Content-Type: application/x-www-form-urlencoded"                           \
    -d "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion=${z_jwt}" \
    > "${ZRBGO_OAUTH_RESPONSE_FILE}" 2>"${ZRBGO_CURL_STDERR_FILE}" || {
      buc_warn "OAuth curl failed: $(<"${ZRBGO_CURL_STDERR_FILE}")"
      return 1
    }

  buc_log_args "Debug: Show the actual response (minus secrets)"
  jq 'del(.access_token, .refresh_token)
      | with_entries(select(.key | test("token|secret|key|password"; "i") | not))' \
    "${ZRBGO_OAUTH_RESPONSE_FILE}" 2>"${ZRBGO_JQ_STDERR_FILE}" | buc_log_pipe || buc_log_args "OAuth response parsing failed"

  buc_log_args "OAuth token exchange completed"

  buc_log_args "Extract access token"
  local z_token
  z_token=$(jq -r '.access_token // empty' "${ZRBGO_OAUTH_RESPONSE_FILE}") || return 1
  test -n "${z_token}" || {
    buc_warn "OAuth response missing access_token; redacted response body follows:"
    jq 'with_entries(select(.key | test("token|secret|key|password"; "i") | not))' \
      "${ZRBGO_OAUTH_RESPONSE_FILE}" 2>"${ZRBGO_JQ_STDERR_FILE}" >&2 \
      || cat "${ZRBGO_OAUTH_RESPONSE_FILE}" >&2
    return 1
  }

  printf '%s\n' "${z_token}"
}

######################################################################
# External Functions (rbgo_*)

rbgo_get_token_capture() {
  zrbgo_sentinel

  local -r z_rbra_file="$1"

  # Documentation block
  buc_doc_brief "Exchange service account credentials for OAuth2 access token"
  buc_doc_param "rbra_file" "Path to RBRA credentials file containing RBRA_* variables"
  buc_doc_shown || return 0

  buc_log_args "Validate RBRA file exists"
  test -f "${z_rbra_file}" || return 1

  buc_log_args "Build JWT"
  local z_jwt
  z_jwt=$(zrbgo_build_jwt_capture "${z_rbra_file}") || return 1

  buc_log_args "Exchange for OAuth token"
  local z_token
  z_token=$(zrbgo_exchange_jwt_capture "${z_jwt}") || return 1

  printf '%s\n' "${z_token}"
}

# Bound the post-write race where Google's auth backend has not yet accepted
# the freshly-minted SA key — die on timeout per pristine-tier contract.
rbgo_probe_jwt_bearer_propagation() {
  zrbgo_sentinel

  local -r z_rbra_file="${1:-}"
  local -r z_role_label="${2:-}"

  buc_doc_brief "Probe JWT-bearer mint until freshly-written RBRA is usable"
  buc_doc_param "rbra_file"  "Path to just-written RBRA credentials file"
  buc_doc_param "role_label" "Diagnostic label for timeout/success messages"
  buc_doc_shown || return 0

  test -n "${z_rbra_file}"  || buc_die "rbra_file required"
  test -n "${z_role_label}" || buc_die "role_label required"
  test -f "${z_rbra_file}"  || buc_die "RBRA file not found: ${z_rbra_file}"

  buc_step "JWT-bearer propagation probe [${z_role_label}]: budget ${RBGC_SA_KEY_PROBE_BUDGET_SEC}s"

  local z_attempt=0
  local z_elapsed=0
  local z_delay="${RBGC_SA_KEY_PROBE_INITIAL_DELAY_SEC}"
  local z_token=""
  local z_status=0

  while :; do
    z_attempt=$((z_attempt + 1))

    z_token=""
    z_status=0
    z_token=$(rbgo_get_token_capture "${z_rbra_file}" 2>"${ZRBGO_PROBE_STDERR_FILE}") || z_status=$?

    if test "${z_status}" -eq 0 && test -n "${z_token}"; then
      buc_step "JWT-bearer probe [${z_role_label}]: OK after ${z_attempt} attempt(s), ${z_elapsed}s elapsed"
      return 0
    fi

    test "${z_elapsed}" -lt "${RBGC_SA_KEY_PROBE_BUDGET_SEC}" \
      || buc_die "JWT-bearer probe [${z_role_label}]: timeout after ${z_elapsed}s (${z_attempt} attempts) — RBRA at ${z_rbra_file} not mintable; last stderr: ${ZRBGO_PROBE_STDERR_FILE}"

    buc_log_args "JWT-bearer probe [${z_role_label}]: attempt ${z_attempt} failed at ${z_elapsed}s, sleep ${z_delay}s"
    sleep "${z_delay}"
    z_elapsed=$((z_elapsed + z_delay))
    z_delay=$((z_delay * 2))
    test "${z_delay}" -le "${RBGC_SA_KEY_PROBE_MAX_DELAY_SEC}" || z_delay="${RBGC_SA_KEY_PROBE_MAX_DELAY_SEC}"
  done
}

# eof
