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
# Recipe Bottle Google Utility - Implementation

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGU_SOURCED:-}" || bcu_die "Module rbgu multiply sourced - check sourcing hierarchy"
ZRBGU_SOURCED=1

######################################################################
# Internal Functions (zrbgu_*)

zrbgu_kindle() {
  test -z "${ZRBGU_KINDLED:-}" || bcu_die "Module rbgu already kindled"

  # Validate dependencies
  bvu_dir_exists "${BDU_TEMP_DIR}"

  # Ensure dependencies kindled first
  zrbgc_sentinel
  zrbgo_sentinel

  # Module prefix for temp files
  ZRBGU_PREFIX="${BDU_TEMP_DIR}/rbgu_"
  ZRBGU_EMPTY_JSON="${ZRBGU_PREFIX}empty.json"
  printf '{}' > "${ZRBGU_EMPTY_JSON}"

  # Infix values for HTTP operations
  ZRBGU_POSTFIX_JSON="_u_resp.json"
  ZRBGU_POSTFIX_CODE="_u_code.txt"

  # Validate eventual consistency settings from rbgc
  test -n "${RBGC_EVENTUAL_CONSISTENCY_SEC:-}" || bcu_die "RBGC_EVENTUAL_CONSISTENCY_SEC unset"
  test -n "${RBGC_MAX_CONSISTENCY_SEC:-}"      || bcu_die "RBGC_MAX_CONSISTENCY_SEC unset"

  ZRBGU_KINDLED=1
}

zrbgu_sentinel() {
  test "${ZRBGU_KINDLED:-}" = "1" || bcu_die "Module rbgu not kindled - call zrbgu_kindle first"
}

######################################################################
# Predicate Functions

rbgu_json_valid_predicate() {
  zrbgu_sentinel
  local z_infix="${1:-}"
  test -n "${z_infix}" || return 1

  local z_json_file="${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_JSON}"
  test -f "${z_json_file}" || return 1

  jq -e . "${z_json_file}" >/dev/null 2>&1
}

rbgu_role_member_exists_predicate() {
  zrbgu_sentinel
  local z_infix="${1:-}"
  local z_role="${2:-}"
  local z_member="${3:-}"

  test -n "${z_infix}" || return 1
  test -n "${z_role}"  || return 1
  test -n "${z_member}" || return 1

  local z_json_file="${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_JSON}"
  test -f "${z_json_file}" || return 1

  jq -e --arg r "${z_role}" --arg m "${z_member}" \
    '.bindings[]? | select(.role==$r) | (.members // [])[]? == $m' \
    "${z_json_file}" >/dev/null 2>&1
}

######################################################################
# Capture Functions

rbgu_error_message_capture() {
  zrbgu_sentinel
  local z_infix="${1:-}"
  test -n "${z_infix}" || return 1

  if rbgu_json_valid_predicate "${z_infix}"; then
    rbgu_json_field_capture "${z_infix}" '.error.message' 2>/dev/null || return 1
  else
    return 1
  fi
}

rbgu_urlencode_capture() {
  zrbgu_sentinel
  local z_s="${1:-}"
  local z_out=""
  local z_i=0
  local z_c
  local z_hex

  bcu_log_args "Percent encoding -> ${z_s}"

  while test ${z_i} -lt ${#z_s}; do
    z_c="${z_s:z_i:1}"
    case "${z_c}" in
      [A-Za-z0-9._~\-]) z_out="${z_out}${z_c}" ;;
      *) printf -v z_hex '%%%02X' "'${z_c}"; z_out="${z_out}${z_hex}" ;;
    esac
    z_i=$((z_i + 1))
  done

  bcu_log_args "Encoded ${z_out}"
  test -n "${z_out}" || return 1
  echo "${z_out}"
}

# Add member to IAM policy role binding with version=3 enforcement
#
# RBGU IAM Policy Standard: All IAM policies are standardized to version=3
# to ensure consistent conditional role binding support across Google Cloud APIs.
# This is enforced by default in all policy operations to prevent version drift.
#
# Args: infix role member [etag_optional]
# Returns: JSON policy string with added member and version=3
rbgu_jq_add_member_to_role_capture() {
  zrbgu_sentinel

  local z_infix="${1:-}"
  local z_role="${2:-}"
  local z_member="${3:-}"
  local z_etag_opt="${4:-}"

  local z_policy_file="${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_JSON}"

  test -n "${z_policy_file}" || return 1
  test -f "${z_policy_file}" || return 1
  test -n "${z_role}"        || return 1
  test -n "${z_member}"      || return 1

  local z_out=""
  z_out=$(
    jq --arg role "${z_role}" --arg member "${z_member}" --arg etag "${z_etag_opt}" '
      # Enforce RBGU standard: version=3 for all IAM policies
      .version = 3 |
      .bindings = (.bindings // []) |
      if ([.bindings[]? | .role] | index($role))
      then .bindings |= map(if .role == $role
                            then .members = ((.members // []) + [$member] | unique)
                            else . end)
      else .bindings += [{role: $role, members: [$member]}]
      end
      # Set etag if provided (optimistic concurrency)
      | (if $etag != "" then .etag = $etag else . end)
    ' "${z_policy_file}"
  ) || return 1

  test -n "${z_out}" || return 1
  printf '%s\n' "${z_out}"
}

rbgu_json_field_capture() {
  zrbgu_sentinel
  local z_infix="${1}"
  local z_jq="${2}"
  local z_json_file="${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_JSON}"
  local z_result
  z_result=$(jq -r "${z_jq}" "${z_json_file}")          || return 1
  test -n "${z_result}" && test "${z_result}" != "null" || return 1
  echo "${z_result}"
}

rbgu_http_code_capture() {
  zrbgu_sentinel
  local z_infix="${1}"
  local z_file="${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_CODE}"
  local z_code
  z_code=$(<"${z_file}") || return 1
  test -n "${z_code}"    || return 1
  echo "${z_code}"
}

rbgu_get_admin_token_capture() {
  zrbgu_sentinel

  # Need access to RBRR_ADMIN_RBRA_FILE from regime
  test -n "${RBRR_ADMIN_RBRA_FILE:-}" || return 1

  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_ADMIN_RBRA_FILE}") || return 1

  test -n "${z_token}" || return 1
  echo    "${z_token}"
}

# Poll a Google LRO until done (success or error)
rbgu_wait_lro_capture() {
  zrbgu_sentinel

  local z_token="${1:-}"
  local z_op_url="${2:-}"
  local z_parent="${3:-}"
  local z_poll="${4:-${RBGC_EVENTUAL_CONSISTENCY_SEC}}"
  local z_max="${5:-${RBGC_MAX_CONSISTENCY_SEC}}"

  test -n "${z_token}"   || return 1
  test -n "${z_op_url}"  || return 1
  test -n "${z_parent}"  || return 1
  test -n "${z_poll}"    || return 1
  test -n "${z_max}"     || return 1

  local z_elapsed=0
  local z_infix=""
  local z_done=""

  while :; do
    z_infix="${z_parent}-lro-${z_elapsed}s"

    rbgu_http_json "GET" "${z_op_url}" "${z_token}" "${z_infix}"

    bcu_log_args 'If HTTP not 200, treat as transient unless clearly fatal'
    local z_code=""
    z_code=$(rbgu_http_code_capture "${z_infix}") || return 1
    case "${z_code}" in
      200)                     :                                                                    ;;
      429|408|500|502|503|504) bcu_log_args "HTTP ${z_code} at ${z_elapsed}s LRO transient "        ;;
      *)                       bcu_log_args "HTTP ${z_code} at ${z_elapsed}s LRO non_OK "; return 1 ;;
    esac

    z_done=$(rbgu_json_field_capture "${z_infix}" ".done" 2>/dev/null) || z_done=""
    if test "${z_done}" = "true"; then
      bcu_log_args 'Error?'
      local z_err_msg=""
      z_err_msg=$(rbgu_json_field_capture "${z_infix}" ".error.message" 2>/dev/null) || z_err_msg=""
      if test -n "${z_err_msg}"; then
        bcu_log_args "LRO error at ${z_elapsed}s: ${z_err_msg}"
        return 1
      fi
      bcu_log_args 'Success'
      echo "${z_infix}"
      return 0
    fi

    bcu_log_args 'Not done: check timeout and sleep'
    if test "${z_elapsed}" -ge "${z_max}"; then
      bcu_log_args "LRO timeout at ${z_elapsed}s (max ${z_max}s)"
      return 1
    fi

    bcu_log_args 'Clamp poll interval >= 1 and not overshooting max too far'
    test  "${z_poll}" -ge 1 || z_poll=1
    sleep "${z_poll}"
    z_elapsed=$((z_elapsed + z_poll))
  done
}

# JSON file writer helper (vanilla empty policy)
rbgu_write_vanilla_json() {
  zrbgu_sentinel
  local z_infix="${1:-}"
  test -n "${z_infix}" || return 1

  local z_json_file="${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_JSON}"
  printf '{"bindings":[]}\n' > "${z_json_file}" || return 1
  test -f "${z_json_file}" || return 1
}

# Apply jq filter to file, writing result to same or different file
rbgu_jq_file_to_file_ok() {
  zrbgu_sentinel
  local z_source_infix="${1:-}"
  local z_target_infix="${2:-}"
  local z_jq_filter="${3:-}"

  test -n "${z_source_infix}" || return 1
  test -n "${z_target_infix}"  || return 1
  test -n "${z_jq_filter}"     || return 1

  local z_source_file="${ZRBGU_PREFIX}${z_source_infix}${ZRBGU_POSTFIX_JSON}"
  local z_target_file="${ZRBGU_PREFIX}${z_target_infix}${ZRBGU_POSTFIX_JSON}"

  test -f "${z_source_file}" || return 1

  jq "${z_jq_filter}" "${z_source_file}" > "${z_target_file}" || return 1
  test -f "${z_target_file}" || return 1
}

######################################################################
# External Functions (rbgu_*)

# JSON REST helper (hardcoded headers)
rbgu_http_json() {
  zrbgu_sentinel

  local z_method="${1}"
  local z_url="${2}"
  local z_token="${3}"
  local z_infix="${4}"
  local z_body_file="${5:-}"

  local z_resp_file="${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_JSON}"
  local z_code_file="${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_CODE}"
  local z_code_errs="${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_CODE}.stderr"

  local z_curl_status=0

  if test -n "${z_body_file}"; then
    curl                                              \
        -sS                                           \
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

  bcu_log_args 'Curl status' "${z_curl_status}"
  bcu_log_pipe < "${z_code_errs}"

  test "${z_curl_status}" -eq 0 || bcu_die "HTTP request failed (network/SSL/DNS)"

  local z_code
  z_code=$(<"${z_code_file}") || bcu_die "Failed to read code file"
  test -n "${z_code}"         || bcu_die "Empty HTTP code from curl"

  bcu_log_args "HTTP ${z_method} ${z_url} returned code ${z_code}"
}

rbgu_http_require_ok() {
  zrbgu_sentinel
  local z_ctx="$1"
  local z_infix="$2"
  local z_warn_code="${3:-}"
  local z_warn_message="${4:-already exists}"

  local z_code
  z_code=$(rbgu_http_code_capture "${z_infix}") \
    || bcu_die "${z_ctx}: failed to read HTTP code"

  case "${z_code}" in
    200|201|204) return 0 ;;
  esac

  if test -n "${z_warn_code}" && test "${z_code}" = "${z_warn_code}"; then
    bcu_warn "${z_ctx}: ${z_warn_message}"
    return 0
  fi

  local z_err=""
  local z_response_file="${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_JSON}"

  if jq -e . "${z_response_file}" >/dev/null 2>&1; then
    z_err=$(rbgu_json_field_capture "${z_infix}" '.error.message') || z_err="Unknown error"
  else
    local z_content=$(<"${z_response_file}")
    test -n "${z_content}" || z_content=""
    z_err="${z_content:0:200}"
    z_err="${z_err//$'\n'/ }"
    z_err="${z_err//$'\r'/ }"
    test -n "${z_err}" || z_err="Non-JSON error body"
  fi

  bcu_die "${z_ctx} (HTTP ${z_code}): ${z_err}"
}

rbgu_http_json_ok() {
  zrbgu_sentinel

  local z_label="$1"
  local z_token="$2"
  local z_method="$3"
  local z_url="$4"
  local z_infix="$5"
  local z_body_file="$6"
  local z_warn_code="${7:-}"
  local z_warn_message="${8:-}"

  bcu_log_args "${z_label}"

  # Perform HTTP request (body file may be "")
  rbgu_http_json "${z_method}" "${z_url}" "${z_token}" "${z_infix}" "${z_body_file:-}"

  # Enforce success (warn/ignore if optional params provided)
  rbgu_http_require_ok "${z_label}" "${z_infix}" "${z_warn_code:-}" "${z_warn_message:-}"
}

# POST + strict LRO handling (no heuristics)
rbgu_http_json_lro_ok() {
  zrbgu_sentinel

  local z_label="${1}"
  local z_token="${2}"
  local z_post_url="${3}"
  local z_infix="${4}"
  local z_body="${5}"
  local z_name_jq="${6}"
  local z_poll_root="${7}"
  local z_op_prefix="${8}"
  local z_poll_interval="${9:-${RBGC_EVENTUAL_CONSISTENCY_SEC}}"
  local z_timeout="${10:-${RBGC_MAX_CONSISTENCY_SEC}}"

  bcu_log_args '1) POST the request'
  rbgu_http_json "POST" "${z_post_url}" "${z_token}" "${z_infix}" "${z_body}"
  rbgu_http_require_ok "${z_label}" "${z_infix}"

  local z_done=""
  z_done=$(rbgu_json_field_capture "${z_infix}" ".done") || z_done=""
  test "${z_done}" = "true" && {
    bcu_log_args 'Immediate-done response -> success (no polling)'
    return 0
  }

  bcu_log_args '2) Extract op name (or return if not an LRO)'
  local z_name
  z_name=$(rbgu_json_field_capture "${z_infix}" "${z_name_jq}") || z_name=""
  test -n "${z_name}" || {
    bcu_log_args 'No LRO name present - treat as non-LRO success'
    return 0
  }
  bcu_log_args '3) Build poll URL based on operation name format'
  local z_poll_url=""
  if [[ "${z_name}" =~ ^projects/.*/locations/.*/operations/ ]]; then
    bcu_log_args '  Regional operation with fully-qualified name'
    z_poll_url="${z_poll_root}/${z_name}"
  elif [[ "${z_name}" =~ ^projects/.*/operations/ ]]; then
    bcu_log_args '  Global operation with project prefix'
    z_poll_url="${z_poll_root}/${z_name}"
  elif test -n "${z_op_prefix}" && [[ ! "${z_name}" =~ ^${z_op_prefix} ]]; then
    bcu_log_args '  Legacy format - apply prefix (not already present)'
    z_poll_url="${z_poll_root}/${z_op_prefix}${z_name}"
  else
    bcu_log_args '  Use name as-is under versioned root'
    z_poll_url="${z_poll_root}/${z_name}"
  fi
  bcu_log_args "Poll URL: ${z_poll_url}"

  bcu_log_args '4) Poll until done or timeout'
  local z_elapsed=0
  while :; do
    sleep "${z_poll_interval}"
    z_elapsed=$((z_elapsed + z_poll_interval))

    local z_poll_infix="${z_infix}-poll-${z_elapsed}s"
    rbgu_http_json "GET" "${z_poll_url}" "${z_token}" "${z_poll_infix}"

    local z_code=""
    z_code=$(rbgu_http_code_capture "${z_poll_infix}") || z_code=""
    test "${z_code}" = "200" || bcu_die "${z_label}: poll failed (HTTP ${z_code})"

    z_done=$(rbgu_json_field_capture "${z_poll_infix}" ".done") || z_done=""
    test "${z_done}" = "true" && {
      bcu_log_args "${z_label}: operation completed after ${z_elapsed}s"
      return 0
    }

    test "${z_elapsed}" -ge "${z_timeout}" && bcu_die "${z_label}: timeout after ${z_timeout}s"
    bcu_log_args "Still running at ${z_elapsed}s..."
  done
}

# Predicate: Check if resource was newly created and apply propagation delay
rbgu_newly_created_delay() {
  zrbgu_sentinel

  local z_infix="${1}"
  local z_resource="${2}"
  local z_delay="${3}"

  local z_code
  z_code=$(rbgu_http_code_capture "${z_infix}") || return 1

  if test "${z_code}" = "200" || test "${z_code}" = "201"; then
    bcu_step "Resource ${z_resource} newly created, waiting ${z_delay}s for propagation"
    sleep "${z_delay}"
  fi
}

rbgu_extract_json_to_rbra() {
  zrbgu_sentinel

  local z_json_path="$1"
  local z_rbra_path="$2"
  local z_lifetime_sec="$3"

  test -f "${z_json_path}" || bcu_die "Service account JSON not found: ${z_json_path}"

  bcu_info "Extracting service account credentials from JSON"

  bcu_log_args 'Extract fields'
  local z_client_email
  z_client_email=$(jq -r '.client_email' "${z_json_path}") \
                                        || bcu_die "Failed to extract client_email"
  test -n "${z_client_email}"           || bcu_die "Empty client_email in JSON"
  test    "${z_client_email}" != "null" || bcu_die "Null client_email in JSON"

  local z_private_key
  z_private_key=$(jq -r '.private_key' "${z_json_path}") \
                                       || bcu_die "Failed to extract private_key"
  test -n "${z_private_key}"           || bcu_die "Empty private_key in JSON"
  test    "${z_private_key}" != "null" || bcu_die "Null private_key in JSON"

  local z_project_id
  z_project_id=$(jq -r '.project_id' "${z_json_path}") \
                                      || bcu_die "Failed to extract project_id"
  test -n "${z_project_id}"           || bcu_die "Empty project_id in JSON"
  test    "${z_project_id}" != "null" || bcu_die "Null project_id in JSON"

  bcu_log_args 'Verify project matches'
  test -n "${RBRR_GCP_PROJECT_ID:-}" || bcu_die "RBRR_GCP_PROJECT_ID not set"
  test "${z_project_id}" = "${RBRR_GCP_PROJECT_ID}" \
    || bcu_die "Project mismatch: JSON has '${z_project_id}', expected '${RBRR_GCP_PROJECT_ID}'"

  bcu_log_args 'Write RBRA file'
  echo "RBRA_CLIENT_EMAIL=\"${z_client_email}\""    > "${z_rbra_path}"
  echo "RBRA_PRIVATE_KEY=\"${z_private_key}\""     >> "${z_rbra_path}"
  echo "RBRA_PROJECT_ID=\"${z_project_id}\""       >> "${z_rbra_path}"
  echo "RBRA_TOKEN_LIFETIME_SEC=${z_lifetime_sec}" >> "${z_rbra_path}"

  test -f "${z_rbra_path}" || bcu_die "Failed to write RBRA file: ${z_rbra_path}"

  bcu_warn "Consider deleting source JSON after verification: ${z_json_path}"
}

# eof

