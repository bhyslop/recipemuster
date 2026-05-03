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
# Node Access). Garrison handles first-run admin trust establishment
# itself (password-fallback on /dev/tty, then key-only thereafter);
# the operator's manual scope is just making sshd reachable on the
# network with a known admin password.

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
  buh_line     "BUK reaches remote nodes through Garrison: a destructive ceremony"
  buh_line     "that provisions one workload account per node, dispatched as the"
  buh_line     "current station user. Garrison never generates or modifies SSH"
  buh_line     "key material; the operator owns all key administration."
  buh_e
  buh_line     "Garrison handles first-run admin trust itself. Its admin SSH"
  buh_line     "session uses PreferredAuthentications=publickey,password — on a"
  buh_line     "fresh node ssh falls through to a password prompt on /dev/tty,"
  buh_line     "the operator types the admin password once, garrison installs"
  buh_line     "the admin pubkey + sshd hardening, and subsequent runs use key"
  buh_line     "auth automatically. The operator's manual scope reduces to:"
  buh_line     "make sshd reachable on the network with a known admin password."
}

zbuhj_render_linux_mac_note() {
  buh_section  "Linux and macOS"
  buh_line     "sshd is typically already installed and reachable. The operator's"
  buh_line     "manual scope is just: confirm sshd is running and the host is"
  buh_line     "reachable from the station. Garrison handles admin pubkey"
  buh_line     "placement and sshd hardening on first run via the password-"
  buh_line     "fallback path."
  buh_e
  buh_line     "If you are unsure of the admin user's password (or it has none),"
  buh_line     "set it to a known value first; garrison clears the password-"
  buh_line     "fallback path on first run, so the value stops affecting SSH:"
  buh_e
  buh_code     "sudo passwd <admin-user>"
  buh_e
  buh_line     "Operators who prefer to pre-establish key trust manually may use"
  buh_line     "the standard recipe instead; garrison's first run then converges"
  buh_line     "as a no-op on the admin pubkey:"
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
  buh_line     "Garrison's first run authenticates as the admin user via password"
  buh_line     "(once) before installing the pubkey. If you already know your local"
  buh_line     "admin password, skip. Otherwise set it to a known value:"
  buh_code     "net user <admin-user> <temp-password>"
  buh_line     "After garrison runs, sshd is hardened to key-only and this value"
  buh_line     "stops affecting SSH (it remains your Windows logon password — leave"
  buh_line     "it or reset to taste)."
  buh_e

  buh_step1    "Install OpenSSH Server:"
  buh_line     "Downloads from Windows Update; may take 10+ minutes."
  buh_code     "Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0"
  buh_e

  buh_step1    "Start and Enable the Service:"
  buh_code     "Start-Service sshd"
  buh_code     "Set-Service -Name sshd -StartupType Automatic"
  buh_e

  buh_step1    "Allow Port ${BUBC_windows_ssh_port} Through Firewall:"
  buh_code     "New-NetFirewallRule -Name ${BUBC_windows_fw_rule_name} -DisplayName \"${BUBC_windows_fw_display_name}\" -Enabled True -Direction Inbound -Protocol TCP -Action Allow -LocalPort ${BUBC_windows_ssh_port}"
  buh_e

  buh_section  "Verify Reachability"
  buh_line     "From the operator's station, confirm sshd answers (a password"
  buh_line     "prompt is expected and correct — garrison's first run will"
  buh_line     "type it through to ssh on /dev/tty):"
  buh_e
  buh_code     "ssh <admin-user>@<windows-host>"
}

zbuhj_render_post_bootstrap() {
  buh_section  "Run Garrison"
  buh_line     "With sshd reachable, run garrison for the chosen workload shell."
  buh_line     "Garrison establishes admin trust on its first run (typing the"
  buh_line     "admin password once when ssh prompts on /dev/tty), then destroys"
  buh_line     "any prior workload account and provisions a fresh one named"
  buh_line     "BURC_WORKLOAD_USER. Subsequent runs use key auth automatically."
  buh_e
  buh_line     "  buw-jpgb <investiture>  — native bash workload (Linux, macOS)"
  buh_line     "  buw-jpgc <investiture>  — Cygwin bash workload (Windows)"
  buh_line     "  buw-jpgw <investiture>  — WSL bash workload (Windows)"
  buh_e
  buh_line     "Choose c or w for Windows by which workload runtime you need."
  buh_line     "On Windows, both share this same admin SSH bootstrap; the"
  buh_line     "shell-letter only routes garrison's own admin session."
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
