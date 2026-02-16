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

  # z_buz_register_colophon set by buz_register (cross-module return channel)
  # Module field is CLI script basename (used by zbuz_exec_lookup)

  # Payor commands
  buz_register "rbw-PC" "rbgp_cli.sh" "rbgp_depot_create"
  RBZ_CREATE_DEPOT="${z_buz_register_colophon}"
  buz_register "rbw-PI" "rbgp_cli.sh" "rbgp_payor_install"
  RBZ_PAYOR_INSTALL="${z_buz_register_colophon}"
  buz_register "rbw-PD" "rbgp_cli.sh" "rbgp_depot_destroy"
  RBZ_DESTROY_DEPOT="${z_buz_register_colophon}"
  buz_register "rbw-PE" "rbgm_cli.sh" "rbgm_payor_establish"
  RBZ_PAYOR_ESTABLISH="${z_buz_register_colophon}"
  buz_register "rbw-PG" "rbgp_cli.sh" "rbgp_governor_reset"
  RBZ_GOVERNOR_RESET="${z_buz_register_colophon}"
  buz_register "rbw-PR" "rbgm_cli.sh" "rbgm_payor_refresh"
  RBZ_PAYOR_REFRESH="${z_buz_register_colophon}"
  buz_register "rbw-QB" "rbgm_cli.sh" "rbgm_quota_build"
  RBZ_QUOTA_BUILD="${z_buz_register_colophon}"

  # General depot operations
  buz_register "rbw-ld" "rbgp_cli.sh" "rbgp_depot_list"
  RBZ_LIST_DEPOT="${z_buz_register_colophon}"

  # Governor commands
  buz_register "rbw-GR" "rbgg_cli.sh" "rbgg_create_retriever"
  RBZ_CREATE_RETRIEVER="${z_buz_register_colophon}"
  buz_register "rbw-GD" "rbgg_cli.sh" "rbgg_create_director"
  RBZ_CREATE_DIRECTOR="${z_buz_register_colophon}"

  # Admin commands
  buz_register "rbw-ps" "rbgm_cli.sh" "rbgm_payor_establish"
  RBZ_ADMIN_ESTABLISH="${z_buz_register_colophon}"
  buz_register "rbw-Gl" "rbgg_cli.sh" "rbgg_list_service_accounts"
  RBZ_LIST_SERVICE_ACCOUNTS="${z_buz_register_colophon}"
  buz_register "rbw-GS" "rbgg_cli.sh" "rbgg_delete_service_account"
  RBZ_DELETE_SERVICE_ACCOUNT="${z_buz_register_colophon}"

  # Ark commands (paired -image + -about artifacts)
  buz_register "rbw-aA" "rbf_cli.sh" "rbf_abjure"
  RBZ_ABJURE_ARK="${z_buz_register_colophon}"
  buz_register "rbw-ab" "rbf_cli.sh" "rbf_beseech"
  RBZ_BESEECH_ARK="${z_buz_register_colophon}"
  buz_register "rbw-aC" "rbf_cli.sh" "rbf_build"
  RBZ_CONJURE_ARK="${z_buz_register_colophon}"
  buz_register "rbw-as" "rbf_cli.sh" "rbf_summon"
  RBZ_SUMMON_ARK="${z_buz_register_colophon}"

  # Image commands (single artifact)
  buz_register "rbw-iB" "rbf_cli.sh" "rbf_build"
  RBZ_BUILD_IMAGE="${z_buz_register_colophon}"
  buz_register "rbw-iD" "rbf_cli.sh" "rbf_delete"
  RBZ_DELETE_IMAGE="${z_buz_register_colophon}"
  buz_register "rbw-il" "rbf_cli.sh" "rbf_list"
  RBZ_LIST_IMAGES="${z_buz_register_colophon}"
  buz_register "rbw-ir" "rbf_cli.sh" "rbf_retrieve"
  RBZ_RETRIEVE_IMAGE="${z_buz_register_colophon}"

  # Nameplate regime operations
  buz_register "rbw-rnr" "rbrn_cli.sh" "render"
  RBZ_RENDER_NAMEPLATE="${z_buz_register_colophon}"
  buz_register "rbw-rnv" "rbrn_cli.sh" "validate"
  RBZ_VALIDATE_NAMEPLATE="${z_buz_register_colophon}"

  # Vessel regime operations
  buz_register "rbw-rvr" "rbrv_cli.sh" "render"
  RBZ_RENDER_VESSEL="${z_buz_register_colophon}"
  buz_register "rbw-rvv" "rbrv_cli.sh" "validate"
  RBZ_VALIDATE_VESSEL="${z_buz_register_colophon}"

  # Repo regime operations
  buz_register "rbw-rrr" "rbrr_cli.sh" "render"
  RBZ_RENDER_REPO="${z_buz_register_colophon}"
  buz_register "rbw-rrv" "rbrr_cli.sh" "validate"
  RBZ_VALIDATE_REPO="${z_buz_register_colophon}"
  buz_register "rbw-rrg" "rbrr_cli.sh" "refresh_gcb_pins"
  RBZ_REFRESH_GCB_PINS="${z_buz_register_colophon}"

  # Payor regime operations
  buz_register "rbw-rpr" "rbrp_cli.sh" "render"
  RBZ_RENDER_PAYOR="${z_buz_register_colophon}"
  buz_register "rbw-rpv" "rbrp_cli.sh" "validate"
  RBZ_VALIDATE_PAYOR="${z_buz_register_colophon}"

  # OAuth regime operations
  buz_register "rbw-ror" "rbro_cli.sh" "render"
  RBZ_RENDER_OAUTH="${z_buz_register_colophon}"
  buz_register "rbw-rov" "rbro_cli.sh" "validate"
  RBZ_VALIDATE_OAUTH="${z_buz_register_colophon}"

  # Station regime operations
  buz_register "rbw-rsr" "rbrs_cli.sh" "render"
  RBZ_RENDER_STATION="${z_buz_register_colophon}"
  buz_register "rbw-rsv" "rbrs_cli.sh" "validate"
  RBZ_VALIDATE_STATION="${z_buz_register_colophon}"

  # Auth regime operations
  buz_register "rbw-rar" "rbra_cli.sh" "render"
  RBZ_RENDER_AUTH="${z_buz_register_colophon}"
  buz_register "rbw-rav" "rbra_cli.sh" "validate"
  RBZ_VALIDATE_AUTH="${z_buz_register_colophon}"
  buz_register "rbw-ral" "rbra_cli.sh" "list"
  RBZ_LIST_AUTH="${z_buz_register_colophon}"

  # Cross-nameplate operations
  buz_register "rbw-ni" "rbrn_cli.sh" "survey"
  RBZ_SURVEY_NAMEPLATES="${z_buz_register_colophon}"
  buz_register "rbw-nv" "rbrn_cli.sh" "audit"
  RBZ_AUDIT_NAMEPLATES="${z_buz_register_colophon}"

  # Test operations (module is testbench, command is colophon)
  buz_register "rbw-ta"  "rbtb_testbench.sh" "rbw-ta"
  RBZ_TEST_ALL="${z_buz_register_colophon}"
  buz_register "rbw-ts"  "rbtb_testbench.sh" "rbw-ts"
  RBZ_TEST_SUITE="${z_buz_register_colophon}"
  buz_register "rbw-to"  "rbtb_testbench.sh" "rbw-to"
  RBZ_TEST_ONE="${z_buz_register_colophon}"
  buz_register "rbw-tn"  "rbtb_testbench.sh" "rbw-tn"
  RBZ_TEST_NAMEPLATE="${z_buz_register_colophon}"
  buz_register "rbw-trg" "rbtb_testbench.sh" "rbw-trg"
  RBZ_TEST_REGIME="${z_buz_register_colophon}"
  buz_register "rbw-trc" "rbtb_testbench.sh" "rbw-trc"
  RBZ_TEST_REGIME_CREDENTIALS="${z_buz_register_colophon}"

  ZRBZ_KINDLED=1
}

######################################################################
# Internal sentinel

zrbz_sentinel() {
  test "${ZRBZ_KINDLED:-}" = "1" || buc_die "Module rbz not kindled - call zrbz_kindle first"
}

# eof
