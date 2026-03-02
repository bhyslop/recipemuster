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
# Recipe Bottle Authentication Regime - Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRA_SOURCED:-}" || buc_die "Module rbra multiply sourced - check sourcing hierarchy"
ZRBRA_SOURCED=1

######################################################################
# Internal Functions (zrbra_*)

zrbra_kindle() {
  test -z "${ZRBRA_KINDLED:-}" || buc_die "Module rbra already kindled"

  # No defaults set — buv uses ${!varname:-} for safe indirect expansion under set -u.
  # Unset variables are detected distinctly from empty by zbuv_check_capture.

  # Enroll all RBRA variables — single source of truth for validation and rendering

  buv_regime_enroll RBRA

  buv_group_enroll "Service Account Credentials"
  buv_string_enroll   RBRA_CLIENT_EMAIL        1   256  "Service account email address"
  buv_string_enroll   RBRA_PRIVATE_KEY         1  4096  "PEM-encoded private key material"
  buv_string_enroll   RBRA_PROJECT_ID          1    64  "GCP project owning the service account"
  buv_decimal_enroll  RBRA_TOKEN_LIFETIME_SEC  300  3600  "OAuth token lifetime in seconds"

  # Guard against unexpected RBRA_ variables not in enrollment
  buv_scope_sentinel RBRA RBRA_

  readonly ZRBRA_KINDLED=1
}

zrbra_sentinel() {
  test "${ZRBRA_KINDLED:-}" = "1" || buc_die "Module rbra not kindled - call zrbra_kindle first"
}

# Enforce all RBRA enrollment validations plus custom format checks
zrbra_enforce() {
  zrbra_sentinel

  buv_vet RBRA

  # Client email must match service account pattern
  [[ "${RBRA_CLIENT_EMAIL}" =~ \.iam\.gserviceaccount\.com$ ]] \
    || buc_die "RBRA_CLIENT_EMAIL does not match service account pattern (*.iam.gserviceaccount.com): '${RBRA_CLIENT_EMAIL}'"

  # Private key must contain PEM key material
  [[ "${RBRA_PRIVATE_KEY}" =~ BEGIN ]] \
    || buc_die "RBRA_PRIVATE_KEY does not contain PEM key material"
}

# Lock step — lock enrolled variables against mutation after enforcement
zrbra_lock() {
  zrbra_sentinel

  # Lock all enrolled RBRA_ variables against mutation
  buv_lock RBRA
}

# eof
