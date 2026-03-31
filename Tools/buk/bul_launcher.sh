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
# BUL Launcher - Shared launcher logic for BUK workbenches.
# Sourced by individual launcher stubs in .buk/
# Compatible with Bash 3.2 (e.g., macOS default shell)
#
# NOTE: This is bootstrap infrastructure, not a full BCG module.
# No kindle/sentinel pattern - this runs before BCG modules are loaded.

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

# Load BURS configuration and kindle
z_station_file="${ZBUL_PROJECT_ROOT}/${BURC_STATION_FILE}"
z_burs_log_dir="BURS_LOG_DIR"
if ! test -f "${z_station_file}"; then
  echo ""
  echo "SETUP NEEDED: Station Regime file not found"
  echo ""
  echo "  Missing: ${z_station_file}"
  echo ""
  echo "  The Bash Utility Kit (BUK) launcher uses two regime files:"
  echo ""
  echo "    Config Regime (BURC) - checked into the repo at ${BURD_REGIME_FILE}"
  echo "      Project-level settings: tool paths, tabtarget layout, and the"
  echo "      location of the Station Regime file."
  echo "      Inspect: tt/buw-rcr.RenderConfigRegime.sh"
  echo ""
  echo "    Station Regime (BURS) - developer-specific, NOT in git"
  echo "      Machine-level settings that vary per developer or workstation."
  echo "      The Config Regime says to look for it at: ${BURC_STATION_FILE}"
  echo "      Inspect: tt/buw-rsr.RenderStationRegime.sh"
  echo ""
  echo "  Other toolkits in the project may define additional regime files."
  echo ""
  echo "  To get started, create the Station Regime file with this content:"
  echo ""
  echo "    ${z_burs_log_dir}=../logs-buk"
  echo ""
  echo "  ${z_burs_log_dir} is the first variable required in the Station Regime."
  echo "  It names the directory for operation logs. All tabtargets run from the"
  echo "  project root, so relative paths resolve from there. The example above"
  echo "  places logs in the parent directory of the repo. You may also use an"
  echo "  absolute path, or a path inside the repo itself (.gitignored) — the"
  echo "  Config Regime's choice of BURC_STATION_FILE path often signals which"
  echo "  convention a project prefers."
  echo ""
  exit 1
fi
source "${z_station_file}" || buc_die "Failed to source: ${z_station_file}"
source "${BURC_TOOLS_DIR}/buk/burs_regime.sh" || buc_die "Failed to source burs_regime.sh"
zburs_kindle
zburs_enforce

# Helper function to delegate to BURD
# Usage: bul_launch "path/to/workbench.sh" "$@"
bul_launch() {
  local z_coordinator="$1"
  shift

  # Detect terminal width via /dev/tty (survives exec chain and dispatch pipes)
  # Subshell probe: /dev/tty may exist but not be openable (CI, sandbox)
  BURD_TERM_COLS=80
  if (exec </dev/tty) 2>/dev/null; then
    read -r BURD_TERM_COLS < <(tput cols 2>/dev/null)
    test -n "${BURD_TERM_COLS}" || BURD_TERM_COLS=80
  fi
  export BURD_TERM_COLS

  export BURD_COORDINATOR_SCRIPT="${z_coordinator}"
  exec "${BURC_TOOLS_DIR}/buk/bud_dispatch.sh" "${1##*/}" "${@:2}"
}
