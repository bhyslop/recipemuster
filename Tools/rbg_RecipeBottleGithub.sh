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
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# Recipe Bottle GitHub - Image Registry Management

set -euo pipefail

# Multiple inclusion guard
[[ -n "${ZRBG_INCLUDED:-}" ]] && return 0
ZRBG_INCLUDED=1

ZRBG_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBG_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBG_SCRIPT_DIR}/bvu_BashValidationUtility.sh"

######################################################################
# Internal Functions (zrbg_*)

zrbg_environment() {
  # Handle documentation mode
  bcu_doc_env "RBG_TEMP_DIR  " "Empty temporary directory"
  bcu_doc_env "RBG_NOW_STAMP " "Timestamp for per run branding"
  bcu_doc_env "RBG_RBRR_FILE " "File containing the RBRR constants"
  bcu_doc_env "RBG_RUNTIME   " "Container runtime (docker/podman)"
  bcu_doc_env "RBG_RUNTIME_ARG" "Container runtime arguments (optional)"

  bcu_env_done || return 0

  # Validate environment
  bvu_dir_exists  "${RBG_TEMP_DIR}"
  bvu_dir_empty   "${RBG_TEMP_DIR}"
  bvu_env_string     RBG_NOW_STAMP   1 128   # weak validation but infrastructure managed
  bvu_file_exists "${RBG_RBRR_FILE}"
  bvu_env_string     RBG_RUNTIME     1 20 "podman"

  # Optional runtime args
  RBG_RUNTIME_ARG="${RBG_RUNTIME_ARG:-}"

  source              "${RBG_RBRR_FILE}"
  source "${ZRBG_SCRIPT_DIR}/rbrr.validator.sh"

  bvu_file_exists "${RBRR_GITHUB_PAT_ENV}"
  source          "${RBRR_GITHUB_PAT_ENV}"

  # Extract PAT credentials
  test -n "${RBRG_PAT:-}"      || bcu_die "RBRG_PAT missing from ${RBRR_GITHUB_PAT_ENV}"
  test -n "${RBRG_USERNAME:-}" || bcu_die "RBRG_USERNAME missing from ${RBRR_GITHUB_PAT_ENV}"

  # Module Variables (ZRBG_*)

  # Base URLs
  ZRBG_GIT_REGISTRY="ghcr.io"
  ZRBG_GITAPI_URL="https://api.github.com"
  ZRBG_REPO_PREFIX="${ZRBG_GITAPI_URL}/repos"

  # Derived URLs
  ZRBG_DISPATCH_URL="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/dispatches"
  ZRBG_RUNS_URL_BASE="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs"
  ZRBG_PACKAGES_URL="${ZRBG_GITAPI_URL}/user/packages/container/${RBRR_REGISTRY_NAME}/versions"
  ZRBG_IMAGE_PREFIX="${ZRBG_GIT_REGISTRY}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}"
  ZRBG_GITHUB_ACTIONS_URL="https://github.com/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs/"
  ZRBG_GITHUB_PACKAGES_URL="https://github.com/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/pkgs/container/${RBRR_REGISTRY_NAME}"

  # JSON output files
  # IMAGE_RECORDS.json - JSON array of image tag metadata from GHCR API
  # Each object: {tag: "<tag>", fqin: "<registry>/<owner>/<repo>:<tag>"}
  ZRBG_IMAGE_RECORDS_FILE="${RBG_TEMP_DIR}/IMAGE_RECORDS.json"

  # IMAGE_DETAILS.json - JSON array of per-tag/platform image details
  # Each object: {tag: "<tag>", platform: "<os>/<arch>", digest: "<sha256>",
  #               layers: [{digest: "<sha256>", size: <bytes>}],
  #               config: {created: "<iso>", architecture: "<arch>", os: "<os>"}}
  ZRBG_IMAGE_DETAIL_FILE="${RBG_TEMP_DIR}/IMAGE_DETAILS.json"

  # IMAGE_STATS.json - JSON array of deduplicated layer statistics
  # Each object: {digest: "<sha256>", size: <bytes>, tag_count: <int>,
  #               total_usage: <int>, tag_details: [{tag: "<tag>", count: <int>}]}
  ZRBG_IMAGE_STATS_FILE="${RBG_TEMP_DIR}/IMAGE_STATS.json"

  # Media types
  ZRBG_MTYPE_DLIST="application/vnd.docker.distribution.manifest.list.v2+json"
  ZRBG_MTYPE_OCI="application/vnd.oci.image.index.v1+json"
  ZRBG_MTYPE_DV2="application/vnd.docker.distribution.manifest.v2+json"
  ZRBG_MTYPE_OCM="application/vnd.oci.image.manifest.v1+json"
  ZRBG_ACCEPT_MANIFEST_MTYPES="${ZRBG_MTYPE_DV2},${ZRBG_MTYPE_DLIST},${ZRBG_MTYPE_OCI},${ZRBG_MTYPE_OCM}"
  ZRBG_SCHEMA_V2="2" # Docker Registry HTTP API V2 Schema 2 format - an older manifest format
  ZRBG_MTYPE_GHV3="application/vnd.github.v3+json"

  # GHCR v2 API
  ZRBG_GHCR_V2_API="https://ghcr.io/v2/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}"
  ZRBG_TOKEN_URL="https://ghcr.io/token?scope=repository:${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:pull&service=ghcr.io"

  # Temp files
  ZRBG_COLLECT_FULL_JSON="${RBG_TEMP_DIR}/RBG_COMBINED__${RBG_NOW_STAMP}.json"
  ZRBG_COLLECT_TEMP_PAGE="${RBG_TEMP_DIR}/RBG_PAGE__${RBG_NOW_STAMP}.json"
  ZRBG_CURRENT_WORKFLOW_RUN_CACHE="${RBG_TEMP_DIR}/CURR_WORKFLOW_RUN__${RBG_NOW_STAMP}.txt"
  ZRBG_DELETE_VERSION_ID_CACHE="${RBG_TEMP_DIR}/RBG_VERSION_ID__${RBG_NOW_STAMP}.txt"
  ZRBG_DELETE_RESULT_CACHE="${RBG_TEMP_DIR}/RBG_DELETE__${RBG_NOW_STAMP}.txt"
  ZRBG_WORKFLOW_LOGS="${RBG_TEMP_DIR}/workflow_logs__${RBG_NOW_STAMP}.txt"

  # Obtain bearer token for GHCR v2 API
  bcu_step "Obtaining bearer token for GHCR registry API"
  local token_out="${RBG_TEMP_DIR}/bearer_token.out"
  local token_err="${RBG_TEMP_DIR}/bearer_token.err"

  curl -sL -u "${RBRG_USERNAME}:${RBRG_PAT}" "${ZRBG_TOKEN_URL}" >"${token_out}" 2>"${token_err}" && \
    ZRBG_AUTH_TOKEN=$(jq -r '.token' "${token_out}") && \
    test -n "${ZRBG_AUTH_TOKEN}" && \
    test "${ZRBG_AUTH_TOKEN}" != "null" || {
      bcu_warn "Failed to obtain bearer token for GHCR API"
      bcu_warn "STDERR: $(<"${token_err}")"
      bcu_warn "STDOUT: $(<"${token_out}")"
      bcu_die "Cannot proceed without bearer token"
    }
}

# Perform authenticated GET request
# Usage: zrbg_curl_get <url>
zrbg_curl_get() {
  local url="$1"

  curl -s -H "Authorization: token ${RBRG_PAT}" \
          -H "Accept: ${ZRBG_MTYPE_GHV3}"       \
          "$url"
}

# Perform authenticated POST request
# Usage: zrbg_curl_post <url> <data>
zrbg_curl_post() {
  local url="$1"
  local data="$2"

  curl                                             \
       -s                                          \
       -X POST                                     \
       -H "Authorization: token ${RBRG_PAT}"       \
       -H "Accept: ${ZRBG_MTYPE_GHV3}"             \
       "$url"                                      \
       -d "$data"                                  \
    || bcu_die "POST request to GitHub API failed"
}

# Perform authenticated DELETE request
# Usage: zrbg_curl_delete <url>
zrbg_curl_delete() {
  local url="$1"

  curl -X DELETE -s -H "Authorization: token ${RBRG_PAT}"       \
                    -H "Accept: ${ZRBG_MTYPE_GHV3}"             \
                    "$url"                                      \
                    -w "\nHTTP_STATUS:%{http_code}\n"
}

# Collect all image records (version_id, tag, fqin) with pagination
#
# Outputs: JSON file at ZRBG_IMAGE_RECORDS_FILE
zrbg_collect_image_records() {
  bcu_step "Fetching all image records with pagination to ${ZRBG_IMAGE_RECORDS_FILE}"

  # Initialize empty array
  echo "[]" > "${ZRBG_IMAGE_RECORDS_FILE}"

  bcu_info "Retrieving paged results..."

  local page=1
  local temp_records="${RBG_TEMP_DIR}/temp_records_${RBG_NOW_STAMP}.json"

  while true; do
    bcu_info "  Fetching page ${page}..."

    local url="${ZRBG_PACKAGES_URL}?per_page=100&page=${page}"
    zrbg_curl_get "$url" > "${ZRBG_COLLECT_TEMP_PAGE}"

    local items=$(jq '. | length' "${ZRBG_COLLECT_TEMP_PAGE}")
    bcu_info "  Saw ${items} items on page ${page}..."

    test "${items}" -ne 0 || break

    # Transform to simplified records
    jq -r --arg prefix "${ZRBG_IMAGE_PREFIX}" \
      '[.[] | select(.metadata.container.tags | length > 0) |
       .id as $id | .metadata.container.tags[] as $tag |
       {version_id: $id, tag: $tag, fqin: ($prefix + ":" + $tag)}]' \
      "${ZRBG_COLLECT_TEMP_PAGE}" > "${temp_records}"

    # Merge with existing records
    jq -s '.[0] + .[1]' "${ZRBG_IMAGE_RECORDS_FILE}" "${temp_records}" > \
       "${ZRBG_IMAGE_RECORDS_FILE}.tmp"
    mv "${ZRBG_IMAGE_RECORDS_FILE}.tmp" "${ZRBG_IMAGE_RECORDS_FILE}"

    page=$((page + 1))
  done

  local total=$(jq '. | length' "${ZRBG_IMAGE_RECORDS_FILE}")
  bcu_info "  Retrieved ${total} total image records"
  bcu_success "Pagination complete."
}

# Check git repository status
zrbg_check_git_status() {
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
# Usage: zrbg_get_latest_build_dir <recipe_basename>
zrbg_get_latest_build_dir() {
  local recipe_basename="$1"
  local basename_no_ext="${recipe_basename%.*}"

  find "${RBRR_HISTORY_DIR}" -name "${basename_no_ext}*" -type d -print | sort -r | head -n1
}

# Prompt for confirmation
# Usage: zrbg_confirm_action <prompt_message>
# Returns 0 if confirmed, 1 if not
zrbg_confirm_action() {
  local prompt="$1"

  echo -e "${ZBCU_YELLOW}${prompt}${ZBCU_RESET}"
  read -p "Type YES: " confirm
  test "$confirm" = "YES"
}

# Login to container registry
zrbg_registry_login() {
  bcu_step "Log in to container registry"

  ${RBG_RUNTIME} ${RBG_RUNTIME_ARG} login "${ZRBG_GIT_REGISTRY}" -u "${RBRG_USERNAME}" -p "${RBRG_PAT}"
}

# Execute GitHub Actions workflow and wait for completion
zrbg_execute_workflow() {
  local event_type="${1}"
  local payload_json="${2}"
  local success_message="${3:-Workflow completed}"
  local no_commits_msg="${4:-No new commits after many attempts}"

  bcu_info "Trigger workflow..."
  local dispatch_data='{"event_type": "'${event_type}'", "client_payload": '${payload_json}'}'
  zrbg_curl_post "${ZRBG_DISPATCH_URL}" "${dispatch_data}"

  bcu_info "Polling for completion..."
  sleep 5

  bcu_info "Retrieve workflow run ID..."
  local runs_url="${ZRBG_RUNS_URL_BASE}?event=repository_dispatch&branch=main&per_page=1"
  zrbg_curl_get "${runs_url}" | jq -r '.workflow_runs[0].id' > "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}"
  test -s "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}" || bcu_die "Failed to get workflow run ID"

  local run_id
  run_id=$(<"${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}")

  bcu_info "Workflow online at:"
  echo -e "${ZBCU_YELLOW}   ${ZRBG_GITHUB_ACTIONS_URL}${run_id}${ZBCU_RESET}"

  bcu_info "Polling to completion..."
  local status=""
  local conclusion=""

  while true; do
    local run_url="${ZRBG_RUNS_URL_BASE}/${run_id}"
    local response
    response=$(zrbg_curl_get "${run_url}")

    status=$(echo "${response}" | jq -r '.status')
    conclusion=$(echo "${response}" | jq -r '.conclusion')

    echo "  Status: ${status}    Conclusion: ${conclusion}"

    if [ "${status}" = "completed" ]; then
      break
    fi

    sleep 3
  done

  test "${conclusion}" = "success" || bcu_die "Workflow fail: ${conclusion}"

  bcu_info "${success_message}"
  bcu_info "Git pull with retry..."

  local retry_wait=5
  local max_attempts=30
  local i=0
  local found=0

  while [ ${i} -lt ${max_attempts} ]; do
    echo "  Attempt $((i + 1)): Checking for remote changes..."
    git fetch --quiet

    local count
    count=$(git rev-list --count HEAD..origin/main 2>/dev/null || echo 0)

    if [ "${count}" -gt 0 ]; then
      echo "  Found ${count} new commits, pulling..."
      git pull
      echo "  Pull successful"
      found=1
      break
    fi

    echo "  No new commits yet, waiting ${retry_wait} seconds (attempt $((i + 1)) of ${max_attempts})"
    sleep ${retry_wait}
    i=$((i + 1))
  done

  if [ ${found} -ne 1 ]; then
    echo "  ${no_commits_msg}"
    bcu_die "Expected git commits from workflow not found"
  fi

  bcu_info "Pull logs..."
  zrbg_curl_get "${ZRBG_RUNS_URL_BASE}/${run_id}/logs" > "${ZRBG_WORKFLOW_LOGS}"

  bcu_info "Everything went right, delete the run cache..."
  rm -f "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}"
}

# Helper function to process a single manifest
zrbg_process_single_manifest() {
  local tag="$1"
  local manifest_file="$2"
  local platform="$3"  # Empty for single-platform

  # Diagnostic: Check file exists
  test -f "${manifest_file}" || bcu_die "Manifest file does not exist: ${manifest_file}"

  # Diagnostic: Validate JSON
  jq . "${manifest_file}" >/dev/null 2>&1 || bcu_die "Invalid JSON in manifest file: ${manifest_file}"

  local config_digest
  config_digest=$(jq -r '.config.digest' "${manifest_file}")

  test -n "${config_digest}" || bcu_die "Missing config.digest ${config_digest}"

  test "${config_digest}" != "null" || {
    bcu_warn "    null config.digest in manifest: ${manifest_file}"
    bcu_warn "    Content:"
    cat "${manifest_file}" >&2
    bcu_warn "SKIPPING FOR NOW UNTIL WE FIGURE OUT WHY THIS CORRUPTION HAPPENS."
    return 0
  }

  local config_out="${manifest_file%.json}_config.json"
  local config_err="${manifest_file%.json}_config.err"

  curl -sL -H "Authorization: Bearer ${ZRBG_AUTH_TOKEN}" "${ZRBG_GHCR_V2_API}/blobs/${config_digest}" \
        >"${config_out}" 2>"${config_err}" && \
    jq . "${config_out}" >/dev/null || {
      bcu_warn "    Failed to fetch or parse config blob"
      bcu_warn "    Digest: ${config_digest}"
      bcu_warn "    STDERR: $(<"${config_err}")"
      bcu_warn "    STDOUT: $(<"${config_out}")"
      bcu_die "Failed to retrieve config blob from GHCR"
    }

  test -n "${config_digest}" && test "${config_digest}" != "null" || {
    bcu_warn "    Failed to fetch or parse config blob"
    bcu_warn "    Digest: ${config_digest}"
    bcu_die "Config blob validation failed"
  }

  local temp_detail="${RBG_TEMP_DIR}/temp_detail.json"
  local manifest_json config_json
  manifest_json="$(<"${manifest_file}")"
  config_json=$(jq '. + {
    created: (.created // "1970-01-01T00:00:00Z"),
    architecture: (.architecture // "unknown"),
    os: (.os // "unknown")
  }' "${config_out}")

  echo "${manifest_json}" | jq -e '.layers and (.layers | type == "array")' >/dev/null \
    || bcu_die "Missing or invalid .layers array in manifest: ${manifest_file}"

  echo "${config_json}" | jq -e '.created and .architecture and .os' >/dev/null || {
    bcu_warn "Missing required fields in config blob"
    bcu_warn "    BLOB: $(<"${config_out}")"
    bcu_die  "Config blob missing required fields"
  }

  echo "${config_json}" | jq -e '(.created // empty) and (.architecture // empty) and (.os // empty)' >/dev/null \
    || bcu_die "Null or missing fields in config blob: ${config_out}"

  if [ -n "${platform}" ]; then
    bcu_info "tag ${tag}: Multi-platform entry..."
    jq -n \
      --arg tag          "${tag}"           \
      --arg platform     "${platform}"      \
      --arg digest       "${config_digest}" \
      --argjson manifest "${manifest_json}" \
      --argjson config   "${config_json}" '
      {
        tag: $tag,
        platform: $platform,
        digest: $digest,
        layers: $manifest.layers,
        config: {
          created: $config.created,
          architecture: $config.architecture,
          os: $config.os
        }
      }' > "${temp_detail}"
  else
    bcu_info "tag ${tag}: Single platform entry (no platform field)..."
    jq -n \
      --arg     tag      "${tag}"           \
      --arg     digest   "${config_digest}" \
      --argjson manifest "${manifest_json}" \
      --argjson config   "${config_json}" '
      {
        tag: $tag,
        digest: $digest,
        layers: $manifest.layers,
        config: {
          created: $config.created,
          architecture: $config.architecture,
          os: $config.os
        }
      }' > "${temp_detail}"
  fi

  bcu_info "tag ${tag}: Append to image details file..."
  jq -e . "${temp_detail}"            >/dev/null || bcu_die "Invalid temp_detail.json: ${temp_detail}"
  jq -e . "${ZRBG_IMAGE_DETAIL_FILE}" >/dev/null || bcu_die "Invalid IMAGE_DETAIL_FILE: ${ZRBG_IMAGE_DETAIL_FILE}"

  jq -s '
    if (.[0] | type) == "array" and (.[1] | type) == "object" then
      .[0] + [.[1]]
    else
      error("Invalid JSON types for merge")
    end
  '  "${ZRBG_IMAGE_DETAIL_FILE}" "${temp_detail}" \
   > "${ZRBG_IMAGE_DETAIL_FILE}.tmp" || bcu_die "Failed to merge image detail JSON"
  mv "${ZRBG_IMAGE_DETAIL_FILE}.tmp" "${ZRBG_IMAGE_DETAIL_FILE}"
}

######################################################################
# External Functions (rbg_*)
# These are functions used from outside this module
# Naming convention: rbg_<action>

rbg_build() {
  # Name parameters
  local recipe_file="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Build image from recipe"
  bcu_doc_param "recipe_file" "Path to recipe file containing build instructions"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$recipe_file" || bcu_usage_die
  test -f "$recipe_file" || bcu_die "Recipe file not found: $recipe_file"

  # Perform command
  local recipe_basename=$(basename "$recipe_file")
  echo "$recipe_basename" | grep -q '[A-Z]' && \
      bcu_die "Basename of '$recipe_file' contains uppercase letters so cannot use in image name"

  bcu_step "Trigger image build from $recipe_file"

  zrbg_check_git_status

  bcu_step "Triggering GitHub Actions workflow for image build"
  zrbg_execute_workflow "build_images"                         \
                        '{"dockerfile": "'$recipe_file'"}'     \
                        "Git Pull for artifacts with retry..."

  bcu_info "Verifying build output..."
  local build_dir=$(zrbg_get_latest_build_dir "$recipe_basename")
  test -n "$build_dir"                       || bcu_die "Missing build directory"
  test -d "$build_dir"                       || bcu_die "Invalid build directory"
  test -f "$build_dir/recipe.txt"            || bcu_die "recipe.txt not found"
  cmp "$recipe_file" "$build_dir/recipe.txt" || bcu_die "recipe mismatch"

  bcu_info "Extracting FQIN..."
  local fqin_file="$build_dir/docker_inspect_RepoTags_0.txt"
  test -f "${fqin_file}" || bcu_die "Could not find FQIN in build output"

  local fqin_contents
  fqin_contents=$(<"${fqin_file}")

  bcu_info "Built image FQIN: ${fqin_contents}"

  if [ -n                  "${RBG_ARG_FQIN_OUTPUT:-}" ]; then
    cp "${fqin_file}"      "${RBG_ARG_FQIN_OUTPUT}"
    bcu_info "Wrote FQIN to ${RBG_ARG_FQIN_OUTPUT}"
  fi

  bcu_info "Verifying image availability in registry..."
  local tag="${fqin_contents#*:}"
  echo "Waiting for tag: ${tag} to become available..."
  for i in 1 2 3 4 5; do
    zrbg_curl_get "${ZRBG_PACKAGES_URL}?per_page=100" | \
      jq -e '.[] | select(.metadata.container.tags[] | contains("'"$tag"'"))' > /dev/null && break

    echo "  Image not yet available, attempt $i of 5"
    test $i -ne 5 || bcu_die "Image '${tag}' not available in registry after 5 attempts"
    sleep 5
  done

  bcu_success "No errors."
}

rbg_list() {
  # Handle documentation mode
  bcu_doc_brief "List registry images"
  bcu_doc_shown || return 0

  # Perform command
  zrbg_collect_image_records

  bcu_step "List Current Registry Images"
  echo "Package: ${RBRR_REGISTRY_NAME}"
  echo -e "${ZBCU_YELLOW}    ${ZRBG_GITHUB_PACKAGES_URL}${ZBCU_RESET}"
  echo "Versions:"

  printf "%-13s %-70s\n" "Version ID" "Fully Qualified Image Name"

  jq -r '.[] | [.version_id, .fqin] | @tsv' "${ZRBG_IMAGE_RECORDS_FILE}" | \
    sort -k2 -r | while IFS=$'\t' read -r id fqin; do
    printf "%-13s %s\n" "$id" "${fqin}"
  done

  echo "${ZBCU_RESET}"

  local total=$(jq '. | length' "${ZRBG_IMAGE_RECORDS_FILE}")
  bcu_info "Total image versions: ${total}"

  bcu_success "No errors."
}

rbg_delete() {
  # Name parameters
  local fqin="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Delete image from registry and clean orphans"
  bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$fqin" || bcu_usage_die
  bvu_val_fqin "fqin" "$fqin" 1 512

  # Perform command
  bcu_step "Delete image from GitHub Container Registry"
  zrbg_check_git_status

  # Confirm deletion unless skipped
  if [ "${RBG_ARG_SKIP_DELETE_CONFIRMATION:-}" != "SKIP" ]; then
    zrbg_confirm_action "Confirm delete image ${fqin}?" || bcu_die "WONT DELETE"
  fi

  bcu_step "Triggering GitHub Actions workflow for image deletion"
  zrbg_execute_workflow "delete_image"                         \
                        '{"fqin": "'$fqin'"}'                  \
                        "Git Pull for deletion history..."     \
                        "No deletion history recorded"

  bcu_info "Verifying deletion..."
  local tag=$(echo "$fqin" | sed 's/.*://')

  echo "  Checking that tag '${tag}' is gone..."
  if zrbg_curl_get "${ZRBG_PACKAGES_URL}?per_page=100" | \
    jq -e '.[] | select(.metadata.container.tags[] | contains("'"$tag"'"))' > /dev/null 2>&1; then
    bcu_die "Tag '${tag}' still exists in registry after deletion"
  fi

  echo "  Confirmed: Tag '${tag}' has been deleted"

  bcu_success "No errors."
}

rbg_retrieve() {
  # Name parameters
  local fqin="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Pull image from registry"
  bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$fqin" || bcu_usage_die
  bvu_val_fqin "fqin" "$fqin" 1 512

  # Perform command
  bcu_step "Pull image from GitHub Container Registry"
  zrbg_registry_login
  bcu_info "Fetch image..."
  ${RBG_RUNTIME} ${RBG_RUNTIME_ARG} pull "$fqin"
  bcu_success "No errors."
}

# Gather image info from GHCR tags only (Bash 3.2 compliant)
rbg_image_info() {
  # Name parameters
  local filter="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Extracts per-image and per-layer info from GHCR tags using GitHub API"
  bcu_doc_lines \
    "Creates image detail entries for each tag/platform combination, extracts creation date," \
    "layers, and layer sizes. Handles both single and multi-platform images."
  bcu_doc_oparm "filter" "only process tags containing this string, if provided"
  bcu_doc_shown || return 0

  zrbg_collect_image_records

  echo "[]" > "${ZRBG_IMAGE_DETAIL_FILE}"

  bcu_step "Processing each tag for image details with filter:${filter}"
  local tag

  for tag in $(jq -r '.[].tag' "${ZRBG_IMAGE_RECORDS_FILE}" | sort -u); do

    if [ -n "${filter}" ] && [[ "${tag}" != *"${filter}"* ]]; then
      bcu_info "Skipping tag: ${tag}"
      continue
    fi

    bcu_info "Processing tag: ${tag}"
    local safe_tag="${tag//\//_}"
    local manifest_out="${RBG_TEMP_DIR}/manifest__${safe_tag}.json"
    local manifest_err="${RBG_TEMP_DIR}/manifest__${safe_tag}.err"

    bcu_step "Request both single and multi-platform manifest types for -> ${safe_tag}"

    curl -sL -H "Authorization: Bearer ${ZRBG_AUTH_TOKEN}" \
         -H "Accept: ${ZRBG_ACCEPT_MANIFEST_MTYPES}" \
         "${ZRBG_GHCR_V2_API}/manifests/${tag}"      \
          >"${manifest_out}" 2>"${manifest_err}" && \
      jq . "${manifest_out}" >/dev/null          || {
        bcu_warn "  Failed to fetch or parse manifest for ${tag}"
        bcu_warn "  STDERR: $(<"${manifest_err}")"
        bcu_warn "  STDOUT: $(<"${manifest_out}")"
        bcu_die  "  This image appears corrupted - RECOMMENDED ACTION: Delete this tag"
        continue
      }

    local media_type
    media_type=$(jq -r '.mediaType // .schemaVersion' "${manifest_out}")

    if [[ "${media_type}" == "${ZRBG_MTYPE_DLIST}" ]] || \
       [[ "${media_type}" == "${ZRBG_MTYPE_OCI}"   ]]; then
      bcu_info "  Multi-platform image detected"

      local platform_idx=1
      local manifests
      manifests=$(jq -c '.manifests[]' "${manifest_out}")

      while IFS= read -r platform_manifest; do
        local platform_digest platform_info
        platform_digest=$(echo "${platform_manifest}" | jq -r '.digest')
        platform_info=$(echo   "${platform_manifest}" | jq -r '"\(.platform.os)/\(.platform.architecture)"')

        bcu_info "    Processing platform: ${platform_info}"

        local platform_out="${RBG_TEMP_DIR}/manifest__${safe_tag}__${platform_idx}.json"
        local platform_err="${RBG_TEMP_DIR}/manifest__${safe_tag}__${platform_idx}.err"

        curl -sL -H "Authorization: Bearer ${ZRBG_AUTH_TOKEN}" \
             -H "Accept: ${ZRBG_ACCEPT_MANIFEST_MTYPES}"        \
             "${ZRBG_GHCR_V2_API}/manifests/${platform_digest}" \
              >"${platform_out}" 2>"${platform_err}"          && \
          jq . "${platform_out}" >/dev/null                   || {
            bcu_warn "    Failed to fetch platform manifest"
            bcu_warn "    Platform: ${platform_info}"
            bcu_warn "    Digest: ${platform_digest}"
            bcu_warn "    STDERR: $(<"${platform_err}")"
            bcu_warn "    STDOUT: $(<"${platform_out}")"

            ((platform_idx++))
            continue
          }

        zrbg_process_single_manifest "${tag}" "${platform_out}" "${platform_info}"

        ((platform_idx++))
      done <<< "$manifests"

    elif [[ "${media_type}" == "${ZRBG_MTYPE_DV2}" ]] || \
         [[ "${media_type}" == "${ZRBG_MTYPE_OCM}" ]] || \
         [[ "${media_type}" == "${ZRBG_SCHEMA_V2}" ]]; then

      bcu_info "  Single platform image"
      zrbg_process_single_manifest "${tag}" "${manifest_out}" ""
    else
      bcu_warn "  Unknown manifest type: ${media_type}, skipping"
    fi
  done

  bcu_step "Comprehensive image info next ${ZRBG_IMAGE_DETAIL_FILE}"

  jq '
    .[]
    | {tag} as $t
    | .layers[]
    | {digest, size, tag: $t.tag}
  ' "${ZRBG_IMAGE_DETAIL_FILE}" |
  jq -s '
    group_by([.digest, .tag])
    | map({
        digest: .[0].digest,
        size: .[0].size,
        tag: .[0].tag,
        count: length
      })
    | group_by(.digest)
    | map({
        digest: .[0].digest,
        size: .[0].size,
        tag_count: length,
        total_usage: (map(.count) | add),
        tag_details: map({tag: .tag, count: .count})
      })
    | sort_by(-.size)
  ' > "${ZRBG_IMAGE_STATS_FILE}" || bcu_die "Failed to generate ${ZRBG_IMAGE_STATS_FILE}"

  bcu_step "Listing layers per tag..."
    jq -r '
      .[] |
      "\nTag: \(.tag)" +
      if .platform then " (\(.platform))" else "" end +
      "\nCreated: \(.config.created)" +
      "\nTotal size: \((.layers | map(.size) | add) // 0) bytes" +
      "\nLayers:" +
      (.layers | to_entries | map(
        "\n  [\(.key + 1)] \(.value.digest[0:19])... \(.value.size) bytes"
      ) | join(""))
    ' "${ZRBG_IMAGE_DETAIL_FILE}"

  bcu_step "Listing shared layers and the tags that use them..."
  jq -r '
    .[] | select(.tag_count > 1 or .total_usage > 1) |
    "Layer: \(.digest[0:19]) (used by \(.tag_count) tag(s), \(.size) bytes)\n" +
    (.tag_details | map("  - \(.tag)" + if .count > 1 then " (\(.count) times)" else "" end) | join("\n"))
  ' "${ZRBG_IMAGE_STATS_FILE}"

  bcu_step "Rendering layer usage summary..."
  total_bytes=0
  total_layers=0

  printf "%-22s %12s %8s %8s\n" "Layer Digest" "Bytes" "Tags" "Uses"
  printf "%-22s %12s %8s %8s\n" "------------" "-----" "----" "----"

  while IFS=$'\t' read -r digest size tag_count total_usage; do
    short_digest="${digest:0:19}"  # Includes 'sha256:' + 12 chars
    printf "%-22s %12d %8d %8d\n" "$short_digest" "$size" "$tag_count" "$total_usage"

    total_bytes=$((total_bytes + size))
    total_layers=$((total_layers + 1))
  done < <(jq -r '.[] | [.digest, .size, .tag_count, .total_usage] | @tsv' "${ZRBG_IMAGE_STATS_FILE}")

  printf "\nTotal unique layers: %d\n" "${total_layers}"
  printf "Total deduplicated size: %d MB\n" "$((total_bytes / 1024 / 1024))"

  bcu_success "No errors."
}

bcu_execute rbg_ "Recipe Bottle GitHub - Image Registry Management" zrbg_environment "$@"

# eof
