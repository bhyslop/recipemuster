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
# Recipe Bottle Lode - kindle entry: the single rbld inclusion-guard and kindle,
# sourcing the guard-free body clusters (rbldl_ lifecycle, rbldb_ bole). The
# readonly ZRBLD_* constants the kindle sets are read globally by the clusters.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBLD_SOURCED:-}" || buc_die "Module rbld multiply sourced - check sourcing hierarchy"
ZRBLD_SOURCED=1

# Source shared Foundry Core module
source "${BASH_SOURCE[0]%/*}/rbfc_FoundryCore.sh"

# Lode body clusters (guard-free; sourced once here, the single rbld entry)
source "${BASH_SOURCE[0]%/*}/rbldl_Lifecycle.sh"
source "${BASH_SOURCE[0]%/*}/rbldb_Bole.sh"

######################################################################
# Internal Functions (zrbld_*)

zrbld_kindle() {
  test -z "${ZRBLD_KINDLED:-}" || buc_die "Module rbld already kindled"

  buc_log_args 'Validate Foundry Core is kindled'
  zrbfc_sentinel

  buc_log_args 'Verify Director RBRA file'
  test -n "${RBDC_DIRECTOR_RBRA_FILE:-}" || buc_die "RBDC_DIRECTOR_RBRA_FILE not set"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}"   || buc_die "GCB service env file not found: ${RBDC_DIRECTOR_RBRA_FILE}"

  buc_log_args 'RBGJL ensconce step scripts (same Tools directory)'
  local z_self_dir="${BASH_SOURCE[0]%/*}"
  readonly ZRBLD_RBGJL_STEPS_DIR="${z_self_dir}/rbgjl"
  test -d "${ZRBLD_RBGJL_STEPS_DIR}" || buc_die "RBGJL steps directory not found: ${ZRBLD_RBGJL_STEPS_DIR}"

  buc_log_args 'Define ensconce operation file prefix'
  readonly ZRBLD_ENSCONCE_PREFIX="${BURD_TEMP_DIR}/rbld_ensconce_"

  buc_log_args 'Define divine operation file prefix'
  readonly ZRBLD_DIVINE_PREFIX="${BURD_TEMP_DIR}/rbld_divine_"

  buc_log_args 'Define banish operation file prefix'
  readonly ZRBLD_BANISH_PREFIX="${BURD_TEMP_DIR}/rbld_banish_"

  readonly ZRBLD_KINDLED=1
}

zrbld_sentinel() {
  zrbfc_sentinel
  test "${ZRBLD_KINDLED:-}" = "1" || buc_die "Module rbld not kindled - call zrbld_kindle first"
}

# eof
