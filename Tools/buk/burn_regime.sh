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
# BURN Regime - Bash Utility Regime Node Module
#
# BURN is a multi-instance regime — one file per SSH connection profile.
# Each profile lives at .buk/users/${BURS_USER}/<alias>/burn.env

set -euo pipefail

# Multiple inclusion detection
test -z "${ZBURN_SOURCED:-}" || buc_die "Module burn multiply sourced - check sourcing hierarchy"
ZBURN_SOURCED=1

######################################################################
# Internal Functions (zburn_*)

zburn_kindle() {
  test -z "${ZBURN_KINDLED:-}" || buc_die "Module burn already kindled"

  # No defaults set — buv uses ${!varname:-} for safe indirect expansion under set -u.
  # Unset variables are detected distinctly from empty by zbuv_check_capture.

  # Enroll all BURN variables — single source of truth for validation and rendering

  buv_regime_enroll BURN

  buv_group_enroll "Connection Identity"
  buv_string_enroll  BURN_HOST        1  253  "IP address or hostname of the remote machine"
  buv_xname_enroll   BURN_USER        1   32  "Username on the remote host"
  buv_xname_enroll   BURN_ALIAS       1   64  "SSH alias and key filename (must match directory name)"

  buv_group_enroll "Authentication"
  buv_string_enroll  BURN_SSH_PUBKEY  20 1024  "Full public key line (e.g., ssh-ed25519 AAAA... user@host)"
  buv_string_enroll  BURN_KEY_FILE     0  256  "SSH private key filename in ~/.ssh/ (empty defaults to alias)"

  buv_group_enroll "Session Routing"
  buv_string_enroll  BURN_COMMAND      0  512  "Shell command for command= routing in authorized_keys (empty for direct shell)"

  buv_group_enroll "Privilege Tier"
  buv_enum_enroll    BURN_TIER  "Tier classification: privileged for node-level ops, workload for ephemeral remote work"  privileged  workload

  # Guard against unexpected BURN_ variables not in enrollment
  buv_scope_sentinel BURN BURN_

  # Lock all enrolled BURN_ variables against mutation
  buv_lock BURN

  readonly ZBURN_KINDLED=1
}

zburn_sentinel() {
  test "${ZBURN_KINDLED:-}" = "1" || buc_die "Module burn not kindled - call zburn_kindle first"
}

# Enforce all BURN enrollment validations
zburn_enforce() {
  zburn_sentinel

  buv_vet BURN
}

######################################################################
# Public Functions (burn_*)

# Build a BURN authorized_keys line with alias marker.
# Pure string builder — no I/O. Caller decides how to write.
# Sets ZBURN_KEY_LINE (the full line) and ZBURN_KEY_MARKER (the grep tag).
# Prerequisite: BURN kindled (needs BURN_ALIAS, BURN_COMMAND, BURN_SSH_PUBKEY)
zburn_build_key_line() {
  zburn_sentinel

  ZBURN_KEY_MARKER="# BURN:${BURN_ALIAS}"

  if test -n "${BURN_COMMAND}"; then
    ZBURN_KEY_LINE="command=\"${BURN_COMMAND}\" ${BURN_SSH_PUBKEY} ${ZBURN_KEY_MARKER}"
  else
    ZBURN_KEY_LINE="${BURN_SSH_PUBKEY} ${ZBURN_KEY_MARKER}"
  fi
}

# List available alias names as space-separated tokens.
# Emits one <alias>.${BUF_EXT_ALIAS} observation fact per profile (empty
# content; presence is the fact).
# Prerequisite: BURS kindled (needs BURS_USER), BURD kindled (needs BURD_CONFIG_DIR)
burn_list_capture() {
  zburs_sentinel

  local -r z_user_dir="${BURD_CONFIG_DIR}/users/${BURS_USER}"
  test -d "${z_user_dir}" || return 1

  local z_result=""
  local z_files=("${z_user_dir}"/*/burn.env)
  local z_i=""
  for z_i in "${!z_files[@]}"; do
    test -f "${z_files[$z_i]}" || continue
    local z_dir="${z_files[$z_i]%/*}"
    local z_alias="${z_dir##*/}"
    buf_write_fact_multi "${z_alias}" "${BUF_EXT_ALIAS}" ""
    z_result="${z_result}${z_result:+ }${z_alias}"
  done
  test -n "${z_result}" || return 1
  echo "${z_result}"
}

# Assert that the named alias's profile has the expected BURN_TIER value.
# Exits non-zero with diagnostic before any side effect on tier mismatch.
# Prerequisite: BURS kindled (BURS_USER), BURD kindled (BURD_CONFIG_DIR)
burn_assert_tier() {
  zburs_sentinel

  local -r z_expected="${1:-}"
  local -r z_alias="${2:-}"
  test -n "${z_expected}" || buc_die "burn_assert_tier: expected tier required (privileged|workload)"
  test -n "${z_alias}"    || buc_die "burn_assert_tier: alias required"

  local -r z_profile_file="${BURD_CONFIG_DIR}/users/${BURS_USER}/${z_alias}/burn.env"
  test -f "${z_profile_file}" || buc_die "burn_assert_tier: profile not found for alias '${z_alias}': ${z_profile_file}"

  local z_observed=""
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    case "${z_line}" in
      BURN_TIER=*) z_observed="${z_line#BURN_TIER=}" ; break ;;
    esac
  done < "${z_profile_file}"

  test -n "${z_observed}"                || buc_die "burn_assert_tier: BURN_TIER missing or empty in profile for alias '${z_alias}'"
  test "${z_observed}" = "${z_expected}" || buc_die "burn_assert_tier: alias '${z_alias}' has BURN_TIER='${z_observed}', expected '${z_expected}'"
}

# eof
