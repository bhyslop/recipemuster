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
# - Policy: All HTTP 409 Conflict responses are fatal (bcu_die). We do not
#   treat 409 as idempotent success anywhere in RBGI.
#   If you see a 409, resolve state drift first (destroy/reset), then rerun.
# ----------------------------------------------------------------------

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGI_SOURCED:-}" || bcu_die "Module rbgi multiply sourced - check sourcing hierarchy"
ZRBGI_SOURCED=1

######################################################################
# Internal Functions (zrbgi_*)

zrbgi_kindle() {
  test -z "${ZRBGI_KINDLED:-}" || bcu_die "Module rbgi already kindled"

  # Validate dependencies
  bvu_dir_exists "${BDU_TEMP_DIR}"
  
  # Ensure dependencies are kindled
  zrbgc_sentinel
  zrbgu_sentinel

  # Module prefix for temp files
  ZRBGI_PREFIX="${BDU_TEMP_DIR}/rbgi_"
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
  test "${ZRBGI_KINDLED:-}" = "1" || bcu_die "Module rbgi not kindled - call zrbgi_kindle first"
}

######################################################################
# External Functions (rbgi_*)

# Add a project-scoped IAM role binding with optimistic concurrency and strong read-back.
rbgi_add_project_iam_role() {
  zrbgi_sentinel
  
  local z_label="${1:-}"
  local z_token="${2:-}"
  local z_resource="${3:-}"  # resource_base: Base resource URL
  local z_role="${4:-}"
  local z_member="${5:-}"
  local z_parent_infix="${6:-newrole}"
  local z_get_url="${z_resource}:getIamPolicy"
  local z_set_url="${z_resource}:setIamPolicy"

  test -n "${z_token}"    || bcu_die "token required"
  test -n "${z_resource}" || bcu_die "resource required"
  test -n "${z_role}"     || bcu_die "role required"
  test -n "${z_member}"   || bcu_die "member required"

  bcu_log_args "${z_label}: add ${z_member} to ${z_role}"

  bcu_log_args '1) GET policy (v3)'
  local z_get_body="${ZRBGI_PREFIX}${z_parent_infix}_get_body.json"
  local z_get_infix="${z_parent_infix}-get"
  printf '%s\n' '{"options":{"requestedPolicyVersion":3}}' > "${z_get_body}"
  rbgu_http_json_ok "${z_label} (get policy)" "${z_token}" "POST" \
    "${z_get_url}" "${z_get_infix}" "${z_get_body}"

  bcu_log_args 'Extract etag; require non-empty'
  local z_etag=""
  z_etag=$(rbgu_json_field_capture "${z_get_infix}" ".etag") || bcu_die "Missing etag"
  test -n "${z_etag}" || bcu_die "Empty etag"

  bcu_log_args "Using etag ${z_etag}"

  bcu_log_args '2) Build new policy JSON in temp (bindings unique; version=3; keep etag)'
  local z_new_policy_json=""
  z_new_policy_json=$(rbgu_jq_add_member_to_role_capture "${z_get_infix}" "${z_role}" "${z_member}" "${z_etag}") \
    || bcu_die "Failed to compose policy JSON"
  bcu_log_args 'Ensure version=3'
  z_new_policy_json=$(jq '.version=3' <<<"${z_new_policy_json}") || bcu_die "Failed to set version=3"

  local z_set_body="${ZRBGI_PREFIX}${z_parent_infix}_set_body.json"
  printf '{"policy":%s}\n' "${z_new_policy_json}" > "${z_set_body}"

  bcu_log_args '3) setIamPolicy (fatal on 409/412 by policy)'
  local z_elapsed=0
  local z_set_infix=""
  while :; do
    z_set_infix="${z_parent_infix}-set-${z_elapsed}s"
    rbgu_http_json "POST" "${z_set_url}" "${z_token}" "${z_set_infix}" "${z_set_body}"

    local z_code=""
    z_code=$(rbgu_http_code_capture "${z_set_infix}") || bcu_die "No HTTP code"
    case "${z_code}" in
      200)                 break ;;
      412)                 bcu_die "${z_label}: precondition failed (etag mismatch)"    ;;
      429|500|502|503|504) bcu_log_args "Transient ${z_code} at ${z_elapsed}s; retry"   ;;
      409)                 bcu_die "${z_label}: HTTP 409 Conflict (fatal by invariant)" ;;
      *)                   rbgu_http_require_ok "${z_label} (set policy)" "${z_set_infix}" "" ;;
    esac

    test "${z_elapsed}" -ge "${RBGC_MAX_CONSISTENCY_SEC}" && bcu_die "${z_label}: timeout setting policy"
    sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
    z_elapsed=$((z_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
  done

  bcu_log_args '4) Verify membership within bounded wait'
  z_elapsed=0
  while :; do
    local z_verify_infix="${z_parent_infix}-verify-${z_elapsed}s"
    rbgu_http_json_ok "${z_label} (verify)" "${z_token}" "POST" \
                       "${z_get_url}" "${z_verify_infix}" "${z_get_body}"

    if jq -e --arg r "${z_role}" --arg m "${z_member}" \
         '.bindings[]? | select(.role==$r) | (.members // [])[]? == $m' \
         "${ZRBGU_PREFIX}${z_verify_infix}${ZRBGU_POSTFIX_JSON}" >/dev/null; then
      bcu_log_args "Observed ${z_role} for ${z_member}"

      bcu_log_args "Post-set etag $(rbgu_json_field_capture "${z_verify_infix}" ".etag" 2>/dev/null || echo "")"

      return 0
    fi

    test "${z_elapsed}" -ge "${RBGC_MAX_CONSISTENCY_SEC}" && bcu_die "${z_label}: verify timeout"
    sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
    z_elapsed=$((z_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
  done
}

rbgi_add_repo_iam_role() {
  zrbgi_sentinel

  local z_account_email="${1:-}"
  local z_location="${2:-}"
  local z_repository="${3:-}"
  local z_role="${4:-}"

  test -n "${z_account_email}" || bcu_die "Service account email required"
  test -n "${z_location}"      || bcu_die "Location is required"
  test -n "${z_repository}"    || bcu_die "Repository is required"
  test -n "${z_role}"          || bcu_die "Role is required"

  bcu_log_args 'Adding repo-scoped IAM role' \
               " ${z_role} to ${z_account_email} on ${z_location}/${z_repository}"

  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  local z_resource="projects/${RBRR_GCP_PROJECT_ID}${RBGC_PATH_LOCATIONS}/${z_location}${RBGC_PATH_REPOSITORIES}/${z_repository}"
  local z_get_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:getIamPolicy"
  local z_set_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:setIamPolicy"

  bcu_log_args 'Get current repo IAM policy'
  local z_get_code
  rbgu_http_json "POST" "${z_get_url}" "${z_token}" \
                                      "${ZRBGI_INFIX_REPO_ROLE}" "${ZRBGI_EMPTY_JSON}"
  z_get_code=$(rbgu_http_code_capture "${ZRBGI_INFIX_REPO_ROLE}") || z_get_code=""

  if test "${z_get_code}" = "404"; then
    # 404 means repo exists but has no IAM policy yet - this is normal for new repos
    bcu_log_args 'No IAM policy exists yet (404), initializing with empty bindings'
    echo '{"bindings":[]}' > "${ZRBGI_PREFIX}${ZRBGI_INFIX_REPO_ROLE}${ZRBGI_POSTFIX_JSON}"
  elif test "${z_get_code}" != "200"; then
    local z_err="HTTP ${z_get_code}"
    if jq -e .         "${ZRBGI_PREFIX}${ZRBGI_INFIX_REPO_ROLE}${ZRBGI_POSTFIX_JSON}" >/dev/null 2>&1; then
      z_err=$(rbgu_json_field_capture "${ZRBGI_INFIX_REPO_ROLE}" '.error.message') || z_err="HTTP ${z_get_code}"
    fi
    bcu_die "Get repo IAM policy failed: ${z_err}"
  fi

  bcu_log_args 'Update repo IAM policy'
  local z_updated_policy="${BDU_TEMP_DIR}/rbgi_repo_updated_policy.json"
  jq --arg role   "${z_role}"                                      \
     --arg member "serviceAccount:${z_account_email}"              \
     '
       .bindings = (.bindings // []) |
       if ( ([ .bindings[]? | .role ] | index($role)) ) then
         .bindings = ( .bindings | map(
           if .role == $role then .members = ( (.members // []) + [$member] | unique )
           else .
           end))
       else
         .bindings += [{role: $role, members: [$member]}]
       end
     ' "${ZRBGI_PREFIX}${ZRBGI_INFIX_REPO_ROLE}${ZRBGI_POSTFIX_JSON}" \
     > "${z_updated_policy}" || bcu_die "Failed to update policy json"

  bcu_log_args 'Set updated repo IAM policy'
  local z_repo_set_body="${BDU_TEMP_DIR}/rbgi_repo_set_policy_body.json"
  jq -n --slurpfile p "${z_updated_policy}" '{policy:$p[0]}' > "${z_repo_set_body}" \
    || bcu_die "Failed to build repo setIamPolicy body"
  rbgu_http_json "POST" "${z_set_url}" "${z_token}" \
                                             "${ZRBGI_INFIX_REPO_ROLE_SET}" "${z_repo_set_body}"
  rbgu_http_require_ok "Set repo IAM policy" "${ZRBGI_INFIX_REPO_ROLE_SET}"

  bcu_log_args 'Successfully added repo-scoped role' "${z_role}"
}

rbgi_add_sa_iam_role() {
  zrbgi_sentinel

  local z_target_sa_email="$1"
  local z_member_sa_email="$2"
  local z_role="$3"

  bcu_log_args "Granting ${z_role} on SA ${z_target_sa_email} to ${z_member_sa_email}"

  # Caller must have already primed Cloud Build if this is the runtime SA.
  # We do a hard existence check and crash if not accessible.
  local z_token
  z_token=$(rbgu_get_admin_token_capture) || bcu_die "Failed to get admin token"

  bcu_log_args 'Verify target SA exists'
  local z_target_encoded
  z_target_encoded=$(rbgu_urlencode_capture "${z_target_sa_email}") \
    || bcu_die "Failed to encode SA email"

  local z_verify_code
  rbgu_http_json "GET" \
    "${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/-/serviceAccounts/${z_target_encoded}" \
                            "${z_token}" "${ZRBGI_INFIX_SA_IAM_VERIFY}"
  z_verify_code=$(rbgu_http_code_capture "${ZRBGI_INFIX_SA_IAM_VERIFY}") || z_verify_code=""
  test "${z_verify_code}" = "200" || \
    bcu_die "Target service account not accessible: ${z_target_sa_email} (HTTP ${z_verify_code})"

  local z_sa_resource="${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/-/serviceAccounts/${z_target_encoded}"

  bcu_log_args 'Get current SA IAM policy'
  rbgu_http_json "POST" "${z_sa_resource}:getIamPolicy" "${z_token}" \
    "${ZRBGI_INFIX_ROLE}" "${ZRBGI_EMPTY_JSON}"

  local z_code
  z_code=$(rbgu_http_code_capture "${ZRBGI_INFIX_ROLE}") || z_code=""
  if test "${z_code}" != "200"; then
    bcu_log_args 'No IAM policy exists yet, initializing'
    echo '{"bindings":[]}' > "${ZRBGI_PREFIX}${ZRBGI_INFIX_ROLE}${ZRBGI_POSTFIX_JSON}"
  fi

  bcu_log_args 'Update SA IAM policy with new role binding'
  local z_updated_policy="${BDU_TEMP_DIR}/rbgi_sa_updated_policy.json"
  jq --arg role   "${z_role}"                              \
     --arg member "serviceAccount:${z_member_sa_email}"    \
     '
       .bindings = (.bindings // []) |
       if ( ([ .bindings[]? | .role ] | index($role)) ) then
         .bindings = ( .bindings | map(
           if .role == $role then .members = ( (.members // []) + [$member] | unique )
           else . end))
       else
         .bindings += [{role: $role, members: [$member]}]
       end
     ' "${ZRBGI_PREFIX}${ZRBGI_INFIX_ROLE}${ZRBGI_POSTFIX_JSON}" \
     > "${z_updated_policy}" || bcu_die "Failed to update SA IAM policy"

  bcu_log_args 'Set updated SA IAM policy'
  local z_set_body="${BDU_TEMP_DIR}/rbgi_sa_set_policy_body.json"
  jq -n --slurpfile p "${z_updated_policy}" '{policy:$p[0]}' > "${z_set_body}" \
    || bcu_die "Failed to build SA setIamPolicy body"

  rbgu_http_json "POST" "${z_sa_resource}:setIamPolicy" "${z_token}" \
                                           "${ZRBGI_INFIX_ROLE_SET}" "${z_set_body}"
  rbgu_http_require_ok "Set SA IAM policy" "${ZRBGI_INFIX_ROLE_SET}"

  bcu_log_args 'Successfully granted SA role' "${z_role}"
}

rbgi_add_bucket_iam_role() {
  zrbgi_sentinel

  local z_bucket_name="${1}"
  local z_account_email="${2}"
  local z_role="${3}"
  local z_token="${4}"

  bcu_log_args "Adding bucket IAM role ${z_role} to ${z_account_email}"

  local z_code

  bcu_log_args 'Get current bucket IAM policy'
  local z_iam_url="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}/iam"
  rbgu_http_json "GET" "${z_iam_url}" "${z_token}" "${ZRBGI_INFIX_BUCKET_IAM}"
  z_code=$(rbgu_http_code_capture                  "${ZRBGI_INFIX_BUCKET_IAM}") || z_code=""
  if test "${z_code}" != "200"; then
    bcu_log_args 'Initialize empty IAM policy for bucket'
    echo '{"bindings":[]}' > "${ZRBGI_PREFIX}${ZRBGI_INFIX_BUCKET_IAM}${ZRBGI_POSTFIX_JSON}"
  fi

  bcu_log_args 'Update bucket IAM policy'
  local z_updated="${BDU_TEMP_DIR}/rbgi_bucket_iam_updated.json"
  local z_etag
  z_etag=$(rbgu_json_field_capture "${ZRBGI_INFIX_BUCKET_IAM}" '.etag') || z_etag=""
  jq --arg role "${z_role}"                           \
     --arg member "serviceAccount:${z_account_email}" \
     --arg etag "${z_etag}"                           \
     '
       .bindings = (.bindings // []) |
       if ( ([ .bindings[]? | .role ] | index($role)) ) then
         .bindings = ( .bindings | map(
           if .role == $role then .members = ( (.members // []) + [$member] | unique )
           else .
           end))
       else
         .bindings += [{role: $role, members: [$member]}]
       end
       | ( if $etag != "" then .etag = $etag else . end )
     ' "${ZRBGI_PREFIX}${ZRBGI_INFIX_BUCKET_IAM}${ZRBGI_POSTFIX_JSON}" \
     > "${z_updated}" || bcu_die "Failed to update bucket IAM policy"

  local z_err

  bcu_log_args 'Set updated bucket IAM policy'
  rbgu_http_json "PUT" "${z_iam_url}" "${z_token}" \
                                  "${ZRBGI_INFIX_BUCKET_IAM_SET}" "${z_updated}"
  z_code=$(rbgu_http_code_capture "${ZRBGI_INFIX_BUCKET_IAM_SET}")                  || z_code=""
  z_err=$(rbgu_json_field_capture "${ZRBGI_INFIX_BUCKET_IAM_SET}" '.error.message') || z_err="HTTP ${z_code}"
  test "${z_code}" = "200" || bcu_die "Failed to set bucket IAM policy: ${z_err}"

  bcu_log_args "Successfully added bucket role ${z_role}"
}

# eof

