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
# BUJB Jurisdiction Module - Implementation seat for fenestrate + garrison
#
# BCG-compliant module housing both BUS0 jurisdiction verbs (fenestrate,
# garrison) and the contracts not expressed in regime data:
# the three shell-letter -> command= directive mappings (b/c/w),
# the workload privkey destination paths on the remote per shell-letter,
# the canonical WSL distribution name, and the Windows OpenSSH sshd_config
# hardening directive set.
#
# Sub-letter b in bujb signals the bash-format implementation. A future
# PowerShell sibling would mint as bujp_jurisdiction.ps1.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZBUJB_SOURCED:-}" || buc_die "Module bujb multiply sourced - check sourcing hierarchy"
ZBUJB_SOURCED=1

######################################################################
# Internal Functions (zbujb_*)

zbujb_kindle() {
  test -z "${ZBUJB_KINDLED:-}" || buc_die "Module bujb already kindled"

  # Shell-letter -> command= directive mappings.
  # Forced commands routed through SSH_ORIGINAL_COMMAND keep workload account
  # behaviour pinned to the chosen shell regardless of what the SSH client
  # requests. Locked spec content; mirrored in BUSJG{B,C,W}.
  BUJB_command_b='command="/bin/bash -lc \"$SSH_ORIGINAL_COMMAND\"",no-port-forwarding,no-X11-forwarding,no-agent-forwarding'
  BUJB_command_c='command="C:/cygwin64/bin/bash --login -c \"$SSH_ORIGINAL_COMMAND\"",no-port-forwarding,no-X11-forwarding,no-agent-forwarding'
  BUJB_command_w='command="C:/Windows/System32/wsl.exe --distribution rbtww-main --exec /bin/bash -lc \"$SSH_ORIGINAL_COMMAND\"",no-port-forwarding,no-X11-forwarding,no-agent-forwarding'

  # Shell-letter -> workload privkey destination path on the remote
  # (relative to the workload account home directory).
  BUJB_workload_keypath_b='.ssh/id_ed25519'
  BUJB_workload_keypath_c='.ssh/id_ed25519'
  BUJB_workload_keypath_w='.ssh/id_ed25519'

  # Canonical WSL distribution name reached by garrison-w.
  BUJB_wsl_distribution='rbtww-main'

  # Windows OpenSSH sshd_config hardening directive set written by
  # fenestrate phase 1. Newline-joined; each directive is asserted by
  # bash-side parse after PowerShell Get-Content returns the raw bytes.
  BUJB_sshd_hardening='PubkeyAuthentication yes
PasswordAuthentication no
PermitEmptyPasswords no'

  readonly BUJB_command_b BUJB_command_c BUJB_command_w
  readonly BUJB_workload_keypath_b BUJB_workload_keypath_c BUJB_workload_keypath_w
  readonly BUJB_wsl_distribution
  readonly BUJB_sshd_hardening

  readonly ZBUJB_KINDLED=1
}

zbujb_sentinel() {
  test "${ZBUJB_KINDLED:-}" = "1" || buc_die "Module bujb not kindled - call zbujb_kindle first"
}

# zbujb_check_key_file PATH VARNAME -- validate SSH private key at PATH:
# exists, mode 0600, parseable + unencrypted (ssh-keygen -y dry-load with
# empty passphrase). Diagnostic uses VARNAME as the field label.
zbujb_check_key_file() {
  local z_path="${1:-}"
  local z_varname="${2:-}"

  test -n "${z_path}"   || buc_die "${z_varname}: empty path"
  test -f "${z_path}"   || buc_die "${z_varname}: file not found: ${z_path}"

  local z_mode=""
  if z_mode=$(stat -f '%A' "${z_path}" 2>/dev/null); then
    : # macOS / BSD stat
  elif z_mode=$(stat -c '%a' "${z_path}" 2>/dev/null); then
    : # GNU/Linux stat
  else
    buc_die "${z_varname}: cannot stat ${z_path}"
  fi
  test "${z_mode}" = "600" || buc_die "${z_varname}: mode must be 0600, got ${z_mode}: ${z_path}"

  ssh-keygen -y -P '' -f "${z_path}" >/dev/null 2>&1 \
    || buc_die "${z_varname}: ssh-keygen -y dry-load failed (passphrase-protected or malformed): ${z_path}"
}

######################################################################
# Public Functions (bujb_*)

# bujb_resolve_investiture -- sole load-and-cross-validate entrypoint.
#
# Preconditions: BURC, BURN, BURP regimes already kindled and enforced
# (the bujb_cli furnish handles this for tabtarget callers; the cli's
# BURP and BURN sourcing also implicitly cross-validates that
# BURP_VICEROYALTY refers to a registered BURN profile by file presence).
#
# Behaviour: validates BURP_PRIVILEGED_KEY_FILE and BURP_WORKLOAD_KEY_FILE
# (exist, mode 0600, ssh-keygen -y dry-load proves parseable + unencrypted),
# then publishes BUJB_RESOLVED_* globals as readonly. Single-call-per-process;
# subsequent calls die.
bujb_resolve_investiture() {
  zbujb_sentinel
  zburc_sentinel
  zburn_sentinel
  zburp_sentinel

  test -z "${ZBUJB_RESOLVED:-}" \
    || buc_die "bujb_resolve_investiture already called - single-call-per-process"

  zbujb_check_key_file "${BURP_PRIVILEGED_KEY_FILE}" "BURP_PRIVILEGED_KEY_FILE"
  zbujb_check_key_file "${BURP_WORKLOAD_KEY_FILE}"   "BURP_WORKLOAD_KEY_FILE"

  BUJB_RESOLVED_VICEROYALTY="${BURP_VICEROYALTY}"
  BUJB_RESOLVED_HOST="${BURN_HOST}"
  BUJB_RESOLVED_PLATFORM="${BURN_PLATFORM}"
  BUJB_RESOLVED_PRIVILEGED_USER="${BURP_PRIVILEGED_USER}"
  BUJB_RESOLVED_PRIVILEGED_KEY_FILE="${BURP_PRIVILEGED_KEY_FILE}"
  BUJB_RESOLVED_WORKLOAD_KEY_FILE="${BURP_WORKLOAD_KEY_FILE}"
  BUJB_RESOLVED_WORKLOAD_USER="${BURC_WORKLOAD_USER}"

  readonly BUJB_RESOLVED_VICEROYALTY
  readonly BUJB_RESOLVED_HOST
  readonly BUJB_RESOLVED_PLATFORM
  readonly BUJB_RESOLVED_PRIVILEGED_USER
  readonly BUJB_RESOLVED_PRIVILEGED_KEY_FILE
  readonly BUJB_RESOLVED_WORKLOAD_KEY_FILE
  readonly BUJB_RESOLVED_WORKLOAD_USER

  readonly ZBUJB_RESOLVED=1
}

# bujb_command_for SHELL_LETTER -- emit the workload authorized_keys
# command= directive for shell-letter b, c, or w (echoed to stdout).
bujb_command_for() {
  zbujb_sentinel
  local z_letter="${1:-}"
  case "${z_letter}" in
    b) echo "${BUJB_command_b}" ;;
    c) echo "${BUJB_command_c}" ;;
    w) echo "${BUJB_command_w}" ;;
    *) buc_die "bujb_command_for: invalid shell-letter (expected b/c/w): '${z_letter}'" ;;
  esac
}

# bujb_workload_keypath_for SHELL_LETTER -- emit the workload privkey
# remote destination path (relative to workload home) for b, c, or w.
bujb_workload_keypath_for() {
  zbujb_sentinel
  local z_letter="${1:-}"
  case "${z_letter}" in
    b) echo "${BUJB_workload_keypath_b}" ;;
    c) echo "${BUJB_workload_keypath_c}" ;;
    w) echo "${BUJB_workload_keypath_w}" ;;
    *) buc_die "bujb_workload_keypath_for: invalid shell-letter (expected b/c/w): '${z_letter}'" ;;
  esac
}

# eof
