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
# BURP Regime - Bash Utility Regime Privileged Module
#
# BURP is a multi-instance regime — one file per privileged investiture.
# Each instance lives at .buk/rbmu_users/<BURS_USER>/<investiture>/burp.env
# and is per-station-user (operator-authored, not git-tracked).

set -euo pipefail

# Multiple inclusion detection
test -z "${ZBURP_SOURCED:-}" || buc_die "Module burp multiply sourced - check sourcing hierarchy"
ZBURP_SOURCED=1

######################################################################
# Internal Functions (zburp_*)

zburp_kindle() {
  test -z "${ZBURP_KINDLED:-}" || buc_die "Module burp already kindled"

  buv_regime_enroll BURP

  buv_group_enroll "Investiture Identity"
  buv_xname_enroll   BURP_NODE        1   64  "Viceroyalty referenced by this investiture (BURN directory name)"
  buv_string_enroll  BURP_USER        1   64  "Remote OS user authenticated by this investiture"

  buv_group_enroll "Authentication"
  buv_string_enroll  BURP_SSH_PUBKEY  20 1024  "Full public key line of the operator-managed admin keypair"
  buv_string_enroll  BURP_KEY_FILE     0  256  "SSH private key filename (empty defaults to investiture)"

  buv_group_enroll "Session Routing"
  buv_string_enroll  BURP_COMMAND      0  512  "Shell command for command= routing in administrators_authorized_keys (empty for direct shell)"

  # Guard against unexpected BURP_ variables not in enrollment
  buv_scope_sentinel BURP BURP_

  # Lock all enrolled BURP_ variables against mutation
  buv_lock BURP

  readonly ZBURP_KINDLED=1
}

zburp_sentinel() {
  test "${ZBURP_KINDLED:-}" = "1" || buc_die "Module burp not kindled - call zburp_kindle first"
}

# Enforce all BURP enrollment validations
zburp_enforce() {
  zburp_sentinel

  buv_vet BURP
}

######################################################################
# Public Functions (burp_*)

# List available investiture names as space-separated tokens for the current
# BURS_USER. Emits one <investiture>.${BUF_EXT_ALIAS} observation fact per
# profile (empty content; presence is the fact).
# Returns non-zero if no profiles are present.
# Prerequisite: BURS kindled (BURS_USER), BURD kindled (BURD_CONFIG_DIR)
burp_list_capture() {
  zburs_sentinel

  local -r z_user_dir="${BURD_CONFIG_DIR}/${BUBC_rbmu_users_subdir}/${BURS_USER}"
  test -d "${z_user_dir}" || return 1

  local z_result=""
  local z_files=("${z_user_dir}"/*/burp.env)
  local z_i=""
  for z_i in "${!z_files[@]}"; do
    test -f "${z_files[$z_i]}" || continue
    local z_dir="${z_files[$z_i]%/*}"
    local z_investiture="${z_dir##*/}"
    buf_write_fact_multi "${z_investiture}" "${BUF_EXT_ALIAS}" ""
    z_result="${z_result}${z_result:+ }${z_investiture}"
  done
  test -n "${z_result}" || return 1
  echo "${z_result}"
}

# Friendly-error die: emit "no folio supplied" message followed by available
# investitures (or a "no profiles" hint when the directory is empty).
# Prerequisite: BURS kindled (BURS_USER), BURD kindled (BURD_CONFIG_DIR)
burp_die_no_folio() {
  zburs_sentinel
  local z_aliases=""
  if z_aliases=$(burp_list_capture 2>/dev/null); then
    buc_warn "BURP investiture required as first argument."
    buc_step "Available investitures under .buk/${BUBC_rbmu_users_subdir}/${BURS_USER}/:"
    local z_v=""
    for z_v in ${z_aliases}; do
      buc_bare "        ${z_v}"
    done
  else
    buc_warn "BURP investiture required as first argument."
    buc_step "No profiles found under .buk/${BUBC_rbmu_users_subdir}/${BURS_USER}/."
    buc_bare "        Author one at .buk/${BUBC_rbmu_users_subdir}/${BURS_USER}/<investiture>/burp.env"
  fi
  buc_die "No BURP investiture supplied."
}

# eof
