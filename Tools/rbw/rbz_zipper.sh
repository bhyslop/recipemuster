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
# RBW Zipper - Colophon registry for RBW coordinator

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

  # z1z_buz_colophon set by buz_register (cross-module return channel)

  # Payor commands
  buz_register "rbw-PC" "rbgp_Payor" "rbgp_depot_create"
  RBZ_CREATE_DEPOT="${z1z_buz_colophon}"
  buz_register "rbw-PI" "rbgp_Payor" "rbgp_payor_install"
  RBZ_PAYOR_INSTALL="${z1z_buz_colophon}"
  buz_register "rbw-PD" "rbgp_Payor" "rbgp_depot_destroy"
  RBZ_DESTROY_DEPOT="${z1z_buz_colophon}"
  buz_register "rbw-PE" "rbgm_Manual" "rbgm_payor_establish"
  RBZ_PAYOR_ESTABLISH="${z1z_buz_colophon}"
  buz_register "rbw-PG" "rbgp_Payor" "rbgp_governor_reset"
  RBZ_GOVERNOR_RESET="${z1z_buz_colophon}"
  buz_register "rbw-PR" "rbgm_Manual" "rbgm_payor_refresh"
  RBZ_PAYOR_REFRESH="${z1z_buz_colophon}"
  buz_register "rbw-QB" "rbgm_Manual" "rbgm_quota_build"
  RBZ_QUOTA_BUILD="${z1z_buz_colophon}"

  # General depot operations
  buz_register "rbw-ld" "rbgp_Payor" "rbgp_depot_list"
  RBZ_LIST_DEPOT="${z1z_buz_colophon}"

  # Governor commands
  buz_register "rbw-GR" "rbgg_Governor" "rbgg_create_retriever"
  RBZ_CREATE_RETRIEVER="${z1z_buz_colophon}"
  buz_register "rbw-GD" "rbgg_Governor" "rbgg_create_director"
  RBZ_CREATE_DIRECTOR="${z1z_buz_colophon}"

  # Admin commands
  buz_register "rbw-ps" "rbgm_Manual" "rbgm_payor_establish"
  RBZ_ADMIN_ESTABLISH="${z1z_buz_colophon}"
  buz_register "rbw-Gl" "rbgg_Governor" "rbgg_list_service_accounts"
  RBZ_LIST_SERVICE_ACCOUNTS="${z1z_buz_colophon}"
  buz_register "rbw-GS" "rbgg_Governor" "rbgg_delete_service_account"
  RBZ_DELETE_SERVICE_ACCOUNT="${z1z_buz_colophon}"

  # Ark commands (paired -image + -about artifacts)
  buz_register "rbw-aA" "rbf_Foundry" "rbf_abjure"
  RBZ_ABJURE_ARK="${z1z_buz_colophon}"
  buz_register "rbw-ab" "rbf_Foundry" "rbf_beseech"
  RBZ_BESEECH_ARK="${z1z_buz_colophon}"
  buz_register "rbw-aC" "rbf_Foundry" "rbf_build"
  RBZ_CONJURE_ARK="${z1z_buz_colophon}"
  buz_register "rbw-as" "rbf_Foundry" "rbf_summon"
  RBZ_SUMMON_ARK="${z1z_buz_colophon}"

  # Image commands (single artifact)
  buz_register "rbw-iD" "rbf_Foundry" "rbf_delete"
  RBZ_DELETE_IMAGE="${z1z_buz_colophon}"
  buz_register "rbw-il" "rbf_Foundry" "rbf_list"
  RBZ_LIST_IMAGES="${z1z_buz_colophon}"
  buz_register "rbw-ir" "rbf_Foundry" "rbf_retrieve"
  RBZ_RETRIEVE_IMAGE="${z1z_buz_colophon}"

  ZRBZ_KINDLED=1
}

######################################################################
# Internal sentinel

zrbz_sentinel() {
  test "${ZRBZ_KINDLED:-}" = "1" || buc_die "Module rbz not kindled - call zrbz_kindle first"
}

# eof
