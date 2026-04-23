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
# Recipe Bottle GAR Layout - Implementation
#
# Single source of truth for GAR categorical-namespace path construction.
# All producer and consumer sites route through these helpers; no direct concat.
# Output is the prefix-rooted relative path (from GAR repo root); callers
# prepend "${host}/${path}/" for full URL.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGL_SOURCED:-}" || buc_die "Module rbgl multiply sourced - check sourcing hierarchy"
ZRBGL_SOURCED=1

######################################################################
# Internal Functions (zrbgl_*)

zrbgl_kindle() {
  test -z "${ZRBGL_KINDLED:-}" || buc_die "Module rbgl already kindled"

  # Category constants come from rbgc.
  zrbgc_sentinel

  readonly ZRBGL_KINDLED=1
}

zrbgl_sentinel() {
  test "${ZRBGL_KINDLED:-}" = "1" || buc_die "Module rbgl not kindled - call zrbgl_kindle first"
}

######################################################################
# Path Construction Functions
#
# Each function emits a prefix-rooted relative path to stdout.
# Requires RBRR_CLOUD_PREFIX to be set (regime contract; enforced by
# rbrr_validate ahead of any consumer).

rbgl_hallmark_ark_path() {
  zrbgl_sentinel
  local -r z_hallmark="${1:?rbgl_hallmark_ark_path: hallmark required}"
  local -r z_basename="${2:?rbgl_hallmark_ark_path: ark basename required}"
  printf '%s%s/%s/%s\n' \
    "${RBRR_CLOUD_PREFIX}" "${RBGC_GAR_CATEGORY_HALLMARKS}" "${z_hallmark}" "${z_basename}"
}

rbgl_hallmark_subtree() {
  zrbgl_sentinel
  local -r z_hallmark="${1:?rbgl_hallmark_subtree: hallmark required}"
  printf '%s%s/%s\n' \
    "${RBRR_CLOUD_PREFIX}" "${RBGC_GAR_CATEGORY_HALLMARKS}" "${z_hallmark}"
}

rbgl_reliquary_path() {
  zrbgl_sentinel
  local -r z_date="${1:?rbgl_reliquary_path: reliquary date required}"
  local -r z_tool="${2:?rbgl_reliquary_path: tool name required}"
  printf '%s%s/%s/%s\n' \
    "${RBRR_CLOUD_PREFIX}" "${RBGC_GAR_CATEGORY_RELIQUARIES}" "${z_date}" "${z_tool}"
}

rbgl_reliquary_subtree() {
  zrbgl_sentinel
  local -r z_date="${1:?rbgl_reliquary_subtree: reliquary date required}"
  printf '%s%s/%s\n' \
    "${RBRR_CLOUD_PREFIX}" "${RBGC_GAR_CATEGORY_RELIQUARIES}" "${z_date}"
}

rbgl_enshrine_path() {
  zrbgl_sentinel
  local -r z_anchor="${1:?rbgl_enshrine_path: anchor required}"
  printf '%s%s/%s\n' \
    "${RBRR_CLOUD_PREFIX}" "${RBGC_GAR_CATEGORY_ENSHRINES}" "${z_anchor}"
}

# eof
