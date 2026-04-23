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
# Recipe Bottle Foundry Verify - about, vouch, and batch_vouch operations (director credentials)

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBFV_SOURCED:-}" || buc_die "Module rbfv multiply sourced - check sourcing hierarchy"
ZRBFV_SOURCED=1

# Source shared Foundry Core module
source "${BASH_SOURCE[0]%/*}/rbfc_FoundryCore.sh"

######################################################################
# Internal Functions (zrbfv_*)

zrbfv_kindle() {
  test -z "${ZRBFV_KINDLED:-}" || buc_die "Module rbfv already kindled"

  buc_log_args 'Validate Foundry Core is kindled'
  zrbfc_sentinel

  buc_log_args 'Verify Director RBRA file'
  test -n "${RBDC_DIRECTOR_RBRA_FILE:-}" || buc_die "RBDC_DIRECTOR_RBRA_FILE not set"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}"   || buc_die "GCB service env file not found: ${RBDC_DIRECTOR_RBRA_FILE}"

  buc_log_args 'Define vouch operation file prefix'
  readonly ZRBFV_VOUCH_PREFIX="${BURD_TEMP_DIR}/rbfv_vouch_"

  buc_log_args 'Define about operation file prefix'
  readonly ZRBFV_ABOUT_PREFIX="${BURD_TEMP_DIR}/rbfv_about_"

  buc_log_args 'Define graft metadata operation file prefix'
  readonly ZRBFV_GRAFT_META_PREFIX="${BURD_TEMP_DIR}/rbfv_graft_meta_"

  readonly ZRBFV_KINDLED=1
}

zrbfv_sentinel() {
  zrbfc_sentinel
  test "${ZRBFV_KINDLED:-}" = "1" || buc_die "Module rbfv not kindled - call zrbfv_kindle first"
}

######################################################################
# Public Functions (rbfv_*)

rbfv_vouch_gate() {
  zrbfv_sentinel

  local -r z_vessel="${1:-}"
  local -r z_hallmark="${2:-}"

  test -n "${z_vessel}"       || buc_die "rbfv_vouch_gate: vessel required"
  test -n "${z_hallmark}" || buc_die "rbfv_vouch_gate: hallmark required"

  local -r z_registry_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_registry_api_base="https://${z_registry_host}/v2/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"

  local -r z_vouch_tag="${z_hallmark}${RBGC_ARK_SUFFIX_VOUCH}"
  buc_step "Vouch gate: checking ${z_vessel}:${z_vouch_tag}"

  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "rbfv_vouch_gate: failed to get Director OAuth token"

  local z_vouch_http_code
  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -o /dev/null \
    -w "%{http_code}" \
    "${z_registry_api_base}/${RBRR_CLOUD_PREFIX}${z_vessel}/manifests/${z_vouch_tag}" \
    > "${ZRBFC_SCRATCH_FILE}" \
    || buc_die "rbfv_vouch_gate: HEAD request failed for ${z_vessel}:${z_vouch_tag}"
  z_vouch_http_code=$(<"${ZRBFC_SCRATCH_FILE}")

  if test "${z_vouch_http_code}" != "200"; then
    buc_die "Hallmark not vouched: ${z_vessel}:${z_hallmark} (HTTP ${z_vouch_http_code} — refusing to use unvouched image)"
  fi

  buc_info "Vouch verified: ${z_vessel}:${z_vouch_tag}"
}

rbfv_about() {
  zrbfv_sentinel

  local -r z_hallmark="${2:-}"
  local -r z_conjure_build_id="${3:-}"  # Optional: conjure BUILD_ID for provenance

  buc_doc_brief "Assemble about metadata artifact for an existing hallmark image"
  buc_doc_param "vessel" "Vessel sigil or path to vessel directory"
  buc_doc_param "hallmark" "Full hallmark (e.g., c260305133650-r260305160530)"
  buc_doc_param "conjure_build_id" "(Optional) Cloud Build job ID from conjure"
  buc_doc_shown || return 0

  # Resolve vessel argument (sigil or path) and load
  zrbfc_resolve_vessel "${1:-}"
  local -r z_vessel_dir=$(<"${ZRBFC_VESSEL_RESOLVED_DIR_FILE}")
  test -n "${z_vessel_dir}" || buc_die "Empty resolved vessel path"
  zrbfc_load_vessel "${z_vessel_dir}"
  test -n "${z_hallmark}" || buc_die "Hallmark parameter required"

  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Gate: require -image exists
  buc_step "Gating on image artifact existence"
  local -r z_image_tag="${z_hallmark}${RBGC_ARK_SUFFIX_IMAGE}"
  local -r z_image_gate_status="${ZRBFV_ABOUT_PREFIX}image_status.txt"
  local -r z_image_gate_response="${ZRBFV_ABOUT_PREFIX}image_response.json"
  local -r z_image_gate_stderr="${ZRBFV_ABOUT_PREFIX}image_stderr.txt"

  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_image_gate_response}" \
    "${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${RBRV_SIGIL}/manifests/${z_image_tag}" \
    > "${z_image_gate_status}" 2>"${z_image_gate_stderr}" \
    || buc_die "HEAD request failed for -image artifact — see ${z_image_gate_stderr}"

  local -r z_image_http_code=$(<"${z_image_gate_status}")
  test -n "${z_image_http_code}" || buc_die "HTTP status code is empty for -image"
  test "${z_image_http_code}" = "200" \
    || buc_die "Image artifact not found (HTTP ${z_image_http_code}) — image must exist before about"

  buc_info "Image artifact confirmed: ${z_image_tag}"

  # Gate: warn if -about already exists (re-about is idempotent overwrite)
  local -r z_about_tag="${z_hallmark}${RBGC_ARK_SUFFIX_ABOUT}"
  local -r z_about_gate_status="${ZRBFV_ABOUT_PREFIX}about_status.txt"
  local -r z_about_gate_response="${ZRBFV_ABOUT_PREFIX}about_response.json"
  local -r z_about_gate_stderr="${ZRBFV_ABOUT_PREFIX}about_stderr.txt"

  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_about_gate_response}" \
    "${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${RBRV_SIGIL}/manifests/${z_about_tag}" \
    > "${z_about_gate_status}" 2>"${z_about_gate_stderr}" \
    || buc_die "HEAD request failed for -about artifact — see ${z_about_gate_stderr}"

  local -r z_about_http_code=$(<"${z_about_gate_status}")
  test -n "${z_about_http_code}" || buc_die "HTTP status code is empty for -about"
  if test "${z_about_http_code}" = "200"; then
    buc_warn "Re-about in progress: ${z_about_tag} already exists"
  fi

  # Submit about Cloud Build
  zrbfv_about_submit "${z_hallmark}" "${z_token}" "${z_conjure_build_id}"

  buc_success "About complete: ${RBRV_SIGIL}/${z_hallmark}"
  buc_info "About artifact: ${RBRV_SIGIL}:${z_about_tag}"
}

# Internal: submit combined about+vouch Cloud Build job for graft mode.
# Eliminates the orphan gap between standalone about and vouch by running
# both step sets in a single GCB submission.
# Args: vessel_dir hallmark
zrbfv_graft_metadata_submit() {
  zrbfv_sentinel

  local -r z_vessel_dir="$1"
  local -r z_hallmark="$2"

  # Load vessel (follows reload pattern used by rbfv_about/rbfv_vouch)
  zrbfc_load_vessel "${z_vessel_dir}"
  test -n "${z_hallmark}" || buc_die "Hallmark parameter required"

  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  buc_step "Constructing combined about+vouch Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBRR_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Gate: require -image exists (graft push must have completed)
  buc_step "Gating on image artifact existence"
  local -r z_image_tag="${z_hallmark}${RBGC_ARK_SUFFIX_IMAGE}"
  local -r z_image_gate_status="${ZRBFV_GRAFT_META_PREFIX}image_status.txt"
  local -r z_image_gate_response="${ZRBFV_GRAFT_META_PREFIX}image_response.json"
  local -r z_image_gate_stderr="${ZRBFV_GRAFT_META_PREFIX}image_stderr.txt"

  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_image_gate_response}" \
    "${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${RBRV_SIGIL}/manifests/${z_image_tag}" \
    > "${z_image_gate_status}" 2>"${z_image_gate_stderr}" \
    || buc_die "HEAD request failed for -image artifact — see ${z_image_gate_stderr}"

  local -r z_image_http_code=$(<"${z_image_gate_status}")
  test -n "${z_image_http_code}" || buc_die "HTTP status code is empty for -image"
  test "${z_image_http_code}" = "200" \
    || buc_die "Image artifact not found (HTTP ${z_image_http_code}) — graft push must complete before about+vouch"

  buc_info "Image artifact confirmed: ${z_image_tag}"

  # Git metadata (shared temp files, idempotent)
  zrbfc_ensure_git_metadata
  local z_git_commit=""
  z_git_commit=$(<"${ZRBFC_GIT_COMMIT_FILE}")
  local z_git_branch=""
  z_git_branch=$(<"${ZRBFC_GIT_BRANCH_FILE}")
  local z_git_repo=""
  z_git_repo=$(<"${ZRBFC_GIT_REPO_FILE}")

  # Graft-specific about substitution values
  local -r z_graft_source="${RBRV_GRAFT_IMAGE:-}"
  local z_dockerfile_content=""
  local -r z_dockerfile_max_bytes=4000
  if test -n "${RBRV_GRAFT_OPTIONAL_DOCKERFILE:-}" && test -f "${RBRV_GRAFT_OPTIONAL_DOCKERFILE}"; then
    local -r z_df_size_file="${ZRBFV_GRAFT_META_PREFIX}df_size.txt"
    wc -c < "${RBRV_GRAFT_OPTIONAL_DOCKERFILE}" > "${z_df_size_file}" \
      || buc_die "Failed to measure Dockerfile size"
    local z_df_size=""
    z_df_size=$(<"${z_df_size_file}")
    z_df_size="${z_df_size// /}"
    if test "${z_df_size}" -le "${z_dockerfile_max_bytes}"; then
      z_dockerfile_content=$(<"${RBRV_GRAFT_OPTIONAL_DOCKERFILE}")
    else
      buc_warn "Dockerfile exceeds 4KB substitution limit (${z_df_size} bytes) — recipe.txt omitted"
    fi
  fi

  # === Assemble about steps ===
  local -r z_about_steps_file="${ZRBFV_GRAFT_META_PREFIX}about_steps.json"
  zrbfc_assemble_about_steps "${z_about_steps_file}" "${ZRBFV_GRAFT_META_PREFIX}about_"

  # === Resolve base image provenance (for vouch summary) ===
  local -r z_vi_gar_prefix="${z_gar_host}/${z_gar_path}/${RBRR_CLOUD_PREFIX}${RBRV_SIGIL}"
  local z_vi_ref_1="" z_vi_ref_2="" z_vi_ref_3=""
  local z_vi_prov_1="" z_vi_prov_2="" z_vi_prov_3=""
  local z_vi_n="" z_vi_origin_var="" z_vi_anchor_var="" z_vi_origin="" z_vi_anchor=""
  for z_vi_n in 1 2 3; do
    z_vi_origin_var="RBRV_IMAGE_${z_vi_n}_ORIGIN"
    z_vi_anchor_var="RBRV_IMAGE_${z_vi_n}_ANCHOR"
    z_vi_origin="${!z_vi_origin_var:-}"
    z_vi_anchor="${!z_vi_anchor_var:-}"
    test -n "${z_vi_origin}" || continue
    local z_vi_ref="" z_vi_prov=""
    if test -n "${z_vi_anchor}"; then
      z_vi_ref="${z_vi_gar_prefix}:${z_vi_anchor}"
      z_vi_prov="anchored"
    else
      z_vi_ref="${z_vi_origin}"
      z_vi_prov="pass-through"
    fi
    case "${z_vi_n}" in
      1) z_vi_ref_1="${z_vi_ref}"; z_vi_prov_1="${z_vi_prov}" ;;
      2) z_vi_ref_2="${z_vi_ref}"; z_vi_prov_2="${z_vi_prov}" ;;
      3) z_vi_ref_3="${z_vi_ref}"; z_vi_prov_3="${z_vi_prov}" ;;
    esac
  done

  # === Assemble vouch steps ===
  local -r z_vouch_steps_file="${ZRBFV_GRAFT_META_PREFIX}vouch_steps.json"
  zrbfc_assemble_vouch_steps "${z_vouch_steps_file}" "${ZRBFV_GRAFT_META_PREFIX}vouch_"

  # === Combine: about steps + vouch steps ===
  local -r z_combined_steps="${ZRBFV_GRAFT_META_PREFIX}combined_steps.json"
  jq -s '.[0] + .[1]' "${z_about_steps_file}" "${z_vouch_steps_file}" \
    > "${z_combined_steps}" || buc_die "Failed to combine about and vouch steps"

  # Compose Build resource JSON with both _RBGA_ and _RBGV_ substitutions
  buc_log_args "Composing combined about+vouch Build resource JSON"
  local -r z_build_file="${ZRBFV_GRAFT_META_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps       "${z_combined_steps}" \
    --arg zjq_sa                "${z_mason_sa}" \
    --arg zjq_gar_host          "${z_gar_host}" \
    --arg zjq_gar_path          "${z_gar_path}" \
    --arg zjq_vessel            "${RBRR_CLOUD_PREFIX}${RBRV_SIGIL}" \
    --arg zjq_hallmark      "${z_hallmark}" \
    --arg zjq_git_commit        "${z_git_commit}" \
    --arg zjq_git_branch        "${z_git_branch}" \
    --arg zjq_git_repo          "${z_git_repo}" \
    --arg zjq_graft_source      "${z_graft_source}" \
    --arg zjq_dockerfile        "${z_dockerfile_content}" \
    --arg zjq_ark_suffix_image  "${RBGC_ARK_SUFFIX_IMAGE}" \
    --arg zjq_ark_suffix_about  "${RBGC_ARK_SUFFIX_ABOUT}" \
    --arg zjq_ark_suffix_vouch  "${RBGC_ARK_SUFFIX_VOUCH}" \
    --arg zjq_ark_suffix_diags  "${RBGC_ARK_SUFFIX_DIAGS}" \
    --arg zjq_vouches_package   "${RBRR_CLOUD_PREFIX}${RBGC_VOUCHES_PACKAGE}" \
    --arg zjq_vi_ref_1          "${z_vi_ref_1}" \
    --arg zjq_vi_prov_1         "${z_vi_prov_1}" \
    --arg zjq_vi_ref_2          "${z_vi_ref_2}" \
    --arg zjq_vi_prov_2         "${z_vi_prov_2}" \
    --arg zjq_vi_ref_3          "${z_vi_ref_3}" \
    --arg zjq_vi_prov_3         "${z_vi_prov_3}" \
    --arg zjq_pool              "${RBDC_POOL_AIRGAP}" \
    --arg zjq_timeout           "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGA_GAR_HOST:              $zjq_gar_host,
        _RBGA_GAR_PATH:              $zjq_gar_path,
        _RBGA_VESSEL:                $zjq_vessel,
        _RBGA_HALLMARK:          $zjq_hallmark,
        _RBGA_VESSEL_MODE:           "graft",
        _RBGA_GIT_COMMIT:            $zjq_git_commit,
        _RBGA_GIT_BRANCH:            $zjq_git_branch,
        _RBGA_GIT_REPO:              $zjq_git_repo,
        _RBGA_BUILD_ID:              "",
        _RBGA_INSCRIBE_TIMESTAMP:    "",
        _RBGA_BIND_SOURCE:           "",
        _RBGA_GRAFT_SOURCE:          $zjq_graft_source,
        _RBGA_DOCKERFILE_CONTENT:    $zjq_dockerfile,
        _RBGA_ARK_SUFFIX_IMAGE:      $zjq_ark_suffix_image,
        _RBGA_ARK_SUFFIX_ABOUT:      $zjq_ark_suffix_about,
        _RBGA_ARK_SUFFIX_DIAGS:      $zjq_ark_suffix_diags,
        _RBGV_GAR_HOST:              $zjq_gar_host,
        _RBGV_GAR_PATH:              $zjq_gar_path,
        _RBGV_VESSEL:                $zjq_vessel,
        _RBGV_HALLMARK:          $zjq_hallmark,
        _RBGV_VESSEL_MODE:           "graft",
        _RBGV_BIND_SOURCE:           "",
        _RBGV_GRAFT_SOURCE:          $zjq_graft_source,
        _RBGV_ARK_SUFFIX_IMAGE:      $zjq_ark_suffix_image,
        _RBGV_ARK_SUFFIX_VOUCH:      $zjq_ark_suffix_vouch,
        _RBGV_VOUCHES_PACKAGE:       $zjq_vouches_package,
        _RBGV_IMAGE_1:               $zjq_vi_ref_1,
        _RBGV_IMAGE_1_PROVENANCE:    $zjq_vi_prov_1,
        _RBGV_IMAGE_2:               $zjq_vi_ref_2,
        _RBGV_IMAGE_2_PROVENANCE:    $zjq_vi_prov_2,
        _RBGV_IMAGE_3:               $zjq_vi_ref_3,
        _RBGV_IMAGE_3_PROVENANCE:    $zjq_vi_prov_3
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_build_file}" \
    || buc_die "Failed to compose combined about+vouch build JSON"

  buc_log_args "Combined about+vouch build JSON: ${z_build_file}"

  buc_step "Submitting combined about+vouch Cloud Build"
  rbgu_http_json "POST" "${ZRBFC_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "graft_meta_build_create" "${z_build_file}"
  rbgu_http_require_ok "Combined about+vouch build submission" "graft_meta_build_create"

  local z_build_id=""
  z_build_id=$(rbgu_json_field_capture "graft_meta_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBFC_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBFC_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Combined about+vouch build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbfc_wait_build_completion 100 "About+Vouch"  # ~8 minutes at 5s intervals

  buc_success "About+Vouch complete: ${RBRV_SIGIL}/${z_hallmark}"
  local -r z_about_tag="${z_hallmark}${RBGC_ARK_SUFFIX_ABOUT}"
  local -r z_vouch_tag="${z_hallmark}${RBGC_ARK_SUFFIX_VOUCH}"
  buc_info "About artifact: ${RBRV_SIGIL}:${z_about_tag}"
  buc_info "Vouch artifact: ${RBRV_SIGIL}:${z_vouch_tag}"
}

# Internal: submit about Cloud Build job and wait for completion
zrbfv_about_submit() {
  zrbfv_sentinel

  local -r z_hallmark="$1"
  local -r z_token="$2"
  local -r z_conjure_build_id="${3:-}"

  buc_step "Constructing about Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBRR_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Determine mode-specific substitution values
  local z_vessel_mode="${RBRV_VESSEL_MODE}"
  local z_bind_source=""
  local z_graft_source=""
  local z_inscribe_ts=""
  local z_dockerfile_content=""
  # Cloud Build substitution values are limited to 4096 bytes. We use 4000 as a
  # conservative guard to account for encoding overhead and avoid edge-case failures.
  local -r z_dockerfile_max_bytes=4000
  local -r z_df_size_file="${ZRBFV_ABOUT_PREFIX}df_size.txt"

  case "${z_vessel_mode}" in
    conjure)
      # Extract inscribe timestamp from hallmark (e.g., c260305133650 from c260305133650-r260305160530)
      z_inscribe_ts="${z_hallmark%%-r*}"
      # Read Dockerfile content for recipe.txt
      if test -f "${RBRV_CONJURE_DOCKERFILE:-}"; then
        wc -c < "${RBRV_CONJURE_DOCKERFILE}" > "${z_df_size_file}" \
          || buc_die "Failed to measure Dockerfile size"
        local z_df_size=""
        z_df_size=$(<"${z_df_size_file}")
        z_df_size="${z_df_size// /}"  # Strip spaces (wc output varies by platform)
        if test "${z_df_size}" -le "${z_dockerfile_max_bytes}"; then
          z_dockerfile_content=$(<"${RBRV_CONJURE_DOCKERFILE}")
        else
          buc_warn "Dockerfile exceeds 4KB substitution limit (${z_df_size} bytes) — recipe.txt omitted"
        fi
      fi
      ;;
    bind)
      z_bind_source="${RBRV_BIND_IMAGE:-}"
      if test -n "${RBRV_BIND_OPTIONAL_DOCKERFILE:-}" && test -f "${RBRV_BIND_OPTIONAL_DOCKERFILE}"; then
        wc -c < "${RBRV_BIND_OPTIONAL_DOCKERFILE}" > "${z_df_size_file}" \
          || buc_die "Failed to measure Dockerfile size"
        local z_df_size=""
        z_df_size=$(<"${z_df_size_file}")
        z_df_size="${z_df_size// /}"  # Strip spaces (wc output varies by platform)
        if test "${z_df_size}" -le "${z_dockerfile_max_bytes}"; then
          z_dockerfile_content=$(<"${RBRV_BIND_OPTIONAL_DOCKERFILE}")
        else
          buc_warn "Dockerfile exceeds 4KB substitution limit (${z_df_size} bytes) — recipe.txt omitted"
        fi
      fi
      ;;
    graft)
      z_graft_source="${RBRV_GRAFT_IMAGE:-}"
      if test -n "${RBRV_GRAFT_OPTIONAL_DOCKERFILE:-}" && test -f "${RBRV_GRAFT_OPTIONAL_DOCKERFILE}"; then
        wc -c < "${RBRV_GRAFT_OPTIONAL_DOCKERFILE}" > "${z_df_size_file}" \
          || buc_die "Failed to measure Dockerfile size"
        local z_df_size=""
        z_df_size=$(<"${z_df_size_file}")
        z_df_size="${z_df_size// /}"  # Strip spaces (wc output varies by platform)
        if test "${z_df_size}" -le "${z_dockerfile_max_bytes}"; then
          z_dockerfile_content=$(<"${RBRV_GRAFT_OPTIONAL_DOCKERFILE}")
        else
          buc_warn "Dockerfile exceeds 4KB substitution limit (${z_df_size} bytes) — recipe.txt omitted"
        fi
      fi
      ;;
    *)
      buc_die "Unknown vessel mode: ${z_vessel_mode}"
      ;;
  esac

  # Git metadata (shared temp files, idempotent)
  zrbfc_ensure_git_metadata
  local z_git_commit=""
  z_git_commit=$(<"${ZRBFC_GIT_COMMIT_FILE}")
  local z_git_branch=""
  z_git_branch=$(<"${ZRBFC_GIT_BRANCH_FILE}")
  local z_git_repo=""
  z_git_repo=$(<"${ZRBFC_GIT_REPO_FILE}")

  # Assemble about steps via shared helper
  local -r z_about_steps_accumulator="${ZRBFV_ABOUT_PREFIX}steps.json"
  zrbfc_assemble_about_steps "${z_about_steps_accumulator}" "${ZRBFV_ABOUT_PREFIX}"

  buc_log_args "Composing about Build resource JSON"
  local -r z_about_build_file="${ZRBFV_ABOUT_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps  "${z_about_steps_accumulator}" \
    --arg zjq_sa           "${z_mason_sa}" \
    --arg zjq_gar_host     "${z_gar_host}" \
    --arg zjq_gar_path     "${z_gar_path}" \
    --arg zjq_vessel       "${RBRR_CLOUD_PREFIX}${RBRV_SIGIL}" \
    --arg zjq_hallmark "${z_hallmark}" \
    --arg zjq_vessel_mode  "${z_vessel_mode}" \
    --arg zjq_git_commit   "${z_git_commit}" \
    --arg zjq_git_branch   "${z_git_branch}" \
    --arg zjq_git_repo     "${z_git_repo}" \
    --arg zjq_build_id     "${z_conjure_build_id}" \
    --arg zjq_inscribe_ts  "${z_inscribe_ts}" \
    --arg zjq_bind_source  "${z_bind_source}" \
    --arg zjq_graft_source "${z_graft_source}" \
    --arg zjq_dockerfile   "${z_dockerfile_content}" \
    --arg zjq_ark_suffix_image "${RBGC_ARK_SUFFIX_IMAGE}" \
    --arg zjq_ark_suffix_about "${RBGC_ARK_SUFFIX_ABOUT}" \
    --arg zjq_ark_suffix_diags "${RBGC_ARK_SUFFIX_DIAGS}" \
    --arg zjq_pool         "${RBDC_POOL_AIRGAP}" \
    --arg zjq_timeout      "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGA_GAR_HOST:              $zjq_gar_host,
        _RBGA_GAR_PATH:              $zjq_gar_path,
        _RBGA_VESSEL:                $zjq_vessel,
        _RBGA_HALLMARK:          $zjq_hallmark,
        _RBGA_VESSEL_MODE:           $zjq_vessel_mode,
        _RBGA_GIT_COMMIT:            $zjq_git_commit,
        _RBGA_GIT_BRANCH:            $zjq_git_branch,
        _RBGA_GIT_REPO:              $zjq_git_repo,
        _RBGA_BUILD_ID:              $zjq_build_id,
        _RBGA_INSCRIBE_TIMESTAMP:    $zjq_inscribe_ts,
        _RBGA_BIND_SOURCE:           $zjq_bind_source,
        _RBGA_GRAFT_SOURCE:          $zjq_graft_source,
        _RBGA_DOCKERFILE_CONTENT:    $zjq_dockerfile,
        _RBGA_ARK_SUFFIX_IMAGE:      $zjq_ark_suffix_image,
        _RBGA_ARK_SUFFIX_ABOUT:      $zjq_ark_suffix_about,
        _RBGA_ARK_SUFFIX_DIAGS:      $zjq_ark_suffix_diags
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_about_build_file}" \
    || buc_die "Failed to compose about build JSON"

  buc_log_args "About build JSON: ${z_about_build_file}"

  buc_step "Submitting about Cloud Build"
  rbgu_http_json "POST" "${ZRBFC_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "about_build_create" "${z_about_build_file}"
  rbgu_http_require_ok "About build submission" "about_build_create"

  local z_build_id=""
  z_build_id=$(rbgu_json_field_capture "about_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBFC_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBFC_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "About build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbfc_wait_build_completion 50 "About"  # ~4 minutes at 5s intervals (private pool)
}

######################################################################
# Vouch

rbfv_vouch() {
  zrbfv_sentinel

  local -r z_vessel_dir="${1:-}"
  local -r z_hallmark="${2:-}"

  buc_doc_brief "Vouch for an ark by mode-aware verification in Cloud Build"
  buc_doc_param "vessel_dir" "Path to vessel directory containing rbrv.env"
  buc_doc_param "hallmark" "Full hallmark (e.g., c260305133650-r260305160530)"
  buc_doc_shown || return 0

  if test -z "${z_vessel_dir}"; then
    local z_sigils
    z_sigils=$(rbrv_list_capture) || buc_die "No vessels found"
    buc_step "Available vessels:"
    local z_sigil=""
    for z_sigil in ${z_sigils}; do
      buc_bare "        ${RBRR_VESSEL_DIR}/${z_sigil}"
    done
    buc_die "Vessel directory required"
  fi

  zrbfc_load_vessel "${z_vessel_dir}"
  test -n "${z_hallmark}" || buc_die "Hallmark parameter required"

  # Resolve tool images from reliquary (vouch steps use tool images)
  zrbfc_resolve_tool_images

  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Gate: require -about exists (about must complete before vouch)
  buc_step "Gating on about artifact existence"
  local -r z_about_tag="${z_hallmark}${RBGC_ARK_SUFFIX_ABOUT}"
  local -r z_about_gate_status="${ZRBFV_VOUCH_PREFIX}about_status.txt"
  local -r z_about_gate_response="${ZRBFV_VOUCH_PREFIX}about_response.json"
  local -r z_about_gate_stderr="${ZRBFV_VOUCH_PREFIX}about_stderr.txt"

  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_about_gate_response}" \
    "${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${RBRV_SIGIL}/manifests/${z_about_tag}" \
    > "${z_about_gate_status}" 2>"${z_about_gate_stderr}" \
    || buc_die "HEAD request failed for -about artifact — see ${z_about_gate_stderr}"

  local -r z_about_http_code=$(<"${z_about_gate_status}")
  test -n "${z_about_http_code}" || buc_die "HTTP status code is empty for -about"
  test "${z_about_http_code}" = "200" \
    || buc_die "About artifact not found (HTTP ${z_about_http_code}) — about must complete before vouch"

  buc_info "About artifact confirmed: ${z_about_tag}"

  # Gate: warn if -vouch already exists (re-vouch)
  local -r z_vouch_tag="${z_hallmark}${RBGC_ARK_SUFFIX_VOUCH}"
  local -r z_vouch_gate_status="${ZRBFV_VOUCH_PREFIX}vouch_status.txt"
  local -r z_vouch_gate_response="${ZRBFV_VOUCH_PREFIX}vouch_response.json"
  local -r z_vouch_gate_stderr="${ZRBFV_VOUCH_PREFIX}vouch_stderr.txt"

  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_vouch_gate_response}" \
    "${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${RBRV_SIGIL}/manifests/${z_vouch_tag}" \
    > "${z_vouch_gate_status}" 2>"${z_vouch_gate_stderr}" \
    || buc_die "HEAD request failed for -vouch artifact — see ${z_vouch_gate_stderr}"

  local -r z_vouch_http_code=$(<"${z_vouch_gate_status}")
  test -n "${z_vouch_http_code}" || buc_die "HTTP status code is empty for -vouch"
  if test "${z_vouch_http_code}" = "200"; then
    buc_warn "Re-vouch in progress: ${z_vouch_tag} already exists"
  fi

  # All modes use Cloud Build for vouch (mode-aware verification inside the build)
  zrbfv_vouch_submit "${z_hallmark}" "${z_vouch_tag}" "${z_token}"

  buc_success "Vouch complete: ${RBRV_SIGIL}/${z_hallmark}"
  buc_info "Vouch artifact: ${RBRV_SIGIL}:${z_vouch_tag}"
}

# Internal: Submit vouch Cloud Build job (mode-aware verification)
# All vessel modes use Cloud Build. The build scripts branch on _RBGV_VESSEL_MODE:
#   conjure: DSSE envelope signature verification (Python 3 + openssl)
#   bind: digest-pin comparison against upstream reference
#   graft: GRAFTED stamp (no verification)
zrbfv_vouch_submit() {
  zrbfv_sentinel

  local -r z_hallmark="$1"
  local -r z_vouch_tag="$2"
  local -r z_token="$3"

  buc_step "Constructing vouch Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBRR_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Mode-specific substitution values (empty strings for non-applicable modes)
  local z_bind_source=""
  local z_graft_source=""

  case "${RBRV_VESSEL_MODE}" in
    conjure) : ;;  # DSSE verification uses embedded keys, no extra substitutions
    bind)    z_bind_source="${RBRV_BIND_IMAGE:-}" ;;
    graft)   z_graft_source="${RBRV_GRAFT_IMAGE:-}" ;;
    *)       buc_die "Unknown vessel mode: ${RBRV_VESSEL_MODE}" ;;
  esac

  # Resolve base image provenance (for vouch summary recording)
  local -r z_vi_gar_prefix="${z_gar_host}/${z_gar_path}/${RBRR_CLOUD_PREFIX}${RBRV_SIGIL}"
  local z_vi_ref_1="" z_vi_ref_2="" z_vi_ref_3=""
  local z_vi_prov_1="" z_vi_prov_2="" z_vi_prov_3=""
  local z_vi_n="" z_vi_origin_var="" z_vi_anchor_var="" z_vi_origin="" z_vi_anchor=""
  for z_vi_n in 1 2 3; do
    z_vi_origin_var="RBRV_IMAGE_${z_vi_n}_ORIGIN"
    z_vi_anchor_var="RBRV_IMAGE_${z_vi_n}_ANCHOR"
    z_vi_origin="${!z_vi_origin_var:-}"
    z_vi_anchor="${!z_vi_anchor_var:-}"
    test -n "${z_vi_origin}" || continue
    local z_vi_ref="" z_vi_prov=""
    if test -n "${z_vi_anchor}"; then
      z_vi_ref="${z_vi_gar_prefix}:${z_vi_anchor}"
      z_vi_prov="anchored"
    else
      z_vi_ref="${z_vi_origin}"
      z_vi_prov="pass-through"
    fi
    case "${z_vi_n}" in
      1) z_vi_ref_1="${z_vi_ref}"; z_vi_prov_1="${z_vi_prov}" ;;
      2) z_vi_ref_2="${z_vi_ref}"; z_vi_prov_2="${z_vi_prov}" ;;
      3) z_vi_ref_3="${z_vi_ref}"; z_vi_prov_3="${z_vi_prov}" ;;
    esac
  done

  # Assemble vouch steps via shared helper
  local -r z_vouch_steps_accumulator="${ZRBFV_VOUCH_PREFIX}steps.json"
  zrbfc_assemble_vouch_steps "${z_vouch_steps_accumulator}" "${ZRBFV_VOUCH_PREFIX}"

  buc_log_args "Composing vouch Build resource JSON"
  local -r z_vouch_build_file="${ZRBFV_VOUCH_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps       "${z_vouch_steps_accumulator}" \
    --arg zjq_sa                "${z_mason_sa}" \
    --arg zjq_gar_host          "${z_gar_host}" \
    --arg zjq_gar_path          "${z_gar_path}" \
    --arg zjq_vessel            "${RBRR_CLOUD_PREFIX}${RBRV_SIGIL}" \
    --arg zjq_hallmark      "${z_hallmark}" \
    --arg zjq_vessel_mode       "${RBRV_VESSEL_MODE}" \
    --arg zjq_bind_source       "${z_bind_source}" \
    --arg zjq_graft_source      "${z_graft_source}" \
    --arg zjq_ark_suffix_image  "${RBGC_ARK_SUFFIX_IMAGE}" \
    --arg zjq_ark_suffix_attest "${RBGC_ARK_SUFFIX_ATTEST}" \
    --arg zjq_ark_suffix_vouch  "${RBGC_ARK_SUFFIX_VOUCH}" \
    --arg zjq_vouches_package   "${RBRR_CLOUD_PREFIX}${RBGC_VOUCHES_PACKAGE}" \
    --arg zjq_vi_ref_1          "${z_vi_ref_1}" \
    --arg zjq_vi_prov_1         "${z_vi_prov_1}" \
    --arg zjq_vi_ref_2          "${z_vi_ref_2}" \
    --arg zjq_vi_prov_2         "${z_vi_prov_2}" \
    --arg zjq_vi_ref_3          "${z_vi_ref_3}" \
    --arg zjq_vi_prov_3         "${z_vi_prov_3}" \
    --arg zjq_pool              "${RBDC_POOL_AIRGAP}" \
    --arg zjq_timeout           "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGV_GAR_HOST:          $zjq_gar_host,
        _RBGV_GAR_PATH:          $zjq_gar_path,
        _RBGV_VESSEL:            $zjq_vessel,
        _RBGV_HALLMARK:      $zjq_hallmark,
        _RBGV_VESSEL_MODE:       $zjq_vessel_mode,
        _RBGV_BIND_SOURCE:       $zjq_bind_source,
        _RBGV_GRAFT_SOURCE:      $zjq_graft_source,
        _RBGV_ARK_SUFFIX_IMAGE:  $zjq_ark_suffix_image,
        _RBGV_ARK_SUFFIX_ATTEST: $zjq_ark_suffix_attest,
        _RBGV_ARK_SUFFIX_VOUCH:  $zjq_ark_suffix_vouch,
        _RBGV_VOUCHES_PACKAGE:   $zjq_vouches_package,
        _RBGV_IMAGE_1:           $zjq_vi_ref_1,
        _RBGV_IMAGE_1_PROVENANCE: $zjq_vi_prov_1,
        _RBGV_IMAGE_2:           $zjq_vi_ref_2,
        _RBGV_IMAGE_2_PROVENANCE: $zjq_vi_prov_2,
        _RBGV_IMAGE_3:           $zjq_vi_ref_3,
        _RBGV_IMAGE_3_PROVENANCE: $zjq_vi_prov_3
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_vouch_build_file}" \
    || buc_die "Failed to compose vouch build JSON"

  buc_log_args "Vouch build JSON: ${z_vouch_build_file}"

  buc_step "Submitting vouch Cloud Build"
  rbgu_http_json "POST" "${ZRBFC_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "vouch_build_create" "${z_vouch_build_file}"
  rbgu_http_require_ok "Vouch build submission" "vouch_build_create"

  local z_build_id=""
  z_build_id=$(rbgu_json_field_capture "vouch_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBFC_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBFC_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Vouch build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbfc_wait_build_completion 50 "Vouch"  # ~4 minutes at 5s intervals (private pool is slower)
}

######################################################################
# Batch Vouch

rbfv_batch_vouch() {
  zrbfv_sentinel

  buc_doc_brief "Vouch all pending hallmarks across all vessels"
  buc_doc_shown || return 0

  buc_step "Enumerating vessels"
  local z_sigils
  z_sigils=$(rbrv_list_capture) || buc_die "No vessels found"

  buc_step "Fetching OAuth token (Director)"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  local z_vouched_count=0
  local z_already_count=0
  local z_failed_count=0

  local z_sigil=""
  for z_sigil in ${z_sigils}; do

    buc_step "Scanning ${z_sigil} for pending hallmarks"
    local z_tags_file="${BURD_TEMP_DIR}/rbfv_bv_${z_sigil}_tags.json"
    local z_stderr_file="${BURD_TEMP_DIR}/rbfv_bv_${z_sigil}_stderr.txt"
    curl -sL \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
      -H "Authorization: Bearer ${z_token}" \
      "${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${z_sigil}/tags/list" \
      > "${z_tags_file}" 2>"${z_stderr_file}" \
      || buc_die "Failed to fetch tags for ${z_sigil} — see ${z_stderr_file}"

    if jq -e '.errors' "${z_tags_file}" >/dev/null 2>&1; then
      local z_err
      jq -r '.errors[0].message // "Unknown error"' "${z_tags_file}" > "${ZRBFC_SCRATCH_FILE}" \
        || buc_die "Failed to extract error message from registry response for ${z_sigil}"
      z_err=$(<"${ZRBFC_SCRATCH_FILE}")
      buc_die "Registry API error for ${z_sigil}: ${z_err}"
    fi

    local z_all_tags_file="${BURD_TEMP_DIR}/rbfv_bv_${z_sigil}_all_tags.txt"
    jq -r '.tags[]? // empty' "${z_tags_file}" > "${z_all_tags_file}" \
      || buc_die "Failed to extract tags for ${z_sigil}"

    # Classify: find hallmarks with -about but no -vouch
    local z_about_file="${BURD_TEMP_DIR}/rbfv_bv_${z_sigil}_has_about.txt"
    local z_vouch_file="${BURD_TEMP_DIR}/rbfv_bv_${z_sigil}_has_vouch.txt"
    : > "${z_about_file}"
    : > "${z_vouch_file}"

    while IFS= read -r z_tag || test -n "${z_tag}"; do
      local z_consec=""
      if [[ "${z_tag}" =~ ^([cbg][0-9]{12}-r[0-9]{12}) ]]; then
        z_consec="${BASH_REMATCH[1]}"
      else
        continue
      fi
      case "${z_tag}" in
        *"${RBGC_ARK_SUFFIX_VOUCH}")
          echo "${z_consec}" >> "${z_vouch_file}"
          ;;
        *"${RBGC_ARK_SUFFIX_ABOUT}")
          echo "${z_consec}" >> "${z_about_file}"
          ;;
      esac
    done < "${z_all_tags_file}"

    # Find pending: has about, no vouch
    local z_pending_file="${BURD_TEMP_DIR}/rbfv_bv_${z_sigil}_pending.txt"
    sort -u "${z_about_file}" > "${z_about_file}.sorted" \
      || buc_die "Failed to sort about file for ${z_sigil}"
    sort -u "${z_vouch_file}" > "${z_vouch_file}.sorted" \
      || buc_die "Failed to sort vouch file for ${z_sigil}"
    comm -23 "${z_about_file}.sorted" "${z_vouch_file}.sorted" > "${z_pending_file}" \
      || buc_die "Failed to compute pending hallmarks for ${z_sigil}"

    # Count already vouched for this vessel
    local z_count_file="${BURD_TEMP_DIR}/rbfv_bv_${z_sigil}_vouch_count.txt"
    wc -l < "${z_vouch_file}.sorted" > "${z_count_file}" \
      || buc_die "Failed to count vouched hallmarks for ${z_sigil}"
    local z_vessel_already=0
    z_vessel_already=$(<"${z_count_file}")
    z_vessel_already="${z_vessel_already// /}"
    z_already_count=$((z_already_count + z_vessel_already))

    if ! test -s "${z_pending_file}"; then
      buc_info "No pending hallmarks for ${z_sigil}"
      continue
    fi

    # Load pending hallmarks into array (load-then-iterate)
    local z_pending_items=()
    while IFS= read -r z_pline || test -n "${z_pline}"; do
      z_pending_items+=("${z_pline}")
    done < "${z_pending_file}"

    local z_vessel_dir="${RBRR_VESSEL_DIR}/${z_sigil}"
    local z_pi=""
    for z_pi in "${!z_pending_items[@]}"; do
      local z_hallmark="${z_pending_items[$z_pi]}"
      test -n "${z_hallmark}" || continue

      buc_step "Vouching ${z_sigil}/${z_hallmark}"

      # Run vouch in isolation subshell — buc_die inside kills only the subshell
      local z_vouch_status=0
      (
        rbfv_vouch "${z_vessel_dir}" "${z_hallmark}" \
          || buc_die "rbfv_vouch failed for ${z_sigil}/${z_hallmark}"
      ) || z_vouch_status=$?

      if test "${z_vouch_status}" = "0"; then
        z_vouched_count=$((z_vouched_count + 1))
      else
        buc_warn "Vouch failed for ${z_sigil}/${z_hallmark} (exit ${z_vouch_status}) — skipping"
        z_failed_count=$((z_failed_count + 1))
      fi
    done
  done

  echo ""
  buc_step "Batch vouch summary"
  buc_info "  Vouched:        ${z_vouched_count}"
  buc_info "  Already vouched: ${z_already_count}"
  buc_info "  Failed/skipped:  ${z_failed_count}"

  if test "${z_failed_count}" -gt 0; then
    buc_warn "Some hallmarks failed — older builds may lack SLSA provenance"
  fi

  buc_success "Batch vouch complete"
}

# eof
