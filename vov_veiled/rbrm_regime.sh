#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# RBRM Regime - Podman VM Supply Chain Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRM_SOURCED:-}" || buc_die_now "Module rbrm multiply sourced - check sourcing hierarchy"
ZRBRM_SOURCED=1

######################################################################
# Internal Functions (zrbrm_*)

zrbrm_kindle() {
  test -z "${ZRBRM_KINDLED:-}" || buc_die_now "Module rbrm already kindled"

  buv_regime_enroll RBRM

  buv_group_enroll "Podman VM Machine Names"
  buv_xname_enroll   RBRM_IGNITE_MACHINE_NAME      1   64  "Podman machine for ignite operations"
  buv_xname_enroll   RBRM_DEPLOY_MACHINE_NAME      1   64  "Podman machine for deploy operations"

  buv_group_enroll "Podman VM Supply Chain"
  buv_string_enroll  RBRM_CRANE_TAR_GZ             1  512  "Crane binary archive path"
  buv_string_enroll  RBRM_MANIFEST_PLATFORMS        1  512  "Target platforms for manifests"
  buv_string_enroll  RBRM_CHOSEN_PODMAN_VERSION     1   16  "Podman version (semantic version)"
  buv_fqin_enroll    RBRM_CHOSEN_VMIMAGE_ORIGIN     1  256  "VM image origin reference"
  buv_string_enroll  RBRM_CHOSEN_IDENTITY           1  128  "Identity for operations"

  # Guard against unexpected RBRM_ variables not in enrollment
  buv_scope_sentinel RBRM RBRM_

  # Lock all enrolled RBRM_ variables against mutation
  buv_lock RBRM

  readonly ZRBRM_KINDLED=1
}

zrbrm_sentinel() {
  test "${ZRBRM_KINDLED:-}" = "1" || buc_die_now "Module rbrm not kindled - call zrbrm_kindle first"
}

# Enforce all RBRM enrollment validations and custom format checks
zrbrm_enforce() {
  zrbrm_sentinel

  buv_vet RBRM

  local z_platform=""
  for z_platform in ${RBRM_MANIFEST_PLATFORMS}; do
    [[ "${z_platform}" =~ ^[a-z0-9_]+$ ]] \
      || buc_die_now "Invalid platform format in RBRM_MANIFEST_PLATFORMS: ${z_platform}"
  done

  [[ "${RBRM_CHOSEN_PODMAN_VERSION}" =~ ^[0-9]+\.[0-9]+(\.[0-9]+)?$ ]] \
    || buc_die_now "Invalid RBRM_CHOSEN_PODMAN_VERSION format: ${RBRM_CHOSEN_PODMAN_VERSION} (expected N.N or N.N.N)"
}

# eof
