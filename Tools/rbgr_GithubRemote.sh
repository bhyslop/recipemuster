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
  test -n "${RBRR_BUILD_ARCHITECTURES:-}" || bcu_die "RBRR_BUILD_ARCHITECTURES not set"
  test -n "${RBRR_HISTORY_DIR:-}"         || bcu_die "RBRR_HISTORY_DIR not set"
  test -n "${RBRR_REGISTRY_OWNER:-}"      || bcu_die "RBRR_REGISTRY_OWNER not set"
  test -n "${RBRR_REGISTRY_NAME:-}"       || bcu_die "RBRR_REGISTRY_NAME not set"

  ZRBGR_KINDLED=1
}

zrbgr_sentinel() {
  test "${ZRBGR_KINDLED:-}" = "1" || bcu_die "Module rbgr not started - call zrbgr_kindle first"
}

######################################################################
# External Functions (rbgr_*)

rbgr_build_image() {
  # Name parameters
  local z_dockerfile="${1:-}"

  # Ensure module started
  zrbgr_sentinel

  # Validate parameters
  test -n "${z_dockerfile}" || bcu_die "Dockerfile path required"
  test -f "${z_dockerfile}" || bcu_die "Dockerfile not found: ${z_dockerfile}"

  # Generate build metadata
  local z_timestamp
  z_timestamp=$(date +'%Y%m%d__%H%M%S')
  local z_full_filename
  z_full_filename=$(basename "${z_dockerfile}")
  local z_filename_no_ext="${z_full_filename%.*}"
  local z_build_label="${z_filename_no_ext}.${z_timestamp}"

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
  docker buildx build \
    --push \
    --tag "${z_ghcr_path}" \
    --platform "${RBRR_BUILD_ARCHITECTURES}" \
    --provenance=true \
    --sbom=true \
    --file "${z_dockerfile}" \
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
  docker inspect "${z_ghcr_path}" | jq -r '.[0].RepoTags[0] // empty' > "${z_history_dir}/docker_inspect_RepoTags_0.txt"
  docker inspect "${z_ghcr_path}" | jq -r '.[0].RepoDigests[-1]' > "${z_history_dir}/docker_inspect_RepoDigests_last.txt"
  docker inspect "${z_ghcr_path}" | jq -r '.[0].Created' > "${z_history_dir}/docker_inspect_Created.txt"
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
  local z_timestamp
  z_timestamp=$(date +'%Y%m%d__%H%M%S')
  local z_delete_dir="${RBRR_HISTORY_DIR}/_deletions/${z_timestamp}_${z_tag}"

  mkdir -p "${z_delete_dir}"
  echo "${z_fqin}" > "${z_delete_dir}/deleted_fqin.txt"
  echo "${z_tag}" > "${z_delete_dir}/deleted_tag.txt"
  date -u +"%Y-%m-%dT%H:%M:%SZ" > "${z_delete_dir}/deletion_timestamp.txt"
}

rbgr_clean_orphans() {
  # Ensure module started
  zrbgr_sentinel

  bcu_step "Cleaning orphaned image versions"

  # Get all versions with pagination
  local z_page=1
  echo "[]" > /tmp/all_versions.json

  while true; do
    curl -s -H "Authorization: token ${GITHUB_TOKEN}" \
         -H "Accept: application/vnd.github.v3+json" \
         "https://api.github.com/user/packages/container/${RBRR_REGISTRY_NAME}/versions?per_page=100&page=${z_page}" \
         > /tmp/page.json

    local z_items
    z_items=$(jq '. | length' /tmp/page.json)
    test "${z_items}" -ne 0 || break

    jq -s '.[0] + .[1]' /tmp/all_versions.json /tmp/page.json > /tmp/all_versions.tmp
    mv /tmp/all_versions.tmp /tmp/all_versions.json

    z_page=$((z_page + 1))
  done

  # Get untagged versions
  local z_orphan_count
  z_orphan_count=$(jq '[.[] | select(.metadata.container.tags | length == 0)] | length' /tmp/all_versions.json)

  if test "${z_orphan_count}" -eq 0; then
    bcu_info "No orphaned versions to clean"
    return 0
  fi

  bcu_info "Found ${z_orphan_count} untagged versions to clean"

  # Delete orphans
  local z_deleted_count=0
  local z_orphan_id

  jq -r '.[] | select(.metadata.container.tags | length == 0) | .id' /tmp/all_versions.json | \
  while read -r z_orphan_id; do
    echo -n "Deleting orphan ID ${z_orphan_id}... "

    local z_http_code
    z_http_code=$(curl -X DELETE -s \
      -H "Authorization: token ${GITHUB_TOKEN}" \
      -H "Accept: application/vnd.github.v3+json" \
      -w "%{http_code}" \
      -o /tmp/delete_result.txt \
      "https://api.github.com/user/packages/container/${RBRR_REGISTRY_NAME}/versions/${z_orphan_id}")

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

