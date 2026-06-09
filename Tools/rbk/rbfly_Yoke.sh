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
# Recipe Bottle Foundry Ledger - yoke cluster (guard-free, sourced by rbflk_):
# validate a reliquary stamp against GAR, then rewrite RBRV_RELIQUARY across every
# vessel's rbrv.env (Director credentials).

set -euo pipefail

######################################################################
# Yoke (rbfl_*)

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

  rbuh_json "GET" "${z_list_url}" "${z_token}" "${z_list_infix}"
  rbuh_require_ok "List reliquary packages" "${z_list_infix}"

  local -r z_resp_file="${ZRBUH_PREFIX}${z_list_infix}${ZRBUH_POSTFIX_JSON}"
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
    "${RBGC_RELIQUARY_TOOL_GCRANE}"
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

  # Derive count and roster from the expected array so a cohort change (a tool
  # added or evicted) keeps these messages accurate without a hand-edit.
  local z_roster=""
  local z_r=""
  for z_r in "${z_expected[@]}"; do z_roster="${z_roster}${z_roster:+, }${z_r}"; done

  test -z "${z_missing}" || buc_die "Reliquary stamp '${z_stamp}' not found in Depot — expected ${#z_expected[@]} tool images under ${z_rqy_subtree}; missing: ${z_missing}. Re-run tt/rbw-dI.DirectorInscribesReliquary.sh to mint a fresh reliquary, or verify the stamp spelling."
  buc_info "Reliquary valid — all ${#z_expected[@]} tool images present (${z_roster})"

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

# eof
