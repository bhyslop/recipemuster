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
# BURN CLI - Command line interface for BURN node regime operations
#
# Four command families:
#   burn_validate/render/list  — regime read operations (require BUZ_FOLIO)
#   burn_construct_*           — profile constructors (write burn.env)
#   burn_verify_*              — SSH automation (verify)
#   burn_install_key           — authorized_keys management (per-profile, idempotent)

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Command Functions — Regime Operations

burn_validate() {
  buc_doc_brief "Validate BURN node regime profile via enrollment report"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || buc_die "Profile alias required (e.g., winhost-cyg)"
  buc_step "Validating BURN node regime"
  buv_report BURN "Node Regime"
  buc_step "BURN node regime valid"
}

burn_render() {
  buc_doc_brief "Display diagnostic view of BURN node regime profile"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || buc_die "Profile alias required (e.g., winhost-cyg)"
  buv_render BURN "BURN - Bash Utility Node Regime"
}

burn_list() {
  buc_doc_brief "List available BURN host profiles for current user"
  buc_doc_shown || return 0

  local z_aliases
  z_aliases=$(burn_list_capture) || buc_die "No BURN profiles found for user: ${BURS_USER}"
  buc_step "Available profiles for ${BURS_USER}:"
  local z_alias=""
  for z_alias in ${z_aliases}; do
    buc_bare "        ${z_alias}"
  done
}

######################################################################
# Internal: Constructor Helper
#
# Shared logic for all platform constructors.
# Args: host user moniker suffix command_value key_file
# command_value and key_file may be empty.

zburn_construct() {
  local z_host="${1:-}"
  local z_user="${2:-}"
  local z_moniker="${3:-}"
  local z_suffix="${4:-}"
  local z_command="${5:-}"
  local z_key_file="${6:-}"

  test -n "${z_host}"    || buc_die "host required (arg 1)"
  test -n "${z_user}"    || buc_die "user required (arg 2)"
  test -n "${z_moniker}" || buc_die "moniker required (arg 3)"

  local -r z_alias="${z_moniker}-${z_suffix}"
  local -r z_profile_dir="${BURD_CONFIG_DIR}/users/${BURS_USER}/${z_alias}"
  local -r z_profile_file="${z_profile_dir}/burn.env"
  local -r z_pubkey_file="${HOME}/.ssh/${z_alias}.pub"
  local z_pubkey=""
  local z_has_pubkey=0

  if test -f "${z_pubkey_file}"; then
    z_pubkey=$(head -1 "${z_pubkey_file}")
    z_has_pubkey=1
    buc_step "Found public key: ${z_pubkey_file}"
  else
    z_pubkey="ssh-ed25519 REPLACE_WITH_ACTUAL_PUBKEY ${BURS_USER}@$(hostname -s)"
  fi

  mkdir -p "${z_profile_dir}"

  printf '%s\n' "BURN_HOST=${z_host}"           >  "${z_profile_file}"
  printf '%s\n' "BURN_USER=${z_user}"            >> "${z_profile_file}"
  printf '%s\n' "BURN_ALIAS=${z_alias}"          >> "${z_profile_file}"
  printf '%s\n' "BURN_SSH_PUBKEY='${z_pubkey}'"  >> "${z_profile_file}"
  printf '%s\n' "BURN_KEY_FILE=${z_key_file}"    >> "${z_profile_file}"
  printf '%s\n' "BURN_COMMAND='${z_command}'"    >> "${z_profile_file}"

  buc_step "BURN profile written: ${z_profile_file}"

  if test "${z_has_pubkey}" = "0"; then
    buh_e
    buh_section "SSH Key Required"
    buyy_cmd_yawp "${z_pubkey_file}"; local -r z_pubkey_yelp="${z_buym_yelp}"
    buh_line "No public key at ${z_pubkey_yelp}."
    buh_line   "Generate one:"
    buh_e
    buh_code   "ssh-keygen -t ed25519 -f ~/.ssh/${z_alias}"
    buh_e
    buh_line   "Then re-run this constructor to populate the public key."
  fi
}

######################################################################
# Command Functions — Constructors

burn_construct_linux() {
  buc_doc_brief "Construct BURN profile for Linux target"
  buc_doc_param "host" "IP or hostname of the Linux machine"
  buc_doc_param "user" "Username on the remote host"
  buc_doc_param "moniker" "Short name (alias = moniker-linux)"
  buc_doc_shown || return 0

  zburn_construct "${1:-}" "${2:-}" "${3:-}" "linux" ""
}

burn_construct_mac() {
  buc_doc_brief "Construct BURN profile for macOS target"
  buc_doc_param "host" "IP or hostname of the Mac"
  buc_doc_param "user" "Username on the remote host"
  buc_doc_param "moniker" "Short name (alias = moniker-mac)"
  buc_doc_shown || return 0

  zburn_construct "${1:-}" "${2:-}" "${3:-}" "mac" ""
}

burn_construct_cygwin() {
  buc_doc_brief "Construct BURN profile for Windows Cygwin target"
  buc_doc_param "host" "IP or hostname of the Windows machine"
  buc_doc_param "user" "Username on the Windows host"
  buc_doc_param "moniker" "Short name (alias = moniker-cyg)"
  buc_doc_shown || return 0

  zburn_construct "${1:-}" "${2:-}" "${3:-}" "cyg" 'C:\cygwin64\bin\bash.exe -l'
}

burn_construct_wsl() {
  buc_doc_brief "Construct BURN profile for Windows WSL target"
  buc_doc_param "host" "IP or hostname of the Windows machine"
  buc_doc_param "user" "Username on the Windows host"
  buc_doc_param "moniker" "Short name (alias = moniker-wsl)"
  buc_doc_shown || return 0

  local -r z_wsl_distro="Ubuntu"
  zburn_construct "${1:-}" "${2:-}" "${3:-}" "wsl" "C:\\Windows\\System32\\wsl.exe -d ${z_wsl_distro}"
}

burn_construct_powershell() {
  buc_doc_brief "Construct BURN profile for Windows PowerShell target"
  buc_doc_param "host" "IP or hostname of the Windows machine"
  buc_doc_param "user" "Username on the Windows host"
  buc_doc_param "moniker" "Short name (alias = moniker-ps)"
  buc_doc_shown || return 0

  zburn_construct "${1:-}" "${2:-}" "${3:-}" "ps" 'C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe'
}

burn_construct_localhost() {
  buc_doc_brief "Construct BURN profile for localhost (local SSH)"
  buc_doc_param "user" "Username on this machine"
  buc_doc_param "moniker" "Short name (alias = moniker-local)"
  buc_doc_shown || return 0

  zburn_construct "localhost" "${1:-}" "${2:-}" "local" "" "id_ed25519"
}

######################################################################
# Command Functions — SSH Automation

burn_verify_ssh() {
  buc_doc_brief "Verify SSH connectivity to a BURN profile"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || buc_die "Profile alias required (e.g., winhost-cyg)"

  buc_step "Testing SSH to ${BURN_ALIAS} (${BURN_HOST})..."
  local z_result=""
  if z_result=$(ssh -o BatchMode=yes -o ConnectTimeout=10 "${BURN_ALIAS}" whoami 2>&1); then
    buc_success "SSH OK: ${BURN_ALIAS} -> ${z_result}"
  else
    buc_die "SSH FAILED: ${BURN_ALIAS} -> ${z_result}"
  fi
}

burn_install_key() {
  buc_doc_brief "Install BURN profile SSH key into authorized_keys (idempotent)"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || buc_die "Profile alias required (e.g., winhost-cyg)"

  local -r z_auth_keys="${HOME}/.ssh/authorized_keys"

  zburn_build_key_line

  mkdir -p "${HOME}/.ssh"

  if test -f "${z_auth_keys}"; then
    local -r z_temp=$(mktemp)
    grep -v "${ZBURN_KEY_MARKER}" "${z_auth_keys}" > "${z_temp}" || true
    mv "${z_temp}" "${z_auth_keys}"
  fi

  printf '%s\n' "${ZBURN_KEY_LINE}" >> "${z_auth_keys}"
  chmod 600 "${z_auth_keys}"

  buc_success "Key installed for ${BURN_ALIAS} in ${z_auth_keys}"
}

######################################################################
# Furnish and Main

zburn_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_CONFIG_DIR       " "BUK config directory (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/burc_regime.sh"
  source "${BURD_BUK_DIR}/burs_regime.sh"
  source "${BURD_BUK_DIR}/buf_fact.sh"
  source "${BURD_BUK_DIR}/burn_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"
  source "${BURD_BUK_DIR}/buym_yelp.sh"
  source "${BURD_BUK_DIR}/buh_handbook.sh"

  zbuv_kindle
  zburd_kindle
  zburd_enforce

  source "${BURD_REGIME_FILE}" || buc_die "Failed to source BURC: ${BURD_REGIME_FILE}"

  zburc_kindle
  zburc_enforce

  source "${BURD_STATION_FILE}" || buc_die "Failed to source BURS: ${BURD_STATION_FILE}"

  zburs_kindle
  zburs_enforce

  zbupr_kindle

  # If BUZ_FOLIO is set, load and kindle the specified profile
  if test -n "${BUZ_FOLIO:-}"; then
    local -r z_profile_file="${BURD_CONFIG_DIR}/users/${BURS_USER}/${BUZ_FOLIO}/burn.env"
    test -f "${z_profile_file}" || buc_die "BURN profile not found: ${z_profile_file}"
    source "${z_profile_file}" || buc_die "Failed to source BURN: ${z_profile_file}"
    zburn_kindle
    zburn_enforce
  fi
}

buc_execute burn_ "Bash Utility Node Regime" zburn_furnish "$@"

# eof
