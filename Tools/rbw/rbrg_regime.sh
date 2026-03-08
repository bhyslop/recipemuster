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
# RBRG Regime - GCB Image Pins Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRG_SOURCED:-}" || buc_die "Module rbrg multiply sourced - check sourcing hierarchy"
ZRBRG_SOURCED=1

######################################################################
# Internal Functions (zrbrg_*)

zrbrg_kindle() {
  test -z "${ZRBRG_KINDLED:-}" || buc_die "Module rbrg already kindled"

  buv_regime_enroll RBRG

  buv_group_enroll "GCB Image Pins"
  buv_odref_enroll   RBRG_ORAS_IMAGE_REF                 "oras image reference (digest-pinned)"
  buv_odref_enroll   RBRG_GCLOUD_IMAGE_REF               "gcloud image reference (digest-pinned)"
  buv_odref_enroll   RBRG_DOCKER_IMAGE_REF               "docker image reference (digest-pinned)"
  buv_odref_enroll   RBRG_ALPINE_IMAGE_REF               "alpine image reference (digest-pinned)"
  buv_odref_enroll   RBRG_SYFT_IMAGE_REF                 "syft image reference (digest-pinned)"
  buv_odref_enroll   RBRG_BINFMT_IMAGE_REF               "binfmt image reference (digest-pinned)"
  buv_odref_enroll   RBRG_SKOPEO_IMAGE_REF               "skopeo image reference (digest-pinned)"

  buv_group_enroll "GCB Binary Pins"
  buv_string_enroll  RBRG_SLSA_VERIFIER_URL       1  512  "slsa-verifier binary download URL"
  buv_string_enroll  RBRG_SLSA_VERIFIER_SHA256   64   64  "slsa-verifier binary SHA256 checksum"

  buv_decimal_enroll RBRG_PINS_REFRESHED_AT  0  9999999999  "Epoch seconds of last successful pin refresh (0=never)"

  # Guard against unexpected RBRG_ variables not in enrollment
  buv_scope_sentinel RBRG RBRG_

  # Lock all enrolled RBRG_ variables against mutation
  buv_lock RBRG

  readonly ZRBRG_KINDLED=1
}

zrbrg_sentinel() {
  test "${ZRBRG_KINDLED:-}" = "1" || buc_die "Module rbrg not kindled - call zrbrg_kindle first"
}

# Enforce all RBRG enrollment validations
zrbrg_enforce() {
  zrbrg_sentinel
  buv_vet RBRG
}

# eof
