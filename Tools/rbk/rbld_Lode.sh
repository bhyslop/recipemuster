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
# Recipe Bottle Lode - fetched-side universal capture (base kind)
#   ensconce — capture an upstream base image into a Lode (Director credentials)
#   divine   — enumerate Lodes / inspect one Lode's members (read-only)
#   banish   — delete a whole Lode (Director credentials)

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBLD_SOURCED:-}" || buc_die "Module rbld multiply sourced - check sourcing hierarchy"
ZRBLD_SOURCED=1

# Source shared Foundry Core module
source "${BASH_SOURCE[0]%/*}/rbfc_FoundryCore.sh"

# Capture-file extension for buf_write_fact_multi (filesystem-as-data-bus).
# Pure literal tinder — one provenance fact-file per captured Lode, keyed by
# stamp: "<stamp>.${RBCC_fact_ext_lode}". Mirrors the roster fact precedent.

######################################################################
# Internal Functions (zrbld_*)

zrbld_kindle() {
  test -z "${ZRBLD_KINDLED:-}" || buc_die "Module rbld already kindled"

  buc_log_args 'Validate Foundry Core is kindled'
  zrbfc_sentinel

  buc_log_args 'Verify Director RBRA file'
  test -n "${RBDC_DIRECTOR_RBRA_FILE:-}" || buc_die "RBDC_DIRECTOR_RBRA_FILE not set"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}"   || buc_die "GCB service env file not found: ${RBDC_DIRECTOR_RBRA_FILE}"

  buc_log_args 'RBGJL ensconce step scripts (same Tools directory)'
  local z_self_dir="${BASH_SOURCE[0]%/*}"
  readonly ZRBLD_RBGJL_STEPS_DIR="${z_self_dir}/rbgjl"
  test -d "${ZRBLD_RBGJL_STEPS_DIR}" || buc_die "RBGJL steps directory not found: ${ZRBLD_RBGJL_STEPS_DIR}"

  buc_log_args 'Define ensconce operation file prefix'
  readonly ZRBLD_ENSCONCE_PREFIX="${BURD_TEMP_DIR}/rbld_ensconce_"

  buc_log_args 'Define divine operation file prefix'
  readonly ZRBLD_DIVINE_PREFIX="${BURD_TEMP_DIR}/rbld_divine_"

  buc_log_args 'Define banish operation file prefix'
  readonly ZRBLD_BANISH_PREFIX="${BURD_TEMP_DIR}/rbld_banish_"

  readonly ZRBLD_KINDLED=1
}

zrbld_sentinel() {
  zrbfc_sentinel
  test "${ZRBLD_KINDLED:-}" = "1" || buc_die "Module rbld not kindled - call zrbld_kindle first"
}

######################################################################
# Internal Helpers (zrbld_*)

# Internal: Submit the two-step ensconce Cloud Build job (skopeo capture +
# docker vouch push). Mirrors zrbfl_inscribe_submit; pool is TETHER because the
# skopeo step fetches upstream bytes over the network from within GCP.
# Args: token origin stamp
zrbld_ensconce_submit() {
  zrbld_sentinel

  local -r z_token="${1:?Token required}"
  local -r z_origin="${2:?Origin required}"
  local -r z_stamp="${3:?Stamp required}"

  buc_step "Constructing ensconce Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBDC_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Step definitions: script|builder|id (skopeo capture, then docker vouch push).
  # Delimiter is | because builder refs contain colons (image tags).
  local -r z_step_defs=(
    "rbgjl01-ensconce-capture.sh|${z_rbfc_tool_skopeo}|ensconce-capture"
    "rbgjl02-assemble-push-vouch.sh|${z_rbfc_tool_docker}|assemble-push-vouch"
  )

  local -r z_steps_file="${ZRBLD_ENSCONCE_PREFIX}steps.json"
  echo "[]" > "${z_steps_file}" || buc_die "Failed to initialize ensconce steps JSON"

  local z_def=""
  local z_script=""
  local z_builder=""
  local z_id=""
  local z_script_path=""
  local z_body=""
  local z_body_file=""
  local z_escaped_file=""
  local z_steps_built=""
  local z_index=0
  for z_def in "${z_step_defs[@]}"; do
    IFS='|' read -r z_script z_builder z_id <<<"${z_def}"
    z_script_path="${ZRBLD_RBGJL_STEPS_DIR}/${z_script}"
    z_body_file="${ZRBLD_ENSCONCE_PREFIX}${z_index}_body.txt"
    z_escaped_file="${ZRBLD_ENSCONCE_PREFIX}${z_index}_escaped.txt"
    z_steps_built="${ZRBLD_ENSCONCE_PREFIX}${z_index}_steps.json"

    test -f "${z_script_path}" || buc_die "Ensconce step script not found: ${z_script_path}"

    zrbfc_write_script_body "${z_script_path}" "${z_body_file}" \
      || buc_die "Failed to read ensconce step script: ${z_script_path}"
    z_body=$(<"${z_body_file}")
    test -n "${z_body}" || buc_die "Empty ensconce script body: ${z_script_path}"

    printf '#!/bin/bash\n%s' "${z_body}" > "${z_escaped_file}" \
      || buc_die "Failed to write escaped ensconce script body for ${z_id}"

    jq \
      --arg name "${z_builder}" \
      --arg id "${z_id}" \
      --rawfile script "${z_escaped_file}" \
      '. + [{name: $name, id: $id, script: $script}]' \
      "${z_steps_file}" > "${z_steps_built}" \
      || buc_die "Failed to append ensconce step ${z_id}"
    mv "${z_steps_built}" "${z_steps_file}" \
      || buc_die "Failed to update ensconce steps JSON for ${z_id}"

    z_index=$((z_index + 1))
  done

  buc_log_args "Composing ensconce Build resource JSON"
  local -r z_build_file="${ZRBLD_ENSCONCE_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps      "${z_steps_file}" \
    --arg zjq_sa               "${z_mason_sa}" \
    --arg zjq_gar_host         "${z_gar_host}" \
    --arg zjq_gar_path         "${z_gar_path}" \
    --arg zjq_lodes_root       "${RBGL_LODES_ROOT}" \
    --arg zjq_tag_base         "${RBGC_LODE_TAG_BASE}" \
    --arg zjq_tag_vouch        "${RBGC_LODE_TAG_VOUCH}" \
    --arg zjq_tag_digest       "${RBGC_LODE_TAG_DIGEST_PREFIX}" \
    --arg zjq_trust_grade      "${RBGC_LODE_TRUST_VERIFIED}" \
    --arg zjq_vouch_schema     "${RBGC_LODE_VOUCH_SCHEMA}" \
    --arg zjq_acquired_by      "${RBGD_MASON_EMAIL}" \
    --arg zjq_origin_1         "${z_origin}" \
    --arg zjq_stamp_1          "${z_stamp}" \
    --arg zjq_pool             "${RBDC_POOL_TETHER}" \
    --arg zjq_timeout          "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGL_GAR_HOST:          $zjq_gar_host,
        _RBGL_GAR_PATH:          $zjq_gar_path,
        _RBGL_LODES_ROOT:        $zjq_lodes_root,
        _RBGL_TAG_BASE:          $zjq_tag_base,
        _RBGL_TAG_VOUCH:         $zjq_tag_vouch,
        _RBGL_TAG_DIGEST_PREFIX: $zjq_tag_digest,
        _RBGL_TRUST_GRADE:       $zjq_trust_grade,
        _RBGL_VOUCH_SCHEMA:      $zjq_vouch_schema,
        _RBGL_ACQUIRED_BY:       $zjq_acquired_by,
        _RBGL_IMAGE_1_ORIGIN:    $zjq_origin_1,
        _RBGL_IMAGE_2_ORIGIN:    "",
        _RBGL_IMAGE_3_ORIGIN:    "",
        _RBGL_LODE_1_STAMP:      $zjq_stamp_1,
        _RBGL_LODE_2_STAMP:      "",
        _RBGL_LODE_3_STAMP:      ""
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_build_file}" \
    || buc_die "Failed to compose ensconce build JSON"

  buc_log_args "Ensconce build JSON: ${z_build_file}"

  rbrd_check "${z_token}"

  buc_step "Submitting ensconce Cloud Build"
  rbuh_json "POST" "${ZRBFC_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "lode_build_create" "${z_build_file}"
  rbuh_require_ok "Ensconce build submission" "lode_build_create"

  local z_build_id=""
  z_build_id=$(rbuh_json_field_capture "lode_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBFC_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBFC_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Ensconce Cloud Build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbfc_wait_build_completion "${ZRBFC_BUILD_POLL_CEILING_ENSHRINE}" "Ensconce"
}

# Internal: Extract per-Lode provenance envelopes from the completed ensconce
# build and write one capture-file per Lode. buildStepOutputs[0] is the base64
# JSON authored by the skopeo capture step (step 0); the docker vouch step
# writes no /builder/outputs/output.
zrbld_ensconce_extract() {
  zrbld_sentinel

  buc_step "Extracting capture results from build step outputs"

  local -r z_b64_file="${ZRBLD_ENSCONCE_PREFIX}output_b64.txt"
  local -r z_output_file="${ZRBLD_ENSCONCE_PREFIX}output.json"

  jq -r '.results.buildStepOutputs[0] // empty' "${ZRBFC_BUILD_STATUS_FILE}" \
    > "${z_b64_file}" || buc_die "Failed to extract buildStepOutputs from build result"
  test -s "${z_b64_file}" || buc_die "No buildStepOutputs in build result — capture step produced no output"

  rbgo_base64_decode_file_to_file "${z_b64_file}" "${z_output_file}" \
    || buc_die "Failed to decode buildStepOutputs base64"
  test -s "${z_output_file}" || buc_die "Empty decoded buildStepOutputs"

  buc_log_args "Ensconce output:"
  buc_log_pipe < "${z_output_file}"

  local z_n=""
  local z_slot_key=""
  local z_stamp=""
  local z_vouch_file=""
  local z_vouch=""
  for z_n in 1 2 3; do
    z_slot_key="slot_${z_n}"
    z_stamp=$(jq -r ".${z_slot_key}.stamp // empty" "${z_output_file}") || z_stamp=""
    test -n "${z_stamp}" || continue

    z_vouch_file="${ZRBLD_ENSCONCE_PREFIX}${z_n}_vouch.json"
    jq -c ".${z_slot_key}.vouch" "${z_output_file}" > "${z_vouch_file}" \
      || buc_die "Failed to extract vouch envelope for ${z_slot_key}"
    z_vouch=$(<"${z_vouch_file}")
    test -n "${z_vouch}" || buc_die "Empty vouch envelope for ${z_slot_key}"

    buf_write_fact_multi "${z_stamp}" "${RBCC_fact_ext_lode}" "${z_vouch}"
    buc_success "Ensconced Lode ${z_stamp} — capture-file ${z_stamp}.${RBCC_fact_ext_lode}"
  done
}

######################################################################
# External Functions (rbld_*)

rbld_ensconce() {
  zrbld_sentinel

  buc_doc_brief "Ensconce an upstream base image into a Lode (parallel rbi_ld capture)"
  buc_doc_param "vessel" "Vessel sigil or path to vessel directory declaring the base ORIGIN"
  buc_doc_shown || return 0

  # Resolve vessel argument (sigil or path) and load.
  zrbfc_resolve_vessel "${BUZ_FOLIO:-}"
  local -r z_vessel_dir=$(<"${ZRBFC_VESSEL_RESOLVED_DIR_FILE}")
  test -n "${z_vessel_dir}" || buc_die "Empty resolved vessel path"
  zrbfc_load_vessel "${z_vessel_dir}"

  # Resolve the single base ORIGIN. Base-kind ensconce captures one base per
  # Lode per invocation; multi-base vessels are dispatched per base. Every real
  # vessel declares exactly one base slot today.
  local z_origin=""
  local z_origin_count=0
  local z_n=""
  local z_origin_var=""
  local z_slot_origin=""
  for z_n in 1 2 3; do
    z_origin_var="RBRV_IMAGE_${z_n}_ORIGIN"
    z_slot_origin="${!z_origin_var:-}"
    test -n "${z_slot_origin}" || continue
    z_origin="${z_slot_origin}"
    z_origin_count=$((z_origin_count + 1))
  done

  test "${z_origin_count}" -ne 0 \
    || buc_die "Vessel '${RBRV_SIGIL}' declares no upstream base-image slot (RBRV_IMAGE_n_ORIGIN)"
  test "${z_origin_count}" -eq 1 \
    || buc_die "Vessel '${RBRV_SIGIL}' declares ${z_origin_count} base slots; base-kind ensconce captures one base per Lode — invoke per base"

  # Reject producer-vessel pins: an origin naming a local vessel directory is a
  # made-side hallmark-pin (bind/airgap), not an upstream base to capture.
  test ! -d "${RBRR_VESSEL_DIR}/${z_origin}" \
    || buc_die "Origin '${z_origin}' names a producer vessel — base-Lode capture is for upstream bases, not hallmark-pins"

  buc_info "Ensconce base: ${z_origin}"

  # Resolve tool images from reliquary (skopeo for capture, docker for vouch).
  zrbfc_resolve_tool_images

  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Mint the Lode stamp on the host: <kind-letter><YYMMDDHHMMSS>. The host owns
  # the stamp so the touchmark is known before the build for the capture-file.
  local -r z_stamp="${RBGC_LODE_KIND_BASE}${BURD_NOW_STAMP:2:6}${BURD_NOW_STAMP:9:6}"
  buc_info "Lode: ${RBGL_LODES_ROOT}/${z_stamp}"

  zrbld_ensconce_submit "${z_token}" "${z_origin}" "${z_stamp}"
  zrbld_ensconce_extract

  buc_success "Ensconce complete: ${z_origin} -> ${RBGL_LODES_ROOT}/${z_stamp}"
}

rbld_divine() {
  zrbld_sentinel

  local -r z_touchmark="${BUZ_FOLIO:-}"

  buc_doc_brief "Divine Lodes — enumerate all Lodes, or inspect one Lode's members (read-only)"
  buc_doc_oparm "touchmark" "Lode stamp to inspect (e.g., b260602120000); omit to enumerate all Lodes"
  buc_doc_shown || return 0

  buc_step "Authenticating as Director"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}" \
    || buc_die "Director credential not found: ${RBDC_DIRECTOR_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  if test -z "${z_touchmark}"; then
    buc_step "Enumerating Lodes under ${RBGL_LODES_ROOT}/"
    zrbfc_list_anchors_capture "${z_token}" "${RBGL_LODES_ROOT}"

    if ! test -s "${ZRBFC_PACKAGE_LIST_FILE}"; then
      buc_info "No Lodes found under ${RBGL_LODES_ROOT}/"
      buc_success "Divine complete — 0 Lodes"
      return 0
    fi

    # Kind-letter legend, printed once so rows carry no repeated per-row column.
    # A touchmark's leading letter is its kind (b260602075327 -> base); the
    # reader decodes the prefix from this key. One entry per implemented kind.
    local -r z_kind_fmt="    %-3s %-10s %s\n"
    echo ""
    printf "  Kinds (touchmark prefix):\n"
    printf "${z_kind_fmt}" "${RBGC_LODE_KIND_BASE}" "base" "upstream OCI image, consumed as a FROM line"

    # Load the touchmark list fully before iterating: the per-Lode tags fetch
    # spawns curl (via rbuh), and a child touching stdin would consume the
    # loop's remaining input. Load-then-iterate keeps that FD closed.
    local z_touchmarks=()
    local z_touch=""
    while IFS= read -r z_touch || test -n "${z_touch}"; do
      test -n "${z_touch}" || continue
      z_touchmarks+=("${z_touch}")
    done < "${ZRBFC_PACKAGE_LIST_FILE}"

    local -r z_row_fmt="  %-15s %s\n"
    echo ""
    printf "${z_row_fmt}" "TOUCHMARK" "IMAGE"
    printf "${z_row_fmt}" "---------------" "--------------------------------------"

    local z_idx=0
    local z_pkg=""
    local z_pkg_encoded=""
    local z_tags_url=""
    local z_enum_infix=""
    local z_resp_file=""
    local z_image_file=""
    local z_image=""
    for z_idx in "${!z_touchmarks[@]}"; do
      z_touch="${z_touchmarks[$z_idx]}"

      # One tags-list per Lode. IMAGE is the unsprued fingerprint tag
      # <sanitized-origin>-<sha10>; it is located via the sha10 taken from the
      # rbi_sha256-<hex> member tag, so Director semantic names (also unsprued)
      # cannot masquerade as the fingerprint. Per-Lode infix preserves each
      # response for forensics.
      z_pkg="${RBGL_LODES_ROOT}/${z_touch}"
      z_pkg_encoded="${z_pkg//\//%2F}"
      z_tags_url="${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages/${z_pkg_encoded}/tags?pageSize=1000"
      z_enum_infix="rbld_divine_enum_${z_idx}"
      rbuh_json "GET" "${z_tags_url}" "${z_token}" "${z_enum_infix}"
      rbuh_require_ok "List tags for Lode ${z_touch}" "${z_enum_infix}"
      z_resp_file="${ZRBUH_PREFIX}${z_enum_infix}${ZRBUH_POSTFIX_JSON}"

      z_image_file="${ZRBLD_DIVINE_PREFIX}enum_${z_idx}_image.txt"
      jq -r --arg dp "${RBGC_LODE_TAG_DIGEST_PREFIX}" '
        [.tags[]?.name | sub(".*/tags/"; "")] as $names
        | ([$names[] | select(startswith($dp)) | ltrimstr($dp)[0:10]][0]) as $sha10
        | ([$names[] | select((startswith("rbi_") | not) and ($sha10 != null) and endswith("-" + $sha10))][0]) // "(no fingerprint)"
      ' "${z_resp_file}" > "${z_image_file}" \
        || buc_die "Failed to extract fingerprint for Lode ${z_touch}"
      z_image=$(<"${z_image_file}")
      test -n "${z_image}" || buc_die "Empty fingerprint extraction for Lode ${z_touch}"

      printf "${z_row_fmt}" "${z_touch}" "${z_image}"
    done
    echo ""
    buc_info "Total Lodes: ${#z_touchmarks[@]}"
    buc_success "Divine complete"
    return 0
  fi

  # Inspect depth: list the member tags on one Lode package.
  buc_step "Inspecting Lode ${RBGL_LODES_ROOT}/${z_touchmark}"
  local -r z_pkg="${RBGL_LODES_ROOT}/${z_touchmark}"
  local -r z_pkg_encoded="${z_pkg//\//%2F}"
  local -r z_tags_url="${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages/${z_pkg_encoded}/tags?pageSize=1000"
  local -r z_tags_infix="rbld_divine_tags"

  rbuh_json "GET" "${z_tags_url}" "${z_token}" "${z_tags_infix}"
  rbuh_require_ok "List tags for Lode ${z_touchmark}" "${z_tags_infix}"

  local -r z_resp_file="${ZRBUH_PREFIX}${z_tags_infix}${ZRBUH_POSTFIX_JSON}"
  local -r z_tags_file="${ZRBLD_DIVINE_PREFIX}tags.txt"
  jq -r '.tags[]?.name | sub(".*/tags/"; "")' "${z_resp_file}" > "${z_tags_file}" \
    || buc_die "Failed to extract member tags for Lode ${z_touchmark}"

  if ! test -s "${z_tags_file}"; then
    buc_die "No member tags found under ${z_pkg} — Lode not present in registry"
  fi

  echo ""
  printf "  %s\n" "MEMBER-TAG"
  printf "  %s\n" "------------------------------"
  local z_count=0
  local z_tag=""
  while IFS= read -r z_tag || test -n "${z_tag}"; do
    test -n "${z_tag}" || continue
    printf "  %s\n" "${z_tag}"
    z_count=$((z_count + 1))
  done < "${z_tags_file}"
  echo ""
  buc_info "Total members: ${z_count}"
  buc_success "Divine complete — Lode ${z_touchmark}"
}

rbld_banish() {
  zrbld_sentinel

  local -r z_touchmark="${BUZ_FOLIO:-}"

  buc_doc_brief "Banish a Lode — delete the whole rbi_ld/<touchmark> GAR package"
  buc_doc_param "touchmark" "Lode stamp to delete (e.g., b260602120000)"
  buc_doc_shown || return 0

  test -n "${z_touchmark}" || buc_die "Touchmark parameter required"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  local -r z_pkg="${RBGL_LODES_ROOT}/${z_touchmark}"
  local -r z_pkg_encoded="${z_pkg//\//%2F}"

  # Verify presence before delete so banish reports a clean not-found.
  buc_step "Verifying Lode present: ${z_pkg}"
  local -r z_tags_url="${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages/${z_pkg_encoded}/tags?pageSize=1"
  local -r z_probe_infix="rbld_banish_probe"
  rbuh_json "GET" "${z_tags_url}" "${z_token}" "${z_probe_infix}"
  rbuh_require_ok "Probe Lode ${z_touchmark}" "${z_probe_infix}"

  local z_tag_count=""
  z_tag_count=$(rbuh_json_field_capture "${z_probe_infix}" '(.tags // []) | length') \
    || buc_die "Failed to count tags for ${z_pkg}"
  test "${z_tag_count}" -gt 0 \
    || buc_die "No Lode found at ${z_pkg} — nothing to banish"

  buc_require "Will banish the whole Lode ${z_pkg} (single packages delete)" "yes"

  # Single packages delete removes the package and all its member versions/tags
  # atomically. DELETE returns a long-running operation; trust 200 as accepted.
  buc_step "Deleting Lode package: ${z_pkg}"
  local -r z_del_url="${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages/${z_pkg_encoded}"
  local -r z_del_infix="rbld_banish_del"
  rbuh_json "DELETE" "${z_del_url}" "${z_token}" "${z_del_infix}"
  rbuh_require_ok "Delete Lode package ${z_pkg}" "${z_del_infix}"

  echo ""
  buc_success "Lode banished: ${z_touchmark}"
}

# eof
