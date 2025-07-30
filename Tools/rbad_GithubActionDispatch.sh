#!/bin/bash
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
# Recipe Bottle Action Dispatch - GitHub Actions Workflow Management

set -euo pipefail

ZRBAD_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBAD_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBAD_SCRIPT_DIR}/bvu_BashValidationUtility.sh"
source "${ZRBAD_SCRIPT_DIR}/rbad_implementation.sh"

######################################################################
# Internal Functions (zrbad_*)

zrbad_environment() {
  # Handle documentation mode
  bcu_doc_env "RBAD_TEMP_DIR    " "Empty temporary directory"
  bcu_doc_env "RBAD_NOW_STAMP   " "Timestamp for per run branding"
  bcu_doc_env "RBAD_RBRR_FILE   " "File containing the RBRR constants"

  bcu_env_done || return 0

  # Validate environment
  bvu_dir_exists  "${RBAD_TEMP_DIR}"
  bvu_dir_empty   "${RBAD_TEMP_DIR}"
  bvu_env_string     RBAD_NOW_STAMP   1 128   # weak validation but infrastructure managed
  bvu_file_exists "${RBAD_RBRR_FILE}"

  source              "${RBAD_RBRR_FILE}"
  source "${ZRBAD_SCRIPT_DIR}/rbrr.validator.sh"

  # Source GitHub PAT credentials
  bvu_file_exists "${RBRR_GITHUB_PAT_ENV}"
  source          "${RBRR_GITHUB_PAT_ENV}"

  # Extract and validate PAT credentials
  test -n "${RBRG_PAT:-}"      || bcu_die "RBRG_PAT missing from ${RBRR_GITHUB_PAT_ENV}"
  test -n "${RBRG_USERNAME:-}" || bcu_die "RBRG_USERNAME missing from ${RBRR_GITHUB_PAT_ENV}"

  # Export variables for implementation layer
  export RBAD_TEMP_DIR="${RBAD_TEMP_DIR}"
  export RBAD_NOW_STAMP="${RBAD_NOW_STAMP}"
  export RBRG_PAT="${RBRG_PAT}"
  export RBRG_USERNAME="${RBRG_USERNAME}"

  # Module Variables (ZRBAD_*)
  ZRBAD_WORKFLOW_LOGS="${RBAD_TEMP_DIR}/workflow_logs__${RBAD_NOW_STAMP}.txt"
}

# Check git repository status
zrbad_check_git_status() {
  bcu_info "Make sure your local repo is up to date with github variant..."

  git fetch

  git status -uno | grep -q 'Your branch is up to date'                \
    || bcu_die "ERROR: Your repo is behind the remote branch."         \
               "       Pull latest changes to proceed (prevents merge" \
               "       conflicts with image history tracking)."

  git diff-index --quiet HEAD --                                           \
    || bcu_die "ERROR: Your repo has uncommitted changes."                 \
               "       Commit or stash changes to proceed (prevents merge" \
               "       conflicts with image history tracking)."
}

# Find the latest build directory for a recipe
zrbad_get_latest_build_dir() {
  local recipe_basename="$1"
  local basename_no_ext="${recipe_basename%.*}"

  find "${RBRR_HISTORY_DIR}" -name "${basename_no_ext}*" -type d -print | sort -r | head -n1
}

######################################################################
# External Functions (rbad_*)

rbad_build() {
  local recipe_file="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Build container image using GitHub Actions"
  bcu_doc_param "recipe_file" "Path to recipe file containing build instructions"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "${recipe_file}" || bcu_usage_die
  test -f "${recipe_file}" || bcu_die "Recipe file not found: ${recipe_file}"

  # Perform command
  local  recipe_basename=$(basename "${recipe_file}")
  echo "$recipe_basename" | grep -q '[A-Z]' && \
      bcu_die "Basename of '${recipe_file}' contains uppercase letters so cannot use in image name"

  bcu_step "Trigger image build from ${recipe_file}"

  zrbad_check_git_status

  bcu_step "Triggering GitHub Actions workflow for image build"
  rbadi_execute_workflow "build_images"                         \
                         '{"dockerfile": "'${recipe_file}'"}'   \
                         "Git Pull for artifacts with retry..."

  bcu_info "Verifying build output..."
  local      build_dir=$(zrbad_get_latest_build_dir "$recipe_basename")
  test -n "${build_dir}"                         || bcu_die "Missing build directory"
  test -d "${build_dir}"                         || bcu_die "Invalid build directory"
  test -f "${build_dir}/recipe.txt"              || bcu_die "recipe.txt not found"
  cmp "${recipe_file}" "${build_dir}/recipe.txt" || bcu_die "recipe mismatch"

  bcu_info "Extracting FQIN..."
  local fqin_file="${build_dir}/docker_inspect_RepoTags_0.txt"
  test -f "${fqin_file}" || bcu_die "Could not find FQIN in build output"

  local fqin_contents
  fqin_contents=$(<"${fqin_file}")

  bcu_info "Built image FQIN: ${fqin_contents}"

  if [ -n                  "${RBAD_ARG_FQIN_OUTPUT:-}" ]; then
    cp "${fqin_file}"      "${RBAD_ARG_FQIN_OUTPUT}"
    bcu_info "Wrote FQIN to ${RBAD_ARG_FQIN_OUTPUT}"
  fi

  bcu_info "Verifying image availability in registry..."
  local tag="${fqin_contents#*:}"

  # Import RBCR for verification
  source "${ZRBAD_SCRIPT_DIR}/rbcr_implementation.sh"
  export RBC_TEMP_DIR="${RBAD_TEMP_DIR}"
  export RBC_RUNTIME="docker"
  export RBC_RUNTIME_ARG=""

  rbcri_start

  echo "Waiting for tag: ${tag} to become available..."
  for i in 1 2 3 4 5; do
    ! rbcri_exists "${tag}" || break

    echo "  Image not yet available, attempt $i of 5"
    test $i -ne 5 || bcu_die "Image '${tag}' not available in registry after 5 attempts"
    sleep 5
  done

  bcu_success "No errors."
}

rbad_dispatch() {
  local event_type="${1:-}"
  local payload="${2:-}"

  # Handle documentation mode
  bcu_doc_brief "Dispatch a GitHub Actions workflow"
  bcu_doc_param "event_type" "The event type to trigger"
  bcu_doc_param "payload"    "JSON payload for the workflow"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "${event_type}" || bcu_usage_die
  test -n "${payload}"    || bcu_usage_die

  # Perform command
  bcu_step "Dispatching workflow with event type: ${event_type}"

  rbadi_dispatch_workflow "${event_type}" "${payload}"

  bcu_success "Workflow dispatched."
}

rbad_wait() {
  local run_id="${1:-}"
  local success_message="${2:-Workflow completed}"

  # Handle documentation mode
  bcu_doc_brief "Wait for a GitHub Actions workflow to complete"
  bcu_doc_param "run_id"          "Workflow run ID to monitor"
  bcu_doc_oparm "success_message" "Message to display on success"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "${run_id}" || bcu_usage_die

  # Perform command
  bcu_step "Waiting for workflow run ${run_id}"

  rbadi_wait_for_workflow "${run_id}" "${success_message}"

  # Save logs
  rbadi_fetch_workflow_logs "${run_id}" "${ZRBAD_WORKFLOW_LOGS}"

  bcu_success "Workflow completed successfully."
}

test "${BASH_SOURCE[0]}" != "${0}" || bcu_die "This file must be directly executed, not sourced."

bcu_execute rbad_ "Recipe Bottle Action Dispatch - GitHub Actions Workflow Management" zrbad_environment "$@"

# eof
