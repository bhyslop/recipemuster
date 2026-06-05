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
# Recipe Bottle Foundry Core - build-host primitives (wait-build-completion,
# git-metadata, write-script-body, native-path). Relocated verbatim from the
# former rbfc_FoundryCore.sh monolith; sourced by the rbfc kindle-entry (rbfck_)
# so every consumer reaches them unchanged, and sourced directly by the Rust
# fast-path driver and the capture spine. Reads ZRBFC_* kindle constants at call
# time; write-script-body and native-path are kindle-independent and carry no
# sentinel.

set -euo pipefail

# Sourced-guard (silent skip — reached via rbfc and, later, the capture spine)
test -z "${ZRBFCB_SOURCED:-}" || return 0
ZRBFCB_SOURCED=1

zrbfc_wait_build_completion() {
  zrbfc_sentinel

  local z_max_polls="${1:?zrbfc_wait_build_completion: max_polls required}"
  local z_label="${2:?zrbfc_wait_build_completion: label required}"

  buc_step "${z_label}: Waiting for build completion"

  local z_build_id=""
  z_build_id=$(<"${ZRBFC_BUILD_ID_FILE}") || buc_die "No build ID found"
  test -n "${z_build_id}" || buc_die "Build ID file empty"

  buc_log_args 'Get fresh token for polling'
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get GCB OAuth token"

  local z_status="PENDING"
  local z_polls=0
  local z_queued_advisory_shown=0
  local z_consecutive_failures=0
  local z_response_file=""
  local z_code_file=""
  local z_stderr_file=""
  local z_err_check_file=""
  local z_status_check_file=""
  local z_last_good_response=""
  local z_curl_rc=0

  while true; do
    case "${z_status}" in PENDING|QUEUED|WORKING) : ;; *) break;; esac
    sleep "${ZRBFC_BUILD_POLL_INTERVAL_SEC}"

    z_polls=$((z_polls + 1))
    test "${z_polls}" -le "${z_max_polls}" || buc_die "${z_label}: Build timeout after ${z_max_polls} polls"

    z_response_file="${ZRBFC_POLL_RESPONSE_PREFIX}${z_polls}.json"
    z_code_file="${ZRBFC_POLL_CODE_PREFIX}${z_polls}.txt"
    z_stderr_file="${ZRBFC_POLL_STDERR_PREFIX}${z_polls}.txt"
    z_err_check_file="${ZRBFC_POLL_ERR_CHECK_PREFIX}${z_polls}.txt"
    z_status_check_file="${ZRBFC_POLL_STATUS_PREFIX}${z_polls}.txt"

    buc_log_args "Fetch build status (poll ${z_polls}/${z_max_polls})"
    z_curl_rc=0
    rbuh_request "GET" "${ZRBFC_GCB_PROJECT_BUILDS_URL}/${z_build_id}" \
                      "${z_token}"                                          \
                      "${z_response_file}" "${z_code_file}" "${z_stderr_file}" \
      || z_curl_rc=$?

    if test "${z_curl_rc}" -ne 0; then
      z_consecutive_failures=$((z_consecutive_failures + 1))
      buc_warn "Curl failed (rc=${z_curl_rc}; ${z_consecutive_failures}/${ZRBFC_BUILD_POLL_RETRY_TOLERANCE} consecutive) — see ${z_stderr_file}"
      buc_log_pipe < "${z_stderr_file}"
      test "${z_consecutive_failures}" -ge "${ZRBFC_BUILD_POLL_RETRY_TOLERANCE}" \
        && buc_die "Failed to get build status after ${ZRBFC_BUILD_POLL_RETRY_TOLERANCE} consecutive failures (last rc=${z_curl_rc}; see ${z_stderr_file})"
      continue
    fi

    if ! test -s "${z_response_file}"; then
      z_consecutive_failures=$((z_consecutive_failures + 1))
      buc_warn "Empty response (poll ${z_polls}; ${z_consecutive_failures}/${ZRBFC_BUILD_POLL_RETRY_TOLERANCE} consecutive) — see ${z_response_file}"
      test "${z_consecutive_failures}" -ge "${ZRBFC_BUILD_POLL_RETRY_TOLERANCE}" \
        && buc_die "Empty build status after ${ZRBFC_BUILD_POLL_RETRY_TOLERANCE} consecutive failures"
      continue
    fi

    jq -r '.error.code // empty' "${z_response_file}" > "${z_err_check_file}" 2>/dev/null
    if test -s "${z_err_check_file}"; then
      z_consecutive_failures=$((z_consecutive_failures + 1))
      buc_warn "HTTP error $(<"${z_err_check_file}") (poll ${z_polls}; ${z_consecutive_failures}/${ZRBFC_BUILD_POLL_RETRY_TOLERANCE} consecutive) — see ${z_response_file}"
      test "${z_consecutive_failures}" -ge "${ZRBFC_BUILD_POLL_RETRY_TOLERANCE}" \
        && buc_die "HTTP errors after ${ZRBFC_BUILD_POLL_RETRY_TOLERANCE} consecutive failures"
      continue
    fi

    z_consecutive_failures=0

    jq -r '.status' "${z_response_file}" > "${z_status_check_file}" || buc_die "Failed to extract status (poll ${z_polls})"
    z_status=$(<"${z_status_check_file}")
    test -n "${z_status}" || buc_die "Status is empty (poll ${z_polls})"

    z_last_good_response="${z_response_file}"

    buc_info "${z_label}: ${z_status} (poll ${z_polls}/${z_max_polls})"

    if test "${z_status}" = "QUEUED" && test "${z_polls}" -ge 20 && test "${z_queued_advisory_shown}" = "0"; then
      z_queued_advisory_shown=1
      buc_warn "Build queued longer than normal — another build may be holding the private pool"
      buc_tabtarget "${RBZ_QUOTA_BUILD}"
    fi
  done

  test -n "${z_last_good_response}" \
    || buc_die "${z_label}: no successful poll observed"
  cp "${z_last_good_response}" "${ZRBFC_BUILD_STATUS_FILE}" \
    || buc_die "Failed to register winner response at ${ZRBFC_BUILD_STATUS_FILE}"

  test "${z_status}" = "SUCCESS" || buc_die "${z_label}: Build failed with status: ${z_status}"

  # Extract build wall-clock timing from terminal status response
  jq -r '.startTime // empty' "${ZRBFC_BUILD_STATUS_FILE}" > "${ZRBFC_BUILD_START_FILE}"
  jq -r '.finishTime // empty' "${ZRBFC_BUILD_STATUS_FILE}" > "${ZRBFC_BUILD_FINISH_FILE}"
  local z_start_time=""
  z_start_time=$(<"${ZRBFC_BUILD_START_FILE}")
  local z_finish_time=""
  z_finish_time=$(<"${ZRBFC_BUILD_FINISH_FILE}")

  if test -n "${z_start_time}" && test -n "${z_finish_time}"; then
    local z_start_clean="${z_start_time%%.*}"
    z_start_clean="${z_start_clean%%Z}"
    local z_finish_clean="${z_finish_time%%.*}"
    z_finish_clean="${z_finish_clean%%Z}"
    local z_start_epoch=""
    z_start_epoch=$(date -d "${z_start_clean}Z" '+%s' 2>/dev/null) \
      || z_start_epoch=$(date -j -f '%Y-%m-%dT%H:%M:%S' "${z_start_clean}" '+%s' 2>/dev/null) \
      || z_start_epoch=""
    local z_finish_epoch=""
    z_finish_epoch=$(date -d "${z_finish_clean}Z" '+%s' 2>/dev/null) \
      || z_finish_epoch=$(date -j -f '%Y-%m-%dT%H:%M:%S' "${z_finish_clean}" '+%s' 2>/dev/null) \
      || z_finish_epoch=""
    if test -n "${z_start_epoch}" && test -n "${z_finish_epoch}"; then
      local -r z_duration=$((z_finish_epoch - z_start_epoch))
      local -r z_minutes=$((z_duration / 60))
      local -r z_seconds=$((z_duration % 60))
      buc_info "${z_label}: Wall clock ${z_minutes}m ${z_seconds}s"
    fi
  fi

  buc_success "${z_label}: Build completed successfully"
}

# Internal: capture git metadata to module temp files (idempotent)
# No args — reads from git, writes to ZRBFC_GIT_*_FILE kindle constants
zrbfc_ensure_git_metadata() {
  zrbfc_sentinel

  # Idempotent — skip if already captured
  test ! -s "${ZRBFC_GIT_COMMIT_FILE}" || return 0

  buc_log_args "Capturing git metadata to temp files"

  local -r z_remote_file="${ZRBFC_GIT_PREFIX}remote.txt"
  local -r z_url_file="${ZRBFC_GIT_PREFIX}url.txt"

  git rev-parse HEAD > "${ZRBFC_GIT_COMMIT_FILE}" \
    || buc_die "Failed to get git commit"
  test -s "${ZRBFC_GIT_COMMIT_FILE}" || buc_die "Empty git commit file"

  git rev-parse --abbrev-ref HEAD > "${ZRBFC_GIT_BRANCH_FILE}" \
    || buc_die "Failed to get git branch"
  test -s "${ZRBFC_GIT_BRANCH_FILE}" || buc_die "Empty git branch file"

  git remote > "${z_remote_file}" \
    || buc_die "Failed to list git remotes"
  local z_remote=""
  read -r z_remote < "${z_remote_file}" \
    || buc_die "Failed to read git remote from ${z_remote_file}"
  test -n "${z_remote}" || buc_die "No git remotes found"

  git config --get "remote.${z_remote}.url" > "${z_url_file}" \
    || buc_die "Failed to get git repo URL"

  local z_url=""
  z_url=$(<"${z_url_file}")
  test -n "${z_url}" || buc_die "Empty git repo URL from ${z_url_file}"
  local z_repo="${z_url#*://*/}"
  z_repo="${z_repo%.git}"
  echo "${z_repo}" > "${ZRBFC_GIT_REPO_FILE}" \
    || buc_die "Failed to write derived git repo"
}

# Internal: write a script's body (everything after the shebang line) to a file
# using only builtins — the portable replacement for `tail -n +2 src > dst`.
# Returns non-zero if the source is unreadable or the destination unwritable.
zrbfc_write_script_body() {
  local -r z_src="$1"
  local -r z_dst="$2"
  local z_line
  local z_seen_shebang=""
  test -r "${z_src}" || return 1
  : > "${z_dst}"     || return 1
  while IFS= read -r z_line || [ -n "${z_line}" ]; do
    if [ -z "${z_seen_shebang}" ]; then
      z_seen_shebang=1
      continue
    fi
    printf '%s\n' "${z_line}" >> "${z_dst}"
  done < "${z_src}"
  return 0
}

# Internal: expand "#@rbgjs_include <name>" markers in a step body file IN PLACE.
# Each marker line (any leading indentation tolerated) is replaced by the body of
# <snippets_dir>/rbgjs-<name>.sh with its leading shebang stripped — the same
# shebang-strip rule zrbfc_write_script_body applies. A body with no markers is
# rewritten unchanged (a no-op), so every assembler may call this for every step.
# This is the host side of the shared cloud-step library (RBSCJ "Composed-snippet
# library"): a snippet reads shell vars the kind sets before the marker and is
# blind to substitution names, which is what lets one snippet serve callers with
# disjoint _RBGx_ substitution sets. Pure primitive — args only, no kindle state,
# no sentinel — so it stays unit-testable alongside zrbfc_write_script_body.
# Returns non-zero if the body is unreadable, the snippets dir is missing, a
# marker names no snippet, or a named snippet file is absent (crash-fast: the
# caller dies, no silent skip).
# Args: body_file snippets_dir
zrbfc_expand_includes() {
  local -r z_body_file="$1"
  local -r z_snippets_dir="$2"
  test -f "${z_body_file}"    || return 1
  test -d "${z_snippets_dir}" || return 1

  local -r z_tmp="${z_body_file}.expanded"
  : > "${z_tmp}" || return 1

  local z_line=""
  local z_trimmed=""
  local z_name=""
  local z_snippet=""
  local z_sline=""
  local z_seen_shebang=""
  while IFS= read -r z_line || [ -n "${z_line}" ]; do
    z_trimmed="${z_line#"${z_line%%[![:space:]]*}"}"
    case "${z_trimmed}" in
      '#@rbgjs_include '*)
        z_name="${z_trimmed#'#@rbgjs_include '}"
        z_name="${z_name%%[[:space:]]*}"
        test -n "${z_name}" || return 1
        z_snippet="${z_snippets_dir}/rbgjs-${z_name}.sh"
        test -f "${z_snippet}" || return 1
        z_seen_shebang=""
        while IFS= read -r z_sline || [ -n "${z_sline}" ]; do
          if [ -z "${z_seen_shebang}" ]; then
            z_seen_shebang=1
            case "${z_sline}" in '#!'*) continue ;; esac
          fi
          printf '%s\n' "${z_sline}" >> "${z_tmp}"
        done < "${z_snippet}"
        ;;
      *)
        printf '%s\n' "${z_line}" >> "${z_tmp}"
        ;;
    esac
  done < "${z_body_file}"

  mv "${z_tmp}" "${z_body_file}" || return 1
  return 0
}

# Internal: normalize a path argument for a Windows-native build tool (docker
# under Cygwin). A Windows-native binary reads a Cygwin /cygdrive/X/... path as a
# literal Windows path and reports "does not exist"; hand it the drive-letter
# form (X:/... — forward slashes, which Windows accepts). Pure /cygdrive
# parameter expansion, no cygpath, mirroring RBTDRX's fast path and zrbte_kindle.
# Gated on BURD_OSTYPE (the dispatch-synthesized platform fact): off Cygwin every
# path is emitted unchanged. A relative or already-native path passes through; a
# bare-absolute POSIX path (leading / but not /cygdrive) is an unsurveyed shape
# and returns 1 so the caller dies. Kindle-independent — reads only its argument
# and BURD_OSTYPE — so, like zrbfc_write_script_body, it carries no sentinel and
# stays unit-testable without the foundry kindle chain. Retire when the foundry
# builds as a true Cygwin binary.
zrbfc_native_path_capture() {
  local -r z_path="${1:?zrbfc_native_path_capture: path required}"

  if test "${BURD_OSTYPE:-}" != "cygwin"; then
    printf '%s\n' "${z_path}"
    return 0
  fi

  case "${z_path}" in
    /cygdrive/?/*)
      local -r z_drive_rest="${z_path#/cygdrive/}"
      local -r z_drive="${z_drive_rest%%/*}"
      local -r z_drive_tail="${z_drive_rest#"${z_drive}/"}"
      printf '%s\n' "${z_drive}:/${z_drive_tail}"
      ;;
    /*)
      return 1
      ;;
    *)
      printf '%s\n' "${z_path}"
      ;;
  esac
}

# eof
