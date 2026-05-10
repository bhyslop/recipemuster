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
# BUJP Preflight Module — garrison step-1 precondition gate.

set -euo pipefail

test -z "${ZBUJP_SOURCED:-}" || buc_die "Module bujp multiply sourced - check sourcing hierarchy"
ZBUJP_SOURCED=1

######################################################################
# Tinder constants

# Sudoers drop-in filename (dot-free, lives at /etc/sudoers.d/<name>).
BUJP_sudoers_filename='bujb-garrison'

# Sudoers drop-in mode (per visudo convention).
BUJP_sudoers_mode='0440'

# Remote home-relative cache directory for the staged sudoers snippet.
# Per docket: home-cache, not /tmp.
BUJP_remote_cache_dir='.cache/bujb-garrison'

######################################################################
# Internal Functions (zbujp_*)

zbujp_kindle() {
  test -z "${ZBUJP_KINDLED:-}" || buc_die "Module bujp already kindled"

  # Per-probe stdout/stderr captures.
  readonly ZBUJP_SSH_PROBE_PREFIX="${BURD_TEMP_DIR}/bujp_ssh_probe_"
  readonly ZBUJP_SUDO_PROBE_PREFIX="${BURD_TEMP_DIR}/bujp_sudo_probe_"
  readonly ZBUJP_ADMIN_GROUP_STDOUT="${BURD_TEMP_DIR}/bujp_admin_group_stdout.txt"
  readonly ZBUJP_ADMIN_GROUP_STDERR="${BURD_TEMP_DIR}/bujp_admin_group_stderr.txt"

  # Failure-recovery captures (rendered snippet + scp/visudo evidence).
  readonly ZBUJP_SUDOERS_SNIPPET="${BURD_TEMP_DIR}/bujp_sudoers_snippet.txt"
  readonly ZBUJP_SCP_STDERR="${BURD_TEMP_DIR}/bujp_scp_stderr.txt"
  readonly ZBUJP_VISUDO_STDOUT="${BURD_TEMP_DIR}/bujp_visudo_stdout.txt"
  readonly ZBUJP_VISUDO_STDERR="${BURD_TEMP_DIR}/bujp_visudo_stderr.txt"

  readonly ZBUJP_KINDLED=1
}

zbujp_sentinel() {
  test "${ZBUJP_KINDLED:-}" = "1" || buc_die "Module bujp not kindled - call zbujp_kindle first"
}

# zbujp_probe_ssh_connect LETTER -- exit 0 if admin SSH reaches the
# letter's shell-environment under key-only auth; die with a platform-
# specific recovery hint otherwise.
zbujp_probe_ssh_connect() {
  zbujp_sentinel
  local z_letter="${1:-}"

  local z_exit=0
  case "${z_letter}" in
    b)
      zbujb_admin_exec_native 'exit 0' \
          > "${ZBUJP_SSH_PROBE_PREFIX}stdout.txt" \
          2> "${ZBUJP_SSH_PROBE_PREFIX}stderr.txt" \
        || z_exit=$?
      ;;
    c)
      zbujb_admin_exec_cygwin 'exit 0' \
          > "${ZBUJP_SSH_PROBE_PREFIX}stdout.txt" \
          2> "${ZBUJP_SSH_PROBE_PREFIX}stderr.txt" \
        || z_exit=$?
      ;;
    w)
      zbujb_admin_exec_wsl 'exit 0' \
          > "${ZBUJP_SSH_PROBE_PREFIX}stdout.txt" \
          2> "${ZBUJP_SSH_PROBE_PREFIX}stderr.txt" \
        || z_exit=$?
      ;;
    *)
      buc_die "zbujp_probe_ssh_connect: invalid shell-letter '${z_letter}'"
      ;;
  esac

  test "${z_exit}" -eq 0 && return 0

  case "${BURN_PLATFORM}" in
    bubep_windows)
      buc_die "Admin SSH unreachable for ${BURP_PRIVILEGED_USER}@${BURN_HOST} (key=${BURP_PRIVILEGED_KEY_FILE}, exit ${z_exit}).

Run fenestrate first:
  tt/buw-jpF.Fenestrate.sh ${BUZ_FOLIO}

ssh stderr (see ${ZBUJP_SSH_PROBE_PREFIX}stderr.txt):
$(<"${ZBUJP_SSH_PROBE_PREFIX}stderr.txt")"
      ;;
    *)
      buc_die "Admin SSH unreachable for ${BURP_PRIVILEGED_USER}@${BURN_HOST} (key=${BURP_PRIVILEGED_KEY_FILE}, exit ${z_exit}).

Place admin pubkey via:
  ssh-copy-id -i ${BURP_PRIVILEGED_KEY_FILE}.pub ${BURP_PRIVILEGED_USER}@${BURN_HOST}

ssh stderr (see ${ZBUJP_SSH_PROBE_PREFIX}stderr.txt):
$(<"${ZBUJP_SSH_PROBE_PREFIX}stderr.txt")"
      ;;
  esac
}

# zbujp_probe_admin_group_mac -- exit 0 if BURP_PRIVILEGED_USER is a
# member of the macOS admin group; die naming `dseditgroup -o edit`
# otherwise.
zbujp_probe_admin_group_mac() {
  zbujp_sentinel

  local z_exit=0
  zbujb_admin_exec_native "dseditgroup -o checkmember -m '${BURP_PRIVILEGED_USER}' admin" \
      > "${ZBUJP_ADMIN_GROUP_STDOUT}" \
      2> "${ZBUJP_ADMIN_GROUP_STDERR}" \
    || z_exit=$?

  test "${z_exit}" -eq 0 && return 0

  buc_die "${BURP_PRIVILEGED_USER} is not a member of the admin group on ${BURN_HOST} (dseditgroup checkmember exit ${z_exit}).

Add to admin group from a session with directory-edit privilege:

  sudo dseditgroup -o edit -a ${BURP_PRIVILEGED_USER} -t user admin

Then re-run this tabtarget."
}

# zbujp_probe_sudo PLATFORM -- exit 0 if BURP_PRIVILEGED_USER has
# NOPASSWD sudo on PLATFORM (bubep_linux | bubep_mac); route to
# zbujp_diag_sudo_missing on miss.
zbujp_probe_sudo() {
  zbujp_sentinel
  local z_platform="${1:-}"

  local z_exit=0
  zbujb_admin_exec_native "sudo -n true" \
      > "${ZBUJP_SUDO_PROBE_PREFIX}baseline_stdout.txt" \
      2> "${ZBUJP_SUDO_PROBE_PREFIX}baseline_stderr.txt" \
    || z_exit=$?

  if test "${z_exit}" -ne 0; then
    zbujp_diag_sudo_missing "${z_platform}"
  fi

  local z_probe_cmd
  case "${z_platform}" in
    bubep_linux) z_probe_cmd='userdel' ;;
    bubep_mac)   z_probe_cmd='sysadminctl' ;;
    *)           buc_die "zbujp_probe_sudo: unsupported platform '${z_platform}'" ;;
  esac

  z_exit=0
  zbujb_admin_exec_native "sudo -ln ${z_probe_cmd}" \
      > "${ZBUJP_SUDO_PROBE_PREFIX}scope_stdout.txt" \
      2> "${ZBUJP_SUDO_PROBE_PREFIX}scope_stderr.txt" \
    || z_exit=$?

  if test "${z_exit}" -ne 0; then
    zbujp_diag_sudo_missing "${z_platform}"
  fi
}

# zbujp_diag_sudo_missing PLATFORM -- render a scoped NOPASSWD snippet
# locally, stage it to the remote home-cache, validate with visudo -cf,
# then die with a copy-paste-safe `sudo install` line.
zbujp_diag_sudo_missing() {
  local z_platform="${1:-}"

  local z_cmd_list
  case "${z_platform}" in
    bubep_linux)
      z_cmd_list='/usr/sbin/userdel, /usr/sbin/useradd, /usr/bin/passwd, /usr/bin/install, /bin/rm, /bin/mkdir, /bin/chmod, /bin/chown, /usr/bin/tee'
      ;;
    bubep_mac)
      z_cmd_list='/usr/sbin/sysadminctl, /usr/bin/dscl, /usr/bin/install, /bin/rm, /bin/mkdir, /bin/chmod, /usr/sbin/chown, /usr/bin/tee'
      ;;
    *)
      buc_die "zbujp_diag_sudo_missing: unsupported platform '${z_platform}'"
      ;;
  esac

  cat > "${ZBUJP_SUDOERS_SNIPPET}" <<SNIPPET
# Generated by bujp_preflight failure-recovery for ${BURP_PRIVILEGED_USER}.
# Scoped command list derived from sudo -n callsites in bujb_jurisdiction.sh.
# Review before installing.
${BURP_PRIVILEGED_USER} ALL=(ALL) NOPASSWD: ${z_cmd_list}
SNIPPET

  local z_remote_path="${BUJP_remote_cache_dir}/${BUJP_sudoers_filename}.snippet"

  local z_exit=0
  zbujb_admin_exec_native "mkdir -p '${BUJP_remote_cache_dir}'" \
      > /dev/null 2> "${ZBUJP_SCP_STDERR}" \
    || z_exit=$?
  if test "${z_exit}" -ne 0; then
    buc_die "sudo NOPASSWD grant missing for ${BURP_PRIVILEGED_USER} on ${z_platform}, and could not create remote ~/${BUJP_remote_cache_dir} for staging (ssh exit ${z_exit}). See ${ZBUJP_SCP_STDERR}.

Local snippet (install manually after review):
$(<"${ZBUJP_SUDOERS_SNIPPET}")"
  fi

  z_exit=0
  scp -i "${BURP_PRIVILEGED_KEY_FILE}"                           \
      -o IdentitiesOnly=yes                                      \
      -o StrictHostKeyChecking=accept-new                        \
      -o "${BUJB_ssh_opt_batchmode_yes}"                         \
      -o "${BUJB_ssh_opt_connecttimeout_15}"                     \
      "${ZBUJP_SUDOERS_SNIPPET}"                                 \
      "${BURP_PRIVILEGED_USER}@${BURN_HOST}:${z_remote_path}"    \
      > /dev/null 2> "${ZBUJP_SCP_STDERR}"                       \
    || z_exit=$?
  if test "${z_exit}" -ne 0; then
    buc_die "sudo NOPASSWD grant missing for ${BURP_PRIVILEGED_USER} on ${z_platform}, and scp of staged snippet failed (exit ${z_exit}). See ${ZBUJP_SCP_STDERR}.

Local snippet (install manually after review):
$(<"${ZBUJP_SUDOERS_SNIPPET}")"
  fi

  z_exit=0
  zbujb_admin_exec_native "visudo -cf '${z_remote_path}'" \
      > "${ZBUJP_VISUDO_STDOUT}" \
      2> "${ZBUJP_VISUDO_STDERR}" \
    || z_exit=$?
  if test "${z_exit}" -ne 0; then
    buc_die "sudo NOPASSWD grant missing for ${BURP_PRIVILEGED_USER} on ${z_platform}, and visudo -cf validation of staged snippet failed (exit ${z_exit}). See ${ZBUJP_VISUDO_STDERR}.

Snippet staged at ${BURP_PRIVILEGED_USER}@${BURN_HOST}:~/${z_remote_path}; fix and install manually."
  fi

  buc_die "sudo NOPASSWD grant missing for ${BURP_PRIVILEGED_USER} on ${z_platform}.

Snippet staged at ${BURP_PRIVILEGED_USER}@${BURN_HOST}:~/${z_remote_path} and validated by visudo -cf.

Install with one command from a session as ${BURP_PRIVILEGED_USER}@${BURN_HOST}:

  sudo install -m ${BUJP_sudoers_mode} -o root -g root '~/${z_remote_path}' '/etc/sudoers.d/${BUJP_sudoers_filename}'

Then re-run this tabtarget.

Alternative for a personal workstation — blanket NOPASSWD: ALL:

  echo '${BURP_PRIVILEGED_USER} ALL=(ALL) NOPASSWD: ALL' | sudo tee /etc/sudoers.d/${BUJP_sudoers_filename}
  sudo chmod ${BUJP_sudoers_mode} /etc/sudoers.d/${BUJP_sudoers_filename}"
}

######################################################################
# Public: Preflight ceremony

# bujp_preflight LETTER -- garrison step-1 gate. Three-part shape:
#   (a) Workload shell-environment reachability per letter (b/c/w).
#   (b) Host-posture verification per platform — delegates to invigilate
#       (single source of truth for "correctly-configured", BUSJI{W,M,L}).
#   (c) Workload-precondition probes that are not host-posture
#       (privilege grants on the admin account: sudo NOPASSWD, admin-group).
bujp_preflight() {
  zbujp_sentinel
  local z_letter="${1:-}"

  buc_step "  [1/6] Preflight (${BURP_PRIVILEGED_USER}@${BURN_HOST}, letter=${z_letter})"

  zbujp_probe_ssh_connect "${z_letter}"

  case "${BURN_PLATFORM}" in
    bubep_linux)   bujb_invigilate_linux   ;;
    bubep_mac)     bujb_invigilate_macos   ;;
    bubep_windows) bujb_invigilate_windows ;;
    *) buc_die "bujp_preflight: unsupported BURN_PLATFORM '${BURN_PLATFORM}'" ;;
  esac

  case "${z_letter}" in
    b)
      case "${BURN_PLATFORM}" in
        bubep_linux)
          zbujp_probe_sudo bubep_linux
          ;;
        bubep_mac)
          zbujp_probe_admin_group_mac
          zbujp_probe_sudo bubep_mac
          ;;
        *)
          buc_die "bujp_preflight: letter=b incompatible with BURN_PLATFORM='${BURN_PLATFORM}'"
          ;;
      esac
      ;;
    c|w)
      : # Windows: invigilate-windows covers WSL distribution + admin posture.
      ;;
    *)
      buc_die "bujp_preflight: invalid shell-letter '${z_letter}'"
      ;;
  esac
}

# eof
