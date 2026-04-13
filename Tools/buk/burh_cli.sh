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
# BURH CLI - Command line interface for BURH host regime operations
#
# Three command families:
#   burh_validate/render/list  — regime read operations (require BUZ_FOLIO)
#   burh_construct_*           — profile constructors (write burh.env)
#   burh_ssh_*/burh_bootstrap* — SSH automation (config, verify, sshd setup)

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Command Functions — Regime Operations

burh_validate() {
  buc_doc_brief "Validate BURH host regime profile via enrollment report"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || buc_die "Profile alias required (e.g., winhost-cyg)"
  buc_step "Validating BURH host regime"
  buv_report BURH "Host Regime"
  buc_step "BURH host regime valid"
}

burh_render() {
  buc_doc_brief "Display diagnostic view of BURH host regime profile"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || buc_die "Profile alias required (e.g., winhost-cyg)"
  buv_render BURH "BURH - Bash Utility Host Regime"
}

burh_list() {
  buc_doc_brief "List available BURH host profiles for current user"
  buc_doc_shown || return 0

  local z_aliases
  z_aliases=$(burh_list_capture) || buc_die "No BURH profiles found for user: ${BURS_USER}"
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
# Args: host user moniker suffix command_value
# command_value may be empty (Linux, macOS, localhost).

zburh_construct() {
  local z_host="${1:-}"
  local z_user="${2:-}"
  local z_moniker="${3:-}"
  local z_suffix="${4:-}"
  local z_command="${5:-}"

  test -n "${z_host}"    || buc_die "host required (arg 1)"
  test -n "${z_user}"    || buc_die "user required (arg 2)"
  test -n "${z_moniker}" || buc_die "moniker required (arg 3)"

  local -r z_alias="${z_moniker}-${z_suffix}"
  local -r z_profile_dir="${BURD_CONFIG_DIR}/users/${BURS_USER}/${z_alias}"
  local -r z_profile_file="${z_profile_dir}/burh.env"
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

  printf '%s\n' "BURH_HOST=${z_host}"           >  "${z_profile_file}"
  printf '%s\n' "BURH_USER=${z_user}"            >> "${z_profile_file}"
  printf '%s\n' "BURH_ALIAS=${z_alias}"          >> "${z_profile_file}"
  printf '%s\n' "BURH_SSH_PUBKEY='${z_pubkey}'"  >> "${z_profile_file}"
  printf '%s\n' "BURH_COMMAND='${z_command}'"    >> "${z_profile_file}"

  buc_step "BURH profile written: ${z_profile_file}"

  if test "${z_has_pubkey}" = "0"; then
    buh_e
    buh_section "SSH Key Required"
    buh_tct    "No public key at " "${z_pubkey_file}" "."
    buh_t      "Generate one:"
    buh_e
    buh_c      "ssh-keygen -t ed25519 -f ~/.ssh/${z_alias}"
    buh_e
    buh_t      "Then re-run this constructor to populate the public key."
  fi
}

######################################################################
# Command Functions — Constructors

burh_construct_linux() {
  buc_doc_brief "Construct BURH profile for Linux target"
  buc_doc_param "host" "IP or hostname of the Linux machine"
  buc_doc_param "user" "Username on the remote host"
  buc_doc_param "moniker" "Short name (alias = moniker-linux)"
  buc_doc_shown || return 0

  zburh_construct "${1:-}" "${2:-}" "${3:-}" "linux" ""
}

burh_construct_mac() {
  buc_doc_brief "Construct BURH profile for macOS target"
  buc_doc_param "host" "IP or hostname of the Mac"
  buc_doc_param "user" "Username on the remote host"
  buc_doc_param "moniker" "Short name (alias = moniker-mac)"
  buc_doc_shown || return 0

  zburh_construct "${1:-}" "${2:-}" "${3:-}" "mac" ""
}

burh_construct_cygwin() {
  buc_doc_brief "Construct BURH profile for Windows Cygwin target"
  buc_doc_param "host" "IP or hostname of the Windows machine"
  buc_doc_param "user" "Username on the Windows host"
  buc_doc_param "moniker" "Short name (alias = moniker-cyg)"
  buc_doc_shown || return 0

  zburh_construct "${1:-}" "${2:-}" "${3:-}" "cyg" 'C:\cygwin64\bin\bash.exe -l'
}

burh_construct_wsl() {
  buc_doc_brief "Construct BURH profile for Windows WSL target"
  buc_doc_param "host" "IP or hostname of the Windows machine"
  buc_doc_param "user" "Username on the Windows host"
  buc_doc_param "moniker" "Short name (alias = moniker-wsl)"
  buc_doc_shown || return 0

  local -r z_wsl_distro="Ubuntu"
  zburh_construct "${1:-}" "${2:-}" "${3:-}" "wsl" "C:\\Windows\\System32\\wsl.exe -d ${z_wsl_distro}"
}

burh_construct_powershell() {
  buc_doc_brief "Construct BURH profile for Windows PowerShell target"
  buc_doc_param "host" "IP or hostname of the Windows machine"
  buc_doc_param "user" "Username on the Windows host"
  buc_doc_param "moniker" "Short name (alias = moniker-ps)"
  buc_doc_shown || return 0

  zburh_construct "${1:-}" "${2:-}" "${3:-}" "ps" 'C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe'
}

burh_construct_localhost() {
  buc_doc_brief "Construct BURH profile for localhost (local SSH)"
  buc_doc_param "user" "Username on this machine"
  buc_doc_param "moniker" "Short name (alias = moniker-local)"
  buc_doc_shown || return 0

  zburh_construct "localhost" "${1:-}" "${2:-}" "local" ""
}

######################################################################
# Command Functions — SSH Automation

burh_ssh_config() {
  buc_doc_brief "Generate SSH config entries from all BURH profiles"
  buc_doc_shown || return 0

  local z_aliases
  z_aliases=$(burh_list_capture) || buc_die "No BURH profiles found for user: ${BURS_USER}"

  local -r z_config="${HOME}/.ssh/config"
  local -r z_begin="# --- BEGIN BURH MANAGED ---"
  local -r z_end="# --- END BURH MANAGED ---"

  # Build managed section content
  local z_section=""
  z_section="${z_begin}"$'\n'
  z_section+="# Generated by buw-HWsc — do not edit this section manually"$'\n'

  local z_alias=""
  for z_alias in ${z_aliases}; do
    local z_profile="${BURD_CONFIG_DIR}/users/${BURS_USER}/${z_alias}/burh.env"
    # Source directly — BURH not kindled, no readonly conflict
    source "${z_profile}"
    z_section+=$'\n'
    z_section+="Host ${BURH_ALIAS}"$'\n'
    z_section+="  HostName ${BURH_HOST}"$'\n'
    z_section+="  User ${BURH_USER}"$'\n'
    z_section+="  IdentityFile ~/.ssh/${BURH_ALIAS}"$'\n'
  done

  z_section+="${z_end}"$'\n'

  # Write managed section into config
  mkdir -p "${HOME}/.ssh"

  if test -f "${z_config}" && grep -q "${z_begin}" "${z_config}"; then
    # Replace existing managed section
    local -r z_temp=$(mktemp)
    local z_in_managed=0
    while IFS= read -r z_line || test -n "${z_line}"; do
      if test "${z_line}" = "${z_begin}"; then
        z_in_managed=1
        printf '%s' "${z_section}" >> "${z_temp}"
        continue
      fi
      if test "${z_in_managed}" = "1"; then
        test "${z_line}" = "${z_end}" && z_in_managed=0
        continue
      fi
      printf '%s\n' "${z_line}" >> "${z_temp}"
    done < "${z_config}"
    mv "${z_temp}" "${z_config}"
  elif test -f "${z_config}"; then
    # Append managed section
    printf '\n%s' "${z_section}" >> "${z_config}"
  else
    # Create new config
    printf '%s' "${z_section}" > "${z_config}"
  fi

  chmod 600 "${z_config}"
  buc_step "SSH config updated: ${z_config}"
  buc_step "Managed profiles: ${z_aliases}"
}

burh_verify_ssh() {
  buc_doc_brief "Verify SSH connectivity to a BURH profile"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || buc_die "Profile alias required (e.g., winhost-cyg)"

  buc_step "Testing SSH to ${BURH_ALIAS} (${BURH_HOST})..."
  local z_result=""
  if z_result=$(ssh -o BatchMode=yes -o ConnectTimeout=10 "${BURH_ALIAS}" whoami 2>&1); then
    buc_success "SSH OK: ${BURH_ALIAS} -> ${z_result}"
  else
    buc_die "SSH FAILED: ${BURH_ALIAS} -> ${z_result}"
  fi
}

burh_bootstrap_sshd() {
  buc_doc_brief "Bootstrap OpenSSH server on Windows via PowerShell from WSL"
  buc_doc_shown || return 0

  command -v powershell.exe >/dev/null 2>&1 \
    || buc_die "powershell.exe not found — this command must run from WSL"

  zbuhw_sentinel

  local z_aliases
  z_aliases=$(burh_list_capture) || buc_die "No BURH profiles found for user: ${BURS_USER}"

  buc_step "Bootstrap: OpenSSH Server on Windows"

  # Step 1: Probe sshd
  buc_step "Probing OpenSSH Server..."
  local z_status=""
  if z_status=$(powershell.exe -Command 'Get-Service sshd -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Status' 2>&1); then
    z_status=$(printf '%s' "${z_status}" | tr -d '\r\n')
    buc_step "OpenSSH Server status: ${z_status}"
  else
    buc_step "OpenSSH Server not installed — installing..."
    powershell.exe -Command 'Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0' \
      || buc_die "Failed to install OpenSSH Server (elevated terminal required)"
    buc_step "OpenSSH Server installed"
  fi

  # Step 2: Write sshd_config from template
  buc_step "Writing sshd_config..."
  {
    echo "# Generated by buw-HWbs — BURH bootstrap"
    echo "Port ${ZBUHW_SSH_PORT}"
    echo "PasswordAuthentication no"
    echo "PubkeyAuthentication yes"
    echo "PermitEmptyPasswords no"
    echo "AuthorizedKeysFile .ssh/authorized_keys"
    echo "Subsystem sftp sftp-server.exe"
    echo "Match Group administrators"
    echo "  AuthorizedKeysFile __PROGRAMDATA__/ssh/administrators_authorized_keys"
  } | powershell.exe -Command '$input | Set-Content -Path "C:/ProgramData/ssh/sshd_config" -Force' \
    || buc_die "Failed to write sshd_config (elevated terminal required)"
  buc_step "sshd_config written"

  # Validate sshd_config
  powershell.exe -Command 'sshd -t -f "C:/ProgramData/ssh/sshd_config"' \
    || buc_die "sshd_config validation failed — check config syntax"
  buc_step "sshd_config validated"

  # Step 3: Write authorized_keys from BURH profiles
  buc_step "Writing administrators_authorized_keys..."
  {
    echo "# Generated by buw-HWbs — BURH bootstrap"
    local z_alias=""
    for z_alias in ${z_aliases}; do
      local z_profile="${BURD_CONFIG_DIR}/users/${BURS_USER}/${z_alias}/burh.env"
      source "${z_profile}"
      if test -n "${BURH_COMMAND}"; then
        echo "command=\"${BURH_COMMAND}\" ${BURH_SSH_PUBKEY}"
      else
        echo "${BURH_SSH_PUBKEY}"
      fi
    done
  } | powershell.exe -Command '$input | Set-Content -Path "C:/ProgramData/ssh/administrators_authorized_keys" -Force' \
    || buc_die "Failed to write authorized_keys (elevated terminal required)"
  buc_step "authorized_keys written"

  # Step 4: Set authorized_keys permissions
  buc_step "Setting authorized_keys permissions..."
  powershell.exe -Command 'icacls "C:/ProgramData/ssh/administrators_authorized_keys" /inheritance:r /grant "Administrators:F" /grant "SYSTEM:F"' \
    || buc_die "Failed to set authorized_keys permissions"
  buc_step "Permissions set"

  # Step 5: Configure and start sshd service
  buc_step "Configuring sshd service..."
  powershell.exe -Command 'Set-Service -Name sshd -StartupType Automatic' \
    || buc_die "Failed to configure sshd service"
  powershell.exe -Command 'Restart-Service sshd' \
    || buc_die "Failed to restart sshd service"
  buc_step "sshd service configured and started"

  # Step 6: Firewall rule (idempotent)
  buc_step "Checking firewall rule..."
  if powershell.exe -Command "Get-NetFirewallRule -Name '${ZBUHW_FW_RULE_NAME}' -ErrorAction SilentlyContinue" >/dev/null 2>&1; then
    buc_step "Firewall rule already exists"
  else
    powershell.exe -Command "New-NetFirewallRule -Name '${ZBUHW_FW_RULE_NAME}' -DisplayName '${ZBUHW_FW_DISPLAY_NAME}' -Enabled True -Direction Inbound -Protocol TCP -Action Allow -LocalPort ${ZBUHW_SSH_PORT}" \
      || buc_die "Failed to create firewall rule"
    buc_step "Firewall rule created"
  fi

  # Step 7: Loopback verify
  buc_step "Verifying loopback SSH..."
  local z_whoami=""
  if z_whoami=$(ssh -o BatchMode=yes -o ConnectTimeout=5 -o StrictHostKeyChecking=no localhost whoami 2>&1); then
    z_whoami=$(printf '%s' "${z_whoami}" | tr -d '\r\n')
    buc_success "Bootstrap complete — loopback SSH: ${z_whoami}"
  else
    buc_warn "Loopback verify deferred (keys may need setup from client machine)"
    buc_step "Bootstrap structurally complete"
  fi
}

######################################################################
# Furnish and Main

zburh_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_CONFIG_DIR       " "BUK config directory (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/burc_regime.sh"
  source "${BURD_BUK_DIR}/burs_regime.sh"
  source "${BURD_BUK_DIR}/burh_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"
  source "${BURD_BUK_DIR}/buh_handbook.sh"
  source "${BURD_BUK_DIR}/buhw_windows.sh"

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
  zbuhw_kindle

  # If BUZ_FOLIO is set, load and kindle the specified profile
  if test -n "${BUZ_FOLIO:-}"; then
    local -r z_profile_file="${BURD_CONFIG_DIR}/users/${BURS_USER}/${BUZ_FOLIO}/burh.env"
    test -f "${z_profile_file}" || buc_die "BURH profile not found: ${z_profile_file}"
    source "${z_profile_file}" || buc_die "Failed to source BURH: ${z_profile_file}"
    zburh_kindle
    zburh_enforce
  fi
}

buc_execute burh_ "Bash Utility Host Regime" zburh_furnish "$@"

# eof
