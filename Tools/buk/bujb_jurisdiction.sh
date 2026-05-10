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

# Tinder constants (pure string literals or tinder-on-tinder composition —
# available at source time)

# Canonical workload OS user name provisioned on every node by garrison.
# Project-wide convention; consumed by every account-relative path tinder
# below, substituted into BUJB_command_w via bujb_command_for_capture,
# and consumed by garrison/cli display strings.
BUJB_workload_user='bujuw_user'

# Canonical WSL distribution name reached by garrison-w.
BUJB_wsl_distribution='rbtww-main'

# Path segment shared by every account-relative SSH state directory.
BUJB_path_dotssh='.ssh'

# WSL artifact basenames placed under the workload Windows profile by the
# garrison-w seed/import sequence.
BUJB_seed_basename='rbtww-seed.tar'
BUJB_wsl_root_basename='rbtww-fs'

# Cygwin install root in two slash conventions. Forward-slash form
# survives `command="..."` directives and cmd.exe / PS argv layers
# without escape pain (used in BUJB_command_c and zbujb_admin_exec_cygwin).
# Back-slash form is for PowerShell path arguments where Test-Path /
# Remove-Item expect native Windows separators (used by the obliterate
# Cygwin-home probe/remove). Do not collapse — the divergence is
# load-bearing per the layer that consumes each form.
BUJB_path_cygwin_root_fwd='C:/cygwin64'
BUJB_path_cygwin_root_bs="C:\\cygwin64"

# Windows-OpenSSH path to wsl.exe, embedded in the locked-down command=
# directive for shell-letter w. Forward-slash form per the same argv-layer
# rule as BUJB_path_cygwin_root_fwd.
BUJB_path_wsl_exe='C:/Windows/System32/wsl.exe'

# Workload home directories per coordinate system. Tinder-on-tinder so a
# future BUJB_workload_user rename is a one-line edit.
BUJB_path_win_user_home="C:\\Users\\${BUJB_workload_user}"
BUJB_path_winenv_user_home="%SystemDrive%\\Users\\${BUJB_workload_user}"
BUJB_path_wsl_user_home="/mnt/c/Users/${BUJB_workload_user}"
BUJB_path_cyg_user_home="/cygdrive/c/Users/${BUJB_workload_user}"
BUJB_path_mac_user_home="/Users/${BUJB_workload_user}"
BUJB_path_posix_user_home="/home/${BUJB_workload_user}"
BUJB_path_cygwin_user_home="${BUJB_path_cygwin_root_bs}\\home\\${BUJB_workload_user}"

# .ssh directories under each home.
BUJB_path_win_user_ssh="${BUJB_path_win_user_home}\\${BUJB_path_dotssh}"
BUJB_path_wsl_user_ssh="${BUJB_path_wsl_user_home}/${BUJB_path_dotssh}"
BUJB_path_cyg_user_ssh="${BUJB_path_cyg_user_home}/${BUJB_path_dotssh}"
BUJB_path_posix_user_ssh="${BUJB_path_posix_user_home}/${BUJB_path_dotssh}"

# authorized_keys file under each .ssh.
BUJB_path_win_user_authkeys="${BUJB_path_win_user_ssh}\\authorized_keys"
BUJB_path_wsl_user_authkeys="${BUJB_path_wsl_user_ssh}/authorized_keys"
BUJB_path_cyg_user_authkeys="${BUJB_path_cyg_user_ssh}/authorized_keys"

# WSL artifacts under the Windows workload profile.
BUJB_path_win_seed_tarball="${BUJB_path_win_user_home}\\${BUJB_seed_basename}"
BUJB_path_win_wsl_root="${BUJB_path_win_user_home}\\${BUJB_wsl_root_basename}"

# WSL-install seed distribution: the Microsoft-published distribution that
# bujb_wsl_install fetches via `wsl.exe --install --no-launch -d`, exports
# to .tar, then re-imports under BUJB_wsl_distribution.
BUJB_wsl_seed_distribution='Ubuntu-24.04'

# Windows-side install directory housing both seed and canonical
# distribution VHD trees plus the intermediate .tar export.
BUJB_path_win_wsl_install_root='C:\WSL'

# Windows ACL principal names used in icacls /grant arguments. The :F
# permission suffix is appended at use sites alongside ${BUJB_workload_user}:F
# so all three principals follow the same `<principal>:F` shape.
BUJB_acl_principal_system='SYSTEM'
BUJB_acl_principal_admins='BUILTIN\Administrators'

# Shell-letter -> command= directive mappings.
# Forced commands routed through SSH_ORIGINAL_COMMAND keep workload account
# behaviour pinned to the chosen shell regardless of what the SSH client
# requests. Locked spec content; mirrored in BUSJG{B,C,W}.
# Tinder-on-tinder source-time interpolation: cygwin/wsl/distribution/user
# resolve at source time so the value is a fully-baked literal by the time
# bujb_command_for_capture echoes it. \"...\" pairs are literal escaped
# quotes the remote sshd parser expects around the command directive's
# argument; \$SSH_ORIGINAL_COMMAND defers expansion to the remote shell.
BUJB_command_b="command=\"/bin/bash -lc \\\"\$SSH_ORIGINAL_COMMAND\\\"\",no-port-forwarding,no-X11-forwarding,no-agent-forwarding"
BUJB_command_c="command=\"${BUJB_path_cygwin_root_fwd}/bin/bash --login -c \\\"\$SSH_ORIGINAL_COMMAND\\\"\",no-port-forwarding,no-X11-forwarding,no-agent-forwarding"
BUJB_command_w="command=\"${BUJB_path_wsl_exe} --distribution ${BUJB_wsl_distribution} --user ${BUJB_workload_user} --exec /bin/bash -lc \\\"\$SSH_ORIGINAL_COMMAND\\\"\",no-port-forwarding,no-X11-forwarding,no-agent-forwarding"

# Workload privkey destination path on the remote (relative to the workload
# account home directory). Uniform across shell-letters under the current
# design — if a future shell needs a divergent keypath, fan out per-letter.
BUJB_workload_keypath='.ssh/id_ed25519'

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

  # base64 capture (garrison step4 w-branch = authorized_keys line b64-encoded
  # for transport to remote).
  readonly ZBUJB_AUTHKEYS_B64_STDOUT="${BURD_TEMP_DIR}/bujb_authkeys_b64_stdout.txt"
  readonly ZBUJB_AUTHKEYS_B64_STDERR="${BURD_TEMP_DIR}/bujb_authkeys_b64_stderr.txt"

  # WSL distribution preflight captures (garrison-w only).
  readonly ZBUJB_WSL_PREFLIGHT_STDOUT="${BURD_TEMP_DIR}/bujb_wsl_preflight_stdout.txt"
  readonly ZBUJB_WSL_PREFLIGHT_STDERR="${BURD_TEMP_DIR}/bujb_wsl_preflight_stderr.txt"

  # Obliterate per-call captures (reused across the Windows sub-step
  # sequence — last call's content is what's preserved at die time).
  readonly ZBUJB_OBLITERATE_STDOUT="${BURD_TEMP_DIR}/bujb_obliterate_stdout.txt"
  readonly ZBUJB_OBLITERATE_STDERR="${BURD_TEMP_DIR}/bujb_obliterate_stderr.txt"

  # Per-call captures for zbujb_garrison_step4_place_trust.
  readonly ZBUJB_PLACE_TRUST_PREFIX="${BURD_TEMP_DIR}/bujb_place_trust_"

  # Per-call captures for zbujb_garrison_step6_validate.
  readonly ZBUJB_VALIDATE_PREFIX="${BURD_TEMP_DIR}/bujb_validate_"

  # Per-call captures for the garrison-w SSH-as-workload phases
  # (export-seed, init-wsl, lockdown, seed-cleanup).
  readonly ZBUJB_W_INIT_PREFIX="${BURD_TEMP_DIR}/bujb_w_init_"

  # Per-call forensic capture for obliterate sub-steps (counter pattern,
  # like the other _run wrappers). Each ssh call lands at a uniquely
  # numbered file, so probe and destructive ops both leave evidence.
  readonly ZBUJB_OBLITERATE_PREFIX="${BURD_TEMP_DIR}/bujb_obliterate_run_"

  # Single shared emission counter across all _run wrappers — file numbers
  # are continuous across originating functions so chronological order is
  # readable from the embedded number even when prefixes differ.
  z_bujb_emit_index=0

  # SSH base options shared by every ssh invocation in this module:
  # IdentitiesOnly pins the key to ssh -i; StrictHostKeyChecking=accept-new
  # records first-contact host keys without prompting. BatchMode,
  # ConnectTimeout, and PreferredAuthentications vary per call site so they
  # remain inline at each ssh — visible to security review.
  ZBUJB_SSH_BASE_ARGS=(
    -o IdentitiesOnly=yes
    -o StrictHostKeyChecking=accept-new
  )
  readonly ZBUJB_SSH_BASE_ARGS

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
#
# BCG deviation: ZBUJB_RESOLVED is a kindle-shaped (SCREAMING + readonly)
# constant set outside zbujb_kindle. The deviation is load-bearing —
# resolve-investiture runs ssh-keygen dry-loads against private keys, which
# kindle cannot do (kindle establishes module state at sourcing time, not
# after BURP regime enforcement). This matches the regime archetype's
# "enforce after kindle, lock after enforce" pattern (BCG line 113); we
# just spell the lock as a discrete sentinel because there's no enrolled-
# variable set to readonly in this module.
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
# `|| buc_die`. All tinder-on-tinder interpolation already resolved at
# source time — uniform letter-case, no per-letter substitution gymnastics.
bujb_command_for_capture() {
  zbujb_sentinel
  local z_letter="${1:-}"
  case "${z_letter}" in
    b) echo "${BUJB_command_b}" ;;
    c) echo "${BUJB_command_c}" ;;
    w) echo "${BUJB_command_w}" ;;
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
  case "${z_letter}" in
    b)
      case "${BURN_PLATFORM}" in
        bubep_linux) echo "${BUJB_path_posix_user_home}" ;;
        bubep_mac)   echo "${BUJB_path_mac_user_home}"   ;;
        *)           return 1 ;;
      esac
      ;;
    c|w) echo "${BUJB_path_posix_user_home}" ;;
    *)   return 1 ;;
  esac
}

# zbujb_admin_exec_{native,cygwin,wsl} STMT -- run a single bash statement as
# the privileged user on the remote node, in the named shell environment.
# Variant functions per platform (no cryptic letter parameter): the call
# site picks the shell by name, and the function selects the matching
# remote invoker (`bash`, cygwin's bash --login, or wsl.exe ... bash).
# Embedded " characters are escaped to \" so the outer "..." of
# `bash -c "..."` survives the cmd.exe / Windows argv-parser layer
# (relevant for cygwin/wsl variants via Windows OpenSSH).
#
# Single-statement contract per WSG SH-10: STMT must be exactly one of
# (1) one simple command with arguments and redirections, (2) one pipeline
# optionally preceded by `set -o pipefail`, or (3) one simple command
# whose argument is itself an inner-shell body. Multi-statement work
# decomposes into multiple round-trips per the Capture-Decide-Dispatch
# pattern; bash on the curia owns the state machine.
#
# Caller may pipe stdin into any variant (e.g.
#   `zbujb_admin_exec_native "install ... /dev/stdin '<target>'" < <file>`)
# — bash redirection attaches the file to FD 0, ssh inherits it, sshd
# forwards it to the remote command's FD 0, the body's single command
# reads from /dev/stdin. Used by garrison step5 to plant the workload
# key without ever materializing the plaintext on either side's disk.
#
# Caller redirects stdout/stderr for capture; returns ssh exit code.
# These functions do NOT transform $. Body authors are responsible for
# per-shell $-escape discipline (see WSG SH-6: only the wsl variant's
# bodies need `\$name` escaping).
zbujb_admin_exec_native() {
  zbujb_admin_exec_impl 'bash' "$@"
}

zbujb_admin_exec_cygwin() {
  zbujb_admin_exec_impl "${BUJB_path_cygwin_root_fwd}/bin/bash --login" "$@"
}

zbujb_admin_exec_wsl() {
  zbujb_admin_exec_impl "wsl.exe --distribution ${BUJB_wsl_distribution} --user root bash" "$@"
}

# zbujb_admin_exec_impl REMOTE_INVOKER STMT -- shared body for the three
# zbujb_admin_exec_{native,cygwin,wsl} variants. Not called directly by
# step code; callers always go through one of the named variants so the
# call site declares its target shell.
zbujb_admin_exec_impl() {
  zbujb_sentinel
  local z_remote_invoker="${1:-}"
  test -n "${z_remote_invoker}" \
    || buc_die "zbujb_admin_exec_impl: REMOTE_INVOKER required (call via zbujb_admin_exec_{native,cygwin,wsl})"
  shift
  test $# -eq 1 \
    || buc_die "zbujb_admin_exec_*: requires exactly one statement (got $#); decompose multi-statement work via Capture-Decide-Dispatch per WSG SH-10"
  local z_body="$1"

  # Escape " → \" for the cmd.exe / Windows argv-parser layer.
  local z_body_escaped="${z_body//\"/\\\"}"

  ssh -i "${BURP_PRIVILEGED_KEY_FILE}"        \
      "${ZBUJB_SSH_BASE_ARGS[@]}"             \
      -o BatchMode=yes                        \
      -o ConnectTimeout=15                    \
      "${BURP_PRIVILEGED_USER}@${BURN_HOST}"  \
      "${z_remote_invoker} -c \"${z_body_escaped}\""
}

# zbujb_admin_powershell BODY... -- run a PowerShell statement chain on
# the remote node as the privileged user. Prepends discipline prelude:
# $ErrorActionPreference='Stop' (PS errors terminate), $env:WSL_UTF8=1
# (wsl.exe emits UTF-8), $LASTEXITCODE=0 (initialize so the trailing
# if-check doesn't trip on the $null default — $null -ne 0 evaluates True
# in PowerShell typed comparison, which would fire `exit $null` and
# discard buffered object-formatter output for cmdlets like Get-LocalUser
# whose tables haven't flushed to stdout yet). Trailing if-check
# propagates $LASTEXITCODE on non-zero from native commands in the body.
# Caller redirects stdout/stderr; returns ssh's exit code.
zbujb_admin_powershell() {
  zbujb_sentinel
  test $# -ge 1 \
    || buc_die "zbujb_admin_powershell: PowerShell command body required"
  local -r z_body="$*"

  ssh -i "${BURP_PRIVILEGED_KEY_FILE}"        \
      "${ZBUJB_SSH_BASE_ARGS[@]}"             \
      -o BatchMode=yes                        \
      -o ConnectTimeout=15                    \
      "${BURP_PRIVILEGED_USER}@${BURN_HOST}"  \
      "powershell -NoProfile -Command \"\$ErrorActionPreference = 'Stop'; \$env:WSL_UTF8 = 1; \$LASTEXITCODE = 0; ${z_body}; if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE }\""
}

# zbujb_powershell_capture ROLE BODY -- run a single PS expression on the
# remote node as the named role and emit its stdout. Windows CR is stripped
# via bash parameter expansion (no subprocess) so $(...) yields a clean
# value. Single-expression discipline per WSG-PS-5; multi-statement bodies
# do not belong here. Caller || buc_die on capture failure.
# ROLE: zbujb_privileged | zbujb_workload
zbujb_powershell_capture() {
  zbujb_sentinel
  test $# -ge 2 \
    || buc_die "zbujb_powershell_capture: ROLE BODY required"
  local -r z_role="$1"; shift
  local -r z_body="$*"

  local z_key z_user
  case "${z_role}" in
    zbujb_privileged) z_key="${BURP_PRIVILEGED_KEY_FILE}"; z_user="${BURP_PRIVILEGED_USER}" ;;
    zbujb_workload)   z_key="${BURP_WORKLOAD_KEY_FILE}";   z_user="${BUJB_workload_user}"   ;;
    *)                buc_die "zbujb_powershell_capture: unknown role '${z_role}' (zbujb_privileged|zbujb_workload)" ;;
  esac

  local z_out z_exit=0
  z_out=$(ssh -i "${z_key}"                      \
              "${ZBUJB_SSH_BASE_ARGS[@]}"        \
              -o BatchMode=yes                   \
              -o ConnectTimeout=15               \
              "${z_user}@${BURN_HOST}"           \
              "powershell -NoProfile -Command \"\$ErrorActionPreference = 'Stop'; \$env:WSL_UTF8 = 1; \$LASTEXITCODE = 0; ${z_body}\"") \
    || z_exit=$?

  printf '%s' "${z_out//$'\r'/}"
  return "${z_exit}"
}

# zbujb_workload_ssh REMOTE_CMD -- execute a Windows-native command line
# as the workload user via SSH. The default Windows OpenSSH shell is
# cmd.exe; the command line goes through cmd.exe's argv layer to the named
# binary. Used by garrison-w for `wsl.exe --import` (which writes to the
# workload's HKCU\Lxss because the SSH session is a real workload logon)
# and `wsl.exe --distribution rbtww-main --user root bash -c "..."`
# invocations that follow the import. Caller redirects stdout/stderr;
# returns ssh's exit code.
zbujb_workload_ssh() {
  zbujb_sentinel
  test $# -ge 1 \
    || buc_die "zbujb_workload_ssh: REMOTE_CMD required"
  local -r z_remote_cmd="$*"

  ssh -i "${BURP_WORKLOAD_KEY_FILE}"       \
      "${ZBUJB_SSH_BASE_ARGS[@]}"          \
      -o BatchMode=yes                     \
      -o ConnectTimeout=15                 \
      "${BUJB_workload_user}@${BURN_HOST}" \
      "${z_remote_cmd}"
}

######################################################################
# Internal: Garrison steps (6-step ceremony per BUSJG{B,C,W})

# Step 1 -- open admin SSH (test reachability under key-only auth).
zbujb_garrison_step1_admin_open() {
  local z_letter="${1:-}"
  buc_step "  [1/6] Open admin SSH (${BURP_PRIVILEGED_USER}@${BURN_HOST})"

  local z_exit=0
  case "${z_letter}" in
    b) zbujb_admin_exec_native 'exit 0' || z_exit=$? ;;
    c) zbujb_admin_exec_cygwin 'exit 0' || z_exit=$? ;;
    w) zbujb_admin_exec_wsl    'exit 0' || z_exit=$? ;;
    *) buc_die "step1: invalid shell-letter '${z_letter}'" ;;
  esac
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

# zbujb_obliterate_diag_dump LABEL -- diagnostic helper: preserve the
# current ZBUJB_OBLITERATE_STDOUT/STDERR contents under per-label paths
# (so subsequent calls don't overwrite them) and emit a single-line
# preview to buc_step. CR stripped, newlines rendered as `|` so the
# preview stays one line. Inserted after every PS call in the obliterate
# flow so post-mortem inspection has full per-step traces.
zbujb_obliterate_diag_dump() {
  local z_label="${1:-}"
  local z_out_dst="${BURD_TEMP_DIR}/bujb_obliterate_${z_label}_stdout.txt"
  local z_err_dst="${BURD_TEMP_DIR}/bujb_obliterate_${z_label}_stderr.txt"
  cp "${ZBUJB_OBLITERATE_STDOUT}" "${z_out_dst}"
  cp "${ZBUJB_OBLITERATE_STDERR}" "${z_err_dst}"
  local z_out_bytes z_err_bytes z_out_preview z_err_preview
  z_out_bytes=$(wc -c < "${z_out_dst}" | tr -d ' ')
  z_err_bytes=$(wc -c < "${z_err_dst}" | tr -d ' ')
  z_out_preview=$(head -c 240 < "${z_out_dst}" | tr -d '\r' | tr '\n' '|')
  z_err_preview=$(head -c 240 < "${z_err_dst}" | tr -d '\r' | tr '\n' '|')
  buc_step "      [diag/${z_label}] stdout (${z_out_bytes}B): ${z_out_preview}"
  buc_step "      [diag/${z_label}] stderr (${z_err_bytes}B): ${z_err_preview}"
}

zbujb_diag_dump_pair() {
  local z_label="${1:-}"
  local z_stdout="${2:-}"
  local z_stderr="${3:-}"
  local z_out_bytes z_err_bytes z_out_preview z_err_preview
  z_out_bytes=$(wc -c < "${z_stdout}" | tr -d ' ')
  z_err_bytes=$(wc -c < "${z_stderr}" | tr -d ' ')
  z_out_preview=$(head -c 240 < "${z_stdout}" | tr -d '\r' | tr '\n' '|')
  z_err_preview=$(head -c 240 < "${z_stderr}" | tr -d '\r' | tr '\n' '|')
  buc_step "      [diag/${z_label}] stdout (${z_out_bytes}B): ${z_out_preview}"
  buc_step "      [diag/${z_label}] stderr (${z_err_bytes}B): ${z_err_preview}"
}

# zbujb_emit_index_advance OUT_REF -- validate the module-level
# z_bujb_emit_index counter is a non-negative integer, format it as %02d
# into OUT_REF via printf -v, then bump the counter. Single source of
# truth for the four _run wrappers (place_trust, validate, w_init,
# obliterate) so format-and-bump is not inlined four times. Dies on
# non-numeric counter (corruption guard) or missing OUT_REF.
zbujb_emit_index_advance() {
  local -r z_ref="${1:-}"
  test -n "${z_ref}" \
    || buc_die "zbujb_emit_index_advance: OUT_REF (caller var name) required"
  test "${z_bujb_emit_index:-x}" -ge 0 2>/dev/null \
    || buc_die "zbujb_emit_index_advance: z_bujb_emit_index is not a non-negative integer (got '${z_bujb_emit_index:-<unset>}')"
  printf -v "${z_ref}" '%02d' "${z_bujb_emit_index}"
  z_bujb_emit_index=$((z_bujb_emit_index + 1))
}

zbujb_place_trust_run() {
  local z_label="${1:-}"
  shift
  local z_idx_str
  zbujb_emit_index_advance z_idx_str
  local z_out="${ZBUJB_PLACE_TRUST_PREFIX}${z_idx_str}_${z_label}_stdout.txt"
  local z_err="${ZBUJB_PLACE_TRUST_PREFIX}${z_idx_str}_${z_label}_stderr.txt"
  local z_exit=0
  "$@" > "${z_out}" 2> "${z_err}" || z_exit=$?
  zbujb_diag_dump_pair "${z_label}" "${z_out}" "${z_err}"
  return ${z_exit}
}

zbujb_validate_run() {
  local z_label="${1:-}"
  shift
  local z_idx_str
  zbujb_emit_index_advance z_idx_str
  local z_out="${ZBUJB_VALIDATE_PREFIX}${z_idx_str}_${z_label}_stdout.txt"
  local z_err="${ZBUJB_VALIDATE_PREFIX}${z_idx_str}_${z_label}_stderr.txt"
  local z_exit=0
  "$@" > "${z_out}" 2> "${z_err}" || z_exit=$?
  zbujb_diag_dump_pair "${z_label}" "${z_out}" "${z_err}"
  return ${z_exit}
}

zbujb_w_init_run() {
  local z_label="${1:-}"
  shift
  local z_idx_str
  zbujb_emit_index_advance z_idx_str
  local z_out="${ZBUJB_W_INIT_PREFIX}${z_idx_str}_${z_label}_stdout.txt"
  local z_err="${ZBUJB_W_INIT_PREFIX}${z_idx_str}_${z_label}_stderr.txt"
  local z_exit=0
  "$@" > "${z_out}" 2> "${z_err}" || z_exit=$?
  zbujb_diag_dump_pair "${z_label}" "${z_out}" "${z_err}"
  return ${z_exit}
}

zbujb_obliterate_run() {
  local z_label="${1:-}"
  shift
  local z_idx_str
  zbujb_emit_index_advance z_idx_str
  local z_out="${ZBUJB_OBLITERATE_PREFIX}${z_idx_str}_${z_label}_stdout.txt"
  local z_err="${ZBUJB_OBLITERATE_PREFIX}${z_idx_str}_${z_label}_stderr.txt"
  local z_exit=0
  "$@" > "${z_out}" 2> "${z_err}" || z_exit=$?
  zbujb_diag_dump_pair "${z_label}" "${z_out}" "${z_err}"
  return ${z_exit}
}

# zbujb_obliterate_windows_namespaces -- run the four-namespace Windows
# obliterate sequence as a series of atomic PowerShell calls. Operates on
# BUJB_workload_user (the project-wide canonical workload identity).
# Each PS call does ONE thing: a probe (emits raw text) or a destructive
# operation (Remove-LocalUser / Remove-Item / wsl.exe userdel). All
# conditional logic lives bash-side via `test ... || { … }` per BCG.
# Probes use only PS cmdlets or wsl.exe with bash-internal `|| true`,
# avoiding the PowerShell native-command stderr-to-terminating-error
# escalation that bites multi-statement absent-tolerant bodies.
#
# Currently instrumented with zbujb_obliterate_diag_dump after every PS
# call to capture per-step output for diagnosing the wrapper round-trip
# (see [diag/baseline] for known-good control reference).
zbujb_obliterate_windows_namespaces() {
  zbujb_sentinel

  local z_state

  # Diagnostic baseline — verify the wrapper round-trip is producing
  # capturable text on this run before evaluating SAM-probe-empty et al.
  # Get-Date returns a deterministic short string; if this comes back
  # empty then ALL subsequent probes are suspect.
  buc_step "    [diag] Baseline wrapper probe (Get-Date)"
  zbujb_admin_powershell "Get-Date -Format 'yyyy-MM-dd HH:mm:ss'" \
      > "${ZBUJB_OBLITERATE_STDOUT}" \
      2> "${ZBUJB_OBLITERATE_STDERR}" \
    || buc_die "Baseline wrapper probe failed — see ${ZBUJB_OBLITERATE_STDERR}"
  zbujb_obliterate_diag_dump baseline

  # SAM entry — Get-LocalUser emits object table if present, empty if
  # absent (SilentlyContinue swallows the not-found error).
  buc_step "    [SAM] Probe Windows local user (${BUJB_workload_user})"
  zbujb_admin_powershell "Get-LocalUser -Name '${BUJB_workload_user}' -ErrorAction SilentlyContinue" \
      > "${ZBUJB_OBLITERATE_STDOUT}" \
      2> "${ZBUJB_OBLITERATE_STDERR}" \
    || buc_die "SAM probe failed — see ${ZBUJB_OBLITERATE_STDERR}"
  zbujb_obliterate_diag_dump sam_probe
  z_state=$(<"${ZBUJB_OBLITERATE_STDOUT}")
  z_state="${z_state//[$'\r\n\t ']/}"
  test -z "${z_state}" || {
    buc_step "    [SAM] Remove Windows local user (${BUJB_workload_user})"
    zbujb_admin_powershell "Remove-LocalUser -Name '${BUJB_workload_user}'" \
        > "${ZBUJB_OBLITERATE_STDOUT}" \
        2> "${ZBUJB_OBLITERATE_STDERR}" \
      || buc_die "Remove-LocalUser failed — see ${ZBUJB_OBLITERATE_STDERR}"
    zbujb_obliterate_diag_dump sam_remove
  }

  # Windows profile directory.
  buc_step "    [Profile] Probe ${BUJB_path_win_user_home}"
  zbujb_admin_powershell "Test-Path '${BUJB_path_win_user_home}'" \
      > "${ZBUJB_OBLITERATE_STDOUT}" \
      2> "${ZBUJB_OBLITERATE_STDERR}" \
    || buc_die "Profile-dir probe failed — see ${ZBUJB_OBLITERATE_STDERR}"
  zbujb_obliterate_diag_dump profile_probe
  z_state=$(<"${ZBUJB_OBLITERATE_STDOUT}")
  z_state="${z_state//[$'\r\n']/}"
  case "${z_state}" in
    True)
      buc_step "    [Profile] Remove ${BUJB_path_win_user_home}"
      zbujb_admin_powershell "Remove-Item -Recurse -Force '${BUJB_path_win_user_home}'" \
          > "${ZBUJB_OBLITERATE_STDOUT}" \
          2> "${ZBUJB_OBLITERATE_STDERR}" \
        || buc_die "Remove ${BUJB_path_win_user_home} failed — see ${ZBUJB_OBLITERATE_STDERR}"
      zbujb_obliterate_diag_dump profile_remove
      ;;
    False)
      ;;
    *)
      buc_die "Test-Path on ${BUJB_path_win_user_home} returned unexpected: '${z_state}'"
      ;;
  esac

  # Cygwin home directory.
  buc_step "    [Cygwin] Probe ${BUJB_path_cygwin_user_home}"
  zbujb_admin_powershell "Test-Path '${BUJB_path_cygwin_user_home}'" \
      > "${ZBUJB_OBLITERATE_STDOUT}" \
      2> "${ZBUJB_OBLITERATE_STDERR}" \
    || buc_die "Cygwin-home probe failed — see ${ZBUJB_OBLITERATE_STDERR}"
  zbujb_obliterate_diag_dump cygwin_probe
  z_state=$(<"${ZBUJB_OBLITERATE_STDOUT}")
  z_state="${z_state//[$'\r\n']/}"
  case "${z_state}" in
    True)
      buc_step "    [Cygwin] Remove ${BUJB_path_cygwin_user_home}"
      zbujb_admin_powershell "Remove-Item -Recurse -Force '${BUJB_path_cygwin_user_home}'" \
          > "${ZBUJB_OBLITERATE_STDOUT}" \
          2> "${ZBUJB_OBLITERATE_STDERR}" \
        || buc_die "Remove ${BUJB_path_cygwin_user_home} failed — see ${ZBUJB_OBLITERATE_STDERR}"
      zbujb_obliterate_diag_dump cygwin_remove
      ;;
    False)
      ;;
    *)
      buc_die "Test-Path on ${BUJB_path_cygwin_user_home} returned unexpected: '${z_state}'"
      ;;
  esac

  # WSL distribution presence — case-scan stripped output for the
  # canonical name (preflight already verified, but stay defensive).
  buc_step "    [WSL] Probe distribution list"
  zbujb_admin_powershell "wsl.exe --list --quiet 2>\$null" \
      > "${ZBUJB_OBLITERATE_STDOUT}" \
      2> "${ZBUJB_OBLITERATE_STDERR}" \
    || buc_die "WSL distro probe failed — see ${ZBUJB_OBLITERATE_STDERR}"
  zbujb_obliterate_diag_dump wsl_distro_list
  z_state=$(<"${ZBUJB_OBLITERATE_STDOUT}")
  z_state="${z_state//$'\r'/}"
  case $'\n'"${z_state}"$'\n' in
    *$'\n'"${BUJB_wsl_distribution}"$'\n'*)
      # Order: orphan home FIRST, then user. userdel -r refuses to remove
      # /home/<user> when it is not owned by <user> (the orphan case from
      # a prior partial garrison) — separating the two keeps userdel a
      # passwd-table-only operation and lets rm -rf handle the home dir
      # unconditionally.
      #
      # bash-inside-wsl handles absent-state via `|| true` so wsl.exe
      # still exits 0 when the target is absent. The `bash -c` argument
      # uses a PS single-quoted string ('' escapes an embedded single
      # quote) so no double quotes appear inside the cmd.exe-level
      # powershell -Command token.
      buc_step "    [WSL] Probe orphan home (${BUJB_path_posix_user_home}) inside ${BUJB_wsl_distribution}"
      zbujb_admin_powershell "wsl.exe --distribution ${BUJB_wsl_distribution} --user root bash -c 'test -e ''${BUJB_path_posix_user_home}'' && echo PRESENT || true'" \
          > "${ZBUJB_OBLITERATE_STDOUT}" \
          2> "${ZBUJB_OBLITERATE_STDERR}" \
        || buc_die "WSL orphan-home probe failed — see ${ZBUJB_OBLITERATE_STDERR}"
      zbujb_obliterate_diag_dump wsl_home_probe
      z_state=$(<"${ZBUJB_OBLITERATE_STDOUT}")
      z_state="${z_state//[$'\r\n']/}"
      test "${z_state}" != "PRESENT" || {
        buc_step "    [WSL] Remove orphan home (${BUJB_path_posix_user_home}) inside ${BUJB_wsl_distribution}"
        zbujb_admin_powershell "wsl.exe --distribution ${BUJB_wsl_distribution} --user root rm -rf '${BUJB_path_posix_user_home}'" \
            > "${ZBUJB_OBLITERATE_STDOUT}" \
            2> "${ZBUJB_OBLITERATE_STDERR}" \
          || buc_die "WSL orphan-home removal failed — see ${ZBUJB_OBLITERATE_STDERR}"
        zbujb_obliterate_diag_dump wsl_home_remove
      }

      buc_step "    [WSL] Probe Linux user (${BUJB_workload_user}) inside ${BUJB_wsl_distribution}"
      zbujb_admin_powershell "wsl.exe --distribution ${BUJB_wsl_distribution} --user root bash -c 'getent passwd ''${BUJB_workload_user}'' 2>/dev/null || true'" \
          > "${ZBUJB_OBLITERATE_STDOUT}" \
          2> "${ZBUJB_OBLITERATE_STDERR}" \
        || buc_die "WSL user probe failed — see ${ZBUJB_OBLITERATE_STDERR}"
      zbujb_obliterate_diag_dump wsl_user_probe
      z_state=$(<"${ZBUJB_OBLITERATE_STDOUT}")
      z_state="${z_state//[$'\r\n']/}"
      test -z "${z_state}" || {
        buc_step "    [WSL] Remove Linux user (${BUJB_workload_user}) inside ${BUJB_wsl_distribution}"
        zbujb_admin_powershell "wsl.exe --distribution ${BUJB_wsl_distribution} --user root userdel '${BUJB_workload_user}'" \
            > "${ZBUJB_OBLITERATE_STDOUT}" \
            2> "${ZBUJB_OBLITERATE_STDERR}" \
          || buc_die "WSL userdel failed for ${BUJB_workload_user} — see ${ZBUJB_OBLITERATE_STDERR}"
        zbujb_obliterate_diag_dump wsl_user_remove
      }
      ;;
  esac
}

# zbujb_obliterate_workload -- letter-agnostic, platform-dispatched destroy
# helper. Single greppable verb covering every namespace a prior garrison
# of any letter could have populated. Idempotent on absent state at every
# step; (letter, platform) compatibility is asserted by garrison entry, so
# the helper dispatches on BURN_PLATFORM alone.
#
# Windows scope (one PowerShell session per atomic operation):
#   - Windows SAM entry (Remove-LocalUser if Get-LocalUser found it)
#   - C:\Users\<user>\ Windows profile directory (Remove-Item if Test-Path)
#   - C:\cygwin64\home\<user>\ Cygwin home (Remove-Item if Test-Path)
#   - WSL-side /home/<user> via wsl.exe userdel -r (gated on rbtww-main
#     present and getent passwd hit). Decisions live bash-side; PS is
#     atomic. See zbujb_obliterate_windows_namespaces.
# Linux scope: native userdel -r and home purge.
# Mac scope: sysadminctl -deleteUser and home purge.
zbujb_obliterate_workload() {
  zbujb_sentinel
  buc_step "  [2/6] Obliterate workload (${BUJB_workload_user}) on ${BURN_PLATFORM}"

  case "${BURN_PLATFORM}" in
    bubep_linux)
      # CDD via the counter-based zbujb_obliterate_run wrapper: per-call
      # forensic capture under ZBUJB_OBLITERATE_PREFIX, no inline
      # redirection boilerplate, no `|| true` (BCG-forbidden silent
      # absorption), no body-side `2>/dev/null`.
      local z_present=0
      zbujb_obliterate_run "user-probe" zbujb_admin_exec_native "id '${BUJB_workload_user}'" || z_present=$?
      if test "${z_present}" -eq 0; then
        zbujb_obliterate_run "userdel"  zbujb_admin_exec_native "sudo -n userdel -r '${BUJB_workload_user}'" \
          || buc_die "obliterate (b/linux): userdel failed for ${BUJB_workload_user} — see ${ZBUJB_OBLITERATE_PREFIX}*userdel*"
      fi
      z_present=0
      zbujb_obliterate_run "home-probe" zbujb_admin_exec_native "test -d '${BUJB_path_posix_user_home}'" || z_present=$?
      if test "${z_present}" -eq 0; then
        zbujb_obliterate_run "rm-home"  zbujb_admin_exec_native "sudo -n rm -rf '${BUJB_path_posix_user_home}'" \
          || buc_die "obliterate (b/linux): rm -rf of ${BUJB_path_posix_user_home} failed — see ${ZBUJB_OBLITERATE_PREFIX}*rm-home*"
      fi
      ;;
    bubep_mac)
      local z_present=0
      zbujb_obliterate_run "user-probe" zbujb_admin_exec_native "id '${BUJB_workload_user}'" || z_present=$?
      if test "${z_present}" -eq 0; then
        zbujb_obliterate_run "userdel"  zbujb_admin_exec_native "sudo -n sysadminctl -deleteUser '${BUJB_workload_user}'" \
          || buc_die "obliterate (b/mac): sysadminctl -deleteUser failed for ${BUJB_workload_user} — see ${ZBUJB_OBLITERATE_PREFIX}*userdel*"
      fi
      z_present=0
      zbujb_obliterate_run "home-probe" zbujb_admin_exec_native "test -d '${BUJB_path_mac_user_home}'" || z_present=$?
      if test "${z_present}" -eq 0; then
        zbujb_obliterate_run "rm-home"  zbujb_admin_exec_native "sudo -n rm -rf '${BUJB_path_mac_user_home}'" \
          || buc_die "obliterate (b/mac): rm -rf of ${BUJB_path_mac_user_home} failed — see ${ZBUJB_OBLITERATE_PREFIX}*rm-home*"
      fi
      ;;
    bubep_windows)
      zbujb_obliterate_windows_namespaces
      ;;
    *)
      buc_die "obliterate: unsupported BURN_PLATFORM '${BURN_PLATFORM}'"
      ;;
  esac
}

# Step 3 -- create the workload account fresh, ssh-only, no privilege.
zbujb_garrison_step3_create() {
  local z_letter="${1:-}"
  buc_step "  [3/6] Create workload (${BUJB_workload_user})"

  case "${z_letter}" in
    b)
      case "${BURN_PLATFORM}" in
        bubep_linux)
          zbujb_admin_exec_native "sudo -n useradd --create-home --shell /bin/bash '${BUJB_workload_user}'" \
            || buc_die "step3 (b/linux): useradd failed for ${BUJB_workload_user}"
          zbujb_admin_exec_native "sudo -n passwd --lock '${BUJB_workload_user}'" \
            || buc_die "step3 (b/linux): passwd --lock failed for ${BUJB_workload_user}"
          ;;
        bubep_mac)
          # Mac uses dscl/sysadminctl; left for in-environment refinement.
          # Operator may need to seat a more idiomatic primary group ID.
          zbujb_admin_exec_native "sudo -n sysadminctl -addUser '${BUJB_workload_user}' -roleAccount" \
            || buc_die "step3 (b/mac): sysadminctl -addUser failed for ${BUJB_workload_user}"
          zbujb_admin_exec_native "sudo -n dscl . -create '${BUJB_path_mac_user_home}' UserShell /bin/bash" \
            || buc_die "step3 (b/mac): dscl UserShell create failed for ${BUJB_path_mac_user_home}"
          ;;
      esac
      ;;
    c)
      # Cygwin reflects Windows user accounts; mint via net.exe with a
      # disabled-password posture (we want ssh-key-only).
      zbujb_admin_exec_cygwin "net.exe user '${BUJB_workload_user}' /add /passwordreq:no /active:yes > /dev/null" \
        || buc_die "step3 (c): net.exe user /add failed for ${BUJB_workload_user}"
      zbujb_admin_exec_cygwin "mkpasswd -l -u '${BUJB_workload_user}' >> /etc/passwd" \
        || buc_die "step3 (c): mkpasswd append to /etc/passwd failed for ${BUJB_workload_user}"
      zbujb_admin_exec_cygwin "mkdir -p '${BUJB_path_posix_user_home}'" \
        || buc_die "step3 (c): mkdir of ${BUJB_path_posix_user_home} failed"
      zbujb_admin_exec_cygwin "chown -R '${BUJB_workload_user}' '${BUJB_path_posix_user_home}'" \
        || buc_die "step3 (c): chown of ${BUJB_path_posix_user_home} failed"
      ;;
    w)
      # Windows-side workload account only. The WSL-side Linux user is
      # provisioned later inside the workload's *own* imported rbtww-main
      # distribution by zbujb_garrison_w_init_wsl — admin's WSL is not
      # the workload's runtime distribution under the redesigned
      # garrison-w (Microsoft per-user WSL constraint, BUSJGW).
      zbujb_admin_exec_wsl "net.exe user '${BUJB_workload_user}' /add /passwordreq:no /active:yes > /dev/null" \
        || buc_die "step3 (w): net.exe user /add failed for ${BUJB_workload_user}"
      # OpenSSH-Win32 silently closes at preauth if the workload SID has
      # no HKLM\...\ProfileList entry. net.exe user /add creates the SAM
      # entry but not the profile registration. Win32-OpenSSH issue #1383.
      # Bash-orchestrated per WSG-PS-5: capture SID, build registry path
      # in bash, then two atomic single-expression PS calls.
      local z_sid z_regkey
      z_sid=$(zbujb_powershell_capture zbujb_privileged "(Get-LocalUser '${BUJB_workload_user}').SID.Value") \
        || buc_die "step 3 (w): could not resolve SID for ${BUJB_workload_user}"
      z_regkey="HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ProfileList\\${z_sid}"
      zbujb_admin_powershell "New-Item -Path '${z_regkey}' -Force | Out-Null"
      zbujb_admin_powershell "New-ItemProperty -Path '${z_regkey}' -Name 'ProfileImagePath' -Value '${BUJB_path_winenv_user_home}' -PropertyType ExpandString -Force | Out-Null"
      ;;
  esac
}

# Step 4 -- write workload authorized_keys with the shell-letter command=
# directive and the workload pubkey (derived locally from the privkey).
zbujb_garrison_step4_place_trust() {
  local z_letter="${1:-}"
  local z_wlhome
  z_wlhome=$(zbujb_workload_home_capture "${z_letter}") \
    || buc_die "step4: workload home unresolvable for letter='${z_letter}' platform='${BURN_PLATFORM}'"

  # Authorized_keys directory differs by letter: b/c land in the workload
  # home; w lands Windows-side under /mnt/c/Users so Windows OpenSSH
  # discovers it natively (the SSH auth boundary is the Windows user).
  local z_authkeys_dir
  case "${z_letter}" in
    b|c) z_authkeys_dir="${z_wlhome}/${BUJB_path_dotssh}" ;;
    w)   z_authkeys_dir="${BUJB_path_wsl_user_ssh}" ;;
  esac
  buc_step "  [4/6] Place workload trust (${z_authkeys_dir}/authorized_keys)"

  # For w letter, step 4 writes a BARE authorized_keys entry (pubkey only,
  # no command= directive) so the SSH-as-workload session opened by
  # zbujb_garrison_w_init_wsl can drop into a normal cmd.exe shell and run
  # `wsl --import` (which would fail under the locked-down command= form
  # because rbtww-main is not yet registered in the workload's HKCU).
  # The command= form is written later by zbujb_garrison_w_lockdown.
  # b/c letters keep the locked-down directive in step 4 as before.
  local z_command_directive=""
  case "${z_letter}" in
    b|c)
      z_command_directive=$(bujb_command_for_capture "${z_letter}") \
        || buc_die "step4: bujb_command_for_capture failed for letter='${z_letter}'"
      ;;
  esac

  ssh-keygen -y -P '' -f "${BURP_WORKLOAD_KEY_FILE}" \
      > "${ZBUJB_PUBKEY_STDOUT_WORK}" \
      2> "${ZBUJB_PUBKEY_STDERR_WORK}" \
    || buc_die "ssh-keygen -y failed for workload key: ${BURP_WORKLOAD_KEY_FILE} — see ${ZBUJB_PUBKEY_STDERR_WORK}"
  local z_pubkey
  z_pubkey=$(<"${ZBUJB_PUBKEY_STDOUT_WORK}")
  z_pubkey="${z_pubkey//$'\n'/}"

  local z_authkeys_line
  if test -n "${z_command_directive}"; then
    z_authkeys_line="${z_command_directive} ${z_pubkey}"$'\n'
  else
    z_authkeys_line="${z_pubkey}"$'\n'
  fi

  local z_sudo_prefix=""
  test "${z_letter}" != "b" || z_sudo_prefix="sudo -n "

  case "${z_letter}" in
    b)
      zbujb_admin_exec_native "${z_sudo_prefix}mkdir -p '${z_authkeys_dir}'" \
        || buc_die "step4 (b): mkdir of ${z_authkeys_dir} failed"
      zbujb_admin_exec_native "${z_sudo_prefix}chmod 700 '${z_authkeys_dir}'" \
        || buc_die "step4 (b): chmod 700 of ${z_authkeys_dir} failed"
      zbujb_admin_exec_native "echo '${z_authkeys_line}' | ${z_sudo_prefix}tee '${z_authkeys_dir}/authorized_keys' > /dev/null" \
        || buc_die "step4 (b): write to ${z_authkeys_dir}/authorized_keys failed"
      zbujb_admin_exec_native "${z_sudo_prefix}chmod 600 '${z_authkeys_dir}/authorized_keys'" \
        || buc_die "step4 (b): chmod 600 of authorized_keys failed"
      zbujb_admin_exec_native "${z_sudo_prefix}chown -R '${BUJB_workload_user}:${BUJB_workload_user}' '${z_authkeys_dir}'" \
        || buc_die "step4 (b): chown of ${z_authkeys_dir} failed"
      ;;
    c)
      # In Cygwin, file ownership/permissions interplay with NTFS ACLs.
      # mkdir/chmod/chown are Cygwin-mediated; run as the admin user
      # without sudo (Windows admins already have privilege).
      zbujb_admin_exec_cygwin "mkdir -p '${z_authkeys_dir}'" \
        || buc_die "step4 (c): mkdir of ${z_authkeys_dir} failed"
      zbujb_admin_exec_cygwin "chmod 700 '${z_authkeys_dir}'" \
        || buc_die "step4 (c): chmod 700 of ${z_authkeys_dir} failed"
      zbujb_admin_exec_cygwin "echo '${z_authkeys_line}' > '${z_authkeys_dir}/authorized_keys'" \
        || buc_die "step4 (c): write to ${z_authkeys_dir}/authorized_keys failed"
      zbujb_admin_exec_cygwin "chmod 600 '${z_authkeys_dir}/authorized_keys'" \
        || buc_die "step4 (c): chmod 600 of authorized_keys failed"
      zbujb_admin_exec_cygwin "chown -R '${BUJB_workload_user}' '${z_authkeys_dir}'" \
        || buc_die "step4 (c): chown of ${z_authkeys_dir} failed"
      ;;
    w)
      printf '%s' "${z_authkeys_line}"                                    \
        | openssl enc -base64 -A                                          \
            > "${ZBUJB_AUTHKEYS_B64_STDOUT}"                              \
            2> "${ZBUJB_AUTHKEYS_B64_STDERR}"                             \
        || buc_die "base64 encode failed for authorized_keys line — see ${ZBUJB_AUTHKEYS_B64_STDERR}"
      local z_authkeys_b64
      z_authkeys_b64=$(<"${ZBUJB_AUTHKEYS_B64_STDOUT}")
      z_authkeys_b64="${z_authkeys_b64//$'\n'/}"

      buc_step "    [diag/curia] z_authkeys_line ${#z_authkeys_line}B; z_authkeys_b64 ${#z_authkeys_b64}B"
      buc_step "    [diag/curia-pubkey] z_pubkey ${#z_pubkey}B: ${z_pubkey}"

      zbujb_place_trust_run "mkdir"            zbujb_admin_exec_wsl    "mkdir -p '${z_authkeys_dir}'"
      zbujb_place_trust_run "chmod-dir"        zbujb_admin_exec_wsl    "chmod 700 '${z_authkeys_dir}'"
      zbujb_place_trust_run "decode-write"     zbujb_admin_exec_wsl    "set -o pipefail; echo '${z_authkeys_b64}' | openssl enc -base64 -d -A > '${z_authkeys_dir}/authorized_keys'"
      zbujb_place_trust_run "chmod-file"       zbujb_admin_exec_wsl    "chmod 600 '${z_authkeys_dir}/authorized_keys'"
      zbujb_place_trust_run "prelock-readback" zbujb_admin_exec_cygwin "cat '${BUJB_path_cyg_user_authkeys}'"

      local z_authkeys_win="${BUJB_path_win_user_authkeys}"
      local z_authkeys_dir_win="${BUJB_path_win_user_ssh}"
      local z_home_win="${BUJB_path_win_user_home}"

      zbujb_place_trust_run "icacls-grant"         zbujb_admin_powershell "icacls '${z_authkeys_win}' /inheritance:r /grant '${BUJB_acl_principal_system}:F' /grant '${BUJB_workload_user}:F'"
      zbujb_place_trust_run "icacls-setowner"      zbujb_admin_powershell "icacls '${z_authkeys_win}' /setowner '${BUJB_workload_user}'"
      zbujb_place_trust_run "icacls-dir-grant"     zbujb_admin_powershell "icacls '${z_authkeys_dir_win}' /inheritance:r /grant '${BUJB_acl_principal_system}:F' /grant '${BUJB_workload_user}:F'"
      zbujb_place_trust_run "icacls-dir-setowner"  zbujb_admin_powershell "icacls '${z_authkeys_dir_win}' /setowner '${BUJB_workload_user}'"
      zbujb_place_trust_run "icacls-home-grant"    zbujb_admin_powershell "icacls '${z_home_win}' /inheritance:r /grant '${BUJB_acl_principal_system}:F' /grant '${BUJB_acl_principal_admins}:F' /grant '${BUJB_workload_user}:F'"
      zbujb_place_trust_run "icacls-home-setowner" zbujb_admin_powershell "icacls '${z_home_win}' /setowner '${BUJB_workload_user}'"
      ;;
  esac
}

# Step 5 -- copy workload privkey to the remote at the shell-letter's
# hardcoded destination path, with workload ownership and 0600 mode.
zbujb_garrison_step5_plant_key() {
  local z_letter="${1:-}"
  local z_wlhome
  z_wlhome=$(zbujb_workload_home_capture "${z_letter}") \
    || buc_die "step5: workload home unresolvable for letter='${z_letter}' platform='${BURN_PLATFORM}'"
  local z_target="${z_wlhome}/${BUJB_workload_keypath}"
  buc_step "  [5/6] Plant workload privkey (${z_target})"

  # Single ssh-stdin pipeline per shell (pace ₢A-AA9): the workload key
  # file flows from BURP_WORKLOAD_KEY_FILE on the curia, over ssh stdin
  # (FD 0), into the remote shell, into install's /dev/stdin, and lands
  # at z_target with mode + owner atomically set by install -D. No
  # plaintext on either side's disk; no remote temp file, trap, or
  # mktemp; no curia-side base64 encode; no key material in argv.
  # See WSG SH-10 for the single-statement-body contract.
  case "${z_letter}" in
    b)
      # Linux/Mac: privileged user needs sudo to write into workload's
      # home; both -o and -g supplied because useradd auto-mints a
      # self-named primary group on Linux/Mac.
      zbujb_admin_exec_native \
          "sudo -n install -D -m 600 -o '${BUJB_workload_user}' -g '${BUJB_workload_user}' /dev/stdin '${z_target}'" \
          < "${BURP_WORKLOAD_KEY_FILE}" \
        || buc_die "step5 (b): failed to plant workload key at ${z_target}"
      ;;
    w)
      # WSL: wsl.exe --user root provides root inside the distribution,
      # no sudo needed. -o and -g supplied (WSL distro is Linux-flavoured;
      # useradd inside it mints a self-named group).
      zbujb_admin_exec_wsl \
          "install -D -m 600 -o '${BUJB_workload_user}' -g '${BUJB_workload_user}' /dev/stdin '${z_target}'" \
          < "${BURP_WORKLOAD_KEY_FILE}" \
        || buc_die "step5 (w): failed to plant workload key at ${z_target}"
      ;;
    c)
      # Cygwin: privileged user is a Windows admin; chown works via NTFS
      # ACL without sudo. -g omitted because Windows accounts (minted via
      # net.exe user /add) do not get an auto-self-named group; passing
      # -g '${BUJB_workload_user}' would fail.
      zbujb_admin_exec_cygwin \
          "install -D -m 600 -o '${BUJB_workload_user}' /dev/stdin '${z_target}'" \
          < "${BURP_WORKLOAD_KEY_FILE}" \
        || buc_die "step5 (c): failed to plant workload key at ${z_target}"
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

# zbujb_garrison_w_export_seed -- export admin's BUJB_wsl_distribution
# (the seed source) to a workload-readable path inside the workload's
# Windows profile. Runs from admin context. The tarball is the sole
# on-disk artifact carrying admin's rbtww-main contents into the
# workload's identity scope; zbujb_garrison_w_seed_cleanup removes it
# after the import has consumed it.
zbujb_garrison_w_export_seed() {
  zbujb_sentinel
  buc_step "  [w-export-seed] Export admin's ${BUJB_wsl_distribution} to workload-readable seed"

  local z_seed_win="${BUJB_path_win_seed_tarball}"

  # PowerShell-routed: single quotes are literal-string delimiters that
  # PS strips before invoking wsl.exe.
  zbujb_w_init_run "wsl-export" zbujb_admin_powershell \
    "wsl.exe --export ${BUJB_wsl_distribution} '${z_seed_win}'" \
    || buc_die "w-export-seed: wsl --export failed (admin's ${BUJB_wsl_distribution} not exportable?) — see ${ZBUJB_W_INIT_PREFIX}*wsl-export*"
}

# zbujb_garrison_w_init_wsl -- SSH-as-workload session that imports the
# workload's own rbtww-main, provisions the inner Linux workload user,
# and plants the workload privkey for outbound auth. The session is a
# real Windows logon for BUJB_workload_user (HKCU mounted), so wsl.exe
# --import writes the registration to the workload's HKCU\Lxss naturally
# rather than admin's. The bare authorized_keys placed by step 4 lets
# this session open without the locked-down command= directive routing
# through an unimported wsl distribution.
#
# Each phase is a separate ssh-as-workload connection, wrapped in
# zbujb_w_init_run for per-call diag capture. Connection setup is fast
# on Windows OpenSSH after first auth; the clarity of one-call-per-step
# outweighs the connection overhead.
zbujb_garrison_w_init_wsl() {
  zbujb_sentinel
  buc_step "  [w-init-wsl] SSH-as-workload: import distribution, provision Linux user, plant privkey"

  local z_seed_win="${BUJB_path_win_seed_tarball}"
  local z_wsl_root_win="${BUJB_path_win_wsl_root}"

  # cmd.exe-routed (workload's default Win32-OpenSSH shell): double-quote
  # Windows paths so cmd.exe's argv parser passes them as single args to
  # wsl.exe. Single quotes would be literal chars (cmd.exe doesn't strip
  # them) and break path resolution.
  zbujb_w_init_run "wsl-import" zbujb_workload_ssh \
    "wsl.exe --import ${BUJB_wsl_distribution} \"${z_wsl_root_win}\" \"${z_seed_win}\" --version 2" \
    || buc_die "w-init-wsl: wsl --import failed (see ${ZBUJB_W_INIT_PREFIX}*wsl-import*)"

  zbujb_w_init_run "wsl-useradd" zbujb_workload_ssh \
    "wsl.exe --distribution ${BUJB_wsl_distribution} --user root bash -c \"useradd --create-home --shell /bin/bash ${BUJB_workload_user}\"" \
    || buc_die "w-init-wsl: useradd failed inside workload's distribution"

  zbujb_w_init_run "wsl-passwd-lock" zbujb_workload_ssh \
    "wsl.exe --distribution ${BUJB_wsl_distribution} --user root bash -c \"passwd --lock ${BUJB_workload_user}\"" \
    || buc_die "w-init-wsl: passwd --lock failed for inner Linux user"

  # Plant the workload privkey inside the workload's WSL distribution
  # via two atomic install ops (pace ₢A-AA9): no curia-side b64 encode,
  # no remote b64 decode, no key material in argv, no `&&`-chained body.
  # The key file flows from BURP_WORKLOAD_KEY_FILE on the curia, over
  # ssh stdin, through wsl.exe, into the inner bash, and into install's
  # /dev/stdin where mode + owner are set atomically. See WSG SH-10 for
  # the single-statement-body contract.
  zbujb_w_init_run "wsl-plant-sshdir" zbujb_workload_ssh \
      "wsl.exe --distribution ${BUJB_wsl_distribution} --user root bash -c \"install -d -m 700 -o ${BUJB_workload_user} -g ${BUJB_workload_user} '${BUJB_path_posix_user_ssh}'\"" \
    || buc_die "w-init-wsl: failed to create ${BUJB_path_posix_user_ssh} inside workload's distribution"

  zbujb_w_init_run "wsl-plant-privkey" zbujb_workload_ssh \
      "wsl.exe --distribution ${BUJB_wsl_distribution} --user root bash -c \"install -m 600 -o ${BUJB_workload_user} -g ${BUJB_workload_user} /dev/stdin '${BUJB_path_posix_user_home}/${BUJB_workload_keypath}'\"" \
      < "${BURP_WORKLOAD_KEY_FILE}" \
    || buc_die "w-init-wsl: failed to plant workload privkey inside workload's distribution"
}

# zbujb_garrison_w_lockdown -- replace the bare workload authorized_keys
# (placed by step 4 for w letter) with the locked-down command= form
# from BUJB_command_w. The rewrite happens via SSH-as-workload + wsl.exe
# bash; the file is workload-owned per step 4's icacls grants, so the
# workload's session has direct write privilege without requiring admin
# ownership transfers.
zbujb_garrison_w_lockdown() {
  zbujb_sentinel
  buc_step "  [w-lockdown] Replace bare authorized_keys with command= directive"

  local z_command_directive
  z_command_directive=$(bujb_command_for_capture w) \
    || buc_die "w-lockdown: bujb_command_for_capture failed for letter='w'"

  ssh-keygen -y -P '' -f "${BURP_WORKLOAD_KEY_FILE}" \
      > "${ZBUJB_PUBKEY_STDOUT_WORK}" \
      2> "${ZBUJB_PUBKEY_STDERR_WORK}" \
    || buc_die "w-lockdown: ssh-keygen -y failed — see ${ZBUJB_PUBKEY_STDERR_WORK}"
  local z_pubkey=$(<"${ZBUJB_PUBKEY_STDOUT_WORK}")
  z_pubkey="${z_pubkey//$'\n'/}"

  local z_authkeys_line="${z_command_directive} ${z_pubkey}"$'\n'

  # Base64 transport so the embedded $SSH_ORIGINAL_COMMAND, $-substitution,
  # and special chars in the directive survive the bash + cmd.exe + wsl
  # argv layers cleanly. Same encode pattern as step 4 w-branch.
  printf '%s' "${z_authkeys_line}"                           \
      | openssl enc -base64 -A                               \
          > "${ZBUJB_AUTHKEYS_B64_STDOUT}"                   \
          2> "${ZBUJB_AUTHKEYS_B64_STDERR}"                  \
    || buc_die "w-lockdown: base64 encode failed for authorized_keys line — see ${ZBUJB_AUTHKEYS_B64_STDERR}"
  local z_authkeys_b64=$(<"${ZBUJB_AUTHKEYS_B64_STDOUT}")
  z_authkeys_b64="${z_authkeys_b64//$'\n'/}"

  local z_authkeys_path="${BUJB_path_wsl_user_authkeys}"
  local z_wsl_body
  z_wsl_body="echo '${z_authkeys_b64}' | openssl enc -base64 -d -A > '${z_authkeys_path}'"

  zbujb_w_init_run "lockdown-write" zbujb_workload_ssh \
    "wsl.exe --distribution ${BUJB_wsl_distribution} --user root bash -c \"${z_wsl_body}\"" \
    || buc_die "w-lockdown: failed to overwrite authorized_keys with command= form"
}

# zbujb_garrison_w_seed_cleanup -- remove the seed tarball placed by
# zbujb_garrison_w_export_seed. Admin context (workload session post-
# lockdown is routed through the command= directive and cannot run
# arbitrary native commands; admin retains full access to the workload
# profile dir per the icacls grant in step 4).
zbujb_garrison_w_seed_cleanup() {
  zbujb_sentinel
  buc_step "  [w-seed-cleanup] Remove seed tarball"

  local z_seed_win="${BUJB_path_win_seed_tarball}"
  zbujb_w_init_run "seed-cleanup" zbujb_admin_powershell \
    "Remove-Item -Path '${z_seed_win}' -Force -ErrorAction SilentlyContinue" \
    || buc_die "w-seed-cleanup: failed to remove seed tarball — see ${ZBUJB_W_INIT_PREFIX}*seed-cleanup*"
}

# Step 6 -- workload-side round-trip validation (knock).
zbujb_garrison_step6_validate() {
  local z_letter="${1:-}"
  buc_step "  [6/6] Validate workload round-trip"

  local z_exit=0
  zbujb_validate_run "knock-ssh" ssh -i "${BURP_WORKLOAD_KEY_FILE}" \
      "${ZBUJB_SSH_BASE_ARGS[@]}"                                    \
      -o BatchMode=yes                                               \
      -o ConnectTimeout=10                                           \
      "${BUJB_workload_user}@${BURN_HOST}"                           \
      true                                                           \
    || z_exit=$?

  if test "${z_exit}" -ne 0; then
    zbujb_validate_run "eventlog-operational" zbujb_admin_powershell "Get-WinEvent -FilterHashtable @{LogName='OpenSSH/Operational'; StartTime=(Get-Date).AddSeconds(-30)} -ErrorAction SilentlyContinue | Format-List TimeCreated, Message | Out-String" || true
    zbujb_validate_run "eventlog-admin"       zbujb_admin_powershell "Get-WinEvent -FilterHashtable @{LogName='OpenSSH/Admin'; StartTime=(Get-Date).AddSeconds(-30)} -ErrorAction SilentlyContinue | Format-List TimeCreated, Message | Out-String"       || true
    zbujb_validate_run "sshd-config"          zbujb_admin_powershell "Get-Content \$env:ProgramData\\ssh\\sshd_config | Out-String"                                                                                                                       || true
    zbujb_validate_run "acl-home"             zbujb_admin_powershell "icacls '${BUJB_path_win_user_home}'"                                                                                                                                                || true
    zbujb_validate_run "acl-dotssh"           zbujb_admin_powershell "icacls '${BUJB_path_win_user_ssh}'"                                                                                                                                                 || true
    zbujb_validate_run "getacl-home"          zbujb_admin_powershell "Get-Acl '${BUJB_path_win_user_home}' | Format-List | Out-String"                                                                                                                    || true
    zbujb_validate_run "getacl-dotssh"        zbujb_admin_powershell "Get-Acl '${BUJB_path_win_user_ssh}' | Format-List | Out-String"                                                                                                                     || true
    zbujb_validate_run "getacl-authkeys"      zbujb_admin_powershell "Get-Acl '${BUJB_path_win_user_authkeys}' | Format-List | Out-String"                                                                                                                || true
    zbujb_validate_run "localuser"            zbujb_admin_powershell "Get-LocalUser '${BUJB_workload_user}' | Format-List | Out-String"                                                                                                                   || true
    zbujb_validate_run "service-sshd"         zbujb_admin_powershell "Get-Service sshd | Format-List | Out-String"                                                                                                                                        || true
    zbujb_validate_run "sshdir-listing"       zbujb_admin_powershell "Get-ChildItem \$env:ProgramData\\ssh -ErrorAction SilentlyContinue | Format-List Name, Length, LastWriteTime | Out-String"                                                          || true
    zbujb_validate_run "limit-blank-password" zbujb_admin_powershell "reg.exe query 'HKLM\\SYSTEM\\CurrentControlSet\\Control\\Lsa' /v LimitBlankPasswordUse 2>\$null | Out-String"                                                                        || true
    zbujb_validate_run "profile-list"         zbujb_admin_powershell "reg.exe query 'HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ProfileList' /s 2>\$null | Out-String"                                                                        || true

    buc_die "Workload round-trip failed (ssh exit ${z_exit}); the new account did not accept its own key."
  fi
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
  zbujb_obliterate_workload
  zbujb_garrison_step3_create        "${z_letter}"
  zbujb_garrison_step4_place_trust   "${z_letter}"
  case "${z_letter}" in
    w)
      # Three-namespace garrison-w (BUSJGW): the workload owns its own
      # WSL distribution. Privileged orchestrator conducts; workload's
      # SSH logon session executes the per-user-HKCU work.
      zbujb_garrison_w_export_seed
      zbujb_garrison_w_init_wsl
      zbujb_garrison_w_lockdown
      zbujb_garrison_w_seed_cleanup
      ;;
    *)
      zbujb_garrison_step5_plant_key   "${z_letter}"
      ;;
  esac
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

  ssh -i "${BURP_PRIVILEGED_KEY_FILE}"           \
      "${ZBUJB_SSH_BASE_ARGS[@]}"                \
      -o BatchMode=no                            \
      -o PreferredAuthentications=publickey,password \
      -o ConnectTimeout=15                       \
      "${BURP_PRIVILEGED_USER}@${BURN_HOST}"     \
      'powershell -NoProfile -File -'            \
      > "${z_stdout_file}"                       \
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

  ssh -i "${BURP_PRIVILEGED_KEY_FILE}"           \
      "${ZBUJB_SSH_BASE_ARGS[@]}"                \
      -o BatchMode=yes                           \
      -o PreferredAuthentications=publickey      \
      -o ConnectTimeout=15                       \
      "${BURP_PRIVILEGED_USER}@${BURN_HOST}"     \
      'powershell -NoProfile -File -'            \
      > "${z_stdout_file}"                       \
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

  # Build the PS hashtable block from BUJB_sshd_hardening so the bash-side
  # tinder is the sole source of truth for directive names + expected
  # values; phase 2's verify and the PS-side rewrite cannot drift.
  local z_ps_directives_block=""
  local z_pair="" z_dir="" z_val=""
  while IFS= read -r z_pair; do
    test -n "${z_pair}" || continue
    z_dir="${z_pair%% *}"
    z_val="${z_pair#* }"
    z_ps_directives_block+="  '${z_dir}' = '${z_val}'"$'\n'
  done <<<"${BUJB_sshd_hardening}"

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
icacls \$adminAuthKeys /inheritance:r /grant '${BUJB_acl_principal_system}:F' /grant '${BUJB_acl_principal_admins}:F' 2>&1 | Out-Null
if (\$LASTEXITCODE -ne 0) { throw "icacls failed (exit \$LASTEXITCODE)" }

# Idempotent sshd_config harden — replace the first matching line for
# each directive (commented or otherwise) and drop subsequent duplicates;
# append directives that are absent. Convergent on re-run.
\$directives = [ordered]@{
${z_ps_directives_block}}

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

  local -r z_distro_dir="${BUJB_path_win_wsl_install_root}\\${BUJB_wsl_distribution}"
  local -r z_tar_path="${BUJB_path_win_wsl_install_root}\\${BUJB_wsl_distribution}.tar"

  buc_step "WSL Install: ${BUJB_wsl_distribution} on ${BURN_HOST} (seed: ${BUJB_wsl_seed_distribution})"

  buc_step "  [1/6] Purge prior state (idempotent: unregister both, remove tar+dir)"
  local z_wsl_list=""
  z_wsl_list=$(zbujb_powershell_capture zbujb_privileged "wsl.exe --list --quiet") \
    || z_wsl_list=""
  if grep -qFx "${BUJB_wsl_distribution}" <<<"${z_wsl_list}"; then
    zbujb_admin_powershell "wsl.exe --unregister ${BUJB_wsl_distribution}" \
      || buc_die "Failed to unregister prior ${BUJB_wsl_distribution}"
  fi
  if grep -qFx "${BUJB_wsl_seed_distribution}" <<<"${z_wsl_list}"; then
    zbujb_admin_powershell "wsl.exe --unregister ${BUJB_wsl_seed_distribution}" \
      || buc_die "Failed to unregister prior ${BUJB_wsl_seed_distribution}"
  fi
  local z_tar_present z_dir_present
  z_tar_present=$(zbujb_powershell_capture zbujb_privileged "Test-Path '${z_tar_path}'") \
    || buc_die "Failed to probe ${z_tar_path}"
  z_dir_present=$(zbujb_powershell_capture zbujb_privileged "Test-Path '${z_distro_dir}'") \
    || buc_die "Failed to probe ${z_distro_dir}"
  if [[ "${z_tar_present}" == "True" ]]; then
    zbujb_admin_powershell "Remove-Item -Force '${z_tar_path}'" \
      || buc_die "Failed to remove prior ${z_tar_path}"
  fi
  if [[ "${z_dir_present}" == "True" ]]; then
    zbujb_admin_powershell "Remove-Item -Recurse -Force '${z_distro_dir}'" \
      || buc_die "Failed to remove prior ${z_distro_dir}"
  fi

  buc_step "  [2/6] Ensure ${BUJB_path_win_wsl_install_root} directory"
  zbujb_admin_powershell "New-Item -ItemType Directory -Path '${BUJB_path_win_wsl_install_root}' -Force | Out-Null" \
    || buc_die "Failed to create ${BUJB_path_win_wsl_install_root}"

  buc_step "  [3/6] Install ${BUJB_wsl_seed_distribution} seed distribution"
  zbujb_admin_powershell "wsl.exe --install --no-launch -d ${BUJB_wsl_seed_distribution}" \
    || buc_die "Failed to install ${BUJB_wsl_seed_distribution} seed"

  buc_step "  [4/6] Export ${BUJB_wsl_seed_distribution} to ${z_tar_path}"
  zbujb_admin_powershell "wsl.exe --export ${BUJB_wsl_seed_distribution} '${z_tar_path}'" \
    || buc_die "Failed to export ${BUJB_wsl_seed_distribution}"

  buc_step "  [5/6] Import ${BUJB_wsl_distribution} from ${z_tar_path}"
  zbujb_admin_powershell "wsl.exe --import ${BUJB_wsl_distribution} '${z_distro_dir}' '${z_tar_path}'" \
    || buc_die "Failed to import ${BUJB_wsl_distribution}"

  buc_step "  [6/6] Cleanup: unregister ${BUJB_wsl_seed_distribution} and remove .tar"
  zbujb_admin_powershell "wsl.exe --unregister ${BUJB_wsl_seed_distribution}" \
    || buc_die "Failed to unregister ${BUJB_wsl_seed_distribution}"
  z_tar_present=$(zbujb_powershell_capture zbujb_privileged "Test-Path '${z_tar_path}'") \
    || buc_die "Failed to probe ${z_tar_path}"
  if [[ "${z_tar_present}" == "True" ]]; then
    zbujb_admin_powershell "Remove-Item -Force '${z_tar_path}'" \
      || buc_die "Failed to remove ${z_tar_path}"
  fi

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
  ssh -i "${BURP_PRIVILEGED_KEY_FILE}"           \
      "${ZBUJB_SSH_BASE_ARGS[@]}"                \
      -o BatchMode=yes                           \
      -o ConnectTimeout=15                       \
      "${BURP_PRIVILEGED_USER}@${BURN_HOST}"     \
      "$@"                                       \
    || z_exit=$?
  return "${z_exit}"
}

# eof
