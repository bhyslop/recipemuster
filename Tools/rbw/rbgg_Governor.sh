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
# Recipe Bottle GCP Governor - Project Orchestration


# ----------------------------------------------------------------------
# Operational Invariants (RBGG is single writer; 409 is fatal)
#
# - Single admin actor: All RBGG operations are executed by a single admin
#   identity. There are no concurrent writers in the same project.
# - Pristine-state expectation: RBGG init/creation flows assume the project
#   is pristine for the resources they manage. If a resource "already exists"
#   (HTTP 409), that's treated as state drift or prior manual activity.
# - Policy: All HTTP 409 Conflict responses are fatal (buc_die). We do not
#   treat 409 as idempotent success anywhere in RBGG.
#   If you see a 409, resolve state drift first (destroy/reset), then rerun.
# ----------------------------------------------------------------------

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGG_SOURCED:-}" || buc_die "Module rbgg multiply sourced - check sourcing hierarchy"
ZRBGG_SOURCED=1

######################################################################
# Internal Functions (zrbgg_*)

zrbgg_kindle() {
  test -z "${ZRBGG_KINDLED:-}" || buc_die "Module rbgg already kindled"

  test -n "${RBRR_DEPOT_PROJECT_ID:-}"     || buc_die "RBRR_DEPOT_PROJECT_ID is not set"
  test   "${#RBRR_DEPOT_PROJECT_ID}" -gt 0 || buc_die "RBRR_DEPOT_PROJECT_ID is empty"

  buc_log_args 'Ensure dependencies are kindled first'
  zrbgc_sentinel
  zrbgo_sentinel
  zrbgu_sentinel
  zrbgi_sentinel

  ZRBGG_PREFIX="${BURD_TEMP_DIR}/rbgg_"
  ZRBGG_EMPTY_JSON="${ZRBGG_PREFIX}empty.json"
  printf '{}' > "${ZRBGG_EMPTY_JSON}"

  # Infix values for HTTP operations
  ZRBGG_INFIX_CREATE="create"
  ZRBGG_INFIX_VERIFY="verify"
  ZRBGG_INFIX_KEY="key"
  ZRBGG_INFIX_API_IAM_ENABLE="api_iam_enable"
  ZRBGG_INFIX_API_CRM_ENABLE="api_crm_enable"
  ZRBGG_INFIX_API_ART_ENABLE="api_art_enable"
  ZRBGG_INFIX_API_BUILD_ENABLE="api_build_enable"
  ZRBGG_INFIX_CB_SA_ACCOUNT_GEN="cb_account_gen"
  ZRBGG_INFIX_CB_PRIME="cb_prime"
  ZRBGG_INFIX_API_CONTAINERANALYSIS_ENABLE="api_containeranalysis_enable"
  ZRBGG_INFIX_API_STORAGE_ENABLE="api_storage_enable"
  ZRBGG_INFIX_PROJECT_INFO="project_info"
  ZRBGG_INFIX_CREATE_REPO="create_repo"
  ZRBGG_INFIX_VERIFY_REPO="verify_repo"
  ZRBGG_INFIX_DELETE_REPO="delete_repo"
  ZRBGG_INFIX_REPO_POLICY="repo_policy"
  ZRBGG_INFIX_RPOLICY_SET="repo_policy_set"
  ZRBGG_INFIX_LIST="list"
  ZRBGG_INFIX_API_CHECK="api_checking"
  ZRBGG_INFIX_DELETE="delete"
  ZRBGG_INFIX_LIST_KEYS="list_keys"
  ZRBGG_INFIX_BUCKET_CREATE="bucket_create"
  ZRBGG_INFIX_BUCKET_DELETE="bucket_delete"
  ZRBGG_INFIX_BUCKET_LIST="bucket_list"
  ZRBGG_INFIX_OBJECT_DELETE="object_delete"
  ZRBGG_INFIX_LIST_LIENS="list_liens"
  ZRBGG_INFIX_PROJECT_DELETE="project_delete"
  ZRBGG_INFIX_PROJECT_STATE="project_state"
  ZRBGG_INFIX_PROJECT_RESTORE="project_restore"

  ZRBGG_KINDLED=1
}

zrbgg_sentinel() {
  test "${ZRBGG_KINDLED:-}" = "1" || buc_die "Module rbgg not kindled - call zrbgg_kindle first"
}

######################################################################
# Capture: list required services that are NOT enabled (blank = all enabled)
zrbgg_required_apis_missing_capture() {
  zrbgg_sentinel

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
    z_infix="${ZRBGG_INFIX_API_CHECK}_${z_service}"

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

zrbgg_create_service_account_with_key() {
  zrbgg_sentinel

  local z_account_name="$1"
  local z_display_name="$2"
  local z_description="$3"
  local z_instance="$4"

  local z_account_email="${z_account_name}@${RBGD_SA_EMAIL_FULL}"

  buc_step 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_governor_token_capture) || buc_die "Failed to get admin token"

  buc_step "Create request JSON for ${z_account_name}"
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
    }' > "${ZRBGG_PREFIX}create_request.json" || buc_die "Failed to create request JSON"

  buc_step 'Create service account via REST API'
  rbgu_http_json "POST" "${RBGD_API_SERVICE_ACCOUNTS}" "${z_token}" \
    "${ZRBGG_INFIX_CREATE}" "${ZRBGG_PREFIX}create_request.json"
  rbgu_http_require_ok "Create service account" "${ZRBGG_INFIX_CREATE}"
  rbgu_newly_created_delay                      "${ZRBGG_INFIX_CREATE}" "service account" 15
  buc_info "Service account created: ${z_account_email}"

  rbgu_http_json "GET" "${RBGD_API_SERVICE_ACCOUNTS}/${z_account_email}" \
                                   "${z_token}" "${ZRBGG_INFIX_VERIFY}"
  rbgu_http_require_ok "Verify service account" "${ZRBGG_INFIX_VERIFY}"

  buc_step 'Preflight: ensure no existing USER_MANAGED keys (manual cleanup path)'

  buc_log_args 'List keys'
  rbgu_http_json "GET"                                                        \
    "${RBGD_API_SERVICE_ACCOUNTS}/${z_account_email}${RBGC_PATH_KEYS}"        \
                                      "${z_token}" "${ZRBGG_INFIX_LIST_KEYS}"
  rbgu_http_require_ok "List service account keys" "${ZRBGG_INFIX_LIST_KEYS}"

  buc_log_args 'Count existing user-managed keys'
  local z_user_keys
  z_user_keys=$(jq -r '[.keys[]? | select(.keyType=="USER_MANAGED")] | length' \
                 "${ZRBGU_PREFIX}${ZRBGG_INFIX_LIST_KEYS}${ZRBGU_POSTFIX_JSON}") \
    || buc_die "Failed to parse service account keys"

  if test "${z_user_keys}" -gt 0; then
    buc_log_args 'Provide a console URL to delete keys manually, then rerun this command'
    local z_sa_email_enc="${z_account_email//@/%40}"
    local z_keys_url="${RBGC_CONSOLE_URL}iam-admin/serviceaccounts/details/${z_sa_email_enc}?project=${RBRR_DEPOT_PROJECT_ID}"

    buc_warn "Found ${z_user_keys} existing USER_MANAGED key(s) on ${z_account_email}."
    buc_info "Open Console, select the **Keys** tab, delete old keys, then rerun:"
    buc_info "  ${z_keys_url}"
    buc_die  "Aborting to avoid minting additional keys."
  fi

  buc_step 'Generate service account key'
  local z_key_req="${BURD_TEMP_DIR}/rbgg_key_request.json"
  printf '%s' '{"privateKeyType": "TYPE_GOOGLE_CREDENTIALS_FILE"}' > "${z_key_req}"
  rbgu_http_json "POST" "${RBGD_API_SERVICE_ACCOUNTS}/${z_account_email}${RBGC_PATH_KEYS}" \
                                         "${z_token}" "${ZRBGG_INFIX_KEY}" "${z_key_req}"
  rbgu_http_require_ok "Generate service account key" "${ZRBGG_INFIX_KEY}"

  buc_step 'Extract and decode key data'
  local z_key_b64
  z_key_b64=$(rbgu_json_field_capture "${ZRBGG_INFIX_KEY}" '.privateKeyData') \
    || buc_die "Failed to extract privateKeyData"
  local z_key_json="${BURD_TEMP_DIR}/rbgg_key_${z_instance}.json"
  buc_log_args 'Tolerate macos base64 difference'
  if ! printf '%s' "${z_key_b64}" | base64 -d > "${z_key_json}" 2>/dev/null; then
       printf '%s' "${z_key_b64}" | base64 -D > "${z_key_json}" 2>/dev/null \
      || buc_die "Failed to decode key data"
  fi

  buc_step 'Convert JSON key to RBRA format'
  local z_rbra_file="${BURD_OUTPUT_DIR}/${z_instance}.rbra"

  local z_client_email
  z_client_email=$(jq -r '.client_email' "${z_key_json}") || buc_die "Failed to extract client_email"
  test -n "${z_client_email}" || buc_die "Empty client_email in key JSON"

  local z_private_key
  z_private_key=$(jq -r '.private_key' "${z_key_json}") || buc_die "Failed to extract private_key"
  test -n "${z_private_key}" || buc_die "Empty private_key in key JSON"

  local z_project_id
  z_project_id=$(jq -r '.project_id' "${z_key_json}") || buc_die "Failed to extract project_id"
  test -n "${z_project_id}" || buc_die "Empty project_id in key JSON"

  buc_step 'Write RBRA file' "${z_rbra_file}"
  {
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "$z_client_email"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "$z_private_key"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "$z_project_id"
    printf 'RBRA_TOKEN_LIFETIME_SEC=1800\n'
  } > "${z_rbra_file}" || buc_die "Failed to write RBRA file ${z_rbra_file}"

  test -f "${z_rbra_file}" || buc_die "Failed to write RBRA file ${z_rbra_file}"

  rm -f "${z_key_json}"
  buc_info "RBRA file written: ${z_rbra_file}"
}

zrbgg_create_service_account_no_key() {
  zrbgg_sentinel

  local z_account_name="${1:-}"
  local z_display_name="${2:-}"

  test -n "${z_account_name}" || buc_die "Service account name required"
  test -n "${z_display_name}" || buc_die "Display name required"

  buc_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_governor_token_capture) || buc_die "Failed to get admin token"

  local z_account_email="${z_account_name}@${RBGD_SA_EMAIL_FULL}"

  buc_step "Create service account (no key): ${z_account_name}"
  # write body FIRST
  local z_body="${BURD_TEMP_DIR}/rbgg_sa_create_nokey.json"
  jq -n --arg account_id   "${z_account_name}" \
        --arg display_name "${z_display_name}" '
    { accountId: $account_id, serviceAccount: { displayName: $display_name } }
  ' > "${z_body}" || buc_die "Failed to build SA create body"

  # correct endpoint (no trailing slash)
  rbgu_http_json "POST" "${RBGD_API_SERVICE_ACCOUNTS}" "${z_token}" "${ZRBGG_INFIX_CREATE}" "${z_body}"
  rbgu_http_require_ok "Create service account" "${ZRBGG_INFIX_CREATE}"

  buc_log_args 'Allow IAM propagation, then verify using URL-encoded email'
  rbgu_newly_created_delay "${ZRBGG_INFIX_CREATE}" "service account" 15

  buc_log_args 'Verify service account'
  local z_account_email_enc
  z_account_email_enc=$(rbgu_urlencode_capture "${z_account_email}") || buc_die "Failed to encode SA email"
  rbgu_http_json "GET" "${RBGD_API_SERVICE_ACCOUNTS}/${z_account_email_enc}" "${z_token}" "${ZRBGG_INFIX_VERIFY}"
  rbgu_http_require_ok "Verify service account" "${ZRBGG_INFIX_VERIFY}"

  buc_success "Service account ensured (no keys): ${z_account_email}"
}

zrbgg_create_gcs_bucket() {
  zrbgg_sentinel

  local z_token="${1}"
  local z_bucket_name="${2}"

  buc_log_args 'Create bucket request JSON for '"${z_bucket_name}"
  local z_bucket_req="${BURD_TEMP_DIR}/rbgg_bucket_create_req.json"
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
  rbgu_http_json "POST" "${RBGD_API_GCS_BUCKET_CREATE}" "${z_token}" \
                                  "${ZRBGG_INFIX_BUCKET_CREATE}" "${z_bucket_req}"
  z_code=$(rbgu_http_code_capture "${ZRBGG_INFIX_BUCKET_CREATE}") || buc_die "Bad bucket creation HTTP code"
  z_err=$(rbgu_json_field_capture "${ZRBGG_INFIX_BUCKET_CREATE}" '.error.message') || z_err="HTTP ${z_code}"

  case "${z_code}" in
    200|201) buc_info "Bucket ${z_bucket_name} created";                    return 0 ;;
    409)     buc_die  "Bucket ${z_bucket_name} already exists (pristine-state violation)" ;;
    *)       buc_die  "Failed to create bucket: ${z_err}"                             ;;
  esac
}

zrbgg_list_bucket_objects_capture() {
  zrbgg_sentinel

  local z_token="${1}"
  local z_bucket_name="${2}"

  local z_list_url_base="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}/o"
  local z_page_token=""
  local z_first=1

  while :; do
    buc_log_args "Build URL with optional pageToken -> ${z_first}"
    local z_url="${z_list_url_base}"
    if test -n "${z_page_token}"; then
      buc_log_args 'pageToken must be URL-encoded'
      local z_tok_enc
      z_tok_enc=$(rbgu_urlencode_capture "${z_page_token}") || return 1
      z_url="${z_url}?pageToken=${z_tok_enc}"
    fi

    buc_log_args 'Use a unique infix per page to avoid clobbering files'
    local z_infix="${ZRBGG_INFIX_BUCKET_LIST}${z_first}"
    rbgu_http_json "GET" "${z_url}" "${z_token}" "${z_infix}"

    local z_code
    z_code=$(rbgu_http_code_capture "${z_infix}") || return 1
    test "${z_code}" = "200" || return 1

    buc_log_args 'Print names from this page (if any)'
    buc_log_args 'Next page?'
    jq -r                '.items[]?.name // empty' "${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_JSON}"  || return 1
    z_page_token=$(jq -r '.nextPageToken // empty' "${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_JSON}") || return 1

    test -n "${z_page_token}" || break
    z_first=$((z_first + 1))
  done
}

zrbgg_get_project_number_capture() {
  zrbgg_sentinel

  local z_token
  z_token=$(rbgu_get_governor_token_capture) || return 1

  rbgu_http_json "GET" "${RBGD_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGG_INFIX_PROJECT_INFO}"
  rbgu_http_require_ok "Get project info"                         "${ZRBGG_INFIX_PROJECT_INFO}" || return 1

  local z_project_number
  z_project_number=$(rbgu_json_field_capture "${ZRBGG_INFIX_PROJECT_INFO}" '.projectNumber') || return 1
  test -n "${z_project_number}" || return 1

  echo "${z_project_number}"
}

zrbgg_empty_gcs_bucket() {
  zrbgg_sentinel

  local z_token="${1}"
  local z_bucket_name="${2}"

  buc_log_args 'Get list of objects to delete'
  local z_objects
  z_objects=$(zrbgg_list_bucket_objects_capture "${z_token}" "${z_bucket_name}") || {
    buc_log_args 'No objects found or bucket not accessible'
    return 0
  }

  test -n "${z_objects}" || { buc_log_args 'Bucket is empty'; return 0; }

  buc_log_args 'Delete each object'
  local z_object=""
  local z_delete_url=""
  local z_delete_code=""
  while IFS= read -r z_object; do
    test -n "${z_object}" || continue
    buc_log_args "Deleting object: ${z_object}"

    local z_object_enc
    z_object_enc=$(rbgu_urlencode_capture "${z_object}") || z_object_enc=""
    test -n "${z_object_enc}" || { buc_warn "Failed to encode object name: ${z_object}"; continue; }
    z_delete_url="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}/o/${z_object_enc}"

    rbgu_http_json "DELETE" "${z_delete_url}" \
                              "${z_token}" "${ZRBGG_INFIX_OBJECT_DELETE}"
    z_delete_code=$(rbgu_http_code_capture "${ZRBGG_INFIX_OBJECT_DELETE}") || z_delete_code=""
    case "${z_delete_code}" in
      204|404) buc_log_args "Object ${z_object}: deleted or not found"                     ;;
      *)       buc_warn     "Object ${z_object}: Failed to delete (HTTP ${z_delete_code})" ;;
    esac
  done <<< "${z_objects}"
}

zrbgg_delete_gcs_bucket_predicate() {
  zrbgg_sentinel

  local z_token="${1}"
  local z_bucket_name="${2}"

  buc_log_args 'Empty bucket before deletion: '"${z_bucket_name}"
  zrbgg_empty_gcs_bucket "${z_token}" "${z_bucket_name}"

  buc_log_args 'Delete the bucket'
  local z_code
  local z_err
  local z_delete_url="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}"
  rbgu_http_json "DELETE" "${z_delete_url}" \
                      "${z_token}" "${ZRBGG_INFIX_BUCKET_DELETE}"
  z_code=$(rbgu_http_code_capture "${ZRBGG_INFIX_BUCKET_DELETE}") || z_code=""
  z_err=$(rbgu_json_field_capture "${ZRBGG_INFIX_BUCKET_DELETE}" '.error.message') || z_err="HTTP ${z_code}"
  case "${z_code}" in
    204) buc_info "Bucket ${z_bucket_name} deleted";                           return 0 ;;
    404) buc_warn "Bucket ${z_bucket_name} not found (already deleted)";       return 0 ;;
    409) buc_warn "Bucket ${z_bucket_name} not empty or has retention policy"; return 1 ;;
    *)   buc_warn "Bucket ${z_bucket_name} failed delete";                     return 1 ;;
  esac
}

######################################################################
# External Functions (rbgg_*)

rbgg_list_service_accounts() {
  zrbgg_sentinel

  buc_doc_brief "List all service accounts in the project"
  buc_doc_shown || return 0

  buc_step 'Listing service accounts in project: '"${RBRR_DEPOT_PROJECT_ID}"

  buc_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_governor_token_capture) || buc_die "Failed to get admin token (rc=$?)"

  buc_log_args 'List service accounts via REST API'
  rbgu_http_json "GET" "${RBGD_API_SERVICE_ACCOUNTS}" "${z_token}" "${ZRBGG_INFIX_LIST}"
  rbgu_http_require_ok "List service accounts"                     "${ZRBGG_INFIX_LIST}"

  local z_count
  z_count=$(rbgu_json_field_capture "${ZRBGG_INFIX_LIST}" '.accounts | length') \
    || buc_die "Failed to parse response"

  if test "${z_count}" = "0"; then
    buc_info "No service accounts found in project"
    return 0
  fi

  buc_step "Found ${z_count} service account(s):"

  local z_max_width
  z_max_width=$(jq -r '.accounts[].email | length' "${ZRBGU_PREFIX}${ZRBGG_INFIX_LIST}${ZRBGU_POSTFIX_JSON}" | sort -n | tail -1) \
    || buc_die "Failed to calculate max width"

  jq -r --argjson width "${z_max_width}" \
    '.accounts[] | "  " + (.email | tostring | ((" " * ($width - length)) + .)) + " - " + (.displayName // "(no display name)")' \
    "${ZRBGU_PREFIX}${ZRBGG_INFIX_LIST}${ZRBGU_POSTFIX_JSON}" || buc_die "Failed to format accounts"

  buc_success "Service account listing completed"
}

rbgg_create_retriever() {
  zrbgg_sentinel

  local z_instance="${1:-}"

  buc_doc_brief "Create Retriever service account instance"
  buc_doc_param "instance" "Instance name (required)"
  buc_doc_shown || return 0

  test -n "${z_instance}" || buc_die "Instance name required"
  zburd_sentinel
  test -d "${BURD_OUTPUT_DIR}" || buc_die "BURD_OUTPUT_DIR does not exist: ${BURD_OUTPUT_DIR}"

  local z_account_name="${RBGC_RETRIEVER_PREFIX}-${z_instance}"
  local z_account_email="${z_account_name}@${RBGD_SA_EMAIL_FULL}"

  buc_step "Creating Retriever service account: ${z_account_name}"

  zrbgg_create_service_account_with_key                                        \
    "${z_account_name}"                                                        \
    "Recipe Bottle Retriever (${z_instance})"                                  \
    "Read-only access to Google Artifact Registry - instance: ${z_instance}"   \
    "${z_instance}"

  local z_token
  z_token=$(rbgu_get_governor_token_capture) || buc_die "Failed to get admin token"

  buc_step 'Adding Artifact Registry Reader role'
  rbgi_add_project_iam_role                 \
    "${z_token}"                            \
    "Grant Artifact Registry Reader"        \
    "${RBGD_PROJECT_RESOURCE}"              \
    "${RBGC_ROLE_ARTIFACTREGISTRY_READER}"  \
    "serviceAccount:${z_account_email}"     \
    "retriever-reader"

  local z_actual_rbra_file="${BURD_OUTPUT_DIR}/${z_instance}.rbra"

  buc_step 'To install the RBRA file locally, run:'
  buc_code ""
  buc_code "    cp \"${z_actual_rbra_file}\" \"${RBRR_RETRIEVER_RBRA_FILE}\""
  buc_code ""
  buc_success "Retriever created successfully at -> ${z_actual_rbra_file}"
}

rbgg_create_director() {
  zrbgg_sentinel

  local z_instance="${1:-}"

  buc_doc_brief "Create Director service account instance"
  buc_doc_param "instance" "Instance name (required)"
  buc_doc_shown || return 0

  test -n "${z_instance}"     || buc_die "Instance name required"
  zburd_sentinel
  test -d "${BURD_OUTPUT_DIR}" || buc_die "BURD_OUTPUT_DIR does not exist: ${BURD_OUTPUT_DIR}"

  local z_account_name="${RBGC_DIRECTOR_PREFIX}-${z_instance}"
  local z_account_email="${z_account_name}@${RBGD_SA_EMAIL_FULL}"

  buc_step "Creating Director service account: ${z_account_name}"

  zrbgg_create_service_account_with_key                    \
    "${z_account_name}"                                    \
    "Recipe Bottle Director (${z_instance})"               \
    "Create/destroy container images for ${z_instance}"    \
    "${z_instance}"

  buc_step 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_governor_token_capture) || buc_die "Failed to get admin token"

  buc_step 'Get project number for Cloud Build SA'
  rbgu_http_json "GET" "${RBGD_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGG_INFIX_PROJECT_INFO}"
  rbgu_http_require_ok "Get project info"                         "${ZRBGG_INFIX_PROJECT_INFO}"

  local z_project_number
  z_project_number=$(rbgu_json_field_capture "${ZRBGG_INFIX_PROJECT_INFO}" '.projectNumber') \
    || buc_die "Failed to extract project number"

  buc_step 'Adding Cloud Build Editor role (project scope)'
  rbgi_add_project_iam_role                 \
    "${z_token}"                            \
    "Grant Cloud Build Editor"              \
    "${RBGD_PROJECT_RESOURCE}"              \
    "${RBGC_ROLE_CLOUDBUILD_BUILDS_EDITOR}" \
    "serviceAccount:${z_account_email}"     \
    "director-cb"

  rbgi_add_project_iam_role                 \
    "${z_token}"                            \
    "Grant Project Viewer"                  \
    "${RBGD_PROJECT_RESOURCE}"              \
    "roles/viewer"                          \
    "serviceAccount:${z_account_email}"     \
    "director-viewer"

  buc_step 'Grant serviceAccountUser on Mason'
  rbgi_add_sa_iam_role "${z_token}" "${RBGD_MASON_EMAIL}" "${z_account_email}" "roles/iam.serviceAccountUser"

  buc_step 'Grant Storage Object Creator on artifacts bucket (only if pre-upload used)'
  rbgi_add_bucket_iam_role "${z_token}" "${RBGD_GCS_BUCKET}" "${z_account_email}" "roles/storage.objectCreator"
  rbgi_add_bucket_iam_role "${z_token}" "${RBGD_GCS_BUCKET}" "${z_account_email}" "roles/storage.objectViewer"

  buc_step 'Grant Artifact Registry repoAdmin (for image delete/manage)'
  rbgi_add_repo_iam_role "${z_token}" "${RBGD_GAR_PROJECT_ID}" "${z_account_email}" \
    "${RBGD_GAR_LOCATION}" "${RBRR_GAR_REPOSITORY}" "roles/artifactregistry.repoAdmin"

  local z_actual_rbra_file="${BURD_OUTPUT_DIR}/${z_instance}.rbra"

  buc_step 'To install the RBRA file locally, run:'
  buc_code ""
  buc_code "    cp \"${z_actual_rbra_file}\" \"${RBRR_DIRECTOR_RBRA_FILE}\""
  buc_code ""
  buc_success "Director created successfully at -> ${z_actual_rbra_file}"
}

rbgg_delete_service_account() {
  zrbgg_sentinel

  local z_sa_email="${1:-}"

  buc_doc_brief "Delete a service account"
  buc_doc_param "email" "Email address of the service account to delete"
  buc_doc_shown || return 0

  test -n "${z_sa_email}" || buc_die "Service account email required"

  buc_step "Deleting service account: ${z_sa_email}"

  buc_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_governor_token_capture) || buc_die "Failed to get admin token"

  buc_log_args 'Delete via REST API'
  rbgu_http_json "DELETE" "${RBGD_API_SERVICE_ACCOUNTS}/${z_sa_email}" "${z_token}" \
                                                 "${ZRBGG_INFIX_DELETE}"
  rbgu_http_require_ok "Delete service account" "${ZRBGG_INFIX_DELETE}" \
    404 "not found (already deleted)"

  buc_success "Delete operation completed"
}

rbgg_destroy_project() {
  zrbgg_sentinel
  buc_doc_brief "DEPRECATED: Use rbgp_project_delete instead - moved to Payor module for billing/destructive ops"
  buc_doc_shown || return 0

  buc_warn "========================================================================"
  buc_warn "DEPRECATION NOTICE: rbgg_destroy_project is deprecated"
  buc_warn "========================================================================"
  buc_warn ""
  buc_warn "Project deletion has been moved to the Payor module which handles"
  buc_warn "all billing and destructive lifecycle operations."
  buc_warn ""
  buc_warn "Use instead:"
  buc_warn "  rbgp_project_delete - Full project deletion with proper safeguards"
  buc_warn ""
  buc_warn "The Payor module provides additional features like lien management,"
  buc_warn "billing detachment, and project restoration capabilities."
  buc_warn "========================================================================"

  buc_die "Function moved to Payor module - use rbgp_project_delete"

  if [[ "${DEBUG_ONLY:-0}" != "1" ]]; then
    buc_die "This dangerous operation requires DEBUG_ONLY=1 environment variable"
  fi

  buc_step 'Mint admin OAuth token'
  local z_token
  z_token=$(rbgu_get_governor_token_capture) || buc_die "Failed to get admin token"

  buc_step 'Triple confirmation required'
  buc_warn ""
  buc_warn "========================================================================"
  buc_warn "CRITICAL WARNING: You are about to PERMANENTLY DELETE the entire project:"
  buc_warn "  Project: ${RBRR_DEPOT_PROJECT_ID}"
  buc_warn "This will:"
  buc_warn "  - Delete ALL resources in the project"
  buc_warn "  - Delete ALL data permanently"
  buc_warn "  - Break billing associations"
  buc_warn "  - Make the project unusable immediately"
  buc_warn "  - Cannot be undone after 30-day grace period"
  buc_warn "========================================================================"
  buc_warn ""

  buc_require "Type the exact project ID to confirm deletion" "${RBRR_DEPOT_PROJECT_ID}"
  buc_require "Confirm you understand this DELETES EVERYTHING in the project" "DELETE-EVERYTHING"
  buc_require "Final confirmation - type OBLITERATE to proceed" "OBLITERATE"

  buc_step 'Check for liens (will block deletion)'
  rbgu_http_json "GET" "${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens?parent=projects/${RBRR_DEPOT_PROJECT_ID}" "${z_token}" "${ZRBGG_INFIX_LIST_LIENS}"
  rbgu_http_require_ok "List liens" "${ZRBGG_INFIX_LIST_LIENS}"

  local z_lien_count
  z_lien_count=$(rbgu_json_field_capture "${ZRBGG_INFIX_LIST_LIENS}" '.liens // [] | length') || buc_die "Failed to parse liens response"

  if [[ "${z_lien_count}" -gt 0 ]]; then
    buc_step 'BLOCKED: Liens exist on project'
    buc_warn "Project has ${z_lien_count} lien(s) that prevent deletion"
    buc_warn "You must remove all liens first:"
    buc_code "  gcloud resource-manager liens list --project=${RBRR_DEPOT_PROJECT_ID}"
    buc_code "  gcloud resource-manager liens delete LIEN_NAME --project=${RBRR_DEPOT_PROJECT_ID}"
    buc_warn "Then re-run this command."
    buc_die "Cannot proceed with active liens"
  fi

  buc_step 'Delete project (immediate lifecycle change to DELETE_REQUESTED)'
  rbgu_http_json "DELETE" "${RBGD_API_CRM_DELETE_PROJECT}" "${z_token}" "${ZRBGG_INFIX_PROJECT_DELETE}"
  rbgu_http_require_ok "Delete project" "${ZRBGG_INFIX_PROJECT_DELETE}"

  buc_step 'Verify deletion state'
  rbgu_http_json "GET" "${RBGD_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGG_INFIX_PROJECT_STATE}"
  rbgu_http_require_ok "Get project state" "${ZRBGG_INFIX_PROJECT_STATE}"

  local z_lifecycle_state
  z_lifecycle_state=$(rbgu_json_field_capture "${ZRBGG_INFIX_PROJECT_STATE}" '.lifecycleState // "UNKNOWN"') || buc_die "Failed to parse project state"

  if [[ "${z_lifecycle_state}" == "DELETE_REQUESTED" ]]; then
    buc_success "Project successfully marked for deletion"
    buc_step "Project Status: ${z_lifecycle_state}"
    buc_step "Grace period: Up to 30 days"
    buc_code "To restore (if still possible): rbgg_restore_project"
    buc_step "WARNING: Project is now unusable but may remain visible in listings"
  else
    buc_die "Unexpected project state after deletion: ${z_lifecycle_state}"
  fi
}

rbgg_restore_project() {
  zrbgg_sentinel
  buc_doc_brief "Attempt to restore a deleted project within the 30-day grace period"
  buc_doc_shown || return 0

  buc_step 'Mint admin OAuth token'
  local z_token
  z_token=$(rbgu_get_governor_token_capture) || buc_die "Failed to get admin token"

  buc_step 'Check current project state'
  rbgu_http_json "GET" "${RBGD_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGG_INFIX_PROJECT_STATE}"

  if ! rbgu_http_is_ok "${ZRBGG_INFIX_PROJECT_STATE}"; then
    buc_die "Cannot access project - it may have been permanently deleted or never existed"
  fi

  local z_lifecycle_state
  z_lifecycle_state=$(rbgu_json_field_capture "${ZRBGG_INFIX_PROJECT_STATE}" '.lifecycleState // "UNKNOWN"') || buc_die "Failed to parse project state"

  if [[ "${z_lifecycle_state}" != "DELETE_REQUESTED" ]]; then
    buc_die "Project state is ${z_lifecycle_state} - can only restore projects in DELETE_REQUESTED state"
  fi

  buc_step 'Confirm restoration'
  buc_log_args "Project Status: ${z_lifecycle_state}"
  buc_log_args "Attempting to restore project: ${RBRR_DEPOT_PROJECT_ID}"
  buc_log_args "WARNING: Restore may fail if deletion process has already started"
  buc_require "Confirm restoration of project" "RESTORE"

  buc_step 'Attempt project restoration'
  rbgu_http_json "POST" "${RBGD_API_CRM_UNDELETE_PROJECT}" "${z_token}" "${ZRBGG_INFIX_PROJECT_RESTORE}"
  if rbgu_http_is_ok                                                    "${ZRBGG_INFIX_PROJECT_RESTORE}"; then
    buc_step 'Verify restoration'
    rbgu_http_json "GET" "${RBGD_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGG_INFIX_PROJECT_STATE}"
    rbgu_http_require_ok "Get restored project state"               "${ZRBGG_INFIX_PROJECT_STATE}"

    z_lifecycle_state=$(rbgu_json_field_capture "${ZRBGG_INFIX_PROJECT_STATE}" '.lifecycleState // "UNKNOWN"') || buc_die "Failed to parse restored project state"

    if [[ "${z_lifecycle_state}" == "ACTIVE" ]]; then
      buc_success "Project successfully restored to ACTIVE state"
      buc_log_args "Project Status: ${z_lifecycle_state}"
      buc_log_args "Project is now usable again"
    else
      buc_die "Restoration completed but project state is unexpected: ${z_lifecycle_state}"
    fi
  else
    local z_error_msg
    z_error_msg=$(rbgu_json_field_capture "${ZRBGG_INFIX_PROJECT_RESTORE}" '.error.message // "Unknown error"') || z_error_msg="Failed to parse error"
    buc_die "Project restoration failed: ${z_error_msg}"
  fi
}

# eof

