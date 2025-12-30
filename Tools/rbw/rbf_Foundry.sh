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
# Recipe Bottle Foundry - GCB image creation and GAR deletion

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBF_SOURCED:-}" || buc_die "Module rbf multiply sourced - check sourcing hierarchy"
ZRBF_SOURCED=1

######################################################################
# Internal Functions (zrbf_*)

zrbf_kindle() {
  test -z "${ZRBF_KINDLED:-}" || buc_die "Module rbf already kindled"

  # Validate environment
  buv_dir_exists "${BUD_TEMP_DIR}"
  test -n "${BUD_NOW_STAMP:-}" || buc_die "BUD_NOW_STAMP is unset or empty"

  buc_log_args 'Check required GCB/GAR environment variables'
  zrbgc_sentinel

  buc_log_args 'Verify service account files'
  test -n "${RBRR_DIRECTOR_RBRA_FILE:-}" || buc_die "RBRR_DIRECTOR_RBRA_FILE not set"
  test -f "${RBRR_DIRECTOR_RBRA_FILE}"   || buc_die "GCB service env file not found: ${RBRR_DIRECTOR_RBRA_FILE}"

  buc_log_args 'Module Variables (ZRBF_*)'
  ZRBF_GCB_API_BASE="https://cloudbuild.googleapis.com/v1"
  ZRBF_GAR_API_BASE="https://artifactregistry.googleapis.com/v1"
  ZRBF_CLOUD_QUERY_BASE="https://console.cloud.google.com/cloud-build/builds"

  ZRBF_GCB_API_BASE_UPLOAD="https://cloudbuild.googleapis.com/upload/v1"

  # GCS endpoints (build-source uploads)
  ZRBF_GCS_API_BASE="https://storage.googleapis.com/storage/v1"
  ZRBF_GCS_UPLOAD_BASE="https://storage.googleapis.com/upload/storage/v1"

  # Temp files for object naming and requests
  ZRBF_TARBALL_NAME_FILE="${BUD_TEMP_DIR}/rbf_tarball_name.txt"
  ZRBF_GCS_OBJECT_FILE="${BUD_TEMP_DIR}/rbf_gcs_object.txt"
  ZRBF_BUILD_REQUEST_FILE="${BUD_TEMP_DIR}/rbf_build_request.json"
  ZRBF_GCS_UPLOAD_RESP="${BUD_TEMP_DIR}/rbf_gcs_upload_resp.json"
  ZRBF_GCS_UPLOAD_HTTP="${BUD_TEMP_DIR}/rbf_gcs_upload_http.txt"

  ZRBF_GCB_PROJECT_BUILDS_URL="${ZRBF_GCB_API_BASE}/projects/${RBGD_GCB_PROJECT_ID}/locations/${RBGD_GCB_REGION}/builds"
  ZRBF_GCB_PROJECT_BUILDS_UPLOAD_URL="${ZRBF_GCB_API_BASE_UPLOAD}/projects/${RBGD_GCB_PROJECT_ID}/locations/${RBGD_GCB_REGION}/builds"
  ZRBF_GAR_PACKAGE_BASE="projects/${RBGD_GAR_PROJECT_ID}/locations/${RBGD_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}"

  buc_log_args 'Registry API endpoints for delete'
  ZRBF_REGISTRY_HOST="${RBGD_GAR_LOCATION}-docker.pkg.dev"
  ZRBF_REGISTRY_PATH="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  ZRBF_REGISTRY_API_BASE="https://${ZRBF_REGISTRY_HOST}/v2/${ZRBF_REGISTRY_PATH}"

  buc_log_args 'Media types for delete operation'
  ZRBF_ACCEPT_MANIFEST_MTYPES="application/vnd.docker.distribution.manifest.v2+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.oci.image.index.v1+json,application/vnd.oci.image.manifest.v1+json"

  buc_log_args 'RBGY files presumed to be in the same Tools directory as this implementation'
  local z_self_dir="${BASH_SOURCE[0]%/*}"
  ZRBF_RBGY_BUILD_FILE="${z_self_dir}/rbgy_build.yaml"
  ZRBF_RBGY_COPY_FILE="${z_self_dir}/rbgy_copy.yaml"
  test -f "${ZRBF_RBGY_BUILD_FILE}" || buc_die "RBGY build file not found in Tools: ${ZRBF_RBGY_BUILD_FILE}"
  test -f "${ZRBF_RBGY_COPY_FILE}"  || buc_die "RBGY copy file not found in Tools: ${ZRBF_RBGY_COPY_FILE}"

  buc_log_args 'Define temp files for build operations'
  ZRBF_BUILD_CONTEXT_TAR="${BUD_TEMP_DIR}/rbf_build_context.tar.gz"
  ZRBF_BUILD_CONFIG_FILE="${BUD_TEMP_DIR}/rbf_build_config.json"
  ZRBF_BUILD_ID_FILE="${BUD_TEMP_DIR}/rbf_build_id.txt"
  ZRBF_BUILD_STATUS_FILE="${BUD_TEMP_DIR}/rbf_build_status.json"
  ZRBF_BUILD_LOG_FILE="${BUD_TEMP_DIR}/rbf_build_log.txt"
  ZRBF_BUILD_RESPONSE_FILE="${BUD_TEMP_DIR}/rbf_build_response.json"
  ZRBF_BUILD_HTTP_CODE="${BUD_TEMP_DIR}/rbf_build_http_code.txt"

  buc_log_args 'Define copy staging files'
  ZRBF_COPY_STAGING_DIR="${BUD_TEMP_DIR}/rbf_copy_staging"
  ZRBF_COPY_CONTEXT_TAR="${BUD_TEMP_DIR}/rbf_copy_context.tar.gz"

  buc_log_args 'Define git info files'
  ZRBF_GIT_INFO_FILE="${BUD_TEMP_DIR}/rbf_git_info.json"
  ZRBF_GIT_COMMIT_FILE="${BUD_TEMP_DIR}/rbf_git_commit.txt"
  ZRBF_GIT_BRANCH_FILE="${BUD_TEMP_DIR}/rbf_git_branch.txt"
  ZRBF_GIT_REPO_FILE="${BUD_TEMP_DIR}/rbf_git_repo_url.txt"
  ZRBF_GIT_UNTRACKED_FILE="${BUD_TEMP_DIR}/rbf_git_untracked.txt"
  ZRBF_GIT_REMOTE_FILE="${BUD_TEMP_DIR}/rbf_git_remote.txt"

  buc_log_args 'Define staging and size files'
  ZRBF_STAGING_DIR="${BUD_TEMP_DIR}/rbf_staging"
  ZRBF_CONTEXT_SIZE_FILE="${BUD_TEMP_DIR}/rbf_context_size_bytes.txt"

  buc_log_args 'Define validation files'
  ZRBF_STATUS_CHECK_FILE="${BUD_TEMP_DIR}/rbf_status_check.txt"
  ZRBF_BUILD_ID_TMP_FILE="${BUD_TEMP_DIR}/rbf_build_id_tmp.txt"

  buc_log_args 'Define delete operation files'
  ZRBF_DELETE_PREFIX="${BUD_TEMP_DIR}/rbf_delete_"
  ZRBF_TOKEN_FILE="${BUD_TEMP_DIR}/rbf_token.txt"

  buc_log_args 'Define copy operation files'
  ZRBF_COPY_CONFIG_FILE="${BUD_TEMP_DIR}/rbf_copy_config.json"
  ZRBF_COPY_RESPONSE_FILE="${BUD_TEMP_DIR}/rbf_copy_response.json"

  buc_log_args 'Vessel-related files'
  ZRBF_VESSEL_ENV_FILE="${BUD_TEMP_DIR}/rbf_vessel_env.txt"
  ZRBF_VESSEL_SIGIL_FILE="${BUD_TEMP_DIR}/rbf_vessel_sigil.txt"

  buc_log_args 'For now lets double check these'
  test -n "${RBRR_GCB_JQ_IMAGE_REF:-}"     || buc_die "RBRR_GCB_JQ_IMAGE_REF not set"
  test -n "${RBRR_GCB_SYFT_IMAGE_REF:-}"   || buc_die "RBRR_GCB_SYFT_IMAGE_REF not set"
  test -n "${RBRR_GCB_GCRANE_IMAGE_REF:-}" || buc_die "RBRR_GCB_GCRANE_IMAGE_REF not set"
  test -n "${RBRR_GCB_ORAS_IMAGE_REF:-}"   || buc_die "RBRR_GCB_ORAS_IMAGE_REF not set"

  ZRBF_KINDLED=1
}

zrbf_sentinel() {
  test "${ZRBF_KINDLED:-}" = "1" || buc_die "Module rbf not kindled - call zrbf_kindle first"
}

zrbf_load_vessel() {
  zrbf_sentinel

  local z_vessel_dir="$1"

  buc_log_args 'Validate vessel directory exists'
  test -d "${z_vessel_dir}" || buc_die "Vessel directory not found: ${z_vessel_dir}"

  buc_log_args 'Check for rbrv.env file'
  local z_vessel_env="${z_vessel_dir}/rbrv.env"
  test -f "${z_vessel_env}" || buc_die "Vessel configuration not found: ${z_vessel_env}"

  buc_log_args 'Source vessel configuration'
  source "${z_vessel_env}" || buc_die "Failed to source vessel config: ${z_vessel_env}"

  buc_log_args 'Validate vessel configuration'
  local z_validator_dir="${BASH_SOURCE[0]%/*}"
  source "${z_validator_dir}/rbrv_regime.sh" || buc_die "Failed to validate vessel configuration"

  buc_log_args 'Validate vessel directory matches sigil'
  local z_vessel_dir_clean="${z_vessel_dir%/}"  # Strip any trailing slash
  local z_dir_name="${z_vessel_dir_clean##*/}"  # Extract directory name
  buc_log_args "  z_vessel_dir = ${z_vessel_dir}"
  buc_log_args "  z_dir_name   = ${z_dir_name}"
  test "${z_dir_name}" = "${RBRV_SIGIL}" || buc_die "Vessel sigil '${RBRV_SIGIL}' does not match directory name '${z_dir_name}'"

  buc_log_args 'Validate vessel path matches expected pattern'
  local z_expected_vessel_dir="${RBRR_VESSEL_DIR}/${RBRV_SIGIL}"
  local z_vessel_realpath=""
  z_vessel_realpath=$(cd "${z_vessel_dir}" && pwd) || buc_die "Failed to resolve vessel directory path"
  local z_expected_realpath=""
  z_expected_realpath=$(cd "${z_expected_vessel_dir}" && pwd) || buc_die "Failed to resolve expected vessel path"
  test "${z_vessel_realpath}" = "${z_expected_realpath}" || buc_die "Vessel directory '${z_vessel_dir}' does not match expected location '${z_expected_vessel_dir}'"

  buc_log_args 'Store loaded vessel info for use by commands'
  echo "${RBRV_SIGIL}" > "${ZRBF_VESSEL_SIGIL_FILE}" || buc_die "Failed to store vessel sigil"

  buc_info "Loaded vessel: ${RBRV_SIGIL}"
}

zrbf_verify_git_clean() {
  zrbf_sentinel

  buc_step "Verifying git repository state"

  buc_log_args 'Check for uncommitted changes'
  git diff-index --quiet HEAD -- || buc_die "Uncommitted changes detected - commit or stash first"

  buc_log_args 'Check for untracked files'
  git ls-files --others --exclude-standard > "${ZRBF_GIT_UNTRACKED_FILE}" || buc_die "Failed to check untracked files"
  local z_untracked=""
  z_untracked=$(<"${ZRBF_GIT_UNTRACKED_FILE}") || buc_die "Failed to read untracked list"
  test -z "${z_untracked}" || buc_die "Untracked files present - commit or clean first"

  buc_log_args 'Check if all commits are pushed'
  git fetch --quiet || buc_die "git fetch failed"
  git rev-list @{u}..HEAD --count > "${ZRBF_GIT_UNTRACKED_FILE}" 2>/dev/null || echo "0" > "${ZRBF_GIT_UNTRACKED_FILE}"
  local z_unpushed=""
  z_unpushed=$(<"${ZRBF_GIT_UNTRACKED_FILE}") || buc_die "Failed to read ahead count"
  test "${z_unpushed}" -eq 0 || buc_die "Local commits not pushed (${z_unpushed} commits ahead)"

  buc_log_args 'Get git metadata'
  git rev-parse HEAD              > "${ZRBF_GIT_COMMIT_FILE}" || buc_die "Failed to get commit SHA"
  git rev-parse --abbrev-ref HEAD > "${ZRBF_GIT_BRANCH_FILE}" || buc_die "Failed to get branch name"

  buc_log_args 'Get first available remote'
  git remote | head -1 > "${ZRBF_GIT_REMOTE_FILE}" || buc_die "No git remotes configured"
  local z_remote=""
  z_remote=$(<"${ZRBF_GIT_REMOTE_FILE}") || buc_die "Failed to read remote name"
  test -n "${z_remote}" || buc_die "No git remotes found"

  buc_log_args 'Get repo URL from remote: ${z_remote}'
  git config --get "remote.${z_remote}.url" > "${ZRBF_GIT_REPO_FILE}" || buc_die "Failed to get repo URL"

  local z_commit=""
  local z_branch=""
  local z_repo_url=""
  z_commit=$(<"${ZRBF_GIT_COMMIT_FILE}") || buc_die "Failed to read commit"
  z_branch=$(<"${ZRBF_GIT_BRANCH_FILE}") || buc_die "Failed to read branch"
  z_repo_url=$(<"${ZRBF_GIT_REPO_FILE}") || buc_die "Failed to read repo url"

  test -n "${z_commit}"   || buc_die "Git commit is empty"
  test -n "${z_branch}"   || buc_die "Git branch is empty"
  test -n "${z_repo_url}" || buc_die "Git repo URL is empty"

  buc_log_args 'Extract owner/repo from URL (handles both HTTPS and SSH)'
  # Example HTTPS: https://github.com/owner/repo.git
  # Example SSH  : git@github.com:owner/repo.git
  local z_repo="${z_repo_url#*github.com[:/]}"
  z_repo="${z_repo%.git}"

  buc_log_args 'Write git info JSON'
  jq -n                        \
    --arg commit "${z_commit}" \
    --arg branch "${z_branch}" \
    --arg repo   "${z_repo}"   \
    '{"commit": $commit, "branch": $branch, "repo": $repo}' \
    > "${ZRBF_GIT_INFO_FILE}" || buc_die "Failed to write git info"

  buc_info "Git state clean - commit: ${z_commit:0:8} on ${z_branch}"
}

zrbf_package_context() {
  zrbf_sentinel

  local z_dockerfile="$1"
  local z_context_dir="$2"
  local z_yaml_file="$3"

  buc_step 'Packaging build context'

  buc_log_args 'Create temp directory for context'
  rm -rf "${ZRBF_STAGING_DIR}" || buc_warn "Failed to clean existing staging directory"
  mkdir -p "${ZRBF_STAGING_DIR}" || buc_die "Failed to create staging directory"

  buc_log_args 'Copy context to staging'
  cp -r "${z_context_dir}/." "${ZRBF_STAGING_DIR}/" || buc_die "Failed to copy context"

  buc_log_args 'Copy Dockerfile to context root if not already there'
  local z_dockerfile_name="${z_dockerfile##*/}"
  cp "${z_dockerfile}" "${ZRBF_STAGING_DIR}/${z_dockerfile_name}" || buc_die "Failed to copy Dockerfile"

  buc_log_args 'Copy RBGY YAML to context (fixed name: cloudbuild.yaml)'
  cp "${z_yaml_file}" "${ZRBF_STAGING_DIR}/cloudbuild.yaml" || buc_die "Failed to copy RBGY file"

  buc_log_args "Create tarball"
  tar -czf "${ZRBF_BUILD_CONTEXT_TAR}" -C "${ZRBF_STAGING_DIR}" . || buc_die "Failed to create context archive"

  buc_log_args "Clean up staging"
  rm -rf "${ZRBF_STAGING_DIR}" || buc_warn "Failed to cleanup staging directory"

  buc_log_args 'Compute archive size (bytes) using temp file'
  wc -c < "${ZRBF_BUILD_CONTEXT_TAR}" > "${ZRBF_CONTEXT_SIZE_FILE}" || buc_die "Failed to compute context size"
  local z_size_bytes=""
  z_size_bytes=$(<"${ZRBF_CONTEXT_SIZE_FILE}") || buc_die "Failed to read context size"
  test -n "${z_size_bytes}" || buc_die "Context size is empty"

  # Size policy: warn >1MB; die >10MB
  if test "${z_size_bytes}" -gt 10485760; then
    buc_die  "Context tarball too large: ${z_size_bytes} bytes (>10MB limit)"
  elif test "${z_size_bytes}" -gt 1048576; then
    buc_warn "Context tarball large: ${z_size_bytes} bytes (>1MB warning)"
  fi

  buc_info "Build context packaged: ${z_size_bytes} bytes"
}

zrbf_package_copy_context() {
  zrbf_sentinel
  buc_step 'Packaging copy context'

  rm -rf "${ZRBF_COPY_STAGING_DIR}" || buc_warn "Failed to clean existing copy staging directory"
  mkdir -p "${ZRBF_COPY_STAGING_DIR}" || buc_die "Failed to create copy staging directory"

  buc_log_args 'Copy rbgy_copy.yaml into the root as cloudbuild.yaml'
  cp "${ZRBF_RBGY_COPY_FILE}" "${ZRBF_COPY_STAGING_DIR}/cloudbuild.yaml" \
    || buc_die "Failed to copy RBGY copy YAML"

  tar -czf "${ZRBF_COPY_CONTEXT_TAR}" -C "${ZRBF_COPY_STAGING_DIR}" . \
    || buc_die "Failed to create copy context archive"

  rm -rf "${ZRBF_COPY_STAGING_DIR}" || buc_warn "Failed to cleanup copy staging directory"
}

zrbf_compose_tarball_name() {
  zrbf_sentinel
  test -s "${ZRBF_VESSEL_SIGIL_FILE}" || buc_die "Vessel sigil file missing"
  local z_sigil=""
  z_sigil=$(<"${ZRBF_VESSEL_SIGIL_FILE}") || buc_die "Failed to read vessel sigil"
  test -n "${z_sigil}" || buc_die "Empty vessel sigil"

  # Flat namespace (no subdirectories), BUD_NOW_STAMP for source artifact
  local z_name="${z_sigil}.${BUD_NOW_STAMP}.source.tar.gz"
  echo "${z_name}" > "${ZRBF_TARBALL_NAME_FILE}"      || buc_die "Failed to write tarball name"
  echo "${RBGD_GCS_BUCKET}/${z_name}" > "${ZRBF_GCS_OBJECT_FILE}" || buc_die "Failed to write bucket/object"
  buc_log_args "Tarball object: ${z_name}"
}

zrbf_upload_context_to_gcs() {
  zrbf_sentinel
  buc_step "Uploading build context tarball to GCS staging bucket"

  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  local z_obj_name=""
  z_obj_name=$(<"${ZRBF_TARBALL_NAME_FILE}") || buc_die "Missing tarball name"
  test -s "${ZRBF_BUILD_CONTEXT_TAR}" || buc_die "Context tar is empty"

  local z_url="${ZRBF_GCS_UPLOAD_BASE}/b/${RBGD_GCS_BUCKET}/o?uploadType=media&name=${z_obj_name}"

  curl -sS -X POST                                 \
    -H "Authorization: Bearer ${z_token}"          \
    -H "Content-Type: application/gzip"            \
    -H "Accept: application/json"                  \
    --data-binary @"${ZRBF_BUILD_CONTEXT_TAR}"     \
    -o "${ZRBF_GCS_UPLOAD_RESP}"                   \
    -w "%{http_code}"                              \
    "${z_url}" > "${ZRBF_GCS_UPLOAD_HTTP}" || buc_die "Upload request failed"

  local z_http=""
  z_http=$(<"${ZRBF_GCS_UPLOAD_HTTP}") || buc_die "No HTTP status from GCS upload"
  test "${z_http}" = "200" || buc_die "GCS upload failed (HTTP ${z_http})"

  buc_success "Uploaded: gs://${RBGD_GCS_BUCKET}/${z_obj_name}"
}

zrbf_compose_build_request_json() {
  zrbf_sentinel
  jq empty "${ZRBF_BUILD_CONFIG_FILE}" || buc_die "Build config is not valid JSON"

  local z_obj_name=""
  z_obj_name=$(<"${ZRBF_TARBALL_NAME_FILE}") || buc_die "Missing tarball name"

  jq -n --slurpfile sub "${ZRBF_BUILD_CONFIG_FILE}"            \
    --arg bucket "${RBGD_GCS_BUCKET}"                           \
    --arg object "${z_obj_name}"                                 \
    --arg sa     "${RBGD_MASON_EMAIL}"                           \
    --arg mtype  "${RBRR_GCB_MACHINE_TYPE}"                      \
    --arg to     "${RBRR_GCB_TIMEOUT}"                           \
    '{
      source: { storageSource: { bucket: $bucket, object: $object } },
      substitutions: ($sub[0].substitutions),
      options: { logging: "CLOUD_LOGGING_ONLY", machineType: $mtype },
      serviceAccount: $sa,
      timeout: $to
    }' > "${ZRBF_BUILD_REQUEST_FILE}" || buc_die "Failed to compose build request json"
}

zrbf_submit_build_json() {
  zrbf_sentinel
  buc_step 'Submitting build (JSON) to Google Cloud Build'

  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get GCB OAuth token"

  curl -sS -X POST                              \
    -H "Authorization: Bearer ${z_token}"       \
    -H "Content-Type: application/json"         \
    -H "Accept: application/json"               \
    --data-binary @"${ZRBF_BUILD_REQUEST_FILE}" \
    -o "${ZRBF_BUILD_RESPONSE_FILE}"            \
    -w "%{http_code}"                           \
    "${ZRBF_GCB_PROJECT_BUILDS_URL}" > "${ZRBF_BUILD_HTTP_CODE}" || buc_die "Build create request failed"

  local z_http=""
  z_http=$(<"${ZRBF_BUILD_HTTP_CODE}") || buc_die "No HTTP status from Cloud Build create"
  test "${z_http}" = "200" -o "${z_http}" = "201" || buc_die "Cloud Build create failed (HTTP ${z_http})"
  test -s "${ZRBF_BUILD_RESPONSE_FILE}" || buc_die "Empty Cloud Build response"

  # Accept Operation or Build; prefer metadata.build.id else .id
  local z_build_id=""
  z_build_id=$(jq -r '(.metadata.build.id // .id // empty)' "${ZRBF_BUILD_RESPONSE_FILE}") || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build id not found in response"
  echo "${z_build_id}" > "${ZRBF_BUILD_ID_FILE}" || buc_die "Failed to persist build id"

  local z_console_url="${ZRBF_CLOUD_QUERY_BASE}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"
}


zrbf_submit_copy() {
  zrbf_sentinel

  buc_die 'ELIDED FOR NOW'
}

zrbf_wait_build_completion() {
  zrbf_sentinel

  buc_step 'Waiting for build completion'

  local z_build_id=""
  z_build_id=$(<"${ZRBF_BUILD_ID_FILE}") || buc_die "No build ID found"
  test -n "${z_build_id}" || buc_die "Build ID file empty"

  buc_log_args 'Get fresh token for polling'
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get GCB OAuth token"

  local z_status="PENDING"
  local z_attempts=0
  local z_max_attempts=240  # 20 minutes with 5 second intervals

  while true; do
    case "${z_status}" in PENDING|QUEUED|WORKING) : ;; *) break;; esac
    sleep 5

    z_attempts=$((z_attempts + 1))
    test ${z_attempts} -le ${z_max_attempts} || buc_die "Build timeout after ${z_max_attempts} attempts"

    buc_log_args "Fetch build status (attempt ${z_attempts}/${z_max_attempts})"
    curl -s                                                \
         -H "Authorization: Bearer ${z_token}"             \
         "${ZRBF_GCB_PROJECT_BUILDS_URL}/${z_build_id}"    \
         > "${ZRBF_BUILD_STATUS_FILE}"                     \
      || buc_die "Failed to get build status"

    test -f "${ZRBF_BUILD_STATUS_FILE}" || buc_die "Build status file not created"
    test -s "${ZRBF_BUILD_STATUS_FILE}" || buc_die "Build status file is empty"

    jq -r '.status' "${ZRBF_BUILD_STATUS_FILE}" > "${ZRBF_STATUS_CHECK_FILE}" || buc_die "Failed to extract status"
    z_status=$(<"${ZRBF_STATUS_CHECK_FILE}") || buc_die "Failed to read status"
    test -n "${z_status}" || buc_die "Status is empty"

    buc_info "Build status: ${z_status} (attempt ${z_attempts}/${z_max_attempts})"
  done

  test "${z_status}" = "SUCCESS" || buc_die "Build failed with status: ${z_status}"

  buc_success 'Build completed successfully'
}

######################################################################
# External Functions (rbf_*)

rbf_build() {
  zrbf_sentinel

  local z_vessel_dir="${1:-}"

  # Documentation block
  buc_doc_brief "Build container image from vessel using Google Cloud Build"
  buc_doc_param "vessel_dir" "Path to vessel directory containing rbrv.env"
  buc_doc_shown || return 0

  buc_log_args "Validate parameters"
  test -n "${z_vessel_dir}" || buc_die "Vessel directory required"

  # Load and validate vessel
  zrbf_load_vessel "${z_vessel_dir}"

  buc_log_args "Verify vessel has conjuring configuration"
  test -n "${RBRV_CONJURE_DOCKERFILE:-}" || buc_die "Vessel '${RBRV_SIGIL}' is not configured for conjuring (no RBRV_CONJURE_DOCKERFILE)"
  test -n "${RBRV_CONJURE_BLDCONTEXT:-}" || buc_die "Vessel '${RBRV_SIGIL}' is not configured for conjuring (no RBRV_CONJURE_BLDCONTEXT)"

  buc_log_args "Resolve paths from vessel configuration"
  test -f "${RBRV_CONJURE_DOCKERFILE}" || buc_die "Dockerfile not found: ${RBRV_CONJURE_DOCKERFILE}"
  test -d "${RBRV_CONJURE_BLDCONTEXT}" || buc_die "Build context not found: ${RBRV_CONJURE_BLDCONTEXT}"

  buc_log_args "Enforce vessel binfmt policy"
  if test "${RBRV_CONJURE_BINFMT_POLICY}" = "forbid"; then
    local z_native="$(uname -s | tr A-Z a-z)/$(uname -m)"
    case " ${RBRV_CONJURE_PLATFORMS} " in
      *"${z_native}"*) : ;;  # native arch allowed
      *) buc_die "Vessel '${RBRV_SIGIL}' forbids binfmt but RBRV_CONJURE_PLATFORMS='${RBRV_CONJURE_PLATFORMS}'" ;;
    esac
  fi

  buc_log_args "Generate build tag using vessel sigil"
  local z_tag="${RBRV_SIGIL}.${BUD_NOW_STAMP}"

  buc_info "Building vessel image: ${RBRV_SIGIL} -> ${z_tag}"

  # Verify git state + capture metadata
  zrbf_verify_git_clean

  # Package build context (includes RBGY file as cloudbuild.yaml)
  zrbf_package_context "${RBRV_CONJURE_DOCKERFILE}" "${RBRV_CONJURE_BLDCONTEXT}" "${ZRBF_RBGY_BUILD_FILE}"

  # Prepare RBGY substitutions file
  local z_dockerfile_name="${RBRV_CONJURE_DOCKERFILE##*/}"

  buc_log_args 'Read git info from file'
  jq -r '.commit' "${ZRBF_GIT_INFO_FILE}" > "${ZRBF_GIT_COMMIT_FILE}" || buc_die "Failed to extract git commit"
  jq -r '.branch' "${ZRBF_GIT_INFO_FILE}" > "${ZRBF_GIT_BRANCH_FILE}" || buc_die "Failed to extract git branch"
  jq -r '.repo'   "${ZRBF_GIT_INFO_FILE}" > "${ZRBF_GIT_REPO_FILE}"   || buc_die "Failed to extract git repo"

  local z_git_commit=""
  local z_git_branch=""
  local z_git_repo=""
  z_git_commit=$(<"${ZRBF_GIT_COMMIT_FILE}") || buc_die "Failed to read git commit"
  z_git_branch=$(<"${ZRBF_GIT_BRANCH_FILE}") || buc_die "Failed to read git branch"
  z_git_repo=$(<"${ZRBF_GIT_REPO_FILE}")     || buc_die "Failed to read git repo"

  test -n "${z_git_commit}" || buc_die "Git commit is empty"
  test -n "${z_git_branch}" || buc_die "Git branch is empty"
  test -n "${z_git_repo}"   || buc_die "Git repo is empty"

  buc_log_args 'Create build config with RBGY substitutions'
  jq -n                                                       \
    --arg zjq_dockerfile     "${z_dockerfile_name}"             \
    --arg zjq_moniker        "${RBRV_SIGIL}"                    \
    --arg zjq_platforms      "${RBRV_CONJURE_PLATFORMS}"        \
    --arg zjq_gar_location   "${RBGD_GAR_LOCATION}"             \
    --arg zjq_gar_project    "${RBGD_GAR_PROJECT_ID}"           \
    --arg zjq_gar_repository "${RBRR_GAR_REPOSITORY}"           \
    --arg zjq_git_commit     "${z_git_commit}"                  \
    --arg zjq_git_branch     "${z_git_branch}"                  \
    --arg zjq_git_repo       "${z_git_repo}"                    \
    --arg zjq_machine_type   "${RBRR_GCB_MACHINE_TYPE}"         \
    --arg zjq_timeout        "${RBRR_GCB_TIMEOUT}"              \
    --arg zjq_jq_ref         "${RBRR_GCB_JQ_IMAGE_REF:-}"       \
    --arg zjq_syft_ref       "${RBRR_GCB_SYFT_IMAGE_REF:-}"     \
    '{
      substitutions: {
        _RBGY_DOCKERFILE:     $zjq_dockerfile,
        _RBGY_MONIKER:        $zjq_moniker,
        _RBGY_PLATFORMS:      $zjq_platforms,
        _RBGY_GAR_LOCATION:   $zjq_gar_location,
        _RBGY_GAR_PROJECT:    $zjq_gar_project,
        _RBGY_GAR_REPOSITORY: $zjq_gar_repository,
        _RBGY_GIT_COMMIT:     $zjq_git_commit,
        _RBGY_GIT_BRANCH:     $zjq_git_branch,
        _RBGY_GIT_REPO:       $zjq_git_repo,
        _RBGY_MACHINE_TYPE:   $zjq_machine_type,
        _RBGY_TIMEOUT:        $zjq_timeout,
        _RBGY_JQ_REF:         $zjq_jq_ref,
        _RBGY_SYFT_REF:       $zjq_syft_ref
      }
    }' >  "${ZRBF_BUILD_CONFIG_FILE}" || buc_die "Failed to create build config"

  # Stage & submit new flow
  zrbf_compose_tarball_name
  zrbf_upload_context_to_gcs
  zrbf_compose_build_request_json
  zrbf_submit_build_json

  # Wait for completion (5s x 240 = 20m, no backoff)
  zrbf_wait_build_completion

  buc_success "Vessel image built: ${z_tag}"
}

rbf_copy() {
  zrbf_sentinel

  buc_die 'ELIDED FOR NOW'
}

rbf_study() {
  zrbf_sentinel

  buc_doc_brief "Run minimal Cloud Build multipart study in-place"
  buc_doc_shown || return 0

  buc_step "Mint Cloud Build OAuth token (Director)"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get GCB OAuth token"

  zrbgc_sentinel

  buc_step "Execute study script from its own directory with parameters"
  local z_tools_dir="${BASH_SOURCE[0]%/*}"
  local z_repo_root="${z_tools_dir%/*}"
  local z_study_dir="${z_repo_root}/Study/study-gcb-build-submit-debug"
  local z_script="${z_study_dir}/sgbs-debug.sh"
  test -f "${z_script}" || buc_die "Study script not found: ${z_script}"

  ( cd "${z_study_dir}" && ./sgbs-debug.sh "${z_token}" ) || buc_die "Study script failed"

  buc_success "Study run completed"
}

rbf_delete() {
  zrbf_sentinel

  local z_tag="${1:-}"

  # Documentation block
  buc_doc_brief "Delete an image tag from the registry"
  buc_doc_param "tag" "Image tag to delete"
  buc_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || buc_die "Tag parameter required"

  buc_step "Fetching manifest digest for deletion"

  # Get OAuth token using Director credentials
  # Note: This requires GCB service account to have artifactregistry.repoAdmin role
  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"
  echo "${z_token}" > "${ZRBF_TOKEN_FILE}" || buc_die "Failed to write token file"

  # Get manifest with digest header
  local z_manifest_headers="${ZRBF_DELETE_PREFIX}headers.txt"

  curl -sL -I                                         \
    -H "Authorization: Bearer ${z_token}"             \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}"       \
    "${ZRBF_REGISTRY_API_BASE}/manifests/${z_tag}"    \
    > "${z_manifest_headers}" || buc_die "Failed to fetch manifest headers"

  # Extract digest from Docker-Content-Digest header
  local z_digest_file="${ZRBF_DELETE_PREFIX}digest.txt"
  grep -i "docker-content-digest:" "${z_manifest_headers}" | \
    sed 's/.*: //' | tr -d '\r\n' > "${z_digest_file}" || buc_die "Failed to extract digest header"

  local z_digest
  z_digest=$(<"${z_digest_file}")
  test -n "${z_digest}" || buc_die "Manifest digest is empty"

  buc_info "Deleting manifest: ${z_digest}"

  # Delete by digest
  local z_status_file="${ZRBF_DELETE_PREFIX}status.txt"

  curl -X DELETE -s                                   \
    -H "Authorization: Bearer ${z_token}"             \
    -w "%{http_code}"                                 \
    -o /dev/null                                      \
    "${ZRBF_REGISTRY_API_BASE}/manifests/${z_digest}" \
    > "${z_status_file}" || buc_die "DELETE request failed"

  local z_http_code
  z_http_code=$(<"${z_status_file}")
  test -n "${z_http_code}" || buc_die "HTTP status code is empty"
  test "${z_http_code}" = "202" || test "${z_http_code}" = "204" || \
    buc_die "Delete failed with HTTP ${z_http_code}"

  buc_success "Image deleted: ${z_tag}"
}

# eof

