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
  ZRBGP_INFIX_API_IAM_ENABLE="api_iam_enable"
  ZRBGP_INFIX_API_CRM_ENABLE="api_crm_enable"
  ZRBGP_INFIX_API_ART_ENABLE="api_art_enable"
  ZRBGP_INFIX_API_BUILD_ENABLE="api_build_enable"
  ZRBGP_INFIX_API_CONTAINERANALYSIS_ENABLE="api_containeranalysis_enable"
  ZRBGP_INFIX_API_STORAGE_ENABLE="api_storage_enable"
  ZRBGP_INFIX_CREATE_REPO="create_repo"
  ZRBGP_INFIX_VERIFY_REPO="verify_repo"
  ZRBGP_INFIX_PROJECT_INFO="project_info"
  ZRBGP_INFIX_CB_SA_ACCOUNT_GEN="cb_account_gen"
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

# Ensure Cloud Build service agent exists and admin can trigger builds
rbgp_ensure_cloudbuild_service_agent() {
  zrbgp_sentinel

  local z_token="${1}"
  local z_project_number="${2}"

  local z_cb_service_agent="service-${z_project_number}@gcp-sa-cloudbuild.iam.gserviceaccount.com"
  local z_admin_sa_email="${RBGC_ADMIN_ROLE}@${RBGC_SA_EMAIL_FULL}"
  local z_gen_url="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1BETA1}/projects/${z_project_number}/services/cloudbuild.googleapis.com:generateServiceIdentity"

  rbgu_http_json_lro_ok                                              \
       "Generate Cloud Build service agent"                          \
       "${z_token}"                                                  \
       "${z_gen_url}"                                                \
       "${ZRBGP_INFIX_CB_SA_ACCOUNT_GEN}"                            \
       "${ZRBGP_EMPTY_JSON}"                                         \
       ".name"                                                       \
       "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1BETA1}"   \
       "${RBGC_OP_PREFIX_GLOBAL}"                                    \
       "5"                                                           \
       "60"

  bcu_step 'Grant Cloud Build Service Agent role'
  rbgi_add_project_iam_role                 \
    "${z_token}"                            \
    "Grant Cloud Build Service Agent role"  \
    "${RBGC_PROJECT_RESOURCE}"              \
    "roles/cloudbuild.serviceAgent"         \
    "serviceAccount:${z_cb_service_agent}"  \
    "cb-agent"

  bcu_step 'Grant admin Viewer for Cloud Build service visibility'
  rbgi_add_project_iam_role                 \
    "${z_token}"                            \
    "Grant admin Viewer"                    \
    "${RBGC_PROJECT_RESOURCE}"              \
    "roles/viewer"                          \
    "serviceAccount:${z_admin_sa_email}"    \
    "admin-viewer"

  bcu_info "Cloud Build service agent configured with admin permissions"
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
    200|201) bcu_info "Bucket ${z_bucket_name} created";                    return 0 ;;
    409)     bcu_die  "Bucket ${z_bucket_name} already exists (pristine-state violation)" ;;
    *)       bcu_die  "Failed to create bucket: ${z_err}"                             ;;
  esac
}

rbgp_project_create() {
  zrbgp_sentinel

  local z_json_path="${1:-}"

  bcu_doc_brief "Initialize GCP project infrastructure: enable/verify APIs, create GAR repo, and grant Cloud Build SA."
  bcu_doc_param "json_path" "Path to downloaded admin JSON key (will be converted to RBRA)"
  bcu_doc_shown || return 0

  test -n "${z_json_path}" || bcu_die "First argument must be path to downloaded JSON key file."

  local z_admin_sa_email="${RBGC_ADMIN_ROLE}@${RBGC_SA_EMAIL_FULL}"

  bcu_step 'Convert admin JSON to RBRA'
  rbgu_extract_json_to_rbra "${z_json_path}" "${RBRR_ADMIN_RBRA_FILE}" "1800"

  bcu_step 'Mint admin OAuth token'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step 'Check which required APIs need enabling'
  local z_missing=""
  z_missing=$(rbgp_required_apis_missing_capture "${z_token}") \
    || bcu_die "Failed to check API status"

  if test -n "${z_missing}"; then
    bcu_info "APIs needing enablement: ${z_missing}"

    # Invariant: API enable is gated by the preflight above.
    # Any 409 here means the preflight or our assumptions are wrong -> die.

    bcu_step 'Enable IAM API'
    rbgu_http_json_lro_ok                                       \
      "Enable IAM API"                                          \
      "${z_token}"                                              \
      "${RBGC_API_SU_ENABLE_IAM}"                               \
      "${ZRBGP_INFIX_API_IAM_ENABLE}"                           \
      "${ZRBGP_EMPTY_JSON}"                                     \
      ".name"                                                   \
      "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}"    \
      "${RBGC_OP_PREFIX_GLOBAL}"                                \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                        \
      "${RBGC_MAX_CONSISTENCY_SEC}"

    bcu_step 'Enable Cloud Resource Manager API'
    rbgu_http_json_lro_ok                                       \
      "Enable Cloud Resource Manager API"                       \
      "${z_token}"                                              \
      "${RBGC_API_SU_ENABLE_CRM}"                               \
      "${ZRBGP_INFIX_API_CRM_ENABLE}"                           \
      "${ZRBGP_EMPTY_JSON}"                                     \
      ".name"                                                   \
      "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}"    \
      "${RBGC_OP_PREFIX_GLOBAL}"                                \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                        \
      "${RBGC_MAX_CONSISTENCY_SEC}"

    bcu_step 'Enable Artifact Registry API'
    rbgu_http_json_lro_ok                                       \
      "Enable Artifact Registry API"                            \
      "${z_token}"                                              \
      "${RBGC_API_SU_ENABLE_GAR}"                               \
      "${ZRBGP_INFIX_API_ART_ENABLE}"                           \
      "${ZRBGP_EMPTY_JSON}"                                     \
      ".name"                                                   \
      "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}"    \
      "${RBGC_OP_PREFIX_GLOBAL}"                                \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                        \
      "${RBGC_MAX_CONSISTENCY_SEC}"

    bcu_step 'Enable Cloud Build API'
    rbgu_http_json_lro_ok                                       \
      "Enable Cloud Build API"                                  \
      "${z_token}"                                              \
      "${RBGC_API_SU_ENABLE_BUILD}"                             \
      "${ZRBGP_INFIX_API_BUILD_ENABLE}"                         \
      "${ZRBGP_EMPTY_JSON}"                                     \
      ".name"                                                   \
      "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}"    \
      "${RBGC_OP_PREFIX_GLOBAL}"                                \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                        \
      "${RBGC_MAX_CONSISTENCY_SEC}"

    bcu_step 'Enable Container Analysis API'
    rbgu_http_json_lro_ok                                       \
      "Enable Container Analysis API"                           \
      "${z_token}"                                              \
      "${RBGC_API_SU_ENABLE_ANALYSIS}"                          \
      "${ZRBGP_INFIX_API_CONTAINERANALYSIS_ENABLE}"             \
      "${ZRBGP_EMPTY_JSON}"                                     \
      ".name"                                                   \
      "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}"    \
      "${RBGC_OP_PREFIX_GLOBAL}"                                \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                        \
      "${RBGC_MAX_CONSISTENCY_SEC}"

    bcu_step 'Enable Cloud Storage API (build bucket deps)'
    rbgu_http_json_lro_ok                                       \
      "Enable Cloud Storage API"                                \
      "${z_token}"                                              \
      "${RBGC_API_SU_ENABLE_STORAGE}"                           \
      "${ZRBGP_INFIX_API_STORAGE_ENABLE}"                       \
      "${ZRBGP_EMPTY_JSON}"                                     \
      ".name"                                                   \
      "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}"    \
      "${RBGC_OP_PREFIX_GLOBAL}"                                \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                        \
      "${RBGC_MAX_CONSISTENCY_SEC}"
  fi

  bcu_step 'Discover Project Number'
  local z_project_number
  z_project_number=$(rbgp_get_project_number_capture) || bcu_die "Failed to get project number"

  bcu_step 'Directly create the cloudbuild service agent'
  rbgp_ensure_cloudbuild_service_agent "${z_token}" "${z_project_number}"

  bcu_step 'Grant Cloud Build invoke permissions to admin (idempotent)'
  rbgi_add_project_iam_role                      \
    "Grant Cloud Build invoke permissions"       \
    "${z_token}"                                 \
    "${RBGC_PROJECT_RESOURCE}"                   \
    "${RBGC_ROLE_CLOUDBUILD_BUILDS_EDITOR}"      \
    "serviceAccount:${z_admin_sa_email}"         \
    "admin-cb-invoke"

  rbgi_add_project_iam_role                      \
    "Grant Service Usage Consumer"               \
    "${z_token}"                                 \
    "${RBGC_PROJECT_RESOURCE}"                   \
    "roles/serviceusage.serviceUsageConsumer"    \
    "serviceAccount:${z_admin_sa_email}"         \
    "admin-su"

  bcu_step 'Create/verify Cloud Storage bucket'
  rbgp_create_gcs_bucket "${z_token}" "${RBGC_GCS_BUCKET}"

  bcu_step 'Create/verify Docker format Artifact Registry repo'
  bcu_log_args "  The repo is ${RBRR_GAR_REPOSITORY} in ${RBGC_GAR_LOCATION}"

  test -n "${RBGC_GAR_LOCATION:-}"   || bcu_die "RBGC_GAR_LOCATION is not set"
  test -n "${RBRR_GAR_REPOSITORY:-}" || bcu_die "RBRR_GAR_REPOSITORY is not set"

  local z_parent="projects/${RBRR_GCP_PROJECT_ID}${RBGC_PATH_LOCATIONS}/${RBGC_GAR_LOCATION}"
  local z_resource="${z_parent}${RBGC_PATH_REPOSITORIES}/${RBRR_GAR_REPOSITORY}"
  local z_create_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_parent}${RBGC_PATH_REPOSITORIES}?repositoryId=${RBRR_GAR_REPOSITORY}"
  local z_get_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}"
  local z_create_body="${BDU_TEMP_DIR}/rbgp_create_repo_body.json"

  jq -n '{format:"DOCKER"}' > "${z_create_body}" || bcu_die "Failed to build create-repo body"

  bcu_step 'Create DOCKER format repo'
  rbgu_http_json_lro_ok                                              \
    "Create Artifact Registry repo"                                  \
    "${z_token}"                                                     \
    "${z_create_url}"                                                \
    "${ZRBGP_INFIX_CREATE_REPO}"                                     \
    "${z_create_body}"                                               \
    ".name"                                                          \
    "${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}"   \
    "${RBGC_OP_PREFIX_GLOBAL}"                                       \
    "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                               \
    "${RBGC_MAX_CONSISTENCY_SEC}"

  bcu_step 'Verify repository exists and is DOCKER format'
  rbgu_http_json "GET" "${z_get_url}" "${z_token}" "${ZRBGP_INFIX_VERIFY_REPO}"
  rbgu_http_require_ok "Verify repository"         "${ZRBGP_INFIX_VERIFY_REPO}"
  test "$(rbgu_json_field_capture                  "${ZRBGP_INFIX_VERIFY_REPO}" '.format')" = "DOCKER" \
    || bcu_die "Repository exists but not DOCKER format"

  bcu_step 'Ensure Mason service account exists (no keys)'
  rgbs_sa_create "${RBGC_MASON_NAME}" "RBGG Mason (build executor)"

  local z_mason_sa="${RBGC_MASON_EMAIL}"
  local z_cb_service_agent="service-${z_project_number}@gcp-sa-cloudbuild.iam.gserviceaccount.com"

  bcu_step 'Allow Cloud Build service agent to impersonate Mason (TokenCreator on Mason)'
  rbgi_add_sa_iam_role "${z_mason_sa}" "${z_cb_service_agent}" "roles/iam.serviceAccountTokenCreator"

  bcu_step 'Grant Artifact Registry Admin (repo-scoped) to Mason'
  rbgi_add_repo_iam_role "${z_token}" "${z_mason_sa}" "${RBGC_GAR_LOCATION}" "${RBRR_GAR_REPOSITORY}" \
    "${RBGC_ROLE_ARTIFACTREGISTRY_ADMIN}"

  bcu_step 'Grant Storage Object Viewer on artifacts bucket to Mason'
  rbgi_add_bucket_iam_role "${z_token}" "${RBGC_GCS_BUCKET}" "${z_mason_sa}" "roles/storage.objectViewer"

  bcu_step 'Grant Project Viewer to Mason'
  rbgi_add_project_iam_role "Grant Project Viewer" "${z_token}" "${RBGC_PROJECT_RESOURCE}" \
                            "roles/viewer" "serviceAccount:${z_mason_sa}" "mason-viewer"

  bcu_info "RBRA (admin): ${RBRR_ADMIN_RBRA_FILE}"
  bcu_info "GAR: ${RBGC_GAR_LOCATION}/${RBRR_GAR_REPOSITORY} (DOCKER)"
  bcu_info "Mason SA configured with repo access"
  bcu_warn "RBRR file stashed. Consider deleting carriage JSON:"
  bcu_code ""
  bcu_code "    rm \"${z_json_path}\""
  bcu_code ""

  bcu_success 'Project creation complete'
}

# eof
