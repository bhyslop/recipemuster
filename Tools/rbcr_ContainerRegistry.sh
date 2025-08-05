#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle Container Registry - Google Artifact Registry interface

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBCR_SOURCED:-}" || bcu_die "Module rbcr multiply sourced - check sourcing hierarchy"
ZRBCR_SOURCED=1

######################################################################
# Internal Functions (zrbcr_*)

zrbcr_kindle() {
  test -z "${ZRBCR_KINDLED:-}" || bcu_die "Module rbcr already kindled"

  # Check required environment
  test -n "${RBRR_REGISTRY:-}"       || bcu_die "RBRR_REGISTRY not set"
  test -n "${RBRR_GAR_PROJECT_ID:-}" || bcu_die "RBRR_GAR_PROJECT_ID not set"
  test -n "${RBRR_GAR_LOCATION:-}"   || bcu_die "RBRR_GAR_LOCATION not set"
  test -n "${RBRR_GAR_REPOSITORY:-}" || bcu_die "RBRR_GAR_REPOSITORY not set"
  test -n "${RBG_RUNTIME:-}"         || bcu_die "RBG_RUNTIME not set"
  test -n "${RBG_TEMP_DIR:-}"        || bcu_die "RBG_TEMP_DIR not set"

  # Source GAR credentials
  test -n "${RBRR_GAR_SERVICE_ENV:-}" || bcu_die "RBRR_GAR_SERVICE_ENV not set"
  test -f "${RBRR_GAR_SERVICE_ENV}"   || bcu_die "GAR service env file not found: ${RBRR_GAR_SERVICE_ENV}"
  source "${RBRR_GAR_SERVICE_ENV}"
  test -n "${RBRG_GAR_SERVICE_ACCOUNT_KEY:-}" || bcu_die "RBRG_GAR_SERVICE_ACCOUNT_KEY missing from ${RBRR_GAR_SERVICE_ENV}"
  test -f "${RBRG_GAR_SERVICE_ACCOUNT_KEY}"   || bcu_die "Service account key file not found: ${RBRG_GAR_SERVICE_ACCOUNT_KEY}"

  # Module Variables (ZRBCR_*)
  ZRBCR_REGISTRY_HOST="${RBRR_GAR_LOCATION}-docker.pkg.dev"
  ZRBCR_REGISTRY_PATH="${RBRR_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  ZRBCR_REGISTRY_API_BASE="https://${ZRBCR_REGISTRY_HOST}/v2/${ZRBCR_REGISTRY_PATH}"

  # Media types
  ZRBCR_MTYPE_DLIST="application/vnd.docker.distribution.manifest.list.v2+json"
  ZRBCR_MTYPE_OCI="application/vnd.oci.image.index.v1+json"
  ZRBCR_MTYPE_DV2="application/vnd.docker.distribution.manifest.v2+json"
  ZRBCR_MTYPE_OCM="application/vnd.oci.image.manifest.v1+json"
  ZRBCR_ACCEPT_MANIFEST_MTYPES="${ZRBCR_MTYPE_DV2},${ZRBCR_MTYPE_DLIST},${ZRBCR_MTYPE_OCI},${ZRBCR_MTYPE_OCM}"

  # File prefixes for all operations
  ZRBCR_MANIFEST_PREFIX="${RBG_TEMP_DIR}/manifest_"
  ZRBCR_CONFIG_PREFIX="${RBG_TEMP_DIR}/config_"
  ZRBCR_DELETE_PREFIX="${RBG_TEMP_DIR}/delete_"
  ZRBCR_DETAIL_PREFIX="${RBG_TEMP_DIR}/detail_"
  ZRBCR_TOKEN_PREFIX="${RBG_TEMP_DIR}/token_"
  ZRBCR_JWT_PREFIX="${RBG_TEMP_DIR}/jwt_"

  # Output files
  ZRBCR_IMAGE_RECORDS_FILE="${RBG_TEMP_DIR}/IMAGE_RECORDS.json"
  ZRBCR_IMAGE_DETAIL_FILE="${RBG_TEMP_DIR}/IMAGE_DETAILS.json"
  ZRBCR_IMAGE_STATS_FILE="${RBG_TEMP_DIR}/IMAGE_STATS.json"
  ZRBCR_FQIN_FILE="${RBG_TEMP_DIR}/FQIN.txt"
  ZRBCR_TOKEN_FILE="${ZRBCR_TOKEN_PREFIX}access.txt"

  # File index counter
  ZRBCR_FILE_INDEX=0

  # Extract service account details
  ZRBCR_SA_EMAIL=$(jq -r '.client_email' "${RBRG_GAR_SERVICE_ACCOUNT_KEY}")
  ZRBCR_SA_KEY_FILE="${ZRBCR_JWT_PREFIX}private.pem"
  jq -r '.private_key' "${RBRG_GAR_SERVICE_ACCOUNT_KEY}" > "${ZRBCR_SA_KEY_FILE}"

  bcu_step "Obtaining OAuth token for GAR API"
  zrbcr_refresh_token || bcu_die "Cannot proceed without OAuth token"

  # Login to registry
  bcu_step "Log in to container registry"
  ${RBG_RUNTIME} ${RBG_RUNTIME_ARG:-} login "${ZRBCR_REGISTRY_HOST}" \
    -u oauth2accesstoken -p "$(<"${ZRBCR_TOKEN_FILE}")"

  ZRBCR_KINDLED=1
}

zrbcr_sentinel() {
  test "${ZRBCR_KINDLED:-}" = "1" || bcu_die "Module rbcr not kindled - call zrbcr_kindle first"
}

zrbcr_base64url_encode() {
  zrbcr_sentinel

  bcu_info "Base64url encoding (no padding, URL-safe chars)"
  base64 -w 0 | tr '+/' '-_' | tr -d '='
}

zrbcr_refresh_token() {
  zrbcr_sentinel

  local z_now z_exp z_header z_claim z_jwt_unsigned z_signature z_jwt

  bcu_info "Building JWT header"
  z_header='{"alg":"RS256","typ":"JWT"}'

  bcu_info "Building JWT claims (1 hour expiry)"
  z_now=$(date +%s)
  z_exp=$((z_now + 3600))

  z_claim=$(jq -n \
    --arg iss "${ZRBCR_SA_EMAIL}" \
    --arg scope "https://www.googleapis.com/auth/cloud-platform" \
    --arg aud "https://oauth2.googleapis.com/token" \
    --arg iat "${z_now}" \
    --arg exp "${z_exp}" \
    '{"iss":$iss,"scope":$scope,"aud":$aud,"iat":($iat|tonumber),"exp":($exp|tonumber)}')

  # Build JWT
  z_header_enc=$(echo -n "${z_header}" | zrbcr_base64url_encode)
  z_claim_enc=$(echo -n "${z_claim}" | zrbcr_base64url_encode)
  z_jwt_unsigned="${z_header_enc}.${z_claim_enc}"

  # Sign JWT
  z_signature=$(echo -n "${z_jwt_unsigned}" | \
    openssl dgst -sha256 -sign "${ZRBCR_SA_KEY_FILE}" | \
    zrbcr_base64url_encode)

  z_jwt="${z_jwt_unsigned}.${z_signature}"

  # Exchange JWT for access token
  local z_response="${ZRBCR_TOKEN_PREFIX}response.json"
  curl -s -X POST https://oauth2.googleapis.com/token \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion=${z_jwt}" \
    > "${z_response}" || bcu_die "Failed to obtain OAuth token"

  # Extract access token
  jq -r '.access_token' "${z_response}" > "${ZRBCR_TOKEN_FILE}"
  test -s "${ZRBCR_TOKEN_FILE}" || bcu_die "Failed to extract access token"

  # Store expiry for potential refresh logic
  ZRBCR_TOKEN_EXPIRY="${z_exp}"
}

zrbcr_get_next_index() {
  zrbcr_sentinel

  ZRBCR_FILE_INDEX=$((ZRBCR_FILE_INDEX + 1))
  printf "%03d" "${ZRBCR_FILE_INDEX}" || bcu_die "Failed to format file index"
}

zrbcr_curl_registry() {
  zrbcr_sentinel

  local z_url="$1"
  local z_token
  z_token=$(<"${ZRBCR_TOKEN_FILE}")

  curl -sL \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBCR_ACCEPT_MANIFEST_MTYPES}" \
    "${z_url}"
}

zrbcr_process_single_manifest() {
  zrbcr_sentinel

  local z_tag="$1"
  local z_manifest_file="$2"
  local z_platform="$3"  # Empty for single-platform

  # Get config digest
  local z_config_digest
  z_config_digest=$(jq -r '.config.digest' "${z_manifest_file}")

  test -n "${z_config_digest}" || bcu_die "Missing config.digest"
  test "${z_config_digest}" != "null" || {
    bcu_warn "null config.digest in manifest"
    return 0
  }

  # Fetch config blob
  local z_idx
  z_idx=$(zrbcr_get_next_index)
  local z_config_out="${ZRBCR_CONFIG_PREFIX}${z_idx}.json"

  zrbcr_curl_registry "${ZRBCR_REGISTRY_API_BASE}/blobs/${z_config_digest}" \
    >"${z_config_out}" || bcu_die "Failed to fetch config blob"

  bcu_info "Validating config JSON"
  jq . "${z_config_out}" >/dev/null || bcu_die "Invalid config JSON"

  # Build detail entry
  local z_temp_detail="${ZRBCR_DETAIL_PREFIX}$(zrbcr_get_next_index).json"
  local z_manifest_json z_config_json
  z_manifest_json="$(<"${z_manifest_file}")"
  z_config_json=$(jq '. + {
    created: (.created // "1970-01-01T00:00:00Z"),
    architecture: (.architecture // "unknown"),
    os: (.os // "unknown")
  }' "${z_config_out}")

  if test -n "${z_platform}"; then
    jq -n \
      --arg tag          "${z_tag}"           \
      --arg platform     "${z_platform}"      \
      --arg digest       "${z_config_digest}" \
      --argjson manifest "${z_manifest_json}" \
      --argjson config   "${z_config_json}" '
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
      }' > "${z_temp_detail}"
  else
    jq -n \
      --arg     tag      "${z_tag}"           \
      --arg     digest   "${z_config_digest}" \
      --argjson manifest "${z_manifest_json}" \
      --argjson config   "${z_config_json}" '
      {
        tag: $tag,
        digest: $digest,
        layers: $manifest.layers,
        config: {
          created: $config.created,
          architecture: $config.architecture,
          os: $config.os
        }
      }' > "${z_temp_detail}"
  fi

  # Append to detail file
  bcu_info "Merging image detail"
  jq -s '.[0] + [.[1]]' "${ZRBCR_IMAGE_DETAIL_FILE}" "${z_temp_detail}" \
    > "${ZRBCR_IMAGE_DETAIL_FILE}.tmp" || bcu_die "Failed to merge image detail"
  mv "${ZRBCR_IMAGE_DETAIL_FILE}.tmp" "${ZRBCR_IMAGE_DETAIL_FILE}" || bcu_die "Failed to move detail file"
}

######################################################################
# External Functions (rbcr_*)

rbcr_make_fqin() {
  zrbcr_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Create fully qualified image name"
  bcu_doc_param "tag" "Image tag to qualify"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  # Write FQIN to file
  echo "${ZRBCR_REGISTRY_HOST}/${ZRBCR_REGISTRY_PATH}/${z_tag}" > "${ZRBCR_FQIN_FILE}"
}

rbcr_list_tags() {
  zrbcr_sentinel

  # Documentation block
  bcu_doc_brief "List all available image tags from registry"
  bcu_doc_shown || return 0

  bcu_step "Fetching image tags from GAR"

  # GAR uses Docker Registry v2 API - list tags endpoint
  local z_tags_response="${RBG_TEMP_DIR}/tags_response.json"

  zrbcr_curl_registry "${ZRBCR_REGISTRY_API_BASE}/tags/list" > "${z_tags_response}" \
    || bcu_die "Failed to fetch tags list"

  # Transform to records format matching GHCR output
  jq -r --arg prefix "${ZRBCR_REGISTRY_HOST}/${ZRBCR_REGISTRY_PATH}" \
    '[.tags[] | {tag: ., fqin: ($prefix + "/" + .)}]' \
    "${z_tags_response}" > "${ZRBCR_IMAGE_RECORDS_FILE}"

  local z_total
  z_total=$(jq '. | length' "${ZRBCR_IMAGE_RECORDS_FILE}")
  bcu_info "Retrieved ${z_total} total image records"
}

rbcr_get_manifest() {
  zrbcr_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Fetch and process manifest for an image tag"
  bcu_doc_param "tag" "Image tag to fetch manifest for"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  local z_idx
  z_idx=$(zrbcr_get_next_index)
  local z_manifest_out="${ZRBCR_MANIFEST_PREFIX}${z_idx}.json"

  zrbcr_curl_registry "${ZRBCR_REGISTRY_API_BASE}/manifests/${z_tag}" \
    >"${z_manifest_out}" || bcu_die "Failed to fetch manifest for ${z_tag}"

  jq . "${z_manifest_out}" >/dev/null || bcu_die "Invalid manifest JSON"

  local z_media_type
  z_media_type=$(jq -r '.mediaType // .schemaVersion' "${z_manifest_out}")

  if test "${z_media_type}" = "${ZRBCR_MTYPE_DLIST}" || \
     test "${z_media_type}" = "${ZRBCR_MTYPE_OCI}"; then

    bcu_info "Multi-platform image detected"

    local z_platform_idx=1
    local z_manifests
    z_manifests=$(jq -c '.manifests[]' "${z_manifest_out}")

    while IFS= read -r z_platform_manifest; do
      local z_platform_digest z_platform_info
      z_platform_digest=$(echo "${z_platform_manifest}" | jq -r '.digest')
      z_platform_info=$(echo "${z_platform_manifest}" | jq -r '"\(.platform.os)/\(.platform.architecture)"')

      bcu_info "Processing platform: ${z_platform_info}"

      local z_platform_idx_str
      z_platform_idx_str=$(zrbcr_get_next_index)
      local z_platform_out="${ZRBCR_MANIFEST_PREFIX}${z_platform_idx_str}.json"

      zrbcr_curl_registry "${ZRBCR_REGISTRY_API_BASE}/manifests/${z_platform_digest}" \
        >"${z_platform_out}" || bcu_die "Failed to fetch platform manifest"

      jq . "${z_platform_out}" >/dev/null || bcu_die "Invalid platform manifest JSON"

      zrbcr_process_single_manifest "${z_tag}" "${z_platform_out}" "${z_platform_info}"

      ((z_platform_idx++))
    done <<< "${z_manifests}"

  else
    bcu_info "Single platform image"
    zrbcr_process_single_manifest "${z_tag}" "${z_manifest_out}" ""
  fi
}

rbcr_get_config() {
  zrbcr_sentinel

  local z_digest="${1:-}"

  # Documentation block
  bcu_doc_brief "Fetch configuration blob for a digest"
  bcu_doc_param "digest" "Digest of config blob to fetch"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_digest}" || bcu_die "Digest parameter required"

  # Fetch config blob
  local z_idx
  z_idx=$(zrbcr_get_next_index)
  local z_config_out="${ZRBCR_CONFIG_PREFIX}${z_idx}.json"

  zrbcr_curl_registry "${ZRBCR_REGISTRY_API_BASE}/blobs/${z_digest}" \
    > "${z_config_out}" || bcu_die "Failed to fetch config blob"

  jq . "${z_config_out}" >/dev/null || bcu_die "Invalid config JSON"
}

rbcr_delete() {
  zrbcr_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Delete an image tag from the registry"
  bcu_doc_param "tag" "Image tag to delete"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  bcu_step "Fetching manifest digest for deletion"

  # Get manifest with digest header
  local z_manifest_headers="${ZRBCR_DELETE_PREFIX}headers.txt"
  local z_token
  z_token=$(<"${ZRBCR_TOKEN_FILE}")

  curl -sL -I \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBCR_ACCEPT_MANIFEST_MTYPES}" \
    "${ZRBCR_REGISTRY_API_BASE}/manifests/${z_tag}" \
    > "${z_manifest_headers}" || bcu_die "Failed to fetch manifest headers"

  # Extract digest from Docker-Content-Digest header
  local z_digest
  z_digest=$(grep -i "docker-content-digest:" "${z_manifest_headers}" | \
    sed 's/.*: //' | tr -d '\r\n')

  test -n "${z_digest}" || bcu_die "Failed to extract manifest digest"

  bcu_info "Deleting manifest: ${z_digest}"

  # Delete by digest
  local z_status_file="${ZRBCR_DELETE_PREFIX}status.txt"

  curl -X DELETE -s \
    -H "Authorization: Bearer ${z_token}" \
    -w "%{http_code}" \
    -o /dev/null \
    "${ZRBCR_REGISTRY_API_BASE}/manifests/${z_digest}" \
    > "${z_status_file}"

  local z_http_code
  z_http_code=$(<"${z_status_file}")
  test "${z_http_code}" = "202" || test "${z_http_code}" = "204" || \
    bcu_die "Delete failed with HTTP ${z_http_code}"
}

rbcr_pull() {
  zrbcr_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Pull an image from the registry"
  bcu_doc_param "tag" "Image tag to pull"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  # Construct FQIN from tag
  rbcr_make_fqin "${z_tag}"
  local z_fqin
  z_fqin=$(<"${ZRBCR_FQIN_FILE}")

  bcu_info "Fetch image..."
  ${RBG_RUNTIME} ${RBG_RUNTIME_ARG:-} pull "${z_fqin}"
}

rbcr_exists_predicate() {
  zrbcr_sentinel

  local z_tag="${1:-}"

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  # Check if tag exists
  local z_tags_response="${RBG_TEMP_DIR}/exists_check.json"

  zrbcr_curl_registry "${ZRBCR_REGISTRY_API_BASE}/tags/list" > "${z_tags_response}" 2>/dev/null || return 1

  jq -e '.tags[] | select(. == "'"${z_tag}"'")' "${z_tags_response}" > /dev/null
}

# eof

