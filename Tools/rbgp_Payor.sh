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

  test -n "${RBRR_GCP_PROJECT_ID:-}"     || bcu_die "RBRR_GCP_PROJECT_ID is not set"
  test   "${#RBRR_GCP_PROJECT_ID}" -gt 0 || bcu_die "RBRR_GCP_PROJECT_ID is empty"

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
# External Functions (rbgp_*)

rbgp_billing_attach() {
  zrbgp_sentinel

  local z_billing_account="${1:-}"

  bcu_doc_brief "Attach a billing account to the project"
  bcu_doc_param "billing_account" "Billing account ID to attach"
  bcu_doc_shown || return 0

  test -n "${z_billing_account}" || bcu_die "Billing account ID required"

  bcu_step "Attaching billing account: ${z_billing_account}"

  bcu_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_log_args 'Attach billing account'
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

rbgp_billing_detach() {
  zrbgp_sentinel

  bcu_doc_brief "Detach billing account from the project"
  bcu_doc_shown || return 0

  bcu_step "Detaching billing account from project"

  bcu_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_log_args 'Detach billing account'
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

rbgp_project_delete() {
  zrbgp_sentinel

  bcu_doc_brief "DANGER: Permanently destroy the entire GCP project. Cannot be undone after 30 days."
  bcu_doc_shown || return 0

  if [[ "${DEBUG_ONLY:-0}" != "1" ]]; then
    bcu_die "This dangerous operation requires DEBUG_ONLY=1 environment variable"
  fi

  bcu_step 'Mint admin OAuth token'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step 'Triple confirmation required'
  bcu_warn ""
  bcu_warn "========================================================================"
  bcu_warn "CRITICAL WARNING: You are about to PERMANENTLY DELETE the entire project:"
  bcu_warn "  Project: ${RBRR_GCP_PROJECT_ID}"
  bcu_warn "This will:"
  bcu_warn "  - Delete ALL resources in the project"
  bcu_warn "  - Delete ALL data permanently"
  bcu_warn "  - Break billing associations"
  bcu_warn "  - Make the project unusable immediately"
  bcu_warn "  - Cannot be undone after 30-day grace period"
  bcu_warn "========================================================================"
  bcu_warn ""

  bcu_require "Type the exact project ID to confirm deletion" "${RBRR_GCP_PROJECT_ID}"
  bcu_require "Confirm you understand this DELETES EVERYTHING in the project" "DELETE-EVERYTHING"
  bcu_require "Final confirmation - type OBLITERATE to proceed" "OBLITERATE"

  bcu_step 'Check for liens (will block deletion)'
  rbgu_http_json "GET" "${RBGC_API_CRM_LIST_LIENS}?parent=projects/${RBRR_GCP_PROJECT_ID}" "${z_token}" "${ZRBGP_INFIX_LIST_LIENS}"
  rbgu_http_require_ok "List liens" "${ZRBGP_INFIX_LIST_LIENS}"

  local z_lien_count
  z_lien_count=$(rbgu_json_field_capture "${ZRBGP_INFIX_LIST_LIENS}" '.liens // [] | length') || bcu_die "Failed to parse liens response"

  if [[ "${z_lien_count}" -gt 0 ]]; then
    bcu_step 'BLOCKED: Liens exist on project'
    bcu_warn "Project has ${z_lien_count} lien(s) that prevent deletion"
    bcu_warn "You must remove all liens first:"
    bcu_code "  gcloud resource-manager liens list --project=${RBRR_GCP_PROJECT_ID}"
    bcu_code "  gcloud resource-manager liens delete LIEN_NAME --project=${RBRR_GCP_PROJECT_ID}"
    bcu_warn "Then re-run this command."
    bcu_die "Cannot proceed with active liens"
  fi

  bcu_step 'Delete project (immediate lifecycle change to DELETE_REQUESTED)'
  rbgu_http_json "DELETE" "${RBGC_API_CRM_DELETE_PROJECT}" "${z_token}" "${ZRBGP_INFIX_PROJECT_DELETE}"
  rbgu_http_require_ok "Delete project" "${ZRBGP_INFIX_PROJECT_DELETE}"

  bcu_step 'Verify deletion state'
  rbgu_http_json "GET" "${RBGC_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGP_INFIX_PROJECT_STATE}"
  rbgu_http_require_ok "Get project state" "${ZRBGP_INFIX_PROJECT_STATE}"

  local z_lifecycle_state
  z_lifecycle_state=$(rbgu_json_field_capture "${ZRBGP_INFIX_PROJECT_STATE}" '.lifecycleState // "UNKNOWN"') || bcu_die "Failed to parse project state"

  if [[ "${z_lifecycle_state}" == "DELETE_REQUESTED" ]]; then
    bcu_success "Project successfully marked for deletion"
    bcu_step "Project Status: ${z_lifecycle_state}"
    bcu_step "Grace period: Up to 30 days"
    bcu_code "To restore (if still possible): rbgp_project_undelete"
    bcu_step "WARNING: Project is now unusable but may remain visible in listings"
  else
    bcu_die "Unexpected project state after deletion: ${z_lifecycle_state}"
  fi
}

rbgp_project_undelete() {
  zrbgp_sentinel

  bcu_doc_brief "Attempt to restore a deleted project within the 30-day grace period"
  bcu_doc_shown || return 0

  bcu_step 'Mint admin OAuth token'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step 'Check current project state'
  rbgu_http_json "GET" "${RBGC_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGP_INFIX_PROJECT_STATE}"

  if ! rbgu_http_is_ok "${ZRBGP_INFIX_PROJECT_STATE}"; then
    bcu_die "Cannot access project - it may have been permanently deleted or never existed"
  fi

  local z_lifecycle_state
  z_lifecycle_state=$(rbgu_json_field_capture "${ZRBGP_INFIX_PROJECT_STATE}" '.lifecycleState // "UNKNOWN"') || bcu_die "Failed to parse project state"

  if [[ "${z_lifecycle_state}" != "DELETE_REQUESTED" ]]; then
    bcu_die "Project state is ${z_lifecycle_state} - can only restore projects in DELETE_REQUESTED state"
  fi

  bcu_step 'Confirm restoration'
  bcu_log_args "Project Status: ${z_lifecycle_state}"
  bcu_log_args "Attempting to restore project: ${RBRR_GCP_PROJECT_ID}"
  bcu_log_args "WARNING: Restore may fail if deletion process has already started"
  bcu_require "Confirm restoration of project" "RESTORE"

  bcu_step 'Attempt project restoration'
  rbgu_http_json "POST" "${RBGC_API_CRM_UNDELETE_PROJECT}" "${z_token}" "${ZRBGP_INFIX_PROJECT_RESTORE}"

  if rbgu_http_is_ok "${ZRBGP_INFIX_PROJECT_RESTORE}"; then
    bcu_step 'Verify restoration'
    rbgu_http_json "GET" "${RBGC_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGP_INFIX_PROJECT_STATE}"
    rbgu_http_require_ok "Get restored project state" "${ZRBGP_INFIX_PROJECT_STATE}"

    z_lifecycle_state=$(rbgu_json_field_capture "${ZRBGP_INFIX_PROJECT_STATE}" '.lifecycleState // "UNKNOWN"') || bcu_die "Failed to parse restored project state"

    if [[ "${z_lifecycle_state}" == "ACTIVE" ]]; then
      bcu_success "Project successfully restored to ACTIVE state"
      bcu_log_args "Project Status: ${z_lifecycle_state}"
      bcu_log_args "Project is now usable again"
    else
      bcu_die "Restoration completed but project state is unexpected: ${z_lifecycle_state}"
    fi
  else
    local z_error_msg
    z_error_msg=$(rbgu_json_field_capture "${ZRBGP_INFIX_PROJECT_RESTORE}" '.error.message // "Unknown error"') || z_error_msg="Failed to parse error"
    bcu_die "Project restoration failed: ${z_error_msg}"
  fi
}

rbgp_liens_list() {
  zrbgp_sentinel

  bcu_doc_brief "List all liens on the project"
  bcu_doc_shown || return 0

  bcu_step "Listing liens on project: ${RBRR_GCP_PROJECT_ID}"

  bcu_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_log_args 'List liens'
  rbgu_http_json "GET" "${RBGC_API_CRM_LIST_LIENS}?parent=projects/${RBRR_GCP_PROJECT_ID}" "${z_token}" "${ZRBGP_INFIX_LIST_LIENS}"
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

rbgp_lien_delete() {
  zrbgp_sentinel

  local z_lien_name="${1:-}"

  bcu_doc_brief "Delete a specific lien from the project"
  bcu_doc_param "lien_name" "Full resource name of the lien to delete"
  bcu_doc_shown || return 0

  test -n "${z_lien_name}" || bcu_die "Lien name required"

  bcu_step "Deleting lien: ${z_lien_name}"

  bcu_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_log_args 'Delete lien'
  rbgu_http_json "DELETE" "${RBGC_API_CRM_DELETE_LIEN}/${z_lien_name}" "${z_token}" "${ZRBGP_INFIX_DELETE_LIEN}"
  rbgu_http_require_ok "Delete lien" "${ZRBGP_INFIX_DELETE_LIEN}" 404 "not found (already deleted)"

  bcu_success "Lien deleted: ${z_lien_name}"
}

rbgp_billing_account_create_manual() {
  zrbgp_sentinel

  bcu_doc_brief "Display manual procedure for creating a billing account"
  bcu_doc_shown || return 0

  bcu_step "Manual Billing Account Creation Procedure"
  bcu_info ""
  bcu_info "================================================================================================="
  bcu_info "BILLING ACCOUNT SETUP"
  bcu_info "================================================================================================="
  bcu_info ""
  bcu_info "Google Cloud requires a billing account to use most services beyond the free tier."
  bcu_info "Billing accounts must be created through the Google Cloud Console."
  bcu_info ""
  bcu_info "Steps:"
  bcu_info "1. Open Google Cloud Console: ${RBGC_CONSOLE_URL}"
  bcu_info "2. Navigate to 'Billing' in the main menu"
  bcu_info "3. Click 'CREATE ACCOUNT' or 'MANAGE BILLING ACCOUNTS'"
  bcu_info "4. Follow the setup wizard:"
  bcu_info "   - Choose account type (Individual or Business)"
  bcu_info "   - Provide payment information (credit card)"
  bcu_info "   - Verify your identity if prompted"
  bcu_info "   - Accept terms and conditions"
  bcu_info "5. Note the billing account ID (format: XXXXXX-XXXXXX-XXXXXX)"
  bcu_info ""
  bcu_info "After creating the billing account:"
  bcu_info "- Use rbgp_billing_attach <billing-account-id> to attach it to your project"
  bcu_info "- Billing account IDs can be found in the Console under 'Billing'"
  bcu_info ""
  bcu_info "IMPORTANT NOTES:"
  bcu_info "- You will be charged for resources that exceed the free tier limits"
  bcu_info "- Set up billing alerts and budgets to control costs"
  bcu_info "- Review Google Cloud pricing documentation before proceeding"
  bcu_info "- Consider enabling billing export to BigQuery for detailed cost analysis"
  bcu_info ""
  bcu_info "Free tier details: https://cloud.google.com/free"
  bcu_info "Pricing calculator: https://cloud.google.com/products/calculator"
  bcu_info "================================================================================================="
}

######################################################################
# Capture: list required services that are NOT enabled (blank = all enabled)
rbgp_required_apis_missing_capture() {
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

rbgp_get_project_number_capture() {
  zrbgp_sentinel

  local z_token
  z_token=$(rbgu_get_admin_token_capture) || return 1

  rbgu_http_json "GET" "${RBGC_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGP_INFIX_PROJECT_INFO}"
  rbgu_http_require_ok "Get project info" "${ZRBGP_INFIX_PROJECT_INFO}" || return 1

  local z_project_number
  z_project_number=$(rbgu_json_field_capture "${ZRBGP_INFIX_PROJECT_INFO}" '.projectNumber') || return 1
  test -n "${z_project_number}" || return 1

  echo "${z_project_number}"
}


rbgp_create_gcs_bucket() {
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
  rbgu_http_json "POST" "${RBGC_API_GCS_BUCKET_CREATE}" "${z_token}" \
                                  "${ZRBGP_INFIX_BUCKET_CREATE}" "${z_bucket_req}"
  z_code=$(rbgu_http_code_capture "${ZRBGP_INFIX_BUCKET_CREATE}") || bcu_die "Bad bucket creation HTTP code"
  z_err=$(rbgu_json_field_capture "${ZRBGP_INFIX_BUCKET_CREATE}" '.error.message') || z_err="HTTP ${z_code}"

  case "${z_code}" in
    200|201) bcu_info "Bucket ${z_bucket_name} created";                         return 0 ;;
    409)     bcu_die  "Bucket ${z_bucket_name} already exists (pristine-state violation)" ;;
    *)       bcu_die  "Failed to create bucket: ${z_err}"                                 ;;
  esac
}

rbgp_depot_create() {
  zrbgp_sentinel

  local z_depot_name="${1:-}"
  local z_region="${2:-}"

  bcu_doc_brief "Create new depot infrastructure following RBAGS specification"
  bcu_doc_param "depot_name" "Depot name (lowercase/numbers/hyphens, max 20 chars)"
  bcu_doc_param "region" "GCP region for depot resources"
  bcu_doc_shown || return 0

  # Step 1: Validate Input Parameters
  bcu_step 'Validate input parameters'
  test -n "${z_depot_name}" || bcu_die "Depot name required as first argument"
  test -n "${z_region}" || bcu_die "Region required as second argument"
  
  # Validate depot name format
  if ! printf '%s' "${z_depot_name}" | grep -qE '^[a-z0-9-]+$'; then
    bcu_die "Depot name must contain only lowercase letters, numbers, and hyphens"
  fi
  
  if [ "${#z_depot_name}" -gt 20 ]; then
    bcu_die "Depot name must be 20 characters or less"
  fi

  # Validate region exists in Artifact Registry locations
  bcu_log_args 'Validating region exists in Artifact Registry locations'
  local z_token
  z_token=$(rbgu_get_admin_token_capture "${RBRR_PAYOR_RBRA_FILE}") || bcu_die "Failed to get payor token for region validation"
  
  local z_locations_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/projects/${RBRP_PAYOR_PROJECT_ID}/locations"
  rbgu_http_json "GET" "${z_locations_url}" "${z_token}" "region_validation"
  rbgu_http_require_ok "Validate region" "region_validation"
  
  local z_valid_regions
  z_valid_regions=$(rbgu_json_field_capture "region_validation" '.locations[].locationId' | tr '\n' ' ') || bcu_die "Failed to parse region list"
  
  if ! printf '%s' "${z_valid_regions}" | grep -qw "${z_region}"; then
    bcu_die "Invalid region. Valid regions: ${z_valid_regions}"
  fi

  # Step 2: Authenticate as Payor  
  bcu_step 'Authenticate as Payor'
  test -n "${RBRR_PAYOR_RBRA_FILE:-}" || bcu_die "RBRR_PAYOR_RBRA_FILE is not set"
  test -f "${RBRR_PAYOR_RBRA_FILE}" || bcu_die "Payor RBRA file not found: ${RBRR_PAYOR_RBRA_FILE}"
  
  # Load RBRP configuration
  test -n "${RBRP_PAYOR_PROJECT_ID:-}" || bcu_die "RBRP_PAYOR_PROJECT_ID is not set"
  test -n "${RBRP_BILLING_ACCOUNT_ID:-}" || bcu_die "RBRP_BILLING_ACCOUNT_ID is not set"
  test -n "${RBRP_PARENT_TYPE:-}" || bcu_die "RBRP_PARENT_TYPE is not set"
  test -n "${RBRP_PARENT_ID:-}" || bcu_die "RBRP_PARENT_ID is not set"
  
  source "${RBRR_PAYOR_RBRA_FILE}" || bcu_die "Failed to source Payor RBRA credentials"
  z_token=$(rbgu_get_admin_token_capture "${RBRR_PAYOR_RBRA_FILE}") || bcu_die "Failed to get payor token"

  # Step 3: Generate Depot Project ID
  bcu_step 'Generate depot project ID'
  local z_timestamp
  z_timestamp=$(date +%Y%m%d%H%M) || bcu_die "Failed to generate timestamp"
  local z_depot_project_id="rbw-${z_depot_name}-${z_timestamp}"
  
  if [ "${#z_depot_project_id}" -gt 30 ]; then
    bcu_die "Generated project ID too long (${#z_depot_project_id} > 30): ${z_depot_project_id}"
  fi
  
  bcu_log_args "Generated depot project ID: ${z_depot_project_id}"

  # Step 4: Create Depot Project
  bcu_step 'Create depot project'
  local z_create_project_body="${BDU_TEMP_DIR}/rbgp_create_project.json"
  local z_parent_resource=""
  
  if [ "${RBRP_PARENT_TYPE}" != "none" ]; then
    z_parent_resource="${RBRP_PARENT_TYPE}s/${RBRP_PARENT_ID}"
  fi
  
  jq -n \
    --arg projectId "${z_depot_project_id}" \
    --arg displayName "RB Depot: ${z_depot_name}" \
    --arg parent "${z_parent_resource}" \
    '{
      projectId: $projectId,
      displayName: $displayName
    } + (if $parent != "" then {parent: $parent} else {} end)' > "${z_create_project_body}" || bcu_die "Failed to build project creation body"

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

  # Step 5: Link Billing Account
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

  # Step 6: Get Depot Project Number
  bcu_step 'Get depot project number'
  local z_project_info_url="${RBGC_API_ROOT_CRM}${RBGC_CRM_V3}/projects/${z_depot_project_id}"
  rbgu_http_json "GET" "${z_project_info_url}" "${z_token}" "depot_project_info"
  rbgu_http_require_ok "Get project info" "depot_project_info"
  
  local z_project_number
  z_project_number=$(rbgu_json_field_capture "depot_project_info" '.projectNumber') || bcu_die "Failed to get project number"
  test -n "${z_project_number}" || bcu_die "Project number is empty"

  # Step 7: Enable Depot Project APIs
  bcu_step 'Enable depot project APIs'
  local z_api_services="artifactregistry cloudbuild containeranalysis storage iam serviceusage"
  for z_service in ${z_api_services}; do
    bcu_log_args "Enabling API: ${z_service}"
    local z_enable_url="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}/projects/${z_depot_project_id}/services/${z_service}.googleapis.com:enable"
    rbgu_http_json "POST" "${z_enable_url}" "${z_token}" "enable_${z_service}" "${ZRBGP_EMPTY_JSON}"
    
    if rbgu_http_is_ok "enable_${z_service}"; then
      bcu_log_args "API ${z_service} enabled successfully"
    else
      local z_error_code
      z_error_code=$(rbgu_http_code_capture "enable_${z_service}") || z_error_code="unknown"
      if [ "${z_error_code}" = "400" ]; then
        bcu_log_args "API ${z_service} already enabled"
      else
        bcu_die "Failed to enable API ${z_service}: HTTP ${z_error_code}"
      fi
    fi
  done

  # Step 8: Grant Payor Permissions on Depot
  bcu_step 'Grant Payor permissions on depot'
  local z_payor_sa_email="${RBRA_CLIENT_EMAIL}"
  rbgi_add_project_iam_role "${z_token}" "Grant Payor Viewer" "projects/${z_depot_project_id}" \
    "roles/viewer" "serviceAccount:${z_payor_sa_email}" "payor-viewer"
  rbgi_add_project_iam_role "${z_token}" "Grant Payor Cloud Build Editor" "projects/${z_depot_project_id}" \
    "roles/cloudbuild.builds.editor" "serviceAccount:${z_payor_sa_email}" "payor-cb-editor"
  rbgi_add_project_iam_role "${z_token}" "Grant Payor Service Usage Consumer" "projects/${z_depot_project_id}" \
    "roles/serviceusage.serviceUsageConsumer" "serviceAccount:${z_payor_sa_email}" "payor-su-consumer"

  # Step 9: Create Build Bucket
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

  # Step 10: Create Container Repository
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

  # Step 11: Create Mason Service Account
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

  # Step 12: Configure Mason Permissions
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

  # Step 13: Enable Cloud Build service agent to impersonate Mason
  bcu_step 'Enable Cloud Build service agent to impersonate Mason'
  local z_cb_service_agent="service-${z_project_number}@gcp-sa-cloudbuild.iam.gserviceaccount.com"
  rbgi_add_sa_iam_role "${z_token}" "${z_mason_sa_email}" "${z_cb_service_agent}" "roles/iam.serviceAccountTokenCreator"

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

# eof
