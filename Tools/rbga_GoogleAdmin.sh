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
# Recipe Bottle Google Admin - Implementation


# ----------------------------------------------------------------------
# Operational Invariants (RBGA is single writer; 409 is fatal)
#
# - Single admin actor: All RBGA operations are executed by a single admin
#   identity. There are no concurrent writers in the same project.
# - Pristine-state expectation: RBGA init/creation flows assume the project
#   is pristine for the resources they manage. If a resource "already exists"
#   (HTTP 409), that's treated as state drift or prior manual activity.
# - Policy: All HTTP 409 Conflict responses are fatal (bcu_die). We do not
#   treat 409 as idempotent success anywhere in RBGA.
#   If you see a 409, resolve state drift first (destroy/reset), then rerun.
# ----------------------------------------------------------------------

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGA_SOURCED:-}" || bcu_die "Module rbga multiply sourced - check sourcing hierarchy"
ZRBGA_SOURCED=1

######################################################################
# Internal Functions (zrbga_*)

zrbga_kindle() {
  test -z "${ZRBGA_KINDLED:-}" || bcu_die "Module rbga already kindled"

  test -n "${RBRR_GCP_PROJECT_ID:-}"     || bcu_die "RBRR_GCP_PROJECT_ID is not set"
  test   "${#RBRR_GCP_PROJECT_ID}" -gt 0 || bcu_die "RBRR_GCP_PROJECT_ID is empty"

  bcu_log_args 'Ensure RBGC is kindled first'
  zrbgc_sentinel

  ZRBGA_PREFIX="${BDU_TEMP_DIR}/rbga_"
  ZRBGA_EMPTY_JSON="${ZRBGA_PREFIX}empty.json"
  printf '{}' > "${ZRBGA_EMPTY_JSON}"

  # Infix values for HTTP operations
  ZRBGA_INFIX_CREATE="create"
  ZRBGA_INFIX_VERIFY="verify"
  ZRBGA_INFIX_SA_IAM_VERIFY="another_sa_iamverify"
  ZRBGA_INFIX_KEY="key"
  ZRBGA_INFIX_ROLE="role"
  ZRBGA_INFIX_ROLE_SET="role_set"
  ZRBGA_INFIX_REPO_ROLE="repo_role"
  ZRBGA_INFIX_REPO_ROLE_SET="repo_role_set"
  ZRBGA_INFIX_API_IAM_ENABLE="api_iam_enable"
  ZRBGA_INFIX_API_CRM_ENABLE="api_crm_enable"
  ZRBGA_INFIX_API_ART_ENABLE="api_art_enable"
  ZRBGA_INFIX_API_IAM_VERIFY="api_iam_verify"
  ZRBGA_INFIX_API_CRM_VERIFY="api_crm_verify"
  ZRBGA_INFIX_API_ART_VERIFY="api_art_verify"
  ZRBGA_INFIX_API_BUILD_ENABLE="api_build_enable"
  ZRBGA_INFIX_API_BUILD_VERIFY="api_build_verify"
  ZRBGA_INFIX_CB_SA_ACCOUNT_GEN="cb_account_gen"
  ZRBGA_INFIX_CB_PRIME="cb_prime"
  ZRBGA_INFIX_API_CONTAINERANALYSIS_ENABLE="api_containeranalysis_enable"
  ZRBGA_INFIX_API_CONTAINERANALYSIS_VERIFY="api_containeranalysis_verify"
  ZRBGA_INFIX_API_STORAGE_ENABLE="api_storage_enable"
  ZRBGA_INFIX_API_STORAGE_VERIFY="api_storage_verify"
  ZRBGA_INFIX_PROJECT_INFO="project_info"
  ZRBGA_INFIX_CREATE_REPO="create_repo"
  ZRBGA_INFIX_CB_RUNTIME_SA_PEEK="cb_runtime_sa_peek"
  ZRBGA_INFIX_VERIFY_REPO="verify_repo"
  ZRBGA_INFIX_DELETE_REPO="delete_repo"
  ZRBGA_INFIX_REPO_POLICY="repo_policy"
  ZRBGA_INFIX_RPOLICY_SET="repo_policy_set"
  ZRBGA_INFIX_LIST="list"
  ZRBGA_INFIX_API_CHECK="api_checking"
  ZRBGA_INFIX_DELETE="delete"
  ZRBGA_INFIX_LIST_KEYS="list_keys"
  ZRBGA_INFIX_BUCKET_CREATE="bucket_create"
  ZRBGA_INFIX_BUCKET_DELETE="bucket_delete"
  ZRBGA_INFIX_BUCKET_LIST="bucket_list"
  ZRBGA_INFIX_BUCKET_IAM="bucket_iam"
  ZRBGA_INFIX_BUCKET_IAM_SET="bucket_iam_set"
  ZRBGA_INFIX_OBJECT_DELETE="object_delete"

  ZRBGA_POSTFIX_JSON="_response.json"
  ZRBGA_POSTFIX_CODE="_code.txt"

  # Silly but keeps llms happy
  test -n "${RBGC_EVENTUAL_CONSISTENCY_SEC:-}"     || bcu_die "RBGC_EVENTUAL_CONSISTENCY_SEC unset"
  test -n "${RBGC_MAX_CONSISTENCY_SEC:-}"          || bcu_die "RBGC_MAX_CONSISTENCY_SEC unset"


  ZRBGA_KINDLED=1
}

zrbga_sentinel() {
  test "${ZRBGA_KINDLED:-}" = "1" || bcu_die "Module rbga not kindled - call zrbga_kindle first"
}

zrbga_urlencode_capture() {
  zrbga_sentinel
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

zrbga_jq_add_member_to_role_capture() {
  zrbga_sentinel

  local z_infix="${1:-}"
  local z_role="${2:-}"
  local z_member="${3:-}"
  local z_etag_opt="${4:-}"

  local z_policy_file="${ZRBGA_PREFIX}${z_infix}${ZRBGA_POSTFIX_JSON}"

  test -n "${z_policy_file}" || return 1
  test -f "${z_policy_file}" || return 1
  test -n "${z_role}"        || return 1
  test -n "${z_member}"      || return 1

  # NEW: hard requirement - never do setIamPolicy without an etag
  test -n "${z_etag_opt}"    || return 1

  local z_out=""
  z_out=$(
    jq --arg role "${z_role}" --arg member "${z_member}" --arg etag "${z_etag_opt}" '
      .bindings = (.bindings // []) |
      if ([.bindings[]? | .role] | index($role))
      then .bindings |= map(if .role == $role
                            then .members = ((.members // []) + [$member] | unique)
                            else . end)
      else .bindings += [{role: $role, members: [$member]}]
      end
      # NEW: always set the etag we read - this is the optimistic concurrency guard
      | .etag = $etag
    ' "${z_policy_file}"
  ) || return 1

  test -n "${z_out}" || return 1
  printf '%s\n' "${z_out}"
}


# Predicate: Check if resource was newly created and apply propagation delay
# Usage: if zrbga_newly_created_delay "infix" "resource_type" "15"; then ...
zrbga_newly_created_delay() {
  zrbga_sentinel

  local z_infix="${1}"
  local z_resource="${2}"
  local z_delay="${3}"

  local z_code
  z_code=$(zrbga_http_code_capture "${z_infix}") || return 1

  if test "${z_code}" = "200" || test "${z_code}" = "201"; then
    bcu_step "Resource ${z_resource} newly created, waiting ${z_delay}s for propagation"
    sleep "${z_delay}"
  fi
}

######################################################################
# Capture: list required services that are NOT enabled (blank = all enabled)
# arg1: OAuth access token (required, non-empty)
# stdout: space-separated short service names; blank if all enabled
# rc: 0 on success; 1 on internal/processing failure
zrbga_required_apis_missing_capture() {
  zrbga_sentinel

  local z_token="${1:-}"
  test -n "${z_token}" || { echo ""; return 1; }

  local z_missing=""
  local z_api=""
  local z_service=""
  local z_infix=""
  local z_state=""
  local z_code=""

  for z_api in                       \
    "${RBGC_API_SU_VERIFY_CRM}"      \
    "${RBGC_API_SU_VERIFY_GAR}"      \
    "${RBGC_API_SU_VERIFY_IAM}"      \
    "${RBGC_API_SU_VERIFY_BUILD}"    \
    "${RBGC_API_SU_VERIFY_ANALYSIS}" \
    "${RBGC_API_SU_VERIFY_STORAGE}"
  do
    z_service="${z_api##*/}"
    z_infix="${ZRBGA_INFIX_API_CHECK}_${z_service}"

    zrbga_http_json "GET" "${z_api}" "${z_token}" "${z_infix}" || true

    bcu_log_args 'If we cannot even read an HTTP code file, that is a processing failure.'
    z_code=$(zrbga_http_code_capture "${z_infix}") || z_code=""
    test -n "${z_code}" || return 1

    if test "${z_code}" = "200"; then
      z_state=$(zrbga_json_field_capture "${z_infix}" ".state") || z_state=""
      test "${z_state}" = "ENABLED" || z_missing="${z_missing} ${z_service}"
    else
      bcu_log_args 'Any non-200 (403/404/5xx/etc) => treat as NOT enabled'
      z_missing="${z_missing} ${z_service}"
    fi
  done

  printf '%s' "${z_missing# }"
}

# Usage: zrbga_json_field_capture "infix" "jq_expr"
zrbga_json_field_capture() {
  zrbga_sentinel
  local z_infix="${1}"
  local z_jq="${2}"
  local z_json_file="${ZRBGA_PREFIX}${z_infix}${ZRBGA_POSTFIX_JSON}"
  local z_result
  z_result=$(jq -r "${z_jq}" "${z_json_file}")          || return 1
  test -n "${z_result}" && test "${z_result}" != "null" || return 1
  echo "${z_result}"
}

zrbga_http_code_capture() {
  zrbga_sentinel
  local z_infix="${1}"
  local z_file="${ZRBGA_PREFIX}${z_infix}${ZRBGA_POSTFIX_CODE}"
  local z_code
  z_code=$(<"${z_file}") || return 1
  test -n "${z_code}"    || return 1
  echo "${z_code}"
}

# JSON REST helper (hardcoded headers)
# Usage:
#   zrbga_http_json "METHOD" "URL" "TOKEN" "INFIX" ["BODY_FILE"]
zrbga_http_json() {
  zrbga_sentinel

  local z_method="${1}"
  local z_url="${2}"
  local z_token="${3}"
  local z_infix="${4}"
  local z_body_file="${5:-}"

  local z_resp_file="${ZRBGA_PREFIX}${z_infix}${ZRBGA_POSTFIX_JSON}"
  local z_code_file="${ZRBGA_PREFIX}${z_infix}${ZRBGA_POSTFIX_CODE}"
  local z_code_errs="${ZRBGA_PREFIX}${z_infix}${ZRBGA_POSTFIX_CODE}.stderr"

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

# Usage: zrbga_http_require_ok "context" "infix" [warn_code [warn_message]]
zrbga_http_require_ok() {
  zrbga_sentinel
  local z_ctx="$1"
  local z_infix="$2"
  local z_warn_code="${3:-}"
  local z_warn_message="${4:-already exists}"

  local z_code
  z_code=$(zrbga_http_code_capture "${z_infix}") \
    || bcu_die "${z_ctx}: failed to read HTTP code"

  case "${z_code}" in
    200|201|204) return 0 ;;
  esac

  if test -n "${z_warn_code}" && test "${z_code}" = "${z_warn_code}"; then
    bcu_warn "${z_ctx}: ${z_warn_message}"
    return 0
  fi

  local z_err=""
  local z_response_file="${ZRBGA_PREFIX}${z_infix}${ZRBGA_POSTFIX_JSON}"

  if jq -e . "${z_response_file}" >/dev/null 2>&1; then
    z_err=$(zrbga_json_field_capture "${z_infix}" '.error.message') || z_err="Unknown error"
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

# Usage:
#   zrbga_http_json_ok \
#     "Label for logs/context" \        # label (required)
#     "${token}" \                      # OAuth token (required)
#     "POST" \                          # HTTP method (required)
#     "https://..." \                   # URL (required)
#     "INFIX" \                         # file infix for output (required)
#     "" \                              # body file path, or "" if none
#     "" \                              # tolerated warn code (e.g. 409), or "" if none
#     ""                                # tolerated warn message, or "" if none
zrbga_http_json_ok() {
  zrbga_sentinel

  local z_label="$1"        # descriptive label
  local z_token="$2"        # OAuth token
  local z_method="$3"       # HTTP verb
  local z_url="$4"          # full URL
  local z_infix="$5"        # infix for response file naming
  local z_body_file="$6"    # optional: path to request body JSON file, or "" if none
  local z_warn_code="$7"    # optional: tolerated HTTP code (e.g. 409), or "" if none
  local z_warn_message="$8" # optional: tolerated warning message, or "" if none

  bcu_log_args "${z_label}"

  # Perform HTTP request (body file may be "")
  zrbga_http_json "${z_method}" "${z_url}" "${z_token}" "${z_infix}" "${z_body_file:-}"

  # Enforce success (warn/ignore if optional params provided)
  zrbga_http_require_ok "${z_label}" "${z_infix}" "${z_warn_code:-}" "${z_warn_message:-}"
}

# Poll a Google LRO until done (success or error)
# Args:
#   $1  token                (Bearer)
#   $2  op_url               (e.g., https://cloudbuild.googleapis.com/v1/operations/xyz)
#   $3  parent_infix         (namespace, e.g., "cb_prime")
#   $4  poll_sec             (optional; default ${RBGC_EVENTUAL_CONSISTENCY_SEC})
#   $5  max_sec              (optional; default ${RBGC_MAX_CONSISTENCY_SEC})
# Output (stdout on success): final poll infix so caller can find files
# Return: 0 on success; 1 on error/timeout
zrbga_wait_lro_capture() {
  zrbga_sentinel

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

    zrbga_http_json "GET" "${z_op_url}" "${z_token}" "${z_infix}"

    bcu_log_args 'If HTTP not 200, treat as transient unless clearly fatal'
    local z_code=""
    z_code=$(zrbga_http_code_capture "${z_infix}") || return 1
    case "${z_code}" in
      200)                     :                                                                   ;;
      429|408|500|502|503|504) bcu_log_args "LRO transient HTTP ${z_code} at ${z_elapsed}s"        ;;
      *)                       bcu_log_args "LRO non-OK HTTP ${z_code} at ${z_elapsed}s"; return 1 ;;
    esac

    z_done=$(zrbga_json_field_capture "${z_infix}" ".done" 2>/dev/null) || z_done=""
    if test "${z_done}" = "true"; then
      bcu_log_args 'Error?'
      local z_err_msg=""
      z_err_msg=$(zrbga_json_field_capture "${z_infix}" ".error.message" 2>/dev/null) || z_err_msg=""
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

# POST + strict LRO handling (no heuristics)
# Args:
#  1 label
#  2 token
#  3 post_url                # where we send the POST
#  4 resp_infix              # file namespace
#  5 body_file               # optional
#  6 expected_op_field       # jq path to op name: ".name" (default) or ".operation.name"
#  7 op_root_base            # e.g., "https://serviceusage.googleapis.com/v1"
#  8 op_name_prefix          # e.g., "operations/" or "projects/.../operations/"
#  9 poll_sec                # optional
# 10 max_sec                 # optional
zrbga_http_json_lro_ok() {
  zrbga_sentinel

  local z_label="${1}"; local z_token="${2}"; local z_post_url="${3}"
  local z_infix="${4}"; local z_body="${5:-}"
  local z_op_field="${6:-.name}"
  local z_op_root="${7:-}"; local z_op_prefix="${8:-}"
  local z_poll="${9:-${RBGC_EVENTUAL_CONSISTENCY_SEC}}"
  local z_max="${10:-${RBGC_MAX_CONSISTENCY_SEC}}"

  test -n "${z_op_root}"   || bcu_die "${z_label}: op_root_base required"
  test -n "${z_op_prefix}" || bcu_die "${z_label}: op_name_prefix required"

  bcu_log_args '1) Fire the POST and enforce HTTP success.'
  zrbga_http_json "POST" "${z_post_url}" "${z_token}" "${z_infix}" "${z_body:-}"
  zrbga_http_require_ok "${z_label}" "${z_infix}"

  bcu_log_args '2) Extract op name (or return if not an LRO)'
  local z_op_name=""
  z_op_name=$(jq -r "${z_op_field} // empty" "${ZRBGA_PREFIX}${z_infix}${ZRBGA_POSTFIX_JSON}") || z_op_name=""
  test -n "${z_op_name}" || return 0

  bcu_log_args '3) Assert expected shape and build absolute poll URL deterministically'
  case "${z_op_name}" in
    https://*) : ;;  # absolute ok, but still assert prefix match if provided
    *)
      case "${z_op_name}" in
        ${z_op_prefix}*) z_op_name="${z_op_root}/${z_op_name}" ;;
        *) bcu_die "${z_label}: unexpected op name '${z_op_name}' (wanted prefix '${z_op_prefix}')"
      esac
      ;;
  esac

  bcu_log_args '4) Record and poll'
  printf '%s\n' "${z_op_name}" > "${ZRBGA_PREFIX}${z_infix}_op.txt"
  zrbga_wait_lro_capture "${z_token}" "${z_op_name}" "${z_infix}" "${z_poll}" "${z_max}" >/dev/null \
    || bcu_die "${z_label}: operation failed"
}

zrbga_get_admin_token_capture() {
  zrbga_sentinel

  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_ADMIN_RBRA_FILE}") || return 1

  test -n "${z_token}" || return 1
  echo    "${z_token}"
}

zrbga_extract_json_to_rbra() {
  zrbga_sentinel

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

zrbga_create_service_account_with_key() {
  zrbga_sentinel

  local z_account_name="$1"
  local z_display_name="$2"
  local z_description="$3"
  local z_instance="$4"

  local z_account_email="${z_account_name}@${RBGC_SA_EMAIL_FULL}"

  bcu_step 'Get OAuth token from admin'
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step "Create request JSON for ${z_account_name}"
  jq -n                                      \
    --arg account_id "${z_account_name}"     \
    --arg display_name "${z_display_name}"   \
    --arg description "${z_description}"     \
    '{
      accountId: $account_id,
      serviceAccount: {
        displayName: $display_name,
        description: $description
      }
    }' > "${ZRBGA_PREFIX}create_request.json" || bcu_die "Failed to create request JSON"

  bcu_step 'Create service account via REST API'
  zrbga_http_json "POST" "${RBGC_API_SERVICE_ACCOUNTS}" "${z_token}" \
    "${ZRBGA_INFIX_CREATE}" "${ZRBGA_PREFIX}create_request.json"
  zrbga_http_require_ok "Create service account" "${ZRBGA_INFIX_CREATE}" 409 "already exists"
  zrbga_newly_created_delay                      "${ZRBGA_INFIX_CREATE}" "service account" 15
  bcu_info "Service account created: ${z_account_email}"

  zrbga_http_json "GET" "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email}" \
                                    "${z_token}" "${ZRBGA_INFIX_VERIFY}"
  zrbga_http_require_ok "Verify service account" "${ZRBGA_INFIX_VERIFY}"

  bcu_step 'Preflight: ensure no existing USER_MANAGED keys (manual cleanup path)'

  bcu_log_args 'List keys'
  zrbga_http_json "GET" \
    "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email}${RBGC_PATH_KEYS}" \
    "${z_token}" "${ZRBGA_INFIX_LIST_KEYS}"
  zrbga_http_require_ok "List service account keys" "${ZRBGA_INFIX_LIST_KEYS}"

  bcu_log_args 'Count existing user-managed keys'
  local z_user_keys
  z_user_keys=$(jq -r '[.keys[]? | select(.keyType=="USER_MANAGED")] | length' \
                 "${ZRBGA_PREFIX}${ZRBGA_INFIX_LIST_KEYS}${ZRBGA_POSTFIX_JSON}") \
    || bcu_die "Failed to parse service account keys"

  if test "${z_user_keys}" -gt 0; then
    bcu_log_args 'Provide a console URL to delete keys manually, then rerun this command'
    local z_sa_email_enc="${z_account_email//@/%40}"
    local z_keys_url="${RBGC_CONSOLE_URL}iam-admin/serviceaccounts/details/${z_sa_email_enc}?project=${RBRR_GCP_PROJECT_ID}"

    bcu_warn "Found ${z_user_keys} existing USER_MANAGED key(s) on ${z_account_email}."
    bcu_info "Open Console, select the **Keys** tab, delete old keys, then rerun:"
    bcu_info "  ${z_keys_url}"
    bcu_die  "Aborting to avoid minting additional keys."
  fi

  bcu_step 'Generate service account key'
  local z_key_req="${BDU_TEMP_DIR}/rbga_key_request.json"
  printf '%s' '{"privateKeyType": "TYPE_GOOGLE_CREDENTIALS_FILE"}' > "${z_key_req}"
  zrbga_http_json "POST" \
    "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email}${RBGC_PATH_KEYS}" \
    "${z_token}" \
    "${ZRBGA_INFIX_KEY}" \
    "${z_key_req}"
  zrbga_http_require_ok "Generate service account key" "${ZRBGA_INFIX_KEY}"

  bcu_step 'Extract and decode key data'
  local z_key_b64
  z_key_b64=$(zrbga_json_field_capture "${ZRBGA_INFIX_KEY}" '.privateKeyData') \
    || bcu_die "Failed to extract privateKeyData"
  local z_key_json="${BDU_TEMP_DIR}/rbga_key_${z_instance}.json"
  bcu_log_args 'Tolerate macos base64 difference'
  if ! printf '%s' "${z_key_b64}" | base64 -d > "${z_key_json}" 2>/dev/null; then
       printf '%s' "${z_key_b64}" | base64 -D > "${z_key_json}" 2>/dev/null \
      || bcu_die "Failed to decode key data"
  fi

  bcu_step 'Convert JSON key to RBRA format'
  local z_rbra_file="${BDU_OUTPUT_DIR}/${z_instance}.rbra"

  local z_client_email
  z_client_email=$(jq -r '.client_email' "${z_key_json}") || bcu_die "Failed to extract client_email"
  test -n "${z_client_email}" || bcu_die "Empty client_email in key JSON"

  local z_private_key
  z_private_key=$(jq -r '.private_key' "${z_key_json}") || bcu_die "Failed to extract private_key"
  test -n "${z_private_key}" || bcu_die "Empty private_key in key JSON"

  local z_project_id
  z_project_id=$(jq -r '.project_id' "${z_key_json}") || bcu_die "Failed to extract project_id"
  test -n "${z_project_id}" || bcu_die "Empty project_id in key JSON"

  bcu_step 'Write RBRA file' "${z_rbra_file}"
  echo "RBRA_CLIENT_EMAIL=\"${z_client_email}\""  > "${z_rbra_file}"
  echo "RBRA_PRIVATE_KEY=\"${z_private_key}\""   >> "${z_rbra_file}"
  echo "RBRA_PROJECT_ID=\"${z_project_id}\""     >> "${z_rbra_file}"
  echo "RBRA_TOKEN_LIFETIME_SEC=1800"            >> "${z_rbra_file}"

  test -f "${z_rbra_file}" || bcu_die "Failed to write RBRA file ${z_rbra_file}"

  rm -f "${z_key_json}"
  bcu_info "RBRA file written: ${z_rbra_file}"
}

# Add a project-scoped IAM role binding with optimistic concurrency and strong read-back.
#
# Contract / Behavior:
# - Reads policy (v3), preserves returned etag, and writes via setIamPolicy using that etag.
# - Retries set on transient HTTP (429/500/502/503/504) within RBGC_* bounds; dies on 409/412.
# - Verifies binding presence by re-GET within bounded wait; dies on timeout.
#
# Invariants:
# - Single writer regime; HTTP 409 is fatal (state drift). HTTP 412 indicates etag mismatch and is fatal.
# - Never sends setIamPolicy without an etag.
#
# Parameters:
#   $1  label           (string) Log context for diagnostics.
#   $2  token           (string) OAuth bearer (do not write to disk).
#   $3  resource_base   (string) Base resource URL, e.g.:
#                       "https://cloudresourcemanager.googleapis.com/v1/projects/${RBRR_GCP_PROJECT_ID}"
#                       The function derives:
#                         GET: "${resource_base}:getIamPolicy"
#                         SET: "${resource_base}:setIamPolicy"
#   $4  role            (string) Role to grant, e.g. "roles/viewer".
#   $5  member          (string) Member, e.g. "serviceAccount:name@project.iam.gserviceaccount.com".
#   $6  parent_infix    (string, optional) Forensic namespace for temp files; default "newrole".
#
# Input/Output Files:
# - Uses module temp prefixes from kindle; writes JSON/code artifacts for each HTTP call (forensics).
#
# Returns:
# - 0 on success after verifying binding; dies (bcu_die) with actionable message on failure.
#
# Notes:
# - Always POST the body {"options":{"requestedPolicyVersion":3}} to getIamPolicy; do not rely on query params.
# - Validate only resource_base (single source of truth) and derive URLs internally.
zrbga_add_iam_role() {
  zrbga_sentinel
  
  local z_label="${1:-}"
  local z_token="${2:-}"
  local z_resource="${3:-}"  # resource_base: Base resource URL
  local z_role="${4:-}"
  local z_member="${5:-}"
  local z_parent_infix="${6:-newrole}"
  local z_get_url="${z_resource}:getIamPolicy"
  local z_set_url="${z_resource}:setIamPolicy"

  test -n "${z_token}"    || bcu_die "token required"
  test -n "${z_resource}" || bcu_die "resource required"
  test -n "${z_role}"     || bcu_die "role required"
  test -n "${z_member}"   || bcu_die "member required"

  bcu_log_args "${z_label}: add ${z_member} to ${z_role}"

  bcu_log_args '1) GET policy (v3)'
  local z_get_body="${ZRBGA_PREFIX}${z_parent_infix}_get_body.json"
  local z_get_infix="${z_parent_infix}-get"
  printf '%s\n' '{"options":{"requestedPolicyVersion":3}}' > "${z_get_body}"
  zrbga_http_json_ok "${z_label} (get policy)" "${z_token}" "POST" \
    "${z_get_url}" "${z_get_infix}" "${z_get_body}"

  bcu_log_args 'Extract etag; require non-empty'
  local z_etag=""
  z_etag=$(zrbga_json_field_capture "${z_get_infix}" ".etag") || bcu_die "Missing etag"
  test -n "${z_etag}" || bcu_die "Empty etag"

  bcu_log_args "Using etag ${z_etag}"

  bcu_log_args '2) Build new policy JSON in temp (bindings unique; version=3; keep etag)'
  local z_new_policy_json=""
  z_new_policy_json=$(zrbga_jq_add_member_to_role_capture "${z_get_infix}" "${z_role}" "${z_member}" "${z_etag}") \
    || bcu_die "Failed to compose policy JSON"
  bcu_log_args 'Ensure version=3'
  z_new_policy_json=$(jq '.version=3' <<<"${z_new_policy_json}") || bcu_die "Failed to set version=3"

  local z_set_body="${ZRBGA_PREFIX}${z_parent_infix}_set_body.json"
  printf '{"policy":%s}\n' "${z_new_policy_json}" > "${z_set_body}"

  bcu_log_args '3) setIamPolicy (fatal on 409/412 by policy)'
  local z_elapsed=0
  local z_set_infix=""
  while :; do
    z_set_infix="${z_parent_infix}-set-${z_elapsed}s"
    zrbga_http_json "POST" "${z_set_url}" "${z_token}" "${z_set_infix}" "${z_set_body}"

    local z_code=""
    z_code=$(zrbga_http_code_capture "${z_set_infix}") || bcu_die "No HTTP code"
    case "${z_code}" in
      200)                 break ;;
      412)                 bcu_die "${z_label}: precondition failed (etag mismatch)"    ;;
      429|500|502|503|504) bcu_log_args "Transient ${z_code} at ${z_elapsed}s; retry"   ;;
      409)                 bcu_die "${z_label}: HTTP 409 Conflict (fatal by invariant)" ;;
      *)                   zrbga_http_require_ok "${z_label} (set policy)" "${z_set_infix}" "" ;;
    esac

    test "${z_elapsed}" -ge "${RBGC_MAX_CONSISTENCY_SEC}" && bcu_die "${z_label}: timeout setting policy"
    sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
    z_elapsed=$((z_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
  done

  bcu_log_args '4) Verify membership within bounded wait'
  z_elapsed=0
  while :; do
    local z_verify_infix="${z_parent_infix}-verify-${z_elapsed}s"
    zrbga_http_json_ok "${z_label} (verify)" "${z_token}" "POST" \
                       "${z_get_url}" "${z_verify_infix}" "${z_get_body}"

    if jq -e --arg r "${z_role}" --arg m "${z_member}" \
         '.bindings[]? | select(.role==$r) | (.members // [])[]? == $m' \
         "${ZRBGA_PREFIX}${z_verify_infix}${ZRBGA_POSTFIX_JSON}" >/dev/null; then
      bcu_log_args "Observed ${z_role} for ${z_member}"

      bcu_log_args "Post-set etag $(zrbga_json_field_capture "${z_verify_infix}" ".etag" 2>/dev/null || echo "")"

      return 0
    fi

    test "${z_elapsed}" -ge "${RBGC_MAX_CONSISTENCY_SEC}" && bcu_die "${z_label}: verify timeout"
    sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
    z_elapsed=$((z_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
  done
}

zrbga_add_repo_iam_role() {
  zrbga_sentinel

  local z_account_email="${1:-}"
  local z_location="${2:-}"
  local z_repository="${3:-}"
  local z_role="${4:-}"

  test -n "${z_account_email}" || bcu_die "Service account email required"
  test -n "${z_location}"      || bcu_die "Location is required"
  test -n "${z_repository}"    || bcu_die "RBRR_GAR_REPOSITORY is required"
  test -n "${z_role}"          || bcu_die "Role is required"

  bcu_log_args 'Adding repo-scoped IAM role' \
               " ${z_role} to ${z_account_email} on ${z_location}/${z_repository}"

  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  local z_resource="projects/${RBRR_GCP_PROJECT_ID}${RBGC_PATH_LOCATIONS}/${z_location}${RBGC_PATH_REPOSITORIES}/${z_repository}"
  local z_get_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:getIamPolicy"
  local z_set_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:setIamPolicy"

  bcu_log_args 'Get current repo IAM policy'
  zrbga_http_json "POST" "${z_get_url}" "${z_token}" \
                                              "${ZRBGA_INFIX_REPO_ROLE}" "${ZRBGA_EMPTY_JSON}"

  local z_get_code
  z_get_code=$(zrbga_http_code_capture "${ZRBGA_INFIX_REPO_ROLE}") || z_get_code=""

  if test "${z_get_code}" = "404"; then
    # 404 means repo exists but has no IAM policy yet - this is normal for new repos
    bcu_log_args 'No IAM policy exists yet (404), initializing with empty bindings'
    echo '{"bindings":[]}' > "${ZRBGA_PREFIX}${ZRBGA_INFIX_REPO_ROLE}${ZRBGA_POSTFIX_JSON}"
  elif test "${z_get_code}" != "200"; then
    local z_err="HTTP ${z_get_code}"
    if jq -e . "${ZRBGA_PREFIX}${ZRBGA_INFIX_REPO_ROLE}${ZRBGA_POSTFIX_JSON}" >/dev/null 2>&1; then
      z_err=$(zrbga_json_field_capture "${ZRBGA_INFIX_REPO_ROLE}" '.error.message') || z_err="HTTP ${z_get_code}"
    fi
    bcu_die "Get repo IAM policy failed: ${z_err}"
  fi

  bcu_log_args 'Update repo IAM policy'
  local z_updated_policy="${BDU_TEMP_DIR}/rbga_repo_updated_policy.json"
  jq --arg role   "${z_role}"                                      \
     --arg member "serviceAccount:${z_account_email}"              \
     '
       .bindings = (.bindings // []) |
       if ( ([ .bindings[]? | .role ] | index($role)) ) then
         .bindings = ( .bindings | map(
           if .role == $role then .members = ( (.members // []) + [$member] | unique )
           else .
           end))
       else
         .bindings += [{role: $role, members: [$member]}]
       end
     ' "${ZRBGA_PREFIX}${ZRBGA_INFIX_REPO_ROLE}${ZRBGA_POSTFIX_JSON}" \
     > "${z_updated_policy}" || bcu_die "Failed to update policy json"

  bcu_log_args 'Set updated repo IAM policy'
  local z_repo_set_body="${BDU_TEMP_DIR}/rbga_repo_set_policy_body.json"
  jq -n --slurpfile p "${z_updated_policy}" '{policy:$p[0]}' > "${z_repo_set_body}" \
    || bcu_die "Failed to build repo setIamPolicy body"
  zrbga_http_json "POST" "${z_set_url}" "${z_token}" \
                                              "${ZRBGA_INFIX_REPO_ROLE_SET}" "${z_repo_set_body}"
  zrbga_http_require_ok "Set repo IAM policy" "${ZRBGA_INFIX_REPO_ROLE_SET}"

  bcu_log_args 'Successfully added repo-scoped role' "${z_role}"
}

zrbga_add_sa_iam_role() {
  zrbga_sentinel

  local z_target_sa_email="$1"
  local z_member_sa_email="$2"
  local z_role="$3"

  bcu_log_args "Granting ${z_role} on SA ${z_target_sa_email} to ${z_member_sa_email}"

  # Caller must have already primed Cloud Build if this is the runtime SA.
  # We do a hard existence check and crash if not accessible.
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_log_args 'Verify target SA exists'
  local z_target_encoded
  z_target_encoded=$(zrbga_urlencode_capture "${z_target_sa_email}") \
    || bcu_die "Failed to encode SA email"

  local z_verify_code
  zrbga_http_json "GET" \
    "${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/-/serviceAccounts/${z_target_encoded}" \
                             "${z_token}" "${ZRBGA_INFIX_SA_IAM_VERIFY}"
  z_verify_code=$(zrbga_http_code_capture "${ZRBGA_INFIX_SA_IAM_VERIFY}") || z_verify_code=""
  test "${z_verify_code}" = "200" || \
    bcu_die "Target service account not accessible: ${z_target_sa_email} (HTTP ${z_verify_code})"

  local z_sa_resource="${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/-/serviceAccounts/${z_target_encoded}"

  bcu_log_args 'Get current SA IAM policy'
  zrbga_http_json "POST" "${z_sa_resource}:getIamPolicy" "${z_token}" \
    "${ZRBGA_INFIX_ROLE}" "${ZRBGA_EMPTY_JSON}"

  local z_code
  z_code=$(zrbga_http_code_capture "${ZRBGA_INFIX_ROLE}") || z_code=""
  if test "${z_code}" != "200"; then
    bcu_log_args 'No IAM policy exists yet, initializing'
    echo '{"bindings":[]}' > "${ZRBGA_PREFIX}${ZRBGA_INFIX_ROLE}${ZRBGA_POSTFIX_JSON}"
  fi

  bcu_log_args 'Update SA IAM policy with new role binding'
  local z_updated_policy="${BDU_TEMP_DIR}/rbga_sa_updated_policy.json"
  jq --arg role   "${z_role}"                              \
     --arg member "serviceAccount:${z_member_sa_email}"    \
     '
       .bindings = (.bindings // []) |
       if ( ([ .bindings[]? | .role ] | index($role)) ) then
         .bindings = ( .bindings | map(
           if .role == $role then .members = ( (.members // []) + [$member] | unique )
           else . end))
       else
         .bindings += [{role: $role, members: [$member]}]
       end
     ' "${ZRBGA_PREFIX}${ZRBGA_INFIX_ROLE}${ZRBGA_POSTFIX_JSON}" \
     > "${z_updated_policy}" || bcu_die "Failed to update SA IAM policy"

  bcu_log_args 'Set updated SA IAM policy'
  local z_set_body="${BDU_TEMP_DIR}/rbga_sa_set_policy_body.json"
  jq -n --slurpfile p "${z_updated_policy}" '{policy:$p[0]}' > "${z_set_body}" \
    || bcu_die "Failed to build SA setIamPolicy body"

  zrbga_http_json "POST" "${z_sa_resource}:setIamPolicy" "${z_token}" \
    "${ZRBGA_INFIX_ROLE_SET}" "${z_set_body}"
  zrbga_http_require_ok "Set SA IAM policy" "${ZRBGA_INFIX_ROLE_SET}"

  bcu_log_args 'Successfully granted SA role' "${z_role}"
}

zrbga_create_gcs_bucket() {
  zrbga_sentinel

  local z_token="${1}"
  local z_bucket_name="${2}"

  bcu_log_args 'Create bucket request JSON for '"${z_bucket_name}"
  local z_bucket_req="${BDU_TEMP_DIR}/rbga_bucket_create_req.json"
  jq -n --arg name "${z_bucket_name}"            \
        --arg location "${RBGC_GAR_LOCATION}"    \
    '{
      name: $name,
      location: $location,
      storageClass: "STANDARD"
    }' > "${z_bucket_req}" || bcu_die "Failed to create bucket request JSON"

  bcu_log_args 'Send bucket creation request'
  local z_code
  local z_err
  zrbga_http_json "POST" "${RBGC_API_GCS_BUCKET_CREATE}" "${z_token}" \
                                   "${ZRBGA_INFIX_BUCKET_CREATE}" "${z_bucket_req}"
  z_code=$(zrbga_http_code_capture "${ZRBGA_INFIX_BUCKET_CREATE}") || bcu_die "Bad bucket creation HTTP code"
  z_err=$(zrbga_json_field_capture "${ZRBGA_INFIX_BUCKET_CREATE}" '.error.message') || z_err="HTTP ${z_code}"

  case "${z_code}" in
    200|201) bcu_info "Bucket ${z_bucket_name} created";        return 0 ;;
    409)     bcu_warn "Bucket ${z_bucket_name} already exists"; return 0 ;;
    *)       bcu_die "Failed to create bucket: ${z_err}"                 ;;
  esac
}

zrbga_list_bucket_objects_capture() {
  zrbga_sentinel

  local z_token="${1}"
  local z_bucket_name="${2}"

  local z_list_url_base="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}/o"
  local z_page_token=""
  local z_first=1

  while :; do
    bcu_log_args "Build URL with optional pageToken -> ${z_first}"
    local z_url="${z_list_url_base}"
    if test -n "${z_page_token}"; then
      bcu_log_args 'pageToken must be URL-encoded'
      local z_tok_enc
      z_tok_enc=$(zrbga_urlencode_capture "${z_page_token}") || return 1
      z_url="${z_url}?pageToken=${z_tok_enc}"
    fi

    bcu_log_args 'Use a unique infix per page to avoid clobbering files'
    local z_infix="${ZRBGA_INFIX_BUCKET_LIST}${z_first}"
    zrbga_http_json "GET" "${z_url}" "${z_token}" "${z_infix}"

    local z_code
    z_code=$(zrbga_http_code_capture "${z_infix}") || return 1
    test "${z_code}" = "200" || return 1

    bcu_log_args 'Print names from this page (if any)'
    jq -r '.items[]?.name // empty' \
      "${ZRBGA_PREFIX}${z_infix}${ZRBGA_POSTFIX_JSON}" || return 1

    bcu_log_args 'Next page?'
    z_page_token=$(jq -r '.nextPageToken // empty' \
      "${ZRBGA_PREFIX}${z_infix}${ZRBGA_POSTFIX_JSON}") || return 1

    test -n "${z_page_token}" || break
    z_first=$((z_first + 1))
  done
}

# Ensure Cloud Build service agent exists and admin can trigger builds
zrbga_ensure_cloudbuild_service_agent() {
  zrbga_sentinel

  local z_token="${1}"
  local z_project_number="${2}"

  local z_cb_service_agent="service-${z_project_number}@gcp-sa-cloudbuild.iam.gserviceaccount.com"
  local z_admin_sa_email="${RBGC_ADMIN_ROLE}@${RBGC_SA_EMAIL_FULL}"
  local z_gen_url="${RBGC_API_CB_GENERATE_SA}"

  zrbga_http_json_lro_ok                                   \
    "Generate Cloud Build service agent"                   \
    "${z_token}"                                           \
    "${z_gen_url}"                                         \
    "${ZRBGA_INFIX_CB_SA_ACCOUNT_GEN}"                     \
    "${ZRBGA_EMPTY_JSON}"                                  \
    ".name"                                                \
    "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}" \
    "${RBGC_OP_PREFIX_GLOBAL}"                             \
    "5"                                                    \
    "60"

  bcu_step 'Grant Cloud Build Service Agent role'
  zrbga_add_iam_role                        \
    "Grant Cloud Build Service Agent role"  \
    "${z_token}"                            \
    "${RBGC_PROJECT_RESOURCE}"              \
    "roles/cloudbuild.serviceAgent"         \
    "serviceAccount:${z_cb_service_agent}"  \
    "cb-agent"

  bcu_step 'Grant admin necessary permissions to trigger builds'
  bcu_step "Grant admin Cloud Build permissions"

  bcu_step 'Admin needs serviceAccountUser on the service agent'
  zrbga_add_sa_iam_role "${z_cb_service_agent}" "${z_admin_sa_email}" "roles/iam.serviceAccountUser"

  bcu_step 'Admin needs Cloud Build Editor for builds.create and viz'
  zrbga_add_iam_role                        \
    "Grant admin Cloud Build Editor"        \
    "${z_token}"                            \
    "${RBGC_PROJECT_RESOURCE}"              \
    "roles/cloudbuild.builds.editor"        \
    "serviceAccount:${z_admin_sa_email}"    \
    "admin-cb"

  zrbga_add_iam_role                        \
    "Grant admin Viewer"                    \
    "${z_token}"                            \
    "${RBGC_PROJECT_RESOURCE}"              \
    "roles/viewer"                          \
    "serviceAccount:${z_admin_sa_email}"    \
    "admin-viewer"

  bcu_info "Cloud Build service agent configured with admin permissions"
}

zrbga_prime_cloud_build() {
  zrbga_sentinel

  local z_token="${1:-}"
  test -n "${z_token}" || bcu_die "zrbga_prime_cloud_build: token required"

  bcu_log_args 'Create degenerate build body with jq'
  local z_body="${BDU_TEMP_DIR}/rbga_cb_prime_body.json"
  jq -n --arg mt "${RBRR_GCB_MACHINE_TYPE:-E2_HIGHCPU_8}" --arg to "${RBRR_GCB_TIMEOUT:-300s}" '
    {
      steps: [
        {
          name: "gcr.io/cloud-builders/gcloud",
          entrypoint: "bash",
          args: ["-lc", "true"]  # intentionally no-op
        }
      ],
      options: { machineType: $mt, logging: "CLOUD_LOGGING_ONLY" },
      timeout: $to
    }' > "${z_body}" || bcu_die "Failed to write cb prime body"

  local z_url="${RBGC_API_ROOT_CLOUDBUILD}${RBGC_CLOUDBUILD_V1}/projects/${RBGC_GCB_PROJECT_ID}/locations/${RBGC_GCB_REGION}/builds"

  zrbga_http_json_lro_ok                                   \
    "Prime Cloud Build"                                    \
    "${z_token}"                                           \
    "${z_url}"                                             \
    "${ZRBGA_INFIX_CB_PRIME}"                              \
    "${z_body}"                                            \
    ".name"                                                \
    "${RBGC_API_ROOT_CLOUDBUILD}${RBGC_CLOUDBUILD_V1}"     \
    "${RBGC_OP_PREFIX_GLOBAL}"                             \
    "10"                                                   \
    "300"

  bcu_log_args 'Prime complete.'
}

zrbga_empty_gcs_bucket() {
  zrbga_sentinel

  local z_token="${1}"
  local z_bucket_name="${2}"

  bcu_log_args 'Get list of objects to delete'
  local z_objects
  z_objects=$(zrbga_list_bucket_objects_capture "${z_token}" "${z_bucket_name}") || {
    bcu_log_args 'No objects found or bucket not accessible'
    return 0
  }

  test -n "${z_objects}" || { bcu_log_args 'Bucket is empty'; return 0; }

  bcu_log_args 'Delete each object'
  local z_object=""
  local z_delete_url=""
  local z_delete_code=""
  while IFS= read -r z_object; do
    test -n "${z_object}" || continue
    bcu_log_args "Deleting object: ${z_object}"

    local z_object_enc
    z_object_enc=$(zrbga_urlencode_capture "${z_object}") || z_object_enc=""
    test -n "${z_object_enc}" || { bcu_warn "Failed to encode object name: ${z_object}"; continue; }
    z_delete_url="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}/o/${z_object_enc}"


    zrbga_http_json "DELETE" "${z_delete_url}" \
                               "${z_token}" "${ZRBGA_INFIX_OBJECT_DELETE}"
    z_delete_code=$(zrbga_http_code_capture "${ZRBGA_INFIX_OBJECT_DELETE}") || z_delete_code=""
    case "${z_delete_code}" in
      204|404) bcu_log_args "Object deleted or not found: ${z_object}"                ;;
      *)       bcu_warn "Failed to delete object ${z_object} (HTTP ${z_delete_code})" ;;
    esac
  done <<< "${z_objects}"
}

zrbga_delete_gcs_bucket_predicate() {
  zrbga_sentinel

  local z_token="${1}"
  local z_bucket_name="${2}"

  bcu_log_args 'Empty bucket before deletion: '"${z_bucket_name}"
  zrbga_empty_gcs_bucket "${z_token}" "${z_bucket_name}"

  bcu_log_args 'Delete the bucket'
  local z_code
  local z_err
  local z_delete_url="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}"
  zrbga_http_json "DELETE" "${z_delete_url}" \
                      "${z_token}" "${ZRBGA_INFIX_BUCKET_DELETE}"
  z_code=$(zrbga_http_code_capture "${ZRBGA_INFIX_BUCKET_DELETE}") || z_code=""
  z_err=$(zrbga_json_field_capture "${ZRBGA_INFIX_BUCKET_DELETE}" '.error.message') || z_err="HTTP ${z_code}"
  case "${z_code}" in
    204) bcu_info "Bucket ${z_bucket_name} deleted";                           return 0 ;;
    404) bcu_warn "Bucket ${z_bucket_name} not found (already deleted)";       return 0 ;;
    409) bcu_warn "Bucket ${z_bucket_name} not empty or has retention policy"; return 1 ;;
    *)   bcu_warn "Bucket ${z_bucket_name} failed delete";                     return 1 ;;
  esac
}

zrbga_add_bucket_iam_role() {
  zrbga_sentinel

  local z_bucket_name="${1}"
  local z_account_email="${2}"
  local z_role="${3}"
  local z_token="${4}"

  bcu_log_args "Adding bucket IAM role ${z_role} to ${z_account_email}"

  local z_code

  bcu_log_args 'Get current bucket IAM policy'
  local z_iam_url="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}/iam"
  zrbga_http_json "GET" "${z_iam_url}" "${z_token}" "${ZRBGA_INFIX_BUCKET_IAM}"
  z_code=$(zrbga_http_code_capture                  "${ZRBGA_INFIX_BUCKET_IAM}") || z_code=""
  if test "${z_code}" != "200"; then
    bcu_log_args 'Initialize empty IAM policy for bucket'
    echo '{"bindings":[]}' > "${ZRBGA_PREFIX}${ZRBGA_INFIX_BUCKET_IAM}${ZRBGA_POSTFIX_JSON}"
  fi

  bcu_log_args 'Update bucket IAM policy'
  local z_updated="${BDU_TEMP_DIR}/rbga_bucket_iam_updated.json"
  local z_etag
  z_etag=$(zrbga_json_field_capture "${ZRBGA_INFIX_BUCKET_IAM}" '.etag') || z_etag=""
  jq --arg role "${z_role}"                           \
     --arg member "serviceAccount:${z_account_email}" \
     --arg etag "${z_etag}"                           \
     '
       .bindings = (.bindings // []) |
       if ( ([ .bindings[]? | .role ] | index($role)) ) then
         .bindings = ( .bindings | map(
           if .role == $role then .members = ( (.members // []) + [$member] | unique )
           else .
           end))
       else
         .bindings += [{role: $role, members: [$member]}]
       end
       | ( if $etag != "" then .etag = $etag else . end )
     ' "${ZRBGA_PREFIX}${ZRBGA_INFIX_BUCKET_IAM}${ZRBGA_POSTFIX_JSON}" \
     > "${z_updated}" || bcu_die "Failed to update bucket IAM policy"

  local z_err

  bcu_log_args 'Set updated bucket IAM policy'
  zrbga_http_json "PUT" "${z_iam_url}" "${z_token}" \
                                   "${ZRBGA_INFIX_BUCKET_IAM_SET}" "${z_updated}"
  z_code=$(zrbga_http_code_capture "${ZRBGA_INFIX_BUCKET_IAM_SET}")                  || z_code=""
  z_err=$(zrbga_json_field_capture "${ZRBGA_INFIX_BUCKET_IAM_SET}" '.error.message') || z_err="HTTP ${z_code}"
  test "${z_code}" = "200" || bcu_die "Failed to set bucket IAM policy: ${z_err}"

  bcu_log_args "Successfully added bucket role ${z_role}"
}

######################################################################
# External Functions (rbga_*)

rbga_initialize_admin() {
  zrbga_sentinel

  local z_json_path="${1:-}"

  bcu_doc_brief "Initialize RBGA for this project: enable/verify APIs, create GAR repo, and grant Cloud Build SA."
  bcu_doc_param "json_path" "Path to downloaded admin JSON key (will be converted to RBRA)"
  bcu_doc_shown || return 0

  test -n "${z_json_path}" || bcu_die "First argument must be path to downloaded JSON key file."

  local z_admin_sa_email="${RBGC_ADMIN_ROLE}@${RBGC_SA_EMAIL_FULL}"
  local z_prime_pause_sec=120

  bcu_step 'Convert admin JSON to RBRA'
  zrbga_extract_json_to_rbra "${z_json_path}" "${RBRR_ADMIN_RBRA_FILE}" "1800"

  bcu_step 'Mint admin OAuth token'
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step 'Check which required APIs need enabling'
  local z_missing=""
  z_missing=$(zrbga_required_apis_missing_capture "${z_token}") \
    || bcu_die "Failed to check API status"


  if test -n "${z_missing}"; then
    bcu_info "APIs needing enablement: ${z_missing}"

    # Invariant: API enable is gated by the preflight above.
    # Any 409 here means the preflight or our assumptions are wrong -> die.

    bcu_step 'Enable IAM API'
    zrbga_http_json_lro_ok                                      \
      "Enable IAM API"                                          \
      "${z_token}"                                              \
      "${RBGC_API_SU_ENABLE_IAM}"                               \
      "${ZRBGA_INFIX_API_IAM_ENABLE}"                           \
      "${ZRBGA_EMPTY_JSON}"                                     \
      ".name"                                                   \
      "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}"    \
      "${RBGC_OP_PREFIX_GLOBAL}"                                \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                        \
      "${RBGC_MAX_CONSISTENCY_SEC}"

    bcu_step 'Enable Cloud Resource Manager API'
    zrbga_http_json_lro_ok                                      \
      "Enable Cloud Resource Manager API"                       \
      "${z_token}"                                              \
      "${RBGC_API_SU_ENABLE_CRM}"                               \
      "${ZRBGA_INFIX_API_CRM_ENABLE}"                           \
      "${ZRBGA_EMPTY_JSON}"                                     \
      ".name"                                                   \
      "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}"    \
      "${RBGC_OP_PREFIX_GLOBAL}"                                \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                        \
      "${RBGC_MAX_CONSISTENCY_SEC}"

    bcu_step 'Enable Artifact Registry API'
    zrbga_http_json_lro_ok                                      \
      "Enable Artifact Registry API"                            \
      "${z_token}"                                              \
      "${RBGC_API_SU_ENABLE_GAR}"                               \
      "${ZRBGA_INFIX_API_ART_ENABLE}"                           \
      "${ZRBGA_EMPTY_JSON}"                                     \
      ".name"                                                   \
      "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}"    \
      "${RBGC_OP_PREFIX_GLOBAL}"                                \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                        \
      "${RBGC_MAX_CONSISTENCY_SEC}"

    bcu_step 'Enable Cloud Build API'
    zrbga_http_json_lro_ok                                      \
      "Enable Cloud Build API"                                  \
      "${z_token}"                                              \
      "${RBGC_API_SU_ENABLE_BUILD}"                             \
      "${ZRBGA_INFIX_API_BUILD_ENABLE}"                         \
      "${ZRBGA_EMPTY_JSON}"                                     \
      ".name"                                                   \
      "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}"    \
      "${RBGC_OP_PREFIX_GLOBAL}"                                \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                        \
      "${RBGC_MAX_CONSISTENCY_SEC}"

    bcu_step 'Enable Container Analysis API'
    zrbga_http_json_lro_ok                                      \
      "Enable Container Analysis API"                           \
      "${z_token}"                                              \
      "${RBGC_API_SU_ENABLE_ANALYSIS}"                          \
      "${ZRBGA_INFIX_API_CONTAINERANALYSIS_ENABLE}"             \
      "${ZRBGA_EMPTY_JSON}"                                     \
      ".name"                                                   \
      "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}"    \
      "${RBGC_OP_PREFIX_GLOBAL}"                                \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                        \
      "${RBGC_MAX_CONSISTENCY_SEC}"

    bcu_step 'Enable Cloud Storage API (build bucket deps)'
    zrbga_http_json_lro_ok                                      \
      "Enable Cloud Storage API"                                \
      "${z_token}"                                              \
      "${RBGC_API_SU_ENABLE_STORAGE}"                           \
      "${ZRBGA_INFIX_API_STORAGE_ENABLE}"                       \
      "${ZRBGA_EMPTY_JSON}"                                     \
      ".name"                                                   \
      "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}"    \
      "${RBGC_OP_PREFIX_GLOBAL}"                                \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                        \
      "${RBGC_MAX_CONSISTENCY_SEC}"
  fi

  bcu_step 'Discover Project Number'
  local z_project_number
  zrbga_http_json "GET" "${RBGC_API_CRM_GET_PROJECT}" \
                                 "${z_token}" "${ZRBGA_INFIX_PROJECT_INFO}"
  zrbga_http_require_ok "Get project info"    "${ZRBGA_INFIX_PROJECT_INFO}"
  z_project_number=$(zrbga_json_field_capture "${ZRBGA_INFIX_PROJECT_INFO}" '.projectNumber') \
    || bcu_die "Failed to extract project number"

  bcu_step 'Directly create the cloudbuild service agent'
  zrbga_ensure_cloudbuild_service_agent "${z_token}" "${z_project_number}"

  bcu_step 'Grant Cloud Build invoke permissions to admin (idempotent)'
  zrbga_add_iam_role                             \
    "Grant Cloud Build invoke permissions"       \
    "${z_token}"                                 \
    "${RBGC_PROJECT_RESOURCE}"                   \
    "${RBGC_ROLE_CLOUDBUILD_BUILDS_EDITOR}"      \
    "serviceAccount:${z_admin_sa_email}"         \
    "admin-cb-invoke"

  zrbga_add_iam_role                             \
    "Grant Service Usage Consumer"               \
    "${z_token}"                                 \
    "${RBGC_PROJECT_RESOURCE}"                   \
    "roles/serviceusage.serviceUsageConsumer"    \
    "serviceAccount:${z_admin_sa_email}"         \
    "admin-su"

  bcu_step 'Create/verify Cloud Storage bucket'
  zrbga_create_gcs_bucket "${z_token}" "${RBGC_GCS_BUCKET}"

  bcu_step 'Create/verify Docker format Artifact Registry repo'
  bcu_log_args "  The repo is ${RBRR_GAR_REPOSITORY} in ${RBGC_GAR_LOCATION}"

  test -n "${RBGC_GAR_LOCATION:-}"   || bcu_die "RBGC_GAR_LOCATION is not set"
  test -n "${RBRR_GAR_REPOSITORY:-}" || bcu_die "RBRR_GAR_REPOSITORY is not set"

  local z_parent="projects/${RBRR_GCP_PROJECT_ID}${RBGC_PATH_LOCATIONS}/${RBGC_GAR_LOCATION}"
  local z_resource="${z_parent}${RBGC_PATH_REPOSITORIES}/${RBRR_GAR_REPOSITORY}"
  local z_create_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_parent}${RBGC_PATH_REPOSITORIES}?repositoryId=${RBRR_GAR_REPOSITORY}"
  local z_get_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}"
  local z_create_body="${BDU_TEMP_DIR}/rbga_create_repo_body.json"

  jq -n '{format:"DOCKER"}' > "${z_create_body}" || bcu_die "Failed to build create-repo body"

  bcu_step 'Create DOCKER format repo'
  zrbga_http_json_lro_ok                                             \
    "Create Artifact Registry repo"                                  \
    "${z_token}"                                                     \
    "${z_create_url}"                                                \
    "${ZRBGA_INFIX_CREATE_REPO}"                                     \
    "${z_create_body}"                                               \
    ".operation.name"                                                \
    "${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}"   \
    "${RBGC_OP_PREFIX_GLOBAL}"                                       \
    "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                               \
    "${RBGC_MAX_CONSISTENCY_SEC}"

  bcu_step 'One-time propagation pause before Cloud Build priming'
  bcu_step "  About to sleep ${z_prime_pause_sec}s"
  sleep "${z_prime_pause_sec}"

  bcu_step 'Trigger degenerate build to assure builder account creation'
  zrbga_prime_cloud_build "${z_token}"

  bcu_step 'Verify repository exists and is DOCKER format'
  zrbga_http_json "GET" "${z_get_url}" "${z_token}" "${ZRBGA_INFIX_VERIFY_REPO}"
  zrbga_http_require_ok "Verify repository"         "${ZRBGA_INFIX_VERIFY_REPO}"
  test "$(zrbga_json_field_capture                  "${ZRBGA_INFIX_VERIFY_REPO}" '.format')" = "DOCKER" \
    || bcu_die "Repository exists but not DOCKER format"

  bcu_step 'Verify Cloud Build runtime SA is readable after propagation pause'
  local z_cb_sa="${z_project_number}@cloudbuild.gserviceaccount.com"
  local z_cb_sa_enc
  z_cb_sa_enc=$(zrbga_urlencode_capture "${z_cb_sa}") || bcu_die "Failed to encode SA email"
  local z_peek_code
  zrbga_http_json "GET" "${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/-/serviceAccounts/${z_cb_sa_enc}" \
                           "${z_token}" "${ZRBGA_INFIX_CB_RUNTIME_SA_PEEK}"
  z_peek_code=$(zrbga_http_code_capture "${ZRBGA_INFIX_CB_RUNTIME_SA_PEEK}") || z_peek_code="000"
  test "${z_peek_code}" = "200" || bcu_die "Cloud Build runtime SA not readable after fixed pause (HTTP ${z_peek_code})"

  bcu_step 'Grant Storage Object Admin to Cloud Build SA on bucket'
  zrbga_add_bucket_iam_role "${RBGC_GCS_BUCKET}" "${z_cb_sa}" "roles/storage.objectAdmin" "${z_token}"

  bcu_step 'Grant Artifact Registry Writer to Cloud Build SA on repo'
  zrbga_add_repo_iam_role "${z_cb_sa}" "${RBGC_GAR_LOCATION}" "${RBRR_GAR_REPOSITORY}" "${RBGC_ROLE_ARTIFACTREGISTRY_WRITER}"

  bcu_info "RBRA (admin): ${RBRR_ADMIN_RBRA_FILE}"
  bcu_info "GAR: ${RBGC_GAR_LOCATION}/${RBRR_GAR_REPOSITORY} (DOCKER)"
  bcu_info "Cloud Build SA granted writer on repo: ${z_cb_sa}"
  bcu_warn "RBRR file stashed. Consider deleting carriage JSON:"
  bcu_code ""
  bcu_code "    rm \"${z_json_path}\""
  bcu_code ""

  bcu_success 'Admin initialization complete'
}

rbga_destroy_admin() {
  zrbga_sentinel

  bcu_doc_brief "Destroy project-specific GAR resources and related repo-scoped IAM. Leaves project-wide APIs and SAs unchanged."
  bcu_doc_shown || return 0

  bcu_step 'Mint admin OAuth token'
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step 'Preflight: Determine if all required APIs enabled'
  local z_missing
  z_missing=$(zrbga_required_apis_missing_capture "${z_token}") || bcu_die "Failed to check API status"

  if test -n "${z_missing}"; then
    bcu_die "Required APIs not enabled: ${z_missing}. Run rbga_initialize_admin to enable them, then re-run destroy."
  fi

  bcu_step 'Confirm'
  bcu_require "Confirm full reset of this project?" "YES"
  bcu_require "Be very very sure!" "I-AM-SURE"

  bcu_step 'Discover Project Number Cloud Build SA (to prune repo binding cleanly)'

  zrbga_http_json "GET" "${RBGC_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGA_INFIX_PROJECT_INFO}"
  zrbga_http_require_ok "Get project info"                         "${ZRBGA_INFIX_PROJECT_INFO}"
  local z_project_number
  z_project_number=$(zrbga_json_field_capture "${ZRBGA_INFIX_PROJECT_INFO}" '.projectNumber') \
    || bcu_die "Failed to extract project number"

  local z_cb_sa_member="serviceAccount:${z_project_number}@cloudbuild.gserviceaccount.com"
  local z_resource="projects/${RBRR_GCP_PROJECT_ID}${RBGC_PATH_LOCATIONS}/${RBGC_GAR_LOCATION}${RBGC_PATH_REPOSITORIES}/${RBRR_GAR_REPOSITORY}"
  local z_get_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:getIamPolicy"
  local z_set_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:setIamPolicy"

  bcu_step 'Prune Cloud Build SA writer binding (idempotent; harmless if missing)'

  bcu_step 'Fetch current repo IAM policy'
  zrbga_http_json "POST" "${z_get_url}" "${z_token}" "${ZRBGA_INFIX_REPO_POLICY}" "${ZRBGA_EMPTY_JSON}"
  zrbga_http_require_ok "Get repo IAM policy"        "${ZRBGA_INFIX_REPO_POLICY}" 404 "repo not found (already deleted)"

  bcu_log_args 'Guard the prune+set when the repo is already gone'
  local z_get_code
  z_get_code=$(zrbga_http_code_capture "${ZRBGA_INFIX_REPO_POLICY}") || z_get_code="000"
  if test "${z_get_code}" = "404"; then
    bcu_warn "Repo missing; skip writer-binding prune."
  else
    bcu_step 'Strip Cloud Build SA from artifactregistry.writer binding'
    local z_updated_policy="${BDU_TEMP_DIR}/rbga_repo_policy_pruned.json"
    jq --arg role "${RBGC_ROLE_ARTIFACTREGISTRY_WRITER}" \
       --arg member "${z_cb_sa_member}" \
       '
         .bindings = (.bindings // []) |
         .bindings = [ .bindings[] |
           if .role == $role then .members = ((.members // []) | map(select(. != $member)))
           else . end
         ] |
         .bindings = [ .bindings[] | select((.members // []) | length > 0) ]
       ' "${ZRBGA_PREFIX}${ZRBGA_INFIX_REPO_POLICY}${ZRBGA_POSTFIX_JSON}" > "${z_updated_policy}" \
      || bcu_die "Failed to prune writer binding"

    local z_repo_set_body="${BDU_TEMP_DIR}/rbga_repo_set_policy_body.json"
    jq -n --slurpfile p "${z_updated_policy}" '{policy:$p[0]}' > "${z_repo_set_body}" \
      || bcu_die "Failed to build repo setIamPolicy body"

    zrbga_http_json "POST" "${z_set_url}" "${z_token}" "${ZRBGA_INFIX_RPOLICY_SET}" "${z_repo_set_body}"
    zrbga_http_require_ok "Set repo IAM policy"        "${ZRBGA_INFIX_RPOLICY_SET}"
  fi

  bcu_step 'Delete the GAR repository (removes remaining repo-scoped bindings/data)'

  bcu_step "Delete Artifact Registry repo '${RBRR_GAR_REPOSITORY}' in ${RBGC_GAR_LOCATION}"
  local z_delete_code
  zrbga_http_json "DELETE" \
    "${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}" \
                             "${z_token}" "${ZRBGA_INFIX_DELETE_REPO}"
  z_delete_code=$(zrbga_http_code_capture "${ZRBGA_INFIX_DELETE_REPO}") || z_delete_code="000"
  case "${z_delete_code}" in
    200|204) bcu_info "Repository deleted" ;;
    404)     bcu_warn "Repository not found (already deleted)" ;;
    *)
      local z_err
      z_err=$(zrbga_json_field_capture "${ZRBGA_INFIX_DELETE_REPO}" '.error.message') || z_err="Unknown error"
      bcu_die "Failed to delete repository: ${z_err}"
      ;;
  esac

  bcu_step 'Delete Cloud Storage bucket'
  zrbga_delete_gcs_bucket_predicate "${z_token}"  "${RBGC_GCS_BUCKET}"

  bcu_step 'Delete all service accounts except admin'

  bcu_log_args 'List all service accounts'
  zrbga_http_json "GET" "${RBGC_API_SERVICE_ACCOUNTS}" "${z_token}" "${ZRBGA_INFIX_LIST}"
  zrbga_http_require_ok "List service accounts" "${ZRBGA_INFIX_LIST}"

  bcu_log_args 'Extract emails to delete (all except admin)'
  local z_admin_email="${RBGC_ADMIN_ROLE}@${RBGC_SA_EMAIL_FULL}"
  local z_emails_to_delete
  z_emails_to_delete=$(jq -r --arg admin "${z_admin_email}" \
    '.accounts[]? | select(.email != $admin) | .email' \
    "${ZRBGA_PREFIX}${ZRBGA_INFIX_LIST}${ZRBGA_POSTFIX_JSON}") || z_emails_to_delete=""

  if test -n "${z_emails_to_delete}"; then
    local z_sa_email
    local z_del_code
    while IFS= read -r z_sa_email; do
      test -n "${z_sa_email}" || continue
      bcu_log_args "Deleting service account: ${z_sa_email}"

      zrbga_http_json "DELETE" "${RBGC_API_SERVICE_ACCOUNTS}/${z_sa_email}" \
        "${z_token}" "${ZRBGA_INFIX_DELETE}"

      z_del_code=$(zrbga_http_code_capture "${ZRBGA_INFIX_DELETE}") || z_del_code=""
      case "${z_del_code}" in
        200|204) bcu_log_args "Deleted: ${z_sa_email}" ;;
        404)     bcu_log_args "Already gone: ${z_sa_email}" ;;
        *)       bcu_warn "Failed to delete ${z_sa_email} (HTTP ${z_del_code}), continuing" ;;
      esac
    done <<< "${z_emails_to_delete}"
  fi

  bcu_step 'Waiting 45s for IAM deletion propagation'
  sleep 45

  bcu_success 'RBGA nuclear destruction complete (admin account preserved)'
}

rbga_list_service_accounts() {
  zrbga_sentinel

  bcu_doc_brief "List all service accounts in the project"
  bcu_doc_shown || return 0

  bcu_step 'Listing service accounts in project: '"${RBRR_GCP_PROJECT_ID}"

  bcu_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token (rc=$?)"

  bcu_log_args 'List service accounts via REST API'
  zrbga_http_json "GET" "${RBGC_API_SERVICE_ACCOUNTS}" "${z_token}" "${ZRBGA_INFIX_LIST}"
  zrbga_http_require_ok "List service accounts"                     "${ZRBGA_INFIX_LIST}"

  local z_count
  z_count=$(zrbga_json_field_capture "${ZRBGA_INFIX_LIST}" '.accounts | length') \
    || bcu_die "Failed to parse response"

  if test "${z_count}" = "0"; then
    bcu_info "No service accounts found in project"
    return 0
  fi

  bcu_step "Found ${z_count} service account(s):"

  local z_max_width
  z_max_width=$(jq -r '.accounts[].email | length' "${ZRBGA_PREFIX}${ZRBGA_INFIX_LIST}${ZRBGA_POSTFIX_JSON}" | sort -n | tail -1) \
    || bcu_die "Failed to calculate max width"

  jq -r --argjson width "${z_max_width}" \
    '.accounts[] | "  " + (.email | tostring | ((" " * ($width - length)) + .)) + " - " + (.displayName // "(no display name)")' \
    "${ZRBGA_PREFIX}${ZRBGA_INFIX_LIST}${ZRBGA_POSTFIX_JSON}" || bcu_die "Failed to format accounts"

  bcu_success "Service account listing completed"
}

rbga_create_retriever() {
  zrbga_sentinel

  local z_instance="${1:-}"

  bcu_doc_brief "Create Retriever service account instance"
  bcu_doc_param "instance" "Instance name (required)"
  bcu_doc_shown || return 0

  test -n "${z_instance}" || bcu_die "Instance name required"
  test -n "${BDU_OUTPUT_DIR}" || bcu_die "BDU_OUTPUT_DIR not set"
  test -d "${BDU_OUTPUT_DIR}" || bcu_die "BDU_OUTPUT_DIR does not exist: ${BDU_OUTPUT_DIR}"

  local z_account_name="rbga-retriever-${z_instance}"
  local z_account_email="${z_account_name}@${RBGC_SA_EMAIL_FULL}"

  bcu_step "Creating Retriever service account: ${z_account_name}"

  zrbga_create_service_account_with_key \
    "${z_account_name}" \
    "Recipe Bottle Retriever (${z_instance})" \
    "Read-only access to Google Artifact Registry - instance: ${z_instance}" \
    "${z_instance}"

  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step 'Adding Artifact Registry Reader role'
  zrbga_add_iam_role                        \
    "Grant Artifact Registry Reader"        \
    "${z_token}"                            \
    "${RBGC_PROJECT_RESOURCE}"              \
    "${RBGC_ROLE_ARTIFACTREGISTRY_READER}"  \
    "serviceAccount:${z_account_email}"     \
    "retriever-reader"

  local z_actual_rbra_file="${BDU_OUTPUT_DIR}/${z_instance}.rbra"

  bcu_step 'To install the RBRA file locally, run:'
  bcu_code ""
  bcu_code "    cp \"${z_actual_rbra_file}\" \"${RBRR_RETRIEVER_RBRA_FILE}\""
  bcu_code ""
  bcu_success "Retriever created successfully at -> ${z_actual_rbra_file}"
}

rbga_create_director() {
  zrbga_sentinel

  local z_instance="${1:-}"

  bcu_doc_brief "Create Director service account instance"
  bcu_doc_param "instance" "Instance name (required)"
  bcu_doc_shown || return 0

  test -n "${z_instance}" || bcu_die "Instance name required"
  test -n "${BDU_OUTPUT_DIR}" || bcu_die "BDU_OUTPUT_DIR not set"
  test -d "${BDU_OUTPUT_DIR}" || bcu_die "BDU_OUTPUT_DIR does not exist: ${BDU_OUTPUT_DIR}"

  local z_account_name="rbga-director-${z_instance}"
  local z_account_email="${z_account_name}@${RBGC_SA_EMAIL_FULL}"

  bcu_step "Creating Director service account: ${z_account_name}"

  zrbga_create_service_account_with_key                    \
    "${z_account_name}"                                    \
    "Recipe Bottle Director (${z_instance})"               \
    "Create/destroy container images for ${z_instance}"    \
    "${z_instance}"

  bcu_step 'Get OAuth token from admin'
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step 'Get project number for Cloud Build SA'
  zrbga_http_json "GET" "${RBGC_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGA_INFIX_PROJECT_INFO}"
  zrbga_http_require_ok "Get project info" "${ZRBGA_INFIX_PROJECT_INFO}"

  local z_project_number
  z_project_number=$(zrbga_json_field_capture "${ZRBGA_INFIX_PROJECT_INFO}" '.projectNumber') \
    || bcu_die "Failed to extract project number"

  bcu_step 'Adding Cloud Build Editor role (project scope)'
  zrbga_add_iam_role                        \
    "Grant Cloud Build Editor"              \
    "${z_token}"                            \
    "${RBGC_PROJECT_RESOURCE}"              \
    "${RBGC_ROLE_CLOUDBUILD_BUILDS_EDITOR}" \
    "serviceAccount:${z_account_email}"     \
    "director-cb"

  zrbga_add_iam_role                        \
    "Grant Project Viewer"                  \
    "${z_token}"                            \
    "${RBGC_PROJECT_RESOURCE}"              \
    "roles/viewer"                          \
    "serviceAccount:${z_account_email}"     \
    "director-viewer"

  bcu_step 'Grant serviceAccountUser on Cloud Build runtime SA'
  local z_cb_runtime_sa="${z_project_number}@cloudbuild.gserviceaccount.com"
  zrbga_add_sa_iam_role "${z_cb_runtime_sa}" "${z_account_email}" "roles/iam.serviceAccountUser"

  bcu_step 'Grant Artifact Registry Writer (repo-scoped)'
  zrbga_add_repo_iam_role "${z_account_email}" "${RBGC_GAR_LOCATION}" "${RBRR_GAR_REPOSITORY}" "${RBGC_ROLE_ARTIFACTREGISTRY_WRITER}"

  bcu_step 'Grant Artifact Registry Admin (repo-scoped) for delete in own repo'
  zrbga_add_repo_iam_role "${z_account_email}" "${RBGC_GAR_LOCATION}" "${RBRR_GAR_REPOSITORY}" "${RBGC_ROLE_ARTIFACTREGISTRY_ADMIN}"

  bcu_step 'Grant Storage Admin on Cloud Build artifacts bucket'
  zrbga_add_bucket_iam_role "${RBGC_GCS_BUCKET}" "${z_account_email}" \
                            "roles/storage.admin" "${z_token}"

  local z_actual_rbra_file="${BDU_OUTPUT_DIR}/${z_instance}.rbra"

  bcu_step 'To install the RBRA file locally, run:'
  bcu_code ""
  bcu_code "    cp \"${z_actual_rbra_file}\" \"${RBRR_DIRECTOR_RBRA_FILE}\""
  bcu_code ""
  bcu_success "Director created successfully at -> ${z_actual_rbra_file}"
}

rbga_delete_service_account() {
  zrbga_sentinel

  local z_sa_email="${1:-}"

  bcu_doc_brief "Delete a service account"
  bcu_doc_param "email" "Email address of the service account to delete"
  bcu_doc_shown || return 0

  test -n "${z_sa_email}" || bcu_die "Service account email required"

  bcu_step "Deleting service account: ${z_sa_email}"

  bcu_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_log_args 'Delete via REST API'
  zrbga_http_json "DELETE" "${RBGC_API_SERVICE_ACCOUNTS}/${z_sa_email}" "${z_token}" \
                                                 "${ZRBGA_INFIX_DELETE}"
  zrbga_http_require_ok "Delete service account" "${ZRBGA_INFIX_DELETE}" \
    404 "not found (already deleted)"

  bcu_success "Delete operation completed"
}

# eof

