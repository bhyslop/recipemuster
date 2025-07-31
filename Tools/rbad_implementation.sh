#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
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
# Recipe Bottle Action Dispatch - Implementation Layer

set -euo pipefail


zrbad_init() {
  test -n "${RBRR_REGISTRY_OWNER:-}" || bcu_die "RBRR_REGISTRY_OWNER not set"
  test -n "${RBRR_REGISTRY_NAME:-}"  || bcu_die "RBRR_REGISTRY_NAME not set"
  test -n "${RBRR_HISTORY_DIR:-}"    || bcu_die "RBRR_HISTORY_DIR not set"
  test -n "${RBRG_PAT:-}"            || bcu_die "RBRG_PAT not set"
  test -n "${RBRG_USERNAME:-}"       || bcu_die "RBRG_USERNAME not set"
  test -n "${RBAD_TEMP_DIR:-}"       || bcu_die "RBAD_TEMP_DIR not set"
  test -n "${RBAD_NOW_STAMP:-}"      || bcu_die "RBAD_NOW_STAMP not set"

  ZRBAD_DISPATCH_URL="${RBADI_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/dispatches"
  ZRBAD_RUNS_URL_BASE="${RBADI_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs"
  ZRBAD_GITHUB_ACTIONS_URL="https://github.com/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs/"
  ZRBAD_CURL_CACHE="${RBAD_TEMP_DIR}/ZRBAD_CURL_GET_CACHE"
  ZRBAD_RUN_CACHE="${RBAD_TEMP_DIR}/CURR_WORKFLOW_RUN__${RBAD_NOW_STAMP}.txt"
  ZRBAD_MTYPE_GHV3="application/vnd.github.v3+json"
  ZRBAD_INITIALIZATION_COMPLETE="Initialization complete"
}

zrbad_is_initialized() {
  test "${ZRBAD_INITIALIZATION_COMPLETE:-}" = "Initialization complete" \
    || bcu_die "zrbad_init() not called"
}


######################################################################
# GitHub API Functions (rbadi_*)

# Perform authenticated GET request
zrbad_curl_get() {
  local url="$1"

  zrbad_is_initialized || bcu_die "Needed variables not set."

  curl -s                                    \
       -H "Authorization: token ${RBRG_PAT}" \
       -H "Accept: ${ZRBAD_MTYPE_GHV3}"      \
       "${url}"                              \
                   > "${ZRBAD_CURL_CACHE}"   \
    || bcu_die "Curl get failed."
}

# Perform authenticated POST request
rbadi_curl_post() {
  local url="$1"
  local data="$2"

  zrbad_is_initialized || bcu_die "Needed variables not set."

  curl -s                                       \
       -X POST                                  \
       -H "Authorization: token ${RBRG_PAT}"    \
       -H "Accept: ${ZRBAD_MTYPE_GHV3}"         \
       "$url"                                   \
       -d "$data"                               \
    || bcu_die "Curl post failed"
}

# Dispatch a workflow
rbadi_dispatch_workflow() {
  local event_type="$1"
  local payload_json="$2"

  zrbad_is_initialized || bcu_die "Needed variables not set."

  local dispatch_data='{"event_type": "'${event_type}'", "client_payload": '${payload_json}'}'
  rbadi_curl_post "${RBADI_DISPATCH_URL}" "${dispatch_data}"
}

# Get latest workflow run ID
rbadi_get_latest_run_id() {

  zrbad_is_initialized || bcu_die "Needed variables not set."

  local runs_url="${RBADI_RUNS_URL_BASE}?event=repository_dispatch&branch=main&per_page=1"
  zrbad_curl_get "${runs_url}"
  jq -r '.workflow_runs[0].id' < "${ZRBAD_CURL_CACHE}"
}

# Wait for workflow to complete
rbadi_wait_for_workflow() {
  rbadi_validate_env || return 1
  rbadi_init_vars

  local run_id="$1"
  local success_message="${2:-Workflow completed}"

  echo "Workflow online at:"
  echo "   ${RBADI_GITHUB_ACTIONS_URL}${run_id}"
  echo ""
  echo "Polling to completion..."

  local status=""
  local conclusion=""

  while true; do
    local run_url="${RBADI_RUNS_URL_BASE}/${run_id}"
    rbadi_curl_get "${run_url}"
    local response
    response=$()

    status=$(echo "${response}" | jq -r '.status')
    conclusion=$(echo "${response}" | jq -r '.conclusion')

    echo "  Status: ${status}    Conclusion: ${conclusion}"

    if [ "${status}" = "completed" ]; then
      break
    fi

    sleep 3
  done

  if [ "${conclusion}" != "success" ]; then
    echo "Error: Workflow failed with conclusion: ${conclusion}" >&2
    return 1
  fi

  echo "${success_message}"
  return 0
}

# Fetch workflow logs
rbadi_fetch_workflow_logs() {
  rbadi_validate_env || return 1
  rbadi_init_vars

  local run_id="$1"
  local output_file="$2"

  rbadi_curl_get "${RBADI_RUNS_URL_BASE}/${run_id}/logs" > "${output_file}"
}

# Execute workflow and wait for completion with git pull
rbadi_execute_workflow() {
  rbadi_validate_env || return 1
  rbadi_init_vars

  local event_type="$1"
  local payload_json="$2"
  local success_message="${3:-Workflow completed}"
  local no_commits_msg="${4:-No new commits after many attempts}"

  echo "Trigger workflow..."
  rbadi_dispatch_workflow "${event_type}" "${payload_json}" || return 1

  echo "Polling for completion..."
  sleep 5

  echo "Retrieve workflow run ID..."
  local run_id
  run_id=$(rbadi_get_latest_run_id)
  test -n "${run_id}" && test "${run_id}" != "null" || {
    echo "Error: Failed to get workflow run ID" >&2
    return 1
  }

  echo "${run_id}" > "${RBADI_CURRENT_WORKFLOW_RUN_CACHE}"

  rbadi_wait_for_workflow "${run_id}" "${success_message}" || return 1

  echo "Git pull with retry..."

  local retry_wait=5
  local max_attempts=30
  local i=0
  local found=0

  while [ ${i} -lt ${max_attempts} ]; do
    echo "  Attempt $((i + 1)): Checking for remote changes..."
    git fetch --quiet

    local count
    count=$(git rev-list --count HEAD..origin/main 2>/dev/null || echo 0)

    if [ "${count}" -gt 0 ]; then
      echo "  Found new commits, pulling..."
      git pull
      echo "  Pull successful"
      found=1
      break
    fi

    echo "  No new commits yet, waiting ${retry_wait} seconds (attempt $((i + 1)))"
    sleep ${retry_wait}
    i=$((i + 1))
  done

  if [ ${found} -ne 1 ]; then
    echo "  ${no_commits_msg}"
    echo "Error: No commits found" >&2
    return 1
  fi

  echo "Pull logs..."
  rbadi_fetch_workflow_logs "${run_id}" "${RBAD_TEMP_DIR}/workflow_logs__${RBAD_NOW_STAMP}.txt"

  echo "Everything went right, delete the run cache..."
  rm -f "${RBADI_CURRENT_WORKFLOW_RUN_CACHE}"

  return 0
}

# eof
