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

  # Payor depot/governor commands
  local z_mod="rbgp_cli.sh"
  buz_enroll RBZ_LEVY_DEPOT                 "rbw-PL"       "${z_mod}" "rbgp_depot_levy"
  buz_enroll RBZ_PAYOR_INSTALL              "rbw-gPI"      "${z_mod}" "rbgp_payor_install"
  buz_enroll RBZ_UNMAKE_DEPOT               "rbw-PU"       "${z_mod}" "rbgp_depot_unmake"
  buz_enroll RBZ_MANTLE_GOVERNOR            "rbw-PM"       "${z_mod}" "rbgp_governor_mantle"
  # Payor guide/manual procedures (rbgm_cli.sh)
  z_mod="rbgm_cli.sh"
  buz_enroll RBZ_PAYOR_ESTABLISH            "rbw-gPE"      "${z_mod}" "rbgm_payor_establish"
  buz_enroll RBZ_PAYOR_REFRESH              "rbw-gPR"      "${z_mod}" "rbgm_payor_refresh"
  buz_enroll RBZ_QUOTA_BUILD                "rbw-gq"       "${z_mod}" "rbgm_quota_build"
  buz_enroll RBZ_GITLAB_SETUP               "rbw-gPL"      "${z_mod}" "rbgm_gitlab_setup"
  buz_enroll RBZ_ONBOARDING                 "rbw-gO"       "${z_mod}" "rbgm_onboarding"
  # Marshal operations (rblm_cli.sh — Lifecycle Marshal)
  buz_enroll RBZ_MARSHAL_ZERO              "rbw-MZ"       "rblm_cli.sh" "rblm_zero"
  buz_enroll RBZ_MARSHAL_PROOF             "rbw-MP"       "rblm_cli.sh" "rblm_proof"

  # Payor depot operations (rbgp_cli.sh)
  z_mod="rbgp_cli.sh"
  buz_enroll RBZ_LIST_DEPOT                 "rbw-Pl"      "${z_mod}" "rbgp_depot_list"

  # Governor commands
  z_mod="rbgg_cli.sh"
  buz_enroll RBZ_CHARTER_RETRIEVER         "rbw-GC"        "${z_mod}" "rbgg_charter_retriever"
  buz_enroll RBZ_KNIGHT_DIRECTOR           "rbw-GK"        "${z_mod}" "rbgg_knight_director"

  z_mod="rbgg_cli.sh"
  buz_enroll RBZ_LIST_SERVICE_ACCOUNTS     "rbw-Gl"        "${z_mod}" "rbgg_list_service_accounts"
  buz_enroll RBZ_FORFEIT_SERVICE_ACCOUNT   "rbw-GF"        "${z_mod}" "rbgg_forfeit_service_account"

  # Retriever/Image commands (rbw-R* colophon family → rbf_cli.sh)
  z_mod="rbf_cli.sh"
  buz_enroll RBZ_ENSHRINE_VESSEL             "rbw-DE"       "${z_mod}" "rbf_enshrine"
  buz_enroll RBZ_ORDAIN_CONSECRATION        "rbw-DO"       "${z_mod}" "rbf_ordain"
  buz_enroll RBZ_ABJURE_CONSECRATION        "rbw-DA"       "${z_mod}" "rbf_abjure"
  buz_enroll RBZ_SUMMON_CONSECRATION                 "rbw-Rs"       "${z_mod}" "rbf_summon"
  buz_enroll RBZ_INSCRIBE_RELIQUARY         "rbw-DI"       "${z_mod}" "rbf_inscribe"
  buz_enroll RBZ_DELETE_IMAGE               "rbw-DD"       "${z_mod}" "rbf_delete"
  buz_enroll RBZ_RETRIEVE_IMAGE             "rbw-Rr"       "${z_mod}" "rbf_retrieve"
  buz_enroll RBZ_INSPECT_FULL              "rbw-RiF"      "${z_mod}" "rbf_inspect_full"
  buz_enroll RBZ_INSPECT_COMPACT           "rbw-Ric"      "${z_mod}" "rbf_inspect_compact"
  buz_enroll RBZ_TALLY_CONSECRATIONS        "rbw-Dt"       "${z_mod}" "rbf_tally"
  buz_enroll RBZ_VOUCH_CONSECRATIONS        "rbw-DV"       "${z_mod}" "rbf_batch_vouch"
  buz_enroll RBZ_KLUDGE_VESSEL             "rbw-ak"       "${z_mod}" "rbf_kludge"

  # Regime operations
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

  # Nameplate regime list
  z_mod="rbrn_cli.sh"
  buz_enroll RBZ_LIST_NAMEPLATES "rbw-rnl" "${z_mod}" "rbrn_list"

  # Cross-nameplate operations
  buz_enroll RBZ_SURVEY_NAMEPLATES "rbw-ni" "${z_mod}" "rbrn_survey"
  buz_enroll RBZ_AUDIT_NAMEPLATES  "rbw-nv" "${z_mod}" "rbrn_audit"

  # Bottle operations (imprint channel sets BUZ_FOLIO from BURD_TOKEN_3)
  z_mod="rbob_cli.sh"
  buz_enroll RBZ_CRUCIBLE_CHARGE "rbw-cC" "${z_mod}" "rbob_charge"         "imprint"
  buz_enroll RBZ_CRUCIBLE_QUENCH "rbw-cQ" "${z_mod}" "rbob_quench"       "imprint"
  buz_enroll RBZ_CRUCIBLE_HAIL  "rbw-ch" "${z_mod}" "rbob_hail"          "imprint"
  buz_enroll RBZ_CRUCIBLE_RACK  "rbw-cr" "${z_mod}" "rbob_rack"          "imprint"
  buz_enroll RBZ_CRUCIBLE_SCRY  "rbw-cs" "${z_mod}" "rbob_scry"          "imprint"
  buz_enroll RBZ_BOTTLE_IFRIT   "rbw-Ic" "${z_mod}" "rbob_ifrit_client"  "imprint"
  buz_enroll RBZ_BOTTLE_SORTIE  "rbw-Is" "${z_mod}" "rbob_ifrit_sortie"  "imprint"

  # Qualification operations
  z_mod="rbq_cli.sh"
  buz_enroll RBZ_QUALIFY_FAST    "rbw-Qf" "${z_mod}" "rbq_qualify_fast"
  buz_enroll RBZ_QUALIFY_RELEASE "rbw-QR" "${z_mod}" "rbq_qualify_release"

  # Test operations (module is testbench, command is colophon)
  z_mod="rbtb_testbench.sh"
  buz_enroll RBZ_TEST_FIXTURE "rbw-tf" "${z_mod}" "rbw-tf" "imprint"
  buz_enroll RBZ_TEST_SUITE   "rbw-ts" "${z_mod}" "rbw-ts" "imprint"
  buz_enroll RBZ_TEST_ONE     "rbw-to" "${z_mod}" "rbw-to"

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
