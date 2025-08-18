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
test -z "${ZRBF_SOURCED:-}" || bcu_die "Module rbf multiply sourced - check sourcing hierarchy"
ZRBF_SOURCED=1

######################################################################
# Internal Functions (zrbf_*)

zrbf_kindle() {
  test -z "${ZRBF_KINDLED:-}" || bcu_die "Module rbf already kindled"

  # Validate environment
  bvu_dir_exists "${BDU_TEMP_DIR}"
  test -n "${BDU_NOW_STAMP:-}" || bcu_die "BDU_NOW_STAMP is unset or empty"

  bcu_log_args 'Check required GCB/GAR environment variables'
  test -n "${RBRR_GCB_PROJECT_ID:-}"      || bcu_die "RBRR_GCB_PROJECT_ID not set"
  test -n "${RBRR_GCB_REGION:-}"          || bcu_die "RBRR_GCB_REGION not set"
  test -n "${RBRR_GAR_PROJECT_ID:-}"      || bcu_die "RBRR_GAR_PROJECT_ID not set"
  test -n "${RBRR_GAR_LOCATION:-}"        || bcu_die "RBRR_GAR_LOCATION not set"
  test -n "${RBRR_GAR_REPOSITORY:-}"      || bcu_die "RBRR_GAR_REPOSITORY not set"
  test -n "${RBRR_VESSEL_DIR:-}"          || bcu_die "RBRR_VESSEL_DIR not set"

  bcu_log_args 'Verify service account files'
  test -n "${RBRR_DIRECTOR_RBRA_FILE:-}" || bcu_die "RBRR_DIRECTOR_RBRA_FILE not set"
  test -f "${RBRR_DIRECTOR_RBRA_FILE}"   || bcu_die "GCB service env file not found: ${RBRR_DIRECTOR_RBRA_FILE}"

  bcu_log_args 'Module Variables (ZRBF_*)'
  ZRBF_GCB_API_BASE="https://cloudbuild.googleapis.com/v1"
  ZRBF_GAR_API_BASE="https://artifactregistry.googleapis.com/v1"
  ZRBF_CLOUD_QUERY_BASE="https://console.cloud.google.com/cloud-build/builds"

  ZRBF_GCB_API_BASE_UPLOAD="https://cloudbuild.googleapis.com/upload/v1"

  ZRBF_GCB_PROJECT_BUILDS_URL="${ZRBF_GCB_API_BASE}/projects/${RBRR_GCB_PROJECT_ID}/locations/${RBRR_GCB_REGION}/builds"
  ZRBF_GCB_PROJECT_BUILDS_UPLOAD_URL="${ZRBF_GCB_API_BASE_UPLOAD}/projects/${RBRR_GCB_PROJECT_ID}/locations/${RBRR_GCB_REGION}/builds"
  ZRBF_GAR_PACKAGE_BASE="projects/${RBRR_GAR_PROJECT_ID}/locations/${RBRR_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}"

  bcu_log_args 'Registry API endpoints for delete'
  ZRBF_REGISTRY_HOST="${RBRR_GAR_LOCATION}-docker.pkg.dev"
  ZRBF_REGISTRY_PATH="${RBRR_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  ZRBF_REGISTRY_API_BASE="https://${ZRBF_REGISTRY_HOST}/v2/${ZRBF_REGISTRY_PATH}"

  bcu_log_args 'Media types for delete operation'
  ZRBF_ACCEPT_MANIFEST_MTYPES="application/vnd.docker.distribution.manifest.v2+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.oci.image.index.v1+json,application/vnd.oci.image.manifest.v1+json"

  bcu_log_args 'RBGY files presumed to be in the same Tools directory as this implementation'
  local z_self_dir="${BASH_SOURCE[0]%/*}"
  ZRBF_RBGY_BUILD_FILE="${z_self_dir}/rbgy_build.yaml"
  ZRBF_RBGY_COPY_FILE="${z_self_dir}/rbgy_copy.yaml"
  test -f "${ZRBF_RBGY_BUILD_FILE}" || bcu_die "RBGY build file not found in Tools: ${ZRBF_RBGY_BUILD_FILE}"
  test -f "${ZRBF_RBGY_COPY_FILE}"  || bcu_die "RBGY copy file not found in Tools: ${ZRBF_RBGY_COPY_FILE}"

  bcu_log_args 'Define temp files for build operations'
  ZRBF_BUILD_CONTEXT_TAR="${BDU_TEMP_DIR}/rbf_build_context.tar.gz"
  ZRBF_BUILD_CONFIG_FILE="${BDU_TEMP_DIR}/rbf_build_config.json"
  ZRBF_BUILD_ID_FILE="${BDU_TEMP_DIR}/rbf_build_id.txt"
  ZRBF_BUILD_STATUS_FILE="${BDU_TEMP_DIR}/rbf_build_status.json"
  ZRBF_BUILD_LOG_FILE="${BDU_TEMP_DIR}/rbf_build_log.txt"
  ZRBF_BUILD_RESPONSE_FILE="${BDU_TEMP_DIR}/rbf_build_response.json"
  ZRBF_BUILD_HTTP_CODE="${BDU_TEMP_DIR}/rbf_build_http_code.txt"

  bcu_log_args 'Define copy staging files'
  ZRBF_COPY_STAGING_DIR="${BDU_TEMP_DIR}/rbf_copy_staging"
  ZRBF_COPY_CONTEXT_TAR="${BDU_TEMP_DIR}/rbf_copy_context.tar.gz"

  bcu_log_args 'Define git info files'
  ZRBF_GIT_INFO_FILE="${BDU_TEMP_DIR}/rbf_git_info.json"
  ZRBF_GIT_COMMIT_FILE="${BDU_TEMP_DIR}/rbf_git_commit.txt"
  ZRBF_GIT_BRANCH_FILE="${BDU_TEMP_DIR}/rbf_git_branch.txt"
  ZRBF_GIT_REPO_FILE="${BDU_TEMP_DIR}/rbf_git_repo_url.txt"
  ZRBF_GIT_UNTRACKED_FILE="${BDU_TEMP_DIR}/rbf_git_untracked.txt"
  ZRBF_GIT_REMOTE_FILE="${BDU_TEMP_DIR}/rbf_git_remote.txt"

  bcu_log_args 'Define staging and size files'
  ZRBF_STAGING_DIR="${BDU_TEMP_DIR}/rbf_staging"
  ZRBF_CONTEXT_SIZE_FILE="${BDU_TEMP_DIR}/rbf_context_size_bytes.txt"

  bcu_log_args 'Define validation files'
  ZRBF_STATUS_CHECK_FILE="${BDU_TEMP_DIR}/rbf_status_check.txt"
  ZRBF_BUILD_ID_TMP_FILE="${BDU_TEMP_DIR}/rbf_build_id_tmp.txt"

  bcu_log_args 'Define delete operation files'
  ZRBF_DELETE_PREFIX="${BDU_TEMP_DIR}/rbf_delete_"
  ZRBF_TOKEN_FILE="${BDU_TEMP_DIR}/rbf_token.txt"

  bcu_log_args 'Define copy operation files'
  ZRBF_COPY_CONFIG_FILE="${BDU_TEMP_DIR}/rbf_copy_config.json"
  ZRBF_COPY_RESPONSE_FILE="${BDU_TEMP_DIR}/rbf_copy_response.json"

  bcu_log_args 'Vessel-related files'
  ZRBF_VESSEL_ENV_FILE="${BDU_TEMP_DIR}/rbf_vessel_env.txt"
  ZRBF_VESSEL_SIGIL_FILE="${BDU_TEMP_DIR}/rbf_vessel_sigil.txt"

  bcu_log_args 'For now lets double check these'
  test -n "${RBRR_GCB_JQ_IMAGE_REF:-}"     || bcu_die "RBRR_GCB_JQ_IMAGE_REF not set"
  test -n "${RBRR_GCB_SYFT_IMAGE_REF:-}"   || bcu_die "RBRR_GCB_SYFT_IMAGE_REF not set"
  test -n "${RBRR_GCB_GCRANE_IMAGE_REF:-}" || bcu_die "RBRR_GCB_GCRANE_IMAGE_REF not set"
  test -n "${RBRR_GCB_ORAS_IMAGE_REF:-}"   || bcu_die "RBRR_GCB_ORAS_IMAGE_REF not set"

  ZRBF_KINDLED=1
}

zrbf_sentinel() {
  test "${ZRBF_KINDLED:-}" = "1" || bcu_die "Module rbf not kindled - call zrbf_kindle first"
}

zrbf_load_vessel() {
  zrbf_sentinel

  local z_vessel_dir="$1"

  bcu_log_args 'Validate vessel directory exists'
  test -d "${z_vessel_dir}" || bcu_die "Vessel directory not found: ${z_vessel_dir}"

  bcu_log_args 'Check for rbrv.env file'
  local z_vessel_env="${z_vessel_dir}/rbrv.env"
  test -f "${z_vessel_env}" || bcu_die "Vessel configuration not found: ${z_vessel_env}"

  bcu_log_args 'Source vessel configuration'
  source "${z_vessel_env}" || bcu_die "Failed to source vessel config: ${z_vessel_env}"

  bcu_log_args 'Validate vessel configuration'
  local z_validator_dir="${BASH_SOURCE[0]%/*}"
  source "${z_validator_dir}/rbrv.validator.sh" || bcu_die "Failed to validate vessel configuration"

  bcu_log_args 'Validate vessel directory matches sigil'
  local z_vessel_dir_clean="${z_vessel_dir%/}"  # Strip any trailing slash
  local z_dir_name="${z_vessel_dir_clean##*/}"  # Extract directory name
  bcu_log_args "  z_vessel_dir = ${z_vessel_dir}"
  bcu_log_args "  z_dir_name   = ${z_dir_name}"
  test "${z_dir_name}" = "${RBRV_SIGIL}" || bcu_die "Vessel sigil '${RBRV_SIGIL}' does not match directory name '${z_dir_name}'"

  bcu_log_args 'Validate vessel path matches expected pattern'
  local z_expected_vessel_dir="${RBRR_VESSEL_DIR}/${RBRV_SIGIL}"
  local z_vessel_realpath=""
  z_vessel_realpath=$(cd "${z_vessel_dir}" && pwd) || bcu_die "Failed to resolve vessel directory path"
  local z_expected_realpath=""
  z_expected_realpath=$(cd "${z_expected_vessel_dir}" && pwd) || bcu_die "Failed to resolve expected vessel path"
  test "${z_vessel_realpath}" = "${z_expected_realpath}" || bcu_die "Vessel directory '${z_vessel_dir}' does not match expected location '${z_expected_vessel_dir}'"

  bcu_log_args 'Store loaded vessel info for use by commands'
  echo "${RBRV_SIGIL}" > "${ZRBF_VESSEL_SIGIL_FILE}" || bcu_die "Failed to store vessel sigil"

  bcu_info "Loaded vessel: ${RBRV_SIGIL}"
}

zrbf_verify_git_clean() {
  zrbf_sentinel

  bcu_step "Verifying git repository state"

  bcu_log_args 'Check for uncommitted changes'
  git diff-index --quiet HEAD -- || bcu_die "Uncommitted changes detected - commit or stash first"

  bcu_log_args 'Check for untracked files'
  git ls-files --others --exclude-standard > "${ZRBF_GIT_UNTRACKED_FILE}" || bcu_die "Failed to check untracked files"
  local z_untracked=""
  z_untracked=$(<"${ZRBF_GIT_UNTRACKED_FILE}") || bcu_die "Failed to read untracked list"
  test -z "${z_untracked}" || bcu_die "Untracked files present - commit or clean first"

  bcu_log_args 'Check if all commits are pushed'
  git fetch --quiet || bcu_die "git fetch failed"
  git rev-list @{u}..HEAD --count > "${ZRBF_GIT_UNTRACKED_FILE}" 2>/dev/null || echo "0" > "${ZRBF_GIT_UNTRACKED_FILE}"
  local z_unpushed=""
  z_unpushed=$(<"${ZRBF_GIT_UNTRACKED_FILE}") || bcu_die "Failed to read ahead count"
  test "${z_unpushed}" -eq 0 || bcu_die "Local commits not pushed (${z_unpushed} commits ahead)"

  bcu_log_args 'Get git metadata'
  git rev-parse HEAD              > "${ZRBF_GIT_COMMIT_FILE}" || bcu_die "Failed to get commit SHA"
  git rev-parse --abbrev-ref HEAD > "${ZRBF_GIT_BRANCH_FILE}" || bcu_die "Failed to get branch name"

  bcu_log_args 'Get first available remote'
  git remote | head -1 > "${ZRBF_GIT_REMOTE_FILE}" || bcu_die "No git remotes configured"
  local z_remote=""
  z_remote=$(<"${ZRBF_GIT_REMOTE_FILE}") || bcu_die "Failed to read remote name"
  test -n "${z_remote}" || bcu_die "No git remotes found"

  bcu_log_args 'Get repo URL from remote: ${z_remote}'
  git config --get "remote.${z_remote}.url" > "${ZRBF_GIT_REPO_FILE}" || bcu_die "Failed to get repo URL"

  local z_commit=""
  local z_branch=""
  local z_repo_url=""
  z_commit=$(<"${ZRBF_GIT_COMMIT_FILE}") || bcu_die "Failed to read commit"
  z_branch=$(<"${ZRBF_GIT_BRANCH_FILE}") || bcu_die "Failed to read branch"
  z_repo_url=$(<"${ZRBF_GIT_REPO_FILE}") || bcu_die "Failed to read repo url"

  test -n "${z_commit}"   || bcu_die "Git commit is empty"
  test -n "${z_branch}"   || bcu_die "Git branch is empty"
  test -n "${z_repo_url}" || bcu_die "Git repo URL is empty"

  bcu_log_args 'Extract owner/repo from URL (handles both HTTPS and SSH)'
  # Example HTTPS: https://github.com/owner/repo.git
  # Example SSH  : git@github.com:owner/repo.git
  local z_repo="${z_repo_url#*github.com[:/]}"
  z_repo="${z_repo%.git}"

  bcu_log_args 'Write git info JSON'
  jq -n                        \
    --arg commit "${z_commit}" \
    --arg branch "${z_branch}" \
    --arg repo   "${z_repo}"   \
    '{"commit": $commit, "branch": $branch, "repo": $repo}' \
    > "${ZRBF_GIT_INFO_FILE}" || bcu_die "Failed to write git info"

  bcu_info "Git state clean - commit: ${z_commit:0:8} on ${z_branch}"
}

zrbf_package_context() {
  zrbf_sentinel

  local z_dockerfile="$1"
  local z_context_dir="$2"
  local z_yaml_file="$3"

  bcu_step 'Packaging build context'

  bcu_log_args 'Create temp directory for context'
  rm -rf "${ZRBF_STAGING_DIR}" || bcu_warn "Failed to clean existing staging directory"
  mkdir -p "${ZRBF_STAGING_DIR}" || bcu_die "Failed to create staging directory"

  bcu_log_args 'Copy context to staging'
  cp -r "${z_context_dir}/." "${ZRBF_STAGING_DIR}/" || bcu_die "Failed to copy context"

  bcu_log_args 'Copy Dockerfile to context root if not already there'
  local z_dockerfile_name="${z_dockerfile##*/}"
  cp "${z_dockerfile}" "${ZRBF_STAGING_DIR}/${z_dockerfile_name}" || bcu_die "Failed to copy Dockerfile"

  bcu_log_args 'Copy RBGY YAML to context (fixed name: cloudbuild.yaml)'
  cp "${z_yaml_file}" "${ZRBF_STAGING_DIR}/cloudbuild.yaml" || bcu_die "Failed to copy RBGY file"

  bcu_log_args "Create tarball"
  tar -czf "${ZRBF_BUILD_CONTEXT_TAR}" -C "${ZRBF_STAGING_DIR}" . || bcu_die "Failed to create context archive"

  bcu_log_args "Clean up staging"
  rm -rf "${ZRBF_STAGING_DIR}" || bcu_warn "Failed to cleanup staging directory"

  bcu_log_args 'Compute archive size (bytes) using temp file'
  wc -c < "${ZRBF_BUILD_CONTEXT_TAR}" > "${ZRBF_CONTEXT_SIZE_FILE}" || bcu_die "Failed to compute context size"
  local z_size_bytes=""
  z_size_bytes=$(<"${ZRBF_CONTEXT_SIZE_FILE}") || bcu_die "Failed to read context size"
  test -n "${z_size_bytes}" || bcu_die "Context size is empty"

  bcu_info "Build context packaged: ${z_size_bytes} bytes"
}

zrbf_package_copy_context() {
  zrbf_sentinel
  bcu_step 'Packaging copy context'

  rm -rf "${ZRBF_COPY_STAGING_DIR}" || bcu_warn "Failed to clean existing copy staging directory"
  mkdir -p "${ZRBF_COPY_STAGING_DIR}" || bcu_die "Failed to create copy staging directory"

  bcu_log_args 'Copy rbgy_copy.yaml into the root as cloudbuild.yaml'
  cp "${ZRBF_RBGY_COPY_FILE}" "${ZRBF_COPY_STAGING_DIR}/cloudbuild.yaml" \
    || bcu_die "Failed to copy RBGY copy YAML"

  tar -czf "${ZRBF_COPY_CONTEXT_TAR}" -C "${ZRBF_COPY_STAGING_DIR}" . \
    || bcu_die "Failed to create copy context archive"

  rm -rf "${ZRBF_COPY_STAGING_DIR}" || bcu_warn "Failed to cleanup copy staging directory"
}

zrbf_submit_build() {
  zrbf_sentinel

  local z_dockerfile_name="$1"
  local z_tag="$2"
  local z_sigil="$3"

  bcu_step 'Submitting build to Google Cloud Build'

  bcu_log_args 'Get OAuth token using capture function'
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || bcu_die "Failed to get GCB OAuth token"

  bcu_log_args 'Read git info from file'
  jq -r '.commit' "${ZRBF_GIT_INFO_FILE}" > "${ZRBF_GIT_COMMIT_FILE}" || bcu_die "Failed to extract git commit"
  jq -r '.branch' "${ZRBF_GIT_INFO_FILE}" > "${ZRBF_GIT_BRANCH_FILE}" || bcu_die "Failed to extract git branch"
  jq -r '.repo'   "${ZRBF_GIT_INFO_FILE}" > "${ZRBF_GIT_REPO_FILE}"   || bcu_die "Failed to extract git repo"

  local z_git_commit=""
  local z_git_branch=""
  local z_git_repo=""
  z_git_commit=$(<"${ZRBF_GIT_COMMIT_FILE}") || bcu_die "Failed to read git commit"
  z_git_branch=$(<"${ZRBF_GIT_BRANCH_FILE}") || bcu_die "Failed to read git branch"
  z_git_repo=$(<"${ZRBF_GIT_REPO_FILE}")     || bcu_die "Failed to read git repo"

  test -n "${z_git_commit}" || bcu_die "Git commit is empty"
  test -n "${z_git_branch}" || bcu_die "Git branch is empty"
  test -n "${z_git_repo}"   || bcu_die "Git repo is empty"

  bcu_log_args 'Extract recipe name without extension'
  local z_recipe_name="${z_dockerfile_name%.*}"

  bcu_log_args 'Create build config with RBGY substitutions'
    jq -n                                                       \
    --arg zjq_dockerfile     "${z_dockerfile_name}"             \
    --arg zjq_tag            "${z_tag}"                         \
    --arg zjq_moniker        "${z_sigil}"                       \
    --arg zjq_platforms      "${RBRV_CONJURE_PLATFORMS}"        \
    --arg zjq_gar_location   "${RBRR_GAR_LOCATION}"             \
    --arg zjq_gar_project    "${RBRR_GAR_PROJECT_ID}"           \
    --arg zjq_gar_repository "${RBRR_GAR_REPOSITORY}"           \
    --arg zjq_git_commit     "${z_git_commit}"                  \
    --arg zjq_git_branch     "${z_git_branch}"                  \
    --arg zjq_git_repo       "${z_git_repo}"                    \
    --arg zjq_machine_type   "${RBRR_GCB_MACHINE_TYPE}"         \
    --arg zjq_timeout        "${RBRR_GCB_TIMEOUT}"              \
    --arg zjq_recipe_name    "${z_recipe_name}"                 \
    --arg zjq_jq_ref         "${RBRR_GCB_JQ_IMAGE_REF:-}"       \
    --arg zjq_syft_ref       "${RBRR_GCB_SYFT_IMAGE_REF:-}"     \
    --arg zjq_gcrane_ref     "${RBRR_GCB_GCRANE_IMAGE_REF:-}"   \
    --arg zjq_oras_ref       "${RBRR_GCB_ORAS_IMAGE_REF:-}"     \
    '{
      substitutions: {
        _RBGY_DOCKERFILE:     $zjq_dockerfile,
        _RBGY_TAG:            $zjq_tag,
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
        _RBGY_RECIPE_NAME:    $zjq_recipe_name,
        _RBGY_JQ_REF:         $zjq_jq_ref,
        _RBGY_SYFT_REF:       $zjq_syft_ref,
        _RBGY_GCRANE_REF:     $zjq_gcrane_ref,
        _RBGY_ORAS_REF:       $zjq_oras_ref
      }
    }' >  "${ZRBF_BUILD_CONFIG_FILE}" || bcu_die "Failed to create build config"
  test -s "${ZRBF_BUILD_CONFIG_FILE}" || bcu_die "Build config file is empty"

  bcu_log_args 'Build a multipart/related body: part 1 = build JSON, part 2 = tar.gz'
  local z_boundary="__rbf_cb_$$_${BDU_NOW_STAMP}"
  local z_body_file="${BDU_TEMP_DIR}/rbf_build_mpart.body"
  : > "${z_body_file}"

  {
    printf -- "--%s\r\n" "${z_boundary}"
    printf "Content-Type: application/json; charset=UTF-8\r\n\r\n"
    cat "${ZRBF_BUILD_CONFIG_FILE}"
    printf "\r\n--%s\r\n" "${z_boundary}"
    printf "Content-Type: application/octet-stream\r\n\r\n"
    cat "${ZRBF_BUILD_CONTEXT_TAR}"
    printf "\r\n--%s--\r\n" "${z_boundary}"
  } >> "${z_body_file}"

  bcu_log_args 'Submit build'
  local z_resp_headers="${BDU_TEMP_DIR}/rbf_build_http_headers.txt"

  curl -sS -X POST \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: application/json" \
    -F "build=@${ZRBF_BUILD_CONFIG_FILE};type=application/json" \
    -F "source=@${ZRBF_BUILD_CONTEXT_TAR};type=application/octet-stream" \
    -D "${z_resp_headers}" \
    -o "${ZRBF_BUILD_RESPONSE_FILE}" \
    -w "%{http_code}" \
    "${ZRBF_GCB_PROJECT_BUILDS_UPLOAD_URL}?uploadType=multipart" > "${ZRBF_BUILD_HTTP_CODE}"

  z_http=$(<"${ZRBF_BUILD_HTTP_CODE}")
  test -n "${z_http}"                   || bcu_die "No HTTP status from Cloud Build create"
  test -s "${ZRBF_BUILD_RESPONSE_FILE}" || bcu_die "Empty Cloud Build response"

  bcu_log_args 'If not 200 OK, show the API error and stop BEFORE making a Console URL'
  if [ "${z_http}" != "200" ]; then
    bcu_log_args "Response headers:"
    bcu_log_pipe < "${z_resp_headers}"

    bcu_log_args 'Try to parse JSON error; if it fails, show a raw tail of the body'
    if ! z_err=$(jq -r 'if .error then (.error.message // "Unknown error") else "Unknown error (no .error in response)" end' "${ZRBF_BUILD_RESPONSE_FILE}" 2>/dev/null); then
      bcu_log_args "Non-JSON response body (tail, 1KB):"
      tail -c 1024 "${ZRBF_BUILD_RESPONSE_FILE}" | bcu_log_pipe
      z_err="Unknown error (non-JSON response)"
    else
      bcu_log_args "Raw response body on error follows:"
      bcu_log_pipe < "${ZRBF_BUILD_RESPONSE_FILE}"
    fi
    bcu_die "Cloud Build create failed (HTTP ${z_http}): ${z_err}"
  fi

  bcu_log_args 'Parse Long-Running Operation (LRO)'
  jq -r '.name // empty' "${ZRBF_BUILD_RESPONSE_FILE}" > "${BDU_TEMP_DIR}/rbf_operation_name.txt"

  bcu_log_args 'Build ID is under operation.metadata.build.id'
  jq -r '.metadata.build.id // empty' "${ZRBF_BUILD_RESPONSE_FILE}" > "${ZRBF_BUILD_ID_FILE}"

  z_build_id=$(<"${ZRBF_BUILD_ID_FILE}")
  test -n "${z_build_id}" || bcu_die "Cloud Build did not return a build id in operation.metadata.build.id"

  bcu_log_args 'Now make the Console link with a real BUILD ID'
  local z_console_url="${ZRBF_CLOUD_QUERY_BASE}/${z_build_id}?project=${RBRR_GCB_PROJECT_ID}"
  bcu_info "Build submitted: ${z_build_id}"
  bcu_link "Click to " "Open build in Cloud Console" "${z_console_url}"
}

zrbf_submit_copy() {
  zrbf_sentinel

  bcu_die 'ELIDED FOR NOW'
}

zrbf_wait_build_completion() {
  zrbf_sentinel

  bcu_step 'Waiting for build completion'

  local z_build_id=""
  z_build_id=$(<"${ZRBF_BUILD_ID_FILE}") || bcu_die "No build ID found"
  test -n "${z_build_id}" || bcu_die "Build ID file empty"

  bcu_log_args 'Get fresh token for polling'
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || bcu_die "Failed to get GCB OAuth token"

  local z_status="PENDING"
  local z_attempts=0
  local z_max_attempts=240  # 20 minutes with 5 second intervals

  while true; do
    case "${z_status}" in PENDING|QUEUED|WORKING) : ;; *) break;; esac
    sleep 5

    z_attempts=$((z_attempts + 1))
    test ${z_attempts} -le ${z_max_attempts} || bcu_die "Build timeout after ${z_max_attempts} attempts"

    bcu_log_args "Fetch build status (attempt ${z_attempts}/${z_max_attempts})"
    curl -s                                                \
         -H "Authorization: Bearer ${z_token}"             \
         "${ZRBF_GCB_PROJECT_BUILDS_URL}/${z_build_id}"    \
         > "${ZRBF_BUILD_STATUS_FILE}"                     \
      || bcu_die "Failed to get build status"

    test -f "${ZRBF_BUILD_STATUS_FILE}" || bcu_die "Build status file not created"
    test -s "${ZRBF_BUILD_STATUS_FILE}" || bcu_die "Build status file is empty"

    jq -r '.status' "${ZRBF_BUILD_STATUS_FILE}" > "${ZRBF_STATUS_CHECK_FILE}" || bcu_die "Failed to extract status"
    z_status=$(<"${ZRBF_STATUS_CHECK_FILE}") || bcu_die "Failed to read status"
    test -n "${z_status}" || bcu_die "Status is empty"

    bcu_info "Build status: ${z_status} (attempt ${z_attempts}/${z_max_attempts})"
  done

  test "${z_status}" = "SUCCESS" || bcu_die "Build failed with status: ${z_status}"

  bcu_success 'Build completed successfully'
}

######################################################################
# External Functions (rbf_*)

rbf_build() {
  zrbf_sentinel

  local z_vessel_dir="${1:-}"

  # Documentation block
  bcu_doc_brief "Build container image from vessel using Google Cloud Build"
  bcu_doc_param "vessel_dir" "Path to vessel directory containing rbrv.env"
  bcu_doc_shown || return 0

  bcu_log_args "Validate parameters"
  test -n "${z_vessel_dir}" || bcu_die "Vessel directory required"

  # Load and validate vessel
  zrbf_load_vessel "${z_vessel_dir}"

  bcu_log_args "Verify vessel has conjuring configuration"
  test -n "${RBRV_CONJURE_DOCKERFILE:-}" || bcu_die "Vessel '${RBRV_SIGIL}' is not configured for conjuring (no RBRV_CONJURE_DOCKERFILE)"
  test -n "${RBRV_CONJURE_BLDCONTEXT:-}" || bcu_die "Vessel '${RBRV_SIGIL}' is not configured for conjuring (no RBRV_CONJURE_BLDCONTEXT)"

  bcu_log_args "Resolve paths from vessel configuration"
  test -f "${RBRV_CONJURE_DOCKERFILE}" || bcu_die "Dockerfile not found: ${RBRV_CONJURE_DOCKERFILE}"
  test -d "${RBRV_CONJURE_BLDCONTEXT}" || bcu_die "Build context not found: ${RBRV_CONJURE_BLDCONTEXT}"

  bcu_log_args "Enforce vessel binfmt policy"
  if test "${RBRV_CONJURE_BINFMT_POLICY}" = "forbid"; then
    local z_native="$(uname -s | tr A-Z a-z)/$(uname -m)"
    case " ${RBRV_CONJURE_PLATFORMS} " in
      *"${z_native}"*) : ;;  # native arch allowed
      *) bcu_die "Vessel '${RBRV_SIGIL}' forbids binfmt but RBRV_CONJURE_PLATFORMS='${RBRV_CONJURE_PLATFORMS}'" ;;
    esac
  fi

  bcu_log_args "Generate build tag using vessel sigil"
  local z_tag="${RBRV_SIGIL}.${BDU_NOW_STAMP}"

  bcu_info "Building vessel image: ${RBRV_SIGIL} -> ${z_tag}"

  # Verify git state + capture metadata
  zrbf_verify_git_clean

  # Package build context (includes RBGY file as cloudbuild.yaml)
  zrbf_package_context "${RBRV_CONJURE_DOCKERFILE}" "${RBRV_CONJURE_BLDCONTEXT}" "${ZRBF_RBGY_BUILD_FILE}"

  # Submit build with substitutions (uses RBRR_GCB_* pins)
  local z_dockerfile_name="${RBRV_CONJURE_DOCKERFILE##*/}"
  zrbf_submit_build "${z_dockerfile_name}" "${z_tag}" "${RBRV_SIGIL}"

  # Wait for completion
  zrbf_wait_build_completion

  bcu_success "Vessel image built: ${z_tag}"
}

rbf_copy() {
  zrbf_sentinel

  bcu_die 'ELIDED FOR NOW'
}

rbf_delete() {
  zrbf_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Delete an image tag from the registry"
  bcu_doc_param "tag" "Image tag to delete"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  bcu_step "Fetching manifest digest for deletion"

  # Get OAuth token using Director credentials
  # Note: This requires GCB service account to have artifactregistry.repoAdmin role
  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || bcu_die "Failed to get OAuth token"
  echo "${z_token}" > "${ZRBF_TOKEN_FILE}" || bcu_die "Failed to write token file"

  # Get manifest with digest header
  local z_manifest_headers="${ZRBF_DELETE_PREFIX}headers.txt"

  curl -sL -I                                         \
    -H "Authorization: Bearer ${z_token}"             \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}"       \
    "${ZRBF_REGISTRY_API_BASE}/manifests/${z_tag}"    \
    > "${z_manifest_headers}" || bcu_die "Failed to fetch manifest headers"

  # Extract digest from Docker-Content-Digest header
  local z_digest_file="${ZRBF_DELETE_PREFIX}digest.txt"
  grep -i "docker-content-digest:" "${z_manifest_headers}" | \
    sed 's/.*: //' | tr -d '\r\n' > "${z_digest_file}" || bcu_die "Failed to extract digest header"

  local z_digest
  z_digest=$(<"${z_digest_file}")
  test -n "${z_digest}" || bcu_die "Manifest digest is empty"

  bcu_info "Deleting manifest: ${z_digest}"

  # Delete by digest
  local z_status_file="${ZRBF_DELETE_PREFIX}status.txt"

  curl -X DELETE -s                                   \
    -H "Authorization: Bearer ${z_token}"             \
    -w "%{http_code}"                                 \
    -o /dev/null                                      \
    "${ZRBF_REGISTRY_API_BASE}/manifests/${z_digest}" \
    > "${z_status_file}" || bcu_die "DELETE request failed"

  local z_http_code
  z_http_code=$(<"${z_status_file}")
  test -n "${z_http_code}" || bcu_die "HTTP status code is empty"
  test "${z_http_code}" = "202" || test "${z_http_code}" = "204" || \
    bcu_die "Delete failed with HTTP ${z_http_code}"

  bcu_success "Image deleted: ${z_tag}"
}

# eof

