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
# Recipe Bottle Utility HTTP - JSON REST, polling, and shared temp-file machinery

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBUH_SOURCED:-}" || buc_die "Module rbuh multiply sourced - check sourcing hierarchy"
ZRBUH_SOURCED=1

######################################################################
# Internal Functions (zrbuh_*)

zrbuh_kindle() {
  test -z "${ZRBUH_KINDLED:-}" || buc_die "Module rbuh already kindled"

  buv_dir_exists "${BURD_TEMP_DIR}"

  # Ensure dependencies kindled first
  zrbgc_sentinel
  zrbgo_sentinel

  # Module prefix for temp files (rbuh owns the shared HTTP temp-file machinery)
  readonly ZRBUH_PREFIX="${BURD_TEMP_DIR}/rbuh_"

  # Infix postfixes for HTTP operations
  readonly ZRBUH_POSTFIX_JSON="_u_resp.json"
  readonly ZRBUH_POSTFIX_CODE="_u_code.txt"

  # Validate eventual consistency settings from rbgc
  test -n "${RBGC_EVENTUAL_CONSISTENCY_SEC:-}" || buc_die "RBGC_EVENTUAL_CONSISTENCY_SEC unset"
  test -n "${RBGC_MAX_CONSISTENCY_SEC:-}"      || buc_die "RBGC_MAX_CONSISTENCY_SEC unset"

  readonly ZRBUH_KINDLED=1
}

zrbuh_sentinel() {
  test "${ZRBUH_KINDLED:-}" = "1" || buc_die "Module rbuh not kindled - call zrbuh_kindle first"
}

######################################################################
# Predicate Functions

rbuh_json_valid_predicate() {
  zrbuh_sentinel
  local -r z_infix="${1:-}"
  test -n "${z_infix}" || return 1

  local -r z_json_file="${ZRBUH_PREFIX}${z_infix}${ZRBUH_POSTFIX_JSON}"
  test -f "${z_json_file}" || return 1

  jq -e . "${z_json_file}" >/dev/null 2>&1
}

# Boolean: true (0) when the captured HTTP code for an infix is a 2xx success
# (200/201/204 — the same success set rbuh_require_ok accepts). Unlike
# rbuh_require_ok it does not buc_die on failure; callers branch on the result.
rbuh_code_ok_predicate() {
  zrbuh_sentinel
  local -r z_infix="${1:-}"
  test -n "${z_infix}" || return 1

  local z_code
  z_code=$(rbuh_code_capture "${z_infix}") || return 1
  case "${z_code}" in
    200|201|204) return 0 ;;
    *)           return 1 ;;
  esac
}

######################################################################
# Capture Functions

rbuh_urlencode_capture() {
  zrbuh_sentinel
  local z_s="${1:-}"
  local z_out=""
  local z_i=0
  local z_c
  local z_hex

  buc_log_args "Percent encoding -> ${z_s}"

  while test ${z_i} -lt ${#z_s}; do
    z_c="${z_s:z_i:1}"
    case "${z_c}" in
      [A-Za-z0-9._~-]) z_out="${z_out}${z_c}" ;;
      *) printf -v z_hex '%%%02X' "'${z_c}"; z_out="${z_out}${z_hex}" ;;
    esac
    z_i=$((z_i + 1))
  done

  buc_log_args "Encoded ${z_out}"
  test -n "${z_out}" || return 1
  echo "${z_out}"
}

rbuh_json_field_capture() {
  zrbuh_sentinel
  local -r z_infix="${1}"
  local -r z_jq="${2}"
  local -r z_json_file="${ZRBUH_PREFIX}${z_infix}${ZRBUH_POSTFIX_JSON}"
  local z_result
  z_result=$(jq -r "${z_jq}" "${z_json_file}")          || return 1
  if test -z "${z_result}" || test "${z_result}" = "null"; then return 1; fi
  echo "${z_result}"
}

rbuh_code_capture() {
  zrbuh_sentinel
  local -r z_infix="${1}"
  local -r z_file="${ZRBUH_PREFIX}${z_infix}${ZRBUH_POSTFIX_CODE}"
  local z_code
  z_code=$(<"${z_file}") || return 1
  test -n "${z_code}"    || return 1
  echo "${z_code}"
}

# Apply jq filter to file, writing result to same or different file
rbuh_jq_file_to_file_ok() {
  zrbuh_sentinel
  local -r z_source_infix="${1:-}"
  local -r z_target_infix="${2:-}"
  local -r z_jq_filter="${3:-}"

  test -n "${z_source_infix}" || return 1
  test -n "${z_target_infix}" || return 1
  test -n "${z_jq_filter}"    || return 1

  local -r z_source_file="${ZRBUH_PREFIX}${z_source_infix}${ZRBUH_POSTFIX_JSON}"
  local -r z_target_file="${ZRBUH_PREFIX}${z_target_infix}${ZRBUH_POSTFIX_JSON}"

  test -f "${z_source_file}" || return 1

  jq "${z_jq_filter}" "${z_source_file}" > "${z_target_file}" || return 1
  test -f "${z_target_file}" || return 1
}

######################################################################
# External Functions (rbuh_*)

# JSON REST helper (hardcoded headers)
# Args: method url token infix [body_file]
# Pass "-" as body_file to read body from stdin (curl @- convention).
# Use this for request bodies containing secrets to avoid disk persistence.
rbuh_json() {
  zrbuh_sentinel

  local -r z_method="${1}"
  local -r z_url="${2}"
  local -r z_token="${3}"
  local -r z_infix="${4}"
  local -r z_body_file="${5:-}"

  local z_curl_status=0
  local z_attempt=0

  while :; do
    z_curl_status=0
    z_attempt=$((z_attempt + 1))

    # Each attempt writes to uniquely-suffixed files; callers use z_infix_u
    z_infix_u="${z_infix}_${z_attempt}"
    local z_resp_file="${ZRBUH_PREFIX}${z_infix_u}${ZRBUH_POSTFIX_JSON}"
    local z_code_file="${ZRBUH_PREFIX}${z_infix_u}${ZRBUH_POSTFIX_CODE}"
    local z_code_errs="${ZRBUH_PREFIX}${z_infix_u}${ZRBUH_POSTFIX_CODE}.stderr"

    if test -n "${z_body_file}"; then
      curl                                              \
          -sS                                           \
          --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
          --max-time "${RBCC_CURL_MAX_TIME_SEC}"        \
          -X "${z_method}"                              \
          -H "Authorization: Bearer ${z_token}"         \
          -H "Content-Type: application/json"           \
          -H "Accept: application/json"                 \
          -d @"${z_body_file}"                          \
          -o "${z_resp_file}"                           \
          -w "%{http_code}"                             \
          "${z_url}" > "${z_code_file}"                 \
                    2> "${z_code_errs}"                 \
        || z_curl_status=$?
    else
      curl                                              \
          -sS                                           \
          --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
          --max-time "${RBCC_CURL_MAX_TIME_SEC}"        \
          -X "${z_method}"                              \
          -H "Authorization: Bearer ${z_token}"         \
          -H "Content-Type: application/json"           \
          -H "Accept: application/json"                 \
          -o "${z_resp_file}"                           \
          -w "%{http_code}"                             \
          "${z_url}" > "${z_code_file}"                 \
                    2> "${z_code_errs}"                 \
        || z_curl_status=$?
    fi

    buc_log_args 'Curl status' "${z_curl_status}"
    buc_log_pipe < "${z_code_errs}"

    # Success — break out of retry loop
    test "${z_curl_status}" -ne 0 || break

    rbgo_curl_status_is_transient_predicate "${z_curl_status}" \
      || buc_die "HTTP request failed (curl exit ${z_curl_status})"

    test "${z_attempt}" -lt "${RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS}" \
      || buc_die "HTTP request failed after ${RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS} attempts (curl exit ${z_curl_status})"

    buc_log_args "Transient curl error (exit ${z_curl_status}), retry ${z_attempt}/${RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS} in ${RBGC_HTTP_TRANSIENT_RETRY_SLEEP_SEC}s"
    sleep "${RBGC_HTTP_TRANSIENT_RETRY_SLEEP_SEC}"
  done

  # Register successful attempt under bare infix for capture functions
  cp "${ZRBUH_PREFIX}${z_infix_u}${ZRBUH_POSTFIX_JSON}" "${ZRBUH_PREFIX}${z_infix}${ZRBUH_POSTFIX_JSON}"
  cp "${ZRBUH_PREFIX}${z_infix_u}${ZRBUH_POSTFIX_CODE}" "${ZRBUH_PREFIX}${z_infix}${ZRBUH_POSTFIX_CODE}"

  local z_code
  z_code=$(<"${z_code_file}") || buc_die "Failed to read code file"
  test -n "${z_code}"         || buc_die "Empty HTTP code from curl"

  buc_log_args "HTTP ${z_method} ${z_url} returned code ${z_code}"
}

# Single HTTP request — perform curl with stderr capture, return rc.
# Args: method url token resp_file code_file stderr_file [body_file]
# Returns: 0 on success, curl exit rc on failure (no buc_die — caller decides policy).
# Hardcoded headers: Authorization: Bearer ${token}, Accept: application/json.
# When body_file is non-empty: adds Content-Type: application/json, sends @body_file.
# Pass "-" as body_file to read body from stdin (curl @- convention).
# This is the BCG-conformant primitive — caller supplies temp file paths so per-attempt
# numbering (BCG §"In loops, use an auto-incrementing integer") is the caller's choice.
rbuh_request() {
  zrbuh_sentinel

  local -r z_method="${1}"
  local -r z_url="${2}"
  local -r z_token="${3}"
  local -r z_resp_file="${4}"
  local -r z_code_file="${5}"
  local -r z_stderr_file="${6}"
  local -r z_body_file="${7:-}"

  local z_curl_status=0
  if test -n "${z_body_file}"; then
    curl                                                     \
        -sS                                                  \
        --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
        --max-time "${RBCC_CURL_MAX_TIME_SEC}"               \
        -X "${z_method}"                                     \
        -H "Authorization: Bearer ${z_token}"                \
        -H "Content-Type: application/json"                  \
        -H "Accept: application/json"                        \
        -d @"${z_body_file}"                                 \
        -o "${z_resp_file}"                                  \
        -w "%{http_code}"                                    \
        "${z_url}" > "${z_code_file}"                        \
                  2> "${z_stderr_file}"                      \
      || z_curl_status=$?
  else
    curl                                                     \
        -sS                                                  \
        --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
        --max-time "${RBCC_CURL_MAX_TIME_SEC}"               \
        -X "${z_method}"                                     \
        -H "Authorization: Bearer ${z_token}"                \
        -H "Accept: application/json"                        \
        -o "${z_resp_file}"                                  \
        -w "%{http_code}"                                    \
        "${z_url}" > "${z_code_file}"                        \
                  2> "${z_stderr_file}"                      \
      || z_curl_status=$?
  fi

  return "${z_curl_status}"
}

rbuh_require_ok() {
  zrbuh_sentinel
  local -r z_ctx="$1"
  local -r z_infix="$2"
  local -r z_warn_code="${3:-}"
  local -r z_warn_message="${4:-already exists}"

  local z_code
  z_code=$(rbuh_code_capture "${z_infix}") \
    || buc_die "${z_ctx}: failed to read HTTP code"

  case "${z_code}" in
    200|201|204) return 0 ;;
  esac

  if test -n "${z_warn_code}" && test "${z_code}" = "${z_warn_code}"; then
    buc_warn "${z_ctx}: ${z_warn_message}"
    return 0
  fi

  local z_err=""
  local -r z_response_file="${ZRBUH_PREFIX}${z_infix}${ZRBUH_POSTFIX_JSON}"

  if jq -e . "${z_response_file}" >/dev/null 2>&1; then
    z_err=$(rbuh_json_field_capture "${z_infix}" '.error.message') || z_err="Unknown error"
  else
    local z_content=$(<"${z_response_file}")
    test -n "${z_content}" || z_content=""
    z_err="${z_content:0:200}"
    z_err="${z_err//$'\n'/ }"
    z_err="${z_err//$'\r'/ }"
    test -n "${z_err}" || z_err="Non-JSON error body"
  fi

  buc_die "${z_ctx} (HTTP ${z_code}): ${z_err}"
}

# Poll endpoint until HTTP 200 (for IAM/resource propagation waits)
rbuh_poll_until_ok() {
  zrbuh_sentinel

  local -r z_label="${1}"
  local -r z_url="${2}"
  local -r z_token="${3}"
  local -r z_infix="${4}"

  local z_elapsed=0
  while :; do
    local z_poll_infix="${z_infix}-${z_elapsed}s"
    rbuh_json "GET" "${z_url}" "${z_token}" "${z_poll_infix}" || true

    local z_code
    z_code=$(rbuh_code_capture "${z_poll_infix}") || z_code=""

    if test "${z_code}" = "200"; then
      buc_log_args "${z_label} ready after ${z_elapsed} seconds"
      return 0
    fi

    test "${z_elapsed}" -ge "${RBGC_MAX_CONSISTENCY_SEC}" && buc_die "${z_label}: timeout after ${RBGC_MAX_CONSISTENCY_SEC}s"
    buc_log_args "${z_label} not ready (HTTP ${z_code}), waiting ${RBGC_EVENTUAL_CONSISTENCY_SEC}s..."
    sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
    z_elapsed=$((z_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
  done
}

# Inverse of rbuh_poll_until_ok: poll a GET endpoint until it returns HTTP 404.
# Used after a DELETE to confirm the resource is gone (not merely delete-accepted)
# before a same-name recreate — absorbs the seconds-scale deletion-propagation
# race. A GET still returning 200 means the deletion has not yet propagated.
rbuh_poll_until_gone() {
  zrbuh_sentinel

  local -r z_label="${1}"
  local -r z_url="${2}"
  local -r z_token="${3}"
  local -r z_infix="${4}"

  local z_elapsed=0
  while :; do
    local z_poll_infix="${z_infix}-${z_elapsed}s"
    rbuh_json "GET" "${z_url}" "${z_token}" "${z_poll_infix}" || true

    local z_code
    z_code=$(rbuh_code_capture "${z_poll_infix}") || z_code=""

    if test "${z_code}" = "404"; then
      buc_log_args "${z_label} gone after ${z_elapsed} seconds"
      return 0
    fi

    test "${z_elapsed}" -ge "${RBGC_MAX_CONSISTENCY_SEC}" && buc_die "${z_label}: still present after ${RBGC_MAX_CONSISTENCY_SEC}s"
    buc_log_args "${z_label} still present (HTTP ${z_code}), waiting ${RBGC_EVENTUAL_CONSISTENCY_SEC}s..."
    sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
    z_elapsed=$((z_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
  done
}

# eof
