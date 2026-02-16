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
# RBQ Qualify - Qualification orchestrator for tabtarget and colophon health

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBQ_SOURCED:-}" || buc_die "Module rbq multiply sourced - check sourcing hierarchy"
ZRBQ_SOURCED=1

######################################################################
# Internal Functions (zrbq_*)

zrbq_kindle() {
  test -z "${ZRBQ_KINDLED:-}" || buc_die "Module rbq already kindled"

  zbuz_sentinel
  zrbz_sentinel
  zrbcc_sentinel

  test -n "${BURC_TABTARGET_DIR:-}" || buc_die "BURC_TABTARGET_DIR not set"
  test -n "${BURC_TOOLS_DIR:-}"     || buc_die "BURC_TOOLS_DIR not set"

  ZRBQ_TT_DIR="${BURC_TABTARGET_DIR}"
  ZRBQ_PROJECT_ROOT="${BURC_TOOLS_DIR}/.."
  ZRBQ_RBW_DIR="${BURC_TOOLS_DIR}/rbw"
  ZRBQ_RBW_LAUNCHER=".buk/launcher.rbw_workbench.sh"

  ZRBQ_KINDLED=1
}

zrbq_sentinel() {
  test "${ZRBQ_KINDLED:-}" = "1" || buc_die "Module rbq not kindled - call zrbq_kindle first"
}

######################################################################
# External Functions (rbq_*)

rbq_qualify_colophons() {
  zrbq_sentinel

  buc_step "Qualifying RBW colophon registrations"

  local z_fail_files=()
  local z_fail_reasons=()
  local z_checked=0

  local z_file=""
  for z_file in "${ZRBQ_TT_DIR}"/*.sh; do
    test -e "${z_file}" || continue

    local z_basename="${z_file##*/}"

    # Load file lines (load-then-iterate)
    local z_lines=()
    local z_line=""
    while IFS= read -r z_line || test -n "${z_line}"; do
      z_lines+=("${z_line}")
    done < "${z_file}"

    # Only check tabtargets using the RBW workbench launcher
    local z_is_rbw=0
    if (( ${#z_lines[@]} > 1 )); then
      case "${z_lines[1]}" in
        *"${ZRBQ_RBW_LAUNCHER}"*) z_is_rbw=1 ;;
      esac
    fi
    test "${z_is_rbw}" = "1" || continue

    z_checked=$((z_checked + 1))

    # Extract colophon from filename (everything before first delimiter)
    local z_colophon="${z_basename%%.*}"

    # Check colophon exists in zbuz_colophons registry
    local z_found=0
    local z_i=0
    for z_i in "${!zbuz_colophons[@]}"; do
      test "${zbuz_colophons[$z_i]}" = "${z_colophon}" || continue
      z_found=1

      # Verify module file exists
      local z_module="${zbuz_modules[$z_i]}"
      test -f "${ZRBQ_RBW_DIR}/${z_module}" || {
        z_fail_files+=("${z_basename}")
        z_fail_reasons+=("module not found: ${z_module}")
      }
      break
    done

    test "${z_found}" = "1" || {
      z_fail_files+=("${z_basename}")
      z_fail_reasons+=("colophon '${z_colophon}' not registered in rbz_zipper")
    }
  done

  buc_log_args "Checked ${z_checked} RBW tabtargets"

  if (( ${#z_fail_files[@]} )); then
    local z_j=0
    for z_j in "${!z_fail_files[@]}"; do
      buc_warn "${z_fail_files[$z_j]}: ${z_fail_reasons[$z_j]}"
    done
    buc_die "Colophon qualification failed: ${#z_fail_files[@]} of ${z_checked} tabtargets"
  fi

  buc_log_args "All ${z_checked} RBW colophons registered"
}

rbq_qualify_all() {
  zrbq_sentinel

  buc_step "Running full qualification"

  buv_qualify_tabtargets "${ZRBQ_TT_DIR}" "${ZRBQ_PROJECT_ROOT}"
  rbq_qualify_colophons
  rbrn_preflight

  buc_step "Full qualification passed"
}

# eof
