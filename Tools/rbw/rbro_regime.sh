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
# Recipe Bottle OAuth Regime - Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRO_SOURCED:-}" || buc_die "Module rbro multiply sourced - check sourcing hierarchy"
ZRBRO_SOURCED=1

######################################################################
# Internal Functions (zrbro_*)

zrbro_kindle() {
  test -z "${ZRBRO_KINDLED:-}" || buc_die "Module rbro already kindled"

  # Set defaults for all fields (enrollment enforces required-ness)
  RBRO_CLIENT_SECRET="${RBRO_CLIENT_SECRET:-}"
  RBRO_REFRESH_TOKEN="${RBRO_REFRESH_TOKEN:-}"

  # Enroll all RBRO variables — single source of truth for validation and rendering
  buv_regime_enroll RBRO

  buv_group_enroll "OAuth Credentials"
  buv_string_enroll  RBRO_CLIENT_SECRET  1  512  "OAuth client secret"
  buv_string_enroll  RBRO_REFRESH_TOKEN  1  512  "OAuth refresh token"

  # Guard against unexpected RBRO_ variables not in enrollment
  buv_scope_sentinel RBRO RBRO_

  ZRBRO_KINDLED=1
}

zrbro_sentinel() {
  test "${ZRBRO_KINDLED:-}" = "1" || buc_die "Module rbro not kindled - call zrbro_kindle first"
}

# Enforce all RBRO enrollment validations
zrbro_enforce() {
  zrbro_sentinel
  buv_vet RBRO
}

######################################################################
# Public Functions (rbro_*)

# Load RBRO from the canonical location (~/.rbw/rbro.env)
rbro_load() {
  local z_rbro_dir="${HOME}/.rbw"
  local z_rbro_file="${z_rbro_dir}/rbro.env"

  # Check directory and file exist
  test -d "${z_rbro_dir}" || buc_die "RBRO directory missing (~/.rbw) - run rbgp_payor_install"
  test -f "${z_rbro_file}" || buc_die "RBRO credentials missing (~/.rbw/rbro.env) - run rbgp_payor_install"
  test -r "${z_rbro_file}" || buc_die "RBRO file not readable - check permissions"

  # Source and validate
  source "${z_rbro_file}" || buc_die "Failed to source RBRO credentials"
  zrbro_kindle
  zrbro_enforce
}

# eof
