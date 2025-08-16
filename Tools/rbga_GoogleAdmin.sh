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

  bcu_log_args "Ensure RBGC is kindled first"
  zrbgc_sentinel

  ZRBGA_ADMIN_ROLE="rbga-admin"
  ZRBGA_RBRR_FILE="./rbrr_RecipeBottleRegimeRepo.sh"

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
  ZRBGA_INFIX_LIST="list"
  ZRBGA_INFIX_DELETE="delete"

  ZRBGA_KINDLED=1
}

zrbga_sentinel() {
  test "${ZRBGA_KINDLED:-}" = "1" || bcu_die "Module rbga not kindled - call zrbga_kindle first"
}

zrbga_show() {
  zrbga_sentinel
  echo -e "${1:-}"
}

# JSON REST helper (hardcoded headers)
# Usage:
#   zrbga_http_json "METHOD" "URL" "TOKEN" "INFIX" ["BODY_FILE"] ["ACCEPT"]
zrbga_http_json() {
  zrbga_sentinel
  local z_method="$1"
  local z_url="$2"
  local z_token="$3"
  local z_infix="$4"
  local z_body_file="${5:-}"
  local z_accept="${6:-application/json}"

  test -n "${z_method}" || bcu_die "zrbga_http_json: method required"
  test -n "${z_url}"    || bcu_die "zrbga_http_json: url required"
  test -n "${z_token}"  || bcu_die "zrbga_http_json: token required"
  test -n "${z_infix}"  || bcu_die "zrbga_http_json: infix required"

  local z_out_json="${ZRBGA_PREFIX}${z_infix}_response.json"
  local z_out_code="${ZRBGA_PREFIX}${z_infix}_code.txt"

  if test -n "${z_body_file}"; then
    curl -s -X "${z_method}"                         \
      -H "Authorization: Bearer ${z_token}"          \
      -H "Content-Type: application/json"            \
      -H "Accept: ${z_accept}"                       \
      -d @"${z_body_file}"                           \
      "${z_url}"                                     \
      -o "${z_out_json}"                             \
      -w "%{http_code}" > "${z_out_code}" 2>/dev/null
  else
    curl -s -X "${z_method}"                         \
      -H "Authorization: Bearer ${z_token}"          \
      -H "Accept: ${z_accept}"                       \
      "${z_url}"                                     \
      -o "${z_out_json}"                             \
      -w "%{http_code}" > "${z_out_code}" 2>/dev/null
  fi
}

# Enhanced version of zrbga_http_expect  
# Usage: zrbga_http_expect "context" "infix" [warn_code [warn_message]]
zrbga_http_expect() {
  zrbga_sentinel
  local z_ctx="$1" 
  local z_infix="$2"
  local z_warn_code="${3:-}"
  local z_warn_message="${4:-already exists}"
  
  local z_code_file="${ZRBGA_PREFIX}${z_infix}_code.txt"
  local z_json_file="${ZRBGA_PREFIX}${z_infix}_response.json"

  local z_code
  z_code=$(<"${z_code_file}") || bcu_die "${z_ctx}: failed to read HTTP code"

  bcu_log_args "Check success" #
  if test "${z_code}" = "200"; then
    return 0
  fi
  
  bcu_log_args "Check warn case" #
  if test -n "${z_warn_code}" && test "${z_code}" = "${z_warn_code}"; then
    bcu_warn "${z_ctx}: ${z_warn_message}"
    return 0
  fi

  bcu_log_args "Error case" #
  local z_err
  z_err=$(jq -r '.error.message // "Unknown error"' "${z_json_file}") || z_err="Parse error"
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

  bcu_log_args "Extract fields"
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

  bcu_log_args "Verify project matches"
  test "${z_project_id}" = "${RBRR_GCP_PROJECT_ID}" \
    || bcu_die "Project mismatch: JSON has '${z_project_id}', expected '${RBRR_GCP_PROJECT_ID}'"

  bcu_log_args "Write RBRA file"
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

  bcu_step "Get OAuth token from admin"
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step "Create request JSON for ${z_account_name}"
  jq -n \
    --arg account_id "${z_account_name}" \
    --arg display_name "${z_display_name}" \
    --arg description "${z_description}" \
    '{
      accountId: $account_id,
      serviceAccount: {
        displayName: $display_name,
        description: $description
      }
    }' > "${ZRBGA_PREFIX}create_request.json" || bcu_die "Failed to create request JSON"

  bcu_step "Create service account via REST API"
  zrbga_http_json "POST" "${RBGC_API_SERVICE_ACCOUNTS}" "${z_token}" \
    "${ZRBGA_INFIX_CREATE}" "${ZRBGA_PREFIX}create_request.json"

  local z_http_code
  z_http_code=$(<"${ZRBGA_PREFIX}create_code.txt")
  test -n "${z_http_code}" || bcu_die "Failed to read HTTP code"

  if test "${z_http_code}" = "409"; then
    bcu_die "Service account already exists: ${z_account_email}"
  elif test "${z_http_code}" != "200"; then
    local z_error
    z_error=$(jq -r '.error.message // "Unknown error"' "${ZRBGA_PREFIX}create_response.json") || z_error="Parse error"
    bcu_log_args "Service account creation failed. Response: $(cat "${ZRBGA_PREFIX}create_response.json")"
    bcu_die "Failed to create service account (HTTP ${z_http_code}): ${z_error}"
  fi

  bcu_info "Service account created: ${z_account_email}"

  bcu_step     "Wait for service account propagation and verify existence"

  # Wait and verify the service account exists before proceeding
  local z_retry_count=0
  local z_max_retries=10
  local z_verify_success=false

  while [ $z_retry_count -lt $z_max_retries ]; do
    sleep 3
    z_retry_count=$((z_retry_count + 1))

    # Try to get the service account to verify it exists
    zrbga_http_json "GET" \
      "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email}" "${z_token}" \
      "${ZRBGA_INFIX_VERIFY}"

    if zrbga_http_expect "Verify service account" "${ZRBGA_INFIX_VERIFY}"; then
      z_verify_success=true
      bcu_log_args "Service account verified after ${z_retry_count} attempts"
      break
    else
      bcu_log_args "Service account not ready yet (attempt ${z_retry_count}/${z_max_retries})"
    fi
  done

  if [ "$z_verify_success" = false ]; then
    bcu_die "Service account verification failed after ${z_max_retries} attempts"
  fi

  bcu_step     "Generate service account key"
  local z_key_req="${BDU_TEMP_DIR}/rbga_key_request.json"
  printf '%s' '{"privateKeyType": "TYPE_GOOGLE_CREDENTIALS_FILE"}' > "${z_key_req}"
  zrbga_http_json "POST" \
    "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email}${RBGC_PATH_KEYS}" \
    "${z_token}" \
    "${ZRBGA_INFIX_KEY}" \
    "${z_key_req}"

  zrbga_http_expect "Generate service account key" "${ZRBGA_INFIX_KEY}"

  bcu_step "Extract and decode key data"
  local z_key_json="${BDU_TEMP_DIR}/rbga_key_${z_instance}.json"
  jq -r '.privateKeyData' "${ZRBGA_PREFIX}key_response.json" | base64 -d > "${z_key_json}" \
    || bcu_die "Failed to extract/decode key data"

  bcu_step "Convert JSON key to RBRA format"
  local z_account_suffix="${z_account_name##rbga-}"
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

  bcu_step "Write RBRA file: ${z_rbra_file}"
  echo "RBRA_CLIENT_EMAIL=\"${z_client_email}\""  > "${z_rbra_file}"
  echo "RBRA_PRIVATE_KEY=\"${z_private_key}\""   >> "${z_rbra_file}"
  echo "RBRA_PROJECT_ID=\"${z_project_id}\""     >> "${z_rbra_file}"
  echo "RBRA_TOKEN_LIFETIME_SEC=1800"            >> "${z_rbra_file}"

  test -f "${z_rbra_file}" || bcu_die "Failed to write RBRA file"

  # Clean up temp key JSON
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

  bcu_log_args "Get current IAM policy" #
  zrbga_http_json "POST" "${RBGC_API_CRM_GET_IAM_POLICY}" "${z_token}" \
    "${ZRBGA_INFIX_ROLE}" "${ZRBGA_EMPTY_JSON}"

  zrbga_http_expect "Get IAM policy" "${ZRBGA_INFIX_ROLE}"

  bcu_log_args "Update IAM policy with new role binding" #
  local z_updated_policy="${BDU_TEMP_DIR}/rbga_updated_policy.json"

  jq --arg role "${z_role}"                                \
     --arg member "serviceAccount:${z_account_email}"      \
     '.bindings += [{role: $role, members: [$member]}]'    \
     "${ZRBGA_PREFIX}role_response.json" > "${z_updated_policy}"      \
     || bcu_die "Failed to update IAM policy"

  bcu_log_args "Set updated IAM policy" #
  local z_set_body="${BDU_TEMP_DIR}/rbga_set_policy_body.json"
  jq -n --slurpfile p "${z_updated_policy}" '{policy:$p[0]}' > "${z_set_body}" \
    || bcu_die "Failed to build setIamPolicy body"
  zrbga_http_json "POST" "${RBGC_API_CRM_SET_IAM_POLICY}" "${z_token}" \
    "${ZRBGA_INFIX_ROLE_SET}" "${z_set_body}"

  zrbga_http_expect "Set IAM policy" "${ZRBGA_INFIX_ROLE_SET}"

  bcu_log_args "Successfully added role ${z_role}"
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

  bcu_log_args "Adding repo-scoped IAM role ${z_role} to ${z_account_email} on ${z_location}/${z_repository}"

  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  local z_resource="projects/${RBRR_GCP_PROJECT_ID}${RBGC_PATH_LOCATIONS}/${z_location}${RBGC_PATH_REPOSITORIES}/${z_repository}"
  local z_get_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:getIamPolicy"
  local z_set_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:setIamPolicy"

  bcu_log_args "Get current repo IAM policy"
  zrbga_http_json "POST" "${z_get_url}" "${z_token}" \
    "${ZRBGA_INFIX_REPO_ROLE}" "${ZRBGA_EMPTY_JSON}"

  zrbga_http_expect "Get repo IAM policy" "${ZRBGA_INFIX_REPO_ROLE}"

  bcu_log_args "Update repo IAM policy"
  local z_updated_policy="${BDU_TEMP_DIR}/rbga_repo_updated_policy.json"
  jq --arg role   "${z_role}"                                      \
     --arg member "serviceAccount:${z_account_email}"              \
     '
       .bindings = (.bindings // []) |
       if ( [ .bindings[]?.role ] | index($role) ) then
         .bindings = ( .bindings | map(
           if .role == $role then .members = ( (.members // []) + [$member] | unique )
           else .
           end))
       else
         .bindings += [{role: $role, members: [$member]}]
       end
     ' "${ZRBGA_PREFIX}repo_role_response.json" > "${z_updated_policy}" || bcu_die "Failed to update policy json"

  bcu_log_args "Set updated repo IAM policy"
  local z_repo_set_body="${BDU_TEMP_DIR}/rbga_repo_set_policy_body.json"
  jq -n --slurpfile p "${z_updated_policy}" '{policy:$p[0]}' > "${z_repo_set_body}" \
    || bcu_die "Failed to build repo setIamPolicy body"
  zrbga_http_json "POST" "${z_set_url}" "${z_token}" \
    "${ZRBGA_INFIX_REPO_ROLE_SET}" "${z_repo_set_body}"

  zrbga_http_expect "Set repo IAM policy" "${ZRBGA_INFIX_REPO_ROLE_SET}"

  bcu_log_args "Successfully added repo-scoped role ${z_role}"
}


######################################################################
# External Functions (rbga_*)

rbga_initialize_admin() {
  zrbga_sentinel

  local z_json_path="${1:-}"

  bcu_doc_brief "Initialize admin account and enable required APIs"
  bcu_doc_param "json_path" "Path to downloaded admin JSON file"
  bcu_doc_shown || return 0

  test -n "${z_json_path}" || bcu_die "First argument must be path to downloaded JSON key file."

  bcu_step "Converting admin JSON to RBRA format"

  zrbga_extract_json_to_rbra  \
    "${z_json_path}"          \
    "${RBRR_ADMIN_RBRA_FILE}" \
    "1800"

  bcu_step "Get token using the newly created RBRA"
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step "Enable IAM API (required for service account operations)"
  zrbga_http_json "POST" "${RBGC_API_SERVICEUSAGE_ENABLE_IAM}" "${z_token}" \
    "${ZRBGA_INFIX_API_IAM_ENABLE}" "${ZRBGA_EMPTY_JSON}"

  zrbga_http_expect "Enable IAM API" "${ZRBGA_INFIX_API_IAM_ENABLE}" 409 "already enabled"

  bcu_step "Enable Cloud Resource Manager API (required for IAM policy operations)"
  zrbga_http_json "POST" "${RBGC_API_SERVICEUSAGE_ENABLE_CRM}" "${z_token}" \
    "${ZRBGA_INFIX_API_CRM_ENABLE}" "${ZRBGA_EMPTY_JSON}"

  zrbga_http_expect "Enable Cloud Resource Manager API" "${ZRBGA_INFIX_API_CRM_ENABLE}" 409 "already enabled"  

  bcu_step "Enable Artifact Registry API (required for repo-scoped IAM + image operations)"
  zrbga_http_json "POST" "${RBGC_API_SERVICEUSAGE_ENABLE_ARTIFACTREGISTRY}" "${z_token}" \
    "${ZRBGA_INFIX_API_ART_ENABLE}" "${ZRBGA_EMPTY_JSON}"

  zrbga_http_expect "Enable Artifact Registry API" "${ZRBGA_INFIX_API_ART_ENABLE}" 409 "already enabled"

  local z_prop_delay_seconds=45
  bcu_step "Waiting ${z_prop_delay_seconds} seconds for API changes to propagate"
  bcu_info "This delay ensures APIs are fully available across all Google regions"
  local z_countdown
  for z_countdown in $(seq ${z_prop_delay_seconds} -1 1); do
    printf "\rTime remaining: %2d seconds" "${z_countdown}"
    sleep 1
  done
  printf "\rAPI propagation wait complete                    \n"

  bcu_step "Verifying API enablement"
  zrbga_http_json "GET" "${RBGC_API_SERVICEUSAGE_VERIFY_IAM}" "${z_token}" \
    "${ZRBGA_INFIX_API_IAM_VERIFY}"

  zrbga_http_expect "Verify IAM API" "${ZRBGA_INFIX_API_IAM_VERIFY}"
  local z_state
  z_state=$(jq -r '.state // "UNKNOWN"' "${ZRBGA_PREFIX}api_iam_verify_response.json")
  if test "${z_state}" = "ENABLED"; then
    bcu_info "IAM API verified: ENABLED"
  else
    bcu_die "IAM API not enabled. State: ${z_state}"
  fi

  bcu_step "Verify Cloud Resource Manager API..."
  zrbga_http_json "GET" "${RBGC_API_SERVICEUSAGE_VERIFY_CRM}" "${z_token}" \
    "${ZRBGA_INFIX_API_CRM_VERIFY}"

  zrbga_http_expect "Verify Cloud Resource Manager API" "${ZRBGA_INFIX_API_CRM_VERIFY}"
  local z_state
  z_state=$(jq -r '.state // "UNKNOWN"' "${ZRBGA_PREFIX}api_crm_verify_response.json")
  if test "${z_state}" = "ENABLED"; then
    bcu_info "Cloud Resource Manager API verified: ENABLED"
  else
    bcu_die "Cloud Resource Manager API not enabled. State: ${z_state}"
  fi

  bcu_step "Verify Artifact Registry API..."
  zrbga_http_json "GET" "${RBGC_API_SERVICEUSAGE_VERIFY_ARTIFACTREGISTRY}" "${z_token}" \
    "${ZRBGA_INFIX_API_ART_VERIFY}"

  zrbga_http_expect "Verify Artifact Registry API" "${ZRBGA_INFIX_API_ART_VERIFY}"
  local z_state
  z_state=$(jq -r '.state // "UNKNOWN"' "${ZRBGA_PREFIX}api_art_verify_response.json")
  if test "${z_state}" = "ENABLED"; then
    bcu_info "Artifact Registry API verified: ENABLED"
  else
    bcu_die "Artifact Registry API not enabled. State: ${z_state}"
  fi

  bcu_success "Admin initialization complete"

  bcu_info "Admin RBRA file created: ${RBRR_ADMIN_RBRA_FILE}"
  bcu_warn "Consider deleting source JSON after verification: ${z_json_path}"
}

rbga_list_service_accounts() {
  zrbga_sentinel

  bcu_doc_brief "List all service accounts in the project"
  bcu_doc_shown || return 0

  bcu_step "Listing service accounts in project: ${RBRR_GCP_PROJECT_ID}"

  bcu_log_args "Get OAuth token from admin"
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token (rc=$?)"

  bcu_log_args "List service accounts via REST API"
  zrbga_http_json "GET" "${RBGC_API_SERVICE_ACCOUNTS}" "${z_token}" \
    "${ZRBGA_INFIX_LIST}"

  zrbga_http_expect "List service accounts" "${ZRBGA_INFIX_LIST}"

  bcu_log_args "Check if accounts exist"
  local z_count
  z_count=$(jq -r '.accounts | length' "${ZRBGA_PREFIX}list_response.json") || bcu_die "Failed to parse response"

  if test "${z_count}" = "0" || test "${z_count}" = "null"; then
    bcu_info "No service accounts found in project"
    return 0
  fi

  bcu_log_args "Display accounts"
  bcu_step "Found ${z_count} service account(s):"

  bcu_log_args "Calculate max email width for right-justification"
  local z_max_width
  z_max_width=$(jq -r '.accounts[].email | length' "${ZRBGA_PREFIX}list_response.json" | sort -n | tail -1) || bcu_die "Failed to calculate max width"

  bcu_log_args "Display with right-justified email column"
  jq -r --argjson width "${z_max_width}" \
    '.accounts[] | "  " + (.email | tostring | ((" " * ($width - length)) + .)) + " - " + (.displayName // "(no display name)")' \
    "${ZRBGA_PREFIX}list_response.json" || bcu_die "Failed to format accounts"

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

  bcu_step "Adding Artifact Registry Reader role"
  zrbga_add_iam_role "${z_account_email}" "${RBGC_ROLE_ARTIFACTREGISTRY_READER}"

  local z_actual_rbra_file="${BDU_OUTPUT_DIR}/${z_instance}.rbra"

  bcu_step "To install the RBRA file locally, run:"
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

  bcu_step "Adding Cloud Build Editor role (project scope)"
  zrbga_add_iam_role "${z_account_email}" "${RBGC_ROLE_CLOUDBUILD_BUILDS_EDITOR}"

  bcu_step "Grant Artifact Registry Writer (repo-scoped)"
  zrbga_add_repo_iam_role "${z_account_email}" "${RBRR_GAR_LOCATION}" "${RBRR_GAR_REPOSITORY}" "${RBGC_ROLE_ARTIFACTREGISTRY_WRITER}"

  bcu_step "Grant Artifact Registry Admin (repo-scoped) for delete in own repo"
  zrbga_add_repo_iam_role "${z_account_email}" "${RBRR_GAR_LOCATION}" "${RBRR_GAR_REPOSITORY}" "${RBGC_ROLE_ARTIFACTREGISTRY_ADMIN}"

  local z_actual_rbra_file="${BDU_OUTPUT_DIR}/${z_instance}.rbra"

  bcu_step "To install the RBRA file locally, run:"
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

  bcu_log_args "Get OAuth token from admin"
  local z_token
  z_token=$(zrbga_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_log_args "Delete via REST API"
  zrbga_http_json "DELETE" "${RBGC_API_SERVICE_ACCOUNTS}/${z_sa_email}" "${z_token}" \
    "${ZRBGA_INFIX_DELETE}"

  local z_http_code
  z_http_code=$(<"${ZRBGA_PREFIX}delete_code.txt")
  test -n "${z_http_code}" || bcu_die "Failed to read HTTP code"

  if test "${z_http_code}" = "200" || test "${z_http_code}" = "204"; then
    bcu_info "Deleted service account: ${z_sa_email}"
  elif test "${z_http_code}" = "404"; then
    bcu_warn "Service account not found (already deleted): ${z_sa_email}"
  else
    local z_error
    z_error=$(jq -r '.error.message // "Unknown error"' "${ZRBGA_PREFIX}delete_response.json") || z_error="Parse error"
    bcu_die "Failed to delete (HTTP ${z_http_code}): ${z_error}"
  fi

  bcu_success "Delete operation completed"
}

# eof

