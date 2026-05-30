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
# Recipe Bottle Google REST - LRO polling and API-enable patterns over rbuh

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGE_SOURCED:-}" || buc_die "Module rbge multiply sourced - check sourcing hierarchy"
ZRBGE_SOURCED=1

######################################################################
# Internal Functions (zrbge_*)

zrbge_kindle() {
  test -z "${ZRBGE_KINDLED:-}" || buc_die "Module rbge already kindled"

  # Ensure dependency kindled first (rbuh owns the HTTP temp-file machinery rbge consumes)
  zrbuh_sentinel

  readonly ZRBGE_KINDLED=1
}

zrbge_sentinel() {
  test "${ZRBGE_KINDLED:-}" = "1" || buc_die "Module rbge not kindled - call zrbge_kindle first"
}

######################################################################
# Capture Functions

rbge_error_message_capture() {
  zrbge_sentinel
  local -r z_infix="${1:-}"
  test -n "${z_infix}" || return 1

  if rbuh_json_valid_predicate "${z_infix}"; then
    rbuh_json_field_capture "${z_infix}" '.error.message' 2>/dev/null || return 1
  else
    return 1
  fi
}

######################################################################
# External / RBTOE Pattern Functions

# Predicate: Check if resource was newly created and apply propagation delay
rbge_newly_created_delay() {
  zrbge_sentinel

  local -r z_infix="${1}"
  local -r z_resource="${2}"
  local -r z_delay="${3}"

  local z_code
  z_code=$(rbuh_code_capture "${z_infix}") || return 1

  if test "${z_code}" = "200" || test "${z_code}" = "201"; then
    buc_step "Resource ${z_resource} newly created, waiting ${z_delay}s for propagation"
    sleep "${z_delay}"
  fi
}

# POST + strict LRO handling (no heuristics)
rbge_lro_ok() {
  zrbge_sentinel

  local -r z_label="${1}"
  local -r z_token="${2}"
  local -r z_post_url="${3}"
  local -r z_infix="${4}"
  local -r z_body="${5}"
  local -r z_name_jq="${6}"
  local -r z_poll_root="${7}"
  local -r z_op_prefix="${8}"
  local -r z_poll_interval="${9:-${RBGC_EVENTUAL_CONSISTENCY_SEC}}"
  local -r z_timeout="${10:-${RBGC_MAX_CONSISTENCY_SEC}}"

  buc_log_args '1) POST the request'
  rbuh_json "POST" "${z_post_url}" "${z_token}" "${z_infix}" "${z_body}"
  rbuh_require_ok "${z_label}" "${z_infix}"

  local z_done=""
  z_done=$(rbuh_json_field_capture "${z_infix}" ".done") || z_done=""
  test "${z_done}" = "true" && {
    local z_lro_error=""
    z_lro_error=$(rbuh_json_field_capture "${z_infix}" '.error.message // empty') || z_lro_error=""
    if test -n "${z_lro_error}"; then
      local z_lro_resp_file="${ZRBUH_PREFIX}${z_infix}${ZRBUH_POSTFIX_JSON}"
      buc_warn "${z_label}: LRO completed with error — response saved: ${z_lro_resp_file}"
      buc_die "${z_label}: ${z_lro_error}"
    fi
    buc_log_args 'Immediate-done response -> success (no polling)'
    return 0
  }

  buc_log_args '2) Extract op name (or return if not an LRO)'
  local z_name
  z_name=$(rbuh_json_field_capture "${z_infix}" "${z_name_jq}") || z_name=""
  test -n "${z_name}" || {
    buc_log_args 'No LRO name present - treat as non-LRO success'
    return 0
  }
  buc_log_args '3) Build poll URL based on operation name format'
  local z_poll_url
  if [[ "${z_name}" =~ ^projects/.*/locations/.*/operations/ ]]; then
    buc_log_args '  Regional operation with fully-qualified name'
    z_poll_url="${z_poll_root}/${z_name}"
  elif [[ "${z_name}" =~ ^projects/.*/operations/ ]]; then
    buc_log_args '  Global operation with project prefix'
    z_poll_url="${z_poll_root}/${z_name}"
  elif test -n "${z_op_prefix}" && [[ ! "${z_name}" =~ ^${z_op_prefix} ]]; then
    buc_log_args '  Legacy format - apply prefix (not already present)'
    z_poll_url="${z_poll_root}/${z_op_prefix}${z_name}"
  else
    buc_log_args '  Use name as-is under versioned root'
    z_poll_url="${z_poll_root}/${z_name}"
  fi
  buc_log_args "Poll URL: ${z_poll_url}"

  buc_log_args '4) Poll until done or timeout'
  local z_elapsed=0
  while :; do
    sleep "${z_poll_interval}"
    z_elapsed=$((z_elapsed + z_poll_interval))

    local z_poll_infix="${z_infix}-poll-${z_elapsed}s"
    rbuh_json "GET" "${z_poll_url}" "${z_token}" "${z_poll_infix}"

    local z_code=""
    z_code=$(rbuh_code_capture "${z_poll_infix}") || z_code=""
    test "${z_code}" = "200" || buc_die "${z_label}: poll failed (HTTP ${z_code})"

    z_done=$(rbuh_json_field_capture "${z_poll_infix}" ".done") || z_done=""
    test "${z_done}" = "true" && {
      local z_lro_error=""
      z_lro_error=$(rbuh_json_field_capture "${z_poll_infix}" '.error.message // empty') || z_lro_error=""
      if test -n "${z_lro_error}"; then
        local z_lro_resp_file="${ZRBUH_PREFIX}${z_poll_infix}${ZRBUH_POSTFIX_JSON}"
        buc_warn "${z_label}: LRO completed with error — response saved: ${z_lro_resp_file}"
        buc_die "${z_label}: ${z_lro_error}"
      fi
      buc_log_args "${z_label}: operation completed after ${z_elapsed}s"
      return 0
    }

    test "${z_elapsed}" -ge "${z_timeout}" && buc_die "${z_label}: timeout after ${z_timeout}s"
    buc_log_args "Still running at ${z_elapsed}s..."
  done
}

# RBTOE: API Enable Pattern
# Ensures a specified Google Cloud API is enabled in a project with idempotent behavior
rbge_api_enable() {
  zrbge_sentinel

  local -r z_api_service="${1}"
  local -r z_project_id="${2}"
  local -r z_token="${3}"

  test -n "${z_api_service}" || buc_die "rbge_api_enable: API service name required"
  test -n "${z_project_id}" || buc_die "rbge_api_enable: project ID required"
  test -n "${z_token}" || buc_die "rbge_api_enable: access token required"

  buc_log_args "Enabling API ${z_api_service} in project ${z_project_id}"

  local -r z_infix="api-enable-${z_api_service}"
  local -r z_enable_url="https://serviceusage.googleapis.com/v1/projects/${z_project_id}/services/${z_api_service}.googleapis.com:enable"

  # Attempt to enable the API
  rbuh_json "POST" "${z_enable_url}" "${z_token}" "${z_infix}" ""

  local z_code
  z_code=$(rbuh_code_capture "${z_infix}") || buc_die "rbge_api_enable: failed to read HTTP code"

  case "${z_code}" in
    200|201|204)
      buc_log_args "API enable request successful (HTTP ${z_code})"
      ;;
    400)
      # Check if already enabled
      local z_err
      z_err=$(rbge_error_message_capture "${z_infix}") || z_err="Unknown error"
      if [[ "${z_err}" =~ already.enabled ]] || [[ "${z_err}" =~ "already enabled" ]]; then
        buc_log_args "API ${z_api_service} already enabled"
        return 0
      else
        buc_die "rbge_api_enable (HTTP ${z_code}): ${z_err}"
      fi
      ;;
    *)
      local z_err
      z_err=$(rbge_error_message_capture "${z_infix}") || z_err="Unknown error"
      buc_die "rbge_api_enable (HTTP ${z_code}): ${z_err}"
      ;;
  esac

  # Check if this returned an LRO that needs polling
  local z_operation_name
  z_operation_name=$(rbuh_json_field_capture "${z_infix}" ".name") || z_operation_name=""

  if test -n "${z_operation_name}"; then
    buc_log_args "API enable returned LRO, polling for completion"
    # Use the LRO polling mechanism
    local -r z_poll_root="https://serviceusage.googleapis.com/v1"
    rbge_lro_ok \
      "API Enable ${z_api_service}" \
      "${z_token}" \
      "${z_enable_url}" \
      "${z_infix}" \
      "" \
      ".name" \
      "${z_poll_root}" \
      ""
  fi

  # Verify API is enabled
  local -r z_verify_infix="api-verify-${z_api_service}"
  local -r z_verify_url="https://serviceusage.googleapis.com/v1/projects/${z_project_id}/services/${z_api_service}.googleapis.com"

  rbuh_json "GET" "${z_verify_url}" "${z_token}" "${z_verify_infix}" ""
  rbuh_require_ok "API Enable Verify ${z_api_service}" "${z_verify_infix}"

  local z_state
  z_state=$(rbuh_json_field_capture "${z_verify_infix}" ".state") || buc_die "Failed to read API state"

  if test "${z_state}" != "ENABLED"; then
    buc_die "API ${z_api_service} not enabled after request (state: ${z_state})"
  fi

  buc_log_args "API ${z_api_service} confirmed enabled"
}

# eof
