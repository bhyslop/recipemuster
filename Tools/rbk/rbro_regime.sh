#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
# Recipe Bottle OAuth Regime - Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRO_SOURCED:-}" || buc_die_now "Module rbro multiply sourced - check sourcing hierarchy"
ZRBRO_SOURCED=1

######################################################################
# Internal Functions (zrbro_*)

zrbro_kindle() {
  test -z "${ZRBRO_KINDLED:-}" || buc_die_now "Module rbro already kindled"

  # The credential store is sourced here, at kindle, ahead of its enrollment.
  # RBDC_PAYOR_RBRO_FILE is rbdc's kindle constant, derived from RBRR_SECRETS_DIR.
  # Presence and readability first: a failed source on bash 3.2 exits before its guard.
  test -f "${RBDC_PAYOR_RBRO_FILE}" || buc_die_now "RBRO credentials missing (${RBDC_PAYOR_RBRO_FILE}) - run rbgp_install"
  test -r "${RBDC_PAYOR_RBRO_FILE}" || buc_die_now "RBRO file not readable - check permissions"
  source "${RBDC_PAYOR_RBRO_FILE}" || buc_die_now "Failed to source RBRO credentials"

  # No defaults set — buv uses ${!varname:-} for safe indirect expansion under set -u.
  # Unset variables are detected distinctly from empty by zbuv_check_capture.

  # Enroll all RBRO variables — single source of truth for validation and rendering
  buv_regime_enroll RBRO

  buv_group_enroll "OAuth Credentials"
  buv_string_enroll  RBRO_CLIENT_SECRET  1  512  "OAuth client secret"
  buv_string_enroll  RBRO_REFRESH_TOKEN  1  512  "OAuth refresh token"

  # Guard against unexpected RBRO_ variables not in enrollment
  buv_scope_sentinel RBRO RBRO_

  # Lock all enrolled RBRO_ variables against mutation
  buv_lock RBRO

  readonly ZRBRO_KINDLED=1
}

zrbro_sentinel() {
  test "${ZRBRO_KINDLED:-}" = "1" || buc_die_now "Module rbro not kindled - call zrbro_kindle first"
}

# Enforce all RBRO enrollment validations
zrbro_enforce() {
  zrbro_sentinel
  buv_vet RBRO
}

# eof
