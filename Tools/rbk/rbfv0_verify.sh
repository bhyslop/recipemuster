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
# Recipe Bottle Foundry Verify - kindle entry: the single rbfv
# inclusion-guard and the whole kindle, sourcing the guard-free body clusters
# (rbfvg_ vouch gate, rbfva_ about, rbfvm_ graft metadata, rbfvv_ vouch,
# rbfvb_ batch vouch). The readonly ZRBFV_* constants the kindle sets are read
# globally by the clusters. Two entries source this file — rbfv0_cli, and
# rbfd0_director for the cross-module calls ordain makes into rbfv_vouch and
# zrbfv_graft_metadata_submit — so the guard stands here and nowhere else in
# the family, and no cluster needs one.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBFV_SOURCED:-}" || buc_die_now "Module rbfv multiply sourced - check sourcing hierarchy"
ZRBFV_SOURCED=1

# Source shared Foundry Core module
source "${BASH_SOURCE[0]%/*}/rbfc0_core.sh"

# Verify body clusters, sourced once here at the single rbfv entry — all
# guard-free: no cluster is sourced by a second entry.
source "${BASH_SOURCE[0]%/*}/rbfvg_gate.sh"
source "${BASH_SOURCE[0]%/*}/rbfva_about.sh"
source "${BASH_SOURCE[0]%/*}/rbfvm_metadata.sh"
source "${BASH_SOURCE[0]%/*}/rbfvv_vouch.sh"
source "${BASH_SOURCE[0]%/*}/rbfvb_batch.sh"

######################################################################
# Internal Functions (zrbfv_*)

zrbfv_kindle() {
  test -z "${ZRBFV_KINDLED:-}" || buc_die_now "Module rbfv already kindled"

  buc_log_args 'Validate Foundry Core is kindled'
  zrbfc_sentinel

  buc_log_args 'Define vouch operation file prefix'
  readonly ZRBFV_VOUCH_PREFIX="${BURD_TEMP_DIR}/rbfv_vouch_"

  buc_log_args 'Define about operation file prefix'
  readonly ZRBFV_ABOUT_PREFIX="${BURD_TEMP_DIR}/rbfv_about_"

  buc_log_args 'Define graft metadata operation file prefix'
  readonly ZRBFV_GRAFT_META_PREFIX="${BURD_TEMP_DIR}/rbfv_graft_meta_"

  readonly ZRBFV_KINDLED=1
}

zrbfv_sentinel() {
  zrbfc_sentinel
  test "${ZRBFV_KINDLED:-}" = "1" || buc_die_now "Module rbfv not kindled - call zrbfv_kindle first"
}

# eof
