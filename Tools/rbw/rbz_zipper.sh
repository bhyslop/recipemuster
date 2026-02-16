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

# shellcheck disable=SC2034,SC2154
zrbz_kindle() {
  test -z "${ZRBZ_KINDLED:-}" || buc_die "rbz already kindled"

  # Verify buz zipper is kindled (CLI furnish must kindle buz first)
  zbuz_sentinel

  # Payor commands
  local z_mod="rbgp_cli.sh"
  buz_blazon RBZ_CREATE_DEPOT   "rbw-PC" "${z_mod}" "rbgp_depot_create"
  buz_blazon RBZ_PAYOR_INSTALL  "rbw-PI" "${z_mod}" "rbgp_payor_install"
  buz_blazon RBZ_DESTROY_DEPOT  "rbw-PD" "${z_mod}" "rbgp_depot_destroy"
  buz_blazon RBZ_GOVERNOR_RESET "rbw-PG" "${z_mod}" "rbgp_governor_reset"
  z_mod="rbgm_cli.sh"
  buz_blazon RBZ_PAYOR_ESTABLISH "rbw-PE" "${z_mod}" "rbgm_payor_establish"
  buz_blazon RBZ_PAYOR_REFRESH   "rbw-PR" "${z_mod}" "rbgm_payor_refresh"
  buz_blazon RBZ_QUOTA_BUILD     "rbw-QB" "${z_mod}" "rbgm_quota_build"

  # General depot operations
  z_mod="rbgp_cli.sh"
  buz_blazon RBZ_LIST_DEPOT "rbw-ld" "${z_mod}" "rbgp_depot_list"

  # Governor commands
  z_mod="rbgg_cli.sh"
  buz_blazon RBZ_CREATE_RETRIEVER "rbw-GR" "${z_mod}" "rbgg_create_retriever"
  buz_blazon RBZ_CREATE_DIRECTOR  "rbw-GD" "${z_mod}" "rbgg_create_director"

  # Admin commands
  z_mod="rbgm_cli.sh"
  buz_blazon RBZ_ADMIN_ESTABLISH "rbw-ps" "${z_mod}" "rbgm_payor_establish"
  z_mod="rbgg_cli.sh"
  buz_blazon RBZ_LIST_SERVICE_ACCOUNTS  "rbw-Gl" "${z_mod}" "rbgg_list_service_accounts"
  buz_blazon RBZ_DELETE_SERVICE_ACCOUNT "rbw-GS" "${z_mod}" "rbgg_delete_service_account"

  # Ark commands (paired -image + -about artifacts)
  z_mod="rbf_cli.sh"
  buz_blazon RBZ_ABJURE_ARK  "rbw-aA" "${z_mod}" "rbf_abjure"
  buz_blazon RBZ_BESEECH_ARK "rbw-ab" "${z_mod}" "rbf_beseech"
  buz_blazon RBZ_CONJURE_ARK "rbw-aC" "${z_mod}" "rbf_build"
  buz_blazon RBZ_SUMMON_ARK  "rbw-as" "${z_mod}" "rbf_summon"

  # Image commands (single artifact)
  buz_blazon RBZ_BUILD_IMAGE    "rbw-iB" "${z_mod}" "rbf_build"
  buz_blazon RBZ_DELETE_IMAGE   "rbw-iD" "${z_mod}" "rbf_delete"
  buz_blazon RBZ_LIST_IMAGES    "rbw-il" "${z_mod}" "rbf_list"
  buz_blazon RBZ_RETRIEVE_IMAGE "rbw-ir" "${z_mod}" "rbf_retrieve"

  # Nameplate regime operations
  z_mod="rbrn_cli.sh"
  buz_blazon RBZ_RENDER_NAMEPLATE   "rbw-rnr" "${z_mod}" "render"
  buz_blazon RBZ_VALIDATE_NAMEPLATE "rbw-rnv" "${z_mod}" "validate"

  # Vessel regime operations
  z_mod="rbrv_cli.sh"
  buz_blazon RBZ_RENDER_VESSEL   "rbw-rvr" "${z_mod}" "render"
  buz_blazon RBZ_VALIDATE_VESSEL "rbw-rvv" "${z_mod}" "validate"

  # Repo regime operations
  z_mod="rbrr_cli.sh"
  buz_blazon RBZ_RENDER_REPO     "rbw-rrr" "${z_mod}" "render"
  buz_blazon RBZ_VALIDATE_REPO   "rbw-rrv" "${z_mod}" "validate"
  buz_blazon RBZ_REFRESH_GCB_PINS "rbw-rrg" "${z_mod}" "refresh_gcb_pins"

  # Payor regime operations
  z_mod="rbrp_cli.sh"
  buz_blazon RBZ_RENDER_PAYOR   "rbw-rpr" "${z_mod}" "render"
  buz_blazon RBZ_VALIDATE_PAYOR "rbw-rpv" "${z_mod}" "validate"

  # OAuth regime operations
  z_mod="rbro_cli.sh"
  buz_blazon RBZ_RENDER_OAUTH   "rbw-ror" "${z_mod}" "render"
  buz_blazon RBZ_VALIDATE_OAUTH "rbw-rov" "${z_mod}" "validate"

  # Station regime operations
  z_mod="rbrs_cli.sh"
  buz_blazon RBZ_RENDER_STATION   "rbw-rsr" "${z_mod}" "render"
  buz_blazon RBZ_VALIDATE_STATION "rbw-rsv" "${z_mod}" "validate"

  # Auth regime operations
  z_mod="rbra_cli.sh"
  buz_blazon RBZ_RENDER_AUTH   "rbw-rar" "${z_mod}" "render"
  buz_blazon RBZ_VALIDATE_AUTH "rbw-rav" "${z_mod}" "validate"
  buz_blazon RBZ_LIST_AUTH     "rbw-ral" "${z_mod}" "list"

  # Cross-nameplate operations
  z_mod="rbrn_cli.sh"
  buz_blazon RBZ_SURVEY_NAMEPLATES "rbw-ni" "${z_mod}" "survey"
  buz_blazon RBZ_AUDIT_NAMEPLATES  "rbw-nv" "${z_mod}" "audit"

  # Bottle operations (imprint-translated by workbench case arm, not zbuz_exec_lookup)
  z_mod="rbob_cli.sh"
  buz_blazon RBZ_BOTTLE_START   "rbw-s" "${z_mod}" "rbob_start"
  buz_blazon RBZ_BOTTLE_SENTRY  "rbw-S" "${z_mod}" "rbob_connect_sentry"
  buz_blazon RBZ_BOTTLE_CENSER  "rbw-C" "${z_mod}" "rbob_connect_censer"
  buz_blazon RBZ_BOTTLE_CONNECT "rbw-B" "${z_mod}" "rbob_connect_bottle"
  buz_blazon RBZ_BOTTLE_OBSERVE "rbw-o" "${z_mod}" "rbob_observe"

  # Qualification operations
  z_mod="rbq_cli.sh"
  buz_blazon RBZ_QUALIFY_ALL "rbw-qa" "${z_mod}" "qualify_all"

  # Test operations (module is testbench, command is colophon)
  z_mod="rbtb_testbench.sh"
  buz_blazon RBZ_TEST_ALL              "rbw-ta"  "${z_mod}" "rbw-ta"
  buz_blazon RBZ_TEST_SUITE            "rbw-ts"  "${z_mod}" "rbw-ts"
  buz_blazon RBZ_TEST_ONE              "rbw-to"  "${z_mod}" "rbw-to"
  buz_blazon RBZ_TEST_NAMEPLATE        "rbw-tn"  "${z_mod}" "rbw-tn"
  buz_blazon RBZ_TEST_REGIME           "rbw-trg" "${z_mod}" "rbw-trg"
  buz_blazon RBZ_TEST_REGIME_CREDENTIALS "rbw-trc" "${z_mod}" "rbw-trc"

  ZRBZ_KINDLED=1
}

######################################################################
# Internal sentinel

zrbz_sentinel() {
  test "${ZRBZ_KINDLED:-}" = "1" || buc_die "Module rbz not kindled - call zrbz_kindle first"
}

# eof
