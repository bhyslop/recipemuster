#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle GitHub Actions - Generic workflow dispatch

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGA_INCLUDED:-}" || bcu_die "Module rbga multiply included - check sourcing hierarchy"
ZRBGA_INCLUDED=1

######################################################################
# Internal Functions (zrbga_*)

zrbga_kindle() {
  # Check required environment
  test -n "${RBRG_PAT:-}"      || bcu_die "RBRG_PAT not set"
  test -n "${RBG_TEMP_DIR:-}"  || bcu_die "RBG_TEMP_DIR not set"
  test -n "${RBG_NOW_STAMP:-}" || bcu_die "RBG_NOW_STAMP not set"

  # Module Variables (ZRBGA_*)
  ZRBGA_GITAPI_URL="https://api.github.com"
  ZRBGA_MTYPE_GHV3="application/vnd.github.v3+json"

  ZRBGA_WORKFLOW_RUN_ID_FILE="${RBG_TEMP_DIR}/workflow_run_id__${RBG_NOW_STAMP}.txt"
  ZRBGA_WORKFLOW_LOGS_FILE="${RBG_TEMP_DIR}/workflow_logs__${RBG_NOW_STAMP}.txt"

  ZRBGA_KINDLED=1
}

zrbga_sentinel() {
  test "${ZRBGA_KINDLED:-}" = "1" || bcu_die "Module rbga not kindled - call zrbga_kindle first"
}

zrbga_curl_api() {
  local z_url="$1"

  curl -s                                    \
       -H "Authorization: token ${RBRG_PAT}" \
       -H "Accept: ${ZRBGA_MTYPE_GHV3}"      \
       "${z_url}"
}

zrbga_curl_post() {
  local z_url="$1"
  local z_data="$2"

  curl -s                                    \
       -X POST                               \
       -H "Authorization: token ${RBRG_PAT}" \
       -H "Accept: ${ZRBGA_MTYPE_GHV3}"      \
       "${z_url}"                            \
       -d "${z_data}"                        \
    || bcu_die "POST request to GitHub API failed"
}

######################################################################
# External Functions (rbga_*)

rbga_dispatch() {
  # Name parameters
  local z_repo_owner="${1:-}"
  local z_repo_name="${2:-}"
  local z_event_type="${3:-}"
  local z_payload_json="${4:-}"

  # Ensure module started
  zrbga_sentinel

  # Validate parameters
  test -n "${z_repo_owner}"   || bcu_die "Repository owner required"
  test -n "${z_repo_name}"    || bcu_die "Repository name required"
  test -n "${z_event_type}"   || bcu_die "Event type required"
  test -n "${z_payload_json}" || bcu_die "Payload JSON required"

  # Dispatch workflow
  bcu_info "Trigger workflow..."
  local z_dispatch_url="${ZRBGA_GITAPI_URL}/repos/${z_repo_owner}/${z_repo_name}/dispatches"
  local z_dispatch_data='{"event_type": "'${z_event_type}'", "client_payload": '${z_payload_json}'}'

  zrbga_curl_post "${z_dispatch_url}" "${z_dispatch_data}"

  # Wait for workflow to start
  bcu_info "Waiting for workflow to start..."
  sleep 5

  # Get workflow run ID
  bcu_info "Retrieve workflow run ID..."
  local z_runs_url="${ZRBGA_GITAPI_URL}/repos/${z_repo_owner}/${z_repo_name}/actions/runs?event=repository_dispatch&branch=main&per_page=1"
  zrbga_curl_api "${z_runs_url}" | jq -r '.workflow_runs[0].id' > "${ZRBGA_WORKFLOW_RUN_ID_FILE}"

  test -s "${ZRBGA_WORKFLOW_RUN_ID_FILE}" || bcu_die "Failed to get workflow run ID"

  local z_run_id
  z_run_id=$(<"${ZRBGA_WORKFLOW_RUN_ID_FILE}")

  bcu_info "Workflow online at:"
  echo -e "${ZBCU_YELLOW}   https://github.com/${z_repo_owner}/${z_repo_name}/actions/runs/${z_run_id}${ZBCU_RESET}"
}

rbga_wait_completion() {
  # Name parameters
  local z_repo_owner="${1:-}"
  local z_repo_name="${2:-}"

  # Ensure module started
  zrbga_sentinel

  # Validate parameters
  test -n "${z_repo_owner}" || bcu_die "Repository owner required"
  test -n "${z_repo_name}" || bcu_die "Repository name required"

  # Get run ID
  test -s "${ZRBGA_WORKFLOW_RUN_ID_FILE}" || bcu_die "No workflow run ID found - dispatch first"
  local z_run_id
  z_run_id=$(<"${ZRBGA_WORKFLOW_RUN_ID_FILE}")

  # Poll for completion
  bcu_info "Polling to completion..."
  local z_status=""
  local z_conclusion=""

  while true; do
    local z_run_url="${ZRBGA_GITAPI_URL}/repos/${z_repo_owner}/${z_repo_name}/actions/runs/${z_run_id}"
    local z_response
    z_response=$(zrbga_curl_api "${z_run_url}")

    z_status=$(echo "${z_response}" | jq -r '.status')
    z_conclusion=$(echo "${z_response}" | jq -r '.conclusion')

    bcu_step "  Status: ${z_status}"

    if test "${z_status}" = "completed"; then
    bcu_step "    Conclusion: ${z_conclusion}"
      break
    fi

    sleep 3
  done

  test "${z_conclusion}" = "success" || bcu_die "Workflow fail: ${z_conclusion}"

  # Clean up run ID file on success
  rm -f "${ZRBGA_WORKFLOW_RUN_ID_FILE}"
}

rbga_get_logs() {
  # Name parameters
  local z_repo_owner="${1:-}"
  local z_repo_name="${2:-}"
  local z_run_id="${3:-}"

  # Ensure module started
  zrbga_sentinel

  # Validate parameters
  test -n "${z_repo_owner}" || bcu_die "Repository owner required"
  test -n "${z_repo_name}" || bcu_die "Repository name required"
  test -n "${z_run_id}" || bcu_die "Run ID required"

  # Get logs
  bcu_info "Pull logs..."
  local z_logs_url="${ZRBGA_GITAPI_URL}/repos/${z_repo_owner}/${z_repo_name}/actions/runs/${z_run_id}/logs"
  zrbga_curl_api "${z_logs_url}" > "${ZRBGA_WORKFLOW_LOGS_FILE}"
}

# eof

