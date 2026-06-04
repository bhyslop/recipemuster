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

  test -n "${RBDC_DEPOT_PROJECT_ID:-}"     || buc_die "RBDC_DEPOT_PROJECT_ID is not set"
  test   "${#RBDC_DEPOT_PROJECT_ID}" -gt 0 || buc_die "RBDC_DEPOT_PROJECT_ID is empty"

  buc_log_args 'Ensure dependencies are kindled first'
  zrbgc_sentinel
  zrbgo_sentinel
  zrbuh_sentinel
  zrbgi_sentinel

  readonly ZRBGG_PREFIX="${BURD_TEMP_DIR}/rbgg_"
  readonly ZRBGG_EMPTY_JSON="${ZRBGG_PREFIX}empty.json"
  printf '{}' > "${ZRBGG_EMPTY_JSON}"

  # Infix values for HTTP operations
  readonly ZRBGG_INFIX_CREATE="create"
  readonly ZRBGG_INFIX_PREFLIGHT="preflight"
  readonly ZRBGG_INFIX_VERIFY="verify"
  readonly ZRBGG_INFIX_KEY="key"
  readonly ZRBGG_INFIX_API_IAM_ENABLE="api_iam_enable"
  readonly ZRBGG_INFIX_API_CRM_ENABLE="api_crm_enable"
  readonly ZRBGG_INFIX_API_ART_ENABLE="api_art_enable"
  readonly ZRBGG_INFIX_API_BUILD_ENABLE="api_build_enable"
  readonly ZRBGG_INFIX_CB_SA_ACCOUNT_GEN="cb_account_gen"
  readonly ZRBGG_INFIX_CB_PRIME="cb_prime"
  readonly ZRBGG_INFIX_API_CONTAINERANALYSIS_ENABLE="api_containeranalysis_enable"
  readonly ZRBGG_INFIX_API_STORAGE_ENABLE="api_storage_enable"
  readonly ZRBGG_INFIX_PROJECT_INFO="project_info"
  readonly ZRBGG_INFIX_CREATE_REPO="create_repo"
  readonly ZRBGG_INFIX_VERIFY_REPO="verify_repo"
  readonly ZRBGG_INFIX_DELETE_REPO="delete_repo"
  readonly ZRBGG_INFIX_REPO_POLICY="repo_policy"
  readonly ZRBGG_INFIX_RPOLICY_SET="repo_policy_set"
  readonly ZRBGG_INFIX_ROSTER="roster"
  readonly ZRBGG_INFIX_API_CHECK="api_checking"
  readonly ZRBGG_INFIX_DELETE="delete"
  readonly ZRBGG_INFIX_DELETE_GONE="delete_gone"
  readonly ZRBGG_INFIX_LIST_KEYS="list_keys"
  readonly ZRBGG_INFIX_BUCKET_CREATE="bucket_create"
  readonly ZRBGG_INFIX_BUCKET_DELETE="bucket_delete"
  readonly ZRBGG_INFIX_BUCKET_LIST="bucket_list"
  readonly ZRBGG_INFIX_OBJECT_DELETE="object_delete"
  readonly ZRBGG_INFIX_LIST_LIENS="list_liens"
  readonly ZRBGG_INFIX_PROJECT_DELETE="project_delete"
  readonly ZRBGG_INFIX_PROJECT_STATE="project_state"
  readonly ZRBGG_INFIX_PROJECT_RESTORE="project_restore"

  readonly ZRBGG_KINDLED=1
}

zrbgg_sentinel() {
  test "${ZRBGG_KINDLED:-}" = "1" || buc_die "Module rbgg not kindled - call zrbgg_kindle first"
}

# Path-driven sibling of rbuh_json_field_capture for files that live outside
# the rbgu cache (under RBRR_SECRETS_DIR/assay/* for credential bytes that
# must not leak into BURD_TEMP_DIR).
zrbgg_secrets_json_field_capture() {
  zrbgg_sentinel
  local -r z_path="${1}"
  local -r z_jq="${2}"
  local z_result
  z_result=$(jq -r "${z_jq}" "${z_path}")  || return 1
  if test -z "${z_result}" || test "${z_result}" = "null"; then return 1; fi
  echo "${z_result}"
}

######################################################################
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

    rbuh_json "GET" "${z_api}" "${z_token}" "${z_infix}" || true

    buc_log_args 'If we cannot even read an HTTP code file, that is a processing failure.'
    z_code=$(rbuh_code_capture "${z_infix}") || z_code=""
    test -n "${z_code}" || return 1

    if test "${z_code}" = "200"; then
      z_state=$(rbuh_json_field_capture "${z_infix}" ".state") || z_state=""
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
  local z_role="$4"

  local z_account_email="${z_account_name}@${RBGD_SA_EMAIL_FULL}"

  buc_step 'Get OAuth token from admin'
  local z_token
  z_token=$(rba_get_governor_token_capture) || buc_die "Failed to get admin token"

  buc_step "Preflight: confirm ${z_account_name} does not already exist"
  # Invest is fail-loud by design (RBSRK/RBSDK): a pre-existing SA is treated as
  # state drift, not an idempotent rerun. The operator clears it with the
  # matching GovernorDivests verb (rbw-arD/rbw-adD) before re-investing.
  rbuh_json "GET" "${RBGD_API_SERVICE_ACCOUNTS}/${z_account_email}" "${z_token}" \
                                                 "${ZRBGG_INFIX_PREFLIGHT}"
  local z_preflight_code
  z_preflight_code=$(rbuh_code_capture "${ZRBGG_INFIX_PREFLIGHT}") || z_preflight_code=""
  case "${z_preflight_code}" in
    404) buc_log_args "Service account ${z_account_email} absent — clear to create" ;;
    200) buc_die "Service account ${z_account_email} already exists — run the matching GovernorDivests verb (rbw-arD/rbw-adD) first to re-key" ;;
    *)   buc_die "Preflight GET for ${z_account_email} returned unexpected HTTP ${z_preflight_code}" ;;
  esac

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
  rbuh_json "POST" "${RBGD_API_SERVICE_ACCOUNTS}" "${z_token}" \
    "${ZRBGG_INFIX_CREATE}" "${ZRBGG_PREFIX}create_request.json"
  rbuh_require_ok "Create service account" "${ZRBGG_INFIX_CREATE}"

  local z_sa_uid
  z_sa_uid=$(rbuh_json_field_capture "${ZRBGG_INFIX_CREATE}" '.uniqueId') \
    || buc_die "Failed to get uniqueId from SA creation response"
  buc_info "Service account created: ${z_account_email} (uid: ${z_sa_uid})"

  rbuh_poll_until_ok "SA propagation (by email)" \
    "${RBGD_API_SERVICE_ACCOUNTS}/${z_account_email}" "${z_token}" "${ZRBGG_INFIX_VERIFY}"

  buc_step 'Preflight: ensure no existing USER_MANAGED keys (manual cleanup path)'

  # Subresource read-path can lag SA email-path readiness — flap observed
  # 200→200→404 on freshly-minted depots. Retry on the actual call, not a
  # proxy probe (see RBSCIP "SA keys subresource propagation").
  local z_list_attempt=0
  local z_list_infix=""
  local z_list_code=""
  while :; do
    z_list_attempt=$((z_list_attempt + 1))
    z_list_infix="${ZRBGG_INFIX_LIST_KEYS}-attempt${z_list_attempt}"
    rbuh_json "GET"                                                        \
      "${RBGD_API_SERVICE_ACCOUNTS}/${z_account_email}${RBGC_PATH_KEYS}"        \
                                        "${z_token}" "${z_list_infix}"

    z_list_code=$(rbuh_code_capture "${z_list_infix}") || z_list_code=""

    if test "${z_list_code}" = "200"; then
      break
    fi

    if test "${z_list_code}" = "404" && test "${z_list_attempt}" -lt "${RBGC_SA_KEY_CREATE_RETRY_MAX}"; then
      buc_warn "keys.list returned 404 (SA read-path propagation delay), retry ${z_list_attempt}/${RBGC_SA_KEY_CREATE_RETRY_MAX} in ${RBGC_SA_KEY_CREATE_RETRY_DELAY_SEC}s..."
      sleep "${RBGC_SA_KEY_CREATE_RETRY_DELAY_SEC}"
      continue
    fi

    rbuh_require_ok "List service account keys" "${z_list_infix}"
  done

  buc_log_args 'Count existing user-managed keys'
  local z_user_keys
  z_user_keys=$(rbuh_json_field_capture "${z_list_infix}" \
                 '[.keys[]? | select(.keyType=="USER_MANAGED")] | length') \
    || buc_die "Failed to parse service account keys"

  if test "${z_user_keys}" -gt 0; then
    buc_log_args 'Provide a console URL to delete keys manually, then rerun this command'
    local z_sa_email_enc="${z_account_email//@/%40}"
    local z_keys_url="${RBGC_CONSOLE_URL}iam-admin/serviceaccounts/details/${z_sa_email_enc}?project=${RBDC_DEPOT_PROJECT_ID}"

    buc_warn "Found ${z_user_keys} existing USER_MANAGED key(s) on ${z_account_email}."
    buc_info "Open Console, select the **Keys** tab, delete old keys, then rerun:"
    buc_info "  ${z_keys_url}"
    buc_die  "Aborting to avoid minting additional keys."
  fi

  buc_step 'Generate service account key (with propagation retry)'
  local z_key_req="${BURD_TEMP_DIR}/rbgg_key_request.json"
  printf '%s' '{"privateKeyType": "TYPE_GOOGLE_CREDENTIALS_FILE"}' > "${z_key_req}"

  local z_key_attempt=0
  local z_key_infix=""
  local z_key_code=""
  while :; do
    z_key_attempt=$((z_key_attempt + 1))
    z_key_infix="${ZRBGG_INFIX_KEY}-attempt${z_key_attempt}"
    rbuh_json "POST" "${RBGD_API_SERVICE_ACCOUNTS}/${z_account_email}${RBGC_PATH_KEYS}" \
                                           "${z_token}" "${z_key_infix}" "${z_key_req}"

    z_key_code=$(rbuh_code_capture "${z_key_infix}") || z_key_code=""

    if test "${z_key_code}" = "200"; then
      break
    fi

    if test "${z_key_code}" = "404" && test "${z_key_attempt}" -lt "${RBGC_SA_KEY_CREATE_RETRY_MAX}"; then
      buc_warn "keys.create returned 404 (SA write-path propagation delay), retry ${z_key_attempt}/${RBGC_SA_KEY_CREATE_RETRY_MAX} in ${RBGC_SA_KEY_CREATE_RETRY_DELAY_SEC}s..."
      sleep "${RBGC_SA_KEY_CREATE_RETRY_DELAY_SEC}"
      continue
    fi

    rbuh_require_ok "Generate service account key" "${z_key_infix}"
  done

  buc_step 'Extract and decode key data'
  local z_key_b64
  z_key_b64=$(rbuh_json_field_capture "${z_key_infix}" '.privateKeyData') \
    || buc_die "Failed to extract privateKeyData"
  # Decode into the assay subdirectory (RBRR_SECRETS_DIR/assay/) so the
  # only readable form of the private key shares lifecycle and location
  # with the final RBRA file — credentials never leak into BURD_TEMP_DIR.
  local -r z_assay_dir="${RBDC_ASSAY_RBRA_FILE%/*}"
  local -r z_key_json="${z_assay_dir}/_decoded_key_${z_account_name}.json"
  rbgo_base64_decode_string_to_file "${z_key_b64}" "${z_key_json}" \
    || buc_die "Failed to decode key data"

  buc_step 'Convert JSON key to RBRA format'
  local z_rbra_file="${RBDC_ASSAY_RBRA_FILE}"

  local z_client_email
  z_client_email=$(zrbgg_secrets_json_field_capture "${z_key_json}" '.client_email') \
    || buc_die "Failed to extract client_email from key JSON"

  local z_private_key
  z_private_key=$(zrbgg_secrets_json_field_capture "${z_key_json}" '.private_key') \
    || buc_die "Failed to extract private_key from key JSON"

  local z_project_id
  z_project_id=$(zrbgg_secrets_json_field_capture "${z_key_json}" '.project_id') \
    || buc_die "Failed to extract project_id from key JSON"

  buc_step 'Write RBRA file' "${z_rbra_file}"
  # CAUTION: jq -r unescapes JSON \n to real newlines, so z_private_key holds a
  # multi-line PEM string. printf '%s' below preserves real newlines into the
  # RBRA file. Consumer (rbgo_OAuth.sh:zrbgo_build_jwt_capture) tolerates either
  # real-newline or '\n'-escape form via printf '%b'; do not "normalize" to one
  # form without auditing the consumer.
  {
    printf 'RBRA_ROLE=%s\n'                  "${z_role}"
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "${z_client_email}"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "${z_private_key}"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "${z_project_id}"
    printf 'RBRA_TOKEN_LIFETIME_SEC=1800\n'
  } > "${z_rbra_file}" || buc_die "Failed to write RBRA file ${z_rbra_file}"

  test -f "${z_rbra_file}" || buc_die "Failed to write RBRA file ${z_rbra_file}"

  # Decoded JSON lives in RBRR_SECRETS_DIR (not BURD_TEMP_DIR), so
  # BCG:518's no-module-temp-deletion rule does not bind. Remove now
  # that the bytes are persisted in RBRA form.
  rm -f "${z_key_json}"

  buc_info "RBRA file written: ${z_rbra_file}"

  # Echo uniqueId to stdout for callers (all logging goes to stderr per BCG)
  printf '%s' "${z_sa_uid}"
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
  rbuh_json "POST" "${RBGD_API_GCS_BUCKET_CREATE}" "${z_token}" \
                                  "${ZRBGG_INFIX_BUCKET_CREATE}" "${z_bucket_req}"
  z_code=$(rbuh_code_capture "${ZRBGG_INFIX_BUCKET_CREATE}") || buc_die "Bad bucket creation HTTP code"
  z_err=$(rbuh_json_field_capture "${ZRBGG_INFIX_BUCKET_CREATE}" '.error.message') || z_err="HTTP ${z_code}"

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
      z_tok_enc=$(rbuh_urlencode_capture "${z_page_token}") || return 1
      z_url="${z_url}?pageToken=${z_tok_enc}"
    fi

    buc_log_args 'Use a unique infix per page to avoid clobbering files'
    local z_infix="${ZRBGG_INFIX_BUCKET_LIST}${z_first}"
    rbuh_json "GET" "${z_url}" "${z_token}" "${z_infix}"

    local z_code
    z_code=$(rbuh_code_capture "${z_infix}") || return 1
    test "${z_code}" = "200" || return 1

    buc_log_args 'Print names from this page (if any)'
    buc_log_args 'Next page?'
    jq -r                '.items[]?.name // empty' "${ZRBUH_PREFIX}${z_infix}${ZRBUH_POSTFIX_JSON}"  || return 1
    z_page_token=$(rbuh_json_field_capture "${z_infix}" '.nextPageToken') || z_page_token=""

    test -n "${z_page_token}" || break
    z_first=$((z_first + 1))
  done
}

zrbgg_get_project_number_capture() {
  zrbgg_sentinel

  local z_token
  z_token=$(rba_get_governor_token_capture) || return 1

  rbuh_json "GET" "${RBGD_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGG_INFIX_PROJECT_INFO}"
  rbuh_require_ok "Get project info"                         "${ZRBGG_INFIX_PROJECT_INFO}" || return 1

  local z_project_number
  z_project_number=$(rbuh_json_field_capture "${ZRBGG_INFIX_PROJECT_INFO}" '.projectNumber') || return 1
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
    z_object_enc=$(rbuh_urlencode_capture "${z_object}") || z_object_enc=""
    test -n "${z_object_enc}" || { buc_warn "Failed to encode object name: ${z_object}"; continue; }
    z_delete_url="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}/o/${z_object_enc}"

    rbuh_json "DELETE" "${z_delete_url}" \
                              "${z_token}" "${ZRBGG_INFIX_OBJECT_DELETE}"
    z_delete_code=$(rbuh_code_capture "${ZRBGG_INFIX_OBJECT_DELETE}") || z_delete_code=""
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
  rbuh_json "DELETE" "${z_delete_url}" \
                      "${z_token}" "${ZRBGG_INFIX_BUCKET_DELETE}"
  z_code=$(rbuh_code_capture "${ZRBGG_INFIX_BUCKET_DELETE}") || z_code=""
  z_err=$(rbuh_json_field_capture "${ZRBGG_INFIX_BUCKET_DELETE}" '.error.message') || z_err="HTTP ${z_code}"
  case "${z_code}" in
    204) buc_info "Bucket ${z_bucket_name} deleted";                           return 0 ;;
    404) buc_warn "Bucket ${z_bucket_name} not found (already deleted)";       return 0 ;;
    409) buc_warn "Bucket ${z_bucket_name} not empty or has retention policy"; return 1 ;;
    *)   buc_warn "Bucket ${z_bucket_name} failed delete";                     return 1 ;;
  esac
}

######################################################################
# External Functions (rbgg_*)

# Roster a single role: paginate the SA list, filter by role-prefix,
# emit per-identity fact files (basename = identity, content = full email)
# and a human-readable line per match. Used by rbgg_roster_retrievers /
# rbgg_roster_directors via $1 = role tinder, $2 = fact-extension tinder.
zrbgg_roster_role() {
  zrbgg_sentinel

  local -r z_role="$1"
  local -r z_fact_ext="$2"

  buc_step "Rostering ${z_role} service accounts in project: ${RBDC_DEPOT_PROJECT_ID}"

  buc_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rba_get_governor_token_capture) || buc_die "Failed to get admin token (rc=$?)"

  local z_url_base="${RBGD_API_SERVICE_ACCOUNTS}"
  local z_page_token=""
  local z_page=1
  local z_count=0

  while :; do
    local z_url="${z_url_base}"
    if test -n "${z_page_token}"; then
      local z_tok_enc
      z_tok_enc=$(rbuh_urlencode_capture "${z_page_token}") \
        || buc_die "Failed to URL-encode pageToken"
      z_url="${z_url}?pageToken=${z_tok_enc}"
    fi

    local z_infix="${ZRBGG_INFIX_ROSTER}_${z_role}_${z_page}"
    rbuh_json "GET" "${z_url}" "${z_token}" "${z_infix}"
    rbuh_require_ok "List service accounts (page ${z_page})" "${z_infix}"

    local z_sa_count
    z_sa_count=$(rbuh_json_field_capture "${z_infix}" '.accounts // [] | length') \
      || buc_die "Failed to parse service accounts page"

    local z_index=0
    while test "${z_index}" -lt "${z_sa_count}"; do
      local z_sa_email
      z_sa_email=$(rbuh_json_field_capture "${z_infix}" ".accounts[${z_index}].email") \
        || { z_index=$((z_index + 1)); continue; }
      if [[ "${z_sa_email}" =~ ^${z_role}-(.+)@${RBGD_SA_EMAIL_FULL}$ ]]; then
        local z_identity="${BASH_REMATCH[1]}"
        buf_write_fact_multi "${z_identity}" "${z_fact_ext}" "${z_sa_email}"
        buc_bare "  ${z_identity}  ${z_sa_email}"
        z_count=$((z_count + 1))
      fi
      z_index=$((z_index + 1))
    done

    z_page_token=$(rbuh_json_field_capture "${z_infix}" '.nextPageToken') || z_page_token=""
    test -n "${z_page_token}" || break
    z_page=$((z_page + 1))
  done

  buc_info "Found ${z_count} ${z_role} service account(s)"
  buc_success "Roster operation completed"
}

rbgg_roster_retrievers() {
  zrbgg_sentinel

  buc_doc_brief "Roster Retriever service accounts (emit per-identity fact files)"
  buc_doc_shown || return 0

  zrbgg_roster_role "${RBCC_account_retriever}" "${RBCC_fact_ext_roster_retriever}"
}

rbgg_roster_directors() {
  zrbgg_sentinel

  buc_doc_brief "Roster Director service accounts (emit per-identity fact files)"
  buc_doc_shown || return 0

  zrbgg_roster_role "${RBCC_account_director}" "${RBCC_fact_ext_roster_director}"
}

rbgg_invest_retriever() {
  zrbgg_sentinel

  local z_identity="${BUZ_FOLIO:-}"

  buc_doc_brief "Invest a Retriever service account for an identity"
  buc_doc_param "identity" "Identity (required) — composes ${RBCC_account_retriever}-<identity>"
  buc_doc_shown || return 0

  test -n "${z_identity}" || buc_die "Identity required"

  local z_account_name="${RBCC_account_retriever}-${z_identity}"
  local z_account_email="${z_account_name}@${RBGD_SA_EMAIL_FULL}"

  buc_step "Investing Retriever service account: ${z_account_name}"

  zrbgg_create_service_account_with_key                                          \
    "${z_account_name}"                                                        \
    "Recipe Bottle Retriever (${z_identity})"                                  \
    "Read-only access to Google Artifact Registry - identity: ${z_identity}"   \
    "${RBCC_role_retriever}" > /dev/null || buc_die "Failed to create Retriever SA"

  local z_token
  z_token=$(rba_get_governor_token_capture) || buc_die "Failed to get admin token"

  buc_step 'Adding Artifact Registry Reader role'
  rbgi_add_project_iam_role                 \
    "${z_token}"                            \
    "Grant Artifact Registry Reader"        \
    "${RBGD_PROJECT_RESOURCE}"              \
    "${RBGC_ROLE_ARTIFACTREGISTRY_READER}"  \
    "serviceAccount:${z_account_email}"     \
    "retriever-reader"

  buc_step 'Adding Container Analysis Occurrences Viewer role'
  rbgi_add_project_iam_role                              \
    "${z_token}"                                         \
    "Grant Container Analysis Occurrences Viewer"        \
    "${RBGD_PROJECT_RESOURCE}"                           \
    "${RBGC_ROLE_CONTAINERANALYSIS_OCCURRENCES_VIEWER}"  \
    "serviceAccount:${z_account_email}"                  \
    "retriever-analysis"

  buc_info "RBRA file written: ${RBDC_ASSAY_RBRA_FILE}"
  buc_info ""
  buc_info "Move the RBRA file to a safe place — typically delivered to the"
  buc_info "operator who consumes this identity. When the invester is also the"
  buc_info "consumer, the local role slot is the canonical destination:"
  buc_bare "        mv ${RBDC_ASSAY_RBRA_FILE} ${RBDC_RETRIEVER_RBRA_FILE}"
}

rbgg_invest_director() {
  zrbgg_sentinel

  local z_identity="${BUZ_FOLIO:-}"

  buc_doc_brief "Invest a Director service account for an identity"
  buc_doc_param "identity" "Identity (required) — composes ${RBCC_account_director}-<identity>"
  buc_doc_shown || return 0

  test -n "${z_identity}" || buc_die "Identity required"

  local z_account_name="${RBCC_account_director}-${z_identity}"
  local z_account_email="${z_account_name}@${RBGD_SA_EMAIL_FULL}"

  buc_step "Investing Director service account: ${z_account_name}"

  zrbgg_create_service_account_with_key                      \
    "${z_account_name}"                                    \
    "Recipe Bottle Director (${z_identity})"               \
    "Create/destroy container images for ${z_identity}"    \
    "${RBCC_role_director}" > /dev/null || buc_die "Failed to create Director SA"

  buc_step 'Get OAuth token from admin'
  local z_token
  z_token=$(rba_get_governor_token_capture) || buc_die "Failed to get admin token"

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

  rbgi_add_project_iam_role                 \
    "${z_token}"                            \
    "Grant Worker Pool User"               \
    "${RBGD_PROJECT_RESOURCE}"              \
    "roles/cloudbuild.workerPoolUser"       \
    "serviceAccount:${z_account_email}"     \
    "director-pool"

  buc_step 'Grant serviceAccountUser on Mason'
  rbgi_add_sa_iam_role "${z_token}" "${RBGD_MASON_EMAIL}" "${z_account_email}" "roles/iam.serviceAccountUser"

  buc_step 'Grant Artifact Registry roles (complete expected policy)'
  # Complete policy: Director repoAdmin + Mason writer in one setIamPolicy.
  # Prevents read-modify-write race where stale getIamPolicy omits Mason's binding.
  local -r z_gar_resource="projects/${RBGD_GAR_PROJECT_ID}/locations/${RBGD_GAR_LOCATION}/repositories/${RBDC_GAR_REPOSITORY}"
  local -r z_gar_get_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_gar_resource}:getIamPolicy?options.requestedPolicyVersion=3"
  local -r z_gar_set_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_gar_resource}:setIamPolicy"

  # Propagation retry — AR repo is resource-scope: member-visibility 400s plus
  # caller-recently-empowered 403 from the resource-scope IAM cache (the
  # governor's roles/owner grant may not yet have reached the AR cache).
  local -ra z_gar_tolerance=(
    "400" "*does not exist*"
    "400" "*is not deleted*"
    "403" ""
  )
  local z_gar_prop_delay=${RBGC_PROPAGATION_INITIAL_DELAY_SEC}
  local z_gar_prop_elapsed=0
  local -r z_gar_prop_deadline=${RBGC_PROPAGATION_DEADLINE_SEC}
  local z_gar_prop_attempt=0
  local z_gar_get_infix=""
  local z_gar_set_infix=""
  local z_gar_get_code=""

  while :; do
    z_gar_prop_attempt=$((z_gar_prop_attempt + 1))
    z_gar_get_infix="director_gar_get_iam-${z_gar_prop_elapsed}s"
    z_gar_set_infix="director_gar_set_iam-${z_gar_prop_elapsed}s"

    rbuh_json "GET" "${z_gar_get_url}" "${z_token}" "${z_gar_get_infix}"
    z_gar_get_code=$(rbuh_code_capture "${z_gar_get_infix}") || z_gar_get_code=""

    # Propagation retry on GET — covers newly-empowered governor (403) and
    # newly-created Director SA member-visibility lag (400 patterns).
    if zrbgi_propagation_error_predicate "${z_gar_get_infix}" "${z_gar_get_code}" "${z_gar_tolerance[@]}"; then
      test "${z_gar_prop_elapsed}" -lt "${z_gar_prop_deadline}" \
        || buc_die "GAR IAM: propagation timeout after ${z_gar_prop_elapsed}s"
      buc_log_args "GAR getIamPolicy returned ${z_gar_get_code} (propagation delay; attempt ${z_gar_prop_attempt}, ${z_gar_prop_elapsed}s)"
      sleep "${z_gar_prop_delay}"
      z_gar_prop_elapsed=$((z_gar_prop_elapsed + z_gar_prop_delay))
      z_gar_prop_delay=$((z_gar_prop_delay * 2))
      test "${z_gar_prop_delay}" -le "${RBGC_PROPAGATION_MAX_DELAY_SEC}" || z_gar_prop_delay=${RBGC_PROPAGATION_MAX_DELAY_SEC}
      continue
    fi

    rbuh_require_ok "Get GAR repo IAM policy" "${z_gar_get_infix}"

    # Build complete expected policy: Director repoAdmin + Mason writer
    local z_gar_partial
    z_gar_partial=$(rbgi_jq_add_member_to_role_capture "${z_gar_get_infix}" \
      "roles/artifactregistry.repoAdmin" "serviceAccount:${z_account_email}" "") \
      || buc_die "Failed to add Director repoAdmin to GAR IAM policy"

    local z_gar_intermediate="${BURD_TEMP_DIR}/rbuh_director_gar_complete_iam_u_resp.json"
    printf '%s\n' "${z_gar_partial}" > "${z_gar_intermediate}" \
      || buc_die "Failed to write intermediate GAR IAM policy"

    local z_gar_complete
    z_gar_complete=$(rbgi_jq_add_member_to_role_capture "director_gar_complete_iam" \
      "roles/artifactregistry.writer" "serviceAccount:${RBGD_MASON_EMAIL}" "") \
      || buc_die "Failed to add Mason writer to GAR IAM policy"

    local z_gar_set_body="${BURD_TEMP_DIR}/rbgg_gar_complete_policy_body.json"
    printf '{"policy":%s}\n' "${z_gar_complete}" > "${z_gar_set_body}" \
      || buc_die "Failed to write GAR setIamPolicy body"
    rbuh_json "POST" "${z_gar_set_url}" "${z_token}" "${z_gar_set_infix}" "${z_gar_set_body}"

    local z_gar_set_code
    z_gar_set_code=$(rbuh_code_capture "${z_gar_set_infix}") || buc_die "No HTTP code from GAR setIamPolicy"

    # Propagation retry on SET — same tolerance list as GET.
    if zrbgi_propagation_error_predicate "${z_gar_set_infix}" "${z_gar_set_code}" "${z_gar_tolerance[@]}"; then
      test "${z_gar_prop_elapsed}" -lt "${z_gar_prop_deadline}" \
        || buc_die "GAR IAM: propagation timeout after ${z_gar_prop_elapsed}s"
      buc_log_args "GAR setIamPolicy returned ${z_gar_set_code} (propagation delay; attempt ${z_gar_prop_attempt}, ${z_gar_prop_elapsed}s)"
      sleep "${z_gar_prop_delay}"
      z_gar_prop_elapsed=$((z_gar_prop_elapsed + z_gar_prop_delay))
      z_gar_prop_delay=$((z_gar_prop_delay * 2))
      test "${z_gar_prop_delay}" -le "${RBGC_PROPAGATION_MAX_DELAY_SEC}" || z_gar_prop_delay=${RBGC_PROPAGATION_MAX_DELAY_SEC}
      continue
    fi

    rbuh_require_ok "Set GAR repo IAM policy (complete)" "${z_gar_set_infix}"
    break
  done

  buc_info "RBRA file written: ${RBDC_ASSAY_RBRA_FILE}"
  buc_info ""
  buc_info "Move the RBRA file to a safe place — typically delivered to the"
  buc_info "operator who consumes this identity. When the invester is also the"
  buc_info "consumer, the local role slot is the canonical destination:"
  buc_bare "        mv ${RBDC_ASSAY_RBRA_FILE} ${RBDC_DIRECTOR_RBRA_FILE}"
}

# Divest a single role+identity: synthesize email, DELETE (404-tolerant),
# opportunistic role-RBRA cleanup only on email match.
zrbgg_divest_role() {
  zrbgg_sentinel

  local -r z_role="$1"
  local -r z_identity="$2"
  local -r z_role_rbra_file="$3"

  local z_account_email="${z_role}-${z_identity}@${RBGD_SA_EMAIL_FULL}"

  buc_step "Divesting service account: ${z_account_email}"

  buc_log_args 'Get OAuth token from admin'
  local z_token
  z_token=$(rba_get_governor_token_capture) || buc_die "Failed to get admin token"

  buc_log_args 'Delete via REST API (404-tolerant)'
  rbuh_json "DELETE" "${RBGD_API_SERVICE_ACCOUNTS}/${z_account_email}" "${z_token}" \
                                                 "${ZRBGG_INFIX_DELETE}"
  rbuh_require_ok "Delete service account" "${ZRBGG_INFIX_DELETE}" \
    404 "not found (already deleted)"

  buc_log_args 'Confirm deletion propagated before any same-name recreate'
  # When the DELETE actually removed an SA, wait until the GET path reports it
  # gone — a delete-accepted response precedes propagation, and an immediately
  # following same-name invest would otherwise race a still-visible account.
  # Skipped on 404: the account was already absent, nothing to wait for.
  local z_delete_code
  z_delete_code=$(rbuh_code_capture "${ZRBGG_INFIX_DELETE}") || z_delete_code=""
  if test "${z_delete_code}" != "404"; then
    rbuh_poll_until_gone "Service account ${z_account_email}" \
      "${RBGD_API_SERVICE_ACCOUNTS}/${z_account_email}" "${z_token}" "${ZRBGG_INFIX_DELETE_GONE}"
  fi

  buc_log_args 'Opportunistic role-RBRA cleanup if installed credential matches'
  if test -f "${z_role_rbra_file}"; then
    local z_installed_email
    z_installed_email=$(. "${z_role_rbra_file}" 2>/dev/null && printf '%s' "${RBRA_CLIENT_EMAIL:-}") \
      || z_installed_email=""
    if test "${z_installed_email}" = "${z_account_email}"; then
      rm -f "${z_role_rbra_file}"
      buc_info "Removed installed RBRA at ${z_role_rbra_file} (matched divested email)"
    fi
  fi

  buc_success "Divest operation completed"
}

rbgg_divest_retriever() {
  zrbgg_sentinel

  local z_identity="${BUZ_FOLIO:-}"

  buc_doc_brief "Divest a Retriever service account by identity"
  buc_doc_param "identity" "Identity (required) — composes ${RBCC_account_retriever}-<identity>"
  buc_doc_shown || return 0

  test -n "${z_identity}" || buc_die "Identity required"

  zrbgg_divest_role "${RBCC_account_retriever}" "${z_identity}" "${RBDC_RETRIEVER_RBRA_FILE}"
}

rbgg_divest_director() {
  zrbgg_sentinel

  local z_identity="${BUZ_FOLIO:-}"

  buc_doc_brief "Divest a Director service account by identity"
  buc_doc_param "identity" "Identity (required) — composes ${RBCC_account_director}-<identity>"
  buc_doc_shown || return 0

  test -n "${z_identity}" || buc_die "Identity required"

  zrbgg_divest_role "${RBCC_account_director}" "${z_identity}" "${RBDC_DIRECTOR_RBRA_FILE}"
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
  z_token=$(rba_get_governor_token_capture) || buc_die "Failed to get admin token"

  buc_step 'Triple confirmation required'
  buc_warn ""
  buc_warn "========================================================================"
  buc_warn "CRITICAL WARNING: You are about to PERMANENTLY DELETE the entire project:"
  buc_warn "  Project: ${RBDC_DEPOT_PROJECT_ID}"
  buc_warn "This will:"
  buc_warn "  - Delete ALL resources in the project"
  buc_warn "  - Delete ALL data permanently"
  buc_warn "  - Break billing associations"
  buc_warn "  - Make the project unusable immediately"
  buc_warn "  - Cannot be undone after 30-day grace period"
  buc_warn "========================================================================"
  buc_warn ""

  buc_require "Type the exact project ID to confirm deletion" "${RBDC_DEPOT_PROJECT_ID}"
  buc_require "Confirm you understand this DELETES EVERYTHING in the project" "DELETE-EVERYTHING"
  buc_require "Final confirmation - type OBLITERATE to proceed" "OBLITERATE"

  buc_step 'Check for liens (will block deletion)'
  rbuh_json "GET" "${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens?parent=projects/${RBDC_DEPOT_PROJECT_ID}" "${z_token}" "${ZRBGG_INFIX_LIST_LIENS}"
  rbuh_require_ok "List liens" "${ZRBGG_INFIX_LIST_LIENS}"

  local z_lien_count
  z_lien_count=$(rbuh_json_field_capture "${ZRBGG_INFIX_LIST_LIENS}" '.liens // [] | length') || buc_die "Failed to parse liens response"

  if [[ "${z_lien_count}" -gt 0 ]]; then
    buc_step 'BLOCKED: Liens exist on project'
    buc_warn "Project has ${z_lien_count} lien(s) that prevent deletion"
    buc_warn "You must remove all liens first:"
    buc_code "  gcloud resource-manager liens list --project=${RBDC_DEPOT_PROJECT_ID}"
    buc_code "  gcloud resource-manager liens delete LIEN_NAME --project=${RBDC_DEPOT_PROJECT_ID}"
    buc_warn "Then re-run this command."
    buc_die "Cannot proceed with active liens"
  fi

  buc_step 'Delete project (immediate lifecycle change to DELETE_REQUESTED)'
  rbuh_json "DELETE" "${RBGD_API_CRM_DELETE_PROJECT}" "${z_token}" "${ZRBGG_INFIX_PROJECT_DELETE}"
  rbuh_require_ok "Delete project" "${ZRBGG_INFIX_PROJECT_DELETE}"

  buc_step 'Verify deletion state'
  rbuh_json "GET" "${RBGD_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGG_INFIX_PROJECT_STATE}"
  rbuh_require_ok "Get project state" "${ZRBGG_INFIX_PROJECT_STATE}"

  local z_lifecycle_state
  z_lifecycle_state=$(rbuh_json_field_capture "${ZRBGG_INFIX_PROJECT_STATE}" '.lifecycleState // "UNKNOWN"') || buc_die "Failed to parse project state"

  if test "${z_lifecycle_state}" = "DELETE_REQUESTED"; then
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
  z_token=$(rba_get_governor_token_capture) || buc_die "Failed to get admin token"

  buc_step 'Check current project state'
  rbuh_json "GET" "${RBGD_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGG_INFIX_PROJECT_STATE}"

  if ! rbuh_code_ok_predicate "${ZRBGG_INFIX_PROJECT_STATE}"; then
    buc_die "Cannot access project - it may have been permanently deleted or never existed"
  fi

  local z_lifecycle_state
  z_lifecycle_state=$(rbuh_json_field_capture "${ZRBGG_INFIX_PROJECT_STATE}" '.lifecycleState // "UNKNOWN"') || buc_die "Failed to parse project state"

  if test "${z_lifecycle_state}" != "DELETE_REQUESTED"; then
    buc_die "Project state is ${z_lifecycle_state} - can only restore projects in DELETE_REQUESTED state"
  fi

  buc_step 'Confirm restoration'
  buc_log_args "Project Status: ${z_lifecycle_state}"
  buc_log_args "Attempting to restore project: ${RBDC_DEPOT_PROJECT_ID}"
  buc_log_args "WARNING: Restore may fail if deletion process has already started"
  buc_require "Confirm restoration of project" "RESTORE"

  buc_step 'Attempt project restoration'
  rbuh_json "POST" "${RBGD_API_CRM_UNDELETE_PROJECT}" "${z_token}" "${ZRBGG_INFIX_PROJECT_RESTORE}"
  if rbuh_code_ok_predicate                                                    "${ZRBGG_INFIX_PROJECT_RESTORE}"; then
    buc_step 'Verify restoration'
    rbuh_json "GET" "${RBGD_API_CRM_GET_PROJECT}" "${z_token}" "${ZRBGG_INFIX_PROJECT_STATE}"
    rbuh_require_ok "Get restored project state"               "${ZRBGG_INFIX_PROJECT_STATE}"

    z_lifecycle_state=$(rbuh_json_field_capture "${ZRBGG_INFIX_PROJECT_STATE}" '.lifecycleState // "UNKNOWN"') || buc_die "Failed to parse restored project state"

    if test "${z_lifecycle_state}" = "ACTIVE"; then
      buc_success "Project successfully restored to ACTIVE state"
      buc_log_args "Project Status: ${z_lifecycle_state}"
      buc_log_args "Project is now usable again"
    else
      buc_die "Restoration completed but project state is unexpected: ${z_lifecycle_state}"
    fi
  else
    local z_error_msg
    z_error_msg=$(rbuh_json_field_capture "${ZRBGG_INFIX_PROJECT_RESTORE}" '.error.message // "Unknown error"') || z_error_msg="Failed to parse error"
    buc_die "Project restoration failed: ${z_error_msg}"
  fi
}

# eof

