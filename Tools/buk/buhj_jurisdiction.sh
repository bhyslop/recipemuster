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
# Bash Utility Handbook - Jurisdiction Procedures
#
# Handbook content for BUK's remote-node feature area (BUS0 §Remote
# Node Access). On Windows, fenestrate handles first-run admin trust
# establishment and sshd_config hardening (password-fallback on
# /dev/tty during its phase-1 admin session, then key-only
# thereafter); garrison handles workload account provisioning. The
# operator's manual scope is just making sshd reachable on the
# network with a known admin password. Linux/Mac admin trust is
# operator-manual (e.g., ssh-copy-id); BUK ships no fenestrate for
# non-Windows-OpenSSH nodes.

set -euo pipefail

test -z "${ZBUHJ_SOURCED:-}" || buc_die "Module buhj multiply sourced - check sourcing hierarchy"
ZBUHJ_SOURCED=1

######################################################################
# Internal: Kindle and Sentinel

zbuhj_kindle() {
  test -z "${ZBUHJ_KINDLED:-}" || buc_die "Module buhj already kindled"

  test -n "${BUBC_windows_ssh_port:-}" || buc_die "buhj requires bubc_constants.sh sourced before kindle"

  readonly ZBUHJ_KINDLED=1
}

zbuhj_sentinel() {
  test "${ZBUHJ_KINDLED:-}" = "1" || buc_die "Module buhj not kindled - call zbuhj_kindle first"
}

######################################################################
# Internal: Section renderers

zbuhj_render_landing() {
  buh_section  "Jurisdiction Handbook"
  buh_line     "BUK reaches remote nodes through two ceremonies. Fenestrate"
  buh_line     "(Windows OpenSSH only) hardens admin SSH trust and sshd_config"
  buh_line     "to key-only; Garrison provisions the workload account. BUK"
  buh_line     "never generates or modifies SSH key material; the operator owns"
  buh_line     "all key administration."
  buh_e
  buh_line     "On Windows, fenestrate handles first-run admin trust itself: its"
  buh_line     "phase-1 admin SSH session uses PreferredAuthentications="
  buh_line     "publickey,password, falls through to a /dev/tty password prompt"
  buh_line     "on a fresh node, the operator types the admin password once,"
  buh_line     "fenestrate installs the admin pubkey + sshd hardening + restarts"
  buh_line     "sshd, and phase 2 reconnects by key alone. Subsequent fenestrate"
  buh_line     "and garrison runs use key auth automatically. The operator's"
  buh_line     "manual scope on Windows reduces to: make sshd reachable on the"
  buh_line     "network with a known admin password."
  buh_e
  buh_line     "On Linux/Mac there is no fenestrate verb; admin trust is"
  buh_line     "operator-manual (ssh-copy-id or equivalent). Garrison runs"
  buh_line     "directly against an existing key-trusted admin foothold."
}

zbuhj_render_linux_mac_note() {
  buh_section  "Linux and macOS"
  buh_line     "sshd is typically already installed and reachable. The operator's"
  buh_line     "manual scope is: confirm sshd is running and the host is reachable,"
  buh_line     "then place the admin pubkey via ssh-copy-id (or equivalent) so"
  buh_line     "garrison's first admin SSH succeeds by key alone. There is no"
  buh_line     "fenestrate verb for non-Windows-OpenSSH nodes; sshd hardening is"
  buh_line     "operator-managed."
  buh_e
  buh_code     "ssh-copy-id -i ~/.ssh/<admin-pubkey>.pub <admin-user>@<host>"
}

zbuhj_render_windows_bootstrap() {
  buh_section  "Windows: sshd Reachability"
  buh_line     "All steps run on the Windows host in an elevated PowerShell."
  buh_line     "Right-click Start, Terminal (Admin), or search 'PowerShell' and"
  buh_line     "Run as Administrator."
  buh_e
  buh_section  "Preconditions:"
  buh_line     "- Windows host with administrator access"
  buh_line     "- Network reachable on TCP/${BUBC_windows_ssh_port} from operator's station"
  buh_e

  buh_step1    "Set or Confirm Admin Password:"
  buh_line     "Fenestrate's phase-1 admin SSH session authenticates via password"
  buh_line     "(once) before installing the pubkey. If you already know your local"
  buh_line     "admin password, skip. Otherwise set it to a known value:"
  buh_code     "net user <admin-user> <temp-password>"
  buh_line     "After fenestrate runs, sshd is hardened to key-only and this value"
  buh_line     "stops affecting SSH (it remains your Windows logon password — leave"
  buh_line     "it or reset to taste)."
  buh_e

  buh_step1    "Install OpenSSH Server:"
  buh_line     "Downloads from Windows Update; may take 10+ minutes."
  buh_code     "Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0"
  buh_e

  buh_step1    "Start and Enable the Service:"
  buh_line     "First start also generates the default sshd_config."
  buh_code     "Start-Service sshd"
  buh_code     "Set-Service -Name sshd -StartupType Automatic"
  buh_e

  buyy_cmd_yawp "${BUBC_windows_sshd_config}"; local -r z_sshd_config_yelp="${z_buym_yelp}"
  buh_step1    "Enable Password Authentication in sshd_config:"
  buh_line     "Windows OpenSSH ships with sshd_config in a state that blocks"
  buh_line     "fenestrate's phase-1 password fallback (the wire protocol"
  buh_line     "advertises keyboard-interactive but the server refuses to"
  buh_line     "actually prompt). Open sshd_config in an editor; notepad"
  buh_line     "inherits your elevated context so SYSTEM-owned writes succeed."
  buh_line     "File: ${z_sshd_config_yelp}"
  buh_e
  buh_code     "notepad ${BUBC_windows_sshd_config}"
  buh_e
  buh_line     "Find the PasswordAuthentication directive (commented or"
  buh_line     "otherwise) and set it to exactly:"
  buh_e
  buh_code     "PasswordAuthentication yes"
  buh_e
  buh_line     "If no PasswordAuthentication line exists, add one. Save."
  buh_line     "Fenestrate flips this back to 'no' as part of its hardening"
  buh_line     "step, so this is a temporary state. Restart sshd to pick up"
  buh_line     "the change:"
  buh_e
  buh_code     "Restart-Service sshd"
  buh_e

  buh_step1    "Allow Port ${BUBC_windows_ssh_port} Through Firewall:"
  buh_code     "New-NetFirewallRule -Name ${BUBC_windows_fw_rule_name} -DisplayName \"${BUBC_windows_fw_display_name}\" -Enabled True -Direction Inbound -Protocol TCP -Action Allow -LocalPort ${BUBC_windows_ssh_port}"
  buh_e

  buh_section  "Verify Reachability"
  buh_line     "From the operator's station, confirm sshd answers (a password"
  buh_line     "prompt is expected and correct — fenestrate's phase-1 admin"
  buh_line     "session will type it through to ssh on /dev/tty):"
  buh_e
  buh_code     "ssh <admin-user>@<windows-host>"
}

zbuhj_render_post_bootstrap() {
  buh_section  "Run Fenestrate, then Garrison"
  buh_line     "On Windows: with sshd reachable, run fenestrate first. Fenestrate"
  buh_line     "establishes admin trust (typing the admin password once when its"
  buh_line     "phase-1 ssh prompts on /dev/tty), hardens sshd_config to"
  buh_line     "key-only, restarts sshd, and reconnects to verify. After"
  buh_line     "fenestrate succeeds, run garrison for the chosen workload shell:"
  buh_e
  buh_line     "  buw-jpF  <investiture>  — fenestrate (Windows OpenSSH only)"
  buh_e
  buh_line     "  buw-jpGb <investiture>  — native bash workload (Linux, macOS)"
  buh_line     "  buw-jpGc <investiture>  — Cygwin bash workload (Windows)"
  buh_line     "  buw-jpGw <investiture>  — WSL bash workload (Windows)"
  buh_e
  buh_line     "Choose c or w for Windows by which workload runtime you need."
  buh_line     "Both share fenestrate's admin SSH harden; the shell-letter on"
  buh_line     "garrison only routes the workload account ceremony."
  buh_e
  buh_line     "On Linux/Mac: skip fenestrate (no equivalent verb). Run garrison"
  buh_line     "directly against an existing key-trusted admin foothold (place"
  buh_line     "the admin pubkey via ssh-copy-id beforehand if not already in"
  buh_line     "place)."
  buh_e
  buh_line     "After garrison succeeds, dispatch work via the workload"
  buh_line     "tabtargets:"
  buh_line     "  buw-jwk <investiture>           — probe workload reachability"
  buh_line     "  buw-jwc <investiture> <file>    — run a command file"
  buh_line     "  buw-jws <investiture>           — interactive workload session"
}

######################################################################
# External Functions (buhj_*)

buhj_top() {
  zbuhj_sentinel

  buc_doc_brief "Display jurisdiction handbook landing + sshd-reachability procedures"
  buc_doc_shown || return 0

  zbuhj_render_landing
  buh_e
  zbuhj_render_linux_mac_note
  buh_e
  zbuhj_render_windows_bootstrap
  buh_e
  zbuhj_render_post_bootstrap
}

# eof
