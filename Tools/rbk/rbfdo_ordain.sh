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
# Recipe Bottle Foundry Director Build - ordain body (guard-free cluster,
# sourced by rbfd0_director): rbfd_ordain — the mode dispatcher over the
# operation bodies (conjure -> rbfd_build, bind -> rbfd_mirror, graft ->
# rbfd_graft), then the metadata pipeline (vouch or graft-metadata submit,
# cross-module into rbfv) and the hallmark beckon. Owns no operation
# machinery of its own.

set -euo pipefail

######################################################################
# External Functions (rbfd_*)

rbfd_ordain() {
  zrbfd_sentinel

  buc_doc_brief "Ordain a hallmark from a vessel (conjure, bind, or graft based on vessel mode)"
  buc_doc_param "vessel" "Vessel sigil or path to vessel directory"
  buc_doc_shown || return 0

  # Resolve vessel argument (sigil or path)
  zrbfc_resolve_vessel "${BUZ_FOLIO:-}"
  local -r z_vessel_dir=$(<"${ZRBFC_VESSEL_RESOLVED_DIR_FILE}")
  test -n "${z_vessel_dir}" || buc_die_now "Empty resolved vessel path"

  # Peek at vessel mode without sourcing (sourcing makes vars readonly,
  # and the downstream function will source again via zrbfc_load_vessel)
  local -r z_rbrv_file="${z_vessel_dir}/${RBCC_rbrv_file}"
  local z_mode=""
  local z_mode_line=""
  while IFS= read -r z_mode_line || test -n "${z_mode_line}"; do
    case "${z_mode_line}" in
      RBRV_VESSEL_MODE=*) z_mode="${z_mode_line#RBRV_VESSEL_MODE=}"; break ;;
    esac
  done < "${z_rbrv_file}"
  z_mode="${z_mode:-rbnve_conjure}"

  # Mode dispatch. Each mode owns its own dirty-tree posture: conjure gates
  # inside rbfd_build, bind gates inside rbfd_mirror, graft is deliberately
  # ungated (RBr_d71).
  case "${z_mode}" in
    rbnve_conjure) rbfd_build "${z_vessel_dir}" ;;
    rbnve_bind)    rbfd_mirror "${z_vessel_dir}" ;;
    rbnve_graft)   rbfd_graft "${z_vessel_dir}" ;;
    *)             buc_die_now "Unknown vessel mode: ${z_mode}" ;;
  esac

  # Chaining: read hallmark persisted by mode dispatch
  buc_step "Reading hallmark from mode dispatch output"
  local z_hallmark=""
  z_hallmark=$(<"${BURD_OUTPUT_DIR}/${RBF_FACT_HALLMARK}") \
    || buc_die_now "Failed to read hallmark from output"
  test -n "${z_hallmark}" || buc_die_now "Empty hallmark in output"

  # Metadata pipeline: graft uses combined about+vouch; conjure/bind already have about, need standalone vouch
  case "${z_mode}" in
    rbnve_conjure)
      buc_info "About produced by combined conjure job — proceeding to vouch"
      rbfv_vouch "${z_vessel_dir}" "${z_hallmark}"
      ;;
    rbnve_graft)
      zrbfv_graft_metadata_submit "${z_vessel_dir}" "${z_hallmark}"
      ;;
    rbnve_bind)
      buc_info "About produced by combined bind job — proceeding to vouch"
      rbfv_vouch "${z_vessel_dir}" "${z_hallmark}"
      ;;
    *)
      buc_die_now "Unknown vessel mode in chaining: ${z_mode}"
      ;;
  esac

  # Beckon the consumers of the hallmark this ordain just wrote
  rbfb_hallmark "${z_hallmark}"
}

# eof
