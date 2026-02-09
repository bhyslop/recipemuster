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
# Recipe Bottle Google IAM - Implementation

# ----------------------------------------------------------------------
# Operational Invariants (RBGI is single writer; 409 is fatal)
#
# - Single admin actor: All RBGI operations are executed by a single admin
#   identity. There are no concurrent writers in the same project.
# - Pristine-state expectation: RBGI init/creation flows assume the project
#   is pristine for the resources they manage. If a resource "already exists"
#   (HTTP 409), that's treated as state drift or prior manual activity.
# - Policy: All HTTP 409 Conflict responses are fatal (buc_die). We do not
#   treat 409 as idempotent success anywhere in RBGI.
#   If you see a 409, resolve state drift first (destroy/reset), then rerun.
# ----------------------------------------------------------------------

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGI_SOURCED:-}" || buc_die "Module rbgi multiply sourced - check sourcing hierarchy"
ZRBGI_SOURCED=1

######################################################################
# Internal Functions (zrbgi_*)

zrbgi_kindle() {
  test -z "${ZRBGI_KINDLED:-}" || buc_die "Module rbgi already kindled"

  # Validate dependencies
  buv_dir_exists "${BURD_TEMP_DIR}"

  # Ensure dependencies are kindled
  zrbgc_sentinel
  zrbgu_sentinel

  # Module prefix for temp files
  ZRBGI_PREFIX="${BURD_TEMP_DIR}/rbgi_"
  ZRBGI_EMPTY_JSON="${ZRBGI_PREFIX}empty.json"
  printf '{}' > "${ZRBGI_EMPTY_JSON}"

  # Infix values for IAM operations
  ZRBGI_INFIX_ROLE="role"
  ZRBGI_INFIX_ROLE_SET="role_set"
  ZRBGI_INFIX_REPO_ROLE="repo_role"
  ZRBGI_INFIX_REPO_ROLE_SET="repo_role_set"
  ZRBGI_INFIX_SA_IAM_VERIFY="sa_iamverify"
  ZRBGI_INFIX_REPO_POLICY="repo_policy"
  ZRBGI_INFIX_RPOLICY_SET="repo_policy_set"
  ZRBGI_INFIX_BUCKET_IAM="bucket_iam"
  ZRBGI_INFIX_BUCKET_IAM_SET="bucket_iam_set"

  ZRBGI_POSTFIX_JSON="_i_resp.json"

  ZRBGI_KINDLED=1
}

zrbgi_sentinel() {
  test "${ZRBGI_KINDLED:-}" = "1" || buc_die "Module rbgi not kindled - call zrbgi_kindle first"
}

######################################################################
# External Functions (rbgi_*)

# Add a project-scoped IAM role binding with optimistic concurrency and strong read-back.
rbgi_add_project_iam_role() {
  zrbgi_sentinel

  local z_token="${1:-}"
  local z_label="${2:-}"
  local z_resource="${3:-}"  # resource_base: Base resource URL
  local z_role="${4:-}"
  local z_member="${5:-}"
  local z_parent_infix="${6:-}"

  test -n "${z_token}" || buc_die "Token required"
  buc_log_args "Using admin token (value not logged)"

  local z_resource_path="${z_resource#/}"  # strip leading slash if present
  local z_base="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/${z_resource_path}"
  local z_get_url="${z_base}${RBGC_CRM_GET_IAM_POLICY_SUFFIX}"
  local z_set_url="${z_base}${RBGC_CRM_SET_IAM_POLICY_SUFFIX}"

  test -n "${z_resource}" || buc_die "resource required"
  test -n "${z_role}"     || buc_die "role required"
  test -n "${z_member}"   || buc_die "member required"

  buc_log_args "${z_label}: add ${z_member} to ${z_role}"

  buc_log_args '1) GET policy (v3)'
  buc_log_args "GET_POLICY_URL_DEBUG z_resource:${z_resource} z_get_url:${z_get_url}"
  local z_get_body="${ZRBGI_PREFIX}${z_parent_infix}_get_body.json"
  local z_get_infix="${z_parent_infix}-get"
  printf '%s\n' '{"options":{"requestedPolicyVersion":3}}' > "${z_get_body}"
  rbgu_http_json_ok "${z_label} (get policy)" "${z_token}" "POST" \
    "${z_get_url}" "${z_get_infix}" "${z_get_body}"

  buc_log_args 'Extract etag; require non-empty'
  local z_etag=""
  z_etag=$(rbgu_json_field_capture "${z_get_infix}" ".etag") || buc_die "Missing etag"
  test -n "${z_etag}" || buc_die "Empty etag"

  buc_log_args "Using etag ${z_etag}"

  buc_log_args '2) Build new policy JSON in temp (bindings unique; version=3; keep etag)'
  local z_new_policy_json=""
  z_new_policy_json=$(rbgu_jq_add_member_to_role_capture "${z_get_infix}" "${z_role}" "${z_member}" "${z_etag}") \
    || buc_die "Failed to compose policy JSON"

  local z_set_body="${ZRBGI_PREFIX}${z_parent_infix}_set_body.json"
  printf '{"policy":%s}\n' "${z_new_policy_json}" > "${z_set_body}"

  buc_log_args '3) setIamPolicy (fatal on 409/412 by policy)'
  local z_elapsed=0
  local z_set_infix=""
  while :; do
    z_set_infix="${z_parent_infix}-set-${z_elapsed}s"
    rbgu_http_json "POST" "${z_set_url}" "${z_token}" "${z_set_infix}" "${z_set_body}"

    local z_code=""
    z_code=$(rbgu_http_code_capture "${z_set_infix}") || buc_die "No HTTP code"
    case "${z_code}" in
      200)                 break ;;
      412)                 buc_die "${z_label}: precondition failed (etag mismatch)"    ;;
      429|500|502|503|504) buc_log_args "Transient ${z_code} at ${z_elapsed}s; retry"   ;;
      409)                 buc_die "${z_label}: HTTP 409 Conflict (fatal by invariant)" ;;
      *)                   rbgu_http_require_ok "${z_label} (set policy)" "${z_set_infix}" "" ;;
    esac

    test "${z_elapsed}" -ge "${RBGC_MAX_CONSISTENCY_SEC}" && buc_die "${z_label}: timeout setting policy"
    sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
    z_elapsed=$((z_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
  done

  buc_log_args '4) Verify membership within bounded wait'
  z_elapsed=0
  while :; do
    local z_verify_infix="${z_parent_infix}-verify-${z_elapsed}s"
    rbgu_http_json_ok "${z_label} (verify)" "${z_token}" "POST" \
                       "${z_get_url}" "${z_verify_infix}" "${z_get_body}"

    if rbgu_role_member_exists_predicate "${z_verify_infix}" "${z_role}" "${z_member}"; then
      buc_log_args "Observed ${z_role} for ${z_member}"

      buc_log_args "Post-set etag $(rbgu_json_field_capture "${z_verify_infix}" ".etag" 2>/dev/null || echo "")"

      return 0
    fi

    test "${z_elapsed}" -ge "${RBGC_MAX_CONSISTENCY_SEC}" && buc_die "${z_label}: verify timeout"
    sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
    z_elapsed=$((z_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
  done
}

rbgi_add_repo_iam_role() {
  zrbgi_sentinel

  local z_token="${1:-}"
  local z_project_id="${2:-}"
  local z_account_email="${3:-}"
  local z_location="${4:-}"
  local z_repository="${5:-}"
  local z_role="${6:-}"

  test -n "${z_token}"         || buc_die "Token required"
  test -n "${z_project_id}"    || buc_die "Project ID required"
  test -n "${z_account_email}" || buc_die "Service account email required"
  test -n "${z_location}"      || buc_die "Location is required"
  test -n "${z_repository}"    || buc_die "Repository is required"
  test -n "${z_role}"          || buc_die "Role is required"

  buc_log_args "Using admin token (value not logged)"

  local z_resource="projects/${z_project_id}/locations/${z_location}/repositories/${z_repository}"
  local z_get_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:getIamPolicy"
  local z_set_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:setIamPolicy"

  buc_log_args 'Adding repo-scoped IAM role' \
               " ${z_role} to ${z_account_email} on ${z_location}/${z_repository}"

  buc_log_args 'Get current repo IAM policy'
  rbgu_http_json "POST" "${z_get_url}" "${z_token}" \
                                      "${ZRBGI_INFIX_REPO_ROLE}" "${ZRBGI_EMPTY_JSON}"

  local z_get_code
  z_get_code=$(rbgu_http_code_capture "${ZRBGI_INFIX_REPO_ROLE}") || z_get_code=""
  if test "${z_get_code}" = "404"; then
    # 404 means repo exists but has no IAM policy yet - this is normal for new repos
    buc_log_args 'No IAM policy exists yet (404), initializing with empty bindings'
    rbgu_write_vanilla_json "${ZRBGI_INFIX_REPO_ROLE}"
  else
    rbgu_http_require_ok "Get repo IAM policy" "${ZRBGI_INFIX_REPO_ROLE}"
  fi

  buc_log_args 'Update repo IAM policy'
  local z_updated_policy_json=""
  z_updated_policy_json=$(rbgu_jq_add_member_to_role_capture "${ZRBGI_INFIX_REPO_ROLE}" \
    "${z_role}" "serviceAccount:${z_account_email}" "") \
    || buc_die "Failed to update policy JSON"

  buc_log_args 'Set updated repo IAM policy'
  local z_repo_set_body="${BURD_TEMP_DIR}/rbgi_repo_set_policy_body.json"
  printf '{"policy":%s}\n' "${z_updated_policy_json}" > "${z_repo_set_body}" \
    || buc_die "Failed to build repo setIamPolicy body"
  rbgu_http_json "POST" "${z_set_url}" "${z_token}" \
                                             "${ZRBGI_INFIX_REPO_ROLE_SET}" "${z_repo_set_body}"
  rbgu_http_require_ok "Set repo IAM policy" "${ZRBGI_INFIX_REPO_ROLE_SET}"

  buc_log_args 'Successfully added repo-scoped role' "${z_role}"
}

rbgi_add_sa_iam_role() {
  zrbgi_sentinel

  local z_token="${1:-}"
  local z_target_sa_email="${2:-}"
  local z_member_email="${3:-}"  # email only; function adds serviceAccount: prefix
  local z_role="${4:-}"

  test -n "${z_token}" || buc_die "Token required"

  buc_log_args "Using admin token (value not logged)"
  buc_log_args "Granting ${z_role} on SA ${z_target_sa_email} to ${z_member_email}"

  # Caller must have already primed Cloud Build if this is the runtime SA.
  # We do a hard existence check and crash if not accessible.

  buc_log_args 'Verify target SA exists'
  local z_target_encoded
  z_target_encoded=$(rbgu_urlencode_capture "${z_target_sa_email}") \
    || buc_die "Failed to encode SA email"

  local z_verify_code
  rbgu_http_json "GET" \
    "${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/-/serviceAccounts/${z_target_encoded}" \
                            "${z_token}" "${ZRBGI_INFIX_SA_IAM_VERIFY}"
  z_verify_code=$(rbgu_http_code_capture "${ZRBGI_INFIX_SA_IAM_VERIFY}") || z_verify_code=""
  test "${z_verify_code}" = "200" || \
    buc_die "Target service account not accessible: ${z_target_sa_email} (HTTP ${z_verify_code})"

  local z_sa_resource="${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/-/serviceAccounts/${z_target_encoded}"

  buc_log_args 'Get current SA IAM policy'
  rbgu_http_json "POST" "${z_sa_resource}:getIamPolicy" "${z_token}" \
    "${ZRBGI_INFIX_ROLE}" "${ZRBGI_EMPTY_JSON}"

  local z_code
  z_code=$(rbgu_http_code_capture "${ZRBGI_INFIX_ROLE}") || z_code=""
  if test "${z_code}" != "200"; then
    buc_log_args 'No IAM policy exists yet, initializing'
    rbgu_write_vanilla_json "${ZRBGI_INFIX_ROLE}"
  fi

  buc_log_args 'Update SA IAM policy with new role binding'
  local z_updated_policy_json=""
  z_updated_policy_json=$(rbgu_jq_add_member_to_role_capture "${ZRBGI_INFIX_ROLE}" \
    "${z_role}" "serviceAccount:${z_member_email}" "") \
    || buc_die "Failed to update SA IAM policy"

  buc_log_args 'Set updated SA IAM policy'
  local z_set_body="${BURD_TEMP_DIR}/rbgi_sa_set_policy_body.json"
  printf '{"policy":%s}\n' "${z_updated_policy_json}" > "${z_set_body}" \
    || buc_die "Failed to build SA setIamPolicy body"

  rbgu_http_json "POST" "${z_sa_resource}:setIamPolicy" "${z_token}" \
                                           "${ZRBGI_INFIX_ROLE_SET}" "${z_set_body}"
  rbgu_http_require_ok "Set SA IAM policy" "${ZRBGI_INFIX_ROLE_SET}"

  buc_log_args 'Successfully granted SA role' "${z_role}"
}

rbgi_add_bucket_iam_role() {
  zrbgi_sentinel

  local z_token="${1:-}"
  local z_bucket_name="${2:-}"
  local z_account_email="${3:-}"
  local z_role="${4:-}"

  test -n "${z_token}" || buc_die "Token required"

  buc_log_args "Using admin token (value not logged)"
  buc_log_args "Adding bucket IAM role ${z_role} to ${z_account_email}"

  local z_code

  buc_log_args 'Get current bucket IAM policy'
  local z_iam_url="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}/iam"
  rbgu_http_json "GET" "${z_iam_url}" "${z_token}" "${ZRBGI_INFIX_BUCKET_IAM}"
  z_code=$(rbgu_http_code_capture                  "${ZRBGI_INFIX_BUCKET_IAM}") || z_code=""
  if test "${z_code}" != "200"; then
    buc_log_args 'Initialize empty IAM policy for bucket'
    rbgu_write_vanilla_json "${ZRBGI_INFIX_BUCKET_IAM}"
  fi

  buc_log_args 'Update bucket IAM policy'
  local z_etag
  z_etag=$(rbgu_json_field_capture "${ZRBGI_INFIX_BUCKET_IAM}" '.etag') || z_etag=""
  local z_updated_policy_json=""
  z_updated_policy_json=$(rbgu_jq_add_member_to_role_capture "${ZRBGI_INFIX_BUCKET_IAM}" \
    "${z_role}" "serviceAccount:${z_account_email}" "${z_etag}") \
    || buc_die "Failed to update bucket IAM policy"

  buc_log_args 'Set updated bucket IAM policy'
  local z_bucket_set_body="${BURD_TEMP_DIR}/rbgi_bucket_set_policy_body.json"
  printf '%s\n' "${z_updated_policy_json}" > "${z_bucket_set_body}" \
    || buc_die "Failed to write bucket policy body"

  local z_elapsed=0
  local z_set_infix=""
  while :; do
    z_set_infix="${ZRBGI_INFIX_BUCKET_IAM_SET}-${z_elapsed}s"
    rbgu_http_json "PUT" "${z_iam_url}" "${z_token}" "${z_set_infix}" "${z_bucket_set_body}"

    z_code=$(rbgu_http_code_capture "${z_set_infix}") || buc_die "No HTTP code"
    case "${z_code}" in
      200) break ;;
      400)
        local z_err_msg
        z_err_msg=$(rbgu_error_message_capture "${z_set_infix}") || z_err_msg=""
        case "${z_err_msg}" in
          *"does not exist"*) buc_log_args "SA not yet visible to GCS (HTTP 400), waiting ${RBGC_EVENTUAL_CONSISTENCY_SEC}s..." ;;
          *) buc_die "Set bucket IAM policy (HTTP 400): ${z_err_msg}" ;;
        esac
        ;;
      429|500|502|503|504) buc_log_args "Transient ${z_code} at ${z_elapsed}s; retry" ;;
      *) rbgu_http_require_ok "Set bucket IAM policy" "${z_set_infix}" ;;
    esac

    test "${z_elapsed}" -ge "${RBGC_MAX_CONSISTENCY_SEC}" && buc_die "Set bucket IAM policy: timeout after ${RBGC_MAX_CONSISTENCY_SEC}s"
    sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
    z_elapsed=$((z_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
  done

  buc_log_args "Successfully added bucket role ${z_role}"
}

# eof

