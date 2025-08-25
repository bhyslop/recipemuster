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

# eof