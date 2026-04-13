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
# BURH Regime - Bash Utility Regime Host Module
#
# BURH is a multi-instance regime — one file per SSH connection profile.
# Each profile lives at .buk/users/${BURS_USER}/<alias>/burh.env

set -euo pipefail

# Multiple inclusion detection
test -z "${ZBURH_SOURCED:-}" || buc_die "Module burh multiply sourced - check sourcing hierarchy"
ZBURH_SOURCED=1

######################################################################
# Internal Functions (zburh_*)

zburh_kindle() {
  test -z "${ZBURH_KINDLED:-}" || buc_die "Module burh already kindled"

  # No defaults set — buv uses ${!varname:-} for safe indirect expansion under set -u.
  # Unset variables are detected distinctly from empty by zbuv_check_capture.

  # Enroll all BURH variables — single source of truth for validation and rendering

  buv_regime_enroll BURH

  buv_group_enroll "Connection Identity"
  buv_string_enroll  BURH_HOST        1  253  "IP address or hostname of the remote machine"
  buv_xname_enroll   BURH_USER        1   32  "Username on the remote host"
  buv_xname_enroll   BURH_ALIAS       1   64  "SSH alias and key filename (must match directory name)"

  buv_group_enroll "Authentication"
  buv_string_enroll  BURH_SSH_PUBKEY  20 1024  "Full public key line (e.g., ssh-ed25519 AAAA... user@host)"
  buv_string_enroll  BURH_KEY_FILE     0  256  "SSH private key filename in ~/.ssh/ (empty defaults to alias)"

  buv_group_enroll "Session Routing"
  buv_string_enroll  BURH_COMMAND      0  512  "Shell command for command= routing in authorized_keys (empty for direct shell)"

  # Guard against unexpected BURH_ variables not in enrollment
  buv_scope_sentinel BURH BURH_

  # Lock all enrolled BURH_ variables against mutation
  buv_lock BURH

  readonly ZBURH_KINDLED=1
}

zburh_sentinel() {
  test "${ZBURH_KINDLED:-}" = "1" || buc_die "Module burh not kindled - call zburh_kindle first"
}

# Enforce all BURH enrollment validations
zburh_enforce() {
  zburh_sentinel

  buv_vet BURH
}

######################################################################
# Public Functions (burh_*)

# List available alias names as space-separated tokens
# Prerequisite: BURS kindled (needs BURS_USER), BURD kindled (needs BURD_CONFIG_DIR)
burh_list_capture() {
  zburs_sentinel

  local -r z_user_dir="${BURD_CONFIG_DIR}/users/${BURS_USER}"
  test -d "${z_user_dir}" || return 1

  local z_result=""
  local z_files=("${z_user_dir}"/*/burh.env)
  local z_i=""
  for z_i in "${!z_files[@]}"; do
    test -f "${z_files[$z_i]}" || continue
    local z_dir="${z_files[$z_i]%/*}"
    z_result="${z_result}${z_result:+ }${z_dir##*/}"
  done
  test -n "${z_result}" || return 1
  echo "${z_result}"
}

# eof
