#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle GitHub Host - GitHub-specific orchestration

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBHH_INCLUDED:-}" || buc_die "Module rbhh multiply included - check sourcing hierarchy"
ZRBHH_INCLUDED=1

######################################################################
# Internal Functions (zrbhh_*)

zrbhh_kindle() {
  # Check required environment
  test -n "${RBRR_REGISTRY_OWNER:-}" || buc_die "RBRR_REGISTRY_OWNER not set"
  test -n "${RBRR_REGISTRY_NAME:-}"  || buc_die "RBRR_REGISTRY_NAME not set"
  test -n "${RBRR_HISTORY_DIR:-}"    || buc_die "RBRR_HISTORY_DIR not set"
  test -n "${BURD_TEMP_DIR:-}"        || buc_die "BURD_TEMP_DIR not set"

  # Module Variables (ZRBHH_*)
  ZRBHH_BUILD_DIR_LATEST_FILE="${BURD_TEMP_DIR}/latest_build_dir.txt"

  ZRBHH_KINDLED=1
}

zrbhh_sentinel() {
  test "${ZRBHH_KINDLED:-}" = "1" || buc_die "Module rbhh not kindled - call zrbhh_kindle first"
}

zrbhh_get_latest_build_dir() {
  local z_recipe_basename="$1"
  local z_basename_no_ext="${z_recipe_basename%.*}"

  find "${RBRR_HISTORY_DIR}" -name "${z_basename_no_ext}*" -type d -print | \
    sort -r | head -n1 > "${ZRBHH_BUILD_DIR_LATEST_FILE}"
}

zrbhh_pull_with_retry() {
  local z_success_msg="${1:-Git pull completed}"
  local z_no_commits_msg="${2:-No new commits after many attempts}"

  buc_info "Git pull with retry..."

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
    buc_die "Expected git commits from workflow not found"
  }

  buc_info "${z_success_msg}"
}

######################################################################
# External Functions (rbhh_*)

rbhh_check_git_status() {
  # Ensure module started
  zrbhh_sentinel

  buc_info "Make sure your local repo is up to date..."

  git fetch

# Replace the existing ahead/behind detection in rbhh_check_git_status with:

  local z_commits_ahead z_commits_behind
  z_commits_ahead=$(git rev-list --count @{u}..HEAD 2>/dev/null || echo "0")
  z_commits_behind=$(git rev-list --count HEAD..@{u} 2>/dev/null || echo "0")

  if test "${z_commits_ahead}" -gt 0; then
    buc_die "Your repo is ahead of the remote branch by ${z_commits_ahead} commit(s). Push changes to proceed: git push"
  elif test "${z_commits_behind}" -gt 0; then
    buc_die "Your repo is behind the remote branch by ${z_commits_behind} commit(s). Pull latest changes to proceed: git pull"
  fi

  git status -uno | grep -q 'Your branch is up to date' || \
    buc_die "ERROR: Your repo is behind the remote branch. Pull latest changes to proceed."

  git diff-index --quiet HEAD -- || \
    buc_die "ERROR: Your repo has uncommitted changes. Commit or stash changes to proceed."
}

rbhh_build_workflow() {
  # Name parameters
  local z_recipe_file="${1:-}"

  # Ensure module started
  zrbhh_sentinel

  # Validate parameters
  test -n "${z_recipe_file}" || buc_die "Recipe file required"
  test -f "${z_recipe_file}" || buc_die "Recipe file not found: ${z_recipe_file}"

  # Get current commit hash
  local z_commit_ref
  z_commit_ref=$(git rev-parse HEAD) || buc_die "Failed to get current commit hash"

  # Dispatch workflow
  buc_step "Triggering GitHub Actions workflow for image build"
  rbga_dispatch "${RBRR_REGISTRY_OWNER}" "${RBRR_REGISTRY_NAME}" \
                "rbgr_build" '{"dockerfile": "'${z_recipe_file}'", "ref": "'${z_commit_ref}'"}'

  # Wait for completion
  rbga_wait_completion "${RBRR_REGISTRY_OWNER}" "${RBRR_REGISTRY_NAME}"

  # Pull artifacts
  zrbhh_pull_with_retry "Build artifacts retrieved"

  # Verify build output
  buc_info "Verifying build output..."
  local z_recipe_basename
  z_recipe_basename=$(basename "${z_recipe_file}")

  zrbhh_get_latest_build_dir "${z_recipe_basename}"
  local z_build_dir
  z_build_dir=$(<"${ZRBHH_BUILD_DIR_LATEST_FILE}")

  test -n "${z_build_dir}"                           || buc_die "Missing build directory"
  test -d "${z_build_dir}"                           || buc_die "Invalid build directory"
  test -f "${z_build_dir}/recipe.txt"                || buc_die "recipe.txt not found"
  cmp "${z_recipe_file}" "${z_build_dir}/recipe.txt" || buc_die "recipe mismatch"

  # Extract FQIN
  buc_info "Extracting FQIN..."
  local z_fqin_file="${z_build_dir}/docker_inspect_RepoTags_0.txt"
  test -f "${z_fqin_file}" || buc_die "Could not find FQIN in build output"

  local z_fqin_contents
  z_fqin_contents=$(<"${z_fqin_file}")

  buc_info "Built image FQIN: ${z_fqin_contents}"

  # Handle optional output
  if test -n "${RBG_ARG_FQIN_OUTPUT:-}"; then
    cp "${z_fqin_file}" "${RBG_ARG_FQIN_OUTPUT}"
    buc_info "Wrote FQIN to ${RBG_ARG_FQIN_OUTPUT}"
  fi

  # Verify availability
  buc_info "Verifying image availability in registry..."
  local z_tag="${z_fqin_contents#*:}"
  echo "Waiting for tag: ${z_tag} to become available..."

  local z_i
  for z_i in 1 2 3 4 5; do
    if rbcr_exists_predicate "${z_tag}"; then
      break
    fi

    echo "  Image not yet available, attempt ${z_i} of 5"
    test ${z_i} -ne 5 || buc_die "Image '${z_tag}' not available in registry after 5 attempts"
    sleep 5
  done
}

rbhh_delete_workflow() {
  # Name parameters
  local z_fqin="${1:-}"

  # Ensure module started
  zrbhh_sentinel

  # Validate parameters
  test -n "${z_fqin}" || buc_die "FQIN required"

  local z_commit_ref
  z_commit_ref=$(git rev-parse HEAD) || buc_die "Failed to get current commit hash"

  buc_step "Preparing GitHub Actions deletion arguments"
  local z_escaped_fqin="${z_fqin//\\/\\\\}"
  z_escaped_fqin="${z_escaped_fqin//\"/\\\"}"

  local z_escaped_ref="${z_commit_ref//\\/\\\\}"
  z_escaped_ref="${z_escaped_ref//\"/\\\"}"

  printf '{"fqin": "%s", "ref": "%s"}' "${z_escaped_fqin}" "${z_escaped_ref}" > "${BURD_TEMP_DIR}/delete_payload.json"
  local z_payload
  z_payload=$(<"${BURD_TEMP_DIR}/delete_payload.json")

  buc_step "Triggering GitHub Actions deletion with z_payload=${z_payload}"
  rbga_dispatch "${RBRR_REGISTRY_OWNER}" "${RBRR_REGISTRY_NAME}" "rbgr_delete" "${z_payload}"

  # Wait for completion
  rbga_wait_completion "${RBRR_REGISTRY_OWNER}" "${RBRR_REGISTRY_NAME}"

  # Pull deletion history
  zrbhh_pull_with_retry "Deletion history retrieved" "No deletion history recorded"

  # Verify deletion
  buc_info "Verifying deletion..."
  local z_tag="${z_fqin#*:}"

  echo "  Checking that tag '${z_tag}' is gone..."
  if rbcr_exists_predicate "${z_tag}"; then
    buc_die "Tag '${z_tag}' still exists in registry after deletion"
  fi

  echo "  Confirmed: Tag '${z_tag}' has been deleted"
}

# eof

