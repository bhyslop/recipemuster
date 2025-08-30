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
# Recipe Bottle GCP Payor - Billing and Destructive Lifecycle Operations

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGP_SOURCED:-}" || bcu_die "Module rbgp multiply sourced - check sourcing hierarchy"
ZRBGP_SOURCED=1

######################################################################
# Internal Functions (zrbgp_*)

zrbgp_kindle() {
  test -z "${ZRBGP_KINDLED:-}" || bcu_die "Module rbgp already kindled"

  bcu_log_args "Ensure dependencies are kindled first"
  zrbgc_sentinel
  zrbgo_sentinel
  zrbgu_sentinel

  ZRBGP_PREFIX="${BDU_TEMP_DIR}/rbgp_"
  ZRBGP_EMPTY_JSON="${ZRBGP_PREFIX}empty.json"
  printf '{}' > "${ZRBGP_EMPTY_JSON}"

  # Infix values for HTTP operations
  ZRBGP_INFIX_PROJECT_DELETE="project_delete"
  ZRBGP_INFIX_PROJECT_RESTORE="project_restore"
  ZRBGP_INFIX_PROJECT_STATE="project_state"
  ZRBGP_INFIX_LIST_LIENS="list_liens"
  ZRBGP_INFIX_DELETE_LIEN="delete_lien"
  ZRBGP_INFIX_BILLING_ATTACH="billing_attach"
  ZRBGP_INFIX_BILLING_DETACH="billing_detach"
  ZRBGP_INFIX_CREATE_REPO="create_repo"
  ZRBGP_INFIX_VERIFY_REPO="verify_repo"
  ZRBGP_INFIX_PROJECT_INFO="project_info"
  ZRBGP_INFIX_BUCKET_CREATE="bucket_create"
  ZRBGP_INFIX_API_CHECK="api_checking"

  ZRBGP_KINDLED=1
}

zrbgp_sentinel() {
  test "${ZRBGP_KINDLED:-}" = "1" || bcu_die "Module rbgp not kindled - call zrbgp_kindle first"
}


######################################################################
# OAuth Authentication Functions (zrbgp_oauth_*)

zrbgp_refresh_capture() {
  zrbgp_sentinel

  bcu_log_args "Loading RBRO credentials for OAuth token refresh"
  local z_rbro_file="${HOME}/.rbw/rbro.env"
  test -d "${HOME}/.rbw" || bcu_die "RBRO directory missing - run rbgp_payor_install"
  test -f "${z_rbro_file}" || bcu_die "RBRO credentials missing - run rbgp_payor_install"
  
  # Check file permissions
  local z_perms
  z_perms=$(stat -c %a "${z_rbro_file}" 2>/dev/null) || bcu_die "Failed to check RBRO file permissions"
  if [ "${z_perms}" != "600" ]; then
    bcu_log_args "Warning: RBRO file permissions should be 600, found ${z_perms}"
  fi
  
  # Source RBRO credentials
  # shellcheck source=/dev/null
  source "${z_rbro_file}" || bcu_die "Failed to source RBRO credentials"
  
  test -n "${RBRO_CLIENT_SECRET:-}" || bcu_die "RBRO_CLIENT_SECRET missing from ${z_rbro_file}"
  test -n "${RBRO_REFRESH_TOKEN:-}" || bcu_die "RBRO_REFRESH_TOKEN missing from ${z_rbro_file}"
  test -n "${RBRP_OAUTH_CLIENT_ID:-}" || bcu_die "RBRP_OAUTH_CLIENT_ID not set in environment"

  bcu_log_args "Constructing OAuth token refresh request"
  local z_token_request="${BDU_TEMP_DIR}/rbgp_oauth_refresh.json"
  jq -n \
    --arg refresh_token "${RBRO_REFRESH_TOKEN}" \
    --arg client_id "${RBRP_OAUTH_CLIENT_ID}" \
    --arg client_secret "${RBRO_CLIENT_SECRET}" \
    --arg grant_type "refresh_token" \
    '{
      refresh_token: $refresh_token,
      client_id: $client_id,
      client_secret: $client_secret,
      grant_type: $grant_type
    }' > "${z_token_request}" || bcu_die "Failed to create OAuth refresh request"

  local z_token_response="${BDU_TEMP_DIR}/rbgp_oauth_refresh_response.json"
  local z_token_code="${BDU_TEMP_DIR}/rbgp_oauth_refresh_code.txt"
  
  bcu_log_args "Exchanging refresh token for access token"
  curl -s -X POST \
    -H "Content-Type: application/json" \
    -d @"${z_token_request}" \
    -w "%{http_code}" \
    -o "${z_token_response}" \
    "https://oauth2.googleapis.com/token" > "${z_token_code}" || bcu_die "Failed to execute OAuth refresh request"
  
  local z_code
  z_code=$(<"${z_token_code}") || bcu_die "Failed to read OAuth response code"
  
  if [ "${z_code}" != "200" ]; then
    local z_error_desc
    z_error_desc=$(jq -r '.error_description // .error // "Unknown error"' "${z_token_response}" 2>/dev/null || echo "HTTP ${z_code}")
    if [ "${z_code}" = "400" ] || [ "${z_code}" = "401" ]; then
      bcu_die "OAuth credentials expired or invalid - run rbgp_payor_oauth_refresh: ${z_error_desc}"
    else
      bcu_die "OAuth token refresh failed: ${z_error_desc}"
    fi
  fi
  
  local z_access_token
  z_access_token=$(jq -r '.access_token // empty' "${z_token_response}" 2>/dev/null) || bcu_die "Failed to extract access token"
  test -n "${z_access_token}" || bcu_die "OAuth response missing access_token"
  
  echo "${z_access_token}"
}

# RBTOE: Payor OAuth Authentication Pattern
# Establishes Payor OAuth context by loading RBRO credentials and obtaining access token
zrbgp_authenticate_capture() {
  zrbgp_sentinel
  
  bcu_log_args "Establishing Payor OAuth authentication context"
  
  # Load RBRO credentials
  rbgu_rbro_load
  
  # Load RBRP_OAUTH_CLIENT_ID from environment
  test -n "${RBRP_OAUTH_CLIENT_ID:-}" || bcu_die "RBRP_OAUTH_CLIENT_ID not set in environment"
  
  # Exchange refresh token for access token
  local z_access_token
  z_access_token=$(zrbgp_refresh_capture) || bcu_die "Failed to exchange OAuth refresh token"
  
  test -n "${z_access_token}" || bcu_die "Empty access token from OAuth exchange"
  
  bcu_log_args "Payor OAuth authentication successful"
  echo "${z_access_token}"
}

zrbgp_depot_list_update() {
  zrbgp_sentinel

  bcu_log_args "Updating depot project tracking in RBRP configuration"
  
  # Get OAuth access token
  local z_access_token
  z_access_token=$(zrbgp_authenticate_capture) || bcu_die "Failed to authenticate for depot list update"
  
  # Query active depot projects
  local z_filter="projectId:rbw-depot-* AND lifecycleState:ACTIVE"
  local z_list_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V3}/projects?filter=${z_filter// /%20}"
  rbgu_http_json "GET" "${z_list_url}" "${z_access_token}" "depot_list_tracking"
  rbgu_http_require_ok "Query depot projects" "depot_list_tracking" || bcu_die "Failed to query depot projects"
  
  # Extract and validate depot project IDs
  local z_depot_ids=""
  local z_project_count
  z_project_count=$(rbgu_json_field_capture "depot_list_tracking" '.projects // [] | length') || z_project_count=0
  
  if [ "${z_project_count}" -gt 0 ]; then
    local z_index=0
    while [ "${z_index}" -lt "${z_project_count}" ]; do
      local z_project_id
      z_project_id=$(rbgu_json_field_capture "depot_list_tracking" ".projects[${z_index}].projectId") || continue
      
      # Validate depot project ID pattern
      if printf '%s' "${z_project_id}" | grep -qE '^rbw-[a-z0-9-]+-[0-9]{12}$'; then
        z_depot_ids="${z_depot_ids} ${z_project_id}"
      else
        bcu_log_args "Warning: Skipping project with invalid depot pattern: ${z_project_id}"
      fi
      
      z_index=$((z_index + 1))
    done
  fi
  
  # Update RBRP configuration
  # Note: This would normally update rbrp.env file
  # For now, we'll export to environment and log the result
  export RBRP_DEPOT_PROJECT_IDS="${z_depot_ids# }"
  
  bcu_log_args "Updated RBRP_DEPOT_PROJECT_IDS: '${RBRP_DEPOT_PROJECT_IDS}'"
  bcu_log_args "Found ${z_project_count} depot projects, ${z_depot_ids:+ $(($(printf '%s' "${z_depot_ids}" | wc -w))):+0} valid"
  
  return 0
}

######################################################################
# External Functions (rbgp_*)

zrbgp_billing_attach() {
  zrbgp_sentinel

  local z_billing_account="${1:-}"

  bcu_doc_brief "Attach a billing account to the project"
  bcu_doc_param "billing_account" "Billing account ID to attach"
  bcu_doc_shown || return 0

  test -n "${z_billing_account}" || bcu_die "Billing account ID required"

  bcu_step "Attaching billing account: ${z_billing_account}"

  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"
  local z_billing_body="${BDU_TEMP_DIR}/rbgp_billing_attach.json"
  jq -n --arg billingAccountName "billingAccounts/${z_billing_account}" \
    --arg projectId "${RBRR_GCP_PROJECT_ID}" \
    '{
      billingAccountName: $billingAccountName,
      projectId: $projectId,
      billingEnabled: true
    }' > "${z_billing_body}" || bcu_die "Failed to build billing attach body"

  local z_billing_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/projects/${RBRR_GCP_PROJECT_ID}:setBillingInfo"
  rbgu_http_json "PUT" "${z_billing_url}" "${z_token}" \
                                  "${ZRBGP_INFIX_BILLING_ATTACH}" "${z_billing_body}"
  rbgu_http_require_ok "Attach billing account" "${ZRBGP_INFIX_BILLING_ATTACH}"

  bcu_success "Billing account ${z_billing_account} attached to project"
}

zrbgp_billing_detach() {
  zrbgp_sentinel

  bcu_doc_brief "Detach billing account from the project"
  bcu_doc_shown || return 0

  bcu_step "Detaching billing account from project"

  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"
  local z_billing_body="${BDU_TEMP_DIR}/rbgp_billing_detach.json"
  jq -n --arg projectId "${RBRR_GCP_PROJECT_ID}" \
    '{
      projectId: $projectId,
      billingEnabled: false
    }' > "${z_billing_body}" || bcu_die "Failed to build billing detach body"

  local z_billing_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/projects/${RBRR_GCP_PROJECT_ID}:setBillingInfo"
  rbgu_http_json "PUT" "${z_billing_url}" "${z_token}" \
                                  "${ZRBGP_INFIX_BILLING_DETACH}" "${z_billing_body}"
  rbgu_http_require_ok "Detach billing account" "${ZRBGP_INFIX_BILLING_DETACH}"

  bcu_success "Billing account detached from project"
}



zrbgp_liens_list() {
  zrbgp_sentinel

  bcu_doc_brief "List all liens on the project"
  bcu_doc_shown || return 0

  bcu_step "Listing liens on project: ${RBRR_GCP_PROJECT_ID}"

  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"
  rbgu_http_json "GET" "${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens?parent=projects/${RBRR_GCP_PROJECT_ID}" "${z_token}" "${ZRBGP_INFIX_LIST_LIENS}"
  rbgu_http_require_ok "List liens" "${ZRBGP_INFIX_LIST_LIENS}"

  local z_lien_count
  z_lien_count=$(rbgu_json_field_capture "${ZRBGP_INFIX_LIST_LIENS}" '.liens // [] | length') || bcu_die "Failed to parse liens response"

  if [[ "${z_lien_count}" -eq 0 ]]; then
    bcu_info "No liens found on project"
    return 0
  fi

  bcu_step "Found ${z_lien_count} lien(s):"
  jq -r '.liens[]? | "  - " + .name + " (reason: " + .reason + ")"' \
    "${ZRBGU_PREFIX}${ZRBGP_INFIX_LIST_LIENS}${ZRBGU_POSTFIX_JSON}" || true

  return 0
}

zrbgp_lien_delete() {
  zrbgp_sentinel

  local z_lien_name="${1:-}"

  bcu_doc_brief "Delete a specific lien from the project"
  bcu_doc_param "lien_name" "Full resource name of the lien to delete"
  bcu_doc_shown || return 0

  test -n "${z_lien_name}" || bcu_die "Lien name required"

  bcu_step "Deleting lien: ${z_lien_name}"

  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"
  rbgu_http_json "DELETE" "${RBGC_API_CRM_DELETE_LIEN}/${z_lien_name}" "${z_token}" "${ZRBGP_INFIX_DELETE_LIEN}"
  rbgu_http_require_ok "Delete lien" "${ZRBGP_INFIX_DELETE_LIEN}" 404 "not found (already deleted)"

  bcu_success "Lien deleted: ${z_lien_name}"
}


######################################################################
# Capture: list required services that are NOT enabled (blank = all enabled)
zrbgp_required_apis_missing_capture() {
  zrbgp_sentinel

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
    z_infix="${ZRBGP_INFIX_API_CHECK}_${z_service}"

    rbgu_http_json "GET" "${z_api}" "${z_token}" "${z_infix}" || true

    bcu_log_args 'If we cannot even read an HTTP code file, that is a processing failure.'
    z_code=$(rbgu_http_code_capture "${z_infix}") || z_code=""
    test -n "${z_code}" || return 1

    if test "${z_code}" = "200"; then
      z_state=$(rbgu_json_field_capture "${z_infix}" ".state") || z_state=""
      test "${z_state}" = "ENABLED" || z_missing="${z_missing} ${z_service}"
    else
      bcu_log_args 'Any non-200 (403/404/5xx/etc) => treat as NOT enabled'
      z_missing="${z_missing} ${z_service}"
    fi
  done

  printf '%s' "${z_missing# }"
}

zrbgp_get_project_number_capture() {
  zrbgp_sentinel

  local z_token
  z_token=$(rbgu_get_admin_token_capture) || return 1

  rbgu_http_json "GET" "${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}" "${z_token}" "${ZRBGP_INFIX_PROJECT_INFO}"
  rbgu_http_require_ok "Get project info" "${ZRBGP_INFIX_PROJECT_INFO}" || return 1

  local z_project_number
  z_project_number=$(rbgu_json_field_capture "${ZRBGP_INFIX_PROJECT_INFO}" '.projectNumber') || return 1
  test -n "${z_project_number}" || return 1

  echo "${z_project_number}"
}


zrbgp_create_gcs_bucket() {
  zrbgp_sentinel

  local z_token="${1}"
  local z_bucket_name="${2}"

  bcu_log_args 'Create bucket request JSON for '"${z_bucket_name}"
  local z_bucket_req="${BDU_TEMP_DIR}/rbgp_bucket_create_req.json"
  jq -n --arg name "${z_bucket_name}" --arg location "${RBGC_GAR_LOCATION}" '
    {
      name: $name,
      location: $location,
      storageClass: "STANDARD",
      lifecycle: { rule: [ { action: { type: "Delete" }, condition: { age: 1 } } ] }
    }' > "${z_bucket_req}" || bcu_die "Failed to create bucket request JSON"

  bcu_log_args 'Send bucket creation request'
  local z_code
  local z_err
  rbgu_http_json "POST" "${RBGC_API_GCS_BUCKETS}?project=${RBRR_GCP_PROJECT_ID}" "${z_token}" \
                                  "${ZRBGP_INFIX_BUCKET_CREATE}" "${z_bucket_req}"
  z_code=$(rbgu_http_code_capture "${ZRBGP_INFIX_BUCKET_CREATE}") || bcu_die "Bad bucket creation HTTP code"
  z_err=$(rbgu_json_field_capture "${ZRBGP_INFIX_BUCKET_CREATE}" '.error.message') || z_err="HTTP ${z_code}"

  case "${z_code}" in
    200|201) bcu_info "Bucket ${z_bucket_name} created";                         return 0 ;;
    409)     bcu_die  "Bucket ${z_bucket_name} already exists (pristine-state violation)" ;;
    *)       bcu_die  "Failed to create bucket: ${z_err}"                                 ;;
  esac
}

######################################################################
# External Functions (rbgp_*)

rbgp_payor_install() {
  zrbgp_sentinel

  local z_oauth_json_file="${1:-}"

  bcu_doc_brief "Install Payor OAuth credentials from client JSON file following RBAGS specification"
  bcu_doc_param "oauth_json_file" "Path to downloaded OAuth client JSON file from establish procedure"
  bcu_doc_lines "REQUIREMENT: OAuth consent screen must be configured in testing mode"
  bcu_doc_lines "            and the Payor project must have required APIs enabled"
  bcu_doc_shown || return 0

  bcu_step 'Validate input parameters'
  test -n "${z_oauth_json_file}" || bcu_die "OAuth JSON file path required as first argument"
  test -f "${z_oauth_json_file}" || bcu_die "OAuth JSON file not found: ${z_oauth_json_file}"

  bcu_step 'Parse OAuth client JSON'
  local z_client_id
  z_client_id=$(jq -r '.installed.client_id // .client_id // empty' "${z_oauth_json_file}" 2>/dev/null) || bcu_die "Failed to parse OAuth JSON file"
  test -n "${z_client_id}" || bcu_die "OAuth JSON file missing client_id field"
  
  local z_client_secret
  z_client_secret=$(jq -r '.installed.client_secret // .client_secret // empty' "${z_oauth_json_file}" 2>/dev/null) || bcu_die "Failed to extract client_secret from OAuth JSON file"
  test -n "${z_client_secret}" || bcu_die "OAuth JSON file missing client_secret field"
  
  local z_project_id
  z_project_id=$(jq -r '.installed.project_id // .project_id // empty' "${z_oauth_json_file}" 2>/dev/null) || bcu_die "Failed to extract project_id from OAuth JSON file"
  test -n "${z_project_id}" || bcu_die "OAuth JSON file missing project_id field"

  bcu_step 'Check existing credentials'
  local z_rbro_file="${HOME}/.rbw/rbro.env"
  local z_skip_auth=0
  if test -f "${z_rbro_file}"; then
    bcu_log_args "Found existing RBRO credentials at ${z_rbro_file}"
    z_skip_auth=1
  else
    bcu_log_args "No existing RBRO credentials found, authorization flow required"
  fi

  local z_refresh_token=""
  if test "${z_skip_auth}" = "0"; then
    bcu_step 'OAuth authorization flow'
    local z_auth_url="https://accounts.google.com/o/oauth2/v2/auth?client_id=${z_client_id}&redirect_uri=urn:ietf:wg:oauth:2.0:oob&scope=https://www.googleapis.com/auth/cloud-platform%20https://www.googleapis.com/auth/cloud-billing&response_type=code&access_type=offline"
    
    bcu_info "Open this URL in your browser:"
    bcu_info "${z_auth_url}"
    bcu_info ""
    printf "Copy the authorization code and paste here: "
    read -r z_auth_code
    test -n "${z_auth_code}" || bcu_die "Authorization code is required"
    
    bcu_log_args "Exchanging authorization code for tokens"
    local z_token_request="${BDU_TEMP_DIR}/rbgp_token_request.json"
    jq -n \
      --arg code "${z_auth_code}" \
      --arg client_id "${z_client_id}" \
      --arg client_secret "${z_client_secret}" \
      --arg redirect_uri "urn:ietf:wg:oauth:2.0:oob" \
      --arg grant_type "authorization_code" \
      '{
        code: $code,
        client_id: $client_id,
        client_secret: $client_secret,
        redirect_uri: $redirect_uri,
        grant_type: $grant_type
      }' > "${z_token_request}" || bcu_die "Failed to create token request"
    
    local z_token_response="${BDU_TEMP_DIR}/rbgp_token_response.json"
    local z_token_code="${BDU_TEMP_DIR}/rbgp_token_code.txt"
    
    curl -s -X POST \
      -H "Content-Type: application/json" \
      -d @"${z_token_request}" \
      -w "%{http_code}" \
      -o "${z_token_response}" \
      "https://oauth2.googleapis.com/token" > "${z_token_code}" || bcu_die "Failed to execute token exchange request"
    
    local z_code
    z_code=$(<"${z_token_code}") || bcu_die "Failed to read HTTP response code"
    
    if test "${z_code}" != "200"; then
      local z_error_desc
      z_error_desc=$(jq -r '.error_description // .error // "Unknown error"' "${z_token_response}" 2>/dev/null || echo "HTTP ${z_code}")
      bcu_die "OAuth token exchange failed: ${z_error_desc}"
    fi
    
    z_refresh_token=$(jq -r '.refresh_token // empty' "${z_token_response}" 2>/dev/null) || bcu_die "Failed to extract refresh token from response"
    test -n "${z_refresh_token}" || bcu_die "OAuth response missing refresh_token field"
  fi

  if test "${z_skip_auth}" = "0"; then
    bcu_step 'Create local credentials directory'
    mkdir -p "${HOME}/.rbw" || bcu_die "Failed to create ~/.rbw directory"
    chmod 700 "${HOME}/.rbw" || bcu_die "Failed to set ~/.rbw directory permissions"
    
    bcu_step 'Store OAuth credentials'
    cat > "${z_rbro_file}" <<-EOF || bcu_die "Failed to write RBRO credentials file"
RBRO_CLIENT_SECRET=${z_client_secret}
RBRO_REFRESH_TOKEN=${z_refresh_token}
EOF
    chmod 600 "${z_rbro_file}" || bcu_die "Failed to set RBRO file permissions"
  fi
  
  bcu_step 'Store public configuration'
  # Update RBRP configuration (these are safe to commit)
  bcu_log_args "Updating RBRP configuration with OAuth client details"
  # Note: This would normally update rbrp.env file with these values
  # For now we just validate the project ID matches expectation
  
  bcu_step 'Test OAuth authentication'
  local z_access_token
  z_access_token=$(zrbgp_authenticate_capture) || bcu_die "Failed to test OAuth authentication"
  test -n "${z_access_token}" || bcu_die "OAuth authentication test returned empty token"
  
  bcu_step 'Verify payor project access'
  local z_project_info_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/projects/${z_project_id}"
  rbgu_http_json "GET" "${z_project_info_url}" "${z_access_token}" "payor_verify" 
  rbgu_http_require_ok "Verify payor project" "payor_verify"
  
  local z_project_state
  z_project_state=$(rbgu_json_field_capture "payor_verify" '.lifecycleState') || bcu_die "Failed to get project state"
  test "${z_project_state}" = "ACTIVE" || bcu_die "Payor project is not ACTIVE (state: ${z_project_state})"

  bcu_step 'Initialize depot tracking'
  zrbgp_depot_list_update || bcu_die "Failed to initialize depot tracking"

  bcu_success "Payor OAuth installation completed successfully"
  bcu_info "Project ID: ${z_project_id}"
  bcu_info "OAuth Client ID: ${z_client_id}"
  bcu_info "RBRO File: ${z_rbro_file}"
  bcu_info ""
  bcu_info "Configuration values for RBRP:"
  bcu_info "  RBRP_PAYOR_PROJECT_ID=${z_project_id}"
  bcu_info "  RBRP_OAUTH_CLIENT_ID=${z_client_id}"
  bcu_info ""
  bcu_info "Next steps:"
  bcu_info "1. Update your rbrp.env with the above configuration values"
  bcu_info "2. Use rbgp_depot_create to create new depot infrastructure"
}

rbgp_depot_create() {
  zrbgp_sentinel

  local z_depot_name="${1:-}"
  local z_region="${2:-}"

  bcu_doc_brief "Create new depot infrastructure following RBAGS specification"
  bcu_doc_param "depot_name" "Depot name (lowercase/numbers/hyphens, max 20 chars)"
  bcu_doc_param "region" "GCP region for depot resources"
  bcu_doc_shown || return 0

  bcu_step 'Validate input parameters'
  test -n "${z_depot_name}" || bcu_die "Depot name required as first argument"
  test -n "${z_region}" || bcu_die "Region required as second argument"
  
  if ! printf '%s' "${z_depot_name}" | grep -qE '^[a-z0-9-]+$'; then
    bcu_die "Depot name must contain only lowercase letters, numbers, and hyphens"
  fi
  
  if [ "${#z_depot_name}" -gt 20 ]; then
    bcu_die "Depot name must be 20 characters or less"
  fi

  # Validate region exists in Artifact Registry locations
  bcu_log_args 'Validating region exists in Artifact Registry locations'
  local z_token
  z_token=$(zrbgp_authenticate_capture) || bcu_die "Failed to authenticate as Payor for region validation"
  
  local z_locations_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/projects/${RBRP_PAYOR_PROJECT_ID}/locations"
  rbgu_http_json "GET" "${z_locations_url}" "${z_token}" "region_validation"
  rbgu_http_require_ok "Validate region" "region_validation"
  
  local z_valid_regions
  z_valid_regions=$(rbgu_json_field_capture "region_validation" '.locations[].locationId' | tr '\n' ' ') || bcu_die "Failed to parse region list"
  
  if ! printf '%s' "${z_valid_regions}" | grep -qw "${z_region}"; then
    bcu_die "Invalid region. Valid regions: ${z_valid_regions}"
  fi

  bcu_step 'Authenticate as Payor'
  test -n "${RBRP_PAYOR_PROJECT_ID:-}" || bcu_die "RBRP_PAYOR_PROJECT_ID is not set"
  test -n "${RBRP_BILLING_ACCOUNT_ID:-}" || bcu_die "RBRP_BILLING_ACCOUNT_ID is not set"
  test -n "${RBRP_OAUTH_CLIENT_ID:-}" || bcu_die "RBRP_OAUTH_CLIENT_ID is not set"
  
  local z_token
  z_token=$(zrbgp_authenticate_capture) || bcu_die "Failed to authenticate as Payor via OAuth"

  bcu_step 'Generate depot project ID'
  local z_timestamp
  z_timestamp=$(date +%Y%m%d%H%M) || bcu_die "Failed to generate timestamp"
  local z_depot_project_id="rbw-${z_depot_name}-${z_timestamp}"
  
  if [ "${#z_depot_project_id}" -gt 30 ]; then
    bcu_die "Generated project ID too long (${#z_depot_project_id} > 30): ${z_depot_project_id}"
  fi
  
  bcu_log_args "Generated depot project ID: ${z_depot_project_id}"

  bcu_step 'Create depot project'
  local z_create_project_body="${BDU_TEMP_DIR}/rbgp_create_project.json"
  
  # OAuth users create projects without parent (per MPCR)
  jq -n \
    --arg projectId "${z_depot_project_id}" \
    --arg displayName "RB Depot ${z_depot_name}" \
    '{
      projectId: $projectId,
      displayName: $displayName
    }' > "${z_create_project_body}" || bcu_die "Failed to build project creation body"

  local z_create_project_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V3}/projects"
  rbgu_http_json_lro_ok \
    "Create depot project" \
    "${z_token}" \
    "${z_create_project_url}" \
    "depot_project_create" \
    "${z_create_project_body}" \
    ".name" \
    "${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}" \
    "operations/" \
    "${RBGC_EVENTUAL_CONSISTENCY_SEC}" \
    "${RBGC_MAX_CONSISTENCY_SEC}"

  bcu_step 'Link billing account'
  local z_billing_body="${BDU_TEMP_DIR}/rbgp_billing_link.json"
  jq -n \
    --arg billingAccountName "billingAccounts/${RBRP_BILLING_ACCOUNT_ID}" \
    --arg projectId "${z_depot_project_id}" \
    '{
      billingAccountName: $billingAccountName,
      projectId: $projectId,
      billingEnabled: true
    }' > "${z_billing_body}" || bcu_die "Failed to build billing link body"

  local z_billing_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/projects/${z_depot_project_id}:setBillingInfo"
  rbgu_http_json "PUT" "${z_billing_url}" "${z_token}" "depot_billing_link" "${z_billing_body}"
  rbgu_http_require_ok "Link billing account" "depot_billing_link"

  bcu_step 'Get depot project number'
  local z_project_info_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V3}/projects/${z_depot_project_id}"
  rbgu_http_json "GET" "${z_project_info_url}" "${z_token}" "depot_project_info"
  rbgu_http_require_ok "Get project info" "depot_project_info"
  
  local z_project_number
  z_project_number=$(rbgu_json_field_capture "depot_project_info" '.projectNumber') || bcu_die "Failed to get project number"
  test -n "${z_project_number}" || bcu_die "Project number is empty"

  bcu_step 'Enable depot project APIs'
  local z_api_services="artifactregistry cloudbuild containeranalysis storage iam serviceusage"
  for z_service in ${z_api_services}; do
    rbgu_api_enable "${z_service}" "${z_depot_project_id}" "${z_token}"
  done

  # Note: OAuth Payor doesn't need explicit permissions on depot since it uses user identity
  # Skip Payor permission grants - OAuth user context provides necessary access

  bcu_step 'Create build bucket'
  local z_build_bucket="rbw-${z_depot_name}-bucket"
  local z_bucket_req="${BDU_TEMP_DIR}/rbgp_bucket_create_req.json"
  jq -n \
    --arg name "${z_build_bucket}" \
    --arg location "${z_region}" \
    --arg project "${z_depot_project_id}" \
    '{
      name: $name,
      location: $location,
      storageClass: "STANDARD",
      lifecycle: { rule: [ { action: { type: "Delete" }, condition: { age: 1 } } ] }
    }' > "${z_bucket_req}" || bcu_die "Failed to create bucket request JSON"

  local z_bucket_create_url="${RBGC_API_ROOT_GCS}${RBGC_GCS_V1}/b?project=${z_depot_project_id}"
  rbgu_http_json "POST" "${z_bucket_create_url}" "${z_token}" "depot_bucket_create" "${z_bucket_req}"
  
  local z_bucket_code
  z_bucket_code=$(rbgu_http_code_capture "depot_bucket_create") || bcu_die "Bad bucket creation HTTP code"
  case "${z_bucket_code}" in
    200|201) bcu_log_args "Build bucket ${z_build_bucket} created" ;;
    409)     bcu_die "Build bucket ${z_build_bucket} already exists" ;;
    *)       bcu_die "Failed to create build bucket: HTTP ${z_bucket_code}" ;;
  esac

  bcu_step 'Create container repository'
  local z_repository_name="rbw-${z_depot_name}-repository"
  local z_parent="projects/${z_depot_project_id}/locations/${z_region}"
  local z_create_repo_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_parent}/repositories?repositoryId=${z_repository_name}"
  local z_create_repo_body="${BDU_TEMP_DIR}/rbgp_create_repo.json"
  
  jq -n '{format:"DOCKER"}' > "${z_create_repo_body}" || bcu_die "Failed to build create-repo body"

  rbgu_http_json_lro_ok \
    "Create container repository" \
    "${z_token}" \
    "${z_create_repo_url}" \
    "depot_repo_create" \
    "${z_create_repo_body}" \
    ".name" \
    "${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}" \
    "${RBGC_OP_PREFIX_GLOBAL}" \
    "${RBGC_EVENTUAL_CONSISTENCY_SEC}" \
    "${RBGC_MAX_CONSISTENCY_SEC}"

  bcu_step 'Create Mason service account'
  local z_mason_name="rbw-${z_depot_name}-mason"
  local z_mason_display_name="Mason for RB Depot: ${z_depot_name}"
  local z_create_sa_body="${BDU_TEMP_DIR}/rbgp_create_mason.json"
  
  jq -n \
    --arg accountId "${z_mason_name}" \
    --arg displayName "${z_mason_display_name}" \
    '{
      accountId: $accountId,
      displayName: $displayName
    }' > "${z_create_sa_body}" || bcu_die "Failed to build Mason creation body"

  local z_create_sa_url="${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/${z_depot_project_id}/serviceAccounts"
  rbgu_http_json "POST" "${z_create_sa_url}" "${z_token}" "depot_mason_create" "${z_create_sa_body}"
  rbgu_http_require_ok "Create Mason service account" "depot_mason_create"
  
  local z_mason_sa_email
  z_mason_sa_email=$(rbgu_json_field_capture "depot_mason_create" '.email') || bcu_die "Failed to get Mason email"

  bcu_step 'Configure Mason permissions'
  # Repository admin
  local z_repo_resource="${z_parent}/repositories/${z_repository_name}"
  rbgi_add_repo_iam_role "${z_token}" "${z_mason_sa_email}" "${z_region}" "${z_repository_name}" \
    "roles/artifactregistry.admin"
  
  # Bucket viewer
  rbgi_add_bucket_iam_role "${z_token}" "${z_build_bucket}" "${z_mason_sa_email}" "roles/storage.objectViewer"
  
  # Project viewer
  rbgi_add_project_iam_role "${z_token}" "Grant Mason Project Viewer" "projects/${z_depot_project_id}" \
    "roles/viewer" "serviceAccount:${z_mason_sa_email}" "mason-viewer"

  bcu_step 'Enable Cloud Build service agent to impersonate Mason'
  local z_cb_service_agent="service-${z_project_number}@gcp-sa-cloudbuild.iam.gserviceaccount.com"
  rbgi_add_sa_iam_role "${z_token}" "${z_mason_sa_email}" "${z_cb_service_agent}" "roles/iam.serviceAccountTokenCreator"

  bcu_step 'Update depot tracking'
  zrbgp_depot_list_update || bcu_die "Failed to update depot tracking after creation"

  # Display Depot Configuration
  bcu_step 'Display depot configuration'
  bcu_success 'Depot creation successful'
  bcu_info "Required RBRR configuration values:"
  bcu_info "  RBRR_DEPOT_PROJECT_ID=${z_depot_project_id}"
  bcu_info "  RBRR_GCP_REGION=${z_region}"
  bcu_info "  RBRR_GAR_REPOSITORY=${z_repository_name}"
  bcu_info "Mason service account: ${z_mason_sa_email}"
  bcu_info "Depot ready for Governor creation"
}

rbgp_depot_destroy() {
  zrbgp_sentinel

  local z_depot_project_id="${1:-}"

  bcu_doc_brief "DANGER: Permanently destroy an entire depot infrastructure"
  bcu_doc_param "depot_project_id" "The depot project ID to destroy"
  bcu_doc_shown || return 0

  bcu_step 'Safety confirmation required'
  test "${DEBUG_ONLY:-}" = "1" || bcu_die "DEBUG_ONLY=1 environment variable required for execution"
  test -n "${z_depot_project_id}" || bcu_die "Depot project ID required as first argument"
  
  bcu_info ""
  bcu_info "==============================================="
  bcu_info "           DANGER: DEPOT DESTRUCTION"
  bcu_info "==============================================="
  bcu_info "Target depot: ${z_depot_project_id}"
  bcu_info ""
  bcu_info "This operation will PERMANENTLY DESTROY:"
  bcu_info "  • Depot project and ALL contained resources"
  bcu_info "  • Mason service account and credentials"
  bcu_info "  • Container repository and ALL images"
  bcu_info "  • Build bucket and ALL artifacts"
  bcu_info "  • Governor, Director, Retriever service accounts"
  bcu_info "  • ALL IAM bindings and permissions"
  bcu_info ""
  bcu_info "Project will enter 30-day retention period."
  bcu_info "Billing will be immediately stopped."
  bcu_info ""
  bcu_info "==============================================="
  bcu_info ""
  
  printf "To confirm destruction, type the exact depot project ID: "
  read -r z_confirmation
  
  if [ "${z_confirmation}" != "${z_depot_project_id}" ]; then
    bcu_die "Confirmation failed. Expected '${z_depot_project_id}', got '${z_confirmation}'"
  fi
  
  bcu_info "Confirmation received. Proceeding with depot destruction."
  bcu_info ""

  bcu_step 'Authenticate as Payor'
  test -n "${RBRP_PAYOR_PROJECT_ID:-}" || bcu_die "RBRP_PAYOR_PROJECT_ID is not set"
  test -n "${RBRP_OAUTH_CLIENT_ID:-}" || bcu_die "RBRP_OAUTH_CLIENT_ID is not set"
  
  local z_token
  z_token=$(zrbgp_authenticate_capture) || bcu_die "Failed to authenticate as Payor via OAuth"

  bcu_step 'Validate target depot'
  test -n "${z_depot_project_id}" || bcu_die "Depot project ID required as first argument"
  
  local z_project_info_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V3}/projects/${z_depot_project_id}"
  rbgu_http_json "GET" "${z_project_info_url}" "${z_token}" "depot_destroy_validate"
  rbgu_http_require_ok "Validate depot project" "depot_destroy_validate"
  
  local z_lifecycle_state
  z_lifecycle_state=$(rbgu_json_field_capture "depot_destroy_validate" '.lifecycleState // "UNKNOWN"') || bcu_die "Failed to parse project lifecycle state"
  
  if [ "${z_lifecycle_state}" != "ACTIVE" ]; then
    if [ "${z_lifecycle_state}" = "DELETE_REQUESTED" ]; then
      bcu_die "Project already marked for deletion"
    else
      bcu_die "Project state is ${z_lifecycle_state} - can only destroy ACTIVE projects"
    fi
  fi

  bcu_step 'Check for and remove liens'
  local z_liens_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens?parent=projects%2F${z_depot_project_id}"
  rbgu_http_json "GET" "${z_liens_url}" "${z_token}" "depot_destroy_liens_list"
  rbgu_http_require_ok "List liens" "depot_destroy_liens_list"
  
  local z_lien_count
  z_lien_count=$(rbgu_json_field_capture "depot_destroy_liens_list" '.liens // [] | length') || bcu_die "Failed to parse liens response"
  
  if [ "${z_lien_count}" -gt 0 ]; then
    bcu_log_args "Found ${z_lien_count} lien(s) - removing them"
    local z_lien_names
    z_lien_names=$(rbgu_json_field_capture "depot_destroy_liens_list" '.liens[].name' | tr '\n' ' ') || bcu_die "Failed to extract lien names"
    
    for z_lien_name in ${z_lien_names}; do
      if [ -n "${z_lien_name}" ]; then
        bcu_log_args "Removing lien: ${z_lien_name}"
        local z_delete_lien_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens/${z_lien_name}"
        rbgu_http_json "DELETE" "${z_delete_lien_url}" "${z_token}" "depot_destroy_lien_delete"
        rbgu_http_require_ok "Delete lien" "depot_destroy_lien_delete"
      fi
    done
  fi

  bcu_step 'Initiate depot deletion'
  local z_delete_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V3}/projects/${z_depot_project_id}"
  rbgu_http_json "DELETE" "${z_delete_url}" "${z_token}" "depot_destroy_delete"
  
  local z_delete_response
  z_delete_response=$(rbgu_http_code_capture "depot_destroy_delete") || bcu_die "Failed to get deletion response code"
  
  if [ "${z_delete_response}" = "200" ] || [ "${z_delete_response}" = "204" ]; then
    bcu_log_args "Project deletion initiated successfully"
  else
    local z_error_msg
    z_error_msg=$(rbgu_json_field_capture "depot_destroy_delete" '.error.message // "Unknown error"') || z_error_msg="HTTP ${z_delete_response}"
    bcu_die "Failed to initiate project deletion: ${z_error_msg}"
  fi

  bcu_step 'Verify deletion state transition'
  local z_max_attempts=12
  local z_attempt=1
  local z_final_state=""
  
  while [ "${z_attempt}" -le "${z_max_attempts}" ]; do
    sleep 5
    bcu_log_args "Checking deletion state (attempt ${z_attempt}/${z_max_attempts})"
    
    rbgu_http_json "GET" "${z_project_info_url}" "${z_token}" "depot_destroy_state_check"
    
    if rbgu_http_is_ok "depot_destroy_state_check"; then
      z_final_state=$(rbgu_json_field_capture "depot_destroy_state_check" '.lifecycleState // "UNKNOWN"') || z_final_state="UNKNOWN"
      
      if [ "${z_final_state}" = "DELETE_REQUESTED" ]; then
        break
      fi
    fi
    
    z_attempt=$((z_attempt + 1))
  done
  
  if [ "${z_final_state}" != "DELETE_REQUESTED" ]; then
    bcu_die "Failed to verify deletion state transition. Current state: ${z_final_state}"
  fi

  bcu_step 'Immediate billing stop'
  local z_billing_body="${BDU_TEMP_DIR}/rbgp_billing_stop.json"
  jq -n \
    --arg projectId "${z_depot_project_id}" \
    '{
      projectId: $projectId,
      billingEnabled: false
    }' > "${z_billing_body}" || bcu_die "Failed to build billing stop body"

  local z_billing_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/projects/${z_depot_project_id}:setBillingInfo"
  rbgu_http_json "PUT" "${z_billing_url}" "${z_token}" "depot_destroy_billing_stop" "${z_billing_body}"
  
  if rbgu_http_is_ok "depot_destroy_billing_stop"; then
    bcu_log_args "Billing immediately stopped"
  else
    bcu_log_args "Warning: Could not immediately stop billing (normal if deletion already in progress)"
  fi

  bcu_step 'Update depot tracking'
  zrbgp_depot_list_update || bcu_log_args "Warning: Failed to update depot tracking after deletion"

  # Success
  bcu_success "Depot ${z_depot_project_id} successfully marked for deletion"
  bcu_info "Project Status: DELETE_REQUESTED"
  bcu_info "Grace period: Up to 30 days"
  bcu_info "Project is now unusable but may remain visible in listings"
  bcu_info "All infrastructure (Mason SA, repository, bucket) will be automatically removed"
}

rbgp_depot_list() {
  zrbgp_sentinel

  bcu_doc_brief "List all depot instances and their status"
  bcu_doc_shown || return 0

  bcu_step 'Authenticate as Payor'
  test -n "${RBRP_PAYOR_PROJECT_ID:-}" || bcu_die "RBRP_PAYOR_PROJECT_ID is not set"
  test -n "${RBRP_OAUTH_CLIENT_ID:-}"  || bcu_die "RBRP_OAUTH_CLIENT_ID is not set"
  
  local z_token
  z_token=$(zrbgp_authenticate_capture) || bcu_die "Failed to authenticate as Payor via OAuth"

  bcu_step 'Query depot projects'
  local z_filter="projectId:rbw-* AND lifecycleState:ACTIVE"
  local z_list_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/projects?filter=${z_filter// /%20}"
  rbgu_http_json "GET" "${z_list_url}" "${z_token}" "depot_list_projects"
  rbgu_http_require_ok "List depot projects" "depot_list_projects"
  
  local z_project_count
  z_project_count=$(rbgu_json_field_capture "depot_list_projects" '.projects // [] | length') || z_project_count=0

  if [ "${z_project_count}" -eq 0 ]; then
    bcu_info "No active depot projects found"
    return 0
  fi

  bcu_step "Validating ${z_project_count} depot(s)"
  
  local z_depot_index=0
  local z_complete_count=0
  local z_broken_count=0
  
  bcu_info ""
  bcu_info "=== DEPOT SUMMARY ==="
  
  while [ "${z_depot_index}" -lt "${z_project_count}" ]; do
    local z_project_id
    z_project_id=$(rbgu_json_field_capture "depot_list_projects" ".projects[${z_depot_index}].projectId") || continue
    
    local z_display_name  
    z_display_name=$(rbgu_json_field_capture "depot_list_projects" ".projects[${z_depot_index}].displayName") || z_display_name="N/A"
    
    # Extract depot name from project ID pattern rbw-NAME-TIMESTAMP
    local z_depot_name=""
    if printf '%s' "${z_project_id}" | grep -qE '^rbw-[a-z0-9-]+-[0-9]{12}$'; then
      z_depot_name=$(printf '%s' "${z_project_id}" | sed 's/^rbw-\(.*\)-[0-9]\{12\}$/\1/')
    fi
    
    # Check depot components
    local z_status="CHECKING"
    local z_region="unknown"
    
    # Try to detect region and validate components
    local z_mason_expected="rbw-${z_depot_name}-mason"
    local z_repo_expected="rbw-${z_depot_name}-repository"  
    local z_bucket_expected="rbw-${z_depot_name}-bucket"
    
    # Quick validation - check if Mason service account exists
    local z_mason_url="${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/${z_project_id}/serviceAccounts/${z_mason_expected}@${z_project_id}.iam.gserviceaccount.com"
    rbgu_http_json "GET" "${z_mason_url}" "${z_token}" "depot_list_mason_${z_depot_index}" || true
    
    if rbgu_http_is_ok "depot_list_mason_${z_depot_index}"; then
      z_status="COMPLETE"
      z_complete_count=$((z_complete_count + 1))
    else
      z_status="BROKEN"
      z_broken_count=$((z_broken_count + 1))
    fi
    
    # Display depot info
    printf "%-25s %-20s %-15s %s\n" "${z_project_id}" "${z_depot_name:-N/A}" "${z_region}" "${z_status}"
    
    z_depot_index=$((z_depot_index + 1))
  done
  
  bcu_info ""
  bcu_info "=== SUMMARY ==="
  bcu_info "Total depots: ${z_project_count}"
  bcu_info "Complete: ${z_complete_count}"
  bcu_info "Broken: ${z_broken_count}"
  
  if [ "${z_broken_count}" -gt 0 ]; then
    bcu_info ""
    bcu_info "Note: BROKEN depots may have missing resources (Mason SA, repository, or bucket)"
    bcu_info "Consider investigating broken depots or destroying them if no longer needed"
  fi
}

rbgp_payor_oauth_refresh() {
  zrbgp_sentinel

  bcu_doc_brief "Refresh expired OAuth credentials following RBAGS manual procedure"
  bcu_doc_lines "Use this when OAuth tokens expire after 6 months or are compromised"
  bcu_doc_lines "Requires downloading new OAuth JSON from Google Cloud Console"
  bcu_doc_shown || return 0

  bcu_step 'Display OAuth refresh procedure'
  bcu_info ""
  bcu_info "=== Manual Payor OAuth Refresh Procedure ==="
  bcu_info ""
  bcu_info "OAuth credentials need to be refreshed. Follow these steps:"
  bcu_info ""
  bcu_info "1. Navigate to APIs & Services > Credentials in Payor Project"
  bcu_info "   Console URL: https://console.cloud.google.com/apis/credentials?project=${RBRP_PAYOR_PROJECT_ID:-rbw-payor}"
  bcu_info ""
  bcu_info "2. Find existing 'Recipe Bottle Payor' OAuth client"
  bcu_info ""  
  bcu_info "3. Download new JSON credentials:"
  bcu_info "   - Click the download icon next to the OAuth client"
  bcu_info "   - Or regenerate client secret if compromised"
  bcu_info ""
  bcu_info "4. Save as timestamped file:"
  bcu_info "   - Example: payor-oauth-$(date +%Y%m%d).json"
  bcu_info ""
  bcu_info "5. Run installation command with new JSON:"
  bcu_info "   rbgp_payor_install /path/to/payor-oauth-[timestamp].json"
  bcu_info ""
  bcu_info "This will regenerate OAuth credentials and update RBRO file."
  bcu_info ""
  
  bcu_success "OAuth refresh procedure displayed"
  bcu_info "Note: OAuth refresh tokens expire after 6 months of non-use in testing mode"
  bcu_info "Any successful payor operation resets the 6-month timer"
}

# eof

