#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# RBRQ Qualification Regime - Validator Module (veiled). The values a walk's
# paredros brings, held at the station beside its station file. A field filling
# a clone's field is checked by that field's own regime, run whole in a child
# shell over the work tree's own file with this regime's values laid over it;
# no rule of another regime is written again here.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRQ_SOURCED:-}" || buc_die_now "Module rbrq multiply sourced - check sourcing hierarchy"
ZRBRQ_SOURCED=1

######################################################################
# Internal Functions (zrbrq_*)

zrbrq_kindle() {
  test -z "${ZRBRQ_KINDLED:-}" || buc_die_now "Module rbrq already kindled"

  buv_dir_exists "${BURD_TEMP_DIR}"

  # The shipped kit whose regimes the fields fill, reached from the repository root
  readonly ZRBRQ_RBK_DIR="${BURD_TOOLS_DIR}/rbk"
  buv_dir_exists "${ZRBRQ_RBK_DIR}"

  # Each child shell's words land here, read back only on a refusal
  readonly ZRBRQ_ALLY_PREFIX="${BURD_TEMP_DIR}/rbrq_ally_"

  # The one string each enrollment carries is the field's plain name: the
  # render shows it, the block prints each value beside it, the laying door
  # writes it above each field. It stands nowhere else.

  buv_regime_enroll RBRQ

  buv_group_enroll "The station and its paredros"
  buv_string_enroll  RBRQ_OPERATOR_EMAIL      1  512  "Google account"
  buv_string_enroll  RBRQ_ORG_DISPLAY         1  512  "Google Cloud organization, as the console shows it"
  buv_string_enroll  RBRQ_BILLING_ACCOUNT_ID  1  512  "Billing account ID"
  buv_string_enroll  RBRQ_ENTRA_ADMIN_EMAIL   1  512  "Microsoft Entra administrator"
  buv_string_enroll  RBRQ_ENTRA_DIRECTORY     1  512  "Microsoft Entra directory, as the account button shows it"
  buv_string_enroll  RBRQ_WORKFORCE_POOL_ID   1  512  "Workforce pool ID"
  buv_string_enroll  RBRQ_SESSION_DURATION    1  512  "Workforce pool session duration"

  buv_group_enroll "The try"
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
  readonly ZRBRQ_DEPOT_MONIKER="${RBRQ_MONIKER_STEM:-}${RBRQ_INDEX:-}"
  readonly ZRBRQ_PROVIDER_ID="${RBRQ_PROVIDER_STEM:-}${RBRQ_INDEX:-}"

  readonly ZRBRQ_KINDLED=1
}

zrbrq_sentinel() {
  test "${ZRBRQ_KINDLED:-}" = "1" || buc_die_now "Module rbrq not kindled - call zrbrq_kindle first"
}

# The index's own shape: four digits, 1000 to 9999, no leading zero.
zrbrq_index_predicate() {
  [[ "${1:-}" =~ ^[123456789][0123456789]{3}$ ]]
}

######################################################################
# The child-shell checks — each runs what that regime's own furnish runs:
# constants, module, the work tree's own file, this regime's values laid over,
# kindle, enforce. The kindle locks, so the values are laid before it.

zrbrq_ally_payor() {
  source "${ZRBRQ_RBK_DIR}/rbcc_constants.sh"
  source "${ZRBRQ_RBK_DIR}/rbgc_constants.sh"
  source "${ZRBRQ_RBK_DIR}/rbrp_regime.sh"
  zrbcc_kindle
  zrbgc_kindle
  source "${RBCC_rbrp_file}"
  RBRP_OPERATOR_EMAIL="${RBRQ_OPERATOR_EMAIL}"
  RBRP_BILLING_ACCOUNT_ID="${RBRQ_BILLING_ACCOUNT_ID}"
  zrbrp_kindle
  zrbrp_enforce
}

zrbrq_ally_workforce() {
  source "${ZRBRQ_RBK_DIR}/rbcc_constants.sh"
  source "${ZRBRQ_RBK_DIR}/rbrw_regime.sh"
  source "${RBCC_rbrw_file}"
  RBRW_WORKFORCE_POOL_ID="${RBRQ_WORKFORCE_POOL_ID}"
  RBRW_SESSION_DURATION="${RBRQ_SESSION_DURATION}"
  zrbcc_kindle
  zrbrw_kindle
  zrbrw_enforce
}

zrbrq_ally_depot() {
  source "${ZRBRQ_RBK_DIR}/rbcc_constants.sh"
  source "${ZRBRQ_RBK_DIR}/rbrd_regime.sh"
  source "${RBCC_rbrd_file}"
  RBRD_CLOUD_PREFIX="${RBRQ_CLOUD_PREFIX}"
  RBRD_DEPOT_MONIKER="${ZRBRQ_DEPOT_MONIKER}"
  zrbcc_kindle
  zrbrd_kindle
  zrbrd_enforce
}

# The federation regime's file is the active foedus's, found as its furnish finds it
zrbrq_ally_federation() {
  source "${BURD_BUK_DIR}/bug_git.sh"
  source "${ZRBRQ_RBK_DIR}/rbcc_constants.sh"
  source "${ZRBRQ_RBK_DIR}/rbrf_regime.sh"
  source "${RBCC_rbrr_file}"
  rbcc_source_active_rbrf_flint "${RBRR_ACTIVE_FOEDUS:-}"
  RBRF_PROVIDER_ID="${ZRBRQ_PROVIDER_ID}"
  zrbcc_kindle
  zrbrf_kindle
  zrbrf_enforce
}

# zrbrq_ally_check LABEL BODY PAIR... — run BODY in a child shell; on refusal,
# name the field of this regime that fed it. Each PAIR is RBRQ_FIELD:ALLY_FIELD.
# A refusal naming one ally field is reported beside the RBRQ field that fed it;
# one naming none of them is reported beside every field laid.
zrbrq_ally_check() {
  zrbrq_sentinel

  local -r z_label="${1:-}"
  local -r z_body="${2:-}"
  test "$#" -lt 2 || shift 2
  test -n "${z_label}" || buc_die_now "zrbrq_ally_check: label required"
  test -n "${z_body}"  || buc_die_now "zrbrq_ally_check: body required"

  local -r z_words="${ZRBRQ_ALLY_PREFIX}${z_body}.txt"

  local z_status=0
  set +e
  ( set -e; "${z_body}" ) >"${z_words}" 2>&1
  z_status=$?
  set -e

  test "${z_status}" -ne 0 || return 0

  local z_said=""
  z_said=$(<"${z_words}") || buc_die_now "Could not read the ${z_label}'s words: ${z_words}"

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

  buc_reject "${BUBG_REGIME_DENTIL}" "${z_fed}: refused by the ${z_label}'s own check, which said: ${z_said}"
}

# Enforce this regime's shapes, then each filled field by its own regime
zrbrq_enforce() {
  zrbrq_sentinel

  buv_vet RBRQ

  zrbrq_index_predicate "${RBRQ_INDEX}" \
    || buc_reject "${BUBG_REGIME_DENTIL}" "RBRQ_INDEX must be four digits, 1000 to 9999: ${RBRQ_INDEX}"

  [[ "${RBRQ_ENTRA_ADMIN_EMAIL}" =~ ^[^@[:space:]]+@[^@[:space:]]+$ ]] \
    || buc_reject "${BUBG_REGIME_DENTIL}" "RBRQ_ENTRA_ADMIN_EMAIL must be an address carrying one @: ${RBRQ_ENTRA_ADMIN_EMAIL}"

  zrbrq_ally_check "payor regime" zrbrq_ally_payor \
    "RBRQ_OPERATOR_EMAIL:RBRP_OPERATOR_EMAIL" \
    "RBRQ_BILLING_ACCOUNT_ID:RBRP_BILLING_ACCOUNT_ID"

  zrbrq_ally_check "workforce regime" zrbrq_ally_workforce \
    "RBRQ_WORKFORCE_POOL_ID:RBRW_WORKFORCE_POOL_ID" \
    "RBRQ_SESSION_DURATION:RBRW_SESSION_DURATION"

  zrbrq_ally_check "depot regime" zrbrq_ally_depot \
    "RBRQ_CLOUD_PREFIX:RBRD_CLOUD_PREFIX" \
    "RBRQ_MONIKER_STEM:RBRD_DEPOT_MONIKER"

  zrbrq_ally_check "federation regime" zrbrq_ally_federation \
    "RBRQ_PROVIDER_STEM:RBRF_PROVIDER_ID"
}

# eof
