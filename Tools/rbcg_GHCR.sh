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
# Recipe Bottle Container GitHub - GHCR Registry Implementation

set -euo pipefail

ZRBCG_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBCG_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBCG_SCRIPT_DIR}/bvu_BashValidationUtility.sh"

######################################################################
# Internal Functions (zrbcg_*)


# Perform authenticated GET request to GitHub API
zrbcg_curl_get() {
  local url="$1"
  curl -s -H "Authorization: token ${RBRG_PAT}" \
          -H "Accept: ${RBC_MTYPE_GHV3}"        \
          "$url"
}

######################################################################
# External Functions (rbcg_*)

rbcg_start() {
  # Handle documentation mode
  bcu_doc_brief "Initialize GHCR session with login and token setup"
  bcu_doc_shown || return 0

  # Environment validation
  bvu_dir_exists  "${RBC_TEMP_DIR}"
  bvu_file_exists "${RBC_RBRR_FILE}"

  test -n "${RBC_RUNTIME}"       || bcu_die "RBC_RUNTIME missing"
  test -n "${RBC_RUNTIME_ARG+x}" || bcu_die "RBC_RUNTIME_ARG not set"

  # Source GitHub PAT credentials
  bvu_file_exists "${RBRR_GITHUB_PAT_ENV}"
  source          "${RBRR_GITHUB_PAT_ENV}"

  # Extract and validate PAT credentials
  test -n "${RBRG_PAT:-}"      || bcu_die "RBRG_PAT missing from ${RBRR_GITHUB_PAT_ENV}"
  test -n "${RBRG_USERNAME:-}" || bcu_die "RBRG_USERNAME missing from ${RBRR_GITHUB_PAT_ENV}"

  # Module Variables (ZRBCG_*)
  ZRBCG_REGISTRY="ghcr.io"
  ZRBCG_GITAPI_URL="https://api.github.com"
  ZRBCG_PACKAGES_URL="${ZRBCG_GITAPI_URL}/user/packages/container/${RBRR_REGISTRY_NAME}/versions"
  ZRBCG_IMAGE_PREFIX="${ZRBCG_REGISTRY}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}"
  ZRBCG_GHCR_V2_API="https://ghcr.io/v2/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}"
  ZRBCG_TOKEN_URL="https://ghcr.io/token?scope=repository:${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:pull&service=ghcr.io"
  ZRBCG_AUTH_TOKEN=""

  bcu_step "Login to GitHub Container Registry"
  ${RBC_RUNTIME} ${RBC_RUNTIME_ARG} login "${ZRBCG_REGISTRY}" \
                                     -u "${RBRG_USERNAME}"    \
                                     -p "${RBRG_PAT}"         \
                  || bcu_die "Failed cmd"
  bcu_step "Logged in to ${ZRBCG_REGISTRY}"

  bcu_step "Obtaining bearer token for registry API"
  local token_out="${RBC_TEMP_DIR}/bearer_token.out"
  local token_err="${RBC_TEMP_DIR}/bearer_token.err"
  local bearer_token

  curl -sL -u "${RBRG_USERNAME}:${RBRG_PAT}" "${ZRBCG_TOKEN_URL}" >"${token_out}" 2>"${token_err}" && \
               bearer_token=$(jq -r '.token'                       "${token_out}")                 && \
    test -n "${bearer_token}"                                                                      && \
    test    "${bearer_token}" != "null" || bcu_die "Failed to obtain bearer token"

  ZRBCG_AUTH_TOKEN="${bearer_token}"
  bcu_step "Bearer token obtained"
}

rbcg_push() {
  local tag="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Push image to GHCR"
  bcu_doc_param "tag" "Image tag to push"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die

  local fqin="${ZRBCG_IMAGE_PREFIX}:${tag}"
  bcu_step "Push image ${fqin}"

  ${RBC_RUNTIME} ${RBC_RUNTIME_ARG} push "${fqin}" || bcu_die "Failed push"

  bcu_step "Image pushed successfully"
}

rbcg_pull() {
  local tag="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Pull image from GHCR"
  bcu_doc_param "tag" "Image tag to pull"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die

  local fqin="${ZRBCG_IMAGE_PREFIX}:${tag}"
  bcu_step "Pull image ${fqin}"

  ${RBC_RUNTIME} ${RBC_RUNTIME_ARG} pull "${fqin}" || bcu_die "Failed pull"

  bcu_step "Image pulled successfully"
}

rbcg_delete() {
  local tag="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Delete image from GHCR"
  bcu_doc_param "tag" "Image tag to delete"
  bcu_doc_lines \
    "WARNING: GHCR has known issues with layer reference counting." \
    "Deleting images may orphan or incorrectly delete shared layers."
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die

  bcu_warn "GHCR DELETE HAS KNOWN ISSUES WITH LAYER MANAGEMENT"
  bcu_warn "Shared layers may be incorrectly deleted or orphaned"

  bcu_step "Delete tag ${tag} from GHCR"

  # Find version ID for tag
  local version_id
  version_id=$(zrbcg_curl_get "${ZRBCG_PACKAGES_URL}?per_page=100" | \
    jq -r '.[] | select(.metadata.container.tags[] | contains("'"${tag}"'")) | .id' | head -n1)

  test -n "${version_id}" || bcu_die "Tag ${tag} not found"

  local delete_url="${ZRBCG_PACKAGES_URL}/${version_id}"
  local response
  response=$(curl -X DELETE -s -H "Authorization: token ${RBRG_PAT}" \
                               -H "Accept: ${RBC_MTYPE_GHV3}"        \
                               "${delete_url}"                       \
                               -w "\nHTTP_STATUS:%{http_code}\n")

  echo "${response}" | grep -q "HTTP_STATUS:204" || bcu_die "Delete failed"

  bcu_step "Tag deleted (layer cleanup unreliable on GHCR)"
}

rbcg_exists() {
  local tag="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Check if tag exists in GHCR"
  bcu_doc_param "tag" "Image tag to check"
  bcu_doc_lines "Returns exit code 0 if exists, 1 if not"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die

  # Check using GitHub API
  zrbcg_curl_get "${ZRBCG_PACKAGES_URL}?per_page=100" | \
    jq -e '.[] | select(.metadata.container.tags[] | contains("'"${tag}"'"))' >/dev/null 2>&1
}

rbcg_tags() {
  local output_IMAGE_RECORDS_json="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "List all tags in GHCR repository"
  bcu_doc_param "output_IMAGE_RECORDS_json"  "Path to write tags JSON (IMAGE_RECORDS schema)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$output_IMAGE_RECORDS_json" || bcu_usage_die

  bcu_step "Fetching all tags from GHCR"

  # Initialize empty array
  echo "[]" > "${output_IMAGE_RECORDS_json}"

  local page=1
  local temp_page="${RBC_TEMP_DIR}/temp_page.json"
  local temp_tags="${RBC_TEMP_DIR}/temp_tags.json"

  while true; do
    local url="${ZRBCG_PACKAGES_URL}?per_page=100&page=${page}"
    zrbcg_curl_get "$url" > "${temp_page}"

    local items=$(jq '. | length' "${temp_page}")
    test "${items}" -ne 0 || break

    # Extract tags without version_id
    jq -r '[.[] | select(.metadata.container.tags | length > 0) |
            .id as $version_id |
            .metadata.container.tags[] as $tag |
            {ghcr_version_id: $version_id, tag: $tag, fqin: ("'${ZRBCG_IMAGE_PREFIX}':" + $tag)}]' \
      "${temp_page}" > "${temp_tags}"

    # Merge with existing
    jq -s '.[0] + .[1]' "${output_IMAGE_RECORDS_json}" "${temp_tags}" > "${output_IMAGE_RECORDS_json}.tmp"
    mv "${output_IMAGE_RECORDS_json}.tmp" "${output_IMAGE_RECORDS_json}"

    page=$((page + 1))
  done

  local total=$(jq '. | length' "${output_IMAGE_RECORDS_json}")

  bcu_step "Retrieved ${total} tags"
}

rbcg_fetch_config_blob() {
  local config_digest="${1:-}"
  local output_config_ocijson="${2:-}"

  # Handle documentation mode
  bcu_doc_brief "Fetch OCI config blob from GHCR registry"
  bcu_doc_param "config_digest"          "SHA256 digest of config blob (e.g., sha256:abc123...)"
  bcu_doc_param "output_config_ocijson"  "Path to write OCI config JSON"
  bcu_doc_lines \
    "Retrieves an OCI/Docker image configuration blob from GHCR's blob store." \
    "The output follows OCI Image Configuration Specification." \
    "Key fields: architecture, os, created, rootfs.type, rootfs.diff_ids[]," \
    "config.Env[], config.Cmd[], config.WorkingDir, config.User, history[]" \
    "This blob contains the image metadata and layer history." \
    "Requires rbcg_start() to be called first for authentication."
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$config_digest" || bcu_usage_die
  test -n "$output_config_ocijson" || bcu_usage_die
  test -n "${ZRBCG_AUTH_TOKEN}" || bcu_die "No auth token - call rbcg_start first"

  local headers="Authorization: Bearer ${ZRBCG_AUTH_TOKEN}"

  curl -sL -H "${headers}" \
       "${ZRBCG_GHCR_V2_API}/blobs/${config_digest}" \
       > "${output_config_ocijson}" 2>/dev/null

  # Validate JSON
  jq . "${output_config_ocijson}" >/dev/null 2>&1
}

rbcg_fetch_manifest() {
  local tag="${1:-}"
  local output_manifest_ocijson="${2:-}"

  # Handle documentation mode
  bcu_doc_brief "Fetch OCI manifest from GHCR registry"
  bcu_doc_param "tag"                      "Image tag or manifest digest to fetch"
  bcu_doc_param "output_manifest_ocijson"  "Path to write OCI manifest JSON"
  bcu_doc_lines \
    "Retrieves an OCI/Docker manifest from GHCR's v2 registry API." \
    "The output follows OCI Image Manifest Specification or Docker Manifest v2 Schema." \
    "For multi-platform images, returns a manifest list (mediaType: application/vnd.docker.distribution.manifest.list.v2+json)" \
    "For single-platform images, returns a manifest (mediaType: application/vnd.docker.distribution.manifest.v2+json)" \
    "Key fields: schemaVersion, mediaType, config.digest, layers[], manifests[] (for lists)" \
    "Requires rbcg_start() to be called first for authentication."
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die
  test -n "$output_manifest_ocijson" || bcu_usage_die
  test -n "${ZRBCG_AUTH_TOKEN}" || bcu_die "No auth token - call rbcg_start first"

  # OCI Registry v2 API standard media types
  local accept_mtypes="application/vnd.docker.distribution.manifest.v2+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.oci.image.index.v1+json,application/vnd.oci.image.manifest.v1+json"
  local headers="Authorization: Bearer ${ZRBCG_AUTH_TOKEN}"

  curl -sL -H "${headers}" \
       -H "Accept: ${accept_mtypes}" \
       "${ZRBCG_GHCR_V2_API}/manifests/${tag}" \
       > "${output_manifest_ocijson}" 2>/dev/null

  # Validate JSON
  jq . "${output_manifest_ocijson}" >/dev/null 2>&1
}

rbcg_inspect() {
  local tag="${1:-}"
  local output_MANIFEST_json="${2:-}"

  # Handle documentation mode
  bcu_doc_brief "Get raw manifest and config for a tag"
  bcu_doc_param "tag"                   "Image tag to inspect"
  bcu_doc_param "output_MANIFEST_json"  "Path to write manifest JSON"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die
  test -n "$output_MANIFEST_json" || bcu_usage_die

  bcu_step "Fetching manifest for tag ${tag}"

  # Use fetch function (requires auth token from rbcg_start)
  rbcg_fetch_manifest "${tag}" "${output_MANIFEST_json}" \
    || bcu_die "Failed to fetch manifest"

  bcu_step "Manifest saved to ${output_MANIFEST_json}"
}

# eof

