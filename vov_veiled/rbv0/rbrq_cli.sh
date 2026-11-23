#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# RBRQ CLI - the Qualification Regime's doors (veiled). Dispatched by the
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
# Internal helpers

# The one argument a door takes is its folio; anything beyond it is refused.
zrbrq_no_extra_arguments() {
  test "$#" -eq 0 || buc_die_now "This door takes one argument at most, the file; refused: $*"
}

# Source the file standing at the door's place, then kindle the regime over it.
zrbrq_load() {
  test -f "${ZRBRQ_FILE}" || buc_die_now "No Qualification Regime file at ${ZRBRQ_FILE}; lay one with the laying door, rbvw-ql"
  source "${ZRBRQ_FILE}" || buc_die_now "Failed to source ${ZRBRQ_FILE}"
  zrbrq_kindle
}

######################################################################
# Command Functions

rbrq_render() {
  buc_doc_brief "Display every Qualification Regime field as the file holds it, before any validation"
  buc_doc_param "file" "Optional: the values file; rbrq.env beside the station file when omitted"
  buc_doc_shown || return 0
  zrbrq_no_extra_arguments "$@"

  zrbrq_load
  buv_render RBRQ "RBRQ - Recipe Bottle Regime Qualification" "${ZRBRQ_FILE}"
}

rbrq_validate() {
  buc_doc_brief "Validate the Qualification Regime: each field's shape, and each filled field by its own regime"
  buc_doc_param "file" "Optional: the values file; rbrq.env beside the station file when omitted"
  buc_doc_shown || return 0
  zrbrq_no_extra_arguments "$@"

  zrbrq_load
  buc_step "Validating Qualification Regime file: ${ZRBRQ_FILE}"
  local z_report_status=0
  buv_report RBRQ "Qualification Regime" || z_report_status=$?
  test "${z_report_status}" -eq 0 || buc_reject "${BUBG_ENROLL_DENTIL}" "A field above fails its shape: ${ZRBRQ_FILE}"
  zrbrq_enforce
  buc_step "Qualification Regime valid"
}

rbrq_block() {
  buc_doc_brief "Validate, then print the block a paredros pastes to a walking session"
  buc_doc_param "file" "Optional: the values file; rbrq.env beside the station file when omitted"
  buc_doc_shown || return 0
  zrbrq_no_extra_arguments "$@"

  zrbrq_load
  zrbrq_enforce

  # Each value beside the one string its enrollment carries; each stem shown
  # joined with the index, under the stem's plain name, which names the joined value.
  local z_i=""
  local z_varname=""
  local z_value=""
  for z_i in "${!z_buv_scope_roll[@]}"; do
    test "${z_buv_scope_roll[z_i]}" = "RBRQ" || continue
    z_varname="${z_buv_varname_roll[z_i]}"
    case "${z_varname}" in
      RBRQ_MONIKER_STEM)  z_value="${ZRBRQ_DEPOT_MONIKER}" ;;
      RBRQ_PROVIDER_STEM) z_value="${ZRBRQ_PROVIDER_ID}" ;;
      *)                  z_value="${!z_varname}" ;;
    esac
    printf '%s: %s\n' "${z_buv_desc_roll[z_i]}" "${z_value}" || buc_die_now
  done
}

rbrq_advance() {
  buc_doc_brief "Add one to the try index and write the file, refusing at 9999"
  buc_doc_param "file" "Optional: the values file; rbrq.env beside the station file when omitted"
  buc_doc_shown || return 0
  zrbrq_no_extra_arguments "$@"

  zrbrq_load

  zrbrq_index_predicate "${RBRQ_INDEX}" \
    || buc_reject "${BUBG_REGIME_DENTIL}" "RBRQ_INDEX must be four digits, 1000 to 9999, before it can advance: ${RBRQ_INDEX}"
  test "${RBRQ_INDEX}" -lt 9999 \
    || buc_reject "${BUBG_REGIME_DENTIL}" "RBRQ_INDEX stands at 9999 and advances no further: ${ZRBRQ_FILE}"

  local -r z_next=$(( 10#${RBRQ_INDEX} + 1 ))
  local -r z_staged="${ZRBRQ_FILE}.advancing"

  # Rewrite the index's line alone; every other line passes through as it stood.
  local z_count=0
  local z_line=""
  : > "${z_staged}" || buc_die_now "Could not write ${z_staged}"
  while IFS= read -r z_line || test -n "${z_line}"; do
    case "${z_line}" in
      RBRQ_INDEX=*)
        z_count=$(( z_count + 1 ))
        printf '%s\n' "RBRQ_INDEX=${z_next}" >> "${z_staged}" || buc_die_now "Could not write ${z_staged}"
        ;;
      *)
        printf '%s\n' "${z_line}" >> "${z_staged}" || buc_die_now "Could not write ${z_staged}"
        ;;
    esac
  done < "${ZRBRQ_FILE}"

  if test "${z_count}" -ne 1; then
    rm -f "${z_staged}" || buc_die_now "Could not remove ${z_staged}"
    buc_reject "${BUBG_REGIME_DENTIL}" "The file must hold one RBRQ_INDEX line, and holds ${z_count}: ${ZRBRQ_FILE}"
  fi

  mv "${z_staged}" "${ZRBRQ_FILE}" || buc_die_now "Could not replace ${ZRBRQ_FILE}"
  buc_step "Try index advanced: ${RBRQ_INDEX} -> ${z_next} in ${ZRBRQ_FILE}"
}

rbrq_lay() {
  buc_doc_brief "Write a first Qualification Regime file, index 1000 and every other field blank; never over a file standing"
  buc_doc_param "file" "Optional: the values file; rbrq.env beside the station file when omitted"
  buc_doc_shown || return 0
  zrbrq_no_extra_arguments "$@"

  test ! -e "${ZRBRQ_FILE}" || buc_die_now "A file already stands at ${ZRBRQ_FILE}; the laying door never writes over one"
  local -r z_dir="${ZRBRQ_FILE%/*}"
  test -d "${z_dir}" || buc_die_now "No folder stands at ${z_dir}"

  zrbrq_kindle

  local -r z_staged="${ZRBRQ_FILE}.laying"
  {
    printf '%s\n' "# Recipe Bottle Qualification Regime — the values a walk's paredros brings."
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
      case "${z_varname}" in
        RBRQ_INDEX) printf '%s\n' "${z_varname}=1000" ;;
        *)          printf '%s\n' "${z_varname}=" ;;
      esac
    done
    printf '\n%s\n' "# eof"
  } > "${z_staged}" || buc_die_now "Could not write ${z_staged}"

  test ! -e "${ZRBRQ_FILE}" || buc_die_now "A file came to stand at ${ZRBRQ_FILE} while laying; ${z_staged} left beside it"
  mv "${z_staged}" "${ZRBRQ_FILE}" || buc_die_now "Could not place ${ZRBRQ_FILE}"
  buc_step "Qualification Regime file laid: ${ZRBRQ_FILE}"
}

######################################################################
# Furnish and Main

zrbrq_furnish() {
  buc_doc_env_row "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_row "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided; the shipped regimes' home)"
  buc_doc_env_row "BURD_TEMP_DIR         " "Temp directory (dispatch-provided)"
  buc_doc_env_row "BURD_STATION_FILE     " "The BURS file the dispatch sourced (dispatch-provided; the values file stands beside it)"
  buc_doc_env_done || return 0

  local -r z_dir="${BASH_SOURCE[0]%/*}"

  source "${BURD_BUK_DIR}/bupr_regime.sh" || buc_die_now "Failed to source bupr_regime.sh"
  source "${z_dir}/rbrq_regime.sh" || buc_die_now "Failed to source rbrq_regime.sh"

  zburd_enforce
  zbupr_kindle

  if test -n "${BUZ_FOLIO:-}"; then
    readonly ZRBRQ_FILE="${BUZ_FOLIO}"
  else
    readonly ZRBRQ_FILE="${BURD_STATION_FILE%/*}/rbrq.env"
  fi
}

buc_execute rbrq_ "Recipe Bottle Qualification Regime (veiled)" zrbrq_furnish "$@"

# eof
