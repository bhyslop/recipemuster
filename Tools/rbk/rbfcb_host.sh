#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
# fast-path driver and the capture spine. Reads ZRBFC0_* kindle constants at call
# time; write-script-body and native-path are kindle-independent and carry no
# sentinel.

set -euo pipefail

# The gate codes this module rejects with, from rb's generated constants. The
# sourced file carries its own inclusion guard, so a sibling sourcing it costs
# nothing.
source "${BASH_SOURCE[0]%/*}/rbcg_breviary.sh" || buc_die_now "Failed to source rbcg_breviary.sh"

# Sourced-guard (silent skip — reached via rbfc and, later, the capture spine)
test -z "${ZRBFCB_SOURCED:-}" || return 0
ZRBFCB_SOURCED=1

# zrbfcb_redon_tick — judge one mid-flight re-don attempt, on the poll's
# cadence, by the status of the don its caller just ran: the don alone, never
# the avow-folding accessor, so a lapsed sederunt fails loud here instead of
# re-entering the interactive avowal mid-loop. The fresh mantle token rides
# the caller's local, assigned from the don capture's output, and never this
# function. Called in the process frame, never `$()`: it announces on the
# progress stream and its deaths must exit the run. Status 0 announces the
# re-don; a don failed transiently while the sederunt is still live warns, and
# the caller keeps its prior token and retries next poll (the status poll's
# consecutive-failure counter stays the backstop). A lapsed sederunt dies via
# buc_die_now with the novate advisory; the don's admission deficit
# buc_rejects on its band. Sentinel-free by design — reads no ZRBFC kindle
# state (callees guard their own kindles) — so the lapse branch stays
# unit-testable without a live build.
zrbfcb_redon_tick() {
  local -r z_label="${1:?zrbfcb_redon_tick: label required}"
  local -r z_poll_tag="${2:?zrbfcb_redon_tick: poll tag required}"
  local -r z_don_rc="${3:-}"
  test -n "${z_don_rc}" || buc_die_now "zrbfcb_redon_tick: status required"

  if test "${z_don_rc}" -eq 0; then
    buc_info "${z_label}: Re-donned the director mantle mid-flight (${z_poll_tag})"
    return 0
  fi

  test "${z_don_rc}" -ne "${RBCG_ADMISSION_DENTIL}" \
    || buc_reject "${RBCG_ADMISSION_DENTIL}" "${z_label}: re-don denied mid-build — the director admission was revoked while the build ran; brevet the citizen back onto the mantle, then re-run"

  if zrbau_sederunt_live_predicate; then
    buc_warn "${z_label}: mid-flight re-don failed transiently (${z_poll_tag}) — keeping the prior token; retrying next poll"
    return 0
  fi

  buc_die_now "${z_label}: ${RBCC_noun_sederunt} lapsed mid-build — the mantle cannot be re-donned; open a fresh ${RBCC_noun_sederunt} with rbw-aN, then re-run"
}

# zrbfcb_failure_tail — best-effort view of why a build ended short of SUCCESS:
# the failure detail and the steps that did not succeed, from the registered
# status response, then the newest Cloud Logging entries of the build in time
# order, each led by its step label (a step's own lines carry theirs already).
# Build steps log CLOUD_LOGGING_ONLY, so this is the host's only view of a cloud
# step's own words. Never dies and never touches the verdict: a read that fails,
# is denied, or finds nothing yet ingested warns so and returns 0, leaving the
# caller's status die to stand. Emits through buc, the stream the caller's die
# rides. Reads ZRBFC0_BUILD_STATUS_FILE, registered by the caller.
zrbfcb_failure_tail() {
  local -r z_label="${1:?zrbfcb_failure_tail: label required}"
  local -r z_build_id="${2:?zrbfcb_failure_tail: build id required}"
  local -r zSECRET_token="${3:-}"
  test -n "${zSECRET_token}" || buc_die_now "zrbfcb_failure_tail: token required"

  local -r z_page_size=40
  local -r z_attempts=3
  local -r z_prefix="${BURD_TEMP_DIR}/rbfc_failure_"
  local -r z_detail_file="${z_prefix}detail.txt"
  local -r z_detail_stderr="${z_prefix}detail_stderr.txt"
  local -r z_body_file="${z_prefix}log_body.json"
  local -r z_body_stderr="${z_prefix}log_body_stderr.txt"
  local -r z_url="${RBGC_API_ROOT_LOGGING}${RBGC_LOGGING_V2}${RBGC_LOGGING_ENTRIES_LIST_SUFFIX}"
  local -r z_log_name="projects/${RBGD_GCB_PROJECT_ID}/logs/cloudbuild"

  local z_line=""
  local z_lines=()
  local z_i=0

  local z_detail_status=0
  jq -r '
      (.failureInfo.detail // empty),
      ( (.steps // []) | to_entries[]
        | select(.value.status != "SUCCESS" and .value.status != "QUEUED" and .value.status != "STATUS_UNKNOWN")
        | "step #\(.key) \(.value.id // .value.name // "?"): \(.value.status)" )
    ' "${ZRBFC0_BUILD_STATUS_FILE}" > "${z_detail_file}" 2>"${z_detail_stderr}" \
    || z_detail_status=$?
  if test "${z_detail_status}" -eq 0; then
    while IFS= read -r z_line || test -n "${z_line}"; do
      z_lines+=("${z_line}")
    done < "${z_detail_file}"
    for z_i in "${!z_lines[@]}"; do
      buc_warn "${z_label}: ${z_lines[${z_i}]}"
    done
  else
    buc_warn "${z_label}: the build status carries no readable failure detail (see ${z_detail_stderr})"
  fi

  local z_body_status=0
  jq -n                                                                                        \
     --arg     proj   "projects/${RBGD_GCB_PROJECT_ID}"                                        \
     --arg     filter "logName=\"${z_log_name}\" AND resource.type=\"build\" AND resource.labels.build_id=\"${z_build_id}\"" \
     --argjson size   "${z_page_size}"                                                         \
     '{resourceNames: [$proj], filter: $filter, orderBy: "timestamp desc", pageSize: $size}'   \
     > "${z_body_file}" 2>"${z_body_stderr}" \
    || z_body_status=$?
  if test "${z_body_status}" -ne 0; then
    buc_warn "${z_label}: could not compose the build log request (see ${z_body_stderr}) — no step log tail"
    return 0
  fi

  local z_attempt=0
  local z_resp_file=""
  local z_code_file=""
  local z_curl_stderr=""
  local z_tail_file=""
  local z_tail_stderr=""
  local z_code=""
  local z_code_status=0
  local z_curl_status=0
  local z_tail_status=0
  while :; do
    z_attempt=$((z_attempt + 1))
    z_resp_file="${z_prefix}log_${z_attempt}.json"
    z_code_file="${z_prefix}log_${z_attempt}_code.txt"
    z_curl_stderr="${z_prefix}log_${z_attempt}_curl_stderr.txt"
    z_tail_file="${z_prefix}log_${z_attempt}_tail.txt"
    z_tail_stderr="${z_prefix}log_${z_attempt}_tail_stderr.txt"

    z_curl_status=0
    rbuh_request "POST" "${z_url}" "${zSECRET_token}" \
                 "${z_resp_file}" "${z_code_file}" "${z_curl_stderr}" "${z_body_file}" \
      || z_curl_status=$?
    if test "${z_curl_status}" -ne 0; then
      buc_warn "${z_label}: the build log read failed (curl exit ${z_curl_status}; see ${z_curl_stderr}) — no step log tail"
      return 0
    fi

    z_code_status=0
    z_code=$(<"${z_code_file}") || z_code_status=$?
    test "${z_code_status}" -eq 0 || z_code=""
    case "${z_code}" in
      200) : ;;
      403) buc_warn "${z_label}: the build log read was denied (HTTP 403) — the director cannot read Cloud Logging on ${RBGD_GCB_PROJECT_ID}; no step log tail"
           return 0 ;;
      *)   buc_warn "${z_label}: the build log read failed (HTTP ${z_code:-none}; see ${z_resp_file}) — no step log tail"
           return 0 ;;
    esac

    z_tail_status=0
    jq -r '
        (.entries // []) | reverse | .[]
        | (.labels.build_step // "") as $step
        | (.textPayload // "") as $text
        | if $step == "" or ($text | startswith($step)) then $text else "\($step): \($text)" end
      ' "${z_resp_file}" > "${z_tail_file}" 2>"${z_tail_stderr}" \
      || z_tail_status=$?
    if test "${z_tail_status}" -ne 0; then
      buc_warn "${z_label}: the build log response was unreadable (see ${z_resp_file} and ${z_tail_stderr}) — no step log tail"
      return 0
    fi

    test ! -s "${z_tail_file}" || break

    if test "${z_attempt}" -ge "${z_attempts}"; then
      buc_warn "${z_label}: no build log entries ingested after ${z_attempts} reads — no step log tail"
      return 0
    fi
    sleep "${ZRBFC0_BUILD_POLL_INTERVAL_SEC}" || buc_die_now "${z_label}: sleep failed between build log reads"
  done

  z_lines=()
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_lines+=("${z_line}")
  done < "${z_tail_file}"

  buc_info "${z_label}: build log tail — newest ${z_page_size} entries of build ${z_build_id}:"
  for z_i in "${!z_lines[@]}"; do
    buc_info "  ${z_lines[${z_i}]}"
  done
}

zrbfcb_wait_build_completion() {
  zrbfc0_sentinel

  local z_max_polls="${1:?zrbfcb_wait_build_completion: max_polls required}"
  local z_label="${2:?zrbfcb_wait_build_completion: label required}"

  buc_step "${z_label}: Waiting for build completion"

  local z_build_id=""
  z_build_id=$(<"${ZRBFC0_BUILD_ID_FILE}") || buc_die_now "No build ID found"
  test -n "${z_build_id}" || buc_die_now "Build ID file empty"

  buc_log_args 'Get fresh token for polling'
  local zSECRET_token=""
  zSECRET_token=$(rbau_token_capture "${RBCC_mantle_director}") || buc_die_now "Failed to get GCB OAuth token"

  # Re-don cadence — kindled default, or the BURE test seam's override so a
  # short real build exercises the tick (a malformed
  # value dies loud, any other tweak occupying the slot rides inert).
  local z_redon_cadence="${ZRBFC0_BUILD_POLL_REDON_CADENCE}"
  if test "${BURE_TWEAK_NAME:-}" = "${RBCC_tweak_redon_cadence}"; then
    z_redon_cadence="${BURE_TWEAK_VALUE:-}"
    [[ "${z_redon_cadence}" =~ ^[1-9][0-9]*$ ]] \
      || buc_die_now "redon cadence tweak: BURE_TWEAK_VALUE must be a positive poll count, got '${z_redon_cadence}'"
    buc_log_args "redon cadence tweak: cadence forced to ${z_redon_cadence} polls"
  fi

  local z_status="PENDING"
  local z_polls=0
  local z_polls_since_don=0
  local z_redon_rc=0
  local zSECRET_minted=""
  local z_queue_polls=0
  local z_exec_polls=0
  local z_seen_working=0
  local z_phase=""
  local z_phase_polls=0
  local z_phase_ceiling=0
  local z_consecutive_failures=0
  local z_response_file=""
  local z_code_file=""
  local z_stderr_file=""
  local z_err_check_file=""
  local z_err_check_stderr_file=""
  local z_status_check_file=""
  local z_last_good_response=""
  local z_curl_rc=0

  while true; do
    case "${z_status}" in PENDING|QUEUED|WORKING) : ;; *) break;; esac
    sleep "${ZRBFC0_BUILD_POLL_INTERVAL_SEC}" || buc_die_now "${z_label}: sleep failed between status polls"

    z_polls=$((z_polls + 1))

    # Two clocks: queue polls (pre-WORKING) charge the shared queue ceiling;
    # execution polls (post first-WORKING) charge the per-kind z_max_polls.
    if test "${z_seen_working}" -eq 0; then
      z_queue_polls=$((z_queue_polls + 1))
      z_phase="queue"
      z_phase_polls="${z_queue_polls}"
      z_phase_ceiling="${ZRBFC0_BUILD_POLL_CEILING_QUEUE}"
      if test "${z_queue_polls}" -gt "${ZRBFC0_BUILD_POLL_CEILING_QUEUE}"; then
        buc_tabtarget "${RBZ_QUOTA_BUILD}"
        buc_die_now "${z_label}: pool never took the build — still queued after ${ZRBFC0_BUILD_POLL_CEILING_QUEUE} queue polls; the worker pool never started execution"
      fi
    else
      z_exec_polls=$((z_exec_polls + 1))
      z_phase="exec"
      z_phase_polls="${z_exec_polls}"
      z_phase_ceiling="${z_max_polls}"
      test "${z_exec_polls}" -le "${z_max_polls}" \
        || buc_die_now "${z_label}: Build timeout after ${z_max_polls} execution polls"
    fi

    # Cadence tick — re-don before the status fetch so the fetch always rides
    # a token younger than the mantle ceiling. An empty result is the tick's
    # tolerated-transient outcome: counter untouched, and the since-don count
    # stays past the cadence so the next poll retries.
    z_polls_since_don=$((z_polls_since_don + 1))
    if test "${z_polls_since_don}" -ge "${z_redon_cadence}"; then
      z_redon_rc=0
      zSECRET_minted=$(rbau_don_capture "${RBCC_mantle_director}") || z_redon_rc=$?
      zrbfcb_redon_tick "${z_label}" "poll ${z_polls}; cadence ${z_redon_cadence}" "${z_redon_rc}"
      if test "${z_redon_rc}" -eq 0; then
        zSECRET_token="${zSECRET_minted}"
        z_polls_since_don=0
      fi
    fi

    z_response_file="${ZRBFC0_POLL_RESPONSE_PREFIX}${z_polls}.json"
    z_code_file="${ZRBFC0_POLL_CODE_PREFIX}${z_polls}.txt"
    z_stderr_file="${ZRBFC0_POLL_STDERR_PREFIX}${z_polls}.txt"
    z_err_check_file="${ZRBFC0_POLL_ERR_CHECK_PREFIX}${z_polls}.txt"
    z_status_check_file="${ZRBFC0_POLL_STATUS_PREFIX}${z_polls}.txt"

    buc_log_args "Fetch build status (poll ${z_polls}; ${z_phase} ${z_phase_polls}/${z_phase_ceiling})"
    z_curl_rc=0
    rbuh_request "GET" "${ZRBFC0_GCB_PROJECT_BUILDS_URL}/${z_build_id}" \
                      "${zSECRET_token}"                                          \
                      "${z_response_file}" "${z_code_file}" "${z_stderr_file}" \
      || z_curl_rc=$?

    if test "${z_curl_rc}" -ne 0; then
      z_consecutive_failures=$((z_consecutive_failures + 1))
      buc_warn "Curl failed (rc=${z_curl_rc}; ${z_consecutive_failures}/${ZRBFC0_BUILD_POLL_RETRY_TOLERANCE} consecutive) — see ${z_stderr_file}"
      buc_log_pipe < "${z_stderr_file}"
      test "${z_consecutive_failures}" -lt "${ZRBFC0_BUILD_POLL_RETRY_TOLERANCE}" \
        || buc_die_now "Failed to get build status after ${ZRBFC0_BUILD_POLL_RETRY_TOLERANCE} consecutive failures (last rc=${z_curl_rc}; see ${z_stderr_file})"
      continue
    fi

    if ! test -s "${z_response_file}"; then
      z_consecutive_failures=$((z_consecutive_failures + 1))
      buc_warn "Empty response (poll ${z_polls}; ${z_consecutive_failures}/${ZRBFC0_BUILD_POLL_RETRY_TOLERANCE} consecutive) — see ${z_response_file}"
      test "${z_consecutive_failures}" -lt "${ZRBFC0_BUILD_POLL_RETRY_TOLERANCE}" \
        || buc_die_now "Empty build status after ${ZRBFC0_BUILD_POLL_RETRY_TOLERANCE} consecutive failures"
      continue
    fi

    z_err_check_stderr_file="${ZRBFC0_POLL_ERR_CHECK_PREFIX}${z_polls}_stderr.txt"
    jq -r '.error.code // empty' "${z_response_file}" > "${z_err_check_file}" 2>"${z_err_check_stderr_file}" \
      || buc_die_now "Failed to read error code from build status (poll ${z_polls}) — see ${z_err_check_stderr_file}"
    if test -s "${z_err_check_file}"; then
      z_consecutive_failures=$((z_consecutive_failures + 1))
      local z_err_code
      z_err_code=$(<"${z_err_check_file}") || buc_die_now "Failed to read: ${z_err_check_file}"
      buc_warn "HTTP error ${z_err_code} (poll ${z_polls}; ${z_consecutive_failures}/${ZRBFC0_BUILD_POLL_RETRY_TOLERANCE} consecutive) — see ${z_response_file}"
      test "${z_consecutive_failures}" -lt "${ZRBFC0_BUILD_POLL_RETRY_TOLERANCE}" \
        || buc_die_now "HTTP errors after ${ZRBFC0_BUILD_POLL_RETRY_TOLERANCE} consecutive failures"
      continue
    fi

    z_consecutive_failures=0

    jq -r '.status' "${z_response_file}" > "${z_status_check_file}" || buc_die_now "Failed to extract status (poll ${z_polls})"
    z_status=$(<"${z_status_check_file}") || buc_die_now "Failed to read: ${z_status_check_file}"
    test -n "${z_status}" || buc_die_now "Status is empty (poll ${z_polls})"

    z_last_good_response="${z_response_file}"

    buc_info "${z_label}: ${z_status} (poll ${z_polls}; ${z_phase} ${z_phase_polls}/${z_phase_ceiling})"

    if test "${z_status}" = "WORKING" && test "${z_seen_working}" -eq 0; then
      z_seen_working=1
      buc_info "${z_label}: QUEUED→WORKING after ${z_queue_polls} queue polls (execution budget ${z_max_polls} polls)"
    fi

    if test "${z_seen_working}" -eq 0 && test "$(( z_queue_polls % 20 ))" -eq 0; then
      buc_warn "Build queued longer than normal (${z_queue_polls} queue polls) — another build may be holding the private pool"
      buc_tabtarget "${RBZ_QUOTA_BUILD}"
    fi
  done

  test -n "${z_last_good_response}" \
    || buc_die_now "${z_label}: no successful poll observed"
  cp "${z_last_good_response}" "${ZRBFC0_BUILD_STATUS_FILE}" \
    || buc_die_now "Failed to register winner response at ${ZRBFC0_BUILD_STATUS_FILE}"

  if test "${z_status}" != "SUCCESS"; then
    zrbfcb_failure_tail "${z_label}" "${z_build_id}" "${zSECRET_token}"
    buc_die_now "${z_label}: Build failed with status: ${z_status}"
  fi

  # Extract build wall-clock timing from terminal status response
  jq -r '.startTime // empty' "${ZRBFC0_BUILD_STATUS_FILE}" > "${ZRBFC0_BUILD_START_FILE}" \
    || buc_die_now "Failed to extract build start time from ${ZRBFC0_BUILD_STATUS_FILE}"
  jq -r '.finishTime // empty' "${ZRBFC0_BUILD_STATUS_FILE}" > "${ZRBFC0_BUILD_FINISH_FILE}" \
    || buc_die_now "Failed to extract build finish time from ${ZRBFC0_BUILD_STATUS_FILE}"
  local z_start_time=""
  local z_start_time_status=0
  z_start_time=$(<"${ZRBFC0_BUILD_START_FILE}") || z_start_time_status=$?
  test "${z_start_time_status}" -eq 0 || z_start_time=""
  local z_finish_time=""
  local z_finish_time_status=0
  z_finish_time=$(<"${ZRBFC0_BUILD_FINISH_FILE}") || z_finish_time_status=$?
  test "${z_finish_time_status}" -eq 0 || z_finish_time=""

  if test -n "${z_start_time}" && test -n "${z_finish_time}"; then
    local z_start_clean="${z_start_time%%.*}"
    z_start_clean="${z_start_clean%%Z}"
    local z_finish_clean="${z_finish_time%%.*}"
    z_finish_clean="${z_finish_clean%%Z}"
    local z_start_epoch=""
    local z_start_epoch_status=0
    z_start_epoch=$(date -d "${z_start_clean}Z" '+%s' 2>"${ZRBFC0_POLL_STDERR_PREFIX}start_date_gnu.txt") || z_start_epoch_status=$?
    if test "${z_start_epoch_status}" -ne 0; then
      z_start_epoch_status=0
      z_start_epoch=$(date -j -f '%Y-%m-%dT%H:%M:%S' "${z_start_clean}" '+%s' 2>"${ZRBFC0_POLL_STDERR_PREFIX}start_date_bsd.txt") || z_start_epoch_status=$?
      test "${z_start_epoch_status}" -eq 0 || z_start_epoch=""
    fi
    local z_finish_epoch=""
    local z_finish_epoch_status=0
    z_finish_epoch=$(date -d "${z_finish_clean}Z" '+%s' 2>"${ZRBFC0_POLL_STDERR_PREFIX}finish_date_gnu.txt") || z_finish_epoch_status=$?
    if test "${z_finish_epoch_status}" -ne 0; then
      z_finish_epoch_status=0
      z_finish_epoch=$(date -j -f '%Y-%m-%dT%H:%M:%S' "${z_finish_clean}" '+%s' 2>"${ZRBFC0_POLL_STDERR_PREFIX}finish_date_bsd.txt") || z_finish_epoch_status=$?
      test "${z_finish_epoch_status}" -eq 0 || z_finish_epoch=""
    fi
    if test -n "${z_start_epoch}" && test -n "${z_finish_epoch}"; then
      case "${z_start_epoch}" in ''|*[!0123456789]*) buc_die_now "Not a decimal number: ${z_start_epoch}" ;; esac
      case "${z_finish_epoch}" in ''|*[!0123456789]*) buc_die_now "Not a decimal number: ${z_finish_epoch}" ;; esac
      local -r z_duration=$((10#${z_finish_epoch} - 10#${z_start_epoch}))
      local -r z_minutes=$((z_duration / 60))
      local -r z_seconds=$((z_duration % 60))
      buc_info "${z_label}: Wall clock ${z_minutes}m ${z_seconds}s"
    fi
  fi

  buc_success "${z_label}: Build completed successfully"
}

# Internal: capture git metadata to module temp files (idempotent)
# No args — reads from git, writes to ZRBFC0_GIT_*_FILE kindle constants
zrbfcb_ensure_git_metadata() {
  zrbfc0_sentinel

  # Idempotent — skip if already captured
  test ! -s "${ZRBFC0_GIT_COMMIT_FILE}" || return 0

  buc_log_args "Capturing git metadata to temp files"

  local -r z_remote_file="${ZRBFC0_GIT_PREFIX}remote.txt"
  local -r z_url_file="${ZRBFC0_GIT_PREFIX}url.txt"

  git rev-parse HEAD > "${ZRBFC0_GIT_COMMIT_FILE}" \
    || buc_die_now "Failed to get git commit"
  test -s "${ZRBFC0_GIT_COMMIT_FILE}" || buc_die_now "Empty git commit file"

  git rev-parse --abbrev-ref HEAD > "${ZRBFC0_GIT_BRANCH_FILE}" \
    || buc_die_now "Failed to get git branch"
  test -s "${ZRBFC0_GIT_BRANCH_FILE}" || buc_die_now "Empty git branch file"

  git remote > "${z_remote_file}" \
    || buc_die_now "Failed to list git remotes"
  local z_remote=""
  read -r z_remote < "${z_remote_file}" \
    || buc_die_now "Failed to read git remote from ${z_remote_file}"
  test -n "${z_remote}" || buc_die_now "No git remotes found"

  git config --get "remote.${z_remote}.url" > "${z_url_file}" \
    || buc_die_now "Failed to get git repo URL"

  local z_url=""
  z_url=$(<"${z_url_file}") || buc_die_now "Failed to read: ${z_url_file}"
  test -n "${z_url}" || buc_die_now "Empty git repo URL from ${z_url_file}"
  local z_repo="${z_url#*://*/}"
  z_repo="${z_repo%.git}"
  echo "${z_repo}" > "${ZRBFC0_GIT_REPO_FILE}" \
    || buc_die_now "Failed to write derived git repo"
}

# Internal: write a script's body (everything after the shebang line) to a file
# using only builtins — the portable replacement for `tail -n +2 src > dst`.
# Dies if the source is unreadable or the destination unwritable.
zrbfcb_write_script_body() {
  local -r z_src="${1:-}"
  local -r z_dst="${2:-}"
  test -n "${z_src}" || buc_die_now "zrbfcb_write_script_body: src required"
  test -n "${z_dst}" || buc_die_now "zrbfcb_write_script_body: dst required"
  local z_line
  local z_seen_shebang=""
  test -r "${z_src}" || buc_die_now "Script not readable: ${z_src}"
  : > "${z_dst}"     || buc_die_now "Failed to truncate script body file: ${z_dst}"
  while IFS= read -r z_line || test -n "${z_line}"; do
    if test -z "${z_seen_shebang}"; then
      z_seen_shebang=1
      continue
    fi
    printf '%s\n' "${z_line}" >> "${z_dst}" || buc_die_now "Failed to write script body: ${z_dst}"
  done < "${z_src}"
  return 0
}

# Internal: expand "#@rbgjs_include <name>" markers in a step body file IN PLACE.
# Each marker line (any leading indentation tolerated) is replaced by the body of
# <snippets_dir>/rbgjs-<name>.sh with its leading shebang stripped — the same
# shebang-strip rule zrbfcb_write_script_body applies. A body with no markers is
# rewritten unchanged (a no-op), so every assembler may call this for every step.
# This is the host side of the shared cloud-step library (the "Composed-snippet
# library"): a snippet reads shell vars the kind sets before the marker and is
# blind to substitution names, which is what lets one snippet serve callers with
# disjoint _RBGx_ substitution sets. Pure primitive — args only, no kindle state,
# no sentinel — so it stays unit-testable alongside zrbfcb_write_script_body.
# Dies if the body is unreadable, the snippets dir is missing, a marker names
# no snippet, or a named snippet file is absent (crash-fast, no silent skip).
# Args: body_file snippets_dir
zrbfcb_expand_includes() {
  local -r z_body_file="${1:-}"
  local -r z_snippets_dir="${2:-}"
  test -n "${z_body_file}" || buc_die_now "zrbfcb_expand_includes: body_file required"
  test -n "${z_snippets_dir}" || buc_die_now "zrbfcb_expand_includes: snippets_dir required"
  test -f "${z_body_file}"    || buc_die_now "Step body file not found: ${z_body_file}"
  test -d "${z_snippets_dir}" || buc_die_now "Snippets directory not found: ${z_snippets_dir}"

  local -r z_tmp="${z_body_file}.expanded"
  : > "${z_tmp}" || buc_die_now "Failed to truncate expansion file: ${z_tmp}"

  local z_line=""
  local z_trimmed=""
  local z_name=""
  local z_snippet=""
  local z_sline=""
  local z_seen_shebang=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_trimmed="${z_line#"${z_line%%[![:space:]]*}"}"
    case "${z_trimmed}" in
      '#@rbgjs_include '*)
        z_name="${z_trimmed#'#@rbgjs_include '}"
        z_name="${z_name%%[[:space:]]*}"
        test -n "${z_name}" || buc_die_now "Include marker names no snippet in ${z_body_file}: ${z_line}"
        z_snippet="${z_snippets_dir}/rbgjs-${z_name}.sh"
        test -f "${z_snippet}" || buc_die_now "Snippet not found: ${z_snippet}"
        z_seen_shebang=""
        while IFS= read -r z_sline || test -n "${z_sline}"; do
          if test -z "${z_seen_shebang}"; then
            z_seen_shebang=1
            case "${z_sline}" in '#!'*) continue ;; esac
          fi
          printf '%s\n' "${z_sline}" >> "${z_tmp}" || buc_die_now "Failed to write expansion file: ${z_tmp}"
        done < "${z_snippet}"
        ;;
      *)
        printf '%s\n' "${z_line}" >> "${z_tmp}" || buc_die_now "Failed to write expansion file: ${z_tmp}"
        ;;
    esac
  done < "${z_body_file}"

  mv "${z_tmp}" "${z_body_file}" || buc_die_now "Failed to replace step body with expansion: ${z_body_file}"
  return 0
}

# eof
