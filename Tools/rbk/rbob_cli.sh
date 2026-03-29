#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
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
# RBOB CLI - Recipe Bottle Orchestration Bottle command-line interface
#
# Commands:
#   start           Start bottle service (sentry + censer + bottle)
#   stop            Stop bottle service
#   connect_sentry  Connect to sentry container
#   connect_censer  Connect to censer container
#   connect_bottle  Connect to bottle container
#   observe         Observe network traffic (tcpdump)
#   info            Show container names, network, and runtime
#   validate        Validate configuration is complete

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# CLI Commands

rbob_validate() {
  zrbob_sentinel

  buc_doc_brief "Validate that RBOB configuration is complete and runnable"
  buc_doc_shown || return 0

  buc_step "RBOB Validate: ${RBRN_MONIKER}"

  # All values computed at kindle - just verify they exist
  test -n "${ZRBOB_RUNTIME}" || buc_die "ZRBOB_RUNTIME not set"
  test -n "${ZRBOB_SENTRY}" || buc_die "ZRBOB_SENTRY not set"
  test -n "${ZRBOB_CENSER}" || buc_die "ZRBOB_CENSER not set"
  test -n "${ZRBOB_BOTTLE}" || buc_die "ZRBOB_BOTTLE not set"
  test -n "${ZRBOB_NETWORK}" || buc_die "ZRBOB_NETWORK not set"
  test -f "${ZRBOB_SENTRY_SCRIPT}" || buc_die "Sentry script not found: ${ZRBOB_SENTRY_SCRIPT}"

  buc_step "RBOB configuration valid"
  echo "Moniker:   ${RBRN_MONIKER}"
  echo "Runtime:   ${ZRBOB_RUNTIME}"
  echo "Sentry:    ${ZRBOB_SENTRY}"
  echo "Censer:    ${ZRBOB_CENSER}"
  echo "Bottle:    ${ZRBOB_BOTTLE}"
  echo "Network:   ${ZRBOB_NETWORK}"
}

rbob_info() {
  zrbob_sentinel

  buc_doc_brief "Show container names, network, and runtime for kindled nameplate"
  buc_doc_shown || return 0

  buc_step "RBOB Info: ${RBRN_MONIKER}"
  echo "Runtime:   ${ZRBOB_RUNTIME}"
  echo "Sentry:    ${ZRBOB_SENTRY}"
  echo "Censer:    ${ZRBOB_CENSER}"
  echo "Bottle:    ${ZRBOB_BOTTLE}"
  echo "Network:   ${ZRBOB_NETWORK}"
  echo "Sentry IP: ${RBRN_ENCLAVE_SENTRY_IP}"
  echo "Bottle IP: ${RBRN_ENCLAVE_BOTTLE_IP}"
}

rbob_observe() {
  zrbob_sentinel

  buc_doc_brief "Observe network traffic on bottle service containers"
  buc_doc_shown || return 0

  # Kindle observe module and delegate
  zrboo_kindle
  rboo_observe
}

######################################################################
# Furnish and Main

zrbob_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env "BUZ_FOLIO             " "Nameplate moniker (e.g., tadmor)"
  buc_doc_env_done || return 0

  source "${BURD_CONFIG_DIR}/rbbc_constants.sh"
  local z_rbk_kit_dir="${BURD_TOOLS_DIR}/${RBBC_kit_subdir}"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${z_rbk_kit_dir}/rbrn_regime.sh"
  source "${z_rbk_kit_dir}/rbcc_Constants.sh"
  source "${z_rbk_kit_dir}/rbrr_regime.sh"
  source "${z_rbk_kit_dir}/rbdc_DerivedConstants.sh"
  source "${RBBC_rbrr_file}"
  source "${z_rbk_kit_dir}/rbgc_Constants.sh"
  source "${z_rbk_kit_dir}/rbgd_DepotConstants.sh"
  source "${z_rbk_kit_dir}/rbgo_OAuth.sh"
  source "${z_rbk_kit_dir}/rbob_bottle.sh"
  source "${z_rbk_kit_dir}/rbf_Foundry.sh"
  source "${z_rbk_kit_dir}/rboo_observe.sh"
  source "${BURD_BUK_DIR}/buz_zipper.sh"
  source "${z_rbk_kit_dir}/rbz_zipper.sh"

  zbuv_kindle
  zburd_kindle
  zrbcc_kindle

  local z_folio="${BUZ_FOLIO:-}"
  test -n "${z_folio}" || buc_die "BUZ_FOLIO must be set to a nameplate moniker"
  local z_nameplate_file="${RBBC_dot_dir}/${RBCC_rbrn_prefix}${z_folio}${RBCC_rbrn_ext}"
  test -f "${z_nameplate_file}" || buc_die "Nameplate not found: ${z_nameplate_file}"
  source "${z_nameplate_file}" || buc_die "Failed to source nameplate: ${z_nameplate_file}"
  zrbrn_kindle
  zrbrn_enforce

  zrbrr_kindle
  zrbrr_enforce
  zrbdc_kindle
  zrbgc_kindle
  zrbgd_kindle
  zrbgo_kindle
  zrbob_kindle

  zbuz_kindle
  zrbz_kindle
}

buc_execute rbob_ "Recipe Bottle Orchestration" zrbob_furnish "$@"

# eof
