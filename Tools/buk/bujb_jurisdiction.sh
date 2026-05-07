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

# Tinder constants (pure string literals — available at source time)

# Shell-letter -> command= directive mappings.
# Forced commands routed through SSH_ORIGINAL_COMMAND keep workload account
# behaviour pinned to the chosen shell regardless of what the SSH client
# requests. Locked spec content; mirrored in BUSJG{B,C,W}.
BUJB_command_b='command="/bin/bash -lc \"$SSH_ORIGINAL_COMMAND\"",no-port-forwarding,no-X11-forwarding,no-agent-forwarding'
BUJB_command_c='command="C:/cygwin64/bin/bash --login -c \"$SSH_ORIGINAL_COMMAND\"",no-port-forwarding,no-X11-forwarding,no-agent-forwarding'
BUJB_command_w='command="C:/Windows/System32/wsl.exe --distribution rbtww-main --user ${BUJB_workload_user} --exec /bin/bash -lc \"$SSH_ORIGINAL_COMMAND\"",no-port-forwarding,no-X11-forwarding,no-agent-forwarding'

# Shell-letter -> workload privkey destination path on the remote
# (relative to the workload account home directory).
BUJB_workload_keypath_b='.ssh/id_ed25519'
BUJB_workload_keypath_c='.ssh/id_ed25519'
BUJB_workload_keypath_w='.ssh/id_ed25519'

# Canonical WSL distribution name reached by garrison-w.
BUJB_wsl_distribution='rbtww-main'

# Canonical workload OS user name provisioned on every node by garrison.
# Project-wide convention; substituted into BUJB_command_w via
# bujb_command_for_capture and consumed by garrison/cli display strings.
BUJB_workload_user='bujuw_user'

# Windows OpenSSH sshd_config hardening directive set written by
# fenestrate phase 1. Newline-joined; each directive is asserted by
# bash-side parse after PowerShell Get-Content returns the raw bytes.
BUJB_sshd_hardening='PubkeyAuthentication yes
PasswordAuthentication no
PermitEmptyPasswords no'

######################################################################
# Internal Functions (zbujb_*)

zbujb_kindle() {
  test -z "${ZBUJB_KINDLED:-}" || buc_die "Module bujb already kindled"

  # Fenestrate temp file paths — ssh stdout/stderr captured here so
  # callers parse from disk (no `$(ssh ...)` capture).
  readonly ZBUJB_FENESTRATE_PHASE1_STDOUT="${BURD_TEMP_DIR}/bujb_fenestrate_phase1_stdout.txt"
  readonly ZBUJB_FENESTRATE_PHASE1_STDERR="${BURD_TEMP_DIR}/bujb_fenestrate_phase1_stderr.txt"
  readonly ZBUJB_FENESTRATE_RESTART_STDOUT="${BURD_TEMP_DIR}/bujb_fenestrate_restart_stdout.txt"
  readonly ZBUJB_FENESTRATE_RESTART_STDERR="${BURD_TEMP_DIR}/bujb_fenestrate_restart_stderr.txt"
  readonly ZBUJB_FENESTRATE_PHASE2_STDOUT="${BURD_TEMP_DIR}/bujb_fenestrate_phase2_stdout.txt"
  readonly ZBUJB_FENESTRATE_PHASE2_STDERR="${BURD_TEMP_DIR}/bujb_fenestrate_phase2_stderr.txt"

  # stat captures (per-key-slot for parallel forensics on resolve).
  readonly ZBUJB_STAT_STDOUT_PRIV="${BURD_TEMP_DIR}/bujb_stat_priv_stdout.txt"
  readonly ZBUJB_STAT_STDERR_PRIV="${BURD_TEMP_DIR}/bujb_stat_priv_stderr.txt"
  readonly ZBUJB_STAT_STDOUT_WORK="${BURD_TEMP_DIR}/bujb_stat_work_stdout.txt"
  readonly ZBUJB_STAT_STDERR_WORK="${BURD_TEMP_DIR}/bujb_stat_work_stderr.txt"

  # ssh-keygen dry-load captures (resolve-time, per slot).
  readonly ZBUJB_DRYLOAD_STDOUT_PRIV="${BURD_TEMP_DIR}/bujb_dryload_priv_stdout.txt"
  readonly ZBUJB_DRYLOAD_STDERR_PRIV="${BURD_TEMP_DIR}/bujb_dryload_priv_stderr.txt"
  readonly ZBUJB_DRYLOAD_STDOUT_WORK="${BURD_TEMP_DIR}/bujb_dryload_work_stdout.txt"
  readonly ZBUJB_DRYLOAD_STDERR_WORK="${BURD_TEMP_DIR}/bujb_dryload_work_stderr.txt"

  # ssh-keygen pubkey-emit captures (garrison step4 = workload pubkey,
  # fenestrate phase1 = admin pubkey).
  readonly ZBUJB_PUBKEY_STDOUT_WORK="${BURD_TEMP_DIR}/bujb_pubkey_work_stdout.txt"
  readonly ZBUJB_PUBKEY_STDERR_WORK="${BURD_TEMP_DIR}/bujb_pubkey_work_stderr.txt"
  readonly ZBUJB_PUBKEY_STDOUT_PRIV="${BURD_TEMP_DIR}/bujb_pubkey_priv_stdout.txt"
  readonly ZBUJB_PUBKEY_STDERR_PRIV="${BURD_TEMP_DIR}/bujb_pubkey_priv_stderr.txt"

  # base64 capture (garrison step5 = workload privkey b64-encoded for
  # heredoc transport to remote).
  readonly ZBUJB_KEY_B64_STDOUT="${BURD_TEMP_DIR}/bujb_key_b64_stdout.txt"
  readonly ZBUJB_KEY_B64_STDERR="${BURD_TEMP_DIR}/bujb_key_b64_stderr.txt"

  # WSL distribution preflight captures (garrison-w only).
  readonly ZBUJB_WSL_PREFLIGHT_STDOUT="${BURD_TEMP_DIR}/bujb_wsl_preflight_stdout.txt"
  readonly ZBUJB_WSL_PREFLIGHT_STDERR="${BURD_TEMP_DIR}/bujb_wsl_preflight_stderr.txt"

  readonly ZBUJB_KINDLED=1
}

zbujb_sentinel() {
  test "${ZBUJB_KINDLED:-}" = "1" || buc_die "Module bujb not kindled - call zbujb_kindle first"
}

# zbujb_check_key_file PATH VARNAME SLOT -- validate SSH private key at PATH:
# exists, mode 0600, parseable + unencrypted (ssh-keygen -y dry-load with
# empty passphrase). Diagnostic uses VARNAME as the field label. SLOT
# (priv|work) selects the kindle temp-file pair used for stat + dry-load
# stdout/stderr capture so failures preserve forensic evidence per BCG.
zbujb_check_key_file() {
  local z_path="${1:-}"
  local z_varname="${2:-}"
  local z_slot="${3:-}"

  test -n "${z_path}"    || buc_die "${z_varname}: empty path"
  test -f "${z_path}"    || buc_die "${z_varname}: file not found: ${z_path}"

  local z_stat_stdout="" z_stat_stderr="" z_dry_stdout="" z_dry_stderr=""
  case "${z_slot}" in
    priv)
      z_stat_stdout="${ZBUJB_STAT_STDOUT_PRIV}"
      z_stat_stderr="${ZBUJB_STAT_STDERR_PRIV}"
      z_dry_stdout="${ZBUJB_DRYLOAD_STDOUT_PRIV}"
      z_dry_stderr="${ZBUJB_DRYLOAD_STDERR_PRIV}"
      ;;
    work)
      z_stat_stdout="${ZBUJB_STAT_STDOUT_WORK}"
      z_stat_stderr="${ZBUJB_STAT_STDERR_WORK}"
      z_dry_stdout="${ZBUJB_DRYLOAD_STDOUT_WORK}"
      z_dry_stderr="${ZBUJB_DRYLOAD_STDERR_WORK}"
      ;;
    *)
      buc_die "zbujb_check_key_file: invalid slot (expected priv|work): '${z_slot}'"
      ;;
  esac

  # BSD stat first (Mac), GNU stat fallback (Linux). The failed flavor's
  # stderr is overwritten by the successful flavor's invocation; on total
  # failure z_stat_stderr holds the GNU-attempt error.
  if stat -f '%A' "${z_path}" > "${z_stat_stdout}" 2> "${z_stat_stderr}"; then
    : # macOS / BSD stat succeeded
  elif stat -c '%a' "${z_path}" > "${z_stat_stdout}" 2> "${z_stat_stderr}"; then
    : # GNU/Linux stat succeeded
  else
    buc_die "${z_varname}: cannot stat ${z_path} — see ${z_stat_stderr}"
  fi

  local z_mode
  z_mode=$(<"${z_stat_stdout}")
  z_mode="${z_mode//$'\n'/}"
  test "${z_mode}" = "600" \
    || buc_die "${z_varname}: mode must be 0600, got ${z_mode}: ${z_path}"

  ssh-keygen -y -P '' -f "${z_path}" \
      > "${z_dry_stdout}" \
      2> "${z_dry_stderr}" \
    || buc_die "${z_varname}: ssh-keygen -y dry-load failed (passphrase-protected or malformed): ${z_path} — see ${z_dry_stderr}"
}

######################################################################
# Public Functions (bujb_*)

# bujb_resolve_investiture -- sole runtime cross-validation entrypoint.
#
# Preconditions: BURC, BURN, BURP regimes already kindled and enforced
# (the bujb_cli furnish handles this for tabtarget callers; the cli's
# BURP and BURN sourcing also implicitly cross-validates that the
# investiture name (BUZ_FOLIO) equals a registered investiture —
# enforcement is by file presence of the matching BURN profile).
#
# Behaviour: validates BURP_PRIVILEGED_KEY_FILE and BURP_WORKLOAD_KEY_FILE
# (exist, mode 0600, ssh-keygen -y dry-load proves parseable + unencrypted) —
# the validation regime enforce cannot do because it requires running
# ssh-keygen against the private key. After validation, sets ZBUJB_RESOLVED=1
# as the I-have-been-validated sentinel. Callers reference the regime vars
# directly (BURP_*, BURN_*, BURC_*); no separate published namespace.
# Single-call-per-process; subsequent calls die.
bujb_resolve_investiture() {
  zbujb_sentinel
  zburc_sentinel
  zburn_sentinel
  zburp_sentinel

  test -z "${ZBUJB_RESOLVED:-}" \
    || buc_die "bujb_resolve_investiture already called - single-call-per-process"

  zbujb_check_key_file "${BURP_PRIVILEGED_KEY_FILE}" "BURP_PRIVILEGED_KEY_FILE" priv
  zbujb_check_key_file "${BURP_WORKLOAD_KEY_FILE}"   "BURP_WORKLOAD_KEY_FILE"   work

  readonly ZBUJB_RESOLVED=1
}

# bujb_command_for_capture SHELL_LETTER -- emit the workload authorized_keys
# command= directive for shell-letter b, c, or w. _capture form so callers
# may use `$()` per BCG line 502; returns 1 on invalid letter, callers must
# `|| buc_die`.
bujb_command_for_capture() {
  zbujb_sentinel
  local z_letter="${1:-}"
  case "${z_letter}" in
    b) echo "${BUJB_command_b}" ;;
    c) echo "${BUJB_command_c}" ;;
    w) echo "${BUJB_command_w//\$\{BUJB_workload_user\}/${BUJB_workload_user}}" ;;
    *) return 1 ;;
  esac
}

# bujb_workload_keypath_for_capture SHELL_LETTER -- emit the workload privkey
# remote destination path (relative to workload home) for b, c, or w.
# Returns 1 on invalid letter; callers must `|| buc_die`.
bujb_workload_keypath_for_capture() {
  zbujb_sentinel
  local z_letter="${1:-}"
  case "${z_letter}" in
    b) echo "${BUJB_workload_keypath_b}" ;;
    c) echo "${BUJB_workload_keypath_c}" ;;
    w) echo "${BUJB_workload_keypath_w}" ;;
    *) return 1 ;;
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

# zbujb_garrison_assert_platform LETTER -- assert BURN_PLATFORM
# matches the shell-letter's required platform set.
zbujb_garrison_assert_platform() {
  zbujb_sentinel
  local z_letter="${1:-}"
  zbujb_assert_shell_letter "${z_letter}"
  case "${z_letter}" in
    b)
      case "${BURN_PLATFORM}" in
        bubep_linux|bubep_mac) ;;
        *) buc_die "garrison-b requires bubep_linux or bubep_mac, got '${BURN_PLATFORM}'" ;;
      esac
      ;;
    c|w)
      test "${BURN_PLATFORM}" = "bubep_windows" \
        || buc_die "garrison-${z_letter} requires bubep_windows, got '${BURN_PLATFORM}'"
      ;;
  esac
}

# zbujb_workload_home_capture LETTER -- emit the absolute workload home path
# on the remote node for the given shell-letter. _capture form per BCG line
# 502; returns 1 on unsupported platform or invalid letter, callers must
# `|| buc_die`.
zbujb_workload_home_capture() {
  zbujb_sentinel
  local z_letter="${1:-}"
  local z_wlu="${BUJB_workload_user}"
  case "${z_letter}" in
    b)
      case "${BURN_PLATFORM}" in
        bubep_linux) echo "/home/${z_wlu}" ;;
        bubep_mac)   echo "/Users/${z_wlu}" ;;
        *)           return 1 ;;
      esac
      ;;
    c|w) echo "/home/${z_wlu}" ;;
    *)   return 1 ;;
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

  ssh -i "${BURP_PRIVILEGED_KEY_FILE}"     \
      -o IdentitiesOnly=yes                         \
      -o BatchMode=yes                              \
      -o StrictHostKeyChecking=accept-new           \
      -o ConnectTimeout=15                          \
      "${BURP_PRIVILEGED_USER}@${BURN_HOST}" \
      "${z_remote_invoker}"
}

# zbujb_admin_powershell BODY... -- run a PowerShell statement chain on
# the remote node as the privileged user. Prepends discipline prelude:
# $ErrorActionPreference='Stop' (PS errors terminate), $env:WSL_UTF8=1
# (wsl.exe emits UTF-8). Trailing if-check propagates $LASTEXITCODE on
# non-zero. Caller redirects stdout/stderr; returns ssh's exit code.
zbujb_admin_powershell() {
  zbujb_sentinel
  test $# -ge 1 \
    || buc_die "zbujb_admin_powershell: PowerShell command body required"
  local -r z_body="$*"

  ssh -i "${BURP_PRIVILEGED_KEY_FILE}"            \
      -o IdentitiesOnly=yes                       \
      -o BatchMode=yes                             \
      -o StrictHostKeyChecking=accept-new          \
      -o ConnectTimeout=15                         \
      "${BURP_PRIVILEGED_USER}@${BURN_HOST}"       \
      "powershell -NoProfile -Command \"\$ErrorActionPreference = 'Stop'; \$env:WSL_UTF8 = 1; ${z_body}; if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE }\""
}

######################################################################
# Internal: Garrison steps (6-step ceremony per BUSJG{B,C,W})

# Step 1 -- open admin SSH (test reachability under key-only auth).
zbujb_garrison_step1_admin_open() {
  local z_letter="${1:-}"
  buc_step "  [1/6] Open admin SSH (${BURP_PRIVILEGED_USER}@${BURN_HOST})"

  local z_exit=0
  zbujb_admin_exec "${z_letter}" <<<'exit 0' || z_exit=$?
  if test "${z_exit}" -ne 0; then
    case "${BURN_PLATFORM}" in
      bubep_windows)
        buc_die "Admin SSH failed (exit ${z_exit}). Run fenestrate first: tt/buw-jpF.Fenestrate.sh ${BUZ_FOLIO}"
        ;;
      *)
        buc_die "Admin SSH failed (exit ${z_exit}). Place admin pubkey via 'ssh-copy-id -i ${BURP_PRIVILEGED_KEY_FILE}.pub ${BURP_PRIVILEGED_USER}@${BURN_HOST}' first."
        ;;
    esac
  fi
}

# Step 2 -- destroy any existing workload account + home.
zbujb_garrison_step2_destroy() {
  local z_letter="${1:-}"
  local z_wlu="${BUJB_workload_user}"
  local z_wlhome
  z_wlhome=$(zbujb_workload_home_capture "${z_letter}") \
    || buc_die "step2: workload home unresolvable for letter='${z_letter}' platform='${BURN_PLATFORM}'"
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
      # Workload identity is mirrored across two namespaces: a Windows user
      # (SSH auth boundary) and a Linux user inside the WSL distro
      # (execution context). Purge both, idempotent.
      zbujb_admin_exec w <<SCRIPT
set -uo pipefail
net.exe user '${z_wlu}' /delete > /dev/null 2>&1 || true
userdel -r '${z_wlu}'                           2>/dev/null || true
rm -rf '${z_wlhome}'                            2>/dev/null || true
SCRIPT
      ;;
  esac
}

# Step 3 -- create the workload account fresh, ssh-only, no privilege.
zbujb_garrison_step3_create() {
  local z_letter="${1:-}"
  local z_wlu="${BUJB_workload_user}"
  buc_step "  [3/6] Create workload (${z_wlu})"

  case "${z_letter}" in
    b)
      case "${BURN_PLATFORM}" in
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
      # Mirrored identity: net.exe creates the Windows user (SSH auth
      # boundary), useradd creates the Linux user inside WSL (execution
      # context). Same Windows-user shape as the c branch above; the
      # WSL-side useradd layered on top.
      zbujb_admin_exec w <<SCRIPT
set -euo pipefail
net.exe user '${z_wlu}' /add /passwordreq:no /active:yes > /dev/null
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
  local z_wlu="${BUJB_workload_user}"
  local z_wlhome
  z_wlhome=$(zbujb_workload_home_capture "${z_letter}") \
    || buc_die "step4: workload home unresolvable for letter='${z_letter}' platform='${BURN_PLATFORM}'"
  buc_step "  [4/6] Place workload trust (${z_wlhome}/.ssh/authorized_keys)"

  local z_command_directive
  z_command_directive=$(bujb_command_for_capture "${z_letter}") \
    || buc_die "step4: bujb_command_for_capture failed for letter='${z_letter}'"

  ssh-keygen -y -P '' -f "${BURP_WORKLOAD_KEY_FILE}" \
      > "${ZBUJB_PUBKEY_STDOUT_WORK}" \
      2> "${ZBUJB_PUBKEY_STDERR_WORK}" \
    || buc_die "ssh-keygen -y failed for workload key: ${BURP_WORKLOAD_KEY_FILE} — see ${ZBUJB_PUBKEY_STDERR_WORK}"
  local z_pubkey
  z_pubkey=$(<"${ZBUJB_PUBKEY_STDOUT_WORK}")
  z_pubkey="${z_pubkey//$'\n'/}"

  local z_authkeys_line="${z_command_directive} ${z_pubkey}"

  # Precompute sudo prefix locally (bash parameter expansion, no $()) so the
  # heredoc lines don't carry inline `$([ ... ] && echo "sudo -n ")` calls.
  local z_sudo_prefix=""
  test "${z_letter}" = "b" && z_sudo_prefix="sudo -n "

  case "${z_letter}" in
    b)
      zbujb_admin_exec b <<SCRIPT
set -euo pipefail
${z_sudo_prefix}mkdir -p   '${z_wlhome}/.ssh'
${z_sudo_prefix}chmod 700  '${z_wlhome}/.ssh'
echo '${z_authkeys_line}' | ${z_sudo_prefix}tee '${z_wlhome}/.ssh/authorized_keys' > /dev/null
${z_sudo_prefix}chmod 600  '${z_wlhome}/.ssh/authorized_keys'
${z_sudo_prefix}chown -R '${z_wlu}:${z_wlu}' '${z_wlhome}/.ssh'
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
    w)
      # Authorized_keys lives Windows-side so Windows OpenSSH discovers
      # it natively (the SSH auth boundary is the Windows user). From
      # WSL bash the Windows profile is reachable at /mnt/c/Users.
      # Structural shape mirrors the c branch above; the path differs.
      zbujb_admin_exec w <<SCRIPT
set -euo pipefail
mkdir -p   '/mnt/c/Users/${z_wlu}/.ssh'
chmod 700  '/mnt/c/Users/${z_wlu}/.ssh'
echo '${z_authkeys_line}' > '/mnt/c/Users/${z_wlu}/.ssh/authorized_keys'
chmod 600  '/mnt/c/Users/${z_wlu}/.ssh/authorized_keys'
chown -R '${z_wlu}' '/mnt/c/Users/${z_wlu}/.ssh'
SCRIPT
      ;;
  esac
}

# Step 5 -- copy workload privkey to the remote at the shell-letter's
# hardcoded destination path, with workload ownership and 0600 mode.
zbujb_garrison_step5_plant_key() {
  local z_letter="${1:-}"
  local z_wlu="${BUJB_workload_user}"
  local z_wlhome
  z_wlhome=$(zbujb_workload_home_capture "${z_letter}") \
    || buc_die "step5: workload home unresolvable for letter='${z_letter}' platform='${BURN_PLATFORM}'"
  local z_keypath
  z_keypath=$(bujb_workload_keypath_for_capture "${z_letter}") \
    || buc_die "step5: bujb_workload_keypath_for_capture failed for letter='${z_letter}'"
  local z_target="${z_wlhome}/${z_keypath}"
  local z_target_dir="${z_target%/*}"  # parameter expansion replaces dirname
  buc_step "  [5/6] Plant workload privkey (${z_target})"

  openssl enc -base64 -A < "${BURP_WORKLOAD_KEY_FILE}" \
      > "${ZBUJB_KEY_B64_STDOUT}" \
      2> "${ZBUJB_KEY_B64_STDERR}" \
    || buc_die "base64 encode failed for workload key: ${BURP_WORKLOAD_KEY_FILE} — see ${ZBUJB_KEY_B64_STDERR}"
  local z_key_b64
  z_key_b64=$(<"${ZBUJB_KEY_B64_STDOUT}")
  z_key_b64="${z_key_b64//$'\n'/}"

  local z_sudo_prefix=""
  test "${z_letter}" = "b" && z_sudo_prefix="sudo -n "

  case "${z_letter}" in
    b|w)
      zbujb_admin_exec "${z_letter}" <<SCRIPT
set -euo pipefail
ztmp=\$(mktemp)
trap 'rm -f "\${ztmp}"' EXIT
echo '${z_key_b64}' | openssl enc -base64 -d -A > "\${ztmp}"
${z_sudo_prefix}mkdir -p   '${z_target_dir}'
${z_sudo_prefix}install -m 600 -o '${z_wlu}' -g '${z_wlu}' "\${ztmp}" '${z_target}'
SCRIPT
      ;;
    c)
      zbujb_admin_exec c <<SCRIPT
set -euo pipefail
ztmp=\$(mktemp)
trap 'rm -f "\${ztmp}"' EXIT
echo '${z_key_b64}' | openssl enc -base64 -d -A > "\${ztmp}"
mkdir -p   '${z_target_dir}'
cp "\${ztmp}" '${z_target}'
chmod 600  '${z_target}'
chown      '${z_wlu}' '${z_target}'
SCRIPT
      ;;
  esac
}

# zbujb_garrison_w_preflight -- pre-flight for shell-letter w only.
# Asserts BUJB_wsl_distribution is installed on the Windows host before any
# WSL-targeted action runs; replaces opaque downstream wsl.exe failures with
# a copy-paste-ready operator hint pointing at the privileged-SSH tabtarget.
zbujb_garrison_w_preflight() {
  zbujb_sentinel
  buc_step "  [w-preflight] Verify WSL distribution '${BUJB_wsl_distribution}' is installed"

  local z_exit=0
  zbujb_admin_powershell "wsl.exe --list --quiet" \
      > "${ZBUJB_WSL_PREFLIGHT_STDOUT}"           \
      2> "${ZBUJB_WSL_PREFLIGHT_STDERR}"          \
    || z_exit=$?

  test "${z_exit}" -eq 0 \
    || buc_die "WSL list query failed (ssh exit ${z_exit}); see ${ZBUJB_WSL_PREFLIGHT_STDERR}"

  local z_output
  z_output=$(<"${ZBUJB_WSL_PREFLIGHT_STDOUT}")
  z_output="${z_output//$'\r'/}"

  case $'\n'"${z_output}"$'\n' in
    *$'\n'"${BUJB_wsl_distribution}"$'\n'*) return 0 ;;
  esac

  buc_die "WSL distribution '${BUJB_wsl_distribution}' (BUJB_wsl_distribution) not installed on ${BURN_HOST}.

Distributions present on host:
${z_output:-<none reported>}

Install the canonical distribution before retrying garrison-w:

  tt/buw-jpW.WslInstall.sh ${BUZ_FOLIO}

Or inspect what is currently installed:

  tt/buw-jpS.PrivilegedSsh.sh ${BUZ_FOLIO} 'powershell -NoProfile -Command \"\$env:WSL_UTF8=1; wsl.exe --list --verbose\"'
"
}

# Step 6 -- workload-side round-trip validation (knock).
zbujb_garrison_step6_validate() {
  local z_letter="${1:-}"
  buc_step "  [6/6] Validate workload round-trip"

  local z_exit=0
  ssh -i "${BURP_WORKLOAD_KEY_FILE}"     \
      -o IdentitiesOnly=yes                       \
      -o BatchMode=yes                            \
      -o StrictHostKeyChecking=accept-new         \
      -o ConnectTimeout=10                        \
      "${BUJB_workload_user}@${BURN_HOST}" \
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

  buc_step "Garrison-${z_letter}: ${BUZ_FOLIO} (${BURN_HOST})"

  case "${z_letter}" in
    w) zbujb_garrison_w_preflight ;;
  esac
  zbujb_garrison_step1_admin_open    "${z_letter}"
  zbujb_garrison_step2_destroy       "${z_letter}"
  zbujb_garrison_step3_create        "${z_letter}"
  zbujb_garrison_step4_place_trust   "${z_letter}"
  zbujb_garrison_step5_plant_key     "${z_letter}"
  zbujb_garrison_step6_validate      "${z_letter}"

  buc_step "Garrison-${z_letter} succeeded"
}

######################################################################
# Internal: Fenestrate helpers (BUSJPF — Windows OpenSSH only)
#
# Each phase 1 chunk is a separate ssh call with publickey,password preferred
# auth. On a fresh node, chunk A's first publickey attempt fails and ssh
# falls through to /dev/tty password prompt — operator types once. After
# chunk A places the admin pubkey, chunk B's publickey attempt succeeds
# (the running sshd's old config still permits pubkey auth and now reads
# the updated authorized_keys), so no further prompt. Phase 2 is key-only.
# No ControlMaster, no traps, no 2>/dev/null.

zbujb_fenestrate_assert_platform() {
  zbujb_sentinel
  test "${BURN_PLATFORM}" = "bubep_windows" \
    || buc_die "fenestrate requires bubep_windows, got '${BURN_PLATFORM}'"
}

# zbujb_fenestrate_exec_with_password_fallback STDOUT_FILE STDERR_FILE
# Reads a PowerShell script from stdin and runs it on the remote node as
# the privileged user. publickey,password preferred (BatchMode=no allows
# /dev/tty password prompt on first run). Default Windows OpenSSH shell is
# cmd.exe; we explicitly invoke `powershell -NoProfile -File -` to feed
# the script via stdin. Returns ssh's exit code (caller decides).
zbujb_fenestrate_exec_with_password_fallback() {
  zbujb_sentinel
  local -r z_stdout_file="${1:-}"
  local -r z_stderr_file="${2:-}"
  test -n "${z_stdout_file}" || buc_die "zbujb_fenestrate_exec_with_password_fallback: stdout_file required"
  test -n "${z_stderr_file}" || buc_die "zbujb_fenestrate_exec_with_password_fallback: stderr_file required"

  ssh -i "${BURP_PRIVILEGED_KEY_FILE}"            \
      -o IdentitiesOnly=yes                                \
      -o BatchMode=no                                      \
      -o PreferredAuthentications=publickey,password       \
      -o StrictHostKeyChecking=accept-new                  \
      -o ConnectTimeout=15                                 \
      "${BURP_PRIVILEGED_USER}@${BURN_HOST}" \
      'powershell -NoProfile -File -'                      \
      > "${z_stdout_file}"                                 \
      2> "${z_stderr_file}"
}

# zbujb_fenestrate_exec_keyonly STDOUT_FILE STDERR_FILE
# Same as above but BatchMode=yes (no password fallback). Used for phase 2.
zbujb_fenestrate_exec_keyonly() {
  zbujb_sentinel
  local -r z_stdout_file="${1:-}"
  local -r z_stderr_file="${2:-}"
  test -n "${z_stdout_file}" || buc_die "zbujb_fenestrate_exec_keyonly: stdout_file required"
  test -n "${z_stderr_file}" || buc_die "zbujb_fenestrate_exec_keyonly: stderr_file required"

  ssh -i "${BURP_PRIVILEGED_KEY_FILE}"            \
      -o IdentitiesOnly=yes                                \
      -o BatchMode=yes                                     \
      -o PreferredAuthentications=publickey                \
      -o StrictHostKeyChecking=accept-new                  \
      -o ConnectTimeout=15                                 \
      "${BURP_PRIVILEGED_USER}@${BURN_HOST}" \
      'powershell -NoProfile -File -'                      \
      > "${z_stdout_file}"                                 \
      2> "${z_stderr_file}"
}

# zbujb_fenestrate_verify_directives REMOTE_FILE -- load REMOTE_FILE (raw
# sshd_config bytes from PowerShell Get-Content), strip CR, then assert
# each directive in BUJB_sshd_hardening appears with the expected value.
# Load-then-iterate (no nested while-read on stdin); pure parameter
# expansion + case (no awk/grep/tr).
zbujb_fenestrate_verify_directives() {
  zbujb_sentinel
  local -r z_remote_file="${1:-}"
  test -n "${z_remote_file}" || buc_die "zbujb_fenestrate_verify_directives: remote_file required"
  test -f "${z_remote_file}" || buc_die "zbujb_fenestrate_verify_directives: remote_file not found: ${z_remote_file}"

  local -r z_raw_bytes=$(<"${z_remote_file}")
  test -n "${z_raw_bytes}" || buc_die "zbujb_fenestrate_verify_directives: empty remote sshd_config bytes: ${z_remote_file}"
  local -r z_clean_bytes="${z_raw_bytes//$'\r'/}"

  local z_directives_roll=()
  local z_pair=""
  while IFS= read -r z_pair || test -n "${z_pair}"; do
    test -n "${z_pair}" || continue
    z_directives_roll+=("${z_pair}")
  done <<<"${BUJB_sshd_hardening}"

  local z_remote_lines_roll=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_remote_lines_roll+=("${z_line}")
  done <<<"${z_clean_bytes}"

  local z_directive=""
  local z_expected=""
  local z_actual=""
  local z_after_directive=""
  local z_i=0
  local z_j=0
  for z_i in "${!z_directives_roll[@]}"; do
    z_directive="${z_directives_roll[$z_i]%% *}"
    z_expected="${z_directives_roll[$z_i]#* }"
    z_actual=""

    for z_j in "${!z_remote_lines_roll[@]}"; do
      case "${z_remote_lines_roll[$z_j]}" in
        "${z_directive} "*)
          z_after_directive="${z_remote_lines_roll[$z_j]#"${z_directive} "}"
          z_actual="${z_after_directive%% *}"
          break
          ;;
      esac
    done

    test -n "${z_actual}" \
      || buc_die "Hardening verify: directive '${z_directive}' missing or commented in remote sshd_config"
    test "${z_actual}" = "${z_expected}" \
      || buc_die "Hardening verify: ${z_directive}: expected '${z_expected}', got '${z_actual}'"
  done
}

######################################################################
# Internal: Fenestrate phases

# zbujb_fenestrate_phase1 -- chunk A (install admin pubkey idempotently +
# icacls + merge sshd_config hardening + sshd -t + emit raw bytes); bash-
# side parse + verify; then chunk B (Restart-Service sshd, disconnect
# expected — exit code ignored).
zbujb_fenestrate_phase1() {
  zbujb_sentinel

  ssh-keygen -y -P '' -f "${BURP_PRIVILEGED_KEY_FILE}" \
      > "${ZBUJB_PUBKEY_STDOUT_PRIV}" \
      2> "${ZBUJB_PUBKEY_STDERR_PRIV}" \
    || buc_die "ssh-keygen -y failed for admin key: ${BURP_PRIVILEGED_KEY_FILE} — see ${ZBUJB_PUBKEY_STDERR_PRIV}"
  local z_pubkey
  z_pubkey=$(<"${ZBUJB_PUBKEY_STDOUT_PRIV}")
  z_pubkey="${z_pubkey//$'\n'/}"

  buc_step "  [Phase 1] Chunk A: pubkey + icacls + sshd_config + sshd -t + emit (operator may be prompted for admin password on first run)"

  zbujb_fenestrate_exec_with_password_fallback \
      "${ZBUJB_FENESTRATE_PHASE1_STDOUT}"      \
      "${ZBUJB_FENESTRATE_PHASE1_STDERR}"      \
    <<PS1 || buc_die "Phase 1 chunk A failed — admin pubkey/icacls/sshd_config/sshd -t did not all succeed; see ${ZBUJB_FENESTRATE_PHASE1_STDERR}"
\$ErrorActionPreference = 'Stop'

\$pubkey = '${z_pubkey}'
\$adminAuthKeys = "\$env:ProgramData\\ssh\\administrators_authorized_keys"
\$sshConfig    = "\$env:ProgramData\\ssh\\sshd_config"

# Idempotent admin pubkey install
if (-not (Test-Path \$adminAuthKeys)) {
  New-Item -Path \$adminAuthKeys -ItemType File -Force | Out-Null
}
\$existingLines = @(Get-Content \$adminAuthKeys -ErrorAction SilentlyContinue)
if (\$existingLines -notcontains \$pubkey) {
  Add-Content -Path \$adminAuthKeys -Value \$pubkey -Encoding ASCII
}

# icacls lockdown (idempotent)
icacls \$adminAuthKeys /inheritance:r /grant 'SYSTEM:F' /grant 'BUILTIN\\Administrators:F' 2>&1 | Out-Null
if (\$LASTEXITCODE -ne 0) { throw "icacls failed (exit \$LASTEXITCODE)" }

# Idempotent sshd_config harden — replace the first matching line for
# each directive (commented or otherwise) and drop subsequent duplicates;
# append directives that are absent. Convergent on re-run.
\$directives = [ordered]@{
  'PubkeyAuthentication'   = 'yes'
  'PasswordAuthentication' = 'no'
  'PermitEmptyPasswords'   = 'no'
}

\$lines = @(Get-Content \$sshConfig)
\$out   = New-Object System.Collections.Generic.List[string]
\$seen  = @{}
foreach (\$line in \$lines) {
  \$matched = \$false
  foreach (\$k in \$directives.Keys) {
    if (\$line -match "^\\s*#*\\s*\$k\\s+\\S") {
      if (-not \$seen.ContainsKey(\$k)) {
        \$out.Add("\$k \$(\$directives[\$k])")
        \$seen[\$k] = \$true
      }
      \$matched = \$true
      break
    }
  }
  if (-not \$matched) { \$out.Add(\$line) }
}
foreach (\$k in \$directives.Keys) {
  if (-not \$seen.ContainsKey(\$k)) { \$out.Add("\$k \$(\$directives[\$k])") }
}
Set-Content -Path \$sshConfig -Value \$out -Encoding ASCII

# sshd -t — penultimate atomic op; aborts before restart on bad config
& 'C:\\Windows\\System32\\OpenSSH\\sshd.exe' -t
if (\$LASTEXITCODE -ne 0) { throw "sshd -t failed (exit \$LASTEXITCODE)" }

# Emit raw bytes for bash-side parse (last op so output is clean)
Get-Content \$sshConfig -Raw
PS1

  buc_step "  [Phase 1] Verify hardened directives via bash-side parse"
  zbujb_fenestrate_verify_directives "${ZBUJB_FENESTRATE_PHASE1_STDOUT}"

  buc_step "  [Phase 1] Chunk B: Restart-Service sshd (disconnect expected — exit code ignored)"
  zbujb_fenestrate_exec_with_password_fallback \
      "${ZBUJB_FENESTRATE_RESTART_STDOUT}"     \
      "${ZBUJB_FENESTRATE_RESTART_STDERR}"     \
    <<'PS1' || true
$ErrorActionPreference = 'Continue'
Restart-Service sshd
PS1
}

# zbujb_fenestrate_phase2 -- reconnect via key-only auth and re-verify the
# hardened directives served by the running sshd.
zbujb_fenestrate_phase2() {
  zbujb_sentinel

  buc_step "  [Phase 2] Reconnect under key-only auth + re-verify"

  # Allow sshd a moment to come back from Restart-Service.
  sleep 3

  zbujb_fenestrate_exec_keyonly                 \
      "${ZBUJB_FENESTRATE_PHASE2_STDOUT}"       \
      "${ZBUJB_FENESTRATE_PHASE2_STDERR}"       \
    <<'PS1' || buc_die "Phase 2 reconnect failed — possible brick: admin pubkey not honored after restart or sshd did not come back up; see ${ZBUJB_FENESTRATE_PHASE2_STDERR}"
$ErrorActionPreference = 'Stop'
Get-Content "$env:ProgramData\ssh\sshd_config" -Raw
PS1

  zbujb_fenestrate_verify_directives "${ZBUJB_FENESTRATE_PHASE2_STDOUT}"
}

######################################################################
# Public: Fenestrate ceremony

# bujb_fenestrate -- run the two-phase fenestrate ceremony for a Windows
# OpenSSH node. Caller must have invoked bujb_resolve_investiture beforehand.
bujb_fenestrate() {
  zbujb_sentinel
  test "${ZBUJB_RESOLVED:-}" = "1" \
    || buc_die "bujb_fenestrate: call bujb_resolve_investiture first"

  zbujb_fenestrate_assert_platform

  buc_step "Fenestrate: ${BUZ_FOLIO} (${BURN_HOST})"

  zbujb_fenestrate_phase1
  zbujb_fenestrate_phase2

  buc_step "Fenestrate succeeded"
}

######################################################################
# Public: WSL Install (provision canonical WSL distribution)

zbujb_wsl_install_assert_platform() {
  zbujb_sentinel
  test "${BURN_PLATFORM}" = "bubep_windows" \
    || buc_die "wsl-install requires bubep_windows, got '${BURN_PLATFORM}'"
}

# bujb_wsl_install -- idempotently provision BUJB_wsl_distribution by purging
# any prior state, installing an Ubuntu-24.04 seed, exporting it to a .tar,
# importing under the canonical name, then unregistering the seed and removing
# the .tar. Caller must have invoked bujb_resolve_investiture beforehand.
# Each step propagates failure via zbujb_admin_powershell + || buc_die.
bujb_wsl_install() {
  zbujb_sentinel
  test "${ZBUJB_RESOLVED:-}" = "1" \
    || buc_die "bujb_wsl_install: call bujb_resolve_investiture first"

  zbujb_wsl_install_assert_platform

  local -r z_seed='Ubuntu-24.04'
  local -r z_install_dir='C:\WSL'
  local -r z_distro_dir="${z_install_dir}\\${BUJB_wsl_distribution}"
  local -r z_tar_path="${z_install_dir}\\${BUJB_wsl_distribution}.tar"

  buc_step "WSL Install: ${BUJB_wsl_distribution} on ${BURN_HOST} (seed: ${z_seed})"

  buc_step "  [1/6] Purge prior state (idempotent: unregister both, remove tar+dir)"
  local -r z_purge_body="if ((wsl.exe --list --quiet) -match '${BUJB_wsl_distribution}') { wsl.exe --unregister ${BUJB_wsl_distribution}; if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE } }; if ((wsl.exe --list --quiet) -match '${z_seed}') { wsl.exe --unregister ${z_seed}; if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE } }; if (Test-Path '${z_tar_path}') { Remove-Item -Force '${z_tar_path}' }; if (Test-Path '${z_distro_dir}') { Remove-Item -Recurse -Force '${z_distro_dir}' }"
  zbujb_admin_powershell "${z_purge_body}" \
    || buc_die "Purge step failed"

  buc_step "  [2/6] Ensure ${z_install_dir} directory"
  zbujb_admin_powershell "New-Item -ItemType Directory -Path '${z_install_dir}' -Force | Out-Null" \
    || buc_die "Failed to create ${z_install_dir}"

  buc_step "  [3/6] Install ${z_seed} seed distribution"
  zbujb_admin_powershell "wsl.exe --install --no-launch -d ${z_seed}" \
    || buc_die "Failed to install ${z_seed} seed"

  buc_step "  [4/6] Export ${z_seed} to ${z_tar_path}"
  zbujb_admin_powershell "wsl.exe --export ${z_seed} '${z_tar_path}'" \
    || buc_die "Failed to export ${z_seed}"

  buc_step "  [5/6] Import ${BUJB_wsl_distribution} from ${z_tar_path}"
  zbujb_admin_powershell "wsl.exe --import ${BUJB_wsl_distribution} '${z_distro_dir}' '${z_tar_path}'" \
    || buc_die "Failed to import ${BUJB_wsl_distribution}"

  buc_step "  [6/6] Cleanup: unregister ${z_seed} and remove .tar"
  local -r z_cleanup_body="wsl.exe --unregister ${z_seed}; if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE }; if (Test-Path '${z_tar_path}') { Remove-Item -Force '${z_tar_path}' }"
  zbujb_admin_powershell "${z_cleanup_body}" \
    || buc_die "Cleanup step failed"

  buc_step "WSL Install succeeded"
}

######################################################################
# Public: Privileged SSH (thin admin-side pass-through)

# bujb_privileged_ssh COMMAND... -- run an arbitrary command on the BURN
# node as BURP_PRIVILEGED_USER under key-only auth. Pass-through: argv is
# handed to ssh as the remote command without shell wrapping (operator
# prepends `powershell -Command`, `bash -c`, etc. as the platform requires).
# Caller must have invoked bujb_resolve_investiture beforehand. Returns
# ssh's exit code.
bujb_privileged_ssh() {
  zbujb_sentinel
  test "${ZBUJB_RESOLVED:-}" = "1" \
    || buc_die "bujb_privileged_ssh: call bujb_resolve_investiture first"
  test $# -ge 1 \
    || buc_die "bujb_privileged_ssh: command required"

  buc_step "Privileged SSH: ${BURP_PRIVILEGED_USER}@${BURN_HOST} (${BUZ_FOLIO})"

  local z_exit=0
  ssh -i "${BURP_PRIVILEGED_KEY_FILE}"            \
      -o IdentitiesOnly=yes                       \
      -o BatchMode=yes                            \
      -o StrictHostKeyChecking=accept-new         \
      -o ConnectTimeout=15                        \
      "${BURP_PRIVILEGED_USER}@${BURN_HOST}"      \
      "$@"                                        \
    || z_exit=$?
  return "${z_exit}"
}

# eof
