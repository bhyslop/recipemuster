#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# Recipe Bottle Regime Station - Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRS_SOURCED:-}" || buc_die_now "Module rbrs multiply sourced - check sourcing hierarchy"
ZRBRS_SOURCED=1

######################################################################
# Internal Functions (zrbrs_*)

zrbrs_kindle() {
  test -z "${ZRBRS_KINDLED:-}" || buc_die_now "Module rbrs already kindled"

  # No defaults set — buv uses ${!varname:-} for safe indirect expansion under set -u.
  # Unset variables are detected distinctly from empty by zbuv_check_capture.

  # Enroll all RBRS variables — single source of truth for validation and rendering

  buv_regime_enroll RBRS

  buv_group_enroll "Station Paths"
  buv_string_enroll  RBRS_PODMAN_ROOT_DIR     1  64  "Podman machine root directory"
  buv_string_enroll  RBRS_VMIMAGE_CACHE_DIR   1  64  "VM image cache directory"
  buv_string_enroll  RBRS_VM_PLATFORM         1  64  "VM platform architecture"

  # Guard against unexpected RBRS_ variables not in enrollment
  buv_scope_sentinel RBRS RBRS_

  # Lock all enrolled RBRS_ variables against mutation
  buv_lock RBRS

  readonly ZRBRS_KINDLED=1
}

zrbrs_sentinel() {
  test "${ZRBRS_KINDLED:-}" = "1" || buc_die_now "Module rbrs not kindled - call zrbrs_kindle first"
}

# Enforce all RBRS enrollment validations
zrbrs_enforce() {
  zrbrs_sentinel

  buv_vet RBRS
}

# eof
