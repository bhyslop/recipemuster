#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
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
# Recipe Bottle Regime Vessel - Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRV_SOURCED:-}" || buc_die "Module rbrv multiply sourced - check sourcing hierarchy"
ZRBRV_SOURCED=1

######################################################################
# Internal Functions (zrbrv_*)

zrbrv_kindle() {
  test -z "${ZRBRV_KINDLED:-}" || buc_die "Module rbrv already kindled"

  # No defaults set — buv uses ${!varname:-} for safe indirect expansion under set -u.
  # Unset variables are detected distinctly from empty by zbuv_check_capture.

  # Enroll all RBRV variables — single source of truth for validation and rendering

  buv_regime_enroll RBRV

  buv_group_enroll "Core Vessel Identity"
  buv_xname_enroll  RBRV_SIGIL             1   64  "Unique identifier (must match directory name)"
  buv_string_enroll RBRV_DESCRIPTION       0  512  "Human-readable description"
  buv_enum_enroll   RBRV_VESSEL_MODE               "Operation mode: bind or conjure" \
                    bind conjure

  buv_group_enroll "Binding Configuration"
  buv_gate_enroll   RBRV_VESSEL_MODE  bind
  buv_fqin_enroll   RBRV_BIND_IMAGE  1  512  "Source image to copy from registry"

  buv_group_enroll "Conjuring Configuration"
  buv_gate_enroll   RBRV_VESSEL_MODE  conjure
  buv_string_enroll RBRV_CONJURE_DOCKERFILE    1  512  "Dockerfile path relative to repo root"
  buv_string_enroll RBRV_CONJURE_BLDCONTEXT    1  512  "Build context relative to repo root"
  buv_string_enroll RBRV_CONJURE_PLATFORMS     1  512  "Space-separated target platforms"

  # Guard against unexpected RBRV_ variables not in enrollment
  buv_scope_sentinel RBRV RBRV_

  # Lock all enrolled RBRV_ variables against mutation
  buv_lock RBRV

  readonly ZRBRV_KINDLED=1
}

zrbrv_sentinel() {
  test "${ZRBRV_KINDLED:-}" = "1" || buc_die "Module rbrv not kindled - call zrbrv_kindle first"
}

# Enforce all RBRV enrollment validations
zrbrv_enforce() {
  zrbrv_sentinel

  buv_vet RBRV
}

######################################################################
# Public Functions (rbrv_*)

# List available vessel sigils as space-separated tokens
# Prerequisite: RBRR kindled (needs RBRR_VESSEL_DIR)
rbrv_list_capture() {
  zrbrr_sentinel

  local z_result=""
  local z_dirs=("${RBRR_VESSEL_DIR}"/*)
  local z_i=""
  for z_i in "${!z_dirs[@]}"; do
    local z_d="${z_dirs[$z_i]}"
    test -d "${z_d}" || continue
    test -f "${z_d}/rbrv.env" || continue
    local z_s="${z_d%/}"
    z_result="${z_result}${z_result:+ }${z_s##*/}"
  done
  test -n "${z_result}" || return 1
  echo "${z_result}"
}

# eof
