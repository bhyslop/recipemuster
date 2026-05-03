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
# Node Access). Garrison authenticates as an admin SSH user using
# operator-managed keys; first-time admin trust is established here,
# out-of-band, before any garrison runs.

set -euo pipefail

test -z "${ZBUHJ_SOURCED:-}" || buc_die "Module buhj multiply sourced - check sourcing hierarchy"
ZBUHJ_SOURCED=1

######################################################################
# Internal: Kindle and Sentinel

zbuhj_kindle() {
  test -z "${ZBUHJ_KINDLED:-}" || buc_die "Module buhj already kindled"

  test -n "${BUBC_windows_admin_auth_keys:-}" || buc_die "buhj requires bubc_constants.sh sourced before kindle"

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
  buh_line     "current station user. Garrison authenticates over key-only admin"
  buh_line     "SSH; it never generates or modifies SSH key material."
  buh_e
  buh_line     "First-time admin trust is operator-manual: install sshd on the"
  buh_line     "node, place the admin pubkey in admin's authorized_keys with"
  buh_line     "no command= directive, and disable password auth. Garrison"
  buh_line     "rewrites the admin authorized_keys on first run with the"
  buh_line     "shell-letter command= directive that routes its sessions."
}

zbuhj_render_linux_mac_note() {
  buh_section  "Linux and macOS"
  buh_line     "Admin trust is the standard SSH client recipe — copy the operator's"
  buh_line     "admin pubkey to the remote admin user's authorized_keys."
  buh_e
  buh_code     "ssh-copy-id -i ~/.ssh/<admin-pubkey>.pub <admin-user>@<host>"
  buh_e
  buh_line     "If the node already disallows password auth, append the pubkey"
  buh_line     "to ~<admin-user>/.ssh/authorized_keys via an existing trusted"
  buh_line     "channel. No further preparation is required before garrison."
}

zbuhj_render_windows_bootstrap() {
  buyy_cmd_yawp "${BUBC_windows_sshd_config}";     local -r z_sshd_config_yelp="${z_buym_yelp}"
  buyy_cmd_yawp "${BUBC_windows_admin_auth_keys}"; local -r z_admin_keys_yelp="${z_buym_yelp}"

  buh_section  "Windows Admin Trust Bootstrap"
  buh_line     "All steps run on the Windows host except Step 9, which runs"
  buh_line     "from the operator's station to verify."
  buh_e
  buh_section  "Preconditions:"
  buh_line     "- Windows host with administrator access"
  buh_line     "- Operator's admin SSH keypair already exists on the station"
  buh_line     "  (key generation is operator-owned; not a handbook concern)"
  buh_line     "- Network reachable on TCP/${BUBC_windows_ssh_port} from operator's station"
  buh_e

  buh_step1    "Open Elevated PowerShell:"
  buh_line     "Right-click Start, Terminal (Admin), or search 'PowerShell' and"
  buh_line     "Run as Administrator. All commands below run in this elevated"
  buh_line     "session unless explicitly noted."
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

  buh_step1    "Harden sshd_config to Key-Only:"
  buh_line     "File: ${z_sshd_config_yelp}"
  buh_line     "SYSTEM-owned. Edit a temp copy, validate, then replace."
  buh_e
  buh_step2    "Copy to editable location:"
  buh_code     "Copy-Item ${BUBC_windows_sshd_config} \$env:TEMP\\sshd_config"
  buh_e
  buh_step2    "Edit the copy:"
  buh_code     "notepad \$env:TEMP\\sshd_config"
  buh_line     "Set these directives (add if absent):"
  buh_code     "PasswordAuthentication no"
  buh_code     "PubkeyAuthentication yes"
  buh_code     "PermitEmptyPasswords no"
  buh_warn     "Do NOT add UsePAM or ChallengeResponseAuthentication — Windows"
  buh_warn     "OpenSSH rejects unrecognized directives and sshd fails to start."
  buh_e
  buh_step2    "Validate the edited copy (silence means valid):"
  buh_code     "sshd -t -f \$env:TEMP\\sshd_config"
  buh_e
  buh_step2    "Replace the original:"
  buh_code     "Copy-Item \$env:TEMP\\sshd_config ${BUBC_windows_sshd_config} -Force"
  buh_e

  buh_step1    "Place the Operator Admin Pubkey:"
  buh_line     "File: ${z_admin_keys_yelp}"
  buh_line     "Create the file if absent. Each line is one pubkey for an"
  buh_line     "administrative user (BuiltIn\\Administrators or equivalent)."
  buh_e
  buh_line     "Paste exactly the bare contents of the operator's station admin"
  buh_line     "public key (e.g., ~/.ssh/id_ed25519.pub) on its own line."
  buh_warn     "NO command= prefix. Garrison rewrites this file on first run"
  buh_warn     "with the shell-letter command= directive (b/c/w). Adding one"
  buh_warn     "manually is forbidden — garrison will overwrite it anyway."
  buh_e

  buh_step1    "Lock Down Admin Keys File ACLs:"
  buh_line     "Windows OpenSSH refuses to read administrators_authorized_keys"
  buh_line     "unless ownership is restricted to Administrators and SYSTEM."
  buh_code     "icacls \"${BUBC_windows_admin_auth_keys}\" /inheritance:r"
  buh_code     "icacls \"${BUBC_windows_admin_auth_keys}\" /grant \"Administrators:F\""
  buh_code     "icacls \"${BUBC_windows_admin_auth_keys}\" /grant \"SYSTEM:F\""
  buh_e

  buh_step1    "Restart sshd to Apply Config:"
  buh_code     "Restart-Service sshd"
  buh_e

  buh_step1    "Verify Key-Only Auth from the Operator Station:"
  buh_line     "From the operator's station (not the Windows host):"
  buh_code     "ssh -i ~/.ssh/<admin-privkey> <admin-user>@<windows-host> whoami"
  buh_line     "Expect: the admin username printed back. Key auth succeeded."
  buh_e
  buh_line     "Symptoms and remedies:"
  buh_line     "- 'Permission denied (publickey)': admin pubkey absent, malformed,"
  buh_line     "  or ACLs on administrators_authorized_keys still inherited."
  buh_line     "- Password prompt appears: PasswordAuthentication directive not"
  buh_line     "  yet applied; revisit Step 5 and confirm sshd was restarted."
  buh_line     "- 'Connection refused': firewall rule absent or sshd not running."
}

zbuhj_render_post_bootstrap() {
  buh_section  "After Bootstrap — Run Garrison"
  buh_line     "With admin trust established, run garrison for the chosen"
  buh_line     "workload shell. Garrison destroys any prior workload account"
  buh_line     "on the node and provisions a fresh one as the project-wide"
  buh_line     "convention name carried in BURC_WORKLOAD_USER."
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

  buc_doc_brief "Display jurisdiction handbook landing + admin SSH bootstrap procedures"
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
