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
  readonly ZRBGO_OPENSSL_STDERR_FILE="${BURD_TEMP_DIR}/rbgo_openssl_stderr.txt"
  readonly ZRBGO_CURL_STDERR_FILE="${BURD_TEMP_DIR}/rbgo_curl_stderr.txt"
  readonly ZRBGO_JQ_STDERR_FILE="${BURD_TEMP_DIR}/rbgo_jq_stderr.txt"
  readonly ZRBGO_CONSUMER_RETRY_BODY_FILE="${BURD_TEMP_DIR}/rbgo_consumer_retry_body.json"

  readonly ZRBGO_KINDLED=1
}

zrbgo_sentinel() {
  test "${ZRBGO_KINDLED:-}" = "1" || buc_die "Module rbgo not kindled - call zrbgo_kindle first"
}

######################################################################
# Base64 primitives — generic openssl wrappers.
#
# Stateless — no sentinel; safe to call from any module regardless of
# kindle order. rbgo base64url-encodes JWT material; rbgg/rbgp decode SA
# private keys and rbfd decodes Cloud Build step outputs. The -A flag
# (suppress line wrapping) is load-bearing on every site and lives only
# here, so it cannot be silently dropped on one path.

rbgo_base64_decode_string_to_file() {
  local -r z_b64="${1:-}"
  local -r z_output="${2:-}"
  test -n "${z_b64}"    || return 1
  test -n "${z_output}" || return 1
  printf '%s' "${z_b64}" | openssl enc -base64 -d -A > "${z_output}" || return 1
}

rbgo_base64_decode_file_to_file() {
  local -r z_input="${1:-}"
  local -r z_output="${2:-}"
  test -n "${z_input}"  || return 1
  test -n "${z_output}" || return 1
  test -f "${z_input}"  || return 1
  openssl enc -base64 -d -A < "${z_input}" > "${z_output}" || return 1
}

rbgo_base64_encode_string_capture() {
  local -r z_input="${1:-}"
  printf '%s' "${z_input}" | openssl enc -base64 -A
}

rbgo_base64_encode_file_capture() {
  local -r z_file="${1:-}"
  test -f "${z_file}" || return 1
  openssl enc -base64 -A < "${z_file}"
}

# Stateless — no sentinel; safe to call from any module regardless of kindle order.
rbgo_curl_status_is_transient_predicate() {
  case "${1:-}" in
    7|28|35|56) return 0 ;;
    *)          return 1 ;;
  esac
}

zrbgo_base64url_encode_capture() {
  zrbgo_sentinel

  local -r z_input="${1:-}"

  # Base64url: encode, then URL-safe transform and strip = padding
  local z_encoded
  z_encoded=$(rbgo_base64_encode_string_capture "${z_input}") || {
    buc_warn "openssl base64url encode failed"
    return 1
  }
  z_encoded="${z_encoded//+/-}"
  z_encoded="${z_encoded//\//_}"
  z_encoded="${z_encoded//=/}"
  printf '%s' "${z_encoded}"
}

zrbgo_build_jwt_capture() {
  zrbgo_sentinel

  local -r z_rbra_file="$1"

  # RBRA_* expected: CLIENT_EMAIL, PRIVATE_KEY, TOKEN_LIFETIME_SEC
  # RBRA_PRIVATE_KEY is sourced as a multi-line string (real newlines from PEM).
  # printf '%b' below is defensive and tolerates either real-newline or '\n'-escape
  # form; PEM keys contain no backslashes so %b is otherwise a no-op.
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
    "${ZRBGO_JWT_UNSIGNED_FILE}" 2>"${ZRBGO_OPENSSL_STDERR_FILE}" || {
      buc_warn "openssl RSA256 sign failed: $(<"${ZRBGO_OPENSSL_STDERR_FILE}")"
      return 1
    }

  buc_log_args "Base64url encode signature"
  local z_signature
  z_signature=$(rbgo_base64_encode_file_capture "${ZRBGO_JWT_SIGNATURE_FILE}") || {
    buc_warn "openssl signature base64 encode failed"
    return 1
  }
  z_signature="${z_signature//+/-}"
  z_signature="${z_signature//\//_}"
  z_signature="${z_signature//=/}"

  buc_log_args "Return complete JWT"
  printf '%s\n' "${z_header_enc}.${z_claims_enc}.${z_signature}"
}

zrbgo_exchange_jwt_capture() {
  zrbgo_sentinel

  local -r z_jwt="$1"

  local z_attempt=0
  local z_curl_status=0
  while :; do
    z_attempt=$((z_attempt + 1))

    buc_log_args "Exchange JWT for OAuth token (attempt ${z_attempt}/${RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS})"
    z_curl_status=0
    curl -sS -X POST "${RBGC_OAUTH_TOKEN_URL}"                                        \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}"                           \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}"                                         \
      -H "Content-Type: application/x-www-form-urlencoded"                           \
      -d "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion=${z_jwt}" \
      > "${ZRBGO_OAUTH_RESPONSE_FILE}" 2>"${ZRBGO_CURL_STDERR_FILE}" || z_curl_status=$?

    test "${z_curl_status}" -eq 0 && break

    buc_warn "OAuth curl exit ${z_curl_status}: $(<"${ZRBGO_CURL_STDERR_FILE}")"
    rbgo_curl_status_is_transient_predicate "${z_curl_status}" || return 1
    test "${z_attempt}" -lt "${RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS}" || return 1

    buc_step "OAuth token mint: transient curl ${z_curl_status}, retry ${z_attempt}/${RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS} in ${RBGC_HTTP_TRANSIENT_RETRY_SLEEP_SEC}s"
    sleep "${RBGC_HTTP_TRANSIENT_RETRY_SLEEP_SEC}"
  done

  # Scrubber filters by field NAME, not value. Deliberate best-effort log hygiene:
  # explicitly deletes access_token/refresh_token, then drops any field whose key
  # matches token|secret|key|password (case-insensitive) — catches id_token,
  # client_secret, etc. If the OAuth provider ever returns a new secret-carrying
  # field whose name doesn't match this regex, the scrub would miss it; update
  # the regex here when that happens.
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
      || printf '%s\n' "$(<"${ZRBGO_OAUTH_RESPONSE_FILE}")" >&2
    return 1
  }

  printf '%s\n' "${z_token}"
}

######################################################################
# External Functions (rbgo_*)

rbgo_get_token_capture() {
  zrbgo_sentinel

  local -r z_rbra_file="$1"

  buc_doc_brief "Exchange service account credentials for OAuth2 access token (retries on JWT-propagation race)"
  buc_doc_param "rbra_file" "Path to RBRA credentials file containing RBRA_* variables"
  buc_doc_shown || return 0

  buc_log_args "Validate RBRA file exists"
  test -f "${z_rbra_file}" || return 1

  local z_attempt=0
  local z_elapsed=0
  local z_delay="${RBGC_SA_KEY_CONSUMER_RETRY_INITIAL_DELAY_SEC}"
  local z_jwt=""
  local z_token=""
  local z_status=0
  local z_err=""
  local z_err_desc=""

  while :; do
    z_attempt=$((z_attempt + 1))

    z_status=0
    z_jwt=$(zrbgo_build_jwt_capture "${z_rbra_file}") || z_status=$?
    test "${z_status}" -eq 0 || return 1

    z_status=0
    z_token=$(zrbgo_exchange_jwt_capture "${z_jwt}") || z_status=$?

    if test "${z_status}" -eq 0 && test -n "${z_token}"; then
      printf '%s\n' "${z_token}"
      return 0
    fi

    # Discriminate failure: only the SA-propagation race shapes retry —
    #   `invalid_grant` + `Invalid JWT Signature.`         (fresh-key propagation lag)
    #   `invalid_grant` + `Invalid grant: account not found` (fresh-SA propagation lag)
    # Any other shape fails fast.
    z_err=$(jq -r '.error // ""'             "${ZRBGO_OAUTH_RESPONSE_FILE}" 2>"${ZRBGO_JQ_STDERR_FILE}") || z_err=""
    z_err_desc=$(jq -r '.error_description // ""' "${ZRBGO_OAUTH_RESPONSE_FILE}" 2>"${ZRBGO_JQ_STDERR_FILE}") || z_err_desc=""

    test "${z_err}" = "invalid_grant" || return 1
    case "${z_err_desc}" in
      "Invalid JWT Signature."|"Invalid grant: account not found") ;;
      *) return 1 ;;
    esac

    cp "${ZRBGO_OAUTH_RESPONSE_FILE}" "${ZRBGO_CONSUMER_RETRY_BODY_FILE}" || return 1

    if test "${z_elapsed}" -ge "${RBGC_SA_KEY_CONSUMER_RETRY_BUDGET_SEC}"; then
      buc_warn "Token mint: SA propagation race timeout after ${z_elapsed}s (${z_attempt} attempts); last body: ${ZRBGO_CONSUMER_RETRY_BODY_FILE}"
      return 1
    fi

    buc_step "Token mint: attempt ${z_attempt} hit SA propagation race (${z_err_desc}) at ${z_elapsed}s, sleep ${z_delay}s"
    sleep "${z_delay}"
    z_elapsed=$((z_elapsed + z_delay))
    z_delay=$((z_delay * 2))
    test "${z_delay}" -le "${RBGC_SA_KEY_CONSUMER_RETRY_MAX_DELAY_SEC}" || z_delay="${RBGC_SA_KEY_CONSUMER_RETRY_MAX_DELAY_SEC}"
  done
}

# Authenticate the host docker client to a GAR registry with bounded retry on
# the moby/moby#44350 premature-timeout transient (see
# RBGC_DOCKER_LOGIN_TRANSIENT_SIGNATURE). docker login is the lone login/pull/push
# verb with no internal retry and a hardcoded 15s daemon->registry auth timeout;
# against a healthy-but-slow backend it fails where the endpoint is in fact fine.
# This mirrors the curl-transient tolerance in rbgu_http_json: retry only the
# surveyed signature, fail fast on everything else (real auth failures emit
# "unauthorized" and do not match), so clean-failure semantics are preserved.
# Stateless (no sentinel) — safe to call from any module regardless of kindle
# order, like rbgu_curl_status_is_transient_predicate.
# Args: token registry_host
rbgo_docker_login() {
  local -r z_token="${1:?rbgo_docker_login: token required}"
  local -r z_host="${2:?rbgo_docker_login: registry host required}"

  local z_attempt=0
  local z_stderr_file=""
  local z_rc=0

  while :; do
    z_attempt=$((z_attempt + 1))
    z_stderr_file="${BURD_TEMP_DIR}/rbgo_docker_login_${z_attempt}_stderr.txt"

    z_rc=0
    printf '%s' "${z_token}" \
      | docker login -u oauth2accesstoken --password-stdin "https://${z_host}" \
          > /dev/null 2>"${z_stderr_file}" \
      || z_rc=$?

    test "${z_rc}" -ne 0 || break

    buc_log_pipe < "${z_stderr_file}"

    [[ "$(<"${z_stderr_file}")" == *"${RBGC_DOCKER_LOGIN_TRANSIENT_SIGNATURE}"* ]] \
      || buc_die "Docker login to ${z_host} failed — see ${z_stderr_file}"

    test "${z_attempt}" -lt "${RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS}" \
      || buc_die "Docker login to ${z_host} failed after ${RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS} attempts (transient daemon->registry timeout, moby#44350) — see ${z_stderr_file}"

    buc_warn "Docker login transient (attempt ${z_attempt}/${RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS}, moby#44350 timeout) — retrying in ${RBGC_HTTP_TRANSIENT_RETRY_SLEEP_SEC}s"
    sleep "${RBGC_HTTP_TRANSIENT_RETRY_SLEEP_SEC}"
  done
}

# eof
