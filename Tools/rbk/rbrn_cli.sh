#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
# RBRN CLI - Command line interface for RBRN nameplate regime operations
#
# Differential furnish: light deps (validate, render) always loaded;
# heavy deps (survey, audit) loaded only when needed.

set -euo pipefail

# Source dependencies
source "${BURD_BUK_DIR}/buc_command.sh" || exit 1
source "${BURD_BUK_DIR}/buym_yelp.sh" || buc_die_now "Failed to source buym_yelp.sh"

######################################################################
# Internal Helpers

# Local image presence for the fleet survey — status only: 0 when the runtime
# holds the image, 1 otherwise. The runtime's stderr, a miss's own message
# among it, stands in a temp file keyed by the survey row and role.
zrbrn_image_present_flint_predicate() {
  local -r z_runtime="${1:-}"
  local -r z_image="${2:-}"
  local -r z_key="${3:-}"
  test -n "${z_runtime}" || return 1
  test -n "${z_image}" || return 1
  test -n "${z_key}" || return 1

  "${z_runtime}" image inspect "${z_image}" \
    >/dev/null 2>"${BURD_TEMP_DIR}/rbrn_survey_${z_key}_inspect_stderr.txt" || return 1
  return 0
}

# Fleet info table — non-opinionated display of all nameplate configuration
# Requires: RBCC, RBGC, RBGD kindled + RBRR loaded (via differential furnish)
zrbrn_fleet_survey() {
  zrbcc_sentinel
  zrbgc_sentinel
  zrbgd_sentinel
  zrbrr_sentinel

  local -r z_gar_base="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}/${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"
  local -r z_row_fmt="%-10s %-8s %6s %6s  %-17s %-14s %-14s  %4s %3s %3s\n"
  echo "" || buc_die_now "Failed to echo survey lead line"
  printf "${z_row_fmt}" \
    "Moniker" "Entry" "WS" "Enc" "Subnet" "Sentry IP" "Bottle IP" "Arm" "Snt" "Btl" \
    || buc_die_now "Failed to printf survey header"
  printf "${z_row_fmt}" \
    "--------" "-----" "------" "------" "-----------------" "--------------" "--------------" "----" "---" "---" \
    || buc_die_now "Failed to printf survey rule"

  local -r z_sv_files=("${RBCC_moorings_dir}/"*"/${RBCC_rbrn_file}")
  local z_sv_i=""
  for z_sv_i in "${!z_sv_files[@]}"; do
    test -f "${z_sv_files[${z_sv_i}]}" || continue
    (
      source "${z_sv_files[${z_sv_i}]}" || buc_die_now "Failed to source: ${z_sv_files[${z_sv_i}]}"

      local -r z_sentry_img="${z_gar_base}/${RBGL_HALLMARKS_ROOT}/${RBRN_SENTRY_HALLMARK}/${RBGC_ARK_BASENAME_IMAGE}:${RBRN_SENTRY_HALLMARK}"
      local -r z_bottle_img="${z_gar_base}/${RBGL_HALLMARKS_ROOT}/${RBRN_BOTTLE_HALLMARK}/${RBGC_ARK_BASENAME_IMAGE}:${RBRN_BOTTLE_HALLMARK}"

      local z_sentry_local="--"
      local z_bottle_local="--"
      if command -v "${RBRN_RUNTIME}" >/dev/null; then
        if zrbrn_image_present_flint_predicate "${RBRN_RUNTIME}" "${z_sentry_img}" "${z_sv_i}_sentry"; then
          z_sentry_local="ok"
        fi
        if zrbrn_image_present_flint_predicate "${RBRN_RUNTIME}" "${z_bottle_img}" "${z_sv_i}_bottle"; then
          z_bottle_local="ok"
        fi
      else
        z_sentry_local="??"
        z_bottle_local="??"
      fi

      # Armament — readiness-to-charge, distinct from the local-image-presence
      # columns above. Per-field so a half-armed transient (rbw-nm arms one
      # field at a time) reads honestly: "SB" both, "S-"/"-B" half, "--" vacant.
      local z_arm_sentry="-"
      local z_arm_bottle="-"
      if rbrn_hallmark_armed_flint_predicate "${RBRN_SENTRY_HALLMARK}"; then
        z_arm_sentry="S"
      fi
      if rbrn_hallmark_armed_flint_predicate "${RBRN_BOTTLE_HALLMARK}"; then
        z_arm_bottle="B"
      fi

      local z_ws_port="${RBRN_ENTRY_PORT_WORKSTATION:-}"
      local z_enc_port="${RBRN_ENTRY_PORT_ENCLAVE:-}"
      if test "${RBRN_ENTRY_MODE}" != "rbnne_enabled"; then
        z_ws_port="-"
        z_enc_port="-"
      fi

      printf "${z_row_fmt}" \
        "${RBRN_MONIKER}" "${RBRN_ENTRY_MODE}" "${z_ws_port}" "${z_enc_port}" \
        "${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" \
        "${RBRN_ENCLAVE_SENTRY_IP}" "${RBRN_ENCLAVE_BOTTLE_IP}" \
        "${z_arm_sentry}${z_arm_bottle}" \
        "${z_sentry_local}" "${z_bottle_local}" || buc_die_now "Failed to printf survey row"
    ) || buc_die_now "Survey isolation failed for: ${z_sv_files[${z_sv_i}]}"
  done
  echo "" || buc_die_now "Failed to echo survey closing line"
}

######################################################################
# Command Functions

# Command: validate - enrollment-based validation report
rbrn_validate() {
  buc_doc_brief "Validate RBRN nameplate regime configuration via enrollment report"
  buc_doc_shown || return 0

  if test -z "${BUZ_FOLIO:-}"; then
    rbrn_list
    buc_die_now "Nameplate moniker required"
  fi
  buc_step "Validating RBRN nameplate regime"
  buv_report RBRN "Nameplate Regime"
  buc_step "RBRN nameplate valid"
}

# Command: render - diagnostic display
rbrn_render() {
  buc_doc_brief "Display diagnostic view of RBRN nameplate regime configuration"
  buc_doc_shown || return 0

  if test -z "${BUZ_FOLIO:-}"; then
    rbrn_list
    buc_die_now "Nameplate moniker required"
  fi
  local -r z_nameplate_file="${RBCC_moorings_dir}/${RBCC_nameplate_sprue}${BUZ_FOLIO}/${RBCC_rbrn_file}"
  buv_render RBRN "RBRN - Recipe Bottle Regime Nameplate" "${z_nameplate_file}"
}

# Command: survey - fleet info table across all nameplates
rbrn_survey() {
  buc_doc_brief "Survey nameplate status — display the configuration table for every nameplate"
  buc_doc_shown || return 0

  zrbrn_fleet_survey
}

# Command: audit - survey display then preflight validation
rbrn_audit() {
  buc_doc_brief "Validate all nameplates — survey every nameplate, then run cross-nameplate preflight validation"
  buc_doc_shown || return 0

  zrbrn_fleet_survey
  # After the survey: the kindle locks the nameplate names the survey's subshells source
  test "${ZRBRN_KINDLED:-}" = "1" || zrbrn_kindle
  rbrn_preflight
  buc_step "Cross-nameplate audit passed"

  buc_step "Full audit passed (nameplates)"
}

# Command: list - show available nameplate monikers
rbrn_list() {
  buc_doc_brief "List available nameplate monikers"
  buc_doc_shown || return 0

  test "${ZRBRN_KINDLED:-}" = "1" || zrbrn_kindle
  local z_monikers
  z_monikers=$(rbrn_list_capture) || buc_die_now "No nameplates found"
  local z_moniker_roll=()
  read -r -a z_moniker_roll <<< "${z_monikers}" || buc_die_now "Failed to split the nameplate listing"
  buc_step "Available nameplates:"
  local z_i=0
  for z_i in "${!z_moniker_roll[@]}"; do
    buc_bare "        ${z_moniker_roll[${z_i}]}"
  done
}

######################################################################
# Furnish and Main

zrbrn_furnish() {
  buc_doc_env_row "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_done || return 0

  local -r z_command="${1:-}"

  # Light sources (always)
  local -r z_rbk_kit_dir="${BASH_SOURCE[0]%/*}"
  source "${BURD_BUK_DIR}/bubc_constants.sh" || buc_die_now "Failed to source bubc_constants.sh"
  source "${BURD_BUK_DIR}/buv_validation.sh" || buc_die_now "Failed to source buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh" || buc_die_now "Failed to source burd_regime.sh"
  source "${BURD_BUK_DIR}/bupr_regime.sh" || buc_die_now "Failed to source bupr_regime.sh"
  source "${z_rbk_kit_dir}/rbcc_constants.sh" || buc_die_now "Failed to source rbcc_constants.sh"
  source "${z_rbk_kit_dir}/rbrn_regime.sh" || buc_die_now "Failed to source rbrn_regime.sh"

  # Heavy sources (survey/audit only)
  case "${z_command}" in
    rbrn_survey|rbrn_audit)
      source "${z_rbk_kit_dir}/rbrr_regime.sh" || buc_die_now "Failed to source rbrr_regime.sh"
      source "${z_rbk_kit_dir}/rbrd_regime.sh" || buc_die_now "Failed to source rbrd_regime.sh"
      source "${z_rbk_kit_dir}/rbdc_derived.sh" || buc_die_now "Failed to source rbdc_derived.sh"
      source "${z_rbk_kit_dir}/rbgc_constants.sh" || buc_die_now "Failed to source rbgc_constants.sh"
      source "${z_rbk_kit_dir}/rbgl_layout.sh" || buc_die_now "Failed to source rbgl_layout.sh"
      source "${z_rbk_kit_dir}/rbgd_depot.sh" || buc_die_now "Failed to source rbgd_depot.sh"
      source "${z_rbk_kit_dir}/rbgo_oauth.sh" || buc_die_now "Failed to source rbgo_oauth.sh"
      ;;
    rbrn_mitre)
      # rbrn_mitre needs only the fact-name constants (rbgc) and the express-or-chain
      # footing (buf_fact) — not the google/GAR stack survey/audit pulls.
      source "${z_rbk_kit_dir}/rbgc_constants.sh" || buc_die_now "Failed to source rbgc_constants.sh"
      source "${BURD_BUK_DIR}/buf_fact.sh" || buc_die_now "Failed to source buf_fact.sh"
      ;;
  esac

  # Light kindles (always)
  zbuv_kindle
  zburd_kindle
  zburd_enforce
  zbupr_kindle
  zrbcc_kindle
  test "${z_rbk_kit_dir}" = "${RBCC_kit_dir}" || buc_die_now "z_rbk_kit_dir mismatch: ${z_rbk_kit_dir} != ${RBCC_kit_dir}"

  # Heavy kindles (survey/audit only)
  case "${z_command}" in
    rbrn_survey|rbrn_audit)
      source "${RBCC_rbrr_file}" || buc_die_now "Failed to source ${RBCC_rbrr_file}"
      source "${RBCC_rbrd_file}" || buc_die_now "Failed to source ${RBCC_rbrd_file}"
      zrbgc_kindle
      zrbgl_kindle
      zrbrr_kindle
      zrbrd_kindle
      zrbrr_enforce
      zrbrd_enforce
      zrbdc_kindle
      zrbgo_kindle
      zrbgd_kindle
      ;;
    rbrn_mitre)
      # rbgc holds RBF_FACT_HALLMARK; it is standalone (pure readonly constants).
      # The `mitre` is rbrn's own, so rbrn kindles with no nameplate loaded.
      zrbgc_kindle
      zrbrn_kindle
      ;;
  esac

  # If BUZ_FOLIO is set, load and kindle the specified nameplate — EXCEPT for
  # rbrn_mitre, which addresses a target nameplate by moniker and rewrites one
  # RBRN_*_HALLMARK line WITHOUT loading it (loading enforces the regime, which
  # would reject a nameplate whose hallmark field is still blank — the very state
  # the `mitre` fills; mirrors feoff, which never loads the vessel it rewrites).
  if test -n "${BUZ_FOLIO:-}" && test "${z_command}" != "rbrn_mitre"; then
    local -r z_nameplate_file="${RBCC_moorings_dir}/${RBCC_nameplate_sprue}${BUZ_FOLIO}/${RBCC_rbrn_file}"
    test -f "${z_nameplate_file}" || buc_die_now "Nameplate not found: ${z_nameplate_file}"
    source "${z_nameplate_file}" || buc_die_now "Failed to source nameplate: ${z_nameplate_file}"
    zrbrn_kindle
    zrbrn_enforce
  fi
}

buc_execute rbrn_ "Recipe Bottle Nameplate Regime" zrbrn_furnish "$@"

# eof
