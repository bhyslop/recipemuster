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
# Recipe Bottle Foedus — test-bed cardinality verbs over the moorings foedera
# library. Two atomic, first-class toothings on a STANDING foedus:
#
#   descry  (rbw-jd) — read a named foedus's workforce-pool health from the
#                      Manor (present-and-active, or a named deficit). Read-only,
#                      payor-credentialed, mutates nothing. Contract: RBSFD.
#   instate (rbw-j)  — re-point the active-foedus selector RBRR_ACTIVE_FOEDUS in
#                      rbrr.env via an atomic single-field rewrite. No clean-tree
#                      gate, no commit, no Manor mutation, no sitting reset.
#                      Contract: RBSFI.
#
# Neither founds nor dissolves a foedus — that is the federation manor verbs'
# (affiance/jilt) concern. The reuse-if-valid-else-establish decision lives in
# the fixture that composes these atoms, never folded into a fat verb.

set -euo pipefail

######################################################################
# Internal (zrbof_*)

zrbof_kindle() {
  test -z "${ZRBOF_KINDLED:-}" || buc_die "Module rbof already kindled"

  # Foedera library root — the moorings directory holding one rbef_ subdirectory
  # per standing foedus, each carrying that foedus's rbrf.env. rbcc must be
  # kindled first (RBCC_moorings_dir / RBCC_foedera_subdir).
  readonly ZRBOF_FOEDERA_DIR="${RBCC_moorings_dir}/${RBCC_foedera_subdir}"

  readonly ZRBOF_KINDLED=1
}

zrbof_sentinel() {
  test "${ZRBOF_KINDLED:-}" = "1" || buc_die "Module rbof not kindled - call zrbof_kindle first"
}

# Echo the discovered foedus identities (rbef_ subdirectory names), space-
# separated, or "(none)". Pure — for embedding in a rejection message so a bad
# or missing identity fails by listing the available ones (RBSFD/RBSFI shape).
zrbof_list_foedera() {
  local z_avail=""
  local z_entry=""
  for z_entry in "${ZRBOF_FOEDERA_DIR}"/rbef_*/; do
    test -d "${z_entry}" || continue
    z_entry="${z_entry%/}"
    z_avail="${z_avail} ${z_entry##*/}"
  done
  if test -n "${z_avail}"; then
    printf '%s\n' "${z_avail# }"
  else
    printf '%s\n' "(none)"
  fi
}

# Validate that a foedus identity resolves to a library subdirectory holding an
# rbrf.env, rejecting in the GIVEN band (each verb owns its own precision band)
# and listing the discovered foedera. Runs in the caller's process — NEVER a
# command substitution — so buc_reject's band-coded exit propagates to the
# dispatch boundary.
zrbof_require_foedus() {
  local -r z_foedus="${1:-}"
  local -r z_band="${2:-}"
  local -r z_avail="$(zrbof_list_foedera)"

  test -n "${z_foedus}" \
    || buc_reject "${z_band}" "Foedus identity required (param1). Available foedera: ${z_avail}"
  [[ "${z_foedus}" == rbef_* ]] \
    || buc_reject "${z_band}" "Foedus identity must bear the rbef_ sprue: ${z_foedus}. Available foedera: ${z_avail}"
  test -d "${ZRBOF_FOEDERA_DIR}/${z_foedus}" \
    || buc_reject "${z_band}" "No foedus subdirectory '${z_foedus}' in the foedera library. Available foedera: ${z_avail}"
  test -f "${ZRBOF_FOEDERA_DIR}/${z_foedus}/rbrf.env" \
    || buc_reject "${z_band}" "Foedus '${z_foedus}' has no rbrf.env. Available foedera: ${z_avail}"
}

# Extract one RBRF_ assignment value from a foedus's rbrf.env by PARSING the file
# (never sourcing — the active foedus's RBRF_* are already kindled readonly, and
# a descry subject may differ from the active one). Echoes the bare value or
# returns 1; the caller guards with || buc_reject.
zrbof_rbrf_field_capture() {
  local -r z_file="${1:-}"
  local -r z_var="${2:-}"
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    if [[ "${z_line}" == ${z_var}=* ]]; then
      printf '%s' "${z_line#*=}"
      return 0
    fi
  done < "${z_file}"
  return 1
}

######################################################################
# Descry (rbof_descry) — read-only pool-health probe of a named foedus.

rbof_descry() {
  zrbof_sentinel

  # The foedus operand arrives via the BUZ_FOLIO env channel (param1 colophon).
  local -r z_foedus="${BUZ_FOLIO:-}"

  buc_doc_brief "Descry a standing foedus — read its workforce-pool health from the Manor (present-and-active, or a named deficit); read-only"
  buc_doc_param "foedus" "Foedus identity — the rbef_ subdirectory name of a standing foedus in the moorings foedera library"
  buc_doc_shown || return 0

  zrbof_require_foedus "${z_foedus}" "${BUBC_band_descry}"

  # Resolve the named foedus's pool by parsing its own rbrf.env (org / pool /
  # provider come from the inspected foedus, not the active selector).
  local -r z_rbrf="${ZRBOF_FOEDERA_DIR}/${z_foedus}/rbrf.env"
  local z_org=""
  local z_pool=""
  local z_provider=""
  z_org=$(zrbof_rbrf_field_capture "${z_rbrf}" "RBRF_ORG_ID") \
    || buc_reject "${BUBC_band_descry}" "Foedus '${z_foedus}' rbrf.env carries no RBRF_ORG_ID: ${z_rbrf}"
  z_pool=$(zrbof_rbrf_field_capture "${z_rbrf}" "RBRF_WORKFORCE_POOL_ID") \
    || buc_reject "${BUBC_band_descry}" "Foedus '${z_foedus}' rbrf.env carries no RBRF_WORKFORCE_POOL_ID: ${z_rbrf}"
  z_provider=$(zrbof_rbrf_field_capture "${z_rbrf}" "RBRF_PROVIDER_ID") \
    || buc_reject "${BUBC_band_descry}" "Foedus '${z_foedus}' rbrf.env carries no RBRF_PROVIDER_ID: ${z_rbrf}"

  buc_step "Descry foedus ${z_foedus} — pool ${z_pool} under organizations/${z_org}"

  # Payor OAuth — the same credential affiance/jilt use to work the org-level
  # workforce pool (workforcePools.get is org-scoped; the payor holds it). The
  # credless guard rides inside this capture: a reveille-tier run rejects here
  # before any credential touch.
  local z_token=""
  z_token=$(zrbgp_authenticate_capture) || buc_die "Failed to authenticate as Payor via OAuth"

  local -r z_iam_root="${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}"
  local -r z_pools_base="${z_iam_root}/locations/global/workforcePools"

  # Pool health: 404 absent; 200 + state DELETED soft-deleted (squatting the id
  # through the ~30-day purge window — not healthy); 200 otherwise live. A
  # broken read (any other code) is descry's OWN error, not a verdict — reject
  # in descry's band.
  buc_step "Read workforce pool health"
  rbuh_json "GET" "${z_pools_base}/${z_pool}" "${z_token}" "descry_pool_get"
  local z_pool_code=""
  z_pool_code=$(rbuh_code_capture "descry_pool_get") \
    || buc_reject "${BUBC_band_descry}" "No HTTP code from workforcePools.get for pool ${z_pool}"

  local z_pool_state=""
  local z_verdict=""
  case "${z_pool_code}" in
    200)
      z_pool_state=$(rbuh_json_field_capture "descry_pool_get" ".state // \"${RBGC_STATE_UNSPECIFIED}\"") \
        || z_pool_state="${RBGC_STATE_UNSPECIFIED}"
      if test "${z_pool_state}" = "${RBGC_STATE_DELETED}"; then
        z_verdict="pool-deleted"
      else
        z_verdict="active"
      fi
      ;;
    404)
      z_verdict="pool-absent"
      ;;
    *)
      buc_reject "${BUBC_band_descry}" "Unexpected HTTP ${z_pool_code} reading workforce pool ${z_pool} — descry cannot determine health"
      ;;
  esac

  # Provider presence (only meaningful when the pool stands; an absent/deleted
  # pool carries no provider). 200 present / 404 absent; any other code is a
  # broken read.
  if test "${z_verdict}" = "active"; then
    buc_step "Read pool provider presence"
    rbuh_json "GET" "${z_pools_base}/${z_pool}/providers/${z_provider}" "${z_token}" "descry_provider_get"
    local z_provider_code=""
    z_provider_code=$(rbuh_code_capture "descry_provider_get") \
      || buc_reject "${BUBC_band_descry}" "No HTTP code from providers.get for provider ${z_provider}"
    case "${z_provider_code}" in
      200) z_verdict="healthy" ;;
      404) z_verdict="provider-absent" ;;
      *)   buc_reject "${BUBC_band_descry}" "Unexpected HTTP ${z_provider_code} reading provider ${z_provider} — descry cannot determine health" ;;
    esac
  fi

  # Report the verdict (NOT a gate — a deficit is a successful read, reported
  # for the fixture to branch on). The verdict rides a fact file keyed by foedus
  # so the reuse-or-establish fixture can chain it.
  buf_write_fact_multi "${z_foedus}" "${RBCC_fact_ext_foedus_health}" "${z_verdict}"

  if test "${z_verdict}" = "healthy"; then
    buc_success "Foedus ${z_foedus} is HEALTHY — pool ${z_pool} present-and-active, provider ${z_provider} present"
  else
    buc_warn "Foedus ${z_foedus} is NOT healthy — verdict '${z_verdict}' (pool ${z_pool}, provider ${z_provider})"
  fi
}

######################################################################
# Instate (rbof_instate) — re-point the active-foedus selector.

rbof_instate() {
  zrbof_sentinel

  local -r z_foedus="${BUZ_FOLIO:-}"

  buc_doc_brief "Instate a standing foedus as active — re-point the RBRR_ACTIVE_FOEDUS selector in rbrr.env (atomic, uncommitted; the operator commits)"
  buc_doc_param "foedus" "Foedus identity — the rbef_ subdirectory name of a standing foedus in the moorings foedera library"
  buc_doc_shown || return 0

  zrbof_require_foedus "${z_foedus}" "${BUBC_band_instate}"

  # Atomic single-field rewrite of the active-foedus selector, reusing the
  # durable-config-link mechanics (feoff/yoke/anoint): substitute the matching
  # assignment, pass the rest through unchanged, write a temp file then rename.
  # No other field is touched; no clean-tree gate (instate writes the very change
  # the operator is about to commit); not committed; no Manor mutation; no
  # sitting reset (re-signing against the new foedus is avow's concern). RBSFI.
  local -r z_file="${RBCC_rbrr_file}"
  test -f "${z_file}" || buc_die "Repo regime file not found: ${z_file}"

  local -r z_var="RBRR_ACTIVE_FOEDUS"
  local -r z_line_new="${z_var}=${z_foedus}"
  local -r z_tmp="${BURD_TEMP_DIR}/rbof_instate_rbrr.env.new"
  local z_line=""
  local z_found=false
  while IFS= read -r z_line || test -n "${z_line}"; do
    if [[ "${z_line}" == ${z_var}=* ]]; then
      printf '%s\n' "${z_line_new}"; z_found=true
    else
      printf '%s\n' "${z_line}"
    fi
  done < "${z_file}" > "${z_tmp}" \
    || buc_die "Failed to rewrite ${z_file} for ${z_var}"

  # Unlike feoff (replace-or-append), the selector is a required enrolled field
  # that must already exist — a missing assignment is a corrupt repo regime, not
  # an append site.
  test "${z_found}" = "true" \
    || buc_die "No ${z_var} assignment in ${z_file} — the selector must be enrolled and present before instate can re-point it"

  mv "${z_tmp}" "${z_file}" || buc_die "Failed to finalize ${z_file}"

  buc_success "Instated ${z_foedus} as the active foedus: ${z_var}=${z_foedus}"
  buc_info "Commit the rbrr.env change with your usual git workflow; the authenticate-against-active consumers (avow, the accessor, the federated-access and mantle-access probes) require the selector committed before they run."
}

# eof
