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
# BURE Regime - Bash Utility Regime Environment Module
#
# BURE is an ambient regime — variables are set in the environment by callers,
# not sourced from a file. Callers export BURE_* variables before invoking.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZBURE_SOURCED:-}" || buc_die "Module bure multiply sourced - check sourcing hierarchy"
ZBURE_SOURCED=1

######################################################################
# Internal Functions (zbure_*)

zbure_kindle() {
  test -z "${ZBURE_KINDLED:-}" || buc_die "Module bure already kindled"

  # Set default for all fields (validate enforces required-ness)
  BURE_COUNTDOWN="${BURE_COUNTDOWN:-}"
  BURE_VERBOSE="${BURE_VERBOSE:-0}"
  BURE_COLOR="${BURE_COLOR:-auto}"

  # Detect unexpected BURE_ variables
  local z_known="BURE_COUNTDOWN BURE_VERBOSE BURE_COLOR"
  ZBURE_UNEXPECTED=()
  local z_var
  for z_var in $(compgen -v BURE_); do
    case " ${z_known} " in
      *" ${z_var} "*) : ;;
      *) ZBURE_UNEXPECTED+=("${z_var}") ;;
    esac
  done

  ZBURE_KINDLED=1
}

zbure_sentinel() {
  test "${ZBURE_KINDLED:-}" = "1" || buc_die "Module bure not kindled - call zbure_kindle first"
}

# Validate BURE variables via buv_* (dies on first error)
# Prerequisite: kindle must have been called; buv_validation.sh must be sourced
zbure_validate_fields() {
  zbure_sentinel

  # Die on unexpected variables
  if test ${#ZBURE_UNEXPECTED[@]} -gt 0; then
    buc_die "Unexpected BURE_ variables: ${ZBURE_UNEXPECTED[*]}"
  fi

  # Validate fields (BURE_COUNTDOWN is optional; if set must be "skip")
  buv_opt_enum        BURE_COUNTDOWN             skip
  buv_env_enum        BURE_VERBOSE               0 1 2 3
  buv_env_string      BURE_COLOR                 1   4
}

# eof
