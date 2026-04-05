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
# Recipe Bottle Foundry Ledger - inscribe, jettison, abjure, and tally operations (director credentials)

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBFL_SOURCED:-}" || buc_die "Module rbfl multiply sourced - check sourcing hierarchy"
ZRBFL_SOURCED=1

# Source shared Foundry Core module
source "${BASH_SOURCE[0]%/*}/rbfc_FoundryCore.sh"

######################################################################
# Internal Functions (zrbfl_*)

zrbfl_kindle() {
  test -z "${ZRBFL_KINDLED:-}" || buc_die "Module rbfl already kindled"

  buc_log_args 'Validate Foundry Core is kindled'
  zrbfc_sentinel

  buc_log_args 'Verify Director RBRA file'
  test -n "${RBDC_DIRECTOR_RBRA_FILE:-}" || buc_die "RBDC_DIRECTOR_RBRA_FILE not set"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}"   || buc_die "GCB service env file not found: ${RBDC_DIRECTOR_RBRA_FILE}"

  buc_log_args 'RBGJI inscribe step scripts (same Tools directory)'
  local z_self_dir="${BASH_SOURCE[0]%/*}"
  readonly ZRBFL_RBGJI_STEPS_DIR="${z_self_dir}/rbgji"
  test -d "${ZRBFL_RBGJI_STEPS_DIR}"   || buc_die "RBGJI steps directory not found: ${ZRBFL_RBGJI_STEPS_DIR}"

  buc_log_args 'Define delete operation file prefix'
  readonly ZRBFL_DELETE_PREFIX="${BURD_TEMP_DIR}/rbfl_delete_"

  buc_log_args 'Define reliquary inscribe operation file prefix'
  readonly ZRBFL_RELIQUARY_PREFIX="${BURD_TEMP_DIR}/rbfl_reliquary_"

  readonly ZRBFL_KINDLED=1
}

zrbfl_sentinel() {
  zrbfc_sentinel
  test "${ZRBFL_KINDLED:-}" = "1" || buc_die "Module rbfl not kindled - call zrbfl_kindle first"
}

######################################################################
# Internal Helpers (zrbfl_*)

# Internal: Submit inscribe Cloud Build job.
# Single step: docker pull each upstream tool image, tag for GAR, push.
# Uses gcr.io/cloud-builders/docker as step image (always pullable — Google-hosted).
zrbfl_inscribe_submit() {
  zrbfl_sentinel

  local -r z_token="${1:?Token required}"
  local -r z_reliquary="${2:?Reliquary datestamp required}"

  buc_step "Constructing inscribe Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBRR_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Inscribe step image: Google-hosted docker builder (always pullable, even under NO_PUBLIC_EGRESS)
  local -r z_step_image="gcr.io/cloud-builders/docker"

  # Assemble inscribe step from script
  local -r z_script_path="${ZRBFL_RBGJI_STEPS_DIR}/rbgji01-inscribe-mirror.sh"
  test -f "${z_script_path}" || buc_die "Inscribe step script not found: ${z_script_path}"

  local -r z_body_file="${ZRBFL_RELIQUARY_PREFIX}body.txt"
  local -r z_escaped_file="${ZRBFL_RELIQUARY_PREFIX}escaped.txt"

  buc_log_args "Reading inscribe step script (skip shebang)"
  tail -n +2 "${z_script_path}" > "${z_body_file}" \
    || buc_die "Failed to read inscribe step script"
  local z_body=""
  z_body=$(<"${z_body_file}")
  test -n "${z_body}" || buc_die "Empty inscribe script body"

  printf '#!/bin/bash\n%s' "${z_body}" > "${z_escaped_file}" \
    || buc_die "Failed to write escaped inscribe script body"

  local -r z_step_file="${ZRBFL_RELIQUARY_PREFIX}step.json"
  echo "[]" > "${z_step_file}" || buc_die "Failed to initialize inscribe step JSON"

  local -r z_step_built="${ZRBFL_RELIQUARY_PREFIX}step_built.json"
  jq \
    --arg name "${z_step_image}" \
    --arg id "inscribe-mirror" \
    --rawfile script "${z_escaped_file}" \
    '. + [{name: $name, id: $id, script: $script}]' \
    "${z_step_file}" > "${z_step_built}" \
    || buc_die "Failed to build inscribe step JSON"
  mv "${z_step_built}" "${z_step_file}" \
    || buc_die "Failed to finalize inscribe step JSON"

  # Compose Build resource JSON
  buc_log_args "Composing inscribe Build resource JSON"
  local -r z_build_file="${ZRBFL_RELIQUARY_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps  "${z_step_file}" \
    --arg zjq_sa           "${z_mason_sa}" \
    --arg zjq_gar_host     "${z_gar_host}" \
    --arg zjq_gar_path     "${z_gar_path}" \
    --arg zjq_reliquary    "${z_reliquary}" \
    --arg zjq_pool         "${RBDC_POOL_TETHER}" \
    --arg zjq_timeout      "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGN_GAR_HOST:     $zjq_gar_host,
        _RBGN_GAR_PATH:     $zjq_gar_path,
        _RBGN_RELIQUARY:    $zjq_reliquary
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_build_file}" \
    || buc_die "Failed to compose inscribe build JSON"

  buc_log_args "Inscribe build JSON: ${z_build_file}"

  buc_step "Submitting inscribe Cloud Build"
  rbgu_http_json "POST" "${ZRBFC_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "reliquary_build_create" "${z_build_file}"
  rbgu_http_require_ok "Inscribe build submission" "reliquary_build_create"

  local z_build_id=""
  z_build_id=$(rbgu_json_field_capture "reliquary_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBFC_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBFC_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Inscribe build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbfc_wait_build_completion 120 "Inscribe"  # ~10 minutes at 5s intervals (7 images to pull+push)
}

######################################################################
# Public Functions (rbfl_*)

rbfl_inscribe() {
  zrbfl_sentinel

  buc_doc_brief "Inscribe a reliquary: mirror all tool images from upstream to a datestamped GAR namespace"
  buc_doc_shown || return 0

  # Authenticate as Director
  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Compute reliquary datestamp: r + YYMMDDHHMMSS
  local -r z_reliquary="r${BURD_NOW_STAMP:2:6}${BURD_NOW_STAMP:9:6}"
  buc_info "Reliquary: ${z_reliquary}"

  # Submit inscribe as a Cloud Build job (docker pull/tag/push on GCB)
  zrbfl_inscribe_submit "${z_token}" "${z_reliquary}"

  buc_success "Inscribe complete — reliquary ${z_reliquary} created"
  buc_info "Add RBRV_RELIQUARY=${z_reliquary} to vessel rbrv.env files to use this reliquary"
}

rbfl_jettison() {
  zrbfl_sentinel

  local z_locator="${1:-}"
  local z_force="${2:-}"

  # Documentation block
  buc_doc_brief "Jettison an image tag from the registry by locator"
  buc_doc_param "locator" "Image locator in moniker:tag format (e.g., rbev-busybox:20251231T160211Z-img)"
  buc_doc_param "--force" "Optional: skip confirmation prompt"
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

  # Check for --force flag
  local z_skip_confirm=false
  if test "${z_force}" = "--force"; then
    z_skip_confirm=true
  fi

  buc_step "Authenticating as Director"

  # Get OAuth token using Director credentials
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  # Confirm jettison unless --force
  if test "${z_skip_confirm}" = "false"; then
    buc_require "Will jettison: ${z_locator}" "yes"
  fi

  buc_step "Jettisoning: ${z_locator}"

  # Jettison by tag reference
  local z_status_file="${ZRBFL_DELETE_PREFIX}status.txt"
  local z_response_file="${ZRBFL_DELETE_PREFIX}response.json"

  curl -X DELETE -s                                   \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"             \
    -H "Authorization: Bearer ${z_token}"             \
    -w "%{http_code}"                                 \
    -o "${z_response_file}"                           \
    "${ZRBFC_REGISTRY_API_BASE}/${z_moniker}/manifests/${z_tag}" \
    > "${z_status_file}" || buc_die "DELETE request failed"

  local z_http_code
  z_http_code=$(<"${z_status_file}")
  test -n "${z_http_code}" || buc_die "HTTP status code is empty"

  if test "${z_http_code}" != "202" && test "${z_http_code}" != "204"; then
    local z_body="empty"
    if test -f "${z_response_file}"; then z_body=$(<"${z_response_file}"); fi
    buc_warn "Response body: ${z_body}"
    buc_die "Jettison failed with HTTP ${z_http_code}"
  fi

  buc_success "Jettisoned or nonexistent: ${z_locator}"
}

rbfl_abjure() {
  zrbfl_sentinel

  local z_hallmark="${2:-}"
  local z_force="${3:-}"

  # Documentation block
  buc_doc_brief "Abjure a hallmark (delete all per-platform image, about, and vouch artifacts)"
  buc_doc_param "vessel" "Vessel sigil or path to vessel directory"
  buc_doc_param "hallmark" "Full hallmark (e.g., c260305133650-r260305160530)"
  buc_doc_param "--force" "Optional: skip confirmation prompt"
  buc_doc_shown || return 0

  # Resolve vessel argument (sigil or path) and load
  zrbfc_resolve_vessel "${1:-}"
  local -r z_vessel_dir=$(<"${ZRBFC_VESSEL_RESOLVED_DIR_FILE}")
  test -n "${z_vessel_dir}" || buc_die "Empty resolved vessel path"
  zrbfc_load_vessel "${z_vessel_dir}"

  # Validate remaining parameters
  test -n "${z_hallmark}" || buc_die "Hallmark parameter required"

  # Derive inscribe timestamp from full hallmark (needed for -multi intermediate tag)
  local -r z_inscribe_ts="${z_hallmark%%-r*}"
  test -n "${z_inscribe_ts}" || buc_die "Failed to derive inscribe timestamp from hallmark"

  # Check for --force flag
  local z_skip_confirm=false
  if test "${z_force}" = "--force"; then
    z_skip_confirm=true
  fi

  buc_step "Authenticating as Director"

  # Get OAuth token using Director credentials
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  # Build list of image tags to check/delete
  local z_image_tags=()
  if test "${RBRV_VESSEL_MODE:-conjure}" = "bind" || test "${RBRV_VESSEL_MODE:-conjure}" = "graft"; then
    # Bind and graft vessels have a single image tag (no per-platform suffixes)
    z_image_tags+=("${z_hallmark}${RBGC_ARK_SUFFIX_IMAGE}")
  else
    # Conjure vessels have per-platform suffixed tags + consumer-facing + intermediate
    local z_platforms="${RBRV_CONJURE_PLATFORMS// /,}"
    local z_platform_suffixes=()
    local z_remaining_plats="${z_platforms}"
    local z_plat=""
    local z_suffix=""
    while test -n "${z_remaining_plats}"; do
      z_plat="${z_remaining_plats%%,*}"
      z_suffix="${z_plat#linux/}"
      z_suffix="${z_suffix//\//}"
      z_platform_suffixes+=("-${z_suffix}")
      test "${z_remaining_plats}" != "${z_plat}" || break
      z_remaining_plats="${z_remaining_plats#*,}"
    done

    local z_idx=0
    for z_idx in "${!z_platform_suffixes[@]}"; do
      z_image_tags+=("${z_hallmark}${RBGC_ARK_SUFFIX_IMAGE}${z_platform_suffixes[$z_idx]}")
    done
    if test "${#z_platform_suffixes[@]}" -gt 1; then
      z_image_tags+=("${z_hallmark}${RBGC_ARK_SUFFIX_IMAGE}")
      z_image_tags+=("${z_inscribe_ts}-multi")
    fi
  fi

  # About, vouch, and diags tags use full hallmark
  local -r z_about_tag="${z_hallmark}${RBGC_ARK_SUFFIX_ABOUT}"
  local -r z_vouch_tag="${z_hallmark}${RBGC_ARK_SUFFIX_VOUCH}"
  local -r z_diags_tag="${z_hallmark}${RBGC_ARK_SUFFIX_DIAGS}"

  buc_step "Verifying ark existence"

  # Check all image tags
  local z_existing_image_tags=()
  local z_img_tag=""
  local z_img_check_idx=0
  for z_img_tag in "${z_image_tags[@]}"; do
    local z_img_status_file="${ZRBFL_DELETE_PREFIX}image_${z_img_check_idx}_status.txt"
    local z_img_response_file="${ZRBFL_DELETE_PREFIX}image_${z_img_check_idx}_response.json"

    curl --head -s                                     \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
      -H "Authorization: Bearer ${z_token}"           \
      -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}"     \
      -w "%{http_code}"                               \
      -o "${z_img_response_file}"                     \
      "${ZRBFC_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_img_tag}" \
      > "${z_img_status_file}" || buc_die "HEAD request failed for image tag: ${z_img_tag}"

    local z_img_http_code
    z_img_http_code=$(<"${z_img_status_file}")
    test -n "${z_img_http_code}" || buc_die "HTTP status code is empty for image tag: ${z_img_tag}"

    if test "${z_img_http_code}" = "200"; then
      z_existing_image_tags+=("${z_img_tag}")
    elif test "${z_img_http_code}" != "404"; then
      buc_die "Unexpected HTTP status ${z_img_http_code} for image tag: ${z_img_tag}"
    fi
    z_img_check_idx=$((z_img_check_idx + 1))
  done

  # Check if -about artifact exists
  local z_about_status_file="${ZRBFL_DELETE_PREFIX}about_status.txt"
  local z_about_response_file="${ZRBFL_DELETE_PREFIX}about_response.json"

  curl --head -s                                     \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
    -H "Authorization: Bearer ${z_token}"           \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}"     \
    -w "%{http_code}"                               \
    -o "${z_about_response_file}"                   \
    "${ZRBFC_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_about_tag}" \
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

  # Check if -vouch artifact exists (optional — older arks won't have one)
  local z_vouch_status_file="${ZRBFL_DELETE_PREFIX}vouch_status.txt"
  local z_vouch_response_file="${ZRBFL_DELETE_PREFIX}vouch_response.json"

  curl --head -s                                     \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
    -H "Authorization: Bearer ${z_token}"           \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}"     \
    -w "%{http_code}"                               \
    -o "${z_vouch_response_file}"                   \
    "${ZRBFC_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_vouch_tag}" \
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

  # Check if -diags artifact exists (optional — conjure-only, absent for bind/graft)
  local z_diags_status_file="${ZRBFL_DELETE_PREFIX}diags_status.txt"
  local z_diags_response_file="${ZRBFL_DELETE_PREFIX}diags_response.json"

  curl --head -s                                     \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
    -H "Authorization: Bearer ${z_token}"           \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}"     \
    -w "%{http_code}"                               \
    -o "${z_diags_response_file}"                   \
    "${ZRBFC_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_diags_tag}" \
    > "${z_diags_status_file}" || buc_die "HEAD request failed for -diags artifact"

  local z_diags_http_code
  z_diags_http_code=$(<"${z_diags_status_file}")
  test -n "${z_diags_http_code}" || buc_die "HTTP status code is empty for -diags"

  local z_diags_exists=false
  if test "${z_diags_http_code}" = "200"; then
    z_diags_exists=true
  elif test "${z_diags_http_code}" != "404"; then
    buc_die "Unexpected HTTP status ${z_diags_http_code} when checking -diags artifact"
  fi

  # Evaluate ark state
  if test "${#z_existing_image_tags[@]}" -eq 0 && test "${z_about_exists}" = "false"; then
    buc_die "Hallmark not found: no image tags and no -about exists for ${RBRV_SIGIL}/${z_hallmark}"
  fi

  if test "${#z_existing_image_tags[@]}" -gt 0 && test "${z_about_exists}" = "false"; then
    buc_warn "Orphaned artifact detected: image tags exist but -about is missing"
  elif test "${#z_existing_image_tags[@]}" -eq 0 && test "${z_about_exists}" = "true"; then
    buc_warn "Orphaned artifact detected: -about exists but no image tags found"
  fi

  # Confirm abjuration unless --force
  if test "${z_skip_confirm}" = "false"; then
    local z_confirm_msg="Will abjure ark ${RBRV_SIGIL}/${z_hallmark}:"
    if (( ${#z_existing_image_tags[@]} )); then
      for z_img_tag in "${z_existing_image_tags[@]}"; do
        z_confirm_msg="${z_confirm_msg}\n  - ${RBRV_SIGIL}:${z_img_tag}"
      done
    fi
    if test "${z_about_exists}" = "true"; then
      z_confirm_msg="${z_confirm_msg}\n  - ${RBRV_SIGIL}:${z_about_tag}"
    fi
    if test "${z_vouch_exists}" = "true"; then
      z_confirm_msg="${z_confirm_msg}\n  - ${RBRV_SIGIL}:${z_vouch_tag}"
    fi
    if test "${z_diags_exists}" = "true"; then
      z_confirm_msg="${z_confirm_msg}\n  - ${RBRV_SIGIL}:${z_diags_tag}"
    fi
    buc_require "${z_confirm_msg}" "yes"
  fi

  # Delete all existing image tags
  if (( ${#z_existing_image_tags[@]} )); then
    local z_img_del_idx=0
    for z_img_tag in "${z_existing_image_tags[@]}"; do
      buc_step "Deleting image tag: ${z_img_tag}"

      local z_delete_img_status="${ZRBFL_DELETE_PREFIX}delete_image_${z_img_del_idx}_status.txt"
      local z_delete_img_response="${ZRBFL_DELETE_PREFIX}delete_image_${z_img_del_idx}_response.json"

      curl -X DELETE -s                                   \
        --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
        --max-time "${RBCC_CURL_MAX_TIME_SEC}"             \
        -H "Authorization: Bearer ${z_token}"             \
        -w "%{http_code}"                                 \
        -o "${z_delete_img_response}"                     \
        "${ZRBFC_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_img_tag}" \
        > "${z_delete_img_status}" || buc_die "DELETE request failed for image tag: ${z_img_tag}"

      local z_delete_img_code
      z_delete_img_code=$(<"${z_delete_img_status}")
      test -n "${z_delete_img_code}" || buc_die "HTTP status code is empty for image tag delete: ${z_img_tag}"

      if test "${z_delete_img_code}" != "202" && test "${z_delete_img_code}" != "204"; then
        local z_body="empty"
        if test -f "${z_delete_img_response}"; then z_body=$(<"${z_delete_img_response}"); fi
        buc_warn "Response body: ${z_body}"
        buc_die "Failed to delete image tag ${z_img_tag} (HTTP ${z_delete_img_code})"
      fi

      buc_info "Deleted: ${RBRV_SIGIL}:${z_img_tag}"
      z_img_del_idx=$((z_img_del_idx + 1))
    done
  fi

  # Delete -about artifact if exists
  if test "${z_about_exists}" = "true"; then
    buc_step "Deleting -about artifact"

    local z_delete_about_status="${ZRBFL_DELETE_PREFIX}delete_about_status.txt"
    local z_delete_about_response="${ZRBFL_DELETE_PREFIX}delete_about_response.json"

    curl -X DELETE -s                                   \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}"             \
      -H "Authorization: Bearer ${z_token}"             \
      -w "%{http_code}"                                 \
      -o "${z_delete_about_response}"                   \
      "${ZRBFC_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_about_tag}" \
      > "${z_delete_about_status}" || buc_die "DELETE request failed for -about"

    local z_delete_about_code
    z_delete_about_code=$(<"${z_delete_about_status}")
    test -n "${z_delete_about_code}" || buc_die "HTTP status code is empty for -about delete"

    if test "${z_delete_about_code}" != "202" && test "${z_delete_about_code}" != "204"; then
      local z_body="empty"
      if test -f "${z_delete_about_response}"; then z_body=$(<"${z_delete_about_response}"); fi
      buc_warn "Response body: ${z_body}"
      buc_die "Failed to delete -about artifact (HTTP ${z_delete_about_code})"
    fi

    buc_info "Deleted: ${RBRV_SIGIL}:${z_about_tag}"
  fi

  # Delete -vouch artifact if exists (optional — older arks won't have one)
  if test "${z_vouch_exists}" = "true"; then
    buc_step "Deleting -vouch artifact"

    local z_delete_vouch_status="${ZRBFL_DELETE_PREFIX}delete_vouch_status.txt"
    local z_delete_vouch_response="${ZRBFL_DELETE_PREFIX}delete_vouch_response.json"

    curl -X DELETE -s                                   \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}"             \
      -H "Authorization: Bearer ${z_token}"             \
      -w "%{http_code}"                                 \
      -o "${z_delete_vouch_response}"                   \
      "${ZRBFC_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_vouch_tag}" \
      > "${z_delete_vouch_status}" || buc_die "DELETE request failed for -vouch"

    local z_delete_vouch_code
    z_delete_vouch_code=$(<"${z_delete_vouch_status}")
    test -n "${z_delete_vouch_code}" || buc_die "HTTP status code is empty for -vouch delete"

    if test "${z_delete_vouch_code}" != "202" && test "${z_delete_vouch_code}" != "204"; then
      local z_body="empty"
      if test -f "${z_delete_vouch_response}"; then z_body=$(<"${z_delete_vouch_response}"); fi
      buc_warn "Response body: ${z_body}"
      buc_die "Failed to delete -vouch artifact (HTTP ${z_delete_vouch_code})"
    fi

    buc_info "Deleted: ${RBRV_SIGIL}:${z_vouch_tag}"
  fi

  # Delete -diags artifact if exists (optional — conjure-only)
  if test "${z_diags_exists}" = "true"; then
    buc_step "Deleting -diags artifact"

    local z_delete_diags_status="${ZRBFL_DELETE_PREFIX}delete_diags_status.txt"
    local z_delete_diags_response="${ZRBFL_DELETE_PREFIX}delete_diags_response.json"

    curl -X DELETE -s                                   \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}"             \
      -H "Authorization: Bearer ${z_token}"             \
      -w "%{http_code}"                                 \
      -o "${z_delete_diags_response}"                   \
      "${ZRBFC_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_diags_tag}" \
      > "${z_delete_diags_status}" || buc_die "DELETE request failed for -diags"

    local z_delete_diags_code
    z_delete_diags_code=$(<"${z_delete_diags_status}")
    test -n "${z_delete_diags_code}" || buc_die "HTTP status code is empty for -diags delete"

    if test "${z_delete_diags_code}" != "202" && test "${z_delete_diags_code}" != "204"; then
      local z_body="empty"
      if test -f "${z_delete_diags_response}"; then z_body=$(<"${z_delete_diags_response}"); fi
      buc_warn "Response body: ${z_body}"
      buc_die "Failed to delete -diags artifact (HTTP ${z_delete_diags_code})"
    fi

    buc_info "Deleted: ${RBRV_SIGIL}:${z_diags_tag}"
  fi

  # Display results
  echo ""
  buc_success "Hallmark abjured: ${RBRV_SIGIL}/${z_hallmark}"
  if (( ${#z_existing_image_tags[@]} )); then
    for z_img_tag in "${z_existing_image_tags[@]}"; do
      echo "  - ${RBRV_SIGIL}:${z_img_tag} deleted"
    done
  fi
  if test "${z_about_exists}" = "true"; then
    echo "  - ${RBRV_SIGIL}:${z_about_tag} deleted"
  fi
  if test "${z_vouch_exists}" = "true"; then
    echo "  - ${RBRV_SIGIL}:${z_vouch_tag} deleted"
  fi
  if test "${z_diags_exists}" = "true"; then
    echo "  - ${RBRV_SIGIL}:${z_diags_tag} deleted"
  fi
}

rbfl_tally() {
  zrbfl_sentinel

  buc_doc_brief "List hallmarks across all vessels with health status"
  buc_doc_shown || return 0

  buc_step "Enumerating vessels"
  local z_sigils
  z_sigils=$(rbrv_list_capture) || buc_die "No vessels found"

  buc_step "Fetching OAuth token (Director)"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  local z_any_pending=0
  local z_any_incomplete=0

  local z_sigil=""
  for z_sigil in ${z_sigils}; do

    buc_step "Querying GAR tags for ${z_sigil}"
    local z_tags_file="${BURD_TEMP_DIR}/rbfl_dc_${z_sigil}_tags.json"
    local z_stderr_file="${BURD_TEMP_DIR}/rbfl_dc_${z_sigil}_stderr.txt"
    curl -sL \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
      -H "Authorization: Bearer ${z_token}" \
      "${ZRBFC_REGISTRY_API_BASE}/${z_sigil}/tags/list" \
      > "${z_tags_file}" 2>"${z_stderr_file}" \
      || buc_die "Failed to fetch tags for ${z_sigil} — see ${z_stderr_file}"

    if jq -e '.errors' "${z_tags_file}" >/dev/null 2>&1; then
      local z_err
      jq -r '.errors[0].message // "Unknown error"' "${z_tags_file}" > "${ZRBFC_SCRATCH_FILE}" \
        || buc_die "Failed to extract error message from registry response for ${z_sigil}"
      z_err=$(<"${ZRBFC_SCRATCH_FILE}")
      buc_die "Registry API error for ${z_sigil}: ${z_err}"
    fi

    # Extract tags and identify full hallmarks
    local z_all_tags_file="${BURD_TEMP_DIR}/rbfl_dc_${z_sigil}_all_tags.txt"
    jq -r '.tags[]? // empty' "${z_tags_file}" > "${z_all_tags_file}" \
      || buc_die "Failed to extract tags for ${z_sigil}"

    local z_consec_file="${BURD_TEMP_DIR}/rbfl_dc_${z_sigil}_hallmarks.txt"
    local z_tag_data_file="${BURD_TEMP_DIR}/rbfl_dc_${z_sigil}_tag_data.txt"
    : > "${z_consec_file}"
    : > "${z_tag_data_file}"

    while IFS= read -r z_tag || test -n "${z_tag}"; do
      local z_consec=""
      if [[ "${z_tag}" =~ ^([cbg][0-9]{12}-r[0-9]{12}) ]]; then
        z_consec="${BASH_REMATCH[1]}"
      else
        continue
      fi

      case "${z_tag}" in
        *"${RBGC_ARK_SUFFIX_IMAGE}"-*|*"${RBGC_ARK_SUFFIX_IMAGE}")
          local z_suffix="${z_tag#*"${RBGC_ARK_SUFFIX_IMAGE}"}"
          if test -z "${z_suffix}"; then
            echo "${z_consec}|image|consumer" >> "${z_tag_data_file}"
          else
            echo "${z_consec}|image|${z_suffix#-}" >> "${z_tag_data_file}"
          fi
          echo "${z_consec}" >> "${z_consec_file}"
          ;;
        *"${RBGC_ARK_SUFFIX_VOUCH}")
          echo "${z_consec}|vouch|" >> "${z_tag_data_file}"
          echo "${z_consec}" >> "${z_consec_file}"
          ;;
        *"${RBGC_ARK_SUFFIX_ABOUT}")
          echo "${z_consec}|about|" >> "${z_tag_data_file}"
          echo "${z_consec}" >> "${z_consec_file}"
          ;;
        *"${RBGC_ARK_SUFFIX_DIAGS}")
          echo "${z_consec}|diags|" >> "${z_tag_data_file}"
          echo "${z_consec}" >> "${z_consec_file}"
          ;;
      esac
    done < "${z_all_tags_file}"

    local z_unique_file="${BURD_TEMP_DIR}/rbfl_dc_${z_sigil}_unique.txt"
    sort -ur "${z_consec_file}" > "${z_unique_file}" \
      || buc_die "Failed to sort hallmarks for ${z_sigil}"

    if ! test -s "${z_unique_file}"; then
      buc_info "No hallmarks found for ${z_sigil}"
      continue
    fi

    # Display per-vessel table with health states
    printf "\nVessel: %s\n" "${z_sigil}"
    printf "  %-42s %-30s %-10s\n" "Hallmark" "Platforms" "Health"

    while IFS= read -r z_hallmark || test -n "${z_hallmark}"; do
      local z_consec_platforms=""
      local z_has_about="no"
      local z_has_vouch="no"
      local z_has_image="no"

      while IFS='|' read -r z_c z_type z_detail; do
        test "${z_c}" = "${z_hallmark}" || continue
        if test "${z_type}" = "image"; then
          z_has_image="yes"
          if test "${z_detail}" != "consumer"; then
            if test -n "${z_consec_platforms}"; then
              z_consec_platforms="${z_consec_platforms},${z_detail}"
            else
              z_consec_platforms="${z_detail}"
            fi
          fi
        elif test "${z_type}" = "about"; then
          z_has_about="yes"
        elif test "${z_type}" = "vouch"; then
          z_has_vouch="yes"
        fi
      done < "${z_tag_data_file}"

      local z_plat_display="${z_consec_platforms:-single}"
      local z_health=""
      if test "${z_has_about}" = "yes" && test "${z_has_vouch}" = "yes"; then
        z_health="vouched"
      elif test "${z_has_about}" = "yes" && test "${z_has_vouch}" = "no"; then
        z_health="pending"
        z_any_pending=1
      elif test "${z_has_image}" = "yes" && test "${z_has_about}" = "no"; then
        z_health="incomplete"
        z_any_incomplete=1
      fi

      printf "  %-42s %-30s %-10s\n" "${z_hallmark}" "${z_plat_display}" "${z_health}"

      # Write per-hallmark fact file for test observability
      test -n "${z_hallmark}" || buc_die "Empty hallmark in unique file for ${z_sigil}"
      buf_write_fact "${z_sigil}${RBCC_FACT_CONSEC_INFIX}${z_hallmark}" "${z_sigil}"
    done < "${z_unique_file}"

  done

  echo ""

  # Tabtarget recommendations
  if test "${z_any_pending}" = "1"; then
    buc_step "Pending hallmarks can be vouched:"
    buc_tabtarget "${RBZ_VOUCH_HALLMARKS}"
  fi
  if test "${z_any_incomplete}" = "1"; then
    buc_step "Incomplete hallmarks should be abjured and re-conjured:"
    buc_tabtarget "${RBZ_ABJURE_HALLMARK}"
  fi

  buc_success "Hallmark check complete"
}


# eof
