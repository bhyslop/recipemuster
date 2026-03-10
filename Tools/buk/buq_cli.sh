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
# BUQ CLI - BUK-level qualification operations

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Command Functions

buq_shellcheck() {
  buc_doc_brief "Run shellcheck with BCG-structural suppressions across Tools/"
  buc_doc_shown || return 0

  buc_step "Running shellcheck qualification"

  local -r z_rcfile="${BURD_BUK_DIR}/busc_shellcheckrc"
  local -r z_tools_dir="${BURD_TOOLS_DIR}"

  test -f "${z_rcfile}" || buc_die "Shellcheck rcfile not found: ${z_rcfile}"
  test -d "${z_tools_dir}" || buc_die "Tools directory not found: ${z_tools_dir}"

  # Verify shellcheck is available
  command -v shellcheck >/dev/null 2>&1 || buc_die "shellcheck not found — install from https://www.shellcheck.net"

  # Collect .sh files (load-then-iterate per BCG)
  local z_files=()
  local z_file=""
  while IFS= read -r z_file || test -n "${z_file}"; do
    z_files+=("${z_file}")
  done < <(find "${z_tools_dir}" -name '*.sh' -type f | sort)

  local -r z_file_count=${#z_files[@]}
  buc_log_args "Found ${z_file_count} shell files under ${z_tools_dir}"

  test "${z_file_count}" -gt 0 || buc_die "No .sh files found under ${z_tools_dir}"

  # Run shellcheck — capture output to temp file for forensics
  local -r z_result_file="${BURD_TEMP_DIR}/buq_shellcheck_results.txt"
  local z_status=0
  shellcheck --rcfile="${z_rcfile}" -S style -f gcc "${z_files[@]}" \
    > "${z_result_file}" 2>&1 \
    || z_status=$?

  if test "${z_status}" = "0"; then
    buc_step "Shellcheck qualification passed: ${z_file_count} files clean"
    return 0
  fi

  # Count and display findings
  local -r z_finding_count=$(<"${z_result_file}" wc -l)
  buc_log_args "Shellcheck findings: ${z_finding_count} (see ${z_result_file})"

  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    buc_warn "${z_line}"
  done < "${z_result_file}"

  buc_die "Shellcheck qualification failed: ${z_finding_count} findings across ${z_file_count} files"
}

######################################################################
# Furnish and Main

zbuq_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env "BURD_TEMP_DIR         " "Temporary file directory (dispatch-provided)"
  buc_doc_env_done || return 0
}

buc_execute buq_ "BUK Qualification" zbuq_furnish "$@"

# eof
