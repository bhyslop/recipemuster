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
# Recipe Bottle Foundry Verify - graft metadata body (guard-free cluster,
# sourced by rbfv0_verify): zrbfv_graft_metadata_submit — the combined
# about+vouch Cloud Build submission graft mode takes, closing the orphan gap
# a standalone about followed by a standalone vouch would leave. Nothing
# inside rbfv calls it: its one caller is rbfd's ordain, reaching across the
# module boundary. Reads the entry's ZRBFV_GRAFT_META_PREFIX kindle constant.

set -euo pipefail

######################################################################
# Internal Functions (zrbfv_*)

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
  test -n "${z_hallmark}" || buc_die_now "Hallmark parameter required"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rba_token_capture "${RBCC_mantle_director}") \
    || buc_die_now "Failed to get Director OAuth token"

  buc_step "Constructing combined about+vouch Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBDC_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Gate: require image exists (graft push must have completed)
  buc_step "Gating on image artifact existence"
  local -r z_hallmark_subtree="${RBGL_HALLMARKS_ROOT}/${z_hallmark}"
  local -r z_image_gate_status="${ZRBFV_GRAFT_META_PREFIX}image_status.txt"
  local -r z_image_gate_response="${ZRBFV_GRAFT_META_PREFIX}image_response.json"
  local -r z_image_gate_stderr="${ZRBFV_GRAFT_META_PREFIX}image_stderr.txt"

  local z_curl_status=0
  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_image_gate_response}" \
    "${ZRBFC_REGISTRY_API_BASE}/${z_hallmark_subtree}/${RBGC_ARK_BASENAME_IMAGE}/manifests/${z_hallmark}" \
    > "${z_image_gate_status}" 2>"${z_image_gate_stderr}" \
    || z_curl_status=$?
  test "${z_curl_status}" -eq 0 \
    || buc_die_now "HEAD request failed for image artifact (curl exit ${z_curl_status}) — see ${z_image_gate_stderr}"

  local -r z_image_http_code=$(<"${z_image_gate_status}")
  test -n "${z_image_http_code}" || buc_die_now "HTTP status code is empty for image"
  test "${z_image_http_code}" = "200" \
    || buc_die_now "Image artifact not found (HTTP ${z_image_http_code}) — graft push must complete before about+vouch"

  buc_info "Image artifact confirmed: ${z_hallmark_subtree}/${RBGC_ARK_BASENAME_IMAGE}:${z_hallmark}"

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
    z_dockerfile_content=$(<"${RBRV_GRAFT_OPTIONAL_DOCKERFILE}")
    if test "${#z_dockerfile_content}" -gt "${z_dockerfile_max_bytes}"; then
      buc_warn "Dockerfile exceeds 4KB substitution limit (${#z_dockerfile_content} bytes) — recipe.txt omitted"
      z_dockerfile_content=""
    fi
  fi

  # === Assemble about steps ===
  local -r z_about_steps_file="${ZRBFV_GRAFT_META_PREFIX}about_steps.json"
  zrbfc_assemble_about_steps "${z_about_steps_file}" "${ZRBFV_GRAFT_META_PREFIX}about_"

  # === Resolve base image provenance (for vouch summary) ===
  # ANCHOR carries a locator (package-path:tag); cloud prefix applied at use-site.
  local -r z_vi_gar_repo_base="${z_gar_host}/${z_gar_path}"
  local z_vi_ref_1="" z_vi_ref_2="" z_vi_ref_3=""
  local z_vi_prov_1="" z_vi_prov_2="" z_vi_prov_3=""
  local z_vi_n="" z_vi_origin_var="" z_vi_anchor_var="" z_vi_origin="" z_vi_anchor=""
  local z_vi_pkg_path=""
  local z_vi_tag=""
  for z_vi_n in 1 2 3; do
    z_vi_origin_var="RBRV_IMAGE_${z_vi_n}_ORIGIN"
    z_vi_anchor_var="RBRV_IMAGE_${z_vi_n}_ANCHOR"
    z_vi_origin="${!z_vi_origin_var:-}"
    z_vi_anchor="${!z_vi_anchor_var:-}"
    test -n "${z_vi_origin}" || continue
    local z_vi_ref="" z_vi_prov=""
    if test -n "${z_vi_anchor}"; then
      case "${z_vi_anchor}" in
        *:*) : ;;
        *)   buc_die_now "Invalid ${z_vi_anchor_var} locator format (expected package-path:tag): ${z_vi_anchor}" ;;
      esac
      z_vi_pkg_path="${z_vi_anchor%:*}"
      z_vi_tag="${z_vi_anchor##*:}"
      test -n "${z_vi_pkg_path}" || buc_die_now "Package path is empty in ${z_vi_anchor_var}: ${z_vi_anchor}"
      test -n "${z_vi_tag}"      || buc_die_now "Tag is empty in ${z_vi_anchor_var}: ${z_vi_anchor}"
      z_vi_ref="${z_vi_gar_repo_base}/${z_vi_pkg_path}:${z_vi_tag}"
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

  # === Step 0: in-pool reliquary preflight (defense-in-depth) ===
  local -r z_preflight_step_file="${ZRBFV_GRAFT_META_PREFIX}preflight_step.json"
  zrbfc_assemble_preflight_step "${z_preflight_step_file}" "${ZRBFV_GRAFT_META_PREFIX}"

  # === Combine: preflight + about steps + vouch steps ===
  local -r z_combined_steps="${ZRBFV_GRAFT_META_PREFIX}combined_steps.json"
  jq -s '.[0] + .[1] + .[2]' "${z_preflight_step_file}" "${z_about_steps_file}" "${z_vouch_steps_file}" \
    > "${z_combined_steps}" || buc_die_now "Failed to combine preflight, about, and vouch steps"

  # Compose Build resource JSON with both _RBGA_ and _RBGV_ substitutions
  buc_log_args "Composing combined about+vouch Build resource JSON"
  local -r z_build_file="${ZRBFV_GRAFT_META_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps       "${z_combined_steps}" \
    --arg zjq_sa                "${z_mason_sa}" \
    --arg zjq_gar_host          "${z_gar_host}" \
    --arg zjq_gar_path          "${z_gar_path}" \
    --arg zjq_hallmarks_root    "${RBGL_HALLMARKS_ROOT}" \
    --arg zjq_hallmark          "${z_hallmark}" \
    --arg zjq_vessel            "${RBRV_SIGIL}" \
    --arg zjq_git_commit        "${z_git_commit}" \
    --arg zjq_git_branch        "${z_git_branch}" \
    --arg zjq_git_repo          "${z_git_repo}" \
    --arg zjq_graft_source      "${z_graft_source}" \
    --arg zjq_dockerfile        "${z_dockerfile_content}" \
    --arg zjq_vi_ref_1          "${z_vi_ref_1}" \
    --arg zjq_vi_prov_1         "${z_vi_prov_1}" \
    --arg zjq_vi_ref_2          "${z_vi_ref_2}" \
    --arg zjq_vi_prov_2         "${z_vi_prov_2}" \
    --arg zjq_vi_ref_3          "${z_vi_ref_3}" \
    --arg zjq_vi_prov_3         "${z_vi_prov_3}" \
    --arg zjq_pool              "${RBDC_POOL_AIRGAP}" \
    --arg zjq_timeout           "${RBRR_GCB_TIMEOUT}" \
    --arg zjq_basename_image    "${RBGC_ARK_BASENAME_IMAGE}" \
    --arg zjq_basename_about    "${RBGC_ARK_BASENAME_ABOUT}" \
    --arg zjq_basename_vouch    "${RBGC_ARK_BASENAME_VOUCH}" \
    --arg zjq_basename_attest   "${RBGC_ARK_BASENAME_ATTEST}" \
    --arg zjq_basename_diags    "${RBGC_ARK_BASENAME_DIAGS}" \
    --arg zjq_lodes_root        "${RBGL_LODES_ROOT}" \
    --arg zjq_tag_sprue         "${RBGC_LODE_TAG_SPRUE}" \
    --arg zjq_reliquary         "${RBRV_RELIQUARY}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGA_GAR_HOST:              $zjq_gar_host,
        _RBGA_GAR_PATH:              $zjq_gar_path,
        _RBGA_HALLMARKS_ROOT:        $zjq_hallmarks_root,
        _RBGA_HALLMARK:              $zjq_hallmark,
        _RBGA_VESSEL:                $zjq_vessel,
        _RBGA_VESSEL_MODE:           "rbnve_graft",
        _RBGA_GIT_COMMIT:            $zjq_git_commit,
        _RBGA_GIT_BRANCH:            $zjq_git_branch,
        _RBGA_GIT_REPO:              $zjq_git_repo,
        _RBGA_BUILD_ID:              "",
        _RBGA_INSCRIBE_TIMESTAMP:    "",
        _RBGA_BIND_SOURCE:           "",
        _RBGA_GRAFT_SOURCE:          $zjq_graft_source,
        _RBGA_DOCKERFILE_CONTENT:    $zjq_dockerfile,
        _RBGA_ARK_BASENAME_IMAGE:    $zjq_basename_image,
        _RBGA_ARK_BASENAME_ABOUT:    $zjq_basename_about,
        _RBGA_ARK_BASENAME_DIAGS:    $zjq_basename_diags,
        _RBGV_GAR_HOST:              $zjq_gar_host,
        _RBGV_GAR_PATH:              $zjq_gar_path,
        _RBGV_HALLMARKS_ROOT:        $zjq_hallmarks_root,
        _RBGV_HALLMARK:              $zjq_hallmark,
        _RBGV_VESSEL:                $zjq_vessel,
        _RBGV_VESSEL_MODE:           "rbnve_graft",
        _RBGV_BIND_SOURCE:           "",
        _RBGV_GRAFT_SOURCE:          $zjq_graft_source,
        _RBGV_IMAGE_1:               $zjq_vi_ref_1,
        _RBGV_IMAGE_1_PROVENANCE:    $zjq_vi_prov_1,
        _RBGV_IMAGE_2:               $zjq_vi_ref_2,
        _RBGV_IMAGE_2_PROVENANCE:    $zjq_vi_prov_2,
        _RBGV_IMAGE_3:               $zjq_vi_ref_3,
        _RBGV_IMAGE_3_PROVENANCE:    $zjq_vi_prov_3,
        _RBGV_ARK_BASENAME_IMAGE:    $zjq_basename_image,
        _RBGV_ARK_BASENAME_VOUCH:    $zjq_basename_vouch,
        _RBGV_ARK_BASENAME_ATTEST:   $zjq_basename_attest,
        _RBGR_GAR_HOST:              $zjq_gar_host,
        _RBGR_GAR_PATH:              $zjq_gar_path,
        _RBGR_LODES_ROOT:            $zjq_lodes_root,
        _RBGR_TAG_SPRUE:             $zjq_tag_sprue,
        _RBGR_RELIQUARY:             $zjq_reliquary,
        _RBGR_BASE_LOCATOR_1:    "",
        _RBGR_BASE_LOCATOR_2:    "",
        _RBGR_BASE_LOCATOR_3:    ""
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_build_file}" \
    || buc_die_now "Failed to compose combined about+vouch build JSON"

  buc_log_args "Combined about+vouch build JSON: ${z_build_file}"

  rbndb_check "${z_token}"

  buc_step "Submitting combined about+vouch Cloud Build"
  rbuh_json "POST" "${ZRBFC_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "graft_meta_build_create" "${z_build_file}"
  rbuh_require_ok "Combined about+vouch build submission" "graft_meta_build_create"

  local z_build_id=""
  z_build_id=$(rbuh_json_field_capture "graft_meta_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die_now "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBFC_BUILD_ID_FILE}" || buc_die_now "Failed to persist build ID"

  local -r z_console_url="${ZRBFC_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Combined about+vouch build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbfc_wait_build_completion "${ZRBFC_BUILD_POLL_CEILING_ABOUT_VOUCH}" "About+Vouch"

  buc_success "About+Vouch complete: ${z_hallmark}"
  buc_info "About artifact: ${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_ABOUT}:${z_hallmark}"
  buc_info "Vouch artifact: ${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_VOUCH}:${z_hallmark}"
}

# eof
