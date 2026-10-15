#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# Recipe Bottle Common Vocabulary — handbook-facing constants
#
# Kindled after regime loads (needs RBRR_PUBLIC_DOCS_URL).  Does not
# source or kindle anything itself — the consuming _cli.sh is responsible
# for sourcing buym_yelp.sh and this file, then calling zrbyc_kindle.
#
# Contents organized by kind:
#
#   1. Linked-term yelp fragments (RBYC_DEPOT, RBYC_VESSEL, ...) —
#      diastema-marked yelp fragments for interpolation into buh_line:
#          buh_line "A ${RBYC_DEPOT} is where images live."
#      Variant forms (_S plural, _P possessive) use alternate display text.
#
#   2. Probe markers (RBYC_PROBE_YES / RBYC_PROBE_NO) — pass/warn yawp
#      captures for handbook status indicators.
#
#   3. Handbook env var metadata (RBYC_HANDBOOK_*_NAME / _REF) — bare
#      strings naming the learner-facing env vars that handbooks teach
#      learners to export and consume. _NAME for export-teaching lines,
#      _REF for command-arg interpolation.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBYC_SOURCED:-}" || buc_die_now "Module rbyc multiply sourced - check sourcing hierarchy"
ZRBYC_SOURCED=1

######################################################################
# Module kindle

zrbyc_kindle() {
  test -z "${ZRBYC_KINDLED:-}" || buc_die_now "Module rbyc already kindled"

  zbuym_sentinel

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"


  # --- Paddock anchor inventory ---

  # Operations
  buym_link_yawp "${z_docs}" "Ordain"; local -r z_ordain="${z_buym_yelp}"; readonly RBYC_ORDAIN="${z_ordain}"
  buym_link_yawp "${z_docs}" "Conjure"; local -r z_conjure="${z_buym_yelp}"; readonly RBYC_CONJURE="${z_conjure}"
  buym_link_yawp "${z_docs}" "Bind"; local -r z_bind="${z_buym_yelp}"; readonly RBYC_BIND="${z_bind}"
  buym_link_yawp "${z_docs}" "Graft"; local -r z_graft="${z_buym_yelp}"; readonly RBYC_GRAFT="${z_graft}"
  buym_link_yawp "${z_docs}" "Kludge"; local -r z_kludge="${z_buym_yelp}"; readonly RBYC_KLUDGE="${z_kludge}"
  buym_link_yawp "${z_docs}" "Kludge" "Kludged"; local -r z_kludge_d="${z_buym_yelp}"; readonly RBYC_KLUDGE_D="${z_kludge_d}"
  buym_link_yawp "${z_docs}" "Summon"; local -r z_summon="${z_buym_yelp}"; readonly RBYC_SUMMON="${z_summon}"
  buym_link_yawp "${z_docs}" "Plumb"; local -r z_plumb="${z_buym_yelp}"; readonly RBYC_PLUMB="${z_plumb}"
  buym_link_yawp "${z_docs}" "Tally"; local -r z_tally="${z_buym_yelp}"; readonly RBYC_TALLY="${z_tally}"
  buym_link_yawp "${z_docs}" "Vouch"; local -r z_vouch="${z_buym_yelp}"; readonly RBYC_VOUCH="${z_vouch}"
  buym_link_yawp "${z_docs}" "Levy"; local -r z_levy="${z_buym_yelp}"; readonly RBYC_LEVY="${z_levy}"
  buym_link_yawp "${z_docs}" "Charge"; local -r z_charge="${z_buym_yelp}"; readonly RBYC_CHARGE="${z_charge}"
  buym_link_yawp "${z_docs}" "Charge" "Charged"; local -r z_charge_d="${z_buym_yelp}"; readonly RBYC_CHARGE_D="${z_charge_d}"
  buym_link_yawp "${z_docs}" "Quench"; local -r z_quench="${z_buym_yelp}"; readonly RBYC_QUENCH="${z_quench}"

  # Artifacts and infrastructure
  buym_link_yawp "${z_docs}" "Vessel"; local -r z_vessel="${z_buym_yelp}"; readonly RBYC_VESSEL="${z_vessel}"
  buym_link_yawp "${z_docs}" "Hallmark"; local -r z_hallmark="${z_buym_yelp}"; readonly RBYC_HALLMARK="${z_hallmark}"
  buym_link_yawp "${z_docs}" "Depot"; local -r z_depot="${z_buym_yelp}"; readonly RBYC_DEPOT="${z_depot}"
  buym_link_yawp "${z_docs}" "Reliquary"; local -r z_reliquary="${z_buym_yelp}"; readonly RBYC_RELIQUARY="${z_reliquary}"
  buym_link_yawp "${z_docs}" "Pouch"; local -r z_pouch="${z_buym_yelp}"; readonly RBYC_POUCH="${z_pouch}"
  buym_link_yawp "${z_docs}" "Nameplate"; local -r z_nameplate="${z_buym_yelp}"; readonly RBYC_NAMEPLATE="${z_nameplate}"

  # Egress modes
  buym_link_yawp "${z_docs}" "Tethered"; local -r z_tethered="${z_buym_yelp}"; readonly RBYC_TETHERED="${z_tethered}"
  buym_link_yawp "${z_docs}" "Airgap"; local -r z_airgap="${z_buym_yelp}"; readonly RBYC_AIRGAP="${z_airgap}"

  # Crucible components
  buym_link_yawp "${z_docs}" "Crucible"; local -r z_crucible="${z_buym_yelp}"; readonly RBYC_CRUCIBLE="${z_crucible}"
  buym_link_yawp "${z_docs}" "Enclave"; local -r z_enclave="${z_buym_yelp}"; readonly RBYC_ENCLAVE="${z_enclave}"
  buym_link_yawp "${z_docs}" "Sentry"; local -r z_sentry="${z_buym_yelp}"; readonly RBYC_SENTRY="${z_sentry}"
  buym_link_yawp "${z_docs}" "Sentry" "Sentries"; local -r z_sentry_s="${z_buym_yelp}"; readonly RBYC_SENTRY_S="${z_sentry_s}"
  buym_link_yawp "${z_docs}" "Sentry" "Sentry's"; local -r z_sentry_p="${z_buym_yelp}"; readonly RBYC_SENTRY_P="${z_sentry_p}"
  buym_link_yawp "${z_docs}" "Pentacle"; local -r z_pentacle="${z_buym_yelp}"; readonly RBYC_PENTACLE="${z_pentacle}"
  buym_link_yawp "${z_docs}" "Bottle"; local -r z_bottle="${z_buym_yelp}"; readonly RBYC_BOTTLE="${z_bottle}"

  # Roles
  buym_link_yawp "${z_docs}" "Payor"; local -r z_payor="${z_buym_yelp}"; readonly RBYC_PAYOR="${z_payor}"
  buym_link_yawp "${z_docs}" "Governor"; local -r z_governor="${z_buym_yelp}"; readonly RBYC_GOVERNOR="${z_governor}"
  buym_link_yawp "${z_docs}" "Director"; local -r z_director="${z_buym_yelp}"; readonly RBYC_DIRECTOR="${z_director}"
  buym_link_yawp "${z_docs}" "Retriever"; local -r z_retriever="${z_buym_yelp}"; readonly RBYC_RETRIEVER="${z_retriever}"

  # Infrastructure concepts
  buym_link_yawp "${z_docs}" "Tabtarget"; local -r z_tabtarget="${z_buym_yelp}"; readonly RBYC_TABTARGET="${z_tabtarget}"
  buym_link_yawp "${z_docs}" "Regime"; local -r z_regime="${z_buym_yelp}"; readonly RBYC_REGIME="${z_regime}"

  # Regime file types
  buym_link_yawp "${z_docs}" "RBRP"; local -r z_rbrp="${z_buym_yelp}"; readonly RBYC_RBRP="${z_rbrp}"
  buym_link_yawp "${z_docs}" "RBRR"; local -r z_rbrr="${z_buym_yelp}"; readonly RBYC_RBRR="${z_rbrr}"
  buym_link_yawp "${z_docs}" "RBRD"; local -r z_rbrd="${z_buym_yelp}"; readonly RBYC_RBRD="${z_rbrd}"
  buym_link_yawp "${z_docs}" "RBRN"; local -r z_rbrn="${z_buym_yelp}"; readonly RBYC_RBRN="${z_rbrn}"
  buym_link_yawp "${z_docs}" "RBRV"; local -r z_rbrv="${z_buym_yelp}"; readonly RBYC_RBRV="${z_rbrv}"
  buym_link_yawp "${z_docs}" "BURC"; local -r z_burc="${z_buym_yelp}"; readonly RBYC_BURC="${z_burc}"
  buym_link_yawp "${z_docs}" "BURS"; local -r z_burs="${z_buym_yelp}"; readonly RBYC_BURS="${z_burs}"
  buym_link_yawp "${z_docs}" "RBRO"; local -r z_rbro="${z_buym_yelp}"; readonly RBYC_RBRO="${z_rbro}"

  # --- Additional anchors used in handbooks ---

  buym_link_yawp "${z_docs}" "" "Recipe Bottle"; local -r z_recipe_bottle="${z_buym_yelp}"; readonly RBYC_RECIPE_BOTTLE="${z_recipe_bottle}"

  buym_link_yawp "${z_docs}" "Rack"; local -r z_rack="${z_buym_yelp}"; readonly RBYC_RACK="${z_rack}"
  buym_link_yawp "${z_docs}" "Manor"; local -r z_manor="${z_buym_yelp}"; readonly RBYC_MANOR="${z_manor}"
  buym_link_yawp "${z_docs}" "ccyolo"; local -r z_ccyolo="${z_buym_yelp}"; readonly RBYC_CCYOLO="${z_ccyolo}"
  buym_link_yawp "${z_docs}" "Ifrit"; local -r z_ifrit="${z_buym_yelp}"; readonly RBYC_IFRIT="${z_ifrit}"
  buym_link_yawp "${z_docs}" "Theurge"; local -r z_theurge="${z_buym_yelp}"; readonly RBYC_THEURGE="${z_theurge}"
  buym_link_yawp "${z_docs}" "Abjure"; local -r z_abjure="${z_buym_yelp}"; readonly RBYC_ABJURE="${z_abjure}"
  buym_link_yawp "${z_docs}" "Rekon"; local -r z_rekon="${z_buym_yelp}"; readonly RBYC_REKON="${z_rekon}"
  buym_link_yawp "${z_docs}" "Log"; local -r z_log="${z_buym_yelp}"; readonly RBYC_LOG="${z_log}"
  buym_link_yawp "${z_docs}" "Transcript"; local -r z_transcript="${z_buym_yelp}"; readonly RBYC_TRANSCRIPT="${z_transcript}"
  buym_link_yawp "${z_docs}" "Output"; local -r z_output="${z_buym_yelp}"; readonly RBYC_OUTPUT="${z_output}"
  buym_link_yawp "${z_docs}" "Provenance"; local -r z_provenance="${z_buym_yelp}"; readonly RBYC_PROVENANCE="${z_provenance}"
  buym_link_yawp "${z_docs}" "SBOM"; local -r z_sbom="${z_buym_yelp}"; readonly RBYC_SBOM="${z_sbom}"

  # --- Variant forms (plural, possessive) ---

  buym_link_yawp "${z_docs}" "Hallmark" "Hallmarks"; local -r z_hallmarks="${z_buym_yelp}"; readonly RBYC_HALLMARKS="${z_hallmarks}"
  buym_link_yawp "${z_docs}" "Nameplate" "Nameplates"; local -r z_nameplates="${z_buym_yelp}"; readonly RBYC_NAMEPLATES="${z_nameplates}"
  buym_link_yawp "${z_docs}" "Vessel" "Vessels"; local -r z_vessels="${z_buym_yelp}"; readonly RBYC_VESSELS="${z_vessels}"
  buym_link_yawp "${z_docs}" "Crucible" "Crucibles"; local -r z_crucibles="${z_buym_yelp}"; readonly RBYC_CRUCIBLES="${z_crucibles}"
  buym_link_yawp "${z_docs}" "Director" "Directors"; local -r z_directors="${z_buym_yelp}"; readonly RBYC_DIRECTORS="${z_directors}"
  buym_link_yawp "${z_docs}" "Retriever" "Retrievers"; local -r z_retrievers="${z_buym_yelp}"; readonly RBYC_RETRIEVERS="${z_retrievers}"
  buym_link_yawp "${z_docs}" "Tabtarget" "Tabtargets"; local -r z_tabtargets="${z_buym_yelp}"; readonly RBYC_TABTARGETS="${z_tabtargets}"
  buym_link_yawp "${z_docs}" "Regime" "Regimes"; local -r z_regimes="${z_buym_yelp}"; readonly RBYC_REGIMES="${z_regimes}"
  buym_link_yawp "${z_docs}" "Log" "Logs"; local -r z_logs="${z_buym_yelp}"; readonly RBYC_LOGS="${z_logs}"
  buym_link_yawp "${z_docs}" "Manor" "Manor's"; local -r z_manors="${z_buym_yelp}"; readonly RBYC_MANORS="${z_manors}"
  buym_link_yawp "${z_docs}" "Payor" "Payor's"; local -r z_payors="${z_buym_yelp}"; readonly RBYC_PAYORS="${z_payors}"
  buym_link_yawp "${z_docs}" "Conjure" "Conjured"; local -r z_conjured="${z_buym_yelp}"; readonly RBYC_CONJURED="${z_conjured}"
  buym_link_yawp "${z_docs}" "Vouch" "Vouched"; local -r z_vouched="${z_buym_yelp}"; readonly RBYC_VOUCHED="${z_vouched}"

  # --- Probe markers (pass/warn for handbook status probes) ---

  buym_pass_yawp " [*] "; local -r z_probe_yes="${z_buym_yelp}"; readonly RBYC_PROBE_YES="${z_probe_yes}"
  buym_warn_yawp " [ ] "; local -r z_probe_no="${z_buym_yelp}"; readonly RBYC_PROBE_NO="${z_probe_no}"

  # --- Handbook env var metadata (learner-facing shell vars) ---
  #
  # _NAME: env var name for buh_code "export ___=..." teaching lines.
  # _REF:  interpolation-ready "${HANDBOOK_*}" literal for buh_tt args.
  # Prefix centralizes HANDBOOK_↔ONBOARDING_ swap as a one-line change.

  readonly RBYC_HANDBOOK_ENV_PREFIX="HANDBOOK_"

  readonly RBYC_HANDBOOK_VESSEL_NAME="${RBYC_HANDBOOK_ENV_PREFIX}VESSEL"
  readonly RBYC_HANDBOOK_VESSEL_REF="\${${RBYC_HANDBOOK_VESSEL_NAME}}"

  readonly RBYC_HANDBOOK_NAMEPLATE_NAME="${RBYC_HANDBOOK_ENV_PREFIX}NAMEPLATE"
  readonly RBYC_HANDBOOK_NAMEPLATE_REF="\${${RBYC_HANDBOOK_NAMEPLATE_NAME}}"

  readonly RBYC_HANDBOOK_HALLMARK_NAME="${RBYC_HANDBOOK_ENV_PREFIX}HALLMARK"
  readonly RBYC_HANDBOOK_HALLMARK_REF="\${${RBYC_HANDBOOK_HALLMARK_NAME}}"

  readonly ZRBYC_KINDLED=1
}

zrbyc_sentinel() {
  test "${ZRBYC_KINDLED:-}" = "1" || buc_die_now "Module rbyc not kindled - call zrbyc_kindle first"
}

# eof
