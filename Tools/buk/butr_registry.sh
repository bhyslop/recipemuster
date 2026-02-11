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
# BUK Test Registry - Suite registration via explicit enrollment following BCG enroll/recite patterns

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBUTR_INCLUDED:-}" || buto_fatal "butr_registry multiply sourced"
ZBUTR_INCLUDED=1

######################################################################
# Internal kindle boilerplate

butr_kindle() {
  test -z "${ZBUTR_KINDLED:-}" || buto_fatal "butr already kindled"

  # Suite rolls (parallel arrays)
  z_butr_name_roll=()
  z_butr_init_roll=()
  z_butr_setup_roll=()

  # Case rolls (parallel arrays with foreign key to suite)
  z_butr_case_fn_roll=()
  z_butr_case_suite_roll=()

  ZBUTR_KINDLED=1
}

######################################################################
# Internal sentinel

zbutr_sentinel() {
  test "${ZBUTR_KINDLED:-}" = "1" || buto_fatal "Module butr not kindled - call butr_kindle first"
}

######################################################################
# Public enrollment functions

# butr_suite_enroll() - Register a test suite with init and setup functions
# Args: suite_name, init_fn, setup_fn
#   suite_name: unique name for the suite
#   init_fn: function to check readiness ("" for always ready)
#   setup_fn: function to call before running suite ("" for none)
butr_suite_enroll() {
  zbutr_sentinel

  local z_name="${1:-}"
  local z_init="${2:-}"
  local z_setup="${3:-}"

  test -n "${z_name}" || buto_fatal "butr_suite_enroll: suite_name required"

  # Check for duplicate suite names
  local z_i
  for z_i in "${!z_butr_name_roll[@]}"; do
    test "${z_butr_name_roll[$z_i]}" != "${z_name}" || buto_fatal "butr_suite_enroll: duplicate suite '${z_name}'"
  done

  # Validate init function if specified
  if test -n "${z_init}"; then
    declare -F "${z_init}" >/dev/null || buto_fatal "butr_suite_enroll: init function not found: ${z_init}"
  fi

  # Validate setup function if specified
  if test -n "${z_setup}"; then
    declare -F "${z_setup}" >/dev/null || buto_fatal "butr_suite_enroll: setup function not found: ${z_setup}"
  fi

  z_butr_name_roll+=("${z_name}")
  z_butr_init_roll+=("${z_init}")
  z_butr_setup_roll+=("${z_setup}")
}

# butr_case_enroll() - Register a single test case for a suite
# Args: suite_name, case_function
butr_case_enroll() {
  zbutr_sentinel

  local z_suite="${1:-}"
  local z_case="${2:-}"

  test -n "${z_suite}" || buto_fatal "butr_case_enroll: suite_name required"
  test -n "${z_case}" || buto_fatal "butr_case_enroll: case_function required"

  # Find suite index
  local z_suite_idx=""
  local z_i
  for z_i in "${!z_butr_name_roll[@]}"; do
    if test "${z_butr_name_roll[$z_i]}" = "${z_suite}"; then
      z_suite_idx="${z_i}"
      break
    fi
  done

  test -n "${z_suite_idx}" || buto_fatal "butr_case_enroll: unknown suite '${z_suite}'"

  # Validate case function exists
  declare -F "${z_case}" >/dev/null || buto_fatal "butr_case_enroll: case function not found: ${z_case}"

  z_butr_case_fn_roll+=("${z_case}")
  z_butr_case_suite_roll+=("${z_suite_idx}")
}

######################################################################
# Public recite functions

# butr_suites_recite() - List all suite names
butr_suites_recite() {
  zbutr_sentinel
  local z_i
  for z_i in "${!z_butr_name_roll[@]}"; do
    echo "${z_butr_name_roll[$z_i]}" || return 1
  done
}

# butr_init_recite() - Get init function for named suite
# Args: suite_name
# Returns: init function name (or empty string)
butr_init_recite() {
  zbutr_sentinel
  local z_name="${1:-}"
  test -n "${z_name}" || return 1
  local z_i
  for z_i in "${!z_butr_name_roll[@]}"; do
    if test "${z_butr_name_roll[$z_i]}" = "${z_name}"; then
      echo "${z_butr_init_roll[$z_i]}" || return 1
      return 0
    fi
  done
  return 1
}

# butr_setup_recite() - Get setup function for named suite
# Args: suite_name
# Returns: setup function name (or empty string)
butr_setup_recite() {
  zbutr_sentinel
  local z_name="${1:-}"
  test -n "${z_name}" || return 1
  local z_i
  for z_i in "${!z_butr_name_roll[@]}"; do
    if test "${z_butr_name_roll[$z_i]}" = "${z_name}"; then
      echo "${z_butr_setup_roll[$z_i]}" || return 1
      return 0
    fi
  done
  return 1
}

# butr_cases_recite() - List case function names for suite
# Args: suite_name
butr_cases_recite() {
  zbutr_sentinel
  local z_suite="${1:-}"
  test -n "${z_suite}" || return 1

  # Find suite index
  local z_suite_idx=""
  local z_i
  for z_i in "${!z_butr_name_roll[@]}"; do
    if test "${z_butr_name_roll[$z_i]}" = "${z_suite}"; then
      z_suite_idx="${z_i}"
      break
    fi
  done

  test -n "${z_suite_idx}" || return 1

  # List all cases for this suite
  for z_i in "${!z_butr_case_fn_roll[@]}"; do
    if test "${z_butr_case_suite_roll[$z_i]}" = "${z_suite_idx}"; then
      echo "${z_butr_case_fn_roll[$z_i]}" || return 1
    fi
  done
}

# butr_suite_for_case_recite() - Find owning suite name for a case function
# Args: case_function
butr_suite_for_case_recite() {
  zbutr_sentinel
  local z_case="${1:-}"
  test -n "${z_case}" || return 1

  local z_i
  for z_i in "${!z_butr_case_fn_roll[@]}"; do
    if test "${z_butr_case_fn_roll[$z_i]}" = "${z_case}"; then
      local z_suite_idx="${z_butr_case_suite_roll[$z_i]}"
      echo "${z_butr_name_roll[$z_suite_idx]}" || return 1
      return 0
    fi
  done
  return 1
}

# eof
