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

  # Registry arrays (populated by coordinator kindle, same-process only)
  zbuz_colophons=()
  zbuz_modules=()
  zbuz_commands=()
  zbuz_tabtargets=()

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

  test "${#z_matches[@]}" -eq 1 || return 1

  echo "${z_matches[0]}"
}

######################################################################
# Public registry operations

# buz_register() - Register colophon tuple in parallel arrays
# Args: colophon, module, command
# Sets: z1z_buz_colophon (colophon string for caller to store as constant)
# Side effects: populates registry arrays (must be called in same process, NOT inside $())
buz_register() {
  zbuz_sentinel

  local z_colophon="${1:-}"
  local z_module="${2:-}"
  local z_command="${3:-}"
  test -n "${z_colophon}" || return 1
  test -n "${z_module}"   || return 1
  test -n "${z_command}"  || return 1

  # Validate tabtarget resolution (die on 0 or >1 matches)
  local z_tabtarget
  z_tabtarget=$(zbuz_resolve_tabtarget_capture "${z_colophon}") || buc_die "No unique tabtarget for colophon '${z_colophon}' in ${BURC_TABTARGET_DIR}/"

  # Registry population (only persists in same-process context, lost in $() subshell)
  zbuz_colophons+=("${z_colophon}")
  zbuz_modules+=("${z_module}")
  zbuz_commands+=("${z_command}")
  zbuz_tabtargets+=("${z_tabtarget}")

  # shellcheck disable=SC2034
  z1z_buz_colophon="${z_colophon}"
}

# eof
