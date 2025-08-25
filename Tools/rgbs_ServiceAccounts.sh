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
# Recipe Bottle GCP Service Accounts - Implementation

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRGBS_SOURCED:-}" || bcu_die "Module rgbs multiply sourced - check sourcing hierarchy"
ZRGBS_SOURCED=1

######################################################################
# Internal Functions (zrgbs_*)

zrgbs_kindle() {
  test -z "${ZRGBS_KINDLED:-}" || bcu_die "Module rgbs already kindled"

  test -n "${RBRR_GCP_PROJECT_ID:-}"     || bcu_die "RBRR_GCP_PROJECT_ID is not set"
  test   "${#RBRR_GCP_PROJECT_ID}" -gt 0 || bcu_die "RBRR_GCP_PROJECT_ID is empty"

  bcu_log_args "Ensure dependencies are kindled first"
  zrbgc_sentinel
  zrbgo_sentinel
  zrbgu_sentinel

  ZRGBS_PREFIX="${BDU_TEMP_DIR}/rgbs_"
  ZRGBS_EMPTY_JSON="${ZRGBS_PREFIX}empty.json"
  printf '{}' > "${ZRGBS_EMPTY_JSON}"

  # Infix values for HTTP operations
  ZRGBS_INFIX_CREATE="create"
  ZRGBS_INFIX_VERIFY="verify"
  ZRGBS_INFIX_KEY="key"
  ZRGBS_INFIX_LIST="list"
  ZRGBS_INFIX_LIST_KEYS="list_keys"
  ZRGBS_INFIX_DELETE="delete"

  ZRGBS_KINDLED=1
}

zrgbs_sentinel() {
  test "${ZRGBS_KINDLED:-}" = "1" || bcu_die "Module rgbs not kindled - call zrgbs_kindle first"
}

zrgbs_create_service_account_with_key() {
  zrgbs_sentinel

  local z_account_name="$1"
  local z_display_name="$2"
  local z_description="$3"
  local z_instance="$4"

  local z_account_email="${z_account_name}@${RBGC_SA_EMAIL_FULL}"

  bcu_step 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

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
    }' > "${ZRGBS_PREFIX}create_request.json" || bcu_die "Failed to create request JSON"

  bcu_step 'Create service account via REST API'
  rbgu_http_json "POST" "${RBGC_API_SERVICE_ACCOUNTS}" "${z_token}" \
    "${ZRGBS_INFIX_CREATE}" "${ZRGBS_PREFIX}create_request.json"
  rbgu_http_require_ok "Create service account" "${ZRGBS_INFIX_CREATE}"
  rbgu_newly_created_delay                      "${ZRGBS_INFIX_CREATE}" "service account" 15
  bcu_info "Service account created: ${z_account_email}"

  rbgu_http_json "GET" "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email}" \
                                   "${z_token}" "${ZRGBS_INFIX_VERIFY}"
  rbgu_http_require_ok "Verify service account" "${ZRGBS_INFIX_VERIFY}"

  bcu_step 'Preflight: ensure no existing USER_MANAGED keys (manual cleanup path)'

  bcu_log_args 'List keys'
  rbgu_http_json "GET"                                                        \
    "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email}${RBGC_PATH_KEYS}"        \
                                      "${z_token}" "${ZRGBS_INFIX_LIST_KEYS}"
  rbgu_http_require_ok "List service account keys" "${ZRGBS_INFIX_LIST_KEYS}"

  bcu_log_args 'Count existing user-managed keys'
  local z_user_keys
  z_user_keys=$(jq -r '[.keys[]? | select(.keyType=="USER_MANAGED")] | length' \
                 "${ZRBGU_PREFIX}${ZRGBS_INFIX_LIST_KEYS}${ZRBGU_POSTFIX_JSON}") \
    || bcu_die "Failed to count user-managed keys"

  if test "${z_user_keys}" -gt 0; then
    bcu_warning "Service account ${z_account_email} has ${z_user_keys} existing USER_MANAGED keys"
    bcu_warning "Manual cleanup required - delete these keys before proceeding:"
    jq -r '.keys[]? | select(.keyType=="USER_MANAGED") | .name' \
      "${ZRBGU_PREFIX}${ZRGBS_INFIX_LIST_KEYS}${ZRBGU_POSTFIX_JSON}" || true
    bcu_die "Cleanup required - cannot proceed with key generation"
  fi

  bcu_step 'Create service account key'
  local z_key_body="${ZRGBS_PREFIX}key_body.json"
  printf '{"keyAlgorithm":"KEY_ALG_RSA_2048"}\n' > "${z_key_body}"

  rbgu_http_json "POST"                                                       \
    "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email}${RBGC_PATH_KEYS}"        \
                                            "${z_token}" "${ZRGBS_INFIX_KEY}" "${z_key_body}"
  rbgu_http_require_ok "Create service account key" "${ZRGBS_INFIX_KEY}"

  bcu_step 'Extract and write service account credentials'
  local z_private_key_data
  z_private_key_data=$(rbgu_json_field_capture "${ZRGBS_INFIX_KEY}" '.privateKeyData') \
    || bcu_die "Failed to extract private key"

  echo "${z_private_key_data}" | base64 -d > "${z_instance}" \
    || bcu_die "Failed to decode and write key file"

  bcu_step 'Set restrictive permissions'
  chmod 600 "${z_instance}" || bcu_die "Failed to set permissions on key file"

  bcu_step 'Test key for basic validity'
  local z_json_fields
  z_json_fields=$(jq -r 'keys | join(",")' "${z_instance}" 2>/dev/null) || z_json_fields=""
  test -n "${z_json_fields}" || bcu_die "Key file does not contain valid JSON"
  echo "${z_json_fields}" | grep -q 'private_key\|client_email' || bcu_die "Key file missing required fields"

  bcu_success "Service account and key created: ${z_account_email}"
}

zrgbs_create_service_account_no_key() {
  zrgbs_sentinel

  local z_account_name="${1:-}"
  local z_display_name="${2:-}"

  test -n "${z_account_name}" || bcu_die "Service account name required"
  test -n "${z_display_name}" || bcu_die "Display name required"

  bcu_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  local z_account_email="${z_account_name}@${RBGC_SA_EMAIL_FULL}"

  bcu_step "Create service account (no key): ${z_account_name}"
  local z_body="${BDU_TEMP_DIR}/rgbs_sa_create_nokey.json"
  jq -n --arg account_id   "${z_account_name}" \
        --arg display_name "${z_display_name}" '
    { accountId: $account_id, serviceAccount: { displayName: $display_name } }
  ' > "${z_body}" || bcu_die "Failed to build SA create body"

  rbgu_http_json "POST" "${RBGC_API_SERVICE_ACCOUNTS}" "${z_token}" "${ZRGBS_INFIX_CREATE}" "${z_body}"
  rbgu_http_require_ok "Create service account" "${ZRGBS_INFIX_CREATE}"

  bcu_log_args 'Allow IAM propagation, then verify using URL-encoded email'
  rbgu_newly_created_delay "${ZRGBS_INFIX_CREATE}" "service account" 15

  bcu_log_args 'Verify service account'
  local z_account_email_enc
  z_account_email_enc=$(rbgu_urlencode_capture "${z_account_email}") || bcu_die "Failed to encode SA email"
  rbgu_http_json "GET" "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email_enc}" "${z_token}" "${ZRGBS_INFIX_VERIFY}"
  rbgu_http_require_ok "Verify service account" "${ZRGBS_INFIX_VERIFY}"

  bcu_success "Service account ensured (no keys): ${z_account_email}"
}

######################################################################
# External Functions (rgbs_*)

rgbs_sa_create() {
  zrgbs_sentinel

  local z_account_name="${1:-}"
  local z_display_name="${2:-}"

  bcu_doc_brief "Create a service account without keys"
  bcu_doc_param "account_name" "Service account name (without domain)"
  bcu_doc_param "display_name" "Human-readable display name"
  bcu_doc_shown || return 0

  zrgbs_create_service_account_no_key "${z_account_name}" "${z_display_name}"
}

rgbs_sa_get() {
  zrgbs_sentinel

  local z_sa_email="${1:-}"

  bcu_doc_brief "Get service account details"
  bcu_doc_param "sa_email" "Email address of the service account"
  bcu_doc_shown || return 0

  test -n "${z_sa_email}" || bcu_die "Service account email required"

  bcu_step "Getting service account: ${z_sa_email}"

  bcu_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_log_args 'Get service account via REST API'
  local z_sa_email_enc
  z_sa_email_enc=$(rbgu_urlencode_capture "${z_sa_email}") || bcu_die "Failed to encode SA email"
  rbgu_http_json "GET" "${RBGC_API_SERVICE_ACCOUNTS}/${z_sa_email_enc}" "${z_token}" "${ZRGBS_INFIX_VERIFY}"
  rbgu_http_require_ok "Get service account" "${ZRGBS_INFIX_VERIFY}" 404 "not found"

  if rbgu_http_code_capture "${ZRGBS_INFIX_VERIFY}" | grep -q "404"; then
    bcu_info "Service account not found: ${z_sa_email}"
    return 1
  fi

  bcu_success "Service account found: ${z_sa_email}"
  return 0
}

rgbs_sa_delete() {
  zrgbs_sentinel

  local z_sa_email="${1:-}"

  bcu_doc_brief "Delete a service account"
  bcu_doc_param "email" "Email address of the service account to delete"
  bcu_doc_shown || return 0

  test -n "${z_sa_email}" || bcu_die "Service account email required"

  bcu_step "Deleting service account: ${z_sa_email}"

  bcu_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_log_args 'Delete via REST API'
  rbgu_http_json "DELETE" "${RBGC_API_SERVICE_ACCOUNTS}/${z_sa_email}" "${z_token}" \
                                                 "${ZRGBS_INFIX_DELETE}"
  rbgu_http_require_ok "Delete service account" "${ZRGBS_INFIX_DELETE}" \
    404 "not found (already deleted)"

  bcu_success "Delete operation completed"
}

rgbs_sa_keys_policy_enforce() {
  zrgbs_sentinel

  local z_sa_email="${1:-}"

  bcu_doc_brief "Enforce no-keys policy on a service account"
  bcu_doc_param "sa_email" "Email address of the service account"
  bcu_doc_shown || return 0

  test -n "${z_sa_email}" || bcu_die "Service account email required"

  bcu_step "Enforcing no-keys policy for: ${z_sa_email}"

  bcu_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_log_args 'List existing keys'
  rbgu_http_json "GET"                                                        \
    "${RBGC_API_SERVICE_ACCOUNTS}/${z_sa_email}${RBGC_PATH_KEYS}"        \
                                      "${z_token}" "${ZRGBS_INFIX_LIST_KEYS}"
  rbgu_http_require_ok "List service account keys" "${ZRGBS_INFIX_LIST_KEYS}" 404 "service account not found"

  if rbgu_http_code_capture "${ZRGBS_INFIX_LIST_KEYS}" | grep -q "404"; then
    bcu_info "Service account not found: ${z_sa_email}"
    return 1
  fi

  bcu_log_args 'Check for user-managed keys'
  local z_user_keys
  z_user_keys=$(jq -r '[.keys[]? | select(.keyType=="USER_MANAGED")] | length' \
                 "${ZRBGU_PREFIX}${ZRGBS_INFIX_LIST_KEYS}${ZRBGU_POSTFIX_JSON}") \
    || bcu_die "Failed to count user-managed keys"

  if test "${z_user_keys}" -eq 0; then
    bcu_success "No user-managed keys found - policy compliant"
    return 0
  fi

  bcu_warning "Found ${z_user_keys} user-managed keys - policy violation"
  bcu_info "User-managed keys should be deleted to maintain security compliance"
  
  # List the keys for reference
  jq -r '.keys[]? | select(.keyType=="USER_MANAGED") | "  - " + .name' \
    "${ZRBGU_PREFIX}${ZRGBS_INFIX_LIST_KEYS}${ZRBGU_POSTFIX_JSON}" || true

  return 1
}

# eof