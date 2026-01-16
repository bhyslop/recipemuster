#!/bin/bash
#
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
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# JJU Utility - Job Jockey Gallops operations (thin wrappers around vvx)
#
# This file is sourced by jju_cli.sh. All operations delegate to the
# vvx Rust binary which handles Gallops JSON manipulation.
#
# Terminology:
#   Firemark: Heat identity (₣XX format, 2 base64 chars)
#   Coronet:  Pace identity (₢XXXXX format, 5 base64 chars, embeds parent Firemark)
#   Gallops:  The JSON file tracking heats and paces (.claude/jjm/jjg_gallops.json)

set -euo pipefail

# Multiple inclusion detection
test -z "${ZJJU_SOURCED:-}" || buc_die "Module jju multiply sourced"
ZJJU_SOURCED=1

######################################################################
# Boilerplate Functions

zjju_kindle() {
  test -z "${ZJJU_KINDLED:-}" || buc_die "Module jju already kindled"

  # Validate BUD environment
  buv_dir_exists "${BUD_TEMP_DIR}"
  test -n "${BUD_NOW_STAMP:-}" || buc_die "BUD_NOW_STAMP is unset"

  # Internal constants
  ZJJU_GALLOPS_FILE=".claude/jjm/jjg_gallops.json"
  ZJJU_PADDOCK_DIR=".claude/jjm"
  ZJJU_TROPHY_DIR=".claude/jjm/retired"

  # Find vvx binary
  ZJJU_VVX_BIN=""
  if command -v vvx >/dev/null 2>&1; then
    ZJJU_VVX_BIN="vvx"
  elif test -x "Tools/vvk/bin/vvx"; then
    ZJJU_VVX_BIN="Tools/vvk/bin/vvx"
  else
    buc_die "vvx binary not found (install VVK or add to PATH)"
  fi

  ZJJU_KINDLED=1
}

zjju_sentinel() {
  test "${ZJJU_KINDLED:-}" = "1" || buc_die "Module jju not kindled"
}

######################################################################
# Gallops Operations (delegate to vvx)

jju_muster() {
  zjju_sentinel
  buc_doc_brief "List heats with Firemarks and silks"
  buc_doc_shown || return 0

  "${ZJJU_VVX_BIN}" jjx_muster --file "${ZJJU_GALLOPS_FILE}"
}

jju_saddle() {
  zjju_sentinel
  local z_firemark="${1:-}"

  buc_doc_brief "Get context for heat (JSON output)"
  buc_doc_param "firemark" "Heat Firemark (₣XX or bare XX)"
  buc_doc_shown || return 0

  test -n "${z_firemark}" || buc_die "Parameter 'firemark' is required"

  "${ZJJU_VVX_BIN}" jjx_saddle --file "${ZJJU_GALLOPS_FILE}" "${z_firemark}"
}

jju_nominate() {
  zjju_sentinel
  local z_silks="${1:-}"

  buc_doc_brief "Create new heat"
  buc_doc_param "silks" "Kebab-case identifier"
  buc_doc_shown || return 0

  test -n "${z_silks}" || buc_die "Parameter 'silks' is required"

  local z_created="${BUD_NOW_STAMP:0:6}"
  "${ZJJU_VVX_BIN}" jjx_nominate --file "${ZJJU_GALLOPS_FILE}" --silks "${z_silks}" --created "${z_created}"
}

jju_slate() {
  zjju_sentinel
  local z_firemark="${1:-}"
  local z_silks="${2:-}"
  local z_text="${3:-}"

  buc_doc_brief "Add new pace to heat"
  buc_doc_param "firemark" "Heat Firemark (₣XX or bare XX)"
  buc_doc_param "silks" "Kebab-case pace identifier"
  buc_doc_param "text" "Initial pace specification (optional, reads stdin if empty)"
  buc_doc_shown || return 0

  test -n "${z_firemark}" || buc_die "Parameter 'firemark' is required"
  test -n "${z_silks}" || buc_die "Parameter 'silks' is required"

  if test -n "${z_text}"; then
    echo "${z_text}" | "${ZJJU_VVX_BIN}" jjx_slate --file "${ZJJU_GALLOPS_FILE}" "${z_firemark}" --silks "${z_silks}"
  else
    "${ZJJU_VVX_BIN}" jjx_slate --file "${ZJJU_GALLOPS_FILE}" "${z_firemark}" --silks "${z_silks}"
  fi
}

jju_rail() {
  zjju_sentinel
  local z_firemark="${1:-}"
  shift || true
  local z_order="$*"

  buc_doc_brief "Reorder paces in heat"
  buc_doc_param "firemark" "Heat Firemark (₣XX or bare XX)"
  buc_doc_param "order..." "Coronets in new order"
  buc_doc_shown || return 0

  test -n "${z_firemark}" || buc_die "Parameter 'firemark' is required"
  test -n "${z_order}" || buc_die "Parameter 'order' is required"

  # Convert space-separated to JSON array
  local z_json_array="["
  local z_first=1
  for z_coronet in ${z_order}; do
    if test "${z_first}" -eq 1; then
      z_json_array="${z_json_array}\"${z_coronet}\""
      z_first=0
    else
      z_json_array="${z_json_array},\"${z_coronet}\""
    fi
  done
  z_json_array="${z_json_array}]"

  "${ZJJU_VVX_BIN}" jjx_rail --file "${ZJJU_GALLOPS_FILE}" "${z_firemark}" --order "${z_json_array}"
}

jju_tally() {
  zjju_sentinel
  local z_coronet="${1:-}"
  local z_state="${2:-}"
  local z_text="${3:-}"

  buc_doc_brief "Add tack to pace (state transition)"
  buc_doc_param "coronet" "Pace Coronet (₢XXXXX or bare XXXXX)"
  buc_doc_param "state" "New state: rough|primed|complete|abandoned"
  buc_doc_param "text" "Tack text (optional, reads stdin if empty)"
  buc_doc_shown || return 0

  test -n "${z_coronet}" || buc_die "Parameter 'coronet' is required"
  test -n "${z_state}" || buc_die "Parameter 'state' is required"

  if test -n "${z_text}"; then
    echo "${z_text}" | "${ZJJU_VVX_BIN}" jjx_tally --file "${ZJJU_GALLOPS_FILE}" "${z_coronet}" --state "${z_state}"
  else
    "${ZJJU_VVX_BIN}" jjx_tally --file "${ZJJU_GALLOPS_FILE}" "${z_coronet}" --state "${z_state}"
  fi
}

jju_parade() {
  zjju_sentinel
  local z_firemark="${1:-}"

  buc_doc_brief "Display comprehensive heat status"
  buc_doc_param "firemark" "Heat Firemark (₣XX or bare XX)"
  buc_doc_shown || return 0

  test -n "${z_firemark}" || buc_die "Parameter 'firemark' is required"

  "${ZJJU_VVX_BIN}" jjx_parade --file "${ZJJU_GALLOPS_FILE}" "${z_firemark}"
}

jju_retire_extract() {
  zjju_sentinel
  local z_firemark="${1:-}"

  buc_doc_brief "Extract heat data for archival (JSON output)"
  buc_doc_param "firemark" "Heat Firemark (₣XX or bare XX)"
  buc_doc_shown || return 0

  test -n "${z_firemark}" || buc_die "Parameter 'firemark' is required"

  "${ZJJU_VVX_BIN}" jjx_retire --file "${ZJJU_GALLOPS_FILE}" "${z_firemark}"
}

######################################################################
# Steeplechase Operations (delegate to vvx)

jju_chalk() {
  zjju_sentinel
  local z_firemark="${1:-}"
  local z_marker="${2:-}"
  local z_description="${3:-}"

  buc_doc_brief "Write steeplechase marker (empty commit)"
  buc_doc_param "firemark" "Heat Firemark (₣XX or bare XX)"
  buc_doc_param "marker" "Marker type: APPROACH|WRAP|FLY|DISCUSSION"
  buc_doc_param "description" "Marker description"
  buc_doc_shown || return 0

  test -n "${z_firemark}" || buc_die "Parameter 'firemark' is required"
  test -n "${z_marker}" || buc_die "Parameter 'marker' is required"
  test -n "${z_description}" || buc_die "Parameter 'description' is required"

  "${ZJJU_VVX_BIN}" jjx_chalk "${z_firemark}" --marker "${z_marker}" --description "${z_description}"
}

jju_notch() {
  zjju_sentinel
  local z_firemark="${1:-}"
  local z_pace_silks="${2:-}"
  local z_message="${3:-}"

  buc_doc_brief "JJ-aware commit with heat/pace prefix"
  buc_doc_param "firemark" "Heat Firemark (₣XX or bare XX)"
  buc_doc_param "pace_silks" "Pace silks for commit prefix"
  buc_doc_param "message" "Commit message (optional, claude generates if empty)"
  buc_doc_shown || return 0

  test -n "${z_firemark}" || buc_die "Parameter 'firemark' is required"
  test -n "${z_pace_silks}" || buc_die "Parameter 'pace_silks' is required"

  if test -n "${z_message}"; then
    "${ZJJU_VVX_BIN}" jjx_notch "${z_firemark}" --pace "${z_pace_silks}" --message "${z_message}"
  else
    "${ZJJU_VVX_BIN}" jjx_notch "${z_firemark}" --pace "${z_pace_silks}"
  fi
}

######################################################################
# High-level Workflows (orchestrate multiple vvx calls)

jju_wrap() {
  zjju_sentinel
  local z_coronet="${1:-}"

  buc_doc_brief "Complete pace: tally complete + chalk WRAP"
  buc_doc_param "coronet" "Pace Coronet (₢XXXXX or bare XXXXX)"
  buc_doc_shown || return 0

  test -n "${z_coronet}" || buc_die "Parameter 'coronet' is required"

  # Get pace info via saddle
  local z_firemark
  if test "${#z_coronet}" -eq 5; then
    z_firemark="${z_coronet:0:2}"
  elif test "${z_coronet:0:1}" = "₢"; then
    z_firemark="${z_coronet:1:2}"
  else
    buc_die "Invalid coronet format: ${z_coronet}"
  fi

  # Mark complete
  echo "Pace completed" | "${ZJJU_VVX_BIN}" jjx_tally --file "${ZJJU_GALLOPS_FILE}" "${z_coronet}" --state complete

  # Get saddle info for the heat to show next pace
  "${ZJJU_VVX_BIN}" jjx_saddle --file "${ZJJU_GALLOPS_FILE}" "${z_firemark}"
}

jju_retire() {
  zjju_sentinel
  local z_firemark="${1:-}"

  buc_doc_brief "Retire heat: extract trophy data"
  buc_doc_param "firemark" "Heat Firemark (₣XX or bare XX)"
  buc_doc_shown || return 0

  test -n "${z_firemark}" || buc_die "Parameter 'firemark' is required"

  # For now, just extract the data - full retire workflow handled elsewhere
  "${ZJJU_VVX_BIN}" jjx_retire --file "${ZJJU_GALLOPS_FILE}" "${z_firemark}"
}

######################################################################
# Legacy compatibility stubs (removed functions)

jju_reslate() {
  buc_die "jju_reslate removed: use jju_tally with text input instead"
}

# Encoding functions removed - handled internally by Rust
# If needed for debugging, use: vvx jjx_validate --file <path>

# eof
