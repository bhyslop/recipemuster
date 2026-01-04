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
# JJU Utility - Job Jockey studbook operations
#
# This file is sourced by jju_cli.sh. All functions use buc_doc_*
# for introspection and end with buc_die until implemented.

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
  ZJJU_STUDBOOK_FILE=".claude/jjm/jjs_studbook.json"
  ZJJU_PADDOCK_DIR=".claude/jjm"
  ZJJU_TROPHY_DIR=".claude/jjm/retired"

  ZJJU_KINDLED=1
}

zjju_sentinel() {
  test "${ZJJU_KINDLED:-}" = "1" || buc_die "Module jju not kindled"
}

######################################################################
# Favor Encoding/Decoding
#
# Favor format: HHPPP (5 URL-safe base64 digits)
# - HH = heat (2 digits, 0-4095)
# - PPP = pace (3 digits, 0-262143, 000 = heat-only reference)
# Character set: A-Za-z0-9-_ (64 chars, URL-safe)

# Internal: URL-safe base64 character set (64 chars)
# Position 0-25: A-Z, 26-51: a-z, 52-61: 0-9, 62: -, 63: _
zjju_favor_charset() {
  echo "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"
}

# Internal: Get character at position in charset
zjju_favor_char_at() {
  local z_pos="${1}"
  local z_charset
  z_charset="$(zjju_favor_charset)"
  echo "${z_charset:${z_pos}:1}"
}

# Internal: Get position of character in charset (-1 if not found)
zjju_favor_pos_of() {
  local z_char="${1}"
  local z_charset
  local z_pos=0

  z_charset="$(zjju_favor_charset)"

  while test "${z_pos}" -lt 64; do
    if test "${z_charset:${z_pos}:1}" = "${z_char}"; then
      echo "${z_pos}"
      return 0
    fi
    z_pos=$((z_pos + 1))
  done

  echo "-1"
}

# Internal: Encode heat+pace into 5-digit Favor
# Args: heat (0-4095), pace (0-262143)
# Output: 5-char Favor string (e.g., "KbAAB")
zjju_favor_encode() {
  zjju_sentinel
  local z_heat="${1:-}"
  local z_pace="${2:-}"

  # Validate parameters
  test -n "${z_heat}" || buc_die "Parameter 'heat' is required"
  test -n "${z_pace}" || buc_die "Parameter 'pace' is required"

  # Validate numeric
  test "${z_heat}" -eq "${z_heat}" 2>/dev/null || buc_die "Heat must be numeric: ${z_heat}"
  test "${z_pace}" -eq "${z_pace}" 2>/dev/null || buc_die "Pace must be numeric: ${z_pace}"

  # Validate ranges
  test "${z_heat}" -ge 0 || buc_die "Heat must be >= 0: ${z_heat}"
  test "${z_heat}" -le 4095 || buc_die "Heat must be <= 4095: ${z_heat}"
  test "${z_pace}" -ge 0 || buc_die "Pace must be >= 0: ${z_pace}"
  test "${z_pace}" -le 262143 || buc_die "Pace must be <= 262143: ${z_pace}"

  # Encode heat (2 base64 digits)
  local z_heat_d1
  local z_heat_d2
  z_heat_d1=$((z_heat / 64))
  z_heat_d2=$((z_heat % 64))

  # Encode pace (3 base64 digits)
  local z_pace_d1
  local z_pace_d2
  local z_pace_d3
  z_pace_d1=$((z_pace / 4096))        # pace / (64*64)
  z_pace_d2=$(((z_pace / 64) % 64))
  z_pace_d3=$((z_pace % 64))

  # Convert positions to characters
  local z_c1
  local z_c2
  local z_c3
  local z_c4
  local z_c5
  z_c1="$(zjju_favor_char_at "${z_heat_d1}")"
  z_c2="$(zjju_favor_char_at "${z_heat_d2}")"
  z_c3="$(zjju_favor_char_at "${z_pace_d1}")"
  z_c4="$(zjju_favor_char_at "${z_pace_d2}")"
  z_c5="$(zjju_favor_char_at "${z_pace_d3}")"

  echo "${z_c1}${z_c2}${z_c3}${z_c4}${z_c5}"
}

# Internal: Decode Favor into heat and pace numbers
# Args: favor (5-char string)
# Output: tab-delimited "heat\tpace"
zjju_favor_decode() {
  zjju_sentinel
  local z_favor="${1:-}"

  # Validate parameter
  test -n "${z_favor}" || buc_die "Parameter 'favor' is required"

  # Validate length
  test "${#z_favor}" -eq 5 || buc_die "Favor must be exactly 5 characters: ${z_favor}"

  # Extract characters
  local z_c1="${z_favor:0:1}"
  local z_c2="${z_favor:1:1}"
  local z_c3="${z_favor:2:1}"
  local z_c4="${z_favor:3:1}"
  local z_c5="${z_favor:4:1}"

  # Convert characters to positions
  local z_p1
  local z_p2
  local z_p3
  local z_p4
  local z_p5
  z_p1="$(zjju_favor_pos_of "${z_c1}")"
  z_p2="$(zjju_favor_pos_of "${z_c2}")"
  z_p3="$(zjju_favor_pos_of "${z_c3}")"
  z_p4="$(zjju_favor_pos_of "${z_c4}")"
  z_p5="$(zjju_favor_pos_of "${z_c5}")"

  # Validate all characters were found
  test "${z_p1}" -ge 0 || buc_die "Invalid character in favor: ${z_c1}"
  test "${z_p2}" -ge 0 || buc_die "Invalid character in favor: ${z_c2}"
  test "${z_p3}" -ge 0 || buc_die "Invalid character in favor: ${z_c3}"
  test "${z_p4}" -ge 0 || buc_die "Invalid character in favor: ${z_c4}"
  test "${z_p5}" -ge 0 || buc_die "Invalid character in favor: ${z_c5}"

  # Decode heat (2 digits)
  local z_heat
  z_heat=$((z_p1 * 64 + z_p2))

  # Decode pace (3 digits)
  local z_pace
  z_pace=$((z_p3 * 4096 + z_p4 * 64 + z_p5))

  # Output tab-delimited
  echo "${z_heat}	${z_pace}"
}

######################################################################
# Studbook File Operations
#
# Single gate for all studbook reads/writes. Validation enforced on write.

# Internal: Validate studbook JSON structure
# Args: json-string
# Returns: 0 if valid, 1 if invalid (with buc_die)
zjju_studbook_validate() {
  zjju_sentinel
  local z_json="${1:-}"
  local z_file="${BUD_TEMP_DIR}/studbook_validate.json"

  buc_trace "Validating studbook JSON"
  test -n "${z_json}" || buc_die "Studbook JSON is empty"

  buc_trace "Writing JSON to temp file: ${z_file}"
  printf '%s' "${z_json}" > "${z_file}"

  # Step 1: Valid JSON object
  jq -e 'type == "object"' "${z_file}" >/dev/null 2>&1 \
    || buc_die "Not a valid JSON object"

  # Step 2: Required top-level fields
  jq -e 'has("heats") and has("saddled") and has("next_heat_seed")' "${z_file}" >/dev/null 2>&1 \
    || buc_die "Missing required fields (heats, saddled, next_heat_seed)"

  # Step 3: Top-level field types
  jq -e '(.heats | type) == "object" and (.saddled | type) == "string" and (.next_heat_seed | type) == "string"' "${z_file}" >/dev/null 2>&1 \
    || buc_die "Type mismatch in top-level fields"

  # Step 4: Heat keys format (₣XX)
  jq -e '.heats | to_entries | all(.key | test("^₣[A-Za-z0-9_-]{2}$"))' "${z_file}" >/dev/null 2>&1 \
    || buc_die "Invalid heat key format (expected ₣XX)"

  # Step 5: Heat required fields
  jq -e '.heats | to_entries | all(.value | has("datestamp") and has("display") and has("silks") and has("status") and has("paces"))' "${z_file}" >/dev/null 2>&1 \
    || buc_die "Heat missing required fields (datestamp, display, silks, status, paces)"

  # Step 6: Heat field formats
  jq -e '.heats | to_entries | all(.value | (.datestamp | test("^[0-9]{6}$")) and (.silks | test("^[a-z0-9-]+$")) and (.status | . == "current" or . == "retired"))' "${z_file}" >/dev/null 2>&1 \
    || buc_die "Invalid heat field format (datestamp=YYMMDD, silks=kebab-case, status=current|retired)"

  # Step 7: Pace validation
  jq -e '.heats | to_entries | all(.value.paces | all(has("id") and has("display") and has("status") and (.id | test("^[0-9]{3}$")) and (.status | . == "current" or . == "pending" or . == "complete" or . == "abandoned" or . == "malformed")))' "${z_file}" >/dev/null 2>&1 \
    || buc_die "Invalid pace (id=XXX, status=current|pending|complete|abandoned|malformed)"

  # Step 8: Saddled format (empty or ₣XX)
  jq -e '(.saddled == "") or (.saddled | test("^₣[A-Za-z0-9_-]{2}$"))' "${z_file}" >/dev/null 2>&1 \
    || buc_die "Invalid saddled format (expected empty or ₣XX)"

  # Step 9: next_heat_seed format (XX)
  jq -e '.next_heat_seed | test("^[A-Za-z0-9_-]{2}$")' "${z_file}" >/dev/null 2>&1 \
    || buc_die "Invalid next_heat_seed format (expected 2 base64 chars)"

  buc_trace "Studbook validation passed"
}

# Internal: Read studbook from disk
# Output: JSON content to stdout
zjju_studbook_read() {
  zjju_sentinel

  test -f "${ZJJU_STUDBOOK_FILE}" || buc_die "Studbook not found: ${ZJJU_STUDBOOK_FILE}"

  cat "${ZJJU_STUDBOOK_FILE}"
}

# Internal: Write studbook to disk (validates first)
# Args: json-string
zjju_studbook_write() {
  zjju_sentinel
  local z_json="${1:-}"

  # Validate before writing
  zjju_studbook_validate "${z_json}"

  # Normalize and write
  jq --sort-keys --indent 2 '.' <<< "${z_json}" > "${ZJJU_STUDBOOK_FILE}"
}

######################################################################
# Studbook Operations

# Internal: Increment heat seed (2 base64 chars)
# Args: current seed (e.g., "AA")
# Output: next seed (e.g., "AB")
zjju_heat_seed_next() {
  zjju_sentinel
  local z_seed="${1:-}"

  test -n "${z_seed}" || buc_die "Seed is required"
  test "${#z_seed}" -eq 2 || buc_die "Seed must be 2 characters: ${z_seed}"

  local z_c1="${z_seed:0:1}"
  local z_c2="${z_seed:1:1}"

  local z_p1
  local z_p2
  z_p1="$(zjju_favor_pos_of "${z_c1}")"
  z_p2="$(zjju_favor_pos_of "${z_c2}")"

  test "${z_p1}" -ge 0 || buc_die "Invalid character in seed: ${z_c1}"
  test "${z_p2}" -ge 0 || buc_die "Invalid character in seed: ${z_c2}"

  # Increment (base64 arithmetic)
  z_p2=$((z_p2 + 1))
  if test "${z_p2}" -ge 64; then
    z_p2=0
    z_p1=$((z_p1 + 1))
    test "${z_p1}" -lt 64 || buc_die "Heat seed overflow (max 4096 heats)"
  fi

  local z_nc1
  local z_nc2
  z_nc1="$(zjju_favor_char_at "${z_p1}")"
  z_nc2="$(zjju_favor_char_at "${z_p2}")"

  echo "${z_nc1}${z_nc2}"
}

jju_muster() {
  zjju_sentinel
  buc_doc_brief "List current heats with Favors and silks"
  buc_doc_shown || return 0

  # Read studbook to temp file
  local z_temp="${BUD_TEMP_DIR}/studbook_muster.json"
  zjju_studbook_read > "${z_temp}"

  # Check if there are any heats
  local z_heat_count
  z_heat_count="$(jq -r '.heats | length' "${z_temp}")"

  if test "${z_heat_count}" -eq 0; then
    echo "No heats in studbook"
    buc_trace "Muster found no heats, temp file: ${z_temp}"
    return 0
  fi

  # Get saddled heat for marking
  local z_saddled
  z_saddled="$(jq -r '.saddled' "${z_temp}")"

  # List heats with their details
  echo "Current heats:"
  echo ""

  jq -r '.heats | to_entries | .[] |
    "\(.key)\t\(.value.datestamp)\t\(.value.display)\t\(.value.silks)\t\(.value.paces | length)"' \
    "${z_temp}" | while IFS=$'\t' read -r z_favor z_date z_display z_silks z_pace_count; do

    # Mark saddled heat
    local z_mark=""
    if test "${z_favor}" = "${z_saddled}"; then
      z_mark="* "
    else
      z_mark="  "
    fi

    # Format output
    printf "%s%s  %s  [%s]  (%s paces)\n" \
      "${z_mark}" "${z_favor}" "${z_display}" "${z_silks}" "${z_pace_count}"
  done

  buc_trace "Muster complete, temp file: ${z_temp}"
}

jju_saddle() {
  zjju_sentinel
  local z_favor="${1:-}"

  buc_doc_brief "Mount heat, show paddock + paces + recent steeple"
  buc_doc_param "favor" "Heat Favor (HH with PPP=000)"
  buc_doc_shown || return 0

  buc_die "not implemented yet"
}

jju_nominate() {
  zjju_sentinel
  local z_display="${1:-}"
  local z_silks="${2:-}"

  buc_doc_brief "Create new heat with paddock stub"
  buc_doc_param "display" "Human-readable heat name"
  buc_doc_param "silks" "Kebab-case identifier"
  buc_doc_shown || return 0

  # Validate parameters
  test -n "${z_display}" || buc_die "Parameter 'display' is required"
  test -n "${z_silks}" || buc_die "Parameter 'silks' is required"

  # Validate silks format (kebab-case)
  echo "${z_silks}" | grep -Eq '^[a-z0-9-]+$' || buc_die "Silks must be kebab-case: ${z_silks}"

  # Read studbook
  local z_studbook
  z_studbook="$(zjju_studbook_read)"

  # Get next heat seed
  local z_seed
  z_seed="$(jq -r '.next_heat_seed' <<< "${z_studbook}")"

  # Format datestamp (YYMMDD)
  local z_datestamp
  z_datestamp="${BUD_NOW_STAMP:0:6}"

  # Create heat entry with Favor prefix
  local z_heat_key="₣${z_seed}"

  # Build new studbook JSON with new heat
  local z_temp="${BUD_TEMP_DIR}/studbook_nominate.json"
  jq --arg key "${z_heat_key}" \
     --arg datestamp "${z_datestamp}" \
     --arg display "${z_display}" \
     --arg silks "${z_silks}" \
     '.heats[$key] = {
        "datestamp": $datestamp,
        "display": $display,
        "silks": $silks,
        "status": "current",
        "paces": []
      }' <<< "${z_studbook}" > "${z_temp}"

  # Increment next_heat_seed
  local z_next_seed
  z_next_seed="$(zjju_heat_seed_next "${z_seed}")"

  jq --arg seed "${z_next_seed}" \
     '.next_heat_seed = $seed' "${z_temp}" > "${z_temp}.2"

  # Read the final JSON
  local z_new_studbook
  z_new_studbook="$(cat "${z_temp}.2")"

  # Write studbook (validates internally)
  zjju_studbook_write "${z_new_studbook}"

  # Create paddock file stub
  local z_paddock_file="${ZJJU_PADDOCK_DIR}/jjp_${z_seed}.md"
  cat > "${z_paddock_file}" <<EOF
# ${z_display}

## Goal

[Describe the goal of this heat]

## Approach

[Describe the approach]

## Constraints

[List any constraints or guidelines]
EOF

  buc_trace "Nominated heat ${z_heat_key}, temp files: ${z_temp}, ${z_temp}.2"
  echo "Heat ${z_heat_key} nominated: ${z_display}"
  echo "Silks: ${z_silks}"
  echo "Paddock: ${z_paddock_file}"
}

jju_slate() {
  zjju_sentinel
  local z_display="${1:-}"

  buc_doc_brief "Add new pace to current heat (append-only)"
  buc_doc_param "display" "Human-readable pace name"
  buc_doc_shown || return 0

  # Validate parameter
  test -n "${z_display}" || buc_die "Parameter 'display' is required"

  # Read studbook to temp file
  local z_temp="${BUD_TEMP_DIR}/studbook_slate.json"
  zjju_studbook_read > "${z_temp}"

  # Get saddled heat and validate it exists
  local z_saddled
  z_saddled="$(jq -r '.saddled' "${z_temp}")"
  test -n "${z_saddled}" || buc_die "No heat saddled"

  # Verify heat exists
  jq -e --arg heat "${z_saddled}" '.heats[$heat] != null' "${z_temp}" >/dev/null 2>&1 \
    || buc_die "Saddled heat not found: ${z_saddled}"

  # Get pace count to determine next ID and status
  local z_pace_count
  z_pace_count="$(jq -r --arg heat "${z_saddled}" \
    '.heats[$heat].paces | length' "${z_temp}")"

  # Calculate next pace ID
  local z_next_id
  if test "${z_pace_count}" -eq 0; then
    z_next_id="001"
  else
    # Get max ID and increment
    local z_max_id
    z_max_id="$(jq -r --arg heat "${z_saddled}" \
      '.heats[$heat].paces | map(.id | tonumber) | max' "${z_temp}")"
    local z_next_num=$((z_max_id + 1))

    # Format as 3-digit string
    if test "${z_next_num}" -lt 10; then
      z_next_id="00${z_next_num}"
    elif test "${z_next_num}" -lt 100; then
      z_next_id="0${z_next_num}"
    else
      z_next_id="${z_next_num}"
    fi
  fi

  # Determine status (current if first pace, pending otherwise)
  local z_status
  if test "${z_pace_count}" -eq 0; then
    z_status="current"
  else
    z_status="pending"
  fi

  # Add pace to heat
  jq --arg heat "${z_saddled}" \
     --arg id "${z_next_id}" \
     --arg display "${z_display}" \
     --arg status "${z_status}" \
     '.heats[$heat].paces += [{
        "id": $id,
        "display": $display,
        "status": $status
      }]' "${z_temp}" > "${z_temp}.2"

  # Read the new studbook
  local z_new_studbook
  z_new_studbook="$(cat "${z_temp}.2")"

  # Write studbook (validates internally)
  zjju_studbook_write "${z_new_studbook}"

  buc_trace "Slated pace ${z_next_id}, temp files: ${z_temp}, ${z_temp}.2"
  echo "Pace ${z_saddled}${z_next_id} slated: ${z_display}"
}

jju_reslate() {
  zjju_sentinel
  local z_favor="${1:-}"
  local z_display="${2:-}"

  buc_doc_brief "Revise pace description"
  buc_doc_param "favor" "Pace Favor (₣HHPPP)"
  buc_doc_param "display" "New description"
  buc_doc_shown || return 0

  # Validate parameters
  test -n "${z_favor}" || buc_die "Parameter 'favor' is required"
  test -n "${z_display}" || buc_die "Parameter 'display' is required"

  # Parse favor (₣HHPPP format)
  test "${z_favor:0:1}" = "₣" || buc_die "Favor must start with ₣: ${z_favor}"
  test "${#z_favor}" -eq 6 || buc_die "Favor must be 6 characters (₣HHPPP): ${z_favor}"

  local z_favor_digits="${z_favor:1}"  # Remove ₣ prefix
  local z_decoded
  z_decoded="$(zjju_favor_decode "${z_favor_digits}")"

  local z_heat_num
  local z_pace_num
  z_heat_num="$(echo "${z_decoded}" | cut -f1)"
  z_pace_num="$(echo "${z_decoded}" | cut -f2)"

  # Reconstruct heat favor (₣HH)
  local z_heat_digits="${z_favor_digits:0:2}"
  local z_heat_favor="₣${z_heat_digits}"

  # Format pace ID as 3-digit string
  local z_pace_id
  if test "${z_pace_num}" -lt 10; then
    z_pace_id="00${z_pace_num}"
  elif test "${z_pace_num}" -lt 100; then
    z_pace_id="0${z_pace_num}"
  else
    z_pace_id="${z_pace_num}"
  fi

  # Read studbook to temp file
  local z_temp="${BUD_TEMP_DIR}/studbook_reslate.json"
  zjju_studbook_read > "${z_temp}"

  # Verify heat exists
  jq -e --arg heat "${z_heat_favor}" '.heats[$heat] != null' "${z_temp}" >/dev/null 2>&1 \
    || buc_die "Heat not found: ${z_heat_favor}"

  # Verify pace exists
  jq -e --arg heat "${z_heat_favor}" --arg id "${z_pace_id}" \
    '.heats[$heat].paces | any(.id == $id)' "${z_temp}" >/dev/null 2>&1 \
    || buc_die "Pace not found: ${z_favor}"

  # Update pace display
  jq --arg heat "${z_heat_favor}" \
     --arg id "${z_pace_id}" \
     --arg display "${z_display}" \
     '.heats[$heat].paces |= map(
        if .id == $id then .display = $display else . end
      )' "${z_temp}" > "${z_temp}.2"

  # Read the new studbook
  local z_new_studbook
  z_new_studbook="$(cat "${z_temp}.2")"

  # Write studbook (validates internally)
  zjju_studbook_write "${z_new_studbook}"

  buc_trace "Reslated pace ${z_favor}, temp files: ${z_temp}, ${z_temp}.2"
  echo "Pace ${z_favor} reslated: ${z_display}"
}

jju_rail() {
  zjju_sentinel
  local z_order="${1:-}"

  buc_doc_brief "Reorder paces in current heat"
  buc_doc_param "order" "Space-separated pace IDs in new order (e.g., '001 003 002')"
  buc_doc_shown || return 0

  # Validate parameter
  test -n "${z_order}" || buc_die "Parameter 'order' is required"

  # Read studbook to temp file
  local z_temp="${BUD_TEMP_DIR}/studbook_rail.json"
  zjju_studbook_read > "${z_temp}"

  # Get saddled heat
  local z_saddled
  z_saddled="$(jq -r '.saddled' "${z_temp}")"
  test -n "${z_saddled}" || buc_die "No heat saddled"

  # Verify heat exists
  jq -e --arg heat "${z_saddled}" '.heats[$heat] != null' "${z_temp}" >/dev/null 2>&1 \
    || buc_die "Saddled heat not found: ${z_saddled}"

  # Get current pace count
  local z_pace_count
  z_pace_count="$(jq -r --arg heat "${z_saddled}" \
    '.heats[$heat].paces | length' "${z_temp}")"

  # Count IDs in order list
  local z_order_count=0
  for z_id in ${z_order}; do
    z_order_count=$((z_order_count + 1))
  done

  # Verify count matches
  test "${z_order_count}" -eq "${z_pace_count}" \
    || buc_die "Order list has ${z_order_count} IDs but heat has ${z_pace_count} paces"

  # Build JSON array of IDs for jq, checking for duplicates
  local z_order_json="["
  local z_first=1
  local z_seen_ids=""
  for z_id in ${z_order}; do
    buc_trace "Processing pace ID: ${z_id}"

    # Validate ID format (3 digits)
    echo "${z_id}" | grep -Eq '^[0-9]{3}$' \
      || buc_die "Invalid pace ID format: ${z_id} (must be 3 digits)"

    # Check for duplicates
    echo "${z_seen_ids}" | grep -q " ${z_id} " \
      && buc_die "Duplicate pace ID in order list: ${z_id}"
    z_seen_ids="${z_seen_ids} ${z_id} "

    # Verify pace exists
    jq -e --arg heat "${z_saddled}" --arg id "${z_id}" \
      '.heats[$heat].paces | any(.id == $id)' "${z_temp}" >/dev/null 2>&1 \
      || buc_die "Pace not found: ${z_id}"

    if test "${z_first}" -eq 1; then
      z_order_json="${z_order_json}\"${z_id}\""
      z_first=0
    else
      z_order_json="${z_order_json},\"${z_id}\""
    fi
  done
  z_order_json="${z_order_json}]"

  # Reorder paces using jq
  jq --arg heat "${z_saddled}" \
     --argjson order "${z_order_json}" \
     '.heats[$heat].paces = [
        $order[] as $id |
        (.heats[$heat].paces[] | select(.id == $id))
      ]' "${z_temp}" > "${z_temp}.2"

  # Read the new studbook
  local z_new_studbook
  z_new_studbook="$(cat "${z_temp}.2")"

  # Write studbook (validates internally)
  zjju_studbook_write "${z_new_studbook}"

  buc_trace "Railed paces ${z_order}, temp files: ${z_temp}, ${z_temp}.2"
  echo "Paces railed: ${z_order}"
}

jju_tally() {
  zjju_sentinel
  local z_favor="${1:-}"
  local z_state="${2:-}"

  buc_doc_brief "Set pace state"
  buc_doc_param "favor" "Pace Favor (₣HHPPP)"
  buc_doc_param "state" "New state: current|pending|complete|abandoned|malformed"
  buc_doc_shown || return 0

  # Validate parameters
  test -n "${z_favor}" || buc_die "Parameter 'favor' is required"
  test -n "${z_state}" || buc_die "Parameter 'state' is required"

  # Validate state value
  case "${z_state}" in
    current|pending|complete|abandoned|malformed)
      ;;
    *)
      buc_die "Invalid state: ${z_state} (must be current|pending|complete|abandoned|malformed)"
      ;;
  esac

  # Parse favor (₣HHPPP format)
  test "${z_favor:0:1}" = "₣" || buc_die "Favor must start with ₣: ${z_favor}"
  test "${#z_favor}" -eq 6 || buc_die "Favor must be 6 characters (₣HHPPP): ${z_favor}"

  local z_favor_digits="${z_favor:1}"  # Remove ₣ prefix
  local z_decoded
  z_decoded="$(zjju_favor_decode "${z_favor_digits}")"

  local z_heat_num
  local z_pace_num
  z_heat_num="$(echo "${z_decoded}" | cut -f1)"
  z_pace_num="$(echo "${z_decoded}" | cut -f2)"

  # Reconstruct heat favor (₣HH)
  local z_heat_digits="${z_favor_digits:0:2}"
  local z_heat_favor="₣${z_heat_digits}"

  # Format pace ID as 3-digit string
  local z_pace_id
  if test "${z_pace_num}" -lt 10; then
    z_pace_id="00${z_pace_num}"
  elif test "${z_pace_num}" -lt 100; then
    z_pace_id="0${z_pace_num}"
  else
    z_pace_id="${z_pace_num}"
  fi

  # Read studbook to temp file
  local z_temp="${BUD_TEMP_DIR}/studbook_tally.json"
  zjju_studbook_read > "${z_temp}"

  # Verify heat exists
  jq -e --arg heat "${z_heat_favor}" '.heats[$heat] != null' "${z_temp}" >/dev/null 2>&1 \
    || buc_die "Heat not found: ${z_heat_favor}"

  # Verify pace exists
  jq -e --arg heat "${z_heat_favor}" --arg id "${z_pace_id}" \
    '.heats[$heat].paces | any(.id == $id)' "${z_temp}" >/dev/null 2>&1 \
    || buc_die "Pace not found: ${z_favor}"

  # Update pace status
  jq --arg heat "${z_heat_favor}" \
     --arg id "${z_pace_id}" \
     --arg state "${z_state}" \
     '.heats[$heat].paces |= map(
        if .id == $id then .status = $state else . end
      )' "${z_temp}" > "${z_temp}.2"

  # Read the new studbook
  local z_new_studbook
  z_new_studbook="$(cat "${z_temp}.2")"

  # Write studbook (validates internally)
  zjju_studbook_write "${z_new_studbook}"

  buc_trace "Tallied pace ${z_favor} to ${z_state}, temp files: ${z_temp}, ${z_temp}.2"
  echo "Pace ${z_favor} tallied: ${z_state}"
}

jju_wrap() {
  zjju_sentinel
  buc_doc_brief "Complete current pace: tally + chalk + advance"
  buc_doc_shown || return 0

  buc_die "not implemented yet"
}

jju_retire_extract() {
  zjju_sentinel
  local z_favor="${1:-}"

  buc_doc_brief "Extract heat data for trophy creation"
  buc_doc_param "favor" "Heat Favor (₣HH)"
  buc_doc_shown || return 0

  # Validate parameter
  test -n "${z_favor}" || buc_die "Parameter 'favor' is required"

  # Validate heat favor format
  test "${z_favor:0:1}" = "₣" || buc_die "Heat favor must start with ₣: ${z_favor}"
  test "${#z_favor}" -eq 3 || buc_die "Heat favor must be 3 characters (₣HH): ${z_favor}"

  # Read studbook to temp file
  local z_temp="${BUD_TEMP_DIR}/studbook_retire.json"
  zjju_studbook_read > "${z_temp}"

  # Verify heat exists
  jq -e --arg heat "${z_favor}" '.heats[$heat] != null' "${z_temp}" >/dev/null 2>&1 \
    || buc_die "Heat not found: ${z_favor}"

  # Extract heat metadata
  local z_datestamp
  local z_display
  local z_silks
  z_datestamp="$(jq -r --arg heat "${z_favor}" '.heats[$heat].datestamp' "${z_temp}")"
  z_display="$(jq -r --arg heat "${z_favor}" '.heats[$heat].display' "${z_temp}")"
  z_silks="$(jq -r --arg heat "${z_favor}" '.heats[$heat].silks' "${z_temp}")"

  # Count paces by status
  local z_total
  local z_complete
  local z_pending
  local z_abandoned
  z_total="$(jq -r --arg heat "${z_favor}" '.heats[$heat].paces | length' "${z_temp}")"
  z_complete="$(jq -r --arg heat "${z_favor}" \
    '.heats[$heat].paces | map(select(.status == "complete")) | length' "${z_temp}")"
  z_pending="$(jq -r --arg heat "${z_favor}" \
    '.heats[$heat].paces | map(select(.status == "pending")) | length' "${z_temp}")"
  z_abandoned="$(jq -r --arg heat "${z_favor}" \
    '.heats[$heat].paces | map(select(.status == "abandoned")) | length' "${z_temp}")"

  # Get heat seed (without ₣)
  local z_seed="${z_favor:1}"

  # Read paddock file
  local z_paddock_file="${ZJJU_PADDOCK_DIR}/jjp_${z_seed}.md"
  test -f "${z_paddock_file}" || buc_die "Paddock file not found: ${z_paddock_file}"

  # Output trophy markdown
  echo "# Trophy: ${z_display}"
  echo ""
  echo "**Favor**: ${z_favor}"
  echo "**Silks**: ${z_silks}"
  echo "**Started**: ${z_datestamp}"
  echo "**Paces**: ${z_total} total (${z_complete} complete, ${z_pending} pending, ${z_abandoned} abandoned)"
  echo ""
  echo "---"
  echo ""
  echo "## Paddock"
  echo ""
  cat "${z_paddock_file}"
  echo ""
  echo "---"
  echo ""
  echo "## Paces"
  echo ""

  # Output paces table
  jq -r --arg heat "${z_favor}" \
    '.heats[$heat].paces | .[] |
     "| \(.id) | \(.display) | \(.status) |"' "${z_temp}" | {
    echo "| ID | Description | Status |"
    echo "|---:|:------------|:-------|"
    cat
  }

  echo ""
  echo "---"
  echo ""

  buc_trace "Retire extract for ${z_favor}, temp file: ${z_temp}"

  echo "## Steeplechase"
  echo ""
  echo "(Steeplechase extraction from git log not yet implemented)"
  echo ""
}

######################################################################
# Steeplechase Operations (Git)

jju_chalk() {
  zjju_sentinel
  local z_emblem="${1:-}"
  local z_title="${2:-}"

  buc_doc_brief "Write steeplechase entry as empty git commit"
  buc_doc_param "emblem" "Entry type (APPROACH, WRAP, BLOCKED, NOTE, etc.)"
  buc_doc_param "title" "Entry title"
  buc_doc_shown || return 0

  buc_die "not implemented yet"
}

jju_rein() {
  zjju_sentinel
  local z_favor="${1:-}"

  buc_doc_brief "Query steeplechase entries from git log"
  buc_doc_param "favor" "Heat Favor (HH) for all entries, or Pace Favor (HHPPP) for filtered"
  buc_doc_shown || return 0

  buc_die "not implemented yet"
}

jju_notch() {
  zjju_sentinel
  local z_message="${1:-}"

  buc_doc_brief "Git commit with JJ metadata, then push"
  buc_doc_param "message" "Commit message (JJ prefix auto-added)"
  buc_doc_shown || return 0

  buc_die "not implemented yet"
}

# eof
