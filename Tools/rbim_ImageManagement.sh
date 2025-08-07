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

  # Verify RBGO is available
  test "${ZRBGO_KINDLED:-}" = "1" || bcu_die "Module rbgo not kindled - must kindle rbgo before rbim"

  # Check required GCB environment
  test -n "${RBRR_GCB_PROJECT_ID:-}"   || bcu_die "RBRR_GCB_PROJECT_ID not set"
  test -n "${RBRR_GCB_REGION:-}"       || bcu_die "RBRR_GCB_REGION not set"
  test -n "${RBRR_GAR_PROJECT_ID:-}"   || bcu_die "RBRR_GAR_PROJECT_ID not set"
  test -n "${RBRR_GAR_LOCATION:-}"     || bcu_die "RBRR_GAR_LOCATION not set"
  test -n "${RBRR_GAR_REPOSITORY:-}"   || bcu_die "RBRR_GAR_REPOSITORY not set"
  test -n "${RBRR_BUILD_ARCHITECTURES:-}" || bcu_die "RBRR_BUILD_ARCHITECTURES not set"

  # Verify service account files
  test -n "${RBRR_GCB_RBRA_FILE:-}" || bcu_die "RBRR_GCB_RBRA_FILE not set"
  test -f "${RBRR_GCB_RBRA_FILE}"   || bcu_die "GCB service env file not found: ${RBRR_GCB_RBRA_FILE}"

  # Module Variables (ZRBIM_*)
  ZRBIM_GCB_API_BASE="https://cloudbuild.googleapis.com/v1"
  ZRBIM_GAR_API_BASE="https://artifactregistry.googleapis.com/v1"

  ZRBIM_BUILD_CONTEXT_TAR="${BDU_TEMP_DIR}/rbim_build_context.tar.gz"
  ZRBIM_BUILD_CONFIG_FILE="${BDU_TEMP_DIR}/rbim_build_config.json"
  ZRBIM_BUILD_ID_FILE="${BDU_TEMP_DIR}/rbim_build_id.txt"
  ZRBIM_BUILD_STATUS_FILE="${BDU_TEMP_DIR}/rbim_build_status.json"
  ZRBIM_BUILD_LOG_FILE="${BDU_TEMP_DIR}/rbim_build_log.txt"
  ZRBIM_METADATA_ARCHIVE="${BDU_TEMP_DIR}/rbim_metadata.tar.gz"
  ZRBIM_GIT_INFO_FILE="${BDU_TEMP_DIR}/rbim_git_info.json"
  ZRBIM_TOKEN_FILE="${BDU_TEMP_DIR}/rbim_gcb_token.txt"

  ZRBIM_KINDLED=1
}

zrbim_sentinel() {
  test "${ZRBIM_KINDLED:-}" = "1" || bcu_die "Module rbim not kindled - call zrbim_kindle first"
}

zrbim_verify_git_clean() {
  zrbim_sentinel

  bcu_step "Verifying git repository state"

  # Check uncommitted changes
  git diff-index --quiet HEAD -- || bcu_die "Uncommitted changes detected - commit or stash first"

  # Check untracked files
  test -z "$(git ls-files --others --exclude-standard)" || bcu_die "Untracked files present - commit or clean first"

  # Check if pushed
  git fetch --quiet
  local z_unpushed
  z_unpushed=$(git rev-list @{u}..HEAD --count 2>/dev/null || echo "0")
  test "${z_unpushed}" -eq 0 || bcu_die "Local commits not pushed (${z_unpushed} commits ahead)"

  # Capture git metadata
  local z_commit z_branch z_repo_url z_repo
  z_commit=$(git rev-parse HEAD)              || bcu_die "Failed to get commit SHA"
  z_branch=$(git rev-parse --abbrev-ref HEAD) || bcu_die "Failed to get branch name"
  z_repo_url=$(git config --get remote.origin.url) || bcu_die "Failed to get repo URL"

  # Extract owner/repo from URL (handles both HTTPS and SSH)
  z_repo="${z_repo_url#*github.com[:/]}"
  z_repo="${z_repo%.git}"

  # Write git info
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

  # Create temp directory for context
  local z_staging="${BDU_TEMP_DIR}/rbim_staging"
  rm -rf "${z_staging}"
  mkdir -p "${z_staging}" || bcu_die "Failed to create staging directory"

  # Copy context
  cp -r "${z_context_dir}/." "${z_staging}/" || bcu_die "Failed to copy context"

  # Copy Dockerfile to context root if not already there
  local z_dockerfile_name
  z_dockerfile_name=$(basename "${z_dockerfile}")
  cp "${z_dockerfile}" "${z_staging}/${z_dockerfile_name}" || bcu_die "Failed to copy Dockerfile"

  # Create tarball
  tar -czf "${ZRBIM_BUILD_CONTEXT_TAR}" -C "${z_staging}" . || bcu_die "Failed to create context archive"

  # Clean up staging
  rm -rf "${z_staging}"

  local z_size
  z_size=$(du -h "${ZRBIM_BUILD_CONTEXT_TAR}" | cut -f1)
  bcu_info "Build context packaged: ${z_size}"
}

zrbim_submit_build() {
  zrbim_sentinel

  local z_dockerfile_name="$1"
  local z_tag="$2"
  local z_moniker="$3"

  bcu_step "Submitting build to Google Cloud Build"

  # Get OAuth token
  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_GCB_RBRA_FILE}") || bcu_die "Failed to get GCB OAuth token"
  echo "${z_token}" > "${ZRBIM_TOKEN_FILE}"

  # Read git info
  local z_git_commit z_git_branch z_git_repo
  z_git_commit=$(jq -r '.commit' "${ZRBIM_GIT_INFO_FILE}") || bcu_die "Failed to read git commit"
  z_git_branch=$(jq -r '.branch' "${ZRBIM_GIT_INFO_FILE}") || bcu_die "Failed to read git branch"
  z_git_repo=$(jq -r '.repo' "${ZRBIM_GIT_INFO_FILE}")     || bcu_die "Failed to read git repo"

  # Extract recipe name without extension
  local z_recipe_name="${z_dockerfile_name%.*}"

  # Create build config with substitutions
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
      "source": {
        "storageSource": {
          "bucket": "_",
          "object": "_"
        }
      },
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

  # Submit build with inline source upload
  local z_build_response="${BDU_TEMP_DIR}/rbim_build_response.json"

  curl -X POST \
    -H "Authorization: Bearer ${z_token}" \
    -H "Content-Type: application/json" \
    -H "x-goog-upload-protocol: multipart" \
    -F "metadata=@${ZRBIM_BUILD_CONFIG_FILE};type=application/json" \
    -F "source=@${ZRBIM_BUILD_CONTEXT_TAR};type=application/gzip" \
    "${ZRBIM_GCB_API_BASE}/projects/${RBRR_GCB_PROJECT_ID}/locations/${RBRR_GCB_REGION}/builds" \
    > "${z_build_response}" 2>/dev/null || bcu_die "Failed to submit build"

  # Extract build ID
  jq -r '.name' "${z_build_response}" | sed 's|.*/||' > "${ZRBIM_BUILD_ID_FILE}" \
    || bcu_die "Failed to extract build ID"

  local z_build_id
  z_build_id=$(<"${ZRBIM_BUILD_ID_FILE}")
  test -n "${z_build_id}" || bcu_die "Build ID is empty"

  bcu_info "Build submitted: ${z_build_id}"
  bcu_info "View at: https://console.cloud.google.com/cloud-build/builds/${z_build_id}?project=${RBRR_GCB_PROJECT_ID}"
}

zrbim_wait_build_completion() {
  zrbim_sentinel

  bcu_step "Waiting for build completion"

  local z_build_id
  z_build_id=$(<"${ZRBIM_BUILD_ID_FILE}")
  test -n "${z_build_id}" || bcu_die "No build ID found"

  local z_token
  z_token=$(<"${ZRBIM_TOKEN_FILE}")
  test -n "${z_token}" || bcu_die "Token is empty"

  local z_status="PENDING"
  local z_attempts=0
  local z_max_attempts=240  # 20 minutes with 5 second intervals

  while test "${z_status}" = "PENDING" -o "${z_status}" = "QUEUED" -o "${z_status}" = "WORKING"; do
    sleep 5
    z_attempts=$((z_attempts + 1))

    test ${z_attempts} -le ${z_max_attempts} || bcu_die "Build timeout after ${z_max_attempts} attempts"

    curl -s \
      -H "Authorization: Bearer ${z_token}" \
      "${ZRBIM_GCB_API_BASE}/projects/${RBRR_GCB_PROJECT_ID}/locations/${RBRR_GCB_REGION}/builds/${z_build_id}" \
      > "${ZRBIM_BUILD_STATUS_FILE}" || bcu_die "Failed to get build status"

    z_status=$(jq -r '.status' "${ZRBIM_BUILD_STATUS_FILE}") || bcu_die "Failed to extract status"

    bcu_info "Build status: ${z_status} (attempt ${z_attempts}/${z_max_attempts})"
  done

  test "${z_status}" = "SUCCESS" || bcu_die "Build failed with status: ${z_status}"

  bcu_success "Build completed successfully"
}

zrbim_retrieve_metadata() {
  zrbim_sentinel

  local z_tag="$1"

  bcu_step "Retrieving build metadata from GAR"

  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_GAR_RBRA_FILE}") || bcu_die "Failed to get GAR OAuth token"

  # Download metadata artifact
  local z_package_path="projects/${RBRR_GAR_PROJECT_ID}/locations/${RBRR_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}/packages/${z_tag}"

  curl -s \
    -H "Authorization: Bearer ${z_token}" \
    "${ZRBIM_GAR_API_BASE}/${z_package_path}/versions/metadata:download" \
    -o "${ZRBIM_METADATA_ARCHIVE}" || bcu_die "Failed to download metadata"

  # Extract metadata
  local z_extract_dir="${BDU_TEMP_DIR}/rbim_metadata"
  rm -rf "${z_extract_dir}"
  mkdir -p "${z_extract_dir}" || bcu_die "Failed to create extract directory"

  tar -xzf "${ZRBIM_METADATA_ARCHIVE}" -C "${z_extract_dir}" || bcu_die "Failed to extract metadata"

  # Display key information
  if test -f "${z_extract_dir}/package_summary.txt"; then
    bcu_info "Top packages in image:"
    head -5 "${z_extract_dir}/package_summary.txt"
  fi

  bcu_success "Metadata retrieved to ${z_extract_dir}"
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

  # Validate parameters
  test -n "${z_dockerfile}"  || bcu_die "Dockerfile required"
  test -f "${z_dockerfile}"  || bcu_die "Dockerfile not found: ${z_dockerfile}"
  test -n "${z_context_dir}" || bcu_die "Context directory required"
  test -d "${z_context_dir}" || bcu_die "Context directory not found: ${z_context_dir}"
  test -n "${z_moniker}"     || bcu_die "Moniker required"

  # Validate moniker format (lowercase, alphanumeric)
  echo "${z_moniker}" | grep -q '^[a-z0-9_-]*$' || bcu_die "Moniker must be lowercase alphanumeric"

  # Generate build tag
  local z_dockerfile_name
  z_dockerfile_name=$(basename "${z_dockerfile}")
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

  # Use rbcr for listing
  rbcr_list_tags

  local z_records_file="${ZRBCR_IMAGE_RECORDS_FILE}"

  echo "Repository: ${RBRR_GAR_LOCATION}-docker.pkg.dev/${RBRR_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  echo "Images:"

  jq -r '.[] | .tag' "${z_records_file}" | sort -r | head -20

  local z_total
  z_total=$(jq '. | length' "${z_records_file}")
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

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag required"

  # Use rbcr for pulling
  rbcr_pull "${z_tag}"

  bcu_success "Image pulled: ${z_tag}"
}

# eof

