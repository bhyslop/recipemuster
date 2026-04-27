#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# Recipe Bottle Google Verification - JWT SA and Payor OAuth access verification

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGV_SOURCED:-}" || buc_die "Module rbgv multiply sourced - check sourcing hierarchy"
ZRBGV_SOURCED=1

######################################################################
# Internal Functions (zrbgv_*)

zrbgv_kindle() {
  test -z "${ZRBGV_KINDLED:-}" || buc_die "Module rbgv already kindled"

  buc_log_args "Ensure dependencies are kindled first"
  zrbgc_sentinel
  zrbgo_sentinel
  zrbgu_sentinel
  zrbgp_sentinel

  # Kindle-constant temp file paths for forensic visibility
  readonly ZRBGV_AR_RESP_FILE="${BURD_TEMP_DIR}/rbgv_ar_resp.json"
  readonly ZRBGV_AR_CODE_FILE="${BURD_TEMP_DIR}/rbgv_ar_code.txt"
  readonly ZRBGV_AR_STDERR_FILE="${BURD_TEMP_DIR}/rbgv_ar_stderr.txt"
  readonly ZRBGV_CRM_RESP_FILE="${BURD_TEMP_DIR}/rbgv_crm_resp.json"
  readonly ZRBGV_CRM_CODE_FILE="${BURD_TEMP_DIR}/rbgv_crm_code.txt"
  readonly ZRBGV_CRM_STDERR_FILE="${BURD_TEMP_DIR}/rbgv_crm_stderr.txt"

  # Transient-5xx retry policy: 3 attempts with initial 2s backoff doubling each retry (worst-case +6s)
  readonly ZRBGV_HTTP_RETRY_ATTEMPTS=3
  readonly ZRBGV_HTTP_RETRY_INITIAL_DELAY_SEC=2

  readonly ZRBGV_KINDLED=1
}

zrbgv_sentinel() {
  test "${ZRBGV_KINDLED:-}" = "1" || buc_die "Module rbgv not kindled - call zrbgv_kindle first"
}

# Convert milliseconds to a decimal seconds string suitable for sleep
zrbgv_ms_to_sleep_capture() {
  zrbgv_sentinel

  local z_ms="${1}"

  buc_log_args "Converting ${z_ms}ms to sleep duration"

  # Compute whole seconds and remainder milliseconds using integer arithmetic
  local z_sec=$(( z_ms / 1000 ))
  local z_rem=$(( z_ms % 1000 ))

  # Format as decimal with three fractional digits (sleep accepts e.g. 1.500)
  printf '%d.%03d' "${z_sec}" "${z_rem}"
}

# Resolve the RBRA file path for a given role name
# Role names: governor | director | retriever
zrbgv_role_rbra_file_capture() {
  zrbgv_sentinel

  local z_role="${1}"

  case "${z_role}" in
    governor)
      test -n "${RBDC_GOVERNOR_RBRA_FILE:-}"  || buc_die "RBDC_GOVERNOR_RBRA_FILE is not set"
      printf '%s\n' "${RBDC_GOVERNOR_RBRA_FILE}"
      ;;
    director)
      test -n "${RBDC_DIRECTOR_RBRA_FILE:-}"  || buc_die "RBDC_DIRECTOR_RBRA_FILE is not set"
      printf '%s\n' "${RBDC_DIRECTOR_RBRA_FILE}"
      ;;
    retriever)
      test -n "${RBDC_RETRIEVER_RBRA_FILE:-}" || buc_die "RBDC_RETRIEVER_RBRA_FILE is not set"
      printf '%s\n' "${RBDC_RETRIEVER_RBRA_FILE}"
      ;;
    *)
      buc_die "Unknown role '${z_role}': expected governor | director | retriever"
      ;;
  esac
}

# HTTP GET with bounded exponential-backoff retry on transient 5xx responses.
# On any non-5xx response (including auth errors), populates z_code_file and returns.
# On curl-network failure or exhausted 5xx retries, buc_die. The caller evaluates
# the final HTTP status via its own case-switch on the populated z_code_file.
zrbgv_http_get_with_5xx_retry() {
  zrbgv_sentinel

  local -r z_label="${1}"
  local -r z_url="${2}"
  local -r z_token="${3}"
  local -r z_resp_file="${4}"
  local -r z_code_file="${5}"
  local -r z_stderr_file="${6}"

  local z_attempt=1
  local z_delay="${ZRBGV_HTTP_RETRY_INITIAL_DELAY_SEC}"
  local z_curl_status=0
  local z_code=""

  while test "${z_attempt}" -le "${ZRBGV_HTTP_RETRY_ATTEMPTS}"; do
    buc_log_args "${z_label}: HTTP GET attempt ${z_attempt}/${ZRBGV_HTTP_RETRY_ATTEMPTS}"

    z_curl_status=0
    curl                                                     \
        -sS                                                  \
        -X GET                                               \
        -H "Authorization: Bearer ${z_token}"                \
        -H "Accept: application/json"                        \
        -o "${z_resp_file}"                                  \
        -w "%{http_code}"                                    \
        --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
        --max-time "${RBCC_CURL_MAX_TIME_SEC}"               \
        "${z_url}" > "${z_code_file}"                        \
                   2> "${z_stderr_file}"                     \
      || z_curl_status=$?

    buc_log_args "${z_label}: curl exit status ${z_curl_status}"
    buc_log_pipe < "${z_stderr_file}"

    test "${z_curl_status}" -eq 0 \
      || buc_die "${z_label}: curl failed (network/SSL/DNS) — see ${z_stderr_file}"

    z_code=$(<"${z_code_file}") || buc_die "${z_label}: failed to read HTTP code file"
    test -n "${z_code}" || buc_die "${z_label}: empty HTTP code from curl"

    case "${z_code}" in
      500|502|503|504)
        if test "${z_attempt}" -lt "${ZRBGV_HTTP_RETRY_ATTEMPTS}"; then
          buc_step "${z_label}: transient HTTP ${z_code}, retrying in ${z_delay}s (attempt ${z_attempt}/${ZRBGV_HTTP_RETRY_ATTEMPTS})"
          sleep "${z_delay}"
          z_delay=$(( z_delay * 2 ))
          z_attempt=$(( z_attempt + 1 ))
        else
          buc_die "${z_label}: repeated transient HTTP ${z_code} after ${ZRBGV_HTTP_RETRY_ATTEMPTS} attempts — see ${z_resp_file}"
        fi
        ;;
      *)
        return 0
        ;;
    esac
  done
}

# Execute one Artifact Registry packages.list probe iteration for a JWT SA role.
# Writes response to ZRBGV_AR_RESP_FILE, HTTP code to ZRBGV_AR_CODE_FILE,
# and stderr to ZRBGV_AR_STDERR_FILE (kindle-constant paths for forensic visibility).
zrbgv_jwt_ar_probe_once() {
  zrbgv_sentinel

  local z_role="${1}"
  local z_iteration="${2}"

  buc_log_args "JWT SA probe iteration ${z_iteration} for role '${z_role}'"

  buc_log_args "Resolve RBRA file for role"
  local z_rbra_file
  z_rbra_file=$(zrbgv_role_rbra_file_capture "${z_role}") || return 1

  buc_log_args "Validate RBRA file exists"
  test -f "${z_rbra_file}" || buc_die "RBRA file not found for role ${z_role}: ${z_rbra_file}"

  buc_log_args "Exchange JWT for OAuth token"
  local z_token
  z_token=$(rbgo_get_token_capture "${z_rbra_file}") \
    || buc_die "Failed to obtain OAuth token for role ${z_role} (iteration ${z_iteration})"
  test -n "${z_token}" || buc_die "Empty OAuth token for role ${z_role} (iteration ${z_iteration})"

  buc_log_args "Build Artifact Registry packages.list URL"
  test -n "${RBDC_DEPOT_PROJECT_ID:-}" || buc_die "RBDC_DEPOT_PROJECT_ID is not set"
  test -n "${RBRR_GCP_REGION:-}"       || buc_die "RBRR_GCP_REGION is not set"
  test -n "${RBDC_GAR_REPOSITORY:-}"   || buc_die "RBDC_GAR_REPOSITORY is not set"

  local -r z_ar_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/projects/${RBDC_DEPOT_PROJECT_ID}/locations/${RBRR_GCP_REGION}/repositories/${RBDC_GAR_REPOSITORY}/packages"
  local -r z_ar_label="JWT SA probe [${z_role}] iteration ${z_iteration}"

  buc_log_args "Call Artifact Registry packages.list (with transient-5xx retry)"
  zrbgv_http_get_with_5xx_retry \
    "${z_ar_label}"             \
    "${z_ar_url}"               \
    "${z_token}"                \
    "${ZRBGV_AR_RESP_FILE}"     \
    "${ZRBGV_AR_CODE_FILE}"     \
    "${ZRBGV_AR_STDERR_FILE}"

  local z_code
  z_code=$(<"${ZRBGV_AR_CODE_FILE}") || buc_die "Failed to read AR HTTP code file"
  test -n "${z_code}"                || buc_die "Empty HTTP code from AR curl"

  buc_log_args "AR packages.list HTTP ${z_code} for role ${z_role} iteration ${z_iteration}"

  case "${z_code}" in
    200|206)
      buc_step "JWT SA probe [${z_role}] iteration ${z_iteration}: OK (HTTP ${z_code})"
      ;;
    401|403)
      buc_die "JWT SA probe [${z_role}] iteration ${z_iteration}: access denied (HTTP ${z_code})"
      ;;
    *)
      local z_err=""
      if jq -e . "${ZRBGV_AR_RESP_FILE}" >/dev/null 2>&1; then
        z_err=$(jq -r '.error.message // "Unknown error"' "${ZRBGV_AR_RESP_FILE}" 2>/dev/null) || z_err="Unknown error"
      else
        z_err="Non-JSON response (HTTP ${z_code})"
      fi
      buc_die "JWT SA probe [${z_role}] iteration ${z_iteration}: unexpected HTTP ${z_code}: ${z_err}"
      ;;
  esac
}

# Execute one CRM projects.get probe iteration for the Payor OAuth flow.
# Writes response to ZRBGV_CRM_RESP_FILE, HTTP code to ZRBGV_CRM_CODE_FILE,
# and stderr to ZRBGV_CRM_STDERR_FILE (kindle-constant paths for forensic visibility).
zrbgv_payor_crm_probe_once() {
  zrbgv_sentinel

  local z_iteration="${1}"

  buc_log_args "Payor OAuth probe iteration ${z_iteration}"

  buc_log_args "Authenticate via Payor OAuth refresh token flow"
  local z_token
  z_token=$(zrbgp_authenticate_capture) \
    || buc_die "Failed to obtain Payor OAuth access token (iteration ${z_iteration})"
  test -n "${z_token}" || buc_die "Empty Payor OAuth access token (iteration ${z_iteration})"

  buc_log_args "Build CRM projects.get URL"
  test -n "${RBRP_PAYOR_PROJECT_ID:-}" || buc_die "RBRP_PAYOR_PROJECT_ID is not set"

  local -r z_crm_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/projects/${RBRP_PAYOR_PROJECT_ID}"
  local -r z_crm_label="Payor OAuth probe iteration ${z_iteration}"

  buc_log_args "Call CRM projects.get on payor project (with transient-5xx retry)"
  zrbgv_http_get_with_5xx_retry \
    "${z_crm_label}"            \
    "${z_crm_url}"              \
    "${z_token}"                \
    "${ZRBGV_CRM_RESP_FILE}"    \
    "${ZRBGV_CRM_CODE_FILE}"    \
    "${ZRBGV_CRM_STDERR_FILE}"

  local z_code
  z_code=$(<"${ZRBGV_CRM_CODE_FILE}") || buc_die "Failed to read CRM HTTP code file"
  test -n "${z_code}"                 || buc_die "Empty HTTP code from CRM curl"

  buc_log_args "CRM projects.get HTTP ${z_code} for Payor iteration ${z_iteration}"

  case "${z_code}" in
    200)
      buc_step "Payor OAuth probe iteration ${z_iteration}: OK (HTTP ${z_code})"
      ;;
    401|403)
      buc_die "Payor OAuth probe iteration ${z_iteration}: access denied (HTTP ${z_code})"
      ;;
    *)
      local z_err=""
      if jq -e . "${ZRBGV_CRM_RESP_FILE}" >/dev/null 2>&1; then
        z_err=$(jq -r '.error.message // "Unknown error"' "${ZRBGV_CRM_RESP_FILE}" 2>/dev/null) || z_err="Unknown error"
      else
        z_err="Non-JSON response (HTTP ${z_code})"
      fi
      buc_die "Payor OAuth probe iteration ${z_iteration}: unexpected HTTP ${z_code}: ${z_err}"
      ;;
  esac
}

######################################################################
# External Functions (rbgv_*)

# Probe 1: JWT SA Access Probe
#
# For each iteration:
#   1. Exchange JWT for OAuth token via rbgo_get_token_capture with role's RBRA file
#   2. Call Artifact Registry packages.list to verify token grants read access
#   3. Sleep for configured delay
#
# All three roles have read access:
#   Governor  = Owner
#   Director  = repoAdmin
#   Retriever = reader
rbgv_jwt_sa_probe() {
  zrbgv_sentinel

  local z_role="${1:-}"
  local z_count="${2:-1}"
  local z_delay_ms="${3:-0}"

  buc_doc_brief "Probe JWT service account access for a given role against Artifact Registry"
  buc_doc_param "role"     "Role to probe: governor | director | retriever"
  buc_doc_param "count"    "Number of iterations (default: 1)"
  buc_doc_param "delay_ms" "Milliseconds to sleep between iterations (default: 0)"
  buc_doc_shown || return 0

  test -n "${z_role}"  || buc_die "role parameter required (governor | director | retriever)"
  test -n "${z_count}" || buc_die "count parameter required"

  buc_step "JWT SA access probe: role=${z_role} count=${z_count} delay_ms=${z_delay_ms}"

  local z_iter=1
  while test "${z_iter}" -le "${z_count}"; do
    buc_step "JWT SA probe [${z_role}] iteration ${z_iter}/${z_count}"

    zrbgv_jwt_ar_probe_once "${z_role}" "${z_iter}"

    if test "${z_iter}" -lt "${z_count}" && test "${z_delay_ms}" -gt 0; then
      local z_sleep
      z_sleep=$(zrbgv_ms_to_sleep_capture "${z_delay_ms}")
      buc_log_args "Sleeping ${z_sleep}s (${z_delay_ms}ms) before next iteration"
      sleep "${z_sleep}"
    fi

    z_iter=$(( z_iter + 1 ))
  done

  buc_step "JWT SA access probe complete: role=${z_role} iterations=${z_count}"
}

# Probe 2: Payor OAuth Access Probe
#
# For each iteration:
#   1. Authenticate via zrbgp_authenticate_capture (RBRO refresh token -> access token)
#   2. Call CRM projects.get on payor project to verify token works
#   3. Sleep for configured delay
rbgv_payor_oauth_probe() {
  zrbgv_sentinel

  local z_count="${1:-1}"
  local z_delay_ms="${2:-0}"

  buc_doc_brief "Probe Payor OAuth access against Cloud Resource Manager (CRM) projects.get"
  buc_doc_param "count"    "Number of iterations (default: 1)"
  buc_doc_param "delay_ms" "Milliseconds to sleep between iterations (default: 0)"
  buc_doc_shown || return 0

  test -n "${z_count}" || buc_die "count parameter required"

  buc_step "Payor OAuth access probe: count=${z_count} delay_ms=${z_delay_ms}"

  local z_iter=1
  while test "${z_iter}" -le "${z_count}"; do
    buc_step "Payor OAuth probe iteration ${z_iter}/${z_count}"

    zrbgv_payor_crm_probe_once "${z_iter}"

    if test "${z_iter}" -lt "${z_count}" && test "${z_delay_ms}" -gt 0; then
      local z_sleep
      z_sleep=$(zrbgv_ms_to_sleep_capture "${z_delay_ms}")
      buc_log_args "Sleeping ${z_sleep}s (${z_delay_ms}ms) before next iteration"
      sleep "${z_sleep}"
    fi

    z_iter=$(( z_iter + 1 ))
  done

  buc_step "Payor OAuth access probe complete: iterations=${z_count}"
}

# eof
