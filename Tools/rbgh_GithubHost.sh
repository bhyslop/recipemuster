#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle GitHub Host - GitHub-specific orchestration

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGH_INCLUDED:-}" || bcu_die "Module rbgh multiply included - check sourcing hierarchy"
ZRBGH_INCLUDED=1

######################################################################
# Internal Functions (zrbgh_*)

zrbgh_kindle() {
  # Check required environment
  test -n "${RBRR_REGISTRY_OWNER:-}" || bcu_die "RBRR_REGISTRY_OWNER not set"
  test -n "${RBRR_REGISTRY_NAME:-}" || bcu_die "RBRR_REGISTRY_NAME not set"
  test -n "${RBRR_HISTORY_DIR:-}" || bcu_die "RBRR_HISTORY_DIR not set"
  test -n "${RBG_TEMP_DIR:-}" || bcu_die "RBG_TEMP_DIR not set"

  # Module Variables (ZRBGH_*)
  ZRBGH_BUILD_DIR_LATEST_FILE="${RBG_TEMP_DIR}/latest_build_dir.txt"

  ZRBGH_KINDLED=1
}

zrbgh_sentinel() {
  test "${ZRBGH_KINDLED:-}" = "1" || bcu_die "Module rbgh not kindled - call zrbgh_kindle first"
}

zrbgh_get_latest_build_dir() {
  local z_recipe_basename="$1"
  local z_basename_no_ext="${z_recipe_basename%.*}"

  find "${RBRR_HISTORY_DIR}" -name "${z_basename_no_ext}*" -type d -print | \
    sort -r | head -n1 > "${ZRBGH_BUILD_DIR_LATEST_FILE}"
}

zrbgh_pull_with_retry() {
  local z_success_msg="${1:-Git pull completed}"
  local z_no_commits_msg="${2:-No new commits after many attempts}"

  bcu_info "Git pull with retry..."

  local z_retry_wait=5
  local z_max_attempts=30
  local z_i=0
  local z_found=0

  while test ${z_i} -lt ${z_max_attempts}; do
    echo "  Attempt $((z_i + 1)): Checking for remote changes..."
    git fetch --quiet

    local z_count
    z_count=$(git rev-list --count HEAD..origin/main 2>/dev/null || echo 0)

    if test "${z_count}" -gt 0; then
      echo "  Found ${z_count} new commits, pulling..."
      git pull
      echo "  Pull successful"
      z_found=1
      break
    fi

    echo "  No new commits yet, waiting ${z_retry_wait} seconds (attempt $((z_i + 1)) of ${z_max_attempts})"
    sleep ${z_retry_wait}
    z_i=$((z_i + 1))
  done

  test ${z_found} -eq 1 || {
    echo "  ${z_no_commits_msg}"
    bcu_die "Expected git commits from workflow not found"
  }

  bcu_info "${z_success_msg}"
}

######################################################################
# External Functions (rbgh_*)

rbgh_check_git_status() {
  # Ensure module started
  zrbgh_sentinel

  bcu_info "Make sure your local repo is up to date..."

  git fetch

  git status -uno | grep -q 'Your branch is up to date' || \
    bcu_die "ERROR: Your repo is behind the remote branch. Pull latest changes to proceed."

  git diff-index --quiet HEAD -- || \
    bcu_die "ERROR: Your repo has uncommitted changes. Commit or stash changes to proceed."
}

rbgh_build_workflow() {
  # Name parameters
  local z_recipe_file="${1:-}"

  # Ensure module started
  zrbgh_sentinel

  # Validate parameters
  test -n "${z_recipe_file}" || bcu_die "Recipe file required"
  test -f "${z_recipe_file}" || bcu_die "Recipe file not found: ${z_recipe_file}"

  # Dispatch workflow
  bcu_step "Triggering GitHub Actions workflow for image build"
  rbga_dispatch "${RBRR_REGISTRY_OWNER}" "${RBRR_REGISTRY_NAME}" \
                "rbgr_build" '{"dockerfile": "'${z_recipe_file}'"}'

  # Wait for completion
  rbga_wait_completion "${RBRR_REGISTRY_OWNER}" "${RBRR_REGISTRY_NAME}"

  # Pull artifacts
  zrbgh_pull_with_retry "Build artifacts retrieved"

  # Verify build output
  bcu_info "Verifying build output..."
  local z_recipe_basename
  z_recipe_basename=$(basename "${z_recipe_file}")

  zrbgh_get_latest_build_dir "${z_recipe_basename}"
  local z_build_dir
  z_build_dir=$(<"${ZRBGH_BUILD_DIR_LATEST_FILE}")

  test -n "${z_build_dir}" || bcu_die "Missing build directory"
  test -d "${z_build_dir}" || bcu_die "Invalid build directory"
  test -f "${z_build_dir}/recipe.txt" || bcu_die "recipe.txt not found"
  cmp "${z_recipe_file}" "${z_build_dir}/recipe.txt" || bcu_die "recipe mismatch"

  # Extract FQIN
  bcu_info "Extracting FQIN..."
  local z_fqin_file="${z_build_dir}/docker_inspect_RepoTags_0.txt"
  test -f "${z_fqin_file}" || bcu_die "Could not find FQIN in build output"

  local z_fqin_contents
  z_fqin_contents=$(<"${z_fqin_file}")

  bcu_info "Built image FQIN: ${z_fqin_contents}"

  # Handle optional output
  if test -n "${RBG_ARG_FQIN_OUTPUT:-}"; then
    cp "${z_fqin_file}" "${RBG_ARG_FQIN_OUTPUT}"
    bcu_info "Wrote FQIN to ${RBG_ARG_FQIN_OUTPUT}"
  fi

  # Verify availability
  bcu_info "Verifying image availability in registry..."
  local z_tag="${z_fqin_contents#*:}"
  echo "Waiting for tag: ${z_tag} to become available..."

  local z_i
  for z_i in 1 2 3 4 5; do
    if rbcr_exists_predicate "${z_tag}"; then
      break
    fi

    echo "  Image not yet available, attempt ${z_i} of 5"
    test ${z_i} -ne 5 || bcu_die "Image '${z_tag}' not available in registry after 5 attempts"
    sleep 5
  done
}

rbgh_delete_workflow() {
  # Name parameters
  local z_fqin="${1:-}"

  # Ensure module started
  zrbgh_sentinel

  # Validate parameters
  test -n "${z_fqin}" || bcu_die "FQIN required"

  # Dispatch workflow
  bcu_step "Triggering GitHub Actions workflow for image deletion"
  rbga_dispatch "${RBRR_REGISTRY_OWNER}" "${RBRR_REGISTRY_NAME}" \
                "rbgr_delete" '{"fqin": "'${z_fqin}'"}'

  # Wait for completion
  rbga_wait_completion "${RBRR_REGISTRY_OWNER}" "${RBRR_REGISTRY_NAME}"

  # Pull deletion history
  zrbgh_pull_with_retry "Deletion history retrieved" "No deletion history recorded"

  # Verify deletion
  bcu_info "Verifying deletion..."
  local z_tag="${z_fqin#*:}"

  echo "  Checking that tag '${z_tag}' is gone..."
  if rbcr_exists_predicate "${z_tag}"; then
    bcu_die "Tag '${z_tag}' still exists in registry after deletion"
  fi

  echo "  Confirmed: Tag '${z_tag}' has been deleted"
}

# eof

