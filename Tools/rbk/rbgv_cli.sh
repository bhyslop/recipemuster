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
# RBGV CLI - Recipe Bottle Access Probe command-line interface
#
# Surfaces the credential access probes implemented by the rbgv library
# module as four operator tabtargets (one per role). Theurge consumes these
# as plain subprocesses, owning no colophons of its own.
#
# Commands:
#   check_governor   JWT SA access probe for the governor credential
#   check_retriever  JWT SA access probe for the retriever credential
#   check_director   JWT SA access probe for the director credential
#   check_payor      OAuth access probe for the payor credential

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"

######################################################################
# CLI Commands

# JWT SA propagation-absorbing budget: 60 attempts × 3000ms ≈ 180s worst case.
# Happy path exits on first 2xx; budget consumed only when post-mint IAM
# propagation lags. Each retry mints a fresh JWT-OAuth token, so the inner
# rbgo_get_token_capture race (BBAAp) is exercised independently per attempt.
zrbgv_jwt_check() {
  local -r z_role="$1"
  local -r z_iterations=60
  local -r z_delay_ms=3000
  buc_step "JWT SA access probe: ${z_role}"
  rbgv_jwt_sa_probe "${z_role}" "${z_iterations}" "${z_delay_ms}"
  buc_success "${z_role} JWT access probe passed"
}

rbgv_check_governor() {
  zrbgv_sentinel
  buc_doc_brief "Check the governor credential reaches Google Cloud (JWT SA access probe)"
  buc_doc_shown || return 0
  zrbgv_jwt_check "governor"
}

rbgv_check_retriever() {
  zrbgv_sentinel
  buc_doc_brief "Check the retriever credential reaches Google Cloud (JWT SA access probe)"
  buc_doc_shown || return 0
  zrbgv_jwt_check "retriever"
}

rbgv_check_director() {
  zrbgv_sentinel
  buc_doc_brief "Check the director credential reaches Google Cloud (JWT SA access probe)"
  buc_doc_shown || return 0
  zrbgv_jwt_check "director"
}

rbgv_check_payor() {
  zrbgv_sentinel
  buc_doc_brief "Check the payor credential reaches Google Cloud (OAuth access probe)"
  buc_doc_shown || return 0

  # Payor probe semantic: stability-sample loop.
  local -r z_iterations=5
  local -r z_delay_ms=1500
  buc_step "Payor OAuth access probe"
  source "${RBCC_rbrp_file}" || buc_die "Failed to source RBRP: ${RBCC_rbrp_file}"
  zrbrp_kindle
  zrbrp_enforce
  rbgv_payor_oauth_probe "${z_iterations}" "${z_delay_ms}"
  buc_success "Payor OAuth access probe passed"
}

######################################################################
# Furnish and Main

zrbgv_furnish() {
  local -r z_command="${1:-}"

  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TEMP_DIR         " "Bash Dispatch Utility provided temporary directory, empty at start of command"
  buc_doc_env_done || return 0

  local z_rbk="${BASH_SOURCE[0]%/*}"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${z_rbk}/rbrr_regime.sh"
  source "${z_rbk}/rbrd_regime.sh"
  source "${z_rbk}/rbrp_regime.sh"
  source "${z_rbk}/rbcc_Constants.sh"
  source "${z_rbk}/rbgc_Constants.sh"
  source "${z_rbk}/rbdc_DerivedConstants.sh"
  source "${z_rbk}/rbgo_OAuth.sh"
  source "${z_rbk}/rbgu_Utility.sh"
  source "${z_rbk}/rbgi_IAM.sh"
  source "${z_rbk}/rbgp_Payor.sh"
  source "${z_rbk}/rbgv_AccessProbe.sh"

  zbuv_kindle
  zburd_kindle

  source "${RBCC_rbrr_file}" || buc_die "Failed to source ${RBCC_rbrr_file}"
  source "${RBCC_rbrd_file}" || buc_die "Failed to source RBRD: ${RBCC_rbrd_file}"
  zrbrr_kindle
  zrbrd_kindle

  # Payor probe is depot-agnostic: skip RBRR enforcement so it runs against
  # blank-template RBRR. zrbdc_kindle still runs to derive RBDC_PAYOR_RBRO_FILE
  # (credential path needed by the probe); depot-identity RBDC_* values it
  # also composes are unread on the Payor path. Mirrors BBAAS pattern in
  # rbgp_cli.sh:56-60 for rbgp_depot_list.
  if test "${z_command}" != "rbgv_check_payor"; then
    zrbrr_enforce
    zrbrd_enforce
  fi
  zrbcc_kindle
  zrbdc_kindle
  zrbgc_kindle
  zrbgo_kindle
  zrbgu_kindle
  zrbgi_kindle
  zrbgp_kindle
  zrbgv_kindle
}

buc_execute rbgv_ "Recipe Bottle Access Probe" zrbgv_furnish "$@"

# eof
