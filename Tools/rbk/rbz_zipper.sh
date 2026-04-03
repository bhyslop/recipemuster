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
# RBW Zipper - Colophon registry for RBW workbench dispatch

set -euo pipefail

# Multiple inclusion guard
test -z "${ZRBZ_SOURCED:-}" || return 0
ZRBZ_SOURCED=1

######################################################################
# Colophon registry initialization

zrbz_kindle() {
  test -z "${ZRBZ_KINDLED:-}" || buc_die "rbz already kindled"

  # Verify buz zipper is kindled (CLI furnish must kindle buz first)
  zbuz_sentinel

  # Colophon group taxonomy (domain-object-organized categories)
  buz_group RBZ__GROUP_ACCOUNTS   "rbw-a"   "Google Cloud service accounts"
  buz_group RBZ__GROUP_CRUCIBLE   "rbw-c"   "Crucible container runtime"
  buz_group RBZ__GROUP_DEPOT      "rbw-d"   "GCP project infrastructure and supply chain"
  buz_group RBZ__GROUP_GUIDE      "rbw-g"   "Human-directed procedures"
  buz_group RBZ__GROUP_HALLMARK   "rbw-h"   "Registry artifact lifecycle"
  buz_group RBZ__GROUP_IFRIT      "rbw-I"   "Ifrit attack binary"
  buz_group RBZ__GROUP_IMAGE      "rbw-i"   "Container image operations"
  buz_group RBZ__GROUP_MARSHAL    "rbw-M"   "Lifecycle marshal"
  buz_group RBZ__GROUP_NAMEPLATE  "rbw-n"   "Cross-nameplate operations"
  buz_group RBZ__GROUP_REGIME     "rbw-r"   "Regime config files"
  buz_group RBZ__GROUP_THEURGE    "rbw-t"   "Theurge test infrastructure"

  # Accounts — Google Cloud service accounts (rbw-a)
  local z_mod="rbgp_cli.sh"
  buz_enroll RBZ_MANTLE_GOVERNOR            "rbw-aM"       "${z_mod}" "rbgp_governor_mantle"
  z_mod="rbgg_cli.sh"
  buz_enroll RBZ_CHARTER_RETRIEVER          "rbw-aC"       "${z_mod}" "rbgg_charter_retriever"
  buz_enroll RBZ_KNIGHT_DIRECTOR            "rbw-aK"       "${z_mod}" "rbgg_knight_director"
  buz_enroll RBZ_LIST_SERVICE_ACCOUNTS      "rbw-aL"       "${z_mod}" "rbgg_list_service_accounts"
  buz_enroll RBZ_FORFEIT_SERVICE_ACCOUNT    "rbw-aF"       "${z_mod}" "rbgg_forfeit_service_account"

  # Crucible — container runtime (rbw-c)
  z_mod="rbob_cli.sh"
  buz_enroll RBZ_CRUCIBLE_CHARGE  "rbw-cC"  "${z_mod}" "rbob_charge"         "imprint"
  buz_enroll RBZ_CRUCIBLE_QUENCH  "rbw-cQ"  "${z_mod}" "rbob_quench"         "imprint"
  buz_enroll RBZ_CRUCIBLE_HAIL   "rbw-ch"  "${z_mod}" "rbob_hail"           "param1"
  buz_enroll RBZ_CRUCIBLE_RACK   "rbw-cr"  "${z_mod}" "rbob_rack"           "param1"
  buz_enroll RBZ_CRUCIBLE_SCRY   "rbw-cs"  "${z_mod}" "rbob_scry"           "param1"
  buz_enroll RBZ_CRUCIBLE_WRIT   "rbw-cw"  "${z_mod}" "rbob_writ"           "imprint"
  buz_enroll RBZ_CRUCIBLE_FIAT   "rbw-cf"  "${z_mod}" "rbob_fiat"           "imprint"
  buz_enroll RBZ_CRUCIBLE_BARK   "rbw-cb"  "${z_mod}" "rbob_bark"           "imprint"
  buz_enroll RBZ_CRUCIBLE_ACTIVE  "rbw-cic" "${z_mod}" "rbob_charged"        "param1"

  # Depot — GCP project infrastructure (rbw-d, UPPER=mutates, lower=read)
  z_mod="rbgp_cli.sh"
  buz_enroll RBZ_LEVY_DEPOT                 "rbw-dL"       "${z_mod}" "rbgp_depot_levy"
  buz_enroll RBZ_UNMAKE_DEPOT               "rbw-dU"       "${z_mod}" "rbgp_depot_unmake"
  buz_enroll RBZ_LIST_DEPOT                 "rbw-dl"       "${z_mod}" "rbgp_depot_list"
  z_mod="rbfd_cli.sh"
  buz_enroll RBZ_ENSHRINE_VESSEL            "rbw-dE"       "${z_mod}" "rbfd_enshrine"
  z_mod="rbfl_cli.sh"
  buz_enroll RBZ_INSCRIBE_RELIQUARY         "rbw-dI"       "${z_mod}" "rbfl_inscribe"

  # Guide — human-directed procedures (rbw-g)
  z_mod="rbgp_cli.sh"
  buz_enroll RBZ_PAYOR_INSTALL              "rbw-gPI"      "${z_mod}" "rbgp_payor_install"
  z_mod="rbgm_cli.sh"
  buz_enroll RBZ_PAYOR_ESTABLISH            "rbw-gPE"      "${z_mod}" "rbgm_payor_establish"
  buz_enroll RBZ_PAYOR_REFRESH              "rbw-gPR"      "${z_mod}" "rbgm_payor_refresh"
  buz_enroll RBZ_QUOTA_BUILD                "rbw-gq"       "${z_mod}" "rbgm_quota_build"
  buz_enroll RBZ_ONBOARDING                 "rbw-gO"       "${z_mod}" "rbgm_onboarding"

  # Hallmark — registry artifact lifecycle (rbw-h, UPPER=mutates GAR, lower=read/local)
  z_mod="rbfd_cli.sh"
  buz_enroll RBZ_ORDAIN_HALLMARK            "rbw-hO"       "${z_mod}" "rbfd_ordain"
  buz_enroll RBZ_KLUDGE_VESSEL              "rbw-hk"       "${z_mod}" "rbfd_kludge"
  z_mod="rbfl_cli.sh"
  buz_enroll RBZ_ABJURE_HALLMARK            "rbw-hA"       "${z_mod}" "rbfl_abjure"
  buz_enroll RBZ_TALLY_HALLMARKS            "rbw-ht"       "${z_mod}" "rbfl_tally"
  z_mod="rbfv_cli.sh"
  buz_enroll RBZ_VOUCH_HALLMARKS            "rbw-hV"       "${z_mod}" "rbfv_batch_vouch"
  z_mod="rbfr_cli.sh"
  buz_enroll RBZ_SUMMON_HALLMARK            "rbw-hs"       "${z_mod}" "rbfr_summon"
  z_mod="rbfc_cli.sh"
  buz_enroll RBZ_PLUMB_FULL                 "rbw-hpf"      "${z_mod}" "rbfc_plumb_full"
  buz_enroll RBZ_PLUMB_COMPACT              "rbw-hpc"      "${z_mod}" "rbfc_plumb_compact"

  # Ifrit — attack binary (rbw-I)
  z_mod="rbob_cli.sh"
  buz_enroll RBZ_BOTTLE_IFRIT    "rbw-Ic"  "${z_mod}" "rbob_ifrit_client"   "imprint"
  buz_enroll RBZ_BOTTLE_SORTIE   "rbw-Is"  "${z_mod}" "rbob_ifrit_sortie"   "imprint"

  # Image — container image operations (rbw-i, UPPER=mutates, lower=read)
  z_mod="rbfl_cli.sh"
  buz_enroll RBZ_JETTISON_IMAGE             "rbw-iJ"       "${z_mod}" "rbfl_jettison"
  z_mod="rbfr_cli.sh"
  buz_enroll RBZ_WREST_IMAGE                "rbw-iw"       "${z_mod}" "rbfr_wrest"

  # Marshal — lifecycle (rbw-M)
  buz_enroll RBZ_MARSHAL_ZERO               "rbw-MZ"       "rblm_cli.sh" "rblm_zero"
  buz_enroll RBZ_MARSHAL_PROOF              "rbw-MP"       "rblm_cli.sh" "rblm_proof"

  # Nameplate — cross-nameplate operations (rbw-n)
  z_mod="rbrn_cli.sh"
  buz_enroll RBZ_LIST_NAMEPLATES             "rbw-rnl"      "${z_mod}" "rbrn_list"
  buz_enroll RBZ_SURVEY_NAMEPLATES           "rbw-ni"       "${z_mod}" "rbrn_survey"
  buz_enroll RBZ_AUDIT_NAMEPLATES            "rbw-nv"       "${z_mod}" "rbrn_audit"

  # Regime — config files (rbw-r)
  z_mod="rbrn_cli.sh"
  buz_enroll RBZ_RENDER_NAMEPLATE           "rbw-rnr"      "${z_mod}" "rbrn_render"   "param1"
  buz_enroll RBZ_VALIDATE_NAMEPLATE         "rbw-rnv"      "${z_mod}" "rbrn_validate" "param1"
  z_mod="rbrv_cli.sh"
  buz_enroll RBZ_LIST_VESSELS               "rbw-rvl"      "${z_mod}" "rbrv_list"
  buz_enroll RBZ_RENDER_VESSEL              "rbw-rvr"      "${z_mod}" "rbrv_render"   "param1"
  buz_enroll RBZ_VALIDATE_VESSEL            "rbw-rvv"      "${z_mod}" "rbrv_validate" "param1"
  z_mod="rbrr_cli.sh"
  buz_enroll RBZ_RENDER_REPO                "rbw-rrr"      "${z_mod}" "rbrr_render"
  buz_enroll RBZ_VALIDATE_REPO              "rbw-rrv"      "${z_mod}" "rbrr_validate"
  z_mod="rbrp_cli.sh"
  buz_enroll RBZ_RENDER_PAYOR               "rbw-rpr"      "${z_mod}" "rbrp_render"
  buz_enroll RBZ_VALIDATE_PAYOR             "rbw-rpv"      "${z_mod}" "rbrp_validate"
  z_mod="rbro_cli.sh"
  buz_enroll RBZ_RENDER_OAUTH               "rbw-ror"      "${z_mod}" "rbro_render"
  buz_enroll RBZ_VALIDATE_OAUTH             "rbw-rov"      "${z_mod}" "rbro_validate"
  z_mod="rbrs_cli.sh"
  buz_enroll RBZ_RENDER_STATION             "rbw-rsr"      "${z_mod}" "rbrs_render"
  buz_enroll RBZ_VALIDATE_STATION           "rbw-rsv"      "${z_mod}" "rbrs_validate"
  z_mod="rbra_cli.sh"
  buz_enroll RBZ_RENDER_AUTH                "rbw-rar"      "${z_mod}" "rbra_render"   "param1"
  buz_enroll RBZ_VALIDATE_AUTH              "rbw-rav"      "${z_mod}" "rbra_validate" "param1"
  buz_enroll RBZ_LIST_AUTH                  "rbw-ral"      "${z_mod}" "rbra_list"

  # Theurge — test infrastructure (rbw-t)
  z_mod="rbob_cli.sh"
  buz_enroll RBZ_THEURGE_KLUDGE  "rbw-tK"  "${z_mod}" "rbob_kludge"         "imprint"
  buz_enroll RBZ_THEURGE_ORDAIN  "rbw-tO"  "${z_mod}" "rbob_ordain"         "imprint"
  z_mod="rbq_cli.sh"
  buz_enroll RBZ_QUALIFY_FAST    "rbw-tf"   "${z_mod}" "rbq_qualify_fast"
  buz_enroll RBZ_QUALIFY_RELEASE "rbw-tr"   "${z_mod}" "rbq_qualify_release"

  readonly ZRBZ_COLOPHON_MANIFEST="${z_buz_colophon_roll[*]}"

  readonly ZRBZ_KINDLED=1
}

######################################################################
# Healthcheck (validates all enrolled tabtargets exist on disk)

zrbz_healthcheck() {
  zrbz_sentinel
  buz_healthcheck
}

######################################################################
# Internal sentinel

zrbz_sentinel() {
  test "${ZRBZ_KINDLED:-}" = "1" || buc_die "Module rbz not kindled - call zrbz_kindle first"
}

# eof
