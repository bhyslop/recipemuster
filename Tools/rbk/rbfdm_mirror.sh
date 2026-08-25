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
# Recipe Bottle Foundry Director Build - mirror body (guard-free cluster,
# sourced by rbfd0_director): rbfd_mirror — bind vessel image mirrored from
# upstream into GAR via combined Cloud Build (gcrane cp + about) — and its
# per-operation submit helper zrbfd_mirror_submit. Reads the entry's
# ZRBFD_MIRROR_PREFIX and ZRBFD_RBGJM_STEPS_DIR kindle constants; the
# registry preflight it calls lives in rbfdp_preflight.sh.

set -euo pipefail

######################################################################
# External Functions (rbfd_*)

rbfd_mirror() {
  zrbfd_sentinel

  local z_vessel_dir="${1:-}"

  # Documentation block
  buc_doc_brief "Mirror a bind vessel image from upstream to GAR via combined Cloud Build (gcrane cp + about)"
  buc_doc_param "vessel_dir" "Path to vessel directory containing rbrv.env"
  buc_doc_shown || return 0

  # No-arg: list available vessels
  if test -z "${z_vessel_dir}"; then
    local z_sigils
    z_sigils=$(rbrv_list_capture) || buc_die_now "No vessels found"
    buc_step "Available vessels:"
    local z_sigil=""
    for z_sigil in ${z_sigils}; do
      buc_bare "        ${RBRR_VESSEL_DIR}/${z_sigil}"
    done
    buc_die_now "Vessel directory required"
  fi

  # Load and validate vessel
  zrbfc_load_vessel "${z_vessel_dir}"
  test "${RBRV_VESSEL_MODE:-}" = "rbnve_bind" \
    || buc_die_now "Vessel '${RBRV_SIGIL}' is not a bind vessel (mode: ${RBRV_VESSEL_MODE:-unset})"
  test -n "${RBRV_BIND_IMAGE:-}" \
    || buc_die_now "RBRV_BIND_IMAGE not set for bind vessel '${RBRV_SIGIL}'"

  # Resolve tool images from reliquary (mirror uses gcrane + about steps from reliquary)
  zrbfc_resolve_tool_images

  # Dirty-tree guard — mirror stamps HEAD into the about metadata and composes
  # its cloud step bodies from the working tree; both must match a commit.
  bug_require_clean_tree_creed "${RBCC_creed_clean_build}"

  # Authenticate as Director
  buc_step "Authenticating as Director"
  local z_token
  z_token=$(rba_token_capture "${RBCC_mantle_director}") \
    || buc_die_now "Failed to get Director OAuth token"

  # Registry preflight -- verify reliquary and base images exist before expensive operations
  zrbfd_registry_preflight "${z_token}" "${z_vessel_dir}"

  # GAR coordinates
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_base="${z_gar_host}/${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"

  # Generate hallmark timestamps: bYYMMDDHHMMSS-rYYMMDDHHMMSS
  local -r z_mirror_ts="${RBGC_HALLMARK_PREFIX_BIND}${BURD_NOW_STAMP:2:6}${BURD_NOW_STAMP:9:6}"
  local -r z_build_ts_file="${ZRBFD_MIRROR_PREFIX}build_ts.txt"
  date -u +'%y%m%d%H%M%S' > "${z_build_ts_file}" || buc_die_now "Failed to generate build timestamp"
  local z_build_ts
  z_build_ts="r$(<"${z_build_ts_file}")"
  test -n "${z_build_ts}" || buc_die_now "Empty build timestamp from ${z_build_ts_file}"
  local -r z_hallmark="${z_mirror_ts}-${z_build_ts}"

  buc_info "Hallmark: ${z_hallmark}"

  # Persist to output directory for chaining by rbfd_ordain
  echo "${z_vessel_dir}" > "${ZRBFC_OUTPUT_VESSEL_DIR}" \
    || buc_die_now "Failed to write vessel dir to output"
  buf_write_fact_single "${RBF_FACT_HALLMARK}" "${z_hallmark}"

  # Write GAR root fact file
  buf_write_fact_single "${RBF_FACT_GAR_ROOT}" "${z_gar_base}"

  # Write ark stem fact file (hallmark subtree under HALLMARKS_ROOT)
  buf_write_fact_single "${RBF_FACT_ARK_STEM}" "${RBGL_HALLMARKS_ROOT}/${z_hallmark}"

  # Write yield fact file (single-platform bind image)
  buf_write_fact_single "${RBF_FACT_ARK_YIELD}-${RBGC_ARK_BASENAME_IMAGE}" \
    "${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_IMAGE}:${z_hallmark}"

  # Submit combined Cloud Build (gcrane image copy + about steps)
  zrbfd_mirror_submit "${z_hallmark}" "${z_token}"

  # Summary
  echo ""
  buc_success "Mirror complete: ${RBRV_SIGIL}"
  echo "  Hallmark: ${z_hallmark}"
}

######################################################################
# Internal Functions (zrbfd_*)

# Internal: submit combined mirror Cloud Build job (gcrane image copy + about steps)
# Args: hallmark token
zrbfd_mirror_submit() {
  zrbfd_sentinel

  local -r z_hallmark="$1"
  local -r z_token="$2"

  buc_step "Constructing combined mirror Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBDC_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Step 0: Mirror image via gcrane
  local -r z_mscript_path="${ZRBFD_RBGJM_STEPS_DIR}/rbgjm01-mirror-image.sh"
  test -f "${z_mscript_path}" || buc_die_now "Mirror step script not found: ${z_mscript_path}"

  local -r z_mbody_file="${ZRBFD_MIRROR_PREFIX}mirror_body.txt"
  local -r z_mescaped_file="${ZRBFD_MIRROR_PREFIX}mirror_escaped.txt"
  local -r z_mirror_step_file="${ZRBFD_MIRROR_PREFIX}mirror_step.json"
  local -r z_mirror_step_built="${ZRBFD_MIRROR_PREFIX}mirror_step_built.json"

  buc_log_args "Reading mirror step script (skip shebang)"
  zrbfc_write_script_body "${z_mscript_path}" "${z_mbody_file}" \
    || buc_die_now "Failed to read mirror step script"
  local z_mbody
  z_mbody=$(<"${z_mbody_file}")
  test -n "${z_mbody}" || buc_die_now "Empty mirror script body"

  printf '#!/busybox/sh\n%s' "${z_mbody}" > "${z_mescaped_file}" \
    || buc_die_now "Failed to escape mirror script body"

  echo "[]" > "${z_mirror_step_file}" || buc_die_now "Failed to initialize mirror step JSON"
  jq \
    --arg name "${z_rbfc_tool_gcrane}" \
    --arg id "mirror-image" \
    --rawfile script "${z_mescaped_file}" \
    '. + [{name: $name, id: $id, script: $script}]' \
    "${z_mirror_step_file}" > "${z_mirror_step_built}" \
    || buc_die_now "Failed to build mirror step JSON"
  mv "${z_mirror_step_built}" "${z_mirror_step_file}" \
    || buc_die_now "Failed to finalize mirror step JSON"

  # Steps 1-4: About (shared with standalone about pipeline)
  local -r z_about_steps_file="${ZRBFD_MIRROR_PREFIX}about_steps.json"
  zrbfc_assemble_about_steps "${z_about_steps_file}" "${ZRBFD_MIRROR_PREFIX}about_"

  # Step 0: in-pool reliquary preflight (defense-in-depth)
  local -r z_preflight_step_file="${ZRBFD_MIRROR_PREFIX}preflight_step.json"
  zrbfc_assemble_preflight_step "${z_preflight_step_file}" "${ZRBFD_MIRROR_PREFIX}"

  # Combine: preflight step + mirror step + about steps
  local -r z_combined_steps="${ZRBFD_MIRROR_PREFIX}combined_steps.json"
  jq -s '.[0] + .[1] + .[2]' "${z_preflight_step_file}" "${z_mirror_step_file}" "${z_about_steps_file}" \
    > "${z_combined_steps}" || buc_die_now "Failed to combine preflight, mirror, and about steps"

  # Git metadata (shared temp files, idempotent)
  zrbfc_ensure_git_metadata
  local z_git_commit=""
  z_git_commit=$(<"${ZRBFC_GIT_COMMIT_FILE}")
  local z_git_branch=""
  z_git_branch=$(<"${ZRBFC_GIT_BRANCH_FILE}")
  local z_git_repo=""
  z_git_repo=$(<"${ZRBFC_GIT_REPO_FILE}")

  # Mode-specific substitution values for bind
  local -r z_bind_source="${RBRV_BIND_IMAGE:-}"
  local z_dockerfile_content=""
  local -r z_dockerfile_max_bytes=4000
  if test -n "${RBRV_BIND_OPTIONAL_DOCKERFILE:-}" && test -f "${RBRV_BIND_OPTIONAL_DOCKERFILE}"; then
    z_dockerfile_content=$(<"${RBRV_BIND_OPTIONAL_DOCKERFILE}")
    if test "${#z_dockerfile_content}" -gt "${z_dockerfile_max_bytes}"; then
      buc_warn "Dockerfile exceeds 4KB substitution limit (${#z_dockerfile_content} bytes) — recipe.txt omitted"
      z_dockerfile_content=""
    fi
  fi

  # Pool routing: bind uses vessel's egress mode (tether for upstream pulls, airgap if pre-staged)
  local z_mirror_pool=""
  case "${RBRV_EGRESS_MODE}" in
    rbnve_tether) z_mirror_pool="${RBDC_POOL_TETHER}" ;;
    rbnve_airgap) z_mirror_pool="${RBDC_POOL_AIRGAP}" ;;
    *) buc_die_now "Unknown RBRV_EGRESS_MODE: ${RBRV_EGRESS_MODE}" ;;
  esac

  # Compose Build resource JSON
  buc_log_args "Composing combined mirror Build resource JSON"
  local -r z_mirror_build_file="${ZRBFD_MIRROR_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps    "${z_combined_steps}" \
    --arg zjq_sa             "${z_mason_sa}" \
    --arg zjq_gar_host       "${z_gar_host}" \
    --arg zjq_gar_path       "${z_gar_path}" \
    --arg zjq_hallmarks_root "${RBGL_HALLMARKS_ROOT}" \
    --arg zjq_hallmark       "${z_hallmark}" \
    --arg zjq_vessel         "${RBRV_SIGIL}" \
    --arg zjq_vessel_mode    "rbnve_bind" \
    --arg zjq_git_commit     "${z_git_commit}" \
    --arg zjq_git_branch     "${z_git_branch}" \
    --arg zjq_git_repo       "${z_git_repo}" \
    --arg zjq_build_id       "" \
    --arg zjq_inscribe_ts    "" \
    --arg zjq_bind_source    "${z_bind_source}" \
    --arg zjq_graft_source   "" \
    --arg zjq_dockerfile     "${z_dockerfile_content}" \
    --arg zjq_pool           "${z_mirror_pool}" \
    --arg zjq_timeout        "${RBRR_GCB_TIMEOUT}" \
    --arg zjq_basename_image "${RBGC_ARK_BASENAME_IMAGE}" \
    --arg zjq_basename_about "${RBGC_ARK_BASENAME_ABOUT}" \
    --arg zjq_basename_diags "${RBGC_ARK_BASENAME_DIAGS}" \
    --arg zjq_lodes_root     "${RBGL_LODES_ROOT}" \
    --arg zjq_tag_sprue      "${RBGC_LODE_TAG_SPRUE}" \
    --arg zjq_reliquary      "${RBRV_RELIQUARY}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGA_GAR_HOST:              $zjq_gar_host,
        _RBGA_GAR_PATH:              $zjq_gar_path,
        _RBGA_HALLMARKS_ROOT:        $zjq_hallmarks_root,
        _RBGA_HALLMARK:              $zjq_hallmark,
        _RBGA_VESSEL:                $zjq_vessel,
        _RBGA_VESSEL_MODE:           $zjq_vessel_mode,
        _RBGA_GIT_COMMIT:            $zjq_git_commit,
        _RBGA_GIT_BRANCH:            $zjq_git_branch,
        _RBGA_GIT_REPO:              $zjq_git_repo,
        _RBGA_BUILD_ID:              $zjq_build_id,
        _RBGA_INSCRIBE_TIMESTAMP:    $zjq_inscribe_ts,
        _RBGA_BIND_SOURCE:           $zjq_bind_source,
        _RBGA_GRAFT_SOURCE:          $zjq_graft_source,
        _RBGA_DOCKERFILE_CONTENT:    $zjq_dockerfile,
        _RBGA_ARK_BASENAME_IMAGE:    $zjq_basename_image,
        _RBGA_ARK_BASENAME_ABOUT:    $zjq_basename_about,
        _RBGA_ARK_BASENAME_DIAGS:    $zjq_basename_diags,
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
    }' > "${z_mirror_build_file}" \
    || buc_die_now "Failed to compose mirror build JSON"

  buc_log_args "Mirror build JSON: ${z_mirror_build_file}"

  rbndb_check "${z_token}"

  buc_step "Submitting combined mirror Cloud Build"
  rbuh_json "POST" "${ZRBFC_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "mirror_build_create" "${z_mirror_build_file}"
  rbuh_require_ok "Mirror build submission" "mirror_build_create"

  local z_build_id=""
  z_build_id=$(rbuh_json_field_capture "mirror_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die_now "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBFC_BUILD_ID_FILE}" || buc_die_now "Failed to persist build ID"

  local -r z_console_url="${ZRBFC_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Mirror build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbfc_wait_build_completion "${ZRBFC_BUILD_POLL_CEILING_MIRROR}" "Mirror"
}

# eof
