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
# Recipe Bottle Common Vocabulary — linked term constants
#
# Kindled after regime loads (needs RBRR_PUBLIC_DOCS_URL).  Does not
# source or kindle anything itself — the consuming _cli.sh is responsible
# for sourcing buym_yelp.sh and this file, then calling zrbyc_kindle.
#
# Each constant is a diastema-marked yelp fragment suitable for
# interpolation into buh_line calls:
#   buh_line "A ${RBYC_DEPOT} is where images live."
#
# Variant forms (_S plural, _POSS possessive) link to the base anchor
# with alternate display text.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBYC_SOURCED:-}" || buc_die "Module rbyc multiply sourced - check sourcing hierarchy"
ZRBYC_SOURCED=1

######################################################################
# Module kindle

zrbyc_kindle() {
  test -z "${ZRBYC_KINDLED:-}" || buc_die "Module rbyc already kindled"

  zbuym_sentinel

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  # Helper: yawp then capture into named readonly
  # Usage: zrbyc_yk RBYC_TERM base_url anchor [display]
  zrbyc_yk() {
    buyy_link_yawp "${2}" "${3}" "${4:-}"
    readonly "${1}=${z_buym_yelp}"
  }

  # --- Paddock anchor inventory (38 base terms) ---

  # Operations
  zrbyc_yk RBYC_ORDAIN    "${z_docs}" "Ordain"
  zrbyc_yk RBYC_CONJURE   "${z_docs}" "Conjure"
  zrbyc_yk RBYC_BIND      "${z_docs}" "Bind"
  zrbyc_yk RBYC_GRAFT     "${z_docs}" "Graft"
  zrbyc_yk RBYC_KLUDGE    "${z_docs}" "Kludge"
  zrbyc_yk RBYC_ENSHRINE  "${z_docs}" "Enshrine"
  zrbyc_yk RBYC_SUMMON    "${z_docs}" "Summon"
  zrbyc_yk RBYC_PLUMB     "${z_docs}" "Plumb"
  zrbyc_yk RBYC_TALLY     "${z_docs}" "Tally"
  zrbyc_yk RBYC_VOUCH     "${z_docs}" "Vouch"
  zrbyc_yk RBYC_LEVY      "${z_docs}" "Levy"
  zrbyc_yk RBYC_CHARGE    "${z_docs}" "Charge"
  zrbyc_yk RBYC_QUENCH    "${z_docs}" "Quench"

  # Artifacts and infrastructure
  zrbyc_yk RBYC_VESSEL    "${z_docs}" "Vessel"
  zrbyc_yk RBYC_HALLMARK  "${z_docs}" "Hallmark"
  zrbyc_yk RBYC_DEPOT     "${z_docs}" "Depot"
  zrbyc_yk RBYC_RELIQUARY "${z_docs}" "Reliquary"
  zrbyc_yk RBYC_POUCH     "${z_docs}" "Pouch"
  zrbyc_yk RBYC_NAMEPLATE "${z_docs}" "Nameplate"

  # Egress modes
  zrbyc_yk RBYC_TETHERED  "${z_docs}" "Tethered"
  zrbyc_yk RBYC_AIRGAP    "${z_docs}" "Airgap"

  # Crucible components
  zrbyc_yk RBYC_CRUCIBLE  "${z_docs}" "Crucible"
  zrbyc_yk RBYC_SENTRY    "${z_docs}" "Sentry"
  zrbyc_yk RBYC_PENTACLE  "${z_docs}" "Pentacle"
  zrbyc_yk RBYC_BOTTLE    "${z_docs}" "Bottle"

  # Roles
  zrbyc_yk RBYC_PAYOR     "${z_docs}" "Payor"
  zrbyc_yk RBYC_GOVERNOR  "${z_docs}" "Governor"
  zrbyc_yk RBYC_DIRECTOR  "${z_docs}" "Director"
  zrbyc_yk RBYC_RETRIEVER "${z_docs}" "Retriever"
  zrbyc_yk RBYC_CHARTER   "${z_docs}" "Charter"
  zrbyc_yk RBYC_KNIGHT    "${z_docs}" "Knight"

  # Infrastructure concepts
  zrbyc_yk RBYC_TABTARGET "${z_docs}" "Tabtarget"
  zrbyc_yk RBYC_REGIME    "${z_docs}" "Regime"

  # Regime file types
  zrbyc_yk RBYC_RBRP      "${z_docs}" "RBRP"
  zrbyc_yk RBYC_RBRR      "${z_docs}" "RBRR"
  zrbyc_yk RBYC_RBRN      "${z_docs}" "RBRN"
  zrbyc_yk RBYC_RBRV      "${z_docs}" "RBRV"
  zrbyc_yk RBYC_BURC      "${z_docs}" "BURC"
  zrbyc_yk RBYC_BURS      "${z_docs}" "BURS"
  zrbyc_yk RBYC_RBRO      "${z_docs}" "RBRO"

  # --- Additional anchors used in handbooks ---

  zrbyc_yk RBYC_RECIPE_BOTTLE "${z_docs}" "" "Recipe Bottle"

  zrbyc_yk RBYC_RACK      "${z_docs}" "Rack"
  zrbyc_yk RBYC_MANOR     "${z_docs}" "Manor"
  zrbyc_yk RBYC_RBRA      "${z_docs}" "RBRA"
  zrbyc_yk RBYC_CCYOLO    "${z_docs}" "ccyolo"
  zrbyc_yk RBYC_ABJURE    "${z_docs}" "Abjure"
  zrbyc_yk RBYC_REKON     "${z_docs}" "Rekon"
  zrbyc_yk RBYC_LOG       "${z_docs}" "Log"
  zrbyc_yk RBYC_TRANSCRIPT "${z_docs}" "Transcript"
  zrbyc_yk RBYC_OUTPUT    "${z_docs}" "Output"

  # --- Variant forms (plural, possessive) ---

  zrbyc_yk RBYC_HALLMARKS  "${z_docs}" "Hallmark"   "Hallmarks"
  zrbyc_yk RBYC_VESSELS    "${z_docs}" "Vessel"     "Vessels"
  zrbyc_yk RBYC_DIRECTORS  "${z_docs}" "Director"   "Directors"
  zrbyc_yk RBYC_RETRIEVERS "${z_docs}" "Retriever"  "Retrievers"
  zrbyc_yk RBYC_TABTARGETS "${z_docs}" "Tabtarget"  "Tabtargets"
  zrbyc_yk RBYC_REGIMES    "${z_docs}" "Regime"     "Regimes"
  zrbyc_yk RBYC_LOGS       "${z_docs}" "Log"        "Logs"
  zrbyc_yk RBYC_MANORS     "${z_docs}" "Manor"      "Manor's"
  zrbyc_yk RBYC_PAYORS     "${z_docs}" "Payor"      "Payor's"
  zrbyc_yk RBYC_VOUCHED    "${z_docs}" "Vouch"      "Vouched"

  unset -f zrbyc_yk
  readonly ZRBYC_KINDLED=1
}

zrbyc_sentinel() {
  test "${ZRBYC_KINDLED:-}" = "1" || buc_die "Module rbyc not kindled - call zrbyc_kindle first"
}

# eof
