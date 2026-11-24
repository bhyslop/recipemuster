#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# RBVQ CLI - the Qualification Regime's doors (veiled). Dispatched by the
# hierophant's own workbench (rbvw_workbench.sh) via its own zipper
# (rbvz_zipper.sh), never the shipped rbw workbench.
#
# The file is rbrq.env beside the station file the dispatch names
# (BURD_STATION_FILE). Each door reads that place when driven with no argument,
# and the file its one argument names when given one (the param1 folio). No
# environment variable carries a path.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"

######################################################################
# Furnish replicas — each runs, in the child shell the module's ally check
# opens, what that regime's own furnish runs: constants, module, the work
# tree's own file, this regime's values laid over, kindle, enforce. The kindle
# locks, so the values are laid before it.

zrbvq_rbrp_furnish() {
  source "${ZRBVQ_RBK_DIR}/rbcc_constants.sh"
  source "${ZRBVQ_RBK_DIR}/rbgc_constants.sh"
  source "${ZRBVQ_RBK_DIR}/rbrp_regime.sh"
  zrbcc_kindle
  zrbgc_kindle
  source "${RBCC_rbrp_file}"
  RBRP_OPERATOR_EMAIL="${RBRQ_OPERATOR_EMAIL}"
  RBRP_BILLING_ACCOUNT_ID="${RBRQ_BILLING_ACCOUNT_ID}"
  zrbrp_kindle
  zrbrp_enforce
}

zrbvq_rbrw_furnish() {
  source "${ZRBVQ_RBK_DIR}/rbcc_constants.sh"
  source "${ZRBVQ_RBK_DIR}/rbrw_regime.sh"
  source "${RBCC_rbrw_file}"
  RBRW_WORKFORCE_POOL_ID="${RBRQ_WORKFORCE_POOL_ID}"
  RBRW_SESSION_DURATION="${RBRQ_SESSION_DURATION}"
  zrbcc_kindle
  zrbrw_kindle
  zrbrw_enforce
}

zrbvq_rbrd_furnish() {
  source "${ZRBVQ_RBK_DIR}/rbcc_constants.sh"
  source "${ZRBVQ_RBK_DIR}/rbrd_regime.sh"
  source "${RBCC_rbrd_file}"
  RBRD_CLOUD_PREFIX="${RBRQ_CLOUD_PREFIX}"
  RBRD_DEPOT_MONIKER="${ZRBVQ_DEPOT_MONIKER}"
  zrbcc_kindle
  zrbrd_kindle
  zrbrd_enforce
}

# The federation regime's file is the active foedus's, found as its furnish finds it
zrbvq_rbrf_furnish() {
  source "${BURD_BUK_DIR}/bug_git.sh"
  source "${ZRBVQ_RBK_DIR}/rbcc_constants.sh"
  source "${ZRBVQ_RBK_DIR}/rbrf_regime.sh"
  source "${RBCC_rbrr_file}"
  rbcc_source_active_rbrf_flint "${RBRR_ACTIVE_FOEDUS:-}"
  RBRF_PROVIDER_ID="${ZRBVQ_PROVIDER_ID}"
  zrbcc_kindle
  zrbrf_kindle
  zrbrf_enforce
}

######################################################################
# Internal helpers

# The one argument a door takes is its folio; anything beyond it is refused.
zrbvq_no_extra_arguments() {
  test "$#" -eq 0 || buc_die_now "This door takes one argument at most, the file; refused: $*"
}

# Source the file standing at the door's place, then kindle over it.
zrbvq_load() {
  test -f "${z_rbvq_file}" || buc_die_now "No Qualification file at ${z_rbvq_file}; lay one with the laying door, rbvw-ql"
  source "${z_rbvq_file}" || buc_die_now "Failed to source ${z_rbvq_file}"
  zrbvq_kindle
}

######################################################################
# Command Functions

# Displays every Qualification field as the file holds it, before any check, so a broken value stands visible beside the rest.
rbvq_render() {
  buc_doc_brief "Displays every Qualification field as the file holds it, before any check, so a broken value stands visible beside the rest."
  buc_doc_param "file" "Optional: the values file; rbrq.env beside the BURS file when omitted"
  buc_doc_shown || return 0
  zrbvq_no_extra_arguments "$@"

  zrbvq_load
  buv_render RBRQ "RBRQ - Qualification" "${z_rbvq_file}"
}

# Checks every Qualification field's shape, and each field that fills a clone's field where that field is defined, and fails naming the field that does not hold.
rbvq_validate() {
  buc_doc_brief "Checks every Qualification field's shape, and each field that fills a clone's field where that field is defined, and fails naming the field that does not hold."
  buc_doc_param "file" "Optional: the values file; rbrq.env beside the BURS file when omitted"
  buc_doc_shown || return 0
  zrbvq_no_extra_arguments "$@"

  zrbvq_load
  buc_step "Validating Qualification file: ${z_rbvq_file}"
  local z_report_status=0
  buv_report RBRQ "Qualification" || z_report_status=$?
  test "${z_report_status}" -eq 0 || buc_reject "${BUBG_ENROLL_DENTIL}" "A field above fails its shape: ${z_rbvq_file}"
  zrbvq_enforce
  buc_step "Qualification valid"
}

# Checks the Qualification fields, then prints the block pasted to a walking session: each value beside its plain name, the try's joined values among them; or nothing but the refusal.
rbvq_block() {
  buc_doc_brief "Checks the Qualification fields, then prints the block pasted to a walking session: each value beside its plain name, the try's joined values among them; or nothing but the refusal."
  buc_doc_param "file" "Optional: the values file; rbrq.env beside the BURS file when omitted"
  buc_doc_shown || return 0
  zrbvq_no_extra_arguments "$@"

  zrbvq_load
  zrbvq_enforce

  # Each value beside the one string its enrollment carries; each stem shown
  # joined with the index, under the stem's plain name, which names the joined value.
  # A field of least length 0 may stand unset; its line prints with no value.
  local z_i=""
  local z_varname=""
  local z_value=""
  for z_i in "${!z_buv_scope_roll[@]}"; do
    test "${z_buv_scope_roll[z_i]}" = "RBRQ" || continue
    z_varname="${z_buv_varname_roll[z_i]}"
    case "${z_varname}" in
      RBRQ_MONIKER_STEM)  z_value="${ZRBVQ_DEPOT_MONIKER}" ;;
      RBRQ_PROVIDER_STEM) z_value="${ZRBVQ_PROVIDER_ID}" ;;
      *)                  z_value="${!z_varname:-}" ;;
    esac
    printf '%s: %s\n' "${z_buv_desc_roll[z_i]}" "${z_value}" || buc_die_now
  done
}

# Adds one to the try index and writes the file, refusing at 9999.
rbvq_advance() {
  buc_doc_brief "Adds one to the try index and writes the file, refusing at 9999."
  buc_doc_param "file" "Optional: the values file; rbrq.env beside the BURS file when omitted"
  buc_doc_shown || return 0
  zrbvq_no_extra_arguments "$@"

  zrbvq_load

  zrbvq_index_predicate "${RBRQ_INDEX}" \
    || buc_reject "${BUBG_REGIME_DENTIL}" "RBRQ_INDEX must be four digits, 1000 to 9999, before it can advance: ${RBRQ_INDEX}"
  test "${RBRQ_INDEX}" -lt 9999 \
    || buc_reject "${BUBG_REGIME_DENTIL}" "RBRQ_INDEX stands at 9999 and advances no further: ${z_rbvq_file}"

  local -r z_next=$(( 10#${RBRQ_INDEX} + 1 ))
  local -r z_staged="${z_rbvq_file}.advancing"

  # Rewrite the index's line alone; every other line passes through as it stood.
  local z_count=0
  local z_line=""
  : > "${z_staged}" || buc_die_now "Could not write ${z_staged}"
  while IFS= read -r z_line || test -n "${z_line}"; do
    case "${z_line}" in
      "${ZRBVQ_INDEX_LEAD}"*)
        z_count=$(( z_count + 1 ))
        printf '%s\n' "${ZRBVQ_INDEX_LEAD}${z_next}" >> "${z_staged}" || buc_die_now "Could not write ${z_staged}"
        ;;
      *)
        printf '%s\n' "${z_line}" >> "${z_staged}" || buc_die_now "Could not write ${z_staged}"
        ;;
    esac
  done < "${z_rbvq_file}"

  if test "${z_count}" -ne 1; then
    rm -f "${z_staged}" || buc_die_now "Could not remove ${z_staged}"
    buc_reject "${BUBG_REGIME_DENTIL}" "The file must hold one ${ZRBVQ_INDEX_LEAD} line, and holds ${z_count}: ${z_rbvq_file}"
  fi

  mv "${z_staged}" "${z_rbvq_file}" || buc_die_now "Could not replace ${z_rbvq_file}"
  buc_step "Try index advanced: ${RBRQ_INDEX} -> ${z_next} in ${z_rbvq_file}"
}

# Writes a first Qualification file, its index reading 1000 and its other fields blank, and never writes over a file standing.
rbvq_lay() {
  buc_doc_brief "Writes a first Qualification file, its index reading 1000 and its other fields blank, and never writes over a file standing."
  buc_doc_param "file" "Optional: the values file; rbrq.env beside the BURS file when omitted"
  buc_doc_shown || return 0
  zrbvq_no_extra_arguments "$@"

  test ! -e "${z_rbvq_file}" || buc_die_now "A file already stands at ${z_rbvq_file}; the laying door never writes over one"
  local -r z_dir="${z_rbvq_file%/*}"
  test -d "${z_dir}" || buc_die_now "No folder stands at ${z_dir}"

  zrbvq_kindle

  local -r z_staged="${z_rbvq_file}.laying"
  {
    printf '%s\n' "# Qualification values, brought to each walk."
    printf '%s\n' "# Fill each field, validate with rbvw-qv, and advance the try index with rbvw-qa."
    local z_i=""
    local z_varname=""
    local z_group=""
    for z_i in "${!z_buv_scope_roll[@]}"; do
      test "${z_buv_scope_roll[z_i]}" = "RBRQ" || continue
      if test "${z_buv_group_roll[z_i]}" != "${z_group}"; then
        z_group="${z_buv_group_roll[z_i]}"
        printf '\n%s\n' "# ${z_group}"
      fi
      z_varname="${z_buv_varname_roll[z_i]}"
      printf '\n%s\n' "# ${z_buv_desc_roll[z_i]}"
      case "${z_varname}=" in
        "${ZRBVQ_INDEX_LEAD}") printf '%s\n' "${ZRBVQ_INDEX_LEAD}1000" ;;
        *)                     printf '%s\n' "${z_varname}=" ;;
      esac
    done
    printf '\n%s\n' "# eof"
  } > "${z_staged}" || buc_die_now "Could not write ${z_staged}"

  test ! -e "${z_rbvq_file}" || buc_die_now "A file came to stand at ${z_rbvq_file} while laying; ${z_staged} left beside it"
  mv "${z_staged}" "${z_rbvq_file}" || buc_die_now "Could not place ${z_rbvq_file}"
  buc_step "Qualification file laid: ${z_rbvq_file}"
}

######################################################################
# Furnish and Main

z_rbvq_file=""
zrbvq_furnish() {
  buc_doc_env_row "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_row "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided; the shipped kit's home)"
  buc_doc_env_row "BURD_TEMP_DIR         " "Temp directory (dispatch-provided)"
  buc_doc_env_row "BURD_STATION_FILE     " "The BURS file the dispatch sourced (dispatch-provided; the values file stands beside it)"
  buc_doc_env_done || return 0

  local -r z_dir="${BASH_SOURCE[0]%/*}"

  source "${BURD_BUK_DIR}/bupr_regime.sh" || buc_die_now "Failed to source bupr_regime.sh"
  source "${z_dir}/rbvq_regime.sh" || buc_die_now "Failed to source rbvq_regime.sh"

  zburd_enforce
  zbupr_kindle

  if test -n "${BUZ_FOLIO:-}"; then
    z_rbvq_file="${BUZ_FOLIO}"
  else
    z_rbvq_file="${BURD_STATION_FILE%/*}/rbrq.env"
  fi
}

buc_execute rbvq_ "Qualification values (rbvq)" zrbvq_furnish "$@"

# eof
