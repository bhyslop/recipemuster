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
# Recipe Bottle Foundry Core - shared infrastructure for all Foundry child modules

set -euo pipefail

# Multiple inclusion guard (silent skip — rbfc is sourced by multiple child modules)
test -z "${ZRBFC_SOURCED:-}" || return 0
ZRBFC_SOURCED=1

######################################################################
# Internal Functions (zrbfc_*)

zrbfc_kindle() {
  test -z "${ZRBFC_KINDLED:-}" || buc_die "Module rbfc already kindled"

  # Validate environment
  zburd_sentinel

  buc_log_args 'Check required GCB/GAR environment variables'
  zrbgc_sentinel

  buc_log_args 'Module Variables (ZRBFC_*)'
  readonly ZRBFC_GCB_API_BASE="https://cloudbuild.googleapis.com/v1"
  readonly ZRBFC_GAR_API_BASE="https://artifactregistry.googleapis.com/v1"
  readonly ZRBFC_CLOUD_QUERY_BASE="https://console.cloud.google.com/cloud-build/builds"

  readonly ZRBFC_GCB_PROJECT_BUILDS_URL="${ZRBFC_GCB_API_BASE}/projects/${RBGD_GCB_PROJECT_ID}/locations/${RBGD_GCB_REGION}/builds"
  readonly ZRBFC_GAR_PACKAGE_BASE="projects/${RBGD_GAR_PROJECT_ID}/locations/${RBGD_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}"

  buc_log_args 'Trigger dispatch endpoints'
  readonly ZRBFC_TRIGGERS_URL="${RBGC_API_ROOT_CLOUDBUILD}${RBGC_CLOUDBUILD_V1}/projects/${RBRR_DEPOT_PROJECT_ID}/locations/${RBGD_GCB_REGION}/triggers"

  buc_log_args 'Registry API endpoints'
  readonly ZRBFC_REGISTRY_HOST="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  readonly ZRBFC_REGISTRY_PATH="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  readonly ZRBFC_REGISTRY_API_BASE="https://${ZRBFC_REGISTRY_HOST}/v2/${ZRBFC_REGISTRY_PATH}"

  buc_log_args 'Media types for registry operations'
  readonly ZRBFC_ACCEPT_MANIFEST_MTYPES="application/vnd.docker.distribution.manifest.v2+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.oci.image.index.v1+json,application/vnd.oci.image.manifest.v1+json"

  buc_log_args 'Define build polling files'
  readonly ZRBFC_BUILD_ID_FILE="${BURD_TEMP_DIR}/rbfc_build_id.txt"
  readonly ZRBFC_BUILD_STATUS_FILE="${BURD_TEMP_DIR}/rbfc_build_status.json"
  readonly ZRBFC_BUILD_START_FILE="${BURD_TEMP_DIR}/rbfc_build_start.txt"
  readonly ZRBFC_BUILD_FINISH_FILE="${BURD_TEMP_DIR}/rbfc_build_finish.txt"

  buc_log_args 'Define git info file (used by stitch)'
  readonly ZRBFC_GIT_INFO_FILE="${BURD_TEMP_DIR}/rbfc_git_info.json"

  buc_log_args 'Define git metadata files (shared across about/mirror submissions)'
  readonly ZRBFC_GIT_PREFIX="${BURD_TEMP_DIR}/rbfc_git_"
  readonly ZRBFC_GIT_COMMIT_FILE="${ZRBFC_GIT_PREFIX}commit.txt"
  readonly ZRBFC_GIT_BRANCH_FILE="${ZRBFC_GIT_PREFIX}branch.txt"
  readonly ZRBFC_GIT_REPO_FILE="${ZRBFC_GIT_PREFIX}repo.txt"

  buc_log_args 'Define validation files'
  readonly ZRBFC_STATUS_CHECK_FILE="${BURD_TEMP_DIR}/rbfc_status_check.txt"

  buc_log_args 'Vessel-related files'
  readonly ZRBFC_VESSEL_SIGIL_FILE="${BURD_TEMP_DIR}/rbfc_vessel_sigil.txt"
  readonly ZRBFC_VESSEL_RESOLVED_DIR_FILE="${BURD_TEMP_DIR}/rbfc_vessel_resolved_dir.txt"

  buc_log_args 'Define output files (BURD_OUTPUT_DIR — persists after dispatch)'
  readonly ZRBFC_OUTPUT_VESSEL_DIR="${BURD_OUTPUT_DIR}/rbfc_vessel_dir.txt"

  buc_log_args 'Scratch file for sequential temp-file patterns'
  readonly ZRBFC_SCRATCH_FILE="${BURD_TEMP_DIR}/rbfc_scratch.txt"

  buc_log_args 'Step script directories (used by shared about/vouch assembly)'
  local z_self_dir="${BASH_SOURCE[0]%/*}"
  readonly ZRBFC_RBGJA_STEPS_DIR="${z_self_dir}/rbgja"
  test -d "${ZRBFC_RBGJA_STEPS_DIR}" || buc_die "RBGJA steps directory not found: ${ZRBFC_RBGJA_STEPS_DIR}"
  readonly ZRBFC_RBGJV_STEPS_DIR="${z_self_dir}/rbgjv"
  test -d "${ZRBFC_RBGJV_STEPS_DIR}" || buc_die "RBGJV steps directory not found: ${ZRBFC_RBGJV_STEPS_DIR}"

  readonly ZRBFC_KINDLED=1
}

zrbfc_sentinel() {
  test "${ZRBFC_KINDLED:-}" = "1" || buc_die "Module rbfc not kindled - call zrbfc_kindle first"
}

# Internal: Resolve tool image references from reliquary.
# Must be called after vessel load (reads RBRV_RELIQUARY).
# Sets module-level ZRBFC_TOOL_* variables for downstream step assembly.
# Idempotent — safe to call multiple times per invocation.
zrbfc_resolve_tool_images() {
  zrbfc_sentinel

  local -r z_reliquary="${RBRV_RELIQUARY:-}"
  test -n "${z_reliquary}" \
    || buc_die "RBRV_RELIQUARY is required — run inscribe to create a reliquary first"

  local -r z_rqy_prefix="${ZRBFC_REGISTRY_HOST}/${ZRBFC_REGISTRY_PATH}/${z_reliquary}"
  ZRBFC_TOOL_GCLOUD="${z_rqy_prefix}/gcloud:latest"
  ZRBFC_TOOL_DOCKER="${z_rqy_prefix}/docker:latest"
  ZRBFC_TOOL_ALPINE="${z_rqy_prefix}/alpine:latest"
  ZRBFC_TOOL_SYFT="${z_rqy_prefix}/syft:latest"
  ZRBFC_TOOL_BINFMT="${z_rqy_prefix}/binfmt:latest"
  ZRBFC_TOOL_SKOPEO="${z_rqy_prefix}/skopeo:latest"
  buc_log_args "Tool images resolved from reliquary: ${z_reliquary}"
}

# Validate that a vessel sigil is non-empty and corresponds to a known vessel.
# On missing or invalid sigil, lists available vessels and dies.
# Does NOT resolve to a directory — use zrbfc_resolve_vessel for that.
rbfc_require_vessel_sigil() {
  zrbfc_sentinel

  local -r z_sigil="${1:-}"

  if test -n "${z_sigil}" && test -d "${RBRR_VESSEL_DIR}/${z_sigil}" && test -f "${RBRR_VESSEL_DIR}/${z_sigil}/rbrv.env"; then
    return 0
  fi

  local z_sigils=""
  z_sigils=$(rbrv_list_capture) || buc_die "No vessels found"
  buc_step "Available vessels:"
  local z_s=""
  for z_s in ${z_sigils}; do
    buc_bare "        ${z_s}"
  done
  if test -z "${z_sigil}"; then
    buc_die "Vessel parameter required"
  fi
  buc_die "Vessel not found: ${z_sigil}"
}

# Resolve vessel argument: accepts a sigil (e.g., rbev-sentry-deb-tether) or a path
# (e.g., rbev-vessels/rbev-sentry-deb-tether).  On no-arg or invalid arg, lists
# available vessels and dies.  On success, writes resolved path to ZRBFC_VESSEL_RESOLVED_DIR_FILE.
zrbfc_resolve_vessel() {
  zrbfc_sentinel

  local -r z_arg="${1:-}"

  # Try as path first, then as sigil under RBRR_VESSEL_DIR
  if test -n "${z_arg}" && test -d "${z_arg}" && test -f "${z_arg}/rbrv.env"; then
    printf '%s' "${z_arg}" > "${ZRBFC_VESSEL_RESOLVED_DIR_FILE}" \
      || buc_die "Failed to write resolved vessel path"
    return 0
  fi
  if test -n "${z_arg}" && test -d "${RBRR_VESSEL_DIR}/${z_arg}" && test -f "${RBRR_VESSEL_DIR}/${z_arg}/rbrv.env"; then
    printf '%s' "${RBRR_VESSEL_DIR}/${z_arg}" > "${ZRBFC_VESSEL_RESOLVED_DIR_FILE}" \
      || buc_die "Failed to write resolved vessel path"
    return 0
  fi

  # Resolution failed — list available vessels and die
  local z_sigils=""
  z_sigils=$(rbrv_list_capture) || buc_die "No vessels found"
  buc_step "Available vessels:"
  local z_sigil=""
  for z_sigil in ${z_sigils}; do
    buc_bare "        ${z_sigil}"
  done
  if test -z "${z_arg}"; then
    buc_die "Vessel argument required (sigil or path)"
  fi
  buc_die "Vessel not found: ${z_arg}"
}

zrbfc_load_vessel() {
  zrbfc_sentinel

  local z_vessel_dir="$1"

  buc_log_args 'Validate vessel directory exists'
  test -d "${z_vessel_dir}" || buc_die "Vessel directory not found: ${z_vessel_dir}"

  buc_log_args 'Check for rbrv.env file'
  local z_vessel_env="${z_vessel_dir}/rbrv.env"
  test -f "${z_vessel_env}" || buc_die "Vessel configuration not found: ${z_vessel_env}"

  buc_log_args 'Source vessel configuration'
  source "${z_vessel_env}" || buc_die "Failed to source vessel config: ${z_vessel_env}"

  buc_log_args 'Validate vessel directory matches sigil'
  local z_vessel_dir_clean="${z_vessel_dir%/}"  # Strip any trailing slash
  local z_dir_name="${z_vessel_dir_clean##*/}"  # Extract directory name
  buc_log_args "  z_vessel_dir = ${z_vessel_dir}"
  buc_log_args "  z_dir_name   = ${z_dir_name}"
  test "${z_dir_name}" = "${RBRV_SIGIL}" || buc_die "Vessel sigil '${RBRV_SIGIL}' does not match directory name '${z_dir_name}'"

  buc_log_args 'Validate vessel path matches expected pattern'
  local z_expected_vessel_dir="${RBRR_VESSEL_DIR}/${RBRV_SIGIL}"
  local z_vessel_realpath=""
  z_vessel_realpath=$(cd "${z_vessel_dir}" && pwd) || buc_die "Failed to resolve vessel directory path"
  local z_expected_realpath=""
  z_expected_realpath=$(cd "${z_expected_vessel_dir}" && pwd) || buc_die "Failed to resolve expected vessel path"
  test "${z_vessel_realpath}" = "${z_expected_realpath}" || buc_die "Vessel directory '${z_vessel_dir}' does not match expected location '${z_expected_vessel_dir}'"

  buc_log_args 'Store loaded vessel info for use by commands'
  echo "${RBRV_SIGIL}" > "${ZRBFC_VESSEL_SIGIL_FILE}" || buc_die "Failed to store vessel sigil"

  buc_info "Loaded vessel: ${RBRV_SIGIL}"
}

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
  local z_max_consecutive_failures=3
  local z_err_check_file="${BURD_TEMP_DIR}/rbfc_poll_err_check.txt"

  while true; do
    case "${z_status}" in PENDING|QUEUED|WORKING) : ;; *) break;; esac
    sleep 5

    z_polls=$((z_polls + 1))
    test "${z_polls}" -le "${z_max_polls}" || buc_die "${z_label}: Build timeout after ${z_max_polls} polls"

    buc_log_args "Fetch build status (poll ${z_polls}/${z_max_polls})"
    curl -s                                                \
         --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
         --max-time "${RBCC_CURL_MAX_TIME_SEC}"             \
         -H "Authorization: Bearer ${z_token}"             \
         "${ZRBFC_GCB_PROJECT_BUILDS_URL}/${z_build_id}"    \
         > "${ZRBFC_BUILD_STATUS_FILE}"
    if test $? -ne 0; then
      z_consecutive_failures=$((z_consecutive_failures + 1))
      buc_warn "Curl failed (${z_consecutive_failures}/${z_max_consecutive_failures} consecutive)"
      test ${z_consecutive_failures} -ge ${z_max_consecutive_failures} \
        && buc_die "Failed to get build status after ${z_max_consecutive_failures} consecutive failures"
      continue
    fi

    # Validate response is non-empty
    if ! test -s "${ZRBFC_BUILD_STATUS_FILE}"; then
      z_consecutive_failures=$((z_consecutive_failures + 1))
      buc_warn "Empty response (${z_consecutive_failures}/${z_max_consecutive_failures} consecutive)"
      test ${z_consecutive_failures} -ge ${z_max_consecutive_failures} \
        && buc_die "Empty build status after ${z_max_consecutive_failures} consecutive failures"
      continue
    fi

    # Check for HTTP error responses (401/403/etc) — write to temp file, no subshell
    jq -r '.error.code // empty' "${ZRBFC_BUILD_STATUS_FILE}" > "${z_err_check_file}" 2>/dev/null
    if test -s "${z_err_check_file}"; then
      z_consecutive_failures=$((z_consecutive_failures + 1))
      buc_warn "HTTP error $(<"${z_err_check_file}") (${z_consecutive_failures}/${z_max_consecutive_failures} consecutive)"
      test ${z_consecutive_failures} -ge ${z_max_consecutive_failures} \
        && buc_die "HTTP errors after ${z_max_consecutive_failures} consecutive failures"
      continue
    fi

    # Successful response — reset failure counter
    z_consecutive_failures=0

    jq -r '.status' "${ZRBFC_BUILD_STATUS_FILE}" > "${ZRBFC_STATUS_CHECK_FILE}" || buc_die "Failed to extract status"
    z_status=$(<"${ZRBFC_STATUS_CHECK_FILE}")
    test -n "${z_status}" || buc_die "Status is empty"

    buc_info "${z_label}: ${z_status} (poll ${z_polls}/${z_max_polls})"

    if test "${z_status}" = "QUEUED" && test "${z_polls}" -ge 20 && test "${z_queued_advisory_shown}" = "0"; then
      z_queued_advisory_shown=1
      buc_warn "Build queued longer than normal — another build may be holding the private pool"
      buc_tabtarget "${RBZ_QUOTA_BUILD}"
    fi
  done

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

# Internal: assemble about step scripts into JSON array file
# Args: output_file temp_prefix
# Reads ZRBFC_RBGJA_STEPS_DIR and ZRBFC_TOOL_* image refs from module state
zrbfc_assemble_about_steps() {
  zrbfc_sentinel

  local -r z_output_file="$1"
  local -r z_temp_prefix="$2"

  # Step definitions: script|builder|entrypoint|id
  # Delimiter is | because image refs contain colons (sha256 digests)
  local -r z_about_step_defs=(
    "rbgja01-discover-platforms.py|${ZRBFC_TOOL_GCLOUD}|python3|discover-platforms"
    "rbgja02-syft-per-platform.sh|${ZRBFC_TOOL_DOCKER}|bash|syft-per-platform"
    "rbgja03-build-info-per-platform.py|${ZRBFC_TOOL_GCLOUD}|python3|build-info-per-platform"
    "rbgja04-assemble-push-about.sh|${ZRBFC_TOOL_DOCKER}|bash|assemble-push-about"
  )

  echo "[]" > "${z_output_file}" || buc_die "Failed to initialize about steps JSON"

  local z_adef=""
  local z_ascript=""
  local z_abuilder=""
  local z_aentrypoint=""
  local z_aid=""
  local z_ascript_path=""
  local z_abody_file=""
  local z_aescaped_file=""
  local z_asteps_file=""
  local z_abody=""
  local z_ashebang=""

  for z_adef in "${z_about_step_defs[@]}"; do
    IFS='|' read -r z_ascript z_abuilder z_aentrypoint z_aid <<< "${z_adef}"
    z_ascript_path="${ZRBFC_RBGJA_STEPS_DIR}/${z_ascript}"
    z_abody_file="${z_temp_prefix}${z_aid}_body.txt"
    z_aescaped_file="${z_temp_prefix}${z_aid}_escaped.txt"
    z_asteps_file="${z_temp_prefix}${z_aid}_steps.json"

    test -f "${z_ascript_path}" || buc_die "About step script not found: ${z_ascript_path}"

    buc_log_args "Reading script body for ${z_aid} (skip shebang)"
    tail -n +2 "${z_ascript_path}" > "${z_abody_file}" \
      || buc_die "Failed to read about step script: ${z_ascript_path}"
    z_abody=$(<"${z_abody_file}")
    test -n "${z_abody}" || buc_die "Empty about script body: ${z_ascript_path}"

    buc_log_args "Baking pinned image refs into script text"
    z_abody="${z_abody//\$\{ZRBF_TOOL_SYFT\}/${ZRBFC_TOOL_SYFT}}"

    case "${z_aentrypoint}" in
      bash)    z_ashebang="#!/bin/bash" ;;
      sh)      z_ashebang="#!/bin/sh" ;;
      python3) z_ashebang="#!/usr/bin/env python3" ;;
      *)       buc_die "Unknown entrypoint: ${z_aentrypoint}" ;;
    esac
    printf '%s\n%s' "${z_ashebang}" "${z_abody}" > "${z_aescaped_file}" \
      || buc_die "Failed to write about script body for ${z_aid}"

    buc_log_args "Appending about step ${z_aid} to JSON array"
    jq \
      --arg name "${z_abuilder}" \
      --arg id "${z_aid}" \
      --rawfile script "${z_aescaped_file}" \
      '. + [{name: $name, id: $id, script: $script}]' \
      "${z_output_file}" > "${z_asteps_file}" \
      || buc_die "Failed to append about step ${z_aid} to JSON"
    mv "${z_asteps_file}" "${z_output_file}" \
      || buc_die "Failed to update about steps JSON for ${z_aid}"
  done
}

# Internal: assemble vouch step scripts into JSON array file
# Args: output_file temp_prefix
# Reads ZRBFC_RBGJV_STEPS_DIR and ZRBFC_TOOL_* image refs from module state
zrbfc_assemble_vouch_steps() {
  zrbfc_sentinel

  local -r z_output_file="$1"
  local -r z_temp_prefix="$2"

  # Step definitions: script|builder|entrypoint|id
  # Delimiter is | because image refs contain colons (sha256 digests)
  local -r z_vouch_step_defs=(
    "rbgjv01-download-verifier.sh|${ZRBFC_TOOL_ALPINE}|sh|prepare-keys"
    "rbgjv02-verify-provenance.py|${ZRBFC_TOOL_GCLOUD}|python3|verify-provenance"
    "rbgjv03-assemble-push-vouch.sh|${ZRBFC_TOOL_DOCKER}|bash|assemble-push-vouch"
  )

  echo "[]" > "${z_output_file}" || buc_die "Failed to initialize vouch steps JSON"

  local z_vdef=""
  local z_vscript=""
  local z_vbuilder=""
  local z_ventrypoint=""
  local z_vid=""
  local z_vscript_path=""
  local z_vbody_file=""
  local z_vescaped_file=""
  local z_vsteps_file=""
  local z_vbody=""
  local z_vshebang=""

  for z_vdef in "${z_vouch_step_defs[@]}"; do
    IFS='|' read -r z_vscript z_vbuilder z_ventrypoint z_vid <<< "${z_vdef}"
    z_vscript_path="${ZRBFC_RBGJV_STEPS_DIR}/${z_vscript}"
    z_vbody_file="${z_temp_prefix}${z_vid}_body.txt"
    z_vescaped_file="${z_temp_prefix}${z_vid}_escaped.txt"
    z_vsteps_file="${z_temp_prefix}${z_vid}_steps.json"

    test -f "${z_vscript_path}" || buc_die "Vouch step script not found: ${z_vscript_path}"

    buc_log_args "Reading script body for ${z_vid} (skip shebang)"
    tail -n +2 "${z_vscript_path}" > "${z_vbody_file}" \
      || buc_die "Failed to read vouch step script: ${z_vscript_path}"
    z_vbody=$(<"${z_vbody_file}")
    test -n "${z_vbody}" || buc_die "Empty vouch script body: ${z_vscript_path}"

    case "${z_ventrypoint}" in
      bash)    z_vshebang="#!/bin/bash" ;;
      sh)      z_vshebang="#!/bin/sh" ;;
      python3) z_vshebang="#!/usr/bin/env python3" ;;
      *)       buc_die "Unknown entrypoint: ${z_ventrypoint}" ;;
    esac
    printf '%s\n%s' "${z_vshebang}" "${z_vbody}" > "${z_vescaped_file}" \
      || buc_die "Failed to write vouch script body for ${z_vid}"

    buc_log_args "Appending vouch step ${z_vid} to JSON array"
    jq \
      --arg name "${z_vbuilder}" \
      --arg id "${z_vid}" \
      --rawfile script "${z_vescaped_file}" \
      '. + [{name: $name, id: $id, script: $script}]' \
      "${z_output_file}" > "${z_vsteps_file}" \
      || buc_die "Failed to append vouch step ${z_vid} to JSON"
    mv "${z_vsteps_file}" "${z_output_file}" \
      || buc_die "Failed to update vouch steps JSON for ${z_vid}"
  done
}

######################################################################
# GAR Artifact Extraction (Registry API — no docker required)

# Internal: extract files from a FROM-scratch artifact in GAR to a local directory.
# Handles multi-platform manifest lists by picking the first platform (content is
# identical for architecture-independent artifacts like -about and -vouch).
# Args: token package tag extract_dir
# Returns: 0 if extraction succeeded, 1 if artifact not found in registry.
# Infrastructure failures (curl, jq, tar) are fatal via buc_die.
zrbfc_gar_extract_artifact() {
  zrbfc_sentinel

  local -r z_token="$1"
  local -r z_package="$2"
  local -r z_tag="$3"
  local -r z_extract_dir="$4"

  local -r z_safe_pkg="${z_package//\//_}"
  local -r z_prefix="${BURD_TEMP_DIR}/gar_${z_safe_pkg}_${z_tag}_"

  # HEAD check — does the artifact exist?
  local -r z_head_status="${z_prefix}head_status.txt"
  local -r z_head_response="${z_prefix}head_response.txt"
  local -r z_head_stderr="${z_prefix}head_stderr.txt"
  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_head_response}" \
    "${ZRBFC_REGISTRY_API_BASE}/${z_package}/manifests/${z_tag}" \
    > "${z_head_status}" 2>"${z_head_stderr}" \
    || buc_die "HEAD request failed for ${z_package}:${z_tag} — see ${z_head_stderr}"

  local -r z_http_code=$(<"${z_head_status}")
  test "${z_http_code}" = "200" || return 1

  # GET manifest (may be manifest list/index or single-platform manifest)
  local -r z_manifest="${z_prefix}manifest.json"
  local -r z_manifest_stderr="${z_prefix}manifest_stderr.txt"
  curl -sL \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}" \
    "${ZRBFC_REGISTRY_API_BASE}/${z_package}/manifests/${z_tag}" \
    > "${z_manifest}" 2>"${z_manifest_stderr}" \
    || buc_die "GET manifest failed for ${z_package}:${z_tag} — see ${z_manifest_stderr}"

  # Resolve to a single-platform manifest
  local z_single_manifest="${z_manifest}"
  local -r z_media_type_file="${z_prefix}media_type.txt"
  jq -r '.mediaType // empty' "${z_manifest}" > "${z_media_type_file}" 2>/dev/null || true
  local -r z_media_type=$(<"${z_media_type_file}")

  case "${z_media_type}" in
    *manifest.list*|*image.index*)
      # Multi-platform manifest list — pick first platform's manifest
      local -r z_digest_file="${z_prefix}platform_digest.txt"
      jq -r '.manifests[0].digest // empty' "${z_manifest}" \
        > "${z_digest_file}" 2>/dev/null \
        || buc_die "Failed to extract platform digest from manifest list"
      local -r z_platform_digest=$(<"${z_digest_file}")
      test -n "${z_platform_digest}" || buc_die "Empty platform digest in manifest list"

      local -r z_plat_manifest="${z_prefix}plat_manifest.json"
      local -r z_plat_stderr="${z_prefix}plat_manifest_stderr.txt"
      curl -sL \
        --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
        --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
        -H "Authorization: Bearer ${z_token}" \
        -H "Accept: application/vnd.docker.distribution.manifest.v2+json,application/vnd.oci.image.manifest.v1+json" \
        "${ZRBFC_REGISTRY_API_BASE}/${z_package}/manifests/${z_platform_digest}" \
        > "${z_plat_manifest}" 2>"${z_plat_stderr}" \
        || buc_die "GET platform manifest failed for ${z_package} — see ${z_plat_stderr}"
      z_single_manifest="${z_plat_manifest}"
      ;;
  esac

  # Extract first layer digest (FROM-scratch images have one layer with all COPY'd files)
  local -r z_layer_file="${z_prefix}layer_digest.txt"
  jq -r '.layers[0].digest // empty' "${z_single_manifest}" \
    > "${z_layer_file}" 2>/dev/null \
    || buc_die "Failed to extract layer digest from manifest"
  local -r z_layer_digest=$(<"${z_layer_file}")
  test -n "${z_layer_digest}" || buc_die "No layer digest in manifest for ${z_package}:${z_tag}"

  # Fetch layer blob and extract
  local -r z_blob_file="${z_prefix}blob.tar.gz"
  local -r z_blob_stderr="${z_prefix}blob_stderr.txt"
  curl -sL \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    "${ZRBFC_REGISTRY_API_BASE}/${z_package}/blobs/${z_layer_digest}" \
    > "${z_blob_file}" 2>"${z_blob_stderr}" \
    || buc_die "GET blob failed for ${z_package} layer ${z_layer_digest} — see ${z_blob_stderr}"

  mkdir -p "${z_extract_dir}" || buc_die "Failed to create extraction directory: ${z_extract_dir}"
  local -r z_tar_stderr="${z_prefix}tar_stderr.txt"
  tar -xzf "${z_blob_file}" -C "${z_extract_dir}" 2>"${z_tar_stderr}" \
    || buc_die "Failed to extract layer blob for ${z_package}:${z_tag} — see ${z_tar_stderr}"

  return 0
}

######################################################################
# Plumb (rbw-fpf / rbw-fpc)

# Internal: core plumb logic shared by full and compact modes.
# Queries GAR directly via Registry API — no local docker required.
# Authenticates as Retriever. Supports hallmark-only mode via VOUCHES lookup.
# Args: vessel hallmark mode
zrbfc_plumb_core() {
  zrbfc_sentinel

  local z_vessel="${1:-}"
  z_vessel="${z_vessel##*/}"  # accept directory path or bare moniker
  local -r z_hallmark="${2:-}"
  local -r z_mode="${3}"

  test -n "${z_hallmark}" || buc_die "Hallmark parameter required"

  # Authenticate as Retriever
  buc_step "Authenticating as Retriever"
  test -f "${RBDC_RETRIEVER_RBRA_FILE}" \
    || buc_die "Retriever credential not found: ${RBDC_RETRIEVER_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_RETRIEVER_RBRA_FILE}") \
    || buc_die "Failed to get Retriever OAuth token"

  local -r z_extract="${BURD_TEMP_DIR}/plumb"
  mkdir -p "${z_extract}" || buc_die "Failed to create extraction directory"
  local z_has_about=false
  local z_has_vouch=false

  # Hallmark-only mode: resolve vessel via VOUCHES superdirectory
  if test -z "${z_vessel}"; then
    buc_step "Looking up hallmark in VOUCHES superdirectory"
    local -r z_vouches_dir="${BURD_TEMP_DIR}/plumb_vouches"
    if zrbfc_gar_extract_artifact "${z_token}" "${RBGC_VOUCHES_PACKAGE}" \
         "${z_hallmark}${RBGC_ARK_SUFFIX_VOUCH}" "${z_vouches_dir}"; then
      test -f "${z_vouches_dir}/vouch_summary.json" \
        || buc_die "vouch_summary.json not found in VOUCHES artifact"
      local -r z_vessel_file="${BURD_TEMP_DIR}/plumb_vessel.txt"
      jq -r '.vessel // empty' "${z_vouches_dir}/vouch_summary.json" \
        > "${z_vessel_file}" \
        || buc_die "Failed to read vessel from vouch_summary.json"
      z_vessel=$(<"${z_vessel_file}")
      test -n "${z_vessel}" || buc_die "Vessel field empty in vouch_summary.json"
      buc_info "Resolved hallmark to vessel: ${z_vessel}"
      cp "${z_vouches_dir}"/* "${z_extract}/" 2>/dev/null || true
      z_has_vouch=true
    else
      buc_die "Hallmark not found in VOUCHES superdirectory: ${z_hallmark}"
    fi
  fi

  rbfc_require_vessel_sigil "${z_vessel}"

  # Load vessel config (sets RBRV_VESSEL_MODE, RBRV_BIND_IMAGE, etc.)
  local -r z_vessel_dir="${RBRR_VESSEL_DIR}/${z_vessel}"
  zrbfc_load_vessel "${z_vessel_dir}"

  # Fetch -about artifact from GAR
  buc_step "Fetching -about artifact from GAR"
  local -r z_about_dir="${BURD_TEMP_DIR}/plumb_about"
  if zrbfc_gar_extract_artifact "${z_token}" "${z_vessel}" \
       "${z_hallmark}${RBGC_ARK_SUFFIX_ABOUT}" "${z_about_dir}"; then
    z_has_about=true
    cp "${z_about_dir}"/* "${z_extract}/" 2>/dev/null || true
  fi

  # Fetch -vouch artifact from vessel package (skip if already resolved via VOUCHES)
  if test "${z_has_vouch}" = "false"; then
    buc_step "Fetching -vouch artifact from GAR"
    local -r z_vouch_dir="${BURD_TEMP_DIR}/plumb_vouch"
    if zrbfc_gar_extract_artifact "${z_token}" "${z_vessel}" \
         "${z_hallmark}${RBGC_ARK_SUFFIX_VOUCH}" "${z_vouch_dir}"; then
      z_has_vouch=true
      cp "${z_vouch_dir}"/* "${z_extract}/" 2>/dev/null || true
    fi
  fi

  # Bind vessels: fallback to static display if no -about
  if test "${RBRV_VESSEL_MODE}" = "bind" && test "${z_has_about}" = "false"; then
    zrbfc_plumb_show_bind "${z_vessel}" "${z_hallmark}" "${z_mode}"
    return 0
  fi

  # Require -about for non-bind vessels
  if test "${z_has_about}" = "false"; then
    buc_die "About artifact not found in GAR for ${z_vessel}:${z_hallmark}${RBGC_ARK_SUFFIX_ABOUT}"
  fi

  # Display results
  if test "${z_mode}" = "compact"; then
    zrbfc_plumb_show_compact "${z_vessel}" "${z_hallmark}" "${z_extract}" "${z_has_vouch}"
  else
    zrbfc_plumb_show_full "${z_vessel}" "${z_hallmark}" "${z_extract}" "${z_has_vouch}"
  fi
}

# Internal: display bind vessel info
# Args: vessel hallmark mode
zrbfc_plumb_show_bind() {
  zrbfc_sentinel

  local -r z_vessel="$1"
  local -r z_hallmark="$2"
  local -r z_mode="$3"

  if test "${z_mode}" = "compact"; then
    echo ""
    echo "=== ${z_vessel} / ${z_hallmark} ==="
    echo "  Type: bind | Trust: digest-pin only"
    test -n "${RBRV_BIND_IMAGE:-}" && echo "  Source: ${RBRV_BIND_IMAGE}"
    echo "  No SLSA provenance, SBOM, or build transcript (not built by GCB)"
    echo ""
    return 0
  fi

  echo ""
  echo "================================================================"
  echo "  HALLMARK PLUMB: ${z_vessel} / ${z_hallmark}"
  echo "================================================================"
  echo ""
  echo "  Vessel type:  BIND (external image pinned by digest)"
  echo "  Trust model:  Digest-pin only"
  echo ""
  test -n "${RBRV_BIND_IMAGE:-}" && echo "  Bind source:  ${RBRV_BIND_IMAGE}"
  echo ""
  echo "  TRUST BOUNDARY"
  echo "  This is a bind vessel. The image was not built by Google Cloud"
  echo "  Build. No SLSA provenance, no SBOM, and no build transcript"
  echo "  exist because GCB did not produce this image."
  echo ""
  echo "  Trust is based solely on digest pinning of a known-good"
  echo "  external image from its source registry."
  echo ""
  echo "================================================================"
  echo ""
}

# Internal: shared section rendering used by both compact and full modes
# Args: extract_dir has_vouch
# Outputs: vessel type, source, builder, SLSA, SBOM summary, vouch results
zrbfc_plumb_show_sections() {
  zrbfc_sentinel

  local -r z_dir="$1"
  local -r z_has_vouch="$2"

  local -r z_bi="${z_dir}/build_info.json"
  local -r z_sbom="${z_dir}/sbom.json"
  local -r z_vs="${z_dir}/vouch_summary.json"
  local -r z_bkmeta="${z_dir}/buildkit_metadata.json"

  # Determine vessel mode from build_info.json
  local z_vessel_mode="conjure"
  if test -f "${z_bi}"; then
    local z_mode_raw
    jq -r '.mode // "conjure"' "${z_bi}" > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null \
      || echo "conjure" > "${ZRBFC_SCRATCH_FILE}"
    z_mode_raw=$(<"${ZRBFC_SCRATCH_FILE}")
    z_vessel_mode="${z_mode_raw}"
  fi

  if test -f "${z_bi}" && test "${z_vessel_mode}" = "bind"; then
    # ── Bind vessel sections ──────────────────────────────────────────
    # Batch extract bind fields from build_info.json
    local z_bi_moniker="" z_bi_source_img="" z_bi_mirror_ts="" z_bi_hallmark=""
    local z_bi_image_uri="" z_bi_git_repo="" z_bi_git_branch="" z_bi_git_commit=""
    jq -r '
      (.moniker // "?"),
      (.source.image_ref // "?"),
      (.build.inscribe_timestamp // "?"),
      (.build.hallmark // "?"),
      (.image.uri // "?"),
      (.git.repo // "?"),
      (.git.branch // "?"),
      (.git.commit // "?")
    ' "${z_bi}" > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null || true
    { read -r z_bi_moniker
      read -r z_bi_source_img
      read -r z_bi_mirror_ts
      read -r z_bi_hallmark
      read -r z_bi_image_uri
      read -r z_bi_git_repo
      read -r z_bi_git_branch
      read -r z_bi_git_commit
    } < "${ZRBFC_SCRATCH_FILE}"

    echo ""
    echo "  -- Vessel Type -------------------------------------------------"
    echo "  How this image was produced."
    echo ""
    echo "  Mode:           bind (upstream image mirrored to GAR)"
    echo "  Moniker:        ${z_bi_moniker}"

    echo ""
    echo "  -- Upstream Source -----------------------------------------------"
    echo "  The digest-pinned upstream image that was mirrored."
    echo ""
    echo "  Source image:   ${z_bi_source_img}"
    echo "  Trust model:    digest-pin (image identity is the digest itself)"

    echo ""
    echo "  -- Mirror -------------------------------------------------------"
    echo "  When the image was mirrored from upstream into GAR."
    echo ""
    echo "  Mirror time:    ${z_bi_mirror_ts}"
    echo "  Hallmark:   ${z_bi_hallmark}"
    echo "  Image URI:      ${z_bi_image_uri}"

    echo ""
    echo "  -- Git Context --------------------------------------------------"
    echo "  The repository state when the mirror operation was performed."
    echo ""
    echo "  Repository:     ${z_bi_git_repo}"
    echo "  Branch:         ${z_bi_git_branch}"
    echo "  Commit:         ${z_bi_git_commit}"

    echo ""
    echo "  -- Trust --------------------------------------------------------"
    echo "  Bind vessels are NOT built by Cloud Build. Trust comes from the"
    echo "  digest pin in rbrv.env — the image is exactly the bytes specified."
    echo ""
    echo "  SLSA provenance:  not applicable (no build step)"
    echo "  Verification:     image digest matches the pin in the vessel definition"

  elif test -f "${z_bi}"; then
    # ── Conjure vessel sections ───────────────────────────────────────
    # Batch extract conjure fields from build_info.json
    local z_platform="" z_qemu="" z_cj_moniker=""
    local z_cj_git_repo="" z_cj_git_branch="" z_cj_git_commit=""
    local z_cj_build_id="" z_cj_build_ts="" z_cj_inscribe_ts="" z_cj_image_uri=""
    local z_slsa_level="" z_slsa_invocation="" z_slsa_builder=""
    jq -r '
      (.platform // "unknown"),
      (.qemu_used // "false"),
      (.moniker // "?"),
      (.git.repo // "?"),
      (.git.branch // "?"),
      (.git.commit // "?"),
      (.build.build_id // "?"),
      (.build.timestamp // "?"),
      (.build.inscribe_timestamp // "?"),
      (.image.uri // "?"),
      (.slsa.build_level // "?"),
      (.slsa.build_invocation_id // "?"),
      (.slsa.provenance_builder_id // "?")
    ' "${z_bi}" > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null || true
    { read -r z_platform
      read -r z_qemu
      read -r z_cj_moniker
      read -r z_cj_git_repo
      read -r z_cj_git_branch
      read -r z_cj_git_commit
      read -r z_cj_build_id
      read -r z_cj_build_ts
      read -r z_cj_inscribe_ts
      read -r z_cj_image_uri
      read -r z_slsa_level
      read -r z_slsa_invocation
      read -r z_slsa_builder
    } < "${ZRBFC_SCRATCH_FILE}"

    echo ""
    echo "  -- Vessel Type -------------------------------------------------"
    echo "  How this image was produced and for which CPU architecture."
    echo ""
    echo "  Mode:           conjure (built by Google Cloud Build)"
    local z_strategy="native"
    if test "${z_qemu}" = "true"; then z_strategy="emulated (QEMU)"; fi
    echo "  Platform:       ${z_platform} (host-platform view)"
    echo "  Build strategy: ${z_strategy}"
    echo "  Moniker:        ${z_cj_moniker}"

    echo ""
    echo "  -- Source -------------------------------------------------------"
    echo "  The git repository, branch, and commit that produced this build."
    echo ""
    echo "  Repository:     ${z_cj_git_repo}"
    echo "  Branch:         ${z_cj_git_branch}"
    echo "  Commit:         ${z_cj_git_commit}"

    echo ""
    echo "  -- Builder ------------------------------------------------------"
    echo "  The Cloud Build job that executed this build, with timestamps."
    echo ""
    echo "  Build ID:       ${z_cj_build_id}"
    echo "  Build time:     ${z_cj_build_ts}"
    echo "  Inscribe time:  ${z_cj_inscribe_ts}"
    echo "  Image URI:      ${z_cj_image_uri}"

    echo ""
    echo "  -- SLSA Provenance ----------------------------------------------"
    echo "  Cryptographic proof linking this exact image digest to its build."
    echo ""
    echo "  Build level:    ${z_slsa_level}"
    echo "  Invocation ID:  ${z_slsa_invocation}"
    echo "  Builder ID:     ${z_slsa_builder}"
    echo "  Predicate types:"
    jq -r '.slsa.provenance_predicate_types[]?' "${z_bi}" 2>/dev/null | while IFS= read -r z_pt; do
      echo "                    ${z_pt}"
    done

    echo ""
    echo "  SLSA Build L${z_slsa_level} attests:"
    echo "    + This digest was produced by this Cloud Build invocation"
    echo "    + From this source repo and commit"
    echo "    + On Google's hosted builder (tamper-resistant environment)"
    echo ""
    echo "  SLSA Build L${z_slsa_level} does NOT attest:"
    echo "    - Base image security or supply chain"
    echo "    - Package integrity within the image"
    echo "    - Absence of vulnerabilities"
    echo "    - Correctness or security of the Dockerfile"
  else
    echo ""
    echo "  build_info.json not found in -about artifact"
  fi

  # Base image section — conjure only (bind has no Dockerfile)
  if test "${z_vessel_mode}" != "bind"; then
    echo ""
    echo "  -- Base Image ---------------------------------------------------"
    echo "  The upstream image this build started FROM and the OS syft detected."
    echo ""
    local -r z_recipe="${z_dir}/recipe.txt"
    if test -f "${z_recipe}"; then
      local z_from_line=""
      local z_recipe_line
      while IFS= read -r z_recipe_line; do
        case "${z_recipe_line}" in [Ff][Rr][Oo][Mm]\ *) z_from_line="${z_recipe_line}"; break ;; esac
      done < "${z_recipe}" 2>/dev/null
      if test -n "${z_from_line}"; then
        echo "  Dockerfile FROM: ${z_from_line#FROM }"
      fi
    fi
    if test -f "${z_sbom}"; then
      local z_distro_name="" z_distro_ver=""
      jq -r '(.distro.name // empty), (.distro.version // empty)' "${z_sbom}" \
        > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null || true
      { read -r z_distro_name; read -r z_distro_ver; } < "${ZRBFC_SCRATCH_FILE}"
      if test -n "${z_distro_name}"; then
        echo "  Detected distro: ${z_distro_name} ${z_distro_ver}"
      fi
    fi
  fi

  # Build output — conjure only (bind has no buildx step)
  if test "${z_vessel_mode}" != "bind" && test -f "${z_bkmeta}"; then
    echo ""
    echo "  -- Build Output -------------------------------------------------"
    echo "  The container image manifest produced by this buildx invocation."
    echo ""
    local z_bk_digest="" z_bk_mediatype="" z_bk_ref="" z_bk_imgname=""
    jq -r '
      (."containerimage.digest" // ""),
      (."containerimage.descriptor".mediaType // ""),
      (."buildx.build.ref" // ""),
      (."image.name" // "")
    ' "${z_bkmeta}" > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null || true
    { read -r z_bk_digest
      read -r z_bk_mediatype
      read -r z_bk_ref
      read -r z_bk_imgname
    } < "${ZRBFC_SCRATCH_FILE}"
    test -n "${z_bk_digest}"    && echo "  Output digest:  ${z_bk_digest}"
    test -n "${z_bk_mediatype}" && echo "  Media type:     ${z_bk_mediatype}"
    test -n "${z_bk_ref}"       && echo "  Build ref:      ${z_bk_ref}"
    test -n "${z_bk_imgname}"   && echo "  Image name:     ${z_bk_imgname}"
    # Per-platform digests if present
    local z_bk_platforms=""
    jq -r 'keys[] | select(contains("/"))' "${z_bkmeta}" > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null || true
    z_bk_platforms=$(<"${ZRBFC_SCRATCH_FILE}")
    if test -n "${z_bk_platforms}"; then
      echo "  Per-platform digests:"
      local z_bk_plat=""
      local z_bk_pd=""
      while IFS= read -r z_bk_plat; do
        jq -r --arg p "${z_bk_plat}" '.[$p]["containerimage.digest"] // empty' "${z_bkmeta}" \
          > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null || true
        z_bk_pd=$(<"${ZRBFC_SCRATCH_FILE}")
        test -n "${z_bk_pd}" && echo "    ${z_bk_plat}: ${z_bk_pd}"
      done <<< "${z_bk_platforms}"
    fi
  fi

  # Build cache delta — conjure only
  if test "${z_vessel_mode}" != "bind"; then
    local -r z_cache_before="${z_dir}/cache_before.json"
    local -r z_cache_after="${z_dir}/cache_after.json"
    if test -f "${z_cache_after}"; then
      echo ""
      echo "  -- Build Cache Delta --------------------------------------------"
      echo "  Images on the Cloud Build worker after vs before this build."
      echo ""
      local z_before_count="n/a"
      local z_after_count=""
      if test -f "${z_cache_before}"; then
        jq '.host_daemon_images | length' "${z_cache_before}" > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null \
          || echo "?" > "${ZRBFC_SCRATCH_FILE}"
        z_before_count=$(<"${ZRBFC_SCRATCH_FILE}")
      fi
      jq '.host_daemon_images | length' "${z_cache_after}" > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null \
        || echo "?" > "${ZRBFC_SCRATCH_FILE}"
      z_after_count=$(<"${ZRBFC_SCRATCH_FILE}")
      echo "  Images before: ${z_before_count}"
      echo "  Images after:  ${z_after_count}"
      if test -f "${z_cache_before}"; then
        local z_new_images=""
        jq -r --slurpfile before "${z_cache_before}" '
          ($before[0].host_daemon_images // [] | map(.ID) | unique) as $before_ids |
          [(.host_daemon_images // [])[] |
           select(.ID as $id | $before_ids | index($id) | not)] |
          group_by(.ID) |
          map(.[0] |
            (if (.Repository | split("/") | length) > 2 then
              (.Repository | split("/") | .[-1])
            else .Repository end) as $short |
            [$short, .Tag, .Size, .ID[7:19]] | @tsv) |
          .[]
        ' "${z_cache_after}" > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null || true
        z_new_images=$(<"${ZRBFC_SCRATCH_FILE}")
        if test -n "${z_new_images}"; then
          local z_new_count=0
          local z_count_line=""
          while IFS= read -r z_count_line; do
            z_new_count=$((z_new_count + 1))
          done <<< "${z_new_images}"
          echo ""
          echo "  New images (${z_new_count} unique):"
          printf '%s\n' "${z_new_images}" | while IFS=$'\t' read -r z_repo z_tag z_size z_id; do
            echo "    ${z_id}  ${z_repo}:${z_tag}  ${z_size}"
          done
        else
          echo "  No new images (cache unchanged)"
        fi
      fi
    fi
  fi

  # SBOM — present for both bind and conjure (if syft was available)
  echo ""
  echo "  -- SBOM Summary (syft) ------------------------------------------"
  echo "  Software bill of materials: every package syft found installed."
  echo ""
  if test -f "${z_sbom}"; then
    local z_pkg_count=""
    jq '.artifacts | length' "${z_sbom}" > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null \
      || echo "?" > "${ZRBFC_SCRATCH_FILE}"
    z_pkg_count=$(<"${ZRBFC_SCRATCH_FILE}")
    echo "  Package count:  ${z_pkg_count}"

    echo "  Package types:"
    jq -r '
      [.artifacts[]?.type // empty] | group_by(.) |
      map({type: .[0], count: length}) |
      sort_by(-.count)[] |
      "    \(.count)\t\(.type)"
    ' "${z_sbom}" 2>/dev/null || echo "    (unable to parse)"

    echo ""
    echo "  Syft inventories installed packages. This is not a security"
    echo "  assessment, vulnerability scan, or license audit."
  else
    echo "  sbom.json not found in -about artifact"
  fi

  # Vouch — branched by vessel mode
  echo ""
  echo "  -- Vouch Results ------------------------------------------------"
  if test "${z_vessel_mode}" = "bind"; then
    echo "  Bind verification: was the mirrored image verified against its digest pin?"
    echo ""
    if test "${z_has_vouch}" = "true" && test -f "${z_vs}"; then
      local z_vf_method="" z_vf_result="" z_vf_pin="" z_vf_gar=""
      local z_vf_match="" z_vf_ts="" z_vf_source=""
      jq -r '
        (.verification.method // "?"),
        (.verification.result // .verification.verdict // "?"),
        (.verification.pin_digest // .verification.pinned_digest // "?"),
        (.verification.gar_digest // .verification.actual_digest // "?"),
        (.verification.digest_match // "?"),
        (.verification.timestamp // "?"),
        (.verification.source_image // .verification.bind_source // "?")
      ' "${z_vs}" > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null || true
      { read -r z_vf_method
        read -r z_vf_result
        read -r z_vf_pin
        read -r z_vf_gar
        read -r z_vf_match
        read -r z_vf_ts
        read -r z_vf_source
      } < "${ZRBFC_SCRATCH_FILE}"
      echo "  Method:      ${z_vf_method}"
      echo "  Verdict:     ${z_vf_result}"
      echo "  Pin digest:  ${z_vf_pin}"
      echo "  GAR digest:  ${z_vf_gar}"
      echo "  Match:       ${z_vf_match}"
      echo "  Timestamp:   ${z_vf_ts}"
      echo "  Source:      ${z_vf_source}"
    else
      echo "  Vouch artifact not found in GAR"
    fi
  elif test "${z_vessel_mode}" = "graft"; then
    echo "  Graft acknowledgment: no provenance chain — GRAFTED verdict"
    echo ""
    if test "${z_has_vouch}" = "true" && test -f "${z_vs}"; then
      local z_gf_verdict="" z_gf_source="" z_gf_method=""
      jq -r '
        (.verification.verdict // "?"),
        (.verification.graft_source // "?"),
        (.verification.method // "?")
      ' "${z_vs}" > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null || true
      { read -r z_gf_verdict
        read -r z_gf_source
        read -r z_gf_method
      } < "${ZRBFC_SCRATCH_FILE}"
      echo "  Verdict:     ${z_gf_verdict}"
      echo "  Method:      ${z_gf_method}"
      echo "  Source:      ${z_gf_source}"
    else
      echo "  Vouch artifact not found in GAR"
    fi
  else
    echo "  Independent SLSA verification: did this image pass provenance checks?"
    echo ""
    if test "${z_has_vouch}" = "true" && test -f "${z_vs}"; then
      local z_verifier_url="" z_verifier_sha=""
      jq -r '(.verifier.url // "?"), (.verifier.sha256 // "?")' "${z_vs}" \
        > "${ZRBFC_SCRATCH_FILE}" 2>/dev/null || true
      { read -r z_verifier_url; read -r z_verifier_sha; } < "${ZRBFC_SCRATCH_FILE}"
      echo "  Verifier:"
      echo "    URL:    ${z_verifier_url}"
      echo "    SHA256: ${z_verifier_sha}"
      echo ""
      echo "  Per-platform verdicts:"
      jq -r '.platforms[]? | "    \(.platform): \(.verdict)"' "${z_vs}" 2>/dev/null \
        || echo "    (unable to parse)"
    else
      echo "  Vouch artifact not found in GAR"
    fi
  fi
}

# Internal: display compact vessel info (conjure or bind)
# Args: vessel hallmark extract_dir has_vouch
zrbfc_plumb_show_compact() {
  zrbfc_sentinel

  local -r z_vessel="$1"
  local -r z_hallmark="$2"
  local -r z_dir="$3"
  local -r z_has_vouch="$4"

  echo ""
  echo "================================================================"
  echo "  HALLMARK PLUMB: ${z_vessel} / ${z_hallmark}"
  echo "================================================================"

  zrbfc_plumb_show_sections "${z_dir}" "${z_has_vouch}"

  echo ""
  echo "================================================================"
  echo ""
}

# Internal: display full vessel info (conjure or bind)
# Adds per-package inventory and Dockerfile (conjure only) to the compact sections.
# Args: vessel hallmark extract_dir has_vouch
zrbfc_plumb_show_full() {
  zrbfc_sentinel

  local -r z_vessel="$1"
  local -r z_hallmark="$2"
  local -r z_dir="$3"
  local -r z_has_vouch="$4"

  local -r z_sbom="${z_dir}/sbom.json"

  echo ""
  echo "================================================================"
  echo "  HALLMARK PLUMB (FULL): ${z_vessel} / ${z_hallmark}"
  echo "================================================================"

  zrbfc_plumb_show_sections "${z_dir}" "${z_has_vouch}"

  echo ""
  echo "  -- Package Inventory --------------------------------------------"
  echo "  Every package syft detected, sorted by ecosystem type."
  echo ""
  if test -f "${z_sbom}"; then
    printf "    %-12s %-36s %s\n" "TYPE" "NAME" "VERSION"
    printf "    %-12s %-36s %s\n" "----" "----" "-------"
    jq -r '
      .artifacts[]? |
      [.type // "?", .name // "?", .version // "?"] |
      @tsv
    ' "${z_sbom}" 2>/dev/null | sort | while IFS=$'\t' read -r z_type z_name z_ver; do
      printf "    %-12s %-36s %s\n" "${z_type}" "${z_name}" "${z_ver}"
    done
  else
    echo "    sbom.json not found in -about artifact"
  fi

  echo ""
  echo "  -- Package Licensing & Identity ---------------------------------"
  echo "  License and Package URL for each package (for compliance review)."
  echo ""
  if test -f "${z_sbom}"; then
    printf "    %-36s %-20s %s\n" "NAME" "LICENSE" "PURL"
    printf "    %-36s %-20s %s\n" "----" "-------" "----"
    jq -r '
      .artifacts[]? |
      [
        (.name // "?"),
        ((.licenses // []) | map(.value // .expression // empty) | join(", ") | if . == "" then "-" else . end),
        (.purl // "-")
      ] |
      @tsv
    ' "${z_sbom}" 2>/dev/null | sort | while IFS=$'\t' read -r z_name z_lic z_purl; do
      printf "    %-36s %-20s %s\n" "${z_name}" "${z_lic}" "${z_purl}"
    done
  else
    echo "    sbom.json not found in -about artifact"
  fi

  local -r z_recipe="${z_dir}/recipe.txt"
  if test -f "${z_recipe}"; then
    echo ""
    echo "  -- Recipe (Dockerfile) ------------------------------------------"
    echo "  The exact Dockerfile used to build this image."
    echo ""
    while IFS= read -r z_line; do
      echo "    ${z_line}"
    done < "${z_recipe}"
  fi

  echo ""
  echo "================================================================"
  echo ""
}

######################################################################
# Public Functions (rbfc_*)

rbfc_plumb_full() {
  zrbfc_sentinel

  local z_vessel="${1:-}"
  local z_hallmark="${2:-}"

  # Single-arg: hallmark-only mode (lookup vessel via VOUCHES superdirectory)
  if test -n "${z_vessel}" && test -z "${z_hallmark}"; then
    z_hallmark="${z_vessel}"
    z_vessel=""
  fi

  buc_doc_brief "Plumb a hallmark's trust posture (full detail)"
  buc_doc_param "vessel" "Vessel name (omit for hallmark-only lookup via VOUCHES)"
  buc_doc_param "hallmark" "Full hallmark (e.g., c260305133650-r260305160530)"
  buc_doc_shown || return 0

  zrbfc_plumb_core "${z_vessel}" "${z_hallmark}" "full"
}

rbfc_plumb_compact() {
  zrbfc_sentinel

  local z_vessel="${1:-}"
  local z_hallmark="${2:-}"

  # Single-arg: hallmark-only mode (lookup vessel via VOUCHES superdirectory)
  if test -n "${z_vessel}" && test -z "${z_hallmark}"; then
    z_hallmark="${z_vessel}"
    z_vessel=""
  fi

  buc_doc_brief "Plumb a hallmark's trust posture (compact summary)"
  buc_doc_param "vessel" "Vessel name (omit for hallmark-only lookup via VOUCHES)"
  buc_doc_param "hallmark" "Full hallmark (e.g., c260305133650-r260305160530)"
  buc_doc_shown || return 0

  zrbfc_plumb_core "${z_vessel}" "${z_hallmark}" "compact"
}

# eof
