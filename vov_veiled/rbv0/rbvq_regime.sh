#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# RBVQ - the Qualification Regime's validator module (RBRQ_ variables), veiled.
# The values a walk's paredros brings, held beside the station file. A field
# filling a clone's field is checked by that field's own regime, run whole in a
# child shell by the CLI's furnish replicas; no rule of another regime is
# written again here.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBVQ_SOURCED:-}" || buc_die_now "Module rbvq multiply sourced - check sourcing hierarchy"
ZRBVQ_SOURCED=1

######################################################################
# Internal Functions (zrbvq_*)

zrbvq_kindle() {
  test -z "${ZRBVQ_KINDLED:-}" || buc_die_now "Module rbvq already kindled"

  buv_dir_exists "${BURD_TEMP_DIR}"

  # The shipped kit whose regimes the fields fill, reached from the repository root
  readonly ZRBVQ_RBK_DIR="${BURD_TOOLS_DIR}/rbk"
  buv_dir_exists "${ZRBVQ_RBK_DIR}"

  # Each child shell's words land here, read back only on a refusal
  readonly ZRBVQ_ALLY_PREFIX="${BURD_TEMP_DIR}/rbvq_ally_"

  # The variable the advance rewrites, spelled once
  readonly ZRBVQ_INDEX_LEAD="RBRQ_INDEX="

  # The one string each enrollment carries is the field's plain name: the
  # render shows it, the block prints each value beside it, the laying door
  # writes it above each field. It stands nowhere else.

  buv_regime_enroll RBRQ

  buv_group_enroll "For every try"
  buv_string_enroll  RBRQ_OPERATOR_EMAIL      1  512  "Google account"
  buv_string_enroll  RBRQ_ORG_DISPLAY         1  512  "Google Cloud organization, as the console shows it"
  buv_string_enroll  RBRQ_BILLING_ACCOUNT_ID  1  512  "Billing account ID"
  buv_string_enroll  RBRQ_ENTRA_ADMIN_EMAIL   1  512  "Microsoft Entra administrator"
  buv_string_enroll  RBRQ_ENTRA_DIRECTORY     1  512  "Microsoft Entra directory, as the account button shows it"
  buv_string_enroll  RBRQ_WORKFORCE_POOL_ID   1  512  "Workforce pool ID"
  buv_string_enroll  RBRQ_SESSION_DURATION    1  512  "Workforce pool session duration"

  buv_group_enroll "For this try"
  buv_string_enroll  RBRQ_CLOUD_PREFIX        1  512  "Cloud prefix"
  buv_string_enroll  RBRQ_MONIKER_STEM        1  512  "Depot moniker"
  buv_string_enroll  RBRQ_PROVIDER_STEM       1  512  "Federation provider ID"
  buv_decimal_enroll RBRQ_INDEX            1000 9999  "Try index"

  # Guard against unexpected RBRQ_ variables not in enrollment
  buv_scope_sentinel RBRQ RBRQ_

  # Lock all enrolled RBRQ_ variables against mutation
  buv_lock RBRQ

  # The joined values: each stem with the index's digits straight after it.
  # Composed here and never stored.
  readonly ZRBVQ_DEPOT_MONIKER="${RBRQ_MONIKER_STEM:-}${RBRQ_INDEX:-}"
  readonly ZRBVQ_PROVIDER_ID="${RBRQ_PROVIDER_STEM:-}${RBRQ_INDEX:-}"

  readonly ZRBVQ_KINDLED=1
}

zrbvq_sentinel() {
  test "${ZRBVQ_KINDLED:-}" = "1" || buc_die_now "Module rbvq not kindled - call zrbvq_kindle first"
}

# The index's own shape: four digits, 1000 to 9999, no leading zero.
zrbvq_index_predicate() {
  zrbvq_sentinel
  [[ "${1:-}" =~ ^[123456789][0123456789]{3}$ ]]
}

# zrbvq_ally_check LABEL FURNISH PAIR... — run FURNISH in a child shell; on
# refusal, name the field of this regime that fed it. Each PAIR is
# RBRQ_FIELD:ALLY_FIELD. A refusal naming one ally field is reported beside the
# RBRQ field that fed it; one naming none of them beside every field laid.
zrbvq_ally_check() {
  zrbvq_sentinel

  local -r z_label="${1:-}"
  local -r z_furnish="${2:-}"
  test "$#" -lt 2 || shift 2
  test -n "${z_label}"   || buc_die_now "zrbvq_ally_check: label required"
  test -n "${z_furnish}" || buc_die_now "zrbvq_ally_check: furnish required"
  declare -F "${z_furnish}" >/dev/null || buc_die_now "zrbvq_ally_check: no function ${z_furnish}"

  local -r z_words="${ZRBVQ_ALLY_PREFIX}${z_furnish}.txt"

  local z_status=0
  set +e
  ( set -e; "${z_furnish}" ) >"${z_words}" 2>&1
  z_status=$?
  set -e

  test "${z_status}" -ne 0 || return 0

  local z_said=""
  z_said=$(<"${z_words}") || buc_die_now "Could not read the ${z_label} check's words: ${z_words}"

  local z_fed=""
  local z_all=""
  local z_pair=""
  local z_ours=""
  local z_theirs=""
  for z_pair in "$@"; do
    z_ours="${z_pair%%:*}"
    z_theirs="${z_pair#*:}"
    z_all="${z_all}${z_all:+, }${z_ours} (as ${z_theirs})"
    case "${z_said}" in
      *"${z_theirs}"*) z_fed="${z_fed}${z_fed:+, }${z_ours} (as ${z_theirs})" ;;
    esac
  done
  test -n "${z_fed}" || z_fed="${z_all}"

  buc_reject "${BUBG_REGIME_DENTIL}" "${z_fed}: refused by the ${z_label} check, which said: ${z_said}"
}

# Enforce this regime's own shapes, then each filled field by its own regime,
# through the furnish replicas the CLI defines (zrbvq_*_furnish).
zrbvq_enforce() {
  zrbvq_sentinel

  buv_vet RBRQ

  zrbvq_index_predicate "${RBRQ_INDEX}" \
    || buc_reject "${BUBG_REGIME_DENTIL}" "RBRQ_INDEX must be four digits, 1000 to 9999: ${RBRQ_INDEX}"

  [[ "${RBRQ_ENTRA_ADMIN_EMAIL}" =~ ^[^@[:space:]]+@[^@[:space:]]+$ ]] \
    || buc_reject "${BUBG_REGIME_DENTIL}" "RBRQ_ENTRA_ADMIN_EMAIL must be an address carrying one @: ${RBRQ_ENTRA_ADMIN_EMAIL}"

  zrbvq_ally_check "RBRP" zrbvq_rbrp_furnish \
    "RBRQ_OPERATOR_EMAIL:RBRP_OPERATOR_EMAIL" \
    "RBRQ_BILLING_ACCOUNT_ID:RBRP_BILLING_ACCOUNT_ID"

  zrbvq_ally_check "RBRW" zrbvq_rbrw_furnish \
    "RBRQ_WORKFORCE_POOL_ID:RBRW_WORKFORCE_POOL_ID" \
    "RBRQ_SESSION_DURATION:RBRW_SESSION_DURATION"

  zrbvq_ally_check "RBRD" zrbvq_rbrd_furnish \
    "RBRQ_CLOUD_PREFIX:RBRD_CLOUD_PREFIX" \
    "RBRQ_MONIKER_STEM:RBRD_DEPOT_MONIKER"

  zrbvq_ally_check "RBRF" zrbvq_rbrf_furnish \
    "RBRQ_PROVIDER_STEM:RBRF_PROVIDER_ID"
}

# eof
