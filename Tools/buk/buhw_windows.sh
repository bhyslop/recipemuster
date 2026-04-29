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

buhw_access_base() {
  zbuhw_sentinel

  buc_doc_brief "Display OpenSSH server installation and lockdown procedure"
  buc_doc_shown || return 0

  buh_section  "OpenSSH Server Installation & Lockdown"
  buh_line     "Establish Windows as a keys-only SSH endpoint."
  buh_e
  buh_section  "Preconditions:"
  buh_line     "- Windows host with administrator access"
  buh_line     "- Network reachable on TCP/${ZBUHW_SSH_PORT}"
  buh_e
  buh_step1    "Open Elevated PowerShell:"
  buh_line     "Right-click Start → Terminal (Admin), or search 'PowerShell' and Run as Administrator."
  buh_line     "All commands below run in this elevated session."
  buh_e
  buh_step1    "Install OpenSSH Server:"
  buh_line     "This downloads from Windows Update — may take 10+ minutes."
  buh_code     "Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0"
  buh_e
  buh_step1    "Enable OpenSSH Server:"
  buh_code     "Start-Service sshd"
  buh_code     "Set-Service -Name sshd -StartupType Automatic"
  buh_e
  buh_step1    "Allow Port ${ZBUHW_SSH_PORT} Through Firewall:"
  buh_code     "New-NetFirewallRule -Name ${ZBUHW_FW_RULE_NAME} -DisplayName \"${ZBUHW_FW_DISPLAY_NAME}\" -Enabled True -Direction Inbound -Protocol TCP -Action Allow -LocalPort ${ZBUHW_SSH_PORT}"
  buh_e
  buh_step1    "Configure sshd_config:"
  buyy_cmd_yawp "${ZBUHW_SSHD_CONFIG}"; local -r z_sshd_config_yelp="${z_buym_yelp}"
  buh_line     "File: ${z_sshd_config_yelp}"
  buh_line     "This file is SYSTEM-owned. Edit a temp copy, then replace."
  buh_e
  buh_step2    "Copy to editable location:"
  buh_code     "Copy-Item ${ZBUHW_SSHD_CONFIG} \$env:TEMP\\sshd_config"
  buh_e
  buh_step2    "Edit the copy:"
  buh_code     "notepad \$env:TEMP\\sshd_config"
  buh_line     "Set these directives (add if not present):"
  buh_code     "PasswordAuthentication no"
  buh_code     "PubkeyAuthentication yes"
  buh_code     "PermitEmptyPasswords no"
  buh_line     "Do NOT add UsePAM or ChallengeResponseAuthentication — Windows OpenSSH"
  buh_line     "rejects unrecognized directives and the service will fail to start."
  buh_e
  buh_step2    "Validate before applying:"
  buh_code     "sshd -t -f \$env:TEMP\\sshd_config"
  buh_line     "Expect: no output (silence means valid). Fix any reported errors."
  buh_e
  buh_step2    "Replace the original:"
  buh_code     "Copy-Item \$env:TEMP\\sshd_config ${ZBUHW_SSHD_CONFIG} -Force"
  buh_e
  buh_step1    "Restart Service:"
  buh_code     "Restart-Service sshd"
  buh_e
  buh_step1    "Verify:"
  buh_line     "Discover your Windows username and IP (on the Windows machine):"
  buh_code     "whoami"
  buh_code     "ipconfig"
  buh_line     "Note the username (after the backslash) and IPv4 address."
  buh_e
  buh_line     "From a remote machine, test SSH reachability:"
  buh_code     "ssh <username>@<ip>"
  buh_line     "Expect: Permission denied (publickey). This confirms sshd is running"
  buh_line     "and password login is correctly rejected. Key setup follows in step 2."

}

buhw_access_remote() {
  zbuhw_sentinel

  buc_doc_brief "Display SSH client key generation and host config procedure (args: host user key-name alias)"
  buc_doc_shown || return 0

  local z_host="${1:-}"
  local z_user="${2:-}"
  local z_key_name="${3:-}"
  local z_alias="${4:-}"

  test -n "${z_host}"     || buc_die "buhw_access_remote: host required (arg 1)"
  test -n "${z_user}"     || buc_die "buhw_access_remote: user required (arg 2)"
  test -n "${z_key_name}" || buc_die "buhw_access_remote: key-name required (arg 3)"
  test -n "${z_alias}"    || buc_die "buhw_access_remote: alias required (arg 4)"

  buh_section  "SSH Client Key & Host Configuration"
  buh_line     "Provision client with a dedicated key and deterministic host config."
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
  buh_step1    "Create SSH Config Entry:"
  buyy_cmd_yawp "~/.ssh/config"; local -r z_ssh_config_yelp="${z_buym_yelp}"
  buh_line     "Add to ${z_ssh_config_yelp}"
  buh_e
  buh_code     "Host ${z_alias}"
  buh_code     "  HostName ${z_host}"
  buh_code     "  User ${z_user}"
  buh_code     "  IdentityFile ~/.ssh/${z_key_name}"
  buh_e
  buh_step1    "Copy Public Key to Host:"
  buh_line     "The public key must be added to the host's authorized_keys."
  buyy_cmd_yawp "cat ~/.ssh/${z_key_name}.pub"; local -r z_pubkey_cmd_yelp="${z_buym_yelp}"
  buh_line     "Display the key: ${z_pubkey_cmd_yelp}"
  buh_line     "Copy the output for use in the entrypoints procedure."
  buh_e
  buh_section  "Verification:"
  buh_code     "ssh ${z_alias}"
  buh_line     "Expect: connects (may not yet enter target env — routing not configured)."

}

buhw_access_entrypoints() {
  zbuhw_sentinel

  buc_doc_brief "Display SSH command= routing format and key permissions procedure"
  buc_doc_shown || return 0

  buh_section  "SSH Entrypoint Routing via command= Prefix"
  buh_line     "Deterministically route SSH keys to specific environments."
  buh_line     "Each key forces a single environment — no interactive shell selection."
  buh_e
  buh_section  "Preconditions:"
  buh_line     "- OpenSSH server installed (access-base complete)"
  buh_line     "- Public keys available from access-remote"
  buh_e
  buh_step1    "Edit Administrators Authorized Keys:"
  buyy_cmd_yawp "${ZBUHW_ADMIN_AUTH_KEYS}"; local -r z_admin_auth_keys_yelp="${z_buym_yelp}"
  buh_line     "File: ${z_admin_auth_keys_yelp}"
  buh_line     "Add one line per environment, prefixed with command= directive:"
  buh_e
  buh_code     "command=\"${ZBUHW_CYGWIN_BASH} -l\" ssh-ed25519 AAAA... cygwin"
  buh_code     "command=\"wsl.exe -d DISTRO\"        ssh-ed25519 BBBB... wsl"
  buh_code     "command=\"powershell.exe\"            ssh-ed25519 CCCC... windows"
  buh_e
  buyy_ui_yawp "DISTRO"; local -r z_distro_placeholder_yelp="${z_buym_yelp}"
  buh_line     "Replace ${z_distro_placeholder_yelp} with your WSL distribution name."
  buyy_ui_yawp "AAAA.../BBBB.../CCCC..."; local -r z_key_placeholders_yelp="${z_buym_yelp}"
  buh_line     "Replace ${z_key_placeholders_yelp} with actual public key content."
  buh_e
  buh_step1    "Set File Permissions:"
  buh_line     "Run in an elevated PowerShell:"
  buh_code     "icacls \"${ZBUHW_ADMIN_AUTH_KEYS}\" /inheritance:r"
  buh_code     "icacls \"${ZBUHW_ADMIN_AUTH_KEYS}\" /grant \"Administrators:F\""
  buh_e
  buh_section  "Verification:"
  buh_line     "Test each key lands in the correct environment:"
  buh_code     "ssh -i key-cygwin host   # lands in Cygwin bash"
  buh_code     "ssh -i key-wsl host      # lands in WSL"
  buh_code     "ssh -i key-win host      # lands in PowerShell"

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
  buh_tt       "  Cygwin:     " "${BUWZ_RHC_CYGWIN}" "" " <host> <user> <moniker>"
  buh_tt       "  WSL:        " "${BUWZ_RHC_WSL}" "" " <host> <user> <moniker>"
  buh_tt       "  PowerShell: " "${BUWZ_RHC_POWERSHELL}" "" " <host> <user> <moniker>"
  buh_tt       "  Localhost:  " "${BUWZ_RHC_LOCALHOST}" "" " <user> <moniker>"
  buh_e
  buh_section  "SSH Automation:"
  buh_tt       "  Write SSH config:       " "${BUWZ_HW_SSH_CONFIG}"
  buh_tt       "  Verify SSH connection:  " "${BUWZ_HW_VERIFY_SSH}" "" " <alias>"
  buh_tt       "  Install BURN key:       " "${BUWZ_RH_INSTALL_KEY}" "" " <alias>"
  buh_e
  buh_section  "Windows Commands:"
  buh_tt       "  Bootstrap sshd (WSL):   " "${BUWZ_WC_BOOTSTRAP}"
  buh_e
  buh_section  "Handbook Procedures (manual steps):"
  buh_tt       "  OpenSSH server install:                " "${BUWZ_HW_ACCESS_BASE}"
  buh_tt       "  SSH client key & host config:          " "${BUWZ_HW_ACCESS_REMOTE}"
  buh_tt       "  SSH entrypoint routing (command=):     " "${BUWZ_HW_ACCESS_ENTRY}"
  buh_e
  buh_section  "Environment Procedures:"
  buh_tt       "  WSL distribution setup:                " "${BUWZ_HW_ENV_WSL}"
  buh_tt       "  Cygwin installation:                   " "${BUWZ_HW_ENV_CYGWIN}"

}

# eof
