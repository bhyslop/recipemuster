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

  test -n "${z_json}" || buc_die "Studbook JSON is empty"

  # Validate structure with jq
  local z_result
  z_result=$(jq -e '
    # Top-level structure
    (type == "object") and
    has("heats") and has("saddled") and has("next_heat_seed") and

    # Top-level types
    (.heats | type) == "object" and
    (.saddled | type) == "string" and
    (.next_heat_seed | type) == "string" and

    # All heat entries are valid
    (.heats | to_entries | all(
      .key | test("^₣[A-Za-z0-9_-]{2}$")
    )) and
    (.heats | to_entries | all(
      .value |
      has("datestamp") and has("display") and has("silks") and has("status") and has("paces") and
      (.datestamp | type) == "string" and
      (.datestamp | test("^[0-9]{6}$")) and
      (.display | type) == "string" and
      (.silks | type) == "string" and
      (.silks | test("^[a-z0-9-]+$")) and
      (.status | . == "current" or . == "retired") and
      (.paces | type) == "array" and
      (.paces | all(
        has("id") and has("display") and has("status") and
        (.id | type) == "string" and
        (.id | test("^[0-9]{3}$")) and
        (.display | type) == "string" and
        (.status | . == "pending" or . == "complete" or . == "abandoned" or . == "malformed")
      ))
    )) and

    # saddled is empty or valid favor format
    ((.saddled == "") or (.saddled | test("^₣[A-Za-z0-9_-]{2}[A-Za-z0-9_-]{3}$"))) and

    # next_heat_seed is valid 2-char format
    (.next_heat_seed | test("^[A-Za-z0-9_-]{2}$"))
  ' 2>&1 <<< "${z_json}") || buc_die "Studbook validation failed: ${z_result}"
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

jju_muster() {
  zjju_sentinel
  buc_doc_brief "List current heats with Favors and silks"
  buc_doc_shown || return 0

  buc_die "not implemented yet"
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

  buc_die "not implemented yet"
}

jju_slate() {
  zjju_sentinel
  local z_display="${1:-}"

  buc_doc_brief "Add new pace to current heat (append-only)"
  buc_doc_param "display" "Human-readable pace name"
  buc_doc_shown || return 0

  buc_die "not implemented yet"
}

jju_reslate() {
  zjju_sentinel
  local z_favor="${1:-}"
  local z_display="${2:-}"

  buc_doc_brief "Revise pace description"
  buc_doc_param "favor" "Pace Favor (HHPPP)"
  buc_doc_param "display" "New description"
  buc_doc_shown || return 0

  buc_die "not implemented yet"
}

jju_rail() {
  zjju_sentinel
  local z_order="${1:-}"

  buc_doc_brief "Reorder paces in current heat"
  buc_doc_param "order" "Space-separated pace IDs in new order (e.g., '001 003 002')"
  buc_doc_shown || return 0

  buc_die "not implemented yet"
}

jju_tally() {
  zjju_sentinel
  local z_favor="${1:-}"
  local z_state="${2:-}"

  buc_doc_brief "Set pace state"
  buc_doc_param "favor" "Pace Favor (HHPPP)"
  buc_doc_param "state" "New state: pending|complete|abandoned|malformed"
  buc_doc_shown || return 0

  buc_die "not implemented yet"
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
  buc_doc_param "favor" "Heat Favor (HH with PPP=000)"
  buc_doc_shown || return 0

  buc_die "not implemented yet"
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
