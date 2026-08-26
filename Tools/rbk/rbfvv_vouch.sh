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
# Recipe Bottle Foundry Verify - vouch body (guard-free cluster, sourced by
# rbfv0_verify): rbfv_vouch — the mode-aware vouch operation — with the
# support that partitions to it alone, zrbfv_vouch_submit, which composes the
# vouch step set into a builds.create submission and waits it out. Reads the
# entry's ZRBFV_VOUCH_PREFIX kindle constant; the batch driver that calls
# rbfv_vouch once per pending hallmark lives in rbfvb_batch.sh.

set -euo pipefail

######################################################################
# External Functions (rbfv_*)

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
    z_sigils=$(rbrv_list_capture) || buc_die_now "No vessels found"
    buc_step "Available vessels:"
    local z_sigil=""
    for z_sigil in ${z_sigils}; do
      buc_bare "        ${RBRR_VESSEL_DIR}/${z_sigil}"
    done
    buc_die_now "Vessel directory required"
  fi

  zrbfc_load_vessel "${z_vessel_dir}"
  test -n "${z_hallmark}" || buc_die_now "Hallmark parameter required"

  # Resolve tool images from reliquary (vouch steps use tool images)
  zrbfc_resolve_tool_images

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rba_token_capture "${RBCC_mantle_director}") \
    || buc_die_now "Failed to get Director OAuth token"

  # Gate: require about exists (about must complete before vouch)
  buc_step "Gating on about artifact existence"
  local -r z_hallmark_subtree="${RBGL_HALLMARKS_ROOT}/${z_hallmark}"
  local -r z_about_gate_status="${ZRBFV_VOUCH_PREFIX}about_status.txt"
  local -r z_about_gate_response="${ZRBFV_VOUCH_PREFIX}about_response.json"
  local -r z_about_gate_stderr="${ZRBFV_VOUCH_PREFIX}about_stderr.txt"

  local z_curl_status=0
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
  test "${z_about_http_code}" = "200" \
    || buc_die_now "About artifact not found (HTTP ${z_about_http_code}) — about must complete before vouch"

  buc_info "About artifact confirmed: ${z_hallmark_subtree}/${RBGC_ARK_BASENAME_ABOUT}:${z_hallmark}"

  # Gate: warn if vouch already exists (re-vouch)
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
    "${ZRBFC_REGISTRY_API_BASE}/${z_hallmark_subtree}/${RBGC_ARK_BASENAME_VOUCH}/manifests/${z_hallmark}" \
    > "${z_vouch_gate_status}" 2>"${z_vouch_gate_stderr}" \
    || z_curl_status=$?
  test "${z_curl_status}" -eq 0 \
    || buc_die_now "HEAD request failed for vouch artifact (curl exit ${z_curl_status}) — see ${z_vouch_gate_stderr}"

  local -r z_vouch_http_code=$(<"${z_vouch_gate_status}")
  test -n "${z_vouch_http_code}" || buc_die_now "HTTP status code is empty for vouch"
  if test "${z_vouch_http_code}" = "200"; then
    buc_warn "Re-vouch in progress: ${z_hallmark_subtree}/${RBGC_ARK_BASENAME_VOUCH}:${z_hallmark} already exists"
  fi

  # All modes use Cloud Build for vouch (mode-aware verification inside the build)
  zrbfv_vouch_submit "${z_hallmark}" "${z_token}"

  buc_success "Vouch complete: ${z_hallmark}"
  buc_info "Vouch artifact: ${z_hallmark_subtree}/${RBGC_ARK_BASENAME_VOUCH}:${z_hallmark}"
}

######################################################################
# Internal Functions (zrbfv_*)

# Internal: Submit vouch Cloud Build job (mode-aware verification)
# All vessel modes use Cloud Build. The build scripts branch on _RBGV_VESSEL_MODE:
#   conjure: DSSE envelope signature verification (Python 3 + openssl)
#   bind: digest-pin comparison against upstream reference
#   graft: GRAFTED stamp (no verification)
zrbfv_vouch_submit() {
  zrbfv_sentinel

  local -r z_hallmark="$1"
  local -r z_token="$2"

  buc_step "Constructing vouch Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBDC_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Mode-specific substitution values (empty strings for non-applicable modes)
  local z_bind_source=""
  local z_graft_source=""

  case "${RBRV_VESSEL_MODE}" in
    rbnve_conjure) : ;;  # DSSE verification uses embedded keys, no extra substitutions
    rbnve_bind)    z_bind_source="${RBRV_BIND_IMAGE:-}" ;;
    rbnve_graft)   z_graft_source="${RBRV_GRAFT_IMAGE:-}" ;;
    *)             buc_die_now "Unknown vessel mode: ${RBRV_VESSEL_MODE}" ;;
  esac

  # Resolve base image provenance (for vouch summary recording)
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

  # Assemble vouch steps via shared helper
  local -r z_vouch_steps_accumulator="${ZRBFV_VOUCH_PREFIX}steps.json"
  zrbfc_assemble_vouch_steps "${z_vouch_steps_accumulator}" "${ZRBFV_VOUCH_PREFIX}"

  buc_log_args "Composing vouch Build resource JSON"
  local -r z_vouch_build_file="${ZRBFV_VOUCH_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps    "${z_vouch_steps_accumulator}" \
    --arg zjq_sa             "${z_mason_sa}" \
    --arg zjq_gar_host       "${z_gar_host}" \
    --arg zjq_gar_path       "${z_gar_path}" \
    --arg zjq_hallmarks_root "${RBGL_HALLMARKS_ROOT}" \
    --arg zjq_hallmark       "${z_hallmark}" \
    --arg zjq_vessel         "${RBRV_SIGIL}" \
    --arg zjq_vessel_mode    "${RBRV_VESSEL_MODE}" \
    --arg zjq_bind_source    "${z_bind_source}" \
    --arg zjq_graft_source   "${z_graft_source}" \
    --arg zjq_vi_ref_1       "${z_vi_ref_1}" \
    --arg zjq_vi_prov_1      "${z_vi_prov_1}" \
    --arg zjq_vi_ref_2       "${z_vi_ref_2}" \
    --arg zjq_vi_prov_2      "${z_vi_prov_2}" \
    --arg zjq_vi_ref_3       "${z_vi_ref_3}" \
    --arg zjq_vi_prov_3      "${z_vi_prov_3}" \
    --arg zjq_pool           "${RBDC_POOL_AIRGAP}" \
    --arg zjq_timeout        "${RBRR_GCB_TIMEOUT}" \
    --arg zjq_basename_image  "${RBGC_ARK_BASENAME_IMAGE}" \
    --arg zjq_basename_vouch  "${RBGC_ARK_BASENAME_VOUCH}" \
    --arg zjq_basename_attest "${RBGC_ARK_BASENAME_ATTEST}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGV_GAR_HOST:            $zjq_gar_host,
        _RBGV_GAR_PATH:            $zjq_gar_path,
        _RBGV_HALLMARKS_ROOT:      $zjq_hallmarks_root,
        _RBGV_HALLMARK:            $zjq_hallmark,
        _RBGV_VESSEL:              $zjq_vessel,
        _RBGV_VESSEL_MODE:         $zjq_vessel_mode,
        _RBGV_BIND_SOURCE:         $zjq_bind_source,
        _RBGV_GRAFT_SOURCE:        $zjq_graft_source,
        _RBGV_IMAGE_1:             $zjq_vi_ref_1,
        _RBGV_IMAGE_1_PROVENANCE:  $zjq_vi_prov_1,
        _RBGV_IMAGE_2:             $zjq_vi_ref_2,
        _RBGV_IMAGE_2_PROVENANCE:  $zjq_vi_prov_2,
        _RBGV_IMAGE_3:             $zjq_vi_ref_3,
        _RBGV_IMAGE_3_PROVENANCE:  $zjq_vi_prov_3,
        _RBGV_ARK_BASENAME_IMAGE:  $zjq_basename_image,
        _RBGV_ARK_BASENAME_VOUCH:  $zjq_basename_vouch,
        _RBGV_ARK_BASENAME_ATTEST: $zjq_basename_attest
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_vouch_build_file}" \
    || buc_die_now "Failed to compose vouch build JSON"

  buc_log_args "Vouch build JSON: ${z_vouch_build_file}"

  rbndb_check "${z_token}"

  buc_step "Submitting vouch Cloud Build"
  rbuh_json "POST" "${ZRBFC_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "vouch_build_create" "${z_vouch_build_file}"
  rbuh_require_ok "Vouch build submission" "vouch_build_create"

  local z_build_id=""
  z_build_id=$(rbuh_json_field_capture "vouch_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die_now "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBFC_BUILD_ID_FILE}" || buc_die_now "Failed to persist build ID"

  local -r z_console_url="${ZRBFC_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Vouch build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbfc_wait_build_completion "${ZRBFC_BUILD_POLL_CEILING_VOUCH}" "Vouch"
}

# eof
