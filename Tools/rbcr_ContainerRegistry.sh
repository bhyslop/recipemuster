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
# Recipe Bottle Container Registry - Container Registry Management

set -euo pipefail

ZRBCR_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBCR_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBCR_SCRIPT_DIR}/bvu_BashValidationUtility.sh"
source "${ZRBCR_SCRIPT_DIR}/rbcr_implementation.sh"

######################################################################
# Internal Functions (zrbcr_*)

zrbcr_environment() {
  # Handle documentation mode
  bcu_doc_env "RBCR_TEMP_DIR    " "Empty temporary directory"
  bcu_doc_env "RBCR_NOW_STAMP   " "Timestamp for per run branding"
  bcu_doc_env "RBCR_RBRR_FILE   " "File containing the RBRR constants"
  bcu_doc_env "RBCR_RUNTIME     " "Container Runtime to use"
  bcu_doc_env "RBCR_RUNTIME_ARG " "Container Runtime argument, optional"

  bcu_env_done || return 0

  # Validate environment
  bvu_dir_exists  "${RBCR_TEMP_DIR}"
  bvu_dir_empty   "${RBCR_TEMP_DIR}"
  bvu_env_string     RBCR_NOW_STAMP   1 128   # weak validation but infrastructure managed
  bvu_file_exists "${RBCR_RBRR_FILE}"

  source              "${RBCR_RBRR_FILE}"
  source "${ZRBCR_SCRIPT_DIR}/rbrr.validator.sh"

  # Export variables for implementation layer
  export RBC_TEMP_DIR="${RBCR_TEMP_DIR}"
  export RBC_RUNTIME="${RBCR_RUNTIME}"
  export RBC_RUNTIME_ARG="${RBCR_RUNTIME_ARG}"

  # Source GitHub PAT credentials if using GHCR
  if [ "${RBRR_REGISTRY}" = "ghcr" ]; then
    bvu_file_exists "${RBRR_GITHUB_PAT_ENV}"
    source          "${RBRR_GITHUB_PAT_ENV}"
    export RBRG_PAT="${RBRG_PAT}"
    export RBRG_USERNAME="${RBRG_USERNAME}"
  fi

  # Module Variables (ZRBCR_*)
  ZRBCR_IMAGE_RECORDS_FILE="${RBCR_TEMP_DIR}/IMAGE_RECORDS.json"
  ZRBCR_IMAGE_DETAIL_FILE="${RBCR_TEMP_DIR}/IMAGE_DETAILS.json"
  ZRBCR_IMAGE_STATS_FILE="${RBCR_TEMP_DIR}/IMAGE_STATS.json"
}

######################################################################
# External Functions (rbcr_*)

rbcr_build() {
  local recipe_file="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Build image from recipe (requires GitHub Actions dispatch)"
  bcu_doc_param "recipe_file" "Path to recipe file containing build instructions"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "${recipe_file}" || bcu_usage_die

  bcu_die "Build functionality moved to rbad_GithubActionDispatch.sh"
}

rbcr_list() {
  # Handle documentation mode
  bcu_doc_brief "List registry images"
  bcu_doc_shown || return 0

  bcu_info "Set up container registry"
  rbcri_start

  bcu_info "Use registry implementation to get tags"
  rbcri_tags "${ZRBCR_IMAGE_RECORDS_FILE}"

  bcu_step "List Current Registry Images"
  echo "Package: ${RBRR_REGISTRY_NAME}"

  echo "Versions:"

  # Check if ghcr_version_id exists in the schema
  if [ "${RBRR_REGISTRY}" = "ghcr" ] && jq -e '.[0] | has("ghcr_version_id")' "${ZRBCR_IMAGE_RECORDS_FILE}" >/dev/null 2>&1; then
    printf "%-13s %-70s\n" "Version ID" "Fully Qualified Image Name"
    jq -r '.[] | [.ghcr_version_id, .fqin] | @tsv' "${ZRBCR_IMAGE_RECORDS_FILE}" | \
      sort -k2 -r | while IFS=$'\t' read -r id fqin; do
      printf "%-13s %s\n" "$id" "${fqin}"
    done
  else
    printf "%-70s\n" "Fully Qualified Image Name"
    jq -r '.[].fqin' "${ZRBCR_IMAGE_RECORDS_FILE}" | sort -r | while read -r fqin; do
      printf "%s\n" "${fqin}"
    done
  fi

  echo "${ZBCU_RESET}"

  local total=$(jq '. | length' "${ZRBCR_IMAGE_RECORDS_FILE}")
  bcu_info "Total image versions: ${total}"

  bcu_success "No errors."
}

rbcr_delete() {
  local fqin="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Delete image from registry and clean orphans"
  bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$fqin" || bcu_usage_die
  bvu_val_fqin "fqin" "$fqin" 1 512

  # Extract tag from fqin
  local tag=$(echo "$fqin" | sed 's/.*://')

  # Confirm deletion unless skipped
  if [ "${RBCR_ARG_SKIP_DELETE_CONFIRMATION:-}" != "SKIP" ]; then
    bcu_require "Confirm delete image ${fqin}?" "YES" || bcu_die "WONT DELETE"
  fi

  bcu_step "Deleting image from ${RBRR_REGISTRY} registry"

  # Use registry implementation
  rbcri_delete "${tag}" || bcu_die "Failed to delete from ${RBRR_REGISTRY}"

  bcu_info "Verifying deletion..."
  echo "  Checking that tag '${tag}' is gone..."

  if rbcri_exists "${tag}"; then
    bcu_die "Tag '${tag}' still exists in registry after deletion"
  fi

  echo "  Confirmed: Tag '${tag}' has been deleted"

  bcu_success "No errors."
}

rbcr_retrieve() {
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

  bcu_step "Login using registry implementation..."
  rbcri_start

  local tag="${fqin#*:}"
  rbcri_pull "${tag}"

  bcu_success "No errors."
}

rbcr_layers() {
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

  bcu_step "Login using registry implementation..."
  rbcri_start

  echo "[]" > "${output_IMAGE_DETAILS_json}"

  local tag
  for tag in $(jq -r '.[].tag' "${input_IMAGE_RECORDS_json}" | sort -u); do

    bcu_step "Processing tag: ${tag}"

    local safe_tag="${tag//\//_}"
    local manifest_out="${RBCR_TEMP_DIR}/manifest_${safe_tag}.json"
    local temp_detail="${RBCR_TEMP_DIR}/temp_detail.json"

    rbcri_fetch_manifest "${tag}" "${manifest_out}" || continue

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

        local platform_out="${RBCR_TEMP_DIR}/platform_${safe_tag}.json"

        rbcri_fetch_manifest "${platform_digest}" "${platform_out}" || continue

        if rbcri_process_single_manifest "${tag}" "${platform_out}" "${os_arch}" "${temp_detail}"; then
          jq -s '.[0] + [.[1]]' "${output_IMAGE_DETAILS_json}" "${temp_detail}" \
                              > "${output_IMAGE_DETAILS_json}.tmp"
          mv                    "${output_IMAGE_DETAILS_json}.tmp" \
                                "${output_IMAGE_DETAILS_json}"
        fi
      done <<< "$manifests"

    else
      # Single platform
      if rbcri_process_single_manifest "${tag}" "${manifest_out}" "" "${temp_detail}"; then
        jq -s '.[0] + [.[1]]' "${output_IMAGE_DETAILS_json}" "${temp_detail}" \
                            > "${output_IMAGE_DETAILS_json}.tmp"
        mv                    "${output_IMAGE_DETAILS_json}.tmp" \
                              "${output_IMAGE_DETAILS_json}"
      fi
    fi
  done

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

rbcr_image_info() {
  local filter="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Extracts per-image and per-layer info from registry tags"
  bcu_doc_lines \
    "Creates image detail entries for each tag/platform combination, extracts creation date," \
    "layers, and layer sizes. Handles both single and multi-platform images."
  bcu_doc_oparm "filter" "only process tags containing this string, if provided"
  bcu_doc_shown || return 0

  bcu_step "Login using registry implementation..."
  rbcri_start

  bcu_step "Get tags from registry..."
  rbcri_tags "${ZRBCR_IMAGE_RECORDS_FILE}"

  if [ -n "${filter}" ]; then
    jq --arg filter "${filter}" '[.[] | select(.tag | contains($filter))]' \
      "${ZRBCR_IMAGE_RECORDS_FILE}" > "${ZRBCR_IMAGE_RECORDS_FILE}.filtered"
    mv "${ZRBCR_IMAGE_RECORDS_FILE}.filtered" "${ZRBCR_IMAGE_RECORDS_FILE}"
  fi

  rbcr_layers "${ZRBCR_IMAGE_RECORDS_FILE}" "${ZRBCR_IMAGE_DETAIL_FILE}" "${ZRBCR_IMAGE_STATS_FILE}"

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
    ' "${ZRBCR_IMAGE_DETAIL_FILE}"

  bcu_step "Listing shared layers and the tags that use them..."
  jq -r '
    .[] | select(.tag_count > 1 or .total_usage > 1) |
    "Layer: \(.digest[0:19]) (used by \(.tag_count) tag(s), \(.size) bytes)\n" +
    (.tag_details | map("  - \(.tag)" + if .count > 1 then " (\(.count) times)" else "" end) | join("\n"))
  ' "${ZRBCR_IMAGE_STATS_FILE}"

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
  done < <(jq -r '.[] | [.digest, .size, .tag_count, .total_usage] | @tsv' "${ZRBCR_IMAGE_STATS_FILE}")

  printf "\nTotal unique layers: %d\n" "${total_layers}"
  printf "Total deduplicated size: %d MB\n" "$((total_bytes / 1024 / 1024))"

  bcu_success "No errors."
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  # This allows sourcing this module in another that is being executed
  bcu_execute rbcr_ "Recipe Bottle Container Registry - Container Registry Management" zrbcr_environment "$@"
fi

# eof
