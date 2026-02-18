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
# BURC Regime - Bash Utility Regime Configuration Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZBURC_SOURCED:-}" || buc_die "Module burc multiply sourced - check sourcing hierarchy"
ZBURC_SOURCED=1

######################################################################
# Internal Functions (zburc_*)

zburc_kindle() {
  test -z "${ZBURC_KINDLED:-}" || buc_die "Module burc already kindled"

  # Set defaults for all fields (validate enforces required-ness)
  BURC_STATION_FILE="${BURC_STATION_FILE:-}"
  BURC_TABTARGET_DIR="${BURC_TABTARGET_DIR:-}"
  BURC_TABTARGET_DELIMITER="${BURC_TABTARGET_DELIMITER:-}"
  BURC_TOOLS_DIR="${BURC_TOOLS_DIR:-}"
  BURC_BUK_DIR="${BURC_TOOLS_DIR}/buk"
  BURC_PROJECT_ROOT="${BURC_PROJECT_ROOT:-}"
  BURC_MANAGED_KITS="${BURC_MANAGED_KITS:-}"
  BURC_TEMP_ROOT_DIR="${BURC_TEMP_ROOT_DIR:-}"
  BURC_OUTPUT_ROOT_DIR="${BURC_OUTPUT_ROOT_DIR:-}"
  BURC_LOG_LAST="${BURC_LOG_LAST:-}"
  BURC_LOG_EXT="${BURC_LOG_EXT:-}"

  # Detect unexpected BURC_ variables
  local z_known="BURC_STATION_FILE BURC_TABTARGET_DIR BURC_TABTARGET_DELIMITER BURC_TOOLS_DIR BURC_BUK_DIR BURC_PROJECT_ROOT BURC_MANAGED_KITS BURC_TEMP_ROOT_DIR BURC_OUTPUT_ROOT_DIR BURC_LOG_LAST BURC_LOG_EXT"
  ZBURC_UNEXPECTED=()
  local z_var
  for z_var in $(compgen -v BURC_); do
    case " ${z_known} " in
      *" ${z_var} "*) : ;;
      *) ZBURC_UNEXPECTED+=("${z_var}") ;;
    esac
  done

  # Export variables needed by child processes (exec'd dispatch, workbenches)
  export BURC_TABTARGET_DIR
  export BURC_TOOLS_DIR
  export BURC_BUK_DIR

  ZBURC_KINDLED=1
}

zburc_sentinel() {
  test "${ZBURC_KINDLED:-}" = "1" || buc_die "Module burc not kindled - call zburc_kindle first"
}

# Validate BURC variables via buv_env_* (dies on first error)
# Prerequisite: kindle must have been called; buv_validation.sh must be sourced
zburc_validate_fields() {
  zburc_sentinel

  # Die on unexpected variables
  if test ${#ZBURC_UNEXPECTED[@]} -gt 0; then
    buc_die "Unexpected BURC_ variables: ${ZBURC_UNEXPECTED[*]}"
  fi

  # Validate each field
  buv_env_string      BURC_STATION_FILE          1    512
  buv_env_string      BURC_TABTARGET_DIR         1    128
  buv_env_string      BURC_TABTARGET_DELIMITER   1      1
  buv_env_string      BURC_TOOLS_DIR             1    128
  buv_env_string      BURC_PROJECT_ROOT          1    512
  buv_env_string      BURC_MANAGED_KITS          1    512
  buv_env_string      BURC_TEMP_ROOT_DIR         1    512
  buv_env_string      BURC_OUTPUT_ROOT_DIR       1    512
  buv_env_xname       BURC_LOG_LAST              1     64
  buv_env_xname       BURC_LOG_EXT               1     16
}

# eof
