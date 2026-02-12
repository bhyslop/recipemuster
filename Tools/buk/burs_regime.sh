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
# BURS Regime - Bash Utility Regime Station Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZBURS_SOURCED:-}" || buc_die "Module burs multiply sourced - check sourcing hierarchy"
ZBURS_SOURCED=1

######################################################################
# Internal Functions (zburs_*)

zburs_kindle() {
  test -z "${ZBURS_KINDLED:-}" || buc_die "Module burs already kindled"

  # Set default for all fields (validate enforces required-ness)
  BURS_LOG_DIR="${BURS_LOG_DIR:-}"

  # Detect unexpected BURS_ variables
  local z_known="BURS_LOG_DIR"
  ZBURS_UNEXPECTED=()
  local z_var
  for z_var in $(compgen -v BURS_); do
    case " ${z_known} " in
      *" ${z_var} "*) : ;;
      *) ZBURS_UNEXPECTED+=("${z_var}") ;;
    esac
  done

  ZBURS_KINDLED=1
}

zburs_sentinel() {
  test "${ZBURS_KINDLED:-}" = "1" || buc_die "Module burs not kindled - call zburs_kindle first"
}

# Validate BURS variables via buv_env_* (dies on first error)
# Prerequisite: kindle must have been called; buv_validation.sh must be sourced
zburs_validate_fields() {
  zburs_sentinel

  # Die on unexpected variables
  if test ${#ZBURS_UNEXPECTED[@]} -gt 0; then
    buc_die "Unexpected BURS_ variables: ${ZBURS_UNEXPECTED[*]}"
  fi

  # Validate field
  buv_env_string      BURS_LOG_DIR               1    512
}

# eof
