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
# BUK Zipper - Colophon registry via parallel arrays

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBUZ_SOURCED:-}" || return 0
ZBUZ_SOURCED=1

######################################################################
# Internal kindle boilerplate

zbuz_kindle() {
  test -z "${ZBUZ_KINDLED:-}" || buc_die "buz already kindled"

  # Registry rolls (populated by buz_enroll in consumer kindle, same-process only)
  z_buz_colophon_roll=()
  z_buz_module_roll=()
  z_buz_command_roll=()
  z_buz_tabtarget_roll=()

  ZBUZ_KINDLED=1
}

######################################################################
# Internal sentinel

zbuz_sentinel() {
  test "${ZBUZ_KINDLED:-}" = "1" || buc_die "Module buz not kindled - call zbuz_kindle first"
}

######################################################################
# Internal helpers

# zbuz_resolve_tabtarget_capture() - Resolve colophon to tabtarget path
# Args: colophon
# Returns: tabtarget path or exit 1
zbuz_resolve_tabtarget_capture() {
  zbuz_sentinel

  local z_colophon="${1:-}"
  test -n "${z_colophon}" || return 1

  local z_matches=("${BURC_TABTARGET_DIR}/${z_colophon}."*.sh)

  # Bash 3.2: no-match glob returns literal — check with test -e
  test -e "${z_matches[0]}" || return 1

  # Allow multiple matches (imprinted colophons share a colophon prefix)
  # Return first match as representative
  echo "${z_matches[0]}"
}

######################################################################
# Public enroll (kindle-only registry population)

# buz_enroll() - Register colophon tuple in parallel rolls
# Args: varname, colophon, module, command
# Assigns colophon string to caller's variable via printf -v
# Side effects: populates registry rolls (must be called in same process, NOT inside $())
buz_enroll() {
  zbuz_sentinel

  local z_varname="${1:-}"
  local z_colophon="${2:-}"
  local z_module="${3:-}"
  local z_command="${4:-}"
  test -n "${z_varname}"  || buc_die "buz_enroll: varname required"
  test -n "${z_colophon}" || buc_die "buz_enroll: colophon required"
  test -n "${z_module}"   || buc_die "buz_enroll: module required"
  test -n "${z_command}"  || buc_die "buz_enroll: command required"

  # Validate variable name
  echo "${z_varname}" | grep -qE '^[A-Za-z_][A-Za-z0-9_]*$' \
    || buc_die "buz_enroll: invalid variable name: ${z_varname}"

  # Validate at least one tabtarget exists (imprinted colophons may have multiple)
  local z_tabtarget
  z_tabtarget=$(zbuz_resolve_tabtarget_capture "${z_colophon}") || buc_die "No tabtarget for colophon '${z_colophon}' in ${BURC_TABTARGET_DIR}/"

  # Roll population (only persists in same-process context, lost in $() subshell)
  z_buz_colophon_roll+=("${z_colophon}")
  z_buz_module_roll+=("${z_module}")
  z_buz_command_roll+=("${z_command}")
  z_buz_tabtarget_roll+=("${z_tabtarget}")

  # Assign colophon to caller's variable
  printf -v "${z_varname}" '%s' "${z_colophon}" || buc_die "buz_enroll: printf -v failed for ${z_varname}"
}

######################################################################
# Lookup dispatch

# zbuz_exec_lookup() - Resolve colophon via registry and exec
# Args: colophon, base_dir [, extra args passed through to exec]
# Execs: ${base_dir}/${module} ${command} [extra args]
# Returns: 1 if colophon not found (does not exec)
zbuz_exec_lookup() {
  zbuz_sentinel

  local z_colophon="${1:-}"
  local z_base_dir="${2:-}"
  test -n "${z_colophon}" || buc_die "zbuz_exec_lookup: colophon required"
  test -n "${z_base_dir}" || buc_die "zbuz_exec_lookup: base_dir required"
  shift 2

  local z_i
  for z_i in "${!z_buz_colophon_roll[@]}"; do
    if [ "${z_buz_colophon_roll[z_i]}" = "${z_colophon}" ]; then
      exec "${z_base_dir}/${z_buz_module_roll[z_i]}" "${z_buz_command_roll[z_i]}" "$@"
    fi
  done

  return 1
}

# eof
