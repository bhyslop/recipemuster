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

######################################################################
# Internal: Garrison helpers

zbujb_assert_shell_letter() {
  local z_letter="${1:-}"
  case "${z_letter}" in
    b|c|w) ;;
    *) buc_die "Invalid shell-letter (expected b/c/w): '${z_letter}'" ;;
  esac
}

# zbujb_garrison_assert_platform LETTER -- assert BUJB_RESOLVED_PLATFORM
# matches the shell-letter's required platform set.
zbujb_garrison_assert_platform() {
  zbujb_sentinel
  local z_letter="${1:-}"
  zbujb_assert_shell_letter "${z_letter}"
  case "${z_letter}" in
    b)
      case "${BUJB_RESOLVED_PLATFORM}" in
        bubep_linux|bubep_mac) ;;
        *) buc_die "garrison-b requires bubep_linux or bubep_mac, got '${BUJB_RESOLVED_PLATFORM}'" ;;
      esac
      ;;
    c|w)
      test "${BUJB_RESOLVED_PLATFORM}" = "bubep_windows" \
        || buc_die "garrison-${z_letter} requires bubep_windows, got '${BUJB_RESOLVED_PLATFORM}'"
      ;;
  esac
}

# zbujb_workload_home LETTER -- echo the absolute workload home path on the
# remote node for the given shell-letter.
zbujb_workload_home() {
  zbujb_sentinel
  local z_letter="${1:-}"
  local z_wlu="${BUJB_RESOLVED_WORKLOAD_USER}"
  case "${z_letter}" in
    b)
      case "${BUJB_RESOLVED_PLATFORM}" in
        bubep_linux) echo "/home/${z_wlu}" ;;
        bubep_mac)   echo "/Users/${z_wlu}" ;;
        *)           buc_die "zbujb_workload_home: unsupported platform '${BUJB_RESOLVED_PLATFORM}'" ;;
      esac
      ;;
    c|w) echo "/home/${z_wlu}" ;;
    *)   buc_die "zbujb_workload_home: invalid letter '${z_letter}'" ;;
  esac
}

# zbujb_admin_exec LETTER -- read a bash script from stdin and execute it on
# the remote node as the privileged user, wrapped in the shell-letter-
# appropriate invocation (native bash for b; Cygwin's bash --login for c;
# wsl.exe to BUJB_wsl_distribution as root for w).
zbujb_admin_exec() {
  zbujb_sentinel
  local z_letter="${1:-}"
  zbujb_assert_shell_letter "${z_letter}"

  local z_remote_invoker
  case "${z_letter}" in
    b) z_remote_invoker='bash -s' ;;
    c) z_remote_invoker='C:/cygwin64/bin/bash --login -s' ;;
    w) z_remote_invoker="wsl.exe --distribution ${BUJB_wsl_distribution} --user root bash -s" ;;
  esac

  ssh -i "${BUJB_RESOLVED_PRIVILEGED_KEY_FILE}"     \
      -o IdentitiesOnly=yes                         \
      -o BatchMode=yes                              \
      -o StrictHostKeyChecking=accept-new           \
      -o ConnectTimeout=15                          \
      "${BUJB_RESOLVED_PRIVILEGED_USER}@${BUJB_RESOLVED_HOST}" \
      "${z_remote_invoker}"
}

######################################################################
# Internal: Garrison steps (6-step ceremony per BUSJG{B,C,W})

# Step 1 -- open admin SSH (test reachability under key-only auth).
zbujb_garrison_step1_admin_open() {
  local z_letter="${1:-}"
  buc_step "  [1/6] Open admin SSH (${BUJB_RESOLVED_PRIVILEGED_USER}@${BUJB_RESOLVED_HOST})"

  local z_exit=0
  zbujb_admin_exec "${z_letter}" <<<'exit 0' || z_exit=$?
  if test "${z_exit}" -ne 0; then
    case "${BUJB_RESOLVED_PLATFORM}" in
      bubep_windows)
        buc_die "Admin SSH failed (exit ${z_exit}). Run fenestrate first: tt/buw-jpF.Fenestrate.sh ${BUJB_RESOLVED_VICEROYALTY}"
        ;;
      *)
        buc_die "Admin SSH failed (exit ${z_exit}). Place admin pubkey via 'ssh-copy-id -i ${BUJB_RESOLVED_PRIVILEGED_KEY_FILE}.pub ${BUJB_RESOLVED_PRIVILEGED_USER}@${BUJB_RESOLVED_HOST}' first."
        ;;
    esac
  fi
}

# Step 2 -- destroy any existing workload account + home.
zbujb_garrison_step2_destroy() {
  local z_letter="${1:-}"
  local z_wlu="${BUJB_RESOLVED_WORKLOAD_USER}"
  local z_wlhome
  z_wlhome=$(zbujb_workload_home "${z_letter}")
  buc_step "  [2/6] Destroy workload (${z_wlu})"

  case "${z_letter}" in
    b)
      zbujb_admin_exec b <<SCRIPT
set -uo pipefail
sudo -n userdel -r '${z_wlu}' 2>/dev/null || true
sudo -n rm -rf '${z_wlhome}' 2>/dev/null || true
SCRIPT
      ;;
    c)
      # Cygwin workload is a Windows local user; delete via net.exe then
      # purge the Cygwin home directory.
      zbujb_admin_exec c <<SCRIPT
set -uo pipefail
net.exe user '${z_wlu}' /delete > /dev/null 2>&1 || true
rm -rf '${z_wlhome}'                              2>/dev/null || true
rm -rf '/cygdrive/c/cygwin64/home/${z_wlu}'       2>/dev/null || true
SCRIPT
      ;;
    w)
      zbujb_admin_exec w <<SCRIPT
set -uo pipefail
userdel -r '${z_wlu}' 2>/dev/null || true
rm -rf '${z_wlhome}'  2>/dev/null || true
SCRIPT
      ;;
  esac
}

# Step 3 -- create the workload account fresh, ssh-only, no privilege.
zbujb_garrison_step3_create() {
  local z_letter="${1:-}"
  local z_wlu="${BUJB_RESOLVED_WORKLOAD_USER}"
  buc_step "  [3/6] Create workload (${z_wlu})"

  case "${z_letter}" in
    b)
      case "${BUJB_RESOLVED_PLATFORM}" in
        bubep_linux)
          zbujb_admin_exec b <<SCRIPT
set -euo pipefail
sudo -n useradd --create-home --shell /bin/bash '${z_wlu}'
sudo -n passwd  --lock '${z_wlu}'
SCRIPT
          ;;
        bubep_mac)
          # Mac uses dscl/sysadminctl; left for in-environment refinement.
          # Operator may need to seat a more idiomatic primary group ID.
          zbujb_admin_exec b <<SCRIPT
set -euo pipefail
sudo -n sysadminctl -addUser '${z_wlu}' -roleAccount
sudo -n dscl . -create '/Users/${z_wlu}' UserShell /bin/bash
SCRIPT
          ;;
      esac
      ;;
    c)
      # Cygwin reflects Windows user accounts; mint via net.exe with a
      # disabled-password posture (we want ssh-key-only).
      zbujb_admin_exec c <<SCRIPT
set -euo pipefail
net.exe user '${z_wlu}' /add /passwordreq:no /active:yes > /dev/null
mkpasswd -l -u '${z_wlu}' >> /etc/passwd
mkdir -p '/home/${z_wlu}'
chown -R '${z_wlu}' '/home/${z_wlu}'
SCRIPT
      ;;
    w)
      zbujb_admin_exec w <<SCRIPT
set -euo pipefail
useradd --create-home --shell /bin/bash '${z_wlu}'
passwd  --lock '${z_wlu}'
SCRIPT
      ;;
  esac
}

# Step 4 -- write workload authorized_keys with the shell-letter command=
# directive and the workload pubkey (derived locally from the privkey).
zbujb_garrison_step4_place_trust() {
  local z_letter="${1:-}"
  local z_wlu="${BUJB_RESOLVED_WORKLOAD_USER}"
  local z_wlhome
  z_wlhome=$(zbujb_workload_home "${z_letter}")
  buc_step "  [4/6] Place workload trust (${z_wlhome}/.ssh/authorized_keys)"

  local z_command_directive
  z_command_directive=$(bujb_command_for "${z_letter}")

  local z_pubkey
  z_pubkey=$(ssh-keygen -y -P '' -f "${BUJB_RESOLVED_WORKLOAD_KEY_FILE}") \
    || buc_die "ssh-keygen -y failed for workload key: ${BUJB_RESOLVED_WORKLOAD_KEY_FILE}"

  local z_authkeys_line="${z_command_directive} ${z_pubkey}"

  case "${z_letter}" in
    b|w)
      zbujb_admin_exec "${z_letter}" <<SCRIPT
set -euo pipefail
$([ "${z_letter}" = "b" ] && echo "sudo -n ")mkdir -p   '${z_wlhome}/.ssh'
$([ "${z_letter}" = "b" ] && echo "sudo -n ")chmod 700  '${z_wlhome}/.ssh'
echo '${z_authkeys_line}' | $([ "${z_letter}" = "b" ] && echo "sudo -n ")tee '${z_wlhome}/.ssh/authorized_keys' > /dev/null
$([ "${z_letter}" = "b" ] && echo "sudo -n ")chmod 600  '${z_wlhome}/.ssh/authorized_keys'
$([ "${z_letter}" = "b" ] && echo "sudo -n ")chown -R '${z_wlu}:${z_wlu}' '${z_wlhome}/.ssh'
SCRIPT
      ;;
    c)
      # In Cygwin, file ownership/permissions interplay with NTFS ACLs.
      # mkdir/chmod/chown are Cygwin-mediated; run as the admin user
      # without sudo (Windows admins already have privilege).
      zbujb_admin_exec c <<SCRIPT
set -euo pipefail
mkdir -p   '${z_wlhome}/.ssh'
chmod 700  '${z_wlhome}/.ssh'
echo '${z_authkeys_line}' > '${z_wlhome}/.ssh/authorized_keys'
chmod 600  '${z_wlhome}/.ssh/authorized_keys'
chown -R '${z_wlu}' '${z_wlhome}/.ssh'
SCRIPT
      ;;
  esac
}

# Step 5 -- copy workload privkey to the remote at the shell-letter's
# hardcoded destination path, with workload ownership and 0600 mode.
zbujb_garrison_step5_plant_key() {
  local z_letter="${1:-}"
  local z_wlu="${BUJB_RESOLVED_WORKLOAD_USER}"
  local z_wlhome
  z_wlhome=$(zbujb_workload_home "${z_letter}")
  local z_keypath
  z_keypath=$(bujb_workload_keypath_for "${z_letter}")
  local z_target="${z_wlhome}/${z_keypath}"
  buc_step "  [5/6] Plant workload privkey (${z_target})"

  local z_key_b64
  z_key_b64=$(base64 < "${BUJB_RESOLVED_WORKLOAD_KEY_FILE}") \
    || buc_die "base64 encode failed for workload key: ${BUJB_RESOLVED_WORKLOAD_KEY_FILE}"

  case "${z_letter}" in
    b|w)
      local z_sudo=""
      test "${z_letter}" = "b" && z_sudo="sudo -n"
      zbujb_admin_exec "${z_letter}" <<SCRIPT
set -euo pipefail
ztmp=\$(mktemp)
trap 'rm -f "\${ztmp}"' EXIT
echo '${z_key_b64}' | base64 -d > "\${ztmp}"
${z_sudo} mkdir -p   '$(dirname "${z_target}")'
${z_sudo} install -m 600 -o '${z_wlu}' -g '${z_wlu}' "\${ztmp}" '${z_target}'
SCRIPT
      ;;
    c)
      zbujb_admin_exec c <<SCRIPT
set -euo pipefail
ztmp=\$(mktemp)
trap 'rm -f "\${ztmp}"' EXIT
echo '${z_key_b64}' | base64 -d > "\${ztmp}"
mkdir -p   '$(dirname "${z_target}")'
cp "\${ztmp}" '${z_target}'
chmod 600  '${z_target}'
chown      '${z_wlu}' '${z_target}'
SCRIPT
      ;;
  esac
}

# Step 6 -- workload-side round-trip validation (knock).
zbujb_garrison_step6_validate() {
  local z_letter="${1:-}"
  buc_step "  [6/6] Validate workload round-trip"

  local z_exit=0
  ssh -i "${BUJB_RESOLVED_WORKLOAD_KEY_FILE}"     \
      -o IdentitiesOnly=yes                       \
      -o BatchMode=yes                            \
      -o StrictHostKeyChecking=accept-new         \
      -o ConnectTimeout=10                        \
      "${BUJB_RESOLVED_WORKLOAD_USER}@${BUJB_RESOLVED_HOST}" \
      true                                        \
    || z_exit=$?

  test "${z_exit}" -eq 0 \
    || buc_die "Workload round-trip failed (ssh exit ${z_exit}); the new account did not accept its own key."
}

######################################################################
# Public: Garrison ceremony

# bujb_garrison LETTER -- run the 6-step garrison ceremony for shell-letter
# b, c, or w. Caller must have invoked bujb_resolve_investiture beforehand.
bujb_garrison() {
  zbujb_sentinel
  test "${ZBUJB_RESOLVED:-}" = "1" \
    || buc_die "bujb_garrison: call bujb_resolve_investiture first"

  local z_letter="${1:-}"
  zbujb_garrison_assert_platform "${z_letter}"

  buc_step "Garrison-${z_letter}: ${BUJB_RESOLVED_VICEROYALTY} (${BUJB_RESOLVED_HOST})"

  zbujb_garrison_step1_admin_open    "${z_letter}"
  zbujb_garrison_step2_destroy       "${z_letter}"
  zbujb_garrison_step3_create        "${z_letter}"
  zbujb_garrison_step4_place_trust   "${z_letter}"
  zbujb_garrison_step5_plant_key     "${z_letter}"
  zbujb_garrison_step6_validate      "${z_letter}"

  buc_step "Garrison-${z_letter} succeeded"
}

# eof
