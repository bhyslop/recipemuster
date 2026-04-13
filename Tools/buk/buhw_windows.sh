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
  buh_t        "Establish Windows as a keys-only SSH endpoint."
  buh_e
  buh_section  "Preconditions:"
  buh_t        "- Windows host with administrator access"
  buh_t        "- Network reachable on TCP/${ZBUHW_SSH_PORT}"
  buh_e
  buh_step1    "Open Elevated PowerShell:"
  buh_t        "Right-click Start → Terminal (Admin), or search 'PowerShell' and Run as Administrator."
  buh_t        "All commands below run in this elevated session."
  buh_e
  buh_step1    "Install OpenSSH Server:"
  buh_t        "This downloads from Windows Update — may take 10+ minutes."
  buh_c        "Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0"
  buh_e
  buh_step1    "Enable OpenSSH Server:"
  buh_c        "Start-Service sshd"
  buh_c        "Set-Service -Name sshd -StartupType Automatic"
  buh_e
  buh_step1    "Allow Port ${ZBUHW_SSH_PORT} Through Firewall:"
  buh_c        "New-NetFirewallRule -Name ${ZBUHW_FW_RULE_NAME} -DisplayName \"${ZBUHW_FW_DISPLAY_NAME}\" -Enabled True -Direction Inbound -Protocol TCP -Action Allow -LocalPort ${ZBUHW_SSH_PORT}"
  buh_e
  buh_step1    "Configure sshd_config:"
  buh_tc       "File: " "${ZBUHW_SSHD_CONFIG}"
  buh_t        "This file is SYSTEM-owned. Edit a temp copy, then replace."
  buh_e
  buh_step2    "Copy to editable location:"
  buh_c        "Copy-Item ${ZBUHW_SSHD_CONFIG} \$env:TEMP\\sshd_config"
  buh_e
  buh_step2    "Edit the copy:"
  buh_c        "notepad \$env:TEMP\\sshd_config"
  buh_t        "Set these directives (add if not present):"
  buh_c        "PasswordAuthentication no"
  buh_c        "PubkeyAuthentication yes"
  buh_c        "PermitEmptyPasswords no"
  buh_t        "Do NOT add UsePAM or ChallengeResponseAuthentication — Windows OpenSSH"
  buh_t        "rejects unrecognized directives and the service will fail to start."
  buh_e
  buh_step2    "Validate before applying:"
  buh_c        "sshd -t -f \$env:TEMP\\sshd_config"
  buh_t        "Expect: no output (silence means valid). Fix any reported errors."
  buh_e
  buh_step2    "Replace the original:"
  buh_c        "Copy-Item \$env:TEMP\\sshd_config ${ZBUHW_SSHD_CONFIG} -Force"
  buh_e
  buh_step1    "Restart Service:"
  buh_c        "Restart-Service sshd"
  buh_e
  buh_step1    "Verify:"
  buh_t        "Discover your Windows username and IP (on the Windows machine):"
  buh_c        "whoami"
  buh_c        "ipconfig"
  buh_t        "Note the username (after the backslash) and IPv4 address."
  buh_e
  buh_t        "From a remote machine, test SSH reachability:"
  buh_c        "ssh <username>@<ip>"
  buh_t        "Expect: Permission denied (publickey). This confirms sshd is running"
  buh_t        "and password login is correctly rejected. Key setup follows in step 2."

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
  buh_t        "Provision client with a dedicated key and deterministic host config."
  buh_e
  buh_section  "Parameters:"
  buh_tc       "  Host:     " "${z_host}"
  buh_tc       "  User:     " "${z_user}"
  buh_tc       "  Key name: " "${z_key_name}"
  buh_tc       "  Alias:    " "${z_alias}"
  buh_e
  buh_step1    "Generate Key:"
  buh_c        "ssh-keygen -t ed25519 -f ~/.ssh/${z_key_name}"
  buh_e
  buh_step1    "Create SSH Config Entry:"
  buh_tc       "Add to " "~/.ssh/config"
  buh_e
  buh_c        "Host ${z_alias}"
  buh_c        "  HostName ${z_host}"
  buh_c        "  User ${z_user}"
  buh_c        "  IdentityFile ~/.ssh/${z_key_name}"
  buh_e
  buh_step1    "Copy Public Key to Host:"
  buh_t        "The public key must be added to the host's authorized_keys."
  buh_tc       "Display the key: " "cat ~/.ssh/${z_key_name}.pub"
  buh_t        "Copy the output for use in the entrypoints procedure."
  buh_e
  buh_section  "Verification:"
  buh_c        "ssh ${z_alias}"
  buh_t        "Expect: connects (may not yet enter target env — routing not configured)."

}

buhw_access_entrypoints() {
  zbuhw_sentinel

  buc_doc_brief "Display SSH command= routing format and key permissions procedure"
  buc_doc_shown || return 0

  buh_section  "SSH Entrypoint Routing via command= Prefix"
  buh_t        "Deterministically route SSH keys to specific environments."
  buh_t        "Each key forces a single environment — no interactive shell selection."
  buh_e
  buh_section  "Preconditions:"
  buh_t        "- OpenSSH server installed (access-base complete)"
  buh_t        "- Public keys available from access-remote"
  buh_e
  buh_step1    "Edit Administrators Authorized Keys:"
  buh_tc       "File: " "${ZBUHW_ADMIN_AUTH_KEYS}"
  buh_t        "Add one line per environment, prefixed with command= directive:"
  buh_e
  buh_c        "command=\"${ZBUHW_CYGWIN_BASH} -l\" ssh-ed25519 AAAA... cygwin"
  buh_c        "command=\"wsl.exe -d DISTRO\"        ssh-ed25519 BBBB... wsl"
  buh_c        "command=\"powershell.exe\"            ssh-ed25519 CCCC... windows"
  buh_e
  buh_tut      "Replace " "DISTRO" " with your WSL distribution name."
  buh_tut      "Replace " "AAAA.../BBBB.../CCCC..." " with actual public key content."
  buh_e
  buh_step1    "Set File Permissions:"
  buh_t        "Run in an elevated PowerShell:"
  buh_c        "icacls \"${ZBUHW_ADMIN_AUTH_KEYS}\" /inheritance:r"
  buh_c        "icacls \"${ZBUHW_ADMIN_AUTH_KEYS}\" /grant \"Administrators:F\""
  buh_e
  buh_section  "Verification:"
  buh_t        "Test each key lands in the correct environment:"
  buh_c        "ssh -i key-cygwin host   # lands in Cygwin bash"
  buh_c        "ssh -i key-wsl host      # lands in WSL"
  buh_c        "ssh -i key-win host      # lands in PowerShell"

}

buhw_environment_wsl() {
  zbuhw_sentinel

  buc_doc_brief "Display WSL distribution creation procedure (arg: distro-name)"
  buc_doc_shown || return 0

  local z_distro="${BUZ_FOLIO:-}"
  test -n "${z_distro}" || buc_die "buhw_environment_wsl: distro-name required"

  buh_section  "WSL Distribution Setup"
  buh_tc       "Create canonical Linux environment: " "${z_distro}"
  buh_e
  buh_step1    "Enable WSL (if not already):"
  buh_t        "Run in an elevated PowerShell:"
  buh_c        "wsl --install"
  buh_e
  buh_step1    "Install Base Ubuntu:"
  buh_c        "wsl --install -d Ubuntu"
  buh_e
  buh_step1    "Export and Re-import as Named Distribution:"
  buh_c        "wsl --export Ubuntu ubuntu.tar"
  buh_c        "wsl --import ${z_distro} C:\\\\WSL\\\\${z_distro} ubuntu.tar"
  buh_e
  buh_step1    "Enable systemd:"
  buh_tct      "Inside the distro (wsl -d " "${z_distro}" "):"
  buh_c        "sudo nano /etc/wsl.conf"
  buh_t        "Add:"
  buh_c        "[boot]"
  buh_c        "systemd=true"
  buh_e
  buh_step1    "Restart WSL:"
  buh_c        "wsl --shutdown"
  buh_e
  buh_section  "Verification:"
  buh_c        "wsl -d ${z_distro} systemctl is-system-running"
  buh_t        "Expect: running or degraded (acceptable)."

}

buhw_environment_cygwin() {
  zbuhw_sentinel

  buc_doc_brief "Display Cygwin POSIX userland installation procedure"
  buc_doc_shown || return 0

  buh_section  "Cygwin Installation"
  buh_t        "Install POSIX userland for orchestration testing."
  buh_e
  buh_step1    "Download and Run Cygwin Installer:"
  buh_link     "" "Cygwin Setup (64-bit)" "https://cygwin.com/setup-x86_64.exe"
  buh_tc       "Install to: " "${ZBUHW_CYGWIN_ROOT}"
  buh_e
  buh_step1    "Required Packages:"
  buh_t        "Select these during installation:"
  buh_c        "bash"
  buh_c        "openssl"
  buh_c        "curl"
  buh_e
  buh_step1    "Verify Installation:"
  buh_t        "Launch Cygwin bash:"
  buh_c        "${ZBUHW_CYGWIN_BASH} -l"
  buh_e
  buh_section  "Verification:"
  buh_c        "openssl version"
  buh_c        "bash --version"
  buh_t        "Expect: bash version >= 3.2"

}

buhw_handbook_top() {
  zbuhw_sentinel

  buc_doc_brief "Display BUK top-level handbook index"
  buc_doc_shown || return 0

  buh_section  "Bash Utility Kit Handbook"
  buh_t        "OS-level procedures across all platforms."
  buh_e
  buh_index_buk

}

buhw_top() {
  zbuhw_sentinel

  buc_doc_brief "Display BUK-level Windows OS procedures checklist"
  buc_doc_shown || return 0

  buh_section  "Windows OS Procedures (BUK)"
  buh_t        "Generic Windows mechanisms — SSH access, WSL, and Cygwin."
  buh_t        "These procedures are OS-level and project-independent."
  buh_e
  buh_section  "BURH Profile Constructors:"
  buh_tTc      "  Linux:      " "buw-rhcl" " <host> <user> <moniker>"
  buh_tTc      "  macOS:      " "buw-rhcm" " <host> <user> <moniker>"
  buh_tTc      "  Cygwin:     " "buw-rhcc" " <host> <user> <moniker>"
  buh_tTc      "  WSL:        " "buw-rhcw" " <host> <user> <moniker>"
  buh_tTc      "  PowerShell: " "buw-rhcp" " <host> <user> <moniker>"
  buh_tTc      "  Localhost:  " "buw-rhcx" " <user> <moniker>"
  buh_e
  buh_section  "SSH Automation:"
  buh_tT       "  Write SSH config:       " "buw-HWsc"
  buh_tTc      "  Verify SSH connection:  " "buw-HWvs" " <alias>"
  buh_tTc      "  Install BURH key:       " "buw-rhk" " <alias>"
  buh_e
  buh_section  "Windows Commands:"
  buh_tT       "  Bootstrap sshd (WSL):   " "buw-wcb"
  buh_e
  buh_section  "Handbook Procedures (manual steps):"
  buh_tT       "  OpenSSH server install:                " "buw-HWab"
  buh_tT       "  SSH client key & host config:          " "buw-HWar"
  buh_tT       "  SSH entrypoint routing (command=):     " "buw-HWax"
  buh_e
  buh_section  "Environment Procedures:"
  buh_tT       "  WSL distribution setup:                " "buw-HWew"
  buh_tT       "  Cygwin installation:                   " "buw-HWec"

}

# eof
