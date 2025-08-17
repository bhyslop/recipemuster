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
  ZRBGA_INFIX_API_STORAGE_ENABLE="api_storage_enable"
  ZRBGA_INFIX_API_STORAGE_VERIFY="api_storage_verify"
  ZRBGA_INFIX_PROJECT_INFO="project_info"
  ZRBGA_INFIX_CREATE_REPO="create_repo"
  ZRBGA_INFIX_DELETE_REPO="delete_repo"
  ZRBGA_INFIX_LIST="list"
  ZRBGA_INFIX_DELETE="delete"

  ZRBGA_POSTFIX_JSON="_response.json"
  ZRBGA_POSTFIX_CODE="_code.txt"

  ZRBGA_KINDLED=1
}

zrbga_sentinel() {
  test "${ZRBGA_KINDLED:-}" = "1" || bcu_die "Module rbga not kindled - call zrbga_kindle first"
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

  local z_err
  z_err=$(zrbga_json_field_capture "${z_infix}" '.error.message') || z_err="Parse error"
  bcu_die "${z_ctx} (HTTP ${z_code}): ${z_err}"
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
  zrbga_http_require_ok "Create service account" "${ZRBGA_INFIX_CREATE}"

  bcu_info "Service account created: ${z_account_email}"

  local z_service_prop_s="15"
  bcu_step 'Wait '"${z_service_prop_s}"' for service account to propagate'
  sleep           "${z_service_prop_s}"
  zrbga_http_json "GET" "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email}" "${z_token}" "${ZRBGA_INFIX_VERIFY}"
  zrbga_http_require_ok "Verify service account"                                       "${ZRBGA_INFIX_VERIFY}"

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

zrbga_add_iam_role() {
  zrbga_sentinel

  local z_account_email="$1"
  local z_role="$2"

  bcu_log_args "Adding IAM role ${z_role} to ${z_account_email}"

  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_log_args 'Get current IAM policy'
  zrbga_http_json "POST" "${RBGC_API_CRM_GET_IAM_POLICY}" "${z_token}" \
    "${ZRBGA_INFIX_ROLE}" "${ZRBGA_EMPTY_JSON}"

  zrbga_http_require_ok "Get IAM policy" "${ZRBGA_INFIX_ROLE}"

  bcu_log_args 'Update IAM policy with new role binding'
  local z_updated_policy="${BDU_TEMP_DIR}/rbga_updated_policy.json"

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
     ' "${ZRBGA_PREFIX}${ZRBGA_INFIX_ROLE}${ZRBGA_POSTFIX_JSON}" \
     > "${z_updated_policy}" || bcu_die "Failed to update IAM policy"

  bcu_log_args 'Set updated IAM policy'
  local z_set_body="${BDU_TEMP_DIR}/rbga_set_policy_body.json"
  jq -n --slurpfile p "${z_updated_policy}" '{policy:$p[0]}' > "${z_set_body}" \
    || bcu_die "Failed to build setIamPolicy body"
  zrbga_http_json "POST" "${RBGC_API_CRM_SET_IAM_POLICY}" "${z_token}" \
    "${ZRBGA_INFIX_ROLE_SET}" "${z_set_body}"

  zrbga_http_require_ok "Set IAM policy" "${ZRBGA_INFIX_ROLE_SET}"

  bcu_log_args 'Successfully added role' "${z_role}"
}

zrbga_add_repo_iam_role() {
  zrbga_sentinel

  local z_account_email="${1:-}"
  local z_location="${2:-}"
  local z_repository="${3:-}"
  local z_role="${4:-}"

  test -n "${z_account_email}" || bcu_die "Service account email required"
  test -n "${z_location}"      || bcu_die "RBRR_GAR_LOCATION is required"
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

  zrbga_http_require_ok "Get repo IAM policy" "${ZRBGA_INFIX_REPO_ROLE}"

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


######################################################################
# External Functions (rbga_*)
rbga_initialize_admin() {
  zrbga_sentinel

  local z_json_path="${1:-}"

  bcu_doc_brief "Initialize RBGA for this project: enable/verify APIs, create GAR repo, and grant Cloud Build SA."
  bcu_doc_param "json_path" "Path to downloaded admin JSON key (converted to RBRA)"
  bcu_doc_shown || return 0

  test -n "${z_json_path}" || bcu_die "First argument must be path to downloaded JSON key file."

  bcu_step 'Convert admin JSON to RBRA'
  zrbga_extract_json_to_rbra "${z_json_path}" "${RBRR_ADMIN_RBRA_FILE}" "1800"

  bcu_step 'Mint admin OAuth token'
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  ######################################################################
  # Enable required APIs (idempotent)
  bcu_step 'Enable IAM API'
  zrbga_http_json "POST" "${RBGC_API_SERVICEUSAGE_ENABLE_IAM}" "${z_token}" \
    "${ZRBGA_INFIX_API_IAM_ENABLE}" "${ZRBGA_EMPTY_JSON}"
  zrbga_http_require_ok "Enable IAM API" "${ZRBGA_INFIX_API_IAM_ENABLE}" 409 "already enabled"

  bcu_step 'Enable Cloud Resource Manager API'
  zrbga_http_json "POST" "${RBGC_API_SERVICEUSAGE_ENABLE_CRM}" "${z_token}" \
    "${ZRBGA_INFIX_API_CRM_ENABLE}" "${ZRBGA_EMPTY_JSON}"
  zrbga_http_require_ok "Enable Cloud Resource Manager API" "${ZRBGA_INFIX_API_CRM_ENABLE}" 409 "already enabled"

  bcu_step 'Enable Artifact Registry API'
  zrbga_http_json "POST" "${RBGC_API_SERVICEUSAGE_ENABLE_ARTIFACTREGISTRY}" "${z_token}" \
    "${ZRBGA_INFIX_API_ART_ENABLE}" "${ZRBGA_EMPTY_JSON}"
  zrbga_http_require_ok "Enable Artifact Registry API" "${ZRBGA_INFIX_API_ART_ENABLE}" 409 "already enabled"

  bcu_step 'Enable Cloud Build API'
  zrbga_http_json "POST" \
    "https://serviceusage.googleapis.com/v1/projects/${RBRR_GCP_PROJECT_ID}/services/cloudbuild.googleapis.com:enable" \
    "${z_token}" "${ZRBGA_INFIX_API_BUILD_ENABLE}" "${ZRBGA_EMPTY_JSON}"
  zrbga_http_require_ok "Enable Cloud Build API" "${ZRBGA_INFIX_API_BUILD_ENABLE}" 409 "already enabled"

  bcu_step 'Enable Container Analysis API'
  zrbga_http_json "POST" \
    "https://serviceusage.googleapis.com/v1/projects/${RBRR_GCP_PROJECT_ID}/services/containeranalysis.googleapis.com:enable" \
    "${z_token}" "api_containeranalysis_enable" "${ZRBGA_EMPTY_JSON}"
  zrbga_http_require_ok "Enable Container Analysis API" "api_containeranalysis_enable" 409 "already enabled"

  bcu_step 'Enable Cloud Storage API (build bucket deps)'
  zrbga_http_json "POST" \
    "https://serviceusage.googleapis.com/v1/projects/${RBRR_GCP_PROJECT_ID}/services/storage.googleapis.com:enable" \
    "${z_token}" "${ZRBGA_INFIX_API_STORAGE_ENABLE}" "${ZRBGA_EMPTY_JSON}"
  zrbga_http_require_ok "Enable Cloud Storage API" "${ZRBGA_INFIX_API_STORAGE_ENABLE}" 409 "already enabled"

  local z_prop_delay_seconds=45
  bcu_step "Wait ${z_prop_delay_seconds}s for API propagation"
  sleep "${z_prop_delay_seconds}"

  ######################################################################
  # Verify enablement
  bcu_step 'Verify IAM API'
  zrbga_http_json "GET" "${RBGC_API_SERVICEUSAGE_VERIFY_IAM}" "${z_token}" "${ZRBGA_INFIX_API_IAM_VERIFY}"
  zrbga_http_require_ok "Verify IAM API" "${ZRBGA_INFIX_API_IAM_VERIFY}"
  test "$(zrbga_json_field_capture "${ZRBGA_INFIX_API_IAM_VERIFY}" '.state')" = "ENABLED" || bcu_die "IAM not enabled"

  bcu_step 'Verify Cloud Resource Manager API'
  zrbga_http_json "GET" "${RBGC_API_SERVICEUSAGE_VERIFY_CRM}" "${z_token}" "${ZRBGA_INFIX_API_CRM_VERIFY}"
  zrbga_http_require_ok "Verify Cloud Resource Manager API" "${ZRBGA_INFIX_API_CRM_VERIFY}"
  test "$(zrbga_json_field_capture "${ZRBGA_INFIX_API_CRM_VERIFY}" '.state')" = "ENABLED" || bcu_die "CRM not enabled"

  bcu_step 'Verify Artifact Registry API'
  zrbga_http_json "GET" "${RBGC_API_SERVICEUSAGE_VERIFY_ARTIFACTREGISTRY}" "${z_token}" "${ZRBGA_INFIX_API_ART_VERIFY}"
  zrbga_http_require_ok "Verify Artifact Registry API" "${ZRBGA_INFIX_API_ART_VERIFY}"
  test "$(zrbga_json_field_capture "${ZRBGA_INFIX_API_ART_VERIFY}" '.state')" = "ENABLED" || bcu_die "AR not enabled"

  bcu_step 'Verify Cloud Build API'
  zrbga_http_json "GET" \
    "https://serviceusage.googleapis.com/v1/projects/${RBRR_GCP_PROJECT_ID}/services/cloudbuild.googleapis.com" \
    "${z_token}" "${ZRBGA_INFIX_API_BUILD_VERIFY}"
  zrbga_http_require_ok "Verify Cloud Build API" "${ZRBGA_INFIX_API_BUILD_VERIFY}"
  test "$(zrbga_json_field_capture "${ZRBGA_INFIX_API_BUILD_VERIFY}" '.state')" = "ENABLED" || bcu_die "Cloud Build not enabled"

  bcu_step 'Verify Container Analysis API'
  zrbga_http_json "GET" \
    "https://serviceusage.googleapis.com/v1/projects/${RBRR_GCP_PROJECT_ID}/services/containeranalysis.googleapis.com" \
    "${z_token}" "api_containeranalysis_verify"
  zrbga_http_require_ok "Verify Container Analysis API" "api_containeranalysis_verify"
  test "$(zrbga_json_field_capture "api_containeranalysis_verify" '.state')" = "ENABLED" || bcu_die "Container Analysis not enabled"

  bcu_step 'Verify Cloud Storage API'
  zrbga_http_json "GET" \
    "https://serviceusage.googleapis.com/v1/projects/${RBRR_GCP_PROJECT_ID}/services/storage.googleapis.com" \
    "${z_token}" "${ZRBGA_INFIX_API_STORAGE_VERIFY}"
  zrbga_http_require_ok "Verify Cloud Storage API" "${ZRBGA_INFIX_API_STORAGE_VERIFY}"
  test "$(zrbga_json_field_capture "${ZRBGA_INFIX_API_STORAGE_VERIFY}" '.state')" = "ENABLED" || bcu_die "Cloud Storage not enabled"

  ######################################################################
  # Create primary GAR repository (Docker), then verify format
  bcu_step "Create Artifact Registry repo '${RBRR_GAR_REPOSITORY}' in ${RBRR_GAR_LOCATION}"
  local z_repo_body="${BDU_TEMP_DIR}/rbga_create_repo.json"
  jq -n --arg format "DOCKER" '{format: $format}' > "${z_repo_body}"
  zrbga_http_json "POST" \
    "${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/projects/${RBRR_GCP_PROJECT_ID}${RBGC_PATH_LOCATIONS}/${RBRR_GAR_LOCATION}${RBGC_PATH_REPOSITORIES}?repositoryId=${RBRR_GAR_REPOSITORY}" \
    "${z_token}" "${ZRBGA_INFIX_CREATE_REPO}" "${z_repo_body}"
  zrbga_http_require_ok "Create repository" "${ZRBGA_INFIX_CREATE_REPO}" 409 "already exists"

  bcu_step 'Verify repository exists and is DOCKER format'
  zrbga_http_json "GET" \
    "${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/projects/${RBRR_GCP_PROJECT_ID}${RBGC_PATH_LOCATIONS}/${RBRR_GAR_LOCATION}${RBGC_PATH_REPOSITORIES}/${RBRR_GAR_REPOSITORY}" \
    "${z_token}" "repo_verify"
  zrbga_http_require_ok "Verify repository" "repo_verify"
  test "$(zrbga_json_field_capture repo_verify '.format')" = "DOCKER" || bcu_die "Repo exists but not DOCKER format"

  ######################################################################
  # Compute Cloud Build SA and grant repo-scoped writer
  bcu_step 'Discover Project Number'
  zrbga_http_json "GET" "${RBGC_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGA_INFIX_PROJECT_INFO}"
  zrbga_http_require_ok "Get project info" "${ZRBGA_INFIX_PROJECT_INFO}"
  local z_project_number
  z_project_number=$(zrbga_json_field_capture "${ZRBGA_INFIX_PROJECT_INFO}" '.projectNumber') \
    || bcu_die "Failed to extract project number"

  local z_cb_sa_email="${z_project_number}@cloudbuild.gserviceaccount.com"
  bcu_step "Grant Artifact Registry Writer to Cloud Build SA on repo"
  zrbga_add_repo_iam_role "${z_cb_sa_email}" "${RBRR_GAR_LOCATION}" "${RBRR_GAR_REPOSITORY}" "${RBGC_ROLE_ARTIFACTREGISTRY_WRITER}"

  ######################################################################
  # Summary
  bcu_success "Admin initialization complete"
  bcu_info    "RBRA (admin): ${RBRR_ADMIN_RBRA_FILE}"
  bcu_info    "GAR: ${RBRR_GAR_LOCATION}/${RBRR_GAR_REPOSITORY} (DOCKER)"
  bcu_info    "Cloud Build SA granted writer on repo: ${z_cb_sa_email}"
  bcu_warn    "Consider deleting source JSON after verification: ${z_json_path}"
}

rbga_destroy_admin() {
  zrbga_sentinel

  bcu_doc_brief "Destroy project-specific GAR resources and related repo-scoped IAM. Leaves project-wide APIs and SAs unchanged."
  bcu_doc_shown || return 0

  bcu_step 'Mint admin OAuth token'
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  ######################################################################
  # Discover Project Number for Cloud Build SA (to prune repo binding cleanly)
  bcu_step 'Discover Project Number'
  zrbga_http_json "GET" "${RBGC_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGA_INFIX_PROJECT_INFO}"
  zrbga_http_require_ok "Get project info" "${ZRBGA_INFIX_PROJECT_INFO}"
  local z_project_number
  z_project_number=$(zrbga_json_field_capture "${ZRBGA_INFIX_PROJECT_INFO}" '.projectNumber') \
    || bcu_die "Failed to extract project number"

  local z_cb_sa_member="serviceAccount:${z_project_number}@cloudbuild.gserviceaccount.com"
  local z_resource="projects/${RBRR_GCP_PROJECT_ID}${RBGC_PATH_LOCATIONS}/${RBRR_GAR_LOCATION}${RBGC_PATH_REPOSITORIES}/${RBRR_GAR_REPOSITORY}"
  local z_get_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:getIamPolicy"
  local z_set_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:setIamPolicy"

  ######################################################################
  # Prune Cloud Build SA writer binding (idempotent; harmless if missing)
  bcu_step 'Fetch current repo IAM policy'
  zrbga_http_json "POST" "${z_get_url}" "${z_token}" "repo_policy" "${ZRBGA_EMPTY_JSON}"
  zrbga_http_require_ok "Get repo IAM policy" "repo_policy"

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
     ' "${ZRBGA_PREFIX}repo_policy${ZRBGA_POSTFIX_JSON}" > "${z_updated_policy}" \
     || bcu_die "Failed to prune writer binding"

  local z_repo_set_body="${BDU_TEMP_DIR}/rbga_repo_set_policy_body.json"
  jq -n --slurpfile p "${z_updated_policy}" '{policy:$p[0]}' > "${z_repo_set_body}" \
    || bcu_die "Failed to build repo setIamPolicy body"
  zrbga_http_json "POST" "${z_set_url}" "${z_token}" "repo_policy_set" "${z_repo_set_body}"
  zrbga_http_require_ok "Set repo IAM policy" "repo_policy_set"

  ######################################################################
  # Delete the GAR repository (removes remaining repo-scoped bindings/data)
  bcu_step "Delete Artifact Registry repo '${RBRR_GAR_REPOSITORY}' in ${RBRR_GAR_LOCATION}"
  zrbga_http_json "DELETE" \
    "${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}" \
    "${z_token}" "${ZRBGA_INFIX_DELETE_REPO}"

  # Tolerate already-deleted repos; fail on other errors
  case "$(zrbga_http_code_capture "${ZRBGA_INFIX_DELETE_REPO}" || echo)" in
    200|204) bcu_info "Repository deleted" ;;
    404)     bcu_warn "Repository not found (already deleted)" ;;
    *)
      local z_err
      z_err=$(zrbga_json_field_capture "${ZRBGA_INFIX_DELETE_REPO}" '.error.message') || z_err="Unknown error"
      bcu_die "Failed to delete repository: ${z_err}"
      ;;
  esac

  bcu_success "RBGA project resources destroyed (APIs left enabled; SAs untouched)"
}

rbga_list_service_accounts() {
  zrbga_sentinel

  bcu_doc_brief "List all service accounts in the project"
  bcu_doc_shown || return 0

  bcu_step "Listing service accounts in project: ${RBRR_GCP_PROJECT_ID}"

  bcu_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token (rc=$?)"

  bcu_log_args 'List service accounts via REST API'
  zrbga_http_json "GET" "${RBGC_API_SERVICE_ACCOUNTS}" "${z_token}" "${ZRBGA_INFIX_LIST}"
  zrbga_http_require_ok "List service accounts" "${ZRBGA_INFIX_LIST}"

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

  bcu_step 'Adding Artifact Registry Reader role'
  zrbga_add_iam_role "${z_account_email}" "${RBGC_ROLE_ARTIFACTREGISTRY_READER}"

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

  bcu_step 'Adding Cloud Build Editor role (project scope)'
  zrbga_add_iam_role "${z_account_email}" "${RBGC_ROLE_CLOUDBUILD_BUILDS_EDITOR}"

  bcu_step 'Grant Artifact Registry Writer (repo-scoped)'
  zrbga_add_repo_iam_role "${z_account_email}" "${RBRR_GAR_LOCATION}" "${RBRR_GAR_REPOSITORY}" "${RBGC_ROLE_ARTIFACTREGISTRY_WRITER}"

  bcu_step 'Grant Artifact Registry Admin (repo-scoped) for delete in own repo'
  zrbga_add_repo_iam_role "${z_account_email}" "${RBRR_GAR_LOCATION}" "${RBRR_GAR_REPOSITORY}" "${RBGC_ROLE_ARTIFACTREGISTRY_ADMIN}"

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

