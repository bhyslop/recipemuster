#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
source "${BURD_BUK_DIR}/buc_command.sh" || exit 1
source "${BURD_BUK_DIR}/buv_validation.sh" || buc_die_now "Failed to source buv_validation.sh"
source "${BURD_BUK_DIR}/burd_regime.sh" || buc_die_now "Failed to source burd_regime.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh" || buc_die_now "Failed to source buym_yelp.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

# Verify dispatch completed
zbuv_kindle
zburd_kindle

# Verbose output if BURE_VERBOSE is set
vslw_show() {
  test "${BURE_VERBOSE:-0}" != "1" || echo "VSLWSHOW: $*"
}

# Load BURC configuration
vslw_load_burc() {
  # BURD_REGIME_FILE is the dispatch-resolved burc.env path (exported by bul_launcher).
  local z_burc_file="${BURD_REGIME_FILE}"

  test -f "${z_burc_file}" || buc_die_now "BURC file not found: ${z_burc_file}"

  vslw_show "Loading BURC from: ${z_burc_file}"
  source "${z_burc_file}" || buc_die_now "Failed to source BURC file: ${z_burc_file}"
}

# SlickEdit destination configuration
VSLW_PROJECT_BASE_NAME="${PWD##*/}"
VSLW_DEST_DIR="../_vs/${VSLW_PROJECT_BASE_NAME}"

# Simple routing function
vslw_route() {
  local z_command="${1}"
  shift || buc_die_now "vslw_route: command required"

  vslw_show "Routing command: ${z_command} with args: $*"

  zburd_sentinel

  test -n "${VSLW_TEMPLATE_DIR:-}" || buc_die_now "VSLW_TEMPLATE_DIR not set - must be set by launcher"

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
      test -d "${z_template_dir}" || buc_die_now "Template directory not found: ${z_template_dir}"

      # Step 1: Delete destination (fail if delete fails - catches held file handles)
      if test -d "${z_dest_dir}"; then
        buc_step "Removing existing SlickEdit project directory"
        vslw_show "Deleting: ${z_dest_dir}"
        rm -rf "${z_dest_dir}" || buc_die_now "Failed to delete ${z_dest_dir} - is SlickEdit still open?"

        # Verify deletion succeeded
        test ! -d "${z_dest_dir}" || buc_die_now "Directory still exists after delete: ${z_dest_dir}"
      fi

      # Step 2: Create fresh destination directory
      buc_step "Creating fresh SlickEdit project directory"
      mkdir -p "${z_dest_dir}" || buc_die_now "Failed to create directory: ${z_dest_dir}"

      # Step 3: Copy template files
      buc_step "Copying SlickEdit project templates"
      cp "${z_template_dir}"/* "${z_dest_dir}/" || buc_die_now "Failed to copy templates"

      # Step 4: Substitute project directory placeholder in .vpj files
      buc_step "Substituting project directory placeholder"
      case "${VSLW_PROJECT_BASE_NAME}" in
        *[\&\\]*) buc_die_now "Project directory name carries '&' or a backslash: ${VSLW_PROJECT_BASE_NAME}" ;;
      esac
      local z_vpj_index=0
      local z_vpj_file=""
      local z_vpj_tmp=""
      local z_vpj_line=""
      local z_vpj_lines=()
      local z_i=0
      for z_vpj_file in "${z_dest_dir}"/*.vpj; do
        test -f "${z_vpj_file}" || continue
        z_vpj_tmp="${BURD_TEMP_DIR}/vslw_vpj_${z_vpj_index}.vpj"
        z_vpj_index=$((z_vpj_index + 1))
        z_vpj_lines=()
        while IFS= read -r z_vpj_line || test -n "${z_vpj_line}"; do
          z_vpj_lines+=("${z_vpj_line}")
        done < "${z_vpj_file}"
        for z_i in "${!z_vpj_lines[@]}"; do
          printf '%s\n' "${z_vpj_lines[${z_i}]//__VSLW_PROJECT_DIR__/${VSLW_PROJECT_BASE_NAME}}" \
            || buc_die_now "Failed to substitute placeholder in ${z_vpj_file}"
        done > "${z_vpj_tmp}" \
          || buc_die_now "Failed to write substituted ${z_vpj_file} to ${z_vpj_tmp}"
        mv "${z_vpj_tmp}" "${z_vpj_file}" \
          || buc_die_now "Failed to replace ${z_vpj_file}"
      done

      # Report success
      local z_file_count=0
      local z_entry
      for z_entry in "${z_dest_dir}"/* "${z_dest_dir}"/.*; do
        case "${z_entry##*/}" in
          .|..) continue ;;
        esac
        test -e "${z_entry}" || test -L "${z_entry}" || continue
        z_file_count=$((z_file_count + 1))
      done
      buc_success "SlickEdit project created: ${z_dest_dir} (${z_file_count} files)"
      ;;

    # Unknown command
    *)
      buc_die_now "Unknown command: ${z_command}\nAvailable commands:\n  vslk-i  Install Visual SlickEdit Local Kit"
      ;;
  esac
}

vslw_main() {
  local z_command="${1:-}"
  test "$#" -eq 0 || shift

  test -n "${z_command}" || buc_die_now "No command specified"

  vslw_route "${z_command}" "$@"
}

vslw_main "$@"

# eof
