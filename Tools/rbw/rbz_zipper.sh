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

# shellcheck disable=SC2034
zrbz_kindle() {
  test -z "${ZRBZ_KINDLED:-}" || buc_die "rbz already kindled"

  # Verify buz zipper is kindled (CLI furnish must kindle buz first)
  zbuz_sentinel

  # Payor commands
  RBZ_CREATE_DEPOT=$(buz_create_capture "rbw-PC" "rbgp_Payor" "rbgp_depot_create")
  RBZ_PAYOR_INSTALL=$(buz_create_capture "rbw-PI" "rbgp_Payor" "rbgp_payor_install")
  RBZ_DESTROY_DEPOT=$(buz_create_capture "rbw-PD" "rbgp_Payor" "rbgp_depot_destroy")
  RBZ_PAYOR_ESTABLISH=$(buz_create_capture "rbw-PE" "rbgm_Manual" "rbgm_payor_establish")
  RBZ_GOVERNOR_RESET=$(buz_create_capture "rbw-PG" "rbgp_Payor" "rbgp_governor_reset")
  RBZ_PAYOR_REFRESH=$(buz_create_capture "rbw-PR" "rbgm_Manual" "rbgm_payor_refresh")

  # General depot operations
  RBZ_LIST_DEPOT=$(buz_create_capture "rbw-ld" "rbgp_Payor" "rbgp_depot_list")

  # Governor commands
  RBZ_CREATE_RETRIEVER=$(buz_create_capture "rbw-GR" "rbgg_Governor" "rbgg_create_retriever")
  RBZ_CREATE_DIRECTOR=$(buz_create_capture "rbw-GD" "rbgg_Governor" "rbgg_create_director")

  # Admin commands
  RBZ_ADMIN_ESTABLISH=$(buz_create_capture "rbw-ps" "rbgm_Manual" "rbgm_payor_establish")
  RBZ_LIST_SERVICE_ACCOUNTS=$(buz_create_capture "rbw-Gl" "rbgg_Governor" "rbgg_list_service_accounts")
  RBZ_DELETE_SERVICE_ACCOUNT=$(buz_create_capture "rbw-GS" "rbgg_Governor" "rbgg_delete_service_account")

  # Ark commands (paired -image + -about artifacts)
  RBZ_ABJURE_ARK=$(buz_create_capture "rbw-aA" "rbf_Foundry" "rbf_abjure")
  RBZ_BESEECH_ARK=$(buz_create_capture "rbw-ab" "rbf_Foundry" "rbf_beseech")
  RBZ_CONJURE_ARK=$(buz_create_capture "rbw-aC" "rbf_Foundry" "rbf_build")
  RBZ_SUMMON_ARK=$(buz_create_capture "rbw-as" "rbf_Foundry" "rbf_summon")

  # Image commands (single artifact)
  RBZ_DELETE_IMAGE=$(buz_create_capture "rbw-iD" "rbf_Foundry" "rbf_delete")
  RBZ_LIST_IMAGES=$(buz_create_capture "rbw-il" "rbf_Foundry" "rbf_list")
  RBZ_RETRIEVE_IMAGE=$(buz_create_capture "rbw-ir" "rbf_Foundry" "rbf_retrieve")

  ZRBZ_KINDLED=1
}

######################################################################
# Internal sentinel

zrbz_sentinel() {
  test "${ZRBZ_KINDLED:-}" = "1" || buc_die "Module rbz not kindled - call zrbz_kindle first"
}

# eof
