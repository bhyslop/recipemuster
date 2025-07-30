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
# Recipe Bottle Container ECR - AWS ECR Registry Implementation

set -euo pipefail

ZRBCE_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBCE_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBCE_SCRIPT_DIR}/bvu_BashValidationUtility.sh"

######################################################################
# Internal Functions (zrbce_*)

zrbce_environment() {
  # Handle documentation mode
  bcu_doc_env "RBG_TEMP_DIR  " "Empty temporary directory"
  bcu_doc_env "RBG_RBRR_FILE " "File containing the RBRR constants"

  bcu_env_done || return 0

  # Validate environment
  bvu_dir_exists  "${RBG_TEMP_DIR}"
  bvu_dir_empty   "${RBG_TEMP_DIR}"
  bvu_file_exists "${RBG_RBRR_FILE}"

  # Source RBRR configuration
  source "${ZRBCE_SCRIPT_DIR}/rbrr.validator.sh"

  # Source AWS credentials
  bvu_file_exists "${RBRE_AWS_CREDENTIALS_ENV}"
  source          "${RBRE_AWS_CREDENTIALS_ENV}"

  # Extract and validate AWS credentials
  test -n "${RBRE_AWS_ACCESS_KEY_ID:-}"     || bcu_die "RBRE_AWS_ACCESS_KEY_ID missing from ${RBRE_AWS_CREDENTIALS_ENV}"
  test -n "${RBRE_AWS_SECRET_ACCESS_KEY:-}" || bcu_die "RBRE_AWS_SECRET_ACCESS_KEY missing from ${RBRE_AWS_CREDENTIALS_ENV}"
  test -n "${RBRE_AWS_ACCOUNT_ID:-}"        || bcu_die "RBRE_AWS_ACCOUNT_ID missing from ${RBRE_AWS_CREDENTIALS_ENV}"
  test -n "${RBRE_AWS_REGION:-}"            || bcu_die "RBRE_AWS_REGION missing from ${RBRE_AWS_CREDENTIALS_ENV}"
  test -n "${RBRE_REPOSITORY_NAME:-}"       || bcu_die "RBRE_REPOSITORY_NAME missing from ${RBRE_AWS_CREDENTIALS_ENV}"

  # Module Variables (ZRBCE_*)
  ZRBCE_REGISTRY="${RBRE_AWS_ACCOUNT_ID}.dkr.ecr.${RBRE_AWS_REGION}.amazonaws.com"
  ZRBCE_IMAGE_PREFIX="${ZRBCE_REGISTRY}/${RBRE_REPOSITORY_NAME}"
  ZRBCE_ECR_API="https://ecr.${RBRE_AWS_REGION}.amazonaws.com"

  # ECR API version
  ZRBCE_API_VERSION="2015-09-21"

  # Container runtime variables (set by rbce_login)
  ZRBCE_RUNTIME=""
  ZRBCE_CONNECTION=""

  # Auth token cache
  ZRBCE_AUTH_TOKEN_FILE="${RBG_TEMP_DIR}/ecr_auth_token.txt"
  ZRBCE_AUTH_EXPIRY_FILE="${RBG_TEMP_DIR}/ecr_auth_expiry.txt"

  # Media types (same as Docker/OCI)
  ZRBCE_MTYPE_DLIST="application/vnd.docker.distribution.manifest.list.v2+json"
  ZRBCE_MTYPE_OCI="application/vnd.oci.image.index.v1+json"
  ZRBCE_MTYPE_DV2="application/vnd.docker.distribution.manifest.v2+json"
  ZRBCE_MTYPE_OCM="application/vnd.oci.image.manifest.v1+json"
  ZRBCE_ACCEPT_MANIFEST_MTYPES="${ZRBCE_MTYPE_DV2},${ZRBCE_MTYPE_DLIST},${ZRBCE_MTYPE_OCI},${ZRBCE_MTYPE_OCM}"
}

# Build container command with runtime and connection
zrbce_build_container_cmd() {
  local base_cmd="$1"

  if [ "${ZRBCE_RUNTIME}" = "podman" ] && [ -n "${ZRBCE_CONNECTION}" ]; then
    echo "podman --connection=${ZRBCE_CONNECTION} ${base_cmd}"
  else
    echo "${ZRBCE_RUNTIME} ${base_cmd}"
  fi
}

# Generate SHA256 hash
zrbce_sha256() {
  local input="$1"
  if command -v sha256sum >/dev/null 2>&1; then
    echo -n "$input" | sha256sum | cut -d' ' -f1
  else
    echo -n "$input" | openssl dgst -sha256 -binary | xxd -p -c 256
  fi
}

# Generate HMAC-SHA256
zrbce_hmac_sha256() {
  local key="$1"
  local data="$2"
  echo -n "$data" | openssl dgst -sha256 -mac HMAC -macopt "hexkey:$key" -binary | xxd -p -c 256
}

# AWS SigV4 signing
zrbce_sign_request() {
  local method="$1"
  local uri="$2"
  local query="$3"
  local payload="$4"
  local service="$5"

  # Date/time
  local datetime=$(date -u +"%Y%m%dT%H%M%SZ")
  local date=$(date -u +"%Y%m%d")

  # Headers
  local host="ecr.${RBRE_AWS_REGION}.amazonaws.com"
  local content_type="application/x-amz-json-1.1"

  # Payload hash
  local payload_hash=$(zrbce_sha256 "$payload")

  # Canonical request
  local canonical_headers="content-type:${content_type}\nhost:${host}\nx-amz-date:${datetime}\n"
  local signed_headers="content-type;host;x-amz-date"
  local canonical_request="${method}\n${uri}\n${query}\n${canonical_headers}\n${signed_headers}\n${payload_hash}"
  local canonical_hash=$(zrbce_sha256 "$canonical_request")

  # String to sign
  local credential_scope="${date}/${RBRE_AWS_REGION}/${service}/aws4_request"
  local string_to_sign="AWS4-HMAC-SHA256\n${datetime}\n${credential_scope}\n${canonical_hash}"

  # Signing key
  local k_date=$(zrbce_hmac_sha256 "41575334${RBRE_AWS_SECRET_ACCESS_KEY}" "$date")
  local k_region=$(zrbce_hmac_sha256 "$k_date" "${RBRE_AWS_REGION}")
  local k_service=$(zrbce_hmac_sha256 "$k_region" "$service")
  local k_signing=$(zrbce_hmac_sha256 "$k_service" "aws4_request")

  # Signature
  local signature=$(zrbce_hmac_sha256 "$k_signing" "$string_to_sign")

  # Authorization header
  echo "AWS4-HMAC-SHA256 Credential=${RBRE_AWS_ACCESS_KEY_ID}/${credential_scope}, SignedHeaders=${signed_headers}, Signature=${signature}"
}

# Make authenticated ECR API request
zrbce_api_request() {
  local target="$1"
  local payload="${2:-{}}"

  local auth_header=$(zrbce_sign_request "POST" "/" "" "$payload" "ecr")
  local datetime=$(date -u +"%Y%m%dT%H%M%SZ")

  curl -s -X POST "${ZRBCE_ECR_API}/" \
    -H "Content-Type: application/x-amz-json-1.1" \
    -H "X-Amz-Target: AmazonEC2ContainerRegistry_V${ZRBCE_API_VERSION//-/}.${target}" \
    -H "X-Amz-Date: ${datetime}" \
    -H "Authorization: ${auth_header}" \
    -d "$payload"
}

# Get or refresh ECR auth token
zrbce_get_auth_token() {
  # Check if we have a cached token
  if [ -f "${ZRBCE_AUTH_TOKEN_FILE}" ] && [ -f "${ZRBCE_AUTH_EXPIRY_FILE}" ]; then
    local expiry=$(cat "${ZRBCE_AUTH_EXPIRY_FILE}")
    local now=$(date +%s)

    if [ "$now" -lt "$expiry" ]; then
      cat "${ZRBCE_AUTH_TOKEN_FILE}"
      return
    fi
  fi

  # Get new token
  local response=$(zrbce_api_request "GetAuthorizationToken" '{}')

  local auth_data=$(echo "$response" | jq -r '.authorizationData[0].authorizationToken' 2>/dev/null)
  test -n "$auth_data" && test "$auth_data" != "null" || bcu_die "Failed to get ECR auth token"

  # Cache token (expires in 12 hours)
  echo "$auth_data" > "${ZRBCE_AUTH_TOKEN_FILE}"
  echo $(($(date +%s) + 43200)) > "${ZRBCE_AUTH_EXPIRY_FILE}"

  echo "$auth_data"
}

# Process a single manifest
zrbce_process_single_manifest() {
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

  # Use registry v2 API to fetch config
  rbce_fetch_config_blob "${config_digest}" "${config_out}" || {
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

######################################################################
# External Functions (rbce_*)

rbce_login() {
  local runtime="${1:-}"
  local connection_string="${2:-}"

  # Handle documentation mode
  bcu_doc_brief "Login to ECR"
  bcu_doc_param "runtime" "Container runtime to use (docker or podman)"
  bcu_doc_oparm "connection_string" "Connection string for podman remote connections"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$runtime" || bcu_usage_die

  # Store runtime settings
  ZRBCE_RUNTIME="${runtime}"
  ZRBCE_CONNECTION="${connection_string}"

  bcu_step "Login to AWS ECR"

  # Get auth token
  local auth_token
  auth_token=$(zrbce_get_auth_token)

  # Extract username and password from base64 token
  local decoded=$(echo "$auth_token" | base64 -d)
  local username="${decoded%%:*}"
  local password="${decoded#*:}"

  local login_cmd=$(zrbce_build_container_cmd "login ${ZRBCE_REGISTRY} -u ${username} -p ${password}")
  eval "${login_cmd}"

  bcu_success "Logged in to ${ZRBCE_REGISTRY}"
}

rbce_push() {
  local tag="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Push image to ECR"
  bcu_doc_param "tag" "Image tag to push"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die

  # Check if repository exists
  local response=$(zrbce_api_request "DescribeRepositories" \
    "{\"repositoryNames\":[\"${RBRE_REPOSITORY_NAME}\"]}")

  echo "$response" | jq -e '.repositories[0]' >/dev/null 2>&1 || \
    bcu_die "Repository ${RBRE_REPOSITORY_NAME} does not exist in ECR. Create it first with: aws ecr create-repository --repository-name ${RBRE_REPOSITORY_NAME} --region ${RBRE_AWS_REGION}"

  local fqin="${ZRBCE_IMAGE_PREFIX}:${tag}"
  bcu_step "Push image ${fqin}"

  local push_cmd=$(zrbce_build_container_cmd "push ${fqin}")
  eval "${push_cmd}"

  bcu_success "Image pushed successfully"
}

rbce_pull() {
  local tag="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Pull image from ECR"
  bcu_doc_param "tag" "Image tag to pull"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die

  local fqin="${ZRBCE_IMAGE_PREFIX}:${tag}"
  bcu_step "Pull image ${fqin}"

  local pull_cmd=$(zrbce_build_container_cmd "pull ${fqin}")
  eval "${pull_cmd}"

  bcu_success "Image pulled successfully"
}

rbce_delete() {
  local tag="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Delete image from ECR"
  bcu_doc_param "tag" "Image tag to delete"
  bcu_doc_lines \
    "ECR properly implements OCI layer reference counting," \
    "so deleting images is safe and won't affect shared layers."
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die

  bcu_step "Delete tag ${tag} from ECR"

  # Get image digest
  local response=$(zrbce_api_request "BatchGetImage" \
    "{\"repositoryName\":\"${RBRE_REPOSITORY_NAME}\",\"imageIds\":[{\"imageTag\":\"${tag}\"}]}")

  local digest=$(echo "$response" | jq -r '.images[0].imageId.imageDigest' 2>/dev/null)
  test -n "$digest" && test "$digest" != "null" || bcu_die "Tag ${tag} not found"

  # Delete image
  response=$(zrbce_api_request "BatchDeleteImage" \
    "{\"repositoryName\":\"${RBRE_REPOSITORY_NAME}\",\"imageIds\":[{\"imageTag\":\"${tag}\",\"imageDigest\":\"${digest}\"}]}")

  echo "$response" | jq -e '.imageIds[0]' >/dev/null 2>&1 || bcu_die "Delete failed"

  bcu_success "Tag deleted successfully"
}

rbce_exists() {
  local tag="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Check if tag exists in ECR"
  bcu_doc_param "tag" "Image tag to check"
  bcu_doc_lines "Returns exit code 0 if exists, 1 if not"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die

  # Check using ECR API
  local response=$(zrbce_api_request "BatchGetImage" \
    "{\"repositoryName\":\"${RBRE_REPOSITORY_NAME}\",\"imageIds\":[{\"imageTag\":\"${tag}\"}]}")

  echo "$response" | jq -e '.images[0]' >/dev/null 2>&1
}

rbce_tags() {
  local output_IMAGE_RECORDS_json="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "List all tags in ECR repository"
  bcu_doc_param "output_IMAGE_RECORDS_json" "Path to write tags JSON (IMAGE_RECORDS schema)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$output_IMAGE_RECORDS_json" || bcu_usage_die

  bcu_step "Fetching all tags from ECR"

  # Initialize empty array
  echo "[]" > "${output_IMAGE_RECORDS_json}"

  local next_token=""
  local temp_response="${RBG_TEMP_DIR}/temp_response.json"
  local temp_tags="${RBG_TEMP_DIR}/temp_tags.json"

  while true; do
    # Build request
    local request="{\"repositoryName\":\"${RBRE_REPOSITORY_NAME}\""
    if [ -n "$next_token" ]; then
      request="${request},\"nextToken\":\"${next_token}\""
    fi
    request="${request}}"

    # List images
    zrbce_api_request "ListImages" "$request" > "${temp_response}"

    # Extract tags
    jq -r '[.imageIds[] | select(.imageTag != null) |
            {tag: .imageTag, fqin: ("'${ZRBCE_IMAGE_PREFIX}':" + .imageTag)}]' \
      "${temp_response}" > "${temp_tags}"

    # Merge with existing
    jq -s '.[0] + .[1]' "${output_IMAGE_RECORDS_json}" "${temp_tags}" > "${output_IMAGE_RECORDS_json}.tmp"
    mv "${output_IMAGE_RECORDS_json}.tmp" "${output_IMAGE_RECORDS_json}"

    # Check for more results
    next_token=$(jq -r '.nextToken // empty' "${temp_response}")
    test -n "$next_token" || break
  done

  local total=$(jq '. | length' "${output_IMAGE_RECORDS_json}")
  bcu_success "Retrieved ${total} tags"
}

rbce_fetch_config_blob() {
  local config_digest="${1:-}"
  local output_config_ocijson="${2:-}"

  # Handle documentation mode
  bcu_doc_brief "Fetch OCI config blob from ECR registry"
  bcu_doc_param "config_digest"          "SHA256 digest of config blob (e.g., sha256:abc123...)"
  bcu_doc_param "output_config_ocijson"  "Path to write OCI config JSON"
  bcu_doc_lines \
    "Retrieves an OCI/Docker image configuration blob from ECR's blob store." \
    "Uses the Docker Registry V2 API authenticated with ECR token."
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$config_digest" || bcu_usage_die
  test -n "$output_config_ocijson" || bcu_usage_die

  # Get auth token for registry API
  local auth_token=$(zrbce_get_auth_token)

  curl -sL -H "Authorization: Basic ${auth_token}" \
       "${ZRBCE_REGISTRY}/v2/${RBRE_REPOSITORY_NAME}/blobs/${config_digest}" \
       > "${output_config_ocijson}" 2>/dev/null

  # Validate JSON
  jq . "${output_config_ocijson}" >/dev/null 2>&1
}

rbce_fetch_manifest() {
  local tag="${1:-}"
  local output_manifest_ocijson="${2:-}"

  # Handle documentation mode
  bcu_doc_brief "Fetch OCI manifest from ECR registry"
  bcu_doc_param "tag"                      "Image tag or manifest digest to fetch"
  bcu_doc_param "output_manifest_ocijson"  "Path to write OCI manifest JSON"
  bcu_doc_lines \
    "Retrieves an OCI/Docker manifest from ECR's v2 registry API."
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die
  test -n "$output_manifest_ocijson" || bcu_usage_die

  # Get auth token for registry API
  local auth_token=$(zrbce_get_auth_token)

  curl -sL -H "Authorization: Basic ${auth_token}" \
       -H "Accept: ${ZRBCE_ACCEPT_MANIFEST_MTYPES}" \
       "${ZRBCE_REGISTRY}/v2/${RBRE_REPOSITORY_NAME}/manifests/${tag}" \
       > "${output_manifest_ocijson}" 2>/dev/null

  # Validate JSON
  jq . "${output_manifest_ocijson}" >/dev/null 2>&1
}

rbce_inspect() {
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

  rbce_fetch_manifest "${tag}" "${output_MANIFEST_json}" || \
    bcu_die "Failed to fetch manifest"

  bcu_success "Manifest saved to ${output_MANIFEST_json}"
}

rbce_layers() {
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

  # Initialize details file
  echo "[]" > "${output_IMAGE_DETAILS_json}"

  # Process each tag from input file
  local tag
  for tag in $(jq -r '.[].tag' "${input_IMAGE_RECORDS_json}" | sort -u); do

    bcu_info "Processing tag: ${tag}"

    local safe_tag="${tag//\//_}"
    local manifest_out="${RBG_TEMP_DIR}/manifest_${safe_tag}.json"
    local temp_detail="${RBG_TEMP_DIR}/temp_detail.json"

    # Fetch manifest
    rbce_fetch_manifest "${tag}" "${manifest_out}" || continue

    # Check media type
    local media_type
    media_type=$(jq -r '.mediaType // .schemaVersion' "${manifest_out}")

    if [[ "${media_type}" == "${ZRBCE_MTYPE_DLIST}" ]] || \
       [[ "${media_type}" == "${ZRBCE_MTYPE_OCI}"   ]]; then
      # Multi-platform
      local manifests
      manifests=$(jq -c '.manifests[]' "${manifest_out}")

      while IFS= read -r platform_manifest; do
        local platform_digest os_arch

        { read -r platform_digest os_arch; } < <(
          jq -e -r '.digest, "\(.platform.os)/\(.platform.architecture)"' <<<"${platform_manifest}") \
            || bcu_die "Invalid platform_manifest JSON"

        local platform_out="${RBG_TEMP_DIR}/platform_${safe_tag}.json"

        # Fetch platform manifest
        rbce_fetch_manifest "${platform_digest}" "${platform_out}" || continue

        if zrbce_process_single_manifest "${tag}" "${platform_out}" "${os_arch}" "${temp_detail}"; then
          jq -s '.[0] + [.[1]]' "${output_IMAGE_DETAILS_json}" "${temp_detail}" \
                              > "${output_IMAGE_DETAILS_json}.tmp"
          mv                    "${output_IMAGE_DETAILS_json}.tmp" \
                                "${output_IMAGE_DETAILS_json}"
        fi
      done <<< "$manifests"

    else
      # Single platform
      if zrbce_process_single_manifest "${tag}" "${manifest_out}" "" "${temp_detail}"; then
        jq -s '.[0] + [.[1]]' "${output_IMAGE_DETAILS_json}" "${temp_detail}" \
                            > "${output_IMAGE_DETAILS_json}.tmp"
        mv                    "${output_IMAGE_DETAILS_json}.tmp" \
                              "${output_IMAGE_DETAILS_json}"
      fi
    fi
  done

  # Generate stats (same as GHCR)
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

bcu_execute rbce_ "Recipe Bottle Container ECR - AWS ECR Implementation" zrbce_environment "$@"

# eof
