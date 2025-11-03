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
# BUK Workbench - Routes BUK management commands

set -euo pipefail

# Get script directory
BUW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Verbose output if BDU_VERBOSE is set
buw_show() {
  test "${BDU_VERBOSE:-0}" != "1" || echo "BUWSHOW: $*"
}

# Load BURC configuration
buw_load_burc() {
  local z_project_root
  z_project_root="$(cd "${BUW_SCRIPT_DIR}/../.." && pwd)"
  local z_burc_file="${z_project_root}/.buk/burc.env"

  if [ ! -f "${z_burc_file}" ]; then
    echo "ERROR: BURC file not found: ${z_burc_file}" >&2
    exit 1
  fi

  buw_show "Loading BURC from: ${z_burc_file}"
  # shellcheck disable=SC1090
  source "${z_burc_file}"
}

# Simple routing function
buw_route() {
  local z_command="$1"
  shift
  local z_args="$*"

  buw_show "Routing command: ${z_command} with args: ${z_args}"

  # Verify BDU environment variables are present
  if [ -z "${BDU_TEMP_DIR:-}" ]; then
    echo "ERROR: BDU_TEMP_DIR not set - must be called from BDU" >&2
    exit 1
  fi

  if [ -z "${BDU_NOW_STAMP:-}" ]; then
    echo "ERROR: BDU_NOW_STAMP not set - must be called from BDU" >&2
    exit 1
  fi

  buw_show "BDU environment verified"

  # Load BURC configuration
  buw_load_burc

  # Resolve paths relative to project root
  local z_project_root
  z_project_root="$(cd "${BUW_SCRIPT_DIR}/../.." && pwd)"

  # Route based on command
  case "${z_command}" in

    # Launcher management
    buw-ll)
      # List launchers in .buk/
      buw_show "Listing launchers in .buk/"
      echo "Launchers in ${z_project_root}/.buk/:"
      ls -1 "${z_project_root}/.buk/launcher."*.sh 2>/dev/null || echo "  (none found)"
      ;;

    buw-lc)
      # Create launcher: buw-lc <workbench-name>
      # Example: buw-lc myw_workbench
      local z_workbench_name="${1:-}"
      if [ -z "${z_workbench_name}" ]; then
        echo "ERROR: workbench name required: buw-lc <workbench-name>" >&2
        echo "  Example: buw-lc myw_workbench" >&2
        exit 1
      fi

      local z_launcher_file="${z_project_root}/.buk/launcher.${z_workbench_name}.sh"

      if [ -f "${z_launcher_file}" ]; then
        echo "ERROR: launcher already exists: ${z_launcher_file}" >&2
        exit 1
      fi

      buw_show "Creating launcher: ${z_launcher_file}"

      cat > "${z_launcher_file}" <<'LAUNCHER_EOF'
#!/bin/bash

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
# Compatible with Bash 3.2 (e.g., macOS default shell)

z_project_root_dir="${0%/*}/.."
cd "${z_project_root_dir}" || exit 1

# Load BURC configuration
export BDU_REGIME_FILE="${z_project_root_dir}/.buk/burc.env"
source "${BDU_REGIME_FILE}" || exit 1

# Validate config regimes that are known at launch time
# NOTE: BURC and BURS are the standard BUK regimes (project structure + station config)
# These are always validated here because they're required for BDU operation.
#
# If your workbench has additional regimes (like RBRN, RBRR, etc.), you can either:
# 1. Add validation here if they're known at launch time, OR
# 2. Validate them later in the workbench when runtime context is available
"${BURC_TOOLS_DIR}/buk/burc_regime.sh" validate "${z_project_root_dir}/.buk/burc.env" || {
  echo "ERROR: BURC validation failed" >&2
  "${BURC_TOOLS_DIR}/buk/burc_regime.sh" info
  exit 1
}

z_station_file="${z_project_root_dir}/${BURC_STATION_FILE}"
"${BURC_TOOLS_DIR}/buk/burs_regime.sh" validate "${z_station_file}" || {
  echo "ERROR: BURS validation failed: ${z_station_file}" >&2
  "${BURC_TOOLS_DIR}/buk/burs_regime.sh" info
  exit 1
}

# TODO: Set coordinator script path to your workbench
# Set coordinator and delegate to BDU
export BDU_COORDINATOR_SCRIPT="${BURC_TOOLS_DIR}/TODO/TODO_workbench.sh"
exec "${BURC_TOOLS_DIR}/buk/bdu_BashDispatchUtility.sh" "${1##*/}" "${@:2}"
LAUNCHER_EOF

      chmod +x "${z_launcher_file}"
      echo "Created launcher: ${z_launcher_file}"
      echo "TODO: Edit the file to set BDU_COORDINATOR_SCRIPT to the correct workbench path"
      ;;

    buw-lv)
      # Validate launcher: buw-lv <workbench-name>
      # Example: buw-lv myw_workbench
      local z_workbench_name="${1:-}"
      if [ -z "${z_workbench_name}" ]; then
        echo "ERROR: workbench name required: buw-lv <workbench-name>" >&2
        echo "  Example: buw-lv myw_workbench" >&2
        exit 1
      fi

      local z_launcher_file="${z_project_root}/.buk/launcher.${z_workbench_name}.sh"

      if [ ! -f "${z_launcher_file}" ]; then
        echo "ERROR: launcher not found: ${z_launcher_file}" >&2
        exit 1
      fi

      echo "Validating launcher: ${z_launcher_file}"

      # Basic checks
      if [ ! -x "${z_launcher_file}" ]; then
        echo "  ERROR: Not executable"
        exit 1
      fi

      if ! grep -q "BDU_COORDINATOR_SCRIPT" "${z_launcher_file}"; then
        echo "  WARNING: Does not contain BDU_COORDINATOR_SCRIPT"
      fi

      if ! grep -q "burc_regime.sh.*validate" "${z_launcher_file}"; then
        echo "  WARNING: Does not contain BURC validation"
      fi

      if ! grep -q "burs_regime.sh.*validate" "${z_launcher_file}"; then
        echo "  WARNING: Does not contain BURS validation"
      fi

      if grep -q "TODO" "${z_launcher_file}"; then
        echo "  WARNING: Contains TODO markers - may need configuration"
      fi

      echo "  Launcher appears valid"
      ;;

    # TabTarget management
    buw-tc)
      # Create tabtarget: buw-tc <workbench-name> <tabtarget-name>
      local z_workbench_name="${1:-}"
      local z_tabtarget_name="${2:-}"

      if [ -z "${z_workbench_name}" ] || [ -z "${z_tabtarget_name}" ]; then
        echo "ERROR: usage: buw-tc <workbench-name> <tabtarget-name>" >&2
        exit 1
      fi

      local z_tabtarget_file="${z_project_root}/${BURC_TABTARGET_DIR}/${z_tabtarget_name}.sh"
      local z_workbench_file="${z_project_root}/${BURC_TOOLS_DIR}/${z_workbench_name}/${z_workbench_name}_workbench.sh"

      if [ -f "${z_tabtarget_file}" ]; then
        echo "ERROR: tabtarget already exists: ${z_tabtarget_file}" >&2
        exit 1
      fi

      if [ ! -f "${z_workbench_file}" ]; then
        echo "WARNING: workbench not found: ${z_workbench_file}" >&2
        echo "Creating tabtarget anyway..."
      fi

      buw_show "Creating tabtarget: ${z_tabtarget_file}"

      # Extract command from tabtarget name (first token before delimiter)
      local z_command_token
      z_command_token="${z_tabtarget_name%%${BURC_TABTARGET_DELIMITER}*}"

      cat > "${z_tabtarget_file}" <<TABTARGET_EOF
#!/bin/bash
# Generated tabtarget - delegates to ${z_workbench_name} workbench
exec "\$(dirname "\${BASH_SOURCE[0]}")/../${BURC_TOOLS_DIR}/${z_workbench_name}/${z_workbench_name}_workbench.sh" "${z_command_token}" "\${@}"
TABTARGET_EOF

      chmod +x "${z_tabtarget_file}"
      echo "Created tabtarget: ${z_tabtarget_file}"
      echo "  Delegates to: ${z_workbench_file}"
      echo "  Command: ${z_command_token}"
      ;;

    # Regime management (consolidated)
    buw-rv)
      # Validate both regimes
      echo "=== Validating BURC ==="
      "${BUW_SCRIPT_DIR}/burc_regime.sh" validate "${z_project_root}/.buk/burc.env"

      echo ""
      echo "=== Validating BURS ==="
      local z_station_file="${z_project_root}/${BURC_STATION_FILE}"
      "${BUW_SCRIPT_DIR}/burs_regime.sh" validate "${z_station_file}"

      echo ""
      echo "All regime validations passed"
      ;;

    buw-rr)
      # Render both regimes
      echo "=== BURC Configuration ==="
      "${BUW_SCRIPT_DIR}/burc_regime.sh" render "${z_project_root}/.buk/burc.env"

      echo ""
      echo "=== BURS Configuration ==="
      local z_station_file="${z_project_root}/${BURC_STATION_FILE}"
      "${BUW_SCRIPT_DIR}/burs_regime.sh" render "${z_station_file}"
      ;;

    buw-ri)
      # Show info for both regimes
      echo "=== BURC Specification ==="
      "${BUW_SCRIPT_DIR}/burc_regime.sh" info

      echo ""
      echo "=== BURS Specification ==="
      "${BUW_SCRIPT_DIR}/burs_regime.sh" info
      ;;

    # Unknown command
    *)
      echo "ERROR: Unknown command: ${z_command}" >&2
      echo "Available commands:" >&2
      echo "  Launcher:  buw-ll, buw-lc <name>, buw-lv <name>" >&2
      echo "  TabTarget: buw-tc <workbench> <name>" >&2
      echo "  Regime:    buw-rv, buw-rr, buw-ri" >&2
      exit 1
      ;;
  esac
}

buw_main() {
  local z_command="${1:-}"
  shift || true

  if [ -z "${z_command}" ]; then
    echo "ERROR: No command specified" >&2
    exit 1
  fi

  buw_route "${z_command}" "$@"
}

buw_main "$@"

# eof
