#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle GitHub Actions - Generic workflow dispatch

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBHA_INCLUDED:-}" || buc_die "Module rbgA multiply included - check sourcing hierarchy"
ZRBHA_INCLUDED=1

######################################################################
# Internal Functions (zrbha_*)

zrbha_kindle() {
  # Check required environment
  test -n "${RBRG_PAT:-}"      || buc_die "RBRG_PAT not set"
  test -n "${BURD_TEMP_DIR:-}"  || buc_die "BURD_TEMP_DIR not set"
  test -n "${BURD_NOW_STAMP:-}" || buc_die "BURD_NOW_STAMP not set"

  # Module Variables (zrbha_*)
  ZRBHA_GITAPI_URL="https://api.github.com"
  ZRBHA_MTYPE_GHV3="application/vnd.github.v3+json"

  ZRBHA_WORKFLOW_RUN_ID_FILE="${BURD_TEMP_DIR}/workflow_run_id__${BURD_NOW_STAMP}.txt"
  ZRBHA_WORKFLOW_LOGS_FILE="${BURD_TEMP_DIR}/workflow_logs__${BURD_NOW_STAMP}.txt"

  ZRBHA_KINDLED=1
}

zrbha_sentinel() {
  test "${ZRBHA_KINDLED:-}" = "1" || buc_die "Module rbha not kindled - call zrbha_kindle first"
}

zrbha_curl_api() {
  local z_url="$1"

  curl -s                                    \
       -H "Authorization: token ${RBRG_PAT}" \
       -H "Accept: ${ZRBHA_MTYPE_GHV3}"      \
       "${z_url}"
}

zrbha_curl_post() {
  local z_url="$1"
  local z_data="$2"

  curl -s                                    \
       -X POST                               \
       -H "Authorization: token ${RBRG_PAT}" \
       -H "Accept: ${ZRBHA_MTYPE_GHV3}"      \
       "${z_url}"                            \
       -d "${z_data}"                        \
    || buc_die "POST request to GitHub API failed"
}

######################################################################
# External Functions (rbha_*)

rbha_dispatch() {
  # Name parameters
  local z_repo_owner="${1:-}"
  local z_repo_name="${2:-}"
  local z_event_type="${3:-}"
  local z_payload_json="${4:-}"

  # Ensure module started
  zrbha_sentinel

  # Validate parameters
  test -n "${z_repo_owner}"   || buc_die "Repository owner required"
  test -n "${z_repo_name}"    || buc_die "Repository name required"
  test -n "${z_event_type}"   || buc_die "Event type required"
  test -n "${z_payload_json}" || buc_die "Payload JSON required"

  # Dispatch workflow
  buc_info "Trigger workflow..."
  local z_dispatch_url="${ZRBHA_GITAPI_URL}/repos/${z_repo_owner}/${z_repo_name}/dispatches"
  local z_dispatch_data='{"event_type": "'${z_event_type}'", "client_payload": '${z_payload_json}'}'

  zrbha_curl_post "${z_dispatch_url}" "${z_dispatch_data}"

  # Wait for workflow to start
  buc_info "Waiting for workflow to start..."
  sleep 5

  # Get workflow run ID
  buc_info "Retrieve workflow run ID..."
  local z_runs_url="${ZRBHA_GITAPI_URL}/repos/${z_repo_owner}/${z_repo_name}/actions/runs?event=repository_dispatch&branch=main&per_page=1"
  zrbha_curl_api "${z_runs_url}" | jq -r '.workflow_runs[0].id' > "${ZRBHA_WORKFLOW_RUN_ID_FILE}"

  test -s "${ZRBHA_WORKFLOW_RUN_ID_FILE}" || buc_die "Failed to get workflow run ID"

  local z_run_id
  z_run_id=$(<"${ZRBHA_WORKFLOW_RUN_ID_FILE}")

  buc_info "Workflow online at:"
  echo -e "${ZBUC_YELLOW}   https://github.com/${z_repo_owner}/${z_repo_name}/actions/runs/${z_run_id}${ZBUC_RESET}"
}

rbha_wait_completion() {
  # Name parameters
  local z_repo_owner="${1:-}"
  local z_repo_name="${2:-}"

  # Ensure module started
  zrbha_sentinel

  # Validate parameters
  test -n "${z_repo_owner}" || buc_die "Repository owner required"
  test -n "${z_repo_name}" || buc_die "Repository name required"

  # Get run ID
  test -s "${ZRBHA_WORKFLOW_RUN_ID_FILE}" || buc_die "No workflow run ID found - dispatch first"
  local z_run_id
  z_run_id=$(<"${ZRBHA_WORKFLOW_RUN_ID_FILE}")

  # Poll for completion
  buc_info "Polling to completion..."
  local z_status=""
  local z_conclusion=""

  while true; do
    local z_run_url="${ZRBHA_GITAPI_URL}/repos/${z_repo_owner}/${z_repo_name}/actions/runs/${z_run_id}"
    local z_response
    z_response=$(zrbha_curl_api "${z_run_url}")

    z_status=$(echo "${z_response}" | jq -r '.status')
    z_conclusion=$(echo "${z_response}" | jq -r '.conclusion')

    buc_step "  Status: ${z_status}"

    if test "${z_status}" = "completed"; then
    buc_step "    Conclusion: ${z_conclusion}"
      break
    fi

    sleep 3
  done

  test "${z_conclusion}" = "success" || buc_die "Workflow fail: ${z_conclusion}"

  # Clean up run ID file on success
  rm -f "${ZRBHA_WORKFLOW_RUN_ID_FILE}"
}

rbha_get_logs() {
  # Name parameters
  local z_repo_owner="${1:-}"
  local z_repo_name="${2:-}"
  local z_run_id="${3:-}"

  # Ensure module started
  zrbha_sentinel

  # Validate parameters
  test -n "${z_repo_owner}" || buc_die "Repository owner required"
  test -n "${z_repo_name}" || buc_die "Repository name required"
  test -n "${z_run_id}" || buc_die "Run ID required"

  # Get logs
  buc_info "Pull logs..."
  local z_logs_url="${ZRBHA_GITAPI_URL}/repos/${z_repo_owner}/${z_repo_name}/actions/runs/${z_run_id}/logs"
  zrbha_curl_api "${z_logs_url}" > "${ZRBHA_WORKFLOW_LOGS_FILE}"
}

# eof

