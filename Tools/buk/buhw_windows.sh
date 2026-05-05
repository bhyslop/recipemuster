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
# Bash Utility Handbook - Windows OS Procedures
#
# Generic Windows mechanisms for SSH access, WSL, and Cygwin setup.
# All output is handbook display (buh_* combinators only).
# Project-specific topology and Docker policy live in rbhw_windows.sh.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZBUHW_SOURCED:-}" || buc_die "Module buhw multiply sourced - check sourcing hierarchy"
ZBUHW_SOURCED=1

######################################################################
# Internal: Kindle and Sentinel

zbuhw_kindle() {
  test -z "${ZBUHW_KINDLED:-}" || buc_die "Module buhw already kindled"

  # Tinder — fixed Windows paths and network constants
  # Forward slashes throughout — works in PowerShell, display, and icacls
  readonly ZBUHW_SSHD_CONFIG='C:/ProgramData/ssh/sshd_config'
  readonly ZBUHW_ADMIN_AUTH_KEYS='C:/ProgramData/ssh/administrators_authorized_keys'
  readonly ZBUHW_CYGWIN_ROOT='C:/cygwin64'
  readonly ZBUHW_CYGWIN_BASH='C:/cygwin64/bin/bash.exe'
  readonly ZBUHW_SSH_PORT="22"
  readonly ZBUHW_FW_RULE_NAME="sshd"
  readonly ZBUHW_FW_DISPLAY_NAME="OpenSSH Server"

  readonly ZBUHW_KINDLED=1
}

zbuhw_sentinel() {
  test "${ZBUHW_KINDLED:-}" = "1" || buc_die "Module buhw not kindled - call zbuhw_kindle first"
}

######################################################################
# External Functions (buhw_*)

buhw_access_remote() {
  zbuhw_sentinel

  buc_doc_brief "Display SSH client key generation procedure (args: host user key-name alias)"
  buc_doc_shown || return 0

  local z_host="${1:-}"
  local z_user="${2:-}"
  local z_key_name="${3:-}"
  local z_alias="${4:-}"

  test -n "${z_host}"     || buc_die "buhw_access_remote: host required (arg 1)"
  test -n "${z_user}"     || buc_die "buhw_access_remote: user required (arg 2)"
  test -n "${z_key_name}" || buc_die "buhw_access_remote: key-name required (arg 3)"
  test -n "${z_alias}"    || buc_die "buhw_access_remote: alias required (arg 4)"

  buh_section  "SSH Client Key Generation"
  buh_line     "Provision client with a dedicated key for the target host."
  buh_e
  buh_section  "Parameters:"
  buyy_cmd_yawp "${z_host}"; local -r z_host_yelp="${z_buym_yelp}"
  buh_line     "  Host:     ${z_host_yelp}"
  buyy_cmd_yawp "${z_user}"; local -r z_user_yelp="${z_buym_yelp}"
  buh_line     "  User:     ${z_user_yelp}"
  buyy_cmd_yawp "${z_key_name}"; local -r z_key_name_yelp="${z_buym_yelp}"
  buh_line     "  Key name: ${z_key_name_yelp}"
  buyy_cmd_yawp "${z_alias}"; local -r z_alias_yelp="${z_buym_yelp}"
  buh_line     "  Alias:    ${z_alias_yelp}"
  buh_e
  buh_step1    "Generate Key:"
  buh_code     "ssh-keygen -t ed25519 -f ~/.ssh/${z_key_name}"
  buh_e
  buh_step1    "Copy Public Key to Host:"
  buh_line     "The public key must be added to the host's authorized_keys."
  buyy_cmd_yawp "cat ~/.ssh/${z_key_name}.pub"; local -r z_pubkey_cmd_yelp="${z_buym_yelp}"
  buh_line     "Display the key: ${z_pubkey_cmd_yelp}"
  buh_line     "Copy the output for use in the entrypoints procedure."
  buh_e
  buh_section  "Verification:"
  buh_tt       ""  "${BUWZ_HW_VERIFY_SSH}"  ""  " ${z_alias}"
  buh_line     "Expect: connects (may not yet enter target env — routing not configured)."

}

buhw_environment_wsl() {
  zbuhw_sentinel

  buc_doc_brief "Display WSL distribution creation procedure (arg: distro-name)"
  buc_doc_shown || return 0

  local z_distro="${BUZ_FOLIO:-}"
  test -n "${z_distro}" || buc_die "buhw_environment_wsl: distro-name required"

  buh_section  "WSL Distribution Setup"
  buyy_cmd_yawp "${z_distro}"; local -r z_distro_create_yelp="${z_buym_yelp}"
  buh_line     "Create canonical Linux environment: ${z_distro_create_yelp}"
  buh_e
  buh_step1    "Enable WSL (if not already):"
  buh_line     "Run in an elevated PowerShell:"
  buh_code     "wsl --install"
  buh_e
  buh_step1    "Install Base Ubuntu:"
  buh_code     "wsl --install -d Ubuntu"
  buh_e
  buh_step1    "Export and Re-import as Named Distribution:"
  buh_code     "wsl --export Ubuntu ubuntu.tar"
  buh_code     "wsl --import ${z_distro} C:\\\\WSL\\\\${z_distro} ubuntu.tar"
  buh_e
  buh_step1    "Enable systemd:"
  buyy_cmd_yawp "${z_distro}"; local -r z_distro_inside_yelp="${z_buym_yelp}"
  buh_line     "Inside the distro (wsl -d ${z_distro_inside_yelp}):"
  buh_code     "sudo nano /etc/wsl.conf"
  buh_line     "Add:"
  buh_code     "[boot]"
  buh_code     "systemd=true"
  buh_e
  buh_step1    "Restart WSL:"
  buh_code     "wsl --shutdown"
  buh_e
  buh_section  "Verification:"
  buh_code     "wsl -d ${z_distro} systemctl is-system-running"
  buh_line     "Expect: running or degraded (acceptable)."

}

buhw_environment_cygwin() {
  zbuhw_sentinel

  buc_doc_brief "Display Cygwin POSIX userland installation procedure"
  buc_doc_shown || return 0

  buh_section  "Cygwin Installation"
  buh_line     "Install POSIX userland for orchestration testing."
  buh_e
  buh_step1    "Download and Run Cygwin Installer:"
  buh_link     "" "Cygwin Setup (64-bit)" "https://cygwin.com/setup-x86_64.exe"
  buyy_cmd_yawp "${ZBUHW_CYGWIN_ROOT}"; local -r z_cygwin_root_yelp="${z_buym_yelp}"
  buh_line     "Install to: ${z_cygwin_root_yelp}"
  buh_e
  buh_step1    "Required Packages:"
  buh_line     "Select these during installation:"
  buh_code     "bash"
  buh_code     "openssl"
  buh_code     "curl"
  buh_e
  buh_step1    "Verify Installation:"
  buh_line     "Launch Cygwin bash:"
  buh_code     "${ZBUHW_CYGWIN_BASH} -l"
  buh_e
  buh_section  "Verification:"
  buh_code     "openssl version"
  buh_code     "bash --version"
  buh_line     "Expect: bash version >= 3.2"

}

buhw_handbook_top() {
  zbuhw_sentinel

  buc_doc_brief "Display BUK top-level handbook index"
  buc_doc_shown || return 0

  buh_section  "Bash Utility Kit Handbook"
  buh_line     "OS-level procedures across all platforms."
  buh_e
  buh_index_buk

}

buhw_top() {
  zbuhw_sentinel

  buc_doc_brief "Display BUK-level Windows OS procedures checklist"
  buc_doc_shown || return 0

  buh_section  "Windows OS Procedures (BUK)"
  buh_line     "Generic Windows mechanisms — SSH access, WSL, and Cygwin."
  buh_line     "These procedures are OS-level and project-independent."
  buh_e
  buh_section  "BURN Profile Constructors:"
  buh_tt       "  Linux:      " "${BUWZ_RHC_LINUX}" "" " <host> <user> <moniker>"
  buh_tt       "  macOS:      " "${BUWZ_RHC_MAC}" "" " <host> <user> <moniker>"
  buh_tt       "  Localhost:  " "${BUWZ_RHC_LOCALHOST}" "" " <user> <moniker>"
  buh_e
  buh_section  "BURN Profile SSH Operations:"
  buh_tt       "  Verify SSH connection:  " "${BUWZ_HW_VERIFY_SSH}" "" " <alias>"
  buh_tt       "  Install BURN key:       " "${BUWZ_RN_INSTALL_KEY}" "" " <alias>"
  buh_e
  buh_section  "Handbook Procedures (manual steps):"
  buh_tt       "  SSH client key generation:             " "${BUWZ_HW_ACCESS_REMOTE}"
  buh_e
  buh_section  "Environment Procedures:"
  buh_tt       "  WSL distribution setup:                " "${BUWZ_HW_ENV_WSL}"
  buh_tt       "  Cygwin installation:                   " "${BUWZ_HW_ENV_CYGWIN}"

}

# eof
