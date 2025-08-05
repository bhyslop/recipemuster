#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle GitHub Runner - Remote runner steps

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGR_INCLUDED:-}" || bcu_die "Module rbgr multiply included - check sourcing hierarchy"
ZRBGR_INCLUDED=1

######################################################################
# Internal Functions (zrbgr_*)

zrbgr_kindle() {
  # Check required environment
  test -n "${RBG_TEMP_DIR:-}"             || bcu_die "RBG_TEMP_DIR not set"
  test -n "${RBG_NOW_STAMP:-}"            || bcu_die "RBG_NOW_STAMP not set"
  test -n "${RBRR_BUILD_ARCHITECTURES:-}" || bcu_die "RBRR_BUILD_ARCHITECTURES not set"
  test -n "${RBRR_HISTORY_DIR:-}"         || bcu_die "RBRR_HISTORY_DIR not set"
  test -n "${RBRR_REGISTRY_OWNER:-}"      || bcu_die "RBRR_REGISTRY_OWNER not set"
  test -n "${RBRR_REGISTRY_NAME:-}"       || bcu_die "RBRR_REGISTRY_NAME not set"

  # Module Variables (ZRBGR_*)
  ZRBGR_ALL_VERSIONS_FILE="${RBG_TEMP_DIR}/rbgr_all_versions.json"
  ZRBGR_PAGE_FILE="${RBG_TEMP_DIR}/rbgr_page.json"
  ZRBGR_ALL_VERSIONS_TMP_FILE="${RBG_TEMP_DIR}/rbgr_all_versions.tmp"
  ZRBGR_DELETE_RESULT_FILE="${RBG_TEMP_DIR}/rbgr_delete_result.txt"
  ZRBGR_HTTP_CODE_FILE="${RBG_TEMP_DIR}/rbgr_delete_http_code.txt"
  ZRBGR_MAIN_WORKFLOW_FILE="${RBG_TEMP_DIR}/main_workflow.yml"

  if test -n "${GITHUB_ACTIONS:-}"; then
    local z_workflow_file=".github/workflows/${GITHUB_WORKFLOW}.yml"
    
    bcu_step "Extract workflow from current HEAD"
    test -f "${z_workflow_file}" \
      || bcu_die "Workflow ${z_workflow_file} not found in checked out ref"
    
    bcu_step "Fetch just the main ref (lightweight)"
    git fetch --depth=1 origin main 2>/dev/null \
      || bcu_die "Failed to fetch main branch reference"
    
    bcu_step "Extract workflow from main to temp file"
    git show origin/main:"${z_workflow_file}" > "${ZRBGR_MAIN_WORKFLOW_FILE}" 2>/dev/null \
      || bcu_die "Workflow ${z_workflow_file} not found in main branch"
    
    bcu_step "Compare files"
    diff -q "${z_workflow_file}" "${ZRBGR_MAIN_WORKFLOW_FILE}" >/dev/null \
      || bcu_die "Workflow ${GITHUB_WORKFLOW} differs between main and checked out ref"
  fi

  ZRBGR_KINDLED=1
}

zrbgr_sentinel() {
  test "${ZRBGR_KINDLED:-}" = "1" || bcu_die "Module rbgr not kindled - call zrbgr_kindle first"
}

######################################################################
# External Functions (rbgr_*)

rbgr_build_image() {
  # Name parameters
  local z_dockerfile="${1:-}"
  local z_build_label="${2:-}"

  # Handle documentation mode
  bcu_doc_brief "Build and push container image"
  bcu_doc_param "dockerfile" "Path to Dockerfile"
  bcu_doc_param "build_label" "Build label for image tag"
  bcu_doc_shown || return 0

  # Ensure module started
  zrbgr_sentinel

  # Validate parameters
  test -n "${z_dockerfile}" || bcu_die "Dockerfile path required"
  test -f "${z_dockerfile}" || bcu_die "Dockerfile not found: ${z_dockerfile}"
  test -n "${z_build_label}" || bcu_die "Build label required"

  # Create FQIN using rbcr
  rbcr_make_fqin "${z_build_label}"
  local z_ghcr_path
  z_ghcr_path=$(<"${ZRBCR_FQIN_FILE}")

  # Check if tag exists
  if rbcr_exists_predicate "${z_build_label}"; then
    bcu_die "Tag ${z_build_label} already exists"
  fi

  # Create history directory
  local z_history_dir="${RBRR_HISTORY_DIR}/${z_build_label}"
  mkdir -p "${z_history_dir}" || bcu_die "Failed to create history directory"
  cp "${z_dockerfile}" "${z_history_dir}/recipe.txt" || bcu_die "Failed to copy Dockerfile"
  echo "${GITHUB_SHA:-unknown}" > "${z_history_dir}/commit.txt"

  # Build and push image
  bcu_step "Building multi-platform image"
  docker buildx build                        \
    --push                                   \
    --tag "${z_ghcr_path}"                   \
    --platform "${RBRR_BUILD_ARCHITECTURES}" \
    --provenance=true                        \
    --sbom=true                              \
    --file "${z_dockerfile}"                 \
    . || bcu_die "Docker build failed"

  # Run Syft analysis
  bcu_step "Running Syft analysis"

  # Install Syft
  curl -sSfL https://github.com/anchore/syft/releases/download/v1.14.1/syft_1.14.1_linux_amd64.tar.gz -o syft.tar.gz && \
    tar -xzf syft.tar.gz syft && rm syft.tar.gz && sudo mv syft /usr/local/bin/ || \
    bcu_die "Failed to install Syft"

  # Pull and analyze
  docker pull "${z_ghcr_path}" || bcu_die "Failed to pull built image"
  syft "${z_ghcr_path}" -o json > "${z_history_dir}/syft_analysis.json" || \
    bcu_die "Syft analysis failed"

  # Generate summary
  echo "Package analysis summary:" > "${z_history_dir}/package_summary.txt"
  jq -r '.artifacts[] | "\(.name) \(.version)"' "${z_history_dir}/syft_analysis.json" | \
    sort | uniq -c | sort -rn | head -n 20 >> "${z_history_dir}/package_summary.txt"

  # Create rough digest
  local z_image_size z_image_size_mb
  z_image_size=$(docker image inspect "${z_ghcr_path}" --format='{{.Size}}')
  z_image_size_mb=$((z_image_size / 1024 / 1024))
  echo "Image size: ${z_image_size_mb}MB" > "${z_history_dir}/rough_digest.txt"

  # Extract digest metadata
  docker inspect "${z_ghcr_path}" | jq -r '.[0].Id | sub("sha256:"; "")' > "${z_history_dir}/docker_inspect_Id.txt"
  docker inspect "${z_ghcr_path}" | jq -r '.[0].RepoTags[0] // empty'    > "${z_history_dir}/docker_inspect_RepoTags_0.txt"
  docker inspect "${z_ghcr_path}" | jq -r '.[0].RepoDigests[-1]'         > "${z_history_dir}/docker_inspect_RepoDigests_last.txt"
  docker inspect "${z_ghcr_path}" | jq -r '.[0].Created'                 > "${z_history_dir}/docker_inspect_Created.txt"

  bcu_success "Image built successfully: ${z_build_label}"
}

rbgr_record_history() {
  # Name parameters
  local z_build_label="${1:-}"

  # Ensure module started
  zrbgr_sentinel

  # Validate parameters
  test -n "${z_build_label}" || bcu_die "Build label required"

  # Commit history
  bcu_step "Recording build history"

  git config --local user.email "github-actions[bot]@users.noreply.github.com" || \
    bcu_die "Failed to set git user email"
  git config --local user.name "github-actions[bot]" || \
    bcu_die "Failed to set git user name"

  local z_history_dir="${RBRR_HISTORY_DIR}/${z_build_label}"
  git add "${z_history_dir}" || bcu_die "Failed to stage history directory"
  git commit -m "Add image build history for ${z_build_label}" || \
    bcu_die "Failed to commit image build history"
  git push || bcu_die "Failed to push changes"
}

rbgr_delete_image() {
  # Name parameters
  local z_fqin="${1:-}"

  # Ensure module started
  zrbgr_sentinel

  # Validate parameters
  test -n "${z_fqin}" || bcu_die "FQIN required"

  # Extract tag
  local z_tag="${z_fqin#*:}"

  # Delete via rbcr
  bcu_step "Deleting image tag: ${z_tag}"
  rbcr_delete "${z_tag}"

  # Record deletion
  local z_delete_dir="${RBRR_HISTORY_DIR}/_deletions/${RBG_NOW_STAMP}_${z_tag}"

  mkdir -p                        "${z_delete_dir}"
  echo "${z_fqin}"              > "${z_delete_dir}/deleted_fqin.txt"
  echo "${z_tag}"               > "${z_delete_dir}/deleted_tag.txt"
  date -u +"%Y-%m-%dT%H:%M:%SZ" > "${z_delete_dir}/deletion_timestamp.txt"
}

rbgr_clean_orphans() {
  # Ensure module started
  zrbgr_sentinel

  bcu_step "Cleaning orphaned image versions"

  # Get all versions with pagination
  local z_page=1
  echo "[]" > "${ZRBGR_ALL_VERSIONS_FILE}"

  while true; do
    curl -s -H "Authorization: token ${GITHUB_TOKEN}" \
         -H "Accept: application/vnd.github.v3+json" \
         "https://api.github.com/user/packages/container/${RBRR_REGISTRY_NAME}/versions?per_page=100&page=${z_page}" \
         > "${ZRBGR_PAGE_FILE}"

    local z_items
    z_items=$(jq '. | length' "${ZRBGR_PAGE_FILE}")
    test "${z_items}" -ne 0 || break

    jq -s '.[0] + .[1]' "${ZRBGR_ALL_VERSIONS_FILE}" "${ZRBGR_PAGE_FILE}" > "${ZRBGR_ALL_VERSIONS_TMP_FILE}"
    mv "${ZRBGR_ALL_VERSIONS_TMP_FILE}" "${ZRBGR_ALL_VERSIONS_FILE}"

    z_page=$((z_page + 1))
  done

  # Get untagged versions
  local z_orphan_count
  z_orphan_count=$(jq '[.[] | select(.metadata.container.tags | length == 0)] | length' "${ZRBGR_ALL_VERSIONS_FILE}")

  if test "${z_orphan_count}" -eq 0; then
    bcu_info "No orphaned versions to clean"
    return 0
  fi

  bcu_info "Found ${z_orphan_count} untagged versions to clean"

  # Delete orphans
  local z_deleted_count=0
  local z_orphan_id

  jq -r '.[] | select(.metadata.container.tags | length == 0) | .id' "${ZRBGR_ALL_VERSIONS_FILE}" | \
  while read -r z_orphan_id; do
    bcu_step "Deleting orphan ID ${z_orphan_id}... "

    curl -X DELETE -s                               \
        -H "Authorization: token ${GITHUB_TOKEN}"   \
        -H "Accept: application/vnd.github.v3+json" \
        -w "%{http_code}"                           \
        -o "${ZRBGR_DELETE_RESULT_FILE}"            \
        "https://api.github.com/user/packages/container/${RBRR_REGISTRY_NAME}/versions/${z_orphan_id}" \
        > "${ZRBGR_HTTP_CODE_FILE}"                 \
      || bcu_die "Failed to delete orphan"

    local z_http_code
    z_http_code=$(<"${ZRBGR_HTTP_CODE_FILE}")

    if test "${z_http_code}" = "204"; then
      echo "deleted"
      z_deleted_count=$((z_deleted_count + 1))
    else
      echo "failed (HTTP ${z_http_code})"
    fi

    sleep 0.5
  done

  bcu_info "Deleted ${z_deleted_count} orphaned versions"
}

# eof

