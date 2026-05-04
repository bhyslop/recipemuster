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
# Recipe Bottle Foundry Ledger - inscribe, yoke, jettison, abjure, wrest, rekon, audit, and tally operations
# Director credentials for inscribe, yoke, jettison, abjure, wrest, rekon, audit; retriever credentials for tally

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
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBDC_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

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
    --arg zjq_gar_host         "${z_gar_host}" \
    --arg zjq_gar_path         "${z_gar_path}" \
    --arg zjq_reliquaries_root "${RBGL_RELIQUARIES_ROOT}" \
    --arg zjq_reliquary        "${z_reliquary}" \
    --arg zjq_pool             "${RBDC_POOL_TETHER}" \
    --arg zjq_timeout          "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGN_GAR_HOST:         $zjq_gar_host,
        _RBGN_GAR_PATH:         $zjq_gar_path,
        _RBGN_RELIQUARIES_ROOT: $zjq_reliquaries_root,
        _RBGN_RELIQUARY:        $zjq_reliquary
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

  buf_write_fact_single "${RBF_FACT_RELIQUARY}" "${z_reliquary}"

  buc_success "Inscribe complete — reliquary ${z_reliquary} created"
  buc_info "Add RBRV_RELIQUARY=${z_reliquary} to vessel rbrv.env files to use this reliquary"
}

rbfl_yoke() {
  zrbfl_sentinel

  local -r z_stamp="${BUZ_FOLIO:-}"

  buc_doc_brief "Yoke a reliquary stamp into every vessel's rbrv.env — pre-validate stamp once against GAR, then rewrite RBRV_RELIQUARY across all vessels under \${RBRR_VESSEL_DIR}"
  buc_doc_param "stamp" "Reliquary datestamp (e.g., r260327172456)"
  buc_doc_shown || return 0

  test -n "${z_stamp}" || buc_die "Reliquary stamp required (param1)"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  buc_step "Validating reliquary stamp: ${z_stamp}"
  local -r z_rqy_subtree="${RBGL_RELIQUARIES_ROOT}/${z_stamp}/"
  local -r z_list_url="${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages?pageSize=1000"
  local -r z_list_infix="rbfl_yoke_list"

  rbgu_http_json "GET" "${z_list_url}" "${z_token}" "${z_list_infix}"
  rbgu_http_require_ok "List reliquary packages" "${z_list_infix}"

  local -r z_resp_file="${ZRBGU_PREFIX}${z_list_infix}${ZRBGU_POSTFIX_JSON}"
  local -r z_present_file="${BURD_TEMP_DIR}/rbfl_yoke_present.txt"

  jq -r --arg subtree "${z_rqy_subtree}" '
    .packages[]?.name
    | sub("^.*/packages/"; "")
    | gsub("%2F"; "/")
    | select(startswith($subtree))
    | ltrimstr($subtree)
  ' "${z_resp_file}" > "${z_present_file}" \
    || buc_die "Failed to extract reliquary package names"

  local z_present=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    z_present+=("${z_line}")
  done < "${z_present_file}"

  local -r z_expected=(
    "${RBGC_RELIQUARY_TOOL_GCLOUD}"
    "${RBGC_RELIQUARY_TOOL_DOCKER}"
    "${RBGC_RELIQUARY_TOOL_ALPINE}"
    "${RBGC_RELIQUARY_TOOL_SYFT}"
    "${RBGC_RELIQUARY_TOOL_BINFMT}"
    "${RBGC_RELIQUARY_TOOL_SKOPEO}"
  )

  local z_missing=""
  local z_tool=""
  for z_tool in "${z_expected[@]}"; do
    local z_found=0
    local z_i=""
    for z_i in "${!z_present[@]}"; do
      test "${z_present[$z_i]}" = "${z_tool}" || continue
      z_found=1
      break
    done
    case "${z_found}" in
      1) ;;
      *) z_missing="${z_missing}${z_missing:+, }${z_tool}" ;;
    esac
  done

  test -z "${z_missing}" || buc_die "Reliquary stamp '${z_stamp}' not found in Depot — expected 6 tool images under ${z_rqy_subtree}; missing: ${z_missing}. Re-run tt/rbw-dI.DirectorInscribesReliquary.sh to mint a fresh reliquary, or verify the stamp spelling."
  buc_info "Reliquary valid — all 6 tool images present (gcloud, docker, alpine, syft, binfmt, skopeo)"

  buc_step "Yoking ${z_stamp} into all vessels under ${RBRR_VESSEL_DIR}"

  local z_written=()
  local z_vessel_dir=""
  local z_rbrv_file=""
  local z_sigil=""
  local z_tmp_file=""
  local z_rbrv_lines=()
  local z_rbrv_line=""
  local z_wrote=0
  local z_j=""

  for z_vessel_dir in "${RBRR_VESSEL_DIR}"/*/; do
    test -d "${z_vessel_dir}" || continue
    z_rbrv_file="${z_vessel_dir%/}/rbrv.env"
    test -f "${z_rbrv_file}" || continue
    z_sigil="${z_vessel_dir%/}"
    z_sigil="${z_sigil##*/}"

    z_rbrv_lines=()
    while IFS= read -r z_rbrv_line || test -n "${z_rbrv_line}"; do
      z_rbrv_lines+=("${z_rbrv_line}")
    done < "${z_rbrv_file}"

    z_tmp_file="${BURD_TEMP_DIR}/rbfl_yoke_${z_sigil}_rbrv.env.new"
    : > "${z_tmp_file}" \
      || buc_die "Failed to create ${z_tmp_file} (yoking ${z_sigil}; already wrote: ${z_written[*]:-(none)})"

    z_wrote=0
    for z_j in "${!z_rbrv_lines[@]}"; do
      case "${z_rbrv_lines[$z_j]}" in
        RBRV_RELIQUARY=*)
          printf 'RBRV_RELIQUARY=%s\n' "${z_stamp}" >> "${z_tmp_file}" \
            || buc_die "Failed to write RBRV_RELIQUARY for ${z_sigil} (already wrote: ${z_written[*]:-(none)})"
          z_wrote=1
          ;;
        *)
          printf '%s\n' "${z_rbrv_lines[$z_j]}" >> "${z_tmp_file}" \
            || buc_die "Failed to write line for ${z_sigil} (already wrote: ${z_written[*]:-(none)})"
          ;;
      esac
    done

    case "${z_wrote}" in
      1) ;;
      *) printf '\n# Tool Image Reliquary\nRBRV_RELIQUARY=%s\n' "${z_stamp}" >> "${z_tmp_file}" \
           || buc_die "Failed to append RBRV_RELIQUARY for ${z_sigil} (already wrote: ${z_written[*]:-(none)})" ;;
    esac

    mv "${z_tmp_file}" "${z_rbrv_file}" \
      || buc_die "Failed to finalize ${z_rbrv_file} (yoking ${z_sigil}; already wrote: ${z_written[*]:-(none)})"

    z_written+=("${z_sigil}")
    buc_log_args "Yoked ${z_sigil}"
  done

  test "${#z_written[@]}" -gt 0 \
    || buc_die "No vessels found under ${RBRR_VESSEL_DIR} — nothing yoked"

  buc_success "Yoked ${#z_written[@]} vessel(s) to reliquary ${z_stamp}"
  buc_info "Vessels: ${z_written[*]}"
  buc_info "Commit the rbrv.env changes with your usual git workflow."
  buc_info "Reminder: the reliquary tool images are now linked, but vessel images must be rebuilt (ordain) to pick up the new tool versions."
}

rbfl_jettison() {
  zrbfl_sentinel

  local z_locator="${BUZ_FOLIO:-}"

  # Documentation block
  buc_doc_brief "Jettison an image tag from the registry by locator"
  buc_doc_param "locator" "Image locator in package-path:tag format (e.g. rbi_hm/H/image:H)"
  buc_doc_shown || return 0

  # Validate locator parameter
  test -n "${z_locator}" || buc_die "Locator parameter required (package-path:tag)"

  # Parse locator into package path and tag
  case "${z_locator}" in
    *:*) : ;;
    *)   buc_die "Invalid locator format. Expected package-path:tag" ;;
  esac
  local z_pkg_path="${z_locator%:*}"
  local z_tag="${z_locator##*:}"
  test -n "${z_pkg_path}" || buc_die "Package path is empty in locator"
  test -n "${z_tag}" || buc_die "Tag is empty in locator"

  buc_step "Authenticating as Director"

  # Get OAuth token using Director credentials
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  buc_require "Will jettison: ${z_locator}" "yes"

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
    "${ZRBFC_REGISTRY_API_BASE}/${z_pkg_path}/manifests/${z_tag}" \
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

  local z_hallmark="${BUZ_FOLIO:-}"

  # Documentation block
  buc_doc_brief "Abjure a hallmark — delete all GAR packages under rbi_hm/<hallmark>/"
  buc_doc_param "hallmark" "Full hallmark (e.g., c260305133650-r260305160530)"
  buc_doc_shown || return 0

  test -n "${z_hallmark}" || buc_die "Hallmark parameter required"

  buc_step "Authenticating as Director"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  # Enumerate packages under rbi_hm/<hallmark>/ via GAR REST API.
  # Each immediate child of the subtree is one ark (image, vouch, pouch,
  # about, attest, diags). Iterating discovered children rather than a
  # hardcoded suffix list naturally tolerates graft's missing pouch.
  local -r z_subtree="${RBGL_HALLMARKS_ROOT}/${z_hallmark}/"
  buc_step "Enumerating packages under ${z_subtree}"

  local -r z_list_url="${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages?pageSize=1000"
  local -r z_list_infix="rbfl_abjure_list"

  rbgu_http_json "GET" "${z_list_url}" "${z_token}" "${z_list_infix}"
  rbgu_http_require_ok "List packages for abjure" "${z_list_infix}"

  # GAR returns package names URL-encoded in the resource name (slashes as
  # %2F); decode and prefix-match to the hallmark subtree.
  local -r z_resp_file="${ZRBGU_PREFIX}${z_list_infix}${ZRBGU_POSTFIX_JSON}"
  local -r z_pkg_file="${ZRBFL_DELETE_PREFIX}packages.txt"

  jq -r --arg subtree "${z_subtree}" '
    .packages[]?.name
    | sub("^.*/packages/"; "")
    | gsub("%2F"; "/")
    | select(startswith($subtree))
  ' "${z_resp_file}" > "${z_pkg_file}" \
    || buc_die "Failed to extract package names for hallmark subtree"

  if ! test -s "${z_pkg_file}"; then
    buc_die "No packages found under ${z_subtree} — hallmark not present in registry"
  fi

  local z_count
  z_count=$(wc -l < "${z_pkg_file}" | tr -d ' ')

  local z_confirm_msg="Will abjure ${z_count} packages under ${z_subtree}:"
  local z_pkg_path=""
  while IFS= read -r z_pkg_path || test -n "${z_pkg_path}"; do
    z_confirm_msg="${z_confirm_msg}\n  - ${z_pkg_path}"
  done < "${z_pkg_file}"
  buc_require "${z_confirm_msg}" "yes"

  # Delete each package via GAR REST API.
  # DELETE returns a long-running operation; trust 200 as accepted (matches
  # the prior fire-and-forget semantics with 202/204 on the v2 manifest API).
  local z_pkg_path=""
  local z_del_idx=0
  while IFS= read -r z_pkg_path || test -n "${z_pkg_path}"; do
    buc_step "Deleting package: ${z_pkg_path}"

    local z_pkg_encoded="${z_pkg_path//\//%2F}"
    local z_del_url="${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages/${z_pkg_encoded}"
    local z_del_infix="rbfl_abjure_del_${z_del_idx}"

    rbgu_http_json "DELETE" "${z_del_url}" "${z_token}" "${z_del_infix}"
    rbgu_http_require_ok "Delete package ${z_pkg_path}" "${z_del_infix}"

    buc_info "Deleted: ${z_pkg_path}"
    z_del_idx=$((z_del_idx + 1))
  done < "${z_pkg_file}"

  echo ""
  buc_success "Hallmark abjured: ${z_hallmark} (${z_count} packages)"
}

rbfl_tally() {
  zrbfl_sentinel

  buc_doc_brief "Tally hallmarks with health status (vouched / pending / incomplete)"
  buc_doc_shown || return 0

  buc_step "Authenticating as Retriever"
  test -f "${RBDC_RETRIEVER_RBRA_FILE}" \
    || buc_die "Retriever credential not found: ${RBDC_RETRIEVER_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_RETRIEVER_RBRA_FILE}") \
    || buc_die "Failed to get Retriever OAuth token"

  buc_step "Enumerating hallmarks under ${RBGL_HALLMARKS_ROOT}/"
  zrbfc_list_packages_capture "${z_token}" "${RBGL_HALLMARKS_ROOT}"

  if ! test -s "${ZRBFC_PACKAGE_LIST_FILE}"; then
    buc_info "No hallmarks found under ${RBGL_HALLMARKS_ROOT}/"
    buc_success "Tally complete — 0 hallmarks"
    return 0
  fi

  # Load-then-iterate. A synthetic sentinel element appended to the array
  # lets the final hallmark flush through the same boundary branch as every
  # intermediate one (single flush site).
  local z_lines=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_lines+=("${z_line}")
  done < "${ZRBFC_PACKAGE_LIST_FILE}"
  z_lines+=("__SENTINEL__ __SENTINEL__")

  echo ""
  printf "  %-30s  %-11s  %s\n" "HALLMARK" "HEALTH" "BASENAMES"
  printf "  %-30s  %-11s  %s\n" "------------------------------" "-----------" "---------"

  # State machine over <hallmark> <basename> pairs (file was sorted by the
  # capture helper). Vessel is no longer encoded in the GAR path —
  # restoration via about/vouch metadata is AAL territory.
  local z_prev_h="" z_prev_bns=""
  local z_prev_img=0 z_prev_abt=0 z_prev_vch=0
  local z_count=0 z_vouched_n=0 z_pending_n=0 z_incomplete_n=0
  local z_i="" z_h="" z_b="" z_health=""

  for z_i in "${!z_lines[@]}"; do
    z_line="${z_lines[$z_i]}"
    test -n "${z_line}" || continue

    z_h="${z_line%% *}"
    z_b="${z_line#* }"
    test -n "${z_h}" || continue
    test -n "${z_b}" || continue

    if test "${z_h}" != "${z_prev_h}"; then
      if test -n "${z_prev_h}"; then
        if test "${z_prev_img}" = "1" \
          && test "${z_prev_abt}" = "1" \
          && test "${z_prev_vch}" = "1"; then
          z_health="vouched"
          z_vouched_n=$(( z_vouched_n + 1 ))
        elif test "${z_prev_img}" = "1" \
          && test "${z_prev_abt}" = "1"; then
          z_health="pending"
          z_pending_n=$(( z_pending_n + 1 ))
        else
          z_health="incomplete"
          z_incomplete_n=$(( z_incomplete_n + 1 ))
        fi
        printf "  %-30s  %-11s  %s\n" "${z_prev_h}" "${z_health}" "${z_prev_bns}"
        z_count=$(( z_count + 1 ))
      fi

      case "${z_h}" in
        __SENTINEL__) break ;;
      esac

      z_prev_h="${z_h}"
      z_prev_bns=""
      z_prev_img=0
      z_prev_abt=0
      z_prev_vch=0
    fi

    z_prev_bns="${z_prev_bns}${z_prev_bns:+ }${z_b}"
    case "${z_b}" in
      "${RBGC_ARK_BASENAME_IMAGE}") z_prev_img=1 ;;
      "${RBGC_ARK_BASENAME_ABOUT}") z_prev_abt=1 ;;
      "${RBGC_ARK_BASENAME_VOUCH}") z_prev_vch=1 ;;
    esac
  done

  echo ""
  buc_info "Total hallmarks: ${z_count}  (vouched: ${z_vouched_n}, pending: ${z_pending_n}, incomplete: ${z_incomplete_n})"

  case "${z_pending_n}" in
    0) ;;
    *) buc_info "To vouch pending hallmarks:"
       buc_tabtarget "rbw-fV"
       ;;
  esac

  case "${z_incomplete_n}" in
    0) ;;
    *) buc_info "To abjure incomplete hallmarks:"
       buc_tabtarget "rbw-fA"
       ;;
  esac

  buc_success "Tally complete"
}

rbfl_rekon_hallmark() {
  zrbfl_sentinel

  local -r z_hallmark="${BUZ_FOLIO:-}"

  buc_doc_brief "List ark basenames present under a hallmark's GAR subtree"
  buc_doc_param "hallmark" "Hallmark identifier"
  buc_doc_shown || return 0

  test -n "${z_hallmark}" || buc_die "Usage: rbw-irh <hallmark>"

  buc_step "Authenticating as Director"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}" \
    || buc_die "Director credential not found: ${RBDC_DIRECTOR_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  buc_step "Enumerating arks under ${RBGL_HALLMARKS_ROOT}/${z_hallmark}/"
  zrbfc_list_packages_capture "${z_token}" "${RBGL_HALLMARKS_ROOT}"

  # Filter the full hallmark enumeration to rows for this hallmark.
  local z_present=""
  local z_line=""
  local z_h=""
  local z_b=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    z_h="${z_line%% *}"
    z_b="${z_line#* }"
    if test "${z_h}" = "${z_hallmark}"; then
      z_present="${z_present}${z_present:+ }${z_b}"
    fi
  done < "${ZRBFC_PACKAGE_LIST_FILE}"

  test -n "${z_present}" || buc_die "Hallmark not found: ${z_hallmark}"

  echo ""
  printf "  %-10s  %-6s  %s\n" "BASENAME" "EXISTS" "PACKAGE-PATH"
  printf "  %-10s  %-6s  %s\n" "----------" "------" "------------"

  local z_canon=""
  local z_mark=""
  local z_path=""
  for z_canon in \
    "${RBGC_ARK_BASENAME_IMAGE}" \
    "${RBGC_ARK_BASENAME_ABOUT}" \
    "${RBGC_ARK_BASENAME_VOUCH}" \
    "${RBGC_ARK_BASENAME_ATTEST}" \
    "${RBGC_ARK_BASENAME_POUCH}" \
    "${RBGC_ARK_BASENAME_DIAGS}"; do
    z_mark="no"
    case " ${z_present} " in
      *" ${z_canon} "*) z_mark="yes" ;;
    esac
    if test "${z_mark}" = "yes"; then
      z_path="${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${z_canon}"
    else
      z_path="(absent)"
    fi
    printf "  %-10s  %-6s  %s\n" "${z_canon}" "${z_mark}" "${z_path}"
  done

  echo ""
  buc_success "Rekon complete for ${z_hallmark}"
}

rbfl_rekon_reliquary() {
  zrbfl_sentinel

  local -r z_stamp="${BUZ_FOLIO:-}"

  buc_doc_brief "List tool images present under a reliquary stamp's GAR subtree"
  buc_doc_param "stamp" "Reliquary datestamp (e.g., r260327172456)"
  buc_doc_shown || return 0

  test -n "${z_stamp}" || buc_die "Usage: rbw-irr <stamp>"

  buc_step "Authenticating as Director"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}" \
    || buc_die "Director credential not found: ${RBDC_DIRECTOR_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  buc_step "Enumerating tool images under ${RBGL_RELIQUARIES_ROOT}/${z_stamp}/"
  zrbfc_list_packages_capture "${z_token}" "${RBGL_RELIQUARIES_ROOT}"

  # Filter the full reliquary enumeration to rows for this stamp.
  local z_present=""
  local z_line=""
  local z_s=""
  local z_t=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    z_s="${z_line%% *}"
    z_t="${z_line#* }"
    if test "${z_s}" = "${z_stamp}"; then
      z_present="${z_present}${z_present:+ }${z_t}"
    fi
  done < "${ZRBFC_PACKAGE_LIST_FILE}"

  test -n "${z_present}" || buc_die "Reliquary stamp not found: ${z_stamp}"

  echo ""
  printf "  %-10s  %-6s  %s\n" "TOOL" "EXISTS" "PACKAGE-PATH"
  printf "  %-10s  %-6s  %s\n" "----------" "------" "------------"

  local z_canon=""
  local z_mark=""
  local z_path=""
  for z_canon in \
    "${RBGC_RELIQUARY_TOOL_GCLOUD}" \
    "${RBGC_RELIQUARY_TOOL_DOCKER}" \
    "${RBGC_RELIQUARY_TOOL_ALPINE}" \
    "${RBGC_RELIQUARY_TOOL_SYFT}" \
    "${RBGC_RELIQUARY_TOOL_BINFMT}" \
    "${RBGC_RELIQUARY_TOOL_SKOPEO}"; do
    z_mark="no"
    case " ${z_present} " in
      *" ${z_canon} "*) z_mark="yes" ;;
    esac
    if test "${z_mark}" = "yes"; then
      z_path="${RBGL_RELIQUARIES_ROOT}/${z_stamp}/${z_canon}"
    else
      z_path="(absent)"
    fi
    printf "  %-10s  %-6s  %s\n" "${z_canon}" "${z_mark}" "${z_path}"
  done

  echo ""
  buc_success "Rekon complete for ${z_stamp}"
}

rbfl_wrest() {
  zrbfl_sentinel

  local -r z_locator="${BUZ_FOLIO:-}"

  buc_doc_brief "Wrest an image from the registry to local container runtime by locator"
  buc_doc_param "locator" "Image locator in package-path:tag format (e.g. rbi_hm/H/image:H, rbi_rq/r260327172456/syft:r260327172456, rbi_es/eb-anchor:eb-anchor)"
  buc_doc_shown || return 0

  test -n "${z_locator}" || buc_die "Locator parameter required (package-path:tag)"

  case "${z_locator}" in
    *:*) : ;;
    *)   buc_die "Invalid locator format. Expected package-path:tag" ;;
  esac
  local -r z_pkg_path="${z_locator%:*}"
  local -r z_tag="${z_locator##*:}"
  test -n "${z_pkg_path}" || buc_die "Package path is empty in locator"
  test -n "${z_tag}"      || buc_die "Tag is empty in locator"

  buc_step "Authenticating as Director"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}" \
    || buc_die "Director credential not found: ${RBDC_DIRECTOR_RBRA_FILE}"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get OAuth token"

  buc_step "Logging into container registry"
  local -r z_full_ref="${ZRBFC_REGISTRY_HOST}/${ZRBFC_REGISTRY_PATH}/${z_locator}"

  echo "${z_token}" | docker login -u oauth2accesstoken --password-stdin "https://${ZRBFC_REGISTRY_HOST}" \
    || buc_die "Container runtime authentication failed"

  buc_step "Pulling image: ${z_full_ref}"
  docker pull "${z_full_ref}" || buc_die "Image pull failed"

  local z_image_id
  docker inspect --format='{{.Id}}' "${z_full_ref}" > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null \
    || buc_die "Failed to get image ID"
  z_image_id=$(<"${ZRBFC_SCRATCH_FILE}")

  echo ""
  echo "Image wrested: ${z_full_ref}"
  echo "Local image ID: ${z_image_id}"

  buc_success "Image wrest complete"
}

rbfl_audit_hallmarks() {
  zrbfl_sentinel

  buc_doc_brief "Audit hallmarks — list all hallmark identifiers in registry"
  buc_doc_shown || return 0

  buc_step "Authenticating as Director"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}" \
    || buc_die "Director credential not found: ${RBDC_DIRECTOR_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  buc_step "Enumerating hallmarks under ${RBGL_HALLMARKS_ROOT}/"
  zrbfc_list_packages_capture "${z_token}" "${RBGL_HALLMARKS_ROOT}"

  if ! test -s "${ZRBFC_PACKAGE_LIST_FILE}"; then
    buc_info "No hallmarks found under ${RBGL_HALLMARKS_ROOT}/"
    buc_success "Audit complete — 0 hallmarks"
    return 0
  fi

  echo ""
  printf "  %s\n" "HALLMARK"
  printf "  %s\n" "------------------------------"

  local z_count=0
  local z_prev=""
  local z_line=""
  local z_h=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    z_h="${z_line%% *}"
    test -n "${z_h}" || continue
    if test "${z_h}" != "${z_prev}"; then
      printf "  %s\n" "${z_h}"
      buf_write_fact_multi "${z_h}" "${RBCC_fact_ext_audit_hallmark}" "${z_h}"
      z_count=$(( z_count + 1 ))
      z_prev="${z_h}"
    fi
  done < "${ZRBFC_PACKAGE_LIST_FILE}"

  echo ""
  buc_info "Total hallmarks: ${z_count}"
  buc_success "Audit complete"
}

rbfl_audit_reliquaries() {
  zrbfl_sentinel

  buc_doc_brief "Audit reliquaries — list all reliquary stamps in registry"
  buc_doc_shown || return 0

  buc_step "Authenticating as Director"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}" \
    || buc_die "Director credential not found: ${RBDC_DIRECTOR_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  buc_step "Enumerating reliquaries under ${RBGL_RELIQUARIES_ROOT}/"
  zrbfc_list_packages_capture "${z_token}" "${RBGL_RELIQUARIES_ROOT}"

  if ! test -s "${ZRBFC_PACKAGE_LIST_FILE}"; then
    buc_info "No reliquaries found under ${RBGL_RELIQUARIES_ROOT}/"
    buc_success "Audit complete — 0 reliquaries"
    return 0
  fi

  echo ""
  printf "  %s\n" "RELIQUARY-STAMP"
  printf "  %s\n" "------------------------------"

  local z_count=0
  local z_prev=""
  local z_line=""
  local z_s=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    z_s="${z_line%% *}"
    test -n "${z_s}" || continue
    if test "${z_s}" != "${z_prev}"; then
      printf "  %s\n" "${z_s}"
      z_count=$(( z_count + 1 ))
      z_prev="${z_s}"
    fi
  done < "${ZRBFC_PACKAGE_LIST_FILE}"

  echo ""
  buc_info "Total reliquaries: ${z_count}"
  buc_success "Audit complete"
}

rbfl_audit_enshrinements() {
  zrbfl_sentinel

  buc_doc_brief "Audit enshrinements — list all enshrined base anchors in registry"
  buc_doc_shown || return 0

  buc_step "Authenticating as Director"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}" \
    || buc_die "Director credential not found: ${RBDC_DIRECTOR_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  buc_step "Enumerating enshrinements under ${RBGL_ENSHRINES_ROOT}/"
  zrbfc_list_anchors_capture "${z_token}" "${RBGL_ENSHRINES_ROOT}"

  if ! test -s "${ZRBFC_PACKAGE_LIST_FILE}"; then
    buc_info "No enshrinements found under ${RBGL_ENSHRINES_ROOT}/"
    buc_success "Audit complete — 0 enshrinements"
    return 0
  fi

  echo ""
  printf "  %s\n" "ENSHRINE-ANCHOR"
  printf "  %s\n" "------------------------------"

  local z_count=0
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    printf "  %s\n" "${z_line}"
    z_count=$(( z_count + 1 ))
  done < "${ZRBFC_PACKAGE_LIST_FILE}"

  echo ""
  buc_info "Total enshrinements: ${z_count}"
  buc_success "Audit complete"
}


# eof
