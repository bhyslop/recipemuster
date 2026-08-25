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
# Recipe Bottle Foundry Director Build - kindle entry: the single rbfd
# inclusion-guard, tinder and kindle, sourcing the guard-free body clusters
# (rbfdp_ preflight, rbfdb_ build, rbfdm_ mirror, rbfdg_ graft, rbfdo_
# ordain). The readonly ZRBFD_* constants the kindle sets are read globally
# by the clusters.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBFD_SOURCED:-}" || buc_die_now "Module rbfd multiply sourced - check sourcing hierarchy"
ZRBFD_SOURCED=1

# Source shared Foundry Core module
source "${BASH_SOURCE[0]%/*}/rbfc0_core.sh"

# Source Foundry Verify module (ordain cross-module calls: rbfv_vouch, zrbfv_graft_metadata_submit)
source "${BASH_SOURCE[0]%/*}/rbfv_verify.sh"

# Tinder constants
# Step id of the hallmark-echoing conjure step — single mint shared by the
# step defs and the consistency assert, which locates its output slot by id
RBFD_hallmark_echo_step_id="derive-tag-base"

# Director body clusters, sourced once here at the single rbfd entry — all
# guard-free: no cluster is sourced by a second entry.
source "${BASH_SOURCE[0]%/*}/rbfdp_preflight.sh"
source "${BASH_SOURCE[0]%/*}/rbfdb_build.sh"
source "${BASH_SOURCE[0]%/*}/rbfdm_mirror.sh"
source "${BASH_SOURCE[0]%/*}/rbfdg_graft.sh"
source "${BASH_SOURCE[0]%/*}/rbfdo_ordain.sh"

######################################################################
# Internal Functions (zrbfd_*)

zrbfd_kindle() {
  test -z "${ZRBFD_KINDLED:-}" || buc_die_now "Module rbfd already kindled"

  buc_log_args 'Kindle shared Foundry Core infrastructure'
  zrbfc_kindle

  buc_log_args 'RBGJ files in same Tools directory as this implementation'
  # Acronym: rbgjb = Recipe Bottle Google Json Build (step scripts in rbgjb/ dir)
  local z_self_dir="${BASH_SOURCE[0]%/*}"
  readonly ZRBFD_RBGJB_STEPS_DIR="${z_self_dir}/rbgjb"
  test -d "${ZRBFD_RBGJB_STEPS_DIR}"   || buc_die_now "RBGJB steps directory not found: ${ZRBFD_RBGJB_STEPS_DIR}"

  # RBGJV and RBGJA step dirs now owned by rbfc0_core.sh (shared assembly helpers)

  buc_log_args 'RBGJM mirror step scripts (same Tools directory)'
  # Acronym: rbgjm = Recipe Bottle Google Json Mirror (step scripts in rbgjm/ dir)
  readonly ZRBFD_RBGJM_STEPS_DIR="${z_self_dir}/rbgjm"
  test -d "${ZRBFD_RBGJM_STEPS_DIR}"   || buc_die_now "RBGJM steps directory not found: ${ZRBFD_RBGJM_STEPS_DIR}"

  buc_log_args 'Define stitch operation file prefix (postfixed per step id)'
  readonly ZRBFD_STITCH_PREFIX="${BURD_TEMP_DIR}/rbfd_stitch_"

  buc_log_args 'Define mirror operation files'
  readonly ZRBFD_MIRROR_PREFIX="${BURD_TEMP_DIR}/rbfd_mirror_"

  buc_log_args 'Define graft operation files'
  readonly ZRBFD_GRAFT_PREFIX="${BURD_TEMP_DIR}/rbfd_graft_"

  buc_log_args 'Define base-image registry preflight files'
  readonly ZRBFD_PREFLIGHT_PREFIX="${BURD_TEMP_DIR}/rbfd_preflight_"

  buc_log_args 'Define context push operation files'
  readonly ZRBFD_CONTEXT_PREFIX="${BURD_TEMP_DIR}/rbfd_context_"

  buc_log_args 'Kindle verify module (cross-module calls from ordain)'
  zrbfv_kindle

  readonly ZRBFD_KINDLED=1
}

zrbfd_sentinel() {
  zrbfc_sentinel
  test "${ZRBFD_KINDLED:-}" = "1" || buc_die_now "Module rbfd not kindled - call zrbfd_kindle first"
}

# eof
