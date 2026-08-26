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
# Recipe Bottle Foundry Verify - about body (guard-free cluster, sourced by
# rbfv0_verify): rbfv_about — the standalone about-metadata operation — with
# the support that partitions to it alone, zrbfv_about_submit, which composes
# the about step set into a builds.create submission and waits it out. Reads
# the entry's ZRBFV_ABOUT_PREFIX kindle constant; the combined about+vouch
# submission graft mode takes instead lives in rbfvm_metadata.sh.

set -euo pipefail

######################################################################
# External Functions (rbfv_*)

rbfv_about() {
  zrbfv_sentinel

  # No dirty-tree guard — about constructs metadata for an image already in
  # GAR, not an image. The commit it stamps cannot be made to match the
  # image's build-time tree by gating (standalone re-about is approximate by
  # construction); the ordain paths produce about inside their gated builds.

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
  test -n "${z_vessel_dir}" || buc_die_now "Empty resolved vessel path"
  zrbfc_load_vessel "${z_vessel_dir}"
  test -n "${z_hallmark}" || buc_die_now "Hallmark parameter required"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rba_token_capture "${RBCC_mantle_director}") \
    || buc_die_now "Failed to get Director OAuth token"

  # Gate: require image exists. Image package = rbi_hm/<H>/image, tag = <H>.
  buc_step "Gating on image artifact existence"
  local -r z_hallmark_subtree="${RBGL_HALLMARKS_ROOT}/${z_hallmark}"
  local -r z_image_gate_status="${ZRBFV_ABOUT_PREFIX}image_status.txt"
  local -r z_image_gate_response="${ZRBFV_ABOUT_PREFIX}image_response.json"
  local -r z_image_gate_stderr="${ZRBFV_ABOUT_PREFIX}image_stderr.txt"

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
    || buc_die_now "Image artifact not found (HTTP ${z_image_http_code}) — image must exist before about"

  buc_info "Image artifact confirmed: ${z_hallmark_subtree}/${RBGC_ARK_BASENAME_IMAGE}:${z_hallmark}"

  # Gate: warn if about already exists (re-about is idempotent overwrite)
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
    "${ZRBFC_REGISTRY_API_BASE}/${z_hallmark_subtree}/${RBGC_ARK_BASENAME_ABOUT}/manifests/${z_hallmark}" \
    > "${z_about_gate_status}" 2>"${z_about_gate_stderr}" \
    || z_curl_status=$?
  # RBr_c17
  test "${z_curl_status}" -eq 0 \
    || buc_die_now "HEAD request failed for about artifact (curl exit ${z_curl_status}) — see ${z_about_gate_stderr}"

  local -r z_about_http_code=$(<"${z_about_gate_status}")
  test -n "${z_about_http_code}" || buc_die_now "HTTP status code is empty for about"
  if test "${z_about_http_code}" = "200"; then
    buc_warn "Re-about in progress: ${z_hallmark_subtree}/${RBGC_ARK_BASENAME_ABOUT}:${z_hallmark} already exists"
  fi

  # Submit about Cloud Build
  zrbfv_about_submit "${z_hallmark}" "${z_token}" "${z_conjure_build_id}"

  buc_success "About complete: ${z_hallmark}"
  buc_info "About artifact: ${z_hallmark_subtree}/${RBGC_ARK_BASENAME_ABOUT}:${z_hallmark}"
}

######################################################################
# Internal Functions (zrbfv_*)

# Internal: submit about Cloud Build job and wait for completion
zrbfv_about_submit() {
  zrbfv_sentinel

  local -r z_hallmark="$1"
  local -r z_token="$2"
  local -r z_conjure_build_id="${3:-}"

  buc_step "Constructing about Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBDC_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Determine mode-specific substitution values
  local z_vessel_mode="${RBRV_VESSEL_MODE}"
  local z_bind_source=""
  local z_graft_source=""
  local z_inscribe_ts=""
  local z_dockerfile_content=""
  # Cloud Build substitution values are limited to 4096 bytes. We use 4000 as a
  # conservative guard to account for encoding overhead and avoid edge-case failures.
  local -r z_dockerfile_max_bytes=4000

  case "${z_vessel_mode}" in
    rbnve_conjure)
      # Extract inscribe timestamp from hallmark (e.g., c260305133650 from c260305133650-r260305160530)
      z_inscribe_ts="${z_hallmark%%-r*}"
      # Read Dockerfile content for recipe.txt
      if test -f "${RBRV_CONJURE_DOCKERFILE:-}"; then
        z_dockerfile_content=$(<"${RBRV_CONJURE_DOCKERFILE}")
        if test "${#z_dockerfile_content}" -gt "${z_dockerfile_max_bytes}"; then
          buc_warn "Dockerfile exceeds 4KB substitution limit (${#z_dockerfile_content} bytes) — recipe.txt omitted"
          z_dockerfile_content=""
        fi
      fi
      ;;
    rbnve_bind)
      z_bind_source="${RBRV_BIND_IMAGE:-}"
      if test -n "${RBRV_BIND_OPTIONAL_DOCKERFILE:-}" && test -f "${RBRV_BIND_OPTIONAL_DOCKERFILE}"; then
        z_dockerfile_content=$(<"${RBRV_BIND_OPTIONAL_DOCKERFILE}")
        if test "${#z_dockerfile_content}" -gt "${z_dockerfile_max_bytes}"; then
          buc_warn "Dockerfile exceeds 4KB substitution limit (${#z_dockerfile_content} bytes) — recipe.txt omitted"
          z_dockerfile_content=""
        fi
      fi
      ;;
    rbnve_graft)
      z_graft_source="${RBRV_GRAFT_IMAGE:-}"
      if test -n "${RBRV_GRAFT_OPTIONAL_DOCKERFILE:-}" && test -f "${RBRV_GRAFT_OPTIONAL_DOCKERFILE}"; then
        z_dockerfile_content=$(<"${RBRV_GRAFT_OPTIONAL_DOCKERFILE}")
        if test "${#z_dockerfile_content}" -gt "${z_dockerfile_max_bytes}"; then
          buc_warn "Dockerfile exceeds 4KB substitution limit (${#z_dockerfile_content} bytes) — recipe.txt omitted"
          z_dockerfile_content=""
        fi
      fi
      ;;
    *)
      buc_die_now "Unknown vessel mode: ${z_vessel_mode}"
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
    --slurpfile zjq_steps    "${z_about_steps_accumulator}" \
    --arg zjq_sa             "${z_mason_sa}" \
    --arg zjq_gar_host       "${z_gar_host}" \
    --arg zjq_gar_path       "${z_gar_path}" \
    --arg zjq_hallmarks_root "${RBGL_HALLMARKS_ROOT}" \
    --arg zjq_hallmark       "${z_hallmark}" \
    --arg zjq_vessel         "${RBRV_SIGIL}" \
    --arg zjq_vessel_mode    "${z_vessel_mode}" \
    --arg zjq_git_commit     "${z_git_commit}" \
    --arg zjq_git_branch     "${z_git_branch}" \
    --arg zjq_git_repo       "${z_git_repo}" \
    --arg zjq_build_id       "${z_conjure_build_id}" \
    --arg zjq_inscribe_ts    "${z_inscribe_ts}" \
    --arg zjq_bind_source    "${z_bind_source}" \
    --arg zjq_graft_source   "${z_graft_source}" \
    --arg zjq_dockerfile     "${z_dockerfile_content}" \
    --arg zjq_pool           "${RBDC_POOL_AIRGAP}" \
    --arg zjq_timeout        "${RBRR_GCB_TIMEOUT}" \
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
        _RBGA_DOCKERFILE_CONTENT:    $zjq_dockerfile
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_about_build_file}" \
    || buc_die_now "Failed to compose about build JSON"

  buc_log_args "About build JSON: ${z_about_build_file}"

  rbndb_check "${z_token}"

  buc_step "Submitting about Cloud Build"
  rbuh_json "POST" "${ZRBFC_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "about_build_create" "${z_about_build_file}"
  rbuh_require_ok "About build submission" "about_build_create"

  local z_build_id=""
  z_build_id=$(rbuh_json_field_capture "about_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die_now "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBFC_BUILD_ID_FILE}" || buc_die_now "Failed to persist build ID"

  local -r z_console_url="${ZRBFC_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "About build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbfc_wait_build_completion "${ZRBFC_BUILD_POLL_CEILING_ABOUT}" "About"
}

# eof
