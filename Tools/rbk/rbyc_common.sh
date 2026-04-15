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
# for sourcing buy_yelp.sh and this file, then calling zrbyc_kindle.
#
# Each constant is a pre-formatted yelp fragment suitable for
# interpolation into buh_line calls:
#   buh_line "A %b is where images live." "${RBYC_DEPOT}"
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

  zbuy_sentinel

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  # --- Paddock anchor inventory (38 base terms) ---

  # Operations
  readonly RBYC_ORDAIN=$(buy_link_capture    "${z_docs}" "Ordain")
  readonly RBYC_CONJURE=$(buy_link_capture   "${z_docs}" "Conjure")
  readonly RBYC_BIND=$(buy_link_capture      "${z_docs}" "Bind")
  readonly RBYC_GRAFT=$(buy_link_capture     "${z_docs}" "Graft")
  readonly RBYC_KLUDGE=$(buy_link_capture    "${z_docs}" "Kludge")
  readonly RBYC_ENSHRINE=$(buy_link_capture  "${z_docs}" "Enshrine")
  readonly RBYC_SUMMON=$(buy_link_capture    "${z_docs}" "Summon")
  readonly RBYC_PLUMB=$(buy_link_capture     "${z_docs}" "Plumb")
  readonly RBYC_TALLY=$(buy_link_capture     "${z_docs}" "Tally")
  readonly RBYC_VOUCH=$(buy_link_capture     "${z_docs}" "Vouch")
  readonly RBYC_LEVY=$(buy_link_capture      "${z_docs}" "Levy")
  readonly RBYC_CHARGE=$(buy_link_capture    "${z_docs}" "Charge")
  readonly RBYC_QUENCH=$(buy_link_capture    "${z_docs}" "Quench")

  # Artifacts and infrastructure
  readonly RBYC_VESSEL=$(buy_link_capture    "${z_docs}" "Vessel")
  readonly RBYC_HALLMARK=$(buy_link_capture  "${z_docs}" "Hallmark")
  readonly RBYC_DEPOT=$(buy_link_capture     "${z_docs}" "Depot")
  readonly RBYC_RELIQUARY=$(buy_link_capture "${z_docs}" "Reliquary")
  readonly RBYC_POUCH=$(buy_link_capture     "${z_docs}" "Pouch")
  readonly RBYC_NAMEPLATE=$(buy_link_capture "${z_docs}" "Nameplate")

  # Egress modes
  readonly RBYC_TETHERED=$(buy_link_capture  "${z_docs}" "Tethered")
  readonly RBYC_AIRGAP=$(buy_link_capture    "${z_docs}" "Airgap")

  # Crucible components
  readonly RBYC_CRUCIBLE=$(buy_link_capture  "${z_docs}" "Crucible")
  readonly RBYC_SENTRY=$(buy_link_capture    "${z_docs}" "Sentry")
  readonly RBYC_PENTACLE=$(buy_link_capture  "${z_docs}" "Pentacle")
  readonly RBYC_BOTTLE=$(buy_link_capture    "${z_docs}" "Bottle")

  # Roles
  readonly RBYC_PAYOR=$(buy_link_capture     "${z_docs}" "Payor")
  readonly RBYC_GOVERNOR=$(buy_link_capture  "${z_docs}" "Governor")
  readonly RBYC_DIRECTOR=$(buy_link_capture  "${z_docs}" "Director")
  readonly RBYC_RETRIEVER=$(buy_link_capture "${z_docs}" "Retriever")
  readonly RBYC_CHARTER=$(buy_link_capture   "${z_docs}" "Charter")
  readonly RBYC_KNIGHT=$(buy_link_capture    "${z_docs}" "Knight")

  # Infrastructure concepts
  readonly RBYC_TABTARGET=$(buy_link_capture "${z_docs}" "Tabtarget")
  readonly RBYC_REGIME=$(buy_link_capture    "${z_docs}" "Regime")

  # Regime file types
  readonly RBYC_RBRP=$(buy_link_capture      "${z_docs}" "RBRP")
  readonly RBYC_RBRR=$(buy_link_capture      "${z_docs}" "RBRR")
  readonly RBYC_RBRN=$(buy_link_capture      "${z_docs}" "RBRN")
  readonly RBYC_RBRV=$(buy_link_capture      "${z_docs}" "RBRV")
  readonly RBYC_BURC=$(buy_link_capture      "${z_docs}" "BURC")
  readonly RBYC_BURS=$(buy_link_capture      "${z_docs}" "BURS")

  # --- Additional anchors used in handbooks ---

  readonly RBYC_RACK=$(buy_link_capture      "${z_docs}" "Rack")
  readonly RBYC_MANOR=$(buy_link_capture     "${z_docs}" "Manor")
  readonly RBYC_RBRA=$(buy_link_capture      "${z_docs}" "RBRA")
  readonly RBYC_CCYOLO=$(buy_link_capture    "${z_docs}" "ccyolo")
  readonly RBYC_ABJURE=$(buy_link_capture    "${z_docs}" "Abjure")
  readonly RBYC_REKON=$(buy_link_capture     "${z_docs}" "Rekon")
  readonly RBYC_LOG=$(buy_link_capture       "${z_docs}" "Log")
  readonly RBYC_TRANSCRIPT=$(buy_link_capture "${z_docs}" "Transcript")

  # --- Variant forms (plural, possessive) ---

  readonly RBYC_HALLMARKS=$(buy_link_capture  "${z_docs}" "Hallmark"   "Hallmarks")
  readonly RBYC_VESSELS=$(buy_link_capture     "${z_docs}" "Vessel"     "Vessels")
  readonly RBYC_DIRECTORS=$(buy_link_capture   "${z_docs}" "Director"   "Directors")
  readonly RBYC_RETRIEVERS=$(buy_link_capture  "${z_docs}" "Retriever"  "Retrievers")
  readonly RBYC_TABTARGETS=$(buy_link_capture  "${z_docs}" "Tabtarget"  "Tabtargets")
  readonly RBYC_REGIMES=$(buy_link_capture     "${z_docs}" "Regime"     "Regimes")
  readonly RBYC_LOGS=$(buy_link_capture        "${z_docs}" "Log"        "Logs")
  readonly RBYC_MANORS=$(buy_link_capture      "${z_docs}" "Manor"      "Manor's")
  readonly RBYC_PAYORS=$(buy_link_capture      "${z_docs}" "Payor"      "Payor's")
  readonly RBYC_VOUCHED=$(buy_link_capture     "${z_docs}" "Vouch"      "Vouched")

  readonly ZRBYC_KINDLED=1
}

zrbyc_sentinel() {
  test "${ZRBYC_KINDLED:-}" = "1" || buc_die "Module rbyc not kindled - call zrbyc_kindle first"
}

# eof
