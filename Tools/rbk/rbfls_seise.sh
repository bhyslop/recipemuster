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
# Recipe Bottle Foundry Ledger - seise cluster (guard-free, sourced by rbfl0_):
# elect the repo's substrate reliquary — resolve a reliquary Lode touchmark
# express-or-chain, decode-and-gate its kind to reliquary, then rewrite
# RBRR_SUBSTRATE_RELIQUARY in the one rbrr.env. The repo-regime sibling of feoff
# (which elects a vessel's bole anchor) and yoke (which elects the made-side
# reliquary across vessels). Operator-committed, never self-committing (RBr_a52).

set -euo pipefail

######################################################################
# Seise (rbfl_*)

rbfl_seise() {
  zrbfl_sentinel

  local -r z_express="${BUZ_FOLIO:-}"

  buc_doc_brief "Seise the repo's substrate reliquary — resolve a reliquary Lode touchmark (express-or-chain), then rewrite RBRR_SUBSTRATE_RELIQUARY in rbrr.env for the vessel-less substrate captures (underpin, immure)"
  buc_doc_param "touchmark" "Reliquary Lode touchmark (e.g., r260327172456); optional — absent, falls back to the reliquary touchmark a conclave chained forward"
  buc_doc_shown || return 0

  # Relay-then-read (RBr_3e7): forward the chain baton before any read or failure point.
  buf_relay || buc_die_now "Failed to relay chained facts"

  # Resolve the reliquary touchmark express-or-chain: an express argument wins;
  # absent, fall back to the touchmark a conclave handed forward through the
  # depth-1 chain. No clean-tree gate here (RBr_a52).
  local z_touchmark=""
  z_touchmark=$(buf_elect_fact_capture "${z_express}" "${RBF_FACT_LODE_TOUCHMARK}") \
    || buc_reject "${BUBC_band_chain}" "No reliquary touchmark — pass one (param1) or run a reliquary conclave immediately before seise"
  local z_source="chain"
  test -z "${z_express}" || z_source="express"

  # Assert the touchmark is a reliquary kind up front by decoding its kind-letter
  # prefix — the single home for touchmark kind decode, shared with yoke/feoff/augur. A
  # non-reliquary capture (a bole or underpin chained ahead of this election)
  # carries no tool cohort to resolve from: reject up front rather than fail late
  # at capture time.
  local z_kind=""
  z_kind=$(zrbld_decode_touchmark_kind_capture "${z_touchmark}") \
    || buc_reject "${BUBC_band_chain}" "Touchmark '${z_touchmark}' has no recognizable Lode kind prefix"
  test "${z_kind}" = "${RBGC_LODE_KIND_RELIQUARY}" \
    || buc_reject "${BUBC_band_chain}" "Touchmark '${z_touchmark}' is kind '${z_kind}', not a reliquary — seise elects the substrate-capture tool cohort, which only a reliquary conclave carries"

  buc_step "Seising substrate reliquary ${z_touchmark} into ${RBCC_rbrr_file} (source ${z_source})"

  # Replace-or-append the RBRR_SUBSTRATE_RELIQUARY line in rbrr.env.
  test -f "${RBCC_rbrr_file}" || buc_die_now "Repo regime file not found: ${RBCC_rbrr_file}"
  local -r z_tmp_file="${BURD_TEMP_DIR}/rbfl_seise_${RBCC_rbrr_file##*/}.new"
  local z_line=""
  local z_found=false
  while IFS= read -r z_line || test -n "${z_line}"; do
    if [[ "${z_line}" == RBRR_SUBSTRATE_RELIQUARY=* ]]; then
      printf 'RBRR_SUBSTRATE_RELIQUARY=%s\n' "${z_touchmark}"; z_found=true
    else
      printf '%s\n' "${z_line}"
    fi
  done < "${RBCC_rbrr_file}" > "${z_tmp_file}" \
    || buc_die_now "Failed to rewrite ${RBCC_rbrr_file} for RBRR_SUBSTRATE_RELIQUARY"
  if [[ "${z_found}" != "true" ]]; then
    printf 'RBRR_SUBSTRATE_RELIQUARY=%s\n' "${z_touchmark}" >> "${z_tmp_file}" \
      || buc_die_now "Failed to append RBRR_SUBSTRATE_RELIQUARY"
  fi
  mv "${z_tmp_file}" "${RBCC_rbrr_file}" || buc_die_now "Failed to finalize ${RBCC_rbrr_file}"

  # Loud on success: the elected touchmark and its source named prominently, so a
  # wrong election shows at the moment of action rather than only in the git diff.
  buc_success "Seised substrate reliquary: RBRR_SUBSTRATE_RELIQUARY=${z_touchmark} (source: ${z_source})"
  buc_info "Commit the rbrr.env change with your usual git workflow, then run a substrate capture (underpin, immure) FROM this committed election."
}

# eof
