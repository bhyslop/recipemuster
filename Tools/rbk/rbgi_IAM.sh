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
# - Note: 409 has two distinct meanings in RBGI operations:
#   (a) Resource creation: "already exists" — state drift or prior manual activity.
#   (b) setIamPolicy: "ABORTED" — etag mismatch from concurrent policy change.
#   Both are fatal under single-writer invariant. Google-internal auto-provisioning
#   can trigger (b) outside our control; the 409 surfaces a real concurrency issue.
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
  readonly ZRBGI_PREFIX="${BURD_TEMP_DIR}/rbgi_"
  readonly ZRBGI_EMPTY_JSON="${ZRBGI_PREFIX}empty.json"
  printf '{}' > "${ZRBGI_EMPTY_JSON}"

  readonly ZRBGI_VERSION3_BODY="${ZRBGI_PREFIX}version3_body.json"
  printf '%s\n' '{"options":{"requestedPolicyVersion":3}}' > "${ZRBGI_VERSION3_BODY}"

  # Infix values for IAM operations
  readonly ZRBGI_INFIX_ROLE="role"
  readonly ZRBGI_INFIX_ROLE_SET="role_set"
  readonly ZRBGI_INFIX_REPO_ROLE="repo_role"
  readonly ZRBGI_INFIX_REPO_ROLE_SET="repo_role_set"
  readonly ZRBGI_INFIX_SA_IAM_VERIFY="sa_iamverify"
  readonly ZRBGI_INFIX_BUCKET_IAM="bucket_iam"
  readonly ZRBGI_INFIX_BUCKET_IAM_SET="bucket_iam_set"
  readonly ZRBGI_INFIX_SECRET_IAM="secret_iam"
  readonly ZRBGI_INFIX_SECRET_IAM_SET="secret_iam_set"

  readonly ZRBGI_POSTFIX_JSON="_i_resp.json"

  readonly ZRBGI_KINDLED=1
}

zrbgi_sentinel() {
  test "${ZRBGI_KINDLED:-}" = "1" || buc_die "Module rbgi not kindled - call zrbgi_kindle first"
}

# Check if an HTTP response is a 400 with a transient IAM propagation error.
# Two known patterns:
#   - "does not exist": newly-created SA not yet visible to policy service
#   - "is not deleted": recently-deleted SA still referenced in policy bindings
# Returns 0 (true) if retryable propagation error, 1 otherwise.
zrbgi_propagation_error_predicate() {
  local -r z_infix="${1}"
  local -r z_code="${2}"

  test "${z_code}" = "400" || return 1

  local z_err_msg=""
  z_err_msg=$(rbgu_error_message_capture "${z_infix}") || z_err_msg=""

  case "${z_err_msg}" in
    *"does not exist"*) return 0 ;;
    *"is not deleted"*) return 0 ;;
    *)                  return 1 ;;
  esac
}

######################################################################
# External Functions (rbgi_*)

# Add a project-scoped IAM role binding with optimistic concurrency and strong read-back.
rbgi_add_project_iam_role() {
  zrbgi_sentinel

  local -r z_token="${1:-}"
  local -r z_label="${2:-}"
  local -r z_resource="${3:-}"  # resource_base: Base resource URL
  local -r z_role="${4:-}"
  local -r z_member="${5:-}"
  local -r z_parent_infix="${6:-}"

  test -n "${z_token}" || buc_die "Token required"
  buc_log_args "Using admin token (value not logged)"

  local -r z_resource_path="${z_resource#/}"  # strip leading slash if present
  local -r z_base="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/${z_resource_path}"
  local -r z_get_url="${z_base}${RBGC_CRM_GET_IAM_POLICY_SUFFIX}"
  local -r z_set_url="${z_base}${RBGC_CRM_SET_IAM_POLICY_SUFFIX}"

  test -n "${z_resource}" || buc_die "resource required"
  test -n "${z_role}"     || buc_die "role required"
  test -n "${z_member}"   || buc_die "member required"

  buc_log_args "${z_label}: add ${z_member} to ${z_role}"

  # Propagation retry: steps 1-3 may fail with HTTP 400 "does not exist"
  # when a newly-created SA hasn't propagated to the IAM policy service.
  # Exponential backoff: 3s initial, 2x multiplier, 20s cap, 420s deadline.
  local z_prop_delay=3
  local z_prop_elapsed=0
  local z_prop_deadline=420
  local z_prop_attempt=0
  local z_prop_succeeded=0

  while :; do
    z_prop_attempt=$((z_prop_attempt + 1))

    buc_log_args "1) GET policy (v3) [attempt ${z_prop_attempt}]"
    buc_log_args "GET_POLICY_URL_DEBUG z_resource:${z_resource} z_get_url:${z_get_url}"
    local z_get_body="${ZRBGI_PREFIX}${z_parent_infix}_get_body.json"
    local z_get_infix="${z_parent_infix}-get-${z_prop_elapsed}s"
    printf '%s\n' '{"options":{"requestedPolicyVersion":3}}' > "${z_get_body}"
    rbgu_http_json "POST" "${z_get_url}" "${z_token}" "${z_get_infix}" "${z_get_body}"

    local z_get_code=""
    z_get_code=$(rbgu_http_code_capture "${z_get_infix}") || buc_die "No HTTP code from getIamPolicy"

    # Check for propagation error on GET
    if zrbgi_propagation_error_predicate "${z_get_infix}" "${z_get_code}"; then
      buc_log_args "${z_label}: getIamPolicy returned 400 'does not exist' (propagation delay)"
      test "${z_prop_elapsed}" -lt "${z_prop_deadline}" \
        || buc_die "${z_label}: propagation timeout after ${z_prop_elapsed}s waiting for member visibility"
      buc_log_args "Retry ${z_prop_attempt} at ${z_prop_elapsed}s (next delay ${z_prop_delay}s)"
      sleep "${z_prop_delay}"
      z_prop_elapsed=$((z_prop_elapsed + z_prop_delay))
      z_prop_delay=$((z_prop_delay * 2))
      test "${z_prop_delay}" -le 20 || z_prop_delay=20
      continue
    fi

    # Not a propagation error on GET — require success
    rbgu_http_require_ok "${z_label} (get policy)" "${z_get_infix}"

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

    buc_log_args '3) setIamPolicy (fatal on 409 — etag mismatch)'
    local z_set_elapsed=0
    local z_set_infix=""
    local z_set_succeeded=0
    while :; do
      z_set_infix="${z_parent_infix}-set-${z_prop_elapsed}s-${z_set_elapsed}s"
      rbgu_http_json "POST" "${z_set_url}" "${z_token}" "${z_set_infix}" "${z_set_body}"

      local z_code=""
      z_code=$(rbgu_http_code_capture "${z_set_infix}") || buc_die "No HTTP code"

      # Check for propagation error on SET — break inner loop to retry outer
      if zrbgi_propagation_error_predicate "${z_set_infix}" "${z_code}"; then
        buc_log_args "${z_label}: setIamPolicy returned 400 'does not exist' (propagation delay)"
        break
      fi

      case "${z_code}" in
        200)                 z_set_succeeded=1; break ;;
        409)                 buc_die "${z_label}: HTTP 409 ABORTED (etag mismatch — concurrent policy change)" ;;
        429|500|502|503|504) buc_log_args "Transient ${z_code} at ${z_set_elapsed}s; retry" ;;
        *)                   rbgu_http_require_ok "${z_label} (set policy)" "${z_set_infix}" "" ;;
      esac

      test "${z_set_elapsed}" -lt "${RBGC_MAX_CONSISTENCY_SEC}" || buc_die "${z_label}: timeout setting policy"
      sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
      z_set_elapsed=$((z_set_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
    done

    # If setIamPolicy succeeded, break outer propagation loop
    test "${z_set_succeeded}" != "1" || { z_prop_succeeded=1; break; }

    # setIamPolicy hit propagation error — retry outer loop with fresh getIamPolicy
    test "${z_prop_elapsed}" -lt "${z_prop_deadline}" \
      || buc_die "${z_label}: propagation timeout after ${z_prop_elapsed}s waiting for member visibility"
    buc_log_args "Retry ${z_prop_attempt} at ${z_prop_elapsed}s (next delay ${z_prop_delay}s)"
    sleep "${z_prop_delay}"
    z_prop_elapsed=$((z_prop_elapsed + z_prop_delay))
    z_prop_delay=$((z_prop_delay * 2))
    test "${z_prop_delay}" -le 20 || z_prop_delay=20
  done

  test "${z_prop_succeeded}" = "1" || buc_die "${z_label}: propagation retry loop exited without success"

  buc_log_args '4) Verify membership within bounded wait'
  local z_elapsed=0
  while :; do
    local z_verify_infix="${z_parent_infix}-verify-${z_elapsed}s"
    rbgu_http_json_ok "${z_label} (verify)" "${z_token}" "POST" \
                       "${z_get_url}" "${z_verify_infix}" "${z_get_body}"

    if rbgu_role_member_exists_predicate "${z_verify_infix}" "${z_role}" "${z_member}"; then
      buc_log_args "Observed ${z_role} for ${z_member}"

      buc_log_args "Post-set etag $(rbgu_json_field_capture "${z_verify_infix}" ".etag" 2>/dev/null || echo "")"

      return 0
    fi

    test "${z_elapsed}" -lt "${RBGC_MAX_CONSISTENCY_SEC}" || buc_die "${z_label}: verify timeout"
    sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
    z_elapsed=$((z_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
  done
}

rbgi_add_repo_iam_role() {
  zrbgi_sentinel

  local -r z_token="${1:-}"
  local -r z_project_id="${2:-}"
  local -r z_account_email="${3:-}"
  local -r z_location="${4:-}"
  local -r z_repository="${5:-}"
  local -r z_role="${6:-}"

  test -n "${z_token}"         || buc_die "Token required"
  test -n "${z_project_id}"    || buc_die "Project ID required"
  test -n "${z_account_email}" || buc_die "Service account email required"
  test -n "${z_location}"      || buc_die "Location is required"
  test -n "${z_repository}"    || buc_die "Repository is required"
  test -n "${z_role}"          || buc_die "Role is required"

  buc_log_args "Using admin token (value not logged)"

  local -r z_resource="projects/${z_project_id}/locations/${z_location}/repositories/${z_repository}"
  local -r z_get_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:getIamPolicy?options.requestedPolicyVersion=3"
  local -r z_set_url="${RBGC_API_ROOT_ARTIFACTREGISTRY}${RBGC_ARTIFACTREGISTRY_V1}/${z_resource}:setIamPolicy"

  buc_log_args 'Adding repo-scoped IAM role' \
               " ${z_role} to ${z_account_email} on ${z_location}/${z_repository}"

  # Propagation retry: get-modify-set may fail with HTTP 400 "does not exist"
  # when a newly-created SA hasn't propagated to the IAM policy service.
  # Exponential backoff: 3s initial, 2x multiplier, 20s cap, 420s deadline.
  local z_prop_delay=3
  local z_prop_elapsed=0
  local z_prop_deadline=420
  local z_prop_attempt=0

  local z_prop_succeeded=0

  while :; do
    z_prop_attempt=$((z_prop_attempt + 1))
    local z_get_infix="${ZRBGI_INFIX_REPO_ROLE}-${z_prop_elapsed}s"

    buc_log_args "1) GET repo IAM policy (v3) [attempt ${z_prop_attempt}]"
    rbgu_http_json "GET" "${z_get_url}" "${z_token}" "${z_get_infix}"

    local z_get_code
    z_get_code=$(rbgu_http_code_capture "${z_get_infix}") || buc_die "No HTTP code from repo getIamPolicy"

    # Check for propagation error on GET
    if zrbgi_propagation_error_predicate "${z_get_infix}" "${z_get_code}"; then
      buc_log_args "Repo getIamPolicy returned 400 'does not exist' (propagation delay)"
      test "${z_prop_elapsed}" -lt "${z_prop_deadline}" \
        || buc_die "Repo IAM: propagation timeout after ${z_prop_elapsed}s"
      buc_log_args "Retry ${z_prop_attempt} at ${z_prop_elapsed}s (next delay ${z_prop_delay}s)"
      sleep "${z_prop_delay}"
      z_prop_elapsed=$((z_prop_elapsed + z_prop_delay))
      z_prop_delay=$((z_prop_delay * 2))
      test "${z_prop_delay}" -le 20 || z_prop_delay=20
      continue
    fi

    # Not a propagation error on GET — require success
    rbgu_http_require_ok "Get repo IAM policy" "${z_get_infix}"

    buc_log_args 'Extract etag; require non-empty'
    local z_etag=""
    z_etag=$(rbgu_json_field_capture "${z_get_infix}" ".etag") || buc_die "Missing repo etag"
    test -n "${z_etag}" || buc_die "Empty repo etag"

    buc_log_args "Using etag ${z_etag}"

    buc_log_args '2) Build new policy JSON (bindings unique; version=3; keep etag)'
    local z_updated_policy_json=""
    z_updated_policy_json=$(rbgu_jq_add_member_to_role_capture "${z_get_infix}" \
      "${z_role}" "serviceAccount:${z_account_email}" "${z_etag}") \
      || buc_die "Failed to update policy JSON"

    buc_log_args '3) setIamPolicy (fatal on 409 — etag mismatch)'
    local z_repo_set_body="${BURD_TEMP_DIR}/rbgi_repo_set_policy_body.json"
    printf '{"policy":%s}\n' "${z_updated_policy_json}" > "${z_repo_set_body}" \
      || buc_die "Failed to build repo setIamPolicy body"

    local z_set_elapsed=0
    local z_set_infix=""
    local z_set_succeeded=0
    while :; do
      z_set_infix="${ZRBGI_INFIX_REPO_ROLE_SET}-${z_prop_elapsed}s-${z_set_elapsed}s"
      rbgu_http_json "POST" "${z_set_url}" "${z_token}" "${z_set_infix}" "${z_repo_set_body}"

      local z_set_code=""
      z_set_code=$(rbgu_http_code_capture "${z_set_infix}") || buc_die "No HTTP code from setIamPolicy"

      # Check for propagation error on SET — break inner loop to retry outer
      if zrbgi_propagation_error_predicate "${z_set_infix}" "${z_set_code}"; then
        buc_log_args "Repo setIamPolicy returned 400 'does not exist' (propagation delay)"
        break
      fi

      case "${z_set_code}" in
        200)                 z_set_succeeded=1; break ;;
        409)                 buc_die "Repo IAM: HTTP 409 ABORTED (etag mismatch — concurrent policy change)" ;;
        429|500|502|503|504) buc_log_args "Transient ${z_set_code} at ${z_set_elapsed}s; retry" ;;
        *)                   rbgu_http_require_ok "Set repo IAM policy" "${z_set_infix}" "" ;;
      esac

      test "${z_set_elapsed}" -lt "${RBGC_MAX_CONSISTENCY_SEC}" || buc_die "Repo IAM: timeout setting policy"
      sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
      z_set_elapsed=$((z_set_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
    done

    # If setIamPolicy succeeded, break outer propagation loop
    test "${z_set_succeeded}" != "1" || { z_prop_succeeded=1; break; }

    # setIamPolicy hit propagation error — retry outer loop with fresh getIamPolicy
    test "${z_prop_elapsed}" -lt "${z_prop_deadline}" \
      || buc_die "Repo IAM: propagation timeout after ${z_prop_elapsed}s"
    buc_log_args "Retry ${z_prop_attempt} at ${z_prop_elapsed}s (next delay ${z_prop_delay}s)"
    sleep "${z_prop_delay}"
    z_prop_elapsed=$((z_prop_elapsed + z_prop_delay))
    z_prop_delay=$((z_prop_delay * 2))
    test "${z_prop_delay}" -le 20 || z_prop_delay=20
  done

  test "${z_prop_succeeded}" = "1" || buc_die "Repo IAM: propagation retry loop exited without success"

  buc_log_args 'Successfully added repo-scoped role' "${z_role}"
}

rbgi_add_sa_iam_role() {
  zrbgi_sentinel

  local -r z_token="${1:-}"
  local -r z_target_sa_email="${2:-}"
  local -r z_member_email="${3:-}"  # email only; function adds serviceAccount: prefix
  local -r z_role="${4:-}"

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
  z_verify_code=$(rbgu_http_code_capture "${ZRBGI_INFIX_SA_IAM_VERIFY}") || buc_die "No HTTP code from SA verify"
  test "${z_verify_code}" = "200" || \
    buc_die "Target service account not accessible: ${z_target_sa_email} (HTTP ${z_verify_code})"

  local -r z_sa_resource="${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}/projects/-/serviceAccounts/${z_target_encoded}"

  # Propagation retry: get-modify-set may fail with HTTP 400 "does not exist"
  # when a newly-created SA hasn't propagated to the IAM policy service.
  # Exponential backoff: 3s initial, 2x multiplier, 20s cap, 420s deadline.
  local z_prop_delay=3
  local z_prop_elapsed=0
  local z_prop_deadline=420
  local z_prop_attempt=0

  local z_prop_succeeded=0

  while :; do
    z_prop_attempt=$((z_prop_attempt + 1))
    local z_get_infix="${ZRBGI_INFIX_ROLE}-${z_prop_elapsed}s"

    buc_log_args "1) GET SA IAM policy (v3) [attempt ${z_prop_attempt}]"
    rbgu_http_json "POST" "${z_sa_resource}:getIamPolicy" "${z_token}" \
      "${z_get_infix}" "${ZRBGI_VERSION3_BODY}"

    local z_code
    z_code=$(rbgu_http_code_capture "${z_get_infix}") || buc_die "No HTTP code from SA getIamPolicy"

    # Check for propagation error on GET
    if zrbgi_propagation_error_predicate "${z_get_infix}" "${z_code}"; then
      buc_log_args "SA getIamPolicy returned 400 'does not exist' (propagation delay)"
      test "${z_prop_elapsed}" -lt "${z_prop_deadline}" \
        || buc_die "SA IAM: propagation timeout after ${z_prop_elapsed}s"
      buc_log_args "Retry ${z_prop_attempt} at ${z_prop_elapsed}s (next delay ${z_prop_delay}s)"
      sleep "${z_prop_delay}"
      z_prop_elapsed=$((z_prop_elapsed + z_prop_delay))
      z_prop_delay=$((z_prop_delay * 2))
      test "${z_prop_delay}" -le 20 || z_prop_delay=20
      continue
    fi

    # Not a propagation error on GET — require success
    rbgu_http_require_ok "Get SA IAM policy" "${z_get_infix}"

    buc_log_args 'Extract etag; require non-empty'
    local z_etag=""
    z_etag=$(rbgu_json_field_capture "${z_get_infix}" ".etag") || buc_die "Missing SA etag"
    test -n "${z_etag}" || buc_die "Empty SA etag"

    buc_log_args "Using etag ${z_etag}"

    buc_log_args '2) Build new policy JSON (bindings unique; version=3; keep etag)'
    local z_updated_policy_json=""
    z_updated_policy_json=$(rbgu_jq_add_member_to_role_capture "${z_get_infix}" \
      "${z_role}" "serviceAccount:${z_member_email}" "${z_etag}") \
      || buc_die "Failed to update SA IAM policy"

    buc_log_args '3) setIamPolicy (fatal on 409 — etag mismatch)'
    local z_set_body="${BURD_TEMP_DIR}/rbgi_sa_set_policy_body.json"
    printf '{"policy":%s}\n' "${z_updated_policy_json}" > "${z_set_body}" \
      || buc_die "Failed to build SA setIamPolicy body"

    local z_set_elapsed=0
    local z_set_infix=""
    local z_set_succeeded=0
    while :; do
      z_set_infix="${ZRBGI_INFIX_ROLE_SET}-${z_prop_elapsed}s-${z_set_elapsed}s"
      rbgu_http_json "POST" "${z_sa_resource}:setIamPolicy" "${z_token}" \
        "${z_set_infix}" "${z_set_body}"

      local z_set_code=""
      z_set_code=$(rbgu_http_code_capture "${z_set_infix}") || buc_die "No HTTP code from setIamPolicy"

      # Check for propagation error on SET — break inner loop to retry outer
      if zrbgi_propagation_error_predicate "${z_set_infix}" "${z_set_code}"; then
        buc_log_args "SA setIamPolicy returned 400 'does not exist' (propagation delay)"
        break
      fi

      case "${z_set_code}" in
        200)                 z_set_succeeded=1; break ;;
        409)                 buc_die "SA IAM: HTTP 409 ABORTED (etag mismatch — concurrent policy change)" ;;
        429|500|502|503|504) buc_log_args "Transient ${z_set_code} at ${z_set_elapsed}s; retry" ;;
        *)                   rbgu_http_require_ok "Set SA IAM policy" "${z_set_infix}" "" ;;
      esac

      test "${z_set_elapsed}" -lt "${RBGC_MAX_CONSISTENCY_SEC}" || buc_die "SA IAM: timeout setting policy"
      sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
      z_set_elapsed=$((z_set_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
    done

    # If setIamPolicy succeeded, break outer propagation loop
    test "${z_set_succeeded}" != "1" || { z_prop_succeeded=1; break; }

    # setIamPolicy hit propagation error — retry outer loop with fresh getIamPolicy
    test "${z_prop_elapsed}" -lt "${z_prop_deadline}" \
      || buc_die "SA IAM: propagation timeout after ${z_prop_elapsed}s"
    buc_log_args "Retry ${z_prop_attempt} at ${z_prop_elapsed}s (next delay ${z_prop_delay}s)"
    sleep "${z_prop_delay}"
    z_prop_elapsed=$((z_prop_elapsed + z_prop_delay))
    z_prop_delay=$((z_prop_delay * 2))
    test "${z_prop_delay}" -le 20 || z_prop_delay=20
  done

  test "${z_prop_succeeded}" = "1" || buc_die "SA IAM: propagation retry loop exited without success"

  buc_log_args 'Successfully granted SA role' "${z_role}"
}

rbgi_add_bucket_iam_role() {
  zrbgi_sentinel

  local -r z_token="${1:-}"
  local -r z_bucket_name="${2:-}"
  local -r z_account_email="${3:-}"
  local -r z_role="${4:-}"

  test -n "${z_token}" || buc_die "Token required"

  buc_log_args "Using admin token (value not logged)"
  buc_log_args "Adding bucket IAM role ${z_role} to ${z_account_email}"

  local -r z_iam_url="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b/${z_bucket_name}/iam"

  # Propagation retry: get-modify-set may fail with HTTP 400 "does not exist"
  # when a newly-created SA hasn't propagated to the IAM policy service.
  # Exponential backoff: 3s initial, 2x multiplier, 20s cap, 420s deadline.
  local z_prop_delay=3
  local z_prop_elapsed=0
  local z_prop_deadline=420
  local z_prop_attempt=0

  local z_prop_succeeded=0

  while :; do
    z_prop_attempt=$((z_prop_attempt + 1))
    local z_get_infix="${ZRBGI_INFIX_BUCKET_IAM}-${z_prop_elapsed}s"

    buc_log_args "1) GET bucket IAM policy (v3) [attempt ${z_prop_attempt}]"
    rbgu_http_json "GET" "${z_iam_url}?optionsRequestedPolicyVersion=3" "${z_token}" "${z_get_infix}"

    local z_code
    z_code=$(rbgu_http_code_capture "${z_get_infix}") || buc_die "No HTTP code from bucket getIamPolicy"

    # Check for propagation error on GET
    if zrbgi_propagation_error_predicate "${z_get_infix}" "${z_code}"; then
      buc_log_args "Bucket getIamPolicy returned 400 'does not exist' (propagation delay)"
      test "${z_prop_elapsed}" -lt "${z_prop_deadline}" \
        || buc_die "Bucket IAM: propagation timeout after ${z_prop_elapsed}s"
      buc_log_args "Retry ${z_prop_attempt} at ${z_prop_elapsed}s (next delay ${z_prop_delay}s)"
      sleep "${z_prop_delay}"
      z_prop_elapsed=$((z_prop_elapsed + z_prop_delay))
      z_prop_delay=$((z_prop_delay * 2))
      test "${z_prop_delay}" -le 20 || z_prop_delay=20
      continue
    fi

    # Not a propagation error on GET — require success
    rbgu_http_require_ok "Get bucket IAM policy" "${z_get_infix}"

    buc_log_args 'Extract etag; require non-empty'
    local z_etag=""
    z_etag=$(rbgu_json_field_capture "${z_get_infix}" ".etag") || buc_die "Missing bucket etag"
    test -n "${z_etag}" || buc_die "Empty bucket etag"

    buc_log_args "Using etag ${z_etag}"

    buc_log_args '2) Build new policy JSON (bindings unique; keep etag)'
    local z_updated_policy_json=""
    z_updated_policy_json=$(rbgu_jq_add_member_to_role_capture "${z_get_infix}" \
      "${z_role}" "serviceAccount:${z_account_email}" "${z_etag}") \
      || buc_die "Failed to update bucket IAM policy"

    buc_log_args '3) setIamPolicy (fatal on 412 — etag mismatch; Storage uses 412 not 409)'
    local z_bucket_set_body="${BURD_TEMP_DIR}/rbgi_bucket_set_policy_body.json"
    printf '%s\n' "${z_updated_policy_json}" > "${z_bucket_set_body}" \
      || buc_die "Failed to write bucket policy body"

    local z_set_elapsed=0
    local z_set_infix=""
    local z_set_succeeded=0
    while :; do
      z_set_infix="${ZRBGI_INFIX_BUCKET_IAM_SET}-${z_prop_elapsed}s-${z_set_elapsed}s"
      rbgu_http_json "PUT" "${z_iam_url}" "${z_token}" "${z_set_infix}" "${z_bucket_set_body}"

      local z_set_code=""
      z_set_code=$(rbgu_http_code_capture "${z_set_infix}") || buc_die "No HTTP code from setIamPolicy"

      # Check for propagation error on SET — break inner loop to retry outer
      if zrbgi_propagation_error_predicate "${z_set_infix}" "${z_set_code}"; then
        buc_log_args "Bucket setIamPolicy returned 400 'does not exist' (propagation delay)"
        break
      fi

      case "${z_set_code}" in
        200)                 z_set_succeeded=1; break ;;
        412)                 buc_die "Bucket IAM: HTTP 412 Precondition Failed (etag mismatch)" ;;
        409)                 buc_die "Bucket IAM: HTTP 409 ABORTED (defensive — unexpected for Storage)" ;;
        429|500|502|503|504) buc_log_args "Transient ${z_set_code} at ${z_set_elapsed}s; retry" ;;
        *)                   rbgu_http_require_ok "Set bucket IAM policy" "${z_set_infix}" "" ;;
      esac

      test "${z_set_elapsed}" -lt "${RBGC_MAX_CONSISTENCY_SEC}" || buc_die "Bucket IAM: timeout setting policy"
      sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
      z_set_elapsed=$((z_set_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
    done

    # If setIamPolicy succeeded, break outer propagation loop
    test "${z_set_succeeded}" != "1" || { z_prop_succeeded=1; break; }

    # setIamPolicy hit propagation error — retry outer loop with fresh getIamPolicy
    test "${z_prop_elapsed}" -lt "${z_prop_deadline}" \
      || buc_die "Bucket IAM: propagation timeout after ${z_prop_elapsed}s"
    buc_log_args "Retry ${z_prop_attempt} at ${z_prop_elapsed}s (next delay ${z_prop_delay}s)"
    sleep "${z_prop_delay}"
    z_prop_elapsed=$((z_prop_elapsed + z_prop_delay))
    z_prop_delay=$((z_prop_delay * 2))
    test "${z_prop_delay}" -le 20 || z_prop_delay=20
  done

  test "${z_prop_succeeded}" = "1" || buc_die "Bucket IAM: propagation retry loop exited without success"

  buc_log_args "Successfully added bucket role ${z_role}"
}

# Grant an IAM role binding on a Secret Manager secret with optimistic concurrency and propagation retry.
rbgi_grant_secret_iam() {
  zrbgi_sentinel

  local -r z_token="${1:-}"
  local -r z_secret_resource_path="${2:-}"
  local -r z_member="${3:-}"
  local -r z_role="${4:-}"
  local -r z_parent_infix="${5:-}"

  test -n "${z_token}"                || buc_die "Token required"
  test -n "${z_secret_resource_path}" || buc_die "Secret resource path required"
  test -n "${z_member}"               || buc_die "Member required"
  test -n "${z_role}"                 || buc_die "Role required"
  test -n "${z_parent_infix}"         || buc_die "Parent infix required"

  buc_log_args "Using admin token (value not logged)"
  buc_log_args "Granting ${z_role} on secret ${z_secret_resource_path} to ${z_member}"

  local -r z_get_url="${RBGC_API_ROOT_SECRETMANAGER}${RBGC_SECRETMANAGER_V1}/${z_secret_resource_path}:getIamPolicy?options.requestedPolicyVersion=3"
  local -r z_set_url="${RBGC_API_ROOT_SECRETMANAGER}${RBGC_SECRETMANAGER_V1}/${z_secret_resource_path}:setIamPolicy"

  # Propagation retry: get-modify-set may fail with HTTP 400 "does not exist"
  # when a newly-created SA hasn't propagated to the IAM policy service.
  # Exponential backoff: 3s initial, 2x multiplier, 20s cap, 420s deadline.
  local z_prop_delay=3
  local z_prop_elapsed=0
  local z_prop_deadline=420
  local z_prop_attempt=0

  local z_prop_succeeded=0

  while :; do
    z_prop_attempt=$((z_prop_attempt + 1))
    local z_get_infix="${z_parent_infix}-get-${z_prop_elapsed}s"

    buc_log_args "1) GET secret IAM policy (v3) [attempt ${z_prop_attempt}]"
    rbgu_http_json "GET" "${z_get_url}" "${z_token}" "${z_get_infix}"

    local z_get_code
    z_get_code=$(rbgu_http_code_capture "${z_get_infix}") || buc_die "No HTTP code from secret getIamPolicy"

    # Check for propagation error on GET
    if zrbgi_propagation_error_predicate "${z_get_infix}" "${z_get_code}"; then
      buc_log_args "Secret getIamPolicy returned 400 'does not exist' (propagation delay)"
      test "${z_prop_elapsed}" -lt "${z_prop_deadline}" \
        || buc_die "Secret IAM: propagation timeout after ${z_prop_elapsed}s"
      buc_log_args "Retry ${z_prop_attempt} at ${z_prop_elapsed}s (next delay ${z_prop_delay}s)"
      sleep "${z_prop_delay}"
      z_prop_elapsed=$((z_prop_elapsed + z_prop_delay))
      z_prop_delay=$((z_prop_delay * 2))
      test "${z_prop_delay}" -le 20 || z_prop_delay=20
      continue
    fi

    # Not a propagation error on GET — require success
    rbgu_http_require_ok "Get secret IAM policy" "${z_get_infix}"

    buc_log_args 'Extract etag; require non-empty'
    local z_etag=""
    z_etag=$(rbgu_json_field_capture "${z_get_infix}" ".etag") || buc_die "Missing secret etag"
    test -n "${z_etag}" || buc_die "Empty secret etag"

    buc_log_args "Using etag ${z_etag}"

    buc_log_args '2) Build new policy JSON (bindings unique; version=3; keep etag)'
    local z_updated_policy_json=""
    z_updated_policy_json=$(rbgu_jq_add_member_to_role_capture "${z_get_infix}" \
      "${z_role}" "${z_member}" "${z_etag}") \
      || buc_die "Failed to update secret IAM policy"

    buc_log_args '3) setIamPolicy (fatal on 409 — etag mismatch)'
    local z_set_body="${ZRBGI_PREFIX}${z_parent_infix}_set_body.json"
    printf '{"policy":%s}\n' "${z_updated_policy_json}" > "${z_set_body}" \
      || buc_die "Failed to build secret setIamPolicy body"

    local z_set_elapsed=0
    local z_set_infix=""
    local z_set_succeeded=0
    while :; do
      z_set_infix="${z_parent_infix}-set-${z_prop_elapsed}s-${z_set_elapsed}s"
      rbgu_http_json "POST" "${z_set_url}" "${z_token}" "${z_set_infix}" "${z_set_body}"

      local z_set_code=""
      z_set_code=$(rbgu_http_code_capture "${z_set_infix}") || buc_die "No HTTP code from setIamPolicy"

      # Check for propagation error on SET — break inner loop to retry outer
      if zrbgi_propagation_error_predicate "${z_set_infix}" "${z_set_code}"; then
        buc_log_args "Secret setIamPolicy returned 400 'does not exist' (propagation delay)"
        break
      fi

      case "${z_set_code}" in
        200)                 z_set_succeeded=1; break ;;
        409)                 buc_die "Secret IAM: HTTP 409 ABORTED (etag mismatch — concurrent policy change)" ;;
        429|500|502|503|504) buc_log_args "Transient ${z_set_code} at ${z_set_elapsed}s; retry" ;;
        *)                   rbgu_http_require_ok "Set secret IAM policy" "${z_set_infix}" "" ;;
      esac

      test "${z_set_elapsed}" -lt "${RBGC_MAX_CONSISTENCY_SEC}" || buc_die "Secret IAM: timeout setting policy"
      sleep "${RBGC_EVENTUAL_CONSISTENCY_SEC}"
      z_set_elapsed=$((z_set_elapsed + RBGC_EVENTUAL_CONSISTENCY_SEC))
    done

    # If setIamPolicy succeeded, break outer propagation loop
    test "${z_set_succeeded}" != "1" || { z_prop_succeeded=1; break; }

    # setIamPolicy hit propagation error — retry outer loop with fresh getIamPolicy
    test "${z_prop_elapsed}" -lt "${z_prop_deadline}" \
      || buc_die "Secret IAM: propagation timeout after ${z_prop_elapsed}s"
    buc_log_args "Retry ${z_prop_attempt} at ${z_prop_elapsed}s (next delay ${z_prop_delay}s)"
    sleep "${z_prop_delay}"
    z_prop_elapsed=$((z_prop_elapsed + z_prop_delay))
    z_prop_delay=$((z_prop_delay * 2))
    test "${z_prop_delay}" -le 20 || z_prop_delay=20
  done

  test "${z_prop_succeeded}" = "1" || buc_die "Secret IAM: propagation retry loop exited without success"

  buc_log_args "Successfully granted secret role ${z_role}"
}

# eof

