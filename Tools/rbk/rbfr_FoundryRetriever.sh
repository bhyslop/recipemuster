#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# Recipe Bottle Foundry Retriever - wrest and summon operations
# Director credentials for wrest; retriever credentials for summon

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBFR_SOURCED:-}" || buc_die "Module rbfr multiply sourced - check sourcing hierarchy"
ZRBFR_SOURCED=1

# Source shared Foundry Core module
source "${BASH_SOURCE[0]%/*}/rbfc_FoundryCore.sh"

######################################################################
# Internal Functions (zrbfr_*)

zrbfr_kindle() {
  test -z "${ZRBFR_KINDLED:-}" || buc_die "Module rbfr already kindled"

  buc_log_args 'Validate Foundry Core is kindled'
  zrbfc_sentinel

  buc_log_args 'Define retriever temp file prefix'
  readonly ZRBFR_TEMP_PREFIX="${BURD_TEMP_DIR}/rbfr_"

  readonly ZRBFR_KINDLED=1
}

zrbfr_sentinel() {
  zrbfc_sentinel
  test "${ZRBFR_KINDLED:-}" = "1" || buc_die "Module rbfr not kindled - call zrbfr_kindle first"
}

######################################################################
# Public Functions (rbfr_*)

rbfr_wrest() {
  zrbfr_sentinel

  local z_locator="${1:-}"

  # Documentation block
  buc_doc_brief "Wrest an image from the registry to local container runtime by locator"
  buc_doc_param "locator" "Image locator in moniker:tag format"
  buc_doc_shown || return 0

  # Validate locator parameter
  test -n "${z_locator}" || buc_die "Locator parameter required (moniker:tag)"

  # Parse locator into moniker and tag
  case "${z_locator}" in
    *:*) : ;;
    *)   buc_die "Invalid locator format. Expected moniker:tag" ;;
  esac
  local z_moniker="${z_locator%%:*}"
  local z_tag="${z_locator#*:}"
  test -n "${z_moniker}" || buc_die "Moniker is empty in locator"
  test -n "${z_tag}" || buc_die "Tag is empty in locator"

  buc_step "Authenticating as Director"

  test -f "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Director credential not found: ${RBDC_DIRECTOR_RBRA_FILE}"

  # Get OAuth token
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  buc_step "Logging into container registry"

  # Construct full image reference
  local z_full_ref="${ZRBFC_REGISTRY_HOST}/${ZRBFC_REGISTRY_PATH}/${z_locator}"

  # Docker login to GAR
  echo "${z_token}" | docker login -u oauth2accesstoken --password-stdin "https://${ZRBFC_REGISTRY_HOST}" \
    || buc_die "Container runtime authentication failed"

  buc_step "Pulling image: ${z_full_ref}"

  # Pull image
  docker pull "${z_full_ref}" || buc_die "Image pull failed"

  # Get local image ID
  local z_image_id
  docker inspect --format='{{.Id}}' "${z_full_ref}" > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null \
    || buc_die "Failed to get image ID"
  z_image_id=$(<"${ZRBFC_SCRATCH_FILE}")

  # Display results
  echo ""
  echo "Image wrested: ${z_full_ref}"
  echo "Local image ID: ${z_image_id}"

  buc_success "Image wrest complete"
}

rbfr_summon() {
  zrbfr_sentinel

  local z_vessel="${1:-}"
  z_vessel="${z_vessel##*/}"  # strip path prefix — accept directory path or bare moniker
  local z_hallmark="${2:-}"

  # Documentation block
  buc_doc_brief "Summon an ark (pull -image, -about, and -vouch artifacts as a coherent unit)"
  buc_doc_param "vessel" "Vessel name (e.g., rbev-busybox)"
  buc_doc_param "hallmark" "Full hallmark (e.g., c260305133650-r260305160530)"
  buc_doc_shown || return 0

  buc_log_args "Validate parameters"
  rbfc_require_vessel_sigil "${z_vessel}"

  buc_step "Authenticating for retrieval"
  test -f "${RBDC_RETRIEVER_RBRA_FILE}" || buc_die "Retriever credential not found: ${RBDC_RETRIEVER_RBRA_FILE}"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_RETRIEVER_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  # List vouched hallmarks when hallmark parameter is missing
  if test -z "${z_hallmark}"; then
    local -r z_tags_file="${ZRBFR_TEMP_PREFIX}summon_tags.json"
    local -r z_stderr_file="${ZRBFR_TEMP_PREFIX}summon_tags_stderr.txt"
    curl -sL \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
      -H "Authorization: Bearer ${z_token}" \
      "${ZRBFC_REGISTRY_API_BASE}/${z_vessel}/tags/list" \
      > "${z_tags_file}" 2>"${z_stderr_file}" \
      || buc_die "Failed to fetch tags for ${z_vessel} — see ${z_stderr_file}"
    local -r z_all_tags_file="${ZRBFR_TEMP_PREFIX}summon_all_tags.txt"
    jq -r '.tags[]? // empty' "${z_tags_file}" > "${z_all_tags_file}" \
      || buc_die "Failed to extract tags for ${z_vessel}"
    local -r z_vouched_file="${ZRBFR_TEMP_PREFIX}summon_vouched.txt"
    : > "${z_vouched_file}"
    local -r z_suffix="${RBGC_ARK_SUFFIX_VOUCH}"
    while IFS= read -r z_tag || test -n "${z_tag}"; do
      case "${z_tag}" in
        *"${z_suffix}") printf '%s\n' "${z_tag%"${z_suffix}"}" >> "${z_vouched_file}" ;;
      esac
    done < "${z_all_tags_file}"
    local -r z_sorted_file="${ZRBFR_TEMP_PREFIX}summon_vouched_sorted.txt"
    sort -r "${z_vouched_file}" > "${z_sorted_file}" \
      || buc_die "Failed to sort vouched hallmarks"
    if test -s "${z_sorted_file}"; then
      buc_step "Vouched hallmarks for ${z_vessel}:"
      while IFS= read -r z_h || test -n "${z_h}"; do
        buc_bare "        ${z_h}"
      done < "${z_sorted_file}"
    else
      buc_step "No vouched hallmarks found for ${z_vessel}"
    fi
    buc_die "Hallmark parameter required"
  fi

  # Construct ark tags — all use full hallmark
  local z_image_tag="${z_hallmark}${RBGC_ARK_SUFFIX_IMAGE}"
  local z_about_tag="${z_hallmark}${RBGC_ARK_SUFFIX_ABOUT}"
  local z_vouch_tag="${z_hallmark}${RBGC_ARK_SUFFIX_VOUCH}"

  buc_step "Verifying ark existence"

  # Check if -image artifact exists
  local z_image_status_file="${ZRBFR_TEMP_PREFIX}summon_image_status.txt"
  local z_image_response_file="${ZRBFR_TEMP_PREFIX}summon_image_response.json"

  curl --head -s                                     \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
    -H "Authorization: Bearer ${z_token}"           \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}"     \
    -w "%{http_code}"                               \
    -o "${z_image_response_file}"                   \
    "${ZRBFC_REGISTRY_API_BASE}/${z_vessel}/manifests/${z_image_tag}" \
    > "${z_image_status_file}" || buc_die "HEAD request failed for -image artifact"

  local z_image_http_code
  z_image_http_code=$(<"${z_image_status_file}")
  test -n "${z_image_http_code}" || buc_die "HTTP status code is empty for -image"

  local z_image_exists=false
  if test "${z_image_http_code}" = "200"; then
    z_image_exists=true
  elif test "${z_image_http_code}" != "404"; then
    buc_die "Unexpected HTTP status ${z_image_http_code} when checking -image artifact"
  fi

  # Check if -about artifact exists
  local z_about_status_file="${ZRBFR_TEMP_PREFIX}summon_about_status.txt"
  local z_about_response_file="${ZRBFR_TEMP_PREFIX}summon_about_response.json"

  curl --head -s                                     \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
    -H "Authorization: Bearer ${z_token}"           \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}"     \
    -w "%{http_code}"                               \
    -o "${z_about_response_file}"                   \
    "${ZRBFC_REGISTRY_API_BASE}/${z_vessel}/manifests/${z_about_tag}" \
    > "${z_about_status_file}" || buc_die "HEAD request failed for -about artifact"

  local z_about_http_code
  z_about_http_code=$(<"${z_about_status_file}")
  test -n "${z_about_http_code}" || buc_die "HTTP status code is empty for -about"

  local z_about_exists=false
  if test "${z_about_http_code}" = "200"; then
    z_about_exists=true
  elif test "${z_about_http_code}" != "404"; then
    buc_die "Unexpected HTTP status ${z_about_http_code} when checking -about artifact"
  fi

  # Check if -vouch artifact exists
  local z_vouch_status_file="${ZRBFR_TEMP_PREFIX}summon_vouch_status.txt"
  local z_vouch_response_file="${ZRBFR_TEMP_PREFIX}summon_vouch_response.json"

  curl --head -s                                     \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
    -H "Authorization: Bearer ${z_token}"           \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}"     \
    -w "%{http_code}"                               \
    -o "${z_vouch_response_file}"                   \
    "${ZRBFC_REGISTRY_API_BASE}/${z_vessel}/manifests/${z_vouch_tag}" \
    > "${z_vouch_status_file}" || buc_die "HEAD request failed for -vouch artifact"

  local z_vouch_http_code
  z_vouch_http_code=$(<"${z_vouch_status_file}")
  test -n "${z_vouch_http_code}" || buc_die "HTTP status code is empty for -vouch"

  local z_vouch_exists=false
  if test "${z_vouch_http_code}" = "200"; then
    z_vouch_exists=true
  elif test "${z_vouch_http_code}" != "404"; then
    buc_die "Unexpected HTTP status ${z_vouch_http_code} when checking -vouch artifact"
  fi

  # Evaluate ark state
  if test "${z_image_exists}" = "false" && test "${z_about_exists}" = "false"; then
    buc_die "Hallmark not found: neither -image nor -about exists"
  fi

  if test "${z_image_exists}" = "true" && test "${z_about_exists}" = "false"; then
    buc_warn "Orphaned artifact detected: -image exists but -about is missing"
  elif test "${z_image_exists}" = "false" && test "${z_about_exists}" = "true"; then
    buc_warn "Orphaned artifact detected: -about exists but -image is missing"
  fi

  buc_step "Logging into container registry"

  # Docker login to GAR
  echo "${z_token}" | docker login -u oauth2accesstoken --password-stdin "https://${ZRBFC_REGISTRY_HOST}" \
    || buc_die "Container runtime authentication failed"

  # Pull -image artifact if exists
  if test "${z_image_exists}" = "true"; then
    buc_step "Pulling -image artifact"

    local z_image_ref="${ZRBFC_REGISTRY_HOST}/${ZRBFC_REGISTRY_PATH}/${z_vessel}:${z_image_tag}"
    docker pull "${z_image_ref}" || buc_die "Failed to pull -image artifact"
    buc_info "Retrieved: ${z_image_ref}"
  fi

  # Pull -about artifact if exists
  if test "${z_about_exists}" = "true"; then
    buc_step "Pulling -about artifact"

    local z_about_ref="${ZRBFC_REGISTRY_HOST}/${ZRBFC_REGISTRY_PATH}/${z_vessel}:${z_about_tag}"
    docker pull "${z_about_ref}" || buc_die "Failed to pull -about artifact"
    buc_info "Retrieved: ${z_about_ref}"
  fi

  # Pull -vouch artifact if exists
  if test "${z_vouch_exists}" = "true"; then
    buc_step "Pulling -vouch artifact"

    local z_vouch_ref="${ZRBFC_REGISTRY_HOST}/${ZRBFC_REGISTRY_PATH}/${z_vessel}:${z_vouch_tag}"
    docker pull "${z_vouch_ref}" || buc_die "Failed to pull -vouch artifact"
    buc_info "Retrieved: ${z_vouch_ref}"
  fi

  # Display results
  echo ""
  buc_success "Hallmark summoned: ${z_vessel}/${z_hallmark}"
  if test "${z_image_exists}" = "true"; then
    echo "  - ${z_vessel}:${z_image_tag} retrieved"
  fi
  if test "${z_about_exists}" = "true"; then
    echo "  - ${z_vessel}:${z_about_tag} retrieved"
  fi
  if test "${z_vouch_exists}" = "true"; then
    echo "  - ${z_vessel}:${z_vouch_tag} retrieved"
  fi
}

# eof
