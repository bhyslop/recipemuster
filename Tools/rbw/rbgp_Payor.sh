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
test -z "${ZRBGP_SOURCED:-}" || buc_die "Module rbgp multiply sourced - check sourcing hierarchy"
ZRBGP_SOURCED=1

######################################################################
# Internal Functions (zrbgp_*)

zrbgp_kindle() {
  test -z "${ZRBGP_KINDLED:-}" || buc_die "Module rbgp already kindled"

  buc_log_args "Ensure dependencies are kindled first"
  zrbgc_sentinel
  zrbgo_sentinel
  zrbgu_sentinel
  zrbgi_sentinel

  readonly ZRBGP_PREFIX="${BURD_TEMP_DIR}/rbgp_"
  readonly ZRBGP_EMPTY_JSON="${ZRBGP_PREFIX}empty.json"
  printf '{}' > "${ZRBGP_EMPTY_JSON}"

  # Infix values for HTTP operations
  readonly ZRBGP_INFIX_PROJECT_DELETE="project_delete"
  readonly ZRBGP_INFIX_PROJECT_RESTORE="project_restore"
  readonly ZRBGP_INFIX_PROJECT_STATE="project_state"
  readonly ZRBGP_INFIX_LIST_LIENS="list_liens"
  readonly ZRBGP_INFIX_DELETE_LIEN="delete_lien"
  readonly ZRBGP_INFIX_BILLING_ATTACH="billing_attach"
  readonly ZRBGP_INFIX_BILLING_DETACH="billing_detach"
  readonly ZRBGP_INFIX_CREATE_REPO="create_repo"
  readonly ZRBGP_INFIX_VERIFY_REPO="verify_repo"
  readonly ZRBGP_INFIX_PROJECT_INFO="project_info"
  readonly ZRBGP_INFIX_BUCKET_CREATE="bucket_create"
  readonly ZRBGP_INFIX_API_CHECK="api_checking"
  readonly ZRBGP_INFIX_GOV_LIST_SA="gov_list_sa"
  readonly ZRBGP_INFIX_GOV_DELETE_SA="gov_delete_sa"
  readonly ZRBGP_INFIX_GOV_CREATE_SA="gov_create_sa"
  readonly ZRBGP_INFIX_GOV_VERIFY_SA="gov_verify_sa"
  readonly ZRBGP_INFIX_GOV_KEY="gov_key"
  readonly ZRBGP_INFIX_GOV_IAM="gov_iam"

  readonly ZRBGP_KINDLED=1
}

zrbgp_sentinel() {
  test "${ZRBGP_KINDLED:-}" = "1" || buc_die "Module rbgp not kindled - call zrbgp_kindle first"
}


######################################################################
# OAuth Authentication Functions (zrbgp_oauth_*)

zrbgp_refresh_capture() {
  zrbgp_sentinel

  # RBRO credentials already loaded and validated by caller (rbgu_rbro_load)
  zrbro_sentinel
  test -n "${RBRP_OAUTH_CLIENT_ID:-}" || buc_die "RBRP_OAUTH_CLIENT_ID not set in environment"

  buc_log_args "Exchanging refresh token for access token"

  # Build request and pipe to curl - secrets never touch disk
  local z_response
  z_response=$(jq -n \
    --arg refresh_token "${RBRO_REFRESH_TOKEN}" \
    --arg client_id "${RBRP_OAUTH_CLIENT_ID}" \
    --arg client_secret "${RBRO_CLIENT_SECRET}" \
    --arg grant_type "refresh_token" \
    '{
      refresh_token: $refresh_token,
      client_id: $client_id,
      client_secret: $client_secret,
      grant_type: $grant_type
    }' | curl -s -X POST \
      -H "Content-Type: application/json" \
      -d @- \
      "https://oauth2.googleapis.com/token") || buc_die "Failed to execute OAuth refresh request"

  # Check for error in response
  local z_error
  z_error=$(jq -r '.error // empty' <<<"${z_response}")
  if test -n "${z_error}"; then
    local z_error_desc
    z_error_desc=$(jq -r '.error_description // .error // "Unknown error"' <<<"${z_response}")
    buc_die "OAuth credentials expired or invalid - run rbgp_payor_oauth_refresh: ${z_error_desc}"
  fi

  local z_access_token
  z_access_token=$(jq -r '.access_token // empty' <<<"${z_response}")
  test -n "${z_access_token}" || buc_die "OAuth response missing access_token"

  echo "${z_access_token}"
}

# RBTOE: Payor OAuth Authentication Pattern
# Establishes Payor OAuth context by loading RBRO credentials and obtaining access token
zrbgp_authenticate_capture() {
  zrbgp_sentinel
  
  buc_log_args "Establishing Payor OAuth authentication context"
  
  # Load RBRO credentials
  rbgu_rbro_load
  
  # Load RBRP_OAUTH_CLIENT_ID from environment
  test -n "${RBRP_OAUTH_CLIENT_ID:-}" || buc_die "RBRP_OAUTH_CLIENT_ID not set in environment"
  
  # Exchange refresh token for access token
  local z_access_token
  z_access_token=$(zrbgp_refresh_capture) || buc_die "Failed to exchange OAuth refresh token"
  
  test -n "${z_access_token}" || buc_die "Empty access token from OAuth exchange"
  
  buc_log_args "Payor OAuth authentication successful"
  echo "${z_access_token}"
}

zrbgp_depot_list_update() {
  zrbgp_sentinel

  buc_log_args "Updating depot project tracking in RBRP configuration"

  # Get OAuth access token
  local z_access_token
  z_access_token=$(zrbgp_authenticate_capture) || buc_die "Failed to authenticate for depot list update"

  # Query active depot projects (CRM v1 allows listing accessible projects without parent)
  # Using v1 instead of v3 which requires parent parameter
  local z_list_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/projects"
  rbgu_http_json "GET" "${z_list_url}" "${z_access_token}" "depot_list_tracking"

  # Non-blocking: if query fails, just log and continue (normal on first install with no projects)
  if ! rbgu_http_require_ok "Query depot projects" "depot_list_tracking" 2>/dev/null; then
    buc_log_args "Depot project list query skipped (expected on first install or API access restrictions)"
    return 0
  fi
  
  # Extract and validate depot project IDs
  local z_depot_ids=""
  local z_project_count
  z_project_count=$(rbgu_json_field_capture "depot_list_tracking" '.projects // [] | length') || z_project_count=0
  
  if [ "${z_project_count}" -gt 0 ]; then
    local z_index=0
    while [ "${z_index}" -lt "${z_project_count}" ]; do
      local z_project_id
      z_project_id=$(rbgu_json_field_capture "depot_list_tracking" ".projects[${z_index}].projectId") || continue
      
      # Validate depot project ID pattern (global namespace)
      if printf '%s' "${z_project_id}" | grep -qE "${RBGC_GLOBAL_DEPOT_REGEX}"; then
        z_depot_ids="${z_depot_ids} ${z_project_id}"
      else
        buc_log_args "Warning: Skipping project with invalid depot pattern: ${z_project_id}"
      fi
      
      z_index=$((z_index + 1))
    done
  fi
  
  # Update RBRP configuration
  # Note: This would normally update rbrp.env file
  # For now, we'll export to environment and log the result
  buc_log_args "Found ${z_project_count} depot projects, ${z_depot_ids:+ $(($(printf '%s' "${z_depot_ids}" | wc -w))):+0} valid"

  return 0
}

######################################################################
# External Functions (rbgp_*)

zrbgp_billing_attach() {
  zrbgp_sentinel

  local z_billing_account="${1:-}"

  buc_doc_brief "Attach a billing account to the project"
  buc_doc_param "billing_account" "Billing account ID to attach"
  buc_doc_shown || return 0

  test -n "${z_billing_account}" || buc_die "Billing account ID required"

  buc_step "Attaching billing account: ${z_billing_account}"

  local z_token
  z_token=$(rbgu_get_governor_token_capture) || buc_die "Failed to get admin token"
  local z_billing_body="${BURD_TEMP_DIR}/rbgp_billing_attach.json"
  jq -n --arg billingAccountName "billingAccounts/${z_billing_account}" \
    --arg projectId "${RBRR_DEPOT_PROJECT_ID}" \
    '{
      billingAccountName: $billingAccountName,
      projectId: $projectId,
      billingEnabled: true
    }' > "${z_billing_body}" || buc_die "Failed to build billing attach body"

  local z_billing_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/projects/${RBRR_DEPOT_PROJECT_ID}:setBillingInfo"
  rbgu_http_json "PUT" "${z_billing_url}" "${z_token}" \
                                  "${ZRBGP_INFIX_BILLING_ATTACH}" "${z_billing_body}"
  rbgu_http_require_ok "Attach billing account" "${ZRBGP_INFIX_BILLING_ATTACH}"

  buc_success "Billing account ${z_billing_account} attached to project"
}

zrbgp_billing_detach() {
  zrbgp_sentinel

  buc_doc_brief "Detach billing account from the project"
  buc_doc_shown || return 0

  buc_step "Detaching billing account from project"

  local z_token
  z_token=$(rbgu_get_governor_token_capture) || buc_die "Failed to get admin token"
  local z_billing_body="${BURD_TEMP_DIR}/rbgp_billing_detach.json"
  jq -n --arg projectId "${RBRR_DEPOT_PROJECT_ID}" \
    '{
      projectId: $projectId,
      billingEnabled: false
    }' > "${z_billing_body}" || buc_die "Failed to build billing detach body"

  local z_billing_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/projects/${RBRR_DEPOT_PROJECT_ID}:setBillingInfo"
  rbgu_http_json "PUT" "${z_billing_url}" "${z_token}" \
                                  "${ZRBGP_INFIX_BILLING_DETACH}" "${z_billing_body}"
  rbgu_http_require_ok "Detach billing account" "${ZRBGP_INFIX_BILLING_DETACH}"

  buc_success "Billing account detached from project"
}



zrbgp_liens_list() {
  zrbgp_sentinel

  buc_doc_brief "List all liens on the project"
  buc_doc_shown || return 0

  buc_step "Listing liens on project: ${RBRR_DEPOT_PROJECT_ID}"

  local z_token
  z_token=$(rbgu_get_governor_token_capture) || buc_die "Failed to get admin token"
  rbgu_http_json "GET" "${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens?parent=projects/${RBRR_DEPOT_PROJECT_ID}" "${z_token}" "${ZRBGP_INFIX_LIST_LIENS}"
  rbgu_http_require_ok "List liens" "${ZRBGP_INFIX_LIST_LIENS}"

  local z_lien_count
  z_lien_count=$(rbgu_json_field_capture "${ZRBGP_INFIX_LIST_LIENS}" '.liens // [] | length') || buc_die "Failed to parse liens response"

  if [[ "${z_lien_count}" -eq 0 ]]; then
    buc_info "No liens found on project"
    return 0
  fi

  buc_step "Found ${z_lien_count} lien(s):"
  jq -r '.liens[]? | "  - " + .name + " (reason: " + .reason + ")"' \
    "${ZRBGU_PREFIX}${ZRBGP_INFIX_LIST_LIENS}${ZRBGU_POSTFIX_JSON}" || true

  return 0
}

zrbgp_lien_delete() {
  zrbgp_sentinel

  local z_lien_name="${1:-}"

  buc_doc_brief "Delete a specific lien from the project"
  buc_doc_param "lien_name" "Full resource name of the lien to delete"
  buc_doc_shown || return 0

  test -n "${z_lien_name}" || buc_die "Lien name required"

  buc_step "Deleting lien: ${z_lien_name}"

  local z_token
  z_token=$(rbgu_get_governor_token_capture) || buc_die "Failed to get admin token"
  rbgu_http_json "DELETE" "${RBGC_API_CRM_DELETE_LIEN}/${z_lien_name}" "${z_token}" "${ZRBGP_INFIX_DELETE_LIEN}"
  rbgu_http_require_ok "Delete lien" "${ZRBGP_INFIX_DELETE_LIEN}" 404 "not found (already deleted)"

  buc_success "Lien deleted: ${z_lien_name}"
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

    buc_log_args 'If we cannot even read an HTTP code file, that is a processing failure.'
    z_code=$(rbgu_http_code_capture "${z_infix}") || z_code=""
    test -n "${z_code}" || return 1

    if test "${z_code}" = "200"; then
      z_state=$(rbgu_json_field_capture "${z_infix}" ".state") || z_state=""
      test "${z_state}" = "ENABLED" || z_missing="${z_missing} ${z_service}"
    else
      buc_log_args 'Any non-200 (403/404/5xx/etc) => treat as NOT enabled'
      z_missing="${z_missing} ${z_service}"
    fi
  done

  printf '%s' "${z_missing# }"
}

zrbgp_get_project_number_capture() {
  zrbgp_sentinel

  local z_token
  z_token=$(rbgu_get_governor_token_capture) || return 1

  rbgu_http_json "GET" "${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}${RBGC_PATH_PROJECTS}/${RBRR_DEPOT_PROJECT_ID}" "${z_token}" "${ZRBGP_INFIX_PROJECT_INFO}"
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

  buc_log_args 'Create bucket request JSON for '"${z_bucket_name}"
  local z_bucket_req="${BURD_TEMP_DIR}/rbgp_bucket_create_req.json"
  jq -n --arg name "${z_bucket_name}" --arg location "${RBGC_GAR_LOCATION}" '
    {
      name: $name,
      location: $location,
      storageClass: "STANDARD",
      lifecycle: { rule: [ { action: { type: "Delete" }, condition: { age: 1 } } ] }
    }' > "${z_bucket_req}" || buc_die "Failed to create bucket request JSON"

  buc_log_args 'Send bucket creation request'
  local z_code
  local z_err
  rbgu_http_json "POST" "${RBGC_API_GCS_BUCKETS}?project=${RBRR_DEPOT_PROJECT_ID}" "${z_token}" \
                                  "${ZRBGP_INFIX_BUCKET_CREATE}" "${z_bucket_req}"
  z_code=$(rbgu_http_code_capture "${ZRBGP_INFIX_BUCKET_CREATE}") || buc_die "Bad bucket creation HTTP code"
  z_err=$(rbgu_json_field_capture "${ZRBGP_INFIX_BUCKET_CREATE}" '.error.message') || z_err="HTTP ${z_code}"

  case "${z_code}" in
    200|201) buc_info "Bucket ${z_bucket_name} created";                         return 0 ;;
    409)     buc_die  "Bucket ${z_bucket_name} already exists (pristine-state violation)" ;;
    *)       buc_die  "Failed to create bucket: ${z_err}"                                 ;;
  esac
}

######################################################################
# External Functions (rbgp_*)

rbgp_payor_install() {
  zrbgp_sentinel

  local z_oauth_json_file="${1:-}"

  buc_doc_brief "Install Payor OAuth credentials from client JSON file following RBAGS specification"
  buc_doc_param "oauth_json_file" "Path to downloaded OAuth client JSON file from establish procedure"
  buc_doc_lines "REQUIREMENT: OAuth consent screen must be configured in testing mode"
  buc_doc_lines "            and the Payor project must have required APIs enabled"
  buc_doc_lines "REQUIREMENT: RBRP_BILLING_ACCOUNT_ID must be set in environment"
  buc_doc_shown || return 0

  buc_step 'Validate environment prerequisites'
  test -n "${RBRP_BILLING_ACCOUNT_ID:-}" || buc_die "RBRP_BILLING_ACCOUNT_ID not set in environment - obtain from Cloud Console Billing and set before proceeding"

  buc_step 'Validate input parameters'
  test -n "${z_oauth_json_file}" || buc_die "OAuth JSON file path required as first argument"
  test -f "${z_oauth_json_file}" || buc_die "OAuth JSON file not found: ${z_oauth_json_file}"

  buc_step 'Parse OAuth client JSON'
  local z_client_id
  z_client_id=$(jq -r '.installed.client_id // .client_id // empty' "${z_oauth_json_file}" 2>/dev/null) || buc_die "Failed to parse OAuth JSON file"
  test -n "${z_client_id}" || buc_die "OAuth JSON file missing client_id field"
  
  local z_client_secret
  z_client_secret=$(jq -r '.installed.client_secret // .client_secret // empty' "${z_oauth_json_file}" 2>/dev/null) || buc_die "Failed to extract client_secret from OAuth JSON file"
  test -n "${z_client_secret}" || buc_die "OAuth JSON file missing client_secret field"
  
  local z_project_id
  z_project_id=$(jq -r '.installed.project_id // .project_id // empty' "${z_oauth_json_file}" 2>/dev/null) || buc_die "Failed to extract project_id from OAuth JSON file"
  test -n "${z_project_id}" || buc_die "OAuth JSON file missing project_id field"

  buc_step 'Check existing credentials'
  local z_rbro_file="${RBRR_PAYOR_RBRO_FILE}"
  if test -f "${z_rbro_file}"; then
    buc_log_args "Existing RBRO credentials will be replaced"
  fi

  local z_refresh_token=""
  buc_step 'OAuth authorization flow'
  local z_auth_url="https://accounts.google.com/o/oauth2/v2/auth?client_id=${z_client_id}&redirect_uri=urn:ietf:wg:oauth:2.0:oob&scope=https://www.googleapis.com/auth/cloud-platform%20https://www.googleapis.com/auth/cloud-billing&response_type=code&access_type=offline"

  bug_e
  bug_link "Open this URL in your browser: " "Google OAuth Authorization" "${z_auth_url}"
  bug_e
  bug_t  "You will see three or four screens:"
  bug_tut "  1. " "Choose an account" " - Select the Google account for this payor"
  bug_tut "  2. If screen says " "Google hasn't verified this app" ", click Continue"
  bug_t   "     Otherwise, proceed to next step"
  bug_tut "  3. " "Recipe Bottle Payor wants access" " - Review the requested permissions"
  bug_tu  "     Check the permission checkboxes to grant access, then click " "Continue"
  bug_t   "  4. Authorization code will be displayed"
  bug_e
  local z_auth_code
  z_auth_code=$(bug_prompt "Copy the authorization code and paste here: ")
  test -n "${z_auth_code}" || buc_die "Authorization code is required"

  buc_log_args "Exchanging authorization code for tokens"

  # Build request and pipe to curl - secrets never touch disk
  local z_response
  z_response=$(jq -n \
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
    }' | curl -s -X POST \
      -H "Content-Type: application/json" \
      -d @- \
      "https://oauth2.googleapis.com/token") || buc_die "Failed to execute token exchange request"

  # Check for error in response
  local z_error
  z_error=$(jq -r '.error // empty' <<<"${z_response}")
  if test -n "${z_error}"; then
    local z_error_desc
    z_error_desc=$(jq -r '.error_description // .error // "Unknown error"' <<<"${z_response}")
    buc_die "OAuth token exchange failed: ${z_error_desc}"
  fi

  z_refresh_token=$(jq -r '.refresh_token // empty' <<<"${z_response}")
  test -n "${z_refresh_token}" || buc_die "OAuth response missing refresh_token field"

  buc_step 'Create credentials directory'
  local z_rbro_dir
  z_rbro_dir="$(dirname "${z_rbro_file}")"
  mkdir -p "${z_rbro_dir}" || buc_die "Failed to create credentials directory: ${z_rbro_dir}"
  chmod 700 "${z_rbro_dir}" || buc_die "Failed to set credentials directory permissions"

  buc_step 'Store OAuth credentials'
  cat > "${z_rbro_file}" <<-EOF || buc_die "Failed to write RBRO credentials file"
RBRO_CLIENT_SECRET=${z_client_secret}
RBRO_REFRESH_TOKEN=${z_refresh_token}
EOF
  chmod 600 "${z_rbro_file}" || buc_die "Failed to set RBRO file permissions"
  
  buc_step 'Validate public configuration'
  buc_log_args "Validating RBRP_OAUTH_CLIENT_ID matches OAuth JSON"

  if test -z "${RBRP_OAUTH_CLIENT_ID:-}"; then
    buc_info "RBRP_OAUTH_CLIENT_ID missing from rbrp.env"
    buc_info "Add this line to rbrp.env and commit:"
    buc_code "echo 'RBRP_OAUTH_CLIENT_ID=${z_client_id}' >> rbrp.env"
    buc_die "RBRP_OAUTH_CLIENT_ID must be configured before payor_install"
  fi

  if test "${RBRP_OAUTH_CLIENT_ID}" != "${z_client_id}"; then
    buc_info "RBRP_OAUTH_CLIENT_ID mismatch"
    buc_info "  rbrp.env has: ${RBRP_OAUTH_CLIENT_ID}"
    buc_info "  OAuth JSON:   ${z_client_id}"
    buc_info "Fix with:"
    buc_code "sed -i '' 's|^RBRP_OAUTH_CLIENT_ID=.*|RBRP_OAUTH_CLIENT_ID=${z_client_id}|' rbrp.env"
    buc_die "RBRP_OAUTH_CLIENT_ID in rbrp.env does not match OAuth JSON"
  fi

  buc_log_args "RBRP_OAUTH_CLIENT_ID validated: ${z_client_id}"
  
  buc_step 'Test OAuth authentication'
  local z_access_token
  z_access_token=$(zrbgp_authenticate_capture) || buc_die "Failed to test OAuth authentication"
  test -n "${z_access_token}" || buc_die "OAuth authentication test returned empty token"
  
  buc_step 'Discover operator email'
  rbgu_http_json "GET" "https://www.googleapis.com/oauth2/v3/userinfo" "${z_access_token}" "payor_userinfo"
  rbgu_http_require_ok "Discover operator email" "payor_userinfo"
  local z_operator_email
  z_operator_email=$(rbgu_json_field_capture "payor_userinfo" '.email') \
    || buc_die "Userinfo response missing email — ensure OAuth scope includes email"
  test -n "${z_operator_email}" || buc_die "Userinfo response missing email"
  buc_info "Operator email: ${z_operator_email}"

  buc_step 'Verify payor project access'
  local z_project_info_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/projects/${z_project_id}"
  rbgu_http_json "GET" "${z_project_info_url}" "${z_access_token}" "payor_verify" 
  rbgu_http_require_ok "Verify payor project" "payor_verify"
  
  local z_project_state
  z_project_state=$(rbgu_json_field_capture "payor_verify" '.lifecycleState') || buc_die "Failed to get project state"
  test "${z_project_state}" = "ACTIVE" || buc_die "Payor project is not ACTIVE (state: ${z_project_state})"

  buc_success "Payor OAuth installation completed successfully"
  buc_info "Credentials stored: ${z_rbro_file}"
  buc_info ""
  buc_info "Configuration required in rbrp.env:"
  buc_info "  RBRP_PAYOR_PROJECT_ID=${z_project_id}"
  buc_info "  RBRP_OAUTH_CLIENT_ID=${z_client_id}"
  buc_info "  RBRP_OPERATOR_EMAIL=${z_operator_email}"
  buc_info "  RBRP_BILLING_ACCOUNT_ID=<obtain from Cloud Console Billing>"
  buc_info ""
  buc_info "Next: rbgp_depot_create <depot-name> <region>"
  buc_info "  Example: rbgp_depot_create dev us-central1"
}

rbgp_depot_create() {
  zrbgp_sentinel

  local z_depot_name="${1:-}"
  local z_region="${RBRR_GCP_REGION}"

  buc_doc_brief "Create new depot infrastructure following RBAGS specification"
  buc_doc_param "depot_name" "Depot name (lowercase/numbers/hyphens, max ${RBGC_GLOBAL_DEPOT_NAME_MAX} chars)"
  buc_doc_shown || return 0

  buc_step 'Validate input parameters'
  test -n "${z_depot_name}" || buc_die "Depot name required as first argument"
  
  if ! printf '%s' "${z_depot_name}" | grep -qE '^[a-z0-9-]+$'; then
    buc_die "Depot name must contain only lowercase letters, numbers, and hyphens"
  fi
  
  if [ "${#z_depot_name}" -gt "${RBGC_GLOBAL_DEPOT_NAME_MAX}" ]; then
    buc_die "Depot name must be ${RBGC_GLOBAL_DEPOT_NAME_MAX} characters or less"
  fi

  buc_step 'Read GitHub credentials from stdin'
  local z_github_pat=""
  local z_github_app_installation_id=""
  if ! read -r z_github_pat 2>/dev/null || test -z "${z_github_pat}"; then
    buc_info ""
    buc_info "GitHub credentials required via stdin (two lines: PAT, then installation ID)."
    buc_info ""
    buc_info "Setup:"
    buc_info "  1. Install the 'Google Cloud Build' GitHub App on your org:"
    buc_bare "     https://github.com/apps/google-cloud-build"
    buc_info "  2. Create a classic PAT with scopes: repo, read:user, read:org"
    buc_info "  3. Note the numeric installation ID from the App settings page"
    buc_info ""
    buc_info "Note: other git hosting services are supported — GitHub is the reference implementation"
    buc_die "Provide PAT and installation ID via stdin and re-run"
  fi
  if ! read -r z_github_app_installation_id 2>/dev/null || test -z "${z_github_app_installation_id}"; then
    buc_die "Second stdin line (GitHub App installation ID) is missing or empty"
  fi
  buc_log_args "GitHub credentials read from stdin"

  buc_step 'Validate Rubric Repo URL'
  local -r z_rubric_repo_url="${RBRR_RUBRIC_REPO_URL:-}"
  test -n "${z_rubric_repo_url}" || buc_die "RBRR_RUBRIC_REPO_URL not set in rbrr.env"

  # Construct authenticated URL for validation (PAT must not persist beyond this check)
  local z_auth_url
  z_auth_url=$(printf '%s' "${z_rubric_repo_url}" | sed "s|https://|https://x-access-token:${z_github_pat}@|") \
    || buc_die "Failed to construct authenticated rubric repo URL"

  git ls-remote "${z_auth_url}" HEAD >/dev/null 2>&1 \
    || buc_die "Rubric repo URL unreachable or PAT invalid — check RBRR_RUBRIC_REPO_URL and PAT scopes"
  unset z_auth_url
  buc_log_args "Rubric repo URL validated"

  # Validate region exists in Artifact Registry locations
  buc_log_args 'Validating region exists in Artifact Registry locations'
  local z_token
  z_token=$(zrbgp_authenticate_capture) || buc_die "Failed to authenticate as Payor for region validation"
  
  local z_locations_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/projects/${RBRP_PAYOR_PROJECT_ID}/locations"
  rbgu_http_json "GET" "${z_locations_url}" "${z_token}" "region_validation"
  rbgu_http_require_ok "Validate region" "region_validation"
  
  local z_valid_regions
  z_valid_regions=$(rbgu_json_field_capture "region_validation" '.locations[].locationId' | tr '\n' ' ') || buc_die "Failed to parse region list"
  
  if ! printf '%s' "${z_valid_regions}" | grep -qw "${z_region}"; then
    buc_die "Invalid region. Valid regions: ${z_valid_regions}"
  fi

  buc_step 'Authenticate as Payor'
  test -n "${RBRP_PAYOR_PROJECT_ID:-}" || buc_die "RBRP_PAYOR_PROJECT_ID is not set"
  test -n "${RBRP_BILLING_ACCOUNT_ID:-}" || buc_die "RBRP_BILLING_ACCOUNT_ID is not set"
  test -n "${RBRP_OAUTH_CLIENT_ID:-}" || buc_die "RBRP_OAUTH_CLIENT_ID is not set"
  
  local z_token
  z_token=$(zrbgp_authenticate_capture) || buc_die "Failed to authenticate as Payor via OAuth"

  buc_step 'Generate depot project ID'
  local z_timestamp
  z_timestamp=$(date "${RBGC_GLOBAL_TIMESTAMP_FORMAT}") || buc_die "Failed to generate timestamp"
  local z_depot_project_id="${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_DEPOT}-${z_depot_name}-${z_timestamp}"
  
  if [ "${#z_depot_project_id}" -gt 30 ]; then
    buc_die "Generated project ID too long (${#z_depot_project_id} > 30): ${z_depot_project_id}"
  fi
  
  buc_log_args "Generated depot project ID: ${z_depot_project_id}"

  buc_step 'Create depot project'
  local z_create_project_body="${BURD_TEMP_DIR}/rbgp_create_project.json"
  
  # OAuth users create projects without parent (per MPCR)
  jq -n \
    --arg projectId "${z_depot_project_id}" \
    --arg displayName "RB Depot ${z_depot_name}" \
    '{
      projectId: $projectId,
      displayName: $displayName
    }' > "${z_create_project_body}" || buc_die "Failed to build project creation body"

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

  buc_step 'Link billing account'
  local z_billing_body="${BURD_TEMP_DIR}/rbgp_billing_link.json"
  jq -n \
    --arg billingAccountName "billingAccounts/${RBRP_BILLING_ACCOUNT_ID}" \
    '{
      billingAccountName: $billingAccountName
    }' > "${z_billing_body}" || buc_die "Failed to build billing link body"

  local z_billing_url="${RBGC_API_ROOT_CLOUDBILLING}${RBGC_CLOUDBILLING_V1}/projects/${z_depot_project_id}/billingInfo"
  rbgu_http_json "PUT" "${z_billing_url}" "${z_token}" "depot_billing_link" "${z_billing_body}"
  rbgu_http_require_ok "Link billing account" "depot_billing_link"

  buc_step 'Get depot project number'
  local z_project_info_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V3}/projects/${z_depot_project_id}"
  rbgu_http_json "GET" "${z_project_info_url}" "${z_token}" "depot_project_info"
  rbgu_http_require_ok "Get project info" "depot_project_info"
  
  local z_project_number
  # CRM v3 returns project number in name field as "projects/{number}"
  z_project_number=$(rbgu_json_field_capture "depot_project_info" '.name | sub("projects/"; "")') || buc_die "Failed to get project number"
  test -n "${z_project_number}" || buc_die "Project number is empty"

  buc_step 'Enable depot project APIs'
  local z_api_services="artifactregistry cloudbuild cloudresourcemanager containeranalysis iam secretmanager serviceusage storage"
  for z_service in ${z_api_services}; do
    rbgu_api_enable "${z_service}" "${z_depot_project_id}" "${z_token}"
  done

  # Note: OAuth Payor doesn't need explicit permissions on depot since it uses user identity
  # Skip Payor permission grants - OAuth user context provides necessary access

  buc_step 'Verify IAM propagation before resource creation'
  local z_preflight_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/projects/${z_depot_project_id}/locations/${z_region}/repositories"
  rbgu_poll_get_until_ok "AR IAM propagation" "${z_preflight_url}" "${z_token}" "iam_preflight"

  buc_step 'Create build bucket'
  local z_build_bucket="${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_BUCKET}-${z_depot_name}-${z_timestamp}"
  local z_bucket_req="${BURD_TEMP_DIR}/rbgp_bucket_create_req.json"
  jq -n \
    --arg name "${z_build_bucket}" \
    --arg location "${z_region}" \
    --arg project "${z_depot_project_id}" \
    '{
      name: $name,
      location: $location,
      storageClass: "STANDARD",
      lifecycle: { rule: [ { action: { type: "Delete" }, condition: { age: 1 } } ] }
    }' > "${z_bucket_req}" || buc_die "Failed to create bucket request JSON"

  local z_bucket_create_url="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b?project=${z_depot_project_id}"
  rbgu_http_json "POST" "${z_bucket_create_url}" "${z_token}" "depot_bucket_create" "${z_bucket_req}"
  
  local z_bucket_code
  z_bucket_code=$(rbgu_http_code_capture "depot_bucket_create") || buc_die "Bad bucket creation HTTP code"
  case "${z_bucket_code}" in
    200|201) buc_log_args "Build bucket ${z_build_bucket} created" ;;
    409)     buc_die "Build bucket ${z_build_bucket} already exists" ;;
    *)       buc_die "Failed to create build bucket: HTTP ${z_bucket_code}" ;;
  esac

  buc_step 'Create container repository'
  local z_repository_name="rbw-${z_depot_name}-repository"
  local z_parent="projects/${z_depot_project_id}/locations/${z_region}"
  local z_create_repo_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_parent}/repositories?repositoryId=${z_repository_name}"
  local z_create_repo_body="${BURD_TEMP_DIR}/rbgp_create_repo.json"
  
  jq -n '{format:"DOCKER"}' > "${z_create_repo_body}" || buc_die "Failed to build create-repo body"

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

  buc_step 'Verify IAM API is ready for service account creation'
  local z_iam_preflight_url="${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/${z_depot_project_id}/serviceAccounts"
  rbgu_poll_get_until_ok "IAM API" "${z_iam_preflight_url}" "${z_token}" "iam_sa_preflight"

  buc_step 'Create Mason service account'
  local z_mason_name="${RBGC_MASON_PREFIX}-${z_depot_name}"
  local z_mason_display_name="Mason for RB Depot: ${z_depot_name}"
  local z_create_sa_body="${BURD_TEMP_DIR}/rbgp_create_mason.json"
  
  jq -n \
    --arg accountId "${z_mason_name}" \
    --arg displayName "${z_mason_display_name}" \
    '{
      accountId: $accountId,
      serviceAccount: {
        displayName: $displayName
      }
    }' > "${z_create_sa_body}" || buc_die "Failed to build Mason creation body"

  local z_create_sa_url="${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/${z_depot_project_id}/serviceAccounts"
  rbgu_http_json "POST" "${z_create_sa_url}" "${z_token}" "depot_mason_create" "${z_create_sa_body}"
  rbgu_http_require_ok "Create Mason service account" "depot_mason_create"
  
  local z_mason_sa_email
  z_mason_sa_email=$(rbgu_json_field_capture "depot_mason_create" '.email') || buc_die "Failed to get Mason email"

  buc_step 'Configure Mason permissions'
  # Repository admin (AR repo IAM requires email, not numeric ID)
  rbgi_add_repo_iam_role "${z_token}" "${z_depot_project_id}" "${z_mason_sa_email}" "${z_region}" "${z_repository_name}" \
    "roles/artifactregistry.writer"

  # Bucket viewer (GCS bucket IAM requires email, not numeric ID)
  rbgi_add_bucket_iam_role "${z_token}" "${z_build_bucket}" "${z_mason_sa_email}" "roles/storage.objectViewer"

  # Project viewer
  rbgi_add_project_iam_role "${z_token}" "Grant Mason Project Viewer" "projects/${z_depot_project_id}" \
    "roles/viewer" "serviceAccount:${z_mason_sa_email}" "mason-viewer"

  # Logs writer (for Cloud Build logs to Cloud Logging)
  rbgi_add_project_iam_role "${z_token}" "Grant Mason Logs Writer" "projects/${z_depot_project_id}" \
    "roles/logging.logWriter" "serviceAccount:${z_mason_sa_email}" "mason-logs-writer"

  buc_step 'Provision Cloud Build service agent'
  local z_cb_service_agent
  z_cb_service_agent=$(rbgu_provision_service_agent "cloudbuild" "${z_depot_project_id}" "${z_token}") \
    || buc_die "Failed to provision Cloud Build service agent"

  buc_step 'Enable Cloud Build service agent to impersonate Mason'
  rbgi_add_sa_iam_role "${z_token}" "${z_mason_sa_email}" "${z_cb_service_agent}" "roles/iam.serviceAccountTokenCreator"

  buc_step 'Store GitHub PAT in Secret Manager'
  local -r z_secret_parent="projects/${z_depot_project_id}"
  local -r z_secret_create_url="${RBGC_API_ROOT_SECRETMANAGER}${RBGC_SECRETMANAGER_V1}/${z_secret_parent}/secrets?secretId=${RBGC_CBV2_PAT_SECRET_NAME}"
  local -r z_secret_create_body="${BURD_TEMP_DIR}/rbgp_secret_create.json"

  jq -n '{"replication": {"automatic": {}}}' > "${z_secret_create_body}" \
    || buc_die "Failed to build secret creation body"

  rbgu_http_json "POST" "${z_secret_create_url}" "${z_token}" "depot_secret_create" "${z_secret_create_body}"
  rbgu_http_require_ok "Create Secret Manager secret" "depot_secret_create"

  # Add secret version with PAT data
  local z_pat_b64
  z_pat_b64=$(printf '%s' "${z_github_pat}" | base64) || buc_die "Failed to base64-encode PAT"
  local -r z_secret_version_url="${RBGC_API_ROOT_SECRETMANAGER}${RBGC_SECRETMANAGER_V1}/${z_secret_parent}/secrets/${RBGC_CBV2_PAT_SECRET_NAME}:addVersion"
  local -r z_secret_version_body="${BURD_TEMP_DIR}/rbgp_secret_version.json"

  jq -n --arg data "${z_pat_b64}" '{"payload": {"data": $data}}' > "${z_secret_version_body}" \
    || buc_die "Failed to build secret version body"

  rbgu_http_json "POST" "${z_secret_version_url}" "${z_token}" "depot_secret_version" "${z_secret_version_body}"
  rbgu_http_require_ok "Add secret version" "depot_secret_version"

  local z_secret_version
  z_secret_version=$(rbgu_json_field_capture "depot_secret_version" '.name') \
    || buc_die "Failed to get secret version name"
  buc_log_args "PAT stored as ${z_secret_version}"

  buc_step 'Grant Cloud Build service agent Secret Manager access'
  local -r z_secret_resource="${z_secret_parent}/secrets/${RBGC_CBV2_PAT_SECRET_NAME}"
  local -r z_secret_iam_get_url="${RBGC_API_ROOT_SECRETMANAGER}${RBGC_SECRETMANAGER_V1}/${z_secret_resource}:getIamPolicy"
  rbgu_http_json "POST" "${z_secret_iam_get_url}" "${z_token}" "depot_secret_get_iam"
  local z_secret_iam_get_code
  z_secret_iam_get_code=$(rbgu_http_code_capture "depot_secret_get_iam") || z_secret_iam_get_code=""
  if test "${z_secret_iam_get_code}" != "200"; then
    rbgu_write_vanilla_json "depot_secret_get_iam"
  fi

  local z_secret_updated_policy
  z_secret_updated_policy=$(rbgu_jq_add_member_to_role_capture "depot_secret_get_iam" \
    "roles/secretmanager.secretAccessor" "serviceAccount:${z_cb_service_agent}" "") \
    || buc_die "Failed to build updated secret IAM policy"

  local -r z_secret_set_iam_url="${RBGC_API_ROOT_SECRETMANAGER}${RBGC_SECRETMANAGER_V1}/${z_secret_resource}:setIamPolicy"
  local -r z_secret_set_iam_body="${BURD_TEMP_DIR}/rbgp_secret_set_iam.json"
  printf '{"policy":%s}\n' "${z_secret_updated_policy}" > "${z_secret_set_iam_body}" \
    || buc_die "Failed to write secret IAM policy body"

  rbgu_http_json "POST" "${z_secret_set_iam_url}" "${z_token}" "depot_secret_set_iam" "${z_secret_set_iam_body}"
  rbgu_http_require_ok "Grant CB service agent secret access" "depot_secret_set_iam"

  buc_step 'Grant Cloud Build service agent connection admin'
  rbgi_add_project_iam_role "${z_token}" "Grant CB Connection Admin" "projects/${z_depot_project_id}" \
    "roles/cloudbuild.connectionAdmin" "serviceAccount:${z_cb_service_agent}" "cb-conn-admin"

  buc_step 'Create CB v2 connection'
  local -r z_cbv2_connection_name="rbw-${z_depot_name}-github"
  local -r z_cbv2_parent="projects/${z_depot_project_id}/locations/${z_region}"
  local -r z_cbv2_create_url="${RBGC_API_ROOT_CLOUDBUILD_V2}${RBGC_CLOUDBUILD_V2}/${z_cbv2_parent}/connections?connectionId=${z_cbv2_connection_name}"
  local -r z_cbv2_create_body="${BURD_TEMP_DIR}/rbgp_cbv2_connection.json"

  jq -n \
    --arg secretVersion "${z_secret_version}" \
    --arg installationId "${z_github_app_installation_id}" \
    '{
      githubConfig: {
        authorizerCredential: {
          oauthTokenSecretVersion: $secretVersion
        },
        appInstallationId: $installationId
      }
    }' > "${z_cbv2_create_body}" \
    || buc_die "Failed to build CB v2 connection body"

  rbgu_http_json_lro_ok \
    "Create CB v2 connection" \
    "${z_token}" \
    "${z_cbv2_create_url}" \
    "depot_cbv2_create" \
    "${z_cbv2_create_body}" \
    ".name" \
    "${RBGC_API_ROOT_CLOUDBUILD_V2}${RBGC_CLOUDBUILD_V2}" \
    "${RBGC_OP_PREFIX_GLOBAL}" \
    "${RBGC_EVENTUAL_CONSISTENCY_SEC}" \
    "${RBGC_MAX_CONSISTENCY_SEC}"

  # Poll connection until COMPLETE
  buc_step 'Verify CB v2 connection is active'
  local -r z_cbv2_get_url="${RBGC_API_ROOT_CLOUDBUILD_V2}${RBGC_CLOUDBUILD_V2}/${z_cbv2_parent}/connections/${z_cbv2_connection_name}"
  local z_cbv2_poll_count=0
  local -r z_cbv2_max_polls=$(( RBGC_MAX_CONSISTENCY_SEC / RBGC_EVENTUAL_CONSISTENCY_SEC ))
  local z_cbv2_stage=""

  while true; do
    rbgu_http_json "GET" "${z_cbv2_get_url}" "${z_token}" "depot_cbv2_poll"
    rbgu_http_require_ok "Poll CB v2 connection" "depot_cbv2_poll"

    z_cbv2_stage=$(rbgu_json_field_capture "depot_cbv2_poll" '.installationState.stage // "UNKNOWN"') \
      || buc_die "Failed to parse CB v2 connection stage"

    if [ "${z_cbv2_stage}" = "COMPLETE" ]; then
      buc_log_args "CB v2 connection active"
      break
    fi

    z_cbv2_poll_count=$((z_cbv2_poll_count + 1))
    if [ "${z_cbv2_poll_count}" -ge "${z_cbv2_max_polls}" ]; then
      buc_die "CB v2 connection failed — check PAT scopes and GitHub App installation (stage: ${z_cbv2_stage})"
    fi

    buc_log_args "CB v2 connection stage: ${z_cbv2_stage} (poll ${z_cbv2_poll_count}/${z_cbv2_max_polls})"
    sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
  done

  buc_step 'Create CB v2 repository'
  local -r z_cbv2_repo_url="${RBGC_API_ROOT_CLOUDBUILD_V2}${RBGC_CLOUDBUILD_V2}/${z_cbv2_parent}/connections/${z_cbv2_connection_name}/repositories?repositoryId=${RBGC_CBV2_REPOSITORY_ID}"
  local -r z_cbv2_repo_body="${BURD_TEMP_DIR}/rbgp_cbv2_repository.json"

  jq -n --arg remoteUri "${z_rubric_repo_url}" '{"remoteUri": $remoteUri}' > "${z_cbv2_repo_body}" \
    || buc_die "Failed to build CB v2 repository body"

  rbgu_http_json_lro_ok \
    "Create CB v2 repository" \
    "${z_token}" \
    "${z_cbv2_repo_url}" \
    "depot_cbv2_repo_create" \
    "${z_cbv2_repo_body}" \
    ".name" \
    "${RBGC_API_ROOT_CLOUDBUILD_V2}${RBGC_CLOUDBUILD_V2}" \
    "${RBGC_OP_PREFIX_GLOBAL}" \
    "${RBGC_EVENTUAL_CONSISTENCY_SEC}" \
    "${RBGC_MAX_CONSISTENCY_SEC}"

  buc_step 'Update depot tracking'
  zrbgp_depot_list_update || buc_die "Failed to update depot tracking after creation"

  buc_step 'Display depot configuration'
  buc_success 'Depot creation successful'
  buc_info "Mason service account: ${z_mason_sa_email}"
  buc_info "CB v2 connection '${z_cbv2_connection_name}' active with GitHub App"
  buc_info "GitHub PAT stored in Secret Manager (${RBGC_CBV2_PAT_SECRET_NAME})"
  buc_info "Update RBRR configuration:"
  buc_bare "  RBRR_DEPOT_PROJECT_ID=${z_depot_project_id}"
  buc_bare "  RBRR_GCP_REGION=${z_region}"
  buc_bare "  RBRR_GAR_REPOSITORY=${z_repository_name}"
  buc_bare "  RBRR_CBV2_CONNECTION_NAME=${z_cbv2_connection_name}"
  buc_info "Next: create Governor for this depot:"
  buc_next "rbw-Prg"
}

rbgp_depot_destroy() {
  zrbgp_sentinel

  local z_depot_project_id="${1:-}"

  buc_doc_brief "DANGER: Permanently destroy an entire depot infrastructure"
  buc_doc_param "depot_project_id" "The depot project ID to destroy"
  buc_doc_shown || return 0

  buc_step 'Safety confirmation required'
  test -n "${z_depot_project_id}" || buc_die "Depot project ID required as first argument (current RBRR: ${RBRR_DEPOT_PROJECT_ID})"

  buc_require "DANGER: Permanently destroy depot ${z_depot_project_id} and ALL resources" "${z_depot_project_id}"

  buc_step 'Authenticate as Payor'
  test -n "${RBRP_PAYOR_PROJECT_ID:-}" || buc_die "RBRP_PAYOR_PROJECT_ID is not set"
  test -n "${RBRP_OAUTH_CLIENT_ID:-}" || buc_die "RBRP_OAUTH_CLIENT_ID is not set"
  
  local z_token
  z_token=$(zrbgp_authenticate_capture) || buc_die "Failed to authenticate as Payor via OAuth"

  buc_step 'Validate target depot'
  test -n "${z_depot_project_id}" || buc_die "Depot project ID required as first argument"
  
  local z_project_info_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V3}/projects/${z_depot_project_id}"
  rbgu_http_json "GET" "${z_project_info_url}" "${z_token}" "depot_destroy_validate"
  rbgu_http_require_ok "Validate depot project" "depot_destroy_validate"
  
  local z_lifecycle_state
  z_lifecycle_state=$(rbgu_json_field_capture "depot_destroy_validate" '.state // "UNKNOWN"') || buc_die "Failed to parse project state"
  
  if [ "${z_lifecycle_state}" != "ACTIVE" ]; then
    if [ "${z_lifecycle_state}" = "DELETE_REQUESTED" ]; then
      buc_die "Project already marked for deletion"
    else
      buc_die "Project state is ${z_lifecycle_state} - can only destroy ACTIVE projects"
    fi
  fi

  buc_step 'Check for and remove liens'
  local z_liens_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens?parent=projects%2F${z_depot_project_id}"
  rbgu_http_json "GET" "${z_liens_url}" "${z_token}" "depot_destroy_liens_list"
  rbgu_http_require_ok "List liens" "depot_destroy_liens_list"
  
  local z_lien_count
  z_lien_count=$(rbgu_json_field_capture "depot_destroy_liens_list" '.liens // [] | length') || buc_die "Failed to parse liens response"
  
  if [ "${z_lien_count}" -gt 0 ]; then
    buc_log_args "Found ${z_lien_count} lien(s) - removing them"
    local z_lien_names
    z_lien_names=$(rbgu_json_field_capture "depot_destroy_liens_list" '.liens[].name' | tr '\n' ' ') || buc_die "Failed to extract lien names"
    
    for z_lien_name in ${z_lien_names}; do
      if [ -n "${z_lien_name}" ]; then
        buc_log_args "Removing lien: ${z_lien_name}"
        local z_delete_lien_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens/${z_lien_name}"
        rbgu_http_json "DELETE" "${z_delete_lien_url}" "${z_token}" "depot_destroy_lien_delete"
        rbgu_http_require_ok "Delete lien" "depot_destroy_lien_delete"
      fi
    done
  fi

  buc_step 'Unlink billing account (releases quota immediately)'
  local z_billing_unlink_body="${BURD_TEMP_DIR}/rbgp_billing_unlink.json"
  echo '{"billingAccountName":""}' > "${z_billing_unlink_body}" || buc_die "Failed to build billing unlink body"

  local z_billing_unlink_url="${RBGC_API_ROOT_CLOUDBILLING}${RBGC_CLOUDBILLING_V1}/projects/${z_depot_project_id}/billingInfo"
  rbgu_http_json "PUT" "${z_billing_unlink_url}" "${z_token}" "depot_destroy_billing_unlink" "${z_billing_unlink_body}"

  local z_billing_unlink_code
  z_billing_unlink_code=$(rbgu_http_code_capture "depot_destroy_billing_unlink") || z_billing_unlink_code=""
  if [ "${z_billing_unlink_code}" = "200" ]; then
    buc_log_args "Billing account unlinked - quota released"
  else
    buc_warn "Could not unlink billing (HTTP ${z_billing_unlink_code}) - proceeding with deletion anyway"
  fi

  buc_step 'Delete CB v2 repository (if exists)'
  local -r z_cbv2_conn="${RBRR_CBV2_CONNECTION_NAME:-}"
  if test -n "${z_cbv2_conn}"; then
    local -r z_cbv2_repo_res="projects/${z_depot_project_id}/locations/${RBRR_GCP_REGION}/connections/${z_cbv2_conn}/repositories/${RBGC_CBV2_REPOSITORY_ID}"
    local -r z_cbv2_repo_del_url="${RBGC_API_ROOT_CLOUDBUILD_V2}${RBGC_CLOUDBUILD_V2}/${z_cbv2_repo_res}"
    rbgu_http_json "DELETE" "${z_cbv2_repo_del_url}" "${z_token}" "depot_destroy_cbv2_repo"
    local z_cbv2_repo_del_code
    z_cbv2_repo_del_code=$(rbgu_http_code_capture "depot_destroy_cbv2_repo") || z_cbv2_repo_del_code=""
    case "${z_cbv2_repo_del_code}" in
      200|204|404) buc_log_args "CB v2 repository cleanup: HTTP ${z_cbv2_repo_del_code}" ;;
      *) buc_warn "CB v2 repository cleanup failed: HTTP ${z_cbv2_repo_del_code} — proceeding" ;;
    esac
  fi

  buc_step 'Delete CB v2 connection (if exists)'
  if test -n "${z_cbv2_conn}"; then
    local -r z_cbv2_conn_res="projects/${z_depot_project_id}/locations/${RBRR_GCP_REGION}/connections/${z_cbv2_conn}"
    local -r z_cbv2_conn_del_url="${RBGC_API_ROOT_CLOUDBUILD_V2}${RBGC_CLOUDBUILD_V2}/${z_cbv2_conn_res}"
    rbgu_http_json "DELETE" "${z_cbv2_conn_del_url}" "${z_token}" "depot_destroy_cbv2_conn"
    local z_cbv2_conn_del_code
    z_cbv2_conn_del_code=$(rbgu_http_code_capture "depot_destroy_cbv2_conn") || z_cbv2_conn_del_code=""
    case "${z_cbv2_conn_del_code}" in
      200|204|404) buc_log_args "CB v2 connection cleanup: HTTP ${z_cbv2_conn_del_code}" ;;
      *) buc_warn "CB v2 connection cleanup failed: HTTP ${z_cbv2_conn_del_code} — proceeding" ;;
    esac
  fi

  buc_step 'Delete Secret Manager secret (if exists)'
  local -r z_secret_del_url="${RBGC_API_ROOT_SECRETMANAGER}${RBGC_SECRETMANAGER_V1}/projects/${z_depot_project_id}/secrets/${RBGC_CBV2_PAT_SECRET_NAME}"
  rbgu_http_json "DELETE" "${z_secret_del_url}" "${z_token}" "depot_destroy_secret"
  local z_secret_del_code
  z_secret_del_code=$(rbgu_http_code_capture "depot_destroy_secret") || z_secret_del_code=""
  case "${z_secret_del_code}" in
    200|204|404) buc_log_args "Secret Manager cleanup: HTTP ${z_secret_del_code}" ;;
    *) buc_warn "Secret Manager cleanup failed: HTTP ${z_secret_del_code} — proceeding" ;;
  esac

  buc_step 'Initiate depot deletion'
  local z_delete_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V3}/projects/${z_depot_project_id}"
  rbgu_http_json "DELETE" "${z_delete_url}" "${z_token}" "depot_destroy_delete"
  
  local z_delete_response
  z_delete_response=$(rbgu_http_code_capture "depot_destroy_delete") || buc_die "Failed to get deletion response code"
  
  if [ "${z_delete_response}" = "200" ] || [ "${z_delete_response}" = "204" ]; then
    buc_log_args "Project deletion initiated successfully"
  else
    local z_error_msg
    z_error_msg=$(rbgu_json_field_capture "depot_destroy_delete" '.error.message // "Unknown error"') || z_error_msg="HTTP ${z_delete_response}"
    buc_die "Failed to initiate project deletion: ${z_error_msg}"
  fi

  buc_step 'Verify deletion state transition'
  local z_max_attempts=12
  local z_attempt=1
  local z_final_state=""
  
  while [ "${z_attempt}" -le "${z_max_attempts}" ]; do
    sleep 5
    buc_log_args "Checking deletion state (attempt ${z_attempt}/${z_max_attempts})"
    
    rbgu_http_json "GET" "${z_project_info_url}" "${z_token}" "depot_destroy_state_check"

    local z_state_check_code
    z_state_check_code=$(rbgu_http_code_capture "depot_destroy_state_check") || z_state_check_code=""
    if [ "${z_state_check_code}" = "200" ]; then
      z_final_state=$(rbgu_json_field_capture "depot_destroy_state_check" '.state // "UNKNOWN"') || z_final_state="UNKNOWN"

      if [ "${z_final_state}" = "DELETE_REQUESTED" ]; then
        break
      fi
    fi
    
    z_attempt=$((z_attempt + 1))
  done
  
  if [ "${z_final_state}" != "DELETE_REQUESTED" ]; then
    buc_die "Failed to verify deletion state transition. Current state: ${z_final_state}"
  fi

  buc_step 'Update depot tracking'
  zrbgp_depot_list_update || buc_log_args "Warning: Failed to update depot tracking after deletion"

  # Success
  buc_success "Depot ${z_depot_project_id} successfully marked for deletion"
  buc_info "Project Status: DELETE_REQUESTED"
  buc_info "Billing: Unlinked (quota released immediately)"
  buc_info "Grace period: Up to 30 days before permanent removal"
  buc_info "All infrastructure (Mason SA, repository, bucket) will be automatically removed"
}

rbgp_depot_list() {
  zrbgp_sentinel

  buc_doc_brief "List all depot instances and their status"
  buc_doc_shown || return 0

  buc_step 'Authenticate as Payor'
  test -n "${RBRP_PAYOR_PROJECT_ID:-}" || buc_die "RBRP_PAYOR_PROJECT_ID is not set"
  test -n "${RBRP_OAUTH_CLIENT_ID:-}"  || buc_die "RBRP_OAUTH_CLIENT_ID is not set"
  
  local z_token
  z_token=$(zrbgp_authenticate_capture) || buc_die "Failed to authenticate as Payor via OAuth"

  buc_step 'Query depot projects'
  local z_filter="projectId:${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_DEPOT}-* AND lifecycleState:ACTIVE"
  local z_list_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/projects?filter=${z_filter// /%20}"
  rbgu_http_json "GET" "${z_list_url}" "${z_token}" "depot_list_projects"
  rbgu_http_require_ok "List depot projects" "depot_list_projects"
  
  local z_project_count
  z_project_count=$(rbgu_json_field_capture "depot_list_projects" '.projects // [] | length') || z_project_count=0

  if [ "${z_project_count}" -eq 0 ]; then
    buc_info "No active depot projects found"
    return 0
  fi

  buc_step "Validating ${z_project_count} depot(s)"
  
  local z_depot_index=0
  local z_complete_count=0
  local z_broken_count=0
  
  buc_info ""
  buc_info "=== DEPOT SUMMARY ==="
  
  while [ "${z_depot_index}" -lt "${z_project_count}" ]; do
    local z_project_id
    z_project_id=$(rbgu_json_field_capture "depot_list_projects" ".projects[${z_depot_index}].projectId") || continue
    
    local z_display_name  
    z_display_name=$(rbgu_json_field_capture "depot_list_projects" ".projects[${z_depot_index}].displayName") || z_display_name="N/A"
    
    # Extract depot name and timestamp from project ID pattern rbwg-d-NAME-TIMESTAMP
    # Using bash builtins per BCG
    local z_depot_name=""
    local z_depot_timestamp=""
    if printf '%s' "${z_project_id}" | grep -qE "${RBGC_GLOBAL_DEPOT_REGEX}"; then
      local z_without_prefix="${z_project_id#${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_DEPOT}-}"
      local z_len=${#z_without_prefix}
      local z_suffix_len=$((1 + RBGC_GLOBAL_TIMESTAMP_LEN))
      z_depot_name="${z_without_prefix:0:$((z_len - z_suffix_len))}"
      z_depot_timestamp="${z_project_id:$((${#z_project_id} - RBGC_GLOBAL_TIMESTAMP_LEN))}"
    fi
    
    # Check depot components
    local z_status="CHECKING"
    local z_region="unknown"
    
    # Try to detect region and validate components
    local z_mason_expected="${RBGC_MASON_PREFIX}-${z_depot_name}"
    local z_repo_expected="rbw-${z_depot_name}-repository"
    local z_bucket_expected="${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_BUCKET}-${z_depot_name}-${z_depot_timestamp}"
    
    # Quick validation - check if Mason service account exists
    local z_mason_url="${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/${z_project_id}/serviceAccounts/${z_mason_expected}@${z_project_id}.iam.gserviceaccount.com"
    rbgu_http_json "GET" "${z_mason_url}" "${z_token}" "depot_list_mason_${z_depot_index}" || true

    local z_mason_code
    z_mason_code=$(rbgu_http_code_capture "depot_list_mason_${z_depot_index}") || z_mason_code=""
    if [ "${z_mason_code}" = "200" ]; then
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
  
  buc_info ""
  buc_info "=== SUMMARY ==="
  buc_info "Total depots: ${z_project_count}"
  buc_info "Complete: ${z_complete_count}"
  buc_info "Broken: ${z_broken_count}"
  
  if [ "${z_broken_count}" -gt 0 ]; then
    buc_info ""
    buc_info "Note: BROKEN depots may have missing resources (Mason SA, repository, or bucket)"
    buc_info "Consider investigating broken depots or destroying them if no longer needed"
  fi
}

rbgp_payor_oauth_refresh() {
  zrbgp_sentinel

  buc_doc_brief "Refresh expired OAuth credentials following RBAGS manual procedure"
  buc_doc_lines "Use this when OAuth tokens expire after 6 months or are compromised"
  buc_doc_lines "Requires downloading new OAuth JSON from Google Cloud Console"
  buc_doc_shown || return 0

  buc_step 'Display OAuth refresh procedure'
  buc_info ""
  buc_info "=== Manual Payor OAuth Refresh Procedure ==="
  buc_info ""
  buc_info "OAuth credentials need to be refreshed. Follow these steps:"
  buc_info ""
  buc_info "1. Navigate to APIs & Services > Credentials in Payor Project"
  buc_info "   Console URL: https://console.cloud.google.com/apis/credentials?project=${RBRP_PAYOR_PROJECT_ID}"
  buc_info ""
  buc_info "2. Find existing 'Recipe Bottle Payor' OAuth client"
  buc_info ""  
  buc_info "3. Download new JSON credentials:"
  buc_info "   - Click the download icon next to the OAuth client"
  buc_info "   - Or regenerate client secret if compromised"
  buc_info ""
  buc_info "4. Save as timestamped file:"
  buc_info "   - Example: payor-oauth-$(date +%Y%m%d).json"
  buc_info ""
  buc_info "5. Run installation command with new JSON:"
  buc_info "   rbgp_payor_install /path/to/payor-oauth-[timestamp].json"
  buc_info ""
  buc_info "This will regenerate OAuth credentials and update RBRO file."
  buc_info ""
  
  buc_success "OAuth refresh procedure displayed"
  buc_info "Note: OAuth refresh tokens expire after 6 months of non-use in testing mode"
  buc_info "Any successful payor operation resets the 6-month timer"
}

rbgp_governor_reset() {
  zrbgp_sentinel

  local z_depot_project_id="${RBRR_DEPOT_PROJECT_ID}"

  buc_doc_brief "Create or replace Governor service account in a depot"
  buc_doc_lines "This operation is idempotent: existing governor-* SAs are deleted before creating a new one"
  buc_doc_lines "Uses RBRR_DEPOT_PROJECT_ID from regime configuration"
  buc_doc_shown || return 0

  buc_step 'Validate input parameters'
  test -n "${z_depot_project_id}" || buc_die "RBRR_DEPOT_PROJECT_ID is not set in regime configuration"

  if ! printf '%s' "${z_depot_project_id}" | grep -qE "${RBGC_GLOBAL_DEPOT_REGEX}"; then
    buc_die "Depot project ID must match pattern ${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_DEPOT}-{name}-{timestamp}"
  fi

  buc_step 'Authenticate as Payor'
  test -n "${RBRP_PAYOR_PROJECT_ID:-}" || buc_die "RBRP_PAYOR_PROJECT_ID is not set"
  test -n "${RBRP_OAUTH_CLIENT_ID:-}" || buc_die "RBRP_OAUTH_CLIENT_ID is not set"

  local z_token
  z_token=$(zrbgp_authenticate_capture) || buc_die "Failed to authenticate as Payor via OAuth"

  buc_step 'Validate depot project exists and is active'
  local z_project_info_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V3}/projects/${z_depot_project_id}"
  rbgu_http_json "GET" "${z_project_info_url}" "${z_token}" "${ZRBGP_INFIX_PROJECT_INFO}"
  rbgu_http_require_ok "Validate depot project" "${ZRBGP_INFIX_PROJECT_INFO}"

  local z_lifecycle_state
  z_lifecycle_state=$(rbgu_json_field_capture "${ZRBGP_INFIX_PROJECT_INFO}" '.state') || buc_die "Failed to get project state"
  test "${z_lifecycle_state}" = "ACTIVE" || buc_die "Depot project is not ACTIVE (state: ${z_lifecycle_state})"

  test "${z_depot_project_id}" != "${RBRP_PAYOR_PROJECT_ID}" || buc_die "Cannot create Governor in Payor project"

  buc_step 'List existing service accounts in depot'
  local z_sa_list_url="${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/${z_depot_project_id}/serviceAccounts"
  rbgu_http_json "GET" "${z_sa_list_url}" "${z_token}" "${ZRBGP_INFIX_GOV_LIST_SA}"
  rbgu_http_require_ok "List service accounts" "${ZRBGP_INFIX_GOV_LIST_SA}"

  buc_step 'Find and delete existing governor-* service accounts'
  local z_deleted_count=0
  local z_governor_emails
  z_governor_emails=$(jq -r '.accounts[]? | select(.email | startswith("governor-")) | .email' \
    "${ZRBGU_PREFIX}${ZRBGP_INFIX_GOV_LIST_SA}${ZRBGU_POSTFIX_JSON}" 2>/dev/null) || z_governor_emails=""

  if test -n "${z_governor_emails}"; then
    local z_email
    while IFS= read -r z_email; do
      test -n "${z_email}" || continue
      buc_log_args "Deleting existing governor SA: ${z_email}"

      local z_delete_url="${z_sa_list_url}/${z_email}"
      rbgu_http_json "DELETE" "${z_delete_url}" "${z_token}" "${ZRBGP_INFIX_GOV_DELETE_SA}"

      local z_delete_code
      z_delete_code=$(rbgu_http_code_capture "${ZRBGP_INFIX_GOV_DELETE_SA}") || z_delete_code=""
      case "${z_delete_code}" in
        200|204) z_deleted_count=$((z_deleted_count + 1)) ;;
        404)     buc_log_args "SA already deleted: ${z_email}" ;;
        *)       buc_warn "Failed to delete SA ${z_email}: HTTP ${z_delete_code}" ;;
      esac
    done <<< "${z_governor_emails}"
  fi

  buc_info "Deleted ${z_deleted_count} existing governor service account(s)"

  buc_step 'Generate Governor timestamp and account ID'
  local z_timestamp
  z_timestamp=$(date +%Y%m%d%H%M) || buc_die "Failed to generate timestamp"
  local z_governor_account_id="${RBGC_GOVERNOR_PREFIX}-${z_timestamp}"
  local z_governor_email="${z_governor_account_id}@${z_depot_project_id}.iam.gserviceaccount.com"

  buc_log_args "Governor account ID: ${z_governor_account_id}"

  buc_step 'Create Governor service account'
  local z_create_sa_body="${BURD_TEMP_DIR}/rbgp_create_governor.json"
  jq -n \
    --arg accountId "${z_governor_account_id}" \
    --arg displayName "Governor for RB Depot" \
    '{
      accountId: $accountId,
      serviceAccount: {
        displayName: $displayName
      }
    }' > "${z_create_sa_body}" || buc_die "Failed to build Governor creation body"

  rbgu_http_json "POST" "${z_sa_list_url}" "${z_token}" "${ZRBGP_INFIX_GOV_CREATE_SA}" "${z_create_sa_body}"
  rbgu_http_require_ok "Create Governor service account" "${ZRBGP_INFIX_GOV_CREATE_SA}"

  buc_log_args "Governor service account created: ${z_governor_email}"

  buc_step 'Wait for Governor SA propagation'
  local z_verify_url="${z_sa_list_url}/${z_governor_email}"
  rbgu_poll_get_until_ok "Governor SA" "${z_verify_url}" "${z_token}" "gov_verify"
  buc_step 'Wait for cross-service propagation'
  sleep 15

  buc_step 'Grant roles/owner on depot project'
  rbgi_add_project_iam_role \
    "${z_token}" \
    "Grant Governor Owner" \
    "projects/${z_depot_project_id}" \
    "roles/owner" \
    "serviceAccount:${z_governor_email}" \
    "governor-owner"

  buc_step 'Generate service account key'
  local z_key_req="${BURD_TEMP_DIR}/rbgp_governor_key_request.json"
  printf '%s' '{"privateKeyType": "TYPE_GOOGLE_CREDENTIALS_FILE"}' > "${z_key_req}"

  local z_key_url="${z_sa_list_url}/${z_governor_email}/keys"
  rbgu_http_json "POST" "${z_key_url}" "${z_token}" "${ZRBGP_INFIX_GOV_KEY}" "${z_key_req}"
  rbgu_http_require_ok "Generate Governor key" "${ZRBGP_INFIX_GOV_KEY}"

  buc_step 'Extract and decode key data'
  local z_key_b64
  z_key_b64=$(rbgu_json_field_capture "${ZRBGP_INFIX_GOV_KEY}" '.privateKeyData') \
    || buc_die "Failed to extract privateKeyData"

  local z_key_json="${BURD_TEMP_DIR}/rbgp_governor_key.json"
  buc_log_args 'Tolerate macos base64 difference'
  if ! printf '%s' "${z_key_b64}" | base64 -d > "${z_key_json}" 2>/dev/null; then
       printf '%s' "${z_key_b64}" | base64 -D > "${z_key_json}" 2>/dev/null \
      || buc_die "Failed to decode key data"
  fi

  buc_step 'Convert JSON key to RBRA format'
  local z_rbra_file="${BURD_OUTPUT_DIR}/governor-${z_timestamp}.rbra"

  local z_client_email
  z_client_email=$(jq -r '.client_email' "${z_key_json}") || buc_die "Failed to extract client_email"
  test -n "${z_client_email}" || buc_die "Empty client_email in key JSON"

  local z_private_key
  z_private_key=$(jq -r '.private_key' "${z_key_json}") || buc_die "Failed to extract private_key"
  test -n "${z_private_key}" || buc_die "Empty private_key in key JSON"

  local z_project_id
  z_project_id=$(jq -r '.project_id' "${z_key_json}") || buc_die "Failed to extract project_id"
  test -n "${z_project_id}" || buc_die "Empty project_id in key JSON"

  buc_step 'Write RBRA file'
  {
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "${z_client_email}"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "${z_private_key}"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "${z_project_id}"
    printf 'RBRA_TOKEN_LIFETIME_SEC=1800\n'
  } > "${z_rbra_file}" || buc_die "Failed to write RBRA file ${z_rbra_file}"

  test -f "${z_rbra_file}" || buc_die "Failed to write RBRA file ${z_rbra_file}"

  rm -f "${z_key_json}"

  buc_success "Governor reset completed successfully"
  buc_info "Governor service account: ${z_governor_email}"
  buc_info "RBRA file written: ${z_rbra_file}"
  buc_info ""
  buc_info "Install the RBRA file:"
  buc_bare "        cp ${z_rbra_file} ${RBRR_GOVERNOR_RBRA_FILE}"
}

# eof

