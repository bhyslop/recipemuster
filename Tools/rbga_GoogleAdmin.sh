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


# ----------------------------------------------------------------------
# Operational Invariants (RBGA is single writer; 409 is fatal)
#
# - Single admin actor: All RBGA operations are executed by a single admin
#   identity. There are no concurrent writers in the same project.
# - Pristine-state expectation: RBGA init/creation flows assume the project
#   is pristine for the resources they manage. If a resource "already exists"
#   (HTTP 409), that's treated as state drift or prior manual activity.
# - Policy: All HTTP 409 Conflict responses are fatal (bcu_die). We do not
#   treat 409 as idempotent success anywhere in RBGA.
#   If you see a 409, resolve state drift first (destroy/reset), then rerun.
# ----------------------------------------------------------------------

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

  bcu_log_args 'Ensure dependencies are kindled first'
  zrbgc_sentinel
  zrbgo_sentinel
  zrbgu_sentinel
  zrbgi_sentinel

  ZRBGA_PREFIX="${BDU_TEMP_DIR}/rbga_"
  ZRBGA_EMPTY_JSON="${ZRBGA_PREFIX}empty.json"
  printf '{}' > "${ZRBGA_EMPTY_JSON}"

  # Infix values for HTTP operations
  ZRBGA_INFIX_CREATE="create"
  ZRBGA_INFIX_VERIFY="verify"
  ZRBGA_INFIX_KEY="key"
  ZRBGA_INFIX_API_IAM_ENABLE="api_iam_enable"
  ZRBGA_INFIX_API_CRM_ENABLE="api_crm_enable"
  ZRBGA_INFIX_API_ART_ENABLE="api_art_enable"
  ZRBGA_INFIX_API_BUILD_ENABLE="api_build_enable"
  ZRBGA_INFIX_CB_SA_ACCOUNT_GEN="cb_account_gen"
  ZRBGA_INFIX_CB_PRIME="cb_prime"
  ZRBGA_INFIX_API_CONTAINERANALYSIS_ENABLE="api_containeranalysis_enable"
  ZRBGA_INFIX_API_STORAGE_ENABLE="api_storage_enable"
  ZRBGA_INFIX_PROJECT_INFO="project_info"
  ZRBGA_INFIX_CREATE_REPO="create_repo"
  ZRBGA_INFIX_CB_RUNTIME_SA_PEEK="cb_runtime_sa_peek"
  ZRBGA_INFIX_VERIFY_REPO="verify_repo"
  ZRBGA_INFIX_DELETE_REPO="delete_repo"
  ZRBGA_INFIX_REPO_POLICY="repo_policy"
  ZRBGA_INFIX_RPOLICY_SET="repo_policy_set"
  ZRBGA_INFIX_LIST="list"
  ZRBGA_INFIX_API_CHECK="api_checking"
  ZRBGA_INFIX_DELETE="delete"
  ZRBGA_INFIX_LIST_KEYS="list_keys"
  ZRBGA_INFIX_BUCKET_CREATE="bucket_create"
  ZRBGA_INFIX_BUCKET_DELETE="bucket_delete"
  ZRBGA_INFIX_BUCKET_LIST="bucket_list"
  ZRBGA_INFIX_OBJECT_DELETE="object_delete"

  ZRBGA_KINDLED=1
}

zrbga_sentinel() {
  test "${ZRBGA_KINDLED:-}" = "1" || bcu_die "Module rbga not kindled - call zrbga_kindle first"
}

######################################################################
# Capture: list required services that are NOT enabled (blank = all enabled)
zrbga_required_apis_missing_capture() {
  zrbga_sentinel

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
    z_infix="${ZRBGA_INFIX_API_CHECK}_${z_service}"

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

zrbga_create_service_account_with_key() {
  zrbga_sentinel

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
    }' > "${ZRBGA_PREFIX}create_request.json" || bcu_die "Failed to create request JSON"

  bcu_step 'Create service account via REST API'
  rbgu_http_json "POST" "${RBGC_API_SERVICE_ACCOUNTS}" "${z_token}" \
    "${ZRBGA_INFIX_CREATE}" "${ZRBGA_PREFIX}create_request.json"
  rbgu_http_require_ok "Create service account" "${ZRBGA_INFIX_CREATE}" 409 "already exists"
  rbgu_newly_created_delay                      "${ZRBGA_INFIX_CREATE}" "service account" 15
  bcu_info "Service account created: ${z_account_email}"

  rbgu_http_json "GET" "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email}" \
                                   "${z_token}" "${ZRBGA_INFIX_VERIFY}"
  rbgu_http_require_ok "Verify service account" "${ZRBGA_INFIX_VERIFY}"

  bcu_step 'Preflight: ensure no existing USER_MANAGED keys (manual cleanup path)'

  bcu_log_args 'List keys'
  rbgu_http_json "GET"                                                        \
    "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email}${RBGC_PATH_KEYS}"        \
                                      "${z_token}" "${ZRBGA_INFIX_LIST_KEYS}"
  rbgu_http_require_ok "List service account keys" "${ZRBGA_INFIX_LIST_KEYS}"

  bcu_log_args 'Count existing user-managed keys'
  local z_user_keys
  z_user_keys=$(jq -r '[.keys[]? | select(.keyType=="USER_MANAGED")] | length' \
                 "${ZRBGU_PREFIX}${ZRBGA_INFIX_LIST_KEYS}${ZRBGU_POSTFIX_JSON}") \
    || bcu_die "Failed to parse service account keys"

  if test "${z_user_keys}" -gt 0; then
    bcu_log_args 'Provide a console URL to delete keys manually, then rerun this command'
    local z_sa_email_enc="${z_account_email//@/%40}"
    local z_keys_url="${RBGC_CONSOLE_URL}iam-admin/serviceaccounts/details/${z_sa_email_enc}?project=${RBRR_GCP_PROJECT_ID}"

    bcu_warn "Found ${z_user_keys} existing USER_MANAGED key(s) on ${z_account_email}."
    bcu_info "Open Console, select the **Keys** tab, delete old keys, then rerun:"
    bcu_info "  ${z_keys_url}"
    bcu_die  "Aborting to avoid minting additional keys."
  fi

  bcu_step 'Generate service account key'
  local z_key_req="${BDU_TEMP_DIR}/rbga_key_request.json"
  printf '%s' '{"privateKeyType": "TYPE_GOOGLE_CREDENTIALS_FILE"}' > "${z_key_req}"
  rbgu_http_json "POST" "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email}${RBGC_PATH_KEYS}" \
                                         "${z_token}" "${ZRBGA_INFIX_KEY}" "${z_key_req}"
  rbgu_http_require_ok "Generate service account key" "${ZRBGA_INFIX_KEY}"

  bcu_step 'Extract and decode key data'
  local z_key_b64
  z_key_b64=$(rbgu_json_field_capture "${ZRBGA_INFIX_KEY}" '.privateKeyData') \
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

zrbga_create_service_account_no_key() {
  zrbga_sentinel

  local z_account_name="${1:-}"
  local z_display_name="${2:-}"

  test -n "${z_account_name}" || bcu_die "Service account name required"
  test -n "${z_display_name}" || bcu_die "Display name required"

  bcu_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  local z_account_email="${z_account_name}@${RBGC_SA_EMAIL_FULL}"

  bcu_step "Create service account (no key): ${z_account_name}"
  # write body FIRST
  local z_body="${BDU_TEMP_DIR}/rbga_sa_create_nokey.json"
  jq -n --arg account_id   "${z_account_name}" \
        --arg display_name "${z_display_name}" '
    { accountId: $account_id, serviceAccount: { displayName: $display_name } }
  ' > "${z_body}" || bcu_die "Failed to build SA create body"

  # correct endpoint (no trailing slash)
  rbgu_http_json "POST" "${RBGC_API_SERVICE_ACCOUNTS}" "${z_token}" "${ZRBGA_INFIX_CREATE}" "${z_body}"
  rbgu_http_require_ok "Create service account" "${ZRBGA_INFIX_CREATE}" 409 "already exists"

  bcu_log_args 'Verify service account'
  rbgu_http_json "GET" "${RBGC_API_SERVICE_ACCOUNTS}/${z_account_email}" "${z_token}" "${ZRBGA_INFIX_VERIFY}"
  rbgu_http_require_ok "Verify service account" "${ZRBGA_INFIX_VERIFY}"

  bcu_success "Service account ensured (no keys): ${z_account_email}"
}

# Ensure Cloud Build service agent exists and admin can trigger builds
zrbga_ensure_cloudbuild_service_agent() {
  zrbga_sentinel

  local z_token="${1}"
  local z_project_number="${2}"

  local z_cb_service_agent="service-${z_project_number}@gcp-sa-cloudbuild.iam.gserviceaccount.com"
  local z_admin_sa_email="${RBGC_ADMIN_ROLE}@${RBGC_SA_EMAIL_FULL}"
  local z_gen_url="${RBGC_API_CB_GENERATE_SA}"

  rbgu_http_json_lro_ok                                    \
    "Generate Cloud Build service agent"                   \
    "${z_token}"                                           \
    "${z_gen_url}"                                         \
    "${ZRBGA_INFIX_CB_SA_ACCOUNT_GEN}"                     \
    "${ZRBGA_EMPTY_JSON}"                                  \
    ".name"                                                \
    "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}" \
    "${RBGC_OP_PREFIX_GLOBAL}"                             \
    "5"                                                    \
    "60"

  bcu_step 'Grant Cloud Build Service Agent role'
  rbgi_add_project_iam_role                 \
    "Grant Cloud Build Service Agent role"  \
    "${z_token}"                            \
    "${RBGC_PROJECT_RESOURCE}"              \
    "roles/cloudbuild.serviceAgent"         \
    "serviceAccount:${z_cb_service_agent}"  \
    "cb-agent"

  bcu_step 'Grant admin necessary permissions to trigger builds'
  bcu_step "Grant admin Cloud Build permissions"

  bcu_step 'Admin needs serviceAccountUser on the service agent'
  rbgi_add_sa_iam_role "${z_cb_service_agent}" "${z_admin_sa_email}" "roles/iam.serviceAccountUser"

  bcu_step 'Admin needs Cloud Build Editor for builds.create and viz'
  rbgi_add_project_iam_role                 \
    "Grant admin Cloud Build Editor"        \
    "${z_token}"                            \
    "${RBGC_PROJECT_RESOURCE}"              \
    "roles/cloudbuild.builds.editor"        \
    "serviceAccount:${z_admin_sa_email}"    \
    "admin-cb"

  rbgi_add_project_iam_role                 \
    "Grant admin Viewer"                    \
    "${z_token}"                            \
    "${RBGC_PROJECT_RESOURCE}"              \
    "roles/viewer"                          \
    "serviceAccount:${z_admin_sa_email}"    \
    "admin-viewer"

  bcu_info "Cloud Build service agent configured with admin permissions"
}

zrbga_prime_cloud_build() {
  zrbga_sentinel

  local z_token="${1:-}"
  test -n "${z_token}" || bcu_die "zrbga_prime_cloud_build: token required"

  bcu_log_args 'Create degenerate build body with jq'
  local z_body="${BDU_TEMP_DIR}/rbga_cb_prime_body.json"
  jq -n --arg mt "${RBRR_GCB_MACHINE_TYPE:-E2_HIGHCPU_8}" --arg to "${RBRR_GCB_TIMEOUT:-300s}" '
    {
      steps: [
        {
          name: "gcr.io/cloud-builders/gcloud",
          entrypoint: "bash",
          args: ["-lc", "true"]  # intentionally no-op
        }
      ],
      options: { machineType: $mt, logging: "CLOUD_LOGGING_ONLY" },
      timeout: $to
    }' > "${z_body}" || bcu_die "Failed to write cb prime body"

  local z_url="${RBGC_API_ROOT_CLOUDBUILD}${RBGC_CLOUDBUILD_V1}/projects/${RBGC_GCB_PROJECT_ID}/locations/${RBGC_GCB_REGION}/builds"

  rbgu_http_json_lro_ok                                    \
    "Prime Cloud Build"                                    \
    "${z_token}"                                           \
    "${z_url}"                                             \
    "${ZRBGA_INFIX_CB_PRIME}"                              \
    "${z_body}"                                            \
    ".name"                                                \
    "${RBGC_API_ROOT_CLOUDBUILD}${RBGC_CLOUDBUILD_V1}"     \
    "${RBGC_OP_PREFIX_GLOBAL}"                             \
    "10"                                                   \
    "300"

  bcu_log_args 'Prime complete.'
}

zrbga_create_gcs_bucket() {
  zrbga_sentinel

  local z_token="${1}"
  local z_bucket_name="${2}"

  bcu_log_args 'Create bucket request JSON for '"${z_bucket_name}"
  local z_bucket_req="${BDU_TEMP_DIR}/rbga_bucket_create_req.json"
  jq -n --arg name "${z_bucket_name}"            \
        --arg location "${RBGC_GAR_LOCATION}"    \
    '{
      name: $name,
      location: $location,
      storageClass: "STANDARD"
    }' > "${z_bucket_req}" || bcu_die "Failed to create bucket request JSON"

  bcu_log_args 'Send bucket creation request'
  local z_code
  local z_err
  rbgu_http_json "POST" "${RBGC_API_GCS_BUCKET_CREATE}" "${z_token}" \
                                   "${ZRBGA_INFIX_BUCKET_CREATE}" "${z_bucket_req}"
  z_code=$(rbgu_http_code_capture "${ZRBGA_INFIX_BUCKET_CREATE}") || bcu_die "Bad bucket creation HTTP code"
  z_err=$(rbgu_json_field_capture "${ZRBGA_INFIX_BUCKET_CREATE}" '.error.message') || z_err="HTTP ${z_code}"

  case "${z_code}" in
    200|201) bcu_info "Bucket ${z_bucket_name} created";        return 0 ;;
    409)     bcu_warn "Bucket ${z_bucket_name} already exists"; return 0 ;;
    *)       bcu_die "Failed to create bucket: ${z_err}"                 ;;
  esac
}

zrbga_list_bucket_objects_capture() {
  zrbga_sentinel

  local z_token="${1}"
  local z_bucket_name="${2}"

  local z_list_url_base="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}/o"
  local z_page_token=""
  local z_first=1

  while :; do
    bcu_log_args "Build URL with optional pageToken -> ${z_first}"
    local z_url="${z_list_url_base}"
    if test -n "${z_page_token}"; then
      bcu_log_args 'pageToken must be URL-encoded'
      local z_tok_enc
      z_tok_enc=$(rbgu_urlencode_capture "${z_page_token}") || return 1
      z_url="${z_url}?pageToken=${z_tok_enc}"
    fi

    bcu_log_args 'Use a unique infix per page to avoid clobbering files'
    local z_infix="${ZRBGA_INFIX_BUCKET_LIST}${z_first}"
    rbgu_http_json "GET" "${z_url}" "${z_token}" "${z_infix}"

    local z_code
    z_code=$(rbgu_http_code_capture "${z_infix}") || return 1
    test "${z_code}" = "200" || return 1

    bcu_log_args 'Print names from this page (if any)'
    bcu_log_args 'Next page?'
    jq -r                '.items[]?.name // empty' "${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_JSON}"  || return 1
    z_page_token=$(jq -r '.nextPageToken // empty' "${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_JSON}") || return 1

    test -n "${z_page_token}" || break
    z_first=$((z_first + 1))
  done
}

zrbga_get_project_number_capture() {
  zrbga_sentinel

  local z_token
  z_token=$(rbgu_get_admin_token_capture) || return 1

  rbgu_http_json "GET" "${RBGC_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGA_INFIX_PROJECT_INFO}"
  rbgu_http_require_ok "Get project info" "${ZRBGA_INFIX_PROJECT_INFO}" || return 1

  local z_project_number
  z_project_number=$(rbgu_json_field_capture "${ZRBGA_INFIX_PROJECT_INFO}" '.projectNumber') || return 1
  test -n "${z_project_number}" || return 1

  echo "${z_project_number}"
}

zrbga_empty_gcs_bucket() {
  zrbga_sentinel

  local z_token="${1}"
  local z_bucket_name="${2}"

  bcu_log_args 'Get list of objects to delete'
  local z_objects
  z_objects=$(zrbga_list_bucket_objects_capture "${z_token}" "${z_bucket_name}") || {
    bcu_log_args 'No objects found or bucket not accessible'
    return 0
  }

  test -n "${z_objects}" || { bcu_log_args 'Bucket is empty'; return 0; }

  bcu_log_args 'Delete each object'
  local z_object=""
  local z_delete_url=""
  local z_delete_code=""
  while IFS= read -r z_object; do
    test -n "${z_object}" || continue
    bcu_log_args "Deleting object: ${z_object}"

    local z_object_enc
    z_object_enc=$(rbgu_urlencode_capture "${z_object}") || z_object_enc=""
    test -n "${z_object_enc}" || { bcu_warn "Failed to encode object name: ${z_object}"; continue; }
    z_delete_url="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}/o/${z_object_enc}"

    rbgu_http_json "DELETE" "${z_delete_url}" \
                              "${z_token}" "${ZRBGA_INFIX_OBJECT_DELETE}"
    z_delete_code=$(rbgu_http_code_capture "${ZRBGA_INFIX_OBJECT_DELETE}") || z_delete_code=""
    case "${z_delete_code}" in
      204|404) bcu_log_args "Object ${z_object}: deleted or not found"                     ;;
      *)       bcu_warn     "Object ${z_object}: Failed to delete (HTTP ${z_delete_code})" ;;
    esac
  done <<< "${z_objects}"
}

zrbga_delete_gcs_bucket_predicate() {
  zrbga_sentinel

  local z_token="${1}"
  local z_bucket_name="${2}"

  bcu_log_args 'Empty bucket before deletion: '"${z_bucket_name}"
  zrbga_empty_gcs_bucket "${z_token}" "${z_bucket_name}"

  bcu_log_args 'Delete the bucket'
  local z_code
  local z_err
  local z_delete_url="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}"
  rbgu_http_json "DELETE" "${z_delete_url}" \
                      "${z_token}" "${ZRBGA_INFIX_BUCKET_DELETE}"
  z_code=$(rbgu_http_code_capture "${ZRBGA_INFIX_BUCKET_DELETE}") || z_code=""
  z_err=$(rbgu_json_field_capture "${ZRBGA_INFIX_BUCKET_DELETE}" '.error.message') || z_err="HTTP ${z_code}"
  case "${z_code}" in
    204) bcu_info "Bucket ${z_bucket_name} deleted";                           return 0 ;;
    404) bcu_warn "Bucket ${z_bucket_name} not found (already deleted)";       return 0 ;;
    409) bcu_warn "Bucket ${z_bucket_name} not empty or has retention policy"; return 1 ;;
    *)   bcu_warn "Bucket ${z_bucket_name} failed delete";                     return 1 ;;
  esac
}

######################################################################
# External Functions (rbga_*)

rbga_initialize_admin() {
  zrbga_sentinel

  local z_json_path="${1:-}"

  bcu_doc_brief "Initialize RBGA for this project: enable/verify APIs, create GAR repo, and grant Cloud Build SA."
  bcu_doc_param "json_path" "Path to downloaded admin JSON key (will be converted to RBRA)"
  bcu_doc_shown || return 0

  test -n "${z_json_path}" || bcu_die "First argument must be path to downloaded JSON key file."

  local z_admin_sa_email="${RBGC_ADMIN_ROLE}@${RBGC_SA_EMAIL_FULL}"
  local z_prime_pause_sec=120

  bcu_step 'Convert admin JSON to RBRA'
  rbgu_extract_json_to_rbra "${z_json_path}" "${RBRR_ADMIN_RBRA_FILE}" "1800"

  bcu_step 'Mint admin OAuth token'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step 'Check which required APIs need enabling'
  local z_missing=""
  z_missing=$(zrbga_required_apis_missing_capture "${z_token}") \
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
      "${ZRBGA_INFIX_API_IAM_ENABLE}"                           \
      "${ZRBGA_EMPTY_JSON}"                                     \
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
      "${ZRBGA_INFIX_API_CRM_ENABLE}"                           \
      "${ZRBGA_EMPTY_JSON}"                                     \
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
      "${ZRBGA_INFIX_API_ART_ENABLE}"                           \
      "${ZRBGA_EMPTY_JSON}"                                     \
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
      "${ZRBGA_INFIX_API_BUILD_ENABLE}"                         \
      "${ZRBGA_EMPTY_JSON}"                                     \
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
      "${ZRBGA_INFIX_API_CONTAINERANALYSIS_ENABLE}"             \
      "${ZRBGA_EMPTY_JSON}"                                     \
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
      "${ZRBGA_INFIX_API_STORAGE_ENABLE}"                       \
      "${ZRBGA_EMPTY_JSON}"                                     \
      ".name"                                                   \
      "${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}"    \
      "${RBGC_OP_PREFIX_GLOBAL}"                                \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                        \
      "${RBGC_MAX_CONSISTENCY_SEC}"
  fi

  bcu_step 'Discover Project Number'
  local z_project_number
  z_project_number=$(zrbga_get_project_number_capture) || bcu_die "Failed to get project number"

  bcu_step 'Directly create the cloudbuild service agent'
  zrbga_ensure_cloudbuild_service_agent "${z_token}" "${z_project_number}"

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
  zrbga_create_gcs_bucket "${z_token}" "${RBGC_GCS_BUCKET}"

  bcu_step 'Create/verify Docker format Artifact Registry repo'
  bcu_log_args "  The repo is ${RBRR_GAR_REPOSITORY} in ${RBGC_GAR_LOCATION}"

  test -n "${RBGC_GAR_LOCATION:-}"   || bcu_die "RBGC_GAR_LOCATION is not set"
  test -n "${RBRR_GAR_REPOSITORY:-}" || bcu_die "RBRR_GAR_REPOSITORY is not set"

  local z_parent="projects/${RBRR_GCP_PROJECT_ID}${RBGC_PATH_LOCATIONS}/${RBGC_GAR_LOCATION}"
  local z_resource="${z_parent}${RBGC_PATH_REPOSITORIES}/${RBRR_GAR_REPOSITORY}"
  local z_create_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_parent}${RBGC_PATH_REPOSITORIES}?repositoryId=${RBRR_GAR_REPOSITORY}"
  local z_get_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}"
  local z_create_body="${BDU_TEMP_DIR}/rbga_create_repo_body.json"

  jq -n '{format:"DOCKER"}' > "${z_create_body}" || bcu_die "Failed to build create-repo body"

  bcu_step 'Create DOCKER format repo'
  rbgu_http_json_lro_ok                                              \
    "Create Artifact Registry repo"                                  \
    "${z_token}"                                                     \
    "${z_create_url}"                                                \
    "${ZRBGA_INFIX_CREATE_REPO}"                                     \
    "${z_create_body}"                                               \
    ".operation.name"                                                \
    "${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}"   \
    "${RBGC_OP_PREFIX_GLOBAL}"                                       \
    "${RBGC_EVENTUAL_CONSISTENCY_SEC}"                               \
    "${RBGC_MAX_CONSISTENCY_SEC}"

  bcu_step 'One-time propagation pause before Cloud Build priming'
  bcu_step "  About to sleep ${z_prime_pause_sec}s"
  sleep "${z_prime_pause_sec}"

  bcu_step 'Trigger degenerate build to assure builder account creation'
  zrbga_prime_cloud_build "${z_token}"

  bcu_step 'Verify repository exists and is DOCKER format'
  rbgu_http_json "GET" "${z_get_url}" "${z_token}" "${ZRBGA_INFIX_VERIFY_REPO}"
  rbgu_http_require_ok "Verify repository"         "${ZRBGA_INFIX_VERIFY_REPO}"
  test "$(rbgu_json_field_capture                  "${ZRBGA_INFIX_VERIFY_REPO}" '.format')" = "DOCKER" \
    || bcu_die "Repository exists but not DOCKER format"

  bcu_step 'Verify Cloud Build runtime SA is readable after propagation pause'
  local z_cb_sa="${z_project_number}@cloudbuild.gserviceaccount.com"
  local z_cb_sa_enc
  z_cb_sa_enc=$(rbgu_urlencode_capture "${z_cb_sa}") || bcu_die "Failed to encode SA email"
  local z_peek_code
  rbgu_http_json "GET" "${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/-/serviceAccounts/${z_cb_sa_enc}" \
                           "${z_token}" "${ZRBGA_INFIX_CB_RUNTIME_SA_PEEK}"
  z_peek_code=$(rbgu_http_code_capture "${ZRBGA_INFIX_CB_RUNTIME_SA_PEEK}") || z_peek_code="000"
  test "${z_peek_code}" = "200" || bcu_die "Cloud Build runtime SA not readable after fixed pause (HTTP ${z_peek_code})"

  bcu_step 'Grant Storage Object Admin to Cloud Build SA on bucket'
  rbgi_add_bucket_iam_role "${RBGC_GCS_BUCKET}" "${z_cb_sa}" "roles/storage.objectAdmin" "${z_token}"

  bcu_step 'Grant Artifact Registry Writer to Cloud Build SA on repo'
  rbgi_add_repo_iam_role "${z_cb_sa}" "${RBGC_GAR_LOCATION}" "${RBRR_GAR_REPOSITORY}" "${RBGC_ROLE_ARTIFACTREGISTRY_WRITER}"

  bcu_step 'Ensure Mason service account exists (no keys)'
  zrbga_create_service_account_no_key "${RBGC_MASON_NAME}" "RBGA Mason (build executor)"

  bcu_log_args 'Compute Cloud Build runtime SA and Mason email'
  local z_mason_sa="${RBGC_MASON_EMAIL}"

  bcu_step 'Allow Cloud Build to impersonate Mason (TokenCreator on Mason)'
  rbgi_add_sa_iam_role "${z_mason_sa}" "${z_cb_sa}" "roles/iam.serviceAccountTokenCreator"

  bcu_step 'Grant Artifact Registry Admin (repo-scoped) to Mason'
  rbgi_add_repo_iam_role "${z_mason_sa}" "${RBGC_GAR_LOCATION}" "${RBRR_GAR_REPOSITORY}" \
    "${RBGC_ROLE_ARTIFACTREGISTRY_ADMIN}"

  bcu_step 'Grant Storage Object Admin on artifacts bucket to Mason'
  rbgi_add_bucket_iam_role "${RBGC_GCS_BUCKET}" "${z_mason_sa}" "roles/storage.objectAdmin"

  bcu_step 'Grant Project Viewer to Mason'
  rbgi_add_project_iam_role "Grant Project Viewer" "${z_token}" "${RBGC_PROJECT_RESOURCE}" \
                            "roles/viewer" "serviceAccount:${z_mason_sa}" "mason-viewer"

  bcu_info "RBRA (admin): ${RBRR_ADMIN_RBRA_FILE}"
  bcu_info "GAR: ${RBGC_GAR_LOCATION}/${RBRR_GAR_REPOSITORY} (DOCKER)"
  bcu_info "Cloud Build SA granted writer on repo: ${z_cb_sa}"
  bcu_warn "RBRR file stashed. Consider deleting carriage JSON:"
  bcu_code ""
  bcu_code "    rm \"${z_json_path}\""
  bcu_code ""

  bcu_success 'Admin initialization complete'
}

rbga_destroy_admin() {
  zrbga_sentinel

  bcu_doc_brief "Destroy project-specific GAR resources and related repo-scoped IAM. Leaves project-wide APIs and SAs unchanged."
  bcu_doc_shown || return 0

  bcu_step 'Mint admin OAuth token'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step 'Preflight: Determine if all required APIs enabled'
  local z_missing
  z_missing=$(zrbga_required_apis_missing_capture "${z_token}") || bcu_die "Failed to check API status"

  if test -n "${z_missing}"; then
    bcu_die "Required APIs not enabled: ${z_missing}. Run rbga_initialize_admin to enable them, then re-run destroy."
  fi

  bcu_step 'Confirm'
  bcu_require "Confirm full reset of this project?" "YES"
  bcu_require "Be very very sure!" "I-AM-SURE"

  bcu_step 'Discover Project Number Cloud Build SA (to prune repo binding cleanly)'

  rbgu_http_json "GET" "${RBGC_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGA_INFIX_PROJECT_INFO}"
  rbgu_http_require_ok "Get project info"                         "${ZRBGA_INFIX_PROJECT_INFO}"
  local z_project_number
  z_project_number=$(rbgu_json_field_capture "${ZRBGA_INFIX_PROJECT_INFO}" '.projectNumber') \
    || bcu_die "Failed to extract project number"

  local z_cb_sa_member="serviceAccount:${z_project_number}@cloudbuild.gserviceaccount.com"
  local z_resource="projects/${RBRR_GCP_PROJECT_ID}${RBGC_PATH_LOCATIONS}/${RBGC_GAR_LOCATION}${RBGC_PATH_REPOSITORIES}/${RBRR_GAR_REPOSITORY}"
  local z_get_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:getIamPolicy"
  local z_set_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:setIamPolicy"

  bcu_step 'Prune Cloud Build SA writer binding (idempotent; harmless if missing)'

  bcu_step 'Fetch current repo IAM policy'
  rbgu_http_json "POST" "${z_get_url}" "${z_token}" "${ZRBGA_INFIX_REPO_POLICY}" "${ZRBGA_EMPTY_JSON}"
  rbgu_http_require_ok "Get repo IAM policy"        "${ZRBGA_INFIX_REPO_POLICY}" 404 "repo not found (already deleted)"

  bcu_log_args 'Guard the prune+set when the repo is already gone'
  local z_get_code
  z_get_code=$(rbgu_http_code_capture "${ZRBGA_INFIX_REPO_POLICY}") || z_get_code="000"
  if test "${z_get_code}" = "404"; then
    bcu_warn "Repo missing; skip writer-binding prune."
  else
    bcu_step 'Strip Cloud Build SA from artifactregistry.writer binding'
    local z_updated_policy="${BDU_TEMP_DIR}/rbga_repo_policy_pruned.json"
    jq --arg role "${RBGC_ROLE_ARTIFACTREGISTRY_WRITER}"   \
       --arg member "${z_cb_sa_member}"                    \
       '
         .bindings = (.bindings // []) |
         .bindings = [ .bindings[] |
           if .role == $role then .members = ((.members // []) | map(select(. != $member)))
           else . end
         ] |
         .bindings = [ .bindings[] | select((.members // []) | length > 0) ]
       ' "${ZRBGU_PREFIX}${ZRBGA_INFIX_REPO_POLICY}${ZRBGU_POSTFIX_JSON}" > "${z_updated_policy}" \
      || bcu_die "Failed to prune writer binding"

    local z_repo_set_body="${BDU_TEMP_DIR}/rbga_repo_set_policy_body.json"
    jq -n --slurpfile p "${z_updated_policy}" '{policy:$p[0]}' > "${z_repo_set_body}" \
      || bcu_die "Failed to build repo setIamPolicy body"

    rbgu_http_json "POST" "${z_set_url}" "${z_token}" "${ZRBGA_INFIX_RPOLICY_SET}" "${z_repo_set_body}"
    rbgu_http_require_ok "Set repo IAM policy"        "${ZRBGA_INFIX_RPOLICY_SET}"
  fi

  bcu_step 'Delete the GAR repository (removes remaining repo-scoped bindings/data)'

  bcu_step "Delete Artifact Registry repo '${RBRR_GAR_REPOSITORY}' in ${RBGC_GAR_LOCATION}"
  local z_delete_code
  rbgu_http_json "DELETE"                                                           \
    "${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}"    \
                            "${z_token}" "${ZRBGA_INFIX_DELETE_REPO}"
  z_delete_code=$(rbgu_http_code_capture "${ZRBGA_INFIX_DELETE_REPO}") || z_delete_code="000"
  case "${z_delete_code}" in
    200|204) bcu_info "Repository deleted" ;;
    404)     bcu_warn "Repository not found (already deleted)" ;;
    *)
      local z_err
      z_err=$(rbgu_json_field_capture "${ZRBGA_INFIX_DELETE_REPO}" '.error.message') || z_err="Unknown error"
      bcu_die "Failed to delete repository: ${z_err}"
      ;;
  esac

  bcu_step 'Delete Cloud Storage bucket'
  zrbga_delete_gcs_bucket_predicate "${z_token}"  "${RBGC_GCS_BUCKET}"

  bcu_step 'Delete all service accounts except admin'

  bcu_log_args 'List all service accounts'
  rbgu_http_json "GET" "${RBGC_API_SERVICE_ACCOUNTS}" "${z_token}" "${ZRBGA_INFIX_LIST}"
  rbgu_http_require_ok "List service accounts"                     "${ZRBGA_INFIX_LIST}"

  bcu_log_args 'Extract emails to delete (all except admin)'
  local z_admin_email="${RBGC_ADMIN_ROLE}@${RBGC_SA_EMAIL_FULL}"
  local z_emails_to_delete
  z_emails_to_delete=$(jq -r --arg admin "${z_admin_email}" \
    '.accounts[]? | select(.email != $admin) | .email' \
    "${ZRBGU_PREFIX}${ZRBGA_INFIX_LIST}${ZRBGU_POSTFIX_JSON}") || z_emails_to_delete=""

  if test -n "${z_emails_to_delete}"; then
    local z_sa_email
    local z_del_code
    while IFS= read -r z_sa_email; do
      test -n "${z_sa_email}" || continue
      bcu_log_args "Deleting service account: ${z_sa_email}"

      rbgu_http_json "DELETE" "${RBGC_API_SERVICE_ACCOUNTS}/${z_sa_email}" \
                             "${z_token}" "${ZRBGA_INFIX_DELETE}"
      z_del_code=$(rbgu_http_code_capture "${ZRBGA_INFIX_DELETE}") || z_del_code=""
      case "${z_del_code}" in
        200|204) bcu_log_args "Deleted: ${z_sa_email}" ;;
        404)     bcu_log_args "Already gone: ${z_sa_email}" ;;
        *)       bcu_warn "Failed to delete ${z_sa_email} (HTTP ${z_del_code}), continuing" ;;
      esac
    done <<< "${z_emails_to_delete}"
  fi

  bcu_step 'Waiting 45s for IAM deletion propagation'
  sleep 45

  bcu_success 'RBGA nuclear destruction complete (admin account preserved)'
}

rbga_list_service_accounts() {
  zrbga_sentinel

  bcu_doc_brief "List all service accounts in the project"
  bcu_doc_shown || return 0

  bcu_step 'Listing service accounts in project: '"${RBRR_GCP_PROJECT_ID}"

  bcu_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token (rc=$?)"

  bcu_log_args 'List service accounts via REST API'
  rbgu_http_json "GET" "${RBGC_API_SERVICE_ACCOUNTS}" "${z_token}" "${ZRBGA_INFIX_LIST}"
  rbgu_http_require_ok "List service accounts"                     "${ZRBGA_INFIX_LIST}"

  local z_count
  z_count=$(rbgu_json_field_capture "${ZRBGA_INFIX_LIST}" '.accounts | length') \
    || bcu_die "Failed to parse response"

  if test "${z_count}" = "0"; then
    bcu_info "No service accounts found in project"
    return 0
  fi

  bcu_step "Found ${z_count} service account(s):"

  local z_max_width
  z_max_width=$(jq -r '.accounts[].email | length' "${ZRBGU_PREFIX}${ZRBGA_INFIX_LIST}${ZRBGU_POSTFIX_JSON}" | sort -n | tail -1) \
    || bcu_die "Failed to calculate max width"

  jq -r --argjson width "${z_max_width}" \
    '.accounts[] | "  " + (.email | tostring | ((" " * ($width - length)) + .)) + " - " + (.displayName // "(no display name)")' \
    "${ZRBGU_PREFIX}${ZRBGA_INFIX_LIST}${ZRBGU_POSTFIX_JSON}" || bcu_die "Failed to format accounts"

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

  zrbga_create_service_account_with_key                                        \
    "${z_account_name}"                                                        \
    "Recipe Bottle Retriever (${z_instance})"                                  \
    "Read-only access to Google Artifact Registry - instance: ${z_instance}"   \
    "${z_instance}"

  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step 'Adding Artifact Registry Reader role'
  rbgi_add_project_iam_role                 \
    "Grant Artifact Registry Reader"        \
    "${z_token}"                            \
    "${RBGC_PROJECT_RESOURCE}"              \
    "${RBGC_ROLE_ARTIFACTREGISTRY_READER}"  \
    "serviceAccount:${z_account_email}"     \
    "retriever-reader"

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

  test -n "${z_instance}"     || bcu_die "Instance name required"
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

  bcu_step 'Get OAuth token from admin'
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_step 'Get project number for Cloud Build SA'
  rbgu_http_json "GET" "${RBGC_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGA_INFIX_PROJECT_INFO}"
  rbgu_http_require_ok "Get project info" "${ZRBGA_INFIX_PROJECT_INFO}"

  local z_project_number
  z_project_number=$(rbgu_json_field_capture "${ZRBGA_INFIX_PROJECT_INFO}" '.projectNumber') \
    || bcu_die "Failed to extract project number"

  bcu_step 'Adding Cloud Build Editor role (project scope)'
  rbgi_add_project_iam_role                 \
    "Grant Cloud Build Editor"              \
    "${z_token}"                            \
    "${RBGC_PROJECT_RESOURCE}"              \
    "${RBGC_ROLE_CLOUDBUILD_BUILDS_EDITOR}" \
    "serviceAccount:${z_account_email}"     \
    "director-cb"

  rbgi_add_project_iam_role                 \
    "Grant Project Viewer"                  \
    "${z_token}"                            \
    "${RBGC_PROJECT_RESOURCE}"              \
    "roles/viewer"                          \
    "serviceAccount:${z_account_email}"     \
    "director-viewer"

  bcu_step 'Grant serviceAccountUser on Mason'
  rbgi_add_sa_iam_role "${RBGC_MASON_EMAIL}" "${z_account_email}" "roles/iam.serviceAccountUser"

  bcu_step 'Grant Storage Object Creator on artifacts bucket (only if pre-upload used)'
  rbgi_add_bucket_iam_role "${RBGC_GCS_BUCKET}" "${z_account_email}" "roles/storage.objectCreator"

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
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_log_args 'Delete via REST API'
  rbgu_http_json "DELETE" "${RBGC_API_SERVICE_ACCOUNTS}/${z_sa_email}" "${z_token}" \
                                                 "${ZRBGA_INFIX_DELETE}"
  rbgu_http_require_ok "Delete service account" "${ZRBGA_INFIX_DELETE}" \
    404 "not found (already deleted)"

  bcu_success "Delete operation completed"
}

# eof

