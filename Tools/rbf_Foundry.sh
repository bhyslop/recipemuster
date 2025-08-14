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
# Recipe Bottle Image Management - Google Cloud Build operations

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBI_SOURCED:-}" || bcu_die "Module rbi multiply sourced - check sourcing hierarchy"
ZRBI_SOURCED=1

######################################################################
# Internal Functions (zrbi_*)

zrbi_kindle() {
  test -z "${ZRBI_KINDLED:-}" || bcu_die "Module rbi already kindled"

  # Validate environment
  bvu_dir_exists "${BDU_TEMP_DIR}"
  test -n "${BDU_NOW_STAMP:-}" || bcu_die "BDU_NOW_STAMP is unset or empty"

  bcu_log_args "Check required GCB/GAR environment variables"
  test -n "${RBRR_GCB_PROJECT_ID:-}"      || bcu_die "RBRR_GCB_PROJECT_ID not set"
  test -n "${RBRR_GCB_REGION:-}"          || bcu_die "RBRR_GCB_REGION not set"
  test -n "${RBRR_GAR_PROJECT_ID:-}"      || bcu_die "RBRR_GAR_PROJECT_ID not set"
  test -n "${RBRR_GAR_LOCATION:-}"        || bcu_die "RBRR_GAR_LOCATION not set"
  test -n "${RBRR_GAR_REPOSITORY:-}"      || bcu_die "RBRR_GAR_REPOSITORY not set"
  test -n "${RBRR_BUILD_ARCHITECTURES:-}" || bcu_die "RBRR_BUILD_ARCHITECTURES not set"

  bcu_log_args "Verify service account files"
  test -n "${RBRR_GCB_RBRA_FILE:-}" || bcu_die "RBRR_GCB_RBRA_FILE not set"
  test -f "${RBRR_GCB_RBRA_FILE}"   || bcu_die "GCB service env file not found: ${RBRR_GCB_RBRA_FILE}"

  # Module Variables (ZRBI_*)
  ZRBI_GCB_API_BASE="https://cloudbuild.googleapis.com/v1"
  ZRBI_GAR_API_BASE="https://artifactregistry.googleapis.com/v1"
  ZRBI_CLOUD_QUERY_BASE="https://console.cloud.google.com/cloud-build/builds"

  ZRBI_GCB_PROJECT_BUILDS_URL="${ZRBI_GCB_API_BASE}/projects/${RBRR_GCB_PROJECT_ID}/locations/${RBRR_GCB_REGION}/builds"
  ZRBI_GAR_PACKAGE_BASE="projects/${RBRR_GAR_PROJECT_ID}/locations/${RBRR_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}"

  # Registry API endpoints for delete
  ZRBI_REGISTRY_HOST="${RBRR_GAR_LOCATION}-docker.pkg.dev"
  ZRBI_REGISTRY_PATH="${RBRR_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  ZRBI_REGISTRY_API_BASE="https://${ZRBI_REGISTRY_HOST}/v2/${ZRBI_REGISTRY_PATH}"

  # Media types for delete operation
  ZRBI_ACCEPT_MANIFEST_MTYPES="application/vnd.docker.distribution.manifest.v2+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.oci.image.index.v1+json,application/vnd.oci.image.manifest.v1+json"

  # RBIA file presumed to be in the same Tools directory as this implementation
  local z_self_dir="${BASH_SOURCE[0]%/*}"
  ZRBI_RBIA_FILE="${z_self_dir}/rbia_cloudbuild.yaml"
  test -f "${ZRBI_RBIA_FILE}" || bcu_die "RBIA file not found in Tools: ${ZRBI_RBIA_FILE}"

  bcu_log_args "Define temp files for build operations"
  ZRBI_BUILD_CONTEXT_TAR="${BDU_TEMP_DIR}/rbi_build_context.tar.gz"
  ZRBI_BUILD_CONFIG_FILE="${BDU_TEMP_DIR}/rbi_build_config.json"
  ZRBI_BUILD_ID_FILE="${BDU_TEMP_DIR}/rbi_build_id.txt"
  ZRBI_BUILD_STATUS_FILE="${BDU_TEMP_DIR}/rbi_build_status.json"
  ZRBI_BUILD_LOG_FILE="${BDU_TEMP_DIR}/rbi_build_log.txt"
  ZRBI_BUILD_RESPONSE_FILE="${BDU_TEMP_DIR}/rbi_build_response.json"

  bcu_log_args "Define git info files"
  ZRBI_GIT_INFO_FILE="${BDU_TEMP_DIR}/rbi_git_info.json"
  ZRBI_GIT_COMMIT_FILE="${BDU_TEMP_DIR}/rbi_git_commit.txt"
  ZRBI_GIT_BRANCH_FILE="${BDU_TEMP_DIR}/rbi_git_branch.txt"
  ZRBI_GIT_REPO_FILE="${BDU_TEMP_DIR}/rbi_git_repo_url.txt"
  ZRBI_GIT_UNTRACKED_FILE="${BDU_TEMP_DIR}/rbi_git_untracked.txt"
  ZRBI_GIT_REMOTE_FILE="${BDU_TEMP_DIR}/rbi_git_remote.txt"

  bcu_log_args "Define staging and size files"
  ZRBI_STAGING_DIR="${BDU_TEMP_DIR}/rbi_staging"
  ZRBI_CONTEXT_SIZE_FILE="${BDU_TEMP_DIR}/rbi_context_size_bytes.txt"

  bcu_log_args "Define validation files"
  ZRBI_MONIKER_VALID_FILE="${BDU_TEMP_DIR}/rbi_moniker_valid.txt"
  ZRBI_STATUS_CHECK_FILE="${BDU_TEMP_DIR}/rbi_status_check.txt"
  ZRBI_BUILD_ID_TMP_FILE="${BDU_TEMP_DIR}/rbi_build_id_tmp.txt"

  bcu_log_args "Define delete operation files"
  ZRBI_DELETE_PREFIX="${BDU_TEMP_DIR}/rbi_delete_"
  ZRBI_TOKEN_FILE="${BDU_TEMP_DIR}/rbi_token.txt"

  ZRBI_KINDLED=1
}

zrbi_sentinel() {
  test "${ZRBI_KINDLED:-}" = "1" || bcu_die "Module rbi not kindled - call zrbi_kindle first"
}

zrbi_verify_git_clean() {
  zrbi_sentinel

  bcu_step "Verifying git repository state"

  bcu_log_args "Check for uncommitted changes"
  git diff-index --quiet HEAD -- || bcu_die "Uncommitted changes detected - commit or stash first"

  bcu_log_args "Check for untracked files"
  git ls-files --others --exclude-standard > "${ZRBI_GIT_UNTRACKED_FILE}" || bcu_die "Failed to check untracked files"
  local z_untracked=""
  z_untracked=$(<"${ZRBI_GIT_UNTRACKED_FILE}") || bcu_die "Failed to read untracked list"
  test -z "${z_untracked}" || bcu_die "Untracked files present - commit or clean first"

  bcu_log_args "Check if all commits are pushed"
  git fetch --quiet || bcu_die "git fetch failed"
  # If upstream missing, treat as 0 ahead
  git rev-list @{u}..HEAD --count > "${ZRBI_GIT_UNTRACKED_FILE}" 2>/dev/null || echo "0" > "${ZRBI_GIT_UNTRACKED_FILE}"
  local z_unpushed=""
  z_unpushed=$(<"${ZRBI_GIT_UNTRACKED_FILE}") || bcu_die "Failed to read ahead count"
  test "${z_unpushed}" -eq 0 || bcu_die "Local commits not pushed (${z_unpushed} commits ahead)"

  bcu_log_args "Get git metadata"
  git rev-parse HEAD              > "${ZRBI_GIT_COMMIT_FILE}" || bcu_die "Failed to get commit SHA"
  git rev-parse --abbrev-ref HEAD > "${ZRBI_GIT_BRANCH_FILE}" || bcu_die "Failed to get branch name"

  bcu_log_args "Get first available remote"
  git remote | head -1 > "${ZRBI_GIT_REMOTE_FILE}" || bcu_die "No git remotes configured"
  local z_remote=""
  z_remote=$(<"${ZRBI_GIT_REMOTE_FILE}") || bcu_die "Failed to read remote name"
  test -n "${z_remote}" || bcu_die "No git remotes found"

  bcu_log_args "Get repo URL from remote: ${z_remote}"
  git config --get "remote.${z_remote}.url" > "${ZRBI_GIT_REPO_FILE}" || bcu_die "Failed to get repo URL"

  local z_commit=""
  local z_branch=""
  local z_repo_url=""
  z_commit=$(<"${ZRBI_GIT_COMMIT_FILE}") || bcu_die "Failed to read commit"
  z_branch=$(<"${ZRBI_GIT_BRANCH_FILE}") || bcu_die "Failed to read branch"
  z_repo_url=$(<"${ZRBI_GIT_REPO_FILE}") || bcu_die "Failed to read repo url"

  test -n "${z_commit}"   || bcu_die "Git commit is empty"
  test -n "${z_branch}"   || bcu_die "Git branch is empty"
  test -n "${z_repo_url}" || bcu_die "Git repo URL is empty"

  bcu_log_args "Extract owner/repo from URL (handles both HTTPS and SSH)"
  # Example HTTPS: https://github.com/owner/repo.git
  # Example SSH  : git@github.com:owner/repo.git
  local z_repo="${z_repo_url#*github.com[:/]}"
  z_repo="${z_repo%.git}"

  bcu_log_args "Write git info JSON"
  jq -n                        \
    --arg commit "${z_commit}" \
    --arg branch "${z_branch}" \
    --arg repo   "${z_repo}"   \
    '{"commit": $commit, "branch": $branch, "repo": $repo}' \
    > "${ZRBI_GIT_INFO_FILE}" || bcu_die "Failed to write git info"

  bcu_info "Git state clean - commit: ${z_commit:0:8} on ${z_branch}"
}

zrbi_package_context() {
  zrbi_sentinel

  local z_dockerfile="$1"
  local z_context_dir="$2"

  bcu_step "Packaging build context"

  bcu_log_args "Create temp directory for context"
  rm -rf "${ZRBI_STAGING_DIR}" || bcu_warn "Failed to clean existing staging directory"
  mkdir -p "${ZRBI_STAGING_DIR}" || bcu_die "Failed to create staging directory"

  bcu_log_args "Copy context to staging"
  cp -r "${z_context_dir}/." "${ZRBI_STAGING_DIR}/" || bcu_die "Failed to copy context"

  bcu_log_args "Copy Dockerfile to context root if not already there"
  local z_dockerfile_name="${z_dockerfile##*/}"
  cp "${z_dockerfile}" "${ZRBI_STAGING_DIR}/${z_dockerfile_name}" || bcu_die "Failed to copy Dockerfile"

  bcu_log_args "Copy RBIA Cloud Build YAML to context (fixed name: cloudbuild.yaml)"
  cp "${ZRBI_RBIA_FILE}" "${ZRBI_STAGING_DIR}/cloudbuild.yaml" || bcu_die "Failed to copy RBIA file"

  bcu_log_args "Create tarball"
  tar -czf "${ZRBI_BUILD_CONTEXT_TAR}" -C "${ZRBI_STAGING_DIR}" . || bcu_die "Failed to create context archive"

  bcu_log_args "Clean up staging"
  rm -rf "${ZRBI_STAGING_DIR}" || bcu_warn "Failed to cleanup staging directory"

  bcu_log_args "Compute archive size (bytes) using temp file"
  wc -c < "${ZRBI_BUILD_CONTEXT_TAR}" > "${ZRBI_CONTEXT_SIZE_FILE}" || bcu_die "Failed to compute context size"
  local z_size_bytes=""
  z_size_bytes=$(<"${ZRBI_CONTEXT_SIZE_FILE}") || bcu_die "Failed to read context size"
  test -n "${z_size_bytes}" || bcu_die "Context size is empty"

  bcu_info "Build context packaged: ${z_size_bytes} bytes"
}

zrbi_submit_build() {
  zrbi_sentinel

  local z_dockerfile_name="$1"
  local z_tag="$2"
  local z_moniker="$3"

  bcu_step "Submitting build to Google Cloud Build"

  bcu_log_args "Get OAuth token using capture function"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_GCB_RBRA_FILE}") || bcu_die "Failed to get GCB OAuth token"

  bcu_log_args "Read git info from file"
  jq -r '.commit' "${ZRBI_GIT_INFO_FILE}" > "${ZRBI_GIT_COMMIT_FILE}" || bcu_die "Failed to extract git commit"
  jq -r '.branch' "${ZRBI_GIT_INFO_FILE}" > "${ZRBI_GIT_BRANCH_FILE}" || bcu_die "Failed to extract git branch"
  jq -r '.repo'   "${ZRBI_GIT_INFO_FILE}" > "${ZRBI_GIT_REPO_FILE}"   || bcu_die "Failed to extract git repo"

  local z_git_commit=""
  local z_git_branch=""
  local z_git_repo=""
  z_git_commit=$(<"${ZRBI_GIT_COMMIT_FILE}") || bcu_die "Failed to read git commit"
  z_git_branch=$(<"${ZRBI_GIT_BRANCH_FILE}") || bcu_die "Failed to read git branch"
  z_git_repo=$(<"${ZRBI_GIT_REPO_FILE}")     || bcu_die "Failed to read git repo"

  test -n "${z_git_commit}" || bcu_die "Git commit is empty"
  test -n "${z_git_branch}" || bcu_die "Git branch is empty"
  test -n "${z_git_repo}"   || bcu_die "Git repo is empty"

  bcu_log_args "Extract recipe name without extension"
  local z_recipe_name="${z_dockerfile_name%.*}"

  bcu_log_args "Create build config with RBIA substitutions (no storageSource for inline upload)"
  jq -n '{
        "substitutions": {
          "_RBIA_DOCKERFILE":     "'"${z_dockerfile_name}"'",
          "_RBIA_TAG":            "'"${z_tag}"'",
          "_RBIA_MONIKER":        "'"${z_moniker}"'",
          "_RBIA_PLATFORMS":      "'"${RBRR_BUILD_ARCHITECTURES}"'",
          "_RBIA_GAR_LOCATION":   "'"${RBRR_GAR_LOCATION}"'",
          "_RBIA_GAR_PROJECT":    "'"${RBRR_GAR_PROJECT_ID}"'",
          "_RBIA_GAR_REPOSITORY": "'"${RBRR_GAR_REPOSITORY}"'",
          "_RBIA_GIT_COMMIT":     "'"${z_git_commit}"'",
          "_RBIA_GIT_BRANCH":     "'"${z_git_branch}"'",
          "_RBIA_GIT_REPO":       "'"${z_git_repo}"'",
          "_RBIA_RECIPE_NAME":    "'"${z_recipe_name}"'"
        }
      }' > "${ZRBI_BUILD_CONFIG_FILE}" || bcu_die "Failed to create build config"

  bcu_log_args "Submit build with inline source upload"
  curl -s -X POST                                               \
       -H "Authorization: Bearer ${z_token}"                    \
       -H "Content-Type: application/json"                      \
       -H "x-goog-upload-protocol: multipart"                   \
       -F "metadata=@${ZRBI_BUILD_CONFIG_FILE};type=application/json" \
       -F "source=@${ZRBI_BUILD_CONTEXT_TAR};type=application/gzip"   \
       "${ZRBI_GCB_PROJECT_BUILDS_URL}"                        \
       > "${ZRBI_BUILD_RESPONSE_FILE}" \
    || bcu_die "Failed to submit build"

  bcu_log_args "Validate response file"
  test -f "${ZRBI_BUILD_RESPONSE_FILE}" || bcu_die "Build response file not created"
  test -s "${ZRBI_BUILD_RESPONSE_FILE}" || bcu_die "Build response file is empty"

  bcu_log_args "Extract build ID from response"
  jq -r '.name' "${ZRBI_BUILD_RESPONSE_FILE}" > "${ZRBI_BUILD_ID_FILE}" || bcu_die "Failed to extract build name"

  bcu_log_args "Parse build ID from full path using parameter expansion"
  local z_full=""
  z_full=$(<"${ZRBI_BUILD_ID_FILE}") || bcu_die "Failed to read build name path"
  local z_only="${z_full##*/}"
  printf '%s' "${z_only}" > "${ZRBI_BUILD_ID_TMP_FILE}" || bcu_die "Failed to write temp build ID"
  mv "${ZRBI_BUILD_ID_TMP_FILE}" "${ZRBI_BUILD_ID_FILE}" || bcu_die "Failed to finalize build ID"

  local z_build_id=""
  z_build_id=$(<"${ZRBI_BUILD_ID_FILE}") || bcu_die "Failed to read build ID"
  test -n "${z_build_id}" || bcu_die "Build ID is empty"

  local z_console_url="${ZRBI_CLOUD_QUERY_BASE}/${z_build_id}?project=${RBRR_GCB_PROJECT_ID}"
  bcu_info "Build submitted: ${z_build_id}"
  bcu_link "Open build in Cloud Console" "${z_console_url}"
}

zrbi_wait_build_completion() {
  zrbi_sentinel

  bcu_step "Waiting for build completion"

  local z_build_id=""
  z_build_id=$(<"${ZRBI_BUILD_ID_FILE}") || bcu_die "No build ID found"
  test -n "${z_build_id}" || bcu_die "Build ID file empty"

  bcu_log_args "Get fresh token for polling"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_GCB_RBRA_FILE}") || bcu_die "Failed to get GCB OAuth token"

  local z_status="PENDING"
  local z_attempts=0
  local z_max_attempts=240  # 20 minutes with 5 second intervals

  while true; do
    case "${z_status}" in
      PENDING|QUEUED|WORKING)
        ;;
      *)
        break
        ;;
    esac

    sleep 5
    z_attempts=$((z_attempts + 1))
    test ${z_attempts} -le ${z_max_attempts} || bcu_die "Build timeout after ${z_max_attempts} attempts"

    bcu_log_args "Fetch build status (attempt ${z_attempts}/${z_max_attempts})"
    curl -s \
         -H "Authorization: Bearer ${z_token}" \
         "${ZRBI_GCB_PROJECT_BUILDS_URL}/${z_build_id}" \
         > "${ZRBI_BUILD_STATUS_FILE}" \
      || bcu_die "Failed to get build status"

    test -f "${ZRBI_BUILD_STATUS_FILE}" || bcu_die "Build status file not created"
    test -s "${ZRBI_BUILD_STATUS_FILE}" || bcu_die "Build status file is empty"

    jq -r '.status' "${ZRBI_BUILD_STATUS_FILE}" > "${ZRBI_STATUS_CHECK_FILE}" || bcu_die "Failed to extract status"
    z_status=$(<"${ZRBI_STATUS_CHECK_FILE}") || bcu_die "Failed to read status"
    test -n "${z_status}" || bcu_die "Status is empty"

    bcu_info "Build status: ${z_status} (attempt ${z_attempts}/${z_max_attempts})"
  done

  test "${z_status}" = "SUCCESS" || bcu_die "Build failed with status: ${z_status}"

  bcu_success "Build completed successfully"
}

zrbi_validate_moniker_predicate() {
  zrbi_sentinel

  local z_moniker="$1"

  bcu_log_args "Check moniker format: lowercase alphanumeric with dash/underscore"
  if [[ "${z_moniker}" =~ ^[a-z0-9_-]+$ ]]; then
    return 0
  else
    return 1
  fi
}

######################################################################
# External Functions (rbi_*)

rbi_build() {
  zrbi_sentinel

  local z_dockerfile="${1:-}"
  local z_context_dir="${2:-}"
  local z_moniker="${3:-}"

  # Documentation block
  bcu_doc_brief "Build container image using Google Cloud Build"
  bcu_doc_param "dockerfile"  "Path to Dockerfile"
  bcu_doc_param "context_dir" "Build context directory"
  bcu_doc_param "moniker"     "Service moniker (e.g., srjcl, pluml)"
  bcu_doc_shown || return 0

  bcu_log_args "Validate parameters"
  test -n "${z_dockerfile}"  || bcu_die "Dockerfile required"
  test -f "${z_dockerfile}"  || bcu_die "Dockerfile not found: ${z_dockerfile}"
  test -n "${z_context_dir}" || bcu_die "Context directory required"
  test -d "${z_context_dir}" || bcu_die "Context directory not found: ${z_context_dir}"
  test -n "${z_moniker}"     || bcu_die "Moniker required"

  bcu_log_args "Validate moniker format"
  zrbi_validate_moniker_predicate "${z_moniker}" || bcu_die "Moniker must be lowercase alphanumeric with dash/underscore"

  bcu_log_args "Generate build tag"
  local z_dockerfile_name="${z_dockerfile##*/}"
  local z_recipe_base="${z_dockerfile_name%.*}"
  local z_tag="${z_recipe_base}.${z_moniker}.${BDU_NOW_STAMP}"

  bcu_info "Building image: ${z_tag}"

  # Verify git state
  zrbi_verify_git_clean

  # Package build context (includes RBIA file as cloudbuild.yaml)
  zrbi_package_context "${z_dockerfile}" "${z_context_dir}"

  # Submit build
  zrbi_submit_build "${z_dockerfile_name}" "${z_tag}" "${z_moniker}"

  # Wait for completion
  zrbi_wait_build_completion

  bcu_success "Image built: ${z_tag}"
}

rbi_delete() {
  zrbi_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Delete an image tag from the registry"
  bcu_doc_param "tag" "Image tag to delete"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  bcu_step "Fetching manifest digest for deletion"

  # Get OAuth token using GCB Submitter credentials
  # Note: This requires GCB service account to have artifactregistry.repoAdmin role
  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_GCB_RBRA_FILE}") || bcu_die "Failed to get OAuth token"
  echo "${z_token}" > "${ZRBI_TOKEN_FILE}" || bcu_die "Failed to write token file"

  # Get manifest with digest header
  local z_manifest_headers="${ZRBI_DELETE_PREFIX}headers.txt"

  curl -sL -I \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBI_ACCEPT_MANIFEST_MTYPES}" \
    "${ZRBI_REGISTRY_API_BASE}/manifests/${z_tag}" \
    > "${z_manifest_headers}" || bcu_die "Failed to fetch manifest headers"

  # Extract digest from Docker-Content-Digest header
  local z_digest_file="${ZRBI_DELETE_PREFIX}digest.txt"
  grep -i "docker-content-digest:" "${z_manifest_headers}" | \
    sed 's/.*: //' | tr -d '\r\n' > "${z_digest_file}" || bcu_die "Failed to extract digest header"

  local z_digest
  z_digest=$(<"${z_digest_file}")
  test -n "${z_digest}" || bcu_die "Manifest digest is empty"

  bcu_info "Deleting manifest: ${z_digest}"

  # Delete by digest
  local z_status_file="${ZRBI_DELETE_PREFIX}status.txt"

  curl -X DELETE -s \
    -H "Authorization: Bearer ${z_token}" \
    -w "%{http_code}" \
    -o /dev/null \
    "${ZRBI_REGISTRY_API_BASE}/manifests/${z_digest}" \
    > "${z_status_file}" || bcu_die "DELETE request failed"

  local z_http_code
  z_http_code=$(<"${z_status_file}")
  test -n "${z_http_code}" || bcu_die "HTTP status code is empty"
  test "${z_http_code}" = "202" || test "${z_http_code}" = "204" || \
    bcu_die "Delete failed with HTTP ${z_http_code}"

  bcu_success "Image deleted: ${z_tag}"
}

# eof

