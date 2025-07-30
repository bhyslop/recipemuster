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
# Recipe Bottle Container - Container Registry Management

set -euo pipefail

ZRBC_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBC_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBC_SCRIPT_DIR}/bvu_BashValidationUtility.sh"

######################################################################
# Internal Functions (zrbc_*)

zrbc_environment() {
  # Handle documentation mode
  bcu_doc_env "RBC_TEMP_DIR    " "Empty temporary directory"
  bcu_doc_env "RBC_NOW_STAMP   " "Timestamp for per run branding"
  bcu_doc_env "RBC_RBRR_FILE   " "File containing the RBRR constants"
  bcu_doc_env "RBC_RUNTIME     " "Container runtime to use"
  bcu_doc_env "RBC_RUNTIME_ARG " "Argument to container runtime"

  bcu_env_done || return 0

  # Validate environment
  bvu_dir_exists  "${RBC_TEMP_DIR}"
  bvu_dir_empty   "${RBC_TEMP_DIR}"
  bvu_env_string     RBC_NOW_STAMP   1 128   # weak validation but infrastructure managed
  bvu_file_exists "${RBC_RBRR_FILE}"

  source              "${RBC_RBRR_FILE}"
  source "${ZRBC_SCRIPT_DIR}/rbrr.validator.sh"

  # Validate registry selection
  test -n "${RBRR_REGISTRY:-}" || bcu_die "RBRR_REGISTRY not set"
  case "${RBRR_REGISTRY}" in
    ghcr|ecr|acr|quay) ;;
    *) bcu_die "Unknown registry: ${RBRR_REGISTRY}" ;;
  esac

  # Source registry-specific driver
  case "${RBRR_REGISTRY}" in
    ghcr)
      source "${ZRBC_SCRIPT_DIR}/rbcg_GHCR.sh"
      bvu_file_exists "${RBRR_GITHUB_PAT_ENV}"
      source          "${RBRR_GITHUB_PAT_ENV}"
      test -n "${RBRG_PAT:-}"      || bcu_die "RBRG_PAT missing from ${RBRR_GITHUB_PAT_ENV}"
      test -n "${RBRG_USERNAME:-}" || bcu_die "RBRG_USERNAME missing from ${RBRR_GITHUB_PAT_ENV}"
      ;;
    ecr)
      source "${ZRBC_SCRIPT_DIR}/rbce_ECR.sh"
      # ECR-specific validation would go here
      ;;
    acr)
      source "${ZRBC_SCRIPT_DIR}/rbca_ACR.sh"
      # ACR-specific validation would go here
      ;;
    quay)
      source "${ZRBC_SCRIPT_DIR}/rbcq_Quay.sh"
      # Quay-specific validation would go here
      ;;
  esac

  # Module Variables (ZRBC_*)

  # Base URLs (GHCR-specific, but kept for GitHub Actions workflows)
  ZRBC_GITAPI_URL="https://api.github.com"
  ZRBC_REPO_PREFIX="${ZRBC_GITAPI_URL}/repos"

  # Derived URLs (GHCR-specific, but kept for GitHub Actions workflows)
  ZRBC_DISPATCH_URL="${ZRBC_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/dispatches"
  ZRBC_RUNS_URL_BASE="${ZRBC_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs"
  ZRBC_GITHUB_ACTIONS_URL="https://github.com/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs/"
  ZRBC_GITHUB_PACKAGES_URL="https://github.com/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/pkgs/container/${RBRR_REGISTRY_NAME}"

  # OCI Registry v2 API Standard Media Types
  RBC_MTYPE_DLIST="application/vnd.docker.distribution.manifest.list.v2+json"
  RBC_MTYPE_OCI="application/vnd.oci.image.index.v1+json"
  RBC_MTYPE_DV2="application/vnd.docker.distribution.manifest.v2+json"
  RBC_MTYPE_OCM="application/vnd.oci.image.manifest.v1+json"

  # Schema Documentation for IMAGE_RECORDS.json
  # ------------------------------------------
  # A JSON array of image tag metadata objects. This is the raw tag listing
  # from the container registry, used as input to downstream inspection.
  #
  # Each object has the following structure:
  # {
  #   "tag": "<tag>",                        # The tag string (e.g., "v5.5-20250725-abc_x86_64")
  #   "fqin": "<fully-qualified-image-name>", # Full image reference (e.g., "ghcr.io/owner/repo:tag")
  #   "ghcr_version_id": "<version-id>"      # OPTIONAL: GHCR-specific version identifier
  # }
  #
  # Notes:
  # - ghcr_version_id is only present when RBRR_REGISTRY="ghcr"
  # - This file does not include layer or config information
  # - Downstream code uses this as a seed to resolve manifests and blobs

  ZRBC_IMAGE_RECORDS_FILE="${RBC_TEMP_DIR}/IMAGE_RECORDS.json"
  ZRBC_IMAGE_DETAIL_FILE="${RBC_TEMP_DIR}/IMAGE_DETAILS.json"
  ZRBC_IMAGE_STATS_FILE="${RBC_TEMP_DIR}/IMAGE_STATS.json"

  # Media type for GitHub API
  RBC_MTYPE_GHV3="application/vnd.github.v3+json"

  # Temp files
  ZRBC_CURRENT_WORKFLOW_RUN_CACHE="${RBC_TEMP_DIR}/CURR_WORKFLOW_RUN__${RBC_NOW_STAMP}.txt"
  ZRBC_DELETE_VERSION_ID_CACHE="${RBC_TEMP_DIR}/RBC_VERSION_ID__${RBC_NOW_STAMP}.txt"
  ZRBC_DELETE_RESULT_CACHE="${RBC_TEMP_DIR}/RBC_DELETE__${RBC_NOW_STAMP}.txt"
  ZRBC_WORKFLOW_LOGS="${RBC_TEMP_DIR}/workflow_logs__${RBC_NOW_STAMP}.txt"

  # Container runtime (default to podman)
  ZRBC_RUNTIME="${RBRR_RUNTIME}"
  ZRBC_CONNECTION="${RBC_CONNECTION:-}"
}

# Initialize registry session with authentication
zrbc_start() {

  case "${RBRR_REGISTRY}" in
    ghcr) rbcg_start ;;
    ecr)  rbce_start ;;
    acr)  rbca_start ;;
    quay) rbcq_start ;;
    *) bcu_die "Unknown registry: ${RBRR_REGISTRY}" ;;
  esac
}

# Perform authenticated GET request (GitHub Action workflow triggering/ querying)
zrbc_curl_get() {
  local url="$1"

  curl -s -H "Authorization: token ${RBRG_PAT}" \
          -H "Accept: ${RBC_MTYPE_GHV3}"        \
          "$url"
}

# Perform authenticated POST request (GitHub Action workflow triggering/ querying)
zrbc_curl_post() {
  local url="$1"
  local data="$2"

  curl                                             \
       -s                                          \
       -X POST                                     \
       -H "Authorization: token ${RBRG_PAT}"       \
       -H "Accept: ${RBC_MTYPE_GHV3}"              \
       "$url"                                      \
       -d "$data"                                  \
    || bcu_die "Curl failed."
}

# Check git repository status
zrbc_check_git_status() {
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
zrbc_get_latest_build_dir() {
  local recipe_basename="$1"
  local basename_no_ext="${recipe_basename%.*}"

  find "${RBRR_HISTORY_DIR}" -name "${basename_no_ext}*" -type d -print | sort -r | head -n1
}

# Prompt for confirmation
zrbc_confirm_action() {
  local prompt="$1"
  bcu_require "$prompt" "YES"
}

# Process a single manifest
zrbc_process_single_manifest() {
  local tag="$1"
  local manifest_file="$2"
  local platform="$3"
  local temp_detail="$4"

  local config_digest
  config_digest=$(jq -r '.config.digest' "${manifest_file}")

  test -n "${config_digest}" && test "${config_digest}" != "null" || {
    bcu_warn "null config.digest in manifest"
    return 1
  }

  local config_out="${manifest_file%.json}_config.json"

  # Use registry-specific config blob fetch
  case "${RBRR_REGISTRY}" in
    ghcr) rbcg_fetch_config_blob "${config_digest}" "${config_out}" ;;
    ecr)  rbce_fetch_config_blob "${config_digest}" "${config_out}" ;;
    acr)  rbca_fetch_config_blob "${config_digest}" "${config_out}" ;;
    quay) rbcq_fetch_config_blob "${config_digest}" "${config_out}" ;;
    *) bcu_die "Unknown registry: ${RBRR_REGISTRY}" ;;
  esac || {
    bcu_warn "Failed to fetch config blob"
    return 1
  }

  local manifest_json config_json
  manifest_json="$(<"${manifest_file}")"
  config_json=$(jq '. + {
    created: (.created // "1970-01-01T00:00:00Z"),
    architecture: (.architecture // "unknown"),
    os: (.os // "unknown")
  }' "${config_out}")

  if [ -n "${platform}" ]; then
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

  return 0
}

# Execute GitHub Actions workflow and wait for completion
zrbc_execute_workflow() {
  local event_type="${1}"
  local payload_json="${2}"
  local success_message="${3:-Workflow completed}"
  local no_commits_msg="${4:-No new commits after many attempts}"

  bcu_info "Trigger workflow..."
  local dispatch_data='{"event_type": "'${event_type}'", "client_payload": '${payload_json}'}'
  zrbc_curl_post "${ZRBC_DISPATCH_URL}" "${dispatch_data}"

  bcu_info "Polling for completion..."
  sleep 5

  bcu_info "Retrieve workflow run ID..."
  local runs_url="${ZRBC_RUNS_URL_BASE}?event=repository_dispatch&branch=main&per_page=1"
  zrbc_curl_get "${runs_url}" | jq -r '.workflow_runs[0].id' > "${ZRBC_CURRENT_WORKFLOW_RUN_CACHE}"
  test -s "${ZRBC_CURRENT_WORKFLOW_RUN_CACHE}" || bcu_die "Failed to get workflow run ID"

  local run_id
  run_id=$(<"${ZRBC_CURRENT_WORKFLOW_RUN_CACHE}")

  bcu_info "Workflow online at:"
  echo -e "${ZBCU_YELLOW}   ${ZRBC_GITHUB_ACTIONS_URL}${run_id}${ZBCU_RESET}"

  bcu_info "Polling to completion..."
  local status=""
  local conclusion=""

  while true; do
    local run_url="${ZRBC_RUNS_URL_BASE}/${run_id}"
    local response
    response=$(zrbc_curl_get "${run_url}")

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
      echo "  Found new commits, pulling..."
      git pull
      echo "  Pull successful"
      found=1
      break
    fi

    echo "  No new commits yet, waiting ${retry_wait} seconds (attempt $((i + 1)))"
    sleep ${retry_wait}
    i=$((i + 1))
  done

  if [ ${found} -ne 1 ]; then
    echo "  ${no_commits_msg}"
    bcu_die "No commits found"
  fi

  bcu_info "Pull logs..."
  zrbc_curl_get "${ZRBC_RUNS_URL_BASE}/${run_id}/logs" > "${ZRBC_WORKFLOW_LOGS}"

  bcu_info "Everything went right, delete the run cache..."
  rm -f "${ZRBC_CURRENT_WORKFLOW_RUN_CACHE}"
}

######################################################################
# External Functions (rbc_*)

rbc_build() {
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

  zrbc_check_git_status

  bcu_step "Triggering GitHub Actions workflow for image build"
  zrbc_execute_workflow "build_images"                         \
                        '{"dockerfile": "'$recipe_file'"}'     \
                        "Git Pull for artifacts with retry..."

  bcu_info "Verifying build output..."
  local build_dir=$(zrbc_get_latest_build_dir "$recipe_basename")
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

  if [ -n                  "${RBC_ARG_FQIN_OUTPUT:-}" ]; then
    cp "${fqin_file}"      "${RBC_ARG_FQIN_OUTPUT}"
    bcu_info "Wrote FQIN to ${RBC_ARG_FQIN_OUTPUT}"
  fi

  bcu_info "Verifying image availability in registry..."
  local tag="${fqin_contents#*:}"
  echo "Waiting for tag: ${tag} to become available..."
  for i in 1 2 3 4 5; do
    case "${RBRR_REGISTRY}" in
      ghcr) ! rbcg_exists "${tag}" || break ;;
      ecr)  ! rbce_exists "${tag}" || break ;;
      acr)  ! rbca_exists "${tag}" || break ;;
      quay) ! rbcq_exists "${tag}" || break ;;
    esac

    echo "  Image not yet available, attempt $i of 5"
    test $i -ne 5 || bcu_die "Image '${tag}' not available in registry after 5 attempts"
    sleep 5
  done

  bcu_success "No errors."
}

rbc_list() {
  # Handle documentation mode
  bcu_doc_brief "List registry images"
  bcu_doc_shown || return 0

  # Use registry implementation to get tags
  case "${RBRR_REGISTRY}" in
    ghcr) rbcg_tags "${ZRBC_IMAGE_RECORDS_FILE}" ;;
    ecr)  rbce_tags "${ZRBC_IMAGE_RECORDS_FILE}" ;;
    acr)  rbca_tags "${ZRBC_IMAGE_RECORDS_FILE}" ;;
    quay) rbcq_tags "${ZRBC_IMAGE_RECORDS_FILE}" ;;
  esac

  bcu_step "List Current Registry Images"
  echo "Package: ${RBRR_REGISTRY_NAME}"

  # Display URL based on registry type
  case "${RBRR_REGISTRY}" in
    ghcr) echo -e "${ZBCU_YELLOW}    ${ZRBC_GITHUB_PACKAGES_URL}${ZBCU_RESET}" ;;
    ecr)  echo -e "${ZBCU_YELLOW}    ECR Console URL${ZBCU_RESET}" ;;
    acr)  echo -e "${ZBCU_YELLOW}    ACR Console URL${ZBCU_RESET}" ;;
    quay) echo -e "${ZBCU_YELLOW}    Quay Console URL${ZBCU_RESET}" ;;
  esac

  echo "Versions:"

  # Check if ghcr_version_id exists in the schema
  if [ "${RBRR_REGISTRY}" = "ghcr" ] && jq -e '.[0] | has("ghcr_version_id")' "${ZRBC_IMAGE_RECORDS_FILE}" >/dev/null 2>&1; then
    printf "%-13s %-70s\n" "Version ID" "Fully Qualified Image Name"
    jq -r '.[] | [.ghcr_version_id, .fqin] | @tsv' "${ZRBC_IMAGE_RECORDS_FILE}" | \
      sort -k2 -r | while IFS=$'\t' read -r id fqin; do
      printf "%-13s %s\n" "$id" "${fqin}"
    done
  else
    printf "%-70s\n" "Fully Qualified Image Name"
    jq -r '.[].fqin' "${ZRBC_IMAGE_RECORDS_FILE}" | sort -r | while read -r fqin; do
      printf "%s\n" "${fqin}"
    done
  fi

  echo "${ZBCU_RESET}"

  local total=$(jq '. | length' "${ZRBC_IMAGE_RECORDS_FILE}")
  bcu_info "Total image versions: ${total}"

  bcu_success "No errors."
}

rbc_delete() {
  local fqin="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Delete image from registry and clean orphans"
  bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$fqin" || bcu_usage_die
  bvu_val_fqin "fqin" "$fqin" 1 512

  # Perform command
  bcu_step "Delete image from Container Registry"
  zrbc_check_git_status

  # Confirm deletion unless skipped
  if [ "${RBC_ARG_SKIP_DELETE_CONFIRMATION:-}" != "SKIP" ]; then
    zrbc_confirm_action "Confirm delete image ${fqin}?" || bcu_die "WONT DELETE"
  fi

  # Extract tag from fqin
  local tag=$(echo "$fqin" | sed 's/.*://')

  bcu_step "Deleting image from ${RBRR_REGISTRY} registry"
  
  # Use registry-specific delete operation
  case "${RBRR_REGISTRY}" in
    ghcr)  bcu_die "Unsupported since GHCR deletion does not manage layers safely." ;;
    ecr)   rbce_delete "${tag}" || bcu_die "Failed to delete from ECR"              ;;
    acr)   rbca_delete "${tag}" || bcu_die "Failed to delete from ACR"              ;;
    quay)  rbcq_delete "${tag}" || bcu_die "Failed to delete from Quay"             ;;
    *)     bcu_die "Unknown registry: ${RBRR_REGISTRY}"                             ;;
  esac

  bcu_info "Verifying deletion..."
  echo "  Checking that tag '${tag}' is gone..."

  # Use registry-specific existence check
  if case "${RBRR_REGISTRY}" in
       ecr)  rbce_exists "${tag}" ;;
       acr)  rbca_exists "${tag}" ;;
       quay) rbcq_exists "${tag}" ;;
     esac; then
    bcu_die "Tag '${tag}' still exists in registry after deletion"
  fi

  echo "  Confirmed: Tag '${tag}' has been deleted"

  bcu_success "No errors."
}

rbc_retrieve() {
  local fqin="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Pull image from registry"
  bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$fqin" || bcu_usage_die
  bvu_val_fqin "fqin" "$fqin" 1 512

  # Perform command
  bcu_step "Pull image from Container Registry"

  # Login using registry implementation
  zrbc_start "${ZRBC_RUNTIME}" "${ZRBC_CONNECTION}"

  local tag="${fqin#*:}"
  case "${RBRR_REGISTRY}" in
    ghcr) rbcg_pull "${tag}" ;;
    ecr)  rbce_pull "${tag}" ;;
    acr)  rbca_pull "${tag}" ;;
    quay) rbcq_pull "${tag}" ;;
  esac

  bcu_success "No errors."
}

rbc_layers() {
  local input_IMAGE_RECORDS_json="${1:-}"
  local output_IMAGE_DETAILS_json="${2:-}"
  local output_IMAGE_STATS_json="${3:-}"

  # Handle documentation mode
  bcu_doc_brief "Extract layer information from specified tags"
  bcu_doc_param "input_IMAGE_RECORDS_json"   "Path to JSON file containing tags to analyze"
  bcu_doc_param "output_IMAGE_DETAILS_json"  "Path to write layer details"
  bcu_doc_param "output_IMAGE_STATS_json"    "Path to write layer statistics"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$input_IMAGE_RECORDS_json"  || bcu_usage_die
  test -n "$output_IMAGE_DETAILS_json" || bcu_usage_die
  test -n "$output_IMAGE_STATS_json"   || bcu_usage_die

  bcu_step "Analyzing image layers"

  # Initialize registry session
  zrbc_start "${ZRBC_RUNTIME}" "${ZRBC_CONNECTION}"

  # Initialize details file
  echo "[]" > "${output_IMAGE_DETAILS_json}"

  # Process each tag from input file
  local tag
  for tag in $(jq -r '.[].tag' "${input_IMAGE_RECORDS_json}" | sort -u); do

    bcu_info "Processing tag: ${tag}"

    local safe_tag="${tag//\//_}"
    local manifest_out="${RBC_TEMP_DIR}/manifest_${safe_tag}.json"
    local temp_detail="${RBC_TEMP_DIR}/temp_detail.json"

    # Use registry-specific manifest fetch
    case "${RBRR_REGISTRY}" in
      ghcr) rbcg_fetch_manifest "${tag}" "${manifest_out}" ;;
      ecr)  rbce_fetch_manifest "${tag}" "${manifest_out}" ;;
      acr)  rbca_fetch_manifest "${tag}" "${manifest_out}" ;;
      quay) rbcq_fetch_manifest "${tag}" "${manifest_out}" ;;
      *) bcu_die "Unknown registry: ${RBRR_REGISTRY}" ;;
    esac || continue

    # Check media type
    local media_type
    media_type=$(jq -r '.mediaType // .schemaVersion' "${manifest_out}")

    if [[ "${media_type}" == "${RBC_MTYPE_DLIST}" ]] || \
       [[ "${media_type}" == "${RBC_MTYPE_OCI}"   ]]; then
      # Multi-platform
      local manifests
      manifests=$(jq -c '.manifests[]' "${manifest_out}")

      while IFS= read -r platform_manifest; do
        local platform_digest os_arch

        { read -r platform_digest os_arch; } < <(
          jq -e -r '.digest, "\(.platform.os)/\(.platform.architecture)"' <<<"${platform_manifest}") \
            || bcu_die "Invalid platform_manifest JSON"

        local platform_out="${RBC_TEMP_DIR}/platform_${safe_tag}.json"

        # Use registry-specific manifest fetch for platform
        case "${RBRR_REGISTRY}" in
          ghcr) rbcg_fetch_manifest "${platform_digest}" "${platform_out}" ;;
          ecr)  rbce_fetch_manifest "${platform_digest}" "${platform_out}" ;;
          acr)  rbca_fetch_manifest "${platform_digest}" "${platform_out}" ;;
          quay) rbcq_fetch_manifest "${platform_digest}" "${platform_out}" ;;
          *) bcu_die "Unknown registry: ${RBRR_REGISTRY}" ;;
        esac || continue

        if zrbc_process_single_manifest "${tag}" "${platform_out}" "${os_arch}" "${temp_detail}"; then
          jq -s '.[0] + [.[1]]' "${output_IMAGE_DETAILS_json}"                                     "${temp_detail}" \
                              > "${output_IMAGE_DETAILS_json}.tmp"
          mv                    "${output_IMAGE_DETAILS_json}.tmp" \
                                "${output_IMAGE_DETAILS_json}"
        fi
      done <<< "$manifests"

    else
      # Single platform
      if zrbc_process_single_manifest "${tag}" "${manifest_out}" "" "${temp_detail}"; then
        jq -s '.[0] + [.[1]]' "${output_IMAGE_DETAILS_json}"                           "${temp_detail}" \
                            > "${output_IMAGE_DETAILS_json}.tmp"
        mv                    "${output_IMAGE_DETAILS_json}.tmp" \
                              "${output_IMAGE_DETAILS_json}"
      fi
    fi
  done

  # Generate stats
  jq '
    .[]
    | {tag} as $t
    | .layers[]
    | {digest, size, tag: $t.tag}
  ' "${output_IMAGE_DETAILS_json}" |
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
  ' > "${output_IMAGE_STATS_json}"

  bcu_success "Layer analysis complete"
}

# Gather image info from registry tags only (Bash 3.2 compliant)
rbc_image_info() {
  local filter="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Extracts per-image and per-layer info from registry tags"
  bcu_doc_lines \
    "Creates image detail entries for each tag/platform combination, extracts creation date," \
    "layers, and layer sizes. Handles both single and multi-platform images."
  bcu_doc_oparm "filter" "only process tags containing this string, if provided"
  bcu_doc_shown || return 0

  # Get tags from registry
  case "${RBRR_REGISTRY}" in
    ghcr) rbcg_tags "${ZRBC_IMAGE_RECORDS_FILE}" ;;
    ecr)  rbce_tags "${ZRBC_IMAGE_RECORDS_FILE}" ;;
    acr)  rbca_tags "${ZRBC_IMAGE_RECORDS_FILE}" ;;
    quay) rbcq_tags "${ZRBC_IMAGE_RECORDS_FILE}" ;;
  esac

  # Filter if requested
  if [ -n "${filter}" ]; then
    jq --arg filter "${filter}" '[.[] | select(.tag | contains($filter))]' \
      "${ZRBC_IMAGE_RECORDS_FILE}" > "${ZRBC_IMAGE_RECORDS_FILE}.filtered"
    mv "${ZRBC_IMAGE_RECORDS_FILE}.filtered" "${ZRBC_IMAGE_RECORDS_FILE}"
  fi

  # Use generic layer analysis
  rbc_layers "${ZRBC_IMAGE_RECORDS_FILE}" "${ZRBC_IMAGE_DETAIL_FILE}" "${ZRBC_IMAGE_STATS_FILE}"

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
    ' "${ZRBC_IMAGE_DETAIL_FILE}"

  bcu_step "Listing shared layers and the tags that use them..."
  jq -r '
    .[] | select(.tag_count > 1 or .total_usage > 1) |
    "Layer: \(.digest[0:19]) (used by \(.tag_count) tag(s), \(.size) bytes)\n" +
    (.tag_details | map("  - \(.tag)" + if .count > 1 then " (\(.count) times)" else "" end) | join("\n"))
  ' "${ZRBC_IMAGE_STATS_FILE}"

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
  done < <(jq -r '.[] | [.digest, .size, .tag_count, .total_usage] | @tsv' "${ZRBC_IMAGE_STATS_FILE}")

  printf "\nTotal unique layers: %d\n" "${total_layers}"
  printf "Total deduplicated size: %d MB\n" "$((total_bytes / 1024 / 1024))"

  bcu_success "No errors."
}

bcu_execute rbc_ "Recipe Bottle Container - Container Registry Management" zrbc_environment "$@"

# eof

