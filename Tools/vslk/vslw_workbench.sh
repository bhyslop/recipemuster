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
# VSLW Workbench - Visual SlickEdit Local Kit workbench commands

set -euo pipefail

# Get script directory
VSLW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${VSLW_SCRIPT_DIR}/../buk/buc_command.sh"
source "${VSLW_SCRIPT_DIR}/../buk/burd_regime.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

# Verify dispatch completed
zburd_kindle

# Verbose output if BURE_VERBOSE is set
vslw_show() {
  test "${BURE_VERBOSE:-0}" != "1" || echo "VSLWSHOW: $*"
}

# Load BURC configuration
vslw_load_burc() {
  local z_burc_file="${PWD}/.buk/burc.env"

  test -f "${z_burc_file}" || buc_die "BURC file not found: ${z_burc_file}"

  vslw_show "Loading BURC from: ${z_burc_file}"
  # shellcheck disable=SC1090
  source "${z_burc_file}"
}

# SlickEdit destination configuration
VSLW_PROJECT_BASE_NAME="${PWD##*/}"
VSLW_DEST_DIR="../_vs/${VSLW_PROJECT_BASE_NAME}"

# Simple routing function
vslw_route() {
  local z_command="$1"
  shift

  vslw_show "Routing command: ${z_command} with args: $*"

  zburd_sentinel

  test -n "${VSLW_TEMPLATE_DIR:-}" || buc_die "VSLW_TEMPLATE_DIR not set - must be set by launcher"

  vslw_show "BUD environment verified"

  # Load BURC configuration
  vslw_load_burc

  # Route based on command
  case "${z_command}" in

    # SlickEdit project installer
    vslk-i)
      buc_step "Visual SlickEdit Local Kit Installer"

      local z_template_dir="${PWD}/${VSLW_TEMPLATE_DIR}"
      local z_dest_dir="${PWD}/${VSLW_DEST_DIR}"

      # Validate template directory exists
      test -d "${z_template_dir}" || buc_die "Template directory not found: ${z_template_dir}"

      # Step 1: Delete destination (fail if delete fails - catches held file handles)
      if [ -d "${z_dest_dir}" ]; then
        buc_step "Removing existing SlickEdit project directory"
        vslw_show "Deleting: ${z_dest_dir}"
        rm -rf "${z_dest_dir}" || buc_die "Failed to delete ${z_dest_dir} - is SlickEdit still open?"

        # Verify deletion succeeded
        test ! -d "${z_dest_dir}" || buc_die "Directory still exists after delete: ${z_dest_dir}"
      fi

      # Step 2: Create fresh destination directory
      buc_step "Creating fresh SlickEdit project directory"
      mkdir -p "${z_dest_dir}" || buc_die "Failed to create directory: ${z_dest_dir}"

      # Step 3: Copy template files
      buc_step "Copying SlickEdit project templates"
      cp "${z_template_dir}"/* "${z_dest_dir}/" || buc_die "Failed to copy templates"

      # Step 4: Substitute project directory placeholder in .vpj files
      buc_step "Substituting project directory placeholder"
      local z_vpj_file
      for z_vpj_file in "${z_dest_dir}"/*.vpj; do
        test -f "${z_vpj_file}" || continue
        sed -i '' "s/__VSLW_PROJECT_DIR__/${VSLW_PROJECT_BASE_NAME}/g" "${z_vpj_file}" \
          || buc_die "Failed to substitute placeholder in ${z_vpj_file}"
      done

      # Report success
      local z_file_count
      z_file_count=$(ls -1 "${z_dest_dir}" | wc -l | tr -d ' ')
      buc_success "SlickEdit project created: ${z_dest_dir} (${z_file_count} files)"
      ;;

    # Unknown command
    *)
      buc_die "Unknown command: ${z_command}\nAvailable commands:\n  vslk-i  Install Visual SlickEdit Local Kit"
      ;;
  esac
}

vslw_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  vslw_route "${z_command}" "$@"
}

vslw_main "$@"

# eof
