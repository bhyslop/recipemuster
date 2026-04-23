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
# Recipe Bottle Foundry Ledger - inscribe, jettison, abjure, rekon, and tally operations
# Director credentials for inscribe, jettison, abjure, rekon; retriever credentials for tally

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
    --arg zjq_reliquary    "${RBRR_CLOUD_PREFIX}${z_reliquary}" \
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

  buf_write_fact "${RBF_FACT_RELIQUARY}" "${z_reliquary}"

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
    "${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${z_moniker}/manifests/${z_tag}" \
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
  buc_doc_brief "Abjure a hallmark — delete all GAR packages under hallmarks/<hallmark>/"
  buc_doc_param "vessel" "Vessel sigil or path (retained for AAK signature; drops in AAL)"
  buc_doc_param "hallmark" "Full hallmark (e.g., c260305133650-r260305160530)"
  buc_doc_param "--force" "Optional: skip confirmation prompt"
  buc_doc_shown || return 0

  # Vessel resolution kept for AAK signature compatibility — vessel no longer
  # appears in the registry path, but the load still validates the moniker.
  zrbfc_resolve_vessel "${1:-}"
  local -r z_vessel_dir=$(<"${ZRBFC_VESSEL_RESOLVED_DIR_FILE}")
  test -n "${z_vessel_dir}" || buc_die "Empty resolved vessel path"
  zrbfc_load_vessel "${z_vessel_dir}"

  test -n "${z_hallmark}" || buc_die "Hallmark parameter required"

  local z_skip_confirm=false
  if test "${z_force}" = "--force"; then
    z_skip_confirm=true
  fi

  buc_step "Authenticating as Director"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  # Enumerate packages under hallmarks/<hallmark>/ via GAR REST API.
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

  # Confirm abjuration unless --force
  if test "${z_skip_confirm}" = "false"; then
    local z_confirm_msg="Will abjure ${z_count} packages under ${z_subtree}:"
    local z_pkg_path=""
    while IFS= read -r z_pkg_path || test -n "${z_pkg_path}"; do
      z_confirm_msg="${z_confirm_msg}\n  - ${z_pkg_path}"
    done < "${z_pkg_file}"
    buc_require "${z_confirm_msg}" "yes"
  fi

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

  buc_doc_brief "List hallmarks across all vessels with health status"
  buc_doc_shown || return 0

  # Stubbed in ₢A_AAK Notch 2b. The original implementation enumerated tags
  # under per-vessel registry paths and case-matched suffix patterns, both of
  # which dissolved with the GAR categorical-layout migration. The rewrite
  # (enumerate hallmarks via GAR REST list-packages, classify by basename
  # presence) is architecturally substantial and lands in pace ₢A_AAO.
  buc_die "rbfl_tally: rewrite pending in ₢A_AAO — see RBSCL spec and Tools/rbk/rbfl_FoundryLedger.sh history at commit a9e95201 for the legacy implementation"
}

rbfl_rekon() {
  zrbfl_sentinel

  local z_moniker="${1:-}"

  buc_doc_brief "List all tags for a vessel package in the registry"
  buc_doc_param "moniker" "Vessel moniker (e.g., rbev-sentry-deb-tether)"
  buc_doc_shown || return 0

  if test -z "${z_moniker}"; then
    local z_sigils
    z_sigils=$(rbrv_list_capture 2>/dev/null) || true
    buc_warn "Vessel moniker parameter required"
    if test -n "${z_sigils}"; then
      buc_bare "  Available vessels:"
      local z_s=""
      for z_s in ${z_sigils}; do
        buc_bare "    ${z_s}"
      done
    fi
    buc_die "Usage: rbw-ir <vessel-moniker>"
  fi

  buc_step "Authenticating as Director"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  buc_step "Fetching tags for ${z_moniker}"
  local z_tags_file="${BURD_TEMP_DIR}/rbfl_rekon_tags.json"
  local z_stderr_file="${BURD_TEMP_DIR}/rbfl_rekon_stderr.txt"
  curl -sL \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    "${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${z_moniker}/tags/list" \
    > "${z_tags_file}" 2>"${z_stderr_file}" \
    || buc_die "Failed to fetch tags for ${z_moniker} — see ${z_stderr_file}"

  if jq -e '.errors' "${z_tags_file}" >/dev/null 2>&1; then
    local z_err
    jq -r '.errors[0].message // "Unknown error"' "${z_tags_file}" > "${ZRBFC_SCRATCH_FILE}" \
      || buc_die "Failed to extract error message from registry response"
    z_err=$(<"${ZRBFC_SCRATCH_FILE}")
    buc_die "Registry API error for ${z_moniker}: ${z_err}"
  fi

  local z_all_tags_file="${BURD_TEMP_DIR}/rbfl_rekon_all_tags.txt"
  jq -r '.tags[]? // empty' "${z_tags_file}" > "${z_all_tags_file}" \
    || buc_die "Failed to extract tags"

  local z_count
  z_count=$(wc -l < "${z_all_tags_file}" | tr -d ' ')

  printf "\nVessel: %s  (%s tags)\n\n" "${z_moniker}" "${z_count}"
  if test -s "${z_all_tags_file}"; then
    sort "${z_all_tags_file}"
  else
    echo "  (no tags)"
  fi

  buc_success "Rekon complete"
}


# eof
