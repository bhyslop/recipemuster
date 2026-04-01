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
# Bash Qualify Utility Library - Tabtarget structural qualification

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBUQ_INCLUDED:-}" || return 0
ZBUQ_INCLUDED=1

# Source the console utility library
ZBUQ_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
source "${ZBUQ_SCRIPT_DIR}/buc_command.sh"

######################################################################
# Tabtarget structural qualification

buq_tabtargets() {
  local z_tt_dir="${1:-}"
  local z_project_root="${2:-}"
  test -n "${z_tt_dir}"       || buc_die "buq_tabtargets: tabtarget directory required"
  test -n "${z_project_root}" || buc_die "buq_tabtargets: project root required"
  test -d "${z_tt_dir}"       || buc_die "buq_tabtargets: directory not found: ${z_tt_dir}"
  shift 2

  # Remaining arguments are glob patterns for exempt tabtargets
  local z_exemptions=("$@")

  buc_step "Qualifying tabtarget structure in ${z_tt_dir}"

  local z_fail_files=()
  local z_fail_reasons=()
  local z_count=0
  local z_exempt_count=0

  # Prescribed tabtarget form (from buut_tabtarget.sh generator):
  #   Line 1:      #!/bin/bash
  #   Line 2:      export BURD_LAUNCHER="<launcher_path>"
  #   Lines 3..N-1: optional export BURD_*=* flag lines
  #   Last line:   exec "${BASH_SOURCE[0]%/*}/../${BURD_LAUNCHER}" "${0##*/}" "${@}"
  local z_prescribed_shebang='#!/bin/bash'
  local z_prescribed_exec='exec "${BASH_SOURCE[0]%/*}/../${BURD_LAUNCHER}" "${0##*/}" "${@}"'

  local z_file=""
  for z_file in "${z_tt_dir}"/*.sh; do
    test -e "${z_file}" || continue
    z_count=$((z_count + 1))

    local z_basename="${z_file##*/}"

    # Check exemption patterns
    local z_is_exempt=0
    local z_exempt_pattern=""
    for z_exempt_pattern in "${z_exemptions[@]+"${z_exemptions[@]}"}"; do
      case "${z_basename}" in
        ${z_exempt_pattern}) z_is_exempt=1; break ;;
      esac
    done
    if test "${z_is_exempt}" = "1"; then
      z_exempt_count=$((z_exempt_count + 1))
      continue
    fi

    # Load file lines (load-then-iterate per BCG)
    local z_lines=()
    local z_line=""
    while IFS= read -r z_line || test -n "${z_line}"; do
      z_lines+=("${z_line}")
    done < "${z_file}"

    local z_num_lines=${#z_lines[@]}

    # Must have at least 3 lines: shebang, BURD_LAUNCHER, exec
    test "${z_num_lines}" -ge 3 || {
      z_fail_files+=("${z_basename}")
      z_fail_reasons+=("too few lines: ${z_num_lines} (minimum 3)")
      continue
    }

    # Line 1: prescribed shebang
    test "${z_lines[0]}" = "${z_prescribed_shebang}" || {
      z_fail_files+=("${z_basename}")
      z_fail_reasons+=("line 1: expected '${z_prescribed_shebang}', got '${z_lines[0]}'")
      continue
    }

    # Line 2: must be export BURD_LAUNCHER="..."
    case "${z_lines[1]}" in
      'export BURD_LAUNCHER="'*'"') ;;
      *)
        z_fail_files+=("${z_basename}")
        z_fail_reasons+=("line 2: expected 'export BURD_LAUNCHER=\"...\"', got '${z_lines[1]}'")
        continue
        ;;
    esac

    # Extract launcher path for existence check
    local z_launcher_rhs="${z_lines[1]#export BURD_LAUNCHER=\"}"
    local z_launcher_path="${z_launcher_rhs%\"}"

    # Middle lines (3..N-1): must be export BURD_*=*
    local z_middle_ok=1
    local z_i=2
    local z_last_idx=$((z_num_lines - 1))
    while test "${z_i}" -lt "${z_last_idx}"; do
      case "${z_lines[$z_i]}" in
        'export BURD_'*'='*) ;;
        *)
          z_fail_files+=("${z_basename}")
          z_fail_reasons+=("line $((z_i + 1)): expected 'export BURD_*=*', got '${z_lines[$z_i]}'")
          z_middle_ok=0
          break
          ;;
      esac
      z_i=$((z_i + 1))
    done
    test "${z_middle_ok}" = "1" || continue

    # Last line: prescribed exec (parameter expansion, not dirname)
    test "${z_lines[$z_last_idx]}" = "${z_prescribed_exec}" || {
      z_fail_files+=("${z_basename}")
      z_fail_reasons+=("last line: expected prescribed exec, got '${z_lines[$z_last_idx]}'")
      continue
    }

    # Launcher file must exist
    test -f "${z_project_root}/${z_launcher_path}" || {
      z_fail_files+=("${z_basename}")
      z_fail_reasons+=("launcher not found: ${z_launcher_path}")
      continue
    }
  done

  local z_checked=$((z_count - z_exempt_count))
  local z_summary="Checked ${z_checked} tabtargets"
  test "${z_exempt_count}" = "0" || z_summary="${z_summary} (${z_exempt_count} exempt)"
  buc_log_args "${z_summary}"

  if (( ${#z_fail_files[@]} )); then
    local z_j=0
    for z_j in "${!z_fail_files[@]}"; do
      buc_warn "${z_fail_files[$z_j]}: ${z_fail_reasons[$z_j]}" || buc_die "Failed to warn"
    done
    buc_die "Tabtarget qualification failed: ${#z_fail_files[@]} of ${z_checked} tabtargets"
  fi

  buc_log_args "All ${z_checked} tabtargets structurally valid"
}

######################################################################
# Shellcheck qualification

buq_shellcheck() {
  local z_tools_dir="${1:-${BURD_TOOLS_DIR:-}}"
  local z_rcfile="${2:-${BURD_BUK_DIR:+${BURD_BUK_DIR}/busc_shellcheckrc}}"
  local z_result_file="${3:-${BURD_TEMP_DIR:+${BURD_TEMP_DIR}/buq_shellcheck_results.txt}}"
  test -n "${z_tools_dir}"   || buc_die "buq_shellcheck: tools directory required"
  test -n "${z_rcfile}"      || buc_die "buq_shellcheck: rcfile path required"
  test -n "${z_result_file}" || buc_die "buq_shellcheck: result file path required"

  buc_step "Running shellcheck qualification"

  test -f "${z_rcfile}"    || buc_die "Shellcheck rcfile not found: ${z_rcfile}"
  test -d "${z_tools_dir}" || buc_die "Tools directory not found: ${z_tools_dir}"

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
  local z_status=0
  shellcheck --rcfile="${z_rcfile}" -S style -f gcc "${z_files[@]}" \
    > "${z_result_file}" 2>&1 \
    || z_status=$?

  if test "${z_status}" = "0"; then
    buc_step "Shellcheck qualification passed: ${z_file_count} files clean"
    return 0
  fi

  # Count and display findings
  local z_finding_count=0
  local z_discard
  while IFS= read -r z_discard || test -n "${z_discard:-}"; do
    z_finding_count=$((z_finding_count + 1))
  done < "${z_result_file}"
  buc_log_args "Shellcheck findings: ${z_finding_count} (see ${z_result_file})"

  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    buc_warn "${z_line}"
  done < "${z_result_file}"

  buc_die "Shellcheck qualification failed: ${z_finding_count} findings across ${z_file_count} files"
}

# eof
