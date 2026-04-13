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
# BUL No-Log Launcher - Station-free launcher for no-log handbook tabtargets.
# Loads BURC only — skips BURS station file entirely.
# Sourced by no-log launcher stubs in .buk/
# Compatible with Bash 3.2 (e.g., macOS default shell)
#
# Use case: handbook tabtargets that set BURD_NO_LOG=1 need BURC
# (project config) but not BURS (personal station). This lets
# handbooks run on a fresh clone before the user creates a station file.

# Guard against multiple inclusion
test -z "${ZBUL_LAUNCHER_SOURCED:-}" || return 0
ZBUL_LAUNCHER_SOURCED=1

# Establish project root from the sourcing launcher's location
ZBUL_PROJECT_ROOT="${BASH_SOURCE[1]%/*}/.."
cd -P "${ZBUL_PROJECT_ROOT}" || exit 1 # buc_die not available yet
ZBUL_PROJECT_ROOT="${PWD}"

# Establish config directory — canonical locator for .buk/
export BURD_CONFIG_DIR="${ZBUL_PROJECT_ROOT}/.buk"

# Load BURC configuration
export BURD_REGIME_FILE="${BURD_CONFIG_DIR}/burc.env"
source "${BURD_REGIME_FILE}" || exit 1 # buc_die not available yet

# Apply BURV (Bash Utility Regime Verification) overrides if set
BURC_OUTPUT_ROOT_DIR="${BURV_OUTPUT_ROOT_DIR:-${BURC_OUTPUT_ROOT_DIR}}"
BURC_TEMP_ROOT_DIR="${BURV_TEMP_ROOT_DIR:-${BURC_TEMP_ROOT_DIR}}"

# Source BUK modules
export BURD_STATION_FILE="${ZBUL_PROJECT_ROOT}/${BURC_STATION_FILE}"
source "${BURC_TOOLS_DIR}/buk/buc_command.sh" || exit 1 # buc_die not available yet
source "${BURC_TOOLS_DIR}/buk/buv_validation.sh" || buc_die "Failed to source buv_validation.sh"
zbuv_kindle

# Load and kindle BURC
source "${BURC_TOOLS_DIR}/buk/burc_regime.sh" || buc_die "Failed to source burc_regime.sh"
zburc_kindle
zburc_enforce

# BURS is intentionally NOT loaded — no station file required.
# Dispatch guards BURS references behind BURD_NO_LOG checks.

# Helper function to delegate to BURD
# Usage: bul_launch "path/to/workbench.sh" "$@"
bul_launch() {
  local z_coordinator="$1"
  shift

  # Detect terminal width via /dev/tty (survives exec chain and dispatch pipes)
  # Subshell probe: /dev/tty may exist but not be openable (CI, sandbox)
  BURD_TERM_COLS=80
  if (exec </dev/tty) 2>/dev/null; then
    read -r _ BURD_TERM_COLS < <(stty size </dev/tty 2>/dev/null)
    test -n "${BURD_TERM_COLS}" || BURD_TERM_COLS=80
  fi
  export BURD_TERM_COLS

  export BURD_COORDINATOR_SCRIPT="${z_coordinator}"
  exec "${BURC_TOOLS_DIR}/buk/bud_dispatch.sh" "${1##*/}" "${@:2}"
}
