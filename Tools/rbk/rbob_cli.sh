#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
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
# RBOB CLI - Recipe Bottle Orchestration Bottle command-line interface
#
# Command roster: rbz_zipper.sh (rbw-c Crucible group).

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh" || exit 1
source "${BURD_BUK_DIR}/buym_yelp.sh" || buc_die_now "Failed to source buym_yelp.sh"

######################################################################
# CLI Commands

rbob_validate() {
  zrbob_sentinel

  buc_doc_brief "Validate that RBOB configuration is complete and runnable"
  buc_doc_shown || return 0

  buc_step "RBOB Validate: ${RBRN_MONIKER}"

  # All values computed at kindle - just verify they exist
  test -n "${ZRBOB_RUNTIME}" || buc_die_now "ZRBOB_RUNTIME not set"
  test -n "${ZRBOB_SENTRY}" || buc_die_now "ZRBOB_SENTRY not set"
  test -n "${ZRBOB_PENTACLE}" || buc_die_now "ZRBOB_PENTACLE not set"
  test -n "${ZRBOB_BOTTLE}" || buc_die_now "ZRBOB_BOTTLE not set"
  test -n "${ZRBOB_NETWORK}" || buc_die_now "ZRBOB_NETWORK not set"
  test -f "${ZRBOB_SENTRY_SCRIPT}" || buc_die_now "Sentry script not found: ${ZRBOB_SENTRY_SCRIPT}"
  test -f "${ZRBOB_PENTACLE_SCRIPT}" || buc_die_now "Pentacle script not found: ${ZRBOB_PENTACLE_SCRIPT}"
  test -f "${ZRBOB_COMPOSE_BASE}" || buc_die_now "Compose base not found: ${ZRBOB_COMPOSE_BASE}"

  buc_step "RBOB configuration valid"
  echo "Moniker:       ${RBRN_MONIKER}" || buc_die_now "Failed to write configuration line"
  echo "Runtime:       ${ZRBOB_RUNTIME}" || buc_die_now "Failed to write configuration line"
  echo "Sentry:        ${ZRBOB_SENTRY}" || buc_die_now "Failed to write configuration line"
  echo "Pentacle:        ${ZRBOB_PENTACLE}" || buc_die_now "Failed to write configuration line"
  echo "Bottle:        ${ZRBOB_BOTTLE}" || buc_die_now "Failed to write configuration line"
  echo "Network:       ${ZRBOB_NETWORK}" || buc_die_now "Failed to write configuration line"
  echo "Compose base:  ${ZRBOB_COMPOSE_BASE}" || buc_die_now "Failed to write configuration line"
  local z_fragment_presence
  if test -f "${ZRBOB_COMPOSE_FRAGMENT}"; then
    z_fragment_presence='exists'
  else
    z_fragment_presence='not found'
  fi
  echo "Compose frag:  ${ZRBOB_COMPOSE_FRAGMENT} (${z_fragment_presence})" || buc_die_now "Failed to write configuration line"
}

rbob_info() {
  zrbob_sentinel

  buc_doc_brief "Show container names, network, and runtime for kindled nameplate"
  buc_doc_shown || return 0

  buc_step "RBOB Info: ${RBRN_MONIKER}"
  echo "Runtime:   ${ZRBOB_RUNTIME}" || buc_die_now "Failed to write configuration line"
  echo "Sentry:    ${ZRBOB_SENTRY}" || buc_die_now "Failed to write configuration line"
  echo "Pentacle:    ${ZRBOB_PENTACLE}" || buc_die_now "Failed to write configuration line"
  echo "Bottle:    ${ZRBOB_BOTTLE}" || buc_die_now "Failed to write configuration line"
  echo "Network:   ${ZRBOB_NETWORK}" || buc_die_now "Failed to write configuration line"
  echo "Sentry IP: ${RBRN_ENCLAVE_SENTRY_IP}" || buc_die_now "Failed to write configuration line"
  echo "Bottle IP: ${RBRN_ENCLAVE_BOTTLE_IP}" || buc_die_now "Failed to write configuration line"
}

rbob_scry() {
  zrbob_sentinel

  buc_doc_brief "Observe network traffic on Crucible containers"
  buc_doc_oparm "duration" "bounded capture window for scripted use (e.g. 10, 30s, 1m); omit for interactive run-until-Ctrl+C"
  buc_doc_oparm "filter"   "tcpdump filter expression scoping every leg (e.g. 'host 10.242.0.2')"
  buc_doc_shown || return 0

  # Kindle observe module and delegate
  zrboo_kindle
  rboo_observe "$@"
}

rbob_charged() {
  zrbob_sentinel

  buc_doc_brief "Check whether the Crucible is charged (compose project has running containers)"
  buc_doc_shown || return 0

  rbob_charged_predicate || return 1
}

######################################################################
# Furnish and Main

zrbob_furnish() {
  buc_doc_env_row "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_row "BUZ_FOLIO             " "Nameplate moniker (e.g., tadmor)"
  buc_doc_env_done || return 0

  local z_command="${1:-}"

  local z_rbk_kit_dir="${BASH_SOURCE[0]%/*}"
  source "${BURD_BUK_DIR}/bubc_constants.sh" || buc_die_now "Failed to source bubc_constants.sh"
  source "${BURD_BUK_DIR}/buv_validation.sh" || buc_die_now "Failed to source buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh" || buc_die_now "Failed to source burd_regime.sh"
  source "${z_rbk_kit_dir}/rbrn_regime.sh" || buc_die_now "Failed to source rbrn_regime.sh"
  source "${z_rbk_kit_dir}/rbcc_constants.sh" || buc_die_now "Failed to source rbcc_constants.sh"
  source "${z_rbk_kit_dir}/rbrr_regime.sh" || buc_die_now "Failed to source rbrr_regime.sh"
  source "${z_rbk_kit_dir}/rbrd_regime.sh" || buc_die_now "Failed to source rbrd_regime.sh"
  source "${z_rbk_kit_dir}/rbrf_regime.sh" || buc_die_now "Failed to source rbrf_regime.sh"
  source "${z_rbk_kit_dir}/rbrw_regime.sh" || buc_die_now "Failed to source rbrw_regime.sh"
  source "${z_rbk_kit_dir}/rbdc_derived.sh" || buc_die_now "Failed to source rbdc_derived.sh"
  source "${RBCC_rbrr_file}" || buc_die_now "Failed to source ${RBCC_rbrr_file}"
  source "${RBCC_rbrd_file}" || buc_die_now "Failed to source ${RBCC_rbrd_file}"
  rbcc_source_active_rbrf_flint "${RBRR_ACTIVE_FOEDUS:-}"
  source "${RBCC_rbrw_file}" || buc_die_now "Failed to source ${RBCC_rbrw_file}"
  source "${z_rbk_kit_dir}/rbgc_constants.sh" || buc_die_now "Failed to source rbgc_constants.sh"
  source "${z_rbk_kit_dir}/rbgl_layout.sh" || buc_die_now "Failed to source rbgl_layout.sh"
  source "${z_rbk_kit_dir}/rbgd_depot.sh" || buc_die_now "Failed to source rbgd_depot.sh"
  source "${z_rbk_kit_dir}/rbgo_oauth.sh" || buc_die_now "Failed to source rbgo_oauth.sh"
  source "${z_rbk_kit_dir}/rba_auth.sh" || buc_die_now "Failed to source rba_auth.sh"
  source "${z_rbk_kit_dir}/rbob_bottle.sh" || buc_die_now "Failed to source rbob_bottle.sh"
  source "${BURD_BUK_DIR}/buf_fact.sh" || buc_die_now "Failed to source buf_fact.sh"
  source "${BURD_BUK_DIR}/bug_git.sh" || buc_die_now "Failed to source bug_git.sh"
  source "${z_rbk_kit_dir}/rbfh_hygiene.sh" || buc_die_now "Failed to source rbfh_hygiene.sh"
  source "${z_rbk_kit_dir}/rbfc0_core.sh" || buc_die_now "Failed to source rbfc0_core.sh"
  source "${z_rbk_kit_dir}/rbfk_kludge.sh" || buc_die_now "Failed to source rbfk_kludge.sh"
  source "${z_rbk_kit_dir}/rbfb_beckon.sh" || buc_die_now "Failed to source rbfb_beckon.sh"
  source "${z_rbk_kit_dir}/rboo_observe.sh" || buc_die_now "Failed to source rboo_observe.sh"
  source "${BURD_BUK_DIR}/buz_zipper.sh" || buc_die_now "Failed to source buz_zipper.sh"
  source "${z_rbk_kit_dir}/rbz_zipper.sh" || buc_die_now "Failed to source rbz_zipper.sh"

  zbuv_kindle
  zburd_kindle
  zrbcc_kindle

  local z_folio="${BUZ_FOLIO:-}"
  if test -z "${z_folio}"; then
    zrbrn_kindle
    local z_monikers
    z_monikers=$(rbrn_list_capture) || buc_die_now "No nameplates found"
    buc_step "Available nameplates:"
    local z_moniker_list=()
    read -r -a z_moniker_list <<< "${z_monikers}" || buc_die_now "Failed to split nameplate list"
    local z_i=0
    for z_i in "${!z_moniker_list[@]}"; do
      buc_bare "        ${z_moniker_list[${z_i}]}"
    done
    buc_die_now "Nameplate moniker required (pass as argument)"
  fi
  local z_nameplate_file="${RBCC_moorings_dir}/${RBCC_nameplate_sprue}${z_folio}/${RBCC_rbrn_file}"
  test -f "${z_nameplate_file}" || buc_die_now "Nameplate not found: ${z_nameplate_file}"
  source "${z_nameplate_file}" || buc_die_now "Failed to source nameplate: ${z_nameplate_file}"
  zrbrn_kindle

  # Differential enforce: nameplate-keyed kludge commands write
  # RBRN_*_HALLMARK, so zrbrn_enforce would self-veto on marshal-zero
  # state where those fields are blank by design (rblm zeroes them;
  # rbtdrp_lifecycle treats them as RBTDRP_RBRN_BLANK_FIELDS). Strict
  # validation lives in rbw-rnv (rbrn_cli.sh). Mirrors the yoke/RBRV
  # split (rbfl_cli vs rbrv_cli).
  case "${z_command}" in
    rbob_kludge_bottle|rbob_kludge_sentry)
      ;;
    *)
      zrbrn_enforce
      ;;
  esac

  zrbrr_kindle
  zrbrd_kindle
  zrbrf_kindle
  zrbrw_kindle
  zrbrr_enforce
  zrbgc_kindle

  # Differential cloud-identity enforce (RBRD/RBRF/RBRW): the local-only
  # verbs never read these regimes — kludge is uncredentialed by charter,
  # and quench/exec/observe touch only kindled container names. Charge reads
  # them exactly when a consumed hallmark is ordained (GAR image refs plus
  # the retriever-mantle auto-summon), so the gate keys on hallmark kind —
  # kindles above stay unconditional, so every module is fully kindled and
  # the skipped regimes remain enrolled, locked, inert strings. Strict
  # validation lives in rbw-rdv/rfv/rwv (rbrd/rbrf/rbrw CLIs).
  case "${z_command}" in
    rbob_charge)
      local z_ordained=0
      if test -n "${RBRN_SENTRY_HALLMARK}" && ! zrbob_hallmark_is_kludge_predicate "${RBRN_SENTRY_HALLMARK}"; then
        z_ordained=1
      fi
      if test -n "${RBRN_BOTTLE_HALLMARK}" && ! zrbob_hallmark_is_kludge_predicate "${RBRN_BOTTLE_HALLMARK}"; then
        z_ordained=1
      fi
      if test "${z_ordained}" = "1"; then
        zrbrd_enforce
        zrbrf_enforce
        zrbrw_enforce
      fi
      ;;
    *)
      ;;
  esac

  zrbdc_kindle
  zrbgl_kindle
  zrbgd_kindle
  zrbgo_kindle
  zrba_kindle
  zrbob_kindle

  # rbfh kindle (Dockerfile hygiene) — load-bearing for both kludge and ordain
  # since both wired callsites invoke rbfh_dockerfile_check.
  zrbfh_kindle

  # Kludge commands need the uncredentialed foundry-kludge module.
  case "${z_command}" in
    rbob_kludge_bottle|rbob_kludge_sentry)
      zrbfk_kindle
      ;;
  esac

  zbuz_kindle
  zrbz_kindle
}

buc_execute rbob_ "Recipe Bottle Orchestration" zrbob_furnish "$@"

# eof
