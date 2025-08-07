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
test -z "${ZRBIM_SOURCED:-}" || bcu_die "Module rbim multiply sourced - check sourcing hierarchy"
ZRBIM_SOURCED=1

######################################################################
# Internal Functions (zrbim_*)

zrbim_kindle() {
  test -z "${ZRBIM_KINDLED:-}" || bcu_die "Module rbim already kindled"

  # Validate environment
  bvu_dir_exists "${BDU_TEMP_DIR}"
  test -n "${BDU_NOW_STAMP:-}" || bcu_die "BDU_NOW_STAMP is unset or empty"

  bcu_log "Check required GCB environment variables"
  test -n "${RBRR_GCB_PROJECT_ID:-}"      || bcu_die "RBRR_GCB_PROJECT_ID not set"
  test -n "${RBRR_GCB_REGION:-}"          || bcu_die "RBRR_GCB_REGION not set"
  test -n "${RBRR_GAR_PROJECT_ID:-}"      || bcu_die "RBRR_GAR_PROJECT_ID not set"
  test -n "${RBRR_GAR_LOCATION:-}"        || bcu_die "RBRR_GAR_LOCATION not set"
  test -n "${RBRR_GAR_REPOSITORY:-}"      || bcu_die "RBRR_GAR_REPOSITORY not set"
  test -n "${RBRR_BUILD_ARCHITECTURES:-}" || bcu_die "RBRR_BUILD_ARCHITECTURES not set"

  bcu_log "Verify service account files"
  test -n "${RBRR_GCB_RBRA_FILE:-}" || bcu_die "RBRR_GCB_RBRA_FILE not set"
  test -f "${RBRR_GCB_RBRA_FILE}"   || bcu_die "GCB service env file not found: ${RBRR_GCB_RBRA_FILE}"

  # Module Variables (ZRBIM_*)
  ZRBIM_GCB_API_BASE="https://cloudbuild.googleapis.com/v1"
  ZRBIM_GAR_API_BASE="https://artifactregistry.googleapis.com/v1"

  bcu_log "Define temp files for build operations"
  ZRBIM_BUILD_CONTEXT_TAR="${BDU_TEMP_DIR}/rbim_build_context.tar.gz"
  ZRBIM_BUILD_CONFIG_FILE="${BDU_TEMP_DIR}/rbim_build_config.json"
  ZRBIM_BUILD_ID_FILE="${BDU_TEMP_DIR}/rbim_build_id.txt"
  ZRBIM_BUILD_STATUS_FILE="${BDU_TEMP_DIR}/rbim_build_status.json"
  ZRBIM_BUILD_LOG_FILE="${BDU_TEMP_DIR}/rbim_build_log.txt"
  ZRBIM_BUILD_RESPONSE_FILE="${BDU_TEMP_DIR}/rbim_build_response.json"
  ZRBIM_METADATA_ARCHIVE="${BDU_TEMP_DIR}/rbim_metadata.tar.gz"

  bcu_log "Define git info files"
  ZRBIM_GIT_INFO_FILE="${BDU_TEMP_DIR}/rbim_git_info.json"
  ZRBIM_GIT_COMMIT_FILE="${BDU_TEMP_DIR}/rbim_git_commit.txt"
  ZRBIM_GIT_BRANCH_FILE="${BDU_TEMP_DIR}/rbim_git_branch.txt"
  ZRBIM_GIT_REPO_FILE="${BDU_TEMP_DIR}/rbim_git_repo_url.txt"
  ZRBIM_GIT_UNPUSHED_FILE="${BDU_TEMP_DIR}/rbim_git_unpushed.txt"
  ZRBIM_GIT_REMOTE_FILE="${BDU_TEMP_DIR}/rbim_git_remote.txt"

  bcu_log "Define staging and size files"
  ZRBIM_STAGING_DIR="${BDU_TEMP_DIR}/rbim_staging"
  ZRBIM_CONTEXT_SIZE_FILE="${BDU_TEMP_DIR}/rbim_context_size.txt"

  bcu_log "Define validation files"
  ZRBIM_MONIKER_VALID_FILE="${BDU_TEMP_DIR}/rbim_moniker_valid.txt"
  ZRBIM_STATUS_CHECK_FILE="${BDU_TEMP_DIR}/rbim_status_check.txt"
  ZRBIM_BUILD_ID_TMP_FILE="${BDU_TEMP_DIR}/rbim_build_id_tmp.txt"

  ZRBIM_KINDLED=1
}

zrbim_sentinel() {
  test "${ZRBIM_KINDLED:-}" = "1" || bcu_die "Module rbim not kindled - call zrbim_kindle first"
}

zrbim_verify_git_clean() {
  zrbim_sentinel

  bcu_step "Verifying git repository state"

  bcu_log "Check for uncommitted changes"
  bcu_info "Checking for uncommitted changes"
  git diff-index --quiet HEAD -- || bcu_die "Uncommitted changes detected - commit or stash first"

  bcu_log "Check for untracked files"
  bcu_info "Checking for untracked files"
  git ls-files --others --exclude-standard > "${ZRBIM_GIT_UNPUSHED_FILE}" || bcu_die "Failed to check untracked files"
  local z_untracked=$(<"${ZRBIM_GIT_UNPUSHED_FILE}")
  test -z "${z_untracked}" || bcu_die "Untracked files present - commit or clean first"

  bcu_log "Check if all commits are pushed"
  bcu_info "Checking if all commits are pushed"
  git fetch --quiet
  git rev-list @{u}..HEAD --count > "${ZRBIM_GIT_UNPUSHED_FILE}" 2>/dev/null || echo "0" > "${ZRBIM_GIT_UNPUSHED_FILE}"
  local z_unpushed=$(<"${ZRBIM_GIT_UNPUSHED_FILE}")
  test "${z_unpushed}" -eq 0 || bcu_die "Local commits not pushed (${z_unpushed} commits ahead)"

  bcu_log "Get git metadata"
  git rev-parse HEAD              > "${ZRBIM_GIT_COMMIT_FILE}" || bcu_die "Failed to get commit SHA"
  git rev-parse --abbrev-ref HEAD > "${ZRBIM_GIT_BRANCH_FILE}" || bcu_die "Failed to get branch name"

  bcu_log "Get first available remote"
  git remote | head -1 > "${ZRBIM_GIT_REMOTE_FILE}" || bcu_die "No git remotes configured"
  local z_remote=$(<"${ZRBIM_GIT_REMOTE_FILE}")
  test -n "${z_remote}" || bcu_die "No git remotes found"

  bcu_log "Get repo URL from remote: ${z_remote}"
  git config --get "remote.${z_remote}.url" > "${ZRBIM_GIT_REPO_FILE}" || bcu_die "Failed to get repo URL"

  local z_commit=$(<"${ZRBIM_GIT_COMMIT_FILE}")
  local z_branch=$(<"${ZRBIM_GIT_BRANCH_FILE}")
  local z_repo_url=$(<"${ZRBIM_GIT_REPO_FILE}")

  test -n "${z_commit}"   || bcu_die "Git commit is empty"
  test -n "${z_branch}"   || bcu_die "Git branch is empty"
  test -n "${z_repo_url}" || bcu_die "Git repo URL is empty"

  bcu_log "Extract owner/repo from URL (handles both HTTPS and SSH)"
  local z_repo="${z_repo_url#*github.com[:/]}"
  z_repo="${z_repo%.git}"

  bcu_log "Write git info JSON"
  jq -n \
    --arg commit "${z_commit}" \
    --arg branch "${z_branch}" \
    --arg repo   "${z_repo}" \
    '{"commit": $commit, "branch": $branch, "repo": $repo}' \
    > "${ZRBIM_GIT_INFO_FILE}" || bcu_die "Failed to write git info"

  bcu_info "Git state clean - commit: ${z_commit:0:8} on ${z_branch}"
}

zrbim_package_context() {
  zrbim_sentinel

  local z_dockerfile="$1"
  local z_context_dir="$2"

  bcu_step "Packaging build context"

  bcu_log "Create temp directory for context"
  rm -rf "${ZRBIM_STAGING_DIR}"
  mkdir -p "${ZRBIM_STAGING_DIR}" || bcu_die "Failed to create staging directory"

  bcu_log "Copy context to staging"
  cp -r "${z_context_dir}/." "${ZRBIM_STAGING_DIR}/" || bcu_die "Failed to copy context"

  bcu_log "Copy Dockerfile to context root if not already there"
  local z_dockerfile_name="${z_dockerfile##*/}"
  cp "${z_dockerfile}" "${ZRBIM_STAGING_DIR}/${z_dockerfile_name}" || bcu_die "Failed to copy Dockerfile"

  bcu_log "Create tarball"
  tar -czf "${ZRBIM_BUILD_CONTEXT_TAR}" -C "${ZRBIM_STAGING_DIR}" . || bcu_die "Failed to create context archive"

  bcu_log "Clean up staging"
  rm -rf "${ZRBIM_STAGING_DIR}"

  bcu_log "Get size for info"
  ls -lh "${ZRBIM_BUILD_CONTEXT_TAR}" | awk '{print $5}' > "${ZRBIM_CONTEXT_SIZE_FILE}" || bcu_die "Failed to get context size"
  local z_size=$(<"${ZRBIM_CONTEXT_SIZE_FILE}")
  test -n "${z_size}" || bcu_die "Context size is empty"

  bcu_info "Build context packaged: ${z_size}"
}

zrbim_submit_build() {
  zrbim_sentinel

  local z_dockerfile_name="$1"
  local z_tag="$2"
  local z_moniker="$3"

  bcu_step "Submitting build to Google Cloud Build"

  bcu_log "Get OAuth token using capture function"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_GCB_RBRA_FILE}") || bcu_die "Failed to get GCB OAuth token"

  bcu_log "Read git info from file"
  jq -r '.commit' "${ZRBIM_GIT_INFO_FILE}" > "${ZRBIM_GIT_COMMIT_FILE}" || bcu_die "Failed to extract git commit"
  jq -r '.branch' "${ZRBIM_GIT_INFO_FILE}" > "${ZRBIM_GIT_BRANCH_FILE}" || bcu_die "Failed to extract git branch"
  jq -r '.repo'   "${ZRBIM_GIT_INFO_FILE}" > "${ZRBIM_GIT_REPO_FILE}"   || bcu_die "Failed to extract git repo"

  local z_git_commit=$(<"${ZRBIM_GIT_COMMIT_FILE}")
  local z_git_branch=$(<"${ZRBIM_GIT_BRANCH_FILE}")
  local z_git_repo=$(<"${ZRBIM_GIT_REPO_FILE}")

  test -n "${z_git_commit}" || bcu_die "Git commit is empty"
  test -n "${z_git_branch}" || bcu_die "Git branch is empty"
  test -n "${z_git_repo}"   || bcu_die "Git repo is empty"

  bcu_log "Extract recipe name without extension"
  local z_recipe_name="${z_dockerfile_name%.*}"

  bcu_log "Create build config with substitutions (no storageSource for inline upload)"
  jq -n \
    --arg dockerfile "${z_dockerfile_name}" \
    --arg tag "${z_tag}" \
    --arg moniker "${z_moniker}" \
    --arg platforms "${RBRR_BUILD_ARCHITECTURES}" \
    --arg gar_location "${RBRR_GAR_LOCATION}" \
    --arg gar_project "${RBRR_GAR_PROJECT_ID}" \
    --arg gar_repository "${RBRR_GAR_REPOSITORY}" \
    --arg git_commit "${z_git_commit}" \
    --arg git_branch "${z_git_branch}" \
    --arg git_repo "${z_git_repo}" \
    --arg recipe_name "${z_recipe_name}" \
    '{
      "substitutions": {
        "_DOCKERFILE": $dockerfile,
        "_TAG": $tag,
        "_MONIKER": $moniker,
        "_PLATFORMS": $platforms,
        "_GAR_LOCATION": $gar_location,
        "_GAR_PROJECT": $gar_project,
        "_GAR_REPOSITORY": $gar_repository,
        "_GIT_COMMIT": $git_commit,
        "_GIT_BRANCH": $git_branch,
        "_GIT_REPO": $git_repo,
        "_RECIPE_NAME": $recipe_name
      }
    }' > "${ZRBIM_BUILD_CONFIG_FILE}" || bcu_die "Failed to create build config"

  bcu_log "Submit build with inline source upload"
  curl -X POST \
    -H "Authorization: Bearer ${z_token}" \
    -H "Content-Type: application/json" \
    -H "x-goog-upload-protocol: multipart" \
    -F "metadata=@${ZRBIM_BUILD_CONFIG_FILE};type=application/json" \
    -F "source=@${ZRBIM_BUILD_CONTEXT_TAR};type=application/gzip" \
    "${ZRBIM_GCB_API_BASE}/projects/${RBRR_GCB_PROJECT_ID}/locations/${RBRR_GCB_REGION}/builds" \
    > "${ZRBIM_BUILD_RESPONSE_FILE}" 2>/dev/null || bcu_die "Failed to submit build"

  bcu_log "Validate response file"
  test -f "${ZRBIM_BUILD_RESPONSE_FILE}" || bcu_die "Build response file not created"
  test -s "${ZRBIM_BUILD_RESPONSE_FILE}" || bcu_die "Build response file is empty"

  bcu_log "Extract build ID from response"
  jq -r '.name' "${ZRBIM_BUILD_RESPONSE_FILE}" > "${ZRBIM_BUILD_ID_FILE}" || bcu_die "Failed to extract build name"

  bcu_log "Parse build ID from full path (portable sed)"
  sed 's|.*/||' "${ZRBIM_BUILD_ID_FILE}" > "${ZRBIM_BUILD_ID_TMP_FILE}" || bcu_die "Failed to parse build ID"
  mv "${ZRBIM_BUILD_ID_TMP_FILE}" "${ZRBIM_BUILD_ID_FILE}" || bcu_die "Failed to move parsed build ID"

  local z_build_id=$(<"${ZRBIM_BUILD_ID_FILE}")
  test -n "${z_build_id}" || bcu_die "Build ID is empty"

  bcu_info "Build submitted: ${z_build_id}"
  bcu_info "View at: https://console.cloud.google.com/cloud-build/builds/${z_build_id}?project=${RBRR_GCB_PROJECT_ID}"
}

zrbim_wait_build_completion() {
  zrbim_sentinel

  bcu_step "Waiting for build completion"

  local z_build_id=$(<"${ZRBIM_BUILD_ID_FILE}")
  test -n "${z_build_id}" || bcu_die "No build ID found"

  bcu_log "Get fresh token for polling"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_GCB_RBRA_FILE}") || bcu_die "Failed to get GCB OAuth token"

  local z_status="PENDING"
  local z_attempts=0
  local z_max_attempts=240  # 20 minutes with 5 second intervals

  while true; do
    bcu_log "Check if build status is terminal"
    case "${z_status}" in
      PENDING|QUEUED|WORKING)
        bcu_log "Build still running, status: ${z_status}"
        ;;
      *)
        bcu_log "Build reached terminal status: ${z_status}"
        break
        ;;
    esac

    sleep 5
    z_attempts=$((z_attempts + 1))

    test ${z_attempts} -le ${z_max_attempts} || bcu_die "Build timeout after ${z_max_attempts} attempts"

    bcu_log "Fetch build status (attempt ${z_attempts}/${z_max_attempts})"
    curl -s \
      -H "Authorization: Bearer ${z_token}" \
      "${ZRBIM_GCB_API_BASE}/projects/${RBRR_GCB_PROJECT_ID}/locations/${RBRR_GCB_REGION}/builds/${z_build_id}" \
      > "${ZRBIM_BUILD_STATUS_FILE}" || bcu_die "Failed to get build status"

    bcu_log "Validate status response"
    test -f "${ZRBIM_BUILD_STATUS_FILE}" || bcu_die "Build status file not created"
    test -s "${ZRBIM_BUILD_STATUS_FILE}" || bcu_die "Build status file is empty"

    jq -r '.status' "${ZRBIM_BUILD_STATUS_FILE}" > "${ZRBIM_STATUS_CHECK_FILE}" || bcu_die "Failed to extract status"
    z_status=$(<"${ZRBIM_STATUS_CHECK_FILE}")
    test -n "${z_status}" || bcu_die "Status is empty"

    bcu_info "Build status: ${z_status} (attempt ${z_attempts}/${z_max_attempts})"
  done

  test "${z_status}" = "SUCCESS" || bcu_die "Build failed with status: ${z_status}"

  bcu_success "Build completed successfully"
}

zrbim_retrieve_metadata() {
  zrbim_sentinel

  local z_tag="$1"

  bcu_step "Retrieving build metadata from GAR"

  bcu_log "Get fresh token for GAR"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_GAR_RBRA_FILE}") || bcu_die "Failed to get GAR OAuth token"

  bcu_log "Construct package path"
  local z_package_path="projects/${RBRR_GAR_PROJECT_ID}/locations/${RBRR_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}/packages/${z_tag}"

  bcu_log "Download metadata artifact"
  curl -s \
    -H "Authorization: Bearer ${z_token}" \
    "${ZRBIM_GAR_API_BASE}/${z_package_path}/versions/metadata:download" \
    -o "${ZRBIM_METADATA_ARCHIVE}" || bcu_die "Failed to download metadata"

  bcu_log "Validate metadata archive"
  test -f "${ZRBIM_METADATA_ARCHIVE}" || bcu_die "Metadata archive not created"
  test -s "${ZRBIM_METADATA_ARCHIVE}" || bcu_die "Metadata archive is empty"

  bcu_log "Extract metadata"
  local z_extract_dir="${BDU_TEMP_DIR}/rbim_metadata"
  rm -rf "${z_extract_dir}"
  mkdir -p "${z_extract_dir}" || bcu_die "Failed to create extract directory"

  tar -xzf "${ZRBIM_METADATA_ARCHIVE}" -C "${z_extract_dir}" || bcu_die "Failed to extract metadata"

  bcu_log "Display key information if available"
  if test -f "${z_extract_dir}/package_summary.txt"; then
    bcu_info "Top packages in image:"
    head -5 "${z_extract_dir}/package_summary.txt"
  fi

  bcu_success "Metadata retrieved to ${z_extract_dir}"
}

zrbim_validate_moniker_predicate() {
  zrbim_sentinel

  local z_moniker="$1"

  bcu_log "Check moniker format: lowercase alphanumeric with dash/underscore"
  if [[ "${z_moniker}" =~ ^[a-z0-9_-]+$ ]]; then
    return 0
  else
    return 1
  fi
}

######################################################################
# External Functions (rbim_*)

rbim_build() {
  zrbim_sentinel

  local z_dockerfile="${1:-}"
  local z_context_dir="${2:-}"
  local z_moniker="${3:-}"

  # Documentation block
  bcu_doc_brief "Build container image using Google Cloud Build"
  bcu_doc_param "dockerfile" "Path to Dockerfile"
  bcu_doc_param "context_dir" "Build context directory"
  bcu_doc_param "moniker" "Service moniker (e.g., srjcl, pluml)"
  bcu_doc_shown || return 0

  bcu_log "Validate parameters"
  test -n "${z_dockerfile}"  || bcu_die "Dockerfile required"
  test -f "${z_dockerfile}"  || bcu_die "Dockerfile not found: ${z_dockerfile}"
  test -n "${z_context_dir}" || bcu_die "Context directory required"
  test -d "${z_context_dir}" || bcu_die "Context directory not found: ${z_context_dir}"
  test -n "${z_moniker}"     || bcu_die "Moniker required"

  bcu_log "Validate moniker format"
  zrbim_validate_moniker_predicate "${z_moniker}" || bcu_die "Moniker must be lowercase alphanumeric with dash/underscore"

  bcu_log "Generate build tag"
  local z_dockerfile_name="${z_dockerfile##*/}"
  local z_recipe_base="${z_dockerfile_name%.*}"
  local z_tag="${z_recipe_base}.${z_moniker}.${BDU_NOW_STAMP}"

  bcu_info "Building image: ${z_tag}"

  # Verify git state
  zrbim_verify_git_clean

  # Package build context
  zrbim_package_context "${z_dockerfile}" "${z_context_dir}"

  # Submit build
  zrbim_submit_build "${z_dockerfile_name}" "${z_tag}" "${z_moniker}"

  # Wait for completion
  zrbim_wait_build_completion

  # Retrieve metadata
  zrbim_retrieve_metadata "${z_tag}"

  bcu_success "Image built: ${z_tag}"
}

rbim_list() {
  zrbim_sentinel

  # Documentation block
  bcu_doc_brief "List images in Google Artifact Registry"
  bcu_doc_shown || return 0

  bcu_step "Listing images in GAR"

  bcu_log "Use rbcr for listing"
  rbcr_list_tags

  local z_records_file="${ZRBCR_IMAGE_RECORDS_FILE}"
  test -f "${z_records_file}" || bcu_die "Image records file not found: ${z_records_file}"

  echo "Repository: ${RBRR_GAR_LOCATION}-docker.pkg.dev/${RBRR_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  echo "Images:"

  jq -r '.[] | .tag' "${z_records_file}" | sort -r | head -20

  jq '. | length' "${z_records_file}" > "${ZRBIM_STATUS_CHECK_FILE}" || bcu_die "Failed to count images"
  local z_total=$(<"${ZRBIM_STATUS_CHECK_FILE}")
  test -n "${z_total}" || bcu_die "Total count is empty"

  bcu_info "Total images: ${z_total}"

  bcu_success "List complete"
}

rbim_retrieve() {
  zrbim_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Pull image from Google Artifact Registry"
  bcu_doc_param "tag" "Image tag to pull"
  bcu_doc_shown || return 0

  bcu_log "Validate parameters"
  test -n "${z_tag}" || bcu_die "Tag required"

  bcu_log "Use rbcr for pulling"
  rbcr_pull "${z_tag}"

  bcu_success "Image pulled: ${z_tag}"
}

# eof

