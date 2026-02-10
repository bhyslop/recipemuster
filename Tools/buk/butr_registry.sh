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
# BUK Test Registry - Suite registration via parallel arrays

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBUTR_INCLUDED:-}" || buto_fatal "butr_registry multiply sourced"
ZBUTR_INCLUDED=1

######################################################################
# Internal kindle boilerplate

butr_kindle() {
  test -z "${ZBUTR_KINDLED:-}" || buto_fatal "butr already kindled"

  # Registry arrays
  zbutr_suite_names=()
  zbutr_suite_globs=()
  zbutr_suite_setups=()
  zbutr_suite_tiers=()

  ZBUTR_KINDLED=1
}

######################################################################
# Internal sentinel

zbutr_sentinel() {
  test "${ZBUTR_KINDLED:-}" = "1" || buto_fatal "Module butr not kindled - call butr_kindle first"
}

######################################################################
# Public registry operations

# butr_register() - Register a test suite
# Args: suite_name, glob_pattern, setup_fn, tier
#   suite_name: unique name for the suite
#   glob_pattern: function name prefix for test discovery
#   setup_fn: function to call before running suite ("" for none)
#   tier: "fast" or "slow" (controls --fast/--slow filtering)
butr_register() {
  zbutr_sentinel

  local z_name="${1:-}"
  local z_glob="${2:-}"
  local z_setup="${3:-}"
  local z_tier="${4:-fast}"

  test -n "${z_name}" || buto_fatal "butr_register: suite_name required"
  test -n "${z_glob}" || buto_fatal "butr_register: glob_pattern required"

  # Check for duplicate suite names
  local z_i=0
  for z_i in "${!zbutr_suite_names[@]}"; do
    test "${zbutr_suite_names[$z_i]}" != "${z_name}" || buto_fatal "butr_register: duplicate suite '${z_name}'"
  done

  zbutr_suite_names+=("${z_name}")
  zbutr_suite_globs+=("${z_glob}")
  zbutr_suite_setups+=("${z_setup}")
  zbutr_suite_tiers+=("${z_tier}")
}

######################################################################
# Query functions

butr_get_suites() {
  zbutr_sentinel
  local z_i=0
  for z_i in "${!zbutr_suite_names[@]}"; do
    echo "${zbutr_suite_names[$z_i]}"
  done
}

butr_get_glob() {
  zbutr_sentinel
  local z_name="${1:-}"
  test -n "${z_name}" || return 1
  local z_i=0
  for z_i in "${!zbutr_suite_names[@]}"; do
    if test "${zbutr_suite_names[$z_i]}" = "${z_name}"; then
      echo "${zbutr_suite_globs[$z_i]}"
      return 0
    fi
  done
  return 1
}

butr_get_setup() {
  zbutr_sentinel
  local z_name="${1:-}"
  test -n "${z_name}" || return 1
  local z_i=0
  for z_i in "${!zbutr_suite_names[@]}"; do
    if test "${zbutr_suite_names[$z_i]}" = "${z_name}"; then
      echo "${zbutr_suite_setups[$z_i]}"
      return 0
    fi
  done
  return 1
}

butr_get_tier() {
  zbutr_sentinel
  local z_name="${1:-}"
  test -n "${z_name}" || return 1
  local z_i=0
  for z_i in "${!zbutr_suite_names[@]}"; do
    if test "${zbutr_suite_names[$z_i]}" = "${z_name}"; then
      echo "${zbutr_suite_tiers[$z_i]}"
      return 0
    fi
  done
  return 1
}

# eof
